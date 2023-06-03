use actix_web::{web, App, HttpServer};
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT must be a 16 bit int");
    let path = std::env::var("STATIC_FILE_PATH").expect("STATIC_FILE_PATH must be set");
    let static_files = String::from(path.strip_suffix("/").unwrap_or(&path));
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            actix_files::Files::new("/", static_files.clone())
                .index_file("index.html")
                .default_handler(
                    actix_files::NamedFile::open(
                        vec![static_files.clone(), "index.html".to_string()].join("/"),
                    )
                    .expect("index file should exist"),
                ),
        )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
