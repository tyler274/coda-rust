//! The Postgres store against a real server, gated on `FRONT_DOOR_DATABASE_URL`.
//!
//! The devcontainer compose service provides that URL; without it the store's
//! contract is covered by the memory-store unit tests, and these stay quiet.

use front_door::{
    Account, CADENCE_WEEKLY, CADENCE_WEEKLY_DAILY_CHECK, Consent, FrontDoorStore,
    INVITATION_TTL_SECONDS, InviteError, LinkError, MemoryFrontDoorStore, PostgresFrontDoorStore,
    Premium, RosterPatient, SESSION_TTL_SECONDS, SessionError, SignInError,
};
use sea_orm::ConnectionTrait;

const NOW: u64 = 1_760_000_000;

/// Connects when the gate is set; `None` skips the suite quietly.
///
/// Each test gets its own database — named after the gate URL's database plus
/// the test's unique suffix — so parallel tests and repeated runs never share
/// migration state. The next run removes the previous test database.
async fn store(name: &str) -> Option<PostgresFrontDoorStore> {
    let url = database(name).await?;
    let store = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .expect("connect with migrations");
    Some(store)
}

/// Creates the per-test database and returns its URL; `None` skips quietly.
async fn database(name: &str) -> Option<String> {
    let url = std::env::var("FRONT_DOOR_DATABASE_URL").unwrap_or_default();
    let url = url.trim();
    if url.is_empty() {
        assert!(
            std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
            "FRONT_DOOR_DATABASE_URL is required for the Postgres CI proof"
        );
        return None;
    }
    let mut split: Vec<_> = url.rsplitn(2, '/').collect();
    let db = split.remove(0);
    let base = split.remove(0);
    let test_db = format!("{db}_t_{name}");
    let admin = sea_orm::Database::connect(&format!("{base}/postgres"))
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
    Some(format!("{base}/{test_db}"))
    // The per-test database is intentionally left in place after the test;
    // the next run's `DROP DATABASE IF EXISTS` clears any stale leftover,
    // so a crashed or failed test never wedges the suite.
}

#[tokio::test]
async fn a_database_migrated_under_the_shared_table_keeps_its_init() {
    let Some(url) = database("legacy_init").await else {
        return;
    };
    let store = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .expect("first migrate");
    let (patient, id, _) = invited_account(&store, "pg-proof").await;
    // Rewind the records to what the chain wrote before it had its own
    // table: the init as `mod` in SeaORM's default `seaql_migrations`.
    let db = sea_orm::Database::connect(&url).await.expect("connection");
    db.execute_unprepared("DROP TABLE duet_conversation_events,duet_conversation_attempts; DELETE FROM front_door_seaql_migrations WHERE version='m20261022_duet_execution'").await.unwrap();
    db.execute_unprepared("DROP TRIGGER duet_permission_interrupt ON front_door_permissions; DROP FUNCTION duet_permission_interrupt(); DROP FUNCTION duet_interrupt_accepted(uuid); DROP TABLE duet_conversation_messages,duet_conversation_inputs,duet_conversation_state; DELETE FROM front_door_seaql_migrations WHERE version='m20261021_duet_conversation'").await.unwrap();
    // Fixture-only removal of the new empty outbox. Production rollback refuses
    // to destroy retained privacy receipts.
    db.execute_unprepared("DROP TABLE front_door_luna_restrictions; DELETE FROM front_door_seaql_migrations WHERE version = 'm20261021_luna_journal'").await.unwrap();
    // Recreate the actual old schema before simulating its legacy ledger.
    <front_door::migration::Migrator as sea_orm_migration::MigratorTrait>::down(&db, Some(1))
        .await
        .expect("remove new identity extension for legacy upgrade proof");
    db.execute_unprepared(&format!(
        "DROP TABLE {table}; \
         CREATE TABLE seaql_migrations (version varchar PRIMARY KEY, applied_at bigint NOT NULL); \
         INSERT INTO seaql_migrations VALUES ('mod', 42);",
        table = front_door::migration::MIGRATION_TABLE,
    ))
    .await
    .expect("legacy records");

    // Runtime startup must not perform the migration job's legacy adoption.
    let read_only_url = format!("{url}?options=-c%20default_transaction_read_only%3Don");
    let runtime = PostgresFrontDoorStore::connect_without_migrations(&read_only_url)
        .await
        .expect("runtime on legacy schema");
    assert_eq!(runtime.link(id).await.unwrap(), patient);
    assert!(
        !schema_tables(&url)
            .await
            .iter()
            .any(|table| table == front_door::migration::MIGRATION_TABLE)
    );

    // Migrating again adopts the init rather than re-running it over the
    // existing tables, and the rows the init's tables hold survive.
    let store = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .expect("migrate over the legacy records");
    assert_eq!(store.link(id).await.unwrap(), patient);
    let recorded = db
        .query_all_raw(sea_orm::Statement::from_string(
            sea_orm::DbBackend::Postgres,
            format!(
                "SELECT version, applied_at FROM {}",
                front_door::migration::MIGRATION_TABLE
            ),
        ))
        .await
        .expect("front-door records")
        .into_iter()
        .map(|row| {
            (
                row.try_get::<String>("", "version").unwrap(),
                row.try_get::<i64>("", "applied_at").unwrap(),
            )
        })
        .collect::<Vec<_>>();
    assert!(recorded.contains(&("mod".to_owned(), 42)));
    assert_eq!(
        recorded.len(),
        5,
        "legacy init plus identity, journal and conversation extensions"
    );
    assert!(
        recorded
            .iter()
            .any(|(name, _)| name == "m20261020_identity")
    );
    store.migrate().await.expect("a second migrate is a no-op");
}

