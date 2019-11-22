pub mod account;
pub mod transaction;
pub mod ledger;

use account::Account as acc;
use shrust::{Shell, ShellIO};
use std::io::prelude::*;
use transaction::Transaction as tx;

pub fn input_loop() {
    let v = Vec::new();
    let mut shell = Shell::new(v);
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
