use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, path::Path};

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .expect("DB_PORT must be a valid u16"),
            username: env::var("DB_USERNAME").unwrap_or_else(|_| "postgres".to_string()),
            password: env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string()),
            database: env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string()),
        }
    }

    pub fn to_connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

/// Функция для создания пула подключений
pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    load_env_var();

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&DatabaseConfig::from_env().to_connection_string())
        .await
}

pub fn load_env_var() {
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("config/.env");
    dotenvy::from_path(env_path).expect("> The ENV path is not set in db.rs")
}