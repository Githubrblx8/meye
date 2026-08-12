use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::core::{jwt::JwtClaims, AppState};

/// Extract Bearer token from headers
pub fn extract_bearer_token(headers: &HeaderMap) -> Result<String, StatusCode> {
    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(auth_header[7..].to_string())
}

/// Middleware to require authentication
pub async fn require_auth<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let token = extract_bearer_token(headers)?;

    let state = request
        .extensions()
        .get::<AppState>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .clone();

    let claims = state.jwt_manager.validate_token(&token).map_err(|e| {
        tracing::warn!("Invalid token in require_auth middleware: {}", e);
        match e {
            crate::core::jwt::JwtError::TokenExpired => StatusCode::UNAUTHORIZED,
            _ => StatusCode::UNAUTHORIZED,
        }
    })?;

    // Store claims in request extensions for use in handlers
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Middleware to require specific role
pub async fn require_role<B>(
    mut request: Request<B>,
    next: Next<B>,
    required_role: crate::models::UserRole,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let token = extract_bearer_token(headers)?;

    let state = request
        .extensions()
        .get::<AppState>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .clone();

    let claims = state.jwt_manager.validate_token(&token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check if user has required role or higher
    if !has_required_role(&claims.role, &required_role) {
        tracing::warn!(
            "User with role {:?} attempted to access resource requiring {:?}",
            claims.role,
            required_role
        );
        return Err(StatusCode::FORBIDDEN);
    }

    // Store claims in request extensions for use in handlers
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Check if a role has the required permissions
/// Roles are ordered by permission level: Member < TrustedReporter < Researcher < Moderator < Administrator
fn has_required_role(user_role: &crate::models::UserRole, required: &crate::models::UserRole) -> bool {
    use crate::models::UserRole;
    
    let user_level = match user_role {
        UserRole::Member => 0,
        UserRole::TrustedReporter => 1,
        UserRole::SecurityResearcher => 2,
        UserRole::Moderator => 3,
        UserRole::Administrator => 4,
    };

    let required_level = match required {
        UserRole::Member => 0,
        UserRole::TrustedReporter => 1,
        UserRole::SecurityResearcher => 2,
        UserRole::Moderator => 3,
        UserRole::Administrator => 4,
    };

    user_level >= required_level
}

/// Extension trait to get claims from request
pub trait ClaimsExt {
    fn get_claims(&self) -> Option<&JwtClaims>;
    fn get_user_id(&self) -> Option<Uuid>;
}

impl ClaimsExt for axum::extract::Extension<JwtClaims> {
    fn get_claims(&self) -> Option<&JwtClaims> {
        Some(&self.0)
    }

    fn get_user_id(&self) -> Option<Uuid> {
        Uuid::parse_str(&self.0.sub).ok()
    }
}

/// Helper to get claims from request extensions
pub fn get_claims_from_request(request: &axum::http::Request<axum::body::Body>) -> Option<&JwtClaims> {
    request.extensions().get::<JwtClaims>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::UserRole;

    #[test]
    fn test_role_hierarchy() {
        // Administrator can access everything
        assert!(has_required_role(&UserRole::Administrator, &UserRole::Member));
        assert!(has_required_role(&UserRole::Administrator, &UserRole::Moderator));
        assert!(has_required_role(&UserRole::Administrator, &UserRole::Administrator));

        // Moderator can access Member, TrustedReporter, SecurityResearcher, Moderator
        assert!(has_required_role(&UserRole::Moderator, &UserRole::Member));
        assert!(has_required_role(&UserRole::Moderator, &UserRole::Moderator));
        assert!(!has_required_role(&UserRole::Moderator, &UserRole::Administrator));

        // Member can only access Member resources
        assert!(has_required_role(&UserRole::Member, &UserRole::Member));
        assert!(!has_required_role(&UserRole::Member, &UserRole::TrustedReporter));
    }
}
