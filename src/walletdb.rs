use crate::structs::{Account};
use crate::utils;

use std::io;
use std::fs;
use std::path::PathBuf;

use chrono::prelude::{DateTime, Local};
use time::{Date, format_description};

use rusqlite::{params, Connection};

const DB_NAME: &str = "wallet.db3";

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

pub fn initialize_database() -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id_account      INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            balance         INTEGER DEFAULT 0,
            available       INTEGER DEFAULT 0,
            is_default      INTEGER DEFAULT 0
        )", []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id_transaction  INTEGER PRIMARY KEY,
            message         TEXT NOT NULL,
            value           INTEGER NOT NULL,
            date            INTEGER NOT NULL,
            charged         INTEGER DEFAULT 0,
            t_type          INTEGER NOT NULL,
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


// Functions to read/write rows using structs.
fn select_account(conn: &Connection, opt_str_id: Option<&str>) -> rusqlite::Result<Account> {
    let opt_id = utils::opt_str_to_opt_i64(opt_str_id);

    let mut stmt = if let Some(id) = opt_id {
        conn.prepare(&format!("SELECT * FROM accounts WHERE id_account={}", id))?
    } else {
        conn.prepare("SELECT * FROM accounts WHERE is_default=1")?
    };

    let account: Account;
    {
        let mut rows = stmt.query([])?;
        account = if let Some(row) = rows.next()? {
            Account::from_row(row)
        } else {
            println!("Account with ID '{}' not found or the ID is invalid!", opt_id.unwrap());
            Account::empty()
        };
    }

    stmt.finalize()?;

    Ok(account)
}

fn update_account(conn: &Connection, account: &Account) -> rusqlite::Result<()> {
    let int_balance: i64 = (account.balance * 100.0).round() as i64;
    let int_available: i64 = (account.available * 100.0).round() as i64;

    conn.execute(
        "UPDATE accounts
        SET name = ?1, balance = ?2, available = ?3
        WHERE id_account = ?4
        ",
        params![&account.name, int_balance, int_available, account.id]
    )?;

    Ok(())
}


// Wallet 'account' subcommands are defined below.
pub fn account_default(id: &str) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    let account = select_account(&conn, Some(id))?;

    if account.default {
        println!("This is already the default account!");
        println!("{}", account);

        return Ok(());
    } else if !account.exists() {
        println!("The ID '{}' was not found in accounts!", id);

        return Ok(());
    }

    match conn.execute("UPDATE accounts SET is_default = 0 WHERE is_default = 1", []) {
        Err(e) => utils::validate_tables(&format!("{}", e), "accounts"),
        _ => ()
    };

    match conn.execute(
        "UPDATE accounts SET is_default = 1 WHERE id_account = ?1",
        params![id]
    ) {
        Ok(1) => {
            println!("Success! The account '{}' is now default!", account.name);
        }
        Ok(_) => println!("More than one row was updated! Please check the consistency of IDs..."),
        Err(e) => utils::validate_tables(&format!("{}", e), "accounts")
    };

    Ok(())
}

pub fn account_delete(opt_id: Option<&str>, delete_all: bool) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    let (account, query) = if delete_all {
        (
            Account::empty(),
            format!("DELETE FROM accounts")
        )
    } else if let Some(id) = opt_id {
        (
            select_account(&conn, Some(id))?,
            format!("DELETE FROM accounts WHERE id_account = {}", id).to_string()
        )
    } else {
        panic!("If you won't delete all items you must provide a valid ID!");
    };

    if account.default && !delete_all {
        println!("You can't delete the default account unless you delete all.");
    } else {
        match conn.execute(&query, []) {
            Ok(0) => {
                println!("Zero rows deleted!");
                if delete_all {
                    println!("Table 'accounts' is empty. Try 'wallet new --help'.");
                }
            }
            Ok(n_rows) => println!("Successfully deleted {} rows!", n_rows),
            Err(e) => {
                utils::validate_tables(&format!("{}", e), "accounts");
            }
        }
    }

    conn.close().unwrap();
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
        let original_balance = account.balance;

        account.balance = match balance.parse::<f64>() {
            Ok(new_balance) => {
                value_received = true;
                new_balance
            },
            Err(_) => {
                panic!("Invalid value for balance '{}'! Please enter a valid real number...", balance);
            }
        };

        let diff = account_balance - original_balance;
        account.available += diff;
    }

    if !value_received {
        println!("You must provide at least one valid argument to update!");
        println!("The account will keep its values.");

        return Ok(());
    }

    match update_account(&conn, &account) {
        Ok(_) => println!("Successfully updated account data!"),
        Err(e) => utils::validate_tables(&format!("{}", e), "accounts")
    }

    println!("Resulting account:\n{}", account);

    Ok(())
}


// Wallet 'new' subcommands are defined below.
pub fn new_account(name: &str, balance: f64) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    let exists_account = match conn.execute("SELECT * FROM accounts", []) {
        Ok(_) => { false }
        Err(_) => { true }
    };

    let default: i64 = if exists_account { 0 } else { 1 };
    let int_balance: i64 = (balance * 100.0).round() as i64;

    let result = conn.execute(
        "INSERT INTO accounts (name, balance, available, is_default) VALUES (?1, ?2, ?3, ?4)",
        params![name, int_balance, int_balance, default]
    );

    match result {
        Ok(_) => {
            println!("Successfully created new account!");
            println!("New account {} - ${:.2}", name, balance);
        },
        Err(e) => {
            utils::validate_tables(&format!("{}", e), "accounts");
        }
    }

    conn.close().unwrap();
    Ok(())
}

pub fn new_transaction(
            message: &str, value: f64, t_type: i64, charged: bool, force_price: bool,
            opt_id_account: Option<&str>
        ) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    let mut account = select_account(&conn, opt_id_account)?;

    if value < 0.01 {
        println!("The value of a transaction must be at least one cent '0.01'!");

        return Ok(());
    } else if t_type == 0 && value > account.available && !force_price {
        println!("The account '{}' has no money enough for this purchase!", account.name);
        println!("Available balance is {} and the purchase price is {}.", account.available, value);

        return Ok(());
    }

    let int_value: i64 = (value * 100.0).round() as i64;
    let int_charged: i64 = if charged { 1 } else { 0 };

    let julian_date: i64;
    {
        let local_date: DateTime<Local> = Local::now();
        let str_date = local_date.to_rfc3339().get(0..10).unwrap().to_string();
        
        let parse_format = format_description::parse("[year]-[month]-[day]").unwrap();
        let date = Date::parse(&str_date, &parse_format).unwrap();

        julian_date = date.to_julian_day() as i64;
    }

    let result = conn.execute(
        "INSERT INTO transactions (message, value, date, charged, t_type, id_account)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![message, int_value, julian_date, int_charged, t_type, account.id]
    )

    match result {
        Ok(_) => {
            if t_type == 0 {
                account.available -= value;
                if charged {
                    account.balance -= value;
                }
            } else if t_type == 1{
                account.available += value;
                account.balance += value;
            }

            match update_account(&conn, &account) {
                Ok(_) => println!("Successfully updated account data!"),
                Err(e) => utils::validate_tables(&format!("{}", e), "accounts")
            }
        },
        Err(e) => {
            utils::validate_tables(&format!("{}", e), "accounts");
        }
    }

    Ok(())
}
