//! Reputation engine for M'Eye

use meye_models::{IdentityType, ReputationStatus};

/// Calculate initial risk score for a new identity
pub fn calculate_initial_score(identity_type: &IdentityType) -> (i32, f64) {
    // New identities start with UNKNOWN status and low confidence
    match identity_type {
        IdentityType::Email => (0, 0.3),
        IdentityType::Domain => (0, 0.4),
        IdentityType::Ip => (0, 0.3),
        IdentityType::Url => (0, 0.2),
    }
}

/// Determine status based on risk score
pub fn determine_status(risk_score: i32) -> ReputationStatus {
    match risk_score {
        0..=20 => ReputationStatus::Safe,
        21..=40 => ReputationStatus::Unknown,
        41..=60 => ReputationStatus::Watch,
        61..=80 => ReputationStatus::Blocked,
        81..=100 => ReputationStatus::Compromised,
        _ => ReputationStatus::Unknown,
    }
}

/// Update confidence based on reports count
pub fn update_confidence(base_confidence: f64, reports_count: i32) -> f64 {
    let report_factor = (reports_count as f64 * 0.05).min(0.5);
    (base_confidence + report_factor).min(0.95)
}
