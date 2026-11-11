//! Instances that die mid-turn, against the Pub/Sub emulator, Valkey and a
//! disposable Postgres.
//!
//! A site instance accepts a turn and dies before its wakeup reaches the
//! broker. Another site instance's recovery sweep republishes it, and the
//! worker answers; no one reloads or presses anything. A turn whose worker
//! leases all run out ends as failed: the next push for it tells open pages to
//! refetch and is not acknowledged, so the broker dead-letters it.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use harness::engine::MAX_FAILED_ATTEMPTS;
use local_product::runtime::delivery::{Delivery, DeliveryState};
use local_product::runtime::{Store, WorkStatus};
use physician_auth::AuthStore;
use physician_bus::{Bus, Event};
use serde_json::Value;

#[path = "scale/mod.rs"]
mod support;

fn lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

struct Home(PathBuf);

impl Home {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "physician-scale-{label}-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&path).expect("home");
        Self(path)
    }
}

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn spawn_worker(
    stub: &support::ResponsesStub,
    home: &Home,
    database: &str,
    redis: &str,
    emulator: &str,
    listen: &str,
) -> support::Process {
    support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database),
            ("PHYSICIAN_REDIS_URL", redis),
            ("PUBSUB_EMULATOR_HOST", emulator),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", listen),
            ("PHYSICIAN_RESPONSES_STUB_PORT", &stub.port),
        ],
        &home.0,
    )
}

fn spawn_site(
    name: &str,
    home: &Home,
    database: &str,
    redis: &str,
    emulator: &str,
    topic: &str,
    listen: &str,
) -> support::Process {
    support::Process::spawn(
        name,
        support::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database),
            ("PHYSICIAN_REDIS_URL", redis),
            ("PUBSUB_EMULATOR_HOST", emulator),
            ("GOOGLE_CLOUD_PROJECT", support::PROJECT),
            ("PHYSICIAN_CHAT_TOPIC", topic),
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_LISTEN_ADDR", listen),
        ],
        &home.0,
    )
}

fn port(listen: &str) -> u16 {
    listen
        .rsplit_once(':')
        .and_then(|(_, port)| port.parse().ok())
        .expect("listen port")
}

fn assistant_texts(snapshot: &Value) -> Vec<&str> {
    snapshot["conversation"]
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter(|item| item["kind"] != "mechanical")
                .filter_map(|item| item["text"].as_str())
                .collect()
        })
        .unwrap_or_default()
}

