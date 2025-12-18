use axum::Json;
use axum::extract::Query;
use axum::extract::ws::{Message as WsMessage, WebSocket};
use axum::http::HeaderMap;
use axum::routing::get;
use axum::{
    Extension,
    extract::{Path, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::{create_jwt, verify_jwt};
use crate::db::user_id_from_uuid;
use crate::routes::rooms::is_member;
use crate::{db::room_id_from_uuid, realtime::Realtime};

#[derive(sqlx::FromRow, serde::Serialize, Deserialize)]
pub struct WsAuthQuery {
    pub token: String,
}

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/ws/issue-token/rooms/{room_uuid}", get(issue_ws_token))
        .route("/ws/rooms/{room_uuid}", get(ws_handler))
}

pub async fn issue_ws_token(
    Extension(db): Extension<sqlx::PgPool>,
    headers: HeaderMap,
    Path(room_uuid): Path<Uuid>,
) -> Result<(StatusCode, Json<WsAuthQuery>), (StatusCode, String)> {
    let claims = verify_jwt(headers)?;

    let room_id = room_id_from_uuid(&db, room_uuid).await?;
    let user_id = user_id_from_uuid(&db, claims.sub).await?;

    if !is_member(user_id, room_id, &db).await {
        return Err((
            StatusCode::UNAUTHORIZED,
            String::from("You are not a member of this room"),
        ));
    }

    // tracing::info!(
    //     "recieved token issue request from user {} for room {}",
    //     claims.sub,
    //     room_uuid
    // );

    let token = create_jwt(claims.sub).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    sqlx::query(
        r#"
        insert into ws_token_ (token, room_id, expires_at)
        values ($1, $2, now() + interval '30 seconds')
        "#,
    )
    .bind(&token)
    .bind(room_id)
    .execute(&db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to provide ws token"),
        )
    })?;

    Ok((StatusCode::CREATED, Json(WsAuthQuery { token })))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(room_uuid): Path<Uuid>,
    Query(query): Query<WsAuthQuery>,
    Extension(realtime): Extension<Realtime>,
    Extension(db): Extension<sqlx::PgPool>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    // tracing::info!("recieved ws handshake: {}", room_uuid);

    let room_id = room_id_from_uuid(&db, room_uuid)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let valid: Option<i32> = sqlx::query_scalar(
        r#"
        delete from ws_token_
        where token = $1
          and room_id = $2
          and expires_at > now()
        returning room_id
        "#,
    )
    .bind(query.token)
    .bind(room_id)
    .fetch_optional(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if valid.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let sender = realtime.sender_for(room_id);
    let receiver = sender.subscribe();

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, receiver)))
}

async fn handle_socket(
    mut socket: WebSocket,
    mut receiver: tokio::sync::broadcast::Receiver<crate::routes::messages::Message>,
) {
    while let Ok(msg) = receiver.recv().await {
        if socket
            .send(WsMessage::Text(serde_json::to_string(&msg).unwrap().into()))
            .await
            .is_err()
        {
            break;
        }
    }
}
