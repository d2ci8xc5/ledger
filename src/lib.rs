pub mod account;
pub mod ledger;
pub mod transaction;

use crate::ledger::Ledger;
use account::Account as acc;
use account::Account;
use std::io;
use std::io::prelude::*;
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
    ca\t\t: Create account\n\
    ct\t\t: Create transaction\n\
    la\t\t: Print account\n\
    lt\t\t: Print transaction\n\
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

    let main_ledger = Ledger::new(Vec::new(), Vec::new());
    //let main_file;
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
                
            }
            "ll" => { // Load ledger

            }
            "ca" => { // Create account

            }
            "ct" => { // Create transaction

            }
            "la" => { // List account balance

            }
            "lt" => { // List transactions

            }
            "quit" => { // Quit
                
            }
            "nsquit" => { // No save quit
                
            }
            "" => {

            }
            _ => {
                println!("unknown command, type \"help\" for a list of availiable commands"); 
            }
        }
    }
}
