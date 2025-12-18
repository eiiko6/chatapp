use axum::{
    Extension, Json, Router,
    extract::Path,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

use crate::db::user_id_from_uuid;
use crate::{auth::verify_jwt, db::room_id_from_uuid};

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Room {
    pub uuid: Uuid,
    pub owner_name: String,
    pub name: String,
    pub global: bool,
}

#[derive(serde::Deserialize)]
pub struct NewRoomPayload {
    pub name: String,
    pub global: bool,
}

pub fn routes() -> Router {
    Router::new()
        .route("/rooms/{user_uuid}", get(list_rooms))
        .route("/rooms", post(create_room))
    // .route("/rooms/{user_uuid}/{room_id}", get(get_room))
}

pub async fn is_member(user_id: i32, room_id: i32, db: &Pool<Postgres>) -> bool {
    sqlx::query_scalar(
        r#"
        SELECT r.global OR EXISTS (
            SELECT 1
            FROM membership_ m
            WHERE m.user_id = $1
            AND m.room = r.id
        )
        FROM room_ r
        WHERE r.id = $2
        "#,
    )
    .bind(user_id)
    .bind(room_id)
    .fetch_one(db)
    .await
    .unwrap_or(false)
}

async fn list_rooms(
    Path(user_uuid): Path<Uuid>,
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Room>>, (StatusCode, String)> {
    let claims = verify_jwt(headers)?;
    if claims.sub != user_uuid {
        return Err((StatusCode::FORBIDDEN, "Forbidden".to_string()));
    }

    let user_id = user_id_from_uuid(&db, claims.sub).await?;

    let rooms = sqlx::query_as::<_, Room>(
        r#"
        SELECT r.uuid,
               u.username AS owner_name,
               r.name,
               r.global
        FROM room_ r
        JOIN user_ u ON u.id = r.owner
        WHERE r.global OR EXISTS (
            SELECT 1
            FROM membership_ m
            WHERE m.user_id = $1 AND m.room = r.id
        )
        "#,
    )
    .bind(user_id)
    .fetch_all(&db)
    .await
    .unwrap_or(Vec::new());

    Ok(Json(rooms))
}

async fn create_room(
    Extension(db): Extension<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<NewRoomPayload>,
) -> Result<(StatusCode, Json<Room>), (StatusCode, String)> {
    let claims = verify_jwt(headers)?;

    let user_id = user_id_from_uuid(&db, claims.sub).await?;

    let room_uuid = uuid::Uuid::now_v7();

    sqlx::query(
        "INSERT INTO room_ (uuid, owner, name, global)
        VALUES ($1, $2, $3, $4)",
    )
    .bind(room_uuid)
    .bind(user_id)
    .bind(&payload.name)
    .bind(&payload.global)
    .execute(&db)
    .await
    .map_err(|_| (StatusCode::BAD_REQUEST, format!("Could not create room")))?;

    let room_id = room_id_from_uuid(&db, room_uuid).await?;

    // We do this even for the owner
    sqlx::query("INSERT INTO membership_ (user_id, room) VALUES ($1, $2)")
        .bind(user_id)
        .bind(room_id)
        .execute(&db)
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, format!("Could not create room")))?;

    let owner_name = sqlx::query_scalar("SELECT username FROM user_ WHERE id = $1")
        .bind(user_id)
        .fetch_one(&db)
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, format!("Could not create room")))?;

    Ok((
        StatusCode::CREATED,
        Json(Room {
            uuid: room_uuid,
            owner_name,
            name: payload.name,
            global: payload.global,
        }),
    ))
}

// async fn get_room(
//     Path((user_uuid, room_uuid)): Path<(Uuid, Uuid)>,
//     headers: HeaderMap,
//     Extension(db): Extension<PgPool>,
// ) -> Result<Json<Room>, (StatusCode, String)> {
//     let claims = verify_jwt(headers)?;
//     if claims.sub != user_uuid {
//         return Err((StatusCode::FORBIDDEN, "Forbidden".to_string()));
//     }
//
//     let user_id = user_id_from_uuid(&db, user_uuid).await?;
//
//     let room: Room =
//         sqlx::query_as("SELECT uuid, owner, name FROM room_ WHERE uuid = $1 AND owner = $2")
//             .bind(room_uuid)
//             .bind(user_id)
//             .fetch_one(&db)
//             .await
//             .map_err(|_| (StatusCode::NOT_FOUND, "Room not found".to_string()))?;
//
//     Ok(Json(Room {
//         uuid: room_uuid,
//         owner: room.owner,
//         name: room.name,
//     }))
// }
