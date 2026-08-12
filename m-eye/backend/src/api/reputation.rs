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
use crate::models::{IdentityType, ReputationStatus};

#[derive(Serialize, OpenApi, FromRow)]
pub struct ReputationResponse {
    pub identity: String,
    pub identity_type: String,
    pub status: String,
    pub risk_score: i32,
    pub confidence: f64,
    pub reports_count: i32,
    pub confirmed_incidents: i32,
}

#[utoipa::path(
    get,
    path = "/api/v1/reputation/email/{email}",
    params(
        ("email" = String, Path, description = "Email address to check")
    ),
    responses(
        (status = 200, description = "Reputation found", body = ReputationResponse),
        (status = 404, description = "Reputation not found")
    )
)]
async fn get_email_reputation(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<Json<ReputationResponse>, StatusCode> {
    let reputation = sqlx::query_as::<_, ReputationResponse>(
        r#"
        SELECT 
            identity,
            identity_type::text as identity_type,
            status::text as status,
            risk_score,
            confidence,
            reports_count,
            confirmed_incidents
        FROM reputations
        WHERE identity = $1 AND identity_type = 'email'
        LIMIT 1
        "#,
    )
    .bind(&email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while fetching email reputation: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match reputation {
        Some(rep) => Ok(Json(rep)),
        None => {
            // Return UNKNOWN status for new identities
            Ok(Json(ReputationResponse {
                identity: email,
                identity_type: "email".to_string(),
                status: "UNKNOWN".to_string(),
                risk_score: 0,
                confidence: 0.0,
                reports_count: 0,
                confirmed_incidents: 0,
            }))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/reputation/domain/{domain}",
    params(
        ("domain" = String, Path, description = "Domain to check")
    ),
    responses(
        (status = 200, description = "Reputation found", body = ReputationResponse),
        (status = 404, description = "Reputation not found")
    )
)]
async fn get_domain_reputation(
    State(state): State<AppState>,
    Path(domain): Path<String>,
) -> Result<Json<ReputationResponse>, StatusCode> {
    let reputation = sqlx::query_as::<_, ReputationResponse>(
        r#"
        SELECT 
            identity,
            identity_type::text as identity_type,
            status::text as status,
            risk_score,
            confidence,
            reports_count,
            confirmed_incidents
        FROM reputations
        WHERE identity = $1 AND identity_type = 'domain'
        LIMIT 1
        "#,
    )
    .bind(&domain)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while fetching domain reputation: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match reputation {
        Some(rep) => Ok(Json(rep)),
        None => {
            Ok(Json(ReputationResponse {
                identity: domain,
                identity_type: "domain".to_string(),
                status: "UNKNOWN".to_string(),
                risk_score: 0,
                confidence: 0.0,
                reports_count: 0,
                confirmed_incidents: 0,
            }))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/reputation/ip/{ip}",
    params(
        ("ip" = String, Path, description = "IP address to check")
    ),
    responses(
        (status = 200, description = "Reputation found", body = ReputationResponse),
        (status = 404, description = "Reputation not found")
    )
)]
async fn get_ip_reputation(
    State(state): State<AppState>,
    Path(ip): Path<String>,
) -> Result<Json<ReputationResponse>, StatusCode> {
    let reputation = sqlx::query_as::<_, ReputationResponse>(
        r#"
        SELECT 
            identity,
            identity_type::text as identity_type,
            status::text as status,
            risk_score,
            confidence,
            reports_count,
            confirmed_incidents
        FROM reputations
        WHERE identity = $1 AND identity_type = 'ip'
        LIMIT 1
        "#,
    )
    .bind(&ip)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error while fetching IP reputation: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match reputation {
        Some(rep) => Ok(Json(rep)),
        None => {
            Ok(Json(ReputationResponse {
                identity: ip,
                identity_type: "ip".to_string(),
                status: "UNKNOWN".to_string(),
                risk_score: 0,
                confidence: 0.0,
                reports_count: 0,
                confirmed_incidents: 0,
            }))
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/reputation/email/:email", get(get_email_reputation))
        .route("/api/v1/reputation/domain/:domain", get(get_domain_reputation))
        .route("/api/v1/reputation/ip/:ip", get(get_ip_reputation))
}
