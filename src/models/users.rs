use crate::schema::users;
use diesel::{Insertable, Queryable, AsChangeset};

#[derive(Debug, Clone, Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name=users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
}
