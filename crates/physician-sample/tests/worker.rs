//! Physician worker against the Pub/Sub emulator and a disposable Postgres.
//!
//! The emulator pushes deliveries. A duplicate runs the fixed provider once,
//! a failed run is redelivered and can then succeed, and the fifth failure is
//! forwarded to the dead-letter topic. The worker does not ack that fifth try,
//! and the turn ends as failed so the site's recovery sweep leaves it alone.
//! Concurrent pushes on one worker run side by side.

use std::time::{Duration, Instant};

use local_product::runtime::delivery::Delivery;
use local_product::runtime::{Conversation, Input, Scope, Store};
use physician_auth::{AuthStore, Membership, PostgresAuthStore};
#[path = "scale/mod.rs"]
mod support;

fn lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

struct Running {
    database: support::Database,
    store: Store,
    emulator: support::Emulator,
    worker: support::Process,
    worker_host: String,
    home: std::path::PathBuf,
}

impl Drop for Running {
    fn drop(&mut self) {
        self.worker.kill_now();
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

async fn running(label: &str, failures: Option<u32>, delay_ms: Option<u64>) -> Option<Running> {
    running_with(label, failures, delay_ms, false).await
}

async fn running_with(
    label: &str,
    failures: Option<u32>,
    delay_ms: Option<u64>,
    fail_marker: bool,
) -> Option<Running> {
    let database = support::Database::create(label).await?;
    let store = Store::connect(&database.url).await.expect("runtime store");
    let worker_port = support::free_port();
    let emulator = support::Emulator::start().await;
    emulator.provision(worker_port).await;
    let home = std::env::temp_dir().join(format!(
        "physician-scale-worker-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("worker home");
    let listen = format!("127.0.0.1:{worker_port}");
    let failure_count = failures.map(|count| count.to_string());
    let delay = delay_ms.map(|ms| ms.to_string());
    let mut env = vec![
        ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
        ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
        ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
        ("PHYSICIAN_WORKER_FIXED_TEXT", support::FIXED_TEXT),
    ];
    if let Some(count) = failure_count.as_deref() {
        env.push(("PHYSICIAN_WORKER_FIXED_FAILURES", count));
    }
    if let Some(ms) = delay.as_deref() {
        env.push(("PHYSICIAN_WORKER_FIXED_DELAY_MS", ms));
    }
    if fail_marker {
        env.push(("PHYSICIAN_WORKER_FIXED_FAIL_MARKER", "1"));
    }
    let mut worker = support::Process::spawn("worker", support::worker_bin(), &[], &env, &home);
    support::wait_ready(&mut worker, worker_port).await;
    Some(Running {
        database,
        store,
        emulator,
        worker,
        worker_host: listen,
        home,
    })
}

async fn accept_turn(url: &str, store: &Store) -> (Scope, Delivery) {
    accept_text(url, store, "Hello").await
}

async fn accept_text(url: &str, store: &Store, text: &str) -> (Scope, Delivery) {
    let auth = PostgresAuthStore::connect_without_migrations(url)
        .await
        .expect("auth store");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default();
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let email = format!("worker-{unique}@example.test");
    let practice = format!("practice-{unique}");
    let physician = format!("physician-{unique}");
    let workspace = format!("workspace-{unique}");
    let chat = format!("chat-{unique}");
    auth.ensure_account(
        &practice,
        &physician,
        &email,
        "Synthetic Physician",
        false,
        now,
    )
    .await
    .expect("account");
    let token = auth
        .begin_sign_in(&email, now)
        .await
        .expect("sign-in token");
    let (identity, session) = auth
        .complete_sign_in(&email, &token, now)
        .await
        .expect("session");
    assert_eq!(&identity.practice_id, &practice);
    assert_eq!(&identity.physician_id, &physician);
    auth.register_patient(&identity.practice_id, &workspace)
        .await;
    auth.set_membership(&identity, &workspace, Membership::Allowed)
        .await;
    let scope = Scope {
        practice: identity.practice_id,
        physician: identity.physician_id,
        workspace,
        chat,
    };
    let key = format!("input-{unique}");
    store
        .accept_authorized(
            &scope,
            &Input {
                key: key.clone(),
                text: text.into(),
                timezone: "America/New_York".into(),
            },
            PostgresAuthStore::session_reference(&session),
        )
        .await
        .expect("accept");
    (
        scope.clone(),
        Delivery {
            scope,
            input_id: key,
        },
    )
}

async fn wait_assistant(store: &Store, scope: &Scope, worker: &support::Process) -> Conversation {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let Some(state) = store.read(scope).await.expect("read conversation")
            && state
                .messages
                .iter()
                .any(|message| message.role == "assistant")
        {
            return state;
        }
        assert!(
            Instant::now() < deadline,
            "assistant reply did not arrive\n{}",
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

fn assistant_texts(state: &Conversation) -> Vec<&str> {
    state
        .messages
        .iter()
        .filter(|message| message.role == "assistant")
        .map(|message| message.text.as_str())
        .collect()
}

#[tokio::test]
async fn duplicate_delivery_runs_the_fixed_provider_once() {
    let _guard = lock().lock().await;
    let Some(running) = running("dup", None, None).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&delivery).await;
    let first = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(assistant_texts(&first), [support::FIXED_TEXT]);
    assert_eq!(
        first
            .attempts
            .iter()
            .filter(|attempt| attempt.status == "completed")
            .count(),
        1,
        "first push should complete one attempt"
    );

    running.emulator.publish(&delivery).await;
    let body = support::push_body(&delivery, "duplicate-delivery");
    let (status, text) = support::exchange(
        &running.worker_host,
        "POST",
        "/pubsub/chat-turn",
        &[("content-type", "application/json")],
        Some(&body),
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", running.worker.tail()));
    assert_eq!(status, 204, "duplicate push should ack: {text}");
    tokio::time::sleep(Duration::from_secs(1)).await;
    let again = running
        .store
        .read(&scope)
        .await
        .expect("reread")
        .expect("conversation");
    assert_eq!(
        assistant_texts(&again),
        [support::FIXED_TEXT],
        "a duplicate delivery must not call the provider again"
    );
    assert_eq!(
        again
            .attempts
            .iter()
            .filter(|attempt| attempt.status == "completed")
            .count(),
        1
    );
}

#[tokio::test]
async fn failed_delivery_is_redelivered_and_can_succeed() {
    let _guard = lock().lock().await;
    let Some(running) = running("retry", Some(1), None).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&delivery).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(state.status, "completed");
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert!(
        state
            .attempts
            .iter()
            .any(|attempt| attempt.status == "failed" || attempt.status == "interrupted"),
        "the first delivery should fail before the redelivery succeeds: {:?}",
        state
            .attempts
            .iter()
            .map(|attempt| attempt.status.as_str())
            .collect::<Vec<_>>()
    );
    assert!(
        state
            .attempts
            .iter()
            .any(|attempt| attempt.status == "completed"),
        "redelivery should complete an attempt"
    );
}

fn attempt_statuses(state: &Conversation) -> Vec<&str> {
    state
        .attempts
        .iter()
        .map(|attempt| attempt.status.as_str())
        .collect()
}

/// One worker, both kinds of turn: `[fail:1]` fails once and the redelivery
/// answers; a plain turn on the same worker never fails.
#[tokio::test]
async fn fail_marker_fails_only_the_marked_message() {
    let _guard = lock().lock().await;
    let Some(running) = running_with("marker", None, None, true).await else {
        return;
    };
    let (plain_scope, plain) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&plain).await;
    let state = wait_assistant(&running.store, &plain_scope, &running.worker).await;
    assert_eq!(state.status, "completed");
    assert_eq!(attempt_statuses(&state), ["completed"]);

    let (marked_scope, marked) =
        accept_text(&running.database.url, &running.store, "Retry [fail:1]").await;
    running.emulator.publish(&marked).await;
    let state = wait_assistant(&running.store, &marked_scope, &running.worker).await;
    assert_eq!(state.status, "completed");
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert_eq!(attempt_statuses(&state), ["failed", "completed"]);

    let (again_scope, again) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&again).await;
    let state = wait_assistant(&running.store, &again_scope, &running.worker).await;
    assert_eq!(
        attempt_statuses(&state),
        ["completed"],
        "a plain turn after a marked one must not fail"
    );
}

/// `[fail:K]` counts the conversation's saved attempts, not worker memory: a
/// worker restarted after the first failure fails the second attempt only,
/// and the third answers.
#[tokio::test]
async fn fail_marker_count_survives_a_worker_restart() {
    let _guard = lock().lock().await;
    let Some(mut running) = running_with("marker_restart", None, None, true).await else {
        return;
    };
    let (scope, marked) =
        accept_text(&running.database.url, &running.store, "Restart [fail:2]").await;
    running.emulator.publish(&marked).await;
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let state = running
            .store
            .read(&scope)
            .await
            .expect("read")
            .expect("row");
        if attempt_statuses(&state).contains(&"failed") {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the first marked attempt never failed\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    running.worker.restart();
    let port = running
        .worker_host
        .rsplit_once(':')
        .and_then(|(_, port)| port.parse().ok())
        .expect("worker port");
    support::wait_ready(&mut running.worker, port).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert_eq!(
        attempt_statuses(&state),
        ["failed", "failed", "completed"],
        "a restarted worker must keep counting the saved attempts"
    );
}

#[tokio::test]
async fn fifth_failure_is_forwarded_to_the_dead_letter_topic() {
    let _guard = lock().lock().await;
    let Some(running) = running("dlq", Some(100), None).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&delivery).await;

    let deadline = Instant::now() + Duration::from_secs(60);
    let forwarded = loop {
        let messages = running.emulator.pull_dead_letters().await;
        if let Some(bytes) = messages.into_iter().find(|bytes| {
            serde_json::from_slice::<Delivery>(bytes).ok().as_ref() == Some(&delivery)
        }) {
            break bytes;
        }
        assert!(
            Instant::now() < deadline,
            "dead-letter topic did not receive the delivery\n{}\n{}",
            running.worker.tail(),
            running.emulator.tail()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    };
    let decoded: Delivery = serde_json::from_slice(&forwarded).expect("dead-letter payload");
    assert_eq!(decoded, delivery);

    let mut state = running
        .store
        .read(&scope)
        .await
        .expect("read")
        .expect("conversation");
    let attempts_deadline = Instant::now() + Duration::from_secs(5);
    while state.attempts.len() < 5 && Instant::now() < attempts_deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
        state = running
            .store
            .read(&scope)
            .await
            .expect("read")
            .expect("conversation");
    }
    assert!(
        state.attempts.len() >= 5,
        "the worker should have failed the delivery five times before the emulator forwarded it, saw {:?}",
        state
            .attempts
            .iter()
            .map(|attempt| attempt.status.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        state.status, "failed",
        "recovery gives up with the dead letter, so the turn ends as failed"
    );
    assert!(
        assistant_texts(&state).is_empty(),
        "a dead-lettered turn must not publish an assistant reply"
    );
    // The site's recovery sweep reads the same store. It must not republish
    // the turn, which would restart the five-delivery budget forever.
    let start = Scope {
        practice: scope.practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let swept = running
        .store
        .dispatch_due(100, Some(&start))
        .await
        .expect("recovery sweep");
    assert!(
        !swept.deliveries.contains(&delivery),
        "the recovery sweep republished a turn that recovery gave up on"
    );
}

#[tokio::test]
async fn concurrent_pushes_on_one_worker_run_side_by_side() {
    let _guard = lock().lock().await;
    let delay = Duration::from_secs(2);
    let Some(running) = running("parallel", None, Some(delay.as_millis() as u64)).await else {
        return;
    };
    let (first_scope, first) = accept_turn(&running.database.url, &running.store).await;
    let (second_scope, second) = accept_turn(&running.database.url, &running.store).await;
    let first_body = support::push_body(&first, "parallel-first");
    let second_body = support::push_body(&second, "parallel-second");
    let push = |body: Vec<u8>| {
        let host = running.worker_host.clone();
        async move {
            support::exchange(
                &host,
                "POST",
                "/pubsub/chat-turn",
                &[("content-type", "application/json")],
                Some(&body),
            )
            .await
        }
    };
    let started = Instant::now();
    let (a, b) = tokio::join!(push(first_body), push(second_body));
    let elapsed = started.elapsed();
    for result in [a, b] {
        let (status, text) =
            result.unwrap_or_else(|error| panic!("{error}\n{}", running.worker.tail()));
        assert_eq!(
            status, 204,
            "push should ack after the turn completes: {text}"
        );
    }
    assert!(
        elapsed >= delay,
        "pushes acked after {elapsed:?}, before the {delay:?} turn could finish"
    );
    assert!(
        elapsed < delay * 2 - Duration::from_millis(500),
        "two {delay:?} turns took {elapsed:?}; the worker ran them one after the other"
    );
    for scope in [&first_scope, &second_scope] {
        let state = wait_assistant(&running.store, scope, &running.worker).await;
        assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    }
}
