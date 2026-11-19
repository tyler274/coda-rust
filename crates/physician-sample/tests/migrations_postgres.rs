//! Fresh PostgreSQL proof for every workspace-owned migration chain.
//!
//! Run explicitly with `--ignored`; missing configuration is an error, never
//! a successful skip. Duet's external schema is not a workspace migration chain.

use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

#[path = "common/disposable.rs"]
mod disposable;

async fn migrate(url: &str, physician: bool, front_door: bool) {
    // Match physician-sample's production order on the shared database.
    if physician {
        physician_auth::PostgresAuthStore::connect_with_migrations(url)
            .await
            .expect("physician-auth migrations");
    }
    if front_door {
        front_door::PostgresFrontDoorStore::connect_with_migrations(url)
            .await
            .expect("front-door migrations");
    }
}

async fn history(db: &DatabaseConnection, table: &str) -> Vec<(String, i64)> {
    db.query_all_raw(sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        format!("SELECT version, applied_at FROM {table} ORDER BY version"),
    ))
    .await
    .expect("read migration history")
    .into_iter()
    .map(|row| {
        (
            row.try_get("", "version").expect("version"),
            row.try_get("", "applied_at").expect("applied_at"),
        )
    })
    .collect()
}

#[tokio::test]
#[ignore = "requires a disposable PostgreSQL server; scripts/check-postgres.sh runs this"]
async fn fresh_chains_and_shared_database_are_idempotent() {
    let admin_url = std::env::var("MIGRATION_DATABASE_URL")
        .expect("MIGRATION_DATABASE_URL must point to a disposable PostgreSQL server");
    let (base, _) = admin_url.rsplit_once('/').expect("database URL");
    let admin = Database::connect(&admin_url)
        .await
        .expect("admin connection");
    for (label, physician, front_door) in [
        ("auth", true, false),
        ("front", false, true),
        ("shared", true, true),
    ] {
        let name = format!("migration_{label}_{}", uuid::Uuid::new_v4().simple());
        // Never drop an existing database: each proof starts from a new name.
        disposable::create_empty_database(&admin, &name).await;
        let url = format!("{base}/{name}");
        let db = Database::connect(&url).await.expect("fresh connection");
        db.execute_unprepared("CREATE EXTENSION vector")
            .await
            .expect("product pgvector extension available");
        migrate(&url, physician, front_door).await;
        let mut histories = Vec::new();
        if physician {
            let rows = history(&db, "physician_auth_migrations").await;
            assert_eq!(
                rows.iter().map(|row| row.0.as_str()).collect::<Vec<_>>(),
                [
                    "physician_auth_20260101_000001_init",
                    "physician_auth_20260101_000002_practice_patients",
                    "physician_auth_20261023_000003_unique_physician_identity",
                    "physician_auth_20261024_000004_authority_notify",
                    "physician_auth_20261030_000005_practices",
                    "physician_auth_20261106_000006_session_lifetime",
                    "physician_auth_20261109_000007_fixed_sessions",
                    "physician_auth_20261112_000008_drop_cadence_columns",
                    "physician_auth_20261113_000009_practice_time_zone",
                    "physician_auth_20261113_000010_practice_source_setup",
                ]
            );
            histories.push(("physician_auth_migrations", rows));
        }
        if front_door {
            let rows = history(&db, front_door::migration::MIGRATION_TABLE).await;
            assert_eq!(
                rows.iter().map(|row| row.0.as_str()).collect::<Vec<_>>(),
                [
                    front_door::migration::INIT_MIGRATION_NAME,
                    "m20261020_identity",
                    "m20261021_duet_conversation",
                    "m20261021_luna_journal",
                    "m20261022_duet_execution",
                    "m20261023_duet_admission",
                    "m20261112_drop_premium_cadence",
                    "m20261119_for_my_doctor_shares",
                ]
            );
            histories.push((front_door::migration::MIGRATION_TABLE, rows));
        }
        // Two reruns, not just the initial application. Neither migration
        // versions nor their original timestamps may change.
        for _ in 0..2 {
            migrate(&url, physician, front_door).await;
            for (table, expected) in &histories {
                assert_eq!(&history(&db, table).await, expected);
            }
        }
        db.close().await.expect("close proof database");
        disposable::drop_database(&admin, &name).await;
    }
    admin.close().await.expect("close admin connection");
}
