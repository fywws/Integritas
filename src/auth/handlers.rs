use crate::auth::models::{AuthResponse, LoginRequest, RegisterRequest};
use crate::auth::services::{generate_jwt, hash_password, verify_password};
use actix_web::{web, HttpResponse};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    let hashed_password = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let query = "INSERT INTO users (id, email, password) VALUES ($1, $2, $3)";
    let user_id = Uuid::new_v4();

    if let Err(_) = sqlx::query(query)
        .bind(user_id)
        .bind(&req.email)
        .bind(hashed_password)
        .execute(pool.get_ref())
        .await
    {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    let query = "SELECT id, password FROM users WHERE email = $1";
    let row = match sqlx::query(query)
        .bind(&req.email)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(Some(row)) => row,
        _ => return HttpResponse::Unauthorized().finish(),
    };

    let user_id: Uuid = row.get("id");
    let stored_password: String = row.get("password");

    if !verify_password(&stored_password, &req.password).unwrap_or(false) {
        return HttpResponse::Unauthorized().finish();
    }

    let token = generate_jwt(&user_id.to_string());

    HttpResponse::Ok().json(AuthResponse { token })
}
