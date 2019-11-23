use crate::transaction::Transaction;
use crate::database::Database;
use crate::account::Account;
// TODO: database

pub struct Ledger {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
    //pub database: *Database,
}

impl Ledger {
    // TODO: add database support (?)
    pub fn new(accounts: Vec<Account>, transactions: Vec<Transaction>) -> Ledger {
        //let database: *Database = & Database::new();
        Ledger { accounts, transactions }
    }

    /// Add an account to the ledger
    fn add_account(&mut self, acc: Account) -> bool {

        // Only allow addition of unique accounts to ledger
        for account in self.accounts.iter() {
            if acc.id == account.id {
                return false;
            }
        }
        self.accounts.push(acc);
        return true;
    }

    /// Write a transaction to the ledger
    fn add_transaction(&mut self, tx: Transaction) -> bool {
        
        // Only allow addition of unique txids to ledger
        for transaction in self.transactions.iter() {
            if tx.id == transaction.id {
                return false;
            }
        }
        self.transactions.push(tx);
        return true;
    }

    //// Output ledger state to database
    //pub fn to_database(&self) -> Database {
    //    Database {} 
    //}

    //// Output ledger state to database
    //pub fn from_database(&self) -> Ledger {
    //         
    //}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger_new() {
        let acc_vec = Vec::new();
        let tx_vec = Vec::new();
        let ledger = Ledger::new(acc_vec, tx_vec);

    }
}
