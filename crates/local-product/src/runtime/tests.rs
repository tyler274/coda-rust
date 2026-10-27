#[path = "tests/reply.rs"]
mod reply_tests;

use super::delivery::{Delivery, DeliveryState};
use super::*;

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn concurrent_commands_and_claims_serialize_on_the_composite_conversation_key() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "chat".into(),
    };
    let first = Input {
        key: "first".into(),
        text: "first".into(),
        timezone: "UTC".into(),
    };
    let second = Input {
        key: "second".into(),
        text: "second".into(),
        timezone: "UTC".into(),
    };
    let (a, b) = tokio::join!(store.accept(&scope, &first), store.accept(&scope, &second));
    assert_eq!(a.unwrap(), Accepted::New);
    assert_eq!(b.unwrap(), Accepted::New);
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.revision, 2);
    assert_eq!(state.messages.len(), 2);
    let (a, b) = tokio::join!(store.claim(&scope, 60), store.claim(&scope, 60));
    assert_eq!(
        [a.unwrap(), b.unwrap()]
            .iter()
            .filter(|claim| claim.is_some())
            .count(),
        1
    );
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.attempts.len(), 1);
    assert_eq!(state.attempts[0].revision, 2);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn pending_keyset_orders_all_scope_keys_and_filters_terminal_and_live_work() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let mut expected = Vec::new();
    // Insert out of order; the cursor must compare every composite-key component.
    for (physician, workspace, chat, status, expiry) in [
        ("b", "a", "a", "pending", None),
        ("a", "b", "a", "active", Some(0)),
        ("a", "a", "b", "pending", None),
        ("a", "a", "a", "pending", None),
        ("a", "a", "held", "held", None),
        ("a", "a", "failed", "failed", None),
        ("a", "a", "done", "completed", None),
        ("a", "a", "live", "active", Some(i64::MAX)),
        ("a", "a", "empty", "active", None),
    ] {
        let scope = Scope {
            practice: practice.clone(),
            physician: physician.into(),
            workspace: workspace.into(),
            chat: chat.into(),
        };
        let mut state = Conversation {
            status: status.into(),
            ..Default::default()
        };
        if let Some(expires) = expiry {
            // Only the final attempt's expiry determines eligibility.
            state.attempts = vec![
                Attempt {
                    id: "old".into(),
                    predecessor: None,
                    work: "work".into(),
                    revision: 1,
                    status: "interrupted".into(),
                    expires: i64::MAX,
                    events: vec![],
                },
                Attempt {
                    id: "latest".into(),
                    predecessor: Some("old".into()),
                    work: "work".into(),
                    revision: 1,
                    status: "active".into(),
                    expires,
                    events: vec![],
                },
            ];
        }
        conversation::Entity::insert(conversation_model(
            &scope,
            serde_json::to_value(state).unwrap(),
        ))
        .exec(&store.db)
        .await
        .unwrap();
        if status == "pending" || expiry == Some(0) {
            expected.push(scope);
        }
    }
    expected.sort_by_key(keys);
    let start = Scope {
        practice,
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    assert!(
        store
            .pending_batch(0, Some(&start))
            .await
            .unwrap()
            .is_empty()
    );
    let first = store.pending_batch(2, Some(&start)).await.unwrap();
    assert_eq!(first, expected[..2]);
    let second = store.pending_batch(2, first.last()).await.unwrap();
    assert_eq!(second, expected[2..]);
    let rest = store.pending_batch(100, second.last()).await.unwrap();
    assert!(rest.iter().all(|scope| scope.practice != start.practice));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn corrupt_conversation_rolls_back_receipt_and_conflicts_never_overwrite_it() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "chat".into(),
    };
    let input = Input {
        key: "receipt".into(),
        text: "original".into(),
        timezone: "UTC".into(),
    };
    conversation::Entity::insert(conversation_model(&scope, serde_json::json!({})))
        .exec(&store.db)
        .await
        .unwrap();
    assert!(store.accept(&scope, &input).await.is_err());
    assert!(
        command::Entity::find_by_id((
            scope.practice.clone(),
            scope.physician.clone(),
            input.key.clone()
        ))
        .one(&store.db)
        .await
        .unwrap()
        .is_none()
    );
    conversation_model(
        &scope,
        serde_json::to_value(Conversation::default()).unwrap(),
    )
    .update(&store.db)
    .await
    .unwrap();
    assert_eq!(store.accept(&scope, &input).await.unwrap(), Accepted::New);
    let mut other_chat = scope.clone();
    other_chat.chat = "different-chat".into();
    assert!(store.accept(&other_chat, &input).await.is_err());
    assert!(store.conversation(&other_chat).await.is_err());
    let mut collision = input.clone();
    collision.timezone = "America/New_York".into();
    assert!(store.accept(&scope, &collision).await.is_err());
    assert_eq!(
        store.accept(&scope, &input).await.unwrap(),
        Accepted::Duplicate
    );
    assert_eq!(store.conversation(&scope).await.unwrap().revision, 1);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn expired_attempt_recovery_preserves_accepted_work_and_evidence() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let s = Store::connect(&url).await.unwrap();
    s.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "chat".into(),
    };
    let input = Input {
        key: "command".into(),
        text: "accepted work".into(),
        timezone: "UTC".into(),
    };
    s.accept(&scope, &input).await.unwrap();
    let first = s.claim(&scope, 60).await.unwrap().unwrap();
    s.persist(
        &first,
        engine::Event {
            attempt_id: first.attempt.clone(),
            sequence: 1,
            kind: engine::EventKind::ProviderStarted {
                ordinal: 1,
                context: engine::Context {
                    instructions: "test".into(),
                    items: vec![],
                    tools: vec![],
                },
            },
        },
    )
    .await
    .unwrap();
    s.assert_current(&first).await.unwrap();
    // Establish expiry only after real evidence is durable. A one-second lease
    // can expire before persist() on a busy runner because the DB clock floors
    // to seconds. Keep fixture control at the storage owner, with no API seam.
    let (tx, mut state, _) = s.locked(&scope).await.unwrap();
    state.attempts.last_mut().unwrap().expires = 0;
    Store::save(tx, &scope, &state).await.unwrap();
    drop(s);
    let s = Store::connect(&url).await.unwrap();
    // Read from this test's practice onward: other tests leave pending chats
    // in the shared database, and the first page in key order may be theirs.
    let from = Scope {
        practice: scope.practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    assert!(
        s.pending_batch(100, Some(&from))
            .await
            .unwrap()
            .contains(&scope)
    );
    let next = s.claim(&scope, 60).await.unwrap().unwrap();
    assert_ne!(first.attempt, next.attempt);
    assert!(s.assert_current(&first).await.is_err());
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(state.attempts[0].status, "interrupted");
    assert_eq!(state.attempts[0].events.len(), 1);
    assert_eq!(
        state.attempts[1].predecessor.as_deref(),
        Some(first.attempt.as_str())
    );
    assert_eq!(state.attempts[1].work, state.attempts[0].work);
    assert_eq!(state.messages[0].text, input.text);
    assert_eq!(state.messages.len(), 1);
}

