//! ENG-200: a second message in the same conversation steers the reply in
//! progress. Synthetic identity, deterministic providers, disposable Postgres.
use async_trait::async_trait;
use axum::{Router, body::Body, http::Request as HttpRequest};
use duet_api::{
    ApiError,
    conversation::ConversationStore,
    conversation_routes::ConversationState,
    database::{AccountDatabase, migrate},
    durable::DurableState,
    evidence::EvidenceStore,
    identity::{AccountContext, IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore},
};
use harness::execution::{
    Cancellation, Item, Limits, Owner, Provider, Record, Request, Response, StreamEvent, Usage,
};
use serde_json::{Value, json};
use sqlx::{AssertSqlSafe, ConnectOptions, Connection, PgConnection};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::ServiceExt;
use tracing::{Instrument, instrument::WithSubscriber};
use uuid::Uuid;

#[derive(Clone, Default)]
struct TraceCapture(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for TraceCapture {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl TraceCapture {
    fn output(&self) -> String {
        String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
    }

    fn subscriber(&self) -> impl tracing::Subscriber + Send + Sync + 'static {
        let writer = self.clone();
        tracing_subscriber::fmt()
            .with_ansi(false)
            .with_max_level(tracing::Level::INFO)
            .with_span_events(
                tracing_subscriber::fmt::format::FmtSpan::NEW
                    | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            )
            .with_writer(move || writer.clone())
            .finish()
    }

    async fn completed(&self) -> String {
        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            loop {
                let output = self.output();
                if output.lines().any(|line| {
                    line.contains("duet.conversation.execute") && line.contains("close")
                }) {
                    return output;
                }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap_or_else(|_| {
            panic!(
                "spawned conversation span must close after privacy confirmation: {}",
                self.output()
            )
        })
    }
}

const FIRST: &str = "First question about my sleep";
const SECOND: &str = "Second thought: I also wake at 4am";

struct Identity;
#[async_trait]
impl IdentityVerifier for Identity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        if bearer != "a" {
            return Err(ApiError::Unauthorized);
        }
        Ok(VerifiedIdentity {
            subject: "steering-a".into(),
            authenticated_at: chrono::Utc::now(),
        })
    }
}

