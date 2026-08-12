//! Reports endpoints

use axum::{Json, Router, extract::State, http::StatusCode};
use meye_db::{DatabasePool, report::ReportRepository};
use meye_models::{EvidenceType, ReportCategory};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn routes() -> Router<DatabasePool> {
    Router::new()
        .route("/", axum::routing::post(create_report).get(list_reports))
        .route("/pending", axum::routing::get(list_pending_reports))
}

#[derive(Deserialize)]
struct CreateReportRequest {
    target_type: String,
    target_value: String,
    category: String,
    description: String,
}

#[derive(Serialize)]
struct ReportResponse {
    id: Uuid,
    target_type: String,
    target_value: String,
    category: String,
    description: String,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// Create a new report
async fn create_report(
    State(pool): State<DatabasePool>,
    Json(payload): Json<CreateReportRequest>,
) -> Result<Json<ReportResponse>, StatusCode> {
    // In a real implementation, get user_id from JWT claims
    let fake_user_id = Uuid::nil(); // TODO: Get from auth context
    
    let repo = ReportRepository::new(&pool.pool);
    
    // Parse target type
    let target_type = parse_identity_type(&payload.target_type)
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    // Parse category
    let category = parse_category(&payload.category)
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let report = repo
        .create(
            fake_user_id,
            target_type,
            &payload.target_value,
            category,
            &payload.description,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ReportResponse {
        id: report.id,
        target_type: format!("{:?}", report.target_type),
        target_value: report.target_value,
        category: format!("{:?}", report.category),
        description: report.description,
        status: format!("{:?}", report.status),
        created_at: report.created_at,
    }))
}

/// List all reports
async fn list_reports(
    State(pool): State<DatabasePool>,
) -> Result<Json<Vec<ReportResponse>>, StatusCode> {
    // In a real implementation, filter by user_id from JWT claims
    let fake_user_id = Uuid::nil();
    
    let repo = ReportRepository::new(&pool.pool);
    
    let reports = repo
        .list_by_user(fake_user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(reports.into_iter().map(|r| ReportResponse {
        id: r.id,
        target_type: format!("{:?}", r.target_type),
        target_value: r.target_value,
        category: format!("{:?}", r.category),
        description: r.description,
        status: format!("{:?}", r.status),
        created_at: r.created_at,
    }).collect()))
}

/// List pending reports (for moderators)
async fn list_pending_reports(
    State(pool): State<DatabasePool>,
) -> Result<Json<Vec<ReportResponse>>, StatusCode> {
    let repo = ReportRepository::new(&pool.pool);
    
    let reports = repo
        .list_pending()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(reports.into_iter().map(|r| ReportResponse {
        id: r.id,
        target_type: format!("{:?}", r.target_type),
        target_value: r.target_value,
        category: format!("{:?}", r.category),
        description: r.description,
        status: format!("{:?}", r.status),
        created_at: r.created_at,
    }).collect()))
}

fn parse_identity_type(s: &str) -> Option<meye_models::IdentityType> {
    match s.to_lowercase().as_str() {
        "email" => Some(meye_models::IdentityType::Email),
        "domain" => Some(meye_models::IdentityType::Domain),
        "ip" => Some(meye_models::IdentityType::Ip),
        "url" => Some(meye_models::IdentityType::Url),
        _ => None,
    }
}

fn parse_category(s: &str) -> Option<ReportCategory> {
    match s.to_lowercase().as_str() {
        "spam" => Some(ReportCategory::Spam),
        "phishing" => Some(ReportCategory::Phishing),
        "malware" => Some(ReportCategory::Malware),
        "credentialharvesting" => Some(ReportCategory::CredentialHarvesting),
        "impersonation" => Some(ReportCategory::Impersonation),
        "fraud" => Some(ReportCategory::Fraud),
        "compromisedaccount" => Some(ReportCategory::CompromisedAccount),
        "suspicious" => Some(ReportCategory::Suspicious),
        "other" => Some(ReportCategory::Other),
        _ => None,
    }
}
