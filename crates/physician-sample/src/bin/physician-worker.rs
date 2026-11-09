//! Pull worker for accepted physician chat turns and reply revisions.
//!
//! Postgres owns the work. The worker listens for the notification the
//! accepting transaction sends and polls the due work, so a lost notification
//! only delays a turn (`pull`). A delivery names a scope and an input id,
//! never clinical text, and logs stay that way. Work that is settled or that
//! another worker holds is not claimed, so the model is not called for it.
//! A claim is a short lease this worker keeps renewing while it runs the
//! attempt and hands back when it is told to stop. The HTTP listener serves
//! health and readiness only. The worker loads the packaged skill catalog
//! once at startup and offers chat turns the read-only `skill_view` tool
//! (book: `physician-site.md`, "Skills in hosted chat").
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
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use harness::engine::{Cancellation, Item, Limits, Provider, Request as ModelRequest, Turn};
use local_product::physician_chat::{self, PhysicianSkills};
use local_product::runtime::async_trait;
use local_product::runtime::delivery::{Backoff, Delivery, DeliveryKind, DeliveryState};

#[path = "physician-worker/attempt.rs"]
mod attempt;
#[path = "physician-worker/pull.rs"]
mod pull;
#[path = "physician-worker/revision.rs"]
mod revision;
use attempt::{Claimed, Failure, Next, Outcome, Tally};
use local_product::runtime::{Claim, Conversation, PhysicianHost, Scope, Store};
use physician_auth::PostgresAuthStore;
use physician_bus::{Bus, Event};
use physician_sample::durable::RuntimeAuthority;
use physician_sample::hosting::{self, SimulationPractices, is_hosted};
use serde_json::{Value, json};
use telemetry::genai::{self, RunContext};
use tokio::sync::watch;
use tracing::Instrument;

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
    let (stop, stopped) = watch::channel(false);
    let work = tokio::spawn(pull::run(Arc::clone(&worker), stopped));
    let served = axum::serve(listener, router(worker))
        .with_graceful_shutdown(async move {
            shutdown().await;
            // The pull loop and every running attempt watch this.
            stop.send_replace(true);
        })
        .await;
    work.await
        .map_err(|_| "physician worker pull loop ended abnormally".to_owned())?;
    served.map_err(|_| "physician worker stopped".to_owned())
}

