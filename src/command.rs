use std::str::FromStr;

pub enum Command {
    help,
    create_account,
    create_transaction,
    list_account,
    list_transaction,
    save_ledger,
    clear_ledger,
    load_ledger,
    quit,
    no_save_quit,
}


impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Command, ()> {
        match s {
            "help" => Ok(Command::help),
            "ca" => Ok(Command::create_account),
            "ct" => Ok(Command::create_transaction),
            "la" => Ok(Command::list_account),
            "lt" => Ok(Command::list_transaction),
            "sl" => Ok(Command::save_ledger),
            "cl" => Ok(Command::clear_ledger),
            "ll" => Ok(Command::load_ledger),
            "quit" => Ok(Command::quit),
            "nsquit" => Ok(Command::no_save_quit),
            _ => Err(()),
        }

    }
}
