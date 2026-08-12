use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use utoipa::OpenApi;

use crate::core::AppState;

#[derive(Serialize, OpenApi)]
struct HealthResponse {
    status: String,
    version: String,
    database: String,
    redis: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let db_status = state.db.ping().await.unwrap_or_else(|_| "disconnected".to_string());
    let redis_status = state.redis.ping().await.unwrap_or_else(|_| "disconnected".to_string());

    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
        redis: redis_status,
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/health", get(health_check))
}
