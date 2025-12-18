use axum::{
    Extension, Router,
    http::{Method, header},
};
use std::{env::var, net::SocketAddr, time::Duration};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::cors::{Any, CorsLayer};

mod auth;
mod db;
mod realtime;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::info!("Connecting to database...");
    let db_pool = db::init_db().await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(50)
        .burst_size(200)
        .finish()
        .unwrap();

    let governor_limiter = governor_conf.limiter().clone();

    // a separate background task to clean up
    let interval = Duration::from_secs(60);
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            // tracing::info!("rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    let realtime = realtime::Realtime::new();

    let app = Router::new()
        .merge(routes::users::routes())
        .merge(routes::rooms::routes())
        .merge(routes::messages::routes())
        .merge(routes::friends::routes())
        .merge(routes::ws::routes())
        .layer(Extension(db_pool))
        .layer(Extension(realtime))
        .layer(cors)
        .layer(GovernorLayer::new(governor_conf));

    let port = var("CHATAPP_SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("127.0.0.1:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on {addr}");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();

    Ok(())
}
