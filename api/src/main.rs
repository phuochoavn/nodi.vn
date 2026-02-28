use axum::Router;
use tower_http::cors::CorsLayer;
use axum::http::{HeaderValue, Method, header};
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

/// Security headers middleware
async fn security_headers(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.insert("x-content-type-options", HeaderValue::from_static("nosniff"));
    headers.insert("x-frame-options", HeaderValue::from_static("DENY"));
    headers.insert("x-xss-protection", HeaderValue::from_static("1; mode=block"));
    headers.insert("referrer-policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
    res
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

    let allowed_origins = [
        "https://nodi.vn",
        "https://www.nodi.vn",
        "https://api.nodi.vn",
    ]
    .map(|o| o.parse::<HeaderValue>().unwrap());

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins.to_vec())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT, header::ORIGIN])
        .allow_credentials(true);

    let app = Router::new()
        .merge(routes::health::router())
        .merge(routes::license::router())
        .merge(routes::auth::router())
        .merge(routes::account::router())
        .merge(routes::sync::router())
        .merge(routes::backup::router())
        .merge(routes::upload::router())
        .merge(routes::scanner::router())
        .merge(routes::dashboard::router())
        .merge(routes::admin::router())
        .merge(routes::market::router())
        .merge(routes::support::router())
        .merge(routes::ws_support::router())
        .merge(routes::store::router())
        .layer(axum::middleware::from_fn(security_headers))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(axum::extract::DefaultBodyLimit::max(50 * 1024 * 1024)) // 50MB
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 Nodi API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
