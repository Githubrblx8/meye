//! Risk engine for M'Eye - Calculates risk scores based on multiple factors

use meye_models::{AuthResults, RiskAssessment, RiskDecision, RiskFactor};

/// Main risk evaluation function
pub fn evaluate_risk(
    auth_results: &AuthResults,
    reports_count: i32,
    has_suspicious_urls: bool,
    has_executable_attachment: bool,
) -> RiskAssessment {
    let mut factors = Vec::new();
    let mut total_score = 0;

    // Factor 1: Authentication failures
    if let Some(ref spf) = auth_results.spf {
        if spf.result == "fail" {
            factors.push(RiskFactor {
                name: "SPF Failure".to_string(),
                contribution: 25,
                weight: 2.0,
                confidence: 0.95,
                evidence: "SPF authentication failed".to_string(),
            });
            total_score += 25;
        }
    }

    if let Some(ref dkim) = auth_results.dkim {
        if dkim.result == "fail" {
            factors.push(RiskFactor {
                name: "DKIM Failure".to_string(),
                contribution: 30,
                weight: 2.0,
                confidence: 0.95,
                evidence: "DKIM signature invalid".to_string(),
            });
            total_score += 30;
        }
    }

    if let Some(ref dmarc) = auth_results.dmarc {
        if dmarc.result == "fail" {
            factors.push(RiskFactor {
                name: "DMARC Failure".to_string(),
                contribution: 35,
                weight: 2.0,
                confidence: 0.95,
                evidence: "DMARC policy violation".to_string(),
            });
            total_score += 35;
        }
    }

    // Factor 2: Reports history
    if reports_count > 10 {
        let contribution = (reports_count * 2).min(30);
        factors.push(RiskFactor {
            name: "Multiple Reports".to_string(),
            contribution,
            weight: 1.5,
            confidence: 0.8,
            evidence: format!("{} user reports", reports_count),
        });
        total_score += contribution;
    }

    // Factor 3: Suspicious URLs
    if has_suspicious_urls {
        factors.push(RiskFactor {
            name: "Suspicious URLs".to_string(),
            contribution: 20,
            weight: 1.8,
            confidence: 0.7,
            evidence: "Email contains suspicious URLs".to_string(),
        });
        total_score += 20;
    }

    // Factor 4: Executable attachments
    if has_executable_attachment {
        factors.push(RiskFactor {
            name: "Executable Attachment".to_string(),
            contribution: 35,
            weight: 2.0,
            confidence: 0.9,
            evidence: "Email contains executable file".to_string(),
        });
        total_score += 35;
    }

    // Cap score at 100
    let risk_score = total_score.min(100);

    // Determine decision
    let decision = match risk_score {
        0..=30 => RiskDecision::Allow,
        31..=60 => RiskDecision::Watch,
        61..=85 => RiskDecision::Block,
        _ => RiskDecision::Quarantine,
    };

    // Calculate confidence
    let confidence = if factors.is_empty() {
        0.5
    } else {
        factors.iter().map(|f| f.confidence).sum::<f64>() / factors.len() as f64
    };

    RiskAssessment {
        risk_score,
        decision,
        confidence,
        factors,
        recommendation: generate_recommendation(&decision),
    }
}

fn generate_recommendation(decision: &RiskDecision) -> String {
    match decision {
        RiskDecision::Allow => "Email appears safe to deliver".to_string(),
        RiskDecision::Watch => "Monitor email delivery and user interaction".to_string(),
        RiskDecision::Block => "Block email and log for analysis".to_string(),
        RiskDecision::Quarantine => "Quarantine immediately and alert security team".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_risk_email() {
        let auth = AuthResults::default();
        let result = evaluate_risk(&auth, 0, false, false);
        
        assert_eq!(result.risk_score, 0);
        assert_eq!(result.decision, RiskDecision::Allow);
    }

    #[test]
    fn test_high_risk_email() {
        let mut auth = AuthResults::default();
        auth.dmarc = Some(meye_models::AuthResult {
            result: "fail".to_string(),
            details: None,
        });
        
        let result = evaluate_risk(&auth, 15, true, true);
        
        assert!(result.risk_score > 60);
        assert!(matches!(result.decision, RiskDecision::Block | RiskDecision::Quarantine));
    }
}
