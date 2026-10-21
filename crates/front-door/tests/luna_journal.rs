//! Mandatory real-SQL owner proof; archive is a controlled fixture, not GCS evidence.
use async_trait::async_trait;
use front_door::{
    PostgresFrontDoorStore,
    identity::{AuthorityError, LUNA_VERSION, Principal, TERMS_VERSION},
    journal::{ArchiveUnavailable, LunaReceipt, ReceiptArchive},
    migration::{permission, restriction},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, Database, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use std::{
    collections::HashMap,
    sync::{
        Mutex,
        atomic::{AtomicU8, Ordering},
    },
};
use uuid::Uuid;
const NOW: i64 = 1_800_000_000;
#[derive(Default)]
struct Archive {
    objects: Mutex<HashMap<Uuid, Vec<u8>>>,
    mode: AtomicU8,
}
#[async_trait]
impl ReceiptArchive for Archive {
    async fn create(&self, key: Uuid, bytes: &[u8]) -> Result<(), ArchiveUnavailable> {
        match self.mode.load(Ordering::SeqCst) {
            1 => return Err(ArchiveUnavailable),
            3 => std::future::pending::<()>().await,
            _ => (),
        }
        self.objects
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with(|| bytes.to_vec());
        Err(ArchiveUnavailable) // Simulate lost upload acknowledgment even on success.
    }
    async fn read(&self, key: Uuid) -> Result<Vec<u8>, ArchiveUnavailable> {
        if self.mode.load(Ordering::SeqCst) == 2 {
            return Ok(b"corrupted".to_vec());
        }
        self.objects
            .lock()
            .unwrap()
            .get(&key)
            .cloned()
            .ok_or(ArchiveUnavailable)
    }
}
fn principal() -> Principal {
    let id = Uuid::new_v4();
    Principal {
        project: "fixture".into(),
        subject: id.to_string(),
        email: format!("{id}@example.test"),
        authenticated_at: NOW - 1,
    }
}
#[tokio::test]
async fn transactional_withdrawal_delivery_restart_replay_and_isolation() {
    let url = match std::env::var("DUET_IDENTITY_TEST_DATABASE_URL") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            assert!(
                std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                "DUET_IDENTITY_TEST_DATABASE_URL is required"
            );
            return;
        }
    };
    let base = url.rsplit_once('/').expect("disposable URL").0;
    let admin = Database::connect(format!("{base}/postgres")).await.unwrap();
    let name = format!("duet_recovery_{}", Uuid::new_v4().simple());
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await
        .unwrap();
    let url = format!("{base}/{name}");
    let store = PostgresFrontDoorStore::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    store.migrate().await.unwrap();
    let db = Database::connect(&url).await.unwrap();
    let alice = principal();
    let bob = principal();
    let a = store
        .provision_identity(&alice, NOW)
        .await
        .unwrap()
        .account_id;
    let b = store
        .provision_identity(&bob, NOW)
        .await
        .unwrap()
        .account_id;
    for p in [&alice, &bob] {
        store
            .accept_identity_terms(p, TERMS_VERSION, NOW)
            .await
            .unwrap();
        store
            .set_identity_luna(p, LUNA_VERSION, true, NOW)
            .await
            .unwrap();
    }
    // Force the outbox insert to fail: permission mutation MUST roll back too.
    db.execute_unprepared(&format!("ALTER TABLE front_door_luna_restrictions ADD CONSTRAINT fixture_reject CHECK (account_id <> '{a}')")).await.unwrap();
    assert!(
        store
            .set_identity_luna(&alice, LUNA_VERSION, false, NOW)
            .await
            .is_err()
    );
    assert!(store.identity_access(&alice).await.unwrap().allows_luna());
    db.execute_unprepared(
        "ALTER TABLE front_door_luna_restrictions DROP CONSTRAINT fixture_reject",
    )
    .await
    .unwrap();
    let withdrawn = store
        .set_identity_luna(&alice, LUNA_VERSION, false, NOW)
        .await
        .unwrap();
    let again = store
        .set_identity_luna(&alice, LUNA_VERSION, false, NOW + 1)
        .await
        .unwrap();
    assert_eq!(withdrawn.permissions, again.permissions);
    let rows = restriction::Entity::find()
        .filter(restriction::Column::AccountId.eq(a))
        .all(&db)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    let event = rows[0].id;
    let archive = Archive::default();
    archive.mode.store(1, Ordering::SeqCst);
    assert!(matches!(
        store.confirm_luna_restriction(&alice, &archive, NOW).await,
        Err(AuthorityError::RestrictionPending)
    ));
    assert!(store.identity_access(&bob).await.unwrap().allows_luna());
    assert!(
        store
            .set_identity_luna(&alice, LUNA_VERSION, true, NOW)
            .await
            .is_err()
    );
    drop(store);
    let store = PostgresFrontDoorStore::connect(&url).await.unwrap();
    archive.mode.store(2, Ordering::SeqCst);
    assert!(
        store
            .confirm_luna_restriction(&alice, &archive, NOW + 31)
            .await
            .is_err()
    );
    assert!(
        !restriction::Entity::find_by_id(event)
            .one(&db)
            .await
            .unwrap()
            .unwrap()
            .confirmed
    );
    archive.mode.store(0, Ordering::SeqCst);
    store
        .confirm_luna_restriction(&alice, &archive, NOW + 100)
        .await
        .unwrap();
    assert_eq!(archive.objects.lock().unwrap().len(), 1);
    assert!(
        restriction::Entity::find_by_id(event)
            .one(&db)
            .await
            .unwrap()
            .unwrap()
            .confirmed
    );
    assert!(!store.identity_access(&alice).await.unwrap().allows_luna());
    assert!(
        store
            .set_identity_luna(&alice, LUNA_VERSION, true, NOW + 101)
            .await
            .unwrap()
            .allows_luna()
    );
    // Cancellation retains the same event and lease; another instance cannot
    // redeliver it until expiry. Unrelated ordinary work remains usable.
    store
        .set_identity_luna(&alice, LUNA_VERSION, false, NOW + 102)
        .await
        .unwrap();
    archive.mode.store(3, Ordering::SeqCst);
    assert!(
        tokio::time::timeout(
            std::time::Duration::from_millis(100),
            store.confirm_luna_restriction(&alice, &archive, NOW + 103)
        )
        .await
        .is_err()
    );
    archive.mode.store(0, Ordering::SeqCst);
    assert!(
        store
            .confirm_luna_restriction(&alice, &archive, NOW + 104)
            .await
            .is_err()
    );
    store
        .confirm_luna_restriction(&bob, &archive, NOW + 104)
        .await
        .unwrap();
    store
        .confirm_luna_restriction(&alice, &archive, NOW + 404)
        .await
        .unwrap();
    assert_eq!(archive.objects.lock().unwrap().len(), 2);
    store
        .set_identity_luna(&alice, LUNA_VERSION, true, NOW + 405)
        .await
        .unwrap();
    // Restored newer positive grant is still fenced by an independent receipt.
    let bytes = archive.objects.lock().unwrap()[&event].clone();
    let receipt = LunaReceipt::parse(&bytes).unwrap();
    let ordinary = PostgresFrontDoorStore::connect(&format!("{base}/postgres"))
        .await
        .unwrap();
    assert!(
        matches!(
            ordinary.replay_luna_restriction(&receipt).await,
            Err(AuthorityError::Refused)
        ),
        "replay refuses ordinary database names"
    );
    drop(ordinary);
    store.replay_luna_restriction(&receipt).await.unwrap();
    let after = store.identity_access(&alice).await.unwrap().permissions;
    store.replay_luna_restriction(&receipt).await.unwrap();
    assert_eq!(
        after,
        store.identity_access(&alice).await.unwrap().permissions
    );
    assert!(!store.identity_access(&alice).await.unwrap().allows_luna());
    assert!(store.identity_access(&bob).await.unwrap().allows_luna());
    assert!(
        store
            .set_identity_luna(&alice, LUNA_VERSION, true, NOW + 406)
            .await
            .unwrap()
            .allows_luna(),
        "replay requires fresh consent, not a permanent lockout"
    );
    // Delete/recreate the same external identity: retained history must never
    // revoke a distinct account incarnation, even with the identical email/UID.
    front_door::migration::account::Entity::delete_by_id(a)
        .exec(&db)
        .await
        .unwrap();
    let recreated = store.provision_identity(&alice, NOW + 407).await.unwrap();
    assert_ne!(recreated.account_id, a);
    store
        .accept_identity_terms(&alice, TERMS_VERSION, NOW + 407)
        .await
        .unwrap();
    store
        .set_identity_luna(&alice, LUNA_VERSION, true, NOW + 407)
        .await
        .unwrap();
    store.replay_luna_restriction(&receipt).await.unwrap();
    assert!(store.identity_access(&alice).await.unwrap().allows_luna());
    assert!(
        restriction::Entity::find_by_id(event)
            .one(&db)
            .await
            .unwrap()
            .is_some()
    );
    assert!(LunaReceipt::parse(b"{}").is_err());
    assert!(LunaReceipt::parse(&vec![b' '; 4097]).is_err());
    // A different incarnation is never located through external identity/email.
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value["account_id"] = serde_json::json!(Uuid::new_v4());
    let missing = LunaReceipt::parse(&serde_json::to_vec(&value).unwrap()).unwrap();
    store.replay_luna_restriction(&missing).await.unwrap();
    assert!(store.identity_access(&bob).await.unwrap().allows_luna());
    // Pre-journal pending restrictions with no immutable event stay blocked;
    // migration cannot invent their historical scope/time.
    let mut legacy = permission::Entity::find_by_id(b)
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into_active_model();
    legacy.restriction_pending = Set(true);
    legacy.update(&db).await.unwrap();
    assert!(
        store
            .confirm_luna_restriction(&bob, &archive, NOW + 500)
            .await
            .is_err()
    );
    drop(store);
    db.close().await.unwrap();
    admin
        .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
        .await
        .unwrap();
    admin.close().await.unwrap();
}
