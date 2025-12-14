use axum::{
    Extension, Json, Router,
    extract::Path,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::verify_jwt;
use crate::db::user_id_from_uuid;

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Room {
    pub uuid: Uuid,
    pub owner: i32,
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct NewRoomPayload {
    pub name: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/rooms/{user_uuid}", get(list_rooms))
        .route("/rooms", post(create_room))
        .route("/rooms/{user_uuid}/{room_id}", get(get_room))
}

async fn list_rooms(
    Path(user_uuid): Path<Uuid>,
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Room>>, (StatusCode, String)> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    let claims = verify_jwt(token)?;
    if claims.sub != user_uuid {
        return Err((StatusCode::FORBIDDEN, "Forbidden".to_string()));
    }

    let user_id = user_id_from_uuid(&db, claims.sub).await?;

    let rooms = sqlx::query_as::<_, Room>("SELECT uuid, owner, name FROM room_ WHERE owner = $1")
        .bind(user_id)
        .fetch_all(&db)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("faied to list rooms: {e}");
            Vec::new()
        });

    Ok(Json(rooms))
}

async fn create_room(
    Extension(db): Extension<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<NewRoomPayload>,
) -> Result<(StatusCode, Json<Room>), (StatusCode, String)> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    // Verify auth
    let claims = verify_jwt(token)?;

    let user_id = user_id_from_uuid(&db, claims.sub).await?;

    let room_uuid = uuid::Uuid::now_v7();

    sqlx::query(
        "INSERT INTO room_ (uuid, owner, name)
        VALUES ($1, $2, $3)",
    )
    .bind(room_uuid)
    .bind(user_id)
    .bind(&payload.name)
    .execute(&db)
    .await
    .map_err(|_| (StatusCode::BAD_REQUEST, format!("Could not create room")))?;

    Ok((
        StatusCode::CREATED,
        Json(Room {
            uuid: room_uuid,
            owner: user_id,
            name: payload.name,
        }),
    ))
}

async fn get_room(
    Path((user_uuid, room_uuid)): Path<(Uuid, Uuid)>,
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Room>, (StatusCode, String)> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    // Verify auth
    let claims = verify_jwt(token)?;
    if claims.sub != user_uuid {
        return Err((StatusCode::FORBIDDEN, "Forbidden".to_string()));
    }

    let user_id = user_id_from_uuid(&db, user_uuid).await?;

    let room: Room =
        sqlx::query_as("SELECT uuid, owner, name FROM room_ WHERE uuid = $1 AND owner = $2")
            .bind(room_uuid)
            .bind(user_id)
            .fetch_one(&db)
            .await
            .map_err(|_| (StatusCode::NOT_FOUND, "Room not found".to_string()))?;

    Ok(Json(Room {
        uuid: room_uuid,
        owner: room.owner,
        name: room.name,
    }))
}
