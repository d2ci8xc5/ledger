pub mod account;
pub mod database;
pub mod ledger;
pub mod transaction;

use account::Account as acc;
use account::Account;
use crate::ledger::Ledger;
use shrust::{Shell, ShellIO};
use std::io::prelude::*;
use transaction::Transaction;

pub fn print_header() {
    println!("ledger system, type \"help\" for commands");
}

pub fn run_loop() {
    let mut next_txid = 0i32; 
    let mut next_accid = 0i32; 

    let main_ledger = Ledger::new(Vec::new());

    let v = Vec::new();
    let mut shell = Shell::new(v);

    print_header();
    /// Create account
    shell.new_command(
        "ca <account name> <account balance>",
        "Create account",
        1,
        |io, v, s| {
            for i in 0..s.len() {}
            Ok(())
        },
    );

    /// List account balances
    shell.new_command(
        "la [account_name] | [account_id]",
        "List account balances",
        1,
        |io, v, s| Ok(()),
    );

    /// List transactions
    shell.new_command(
        "lt [account_name][account_id]",
        "List transactions",
        1,
        |io, v, s| Ok(()),
    );

    shell.new_command("push", "Add string to the list", 1, |io, v, s| {
        writeln!(io, "Pushing {}", s[0])?;
        v.push(s[0].to_string());
        Ok(())
    });
    shell.new_command_noargs("list", "List strings", |io, v| {
        for s in v {
            writeln!(io, "{}", s)?;
        }
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}
