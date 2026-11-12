//! Push worker for accepted physician chat turns.
//!
//! Postgres owns the turn. Pub/Sub only wakes this process. The push body
//! carries a scope and an input id, never clinical text, and logs stay that way.
//! A duplicate or settled delivery is acknowledged without calling the model.
//! Skill reads use the package saved with the work at acceptance.
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use base64::Engine;
use harness::engine::recipe::Provider;
use harness::engine::{Cancellation, RunLimits};
use harness::provider::ResponsesProvider;
use local_product::runtime::delivery::{Delivery, DeliveryKind, DeliveryState};

#[path = "physician-worker/attempt.rs"]
mod attempt;
#[path = "physician-worker/practice_job.rs"]
mod practice_job;
#[path = "physician-worker/revision.rs"]
mod revision;
use attempt::{Claimed, Failure, Outcome, Push, Tally};
use local_product::runtime::{Claim, Conversation, PhysicianHost, Scope, Store};
use physician_auth::PostgresAuthStore;
use physician_bus::{Bus, Event};
use physician_sample::durable::RuntimeAuthority;
use physician_sample::hosting::{self, SimulationPractices, is_hosted};
use serde_json::{Value, json};
use telemetry::genai::{self, RunContext};
use tracing::Instrument;

// A dead replica's work waits at most 540 seconds for its ownership to expire.
// Hosting renews every fifth of that duration, independently of the run clock.
const LEASE_SECONDS: i64 = 540;
// A run that marks no step for 600 seconds has stopped moving. Hermes stops
// renewing its session lease after the same 600 seconds without activity.
const STALL: std::time::Duration = std::time::Duration::from_secs(600);
// A Pub/Sub wake carries identities, not clinical payloads. Bound request buffering.
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
    let wake = worker.wake.clone();
    Router::new()
        .route("/health", get(health))
        .route("/readyz", get(readyz))
        .route("/pubsub/chat-turn", post(chat_turn))
        .with_state(worker)
        .merge(wake_router(wake))
        .layer(telemetry::http_layer())
}

fn wake_router(wake: Arc<WakeService>) -> Router {
    Router::new()
        .route("/schedule/wake", post(schedule_wake))
        .with_state(wake)
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
    // scopes name reply work, and practice jobs carry no physician session.
    match push.delivery.kind {
        DeliveryKind::ReplyRevision => return revision::deliver(&worker, &push).await,
        DeliveryKind::PracticeJob => return practice_job::deliver(&worker, &push).await,
        DeliveryKind::ChatTurn => {}
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
        let reason = harness::engine::AttemptFailure::Failed;
        return settle_attempt(worker, &claim, workspace_id, reason, &attempt).await;
    }
    let provider = worker.model.provider(worker.quota.clone()).await;
    drop(input_text);
    let mut host = PhysicianHost {
        store: worker.store.clone(),
        claim,
        authority: worker.model.authority(worker.store.clone()),
    };
    let cancel = Cancellation::default();
    // One provider per turn: concurrent pushes on this instance run side by
    // side instead of queueing behind one another's model call.
    let mut provider = match provider {
        Ok(provider) => provider,
        Err(error) => {
            genai::record_error(&generation, "physician chat provider unavailable");
            let reason = harness::engine::AttemptFailure::from_error(
                &harness::engine::Failure::Provider(error),
                &cancel,
            );
            let attempt = Settling {
                span,
                generation: &generation,
                failure: Failure::ProviderInit,
            };
            return settle_attempt(worker, &host.claim, workspace_id, reason, &attempt).await;
        }
    };
    let claimed = host.claim.clone();
    let hosted = while_owned(
        claimed.lease,
        STALL,
        &cancel,
        || worker.store.renew_claim(&claimed),
        host.run(&mut provider, RunLimits::default(), &cancel)
            .instrument(generation.clone()),
    )
    .await;
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
            // The turn is committed. Redelivery sees it settled and acks.
            announce(worker, workspace_id).await;
            attempt::finish(
                span,
                CHAT_JOB,
                Some(&host.claim.attempt),
                Outcome::Committed,
                None,
            );
            StatusCode::NO_CONTENT
        }
        Err(error) => {
            let reason = harness::engine::AttemptFailure::from_error(&error, &cancel);
            let attempt = Settling {
                span,
                generation: &generation,
                failure: Failure::ModelCall,
            };
            settle_attempt(worker, &host.claim, workspace_id, reason, &attempt).await
        }
    }
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

