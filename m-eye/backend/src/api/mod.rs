use axum::{
    routing::{get, post},
    Router,
};

pub mod auth;
pub mod health;
pub mod middleware;
pub mod moderation;
pub mod reports;
pub mod reputation;
pub mod users;

pub fn routers() -> Router<crate::core::AppState> {
    Router::new()
        .merge(health::router())
        .merge(auth::router())
        .merge(reputation::router())
        .merge(reports::router())
        .merge(users::router())
        .merge(moderation::router())
}
