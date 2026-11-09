//! Discuss against real Postgres.
use super::*;

async fn discuss_store() -> Store {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    store
}

/// Claims the turn and ends it with `text`, through the ordinary fence.
async fn answer(store: &Store, scope: &Scope, text: &str) {
    let claim = store.claim(scope, 60).await.unwrap().expect("a turn");
    answer_claim(store, &claim, text).await;
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn the_same_discuss_returns_its_chat_and_a_later_one_opens_a_fresh_chat() {
    let store = discuss_store().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let source = delivery_scope(&practice, "chat");
    store
        .accept_authorized(&source, &delivery_input("first"), vec![1; 32])
        .await
        .unwrap();
    answer(&store, &source, "Her heart rate allows it.").await;
    let response = 1;
    let discuss = |key: &'static str, chat: &'static str| {
        store.discuss_authorized(&source, key, vec![2; 32], response, chat)
    };
    let opened = |chat: &str| Scope {
        chat: chat.into(),
        ..source.clone()
    };

    assert_eq!(
        discuss("discuss-1", "talk-1").await.unwrap(),
        Discussed::Opened
    );
    let talk = store.conversation(&opened("talk-1")).await.unwrap();
    assert_eq!(talk.messages.len(), 1);
    assert_eq!(talk.messages[0].role, "assistant");
    assert_eq!(talk.messages[0].text, "Her heart rate allows it.");
    assert_eq!(
        talk.discusses,
        Some(Discusses {
            chat: "chat".into(),
            response
        })
    );
    // The chat list shows it, named by the response it discusses.
    let listed = store
        .chats(&practice, &source.physician, &source.workspace)
        .await
        .unwrap();
    let summary = listed
        .iter()
        .find(|chat| chat.chat == "talk-1")
        .expect("the new chat is listed");
    assert_eq!(summary.first_message, "Her heart rate allows it.");

    // The same Discuss asked again names the same chat and opens nothing new.
    let source_before = store.conversation(&source).await.unwrap();
    assert_eq!(
        discuss("discuss-1", "talk-1").await.unwrap(),
        Discussed::Duplicate
    );
    assert_eq!(
        serde_json::to_value(store.conversation(&opened("talk-1")).await.unwrap()).unwrap(),
        serde_json::to_value(&talk).unwrap()
    );
    assert_eq!(
        serde_json::to_value(store.conversation(&source).await.unwrap()).unwrap(),
        serde_json::to_value(&source_before).unwrap()
    );
    // Its key never names another chat.
    assert_eq!(
        discuss("discuss-1", "talk-other").await.unwrap(),
        Discussed::Refused
    );

    // A later, deliberate Discuss of the same response opens a fresh chat.
    assert_eq!(
        discuss("discuss-2", "talk-2").await.unwrap(),
        Discussed::Opened
    );
    let fresh = store.conversation(&opened("talk-2")).await.unwrap();
    assert_eq!(fresh.messages[0].text, "Her heart rate allows it.");
    let listed = store
        .chats(&practice, &source.physician, &source.workspace)
        .await
        .unwrap();
    assert_eq!(
        listed
            .iter()
            .filter(|chat| chat.chat.starts_with("talk-"))
            .count(),
        2
    );
    // An existing chat is never taken over by a new Discuss.
    assert_eq!(
        discuss("discuss-3", "talk-1").await.unwrap(),
        Discussed::Refused
    );

    // Only one of Coda's responses can be discussed: not the physician's
    // message, not a place past the end, and not the source chat itself.
    for (key, place, chat) in [
        ("own", 0, "talk-own"),
        ("past", 9, "talk-past"),
        ("self", response, "chat"),
    ] {
        assert_eq!(
            store
                .discuss_authorized(&source, key, vec![2; 32], place, chat)
                .await
                .unwrap(),
            Discussed::Refused,
            "{key}"
        );
    }
    // A refused Discuss kept no receipt: the key still opens a chat.
    assert_eq!(discuss("own", "talk-own").await.unwrap(), Discussed::Opened);

    // Blank fields, a short session reference and another physician's chat.
    assert_eq!(discuss(" ", "talk-3").await.unwrap(), Discussed::Refused);
    assert_eq!(discuss("blank", " ").await.unwrap(), Discussed::Refused);
    assert!(
        store
            .discuss_authorized(&source, "short", vec![2; 4], response, "talk-3")
            .await
            .is_err()
    );
    let mut other = source.clone();
    other.physician = "someone-else".into();
    assert_eq!(
        store
            .discuss_authorized(&other, "theirs", vec![2; 32], response, "talk-3")
            .await
            .unwrap(),
        Discussed::Refused
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn discuss_holds_running_work_the_way_stop_does_until_resume() {
    let store = discuss_store().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let source = delivery_scope(&practice, "chat");
    store
        .accept_authorized(&source, &delivery_input("first"), vec![1; 32])
        .await
        .unwrap();
    answer(&store, &source, "Her heart rate allows it.").await;
    store
        .send_authorized(
            &source,
            &delivery_input("second"),
            vec![1; 32],
            SendMode::Now,
        )
        .await
        .unwrap();
    let claim = store.claim(&source, 60).await.unwrap().expect("a turn");
    let work = store.conversation(&source).await.unwrap().work.clone();

    assert_eq!(
        store
            .discuss_authorized(&source, "discuss", vec![6; 32], 1, "talk")
            .await
            .unwrap(),
        Discussed::Opened
    );
    // The running answer is held exactly as Stop holds it: its run is
    // cancelled, it can no longer publish, and nothing dispatches it.
    assert!(!store.is_current(&claim).await.unwrap());
    let held = store.conversation(&source).await.unwrap();
    assert_eq!(held.status, "held");
    assert_eq!(held.work, work);
    assert_eq!(held.attempts.last().unwrap().status, "cancelled");
    assert_eq!(held.session_hash.as_deref(), Some(&[6u8; 32][..]));
    assert!(
        store
            .commit_authorized(
                &claim,
                &engine::Candidate {
                    text: "late".into(),
                    transcript: vec![],
                    usage: engine::Usage::default(),
                },
                &mut OpenAuthority,
            )
            .await
            .is_err()
    );
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    assert!(
        !store
            .dispatch_due(100, Some(&start))
            .await
            .unwrap()
            .deliveries
            .iter()
            .any(|delivery| delivery.scope == source)
    );
    // Stop finds it already held.
    assert_eq!(
        store
            .stop_authorized(&source, "stop", vec![6; 32], &work)
            .await
            .unwrap(),
        Stopped::Duplicate
    );

    // The same Discuss asked again leaves it held, and Resume restarts it.
    assert_eq!(
        store
            .discuss_authorized(&source, "discuss", vec![6; 32], 1, "talk")
            .await
            .unwrap(),
        Discussed::Duplicate
    );
    let run = held.attempts.last().unwrap().id.clone();
    assert_eq!(
        store
            .resume_authorized(&source, "resume", vec![9; 32], &work, &run)
            .await
            .unwrap(),
        Resumed::Pending {
            input_id: "second".into()
        }
    );
    let successor = store.claim(&source, 60).await.unwrap().expect("resumed");
    assert_ne!(successor.attempt, claim.attempt);

    // A Discuss of a finished chat leaves it as it was.
    answer_claim(&store, &successor, "Resumed answer.").await;
    let before = store.conversation(&source).await.unwrap();
    assert_eq!(before.status, "completed");
    assert_eq!(
        store
            .discuss_authorized(&source, "discuss-done", vec![6; 32], 1, "talk-done")
            .await
            .unwrap(),
        Discussed::Opened
    );
    assert_eq!(
        serde_json::to_value(store.conversation(&source).await.unwrap()).unwrap(),
        serde_json::to_value(&before).unwrap()
    );
}

async fn answer_claim(store: &Store, claim: &Claim, text: &str) {
    let event = |sequence, kind| Event {
        attempt_id: claim.attempt.clone(),
        sequence,
        kind,
    };
    store
        .persist(claim, event(1, returned(vec![said(text)])))
        .await
        .unwrap();
    store
        .persist(claim, event(2, candidate(text)))
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
