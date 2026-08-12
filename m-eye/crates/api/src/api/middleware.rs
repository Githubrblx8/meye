//! Authentication middleware

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use meye_db::DatabasePool;
use meye_models::JwtClaims;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

/// Middleware to require authentication
pub async fn require_auth<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret_change_me".to_string());

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let token_data = match decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // Insert claims into request extensions
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}

/// Helper function to create middleware from async function
pub fn from_fn<F, B, R>(f: F) -> axum::middleware::FromFn<F, R, B>
where
    F: axum::handler::Handler<R, B>,
{
    axum::middleware::from_fn(f)
}
