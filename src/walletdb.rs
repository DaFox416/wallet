use crate::structs::{Account};

use std::io;
use std::fs;
use std::path::PathBuf;

use rusqlite::{params, Connection};

const DB_NAME: &str = "./wallet.db3";

// Functions to read rows as structs.
fn select_account(conn: &Connection, id: i64) -> rusqlite::Result<Account> {
    let mut stmt = conn.prepare("SELECT * FROM accounts WHERE id_account=?")?;
    let account: Account;
    {
        let mut rows = stmt.query(params![id])?;
        account = if let Some(row) = rows.next()? {
            Account::from_row(row)
        } else {
            println!("Account with ID '{}' not found!", id);
            Account::empty()
        };
    }

    stmt.finalize()?;

    Ok(account)
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
        id: i64,
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
            select_account(&conn, id)?,
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
                    println!("Table '{}' is empty. Try 'wallet new [TYPE]' before delete.", table_name);
                } else {
                    println!("Not found account with id '{}'.", id);
                }
            }
            Ok(n_rows) => println!("Successfully deleted {} rows!", n_rows),
            Err(e) => {
                if format!("{}", e).contains("no such table:") {
                    println!("Table '{}' not found! Try 'wallet init' before use it.", table_name);
                }
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
            value           INTEGER NOT NULL,
            id_account      INTEGER NOT NULL,
            FOREIGN KEY (id_account) REFERENCES accounts (id_account)
        )", []
    )?;

    conn.close().unwrap();

    Ok(())
}

// Wallet 'list' subcommands are defined below.
pub fn list(table_name: &str, id_name: &str) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;
    let mut stmt = conn.prepare("SELECT * FROM accounts")?;

    let accounts = stmt.query_map([], |row| {
        Ok( Account::from_row(row) )
    })?;

    for account in accounts {
        println!("{}", account.unwrap());
    }

    stmt.finalize()?;
    conn.close().unwrap();
    Ok(())
}

// Wallet 'new' subcommands are defined below.
pub fn new_account(name: &str, balance: &f64) -> rusqlite::Result<()> {
    let conn = Connection::open(DB_NAME)?;

    let exists_account = match conn.execute("SELECT * FROM accounts", []) {
        Ok(_) => { true }
        Err(_) => { false }
    };

    let active: i64 = if exists_account { 1 } else { 0 };
    let int_balance: i64 = (balance * 100.0).round() as i64;

    conn.execute(
        "INSERT INTO accounts (name, balance, active) VALUES (?1, ?2, ?3)",
        params![name, int_balance, active]
    )?;

    println!("Success!");
    println!("New account {} - ${:.2}  {}", name, balance, if active == 1 { "Active" } else { "Inactive" } );

    conn.close().unwrap();
    Ok(())
}
