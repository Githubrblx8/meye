//! Report repository for database operations

use chrono::Utc;
use meye_models::{Evidence, EvidenceType, Report, ReportCategory, ReportStatus};
use sqlx::PgPool;
use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportRepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Report not found")]
    NotFound,
}

pub struct ReportRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> ReportRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        reporter_id: Uuid,
        target_type: meye_models::IdentityType,
        target_value: &str,
        category: ReportCategory,
        description: &str,
    ) -> Result<Report, ReportRepositoryError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let report = sqlx::query_as::<_, Report>(
            r#"
            INSERT INTO reports (id, reporter_id, target_type, target_value, category, description, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(reporter_id)
        .bind(&target_type)
        .bind(target_value.to_lowercase())
        .bind(&category)
        .bind(description)
        .bind(ReportStatus::Pending)
        .bind(now)
        .bind(now)
        .fetch_one(self.pool)
        .await?;

        Ok(report)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Report, ReportRepositoryError> {
        let report = sqlx::query_as::<_, Report>("SELECT * FROM reports WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await?
            .ok_or(ReportRepositoryError::NotFound)?;

        Ok(report)
    }

    pub async fn update_status(&self, id: Uuid, status: ReportStatus) -> Result<(), ReportRepositoryError> {
        sqlx::query("UPDATE reports SET status = $1, updated_at = $2 WHERE id = $3")
            .bind(&status)
            .bind(Utc::now())
            .bind(id)
            .execute(self.pool)
            .await?;

        Ok(())
    }

    pub async fn list_pending(&self) -> Result<Vec<Report>, ReportRepositoryError> {
        let reports = sqlx::query_as::<_, Report>(
            "SELECT * FROM reports WHERE status = $1 ORDER BY created_at DESC",
        )
        .bind(ReportStatus::Pending)
        .fetch_all(self.pool)
        .await?;

        Ok(reports)
    }

    pub async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Report>, ReportRepositoryError> {
        let reports = sqlx::query_as::<_, Report>(
            "SELECT * FROM reports WHERE reporter_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;

        Ok(reports)
    }

    pub async fn add_evidence(
        &self,
        report_id: Uuid,
        evidence_type: EvidenceType,
        value: &str,
        source: &str,
        submitted_by: Uuid,
        confidence: f64,
    ) -> Result<Evidence, ReportRepositoryError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let evidence = sqlx::query_as::<_, Evidence>(
            r#"
            INSERT INTO evidence (id, report_id, evidence_type, value, source, submitted_by, confidence, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(report_id)
        .bind(&evidence_type)
        .bind(value)
        .bind(source)
        .bind(submitted_by)
        .bind(confidence)
        .bind(now)
        .fetch_one(self.pool)
        .await?;

        Ok(evidence)
    }

    pub async fn get_evidence_by_report(&self, report_id: Uuid) -> Result<Vec<Evidence>, ReportRepositoryError> {
        let evidence = sqlx::query_as::<_, Evidence>(
            "SELECT * FROM evidence WHERE report_id = $1 ORDER BY created_at ASC",
        )
        .bind(report_id)
        .fetch_all(self.pool)
        .await?;

        Ok(evidence)
    }
}
