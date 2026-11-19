//! Pull worker for accepted physician chat turns and reply revisions.
//!
//! Postgres owns the work. The worker listens for the notification the
//! accepting transaction sends and polls the due work, so a lost notification
//! only delays a turn (`pull`). A delivery names a scope and an input id,
//! never clinical text, and logs stay that way. Work that is settled or that
//! another worker holds is not claimed, so the model is not called for it.
//! A claim is a short lease that hosting keeps renewing while the attempt
//! runs, and that the worker hands back when it is told to stop. The HTTP
//! listener serves health and readiness only. Skill reads use the package
//! saved with the work at acceptance.
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

use std::sync::Arc;
use std::time::Duration;

use axum::Json;
use axum::Router;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use harness::engine::{AttemptFailure, Cancellation, RunLimits};
use local_product::runtime::delivery::{
    AttemptSettlement, Backoff, Delivery, DeliveryKind, DeliveryState,
};
use sea_orm::TransactionTrait;

#[path = "physician-worker/attempt.rs"]
mod attempt;
#[cfg(test)]
#[path = "../../tests/common/disposable.rs"]
mod disposable;
#[path = "physician-worker/practice_job.rs"]
mod practice_job;
#[path = "physician-worker/pull.rs"]
mod pull;
#[path = "physician-worker/revision.rs"]
mod revision;
#[path = "physician-worker/signals.rs"]
mod signals;
use attempt::{Claimed, Failure, Next, Outcome, Tally};
use local_product::runtime::{
    Claim, Conversation, PhysicianHost, RuntimeError, Scope, ShownStepSaved, ShownSteps, Store,
};
use physician_auth::PostgresAuthStore;
use physician_bus::{Bus, Event};
use physician_sample::durable::RuntimeAuthority;
use physician_sample::hosting::{self, SimulationPractices, is_hosted};
use physician_sample::model::{self, ModelSource, ModelTransport, configured};
use serde_json::{Value, json};
use telemetry::genai::{self, RunContext};
use tokio::sync::watch;
use tracing::Instrument;

// A run that marks no step for 600 seconds has stopped moving. Hermes stops
// renewing its session lease after the same 600 seconds without activity.
const STALL: Duration = Duration::from_secs(600);
const BODY_LIMIT: usize = 64 * 1024;

#[tokio::main]
async fn main() {
    if let Err(reason) = hosting::validate_startup(|key| std::env::var(key).ok()) {
        eprintln!("{reason}");
        std::process::exit(1);
    }
    let _telemetry = telemetry::init("physician-worker");
    if let Err(error) = run().await {
        let database = error.database();
        tracing::error!(
            error.type = database.map(|cause| local_product::runtime::DbFailure::of(cause).error_type()),
            db.response.status_code = database.and_then(local_product::runtime::sqlstate).as_deref(),
            error = %error,
            error.cause = database.map(tracing::field::display),
            "physician worker cannot start"
        );
        std::process::exit(1);
    }
}

async fn run() -> Result<(), hosting::StartupError> {
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
    // Cloud Run is where the gauges have a project to go to.
    if std::env::var("K_SERVICE").is_ok() {
        tokio::spawn(signals::run(worker.store.clone(), stopped.clone()));
    }
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
    served.map_err(|_| "physician worker stopped".into())
}

fn router(worker: Arc<Worker>) -> Router {
    let wake = worker.wake.clone();
    Router::new()
        .route("/health", get(health))
        .route("/readyz", get(readyz))
        .with_state(worker)
        .merge(wake_router(wake))
        .layer(telemetry::http_layer())
}

fn wake_router(wake: Arc<WakeService>) -> Router {
    Router::new()
        .route("/schedule/wake", post(schedule_wake))
        .with_state(wake)
}

async fn health(State(worker): State<Arc<Worker>>) -> (StatusCode, Json<Value>) {
    match worker.store.spruce_rollback_halted_practices().await {
        Ok(halted_practices) if halted_practices.is_empty() => (
            StatusCode::OK,
            Json(json!({
                "status": "ok",
                "spruce_channel": { "standing": "healthy" },
            })),
        ),
        Ok(halted_practices) => (
            StatusCode::OK,
            Json(json!({
                "status": "degraded",
                "spruce_channel": {
                    "standing": "rollback_halted",
                    "halted_practices": halted_practices
                        .iter()
                        .map(|(practice, halt)| {
                            json!({
                                "practice": practice,
                                "detected_at": halt.detected_at,
                                "running_release": halt.running_label,
                                "highest_release": halt.highest_label,
                                "reason": halt.reason,
                            })
                        })
                        .collect::<Vec<_>>(),
                }
            })),
        ),
        Err(error) => {
            tracing::warn!(%error, "worker health probe failed to query spruce rollback fence");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "status": "degraded",
                    "spruce_channel": { "standing": "unavailable" },
                })),
            )
        }
    }
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
        DeliveryKind::PracticeJob => practice_job::deliver(worker, delivery, stop).await,
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
    let workspace_id = claim.scope.workspace.clone();
    let session_id = claim.scope.chat.clone();
    let conversation = worker.store.read(&claim.scope).await.ok().flatten();
    let (origin, scenario_id) = worker.turn_origin(
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
            run_id: &claim.attempt,
            work_id: claimed.map(|a| a.work.as_str()),
            predecessor_run_id: claimed.and_then(|a| a.predecessor.as_deref()),
            input_revision: Some(claim.revision),
            attempt_number: earlier_attempts.saturating_add(1),
        },
    );
    drop(conversation);
    let scenario_for_failure = scenario_id.clone();
    let generation = genai::generation_span(
        &RunContext {
            run_id: claim.attempt.clone(),
            job: CHAT_JOB,
            recipe_version: genai::recipe_version(),
            origin,
            scenario_id,
            session_id: Some(session_id),
        },
        "openai",
        &worker.model.roles.chat.adapter,
    );
    genai::record_input(&generation, &input_text);
    // A simulation practice whose harness failure policy covers this turn fails
    // its first K attempts before any model call, without reading message text.
    if worker.simulation_practices.rehearsed_failure(
        &claim.scope.practice,
        scenario_for_failure.as_deref(),
        earlier_attempts,
    ) {
        genai::record_error(&generation, Failure::Rehearsed.description());
        let attempt = Settling {
            span,
            generation: &generation,
            failure: Failure::Rehearsed,
        };
        let reason = AttemptFailure::Failed;
        return settle_attempt(worker, &claim, workspace_id, reason, &attempt).await;
    }
    let provider = worker.model.provider(worker.quota.clone()).await;
    drop(input_text);
    let notice = StepNotice::new(worker.bus.clone(), workspace_id.clone());
    let mut host = PhysicianHost {
        store: worker.store.clone(),
        claim,
        authority: WorkerAuthority::of(&worker.model),
        shown_steps: ShownSteps::new(notice),
    };
    let cancel = Cancellation::default();
    // One provider per turn: concurrent turns on this instance run side by
    // side instead of queueing behind one another's model call.
    let mut provider = match provider {
        Ok(provider) => provider,
        Err(error) => {
            genai::record_error(&generation, "physician chat provider unavailable");
            let reason = AttemptFailure::of(&harness::engine::Failure::Provider(error), &cancel);
            let attempt = Settling {
                span,
                generation: &generation,
                failure: Failure::ProviderInit,
            };
            return settle_attempt(worker, &host.claim, workspace_id, reason, &attempt).await;
        }
    };
    let claimed = host.claim.clone();
    let hosted = tokio::select! {
        hosted = while_owned(
            claimed.lease,
            STALL,
            &cancel,
            || worker.store.renew_claim(&claimed),
            host.run(&mut provider, RunLimits::default(), &cancel)
                .instrument(generation.clone()),
        ) => Some(hosted),
        () = pull::stopping(stop) => None,
    };
    let Some(hosted) = hosted else {
        return released(worker, &host.claim, span, &generation, &cancel).await;
    };
    record_chat_usage(&worker.store, &host.claim, &generation).await;
    let result = match hosted {
        Hosted::Returned(result) => result,
        Hosted::Lost => return lost(span, CHAT_JOB, &host.claim.attempt),
    };
    match result {
        Ok(()) => {
            let output = message_text(&worker.store, &host.claim.scope, "assistant").await;
            genai::record_output(&generation, &output);
            drop(output);
            announce(worker, workspace_id).await;
            attempt::finish(
                span,
                CHAT_JOB,
                Some(&host.claim.attempt),
                Outcome::Committed,
                None,
            );
            Next::Done
        }
        Err(error) => {
            let reason = AttemptFailure::of(&error, &cancel);
            let attempt = Settling {
                span,
                generation: &generation,
                failure: Failure::ModelCall,
            };
            settle_attempt(worker, &host.claim, workspace_id, reason, &attempt).await
        }
    }
}

/// This worker is stopping: the attempt is cancelled and handed back as
/// interrupted, which costs it nothing, so another worker runs it at once.
/// A hand-back that could not be saved leaves the lease to run out.
async fn released(
    worker: &Worker,
    claim: &Claim,
    span: &tracing::Span,
    generation: &tracing::Span,
    cancel: &Cancellation,
) -> Next {
    cancel.cancel();
    genai::record_error(generation, "physician worker stopping");
    let saved = worker
        .store
        .settle_chat_attempt(claim, AttemptFailure::Interrupted, worker.pace.backoff)
        .await;
    if saved.is_err() {
        tracing::warn!("physician worker claim not handed back; its lease will run out");
    }
    attempt::finish(
        span,
        CHAT_JOB,
        Some(&claim.attempt),
        Outcome::Released,
        None,
    );
    Next::Done
}

/// Records the provider-reported usage of this attempt's retained receipts.
/// A provider that reported no usage leaves it unset.
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

/// How hosting's wait for a run ended.
enum Hosted<T> {
    /// The run returned this result while hosting waited for it.
    Returned(Result<T, harness::engine::Failure>),
    /// Hosting cancelled the run, stopped renewing its claim and dropped it,
    /// because the run stopped moving. Nothing is settled. The lease expires
    /// and the next claim ends the attempt as abandoned, as when a worker is lost.
    Lost,
}

/// Hosting keeps ownership independently of the model loop and every product
/// callback, while the run keeps marking steps. A run that marks nothing for
/// `stall` is cancelled and dropped, and its claim is no longer renewed. After
/// a renewal fails, the cancelled run is waited for under the same rule.
async fn while_owned<T, E, F: std::future::Future<Output = Result<(), E>>>(
    lease: local_product::runtime::Lease,
    stall: std::time::Duration,
    cancel: &Cancellation,
    mut renew: impl FnMut() -> F,
    work: impl std::future::Future<Output = Result<T, harness::engine::Failure>>,
) -> Hosted<T>
where
    E: std::error::Error + Send + Sync + 'static,
{
    let renewal = async {
        let interval = lease.renewal_interval();
        let mut tick = tokio::time::interval_at(tokio::time::Instant::now() + interval, interval);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            tick.tick().await;
            if let Err(error) = renew().await {
                break error;
            }
        }
    };
    let stalled = async {
        loop {
            let idle = cancel.since_last_step();
            let Some(left) = stall.checked_sub(idle).filter(|left| !left.is_zero()) else {
                break idle;
            };
            tokio::time::sleep(left).await;
        }
    };
    tokio::pin!(work);
    tokio::pin!(renewal);
    tokio::pin!(stalled);
    tokio::select! {
        biased;
        error = &mut renewal => {
            tracing::warn!(error = %error, "physician attempt ownership could not be renewed");
            cancel.cancel();
            tokio::select! {
                biased;
                result = &mut work => Hosted::Returned(result),
                idle = &mut stalled => {
                    tracing::error!(idle_seconds = idle.as_secs(), "physician attempt stopped moving");
                    Hosted::Lost
                }
            }
        }
        idle = &mut stalled => {
            tracing::error!(idle_seconds = idle.as_secs(), "physician attempt stopped moving");
            cancel.cancel();
            Hosted::Lost
        }
        result = &mut work => Hosted::Returned(result),
    }
}

/// A run hosting dropped settles nothing. The claim is left to run out, as a
/// lost worker leaves it: the next claim after the lease ends the attempt as
/// abandoned.
fn lost(span: &tracing::Span, job: &'static str, attempt: &str) -> Next {
    attempt::finish(
        span,
        job,
        Some(attempt),
        Outcome::Retry,
        Some(Failure::Lost),
    );
    Next::Done
}

/// The spans and the primary failure class of an attempt being settled.
struct Settling<'a> {
    /// The attempt's `physician.worker.attempt` span.
    span: &'a tracing::Span,
    /// The attempt's `gen_ai.generation` span.
    generation: &'a tracing::Span,
    failure: Failure,
}

/// Retains the shared failure classification, then announces the durable
/// state. A counted failure makes the work due again after the backoff, or
/// ends it as failed at the last allowed attempt; a reached limit holds it
/// for explicit Resume.
async fn settle_attempt(
    worker: &Worker,
    claim: &Claim,
    workspace_id: String,
    reason: AttemptFailure,
    attempt: &Settling<'_>,
) -> Next {
    let saved = worker
        .store
        .settle_chat_attempt(claim, reason, worker.pace.backoff)
        .await;
    let ended = settled(claim, &saved, attempt);
    announce(worker, workspace_id).await;
    ended
}

