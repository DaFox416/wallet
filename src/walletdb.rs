use crate::structs::{Account};
use crate::utils;

use std::io;
use std::fs;
use std::path::PathBuf;

use chrono::prelude::{DateTime, Local};

use rusqlite::{params, Connection};

const DB_NAME: &str = "wallet.db3";

// Functions to read/write rows using structs.
fn select_account(conn: &Connection, opt_str_id: Option<&str>) -> rusqlite::Result<Account> {
    let id_or_active: i64;
    let mut stmt = if let Some(str_id) = opt_str_id {
        id_or_active = match str_id.parse() {
            Ok(neg_id) if neg_id < 0 => {
                panic!("The ID is negative! You must provide an unsigned integer.");
            }
            Ok(ok_id) => ok_id,
            Err(_) => {
                panic!("Invalid ID value '{}' (it must be an unsigned integer)!", str_id);
            }
        };
        conn.prepare("SELECT * FROM accounts WHERE id_account=?")?
    } else {
        id_or_active = 1;
        conn.prepare("SELECT * FROM accounts WHERE active=?")?
    };

    let account: Account;
    {
        let mut rows = stmt.query(params![id_or_active])?;
        account = if let Some(row) = rows.next()? {
            Account::from_row(row)
        } else {
            println!("Account with ID '{}' not found or the ID is invalid!", id_or_active);
            Account::empty()
        };
    }

    stmt.finalize()?;

    Ok(account)
}

fn update_account(conn: &Connection, account: &Account) {
    let int_balance: i64 = (account.balance * 100.0).round() as i64;
    
    match conn.execute(
        "UPDATE accounts
        SET name = ?1, balance = ?2
        WHERE id_account = ?3
        ",
        params![&account.name, int_balance, account.id]
    ) {
        Ok(1) => println!("Successfully updated account data!"),
        Ok(_) => println!("More than one row was updated! Please check the consistency of IDs..."),
        Err(e) => utils::validate_tables(&format!("{}", e), "accounts")
    }
}

// Wallet 'account' subcommands are defined below.
pub fn account_active(id: &str) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    let account = select_account(&conn, Some(id))?;

    if account.is_active() {
        println!("This account is already active!");
        println!("{}", account);

        return Ok(());
    } else if !account.exists() {
        println!("The ID '{}' was not found in accounts!", id);

        return Ok(());
    }

    match conn.execute("UPDATE accounts SET active = 0 WHERE active = 1", []) {
        Err(e) => utils::validate_tables(&format!("{}", e), "accounts"),
        _ => ()
    };

    match conn.execute(
        "UPDATE accounts SET active = ?1 WHERE id_account = ?2",
        params![1, id]
    ) {
        Ok(1) => println!("Success! This account is now active!"),
        Ok(_) => println!("More than one row was updated! Please check the consistency of IDs..."),
        Err(e) => utils::validate_tables(&format!("{}", e), "accounts")
    };

    Ok(())
}

pub fn account_edit(
            id: &str, opt_name: Option<&str>, opt_balance: Option<&str>
        ) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    let mut account = select_account(&conn, Some(id))?;

    if !account.exists() {
        return Ok(());
    } else {
        println!("Account to update:\n{}", account);
    }

    let mut value_received = false;

    if let Some(name) = opt_name {
        account.name = name.to_string();
        value_received = true;
    }

    if let Some(balance) = opt_balance {
        account.balance = match balance.parse::<f64>() {
            Ok(new_balance) => {
                value_received = true;
                new_balance
            },
            Err(_) => {
                println!("Invalid value for balance '{}'! Please enter a real number...", balance);
                account.balance
            }
        }
    }

    if !value_received {
        println!("You must provide at least one valid value to update!");
        println!("The account will keep its values.");
        return Ok(());
    }

    update_account(&conn, &account);

    println!("Resulting account:\n{}", account);

    Ok(())
}

// Wallet subcommands are defined below.
pub fn backup_database(backup_path: &PathBuf) -> Result<(), io::Error> {
    match fs::copy(DB_NAME, backup_path.to_str().unwrap()) {
        Ok(_) => println!("Backup created successfully!"),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => println!("Database does not exists! Try 'wallet init'..."),
                _ => println!("Something went wrong! Error: {}", e),
            }
        }
    }

    Ok(())
}

