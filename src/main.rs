use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use sqlx::PgPool;

pub struct AppState {
    db_con: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_con = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let app_state = Data::new(AppState { db_con });

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
