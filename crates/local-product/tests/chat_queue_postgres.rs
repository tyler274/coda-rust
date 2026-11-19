//! Real Postgres proof for queued follow-up messages (`Send next`), inline
//! queue edits/withdraw/send-now, cutoff/failure preservation, and Stop queue
//! return. Run explicitly with `PHYSICIAN_RUNTIME_TEST_DATABASE_URL`.
use std::collections::VecDeque;

use engine::recipe::{Observer, Request};
use local_product::runtime::delivery::Backoff;
use local_product::runtime::{
    Accepted, Authority, Input, PhysicianHost, PublicationTransaction, QueueMutation, Scope,
    Stopped, Store, WorkStatus, async_trait, engine,
};
use physician_wire::SubmitDelivery;
use serde_json::json;

#[path = "support/finish.rs"]
mod finish;

fn scope() -> Scope {
    Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician-a".into(),
        workspace: "patient-a".into(),
        chat: "chat-queue-test".into(),
    }
}

async fn store() -> Store {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .expect("explicit local Postgres URL required");
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    store
}

fn skills() -> local_product::physician_chat::PhysicianSkills {
    local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap()
}

fn recipe() -> engine::recipe::RequestRecipe {
    let binding = local_product::ModelBinding {
        model: "controlled".into(),
        effort: "none".into(),
        adapter: "fixed".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 272_000,
    };
    let skills = skills();
    local_product::physician_chat::recipe(&binding, "Owner", &skills)
}

#[derive(Clone, Copy)]
struct Allowed;

