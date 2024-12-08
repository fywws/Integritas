use std::cell::OnceCell;
use std::sync::LazyLock;
use crate::config::jwt_secret;
use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use dotenvy::var;
use rand_core::OsRng;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};

static SALT_STRING: LazyLock<String> = LazyLock::new(|| {
    var("SALT").unwrap()
});

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::new(&*SALT_STRING.to_string());
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt?)
        .map(|hashed| hashed.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
        .map(|_| true)
        .or_else(|_| Ok(false))
}
pub fn generate_jwt(user_id: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Failed to create expiration date")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_ref()),
    )
        .expect("Failed to generate token")
}
