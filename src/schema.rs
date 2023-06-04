// @generated automatically by Diesel CLI.

diesel::table! {
    movies (id) {
        id -> Int4,
        posted_by -> Int4,
        posted_at -> Timestamp,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        #[max_length = 119]
        password_hash -> Bpchar,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

diesel::joinable!(movies -> users (posted_by));

diesel::allow_tables_to_appear_in_same_query!(
    movies,
    users,
);
