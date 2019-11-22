use crate::account::Account;
use std::fmt;
///
pub struct Transaction {
    pub entries: Vec<(Account, i32)>,
}

impl Transaction {
    pub fn new(entries: Vec<(Account, i32)>) -> Result<Transaction, &'static str> {
        // Only allow legal transactions
        let mut sumation : i32 = 0;
        for entry in &entries {
            if entry.0.balance + entry.1 < 0 {
                return Err("Not enough funds"); 
            }
            sumation += entry.1;
        }

        if sumation != 0 {
            return Err("Transaction is not balanced");
        }

        return Ok(Transaction{ entries });
    }

    ///Execute and commit the transaction to the ledger
    pub fn complete(&mut self) {
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}


mod tests {
    use super::*;
    
    #[test]
    fn test_transaction_new() {
    }

}
