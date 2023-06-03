use actix_web::{post, HttpResponse};
use actix_web::web::{Data, Json};

use crate::db::Database;

#[post("/add_movie")]
pub async fn create_movie(db: Data<Database>) -> HttpResponse {
    todo!();
}
