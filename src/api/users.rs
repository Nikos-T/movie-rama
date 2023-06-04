// TODO: reading from env every time is slow
use actix_web::{post, HttpResponse};
use actix_web::web::{Data, Json};
use argonautica::{Hasher, Verifier};
use hmac::Hmac;
use hmac::digest::KeyInit;
use serde::{Serialize, Deserialize};
use sha2::Sha256;
use jwt::SignWithKey;

use crate::db::Database;
use crate::middleware::TokenClaims;
use crate::models::users::{NewUser, User};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserBody {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl From<User> for UserBody {
    fn from(user: User) -> Self {
        let User {
            id,
            email,
            password_hash: _,
            first_name,
            last_name
        } = user;

        Self {
            id,
            email,
            first_name,
            last_name,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewUserBody {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginUserBody {
    pub email: String,
    pub password: String,
}

/// Body: [NewUser]
/// Response: new user's id on success
#[post("/signup")]
pub async fn create_user(db: Data<Database>, user: Json<NewUserBody>) -> HttpResponse {
    let user = user.into_inner();

    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");
    let mut hasher = Hasher::default();
    let password_hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

    let user = NewUser {
        email: &user.email,
        password_hash: &password_hash,
        first_name: &user.first_name,
        last_name: &user.last_name,
    };

    match db.create_user(user) {
        Ok(user) => {
            // Shortcut to login directly on sign-up
            let jwt_secret = Hmac::<Sha256>::new_from_slice(
                std::env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_bytes()
            ).unwrap();
            let claims = TokenClaims { user_id: user.id };
            let token_str = claims.sign_with_key(&jwt_secret).unwrap();
            // TODO return a better format
            HttpResponse::Ok().json((UserBody::from(user), token_str))
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/login")]
pub async fn login(db: Data<Database>, login_user: Json<LoginUserBody>) -> HttpResponse {
    let login_user = login_user.into_inner();

    let jwt_secret = Hmac::<Sha256>::new_from_slice(
        std::env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_bytes()
    ).unwrap();

    let User {
        id,
        email,
        password_hash,
        first_name,
        last_name
    } = match db.get_user_by_email(&login_user.email) {
        Ok(u) => u,
        Err(e) => return HttpResponse::Unauthorized().body(e.to_string()),
    };

    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");
    let mut verifier = Verifier::default();
    
    match verifier.with_hash(&password_hash).with_password(login_user.password).with_secret_key(hash_secret).verify() {
        Err(e) => {
            eprintln!("Error verifying password: {}", e);
            HttpResponse::InternalServerError().body("Error verifying password".to_string())
        }
        Ok(false) => HttpResponse::Unauthorized().body("Invalid password".to_string()),
        Ok(true) => {
            let claims = TokenClaims { user_id: id };
            let token_str = claims.sign_with_key(&jwt_secret).unwrap();
            let user = UserBody {
                id,
                email,
                first_name,
                last_name,
            };
            // TODO return a better format
            HttpResponse::Ok().json((user, token_str))
        }
    }
}