/// A run hosting dropped settles nothing. Refusing the delivery is what a lost
/// worker leaves: a redelivery finds the claim still live and acknowledges,
/// and after the lease expires the recovery sweep republishes the input.
fn lost(span: &tracing::Span, job: &'static str, attempt: &str) -> StatusCode {
    attempt::finish(
        span,
        job,
        Some(attempt),
        Outcome::Retry,
        Some(Failure::Lost),
    );
    StatusCode::INTERNAL_SERVER_ERROR
}

/// The spans and the primary failure class of an attempt being settled.
struct Settling<'a> {
    /// The delivery's `physician.worker.attempt` span.
    span: &'a tracing::Span,
    /// The attempt's `gen_ai.generation` span.
    generation: &'a tracing::Span,
    failure: Failure,
}

/// Retain the shared failure classification, then announce the durable state.
/// Actual failures refuse the delivery for broker retry or dead-letter handling;
/// a reached limit acknowledges the delivery and stays held for explicit Resume.
async fn settle_attempt(
    worker: &Worker,
    claim: &Claim,
    workspace_id: String,
    reason: harness::engine::AttemptFailure,
    attempt: &Settling<'_>,
) -> StatusCode {
    use local_product::runtime::delivery::AttemptSettlement;
    let run_id = Some(claim.attempt.as_str());
    let settlement = worker.store.settle_chat_attempt(claim, reason).await;
    let status = match settlement {
        Err(_) => {
            attempt::retry_not_persisted(attempt.span, CHAT_JOB, &claim.attempt);
            let failure = Some(attempt.failure);
            attempt::finish(attempt.span, CHAT_JOB, run_id, Outcome::Retry, failure);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        Ok(AttemptSettlement::Applied) if reason.counts_as_failure() => {
            genai::record_error(attempt.generation, "physician chat failed");
            let failure = Some(attempt.failure);
            attempt::finish(attempt.span, CHAT_JOB, run_id, Outcome::Retry, failure);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        // Superseded delivery cannot consume the successor's broker allowance.
        Ok(AttemptSettlement::Applied | AttemptSettlement::Superseded) => {
            attempt::finish(attempt.span, CHAT_JOB, run_id, Outcome::Settled, None);
            StatusCode::NO_CONTENT
        }
    };
    announce(worker, workspace_id).await;
    status
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
        .find(|message| message.role.as_str() == role)
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
    /// One durable provider cooldown for this product across worker replicas.
    quota: Arc<dyn harness::provider::QuotaGate>,
    verifier: Option<google_cloud_auth::credentials::idtoken::verifier::Verifier>,
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

    async fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let hosted = is_hosted(&get);
        let simulation_practices = SimulationPractices::from_env(&get)?;
        let clinical_library = hosting::ClinicalLibraryConfig::from_env(&get)
            .map_err(|error| error.to_string())?
            .map(hosting::ClinicalLibraryClient::new)
            .transpose()
            .map_err(|error| error.to_string())?;
        let (transport, origin) = if let Some(port) =
            configured(&get, "PHYSICIAN_RESPONSES_STUB_PORT")
        {
            if hosted {
                return Err(
                    "hosted physician worker requires the public Responses endpoint".into(),
                );
            }
            let port: u16 = port
                .parse()
                .map_err(|_| "invalid local Responses stub port")?;
            (
                ModelTransport::Responses {
                    endpoint: format!("http://127.0.0.1:{port}/responses"),
                    key: "local-proof".to_owned(),
                },
                genai::Origin::Simulation,
            )
        } else {
            match if hosted {
                local_product::ModelExecution::Api
            } else {
                local_product::model_execution(
                    configured(&get, "CODA_LOCAL_MODEL_EXECUTION").as_deref(),
                )?
            } {
                local_product::ModelExecution::Subscription => {
                    local_product::chatgpt::require_sign_in().map_err(|error| error.to_string())?;
                    (ModelTransport::DeveloperPlan, genai::Origin::Real)
                }
                local_product::ModelExecution::Api => {
                    let key = local_product::open_ai_api_key(
                        configured(&get, "CODA_ALLOW_MODEL_CALLS").as_deref(),
                        configured(&get, "OPENAI_API_KEY").as_deref(),
                    )?;
                    (
                        ModelTransport::Responses {
                            endpoint: harness::responses::ENDPOINT.to_owned(),
                            key,
                        },
                        genai::Origin::Real,
                    )
                }
                local_product::ModelExecution::Disabled => {
                    return Err(
                        "physician worker requires CODA_LOCAL_MODEL_EXECUTION=subscription or api"
                            .into(),
                    );
                }
            }
        };
        let verifier = push_verifier(&get, hosted)?;
        let schedule_auth = wake_auth(&get, hosted)?;
        let wake_mode = if origin == genai::Origin::Simulation || hosting::is_staging_preview(&get)
        {
            WakeMode::RehearsalAllowed
        } else {
            WakeMode::LiveOnly
        };
        let owned = role_environment(&get);
        let environment: Vec<_> = owned
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();
        let roles = local_product::model_roles(transport.adapter(), &environment)
            .map_err(|error| error.to_string())?;
        let wake_roles = roles.clone();
        let wake_skills = local_product::physician_chat::PhysicianSkills::configured(
            configured(&get, "PHYSICIAN_SKILLS_DIR").as_deref(),
            hosted,
        );
        if let Err(err @ local_product::physician_chat::SkillsError::Package { .. }) = &wake_skills
        {
            return Err(err.to_string());
        }
        let wake_queue: Option<Arc<dyn physician_sample::queue::ChatQueue>> = if let Some(topic) =
            configured(&get, "PHYSICIAN_CHAT_TOPIC")
        {
            let emulator = configured(&get, "PUBSUB_EMULATOR_HOST");
            let connected =
                physician_sample::queue::PubSubQueue::connect(&topic, emulator.as_deref(), hosted)
                    .await?;
            Some(Arc::new(connected))
        } else {
            None
        };
        let model = ModelSource { transport, roles };
        let database = configured(&get, "PHYSICIAN_DATABASE_URL")
            .ok_or("PHYSICIAN_DATABASE_URL is required")?;
        let store = Store::connect(&database)
            .await
            .map_err(|_| "physician runtime store unavailable".to_string())?;
        let auth = PostgresAuthStore::connect_without_migrations(&database)
            .await
            .map_err(|_| "physician auth store unavailable".to_string())?;
        let bus = physician_bus::from_env(&get, hosted).await?;
        let quota: Arc<dyn harness::provider::QuotaGate> = Arc::new(store.clone());
        let wake = Arc::new(WakeService {
            auth: schedule_auth,
            mode: wake_mode,
            handler: Arc::new(StoreWakeHandler {
                store: store.clone(),
                auth: auth.clone(),
                roles: wake_roles,
                skills: wake_skills,
                queue: wake_queue,
            }),
            clinical_library,
        });
        Ok(Self {
            store,
            auth,
            bus,
            model,
            quota,
            verifier,
            wake,
            origin,
            simulation_practices,
        })
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
        verifier: google_cloud_auth::credentials::idtoken::verifier::Verifier,
        principal: hosting::ServiceAccountEmail,
    },
    /// No scheduler principal is configured on this revision: missing/invalid tokens
    /// still fail `401`, while valid tokens fail `403`.
    Disabled {
        verifier: Option<google_cloud_auth::credentials::idtoken::verifier::Verifier>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WakeReport {
    due_patients_checked: usize,
    reviews_started: usize,
}

#[derive(Debug)]
enum WakeError {
    Rehearsed,
    RuntimeStore(sea_orm::DbErr),
    AuthStore(sea_orm::DbErr),
    SkillsUnavailable(local_product::physician_chat::SkillsError),
    Dispatch(local_product::runtime::dispatch::DispatchError),
}

impl WakeError {
    fn kind(&self) -> &'static str {
        match self {
            Self::Rehearsed => "rehearsed_failure",
            Self::RuntimeStore(_) => "runtime_store_unavailable",
            Self::AuthStore(_) => "auth_store_unavailable",
            Self::SkillsUnavailable(_) => "skills_unavailable",
            Self::Dispatch(_) => "dispatch_failed",
        }
    }
}

impl std::fmt::Display for WakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rehearsed => f.write_str("rehearsed schedule wake failure"),
            Self::RuntimeStore(source) => write!(f, "runtime store check failed: {source}"),
            Self::AuthStore(source) => write!(f, "auth store check failed: {source}"),
            Self::SkillsUnavailable(source) => write!(f, "physician skills unavailable: {source}"),
            Self::Dispatch(source) => write!(f, "patient dispatch wake failed: {source}"),
        }
    }
}

