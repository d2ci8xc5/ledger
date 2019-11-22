use crate::transaction::Transaction;

pub struct ledger {
    pub record: Vec<Transaction>, 
}

impl ledger {
   // TODO: add database support (?) 


    /// Write a transaction to the ledger history
    pub fn commit(&mut self, tx: Transaction) {
        self.record.push(tx);
    }
}
