//! Postgres proof that feedback stays on the exact response it rates, named
//! by the run that published it, across a fresh store; that every save
//! replaces the rating and details and increments the revision; and that only
//! an authorized save on one of the chat's responses writes anything.
use std::collections::BTreeSet;

use super::run_tests::answer;
use super::*;
use crate::runtime::feedback::{
    Details, Feedback, FeedbackSaved, MAX_DETAILS_CHARS, Rating, Reason,
};

/// A chat in which each question was answered by its own run; answers the
/// runs in order. The replies read "reply to {chat}-{turn}".
async fn answered_chat(store: &Store, scope: &Scope, turns: usize) -> Vec<RunId> {
    let mut runs = Vec::new();
    for turn in 0..turns {
        let claim = answer(store, scope, &format!("{}-{turn}", scope.chat)).await;
        runs.push(RunId(claim.attempt));
    }
    runs
}

fn scope(practice: &str, physician: &str, chat: &str) -> Scope {
    Scope {
        practice: practice.into(),
        physician: physician.into(),
        workspace: "workspace".into(),
        chat: chat.into(),
    }
}

fn saved(outcome: FeedbackSaved) -> Feedback {
    match outcome {
        FeedbackSaved::Saved(feedback) => feedback,
        other => panic!("feedback not saved: {other:?}"),
    }
}

