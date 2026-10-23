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
/// SHARE locks must fence direct writes even without an HTTP-layer mutex.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn runtime_authority_locks_all_four_owners_and_refuses_revoked_access() {
    use physician_auth::migration::{
        Accounts, Memberships, Patients, Sessions, account, membership, patient, session,
    };
    use sea_orm::{
        ColumnTrait, Database, EntityTrait, QueryFilter, TransactionTrait, sea_query::Expr,
    };

    let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL").expect("loopback PostgreSQL required");
    let store = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let db = Database::connect(&url).await.unwrap();
    for mutation in ["membership", "account", "session", "patient", "fence"] {
        let unique = uuid::Uuid::new_v4().to_string();
        let email = format!("{unique}@example.test");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let invite = store
            .invite(&unique, &email, "Synthetic Physician", true, now)
            .await
            .unwrap();
        let (identity, token) = store.redeem_invitation(&email, &invite, now).await.unwrap();
        store.register_patient(&unique, &unique).await;
        store
            .set_membership(&identity, &unique, Membership::Allowed)
            .await;
        let hash = PostgresAuthStore::session_reference(&token);
        let authority = db.begin().await.unwrap();
        PostgresAuthStore::lock_runtime_authority(&authority, &hash, &identity, &unique)
            .await
            .unwrap();
        // Multiple publishers may share this fence, unlike exclusive publication.
        let reader = db.begin().await.unwrap();
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            PostgresAuthStore::lock_runtime_authority(&reader, &hash, &identity, &unique),
        )
        .await
        .unwrap()
        .unwrap();
        reader.commit().await.unwrap();
        let writer_db = db.clone();
        let writer_hash = hash.clone();
        let writer_unique = unique.clone();
        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
        let mut writer = tokio::spawn(async move {
            started_tx.send(()).unwrap();
            match mutation {
                "membership" => {
                    Memberships::update_many()
                        .col_expr(membership::Column::Allowed, Expr::value(false))
                        .filter(membership::Column::PatientWorkspaceId.eq(&writer_unique))
                        .exec(&writer_db)
                        .await
                        .unwrap();
                }
                "account" => {
                    Accounts::update_many()
                        .col_expr(account::Column::Active, Expr::value(false))
                        .filter(account::Column::Email.eq(&email))
                        .exec(&writer_db)
                        .await
                        .unwrap();
                }
                "session" => {
                    Sessions::delete_many()
                        .filter(session::Column::TokenHash.eq(writer_hash))
                        .exec(&writer_db)
                        .await
                        .unwrap();
                }
                "patient" => {
                    Patients::update_many()
                        .col_expr(patient::Column::PracticeId, Expr::value("another-practice"))
                        .filter(patient::Column::PatientWorkspaceId.eq(writer_unique))
                        .exec(&writer_db)
                        .await
                        .unwrap();
                }
                "fence" => {
                    Accounts::update_many()
                        .col_expr(
                            account::Column::SessionsEndedBefore,
                            Expr::value((now + 1) as i64),
                        )
                        .filter(account::Column::Email.eq(&email))
                        .exec(&writer_db)
                        .await
                        .unwrap();
                }
                _ => unreachable!(),
            }
        });
        started_rx.await.unwrap();
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(100), &mut writer)
                .await
                .is_err(),
            "{mutation} must wait for the runtime authority transaction"
        );
        authority.commit().await.unwrap();
        writer.await.unwrap();
        let refused = db.begin().await.unwrap();
        assert!(
            PostgresAuthStore::lock_runtime_authority(&refused, &hash, &identity, &unique)
                .await
                .is_err(),
            "{mutation} must revoke authority"
        );
        refused.rollback().await.unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn runtime_authority_reads_wall_clock_after_waiting_for_locks() {
    use physician_auth::migration::{Sessions, session};
    use sea_orm::{
        ColumnTrait, Database, EntityTrait, QueryFilter, TransactionTrait, sea_query::Expr,
    };

    let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL").expect("loopback PostgreSQL required");
    let store = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let db = Database::connect(&url).await.unwrap();
    let unique = uuid::Uuid::new_v4().to_string();
    let email = format!("{unique}@example.test");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let invite = store
        .invite(&unique, &email, "Synthetic Physician", true, now)
        .await
        .unwrap();
    let (identity, token) = store.redeem_invitation(&email, &invite, now).await.unwrap();
    store.register_patient(&unique, &unique).await;
    store
        .set_membership(&identity, &unique, Membership::Allowed)
        .await;
    let hash = PostgresAuthStore::session_reference(&token);
    let blocker = db.begin().await.unwrap();
    Sessions::update_many()
        .col_expr(session::Column::ExpiresAt, Expr::value((now + 2) as i64))
        .filter(session::Column::TokenHash.eq(hash.clone()))
        .exec(&blocker)
        .await
        .unwrap();
    let mut waiter = tokio::spawn(async move {
        let tx = db.begin().await.unwrap();
        let result =
            PostgresAuthStore::lock_runtime_authority(&tx, &hash, &identity, &unique).await;
        tx.rollback().await.unwrap();
        result
    });
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(2200), &mut waiter)
            .await
            .is_err()
    );
    blocker.commit().await.unwrap();
    assert_eq!(
        waiter.await.unwrap().unwrap_err().to_string(),
        "Custom Error: runtime session expired"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn memory_direct_mutations_obey_publication_fence() {
    let store = Arc::new(MemoryAuthStore::new());
    proof(store.clone(), store).await;
}
