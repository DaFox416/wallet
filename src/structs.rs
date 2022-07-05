use std::fmt::{Display, Formatter, Result};

use rusqlite::Row;

#[derive(Debug)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub balance: f64,
    active: i64
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:<4}.- ", self.id)?;
        write!(f, "{:<20} ", self.name)?;
        write!(f, "${:>15.2} ", self.balance)?;
        write!(f, "{:>10}", if self.active == 0 { "Inactive" } else { "Active" })
    }
}

impl Account {
    pub fn is_active(&self) -> bool {
        self.active == 1
    }

    pub fn empty() -> Account {
        Account {
            id: -1,
            name: "".to_string(),
            balance: 0.0,
            active: 0
        }
    }

    pub fn exists(&self) -> bool {
        self.id != -1
    }

    pub fn from_row(row: &Row<'_>) -> Account {
        let id = row.get(0).unwrap();
        let name: String = row.get(1).unwrap();
        let int_balance: i64 = row.get(2).unwrap();
        let balance: f64 = int_balance as f64 / 100.0;
        let active = row.get(3).unwrap();

        Account {
            id: id,
            name: name,
            balance: balance,
            active: active
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub id: i64,
    pub message: String,
    pub value: f64,
    pub date: String,
    pub charged: i64,
    pub in_out: i64
}
