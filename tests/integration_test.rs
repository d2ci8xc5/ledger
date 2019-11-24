use ledger::account::Account;
use ledger::ledger::Ledger;
use ledger::transaction::Transaction;

#[test]
fn test_user_story() {
    let mut ledger = Ledger::new(Vec::new(), Vec::new());
    let acc_0 = Account::new(0, String::from("acc_0"), 100).unwrap();
    let acc_1 = Account::new(1, String::from("acc_1"), 100).unwrap();

    let tx_0 = Transaction::new(
        0,
        String::from("2018/03/20"),
        String::from("tx_0"),
        vec![(acc_0.clone(), -100), (acc_1.clone(), 100)],
    )
    .unwrap();

    // Add transactions and accounts
    ledger.add_account(acc_0);
    ledger.add_account(acc_1);
    ledger.add_transaction(tx_0);

    // Check transaction was commited
    assert_eq!(ledger.accounts[0].balance, 0i32);
    assert_eq!(ledger.accounts[1].balance, 200i32);
}
