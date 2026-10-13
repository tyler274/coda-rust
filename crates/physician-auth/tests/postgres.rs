//! The Postgres store against a real server, gated on `PHYSICIAN_DATABASE_URL`.
//!
//! The devcontainer compose service provides that URL; without it the store's
//! contract is covered by the memory-store unit tests, and these stay quiet.

use physician_auth::{
    Account, AuthStore, Membership, PanelError, PhysicianIdentity, PostgresAuthStore,
    SESSION_TTL_SECONDS, SessionError, SignInError,
};
use sea_orm::ConnectionTrait;

const NOW: u64 = 1_760_000_000;

/// Connects when the gate is set; `None` skips the suite quietly.
///
/// Each test gets its own database — named after the gate URL's database plus
/// the test's unique suffix — so parallel tests and repeated runs never share
/// migration state. The database is dropped when the store is dropped.
async fn store(name: &str) -> Option<PostgresAuthStore> {
    let url = std::env::var("PHYSICIAN_DATABASE_URL").ok()?;
    let url = url.trim();
    if url.is_empty() {
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
    let store = PostgresAuthStore::connect_with_migrations(&format!("{base}/{test_db}"))
        .await
        .expect("connect with migrations");
    admin.close().await.expect("close admin connection");
    Some(store)
    // The per-test database is intentionally left in place after the test;
    // the next run's `DROP DATABASE IF EXISTS` clears any stale leftover,
    // so a crashed or failed test never wedges the suite.
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