/// Catalog read only: unlike Migrator status helpers, never installs a ledger.
async fn schema_tables(url: &str) -> Vec<String> {
    let db = sea_orm::Database::connect(url)
        .await
        .expect("catalog connection");
    let rows = db.query_all_raw(sea_orm::Statement::from_string(
        sea_orm::DbBackend::Postgres,
        "SELECT tablename::text AS name FROM pg_tables WHERE schemaname = 'public' ORDER BY tablename",
    )).await.expect("catalog read");
    rows.into_iter()
        .map(|row| row.try_get("", "name").unwrap())
        .collect()
}

#[tokio::test]
async fn runtime_connection_leaves_schema_untouched_and_migration_job_reruns() {
    let Some(url) = database("migration_separation").await else {
        return;
    };
    assert!(schema_tables(&url).await.is_empty());
    let runtime = PostgresFrontDoorStore::connect_without_migrations(&url)
        .await
        .expect("runtime connection");
    assert!(
        runtime.check_schema().await.is_err(),
        "empty schema must fail readiness"
    );
    assert!(
        runtime
            .begin_sign_in("absent@test.invalid", NOW)
            .await
            .is_err()
    );
    let _compatibility = PostgresFrontDoorStore::connect(&url)
        .await
        .expect("compatibility connection");
    assert!(
        schema_tables(&url).await.is_empty(),
        "runtime must not install even the migration ledger"
    );

    let migrate = || {
        let status = std::process::Command::new(env!("CARGO_BIN_EXE_front-door-migrate"))
            .env("FRONT_DOOR_DATABASE_URL", &url)
            .status()
            .expect("migration job");
        assert!(status.success(), "migration job failed");
    };
    migrate();
    let expected = vec![
        "duet_conversation_attempts",
        "duet_conversation_events",
        "duet_conversation_inputs",
        "duet_conversation_messages",
        "duet_conversation_state",
        "front_door_accounts",
        "front_door_consents",
        "front_door_identities",
        "front_door_invitations",
        "front_door_links",
        "front_door_luna_restrictions",
        "front_door_permissions",
        "front_door_premium",
        "front_door_seaql_migrations",
        "front_door_sessions",
        "front_door_sign_in_tokens",
    ];
    assert_eq!(schema_tables(&url).await, expected);
    let (_, id, session) = invited_account(&runtime, "separation-proof").await;
    migrate();
    runtime.migrate().await.expect("explicit migration rerun");
    let read_only_url = format!("{url}?options=-c%20default_transaction_read_only%3Don");
    let reopened = PostgresFrontDoorStore::connect_without_migrations(&read_only_url)
        .await
        .expect("reopen runtime with database-enforced read-only transactions");
    reopened
        .check_schema()
        .await
        .expect("read-only schema check");
    assert_eq!(schema_tables(&url).await, expected);
    assert_eq!(reopened.session(&session, NOW).await.unwrap(), id);
    assert!(reopened.account(id).await.is_some());
    assert!(
        reopened.migrate().await.is_err(),
        "read-only runtime cannot migrate"
    );
}

