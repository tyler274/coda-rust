//! Send next, edit and withdraw against real Postgres.
use super::super::queue::QueueChanged;
use super::*;

async fn queue_store() -> (Store, String) {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    (store, url)
}

fn typed(key: &str, text: &str) -> Input {
    Input {
        key: key.into(),
        text: text.into(),
        timezone: "UTC".into(),
    }
}

/// Publishes the claimed attempt's answer through the ordinary fence.
async fn answer(store: &Store, claim: &Claim, text: &str) {
    store
        .persist(claim, event_for(claim, 1, returned(vec![said(text)])))
        .await
        .unwrap();
    store
        .persist(claim, event_for(claim, 2, candidate(text)))
        .await
        .unwrap();
    store
        .commit_authorized(
            claim,
            &engine::Candidate {
                text: text.into(),
                transcript: vec![],
                usage: engine::Usage::default(),
            },
            &mut OpenAuthority,
        )
        .await
        .unwrap();
}

fn event_for(claim: &Claim, sequence: u64, kind: engine::EventKind) -> Event {
    Event {
        attempt_id: claim.attempt.clone(),
        sequence,
        kind,
    }
}

fn texts(state: &Conversation) -> Vec<&str> {
    state.messages.iter().map(|m| m.text.as_str()).collect()
}