/// Records how a chat attempt's settlement ended and what the loop does next.
fn settled(
    claim: &Claim,
    saved: &Result<AttemptSettlement, RuntimeError>,
    attempt: &Settling<'_>,
) -> Next {
    let (outcome, next) = attempt::settled(attempt.span, CHAT_JOB, &claim.attempt, saved);
    let failure = match outcome {
        Outcome::Retry | Outcome::GaveUp => {
            genai::record_error(attempt.generation, "physician chat failed");
            Some(attempt.failure)
        }
        Outcome::Committed | Outcome::Settled | Outcome::Released | Outcome::ClaimError => None,
    };
    attempt::finish(
        attempt.span,
        CHAT_JOB,
        Some(&claim.attempt),
        outcome,
        failure,
    );
    next
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

/// Tells every open page on this workspace to refetch. Awaited after each
/// committed reply and each failed attempt, so a page holding a running turn
/// learns when it completed or ended as "couldn't finish"; [`StepNotice`]
/// sends it for shown steps without waiting. Lossy: a page that misses it
/// still converges on its next refetch or resync.
async fn announce(worker: &Worker, workspace_id: String) {
    workspace_changed(worker.bus.as_ref(), workspace_id).await;
}

async fn workspace_changed(bus: &dyn Bus, workspace_id: String) {
    if bus
        .publish(Event::WorkspaceChanged { workspace_id })
        .await
        .is_err()
    {
        tracing::warn!("physician workspace invalidation not published");
    }
}

/// Tells open pages on one attempt's workspace that a step they show started
/// or returned, so they read it while the run still works. Its own task
/// publishes, so a slow bus never delays the run. Notices that arrive while
/// one publish is in flight become one more publish after it, so a burst of
/// steps causes few refetches and the last notice is never dropped. Once the
/// attempt drops its notice, the task sends what is still pending and ends;
/// the bus bounds each publish by its command timeout.
struct StepNotice(watch::Sender<()>);

impl StepNotice {
    fn new(bus: Arc<dyn Bus>, workspace_id: String) -> Self {
        let (pending, mut notices) = watch::channel(());
        tokio::spawn(async move {
            while notices.changed().await.is_ok() {
                workspace_changed(bus.as_ref(), workspace_id.clone()).await;
            }
        });
        Self(pending)
    }
}

impl ShownStepSaved for StepNotice {
    /// The host's claim, and so its workspace, is fixed for the attempt this
    /// notice was made for.
    fn shown_step_saved(&mut self, _scope: &Scope) {
        self.0.send_replace(());
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
        .find(|message| message.role.as_str() == role)
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
    /// Attempts take a connection only for each statement and wait for one
    /// when the process's pool is busy, so four share what the listener
    /// leaves of [`local_product::runtime::MAX_CONNECTIONS`].
    const HOSTED: Self = Self {
        poll: Duration::from_secs(10),
        lease: 60,
        concurrency: 4,
        backoff: Backoff::HOSTED,
    };

    /// [`Self::HOSTED`], or the non-hosted proof values a test names to see
    /// recovery and retries without waiting out hosted time.
    fn from_env(get: &impl Fn(&str) -> Option<String>, hosted: bool) -> Result<Self, PaceError> {
        let mut pace = Self::HOSTED;
        let proof = |key: &'static str| match configured(get, key) {
            Some(_) if hosted => Err(PaceError::Hosted { key }),
            Some(value) => value
                .parse::<u32>()
                .map(Some)
                .map_err(|source| PaceError::NotWhole { key, source }),
            None => Ok(None),
        };
        let outside = |key, low, high| PaceError::OutOfRange { key, low, high };
        if let Some(millis) = proof("PHYSICIAN_WORKER_POLL_MS")? {
            if millis == 0 {
                return Err(PaceError::Zero {
                    key: "PHYSICIAN_WORKER_POLL_MS",
                });
            }
            pace.poll = Duration::from_millis(millis.into());
        }
        if let Some(seconds) = proof("PHYSICIAN_WORKER_LEASE_SECONDS")? {
            if !(1..=600).contains(&seconds) {
                return Err(outside("PHYSICIAN_WORKER_LEASE_SECONDS", 1, 600));
            }
            pace.lease = seconds.into();
        }
        if let Some(first) = proof("PHYSICIAN_WORKER_RETRY_SECONDS")? {
            pace.backoff = Backoff::new(first.into(), 600).ok_or(outside(
                "PHYSICIAN_WORKER_RETRY_SECONDS",
                0,
                600,
            ))?;
        }
        if let Some(value) = configured(get, "PHYSICIAN_WORKER_CONCURRENCY") {
            let key = "PHYSICIAN_WORKER_CONCURRENCY";
            let count = value
                .parse::<usize>()
                .map_err(|source| PaceError::NotWhole { key, source })?;
            if !(1..=64).contains(&count) {
                return Err(outside(key, 1, 64));
            }
            pace.concurrency = count;
        }
        Ok(pace)
    }
}

/// Why a pacing setting was refused.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
enum PaceError {
    /// A proof-only setting given to a hosted worker.
    #[error("{key} is refused in hosted execution")]
    Hosted { key: &'static str },
    /// A setting that is not a whole number.
    #[error("{key} must be a whole number")]
    NotWhole {
        key: &'static str,
        source: std::num::ParseIntError,
    },
    /// Zero for a setting that must be at least 1.
    #[error("{key} must be at least 1")]
    Zero { key: &'static str },
    /// A whole number outside the setting's range.
    #[error("{key} must be {low} to {high}")]
    OutOfRange {
        key: &'static str,
        low: usize,
        high: usize,
    },
}

struct Worker {
    store: Store,
    auth: PostgresAuthStore,
    bus: Arc<dyn Bus>,
    model: ModelSource,
    /// One durable provider cooldown for this product across worker replicas.
    quota: Arc<dyn harness::provider::QuotaGate>,
    pace: Pace,
    /// Schedule wake endpoint state (`POST /schedule/wake`) and clinical library client.
    wake: Arc<WakeService>,
    /// `Simulation` for every turn when the model is the local Responses stub.
    origin: genai::Origin,
    /// Practices whose turns are rehearsal traffic in telemetry.
    simulation_practices: SimulationPractices,
}

impl Worker {
    /// The telemetry origin and scenario of a turn in `practice` accepted
    /// with `scenario`. A rehearsal practice's turn is `simulation` with its
    /// scenario; any other turn takes this worker's own origin and no
    /// scenario.
    fn turn_origin(
        &self,
        practice: &str,
        scenario: Option<&str>,
    ) -> (genai::Origin, Option<String>) {
        match self.simulation_practices.origin(practice, scenario) {
            (genai::Origin::Real, _) => (self.origin, None),
            rehearsal => rehearsal,
        }
    }

    async fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, hosting::StartupError> {
        let hosted = is_hosted(&get);
        let simulation_practices = SimulationPractices::from_env(&get)?;
        let clinical_library = hosting::ClinicalLibraryConfig::from_env(&get)
            .map_err(|error| error.to_string())?
            .map(hosting::ClinicalLibraryClient::new)
            .transpose()
            .map_err(|error| error.to_string())?;
        let (transport, origin) = ModelTransport::from_env(&get, hosted)?;
        let pace = Pace::from_env(&get, hosted).map_err(|error| error.to_string())?;
        let schedule_auth = wake_auth(&get, hosted)?;
        let wake_mode = if origin == genai::Origin::Simulation || hosting::is_staging_preview(&get)
        {
            WakeMode::RehearsalAllowed
        } else {
            WakeMode::LiveOnly
        };
        let model = ModelSource::from_env(transport, &get)?;
        let wake_roles = model.roles.clone();
        let wake_skills = local_product::physician_chat::PhysicianSkills::configured(
            configured(&get, "PHYSICIAN_SKILLS_DIR").as_deref(),
            hosted,
        );
        if let Err(err @ local_product::physician_chat::SkillsError::Package { .. }) = &wake_skills
        {
            return Err(err.to_string().into());
        }
        let database = configured(&get, "PHYSICIAN_DATABASE_URL")
            .ok_or("PHYSICIAN_DATABASE_URL is required")?;
        // One pool per process: the auth store reads through the runtime's.
        let mut store = Store::connect(&database)
            .await
            .map_err(hosting::StartupError::RuntimeStore)?;
        let elation_client =
            match local_product::physician_sources::elation_fhir::ElationFhirConfig::from_env(&get)
                .map_err(|err| err.to_string())?
            {
                Some(config) => Some(Arc::new(
                    local_product::physician_sources::elation_fhir::ElationFhirClient::new(config)
                        .map_err(|err| err.to_string())?
                        .with_store(store.clone()),
                )),
                None => None,
            };
        let mut refresh_composite =
            local_product::runtime::refresh::CompositePatientSourceRefresh::new();
        refresh_composite.push(Arc::new(
            local_product::runtime::hint_bridge::HintBridgeSourceRefresh::new(store.clone()),
        ));
        if let Some(ref client) = elation_client {
            let adapter: Arc<dyn local_product::runtime::refresh::PatientSourceRefresh> =
                client.clone();
            refresh_composite.push(adapter);
        }
        store = store.with_source_refresh(Arc::new(refresh_composite));
        if let Ok(skills) = local_product::physician_chat::PhysicianSkills::configured(
            configured(&get, "PHYSICIAN_SKILLS_DIR").as_deref(),
            hosted,
        ) {
            store = store.with_understanding_trigger(model.roles.clone(), skills);
        }
        let auth = PostgresAuthStore::from_connection(store.connection().clone());
        let bus = physician_bus::from_env(&get, hosted).await?;
        let quota: Arc<dyn harness::provider::QuotaGate> = Arc::new(store.clone());
        let skills_dir = configured(&get, "PHYSICIAN_SKILLS_DIR");
        let calendar =
            physician_sample::schedule::SyntheticBankCalendar::from_env(&get).map(|source| {
                PracticeSyntheticCalendar {
                    practice: physician_sample::STAGING_PRACTICE_ID.to_owned(),
                    source,
                }
            });
        let elation = elation_client.map(|client| PracticeElationSource {
            practice: configured(&get, "PHYSICIAN_ELATION_PRACTICE_ID")
                .unwrap_or_else(|| physician_sample::STAGING_PRACTICE_ID.to_owned()),
            client,
        });
        let wake = Arc::new(WakeService {
            auth: schedule_auth,
            mode: wake_mode,
            handler: Arc::new(ScheduledWakeHandler {
                dispatch: StoreWakeHandler {
                    store: store.clone(),
                    auth: auth.clone(),
                    roles: wake_roles,
                    skills: wake_skills,
                },
                visit_prep: VisitPrepWakeHandler {
                    store: store.clone(),
                    auth: auth.clone(),
                    bus: Arc::clone(&bus),
                    model: model.clone(),
                    quota: quota.clone(),
                    skills_dir,
                    calendar,
                    elation,
                },
            }),
            clinical_library,
        });
        Ok(Self {
            store,
            auth,
            bus,
            model,
            quota,
            pace,
            wake,
            origin,
            simulation_practices,
        })
    }

    /// Polls configured connected Spruce channels across practices under exclusive leases,
    /// halting filing automatically if the release fence detects a deployment rollback.
    async fn poll_spruce_channels(&self) {
        use physician_auth::AuthStore as _;
        let Ok(practices) = self.store.connected_spruce_practices().await else {
            return;
        };
        if practices.is_empty() {
            return;
        }
        let Ok(skills) = self.store.resolve_worker_skills(None).await else {
            return;
        };
        let mut provider = self.model.provider(self.quota.clone()).await.ok();
        let real_now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let holder_id = format!("physician-worker-{}", std::process::id());
        let lease_ttl = self.pace.lease.max(30);

        for (practice, cfg) in practices {
            let Some(token) = cfg
                .access_token
                .as_deref()
                .map(str::trim)
                .filter(|t| !t.is_empty())
            else {
                continue;
            };
            let base = cfg
                .api_base
                .as_deref()
                .map(str::trim)
                .filter(|b| !b.is_empty())
                .unwrap_or(spruce::api::API_BASE);
            let Ok(client) = spruce::api::Client::with_base(token, base) else {
                continue;
            };
            let reader = spruce::changes::Reader::new(client);
            let now_ts = self
                .auth
                .practice_clock(&practice)
                .await
                .map_or(real_now as i64, |clock| clock.now(real_now) as i64);
            let receiver = local_product::runtime::reviews::PatientReviewReceiver::new(
                &self.store,
                &self.model.roles,
                &skills,
                &self.auth,
            );
            let pass = local_product::runtime::spruce_sync::SpruceSyncPass {
                practice: &practice,
                holder_id: &holder_id,
                lease_ttl_seconds: lease_ttl,
                now_ts,
                roles: &self.model.roles,
                skills: &skills,
                review_receiver: &receiver,
            };
            if let Err(error) = self
                .store
                .sync_spruce_channel_pass(&reader, &self.auth, &pass, provider.as_mut())
                .await
            {
                tracing::warn!(
                    practice = %practice,
                    %error,
                    "spruce channel poll stopped"
                );
            }
        }
    }
}

/// Shared state for `POST /schedule/wake`.
struct WakeService {
    auth: WakeAuth,
    mode: WakeMode,
    handler: Arc<dyn WakeHandler>,
    clinical_library: Option<hosting::ClinicalLibraryClient>,
}

/// Authentication state for `POST /schedule/wake`.
enum WakeAuth {
    /// Verifies Google OIDC signature, issuer, expiry, and audience first (`401`),
    /// then checks that the verified service-account email equals `principal` (`403`).
    Verified {
        verifier: GoogleOidcVerifier,
        principal: hosting::ServiceAccountEmail,
    },
    /// No scheduler principal is configured on this revision: missing/invalid tokens
    /// still fail `401`, while valid tokens fail `403`.
    Disabled {
        verifier: Option<GoogleOidcVerifier>,
    },
    /// Unauthenticated local loopback emulator mode.
    LoopbackEmulator,
}

/// Whether `POST /schedule/wake` honours a rehearsed failure directive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WakeMode {
    /// Simulation or `staging-preview`: allows alert/retry drills.
    RehearsalAllowed,
    /// Live non-staging execution: refuses rehearsed failures.
    LiveOnly,
}

/// Why `POST /schedule/wake` refused a caller before executing any wake work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WakeRefusal {
    /// Missing Bearer header or failed OIDC signature, issuer, audience, or expiry check.
    Unauthenticated,
    /// Valid OIDC token for a caller other than `PHYSICIAN_SCHEDULER_SERVICE_ACCOUNT`.
    ForbiddenPrincipal,
    /// Schedule wakes are not configured on this worker revision.
    ScheduleDisabled,
}

impl WakeRefusal {
    fn status_code(self) -> StatusCode {
        match self {
            Self::Unauthenticated => StatusCode::UNAUTHORIZED,
            Self::ForbiddenPrincipal | Self::ScheduleDisabled => StatusCode::FORBIDDEN,
        }
    }

    fn reason(self) -> &'static str {
        match self {
            Self::Unauthenticated => "unauthenticated",
            Self::ForbiddenPrincipal => "forbidden_principal",
            Self::ScheduleDisabled => "schedule_disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WakeDirective {
    Normal,
    RehearseFailure,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WakeTick {
    job_name: Option<String>,
    schedule_time: Option<String>,
    retry_count: u32,
    directive: WakeDirective,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WakeReport {
    due_patients_checked: usize,
    reviews_started: usize,
    practices_checked: usize,
    briefings_prepared: usize,
    concerns_retired: usize,
}

#[derive(Debug)]
enum WakeError {
    Rehearsed,
    RuntimeStore(sea_orm::DbErr),
    AuthStore(sea_orm::DbErr),
    AuthUnavailable(physician_auth::StoreUnavailable),
    SkillsUnavailable(local_product::physician_chat::SkillsError),
    Dispatch(local_product::runtime::dispatch::DispatchError),
    ModelProvider(harness::engine::Cause),
    DeveloperPlanBoundary(String),
    ScheduleSync(local_product::runtime::schedule::ScheduleError),
    VisitPreparation(local_product::runtime::briefings::BriefingError),
}

impl WakeError {
    fn kind(&self) -> &'static str {
        match self {
            Self::Rehearsed => "rehearsed_failure",
            Self::RuntimeStore(_) => "runtime_store_unavailable",
            Self::AuthStore(_) | Self::AuthUnavailable(_) => "auth_store_unavailable",
            Self::SkillsUnavailable(_) => "skills_unavailable",
            Self::Dispatch(_) => "dispatch_failed",
            Self::ModelProvider(_) => "model_provider_unavailable",
            Self::DeveloperPlanBoundary(_) => "developer_plan_boundary",
            Self::ScheduleSync(_) => "schedule_sync_failed",
            Self::VisitPreparation(_) => "visit_preparation_failed",
        }
    }
}

impl std::fmt::Display for WakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rehearsed => f.write_str("rehearsed schedule wake failure"),
            Self::RuntimeStore(source) => write!(f, "runtime store check failed: {source}"),
            Self::AuthStore(source) => write!(f, "auth store check failed: {source}"),
            Self::AuthUnavailable(source) => write!(f, "auth store unavailable: {source}"),
            Self::SkillsUnavailable(source) => write!(f, "physician skills unavailable: {source}"),
            Self::Dispatch(source) => write!(f, "patient dispatch wake failed: {source}"),
            Self::ModelProvider(source) => {
                write!(f, "model provider initialization failed: {source}")
            }
            Self::DeveloperPlanBoundary(reason) => f.write_str(reason),
            Self::ScheduleSync(source) => write!(f, "schedule sync failed: {source}"),
            Self::VisitPreparation(source) => write!(f, "visit preparation wake failed: {source}"),
        }
    }
}

