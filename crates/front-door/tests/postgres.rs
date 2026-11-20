//! The Postgres store against a real server, gated on `FRONT_DOOR_DATABASE_URL`.
//!
//! The devcontainer compose service provides that URL; without it the store's
//! contract is covered by the memory-store unit tests, and these stay quiet.

use front_door::{
    Account, ConnectionEndReason, ConnectionStanding, ForMyDoctorError, ForMyDoctorSendRequest,
    FrontDoorStore, InvitationCheckError, InvitationCredential, InvitationOpenStanding,
    InviteError, InvitePatientParams, LinkError, MemoryFrontDoorStore, PostgresFrontDoorStore,
    RosterConnectionStatus, RosterPatient, SessionError, SignInError,
};
use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, QuerySelect, TransactionTrait};

#[path = "common/disposable.rs"]
mod disposable;

const NOW: u64 = 1_760_000_000;

#[tokio::test]
async fn concurrent_sign_in_consumes_one_token_and_mints_one_session() {
    let Some(url) = database("concurrent_sign_in").await else {
        return;
    };
    let store = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let (_, id, _) = invited_account(&store, "pg-proof").await;
    let email = store.account(id).await.unwrap().email;
    let token = store.begin_sign_in(&email, NOW).await.unwrap();
    let db = sea_orm::Database::connect(&url).await.unwrap();
    let held = db.begin().await.unwrap();
    front_door::migration::sign_in_token::Entity::find()
        .lock_exclusive()
        .one(&held)
        .await
        .unwrap()
        .unwrap();
    let redeem = || {
        let (store, email, token) = (store.clone(), email.clone(), token.clone());
        tokio::spawn(async move { store.complete_sign_in(&email, &token, NOW).await })
    };
    let first = redeem();
    let second = redeem();
    wait_for_two_lock_waiters(&db).await;
    held.commit().await.unwrap();
    let outcomes = [first.await.unwrap(), second.await.unwrap()];
    assert_eq!(
        outcomes.iter().filter(|result| result.is_ok()).count(),
        1,
        "{outcomes:?}"
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|result| **result == Err(SignInError::Invalid))
            .count(),
        1
    );
    let (who, session) = outcomes.into_iter().find_map(Result::ok).unwrap();
    assert_eq!(who, id);
    assert_eq!(store.session(&session, NOW).await.unwrap(), id);
    assert_eq!(
        front_door::migration::session::Entity::find()
            .count(&db)
            .await
            .unwrap(),
        2
    );
}

#[tokio::test]
async fn concurrent_invitation_redemption_consumes_once_and_connects_one_account() {
    let Some(url) = database("concurrent_invite_redeem").await else {
        return;
    };
    let store = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let patient = patient("pg-proof", "patient-race");
    let email = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());
    let token = store
        .invite(&patient, &email, "PG Race", NOW)
        .await
        .unwrap();
    let db = sea_orm::Database::connect(&url).await.unwrap();
    let held = db.begin().await.unwrap();
    front_door::migration::invitation::Entity::find()
        .lock_exclusive()
        .one(&held)
        .await
        .unwrap()
        .unwrap();
    let redeem = || {
        let (store, email, token) = (store.clone(), email.clone(), token.clone());
        tokio::spawn(async move { store.redeem_invitation(&email, &token, NOW).await })
    };
    let first = redeem();
    let second = redeem();
    wait_for_two_lock_waiters(&db).await;
    held.commit().await.unwrap();
    let outcomes = [first.await.unwrap(), second.await.unwrap()];
    assert_eq!(
        outcomes.iter().filter(|result| result.is_ok()).count(),
        1,
        "{outcomes:?}"
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|result| **result == Err(SignInError::Invalid))
            .count(),
        1
    );
    let (winner_id, _) = outcomes.into_iter().find_map(Result::ok).unwrap();
    assert!(matches!(
        store.roster_connection(&patient).await.unwrap(),
        RosterConnectionStatus::Connected { account_id, .. } if account_id == winner_id
    ));
}

