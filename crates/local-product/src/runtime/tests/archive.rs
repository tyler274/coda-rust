//! Archive and Undo against real Postgres.
use super::super::queue::QueueChanged;
use super::queue_tests::{answer, send};
use super::*;

async fn archive_store() -> Store {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    store
}

async fn listed(store: &Store, scope: &Scope) -> bool {
    store
        .chats(&scope.practice, &scope.physician, &scope.workspace)
        .await
        .unwrap()
        .into_iter()
        .find(|chat| chat.chat == scope.chat)
        .expect("the chat is listed")
        .archived
}

/// Claims the turn and ends it with an answer, through the ordinary fence.
async fn finish(store: &Store, scope: &Scope) {
    let claim = store.claim(scope, 60).await.unwrap().expect("a turn");
    answer(store, &claim, "answer").await;
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn archive_and_undo_each_take_effect_once_across_a_lost_receipt() {
    let store = archive_store().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = delivery_scope(&practice, "chat");
    accept(&store, &scope, &delivery_input("first"), 1)
        .await
        .unwrap();
    finish(&store, &scope).await;
    let archive =
        |key: &'static str| store.archive_authorized(&scope, key, vec![2; 32], Archive::Archive);
    let restore =
        |key: &'static str| store.archive_authorized(&scope, key, vec![2; 32], Archive::Restore);

    assert_eq!(archive("archive-1").await.unwrap(), Archived::Done);
    assert!(listed(&store, &scope).await);
    // The answer was lost and the same Archive is asked again.
    assert_eq!(archive("archive-1").await.unwrap(), Archived::Duplicate);
    assert_eq!(restore("undo-1").await.unwrap(), Archived::Done);
    assert!(!listed(&store, &scope).await);
    // Archive's lost answer, re-asked after Undo, never undoes the Undo.
    assert_eq!(archive("archive-1").await.unwrap(), Archived::Duplicate);
    assert!(!listed(&store, &scope).await);
    // Undo's lost answer, re-asked after a newer Archive, leaves it archived.
    assert_eq!(archive("archive-2").await.unwrap(), Archived::Done);
    assert_eq!(restore("undo-1").await.unwrap(), Archived::Duplicate);
    assert!(listed(&store, &scope).await);
    // A second tab's Archive finds it already archived and its answer is
    // lost; after an Undo, that same Archive asked again leaves it restored.
    assert_eq!(archive("other-tab").await.unwrap(), Archived::Duplicate);
    assert_eq!(restore("undo-2").await.unwrap(), Archived::Done);
    assert_eq!(archive("other-tab").await.unwrap(), Archived::Duplicate);
    assert!(!listed(&store, &scope).await);
    // The same, the other way: a no-op Undo re-asked after a new Archive.
    assert_eq!(restore("other-undo").await.unwrap(), Archived::Duplicate);
    assert_eq!(archive("archive-3").await.unwrap(), Archived::Done);
    assert_eq!(restore("other-undo").await.unwrap(), Archived::Duplicate);
    assert!(listed(&store, &scope).await);
    // One key never names both directions, and a key held by another kind of
    // command is refused.
    assert_eq!(restore("archive-2").await.unwrap(), Archived::Refused);
    assert_eq!(restore("first").await.unwrap(), Archived::Refused);
    assert!(listed(&store, &scope).await);

    // Blank fields, a short session reference and another physician's chat.
    assert_eq!(archive(" ").await.unwrap(), Archived::Refused);
    assert!(
        store
            .archive_authorized(&scope, "short", vec![2; 4], Archive::Archive)
            .await
            .is_err()
    );
    let mut other = scope.clone();
    other.physician = "someone-else".into();
    assert_eq!(
        store
            .archive_authorized(&other, "theirs", vec![2; 32], Archive::Restore)
            .await
            .unwrap(),
        Archived::Refused
    );
    assert!(listed(&store, &scope).await);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn archive_waits_for_running_or_queued_work_and_changes_only_the_list() {
    use chat_draft::ChatDraftSaved;
    let store = archive_store().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = delivery_scope(&practice, "chat");
    accept(&store, &scope, &delivery_input("first"), 1)
        .await
        .unwrap();
    let archive =
        |key: &'static str| store.archive_authorized(&scope, key, vec![2; 32], Archive::Archive);

    // Pending, then running, then running with a message queued behind it.
    assert_eq!(archive("archive").await.unwrap(), Archived::Busy);
    let claim = store.claim(&scope, 60).await.unwrap().expect("a turn");
    assert_eq!(archive("archive").await.unwrap(), Archived::Busy);
    send(&store, &scope, &delivery_input("queued"), 1, SendMode::Next)
        .await
        .unwrap();
    assert_eq!(archive("archive").await.unwrap(), Archived::Busy);
    assert!(!listed(&store, &scope).await);
    // Busy kept no receipt: the queue still takes its own commands, and the
    // same Archive key can be asked again once the work ends.
    assert_eq!(
        store
            .withdraw_queued(&scope, "withdraw", "queued")
            .await
            .unwrap(),
        QueueChanged::Changed
    );
    assert_eq!(archive("archive").await.unwrap(), Archived::Busy);
    answer(&store, &claim, "answer").await;

    assert!(matches!(
        store
            .save_chat_draft(&scope, 0, "unsent", &[1; 32], &mut OpenAuthority)
            .await
            .unwrap(),
        ChatDraftSaved::Saved(_)
    ));
    let before = store.conversation(&scope).await.unwrap();
    let draft = store.chat_draft(&scope).await.unwrap();
    assert_eq!(archive("archive").await.unwrap(), Archived::Done);
    // Archive moves the chat in the list and changes nothing else: the
    // conversation, its work and its draft stay as they were. Attention and
    // follow-ups are not the conversation's, and no path here reaches them.
    let mut after = store.conversation(&scope).await.unwrap();
    assert!(after.archived_at.is_some());
    after.archived_at = None;
    // Conversation has no equality of its own; its Debug form shows every field.
    assert_eq!(format!("{after:?}"), format!("{before:?}"));
    assert_eq!(store.chat_draft(&scope).await.unwrap(), draft);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn sending_resume_and_try_again_bring_an_archived_conversation_back() {
    let store = archive_store().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = delivery_scope(&practice, "chat");
    accept(&store, &scope, &delivery_input("first"), 1)
        .await
        .unwrap();
    finish(&store, &scope).await;

    // Sending.
    store
        .archive_authorized(&scope, "a1", vec![2; 32], Archive::Archive)
        .await
        .unwrap();
    send(&store, &scope, &delivery_input("second"), 1, SendMode::Now)
        .await
        .unwrap();
    assert!(!listed(&store, &scope).await);

    // Try again on a failed turn.
    let turn = Delivery::chat_turn(scope.clone(), "second".into());
    for _ in 0..MAX_FAILED_ATTEMPTS {
        let claim = store.claim_delivery(&turn, 60).await.unwrap().unwrap();
        store
            .settle_chat_attempt(&claim, harness::engine::AttemptFailure::Failed)
            .await
            .unwrap();
    }
    let failed = store.conversation(&scope).await.unwrap();
    assert_eq!(failed.status, WorkStatus::Failed);
    assert_eq!(
        store
            .archive_authorized(&scope, "a2", vec![2; 32], Archive::Archive)
            .await
            .unwrap(),
        Archived::Done,
        "a failed turn is not running"
    );
    let run = failed.run().unwrap().id.to_owned();
    assert_eq!(
        store
            .resume_authorized(&scope, "try-again", vec![2; 32], &failed.work, &run)
            .await
            .unwrap(),
        Resumed::Pending {
            delivery: Delivery::chat_turn(scope.clone(), "second".into())
        }
    );
    assert!(!listed(&store, &scope).await);

    // Resume after Stop.
    let work = store.conversation(&scope).await.unwrap().work;
    store.claim(&scope, 60).await.unwrap().expect("a turn");
    assert_eq!(
        store
            .stop_authorized(&scope, "stop", vec![2; 32], &work)
            .await
            .unwrap(),
        Stopped::Held
    );
    store
        .archive_authorized(&scope, "a3", vec![2; 32], Archive::Archive)
        .await
        .unwrap();
    assert!(listed(&store, &scope).await);
    let held = store.conversation(&scope).await.unwrap();
    let run = held.run().unwrap().id.to_owned();
    store
        .resume_authorized(&scope, "resume", vec![2; 32], &held.work, &run)
        .await
        .unwrap();
    assert!(!listed(&store, &scope).await);
}
