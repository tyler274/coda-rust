//! The runtime chain's ledger decides whether its schema runs.
use super::*;

/// Creates the per-test database and returns its URL. The database is left in
/// place after the test; the next run's `DROP DATABASE IF EXISTS` clears it.
async fn database(name: &str) -> String {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let (base, db) = url.trim().rsplit_once('/').expect("database URL");
    let test_db = format!("{db}_t_{name}");
    let admin = Database::connect(format!("{base}/postgres"))
        .await
        .expect("admin connection");
    admin
        .execute_unprepared(&format!("DROP DATABASE IF EXISTS {test_db}"))
        .await
        .expect("drop stale test database");
    admin
        .execute_unprepared(&format!("CREATE DATABASE {test_db}"))
        .await
        .expect("create test database");
    admin.close().await.expect("close admin connection");
    format!("{base}/{test_db}")
}

async fn names(db: &DatabaseConnection, query: &str) -> Vec<String> {
    db.query_all_raw(sea_orm::Statement::from_string(
        sea_orm::DbBackend::Postgres,
        query,
    ))
    .await
    .expect("catalog read")
    .into_iter()
    .map(|row| row.try_get("", "name").unwrap())
    .collect()
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_second_migrate_records_nothing_new() {
    let store = Store::connect(&database("runtime_rerun").await)
        .await
        .unwrap();
    store.migrate().await.unwrap();
    store.migrate().await.unwrap();
    assert_eq!(
        names(
            &store.db,
            "SELECT version AS name FROM physician_runtime_migrations"
        )
        .await,
        ["m20261111_init", "m20261111_attempt_sources_version"]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn leftover_runtime_tables_fail_the_migration_and_record_nothing() {
    let store = Store::connect(&database("runtime_leftover").await)
        .await
        .unwrap();
    // A runtime table in an earlier shape, with no ledger beside it.
    store
        .db
        .execute_unprepared("CREATE TABLE physician_runtime_conversations (practice text)")
        .await
        .unwrap();
    let error = store.migrate().await.unwrap_err().to_string();
    assert!(
        str::contains(
            &error,
            r#"relation "physician_runtime_conversations" already exists"#
        ),
        "{error}"
    );
    // The failed migration leaves no ledger and no other table behind.
    assert_eq!(
        names(
            &store.db,
            "SELECT tablename::text AS name FROM pg_tables WHERE schemaname = 'public'"
        )
        .await,
        ["physician_runtime_conversations"]
    );
}
