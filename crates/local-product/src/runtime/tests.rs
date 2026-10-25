use super::delivery::Delivery;
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
    assert!(s.pending().await.unwrap().contains(&scope));
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
    let first = Delivery {
        scope: scope.clone(),
        input_id: "first".into(),
    };
    let page = store.dispatch_due(100, Some(&start)).await.unwrap();
    assert_eq!(ours(&page), vec![first.clone()]);
    // Successful publication suppresses republish until the retry hint passes.
    store.dispatched(&first, 600).await.unwrap();
    let page = store.dispatch_due(100, Some(&start)).await.unwrap();
    assert!(ours(&page).is_empty());
    assert!(!store.delivery_settled(&first).await.unwrap());

    // A newer accepted input supersedes the old wakeup: it can never claim,
    // and acknowledging it is safe because the newer input is recoverable.
    store
        .accept_authorized(&scope, &delivery_input("second"), vec![7; 32])
        .await
        .unwrap();
    assert!(store.claim_delivery(&first, 60).await.unwrap().is_none());
    assert!(store.delivery_settled(&first).await.unwrap());
    let second = Delivery {
        scope: scope.clone(),
        input_id: "second".into(),
    };
    assert_eq!(
        ours(&store.dispatch_due(100, Some(&start)).await.unwrap()),
        vec![second.clone()]
    );

    // Duplicate wakeups find no second live claim.
    let claim = store.claim_delivery(&second, 60).await.unwrap().unwrap();
    assert!(store.claim_delivery(&second, 60).await.unwrap().is_none());
    store.heartbeat(&claim, 120).await.unwrap();
    assert!(!store.delivery_settled(&second).await.unwrap());

    // A failed run is released for broker redelivery as a successor attempt.
    store.retry_delivery(&claim).await.unwrap();
    assert_eq!(store.conversation(&scope).await.unwrap().status, "pending");
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
    assert!(store.delivery_settled(&second).await.unwrap());
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
    let turn = Delivery {
        scope: scope.clone(),
        input_id: "turn".into(),
    };
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
    assert!(!store.delivery_settled(&turn).await.unwrap());

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
    let delivery = Delivery {
        scope: scope.clone(),
        input_id: "turn".into(),
    };
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
