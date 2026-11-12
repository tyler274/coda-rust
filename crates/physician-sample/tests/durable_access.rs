//! Hosted chat work answers only the physician who owns it.
//!
//! One site and one slow worker share a disposable Postgres, Valkey and the
//! Pub/Sub emulator. The same `message.submit` sent twice is one turn with one
//! reply, and that reply carries its response id. A colleague on the
//! same panel, a physician without the patient on their panel, and a physician
//! in another practice each try to read, Stop, Resume and write into the
//! owner's chat. None of them sees or changes it.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use physician_auth::{AuthStore, Membership, PostgresAuthStore};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;
use support::{get_json, post, sign_in};

const STOPPED: &str = "Coda stopped this response. Your message is saved.";

struct Home(PathBuf);

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Someone other than the chat's owner, and what the site answers them.
struct Outsider {
    name: &'static str,
    session: String,
    /// The auth check refuses the workspace with a 404 before any handler
    /// runs. Otherwise the handler runs in the outsider's own scope.
    hidden: bool,
}

/// Signs a new physician in through the auth owner and sets their panel.
async fn outsider(
    url: &str,
    name: &'static str,
    practice: &str,
    workspace: &str,
    membership: Membership,
) -> Outsider {
    let auth = PostgresAuthStore::connect_without_migrations(url)
        .await
        .expect("auth store");
    let now = support::unix_now();
    let email = format!("{name}@example.test");
    auth.ensure_account(practice, name, &email, name, false, now)
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
    auth.set_membership(&identity, workspace, membership)
        .await
        .expect("membership");
    Outsider {
        name,
        session,
        hidden: membership != Membership::Allowed,
    }
}