pub fn delete_items(
            table_name: &str,
            id_name: &str,
            id: &str,
            delete_all: bool
        ) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    let (account, query) = if delete_all {
        (
            Account::empty(),
            format!("DELETE FROM {}", table_name).to_string()
        )
    } else {
        (
            select_account(&conn, Some(id))?,
            format!("DELETE FROM {} WHERE {} = {}", table_name, id_name, id).to_string()
        )
    };

    if account.is_active() {
        println!("You can't delete the active account.");
    } else {
        match conn.execute(&query, []) {
            Ok(0) => {
                println!("Zero rows deleted!");
                if delete_all {
                    println!("Table '{}' is empty. Try 'wallet new --help'.", table_name);
                } else {
                    println!("Not found account with id '{}'.", id);
                }
            }
            Ok(n_rows) => println!("Successfully deleted {} rows!", n_rows),
            Err(e) => {
                utils::validate_tables(&format!("{}", e), table_name);
            }
        }
    }

    conn.close().unwrap();
    Ok(())
}

pub fn initialize_database() -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id_account      INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            balance         INTEGER DEFAULT 0,
            active          INTEGER DEFAULT 0
        )", []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id_transaction  INTEGER PRIMARY KEY,
            message         TEXT NOT NULL,
            value           INTEGER NOT NULL,
            date            INTEGER NOT NULL,
            charged         INTEGER DEFAULT 0,
            type            INTEGER NOT NULL,
            id_account      INTEGER NOT NULL,
            FOREIGN KEY (id_account) REFERENCES accounts (id_account)
        )", []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS payments (
            id_payment    INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            price           INTEGER NOT NULL,
            billing_date    INTEGER NOT NULL,
            priodicity      INTEGER NOT NULL,
            id_account      INTEGER NOT NULL,
            FOREIGN KEY (id_account) REFERENCES accounts (id_account)
        )", []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS savings (
            id_saving       INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            goal            INTEGER NOT NULL,
            balance         INTEGER NOT NULL,
            id_account      INTEGER NOT NULL,
            FOREIGN KEY (id_account) REFERENCES accounts (id_account)
        )", []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS msi_purchases (
            id_msi          INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            price           INTEGER NOT NULL,
            installments    INTEGER NOT NULL,
            months_paid     INTEGER NOT NULL,
            id_account      INTEGER NOT NULL,
            FOREIGN KEY (id_account) REFERENCES accounts (id_account)
        )", []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS queued_purchases (
            id_queued       INTEGER PRIMARY KEY,
            message         TEXT NOT NULL,
            price           INTEGER NOT NULL,
            id_account      INTEGER NOT NULL,
            FOREIGN KEY (id_account) REFERENCES accounts (id_account)
        )", []
    )?;

    conn.close().unwrap();

    Ok(())
}

pub fn list(table_name: &str, count: i64) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;
    let mut stmt = conn.prepare(&format!("SELECT * FROM {} LIMIT {}", table_name, count))?;

    let items = match table_name {
        "accounts" => stmt.query_map([], |row| Ok(Account::from_row(row)))?,
        _ => panic!("Not implemented yet!")
    };

    let mut items_len = 0;

    for item in items {
        items_len += 1;
        println!("{}", item.unwrap());
    }

    if items_len == 0 {
        println!("Table '{}' is empty! Try 'wallet new --help'.", table_name);
    }

    stmt.finalize()?;
    conn.close().unwrap();
    Ok(())
}

// Wallet 'new' subcommands are defined below.
pub fn new_account(name: &str, balance: f64) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    let exists_account = match conn.execute("SELECT * FROM accounts", []) {
        Ok(_) => { false }
        Err(_) => { true }
    };

    let active: i64 = if exists_account { 0 } else { 1 };
    let int_balance: i64 = (balance * 100.0).round() as i64;

    let result = conn.execute(
        "INSERT INTO accounts (name, balance, active) VALUES (?1, ?2, ?3)",
        params![name, int_balance, active]
    );

    match result {
        Ok(_) => {
            println!("Successfully created new account!");
            println!("New account {} - ${:.2}  {}", name, balance, if active == 1 { "Active" } else { "Inactive" } );
        },
        Err(e) => {
            utils::validate_tables(&format!("{}", e), "accounts");
        }
    }

    conn.close().unwrap();
    Ok(())
}

pub fn new_expense(
            message: &str, value: f64, charged: bool, force_price: bool,
            opt_id_account: Option<&str>
        ) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    let account = select_account(&conn, opt_id_account)?;

    println!("{}", account);

    Ok(())
}
