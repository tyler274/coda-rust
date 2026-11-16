//! Physician worker against a disposable Postgres, with no broker.
//!
//! Accepting a turn notifies the worker in the same transaction. Hearing of
//! settled work again runs the fixed provider once, a failed attempt is due
//! again and can then succeed, a request the provider rejects holds the turn
//! and is not sent again, and the fifth failure ends the turn as failed so
//! nothing starts it again. A worker whose listener is gone still finds
//! accepted work by polling. Concurrent turns on one worker run side by side.
//! A chat run's memory tools save notes under the run's own scope, and so
//! do a reply revision's, each with the quote the model gave. Every request
//! of a chat opens with the memory the chat started with, the next chat for
//! the same patient starts with those notes, and a recovered run sends the
//! memory its first attempt sent.

use std::time::{Duration, Instant};

use local_product::physician_chat::PhysicianSkills;
use local_product::runtime::delivery::{Backoff, Delivery, DeliveryState, TraceContext};
use local_product::runtime::{
    AttemptStatus, Conversation, Input, Resumed, Scope, Stopped, Store, WorkStatus,
};
use physician_auth::{AuthStore, Membership, PostgresAuthStore};
#[path = "scale/mod.rs"]
mod support;

use support::{ExportedSpan, TraceCapture, trace_attribute};

fn lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

struct Running {
    stub: support::ResponsesStub,
    database: support::Database,
    store: Store,
    worker: support::Process,
    worker_host: String,
    home: std::path::PathBuf,
}

impl Running {
    fn port(&self) -> u16 {
        let (_, port) = self.worker_host.rsplit_once(':').expect("worker host");
        port.parse().expect("worker port")
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        self.worker.kill_now();
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

async fn running(label: &str, failures: Option<u32>, delay_ms: Option<u64>) -> Option<Running> {
    running_with(label, failures, delay_ms, "[]", support::FIXED_TEXT, &[]).await
}

async fn running_with(
    label: &str,
    failures: Option<u32>,
    delay_ms: Option<u64>,
    rounds: &str,
    reply: &str,
    extra: &[(&str, &str)],
) -> Option<Running> {
    let database = support::Database::create(label).await?;
    let store = Store::connect(&database.url).await.expect("runtime store");
    let worker_port = support::free_port();
    let home = std::env::temp_dir().join(format!(
        "physician-scale-worker-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("worker home");
    let listen = format!("127.0.0.1:{worker_port}");
    let stub = support::ResponsesStub::start_with_reply(
        &home,
        failures.unwrap_or(0),
        delay_ms.unwrap_or(0),
        rounds,
        reply,
    )
    .await;
    let mut env = vec![
        ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
        ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
        ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
        support::RETRY_NOW,
    ];
    env.extend_from_slice(extra);
    let mut worker = support::Process::spawn("worker", support::worker_bin(), &[], &env, &home);
    support::wait_ready(&mut worker, worker_port).await;
    Some(Running {
        stub,
        database,
        store,
        worker,
        worker_host: listen,
        home,
    })
}

async fn accept_turn(url: &str, store: &Store) -> (Scope, Delivery) {
    accept_text(url, store, "Hello").await
}

async fn accept_text(url: &str, store: &Store, text: &str) -> (Scope, Delivery) {
    accept_in(url, store, text, None, None).await
}

/// Accepts `text` from a new physician in `practice` (a new one when `None`),
/// tagged with `scenario`.
async fn accept_in(
    url: &str,
    store: &Store,
    text: &str,
    practice: Option<&str>,
    scenario: Option<&str>,
) -> (Scope, Delivery) {
    let skills = PhysicianSkills::configured(None, false).unwrap();
    accept_as(url, store, text, &skills, practice, scenario, None).await
}

/// Accepts `text` whose accepting request carried the trace context `origin`.
async fn accept_from(
    url: &str,
    store: &Store,
    text: &str,
    origin: Option<TraceContext>,
) -> (Scope, Delivery) {
    let skills = PhysicianSkills::configured(None, false).unwrap();
    accept_as(url, store, text, &skills, None, None, origin).await
}

/// Accepts one authorized turn, as the site does: the store notifies the
/// worker in the accepting transaction. `origin` is the accepting request's
/// trace context.
async fn accept_as(
    url: &str,
    store: &Store,
    text: &str,
    skills: &PhysicianSkills,
    practice: Option<&str>,
    scenario: Option<&str>,
    origin: Option<TraceContext>,
) -> (Scope, Delivery) {
    let (scope, session) = signed_in(url, practice).await;
    let delivery = accept_signed_in(store, &scope, &session, text, skills, scenario, origin).await;
    (scope, delivery)
}

/// Accepts one turn in `scope` from the physician signed in as `session`.
async fn accept_signed_in(
    store: &Store,
    scope: &Scope,
    session: &str,
    text: &str,
    skills: &PhysicianSkills,
    scenario: Option<&str>,
    origin: Option<TraceContext>,
) -> Delivery {
    let key = format!("input-{}", uuid::Uuid::new_v4().simple());
    store
        .accept_authorized(
            scope,
            &Input {
                key: key.clone(),
                text: text.into(),
                timezone: "America/New_York".into(),
                scenario: scenario.map(str::to_owned),
            },
            PostgresAuthStore::session_reference(session),
            skills,
            &local_product::physician_chat::recipe(
                &support::models::roles().chat,
                local_product::physician_chat::INSTRUCTIONS,
                skills,
            ),
            origin,
        )
        .await
        .expect("accept");
    Delivery::chat_turn(scope.clone(), key)
}

/// A new physician in `practice` (a new one when `None`), signed in and
/// allowed into a new patient workspace: the scope of a new chat there, and
/// the session's bearer token.
async fn signed_in(url: &str, practice: Option<&str>) -> (Scope, String) {
    let auth = PostgresAuthStore::connect_without_migrations(url)
        .await
        .expect("auth store");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default();
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let email = format!("worker-{unique}@example.test");
    let practice = practice.map_or_else(|| format!("practice-{unique}"), str::to_owned);
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
        .await
        .unwrap();
    auth.set_membership(&identity, &workspace, Membership::Allowed)
        .await
        .unwrap();
    let scope = Scope {
        practice: identity.practice_id,
        physician: identity.physician_id,
        workspace,
        chat,
    };
    (scope, session)
}

async fn wait_assistant(store: &Store, scope: &Scope, worker: &support::Process) -> Conversation {
    wait_assistant_for(store, scope, worker, Duration::from_secs(30)).await
}

async fn wait_assistant_for(
    store: &Store,
    scope: &Scope,
    worker: &support::Process,
    wait: Duration,
) -> Conversation {
    let deadline = Instant::now() + wait;
    loop {
        if let Some(state) = store.read(scope).await.expect("read conversation")
            && state
                .messages
                .iter()
                .any(|message| message.role.as_str() == "assistant")
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
        .filter(|message| message.role.as_str() == "assistant")
        .map(|message| message.text.as_str())
        .collect()
}

#[tokio::test]
async fn hearing_of_settled_work_again_runs_the_fixed_provider_once() {
    let _guard = lock().lock().await;
    let Some(running) = running("dup", None, None).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    let first = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(assistant_texts(&first), [support::FIXED_TEXT]);
    assert_eq!(attempt_statuses(&first), ["completed"]);

    for _ in 0..3 {
        running.database.announce(&delivery).await;
    }
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
        "settled work heard of again must not call the provider again"
    );
    assert_eq!(attempt_statuses(&again), ["completed"]);
}

/// The worker serves health and readiness only: no route takes work, so
/// nothing can hand it work over HTTP.
#[tokio::test]
async fn the_worker_serves_no_work_route() {
    let _guard = lock().lock().await;
    let Some(running) = running("no_route", None, None).await else {
        return;
    };
    let (_, delivery) = accept_turn(&running.database.url, &running.store).await;
    let body = serde_json::to_vec(&delivery).expect("delivery");
    for path in ["/pubsub/chat-turn", "/"] {
        let (status, text) = support::exchange(
            &running.worker_host,
            "POST",
            path,
            &[("content-type", "application/json")],
            Some(&body),
        )
        .await
        .unwrap_or_else(|error| panic!("{error}\n{}", running.worker.tail()));
        assert_eq!(status, 404, "POST {path}: {text}");
    }
    for path in ["/health", "/readyz"] {
        let (status, text) = support::exchange(&running.worker_host, "GET", path, &[], None)
            .await
            .unwrap_or_else(|error| panic!("{error}\n{}", running.worker.tail()));
        assert_eq!(status, 200, "GET {path}: {text}");
    }
}

/// AC7. A hosted worker starts and is ready with the staging profile and no
/// push identity: neither `PHYSICIAN_WORKER_AUDIENCE` nor
/// `PHYSICIAN_PUSH_SERVICE_ACCOUNT` is set. It runs no turn, so the API key
/// is never used.
#[tokio::test]
async fn a_hosted_worker_starts_without_push_identity_settings() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("hosted_start").await else {
        return;
    };
    let home = std::env::temp_dir().join(format!(
        "physician-worker-hosted-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("home");
    let port = support::free_port();
    let listen = format!("127.0.0.1:{port}");
    let skills = concat!(env!("CARGO_MANIFEST_DIR"), "/../../skills");
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("K_SERVICE", "physician-worker"),
            ("CODA_ENVIRONMENT", "stg"),
            (
                "CODA_HOSTED_PROFILE",
                physician_sample::hosting::STAGING_PROFILE,
            ),
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
            ("PHYSICIAN_SKILLS_DIR", skills),
            ("CODA_LOCAL_MODEL_EXECUTION", "api"),
            ("CODA_ALLOW_MODEL_CALLS", "1"),
            ("OPENAI_API_KEY", "unused-no-turn-runs"),
        ],
        &home,
    );
    support::wait_ready(&mut worker, port).await;
    let host = format!("127.0.0.1:{port}");
    let (status, _) = support::exchange(&host, "POST", "/pubsub/chat-turn", &[], Some(b"{}"))
        .await
        .unwrap_or_else(|error| panic!("{error}\n{}", worker.tail()));
    assert_eq!(status, 404);
    println!("hosted worker ready without push settings; POST /pubsub/chat-turn -> {status}");
    worker.kill_now();
    let _ = std::fs::remove_dir_all(&home);
}

/// AC2. The worker's listening connection is ended, as a dropped connection
/// or a database failover would end it, and a turn is accepted at once. The
/// worker still claims it within its poll interval, here the hosted 10 s.
#[tokio::test]
async fn a_worker_whose_listener_is_gone_claims_accepted_work_within_the_poll_interval() {
    let _guard = lock().lock().await;
    let Some(running) = running("listener_gone", None, None).await else {
        return;
    };
    // The listener is up once a first turn is answered or polled; either way
    // a listening session exists before it is ended.
    let deadline = Instant::now() + Duration::from_secs(10);
    let ended = loop {
        let ended = running.database.terminate_listeners().await;
        if ended > 0 {
            break ended;
        }
        assert!(Instant::now() < deadline, "the worker never listened");
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    assert_eq!(ended, 1, "one worker, one listening session");
    let accepted = Instant::now();
    let (scope, _) = accept_turn(&running.database.url, &running.store).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    let waited = accepted.elapsed();
    println!("listener ended; accepted turn answered after {waited:?} (poll interval 10 s)");
    assert_eq!(attempt_statuses(&state), ["completed"]);
    assert!(
        waited < Duration::from_secs(11),
        "the turn waited {waited:?}, longer than the poll interval"
    );
    assert!(
        running
            .worker
            .tail()
            .contains("physician work listener disconnected"),
        "the worker never noticed its listener was gone\n{}",
        running.worker.tail()
    );
}

/// AC2, the poll on its own. Work becomes due with no notification at all: a
/// worker claimed the turn and died, and its lease ran out. Only the timed
/// read of the due work can find it.
#[tokio::test]
async fn work_that_becomes_due_unannounced_is_found_by_the_poll() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("poll_only").await else {
        return;
    };
    let store = Store::connect(&database.url).await.expect("runtime store");
    let (scope, delivery) = accept_turn(&database.url, &store).await;
    // The dead worker's claim. Nothing announces its lease running out.
    let lease = Duration::from_secs(3);
    store
        .claim_delivery(&delivery, lease.as_secs() as i64)
        .await
        .expect("claim")
        .expect("the accepted turn is claimable");
    let claimed = Instant::now();
    let home = std::env::temp_dir().join(format!(
        "physician-worker-poll-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("home");
    let stub = support::ResponsesStub::start(&home, 0, 0, "[]").await;
    let port = support::free_port();
    let listen = format!("127.0.0.1:{port}");
    let poll = Duration::from_millis(500);
    let poll_ms = poll.as_millis().to_string();
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
            ("PHYSICIAN_WORKER_POLL_MS", poll_ms.as_str()),
        ],
        &home,
    );
    support::wait_ready(&mut worker, port).await;
    // The worker's first reads of the due work fall inside the live lease.
    assert!(claimed.elapsed() < lease, "the worker started too late");
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Claimed
    );
    let state = wait_assistant(&store, &scope, &worker).await;
    let waited = claimed.elapsed();
    println!(
        "unannounced work answered {waited:?} after its {lease:?} lease began (poll {poll:?})"
    );
    assert_eq!(attempt_statuses(&state), ["interrupted", "completed"]);
    // The database clock counts whole seconds, so allow one beyond the lease.
    assert!(
        waited < lease + Duration::from_secs(1) + poll * 2,
        "the poll took {waited:?} to find work due after {lease:?}"
    );
    worker.kill_now();
    let _ = std::fs::remove_dir_all(&home);
}

/// AC4 without the poll. A dead worker's lease is live at every read before
/// it runs out, and the poll is far slower than the lease, so only the
/// worker's wake at the soonest lease end can take the turn in time.
#[tokio::test]
async fn a_lease_that_runs_out_unannounced_is_taken_at_its_end_not_at_the_next_poll() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("lease_end").await else {
        return;
    };
    let store = Store::connect(&database.url).await.expect("runtime store");
    let (scope, delivery) = accept_turn(&database.url, &store).await;
    let lease = Duration::from_secs(4);
    store
        .claim_delivery(&delivery, lease.as_secs() as i64)
        .await
        .expect("claim")
        .expect("the accepted turn is claimable");
    let claimed = Instant::now();
    let home = std::env::temp_dir().join(format!(
        "physician-worker-lease-end-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("home");
    let stub = support::ResponsesStub::start(&home, 0, 0, "[]").await;
    let port = support::free_port();
    let listen = format!("127.0.0.1:{port}");
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
            ("PHYSICIAN_WORKER_POLL_MS", "60000"),
        ],
        &home,
    );
    support::wait_ready(&mut worker, port).await;
    assert!(claimed.elapsed() < lease, "the worker started too late");
    let state = wait_assistant(&store, &scope, &worker).await;
    let waited = claimed.elapsed();
    println!(
        "unannounced lease end answered {waited:?} after its {lease:?} lease began (poll 60s)"
    );
    assert_eq!(attempt_statuses(&state), ["interrupted", "completed"]);
    // The database clock counts whole seconds, so allow one beyond the lease.
    assert!(
        waited < lease + Duration::from_secs(2),
        "the turn waited {waited:?} for a {lease:?} lease"
    );
    worker.kill_now();
    let _ = std::fs::remove_dir_all(&home);
}

/// AC4 after Stop. The worker running a turn is stopped and dies before it
/// settles, so its cancelled attempt keeps the claim until the lease runs
/// out. Try again makes the turn pending at once, but nothing can claim it
/// before then and nothing announces the lease ending. The poll is far
/// slower than the lease, so only the wake at the soonest lease end, which
/// counts the stopped attempt's lease, takes the turn in time.
#[tokio::test]
async fn work_resumed_after_a_stopped_worker_died_is_taken_at_its_lease_end() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("stopped_lease_end").await else {
        return;
    };
    let store = Store::connect(&database.url).await.expect("runtime store");
    let (scope, session) = signed_in(&database.url, None).await;
    let skills = PhysicianSkills::configured(None, false).unwrap();
    let session_hash = PostgresAuthStore::session_reference(&session);
    let key = format!("input-{}", uuid::Uuid::new_v4().simple());
    store
        .accept_authorized(
            &scope,
            &Input {
                key: key.clone(),
                text: "Hello".into(),
                timezone: "America/New_York".into(),
                scenario: None,
            },
            session_hash.clone(),
            &skills,
            &local_product::physician_chat::recipe(
                &support::models::roles().chat,
                local_product::physician_chat::INSTRUCTIONS,
                &skills,
            ),
            None,
        )
        .await
        .expect("accept");
    let delivery = Delivery::chat_turn(scope.clone(), key);
    let lease = Duration::from_secs(4);
    store
        .claim_delivery(&delivery, lease.as_secs() as i64)
        .await
        .expect("claim")
        .expect("the accepted turn is claimable");
    let claimed = Instant::now();
    let work = store
        .read(&scope)
        .await
        .expect("read")
        .expect("conversation")
        .work;
    assert_eq!(
        store
            .stop_authorized(&scope, "stop", session_hash.clone(), &work)
            .await
            .expect("stop"),
        Stopped::Held
    );
    let held = store.read(&scope).await.expect("read").expect("held");
    let run = held.run().expect("held run").id.to_owned();
    assert!(matches!(
        store
            .resume_authorized(&scope, "resume", session_hash, &work, &run, None)
            .await
            .expect("resume"),
        Resumed::Pending { .. }
    ));
    let home = std::env::temp_dir().join(format!(
        "physician-worker-stopped-lease-end-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("home");
    let stub = support::ResponsesStub::start(&home, 0, 0, "[]").await;
    let port = support::free_port();
    let listen = format!("127.0.0.1:{port}");
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
            ("PHYSICIAN_WORKER_POLL_MS", "60000"),
        ],
        &home,
    );
    support::wait_ready(&mut worker, port).await;
    assert!(claimed.elapsed() < lease, "the worker started too late");
    let state = wait_assistant(&store, &scope, &worker).await;
    let waited = claimed.elapsed();
    println!(
        "resumed turn behind a stopped lease answered {waited:?} after its {lease:?} lease began (poll 60s)"
    );
    assert_eq!(attempt_statuses(&state), ["cancelled", "completed"]);
    // The database clock counts whole seconds, so allow one beyond the lease.
    assert!(
        waited < lease + Duration::from_secs(2),
        "the turn waited {waited:?} for a stopped {lease:?} lease"
    );
    worker.kill_now();
    let _ = std::fs::remove_dir_all(&home);
}