impl std::error::Error for WakeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rehearsed => None,
            Self::RuntimeStore(source) | Self::AuthStore(source) => Some(source),
            Self::SkillsUnavailable(source) => Some(source),
            Self::Dispatch(source) => Some(source),
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
    }
}

const DISPATCH_RETRY_SECONDS: i64 = 60;

struct StoreWakeHandler {
    store: Store,
    auth: PostgresAuthStore,
    roles: local_product::ModelRoles,
    skills: Result<
        local_product::physician_chat::PhysicianSkills,
        local_product::physician_chat::SkillsError,
    >,
    queue: Option<Arc<dyn physician_sample::queue::ChatQueue>>,
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
        let skills = self
            .skills
            .as_ref()
            .map_err(|err| WakeError::SkillsUnavailable(clone_skills_error(err)))?;
        let checker =
            local_product::runtime::dispatch::StoredObservationChecker::new(self.store.clone());
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
            let deliveries = receiver.take_deliveries().await;
            if let Some(queue) = &self.queue {
                for delivery in deliveries {
                    if queue.publish(&delivery).await.is_ok() {
                        if self
                            .store
                            .dispatched(&delivery, DISPATCH_RETRY_SECONDS)
                            .await
                            .is_err()
                        {
                            tracing::error!(
                                event = "physician.queue.dispatch_failed",
                                error.type = "database",
                                "accepted patient review dispatch marker unavailable"
                            );
                        }
                    } else {
                        tracing::error!(
                            event = "physician.queue.publish_failed",
                            error.type = "publish",
                            "accepted patient review awaits queue recovery"
                        );
                    }
                }
            }
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
        let probe = Scope {
            practice: "schedule-wake".into(),
            physician: "schedule-wake".into(),
            workspace: "schedule-wake".into(),
            chat: "schedule-wake".into(),
        };
        self.store
            .read(&probe)
            .await
            .map_err(WakeError::RuntimeStore)?;
        self.auth
            .check_schema()
            .await
            .map_err(WakeError::AuthStore)?;
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
                clinical_library_configured = wake.clinical_library.is_some(),
                "accepted physician schedule wake"
            );
            (
                StatusCode::OK,
                Json(json!({
                    "status": "ok",
                    "due_patients_checked": report.due_patients_checked,
                    "reviews_started": report.reviews_started,
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

fn audience_verifier(
    audience: String,
) -> google_cloud_auth::credentials::idtoken::verifier::Builder {
    google_cloud_auth::credentials::idtoken::verifier::Builder::new([audience])
}

/// The push route's authentication gate runs before decoding or claiming work.
async fn authorize_push(
    verifier: Option<&google_cloud_auth::credentials::idtoken::verifier::Verifier>,
    header: Option<&str>,
) -> Result<(), StatusCode> {
    let Some(verifier) = verifier else {
        return Ok(());
    };
    let token = bearer_token(header).ok_or(StatusCode::UNAUTHORIZED)?;
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
    local_product::MODEL_SETTING_KEYS
        .into_iter()
        .filter_map(|key| configured(get, key).map(|value| (key.to_owned(), value)))
        .collect()
}

/// Credentials stay in the worker; both choices run the same durable job.
enum ModelTransport {
    Responses { endpoint: String, key: String },
    DeveloperPlan,
}

impl ModelTransport {
    fn adapter(&self) -> &'static str {
        match self {
            Self::Responses { .. } => "openai-responses",
            Self::DeveloperPlan => "chatgpt-plan",
        }
    }
}

/// Transport and model configuration shared by chat and reply revision.
struct ModelSource {
    transport: ModelTransport,
    roles: local_product::ModelRoles,
}

impl ModelSource {
    async fn provider(
        &self,
        quota: Arc<dyn harness::provider::QuotaGate>,
    ) -> Result<ModelProvider, harness::engine::Cause> {
        let provider = match &self.transport {
            ModelTransport::Responses { endpoint, key } => {
                ResponsesProvider::new(endpoint.clone(), key.clone(), quota.clone())?
            }
            ModelTransport::DeveloperPlan => {
                let token = local_product::chatgpt::access_token().await?;
                ResponsesProvider::chatgpt_plan(token, quota.clone())?
            }
        };
        Ok(ModelProvider {
            provider,
            developer_plan: matches!(self.transport, ModelTransport::DeveloperPlan),
            quota,
        })
    }

    fn authority(&self, store: Store) -> WorkerAuthority {
        WorkerAuthority {
            inner: RuntimeAuthority,
            store,
            synthetic_only: match self.transport {
                ModelTransport::DeveloperPlan => true,
                ModelTransport::Responses { .. } => false,
            },
        }
    }
}

/// The plan grant may refresh during a long durable run. Request execution and
/// quota handling remain with the shared Responses provider.
struct ModelProvider {
    provider: ResponsesProvider,
    developer_plan: bool,
    quota: Arc<dyn harness::provider::QuotaGate>,
}

impl Provider for ModelProvider {
    async fn ready(&mut self) -> Result<(), harness::engine::Cause> {
        self.provider.ready().await
    }

    async fn exchange(
        &mut self,
        request: &harness::engine::recipe::Request,
        cancel: &Cancellation,
        limits: &RunLimits,
        observer: &mut dyn harness::engine::recipe::Observer,
    ) -> Result<harness::engine::Turn, harness::engine::Cause> {
        tracing::Span::current().record("gen_ai.request.model", request.model.as_str());
        if self.developer_plan {
            let token = local_product::chatgpt::access_token().await?;
            self.provider = ResponsesProvider::chatgpt_plan(token, self.quota.clone())?;
        }
        self.provider
            .exchange(request, cancel, limits, observer)
            .await
    }
}

/// Developer-plan inference retains the synthetic-chart boundary of local
/// practice. The same auth owner still fences chat and revision publication.
struct WorkerAuthority {
    inner: RuntimeAuthority,
    store: Store,
    synthetic_only: bool,
}

#[local_product::runtime::async_trait]
impl local_product::runtime::Authority for WorkerAuthority {
    async fn check(
        &mut self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.check_source_boundary(scope).await?;
        self.inner.check(tx, scope, hash).await
    }

    async fn lock_publication(
        &mut self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.check_source_boundary(scope).await?;
        self.inner.lock_publication(tx, scope, hash).await
    }
}

impl WorkerAuthority {
    pub(crate) async fn check_source_boundary(&self, scope: &Scope) -> Result<(), String> {
        if !self.synthetic_only {
            return Ok(());
        }
        let outside = self
            .store
            .has_sources_outside(
                &scope.workspace,
                physician_sample::synthetic_chart::SOURCE_ID,
            )
            .await
            .map_err(|e| e.to_string())?;
        if outside {
            return Err("Developer ChatGPT plans can read only the synthetic practice".into());
        }
        Ok(())
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
    use harness::engine::recipe::Provider;
    use serde_json::json;
    use std::time::Duration;

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
                harness::engine::AttemptFailure::from_error(&error, &cancel),
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
        ) -> Result<(), harness::engine::Cause> {
            Err("this run has no partial result".into())
        }
        async fn window_seed(
            &mut self,
        ) -> Result<harness::engine::WindowSeed, harness::engine::Cause> {
            Err("this run has no working windows".into())
        }
        async fn assert_current(&mut self) -> Result<(), harness::engine::Cause> {
            Ok(())
        }
        async fn message_sources(
            &mut self,
        ) -> Result<Option<Vec<harness::engine::ProductMessage>>, harness::engine::Cause> {
            Ok(None)
        }
        async fn admit_binding(
            &mut self,
            _: &harness::engine::recipe::RequestBinding,
        ) -> Result<(), harness::engine::Cause> {
            Ok(())
        }
        async fn persist(
            &mut self,
            _: harness::engine::Event,
        ) -> Result<(), harness::engine::Cause> {
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
        ) -> Result<harness::engine::recipe::Finish, harness::engine::Cause> {
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
        let delivery = r#"{"kind":"chat_turn","scope":{"practice":"p","physician":"d","workspace":"w","chat":"c"},"input_id":"k"}"#;
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
        let practice_job_delivery = Delivery::practice_job(
            &local_product::runtime::jobs::PracticeScope {
                practice: "p".into(),
                workspace: "w".into(),
                job_id: "j".into(),
            },
            "job-key".into(),
        );
        let data = base64::engine::general_purpose::STANDARD
            .encode(serde_json::to_vec(&practice_job_delivery).unwrap());
        let body = json!({"message": {"data": data, "message_id": "practice-job-message"}});
        let push = decode_push(body.to_string().as_bytes()).unwrap();
        assert_eq!(push.delivery.kind, DeliveryKind::PracticeJob);
        assert_eq!(
            push.delivery.scope.physician,
            local_product::runtime::jobs::PRACTICE_OWNER
        );
        assert_eq!(push.delivery.input_id, "job-key");
        assert!(decode_push(b"{}").is_err());
        assert!(decode_push(br#"{"message":{"data":"not base64","messageId":"m"}}"#).is_err());
        let bad = base64::engine::general_purpose::STANDARD.encode("{}");
        let bad_body = json!({"message": {"data": bad, "messageId": "m"}});
        assert!(decode_push(bad_body.to_string().as_bytes()).is_err());
    }

    #[test]
    fn push_body_carries_trace_attributes_and_the_delivery_attempt() {
        let delivery = r#"{"kind":"chat_turn","scope":{"practice":"p","physician":"d","workspace":"w","chat":"c"},"input_id":"k"}"#;
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
    fn hosted_worker_refuses_local_provider_and_local_worker_requires_verified_push() {
        let get = |name: &str| (name == "PHYSICIAN_RESPONSES_STUB_PORT").then(|| "8080".into());
        let hosted = |name: &str| {
            if name == "K_SERVICE" {
                Some("physician-worker".into())
            } else {
                get(name)
            }
        };
        assert!(
            block(Worker::from_env(hosted))
                .err()
                .unwrap()
                .contains("public Responses endpoint")
        );
        assert!(
            block(Worker::from_env(get))
                .err()
                .unwrap()
                .contains("PUBSUB_EMULATOR_HOST")
        );
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
        let mut plan = model(ModelTransport::DeveloperPlan).authority(store.clone());
        let db = sea_orm::Database::connect(url.as_str()).await.unwrap();
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
        let responses = model(ModelTransport::Responses {
            endpoint: "http://127.0.0.1:1/responses".into(),
            key: "synthetic-key".into(),
        })
        .authority(store.clone());
        responses
            .check_source_boundary(&scope(&mixed))
            .await
            .unwrap();

        // A developer plan may read a workspace that holds only the synthetic practice.
        plan.check_source_boundary(&scope(&practice)).await.unwrap();

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
        let empty_pull = local_product::runtime::sources::SourcePull {
            source_id: "ehr".into(),
            source_label: "EHR".into(),
            patient: local_product::physician_sources::SourcePatient {
                source_patient_id: "patient-1".into(),
                display_name: "Synthetic Patient".into(),
                birth_date: None,
                sex_or_gender: None,
                medical_record_numbers: vec![],
            },
            pulled_at: 1,
        };
        store
            .record_chart(&withdrawn, &[], &empty_pull)
            .await
            .unwrap();
        let current = store.sources(&withdrawn).await.unwrap().sources;
        assert!(current.iter().all(|source| source.source_id == synthetic));
        assert_eq!(
            plan.check_source_boundary(&scope(&withdrawn)).await,
            refusal
        );
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
        use jsonwebtoken::{Algorithm, EncodingKey, Header};
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
        header.kid = Some("synthetic-pubsub-proof".into());
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
}
