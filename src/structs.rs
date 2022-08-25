use std::fmt::{Display, Formatter, Result};

use rusqlite::Row;
use time::Date;

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
    pub t_type: i64,
    pub id_account: i64
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:<6}.- ", self.id)?;
        write!(f, "{}${:>15.2} ", if self.charged { " " } else { "*" }, self.value)?;
        writeln!(f, "'{}'", self.message)?;
        write!(f, "         {} {}", self.date, if self.t_type == 0 { ">>>" } else { "<<<" })
    }
}

impl Transaction {
    pub fn empty() -> Transaction {
        Transaction {
            id: -1,
            message: "".to_string(),
            value: 0.0,
            date: "".to_string(),
            charged: false,
            t_type: -1,
            id_account: -1
        }
    }

    pub fn from_row(row: &Row<'_>) -> Transaction {
        let id: i64 = row.get(0).unwrap();
        let message: String = row.get(1).unwrap();

        let int_value: i64 = row.get(2).unwrap();
        let value: f64 = int_value as f64 / 100.0;

        let julian_date: i32 = row.get(3).unwrap();
        let date = Date::from_julian_day(julian_date).unwrap();
        let str_date: String = format!("{}-{}-{}", date.year(), date.month(), date.day());

        let int_charged: i64 = row.get(4).unwrap();
        let charged = int_charged == 1;

        let t_type: i64 = row.get(5).unwrap();
        let id_account: i64 = row.get(6).unwrap();

        Transaction {
            id: id,
            message: message,
            value: value,
            date: str_date,
            charged: charged,
            t_type: t_type,
            id_account: id_account
        }
    }
}