//! Health check endpoints

use axum::{Json, Router, extract::State, http::StatusCode};
use serde_json::json;
use meye_db::DatabasePool;

pub fn routes() -> Router<DatabasePool> {
    Router::new()
        .route("/", axum::routing::get(health_check))
        .route("/ready", axum::routing::get(readiness_check))
}

/// Basic health check
async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "m-eye-api"
    }))
}

/// Readiness check (database connection)
async fn readiness_check(State(pool): State<DatabasePool>) -> Result<Json<serde_json::Value>, StatusCode> {
    if pool.health_check().await {
        Ok(Json(json!({
            "status": "ready",
            "database": "connected",
            "redis": "connected"
        })))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
