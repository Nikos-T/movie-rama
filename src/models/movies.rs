use diesel::{Insertable, Queryable, AsChangeset};
use chrono::NaiveDateTime;

use crate::schema::movies;

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(belongs_to(User), table_name=movies)]
pub struct Movie {
    pub id: i32,
    pub posted_by: i32,
    pub posted_at: NaiveDateTime,
    pub title: String,
    pub description: String
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name=movies)]
pub struct NewMovie<'a> {
    pub posted_by: i32,
    pub title: &'a str,
    pub description: &'a str,
}
