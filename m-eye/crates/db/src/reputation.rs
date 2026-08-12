//! Reputation repository for database operations

use chrono::Utc;
use meye_models::{IdentityType, Reputation, ReputationStatus};
use sqlx::PgPool;
use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReputationRepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Reputation not found")]
    NotFound,
}

pub struct ReputationRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> ReputationRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_or_create(
        &self,
        identity_type: IdentityType,
        identity_value: &str,
    ) -> Result<Reputation, ReputationRepositoryError> {
        // Try to get existing reputation
        if let Ok(rep) = self.get_by_identity(&identity_type, identity_value).await {
            return Ok(rep);
        }

        // Create new reputation with default values
        self.create(identity_type, identity_value).await
    }

    pub async fn create(
        &self,
        identity_type: IdentityType,
        identity_value: &str,
    ) -> Result<Reputation, ReputationRepositoryError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let rep = sqlx::query_as::<_, Reputation>(
            r#"
            INSERT INTO reputations (id, identity_type, identity_value, status, risk_score, confidence, 
                                     first_seen, last_seen, reports_count, confirmed_incidents, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&identity_type)
        .bind(identity_value.to_lowercase())
        .bind(ReputationStatus::Unknown)
        .bind(0i32)
        .bind(0.5f64)
        .bind(now)
        .bind(now)
        .bind(0i32)
        .bind(0i32)
        .bind(now)
        .bind(now)
        .fetch_one(self.pool)
        .await?;

        Ok(rep)
    }

    pub async fn get_by_identity(
        &self,
        identity_type: &IdentityType,
        identity_value: &str,
    ) -> Result<Reputation, ReputationRepositoryError> {
        let rep = sqlx::query_as::<_, Reputation>(
            "SELECT * FROM reputations WHERE identity_type = $1 AND identity_value = $2",
        )
        .bind(identity_type)
        .bind(identity_value.to_lowercase())
        .fetch_one(self.pool)
        .await?;

        Ok(rep)
    }

    pub async fn update_status(
        &self,
        id: Uuid,
        status: ReputationStatus,
        risk_score: i32,
        confidence: f64,
    ) -> Result<Reputation, ReputationRepositoryError> {
        let rep = sqlx::query_as::<_, Reputation>(
            r#"
            UPDATE reputations 
            SET status = $1, risk_score = $2, confidence = $3, updated_at = $4
            WHERE id = $5
            RETURNING *
            "#,
        )
        .bind(&status)
        .bind(risk_score)
        .bind(confidence)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(self.pool)
        .await?;

        Ok(rep)
    }

    pub async fn increment_reports_count(&self, id: Uuid) -> Result<(), ReputationRepositoryError> {
        sqlx::query(
            "UPDATE reputations SET reports_count = reports_count + 1, updated_at = $1 WHERE id = $2",
        )
        .bind(Utc::now())
        .bind(id)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_recent(&self, limit: i64) -> Result<Vec<Reputation>, ReputationRepositoryError> {
        let reps = sqlx::query_as::<_, Reputation>(
            "SELECT * FROM reputations ORDER BY updated_at DESC LIMIT $1",
        )
        .bind(limit)
        .fetch_all(self.pool)
        .await?;

        Ok(reps)
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Reputation>, ReputationRepositoryError> {
        let reps = sqlx::query_as::<_, Reputation>(
            r#"
            SELECT * FROM reputations 
            WHERE identity_value ILIKE $1 
            ORDER BY last_seen DESC 
            LIMIT 50
            "#,
        )
        .bind(format!("%{}%", query))
        .fetch_all(self.pool)
        .await?;

        Ok(reps)
    }
}
