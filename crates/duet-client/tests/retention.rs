use duet_client::retention::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[derive(Default)]
struct Store {
    values: Mutex<HashMap<String, Vec<u8>>>,
    fail: Mutex<bool>,
    deny: Mutex<bool>,
}
impl ProtectedStore for Store {
    fn cleanup_pending(&self) -> Result<bool, String> {
        Ok(*self.deny.lock().unwrap())
    }
    fn set_cleanup_pending(&self, value: bool) -> Result<(), String> {
        *self.deny.lock().unwrap() = value;
        Ok(())
    }
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
        if *self.fail.lock().unwrap() {
            return Err("locked".into());
        }
        Ok(self.values.lock().unwrap().get(key).cloned())
    }
    fn write(&self, key: &str, value: &[u8]) -> Result<(), String> {
        if *self.fail.lock().unwrap() {
            return Err("locked".into());
        }
        self.values.lock().unwrap().insert(key.into(), value.into());
        Ok(())
    }
    fn remove(&self, key: &str) -> Result<(), String> {
        if *self.fail.lock().unwrap() {
            return Err("locked".into());
        }
        self.values.lock().unwrap().remove(key);
        Ok(())
    }
}
fn identity(owner: &str) -> RetainedIdentity {
    RetainedIdentity {
        subject: owner.into(),
        owner: owner.into(),
        refresh_token: "secret".into(),
    }
}
fn draft() -> DoctorDraft {
    DoctorDraft {
        base: duet_wire::models::DoctorCollection::new(7),
        save: duet_wire::models::DoctorPageSave::default(),
        uncertain_save: true,
    }
}
#[test]
fn boxed_recovery_preserves_v1_storage_bytes_and_subject_fence() {
    let store = Arc::new(Store::default());
    let owner = RetentionOwner::new(store.clone());
    let fence = owner.activate(identity("account:incarnation")).unwrap();
    owner.save(&fence, draft()).unwrap();
    let (key, bytes) = store
        .values
        .lock()
        .unwrap()
        .iter()
        .find(|(k, _)| k.starts_with("doctor-draft:"))
        .map(|(k, v)| (k.clone(), v.clone()))
        .unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        value,
        serde_json::json!({"version":1,"owner":"account:incarnation","draft":draft()})
    );
    let restarted = RetentionOwner::new(store.clone());
    let same = restarted.activate(identity("account:incarnation")).unwrap();
    assert_eq!(
        restarted.recover(&same).unwrap(),
        DraftRecovery::Draft(Box::new(draft()))
    );
    assert_eq!(store.values.lock().unwrap()[&key], bytes);
    let mut different_subject = identity("account:incarnation");
    different_subject.subject = "different-verified-subject".into();
    let changed = restarted.activate(different_subject).unwrap();
    assert!(restarted.recover(&same).is_err());
    assert_eq!(restarted.recover(&changed).unwrap(), DraftRecovery::Empty);
    assert!(!store.values.lock().unwrap().contains_key(&key));
}
#[test]
fn publication_intent_survives_restart_cannot_rebase_and_is_erased_on_switch() {
    use duet_client::doctor::*;
    let store = Arc::new(Store::default());
    let owner = RetentionOwner::new(store.clone());
    let a = owner.activate(identity("a")).unwrap();
    let request = PublishRequest {
        operation_id: uuid::Uuid::new_v4(),
        expected_generation: 7,
        expires_in_seconds: 604800,
    };
    let original = PendingDoctorOperation::Publish(request.clone());
    owner.retain_operation(&a, &original).unwrap();
    let restarted = RetentionOwner::new(store);
    let fence = restarted.activate(identity("a")).unwrap();
    assert_eq!(
        restarted.pending_operation(&fence).unwrap(),
        Some(original.clone())
    );
    restarted.retain_operation(&fence, &original).unwrap();
    let mut changed = request;
    changed.expected_generation += 1;
    assert!(
        restarted
            .retain_operation(&fence, &PendingDoctorOperation::Publish(changed))
            .is_err()
    );
    restarted.activate(identity("b")).unwrap();
    assert!(restarted.finish_operation(&fence).is_err());
    let again = restarted.activate(identity("a")).unwrap();
    assert_eq!(restarted.pending_operation(&again).unwrap(), None);
}
#[test]
fn conflict_release_requires_exact_definitive_refusal_and_preserves_draft() {
    use duet_client::doctor::*;
    let store = Arc::new(Store::default());
    let owner = RetentionOwner::new(store.clone());
    let fence = owner.activate(identity("a")).unwrap();
    owner.save(&fence, draft()).unwrap();
    let op = PendingDoctorOperation::Clear {
        operation_id: uuid::Uuid::new_v4(),
        expected_generation: 7,
    };
    owner.retain_operation(&fence, &op).unwrap();
    assert!(owner.archive_conflicting_operation(&fence, &op).is_err());
    owner.mark_operation_conflict(&fence, &op).unwrap();
    let restarted = RetentionOwner::new(store.clone());
    let fence = restarted.activate(identity("a")).unwrap();
    assert_eq!(
        restarted.conflicting_operation(&fence).unwrap(),
        Some(op.clone())
    );
    restarted
        .archive_conflicting_operation(&fence, &op)
        .unwrap();
    assert_eq!(restarted.pending_operation(&fence).unwrap(), None);
    assert_eq!(
        restarted.recover(&fence).unwrap(),
        DraftRecovery::Draft(Box::new(draft()))
    );
    assert!(
        store
            .values
            .lock()
            .unwrap()
            .keys()
            .any(|k| k.starts_with("doctor-operation-archive:"))
    );
    restarted.sign_out().unwrap();
    assert!(
        !store
            .values
            .lock()
            .unwrap()
            .keys()
            .any(|k| k.starts_with("doctor-operation-archive:"))
    );
}
#[test]
fn restart_recovers_exact_uncertain_edit_without_network() {
    let store = Arc::new(Store::default());
    let owner = RetentionOwner::new(store.clone());
    let a = owner.activate(identity("a:incarnation-1")).unwrap();
    owner.save(&a, draft()).unwrap();
    let restarted = RetentionOwner::new(store);
    let retained = restarted.retained_identity().unwrap().unwrap();
    let fence = restarted.activate(retained).unwrap();
    assert_eq!(
        restarted.recover(&fence).unwrap(),
        DraftRecovery::Draft(Box::new(draft()))
    );
}
#[test]
fn switch_erases_previous_owner_and_fences_late_response_even_after_switch_back() {
    let owner = RetentionOwner::new(Arc::new(Store::default()));
    let a = owner.activate(identity("a")).unwrap();
    owner.save(&a, draft()).unwrap();
    owner.activate(identity("b")).unwrap();
    assert!(owner.save(&a, draft()).is_err());
    let again = owner.activate(identity("a")).unwrap();
    assert_ne!(a, again);
    assert_eq!(owner.recover(&again).unwrap(), DraftRecovery::Empty);
}
#[test]
fn damaged_draft_survives_read_and_write_until_explicit_discard() {
    let store = Arc::new(Store::default());
    let owner = RetentionOwner::new(store.clone());
    let a = owner.activate(identity("a")).unwrap();
    store
        .values
        .lock()
        .unwrap()
        .insert("doctor-draft:1:a".into(), b"damaged".to_vec());
    assert_eq!(owner.recover(&a).unwrap(), DraftRecovery::Damaged);
    assert!(owner.save(&a, draft()).is_err());
    assert_eq!(owner.recover(&a).unwrap(), DraftRecovery::Damaged);
    owner.discard(&a).unwrap();
    owner.save(&a, draft()).unwrap();
}
#[test]
fn locked_cleanup_invalidates_ui_but_preserves_marker_for_retry() {
    let store = Arc::new(Store::default());
    let owner = RetentionOwner::new(store.clone());
    let a = owner.activate(identity("a")).unwrap();
    owner.save(&a, draft()).unwrap();
    *store.fail.lock().unwrap() = true;
    assert!(owner.sign_out().is_err());
    assert!(!owner.session().is_current(&a));
    assert!(owner.activate(identity("b")).is_err());
    let relaunched = RetentionOwner::new(store.clone());
    assert!(relaunched.cleanup_required().unwrap());
    assert!(relaunched.recover_offline().is_err());
    *store.fail.lock().unwrap() = false;
    assert!(relaunched.activate(identity("a")).is_err());
    assert!(owner.retained_identity().unwrap().is_some());
    owner.sign_out().unwrap();
    assert!(owner.retained_identity().unwrap().is_none());
    let again = owner.activate(identity("a")).unwrap();
    assert_eq!(owner.recover(&again).unwrap(), DraftRecovery::Empty);
}
#[test]
fn readback_requires_exact_content_order_and_generation() {
    let mut draft = draft();
    draft.save.expected_generation = 7;
    assert_eq!(draft.reconcile(&draft.base), SaveReadback::RetryOriginal);
    let mut saved = draft.base.clone();
    saved.note_generation = 8;
    saved.page = Some(draft.save.page.clone());
    saved.note = draft.save.note.clone();
    assert_eq!(draft.reconcile(&saved), SaveReadback::Saved);
    saved.note = Some(Some("someone else's newer edit".into()));
    assert_eq!(draft.reconcile(&saved), SaveReadback::Conflict);
    saved.note = draft.save.note.clone();
    saved.note_generation = 9;
    assert_eq!(draft.reconcile(&saved), SaveReadback::Conflict);
}
#[test]
fn rotation_preserves_fences_and_uncertain_bytes_cannot_be_replaced() {
    let owner = RetentionOwner::new(Arc::new(Store::default()));
    let a = owner.activate(identity("a")).unwrap();
    owner.save(&a, draft()).unwrap();
    owner.rotate_credential(&a, "a", "rotated".into()).unwrap();
    assert!(owner.session().is_current(&a));
    assert_eq!(
        owner.retained_identity().unwrap().unwrap().refresh_token,
        "rotated"
    );
    assert!(owner.rotate_credential(&a, "other", "bad".into()).is_err());
    let mut changed = draft();
    changed.save.note = Some(Some("new words".into()));
    assert!(owner.save(&a, changed).is_err());
    assert_eq!(
        owner.recover(&a).unwrap(),
        DraftRecovery::Draft(Box::new(draft()))
    );
}
#[test]
fn offline_recovery_never_discards_or_approves_a_draft() {
    let owner = RetentionOwner::new(Arc::new(Store::default()));
    let a = owner.activate(identity("a")).unwrap();
    owner.save(&a, draft()).unwrap();
    assert_eq!(
        owner.recover_offline().unwrap(),
        DraftRecovery::Draft(Box::new(draft()))
    );
    assert!(!owner.session().is_current(&a));
}
#[test]
fn missing_native_host_never_falls_back_to_plaintext() {
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    assert!(NativeProtectedStore.write("credential", b"secret").is_err());
}
