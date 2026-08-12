//! Shared data models for M'Eye platform

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// User roles following RBAC hierarchy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Member,
    TrustedReporter,
    SecurityResearcher,
    Moderator,
    Administrator,
}

impl UserRole {
    pub fn can_moderate(&self) -> bool {
        matches!(self, Self::Moderator | Self::Administrator)
    }

    pub fn can_research(&self) -> bool {
        matches!(
            self,
            Self::SecurityResearcher | Self::Moderator | Self::Administrator
        )
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Administrator)
    }
}

/// Reputation status for an identity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "UPPERCASE")]
pub enum ReputationStatus {
    Safe,
    Unknown,
    Watch,
    Blocked,
    Compromised,
}

impl Default for ReputationStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Identity type that can have a reputation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "lowercase")]
pub enum IdentityType {
    Email,
    Domain,
    Ip,
    Url,
}

/// User model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

/// Reputation entry for an identity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Reputation {
    pub id: Uuid,
    pub identity_type: IdentityType,
    pub identity_value: String,
    pub status: ReputationStatus,
    pub risk_score: i32,
    pub confidence: f64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub reports_count: i32,
    pub confirmed_incidents: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Report submitted by a user
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub id: Uuid,
    pub reporter_id: Uuid,
    pub target_type: IdentityType,
    pub target_value: String,
    pub category: ReportCategory,
    pub description: String,
    pub status: ReportStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Categories for reports
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "snake_case")]
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

/// Status of a report
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    Pending,
    UnderReview,
    Confirmed,
    Rejected,
    Resolved,
}

/// Evidence attached to a report
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Evidence {
    pub id: Uuid,
    pub report_id: Uuid,
    pub evidence_type: EvidenceType,
    pub value: String,
    pub source: String,
    pub submitted_by: Uuid,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
}

/// Types of evidence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "snake_case")]
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

/// Suggestion to change a reputation status
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Suggestion {
    pub id: Uuid,
    pub reputation_id: Uuid,
    pub suggested_by: Uuid,
    pub suggested_status: ReputationStatus,
    pub reason: String,
    pub description: String,
    pub status: SuggestionStatus,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Status of a suggestion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionStatus {
    Pending,
    Approved,
    Rejected,
    Modified,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub details: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String, // user_id
    pub email: String,
    pub role: UserRole,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
}

/// Risk assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub risk_score: i32,
    pub decision: RiskDecision,
    pub confidence: f64,
    pub factors: Vec<RiskFactor>,
    pub recommendation: String,
}

/// Risk decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum RiskDecision {
    Allow,
    Watch,
    Block,
    Quarantine,
}

/// Individual risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub contribution: i32,
    pub weight: f64,
    pub confidence: f64,
    pub evidence: String,
}

/// Email authentication results
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthResults {
    pub spf: Option<AuthResult>,
    pub dkim: Option<AuthResult>,
    pub dmarc: Option<AuthResult>,
    pub arc: Option<AuthResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub result: String,
    pub details: Option<String>,
}

/// Parsed email structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParsedEmail {
    pub message_id: Option<String>,
    pub from: Option<String>,
    pub to: Vec<String>,
    pub subject: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub headers: Vec<EmailHeader>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub attachments: Vec<Attachment>,
    pub urls: Vec<String>,
    pub raw_headers: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: Option<String>,
    pub content_type: String,
    pub size: usize,
    pub hash_sha256: Option<String>,
}
