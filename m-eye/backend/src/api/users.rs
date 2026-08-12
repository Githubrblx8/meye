use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::core::AppState;
use crate::models::UserRole;

#[derive(Serialize, OpenApi, FromRow)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    responses(
        (status = 200, description = "List of users", body = Vec<UserResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin only")
    )
)]
async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let users = sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT id, email, role::text as role, created_at
        FROM users
        ORDER BY created_at DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while listing users: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    )
)]
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT id, email, role::text as role, created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while fetching user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/users", get(list_users))
        .route("/api/v1/users/:id", get(get_user))
}