/// Holds its exchange open until the owner fences it out, then returns a late
/// final answer that never saw the steer.
struct Held {
    sent: Arc<tokio::sync::Notify>,
    cancelled: Arc<AtomicBool>,
}
impl Provider for Held {
    async fn exchange(
        &mut self,
        _: &Request,
        cancel: &Cancellation,
        emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<Response, String> {
        emit(StreamEvent::Usage(Usage {
            input_tokens: Some(5),
            output_tokens: Some(1),
        }))?;
        self.sent.notify_one();
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        while !cancel.is_cancelled() && tokio::time::Instant::now() < deadline {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        self.cancelled
            .store(cancel.is_cancelled(), Ordering::SeqCst);
        Ok(response("Stale answer to the first message only"))
    }
}

/// Records every dispatched request and answers with one final reply each.
struct Recording {
    requests: Arc<Mutex<Vec<Request>>>,
    replies: std::collections::VecDeque<&'static str>,
}
impl Provider for Recording {
    async fn exchange(
        &mut self,
        request: &Request,
        _: &Cancellation,
        _: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<Response, String> {
        self.requests.lock().unwrap().push(request.clone());
        Ok(response(
            self.replies
                .pop_front()
                .expect("unexpected paid/provider dispatch"),
        ))
    }
}

fn response(text: &str) -> Response {
    Response {
        text: text.into(),
        calls: vec![],
        usage: Usage {
            input_tokens: Some(7),
            output_tokens: Some(3),
        },
        provenance: json!({"mode":"controlled"}),
    }
}

fn assistant(id: Uuid) -> Uuid {
    Uuid::from_u128(id.as_u128() ^ (1u128 << 127))
}

/// How many user messages in `request` carry exactly `text`.
fn user_messages(request: &Request, text: &str) -> usize {
    request
        .items
        .iter()
        .filter(
            |item| matches!(item, Item::Message { role, text: t } if role == "user" && t == text),
        )
        .count()
}

/// Every Dispatch request recorded for `command`, in order.
async fn dispatches(admin: &mut PgConnection, command: Uuid) -> Vec<(u64, Request)> {
    let records: Vec<Value> = sqlx::query_scalar(
        "select record from duet_execution_records where command_id=$1 order by ordinal",
    )
    .bind(command)
    .fetch_all(admin)
    .await
    .unwrap();
    records
        .into_iter()
        .filter_map(|record| match serde_json::from_value(record).unwrap() {
            Record::Dispatch { sequence, request } => Some((sequence, request)),
            _ => None,
        })
        .collect()
}

async fn count(admin: &mut PgConnection, sql: &'static str, id: Uuid) -> i64 {
    sqlx::query_scalar(sql)
        .bind(id)
        .fetch_one(admin)
        .await
        .unwrap()
}

struct Fixture {
    name: String,
    role: String,
    owner_url: String,
    runtime_url: String,
    db: AccountDatabase,
    durable: DurableState,
    store: ConversationStore,
    journal: std::path::PathBuf,
}

async fn fixture(prefix: &str) -> (PgConnection, Fixture) {
    let url = std::env::var("DUET_TEST_DATABASE_URL").expect("real local PostgreSQL required");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let mut admin = PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("{prefix}_{suffix}");
    let role = format!("{prefix}_role_{suffix}");
    // A per-run random password lets the role log in over TCP whether the
    // disposable server trusts loopback (local) or requires passwords (CI).
    let password = Uuid::new_v4().simple().to_string();
    sqlx::query(AssertSqlSafe(format!(
        "create role {role} login password '{password}'"
    )))
    .execute(&mut admin)
    .await
    .unwrap();
    sqlx::query(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let options = options.database(&name);
    let owner_url = options.to_url_lossy().to_string();
    migrate(&owner_url, &role).await.unwrap();
    let runtime_url = options
        .username(&role)
        .password(&password)
        .to_url_lossy()
        .to_string();
    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let journal = std::env::temp_dir().join(format!("{prefix}-journal-{suffix}"));
    let durable = DurableState {
        identity: IdentityStore::new(db.clone(), Arc::new(Identity)),
        evidence: EvidenceStore::new(db.clone()),
        privacy: PrivacyStore::new(
            db.clone(),
            Arc::new(FileJournal::open(journal.clone()).await.unwrap()),
        ),
    };
    let store = ConversationStore::new(db.clone());
    (
        admin,
        Fixture {
            name,
            role,
            owner_url,
            runtime_url,
            db,
            durable,
            store,
            journal,
        },
    )
}

async fn request(app: &Router, method: &str, path: &str, body: Value) -> (u16, Value) {
    let response = app
        .clone()
        .oneshot(
            HttpRequest::builder()
                .method(method)
                .uri(path)
                .header("authorization", "Bearer a")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let code = response.status().as_u16();
    let bytes = axum::body::to_bytes(response.into_body(), 3_000_000)
        .await
        .unwrap();
    (code, serde_json::from_slice(&bytes).unwrap_or(Value::Null))
}

async fn observed_request(app: &Router, body: Value, capture: &TraceCapture) -> (u16, Value) {
    async {
        request(app, "POST", "/api/conversation/messages", body)
            .instrument(tracing::info_span!("test.accepted_request"))
            .await
    }
    .with_subscriber(capture.subscriber())
    .await
}

#[tokio::test]
async fn accepted_provider_failure_is_safe_and_correlated() {
    accepted_failure(false).await;
}

#[tokio::test]
async fn failed_steer_lookup_still_attempts_privacy_confirmation() {
    accepted_failure(true).await;
}

async fn accepted_failure(database_failure: bool) {
    let (admin, f) = fixture("async_failure").await;
    let owner_url = f.owner_url.clone();
    let role = f.role.clone();
    let release = Arc::new(tokio::sync::Notify::new());
    let provider_release = release.clone();
    let received = Arc::new(tokio::sync::Notify::new());
    let provider_received = received.clone();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/responses", listener.local_addr().unwrap());
    let provider = Router::new().route(
        "/responses",
        axum::routing::post(move || {
            let release = provider_release.clone();
            let received = provider_received.clone();
            async move {
                received.notify_one();
                release.notified().await;
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "PRIVATE-PROVIDER-ERROR",
                )
            }
        }),
    );
    let server = tokio::spawn(async move { axum::serve(listener, provider).await.unwrap() });
    let app = app(&f, Some((endpoint, "PRIVATE-PROVIDER-KEY".into())));
    enroll(&app).await;
    let c = f.durable.identity.authenticate("a", true).await.unwrap();
    let id = Uuid::new_v4();
    let capture = TraceCapture::default();
    let accepted = observed_request(
        &app,
        json!({"id":id,"text":"PRIVATE-PATIENT-COMMAND","timezone":"UTC"}),
        &capture,
    )
    .await;
    assert_eq!(accepted.0, 202, "{accepted:?}");
    // The accepted route has returned before the provider is allowed to fail.
    tokio::time::timeout(std::time::Duration::from_secs(10), received.notified())
        .await
        .unwrap();
    if database_failure {
        let mut owner = PgConnection::connect(&owner_url).await.unwrap();
        // Test-only fault injection on this fixture's isolated runtime role.
        // Removing table access makes both steer lookup and confirmation fail.
        sqlx::query(AssertSqlSafe(format!(
            "revoke all on duet_commands, privacy_recovery_events from {role}"
        )))
        .execute(&mut owner)
        .await
        .unwrap();
    }
    release.notify_one();
    let output = capture.completed().await;
    let expected = if database_failure {
        vec![("steered", "database"), ("confirm", "database")]
    } else {
        vec![("execute", "execution")]
    };
    let events: Vec<_> = output
        .lines()
        .filter(|line| line.contains("duet.conversation.failed"))
        .collect();
    assert_eq!(events.len(), expected.len(), "{output}");
    for (event, (step, class)) in events.iter().zip(expected) {
        for field in [
            "test.accepted_request".to_owned(),
            "duet.conversation.execute".to_owned(),
            "otel.status_code=\"ERROR\"".to_owned(),
            format!("step=\"{step}\""),
            format!("error_class=\"{class}\""),
        ] {
            assert!(event.contains(&field), "missing {field}: {output}");
        }
    }
    for private in [
        "PRIVATE-PATIENT-COMMAND",
        "PRIVATE-PROVIDER-ERROR",
        "PRIVATE-PROVIDER-KEY",
        &id.to_string(),
        &c.account_id.to_string(),
    ] {
        assert!(
            !output.contains(private),
            "private content in trace: {output}"
        );
    }
    server.abort();
    drop(app);
    teardown(admin, f).await;
}

async fn enroll(app: &Router) {
    for (path, body) in [
        ("/api/auth/account", json!({})),
        ("/api/auth/account/terms", json!({"version":"2026-09-23"})),
        (
            "/api/auth/account/luna-consent",
            json!({"decision":"accepted","version":"2026-09-24","expected_revision":0}),
        ),
    ] {
        let result = request(app, "PUT", path, body).await;
        assert!((200..300).contains(&result.0), "{result:?}");
    }
}

fn app(f: &Fixture, provider: Option<(String, String)>) -> Router {
    duet_api::durable::router(f.durable.clone()).merge(duet_api::conversation_routes::router(
        ConversationState {
            durable: f.durable.clone(),
            store: f.store.clone(),
            model: "local".into(),
            provider,
        },
    ))
}

async fn teardown(mut admin: PgConnection, f: Fixture) {
    let Fixture {
        name,
        role,
        db,
        durable,
        store,
        journal,
        ..
    } = f;
    drop(store);
    drop(durable);
    drop(db);
    sqlx::query(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let _ = std::fs::remove_dir_all(journal);
}

#[tokio::test]
async fn steer_reaches_the_reply_in_progress_once() {
    let (admin, f) = fixture("steer").await;
    let mut owner_db = PgConnection::connect(&f.owner_url).await.unwrap();
    enroll(&app(&f, None)).await;
    let c: AccountContext = f.durable.identity.authenticate("a", true).await.unwrap();
    let store = &f.store;

    // --- A message sent while a reply streams joins that reply. ---
    let a = Uuid::new_v4();
    store.phone_accept(&c, a, FIRST, "local").await.unwrap();
    let sent = Arc::new(tokio::sync::Notify::new());
    let cancelled = Arc::new(AtomicBool::new(false));
    let mut owner = store.claim(&c, a).await.unwrap();
    let mut held = Held {
        sent: sent.clone(),
        cancelled: cancelled.clone(),
    };
    let attempt = tokio::spawn(async move {
        let result = owner
            .execute(&mut held, Limits::default(), &Cancellation::default())
            .await;
        (owner, result)
    });
    sent.notified().await;
    let b = Uuid::new_v4();
    let steered = store.phone_accept(&c, b, SECOND, "local").await.unwrap();
    assert_eq!(steered["message"]["id"], json!(b));
    assert_eq!(steered["message"]["run_id"], json!(a), "{steered}");
    assert_eq!(steered["run"]["id"], json!(a), "{steered}");
    // Retrying the same steer is a no-op; reusing its id for new text is not.
    let retried = store.phone_accept(&c, b, SECOND, "local").await.unwrap();
    assert_eq!(retried["message"], steered["message"]);
    assert!(matches!(
        store.phone_accept(&c, b, "Different words", "local").await,
        Err(ApiError::Conflict(_))
    ));
    assert_eq!(
        count(
            &mut owner_db,
            "select count(*) from duet_command_steers where command_id=$1",
            a
        )
        .await,
        1
    );
    // The fenced attempt is cancelled mid-stream and its late answer is refused.
    let (mut fenced, result) = attempt.await.unwrap();
    assert!(result.is_err(), "fenced attempt must not commit");
    assert!(
        cancelled.load(Ordering::SeqCst),
        "steer_revision fence must cancel the open stream"
    );
    assert!(fenced.complete(&response("late")).await.is_err());
    assert!(fenced.steered().await.unwrap());
    let state: String = sqlx::query_scalar("select state from duet_commands where id=$1")
        .bind(a)
        .fetch_one(&mut owner_db)
        .await
        .unwrap();
    assert_eq!(state, "accepted");

    // --- Same command is reclaimed with both messages; one reply commits. ---
    let requests = Arc::new(Mutex::new(vec![]));
    let mut recording = Recording {
        requests: requests.clone(),
        replies: vec!["You asked about sleep and waking at 4am."].into(),
    };
    let reply = store
        .claim(&c, a)
        .await
        .unwrap()
        .execute(&mut recording, Limits::default(), &Cancellation::default())
        .await
        .unwrap();
    assert_eq!(reply.text, "You asked about sleep and waking at 4am.");
    assert!(
        fenced.steered().await.unwrap(),
        "a replaced attempt remains steered after another launch reclaims and completes"
    );
    let sent_requests = requests.lock().unwrap().clone();
    assert_eq!(sent_requests.len(), 1);
    assert_eq!(user_messages(&sent_requests[0], FIRST), 1);
    assert_eq!(user_messages(&sent_requests[0], SECOND), 1);
    // Durable dispatch history: the steer was carried by exactly one dispatch.
    let history = dispatches(&mut owner_db, a).await;
    assert_eq!(history.len(), 2, "held attempt + reclaimed attempt");
    assert_eq!(user_messages(&history[0].1, SECOND), 0);
    assert_eq!(user_messages(&history[1].1, SECOND), 1);
    let applied: Option<i64> =
        sqlx::query_scalar("select applied_sequence from duet_command_steers where id=$1")
            .bind(b)
            .fetch_one(&mut owner_db)
            .await
            .unwrap();
    assert_eq!(applied, Some(history[1].0 as i64));
    // One command row, every paid call on it, and no second provider job.
    assert_eq!(
        count(
            &mut owner_db,
            "select count(*) from duet_commands where user_id=$1",
            c.account_id
        )
        .await,
        1
    );
    assert_eq!(
        count(
            &mut owner_db,
            "select count(*) from duet_commands where id=$1",
            b
        )
        .await,
        0
    );
    assert_eq!(
        count(
            &mut owner_db,
            "select count(*) from duet_model_calls where user_id=$1",
            c.account_id
        )
        .await,
        2
    );
    assert_eq!(
        count(
            &mut owner_db,
            "select count(distinct command_id) from duet_model_calls where user_id=$1",
            c.account_id
        )
        .await,
        1
    );
    // The transcript interleaves the steer before the one reply, monotonically.
    let page = store.phone_page(&c, true).await.unwrap();
    serde_json::from_value::<duet_wire::models::ConversationPage>(page.clone()).unwrap();
    let ids: Vec<Value> = page["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["id"].clone())
        .collect();
    assert_eq!(ids, vec![json!(a), json!(b), json!(assistant(a))]);
    let sequences: Vec<i64> = page["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["sequence"].as_i64().unwrap())
        .collect();
    assert!(sequences.windows(2).all(|w| w[0] < w[1]), "{sequences:?}");
    assert!(page["revision"].as_i64().unwrap() >= *sequences.last().unwrap());
    assert!(!page.to_string().contains("Stale answer"));
    // A retry after completion is still a no-op that dispatches nothing.
    let after = store.phone_accept(&c, b, SECOND, "local").await.unwrap();
    assert_eq!(after["message"], steered["message"]);
    assert_eq!(
        count(
            &mut owner_db,
            "select count(*) from duet_model_calls where user_id=$1",
            c.account_id
        )
        .await,
        2
    );

    // --- Restart: steer while the lease is held, lose the owner, recover. ---
    let d = Uuid::new_v4();
    store
        .phone_accept(&c, d, "Third: what about naps?", "local")
        .await
        .unwrap();
    let mut lost = store.claim(&c, d).await.unwrap();
    let e = Uuid::new_v4();
    let steer = store
        .phone_accept(&c, e, "Fourth: I nap at 2pm", "local")
        .await
        .unwrap();
    assert_eq!(steer["run"]["id"], json!(d));
    // A late completion from the fenced attempt is refused.
    assert!(lost.complete(&response("late old lease")).await.is_err());
    drop(lost);
    sqlx::query("update duet_commands set lease_until=now()-interval '1 second' where id=$1")
        .bind(d)
        .execute(&mut owner_db)
        .await
        .unwrap();
    let restarted = ConversationStore::new(AccountDatabase::connect(&f.runtime_url).await.unwrap());
    let requests = Arc::new(Mutex::new(vec![]));
    restarted
        .claim(&c, d)
        .await
        .unwrap()
        .execute(
            &mut Recording {
                requests: requests.clone(),
                replies: vec!["Naps at 2pm are fine."].into(),
            },
            Limits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let sent_requests = requests.lock().unwrap().clone();
    assert_eq!(sent_requests.len(), 1);
    assert_eq!(
        user_messages(&sent_requests[0], "Third: what about naps?"),
        1
    );
    assert_eq!(user_messages(&sent_requests[0], "Fourth: I nap at 2pm"), 1);
    let (state, replies): (String, i64) = sqlx::query_as(
        "select state,(select count(*) from duet_commands where id=$1 and reply is not null) from duet_commands where id=$1",
    )
    .bind(d)
    .fetch_one(&mut owner_db)
    .await
    .unwrap();
    assert_eq!((state.as_str(), replies), ("complete", 1));
    // A further restart cannot replay the steer into a second reply.
    assert!(restarted.claim(&c, d).await.is_err());

    // --- ENG-191 refusals hold for everything that is not a steer. ---
    let revision: i64 = sqlx::query_scalar("select revision from duet_conversations where id=$1")
        .bind(c.account_id)
        .fetch_one(&mut owner_db)
        .await
        .unwrap();
    for kind in ["today", "follow_up"] {
        let busy = Uuid::new_v4();
        sqlx::query("insert into duet_commands(user_id,id,conversation_id,context,input,base_revision,request,work_kind,today_assignment) values($1,$2,$1,'{}','',$3,'{}',$4,case when $4='today' then '{}'::jsonb end)")
            .bind(c.account_id).bind(busy).bind(revision).bind(kind)
            .execute(&mut owner_db).await.unwrap();
        assert!(
            matches!(
                store
                    .phone_accept(&c, Uuid::new_v4(), "During background work", "local")
                    .await,
                Err(ApiError::Conflict(_))
            ),
            "{kind} work in flight must still refuse"
        );
        sqlx::query("update duet_commands set state='cancelled' where id=$1")
            .bind(busy)
            .execute(&mut owner_db)
            .await
            .unwrap();
    }
    let g = Uuid::new_v4();
    store
        .phone_accept(&c, g, "Fifth question", "local")
        .await
        .unwrap();
    assert!(matches!(
        store
            .accept(
                &c,
                Uuid::new_v4(),
                Uuid::new_v4(),
                0,
                "Another conversation",
                "local"
            )
            .await,
        Err(ApiError::Conflict(_))
    ));
    assert!(matches!(
        store
            .accept(
                &c,
                Uuid::new_v4(),
                c.account_id,
                revision - 1,
                "Stale revision",
                "local"
            )
            .await,
        Err(ApiError::Conflict(_))
    ));
    // A steer id may not reuse an assistant identity.
    assert!(matches!(
        store
            .phone_accept(&c, assistant(g), "Reused identity", "local")
            .await,
        Err(ApiError::Conflict(_))
    ));
    let via_command = store
        .accept(
            &c,
            Uuid::new_v4(),
            c.account_id,
            revision,
            "Sixth, same conversation",
            "local",
        )
        .await
        .unwrap();
    assert_eq!(via_command["run_id"], json!(g));

    // --- A dispatch that omits an accepted steer is refused. ---
    let mut owner = store.claim(&c, g).await.unwrap();
    let frozen: Request = serde_json::from_value(
        sqlx::query_scalar("select request from duet_commands where id=$1")
            .bind(g)
            .fetch_one(&mut owner_db)
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(
        owner
            .record(Record::Dispatch {
                sequence: 1,
                request: frozen,
            })
            .await
            .is_err()
    );
    drop(owner);
    drop(owner_db);
    teardown(admin, f).await;
}

/// Minimal OpenAI Responses stand-in. A request without the steer is held
/// open (pinging) until the client drops it; one with the steer completes.
async fn fake_responses(listener: tokio::net::TcpListener, bodies: Arc<Mutex<Vec<Value>>>) {
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let bodies = bodies.clone();
        tokio::spawn(async move {
            let mut buffer = Vec::new();
            let mut chunk = [0u8; 8192];
            let (head, length) = loop {
                let n = socket.read(&mut chunk).await.unwrap_or(0);
                if n == 0 {
                    return;
                }
                buffer.extend_from_slice(&chunk[..n]);
                if let Some(end) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
                    let text = String::from_utf8_lossy(&buffer[..end]).to_lowercase();
                    let length = text
                        .lines()
                        .find_map(|l| l.strip_prefix("content-length:"))
                        .map(|v| v.trim().parse::<usize>().unwrap())
                        .unwrap_or(0);
                    break (end + 4, length);
                }
            };
            while buffer.len() < head + length {
                let n = socket.read(&mut chunk).await.unwrap_or(0);
                if n == 0 {
                    return;
                }
                buffer.extend_from_slice(&chunk[..n]);
            }
            let body: Value = serde_json::from_slice(&buffer[head..head + length]).unwrap();
            let steered = body.to_string().contains(SECOND);
            let ordinal = {
                let mut all = bodies.lock().unwrap();
                all.push(body);
                all.len()
            };
            let start =
                "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\nconnection: close\r\n\r\n";
            if socket.write_all(start.as_bytes()).await.is_err() {
                return;
            }
            let created =
                json!({"type":"response.created","response":{"id":format!("resp_{ordinal}")}});
            if socket
                .write_all(format!("data: {created}\n\n").as_bytes())
                .await
                .is_err()
            {
                return;
            }
            if steered {
                let completed = json!({"type":"response.completed","response":{"id":format!("resp_{ordinal}"),"status":"completed","output":[{"type":"message","content":[{"type":"output_text","text":"Both messages answered."}]}],"usage":{"input_tokens":11,"output_tokens":4}}});
                let _ = socket
                    .write_all(format!("data: {completed}\n\n").as_bytes())
                    .await;
                let _ = socket.shutdown().await;
                return;
            }
            for _ in 0..3000 {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                if socket.write_all(b": ping\n\n").await.is_err() {
                    return;
                }
            }
        });
    }
}

#[tokio::test]
async fn http_steer_relaunches_the_same_command_without_the_recovery_poll() {
    let (admin, f) = fixture("steer_http").await;
    let mut owner_db = PgConnection::connect(&f.owner_url).await.unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/responses", listener.local_addr().unwrap());
    let bodies = Arc::new(Mutex::new(vec![]));
    let server = tokio::spawn(fake_responses(listener, bodies.clone()));
    let app = app(&f, Some((endpoint, "loopback-test-key".into())));
    enroll(&app).await;
    let c: AccountContext = f.durable.identity.authenticate("a", true).await.unwrap();

    let a = Uuid::new_v4();
    let first_capture = TraceCapture::default();
    let (status, first) = observed_request(
        &app,
        json!({"id":a,"text":FIRST,"timezone":"UTC"}),
        &first_capture,
    )
    .await;
    assert_eq!(status, 202, "{first}");
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        while bodies.lock().unwrap().is_empty() {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("first provider request");

    let b = Uuid::new_v4();
    let second_capture = TraceCapture::default();
    let (status, second) = observed_request(
        &app,
        json!({"id":b,"text":SECOND,"timezone":"UTC"}),
        &second_capture,
    )
    .await;
    assert_eq!(status, 202, "{second}");
    assert_eq!(second["run"]["id"], json!(a), "{second}");
    assert_eq!(second["message"]["id"], json!(b));

    // Well inside the 30s recovery poll, which this test never starts.
    let page = tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let page = f.store.phone_page(&c, true).await.unwrap();
            if page["messages"]
                .as_array()
                .unwrap()
                .iter()
                .any(|m| m["id"] == json!(assistant(a)))
            {
                return page;
            }
            tokio::time::sleep(std::time::Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("steered reply commits without waiting for recovery");
    let texts: Vec<&str> = page["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["text"].as_str().unwrap())
        .collect();
    assert_eq!(texts, vec![FIRST, SECOND, "Both messages answered."]);
    // The steer route may find the first launch still claimed; that no-op
    // legitimately creates no execution span. Wait only for actual launches.
    first_capture.completed().await;
    for capture in [&first_capture, &second_capture] {
        let output = if capture.output().contains("duet.conversation.execute") {
            capture.completed().await
        } else {
            capture.output()
        };
        assert!(!output.contains("duet.conversation.failed"), "{output}");
        assert!(!output.contains("otel.status_code=\"ERROR\""), "{output}");
        assert!(!output.contains(FIRST), "{output}");
        assert!(!output.contains(SECOND), "{output}");
    }
    let sent = bodies.lock().unwrap().clone();
    assert_eq!(sent.len(), 2, "held request + one steered request");
    assert!(!sent[0].to_string().contains(SECOND));
    let input = sent[1]["input"].as_array().unwrap();
    for text in [FIRST, SECOND] {
        assert_eq!(
            input
                .iter()
                .filter(|item| item["role"] == "user" && item["content"] == text)
                .count(),
            1,
            "{text}"
        );
    }
    assert_eq!(
        count(
            &mut owner_db,
            "select count(distinct command_id) from duet_model_calls where user_id=$1",
            c.account_id
        )
        .await,
        1
    );
    assert_eq!(
        count(
            &mut owner_db,
            "select count(*) from duet_commands where user_id=$1",
            c.account_id
        )
        .await,
        1
    );
    server.abort();
    drop(owner_db);
    drop(app);
    teardown(admin, f).await;
}
