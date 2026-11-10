use super::super::*;
use super::{delivery_input, delivery_scope};

/// Accepts `key`, then runs one attempt to a published reply, "reply to {key}".
pub(super) async fn answer(store: &Store, scope: &Scope, key: &str) -> Claim {
    store.accept(scope, &delivery_input(key)).await.unwrap();
    let claim = store.claim(scope, 60).await.unwrap().unwrap();
    let reply = format!("reply to {key}");
    let kinds = [
        engine::EventKind::ProviderReturned(engine::Turn {
            items: vec![Item::Message {
                role: "assistant".into(),
                text: reply.clone(),
            }],
            receipt: serde_json::json!({}),
        }),
        engine::EventKind::Candidate {
            text: reply.clone(),
        },
    ];
    for (sequence, kind) in (1..).zip(kinds) {
        let event = Event {
            attempt_id: claim.attempt.clone(),
            sequence,
            kind,
        };
        store.persist(&claim, event).await.unwrap();
    }
    let candidate = Candidate {
        text: reply,
        transcript: vec![],
        usage: Usage::default(),
    };
    store.commit(&claim, &candidate).await.unwrap();
    claim
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn each_reply_names_the_run_that_published_it_across_a_fresh_store() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = delivery_scope(&uuid::Uuid::new_v4().to_string(), "chat");
    let first = answer(&store, &scope, "first").await;
    // An interrupted attempt between the two answers never names a reply.
    store
        .accept(&scope, &delivery_input("second"))
        .await
        .unwrap();
    let lost = store.claim(&scope, 60).await.unwrap().unwrap();
    store.interrupt(&lost, false).await.unwrap();
    let third = answer(&store, &scope, "third").await;
    drop(store);

    let store = Store::connect(&url).await.unwrap();
    let state = store.conversation(&scope).await.unwrap();
    let responses: Vec<_> = state
        .responses()
        .into_iter()
        .map(|r| (r.place, r.run.to_string(), r.text.to_owned()))
        .collect();
    assert_eq!(
        responses,
        [
            (1, first.attempt.clone(), "reply to first".into()),
            (4, third.attempt.clone(), "reply to third".into()),
        ]
    );
    let saved: Vec<_> = state.messages.iter().map(|m| m.run.clone()).collect();
    assert_eq!(saved[0], None, "a physician message has no run");
    assert_eq!(saved[1].as_ref().map(RunId::as_str), Some(&*first.attempt));
    assert!(state.response(&RunId(lost.attempt)).is_none());
    assert_eq!(
        state.response(&RunId(third.attempt)).map(|r| r.place),
        Some(4)
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_reply_saved_before_runs_were_recorded_reads_its_completed_attempt() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = delivery_scope(&uuid::Uuid::new_v4().to_string(), "chat");
    let first = answer(&store, &scope, "first").await;
    let second = answer(&store, &scope, "second").await;
    // Save the row as an older build wrote it: no `run` on any message.
    let (tx, state, _) = store.locked(&scope).await.unwrap();
    let mut legacy = serde_json::to_value(&state).unwrap();
    for message in legacy["messages"].as_array_mut().unwrap() {
        message.as_object_mut().unwrap().remove("run");
    }
    conversation::Entity::update(conversation_model(&scope, legacy))
        .exec(&tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let state = store.conversation(&scope).await.unwrap();
    assert!(state.messages.iter().all(|m| m.run.is_none()));
    let runs: Vec<_> = state
        .responses()
        .into_iter()
        .map(|r| (r.place, r.run.to_string()))
        .collect();
    assert_eq!(runs, [(1, first.attempt), (3, second.attempt)]);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn try_again_refuses_another_physician_or_practice() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = delivery_scope(&practice, "chat");
    store
        .accept_authorized(&scope, &delivery_input("turn"), vec![5; 32])
        .await
        .unwrap();
    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    store.interrupt(&claim, true).await.unwrap();
    let failed = store.conversation(&scope).await.unwrap();
    assert_eq!(failed.status, "failed");

    let mut colleague = scope.clone();
    colleague.physician = "colleague".into();
    let mut elsewhere = scope.clone();
    elsewhere.practice = uuid::Uuid::new_v4().to_string();
    for (key, target) in [("colleague", &colleague), ("elsewhere", &elsewhere)] {
        assert_eq!(
            store
                .resume_authorized(target, key, vec![6; 32], &failed.work, &claim.attempt)
                .await
                .unwrap(),
            Resumed::Refused,
            "{key}"
        );
    }
    let unchanged = store.conversation(&scope).await.unwrap();
    assert_eq!(unchanged.status, "failed");
    assert_eq!(unchanged.session_hash.as_deref(), Some(&[5u8; 32][..]));
}
