//! The Postgres store against a real server, gated on `PHYSICIAN_DATABASE_URL`.
//!
//! The devcontainer compose service provides that URL; without it the store's
//! contract is covered by the memory-store unit tests, and these stay quiet.

use physician_auth::{
    Account, AuthStore, Cadence, Membership, PanelError, PhysicianIdentity, PostgresAuthStore,
    PracticeClock, SessionActivity, SessionError, SignInError, StoreUnavailable,
};
use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, QuerySelect, TransactionTrait};

#[path = "common/disposable.rs"]
mod disposable;

const NOW: u64 = 1_760_000_000;

/// Hold the contested row until both independent store calls reach it. The
/// Postgres lock catalog makes the race deterministic without production hooks.
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
    }).await.expect("both redemptions must contend for the held row");
}

#[tokio::test]
async fn a_practice_keeps_the_clock_it_was_created_with() {
    let Some(url) = fresh_database("practice_clock").await else {
        return;
    };
    let store = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let fixed = PracticeClock::Fixed(1_787_846_400);
    store.ensure_practice("synthetic", fixed).await.unwrap();
    store
        .ensure_practice("synthetic", PracticeClock::RealTime)
        .await
        .unwrap();
    store
        .ensure_practice("real", PracticeClock::RealTime)
        .await
        .unwrap();
    assert_eq!(store.practice_clock("synthetic").await.unwrap(), fixed);
    for practice in ["real", "never-created"] {
        assert_eq!(
            store.practice_clock(practice).await.unwrap(),
            PracticeClock::RealTime
        );
    }
}

#[tokio::test]
async fn a_practice_read_from_a_lost_database_is_unavailable_not_empty() {
    let Some(url) = fresh_database("lost_practice").await else {
        return;
    };
    let store = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    store.pool().close().await;
    assert_eq!(
        store.practice_accounts("clinic").await,
        Err(StoreUnavailable)
    );
}

#[tokio::test]
async fn a_patient_whose_practice_could_not_be_read_stays_with_that_practice() {
    let Some(url) = fresh_database("unread_owner").await else {
        return;
    };
    let store = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let (email, _) = invited_account(&store, "owner-clinic").await;
    let doc = store.account_by_email(&email).await.unwrap().identity;
    store
        .register_patient("owner-clinic", "pw-owned")
        .await
        .unwrap();
    store
        .set_membership(&doc, "pw-owned", Membership::Allowed)
        .await
        .unwrap();

    let lost = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    lost.pool().close().await;
    assert_eq!(
        lost.patient_practice("pw-owned").await,
        Err(StoreUnavailable)
    );
    assert_eq!(lost.membership(&doc, "pw-owned").await, Membership::Denied);

    store
        .register_patient("other-clinic", "pw-owned")
        .await
        .unwrap();
    assert_eq!(
        store.patient_practice("pw-owned").await,
        Ok(Some("owner-clinic".to_owned()))
    );
    assert!(store.practice_patients("other-clinic").await.is_empty());
    assert_eq!(store.panel(&doc).await, vec!["pw-owned".to_owned()]);
}

#[tokio::test]
async fn concurrent_sign_in_consumes_one_token_and_mints_one_session() {
    let Some(url) = fresh_database("concurrent_sign_in").await else {
        return;
    };
    let store = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let (email, initial_session) = invited_account(&store, "pg-proof").await;
    let who = store.identity(&initial_session, NOW).await.unwrap();
    let token = store.begin_sign_in(&email, NOW).await.unwrap();
    let db = sea_orm::Database::connect(&url).await.unwrap();
    let held = db.begin().await.unwrap();
    physician_auth::migration::sign_in_token::Entity::find()
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
    let (identity, session) = outcomes.into_iter().find_map(Result::ok).unwrap();
    assert_eq!(identity, who);
    assert_eq!(store.identity(&session, NOW).await.unwrap(), who);
    assert_eq!(
        physician_auth::migration::session::Entity::find()
            .count(&db)
            .await
            .unwrap(),
        2
    );
}

