//! PostgreSQL migration regression; explicit prerequisites, no silent skip.
use physician_auth::migration::{self, Migrator};
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use sea_orm_migration::{MigrationTrait, MigratorTrait, SchemaManager};

#[test]
fn names_are_stable_and_distinct() {
    let names: Vec<_> = Migrator::migrations()
        .iter()
        .map(|m| m.name().to_owned())
        .collect();
    assert_eq!(
        names,
        [
            "physician_auth_20260101_000001_init",
            "physician_auth_20260101_000002_practice_patients",
            "physician_auth_20261023_000003_unique_physician_identity"
        ]
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn postgres_store_connects() {
    let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL").unwrap();
    physician_auth::PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
}

#[tokio::test]
async fn postgres_fresh_upgrade_and_partial_schema_regressions() {
    let base = std::env::var("PHYSICIAN_TEST_DATABASE_URL").expect("loopback PostgreSQL required");
    assert!(base.starts_with("postgresql://"));
    let admin = Database::connect(&base).await.unwrap();
    for kind in ["fresh", "legacy", "partial", "second", "foreign"] {
        let schema = format!(
            "physician_migration_{}_{}",
            kind,
            uuid::Uuid::new_v4().simple()
        );
        admin
            .execute_unprepared(&format!("CREATE SCHEMA {schema}"))
            .await
            .unwrap();
        let separator = if base.contains('?') { '&' } else { '?' };
        let url = format!("{base}{separator}options=-csearch_path%3D{schema}");
        let db = Database::connect(&url).await.unwrap();
        if kind != "fresh" {
            db.execute_unprepared("CREATE TABLE seaql_migrations(version varchar PRIMARY KEY, applied_at bigint NOT NULL); INSERT INTO seaql_migrations VALUES('mod',123)").await.unwrap();
            if kind == "partial" {
                db.execute_unprepared("CREATE TABLE physician_accounts(id uuid PRIMARY KEY)")
                    .await
                    .unwrap();
                assert!(migration::migrate(&db).await.is_err());
                assert!(
                    SchemaManager::new(&db)
                        .has_table("physician_accounts")
                        .await
                        .unwrap()
                );
            } else if kind != "foreign" {
                migration::m20260101_000001_init
                    .up(&SchemaManager::new(&db))
                    .await
                    .unwrap();
                if kind == "second" {
                    migration::m20260101_000002_practice_patients
                        .up(&SchemaManager::new(&db))
                        .await
                        .unwrap();
                }
                db.execute_unprepared("INSERT INTO physician_accounts(id,practice_id,physician_id,email,display_name,created_at) VALUES('00000000-0000-0000-0000-000000000001','p','d','retained@example.test','Retained',123)").await.unwrap();
            }
        }
        if kind != "partial" {
            migration::migrate(&db).await.unwrap();
            migration::migrate(&db).await.unwrap();
            let count = db
                .query_one_raw(Statement::from_string(
                    DbBackend::Postgres,
                    "SELECT count(*) AS n FROM physician_auth_migrations".to_owned(),
                ))
                .await
                .unwrap()
                .unwrap()
                .try_get::<i64>("", "n")
                .unwrap();
            assert_eq!(count, 3);
            assert!(
                SchemaManager::new(&db)
                    .has_table("physician_patients")
                    .await
                    .unwrap()
            );
            if kind != "fresh" && kind != "foreign" {
                let row = db
                    .query_one_raw(Statement::from_string(
                        DbBackend::Postgres,
                        "SELECT display_name FROM physician_accounts".to_owned(),
                    ))
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(
                    row.try_get::<String>("", "display_name").unwrap(),
                    "Retained"
                );
                assert!(
                    db.query_one_raw(Statement::from_string(
                        DbBackend::Postgres,
                        "SELECT * FROM seaql_migrations WHERE version='mod' AND applied_at=123"
                            .to_owned()
                    ))
                    .await
                    .unwrap()
                    .is_some()
                );
            }
        }
        db.close().await.unwrap();
        // Only this test's unique schema; never a caller's existing data.
        admin
            .execute_unprepared(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }
}
