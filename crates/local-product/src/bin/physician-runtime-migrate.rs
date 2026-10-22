//! Explicit physician runtime migration entrypoint; runtime connections never migrate.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("PHYSICIAN_DATABASE_URL")?;
    local_product::runtime::Store::connect(&url)
        .await?
        .migrate()
        .await?;
    Ok(())
}
