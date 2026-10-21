//! Fresh PostgreSQL proof for every workspace-owned migration chain.
//!
//! Run explicitly with `--ignored`; missing configuration is an error, never
//! a successful skip. Duet's external schema is not a workspace migration chain.

use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

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
        admin
            .execute_unprepared(&format!("CREATE DATABASE {name} TEMPLATE template0"))
            .await
            .expect("create fresh database");
        let url = format!("{base}/{name}");
        let db = Database::connect(&url).await.expect("fresh connection");
        db.execute_unprepared("CREATE EXTENSION vector")
            .await
            .expect("product pgvector extension available");
        migrate(&url, physician, front_door).await;
        let mut histories = Vec::new();
        if physician {
            let rows = history(&db, "seaql_migrations").await;
            assert_eq!(
                rows.iter().map(|row| row.0.as_str()).collect::<Vec<_>>(),
                ["m20260101_000002_practice_patients", "mod"]
            );
            histories.push(("seaql_migrations", rows));
        }
        if front_door {
            let rows = history(&db, "front_door_seaql_migrations").await;
            assert_eq!(
                rows.iter().map(|row| row.0.as_str()).collect::<Vec<_>>(),
                [
                    "m20261020_identity",
                    "m20261021_duet_conversation",
                    "m20261021_luna_journal",
                    "mod"
                ]
            );
            histories.push(("front_door_seaql_migrations", rows));
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
        admin
            .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
            .await
            .expect("remove proof database");
    }
    admin.close().await.expect("close admin connection");
}
