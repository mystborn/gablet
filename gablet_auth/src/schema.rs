// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_level"))]
    pub struct UserLevel;
}

diesel::table! {
    refresh_tokens (id) {
        id -> Int4,
        refresh_token -> Text,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        source -> Varchar,
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
        enabled -> Bool,
        created -> Timestamp,
        last_login -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    refresh_tokens,
    users,
);
