use axum::{
    Extension, Json, Router,
    extract::Path,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{user_id_from_uuid, username_from_uuid};
use crate::{auth::verify_jwt, db::room_id_from_uuid};

#[derive(sqlx::FromRow, serde::Serialize, Debug)]
pub struct Message {
    pub sender: String,
    pub message_type: String,
    pub content: String,
}

#[derive(serde::Deserialize)]
pub struct NewMessagePayload {
    pub message_type: String,
    pub content: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/messages/{room_uuid}", get(list_messages))
        .route("/messages/{room_uuid}", post(create_message))
}

async fn list_messages(
    Path(room_uuid): Path<Uuid>,
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let claims = verify_jwt(headers)?;

    let user_id = user_id_from_uuid(&db, claims.sub).await?;
    let room_id = room_id_from_uuid(&db, room_uuid).await?;

    let membership: Vec<i32> =
        sqlx::query_scalar("SELECT user_id FROM membership_ WHERE user_id = $1 AND room = $2")
            .bind(user_id)
            .bind(room_id)
            .fetch_all(&db)
            .await
            .unwrap_or_else(|_| Vec::new());

    if membership.is_empty() {
        return Err((
            StatusCode::UNAUTHORIZED,
            String::from("You are not a member of this room"),
        ));
    }

    let messages = sqlx::query_as::<_, Message>(
        r#"
    SELECT
        u.username AS sender,
        r.uuid AS room,
        m.type AS message_type,
        m.content
    FROM message_ m
    JOIN user_ u ON u.id = m.sender
    JOIN room_ r ON r.id = m.room
    WHERE m.room = $1
    ORDER BY m.id
    "#,
    )
    .bind(room_id)
    .fetch_all(&db)
    .await
    .map_err(|e| {
        tracing::error!("failed to list messages: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to list messages".into(),
        )
    })?;

    Ok(Json(messages))
}

async fn create_message(
    Path(room_uuid): Path<Uuid>,
    Extension(db): Extension<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<NewMessagePayload>,
) -> Result<(StatusCode, Json<Message>), (StatusCode, String)> {
    let claims = verify_jwt(headers)?;

    let user_id = user_id_from_uuid(&db, claims.sub).await?;

    let room_id = room_id_from_uuid(&db, room_uuid).await?;

    sqlx::query(
        "INSERT INTO message_ (sender, room, type, content)
        VALUES ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(room_id)
    .bind(&payload.message_type)
    .bind(&payload.content)
    .execute(&db)
    .await
    .map_err(|_| (StatusCode::BAD_REQUEST, format!("Could not create message")))?;

    let sender_name = username_from_uuid(&db, claims.sub).await?;

    Ok((
        StatusCode::CREATED,
        Json(Message {
            sender: sender_name,
            message_type: payload.message_type,
            content: payload.content,
        }),
    ))
}
