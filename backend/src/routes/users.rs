use axum::{
    Extension, Json, Router,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
    routing::{get, post},
};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;
use validator::ValidateEmail;

use crate::auth::{create_jwt, hash_password, validate_token, verify_password};

const DUMMY_HASH: &str = "$argon2id$v=19$m=4096,t=3,p=1$YWFhYWFhYWFhYWFhYWFhYQ$aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub password_hash: String,
    pub email: String,
}

#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub uuid: Uuid,
    pub token: String,
}

#[derive(serde::Deserialize)]
pub struct NewUserPayload {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register_user))
        .route("/validate-token", get(validate_token))
        .layer(axum::middleware::from_fn(registration_guard))
}

async fn registration_guard(req: Request, next: Next) -> Result<Response, StatusCode> {
    if req.uri().path() == "/register"
        && env::var("CHATAPP_ALLOW_REGISTRATION").map_or(true, |v| v.to_lowercase() == "false")
    {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(next.run(req).await)
}

pub async fn login(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT uuid, email, username, password_hash FROM user_ WHERE email = $1",
    )
    .bind(&payload.email)
    .fetch_optional(&db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?;

    let (user_uuid, password_hash) = if let Some(u) = user {
        (u.uuid, u.password_hash)
    } else {
        // timing shield
        (uuid::Uuid::now_v7(), DUMMY_HASH.to_string())
    };

    if !verify_password(&password_hash, &payload.password) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into()));
    }

    let token = create_jwt(user_uuid).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(LoginResponse {
        uuid: user_uuid,
        token,
    }))
}

pub async fn register_user(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<NewUserPayload>,
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, String)> {
    if payload.email.is_empty() || payload.username.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Cannot create a user with empty fields".into(),
        ));
    }

    if !ValidateEmail::validate_email(&payload.email) {
        return Err((StatusCode::BAD_REQUEST, "Invalid email format".into()));
    }

    if payload.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password must be at least 8 characters long".into(),
        ));
    }

    let password_hash =
        hash_password(&payload.password).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let user_uuid = uuid::Uuid::now_v7();

    sqlx::query(
        "INSERT INTO user_ (uuid, username, email, password_hash)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(user_uuid)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .execute(&db)
    .await
    .map_err(|e| {
        if let Some(db_err) = e.as_database_error() {
            if db_err.code().map(|c| c == "23505").unwrap_or(false) {
                return (
                    StatusCode::CONFLICT,
                    "Email or username already taken".into(),
                );
            }
        }
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let token = create_jwt(user_uuid).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok((
        StatusCode::CREATED,
        Json(LoginResponse {
            uuid: user_uuid,
            token,
        }),
    ))
}
