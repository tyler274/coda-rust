//! Focused Postgres proof for the reply owner without HTTP or a worker.
use super::*;
use crate::runtime::reply::{FINISH_TOOL, RevisionAccepted, RevisionInput};
use harness::execution::{Call, Record, Response, Usage};

struct TestAuthority(bool);

#[async_trait]
impl Authority for TestAuthority {
    async fn check(&mut self, _: &Scope) -> Result<(), String> {
        Ok(())
    }

    async fn lock_publication(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        session: Option<&[u8]>,
    ) -> Result<(), String> {
        assert_eq!(session, Some([7_u8; 32].as_slice()));
        if self.0 {
            Ok(())
        } else {
            Err("revoked".into())
        }
    }
}

async fn setup() -> (Store, Scope) {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "reply".into(),
    };
    (store, scope)
}

fn input(key: &str, revision: u64) -> RevisionInput {
    RevisionInput {
        key: key.into(),
        expected_revision: revision,
        instruction: "Shorten the reply".into(),
        timezone: "UTC".into(),
    }
}

fn response(text: &str) -> Response {
    Response {
        text: String::new(),
        calls: vec![Call {
            id: "finish".into(),
            name: FINISH_TOOL.into(),
            arguments: serde_json::json!({"text": text, "review_basis": "Saved wording"}),
        }],
        usage: Usage::default(),
        provenance: serde_json::json!({"synthetic": true}),
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn reply_acceptance_is_scoped_idempotent_and_claimed_once() {
    let (store, scope) = setup().await;
    let first = input("first", 1);
    assert_eq!(
        store
            .accept_revision(&scope, &first, vec![7; 32], vec![])
            .await
            .unwrap(),
        RevisionAccepted::Refused
    );
    assert_eq!(store.open_draft(&scope, "Original").await.unwrap(), 1);
    assert_eq!(
        store.open_draft(&scope, "Do not overwrite").await.unwrap(),
        1
    );
    assert_eq!(
        store
            .accept_revision(&scope, &first, vec![7; 32], vec![])
            .await
            .unwrap(),
        RevisionAccepted::New
    );
    assert_eq!(
        store
            .accept_revision(&scope, &first, vec![7; 32], vec![])
            .await
            .unwrap(),
        RevisionAccepted::Duplicate
    );
    let mut collision = first.clone();
    collision.instruction = "Different".into();
    assert!(
        store
            .accept_revision(&scope, &collision, vec![7; 32], vec![])
            .await
            .is_err()
    );
    assert_eq!(
        store
            .accept_revision(&scope, &input("busy", 1), vec![7; 32], vec![])
            .await
            .unwrap(),
        RevisionAccepted::Refused
    );
    let mut foreign = scope.clone();
    foreign.physician = "other".into();
    assert!(store.draft(&foreign).await.unwrap().is_none());
    let delivery = Delivery::reply_revision(scope.clone(), first.key);
    assert!(store.claim_delivery(&delivery, 60).await.is_err());
    let (a, b) = tokio::join!(
        store.claim_revision(&delivery, 60),
        store.claim_revision(&delivery, 60)
    );
    let claims = [a.unwrap(), b.unwrap()];
    assert_eq!(claims.iter().filter(|c| c.is_some()).count(), 1);
    let claim = claims.into_iter().flatten().next().unwrap();
    assert_eq!(claim.base_text, "Original");
    assert_eq!(
        store.delivery_state(&delivery).await.unwrap(),
        DeliveryState::Claimed
    );
    store.retry_revision(&claim).await.unwrap();
    let retry = store.claim_revision(&delivery, 60).await.unwrap().unwrap();
    assert_ne!(retry.attempt, claim.attempt);
    assert_eq!(retry.earlier_attempts, 1);
    assert!(store.assert_revision_current(&claim).await.is_err());
    assert!(store.read(&scope).await.unwrap().is_none());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn reply_publication_requires_evidence_authority_and_current_revision() {
    let (store, scope) = setup().await;
    store.open_draft(&scope, "Original").await.unwrap();
    store
        .accept_revision(&scope, &input("first", 1), vec![7; 32], vec![])
        .await
        .unwrap();
    let claim = store
        .claim_revision(&Delivery::reply_revision(scope.clone(), "first".into()), 60)
        .await
        .unwrap()
        .unwrap();
    let output = response("Shorter");
    assert!(
        store
            .commit_revision(&claim, &output, &mut TestAuthority(true))
            .await
            .is_err()
    );
    store
        .record_revision(
            &claim,
            Record::Exchange {
                sequence: 1,
                response: output.clone(),
            },
        )
        .await
        .unwrap();
    assert!(
        store
            .authorize_revision(&scope, &mut TestAuthority(false))
            .await
            .is_err()
    );
    assert!(
        store
            .commit_revision(&claim, &output, &mut TestAuthority(false))
            .await
            .is_err()
    );
    assert!(
        store
            .save_draft(&scope, 1, "Denied", &[7; 32], &mut TestAuthority(false))
            .await
            .is_err()
    );
    assert_eq!(store.draft(&scope).await.unwrap().unwrap().revision, 1);
    assert_eq!(
        store
            .save_draft(
                &scope,
                1,
                "Physician save",
                &[7; 32],
                &mut TestAuthority(true)
            )
            .await
            .unwrap(),
        2
    );
    assert!(
        store
            .save_draft(&scope, 1, "Stale", &[7; 32], &mut TestAuthority(true))
            .await
            .is_err()
    );
    assert!(
        store
            .commit_revision(&claim, &output, &mut TestAuthority(true))
            .await
            .is_err()
    );
    let draft = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(draft.text, "Physician save");
    assert_eq!(draft.status, "superseded");
    assert_eq!(draft.restore_text.as_deref(), Some("Original"));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn reply_history_keeps_completed_attempts_append_only() {
    let (store, scope) = setup().await;
    store.open_draft(&scope, "Original").await.unwrap();
    let mut completed = Vec::new();
    for (index, key) in ["first", "second", "third"].iter().enumerate() {
        store
            .accept_revision(&scope, &input(key, index as u64 + 1), vec![7; 32], vec![])
            .await
            .unwrap();
        let claim = store
            .claim_revision(&Delivery::reply_revision(scope.clone(), (*key).into()), 60)
            .await
            .unwrap()
            .unwrap();
        let output = response(key);
        store
            .record_revision(
                &claim,
                Record::Exchange {
                    sequence: 1,
                    response: output.clone(),
                },
            )
            .await
            .unwrap();
        store
            .commit_revision(&claim, &output, &mut TestAuthority(true))
            .await
            .unwrap();
        completed.push(store.draft(&scope).await.unwrap().unwrap());
        let history = store.earlier_revisions(&scope).await.unwrap();
        assert_eq!(history.len(), index);
        for (earlier, original) in history.iter().zip(&completed) {
            assert_eq!(earlier.input, original.input.clone().unwrap());
            assert_eq!(earlier.status, "completed");
            assert_eq!(
                serde_json::to_value(&earlier.attempts).unwrap(),
                serde_json::to_value(&original.attempts).unwrap()
            );
            assert_eq!(earlier.base_text, original.base_text);
        }
    }
    let current = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(current.attempts.len(), 1);
    assert_eq!(current.revision, 4);
    let mut foreign = scope.clone();
    foreign.physician = "other".into();
    assert!(store.earlier_revisions(&foreign).await.unwrap().is_empty());
}

#[test]
fn reply_delivery_preserves_legacy_chat_wire_shape() {
    let scope = Scope {
        practice: "p".into(),
        physician: "d".into(),
        workspace: "w".into(),
        chat: "c".into(),
    };
    let legacy = serde_json::json!({"scope": scope, "input_id": "turn"});
    let chat: Delivery = serde_json::from_value(legacy.clone()).unwrap();
    assert_eq!(serde_json::to_value(chat).unwrap(), legacy);
    let revision = Delivery::reply_revision(scope, "revise".into());
    assert_eq!(
        serde_json::to_value(&revision).unwrap()["kind"],
        "reply_revision"
    );
    assert_eq!(
        serde_json::from_value::<Delivery>(serde_json::to_value(&revision).unwrap()).unwrap(),
        revision
    );
}