#[tokio::test]
async fn concurrent_patient_invitation_keeps_the_winning_account() {
    let Some(url) = fresh_database("concurrent_patient_invite").await else {
        return;
    };
    let store = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let token = store
        .invite_patient("pg-proof", "pw-elena", NOW)
        .await
        .unwrap();
    let db = sea_orm::Database::connect(&url).await.unwrap();
    let held = db.begin().await.unwrap();
    physician_auth::migration::patient_link::Entity::find()
        .lock_exclusive()
        .one(&held)
        .await
        .unwrap()
        .unwrap();
    let redeem = |account: &'static str| {
        let (store, token) = (store.clone(), token.clone());
        tokio::spawn(async move { store.accept_patient_invitation(&token, account, NOW).await })
    };
    let first = redeem("duet-first");
    let second = redeem("duet-second");
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
    let winner = outcomes.into_iter().find_map(Result::ok).unwrap();
    let persisted = store.patient_link("pg-proof", "pw-elena").await.unwrap();
    assert_eq!(persisted.duet_account_id, winner.duet_account_id);
    assert!(persisted.linked);
    assert!(!persisted.sharing_consented);
    assert_eq!(
        store
            .accept_patient_invitation(&token, "duet-replay", NOW)
            .await,
        Err(SignInError::Invalid)
    );
}

/// Connects when the gate is set; `None` skips the suite quietly.
///
/// Each test gets its own database — named after the gate URL's database plus
/// the test's unique suffix — so parallel tests and repeated runs never share
/// migration state. The next run removes the previous test database.
async fn store(name: &str) -> Option<PostgresAuthStore> {
    let url = fresh_database(name).await?;
    Some(
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("migrations"),
    )
}

