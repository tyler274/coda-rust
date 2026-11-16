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
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use harness::engine::{AttemptFailure, Cancellation, RunLimits};
use local_product::runtime::delivery::{
    AttemptSettlement, Backoff, Delivery, DeliveryKind, DeliveryState,
};

#[path = "physician-worker/attempt.rs"]
mod attempt;
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
    // A rehearsal turn marked `[fail:K]` fails its first K attempts before
    // any model call, so a drill exercises the real failure path at no cost.
    if worker.simulation_practices.rehearsed_failure(
        &claim.scope.practice,
        &input_text,
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
        let (transport, origin) = ModelTransport::from_env(&get, hosted)?;
        let pace = Pace::from_env(&get, hosted).map_err(|error| error.to_string())?;
        let model = ModelSource::from_env(transport, &get)?;
        let database = configured(&get, "PHYSICIAN_DATABASE_URL")
            .ok_or("PHYSICIAN_DATABASE_URL is required")?;
        // One pool per process: the auth store reads through the runtime's.
        let store = Store::connect(&database)
            .await
            .map_err(hosting::StartupError::RuntimeStore)?;
        let auth = PostgresAuthStore::from_connection(store.connection().clone());
        let bus = physician_bus::from_env(&get, hosted).await?;
        let quota: Arc<dyn harness::provider::QuotaGate> = Arc::new(store.clone());
        Ok(Self {
            store,
            auth,
            bus,
            model,
            quota,
            pace,
            origin,
            simulation_practices,
        })
    }
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
}