impl std::error::Error for WakeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rehearsed | Self::DeveloperPlanBoundary(_) => None,
            Self::RuntimeStore(source) | Self::AuthStore(source) => Some(source),
            Self::AuthUnavailable(source) => Some(source),
            Self::SkillsUnavailable(source) => Some(source),
            Self::Dispatch(source) => Some(source),
            Self::ModelProvider(source) => Some(source.as_ref()),
            Self::ScheduleSync(source) => Some(source),
            Self::VisitPreparation(source) => Some(source),
        }
    }
}

/// Typed wake hook invoked on each authenticated Cloud Scheduler tick.
#[async_trait::async_trait]
trait WakeHandler: Send + Sync {
    async fn wake(&self, tick: &WakeTick) -> Result<WakeReport, WakeError>;
}

fn clone_skills_error(
    error: &local_product::physician_chat::SkillsError,
) -> local_product::physician_chat::SkillsError {
    match error {
        local_product::physician_chat::SkillsError::Unconfigured => {
            local_product::physician_chat::SkillsError::Unconfigured
        }
        local_product::physician_chat::SkillsError::Package { root, reason } => {
            local_product::physician_chat::SkillsError::Package {
                root: root.clone(),
                reason: reason.clone(),
            }
        }
        local_product::physician_chat::SkillsError::MissingStartSkill { root, name } => {
            local_product::physician_chat::SkillsError::MissingStartSkill {
                root: root.clone(),
                name: name.clone(),
            }
        }
    }
}

struct StoreWakeHandler {
    store: Store,
    auth: PostgresAuthStore,
    roles: local_product::ModelRoles,
    skills: Result<
        local_product::physician_chat::PhysicianSkills,
        local_product::physician_chat::SkillsError,
    >,
}

async fn check_wake_stores(store: &Store, auth: &PostgresAuthStore) -> Result<(), WakeError> {
    let probe = Scope {
        practice: "schedule-wake".into(),
        physician: "schedule-wake".into(),
        workspace: "schedule-wake".into(),
        chat: "schedule-wake".into(),
    };
    store.read(&probe).await.map_err(WakeError::RuntimeStore)?;
    auth.check_schema().await.map_err(WakeError::AuthStore)?;
    Ok(())
}

impl StoreWakeHandler {
    async fn dispatch_due_patient_reviews(
        &self,
        now_unix: i64,
    ) -> Result<(usize, usize), WakeError> {
        let practices = self
            .store
            .registered_dispatch_practices()
            .await
            .map_err(WakeError::RuntimeStore)?;
        if practices.is_empty() {
            return Ok((0, 0));
        }
        let resolved_skills;
        let skills = match &self.skills {
            Ok(skills) => skills,
            Err(local_product::physician_chat::SkillsError::Unconfigured) => {
                resolved_skills = self
                    .store
                    .resolve_worker_skills(None)
                    .await
                    .map_err(WakeError::VisitPreparation)?;
                &resolved_skills
            }
            Err(err) => return Err(WakeError::SkillsUnavailable(clone_skills_error(err))),
        };
        let checker = local_product::runtime::dispatch::RefreshingSourceChecker::new(
            self.store.clone(),
            &self.auth,
        );
        let mut due_patients_checked = 0_usize;
        let mut reviews_started = 0_usize;
        for practice in practices {
            let receiver = local_product::runtime::reviews::PatientReviewReceiver::new(
                &self.store,
                &self.roles,
                skills,
                &self.auth,
            );
            let summary = self
                .store
                .wake_practice_dispatch(&self.auth, &practice, now_unix, &checker, &receiver)
                .await
                .map_err(WakeError::Dispatch)?;
            due_patients_checked = due_patients_checked
                .saturating_add(summary.checked_patients.len())
                .saturating_add(summary.failed_checks.len());
            reviews_started = reviews_started.saturating_add(summary.started_reviews.len());
            let _ = receiver.take_deliveries().await;
        }
        Ok((due_patients_checked, reviews_started))
    }
}

#[async_trait::async_trait]
impl WakeHandler for StoreWakeHandler {
    async fn wake(&self, tick: &WakeTick) -> Result<WakeReport, WakeError> {
        if tick.directive == WakeDirective::RehearseFailure {
            return Err(WakeError::Rehearsed);
        }
        check_wake_stores(&self.store, &self.auth).await?;
        let now_unix = self
            .store
            .current_unix_seconds()
            .await
            .map_err(WakeError::RuntimeStore)?;
        let (due_patients_checked, reviews_started) =
            self.dispatch_due_patient_reviews(now_unix).await?;
        Ok(WakeReport {
            due_patients_checked,
            reviews_started,
            ..WakeReport::default()
        })
    }
}

struct PracticeSyntheticCalendar {
    practice: String,
    source: physician_sample::schedule::SyntheticBankCalendar,
}

struct PracticeElationSource {
    practice: String,
    client: Arc<local_product::physician_sources::elation_fhir::ElationFhirClient>,
}

struct VisitPrepWakeHandler {
    store: Store,
    auth: PostgresAuthStore,
    bus: Arc<dyn Bus>,
    model: ModelSource,
    quota: Arc<dyn harness::provider::QuotaGate>,
    skills_dir: Option<String>,
    calendar: Option<PracticeSyntheticCalendar>,
    elation: Option<PracticeElationSource>,
}

#[async_trait::async_trait]
impl WakeHandler for VisitPrepWakeHandler {
    async fn wake(&self, tick: &WakeTick) -> Result<WakeReport, WakeError> {
        use local_product::runtime::schedule::ScheduleSource as _;
        use physician_auth::AuthStore as _;

        if tick.directive == WakeDirective::RehearseFailure {
            return Err(WakeError::Rehearsed);
        }
        check_wake_stores(&self.store, &self.auth).await?;

        let mut practices = self
            .store
            .scheduled_practices()
            .await
            .map_err(WakeError::RuntimeStore)?;
        let hint_practices: std::collections::BTreeSet<String> = self
            .store
            .hint_bridge_schedule_practices()
            .await
            .map_err(WakeError::RuntimeStore)?
            .into_iter()
            .collect();
        for hint_practice in &hint_practices {
            if !practices.iter().any(|id| id == hint_practice) {
                practices.push(hint_practice.clone());
            }
        }
        if let Some(elation) = &self.elation
            && !practices.iter().any(|id| id == &elation.practice)
        {
            practices.push(elation.practice.clone());
        }
        if let Some(cal) = &self.calendar
            && !practices.iter().any(|id| id == &cal.practice)
            && !self
                .auth
                .practice_patients(&cal.practice)
                .await
                .map_err(WakeError::AuthUnavailable)?
                .is_empty()
        {
            practices.push(cal.practice.clone());
        }
        if practices.is_empty() {
            return Ok(WakeReport::default());
        }

        let skills = self
            .store
            .resolve_worker_skills(self.skills_dir.as_deref())
            .await
            .map_err(WakeError::VisitPreparation)?;
        let mut provider = self
            .model
            .provider(self.quota.clone())
            .await
            .map_err(WakeError::ModelProvider)?;
        let authority = WorkerAuthority::of(&self.model);
        let real_now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        let mut report = WakeReport::default();
        for practice in practices {
            let mut schedule_synced = false;
            let clock = self
                .auth
                .practice_clock(&practice)
                .await
                .map_err(WakeError::AuthUnavailable)?;
            let now_unix = clock.now(real_now) as i64;
            if let Some(elation) = &self.elation
                && practice == elation.practice
            {
                match elation.client.read_roster().await {
                    Ok(roster) => {
                        if let Err(err) = self
                            .store
                            .sync_practice_roster(
                                &self.auth,
                                &practice,
                                elation.client.source_id(),
                                elation.client.source_label(),
                                &roster.patients,
                                now_unix,
                            )
                            .await
                        {
                            tracing::warn!(%err, %practice, "elation roster sync failed");
                        }
                    }
                    Err(err) => {
                        tracing::warn!(%err, %practice, "elation roster read failed");
                    }
                }
                physician_sample::schedule::sync_synthetic_schedule(
                    &*elation.client,
                    &self.auth,
                    &self.store,
                    &practice,
                )
                .await
                .map_err(WakeError::ScheduleSync)?;
                schedule_synced = true;
            }
            if hint_practices.contains(&practice) {
                let hint_source =
                    local_product::runtime::hint_bridge::HintBridgeScheduleSource::new(
                        self.store.clone(),
                        practice.clone(),
                    );
                physician_sample::schedule::sync_synthetic_schedule(
                    &hint_source,
                    &self.auth,
                    &self.store,
                    &practice,
                )
                .await
                .map_err(WakeError::ScheduleSync)?;
                schedule_synced = true;
            }
            if let Some(cal) = &self.calendar
                && practice == cal.practice
            {
                physician_sample::schedule::sync_synthetic_schedule(
                    &cal.source,
                    &self.auth,
                    &self.store,
                    &practice,
                )
                .await
                .map_err(WakeError::ScheduleSync)?;
                schedule_synced = true;
            }
            let roster = self
                .auth
                .practice_patients(&practice)
                .await
                .map_err(WakeError::AuthUnavailable)?;
            for workspace in &roster {
                authority
                    .check_store_source_boundary(
                        &self.store,
                        &Scope {
                            practice: practice.clone(),
                            physician: local_product::runtime::jobs::PRACTICE_OWNER.into(),
                            workspace: workspace.clone(),
                            chat: "schedule-wake".into(),
                        },
                    )
                    .await
                    .map_err(WakeError::DeveloperPlanBoundary)?;
            }
            let summary = self
                .store
                .wake_visit_preparation(
                    &self.auth,
                    &practice,
                    &self.model.roles,
                    &skills,
                    &mut provider,
                    now_unix,
                )
                .await
                .map_err(WakeError::VisitPreparation)?;
            let mut changed_workspaces = std::collections::BTreeSet::new();
            for job in &summary.queued_briefing_jobs {
                changed_workspaces.insert(job.workspace.clone());
            }
            for job in &summary.failed_jobs {
                changed_workspaces.insert(job.workspace.clone());
            }
            let prep_changed = !summary.completed_briefings.is_empty()
                || !summary.queued_briefing_jobs.is_empty()
                || !summary.failed_jobs.is_empty()
                || !summary.retired_concerns.is_empty();
            for workspace_id in changed_workspaces {
                let _ = self
                    .bus
                    .publish(Event::WorkspaceChanged { workspace_id })
                    .await;
            }
            if schedule_synced || prep_changed {
                let _ = self.bus.publish(Event::Resync).await;
            }
            report.practices_checked += 1;
            report.due_patients_checked += roster.len();
            report.briefings_prepared += summary.completed_briefings.len();
            report.concerns_retired += summary.retired_concerns.len();
        }
        Ok(report)
    }
}

