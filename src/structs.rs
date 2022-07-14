use std::fmt::{Display, Formatter, Result};

use rusqlite::Row;

#[derive(Debug)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub balance: f64,
    pub available: f64,
    pub default: bool
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:<4}.-", self.id)?;
        write!(f, " {} ", if self.default { "*" } else { " " })?;
        write!(f, "{:<20} ", self.name)?;
        write!(f, "${:>15.2} ", self.balance)?;
        write!(f, "-> {:>15.2}", self.available)
    }
}

impl Account {
    pub fn empty() -> Account {
        Account {
            id: -1,
            name: "".to_string(),
            balance: 0.0,
            available: 0.0,
            default: false
        }
    }

    pub fn exists(&self) -> bool {
        self.id != -1
    }

    pub fn from_row(row: &Row<'_>) -> Account {
        let id: i64 = row.get(0).unwrap();
        let name: String = row.get(1).unwrap();

        let int_balance: i64 = row.get(2).unwrap();
        let balance: f64 = int_balance as f64 / 100.0;

        let int_available: i64 = row.get(3).unwrap();
        let available: f64 = int_available as f64 / 100.0;

        let is_default: i64 = row.get(4).unwrap();
        let default = is_default != 0;

        Account {
            id: id,
            name: name,
            balance: balance,
            available: available,
            default: default
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub id: i64,
    pub message: String,
    pub value: f64,
    pub date: String,
    pub charged: bool,
    pub flow_type: i64
}
