use axum::{
    Extension, Json, Router,
    extract::Path,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{auth::verify_jwt, db::room_id_from_uuid, routes::rooms::is_member};
use crate::{
    db::{user_id_from_uuid, username_from_uuid},
    realtime::Realtime,
};

#[derive(sqlx::FromRow, serde::Serialize, Debug)]
pub struct MessageRow {
    pub sender: String,
    pub message_type: String,
    pub content: String,
    pub sent_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, serde::Serialize, Debug, Clone)]
pub struct Message {
    pub uuid: Uuid,
    pub sender: String,
    pub message_type: String,
    pub content: String,
    pub sent_at: String,
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

    if !is_member(user_id, room_id, &db).await {
        return Err((
            StatusCode::UNAUTHORIZED,
            String::from("You are not a member of this room"),
        ));
    }

    let messages = sqlx::query_as::<_, MessageRow>(
        r#"
    SELECT
        u.username AS sender,
        r.uuid AS room,
        m.message_type,
        m.content,
        m.sent_at
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

    let messages: Vec<Message> = messages
        .into_iter()
        .map(|m| Message {
            uuid: uuid::Uuid::now_v7(),
            sender: m.sender,
            message_type: m.message_type,
            content: m.content,
            sent_at: m.sent_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
        .collect();

    Ok(Json(messages))
}

async fn create_message(
    Path(room_uuid): Path<Uuid>,
    Extension(db): Extension<PgPool>,
    Extension(realtime): Extension<Realtime>,
    headers: HeaderMap,
    Json(payload): Json<NewMessagePayload>,
) -> Result<(StatusCode, Json<Message>), (StatusCode, String)> {
    let claims = verify_jwt(headers)?;

    let user_id = user_id_from_uuid(&db, claims.sub).await?;
    let room_id = room_id_from_uuid(&db, room_uuid).await?;

    if !is_member(user_id, room_id, &db).await {
        return Err((
            StatusCode::UNAUTHORIZED,
            String::from("You are not a member of this room"),
        ));
    }

    let sent_at: chrono::NaiveDateTime = sqlx::query_scalar(
        "INSERT INTO message_ (sender, room, message_type, content)
        VALUES ($1, $2, $3, $4) RETURNING sent_at",
    )
    .bind(user_id)
    .bind(room_id)
    .bind(&payload.message_type)
    .bind(&payload.content)
    .fetch_one(&db)
    .await
    .map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Could not create message: {e}"),
        )
    })?;

    let sender_name = username_from_uuid(&db, claims.sub).await?;

    let message = Message {
        uuid: uuid::Uuid::now_v7(),
        sender: sender_name,
        message_type: payload.message_type,
        content: payload.content,
        sent_at: sent_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    let rt_sender = realtime.sender_for(room_id);
    let _ = rt_sender.send(message.clone());

    Ok((StatusCode::CREATED, Json(message)))
}
