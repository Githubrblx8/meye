use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    Member,
    TrustedReporter,
    SecurityResearcher,
    Moderator,
    Administrator,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Reputation {
    pub id: Uuid,
    pub identity: String,
    pub identity_type: IdentityType,
    pub status: ReputationStatus,
    pub risk_score: i32,
    pub confidence: f64,
    pub reports_count: i32,
    pub confirmed_incidents: i32,
    pub first_seen: chrono::DateTime<chrono::Utc>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
#[sqlx(type_name = "identity_type", rename_all = "lowercase")]
pub enum IdentityType {
    Email,
    Domain,
    Ip,
    Url,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
#[sqlx(type_name = "reputation_status", rename_all = "UPPERCASE")]
pub enum ReputationStatus {
    Safe,
    Unknown,
    Watch,
    Blocked,
    Compromised,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub id: Uuid,
    pub target: String,
    pub target_type: IdentityType,
    pub category: ReportCategory,
    pub description: String,
    pub reporter_id: Uuid,
    pub status: ReportStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
#[sqlx(type_name = "report_category", rename_all = "snake_case")]
pub enum ReportCategory {
    Spam,
    Phishing,
    Malware,
    CredentialHarvesting,
    Impersonation,
    Fraud,
    CompromisedAccount,
    Suspicious,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
#[sqlx(type_name = "report_status", rename_all = "snake_case")]
pub enum ReportStatus {
    Pending,
    UnderReview,
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Evidence {
    pub id: Uuid,
    pub report_id: Uuid,
    pub evidence_type: EvidenceType,
    pub value: String,
    pub source: String,
    pub submitted_by: Uuid,
    pub confidence: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
#[sqlx(type_name = "evidence_type", rename_all = "snake_case")]
pub enum EvidenceType {
    EmailHeaders,
    Url,
    Domain,
    Ip,
    FileHash,
    Screenshot,
    SpfResult,
    DkimResult,
    DmarcResult,
    DnsInformation,
    SandboxResult,
    ThreatIntelReference,
    AnalystReport,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Suggestion {
    pub id: Uuid,
    pub target: String,
    pub target_type: IdentityType,
    pub suggested_status: ReputationStatus,
    pub reason: String,
    pub description: Option<String>,
    pub submitted_by: Uuid,
    pub status: SuggestionStatus,
    pub moderator_id: Option<Uuid>,
    pub moderated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
#[sqlx(type_name = "suggestion_status", rename_all = "snake_case")]
pub enum SuggestionStatus {
    Pending,
    Approved,
    Rejected,
}
