use crate::account::Account;
use chrono::prelude::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Transactions represent the transfer of funds between Accounts.
/// All transactions are required to have a net zero balance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub id: i32,
    pub date: String,
    pub name: String,
    pub entries: Vec<(Account, i32)>,
}

impl Transaction {
    pub fn new(
        id: i32,
        date: String,
        name: String,
        entries: Vec<(Account, i32)>,
    ) -> Result<Transaction, &'static str> {
        let mut sumation: i32 = 0;
        for entry in &entries {
            // Account must have required balance to transfer
            if entry.0.balance + entry.1 < 0 {
                return Err("Not enough funds");
            }
            sumation += entry.1;
        }

        // Transaction must have net zero balance
        if sumation != 0 {
            return Err("Transaction is not balanced");
        }

        // Date validification <YYYY/MM/DD>
        match NaiveDate::parse_from_str(&date.to_string(), "%Y/%m/%d") {
            Ok(date) => (),
            Err(chrono_except) => return Err("Invalid date"),
        }

        return Ok(Transaction {
            id,
            date,
            name,
            entries,
        });
    }
}

mod tests {
    use super::*;
    use crate::account::Account;
    #[test]
    fn test_transaction_new() {
        let tuple_0 = (Account::new(0, String::from("acc_0"), 100).unwrap(), 100);
        let tuple_1 = (Account::new(1, String::from("acc_1"), 100).unwrap(), -100);
        let mut vec: Vec<(Account, i32)> = Vec::new();
        vec.push(tuple_0);
        vec.push(tuple_1);
        match Transaction::new(0, String::from("2019/03/20"), String::from(""), vec) {
            Ok(tx) => (),
            Err(reason) => assert!(false),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_transaction_unbalanced() {
        // format (Account, balance to move)
        let tuple_0 = (Account::new(0, String::from("acc_0"), 100).unwrap(), 100);
        let tuple_1 = (Account::new(1, String::from("acc_1"), 100).unwrap(), 100);
        let mut vec: Vec<(Account, i32)> = Vec::new();
        vec.push(tuple_0);
        vec.push(tuple_1);
        match Transaction::new(0, String::from("2019/03/20"), String::from(""), vec) {
            Ok(tx) => assert!(false),
            Err(reason) => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_invalid_date() {
        // format (Account, balance to move)
        let tuple_0 = (Account::new(0, String::from("acc_0"), 100).unwrap(), 100);
        let tuple_1 = (Account::new(1, String::from("acc_1"), 100).unwrap(), 100);
        let mut vec: Vec<(Account, i32)> = Vec::new();
        vec.push(tuple_0);
        vec.push(tuple_1);

        let invalid_date = String::from("2019/22/22");
        match Transaction::new(0, invalid_date, String::from(""), vec) {
            Ok(tx) => assert!(false),
            Err(reason) => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_invalid_date_format() {
        // format (Account, balance to move)
        let tuple_0 = (Account::new(0, String::from("acc_0"), 100).unwrap(), 100);
        let tuple_1 = (Account::new(1, String::from("acc_1"), 100).unwrap(), 100);
        let mut vec: Vec<(Account, i32)> = Vec::new();
        vec.push(tuple_0);
        vec.push(tuple_1);

        let invalid_date = String::from("2019-22-22");
        match Transaction::new(0, invalid_date, String::from(""), vec) {
            Ok(tx) => assert!(false),
            Err(reason) => (),
            _ => assert!(false),
        }
    }
}
