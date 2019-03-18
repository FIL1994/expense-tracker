#[macro_use]
extern crate diesel;
#[macro_use]
extern crate tower_web;

use diesel::prelude::*;
use dotenv::dotenv;
use std::{env, str};

use tower_web::extract::{Context, Extract, Immediate};
use tower_web::middleware::deflate::DeflateMiddleware;
use tower_web::util::BufStream;
use tower_web::ServiceBuilder;

use flate2::Compression;

pub mod models;
pub mod schema;

use models::{Expense, NewExpense};
use schema::expenses::dsl::expenses;

#[derive(Clone, Debug)]
struct ExpenseTrackerAPI;

impl_web! {
    impl ExpenseTrackerAPI {
        #[get("/")]
        fn home(&self) -> Result<String, ()> {
            Ok("Expense Tracker API".to_string())
        }

        #[get("/expenses")]
        #[content_type("json")]
        fn get_expenses(&self, param: API_Param) -> Result<Vec<Expense>, ()> {
            let results = expenses
                .load::<Expense>(&param.connection)
                .expect("Error loading expenses");

            Ok(results)
        }

        #[post("/expenses")]
        #[content_type("json")]
        fn post_expenses(&self, param: API_Param, body: Vec<u8>) -> Result<Expense, ()> {
            let json_string:&str = str::from_utf8(&body).unwrap();
            let expense: NewExpense = serde_json::from_str(json_string).unwrap();
            diesel::insert_into(schema::expenses::table)
                        .values(&expense)
                        .execute(&param.connection)
                        .expect("Error saving expense");

            use schema::expenses::dsl::id;
            let inserted_expense:Expense = schema::expenses::table.order(id.desc()).first(&param.connection).unwrap();
            Ok(inserted_expense)
        }
    }
}

struct API_Config {}
struct API_Param {
    connection: diesel::SqliteConnection,
}

impl<B: BufStream> Extract<B> for API_Param {
    type Future = Immediate<API_Param>;

    fn extract(context: &Context) -> Self::Future {
        let _config = context.config::<API_Config>().unwrap();

        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env file");
        let conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        let param = API_Param { connection: conn };
        Immediate::ok(param)
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(ExpenseTrackerAPI)
        .config(API_Config {})
        .middleware(DeflateMiddleware::new(Compression::best()))
        .run(&addr)
        .unwrap();
}
