//! Discuss against real Postgres.
use super::queue_tests::{answer as answer_claim, send};
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
    accept(&store, &source, &delivery_input("first"), 1)
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
    assert!(matches!(
        talk.messages[0].role,
        MessageRole::Assistant { .. }
    ));
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
        format!("{:?}", store.conversation(&opened("talk-1")).await.unwrap()),
        format!("{talk:?}")
    );
    assert_eq!(
        format!("{:?}", store.conversation(&source).await.unwrap()),
        format!("{source_before:?}")
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
    accept(&store, &source, &delivery_input("first"), 1)
        .await
        .unwrap();
    answer(&store, &source, "Her heart rate allows it.").await;
    send(&store, &source, &delivery_input("second"), 1, SendMode::Now)
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
    assert_eq!(held.status, WorkStatus::Held);
    assert_eq!(held.work, work);
    assert_eq!(
        held.attempts.last().unwrap().status,
        AttemptStatus::Cancelled
    );
    assert_eq!(held.session_hash.as_deref(), Some(&[6u8; 32][..]));
    assert!(
        store
            .commit_authorized(
                &claim,
                &Turn {
                    items: vec![Item::assistant("late")],
                    usage: Default::default(),
                    receipt: serde_json::json!({"source": "fixture"}),
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
    let run = held.run().unwrap().id.to_owned();
    assert_eq!(
        store
            .resume_authorized(&source, "resume", vec![9; 32], &work, &run)
            .await
            .unwrap(),
        Resumed::Pending {
            delivery: Delivery::chat_turn(source.clone(), "second".into())
        }
    );
    // As after Stop, the held worker's lease must lapse before a successor.
    let (tx, mut state, _) = store.locked(&source).await.unwrap();
    state.attempts.last_mut().unwrap().expires = 0;
    Store::save(tx, &source, &state).await.unwrap();
    let successor = store.claim(&source, 60).await.unwrap().expect("resumed");
    assert_ne!(successor.attempt, claim.attempt);

    // A Discuss of a finished chat leaves it as it was.
    answer_claim(&store, &successor, "Resumed answer.").await;
    let before = store.conversation(&source).await.unwrap();
    assert_eq!(before.status, WorkStatus::Completed);
    assert_eq!(
        store
            .discuss_authorized(&source, "discuss-done", vec![6; 32], 1, "talk-done")
            .await
            .unwrap(),
        Discussed::Opened
    );
    assert_eq!(
        format!("{:?}", store.conversation(&source).await.unwrap()),
        format!("{before:?}")
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn discuss_while_messages_are_queued_returns_them_to_the_draft_and_never_sends_them() {
    use chat_draft::ChatDraftSaved;
    let store = discuss_store().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let source = delivery_scope(&practice, "chat");
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    accept(&store, &source, &delivery_input("first"), 1)
        .await
        .unwrap();
    answer(&store, &source, "Her heart rate allows it.").await;
    send(&store, &source, &delivery_input("second"), 1, SendMode::Now)
        .await
        .unwrap();
    let claim = store.claim(&source, 60).await.unwrap().expect("a turn");
    let work = store.conversation(&source).await.unwrap().work.clone();
    // Unsent text in the composer, and two Send next messages behind the
    // running answer.
    assert!(matches!(
        store
            .save_chat_draft(&source, 0, "still typing", &[1; 32], &mut OpenAuthority)
            .await
            .unwrap(),
        ChatDraftSaved::Saved(_)
    ));
    for (key, text) in [("q1", "And the potassium?"), ("q2", "Also the creatinine")] {
        let mut input = delivery_input(key);
        input.text = text.into();
        send(&store, &source, &input, 1, SendMode::Next)
            .await
            .unwrap();
    }
    assert_eq!(store.conversation(&source).await.unwrap().queued.len(), 2);

    assert_eq!(
        store
            .discuss_authorized(&source, "discuss", vec![6; 32], 1, "talk")
            .await
            .unwrap(),
        Discussed::Opened
    );
    // Discuss holds the running answer as Stop does, and the queue goes
    // back to the composer after the text already there.
    let held = store.conversation(&source).await.unwrap();
    assert_eq!(held.status, WorkStatus::Held);
    assert!(held.queued.is_empty(), "{:?}", held.queued);
    let messages = |state: &Conversation| {
        state
            .messages
            .iter()
            .map(|message| message.text.clone())
            .collect::<Vec<_>>()
    };
    assert!(
        !messages(&held)
            .iter()
            .any(|text| text == "And the potassium?" || text == "Also the creatinine"),
        "a queued message was sent: {:?}",
        messages(&held)
    );
    let draft = store.chat_draft(&source).await.unwrap();
    assert_eq!(
        draft.text,
        "still typing\n\nAnd the potassium?\n\nAlso the creatinine"
    );
    // The new chat holds only the discussed response, never the queue.
    let talk = store
        .conversation(&Scope {
            chat: "talk".into(),
            ..source.clone()
        })
        .await
        .unwrap();
    assert_eq!(messages(&talk), ["Her heart rate allows it."]);
    assert!(talk.queued.is_empty());
    assert!(
        store
            .chat_draft(&Scope {
                chat: "talk".into(),
                ..source.clone()
            })
            .await
            .unwrap()
            .text
            .is_empty()
    );

    // Discuss asked again returns nothing twice, and nothing queued is woken.
    assert_eq!(
        store
            .discuss_authorized(&source, "discuss", vec![6; 32], 1, "talk")
            .await
            .unwrap(),
        Discussed::Duplicate
    );
    assert_eq!(store.chat_draft(&source).await.unwrap(), draft);
    assert!(
        !store
            .dispatch_due(100, Some(&start))
            .await
            .unwrap()
            .deliveries
            .iter()
            .any(|delivery| delivery.scope == source)
    );

    // Resume answers the held message only; the returned text stays a draft.
    let run = held.run().unwrap().id.to_owned();
    assert_eq!(
        store
            .resume_authorized(&source, "resume", vec![9; 32], &work, &run)
            .await
            .unwrap(),
        Resumed::Pending {
            delivery: Delivery::chat_turn(source.clone(), "second".into())
        }
    );
    let (tx, mut state, _) = store.locked(&source).await.unwrap();
    state.attempts.last_mut().unwrap().expires = 0;
    Store::save(tx, &source, &state).await.unwrap();
    let successor = store.claim(&source, 60).await.unwrap().expect("resumed");
    assert_ne!(successor.attempt, claim.attempt);
    answer_claim(&store, &successor, "Resumed answer.").await;
    let done = store.conversation(&source).await.unwrap();
    assert_eq!(done.status, WorkStatus::Completed);
    assert!(done.queued.is_empty());
    assert_eq!(
        messages(&done),
        [
            "first",
            "Her heart rate allows it.",
            "second",
            "Resumed answer."
        ]
    );
    assert_eq!(store.chat_draft(&source).await.unwrap().text, draft.text);
}
