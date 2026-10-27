//! Hosted reply revisions through two sites, a worker, and the Pub/Sub emulator.
//! Only initial reply admission is seeded through the backend owner. Reads,
//! physician saves and revision commands use the real authenticated HTTP paths.
//! No model key, patient send, frontend draft creation, or staging is involved.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use harness::execution::{Item, Record, Request};
use local_product::runtime::delivery::Delivery;
use local_product::runtime::reply::{Draft, RevisionAttempt};
use local_product::runtime::{MAX_FAILED_ATTEMPTS, Scope, Store};
use physician_auth::{AuthStore, Membership, PostgresAuthStore};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

const ORIGINAL: &str = "Thank you for your update. Please keep your appointment.";
const SAVED: &str = "Physician's newer saved wording.";
const INSTRUCTION: &str = "Make this reply shorter without changing the plan.";

struct Running {
    database: support::Database,
    store: Store,
    emulator: support::Emulator,
    _valkey: support::Valkey,
    worker: support::Process,
    sites: Vec<support::Process>,
    hosts: [String; 2],
    worker_host: String,
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
        let emulator = support::Emulator::start().await;
        emulator.provision(worker_port).await;
        let worker_host = format!("127.0.0.1:{worker_port}");
        let failures = failures.to_string();
        let delay = delay_ms.to_string();
        let mut worker = support::Process::spawn(
            "worker",
            support::worker_bin(),
            &[],
            &[
                ("PHYSICIAN_DATABASE_URL", &database.url),
                ("PHYSICIAN_REDIS_URL", &redis),
                ("PUBSUB_EMULATOR_HOST", &emulator.host),
                ("PHYSICIAN_WORKER_LISTEN_ADDR", &worker_host),
                ("PHYSICIAN_WORKER_FIXED_TEXT", support::FIXED_TEXT),
                ("PHYSICIAN_WORKER_FIXED_FAILURES", &failures),
                ("PHYSICIAN_WORKER_FIXED_DELAY_MS", &delay),
            ],
            &home,
        );
        support::wait_ready(&mut worker, worker_port).await;
        let topic = support::topic();
        let mut sites = Vec::new();
        let mut hosts = [String::new(), String::new()];
        for (index, host) in hosts.iter_mut().enumerate() {
            let port = support::free_port();
            *host = format!("127.0.0.1:{port}");
            // Different snapshots on A and B prove that a duplicate accepted
            // on B cannot replace the exact observation originally pinned on A.
            let content = format!("Synthetic source observation from site {index}.");
            let sources = vec![local_product::physician_sources::SourceRevision {
                workspace: physician_sample::WORKSPACE_IDS[0].into(),
                candidate_id: format!("source-site-{index}"),
                revision: format!("immutable-site-{index}"),
                sha256: local_product::sha256_hex(content.as_bytes()),
                content,
                provenance: json!({"synthetic": true, "site": index}),
            }];
            let snapshot = home.join(format!("sources-{index}.json"));
            std::fs::write(&snapshot, serde_json::to_vec(&sources).unwrap()).unwrap();
            let mut site = support::Process::spawn(
                &format!("site-{index}"),
                support::site_bin(),
                &[],
                &[
                    ("PHYSICIAN_DATABASE_URL", &database.url),
                    ("PHYSICIAN_REDIS_URL", &redis),
                    ("PUBSUB_EMULATOR_HOST", &emulator.host),
                    ("GOOGLE_CLOUD_PROJECT", support::PROJECT),
                    ("PHYSICIAN_CHAT_TOPIC", &topic),
                    ("CODA_PHYSICIAN_SOURCE_SNAPSHOT", snapshot.to_str().unwrap()),
                    ("PHYSICIAN_AUTH", "required"),
                    ("PHYSICIAN_LISTEN_ADDR", host),
                ],
                &home,
            );
            // Start sequentially, as each instance seeds the practice panel.
            support::wait_ready(&mut site, port).await;
            sites.push(site);
        }
        let session = support::sign_in(&hosts[0]).await;
        Some(Self {
            database,
            store,
            emulator,
            _valkey: valkey,
            worker,
            sites,
            hosts,
            worker_host,
            session,
            home,
        })
    }

    fn logs(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}",
            self.sites[0].tail(),
            self.sites[1].tail(),
            self.worker.tail(),
            self.emulator.tail()
        )
    }

    async fn seed(&self) -> Scope {
        let scope = Scope {
            practice: physician_sample::STAGING_PRACTICE_ID.into(),
            physician: "staging-physician".into(),
            workspace: physician_sample::WORKSPACE_IDS[0].into(),
            chat: format!("reply-{}", uuid::Uuid::new_v4().simple()),
        };
        assert_eq!(self.store.open_draft(&scope, ORIGINAL).await.unwrap(), 1);
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

fn dispatches(draft: &Draft) -> Vec<&Request> {
    draft
        .attempts
        .iter()
        .flat_map(|attempt| &attempt.records)
        .filter_map(|record| match record {
            Record::Dispatch { request, .. } => Some(request),
            _ => None,
        })
        .collect()
}

#[tokio::test]
async fn duplicate_acceptance_and_worker_retry_keep_one_input_and_pinned_sources() {
    let Some(running) = Running::start("reply_retry", 1, 250).await else {
        return;
    };
    let scope = running.seed().await;
    let key = format!("revision-{}", uuid::Uuid::new_v4().simple());
    let command = revise(&scope, &key, 1);
    running.command(0, &command, "accepted").await;
    running.command(1, &command, "accepted").await;
    let accepted = running.store.draft(&scope).await.unwrap().unwrap();
    let input = accepted.input.clone().expect("accepted input");
    assert_eq!(input.key, key);
    assert_eq!(input.expected_revision, 1);
    assert_eq!(input.instruction, INSTRUCTION);
    assert_eq!(input.timezone, "America/New_York");
    assert_eq!(accepted.base_text, ORIGINAL);
    assert_eq!(accepted.sources.len(), 1);
    assert_eq!(accepted.sources[0].candidate_id, "source-site-0");
    assert_eq!(
        accepted.sources[0].content,
        "Synthetic source observation from site 0."
    );
    assert!(
        accepted
            .sources
            .iter()
            .all(|source| source.workspace == scope.workspace)
    );

    let complete = running
        .wait_draft(&scope, |draft| draft.status == "completed")
        .await;
    assert_eq!(complete.input.as_ref(), Some(&input));
    assert_eq!(
        serde_json::to_value(&complete.sources).unwrap(),
        serde_json::to_value(&accepted.sources).unwrap()
    );
    assert_eq!(
        complete
            .attempts
            .iter()
            .map(|attempt| attempt.status.as_str())
            .collect::<Vec<_>>(),
        ["interrupted", "completed"]
    );
    assert!(
        complete
            .attempts
            .iter()
            .all(|attempt| attempt.input_id == input.key)
    );
    assert_eq!(complete.revision, 2);
    assert_eq!(complete.text, support::FIXED_TEXT);
    assert_eq!(complete.restore_text.as_deref(), Some(ORIGINAL));
    let requests = dispatches(&complete);
    assert_eq!(requests.len(), 2, "one provider call on each delivery");
    assert_eq!(
        requests[0], requests[1],
        "retry must dispatch the exact same input and pinned source references"
    );
    let Item::Message { text, .. } = &requests[0].items[0] else {
        panic!("revision context must be a message")
    };
    let context: Value = serde_json::from_str(text).expect("revision request context");
    assert_eq!(context["saved_draft"], ORIGINAL);
    assert_eq!(context["instruction"], INSTRUCTION);
    assert_eq!(
        context["source_references"],
        json!(
            accepted
                .sources
                .iter()
                .map(|source| &source.candidate_id)
                .collect::<Vec<_>>()
        )
    );

    let (status, read) = running.read(1, &scope).await;
    assert_eq!(status, 200, "{read}");
    assert_eq!(read["revision"], 2);
    assert_eq!(read["text"], support::FIXED_TEXT);
    assert_eq!(read["restore_text"], ORIGINAL);

    // Completion does not invalidate the receipt; neither duplicate command
    // nor duplicate Pub/Sub wakeup may create another attempt or draft revision.
    running.command(1, &command, "accepted").await;
    let delivery = Delivery::reply_revision(scope.clone(), key);
    running.emulator.publish(&delivery).await;
    let push = support::push_body(&delivery, "revision-duplicate");
    let (status, body) = support::exchange(
        &running.worker_host,
        "POST",
        "/pubsub/chat-turn",
        &[("content-type", "application/json")],
        Some(&push),
    )
    .await
    .unwrap();
    assert_eq!(status, 204, "{body}\n{}", running.logs());
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

#[tokio::test]
async fn newer_physician_save_fences_out_a_late_revision_completion() {
    let Some(running) = Running::start("reply_save", 0, 2000).await else {
        return;
    };
    let scope = running.seed().await;
    let command = revise(&scope, "revision-before-save", 1);
    running.command(0, &command, "accepted").await;
    // Observe the durable dispatch, not a timing guess, before saving on B.
    running
        .wait_draft(&scope, |draft| {
            draft.status == "active" && !dispatches(draft).is_empty()
        })
        .await;
    let (status, body) = running.save(&scope, 1, SAVED).await;
    assert_eq!(status, 200, "{body}\n{}", running.logs());
    let (status, body) = running.save(&scope, 1, "stale overwrite").await;
    assert_eq!(status, 409, "{body}");
    // The worker may cooperatively cancel the delayed provider at the save
    // fence. Wait for either retained late output or the retained cancellation
    // failure, never merely for the save's synchronous superseded state.
    let late = running
        .wait_draft(&scope, |draft| {
            draft
                .attempts
                .iter()
                .flat_map(|attempt| &attempt.records)
                .any(|record| matches!(record, Record::Exchange { .. } | Record::Failure { .. }))
        })
        .await;
    assert_eq!(late.status, "superseded");
    assert_eq!(late.revision, 2);
    assert_eq!(late.text, SAVED);
    assert_eq!(late.restore_text.as_deref(), Some(ORIGINAL));
    assert!(
        late.attempts
            .iter()
            .all(|attempt| attempt.status != "completed")
    );
    let delivery = Delivery::reply_revision(scope.clone(), "revision-before-save".into());
    let push = support::push_body(&delivery, "superseded-revision");
    let (status, body) = support::exchange(
        &running.worker_host,
        "POST",
        "/pubsub/chat-turn",
        &[("content-type", "application/json")],
        Some(&push),
    )
    .await
    .unwrap();
    assert_eq!(status, 204, "{body}");
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

fn exchanges(attempts: &[RevisionAttempt]) -> usize {
    attempts
        .iter()
        .flat_map(|attempt| &attempt.records)
        .filter(|record| matches!(record, Record::Exchange { .. }))
        .count()
}

#[tokio::test]
async fn a_later_revision_keeps_the_earlier_revisions_execution_evidence() {
    let Some(running) = Running::start("reply_history", 0, 0).await else {
        return;
    };
    let scope = running.seed().await;
    let (status, read) = running.read(0, &scope).await;
    assert_eq!(status, 200, "{read}");
    assert_eq!(read["revision_run"], Value::Null);

    // Three revisions in turn, alternating sites. Snapshot each one while it
    // is current, so the history can be compared byte for byte afterwards.
    let keys = ["revision-first", "revision-second", "revision-third"];
    let mut current = Vec::new();
    for (index, key) in keys.iter().enumerate() {
        let expected = index as u64 + 1;
        running
            .command(index % 2, &revise(&scope, key, expected), "accepted")
            .await;
        let done = running
            .wait_draft(&scope, |draft| {
                draft.status == "completed" && draft.revision == expected + 1
            })
            .await;
        assert_eq!(done.input.as_ref().unwrap().key, *key);
        assert!(done.attempts.iter().all(|attempt| attempt.input_id == *key));
        assert_eq!(exchanges(&done.attempts), 1);
        current.push(done);
    }

    // The draft row holds only the current revision's work.
    let draft = running.store.draft(&scope).await.unwrap().unwrap();
    let stored = serde_json::to_value(&draft).unwrap();
    assert!(stored.get("earlier").is_none(), "{stored}");
    assert_eq!(
        serde_json::to_value(&draft.attempts).unwrap(),
        serde_json::to_value(&current[2].attempts).unwrap()
    );
    for key in &keys[..2] {
        assert!(
            !stored.to_string().contains(key),
            "draft row must not carry earlier revision {key}: {stored}"
        );
    }

    // The history table keeps the first two, oldest first, unchanged.
    let earlier = running.store.earlier_revisions(&scope).await.unwrap();
    assert_eq!(
        earlier
            .iter()
            .map(|e| e.input.key.as_str())
            .collect::<Vec<_>>(),
        &keys[..2],
        "earlier revisions must be retained in order"
    );
    for (kept, then) in earlier.iter().zip(&current) {
        assert_eq!(&kept.input, then.input.as_ref().unwrap());
        assert_eq!(kept.status, "completed");
        assert_eq!(kept.failures, then.failures);
        assert_eq!(kept.base_text, then.base_text);
        assert_eq!(
            serde_json::to_value(&kept.sources).unwrap(),
            serde_json::to_value(&then.sources).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&kept.attempts).unwrap(),
            serde_json::to_value(&then.attempts).unwrap(),
            "accepting a revision must not rewrite or drop the previous attempts"
        );
    }
    assert_eq!(earlier[0].base_text, ORIGINAL);

    // Another physician's scope over the same patient and reply sees none.
    let mut other = scope.clone();
    other.physician = "other-revision-physician".into();
    assert!(
        running
            .store
            .earlier_revisions(&other)
            .await
            .unwrap()
            .is_empty()
    );

    let (status, read) = running.read(1, &scope).await;
    assert_eq!(status, 200, "{read}");
    assert_eq!(read["revision"], 4);
    assert_eq!(
        read["revision_run"],
        json!({"run_id": "revision-third", "status": "completed"})
    );
}

#[tokio::test]
async fn a_terminally_failed_revision_is_visible_over_http() {
    let Some(running) = Running::start("reply_failed", MAX_FAILED_ATTEMPTS, 0).await else {
        return;
    };
    let scope = running.seed().await;
    running
        .command(0, &revise(&scope, "revision-fails", 1), "accepted")
        .await;
    let failed = running
        .wait_draft(&scope, |draft| draft.status == "failed")
        .await;
    assert_eq!(failed.attempts.len(), MAX_FAILED_ATTEMPTS as usize);
    assert_eq!(failed.revision, 1);
    for site in 0..2 {
        let (status, read) = running.read(site, &scope).await;
        assert_eq!(status, 200, "{read}");
        assert_eq!(read["revision"], 1);
        assert_eq!(read["text"], ORIGINAL);
        assert_eq!(
            read["revision_run"],
            json!({"run_id": "revision-fails", "status": "failed"})
        );
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
        .await;
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
    assert!(original.input.is_none());

    let (status, _) = support::exchange(&running.hosts[0], "GET", &path(&scope), &[], None)
        .await
        .unwrap();
    assert_eq!(status, 401);
}
