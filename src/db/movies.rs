use anyhow::Result;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::sql_types::*;
use diesel::FromSqlRow;
use diesel::{PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::models::movies::{Movie, NewMovie};
use crate::schema::movies;

type PooledPgConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_movie(conn: &mut PooledPgConn, movie: NewMovie) -> Result<Movie> {
    Ok(diesel::insert_into(movies::table)
        .values(&movie)
        .get_result(conn)?)
}

// TODO: pagination
pub fn get_all_movies(conn: &mut PooledPgConn) -> Result<Vec<Movie>> {
    Ok(movies::table.load::<Movie>(conn)?)
}

#[derive(QueryableByName, FromSqlRow, Deserialize, Serialize)]
pub struct MovieVerbose {
    #[diesel(sql_type = Integer)]
    pub movie_id: i32,
    #[diesel(sql_type = Integer)]
    pub posted_by: i32,
    #[diesel(sql_type = Timestamp)]
    pub posted_at: NaiveDateTime,
    #[diesel(sql_type = Text)]
    pub title: String,
    #[diesel(sql_type = Text)]
    pub description: String,
    #[diesel(sql_type = BigInt)]
    pub positive_votes: i64,
    #[diesel(sql_type = BigInt)]
    pub negative_votes: i64,
    #[diesel(sql_type = Text)]
    pub first_name: String,
    #[diesel(sql_type = Text)]
    pub last_name: String,
}

pub fn get_movies_verbose(conn: &mut PooledPgConn) -> Result<Vec<MovieVerbose>> {
    // return movies joined with user first_name, last_name
    Ok(
        diesel::sql_query(
            r#"
            SELECT
                movies.id AS movie_id,
                movies.posted_by,
                movies.posted_at,
                movies.title,
                movies.description,
                COALESCE(SUM(CASE WHEN votes.vote = 'positive' THEN 1 ELSE 0 END), 0) AS positive_votes,
                COALESCE(SUM(CASE WHEN votes.vote = 'negative' THEN 1 ELSE 0 END), 0) AS negative_votes,
                users.first_name,
                users.last_name
            FROM
                movies
            LEFT JOIN
                votes ON movies.id = votes.movie_id
            LEFT JOIN
                users ON movies.posted_by = users.id
            GROUP BY
                movies.id, users.id
            ORDER BY
                movies.posted_at
            "#
        )
        .load(conn)?
    )
}