async fn fresh_database(name: &str) -> Option<String> {
    let url = std::env::var("PHYSICIAN_DATABASE_URL").unwrap_or_default();
    let url = url.trim();
    if url.is_empty() {
        assert!(
            std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
            "PHYSICIAN_DATABASE_URL is required for the Postgres CI proof"
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
    // The per-test database is intentionally left in place after the test;
    // the next run's `DROP DATABASE IF EXISTS` clears any stale leftover,
    // so a crashed or failed test never wedges the suite.
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
    let Some(url) = fresh_database("migration_separation").await else {
        return;
    };
    assert!(schema_tables(&url).await.is_empty());
    let runtime = PostgresAuthStore::connect_without_migrations(&url)
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
    let _compatibility = PostgresAuthStore::connect(&url)
        .await
        .expect("compatibility connection");
    assert!(
        schema_tables(&url).await.is_empty(),
        "runtime must not install even the migration ledger"
    );

    let migrate = || {
        let status = std::process::Command::new(env!("CARGO_BIN_EXE_physician-auth-migrate"))
            .env("PHYSICIAN_DATABASE_URL", &url)
            .status()
            .expect("migration job");
        assert!(status.success(), "migration job failed");
    };
    migrate();
    let expected = vec![
        "physician_accounts",
        "physician_auth_migrations",
        "physician_invitations",
        "physician_memberships",
        "physician_patient_links",
        "physician_patients",
        "physician_practices",
        "physician_sessions",
        "physician_sign_in_tokens",
    ];
    assert_eq!(schema_tables(&url).await, expected);
    let (email, session) = invited_account(&runtime, "separation-proof").await;
    let who = runtime
        .identity(&session, NOW)
        .await
        .expect("saved session");
    migrate();
    runtime.migrate().await.expect("explicit migration rerun");
    let read_only_url = format!("{url}?options=-c%20default_transaction_read_only%3Don");
    let reopened = PostgresAuthStore::connect_without_migrations(&read_only_url)
        .await
        .expect("reopen runtime with database-enforced read-only transactions");
    reopened
        .check_schema()
        .await
        .expect("read-only schema check");
    assert_eq!(schema_tables(&url).await, expected);
    assert_eq!(reopened.identity(&session, NOW).await.unwrap(), who);
    assert!(reopened.account_by_email(&email).await.is_some());
    assert!(
        reopened.migrate().await.is_err(),
        "read-only runtime cannot migrate"
    );
}

fn identity(practice: &str, physician: &str) -> PhysicianIdentity {
    PhysicianIdentity {
        practice_id: practice.to_owned(),
        physician_id: physician.to_owned(),
    }
}

/// One account per test: a unique email, invited and redeemed, plus the
/// returned session. The unique email keeps concurrent runs independent.
async fn invited_account(store: &PostgresAuthStore, practice: &str) -> (String, String) {
    let email = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());
    let token = store
        .invite(practice, &email, "PG Proof", false, NOW)
        .await
        .expect("invitation");
    let (_who, session) = store
        .redeem_invitation(&email, &token, NOW)
        .await
        .expect("redemption");
    (email, session)
}

#[tokio::test]
async fn membership_denies_by_default_and_round_trips() {
    let Some(store) = store("membership").await else {
        return;
    };
    let (email, _session) = invited_account(&store, "pg-proof").await;
    let account = store.account_by_email(&email).await.expect("account");
    let who = account.identity;
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    assert_eq!(
        store.membership(&who, &workspace).await,
        Membership::Denied,
        "a missing row denies"
    );
    store
        .set_membership(&who, &workspace, Membership::Allowed)
        .await
        .unwrap();
    assert_eq!(
        store.membership(&who, &workspace).await,
        Membership::Allowed
    );
    store
        .set_membership(&who, &workspace, Membership::Denied)
        .await
        .unwrap();
    assert_eq!(
        store.membership(&who, &workspace).await,
        Membership::Denied,
        "the explicit denial replaces the grant"
    );
    assert_eq!(store.membership(&who, "pw-other").await, Membership::Denied);
}

#[tokio::test]
async fn a_sign_in_token_verifies_once_and_mints_one_session() {
    let Some(store) = store("sign_in_token").await else {
        return;
    };
    let (email, _first) = invited_account(&store, "pg-proof").await;
    let token = store.begin_sign_in(&email, NOW).await.expect("token");
    let (who, session) = store
        .complete_sign_in(&email, &token, NOW)
        .await
        .expect("first verify");
    assert!(session.starts_with("codas-"));
    assert_eq!(
        store
            .complete_sign_in(&email, &token, NOW)
            .await
            .unwrap_err(),
        SignInError::Invalid,
        "a verified token never verifies again"
    );
    assert!(
        store
            .identity(&session, NOW + 12 * 60 * 60 - 1)
            .await
            .is_ok(),
        "an unused session still resolves inside 12 hours"
    );
    let resolved = store.identity(&session, NOW + 12 * 60 * 60).await;
    assert!(
        resolved.is_err(),
        "an unused session stops resolving after 12 hours"
    );
    assert_eq!(store.identity(&session, NOW).await.unwrap(), who);
}

#[tokio::test]
async fn a_fixed_session_needs_the_exact_identity_and_ends_however_much_it_is_used() {
    let Some(store) = store("fixed_session").await else {
        return;
    };
    let (email, _session) = invited_account(&store, "pg-proof").await;
    let who = store
        .account_by_email(&email)
        .await
        .expect("account")
        .identity;
    assert_eq!(
        store
            .mint_fixed_session(&identity("pg-proof", "someone-else"), NOW, NOW + 900)
            .await
            .unwrap_err(),
        SignInError::Invalid,
        "no session for an identity without an account"
    );
    assert_eq!(
        store
            .mint_fixed_session(
                &identity("other-practice", &who.physician_id),
                NOW,
                NOW + 900
            )
            .await
            .unwrap_err(),
        SignInError::Invalid,
        "the physician id alone is not enough"
    );
    let session = store
        .mint_fixed_session(&who, NOW, NOW + 900)
        .await
        .expect("fixed session");
    for minute in 0..15 {
        store
            .note_physician_activity(&session, NOW + minute * 60)
            .await
            .expect("activity noted");
    }
    assert_eq!(store.identity(&session, NOW + 899).await.unwrap(), who);
    assert!(
        store.identity(&session, NOW + 900).await.is_err(),
        "activity never carries a fixed session past its end"
    );
}

#[tokio::test]
async fn repeated_identity_calls_do_not_move_expires_at() {
    let Some(store) = store("identity_read").await else {
        return;
    };
    let (_email, session) = invited_account(&store, "pg-proof").await;
    let before = store
        .verified_session(&session, NOW)
        .await
        .unwrap()
        .lifetime
        .expires_at();
    assert_eq!(before, NOW + 12 * 60 * 60);
    store.identity(&session, NOW).await.unwrap();
    store.identity(&session, NOW + 11 * 60 * 60).await.unwrap();
    let after = store
        .verified_session(&session, NOW)
        .await
        .unwrap()
        .lifetime
        .expires_at();
    assert_eq!(after, before);
}

#[tokio::test]
async fn activity_inside_twelve_hours_keeps_the_session() {
    let Some(store) = store("used_within_12h").await else {
        return;
    };
    let (_email, session) = invited_account(&store, "pg-proof").await;
    let used_at = NOW + 11 * 60 * 60;
    assert_eq!(
        store
            .note_physician_activity(&session, used_at)
            .await
            .unwrap(),
        SessionActivity::Extended
    );
    assert!(store.identity(&session, NOW + 12 * 60 * 60).await.is_ok());
    assert!(
        store
            .identity(&session, used_at + 12 * 60 * 60)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn continuous_activity_is_refused_seven_days_after_sign_in() {
    let Some(store) = store("used_for_7d").await else {
        return;
    };
    let (_email, session) = invited_account(&store, "pg-proof").await;
    let mut at = NOW;
    while at < NOW + 7 * 24 * 60 * 60 {
        assert!(store.identity(&session, at).await.is_ok());
        store.note_physician_activity(&session, at).await.unwrap();
        at += 11 * 60 * 60;
    }
    assert!(
        store
            .identity(&session, NOW + 7 * 24 * 60 * 60 - 1)
            .await
            .is_ok()
    );
    assert!(
        store
            .identity(&session, NOW + 7 * 24 * 60 * 60)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn a_burst_of_notes_inside_one_quantum_persists_one_extension() {
    let Some(store) = store("note_quantum").await else {
        return;
    };
    let (_email, session) = invited_account(&store, "pg-proof").await;
    let at = NOW + 15 * 60;
    let mut extensions = 0;
    for _ in 0..5 {
        if store.note_physician_activity(&session, at).await.unwrap() == SessionActivity::Extended {
            extensions += 1;
        }
    }
    assert_eq!(extensions, 1);
    let once = column_expires_at(&store, &session).await;
    assert_eq!(once, (at + 12 * 60 * 60) as i64);
    assert_eq!(
        store
            .note_physician_activity(&session, at + 15 * 60 - 1)
            .await
            .unwrap(),
        SessionActivity::Covered
    );
    assert_eq!(column_expires_at(&store, &session).await, once);
    assert_eq!(
        store
            .note_physician_activity(&session, at + 15 * 60)
            .await
            .unwrap(),
        SessionActivity::Extended
    );
    assert_eq!(
        column_expires_at(&store, &session).await,
        (at + 15 * 60 + 12 * 60 * 60) as i64
    );
}

#[tokio::test]
async fn a_dead_sessions_note_does_not_move_expires_at() {
    let Some(store) = store("dead_note").await else {
        return;
    };
    let (_email, session) = invited_account(&store, "pg-proof").await;
    let deadline = column_expires_at(&store, &session).await;
    assert_eq!(
        store
            .note_physician_activity(&session, deadline as u64)
            .await
            .unwrap(),
        SessionActivity::Ended
    );
    assert!(store.identity(&session, deadline as u64).await.is_err());
    assert_eq!(column_expires_at(&store, &session).await, deadline);
}

#[expect(
    clippy::disallowed_methods,
    reason = "reads the stored column to prove the store left it unchanged"
)]
async fn column_expires_at(store: &PostgresAuthStore, token: &str) -> i64 {
    use sea_orm::sqlx::Row;
    sea_orm::sqlx::query("SELECT expires_at FROM physician_sessions WHERE token_hash = $1")
        .bind(PostgresAuthStore::session_reference(token))
        .fetch_one(store.pool())
        .await
        .expect("session row")
        .get("expires_at")
}

#[tokio::test]
async fn deactivation_fences_held_sessions_and_removal_deletes_them() {
    let Some(store) = store("deactivation").await else {
        return;
    };
    let (email, session) = invited_account(&store, "pg-proof").await;
    assert!(store.identity(&session, NOW).await.is_ok());
    assert!(store.deactivate(&email).await);
    assert_eq!(
        store.identity(&session, NOW).await.unwrap_err(),
        SessionError::Inactive,
        "deactivation ends every held session"
    );
    assert!(store.reactivate(&email).await);
    assert_eq!(
        store.identity(&session, NOW).await.unwrap_err(),
        SessionError::Inactive,
        "sessions ended by deactivation stay ended"
    );
    // Sign in again after reactivation; then removal ends that session too.
    let token = store.begin_sign_in(&email, NOW).await.expect("token");
    let (_, session) = store
        .complete_sign_in(&email, &token, NOW)
        .await
        .expect("verify");
    assert!(store.remove_account(&email).await);
    assert!(
        store.identity(&session, NOW).await.is_err(),
        "removal deletes the sessions, so the token is simply unknown"
    );
    assert!(store.account_by_email(&email).await.is_none());
}

#[tokio::test]
async fn an_invitation_redeems_once_and_a_raced_second_redeem_finds_nothing() {
    let Some(store) = store("invitation").await else {
        return;
    };
    let email = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());
    let token = store
        .invite("pg-proof", &email, "PG Proof", true, NOW)
        .await
        .expect("invitation");
    let (who, session) = store
        .redeem_invitation(&email, &token, NOW)
        .await
        .expect("first redeem");
    assert_eq!(who.practice_id, "pg-proof");
    assert!(store.identity(&session, NOW).await.is_ok());
    let Account { admin, .. } = store.account_by_email(&email).await.expect("account");
    assert!(admin, "the invitation's admin flag lands on the account");
    assert_eq!(
        store
            .redeem_invitation(&email, &token, NOW)
            .await
            .unwrap_err(),
        SignInError::Invalid,
        "an invitation redeems once"
    );
    let _ = identity("pg-proof", "unused");
}

#[tokio::test]
async fn panel_assignment_requires_admin_permission_in_the_same_practice() {
    let Some(store) = store("panel_assign").await else {
        return;
    };
    let admin_email = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());
    let admin_token = store
        .invite("pg-proof", &admin_email, "Clinic Admin", true, NOW)
        .await
        .expect("admin invite");
    let (admin, _) = store
        .redeem_invitation(&admin_email, &admin_token, NOW)
        .await
        .expect("admin redeem");

    let (doc_email, _) = invited_account(&store, "pg-proof").await;
    let doc = store.account_by_email(&doc_email).await.unwrap().identity;

    let (outside_email, _) = invited_account(&store, "other-clinic").await;
    let outside = store
        .account_by_email(&outside_email)
        .await
        .unwrap()
        .identity;

    store
        .assign_panel(&admin, &doc.physician_id, "pw-1", Membership::Allowed)
        .await
        .expect("admin assigns panel");
    assert_eq!(store.panel(&doc).await, vec!["pw-1".to_owned()]);

    assert_eq!(
        store
            .assign_panel(&doc, &doc.physician_id, "pw-2", Membership::Allowed)
            .await
            .unwrap_err(),
        PanelError::Forbidden,
        "a physician without panel-assignment permission cannot assign panels"
    );
    assert_eq!(
        store
            .assign_panel(&admin, &outside.physician_id, "pw-1", Membership::Allowed)
            .await
            .unwrap_err(),
        PanelError::UnknownTarget,
        "cannot assign panels across practices"
    );

    store
        .register_patient("other-clinic", "pw-1")
        .await
        .unwrap();
    assert!(
        store.panel(&doc).await.is_empty(),
        "a workspace registered to another practice leaves the panel"
    );
}

#[tokio::test]
async fn practice_patients_resolve_clinic_demographic_and_persist_cadence_in_postgres() {
    let Some(store) = store("practice_patients_cadence").await else {
        return;
    };
    let admin_email = format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple());
    let admin_token = store
        .invite("pg-proof", &admin_email, "Clinic Admin", true, NOW)
        .await
        .expect("admin invite");
    let (admin, _) = store
        .redeem_invitation(&admin_email, &admin_token, NOW)
        .await
        .expect("admin redeem");

    let (doc_email, _) = invited_account(&store, "pg-proof").await;
    let doc = store.account_by_email(&doc_email).await.unwrap().identity;

    let (outside_email, _) = invited_account(&store, "other-clinic").await;
    let outside = store
        .account_by_email(&outside_email)
        .await
        .unwrap()
        .identity;

    store
        .register_patient("pg-proof", "pw-clinic-1")
        .await
        .unwrap();
    assert_eq!(
        store
            .patient_practice("pw-clinic-1")
            .await
            .unwrap()
            .as_deref(),
        Some("pg-proof")
    );
    assert_eq!(
        store.membership(&doc, "pw-clinic-1").await,
        Membership::ClinicDemographic
    );
    assert_eq!(
        store.membership(&outside, "pw-clinic-1").await,
        Membership::Denied
    );

    store
        .assign_panel(
            &admin,
            &doc.physician_id,
            "pw-clinic-1",
            Membership::Allowed,
        )
        .await
        .expect("assign panel");
    assert_eq!(
        store.membership(&doc, "pw-clinic-1").await,
        Membership::Allowed
    );

    store
        .assign_panel(&admin, &doc.physician_id, "pw-clinic-1", Membership::Denied)
        .await
        .expect("revoke panel");
    assert_eq!(
        store.membership(&doc, "pw-clinic-1").await,
        Membership::ClinicDemographic
    );

    assert_eq!(
        store.patient_cadence("pw-clinic-1").await,
        physician_auth::ReviewCadence::Weekly
    );
    store
        .set_patient_cadence(
            "pw-clinic-1",
            physician_auth::ReviewCadence::WeeklyAndDailyWhenChanged,
        )
        .await;
    assert_eq!(
        store.patient_cadence("pw-clinic-1").await,
        physician_auth::ReviewCadence::WeeklyAndDailyWhenChanged
    );
}

