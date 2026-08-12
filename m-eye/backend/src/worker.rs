use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod core;
mod models;
mod smtp;
mod workers;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting M'Eye Worker");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database connection pool
    let db_pool = core::db::create_pool()
        .await
        .expect("Failed to create database pool");

    // Initialize Redis connection
    let redis_client = core::cache::create_client()
        .await
        .expect("Failed to create Redis client");

    tracing::info!("Worker connected to database and Redis");

    // Start SMTP gateway listener
    let smtp_handle = tokio::spawn(async move {
        smtp::start_smtp_gateway(db_pool.clone()).await
    });

    // Start background job processor
    let worker_handle = tokio::spawn(async move {
        workers::process_jobs(db_pool.clone(), redis_client).await
    });

    // Wait for both tasks
    tokio::select! {
        _ = smtp_handle => {
            tracing::error!("SMTP gateway task completed unexpectedly");
        }
        _ = worker_handle => {
            tracing::error!("Worker task completed unexpectedly");
        }
    }
}
