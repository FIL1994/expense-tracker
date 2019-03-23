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

use bcrypt::{hash, verify, DEFAULT_COST};
use flate2::Compression;

pub mod models;
pub mod schema;

use models::{Expense, NewExpense, NewUser, User};
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
        fn get_expenses(&self, param: ApiParam) -> Result<Vec<Expense>, ()> {
            let results = expenses
                .load::<Expense>(&param.connection)
                .expect("Error loading expenses");

            Ok(results)
        }

        #[post("/expenses")]
        #[content_type("json")]
        fn post_expenses(&self, param: ApiParam, body: Vec<u8>) -> Result<Expense, ()> {
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

        #[post("/users")]
        #[content_type("json")]
        fn post_user(&self, param: ApiParam, body: Vec<u8>) -> Result<User, ()> {
            let json_string:&str = str::from_utf8(&body).unwrap();
            let mut user: NewUser = serde_json::from_str(json_string).unwrap();

            let password = &hash(user.password, 9).unwrap();
            user.password = password;

            diesel::insert_into(schema::users::table)
                        .values(&user)
                        .execute(&param.connection)
                        .expect("Error saving expense");

            use schema::users::dsl::id;
            let inserted_user:User = schema::users::table.order(id.desc()).first(&param.connection).unwrap();
            Ok(inserted_user)
        }
    }
}

struct ApiConfig {}
struct ApiParam {
    connection: diesel::SqliteConnection,
}

impl<B: BufStream> Extract<B> for ApiParam {
    type Future = Immediate<ApiParam>;

    fn extract(context: &Context) -> Self::Future {
        let _config = context.config::<ApiConfig>().unwrap();

        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env file");
        let conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        let param = ApiParam { connection: conn };
        Immediate::ok(param)
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(ExpenseTrackerAPI)
        .config(ApiConfig {})
        .middleware(DeflateMiddleware::new(Compression::best()))
        .run(&addr)
        .unwrap();
}
