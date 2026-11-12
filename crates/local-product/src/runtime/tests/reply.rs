//! Focused Postgres proof for the reply owner without HTTP or a worker.
use super::*;
use crate::runtime::reply::{DraftStatus, FINISH_TOOL, RevisionAccepted, RevisionInput};
use harness::engine::recipe::Usage;
use harness::engine::runs::{HoldReason, RunEvent, RunOutcome};
use harness::engine::{Event, EventKind};
use harness::engine::{Invocation, Turn};

struct TestAuthority(bool);

#[async_trait]
impl Authority for TestAuthority {
    async fn check(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
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

// This authority denies only the read path, so using publication locking for
// a currency check cannot accidentally satisfy its refusal assertion.
struct RevokedRead;

#[async_trait]
impl Authority for RevokedRead {
    async fn check(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        session: Option<&[u8]>,
    ) -> Result<(), String> {
        assert_eq!(session, Some([7_u8; 32].as_slice()));
        Err("revoked".into())
    }

    async fn lock_publication(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
}

/// Every field of two drafts is equal. Attempts compare through their saved
/// JSON, since their events have no equality of their own.
fn assert_same_draft(left: &reply::Draft, right: &reply::Draft) {
    let reply::Draft {
        revision,
        text,
        restore_text,
        status,
        accepted,
        runs,
        base_text,
        sources,
        sources_version,
        allowance_after,
        attempts,
        session_hash,
        dispatch_after,
        failures,
    } = left;
    assert_eq!(*revision, right.revision);
    assert_eq!(text, &right.text);
    assert_eq!(restore_text, &right.restore_text);
    assert_eq!(*status, right.status);
    assert_eq!(
        accepted.as_ref().map(|a| (&a.input, &a.binding, &a.skills)),
        right
            .accepted
            .as_ref()
            .map(|a| (&a.input, &a.binding, &a.skills))
    );
    assert_eq!(runs, &right.runs);
    assert_eq!(base_text, &right.base_text);
    assert_eq!(sources, &right.sources);
    assert_eq!(*sources_version, right.sources_version);
    assert_eq!(*allowance_after, right.allowance_after);
    assert_eq!(
        serde_json::to_value(attempts).unwrap(),
        serde_json::to_value(&right.attempts).unwrap()
    );
    assert_eq!(session_hash, &right.session_hash);
    assert_eq!(*dispatch_after, right.dispatch_after);
    assert_eq!(*failures, right.failures);
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

fn response(text: &str) -> Turn {
    Turn {
        items: vec![
            Invocation {
                id: "finish".into(),
                name: FINISH_TOOL.into(),
                arguments: serde_json::json!({"text": text, "review_basis": "Saved wording"}),
            }
            .into(),
        ],
        usage: Usage::default(),
        receipt: serde_json::json!({"synthetic": true}),
    }
}

fn event(claim: &reply::RevisionClaim, sequence: u64, kind: EventKind) -> Event {
    Event {
        attempt_id: claim.attempt.clone(),
        sequence,
        ordinal: 1,
        kind,
    }
}

async fn candidate(store: &Store, claim: &reply::RevisionClaim, output: &Turn) {
    for event in [
        event(claim, 1, EventKind::ProviderReturned(output.clone())),
        event(
            claim,
            2,
            EventKind::Candidate {
                text: output.text(),
            },
        ),
    ] {
        store.record_revision(claim, event).await.unwrap();
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn revision_billing_record_atomically_ends_automatic_recovery() {
    let (store, scope) = setup().await;
    open_draft(&store, &scope, "Original").await;
    accept_revision_input(&store, &scope, &input("blocked", 1))
        .await
        .unwrap();
    let delivery = Delivery::reply_revision(scope.clone(), "blocked".into());
    let claim = store.claim_revision(&delivery, 60).await.unwrap().unwrap();
    let blocked = event(&claim, 1, EventKind::ProviderBlocked { reason: "Provider credits are exhausted. Restore billing, then retry this work (insufficient_quota).".into() });
    store
        .record_revision(&claim, blocked.clone())
        .await
        .unwrap();
    drop(store);
    let reopened = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    reopened.record_revision(&claim, blocked).await.unwrap();
    reopened
        .settle_revision_attempt(&claim, harness::engine::AttemptFailure::Failed)
        .await
        .unwrap();
    assert!(
        reopened
            .claim_revision(&delivery, 60)
            .await
            .unwrap()
            .is_none()
    );
    let saved = reopened.draft(&scope).await.unwrap().unwrap();
    assert_eq!(saved.status, DraftStatus::Held);
    assert_eq!(saved.failures, 0);
    assert_eq!(saved.attempts[0].events.len(), 1);
    assert_eq!(saved.attempts[0].status, AttemptStatus::Held);
    assert_eq!(
        saved.runs.current().unwrap().outcome,
        Some(RunOutcome::Held {
            reason: HoldReason::ProviderBlocked,
        })
    );
    assert_eq!(
        saved
            .runs
            .events()
            .iter()
            .filter(|event| matches!(event, RunEvent::RunEnded { .. }))
            .count(),
        1
    );
    assert_eq!(saved.text, "Original");
    assert_eq!(
        accept_revision_input(&reopened, &scope, &input("after-billing-repair", 1))
            .await
            .unwrap(),
        RevisionAccepted::New
    );
    println!(
        "revision billing readback: typed blocked event and failed disposition atomic, no automatic reclaim, explicit revision remains available"
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn reply_acceptance_is_scoped_idempotent_and_claimed_once() {
    let (store, scope) = setup().await;
    let first = input("first", 1);
    let mut configured_recipe = test_revision_recipe();
    assert_eq!(
        store
            .accept_revision(
                &scope,
                &first,
                vec![7; 32],
                &test_skills(),
                &configured_recipe
            )
            .await
            .unwrap(),
        RevisionAccepted::Refused
    );
    assert_eq!(open_draft(&store, &scope, "Original").await, 1);
    assert_eq!(open_draft(&store, &scope, "Do not overwrite").await, 1);
    assert_eq!(
        store
            .accept_revision(
                &scope,
                &first,
                vec![7; 32],
                &test_skills(),
                &configured_recipe
            )
            .await
            .unwrap(),
        RevisionAccepted::New
    );
    let accepted = store.draft(&scope).await.unwrap().unwrap();
    let package = store
        .frozen_skills(&scope, &accepted.accepted.as_ref().unwrap().skills)
        .await
        .unwrap();
    let result: serde_json::Value = serde_json::from_str(
        package
            .execute(
                harness::skills::SKILL_VIEW,
                &serde_json::json!({"name":"patient-record-retrieval"}),
            )
            .text()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        result["status"], "available",
        "the accepted revision's saved package must open every Chords skill"
    );
    assert!(
        accepted.accepted.is_some(),
        "the revision recipe must be frozen before any worker claims it"
    );
    let run = accepted.runs.current().unwrap().id.to_owned();
    assert_eq!(accepted.runs.events().len(), 1);
    configured_recipe.model = "changed-after-acceptance".into();
    configured_recipe.instructions = "changed-after-acceptance".into();
    configured_recipe.options = serde_json::json!({"effort":"high"});
    configured_recipe.context_window_tokens = 200_000;
    configured_recipe.max_input_tokens = 150_000;
    configured_recipe.tools.clear();
    assert_eq!(
        store
            .accept_revision(
                &scope,
                &first,
                vec![7; 32],
                &test_skills(),
                &configured_recipe
            )
            .await
            .unwrap(),
        RevisionAccepted::Duplicate
    );
    let mut collision = first.clone();
    collision.instruction = "Different".into();
    assert!(
        accept_revision_input(&store, &scope, &collision)
            .await
            .is_err()
    );
    assert_eq!(
        accept_revision_input(&store, &scope, &input("busy", 1))
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
    let saved = store
        .draft(&scope)
        .await
        .unwrap()
        .unwrap()
        .accepted
        .unwrap()
        .binding;
    assert_eq!(saved, accepted.accepted.unwrap().binding);
    let saved = store.frozen_binding(&scope, &saved).await.unwrap();
    assert_eq!(saved.recipe.model, "controlled");
    assert_eq!(saved.recipe.options, serde_json::json!({"effort":"none"}));
    assert_eq!(
        saved.recipe.instructions,
        test_revision_recipe().instructions
    );
    assert_eq!(saved.recipe.context_window_tokens, 272_000);
    assert_eq!(saved.recipe.max_input_tokens, 272_000);
    assert!(!saved.recipe.tools.is_empty());
    store.admit_revision_binding(&claim, &saved).await.unwrap();
    assert!(
        store
            .admit_revision_binding(&claim, &configured_recipe.bind(saved.cache_key.clone()))
            .await
            .is_err()
    );
    let mut changed_scope = saved.clone();
    changed_scope.cache_key = Some(harness::PromptCacheKey::scoped(
        "other-revision",
        &["other"],
    ));
    assert!(
        store
            .admit_revision_binding(&claim, &changed_scope)
            .await
            .is_err(),
        "accepted cache scope cannot change when the recipe is shared"
    );
    assert_eq!(claim.base_text, "Original");
    assert_eq!(
        store.delivery_state(&delivery).await.unwrap(),
        DeliveryState::Claimed
    );
    store
        .settle_revision_attempt(&claim, harness::engine::AttemptFailure::Failed)
        .await
        .unwrap();
    let retry = store.claim_revision(&delivery, 60).await.unwrap().unwrap();
    assert_ne!(retry.attempt, claim.attempt);
    let recovering = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(recovering.runs.current().unwrap().id, run);
    assert_eq!(recovering.runs.current().unwrap().outcome, None);
    assert_eq!(recovering.runs.events().len(), 1);

    let lost = store.assert_revision_current(&claim).await.unwrap_err();
    assert_eq!(
        engine::AttemptFailure::from_cause(lost.as_ref()),
        engine::AttemptFailure::Failed,
        "a replaced attempt has lost ownership: {lost}"
    );
    assert!(store.renew_revision_claim(&claim).await.is_err());
    store.renew_revision_claim(&retry).await.unwrap();
    assert!(store.read(&scope).await.unwrap().is_none());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn held_revision_saves_one_limit_outcome_with_its_visible_status() {
    let (store, scope) = setup().await;
    open_draft(&store, &scope, "Original").await;
    accept_revision_input(&store, &scope, &input("held", 1))
        .await
        .unwrap();
    let claim = store
        .claim_revision(&Delivery::reply_revision(scope.clone(), "held".into()), 60)
        .await
        .unwrap()
        .unwrap();
    let frozen = claim.binding.request(
        crate::runtime::reply::RevisionContext {
            saved_draft: "Original".into(),
            instruction: claim.accepted.input.message(),
            physician_time_zone: "UTC".into(),
            expected_draft_revision: 1,
        }
        .items(&[]),
    );
    store
        .admit_revision_binding(&claim, &frozen.binding())
        .await
        .unwrap();
    let handoff = Turn {
        items: vec![harness::engine::Item::Message {
            role: "assistant".into(),
            text: "A retained partial revision.".into(),
        }],
        usage: Usage::default(),
        receipt: serde_json::json!({"synthetic": true}),
    };
    for event in [
        event(
            &claim,
            1,
            EventKind::ProviderStarted {
                dispatched_at_unix_ms: 1,
                request: harness::engine::SavedRequest {
                    added: frozen.items.clone(),
                    items: (0..frozen.items.len())
                        .map(|index| harness::engine::RequestItem::Event {
                            attempt: claim.attempt.clone(),
                            sequence: 1,
                            index,
                        })
                        .collect(),
                },
                context: harness::engine::DispatchContext {
                    window_stage: harness::engine::WindowStage::Accumulated,
                    purpose: harness::engine::RequestPurpose::Final {
                        backstops: harness::engine::ReachedBackstops::new(vec![
                            harness::engine::LimitBackstop::Requests,
                        ])
                        .unwrap(),
                    },
                    ..Default::default()
                },
            },
        ),
        event(&claim, 2, EventKind::ProviderReturned(handoff.clone())),
    ] {
        store.record_revision(&claim, event).await.unwrap();
    }
    store.hold_revision(&claim, &handoff).await.unwrap();
    let held = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(held.status, DraftStatus::Held);
    assert_eq!(held.attempts.last().unwrap().status, AttemptStatus::Held);
    assert_eq!(
        held.runs.current().unwrap().outcome,
        Some(RunOutcome::Held {
            reason: HoldReason::Limit,
        })
    );
    assert_eq!(
        held.runs
            .events()
            .iter()
            .filter(|event| matches!(event, RunEvent::RunEnded { .. }))
            .count(),
        1
    );
    let previous = held.runs.current().unwrap().id.to_owned();
    for (work, run) in [
        ("wrong-work", previous.as_str()),
        (scope.chat.as_str(), "stale-run"),
    ] {
        assert_eq!(
            store
                .resume_authorized(&scope, "retry-held", vec![8; 32], work, run)
                .await
                .unwrap(),
            Resumed::Refused
        );
    }
    let delivery = Delivery::reply_revision(scope.clone(), claim.accepted.input.key.clone());
    assert_eq!(
        store
            .resume_authorized(&scope, "retry-held", vec![8; 32], &scope.chat, &previous)
            .await
            .unwrap(),
        Resumed::Pending {
            delivery: delivery.clone()
        }
    );
    assert_eq!(
        store
            .resume_authorized(&scope, "retry-held", vec![8; 32], &scope.chat, &previous)
            .await
            .unwrap(),
        Resumed::Duplicate
    );
    let restarted = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(restarted.status, DraftStatus::Pending);
    assert_eq!(restarted.text, held.text);
    assert_eq!(
        restarted
            .accepted
            .as_ref()
            .map(|accepted| &accepted.binding),
        held.accepted.as_ref().map(|accepted| &accepted.binding)
    );
    assert_eq!(
        restarted.runs.current().unwrap().continues,
        Some(previous.as_str())
    );
    assert_eq!(restarted.allowance_after, 1);
    assert_eq!(
        restarted.session_hash.as_deref(),
        Some([8_u8; 32].as_slice())
    );
    let successor = store.claim_revision(&delivery, 60).await.unwrap().unwrap();
    assert_ne!(successor.attempt, claim.attempt);
    assert_eq!(
        serde_json::to_value(&store.draft(&scope).await.unwrap().unwrap().attempts[0].events)
            .unwrap(),
        serde_json::to_value(&held.attempts[0].events).unwrap()
    );
    assert!(store.read(&scope).await.unwrap().is_none());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn reply_publication_requires_evidence_authority_and_current_revision() {
    let (store, scope) = setup().await;
    open_draft(&store, &scope, "Original").await;
    accept_revision_input(&store, &scope, &input("first", 1))
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
            event(&claim, 1, EventKind::ProviderReturned(output.clone())),
        )
        .await
        .unwrap();
    // A returned provider turn is evidence, not an accepted completion candidate.
    assert!(
        store
            .commit_revision(&claim, &output, &mut TestAuthority(true))
            .await
            .is_err()
    );
    store
        .record_revision(
            &claim,
            event(
                &claim,
                2,
                EventKind::Candidate {
                    text: output.text(),
                },
            ),
        )
        .await
        .unwrap();
    assert!(
        store
            .authorize_revision(&scope, &mut RevokedRead)
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
    let error = store.assert_revision_current(&claim).await.unwrap_err();
    assert!(
        error.is::<engine::Withdrawn>(),
        "a save cancels the revision, so the running attempt returns at once: {error}"
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
    assert_eq!(draft.status, DraftStatus::Cancelled);
    assert_eq!(
        draft.runs.current().unwrap().outcome,
        Some(RunOutcome::Cancelled)
    );
    assert_eq!(draft.restore_text.as_deref(), Some("Original"));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_save_then_a_new_instruction_withdraws_the_running_attempt_and_the_instruction_runs_next()
{
    let (store, scope) = setup().await;
    open_draft(&store, &scope, "Original").await;
    accept_revision_input(&store, &scope, &input("first", 1))
        .await
        .unwrap();
    let claim = store
        .claim_revision(&Delivery::reply_revision(scope.clone(), "first".into()), 60)
        .await
        .unwrap()
        .unwrap();
    let revision = store
        .save_draft(
            &scope,
            1,
            "Physician save",
            &[7; 32],
            &mut TestAuthority(true),
        )
        .await
        .unwrap();
    // The new instruction arrives before the worker's next check.
    assert_eq!(
        accept_revision_input(&store, &scope, &input("second", revision))
            .await
            .unwrap(),
        RevisionAccepted::New
    );
    let error = store.assert_revision_current(&claim).await.unwrap_err();
    assert!(
        error.is::<engine::Withdrawn>(),
        "the save ended the attempt, so the new instruction does not make it wait: {error}"
    );
    let reason = engine::AttemptFailure::from_cause(error.as_ref());
    store.settle_revision_attempt(&claim, reason).await.unwrap();
    let next = store
        .claim_revision(
            &Delivery::reply_revision(scope.clone(), "second".into()),
            60,
        )
        .await
        .unwrap()
        .unwrap();
    assert_ne!(next.attempt, claim.attempt);
    assert_eq!(next.accepted.input, input("second", revision));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn reply_history_keeps_completed_attempts_append_only() {
    let (store, scope) = setup().await;
    open_draft(&store, &scope, "Original").await;
    let mut completed = Vec::new();
    for (index, key) in ["first", "second", "third"].iter().enumerate() {
        accept_revision_input(&store, &scope, &input(key, index as u64 + 1))
            .await
            .unwrap();
        let claim = store
            .claim_revision(&Delivery::reply_revision(scope.clone(), (*key).into()), 60)
            .await
            .unwrap()
            .unwrap();
        let output = response(key);
        candidate(&store, &claim, &output).await;
        store
            .commit_revision(&claim, &output, &mut TestAuthority(true))
            .await
            .unwrap();
        completed.push(store.draft(&scope).await.unwrap().unwrap());
        let history = store.earlier_revisions(&scope).await.unwrap();
        assert_eq!(history.len(), index);
        for (earlier, original) in history.iter().zip(&completed) {
            assert_eq!(
                earlier.accepted.input,
                original.accepted.clone().unwrap().input
            );
            assert_eq!(
                earlier.runs.current().unwrap().outcome,
                Some(RunOutcome::Completed)
            );
            assert_eq!(earlier.runs.events().len(), 2);
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
    open_draft(&store, &scope, "Original").await;
    let first = input("first", 1);
    accept_revision_input(&store, &scope, &first).await.unwrap();
    let late = store
        .claim_revision(&Delivery::reply_revision(scope.clone(), first.key), 60)
        .await
        .unwrap()
        .unwrap();
    let frozen = late.binding.request(
        crate::runtime::reply::RevisionContext {
            saved_draft: "Original".into(),
            instruction: late.accepted.input.message(),
            physician_time_zone: "UTC".into(),
            expected_draft_revision: 1,
        }
        .items(&[]),
    );
    store
        .admit_revision_binding(&late, &frozen.binding())
        .await
        .unwrap();
    let dispatch = event(
        &late,
        1,
        EventKind::ProviderStarted {
            dispatched_at_unix_ms: 1,
            request: harness::engine::SavedRequest {
                added: frozen.items.clone(),
                items: (0..frozen.items.len())
                    .map(|index| harness::engine::RequestItem::Event {
                        attempt: late.attempt.clone(),
                        sequence: 1,
                        index,
                    })
                    .collect(),
            },
            context: Default::default(),
        },
    );
    store
        .record_revision(&late, dispatch.clone())
        .await
        .unwrap();
    let mut replacement = frozen.binding();
    replacement.recipe.model = "replacement".into();
    assert!(
        store
            .admit_revision_binding(&late, &replacement)
            .await
            .is_err()
    );
    let pinned = store
        .draft(&scope)
        .await
        .unwrap()
        .unwrap()
        .accepted
        .unwrap()
        .binding;
    assert_eq!(
        store.frozen_binding(&scope, &pinned).await.unwrap(),
        frozen.binding()
    );
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
    accept_revision_input(&store, &scope, &input("second", 2))
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
    assert!(before.accepted.is_some());
    let archive_key = (
        scope.practice.clone(),
        scope.physician.clone(),
        scope.workspace.clone(),
        scope.chat.clone(),
        "first".to_owned(),
    );
    let archive_before = entities::reply_revision::Entity::find_by_id(archive_key.clone())
        .one(&store.db)
        .await
        .unwrap()
        .unwrap();
    // The first worker's provider answers late. Its returned evidence is kept;
    // a stale host check cannot produce a completion candidate or publish.
    let output = response("Late first output");
    let exchange = event(&late, 2, EventKind::ProviderReturned(output.clone()));
    store
        .record_revision(&late, exchange.clone())
        .await
        .unwrap();
    assert_eq!(
        entities::reply_revision::Entity::find_by_id(archive_key.clone())
            .one(&store.db)
            .await
            .unwrap()
            .unwrap(),
        archive_before,
        "late evidence appends without rewriting archived revision metadata"
    );
    // A late dispatch, completion or retry from it is still refused.
    assert!(
        store
            .record_revision(
                &late,
                event(
                    &late,
                    3,
                    EventKind::ToolStarted(harness::engine::Invocation {
                        id: "stale-tool".into(),
                        name: "skill_view".into(),
                        arguments: "{}".into(),
                    })
                )
            )
            .await
            .is_err(),
        "a stale revision must not admit a new tool call"
    );
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
    store
        .settle_revision_attempt(&late, harness::engine::AttemptFailure::Failed)
        .await
        .unwrap();
    let history = store.earlier_revisions(&scope).await.unwrap();
    let [earlier] = history.as_slice() else {
        panic!("one earlier revision expected");
    };
    assert_eq!(earlier.accepted.input, input("first", 1));
    assert_eq!(
        earlier.runs.current().unwrap().outcome,
        Some(RunOutcome::Cancelled)
    );
    let [attempt] = earlier.attempts.as_slice() else {
        panic!("one archived attempt expected");
    };
    assert_eq!(attempt.id, late.attempt);
    assert_eq!(attempt.status, AttemptStatus::Interrupted);
    assert_eq!(
        serde_json::to_value(&attempt.events).unwrap(),
        serde_json::to_value([dispatch, exchange]).unwrap()
    );
    // The newer revision, its text and its attempt are untouched.
    let after = store.draft(&scope).await.unwrap().unwrap();
    assert_same_draft(&after, &before);
    assert_eq!(after.text, "Physician save");
    assert_eq!(after.revision, 2);
    assert_eq!(after.status, DraftStatus::Active);
    assert_eq!(after.attempts.len(), 1);
    assert!(after.attempts[0].events.is_empty());
    store.assert_revision_current(&second).await.unwrap();
    // An attempt this draft never had is still refused.
    let mut unknown = late.clone();
    unknown.attempt = "never-claimed".into();
    assert!(
        store
            .record_revision(
                &unknown,
                event(
                    &unknown,
                    1,
                    EventKind::ProviderFailed { reason: "x".into() }
                )
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
    open_draft(&store, &scope, "Original").await;
    // Keys sort opposite to acceptance order.
    let keys = ["z-first", "a-second", "m-third", "b-fourth"];
    for (index, key) in keys.iter().enumerate() {
        let revision = index as u64 + 1;
        accept_revision_input(&store, &scope, &input(key, revision))
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
            .map(|earlier| earlier.accepted.input.key.as_str())
            .collect::<Vec<_>>(),
        keys[..3]
    );
    // Another draft of the same physician numbers its history separately.
    let mut other = scope.clone();
    other.chat = "other-reply".into();
    open_draft(&store, &other, "Other").await;
    for (index, key) in ["other-1", "other-2"].iter().enumerate() {
        let revision = index as u64 + 1;
        accept_revision_input(&store, &other, &input(key, revision))
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
        accept_revision_input(&store, &scope, &input("kept", 2))
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
        reopened
            .accepted
            .as_ref()
            .map(|accepted| accepted.input.key.as_str()),
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