async fn wait_for_two_lock_waiters(db: &sea_orm::DatabaseConnection) {
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let row = db.query_one_raw(sea_orm::Statement::from_string(
                sea_orm::DbBackend::Postgres,
                "SELECT count(*) AS waiting FROM pg_stat_activity WHERE datname = current_database() AND wait_event_type = 'Lock'",
            )).await.unwrap().unwrap();
            if row.try_get::<i64>("", "waiting").unwrap() == 2 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("both callers must contend for the held row");
}

async fn store(name: &str) -> Option<PostgresFrontDoorStore> {
    let url = database(name).await?;
    let store = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .expect("connect with migrations");
    Some(store)
}

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
    disposable::replace_database(&admin, &test_db).await;
    admin.close().await.expect("close admin connection");
    Some(format!("{base}/{test_db}"))
}

#[tokio::test]
async fn a_database_migrated_under_the_shared_table_keeps_its_init() {
    let Some(url) = database("legacy_init").await else {
        return;
    };
    let store = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .expect("first migrate");
    let (_, id, session) = invited_account(&store, "pg-proof").await;
    let db = sea_orm::Database::connect(&url).await.expect("connection");
    rewind_to_legacy_init(&db).await;
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
    assert_eq!(runtime.session(&session, NOW).await.unwrap(), id);
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
    assert_eq!(store.session(&session, NOW).await.unwrap(), id);
    let versions = recorded(&db).await;
    assert!(versions.contains(&(front_door::migration::INIT_MIGRATION_NAME.to_owned(), 42)));
    assert_eq!(
        versions.len(),
        10,
        "legacy init plus all migrations including invitation lifecycle"
    );
    assert!(
        versions
            .iter()
            .any(|(name, _)| name == "m20261120_multi_practice_connections")
    );
    store.migrate().await.expect("a second migrate is a no-op");

    rewind_to_legacy_init(&db).await;
    db.execute_unprepared(&format!(
        "DROP TABLE {table}; \
         CREATE TABLE front_door_seaql_migrations (version varchar PRIMARY KEY, applied_at bigint NOT NULL); \
         INSERT INTO front_door_seaql_migrations VALUES ('mod', 43);",
        table = front_door::migration::MIGRATION_TABLE,
    ))
    .await
    .expect("interim records");
    let store = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .expect("migrate over the interim records");
    assert_eq!(store.session(&session, NOW).await.unwrap(), id);
    let adopted = recorded(&db).await;
    assert!(adopted.iter().any(|(name, applied_at)| {
        name == front_door::migration::INIT_MIGRATION_NAME && *applied_at == 43
    }));
    assert_eq!(adopted.len(), 10);
    for legacy in ["seaql_migrations", "front_door_seaql_migrations"] {
        let rows = db
            .query_all_raw(sea_orm::Statement::from_string(
                sea_orm::DbBackend::Postgres,
                format!("SELECT version FROM {legacy}"),
            ))
            .await
            .expect("legacy ledger kept");
        assert_eq!(rows.len(), 1, "{legacy} keeps its row");
    }
    store.migrate().await.expect("a second migrate is a no-op");
}

