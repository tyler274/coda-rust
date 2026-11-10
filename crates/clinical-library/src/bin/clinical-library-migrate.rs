//! Applies the clinical library's migration chain to
//! `CLINICAL_LIBRARY_DATABASE_URL`.

#[tokio::main]
async fn main() {
    let database_url = std::env::var("CLINICAL_LIBRARY_DATABASE_URL").unwrap_or_default();
    if database_url.is_empty() {
        eprintln!("CLINICAL_LIBRARY_DATABASE_URL is required");
        std::process::exit(1);
    }
    let result = async {
        let store = clinical_library::Store::connect(&database_url).await?;
        store.migrate().await
    }
    .await;
    match result {
        Ok(()) => eprintln!("clinical library migrations applied"),
        Err(error) => {
            eprintln!("clinical library migrations failed: {error}");
            std::process::exit(1);
        }
    }
}
