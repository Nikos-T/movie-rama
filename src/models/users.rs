use diesel::{AsChangeset, Insertable, Queryable};

use crate::schema::users;

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name=users)]
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