fn delivery_scope(practice: &str, chat: &str) -> Scope {
    Scope {
        practice: practice.into(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: chat.into(),
    }
}

fn delivery_input(key: &str) -> Input {
    Input {
        key: key.into(),
        text: key.into(),
        timezone: "UTC".into(),
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn queue_delivery_is_fenced_to_its_input_and_recovers_through_typed_owners() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let scope = delivery_scope(&practice, "chat");
    let unauthorized = delivery_scope(&practice, "local-only");
    store
        .accept_authorized(&scope, &delivery_input("first"), vec![7; 32])
        .await
        .unwrap();
    // Local (session-less) work never becomes a hosted queue wakeup.
    store
        .accept(&unauthorized, &delivery_input("local"))
        .await
        .unwrap();
    let ours = |page: &delivery::DispatchPage| -> Vec<Delivery> {
        page.deliveries
            .iter()
            .filter(|d| d.scope.practice == practice)
            .cloned()
            .collect()
    };
    let first = Delivery::chat_turn(scope.clone(), "first".into());
    let page = store.dispatch_due(100, Some(&start)).await.unwrap();
    assert_eq!(ours(&page), vec![first.clone()]);
    // Successful publication suppresses republish until the retry hint passes.
    store.dispatched(&first, 600).await.unwrap();
    let page = store.dispatch_due(100, Some(&start)).await.unwrap();
    assert!(ours(&page).is_empty());
    assert_eq!(
        store.delivery_state(&first).await.unwrap(),
        DeliveryState::Due
    );

    // A newer accepted input supersedes the old wakeup: it can never claim,
    // and acknowledging it is safe because the newer input is recoverable.
    store
        .accept_authorized(&scope, &delivery_input("second"), vec![7; 32])
        .await
        .unwrap();
    assert!(store.claim_delivery(&first, 60).await.unwrap().is_none());
    assert_eq!(
        store.delivery_state(&first).await.unwrap(),
        DeliveryState::Settled
    );
    let second = Delivery::chat_turn(scope.clone(), "second".into());
    assert_eq!(
        ours(&store.dispatch_due(100, Some(&start)).await.unwrap()),
        vec![second.clone()]
    );

    // Duplicate wakeups find no second live claim.
    let claim = store.claim_delivery(&second, 60).await.unwrap().unwrap();
    assert!(store.claim_delivery(&second, 60).await.unwrap().is_none());
    store.heartbeat(&claim, 120).await.unwrap();
    assert_eq!(
        store.delivery_state(&second).await.unwrap(),
        DeliveryState::Claimed
    );

    // A failed run is released for broker redelivery as a successor attempt.
    store.retry_delivery(&claim).await.unwrap();
    assert_eq!(store.conversation(&scope).await.unwrap().status, "pending");
    assert_eq!(
        store.delivery_state(&second).await.unwrap(),
        DeliveryState::Due
    );
    assert!(store.heartbeat(&claim, 60).await.is_err());
    let retry = store.claim_delivery(&second, 60).await.unwrap().unwrap();
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(
        state.attempts.last().unwrap().predecessor.as_deref(),
        Some(claim.attempt.as_str())
    );

    // Intentional Stop settles the delivery and cannot be reopened by retry.
    store.stop(&scope).await.unwrap();
    store.retry_delivery(&retry).await.unwrap();
    assert_eq!(store.conversation(&scope).await.unwrap().status, "held");
    assert_eq!(
        store.delivery_state(&second).await.unwrap(),
        DeliveryState::Settled
    );
    assert!(ours(&store.dispatch_due(100, Some(&start)).await.unwrap()).is_empty());
    assert!(store.dispatch_due(0, None).await.is_err());
    assert!(store.dispatched(&second, 0).await.is_err());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn repeated_failures_end_the_input_as_failed_until_resume_or_new_input() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let ours = |page: delivery::DispatchPage| -> Vec<Delivery> {
        page.deliveries
            .into_iter()
            .filter(|d| d.scope.practice == practice)
            .collect()
    };
    let scope = delivery_scope(&practice, "chat");
    store
        .accept_authorized(&scope, &delivery_input("turn"), vec![5; 32])
        .await
        .unwrap();
    let turn = Delivery::chat_turn(scope.clone(), "turn".into());
    for failure in 1..MAX_FAILED_ATTEMPTS {
        let claim = store.claim_delivery(&turn, 60).await.unwrap().unwrap();
        store.retry_delivery(&claim).await.unwrap();
        let state = store.conversation(&scope).await.unwrap();
        assert_eq!(state.status, "pending", "failure {failure} still retries");
        assert_eq!(state.failures, failure);
    }
    // The last allowed failure ends the work; recovery has given up.
    let claim = store.claim_delivery(&turn, 60).await.unwrap().unwrap();
    store.retry_delivery(&claim).await.unwrap();
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.status, "failed");
    assert_eq!(state.failures, MAX_FAILED_ATTEMPTS);
    assert!(store.claim_delivery(&turn, 60).await.unwrap().is_none());
    assert!(
        ours(store.dispatch_due(100, Some(&start)).await.unwrap()).is_empty(),
        "the recovery sweep must not republish work that recovery gave up on"
    );
    // Failed is neither settled nor runnable: the worker refuses it so the
    // broker dead-letters the message instead of acknowledging it.
    assert_eq!(
        store.delivery_state(&turn).await.unwrap(),
        DeliveryState::Failed
    );

    // Resume is the physician's Try again: a fresh budget for the same input.
    store.resume(&scope).await.unwrap();
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!((state.status.as_str(), state.failures), ("pending", 0));
    assert_eq!(
        ours(store.dispatch_due(100, Some(&start)).await.unwrap()),
        vec![turn.clone()]
    );

    // A worker that dies holding the lease counts as a failure as well: each
    // later claim records the expired lease before starting a successor.
    for _ in 0..MAX_FAILED_ATTEMPTS {
        store.claim_delivery(&turn, 60).await.unwrap().unwrap();
        let (tx, mut state, _) = store.locked(&scope).await.unwrap();
        state.attempts.last_mut().unwrap().expires = 0;
        Store::save(tx, &scope, &state).await.unwrap();
    }
    assert!(store.claim_delivery(&turn, 60).await.unwrap().is_none());
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.status, "failed");
    assert_eq!(state.failures, MAX_FAILED_ATTEMPTS);
    assert_eq!(
        store.delivery_state(&turn).await.unwrap(),
        DeliveryState::Failed
    );

    // New input starts its own budget.
    store
        .accept_authorized(&scope, &delivery_input("next"), vec![5; 32])
        .await
        .unwrap();
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!((state.status.as_str(), state.failures), ("pending", 0));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn try_again_resumes_only_the_named_failed_run_and_never_completed_work() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let ours = |page: delivery::DispatchPage| -> Vec<Delivery> {
        page.deliveries
            .into_iter()
            .filter(|d| d.scope.practice == practice)
            .collect()
    };
    let scope = delivery_scope(&practice, "chat");
    store
        .accept_authorized(&scope, &delivery_input("turn"), vec![5; 32])
        .await
        .unwrap();
    let turn = Delivery::chat_turn(scope.clone(), "turn".into());
    // Pending work is not failed: nothing to try again.
    let pending = store.conversation(&scope).await.unwrap();
    assert_eq!(
        store
            .resume_authorized(&scope, "early", vec![6; 32], &pending.work, "none")
            .await
            .unwrap(),
        Resumed::Refused
    );
    for _ in 0..MAX_FAILED_ATTEMPTS {
        let claim = store.claim_delivery(&turn, 60).await.unwrap().unwrap();
        store.retry_delivery(&claim).await.unwrap();
    }
    let failed = store.conversation(&scope).await.unwrap();
    assert_eq!(failed.status, "failed");
    let work = failed.work.clone();
    let run = failed.attempts.last().unwrap().id.clone();
    let earlier = failed.attempts.first().unwrap().id.clone();

    // A wrong run, wrong work, or another chat is refused and changes nothing.
    for (key, target, work_id, run_id) in [
        ("wrong-run", &scope, work.as_str(), earlier.as_str()),
        ("wrong-work", &scope, "other-work", run.as_str()),
        (
            "wrong-chat",
            &delivery_scope(&practice, "other"),
            work.as_str(),
            run.as_str(),
        ),
    ] {
        assert_eq!(
            store
                .resume_authorized(target, key, vec![6; 32], work_id, run_id)
                .await
                .unwrap(),
            Resumed::Refused,
            "{key}"
        );
    }
    let unchanged = store.conversation(&scope).await.unwrap();
    assert_eq!(
        (unchanged.status.as_str(), unchanged.failures),
        ("failed", MAX_FAILED_ATTEMPTS)
    );
    assert!(
        store
            .resume_authorized(&scope, "short", vec![6; 4], &work, &run)
            .await
            .is_err()
    );

    // The named failed run resumes with a fresh budget and the current session.
    assert_eq!(
        store
            .resume_authorized(&scope, "again", vec![6; 32], &work, &run)
            .await
            .unwrap(),
        Resumed::Pending {
            input_id: "turn".into()
        }
    );
    let resumed = store.conversation(&scope).await.unwrap();
    assert_eq!((resumed.status.as_str(), resumed.failures), ("pending", 0));
    assert_eq!(resumed.session_hash.as_deref(), Some(&[6u8; 32][..]));
    assert_eq!(resumed.work, work, "Try again keeps the work identity");
    assert_eq!(
        ours(store.dispatch_due(100, Some(&start)).await.unwrap()),
        vec![turn.clone()]
    );
    // The same request is a duplicate; the same key for another target is not.
    assert_eq!(
        store
            .resume_authorized(&scope, "again", vec![6; 32], &work, &run)
            .await
            .unwrap(),
        Resumed::Duplicate
    );
    assert_eq!(
        store
            .resume_authorized(&scope, "again", vec![6; 32], &work, &earlier)
            .await
            .unwrap(),
        Resumed::Refused
    );

    // Completed work is never reopened, even with its last run named.
    let claim = store.claim_delivery(&turn, 60).await.unwrap().unwrap();
    let (tx, mut state, _) = store.locked(&scope).await.unwrap();
    state.attempts.last_mut().unwrap().status = "completed".into();
    state.status = "completed".into();
    Store::save(tx, &scope, &state).await.unwrap();
    assert_eq!(
        store
            .resume_authorized(&scope, "after-done", vec![6; 32], &work, &claim.attempt)
            .await
            .unwrap(),
        Resumed::Refused
    );
    assert_eq!(
        store.conversation(&scope).await.unwrap().status,
        "completed"
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn expired_worker_lease_is_redispatched_by_the_database_clock() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let scope = delivery_scope(&practice, "chat");
    store
        .accept_authorized(&scope, &delivery_input("turn"), vec![9; 32])
        .await
        .unwrap();
    let delivery = Delivery::chat_turn(scope.clone(), "turn".into());
    let claim = store.claim_delivery(&delivery, 60).await.unwrap().unwrap();
    store.dispatched(&delivery, 60).await.unwrap();
    // Expire lease and dispatch hint at the storage owner instead of sleeping
    // across a floored database-clock second (flaky on busy runners).
    let (tx, mut state, _) = store.locked(&scope).await.unwrap();
    state.attempts.last_mut().unwrap().expires = 0;
    state.dispatch_after = 0;
    Store::save(tx, &scope, &state).await.unwrap();
    let page = store.dispatch_due(100, Some(&start)).await.unwrap();
    assert!(page.deliveries.contains(&delivery));
    assert!(store.heartbeat(&claim, 60).await.is_err());
    let successor = store.claim_delivery(&delivery, 60).await.unwrap().unwrap();
    assert_ne!(successor.attempt, claim.attempt);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn chats_list_one_physicians_workspace_by_latest_activity() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = |physician: &str, workspace: &str, chat: &str| Scope {
        practice: practice.clone(),
        physician: physician.into(),
        workspace: workspace.into(),
        chat: chat.into(),
    };
    let input = |key: &str, text: &str| Input {
        key: key.into(),
        text: text.into(),
        timezone: "UTC".into(),
    };
    for (key, s, text) in [
        ("a1", scope("me", "ws", "chat-a"), "About labs"),
        ("b1", scope("me", "ws", "chat-b"), "About meds"),
        ("a2", scope("me", "ws", "chat-a"), "And the follow-up"),
        ("x1", scope("other", "ws", "chat-x"), "Not mine"),
        ("y1", scope("me", "elsewhere", "chat-y"), "Other patient"),
    ] {
        assert_eq!(
            store.accept(&s, &input(key, text)).await.unwrap(),
            Accepted::New
        );
    }
    // Same-second accepts tie on the clock; move chat-a's activity forward so
    // the order is decided by activity, not the id tie-break.
    let (tx, mut state, now) = store.locked(&scope("me", "ws", "chat-a")).await.unwrap();
    assert!(state.opened_at > 0 && state.opened_at <= now);
    state.last_activity_at = now + 100;
    Store::save(tx, &scope("me", "ws", "chat-a"), &state)
        .await
        .unwrap();

    let chats = store.chats(&practice, "me", "ws").await.unwrap();
    let listed: Vec<_> = chats
        .iter()
        .map(|c| (c.chat.as_str(), c.first_message.as_str()))
        .collect();
    assert_eq!(listed, [("chat-a", "About labs"), ("chat-b", "About meds")]);
    assert!(chats.iter().all(|c| c.opened_at > 0));

    // Equal activity falls back to chat id descending.
    let (tx, mut state, _) = store.locked(&scope("me", "ws", "chat-b")).await.unwrap();
    state.last_activity_at = chats[0].last_activity_at;
    Store::save(tx, &scope("me", "ws", "chat-b"), &state)
        .await
        .unwrap();
    let ids: Vec<_> = store
        .chats(&practice, "me", "ws")
        .await
        .unwrap()
        .into_iter()
        .map(|c| c.chat)
        .collect();
    assert_eq!(ids, ["chat-b", "chat-a"]);

    assert!(
        store
            .chats(&practice, "nobody", "ws")
            .await
            .unwrap()
            .is_empty()
    );
    assert!(store.chats(&practice, "", "ws").await.is_err());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn chat_list_orders_by_activity_before_its_limit() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = |physician: &str, chat: String| Scope {
        practice: practice.clone(),
        physician: physician.into(),
        workspace: "ws".into(),
        chat,
    };
    let input = |key: String| Input {
        key,
        text: "Hello".into(),
        timezone: "UTC".into(),
    };
    // More chats than the list holds. Chat-id order alone would list
    // chat-005..chat-204 and leave chat-000 out.
    let total = MAX_LISTED_CHATS as usize + 5;
    for n in 0..total {
        store
            .accept(
                &scope("me", format!("chat-{n:03}")),
                &input(format!("me-{n}")),
            )
            .await
            .unwrap();
    }
    // Another physician's newer chats in the same workspace never list.
    for n in 0..3 {
        let theirs = scope("other", format!("chat-zzz-{n}"));
        store
            .accept(&theirs, &input(format!("other-{n}")))
            .await
            .unwrap();
        let (tx, mut state, now) = store.locked(&theirs).await.unwrap();
        state.last_activity_at = now + 1000;
        Store::save(tx, &theirs, &state).await.unwrap();
    }
    let before = store.chats(&practice, "me", "ws").await.unwrap();
    assert_eq!(before.len(), MAX_LISTED_CHATS as usize);
    assert!(
        before
            .iter()
            .all(|c| str::starts_with(&c.chat, "chat-") && !str::contains(&c.chat, "zzz"))
    );

    // A new message in a later database second makes chat-000 the newest.
    let oldest = scope("me", "chat-000".into());
    let newest_before = before[0].last_activity_at;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let (tx, _, now) = store.locked(&oldest).await.unwrap();
        tx.commit().await.unwrap();
        if now > newest_before {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "database clock stalled"
        );
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    assert_eq!(
        store
            .accept(&oldest, &input("me-again".into()))
            .await
            .unwrap(),
        Accepted::New
    );
    let after = store.chats(&practice, "me", "ws").await.unwrap();
    assert_eq!(after.len(), MAX_LISTED_CHATS as usize);
    assert_eq!(after[0].chat, "chat-000", "the active chat must list first");
    assert!(after[0].last_activity_at > newest_before);
    assert!(
        after
            .windows(2)
            .all(|pair| (pair[0].last_activity_at, &pair[0].chat)
                >= (pair[1].last_activity_at, &pair[1].chat)),
        "newest activity first, chat id descending on ties"
    );
    assert!(after.iter().all(|c| !str::contains(&c.chat, "zzz")));

    // Settle every chat: the restart sweeps in other tests share this
    // database and read pending work from the start of the key order.
    let chats = (0..total)
        .map(|n| scope("me", format!("chat-{n:03}")))
        .chain((0..3).map(|n| scope("other", format!("chat-zzz-{n}"))));
    for chat in chats {
        let (tx, mut state, _) = store.locked(&chat).await.unwrap();
        state.status = "completed".into();
        Store::save(tx, &chat, &state).await.unwrap();
    }
    assert!(
        store
            .pending_batch(100, Some(&scope("", String::new())))
            .await
            .unwrap()
            .iter()
            .all(|s| s.practice != practice)
    );
}

/// Allows or refuses every publication, without the auth owner's tables.
struct Gate(bool);
#[async_trait]
impl Authority for Gate {
    async fn check(&mut self, _: &Scope) -> Result<(), String> {
        Ok(())
    }
    async fn lock_publication(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        if self.0 && hash.is_some() {
            Ok(())
        } else {
            Err("physician authority changed".into())
        }
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn chat_drafts_save_under_a_revision_fence_for_one_physician() {
    use chat_draft::{ChatDraft, ChatDraftSaved};
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "draft".into(),
    };
    let hash = [7_u8; 32];
    let draft = |revision, text: &str| ChatDraft {
        revision,
        text: text.into(),
    };
    assert_eq!(store.chat_draft(&scope).await.unwrap(), draft(0, ""));

    // A refused authority writes nothing, even the first save.
    assert!(
        store
            .save_chat_draft(&scope, 0, "refused", &hash, &mut Gate(false))
            .await
            .is_err()
    );
    assert_eq!(store.chat_draft(&scope).await.unwrap(), draft(0, ""));
    assert!(
        store
            .save_chat_draft(&scope, 0, "short hash", &[1; 4], &mut Gate(true))
            .await
            .is_err()
    );

    let saved = |d| ChatDraftSaved::Saved(d);
    assert_eq!(
        store
            .save_chat_draft(&scope, 0, "first", &hash, &mut Gate(true))
            .await
            .unwrap(),
        saved(draft(1, "first"))
    );
    assert_eq!(store.chat_draft(&scope).await.unwrap(), draft(1, "first"));
    assert_eq!(
        store
            .save_chat_draft(&scope, 1, "second", &hash, &mut Gate(true))
            .await
            .unwrap(),
        saved(draft(2, "second"))
    );
    // Both a second opening save and a save at an old revision are stale and
    // answer the current draft without writing.
    for stale in [0, 1, 3] {
        assert_eq!(
            store
                .save_chat_draft(&scope, stale, "stale", &hash, &mut Gate(true))
                .await
                .unwrap(),
            ChatDraftSaved::Conflict(draft(2, "second"))
        );
    }
    assert!(
        store
            .save_chat_draft(&scope, 2, "refused", &hash, &mut Gate(false))
            .await
            .is_err()
    );
    assert_eq!(store.chat_draft(&scope).await.unwrap(), draft(2, "second"));
    // Two saves naming the same revision: exactly one wins.
    let (a, b) = tokio::join!(
        async {
            store
                .save_chat_draft(&scope, 2, "a", &hash, &mut Gate(true))
                .await
        },
        async {
            store
                .save_chat_draft(&scope, 2, "b", &hash, &mut Gate(true))
                .await
        },
    );
    let outcomes = [a.unwrap(), b.unwrap()];
    assert_eq!(
        outcomes
            .iter()
            .filter(|o| matches!(o, ChatDraftSaved::Saved(_)))
            .count(),
        1,
        "{outcomes:?}"
    );
    assert_eq!(store.chat_draft(&scope).await.unwrap().revision, 3);

    // Another physician, another chat and another workspace see nothing.
    for other in [
        Scope {
            physician: "colleague".into(),
            ..scope.clone()
        },
        Scope {
            chat: "chat-other".into(),
            ..scope.clone()
        },
        Scope {
            workspace: "workspace-other".into(),
            ..scope.clone()
        },
    ] {
        assert_eq!(store.chat_draft(&other).await.unwrap(), draft(0, ""));
    }
    let empty = Scope {
        physician: " ".into(),
        ..scope.clone()
    };
    assert!(store.chat_draft(&empty).await.is_err());
    // Drafts never create chat work.
    assert!(store.conversation(&scope).await.is_err());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn old_receipts_reduce_to_a_digest_that_still_answers_duplicates() {
    use entities::command;
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    // A rerun keeps the column and the existing receipts.
    store.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "chat".into(),
    };
    let input = |key: &str, text: &str| Input {
        key: key.into(),
        text: text.into(),
        timezone: "UTC".into(),
    };
    let stored = |key: &str| {
        let store = store.clone();
        let practice = scope.practice.clone();
        let key = key.to_owned();
        async move {
            command::Entity::find_by_id((practice, "physician".to_owned(), key))
                .one(&store.db)
                .await
                .unwrap()
                .unwrap()
        }
    };
    assert_eq!(
        store
            .accept(&scope, &input("old", "old text"))
            .await
            .unwrap(),
        Accepted::New
    );
    assert_eq!(
        store
            .accept(&scope, &input("new", "new text"))
            .await
            .unwrap(),
        Accepted::New
    );
    let fresh = stored("old").await;
    assert!(fresh.accepted_at > 0);
    assert_eq!(fresh.command["input"]["text"], "old text");
    // Age one receipt past the window; the other was accepted just now.
    command::ActiveModel {
        practice: sea_orm::Unchanged(fresh.practice.clone()),
        physician: sea_orm::Unchanged(fresh.physician.clone()),
        key: sea_orm::Unchanged(fresh.key.clone()),
        command: sea_orm::Unchanged(fresh.command.clone()),
        accepted_at: Set(fresh.accepted_at - receipt::RECEIPT_FULL_SECONDS - 1),
    }
    .update(&store.db)
    .await
    .unwrap();
    assert!(
        store
            .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 0)
            .await
            .is_err()
    );
    assert!(store.compact_receipts(-1, 10).await.is_err());
    // Older test rows elsewhere in this database may share the batch.
    while stored("old").await.command.get("sha256").is_none() {
        assert!(
            store
                .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 1000)
                .await
                .unwrap()
                > 0
        );
    }
    let compacted = stored("old").await;
    let keys: Vec<_> = compacted
        .command
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    assert_eq!(keys, ["salt", "sha256"]);
    assert!(!str::contains(&compacted.command.to_string(), "old text"));
    assert_eq!(stored("new").await.command["input"]["text"], "new text");

    // The digest still recognizes the exact command and refuses a different
    // one under the same key; the conversation keeps the authored text.
    assert_eq!(
        store
            .accept(&scope, &input("old", "old text"))
            .await
            .unwrap(),
        Accepted::Duplicate
    );
    assert!(
        store
            .accept(&scope, &input("old", "other text"))
            .await
            .is_err()
    );
    let mut other_chat = scope.clone();
    other_chat.chat = "other-chat".into();
    assert!(
        store
            .accept(&other_chat, &input("old", "old text"))
            .await
            .is_err()
    );
    let conversation = store.conversation(&scope).await.unwrap();
    assert_eq!(conversation.revision, 2);
    assert_eq!(conversation.messages[0].text, "old text");
    // A reduced receipt is never reduced again.
    store
        .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 1000)
        .await
        .unwrap();
    assert_eq!(stored("old").await.command, compacted.command);

    // Try again's receipt compares the same way once reduced.
    let resume_scope = Scope {
        chat: "resume-chat".into(),
        ..scope.clone()
    };
    let hash = vec![9_u8; 32];
    store
        .accept_authorized(&resume_scope, &input("turn", "turn text"), hash.clone())
        .await
        .unwrap();
    let claim = store.claim(&resume_scope, 60).await.unwrap().unwrap();
    store.interrupt(&claim, true).await.unwrap();
    let failed = store.conversation(&resume_scope).await.unwrap();
    assert_eq!(failed.status, "failed");
    let run = failed.attempts.last().unwrap().id.clone();
    assert!(matches!(
        store
            .resume_authorized(&resume_scope, "resume", hash.clone(), &failed.work, &run)
            .await
            .unwrap(),
        Resumed::Pending { .. }
    ));
    let resume = stored("resume").await;
    command::ActiveModel {
        practice: sea_orm::Unchanged(resume.practice.clone()),
        physician: sea_orm::Unchanged(resume.physician.clone()),
        key: sea_orm::Unchanged(resume.key.clone()),
        command: sea_orm::Unchanged(resume.command.clone()),
        accepted_at: Set(0),
    }
    .update(&store.db)
    .await
    .unwrap();
    while stored("resume").await.command.get("sha256").is_none() {
        store
            .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 1000)
            .await
            .unwrap();
    }
    assert_eq!(
        store
            .resume_authorized(&resume_scope, "resume", hash.clone(), &failed.work, &run)
            .await
            .unwrap(),
        Resumed::Duplicate
    );
    assert_eq!(
        store
            .resume_authorized(&resume_scope, "resume", hash, &failed.work, "other-run")
            .await
            .unwrap(),
        Resumed::Refused
    );
    // A reply revision's receipt compares the same way once reduced.
    let reply_scope = Scope {
        chat: "reply".into(),
        ..scope.clone()
    };
    store.open_draft(&reply_scope, "Reply text.").await.unwrap();
    let revise = reply::RevisionInput {
        key: "revise".into(),
        expected_revision: 1,
        instruction: "Shorter.".into(),
        timezone: "UTC".into(),
    };
    assert_eq!(
        store
            .accept_revision(&reply_scope, &revise, vec![9; 32], vec![])
            .await
            .unwrap(),
        reply::RevisionAccepted::New
    );
    let row = stored("revise").await;
    command::ActiveModel {
        practice: sea_orm::Unchanged(row.practice.clone()),
        physician: sea_orm::Unchanged(row.physician.clone()),
        key: sea_orm::Unchanged(row.key.clone()),
        command: sea_orm::Unchanged(row.command.clone()),
        accepted_at: Set(0),
    }
    .update(&store.db)
    .await
    .unwrap();
    while stored("revise").await.command.get("sha256").is_none() {
        store
            .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 1000)
            .await
            .unwrap();
    }
    assert_eq!(
        store
            .accept_revision(&reply_scope, &revise, vec![9; 32], vec![])
            .await
            .unwrap(),
        reply::RevisionAccepted::Duplicate
    );
    let collision = reply::RevisionInput {
        instruction: "Longer.".into(),
        ..revise.clone()
    };
    assert!(
        store
            .accept_revision(&reply_scope, &collision, vec![9; 32], vec![])
            .await
            .is_err()
    );
    let draft = store.draft(&reply_scope).await.unwrap().unwrap();
    assert_eq!(draft.input.as_ref(), Some(&revise));
    // Settle the revision so other suites' recovery sweeps never pick it up.
    store
        .save_draft(
            &reply_scope,
            1,
            "Physician text.",
            &[9; 32],
            &mut Gate(true),
        )
        .await
        .unwrap();

    // Settle the work so other suites' restart sweeps never pick it up.
    let (tx, mut state, _) = store.locked(&resume_scope).await.unwrap();
    state.status = "completed".into();
    Store::save(tx, &resume_scope, &state).await.unwrap();
    let (tx, mut state, _) = store.locked(&scope).await.unwrap();
    state.status = "completed".into();
    Store::save(tx, &scope, &state).await.unwrap();
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn migration_dates_existing_receipts_and_keeps_their_commands() {
    use entities::command;
    use sea_orm::ConnectionTrait;
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let (base, _) = url.rsplit_once('/').unwrap();
    let name = format!("runtime_upgrade_{}", uuid::Uuid::new_v4().simple());
    let admin = Database::connect(format!("{base}/postgres")).await.unwrap();
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await
        .unwrap();
    let upgraded = format!("{base}/{name}");
    // The schema as it stood before this migration: the recorded tables, and
    // one receipt written by the previous code.
    let before = Database::connect(&upgraded).await.unwrap();
    before
        .execute_unprepared(include_str!("../runtime.sql"))
        .await
        .unwrap();
    before
        .execute_unprepared(
            r#"INSERT INTO physician_runtime_commands (practice, physician, key, command)
               VALUES ('p', 'd', 'k', '{"scope": {}, "input": {"text": "kept"}}')"#,
        )
        .await
        .unwrap();
    before.close().await.unwrap();
    let store = Store::connect(&upgraded).await.unwrap();
    store.migrate().await.unwrap();
    store.migrate().await.unwrap();
    let row = command::Entity::find_by_id(("p".to_owned(), "d".to_owned(), "k".to_owned()))
        .one(&store.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.command["input"]["text"], "kept");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    assert!((now - row.accepted_at).abs() < 300, "{}", row.accepted_at);
    // Not old enough yet: the 90 days count from the migration.
    store
        .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 10)
        .await
        .unwrap();
    let row = command::Entity::find_by_id(("p".to_owned(), "d".to_owned(), "k".to_owned()))
        .one(&store.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.command["input"]["text"], "kept");
    drop(store);
    admin
        .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
        .await
        .unwrap();
}

