//! Direct-store race proof, without an HTTP mutex or a copied authority ledger.
use physician_auth::{AuthStore, Membership, MemoryAuthStore, PostgresAuthStore};
use std::sync::Arc;

async fn proof(store: Arc<dyn AuthStore>, revoker_store: Arc<dyn AuthStore>) {
    let unique = uuid::Uuid::new_v4().to_string();
    let email = format!("{unique}@example.test");
    let workspace = format!("workspace-{unique}");
    let mut now = 1_800_000_000;
    let invite = store
        .invite(&unique, &email, "Synthetic Physician", true, now)
        .await
        .unwrap();
    let (identity, mut token) = store.redeem_invitation(&email, &invite, now).await.unwrap();
    store.register_patient(&unique, &workspace).await;
    store
        .set_membership(&identity, &workspace, Membership::Allowed)
        .await;
    for mutation in ["membership", "account", "session"] {
        let (entered_tx, entered_rx) = tokio::sync::oneshot::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let s = store.clone();
        let t = token.clone();
        let i = identity.clone();
        let w = workspace.clone();
        let publisher = tokio::spawn(async move {
            let mut entered_tx = Some(entered_tx);
            s.publish_authorized(&t, &i, &w, now, &mut move || {
                entered_tx.take().unwrap().send(()).unwrap();
                release_rx
                    .recv_timeout(std::time::Duration::from_secs(5))
                    .unwrap();
                Ok(())
            })
            .await
            .unwrap();
        });
        entered_rx.await.unwrap();
        let s = revoker_store.clone();
        let t = token.clone();
        let i = identity.clone();
        let w = workspace.clone();
        let email_copy = email.clone();
        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
        let mut revoker = tokio::spawn(async move {
            started_tx.send(()).unwrap();
            match mutation {
                "membership" => s.set_membership(&i, &w, Membership::Denied).await,
                "account" => {
                    assert!(s.deactivate(&email_copy).await);
                }
                _ => s.end_session(&t).await,
            }
        });
        started_rx.await.unwrap();
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoker)
                .await
                .is_err(),
            "{mutation} must wait for publication"
        );
        release_tx.send(()).unwrap();
        publisher.await.unwrap();
        revoker.await.unwrap();
        assert!(
            store
                .publish_authorized(&token, &identity, &workspace, now, &mut || panic!(
                    "revoked publication must not run"
                ))
                .await
                .is_err()
        );
        match mutation {
            "membership" => {
                store
                    .set_membership(&identity, &workspace, Membership::Allowed)
                    .await
            }
            "account" => {
                assert!(store.reactivate(&email).await);
                now += 1;
                let pending = store.begin_sign_in(&email, now).await.unwrap();
                token = store
                    .complete_sign_in(&email, &pending, now)
                    .await
                    .unwrap()
                    .1;
            }
            _ => (),
        }
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn postgres_direct_mutations_obey_publication_fence_across_connections() {
    let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL").expect("loopback PostgreSQL required");
    proof(
        Arc::new(
            PostgresAuthStore::connect_with_migrations(&url)
                .await
                .unwrap(),
        ),
        Arc::new(PostgresAuthStore::connect(&url).await.unwrap()),
    )
    .await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn memory_direct_mutations_obey_publication_fence() {
    let store = Arc::new(MemoryAuthStore::new());
    proof(store.clone(), store).await;
}
