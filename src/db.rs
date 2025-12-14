use sqlx::PgPool;

pub async fn init_db() -> PgPool {
    let database_url = "postgres://chatapp:secret@localhost:5432/chatapp";
    PgPool::connect(database_url)
        .await
        .expect("Failed to connect to database")
}
