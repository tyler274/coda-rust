//! Real PostgreSQL fresh/legacy/co-resident migration adoption, never silently skipped.
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use sea_orm_migration::{MigrationTrait, SchemaManager};
#[tokio::test]
async fn fresh_legacy_and_co_resident() {
    let url = std::env::var("FRONT_DOOR_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
    assert!(url.contains("localhost") || url.contains("127.0.0.1"));
    let db = Database::connect(&url).await.unwrap();
    let schema = format!("front_proof_{}", uuid::Uuid::new_v4().simple());
    db.execute_unprepared(&format!(
        "create schema {schema}; set search_path to {schema}"
    ))
    .await
    .unwrap();
    // A single connection is required for a temporary schema search_path.
    let mut options = sea_orm::ConnectOptions::new(url);
    options.max_connections(1).min_connections(1);
    let db = Database::connect(options).await.unwrap();
    db.execute_unprepared(&format!("set search_path to {schema}"))
        .await
        .unwrap();
    front_door::migration::upgrade::migrate(&db).await.unwrap();
    front_door::migration::upgrade::migrate(&db).await.unwrap();
    let count = db
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "select count(*) n from front_door_migrations".to_owned(),
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get::<i64>("", "n")
        .unwrap();
    assert_eq!(count, 1);
    db.execute_unprepared("insert into front_door_accounts(id,email,display_name,created_at) values('00000000-0000-0000-0000-000000000001','preserved@example.test','Preserved',1); drop table front_door_migrations; create table seaql_migrations(version text primary key,applied_at bigint not null); insert into seaql_migrations values('mod',123); create table physician_auth_migrations(version text primary key,applied_at bigint not null); insert into physician_auth_migrations values('unrelated',456)").await.unwrap();
    front_door::migration::upgrade::migrate(&db).await.unwrap();
    let name = db
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "select display_name from front_door_accounts".to_owned(),
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get::<String>("", "display_name")
        .unwrap();
    assert_eq!(name, "Preserved");
    let count = db
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "select count(*) n from seaql_migrations where version='mod'".to_owned(),
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get::<i64>("", "n")
        .unwrap();
    assert_eq!(count, 1);
    db.execute_unprepared(
        "drop table front_door_migrations; drop index front_door_accounts_link_idx",
    )
    .await
    .unwrap();
    assert!(front_door::migration::upgrade::migrate(&db).await.is_err());
    db.execute_unprepared(&format!("drop schema {schema} cascade"))
        .await
        .unwrap();
    // Explicit stable name must not depend on source filename.
    assert_eq!(
        sea_orm_migration::MigrationName::name(&front_door::migration::m20260101_000001_init),
        "front_door_20260101_000001_init"
    );
    let _ = (
        SchemaManager::new(&db),
        &front_door::migration::m20260101_000001_init as &dyn MigrationTrait,
    );
}
