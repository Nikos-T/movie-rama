use anyhow::Result;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{PgConnection, RunQueryDsl};

use crate::models::users::{NewUser, User};
use crate::schema::users;

type PooledPgConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_user(conn: &mut PooledPgConn, user: NewUser) -> Result<User> {
    Ok(diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)?)
}

pub fn get_user_by_email(conn: &mut PooledPgConn, email: &str) -> Result<User> {
    Ok(users::table.filter(users::email.eq(email)).first(conn)?)
}
