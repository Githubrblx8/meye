//! Database layer for M'Eye platform

pub mod pool;
pub mod user;
pub mod reputation;
pub mod report;

pub use pool::DatabasePool;
pub use user::UserRepository;
pub use reputation::ReputationRepository;
pub use report::ReportRepository;

/// Initialize database and run migrations
pub async fn init_database(database_url: &str) -> Result<DatabasePool, sqlx::Error> {
    let pool = DatabasePool::new(database_url).await?;
    
    // Run migrations if needed
    sqlx::migrate!("./migrations")
        .run(&pool.pool)
        .await?;
    
    tracing::info!("Database initialized and migrations applied");
    Ok(pool)
}
