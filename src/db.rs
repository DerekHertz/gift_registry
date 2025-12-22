use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

/// Creates and returns a connection pool to PostgreSQL
/// 
/// # Arguments
/// * `database_url` - PostgreSQL connection string
/// 
/// # Returns
/// * `Result<PgPool, sqlx::Error>` - Connection pool or error
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;
    
    Ok(pool)
}

/// Runs all pending migrations
/// 
/// # Arguments
/// * `pool` - Database connection pool
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    Ok(())
}
