use axum::{
    Extension, Router,
    http::{
        Method,
        header::{self, CONTENT_TYPE},
    },
    middleware,
};
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use clap::Parser;
use std::{net::SocketAddr, time::Duration};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing::info;

mod auth;
mod db;
mod realtime;
mod routes;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Server port
    #[arg(short, long, default_value = "8080")]
    port: String,

    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Connecting to database...");
    let db_pool = db::init_db().await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    let governor_conf = GovernorConfigBuilder::default()
        .burst_size(15)
        .per_millisecond(250)
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

    let mut app = Router::new()
        .merge(routes::users::routes())
        .merge(routes::rooms::routes())
        .merge(routes::messages::routes())
        .merge(routes::friends::routes())
        .merge(routes::ws::routes())
        .layer(Extension(db_pool))
        .layer(Extension(realtime))
        .layer(GovernorLayer::new(governor_conf))
        .layer(cors);

    if cli.verbose {
        app = app
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            )
            .layer(middleware::from_fn(log_json_body));
    }

    let port = cli.port;
    let addr = format!("0.0.0.0:{port}");

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

#[cfg(debug_assertions)]
async fn log_json_body(req: Request, next: Next) -> Response {
    let (parts, body) = req.into_parts();

    // Check if the content type is JSON
    let is_json = parts
        .headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map_or(false, |v| v.contains("application/json"));

    let bytes = if is_json {
        // Read the body bytes
        let bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .unwrap_or_default();

        // Log the body (converting to string)
        if let Ok(body_str) = std::str::from_utf8(&bytes) {
            info!("JSON Request Body: {}", body_str);
        }
        bytes
    } else {
        // If not JSON, we still need to collect it or just pass it through
        axum::body::to_bytes(body, usize::MAX)
            .await
            .unwrap_or_default()
    };

    // Reconstruct the request with the bytes we read
    let req = Request::from_parts(parts, Body::from(bytes));

    next.run(req).await
}
