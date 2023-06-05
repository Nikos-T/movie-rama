mod movies;
mod users;
mod votes;

use anyhow::Result;

use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};

use crate::models::movies::{Movie, NewMovie};
use crate::models::users::{NewUser, User};
use crate::models::votes::Vote;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Self { pool }
    }

    pub fn create_user(&self, user: NewUser) -> Result<User> {
        users::create_user(&mut self.pool.get()?, user)
    }

    pub fn get_user(&self, id: i32) -> Result<User> {
        users::get_user(&mut self.pool.get()?, id)
    }

    pub fn get_user_by_email(&self, email: &str) -> Result<User> {
        users::get_user_by_email(&mut self.pool.get()?, email)
    }

    pub fn create_movie(&self, movie: NewMovie) -> Result<Movie> {
        movies::create_movie(&mut self.pool.get()?, movie)
    }

    pub fn get_all_movies(&self) -> Result<Vec<Movie>> {
        movies::get_all_movies(&mut self.pool.get()?)
    }

    pub fn vote(&self, vote: Vote) -> Result<Vote> {
        votes::vote(&mut self.pool.get()?, vote)
    }

    pub fn delete_vode(&self, (user_id, movie_id): (i32, i32)) -> Result<()> {
        votes::delete_vote(&mut self.pool.get()?, (user_id, movie_id))
    }

    pub fn get_movies_verbose(&self) -> Result<Vec<movies::MovieVerbose>> {
        movies::get_movies_verbose(&mut self.pool.get()?)
    }
}
