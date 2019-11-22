use crate::transaction::Transaction;
use crate::database::Database;

// TODO: database

pub struct Ledger {
    pub record: Vec<Transaction>,
    //pub database: *Database,
}

impl Ledger {
    // TODO: add database support (?)
    pub fn new(record: Vec<Transaction>) -> Ledger {
        //let database: *Database = & Database::new();
        Ledger { record }
    }

    /// Write a transaction to the ledger history
    fn commit(&mut self, tx: Transaction) {
        self.record.push(tx);
    }

    pub fn to_database(&self) {
         
    }
}
