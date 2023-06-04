use actix_web::dev::ServiceRequest;
use actix_web::error::Error;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::{self, BearerAuth};
use actix_web_httpauth::extractors::AuthenticationError;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Clone, Deserialize, Serialize)]
pub struct TokenClaims {
    pub user_id: i32,
}

pub async fn validator(
    req: ServiceRequest,
    creds: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_str = creds.token();

    let claims: Result<TokenClaims, &str> =
        token_str.verify_with_key(&key).map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
