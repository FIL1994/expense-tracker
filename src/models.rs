use super::schema::expenses;

#[derive(Debug, Queryable)]
pub struct Expense {
    pub id: i32,
    pub name: String,
    pub amount: f32,
}

#[derive(Insertable)]
#[table_name = "expenses"]
pub struct NewExpense<'a> {
    pub name: &'a str,
    pub amount: f32,
}
