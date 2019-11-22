use crate::account::Account;
use chrono::prelude::*;
use std::fmt;

/// Transactions on Accounts in the tuple (Account, (+-)amount)
pub struct Transaction {
    pub id: i32,
    pub date: String,
    pub entries: Vec<(Account, i32)>,
}

impl Transaction {
    pub fn new(
        id: i32,
        date: String,
        entries: Vec<(Account, i32)>,
    ) -> Result<Transaction, &'static str> {
        // Only allow legal transactions
        let mut sumation: i32 = 0;
        for entry in &entries {
            if entry.0.balance + entry.1 < 0 {
                return Err("Not enough funds");
            }
            sumation += entry.1;
        }

        if sumation != 0 {
            return Err("Transaction is not balanced");
        }
        return Ok(Transaction {
            id,
            date: String::from(""),
            entries,
        });
    }
}

/// Display transaction in string
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_transaction_new() {}
}