fn router(worker: Arc<Worker>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/readyz", get(readyz))
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

/// Claims and runs the work `delivery` names, if it is this worker's to run.
async fn run_delivery(worker: &Worker, delivery: &Delivery, stop: watch::Receiver<bool>) -> Next {
    match delivery.kind {
        DeliveryKind::ChatTurn => chat_turn(worker, delivery, stop).await,
        DeliveryKind::ReplyRevision => revision::deliver(worker, delivery, stop).await,
    }
}

const CHAT_JOB: &str = "physician_chat";

async fn chat_turn(worker: &Worker, delivery: &Delivery, stop: watch::Receiver<bool>) -> Next {
    let claim = match worker
        .store
        .claim_delivery(delivery, worker.pace.lease)
        .await
    {
        Ok(Some(claim)) => claim,
        Ok(None) => {
            absent_claim(worker, delivery).await;
            return Next::Done;
        }
        Err(_) => {
            let span = attempt::span(DeliveryKind::ChatTurn, CHAT_JOB, None);
            attempt::finish(
                &span,
                CHAT_JOB,
                None,
                Outcome::ClaimError,
                Some(Failure::Claim),
            );
            return Next::Done;
        }
    };
    let span = attempt::span(DeliveryKind::ChatTurn, CHAT_JOB, claim.origin.as_ref());
    chat_attempt(worker, claim, &span, stop)
        .instrument(span.clone())
        .await
}

/// One claimed chat turn inside its `physician.worker.attempt` span.
async fn chat_attempt(
    worker: &Worker,
    claim: Claim,
    span: &tracing::Span,
    stop: watch::Receiver<bool>,
) -> Next {
    let run_id = claim.attempt.clone();
    let workspace_id = claim.scope.workspace.clone();
    let session_id = claim.scope.chat.clone();
    let conversation = worker.store.read(&claim.scope).await.ok().flatten();
    let (origin, scenario_id) = worker.simulation_practices.origin(
        &claim.scope.practice,
        conversation
            .as_ref()
            .and_then(|state| state.scenario.as_deref()),
    );
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
            origin,
            scenario_id,
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
    // One provider per turn: concurrent turns on this instance run side by
    // side instead of queueing behind one another's model call.
    let mut provider = match provider {
        Ok(provider) => provider,
        Err(_) => {
            genai::record_error(&generation, "physician chat provider unavailable");
            // Counted like a model failure, so the fifth one ends the turn.
            let (outcome, next) = failed_attempt(worker, span, &host.claim, workspace_id).await;
            attempt::finish(
                span,
                CHAT_JOB,
                Some(&run_id),
                outcome,
                Some(Failure::ProviderInit),
            );
            return next;
        }
    };
    let cancel = Cancellation::default();
    let watcher = watch_currentness(worker.store.clone(), host.claim.clone(), cancel.clone());
    let lease = keep_lease(worker.store.clone(), host.claim.clone(), worker.pace);
    let result = tokio::select! {
        result = host
            .run(&mut provider, &worker.limits, &cancel)
            .instrument(generation.clone()) => Some(result),
        () = pull::stopping(stop) => None,
    };
    watcher.abort();
    lease.abort();
    let Some(result) = result else {
        // This worker is stopping: another one runs the turn at once.
        cancel.cancel();
        genai::record_error(&generation, "physician worker stopping");
        if worker.store.release(&host.claim).await.is_err() {
            tracing::warn!("physician worker claim not handed back; its lease will run out");
        }
        attempt::finish(span, CHAT_JOB, Some(&run_id), Outcome::Released, None);
        return Next::Done;
    };
    record_chat_usage(&worker.store, &host.claim, &generation).await;
    match result {
        Ok(()) => {
            let output = message_text(&worker.store, &host.claim.scope, "assistant").await;
            genai::record_output(&generation, &output);
            drop(output);
            announce(worker, workspace_id).await;
            attempt::finish(span, CHAT_JOB, Some(&run_id), Outcome::Committed, None);
            Next::Done
        }
        Err(_) => {
            genai::record_error(&generation, "physician chat failed");
            if turn_settled(&worker.store, &host.claim).await {
                // Stop or a newer message already owns the turn: not a failure.
                announce(worker, workspace_id).await;
                attempt::finish(span, CHAT_JOB, Some(&run_id), Outcome::Settled, None);
                Next::Done
            } else {
                let (outcome, next) = failed_attempt(worker, span, &host.claim, workspace_id).await;
                attempt::finish(
                    span,
                    CHAT_JOB,
                    Some(&run_id),
                    outcome,
                    Some(Failure::ModelCall),
                );
                next
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

/// Renews this claim's lease while its attempt runs. A renewal the database
/// refused is tried again on the next beat; a claim that stopped being
/// current is cancelled by [`watch_currentness`].
fn keep_lease(store: Store, claim: Claim, pace: Pace) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut beats = tokio::time::interval(pace.heartbeat());
        beats.tick().await;
        loop {
            beats.tick().await;
            if store.heartbeat(&claim, pace.lease).await.is_err() {
                tracing::debug!("physician worker lease not renewed");
            }
        }
    })
}

/// One unsuccessful attempt, whether the provider could not start or the
/// model call failed. Records it (the fifth ends the turn as failed) with the
/// delay before the next one, and tells open pages to refetch.
async fn failed_attempt(
    worker: &Worker,
    span: &tracing::Span,
    claim: &Claim,
    workspace_id: String,
) -> (Outcome, Next) {
    let saved = worker
        .store
        .retry_delivery(claim, worker.pace.backoff)
        .await;
    let ended = attempt::retried(span, CHAT_JOB, &claim.attempt, saved);
    announce(worker, workspace_id).await;
    ended
}

/// A delivery that had nothing to claim. The claim may itself have ended the
/// work: an exhausted lease is recorded as the last failure and leaves
/// nothing to run, so open pages are told to refetch. Hearing of failed work
/// again repeats the notice; a refetch is cheap.
async fn absent_claim(worker: &Worker, delivery: &Delivery) {
    match worker.store.delivery_state(delivery).await {
        Ok(DeliveryState::Failed) => announce(worker, delivery.scope.workspace.clone()).await,
        Ok(DeliveryState::Settled | DeliveryState::Claimed | DeliveryState::Due) => {}
        Err(_) => tracing::warn!("physician work state unreadable after an empty claim"),
    }
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
/// input revision, so the count spans retries, Try again and worker
/// restarts; a new message starts a new revision at zero.
fn earlier_attempts(conversation: &Conversation, claim: &Claim) -> u32 {
    let count = conversation
        .attempts
        .iter()
        .filter(|attempt| attempt.revision == claim.revision && attempt.id != claim.attempt)
        .count();
    u32::try_from(count).unwrap_or(u32::MAX)
}

/// How this worker paces its work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pace {
    /// Time between reads of the due work.
    poll: Duration,
    /// Seconds a claim lasts unless renewed. A worker that dies leaves its
    /// turn to another worker within this long.
    lease: i64,
    /// Attempts this process runs at once.
    concurrency: usize,
    /// The wait before failed work is due again.
    backoff: Backoff,
}

impl Pace {
    /// Hosted pacing: the poll bounds how late a lost notification is found.
    /// Attempts take a connection only for each statement, so four leave the
    /// store's eight-connection pool room for the listener and renewals.
    const HOSTED: Self = Self {
        poll: Duration::from_secs(10),
        lease: 60,
        concurrency: 4,
        backoff: Backoff::HOSTED,
    };

    /// Three renewals fit in one lease, so one refused renewal loses nothing.
    fn heartbeat(self) -> Duration {
        Duration::from_secs(self.lease.unsigned_abs()) / 3
    }

    /// [`Self::HOSTED`], or the non-hosted proof values a test names to see
    /// recovery and retries without waiting out hosted time.
    fn from_env(get: &impl Fn(&str) -> Option<String>, hosted: bool) -> Result<Self, String> {
        let mut pace = Self::HOSTED;
        let proof = |key: &'static str| match configured(get, key) {
            Some(_) if hosted => Err(format!("{key} is refused in hosted execution")),
            Some(value) => value
                .parse::<u32>()
                .map(Some)
                .map_err(|_| format!("{key} must be a whole number")),
            None => Ok(None),
        };
        if let Some(millis) = proof("PHYSICIAN_WORKER_POLL_MS")? {
            if millis == 0 {
                return Err("PHYSICIAN_WORKER_POLL_MS must be at least 1".into());
            }
            pace.poll = Duration::from_millis(millis.into());
        }
        if let Some(seconds) = proof("PHYSICIAN_WORKER_LEASE_SECONDS")? {
            if !(1..=600).contains(&seconds) {
                return Err("PHYSICIAN_WORKER_LEASE_SECONDS must be 1 to 600".into());
            }
            pace.lease = seconds.into();
        }
        if let Some(first) = proof("PHYSICIAN_WORKER_RETRY_SECONDS")? {
            pace.backoff = Backoff::new(first.into(), 600)
                .ok_or("PHYSICIAN_WORKER_RETRY_SECONDS must be 0 to 600")?;
        }
        if let Some(value) = configured(get, "PHYSICIAN_WORKER_CONCURRENCY") {
            pace.concurrency = value
                .parse::<usize>()
                .ok()
                .filter(|count| (1..=64).contains(count))
                .ok_or("PHYSICIAN_WORKER_CONCURRENCY must be 1 to 64")?;
        }
        Ok(pace)
    }
}

struct Worker {
    store: Store,
    auth: PostgresAuthStore,
    bus: Arc<dyn Bus>,
    model: ModelSource,
    pace: Pace,
    limits: Limits,
    system: &'static str,
    model_name: String,
    adapter: &'static str,
    /// The packaged catalog, loaded once at startup and shared by every turn.
    skills: PhysicianSkills,
    /// Versioned hosted instructions followed by that catalog's metadata.
    instructions: Arc<str>,
    /// Practices whose turns are rehearsal traffic in telemetry.
    simulation_practices: SimulationPractices,
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
        let simulation_practices = SimulationPractices::from_env(&get)?;
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
        // show that concurrent turns on one instance run side by side.
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
        // retry, a turn that gives up and Try again alongside ordinary turns.
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
            // Fail at startup, not on the first turn, when the binding is invalid.
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
        let pace = Pace::from_env(&get, hosted)?;
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
            pace,
            limits: limits(),
            system,
            model_name,
            adapter,
            skills,
            instructions,
            simulation_practices,
        })
    }
}

