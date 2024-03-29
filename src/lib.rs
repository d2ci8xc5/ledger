pub mod account;
pub mod ledger;
pub mod transaction;
pub mod utils;
pub mod command;
#[macro_use]
extern crate prettytable;

use crate::account::Account;
use crate::ledger::Ledger;
use crate::transaction::Transaction;
use crate::command::Command;
use std::str::FromStr;
use crate::utils::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

/// Program command parser and executor
pub fn run_loop() {
    // Id tracker
    let mut next_txid = 0i32;
    let mut next_accid = 0i32;
    let db_path = "./ledger.json";

    // Main ledger and file
    let mut main_ledger = Ledger::new(Vec::new(), Vec::new());
    let mut main_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(db_path)
        .unwrap();

    print_header();
    let input = io::stdin();

    loop {
        let mut buffer = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        if input.read_line(&mut buffer).is_err() {
            break;
        }
        let args: Vec<&str> = buffer.trim_end().split(' ').collect();
        let cmd : Command = Command::from_str(args[0]).unwrap();
        match cmd {
            Command::help => {
                print_help();
            }
            Command::save_ledger => {
                // Save ledger
                match main_ledger.save(&mut main_file) {
                    Err(reason) => {
                        println!("{}", reason);
                    }
                    _ => {}
                }
            }
            Command::load_ledger => {
                // Load ledger
                match main_ledger.load(&mut main_file) {
                    Ok(ledger) => {
                        main_ledger = ledger;
                        println!("Loaded ledger from disk");
                    }
                    Err(reason) => {
                        println!("{}", reason);
                    }
                };
            }
            Command::clear_ledger => {
                // Clear ledger
                main_ledger = Ledger::new(Vec::new(), Vec::new());
            }
            Command::create_account => {
                // Create account
                if args.len() != 3 {
                    println!(
                        "Invalid arguments: {} args given, 2 args required",
                        args.len() - 1
                    );
                    continue;
                }

                // Parse arguments
                let name: String = args[1].to_string();
                let balance: i32 = match args[2].parse::<i32>() {
                    Ok(b) => b,
                    Err(b) => {
                        println!("unable to parse balance (must be integer)");
                        continue;
                    }
                };

                let account = match Account::new(next_accid.clone(), name, balance) {
                    Ok(acc) => acc,
                    Err(reason) => {
                        println!("{}", reason);
                        continue;
                    }
                };
                if !main_ledger.add_account(account) {
                    println!("Unable to add account (is not unique)");
                    continue;
                }
                next_accid += 1;
                println!("Added account to ledger");
            }
            Command::create_transaction => {
                // Create transaction
                if args.len() < 5 {
                    println!(
                        "Invalid arguments: {} arguments given, atleast 5 arguments required",
                        args.len() - 1
                    );
                    continue;
                }
                let date = String::from(args[1]);
                let name = String::from(args[2]);
                let mut entries: Vec<(Account, i32)> = Vec::new();
                for i in (3..args.len()).step_by(2) {
                    // Get account by name
                    let acc = match main_ledger.get_acc_by_name(String::from(args[i])) {
                        Some(account) => account,
                        None => {
                            println!("Account \"{}\" not found", args[i]);
                            continue;
                        }
                    };
                    let amount: i32 = args[i + 1].parse::<i32>().expect("A number is required");

                    &entries.push((acc, amount));
                }

                let tx = match Transaction::new(next_txid, date, name, entries) {
                    Ok(trans) => trans,
                    Err(reason) => {
                        println!("{}", reason);
                        continue;
                    }
                };

                if main_ledger.add_transaction(tx) {
                    next_txid += 1;
                } else {
                    println!("Invalid transaction");
                    continue;
                }
            }
            Command::list_account => {
                // List account
                // Empty call
                if args.len() == 1 {
                    list_account(&main_ledger.accounts);
                } else {
                    // Specified account
                    let acc = main_ledger.get_acc_by_name(String::from(args[1])).unwrap();
                    list_account(&vec![acc]);
                }
            }
            Command::list_transaction => {
                // List transaction
                // Empty call
                if args.len() == 1 {
                    list_transaction(&main_ledger.transactions);
                } else {
                    // Specified transaction
                    let tx = main_ledger.get_tx_by_name(String::from(args[1])).unwrap();
                    list_transaction(&vec![tx]);
                }
            }
            Command::quit => {
                // Quit
                match main_ledger.save(&mut main_file) {
                    Ok(()) => {}
                    Err(reason) => println!("{}", reason),
                    _ => panic!("unknown save error"),
                }
                break;
            }
            Command::no_save_quit => {
                // No save quit
                break;
            }
            _ => {
                println!("Unknown command, type \"help\" for a list of availiable commands");
            }
        }
    }
}
