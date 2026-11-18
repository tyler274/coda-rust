//! The runtime chain's ledger decides whether its schema runs.
use super::*;

/// Creates the per-test database and returns its URL. The database is left in
/// place after the test; the next run's `DROP DATABASE IF EXISTS` clears it.
pub(super) async fn database(name: &str) -> String {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let (base, db) = url.trim().rsplit_once('/').expect("database URL");
    let test_db = format!("{db}_t_{name}");
    // CREATE and DROP DATABASE take no bind parameters, so the name goes in
    // as an identifier the query builder quoted.
    let quoted = {
        use sea_orm::sea_query::{PostgresQueryBuilder, QuotedBuilder};
        let mut quoted = String::new();
        PostgresQueryBuilder.prepare_iden(&test_db.clone().into(), &mut quoted);
        quoted
    };
    let admin = Database::connect(format!("{base}/postgres"))
        .await
        .expect("admin connection");
    admin
        .execute_unprepared(&format!("DROP DATABASE IF EXISTS {quoted}"))
        .await
        .expect("drop stale test database");
    admin
        .execute_unprepared(&format!("CREATE DATABASE {quoted}"))
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
            "SELECT version AS name FROM physician_runtime_migrations ORDER BY version"
        )
        .await,
        [
            "m20261111_init",
            "m20261111_response_feedback",
            "m20261112_source_pull_missing_candidates",
            "m20261113_source_links_and_standing",
            "m20261114_reconnect_commands_and_adapter_state",
            "m20261115_patient_memory",
            "m20261118_01_concerns",
            "m20261118_02_practice_jobs_dispatch",
        ]
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

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn source_pull_missing_candidates_migration_refuses_down_rollback() {
    use sea_orm_migration::{MigrationTrait, SchemaManager};

    let store = Store::connect(&database("runtime_down_refused").await)
        .await
        .unwrap();
    store.migrate().await.unwrap();

    let manager = SchemaManager::new(&store.db);
    let error = migration::m20261112_source_pull_missing_candidates::Migration
        .down(&manager)
        .await
        .expect_err("forward-only runtime migration must refuse rollback")
        .to_string();
    assert!(
        str::contains(
            &error,
            "the physician runtime's records can't be rolled back"
        ),
        "{error}"
    );
    assert!(
        names(
            &store.db,
            "SELECT column_name::text AS name FROM information_schema.columns WHERE table_name = 'physician_runtime_source_pulls' ORDER BY column_name"
        )
        .await
        .iter()
        .any(|n| n == "missing_candidates")
    );

    let links_error = migration::m20261113_source_links_and_standing::Migration
        .down(&manager)
        .await
        .expect_err("forward-only runtime migration must refuse rollback")
        .to_string();
    assert!(
        str::contains(
            &links_error,
            "the physician runtime's records can't be rolled back"
        ),
        "{links_error}"
    );
    assert!(
        names(
            &store.db,
            "SELECT tablename::text AS name FROM pg_tables WHERE schemaname = 'public' ORDER BY name"
        )
        .await
        .iter()
        .any(|n| n == "physician_runtime_patient_source_status")
    );

    let reconnect_error = migration::m20261114_reconnect_commands_and_adapter_state::Migration
        .down(&manager)
        .await
        .expect_err("forward-only runtime migration must refuse rollback")
        .to_string();
    assert!(
        str::contains(
            &reconnect_error,
            "the physician runtime's records can't be rolled back"
        ),
        "{reconnect_error}"
    );
    let tables = names(
        &store.db,
        "SELECT tablename::text AS name FROM pg_tables WHERE schemaname = 'public' ORDER BY name",
    )
    .await;
    assert!(
        tables
            .iter()
            .any(|n| n == "physician_runtime_reconnect_commands")
    );
    assert!(
        tables
            .iter()
            .any(|n| n == "physician_runtime_source_adapter_state")
    );
}