async fn rate(store: &Store, scope: &Scope, run: &RunId, rating: Rating) -> FeedbackSaved {
    store
        .save_feedback(scope, run, rating, None, &[7_u8; 32], &mut Gate(true))
        .await
        .unwrap()
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn each_rating_stays_on_its_own_response_across_a_fresh_store() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let chat = scope(&practice, "physician", "chat");
    let runs = answered_chat(&store, &chat, 2).await;
    let first = saved(rate(&store, &chat, &runs[0], Rating::Up).await);
    assert_eq!(
        (first.rating, first.details.clone(), first.revision),
        (Rating::Up, None, 1)
    );
    assert!(first.saved_at > 0);
    saved(rate(&store, &chat, &runs[1], Rating::Down).await);

    drop(store);
    let store = Store::connect(&url).await.unwrap();
    let read = store.chat_feedback(&chat).await.unwrap();
    assert_eq!(read[&runs[0]].rating, Rating::Up);
    assert_eq!(read[&runs[1]].rating, Rating::Down);
    assert_eq!(read.len(), 2);

    // A repeat converges and the other thumb replaces the rating; every save
    // is a new revision.
    for (rating, revision) in [(Rating::Down, 2), (Rating::Down, 3)] {
        let again = saved(rate(&store, &chat, &runs[0], rating).await);
        assert_eq!((again.rating, again.revision), (Rating::Down, revision));
    }
    let read = store.chat_feedback(&chat).await.unwrap();
    assert_eq!((read.len(), read[&runs[0]].revision), (2, 3));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn only_an_authorized_save_on_a_response_of_the_chat_writes() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let chat = scope(&practice, "physician", "chat");
    let runs = answered_chat(&store, &chat, 1).await;
    let other = scope(&practice, "physician", "other");
    let elsewhere = answered_chat(&store, &other, 1).await;
    // A run interrupted before it answered published nothing.
    store
        .accept(&chat, &delivery_input("unanswered"))
        .await
        .unwrap();
    let lost = store.claim(&chat, 60).await.unwrap().unwrap();
    store.interrupt(&lost, false).await.unwrap();
    let hash = [7_u8; 32];

    // An unknown run, a run that never answered, another chat's run and a
    // chat that doesn't exist hold no response of this chat's.
    for (run, target) in [
        (RunId::from("no-such-run".to_owned()), &chat),
        (RunId(lost.attempt.clone()), &chat),
        (elsewhere[0].clone(), &chat),
        (runs[0].clone(), &scope(&practice, "physician", "none")),
    ] {
        assert_eq!(
            rate(&store, target, &run, Rating::Up).await,
            FeedbackSaved::NotAResponse,
            "{run}"
        );
    }
    // Another physician's chat by the same name is not theirs to rate.
    let colleague = scope(&practice, "colleague", "chat");
    assert_eq!(
        rate(&store, &colleague, &runs[0], Rating::Up).await,
        FeedbackSaved::NotAResponse
    );
    // A refused authority and a malformed session write nothing.
    assert_eq!(
        store
            .save_feedback(&chat, &runs[0], Rating::Up, None, &hash, &mut Gate(false))
            .await
            .unwrap(),
        FeedbackSaved::Refused
    );
    assert!(
        store
            .save_feedback(&chat, &runs[0], Rating::Up, None, &[1; 4], &mut Gate(true))
            .await
            .is_err()
    );
    assert!(store.chat_feedback(&chat).await.unwrap().is_empty());
    assert!(
        store
            .chat_feedback(&scope(&practice, " ", "chat"))
            .await
            .is_err()
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn each_save_replaces_the_rating_and_the_details() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let chat = scope(&practice, "physician", "chat");
    let run = &answered_chat(&store, &chat, 1).await[0];
    let hash = [7_u8; 32];
    let details = Details::new(
        BTreeSet::from([Reason::MissingContext, Reason::IncorrectInformation]),
        "It missed the potassium trend.".into(),
    )
    .unwrap();

    // The first save can carry details with its rating.
    let with_details = saved(
        store
            .save_feedback(
                &chat,
                run,
                Rating::Down,
                Some(&details),
                &hash,
                &mut Gate(true),
            )
            .await
            .unwrap(),
    );
    assert_eq!(
        (
            with_details.rating,
            with_details.details.as_ref(),
            with_details.revision
        ),
        (Rating::Down, Some(&details), 1)
    );
    assert_eq!(
        store
            .save_feedback(&chat, run, Rating::Up, None, &hash, &mut Gate(false))
            .await
            .unwrap(),
        FeedbackSaved::Refused
    );
    assert_eq!(store.chat_feedback(&chat).await.unwrap()[run], with_details);

    // A new rating sent with the same details keeps them; one sent without
    // clears them.
    let rerated = saved(
        store
            .save_feedback(
                &chat,
                run,
                Rating::Up,
                Some(&details),
                &hash,
                &mut Gate(true),
            )
            .await
            .unwrap(),
    );
    assert_eq!(
        (rerated.rating, rerated.details.as_ref(), rerated.revision),
        (Rating::Up, Some(&details), 2)
    );
    let cleared = saved(rate(&store, &chat, run, Rating::Up).await);
    assert_eq!((cleared.details, cleared.revision), (None, 3));
    assert!(
        store
            .chat_feedback(&scope(&practice, "colleague", "chat"))
            .await
            .unwrap()
            .is_empty()
    );
    assert!(Details::new(BTreeSet::new(), "x".repeat(MAX_DETAILS_CHARS + 1)).is_none());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn evaluation_reads_every_row_with_its_run_and_response_text() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let mine = scope(&practice, "physician", "chat");
    let theirs = scope(&practice, "colleague", "chat");
    let my_runs = answered_chat(&store, &mine, 2).await;
    let their_runs = answered_chat(&store, &theirs, 1).await;
    for (scope, run, rating) in [
        (&mine, &my_runs[1], Rating::Up),
        (&mine, &my_runs[0], Rating::Down),
        (&theirs, &their_runs[0], Rating::Up),
    ] {
        saved(rate(&store, scope, run, rating).await);
    }
    let rows: Vec<_> = store
        .feedback_for_evaluation()
        .await
        .unwrap()
        .into_iter()
        .filter(|row| row.scope.practice == practice)
        .map(|row| {
            (
                row.scope.physician,
                row.run,
                row.feedback.rating,
                row.response_text,
            )
        })
        .collect();
    // Within a chat, responses read in the order Coda gave them.
    assert_eq!(
        rows,
        [
            (
                "colleague".into(),
                their_runs[0].clone(),
                Rating::Up,
                Some("reply to chat-0".into())
            ),
            (
                "physician".into(),
                my_runs[0].clone(),
                Rating::Down,
                Some("reply to chat-0".into())
            ),
            (
                "physician".into(),
                my_runs[1].clone(),
                Rating::Up,
                Some("reply to chat-1".into())
            ),
        ]
    );
}
