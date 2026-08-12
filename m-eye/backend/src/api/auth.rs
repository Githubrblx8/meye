use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json,
    Router,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::OpenApi;
use uuid::Uuid;
use validator::Validate;

use crate::core::{jwt::JwtClaims, AppState};
use crate::models::UserRole;

// ============ Login ============

#[derive(Deserialize, Validate, OpenApi)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Serialize, OpenApi)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Serialize, OpenApi)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if let Err(e) = payload.validate() {
        tracing::warn!("Validation error on login: {}", e);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Fetch user from database
    let user = sqlx::query_as::<_, UserDb>(
        r#"
        SELECT id, email, password_hash, role, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error during login: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = match user {
        Some(u) => u,
        None => {
            tracing::warn!("Login attempt for non-existent user: {}", payload.email);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Verify password with Argon2
    let parsed_hash = argon2::PasswordHash::new(&user.password_hash)
        .map_err(|_| {
            tracing::error!("Invalid password hash format for user: {}", user.email);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let argon2 = Argon2::default();
    argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| {
            tracing::warn!("Invalid password for user: {}", payload.email);
            StatusCode::UNAUTHORIZED
        })?;

    // Generate JWT token
    let token = state.jwt_manager
        .generate_token(user.id, user.email.clone(), user.role.clone())
        .map_err(|e| {
            tracing::error!("Failed to generate JWT token: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 3600, // 1 hour
        user: UserInfo {
            id: user.id,
            email: user.email,
            role: user.role,
        },
    }))
}

// ============ Register ============

#[derive(Deserialize, Validate, OpenApi)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 12))]
    pub password: String,
}

#[derive(Serialize, OpenApi)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub email: String,
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
        (status = 400, description = "Invalid request or user already exists")
    )
)]
async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    if let Err(e) = payload.validate() {
        tracing::warn!("Validation error on registration: {}", e);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user already exists
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error checking existing user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if existing > 0 {
        tracing::warn!("Registration attempt for existing email: {}", payload.email);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Hash password with Argon2
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Failed to hash password: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .to_string();

    let user_id = Uuid::new_v4();
    let now = Utc::now();

    // Insert new user with default 'member' role
    sqlx::query(
        r#"
        INSERT INTO users (id, email, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, 'member', $4, $5)
        "#,
    )
    .bind(user_id)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error creating user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(RegisterResponse {
        id: user_id,
        email: payload.email,
        message: "User registered successfully. Please login.".to_string(),
    }))
}

// ============ Get Current User ============

#[derive(Serialize, OpenApi)]
pub struct CurrentUserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    responses(
        (status = 200, description = "Current user info", body = CurrentUserResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
async fn get_current_user(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<CurrentUserResponse>, StatusCode> {
    let token = extract_bearer_token(&headers)?;
    
    let claims = state.jwt_manager.validate_token(&token).map_err(|e| {
        tracing::warn!("Invalid token in get_current_user: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    let user = sqlx::query_as::<_, UserDb>(
        r#"
        SELECT id, email, password_hash, role, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(uuid::Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error fetching current user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match user {
        Some(u) => Ok(Json(CurrentUserResponse {
            id: u.id,
            email: u.email,
            role: u.role,
            created_at: u.created_at,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// ============ Helper Functions ============

#[derive(FromRow)]
struct UserDb {
    id: Uuid,
    email: String,
    password_hash: String,
    role: UserRole,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

fn extract_bearer_token(headers: &HeaderMap) -> Result<String, StatusCode> {
    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        tracing::warn!("Authorization header does not start with 'Bearer '");
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(auth_header[7..].to_string())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/auth/login", axum::routing::post(login))
        .route("/api/v1/auth/register", axum::routing::post(register))
        .route("/api/v1/auth/me", axum::routing::get(get_current_user))
}
