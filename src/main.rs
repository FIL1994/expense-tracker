#[macro_use]
extern crate diesel;

use currency::Currency;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use models::{Expense, NewExpense};
use schema::expenses::dsl::expenses;

fn main() {
    println!("Expense Tracker");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env file");
    let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let expense = NewExpense {
        name: "Expense 1",
        amount: 12,
    };

    diesel::insert_into(schema::expenses::table)
        .values(&expense)
        .execute(&connection)
        .expect("Error saving expense");

    let results = expenses
        .limit(1)
        .load::<Expense>(&connection)
        .expect("Error loading expenses");

    println!("Displaying {} expenses", results.len());
    for expense in results {
        println!("{}", expense.name);
    }
}
