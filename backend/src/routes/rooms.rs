use axum::{
    Extension, Json, Router,
    extract::Path,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

use crate::db::{id_from_username, room_name_from_uuid, user_id_from_uuid, username_from_id};
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

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct RoomInvite {
    pub room_uuid: Uuid,
    pub room_name: String,
    pub sender_uuid: Uuid,
    pub sender_username: String,
}

#[derive(serde::Deserialize)]
pub struct SendRoomInvitePayload {
    pub room_uuid: Uuid,
    pub receiver_username: String,
}

#[derive(serde::Deserialize)]
pub struct AcceptRoomInvitePayload {
    pub room_uuid: Uuid,
    pub sender_uuid: Uuid,
}

pub fn routes() -> Router {
    Router::new()
        .route("/rooms", get(list_rooms))
        .route("/rooms", post(create_room))
        .route("/rooms/{room_id}", get(get_room))
        .route("/rooms/invites", get(list_invites))
        .route("/rooms/invite", post(send_invite))
        .route("/rooms/join", post(accept_request))
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
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Room>>, (StatusCode, String)> {
    let claims = verify_jwt(headers)?;
    if claims.sub != claims.sub {
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

async fn get_room(
    Path(room_uuid): Path<Uuid>,
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Room>, (StatusCode, String)> {
    let claims = verify_jwt(headers)?;

    let user_id = user_id_from_uuid(&db, claims.sub).await?;

    let room: Room = sqlx::query_as(
        r#"
        SELECT uuid, u.name AS owner_name, r.name, r.global
        FROM room_ r
        JOIN user u ON u.id = r.owner
        WHERE uuid = $1 AND owner = $2
        "#,
    )
    .bind(room_uuid)
    .bind(user_id)
    .fetch_one(&db)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "Room not found".to_string()))?;

    Ok(Json(Room {
        uuid: room_uuid,
        owner_name: room.owner_name,
        name: room.name,
        global: room.global,
    }))
}

async fn list_invites(
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<RoomInvite>>, (StatusCode, String)> {
    let claims = verify_jwt(headers)?;
    let user_id = user_id_from_uuid(&db, claims.sub).await?;

    let requests = sqlx::query_as::<_, RoomInvite>(
        r#"
        SELECT
            r.uuid AS room_uuid,
            r.name AS room_name,
            u.uuid AS sender_uuid,
            u.username AS sender_username
        FROM room_invite_ AS i
        JOIN user_ u ON u.id = i.sender
        JOIN room_ r ON r.id = i.room
        WHERE i.receiver = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(&db)
    .await
    .map_err(|e| {
        tracing::error!("{e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not list room invites".into(),
        )
    })?;

    Ok(Json(requests))
}

async fn send_invite(
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
    Json(payload): Json<SendRoomInvitePayload>,
) -> Result<(StatusCode, Json<RoomInvite>), (StatusCode, String)> {
    let claims = verify_jwt(headers)?;

    let sender_id = user_id_from_uuid(&db, claims.sub).await?;
    let receiver_id = id_from_username(&db, payload.receiver_username).await?;
    let room_id = room_id_from_uuid(&db, payload.room_uuid).await?;

    if sender_id == receiver_id {
        return Err((
            StatusCode::BAD_REQUEST,
            "Cannot send a room invite to yourself".into(),
        ));
    }

    let is_already_member = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM membership_
            WHERE user_id = $1
            AND room = $2
        )
        "#,
    )
    .bind(receiver_id)
    .bind(room_id)
    .fetch_one(&db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()))?;

    if is_already_member {
        return Err((
            StatusCode::CONFLICT,
            "This user is already a member of this room".into(),
        ));
    }

    sqlx::query("INSERT INTO room_invite_ (sender, receiver, room) VALUES ($1, $2, $3)")
        .bind(sender_id)
        .bind(receiver_id)
        .bind(room_id)
        .execute(&db)
        .await
        .map_err(|_| (StatusCode::CONFLICT, "Request already exists".into()))?;

    tracing::info!("bro");

    let room_name = room_name_from_uuid(&db, payload.room_uuid).await?;

    Ok((
        StatusCode::CREATED,
        Json(RoomInvite {
            room_uuid: payload.room_uuid,
            room_name,
            sender_uuid: claims.sub,
            sender_username: username_from_id(&db, receiver_id).await?,
        }),
    ))
}

async fn accept_request(
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
    Json(payload): Json<AcceptRoomInvitePayload>,
) -> Result<(StatusCode, Json<Room>), (StatusCode, String)> {
    let claims = verify_jwt(headers)?;

    let receiver_id = user_id_from_uuid(&db, claims.sub).await?;
    let sender_id = user_id_from_uuid(&db, payload.sender_uuid).await?;

    let mut tx = db
        .begin()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?;

    let rows = sqlx::query(
        r#"
        DELETE FROM room_invite_
        WHERE sender = $1 AND receiver = $2
        "#,
    )
    .bind(sender_id)
    .bind(receiver_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?
    .rows_affected();

    if rows == 0 {
        return Err((StatusCode::NOT_FOUND, "No such invite".into()));
    }

    let room_id = room_id_from_uuid(&db, payload.room_uuid).await?;

    sqlx::query("INSERT INTO membership_ (user_id, room) VALUES ($1, $2)")
        .bind(receiver_id)
        .bind(room_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| {
            (
                StatusCode::CONFLICT,
                "Error creating room membership".into(),
            )
        })?;

    let room: Room = sqlx::query_as(
        r#"
        SELECT r.uuid, u.username AS owner_name, r.name, r.global
        FROM room_ r
        JOIN user_ u ON u.id = r.owner
        WHERE r.id = $1 AND r.owner = $2
        "#,
    )
    .bind(room_id)
    .bind(sender_id)
    .fetch_one(&db)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "Room not found".into()))?;

    tx.commit().await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not accept room invite".into(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(Room {
            uuid: payload.room_uuid,
            owner_name: room.owner_name,
            name: room.name,
            global: room.global,
        }),
    ))
}