fn event(sequence: u64, kind: engine::EventKind) -> Event {
    Event {
        attempt_id: "attempt".into(),
        sequence,
        kind,
    }
}

fn said(text: &str) -> Item {
    Item::Message {
        role: "assistant".into(),
        text: text.into(),
    }
}

fn returned(items: Vec<Item>) -> engine::EventKind {
    engine::EventKind::ProviderReturned(engine::Turn {
        items,
        receipt: serde_json::json!({}),
    })
}

fn candidate(text: &str) -> engine::EventKind {
    engine::EventKind::Candidate { text: text.into() }
}

#[test]
fn the_published_answer_is_only_the_terminal_turns_text_in_order() {
    let call = Item::Call(engine::Invocation {
        id: "c".into(),
        name: "skill_view".into(),
        arguments: "{}".into(),
    });
    let events = [
        event(1, returned(vec![said("Narration. "), call.clone()])),
        event(
            2,
            engine::EventKind::ToolReturned {
                call_id: "c".into(),
                output: "{}".into(),
                executed: true,
            },
        ),
        event(
            3,
            returned(vec![
                said("One. "),
                Item::Opaque(serde_json::json!({"type": "reasoning"})),
                said("Two."),
            ]),
        ),
        event(4, candidate("Narration. One. Two.")),
    ];
    assert_eq!(terminal_answer(&events).unwrap(), "One. Two.");
}

#[test]
fn a_blank_or_tool_calling_or_missing_terminal_turn_never_publishes() {
    let call = Item::Call(engine::Invocation {
        id: "c".into(),
        name: "skill_view".into(),
        arguments: "{}".into(),
    });
    let narrated = event(1, returned(vec![said("Narration."), call.clone()]));
    for terminal in [
        vec![],
        vec![said("  \n")],
        vec![Item::Opaque(serde_json::json!({}))],
        vec![said("Answer"), call],
    ] {
        let events = [
            narrated.clone(),
            event(2, returned(terminal)),
            event(3, candidate("Narration.")),
        ];
        assert!(terminal_answer(&events).is_err());
    }
    // The candidate must directly follow the turn that produced it.
    assert!(terminal_answer(&[event(1, candidate("Narration."))]).is_err());
    assert!(
        terminal_answer(&[
            event(1, returned(vec![said("Answer")])),
            event(
                2,
                engine::EventKind::ToolReturned {
                    call_id: "c".into(),
                    output: "{}".into(),
                    executed: false,
                },
            ),
            event(3, candidate("Answer")),
        ])
        .is_err()
    );
}
