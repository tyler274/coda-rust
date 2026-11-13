use harness::engine::MAX_FAILED_ATTEMPTS;
#[path = "tests/feedback.rs"]
mod feedback_tests;
#[path = "tests/migration.rs"]
mod migration_tests;
#[path = "tests/read_snapshot.rs"]
mod read_snapshot;
#[path = "tests/reply.rs"]
mod reply_tests;
#[path = "tests/responses.rs"]
mod response_tests;
#[path = "tests/sources.rs"]
mod source_tests;

use super::delivery::{Backoff, Delivery, DeliveryState};
use super::entities::command;
use super::*;
use harness::engine::recipe::Request;

#[test]
fn a_pool_timeout_is_not_a_failed_query() {
    use sea_orm::{ConnAcquireErr, RuntimeErr, SqlxError};
    let acquire = DbErr::ConnectionAcquire(ConnAcquireErr::Timeout);
    assert_eq!(DbFailure::of(&acquire), DbFailure::PoolTimeout);
    let connect = DbErr::Conn(RuntimeErr::SqlxError(SqlxError::PoolTimedOut.into()));
    assert_eq!(DbFailure::of(&connect), DbFailure::PoolTimeout);
    let closed = DbErr::Conn(RuntimeErr::SqlxError(
        SqlxError::Io(std::io::Error::from(std::io::ErrorKind::ConnectionReset)).into(),
    ));
    assert_eq!(DbFailure::of(&closed), DbFailure::Connection);
    let refused = DbErr::Query(RuntimeErr::Internal("refused".into()));
    assert_eq!(DbFailure::of(&refused), DbFailure::Query);
    for failure in [acquire, connect, closed, refused] {
        assert_eq!(sqlstate(&failure), None);
    }
    let kinds = [
        DbFailure::PoolTimeout,
        DbFailure::Connection,
        DbFailure::SchemaMissing,
        DbFailure::Query,
    ]
    .map(DbFailure::error_type);
    assert_eq!(kinds, ["pool_timeout", "connection", "schema", "database"]);
}

#[tokio::test]
#[ignore = "requires a disposable Postgres in PHYSICIAN_RUNTIME_TEST_DATABASE_URL"]
async fn a_missing_table_is_the_only_failure_that_names_the_schema() {
    use sea_orm::sea_query::{Alias, Expr, ExprTrait, Query};
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    let db = store.connection();
    let missing = db
        .query_one(
            &Query::select()
                .expr(Expr::val(1))
                .from(Alias::new("physician_runtime_no_such_table"))
                .to_owned(),
        )
        .await
        .unwrap_err();
    assert_eq!(DbFailure::of(&missing), DbFailure::SchemaMissing);
    assert_eq!(sqlstate(&missing).as_deref(), Some("42P01"));
    let refused = db
        .query_one(&Query::select().expr(Expr::val(1).div(0)).to_owned())
        .await
        .unwrap_err();
    assert_eq!(DbFailure::of(&refused), DbFailure::Query);
    assert_eq!(sqlstate(&refused).as_deref(), Some("22012"));
}

