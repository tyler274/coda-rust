//! Runs the Postgres-backed clinical library HTTP service serving
//! `/v1/search`, `/v1/read`, and `/v1/freshness`.

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("CLINICAL_LIBRARY_DATABASE_URL").unwrap_or_default();
    if database_url.is_empty() {
        eprintln!("CLINICAL_LIBRARY_DATABASE_URL is required");
        std::process::exit(1);
    }
    let bind_addr =
        std::env::var("CLINICAL_LIBRARY_BIND").unwrap_or_else(|_| "0.0.0.0:8085".to_owned());

    let result = async {
        let mut store = clinical_library::Store::connect(&database_url).await?;
        if let Some(encoder) = clinical_library::SubprocessBgeEncoder::from_env() {
            store = store.with_embedder(std::sync::Arc::new(encoder));
        }
        if std::env::var("CLINICAL_LIBRARY_AUTO_MIGRATE").as_deref() == Ok("1") {
            store.migrate().await?;
        }
        let listener = TcpListener::bind(&bind_addr).await?;
        let app = clinical_library::server::router(store);
        axum::serve(listener, app).await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    }
    .await;

    if let Err(error) = result {
        eprintln!("clinical library server failed: {error}");
        std::process::exit(1);
    }
}
