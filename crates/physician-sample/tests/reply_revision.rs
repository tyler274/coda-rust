//! Hosted reply revisions through two sites and a worker on one Postgres.
//! Only initial reply admission is seeded through the backend owner. Reads,
//! physician saves and revision commands use the real authenticated HTTP paths.
//! No model key, patient send, frontend draft creation, or staging is involved.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use harness::engine::MAX_FAILED_ATTEMPTS;
use harness::engine::recipe::Request;
use harness::engine::runs::RunOutcome;
use harness::engine::{EventKind, Item};
use local_product::physician_sources::SourceRevision;
use local_product::runtime::delivery::Delivery;
use local_product::runtime::reply::{Draft, DraftStatus};
use local_product::runtime::{AttemptStatus, Scope, Store};
use physician_auth::{AuthStore, Membership, PostgresAuthStore};
use serde_json::{Value, json};

#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
#[path = "scale/mod.rs"]
mod support;

const ORIGINAL: &str = "Thank you for your update. Please keep your appointment.";
const SAVED: &str = "Physician's newer saved wording.";
const INSTRUCTION: &str = "Make this reply shorter without changing the plan.";
const SNAPSHOT_SOURCE: &str = "source-snapshot";
const SNAPSHOT_CONTENT: &str = "Synthetic source observation from the retained snapshot.";

/// A synthetic source revision in the seeded workspace.
fn source(id: &str, revision: &str, content: &str) -> SourceRevision {
    SourceRevision {
        workspace: physician_sample::WORKSPACE_IDS[0].into(),
        source_id: "ehr".into(),
        candidate_id: id.into(),
        revision: revision.into(),
        record_type: "Observation".into(),
        dates: vec![],
        sha256: local_product::sha256_hex(content.as_bytes()),
        content: content.into(),
        readable_parts: vec![],
        provenance: json!({"synthetic": true, "native_record": id}),
    }
}

/// The source references named in a dispatched revision context.
fn references(request: &Request) -> Value {
    let Item::Message { text, .. } = &request.items[0] else {
        panic!("revision context must be a message")
    };
    serde_json::from_str::<Value>(text).expect("revision request context")["source_references"]
        .clone()
}

struct Running {
    database: support::Database,
    store: Store,
    valkey: support::Valkey,
    worker: support::Process,
    worker_host: String,
    stub: support::ResponsesStub,
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
        self.worker.kill_now();
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

impl Running {
    async fn start(label: &str, failures: u32, delay_ms: u64) -> Option<Self> {
        Self::start_with(label, failures, delay_ms, false).await
    }

    /// Like `start`; `synthetic` sets the explicit synthetic reply switch on
    /// both sites, so reply work opens without any direct draft seeding.
    async fn start_with(
        label: &str,
        failures: u32,
        delay_ms: u64,
        synthetic: bool,
    ) -> Option<Self> {
        Self::start_full(label, failures, delay_ms, synthetic, &[], &[]).await
    }

    /// Like `start_with`, with extra environment for both sites and the
    /// worker (the browser journey adds the page directory).
    async fn start_full(
        label: &str,
        failures: u32,
        delay_ms: u64,
        synthetic: bool,
        site_env: &[(&str, &str)],
        worker_env: &[(&str, &str)],
    ) -> Option<Self> {
        let database = support::Database::create(label).await?;
        support::staging_account(&database.url).await;
        let store = Store::connect(&database.url).await.expect("runtime store");
        let home = std::env::temp_dir().join(format!(
            "physician-revision-{label}-{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&home).expect("home");
        let valkey = support::Valkey::start().await;
        let redis = valkey.url();
        let worker_port = support::free_port();
        let worker_host = format!("127.0.0.1:{worker_port}");
        let stub = support::ResponsesStub::start(&home, failures, delay_ms, "[]").await;
        let mut env = vec![
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", &redis),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", &worker_host),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
            support::RETRY_NOW,
        ];
        env.extend_from_slice(worker_env);
        let mut worker = support::Process::spawn("worker", support::worker_bin(), &[], &env, &home);
        support::wait_ready(&mut worker, worker_port).await;
        let mut sites = Vec::new();
        let mut hosts = [String::new(), String::new()];
        for (index, host) in hosts.iter_mut().enumerate() {
            let port = support::free_port();
            *host = format!("127.0.0.1:{port}");
            // Both sites import the same retained snapshot into the shared
            // runtime source store; the second import changes nothing.
            let sources = vec![source(SNAPSHOT_SOURCE, "immutable-1", SNAPSHOT_CONTENT)];
            let snapshot = home.join(format!("sources-{index}.json"));
            std::fs::write(&snapshot, serde_json::to_vec(&sources).unwrap()).unwrap();
            let switch = if synthetic {
                physician_sample::WORKSPACE_IDS[0]
            } else {
                ""
            };
            let mut env = vec![
                ("PHYSICIAN_SYNTHETIC_REPLY", switch),
                ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
                ("PHYSICIAN_REDIS_URL", &redis),
                support::DURABLE,
                ("CODA_PHYSICIAN_SOURCE_SNAPSHOT", snapshot.to_str().unwrap()),
                ("PHYSICIAN_AUTH", "required"),
                ("PHYSICIAN_LISTEN_ADDR", host),
            ];
            env.extend_from_slice(site_env);
            let mut site = support::Process::spawn(
                &format!("site-{index}"),
                support::site_bin(),
                &[],
                &env,
                &home,
            );
            // Start sequentially, as each instance seeds the practice panel.
            support::wait_ready(&mut site, port).await;
            sites.push(site);
        }
        let session = support::sign_in(&hosts[0]).await;
        Some(Self {
            stub,
            database,
            store,
            valkey,
            worker,
            worker_host,
            sites,
            hosts,
            session,
            home,
        })
    }

    fn logs(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.sites[0].tail(),
            self.sites[1].tail(),
            self.worker.tail()
        )
    }

    async fn seed(&self) -> Scope {
        let scope = Scope {
            practice: physician_sample::STAGING_PRACTICE_ID.into(),
            physician: "staging-physician".into(),
            workspace: physician_sample::WORKSPACE_IDS[0].into(),
            chat: format!("reply-{}", uuid::Uuid::new_v4().simple()),
        };
        let hash = PostgresAuthStore::session_reference(&self.session);
        let draft = self
            .store
            .open_draft_authorized(
                &scope,
                ORIGINAL,
                &hash,
                &mut physician_sample::durable::RuntimeAuthority,
            )
            .await
            .unwrap()
            .expect("authenticated physician can open fixture reply");
        assert_eq!(draft.revision, 1);
        scope
    }

