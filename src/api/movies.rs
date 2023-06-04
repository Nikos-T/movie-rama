use actix_web::{post, HttpResponse, get};
use actix_web::web::{Data, Json, ReqData};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::db::Database;
use crate::middleware::TokenClaims;
use crate::models::movies::{Movie, NewMovie};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MovieBody {
    pub id: i32,
    pub posted_by: i32,
    pub posted_at: NaiveDateTime,
    pub title: String,
    pub description: String,
}

impl From<Movie> for MovieBody {
    fn from(movie: Movie) -> Self {
        let Movie { id, posted_by, posted_at, title, description } = movie;
        Self { id, posted_by, posted_at, title, description }
    }
}

impl Into<Movie> for MovieBody {
    fn into(self) -> Movie {
        let MovieBody { id, posted_by, posted_at, title, description } = self;
        Movie { id, posted_by, posted_at, title, description }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewMovieBody {
    pub title: String,
    pub description: String,
}

#[post("/add_movie")]
pub async fn create_movie(db: Data<Database>, req_user: Option<ReqData<TokenClaims>>, movie: Json<NewMovieBody>) -> HttpResponse {
    let movie = movie.into_inner();
    let movie = match req_user {
        Some(user) => {
            NewMovie {
                posted_by: user.user_id,
                title: &movie.title,
                description: &movie.description
            }
        }
        _ => { return HttpResponse::Unauthorized().json("Log in to add a movie."); }
    };

    match db.create_movie(movie) {
        Ok(movie) => HttpResponse::Ok().json(MovieBody::from(movie)),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}

// TODO pagination
#[get("/movies")]
pub async fn get_all_movies(db: Data<Database>) -> HttpResponse {
    match db.get_all_movies() {
        Ok(movies) => HttpResponse::Ok().json(movies.into_iter().map(MovieBody::from).collect::<Vec<_>>()),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}

