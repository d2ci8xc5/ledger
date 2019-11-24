pub mod account;
pub mod ledger;
pub mod transaction;

use crate::ledger::Ledger;
use account::Account as acc;
use account::Account;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use transaction::Transaction;

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

/// Program input loop
pub fn run_loop() {
    let mut next_txid = 0i32;
    let mut next_accid = 0i32;
    let db_path = "database/ledger.json";
    let mut main_ledger = Ledger::new(Vec::new(), Vec::new());
    let mut main_file = match File::open(db_path) {
        Ok(file) => file,
        Err(reason) => match File::create(db_path) {
                Ok(file) => file,
                Err(reason) => panic!("failed to create ledger json file"),
            },
    };

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
        match args[0] {
            "help" => {
                print_help();    
            }
            "sl" => { // Save ledger
                match main_ledger.save(&mut main_file) {
                    Ok(ledger) => {},
                    Err(reason) => {
                        println!("{}", reason);    
                    }
                }
            }
            "ll" => { // Load ledger
                match main_ledger.load(&mut main_file) {
                    Ok(ledger) => {},
                    Err(reason) => {
                        println!("{}", reason);    
                    }
                };
            }
            "cl" => { // Clear ledger
                main_ledger = Ledger::new(Vec::new(), Vec::new()); 
            }
            "ca" => { // Create account
                if args.len() != 3 {
                    println!("Invalid arguments: {} args given, 2 args required", args.len()-1);
                }

                let name : String = args[1].to_string();
                let balance : i32 =  match args[2].parse::<i32>() {
                    Ok(b) => b,
                    Err(b) => { println!("unable to parse balance (must be integer)"); continue; },
                };

                let account = match Account::new(next_accid.clone(), name, balance) {
                    Ok(acc) => acc,
                    Err(reason) => { println!("{}", reason); continue;}
                };
                if !main_ledger.add_account(account) {
                    println!("Unable to add account (is not unique)");
                }
                println!("Added account to ledger");
            }
            "ct" => { // Create transaction

            }
            "la" => { // List account balance

            }
            "lt" => { // List transactions

            }
            "quit" => { // Quit
                main_ledger.save(&mut main_file);
                break;
            }
            "nsquit" => { // No save quit
                break;
            }
            _ => {
                println!("Unknown command, type \"help\" for a list of availiable commands"); 
            }
        }
    }
}
