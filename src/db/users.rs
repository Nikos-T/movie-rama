use anyhow::Result;

use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::{PooledConnection, ConnectionManager};

use crate::models::users::{NewUser, User};
use crate::schema::users;

type PooledPgConn = PooledConnection<ConnectionManager<PgConnection>>;

pub(crate) fn create_user(conn: &mut PooledPgConn, user: NewUser) -> Result<User> {
    Ok(
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)?
    )
}
