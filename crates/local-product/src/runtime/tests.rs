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