#[tokio::test]
async fn a_failed_attempt_is_retried_and_links_to_the_accepting_request() {
    let _guard = lock().lock().await;
    let collector = TraceCapture::start().await;
    let Some(running) = running_with(
        "retry",
        Some(1),
        None,
        "[]",
        support::FIXED_TEXT,
        &[
            ("CODA_LOG_FORMAT", "json"),
            ("OTEL_EXPORTER_OTLP_ENDPOINT", &collector.endpoint),
            ("OTEL_BSP_SCHEDULE_DELAY", "50"),
        ],
    )
    .await
    else {
        return;
    };
    let (scope, _) = accept_from(
        &running.database.url,
        &running.store,
        "Hello",
        Some(TraceContext {
            traceparent: "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01".into(),
            tracestate: Some("dd=s:1".into()),
        }),
    )
    .await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert_eq!(attempt_statuses(&state), ["failed", "completed"]);
    assert_eq!(state.attempts.len(), 2);
    let first = &state.attempts[0];
    let second = &state.attempts[1];
    assert_ne!(
        first.id, second.id,
        "the retry claims a new canonical attempt"
    );
    assert_eq!(first.work, second.work);
    assert_eq!(second.predecessor.as_deref(), Some(first.id.as_str()));
    // The stub's broken response ends the first exchange as a failure: it is
    // neither retried within the run nor held as a provider block.
    use harness::engine::EventKind;
    assert!(
        first
            .events
            .iter()
            .any(|event| matches!(event.kind, EventKind::ProviderFailed { .. })),
        "{:?}",
        first.events
    );
    assert!(!first.events.iter().any(|event| matches!(
        event.kind,
        EventKind::TransportRetry { .. } | EventKind::ProviderBlocked { .. }
    )));
    assert_eq!(
        running.stub.requests().len(),
        2,
        "one request for each attempt"
    );
    let logs = running.worker.tail();
    let failure: serde_json::Value = logs
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .find(|event| event["event"] == "physician.worker.attempt_failed")
        .unwrap_or_else(|| panic!("missing asynchronous failure event: {logs}"));
    assert_eq!(failure["coda.run_id"], first.id);
    assert_eq!(failure["error.type"], "model_call");
    let deadline = Instant::now() + Duration::from_secs(10);
    let spans = loop {
        let spans = collector.spans.lock().unwrap().clone();
        if spans
            .iter()
            .filter(|span| span.name == "gen_ai.generation")
            .count()
            == 2
        {
            break spans;
        }
        assert!(
            Instant::now() < deadline,
            "worker traces not exported: {spans:?}\n{logs}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    for saved in [first, second] {
        let find = |name| {
            spans
                .iter()
                .find(|span| {
                    span.name == name
                        && trace_attribute(span, "coda.run_id").as_deref()
                            == Some(saved.id.as_str())
                })
                .unwrap()
        };
        let attempt = find("physician.worker.attempt");
        let generation = find("gen_ai.generation");
        assert_eq!(generation.trace_id, attempt.trace_id);
        assert_eq!(generation.parent_span_id, attempt.span_id);
        let accepting_trace =
            opentelemetry::trace::TraceId::from_hex("0af7651916cd43dd8448eb211c80319c")
                .unwrap()
                .to_bytes();
        // Every attempt is its own trace and links to the request that
        // accepted the turn; one accepted input can have several attempts.
        assert_ne!(
            attempt.trace_id, accepting_trace,
            "attempt span: {attempt:?}"
        );
        assert_eq!(
            trace_attribute(attempt, "coda.trace_relation").as_deref(),
            Some("link")
        );
        assert_eq!(attempt.links.len(), 1);
        assert_eq!(attempt.links[0].trace_id, accepting_trace);
        assert_eq!(
            attempt.links[0].span_id,
            opentelemetry::trace::SpanId::from_hex("b7ad6b7169203331")
                .unwrap()
                .to_bytes()
        );
        assert_eq!(attempt.links[0].trace_state, "dd=s:1");
        assert_eq!(
            trace_attribute(attempt, "messaging.system").as_deref(),
            Some("postgresql")
        );
        assert_eq!(
            trace_attribute(attempt, "coda.work_id").as_deref(),
            Some(saved.work.as_str())
        );
    }
    let failed = spans
        .iter()
        .find(|span| {
            span.name == "physician.worker.attempt"
                && trace_attribute(span, "coda.run_id").as_deref() == Some(first.id.as_str())
        })
        .unwrap();
    assert_eq!(
        failure["dd.trace_id"],
        telemetry::dd_trace_id(opentelemetry::trace::TraceId::from_bytes(
            failed.trace_id.as_slice().try_into().unwrap()
        ))
    );
    assert!(
        spans.iter().all(|span| !span.name.contains("pubsub")),
        "no work arrives over HTTP"
    );
}

/// A turn in a rehearsal practice is `simulation` with its scenario on the
/// generation; a turn in any other practice reports no scenario, even if its
/// input somehow carries one. This worker's model is the local Responses stub,
/// so its own origin is `simulation` for every turn; `hosting`'s unit tests
/// cover a real practice staying `real`.
#[tokio::test]
async fn a_request_the_provider_rejects_holds_the_turn_and_is_not_sent_again() {
    let _guard = lock().lock().await;
    let Some(running) = running("rejected", None, None).await else {
        return;
    };
    running.stub.reject_next(1);
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    let deadline = Instant::now() + Duration::from_secs(30);
    let held = loop {
        let state = running.store.conversation(&scope).await.unwrap();
        if state.status == WorkStatus::Held {
            break state;
        }
        assert!(
            Instant::now() < deadline,
            "the rejected turn was not held: {state:?}\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    };
    assert_eq!(attempt_statuses(&held), ["held"]);
    assert_eq!(
        held.run().unwrap().outcome,
        Some(harness::engine::runs::RunOutcome::Held {
            reason: harness::engine::runs::HoldReason::ProviderBlocked
        })
    );
    assert_eq!(
        held.failures, 0,
        "a rejected request is not a failed attempt"
    );
    assert_eq!(running.stub.requests().len(), 1);

    // Hearing of the held turn again does not send the request again.
    running.database.announce(&delivery).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let after = running.store.conversation(&scope).await.unwrap();
    assert_eq!(attempt_statuses(&after), ["held"]);
    assert!(assistant_texts(&after).is_empty());
    assert_eq!(
        running.stub.requests().len(),
        1,
        "a held turn must not send the rejected request again"
    );
    // The poll leaves the held turn alone.
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
        .expect("due work");
    assert!(
        !swept.deliveries.contains(&delivery),
        "a held turn is still due"
    );
}

#[tokio::test]
async fn origin_follows_the_practice_of_the_claimed_turn() {
    let _guard = lock().lock().await;
    let collector = TraceCapture::start().await;
    let rehearsal = format!("rehearsal-{}", uuid::Uuid::new_v4().simple());
    let Some(running) = running_with(
        "origin",
        None,
        None,
        "[]",
        support::FIXED_TEXT,
        &[
            ("CODA_HOSTED_PROFILE", "staging-preview"),
            ("PHYSICIAN_SIMULATION_PRACTICES", rehearsal.as_str()),
            ("OTEL_EXPORTER_OTLP_ENDPOINT", &collector.endpoint),
            ("OTEL_BSP_SCHEDULE_DELAY", "50"),
        ],
    )
    .await
    else {
        return;
    };
    let url = &running.database.url;
    let (simulated_scope, _) = accept_in(
        url,
        &running.store,
        "Rehearse",
        Some(&rehearsal),
        Some("drill.rehearse"),
    )
    .await;
    let (other_scope, _) =
        accept_in(url, &running.store, "Hello", None, Some("drill.rehearse")).await;
    let simulated = wait_assistant(&running.store, &simulated_scope, &running.worker).await;
    let other = wait_assistant(&running.store, &other_scope, &running.worker).await;
    let deadline = Instant::now() + Duration::from_secs(10);
    let generation = |spans: &[ExportedSpan], state: &Conversation| {
        let run = state.attempts.last().map(|attempt| attempt.id.clone());
        spans
            .iter()
            .find(|span| {
                span.name == "gen_ai.generation" && trace_attribute(span, "coda.run_id") == run
            })
            .cloned()
    };
    let (simulated_span, other_span) = loop {
        let spans = collector.spans.lock().unwrap().clone();
        if let (Some(a), Some(b)) = (generation(&spans, &simulated), generation(&spans, &other)) {
            break (a, b);
        }
        assert!(
            Instant::now() < deadline,
            "generations not exported\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    let tags = |span: &ExportedSpan| -> Vec<String> {
        use opentelemetry_proto::tonic::common::v1::any_value::Value;
        let Some(Value::ArrayValue(array)) = span
            .attributes
            .iter()
            .find(|kv| kv.key == "langfuse.trace.tags")
            .and_then(|kv| kv.value.as_ref())
            .and_then(|value| value.value.clone())
        else {
            return Vec::new();
        };
        array
            .values
            .into_iter()
            .filter_map(|value| match value.value {
                Some(Value::StringValue(text)) => Some(text),
                _ => None,
            })
            .collect()
    };
    assert_eq!(
        trace_attribute(&simulated_span, "langfuse.environment").as_deref(),
        Some("simulation")
    );
    assert_eq!(
        trace_attribute(&simulated_span, "coda.scenario_id").as_deref(),
        Some("drill.rehearse")
    );
    assert_eq!(
        trace_attribute(&simulated_span, "langfuse.session.id").as_deref(),
        Some(simulated_scope.chat.as_str())
    );
    assert_eq!(
        tags(&simulated_span),
        ["physician_chat", "simulation", "drill.rehearse"]
    );
    assert_eq!(
        trace_attribute(&other_span, "langfuse.environment").as_deref(),
        Some("simulation")
    );
    assert_eq!(trace_attribute(&other_span, "coda.scenario_id"), None);
    assert_eq!(tags(&other_span), ["physician_chat", "simulation"]);
}

/// The full export path with content on: the worker's generation goes
/// through the real collector config to a real Langfuse, which holds the
/// prompt and reply under the rehearsal's environment, session and run; the
/// operations pipelines deliver the same generation, with its model, to the
/// test's sink and carry neither text anywhere. Needs Docker with the images
/// in `dev/observability/test/langfuse.yaml` pulled, plus the Postgres gate.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "starts Langfuse and the collector in Docker"]
async fn content_reaches_langfuse_and_never_the_operations_backend() {
    use telemetry::testing::{Langfuse, PUBLIC_KEY, SECRET_KEY, Shape};
    let _guard = lock().lock().await;
    if support::gate_url().is_none() {
        return;
    }
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let prompt_canary = format!("full-path-prompt-{unique}");
    let reply_canary = format!("full-path-reply-{unique}");
    let reply = format!("Noted, {reply_canary}.");
    let ops = TraceCapture::start().await;
    let langfuse = tokio::task::spawn_blocking({
        let port = ops.port;
        move || {
            Langfuse::start(
                "physician-full-path",
                Shape::Ingest {
                    ops_sink_port: port,
                },
            )
        }
    })
    .await
    .expect("compose");
    let otlp = langfuse.otlp_http.clone().expect("collector receiver");
    let rehearsal = format!("rehearsal-{unique}");
    let Some(running) = running_with(
        "full_path",
        None,
        None,
        "[]",
        reply.as_str(),
        &[
            ("CODA_HOSTED_PROFILE", "staging-preview"),
            ("PHYSICIAN_SIMULATION_PRACTICES", rehearsal.as_str()),
            ("CODA_TELEMETRY_CONTENT", "full"),
            ("OTEL_EXPORTER_OTLP_ENDPOINT", otlp.as_str()),
            ("OTEL_BSP_SCHEDULE_DELAY", "50"),
        ],
    )
    .await
    else {
        return;
    };
    let (scope, _) = accept_in(
        &running.database.url,
        &running.store,
        &format!("Rehearse {prompt_canary}"),
        Some(&rehearsal),
        Some("drill.full-path"),
    )
    .await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(assistant_texts(&state), [reply.as_str()]);
    let run = state.attempts.last().expect("an attempt").id.clone();

    // The operations sink: the generation, its run and model, no content.
    let deadline = Instant::now() + Duration::from_secs(60);
    let generation = loop {
        let found = ops
            .spans
            .lock()
            .unwrap()
            .iter()
            .find(|span| {
                span.name == "gen_ai.generation"
                    && trace_attribute(span, "coda.run_id").as_deref() == Some(run.as_str())
            })
            .cloned();
        if let Some(span) = found {
            break span;
        }
        assert!(
            Instant::now() < deadline,
            "no generation reached the operations sink\n{}\n{}",
            running.worker.tail(),
            langfuse.logs()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    };
    assert!(trace_attribute(&generation, "gen_ai.request.model").is_some());
    assert_eq!(
        trace_attribute(&generation, "coda.scenario_id").as_deref(),
        Some("drill.full-path")
    );
    for key in [
        "langfuse.observation.input",
        "langfuse.observation.output",
        "langfuse.session.id",
        "langfuse.trace.tags",
    ] {
        assert_eq!(trace_attribute(&generation, key), None, "{key} reached ops");
    }
    let trace_id: String = generation
        .trace_id
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();

    // Langfuse: the same trace, with the rehearsal's labels and the content.
    let client = reqwest::Client::new();
    let deadline = Instant::now() + Duration::from_secs(180);
    let trace: serde_json::Value = loop {
        let response = client
            .get(format!("{}/api/public/traces/{trace_id}", langfuse.url))
            .basic_auth(PUBLIC_KEY, Some(SECRET_KEY))
            .send()
            .await;
        if let Ok(response) = response
            && response.status().is_success()
        {
            let body: serde_json::Value = response.json().await.expect("trace JSON");
            let generated = body["observations"]
                .as_array()
                .is_some_and(|all| all.iter().any(|o| o["name"] == "gen_ai.generation"));
            if generated {
                break body;
            }
        }
        assert!(
            Instant::now() < deadline,
            "Langfuse never held trace {trace_id}\n{}",
            langfuse.logs()
        );
        tokio::time::sleep(Duration::from_secs(2)).await;
    };
    let observation = trace["observations"]
        .as_array()
        .and_then(|all| all.iter().find(|o| o["name"] == "gen_ai.generation"))
        .expect("generation observation");
    assert_eq!(trace["sessionId"], scope.chat.as_str(), "{trace}");
    assert_eq!(observation["environment"], "simulation", "{observation}");
    assert_eq!(observation["type"], "GENERATION", "{observation}");
    let metadata = observation["metadata"].to_string();
    assert!(metadata.contains(&run), "run id not in {metadata}");
    assert!(
        observation["input"].to_string().contains(&prompt_canary),
        "prompt missing from {observation}"
    );
    assert!(
        observation["output"].to_string().contains(&reply_canary),
        "reply missing from {observation}"
    );

    // Everything the operations pipelines delivered, traces and logs, raw.
    tokio::time::sleep(Duration::from_secs(2)).await;
    let bodies = ops.bodies.lock().unwrap().clone();
    assert!(!bodies.is_empty());
    for body in &bodies {
        for canary in [&prompt_canary, &reply_canary] {
            assert!(
                !body
                    .windows(canary.len())
                    .any(|window| window == canary.as_bytes()),
                "{canary} reached the operations sink"
            );
        }
    }
    drop(running);
    drop(langfuse);
}

/// Each attempt's status as its saved text.
fn attempt_statuses(state: &Conversation) -> Vec<String> {
    state
        .attempts
        .iter()
        .map(|attempt| sea_orm::ActiveEnum::to_value(&attempt.status))
        .collect()
}

/// A rehearsal practice's `[fail:1]` fails its first attempt on purpose,
/// before any model call: the attempt span says `rehearsed`, and the retry
/// completes the same work. A real practice's `[fail:1]` answers on its
/// first attempt.
#[tokio::test]
async fn a_rehearsal_turn_marked_to_fail_fails_once_and_its_successor_answers() {
    let _guard = lock().lock().await;
    let collector = TraceCapture::start().await;
    let rehearsal = format!("rehearsal-{}", uuid::Uuid::new_v4().simple());
    let Some(running) = running_with(
        "rehearsed_failure",
        None,
        None,
        "[]",
        support::FIXED_TEXT,
        &[
            ("CODA_HOSTED_PROFILE", "staging-preview"),
            ("PHYSICIAN_SIMULATION_PRACTICES", rehearsal.as_str()),
            ("OTEL_EXPORTER_OTLP_ENDPOINT", &collector.endpoint),
            ("OTEL_BSP_SCHEDULE_DELAY", "50"),
        ],
    )
    .await
    else {
        return;
    };
    let url = &running.database.url;
    let (scope, _) = accept_in(
        url,
        &running.store,
        "Drill [fail:1]",
        Some(&rehearsal),
        Some("drill.chat-failure"),
    )
    .await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert_eq!(attempt_statuses(&state), ["failed", "completed"]);
    let (failed, successor) = (&state.attempts[0], &state.attempts[1]);
    assert_eq!(failed.work, successor.work, "one work across both attempts");
    assert_eq!(successor.predecessor.as_deref(), Some(failed.id.as_str()));

    let deadline = Instant::now() + Duration::from_secs(10);
    let failed_span = loop {
        let found = collector
            .spans
            .lock()
            .unwrap()
            .iter()
            .find(|span| {
                span.name == "physician.worker.attempt"
                    && trace_attribute(span, "coda.run_id").as_deref() == Some(failed.id.as_str())
            })
            .cloned();
        if let Some(span) = found {
            break span;
        }
        assert!(
            Instant::now() < deadline,
            "failed attempt span not exported\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    assert_eq!(
        trace_attribute(&failed_span, "error.type").as_deref(),
        Some("rehearsed")
    );
    assert_eq!(
        trace_attribute(&failed_span, "coda.work_id").as_deref(),
        Some(failed.work.as_str())
    );

    let (real_scope, _) = accept_in(url, &running.store, "Real [fail:1]", None, None).await;
    let state = wait_assistant(&running.store, &real_scope, &running.worker).await;
    assert_eq!(
        attempt_statuses(&state),
        ["completed"],
        "a real practice never honours the marker"
    );
}

#[tokio::test]
async fn the_fifth_failure_ends_the_turn_and_nothing_starts_it_again() {
    let _guard = lock().lock().await;
    // A retry delay, so the restart lands between the second and third
    // attempts rather than racing an immediate retry.
    let Some(mut running) = running_with(
        "gave_up",
        Some(100),
        None,
        "[]",
        support::FIXED_TEXT,
        &[("PHYSICIAN_WORKER_RETRY_SECONDS", "1")],
    )
    .await
    else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;

    let restart_deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let state = running.store.conversation(&scope).await.unwrap();
        if state.failures == 2 {
            assert_eq!(attempt_statuses(&state), ["failed", "failed"]);
            running.worker.restart();
            let port = running.port();
            support::wait_ready(&mut running.worker, port).await;
            break;
        }
        assert!(
            state.failures < 2,
            "missed the second failed attempt: {state:?}"
        );
        assert!(
            Instant::now() < restart_deadline,
            "second failure did not arrive"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    let deadline = Instant::now() + Duration::from_secs(60);
    let state = loop {
        let state = running
            .store
            .read(&scope)
            .await
            .expect("read")
            .expect("conversation");
        if state.status == WorkStatus::Failed {
            break state;
        }
        assert!(
            Instant::now() < deadline,
            "the turn never ended as failed: {:?}\n{}",
            attempt_statuses(&state),
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    };
    assert_eq!(attempt_statuses(&state), ["failed"; 5]);
    assert!(
        assistant_texts(&state).is_empty(),
        "a turn that gave up must not publish an assistant reply"
    );
    assert_eq!(
        running
            .store
            .delivery_state(&delivery)
            .await
            .expect("state"),
        DeliveryState::Failed
    );
    // Neither the poll nor another announcement starts a sixth attempt.
    let start = Scope {
        practice: scope.practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let due = running
        .store
        .dispatch_due(100, Some(&start))
        .await
        .expect("due work");
    assert!(
        !due.deliveries.contains(&delivery),
        "a turn that gave up is still due"
    );
    running.database.announce(&delivery).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let later = running
        .store
        .read(&scope)
        .await
        .expect("read")
        .expect("conversation");
    assert_eq!(later.status, WorkStatus::Failed);
    assert_eq!(later.attempts.len(), 5);
}

/// AC3. Hosted pacing: each failure makes the turn due later than the last,
/// 10 s then 20 s, and the worker does not run it before then.
#[tokio::test]
async fn failed_attempts_wait_longer_each_time() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("backoff").await else {
        return;
    };
    let store = Store::connect(&database.url).await.expect("runtime store");
    let home = std::env::temp_dir().join(format!(
        "physician-worker-backoff-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("home");
    let stub = support::ResponsesStub::start(&home, 2, 0, "[]").await;
    let port = support::free_port();
    let listen = format!("127.0.0.1:{port}");
    // No retry setting: this worker waits the hosted 10 s, 20 s, 40 s.
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
        ],
        &home,
    );
    support::wait_ready(&mut worker, port).await;
    let (scope, _) = accept_turn(&database.url, &store).await;
    let started = Instant::now();
    let mut seen = vec![];
    let deadline = Instant::now() + Duration::from_secs(60);
    let state = loop {
        let state = store.read(&scope).await.expect("read").expect("row");
        while seen.len() < state.attempts.len() {
            seen.push(started.elapsed());
        }
        if state.status == WorkStatus::Completed {
            break state;
        }
        assert!(
            Instant::now() < deadline,
            "the turn never completed: {:?}\n{}",
            attempt_statuses(&state),
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    assert_eq!(attempt_statuses(&state), ["failed", "failed", "completed"]);
    let first_wait = seen[1] - seen[0];
    let second_wait = seen[2] - seen[1];
    println!("attempts began at {seen:?}: waits of {first_wait:?} then {second_wait:?}");
    // The database clock counts whole seconds; a poll may add its interval.
    for (wait, seconds) in [(first_wait, 10), (second_wait, 20)] {
        assert!(
            wait >= Duration::from_secs(seconds - 1),
            "retried after {wait:?}, before its {seconds} s delay"
        );
        assert!(
            wait < Duration::from_secs(seconds + 11),
            "retried after {wait:?}, long after its {seconds} s delay"
        );
    }
    worker.kill_now();
    let _ = std::fs::remove_dir_all(&home);
}

#[tokio::test]
async fn concurrent_turns_on_one_worker_run_side_by_side() {
    let _guard = lock().lock().await;
    let delay = Duration::from_secs(2);
    let Some(running) = running("parallel", None, Some(delay.as_millis() as u64)).await else {
        return;
    };
    let started = Instant::now();
    let (first_scope, _) = accept_turn(&running.database.url, &running.store).await;
    let (second_scope, _) = accept_turn(&running.database.url, &running.store).await;
    for scope in [&first_scope, &second_scope] {
        let state = wait_assistant(&running.store, scope, &running.worker).await;
        assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    }
    let elapsed = started.elapsed();
    assert!(
        elapsed >= delay,
        "both turns answered after {elapsed:?}, before a {delay:?} turn could finish"
    );
    assert!(
        elapsed < delay * 2 - Duration::from_millis(500),
        "two {delay:?} turns took {elapsed:?}; the worker ran them one after the other"
    );
}

/// A worker with one slot, kept full by chat turns that arrive faster than it
/// answers them, still reaches a due reply revision: its sweeps alternate which
/// kind they read first, so the revision waits for at most the turn running
/// when it was accepted and one more.
#[tokio::test]
async fn a_due_revision_is_claimed_while_chat_turns_keep_arriving() {
    use local_product::runtime::reply::{DraftStatus, RevisionAccepted, RevisionInput};
    let _guard = lock().lock().await;
    let turn = Duration::from_millis(1000);
    let Some(running) = running_with(
        "revision_turn",
        None,
        Some(turn.as_millis() as u64),
        "[]",
        support::FIXED_TEXT,
        &[
            ("PHYSICIAN_WORKER_CONCURRENCY", "1"),
            ("PHYSICIAN_WORKER_POLL_MS", "200"),
        ],
    )
    .await
    else {
        return;
    };
    // Chat turns arrive twice as fast as the worker answers them.
    let turns = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::<Scope>::new()));
    let feeder = tokio::spawn({
        let (url, store, turns) = (
            running.database.url.clone(),
            running.store.clone(),
            turns.clone(),
        );
        async move {
            loop {
                let (scope, _) = accept_turn(&url, &store).await;
                turns.lock().await.push(scope);
                tokio::time::sleep(turn / 2).await;
            }
        }
    });
    let answered = |store: Store, turns: std::sync::Arc<tokio::sync::Mutex<Vec<Scope>>>| async move {
        let scopes = turns.lock().await.clone();
        let mut count = 0;
        for scope in &scopes {
            if store
                .read(scope)
                .await
                .expect("read conversation")
                .is_some_and(|state| state.status == WorkStatus::Completed)
            {
                count += 1;
            }
        }
        (count, scopes.len())
    };
    // Wait until the slot is taken and chat turns are queued behind it.
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let (done, accepted) = answered(running.store.clone(), turns.clone()).await;
        if accepted >= done + 3 {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "chat turns never queued\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let (scope, session) = signed_in(&running.database.url, None).await;
    let hash = PostgresAuthStore::session_reference(&session);
    running
        .store
        .open_draft_authorized(
            &scope,
            "Your results look stable.",
            &hash,
            &mut physician_sample::durable::RuntimeAuthority,
        )
        .await
        .expect("open draft")
        .expect("authorized draft");
    let skills = PhysicianSkills::configured(None, false).unwrap();
    let (before, _) = answered(running.store.clone(), turns.clone()).await;
    let accepted = running
        .store
        .accept_revision(
            &scope,
            &RevisionInput {
                key: "revise-under-load".into(),
                expected_revision: 1,
                instruction: "Make it shorter.".into(),
                timezone: "America/New_York".into(),
                scenario: None,
            },
            hash,
            &skills,
            &local_product::runtime::reply::recipe(&support::models::roles().background, &skills),
            None,
        )
        .await
        .expect("accept revision");
    assert_eq!(accepted, RevisionAccepted::New);

    let deadline = Instant::now() + Duration::from_secs(30);
    let draft = loop {
        let draft = running
            .store
            .draft(&scope)
            .await
            .expect("draft")
            .expect("draft row");
        if draft.status == DraftStatus::Completed {
            break draft;
        }
        assert!(
            Instant::now() < deadline,
            "the revision never ran while chat turns kept arriving: {:?}\n{}",
            draft.status,
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    let (after, accepted) = answered(running.store.clone(), turns.clone()).await;
    feeder.abort();
    assert_eq!(draft.revision, 2);
    assert!(
        accepted > after,
        "chat turns were still waiting when the revision ran ({after} of {accepted} answered)"
    );
    assert!(
        after - before <= 2,
        "{} chat turns were answered before the due revision",
        after - before
    );
}

/// Copies the workspace catalog to a directory outside the source checkout,
/// as the admitting site sees it.
fn copied_skills(home: &std::path::Path) -> std::path::PathBuf {
    fn copy(from: &std::path::Path, to: &std::path::Path) {
        std::fs::create_dir_all(to).expect("skills directory");
        for entry in std::fs::read_dir(from).expect("read skills") {
            let entry = entry.expect("skills entry");
            let target = to.join(entry.file_name());
            if entry.file_type().expect("file type").is_dir() {
                copy(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), target).expect("copy skill file");
            }
        }
    }
    let target = home.join("packaged-skills");
    copy(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../skills"),
        &target,
    );
    target
}

/// Each saved receipt: its original output, or the reason of its refusal.
fn tool_returned(state: &Conversation) -> Vec<(String, String, bool)> {
    receipts(state.attempts.iter().flat_map(|attempt| &attempt.events))
}

/// Each receipt among `events`: its original output, or the reason of its
/// refusal.
fn receipts<'a>(
    events: impl Iterator<Item = &'a harness::engine::Event>,
) -> Vec<(String, String, bool)> {
    use harness::engine::results::ToolResult;
    events
        .filter_map(|event| match &event.kind {
            harness::engine::EventKind::ToolReturned {
                call_id,
                result: ToolResult::Original(output),
            } => Some((call_id.clone(), output.clone(), true)),
            harness::engine::EventKind::ToolReturned {
                call_id,
                result: ToolResult::Refused(refusal),
            } => Some((call_id.clone(), refusal.reason().to_owned(), false)),
            _ => None,
        })
        .collect()
}

/// The real worker binary, its host and Postgres, with a scripted provider:
/// round one reads a skill body next to malformed, unknown and unavailable calls; round two reads a supporting resource; then it answers.
/// Each round's results must reach the next request or the fixed provider
/// fails the attempt. Only the final text is published; every exact result
/// stays in the saved attempt.
#[tokio::test]
async fn skill_reads_run_through_the_worker_and_only_the_answer_is_published() {
    let _guard = lock().lock().await;
    let staging = std::env::temp_dir().join(format!(
        "physician-worker-skills-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    let skills = copied_skills(&staging);
    let resource = skills.join("patient-record-retrieval/references/authoring-guide.md");
    std::fs::create_dir_all(resource.parent().unwrap()).unwrap();
    std::fs::write(
        &resource,
        "Only the admitting site saw these guidance bytes.",
    )
    .unwrap();
    let admitted = PhysicianSkills::load(&skills).unwrap();
    let catalog: harness::skills::Package =
        serde_json::from_value(serde_json::to_value(&admitted).unwrap()).unwrap();
    let content = r#"{"code":"2160-0","value":0.9,"unit":"mg/dL"}"#;
    let revision = local_product::sha256_hex(content.as_bytes());
    let rounds = serde_json::json!([
        [
            {"name": "skill_view", "arguments": {"name": "patient-record-retrieval", "file_path": null}},
            {"name": "skill_view", "arguments": {"name": 0}},
            {"name": "skill_view", "arguments": "not json"},
            {"name": "write_patient_memory", "arguments": {}},
            {"name": "skill_view", "arguments": {"name": "not-a-skill", "file_path": null}},
            {"name": "skill_view", "arguments": {"name": "patient-record-retrieval", "file_path": "references/absent.md"}},
            {"name": "search_patient_information", "arguments": {"query":"2160-0","record_type":"Observation","after":null}}
        ],
        [
            {"name": "skill_view", "arguments": {"name": "patient-record-retrieval", "file_path": "references/authoring-guide.md"}},
            {"name":"read_patient_sources","arguments":{"references":[{"candidate_id":"Observation/result","revision":revision,"cursor":null}],"max_result_bytes":4096}}
        ]
    ])
    .to_string();
    let Some(mut running) =
        running_with("skills", None, None, &rounds, support::FIXED_TEXT, &[]).await
    else {
        let _ = std::fs::remove_dir_all(&staging);
        return;
    };
    // The turn is accepted while no worker runs, so its source is saved
    // before any attempt can search for it.
    running.worker.kill_now();
    let (scope, _) = accept_as(
        &running.database.url,
        &running.store,
        "Hello",
        &admitted,
        None,
        None,
        None,
    )
    .await;
    std::fs::remove_dir_all(&skills).unwrap();
    let record = local_product::physician_sources::SourceRevision {
        workspace: scope.workspace.clone(),
        source_id: "synthetic-charts".into(),
        candidate_id: "Observation/result".into(),
        revision: revision.clone(),
        record_type: "Observation".into(),
        dates: vec![],
        content: content.into(),
        sha256: revision.clone(),
        readable_parts: vec![],
        provenance: serde_json::json!({"native_record":"Observation/result"}),
        envelope: None,
    };
    running.store.record_source(&record).await.unwrap();
    running.worker.restart();
    let port = running.port();
    support::wait_ready(&mut running.worker, port).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(attempt_statuses(&state), ["completed"]);
    assert_eq!(
        assistant_texts(&state),
        [support::FIXED_TEXT],
        "tool-round narration must stay internal"
    );

    // Durable readback through a fresh connection.
    let store = Store::connect(&running.database.url)
        .await
        .expect("fresh store");
    let saved = store
        .read(&scope)
        .await
        .expect("read")
        .expect("conversation");
    let recipe = store
        .frozen_binding(&scope, saved.bindings.get(&saved.work).unwrap())
        .await
        .unwrap();
    let requests: Vec<_> = saved.attempts[0]
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            harness::engine::EventKind::ProviderStarted {
                request: context, ..
            } => Some(
                recipe.request(
                    context
                        .restore(&saved.attempts[0].events, &saved.message_sources())
                        .unwrap()
                        .expect("saved request sources remain available"),
                ),
            ),
            _ => None,
        })
        .collect();
    assert_eq!(requests.len(), 3, "two tool rounds and one answer");
    let offered = local_product::physician_chat::recipe(
        &support::models::roles().chat,
        local_product::physician_chat::INSTRUCTIONS,
        &admitted,
    )
    .tools;
    for context in &requests {
        assert!(
            context
                .instructions
                .contains(local_product::physician_chat::INSTRUCTIONS)
        );
        assert!(
            context
                .instructions
                .contains(&catalog.catalog_instructions())
        );
        assert_eq!(context.tools, offered);
    }
    let returned = tool_returned(&saved);
    assert_eq!(returned.len(), 9);
    let output = |id: &str| {
        returned
            .iter()
            .find(|(call, ..)| call == id)
            .unwrap_or_else(|| panic!("no result for {id}"))
    };
    let found: serde_json::Value = serde_json::from_str(&output("fixed-1-6").1).unwrap();
    assert_eq!(found["results"][0]["reference"]["revision"], revision);
    assert!(!output("fixed-1-6").1.contains("mg/dL"));
    let opened: serde_json::Value = serde_json::from_str(&output("fixed-2-1").1).unwrap();
    assert_eq!(opened["results"][0]["content"], content);
    let exact = |name: &str, path: Option<&str>| {
        catalog
            .execute_skill_view(&serde_json::json!({"name":name,"file_path":path}))
            .unwrap()
            .to_json()
    };
    assert_eq!(
        output("fixed-1-0"),
        &(
            "fixed-1-0".into(),
            exact("patient-record-retrieval", None),
            true
        )
    );
    assert_eq!(output("fixed-1-4").1, exact("not-a-skill", None));
    assert_eq!(
        output("fixed-1-5").1,
        exact("patient-record-retrieval", Some("references/absent.md"))
    );
    assert!(output("fixed-1-5").1.contains("\"unavailable\""));
    assert_eq!(
        output("fixed-2-0").1,
        exact(
            "patient-record-retrieval",
            Some("references/authoring-guide.md")
        )
    );
    for refused in ["fixed-1-1", "fixed-1-2", "fixed-1-3"] {
        let (_, _, executed) = output(refused);
        assert!(!executed, "{refused}");
    }
    drop(running);
    let _ = std::fs::remove_dir_all(&staging);
}

/// The real worker binary, its host and Postgres, with a scripted provider:
/// round one adds a note quoting the physician's message, adds a note whose
/// quote the message does not hold and whose chart record the chart does not
/// hold, and lists memory; round two searches for a word of the first note
/// and reads the patient's summary. The notes are saved under the run's scope
/// with the work and call as their identity, each keeps its quote as given,
/// and each saved result says what the store holds. A second chat about the
/// patient starts with those notes.
#[tokio::test]
async fn memory_tools_run_through_the_worker_and_notes_keep_their_quotes() {
    use local_product::runtime::memory::{MemoryPath, OneLine, PatientKey};
    let _guard = lock().lock().await;
    let message = "Please stop the lisinopril and call her Friday.";
    let summary = "Patient with stage 3 chronic kidney disease.";
    let kept = serde_json::json!({"text": "Physician stopped lisinopril for the cough.", "quote": "stop the lisinopril"});
    let reworded = serde_json::json!({"text": "Recheck potassium in May.", "quote": "recheck potassium in May"});
    let rounds = serde_json::json!([
        [
            {"name": "add_patient_memory_note", "arguments": kept},
            {"name": "add_patient_memory_note", "arguments": reworded},
            {"name": "list_patient_memory", "arguments": {}}
        ],
        [
            {"name": "search_patient_memory", "arguments": {"query": "lisinopril"}},
            {"name": "read_patient_memory", "arguments": {"path": "summary.md", "version": null}}
        ]
    ])
    .to_string();
    let Some(running) = running_with("memory", None, None, &rounds, support::FIXED_TEXT, &[]).await
    else {
        return;
    };
    let skills = PhysicianSkills::configured(None, false).unwrap();
    let (scope, session) = signed_in(&running.database.url, None).await;
    let key = PatientKey::try_from(&scope).unwrap();
    running
        .store
        .edit_memory(
            &key,
            &local_product::runtime::memory::VersionedFile::Summary,
            &[Ok(local_product::runtime::memory::Edit::Create {
                text: local_product::runtime::memory::Words::parse(summary).unwrap(),
            })],
        )
        .await
        .unwrap();
    // The chat starts in a later clock second than the summary's save.
    tokio::time::sleep(Duration::from_secs(1)).await;
    accept_signed_in(
        &running.store,
        &scope,
        &session,
        message,
        &skills,
        None,
        None,
    )
    .await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);

    // Durable readback through a fresh connection.
    let store = Store::connect(&running.database.url)
        .await
        .expect("fresh store");
    let saved = store
        .read(&scope)
        .await
        .expect("read")
        .expect("conversation");
    let recipe = store
        .frozen_binding(&scope, saved.bindings.get(&saved.work).unwrap())
        .await
        .unwrap();
    let requests: Vec<_> = saved.attempts[0]
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            harness::engine::EventKind::ProviderStarted {
                request: context, ..
            } => Some(
                recipe.request(
                    context
                        .restore(&saved.attempts[0].events, &saved.message_sources())
                        .unwrap()
                        .expect("saved request sources remain available"),
                ),
            ),
            _ => None,
        })
        .collect();
    assert_eq!(requests.len(), 3, "two tool rounds and one answer");
    let offered = local_product::physician_chat::recipe(
        &support::models::roles().chat,
        local_product::physician_chat::INSTRUCTIONS,
        &PhysicianSkills::configured(None, false).unwrap(),
    )
    .tools;
    for name in [
        "list_patient_memory",
        "read_patient_memory",
        "search_patient_memory",
        "add_patient_memory_note",
        "edit_patient_memory",
    ] {
        assert!(offered.iter().any(|tool| tool["name"] == name), "{name}");
    }
    for context in &requests {
        assert_eq!(context.tools, offered);
    }

    // The notes the worker saved, each by the run's work and its call; the
    // readback adds nothing.
    let physician_message = saved.messages[0].source();
    let note = |call: &'static str| {
        let (store, key, work) = (store.clone(), key.clone(), saved.work.clone());
        async move {
            store
                .saved_note(&key, &work, call)
                .await
                .unwrap()
                .unwrap_or_else(|| panic!("the worker saved the note of {call}"))
        }
    };
    let first = note("fixed-1-0").await;
    let second = note("fixed-1-1").await;
    assert_eq!(first.text, kept["text"].as_str().unwrap());
    assert_eq!(second.text, reworded["text"].as_str().unwrap());
    for saved_note in [&first, &second] {
        assert_eq!(saved_note.physician, scope.physician);
        assert_eq!(saved_note.chat, scope.chat);
        assert_eq!(saved_note.message, physician_message.id);
    }
    assert_eq!(
        first.physician_words.as_ref().map(OneLine::as_str),
        Some("stop the lisinopril")
    );
    assert_eq!(
        second.physician_words.as_ref().map(OneLine::as_str),
        Some("recheck potassium in May")
    );
    let day = first.time.format("%Y-%m-%d").to_string();
    let log = format!("log/{day}.md");
    let listing = store.list_memory(&key).await.unwrap();
    assert_eq!(
        listing
            .log_days
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        [day]
    );
    let path = MemoryPath::parse(&log).unwrap();
    let markdown = match store.read_memory(&key, &path, None).await.unwrap() {
        local_product::runtime::memory::FileRead::Log { markdown } => markdown,
        other => panic!("a log day has no versions: {other:?}"),
    };
    for number in [&first.number, &second.number] {
        assert_eq!(
            markdown
                .matches(&format!("note {number}, physician {}\n", scope.physician))
                .count(),
            1,
            "{markdown}"
        );
    }
    assert_eq!(markdown.matches("Quoted:").count(), 2, "{markdown}");

    // Each saved result says what the store holds.
    let returned = tool_returned(&saved);
    assert_eq!(returned.len(), 5);
    let output = |id: &str| -> serde_json::Value {
        let (_, output, executed) = returned
            .iter()
            .find(|(call, ..)| call == id)
            .unwrap_or_else(|| panic!("no result for {id}"));
        assert!(executed, "{id} was refused: {output}");
        serde_json::from_str(output).unwrap()
    };
    assert_eq!(
        output("fixed-1-0"),
        serde_json::json!({"note": first.number.to_string(), "path": log})
    );
    assert_eq!(
        output("fixed-1-1"),
        serde_json::json!({"note": second.number.to_string(), "path": log})
    );
    assert_eq!(
        output("fixed-1-2"),
        serde_json::json!({"summary": {"path": "summary.md", "version": 1, "characters": summary.chars().count()}, "direction": null, "person": null, "pages": [], "log": [log]})
    );
    let hits: Vec<_> = store
        .search_memory(&key, "lisinopril")
        .await
        .unwrap()
        .into_iter()
        .map(|hit| serde_json::json!({"path": hit.path.to_string(), "line": hit.line, "text": hit.text}))
        .collect();
    assert_eq!(hits.len(), 2, "the note's words and its text: {hits:?}");
    assert_eq!(output("fixed-2-0"), serde_json::json!({"results": hits}));
    assert_eq!(
        output("fixed-2-1"),
        serde_json::json!({"path": "summary.md", "version": 1, "markdown": summary})
    );

    // The run started with the summary and no note. A second chat about the
    // same patient, started in a later clock second, opens with the summary
    // and both notes the first run added.
    let notes = [
        kept["text"].as_str().unwrap(),
        reworded["text"].as_str().unwrap(),
    ];
    let sent = running.stub.requests();
    assert!(memory_place(&sent[0], summary).is_some(), "{}", sent[0]);
    for words in notes {
        assert_eq!(memory_place(&sent[0], words), None, "{words}");
    }
    running.stub.tool_rounds(&serde_json::json!([]));
    tokio::time::sleep(Duration::from_secs(1)).await;
    let next = Scope {
        chat: format!("{}-next", scope.chat),
        ..scope.clone()
    };
    let asked = "What did I decide about her lisinopril?";
    accept_signed_in(&running.store, &next, &session, asked, &skills, None, None).await;
    let answered = wait_assistant(&running.store, &next, &running.worker).await;
    assert_eq!(answered.status, WorkStatus::Completed);
    let opening = running
        .stub
        .requests()
        .into_iter()
        .find(|request| input_place(request, asked).is_some())
        .expect("the second chat's request");
    for words in [summary].into_iter().chain(notes) {
        assert_eq!(memory_place(&opening, words), Some(0), "{words}: {opening}");
    }
}

/// The real worker binary, its host and Postgres, with a scripted provider.
/// The patient has a summary when the chat starts. The first message's run
/// adds a note; the second message's run saves a working note and opens a
/// fresh window. Every request of the chat, the fresh window's included,
/// opens with the same memory: the summary, without the note.
#[tokio::test]
async fn every_request_of_a_chat_opens_with_the_memory_it_started_with() {
    use local_product::runtime::memory::PatientKey;
    let _guard = lock().lock().await;
    let summary = "Patient with stage 3 chronic kidney disease.";
    let added = "Physician stopped lisinopril for the cough.";
    let working = "Continue from the patient's memory. Do not send anything.";
    let rounds = serde_json::json!([[
        {"name": "add_patient_memory_note", "arguments": {"text": added, "quote": null}}
    ]])
    .to_string();
    let Some(running) =
        running_with("memory_once", None, None, &rounds, support::FIXED_TEXT, &[]).await
    else {
        return;
    };
    let skills = PhysicianSkills::configured(None, false).unwrap();
    let (scope, session) = signed_in(&running.database.url, None).await;
    let key = PatientKey::try_from(&scope).unwrap();
    running
        .store
        .edit_memory(
            &key,
            &local_product::runtime::memory::VersionedFile::Summary,
            &[Ok(local_product::runtime::memory::Edit::Create {
                text: local_product::runtime::memory::Words::parse(summary).unwrap(),
            })],
        )
        .await
        .unwrap();
    // The chat starts in a later clock second than the summary's save.
    tokio::time::sleep(Duration::from_secs(1)).await;
    let first = "Please stop the lisinopril.";
    accept_signed_in(&running.store, &scope, &session, first, &skills, None, None).await;
    wait_assistant(&running.store, &scope, &running.worker).await;
    assert!(
        running
            .store
            .search_memory(&key, "lisinopril")
            .await
            .unwrap()
            .iter()
            .any(|hit| hit.text.contains(added)),
        "the first run saved its note"
    );

    running.stub.tool_rounds(&serde_json::json!([[
        {"name": "save_working_note", "arguments": {"text": working, "fresh_window": true}}
    ]]));
    let second = "What did I decide?";
    accept_signed_in(
        &running.store,
        &scope,
        &session,
        second,
        &skills,
        None,
        None,
    )
    .await;
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let state = running.store.conversation(&scope).await.unwrap();
        if assistant_texts(&state).len() == 2 {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the second message was not answered: {state:?}\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let sent = running.stub.requests();
    let second_requests: Vec<_> = sent
        .iter()
        .filter(|request| input_place(request, second).is_some())
        .collect();
    assert!(
        second_requests.iter().any(|request| {
            request["input"].as_array().unwrap().iter().any(|item| {
                item["content"]
                    .as_str()
                    .is_some_and(|text| text.contains(working))
            })
        }),
        "the second run opened a fresh window: {second_requests:?}"
    );
    let opening = &sent[0]["input"][0];
    assert_eq!(memory_place(&sent[0], summary), Some(0), "{}", sent[0]);
    for request in &sent {
        assert_eq!(&request["input"][0], opening, "{request}");
        assert_eq!(memory_place(request, added), None, "{request}");
    }
    assert!(second_requests.len() >= 2, "{second_requests:?}");
}

/// Where in a request the provider received an item whose text is `text`.
fn input_place(request: &serde_json::Value, text: &str) -> Option<usize> {
    request["input"]
        .as_array()?
        .iter()
        .position(|item| item["content"] == text)
}

/// Where in a request the provider received a developer message that holds
/// `words`.
fn memory_place(request: &serde_json::Value, words: &str) -> Option<usize> {
    request["input"].as_array()?.iter().position(|item| {
        item["role"] == "developer"
            && item["content"]
                .as_str()
                .is_some_and(|text| text.contains(words))
    })
}

/// The real worker binary, its host and Postgres, with a provider whose
/// first response breaks, so the first attempt fails and the next one
/// recovers the run. A note added between the two attempts is not in what
/// the recovered run sends: it sends the memory the first attempt sent.
#[tokio::test]
async fn a_recovered_run_sends_the_memory_it_first_sent() {
    use local_product::runtime::memory::{NoteDraft, NoteOrigin, PatientKey};
    let _guard = lock().lock().await;
    let message = "Please stop the lisinopril.";
    let summary = "Patient with stage 3 chronic kidney disease.";
    let between = "Added between the two attempts.";
    let Some(running) = running_with(
        "memory_recovery",
        Some(1),
        None,
        "[]",
        support::FIXED_TEXT,
        &[("PHYSICIAN_WORKER_RETRY_SECONDS", "3")],
    )
    .await
    else {
        return;
    };
    let (scope, session) = signed_in(&running.database.url, None).await;
    let key = PatientKey::try_from(&scope).unwrap();
    running
        .store
        .edit_memory(
            &key,
            &local_product::runtime::memory::VersionedFile::Summary,
            &[Ok(local_product::runtime::memory::Edit::Create {
                text: local_product::runtime::memory::Words::parse(summary).unwrap(),
            })],
        )
        .await
        .unwrap();
    // The chat starts in a later clock second than the summary's save.
    tokio::time::sleep(Duration::from_secs(1)).await;
    let skills = PhysicianSkills::configured(None, false).unwrap();
    accept_signed_in(
        &running.store,
        &scope,
        &session,
        message,
        &skills,
        None,
        None,
    )
    .await;
    // The first attempt fails; its retry waits three seconds.
    let deadline = Instant::now() + Duration::from_secs(30);
    let failed = loop {
        let state = running.store.conversation(&scope).await.unwrap();
        if attempt_statuses(&state) == ["failed"] {
            break state;
        }
        assert!(
            Instant::now() < deadline,
            "the first attempt did not fail: {state:?}\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    };
    let physician_message = failed.messages[0].source();
    running
        .store
        .append_note(
            &scope,
            &NoteOrigin {
                work: failed.work.clone(),
                call: "between".into(),
                message: physician_message.id,
            },
            &NoteDraft {
                text: between.into(),
                quote: None,
            },
        )
        .await
        .unwrap();
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(attempt_statuses(&state), ["failed", "completed"]);

    let sent = running.stub.requests();
    assert_eq!(sent.len(), 2, "one request for each attempt");
    let memory: Vec<&serde_json::Value> = sent
        .iter()
        .map(|request| {
            let place = memory_place(request, summary).expect("the memory message");
            &request["input"][place]
        })
        .collect();
    assert_eq!(memory[0], memory[1]);
    assert!(
        !memory[1]["content"].as_str().unwrap().contains(between),
        "{}",
        memory[1]
    );
}

/// The real worker binary, its host and Postgres, with a scripted provider
/// running a reply revision: round one adds a note quoting the physician's
/// instruction and a note whose quote the instruction does not hold; round
/// two searches memory for a word of both notes and reads the summary; then
/// the provider finishes the revision. Each note's work is the accepted
/// revision, its message is the instruction, and it keeps its quote as
/// given.
#[tokio::test]
async fn a_reply_revision_reads_memory_and_its_notes_come_from_the_instruction() {
    use local_product::runtime::memory::{FileRead, MemoryPath, OneLine, PatientKey};
    use local_product::runtime::reply::{DraftStatus, RevisionAccepted, RevisionInput};
    let _guard = lock().lock().await;
    let instruction = "Drop the statin line, she stopped it in March.";
    let summary = "Patient with hyperlipidemia.";
    let Some(running) = running_with(
        "revision_memory",
        None,
        None,
        "[]",
        support::FIXED_TEXT,
        &[],
    )
    .await
    else {
        return;
    };
    let (scope, session) = signed_in(&running.database.url, None).await;
    let kept = serde_json::json!({"text": "Patient stopped her statin in March.", "quote": "she stopped it in March"});
    let unquoted = serde_json::json!({"text": "Statin stopped for muscle aches.", "quote": "stopped it for muscle aches"});
    running.stub.tool_rounds(&serde_json::json!([
        [
            {"name": "add_patient_memory_note", "arguments": kept},
            {"name": "add_patient_memory_note", "arguments": unquoted}
        ],
        [
            {"name": "search_patient_memory", "arguments": {"query": "statin"}},
            {"name": "read_patient_memory", "arguments": {"path": "summary.md", "version": null}}
        ]
    ]));
    let hash = PostgresAuthStore::session_reference(&session);
    running
        .store
        .open_draft_authorized(
            &scope,
            "Keep taking your statin every night.",
            &hash,
            &mut physician_sample::durable::RuntimeAuthority,
        )
        .await
        .expect("open draft")
        .expect("authorized draft");
    // The summary is saved in an earlier clock second than the revision's
    // acceptance, so the reply work starts with it.
    let key = PatientKey::try_from(&scope).unwrap();
    running
        .store
        .edit_memory(
            &key,
            &local_product::runtime::memory::VersionedFile::Summary,
            &[Ok(local_product::runtime::memory::Edit::Create {
                text: local_product::runtime::memory::Words::parse(summary).unwrap(),
            })],
        )
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;
    let skills = PhysicianSkills::configured(None, false).unwrap();
    let input = RevisionInput {
        key: format!("revise-{}", uuid::Uuid::new_v4().simple()),
        expected_revision: 1,
        instruction: instruction.into(),
        timezone: "America/New_York".into(),
        scenario: None,
    };
    let accepted = running
        .store
        .accept_revision(
            &scope,
            &input,
            hash,
            &skills,
            &local_product::runtime::reply::recipe(&support::models::roles().background, &skills),
            None,
        )
        .await
        .expect("accept revision");
    assert_eq!(accepted, RevisionAccepted::New);
    let deadline = Instant::now() + Duration::from_secs(30);
    let draft = loop {
        let draft = running
            .store
            .draft(&scope)
            .await
            .expect("draft")
            .expect("draft row");
        if draft.status == DraftStatus::Completed {
            break draft;
        }
        assert!(
            Instant::now() < deadline,
            "the revision did not finish: {:?}\n{}",
            draft.status,
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };

    // The revision still publishes its reply.
    assert_eq!(draft.revision, 2);
    assert_eq!(draft.text, support::FIXED_TEXT);

    // The notes the worker saved, each by the revision's work and its call;
    // the readback adds nothing. The note's work is the accepted revision,
    // and its one message is the instruction.
    let store = Store::connect(&running.database.url)
        .await
        .expect("fresh store");
    let note = |call: &'static str| {
        let (store, key, work) = (store.clone(), key.clone(), input.key.clone());
        async move {
            store
                .saved_note(&key, &work, call)
                .await
                .unwrap()
                .unwrap_or_else(|| panic!("the worker saved the note of {call}"))
        }
    };
    let first = note("fixed-1-0").await;
    let second = note("fixed-1-1").await;
    assert_eq!(first.text, kept["text"].as_str().unwrap());
    assert_eq!(second.text, unquoted["text"].as_str().unwrap());
    for saved_note in [&first, &second] {
        assert_eq!(saved_note.physician, scope.physician);
        assert_eq!(saved_note.chat, scope.chat);
        assert_eq!(saved_note.message, format!("{}:instruction", input.key));
    }
    assert_eq!(
        first.physician_words.as_ref().map(OneLine::as_str),
        Some("she stopped it in March")
    );
    assert_eq!(
        second.physician_words.as_ref().map(OneLine::as_str),
        Some("stopped it for muscle aches")
    );
    let log = format!("log/{}.md", first.time.format("%Y-%m-%d"));
    let markdown = match store
        .read_memory(&key, &MemoryPath::parse(&log).unwrap(), None)
        .await
        .unwrap()
    {
        FileRead::Log { markdown } => markdown,
        other => panic!("a log day has no versions: {other:?}"),
    };
    assert_eq!(markdown.matches(", note ").count(), 2, "{markdown}");
    assert_eq!(markdown.matches("Quoted:").count(), 2, "{markdown}");

    // Each saved result says what the store holds.
    let returned = receipts(draft.attempts.iter().flat_map(|attempt| &attempt.events));
    let output = |id: &str| -> serde_json::Value {
        let (_, output, executed) = returned
            .iter()
            .find(|(call, ..)| call == id)
            .unwrap_or_else(|| panic!("no result for {id}"));
        assert!(executed, "{id} was refused: {output}");
        serde_json::from_str(output).unwrap()
    };
    assert_eq!(
        output("fixed-1-0"),
        serde_json::json!({"note": first.number.to_string(), "path": log})
    );
    assert_eq!(
        output("fixed-1-1"),
        serde_json::json!({"note": second.number.to_string(), "path": log})
    );
    let hits: Vec<_> = store
        .search_memory(&key, "statin")
        .await
        .unwrap()
        .into_iter()
        .map(|hit| serde_json::json!({"path": hit.path.to_string(), "line": hit.line, "text": hit.text}))
        .collect();
    assert_eq!(hits.len(), 2, "the text of each note: {hits:?}");
    assert_eq!(output("fixed-2-0"), serde_json::json!({"results": hits}));
    assert_eq!(
        output("fixed-2-1"),
        serde_json::json!({"path": "summary.md", "version": 1, "markdown": summary})
    );

    // The revision started with the summary and no note.
    let sent = running.stub.requests();
    assert_eq!(memory_place(&sent[0], summary), Some(0), "{}", sent[0]);
    for words in [
        kept["text"].as_str().unwrap(),
        unquoted["text"].as_str().unwrap(),
    ] {
        assert_eq!(memory_place(&sent[0], words), None, "{words}");
    }
}

#[tokio::test]
async fn hosting_cancels_a_chat_whose_claim_has_been_replaced() {
    use harness::engine::{AttemptFailure, EventKind};
    let _guard = lock().lock().await;
    let Some(running) = running_with(
        "claim_replaced",
        None,
        Some(200_000),
        "[]",
        support::FIXED_TEXT,
        &[("PHYSICIAN_WORKER_CONCURRENCY", "1")],
    )
    .await
    else {
        return;
    };
    let (scope, _) = accept_turn(&running.database.url, &running.store).await;
    let started = Instant::now();
    let abandoned = loop {
        let state = running.store.conversation(&scope).await.unwrap();
        if let Some(attempt) = state.attempts.first()
            && attempt
                .events
                .iter()
                .any(|event| matches!(event.kind, EventKind::ProviderStarted { .. }))
        {
            break local_product::runtime::Claim {
                scope: scope.clone(),
                attempt: attempt.id.clone(),
                revision: attempt.revision,
                lease: local_product::runtime::Lease::new(540).unwrap(),
                origin: None,
            };
        }
        assert!(started.elapsed() < Duration::from_secs(10));
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    running
        .store
        .settle_chat_attempt(&abandoned, AttemptFailure::Interrupted, Backoff::IMMEDIATE)
        .await
        .unwrap();
    let successor = running.store.claim(&scope, 540).await.unwrap().unwrap();
    assert_ne!(successor.attempt, abandoned.attempt);
    // The worker runs one attempt at a time, so another physician's turn
    // starts only once hosting has cancelled the replaced run.
    let (other, _) = accept_text(&running.database.url, &running.store, "Next").await;
    let deadline = Instant::now() + Duration::from_secs(115);
    loop {
        let state = running.store.conversation(&other).await.unwrap();
        if state.attempts.first().is_some_and(|attempt| {
            attempt
                .events
                .iter()
                .any(|event| matches!(event.kind, EventKind::ProviderStarted { .. }))
        }) {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "hosting must cancel the run whose claim was replaced"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let saved = running.store.conversation(&scope).await.unwrap();
    assert_eq!(saved.attempts.len(), 2);
    assert_eq!(saved.attempts[1].id, successor.attempt);
    assert_eq!(saved.attempts[1].status, AttemptStatus::Active);
    assert!(assistant_texts(&saved).is_empty());
    assert_eq!(saved.attempts[0].status, AttemptStatus::Interrupted);
    println!(
        "Replaced hosted claim stopped after {} seconds; successor remains current",
        started.elapsed().as_secs()
    );
}

/// The scripted provider hands in a finish that cites what the worker read:
/// the saved answer has no label, and its mark names the version and the
/// quoted bytes.
#[tokio::test]
async fn a_scripted_finish_cites_the_record_the_worker_read() {
    use local_product::physician_sources::SourcePatient;
    use local_product::physician_sources::adapter::file_fhir_record;
    use local_product::runtime::sources::SourcePull;

    let _guard = lock().lock().await;
    let content = r#"{"resourceType":"Observation","id":"creatinine","status":"final","code":{"text":"Creatinine"},"valueQuantity":{"value":0.9,"unit":"mg/dL"},"effectiveDateTime":"2026-08-01"}"#;
    let quote = r#""value":0.9"#;
    let finish = serde_json::json!({
        "answer": "Creatinine was 0.9 mg/dL [S1].",
        "quotes": [{"label": "S1", "quote": quote}],
        "descriptions": [],
        "basis": "The creatinine result [S1].",
        "cites_none": false
    })
    .to_string();
    let Some(mut running) = running_with("finish", None, None, "[]", &finish, &[]).await else {
        return;
    };
    // The record is filed through the FHIR table and stored by a pull, and
    // the script reads it, before any attempt can run.
    running.worker.kill_now();
    let (scope, _) = accept_turn(&running.database.url, &running.store).await;
    let record = file_fhir_record(&scope.workspace, "ehr", content, None).unwrap();
    let receipt = running
        .store
        .record_chart_with_receipt(
            &scope.workspace,
            std::slice::from_ref(&record),
            &SourcePull {
                source_id: "ehr".into(),
                source_label: "EHR".into(),
                patient: SourcePatient {
                    source_patient_id: "P-1".into(),
                    display_name: "Nadia Okafor".into(),
                    birth_date: Some("1984-02-19".into()),
                    sex_or_gender: None,
                    medical_record_numbers: Vec::new(),
                },
                pulled_at: Some(1_760_000_000),
            },
        )
        .await
        .unwrap();
    assert!(receipt.missing_candidates.is_empty(), "{receipt:?}");
    running.stub.tool_rounds(&serde_json::json!([[
        {"name":"read_patient_sources","arguments":{"references":[{"candidate_id":record.candidate_id,"revision":record.revision,"cursor":null}],"max_result_bytes":4096}}
    ]]));
    running.worker.restart();
    let port = running.port();
    support::wait_ready(&mut running.worker, port).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(assistant_texts(&state), ["Creatinine was 0.9 mg/dL."]);
    let saved = serde_json::to_value(&state.messages).unwrap();
    let finished = &saved[1]["kind"]["finished"];
    let source =
        serde_json::json!({"candidate_id": record.candidate_id, "revision": record.revision});
    let start = record.content.find(quote).unwrap();
    assert_eq!(
        finished["marks"],
        serde_json::json!([{"at": 24, "source": source,
                            "place": {"passage": {"spans": [{"part": 0, "start": start, "end": start + quote.len()}]}}}])
    );
    assert_eq!(
        finished["basis"],
        serde_json::json!({
            "text": "The creatinine result.",
            "marks": [{"at": 21, "source": source, "place": null}]
        })
    );
}
