//! Real PostgreSQL fresh/legacy/co-resident migration adoption, never silently skipped.
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use sea_orm_migration::{MigrationTrait, MigratorTrait, SchemaManager};

#[path = "common/disposable.rs"]
mod disposable;
#[tokio::test]
async fn fresh_legacy_and_co_resident() {
    let url = std::env::var("FRONT_DOOR_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
    assert!(url.contains("localhost") || url.contains("127.0.0.1"));
    let db = Database::connect(&url).await.unwrap();
    let schema = format!("front_proof_{}", uuid::Uuid::new_v4().simple());
    disposable::create_schema(&db, &schema).await;
    disposable::use_schema(&db, &schema).await;
    // A single connection is required for a temporary schema search_path.
    let mut options = sea_orm::ConnectOptions::new(url);
    options.max_connections(1).min_connections(1);
    let db = Database::connect(options).await.unwrap();
    disposable::use_schema(&db, &schema).await;
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
    // A fresh install records every migration in the chain exactly once, and
    // the rerun adds nothing.
    assert_eq!(
        count,
        i64::try_from(front_door::migration::Migrator::migrations().len()).unwrap()
    );
    // Rewind to the legacy init-only schema (fixture-only; production rollback
    // of these extensions is refused) before simulating its shared ledger.
    db.execute_unprepared("drop table front_door_connections, front_door_shared_records, front_door_for_my_doctor_shares, duet_model_execution_switch, duet_attempt_events, duet_conversation_attempts, duet_conversation_messages, duet_conversation_inputs, duet_conversation_state, front_door_luna_restrictions; drop trigger duet_permission_interrupt on front_door_permissions; drop function duet_permission_interrupt(); drop function duet_interrupt_accepted(uuid); drop function duet_switch_append_only(); drop table front_door_permissions, front_door_identities; alter table front_door_accounts add column linked_practice_id varchar, add column linked_patient_id varchar; create index front_door_accounts_link_idx on front_door_accounts(linked_practice_id, linked_patient_id); alter table front_door_sessions add column expires_at bigint not null default 0; alter table front_door_invitations add column expires_at bigint not null default 0; drop index front_door_invitations_email_idx, front_door_invitations_roster_uq; alter table front_door_invitations add constraint front_door_invitations_email_key unique(email); create table front_door_links(account_id uuid primary key references front_door_accounts(id) on delete cascade, practice_id varchar not null, patient_id varchar not null, created_at bigint not null); create index front_door_links_roster_idx on front_door_links(practice_id, patient_id); create table front_door_consents(account_id uuid primary key references front_door_accounts(id) on delete cascade, consented_at bigint not null, withdrawn_at bigint); create table front_door_premium(patient_key varchar primary key, premium boolean not null default false, cadence varchar not null default 'weekly', created_at bigint not null)").await.unwrap();
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
        "drop table front_door_migrations; drop index front_door_sessions_account_idx",
    )
    .await
    .unwrap();
    assert!(front_door::migration::upgrade::migrate(&db).await.is_err());
    disposable::drop_schema(&db, &schema).await;
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
