//! Push worker for accepted physician chat turns.
//!
//! Postgres owns the turn. Pub/Sub only wakes this process. The push body
//! carries a scope and an input id, never clinical text, and logs stay that way.
//! A duplicate or settled delivery is acknowledged without calling the model.

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
use local_product::runtime::async_trait;
use local_product::runtime::delivery::{Delivery, DeliveryState};
use local_product::runtime::{PhysicianHost, Scope, Store};
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
    if !worker.authorized(header.as_deref()).await {
        return StatusCode::UNAUTHORIZED;
    }
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST;
    };
    let Ok((delivery, message_id)) = decode_push(&bytes) else {
        return StatusCode::BAD_REQUEST;
    };
    let claim = match worker.store.claim_delivery(&delivery, LEASE_SECONDS).await {
        Ok(Some(claim)) => claim,
        Ok(None) => return absent_claim(&worker, &delivery).await,
        Err(_) => {
            tracing::warn!("physician chat claim failed");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    let workspace_id = claim.scope.workspace.clone();
    let session_id = claim.scope.chat.clone();
    let input_text = message_text(&worker.store, &claim.scope, "user").await;
    let span = genai::generation_span(
        &RunContext {
            run_id: format!("physician-worker-{message_id}"),
            job: "physician_chat",
            recipe_version: genai::recipe_version(),
            origin: genai::Origin::Real,
            scenario_id: None,
            session_id: Some(session_id),
        },
        worker.system,
        &worker.model_name,
        worker.adapter,
    );
    genai::record_input(&span, &input_text);
    drop(input_text);
    let mut host = PhysicianHost {
        store: worker.store.clone(),
        claim,
        authority: RuntimeAuthority {
            store: worker.store.clone(),
        },
        instructions: local_product::codex::PHYSICIAN_CHAT_INSTRUCTIONS.to_owned(),
    };
    let cancel = Cancellation::default();
    // One provider per turn: concurrent pushes on this instance run side by
    // side instead of queueing behind one another's model call.
    let mut provider = match worker.model.provider() {
        Ok(provider) => provider,
        Err(_) => {
            tracing::error!("physician chat provider unavailable");
            if worker.store.retry_delivery(&host.claim).await.is_err() {
                tracing::warn!("physician chat delivery was not returned to pending");
            }
            // Counted like a model failure, so the fifth one ends the turn.
            announce(&worker, workspace_id).await;
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    let result = host
        .run(&mut provider, &worker.limits, &cancel)
        .instrument(span.clone())
        .await;
    match result {
        Ok(()) => {
            let output = message_text(&worker.store, &host.claim.scope, "assistant").await;
            genai::record_output(&span, &output);
            drop(output);
            // The turn is committed. Redelivery sees it settled and acks.
            announce(&worker, workspace_id).await;
            StatusCode::NO_CONTENT
        }
        Err(_) => {
            genai::record_error(&span, "physician chat failed");
            tracing::error!("physician chat failed");
            if worker.store.retry_delivery(&host.claim).await.is_err() {
                tracing::warn!("physician chat delivery was not returned to pending");
            }
            announce(&worker, workspace_id).await;
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
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
        .and_then(|conversation| {
            conversation
                .messages
                .into_iter()
                .rev()
                .find(|message| message.role == role)
                .map(|message| message.text)
        })
        .unwrap_or_default()
}

fn decode_push(bytes: &[u8]) -> Result<(Delivery, String), ()> {
    let envelope: Value = serde_json::from_slice(bytes).map_err(|_| ())?;
    let message = envelope.get("message").ok_or(())?;
    let data = message.get("data").and_then(Value::as_str).ok_or(())?;
    // Pub/Sub sends both messageId and message_id. deliveryAttempt is ignored:
    // a failed turn is never acked, including the fifth attempt.
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
    Ok((delivery, message_id))
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
        let execution = local_product::model_execution(
            configured(&get, "CODA_LOCAL_MODEL_EXECUTION").as_deref(),
        )?;
        if hosted && execution != local_product::ModelExecution::Api {
            return Err("hosted physician worker requires CODA_LOCAL_MODEL_EXECUTION=api".into());
        }
        let (model, system, model_name, adapter) = if let Some(text) = fixed {
            if failure_count > 0 {
                tracing::info!(
                    failure_count,
                    "physician worker fixed provider will fail before succeeding"
                );
            } else {
                tracing::info!("physician worker model execution: fixed text");
            }
            (
                ModelSource::Fixed {
                    text,
                    failures_remaining: Arc::new(std::sync::atomic::AtomicU32::new(failure_count)),
                    delay,
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
                    model: model_name.clone(),
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
        })
    }

    async fn authorized(&self, header: Option<&str>) -> bool {
        let Some(verifier) = &self.verifier else {
            return true;
        };
        let Some(token) = header
            .and_then(|value| value.strip_prefix("Bearer "))
            .map(str::trim)
            .filter(|token| !token.is_empty())
        else {
            return false;
        };
        verifier.verify(token).await.is_ok()
    }
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
        return Ok(Some(identity_verifier(audience, email)));
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
        (Some(audience), Some(email)) => Ok(Some(identity_verifier(audience, email))),
        _ => Err(
            "physician worker requires PHYSICIAN_WORKER_AUDIENCE and PHYSICIAN_PUSH_SERVICE_ACCOUNT, or PUBSUB_EMULATOR_HOST"
                .into(),
        ),
    }
}

fn identity_verifier(
    audience: String,
    email: String,
) -> google_cloud_auth::credentials::idtoken::verifier::Verifier {
    google_cloud_auth::credentials::idtoken::verifier::Builder::new([audience])
        .with_email(email)
        .build()
}

/// Bounds for one hosted chat attempt. The context bound matches the site's
/// in-process chat (`harness::execution::Limits::default`), so a conversation
/// that runs on the site also runs here. The reply bounds match Duet's chat.
/// One model call and no tools: hosted chat has no mutation tools yet.
/// `elapsed` stays inside the 540 s lease and the 600 s push deadline.
fn limits() -> Limits {
    Limits {
        model_calls: 1,
        tool_calls: 0,
        calls_per_turn: 1,
        request_bytes: 1_048_576,
        response_bytes: 131_072,
        tool_result_bytes: 4096,
        total_tool_result_bytes: 4096,
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
        model: String,
    },
    /// Non-hosted proof only. The failure count is shared by every turn on
    /// this process, so "fail N times, then answer" holds across redeliveries.
    Fixed {
        text: String,
        failures_remaining: Arc<std::sync::atomic::AtomicU32>,
        delay: Duration,
    },
}

impl ModelSource {
    fn provider(&self) -> Result<Model, String> {
        match self {
            Self::Api { key, model } => Ok(Model::Api(harness::responses::Responses::new(
                key.clone(),
                model.clone(),
            )?)),
            Self::Fixed {
                text,
                failures_remaining,
                delay,
            } => Ok(Model::Fixed {
                text: text.clone(),
                failures_remaining: failures_remaining.clone(),
                delay: *delay,
            }),
        }
    }
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

enum Model {
    Api(harness::responses::Responses),
    Fixed {
        text: String,
        failures_remaining: Arc<std::sync::atomic::AtomicU32>,
        delay: Duration,
    },
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
                failures_remaining, ..
            } if take_one(failures_remaining) => Err("fixed physician reply failed".into()),
            Self::Fixed { text, .. } => Ok(Turn {
                items: vec![Item::Message {
                    role: "assistant".into(),
                    text: text.clone(),
                }],
                receipt: json!({ "provider": "fixed" }),
            }),
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
    fn fixed_failures_count_down_to_zero_and_stop() {
        let count = std::sync::atomic::AtomicU32::new(2);
        assert!(take_one(&count));
        assert!(take_one(&count));
        assert!(!take_one(&count));
        assert!(!take_one(&count));
        assert_eq!(count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn push_body_decodes_standard_base64_delivery_without_requiring_one_id_field() {
        let delivery = r#"{"scope":{"practice":"p","physician":"d","workspace":"w","chat":"c"},"input_id":"k"}"#;
        let data = base64::engine::general_purpose::STANDARD.encode(delivery);
        let body = json!({
            "message": {"data": data, "messageId": "m1", "message_id": "m1"},
            "deliveryAttempt": 2
        });
        let (decoded, id) = decode_push(body.to_string().as_bytes()).unwrap();
        assert_eq!(id, "m1");
        assert_eq!(decoded.input_id, "k");
        assert_eq!(decoded.scope.workspace, "w");
        assert!(decode_push(b"{}").is_err());
        assert!(decode_push(br#"{"message":{"data":"not base64","messageId":"m"}}"#).is_err());
        let bad = base64::engine::general_purpose::STANDARD.encode("{}");
        let bad_body = json!({"message": {"data": bad, "messageId": "m"}});
        assert!(decode_push(bad_body.to_string().as_bytes()).is_err());
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
                assert!(error.contains("PHYSICIAN_WORKER_FIXED_FAILURES"), "{error}");
            }
            Ok(_) => panic!("hosted fixed failures must refuse to start"),
        }
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
