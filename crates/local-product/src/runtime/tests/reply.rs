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
        scenario: None,
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
            .accept_revision(&scope, &first, vec![7; 32])
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
            .accept_revision(&scope, &first, vec![7; 32])
            .await
            .unwrap(),
        RevisionAccepted::New
    );
    assert_eq!(
        store
            .accept_revision(&scope, &first, vec![7; 32])
            .await
            .unwrap(),
        RevisionAccepted::Duplicate
    );
    let mut collision = first.clone();
    collision.instruction = "Different".into();
    assert!(
        store
            .accept_revision(&scope, &collision, vec![7; 32])
            .await
            .is_err()
    );
    assert_eq!(
        store
            .accept_revision(&scope, &input("busy", 1), vec![7; 32])
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
        .accept_revision(&scope, &input("first", 1), vec![7; 32])
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
            .accept_revision(&scope, &input(key, index as u64 + 1), vec![7; 32])
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

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn late_evidence_for_an_archived_attempt_joins_its_history_and_never_publishes() {
    let (store, scope) = setup().await;
    store.open_draft(&scope, "Original").await.unwrap();
    let first = input("first", 1);
    store
        .accept_revision(&scope, &first, vec![7; 32])
        .await
        .unwrap();
    let late = store
        .claim_revision(&Delivery::reply_revision(scope.clone(), first.key), 60)
        .await
        .unwrap()
        .unwrap();
    let dispatch = Record::Dispatch {
        sequence: 1,
        request: crate::runtime::reply::request("m", "low", "Original", "Shorten", "UTC", &[]),
    };
    store
        .record_revision(&late, dispatch.clone())
        .await
        .unwrap();
    // The physician saves over the running revision, then accepts another,
    // which moves the first revision and its attempt into history.
    store
        .save_draft(
            &scope,
            1,
            "Physician save",
            &[7; 32],
            &mut TestAuthority(true),
        )
        .await
        .unwrap();
    store
        .accept_revision(&scope, &input("second", 2), vec![7; 32])
        .await
        .unwrap();
    let second = store
        .claim_revision(
            &Delivery::reply_revision(scope.clone(), "second".into()),
            60,
        )
        .await
        .unwrap()
        .unwrap();
    let before = store.draft(&scope).await.unwrap().unwrap();
    // The first worker's provider answers late, then its run fails.
    let output = response("Late first output");
    let exchange = Record::Exchange {
        sequence: 1,
        response: output.clone(),
    };
    let failure = Record::Failure {
        sequence: 1,
        reason: "revision is no longer current".into(),
    };
    store
        .record_revision(&late, exchange.clone())
        .await
        .unwrap();
    store.record_revision(&late, failure.clone()).await.unwrap();
    // A late dispatch, completion or retry from it is still refused.
    assert!(
        store
            .record_revision(&late, dispatch.clone())
            .await
            .is_err()
    );
    assert!(
        store
            .commit_revision(&late, &output, &mut TestAuthority(true))
            .await
            .is_err()
    );
    store.retry_revision(&late).await.unwrap();
    let history = store.earlier_revisions(&scope).await.unwrap();
    let [earlier] = history.as_slice() else {
        panic!("one earlier revision expected");
    };
    assert_eq!(earlier.input, input("first", 1));
    assert_eq!(earlier.status, "superseded");
    let [attempt] = earlier.attempts.as_slice() else {
        panic!("one archived attempt expected");
    };
    assert_eq!(attempt.id, late.attempt);
    assert_eq!(attempt.status, "interrupted");
    assert_eq!(
        serde_json::to_value(&attempt.records).unwrap(),
        serde_json::to_value([dispatch, exchange, failure]).unwrap()
    );
    // The newer revision, its text and its attempt are untouched.
    let after = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(
        serde_json::to_value(&after).unwrap(),
        serde_json::to_value(&before).unwrap()
    );
    assert_eq!(after.text, "Physician save");
    assert_eq!(after.revision, 2);
    assert_eq!(after.status, "active");
    assert_eq!(after.attempts.len(), 1);
    assert!(after.attempts[0].records.is_empty());
    store.assert_revision_current(&second).await.unwrap();
    // An attempt this draft never had is still refused.
    let mut unknown = late.clone();
    unknown.attempt = "never-claimed".into();
    assert!(
        store
            .record_revision(
                &unknown,
                Record::Failure {
                    sequence: 1,
                    reason: "x".into()
                }
            )
            .await
            .is_err()
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn earlier_revisions_read_in_archive_order_not_key_order() {
    use crate::runtime::entities::reply_revision;
    let (store, scope) = setup().await;
    store.open_draft(&scope, "Original").await.unwrap();
    // Keys sort opposite to acceptance order.
    let keys = ["z-first", "a-second", "m-third", "b-fourth"];
    for (index, key) in keys.iter().enumerate() {
        let revision = index as u64 + 1;
        store
            .accept_revision(&scope, &input(key, revision), vec![7; 32])
            .await
            .unwrap();
        store
            .save_draft(
                &scope,
                revision,
                &format!("Save {revision}"),
                &[7; 32],
                &mut TestAuthority(true),
            )
            .await
            .unwrap();
    }
    // The archive clock has one-second resolution; give every row the same
    // second so only the durable ordinal can order them.
    reply_revision::Entity::update_many()
        .col_expr(reply_revision::Column::ArchivedAt, Expr::value(1_i64))
        .filter(reply_revision::Column::Practice.eq(scope.practice.as_str()))
        .exec(&store.db)
        .await
        .unwrap();
    let history = store.earlier_revisions(&scope).await.unwrap();
    assert_eq!(
        history
            .iter()
            .map(|earlier| earlier.input.key.as_str())
            .collect::<Vec<_>>(),
        keys[..3]
    );
    // Another draft of the same physician numbers its history separately.
    let mut other = scope.clone();
    other.chat = "other-reply".into();
    store.open_draft(&other, "Other").await.unwrap();
    for (index, key) in ["other-1", "other-2"].iter().enumerate() {
        let revision = index as u64 + 1;
        store
            .accept_revision(&other, &input(key, revision), vec![7; 32])
            .await
            .unwrap();
        store
            .save_draft(
                &other,
                revision,
                "Other save",
                &[7; 32],
                &mut TestAuthority(true),
            )
            .await
            .unwrap();
    }
    let ordinals = |reply: &str| {
        reply_revision::Entity::find()
            .filter(reply_revision::Column::Practice.eq(scope.practice.as_str()))
            .filter(reply_revision::Column::Reply.eq(reply))
            .order_by_asc(reply_revision::Column::Ordinal)
            .all(&store.db)
    };
    let ordinals_of = |rows: Vec<reply_revision::Model>| {
        rows.into_iter().map(|row| row.ordinal).collect::<Vec<_>>()
    };
    assert_eq!(ordinals_of(ordinals("reply").await.unwrap()), [1, 2, 3]);
    assert_eq!(ordinals_of(ordinals("other-reply").await.unwrap()), [1]);
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

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn authorized_opening_is_idempotent_scoped_and_fenced_by_authority() {
    let (store, scope) = setup().await;
    let session = [7_u8; 32];
    // Revoked authority opens nothing and leaves no row behind.
    assert!(
        store
            .open_draft_authorized(&scope, "Original", &session, &mut TestAuthority(false))
            .await
            .unwrap()
            .is_none()
    );
    assert!(store.draft(&scope).await.unwrap().is_none());
    assert!(
        store
            .open_draft_authorized(&scope, " ", &session, &mut TestAuthority(true))
            .await
            .is_err()
    );

    // Concurrent openers agree on exactly one revision-1 draft.
    let (mut a, mut b) = (TestAuthority(true), TestAuthority(true));
    let (first, second) = tokio::join!(
        store.open_draft_authorized(&scope, "Original", &session, &mut a),
        store.open_draft_authorized(&scope, "Racing text", &session, &mut b)
    );
    let (first, second) = (first.unwrap().unwrap(), second.unwrap().unwrap());
    assert_eq!((first.revision, second.revision), (1, 1));
    assert_eq!(first.text, second.text);
    let opened = first.text.clone();

    // A physician edit and an accepted revision survive reopening unchanged.
    let saved = store
        .save_draft(&scope, 1, "Edited", &session, &mut TestAuthority(true))
        .await
        .unwrap();
    assert_eq!(saved, 2);
    assert_eq!(
        store
            .accept_revision(&scope, &input("kept", 2), vec![7; 32])
            .await
            .unwrap(),
        RevisionAccepted::New
    );
    let reopened = store
        .open_draft_authorized(&scope, "Original", &session, &mut TestAuthority(true))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(reopened.revision, 2);
    assert_eq!(reopened.text, "Edited");
    assert_eq!(reopened.restore_text.as_deref(), Some(opened.as_str()));
    assert_eq!(
        reopened.input.as_ref().map(|i| i.key.as_str()),
        Some("kept")
    );
    // Revoked authority cannot even read through the opener once it exists.
    assert!(
        store
            .open_draft_authorized(&scope, "Original", &session, &mut TestAuthority(false))
            .await
            .unwrap()
            .is_none()
    );

    // Another physician and another workspace each get their own draft.
    let mut other_physician = scope.clone();
    other_physician.physician = "other".into();
    let mut other_workspace = scope.clone();
    other_workspace.workspace = "other-workspace".into();
    for other in [&other_physician, &other_workspace] {
        let draft = store
            .open_draft_authorized(other, "Original", &session, &mut TestAuthority(true))
            .await
            .unwrap()
            .unwrap();
        assert_eq!((draft.revision, draft.text.as_str()), (1, "Original"));
    }
    assert_eq!(store.draft(&scope).await.unwrap().unwrap().text, "Edited");

    // Save still refuses an arbitrary reply that was never opened.
    let mut missing = scope.clone();
    missing.chat = "never-opened".into();
    assert!(
        store
            .save_draft(&missing, 1, "Invented", &session, &mut TestAuthority(true))
            .await
            .is_err()
    );
    assert!(store.draft(&missing).await.unwrap().is_none());
}