/// A backend ended while its statement runs reports the end through
/// Postgres's own error response, which reaches the caller as a failed query.
/// It is still a lost connection, not a refused query.
#[tokio::test]
#[ignore = "requires a disposable Postgres in PHYSICIAN_RUNTIME_TEST_DATABASE_URL"]
async fn a_backend_ended_mid_statement_is_a_lost_connection() {
    use sea_orm::sea_query::{Alias, Expr, Func, Query};
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    let db = store.connection();
    let tx = db.begin().await.unwrap();
    let pid: i32 = tx
        .query_one(
            &Query::select()
                .expr_as(Func::cust(Alias::new("pg_backend_pid")), Alias::new("pid"))
                .to_owned(),
        )
        .await
        .unwrap()
        .unwrap()
        .try_get("", "pid")
        .unwrap();
    let sleep = Query::select()
        .expr(Func::cust(Alias::new("pg_sleep")).arg(30))
        .to_owned();
    let sleeping = tx.query_one(&sleep);
    let end = async {
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        db.query_one(
            &Query::select()
                .expr(Func::cust(Alias::new("pg_terminate_backend")).arg(Expr::val(pid)))
                .to_owned(),
        )
        .await
    };
    let (slept, ended) = tokio::join!(sleeping, end);
    ended.unwrap();
    let lost = slept.unwrap_err();
    assert_eq!(sqlstate(&lost).as_deref(), Some("57P01"), "{lost}");
    assert_eq!(DbFailure::of(&lost), DbFailure::Connection, "{lost}");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn concurrent_commands_and_claims_serialize_on_the_composite_conversation_key() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "chat".into(),
    };
    let first = Input {
        key: "first".into(),
        text: "first".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    let second = Input {
        key: "second".into(),
        text: "second".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    let accepted_recipe = test_chat_recipe();
    let accepted_skills = test_skills();
    let (a, b) = tokio::join!(
        store.accept_authorized(
            &scope,
            &first,
            vec![0; 32],
            &accepted_skills,
            &accepted_recipe,
            None
        ),
        store.accept_authorized(
            &scope,
            &second,
            vec![0; 32],
            &accepted_skills,
            &accepted_recipe,
            None
        )
    );
    assert_eq!(a.unwrap(), Accepted::New);
    assert_eq!(b.unwrap(), Accepted::New);
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.revision, 2);
    assert_eq!(state.messages.len(), 2);
    let (a, b) = tokio::join!(store.claim(&scope, 60), store.claim(&scope, 60));
    assert_eq!(
        [a.unwrap(), b.unwrap()]
            .iter()
            .filter(|claim| claim.is_some())
            .count(),
        1
    );
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.attempts.len(), 1);
    assert_eq!(state.attempts[0].revision, 2);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn pending_keyset_orders_all_scope_keys_and_filters_terminal_and_live_work() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let mut expected = Vec::new();
    // Insert out of order; the cursor must compare every composite-key component.
    for (physician, workspace, chat, status, expiry) in [
        ("b", "a", "a", WorkStatus::Pending, None),
        ("a", "b", "a", WorkStatus::Active, Some(0)),
        ("a", "a", "b", WorkStatus::Pending, None),
        ("a", "a", "a", WorkStatus::Pending, None),
        ("a", "a", "held", WorkStatus::Held, None),
        ("a", "a", "failed", WorkStatus::Failed, None),
        ("a", "a", "done", WorkStatus::Completed, None),
        ("a", "a", "live", WorkStatus::Active, Some(i64::MAX)),
        ("a", "a", "empty", WorkStatus::Active, None),
    ] {
        let scope = Scope {
            practice: practice.clone(),
            physician: physician.into(),
            workspace: workspace.into(),
            chat: chat.into(),
        };
        let mut state = Conversation {
            status,
            ..Conversation::new(
                serde_json::from_value(serde_json::json!("fixture-package")).unwrap(),
            )
        };
        if let Some(expires) = expiry {
            // Only the final attempt's expiry determines eligibility.
            state.attempts = vec![
                Attempt {
                    id: "old".into(),
                    predecessor: None,
                    work: "work".into(),
                    revision: 1,
                    status: AttemptStatus::Interrupted,
                    expires: i64::MAX,
                    events: vec![],
                },
                Attempt {
                    id: "latest".into(),
                    predecessor: Some("old".into()),
                    work: "work".into(),
                    revision: 1,
                    status: AttemptStatus::Active,
                    expires,
                    events: vec![],
                },
            ];
        }
        control::save_chat_attempts(&store.db, &scope, &state.attempts)
            .await
            .unwrap();
        conversation::Entity::insert(conversation_model(&scope, &state).unwrap())
            .exec(&store.db)
            .await
            .unwrap();
        if status == WorkStatus::Pending || expiry == Some(0) {
            expected.push(scope);
        }
    }
    expected.sort_by_key(keys);
    let start = Scope {
        practice,
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    assert!(
        store
            .pending_batch(0, Some(&start))
            .await
            .unwrap()
            .is_empty()
    );
    let first = store.pending_batch(2, Some(&start)).await.unwrap();
    assert_eq!(first, expected[..2]);
    let second = store.pending_batch(2, first.last()).await.unwrap();
    assert_eq!(second, expected[2..]);
    let rest = store.pending_batch(100, second.last()).await.unwrap();
    assert!(rest.iter().all(|scope| scope.practice != start.practice));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn corrupt_conversation_rolls_back_receipt_and_conflicts_never_overwrite_it() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "chat".into(),
    };
    let input = Input {
        key: "receipt".into(),
        text: "original".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    let mut corrupt = conversation_model(
        &scope,
        &Conversation::new(serde_json::from_value(serde_json::json!("fixture-package")).unwrap()),
    )
    .unwrap();
    corrupt.state = Set(serde_json::json!({}));
    conversation::Entity::insert(corrupt)
        .exec(&store.db)
        .await
        .unwrap();
    assert!(accept(&store, &scope, &input, 0).await.is_err());
    assert!(
        command::Entity::find_by_id((
            scope.practice.clone(),
            scope.physician.clone(),
            input.key.clone()
        ))
        .one(&store.db)
        .await
        .unwrap()
        .is_none()
    );
    conversation_model(
        &scope,
        &Conversation::new(serde_json::from_value(serde_json::json!("fixture-package")).unwrap()),
    )
    .unwrap()
    .update(&store.db)
    .await
    .unwrap();
    assert_eq!(
        accept(&store, &scope, &input, 0).await.unwrap(),
        Accepted::New
    );
    let mut other_chat = scope.clone();
    other_chat.chat = "different-chat".into();
    assert!(accept(&store, &other_chat, &input, 0).await.is_err());
    assert!(store.conversation(&other_chat).await.is_err());
    let mut collision = input.clone();
    collision.timezone = "America/New_York".into();
    assert!(accept(&store, &scope, &collision, 0).await.is_err());
    assert_eq!(
        accept(&store, &scope, &input, 0).await.unwrap(),
        Accepted::Duplicate
    );
    assert_eq!(store.conversation(&scope).await.unwrap().revision, 1);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn expired_attempt_recovery_preserves_accepted_work_and_evidence() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let s = Store::connect(&url).await.unwrap();
    s.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "chat".into(),
    };
    let input = Input {
        key: "command".into(),
        text: "accepted work".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    accept(&s, &scope, &input, 0).await.unwrap();
    let first = s.claim(&scope, 60).await.unwrap().unwrap();
    let display = s.snapshot().await.unwrap();
    let control_before = conversation::Entity::find_by_id(keys(&scope))
        .one(&display)
        .await
        .unwrap()
        .unwrap();
    s.persist(
        &first,
        engine::Event {
            attempt_id: first.attempt.clone(),
            ordinal: 1,
            sequence: 1,
            kind: engine::EventKind::ProviderStarted {
                dispatched_at_unix_ms: 1,
                request: engine::SavedRequest {
                    added: vec![],
                    items: vec![],
                },
                context: Default::default(),
            },
        },
    )
    .await
    .unwrap();
    let control_after = conversation::Entity::find_by_id(keys(&scope))
        .one(&s.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        control_after, control_before,
        "appending execution evidence must not rewrite conversation control"
    );
    let mut old_view = Store::conversation_control(&display, &scope, control_before)
        .await
        .unwrap();
    Store::conversation_events(&display, &scope, &mut old_view)
        .await
        .unwrap();
    assert!(
        old_view.attempts[0].events.is_empty(),
        "one display snapshot cannot join a later event to earlier control state"
    );
    display.commit().await.unwrap();
    assert_eq!(
        s.read(&scope).await.unwrap().unwrap().attempts[0]
            .events
            .len(),
        1
    );
    s.assert_current(&first).await.unwrap();
    // Establish expiry only after real evidence is durable. A one-second lease
    // can expire before persist() on a busy runner because the DB clock floors
    // to seconds. Keep fixture control at the storage owner, with no API seam.
    let (tx, mut state, _) = s.locked(&scope).await.unwrap();
    state.attempts.last_mut().unwrap().expires = 0;
    Store::save(tx, &scope, &state).await.unwrap();
    let dispatch = || engine::Event {
        attempt_id: first.attempt.clone(),
        ordinal: 1,
        sequence: 2,
        kind: engine::EventKind::ToolStarted(engine::Invocation {
            id: "late".into(),
            name: "skill_view".into(),
            arguments: "{}".into(),
        }),
    };
    let expired = s.persist(&first, dispatch()).await.unwrap_err();
    assert!(
        matches!(
            expired,
            engine::ProductError::NotCurrent(engine::NotCurrent::Lost)
        ),
        "an attempt whose lease ran out has lost the work at a write: {expired}"
    );
    drop(s);
    let s = Store::connect(&url).await.unwrap();
    // Read from this test's practice onward: other tests leave pending chats
    // in the shared database, and the first page in key order may be theirs.
    let from = Scope {
        practice: scope.practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    assert!(
        s.pending_batch(100, Some(&from))
            .await
            .unwrap()
            .contains(&scope)
    );
    let next = s.claim(&scope, 60).await.unwrap().unwrap();
    assert_ne!(first.attempt, next.attempt);
    let lost = s.assert_current(&first).await.unwrap_err();
    assert!(
        matches!(
            lost,
            engine::ProductError::NotCurrent(engine::NotCurrent::Lost)
        ),
        "a replaced attempt has lost ownership: {lost}"
    );
    let replaced = s.persist(&first, dispatch()).await.unwrap_err();
    assert!(
        matches!(
            replaced,
            engine::ProductError::NotCurrent(engine::NotCurrent::Lost)
        ),
        "a replaced attempt has lost the work at a write: {replaced}"
    );
    assert!(s.renew_claim(&first).await.is_err());
    s.renew_claim(&next).await.unwrap();
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(state.attempts[0].status, AttemptStatus::Interrupted);
    assert_eq!(state.attempts[0].events.len(), 1);
    assert_eq!(
        state.attempts[1].predecessor.as_deref(),
        Some(first.attempt.as_str())
    );
    assert_eq!(state.attempts[1].work, state.attempts[0].work);
    assert_eq!(state.messages[0].text, input.text);
    assert_eq!(state.messages.len(), 1);
}

/// A turn that ends the chat: `text` as narration, then the same text handed
/// in through the finish as an answer that cites no source. At a limit only
/// the narration is kept.
fn answered(text: &str) -> Vec<Item> {
    vec![
        Item::assistant(text),
        Item::Call(engine::Invocation {
            id: "finish".into(),
            name: crate::physician_sources::citations::FINISH_TOOL.into(),
            arguments: serde_json::json!({
                "answer": text,
                "quotes": [],
                "descriptions": [],
                "basis": "Fixture basis.",
                "cites_none": true
            })
            .to_string(),
        }),
    ]
}

fn delivery_scope(practice: &str, chat: &str) -> Scope {
    Scope {
        practice: practice.into(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: chat.into(),
    }
}

fn delivery_input(key: &str) -> Input {
    Input {
        key: key.into(),
        text: key.into(),
        timezone: "UTC".into(),
        scenario: None,
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn queue_delivery_is_fenced_to_its_input_and_recovers_through_typed_owners() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let scope = delivery_scope(&practice, "chat");
    accept(&store, &scope, &delivery_input("first"), 7)
        .await
        .unwrap();
    let ours = |page: &delivery::DispatchPage| -> Vec<Delivery> {
        page.deliveries
            .iter()
            .filter(|d| d.scope.practice == practice)
            .cloned()
            .collect()
    };
    let first = Delivery::chat_turn(scope.clone(), "first".into());
    let page = store.dispatch_due(100, Some(&start)).await.unwrap();
    assert_eq!(ours(&page), vec![first.clone()]);
    assert_eq!(
        store.delivery_state(&first).await.unwrap(),
        DeliveryState::Due
    );

    // A newer accepted input supersedes the old wakeup: it can never claim,
    // and dropping it is safe because the newer input is due in its place.
    accept(&store, &scope, &delivery_input("second"), 7)
        .await
        .unwrap();
    assert!(store.claim_delivery(&first, 60).await.unwrap().is_none());
    assert_eq!(
        store.delivery_state(&first).await.unwrap(),
        DeliveryState::Settled
    );
    let second = Delivery::chat_turn(scope.clone(), "second".into());
    assert_eq!(
        ours(&store.dispatch_due(100, Some(&start)).await.unwrap()),
        vec![second.clone()]
    );

    // Duplicate wakeups find no second live claim.
    let claim = store.claim_delivery(&second, 60).await.unwrap().unwrap();
    assert!(store.claim_delivery(&second, 60).await.unwrap().is_none());
    store.renew_claim(&claim).await.unwrap();
    assert_eq!(
        store.delivery_state(&second).await.unwrap(),
        DeliveryState::Claimed
    );

    // A failed run is released to be claimed again as a successor attempt,
    // and its history says it failed: a lost lease is "interrupted" instead.
    store
        .settle_chat_attempt(
            &claim,
            harness::engine::AttemptFailure::Failed,
            Backoff::IMMEDIATE,
        )
        .await
        .unwrap();
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.status, WorkStatus::Pending);
    assert_eq!(state.attempts.last().unwrap().status, AttemptStatus::Failed);
    assert_eq!(
        store.delivery_state(&second).await.unwrap(),
        DeliveryState::Due
    );
    assert!(store.renew_claim(&claim).await.is_err());
    let retry = store.claim_delivery(&second, 60).await.unwrap().unwrap();
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(
        state.attempts.last().unwrap().predecessor.as_deref(),
        Some(claim.attempt.as_str())
    );

    // Intentional Stop settles the delivery and cannot be reopened by retry.
    assert_eq!(
        store
            .stop_authorized(&scope, "stop-delivery", vec![7; 32], &state.work)
            .await
            .unwrap(),
        Stopped::Held
    );
    store
        .settle_chat_attempt(
            &retry,
            harness::engine::AttemptFailure::Failed,
            Backoff::IMMEDIATE,
        )
        .await
        .unwrap();
    assert_eq!(
        store.conversation(&scope).await.unwrap().status,
        WorkStatus::Held
    );
    assert_eq!(
        store.delivery_state(&second).await.unwrap(),
        DeliveryState::Settled
    );
    assert!(ours(&store.dispatch_due(100, Some(&start)).await.unwrap()).is_empty());
    assert!(store.dispatch_due(0, None).await.is_err());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn repeated_failures_end_the_input_as_failed_until_resume_or_new_input() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let ours = |page: delivery::DispatchPage| -> Vec<Delivery> {
        page.deliveries
            .into_iter()
            .filter(|d| d.scope.practice == practice)
            .collect()
    };
    let scope = delivery_scope(&practice, "chat");
    accept(&store, &scope, &delivery_input("turn"), 5)
        .await
        .unwrap();
    let turn = Delivery::chat_turn(scope.clone(), "turn".into());
    for failure in 1..MAX_FAILED_ATTEMPTS {
        let claim = store.claim_delivery(&turn, 60).await.unwrap().unwrap();
        store
            .settle_chat_attempt(
                &claim,
                harness::engine::AttemptFailure::Failed,
                Backoff::IMMEDIATE,
            )
            .await
            .unwrap();
        let state = store.conversation(&scope).await.unwrap();
        assert_eq!(
            state.status,
            WorkStatus::Pending,
            "failure {failure} still retries"
        );
        assert_eq!(state.failures, failure);
    }
    // The last allowed failure ends the work; recovery has given up.
    let claim = store.claim_delivery(&turn, 60).await.unwrap().unwrap();
    store
        .settle_chat_attempt(
            &claim,
            harness::engine::AttemptFailure::Failed,
            Backoff::IMMEDIATE,
        )
        .await
        .unwrap();
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.status, WorkStatus::Failed);
    assert_eq!(state.failures, MAX_FAILED_ATTEMPTS);
    assert!(store.claim_delivery(&turn, 60).await.unwrap().is_none());
    assert!(
        ours(store.dispatch_due(100, Some(&start)).await.unwrap()).is_empty(),
        "the due-work read must not return work that recovery gave up on"
    );
    // Failed is neither settled nor runnable: a worker that hears of it
    // claims nothing and tells open pages to refetch.
    assert_eq!(
        store.delivery_state(&turn).await.unwrap(),
        DeliveryState::Failed
    );

    // Resume is the physician's Try again: a fresh budget for the same input.
    let resumed = store
        .resume_authorized(
            &scope,
            "try-again",
            vec![5; 32],
            &state.work,
            state.run().unwrap().id,
            None,
        )
        .await
        .unwrap();
    assert!(matches!(resumed, crate::runtime::Resumed::Pending { .. }));
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!((state.status, state.failures), (WorkStatus::Pending, 0));
    assert_eq!(
        ours(store.dispatch_due(100, Some(&start)).await.unwrap()),
        vec![turn.clone()]
    );

    // A worker that dies holding the lease counts as a failure as well: each
    // later claim records the expired lease before starting a successor.
    for _ in 0..MAX_FAILED_ATTEMPTS {
        store.claim_delivery(&turn, 60).await.unwrap().unwrap();
        let (tx, mut state, _) = store.locked(&scope).await.unwrap();
        state.attempts.last_mut().unwrap().expires = 0;
        Store::save(tx, &scope, &state).await.unwrap();
    }
    assert!(store.claim_delivery(&turn, 60).await.unwrap().is_none());
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.status, WorkStatus::Failed);
    assert_eq!(state.failures, MAX_FAILED_ATTEMPTS);
    assert_eq!(
        store.delivery_state(&turn).await.unwrap(),
        DeliveryState::Failed
    );

    // New input starts its own budget.
    accept(&store, &scope, &delivery_input("next"), 5)
        .await
        .unwrap();
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!((state.status, state.failures), (WorkStatus::Pending, 0));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn try_again_resumes_only_the_named_failed_run_and_never_completed_work() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let ours = |page: delivery::DispatchPage| -> Vec<Delivery> {
        page.deliveries
            .into_iter()
            .filter(|d| d.scope.practice == practice)
            .collect()
    };
    let scope = delivery_scope(&practice, "chat");
    accept(&store, &scope, &delivery_input("turn"), 5)
        .await
        .unwrap();
    let turn = Delivery::chat_turn(scope.clone(), "turn".into());
    // Pending work is not failed: nothing to try again.
    let pending = store.conversation(&scope).await.unwrap();
    assert_eq!(
        store
            .resume_authorized(&scope, "early", vec![6; 32], &pending.work, "none", None)
            .await
            .unwrap(),
        Resumed::Refused
    );
    for _ in 0..MAX_FAILED_ATTEMPTS {
        let claim = store.claim_delivery(&turn, 60).await.unwrap().unwrap();
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
    let work = failed.work.clone();
    let run = failed.run().unwrap().id.to_owned();
    let earlier = failed.attempts.first().unwrap().id.clone();

    // A wrong run, wrong work, or another chat is refused and changes nothing.
    for (key, target, work_id, run_id) in [
        ("wrong-run", &scope, work.as_str(), earlier.as_str()),
        ("wrong-work", &scope, "other-work", run.as_str()),
        (
            "wrong-chat",
            &delivery_scope(&practice, "other"),
            work.as_str(),
            run.as_str(),
        ),
    ] {
        assert_eq!(
            store
                .resume_authorized(target, key, vec![6; 32], work_id, run_id, None)
                .await
                .unwrap(),
            Resumed::Refused,
            "{key}"
        );
    }
    let unchanged = store.conversation(&scope).await.unwrap();
    assert_eq!(
        (unchanged.status, unchanged.failures),
        (WorkStatus::Failed, MAX_FAILED_ATTEMPTS)
    );
    assert!(
        store
            .resume_authorized(&scope, "short", vec![6; 4], &work, &run, None)
            .await
            .is_err()
    );
    for (key, work_id, run_id) in [
        ("", work.as_str(), run.as_str()),
        ("blank-work", " ", run.as_str()),
        ("blank-run", work.as_str(), ""),
    ] {
        assert_eq!(
            store
                .resume_authorized(&scope, key, vec![6; 32], work_id, run_id, None)
                .await
                .unwrap(),
            Resumed::Refused,
            "{key:?}"
        );
    }
    assert_eq!(
        store.conversation(&scope).await.unwrap().status,
        WorkStatus::Failed
    );

    // The named failed run resumes with a fresh budget and the current session.
    assert_eq!(
        store
            .resume_authorized(&scope, "again", vec![6; 32], &work, &run, None)
            .await
            .unwrap(),
        Resumed::Pending {
            delivery: Delivery::chat_turn(scope.clone(), "turn".into())
        }
    );
    let resumed = store.conversation(&scope).await.unwrap();
    assert_eq!((resumed.status, resumed.failures), (WorkStatus::Pending, 0));
    assert_eq!(resumed.session_hash.as_deref(), Some(&[6u8; 32][..]));
    assert_eq!(resumed.work, work, "Try again keeps the work identity");
    assert_eq!(
        ours(store.dispatch_due(100, Some(&start)).await.unwrap()),
        vec![turn.clone()]
    );
    // The same request is a duplicate; the same key for another target is not.
    assert_eq!(
        store
            .resume_authorized(&scope, "again", vec![6; 32], &work, &run, None)
            .await
            .unwrap(),
        Resumed::Duplicate
    );
    assert_eq!(
        store
            .resume_authorized(&scope, "again", vec![6; 32], &work, &earlier, None)
            .await
            .unwrap(),
        Resumed::Refused
    );

    // Completed work is never reopened, even with its last run named.
    let claim = store.claim_delivery(&turn, 60).await.unwrap().unwrap();
    let (tx, mut state, _) = store.locked(&scope).await.unwrap();
    state.attempts.last_mut().unwrap().status = AttemptStatus::Completed;
    state.status = WorkStatus::Completed;
    Store::save(tx, &scope, &state).await.unwrap();
    assert_eq!(
        store
            .resume_authorized(
                &scope,
                "after-done",
                vec![6; 32],
                &work,
                &claim.attempt,
                None
            )
            .await
            .unwrap(),
        Resumed::Refused
    );
    assert_eq!(
        store.conversation(&scope).await.unwrap().status,
        WorkStatus::Completed
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn stop_withdraws_the_running_attempt_and_a_new_message_supersedes_it() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();

    let stopped = delivery_scope(&practice, "stopped");
    accept(&store, &stopped, &delivery_input("stop-turn"), 5)
        .await
        .unwrap();
    let claim = store.claim(&stopped, 60).await.unwrap().unwrap();
    store.assert_current(&claim).await.unwrap();
    let work = store.conversation(&stopped).await.unwrap().work;
    assert_eq!(
        store
            .stop_authorized(&stopped, "stop", vec![6; 32], &work)
            .await
            .unwrap(),
        Stopped::Held
    );
    assert_eq!(
        store.conversation(&stopped).await.unwrap().revision,
        claim.revision,
        "Stop leaves the input revision"
    );
    let error = store.assert_current(&claim).await.unwrap_err();
    assert!(
        matches!(
            error,
            engine::ProductError::NotCurrent(engine::NotCurrent::Withdrawn)
        ),
        "{error}"
    );
    assert_eq!(
        engine::AttemptFailure::of(
            &engine::Failure::from(error),
            &engine::Cancellation::default()
        ),
        engine::AttemptFailure::Interrupted
    );

    let steered = delivery_scope(&practice, "steered");
    accept(&store, &steered, &delivery_input("first-turn"), 5)
        .await
        .unwrap();
    let claim = store.claim(&steered, 60).await.unwrap().unwrap();
    accept(&store, &steered, &delivery_input("second-turn"), 5)
        .await
        .unwrap();
    assert!(
        store.conversation(&steered).await.unwrap().revision > claim.revision,
        "a new message raises the input revision"
    );
    let error = store.assert_current(&claim).await.unwrap_err();
    assert!(
        matches!(
            error,
            engine::ProductError::NotCurrent(engine::NotCurrent::Superseded)
        ),
        "{error}"
    );
    let work = store.conversation(&steered).await.unwrap().work;
    assert_eq!(
        store
            .stop_authorized(&steered, "stop-steered", vec![6; 32], &work)
            .await
            .unwrap(),
        Stopped::Held
    );
    let error = store.assert_current(&claim).await.unwrap_err();
    assert!(
        matches!(
            error,
            engine::ProductError::NotCurrent(engine::NotCurrent::Withdrawn)
        ),
        "Stop wins over the waiting message: {error}"
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn stop_then_a_new_message_withdraws_the_running_attempt_and_the_message_runs_next() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = delivery_scope(&Uuid::new_v4().to_string(), "chat");
    accept(&store, &scope, &delivery_input("first-turn"), 5)
        .await
        .unwrap();
    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let work = store.conversation(&scope).await.unwrap().work;
    assert_eq!(
        store
            .stop_authorized(&scope, "stop", vec![6; 32], &work)
            .await
            .unwrap(),
        Stopped::Held
    );
    // The new message arrives before the worker's next check.
    accept(&store, &scope, &delivery_input("second-turn"), 5)
        .await
        .unwrap();
    let error = store.assert_current(&claim).await.unwrap_err();
    assert!(
        matches!(
            error,
            engine::ProductError::NotCurrent(engine::NotCurrent::Withdrawn)
        ),
        "Stop ended the attempt, so the waiting message does not make it wait: {error}"
    );
    let reason = engine::AttemptFailure::of(
        &engine::Failure::from(error),
        &engine::Cancellation::default(),
    );
    store
        .settle_chat_attempt(&claim, reason, Backoff::IMMEDIATE)
        .await
        .unwrap();
    let next = store.claim(&scope, 60).await.unwrap().unwrap();
    assert_ne!(next.attempt, claim.attempt);
    assert!(next.revision > claim.revision, "the new message runs next");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn stop_holds_the_named_work_until_resume_and_refuses_a_late_commit() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = delivery_scope(&practice, "chat");
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    accept(&store, &scope, &delivery_input("turn"), 5)
        .await
        .unwrap();
    let claim = store
        .claim(&scope, 60)
        .await
        .unwrap()
        .expect("pending work");
    let work = store.conversation(&scope).await.unwrap().work.clone();
    store
        .persist(
            &claim,
            engine::Event {
                attempt_id: claim.attempt.clone(),
                sequence: 1,
                ordinal: 1,
                kind: engine::EventKind::Candidate {
                    text: "late".into(),
                },
            },
        )
        .await
        .unwrap();
    let candidate = Turn {
        items: vec![Item::assistant("late")],
        usage: Default::default(),
        receipt: serde_json::json!({"source":"fixture"}),
    };

    let mut other = scope.clone();
    other.physician = "someone-else".into();
    assert_eq!(
        store
            .stop_authorized(&other, "theirs", vec![8; 32], &work)
            .await
            .unwrap(),
        Stopped::Refused
    );
    let mut other_practice = scope.clone();
    other_practice.practice = uuid::Uuid::new_v4().to_string();
    assert_eq!(
        store
            .stop_authorized(&other_practice, "other-practice", vec![7; 32], &work)
            .await
            .unwrap(),
        Stopped::Refused
    );
    assert_eq!(
        store.conversation(&scope).await.unwrap().status,
        WorkStatus::Active
    );
    assert_eq!(
        store
            .stop_authorized(&scope, "mismatch", vec![6; 32], "other-work")
            .await
            .unwrap(),
        Stopped::Refused
    );
    assert!(
        store
            .stop_authorized(&scope, "short", vec![6; 4], &work)
            .await
            .is_err()
    );
    // A blank field is the caller's mistake, refused as a value, not an error.
    for (key, work_id) in [(" ", work.as_str()), ("blank-work", "")] {
        assert_eq!(
            store
                .stop_authorized(&scope, key, vec![6; 32], work_id)
                .await
                .unwrap(),
            Stopped::Refused,
            "{key:?} {work_id:?}"
        );
    }
    assert_eq!(
        store.conversation(&scope).await.unwrap().status,
        WorkStatus::Active
    );
    assert!(store.is_current(&claim).await.unwrap());

    assert_eq!(
        store
            .stop_authorized(&scope, "stop", vec![6; 32], &work)
            .await
            .unwrap(),
        Stopped::Held
    );
    assert!(!store.is_current(&claim).await.unwrap());
    let held = store.conversation(&scope).await.unwrap();
    assert_eq!(held.status, WorkStatus::Held);
    assert_eq!(
        held.attempts.last().unwrap().status,
        AttemptStatus::Cancelled
    );
    let endings: Vec<_> = held.runs[&held.work]
        .events()
        .iter()
        .filter(|event| matches!(event, engine::runs::RunEvent::RunEnded { .. }))
        .collect();
    assert_eq!(endings.len(), 1, "Stop must save exactly one run outcome");
    assert_eq!(held.session_hash.as_deref(), Some(&[6u8; 32][..]));
    assert_eq!(
        store
            .stop_authorized(&scope, "stop", vec![6; 32], &work)
            .await
            .unwrap(),
        Stopped::Duplicate
    );
    assert_eq!(
        store
            .stop_authorized(&scope, "stop", vec![6; 32], "other-work")
            .await
            .unwrap(),
        Stopped::Refused
    );
    assert!(
        store
            .commit_authorized(&claim, &candidate, &mut OpenAuthority)
            .await
            .is_err()
    );
    let held = store.conversation(&scope).await.unwrap();
    assert_eq!(held.status, WorkStatus::Held);
    assert_eq!(held.messages.len(), 1);
    assert!(
        !store
            .dispatch_due(100, Some(&start))
            .await
            .unwrap()
            .deliveries
            .iter()
            .any(|delivery| delivery.scope == scope)
    );

    drop(store);
    let store = Store::connect(&url).await.unwrap();
    let held = store.conversation(&scope).await.unwrap();
    assert_eq!(held.status, WorkStatus::Held);
    assert_eq!(
        held.attempts.last().unwrap().status,
        AttemptStatus::Cancelled
    );
    let run = held.run().unwrap().id.to_owned();
    assert_eq!(
        store
            .resume_authorized(&scope, "resume", vec![9; 32], &work, &run, None)
            .await
            .unwrap(),
        Resumed::Pending {
            delivery: Delivery::chat_turn(scope.clone(), "turn".into())
        }
    );
    assert!(store.claim(&scope, 60).await.unwrap().is_none());
    // The first worker disappeared. Recovery waits for its lease to expire.
    let (tx, mut state, _) = store.locked(&scope).await.unwrap();
    state.attempts.last_mut().unwrap().expires = 0;
    Store::save(tx, &scope, &state).await.unwrap();
    let successor = store.claim(&scope, 60).await.unwrap().unwrap();
    assert_eq!(store.conversation(&scope).await.unwrap().failures, 0);
    assert_ne!(successor.attempt, claim.attempt);
    let continued = store.conversation(&scope).await.unwrap();
    assert_ne!(continued.run().unwrap().id, run);
    assert_eq!(continued.run().unwrap().continues, Some(run.as_str()));
    assert_eq!(continued.runs[&work].events().len(), 3);

    // Resume keeps the work identity. Replaying the first Stop is that same,
    // already-taken command and leaves the resumed run going; a second Stop
    // is a command of its own and holds it.
    assert_eq!(
        store
            .stop_authorized(&scope, "stop", vec![6; 32], &work)
            .await
            .unwrap(),
        Stopped::Duplicate
    );
    assert_eq!(
        store.conversation(&scope).await.unwrap().status,
        WorkStatus::Active
    );
    assert_eq!(
        store
            .stop_authorized(&scope, "stop-again", vec![6; 32], &work)
            .await
            .unwrap(),
        Stopped::Held
    );
    let held = store.conversation(&scope).await.unwrap();
    assert_eq!(held.status, WorkStatus::Held);
    assert_eq!(held.attempts.last().unwrap().id, successor.attempt);
    assert_eq!(
        held.attempts.last().unwrap().status,
        AttemptStatus::Cancelled
    );

    // Sending something else releases the hold and the new message is
    // answered. Neither stopped run can publish over it, and its Resume is gone.
    assert_eq!(
        accept(&store, &scope, &delivery_input("steered"), 4)
            .await
            .unwrap(),
        Accepted::New
    );
    let steered = store.conversation(&scope).await.unwrap();
    assert_eq!(steered.status, WorkStatus::Pending);
    assert_eq!(
        steered
            .messages
            .iter()
            .map(|message| message.text.as_str())
            .collect::<Vec<_>>(),
        ["turn", "steered"]
    );
    for stopped in [&claim, &successor] {
        assert!(
            store
                .commit_authorized(stopped, &candidate, &mut OpenAuthority)
                .await
                .is_err()
        );
    }
    // The new message waits while the stopped run drains its calls, and is
    // due once that run is settled.
    let steered_turn = Delivery::chat_turn(scope.clone(), "steered".into());
    assert!(
        !store
            .dispatch_due(100, Some(&start))
            .await
            .unwrap()
            .deliveries
            .contains(&steered_turn)
    );
    assert_eq!(
        store
            .resume_authorized(
                &scope,
                "resume-stopped",
                vec![9; 32],
                &work,
                &successor.attempt,
                None
            )
            .await
            .unwrap(),
        Resumed::Refused
    );
    store
        .settle_chat_attempt(
            &successor,
            engine::AttemptFailure::Interrupted,
            Backoff::IMMEDIATE,
        )
        .await
        .unwrap();
    assert!(
        store
            .dispatch_due(100, Some(&start))
            .await
            .unwrap()
            .deliveries
            .contains(&steered_turn)
    );
    let answering = store.claim(&scope, 60).await.unwrap().unwrap();
    assert_ne!(answering.attempt, successor.attempt);

    let quiet = delivery_scope(&practice, "before-claim");
    accept(&store, &quiet, &delivery_input("early"), 5)
        .await
        .unwrap();
    let early = store.conversation(&quiet).await.unwrap();
    assert!(early.attempts.is_empty());
    assert_eq!(
        store
            .stop_authorized(&quiet, "early-stop", vec![6; 32], &early.work)
            .await
            .unwrap(),
        Stopped::Held
    );
    let stopped = store.conversation(&quiet).await.unwrap();
    assert!(stopped.attempts.is_empty());
    // Pressing Stop again on the held work is not a refusal: it is held.
    assert_eq!(
        store
            .stop_authorized(&quiet, "early-stop-again", vec![6; 32], &early.work)
            .await
            .unwrap(),
        Stopped::Duplicate
    );
    let unchanged = store.conversation(&quiet).await.unwrap();
    assert_eq!(
        (unchanged.status, unchanged.attempts.len()),
        (WorkStatus::Held, 0)
    );
    let first_run = stopped.run().unwrap().id.to_owned();
    // The page keys Resume by the run it names, as `resume-{work}-{run}`.
    assert_eq!(
        store
            .resume_authorized(
                &quiet,
                &format!("resume-{}-{first_run}", stopped.work),
                vec![9; 32],
                &stopped.work,
                &first_run,
                None,
            )
            .await
            .unwrap(),
        Resumed::Pending {
            delivery: Delivery::chat_turn(quiet.clone(), "early".into())
        }
    );
    // Stop again before any worker claims the resumed turn. That hold names
    // a run of its own, so the next Resume is a new command and is taken.
    assert_eq!(
        store
            .stop_authorized(&quiet, "early-stop-resumed", vec![6; 32], &stopped.work)
            .await
            .unwrap(),
        Stopped::Held
    );
    let again = store.conversation(&quiet).await.unwrap();
    assert_eq!(again.status, WorkStatus::Held);
    let second_run = again.run().unwrap().id.to_owned();
    assert_ne!(second_run, first_run);
    assert_eq!(again.run().unwrap().continues, Some(first_run.as_str()));
    assert_eq!(
        store
            .resume_authorized(
                &quiet,
                &format!("resume-{}-{second_run}", again.work),
                vec![9; 32],
                &again.work,
                &second_run,
                None,
            )
            .await
            .unwrap(),
        Resumed::Pending {
            delivery: Delivery::chat_turn(quiet.clone(), "early".into())
        }
    );
    assert_eq!(
        store.conversation(&quiet).await.unwrap().status,
        WorkStatus::Pending
    );

    // Couldn't finish stays Try again. Stop does not turn it into a hold.
    let failed_scope = delivery_scope(&practice, "failed");
    accept(&store, &failed_scope, &delivery_input("gave-up"), 5)
        .await
        .unwrap();
    let turn = Delivery::chat_turn(failed_scope.clone(), "gave-up".into());
    for _ in 0..MAX_FAILED_ATTEMPTS {
        let claim = store.claim_delivery(&turn, 60).await.unwrap().unwrap();
        store
            .settle_chat_attempt(
                &claim,
                harness::engine::AttemptFailure::Failed,
                Backoff::IMMEDIATE,
            )
            .await
            .unwrap();
    }
    let failed = store.conversation(&failed_scope).await.unwrap();
    assert_eq!(failed.status, WorkStatus::Failed);
    assert_eq!(
        store
            .stop_authorized(&failed_scope, "stop-failed", vec![6; 32], &failed.work)
            .await
            .unwrap(),
        Stopped::Refused
    );
    let still = store.conversation(&failed_scope).await.unwrap();
    assert_eq!(still.status, WorkStatus::Failed);
    let run = still.run().unwrap().id.to_owned();
    assert_eq!(
        store
            .resume_authorized(
                &failed_scope,
                "try-again",
                vec![9; 32],
                &still.work,
                &run,
                None,
            )
            .await
            .unwrap(),
        Resumed::Pending {
            delivery: Delivery::chat_turn(failed_scope.clone(), "gave-up".into())
        }
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn engine_failure_stays_leased_until_the_lease_expires() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    for cancelled in [false, true] {
        let scope = delivery_scope(&uuid::Uuid::new_v4().to_string(), "chat");
        accept(&store, &scope, &delivery_input("turn"), 0)
            .await
            .unwrap();
        let claim = store.claim(&scope, 60).await.unwrap().unwrap();
        let mut host = PhysicianHost {
            store: store.clone(),
            claim: claim.clone(),
            authority: OpenAuthority,
            shown_steps: super::ShownSteps::new(()),
        };
        let cancel = engine::Cancellation::default();
        if cancelled {
            cancel.cancel();
        }
        assert!(
            host.run(&mut Fails, harness::engine::RunLimits::default(), &cancel)
                .await
                .is_err()
        );
        let state = store.conversation(&scope).await.unwrap();
        assert_eq!(state.status, WorkStatus::Active);
        assert_eq!(state.attempts.last().unwrap().status, AttemptStatus::Active);
        assert_eq!(state.failures, 0);
        assert_eq!(state.messages.len(), 1);
        if cancelled {
            assert_eq!(
                store
                    .settle_chat_attempt(&claim, AttemptFailure::Interrupted, Backoff::IMMEDIATE)
                    .await
                    .unwrap(),
                delivery::AttemptSettlement::Applied,
                "hosting settlement must be the attempt-end writer"
            );
            let saved = store.conversation(&scope).await.unwrap();
            assert_eq!(saved.attempts[0].status, AttemptStatus::Interrupted);
            assert_eq!(saved.failures, 0);
            continue;
        }
        let (tx, mut state, _) = store.locked(&scope).await.unwrap();
        state.attempts.last_mut().unwrap().expires = 0;
        Store::save(tx, &scope, &state).await.unwrap();
        let successor = store.claim(&scope, 60).await.unwrap().unwrap();
        // Once hosting reclaims the abandoned attempt, its late cancellation cannot
        // release the successor or erase the recorded failure.
        assert_eq!(
            store
                .settle_chat_attempt(
                    &claim,
                    harness::engine::AttemptFailure::Interrupted,
                    Backoff::IMMEDIATE
                )
                .await
                .unwrap(),
            delivery::AttemptSettlement::Superseded
        );
        assert_ne!(successor.attempt, claim.attempt);
        let state = store.conversation(&scope).await.unwrap();
        assert_eq!(state.status, WorkStatus::Active);
        assert_eq!(state.failures, 1);
        assert_eq!(state.messages.len(), 1);
    }
}

/// A steer keeps the work and leaves the old attempt active until its worker
/// settles it. That worker can end at a limit or fail before it next checks
/// currency; its reason belongs to the old input, not to the newer message.
async fn settle_after_steer(reason: AttemptFailure) -> (Conversation, DeliveryState) {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = delivery_scope(&uuid::Uuid::new_v4().to_string(), "chat");
    accept(&store, &scope, &delivery_input("first"), 5)
        .await
        .unwrap();
    let old = store.claim(&scope, 60).await.unwrap().unwrap();
    accept(&store, &scope, &delivery_input("steer"), 5)
        .await
        .unwrap();
    store
        .settle_chat_attempt(&old, reason, Backoff::IMMEDIATE)
        .await
        .unwrap();
    let state = store.conversation(&scope).await.unwrap();
    let steer = store
        .delivery_state(&Delivery::chat_turn(scope, "steer".into()))
        .await
        .unwrap();
    (state, steer)
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_limit_on_a_steered_attempt_leaves_the_newer_message_pending() {
    let (state, steer) = settle_after_steer(AttemptFailure::Limit).await;
    assert_eq!(
        (state.status, state.failures, state.run().unwrap().outcome),
        (WorkStatus::Pending, 0, None),
        "the newer message never ran, so it is not held at a limit"
    );
    assert_eq!(steer, DeliveryState::Due);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_failure_on_a_steered_attempt_leaves_the_newer_messages_allowance_whole() {
    let (state, steer) = settle_after_steer(AttemptFailure::Failed).await;
    assert_eq!(
        (state.status, state.failures, state.run().unwrap().outcome),
        (WorkStatus::Pending, 0, None),
        "a steer consumes no failure allowance, so the newer message keeps all of it"
    );
    assert_eq!(steer, DeliveryState::Due);
}

async fn open_draft(store: &Store, scope: &Scope, text: &str) -> u64 {
    store
        .open_draft_authorized(scope, text, &[7; 32], &mut OpenAuthority)
        .await
        .unwrap()
        .expect("authorized fixture draft")
        .revision
}

struct OpenAuthority;

#[async_trait]
impl Authority for OpenAuthority {
    async fn check(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn lock_publication(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
}

struct Fails;

impl Provider for Fails {
    async fn exchange(
        &mut self,
        _: &Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn engine::recipe::Observer,
    ) -> Result<Turn, engine::Cause> {
        Err("synthetic provider failure".into())
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn expired_worker_lease_is_redispatched_by_the_database_clock() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let start = Scope {
        practice: practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let scope = delivery_scope(&practice, "chat");
    accept(&store, &scope, &delivery_input("turn"), 9)
        .await
        .unwrap();
    let delivery = Delivery::chat_turn(scope.clone(), "turn".into());
    let claim = store.claim_delivery(&delivery, 60).await.unwrap().unwrap();
    // Expire the lease at the storage owner instead of sleeping
    // across a floored database-clock second (flaky on busy runners).
    let (tx, mut state, _) = store.locked(&scope).await.unwrap();
    state.attempts.last_mut().unwrap().expires = 0;
    state.dispatch_after = 0;
    Store::save(tx, &scope, &state).await.unwrap();
    let page = store.dispatch_due(100, Some(&start)).await.unwrap();
    assert!(page.deliveries.contains(&delivery));
    assert!(store.renew_claim(&claim).await.is_err());
    let successor = store.claim_delivery(&delivery, 60).await.unwrap().unwrap();
    assert_ne!(successor.attempt, claim.attempt);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn chats_list_one_physicians_workspace_by_latest_activity() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = |physician: &str, workspace: &str, chat: &str| Scope {
        practice: practice.clone(),
        physician: physician.into(),
        workspace: workspace.into(),
        chat: chat.into(),
    };
    let input = |key: &str, text: &str| Input {
        key: key.into(),
        text: text.into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    for (key, s, text) in [
        ("a1", scope("me", "ws", "chat-a"), "About labs"),
        ("b1", scope("me", "ws", "chat-b"), "About meds"),
        ("a2", scope("me", "ws", "chat-a"), "And the follow-up"),
        ("x1", scope("other", "ws", "chat-x"), "Not mine"),
        ("y1", scope("me", "elsewhere", "chat-y"), "Other patient"),
    ] {
        assert_eq!(
            accept(&store, &s, &input(key, text), 0).await.unwrap(),
            Accepted::New
        );
    }
    // Same-second accepts tie on the clock; move chat-a's activity forward so
    // the order is decided by activity, not the id tie-break.
    let (tx, mut state, now) = store.locked(&scope("me", "ws", "chat-a")).await.unwrap();
    assert!(state.opened_at > 0 && state.opened_at <= now);
    state.last_activity_at = now + 100;
    Store::save(tx, &scope("me", "ws", "chat-a"), &state)
        .await
        .unwrap();

    let chats = store.chats(&practice, "me", "ws").await.unwrap();
    let listed: Vec<_> = chats
        .iter()
        .map(|c| (c.chat.as_str(), c.first_message.as_str()))
        .collect();
    assert_eq!(listed, [("chat-a", "About labs"), ("chat-b", "About meds")]);
    assert!(chats.iter().all(|c| c.opened_at > 0));

    // Equal activity falls back to chat id descending.
    let (tx, mut state, _) = store.locked(&scope("me", "ws", "chat-b")).await.unwrap();
    state.last_activity_at = chats[0].last_activity_at;
    Store::save(tx, &scope("me", "ws", "chat-b"), &state)
        .await
        .unwrap();
    let ids: Vec<_> = store
        .chats(&practice, "me", "ws")
        .await
        .unwrap()
        .into_iter()
        .map(|c| c.chat)
        .collect();
    assert_eq!(ids, ["chat-b", "chat-a"]);

    assert!(
        store
            .chats(&practice, "nobody", "ws")
            .await
            .unwrap()
            .is_empty()
    );
    assert!(store.chats(&practice, "", "ws").await.is_err());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn chat_list_orders_by_activity_before_its_limit() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let practice = uuid::Uuid::new_v4().to_string();
    let scope = |physician: &str, chat: String| Scope {
        practice: practice.clone(),
        physician: physician.into(),
        workspace: "ws".into(),
        chat,
    };
    let input = |key: String| Input {
        key,
        text: "Hello".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    // More chats than the list holds. Chat-id order alone would list
    // chat-005..chat-204 and leave chat-000 out.
    let total = MAX_LISTED_CHATS as usize + 5;
    for n in 0..total {
        accept(
            &store,
            &scope("me", format!("chat-{n:03}")),
            &input(format!("me-{n}")),
            0,
        )
        .await
        .unwrap();
    }
    // Another physician's newer chats in the same workspace never list.
    for n in 0..3 {
        let theirs = scope("other", format!("chat-zzz-{n}"));
        accept(&store, &theirs, &input(format!("other-{n}")), 0)
            .await
            .unwrap();
        let (tx, mut state, now) = store.locked(&theirs).await.unwrap();
        state.last_activity_at = now + 1000;
        Store::save(tx, &theirs, &state).await.unwrap();
    }
    let before = store.chats(&practice, "me", "ws").await.unwrap();
    assert_eq!(before.len(), MAX_LISTED_CHATS as usize);
    assert!(
        before
            .iter()
            .all(|c| str::starts_with(&c.chat, "chat-") && !str::contains(&c.chat, "zzz"))
    );

    // A new message in a later database second makes chat-000 the newest.
    let oldest = scope("me", "chat-000".into());
    let newest_before = before[0].last_activity_at;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let (tx, _, now) = store.locked(&oldest).await.unwrap();
        tx.commit().await.unwrap();
        if now > newest_before {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "database clock stalled"
        );
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    assert_eq!(
        accept(&store, &oldest, &input("me-again".into()), 0)
            .await
            .unwrap(),
        Accepted::New
    );
    let after = store.chats(&practice, "me", "ws").await.unwrap();
    assert_eq!(after.len(), MAX_LISTED_CHATS as usize);
    assert_eq!(after[0].chat, "chat-000", "the active chat must list first");
    assert!(after[0].last_activity_at > newest_before);
    assert!(
        after
            .windows(2)
            .all(|pair| (pair[0].last_activity_at, &pair[0].chat)
                >= (pair[1].last_activity_at, &pair[1].chat)),
        "newest activity first, chat id descending on ties"
    );
    assert!(after.iter().all(|c| !str::contains(&c.chat, "zzz")));

    // Settle every chat: the restart sweeps in other tests share this
    // database and read pending work from the start of the key order.
    let chats = (0..total)
        .map(|n| scope("me", format!("chat-{n:03}")))
        .chain((0..3).map(|n| scope("other", format!("chat-zzz-{n}"))));
    for chat in chats {
        let (tx, mut state, _) = store.locked(&chat).await.unwrap();
        state.status = WorkStatus::Completed;
        Store::save(tx, &chat, &state).await.unwrap();
    }
    assert!(
        store
            .pending_batch(100, Some(&scope("", String::new())))
            .await
            .unwrap()
            .iter()
            .all(|s| s.practice != practice)
    );
}

/// Allows or refuses every publication, without the auth owner's tables.
struct Gate(bool);
#[async_trait]
impl Authority for Gate {
    async fn check(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
    async fn lock_publication(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        if self.0 && hash.is_some() {
            Ok(())
        } else {
            Err("physician authority changed".into())
        }
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn chat_drafts_save_under_a_revision_fence_for_one_physician() {
    use chat_draft::{ChatDraft, ChatDraftSaved};
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "draft".into(),
    };
    let hash = [7_u8; 32];
    let draft = |revision, text: &str| ChatDraft {
        revision,
        text: text.into(),
    };
    assert_eq!(store.chat_draft(&scope).await.unwrap(), draft(0, ""));

    // A refused authority writes nothing, even the first save.
    assert!(
        store
            .save_chat_draft(&scope, 0, "refused", &hash, &mut Gate(false))
            .await
            .is_err()
    );
    assert_eq!(store.chat_draft(&scope).await.unwrap(), draft(0, ""));
    assert!(
        store
            .save_chat_draft(&scope, 0, "short hash", &[1; 4], &mut Gate(true))
            .await
            .is_err()
    );

    let saved = |d| ChatDraftSaved::Saved(d);
    assert_eq!(
        store
            .save_chat_draft(&scope, 0, "first", &hash, &mut Gate(true))
            .await
            .unwrap(),
        saved(draft(1, "first"))
    );
    assert_eq!(store.chat_draft(&scope).await.unwrap(), draft(1, "first"));
    assert_eq!(
        store
            .save_chat_draft(&scope, 1, "second", &hash, &mut Gate(true))
            .await
            .unwrap(),
        saved(draft(2, "second"))
    );
    // Both a second opening save and a save at an old revision are stale and
    // answer the current draft without writing.
    for stale in [0, 1, 3] {
        assert_eq!(
            store
                .save_chat_draft(&scope, stale, "stale", &hash, &mut Gate(true))
                .await
                .unwrap(),
            ChatDraftSaved::Conflict(draft(2, "second"))
        );
    }
    assert!(
        store
            .save_chat_draft(&scope, 2, "refused", &hash, &mut Gate(false))
            .await
            .is_err()
    );
    assert_eq!(store.chat_draft(&scope).await.unwrap(), draft(2, "second"));
    // Two saves naming the same revision: exactly one wins.
    let (a, b) = tokio::join!(
        async {
            store
                .save_chat_draft(&scope, 2, "a", &hash, &mut Gate(true))
                .await
        },
        async {
            store
                .save_chat_draft(&scope, 2, "b", &hash, &mut Gate(true))
                .await
        },
    );
    let outcomes = [a.unwrap(), b.unwrap()];
    assert_eq!(
        outcomes
            .iter()
            .filter(|o| matches!(o, ChatDraftSaved::Saved(_)))
            .count(),
        1,
        "{outcomes:?}"
    );
    assert_eq!(store.chat_draft(&scope).await.unwrap().revision, 3);

    // Another physician, another chat and another workspace see nothing.
    for other in [
        Scope {
            physician: "colleague".into(),
            ..scope.clone()
        },
        Scope {
            chat: "chat-other".into(),
            ..scope.clone()
        },
        Scope {
            workspace: "workspace-other".into(),
            ..scope.clone()
        },
    ] {
        assert_eq!(store.chat_draft(&other).await.unwrap(), draft(0, ""));
    }
    let empty = Scope {
        physician: " ".into(),
        ..scope.clone()
    };
    assert!(store.chat_draft(&empty).await.is_err());
    // Drafts never create chat work.
    assert!(store.conversation(&scope).await.is_err());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn old_receipts_reduce_to_a_digest_that_still_answers_duplicates() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    // A rerun keeps the column and the existing receipts.
    store.migrate().await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "chat".into(),
    };
    let input = |key: &str, text: &str| Input {
        key: key.into(),
        text: text.into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    let stored = |key: &str| {
        let store = store.clone();
        let practice = scope.practice.clone();
        let key = key.to_owned();
        async move {
            command::Entity::find_by_id((practice, "physician".to_owned(), key))
                .one(&store.db)
                .await
                .unwrap()
                .unwrap()
        }
    };
    assert_eq!(
        accept(&store, &scope, &input("old", "old text"), 0)
            .await
            .unwrap(),
        Accepted::New
    );
    assert_eq!(
        accept(&store, &scope, &input("new", "new text"), 0)
            .await
            .unwrap(),
        Accepted::New
    );
    let fresh = stored("old").await;
    assert!(fresh.accepted_at > 0);
    assert_eq!(fresh.command["input"]["text"], "old text");
    // Age one receipt past the window; the other was accepted just now.
    command::ActiveModel {
        practice: sea_orm::Unchanged(fresh.practice.clone()),
        physician: sea_orm::Unchanged(fresh.physician.clone()),
        key: sea_orm::Unchanged(fresh.key.clone()),
        command: sea_orm::Unchanged(fresh.command.clone()),
        accepted_at: Set(fresh.accepted_at - receipt::RECEIPT_FULL_SECONDS - 1),
    }
    .update(&store.db)
    .await
    .unwrap();
    assert!(
        store
            .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 0)
            .await
            .is_err()
    );
    assert!(store.compact_receipts(-1, 10).await.is_err());
    // Older test rows elsewhere in this database may share the batch.
    while stored("old").await.command.get("sha256").is_none() {
        assert!(
            store
                .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 1000)
                .await
                .unwrap()
                > 0
        );
    }
    let compacted = stored("old").await;
    let keys: Vec<_> = compacted
        .command
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    assert_eq!(keys, ["salt", "sha256"]);
    assert!(!str::contains(&compacted.command.to_string(), "old text"));
    assert_eq!(stored("new").await.command["input"]["text"], "new text");

    // The digest still recognizes the exact command and refuses a different
    // one under the same key; the conversation keeps the authored text.
    assert_eq!(
        accept(&store, &scope, &input("old", "old text"), 0)
            .await
            .unwrap(),
        Accepted::Duplicate
    );
    assert!(
        accept(&store, &scope, &input("old", "other text"), 0)
            .await
            .is_err()
    );
    let mut other_chat = scope.clone();
    other_chat.chat = "other-chat".into();
    assert!(
        accept(&store, &other_chat, &input("old", "old text"), 0)
            .await
            .is_err()
    );
    let conversation = store.conversation(&scope).await.unwrap();
    assert_eq!(conversation.revision, 2);
    assert_eq!(conversation.messages[0].text, "old text");
    // A reduced receipt is never reduced again.
    store
        .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 1000)
        .await
        .unwrap();
    assert_eq!(stored("old").await.command, compacted.command);

    // Try again's receipt compares the same way once reduced.
    let resume_scope = Scope {
        chat: "resume-chat".into(),
        ..scope.clone()
    };
    let hash = vec![9_u8; 32];
    store
        .accept_authorized(
            &resume_scope,
            &input("turn", "turn text"),
            hash.clone(),
            &test_skills(),
            &test_chat_recipe(),
            None,
        )
        .await
        .unwrap();
    for _ in 0..MAX_FAILED_ATTEMPTS {
        let claim = store.claim(&resume_scope, 60).await.unwrap().unwrap();
        store
            .settle_chat_attempt(
                &claim,
                harness::engine::AttemptFailure::Failed,
                Backoff::IMMEDIATE,
            )
            .await
            .unwrap();
    }
    let failed = store.conversation(&resume_scope).await.unwrap();
    assert_eq!(failed.status, WorkStatus::Failed);
    let run = failed.run().unwrap().id.to_owned();
    assert!(matches!(
        store
            .resume_authorized(
                &resume_scope,
                "resume",
                hash.clone(),
                &failed.work,
                &run,
                None
            )
            .await
            .unwrap(),
        Resumed::Pending { .. }
    ));
    let resume = stored("resume").await;
    command::ActiveModel {
        practice: sea_orm::Unchanged(resume.practice.clone()),
        physician: sea_orm::Unchanged(resume.physician.clone()),
        key: sea_orm::Unchanged(resume.key.clone()),
        command: sea_orm::Unchanged(resume.command.clone()),
        accepted_at: Set(0),
    }
    .update(&store.db)
    .await
    .unwrap();
    while stored("resume").await.command.get("sha256").is_none() {
        store
            .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 1000)
            .await
            .unwrap();
    }
    assert_eq!(
        store
            .resume_authorized(
                &resume_scope,
                "resume",
                hash.clone(),
                &failed.work,
                &run,
                None
            )
            .await
            .unwrap(),
        Resumed::Duplicate
    );
    assert_eq!(
        store
            .resume_authorized(
                &resume_scope,
                "resume",
                hash,
                &failed.work,
                "other-run",
                None
            )
            .await
            .unwrap(),
        Resumed::Refused
    );
    // A reply revision's receipt compares the same way once reduced.
    let reply_scope = Scope {
        chat: "reply".into(),
        ..scope.clone()
    };
    open_draft(&store, &reply_scope, "Reply text.").await;
    let revise = reply::RevisionInput {
        key: "revise".into(),
        expected_revision: 1,
        instruction: "Shorter.".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    assert_eq!(
        store
            .accept_revision(
                &reply_scope,
                &revise,
                vec![9; 32],
                &test_skills(),
                &test_revision_recipe(),
                None
            )
            .await
            .unwrap(),
        reply::RevisionAccepted::New
    );
    let row = stored("revise").await;
    command::ActiveModel {
        practice: sea_orm::Unchanged(row.practice.clone()),
        physician: sea_orm::Unchanged(row.physician.clone()),
        key: sea_orm::Unchanged(row.key.clone()),
        command: sea_orm::Unchanged(row.command.clone()),
        accepted_at: Set(0),
    }
    .update(&store.db)
    .await
    .unwrap();
    while stored("revise").await.command.get("sha256").is_none() {
        store
            .compact_receipts(receipt::RECEIPT_FULL_SECONDS, 1000)
            .await
            .unwrap();
    }
    assert_eq!(
        store
            .accept_revision(
                &reply_scope,
                &revise,
                vec![9; 32],
                &test_skills(),
                &test_revision_recipe(),
                None
            )
            .await
            .unwrap(),
        reply::RevisionAccepted::Duplicate
    );
    let collision = reply::RevisionInput {
        instruction: "Longer.".into(),
        ..revise.clone()
    };
    assert!(
        store
            .accept_revision(
                &reply_scope,
                &collision,
                vec![9; 32],
                &test_skills(),
                &test_revision_recipe(),
                None
            )
            .await
            .is_err()
    );
    let draft = store.draft(&reply_scope).await.unwrap().unwrap();
    assert_eq!(
        draft.accepted.as_ref().map(|accepted| &accepted.input),
        Some(&revise)
    );
    // Settle the revision so other suites' due-work reads never pick it up.
    store
        .save_draft(
            &reply_scope,
            1,
            "Physician text.",
            &[9; 32],
            &mut Gate(true),
        )
        .await
        .unwrap();

    // Settle the work so other suites' restart sweeps never pick it up.
    let (tx, mut state, _) = store.locked(&resume_scope).await.unwrap();
    state.status = WorkStatus::Completed;
    Store::save(tx, &resume_scope, &state).await.unwrap();
    let (tx, mut state, _) = store.locked(&scope).await.unwrap();
    state.status = WorkStatus::Completed;
    Store::save(tx, &scope, &state).await.unwrap();
}

fn test_skills() -> PhysicianSkills {
    PhysicianSkills::configured(None, false).unwrap()
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn currency_is_read_only_and_renewal_touches_only_the_current_chat_attempt() {
    use entities::chat_attempt::{Column as C, Entity};
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = Scope {
        practice: Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: "workspace".into(),
        chat: "chat".into(),
    };
    accept(
        &store,
        &scope,
        &Input {
            key: "input".into(),
            text: "Hello".into(),
            timezone: "UTC".into(),
            scenario: None,
        },
        0,
    )
    .await
    .unwrap();
    let old = store.claim(&scope, 60).await.unwrap().unwrap();
    assert_eq!(
        store
            .settle_chat_attempt(
                &old,
                engine::AttemptFailure::Interrupted,
                Backoff::IMMEDIATE
            )
            .await
            .unwrap(),
        delivery::AttemptSettlement::Applied
    );
    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let tx = store.db.begin().await.unwrap();
    let near_expiry = now(&tx).await.unwrap() + 5;
    tx.commit().await.unwrap();
    Entity::update_many()
        .col_expr(C::Expires, Expr::value(near_expiry))
        .filter(control::chat_scope(&scope))
        .exec(&store.db)
        .await
        .unwrap();
    let (locked, _, _) = store.locked(&scope).await.unwrap();
    let read = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        store.assert_current(&claim),
    )
    .await;
    let mut host = PhysicianHost {
        store: store.clone(),
        claim: claim.clone(),
        authority: OpenAuthority,
        shown_steps: super::ShownSteps::new(()),
    };
    let sources =
        tokio::time::timeout(std::time::Duration::from_secs(1), host.message_sources()).await;
    let mut reads = tokio::task::JoinSet::new();
    for index in 0..8 {
        let call = Invocation {
            id: format!("skill-{index}"),
            name: harness::skills::SKILL_VIEW.into(),
            arguments: "{}".into(),
        };
        reads.spawn(host.execute(
            &call,
            serde_json::json!({"name":"patient-record-retrieval"}),
        ));
    }
    let resolved = host.read_result("absent-result");
    let tool_reads = tokio::time::timeout(std::time::Duration::from_secs(2), async {
        assert_eq!(resolved.await.unwrap(), None);
        while let Some(result) = reads.join_next().await {
            assert!(matches!(result.unwrap().unwrap(), ToolOutcome::Returned(_)));
        }
    })
    .await;
    locked.rollback().await.unwrap();
    assert!(
        sources.is_ok(),
        "source refresh waited for a publication lock"
    );
    assert!(
        tool_reads.is_ok(),
        "tool reads waited for the conversation mutation lock"
    );
    let sources = sources.unwrap().unwrap().unwrap();
    assert!(sources.iter().any(|message| message.text == "Hello"));
    assert!(read.is_ok(), "currency check waited for a write lock");
    read.unwrap().unwrap();
    assert_eq!(
        store.conversation(&scope).await.unwrap().attempts[1].expires,
        near_expiry,
        "currency check wrote the lease"
    );
    store.renew_claim(&claim).await.unwrap();
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.attempts[0].expires, near_expiry);
    assert!(
        state.attempts[1].expires >= near_expiry + 50,
        "current lease was not renewed"
    );
    assert!(store.renew_claim(&old).await.is_err());
    let lost = store.assert_current(&old).await.unwrap_err();
    assert!(
        matches!(
            lost,
            engine::ProductError::NotCurrent(engine::NotCurrent::Lost)
        ),
        "a replaced attempt has lost ownership: {lost}"
    );
    assert_eq!(
        store
            .settle_chat_attempt(&old, engine::AttemptFailure::Failed, Backoff::IMMEDIATE)
            .await
            .unwrap(),
        delivery::AttemptSettlement::Superseded
    );
    let preserved = store.conversation(&scope).await.unwrap();
    assert_eq!(preserved.failures, state.failures);
    assert_eq!(preserved.attempts[1].status, AttemptStatus::Active);
    assert_eq!(preserved.attempts[1].id, claim.attempt);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn deleted_history_source_is_absent_from_chords_records_and_successor_request() {
    struct Reader {
        requests: Vec<Request>,
        resume: bool,
    }
    impl Provider for Reader {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<Turn, engine::Cause> {
            self.requests.push(request.clone());
            if self.resume {
                return Ok(Turn {
                    items: answered("Continued from remaining sources."),
                    receipt: Value::Null,
                    usage: Default::default(),
                });
            }
            if self.requests.len() == 2 {
                return Err("synthetic process loss before history acceptance".into());
            }
            Ok(Turn {
                items: vec![Item::Call(engine::Invocation {
                    id: "history-read".into(),
                    name: engine::history::READ.into(),
                    arguments: serde_json::json!({"id":"private-old-input"}).to_string(),
                })],
                receipt: Value::Null,
                usage: Default::default(),
            })
        }
    }
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = delivery_scope(&uuid::Uuid::new_v4().to_string(), "history-deletion");
    let private = "Distinct private Chords message 🪕 that the product removes";
    accept(
        &store,
        &scope,
        &Input {
            key: "private-old-input".into(),
            text: private.into(),
            timezone: "UTC".into(),
            scenario: None,
        },
        0,
    )
    .await
    .unwrap();
    accept(&store, &scope, &delivery_input("current-input"), 0)
        .await
        .unwrap();
    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: OpenAuthority,
        shown_steps: super::ShownSteps::new(()),
    };
    let bounds = engine::RunLimits {
        max_calls: 8,
        ..engine::RunLimits::default()
    };
    let mut reader = Reader {
        requests: vec![],
        resume: false,
    };
    assert!(
        host.run(&mut reader, bounds, &engine::Cancellation::default())
            .await
            .is_err()
    );
    assert!(reader.requests[1].items.iter().any(|item| matches!(item,
        Item::MaterializedResult { output, .. } if output.as_str().contains(private))));
    // Remove the authored source at its product owner before the successor reads it.
    let (tx, mut state, _) = store.locked(&scope).await.unwrap();
    state
        .messages
        .retain(|message| message.source().id != "private-old-input");
    state.attempts.last_mut().unwrap().expires = 0;
    Store::save(tx, &scope, &state).await.unwrap();
    let saved = store.conversation(&scope).await.unwrap();
    assert!(
        !serde_json::to_string(&saved.attempts)
            .unwrap()
            .as_str()
            .contains(private),
        "Chords run records must reference history text at its product owner"
    );
    host.claim = store.claim(&scope, 60).await.unwrap().unwrap();
    reader.resume = true;
    host.run(&mut reader, bounds, &engine::Cancellation::default())
        .await
        .unwrap();
    assert_ne!(reader.requests[1].items, reader.requests[2].items);
    assert!(
        !serde_json::to_string(&reader.requests[2])
            .unwrap()
            .as_str()
            .contains(private)
    );
    let finished = store.conversation(&scope).await.unwrap();
    assert_eq!(finished.status, WorkStatus::Completed);
    assert_eq!(
        finished.messages.last().unwrap().text,
        "Continued from remaining sources."
    );
    assert!(
        !serde_json::to_string(&finished.attempts)
            .unwrap()
            .as_str()
            .contains(private)
    );
    assert_eq!(finished.attempts.iter().flat_map(|attempt| &attempt.events).filter(|event|
        matches!(&event.kind, engine::EventKind::ToolStarted(call) if call.id == "history-read")).count(), 1);
}

#[test]
fn a_run_lifecycle_error_keeps_its_cause_and_a_wrapped_database_error_still_retries() {
    use harness::engine::runs::RunError;
    let lifecycle = RuntimeError::from(RunError::StillRunning);
    assert_eq!(
        std::error::Error::source(&lifecycle).and_then(|cause| cause.downcast_ref::<RunError>()),
        Some(&RunError::StillRunning)
    );
    assert!(
        !matches!(persistence_failure(lifecycle), engine::ProductError::Failed(cause) if cause.is::<engine::RetryablePersistence>())
    );
    let timeout = || DbErr::ConnectionAcquire(sea_orm::ConnAcquireErr::Timeout);
    assert!(
        matches!(persistence_failure(timeout()), engine::ProductError::Failed(cause) if cause.is::<engine::RetryablePersistence>())
    );
    assert!(
        matches!(persistence_failure(RuntimeError::from(timeout())), engine::ProductError::Failed(cause) if cause.is::<engine::RetryablePersistence>())
    );
}

fn test_binding() -> crate::ModelBinding {
    crate::ModelBinding {
        model: "controlled".into(),
        effort: "none".into(),
        adapter: "fixed".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 272_000,
    }
}
fn test_chat_recipe() -> harness::engine::recipe::RequestRecipe {
    crate::physician_chat::recipe(&test_binding(), "Owner", &test_skills())
}
fn test_revision_recipe() -> harness::engine::recipe::RequestRecipe {
    crate::runtime::reply::recipe(&test_binding(), &test_skills())
}
/// Accepts a chat input with the test skills and recipe under one session byte.
async fn accept(
    store: &Store,
    scope: &Scope,
    input: &Input,
    session: u8,
) -> Result<Accepted, RuntimeError> {
    store
        .accept_authorized(
            scope,
            input,
            vec![session; 32],
            &test_skills(),
            &test_chat_recipe(),
            None,
        )
        .await
}
/// Accepts a reply revision with the test skills and revision recipe.
async fn accept_revision_input(
    store: &Store,
    scope: &Scope,
    input: &reply::RevisionInput,
) -> Result<reply::RevisionAccepted, RuntimeError> {
    store
        .accept_revision(
            scope,
            input,
            vec![7; 32],
            &test_skills(),
            &test_revision_recipe(),
            None,
        )
        .await
}

#[test]
fn an_input_without_a_scenario_serializes_as_before() {
    let plain = Input {
        key: "k".into(),
        text: "t".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    // Receipts saved before the field existed must still compare equal.
    assert_eq!(
        serde_json::to_value(&plain).unwrap(),
        serde_json::json!({"key": "k", "text": "t", "timezone": "UTC"})
    );
    let old: Input =
        serde_json::from_value(serde_json::json!({"key": "k", "text": "t", "timezone": "UTC"}))
            .unwrap();
    assert_eq!(old, plain);
    let rehearsal = Input {
        scenario: Some("canary".into()),
        ..plain
    };
    assert_eq!(
        serde_json::to_value(&rehearsal).unwrap()["scenario"],
        "canary"
    );
    let untagged = control::conversation_state(&Conversation::new(
        serde_json::from_value(serde_json::json!("fixture-package")).unwrap(),
    ))
    .unwrap();
    assert!(untagged.get("scenario").is_none());
    let state = control::conversation(
        conversation::Model {
            practice: "practice".into(),
            physician: "physician".into(),
            workspace: "workspace".into(),
            chat: "chat".into(),
            status: WorkStatus::Completed,
            revision: 0,
            session_hash: None,
            state: untagged,
        },
        vec![],
    )
    .unwrap();
    assert_eq!(state.scenario, None);
}

/// The saved text of every status value. A renamed variant must not change it.
fn saved_text<S: sea_orm::ActiveEnum<Value = String> + sea_orm::Iterable>() -> Vec<(S, String)> {
    S::iter()
        .map(|status| {
            let text = status.to_value();
            (status, text)
        })
        .collect()
}

#[test]
fn work_status_keeps_its_saved_text() {
    use WorkStatus::*;
    let expected = [
        (Pending, "pending"),
        (Active, "active"),
        (Completed, "completed"),
        (Held, "held"),
        (Failed, "failed"),
        (Cancelled, "cancelled"),
    ];
    assert_eq!(
        saved_text(),
        expected.map(|(status, text)| (status, text.to_owned()))
    );
}

#[test]
fn draft_status_keeps_its_saved_text() {
    use reply::DraftStatus::*;
    let expected = [
        (Idle, "idle"),
        (Pending, "pending"),
        (Active, "active"),
        (Completed, "completed"),
        (Held, "held"),
        (Failed, "failed"),
        (Cancelled, "cancelled"),
    ];
    assert_eq!(
        saved_text(),
        expected.map(|(status, text)| (status, text.to_owned()))
    );
}

#[test]
fn attempt_status_keeps_its_saved_text() {
    use AttemptStatus::*;
    let expected = [
        (Active, "active"),
        (Interrupted, "interrupted"),
        (Cancelled, "cancelled"),
        (Failed, "failed"),
        (Held, "held"),
        (Completed, "completed"),
    ];
    assert_eq!(
        saved_text(),
        expected.map(|(status, text)| (status, text.to_owned()))
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn queue_signals_count_due_and_failed_work_and_age_the_oldest() {
    use std::time::Duration;
    // The signals cover every row, so they are read on a database of their own.
    let store = Store::connect(&migration_tests::database("runtime_signals").await)
        .await
        .unwrap();
    store.migrate().await.unwrap();
    let practice = "signals";
    assert_eq!(
        store.queue_signals().await.unwrap(),
        delivery::QueueSignals::default()
    );

    // Accepted hosted turns are due from acceptance.
    let waiting = delivery_scope(practice, "waiting");
    let running = delivery_scope(practice, "running");
    let failing = delivery_scope(practice, "failing");
    for scope in [&waiting, &running, &failing] {
        accept(&store, scope, &delivery_input(&scope.chat), 3)
            .await
            .unwrap();
    }
    let signals = store.queue_signals().await.unwrap();
    assert_eq!((due_count(&store).await, signals.failed), (3, 0));
    assert!(signals.due.unwrap().oldest < Duration::from_secs(5));

    // A live lease is not due; a retry delay that has not passed is not due.
    let turn = |scope: &Scope| Delivery::chat_turn(scope.clone(), scope.chat.clone());
    store
        .claim_delivery(&turn(&running), 60)
        .await
        .unwrap()
        .unwrap();
    let claim = store
        .claim_delivery(&turn(&failing), 60)
        .await
        .unwrap()
        .unwrap();
    store
        .settle_chat_attempt(&claim, engine::AttemptFailure::Failed, Backoff::HOSTED)
        .await
        .unwrap();
    assert_eq!(due_count(&store).await, 1);

    // The oldest due work is aged from when it became runnable: acceptance
    // for pending work, the lease's end for a lost attempt.
    let (tx, mut state, now) = store.locked(&waiting).await.unwrap();
    state.dispatch_after = now - 300;
    Store::save(tx, &waiting, &state).await.unwrap();
    let (tx, mut state, now) = store.locked(&running).await.unwrap();
    state.attempts.last_mut().unwrap().expires = now - 120;
    Store::save(tx, &running, &state).await.unwrap();
    let due = store.queue_signals().await.unwrap().due.unwrap();
    assert_eq!(due.count.get(), 2);
    let oldest = due.oldest.as_secs();
    assert!((300..305).contains(&oldest), "oldest due {oldest}s");
    let (tx, mut state, now) = store.locked(&waiting).await.unwrap();
    state.dispatch_after = now;
    Store::save(tx, &waiting, &state).await.unwrap();
    let oldest = store.queue_signals().await.unwrap().due.unwrap().oldest;
    assert!(
        (120..125).contains(&oldest.as_secs()),
        "a lost attempt waits from its lease's end, not acceptance: {oldest:?}"
    );

    // Work recovery gave up on counts as failed, not due.
    let (tx, mut state, _) = store.locked(&failing).await.unwrap();
    state.dispatch_after = 0;
    Store::save(tx, &failing, &state).await.unwrap();
    for _ in 1..MAX_FAILED_ATTEMPTS {
        let claim = store
            .claim_delivery(&turn(&failing), 60)
            .await
            .unwrap()
            .unwrap();
        store
            .settle_chat_attempt(&claim, engine::AttemptFailure::Failed, Backoff::IMMEDIATE)
            .await
            .unwrap();
    }
    let signals = store.queue_signals().await.unwrap();
    assert_eq!((due_count(&store).await, signals.failed), (2, 1));

    // Reply revisions count with chat turns, and only while current.
    let reply = delivery_scope(practice, "reply");
    open_draft(&store, &reply, "Original").await;
    let revision = reply::RevisionInput {
        key: "revise".into(),
        expected_revision: 1,
        instruction: "Shorten the reply".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    accept_revision_input(&store, &reply, &revision)
        .await
        .unwrap();
    assert_eq!(due_count(&store).await, 3);
    let (tx, mut draft, _) = store.locked_draft(&reply).await.unwrap();
    draft.revision += 1;
    Store::save_draft_row(tx, &reply, &draft).await.unwrap();
    assert_eq!(due_count(&store).await, 2);
}

/// How much hosted work is due, 0 when none is.
async fn due_count(store: &Store) -> u64 {
    let signals = store.queue_signals().await.unwrap();
    signals.due.map_or(0, |due| due.count.get())
}
