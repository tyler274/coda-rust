//! Runs the Duet-owned chains: the front-door SeaORM migrations, then the
//! vendored Duet schema as its own SeaORM chain. A rerun applies nothing.

#[tokio::main]
async fn main() {
    let url = std::env::var("DATABASE_URL").unwrap_or_default();
    if url.trim().is_empty() {
        eprintln!("DATABASE_URL is required");
        std::process::exit(1);
    }
    if let Err(error) = migrate(&url).await {
        eprintln!("Duet migration coverage incomplete: {error}");
        std::process::exit(1);
    }
}

async fn migrate(url: &str) -> Result<(), String> {
    let front_url = std::env::var("FRONT_DOOR_DATABASE_URL").unwrap_or_else(|_| url.to_owned());
    front_door::PostgresFrontDoorStore::connect_with_migrations(&front_url)
        .await
        .map_err(|error| format!("front-door: {error}"))?;
    duet_api::migration::up(url)
        .await
        .map_err(|error| format!("duet schema: {error}"))?;
    eprintln!("duet schema chain is current");
    let pool = sqlx::PgPool::connect(url)
        .await
        .map_err(|error| error.to_string())?;
    if !duet_api::schema_is_ready(&pool)
        .await
        .map_err(|error| error.to_string())?
    {
        return Err(format!(
            "schema chain completed without {}",
            duet_api::REQUIRED_MIGRATION
        ));
    }
    Ok(())
}
