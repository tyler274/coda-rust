//! Push worker for accepted physician chat turns.
//!
//! Postgres owns the turn. Pub/Sub only wakes this process. The push body
//! carries a scope and an input id, never clinical text, and logs stay that way.
//! A duplicate or settled delivery is acknowledged without calling the model.
//! The worker loads the packaged skill catalog once at startup and offers
//! chat turns the read-only `skill_view` tool (book: `physician-site.md`,
//! "Skills in hosted chat").
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

use axum::Json;
use axum::Router;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use base64::Engine;
use harness::engine::{Cancellation, Item, Limits, Provider, Request as ModelRequest, Turn};
use local_product::physician_chat::{self, PhysicianSkills};
use local_product::runtime::async_trait;
use local_product::runtime::delivery::{Delivery, DeliveryKind, DeliveryState};

#[path = "physician-worker/attempt.rs"]
mod attempt;
#[path = "physician-worker/revision.rs"]
mod revision;
use attempt::{Claimed, Failure, Outcome, Push, Tally};
use local_product::runtime::{Claim, Conversation, PhysicianHost, Scope, Store};
use physician_auth::PostgresAuthStore;
use physician_bus::{Bus, Event};
use physician_sample::durable::RuntimeAuthority;
use physician_sample::hosting::{self, is_hosted};
use serde_json::{Value, json};
use telemetry::genai::{self, RunContext};
use tracing::Instrument;

const LEASE_SECONDS: i64 = 540;
const BODY_LIMIT: usize = 64 * 1024;

