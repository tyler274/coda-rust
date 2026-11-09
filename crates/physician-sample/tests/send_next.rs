//! Send next on the hosted site: Coda holds the queue, not the browser.
//!
//! One site and one slow worker share a disposable Postgres, Valkey and the
//! Pub/Sub emulator. Two messages queued behind a running answer show on the
//! snapshot, take one edit and one withdrawal exactly once, and get one reply
//! after the answer ends. Two more queued behind the next answer go back to
//! the composer on Stop and are never sent.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;
use support::{get_json, sign_in};

struct Home(PathBuf);

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[tokio::test]
async fn queued_messages_get_one_reply_take_edits_once_and_return_on_stop() {
    let Some(database) = support::Database::create("send_next").await else {
        return;
    };
    support::staging_account(&database.url).await;

    let home = Home(std::env::temp_dir().join(format!(
        "physician-scale-send-next-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    )));
    std::fs::create_dir_all(&home.0).expect("home");
    let valkey = support::Valkey::start().await;
    let redis = valkey.url();
    let worker_port = support::free_port();
    let site_port = support::free_port();
    let emulator = support::Emulator::start().await;
    emulator.provision(worker_port).await;

    let topic = support::topic();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let site_listen = format!("127.0.0.1:{site_port}");
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_WORKER_FIXED_TEXT", support::FIXED_TEXT),
            // Long enough to queue, edit and withdraw behind the answer.
            ("PHYSICIAN_WORKER_FIXED_DELAY_MS", "4000"),
        ],
        &home.0,
    );
    support::wait_ready(&mut worker, worker_port).await;
    let mut site = support::Process::spawn(
        "site",
        support::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("GOOGLE_CLOUD_PROJECT", support::PROJECT),
            ("PHYSICIAN_CHAT_TOPIC", topic.as_str()),
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_LISTEN_ADDR", site_listen.as_str()),
        ],
        &home.0,
    );
    support::wait_ready(&mut site, site_port).await;
    let tails = |site: &support::Process, worker: &support::Process| {
        format!("{}\n{}", site.tail(), worker.tail())
    };

    let session = sign_in(&site_listen).await;
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let snapshot_path = format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot");
    let deadline = Instant::now() + Duration::from_secs(15);
    while valkey.subscribers() < 2 {
        assert!(
            Instant::now() < deadline,
            "expected the site and the worker on Valkey\n{}",
            tails(&site, &worker)
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let mut sse = support::Sse::open(
        &site_listen,
        &format!("/patient-workspaces/events?patient_workspace_id={workspace}"),
        &session,
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", site.tail()));
    let post = |body: Value| {
        let site_listen = site_listen.clone();
        let session = session.clone();
        async move { support::post(&site_listen, "/commands", Some(&session), &body).await }
    };

    let (status, body) = post(submit(workspace, &chat, "first", "Hello", None)).await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");
    assert_eq!(body["queued"], false, "{body}");
    wait_claimed(&database.url, &chat)
        .await
        .unwrap_or_else(|error| panic!("never claimed: {error}\n{}", tails(&site, &worker)));

    // Two Send next messages behind the running answer.
    for (key, text) in [("q1", "And the potassium?"), ("q2", "Also the creatinine")] {
        let (status, body) = post(submit(workspace, &chat, key, text, Some("queue"))).await;
        assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
        assert_eq!(body["outcome"], "accepted", "{body}");
        assert_eq!(body["queued"], true, "{body}");
    }
    sse.wait_for("event: workspace_changed", Duration::from_secs(20))
        .await
        .unwrap_or_else(|error| panic!("queue was never announced: {error}"));
    // Another signed-in page (any device) reads the same queue from Coda.
    let other = sign_in(&site_listen).await;
    let (status, body) = get_json(&site_listen, &snapshot_path, &other).await;
    assert_eq!(status, 200, "{body}");
    assert_eq!(
        queued(&body["snapshot"]),
        [("q1", "And the potassium?"), ("q2", "Also the creatinine")],
        "{body}"
    );
    assert_eq!(texts(&body["snapshot"]), ["Hello"]);

    // Edit q1 and withdraw q2, each retried after a lost receipt.
    let edit = change(
        "message.edit",
        workspace,
        &chat,
        "e1",
        "q1",
        Some("And the sodium?"),
    );
    let withdraw = change("message.withdraw", workspace, &chat, "w1", "q2", None);
    for command in [&edit, &edit, &withdraw, &withdraw] {
        let (status, body) = post(command.clone()).await;
        assert_eq!(
            (status, body["outcome"].as_str()),
            (200, Some("accepted")),
            "{command} {body}\n{}",
            tails(&site, &worker)
        );
    }
    // A withdrawn message is out of reach of a new command.
    let (status, body) = post(change(
        "message.edit",
        workspace,
        &chat,
        "e2",
        "q2",
        Some("Never"),
    ))
    .await;
    assert_eq!((status, body["outcome"].as_str()), (200, Some("rejected")));
    let (_, body) = get_json(&site_listen, &snapshot_path, &session).await;
    assert_eq!(
        queued(&body["snapshot"]),
        [("q1", "And the sodium?")],
        "{body}"
    );

    // The answer ends; the queue is answered in one reply.
    let answered = wait_snapshot(&site_listen, &snapshot_path, &session, |snapshot| {
        texts(snapshot).len() == 4 && mechanical(snapshot).is_none()
    })
    .await
    .unwrap_or_else(|snapshot| {
        panic!(
            "the queue was never answered\n{snapshot}\n{}",
            tails(&site, &worker)
        )
    });
    assert_eq!(
        texts(&answered),
        [
            "Hello",
            support::FIXED_TEXT,
            "And the sodium?",
            support::FIXED_TEXT
        ]
    );
    assert!(queued(&answered).is_empty(), "{answered}");
    // The started message can no longer change.
    let (status, body) = post(change(
        "message.withdraw",
        workspace,
        &chat,
        "w2",
        "q1",
        None,
    ))
    .await;
    assert_eq!((status, body["outcome"].as_str()), (200, Some("rejected")));

    // Two more behind the next answer; Stop returns both to the composer.
    let (status, body) = post(submit(workspace, &chat, "second", "One more", None)).await;
    assert_eq!((status, body["outcome"].as_str()), (200, Some("accepted")));
    let writing = wait_snapshot(&site_listen, &snapshot_path, &session, |snapshot| {
        mechanical(snapshot).is_some_and(|line| line.get("stop_target").is_some())
    })
    .await
    .unwrap_or_else(|snapshot| panic!("the next answer never offered Stop\n{snapshot}"));
    let work = mechanical(&writing).unwrap()["stop_target"]["work_item_id"]
        .as_str()
        .unwrap()
        .to_owned();
    for (key, text) in [("q3", "Draft one"), ("q4", "Draft two")] {
        let (status, body) = post(submit(workspace, &chat, key, text, Some("queue"))).await;
        assert_eq!(
            (status, body["queued"].as_bool()),
            (200, Some(true)),
            "{body}"
        );
    }
    let (status, body) = post(json!({
        "type": "work.stop",
        "idempotency_key": format!("stop-{work}"),
        "payload": {"patient_workspace_id": workspace, "chat_id": chat, "work_item_id": work},
    }))
    .await;
    assert_eq!((status, body["outcome"].as_str()), (200, Some("accepted")));
    let (status, draft) = get_json(
        &site_listen,
        &format!("/physician-state/drafts/{workspace}/{chat}"),
        &session,
    )
    .await;
    assert_eq!(status, 200, "{draft}");
    assert_eq!(draft["text"], "Draft one\n\nDraft two", "{draft}");
    // Stay past the 4 s answer so a send that ignored Stop would show.
    tokio::time::sleep(Duration::from_millis(5_000)).await;
    let (_, body) = get_json(&site_listen, &snapshot_path, &session).await;
    let snapshot = &body["snapshot"];
    assert!(queued(snapshot).is_empty(), "{snapshot}");
    assert!(
        !texts(snapshot).contains(&"Draft one") && !texts(snapshot).contains(&"Draft two"),
        "Stop sent a queued message\n{snapshot}"
    );
    worker.kill_now();
}

fn submit(workspace: &str, chat: &str, key: &str, text: &str, intent: Option<&str>) -> Value {
    let mut payload = json!({
        "patient_workspace_id": workspace,
        "chat_id": chat,
        "text": text,
        "physician_time_zone": "America/New_York",
    });
    if let Some(intent) = intent {
        payload["intent"] = json!(intent);
    }
    json!({"type": "message.submit", "idempotency_key": format!("{chat}-{key}"), "payload": payload})
}

fn change(
    kind: &str,
    workspace: &str,
    chat: &str,
    key: &str,
    message: &str,
    text: Option<&str>,
) -> Value {
    let mut payload = json!({
        "patient_workspace_id": workspace,
        "chat_id": chat,
        "message_id": format!("{chat}-{message}"),
    });
    if let Some(text) = text {
        payload["text"] = json!(text);
    }
    json!({"type": kind, "idempotency_key": format!("{chat}-{key}"), "payload": payload})
}

/// `(key suffix, text)` of each queued message, with the chat prefix removed.
fn queued(snapshot: &Value) -> Vec<(&str, &str)> {
    snapshot["queued"]
        .as_array()
        .map(|items| {
            items
                .iter()
                .map(|item| {
                    let id = item["message_id"].as_str().unwrap_or_default();
                    let suffix = id.rsplit_once('-').map_or(id, |(_, suffix)| suffix);
                    (suffix, item["text"].as_str().unwrap_or_default())
                })
                .collect()
        })
        .unwrap_or_default()
}

async fn wait_claimed(url: &str, chat: &str) -> Result<(), String> {
    let store = local_product::runtime::Store::connect(url)
        .await
        .map_err(|error| error.to_string())?;
    let scope = local_product::runtime::Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.to_owned(),
        physician: "staging-physician".to_owned(),
        workspace: physician_sample::WORKSPACE_IDS[0].to_owned(),
        chat: chat.to_owned(),
    };
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let status = store
            .read(&scope)
            .await
            .map_err(|error| error.to_string())?
            .map(|conversation| conversation.status);
        if status.as_deref() == Some("active") {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(format!("status {status:?}"));
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

fn mechanical(snapshot: &Value) -> Option<&Value> {
    snapshot["conversation"]
        .as_array()?
        .iter()
        .find(|item| item["kind"] == "mechanical")
}

fn texts(snapshot: &Value) -> Vec<&str> {
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

async fn wait_snapshot(
    host: &str,
    path: &str,
    session: &str,
    done: impl Fn(&Value) -> bool,
) -> Result<Value, Value> {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let (status, body) = get_json(host, path, session).await;
        assert_eq!(status, 200, "{body}");
        let snapshot = body["snapshot"].clone();
        if done(&snapshot) {
            return Ok(snapshot);
        }
        if Instant::now() >= deadline {
            return Err(snapshot);
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