fn patient(practice: &str, id: &str) -> RosterPatient {
    RosterPatient {
        practice_id: practice.to_owned(),
        patient_id: id.to_owned(),
    }
}

/// One account per test: a unique email, invited and redeemed, plus the
/// returned session. The unique email keeps concurrent runs independent.
async fn invited_account(
    store: &PostgresFrontDoorStore,
    practice: &str,
) -> (RosterPatient, uuid::Uuid, String) {
    let patient = patient(practice, "patient-1");
    let email = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());
    let token = store
        .invite(&patient, &email, "PG Proof", NOW)
        .await
        .expect("invitation");
    let (id, session) = store
        .redeem_invitation(&email, &token, NOW)
        .await
        .expect("redemption");
    (patient, id, session)
}

#[tokio::test]
async fn an_invitation_redeems_once_and_writes_the_link() {
    let Some(store) = store("invitation").await else {
        return;
    };
    let patient = patient("pg-proof", "patient-1");
    let email = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());
    let token = store
        .invite(&patient, &email, "PG Proof", NOW)
        .await
        .expect("invitation");
    let pending = store.invitation(&email).await.expect("pending invitation");
    assert_eq!(pending.patient, patient);
    assert_eq!(
        pending.expires_at,
        NOW + INVITATION_TTL_SECONDS,
        "an invitation is redeemable for fourteen days"
    );
    let (id, session) = store
        .redeem_invitation(&email, &token, NOW)
        .await
        .expect("first redeem");
    assert_eq!(store.session(&session, NOW).await.unwrap(), id);
    let account = store.account_by_email(&email).await.expect("account");
    assert_eq!(account.link, Some(patient.clone()));
    assert_eq!(
        store
            .redeem_invitation(&email, &token, NOW)
            .await
            .unwrap_err(),
        SignInError::Invalid,
        "an invitation redeems once"
    );
    assert_eq!(
        store.invitation(&email).await,
        None,
        "the redeemed invitation is gone"
    );
}

#[tokio::test]
async fn an_account_email_cannot_be_invited_again() {
    let Some(store) = store("invite_conflict").await else {
        return;
    };
    let (patient, _, _) = invited_account(&store, "pg-proof").await;
    let email = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());
    store
        .invite(&patient, &email, "New Patient", NOW)
        .await
        .expect("first invitation");
    // A different email still invites, and the same email after redemption
    // answers conflict.
    let token = store
        .invite(&patient, &email, "New Patient", NOW)
        .await
        .expect("re-invite replaces the pending invitation");
    store
        .redeem_invitation(&email, &token, NOW)
        .await
        .expect("redeem");
    assert_eq!(
        store
            .invite(&patient, &email, "New Patient", NOW)
            .await
            .unwrap_err(),
        InviteError::Conflict,
        "the practice cannot re-invite a linked account"
    );
}

#[tokio::test]
async fn a_sign_in_token_verifies_once_and_mints_one_session() {
    let Some(store) = store("sign_in_token").await else {
        return;
    };
    let (_, id, _) = invited_account(&store, "pg-proof").await;
    let account = store.account(id).await.expect("account");
    let token = store
        .begin_sign_in(&account.email, NOW)
        .await
        .expect("token");
    let (who, session) = store
        .complete_sign_in(&account.email, &token, NOW)
        .await
        .expect("first verify");
    assert_eq!(who, id);
    assert!(session.starts_with("codaf-"));
    assert_eq!(
        store
            .complete_sign_in(&account.email, &token, NOW)
            .await
            .unwrap_err(),
        SignInError::Invalid,
        "a verified token never verifies again"
    );
    assert!(
        store
            .session(&session, NOW + SESSION_TTL_SECONDS)
            .await
            .is_err(),
        "an expired session stops resolving"
    );
    assert_eq!(store.session(&session, NOW).await.unwrap(), id);
}

