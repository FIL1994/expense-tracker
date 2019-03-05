use currency::Currency;

#[derive(Debug)]
struct Expense {
    id: i32,
    name: String,
    amount: Currency,
}

fn main() {
    println!("Expense Tracker");

    let expense = Expense {
        id: 1,
        name: "Expense 1".to_string(),
        amount: Currency::from_str("$12.00").unwrap(),
    };

    println!("{:?}", expense);
    println!("{}", expense.amount);
}
