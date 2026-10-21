use std::env;
use std::sync::Arc;

use duet_api::{DuetApi, PgMoments, router_with_front_door, schema_is_ready};

#[tokio::main]
async fn main() {
    let _telemetry = telemetry::init("duet-api");
    let url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = sqlx::PgPool::connect(&url).await.expect("database");
    if !schema_is_ready(&pool).await.expect("schema") {
        tracing::error!(
            migration = duet_api::REQUIRED_MIGRATION,
            "database is missing a required migration"
        );
        std::process::exit(1);
    }
    let front_door_url = env::var("FRONT_DOOR_DATABASE_URL").unwrap_or_else(|_| url.clone());
    let front_door = front_door::PostgresFrontDoorStore::connect_with_migrations(&front_door_url)
        .await
        .expect("front-door database");
    let app = router_with_front_door(
        DuetApi::new(Arc::new(front_door)),
        Arc::new(PgMoments::new(pool)),
    )
    .layer(telemetry::http_layer());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
