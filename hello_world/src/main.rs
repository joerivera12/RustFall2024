mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(100.0);
    
    println!("Initial balance: ${:.2}", account.balance());

    account.deposit(50.0);
    println!("After depositing $50: ${:.2}", account.balance());

    account.withdraw(30.0);
    println!("After withdrawing $30: ${:.2}", account.balance());

    account.withdraw(150.0); // Attempt to withdraw more than the balance
    println!("After trying to withdraw $150: ${:.2}", account.balance());

    account.deposit(-20.0); // Attempt to deposit a negative amount
    println!("After trying to deposit -$20: ${:.2}", account.balance());
}