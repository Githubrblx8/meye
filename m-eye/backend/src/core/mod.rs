use sqlx::postgres::PgPool;
use redis::Client as RedisClient;

pub mod jwt;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisClient,
    pub jwt_manager: std::sync::Arc<jwt::JwtManager>,
}

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPool::connect(&database_url).await
}

pub async fn create_client() -> Result<RedisClient, redis::RedisError> {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    RedisClient::open(redis_url)
}

pub mod db {
    use super::*;
    pub use sqlx::postgres::PgPool;
    
    pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        PgPool::connect(&database_url).await
    }
}

pub mod cache {
    use super::*;
    
    pub async fn create_client() -> Result<RedisClient, redis::RedisError> {
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());

        RedisClient::open(redis_url)
    }
}