/// Dev's P1: work accepted by an instance that is gone must still finish.
/// Hosted, the physician site keeps one instance whenever scaling is on, so
/// some instance always runs this sweep.
#[tokio::test]
async fn a_turn_accepted_by_a_dead_site_instance_is_finished_by_another_instances_sweep() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("instance_loss").await else {
        return;
    };
    support::staging_account(&database.url).await;
    let home = Home::new("instance-loss");
    let valkey = support::Valkey::start().await;
    let redis = valkey.url();
    let worker_listen = format!("127.0.0.1:{}", support::free_port());
    let emulator = support::Emulator::start().await;
    emulator.provision(port(&worker_listen)).await;

    // Instance A publishes to a topic the broker does not have: every wakeup
    // it sends is refused, the same as an instance that dies before its
    // publish lands. Its acceptance is already saved in Postgres.
    let missing = format!("projects/{}/topics/not-provisioned", support::PROJECT);
    let a_listen = format!("127.0.0.1:{}", support::free_port());
    let mut site_a = spawn_site(
        "site-a",
        &home,
        &database.url,
        &redis,
        &emulator.host,
        &missing,
        &a_listen,
    );
    support::wait_ready(&mut site_a, port(&a_listen)).await;

    let session = support::sign_in(&a_listen).await;
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let snapshot_path = format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot");
    let (status, body) = support::post(
        &a_listen,
        "/commands",
        Some(&session),
        &support::submit(workspace, &chat, "Hello"),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", site_a.tail());
    assert_eq!(body["outcome"], "accepted", "{body}");
    assert!(
        site_a.tail().contains("awaits queue recovery"),
        "instance A's publish should have been refused\n{}",
        site_a.tail()
    );

    // Instance A dies holding the only wakeup it tried to send.
    site_a.kill_now();
    let store = Store::connect(&database.url).await.expect("runtime store");
    let delivery = {
        let start = local_product::runtime::Scope {
            practice: physician_sample::STAGING_PRACTICE_ID.to_owned(),
            physician: String::new(),
            workspace: String::new(),
            chat: String::new(),
        };
        let due = store
            .dispatch_due(100, Some(&start))
            .await
            .expect("dispatch_due");
        let mut matching = due
            .deliveries
            .into_iter()
            .filter(|delivery| delivery.scope.chat == chat);
        let delivery = matching
            .next()
            .expect("the dead instance's turn should be due");
        assert!(matching.next().is_none());
        delivery
    };
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Due
    );

    // Instance B starts (Cloud Run replaces the instance, or another one is
    // already serving). Its first sweep runs at startup. The page reconnects
    // to B; it sends nothing and presses nothing.
    let b_listen = format!("127.0.0.1:{}", support::free_port());
    let topic = support::topic();
    let mut site_b = spawn_site(
        "site-b",
        &home,
        &database.url,
        &redis,
        &emulator.host,
        &topic,
        &b_listen,
    );
    support::wait_ready(&mut site_b, port(&b_listen)).await;
    let tails = |site: &support::Process, worker: &support::Process| {
        format!("{}\n{}", site.tail(), worker.tail())
    };
    let mut sse = support::Sse::open(
        &b_listen,
        &format!("/patient-workspaces/events?patient_workspace_id={workspace}"),
        &session,
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", site_b.tail()));

    // B may already have republished the input, but it cannot complete until
    // the worker starts. Subscribe first: invalidations are live notices, not
    // a replay log for pages that were disconnected when the reply landed.
    let stub = support::ResponsesStub::start(&home.0, 0, 0, "[]").await;
    let mut worker = spawn_worker(
        &stub,
        &home,
        &database.url,
        &redis,
        &emulator.host,
        &worker_listen,
    );
    support::wait_ready(&mut worker, port(&worker_listen)).await;

    let deadline = Instant::now() + Duration::from_secs(45);
    let answered = loop {
        let (status, body) = support::get_json(&b_listen, &snapshot_path, &session).await;
        assert_eq!(status, 200, "{body}");
        let snapshot = body["snapshot"].clone();
        if assistant_texts(&snapshot).contains(&support::FIXED_TEXT) {
            break snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "instance B's sweep never finished the dead instance's turn: {snapshot}\n{}",
            tails(&site_b, &worker)
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    };
    assert_eq!(assistant_texts(&answered), ["Hello", support::FIXED_TEXT]);
    let state = store
        .read(&delivery.scope)
        .await
        .expect("read")
        .expect("conversation");
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(
        state
            .attempts
            .iter()
            .map(|attempt| sea_orm::ActiveEnum::to_value(&attempt.status))
            .collect::<Vec<_>>(),
        ["completed"],
        "recovery republishes the wakeup; it never runs the turn twice"
    );
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Settled
    );
    sse.wait_for("event: workspace_changed", Duration::from_secs(10))
        .await
        .unwrap_or_else(|error| {
            panic!(
                "the page on instance B was never told the reply landed: {error}\n{}",
                tails(&site_b, &worker)
            )
        });
    drop(sse);
    let _reconnected = support::Sse::open(
        &b_listen,
        &format!("/patient-workspaces/events?patient_workspace_id={workspace}"),
        &session,
    )
    .await
    .expect("reconnect after completion");
    let (status, snapshot) = support::get_json(&b_listen, &snapshot_path, &session).await;
    assert_eq!(status, 200);
    assert_eq!(
        assistant_texts(&snapshot["snapshot"]),
        ["Hello", support::FIXED_TEXT],
        "a reconnect reads the completed reply without waiting for a past invalidation"
    );
}

