//! Postgres proof that feedback stays on the exact response it rates, named
//! by its response id, across a fresh store; that every save replaces the
//! rating and details and increments the revision; and that only an
//! authorized save on one of the chat's responses writes anything.
use std::collections::BTreeSet;

use super::response_tests::{Answer, run};
use super::*;
use crate::runtime::feedback::{
    Details, Feedback, FeedbackSaved, MAX_DETAILS_CHARS, Rating, Reason,
};

/// What Coda answers each turn of a chat, in order.
const ANSWERS: [&str; 2] = ["Sodium is stable.", "Potassium is low."];

/// A chat in which each question got its own published reply, the replies
/// reading [`ANSWERS`] in order; returns their response ids in order.
async fn answered_chat(store: &Store, scope: &Scope, turns: usize) -> Vec<ResponseId> {
    for (turn, text) in ANSWERS.into_iter().take(turns).enumerate() {
        accept(
            store,
            scope,
            &delivery_input(&format!("{}-{turn}", scope.chat)),
            7,
        )
        .await
        .unwrap();
        run(store, scope, &mut Answer(text), 8).await;
    }
    let ids: Vec<_> = store
        .conversation(scope)
        .await
        .unwrap()
        .responses()
        .map(|response| response.id)
        .collect();
    assert_eq!(ids.len(), turns);
    ids
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

async fn rate(
    store: &Store,
    scope: &Scope,
    response: &ResponseId,
    rating: Rating,
) -> FeedbackSaved {
    store
        .save_feedback(scope, response, rating, None, &[7_u8; 32], &mut Gate(true))
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
    let responses = answered_chat(&store, &chat, 2).await;
    let first = saved(rate(&store, &chat, &responses[0], Rating::Up).await);
    assert_eq!(
        (first.rating, first.details.clone(), first.revision),
        (Rating::Up, None, 1)
    );
    assert!(first.saved_at > 0);
    saved(rate(&store, &chat, &responses[1], Rating::Down).await);

    drop(store);
    let store = Store::connect(&url).await.unwrap();
    let read = store.chat_feedback(&chat).await.unwrap();
    assert_eq!(read[&responses[0]].rating, Rating::Up);
    assert_eq!(read[&responses[1]].rating, Rating::Down);
    assert_eq!(read.len(), 2);

    // A repeat converges and the other thumb replaces the rating; every save
    // is a new revision.
    for (rating, revision) in [(Rating::Down, 2), (Rating::Down, 3)] {
        let again = saved(rate(&store, &chat, &responses[0], rating).await);
        assert_eq!((again.rating, again.revision), (Rating::Down, revision));
    }
    let read = store.chat_feedback(&chat).await.unwrap();
    assert_eq!((read.len(), read[&responses[0]].revision), (2, 3));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn only_an_authorized_save_on_a_response_of_the_chat_writes() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let chat = scope(&practice, "physician", "chat");
    let responses = answered_chat(&store, &chat, 1).await;
    let other = scope(&practice, "physician", "other");
    let elsewhere = answered_chat(&store, &other, 1).await;
    // Work that reached its request limit kept a partial answer, which is
    // not a response.
    accept(&store, &chat, &delivery_input("unanswered"), 7)
        .await
        .unwrap();
    let held = run(&store, &chat, &mut Answer("The last check did not"), 1).await;
    let partial = store.conversation(&chat).await.unwrap();
    assert!(partial.messages.last().unwrap().partial());
    let hash = [7_u8; 32];

    // An unknown id, a partial answer, another chat's response and a chat
    // that doesn't exist hold no response of this chat's.
    for (response, target) in [
        (ResponseId::from("no-such-response".to_owned()), &chat),
        (ResponseId(format!("{}:reply", held.attempt)), &chat),
        (elsewhere[0].clone(), &chat),
        (responses[0].clone(), &scope(&practice, "physician", "none")),
    ] {
        assert_eq!(
            rate(&store, target, &response, Rating::Up).await,
            FeedbackSaved::NotAResponse,
            "{response}"
        );
    }
    // Another physician's chat by the same name is not theirs to rate.
    let colleague = scope(&practice, "colleague", "chat");
    assert_eq!(
        rate(&store, &colleague, &responses[0], Rating::Up).await,
        FeedbackSaved::NotAResponse
    );
    // A refused authority and a malformed session write nothing.
    assert_eq!(
        store
            .save_feedback(
                &chat,
                &responses[0],
                Rating::Up,
                None,
                &hash,
                &mut Gate(false)
            )
            .await
            .unwrap(),
        FeedbackSaved::Refused
    );
    assert!(
        store
            .save_feedback(
                &chat,
                &responses[0],
                Rating::Up,
                None,
                &[1; 4],
                &mut Gate(true)
            )
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
    let response = &answered_chat(&store, &chat, 1).await[0];
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
                response,
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
            .save_feedback(&chat, response, Rating::Up, None, &hash, &mut Gate(false))
            .await
            .unwrap(),
        FeedbackSaved::Refused
    );
    assert_eq!(
        store.chat_feedback(&chat).await.unwrap()[response],
        with_details
    );

    // A new rating sent with the same details keeps them; one sent without
    // clears them.
    let rerated = saved(
        store
            .save_feedback(
                &chat,
                response,
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
    let cleared = saved(rate(&store, &chat, response, Rating::Up).await);
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
async fn evaluation_reads_every_row_with_its_response_and_text() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let mine = scope(&practice, "physician", "chat");
    let theirs = scope(&practice, "colleague", "chat");
    let my_responses = answered_chat(&store, &mine, 2).await;
    let their_responses = answered_chat(&store, &theirs, 1).await;
    for (scope, response, rating) in [
        (&mine, &my_responses[1], Rating::Up),
        (&mine, &my_responses[0], Rating::Down),
        (&theirs, &their_responses[0], Rating::Up),
    ] {
        saved(rate(&store, scope, response, rating).await);
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
    // Within a chat, responses read in the order Coda gave them.
    assert_eq!(
        rows,
        [
            (
                "colleague".into(),
                their_responses[0].clone(),
                Rating::Up,
                Some("Sodium is stable.".into())
            ),
            (
                "physician".into(),
                my_responses[0].clone(),
                Rating::Down,
                Some("Sodium is stable.".into())
            ),
            (
                "physician".into(),
                my_responses[1].clone(),
                Rating::Up,
                Some("Potassium is low.".into())
            ),
        ]
    );
}
