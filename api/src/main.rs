use axum::Router;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;
use tokio::sync::RwLock;

mod config;
mod db;
mod error;
mod routes;
mod models;
mod middleware;

/// A sender that can push text frames to a connected WebSocket client.
pub type WsSender = tokio::sync::mpsc::UnboundedSender<String>;

/// Shared chat rooms: ticket_id → list of (sender_type, sender_name, ws_sender)
pub type ChatRooms = Arc<RwLock<HashMap<i32, Vec<(String, String, WsSender)>>>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub config: Arc<config::Config>,
    pub start_time: Instant,
    pub chat_rooms: ChatRooms,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("nodi_api=debug,tower_http=debug")
        .init();

    dotenvy::dotenv().ok();
    let config = config::Config::from_env();

    let pool = db::create_pool(&config.database_url).await;

    // Seed admin user if ADMIN_PASSWORD is set
    db::seed_admin(&pool).await;

    let state = AppState {
        pool,
        config: Arc::new(config),
        start_time: Instant::now(),
        chat_rooms: Arc::new(RwLock::new(HashMap::new())),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(routes::health::router())
        .merge(routes::license::router())
        .merge(routes::auth::router())
        .merge(routes::sync::router())
        .merge(routes::backup::router())
        .merge(routes::dashboard::router())
        .merge(routes::admin::router())
        .merge(routes::market::router())
        .merge(routes::support::router())
        .merge(routes::ws_support::router())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(axum::extract::DefaultBodyLimit::max(50 * 1024 * 1024)) // 50MB
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 Nodi API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
