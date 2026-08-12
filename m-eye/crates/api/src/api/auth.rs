//! Authentication endpoints

use axum::{Json, Router, extract::State, http::StatusCode, Extension};
use argon2::{password_hash::PasswordHash, Argon2, PasswordVerifier};
use chrono::Utc;
use meye_db::{DatabasePool, user::UserRepository};
use meye_models::{JwtClaims, UserRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn routes() -> Router<DatabasePool> {
    Router::new()
        .route("/login", axum::routing::post(login))
        .route("/register", axum::routing::post(register))
        .route("/me", axum::routing::get(get_current_user))
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
    user: UserDto,
}

#[derive(Serialize)]
struct UserDto {
    id: Uuid,
    email: String,
    role: UserRole,
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

/// Login endpoint
async fn login(
    State(pool): State<DatabasePool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user_repo = UserRepository::new(&pool.pool);
    
    // Get user by email
    let user = match user_repo.get_by_email(&payload.email).await {
        Ok(u) => u,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let argon2 = Argon2::default();
    if argon2.verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Update last login
    let _ = user_repo.update_last_login(user.id).await;

    // Generate JWT token (simplified - should use proper JWT library)
    let token = generate_jwt_token(&user).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        user: UserDto {
            id: user.id,
            email: user.email,
            role: user.role,
        },
    }))
}

/// Register endpoint (creates user with Member role by default)
async fn register(
    State(pool): State<DatabasePool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user_repo = UserRepository::new(&pool.pool);
    
    // Hash password
    let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    // Create user
    let user = user_repo
        .create(&payload.email, &password_hash, UserRole::Member)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Generate JWT token
    let token = generate_jwt_token(&user).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        user: UserDto {
            id: user.id,
            email: user.email,
            role: user.role,
        },
    }))
}

/// Get current user info
async fn get_current_user(
    Extension(claims): Extension<JwtClaims>,
) -> Json<UserDto> {
    Json(UserDto {
        id: Uuid::parse_str(&claims.sub).unwrap_or_default(),
        email: claims.email,
        role: claims.role,
    })
}

/// Helper function to generate JWT token
fn generate_jwt_token(user: &meye_models::User) -> Result<String, Box<dyn std::error::Error>> {
    use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
    
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret_change_me".to_string());
    
    let now = Utc::now().timestamp();
    let expiry = now + 3600; // 1 hour

    let claims = JwtClaims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: user.role.clone(),
        iss: std::env::var("JWT_ISSUER").unwrap_or_else(|_| "m-eye".to_string()),
        iat: now,
        exp: expiry,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

// Add rand dependency placeholder
mod rand {
    pub fn thread_rng() -> impl rand_core::RngCore {
        rand_core::OsRng
    }
}
