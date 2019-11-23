use crate::account::Account;
use chrono::prelude::*;
use std::fmt;

/// Transactions represent the transfer of balance between Accounts.
/// All transactions are required to have a net zero balance.
#[derive(Debug)]
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

        // TODO: Date validification <YYYY/MM/DD>
        
        return Ok(Transaction {
            id,
            date, 
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
    use crate::account::Account;
    #[test]
    fn test_transaction_new() {
        // format (Account, balance to move)
        let tuple_0 = (Account::new(0, String::from("acc_0"), 100).unwrap(), 100); 
        let tuple_1 = (Account::new(1, String::from("acc_1"), 100).unwrap(), -100); 
        let mut vec: Vec<(Account, i32)> = Vec::new();
        vec.push(tuple_0);
        vec.push(tuple_1);
        match Transaction::new(0, String::from("2019/03/20"), vec) {
            Ok(tx) => (),
            Err(reason) => assert!(false),
            _ => assert!(false),
        }

        // TODO: Check fields are valid 
    }

    #[test]
    fn test_transaction_unbalanced() {
        // format (Account, balance to move)
        let tuple_0 = (Account::new(0, String::from("acc_0"), 100).unwrap(), 100); 
        let tuple_1 = (Account::new(1, String::from("acc_1"), 100).unwrap(), 100); 
        let mut vec: Vec<(Account, i32)> = Vec::new();
        vec.push(tuple_0);
        vec.push(tuple_1);
        match Transaction::new(0, String::from("2019/03/20"), vec) {
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
        match Transaction::new(0, invalid_date, vec) {
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
        
        // Correct format is YYYY/MM/DD
        let invalid_date = "2019-22-22";
        match Transaction::new(0, String::from("2019/03/20"), vec) {
            Ok(tx) => assert!(false),
            Err(reason) => (),
            _ => assert!(false),
        }
    }
}
