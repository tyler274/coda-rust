//! Real PostgreSQL authority race proof; missing prerequisites fail, never skip.
use super::*;
use physician_auth::migration::{Accounts, Memberships, Patients, account, membership, patient};
use physician_auth::{AuthStore, Membership, PostgresAuthStore};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};
use std::sync::Arc;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn postgres_revocation_waits_for_existing_authority_row_publication_fence() {
    let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL")
        .expect("PHYSICIAN_TEST_DATABASE_URL required for authority race proof");
    eprintln!("authority proof: connecting store");
    let store = Arc::new(
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .unwrap(),
    );
    let db = Database::connect(&url).await.unwrap();
    let unique = uuid::Uuid::new_v4().to_string();
    let practice = format!("practice-{unique}");
    let email = format!("{unique}@example.test");
    let workspace = format!("workspace-{unique}");
    eprintln!("authority proof: migrated, inviting");
    let invite = store
        .invite(&practice, &email, "Synthetic Physician", true, auth::now())
        .await
        .unwrap();
    eprintln!("authority proof: invited, redeeming");
    let (identity, token) = store
        .redeem_invitation(&email, &invite, auth::now())
        .await
        .unwrap();
    eprintln!("authority proof: redeemed, registering");
    store.register_patient(&practice, &workspace).await;
    eprintln!("authority proof: registered");
    store
        .set_membership(&identity, &workspace, Membership::Allowed)
        .await;
    let auth = auth::Auth::on(
        store.clone(),
        Arc::new(front_door::MemoryFrontDoorStore::new()),
    )
    .await;
    let ctx = auth::AuthContext {
        auth,
        token: token.clone(),
        identity,
    };
    let (entered_tx, entered_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let publish_ctx = ctx.clone();
    let publish_workspace = workspace.clone();
    let publisher = tokio::spawn(async move {
        let mut entered_tx = Some(entered_tx);
        publication::publish(&publish_ctx, &publish_workspace, &mut move || {
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
    let other = store.clone();
    let token_copy = token.clone();
    let mut revoker = tokio::spawn(async move { other.end_session(&token_copy).await });
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoker)
            .await
            .is_err(),
        "revocation must wait while actual authority rows are locked"
    );
    release_tx.send(()).unwrap();
    publisher.await.unwrap();
    revoker.await.unwrap();
    assert!(
        publication::publish(&ctx, &workspace, &mut || panic!(
            "revoked callback must not run"
        ))
        .await
        .is_err(),
        "new publication must fail after revocation"
    );
    Memberships::delete_many()
        .filter(membership::Column::PracticeId.eq(&practice))
        .exec(&db)
        .await
        .unwrap();
    Patients::delete_many()
        .filter(patient::Column::PracticeId.eq(&practice))
        .exec(&db)
        .await
        .unwrap();
    Accounts::delete_many()
        .filter(account::Column::PracticeId.eq(&practice))
        .exec(&db)
        .await
        .unwrap();
}
