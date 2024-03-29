use serde::{Deserialize, Serialize};
use std::cmp;

/// Accounts represent an amount of funds under a unique name
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub balance: i32,
}

impl Account {
    pub fn new(id: i32, name: String, balance: i32) -> Result<Account, &'static str> {
        return match balance {
            // Balance must be >= 0 on initialisation
            bal if bal >= 0 => Ok(Account { id, name, balance }),
            bal if bal < 0 => Err("invalid balance ( less than zero)"),
            _ => Err("unknown error"),
        };
    }
}

/// Comparsion operation implementation
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
        let acc_0 = Account::new(0, String::from("acc_0"), 100);
        match acc_0 {
            Ok(acc) => assert!(true),
            Err(reason) => assert!(false),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_equality() {
        let acc_0 = Account::new(0, String::from("acc_0"), 100);
        let acc_eq_0 = Account::new(0, String::from("acc_0"), 100);
        assert_eq!(acc_0, acc_eq_0);
    }

    #[test]
    fn test_negative_balance_init() {
        match Account::new(0, String::from("acc_0"), -100) {
            Ok(acc) => assert!(false),
            Err(reason) => (),
            _ => assert!(false),
        }
    }
}
