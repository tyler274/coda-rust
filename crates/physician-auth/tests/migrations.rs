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
            "physician_auth_20261023_000003_unique_physician_identity",
            "physician_auth_20261024_000004_authority_notify",
            "physician_auth_20261030_000005_practices",
            "physician_auth_20261106_000006_session_lifetime",
            "physician_auth_20261109_000007_fixed_sessions",
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
            assert_eq!(count, 7);
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

#[tokio::test]
async fn old_thirty_day_sessions_become_twelve_hours_and_updates_do_not_notify() {
    let base = std::env::var("PHYSICIAN_TEST_DATABASE_URL").expect("loopback PostgreSQL required");
    let admin = Database::connect(&base).await.unwrap();
    let schema = format!(
        "physician_session_lifetime_{}",
        uuid::Uuid::new_v4().simple()
    );
    admin
        .execute_unprepared(&format!("CREATE SCHEMA {schema}"))
        .await
        .unwrap();
    let separator = if base.contains('?') { '&' } else { '?' };
    let url = format!("{base}{separator}options=-csearch_path%3D{schema}");
    let db = Database::connect(&url).await.unwrap();
    migration::migrate(&db).await.unwrap();
    db.execute_unprepared(
        "DROP TRIGGER IF EXISTS physician_sessions_authority_notify ON physician_sessions; \
         CREATE TRIGGER physician_sessions_authority_notify \
         AFTER UPDATE OR DELETE OR TRUNCATE ON physician_sessions \
         FOR EACH STATEMENT EXECUTE FUNCTION physician_authority_notify();",
    )
    .await
    .unwrap();
    db.execute_unprepared(
        "INSERT INTO physician_accounts(id,practice_id,physician_id,email,display_name,created_at) \
         VALUES('00000000-0000-0000-0000-000000000010','p','d','lifetime@example.test','Lifetime',1000)",
    )
    .await
    .unwrap();
    db.execute_unprepared(
        "INSERT INTO physician_sessions(account_id,token_hash,created_at,expires_at) VALUES \
         ('00000000-0000-0000-0000-000000000010', decode(repeat('11', 32), 'hex'), 1000, 1000 + 2592000), \
         ('00000000-0000-0000-0000-000000000010', decode(repeat('22', 32), 'hex'), 2000, 2000 + 43200), \
         ('00000000-0000-0000-0000-000000000010', decode(repeat('33', 32), 'hex'), 3000, 3000 + 1000)",
    )
    .await
    .unwrap();

    let repair = migration::m20261106_000006_session_lifetime;
    let manager = SchemaManager::new(&db);
    repair.up(&manager).await.unwrap();
    assert_eq!(
        session_deadlines(&db).await,
        vec![
            (1000, 1000 + 43200),
            (2000, 2000 + 43200),
            (3000, 3000 + 1000)
        ]
    );
    assert!(
        !session_trigger(&db).await.to_uppercase().contains("UPDATE"),
        "a session update is an idle slide and must not notify"
    );
    repair.up(&manager).await.unwrap();
    assert_eq!(
        session_deadlines(&db).await,
        vec![
            (1000, 1000 + 43200),
            (2000, 2000 + 43200),
            (3000, 3000 + 1000)
        ]
    );
    let trigger = session_trigger(&db).await.to_uppercase();
    assert!(trigger.contains("DELETE"));
    assert!(trigger.contains("TRUNCATE"));
    assert!(!trigger.contains("UPDATE"));

    db.close().await.unwrap();
    admin
        .execute_unprepared(&format!("DROP SCHEMA {schema} CASCADE"))
        .await
        .unwrap();
}

async fn session_deadlines(db: &sea_orm::DatabaseConnection) -> Vec<(i64, i64)> {
    let rows = db
        .query_all_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT created_at, expires_at FROM physician_sessions ORDER BY created_at".to_owned(),
        ))
        .await
        .unwrap();
    rows.into_iter()
        .map(|row| {
            (
                row.try_get("", "created_at").unwrap(),
                row.try_get("", "expires_at").unwrap(),
            )
        })
        .collect()
}

async fn session_trigger(db: &sea_orm::DatabaseConnection) -> String {
    db.query_one_raw(Statement::from_string(
        DbBackend::Postgres,
        "SELECT pg_get_triggerdef(t.oid) AS def \
         FROM pg_trigger t \
         JOIN pg_class c ON c.oid = t.tgrelid \
         JOIN pg_namespace n ON n.oid = c.relnamespace \
         WHERE t.tgname = 'physician_sessions_authority_notify' \
           AND NOT t.tgisinternal \
           AND n.nspname = current_schema()"
            .to_owned(),
    ))
    .await
    .unwrap()
    .unwrap()
    .try_get("", "def")
    .unwrap()
}
