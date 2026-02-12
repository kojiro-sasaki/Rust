use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
struct BankAccount {
    balance: Arc<Mutex<i32>>,
}
impl BankAccount {
    fn new(initial: i32) -> Self {
        BankAccount {
            balance: Arc::new(Mutex::new(initial)),
        }
    }

    fn deposit(&self, amount: i32) {
        let mut bal = self.balance.lock().unwrap();
        *bal += amount;
    }

    fn withdraw(&self, amount: i32) {
        let mut bal = self.balance.lock().unwrap();

        if *bal < amount {
            panic!("Not enough money");
        }

        *bal -= amount;
    }

    fn get_balance(&self) -> i32 {
        let bal = self.balance.lock().unwrap();
        *bal
    }
}

fn main() {
    let account = BankAccount::new(200);

    println!("Balance: {}", account.get_balance());

    account.deposit(20);
    println!("Balance: {}", account.get_balance());

    account.withdraw(170);
    println!("Balance: {}", account.get_balance());
}