struct ScheduledWakeHandler {
    dispatch: StoreWakeHandler,
    visit_prep: VisitPrepWakeHandler,
}

#[async_trait::async_trait]
impl WakeHandler for ScheduledWakeHandler {
    async fn wake(&self, tick: &WakeTick) -> Result<WakeReport, WakeError> {
        let dispatch_report = self.dispatch.wake(tick).await?;
        let prep_report = self.visit_prep.wake(tick).await?;
        Ok(WakeReport {
            due_patients_checked: dispatch_report
                .due_patients_checked
                .saturating_add(prep_report.due_patients_checked),
            reviews_started: dispatch_report.reviews_started,
            practices_checked: prep_report.practices_checked,
            briefings_prepared: prep_report.briefings_prepared,
            concerns_retired: prep_report.concerns_retired,
        })
    }
}

#[derive(Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct WakePayload {
    #[serde(default)]
    rehearse_failure: bool,
}

fn bounded_header(headers: &axum::http::HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty() && value.len() <= 256)
        .map(str::to_owned)
}

async fn schedule_wake(
    State(wake): State<Arc<WakeService>>,
    request: Request,
) -> (StatusCode, Json<Value>) {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    if let Err(refusal) = authorize_wake(&wake.auth, auth_header.as_deref()).await {
        tracing::warn!(
            event = "physician.worker.schedule_wake_refused",
            reason = refusal.reason(),
            status = u64::from(refusal.status_code().as_u16()),
            "refused physician schedule wake"
        );
        return (
            refusal.status_code(),
            Json(json!({ "status": "refused", "reason": refusal.reason() })),
        );
    }
    let job_name = bounded_header(request.headers(), "x-cloudscheduler-jobname");
    let schedule_time = bounded_header(request.headers(), "x-cloudscheduler-scheduletime");
    let retry_count = bounded_header(request.headers(), "x-cloudscheduler-jobexecutioncount")
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(0);
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), BODY_LIMIT).await else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "bad_request" })),
        );
    };
    let payload = if bytes.iter().all(u8::is_ascii_whitespace) {
        WakePayload::default()
    } else {
        match serde_json::from_slice::<WakePayload>(&bytes) {
            Ok(payload) => payload,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "status": "bad_request" })),
                );
            }
        }
    };
    let directive = if payload.rehearse_failure {
        if wake.mode != WakeMode::RehearsalAllowed {
            tracing::warn!(
                event = "physician.worker.schedule_wake_refused",
                reason = "rehearsal_forbidden",
                status = 403_u64,
                "refused rehearsed schedule wake outside simulation or staging-preview"
            );
            return (
                StatusCode::FORBIDDEN,
                Json(json!({ "status": "refused", "reason": "rehearsal_forbidden" })),
            );
        }
        WakeDirective::RehearseFailure
    } else {
        WakeDirective::Normal
    };
    let tick = WakeTick {
        job_name,
        schedule_time,
        retry_count,
        directive,
    };
    match wake.handler.wake(&tick).await {
        Ok(report) => {
            tracing::info!(
                event = "physician.worker.schedule_wake",
                scheduler.job = tick.job_name.as_deref().unwrap_or("manual"),
                scheduler.schedule_time = tick.schedule_time.as_deref().unwrap_or(""),
                scheduler.retry_count = u64::from(tick.retry_count),
                due_patients_checked = report.due_patients_checked as u64,
                reviews_started = report.reviews_started as u64,
                practices_checked = report.practices_checked as u64,
                briefings_prepared = report.briefings_prepared as u64,
                concerns_retired = report.concerns_retired as u64,
                clinical_library_configured = wake.clinical_library.is_some(),
                "accepted physician schedule wake"
            );
            (
                StatusCode::OK,
                Json(json!({
                    "status": "ok",
                    "due_patients_checked": report.due_patients_checked,
                    "reviews_started": report.reviews_started,
                    "practices_checked": report.practices_checked,
                    "briefings_prepared": report.briefings_prepared,
                    "concerns_retired": report.concerns_retired,
                    "clinical_library_configured": wake.clinical_library.is_some(),
                })),
            )
        }
        Err(error) => {
            tracing::error!(
                event = "physician.worker.schedule_wake_failed",
                error.kind = error.kind(),
                scheduler.job = tick.job_name.as_deref().unwrap_or("manual"),
                scheduler.schedule_time = tick.schedule_time.as_deref().unwrap_or(""),
                scheduler.retry_count = u64::from(tick.retry_count),
                "physician schedule wake failed: {error}"
            );
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "status": "failed",
                    "error": error.kind(),
                })),
            )
        }
    }
}

fn bearer_token(header: Option<&str>) -> Option<&str> {
    header
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|token| !token.is_empty())
}

async fn authorize_wake(auth: &WakeAuth, header: Option<&str>) -> Result<(), WakeRefusal> {
    match auth {
        WakeAuth::LoopbackEmulator => Ok(()),
        WakeAuth::Disabled { verifier } => {
            let token = bearer_token(header).ok_or(WakeRefusal::Unauthenticated)?;
            if let Some(verifier) = verifier {
                verifier
                    .verify(token)
                    .await
                    .map_err(|_| WakeRefusal::Unauthenticated)?;
            }
            Err(WakeRefusal::ScheduleDisabled)
        }
        WakeAuth::Verified {
            verifier,
            principal,
        } => {
            let token = bearer_token(header).ok_or(WakeRefusal::Unauthenticated)?;
            let claims = verifier
                .verify(token)
                .await
                .map_err(|_| WakeRefusal::Unauthenticated)?;
            let verified = claims.get("email_verified") == Some(&Value::Bool(true));
            let matches_principal = claims
                .get("email")
                .and_then(Value::as_str)
                .is_some_and(|email| email.eq_ignore_ascii_case(principal.as_str()));
            if verified && matches_principal {
                Ok(())
            } else {
                Err(WakeRefusal::ForbiddenPrincipal)
            }
        }
    }
}

fn wake_auth(get: &impl Fn(&str) -> Option<String>, hosted: bool) -> Result<WakeAuth, String> {
    let audience = configured(get, "PHYSICIAN_WORKER_AUDIENCE");
    if let Some(raw_principal) = configured(get, "PHYSICIAN_SCHEDULER_SERVICE_ACCOUNT") {
        let principal = hosting::ServiceAccountEmail::parse(&raw_principal)
            .map_err(|error| format!("PHYSICIAN_SCHEDULER_SERVICE_ACCOUNT: {error}"))?;
        let audience = audience
            .ok_or("PHYSICIAN_SCHEDULER_SERVICE_ACCOUNT requires PHYSICIAN_WORKER_AUDIENCE")?;
        return Ok(WakeAuth::Verified {
            verifier: audience_verifier(audience).build(),
            principal,
        });
    }
    if !hosted && configured(get, "PUBSUB_EMULATOR_HOST").is_some() {
        let address = listen_addr(get);
        if !is_loopback_listener(&address) {
            return Err(format!(
                "PUBSUB_EMULATOR_HOST disables schedule verification, so the worker must listen on loopback; {address} is not"
            ));
        }
        return Ok(WakeAuth::LoopbackEmulator);
    }
    Ok(WakeAuth::Disabled {
        verifier: audience.map(|aud| audience_verifier(aud).build()),
    })
}

const GOOGLE_OIDC_JWKS_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";
const GOOGLE_OIDC_ISSUERS: [&str; 2] = ["https://accounts.google.com", "accounts.google.com"];
const MAX_OIDC_KEY_BYTES: usize = 256 * 1024;
const MAX_OIDC_TOKEN_BYTES: usize = 16 * 1024;
const OIDC_JWKS_CACHE_TTL: Duration = Duration::from_secs(300);
const OIDC_JWKS_REFRESH_INTERVAL: Duration = Duration::from_secs(30);

#[derive(Default)]
struct OidcJwksCache {
    keys: Option<jsonwebtoken::jwk::JwkSet>,
    fetched: Option<std::time::Instant>,
    attempted: Option<std::time::Instant>,
}

struct GoogleOidcVerifierBuilder {
    audience: String,
    key_url: String,
    https_only: bool,
}

impl GoogleOidcVerifierBuilder {
    #[cfg(test)]
    fn with_jwks_url(mut self, url: String) -> Self {
        self.https_only = url.starts_with("https://");
        self.key_url = url;
        self
    }

    fn build(self) -> GoogleOidcVerifier {
        let client = reqwest::Client::builder()
            .https_only(self.https_only)
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(3))
            .connect_timeout(Duration::from_secs(2))
            .build()
            .expect("OIDC JWKS TLS HTTP client");
        GoogleOidcVerifier {
            audience: self.audience,
            client,
            cache: Arc::new(tokio::sync::Mutex::new(OidcJwksCache::default())),
            key_url: self.key_url,
        }
    }
}

/// Fail-closed Google OIDC ID token verifier for Cloud Scheduler wakes.
#[derive(Clone)]
struct GoogleOidcVerifier {
    audience: String,
    client: reqwest::Client,
    cache: Arc<tokio::sync::Mutex<OidcJwksCache>>,
    key_url: String,
}

impl GoogleOidcVerifier {
    async fn verify(&self, token: &str) -> Result<Value, &'static str> {
        if self.audience.trim().is_empty() || token.len() > MAX_OIDC_TOKEN_BYTES {
            return Err("invalid OIDC token or audience");
        }
        let header = jsonwebtoken::decode_header(token).map_err(|_| "malformed JWT header")?;
        if header.alg != jsonwebtoken::Algorithm::RS256 {
            return Err("unexpected JWT algorithm");
        }
        let kid = header
            .kid
            .filter(|kid| !kid.is_empty() && kid.len() <= 256)
            .ok_or("missing JWT kid")?;
        let key = self.key(&kid).await.ok_or("unknown OIDC signing key")?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "system clock unavailable")?
            .as_secs();
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_issuer(&GOOGLE_OIDC_ISSUERS);
        validation.set_audience(&[self.audience.as_str()]);
        validation.set_required_spec_claims(&["exp", "iat", "iss", "aud"]);
        validation.validate_exp = false;
        validation.validate_nbf = false;
        let claims = jsonwebtoken::decode::<Value>(token, &key, &validation)
            .map_err(|_| "JWT verification failed")?
            .claims;
        let exp = claims
            .get("exp")
            .and_then(Value::as_u64)
            .ok_or("missing exp")?;
        let iat = claims
            .get("iat")
            .and_then(Value::as_u64)
            .ok_or("missing iat")?;
        let nbf_ok = match claims.get("nbf") {
            None => true,
            Some(v) => v
                .as_u64()
                .is_some_and(|nbf| nbf <= now.saturating_add(30) && nbf < exp),
        };
        if exp <= now.saturating_sub(30) || iat >= now.saturating_add(30) || exp <= iat || !nbf_ok {
            return Err("OIDC token outside valid time window");
        }
        Ok(claims)
    }

    async fn key(&self, kid: &str) -> Option<jsonwebtoken::DecodingKey> {
        let mut cache = self.cache.lock().await;
        let fresh = cache
            .fetched
            .is_some_and(|at| at.elapsed() < OIDC_JWKS_CACHE_TTL);
        if fresh
            && let Some(key) = cache
                .keys
                .as_ref()
                .and_then(|keys| oidc_decoding_key(keys, kid))
        {
            return Some(key);
        }
        if cache
            .attempted
            .is_some_and(|at| at.elapsed() < OIDC_JWKS_REFRESH_INTERVAL)
        {
            return None;
        }
        cache.attempted = Some(std::time::Instant::now());
        let keys = self.fetch_keys().await?;
        cache.keys = Some(keys);
        cache.fetched = Some(std::time::Instant::now());
        cache
            .keys
            .as_ref()
            .and_then(|keys| oidc_decoding_key(keys, kid))
    }

    async fn fetch_keys(&self) -> Option<jsonwebtoken::jwk::JwkSet> {
        let mut response = self.client.get(&self.key_url).send().await.ok()?;
        if !response.status().is_success()
            || response
                .content_length()
                .is_some_and(|len| len > MAX_OIDC_KEY_BYTES as u64)
        {
            return None;
        }
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.ok()? {
            if body.len() + chunk.len() > MAX_OIDC_KEY_BYTES {
                return None;
            }
            body.extend_from_slice(&chunk);
        }
        let keys: jsonwebtoken::jwk::JwkSet = serde_json::from_slice(&body).ok()?;
        if keys.keys.is_empty() || keys.keys.len() > 32 {
            return None;
        }
        Some(keys)
    }
}

