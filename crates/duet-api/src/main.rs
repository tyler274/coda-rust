use std::env;
use std::sync::Arc;

use duet_api::{PgMoments, router, schema_is_ready};

#[tokio::main]
async fn main() {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = sqlx::PgPool::connect(&url).await.expect("database");
    if !schema_is_ready(&pool).await.expect("schema") {
        eprintln!(
            "database is missing migration {}",
            duet_api::REQUIRED_MIGRATION
        );
        std::process::exit(1);
    }
    let app = router(Arc::new(PgMoments::new(pool)));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
