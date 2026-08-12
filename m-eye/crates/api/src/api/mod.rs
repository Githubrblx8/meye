//! API routes module

pub mod health;
pub mod auth;
pub mod reputation;
pub mod reports;
pub mod middleware;

use axum::Router;
use meye_db::DatabasePool;

/// Combine all routes
pub fn routes() -> Router<DatabasePool> {
    Router::new()
        .nest("/api/v1/health", health::routes())
        .nest("/api/v1/auth", auth::routes())
        .nest("/api/v1/reputation", reputation::routes())
        .nest("/api/v1/reports", reports::routes())
}