async fn rewind_to_legacy_init(db: &sea_orm::DatabaseConnection) {
    <front_door::migration::Migrator as sea_orm_migration::MigratorTrait>::down(db, Some(4))
        .await
        .expect("restore schema before invitation lifecycle, multi-practice, for-my-doctor shares, and cadence cleanup");
    db.execute_unprepared("DROP TABLE duet_model_execution_switch; DROP FUNCTION duet_switch_append_only(); DROP INDEX duet_conversation_allowance; ALTER TABLE duet_conversation_inputs DROP COLUMN admitted_at; DELETE FROM front_door_migrations WHERE version='m20261023_duet_admission'").await.unwrap();
    db.execute_unprepared("DROP TABLE duet_attempt_events,duet_conversation_attempts; DELETE FROM front_door_migrations WHERE version='m20261022_duet_execution'").await.unwrap();
    db.execute_unprepared("DROP TRIGGER duet_permission_interrupt ON front_door_permissions; DROP FUNCTION duet_permission_interrupt(); DROP FUNCTION duet_interrupt_accepted(uuid); DROP TABLE duet_conversation_messages,duet_conversation_inputs,duet_conversation_state; DELETE FROM front_door_migrations WHERE version='m20261021_duet_conversation'").await.unwrap();
    db.execute_unprepared("DROP TABLE front_door_luna_restrictions; DELETE FROM front_door_migrations WHERE version = 'm20261021_luna_journal'").await.unwrap();
    <front_door::migration::Migrator as sea_orm_migration::MigratorTrait>::down(db, Some(1))
        .await
        .expect("remove new identity extension for legacy upgrade proof");
}

