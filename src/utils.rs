use crate::account::Account;
use crate::ledger::Ledger;
use crate::transaction::Transaction;
use prettytable::{Cell, Row, Table};

/// Initial greeting to the program
pub fn print_header() {
    println!("ledger system, type \"help\" for commands");
}

/// Help command
pub fn print_help() {
    println!(
        "\nsl\t\t: Save ledger state to disk\n\
        ll\t\t: Load ledger state from disk\n\
    ca <name> <balance>\t\t: Create account\n\
    ct <date> <name> <account_name> <amount> ...\t\t: Create transaction\n\
    la [account_name]\t\t: Print account\n\
    lt [transaction_name]\t\t: Print transaction\n\
    quit\t\t: Exit ledger application saving ledger state to disk\n\
    nsquit\t\t: Exit ledger application without saving ledger state to disk\n\
    help\t\t: Print this help text\n\n\
    NOTE: optional arguments are annotated with [optional] \
    and required arguments with <required>
   "
    );
}

/// Print an account table
pub fn list_account(vec_acc: &Vec<Account>) {
    let mut table = Table::new();

    // Table header
    table.add_row(row!("id", "name", "balance"));
    for account in vec_acc.iter() {
        table.add_row(row!(
            account.id.to_string(),
            account.name,
            account.balance.to_string()
        ));
    }
    table.printstd();
}

/// Print an account table
pub fn list_transaction(vec_tx: &Vec<Transaction>) {
    let mut table = Table::new();

    // Table header
    table.add_row(row!(
        "id",
        "date",
        "transaction name",
        "account name",
        "change in balance"
    ));
    for tx in vec_tx.iter() {
        table.add_row(row!(
            tx.id.to_string(),
            tx.date.to_string(),
            tx.name.to_string(),
            "",
            ""
        ));
        for (acc, bal) in tx.entries.iter() {
            table.add_row(row!("", "", "", acc.name.to_string(), bal.to_string()));
        }
    }
    table.printstd();
}
