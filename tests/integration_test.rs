use ledger::ledger::Ledger;
use ledger::account::Account;
use ledger::transaction::Transaction;


#[test]
fn test_user_story() {
    let ledger = Ledger::new(Vec::new(), Vec::new());
    let acc_0 = Account::new(0, String::from("acc_0"), 100); 
    let acc_1 = Account::new(1, String::from("acc_1"), 100); 
}
