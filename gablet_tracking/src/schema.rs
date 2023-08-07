// @generated automatically by Diesel CLI.

diesel::table! {
    book_views (id) {
        id -> Int4,
        book_id -> Int4,
        chapter_id -> Int4,
        user_id -> Int4,
        #[max_length = 50]
        os -> Varchar,
        #[max_length = 50]
        device -> Varchar,
        ip -> Inet,
        dt -> Timestamp,
    }
}

diesel::table! {
    daily_book_views (id) {
        id -> Int4,
        count -> Int4,
        book_id -> Int4,
        chapter_id -> Int4,
        date -> Date,
    }
}

diesel::table! {
    daily_user_views (id) {
        id -> Int4,
        count -> Int4,
        user_id -> Int4,
        date -> Date,
    }
}

diesel::table! {
    daily_web_views (id) {
        id -> Int4,
        count -> Int4,
        href -> Text,
        date -> Date,
    }
}

diesel::table! {
    user_views (id) {
        id -> Int4,
        viewer_id -> Int4,
        user_id -> Int4,
        #[max_length = 50]
        os -> Varchar,
        #[max_length = 50]
        device -> Varchar,
        ip -> Inet,
        dt -> Timestamp,
    }
}

diesel::table! {
    web_views (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 50]
        browser -> Varchar,
        #[max_length = 50]
        os -> Varchar,
        #[max_length = 50]
        device -> Varchar,
        ip -> Inet,
        href -> Text,
        #[max_length = 100]
        domain -> Varchar,
        dt -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    book_views,
    daily_book_views,
    daily_user_views,
    daily_web_views,
    user_views,
    web_views,
);
