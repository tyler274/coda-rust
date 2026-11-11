//! A physician Stop holds the turn until Resume or a newer message.
//!
//! One site and one slow worker share a disposable Postgres, Valkey and the
//! Pub/Sub emulator. Stop during the turn keeps the row held after the worker
//! is dropped, and the late reply is not published. Resume on a new worker
//! starts a successor, a second Stop holds that too, and a message sent
//! while held is answered without either stopped reply.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;
use support::{get_json, sign_in};

const STOPPED: &str = "Coda stopped this response. Your message is saved.";

struct Home(PathBuf);

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[tokio::test]
async fn stop_holds_across_restart_and_again_after_resume_until_a_newer_message() {
    let Some(database) = support::Database::create("stop").await else {
        return;
    };
    support::staging_account(&database.url).await;

    let home = Home(std::env::temp_dir().join(format!(
        "physician-scale-stop-{}-{}",
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
    let stub = support::ResponsesStub::start(&home.0, 0, 4000, "[]").await;
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", &stub.port),
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
    let (status, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &json!({
            "type": "message.submit",
            "idempotency_key": format!("key-{}", uuid::Uuid::new_v4().simple()),
            "payload": {
                "patient_workspace_id": workspace,
                "chat_id": chat,
                "text": "Hello",
                "physician_time_zone": "America/New_York"
            }
        }),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");

    // Stop only after the worker has claimed, so the provider call is in flight
    // and a late reply has time to publish if the fence fails.
    wait_claimed(&database.url, &chat)
        .await
        .unwrap_or_else(|error| {
            panic!(
                "the worker never claimed the turn: {error}\n{}",
                tails(&site, &worker)
            )
        });
    let claimed_at = Instant::now();
    let writing = wait_snapshot(&site_listen, &snapshot_path, &session, |snapshot| {
        mechanical(snapshot).is_some_and(|line| line.get("stop_target").is_some())
    })
    .await
    .unwrap_or_else(|snapshot| {
        panic!(
            "the turn never offered Stop\n{snapshot}\n{}",
            tails(&site, &worker)
        )
    });
    let work = mechanical(&writing).unwrap()["stop_target"]["work_item_id"]
        .as_str()
        .unwrap()
        .to_owned();
    let stop_key = format!("stop-{work}");
    let (status, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &stop(workspace, &chat, &work, &stop_key),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");
    let (status, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &stop(workspace, &chat, &work, &stop_key),
    )
    .await;
    assert_eq!(body["outcome"], "accepted", "{status} {body}");
    // A blank work identity is the caller's mistake: refused, not a 503.
    let (status, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &stop(workspace, &chat, " ", "stop-blank"),
    )
    .await;
    assert_eq!(
        (status, body["outcome"].as_str()),
        (200, Some("rejected")),
        "{body}"
    );

    sse.wait_for("event: workspace_changed", Duration::from_secs(20))
        .await
        .unwrap_or_else(|error| {
            panic!(
                "the held stream was never told the turn stopped: {error}\n{}",
                tails(&site, &worker)
            )
        });
    let held = wait_snapshot(&site_listen, &snapshot_path, &session, |snapshot| {
        mechanical(snapshot).is_some_and(|line| line["text"] == STOPPED)
    })
    .await
    .unwrap_or_else(|snapshot| panic!("Stop did not hold the turn\n{snapshot}"));
    assert_eq!(texts(&held), ["Hello"]);
    assert!(!texts(&held).contains(&support::FIXED_TEXT));
    let run = mechanical(&held).unwrap()["resume_target"]["expected_run_id"]
        .as_str()
        .unwrap()
        .to_owned();
    assert_eq!(held["runs"][0]["state"], "held");

    // The fixed reply waits 4s. Stay up past that so a commit that ignored
    // Stop would already be on the snapshot before the worker is dropped.
    let remain = Duration::from_millis(5_000).saturating_sub(claimed_at.elapsed());
    tokio::time::sleep(remain).await;
    assert!(
        worker.exited().is_none(),
        "the worker exited before a late reply could be refused\n{}",
        worker.tail()
    );
    let (status, body) = get_json(&site_listen, &snapshot_path, &session).await;
    assert_eq!(status, 200, "{body}");
    let snapshot = &body["snapshot"];
    assert_eq!(
        texts(snapshot),
        ["Hello"],
        "a late reply published\n{snapshot}"
    );
    assert_eq!(mechanical(snapshot).unwrap()["text"], STOPPED, "{snapshot}");

    worker.kill_now();
    let (status, body) = get_json(&site_listen, &snapshot_path, &session).await;
    assert_eq!(status, 200, "{body}");
    let snapshot = &body["snapshot"];
    assert_eq!(texts(snapshot), ["Hello"], "{snapshot}");
    assert_eq!(
        mechanical(snapshot).unwrap()["text"],
        STOPPED,
        "restart released Stop\n{snapshot}"
    );

    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", &stub.port),
        ],
        &home.0,
    );
    support::wait_ready(&mut worker, worker_port).await;
    let (status, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &json!({
            "type": "work.resume",
            "idempotency_key": format!("resume-{work}-{run}"),
            "payload": {
                "patient_workspace_id": workspace,
                "chat_id": chat,
                "work_item_id": work,
                "expected_run_id": run,
            }
        }),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");

    // Resume keeps the work's identity. A second Stop is a new press with
    // its own key, and it holds the resumed run just as the first one did.
    wait_claimed(&database.url, &chat)
        .await
        .unwrap_or_else(|error| {
            panic!(
                "the worker never claimed the resumed turn: {error}\n{}",
                tails(&site, &worker)
            )
        });
    let resumed_at = Instant::now();
    let (status, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &stop(workspace, &chat, &work, &format!("stop-again-{work}")),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");
    let held_again = wait_snapshot(&site_listen, &snapshot_path, &session, |snapshot| {
        mechanical(snapshot).is_some_and(|line| line["text"] == STOPPED)
    })
    .await
    .unwrap_or_else(|snapshot| {
        panic!(
            "the second Stop did not hold the resumed run\n{snapshot}\n{}",
            tails(&site, &worker)
        )
    });
    let resumed_run = mechanical(&held_again).unwrap()["resume_target"]["expected_run_id"]
        .as_str()
        .unwrap()
        .to_owned();
    assert_ne!(resumed_run, run, "{held_again}");
    let remain = Duration::from_millis(5_000).saturating_sub(resumed_at.elapsed());
    tokio::time::sleep(remain).await;
    let (status, body) = get_json(&site_listen, &snapshot_path, &session).await;
    assert_eq!(status, 200, "{body}");
    let snapshot = &body["snapshot"];
    assert_eq!(
        texts(snapshot),
        ["Hello"],
        "the resumed run published over the second Stop\n{snapshot}"
    );
    assert_eq!(mechanical(snapshot).unwrap()["text"], STOPPED, "{snapshot}");

    // Sending something else ends the hold: that message is answered once,
    // and neither stopped run's reply appears.
    let (status, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &json!({
            "type": "message.submit",
            "idempotency_key": format!("key-{}", uuid::Uuid::new_v4().simple()),
            "payload": {
                "patient_workspace_id": workspace,
                "chat_id": chat,
                "text": "Something else",
                "physician_time_zone": "America/New_York"
            }
        }),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");
    let answered = wait_snapshot(&site_listen, &snapshot_path, &session, |snapshot| {
        texts(snapshot).contains(&support::FIXED_TEXT)
    })
    .await
    .unwrap_or_else(|snapshot| {
        panic!(
            "the message sent while stopped was never answered\n{snapshot}\n{}",
            tails(&site, &worker)
        )
    });
    assert_eq!(
        texts(&answered),
        ["Hello", "Something else", support::FIXED_TEXT]
    );
    assert!(mechanical(&answered).is_none(), "{answered}");
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
        match store.read(&scope).await {
            Ok(Some(conversation))
                if conversation.status == local_product::runtime::WorkStatus::Active =>
            {
                return Ok(());
            }
            Ok(other) => {
                if Instant::now() >= deadline {
                    let status = other.map(|conversation| conversation.status);
                    return Err(format!("status {status:?}"));
                }
            }
            Err(error) => {
                if Instant::now() >= deadline {
                    return Err(error.to_string());
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

fn stop(workspace: &str, chat: &str, work: &str, key: &str) -> Value {
    json!({
        "type": "work.stop",
        "idempotency_key": key,
        "payload": {
            "patient_workspace_id": workspace,
            "chat_id": chat,
            "work_item_id": work,
        }
    })
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

async fn post_json(host: &str, path: &str, bearer: &str, body: &Value) -> (u16, Value) {
    support::post(host, path, Some(bearer), body).await
}
