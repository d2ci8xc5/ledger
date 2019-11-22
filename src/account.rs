pub struct Account {
    name: String,
    balance: u64,
    id: i32,
}

impl Account {
    fn new(name: String, balance: u64, id: i32) -> Account {
        let mut acc = Account { name, balance, id };
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let acc = Account::new(String::from("a0"), 100: u64, 0: i32);
    }
}