#[tokio::test]
async fn consent_grants_withdraws_and_restores() {
    let Some(store) = store("consent").await else {
        return;
    };
    let (_, id, _) = invited_account(&store, "pg-proof").await;
    assert_eq!(
        store.consent(id).await.unwrap(),
        Consent::None,
        "consent is granted, never assumed"
    );
    store.grant_consent(id, NOW + 1).await.unwrap();
    assert_eq!(
        store.consent(id).await.unwrap(),
        Consent::Granted { at: NOW + 1 }
    );
    store.withdraw_consent(id, NOW + 2).await.unwrap();
    assert_eq!(
        store.consent(id).await.unwrap(),
        Consent::Withdrawn {
            at: NOW + 1,
            withdrawn_at: NOW + 2,
        },
        "what was already shared stays dated evidence"
    );
    store.grant_consent(id, NOW + 3).await.unwrap();
    assert_eq!(
        store.consent(id).await.unwrap(),
        Consent::Granted { at: NOW + 3 },
        "re-consenting restores sharing"
    );
    assert_eq!(
        store.consent(uuid::Uuid::nil()).await.unwrap_err(),
        LinkError::Unlinked,
        "an unknown account carries no consent"
    );
}

#[tokio::test]
async fn premium_defaults_free_and_follows_the_status_per_patient() {
    let Some(store) = store("premium").await else {
        return;
    };
    let (luna, _, _) = invited_account(&store, "pg-proof").await;
    assert_eq!(
        store.premium(&luna).await,
        Premium {
            premium: false,
            cadence: CADENCE_WEEKLY.to_owned(),
        },
        "a patient without a status row reads free"
    );
    store.set_premium(&luna, true, NOW).await.unwrap();
    assert_eq!(
        store.premium(&luna).await,
        Premium {
            premium: true,
            cadence: CADENCE_WEEKLY_DAILY_CHECK.to_owned(),
        },
        "paying patients add the daily check"
    );
    store.set_premium(&luna, false, NOW).await.unwrap();
    assert_eq!(
        store.premium(&luna).await,
        Premium {
            premium: false,
            cadence: CADENCE_WEEKLY.to_owned(),
        }
    );
    assert_eq!(
        store
            .set_premium(&patient("pg-proof", "nobody"), true, NOW)
            .await
            .unwrap_err(),
        LinkError::Unlinked,
        "premium status is written only for a linked patient"
    );
}

#[tokio::test]
async fn linked_accounts_lists_the_practice_s_rows() {
    let Some(store) = store("linked_accounts").await else {
        return;
    };
    let (_, first, _) = invited_account(&store, "pg-proof").await;
    let (_, second, _) = invited_account(&store, "pg-proof").await;
    let (_, _, _) = invited_account(&store, "other-clinic").await;
    let mut listed = store
        .linked_accounts("pg-proof")
        .await
        .into_iter()
        .map(|(account, _)| account.id)
        .collect::<Vec<_>>();
    let mut expected = vec![first, second];
    expected.sort();
    listed.sort();
    assert_eq!(listed, expected);
}

/// A memory store against the same contract, so a regression in one store
/// cannot pass while the other keeps the truth.
#[tokio::test]
async fn the_memory_store_answers_the_same_contract() {
    let memory = MemoryFrontDoorStore::new();
    let patient = patient("pg-proof", "patient-1");
    let token = memory
        .invite(&patient, "luna@pg-proof.test", "Luna Park", NOW)
        .await
        .expect("invitation");
    let (id, session) = memory
        .redeem_invitation("luna@pg-proof.test", &token, NOW)
        .await
        .expect("redeem");
    assert_eq!(memory.session(&session, NOW).await.unwrap(), id);
    let Account { link, .. } = memory.account(id).await.expect("account");
    assert_eq!(link, Some(patient));
    memory.grant_consent(id, NOW).await.unwrap();
    assert!(matches!(
        memory.consent(id).await.unwrap(),
        Consent::Granted { .. }
    ));
    let _ = SessionError::Invalid;
}
