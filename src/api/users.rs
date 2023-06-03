use actix_web::{post, HttpResponse};
use actix_web::web::{Data, Json};
use serde::{Serialize, Deserialize};

use crate::db::Database;
use crate::models::users::NewUser as NewUserModel;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}

impl<'a> Into<NewUserModel<'a>> for &'a NewUser {
    fn into(self) -> NewUserModel<'a> {
        let NewUser {
            email,
            password_hash,
            first_name,
            last_name
        } = self;

        NewUserModel {
            email,
            password_hash,
            first_name,
            last_name,
        }
    }
}

/// Body: [NewUser]
/// Response: new user's id on success
/// TODO: jwt
#[post("/create_user")]
pub async fn create_user(db: Data<Database>, user: Json<NewUser>) -> HttpResponse {
    let user = user.into_inner();
    let user: NewUserModel = (&user).into();
    match db.create_user(user) {
        Ok(user) => HttpResponse::Ok().json(user.id),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}


