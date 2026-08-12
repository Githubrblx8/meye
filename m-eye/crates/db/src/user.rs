//! User repository for database operations

use chrono::Utc;
use meye_models::{User, UserRole};
use sqlx::PgPool;
use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
}

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        email: &str,
        password_hash: &str,
        role: UserRole,
    ) -> Result<User, UserRepositoryError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, role, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, true, $5, $6)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(email.to_lowercase())
        .bind(password_hash)
        .bind(&role)
        .bind(now)
        .bind(now)
        .fetch_one(self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_email(&self, email: &str) -> Result<User, UserRepositoryError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1 AND is_active = true",
        )
        .bind(email.to_lowercase())
        .fetch_optional(self.pool)
        .await?
        .ok_or(UserRepositoryError::NotFound)?;

        Ok(user)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<User, UserRepositoryError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND is_active = true")
            .bind(id)
            .fetch_optional(self.pool)
            .await?
            .ok_or(UserRepositoryError::NotFound)?;

        Ok(user)
    }

    pub async fn update_last_login(&self, id: Uuid) -> Result<(), UserRepositoryError> {
        sqlx::query("UPDATE users SET last_login = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(id)
            .execute(self.pool)
            .await?;

        Ok(())
    }

    pub async fn list_all(&self) -> Result<Vec<User>, UserRepositoryError> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users WHERE is_active = true ORDER BY created_at DESC")
            .fetch_all(self.pool)
            .await?;

        Ok(users)
    }
}