#[async_trait]
impl Authority for Allowed {
    async fn lock_publication(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn check(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
}

struct ScriptedProvider {
    replies: VecDeque<Result<engine::Turn, engine::Cause>>,
    calls: usize,
}

impl engine::recipe::Provider for ScriptedProvider {
    async fn exchange(
        &mut self,
        _request: &Request,
        _cancel: &engine::Cancellation,
        _limits: &engine::RunLimits,
        _observations: &mut dyn Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        self.calls += 1;
        self.replies.pop_front().expect("scripted reply")
    }
}

fn answer(text: &str) -> engine::Turn {
    engine::Turn {
        items: finish::answered(text),
        receipt: json!({"ok": true}),
        usage: Default::default(),
    }
}

fn input(key: &str, text: &str) -> Input {
    Input {
        key: key.into(),
        text: text.into(),
        timezone: "America/Chicago".into(),
        scenario: None,
    }
}

async fn accept_delivery(
    store: &Store,
    scope: &Scope,
    inp: &Input,
    delivery: SubmitDelivery,
) -> Accepted {
    store
        .accept_with_delivery_authorized(
            scope,
            inp,
            vec![0; 32],
            delivery,
            &skills(),
            &recipe(),
            None,
        )
        .await
        .unwrap()
}

async fn run_one(
    store: &Store,
    scope: &Scope,
    provider: &mut ScriptedProvider,
) -> Result<(), engine::Failure> {
    let claim = store.claim(scope, 60).await.unwrap().expect("claim");
    let cancel = engine::Cancellation::default();
    let res = PhysicianHost {
        store: store.clone(),
        claim: claim.clone(),
        authority: Allowed,
        shown_steps: local_product::runtime::ShownSteps::new(()),
    }
    .run(provider, engine::RunLimits::default(), &cancel)
    .await;
    if let Err(ref error) = res {
        store
            .settle_chat_attempt(
                &claim,
                engine::AttemptFailure::of(error, &cancel),
                Backoff::IMMEDIATE,
            )
            .await
            .unwrap();
    }
    res
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn send_next_queues_while_working_and_drains_into_combined_turn_on_finish() {
    let s = store().await;
    let sc = scope();
    let mut provider = ScriptedProvider {
        replies: VecDeque::from([Ok(answer("First answer.")), Ok(answer("Combined answer."))]),
        calls: 0,
    };

    accept_delivery(&s, &sc, &input("k1", "First ask"), SubmitDelivery::SendNow).await;
    accept_delivery(
        &s,
        &sc,
        &input("k2", "Second ask"),
        SubmitDelivery::SendNext,
    )
    .await;
    accept_delivery(&s, &sc, &input("k3", "Third ask"), SubmitDelivery::SendNext).await;

    let before_finish = s.conversation(&sc).await.unwrap();
    assert_eq!(before_finish.status, WorkStatus::Pending);
    assert_eq!(before_finish.messages.len(), 1);
    assert_eq!(before_finish.queued_messages().len(), 2);
    assert_eq!(before_finish.queued_messages()[0].text, "Second ask");
    assert_eq!(before_finish.queued_messages()[1].text, "Third ask");

    // Turn 1 finishes -> queued messages drain atomically into 1 combined follow-up turn.
    run_one(&s, &sc, &mut provider).await.unwrap();
    let mid = s.conversation(&sc).await.unwrap();
    assert_eq!(mid.status, WorkStatus::Pending);
    assert!(mid.queued_messages().is_empty());
    let mid_transcript = mid.transcript(|_| None).unwrap();
    assert_eq!(mid_transcript.len(), 2);
    assert_eq!(
        mid_transcript[1]
            .messages
            .iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>(),
        vec!["Second ask", "Third ask"]
    );

    run_one(&s, &sc, &mut provider).await.unwrap();
    let done = s.conversation(&sc).await.unwrap();
    assert_eq!(done.status, WorkStatus::Completed);
    assert_eq!(provider.calls, 2);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn queue_edit_withdraw_and_send_now_preserve_order_and_idempotency() {
    let s = store().await;
    let sc = scope();
    let mut provider = ScriptedProvider {
        replies: VecDeque::from([Ok(answer("Steered answer.")), Ok(answer("Final queued."))]),
        calls: 0,
    };

    accept_delivery(
        &s,
        &sc,
        &input("k1", "Working turn"),
        SubmitDelivery::SendNow,
    )
    .await;
    accept_delivery(
        &s,
        &sc,
        &input("k2", "Promote me"),
        SubmitDelivery::SendNext,
    )
    .await;
    accept_delivery(&s, &sc, &input("k3", "Edit me"), SubmitDelivery::SendNext).await;
    accept_delivery(&s, &sc, &input("k4", "Delete me"), SubmitDelivery::SendNext).await;

    let convo = s.conversation(&sc).await.unwrap();
    let q = convo.queued_messages();
    assert_eq!(q.len(), 3);

    // Edit middle queued message idempotently.
    assert_eq!(
        s.edit_queued_message_authorized(&sc, "edit-1", &q[1].id.0, "Edited queued")
            .await
            .unwrap(),
        QueueMutation::Applied
    );
    assert_eq!(
        s.edit_queued_message_authorized(&sc, "edit-1", &q[1].id.0, "Edited queued")
            .await
            .unwrap(),
        QueueMutation::Duplicate
    );
    assert_eq!(
        s.edit_queued_message_authorized(&sc, "edit-missing", "no-such-id", "Nope")
            .await
            .unwrap(),
        QueueMutation::Refused
    );
    // Retrying a refused key must remain Refused (receipt never committed).
    assert_eq!(
        s.edit_queued_message_authorized(&sc, "edit-missing", "no-such-id", "Nope")
            .await
            .unwrap(),
        QueueMutation::Refused
    );
    assert_eq!(
        s.withdraw_queued_message_authorized(&sc, "wd-missing", "no-such-id")
            .await
            .unwrap(),
        QueueMutation::Refused
    );
    assert_eq!(
        s.withdraw_queued_message_authorized(&sc, "wd-missing", "no-such-id")
            .await
            .unwrap(),
        QueueMutation::Refused
    );
    assert_eq!(
        s.send_queued_now_authorized(
            &sc,
            "promote-missing",
            vec![0; 32],
            "no-such-id",
            "America/Chicago",
            &skills(),
            &recipe(),
            None,
        )
        .await
        .unwrap(),
        QueueMutation::Refused
    );
    assert_eq!(
        s.send_queued_now_authorized(
            &sc,
            "promote-missing",
            vec![0; 32],
            "no-such-id",
            "America/Chicago",
            &skills(),
            &recipe(),
            None,
        )
        .await
        .unwrap(),
        QueueMutation::Refused
    );

    // Withdraw third queued message idempotently.
    assert_eq!(
        s.withdraw_queued_message_authorized(&sc, "wd-1", &q[2].id.0)
            .await
            .unwrap(),
        QueueMutation::Applied
    );
    assert_eq!(
        s.withdraw_queued_message_authorized(&sc, "wd-1", &q[2].id.0)
            .await
            .unwrap(),
        QueueMutation::Duplicate
    );

    // Promote first queued message via Send now while remaining stays queued.
    assert_eq!(
        s.send_queued_now_authorized(
            &sc,
            "promote-1",
            vec![0; 32],
            &q[0].id.0,
            "America/Chicago",
            &skills(),
            &recipe(),
            None,
        )
        .await
        .unwrap(),
        QueueMutation::Applied
    );

    let after_promote = s.conversation(&sc).await.unwrap();
    assert_eq!(after_promote.queued_messages().len(), 1);
    assert_eq!(after_promote.queued_messages()[0].text, "Edited queued");

    run_one(&s, &sc, &mut provider).await.unwrap();
    run_one(&s, &sc, &mut provider).await.unwrap();
    let final_convo = s.conversation(&sc).await.unwrap();
    assert_eq!(final_convo.status, WorkStatus::Completed);
    let transcript = final_convo.transcript(|_| None).unwrap();
    assert_eq!(transcript.len(), 2);
    assert_eq!(
        transcript[0]
            .messages
            .iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>(),
        vec!["Working turn", "Promote me"]
    );
    assert_eq!(
        transcript[1]
            .messages
            .iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>(),
        vec!["Edited queued"]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn failure_keeps_queue_until_try_again_succeeds() {
    let s = store().await;
    let sc = scope();
    let mut provider = ScriptedProvider {
        replies: VecDeque::from([
            Err(engine::Cause::from("temporary provider glitch")),
            Ok(answer("Recovered.")),
            Ok(answer("Queued ran after recovery.")),
        ]),
        calls: 0,
    };

    accept_delivery(&s, &sc, &input("k1", "First"), SubmitDelivery::SendNow).await;
    accept_delivery(
        &s,
        &sc,
        &input("k2", "Wait for retry"),
        SubmitDelivery::SendNext,
    )
    .await;

    let _ = run_one(&s, &sc, &mut provider).await;
    let during_retry = s.conversation(&sc).await.unwrap();
    assert_eq!(during_retry.queued_messages().len(), 1);
    assert_eq!(during_retry.queued_messages()[0].text, "Wait for retry");

    // Finish old turn first -> queued turn starts afterwards.
    run_one(&s, &sc, &mut provider).await.unwrap();
    let after_first = s.conversation(&sc).await.unwrap();
    assert!(after_first.queued_messages().is_empty());
    assert_eq!(after_first.status, WorkStatus::Pending);

    run_one(&s, &sc, &mut provider).await.unwrap();
    let done = s.conversation(&sc).await.unwrap();
    assert_eq!(done.status, WorkStatus::Completed);
    let transcript = done.transcript(|_| None).unwrap();
    assert_eq!(transcript.len(), 2);
    assert_eq!(transcript[1].messages[0].text, "Wait for retry");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn stop_drains_queue_without_sending_even_on_duplicate_stop() {
    let s = store().await;
    let sc = scope();

    accept_delivery(
        &s,
        &sc,
        &input("k1", "Running turn"),
        SubmitDelivery::SendNow,
    )
    .await;
    accept_delivery(&s, &sc, &input("k2", "Queued 1"), SubmitDelivery::SendNext).await;
    accept_delivery(&s, &sc, &input("k3", "Queued 2"), SubmitDelivery::SendNext).await;

    let running = s.conversation(&sc).await.unwrap();
    let work_id = running.work.clone();

    let first_stop = s
        .stop_chat_authorized(&sc, "stop-key-1", vec![0; 32], &work_id)
        .await
        .unwrap();
    assert_eq!(first_stop.outcome, Stopped::Held);
    assert_eq!(first_stop.returned_messages, vec!["Queued 1", "Queued 2"]);

    let held = s.conversation(&sc).await.unwrap();
    assert_eq!(held.status, WorkStatus::Held);
    assert!(held.queued_messages().is_empty());

    // Duplicate Stop returns the exact same held queue so client recovery always gets it back.
    let second_stop = s
        .stop_chat_authorized(&sc, "stop-key-1", vec![0; 32], &work_id)
        .await
        .unwrap();
    assert_eq!(second_stop.outcome, Stopped::Duplicate);
    assert_eq!(second_stop.returned_messages, vec!["Queued 1", "Queued 2"]);

    // New Stop key against already-held work also returns the stopped queue.
    let third_stop = s
        .stop_chat_authorized(&sc, "stop-key-2", vec![0; 32], &work_id)
        .await
        .unwrap();
    assert_eq!(third_stop.outcome, Stopped::Duplicate);
    assert_eq!(third_stop.returned_messages, vec!["Queued 1", "Queued 2"]);
}
