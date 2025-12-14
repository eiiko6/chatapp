use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn init_db() -> PgPool {
    let database_url = "postgres://chatapp:secret@localhost:5432/chatapp";
    PgPool::connect(database_url)
        .await
        .expect("Failed to connect to database")
}

pub async fn user_id_from_uuid(db: &PgPool, user_uuid: Uuid) -> Result<i32, (StatusCode, String)> {
    sqlx::query_scalar("SELECT id FROM user_ WHERE uuid = $1")
        .bind(user_uuid)
        .fetch_one(db)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, String::from("Wrong token")))
}
