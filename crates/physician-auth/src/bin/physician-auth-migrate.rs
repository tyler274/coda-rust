//! Applies the physician-auth migration chain. The GKE migrator job runs
//! this against Cloud SQL; local runs share the same binary.

#[tokio::main]
async fn main() {
    let database_url = std::env::var("PHYSICIAN_DATABASE_URL").unwrap_or_default();
    if database_url.is_empty() {
        eprintln!("PHYSICIAN_DATABASE_URL is required");
        std::process::exit(1);
    }
    match physician_auth::PostgresAuthStore::connect_with_migrations(&database_url).await {
        Ok(_) => eprintln!("physician-auth migrations applied"),
        Err(error) => {
            eprintln!("physician-auth migrations failed: {error}");
            std::process::exit(1);
        }
    }
}
