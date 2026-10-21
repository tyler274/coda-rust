//! The vendored Duet schema chain: checksum pinning, a fresh install, and an
//! idempotent rerun against a disposable Postgres.

use sha2::{Digest, Sha256};

#[test]
fn compiled_migrations_match_the_checksums_pinned_at_build_time() {
    duet_api::schema::verify_checksums().unwrap();
}

#[test]
fn a_changed_migration_byte_fails_its_checksum() {
    let (file, expected) = duet_api::schema::MIGRATIONS[0];
    let mut bytes = duet_api::schema::migration_sql(file).as_bytes().to_vec();
    bytes[0] ^= 0x01;
    let digest = Sha256::digest(&bytes);
    let digest = digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    assert_ne!(digest, expected, "{file} accepted a flipped byte");
}

/// Applies the whole chain twice on a fresh database. Needs `DATABASE_URL`
/// pointing at a disposable server; the database named `duet_schema_test` is
/// dropped and recreated.
#[tokio::test]
async fn fresh_install_reaches_the_required_migration_and_rerun_applies_nothing() {
    let url = match std::env::var("DATABASE_URL") {
        Ok(url) if !url.trim().is_empty() => url,
        _ => return,
    };
    let name = "duet_schema_test";
    let admin = sqlx::PgPool::connect(&url).await.unwrap();
    // The name is a fixed literal; formatting only satisfies the identifier.
    let drop = format!("drop database if exists {name} with (force)");
    let create = format!("create database {name}");
    sqlx::raw_sql(sqlx::AssertSqlSafe(drop.as_str()))
        .execute(&admin)
        .await
        .ok();
    sqlx::raw_sql(sqlx::AssertSqlSafe(create.as_str()))
        .execute(&admin)
        .await
        .unwrap();
    // Keep the admin connection's authority and point the path at the new database.
    let database = format!(
        "{}//{name}",
        url.rsplit_once('/').map(|(base, _)| base).unwrap_or(&url)
    );

    let first = duet_api::schema::apply(&database)
        .await
        .unwrap_or_else(|error| panic!("apply failed: {error}"));
    assert!(first > 100, "only {first} migrations applied");
    let second = duet_api::schema::apply(&database).await.unwrap();
    assert_eq!(second, 0, "rerun applied migrations");

    let pool = sqlx::PgPool::connect(&database).await.unwrap();
    assert!(duet_api::schema_is_ready(&pool).await.unwrap());
    let newest: String = sqlx::query_scalar(
        "select version from public.schema_migrations order by version desc limit 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(newest, duet_api::REQUIRED_MIGRATION);
}