    async fn command(&self, site: usize, body: &Value, outcome: &str) {
        let (status, body) =
            support::post(&self.hosts[site], "/commands", Some(&self.session), body).await;
        if outcome == "rejected" {
            assert_eq!(status, 409, "{body}\n{}", self.logs());
        } else {
            assert_eq!(status, 200, "{body}\n{}", self.logs());
            assert_eq!(body["outcome"], outcome, "{body}\n{}", self.logs());
        }
    }

    /// Tells the worker about `delivery` again and gives it time to act.
    async fn announce_again(&self, delivery: &Delivery) {
        self.database.announce(delivery).await;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    async fn wait_draft(&self, scope: &Scope, done: impl Fn(&Draft) -> bool) -> Draft {
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            let draft = self
                .store
                .draft(scope)
                .await
                .unwrap()
                .expect("seeded draft");
            if done(&draft) {
                return draft;
            }
            assert!(
                Instant::now() < deadline,
                "draft did not reach expected state: {draft:?}\n{}",
                self.logs()
            );
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    }

    async fn read(&self, site: usize, scope: &Scope) -> (u16, Value) {
        support::get_json(&self.hosts[site], &path(scope), &self.session).await
    }

    async fn restart_worker(&mut self) {
        self.worker.kill_now();
        let redis = self.valkey.url();
        let env = [
            ("PHYSICIAN_DATABASE_URL", self.database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            support::RETRY_NOW,
            ("PHYSICIAN_WORKER_LISTEN_ADDR", self.worker_host.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", self.stub.port.as_str()),
        ];
        self.worker = support::Process::spawn(
            "worker-restarted",
            support::worker_bin(),
            &[],
            &env,
            &self.home,
        );
        let port = self
            .worker_host
            .rsplit(':')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        support::wait_ready(&mut self.worker, port).await;
    }

    async fn save(&self, scope: &Scope, revision: u64, text: &str) -> (u16, String) {
        let authorization = format!("Bearer {}", self.session);
        let bytes = serde_json::to_vec(&json!({"revision": revision, "text": text})).unwrap();
        support::exchange(
            &self.hosts[1],
            "PUT",
            &path(scope),
            &[
                ("authorization", &authorization),
                ("content-type", "application/json"),
            ],
            Some(&bytes),
        )
        .await
        .expect("save request")
    }
}

fn path(scope: &Scope) -> String {
    format!(
        "/physician-state/reply-drafts/{}/{}",
        scope.workspace, scope.chat
    )
}

fn revise(scope: &Scope, key: &str, expected: u64) -> Value {
    json!({
        "type": "reply.revise",
        "idempotency_key": key,
        "payload": {
            "patient_workspace_id": scope.workspace,
            "reply_work_id": scope.chat,
            "expected_draft_revision": expected,
            "instruction": INSTRUCTION,
            "physician_time_zone": "America/New_York"
        }
    })
}

/// Every field of two drafts is equal. Attempts compare through their saved
/// JSON, since their events have no equality of their own.
fn assert_same_draft(left: &Draft, right: &Draft) {
    let Draft {
        revision,
        text,
        restore_text,
        status,
        accepted,
        runs,
        base_text,
        sources,
        sources_version,
        allowance_after,
        attempts,
        session_hash,
        dispatch_after,
        failures,
        origin,
    } = left;
    assert_eq!(*revision, right.revision);
    assert_eq!(text, &right.text);
    assert_eq!(restore_text, &right.restore_text);
    assert_eq!(*status, right.status);
    assert_eq!(
        accepted.as_ref().map(|a| (&a.input, &a.binding, &a.skills)),
        right
            .accepted
            .as_ref()
            .map(|a| (&a.input, &a.binding, &a.skills))
    );
    assert_eq!(runs, &right.runs);
    assert_eq!(base_text, &right.base_text);
    assert_eq!(sources, &right.sources);
    assert_eq!(*sources_version, right.sources_version);
    assert_eq!(*allowance_after, right.allowance_after);
    assert_eq!(
        serde_json::to_value(attempts).unwrap(),
        serde_json::to_value(&right.attempts).unwrap()
    );
    assert_eq!(session_hash, &right.session_hash);
    assert_eq!(*dispatch_after, right.dispatch_after);
    assert_eq!(*failures, right.failures);
    assert_eq!(origin, &right.origin);
}

async fn dispatches(running: &Running, scope: &Scope, draft: &Draft) -> Vec<Request> {
    let messages: Vec<_> = running
        .store
        .revision_messages(scope, draft)
        .await
        .unwrap()
        .iter()
        .map(harness::engine::history::Message::source)
        .collect();
    let recipe = running
        .store
        .frozen_binding(scope, &draft.accepted.as_ref().unwrap().binding)
        .await
        .unwrap();
    draft
        .attempts
        .iter()
        .flat_map(|attempt| &attempt.events)
        .filter_map(|event| match &event.kind {
            EventKind::ProviderStarted { request, .. } => Some(
                recipe.request(
                    request
                        .restore(
                            draft.attempts.iter().flat_map(|attempt| &attempt.events),
                            &messages,
                        )
                        .unwrap()
                        .expect("saved request sources remain available"),
                ),
            ),
            _ => None,
        })
        .collect()
}

#[tokio::test]
async fn duplicate_acceptance_and_worker_retry_keep_one_input_and_pinned_sources() {
    let Some(running) = Running::start_full(
        "reply_retry",
        1,
        250,
        false,
        &[("CODA_BACKGROUND_MODEL", "accepted-site-model")],
        &[
            ("CODA_BACKGROUND_MODEL", "replacement-worker-model"),
            ("CODA_BACKGROUND_EFFORT", "low"),
            ("CODA_BACKGROUND_CONTEXT_WINDOW_TOKENS", "200000"),
            ("CODA_BACKGROUND_INPUT_LIMIT_TOKENS", "250000"),
            ("CODA_LOG_FORMAT", "json"),
        ],
    )
    .await
    else {
        return;
    };
    let scope = running.seed().await;
    let key = format!("revision-{}", uuid::Uuid::new_v4().simple());
    let command = revise(&scope, &key, 1);
    running.command(0, &command, "accepted").await;
    let accepted = running.store.draft(&scope).await.unwrap().unwrap();
    let input = accepted.accepted.clone().expect("accepted input").input;
    assert_eq!(input.key, key);
    assert_eq!(input.expected_revision, 1);
    assert_eq!(input.instruction, INSTRUCTION);
    assert_eq!(input.timezone, "America/New_York");
    assert_eq!(accepted.base_text, ORIGINAL);
    assert_eq!(
        accepted.sources,
        vec![source(SNAPSHOT_SOURCE, "immutable-1", SNAPSHOT_CONTENT)],
        "both sites imported one snapshot into one store"
    );
    // The first attempt dispatches on the accepted pin, then fails.
    let first = running
        .wait_draft(&scope, |draft| {
            draft
                .attempts
                .iter()
                .flat_map(|attempt| &attempt.events)
                .any(|event| matches!(event.kind, EventKind::ProviderStarted { .. }))
        })
        .await;
    assert_eq!(first.attempts[0].sources, accepted.sources);
    // A lab arrives, then the other site accepts the duplicate command.
    let lab = source("lab-arrived", "immutable-1", "Creatinine 2.4 mg/dL (high).");
    running.store.record_source(&lab).await.unwrap();
    running.command(1, &command, "accepted").await;
    let complete = running
        .wait_draft(&scope, |draft| draft.status == DraftStatus::Completed)
        .await;
    assert_eq!(
        complete.accepted.as_ref().map(|accepted| &accepted.input),
        Some(&input)
    );
    assert_eq!(
        complete
            .attempts
            .iter()
            .map(|attempt| sea_orm::ActiveEnum::to_value(&attempt.status))
            .collect::<Vec<_>>(),
        ["failed", "completed"]
    );
    assert!(
        complete
            .attempts
            .iter()
            .all(|attempt| attempt.input_id == input.key)
    );
    // The retry ran on the newest sources, not the original pin.
    let mut newest = accepted.sources.clone();
    newest.push(lab.clone());
    newest.sort_by(|a, b| a.candidate_id.cmp(&b.candidate_id));
    assert_eq!(complete.attempts[0].sources, accepted.sources);
    assert_eq!(complete.attempts[1].sources, newest);
    assert_ne!(complete.attempts[0].id, complete.attempts[1].id);
    let logs = running.worker.tail();
    let failure = logs
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .find(|event| event["event"] == "physician.worker.attempt_failed")
        .unwrap_or_else(|| panic!("missing revision failure event: {logs}"));
    assert_eq!(failure["coda.run_id"], complete.attempts[0].id);
    assert_eq!(failure["coda.job"], "physician_reply_revision");
    assert_eq!(failure["error.type"], "revision");
    for content in [ORIGINAL, INSTRUCTION, SNAPSHOT_CONTENT] {
        assert!(!logs.contains(content), "clinical content in worker logs");
    }
    assert_eq!(complete.sources, newest);
    assert!(complete.attempts[0].refreshes.is_empty());
    assert_eq!(complete.attempts[1].refreshes.len(), 1);
    assert_eq!(
        complete.attempts[1].refreshes[0].added,
        std::slice::from_ref(&lab)
    );
    assert_eq!(complete.revision, 2);
    assert_eq!(complete.text, support::FIXED_TEXT);
    assert_eq!(complete.restore_text.as_deref(), Some(ORIGINAL));
    let requests = dispatches(&running, &scope, &complete).await;
    assert_eq!(
        requests.len(),
        4,
        "failed opening, refreshed successor, read, finish"
    );
    assert_eq!(
        requests[0].model, "accepted-site-model",
        "the worker must use the recipe accepted by the site"
    );
    assert_eq!(requests[0].options["effort"], "max");
    assert_eq!(requests[0].context_window_tokens, 272_000);
    assert_eq!(requests[0].max_input_tokens, 922_000);
    assert!(requests[0].options.get("max_output_tokens").is_none());
    let mut original_recipe = requests[0].clone();
    original_recipe.items = requests[1].items.clone();
    assert_eq!(
        requests[1], original_recipe,
        "a successor changes only authorized source items"
    );
    let Item::Message { text, .. } = &requests[0].items[0] else {
        panic!("revision context must be a message")
    };
    let context: Value = serde_json::from_str(text).expect("revision request context");
    assert_eq!(context["saved_draft"], ORIGINAL);
    assert!(!context.as_object().unwrap().contains_key("instruction"));
    assert!(requests[0].items.iter().any(|item| matches!(item,
        Item::ProductMessage(message) if message.id == format!("{key}:instruction")
            && message.role == harness::engine::history::Role::User && message.text == INSTRUCTION
    )));
    assert_eq!(references(&requests[0]), json!([SNAPSHOT_SOURCE]));
    assert_eq!(references(&requests[1]), references(&requests[0]));
    assert!(requests[1].items.iter().any(|item| matches!(item,
        Item::Feedback { text } if serde_json::from_str::<Value>(text).is_ok_and(|feedback| feedback["outcome"] == "sources_changed" && feedback["added"] == json!([lab.candidate_id]))
    )), "the retained opening receives explicit current source feedback");
    let sent = running.stub.requests();
    assert_eq!(sent.len(), requests.len());
    for request in &sent {
        assert_eq!(request["model"], "accepted-site-model");
        assert_eq!(request["reasoning"]["effort"], "max");
    }
    assert!(has_outcome(&sent[2], "sources_unread"));
    assert!(has_read(&sent[3], &lab));

    let (status, read) = running.read(1, &scope).await;
    assert_eq!(status, 200, "{read}");
    assert_eq!(read["revision"], 2);
    assert_eq!(read["text"], support::FIXED_TEXT);
    assert_eq!(read["restore_text"], ORIGINAL);
    println!(
        "hosted revision readback {}",
        json!({"attempts": complete.attempts, "readback": read})
    );

    // Completion does not invalidate the receipt; neither a duplicate
    // command nor hearing of the settled revision again may create another
    // attempt or draft revision.
    running.command(1, &command, "accepted").await;
    let delivery = Delivery::reply_revision(scope.clone(), key);
    running.announce_again(&delivery).await;
    let unchanged = running.store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(unchanged.attempts.len(), 2);
    assert_eq!(unchanged.revision, 2);
    let mut collision = command.clone();
    collision["payload"]["instruction"] = json!("Different instruction with the same key");
    running.command(0, &collision, "rejected").await;
    assert!(
        running.store.read(&scope).await.unwrap().is_none(),
        "revision must not create chat work"
    );
}

/// Structured outputs at the real provider boundary, including source feedback.
fn result_values(request: &Value) -> impl Iterator<Item = Value> + '_ {
    request["input"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|item| {
            if item["type"] != "function_call_output" {
                return None;
            }
            let encoded = item["output"].as_str()?;
            serde_json::from_str(encoded).ok()
        })
}

fn has_outcome(request: &Value, outcome: &str) -> bool {
    result_values(request).any(|value| value["outcome"] == outcome)
}

fn has_read(request: &Value, source: &SourceRevision) -> bool {
    result_values(request).any(|value| {
        value["results"].as_array().is_some_and(|results| {
            results.iter().any(|result| {
                result["reference"] == source.candidate_id.as_str()
                    && result["revision"] == source.revision.as_str()
                    && result["content"] == source.content.as_str()
            })
        })
    })
}

#[tokio::test]
async fn physician_save_cancels_an_in_flight_revision_and_preserves_the_edit() {
    // The worker enters the Responses exchange and connects to this loopback
    // HTTPS proxy. Keeping the tunnel unanswered holds the provider pending
    // through the save without forwarding a request or using a real API key.
    let proxy = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", proxy.local_addr().unwrap());
    let Some(running) = Running::start_full(
        "reply_save",
        0,
        0,
        false,
        &[],
        &[
            ("PHYSICIAN_RESPONSES_STUB_PORT", ""),
            ("CODA_LOCAL_MODEL_EXECUTION", "api"),
            ("CODA_ALLOW_MODEL_CALLS", "1"),
            ("OPENAI_API_KEY", "synthetic"),
            ("HTTPS_PROXY", &url),
            ("NO_PROXY", "127.0.0.1,localhost"),
        ],
    )
    .await
    else {
        return;
    };
    let scope = running.seed().await;
    let command = revise(&scope, "revision-before-save", 1);
    running.command(0, &command, "accepted").await;
    let _pending_exchange = tokio::time::timeout(Duration::from_secs(60), async {
        use tokio::io::{AsyncBufReadExt, BufReader};
        let (stream, _) = proxy.accept().await.unwrap();
        let mut tunnel = BufReader::new(stream);
        let mut request = String::new();
        tunnel.read_line(&mut request).await.unwrap();
        assert!(
            request.starts_with("CONNECT api.openai.com:443 "),
            "{request}"
        );
        tunnel
    })
    .await
    .unwrap_or_else(|_| panic!("provider did not enter the exchange\n{}", running.logs()));
    let (status, body) = running.save(&scope, 1, SAVED).await;
    assert_eq!(status, 200, "{body}\n{}", running.logs());
    let (status, body) = running.save(&scope, 1, "stale overwrite").await;
    assert_eq!(status, 409, "{body}");
    // The in-flight provider must settle after the save and retain its failure.
    let late = running
        .wait_draft(&scope, |draft| {
            draft
                .attempts
                .iter()
                .flat_map(|attempt| &attempt.events)
                .any(|record| matches!(record.kind, EventKind::ProviderFailed { .. }))
        })
        .await;
    assert_eq!(late.status, DraftStatus::Cancelled);
    assert_eq!(late.revision, 2);
    assert_eq!(late.text, SAVED);
    assert_eq!(late.restore_text.as_deref(), Some(ORIGINAL));
    assert!(
        late.attempts
            .iter()
            .all(|attempt| attempt.status != AttemptStatus::Completed)
    );
    let delivery = Delivery::reply_revision(scope.clone(), "revision-before-save".into());
    running.announce_again(&delivery).await;
    let settled = running.store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(settled.attempts.len(), late.attempts.len());
    for site in 0..2 {
        let (status, read) = running.read(site, &scope).await;
        assert_eq!(status, 200, "{read}");
        assert_eq!(read["text"], SAVED);
        assert_eq!(read["revision"], 2);
    }
    running
        .command(0, &revise(&scope, "stale-revision", 1), "rejected")
        .await;
}

#[tokio::test]
async fn revision_capabilities_survive_a_window_and_worker_replacement_between_attempts() {
    let Some(mut running) = Running::start("reply_capabilities", 0, 500).await else {
        return;
    };
    let scope = running.seed().await;
    running
        .command(0, &revise(&scope, "revision-before", 1), "accepted")
        .await;
    running
        .wait_draft(&scope, |draft| draft.status == DraftStatus::Completed)
        .await;
    let note = "Continue this revision using the saved skill result and earlier reply. Do not send anything.";
    running.stub.tool_rounds(&json!([
        [
            {"name":"skill_view","arguments":{"name":"patient-communication"}},
            {"name":"list_conversation_history","arguments":{"order":"oldest_first"}},
            {"name":"search_conversation_history","arguments":{"text":INSTRUCTION}},
            {"name":"read_conversation_message","arguments":{"id":"revision-before:instruction"}},
            {"name":"read_patient_sources","arguments":{"references":[SNAPSHOT_SOURCE],"max_result_bytes":65536}}
        ],
        [{"name":"save_working_note","arguments":{"text":note,"fresh_window":true}}],
        [
            {"name":"read_working_note","arguments":{"reference":"revision-capabilities:fixed-2-0"}},
            {"name":"read_tool_result","arguments":{"call_id":"fixed-1-0","offset":0}},
            {"name":"read_conversation_message","arguments":{"id":"revision-before:reply"}}
        ]
    ]));
    running
        .command(0, &revise(&scope, "revision-capabilities", 2), "accepted")
        .await;
    let crossed = running
        .wait_draft(&scope, |draft| {
            draft
                .attempts
                .iter()
                .flat_map(|a| &a.events)
                .any(|event| matches!(event.kind, EventKind::WindowOpened { .. }))
        })
        .await;
    assert_eq!(
        crossed.text,
        support::FIXED_TEXT,
        "a working note must not replace the saved draft"
    );
    running.stub.fail_next(1);
    running
        .wait_draft(&scope, |draft| draft.status == DraftStatus::Pending)
        .await;
    running.restart_worker().await;
    let done = running
        .wait_draft(&scope, |draft| draft.status == DraftStatus::Completed)
        .await;
    assert_eq!(done.revision, 3);
    assert_eq!(done.attempts.len(), 2);
    assert_eq!(done.attempts[0].status, AttemptStatus::Failed);
    assert_eq!(done.attempts[1].status, AttemptStatus::Completed);
    let events: Vec<_> = done
        .attempts
        .iter()
        .flat_map(|attempt| &attempt.events)
        .collect();
    for id in [
        "fixed-1-0",
        "fixed-1-1",
        "fixed-1-2",
        "fixed-1-3",
        "fixed-1-4",
        "fixed-2-0",
    ] {
        assert_eq!(
            events
                .iter()
                .filter(
                    |event| matches!(&event.kind, EventKind::ToolStarted(call) if call.id == id)
                )
                .count(),
            1,
            "completed call {id} must not replay"
        );
    }
    let messages: Vec<_> = running
        .store
        .revision_messages(&scope, &done)
        .await
        .unwrap()
        .iter()
        .map(harness::engine::history::Message::source)
        .collect();
    let records = running.store.revision_records(&scope, &done).await.unwrap();
    let result = |id: &str| {
        let output = harness::engine::results::resolve(records.iter(), id, &messages)
            .unwrap()
            .unwrap_or_else(|| panic!("missing successful {id}"));
        serde_json::from_str::<Value>(&output).unwrap()
    };
    assert_eq!(result("fixed-1-3")["items"][0]["text"], INSTRUCTION);
    assert_eq!(result("fixed-3-0")["text"], note);
    assert_eq!(result("fixed-3-2")["items"][0]["text"], support::FIXED_TEXT);
    assert!(
        result("fixed-1-1")["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"] == "revision-before:reply")
    );
    assert_eq!(result("fixed-1-2")["items"].as_array().unwrap().len(), 2);
    let skill = result("fixed-1-0");
    let page = result("fixed-3-1");
    assert_eq!(page["source_call_id"], "fixed-1-0");
    assert_eq!(
        serde_json::from_str::<Value>(page["text"].as_str().unwrap()).unwrap(),
        skill
    );
    for site in 0..2 {
        let (status, read) = running.read(site, &scope).await;
        assert_eq!(status, 200);
        assert_eq!(read["text"], support::FIXED_TEXT);
        assert_eq!(read["revision"], 3);
    }
}

#[tokio::test]
async fn a_failed_revision_retries_over_http_as_a_linked_run() {
    for (ending, failures) in [("failed", MAX_FAILED_ATTEMPTS), ("held", 0)] {
        let Some(running) = Running::start_with("reply_retry", failures, 0, true).await else {
            return;
        };
        if ending == "held" {
            let calls = harness::engine::RunLimits::default().max_calls;
            let rounds: Vec<_> = (0..calls)
                .map(|_| {
                    json!([
                        {"name":"skill_view","arguments":{"name":"patient-communication"}}
                    ])
                })
                .collect();
            running.stub.tool_rounds(&json!(rounds));
        }
        let scope = synthetic_scope();
        let initial = running.listed(0, &running.session).await.unwrap()["draft_text"].clone();
        running
            .command(0, &revise(&scope, "revision-fails", 1), "accepted")
            .await;
        let failed = running
            .wait_draft(&scope, |draft| {
                sea_orm::ActiveEnum::to_value(&draft.status) == ending
            })
            .await;
        assert_eq!(
            failed.attempts.len(),
            if ending == "held" {
                1
            } else {
                failures as usize
            }
        );
        if ending == "held" {
            assert!(matches!(
                failed.runs.current().unwrap().outcome,
                Some(RunOutcome::Held {
                    reason: harness::engine::runs::HoldReason::Limit
                })
            ));
            running.stub.tool_rounds(&json!([]));
        }
        let failed_run = failed.runs.current().unwrap().id.to_owned();
        let resume = json!({
            "type": "work.resume",
            "idempotency_key": "resume-failed-revision",
            "payload": {
                "patient_workspace_id": scope.workspace,
                "chat_id": scope.chat,
                "work_item_id": scope.chat,
                "expected_run_id": failed_run,
            }
        });
        for site in 0..2 {
            let (status, read) = running.read(site, &scope).await;
            assert_eq!(status, 200, "{read}");
            assert_eq!(read["revision"], 1);
            assert_eq!(read["text"], initial);
            assert_eq!(
                read["revision_run"],
                json!({"run_id": failed_run, "status": ending, "resume_target": {"work_item_id": scope.chat, "expected_run_id": failed_run}})
            );
        }
        for site in [0, 1, 0] {
            let (status, body) = support::post(
                &running.hosts[site],
                "/commands",
                Some(&running.session),
                &resume,
            )
            .await;
            assert_eq!(status, 200, "{body}\n{}", running.logs());
            assert_eq!(
                body["outcome"], "accepted",
                "failed revision must accept Try again: {body}"
            );
        }
        let done = running
            .wait_draft(&scope, |draft| {
                draft.runs.current().is_some_and(|run| run.id != failed_run)
                    && matches!(
                        draft.status,
                        DraftStatus::Completed | DraftStatus::Held | DraftStatus::Failed
                    )
            })
            .await;
        assert_eq!(
            done.status,
            DraftStatus::Completed,
            "Try again must give the worker fresh allowance"
        );
        assert_eq!(done.revision, 2);
        assert_eq!(done.text, support::FIXED_TEXT);
        assert_eq!(done.failures, 0);
        assert_eq!(done.attempts.len(), failed.attempts.len() + 1);
        assert_eq!(
            done.runs.current().unwrap().continues,
            Some(failed_run.as_str())
        );
        assert_ne!(done.runs.current().unwrap().id, failed_run);
        assert_eq!(
            &done.runs.events()[..failed.runs.events().len()],
            failed.runs.events()
        );
        for (before, after) in failed.attempts.iter().zip(&done.attempts) {
            assert_eq!(
                serde_json::to_value(before).unwrap(),
                serde_json::to_value(after).unwrap()
            );
        }
        assert_eq!(
            done.accepted.as_ref().map(|accepted| &accepted.binding),
            failed.accepted.as_ref().map(|accepted| &accepted.binding),
            "Try again preserves the accepted recipe"
        );
        assert!(
            running.store.read(&scope).await.unwrap().is_none(),
            "revision retry must not create chat work"
        );
        let mut stale = resume.clone();
        for run_id in [failed_run.as_str(), done.runs.current().unwrap().id] {
            stale["idempotency_key"] = json!(format!("refuse-{run_id}"));
            stale["payload"]["expected_run_id"] = json!(run_id);
            let (status, body) = support::post(
                &running.hosts[1],
                "/commands",
                Some(&running.session),
                &stale,
            )
            .await;
            assert_eq!(status, 200, "{body}");
            assert_eq!(body["outcome"], "rejected", "{body}");
        }
        assert_same_draft(&running.store.draft(&scope).await.unwrap().unwrap(), &done);
        // A physician save supersedes failed work. Both read paths must stop
        // offering the old run even though its historical status remains failed.
        running.stub.fail_next(MAX_FAILED_ATTEMPTS);
        running
            .command(0, &revise(&scope, "revision-edited", 2), "accepted")
            .await;
        running
            .wait_draft(&scope, |draft| draft.status == DraftStatus::Failed)
            .await;
        assert!(running.read(0, &scope).await.1["revision_run"]["resume_target"].is_object());
        let (status, saved) = running.save(&scope, 2, SAVED).await;
        assert_eq!(status, 200, "{saved}");
        for site in 0..2 {
            let read = running.read(site, &scope).await.1;
            assert!(
                read["revision_run"]["resume_target"].is_null(),
                "saved edits cannot retry old work: {read}"
            );
            let listed = running.listed(site, &running.session).await.unwrap();
            assert!(
                listed["revision_run"]["resume_target"].is_null(),
                "saved edits cannot offer retry on the card: {listed}"
            );
        }
    }
}

#[tokio::test]
async fn missing_and_other_physician_drafts_are_not_created_or_exposed() {
    let Some(running) = Running::start("reply_isolation", 0, 0).await else {
        return;
    };
    let scope = running.seed().await;
    let mut missing = scope.clone();
    missing.chat = "not-admitted-reply".into();
    let (status, _) = running.read(1, &missing).await;
    assert_eq!(status, 404);
    running
        .command(0, &revise(&missing, "missing-revision", 1), "rejected")
        .await;
    let (status, _) = running.save(&missing, 1, "cannot create reply work").await;
    assert_eq!(status, 409);
    assert!(running.store.draft(&missing).await.unwrap().is_none());

    // Another physician is allowed to see this patient, but not this
    // physician's reply draft. Authenticate through the real auth owner.
    let auth = PostgresAuthStore::connect_without_migrations(&running.database.url)
        .await
        .unwrap();
    let email = "other-revision-physician@example.test";
    auth.ensure_account(
        &scope.practice,
        "other-revision-physician",
        email,
        "Other physician",
        false,
        support::unix_now(),
    )
    .await
    .unwrap();
    let token = auth
        .begin_sign_in(email, support::unix_now())
        .await
        .unwrap();
    let (identity, session) = auth
        .complete_sign_in(email, &token, support::unix_now())
        .await
        .unwrap();
    auth.set_membership(&identity, &scope.workspace, Membership::Allowed)
        .await
        .unwrap();
    let (status, _) = support::get_json(&running.hosts[1], &path(&scope), &session).await;
    assert_eq!(status, 404);
    let (status, body) = support::post(
        &running.hosts[1],
        "/commands",
        Some(&session),
        &revise(&scope, "foreign-revision", 1),
    )
    .await;
    assert_eq!(status, 409, "{body}");
    let authorization = format!("Bearer {session}");
    let bytes = serde_json::to_vec(&json!({"revision":1,"text":"foreign overwrite"})).unwrap();
    let (status, _) = support::exchange(
        &running.hosts[1],
        "PUT",
        &path(&scope),
        &[
            ("authorization", &authorization),
            ("content-type", "application/json"),
        ],
        Some(&bytes),
    )
    .await
    .unwrap();
    assert_eq!(status, 409);
    let mut other_scope = scope.clone();
    other_scope.physician = identity.physician_id;
    assert!(running.store.draft(&other_scope).await.unwrap().is_none());
    let original = running.store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(original.revision, 1);
    assert_eq!(original.text, ORIGINAL);
    assert!(original.accepted.is_none());

    let (status, _) = support::exchange(&running.hosts[0], "GET", &path(&scope), &[], None)
        .await
        .unwrap();
    assert_eq!(status, 401);
}

/// The explicitly enabled synthetic reply's stable server-owned identity.
const SYNTHETIC_REPLY: &str = "synthetic-reply-medication-question";

fn workspace_path() -> String {
    format!("/patient-workspaces/{}", physician_sample::WORKSPACE_IDS[0])
}

fn synthetic_scope() -> Scope {
    Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.into(),
        physician: "staging-physician".into(),
        workspace: physician_sample::WORKSPACE_IDS[0].into(),
        chat: SYNTHETIC_REPLY.into(),
    }
}

impl Running {
    /// The synthetic reply as the workspace projection on `site` lists it.
    async fn listed(&self, site: usize, session: &str) -> Option<Value> {
        let (status, body) = support::get_json(&self.hosts[site], &workspace_path(), session).await;
        assert_eq!(status, 200, "{body}\n{}", self.logs());
        body["replies"]
            .as_array()
            .and_then(|replies| {
                replies
                    .iter()
                    .find(|reply| reply["reply_work_id"] == SYNTHETIC_REPLY)
            })
            .cloned()
    }
}

#[tokio::test]
async fn the_synthetic_switch_off_opens_no_reply_work() {
    let Some(running) = Running::start("reply_synthetic_off", 0, 0).await else {
        return;
    };
    for site in 0..2 {
        assert!(running.listed(site, &running.session).await.is_none());
    }
    assert!(
        running
            .store
            .draft(&synthetic_scope())
            .await
            .unwrap()
            .is_none()
    );
    // Without the source, PUT and revise still refuse the identifier.
    let (status, _) = running.save(&synthetic_scope(), 0, "invented").await;
    assert_eq!(status, 409);
    running
        .command(
            0,
            &revise(&synthetic_scope(), "synthetic-off", 1),
            "rejected",
        )
        .await;
    assert!(
        running
            .store
            .draft(&synthetic_scope())
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn the_synthetic_reply_opens_once_and_revises_from_the_displayed_revision() {
    let Some(running) = Running::start_with("reply_synthetic", 0, 400, true).await else {
        return;
    };
    let scope = synthetic_scope();
    // Concurrent first loads on both sites open revision 1 exactly once.
    let (a, b) = tokio::join!(
        running.listed(0, &running.session),
        running.listed(1, &running.session)
    );
    let (a, b) = (a.expect("reply on A"), b.expect("reply on B"));
    assert_eq!(a["draft_revision"], 1, "{a}");
    assert_eq!(b["draft_revision"], 1, "{b}");
    assert_eq!(a["draft_text"], b["draft_text"]);
    assert_eq!(a["revision_run"], Value::Null);
    let opened = running.store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(opened.revision, 1);
    assert!(
        running
            .store
            .earlier_revisions(&scope)
            .await
            .unwrap()
            .is_empty()
    );

    // A physician save on B; reopening on A keeps the edit and its revision.
    let (status, body) = running.save(&scope, 1, SAVED).await;
    assert_eq!(status, 200, "{body}");
    let shown = running.listed(0, &running.session).await.unwrap();
    assert_eq!(shown["draft_text"], SAVED);
    assert_eq!(shown["draft_revision"], 2);

    // A revision naming the displayed revision is accepted and shows pending
    // or active; one naming the stale first revision is refused.
    running
        .command(0, &revise(&scope, "synthetic-stale", 1), "rejected")
        .await;
    let displayed = shown["draft_revision"].as_u64().unwrap();
    running
        .command(
            1,
            &revise(&scope, "synthetic-revise", displayed),
            "accepted",
        )
        .await;
    let accepted = running.store.draft(&scope).await.unwrap().unwrap();
    let run_id = accepted.runs.current().unwrap().id.to_owned();
    let running_now = running.listed(0, &running.session).await.unwrap();
    assert_eq!(running_now["revision_run"]["run_id"], run_id);
    assert!(
        matches!(
            running_now["revision_run"]["status"].as_str(),
            Some("pending" | "active" | "completed")
        ),
        "{running_now}"
    );

    // The worker's wording replaces the draft; both sites project it.
    running
        .wait_draft(&scope, |draft| draft.status == DraftStatus::Completed)
        .await;
    for site in 0..2 {
        let done = running.listed(site, &running.session).await.unwrap();
        assert_eq!(done["draft_text"], support::FIXED_TEXT, "{done}");
        assert_eq!(done["draft_revision"], 3);
        assert_eq!(
            done["revision_run"],
            json!({"run_id": run_id, "status": "completed", "resume_target": null})
        );
    }
    // Opening again never resets the revised draft or its history.
    let again = running.store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(again.revision, 3);
    assert_eq!(again.text, support::FIXED_TEXT);
}

#[tokio::test]
async fn synthetic_reply_opening_refuses_other_physicians_and_revoked_authority() {
    let Some(running) = Running::start_with("reply_synthetic_authority", 0, 0, true).await else {
        return;
    };
    let auth = PostgresAuthStore::connect_without_migrations(&running.database.url)
        .await
        .unwrap();
    let email = "synthetic-colleague@example.test";
    auth.ensure_account(
        physician_sample::STAGING_PRACTICE_ID,
        "synthetic-colleague",
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
    let (identity, session) = auth
        .complete_sign_in(email, &token, support::unix_now())
        .await
        .unwrap();
    let mut theirs = synthetic_scope();
    theirs.physician = identity.physician_id.clone();

    // Demographic-only access (same clinic, not on the panel) opens nothing.
    auth.set_membership(&identity, &theirs.workspace, Membership::Denied)
        .await
        .unwrap();
    let (status, body) = support::get_json(&running.hosts[1], &workspace_path(), &session).await;
    assert_eq!(status, 200, "{body}");
    assert_eq!(body["membership"], "clinic_demographic");
    assert!(
        body["replies"].as_array().is_none_or(Vec::is_empty),
        "{body}"
    );
    assert!(running.store.draft(&theirs).await.unwrap().is_none());

    // On the panel the colleague gets their own draft, never the physician's.
    auth.set_membership(&identity, &theirs.workspace, Membership::Allowed)
        .await
        .unwrap();
    let mine = running.listed(0, &running.session).await.unwrap();
    let (status, _) = running.save(&synthetic_scope(), 1, SAVED).await;
    assert_eq!(status, 200);
    let (status, body) = support::get_json(&running.hosts[1], &workspace_path(), &session).await;
    assert_eq!(status, 200, "{body}");
    let listed = body["replies"]
        .as_array()
        .and_then(|r| r.iter().find(|r| r["reply_work_id"] == SYNTHETIC_REPLY))
        .cloned()
        .expect("colleague's own synthetic reply");
    assert_eq!(listed["draft_text"], mine["draft_text"]);
    assert_eq!(listed["draft_revision"], 1);
    assert_eq!(
        running
            .store
            .draft(&synthetic_scope())
            .await
            .unwrap()
            .unwrap()
            .text,
        SAVED
    );

    // A signed-out session can no longer open, read or project the reply.
    let (other_identity, other_session) = {
        let email = "synthetic-revoked@example.test";
        auth.ensure_account(
            physician_sample::STAGING_PRACTICE_ID,
            "synthetic-revoked",
            email,
            "Revoked",
            false,
            support::unix_now(),
        )
        .await
        .unwrap();
        let token = auth
            .begin_sign_in(email, support::unix_now())
            .await
            .unwrap();
        auth.complete_sign_in(email, &token, support::unix_now())
            .await
            .unwrap()
    };
    auth.set_membership(&other_identity, &theirs.workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.end_session(&other_session).await.unwrap();
    let (status, _) = support::get_json(&running.hosts[0], &workspace_path(), &other_session).await;
    assert_eq!(status, 401);
    let mut revoked = synthetic_scope();
    revoked.physician = other_identity.physician_id;
    assert!(running.store.draft(&revoked).await.unwrap().is_none());
}

/// The synthetic reply card as the page on `browser` shows it: its draft
/// revision, latest revision status, draft wording and status words.
fn card(browser: &mut devtools::Browser) -> Option<(u64, String, String, String)> {
    let value = browser.page().evaluate(
        "(() => { const c = document.querySelector('article.reply-review'); \
         if (!c) return null; \
         return [Number(c.dataset.draftRevision), c.dataset.revisionStatus || '', \
         c.innerText, c.querySelector('[data-reply-revision]')?.textContent ?? '']; })()",
    );
    let row = value.as_array()?;
    Some((
        row[0].as_u64()?,
        row[1].as_str()?.to_owned(),
        row[2].as_str()?.to_owned(),
        row[3].as_str()?.to_owned(),
    ))
}

fn capture_reply(browser: &mut devtools::Browser, name: &str) {
    use base64::Engine;
    let Ok(folder) = std::env::var("PHYSICIAN_PROOF_SHOTS") else {
        return;
    };
    let shot = browser
        .page()
        .call_session("Page.captureScreenshot", json!({"format":"png"}));
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(shot["result"]["data"].as_str().unwrap())
        .unwrap();
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(
        std::path::Path::new(&folder).join(format!("{name}.png")),
        bytes,
    )
    .unwrap();
}

/// Waits up to `seconds` until the card satisfies `done`, and returns it.
fn wait_card(
    browser: &mut devtools::Browser,
    seconds: u64,
    what: &str,
    done: impl Fn(&(u64, String, String, String)) -> bool,
) -> (u64, String, String, String) {
    let deadline = Instant::now() + Duration::from_secs(seconds);
    loop {
        let shown = card(browser);
        if let Some(shown) = shown.as_ref().filter(|shown| done(shown)) {
            return shown.clone();
        }
        assert!(
            Instant::now() < deadline,
            "the reply card never showed {what}: {shown:?}\n{}",
            browser.body_text().unwrap_or_default()
        );
        std::thread::sleep(Duration::from_millis(200));
    }
}

/// Asks Coda to revise from the page: types into the revise field and
/// presses its button, which names the revision on screen.
fn ask_from_page(browser: &mut devtools::Browser, instruction: &str) {
    browser.type_into(".reply-review-instruction input", instruction);
    let deadline = Instant::now() + Duration::from_secs(5);
    while browser
        .page()
        .evaluate("!!document.querySelector('button[data-reply-action=\"revise\"]')")
        .as_bool()
        != Some(true)
    {
        assert!(Instant::now() < deadline, "no Ask Coda to revise button");
        std::thread::sleep(Duration::from_millis(50));
    }
    browser.click("button[data-reply-action=\"revise\"]");
}

/// Signs the browser in on `host` and opens the synthetic reply's workspace.
fn open_workspace(browser: &mut devtools::Browser, host: &str) {
    let url = format!(
        "http://{host}/?patient={}",
        physician_sample::WORKSPACE_IDS[0]
    );
    browser.text_until(&url, "Sign in to Coda");
    browser.type_into(r#"input[type="email"]"#, "physician@codahealth.test");
    browser.click(".control-primary");
    browser.wait_for("Records as of");
}

/// The real hosted browser journey for the synthetic reply: two site
/// instances and the worker; nothing seeds the draft.
/// The page shows revision 1, asks from the displayed revision, shows pending
/// and then the worker's wording on both sites; a stale revision is refused
/// and the current draft shown; a failed revision says so and keeps the
/// draft; and a Valkey restart (missed notifications) still converges.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification on the scale stack; run explicitly with --ignored"]
async fn the_synthetic_reply_revises_in_the_browser_across_two_sites() {
    let page = support::page_dir();
    let page = page.to_str().unwrap().to_owned();
    let Some(mut running) = Running::start_full(
        "reply_synthetic_browser",
        0,
        1_500,
        true,
        &[("PHYSICIAN_SITE_DIR", &page)],
        &[],
    )
    .await
    else {
        return;
    };
    let scope = synthetic_scope();
    let hosts = running.hosts.clone();
    let mut a = devtools::Browser::launch();
    a.resize(devtools::DESKTOP);
    open_workspace(&mut a, &hosts[0]);
    let mut b = devtools::Browser::launch();
    b.resize(devtools::DESKTOP);
    open_workspace(&mut b, &hosts[1]);

    // Both sites show the opened first draft at revision 1, with no run yet.
    for browser in [&mut a, &mut b] {
        let (revision, status, text, _) =
            wait_card(browser, 30, "the first draft", |(r, _, _, _)| *r == 1);
        assert_eq!(status, "");
        assert!(text.contains("Please keep taking your medicines"), "{text}");
        assert!(text.contains("Replying in"), "{text}");
        let _ = revision;
    }
    // The synthetic reply offers no close/send lifecycle action.
    assert_eq!(
        a.page()
            .evaluate("!!document.querySelector('[data-reply-action=\"no-reply-needed\"]')"),
        json!(false),
        "the synthetic reply offered an unsupported close action"
    );

    // Ask on A from revision 1: pending/active shows, then the worker's
    // wording lands at revision 2 on both sites, refreshed by the
    // worker's workspace_changed announcement.
    ask_from_page(&mut a, "Make this reply shorter.");
    let (_, status, _, words) = wait_card(&mut a, 15, "a running revision", |(_, s, _, _)| {
        matches!(s.as_str(), "pending" | "active")
    });
    assert!(
        words == "Coda will revise this reply shortly." || words == "Coda is revising this reply.",
        "{status}: {words}"
    );
    let input = running
        .wait_draft(&scope, |draft| draft.accepted.is_some())
        .await
        .accepted
        .unwrap()
        .input;
    assert_eq!(input.expected_revision, 1);
    for browser in [&mut a, &mut b] {
        let (revision, status, text, words) =
            wait_card(browser, 30, "the worker's wording", |(r, s, _, _)| {
                *r == 2 && s == "completed"
            });
        assert_eq!(revision, 2);
        assert!(text.contains(support::FIXED_TEXT), "{status}: {text}");
        assert_eq!(words, "Coda revised this reply.");
    }

    // A physician save on site B (revision 3) while A still shows revision 2:
    // A's ask names revision 2, is refused, and A shows the current draft.
    let (status, body) = running.save(&scope, 2, SAVED).await;
    assert_eq!(status, 200, "{body}");
    // A raw-HTTP save announces nothing to the page, so A still shows 2.
    assert_eq!(card(&mut a).unwrap().0, 2);
    ask_from_page(&mut a, "Use plainer words.");
    a.wait_for("Coda couldn’t take that revision.");
    let (revision, _, text, _) =
        wait_card(&mut a, 30, "the newer saved draft", |(r, _, _, _)| *r == 3);
    assert_eq!(revision, 3);
    assert!(text.contains(SAVED), "{text}");
    assert_eq!(
        running.store.draft(&scope).await.unwrap().unwrap().revision,
        3,
        "a stale ask changed the draft"
    );

    // A revision that fails every attempt: B shows failed and keeps the
    // saved wording. Valkey restarts meanwhile, so workspace_changed may be
    // missed; reconnect resync and bounded reconciliation still converge.
    // The save above went over raw HTTP, which no page hears; B reloads to
    // show revision 3 before asking.
    b.text_until(
        &format!(
            "http://{}/?patient={}",
            hosts[1],
            physician_sample::WORKSPACE_IDS[0]
        ),
        "Records as of",
    );
    wait_card(&mut b, 30, "revision 3 on B", |(r, _, _, _)| *r == 3);
    running.stub.fail_next(9);
    ask_from_page(&mut b, "Rephrase it");
    wait_card(
        &mut b,
        15,
        "the failing revision running",
        |(_, s, _, _)| matches!(s.as_str(), "pending" | "active"),
    );
    running.valkey.restart().await;
    for browser in [&mut a, &mut b] {
        let (revision, _, text, words) =
            wait_card(browser, 120, "the failed revision", |(_, s, _, _)| {
                s == "failed"
            });
        assert_eq!(revision, 3);
        assert!(text.contains(SAVED), "{text}");
        assert_eq!(
            words,
            "Coda couldn’t revise this reply. The saved draft is unchanged."
        );
    }

    // Reload on A: the projection, not page memory, still shows the state.
    a.text_until(
        &format!(
            "http://{}/?patient={}",
            hosts[0],
            physician_sample::WORKSPACE_IDS[0]
        ),
        "Records as of",
    );
    let (revision, status, text, _) =
        wait_card(&mut a, 30, "the reloaded card", |(r, _, _, _)| *r == 3);
    assert_eq!((revision, status.as_str()), (3, "failed"));
    assert!(text.contains(SAVED));
    let previous = running
        .store
        .draft(&scope)
        .await
        .unwrap()
        .unwrap()
        .runs
        .current()
        .unwrap()
        .id
        .to_owned();
    assert_eq!(
        a.page()
            .evaluate("!!document.querySelector('button[data-reply-action=\"try-again\"]')"),
        json!(true)
    );
    capture_reply(&mut a, "failed-revision-try-again");
    running.stub.fail_next(0);
    a.click("button[data-reply-action=\"try-again\"]");
    for browser in [&mut a, &mut b] {
        let (revision, _, text, _) =
            wait_card(browser, 30, "the retried revision", |(r, s, _, _)| {
                *r == 4 && s == "completed"
            });
        assert_eq!(revision, 4);
        assert!(text.contains(support::FIXED_TEXT));
        assert_eq!(
            browser
                .page()
                .evaluate("!!document.querySelector('button[data-reply-action=\"try-again\"]')"),
            json!(false)
        );
    }
    let done = running.store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(
        done.runs.current().unwrap().continues,
        Some(previous.as_str())
    );
    capture_reply(&mut a, "retried-revision-completed");
}