fn queued(state: &Conversation) -> Vec<(&str, &str)> {
    state
        .queued
        .iter()
        .map(|q| (q.id.as_str(), q.text.as_str()))
        .collect()
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn send_next_waits_and_two_queued_messages_get_one_reply_after_the_answer() {
    let (store, url) = queue_store().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = delivery_scope(&practice, "chat");
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    // Send next with nothing running starts a turn at once, like Send.
    assert_eq!(
        store
            .send_authorized(
                &scope,
                &typed("first", "first"),
                vec![1; 32],
                SendMode::Next
            )
            .await
            .unwrap(),
        Accepted::New
    );
    let claim = store.claim(&scope, 60).await.unwrap().expect("first turn");
    let work = store.conversation(&scope).await.unwrap().work;

    for (key, text) in [("q1", "second"), ("q2", "third")] {
        assert_eq!(
            store
                .send_authorized(&scope, &typed(key, text), vec![2; 32], SendMode::Next)
                .await
                .unwrap(),
            Accepted::Queued
        );
    }
    // A retry after a lost receipt is the same command: still one item.
    assert_eq!(
        store
            .send_authorized(&scope, &typed("q1", "second"), vec![2; 32], SendMode::Next)
            .await
            .unwrap(),
        Accepted::Duplicate
    );
    // The same key as Send is another command and collides.
    assert!(
        store
            .send_authorized(&scope, &typed("q1", "second"), vec![2; 32], SendMode::Now)
            .await
            .is_err()
    );
    let running = store.conversation(&scope).await.unwrap();
    assert_eq!(texts(&running), ["first"]);
    assert_eq!(queued(&running), [("q1", "second"), ("q2", "third")]);
    // Queueing neither steers nor wakes anything: the running claim stays current.
    assert!(store.is_current(&claim).await.unwrap());
    assert_eq!(running.input_id.as_deref(), Some("first"));

    // The queue survives a restart of the process that holds the store.
    drop(store);
    let store = Store::connect(&url).await.unwrap();
    assert_eq!(
        queued(&store.conversation(&scope).await.unwrap()),
        [("q1", "second"), ("q2", "third")]
    );

    answer(&store, &claim, "first answer").await;
    let next = store.conversation(&scope).await.unwrap();
    assert_eq!(
        texts(&next),
        ["first", "first answer", "second", "third"],
        "both queued messages join the conversation together, in order"
    );
    assert!(next.queued.is_empty());
    assert_eq!(next.status, "pending");
    assert_ne!(next.work, work, "the queued messages are new work");
    assert_eq!(next.input_id.as_deref(), Some("q2"));
    assert_eq!(next.session_hash.as_deref(), Some(&[2u8; 32][..]));
    // Recovery wakes exactly one turn for both.
    let due: Vec<_> = store
        .dispatch_due(100, Some(&start))
        .await
        .unwrap()
        .deliveries
        .into_iter()
        .filter(|delivery| delivery.scope == scope)
        .collect();
    assert_eq!(due, [Delivery::chat_turn(scope.clone(), "q2".into())]);
    let reply = store
        .claim_delivery(&due[0], 60)
        .await
        .unwrap()
        .expect("one turn for both");
    answer(&store, &reply, "one reply").await;
    let done = store.conversation(&scope).await.unwrap();
    assert_eq!(
        texts(&done),
        ["first", "first answer", "second", "third", "one reply"]
    );
    assert_eq!(done.status, "completed");

    // A retried queue command after the turn started is still a duplicate,
    // never a second send.
    assert_eq!(
        store
            .send_authorized(&scope, &typed("q2", "third"), vec![2; 32], SendMode::Next)
            .await
            .unwrap(),
        Accepted::Duplicate
    );
    assert_eq!(store.conversation(&scope).await.unwrap().messages.len(), 5);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn edit_and_withdraw_take_effect_once_and_never_reach_a_started_message() {
    let (store, url) = queue_store().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = delivery_scope(&practice, "chat");
    store
        .accept_authorized(&scope, &typed("first", "first"), vec![1; 32])
        .await
        .unwrap();
    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    for (key, text) in [("q1", "draft one"), ("q2", "draft two"), ("q3", "gone")] {
        store
            .send_authorized(&scope, &typed(key, text), vec![2; 32], SendMode::Next)
            .await
            .unwrap();
    }

    assert_eq!(
        store
            .edit_queued(&scope, "edit-1", "q1", "edited one")
            .await
            .unwrap(),
        QueueChanged::Changed
    );
    // A lost receipt retried, before and after a restart: no second change.
    assert_eq!(
        store
            .edit_queued(&scope, "edit-1", "q1", "edited one")
            .await
            .unwrap(),
        QueueChanged::Duplicate
    );
    drop(store);
    let store = Store::connect(&url).await.unwrap();
    assert_eq!(
        store
            .edit_queued(&scope, "edit-1", "q1", "edited one")
            .await
            .unwrap(),
        QueueChanged::Duplicate
    );
    // The same key with other text is a different command.
    assert_eq!(
        store
            .edit_queued(&scope, "edit-1", "q1", "something else")
            .await
            .unwrap(),
        QueueChanged::Refused
    );
    // A later, deliberate edit is its own command.
    assert_eq!(
        store
            .edit_queued(&scope, "edit-2", "q2", "edited two")
            .await
            .unwrap(),
        QueueChanged::Changed
    );
    assert_eq!(
        store
            .withdraw_queued(&scope, "withdraw-3", "q3")
            .await
            .unwrap(),
        QueueChanged::Changed
    );
    assert_eq!(
        store
            .withdraw_queued(&scope, "withdraw-3", "q3")
            .await
            .unwrap(),
        QueueChanged::Duplicate
    );
    // A second withdrawal under a new key finds nothing queued.
    assert_eq!(
        store
            .withdraw_queued(&scope, "withdraw-3b", "q3")
            .await
            .unwrap(),
        QueueChanged::Refused
    );
    // Blank fields, another physician and an unknown item are refused.
    assert_eq!(
        store
            .edit_queued(&scope, "edit-blank", "q1", "  ")
            .await
            .unwrap(),
        QueueChanged::Refused
    );
    assert_eq!(
        store.withdraw_queued(&scope, " ", "q1").await.unwrap(),
        QueueChanged::Refused
    );
    let mut other = scope.clone();
    other.physician = "someone-else".into();
    assert_eq!(
        store.withdraw_queued(&other, "theirs", "q1").await.unwrap(),
        QueueChanged::Refused
    );
    assert_eq!(
        store
            .withdraw_queued(&scope, "unknown", "nope")
            .await
            .unwrap(),
        QueueChanged::Refused
    );
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(queued(&state), [("q1", "edited one"), ("q2", "edited two")]);

    answer(&store, &claim, "first answer").await;
    let started = store.conversation(&scope).await.unwrap();
    assert_eq!(
        texts(&started),
        ["first", "first answer", "edited one", "edited two"],
        "the withdrawn message is never sent"
    );
    // The messages have started: edit and withdraw no longer reach them, and
    // a refused command keeps no receipt.
    assert_eq!(
        store
            .edit_queued(&scope, "late-edit", "q1", "too late")
            .await
            .unwrap(),
        QueueChanged::Refused
    );
    assert_eq!(
        store
            .withdraw_queued(&scope, "late-withdraw", "q2")
            .await
            .unwrap(),
        QueueChanged::Refused
    );
    // The earlier edit, retried now, is still the one it was.
    assert_eq!(
        store
            .edit_queued(&scope, "edit-1", "q1", "edited one")
            .await
            .unwrap(),
        QueueChanged::Duplicate
    );
    assert_eq!(
        texts(&store.conversation(&scope).await.unwrap())[2],
        "edited one"
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn stop_returns_queued_text_to_the_composer_and_never_sends_it() {
    use chat_draft::ChatDraftSaved;
    let (store, _) = queue_store().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = delivery_scope(&practice, "chat");
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    store
        .accept_authorized(&scope, &typed("first", "first"), vec![1; 32])
        .await
        .unwrap();
    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let work = store.conversation(&scope).await.unwrap().work;
    // The physician has unsent text in the composer already.
    assert!(matches!(
        store
            .save_chat_draft(&scope, 0, "still typing", &[1; 32], &mut OpenAuthority)
            .await
            .unwrap(),
        ChatDraftSaved::Saved(_)
    ));
    for (key, text) in [("q1", "second"), ("q2", "third")] {
        store
            .send_authorized(&scope, &typed(key, text), vec![2; 32], SendMode::Next)
            .await
            .unwrap();
    }

    assert_eq!(
        store
            .stop_authorized(&scope, "stop", vec![3; 32], &work)
            .await
            .unwrap(),
        Stopped::Held
    );
    let held = store.conversation(&scope).await.unwrap();
    assert!(held.queued.is_empty());
    assert_eq!(texts(&held), ["first"]);
    let draft = store.chat_draft(&scope).await.unwrap();
    assert_eq!(draft.text, "still typing\n\nsecond\n\nthird");
    assert_eq!(draft.revision, 2, "an open page sees the returned text");
    // A replayed Stop returns nothing twice.
    assert_eq!(
        store
            .stop_authorized(&scope, "stop", vec![3; 32], &work)
            .await
            .unwrap(),
        Stopped::Duplicate
    );
    assert_eq!(store.chat_draft(&scope).await.unwrap(), draft);
    // Withdraw finds nothing: the text is the physician's again.
    assert_eq!(
        store.withdraw_queued(&scope, "late", "q1").await.unwrap(),
        QueueChanged::Refused
    );
    // The stopped answer cannot publish, and nothing queued is woken.
    let late = engine::Candidate {
        text: "late".into(),
        transcript: vec![],
        usage: engine::Usage::default(),
    };
    assert!(
        store
            .commit_authorized(&claim, &late, &mut OpenAuthority)
            .await
            .is_err()
    );
    assert!(
        !store
            .dispatch_due(100, Some(&start))
            .await
            .unwrap()
            .deliveries
            .iter()
            .any(|delivery| delivery.scope == scope)
    );

    // While held, nothing is running, so Send next sends at once.
    assert_eq!(
        store
            .send_authorized(
                &scope,
                &typed("after", "after"),
                vec![4; 32],
                SendMode::Next
            )
            .await
            .unwrap(),
        Accepted::New
    );
    let resumed = store.conversation(&scope).await.unwrap();
    assert_eq!(texts(&resumed), ["first", "after"]);
    assert!(resumed.queued.is_empty());

    // Stop with no draft opens one with the queued text.
    let fresh = delivery_scope(&practice, "fresh");
    store
        .accept_authorized(&fresh, &typed("f1", "f1"), vec![1; 32])
        .await
        .unwrap();
    store
        .send_authorized(&fresh, &typed("f2", "queued"), vec![1; 32], SendMode::Next)
        .await
        .unwrap();
    let fresh_work = store.conversation(&fresh).await.unwrap().work;
    assert_eq!(
        store
            .stop_authorized(&fresh, "stop-fresh", vec![1; 32], &fresh_work)
            .await
            .unwrap(),
        Stopped::Held
    );
    let draft = store.chat_draft(&fresh).await.unwrap();
    assert_eq!((draft.revision, draft.text.as_str()), (1, "queued"));
}
