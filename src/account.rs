use std::cmp;
use std::fmt;

pub struct Account {
    pub id: i32,
    pub name: String,
    pub balance: i32,
}
impl Account {
    pub fn new(name: String, balance: i32, id: i32) -> Result<Account, &'static str> {
        return match balance {
            b if b >= 0 => Ok(Account {id, name, balance}),
            b if b < 0 => Err("invalid balance ( less than zero)"),
            _ => Err("unknown error"),
        };
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}): {}\n", self.id, self.name, self.id)
    }
}

impl cmp::PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        (self.name == other.name && self.balance == other.balance && self.id == other.id)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_new() {
        let account = Account::new(String::from("account_0"), 100i32, 0i32);
        match account {
            Ok(acc) => assert!(true),
            Err(reason) => assert!(false),
            _ => assert!(false),
        }
    }
}
