// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        #[max_length = 60]
        password_hash -> Bpchar,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}
