//! Runs the physician product's existing workspace-owned schema chains.
//! Auth, front-door, and durable conversation chains run here, never on startup.

#[tokio::main]
async fn main() {
    let url = std::env::var("PHYSICIAN_DATABASE_URL").unwrap_or_default();
    if url.trim().is_empty() {
        eprintln!("PHYSICIAN_DATABASE_URL is required");
        std::process::exit(1);
    }
    if let Err(error) = migrate(&url).await {
        eprintln!("physician migrations failed: {error}");
        std::process::exit(1);
    }
}

async fn migrate(url: &str) -> Result<(), String> {
    physician_auth::PostgresAuthStore::connect_with_migrations(url)
        .await
        .map_err(|error| format!("physician-auth: {error}"))?;
    front_door::PostgresFrontDoorStore::connect_with_migrations(url)
        .await
        .map_err(|error| format!("front-door: {error}"))?;
    local_product::runtime::Store::connect(url)
        .await
        .map_err(|error| format!("physician runtime connection: {error}"))?
        .migrate()
        .await
        .map_err(|error| format!("physician runtime migration: {error}"))?;
    Ok(())
}
