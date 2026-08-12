use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::core::AppState;
use crate::models::{IdentityType, ReportCategory, ReputationStatus};

#[derive(Deserialize, OpenApi)]
pub struct CreateReportRequest {
    pub target: String,
    pub target_type: IdentityType,
    pub category: ReportCategory,
    pub description: String,
}

#[derive(Serialize, OpenApi)]
pub struct ReportResponse {
    pub id: Uuid,
    pub target: String,
    pub target_type: String,
    pub category: String,
    pub description: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, OpenApi)]
pub struct CreateReportResponse {
    pub id: Uuid,
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/reports",
    request_body = CreateReportRequest,
    responses(
        (status = 201, description = "Report created successfully", body = CreateReportResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    )
)]
async fn create_report(
    State(state): State<AppState>,
    Json(payload): Json<CreateReportRequest>,
) -> Result<Json<CreateReportResponse>, StatusCode> {
    let report_id = Uuid::new_v4();
    
    // Insert report into database
    sqlx::query(
        r#"
        INSERT INTO reports (id, target, target_type, category, description, reporter_id, status)
        VALUES ($1, $2, $3, $4, $5, $6, 'pending')
        "#,
    )
    .bind(report_id)
    .bind(&payload.target)
    .bind(&payload.target_type)
    .bind(&payload.category)
    .bind(&payload.description)
    // In a real implementation, we would get this from the authenticated user
    .bind(Uuid::nil()) // reporter_id - should come from auth context
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while creating report: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(CreateReportResponse {
        id: report_id,
        message: "Report created successfully".to_string(),
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/reports",
    responses(
        (status = 200, description = "List of reports", body = Vec<ReportResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
async fn list_reports(
    State(state): State<AppState>,
) -> Result<Json<Vec<ReportResponse>>, StatusCode> {
    let reports = sqlx::query_as::<_, ReportResponse>(
        r#"
        SELECT 
            id,
            target,
            target_type::text as target_type,
            category::text as category,
            description,
            status::text as status,
            created_at
        FROM reports
        ORDER BY created_at DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while listing reports: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(reports))
}

#[utoipa::path(
    get,
    path = "/api/v1/reports/{id}",
    params(
        ("id" = Uuid, Path, description = "Report ID")
    ),
    responses(
        (status = 200, description = "Report found", body = ReportResponse),
        (status = 404, description = "Report not found")
    )
)]
async fn get_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ReportResponse>, StatusCode> {
    let report = sqlx::query_as::<_, ReportResponse>(
        r#"
        SELECT 
            id,
            target,
            target_type::text as target_type,
            category::text as category,
            description,
            status::text as status,
            created_at
        FROM reports
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while fetching report: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match report {
        Some(rep) => Ok(Json(rep)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/reports", post(create_report))
        .route("/api/v1/reports", get(list_reports))
        .route("/api/v1/reports/:id", get(get_report))
}
