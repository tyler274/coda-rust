//! Postgres proof that feedback stays on the exact response it rates, across
//! a fresh store, and that only an authorized save on one of Coda's responses
//! writes anything.
use std::collections::BTreeSet;

use super::*;
use crate::runtime::feedback::{
    Details, DetailsSaved, Feedback, MAX_DETAILS_CHARS, Rating, RatingSaved, Reason,
};

/// A chat whose messages alternate physician and Coda, ending with Coda.
async fn answered_chat(store: &Store, scope: &Scope, turns: usize) {
    for turn in 0..turns {
        let input = Input {
            key: format!("{}-{turn}", scope.chat),
            text: format!("Question {turn}"),
            timezone: "UTC".into(),
        };
        assert_eq!(store.accept(scope, &input).await.unwrap(), Accepted::New);
        let (tx, mut state, _) = store.locked(scope).await.unwrap();
        state.messages.push(Message {
            role: "assistant".into(),
            text: format!("Answer {turn}"),
        });
        state.status = "completed".into();
        Store::save(tx, scope, &state).await.unwrap();
    }
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
    answered_chat(&store, &chat, 2).await;
    let hash = [7_u8; 32];
    // Messages 1 and 3 are Coda's answers.
    let first = saved(
        store
            .rate_response(&chat, 1, Rating::Up, &hash, &mut Gate(true))
            .await
            .unwrap(),
    );
    assert_eq!((first.rating, first.details.clone()), (Rating::Up, None));
    assert!(first.rated_at > 0);
    saved(
        store
            .rate_response(&chat, 3, Rating::Down, &hash, &mut Gate(true))
            .await
            .unwrap(),
    );

    drop(store);
    let store = Store::connect(&url).await.unwrap();
    let read = store.chat_feedback(&chat).await.unwrap();
    let ratings: Vec<_> = read.iter().map(|(at, f)| (*at, f.rating)).collect();
    assert_eq!(ratings, [(1, Rating::Up), (3, Rating::Down)]);

    // A repeat converges and the other thumb replaces the rating.
    for rating in [Rating::Down, Rating::Down] {
        let again = saved(
            store
                .rate_response(&chat, 1, rating, &hash, &mut Gate(true))
                .await
                .unwrap(),
        );
        assert_eq!(again.rating, Rating::Down);
    }
    assert_eq!(store.chat_feedback(&chat).await.unwrap().len(), 2);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn only_an_authorized_rating_on_codas_response_writes() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let chat = scope(&practice, "physician", "chat");
    answered_chat(&store, &chat, 1).await;
    let hash = [7_u8; 32];

    // The physician's own message, a place past the end, and a chat that
    // doesn't exist hold no response of Coda's.
    for (at, scope) in [
        (0, &chat),
        (2, &chat),
        (1, &scope(&practice, "physician", "none")),
    ] {
        assert_eq!(
            store
                .rate_response(scope, at, Rating::Up, &hash, &mut Gate(true))
                .await
                .unwrap(),
            RatingSaved::NotAResponse
        );
    }
    // Another physician's chat at the same place is not theirs to rate.
    let colleague = scope(&practice, "colleague", "chat");
    assert_eq!(
        store
            .rate_response(&colleague, 1, Rating::Up, &hash, &mut Gate(true))
            .await
            .unwrap(),
        RatingSaved::NotAResponse
    );
    // A refused authority and a malformed session write nothing.
    assert_eq!(
        store
            .rate_response(&chat, 1, Rating::Up, &hash, &mut Gate(false))
            .await
            .unwrap(),
        RatingSaved::Refused
    );
    assert!(
        store
            .rate_response(&chat, 1, Rating::Up, &[1; 4], &mut Gate(true))
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
    answered_chat(&store, &chat, 1).await;
    let hash = [7_u8; 32];
    let details = Details::new(
        BTreeSet::from([Reason::MissingContext, Reason::IncorrectInformation]),
        "It missed the potassium trend.".into(),
    )
    .unwrap();

    assert_eq!(
        store
            .save_feedback_details(&chat, 1, &details, &hash, &mut Gate(true))
            .await
            .unwrap(),
        DetailsSaved::NotRated
    );
    saved(
        store
            .rate_response(&chat, 1, Rating::Down, &hash, &mut Gate(true))
            .await
            .unwrap(),
    );
    assert_eq!(
        store
            .save_feedback_details(&chat, 1, &details, &hash, &mut Gate(false))
            .await
            .unwrap(),
        DetailsSaved::Refused
    );
    assert_eq!(store.chat_feedback(&chat).await.unwrap()[&1].details, None);
    let DetailsSaved::Saved(with_details) = store
        .save_feedback_details(&chat, 1, &details, &hash, &mut Gate(true))
        .await
        .unwrap()
    else {
        panic!("details not saved");
    };
    assert_eq!(with_details.details.as_ref(), Some(&details));
    assert!(with_details.details_saved_at.is_some());

    let rerated = saved(
        store
            .rate_response(&chat, 1, Rating::Up, &hash, &mut Gate(true))
            .await
            .unwrap(),
    );
    assert_eq!(
        (rerated.rating, rerated.details.as_ref()),
        (Rating::Up, Some(&details))
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
async fn evaluation_reads_every_row_with_its_response_text() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let mine = scope(&practice, "physician", "chat");
    let theirs = scope(&practice, "colleague", "chat");
    answered_chat(&store, &mine, 2).await;
    answered_chat(&store, &theirs, 1).await;
    let hash = [7_u8; 32];
    for (scope, at, rating) in [
        (&mine, 3, Rating::Up),
        (&mine, 1, Rating::Down),
        (&theirs, 1, Rating::Up),
    ] {
        saved(
            store
                .rate_response(scope, at, rating, &hash, &mut Gate(true))
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
                row.response,
                row.feedback.rating,
                row.response_text,
            )
        })
        .collect();
    assert_eq!(
        rows,
        [
            ("colleague".into(), 1, Rating::Up, Some("Answer 0".into())),
            ("physician".into(), 1, Rating::Down, Some("Answer 0".into())),
            ("physician".into(), 3, Rating::Up, Some("Answer 1".into())),
        ]
    );
}