fn oidc_decoding_key(
    keys: &jsonwebtoken::jwk::JwkSet,
    kid: &str,
) -> Option<jsonwebtoken::DecodingKey> {
    let mut matching = keys
        .keys
        .iter()
        .filter(|key| key.common.key_id.as_deref() == Some(kid));
    let key = matching.next()?;
    if matching.next().is_some()
        || key
            .common
            .key_algorithm
            .is_some_and(|alg| alg != jsonwebtoken::jwk::KeyAlgorithm::RS256)
        || key
            .common
            .public_key_use
            .as_ref()
            .is_some_and(|usage| *usage != jsonwebtoken::jwk::PublicKeyUse::Signature)
        || key
            .common
            .key_operations
            .as_ref()
            .is_some_and(|ops| !ops.contains(&jsonwebtoken::jwk::KeyOperations::Verify))
    {
        return None;
    }
    if !matches!(
        key.algorithm,
        jsonwebtoken::jwk::AlgorithmParameters::RSA(_)
    ) {
        return None;
    }
    jsonwebtoken::DecodingKey::from_jwk(key).ok()
}

fn audience_verifier(audience: String) -> GoogleOidcVerifierBuilder {
    GoogleOidcVerifierBuilder {
        audience,
        key_url: GOOGLE_OIDC_JWKS_URL.into(),
        https_only: true,
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

/// Developer-plan inference retains the synthetic-chart boundary of local
/// practice. The same auth owner still fences chat and revision publication.
struct WorkerAuthority {
    inner: RuntimeAuthority,
    synthetic_only: bool,
}

impl WorkerAuthority {
    /// The worker's authority for runs `model`'s transport makes.
    fn of(model: &ModelSource) -> Self {
        Self {
            inner: RuntimeAuthority,
            synthetic_only: model.synthetic_only(),
        }
    }
}

#[local_product::runtime::async_trait]
impl local_product::runtime::Authority for WorkerAuthority {
    async fn check(
        &mut self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.check_source_boundary(tx, scope).await?;
        self.inner.check(tx, scope, hash).await
    }

    async fn lock_publication(
        &mut self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.check_source_boundary(tx, scope).await?;
        self.inner.lock_publication(tx, scope, hash).await
    }
}

impl WorkerAuthority {
    /// Reads on the caller's transaction, which already holds one of the
    /// process's pooled connections, rather than waiting for a second.
    async fn check_source_boundary(
        &self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &Scope,
    ) -> Result<(), String> {
        model::source_boundary(self.synthetic_only, tx, &scope.workspace).await
    }

    pub(crate) async fn check_store_source_boundary(
        &self,
        store: &Store,
        scope: &Scope,
    ) -> Result<(), String> {
        if !self.synthetic_only {
            return Ok(());
        }
        let tx = store
            .connection()
            .begin()
            .await
            .map_err(|e| e.to_string())?;
        let check = self.check_source_boundary(&tx, scope).await;
        let _ = tx.commit().await;
        check
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
    use harness::engine::recipe::Provider;
    use harness::provider::ResponsesProvider;
    use model::ModelProvider;

    /// A lease long enough that its renewal beat is slower than a stall check.
    const LEASE_SECONDS: i64 = 540;

    pub(super) struct WaitingGate;

    #[tokio::test]
    async fn successful_work_keeps_renewing_its_ownership() {
        let renewals = std::sync::atomic::AtomicUsize::new(0);
        let hosted = while_owned(
            local_product::runtime::Lease::new(1).unwrap(),
            STALL,
            &Cancellation::default(),
            || async {
                renewals.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Ok::<_, std::io::Error>(())
            },
            async {
                tokio::time::sleep(Duration::from_millis(1500)).await;
                Ok(42)
            },
        )
        .await;
        let Hosted::Returned(result) = hosted else {
            panic!("work that returns is never dropped");
        };
        assert_eq!(result.unwrap(), 42);
        assert!(renewals.load(std::sync::atomic::Ordering::SeqCst) >= 5);
    }

    #[tokio::test]
    async fn renewal_failure_preserves_work_errors_and_classifies_interruption() {
        for other_error in [false, true] {
            let cancel = Cancellation::default();
            let hosted: Hosted<()> = while_owned(
                local_product::runtime::Lease::new(1).unwrap(),
                STALL,
                &cancel,
                || async {
                    Err(sea_orm::DbErr::Custom(
                        "renewal database unavailable".into(),
                    ))
                },
                async {
                    while !cancel.is_cancelled() {
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }
                    if other_error {
                        Err(harness::engine::Failure::Host(
                            "settlement failed during cancellation".into(),
                        ))
                    } else {
                        Err(harness::engine::Failure::Cancelled)
                    }
                },
            )
            .await;
            let Hosted::Returned(result) = hosted else {
                panic!("work that returns after cancellation is not dropped");
            };
            let error = result.unwrap_err();
            assert_eq!(
                matches!(error, harness::engine::Failure::Host(_)),
                other_error,
                "hosting must preserve the actual work error"
            );
            assert_eq!(
                harness::engine::AttemptFailure::of(&error, &cancel),
                harness::engine::AttemptFailure::Interrupted,
                "renewal loss must not charge a failed attempt"
            );
        }
    }

    /// Counts each renewal; every one succeeds.
    fn counted(
        renewals: &std::sync::atomic::AtomicUsize,
    ) -> impl FnMut() -> std::future::Ready<Result<(), std::io::Error>> + '_ {
        || {
            renewals.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            std::future::ready(Ok(()))
        }
    }

    #[tokio::test(start_paused = true)]
    async fn a_run_that_stops_moving_is_cancelled_and_its_claim_is_no_longer_renewed() {
        let renewals = std::sync::atomic::AtomicUsize::new(0);
        let cancel = Cancellation::default();
        let started = tokio::time::Instant::now();
        let hosted: Hosted<()> = tokio::time::timeout(
            Duration::from_secs(3600),
            while_owned(
                local_product::runtime::Lease::new(LEASE_SECONDS).unwrap(),
                STALL,
                &cancel,
                counted(&renewals),
                std::future::pending(),
            ),
        )
        .await
        .expect("hosting stops waiting for a run that stopped moving");
        assert!(matches!(hosted, Hosted::Lost));
        assert!(cancel.is_cancelled());
        assert_eq!(started.elapsed(), STALL);
        assert_eq!(
            renewals.load(std::sync::atomic::Ordering::SeqCst),
            5,
            "renewed every 108 seconds until the stall at 600 seconds, then never"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_wedged_run_is_dropped_after_renewal_fails() {
        let cancel = Cancellation::default();
        let started = tokio::time::Instant::now();
        let hosted: Hosted<()> = tokio::time::timeout(
            Duration::from_secs(3600),
            while_owned(
                local_product::runtime::Lease::new(LEASE_SECONDS).unwrap(),
                STALL,
                &cancel,
                || async {
                    Err(sea_orm::DbErr::Custom(
                        "renewal database unavailable".into(),
                    ))
                },
                std::future::pending(),
            ),
        )
        .await
        .expect("hosting stops waiting for a wedged run after renewal fails");
        assert!(matches!(hosted, Hosted::Lost));
        assert!(cancel.is_cancelled());
        assert_eq!(
            started.elapsed(),
            STALL,
            "the cancelled run is dropped at the stall, 600 seconds after its last step"
        );
    }

    /// A model that thinks for 15 minutes before it answers.
    struct SlowModel;
    impl Provider for SlowModel {
        async fn exchange(
            &mut self,
            _: &harness::engine::recipe::Request,
            _: &Cancellation,
            _: &RunLimits,
            _: &mut dyn harness::engine::recipe::Observer,
        ) -> Result<harness::engine::Turn, harness::engine::Cause> {
            tokio::time::sleep(Duration::from_secs(900)).await;
            Ok(harness::engine::Turn {
                items: vec![harness::engine::Item::assistant("Done")],
                receipt: json!({}),
                usage: harness::engine::recipe::Usage::default(),
            })
        }
    }

    /// Accepts every step of one chat run.
    struct OpenWork;
    #[async_trait::async_trait]
    impl harness::engine::Product for OpenWork {
        fn read_result(&mut self, _: &str) -> harness::engine::recipe::ToolFuture<Option<String>> {
            Box::pin(async { Ok(None) })
        }
        async fn finish_partial(
            &mut self,
            _: &harness::engine::Turn,
            _: &[harness::engine::LimitBackstop],
        ) -> Result<(), harness::engine::ProductError> {
            Err("this run has no partial result".into())
        }
        async fn window_seed(
            &mut self,
        ) -> Result<harness::engine::WindowSeed, harness::engine::ProductError> {
            Err("this run has no working windows".into())
        }
        async fn assert_current(&mut self) -> Result<(), harness::engine::ProductError> {
            Ok(())
        }
        async fn message_sources(
            &mut self,
        ) -> Result<Option<Vec<harness::engine::ProductMessage>>, harness::engine::ProductError>
        {
            Ok(None)
        }
        async fn admit_binding(
            &mut self,
            _: &harness::engine::recipe::RequestBinding,
        ) -> Result<(), harness::engine::ProductError> {
            Ok(())
        }
        async fn persist(
            &mut self,
            _: harness::engine::Event,
        ) -> Result<(), harness::engine::ProductError> {
            Ok(())
        }
        fn execute(
            &mut self,
            _: &harness::engine::Invocation,
            _: Value,
        ) -> harness::engine::recipe::ToolFuture {
            Box::pin(async { Err("this run offers no tools".into()) })
        }
        async fn finish(
            &mut self,
            _: &harness::engine::Turn,
        ) -> Result<harness::engine::recipe::Finish, harness::engine::ProductError> {
            Ok(harness::engine::recipe::Finish::Committed)
        }
    }

    #[tokio::test(start_paused = true)]
    async fn a_run_that_keeps_marking_steps_keeps_its_claim_past_the_stall_limit() {
        use harness::engine::recovery;
        let renewals = std::sync::atomic::AtomicUsize::new(0);
        let cancel = Cancellation::default();
        let work = harness::engine::recipe::Request {
            cache_key: None,
            model: "scripted".into(),
            context_window_tokens: 100_000,
            max_input_tokens: 100_000,
            options: Value::Null,
            instructions: "Answer".into(),
            items: vec![harness::engine::Item::Message {
                role: "user".into(),
                text: "Hello".into(),
            }],
            tools: vec![],
        }
        .freeze()
        .unwrap()
        .recover(
            "attempt",
            recovery::Records {
                events: Vec::new(),
                allowance_after: 0,
            },
            recovery::Messages {
                sources: &[],
                partial: None,
            },
            recovery::Inputs::default(),
        )
        .unwrap();
        let mut model = SlowModel;
        let mut product = OpenWork;
        let hosted = tokio::time::timeout(
            Duration::from_secs(3600),
            while_owned(
                local_product::runtime::Lease::new(LEASE_SECONDS).unwrap(),
                STALL,
                &cancel,
                counted(&renewals),
                work.run(&mut model, &mut product, RunLimits::default(), &cancel),
            ),
        )
        .await
        .expect("the run ends");
        let Hosted::Returned(result) = hosted else {
            panic!("a run that keeps marking steps is never dropped");
        };
        assert_eq!(result.unwrap().text(), "Done");
        assert!(!cancel.is_cancelled());
        assert_eq!(
            renewals.load(std::sync::atomic::Ordering::SeqCst),
            8,
            "renewed every 108 seconds through 900 seconds"
        );
    }

    /// A bus whose publishes each wait for the test to let one through, and
    /// that counts the publishes it starts.
    struct GatedBus {
        started: std::sync::atomic::AtomicUsize,
        gate: tokio::sync::Semaphore,
    }

    impl GatedBus {
        fn closed() -> Arc<Self> {
            Arc::new(Self {
                started: std::sync::atomic::AtomicUsize::new(0),
                gate: tokio::sync::Semaphore::new(0),
            })
        }

        fn started(&self) -> usize {
            self.started.load(std::sync::atomic::Ordering::SeqCst)
        }

        /// Waits until `count` publishes have started.
        async fn until_started(&self, count: usize) {
            while self.started() < count {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        }
    }

    #[async_trait::async_trait]
    impl Bus for GatedBus {
        async fn publish(&self, _: Event) -> Result<(), String> {
            self.started
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            self.gate
                .acquire()
                .await
                .map_err(|e| e.to_string())?
                .forget();
            Ok(())
        }
        fn subscribe(&self) -> physician_bus::Subscription {
            physician_bus::LocalBus::new(1).subscribe()
        }
        fn live(&self) -> bool {
            true
        }
    }

    fn scope() -> Scope {
        Scope {
            practice: "practice".into(),
            physician: "physician".into(),
            workspace: "workspace".into(),
            chat: "chat".into(),
        }
    }

    /// A bus that never answers holds no shown step: each notice returns at
    /// once, so the run that saved the step goes on.
    #[tokio::test(start_paused = true)]
    async fn a_hung_bus_never_holds_a_shown_step() {
        let bus = GatedBus::closed();
        let mut notice = StepNotice::new(bus.clone(), "workspace".into());
        for _ in 0..100 {
            notice.shown_step_saved(&scope());
        }
        bus.until_started(1).await;
        tokio::time::sleep(Duration::from_secs(3600)).await;
        assert_eq!(bus.started(), 1, "one publish waits on the hung bus");
        drop(notice);
    }

    /// Notices that arrive while a publish is in flight become one publish
    /// after it, so a burst refetches few times and its last step is still
    /// announced, also once the attempt has dropped its notice.
    #[tokio::test(start_paused = true)]
    async fn a_burst_of_shown_steps_publishes_once_more_after_the_last() {
        let bus = GatedBus::closed();
        let mut notice = StepNotice::new(bus.clone(), "workspace".into());
        notice.shown_step_saved(&scope());
        bus.until_started(1).await;
        for _ in 0..10 {
            notice.shown_step_saved(&scope());
        }
        drop(notice);
        bus.gate.add_permits(1);
        bus.until_started(2).await;
        bus.gate.add_permits(1);
        tokio::time::sleep(Duration::from_secs(60)).await;
        assert_eq!(
            bus.started(),
            2,
            "eleven notices, one publish in flight and one after the burst"
        );
    }

    #[async_trait::async_trait]
    impl harness::provider::QuotaGate for WaitingGate {
        async fn remaining(&self) -> Result<Duration, harness::engine::Cause> {
            Ok(Duration::from_secs(1))
        }
        async fn defer(&self, _: Duration) -> Result<(), harness::engine::Cause> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn generation_telemetry_uses_the_dispatched_recipe_model() {
        use opentelemetry::trace::TracerProvider as _;
        use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
        use tracing::instrument::WithSubscriber;
        use tracing_subscriber::layer::SubscriberExt;
        let exporter = InMemorySpanExporter::default();
        let tracer = SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let subscriber = tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(tracer.tracer("worker-test")));
        async {
            let span = genai::generation_span(
                &RunContext {
                    run_id: "accepted-work".into(),
                    job: "physician_chat",
                    recipe_version: "test".into(),
                    origin: genai::Origin::Simulation,
                    scenario_id: None,
                    session_id: None,
                },
                "openai",
                "responses",
            );
            let mut provider = ModelProvider {
                provider: ResponsesProvider::new(
                    "http://127.0.0.1:1/responses".into(),
                    "synthetic".into(),
                    Arc::new(WaitingGate),
                )
                .unwrap(),
                developer_plan: false,
                quota: Arc::new(WaitingGate),
            };
            let request = harness::engine::recipe::Request {
                cache_key: None,
                model: "accepted-model".into(),
                context_window_tokens: 272_000,
                max_input_tokens: 272_000,
                options: json!({}),
                instructions: "Owner".into(),
                tools: vec![],
                items: vec![],
            };
            let result = provider
                .exchange(
                    &request,
                    &Cancellation::default(),
                    &RunLimits::default(),
                    &mut |_| Ok(()),
                )
                .instrument(span)
                .await;
            assert!(
                result.is_err(),
                "the unavailable transport still names its attempted model"
            );
        }
        .with_subscriber(subscriber)
        .await;
        tracer.force_flush().unwrap();
        let spans = exporter.get_finished_spans().unwrap();
        let generation = spans
            .iter()
            .find(|span| span.name == "gen_ai.generation")
            .unwrap();
        let model = generation
            .attributes
            .iter()
            .find(|field| field.key.as_str() == "gen_ai.request.model")
            .expect("telemetry must record the dispatched recipe model");
        assert_eq!(
            model.value.to_string(),
            "accepted-model",
            "telemetry must name the dispatched recipe model"
        );
    }

    #[test]
    fn pace_is_hosted_by_default_and_proof_values_are_refused_when_hosted() {
        let none = |_: &str| None;
        assert_eq!(Pace::from_env(&none, true), Ok(Pace::HOSTED));
        assert_eq!(Pace::from_env(&none, false), Ok(Pace::HOSTED));
        for key in [
            "PHYSICIAN_WORKER_POLL_MS",
            "PHYSICIAN_WORKER_LEASE_SECONDS",
            "PHYSICIAN_WORKER_RETRY_SECONDS",
        ] {
            let set = |name: &str| (name == key).then(|| "1".to_owned());
            assert_eq!(Pace::from_env(&set, true), Err(PaceError::Hosted { key }));
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
    fn hosted_worker_refuses_local_provider_and_proof_pacing() {
        let hosted = |extra: &'static str| {
            move |name: &str| match name {
                "K_SERVICE" => Some("physician-worker".into()),
                "PHYSICIAN_RESPONSES_STUB_PORT" if extra == "stub" => Some("8080".into()),
                "PHYSICIAN_WORKER_POLL_MS" if extra == "pace" => Some("250".into()),
                "CODA_ALLOW_MODEL_CALLS" => Some("1".into()),
                "OPENAI_API_KEY" => Some("synthetic-key".into()),
                _ => None,
            }
        };
        let refusal = |extra| block(Worker::from_env(hosted(extra))).err().unwrap();
        assert!(
            refusal("stub")
                .to_string()
                .contains("public Responses endpoint")
        );
        assert!(
            refusal("pace")
                .to_string()
                .contains("PHYSICIAN_WORKER_POLL_MS")
        );
    }

    fn block<T>(future: impl std::future::Future<Output = T>) -> T {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(future)
    }

    /// A developer's own ChatGPT plan never reaches patient data: the worker
    /// refuses any workspace that holds a source outside the synthetic practice.
    #[tokio::test]
    #[ignore = "requires explicit local Postgres"]
    async fn developer_plan_refuses_a_workspace_with_sources_outside_the_synthetic_practice() {
        use local_product::runtime::Authority;
        use sea_orm::TransactionTrait;
        let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();
        let binding = local_product::ModelBinding {
            model: "configured-model".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 272_000,
            effort: "low".into(),
            adapter: "fixed".into(),
        };
        let model = |transport| ModelSource {
            transport,
            roles: local_product::ModelRoles {
                chat: binding.clone(),
                background: binding.clone(),
            },
        };
        let scope = |workspace: &str| Scope {
            practice: "practice".into(),
            physician: "physician".into(),
            workspace: workspace.into(),
            chat: "chat".into(),
        };
        let record = |workspace: &str, source_id: &str| {
            let content = format!("{source_id} record");
            local_product::physician_sources::SourceRevision {
                workspace: workspace.into(),
                source_id: source_id.into(),
                candidate_id: format!("Observation/{source_id}"),
                revision: "r1".into(),
                record_type: "Observation".into(),
                dates: vec![],
                sha256: local_product::sha256_hex(content.as_bytes()),
                content,
                readable_parts: vec![],
                provenance: json!({"native_record": "Observation"}),
                envelope: None,
            }
        };
        let synthetic = physician_sample::synthetic_chart::SOURCE_ID;
        let mixed = uuid::Uuid::new_v4().to_string();
        store
            .record_source(&record(&mixed, synthetic))
            .await
            .unwrap();
        store.record_source(&record(&mixed, "ehr")).await.unwrap();
        let practice = uuid::Uuid::new_v4().to_string();
        store
            .record_source(&record(&practice, synthetic))
            .await
            .unwrap();
        let refusal =
            Err("Developer ChatGPT plans can read only the synthetic practice".to_owned());

        // A developer plan refuses the mixed workspace before the auth owner runs,
        // for the check and for publication.
        let mut plan = WorkerAuthority::of(&model(ModelTransport::DeveloperPlan));
        // The check reads on the caller's transaction: on a pool of one, a
        // second connection taken while it is held would time out.
        let mut single = sea_orm::ConnectOptions::new(url.as_str());
        single
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(2))
            .sqlx_logging(false);
        let db = &sea_orm::Database::connect(single).await.unwrap();
        let tx = db.begin().await.unwrap();
        assert_eq!(
            plan.check(&tx, &scope(&mixed), Some(&[0; 32])).await,
            refusal
        );
        assert_eq!(
            plan.lock_publication(&tx, &scope(&mixed), Some(&[0; 32]))
                .await,
            refusal
        );
        tx.rollback().await.unwrap();

        // A Responses key may read the same workspace.
        let responses = WorkerAuthority::of(&model(ModelTransport::Responses {
            endpoint: "http://127.0.0.1:1/responses".into(),
            key: "synthetic-key".into(),
        }));
        let tx = db.begin().await.unwrap();
        responses
            .check_source_boundary(&tx, &scope(&mixed))
            .await
            .unwrap();

        // A developer plan may read a workspace that holds only the synthetic practice.
        plan.check_source_boundary(&tx, &scope(&practice))
            .await
            .unwrap();
        tx.rollback().await.unwrap();

        // A record from outside the synthetic practice that was later withdrawn
        // still counts: answers written from it stay in the workspace's history.
        let withdrawn = uuid::Uuid::new_v4().to_string();
        store
            .record_source(&record(&withdrawn, synthetic))
            .await
            .unwrap();
        store
            .record_source(&record(&withdrawn, "ehr"))
            .await
            .unwrap();
        store
            .withdraw_source(&withdrawn, "Observation/ehr")
            .await
            .unwrap();
        let current = store.sources(&withdrawn).await.unwrap().sources;
        assert!(current.iter().all(|source| source.source_id == synthetic));
        let tx = db.begin().await.unwrap();
        assert_eq!(
            plan.check_source_boundary(&tx, &scope(&withdrawn)).await,
            refusal
        );
        tx.rollback().await.unwrap();
    }

    struct RecordingWakeHandler {
        ticks: std::sync::Mutex<Vec<WakeTick>>,
        fail_with_store_error: std::sync::atomic::AtomicBool,
    }

    impl RecordingWakeHandler {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                ticks: std::sync::Mutex::new(Vec::new()),
                fail_with_store_error: std::sync::atomic::AtomicBool::new(false),
            })
        }

        fn recorded(&self) -> Vec<WakeTick> {
            self.ticks.lock().unwrap().clone()
        }
    }

    #[async_trait::async_trait]
    impl WakeHandler for RecordingWakeHandler {
        async fn wake(&self, tick: &WakeTick) -> Result<WakeReport, WakeError> {
            self.ticks.lock().unwrap().push(tick.clone());
            if tick.directive == WakeDirective::RehearseFailure {
                return Err(WakeError::Rehearsed);
            }
            if self
                .fail_with_store_error
                .load(std::sync::atomic::Ordering::SeqCst)
            {
                return Err(WakeError::RuntimeStore(sea_orm::DbErr::Custom(
                    "database connection pool timed out".into(),
                )));
            }
            Ok(WakeReport {
                due_patients_checked: 3,
                reviews_started: 1,
                ..WakeReport::default()
            })
        }
    }

    #[derive(Clone, Default)]
    struct CapturedEvents(Arc<std::sync::Mutex<Vec<serde_json::Map<String, Value>>>>);

    struct MapVisitor<'a>(&'a mut serde_json::Map<String, Value>);

    impl tracing::field::Visit for MapVisitor<'_> {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            self.0
                .insert(field.name().to_owned(), Value::String(value.to_owned()));
        }
        fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
            self.0.insert(field.name().to_owned(), Value::Bool(value));
        }
        fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
            self.0.insert(field.name().to_owned(), value.into());
        }
        fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
            self.0.insert(field.name().to_owned(), value.into());
        }
        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            self.0
                .insert(field.name().to_owned(), Value::String(format!("{value:?}")));
        }
    }

    impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for CapturedEvents {
        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _ctx: tracing_subscriber::layer::Context<'_, S>,
        ) {
            let mut fields = serde_json::Map::new();
            fields.insert(
                "level".into(),
                Value::String(event.metadata().level().to_string()),
            );
            event.record(&mut MapVisitor(&mut fields));
            self.0.lock().unwrap().push(fields);
        }
    }

    async fn spawn_jwks_fixture() -> (
        tokio::task::JoinHandle<()>,
        String,
        jsonwebtoken::EncodingKey,
        jsonwebtoken::Header,
    ) {
        use base64::Engine as _;
        use jsonwebtoken::{Algorithm, EncodingKey, Header};
        let fixture: Value = serde_json::from_str(include_str!(
            "../../tests/fixtures/scheduler-public-test-key.json"
        ))
        .unwrap();
        let der = base64::engine::general_purpose::STANDARD
            .decode(
                fixture["private_rsa_der_base64"]
                    .as_str()
                    .expect("private_rsa_der_base64"),
            )
            .unwrap();
        let key = EncodingKey::from_rsa_der(&der);
        let jwks = json!({ "keys": fixture["keys"] });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/certs", listener.local_addr().unwrap());
        let handle = tokio::spawn(async move {
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
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some("scheduler-public-test-key".into());
        (handle, endpoint, key, header)
    }

    #[tokio::test]
    async fn schedule_wake_accepts_dedicated_scheduler_principal_and_logs_wake_tick() {
        use tower::ServiceExt;
        use tracing::instrument::WithSubscriber;
        use tracing_subscriber::layer::SubscriberExt;

        let (jwks_server, jwks_url, key, header) = spawn_jwks_fixture().await;
        let audience = "https://physician-worker.codahealth.test";
        let scheduler_sa = "physician-scheduler@coda-rust-staging.iam.gserviceaccount.com";
        let verifier = audience_verifier(audience.into())
            .with_jwks_url(jwks_url)
            .build();
        let handler = RecordingWakeHandler::new();
        let app = wake_router(Arc::new(WakeService {
            auth: WakeAuth::Verified {
                verifier,
                principal: hosting::ServiceAccountEmail::parse(scheduler_sa).unwrap(),
            },
            mode: WakeMode::RehearsalAllowed,
            handler: handler.clone(),
            clinical_library: None,
        }));

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let claims = json!({
            "iss": "https://accounts.google.com",
            "aud": audience,
            "sub": "synthetic-scheduler-sa",
            "email": scheduler_sa,
            "email_verified": true,
            "iat": now - 30,
            "exp": now + 600,
        });
        let signed = format!(
            "Bearer {}",
            jsonwebtoken::encode(&header, &claims, &key).unwrap()
        );

        let events = CapturedEvents::default();
        let subscriber = tracing_subscriber::registry().with(events.clone());
        let response = async {
            let request = axum::http::Request::builder()
                .method("POST")
                .uri("/schedule/wake")
                .header(axum::http::header::AUTHORIZATION, &signed)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .header(
                    "X-CloudScheduler-JobName",
                    "projects/coda-rust-staging/locations/us-east1/jobs/physician-schedule",
                )
                .header("X-CloudScheduler-ScheduleTime", "2026-11-12T06:00:00Z")
                .header("X-CloudScheduler-JobExecutionCount", "0")
                .body(axum::body::Body::from("{}"))
                .unwrap();
            app.oneshot(request).await.unwrap()
        }
        .with_subscriber(subscriber)
        .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(response.into_body(), BODY_LIMIT)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body["status"], "ok");
        assert_eq!(body["due_patients_checked"], 3);

        let recorded = handler.recorded();
        assert_eq!(
            recorded,
            vec![WakeTick {
                job_name: Some(
                    "projects/coda-rust-staging/locations/us-east1/jobs/physician-schedule".into()
                ),
                schedule_time: Some("2026-11-12T06:00:00Z".into()),
                retry_count: 0,
                directive: WakeDirective::Normal,
            }]
        );

        let captured = events.0.lock().unwrap();
        let wake_log = captured
            .iter()
            .find(|entry| {
                entry.get("event").and_then(Value::as_str) == Some("physician.worker.schedule_wake")
            })
            .expect("structured wake acceptance log emitted");
        assert_eq!(wake_log["level"], "INFO");
        assert_eq!(
            wake_log["scheduler.job"],
            "projects/coda-rust-staging/locations/us-east1/jobs/physician-schedule"
        );
        assert_eq!(wake_log["scheduler.retry_count"], 0);

        jwks_server.abort();
    }

    #[tokio::test]
    async fn schedule_wake_refuses_missing_invalid_and_wrong_service_account_tokens_before_dispatch()
     {
        use tower::ServiceExt;

        let (jwks_server, jwks_url, key, header) = spawn_jwks_fixture().await;
        let audience = "https://physician-worker.codahealth.test";
        let scheduler_sa = "physician-scheduler@coda-rust-staging.iam.gserviceaccount.com";
        let push_sa = "physician-pubsub-push@coda-rust-staging.iam.gserviceaccount.com";
        let verifier = audience_verifier(audience.into())
            .with_jwks_url(jwks_url.clone())
            .build();
        let handler = RecordingWakeHandler::new();
        let app = wake_router(Arc::new(WakeService {
            auth: WakeAuth::Verified {
                verifier,
                principal: hosting::ServiceAccountEmail::parse(scheduler_sa).unwrap(),
            },
            mode: WakeMode::RehearsalAllowed,
            handler: handler.clone(),
            clinical_library: None,
        }));

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let base_claims = json!({
            "iss": "https://accounts.google.com",
            "aud": audience,
            "sub": "synthetic-sa",
            "email": scheduler_sa,
            "email_verified": true,
            "iat": now - 30,
            "exp": now + 600,
        });
        let sign = |claims: &Value| {
            format!(
                "Bearer {}",
                jsonwebtoken::encode(&header, claims, &key).unwrap()
            )
        };
        let send = |router: Router, auth: Option<String>| async move {
            let mut builder = axum::http::Request::builder()
                .method("POST")
                .uri("/schedule/wake")
                .header(axum::http::header::CONTENT_TYPE, "application/json");
            if let Some(auth) = auth {
                builder = builder.header(axum::http::header::AUTHORIZATION, auth);
            }
            router
                .oneshot(builder.body(axum::body::Body::from("{}")).unwrap())
                .await
                .unwrap()
                .status()
        };

        // Missing or unauthenticated tokens -> 401 Unauthorized.
        assert_eq!(send(app.clone(), None).await, StatusCode::UNAUTHORIZED);
        assert_eq!(
            send(app.clone(), Some("Basic dXNlcjpwYXNz".into())).await,
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            send(app.clone(), Some("Bearer not-a-jwt".into())).await,
            StatusCode::UNAUTHORIZED
        );
        for (field, bad_value) in [
            ("aud", json!("https://other-worker.codahealth.test")),
            ("iss", json!("https://untrusted.codahealth.test")),
            ("exp", json!(now - 120)),
        ] {
            let mut invalid = base_claims.clone();
            invalid[field] = bad_value;
            assert_eq!(
                send(app.clone(), Some(sign(&invalid))).await,
                StatusCode::UNAUTHORIZED,
                "invalid {field} must fail 401"
            );
        }

        // Validly signed Google OIDC token for the Pub/Sub push SA (wrong principal) -> 403 Forbidden!
        let mut wrong_principal = base_claims.clone();
        wrong_principal["email"] = json!(push_sa);
        assert_eq!(
            send(app.clone(), Some(sign(&wrong_principal))).await,
            StatusCode::FORBIDDEN
        );

        let mut unverified_email = base_claims.clone();
        unverified_email["email_verified"] = json!(false);
        assert_eq!(
            send(app.clone(), Some(sign(&unverified_email))).await,
            StatusCode::FORBIDDEN
        );

        // Disabled schedule: missing token -> 401, valid token -> 403.
        let disabled_app = wake_router(Arc::new(WakeService {
            auth: WakeAuth::Disabled {
                verifier: Some(
                    audience_verifier(audience.into())
                        .with_jwks_url(jwks_url)
                        .build(),
                ),
            },
            mode: WakeMode::LiveOnly,
            handler: handler.clone(),
            clinical_library: None,
        }));
        assert_eq!(
            send(disabled_app.clone(), None).await,
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            send(disabled_app, Some(sign(&base_claims))).await,
            StatusCode::FORBIDDEN
        );

        assert!(
            handler.recorded().is_empty(),
            "no refused request may reach WakeHandler"
        );
        jwks_server.abort();
    }

    #[tokio::test]
    async fn schedule_wake_forced_failure_returns_503_for_scheduler_retry_and_emits_alert_log() {
        use std::error::Error as _;
        use tower::ServiceExt;
        use tracing::instrument::WithSubscriber;
        use tracing_subscriber::layer::SubscriberExt;

        let handler = RecordingWakeHandler::new();
        let rehearsal_app = wake_router(Arc::new(WakeService {
            auth: WakeAuth::LoopbackEmulator,
            mode: WakeMode::RehearsalAllowed,
            handler: handler.clone(),
            clinical_library: None,
        }));

        let events = CapturedEvents::default();
        let subscriber = tracing_subscriber::registry().with(events.clone());
        let (rehearsed_status, store_status) = async {
            let rehearsed = rehearsal_app
                .clone()
                .oneshot(
                    axum::http::Request::builder()
                        .method("POST")
                        .uri("/schedule/wake")
                        .header(axum::http::header::CONTENT_TYPE, "application/json")
                        .header("X-CloudScheduler-JobExecutionCount", "2")
                        .body(axum::body::Body::from(r#"{"rehearse_failure":true}"#))
                        .unwrap(),
                )
                .await
                .unwrap()
                .status();

            handler
                .fail_with_store_error
                .store(true, std::sync::atomic::Ordering::SeqCst);
            let store_failed = rehearsal_app
                .oneshot(
                    axum::http::Request::builder()
                        .method("POST")
                        .uri("/schedule/wake")
                        .header(axum::http::header::CONTENT_TYPE, "application/json")
                        .body(axum::body::Body::from("{}"))
                        .unwrap(),
                )
                .await
                .unwrap()
                .status();
            (rehearsed, store_failed)
        }
        .with_subscriber(subscriber)
        .await;

        assert_eq!(rehearsed_status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(store_status, StatusCode::SERVICE_UNAVAILABLE);

        let captured = events.0.lock().unwrap().clone();
        let failure_logs: Vec<_> = captured
            .iter()
            .filter(|entry| {
                entry.get("event").and_then(Value::as_str)
                    == Some("physician.worker.schedule_wake_failed")
            })
            .collect();
        assert_eq!(failure_logs.len(), 2);
        assert_eq!(failure_logs[0]["level"], "ERROR");
        assert_eq!(failure_logs[0]["error.kind"], "rehearsed_failure");
        assert_eq!(failure_logs[0]["scheduler.retry_count"], 2);
        assert_eq!(failure_logs[1]["error.kind"], "runtime_store_unavailable");

        // LiveOnly mode refuses rehearsed failures with 403 Forbidden before calling WakeHandler.
        let live_handler = RecordingWakeHandler::new();
        let live_app = wake_router(Arc::new(WakeService {
            auth: WakeAuth::LoopbackEmulator,
            mode: WakeMode::LiveOnly,
            handler: live_handler.clone(),
            clinical_library: None,
        }));
        let live_status = live_app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/schedule/wake")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(r#"{"rehearse_failure":true}"#))
                    .unwrap(),
            )
            .await
            .unwrap()
            .status();
        assert_eq!(live_status, StatusCode::FORBIDDEN);
        assert!(live_handler.recorded().is_empty());

        let caused = WakeError::RuntimeStore(sea_orm::DbErr::Custom("db down".into()));
        assert!(caused.source().is_some());
    }

    #[test]
    fn wake_auth_validates_scheduler_principal_and_emulator_loopback() {
        let with_pairs = |pairs: &[(&str, &str)], hosted: bool| {
            wake_auth(
                &|name| {
                    pairs
                        .iter()
                        .find(|(key, _)| *key == name)
                        .map(|(_, value)| (*value).to_owned())
                },
                hosted,
            )
        };
        assert!(matches!(
            with_pairs(
                &[
                    (
                        "PHYSICIAN_WORKER_AUDIENCE",
                        "https://physician-worker.codahealth.test"
                    ),
                    (
                        "PHYSICIAN_SCHEDULER_SERVICE_ACCOUNT",
                        "physician-scheduler@coda-rust-staging.iam.gserviceaccount.com"
                    ),
                ],
                true
            ),
            Ok(WakeAuth::Verified { .. })
        ));
        assert!(
            with_pairs(
                &[(
                    "PHYSICIAN_SCHEDULER_SERVICE_ACCOUNT",
                    "physician-scheduler@coda-rust-staging.iam.gserviceaccount.com"
                )],
                true
            )
            .is_err(),
            "scheduler principal without audience must refuse startup"
        );
        assert!(
            with_pairs(
                &[
                    (
                        "PHYSICIAN_WORKER_AUDIENCE",
                        "https://physician-worker.codahealth.test"
                    ),
                    (
                        "PHYSICIAN_SCHEDULER_SERVICE_ACCOUNT",
                        "person@codahealth.test"
                    ),
                ],
                true
            )
            .is_err(),
            "human account must be rejected as scheduler principal"
        );
        assert!(matches!(
            with_pairs(&[("PUBSUB_EMULATOR_HOST", "127.0.0.1:8085")], false),
            Ok(WakeAuth::LoopbackEmulator)
        ));
        assert!(
            with_pairs(
                &[("PUBSUB_EMULATOR_HOST", "127.0.0.1:8085"), ("PORT", "8080"),],
                false
            )
            .is_err(),
            "emulator bypass on non-loopback listener must fail"
        );
    }

    #[tokio::test]
    async fn schedule_wake_executes_visit_preparation_against_synthetic_calendar_and_responses_stub()
     {
        use local_product::physician_sources::{
            MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SourcePatient,
            SourceRevision, workspace_id,
        };
        use local_product::runtime::briefings::VisitSlotKey;
        use local_product::runtime::concerns::ExactRecordReference;
        use local_product::runtime::schedule::AppointmentWindow;
        use local_product::runtime::sources::SourcePull;
        use local_product::visit_preparation::{
            BriefingClaimInput, BriefingDecisionInput, SAVE_BRIEFING_TOOL, SELECT_VISITS_TOOL,
            SaveBriefingArgs,
        };
        use physician_auth::{AuthStore, PracticeClock};
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tower::ServiceExt;

        let Some(gate_url) = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
            .or_else(|_| std::env::var("PHYSICIAN_DATABASE_URL"))
            .ok()
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
        else {
            assert!(
                std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
                "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL"
            );
            return;
        };

        let (server, _) = gate_url.rsplit_once('/').expect("database URL");
        let admin_url = format!("{server}/postgres");
        let db_name = format!(
            "worker_wake_vp_{}",
            &uuid::Uuid::new_v4().simple().to_string()[..12]
        );
        let admin = sea_orm::Database::connect(&admin_url).await.unwrap();
        super::disposable::replace_database(&admin, &db_name).await;
        admin.close().await.unwrap();
        let url = format!("{server}/{db_name}");

        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();
        let auth = PostgresAuthStore::connect_with_migrations(&url)
            .await
            .unwrap();

        let practice = format!("practice-worker-wake-{}", uuid::Uuid::new_v4().simple());
        let fixed_now: u64 = 1_787_846_400; // 2026-08-27T16:00:00Z
        auth.ensure_practice(&practice, PracticeClock::Fixed(fixed_now))
            .await
            .unwrap();

        let mrn = format!(
            "MRN-WKR-{}",
            &uuid::Uuid::new_v4().simple().to_string()[..8]
        );
        let ws = workspace_id(physician_sample::synthetic_chart::SOURCE_ID, &mrn);
        let note_text = "Stage 3a CKD (eGFR 51 mL/min) and A1c 8.1% ahead of tomorrow's follow-up.";
        store
            .record_chart(
                &ws,
                &[SourceRevision {
                    workspace: ws.clone(),
                    source_id: physician_sample::synthetic_chart::SOURCE_ID.into(),
                    candidate_id: "Observation/ckd-a1c-1".into(),
                    revision: "rev-wkr-1".into(),
                    record_type: "Observation".into(),
                    dates: vec![RecordDate {
                        name: "effectiveDateTime".into(),
                        value: "2026-08-26".into(),
                    }],
                    sha256: local_product::sha256_hex(note_text.as_bytes()),
                    content: note_text.into(),
                    readable_parts: vec![ReadablePart {
                        label: "Renal & A1c".into(),
                        text: note_text.into(),
                    }],
                    provenance: json!({ "title": "Renal & A1c" }),
                    envelope: None,
                }],
                &SourcePull {
                    source_id: physician_sample::synthetic_chart::SOURCE_ID.into(),
                    source_label: "Synthetic chart".into(),
                    patient: SourcePatient {
                        source_patient_id: mrn.clone(),
                        display_name: "Worker Wake Patient".into(),
                        birth_date: Some("1965-09-12".into()),
                        sex_or_gender: Some("male".into()),
                        medical_record_numbers: vec![MedicalRecordNumber {
                            system: "urn:mrn".into(),
                            value: mrn.clone(),
                        }],
                    },
                    pulled_at: Some(fixed_now as i64 - 600),
                },
            )
            .await
            .unwrap();
        auth.register_patient(&practice, &ws).await.unwrap();

        // Write a temporary synthetic bank calendar file with a visit booked for tomorrow:
        let temp_dir = std::env::temp_dir().join(format!(
            "coda-worker-wake-cal-{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(temp_dir.join("calendar")).unwrap();
        let appt_line = format!(
            r#"{{"resourceType":"Appointment","id":"appt-wkr-tomorrow","meta":{{"versionId":"1","lastUpdated":"2026-08-27T00:00:00+00:00"}},"status":"booked","description":"CKD & diabetes follow-up","start":"2026-08-28T15:00:00+00:00","end":"2026-08-28T15:30:00+00:00","participant":[{{"actor":{{"reference":"Patient/{mrn}"}},"status":"accepted"}}]}}"#
        );
        let cal_file = temp_dir.join("calendar/Appointment.ndjson");
        std::fs::write(&cal_file, format!("{appt_line}\n")).unwrap();
        let calendar = physician_sample::schedule::SyntheticBankCalendar::open(&temp_dir);
        physician_sample::schedule::sync_synthetic_schedule(&calendar, &auth, &store, &practice)
            .await
            .unwrap();

        // Seed a second scheduled practice in the same database so we verify
        // `VisitPrepWakeHandler` syncs `calendar` strictly for its owning practice
        // without overwriting `other_practice`'s schedule.
        let other_practice = "practice-worker-other".to_string();
        auth.ensure_practice(&other_practice, PracticeClock::Fixed(fixed_now))
            .await
            .unwrap();
        let other_temp_dir = std::env::temp_dir().join(format!(
            "coda-worker-other-cal-{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(other_temp_dir.join("calendar")).unwrap();
        let other_appt_line = r#"{"resourceType":"Appointment","id":"appt-other-practice-keep","meta":{"versionId":"1","lastUpdated":"2026-08-27T00:00:00+00:00"},"status":"booked","description":"Unmatched external consult","start":"2026-08-28T16:00:00+00:00","end":"2026-08-28T16:30:00+00:00","participant":[{"actor":{"reference":"Patient/UNMATCHED-OTHER"},"status":"accepted"}]}"#;
        std::fs::write(
            other_temp_dir.join("calendar/Appointment.ndjson"),
            format!("{other_appt_line}\n"),
        )
        .unwrap();
        let other_calendar =
            physician_sample::schedule::SyntheticBankCalendar::open(&other_temp_dir);
        physician_sample::schedule::sync_synthetic_schedule(
            &other_calendar,
            &auth,
            &store,
            &other_practice,
        )
        .await
        .unwrap();
        let _ = std::fs::remove_dir_all(&other_temp_dir);

        let starts_at_unix =
            AppointmentWindow::parse("2026-08-28T15:00:00+00:00", "2026-08-28T15:30:00+00:00")
                .unwrap()
                .starts_at_unix();

        // Spin up local HTTP SSE Responses stub for VisitPrepWakeHandler:
        let sel_args = serde_json::to_string(&json!({
            "decisions": [{
                "source_id": physician_sample::schedule::SOURCE_ID,
                "appointment_id": "appt-wkr-tomorrow",
                "starts_at_unix": starts_at_unix,
                "action": "prepare_now",
                "reason": "Tomorrow's CKD & diabetes visit has A1c 8.1% ready for review."
            }]
        }))
        .unwrap();
        let read_args = serde_json::to_string(&json!({
            "references": [{ "candidate_id": "Observation/ckd-a1c-1", "revision": "rev-wkr-1", "cursor": null }],
            "max_result_bytes": 16_384
        }))
        .unwrap();
        let save_args = serde_json::to_string(&SaveBriefingArgs {
            visit_card_orientation:
                "CKD 3a (eGFR 51) and A1c 8.1% ahead of tomorrow's follow-up; SGLT2i option open."
                    .into(),
            trajectory: vec![BriefingClaimInput {
                text: "Stage 3a CKD (eGFR 51 mL/min) with A1c 8.1% on 2026-08-26.".into(),
                uncertainty: None,
                citations: vec![ExactRecordReference {
                    candidate_id: "Observation/ckd-a1c-1".into(),
                    revision: "rev-wkr-1".into(),
                }],
            }],
            meaningful_changes: vec![BriefingClaimInput {
                text: "Pre-visit A1c is above target at 8.1%.".into(),
                uncertainty: None,
                citations: vec![ExactRecordReference {
                    candidate_id: "Observation/ckd-a1c-1".into(),
                    revision: "rev-wkr-1".into(),
                }],
            }],
            unresolved_issues: vec![BriefingClaimInput {
                text: "Urine albumin-to-creatinine ratio is not in the connected chart.".into(),
                uncertainty: Some("Albuminuria severity is unconfirmed pending spot UACR.".into()),
                citations: vec![ExactRecordReference {
                    candidate_id: "Observation/ckd-a1c-1".into(),
                    revision: "rev-wkr-1".into(),
                }],
            }],
            visit_decisions: vec![BriefingDecisionInput {
                decision: "Glycemic and renal protective therapy review".into(),
                clinical_rationale:
                    "eGFR 51 mL/min with A1c 8.1% supports discussing an SGLT2 inhibitor.".into(),
                options_or_questions: vec![
                    "Discuss adding an SGLT2 inhibitor and ordering spot UACR.".into(),
                ],
                uncertainty: None,
                citations: vec![ExactRecordReference {
                    candidate_id: "Observation/ckd-a1c-1".into(),
                    revision: "rev-wkr-1".into(),
                }],
            }],
            basis_and_limitations: "Prepared from synthetic chart observation 2026-08-26.".into(),
            basis_citations: Vec::new(),
            limits: Vec::new(),
            visit_concern: None,
        })
        .unwrap();

        let sse_frame = |mut response: Value| -> String {
            let mut events =
                vec![json!({"type":"response.created","response":{"id":response["id"]}})];
            for (idx, item) in response["output"]
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .enumerate()
            {
                item["id"] = json!(format!("item-{idx}"));
                let id = item["id"].clone();
                events.push(
                    json!({"type":"response.output_item.added","output_index":idx,"item":item}),
                );
                if item["type"] == "function_call" {
                    events.push(json!({"type":"response.function_call_arguments.delta","output_index":idx,"item_id":id,"delta":item["arguments"]}));
                    events.push(json!({"type":"response.function_call_arguments.done","output_index":idx,"item_id":id,"arguments":item["arguments"]}));
                } else if item["type"] == "message" {
                    let text = item["content"][0]["text"].clone();
                    events.push(json!({"type":"response.content_part.added","output_index":idx,"item_id":id,"content_index":0,"part":{"type":"output_text","text":""}}));
                    events.push(json!({"type":"response.output_text.delta","output_index":idx,"item_id":id,"content_index":0,"delta":text}));
                    events.push(json!({"type":"response.output_text.done","output_index":idx,"item_id":id,"content_index":0,"text":text}));
                    events.push(json!({"type":"response.content_part.done","output_index":idx,"item_id":id,"content_index":0,"part":item["content"][0]}));
                }
                events.push(
                    json!({"type":"response.output_item.done","output_index":idx,"item":item}),
                );
            }
            events.push(json!({"type":"response.completed","response":response}));
            events
                .into_iter()
                .enumerate()
                .map(|(i, mut ev)| {
                    ev["sequence_number"] = json!(i);
                    format!("data: {ev}\n\n")
                })
                .collect()
        };

        let payloads = vec![
            json!({"id":"r1","status":"completed","output":[{"type":"function_call","status":"completed","call_id":"c1","name":SELECT_VISITS_TOOL,"arguments":sel_args}],"usage":{"input_tokens":50,"output_tokens":20}}),
            json!({"id":"r2","status":"completed","output":[{"type":"message","role":"assistant","status":"completed","content":[{"type":"output_text","text":"Selected."}]}],"usage":{"input_tokens":60,"output_tokens":5}}),
            json!({"id":"r3","status":"completed","output":[{"type":"function_call","status":"completed","call_id":"c2","name":READ_TOOL,"arguments":read_args}],"usage":{"input_tokens":80,"output_tokens":15}}),
            json!({"id":"r4","status":"completed","output":[{"type":"function_call","status":"completed","call_id":"c3","name":SAVE_BRIEFING_TOOL,"arguments":save_args}],"usage":{"input_tokens":100,"output_tokens":60}}),
            json!({"id":"r5","status":"completed","output":[{"type":"message","role":"assistant","status":"completed","content":[{"type":"output_text","text":"Briefing saved."}]}],"usage":{"input_tokens":120,"output_tokens":5}}),
        ];

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/responses", listener.local_addr().unwrap());
        let stub_task = tokio::spawn(async move {
            for payload in payloads {
                let body = sse_frame(payload);
                let (mut sock, _) = listener.accept().await.unwrap();
                let mut buf = Vec::new();
                let (off, clen) = loop {
                    let mut chunk = [0_u8; 2048];
                    let n = sock.read(&mut chunk).await.unwrap();
                    buf.extend_from_slice(&chunk[..n]);
                    if let Some(end) = buf.windows(4).position(|p| p == b"\r\n\r\n") {
                        let hdr = String::from_utf8_lossy(&buf[..end]);
                        let len = hdr
                            .lines()
                            .find_map(|l| {
                                let (k, v) = l.split_once(':')?;
                                k.eq_ignore_ascii_case("content-length")
                                    .then(|| v.trim().parse::<usize>().unwrap())
                            })
                            .unwrap();
                        break (end + 4, len);
                    }
                };
                while buf.len() < off + clen {
                    let mut chunk = [0_u8; 2048];
                    let n = sock.read(&mut chunk).await.unwrap();
                    buf.extend_from_slice(&chunk[..n]);
                }
                let reply = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                sock.write_all(reply.as_bytes()).await.unwrap();
            }
        });

        let binding = local_product::ModelBinding {
            model: "worker-wake-bg-model".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            effort: "medium".into(),
            adapter: "openai-responses".into(),
        };
        let wake_bus = Arc::new(physician_bus::LocalBus::new(16));
        let mut bus_events = wake_bus.subscribe();
        let app = wake_router(Arc::new(WakeService {
            auth: WakeAuth::LoopbackEmulator,
            mode: WakeMode::RehearsalAllowed,
            handler: Arc::new(VisitPrepWakeHandler {
                store: store.clone(),
                auth: auth.clone(),
                bus: wake_bus,
                model: ModelSource {
                    transport: ModelTransport::Responses {
                        endpoint,
                        key: "local-proof".into(),
                    },
                    roles: local_product::ModelRoles {
                        chat: binding.clone(),
                        background: binding,
                    },
                },
                quota: Arc::new(store.clone()),
                skills_dir: None,
                calendar: Some(PracticeSyntheticCalendar {
                    practice: practice.clone(),
                    source: calendar,
                }),
                elation: None,
            }),
            clinical_library: None,
        }));

        let resp = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/schedule/wake")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp_bytes = axum::body::to_bytes(resp.into_body(), BODY_LIMIT)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&resp_bytes).unwrap();
        assert_eq!(body["status"], "ok");
        assert!(body["briefings_prepared"].as_u64().unwrap() >= 1);

        stub_task.await.unwrap();
        let saved = store
            .visit_briefing(
                &VisitSlotKey {
                    practice,
                    source_id: physician_sample::schedule::SOURCE_ID.into(),
                    appointment_id: "appt-wkr-tomorrow".into(),
                    starts_at_unix,
                },
                &auth,
            )
            .await
            .unwrap()
            .expect("POST /schedule/wake saved tomorrow's visit briefing");
        assert!(saved.visit_card_orientation.contains("CKD 3a"));

        // Verify background wake published live bus invalidation (`P4.Rule.BackgroundNoStealFocus`):
        let mut saw_resync = false;
        while let Ok(Ok(ev)) =
            tokio::time::timeout(Duration::from_millis(200), bus_events.recv()).await
        {
            if matches!(ev, Event::Resync) {
                saw_resync = true;
                break;
            }
        }
        assert!(
            saw_resync,
            "VisitPrepWakeHandler must publish Event::Resync on physician_bus after schedule/briefing updates"
        );

        // Verify `other_practice`'s schedule was NOT overwritten by `practice`'s synthetic calendar:
        let other_sched = store.practice_schedule(&other_practice).await.unwrap();
        assert_eq!(other_sched.appointments.len(), 1);
        assert_eq!(
            other_sched.appointments[0].appointment_id.as_str(),
            "appt-other-practice-keep"
        );

        drop(store);
        drop(auth);
        let cleanup = sea_orm::Database::connect(&admin_url).await.unwrap();
        super::disposable::drop_database(&cleanup, &db_name).await;
        let _ = cleanup.close().await;
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