async fn recorded(db: &sea_orm::DatabaseConnection) -> Vec<(String, i64)> {
    db.query_all_raw(sea_orm::Statement::from_string(
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
    .collect()
}

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
        "duet_attempt_events",
        "duet_conversation_attempts",
        "duet_conversation_inputs",
        "duet_conversation_messages",
        "duet_conversation_state",
        "duet_model_execution_switch",
        "front_door_accounts",
        "front_door_connections",
        "front_door_for_my_doctor_shares",
        "front_door_identities",
        "front_door_invitation_credentials",
        "front_door_invitations",
        "front_door_luna_restrictions",
        "front_door_migrations",
        "front_door_permissions",
        "front_door_sessions",
        "front_door_shared_records",
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
async fn an_invitation_redeems_once_without_expiry_and_creates_connection() {
    let Some(store) = store("invitation").await else {
        return;
    };
    let patient = patient("pg-proof", "patient-1");
    let email = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());
    assert!(matches!(
        store.roster_connection(&patient).await.unwrap(),
        RosterConnectionStatus::NotInvited
    ));
    let token = store
        .invite(&patient, &email, "PG Proof", NOW)
        .await
        .expect("invitation");
    let pending = store
        .invitation_for(&patient)
        .await
        .expect("pending invitation");
    assert_eq!(pending.patient, patient);
    assert_eq!(pending.invited_at, NOW);
    assert!(matches!(
        store.roster_connection(&patient).await.unwrap(),
        RosterConnectionStatus::Invited { invited_at, .. } if invited_at == NOW
    ));

    let later = NOW + 365 * 86_400;
    let (id, session) = store
        .redeem_invitation(&email, &token, later)
        .await
        .expect("old invitation still redeems without expiry");
    assert_eq!(
        store
            .session(&session, later + 1000 * 86_400)
            .await
            .unwrap(),
        id
    );
    let account = store.account_by_email(&email).await.expect("account");
    assert_eq!(account.active_connection_at(&patient), Some(later));
    assert_eq!(
        store
            .redeem_invitation(&email, &token, later)
            .await
            .unwrap_err(),
        SignInError::Invalid,
        "an invitation redeems once"
    );
    assert_eq!(
        store.invitation_for(&patient).await,
        None,
        "the redeemed invitation is gone"
    );
}

#[tokio::test]
async fn multi_practice_connections_reuse_existing_account_and_reinvite_after_ending() {
    let Some(store) = store("multi_practice").await else {
        return;
    };
    let (practice_a, id, _) = invited_account(&store, "pg-practice-a").await;
    let email = store.account(id).await.unwrap().email;

    // Inviting the same already-Connected roster patient returns Conflict.
    assert_eq!(
        store
            .invite(&practice_a, &email, "PG Proof", NOW + 1)
            .await
            .unwrap_err(),
        InviteError::Conflict,
    );

    // Inviting the same existing Duet account from two more practices works without conflict.
    let practice_b = patient("pg-practice-b", "patient-b");
    let practice_c = patient("pg-practice-c", "patient-c");
    let token_b = store
        .invite(&practice_b, &email, "PG Proof B", NOW + 2)
        .await
        .expect("existing account can be invited to practice B");
    let token_c = store
        .invite(&practice_c, &email, "PG Proof C", NOW + 3)
        .await
        .expect("practice C open invite coexists with practice B open invite on same email");

    let (id_b, _) = store
        .redeem_invitation(&email, &token_b, NOW + 4)
        .await
        .expect("redeem B");
    assert_eq!(id_b, id);
    assert!(
        store.invitation_for(&practice_c).await.is_some(),
        "redeeming B leaves C's open invite intact"
    );
    let (id_c, _) = store
        .redeem_invitation(&email, &token_c, NOW + 5)
        .await
        .expect("redeem C");
    assert_eq!(id_c, id);

    // Multi-practice For my doctor requires explicit target when >1 active connection exists.
    let ambiguous = store
        .send_for_my_doctor(
            id,
            ForMyDoctorSendRequest {
                share_key: "ambig".into(),
                title: "Summary".into(),
                summary_text: "Details".into(),
                origin_label: None,
                target: None,
            },
            NOW + 6,
        )
        .await
        .unwrap_err();
    assert!(matches!(ambiguous, ForMyDoctorError::InvalidInput(_)));
    let targeted = store
        .send_for_my_doctor(
            id,
            ForMyDoctorSendRequest {
                share_key: "share-b".into(),
                title: "For B".into(),
                summary_text: "Details for B".into(),
                origin_label: None,
                target: Some(practice_b.clone()),
            },
            NOW + 6,
        )
        .await
        .unwrap();
    assert_eq!(targeted.patient, practice_b);

    // Ending practice A leaves B and C connected, blocks sends to A, and allows re-invite to A.
    store
        .end_connection(id, &practice_a, ConnectionEndReason::PatientLeft, NOW + 10)
        .await
        .unwrap();
    assert!(matches!(
        store.roster_connection(&practice_a).await.unwrap(),
        RosterConnectionStatus::Ended { account_id, ended_at, .. } if account_id == id && ended_at == NOW + 10
    ));
    assert!(
        store
            .roster_connection(&practice_b)
            .await
            .unwrap()
            .is_connected()
    );
    assert!(matches!(
        store
            .send_for_my_doctor(
                id,
                ForMyDoctorSendRequest {
                    share_key: "after-end".into(),
                    title: "Ended".into(),
                    summary_text: "Blocked".into(),
                    origin_label: None,
                    target: Some(practice_a.clone()),
                },
                NOW + 11,
            )
            .await
            .unwrap_err(),
        ForMyDoctorError::Unlinked
    ));

    let reinvite_a = store
        .invite(&practice_a, &email, "PG Proof A", NOW + 20)
        .await
        .expect("Ended patient can be re-invited");
    let (rejoined_id, _) = store
        .redeem_invitation(&email, &reinvite_a, NOW + 30)
        .await
        .expect("rejoin A");
    assert_eq!(rejoined_id, id);
    let conns = store.connections(id).await.unwrap();
    assert_eq!(conns.len(), 3);
    assert!(
        conns
            .iter()
            .all(|c| matches!(c.standing, ConnectionStanding::Connected { .. }))
    );
}

#[tokio::test]
async fn a_sign_in_token_verifies_once_and_mints_non_expiring_session() {
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
    assert_eq!(
        store.session(&session, NOW + 3650 * 86_400).await.unwrap(),
        id,
        "sessions never expire with time"
    );
    assert!(matches!(
        store.connections(uuid::Uuid::nil()).await,
        Err(LinkError::Unlinked)
    ));
}

#[tokio::test]
async fn connected_accounts_lists_the_practice_s_active_rows() {
    let Some(store) = store("connected_accounts").await else {
        return;
    };
    let (_, first, _) = invited_account(&store, "pg-proof").await;
    let patient2 = patient("pg-proof", "patient-2");
    let email2 = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());
    let tok2 = store.invite(&patient2, &email2, "Two", NOW).await.unwrap();
    let (second, _) = store.redeem_invitation(&email2, &tok2, NOW).await.unwrap();
    let (_, _, _) = invited_account(&store, "other-clinic").await;
    let mut listed = store
        .connected_accounts("pg-proof")
        .await
        .into_iter()
        .map(|(account, _)| account.id)
        .collect::<Vec<_>>();
    let mut expected = vec![first, second];
    expected.sort();
    listed.sort();
    assert_eq!(listed, expected);

    let map = store.practice_connections("pg-proof").await.unwrap();
    assert_eq!(map.len(), 2);
    assert!(map["patient-1"].is_connected());
    assert!(map["patient-2"].is_connected());
}

