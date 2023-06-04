mod users;
mod movies;

use anyhow::Result;

use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};

use crate::models::movies::{NewMovie, Movie};
use crate::models::users::{NewUser, User};

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

    pub fn get_user_by_email(&self, email: &str) -> Result<User> {
        users::get_user_by_email(&mut self.pool.get()?, email)
    }

    pub fn create_movie(&self, movie: NewMovie) -> Result<Movie> {
        movies::create_movie(&mut self.pool.get()?, movie)
    }
}

