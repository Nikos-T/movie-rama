use anyhow::{bail, Result};

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{PgConnection, RunQueryDsl};

use crate::models::movies::Movie;
use crate::models::votes::Vote;
use crate::schema::{movies, votes};

type PooledPgConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn vote(conn: &mut PooledPgConn, vote: Vote) -> Result<Vote> {
    // First if movie_id is posted_by user_id return error
    let movie = movies::table
        .filter(movies::id.eq(vote.movie_id))
        .first::<Movie>(conn)?;
    if movie.posted_by == vote.user_id {
        bail!("You cannot vote for your own movie");
    }

    // Insert or update existing vote as long as the movie does not belong to this user
    let existing_vote = votes::table
        .filter(votes::user_id.eq(vote.user_id))
        .filter(votes::movie_id.eq(vote.movie_id))
        .first::<Vote>(conn);
    match existing_vote {
        Ok(existing_vote) => Ok(diesel::update(&existing_vote)
            .set(votes::vote.eq(vote.vote))
            .get_result(conn)?),
        Err(_) => Ok(diesel::insert_into(votes::table)
            .values(vote)
            .get_result(conn)?),
    }
}

pub fn delete_vote(conn: &mut PooledPgConn, (user_id, movie_id): (i32, i32)) -> Result<()> {
    diesel::delete(votes::table)
        .filter(votes::user_id.eq(user_id))
        .filter(votes::movie_id.eq(movie_id))
        .execute(conn)?;
    Ok(())
}

pub fn get_votes_by_user(conn: &mut PooledPgConn, user_id: i32) -> Result<Vec<Vote>> {
    let votes = votes::table
        .filter(votes::user_id.eq(user_id))
        .load::<Vote>(conn)?;
    Ok(votes)
}

