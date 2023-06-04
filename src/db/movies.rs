use anyhow::Result;

use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::{PooledConnection, ConnectionManager};

use crate::models::movies::{NewMovie, Movie};
use crate::schema::movies;

type PooledPgConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_movie(conn: &mut PooledPgConn, movie: NewMovie) -> Result<Movie> {
    Ok(
        diesel::insert_into(movies::table)
            .values(&movie)
            .get_result(conn)?
    )
}

