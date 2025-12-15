use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use password_hash::SaltString;
use password_hash::rand_core::OsRng;

use axum::http::{HeaderMap, StatusCode};
use uuid::Uuid;

const DEFAULT_SECRET_KEY: &str = "43aaf85b92f1ae6fbcef7732c50a0904";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())
        .map(|ph| ph.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).ok();
    if let Some(parsed_hash) = parsed_hash {
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    } else {
        false
    }
}

pub fn create_jwt(user_uuid: Uuid) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(100))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_uuid,
        exp: expiration as usize,
    };

    let secret =
        std::env::var("CHATAPP_JWT_SECRET").unwrap_or_else(|_| DEFAULT_SECRET_KEY.to_string());

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| "Token creation failed".into())
}

pub fn verify_jwt(headers: HeaderMap) -> Result<Claims, (StatusCode, String)> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    let secret =
        std::env::var("CHATAPP_JWT_SECRET").unwrap_or_else(|_| DEFAULT_SECRET_KEY.to_string());

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
}
