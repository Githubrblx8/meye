use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde::Serialize;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::core::AppState;

#[derive(Serialize, OpenApi)]
pub struct ModerationResponse {
    pub id: Uuid,
    pub status: String,
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/moderation/{id}/approve",
    params(
        ("id" = Uuid, Path, description = "Report or suggestion ID")
    ),
    responses(
        (status = 200, description = "Approved successfully", body = ModerationResponse),
        (status = 404, description = "Item not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Moderator only")
    )
)]
async fn approve(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ModerationResponse>, StatusCode> {
    // Try to approve as a report first
    let updated = sqlx::query(
        r#"
        UPDATE reports
        SET status = 'confirmed', updated_at = NOW()
        WHERE id = $1 AND status = 'pending'
        RETURNING id
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while approving report: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if updated.is_some() {
        return Ok(Json(ModerationResponse {
            id,
            status: "approved".to_string(),
            message: "Report confirmed successfully".to_string(),
        }));
    }

    // Try to approve as a suggestion
    let updated = sqlx::query(
        r#"
        UPDATE suggestions
        SET status = 'approved', moderator_id = NULL, moderated_at = NOW()
        WHERE id = $1 AND status = 'pending'
        RETURNING id
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while approving suggestion: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if updated.is_some() {
        return Ok(Json(ModerationResponse {
            id,
            status: "approved".to_string(),
            message: "Suggestion approved successfully".to_string(),
        }));
    }

    Err(StatusCode::NOT_FOUND)
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/moderation/{id}/reject",
    params(
        ("id" = Uuid, Path, description = "Report or suggestion ID")
    ),
    responses(
        (status = 200, description = "Rejected successfully", body = ModerationResponse),
        (status = 404, description = "Item not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Moderator only")
    )
)]
async fn reject(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ModerationResponse>, StatusCode> {
    // Try to reject as a report first
    let updated = sqlx::query(
        r#"
        UPDATE reports
        SET status = 'rejected', updated_at = NOW()
        WHERE id = $1 AND status = 'pending'
        RETURNING id
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while rejecting report: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if updated.is_some() {
        return Ok(Json(ModerationResponse {
            id,
            status: "rejected".to_string(),
            message: "Report rejected successfully".to_string(),
        }));
    }

    // Try to reject as a suggestion
    let updated = sqlx::query(
        r#"
        UPDATE suggestions
        SET status = 'rejected', moderator_id = NULL, moderated_at = NOW()
        WHERE id = $1 AND status = 'pending'
        RETURNING id
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while rejecting suggestion: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if updated.is_some() {
        return Ok(Json(ModerationResponse {
            id,
            status: "rejected".to_string(),
            message: "Suggestion rejected successfully".to_string(),
        }));
    }

    Err(StatusCode::NOT_FOUND)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/admin/moderation/:id/approve", post(approve))
        .route("/api/v1/admin/moderation/:id/reject", post(reject))
}
