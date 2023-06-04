mod users;
pub use users::NewUserBody;
mod movies;

use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::middleware::validator;

pub fn config(cfg: &mut web::ServiceConfig) {
    let bearer_middleware = HttpAuthentication::bearer(validator);
    cfg.service(
        web::scope("/api")
            .service(users::create_user)
            .service(users::login)
            .service(movies::get_all_movies)
            .service(
                web::scope("")
                    .wrap(bearer_middleware)
                    .service(movies::create_movie),
            )
    );
}
