//! The Postgres store against a real server, gated on `PHYSICIAN_DATABASE_URL`.
//!
//! The devcontainer compose service provides that URL; without it the store's
//! contract is covered by the memory-store unit tests, and these stay quiet.

use physician_auth::{
    Account, AuthStore, Cadence, Membership, PanelError, PhysicianIdentity, PostgresAuthStore,
    SESSION_TTL_SECONDS, SessionError, SignInError,
};
use sea_orm::ConnectionTrait;

const NOW: u64 = 1_760_000_000;

#[tokio::test]
async fn npi_association_round_trips_conflicts_and_clears() {
    let Some(store) = store("npi").await else {
        return;
    };
    let (first_email, _) = invited_account(&store, "first").await;
    let (second_email, _) = invited_account(&store, "second").await;
    let first = store.account_by_email(&first_email).await.unwrap();
    let second = store.account_by_email(&second_email).await.unwrap();
    let mut clinician = npi_registry::FixtureDirectory::sample()
        .clinicians()
        .remove(0);
    clinician.person.as_mut().unwrap().last = "Proof".into();
    let claim = store
        .claim_npi(&first.identity, clinician.clone(), NOW, false)
        .await
        .unwrap();
    assert_eq!(
        store
            .account_by_identity(&first.identity)
            .await
            .unwrap()
            .npi,
        Some(claim)
    );
    assert_eq!(
        store
            .claim_npi(&second.identity, clinician.clone(), NOW, false)
            .await,
        Err(physician_auth::NpiAssociationError::Conflict)
    );
    assert!(
        !store
            .account_by_identity(&first.identity)
            .await
            .unwrap()
            .admin
    );
    store.clear_npi(&second.identity).await.unwrap();
    assert!(
        store
            .account_by_identity(&first.identity)
            .await
            .unwrap()
            .npi
            .is_some()
    );
    store.clear_npi(&first.identity).await.unwrap();
    assert!(
        store
            .account_by_identity(&first.identity)
            .await
            .unwrap()
            .npi
            .is_none()
    );
    store
        .claim_npi(&second.identity, clinician, NOW, false)
        .await
        .unwrap();
    store.migrate().await.unwrap();
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
        "physician_sessions",
        "physician_sign_in_tokens",
    ];
    assert_eq!(schema_tables(&url).await, expected);
    let (email, session) = invited_account(&runtime, "separation-proof").await;
    let who = runtime.session(&session, NOW).await.expect("saved session");
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
    assert_eq!(reopened.session(&session, NOW).await.unwrap(), who);
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
        .await;
    assert_eq!(
        store.membership(&who, &workspace).await,
        Membership::Allowed
    );
    store
        .set_membership(&who, &workspace, Membership::Denied)
        .await;
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
    let resolved = store.session(&session, NOW + SESSION_TTL_SECONDS).await;
    assert!(resolved.is_err(), "an expired session stops resolving");
    assert_eq!(store.session(&session, NOW).await.unwrap(), who);
}

#[tokio::test]
async fn deactivation_fences_held_sessions_and_removal_deletes_them() {
    let Some(store) = store("deactivation").await else {
        return;
    };
    let (email, session) = invited_account(&store, "pg-proof").await;
    assert!(store.session(&session, NOW).await.is_ok());
    assert!(store.deactivate(&email).await);
    assert_eq!(
        store.session(&session, NOW).await.unwrap_err(),
        SessionError::Inactive,
        "deactivation ends every held session"
    );
    assert!(store.reactivate(&email).await);
    assert_eq!(
        store.session(&session, NOW).await.unwrap_err(),
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
        store.session(&session, NOW).await.is_err(),
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
    assert!(store.session(&session, NOW).await.is_ok());
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

    store.register_patient("pg-proof", "pw-clinic-1").await;
    assert_eq!(
        store.patient_practice("pw-clinic-1").await.as_deref(),
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
}
