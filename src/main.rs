mod config;
mod db;
mod auth;
mod utils;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use auth::routes::auth_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = db::init_pool()
        .await
        .expect("Failed to initialize DB pool");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .configure(auth_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}
