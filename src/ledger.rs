use crate::account::Account;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom, Write};

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
    /// Upon addition of a transaction to the ledger, the transaction will be committed
    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        // Only allow addition of unique txids to ledger
        for transaction in self.transactions.iter() {
            if tx.id == transaction.id {
                return false;
            }
        }
        self.commit(&tx);
        self.transactions.push(tx);
        return true;
    }

    /// Add the transaction balance changes to the account
    fn commit(&mut self, tx: &Transaction) {
        for (account, balance) in tx.entries.iter() {
            for i in 0..self.accounts.len() {
                if self.accounts[i].name == account.name {
                    self.accounts[i].balance += balance;
                }
            }
        }
    }

    /// Serialize ledger to disk
    pub fn save(&self, file: &mut File) -> Result<(), Error> {
        file.set_len(0);
        let serialized = serde_json::to_string(&self)?;
        return write!(file, "{}", serialized);
    }

    /// Serialize ledger from disk
    pub fn load(&self, file: &mut File) -> Result<Ledger, &'static str> {
        match file.seek(SeekFrom::Start(0)) {
            Ok(ledger) => {}
            Err(reason) => {
                return Err("Unable to start from 0th line in file");
            }
        };

        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(ledger) => {}
            Err(reason) => {
                return Err("Unable to read ledger json string from file");
            }
        };

        return match serde_json::from_str(&buf.to_string()) {
            Ok(serde_ledger) => Ok(serde_ledger),
            Err(reason) => Err("Unable to parse string to ledger"),
        };
    }

    /// Fetch cloned account from ledger by searching name
    pub fn get_acc_by_name(&self, name: String) -> Option<Account> {
        for account in &self.accounts {
            if account.name == name {
                return Some(account.clone());
            }
        }
        return None;
    }

    /// Fetch cloned transaction from ledger by searching by name
    pub fn get_tx_by_name(&self, name: String) -> Option<Transaction> {
        for tx in &self.transactions {
            if tx.name == name {
                return Some(tx.clone());
            }
        }
        return None;
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
        let tx_0 = Transaction::new(0, String::from("2019/03/20"), String::from(""), vec).unwrap();

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

    #[test]
    fn test_get_acc_by_name() {
        let acc_0 = Account::new(0, String::from("acc_0"), 100).unwrap();
        let acc_1 = Account::new(1, String::from("acc_1"), 100).unwrap();
        let mut acc_vec = Vec::new();
        acc_vec.push(acc_0.clone());
        acc_vec.push(acc_1.clone());
        let tx_vec = Vec::new();
        let ledger = Ledger::new(acc_vec, tx_vec);

        let account: Account = ledger.get_acc_by_name(String::from("acc_0")).unwrap();
        assert_eq!(account.name, String::from("acc_0"));
    }

    #[test]
    fn test_get_tx_by_name() {
        let acc_0 = Account::new(0, String::from("acc_0"), 100).unwrap();
        let acc_1 = Account::new(1, String::from("acc_1"), 100).unwrap();
        let mut acc_vec = Vec::new();
        acc_vec.push(acc_0.clone());
        acc_vec.push(acc_1.clone());
        let tx_vec = Vec::new();
        let ledger = Ledger::new(acc_vec, tx_vec);

        let account: Account = ledger.get_acc_by_name(String::from("acc_0")).unwrap();
        assert_eq!(account.name, String::from("acc_0"));
    }

    #[test]
    fn test_commit() {
        let acc_0 = Account::new(0, String::from("acc_0"), 100).unwrap();
        let acc_1 = Account::new(1, String::from("acc_1"), 100).unwrap();
        let mut acc_vec = Vec::new();
        acc_vec.push(acc_0.clone());
        acc_vec.push(acc_1.clone());
        // Transfer 100 units of currency from acc_0 to acc_1
        let mut ledger = Ledger::new(acc_vec, Vec::new());
        let tx = Transaction::new(
            0,
            String::from("2018/05/3"),
            String::from(""),
            vec![(acc_0, -100), (acc_1, 100)],
        )
        .unwrap();

        // Commit happens here
        ledger.add_transaction(tx);

        assert_eq!(ledger.accounts[0].balance, 0i32);
        assert_eq!(ledger.accounts[1].balance, 200i32);
    }
}