#[tokio::test]
async fn patient_invitation_consent_and_cadence_round_trip() {
    let Some(store) = store("patient_link").await else {
        return;
    };
    let token = store
        .invite_patient("pg-proof", "pw-elena", NOW)
        .await
        .expect("patient invite");
    let linked = store
        .accept_patient_invitation(&token, "duet-elena", NOW + 5)
        .await
        .expect("accept invite");
    assert!(linked.linked);
    assert_eq!(linked.duet_account_id.as_deref(), Some("duet-elena"));

    let consented = store
        .grant_patient_consent("pg-proof", "pw-elena", "duet-elena", "v1", NOW + 10)
        .await
        .expect("grant consent");
    assert!(consented.sharing_consented);
    assert_eq!(consented.consent_version.as_deref(), Some("v1"));

    store
        .set_patient_premium("pg-proof", "pw-elena", true)
        .await;
    assert_eq!(
        store.patient_link_cadence("pg-proof", "pw-elena").await,
        Cadence::DailyWhenChanged
    );

    let revoked = store
        .revoke_patient_consent("pg-proof", "pw-elena", NOW + 20)
        .await
        .expect("revoke consent");
    assert!(!revoked.sharing_consented);
    assert_eq!(
        store
            .grant_patient_consent("pg-proof", "pw-elena", "duet-elena", "v1", NOW + 30)
            .await
            .expect_err("revoked consent is not restored"),
        SignInError::Invalid
    );
    let still = store
        .patient_link("pg-proof", "pw-elena")
        .await
        .expect("link remains");
    assert!(!still.sharing_consented);
    assert_eq!(still.revoked_at, Some(NOW + 20));
}