/// Bounds for one hosted chat attempt. The context bound matches the site's
/// in-process chat (`harness::execution::Limits::default`), so a conversation
/// that runs on the site also runs here. The reply bounds match Duet's chat.
/// The only tool is `skill_view`: up to eight model calls leave room for a
/// skill, its supporting resources and an answer. A single result may be as
/// large as the loader's largest budget (65536 bytes); four such results fill
/// the attempt's allowance. The lease is renewed for as long as the attempt
/// runs, so `elapsed` is what bounds it.
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
    /// this process, so "fail N times, then answer" holds across retries.
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
/// gives up on its fifth failure and its Try again then succeeds.
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

    #[test]
    fn pace_is_hosted_by_default_and_proof_values_are_refused_when_hosted() {
        let none = |_: &str| None;
        assert_eq!(Pace::from_env(&none, true), Ok(Pace::HOSTED));
        assert_eq!(Pace::from_env(&none, false), Ok(Pace::HOSTED));
        assert_eq!(Pace::HOSTED.heartbeat(), Duration::from_secs(20));
        for key in [
            "PHYSICIAN_WORKER_POLL_MS",
            "PHYSICIAN_WORKER_LEASE_SECONDS",
            "PHYSICIAN_WORKER_RETRY_SECONDS",
        ] {
            let set = |name: &str| (name == key).then(|| "1".to_owned());
            assert_eq!(
                Pace::from_env(&set, true),
                Err(format!("{key} is refused in hosted execution"))
            );
            let invalid = |name: &str| (name == key).then(|| "soon".to_owned());
            assert!(Pace::from_env(&invalid, false).is_err(), "{key}");
        }
        let proof = |name: &str| match name {
            "PHYSICIAN_WORKER_POLL_MS" => Some("250".to_owned()),
            "PHYSICIAN_WORKER_LEASE_SECONDS" => Some("3".to_owned()),
            "PHYSICIAN_WORKER_RETRY_SECONDS" => Some("0".to_owned()),
            "PHYSICIAN_WORKER_CONCURRENCY" => Some("2".to_owned()),
            _ => None,
        };
        assert_eq!(
            Pace::from_env(&proof, false),
            Ok(Pace {
                poll: Duration::from_millis(250),
                lease: 3,
                concurrency: 2,
                backoff: Backoff::new(0, 600).unwrap(),
            })
        );
        // Concurrency is an operating value, so hosted execution takes it.
        let hosted = |name: &str| (name == "PHYSICIAN_WORKER_CONCURRENCY").then(|| "16".to_owned());
        assert_eq!(Pace::from_env(&hosted, true).map(|p| p.concurrency), Ok(16));
        for bad in ["0", "65", "many"] {
            let set = |name: &str| (name == "PHYSICIAN_WORKER_CONCURRENCY").then(|| bad.to_owned());
            assert!(Pace::from_env(&set, true).is_err(), "{bad}");
        }
        for (key, bad) in [
            ("PHYSICIAN_WORKER_POLL_MS", "0"),
            ("PHYSICIAN_WORKER_LEASE_SECONDS", "0"),
            ("PHYSICIAN_WORKER_LEASE_SECONDS", "601"),
            ("PHYSICIAN_WORKER_RETRY_SECONDS", "601"),
        ] {
            let set = |name: &str| (name == key).then(|| bad.to_owned());
            assert!(Pace::from_env(&set, false).is_err(), "{key}={bad}");
        }
    }

    #[test]
    fn startup_refuses_hosted_proof_settings_and_a_worker_without_its_database() {
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
            Err(error) => assert_eq!(error, "PHYSICIAN_DATABASE_URL is required"),
            Ok(_) => panic!("a worker without its database must refuse to start"),
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
        // The workspace catalog loads and startup reaches the database check.
        let local = |name: &str| (name == "PHYSICIAN_WORKER_FIXED_TEXT").then(|| "Noted.".into());
        match block(Worker::from_env(local)) {
            Err(error) => assert_eq!(error, "PHYSICIAN_DATABASE_URL is required"),
            Ok(_) => panic!("a worker without its database must refuse to start"),
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
            origin: None,
        };
        assert_eq!(earlier_attempts(&conversation, &claim), 2);
        conversation.attempts.truncate(1);
        assert_eq!(earlier_attempts(&conversation, &claim), 0);
    }

    fn block<T>(future: impl std::future::Future<Output = T>) -> T {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(future)
    }
}
