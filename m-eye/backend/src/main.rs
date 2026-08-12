use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod core;
mod models;
mod smtp;
mod workers;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting M'Eye API server");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database connection pool
    let db_pool = core::db::create_pool()
        .await
        .expect("Failed to create database pool");

    // Initialize Redis connection
    let redis_client = core::cache::create_client()
        .await
        .expect("Failed to create Redis client");

    // Initialize JWT manager
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "m-eye-default-secret-change-in-production".to_string());
    let jwt_issuer = std::env::var("JWT_ISSUER")
        .unwrap_or_else(|_| "m-eye-api".to_string());
    let jwt_lifetime: i64 = std::env::var("JWT_LIFETIME_SECS")
        .unwrap_or_else(|_| "3600".to_string())
        .parse()
        .unwrap_or(3600);

    let jwt_manager = Arc::new(core::jwt::JwtManager::new(
        &jwt_secret,
        jwt_issuer,
        jwt_lifetime,
    ));

    tracing::info!("JWT authentication enabled with {} second lifetime", jwt_lifetime);

    // Build router
    let app = Router::new()
        .merge(api::health::router())
        .merge(api::auth::router())
        .merge(api::reputation::router())
        .merge(api::reports::router())
        .merge(api::users::router())
        .merge(api::moderation::router())
        .layer(TraceLayer::new_for_http())
        .with_state(core::AppState {
            db: db_pool,
            redis: redis_client,
            jwt_manager,
        });

    // Get host and port from environment
    let host = std::env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("API_PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("{}:{}", host, port);

    tracing::info!("Listening on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.unwrap();
}
