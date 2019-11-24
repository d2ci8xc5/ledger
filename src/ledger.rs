use crate::account::Account;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

/// Ledger represents a collection of accounts and transactions.
#[derive(Serialize, Deserialize, Debug)]
pub struct Ledger {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

impl Ledger {
    /// Create a new ledger
    pub fn new(accounts: Vec<Account>, transactions: Vec<Transaction>) -> Ledger {
        Ledger {
            accounts,
            transactions,
        }
    }

    /// Add an account to the ledger
    pub fn add_account(&mut self, acc: Account) -> bool {
        // Only allow addition of unique accounts (id and name are uuid) to ledger
        for account in self.accounts.iter() {
            if acc.id == account.id {
                return false;
            }
            if acc.name == account.name {
                return false;
            }
        }
        self.accounts.push(acc);
        return true;
    }

    // TODO: only allow added accounts to tx (collect)
    /// Write a transaction to the ledger
    pub fn add_transaction(&mut self, tx: Transaction) -> bool {

        // Only allow addition of unique txids to ledger
        for transaction in self.transactions.iter() {
            if tx.id == transaction.id {
                return false;
            }
        }
        self.transactions.push(tx);
        return true;
    }

    /// Serialize ledger to disk
    pub fn save(&self, file: &mut File) -> Result<(), &'static str>{
        let serialized = match serde_json::to_string(&self) {
            Ok(data) => data,
            Err(reason) => return Err("Unable to serialize json"),
        };

        return match write!(file, "{}", serialized) {
            Ok(()) => Ok(()),
            Err(reason) => Err("Unable to write to file")
        };
    }

    /// Serialize ledger from disk
    pub fn load(&self, file: &mut File) -> Result<Ledger, &'static str> {
        match file.seek(SeekFrom::Start(0)) {
            Ok(ledger) => {},
            Err(reason) => {return Err("Unable to start from 0th line in file");}
        };

        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(ledger) => {},
            Err(reason) => {return Err("Unable to read ledger json string from file");}
        };

        return match serde_json::from_str(&buf.to_string()) {
            Ok(serde_ledger) => Ok(serde_ledger),
            Err(reason) =>  Err("Unable to parse string to ledger") 

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[test]
    fn test_ledger_new() {
        let acc_vec = Vec::new();
        let tx_vec = Vec::new();
        let ledger = Ledger::new(acc_vec, tx_vec);
    }

    #[test]
    fn test_serde_disk() {
        // Setup ledger state
        let acc_0 = Account::new(0, String::from("acc_0"), 100).unwrap();
        let acc_1 = Account::new(1, String::from("acc_1"), 100).unwrap();

        let tuple_0 = (acc_0.clone(), 100);
        let tuple_1 = (acc_1.clone(), -100);
        let mut vec: Vec<(Account, i32)> = Vec::new();
        vec.push(tuple_0);
        vec.push(tuple_1);
        let tx_0 = Transaction::new(0, String::from("2019/03/20"), vec).unwrap();

        let acc_vec = vec![acc_0, acc_1];
        let tx_vec = vec![tx_0];
        let ledger = Ledger::new(acc_vec, tx_vec);

        // Serialize ledger to disk
        let mut tmp_file: File = tempfile::tempfile().unwrap();
        ledger.save(&mut tmp_file);
        tmp_file.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        tmp_file.read_to_string(&mut buf).unwrap();
        assert_eq!(serde_json::to_string(&ledger).unwrap(), buf);

        // Deserialize ledger from disk
        let serde_ledger: Ledger = serde_json::from_str(&buf.to_string()).unwrap();
    }
}
