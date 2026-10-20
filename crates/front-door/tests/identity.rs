//! Durable identity and consent proof on disposable Postgres; no provider calls.
use front_door::FrontDoorStore;
use front_door::{
    PostgresFrontDoorStore,
    identity::{AuthorityError, LUNA_VERSION, Principal, TERMS_VERSION},
};
const NOW: i64 = 1_800_000_000;
fn principal(subject: &str, email: &str) -> Principal {
    Principal {
        project: "duet-test".into(),
        subject: subject.into(),
        email: email.into(),
        authenticated_at: NOW - 10,
    }
}

#[tokio::test]
async fn durable_identity_revocation_permissions_and_account_isolation() {
    let url = match std::env::var("DUET_IDENTITY_TEST_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            assert!(
                std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                "DUET_IDENTITY_TEST_DATABASE_URL is required"
            );
            return;
        }
    };
    // Caller supplies a dedicated disposable database. Runtime connect never migrates.
    let store = PostgresFrontDoorStore::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    store.migrate().await.unwrap();
    let suffix = uuid::Uuid::new_v4();
    let alice = principal(
        &format!("alice-{suffix}"),
        &format!("alice-{suffix}@example.test"),
    );
    let bob = principal(
        &format!("bob-{suffix}"),
        &format!("bob-{suffix}@example.test"),
    );
    assert!(matches!(
        store.identity_access(&alice).await,
        Err(AuthorityError::Absent)
    ));
    let a = store.provision_identity(&alice, NOW).await.unwrap();
    let b = store.provision_identity(&bob, NOW).await.unwrap();
    assert_ne!(a.account_id, b.account_id);
    assert!(
        store.begin_sign_in(&alice.email, NOW as u64).await.is_err(),
        "Identity Platform accounts cannot use legacy email proof bypass"
    );
    assert_eq!(
        store
            .provision_identity(&alice, NOW)
            .await
            .unwrap()
            .account_id,
        a.account_id
    );
    assert!(!a.allows_luna());
    let racer = principal(
        &format!("race-{suffix}"),
        &format!("race-{suffix}@example.test"),
    );
    let (left, right) = tokio::join!(
        store.provision_identity(&racer, NOW),
        store.provision_identity(&racer, NOW)
    );
    assert!(left.is_ok() || right.is_ok());
    let canonical = store.identity_access(&racer).await.unwrap().account_id;
    for result in [left, right].into_iter().flatten() {
        assert_eq!(result.account_id, canonical);
    }
    assert_eq!(
        store
            .provision_identity(&racer, NOW)
            .await
            .unwrap()
            .account_id,
        canonical
    );
    let mut email_collision = bob.clone();
    email_collision.subject = format!("intruder-{suffix}");
    email_collision.email = alice.email.clone();
    assert!(
        store
            .provision_identity(&email_collision, NOW)
            .await
            .is_err()
    );
    assert!(matches!(
        store.identity_access(&email_collision).await,
        Err(AuthorityError::Absent)
    ));
    let mut foreign = alice.clone();
    foreign.project = "foreign".into();
    assert!(matches!(
        store.identity_access(&foreign).await,
        Err(AuthorityError::Absent)
    ));
    assert!(
        store
            .accept_identity_terms(&alice, "old", NOW)
            .await
            .is_err()
    );
    let terms = store
        .accept_identity_terms(&alice, TERMS_VERSION, NOW)
        .await
        .unwrap();
    assert_eq!(
        store
            .accept_identity_terms(&alice, TERMS_VERSION, NOW + 1)
            .await
            .unwrap()
            .permissions
            .terms_at,
        terms.permissions.terms_at
    );
    assert!(
        store
            .set_identity_luna(&alice, LUNA_VERSION, true, NOW)
            .await
            .unwrap()
            .allows_luna()
    );
    assert!(!store.identity_access(&bob).await.unwrap().allows_luna());
    drop(store);
    let store = PostgresFrontDoorStore::connect(&url).await.unwrap();
    assert_eq!(
        store.identity_access(&alice).await.unwrap().account_id,
        a.account_id
    );
    assert!(store.identity_access(&alice).await.unwrap().allows_luna());
    let withdrawal = store
        .set_identity_luna(&alice, LUNA_VERSION, false, NOW + 1)
        .await
        .unwrap();
    assert!(!withdrawal.allows_luna());
    assert!(withdrawal.permissions.restriction_pending);
    store
        .revoke_identity_sessions(&alice, NOW + 2)
        .await
        .unwrap();
    drop(store);
    let store = PostgresFrontDoorStore::connect(&url).await.unwrap();
    assert!(matches!(
        store.identity_access(&alice).await,
        Err(AuthorityError::Refused)
    ));
    assert!(
        store
            .accept_identity_terms(&alice, TERMS_VERSION, NOW + 3)
            .await
            .is_err()
    );
    let mut fresh = alice.clone();
    fresh.authenticated_at = NOW + 2;
    assert!(
        store.identity_access(&fresh).await.is_err(),
        "same-second sign-in is revoked"
    );
    fresh.authenticated_at = NOW + 3;
    let after = store.identity_access(&fresh).await.unwrap();
    assert_eq!(after.account_id, a.account_id);
    assert!(!after.allows_luna());
    assert!(matches!(
        store
            .set_identity_luna(&fresh, LUNA_VERSION, true, NOW + 3)
            .await,
        Err(AuthorityError::RestrictionPending)
    ));
    assert_eq!(
        store.identity_access(&bob).await.unwrap().account_id,
        b.account_id
    );
}
