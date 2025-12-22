use dotenvy::dotenv;
use std::env;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load .env file using dotenvy
    dotenv().ok();

    // get database url from env
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file or environment.");
    
    println!("Connecting to database...");
    
    // create connection pool using db::create_pool()
    let pool = db::create_pool(&database_url).await
        .map_err(|e| {
            eprintln!("Failed to connect to database: {}", e);
            eprintln!("Please ensure:");
            eprintln!("  1. PostgreSQL is running");
            eprintln!("  2. DATABASE_URL is correct (format: postgresql://user:password@host:port/database)");
            eprintln!("  3. The database exists");
            e
        })?;
    
    // run migrations using db::run_migrations()
    db::run_migrations(&pool).await?;
    
    // print success
    println!("Database migrations completed successfully!");
    println!("Starting gift registry application...");

    Ok(())
}
