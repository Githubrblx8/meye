use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::UserRole;

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject (user ID)
    pub sub: String,
    /// Email address
    pub email: String,
    /// User role
    pub role: UserRole,
    /// Issuer
    pub iss: String,
    /// Issued at timestamp
    pub iat: i64,
    /// Expiration timestamp
    pub exp: i64,
}

impl JwtClaims {
    /// Create new JWT claims for a user
    pub fn new(user_id: Uuid, email: String, role: UserRole, issuer: String, expires_in_secs: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id.to_string(),
            email,
            role,
            iss: issuer,
            iat: now.timestamp(),
            exp: (now + Duration::seconds(expires_in_secs)).timestamp(),
        }
    }
}

/// JWT Token Manager
pub struct JwtManager {
    secret_key: Vec<u8>,
    issuer: String,
    token_lifetime_secs: i64,
}

impl JwtManager {
    /// Create a new JWT manager
    pub fn new(secret_key: &str, issuer: String, token_lifetime_secs: i64) -> Self {
        Self {
            secret_key: secret_key.as_bytes().to_vec(),
            issuer,
            token_lifetime_secs,
        }
    }

    /// Generate a JWT token for a user
    pub fn generate_token(&self, user_id: Uuid, email: String, role: UserRole) -> Result<String, JwtError> {
        let claims = JwtClaims::new(
            user_id,
            email,
            role,
            self.issuer.clone(),
            self.token_lifetime_secs,
        );

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret_key),
        )
        .map_err(|e| JwtError::TokenGenerationFailed(e.to_string()))
    }

    /// Validate and decode a JWT token
    pub fn validate_token(&self, token: &str) -> Result<JwtClaims, JwtError> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
        validation.set_issuer(&[&self.issuer]);

        decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(&self.secret_key),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => JwtError::InvalidIssuer,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => JwtError::InvalidSignature,
            _ => JwtError::InvalidToken(e.to_string()),
        })
    }

    /// Extract user ID from token without full validation (for logging purposes)
    pub fn extract_user_id_unsafe(token: &str) -> Option<Uuid> {
        // Split token into parts
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        // Decode payload (second part)
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        let payload_bytes = URL_SAFE_NO_PAD.decode(parts[1]).ok()?;
        let payload_str = String::from_utf8(payload_bytes).ok()?;

        // Parse JSON to extract 'sub' field
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&payload_str) {
            if let Some(sub) = json.get("sub").and_then(|v| v.as_str()) {
                return Uuid::parse_str(sub).ok();
            }
        }

        None
    }
}

/// JWT Errors
#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("Failed to generate token: {0}")]
    TokenGenerationFailed(String),
    
    #[error("Token has expired")]
    TokenExpired,
    
    #[error("Invalid token issuer")]
    InvalidIssuer,
    
    #[error("Invalid token signature")]
    InvalidSignature,
    
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    
    #[error("User not found in token")]
    UserNotFound,
}

impl From<JwtError> for axum::http::StatusCode {
    fn from(err: JwtError) -> Self {
        match err {
            JwtError::TokenExpired => axum::http::StatusCode::UNAUTHORIZED,
            JwtError::InvalidIssuer => axum::http::StatusCode::UNAUTHORIZED,
            JwtError::InvalidSignature => axum::http::StatusCode::UNAUTHORIZED,
            JwtError::InvalidToken(_) => axum::http::StatusCode::BAD_REQUEST,
            JwtError::TokenGenerationFailed(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            JwtError::UserNotFound => axum::http::StatusCode::UNAUTHORIZED,
        }
    }
}

/// Extension trait for extracting claims from Axum state
pub trait JwtClaimsExt {
    fn get_claims(&self) -> Option<&JwtClaims>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let manager = JwtManager::new("test-secret-key", "m-eye-test".to_string(), 3600);
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let role = UserRole::Member;

        let token = manager.generate_token(user_id, email.clone(), role.clone()).unwrap();
        assert!(!token.is_empty());

        let claims = manager.validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
        assert_eq!(claims.iss, "m-eye-test");
    }

    #[test]
    fn test_expired_token() {
        let manager = JwtManager::new("test-secret-key", "m-eye-test".to_string(), -1);
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let role = UserRole::Member;

        let token = manager.generate_token(user_id, email, role).unwrap();
        let result = manager.validate_token(&token);
        
        // Token with negative expiry should be invalid
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_signature() {
        let manager1 = JwtManager::new("secret1", "m-eye-test".to_string(), 3600);
        let manager2 = JwtManager::new("secret2", "m-eye-test".to_string(), 3600);
        
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let role = UserRole::Member;

        let token = manager1.generate_token(user_id, email, role).unwrap();
        let result = manager2.validate_token(&token);
        
        assert!(matches!(result, Err(JwtError::InvalidSignature)));
    }
}
