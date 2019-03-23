use super::schema::expenses;
use super::schema::users;

#[derive(Debug, Queryable, Response, Extract, Serialize)]
pub struct Expense {
    pub id: i32,
    pub name: String,
    pub amount: f32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "expenses"]
pub struct NewExpense<'a> {
    pub name: &'a str,
    pub amount: f32,
}

#[derive(Debug, Queryable, Response, Extract, Serialize)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub password: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    #[serde(rename = "userName")]
    pub user_name: &'a str,
    pub password: &'a str,
}
