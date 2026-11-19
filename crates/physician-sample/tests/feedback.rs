//! Hosted response feedback through two authenticated site instances and
//! Postgres. A chat is seeded through the runtime store, so no worker, model,
//! Valkey or Pub/Sub service is needed; the unused publisher points only at
//! loopback.
use std::path::PathBuf;

use physician_auth::{AuthStore, Membership, PostgresAuthStore};
use physician_wire::{AnswerState, Exchange};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

struct Running {
    database: support::Database,
    /// The response ids of Coda's two answers in the rated chat.
    responses: Vec<local_product::runtime::ResponseId>,
    /// The response id of an answer in a different chat of the same physician.
    elsewhere: local_product::runtime::ResponseId,
    sites: Vec<support::Process>,
    hosts: [String; 2],
    session: String,
    home: PathBuf,
}

impl Drop for Running {
    fn drop(&mut self) {
        for site in &mut self.sites {
            site.kill_now();
        }
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

impl Running {
    async fn start(chat: &str) -> Option<Self> {
        let database = support::Database::create("feedback").await?;
        support::staging_account(&database.url).await;
        let store = local_product::runtime::Store::connect(&database.url)
            .await
            .expect("runtime store");
        let responses = support::answered_chat(
            &store,
            physician_sample::WORKSPACE_IDS[0],
            chat,
            &[
                ("Latest potassium?", "Potassium was 4.1 on Monday."),
                ("And sodium?", "Sodium was 139 on Monday."),
            ],
        )
        .await;
        let elsewhere = support::answered_chat(
            &store,
            physician_sample::WORKSPACE_IDS[0],
            "chat-elsewhere",
            &[("Any allergies?", "None recorded.")],
        )
        .await
        .remove(0);
        let home = std::env::temp_dir().join(format!(
            "physician-feedback-{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&home).expect("home");
        let mut sites = Vec::new();
        let mut hosts = [String::new(), String::new()];
        for (index, host) in hosts.iter_mut().enumerate() {
            let port = support::free_port();
            *host = format!("127.0.0.1:{port}");
            let mut site = support::Process::spawn(
                &format!("site-{index}"),
                support::site_bin(),
                &[],
                &[
                    ("PHYSICIAN_DATABASE_URL", &database.url),
                    support::DURABLE,
                    ("PHYSICIAN_AUTH", "required"),
                    ("PHYSICIAN_LISTEN_ADDR", host),
                ],
                &home,
            );
            support::wait_ready(&mut site, port).await;
            sites.push(site);
        }
        let session = support::sign_in(&hosts[0]).await;
        Some(Self {
            database,
            responses,
            elsewhere,
            sites,
            hosts,
            session,
            home,
        })
    }

    fn logs(&self) -> String {
        format!("{}\n{}", self.sites[0].tail(), self.sites[1].tail())
    }

    /// Another physician of the practice, with the patient on their panel or
    /// not; returns their bearer.
    async fn colleague(&self, membership: Membership) -> String {
        let auth = PostgresAuthStore::connect_without_migrations(&self.database.url)
            .await
            .unwrap();
        let email = "colleague@example.test";
        auth.ensure_account(
            physician_sample::STAGING_PRACTICE_ID,
            "colleague",
            email,
            "Colleague",
            false,
            support::unix_now(),
        )
        .await
        .unwrap();
        let token = auth
            .begin_sign_in(email, support::unix_now())
            .await
            .unwrap();
        let (identity, bearer) = auth
            .complete_sign_in(email, &token, support::unix_now())
            .await
            .unwrap();
        auth.set_membership(&identity, physician_sample::WORKSPACE_IDS[0], membership)
            .await
            .unwrap();
        bearer
    }
}

async fn put(host: &str, bearer: Option<&str>, path: &str, body: &Value) -> (u16, Value) {
    let bytes = serde_json::to_vec(body).unwrap();
    let authorization = bearer.map(|bearer| format!("Bearer {bearer}"));
    let mut headers = vec![("content-type", "application/json")];
    if let Some(authorization) = &authorization {
        headers.push(("authorization", authorization));
    }
    let (status, text) = support::exchange(host, "PUT", path, &headers, Some(&bytes))
        .await
        .expect("feedback save");
    (
        status,
        serde_json::from_str(&text).unwrap_or(json!({"raw": text})),
    )
}

/// The model turns of `chat`'s snapshot as `(response_id, feedback)`, read by
/// `bearer` through `host`.
async fn rated_turns(host: &str, bearer: &str, workspace: &str, chat: &str) -> Vec<(Value, Value)> {
    let (status, snapshot) = support::get_json(
        host,
        &format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot"),
        bearer,
    )
    .await;
    assert_eq!(status, 200, "{snapshot}");
    support::chat(&snapshot["snapshot"])
        .transcript
        .iter()
        .map(|exchange| {
            let (id, feedback) = finished(exchange);
            (json!(id), json!(feedback))
        })
        .collect()
}

/// A finished answer's response id and the feedback the snapshot carries on it.
fn finished(exchange: &Exchange) -> (&str, Option<&physician_wire::ResponseFeedback>) {
    match &exchange.answer {
        AnswerState::Finished {
            response_id,
            feedback,
            ..
        } => (response_id.as_str(), feedback.as_ref()),
        other => panic!("an unfinished answer: {other:?}"),
    }
}

/// A save lands on the response it names, replaces its rating and details
/// with a new revision, reads back on the snapshot through the other
/// instance, and belongs to its author.
#[tokio::test]
async fn feedback_stays_on_its_response_and_with_its_author() {
    let chat = "chat-feedback";
    let Some(running) = Running::start(chat).await else {
        return;
    };
    let (a, b, session) = (&running.hosts[0], &running.hosts[1], &running.session);
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let base = format!("/physician-state/feedback/{workspace}/{chat}");

    let [first, second] = [running.responses[0].as_str(), running.responses[1].as_str()];
    assert_ne!(first, second, "each answer has its own response id");

    // The snapshot names each of Coda's responses by its response id, and
    // carries no feedback before any is saved.
    let (status, snapshot) = support::get_json(
        b,
        &format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot"),
        session,
    )
    .await;
    assert_eq!(status, 200, "{snapshot}\n{}", running.logs());
    let named: Vec<_> = support::chat(&snapshot["snapshot"])
        .transcript
        .iter()
        .map(|exchange| {
            let (id, feedback) = finished(exchange);
            (id.to_owned(), feedback.is_none())
        })
        .collect();
    assert_eq!(
        named,
        [(first.to_owned(), true), (second.to_owned(), true)],
        "each answer is named by its response id, and none is rated yet: {snapshot}"
    );

    let rated = put(
        a,
        Some(session),
        &format!("{base}/{second}"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(
        rated,
        (
            200,
            json!({"revision": 1, "rating": "down", "details": null})
        ),
        "{}",
        running.logs()
    );
    // The details travel with the rating; the dialog sends both.
    let details = json!({"reasons": ["missing_context", "incorrect_information"], "text": "Sodium was rechecked Tuesday."});
    let saved = put(
        b,
        Some(session),
        &format!("{base}/{second}"),
        &json!({"rating": "down", "details": details}),
    )
    .await;
    assert_eq!(saved.0, 200, "{}", saved.1);
    assert_eq!(saved.1["revision"], 2);
    assert_eq!(
        saved.1["details"]["reasons"],
        json!(["incorrect_information", "missing_context"])
    );
    // A new thumb sent with the saved details keeps them.
    let rated = put(
        b,
        Some(session),
        &format!("{base}/{second}"),
        &json!({"rating": "up", "details": saved.1["details"]}),
    )
    .await;
    assert_eq!(
        (
            &rated.1["revision"],
            &rated.1["rating"],
            &rated.1["details"]["text"]
        ),
        (
            &json!(3),
            &json!("up"),
            &json!("Sodium was rechecked Tuesday.")
        )
    );
    let up = put(
        a,
        Some(session),
        &format!("{base}/{first}"),
        &json!({"rating": "up"}),
    )
    .await;
    assert_eq!(up.0, 200, "{}", up.1);
    // Each response's snapshot turn carries its own saved feedback.
    assert_eq!(
        rated_turns(b, session, workspace, chat).await,
        [
            (json!(first), up.1.clone()),
            (json!(second), rated.1.clone()),
        ]
    );

    // Only one of this chat's published responses takes feedback.
    let elsewhere = running.elsewhere.as_str();
    for response in [elsewhere, "no-such-response", "1"] {
        let refused = put(
            a,
            Some(session),
            &format!("{base}/{response}"),
            &json!({"rating": "up"}),
        )
        .await;
        assert_eq!(refused.0, 404, "response {response}: {}", refused.1);
    }
    let other_chat = format!("/physician-state/feedback/{workspace}/no-such-chat");
    let refused = put(
        a,
        Some(session),
        &format!("{other_chat}/{first}"),
        &json!({"rating": "up", "details": details}),
    )
    .await;
    assert_eq!(refused.0, 404);
    let too_long = json!({"reasons": [], "text": "x".repeat(4001)});
    let refused = put(
        a,
        Some(session),
        &format!("{base}/{first}"),
        &json!({"rating": "up", "details": too_long}),
    )
    .await;
    assert_eq!(refused.0, 422);
    // Details without a rating can't be expressed.
    let refused = put(a, Some(session), &format!("{base}/{first}"), &details).await;
    assert_eq!(refused.0, 422, "{}", refused.1);
    // The split routes are gone.
    let (status, _) = support::get_json(a, &base, session).await;
    assert!(
        matches!(status, 404 | 405),
        "the chat read route answered {status}"
    );
    let refused = put(
        a,
        Some(session),
        &format!("{base}/{first}/rating"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(refused.0, 404, "{}", refused.1);

    // Without a session: 401. Off the panel: 404 and nothing written.
    let refused = put(
        a,
        None,
        &format!("{base}/{first}"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(refused.0, 401);
    let outsider = running.colleague(Membership::Denied).await;
    let refused = put(
        a,
        Some(&outsider),
        &format!("{base}/{first}"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(refused.0, 404);

    // A colleague with the patient on their panel sees none of the author's
    // feedback, and cannot rate a chat that is not theirs.
    let colleague = running.colleague(Membership::Allowed).await;
    let refused = put(
        b,
        Some(&colleague),
        &format!("{base}/{first}"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(refused.0, 404);
    let (status, theirs) = support::get_json(
        b,
        &format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot"),
        &colleague,
    )
    .await;
    assert_eq!(status, 200, "{theirs}");
    assert!(
        support::chat(&theirs["snapshot"])
            .transcript
            .iter()
            .all(|exchange| finished(exchange).1.is_none()),
        "a colleague read the author's feedback: {theirs}"
    );
    assert_eq!(
        rated_turns(a, session, workspace, chat).await[0],
        (json!(first), up.1)
    );
}

/// A feedback row the site can't read leaves the chat readable: the snapshot
/// still carries every turn and says feedback is unavailable, and the site
/// logs why. A save the store can't make answers 503 and logs why too.
#[tokio::test]
async fn unreadable_feedback_keeps_the_conversation() {
    let chat = "chat-feedback-unreadable";
    let Some(running) = Running::start(chat).await else {
        return;
    };
    let (a, session) = (&running.hosts[0], &running.session);
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let base = format!("/physician-state/feedback/{workspace}/{chat}");
    let first = running.responses[0].as_str();
    let rated = put(
        a,
        Some(session),
        &format!("{base}/{first}"),
        &json!({"rating": "up"}),
    )
    .await;
    assert_eq!(rated.0, 200, "{}\n{}", rated.1, running.logs());

    // Details that are not an object can't be read back as details.
    support::corrupt_feedback_details(&running.database.url).await;
    let snapshot_path = format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot");
    let (status, snapshot) = support::get_json(a, &snapshot_path, session).await;
    assert_eq!(status, 200, "{snapshot}\n{}", running.logs());
    println!("snapshot with an unreadable feedback row: {snapshot}");
    assert_eq!(
        snapshot["snapshot"]["feedback"], "unavailable",
        "{snapshot}"
    );
    assert_eq!(
        support::chat_texts(&snapshot["snapshot"]),
        [
            "Latest potassium?",
            "Potassium was 4.1 on Monday.",
            "And sodium?",
            "Sodium was 139 on Monday.",
        ]
    );
    assert!(
        support::chat(&snapshot["snapshot"])
            .transcript
            .iter()
            .all(|exchange| finished(exchange).1.is_none()),
        "{snapshot}"
    );
    let logs = running.logs();
    assert!(
        logs.contains("physician chat snapshot feedback read failed")
            && logs.contains("stored feedback details"),
        "the snapshot's feedback failure was not logged with its cause:\n{logs}"
    );

    support::remove_feedback_table(&running.database.url).await;
    let refused = put(
        a,
        Some(session),
        &format!("{base}/{first}"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(refused.0, 503, "{}", refused.1);
    let logs = running.logs();
    assert!(
        logs.contains("physician feedback save failed") && logs.contains("does not exist"),
        "the save's failure was not logged with its cause:\n{logs}"
    );
    println!(
        "site log:\n{}",
        logs.lines()
            .filter(|line| line.contains("failed"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Runs `physician-migrate feedback` against `url` with `extra` environment,
/// returning its exit success, stdout and stderr.
fn staff_read(url: &str, extra: &[(&str, &str)]) -> (bool, String, String) {
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_physician-migrate"));
    command
        .arg("feedback")
        .env("PHYSICIAN_DATABASE_URL", url)
        .env_remove("CLOUD_RUN_JOB");
    for (key, value) in extra {
        command.env(key, value);
    }
    let output = command.output().expect("physician-migrate runs");
    (
        output.status.success(),
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

/// Coda staff read every physician's feedback from the operator's terminal,
/// each row naming its response, author, revision and save time beside the response text.
/// Inside a Cloud Run job, where stdout reaches logs, it reads nothing; a
/// database it cannot reach says so with the cause.
#[tokio::test]
async fn staff_read_feedback_with_its_response_and_author() {
    let chat = "chat-feedback-staff";
    let Some(running) = Running::start(chat).await else {
        return;
    };
    let (a, session) = (&running.hosts[0], &running.session);
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let base = format!("/physician-state/feedback/{workspace}/{chat}");
    let [first, second] = [running.responses[0].as_str(), running.responses[1].as_str()];

    let (ok, out, err) = staff_read(&running.database.url, &[]);
    assert!(ok, "{err}");
    assert_eq!(out, "", "nothing rated yet");

    let rated = put(
        a,
        Some(session),
        &format!("{base}/{second}"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(rated.0, 200, "{}\n{}", rated.1, running.logs());
    let details = json!({"reasons": ["missing_context"], "text": "Sodium was rechecked Tuesday."});
    let saved = put(
        a,
        Some(session),
        &format!("{base}/{second}"),
        &json!({"rating": "down", "details": details}),
    )
    .await;
    assert_eq!(saved.0, 200, "{}", saved.1);
    let up = put(
        a,
        Some(session),
        &format!("{base}/{first}"),
        &json!({"rating": "up"}),
    )
    .await;
    assert_eq!(up.0, 200, "{}", up.1);

    let (ok, out, err) = staff_read(&running.database.url, &[]);
    assert!(ok, "{err}");
    println!("physician-migrate feedback:\n{out}");
    let rows: Vec<Value> = out
        .lines()
        .map(|line| serde_json::from_str(line).expect("one JSON object per line"))
        .collect();
    assert_eq!(rows.len(), 2, "{out}");
    for row in &rows {
        assert_eq!(row["practice"], physician_sample::STAGING_PRACTICE_ID);
        assert_eq!(row["physician"], "staging-physician", "the author: {row}");
        assert_eq!(row["workspace"], workspace);
        assert_eq!(row["chat"], chat);
        assert!(row["saved_at"].as_i64().is_some_and(|at| at > 0), "{row}");
    }
    // In the chat's order, each naming its response.
    assert_eq!(rows[0]["response"], json!(first));
    assert_eq!(rows[0]["rating"], "up");
    assert_eq!(rows[0]["revision"], 1);
    assert_eq!(rows[0]["response_text"], "Potassium was 4.1 on Monday.");
    assert!(rows[0]["details"].is_null() && rows[0]["reasons"].is_null());
    assert_eq!(rows[1]["response"], json!(second));
    assert_eq!(rows[1]["rating"], "down");
    assert_eq!(rows[1]["revision"], 2, "rated, then details: {}", rows[1]);
    assert_eq!(rows[1]["response_text"], "Sodium was 139 on Monday.");
    assert_eq!(rows[1]["reasons"], json!(["missing_context"]));
    assert_eq!(rows[1]["details"], "Sodium was rechecked Tuesday.");

    let (ok, out, err) = staff_read(
        &running.database.url,
        &[("CLOUD_RUN_JOB", "physician-migrate")],
    );
    assert!(!ok, "a Cloud Run job read feedback");
    assert_eq!(out, "", "a Cloud Run job printed clinical text");
    assert!(err.contains("never runs as a Cloud Run job"), "{err}");

    let unreachable = format!(
        "postgres://nobody:nothing@127.0.0.1:{}/none",
        support::free_port()
    );
    let (ok, out, err) = staff_read(&unreachable, &[]);
    assert!(!ok);
    assert_eq!(out, "");
    assert!(
        err.contains("feedback not read: the physician database did not connect")
            && err.contains("caused by:"),
        "the cause is lost: {err}"
    );
}

struct PostgresWorkspaceAuthority;

#[local_product::runtime::async_trait]
impl local_product::runtime::Authority for PostgresWorkspaceAuthority {
    async fn check(
        &mut self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &local_product::runtime::Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::check_runtime_authority(
            tx,
            session_hash.ok_or("missing session reference")?,
            &physician_auth::PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|cause| cause.to_string())
    }

    async fn lock_publication(
        &mut self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &local_product::runtime::Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::lock_runtime_authority(
            tx,
            session_hash.ok_or("missing session reference")?,
            &physician_auth::PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|cause| cause.to_string())
    }
}

#[tokio::test]
async fn staff_read_feedback_includes_concern_done_evaluation() {
    use local_product::physician_sources::{RecordDate, SourceRevision};
    use local_product::runtime::Store;
    use local_product::runtime::concerns::{
        CommandKey, ConcernDoneOutcome, ConcernId, ConcernUrgency, ExactRecordReference,
        RaiseConcern, SourceWork, WorkspaceAccess,
    };

    let chat = "chat-feedback-eval";
    let Some(running) = Running::start(chat).await else {
        return;
    };
    let store = Store::connect(&running.database.url)
        .await
        .expect("runtime store");
    let practice = physician_sample::STAGING_PRACTICE_ID;
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let session_hash = PostgresAuthStore::session_reference(&running.session);

    let lab_content =
        r#"{"resourceType":"Observation","id":"bmp-eval-1","date":"2026-11-11","potassium_mmol_L":5.9}"#
            .to_owned();
    let lab = SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "clinic-lab".into(),
        candidate_id: "bmp-eval-1".into(),
        revision: "r1".into(),
        record_type: "Observation".into(),
        dates: vec![RecordDate {
            name: "effectiveDate".into(),
            value: "2026-11-11".into(),
        }],
        sha256: local_product::sha256_hex(lab_content.as_bytes()),
        content: lab_content,
        readable_parts: vec![],
        provenance: json!({"system": "clinic-lab", "id": "bmp-eval-1"}),
        envelope: None,
    };
    store.record_source(&lab).await.expect("record source");

    let concern_id = ConcernId::new("eval-hyperkalemia").expect("concern id");
    store
        .raise_concern(
            &CommandKey::new("raise-eval-k-1").expect("command key"),
            &RaiseConcern {
                practice: practice.to_owned(),
                workspace: workspace.to_owned(),
                id: concern_id.clone(),
                urgency: ConcernUrgency::Now,
                reason: "Potassium 5.9 mmol/L while taking spironolactone.".into(),
                records: vec![ExactRecordReference::new("bmp-eval-1", "r1").expect("ref")],
                source_work: SourceWork::new("morning_briefing", "work-eval-1", "attempt-1")
                    .expect("source work"),
            },
        )
        .await
        .expect("raise concern");

    let access = WorkspaceAccess {
        practice,
        physician: "staging-physician",
        workspace,
        session_hash: &session_hash,
    };
    let done = store
        .mark_concern_done(
            &access,
            &CommandKey::new("done-eval-1").expect("done key"),
            &concern_id,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .expect("mark done");
    assert!(matches!(done, ConcernDoneOutcome::Cleared { .. }));

    // Seed a chat feedback row too so both streams appear in the same export.
    let first = running.responses[0].as_str();
    let saved = put(
        &running.hosts[0],
        Some(&running.session),
        &format!("/physician-state/feedback/{workspace}/{chat}/{first}"),
        &json!({"rating": "up"}),
    )
    .await;
    assert_eq!(saved.0, 200, "{}", saved.1);

    let (ok, out, err) = staff_read(&running.database.url, &[]);
    assert!(ok, "{err}");
    let rows: Vec<Value> = out
        .lines()
        .map(|line| serde_json::from_str(line).expect("one JSON object per line"))
        .collect();
    assert!(
        rows.iter()
            .any(|row| row["response"] == json!(first) && row["rating"] == "up"),
        "expected chat feedback row in {out}"
    );
    assert!(
        rows.iter().any(|row| {
            row["kind"] == "concern_done"
                && row["physician"] == "staging-physician"
                && row["workspace"] == workspace
        }),
        "expected concern_done evaluation row in {out}"
    );
}
