use super::super::*;
use super::{OpenAuthority, accept, delivery_input, delivery_scope};
use crate::runtime::delivery::Backoff;

/// Answers every request with the same text, through the finish.
pub(super) struct Answer(pub(super) &'static str);

impl Provider for Answer {
    async fn exchange(
        &mut self,
        _: &engine::recipe::Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn engine::recipe::Observer,
    ) -> Result<Turn, engine::Cause> {
        Ok(Turn {
            items: super::answered(self.0),
            receipt: Value::Null,
            usage: Default::default(),
        })
    }
}

/// Claims the pending work and runs it to its end with `provider`.
pub(super) async fn run(
    store: &Store,
    scope: &Scope,
    provider: &mut Answer,
    max_calls: u64,
) -> Claim {
    let claim = store.claim(scope, 60).await.unwrap().unwrap();
    PhysicianHost {
        store: store.clone(),
        claim: claim.clone(),
        authority: OpenAuthority,
        shown_steps: crate::runtime::ShownSteps::new(()),
    }
    .run(
        provider,
        engine::RunLimits {
            max_calls,
            ..engine::RunLimits::default()
        },
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    claim
}

fn reply_id(claim: &Claim) -> String {
    format!("{}:reply", claim.attempt)
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn each_published_reply_keeps_its_response_id_across_a_fresh_store() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = delivery_scope(&uuid::Uuid::new_v4().to_string(), "chat");
    accept(&store, &scope, &delivery_input("first"), 0)
        .await
        .unwrap();
    let first = run(&store, &scope, &mut Answer("Sodium is stable."), 8).await;
    // The next work reaches its request limit and keeps a partial answer,
    // which is not a response. Try again then publishes the work's reply.
    accept(&store, &scope, &delivery_input("second"), 0)
        .await
        .unwrap();
    let held = run(&store, &scope, &mut Answer("The last check did not"), 1).await;
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.status, WorkStatus::Held);
    assert!(state.messages.last().unwrap().partial());
    let run_id = state.run().unwrap().id.to_owned();
    assert!(matches!(
        store
            .resume_authorized(&scope, "again", vec![0; 32], &state.work, &run_id, None)
            .await
            .unwrap(),
        Resumed::Pending { .. }
    ));
    let answered = run(&store, &scope, &mut Answer("Potassium is low."), 8).await;
    drop(store);

    let store = Store::connect(&url).await.unwrap();
    let state = store.conversation(&scope).await.unwrap();
    let responses: Vec<_> = state
        .responses()
        .map(|r| (r.place, r.id.to_string(), r.text.to_owned()))
        .collect();
    assert_eq!(
        responses,
        [
            (1, reply_id(&first), "Sodium is stable.".into()),
            (4, reply_id(&answered), "Potassium is low.".into()),
        ]
    );
    assert_eq!(state.messages[3].text, "The last check did not");
    assert!(state.messages[3].partial());
    assert!(
        state.response(&ResponseId(reply_id(&held))).is_none(),
        "a partial answer is not a response"
    );
    assert_eq!(
        state
            .response(&ResponseId(reply_id(&answered)))
            .map(|r| r.place),
        Some(4)
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn try_again_refuses_another_physician_or_practice() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = delivery_scope(&uuid::Uuid::new_v4().to_string(), "chat");
    accept(&store, &scope, &delivery_input("turn"), 5)
        .await
        .unwrap();
    for _ in 0..harness::engine::MAX_FAILED_ATTEMPTS {
        let claim = store.claim(&scope, 60).await.unwrap().unwrap();
        store
            .settle_chat_attempt(
                &claim,
                harness::engine::AttemptFailure::Failed,
                Backoff::IMMEDIATE,
            )
            .await
            .unwrap();
    }
    let failed = store.conversation(&scope).await.unwrap();
    assert_eq!(failed.status, WorkStatus::Failed);
    let run_id = failed.run().unwrap().id.to_owned();

    let mut colleague = scope.clone();
    colleague.physician = "colleague".into();
    let mut elsewhere = scope.clone();
    elsewhere.practice = uuid::Uuid::new_v4().to_string();
    for (key, target) in [("colleague", &colleague), ("elsewhere", &elsewhere)] {
        assert_eq!(
            store
                .resume_authorized(target, key, vec![6; 32], &failed.work, &run_id, None)
                .await
                .unwrap(),
            Resumed::Refused,
            "{key}"
        );
    }
    let unchanged = store.conversation(&scope).await.unwrap();
    assert_eq!(unchanged.status, WorkStatus::Failed);
    assert_eq!(unchanged.session_hash.as_deref(), Some(&[5u8; 32][..]));
}

/// The physician's message and Coda's finished answer each keep the database
/// clock's reading from the transaction that saved them: between readings of
/// that clock taken before and after the save. The answer is saved in a later
/// clock second than the message, so neither bound can hold the other's time.
/// A later exchange's are not earlier than the first's.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn each_saved_message_keeps_the_database_clock_of_its_own_save() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = delivery_scope(&uuid::Uuid::new_v4().to_string(), "chat");
    let clock = async || {
        let tx = store.connection().begin().await.unwrap();
        let read = now(&tx).await.unwrap();
        tx.commit().await.unwrap();
        read
    };
    let saved = |state: &Conversation| -> Vec<i64> {
        state
            .messages
            .iter()
            .map(|message| message.saved_at.seconds())
            .collect()
    };

    let before = clock().await;
    accept(&store, &scope, &delivery_input("first"), 0)
        .await
        .unwrap();
    let accepted = clock().await;
    // The answer runs only once the clock has passed the second the message was saved in, so the
    // two saves fall in different seconds and each bound holds one save alone.
    let answering = loop {
        let read = clock().await;
        if read > accepted {
            break read;
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    };
    run(&store, &scope, &mut Answer("Sodium is stable."), 8).await;
    let finished = clock().await;
    let first = saved(&store.conversation(&scope).await.unwrap());
    let [asked, answered] = first[..] else {
        panic!("one message and its answer: {first:?}")
    };
    assert!(
        (before..=accepted).contains(&asked),
        "the message was saved at {asked}, outside {before}..={accepted}"
    );
    assert!(
        (answering..=finished).contains(&answered),
        "the answer was saved at {answered}, outside {answering}..={finished}"
    );

    accept(&store, &scope, &delivery_input("second"), 0)
        .await
        .unwrap();
    run(&store, &scope, &mut Answer("Potassium is low."), 8).await;
    let both = saved(&store.conversation(&scope).await.unwrap());
    assert_eq!(both.len(), 4, "{both:?}");
    assert!(
        both.is_sorted() && both[2] >= answered,
        "a later exchange was saved earlier: {both:?}"
    );
}