#[tokio::main]
async fn main() {
    if let Err(reason) = hosting::validate_startup(|key| std::env::var(key).ok()) {
        eprintln!("{reason}");
        std::process::exit(1);
    }
    let _telemetry = telemetry::init("physician-worker");
    if let Err(reason) = run().await {
        eprintln!("physician worker cannot start: {reason}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), String> {
    let worker = Arc::new(Worker::from_env(|key| std::env::var(key).ok()).await?);
    let address = listen_addr(&|key: &str| std::env::var(key).ok());
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .map_err(|_| format!("failed to bind {address}"))?;
    tracing::info!(
        "physician worker on http://{}",
        listener
            .local_addr()
            .map_err(|_| "physician worker address unavailable".to_owned())?
    );
    axum::serve(listener, router(worker))
        .with_graceful_shutdown(shutdown())
        .await
        .map_err(|_| "physician worker stopped".to_owned())
}

fn router(worker: Arc<Worker>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/readyz", get(readyz))
        .route("/pubsub/chat-turn", post(chat_turn))
        .layer(telemetry::http_layer())
        .with_state(worker)
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn readyz(State(worker): State<Arc<Worker>>) -> StatusCode {
    let probe = Scope {
        practice: "readyz".into(),
        physician: "readyz".into(),
        workspace: "readyz".into(),
        chat: "readyz".into(),
    };
    let runtime = worker.store.read(&probe).await;
    let auth = worker.auth.check_schema().await;
    if runtime.is_ok() && auth.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

async fn chat_turn(State(worker): State<Arc<Worker>>, request: Request) -> StatusCode {
    let header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    if let Err(status) = authorize_push(worker.verifier.as_ref(), header.as_deref()).await {
        return status;
    }
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST;
    };
    let Ok(push) = decode_push(&bytes) else {
        return StatusCode::BAD_REQUEST;
    };
    // Decode and dispatch only after verifying the push identity. Revision
    // scopes name reply work, never a chat conversation.
    if push.delivery.kind == DeliveryKind::ReplyRevision {
        return revision::deliver(&worker, &push).await;
    }
    let span = attempt::span(&push, CHAT_JOB);
    chat_attempt(&worker, &push.delivery, &span)
        .instrument(span.clone())
        .await
}

const CHAT_JOB: &str = "physician_chat";

/// One chat-turn delivery inside its `physician.worker.attempt` span.
async fn chat_attempt(worker: &Worker, delivery: &Delivery, span: &tracing::Span) -> StatusCode {
    let claim = match worker.store.claim_delivery(delivery, LEASE_SECONDS).await {
        Ok(Some(claim)) => claim,
        Ok(None) => {
            attempt::finish(span, CHAT_JOB, None, Outcome::Absent, None);
            return absent_claim(worker, delivery).await;
        }
        Err(_) => {
            attempt::finish(
                span,
                CHAT_JOB,
                None,
                Outcome::ClaimError,
                Some(Failure::Claim),
            );
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    let run_id = claim.attempt.clone();
    let workspace_id = claim.scope.workspace.clone();
    let session_id = claim.scope.chat.clone();
    let conversation = worker.store.read(&claim.scope).await.ok().flatten();
    let input_text = conversation
        .as_ref()
        .map(|state| last_text(state, "user"))
        .unwrap_or_default();
    let earlier_attempts = conversation
        .as_ref()
        .map_or(0, |state| earlier_attempts(state, &claim));
    let claimed = conversation
        .as_ref()
        .and_then(|state| state.attempts.iter().find(|a| a.id == claim.attempt));
    attempt::record_claim(
        span,
        &Claimed {
            run_id: &run_id,
            work_id: claimed.map(|a| a.work.as_str()),
            predecessor_run_id: claimed.and_then(|a| a.predecessor.as_deref()),
            input_revision: Some(claim.revision),
            attempt_number: earlier_attempts.saturating_add(1),
        },
    );
    drop(conversation);
    let generation = genai::generation_span(
        &RunContext {
            run_id: run_id.clone(),
            job: CHAT_JOB,
            recipe_version: genai::recipe_version(),
            origin: genai::Origin::Real,
            scenario_id: None,
            session_id: Some(session_id),
        },
        worker.system,
        &worker.model_name,
        worker.adapter,
    );
    genai::record_input(&generation, &input_text);
    let provider = worker.model.provider(earlier_attempts, &input_text);
    drop(input_text);
    let mut host = PhysicianHost {
        store: worker.store.clone(),
        claim,
        authority: RuntimeAuthority {
            store: worker.store.clone(),
        },
        instructions: worker.instructions.to_string(),
        skills: Some(worker.skills.clone()),
    };
    let cancel = Cancellation::default();
    let watcher = watch_currentness(worker.store.clone(), host.claim.clone(), cancel.clone());
    // One provider per turn: concurrent pushes on this instance run side by
    // side instead of queueing behind one another's model call.
    let mut provider = match provider {
        Ok(provider) => provider,
        Err(_) => {
            watcher.abort();
            genai::record_error(&generation, "physician chat provider unavailable");
            // Counted like a model failure, so the fifth one ends the turn.
            let status = failed_attempt(worker, span, &host.claim, workspace_id).await;
            attempt::finish(
                span,
                CHAT_JOB,
                Some(&run_id),
                Outcome::Retry,
                Some(Failure::ProviderInit),
            );
            return status;
        }
    };
    let result = host
        .run(&mut provider, &worker.limits, &cancel)
        .instrument(generation.clone())
        .await;
    watcher.abort();
    record_chat_usage(&worker.store, &host.claim, &generation).await;
    match result {
        Ok(()) => {
            let output = message_text(&worker.store, &host.claim.scope, "assistant").await;
            genai::record_output(&generation, &output);
            drop(output);
            // The turn is committed. Redelivery sees it settled and acks.
            announce(worker, workspace_id).await;
            attempt::finish(span, CHAT_JOB, Some(&run_id), Outcome::Committed, None);
            StatusCode::NO_CONTENT
        }
        Err(_) => {
            genai::record_error(&generation, "physician chat failed");
            if turn_settled(&worker.store, &host.claim).await {
                // Stop or a newer message already owns the turn. Ack so the
                // broker does not count this delivery as a failure.
                announce(worker, workspace_id).await;
                attempt::finish(span, CHAT_JOB, Some(&run_id), Outcome::Settled, None);
                StatusCode::NO_CONTENT
            } else {
                let status = failed_attempt(worker, span, &host.claim, workspace_id).await;
                attempt::finish(
                    span,
                    CHAT_JOB,
                    Some(&run_id),
                    Outcome::Retry,
                    Some(Failure::ModelCall),
                );
                status
            }
        }
    }
}

/// Records the provider-reported usage of this attempt's retained receipts.
/// The fixed provider and a provider that reported no usage leave it unset.
async fn record_chat_usage(store: &Store, claim: &Claim, generation: &tracing::Span) {
    let Ok(Some(state)) = store.read(&claim.scope).await else {
        return;
    };
    let usage = state
        .attempts
        .iter()
        .find(|a| a.id == claim.attempt && a.revision == claim.revision)
        .map(|a| Tally::from_events(&a.events))
        .and_then(|tally| tally.known());
    if let Some((input, output)) = usage {
        genai::record_usage(generation, input, output);
    }
}

/// Cancels the provider call once this claim is no longer the current attempt.
/// A database blip does not cancel: publication still refuses a stale result.
fn watch_currentness(
    store: Store,
    claim: Claim,
    cancel: Cancellation,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            match store.is_current(&claim).await {
                Ok(false) => {
                    cancel.cancel();
                    break;
                }
                Ok(true) | Err(_) => tokio::time::sleep(Duration::from_millis(100)).await,
            }
        }
    })
}

/// Whether this claim must not be retried: Stop held it, it committed, or a
/// newer input replaced it.
async fn turn_settled(store: &Store, claim: &Claim) -> bool {
    let Ok(Some(state)) = store.read(&claim.scope).await else {
        return false;
    };
    state.revision != claim.revision || matches!(state.status.as_str(), "completed" | "held")
}

/// One unsuccessful attempt, whether the provider could not start or the
/// model call failed. Records it (the fifth ends the turn as failed), tells
/// open pages to refetch, and refuses the push so the broker redelivers or
/// dead-letters it.
async fn failed_attempt(
    worker: &Worker,
    span: &tracing::Span,
    claim: &Claim,
    workspace_id: String,
) -> StatusCode {
    if worker.store.retry_delivery(claim).await.is_err() {
        attempt::retry_not_persisted(span, CHAT_JOB, &claim.attempt);
    }
    announce(worker, workspace_id).await;
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn absent_claim(worker: &Worker, delivery: &Delivery) -> StatusCode {
    let state = match worker.store.delivery_state(delivery).await {
        Ok(state) => state,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    if state == DeliveryState::Failed {
        // This claim may itself have ended the turn: an exhausted lease is
        // recorded as the last failure and leaves nothing to claim. Every
        // redelivery of a failed turn repeats the notice; a refetch is cheap.
        announce(worker, delivery.scope.workspace.clone()).await;
    }
    absent_status(state)
}

/// Tells every open page on this workspace to refetch. Sent after each
/// committed reply and each failed attempt, so a page holding a running turn
/// learns when it completed or ended as "couldn't finish". Lossy: a page that
/// misses it still converges on its next refetch or resync.
async fn announce(worker: &Worker, workspace_id: String) {
    if worker
        .bus
        .publish(Event::WorkspaceChanged { workspace_id })
        .await
        .is_err()
    {
        tracing::warn!("physician workspace invalidation not published");
    }
}

/// The broker response when this push could not claim its input. Settled,
/// superseded and live-claimed work is acknowledged. Runnable work is not, so
/// Pub/Sub redelivers it. Failed work is not either: Pub/Sub keeps counting
/// this message's deliveries and forwards it to the dead-letter topic, even
/// when the sweep republished the input under a new message ID.
fn absent_status(state: DeliveryState) -> StatusCode {
    match state {
        DeliveryState::Settled | DeliveryState::Claimed => StatusCode::NO_CONTENT,
        DeliveryState::Due | DeliveryState::Failed => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn message_text(store: &Store, scope: &Scope, role: &str) -> String {
    store
        .read(scope)
        .await
        .ok()
        .flatten()
        .map(|conversation| last_text(&conversation, role))
        .unwrap_or_default()
}

fn last_text(conversation: &Conversation, role: &str) -> String {
    conversation
        .messages
        .iter()
        .rev()
        .find(|message| message.role == role)
        .map(|message| message.text.clone())
        .unwrap_or_default()
}

/// Attempts saved before this claim on the same input. Try again keeps the
/// input revision, so the count spans redeliveries, the dead letter, Try again
/// and worker restarts; a new message starts a new revision at zero.
fn earlier_attempts(conversation: &Conversation, claim: &Claim) -> u32 {
    let count = conversation
        .attempts
        .iter()
        .filter(|attempt| attempt.revision == claim.revision && attempt.id != claim.attempt)
        .count();
    u32::try_from(count).unwrap_or(u32::MAX)
}

/// Decodes a Pub/Sub push body. Push delivers publisher attributes (where
/// the trace context travels) in the JSON body, not as HTTP headers.
fn decode_push(bytes: &[u8]) -> Result<Push, ()> {
    let envelope: Value = serde_json::from_slice(bytes).map_err(|_| ())?;
    let message = envelope.get("message").ok_or(())?;
    let data = message.get("data").and_then(Value::as_str).ok_or(())?;
    // Pub/Sub sends both messageId and message_id. deliveryAttempt only
    // shapes telemetry: a failed turn is never acked, including the fifth.
    let message_id = message
        .get("messageId")
        .and_then(Value::as_str)
        .or_else(|| message.get("message_id").and_then(Value::as_str))
        .filter(|id| !id.is_empty())
        .ok_or(())?
        .to_owned();
    let raw = base64::engine::general_purpose::STANDARD
        .decode(data.trim())
        .map_err(|_| ())?;
    let delivery: Delivery = serde_json::from_slice(&raw).map_err(|_| ())?;
    // Non-string attribute values are not Pub/Sub attributes; skip them.
    let attributes = message
        .get("attributes")
        .and_then(Value::as_object)
        .map(|attributes| {
            attributes
                .iter()
                .filter_map(|(key, value)| Some((key.clone(), value.as_str()?.to_owned())))
                .collect()
        })
        .unwrap_or_default();
    let delivery_attempt = envelope
        .get("deliveryAttempt")
        .or_else(|| envelope.get("delivery_attempt"))
        .and_then(Value::as_u64)
        .and_then(|attempt| u32::try_from(attempt).ok());
    Ok(Push {
        delivery,
        message_id,
        attributes,
        delivery_attempt,
    })
}

struct Worker {
    store: Store,
    auth: PostgresAuthStore,
    bus: Arc<dyn Bus>,
    model: ModelSource,
    verifier: Option<google_cloud_auth::credentials::idtoken::verifier::Verifier>,
    limits: Limits,
    system: &'static str,
    model_name: String,
    adapter: &'static str,
    /// The packaged catalog, loaded once at startup and shared by every turn.
    skills: PhysicianSkills,
    /// Versioned hosted instructions followed by that catalog's metadata.
    instructions: Arc<str>,
}

/// The packaged skill catalog. Hosted startup requires `PHYSICIAN_SKILLS_DIR`
/// (the image sets it); a local worker defaults to this workspace's
/// `skills/`. A missing or invalid catalog stops startup: hosted chat never
/// runs with an empty one.
fn skills_dir(get: &impl Fn(&str) -> Option<String>, hosted: bool) -> Result<PathBuf, String> {
    match configured(get, "PHYSICIAN_SKILLS_DIR") {
        Some(dir) => Ok(PathBuf::from(dir)),
        None if hosted => Err("hosted physician worker requires PHYSICIAN_SKILLS_DIR".into()),
        None => Ok(PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../skills"
        ))),
    }
}

impl Worker {
    async fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let hosted = is_hosted(&get);
        let fixed = configured(&get, "PHYSICIAN_WORKER_FIXED_TEXT");
        let failures = configured(&get, "PHYSICIAN_WORKER_FIXED_FAILURES");
        if hosted && fixed.is_some() {
            return Err("PHYSICIAN_WORKER_FIXED_TEXT is refused in hosted execution".into());
        }
        // Non-hosted proof only. A counted number of fixed-provider failures,
        // then the fixed text. Hosted execution refuses the variable itself.
        if hosted && failures.is_some() {
            return Err("PHYSICIAN_WORKER_FIXED_FAILURES is refused in hosted execution".into());
        }
        let failure_count = match failures {
            Some(value) => value
                .parse::<u32>()
                .map_err(|_| "PHYSICIAN_WORKER_FIXED_FAILURES must be a whole number".to_owned())?,
            None => 0,
        };
        if failure_count > 0 && fixed.is_none() {
            return Err(
                "PHYSICIAN_WORKER_FIXED_FAILURES requires PHYSICIAN_WORKER_FIXED_TEXT".into(),
            );
        }
        // Non-hosted proof only: how long each fixed reply takes, so a test can
        // show that concurrent pushes on one instance run side by side.
        let delay = configured(&get, "PHYSICIAN_WORKER_FIXED_DELAY_MS");
        if hosted && delay.is_some() {
            return Err("PHYSICIAN_WORKER_FIXED_DELAY_MS is refused in hosted execution".into());
        }
        let delay = match delay {
            Some(value) => Duration::from_millis(value.parse::<u64>().map_err(|_| {
                "PHYSICIAN_WORKER_FIXED_DELAY_MS must be a whole number".to_owned()
            })?),
            None => Duration::ZERO,
        };
        if !delay.is_zero() && fixed.is_none() {
            return Err(
                "PHYSICIAN_WORKER_FIXED_DELAY_MS requires PHYSICIAN_WORKER_FIXED_TEXT".into(),
            );
        }
        // Non-hosted proof only: a message containing `[fail:K]` fails its
        // first K attempts, counted from the attempts saved on that message, so one stack can drive a
        // retry, a dead letter and Try again alongside ordinary turns.
        let marker = configured(&get, "PHYSICIAN_WORKER_FIXED_FAIL_MARKER");
        if hosted && marker.is_some() {
            return Err("PHYSICIAN_WORKER_FIXED_FAIL_MARKER is refused in hosted execution".into());
        }
        let marker = match marker.as_deref() {
            None | Some("0") => false,
            Some("1") => true,
            Some(_) => return Err("PHYSICIAN_WORKER_FIXED_FAIL_MARKER must be 0 or 1".into()),
        };
        if marker && fixed.is_none() {
            return Err(
                "PHYSICIAN_WORKER_FIXED_FAIL_MARKER requires PHYSICIAN_WORKER_FIXED_TEXT".into(),
            );
        }
        // Non-hosted proof only: tool rounds the fixed provider requests
        // before its text, so a test can drive skill reads through the real
        // host and worker.
        let rounds = configured(&get, "PHYSICIAN_WORKER_FIXED_TOOL_ROUNDS");
        if hosted && rounds.is_some() {
            return Err("PHYSICIAN_WORKER_FIXED_TOOL_ROUNDS is refused in hosted execution".into());
        }
        if rounds.is_some() && fixed.is_none() {
            return Err(
                "PHYSICIAN_WORKER_FIXED_TOOL_ROUNDS requires PHYSICIAN_WORKER_FIXED_TEXT".into(),
            );
        }
        let rounds = Arc::new(match rounds {
            Some(script) => tool_rounds(&script)?,
            None => Vec::new(),
        });
        let execution = local_product::model_execution(
            configured(&get, "CODA_LOCAL_MODEL_EXECUTION").as_deref(),
        )?;
        if hosted && execution != local_product::ModelExecution::Api {
            return Err("hosted physician worker requires CODA_LOCAL_MODEL_EXECUTION=api".into());
        }
        // Before any connection: a worker without its catalog never serves.
        let skills = PhysicianSkills::load(&skills_dir(&get, hosted)?)?;
        let instructions: Arc<str> = skills.instructions(physician_chat::INSTRUCTIONS).into();
        let (model, system, model_name, adapter) = if let Some(text) = fixed {
            if failure_count > 0 {
                tracing::info!(
                    failure_count,
                    "physician worker fixed provider will fail before succeeding"
                );
            } else {
                tracing::info!("physician worker model execution: fixed text");
            }
            if marker {
                tracing::info!("physician worker fixed provider honors [fail:K] markers");
            }
            (
                ModelSource::Fixed {
                    text,
                    failures_remaining: Arc::new(std::sync::atomic::AtomicU32::new(failure_count)),
                    revision_failures: failure_count,
                    delay,
                    marker,
                    rounds,
                },
                "fixed",
                "fixed".to_owned(),
                "fixed",
            )
        } else if execution == local_product::ModelExecution::Api {
            let key = local_product::open_ai_api_key(
                configured(&get, "CODA_ALLOW_MODEL_CALLS").as_deref(),
                configured(&get, "OPENAI_API_KEY").as_deref(),
            )?;
            let owned = role_environment(&get);
            let environment: Vec<(&str, &str)> = owned
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str()))
                .collect();
            let roles = local_product::model_roles("openai-responses", &environment)?;
            let model_name = roles.chat.model.clone();
            // Fail at startup, not on the first push, when the binding is invalid.
            harness::responses::Responses::new(key.clone(), model_name.clone())?;
            tracing::info!("physician worker model execution: api");
            (
                ModelSource::Api {
                    key,
                    binding: roles.chat,
                },
                "openai",
                model_name,
                "openai-responses",
            )
        } else {
            return Err(
                "physician worker requires CODA_LOCAL_MODEL_EXECUTION=api or PHYSICIAN_WORKER_FIXED_TEXT"
                    .into(),
            );
        };
        let verifier = push_verifier(&get, hosted)?;
        let database = configured(&get, "PHYSICIAN_DATABASE_URL")
            .ok_or("PHYSICIAN_DATABASE_URL is required")?;
        let store = Store::connect(&database)
            .await
            .map_err(|_| "physician runtime store unavailable".to_string())?;
        let auth = PostgresAuthStore::connect_without_migrations(&database)
            .await
            .map_err(|_| "physician auth store unavailable".to_string())?;
        let bus = physician_bus::from_env(&get, hosted).await?;
        Ok(Self {
            store,
            auth,
            bus,
            model,
            verifier,
            limits: limits(),
            system,
            model_name,
            adapter,
            skills,
            instructions,
        })
    }
}

/// The push route's authentication gate runs before decoding or claiming work.
async fn authorize_push(
    verifier: Option<&google_cloud_auth::credentials::idtoken::verifier::Verifier>,
    header: Option<&str>,
) -> Result<(), StatusCode> {
    let Some(verifier) = verifier else {
        return Ok(());
    };
    let token = header
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    verifier
        .verify(token)
        .await
        .map(|_| ())
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

fn push_verifier(
    get: &impl Fn(&str) -> Option<String>,
    hosted: bool,
) -> Result<Option<google_cloud_auth::credentials::idtoken::verifier::Verifier>, String> {
    let audience = configured(get, "PHYSICIAN_WORKER_AUDIENCE");
    let email = configured(get, "PHYSICIAN_PUSH_SERVICE_ACCOUNT");
    if hosted {
        let audience =
            audience.ok_or("hosted physician worker requires PHYSICIAN_WORKER_AUDIENCE")?;
        let email =
            email.ok_or("hosted physician worker requires PHYSICIAN_PUSH_SERVICE_ACCOUNT")?;
        return Ok(Some(identity_verifier(audience, email).build()));
    }
    if configured(get, "PUBSUB_EMULATOR_HOST").is_some() {
        // The emulator sends no token, so the worker verifies none. That is
        // safe only when no one but this machine can reach the listener.
        let address = listen_addr(get);
        if !is_loopback_listener(&address) {
            return Err(format!(
                "PUBSUB_EMULATOR_HOST disables push verification, so the worker must listen on loopback; {address} is not"
            ));
        }
        return Ok(None);
    }
    match (audience, email) {
        (Some(audience), Some(email)) => Ok(Some(identity_verifier(audience, email).build())),
        _ => Err(
            "physician worker requires PHYSICIAN_WORKER_AUDIENCE and PHYSICIAN_PUSH_SERVICE_ACCOUNT, or PUBSUB_EMULATOR_HOST"
                .into(),
        ),
    }
}

fn identity_verifier(
    audience: String,
    email: String,
) -> google_cloud_auth::credentials::idtoken::verifier::Builder {
    google_cloud_auth::credentials::idtoken::verifier::Builder::new([audience]).with_email(email)
}

/// Bounds for one hosted chat attempt. The context bound matches the site's
/// in-process chat (`harness::execution::Limits::default`), so a conversation
/// that runs on the site also runs here. The reply bounds match Duet's chat.
/// The only tool is `skill_view`: up to eight model calls leave room for a
/// skill, its supporting resources and an answer. A single result may be as
/// large as the loader's largest budget (65536 bytes); four such results fill
/// the attempt's allowance. `elapsed` stays inside the 540 s lease and the
/// 600 s push deadline.
fn limits() -> Limits {
    Limits {
        model_calls: 8,
        tool_calls: 16,
        calls_per_turn: 8,
        request_bytes: 1_048_576,
        response_bytes: 131_072,
        tool_result_bytes: physician_chat::MAX_RESULT_BYTES as usize,
        total_tool_result_bytes: 4 * physician_chat::MAX_RESULT_BYTES as usize,
        output_tokens: 8192,
        elapsed: Duration::from_secs(480),
    }
}

/// Whether `address` binds only loopback. A name, a wildcard or any other
/// address is refused: it is checked as written, not after resolution.
fn is_loopback_listener(address: &str) -> bool {
    address
        .parse::<std::net::SocketAddr>()
        .is_ok_and(|socket| socket.ip().is_loopback())
}

fn listen_addr(get: &impl Fn(&str) -> Option<String>) -> String {
    configured(get, "PHYSICIAN_WORKER_LISTEN_ADDR").unwrap_or_else(|| {
        configured(get, "PORT")
            .map(|port| format!("0.0.0.0:{port}"))
            .unwrap_or_else(|| "127.0.0.1:8791".into())
    })
}

fn configured(get: &impl Fn(&str) -> Option<String>, key: &str) -> Option<String> {
    get(key)
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn role_environment(get: &impl Fn(&str) -> Option<String>) -> Vec<(String, String)> {
    const KEYS: [&str; 8] = [
        "CODA_CHAT_MODEL",
        "CODA_CHAT_EFFORT",
        "CODA_BACKGROUND_MODEL",
        "CODA_BACKGROUND_EFFORT",
        "CODA_WORKER_MODEL",
        "CODA_WORKER_EFFORT",
        "CODA_LINKING_MODEL",
        "CODA_LINKING_EFFORT",
    ];
    KEYS.into_iter()
        .filter_map(|key| configured(get, key).map(|value| (key.to_owned(), value)))
        .collect()
}

/// Builds one provider per turn. The key stays here and in each provider; it
/// is never logged or sent anywhere but the fixed Responses endpoint.
enum ModelSource {
    Api {
        key: String,
        binding: local_product::ModelBinding,
    },
    /// Non-hosted proof only. The failure count is shared by every turn on
    /// this process, so "fail N times, then answer" holds across redeliveries.
    Fixed {
        text: String,
        failures_remaining: Arc<std::sync::atomic::AtomicU32>,
        // Revision failures use durable attempt counts, including across restarts.
        revision_failures: u32,
        delay: Duration,
        marker: bool,
        rounds: Arc<Vec<ToolRound>>,
    },
}

/// One scripted provider turn: `(tool name, raw JSON arguments)` per call.
type ToolRound = Vec<(String, String)>;

/// Parses `PHYSICIAN_WORKER_FIXED_TOOL_ROUNDS`: a JSON array of rounds, each
/// an array of `{"name": ..., "arguments": ...}`. Object arguments are sent
/// as their JSON; a string is sent as written, so a test can send bytes
/// that are not JSON.
fn tool_rounds(script: &str) -> Result<Vec<ToolRound>, String> {
    let invalid = || "PHYSICIAN_WORKER_FIXED_TOOL_ROUNDS must be a JSON array of call arrays";
    let rounds: Vec<Vec<Value>> = serde_json::from_str(script).map_err(|_| invalid())?;
    rounds
        .into_iter()
        .map(|round| {
            round
                .into_iter()
                .map(|call| {
                    let name = call
                        .get("name")
                        .and_then(Value::as_str)
                        .ok_or_else(invalid)?;
                    let arguments = match call.get("arguments") {
                        Some(Value::String(raw)) => raw.clone(),
                        Some(value @ Value::Object(_)) => value.to_string(),
                        _ => return Err(invalid().to_owned()),
                    };
                    Ok((name.to_owned(), arguments))
                })
                .collect::<Result<ToolRound, String>>()
                .and_then(|round| {
                    if round.is_empty() {
                        Err(invalid().to_owned())
                    } else {
                        Ok(round)
                    }
                })
        })
        .collect()
}

impl ModelSource {
    /// One provider for one attempt on a physician message `input`, after
    /// `earlier_attempts` saved attempts on that input. Only the fixed
    /// provider reads either, for its fail marker.
    fn provider(&self, earlier_attempts: u32, input: &str) -> Result<Model, String> {
        match self {
            Self::Api { key, binding } => Ok(Model::Api(
                harness::responses::Responses::new(key.clone(), binding.model.clone())?
                    .reasoning_effort(binding.effort.clone()),
            )),
            Self::Fixed {
                text,
                failures_remaining,
                delay,
                marker,
                rounds,
                ..
            } => Ok(Model::Fixed {
                text: text.clone(),
                failures_remaining: failures_remaining.clone(),
                delay: *delay,
                marked_failure: *marker && marked_attempt_fails(earlier_attempts, input),
                rounds: rounds.clone(),
            }),
        }
    }
}

/// Whether the attempt after `earlier_attempts` fails for a message marked
/// `[fail:K]`: the first K attempts fail. The count is the conversation's
/// saved attempts on this input, so it survives a worker restart and every
/// worker instance agrees. Try again keeps the input, so `[fail:5]`
/// dead-letters on its fifth failure and its Try again then succeeds.
fn marked_attempt_fails(earlier_attempts: u32, input: &str) -> bool {
    marked_failures(input).is_some_and(|failures| earlier_attempts < failures)
}

/// Takes one from `count` unless it is zero; true when it took one. An
/// explicit compare-exchange loop: `fetch_update` is deprecated on newer
/// stable (renamed `try_update`), which the dev shell's toolchain predates.
fn take_one(count: &std::sync::atomic::AtomicU32) -> bool {
    let mut left = count.load(Ordering::SeqCst);
    while let Some(next) = left.checked_sub(1) {
        match count.compare_exchange_weak(left, next, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return true,
            Err(actual) => left = actual,
        }
    }
    false
}

/// K from the first `[fail:K]` in `input`, if K is a whole number.
fn marked_failures(input: &str) -> Option<u32> {
    let (_, rest) = input.split_once("[fail:")?;
    let (count, _) = rest.split_once(']')?;
    count.parse().ok()
}

enum Model {
    Api(harness::responses::Responses),
    Fixed {
        text: String,
        failures_remaining: Arc<std::sync::atomic::AtomicU32>,
        delay: Duration,
        marked_failure: bool,
        rounds: Arc<Vec<ToolRound>>,
    },
}

/// The scripted turn for request `ordinal`: narration plus that round's
/// calls. Every call of the earlier rounds must already have its result in
/// this request, or the fixed provider fails the attempt.
fn scripted_round(
    rounds: &[ToolRound],
    request: &ModelRequest<'_>,
) -> Result<Option<Turn>, String> {
    let ordinal = request.ordinal as usize;
    let answered = |id: &str| {
        request
            .context
            .items
            .iter()
            .any(|item| matches!(item, Item::Result { call_id, .. } if call_id == id))
    };
    for (round, calls) in rounds.iter().enumerate().take(ordinal - 1) {
        for index in 0..calls.len() {
            if !answered(&fixed_call_id(round + 1, index)) {
                return Err("fixed tool result missing from the next request".into());
            }
        }
    }
    let Some(calls) = rounds.get(ordinal - 1) else {
        return Ok(None);
    };
    let mut items = vec![Item::Message {
        role: "assistant".into(),
        text: format!("Fixed narration before tool round {ordinal}. "),
    }];
    items.extend(calls.iter().enumerate().map(|(index, (name, arguments))| {
        Item::Call(harness::engine::Invocation {
            id: fixed_call_id(ordinal, index),
            name: name.clone(),
            arguments: arguments.clone(),
        })
    }));
    Ok(Some(Turn {
        items,
        receipt: json!({ "provider": "fixed", "round": ordinal }),
    }))
}

fn fixed_call_id(round: usize, index: usize) -> String {
    format!("fixed-{round}-{index}")
}

#[async_trait]
impl Provider for Model {
    fn request_bytes(&self, request: &ModelRequest<'_>) -> Result<usize, String> {
        match self {
            Self::Api(provider) => provider.request_bytes(request),
            Self::Fixed { .. } => Ok(1),
        }
    }

    async fn complete(&mut self, request: ModelRequest<'_>) -> Result<Turn, String> {
        if let Self::Fixed { delay, .. } = self
            && !delay.is_zero()
        {
            tokio::time::sleep(*delay).await;
        }
        match self {
            Self::Api(provider) => provider.complete(request).await,
            Self::Fixed {
                marked_failure: true,
                ..
            } => Err("fixed physician reply failed by marker".into()),
            Self::Fixed {
                failures_remaining, ..
            } if take_one(failures_remaining) => Err("fixed physician reply failed".into()),
            Self::Fixed { text, rounds, .. } => match scripted_round(rounds, &request)? {
                Some(turn) => Ok(turn),
                None => Ok(Turn {
                    items: vec![Item::Message {
                        role: "assistant".into(),
                        text: text.clone(),
                    }],
                    receipt: json!({ "provider": "fixed" }),
                }),
            },
        }
    }
}

async fn shutdown() {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = terminate.recv() => {},
        }
    }
    #[cfg(not(unix))]
    tokio::signal::ctrl_c().await.expect("shutdown signal");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use base64::Engine;
    use serde_json::json;

    #[tokio::test]
    async fn signed_push_tokens_enforce_audience_issuer_email_and_expiry() {
        use jsonwebtoken::{Algorithm, EncodingKey, Header};
        // This publicly committed key has no authority outside this test's
        // loopback JWKS endpoint. Production always uses Google's endpoint.
        let key = EncodingKey::from_rsa_pem(include_bytes!(
            "../../tests/fixtures/pubsub-test-only-key.pem"
        ))
        .unwrap();
        let jwks: Value = serde_json::from_str(include_str!(
            "../../tests/fixtures/pubsub-test-only-jwks.json"
        ))
        .unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/certs", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            axum::serve(
                listener,
                Router::new().route(
                    "/certs",
                    get(move || {
                        let jwks = jwks.clone();
                        async move { Json(jwks) }
                    }),
                ),
            )
            .await
            .unwrap();
        });
        let audience = "https://physician-worker.example.test";
        let email = "physician-push@example.test";
        let verifier = identity_verifier(audience.into(), email.into())
            .with_jwks_url(endpoint)
            .build();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let claims = json!({"iss":"https://accounts.google.com","aud":audience,
            "sub":"synthetic-service-account","email":email,"email_verified":true,
            "iat":now-60,"exp":now+600});
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some("synthetic-pubsub-proof".into());
        let token = |claims: &Value| {
            format!(
                "Bearer {}",
                jsonwebtoken::encode(&header, claims, &key).unwrap()
            )
        };
        // A valid signed control proves the negative cases are not merely
        // failing key retrieval, signature verification, or token parsing.
        assert_eq!(
            authorize_push(Some(&verifier), Some(&token(&claims))).await,
            Ok(())
        );
        for (field, value) in [
            ("aud", json!("https://another-worker.example.test")),
            ("iss", json!("https://untrusted-issuer.example.test")),
            ("email", json!("another-account@example.test")),
            ("email_verified", json!(false)),
            ("exp", json!(now - 60)),
        ] {
            let mut invalid = claims.clone();
            invalid[field] = value;
            assert_eq!(
                authorize_push(Some(&verifier), Some(&token(&invalid))).await,
                Err(StatusCode::UNAUTHORIZED),
                "invalid {field} must be refused before work is claimed"
            );
        }
        assert_eq!(
            authorize_push(Some(&verifier), None).await,
            Err(StatusCode::UNAUTHORIZED)
        );
        server.abort();
    }

    #[test]
    fn settled_and_live_claims_ack_while_due_and_failed_work_does_not() {
        assert_eq!(
            absent_status(DeliveryState::Settled),
            StatusCode::NO_CONTENT
        );
        assert_eq!(
            absent_status(DeliveryState::Claimed),
            StatusCode::NO_CONTENT
        );
        // Not acknowledging failed work is what lets Pub/Sub dead-letter it.
        for state in [DeliveryState::Due, DeliveryState::Failed] {
            assert_eq!(absent_status(state), StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    #[test]
    fn push_body_decodes_standard_base64_delivery_without_requiring_one_id_field() {
        let delivery = r#"{"scope":{"practice":"p","physician":"d","workspace":"w","chat":"c"},"input_id":"k"}"#;
        let data = base64::engine::general_purpose::STANDARD.encode(delivery);
        let body = json!({
            "message": {"data": data, "messageId": "m1", "message_id": "m1"},
            "deliveryAttempt": 2
        });
        let push = decode_push(body.to_string().as_bytes()).unwrap();
        assert_eq!(push.message_id, "m1");
        assert_eq!(push.delivery.input_id, "k");
        assert_eq!(push.delivery.scope.workspace, "w");
        assert_eq!(push.delivery.kind, DeliveryKind::ChatTurn);
        assert_eq!(push.delivery_attempt, Some(2));
        assert!(push.attributes.is_empty());
        let mut revision = push.delivery;
        revision.kind = DeliveryKind::ReplyRevision;
        let data = base64::engine::general_purpose::STANDARD
            .encode(serde_json::to_vec(&revision).unwrap());
        let body = json!({"message": {"data": data, "message_id": "revision-message"}});
        let push = decode_push(body.to_string().as_bytes()).unwrap();
        assert_eq!(push.delivery.kind, DeliveryKind::ReplyRevision);
        assert_eq!(push.delivery.input_id, "k");
        assert_eq!(push.message_id, "revision-message");
        assert_eq!(push.delivery_attempt, None);
        assert!(decode_push(b"{}").is_err());
        assert!(decode_push(br#"{"message":{"data":"not base64","messageId":"m"}}"#).is_err());
        let bad = base64::engine::general_purpose::STANDARD.encode("{}");
        let bad_body = json!({"message": {"data": bad, "messageId": "m"}});
        assert!(decode_push(bad_body.to_string().as_bytes()).is_err());
    }

    #[test]
    fn push_body_carries_trace_attributes_and_the_delivery_attempt() {
        let delivery = r#"{"scope":{"practice":"p","physician":"d","workspace":"w","chat":"c"},"input_id":"k"}"#;
        let data = base64::engine::general_purpose::STANDARD.encode(delivery);
        let traceparent = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
        let body = json!({
            "message": {
                "data": data,
                "messageId": "m1",
                "attributes": {"traceparent": traceparent, "tracestate": "dd=s:1", "ignored": 7}
            },
            "subscription": "projects/p/subscriptions/s",
            "deliveryAttempt": 3
        });
        let push = decode_push(body.to_string().as_bytes()).unwrap();
        assert_eq!(push.delivery_attempt, Some(3));
        assert_eq!(
            push.attributes.get("traceparent").map(String::as_str),
            Some(traceparent)
        );
        assert_eq!(
            push.attributes.get("tracestate").map(String::as_str),
            Some("dd=s:1")
        );
        // Non-string values are not Pub/Sub attributes.
        assert!(!push.attributes.contains_key("ignored"));
        let remote = telemetry::propagation::remote_context(&push.attributes).unwrap();
        // A third delivery links to the publisher instead of continuing it.
        assert_eq!(
            attempt::relation(true, push.delivery_attempt),
            telemetry::propagation::Relation::Link
        );
        assert!(remote.is_remote());
        // An absent attempt count does not establish first delivery, and
        // attributes may be absent entirely.
        let body = json!({"message": {"data": data, "messageId": "m2", "attributes": null}});
        let push = decode_push(body.to_string().as_bytes()).unwrap();
        assert!(push.attributes.is_empty());
        assert_eq!(push.delivery_attempt, None);
    }

    #[test]
    fn startup_refuses_a_hosted_fixed_reply_and_an_unverified_local_worker() {
        let hosted = |name: &str| match name {
            "K_SERVICE" => Some("physician-worker".into()),
            "PHYSICIAN_WORKER_FIXED_TEXT" => Some("Noted.".into()),
            "CODA_LOCAL_MODEL_EXECUTION" => Some("api".into()),
            _ => None,
        };
        match block(Worker::from_env(hosted)) {
            Err(error) => assert!(error.contains("PHYSICIAN_WORKER_FIXED_TEXT"), "{error}"),
            Ok(_) => panic!("hosted fixed text must refuse to start"),
        }
        let local = |name: &str| (name == "PHYSICIAN_WORKER_FIXED_TEXT").then(|| "Noted.".into());
        match block(Worker::from_env(local)) {
            Err(error) => assert!(error.contains("PUBSUB_EMULATOR_HOST"), "{error}"),
            Ok(_) => panic!("a worker without push verification must refuse to start"),
        }
        let hosted_failures = |name: &str| match name {
            "K_SERVICE" => Some("physician-worker".into()),
            "PHYSICIAN_WORKER_FIXED_FAILURES" => Some("2".into()),
            "CODA_LOCAL_MODEL_EXECUTION" => Some("api".into()),
            _ => None,
        };
        match block(Worker::from_env(hosted_failures)) {
            Err(error) => {
                assert_eq!(
                    error,
                    "PHYSICIAN_WORKER_FIXED_FAILURES is refused in hosted execution"
                );
            }
            Ok(_) => panic!("hosted fixed failures must refuse to start"),
        }
        let hosted_marker = |name: &str| match name {
            "K_SERVICE" => Some("physician-worker".into()),
            "PHYSICIAN_WORKER_FIXED_FAIL_MARKER" => Some("1".into()),
            "CODA_LOCAL_MODEL_EXECUTION" => Some("api".into()),
            _ => None,
        };
        match block(Worker::from_env(hosted_marker)) {
            Err(error) => {
                // Exactly the hosted refusal: the "requires fixed text" error
                // also names the variable.
                assert_eq!(
                    error,
                    "PHYSICIAN_WORKER_FIXED_FAIL_MARKER is refused in hosted execution"
                );
            }
            Ok(_) => panic!("a hosted fail marker must refuse to start"),
        }
        let marker_without_text =
            |name: &str| (name == "PHYSICIAN_WORKER_FIXED_FAIL_MARKER").then(|| "1".into());
        match block(Worker::from_env(marker_without_text)) {
            Err(error) => assert!(error.contains("requires PHYSICIAN_WORKER_FIXED_TEXT")),
            Ok(_) => panic!("a fail marker without fixed text must refuse to start"),
        }
    }

    #[test]
    fn startup_requires_a_complete_skill_catalog() {
        // Hosted startup names the variable rather than using the source tree.
        let hosted = |name: &str| match name {
            "K_SERVICE" => Some("physician-worker".into()),
            "CODA_LOCAL_MODEL_EXECUTION" => Some("api".into()),
            _ => None,
        };
        match block(Worker::from_env(hosted)) {
            Err(error) => assert_eq!(
                error,
                "hosted physician worker requires PHYSICIAN_SKILLS_DIR"
            ),
            Ok(_) => panic!("a hosted worker without its catalog must refuse to start"),
        }
        // A configured directory without a catalog stops startup before any
        // connection; there is no empty fallback.
        let missing = std::env::temp_dir().join(format!(
            "physician-worker-missing-skills-{}",
            std::process::id()
        ));
        let missing_text = missing.display().to_string();
        let local_missing = |name: &str| match name {
            "PHYSICIAN_WORKER_FIXED_TEXT" => Some("Noted.".into()),
            "PHYSICIAN_SKILLS_DIR" => Some(missing_text.clone()),
            _ => None,
        };
        match block(Worker::from_env(local_missing)) {
            Err(error) => {
                assert!(error.starts_with("physician skill catalog at"), "{error}");
                assert!(error.contains(&missing_text), "{error}");
            }
            Ok(_) => panic!("a missing catalog must refuse to start"),
        }
        // The workspace catalog loads and reaches the push-verification check.
        let local = |name: &str| (name == "PHYSICIAN_WORKER_FIXED_TEXT").then(|| "Noted.".into());
        match block(Worker::from_env(local)) {
            Err(error) => assert!(error.contains("PUBSUB_EMULATOR_HOST"), "{error}"),
            Ok(_) => panic!("a worker without push verification must refuse to start"),
        }
    }

    #[test]
    fn scripted_tool_rounds_are_proof_only_and_parse_strictly() {
        let hosted = |name: &str| match name {
            "K_SERVICE" => Some("physician-worker".into()),
            "PHYSICIAN_WORKER_FIXED_TOOL_ROUNDS" => Some("[]".into()),
            "CODA_LOCAL_MODEL_EXECUTION" => Some("api".into()),
            _ => None,
        };
        match block(Worker::from_env(hosted)) {
            Err(error) => assert_eq!(
                error,
                "PHYSICIAN_WORKER_FIXED_TOOL_ROUNDS is refused in hosted execution"
            ),
            Ok(_) => panic!("hosted scripted rounds must refuse to start"),
        }
        let without_text =
            |name: &str| (name == "PHYSICIAN_WORKER_FIXED_TOOL_ROUNDS").then(|| "[]".into());
        match block(Worker::from_env(without_text)) {
            Err(error) => assert!(error.contains("requires PHYSICIAN_WORKER_FIXED_TEXT")),
            Ok(_) => panic!("scripted rounds without fixed text must refuse to start"),
        }
        let rounds = tool_rounds(
            r#"[[{"name":"skill_view","arguments":{"name":"a","file_path":null,"max_result_bytes":4096}},
                 {"name":"skill_view","arguments":"not json"}],
                [{"name":"other","arguments":{}}]]"#,
        )
        .unwrap();
        assert_eq!(rounds.len(), 2);
        assert_eq!(rounds[0][1], ("skill_view".into(), "not json".into()));
        assert_eq!(rounds[1][0], ("other".into(), "{}".into()));
        for bad in [
            "{}",
            "[[]]",
            r#"[[{"arguments":{}}]]"#,
            r#"[[{"name":"x"}]]"#,
        ] {
            assert!(tool_rounds(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn fixed_failures_count_down_to_zero_and_stop() {
        let count = std::sync::atomic::AtomicU32::new(2);
        assert!(take_one(&count));
        assert!(take_one(&count));
        assert!(!take_one(&count));
        assert!(!take_one(&count));
        assert_eq!(count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn fail_marker_parses_whole_counts_only() {
        assert_eq!(marked_failures("Retry [fail:1]"), Some(1));
        assert_eq!(marked_failures("[fail:5] then text"), Some(5));
        assert_eq!(marked_failures("Plain turn"), None);
        assert_eq!(marked_failures("[fail:]"), None);
        assert_eq!(marked_failures("[fail:-1]"), None);
        assert_eq!(marked_failures("[fail:2"), None);
        assert_eq!(marked_failures("[fail:x]"), None);
    }

    #[test]
    fn fail_marker_fails_the_first_k_saved_attempts() {
        // `[fail:2]` fails its first two attempts, then succeeds for good.
        assert!(marked_attempt_fails(0, "Retry [fail:2]"));
        assert!(marked_attempt_fails(1, "Retry [fail:2]"));
        assert!(!marked_attempt_fails(2, "Retry [fail:2]"));
        assert!(!marked_attempt_fails(3, "Retry [fail:2]"));
        // An unmarked turn never fails, whatever its history.
        assert!(!marked_attempt_fails(0, "Plain turn"));
        assert!(!marked_attempt_fails(4, "Plain turn"));
        assert!(!marked_attempt_fails(0, "[fail:0]"));
    }

    fn attempt(id: &str, revision: u64) -> local_product::runtime::Attempt {
        local_product::runtime::Attempt {
            id: id.into(),
            predecessor: None,
            work: "work".into(),
            revision,
            status: "failed".into(),
            expires: 0,
            events: vec![],
        }
    }

    #[test]
    fn earlier_attempts_count_only_this_input_and_never_this_claim() {
        let scope = Scope {
            practice: "p".into(),
            physician: "d".into(),
            workspace: "w".into(),
            chat: "c".into(),
        };
        let mut conversation = Conversation {
            revision: 2,
            ..Conversation::default()
        };
        // One attempt on the previous message, two failed ones on this one,
        // then this claim's own attempt.
        conversation.attempts = vec![
            attempt("old", 1),
            attempt("a1", 2),
            attempt("a2", 2),
            attempt("now", 2),
        ];
        let claim = Claim {
            scope,
            attempt: "now".into(),
            revision: 2,
        };
        assert_eq!(earlier_attempts(&conversation, &claim), 2);
        conversation.attempts.truncate(1);
        assert_eq!(earlier_attempts(&conversation, &claim), 0);
    }

    #[test]
    fn emulator_bypass_requires_a_loopback_listener() {
        let emulator = |listen: Option<&'static str>, port: Option<&'static str>| {
            move |name: &str| match name {
                "PUBSUB_EMULATOR_HOST" => Some("127.0.0.1:8085".to_owned()),
                "PHYSICIAN_WORKER_LISTEN_ADDR" => listen.map(str::to_owned),
                "PORT" => port.map(str::to_owned),
                _ => None,
            }
        };
        // The default and explicit loopback listeners keep the bypass.
        for (listen, port) in [
            (None, None),
            (Some("127.0.0.1:8791"), None),
            (Some("[::1]:8791"), None),
            (Some("127.0.0.1:8791"), Some("8080")),
        ] {
            let verifier = push_verifier(&emulator(listen, port), false);
            assert!(matches!(verifier, Ok(None)), "{listen:?} {port:?}");
        }
        // PORT alone binds every interface, as do wildcards and named or
        // external listeners. Each refuses to start without verification.
        for (listen, port) in [
            (None, Some("8080")),
            (Some("0.0.0.0:8791"), None),
            (Some("[::]:8791"), None),
            (Some("10.0.0.5:8791"), None),
            (Some("localhost:8791"), None),
        ] {
            match push_verifier(&emulator(listen, port), false) {
                Err(error) => assert!(error.contains("loopback"), "{error}"),
                Ok(_) => panic!("{listen:?} {port:?} must refuse the emulator bypass"),
            }
        }
    }

    fn block<T>(future: impl std::future::Future<Output = T>) -> T {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(future)
    }
}
