use std::env;
use std::sync::Arc;

use duet_api::{DuetApi, PgMoments, router_with_front_door, schema_is_ready};

#[tokio::main]
async fn main() {
    if let Err(reason) = duet_api::hosting::validate_startup(|key| env::var(key).ok()) {
        eprintln!("{reason}");
        std::process::exit(1);
    }
    let url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = sqlx::PgPool::connect(&url).await.expect("database");
    if !schema_is_ready(&pool).await.expect("schema") {
        eprintln!(
            "database is missing migration {}",
            duet_api::REQUIRED_MIGRATION
        );
        std::process::exit(1);
    }
    let front_door_url = env::var("FRONT_DOOR_DATABASE_URL").unwrap_or_else(|_| url.clone());
    let front_door =
        front_door::PostgresFrontDoorStore::connect_without_migrations(&front_door_url)
            .await
            .expect("front-door database");
    front_door
        .check_schema()
        .await
        .expect("front-door schema; run the migration job first");
    let mut app = router_with_front_door(
        DuetApi::new(Arc::new(front_door)),
        Arc::new(PgMoments::new(pool)),
    );
    if let Some(directory) = env::var_os("DUET_SITE_DIR") {
        let directory = std::path::PathBuf::from(directory);
        for name in ["index.html", "duet-app.js", "duet-app_bg.wasm"] {
            assert!(directory.join(name).is_file(), "missing Duet asset: {name}");
        }
        app = app.merge(duet_api::site::router(directory));
    }
    let address = env::var("DUET_LISTEN_ADDR").unwrap_or_else(|_| {
        env::var("PORT")
            .map(|port| format!("0.0.0.0:{port}"))
            .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
    });
    let app = app.route("/readyz", axum::routing::get(duet_api::hosting::ready));
    let listener = tokio::net::TcpListener::bind(address).await.expect("bind");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("serve");
}

async fn shutdown() {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = terminate.recv() => {},
        }
    }
    #[cfg(not(unix))]
    tokio::signal::ctrl_c().await.expect("shutdown signal");
}