#[tokio::test]
async fn relinking_a_workspace_does_not_transfer_patient_consent() {
    let Some(store) = store("relink_consent").await else {
        return;
    };
    let invite = store
        .invite_patient("practice", "patient", NOW)
        .await
        .unwrap();
    store
        .accept_patient_invitation(&invite, "account-a", NOW)
        .await
        .unwrap();
    store
        .grant_patient_consent("practice", "patient", "account-a", "v1", NOW)
        .await
        .unwrap();
    let invite = store
        .invite_patient("practice", "patient", NOW + 1)
        .await
        .unwrap();
    store
        .accept_patient_invitation(&invite, "account-b", NOW + 1)
        .await
        .unwrap();
    let link = store.patient_link("practice", "patient").await.unwrap();
    assert_eq!(link.duet_account_id.as_deref(), Some("account-b"));
    assert!(!link.sharing_consented);
    assert_eq!((link.consented_at, link.consent_version), (None, None));
    assert!(
        store
            .grant_patient_consent("practice", "patient", "account-a", "v1", NOW + 2)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn expired_and_replaced_patient_invitations_cannot_link_an_account() {
    let Some(store) = store("stale_patient_invite").await else {
        return;
    };
    let expired = store
        .invite_patient("pg-proof", "pw-elena", NOW)
        .await
        .unwrap();
    let later = NOW + physician_auth::INVITATION_TTL_SECONDS;
    assert_eq!(
        store
            .accept_patient_invitation(&expired, "duet-expired", later)
            .await,
        Err(SignInError::Invalid)
    );
    let replaced = store
        .invite_patient("pg-proof", "pw-elena", later)
        .await
        .unwrap();
    let current = store
        .invite_patient("pg-proof", "pw-elena", later)
        .await
        .unwrap();
    assert_eq!(
        store
            .accept_patient_invitation(&replaced, "duet-replaced", later)
            .await,
        Err(SignInError::Invalid)
    );
    assert_eq!(
        store
            .accept_patient_invitation(&current, "  duet-current  ", later)
            .await
            .unwrap()
            .duet_account_id
            .as_deref(),
        Some("duet-current")
    );
}

#[tokio::test]
async fn ensure_account_seeds_once_and_never_undoes_an_admin_change() {
    let Some(store) = store("ensure_account").await else {
        return;
    };
    let email = "Seeded@Practice.test";
    assert!(
        store
            .ensure_account("pg-proof", "staging-physician", email, "Seeded", true, NOW)
            .await
            .expect("seed")
    );
    let account = store
        .account_by_email("seeded@practice.test")
        .await
        .expect("seeded account");
    assert_eq!(account.identity.physician_id, "staging-physician");
    assert!(account.admin);

    // The seeded account signs in through the ordinary token flow.
    let token = store.begin_sign_in(email, NOW).await.expect("token");
    let (who, _) = store
        .complete_sign_in(email, &token, NOW)
        .await
        .expect("verify");
    assert_eq!(who.practice_id, "pg-proof");

    // A rerun neither duplicates nor reactivates the account.
    assert!(store.deactivate(email).await);
    assert!(
        !store
            .ensure_account(
                "pg-proof",
                "replacement-physician",
                email,
                "Renamed",
                false,
                NOW + 1
            )
            .await
            .expect("rerun")
    );
    let account = store
        .account_by_email(email)
        .await
        .expect("still one account");
    assert!(!account.active, "a rerun leaves the deactivation in place");
    assert!(account.admin, "a rerun leaves the admin flag in place");
    assert_eq!(account.display_name, "Seeded");
    assert_eq!(account.identity.physician_id, "staging-physician");
    assert_eq!(
        store.begin_sign_in(email, NOW + 2).await.unwrap_err(),
        SignInError::Inactive
    );
}

#[tokio::test]
async fn distinct_preview_accounts_resolve_their_own_admin_flag_and_panel() {
    let Some(store) = store("distinct_physicians").await else {
        return;
    };
    let practice = "pg-proof";
    assert!(
        store
            .ensure_account(
                practice,
                "physician-admin",
                "admin@practice.test",
                "Admin",
                true,
                NOW
            )
            .await
            .expect("admin")
    );
    assert!(
        store
            .ensure_account(
                practice,
                "physician-member",
                "member@practice.test",
                "Member",
                false,
                NOW
            )
            .await
            .expect("member")
    );
    store.register_patient(practice, "pw-admin").await.unwrap();
    store.register_patient(practice, "pw-member").await.unwrap();
    let admin = PhysicianIdentity {
        practice_id: practice.to_owned(),
        physician_id: "physician-admin".to_owned(),
    };
    store
        .assign_panel(&admin, "physician-admin", "pw-admin", Membership::Allowed)
        .await
        .expect("admin panel");
    store
        .assign_panel(&admin, "physician-member", "pw-member", Membership::Allowed)
        .await
        .expect("member panel");

    let admin_token = store
        .begin_sign_in("admin@practice.test", NOW)
        .await
        .expect("admin token");
    let (_, admin_session) = store
        .complete_sign_in("admin@practice.test", &admin_token, NOW)
        .await
        .expect("admin session");
    let member_token = store
        .begin_sign_in("member@practice.test", NOW)
        .await
        .expect("member token");
    let (_, member_session) = store
        .complete_sign_in("member@practice.test", &member_token, NOW)
        .await
        .expect("member session");

    let admin_identity = store.identity(&admin_session, NOW).await.expect("admin");
    let member_identity = store.identity(&member_session, NOW).await.expect("member");
    assert_ne!(admin_identity.physician_id, member_identity.physician_id);
    let admin_account = store
        .account_by_identity(&admin_identity)
        .await
        .expect("admin account");
    let member_account = store
        .account_by_identity(&member_identity)
        .await
        .expect("member account");
    assert!(admin_account.admin);
    assert!(!member_account.admin);
    assert_eq!(
        store.panel(&admin_identity).await,
        vec!["pw-admin".to_owned()]
    );
    assert_eq!(
        store.panel(&member_identity).await,
        vec!["pw-member".to_owned()]
    );
}

#[tokio::test]
async fn duplicate_physician_ids_keep_the_earliest_account_and_its_panel() {
    let Some(url) = fresh_database("physician_identity_repair").await else {
        return;
    };
    PostgresAuthStore::connect_with_migrations(&url)
        .await
        .expect("migrate");
    let db = sea_orm::Database::connect(&url)
        .await
        .expect("repair connection");
    db.execute_unprepared("drop index physician_accounts_practice_physician_uidx")
        .await
        .expect("drop identity index");
    db.execute_unprepared(
        "insert into physician_accounts (id, practice_id, physician_id, email, display_name, active, admin, sessions_ended_before, created_at) values \
         ('00000000-0000-0000-0000-000000000001', 'pg', 'shared-doc', 'early@test', 'Early', true, true, 0, 10), \
         ('00000000-0000-0000-0000-000000000002', 'pg', 'shared-doc', 'late@test', 'Late', true, false, 0, 20)",
    )
    .await
    .expect("duplicate identities");
    db.execute_unprepared(
        "insert into physician_memberships (practice_id, physician_id, patient_workspace_id, allowed, created_at) values \
         ('pg', 'shared-doc', 'pw-1', true, 10)",
    )
    .await
    .expect("surviving panel");
    db.execute_unprepared(physician_auth::migration::REASSIGN_DUPLICATE_PHYSICIAN_IDS)
        .await
        .expect("reassign");
    let early: String = db
        .query_one_raw(sea_orm::Statement::from_string(
            sea_orm::DbBackend::Postgres,
            "select physician_id from physician_accounts where email='early@test'".to_owned(),
        ))
        .await
        .expect("early query")
        .expect("early row")
        .try_get("", "physician_id")
        .expect("early id");
    let late: String = db
        .query_one_raw(sea_orm::Statement::from_string(
            sea_orm::DbBackend::Postgres,
            "select physician_id from physician_accounts where email='late@test'".to_owned(),
        ))
        .await
        .expect("late query")
        .expect("late row")
        .try_get("", "physician_id")
        .expect("late id");
    assert_eq!(early, "shared-doc");
    assert_ne!(late, "shared-doc");
    assert!(late.starts_with("migrated-"));
    let surviving: i64 = db
        .query_one_raw(sea_orm::Statement::from_string(
            sea_orm::DbBackend::Postgres,
            "select count(*) as n from physician_memberships where physician_id='shared-doc' and patient_workspace_id='pw-1' and allowed".to_owned(),
        ))
        .await
        .expect("panel query")
        .expect("panel row")
        .try_get("", "n")
        .expect("panel count");
    let reassigned: i64 = db
        .query_one_raw(sea_orm::Statement::from_string(
            sea_orm::DbBackend::Postgres,
            format!("select count(*) as n from physician_memberships where physician_id='{late}'"),
        ))
        .await
        .expect("reassigned query")
        .expect("reassigned row")
        .try_get("", "n")
        .expect("reassigned count");
    assert_eq!(surviving, 1);
    assert_eq!(reassigned, 0);
}

/// Two replicas share one database: a read cached on one must not survive a
/// write the other makes. The authority trigger's notification moves the
/// first replica's feed, and the next read goes back to Postgres.
#[tokio::test]
async fn a_cached_read_drops_when_another_replica_writes() {
    let Some(url) = fresh_database("authority_feed").await else {
        return;
    };
    let writer = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .expect("migrations");
    let (email, session) = invited_account(&writer, "pg-feed").await;
    let reader = physician_auth::CachedAuthStore::postgres(
        PostgresAuthStore::connect_without_migrations(&url)
            .await
            .expect("runtime pool"),
    );
    let mut feed = reader.authority_changes().expect("feed");
    tokio::time::timeout(
        std::time::Duration::from_secs(10),
        feed.wait_for(|feed| feed.live),
    )
    .await
    .expect("listener connects")
    .expect("feed open");
    let who = reader
        .identity(&session, NOW)
        .await
        .expect("session resolves");
    assert!(reader.membership(&who, "ws-feed").await == Membership::Denied);

    let seen = feed.borrow_and_update().generation;
    writer
        .set_membership(&who, "ws-feed", Membership::Allowed)
        .await
        .unwrap();
    tokio::time::timeout(
        std::time::Duration::from_secs(10),
        feed.wait_for(|feed| feed.generation > seen),
    )
    .await
    .expect("trigger notifies")
    .expect("feed open");
    assert_eq!(
        reader.membership(&who, "ws-feed").await,
        Membership::Allowed
    );

    let seen = feed.borrow_and_update().generation;
    assert!(writer.deactivate(&email).await);
    tokio::time::timeout(
        std::time::Duration::from_secs(10),
        feed.wait_for(|feed| feed.generation > seen),
    )
    .await
    .expect("trigger notifies")
    .expect("feed open");
    assert_eq!(
        reader.identity(&session, NOW).await,
        Err(SessionError::Inactive)
    );
}

#[tokio::test]
async fn failed_revocations_do_not_acknowledge_or_change_access() {
    let Some(url) = fresh_database("failed_revocations").await else {
        return;
    };
    let store = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let email = "admin@example.test";
    let token = store
        .invite("practice", email, "Admin", true, NOW)
        .await
        .unwrap();
    let (actor, session) = store.redeem_invitation(email, &token, NOW).await.unwrap();
    store.register_patient("practice", "patient").await.unwrap();
    store
        .assign_panel(&actor, &actor.physician_id, "patient", Membership::Allowed)
        .await
        .unwrap();

    let db = sea_orm::Database::connect(&url).await.unwrap();
    db.execute_unprepared("CREATE FUNCTION fail_write() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'simulated storage failure'; END $$;
        CREATE TRIGGER refuse_logout BEFORE DELETE ON physician_sessions FOR EACH ROW EXECUTE FUNCTION fail_write();
        CREATE TRIGGER refuse_removal BEFORE UPDATE ON physician_memberships FOR EACH ROW EXECUTE FUNCTION fail_write();").await.unwrap();
    assert!(store.end_session(&session).await.is_err());
    assert_eq!(
        store
            .assign_panel(&actor, &actor.physician_id, "patient", Membership::Denied)
            .await,
        Err(PanelError::Unavailable)
    );
    assert_eq!(store.identity(&session, NOW).await.unwrap(), actor);
    assert_eq!(
        store.membership(&actor, "patient").await,
        Membership::Allowed
    );
}