#[tokio::test]
async fn the_memory_store_answers_the_same_contract() {
    let memory = MemoryFrontDoorStore::new();
    let patient_a = patient("pg-proof", "patient-1");
    let token = memory
        .invite(&patient_a, "luna@pg-proof.test", "Luna Park", NOW)
        .await
        .expect("invitation");
    let (id, session) = memory
        .redeem_invitation("luna@pg-proof.test", &token, NOW)
        .await
        .expect("redeem");
    assert_eq!(
        memory.session(&session, NOW + 365 * 86_400).await.unwrap(),
        id
    );
    let Account { connections, .. } = memory.account(id).await.expect("account");
    assert_eq!(connections.len(), 1);
    assert_eq!(connections[0].patient, patient_a);
    assert!(matches!(
        connections[0].standing,
        ConnectionStanding::Connected { connected_at } if connected_at == NOW
    ));
    let _ = SessionError::Invalid;
}

#[tokio::test]
async fn postgres_leave_discharge_and_account_delete_preserve_ended_and_fail_closed() {
    let Some(url) = database("leave_discharge_delete").await else {
        return;
    };
    let store = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let p_a = patient("pg-leave-a", "pat-1");
    let p_b = patient("pg-leave-b", "pat-2");
    let p_c = patient("pg-leave-c", "pat-3");
    let email = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());

    let tok_a = store.invite(&p_a, &email, "Luna", NOW).await.unwrap();
    let (id, session) = store.redeem_invitation(&email, &tok_a, NOW).await.unwrap();
    let tok_b = store.invite(&p_b, &email, "Luna", NOW).await.unwrap();
    store
        .redeem_invitation(&email, &tok_b, NOW + 1)
        .await
        .unwrap();
    let tok_c = store.invite(&p_c, &email, "Luna", NOW).await.unwrap();
    store
        .redeem_invitation(&email, &tok_c, NOW + 2)
        .await
        .unwrap();

    // Shared records & shares exist before leaving/discharging.
    store
        .send_for_my_doctor(
            id,
            ForMyDoctorSendRequest {
                share_key: "share-a".into(),
                title: "Pre-leave share".into(),
                summary_text: "Kept on practice side".into(),
                origin_label: None,
                target: Some(p_a.clone()),
            },
            NOW + 3,
        )
        .await
        .unwrap();

    // X1: Patient leaves pg-leave-a.
    let left = store
        .leave_connection(id, "pg-leave-a", NOW + 10)
        .await
        .unwrap();
    assert_eq!(left.len(), 1);
    assert!(matches!(
        store.roster_connection(&p_a).await.unwrap(),
        RosterConnectionStatus::Ended {
            reason: ConnectionEndReason::PatientLeft,
            ended_at,
            ..
        } if ended_at == NOW + 10
    ));
    assert!(store.roster_connection(&p_b).await.unwrap().is_connected());
    assert!(
        store
            .pending_for_my_doctor_shares_for_practice("pg-leave-a")
            .await
            .unwrap()
            .is_empty(),
        "new pickup stops immediately when Ended"
    );

    // X2: Practice discharges pg-leave-b.
    let discharged = store.discharge_connection(&p_b, NOW + 20).await.unwrap();
    assert!(matches!(
        discharged.standing,
        ConnectionStanding::Ended {
            ended_at,
            reason: ConnectionEndReason::PracticeDischarged,
            ..
        } if ended_at == NOW + 20
    ));
    assert!(matches!(
        store.roster_connection(&p_b).await.unwrap(),
        RosterConnectionStatus::Ended {
            reason: ConnectionEndReason::PracticeDischarged,
            ..
        }
    ));

    // X3: Account deleted ends remaining pg-leave-c and revokes sessions while keeping Ended roster status.
    assert_eq!(
        store
            .end_all_connections_for_account(id, NOW + 30)
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        store
            .end_all_connections_for_account(id, NOW + 31)
            .await
            .unwrap(),
        0,
        "idempotent across retries"
    );
    assert!(matches!(
        store.roster_connection(&p_c).await.unwrap(),
        RosterConnectionStatus::Ended {
            reason: ConnectionEndReason::AccountDeleted,
            ended_at,
            ..
        } if ended_at == NOW + 30
    ));
    assert_eq!(
        store.session(&session, NOW + 32).await.unwrap_err(),
        SessionError::Invalid
    );

    // Outage never reads as NotInvited.
    let db = sea_orm::Database::connect(&url).await.unwrap();
    db.execute_unprepared("DROP TABLE front_door_connections")
        .await
        .unwrap();
    assert!(matches!(
        store.roster_connection(&p_c).await,
        Err(ForMyDoctorError::Storage(_))
    ));
    assert!(matches!(
        store.practice_connections("pg-leave-c").await,
        Err(ForMyDoctorError::Storage(_))
    ));
}

