pub mod api;
pub mod db;
pub mod middleware;
pub mod models;
pub mod schema;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: db::Database,
    jwt_secret: String,
    hash_secret: String
}

impl AppState {
    pub fn new(db: db::Database, jwt_secret: String, hash_secret: String) -> Self {
        Self {
            db,
            jwt_secret,
            hash_secret
        }
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }

    pub fn hash_secret(&self) -> &str {
        &self.hash_secret
    }
}