#[tokio::test]
async fn chat_work_answers_only_its_owner_and_a_repeated_submit_is_one_turn() {
    let Some(database) = support::Database::create("durable_access").await else {
        return;
    };
    support::staging_account(&database.url).await;

    let home = Home(std::env::temp_dir().join(format!(
        "physician-scale-access-{}-{}",
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
    let site = format!("127.0.0.1:{site_port}");
    // Each answer takes long enough for every outsider to try Stop while it runs.
    let stub = support::ResponsesStub::start(&home.0, 0, 3000, "[]").await;
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
    let mut site_process = support::Process::spawn(
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
            ("PHYSICIAN_LISTEN_ADDR", site.as_str()),
        ],
        &home.0,
    );
    support::wait_ready(&mut site_process, site_port).await;
    let tails = || format!("{}\n{}", site_process.tail(), worker.tail());

    let owner = sign_in(&site).await;
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let snapshot_path = format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot");

    // The same submit twice, as a page retrying after a lost answer: both
    // are accepted, one turn runs, and one reply is published.
    let first = message(workspace, &chat, "Hello", "submit-once");
    for _ in 0..2 {
        let (status, body) = post(&site, "/commands", Some(&owner), &first).await;
        assert_eq!(status, 200, "{body}\n{}", tails());
        assert_eq!(body["outcome"], "accepted", "{body}");
    }
    wait_snapshot(&site, &snapshot_path, &owner, |snapshot| {
        texts(snapshot).contains(&support::FIXED_TEXT)
    })
    .await
    .unwrap_or_else(|snapshot| panic!("never answered\n{snapshot}\n{}", tails()));
    // Stay past a second turn's delay so a duplicate run would have published.
    tokio::time::sleep(Duration::from_millis(3_500)).await;
    let (status, body) = get_json(&site, &snapshot_path, &owner).await;
    assert_eq!(status, 200, "{body}");
    let answered = &body["snapshot"];
    assert_eq!(
        texts(answered),
        ["Hello", support::FIXED_TEXT],
        "{answered}"
    );
    let reply = &answered["conversation"][1];
    assert_eq!(reply["kind"], "model", "{answered}");
    assert_eq!(answered["runs"][0]["state"], "completed", "{answered}");
    let attempt = answered["runs"][0]["attempt_id"].as_str().unwrap();
    assert_eq!(
        reply["response_id"],
        format!("{attempt}:reply"),
        "the reply is named by the id its answering attempt saved\n{answered}"
    );
    assert!(answered["conversation"][0].get("response_id").is_none());

    // A second message, still writing when the outsiders arrive.
    let (status, body) = post(
        &site,
        "/commands",
        Some(&owner),
        &message(workspace, &chat, "Check sodium", "submit-second"),
    )
    .await;
    assert_eq!(
        (status, &body["outcome"]),
        (200, &json!("accepted")),
        "{body}"
    );
    let writing = wait_snapshot(&site, &snapshot_path, &owner, |snapshot| {
        mechanical(snapshot).is_some_and(|line| line.get("stop_target").is_some())
    })
    .await
    .unwrap_or_else(|snapshot| panic!("never offered Stop\n{snapshot}\n{}", tails()));
    let work = mechanical(&writing).unwrap()["stop_target"]["work_item_id"]
        .as_str()
        .unwrap()
        .to_owned();

    let outsiders = [
        outsider(
            &database.url,
            "colleague",
            physician_sample::STAGING_PRACTICE_ID,
            workspace,
            Membership::Allowed,
        )
        .await,
        outsider(
            &database.url,
            "off-panel",
            physician_sample::STAGING_PRACTICE_ID,
            workspace,
            Membership::Denied,
        )
        .await,
        outsider(
            &database.url,
            "elsewhere",
            &format!("practice-{}", uuid::Uuid::new_v4().simple()),
            workspace,
            Membership::Denied,
        )
        .await,
    ];
    for outsider in &outsiders {
        let name = outsider.name;
        let (status, body) = get_json(&site, &snapshot_path, &outsider.session).await;
        if outsider.hidden {
            assert_eq!(status, 404, "{name} read the chat: {body}");
        } else {
            assert_eq!(status, 200, "{name}: {body}");
            assert_eq!(
                body["snapshot"],
                json!({ "conversation": [], "runs": [] }),
                "{name} read the owner's chat"
            );
        }
        let stop = stop(workspace, &chat, &work, &format!("stop-{name}"));
        refused(&site, &outsider.session, &stop, outsider.hidden, name).await;
    }
    let (_, body) = get_json(&site, &snapshot_path, &owner).await;
    assert!(
        mechanical(&body["snapshot"]).is_some_and(|line| line.get("stop_target").is_some()),
        "an outsider's Stop held the owner's work\n{body}"
    );

    // The owner stops it. Now Resume has a real target to name.
    let (status, body) = post(
        &site,
        "/commands",
        Some(&owner),
        &stop(workspace, &chat, &work, "stop-owner"),
    )
    .await;
    assert_eq!(
        (status, &body["outcome"]),
        (200, &json!("accepted")),
        "{body}"
    );
    let held = wait_snapshot(&site, &snapshot_path, &owner, |snapshot| {
        mechanical(snapshot).is_some_and(|line| line["text"] == STOPPED)
    })
    .await
    .unwrap_or_else(|snapshot| panic!("Stop did not hold\n{snapshot}"));
    let run = mechanical(&held).unwrap()["resume_target"]["expected_run_id"]
        .as_str()
        .unwrap()
        .to_owned();
    for outsider in &outsiders {
        let name = outsider.name;
        let resume = resume(workspace, &chat, &work, &run, &format!("resume-{name}"));
        refused(&site, &outsider.session, &resume, outsider.hidden, name).await;
        // A message under the owner's own key lands, if anywhere, in the
        // outsider's scope: receipts belong to one physician.
        let submit = message(workspace, &chat, "Not yours", "submit-once");
        let (status, body) = post(&site, "/commands", Some(&outsider.session), &submit).await;
        if outsider.hidden {
            assert_eq!(status, 404, "{name} wrote into the workspace: {body}");
        } else {
            assert_eq!(
                (status, body["outcome"].as_str()),
                (200, Some("accepted")),
                "{name}'s own chat under the same key: {body}"
            );
        }
    }
    // Outsiders' work never reaches the owner's chat. Wait past the worker's
    // delay so the colleague's own turn has run.
    tokio::time::sleep(Duration::from_millis(3_500)).await;
    let (_, body) = get_json(&site, &snapshot_path, &owner).await;
    let after = &body["snapshot"];
    assert_eq!(
        texts(after),
        ["Hello", support::FIXED_TEXT, "Check sodium"],
        "{after}"
    );
    assert_eq!(mechanical(after).unwrap()["text"], STOPPED, "{after}");
    assert_eq!(
        mechanical(after).unwrap()["resume_target"]["expected_run_id"],
        run.as_str(),
        "{after}"
    );
    // The colleague's message opened their own chat, which holds only it.
    let colleague = &outsiders[0];
    let (status, body) = get_json(&site, &snapshot_path, &colleague.session).await;
    assert_eq!(status, 200, "{body}");
    assert_eq!(
        texts(&body["snapshot"])[0],
        "Not yours",
        "{}",
        body["snapshot"]
    );
    assert!(!texts(&body["snapshot"]).contains(&"Hello"));
}

/// `command` from `session` is refused: a 404 when the workspace is hidden
/// from them, otherwise the handler's own rejection.
async fn refused(site: &str, session: &str, command: &Value, hidden: bool, name: &str) {
    let kind = &command["type"];
    let (status, body) = post(site, "/commands", Some(session), command).await;
    if hidden {
        assert_eq!(status, 404, "{name} {kind}: {body}");
    } else {
        assert_eq!(
            (status, body["outcome"].as_str()),
            (200, Some("rejected")),
            "{name} {kind}: {body}"
        );
    }
}

fn message(workspace: &str, chat: &str, text: &str, key: &str) -> Value {
    json!({
        "type": "message.submit",
        "idempotency_key": key,
        "payload": {
            "patient_workspace_id": workspace,
            "chat_id": chat,
            "text": text,
            "physician_time_zone": "America/New_York"
        }
    })
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

fn resume(workspace: &str, chat: &str, work: &str, run: &str, key: &str) -> Value {
    json!({
        "type": "work.resume",
        "idempotency_key": key,
        "payload": {
            "patient_workspace_id": workspace,
            "chat_id": chat,
            "work_item_id": work,
            "expected_run_id": run,
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
