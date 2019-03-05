use currency::Currency;
use rusqlite::Connection;

#[derive(Debug)]
struct Expense {
    id: i32,
    name: String,
    amount: Currency,
}

fn main() {
    println!("Expense Tracker");

    let conn = Connection::open_in_memory().unwrap();

    let expense = Expense {
        id: 1,
        name: "Expense 1".to_string(),
        amount: Currency::from_str("$12.00").unwrap(),
    };

    println!("{:?}", expense);
    println!("{}", expense.amount);
}
