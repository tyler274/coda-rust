//! Postgres proof that feedback stays on the exact response it rates, named
//! by the run that published it, across a fresh store, and that only an
//! authorized save on one of the chat's responses writes anything.
use std::collections::BTreeSet;

use super::run_tests::answer;
use super::*;
use crate::runtime::feedback::{
    Details, DetailsSaved, Feedback, MAX_DETAILS_CHARS, Rating, RatingSaved, Reason,
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

fn saved(outcome: RatingSaved) -> Feedback {
    match outcome {
        RatingSaved::Saved(feedback) => feedback,
        other => panic!("rating not saved: {other:?}"),
    }
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
    let hash = [7_u8; 32];
    let first = saved(
        store
            .rate_response(&chat, &runs[0], Rating::Up, &hash, &mut Gate(true))
            .await
            .unwrap(),
    );
    assert_eq!((first.rating, first.details.clone()), (Rating::Up, None));
    assert!(first.rated_at > 0);
    saved(
        store
            .rate_response(&chat, &runs[1], Rating::Down, &hash, &mut Gate(true))
            .await
            .unwrap(),
    );

    drop(store);
    let store = Store::connect(&url).await.unwrap();
    let read = store.chat_feedback(&chat).await.unwrap();
    assert_eq!(read[&runs[0]].rating, Rating::Up);
    assert_eq!(read[&runs[1]].rating, Rating::Down);
    assert_eq!(read.len(), 2);

    // A repeat converges and the other thumb replaces the rating.
    for rating in [Rating::Down, Rating::Down] {
        let again = saved(
            store
                .rate_response(&chat, &runs[0], rating, &hash, &mut Gate(true))
                .await
                .unwrap(),
        );
        assert_eq!(again.rating, Rating::Down);
    }
    assert_eq!(store.chat_feedback(&chat).await.unwrap().len(), 2);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn only_an_authorized_rating_on_a_response_of_the_chat_writes() {
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
            store
                .rate_response(target, &run, Rating::Up, &hash, &mut Gate(true))
                .await
                .unwrap(),
            RatingSaved::NotAResponse,
            "{run}"
        );
    }
    // Another physician's chat by the same name is not theirs to rate.
    let colleague = scope(&practice, "colleague", "chat");
    assert_eq!(
        store
            .rate_response(&colleague, &runs[0], Rating::Up, &hash, &mut Gate(true))
            .await
            .unwrap(),
        RatingSaved::NotAResponse
    );
    // A refused authority and a malformed session write nothing.
    assert_eq!(
        store
            .rate_response(&chat, &runs[0], Rating::Up, &hash, &mut Gate(false))
            .await
            .unwrap(),
        RatingSaved::Refused
    );
    assert!(
        store
            .rate_response(&chat, &runs[0], Rating::Up, &[1; 4], &mut Gate(true))
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
async fn details_need_a_rating_and_survive_a_new_rating() {
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

    assert_eq!(
        store
            .save_feedback_details(&chat, run, &details, &hash, &mut Gate(true))
            .await
            .unwrap(),
        DetailsSaved::NotRated
    );
    saved(
        store
            .rate_response(&chat, run, Rating::Down, &hash, &mut Gate(true))
            .await
            .unwrap(),
    );
    assert_eq!(
        store
            .save_feedback_details(&chat, run, &details, &hash, &mut Gate(false))
            .await
            .unwrap(),
        DetailsSaved::Refused
    );
    assert_eq!(store.chat_feedback(&chat).await.unwrap()[run].details, None);
    let DetailsSaved::Saved(with_details) = store
        .save_feedback_details(&chat, run, &details, &hash, &mut Gate(true))
        .await
        .unwrap()
    else {
        panic!("details not saved");
    };
    assert_eq!(
        with_details.details.as_ref().map(|saved| &saved.details),
        Some(&details)
    );

    let rerated = saved(
        store
            .rate_response(&chat, run, Rating::Up, &hash, &mut Gate(true))
            .await
            .unwrap(),
    );
    assert_eq!(
        (rerated.rating, rerated.details),
        (Rating::Up, with_details.details)
    );
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
    let hash = [7_u8; 32];
    for (scope, run, rating) in [
        (&mine, &my_runs[1], Rating::Up),
        (&mine, &my_runs[0], Rating::Down),
        (&theirs, &their_runs[0], Rating::Up),
    ] {
        saved(
            store
                .rate_response(scope, run, rating, &hash, &mut Gate(true))
                .await
                .unwrap(),
        );
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
