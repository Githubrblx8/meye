//! Reputation endpoints

use axum::{Json, Router, Path, Query, extract::State, http::StatusCode};
use meye_db::{DatabasePool, reputation::ReputationRepository};
use meye_models::{IdentityType, ReputationStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn routes() -> Router<DatabasePool> {
    Router::new()
        .route("/email/:email", axum::routing::get(get_reputation_by_email))
        .route("/domain/:domain", axum::routing::get(get_reputation_by_domain))
        .route("/ip/:ip", axum::routing::get(get_reputation_by_ip))
        .route("/search", axum::routing::get(search_reputations))
        .route("/", axum::routing::get(list_reputations))
}

#[derive(Deserialize)]
struct SearchParams {
    q: String,
}

#[derive(Serialize)]
struct ReputationResponse {
    id: Uuid,
    identity_type: String,
    identity_value: String,
    status: String,
    risk_score: i32,
    confidence: f64,
    reports_count: i32,
    first_seen: chrono::DateTime<chrono::Utc>,
    last_seen: chrono::DateTime<chrono::Utc>,
}

impl From<meye_models::Reputation> for ReputationResponse {
    fn from(rep: meye_models::Reputation) -> Self {
        Self {
            id: rep.id,
            identity_type: format!("{:?}", rep.identity_type),
            identity_value: rep.identity_value,
            status: format!("{:?}", rep.status),
            risk_score: rep.risk_score,
            confidence: rep.confidence,
            reports_count: rep.reports_count,
            first_seen: rep.first_seen,
            last_seen: rep.last_seen,
        }
    }
}

/// Get reputation by email
async fn get_reputation_by_email(
    State(pool): State<DatabasePool>,
    Path(email): Path<String>,
) -> Result<Json<ReputationResponse>, StatusCode> {
    let repo = ReputationRepository::new(&pool.pool);
    
    let rep = repo
        .get_or_create(IdentityType::Email, &email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rep.into()))
}

/// Get reputation by domain
async fn get_reputation_by_domain(
    State(pool): State<DatabasePool>,
    Path(domain): Path<String>,
) -> Result<Json<ReputationResponse>, StatusCode> {
    let repo = ReputationRepository::new(&pool.pool);
    
    let rep = repo
        .get_or_create(IdentityType::Domain, &domain)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rep.into()))
}

/// Get reputation by IP
async fn get_reputation_by_ip(
    State(pool): State<DatabasePool>,
    Path(ip): Path<String>,
) -> Result<Json<ReputationResponse>, StatusCode> {
    let repo = ReputationRepository::new(&pool.pool);
    
    let rep = repo
        .get_or_create(IdentityType::Ip, &ip)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rep.into()))
}

/// Search reputations
async fn search_reputations(
    State(pool): State<DatabasePool>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<ReputationResponse>>, StatusCode> {
    let repo = ReputationRepository::new(&pool.pool);
    
    let reps = repo
        .search(&params.q)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(reps.into_iter().map(|r| r.into()).collect()))
}

/// List recent reputations
async fn list_reputations(
    State(pool): State<DatabasePool>,
) -> Result<Json<Vec<ReputationResponse>>, StatusCode> {
    let repo = ReputationRepository::new(&pool.pool);
    
    let reps = repo
        .list_recent(50)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(reps.into_iter().map(|r| r.into()).collect()))
}
