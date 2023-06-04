// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "vote"))]
    pub struct Vote;
}

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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Vote;

    votes (user_id, movie_id) {
        user_id -> Int4,
        movie_id -> Int4,
        vote -> Vote,
    }
}

diesel::joinable!(movies -> users (posted_by));
diesel::joinable!(votes -> movies (movie_id));
diesel::joinable!(votes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(movies, users, votes,);