/// Dev's P2: a turn whose worker leases all run out (each worker died
/// mid-turn) ends as failed. The push that finds it so announces the change,
/// so a held page shows "couldn't finish", and is not acknowledged, so the
/// broker forwards it to the dead-letter topic after its five deliveries.
#[tokio::test]
async fn exhausted_leases_announce_the_failed_turn_and_dead_letter_its_wakeup() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("lease_exhausted").await else {
        return;
    };
    let home = Home::new("lease-exhausted");
    let valkey = support::Valkey::start().await;
    let redis = valkey.url();
    let worker_listen = format!("127.0.0.1:{}", support::free_port());
    let emulator = support::Emulator::start().await;
    emulator.provision(port(&worker_listen)).await;
    let stub = support::ResponsesStub::start(&home.0, 0, 0, "[]").await;
    let mut worker = spawn_worker(
        &stub,
        &home,
        &database.url,
        &redis,
        &emulator.host,
        &worker_listen,
    );
    support::wait_ready(&mut worker, port(&worker_listen)).await;
    let store = Store::connect(&database.url).await.expect("runtime store");
    let delivery = accepted(&database.url, &store).await;

    // Each claim stands for a worker instance that took the turn and died
    // before it finished: nothing returns it, and its one-second lease runs
    // out. The worker's own lease is 540 s; only the length differs.
    for _ in 0..MAX_FAILED_ATTEMPTS {
        let claim = store
            .claim_delivery(&delivery, 1)
            .await
            .expect("claim")
            .expect("the turn should be claimable while its leases remain");
        assert_eq!(claim.scope, delivery.scope);
        // The database clock counts whole seconds and a lease ends once
        // `expires <= now`, so one second always ends it; 200 ms is margin.
        tokio::time::sleep(Duration::from_millis(1200)).await;
    }
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Due,
        "an expired lease is still runnable until a claim records it"
    );

    // A page on another instance listens for this workspace.
    let bus = physician_bus::RedisBus::connect(&redis, None)
        .await
        .expect("valkey bus");
    let mut events = bus.subscribe();
    let deadline = Instant::now() + Duration::from_secs(10);
    while !bus.live() {
        assert!(Instant::now() < deadline, "test bus never subscribed");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    // The push that finds the last lease run out records the fifth failure.
    let (status, text) = support::exchange(
        &worker_listen,
        "POST",
        "/pubsub/chat-turn",
        &[("content-type", "application/json")],
        Some(&support::push_body(&delivery, "lease-exhausted")),
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", worker.tail()));
    assert_eq!(
        status, 500,
        "a failed turn's wakeup must not be acknowledged: {text}"
    );
    let state = store
        .read(&delivery.scope)
        .await
        .expect("read")
        .expect("conversation");
    assert_eq!(state.status, WorkStatus::Failed, "{:?}", state.attempts);
    assert!(
        state
            .messages
            .iter()
            .all(|message| message.role.as_str() != "assistant"),
        "the model never ran"
    );
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Failed
    );
    let expected = Event::WorkspaceChanged {
        workspace_id: delivery.scope.workspace.clone(),
    };
    loop {
        let event = tokio::time::timeout(Duration::from_secs(10), events.recv())
            .await
            .unwrap_or_else(|_| {
                panic!("no workspace change for the failed turn\n{}", worker.tail())
            })
            .expect("bus");
        if event == expected {
            break;
        }
    }
    // The sweep leaves it alone: Try again is the only way back.
    let swept = store.dispatch_due(100, None).await.expect("sweep");
    assert!(!swept.deliveries.contains(&delivery));

    // The broker's own deliveries of the wakeup are refused too, five times,
    // and the message lands on the dead-letter topic.
    emulator.publish(&delivery).await;
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        if emulator.pull_dead_letters().await.into_iter().any(|bytes| {
            serde_json::from_slice::<Delivery>(&bytes).ok().as_ref() == Some(&delivery)
        }) {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the failed turn's wakeup was never dead-lettered\n{}\n{}",
            worker.tail(),
            emulator.tail()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

/// One authorized, accepted turn saved straight to the runtime store.
async fn accepted(url: &str, store: &Store) -> Delivery {
    let auth = physician_auth::PostgresAuthStore::connect_without_migrations(url)
        .await
        .expect("auth store");
    let now = support::unix_now();
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let email = format!("lease-{unique}@example.test");
    let practice = format!("practice-{unique}");
    let physician = format!("physician-{unique}");
    let workspace = format!("workspace-{unique}");
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
    let token = auth.begin_sign_in(&email, now).await.expect("token");
    let (identity, session) = auth
        .complete_sign_in(&email, &token, now)
        .await
        .expect("session");
    auth.register_patient(&identity.practice_id, &workspace)
        .await
        .unwrap();
    auth.set_membership(&identity, &workspace, physician_auth::Membership::Allowed)
        .await
        .unwrap();
    let scope = local_product::runtime::Scope {
        practice: identity.practice_id,
        physician: identity.physician_id,
        workspace,
        chat: format!("chat-{unique}"),
    };
    let key = format!("input-{unique}");
    store
        .accept_authorized(
            &scope,
            &local_product::runtime::Input {
                key: key.clone(),
                text: "Hello".into(),
                timezone: "America/New_York".into(),
                scenario: None,
            },
            physician_auth::PostgresAuthStore::session_reference(&session),
            &local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap(),
            &support::models::chat_recipe(),
        )
        .await
        .expect("accept");
    Delivery::chat_turn(scope, key)
}
