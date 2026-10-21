//! The vendored Duet schema as a SeaORM chain: a fresh install reaches the
//! required version, and a rerun applies nothing.

/// Applies the chain twice on a fresh database. Needs `DATABASE_URL` pointing
/// at a disposable server; the database named `duet_schema_test` is dropped
/// and recreated.
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
    let database = format!(
        "{}//{name}",
        url.rsplit_once('/').map(|(base, _)| base).unwrap_or(&url)
    );

    duet_api::migration::up(&database)
        .await
        .unwrap_or_else(|error| panic!("apply failed: {error}"));
    duet_api::migration::up(&database)
        .await
        .unwrap_or_else(|error| panic!("rerun failed: {error}"));

    let pool = sqlx::PgPool::connect(&database).await.unwrap();
    assert!(duet_api::schema_is_ready(&pool).await.unwrap());
    let versions: i64 = sqlx::query_scalar("select count(*) from public.duet_seaql_migrations")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(versions, 117, "the chain records one row per vendored file");
    let newest: String = sqlx::query_scalar(
        "select version from public.schema_migrations order by version desc limit 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(newest, duet_api::REQUIRED_MIGRATION);
}
