use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let database_url = "postgres://chatapp:secret@localhost:5432/chatapp";
    PgPool::connect(database_url).await
}

pub async fn user_id_from_uuid(db: &PgPool, user_uuid: Uuid) -> Result<i32, (StatusCode, String)> {
    sqlx::query_scalar("SELECT id FROM user_ WHERE uuid = $1")
        .bind(user_uuid)
        .fetch_one(db)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, String::from("Wrong token")))
}

pub async fn room_id_from_uuid(db: &PgPool, room_uuid: Uuid) -> Result<i32, (StatusCode, String)> {
    sqlx::query_scalar("SELECT id FROM room_ WHERE uuid = $1")
        .bind(room_uuid)
        .fetch_one(db)
        .await
        // FIX: hmm probably the wrong error here
        .map_err(|_| (StatusCode::UNAUTHORIZED, String::from("Wrong token")))
}

pub async fn username_from_uuid(
    db: &PgPool,
    user_uuid: Uuid,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("SELECT username FROM user_ WHERE uuid = $1")
        .bind(user_uuid)
        .fetch_one(db)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, String::from("Wrong token")))
}

pub async fn username_from_id(db: &PgPool, user_id: i32) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("SELECT username FROM user_ WHERE id = $1")
        .bind(user_id)
        .fetch_one(db)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, String::from("Wrong token")))
}

pub async fn id_from_username(db: &PgPool, username: String) -> Result<i32, (StatusCode, String)> {
    sqlx::query_scalar("SELECT id FROM user_ WHERE username = $1")
        .bind(username)
        .fetch_one(db)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".into()))
}
