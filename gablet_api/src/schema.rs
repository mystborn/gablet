// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_level"))]
    pub struct UserLevel;
}

diesel::table! {
    books (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        approved -> Bool,
        #[max_length = 255]
        small_thumbnail -> Nullable<Varchar>,
        #[max_length = 255]
        big_thumbnail -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserLevel;

    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        verified -> Bool,
        level -> UserLevel,
    }
}

diesel::joinable!(books -> users (id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    users,
);
