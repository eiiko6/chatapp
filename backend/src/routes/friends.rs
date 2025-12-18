use axum::{
    Extension, Json, Router,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::user_id_from_uuid;
use crate::{auth::verify_jwt, db::id_from_username};

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Friend {
    pub uuid: Uuid,
    pub username: String,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct FriendRequest {
    pub sender_uuid: Uuid,
    pub sender_username: String,
}

#[derive(serde::Deserialize)]
pub struct SendFriendRequestPayload {
    pub receiver_username: String,
}

#[derive(serde::Deserialize)]
pub struct AcceptFriendRequestPayload {
    pub sender_uuid: Uuid,
}

pub fn routes() -> Router {
    Router::new()
        .route("/friends", get(list_friends))
        .route("/friends/requests", get(list_requests))
        .route("/friends/request", post(send_request))
        .route("/friends/accept", post(accept_request))
}

async fn list_friends(
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Friend>>, (StatusCode, String)> {
    let claims = verify_jwt(headers)?;
    let user_id = user_id_from_uuid(&db, claims.sub).await?;

    let friends = sqlx::query_as::<_, Friend>(
        r#"
        SELECT u.uuid, u.username
        FROM friendship_
        JOIN user_ u
          ON (u.id = friendship_.user_first AND friendship_.user_second = $1)
          OR (u.id = friendship_.user_second AND friendship_.user_first = $1)
        "#,
    )
    .bind(user_id)
    .fetch_all(&db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not list friends".into(),
        )
    })?;

    Ok(Json(friends))
}

async fn list_requests(
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<FriendRequest>>, (StatusCode, String)> {
    let claims = verify_jwt(headers)?;
    let user_id = user_id_from_uuid(&db, claims.sub).await?;

    let requests = sqlx::query_as::<_, FriendRequest>(
        r#"
        SELECT u.uuid AS sender_uuid, u.username AS sender_username
        FROM friend_request_
        JOIN user_ u ON u.id = friend_request_.sender
        WHERE friend_request_.receiver = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(&db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not list friend requests".into(),
        )
    })?;

    Ok(Json(requests))
}

async fn send_request(
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
    Json(payload): Json<SendFriendRequestPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    let claims = verify_jwt(headers)?;

    let sender_id = user_id_from_uuid(&db, claims.sub).await?;
    let receiver_id = id_from_username(&db, payload.receiver_username).await?;

    if sender_id == receiver_id {
        return Err((
            StatusCode::BAD_REQUEST,
            "Cannot send a friend request to yourself".into(),
        ));
    }

    sqlx::query("INSERT INTO friend_request_ (sender, receiver) VALUES ($1, $2)")
        .bind(sender_id)
        .bind(receiver_id)
        .execute(&db)
        .await
        .map_err(|_| (StatusCode::CONFLICT, "Request already exists".into()))?;

    Ok(StatusCode::CREATED)
}

async fn accept_request(
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
    Json(payload): Json<AcceptFriendRequestPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    let claims = verify_jwt(headers)?;

    let receiver_id = user_id_from_uuid(&db, claims.sub).await?;
    let sender_id = user_id_from_uuid(&db, payload.sender_uuid).await?;

    let (first, second) = if sender_id < receiver_id {
        (sender_id, receiver_id)
    } else {
        (receiver_id, sender_id)
    };

    let mut tx = db
        .begin()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?;

    let rows = sqlx::query("DELETE FROM friend_request_ WHERE sender = $1 AND receiver = $2")
        .bind(sender_id)
        .bind(receiver_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?
        .rows_affected();

    if rows == 0 {
        return Err((StatusCode::NOT_FOUND, "No such request".into()));
    }

    sqlx::query("INSERT INTO friendship_ (user_first, user_second) VALUES ($1, $2)")
        .bind(first)
        .bind(second)
        .execute(&mut *tx)
        .await
        .map_err(|_| (StatusCode::CONFLICT, "Already friends".into()))?;

    tx.commit().await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not accept friendship".into(),
        )
    })?;

    Ok(StatusCode::CREATED)
}
