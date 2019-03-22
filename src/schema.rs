table! {
    expenses (id) {
        id -> Integer,
        name -> Text,
        amount -> Float,
    }
}

table! {
    users (id) {
        id -> Integer,
        user_name -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    expenses,
    users,
);
