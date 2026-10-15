//! Applies the front-door migration chain. The migrator job runs this
//! against Cloud SQL; local runs share the same binary.

#[tokio::main]
async fn main() {
    let database_url = std::env::var("FRONT_DOOR_DATABASE_URL").unwrap_or_default();
    if database_url.is_empty() {
        eprintln!("FRONT_DOOR_DATABASE_URL is required");
        std::process::exit(1);
    }
    match front_door::PostgresFrontDoorStore::connect_with_migrations(&database_url).await {
        Ok(_) => eprintln!("front-door migrations applied"),
        Err(error) => {
            eprintln!("front-door migrations failed: {error}");
            std::process::exit(1);
        }
    }
}