#[tokio::test]
async fn cod53_invitation_resend_cancel_atomic_dob_lock_and_share_persist_in_postgres() {
    let Some(url) = database("cod53_invitations").await else {
        return;
    };
    let store = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let rp = patient("pg-cod53", "pat-1");

    // 1. Invite + Resend keeps BOTH link tokens & short codes open (`Rule 4`).
    let issued1 = store
        .invite_patient(InvitePatientParams {
            patient: rp.clone(),
            email: "chart-pat1@pg-cod53.test".to_owned(),
            display_name: "Tamika Washington".to_owned(),
            practice_name: "River Bend Clinic".to_owned(),
            physician_name: "Dr. Roger".to_owned(),
            expected_birth_date: Some("1987-02-18".to_owned()),
            now: NOW,
        })
        .await
        .unwrap();
    let issued2 = store.resend_invitation(&rp, NOW + 5).await.unwrap();
    assert_ne!(issued1.link_token, issued2.link_token);
    assert_ne!(issued1.short_code, issued2.short_code);
    // Calling invite_patient again while open must conflict without clobbering issued1/issued2.
    assert_eq!(
        store
            .invite_patient(InvitePatientParams {
                patient: rp.clone(),
                email: "chart-pat1@pg-cod53.test".to_owned(),
                display_name: "Tamika Washington".to_owned(),
                practice_name: "River Bend Clinic".to_owned(),
                physician_name: "Dr. Roger".to_owned(),
                expected_birth_date: Some("1987-02-18".to_owned()),
                now: NOW + 5,
            })
            .await
            .unwrap_err(),
        front_door::InviteError::Conflict
    );

    // Concurrent wrong DOB checks serialize cleanly via row lock with zero lost strikes (`COD-48`).
    let db = sea_orm::Database::connect(&url).await.unwrap();
    let held = db.begin().await.unwrap();
    front_door::migration::invitation::Entity::find()
        .lock_exclusive()
        .one(&held)
        .await
        .unwrap()
        .unwrap();
    let c1 = InvitationCredential::LinkToken(issued1.link_token.clone());
    let c2 = InvitationCredential::ShortCode(issued2.short_code.clone());
    let s1 = {
        let store = store.clone();
        tokio::spawn(async move {
            store
                .verify_invitation_dob(&c1, "1987-02-19", NOW + 6)
                .await
        })
    };
    let s2 = {
        let store = store.clone();
        tokio::spawn(async move {
            store
                .verify_invitation_dob(&c2, "1987-02-19", NOW + 7)
                .await
        })
    };
    wait_for_two_lock_waiters(&db).await;
    held.commit().await.unwrap();
    let res = [s1.await.unwrap(), s2.await.unwrap()];
    let mut rem: Vec<u8> = res
        .into_iter()
        .map(|r| match r.unwrap_err() {
            InvitationCheckError::WrongBirthDate { attempts_remaining } => attempts_remaining,
            other => panic!("expected WrongBirthDate, got {other:?}"),
        })
        .collect();
    rem.sort_unstable();
    assert_eq!(rem, vec![3, 4]);

    // Strikes 3, 4, then 5 lock the invitation (`Rule 7`) and forbid Resend (`COD-49`).
    let cred2 = InvitationCredential::ShortCode(issued2.short_code.clone());
    assert_eq!(
        store
            .verify_invitation_dob(&cred2, "1987-02-19", NOW + 8)
            .await
            .unwrap_err(),
        InvitationCheckError::WrongBirthDate {
            attempts_remaining: 2
        }
    );
    assert_eq!(
        store
            .verify_invitation_dob(&cred2, "1987-02-19", NOW + 9)
            .await
            .unwrap_err(),
        InvitationCheckError::WrongBirthDate {
            attempts_remaining: 1
        }
    );
    assert_eq!(
        store
            .verify_invitation_dob(&cred2, "1987-02-19", NOW + 10)
            .await
            .unwrap_err(),
        InvitationCheckError::Locked
    );
    assert_eq!(
        store.resend_invitation(&rp, NOW + 11).await.unwrap_err(),
        InviteError::Locked
    );
    assert!(matches!(
        store.inspect_invitation(&cred2).await.unwrap().standing,
        InvitationOpenStanding::Locked { locked_at } if locked_at == NOW + 10
    ));

    // Cancel deletes invitation + cascades ALL credentials (`Rule 5`) so re-invite works.
    store.cancel_invitation(&rp).await.unwrap();
    assert_eq!(
        store.inspect_invitation(&cred2).await.unwrap_err(),
        InvitationCheckError::NotOpen
    );

    let fresh = store
        .invite_patient(InvitePatientParams {
            patient: rp.clone(),
            email: "chart-pat1@pg-cod53.test".to_owned(),
            display_name: "Tamika Washington".to_owned(),
            practice_name: "River Bend Clinic".to_owned(),
            physician_name: "Dr. Roger".to_owned(),
            expected_birth_date: Some("1987-02-18".to_owned()),
            now: NOW + 12,
        })
        .await
        .unwrap();
    store
        .mark_invitation_delivery_failed(&rp, "smtp relay 550", NOW + 13)
        .await
        .unwrap();
    assert!(matches!(
        store.roster_connection(&rp).await.unwrap(),
        RosterConnectionStatus::Invited {
            delivery_failed_at: Some(_),
            delivery_error: Some(ref e),
            ..
        } if e == "smtp relay 550"
    ));

    // Existing Duet account connects via Share even with different account email (`Rule 9`).
    let other_practice = patient("pg-other", "pat-other");
    let tok_other = store
        .invite(
            &other_practice,
            "existing-duet@pg-cod53.test",
            "Existing",
            NOW,
        )
        .await
        .unwrap();
    let (existing_id, _) = store
        .redeem_invitation("existing-duet@pg-cod53.test", &tok_other, NOW + 1)
        .await
        .unwrap();

    let (acc_id, conn) = store
        .accept_invitation_share(
            Some(existing_id),
            Some("existing-duet@pg-cod53.test"),
            &InvitationCredential::ShortCode(fresh.short_code),
            "1987-02-18",
            NOW + 15,
        )
        .await
        .unwrap();
    assert_eq!(acc_id, existing_id);
    assert_eq!(conn.patient, rp);
    assert_eq!(store.connections(existing_id).await.unwrap().len(), 2);
}
