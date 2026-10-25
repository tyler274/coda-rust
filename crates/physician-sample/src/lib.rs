//! Physician projections for the Roger synthetic practice.
//!
//! The FHIR corpus stays in coda-agents. These files are the physician JSON the
//! React site already renders. Unset `CODA_LOCAL_MODEL_EXECUTION` is `live`:
//! `message.submit` runs through the Codex CLI (`codex login`) and does not
//! read an API key. `record` and `replay` use a cassette on that same path.
//! `api` answers `message.submit` through OpenAI Responses. Projections stay
//! on the recording. `PHYSICIAN_UPSTREAM` still forwards physician routes to
//! the Node server when api mode is unset. The same server serves the Dioxus
//! page, which mounts into `#main`.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]

mod auth;
mod care;
mod gmail;
pub mod hosting;
mod live;
#[cfg(test)]
mod live_tests;
mod publication;
#[cfg(test)]
mod publication_tests;
mod reply;
mod simulation;

/// The staging practice id and the auth/front-door wiring, for tests.
pub use auth::{STAGING_PRACTICE_ID, authed_router, authed_router_on};

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::extract::Query;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use futures_util::TryStreamExt;
use serde::Deserialize;
use serde_json::{Value, json};

const INDEX: &str = include_str!("../../physician-app/index.html");
const LOGO: &[u8] = include_bytes!("../../physician-app/assets/coda-health-logo-transparent.png");
const FAVICON: &[u8] = include_bytes!("../../physician-app/assets/favicon.svg");

const TODAY: &str = include_str!("../fixtures/today.json");
const PRACTICE_DAY: &str = include_str!("../fixtures/practice-day.json");
const UPDATES: &str = include_str!("../fixtures/updates.json");
const RECENT: &str = include_str!("../fixtures/recent-conversations.json");
const WORKSPACES: &str = include_str!("../fixtures/patient-workspaces.json");
const ROSTER: &str = include_str!("../fixtures/roster.json");

/// The recorded practice's patient workspace ids, also the roster the
/// staging practice invites from; integration tests read them too.
pub const WORKSPACE_IDS: [&str; 5] = [
    "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee",
    "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d",
    "pw-f5404d89-acde-4bb5-8c42-2fa191766811",
    "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971",
    "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e",
];

/// Builds the HTTP router. `PHYSICIAN_UPSTREAM` forwards physician routes;
/// `CODA_LOCAL_SIMULATION` activates the local simulation engine.
pub fn router() -> Router {
    if let Some(scenario) = simulation_scenario_from_env() {
        return simulation_router_with_scenario(&scenario);
    }
    router_with(upstream_from_env())
}

/// Builds the interactive local simulation router initialized with `connected-walk`.
pub fn simulation_router() -> Router {
    simulation_router_with_scenario(local_product::DEFAULT_SIMULATION_SCENARIO)
}

/// Builds the interactive local simulation router initialized with `scenario`.
pub fn simulation_router_with_scenario(scenario: &str) -> Router {
    let state = simulation::SimulationState::new(scenario);
    Router::new()
        .route("/health", get(|| async { Json(json!({"status": "ok"})) }))
        .route("/today", get(simulation::today))
        .route("/practice-day", get(|| async { json_body(PRACTICE_DAY) }))
        .route("/updates", get(|| async { json_body(UPDATES) }))
        .route("/recent-conversations", get(|| async { json_body(RECENT) }))
        .route(
            "/patient-workspaces",
            get(|| async { json_body(WORKSPACES) }),
        )
        .route("/patient-workspaces/{id}", get(simulation::workspace))
        .route(
            "/patient-workspaces/{id}/chats/{chat}/snapshot",
            get(simulation::snapshot),
        )
        .route("/patient-roster", get(roster))
        .route("/commands", post(simulation::command))
        .route(
            "/local-proof/simulations",
            get(simulation::simulations_index),
        )
        .route(
            "/local-proof/simulations/run",
            post(simulation::run_scenario_endpoint),
        )
        .route("/dictation", post(|| async { dictation_unavailable() }))
        .route(
            "/physician-state/drafts/{workspace}/{chat}",
            get(empty_draft).put(draft_not_saved),
        )
        .route("/", get(site_index))
        .route("/index.html", get(site_index))
        .route("/assets/coda-health-logo-transparent.png", get(logo))
        .route("/favicon.svg", get(favicon))
        .fallback(site_file)
        .with_state(state)
        .layer(middleware::from_fn(cors))
}

pub(crate) fn simulation_scenario_from_env() -> Option<String> {
    let raw = env::var("CODA_LOCAL_SIMULATION").ok()?;
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "0" || trimmed.eq_ignore_ascii_case("false") {
        return None;
    }
    if trimmed == "1" || trimmed.eq_ignore_ascii_case("true") {
        Some(local_product::DEFAULT_SIMULATION_SCENARIO.to_owned())
    } else {
        Some(trimmed.to_owned())
    }
}

fn router_with(upstream: Option<String>) -> Router {
    let routes = match upstream {
        Some(base) => proxied(base),
        None => recorded(),
    };
    routes.layer(middleware::from_fn(cors))
}

/// The recorded practice the sample server serves, for integration tests.
pub fn recorded() -> Router {
    site()
        .route("/commands", post(reject_command))
        .route(
            "/patient-workspaces/{id}/chats/{chat}/snapshot",
            get(empty_snapshot),
        )
        .route("/patient-workspaces/events", get(recorded_events))
}

fn live_router(model: live::ChatModel) -> Router {
    live_router_on(live::Live::new(model))
}

fn live_router_on(live: std::sync::Arc<live::Live>) -> Router {
    site()
        .route("/commands", post(live::submit))
        .route(
            "/physician-state/reply-drafts/{workspace}/{reply}",
            get(reply::read).put(reply::save),
        )
        .route(
            "/patient-workspaces/{id}/chats/{chat}/snapshot",
            get(live::snapshot),
        )
        .route("/patient-workspaces/events", get(live::events))
        .with_state(live)
}

fn site<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let care = care::ConnectedCare::new();
    Router::new()
        .route("/health", get(|| async { Json(json!({"status": "ok"})) }))
        .route("/today", get(today))
        .route("/practice-day", get(|| async { json_body(PRACTICE_DAY) }))
        .route("/updates", get(|| async { json_body(UPDATES) }))
        .route("/recent-conversations", get(|| async { json_body(RECENT) }))
        .route(
            "/patient-workspaces",
            get(|| async { json_body(WORKSPACES) }),
        )
        .route("/patient-workspaces/{id}", get(workspace))
        .route(
            "/patient-workspaces/{id}/invite",
            post(care::invite_patient),
        )
        .route(
            "/patient-invitations/accept",
            post(care::accept_patient_invitation),
        )
        .route("/patient-workspaces/{id}/link", get(care::get_patient_link))
        .route(
            "/patient-workspaces/{id}/consent",
            axum::routing::put(care::put_patient_consent),
        )
        .route(
            "/patient-workspaces/{id}/stop-sharing",
            post(care::post_stop_sharing),
        )
        .route(
            "/patient-workspaces/{id}/cadence",
            axum::routing::put(care::put_patient_cadence),
        )
        .route(
            "/patient-workspaces/{id}/duet-sync",
            post(care::post_duet_sync),
        )
        .route("/patient-workspaces/{id}/plan", get(care::get_plan))
        .route(
            "/patient-workspaces/{id}/plan/draft",
            axum::routing::put(care::put_plan_draft),
        )
        .route(
            "/patient-workspaces/{id}/plan/approve",
            post(care::post_approve_plan),
        )
        .route("/patient-roster", get(roster))
        .route("/dictation", post(|| async { dictation_unavailable() }))
        .route(
            "/physician-state/drafts/{workspace}/{chat}",
            get(empty_draft).put(draft_not_saved),
        )
        .route("/", get(site_index))
        .route("/index.html", get(site_index))
        .route("/assets/coda-health-logo-transparent.png", get(logo))
        .route("/favicon.svg", get(favicon))
        .fallback(site_file)
        .layer(axum::Extension(care))
}

fn proxied(upstream: String) -> Router {
    Router::new()
        .route("/", get(site_index))
        .route("/index.html", get(site_index))
        .route("/assets/coda-health-logo-transparent.png", get(logo))
        .route("/favicon.svg", get(favicon))
        .fallback(move |request: Request<Body>| {
            let upstream = upstream.clone();
            async move {
                let path = request.uri().path().to_owned();
                if physician_route(&path) {
                    forward(&upstream, request).await
                } else {
                    site_file(request).await
                }
            }
        })
}

fn upstream_from_env() -> Option<String> {
    let value = env::var("PHYSICIAN_UPSTREAM").ok()?;
    let value = value.trim().trim_end_matches('/').to_owned();
    if value.is_empty() { None } else { Some(value) }
}

pub(crate) fn physician_route(path: &str) -> bool {
    const PREFIXES: [&str; 14] = [
        "/health",
        "/today",
        "/practice-day",
        "/updates",
        "/recent-conversations",
        "/patient-workspaces",
        "/patient-roster",
        "/commands",
        "/dictation",
        "/physician-state",
        "/connections",
        "/director",
        "/sessions",
        "/local-proof",
    ];
    PREFIXES
        .iter()
        .any(|prefix| path == *prefix || path.starts_with(&format!("{prefix}/")))
}

fn http_client() -> &'static reqwest::Client {
    static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(|| {
        std::thread::Builder::new()
            .stack_size(8 * 1024 * 1024)
            .spawn(reqwest::Client::new)
            .expect("http client thread")
            .join()
            .expect("http client init")
    })
}

async fn forward(upstream: &str, request: Request<Body>) -> Response {
    let path = request
        .uri()
        .path_and_query()
        .map(|query| query.as_str())
        .unwrap_or("/");
    let url = format!("{upstream}{path}");
    let method = request.method().clone();
    let content_type = request
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .cloned();
    let accept = request.headers().get(axum::http::header::ACCEPT).cloned();
    let last_event = request.headers().get("last-event-id").cloned();
    let body = axum::body::to_bytes(request.into_body(), 1_000_000)
        .await
        .unwrap_or_default();
    let method =
        reqwest::Method::from_bytes(method.as_str().as_bytes()).unwrap_or(reqwest::Method::GET);
    let mut outgoing = http_client().request(method, &url).body(body);
    if let Some(value) = content_type {
        outgoing = outgoing.header(axum::http::header::CONTENT_TYPE, value);
    }
    if let Some(value) = accept {
        outgoing = outgoing.header(axum::http::header::ACCEPT, value);
    }
    if let Some(value) = last_event {
        outgoing = outgoing.header("last-event-id", value);
    }
    let Ok(incoming) = outgoing.send().await else {
        return StatusCode::BAD_GATEWAY.into_response();
    };
    let status =
        StatusCode::from_u16(incoming.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let content_type = incoming
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .cloned();
    let cache_control = incoming
        .headers()
        .get(axum::http::header::CACHE_CONTROL)
        .cloned();
    let bytes = incoming.bytes_stream().map_err(std::io::Error::other);
    let mut response = Response::new(Body::from_stream(bytes));
    *response.status_mut() = status;
    if let Some(value) = header_value(content_type) {
        response
            .headers_mut()
            .insert(axum::http::header::CONTENT_TYPE, value);
    }
    if let Some(value) = header_value(cache_control) {
        response
            .headers_mut()
            .insert(axum::http::header::CACHE_CONTROL, value);
    }
    response
}

fn header_value(value: Option<reqwest::header::HeaderValue>) -> Option<HeaderValue> {
    let value = value?;
    HeaderValue::from_bytes(value.as_bytes()).ok()
}

/// Directory of `wasm-bindgen --target web` output. `PHYSICIAN_SITE_DIR` wins,
/// then `target/physician-web` beside the workspace.
pub fn site_dir() -> Option<PathBuf> {
    if let Some(dir) = env::var_os("PHYSICIAN_SITE_DIR") {
        let path = PathBuf::from(dir);
        if path.is_dir() {
            return Some(path);
        }
    }
    let fallback = PathBuf::from("target/physician-web");
    fallback.is_dir().then_some(fallback)
}

async fn logo() -> Response {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "image/png")],
        LOGO,
    )
        .into_response()
}

async fn favicon() -> Response {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "image/svg+xml")],
        FAVICON,
    )
        .into_response()
}

async fn site_index() -> Response {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        INDEX,
    )
        .into_response()
}

async fn empty_snapshot(
    axum::extract::Path((id, _chat)): axum::extract::Path<(String, String)>,
) -> Response {
    if workspace_body(&id).is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }
    Json(json!({
        "snapshot": { "conversation": [] }
    }))
    .into_response()
}

async fn recorded_events() -> Response {
    let stream = futures_util::stream::pending::<Result<String, std::convert::Infallible>>();
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/event-stream"),
        )],
        Body::from_stream(stream),
    )
        .into_response()
}

async fn site_file(request: Request<Body>) -> Response {
    let Some(dir) = site_dir() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let path = request.uri().path().trim_start_matches('/');
    if path.is_empty() || path.contains("..") || path.contains('\\') {
        return StatusCode::NOT_FOUND.into_response();
    }
    let file = dir.join(path);
    if !file.starts_with(&dir) {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Ok(bytes) = fs::read(&file) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Some(mime) = site_mime(&file) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, mime)],
        bytes,
    )
        .into_response()
}

fn site_mime(file: &Path) -> Option<&'static str> {
    match file.extension().and_then(|ext| ext.to_str()) {
        Some("js") => Some("text/javascript"),
        Some("wasm") => Some("application/wasm"),
        Some("png") => Some("image/png"),
        Some("css") => Some("text/css"),
        _ => None,
    }
}

/// Serves the router on `listener`.
pub async fn serve(listener: tokio::net::TcpListener) {
    let base = match router_from_env() {
        Ok(app) => app,
        Err(error) => {
            tracing::error!(%error, "physician site cannot start");
            std::process::exit(1);
        }
    };
    let app = traced(route_readiness(auth::authed_router(base).await).await);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("sample server");
}

/// Resolves on SIGTERM (Cloud Run's stop signal) or Ctrl-C, so in-flight
/// requests drain instead of the process being killed after the grace period.
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

/// Adds `/readyz`, which answers from the same stores the process serves with.
/// Without `PHYSICIAN_DATABASE_URL` there is nothing to check, so it stays 503.
async fn route_readiness(app: Router) -> Router {
    let url = std::env::var("PHYSICIAN_DATABASE_URL").ok();
    let stores = match url.as_deref() {
        Some(url) => {
            let auth = physician_auth::PostgresAuthStore::connect_without_migrations(url).await;
            let front = front_door::PostgresFrontDoorStore::connect_without_migrations(url).await;
            match (auth, front) {
                (Ok(auth), Ok(front)) => Some((auth, front)),
                (Err(error), _) | (_, Err(error)) => {
                    eprintln!("readiness stores unavailable: {error}");
                    None
                }
            }
        }
        None => None,
    };
    let (auth, front) = match stores {
        Some((auth, front)) => (Some(auth), Some(front)),
        None => (None, None),
    };
    app.route(
        "/readyz",
        get(move || {
            let auth = auth.clone();
            let front = front.clone();
            async move { hosting::ready(auth.as_ref(), front.as_ref()).await }
        }),
    )
}

/// Wraps the whole router in the `http.server` tracing layer.
pub fn traced(router: Router) -> Router {
    router.layer(telemetry::http_layer())
}

/// The chat router on one Codex transport (record or a scripted executor),
/// traced like `serve`. For integration tests.
#[doc(hidden)]
pub fn codex_chat_router(transport: local_product::codex::CodexTransport) -> Router {
    let roles = local_product::model_roles("codex-exec", &[]).expect("default roles");
    traced(
        live_router(live::ChatModel::Codex {
            binding: roles.chat,
            transport: std::sync::Arc::new(std::sync::Mutex::new(transport)),
        })
        .layer(middleware::from_fn(cors)),
    )
}

/// Serves the recorded practice on `listener` with authentication required
/// against the given `store`.
pub async fn serve_with_store(
    listener: tokio::net::TcpListener,
    store: std::sync::Arc<dyn physician_auth::AuthStore>,
) {
    let base = router();
    let app = traced(auth::authed_router_with_store(base, store).await);
    axum::serve(listener, app).await.expect("sample server");
}

fn router_from_env() -> Result<Router, String> {
    let execution =
        local_product::model_execution(env::var("CODA_LOCAL_MODEL_EXECUTION").ok().as_deref())?;
    if execution != local_product::ModelExecution::Api
        && (simulation_scenario_from_env().is_some() || upstream_from_env().is_some())
    {
        return Ok(router());
    }
    let allow = env::var("CODA_ALLOW_MODEL_CALLS").ok();
    let key = env::var("OPENAI_API_KEY").ok();
    let authorization = env::var("CODA_CASSETTE_AUTHORIZATION").ok();
    let output = env::var("CODA_CASSETTE_OUTPUT").ok();
    let cassette = env::var("CODA_CASSETTE_PATH").ok();
    let owned = role_environment();
    let environment: Vec<(&str, &str)> = owned
        .iter()
        .map(|(key, value)| (*key, value.as_str()))
        .collect();
    model_router(
        execution,
        allow.as_deref(),
        key.as_deref(),
        local_product::codex::CodexSettings {
            cassette_authorization: authorization.as_deref(),
            cassette_output: output.as_deref(),
            cassette_path: cassette.as_deref(),
        },
        &environment,
    )
}

fn model_router(
    execution: local_product::ModelExecution,
    allow: Option<&str>,
    key: Option<&str>,
    settings: local_product::codex::CodexSettings<'_>,
    environment: &[(&str, &str)],
) -> Result<Router, String> {
    if execution == local_product::ModelExecution::Api {
        let key = local_product::open_ai_api_key(allow, key)?;
        let roles = local_product::model_roles("openai-responses", environment)?;
        tracing::info!(
            "Model execution: api via OpenAI Responses (chat {} {}; background {} {}; worker {} {}; linking {} {})",
            roles.chat.model,
            roles.chat.effort,
            roles.background.model,
            roles.background.effort,
            roles.worker.model,
            roles.worker.effort,
            roles.linking.model,
            roles.linking.effort,
        );
        return Ok(live_router(live::ChatModel::OpenAi {
            key,
            binding: roles.chat,
        })
        .layer(middleware::from_fn(cors)));
    }
    if execution == local_product::ModelExecution::Replay
        && let Some(path) = settings.cassette_path
    {
        let bytes = fs::read(path).map_err(|_| "could not read replay cassette")?;
        let value: Value = serde_json::from_slice(&bytes).map_err(|_| "invalid replay cassette")?;
        if value.get("schema_version").and_then(Value::as_u64) == Some(2) {
            let cassette =
                serde_json::from_value(value).map_err(|_| "invalid structured cassette")?;
            let replay = harness::transport::Replay::new(cassette)?;
            let roles = local_product::model_roles("structured-replay", environment)?;
            return Ok(live_router(live::ChatModel::StructuredReplay {
                binding: roles.chat,
                replay: tokio::sync::Mutex::new(replay),
            }));
        }
    }
    let transport = local_product::codex::open_codex(execution, settings)?;
    let roles = local_product::model_roles("codex-exec", environment)?;
    match execution {
        local_product::ModelExecution::Live => tracing::info!(
            "Model execution: live via Codex CLI (chat {} {}). Sign in with `codex login`. No API key is read.",
            roles.chat.model,
            roles.chat.effort
        ),
        local_product::ModelExecution::Record => tracing::info!(
            "Model execution: record via Codex CLI (chat {} {}). Sign in with `codex login`. Cassette: {}.",
            roles.chat.model,
            roles.chat.effort,
            settings.cassette_output.unwrap_or("")
        ),
        local_product::ModelExecution::Replay => tracing::info!(
            "Model execution: replay via Codex cassette (chat {} {}). Cassette: {}. No CLI call and no API key.",
            roles.chat.model,
            roles.chat.effort,
            settings.cassette_path.unwrap_or("")
        ),
        local_product::ModelExecution::Api => unreachable!("api is handled above"),
    }
    Ok(live_router(live::ChatModel::Codex {
        binding: roles.chat,
        transport: std::sync::Arc::new(std::sync::Mutex::new(transport)),
    })
    .layer(middleware::from_fn(cors)))
}

fn role_environment() -> Vec<(&'static str, String)> {
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
        .filter_map(|key| env::var(key).ok().map(|value| (key, value)))
        .collect()
}

async fn today(Query(query): Query<TodayQuery>) -> Response {
    if let Some(id) = query.patient_workspace_id {
        return match patient_today(&id) {
            Some(body) => json_body(body),
            None => StatusCode::NOT_FOUND.into_response(),
        };
    }
    json_body(TODAY)
}

async fn workspace(
    axum::extract::Path(id): axum::extract::Path<String>,
    request: Request<Body>,
) -> Response {
    let Some(body) = workspace_body(&id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let enriched = care::enrich_workspace(body, &id, request.extensions()).await;
    Json(enriched).into_response()
}

async fn roster(Query(query): Query<RosterQuery>, request: Request<Body>) -> Response {
    let mut patients = search_roster(&query.query.unwrap_or_default());
    if let Some(care) = request.extensions().get::<care::ConnectedCare>() {
        let store = care.auth_store(request.extensions());
        let identity = care::ConnectedCare::identity(request.extensions());
        for patient in &mut patients {
            let Some(ws_id) = patient["patient_workspace_id"].as_str().map(str::to_owned) else {
                continue;
            };
            let link = care.link_wire(&*store, &identity.practice_id, &ws_id).await;
            if let Some(obj) = patient.as_object_mut() {
                obj.insert("duet_linked".to_owned(), json!(link.linked));
                obj.insert(
                    "sharing_consented".to_owned(),
                    json!(link.sharing_consented),
                );
                obj.insert("cadence".to_owned(), json!(link.cadence));
            }
        }
    }
    Json(json!({ "patients": patients, "next_after": null })).into_response()
}

async fn reject_command(body: Option<Json<Value>>) -> Response {
    let command_type = body
        .as_ref()
        .and_then(|json| json.0.get("type"))
        .and_then(Value::as_str)
        .unwrap_or("");
    (
        StatusCode::OK,
        Json(json!({
            "outcome": "rejected",
            "message": rejection_message(command_type),
        })),
    )
        .into_response()
}

async fn empty_draft() -> Response {
    Json(json!({ "revision": 0, "text": "" })).into_response()
}

async fn draft_not_saved() -> Response {
    StatusCode::CONFLICT.into_response()
}

fn dictation_unavailable() -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        "Dictation is unavailable. Check microphone support and server configuration.",
    )
        .into_response()
}

fn json_body(body: &'static str) -> Response {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        body,
    )
        .into_response()
}

#[derive(Deserialize)]
struct TodayQuery {
    patient_workspace_id: Option<String>,
}

#[derive(Deserialize)]
struct RosterQuery {
    query: Option<String>,
}

/// Filters the recorded roster by the query.
pub fn search_roster(query: &str) -> Vec<Value> {
    let words: Vec<String> = query
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .filter(|word| !word.is_empty())
        .collect();
    if words.is_empty() {
        return Vec::new();
    }
    let roster: Value = serde_json::from_str(ROSTER).expect("roster");
    roster["patients"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|patient| {
            let name = patient["patient_display_name"].as_str().unwrap_or("");
            let birth = patient["birth_date"].as_str().unwrap_or("");
            let haystack = format!("{name} {birth}").to_lowercase();
            words.iter().all(|word| haystack.contains(word))
        })
        .take(50)
        .collect()
}

/// Returns why this server rejects the command.
pub fn rejection_message(command_type: &str) -> &'static str {
    match command_type {
        "message.submit" => "Coda couldn’t accept that message.",
        "message.withdraw" => "Coda couldn’t remove that queued message.",
        "work.resume" => "Coda couldn’t resume that work.",
        "work.done" => "Coda couldn’t mark that done.",
        "chat.open" => "Coda couldn’t open that chat.",
        "discussion.open" => "Coda couldn’t open that discussion.",
        "run.cancel" => "Coda couldn’t stop that work.",
        "home.clear" => "Coda couldn’t clear that item.",
        "reply.close" | "reply.use_update" => "Coda couldn’t change that reply.",
        "reply.reopen" => "Coda couldn’t reopen that reply.",
        "reply.request" => "Coda couldn’t ask for that reply.",
        "reply.revise" => "Coda couldn’t revise that reply.",
        "follow_up.review" => "Coda couldn’t review that follow-up now.",
        "follow_up.end" => "Coda couldn’t end that follow-up.",
        "conversation.archive" => "Coda couldn’t archive that conversation.",
        "conversation.restore" => "Coda couldn’t restore that conversation.",
        "cadence.set" | "cadence.override" | "review_cadence.set" => {
            "Physicians cannot override patient review cadence."
        }
        _ => "Coda couldn’t confirm that request. Reconnect to check its status.",
    }
}

pub(crate) fn workspace_body(id: &str) -> Option<&'static str> {
    Some(match id {
        "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee" => {
            include_str!("../fixtures/workspaces/pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee.json")
        }
        "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d" => {
            include_str!("../fixtures/workspaces/pw-82c44e9b-58c0-4af1-bd21-771bf77e101d.json")
        }
        "pw-f5404d89-acde-4bb5-8c42-2fa191766811" => {
            include_str!("../fixtures/workspaces/pw-f5404d89-acde-4bb5-8c42-2fa191766811.json")
        }
        "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971" => {
            include_str!("../fixtures/workspaces/pw-d11a35f7-0d8d-466f-a48e-5812df0a4971.json")
        }
        "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e" => {
            include_str!("../fixtures/workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e.json")
        }
        _ => return None,
    })
}

fn patient_today(id: &str) -> Option<&'static str> {
    if !WORKSPACE_IDS.contains(&id) {
        return None;
    }
    Some(match id {
        "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee" => {
            include_str!(
                "../fixtures/today-by-patient/pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee.json"
            )
        }
        "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d" => {
            include_str!(
                "../fixtures/today-by-patient/pw-82c44e9b-58c0-4af1-bd21-771bf77e101d.json"
            )
        }
        "pw-f5404d89-acde-4bb5-8c42-2fa191766811" => {
            include_str!(
                "../fixtures/today-by-patient/pw-f5404d89-acde-4bb5-8c42-2fa191766811.json"
            )
        }
        "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971" => {
            include_str!(
                "../fixtures/today-by-patient/pw-d11a35f7-0d8d-466f-a48e-5812df0a4971.json"
            )
        }
        "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e" => {
            include_str!(
                "../fixtures/today-by-patient/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e.json"
            )
        }
        _ => return None,
    })
}

async fn cors(request: Request<Body>, next: Next) -> Response {
    if request.method() == Method::OPTIONS {
        return cors_headers(StatusCode::NO_CONTENT.into_response());
    }
    cors_headers(next.run(request).await)
}

fn cors_headers(mut response: Response) -> Response {
    let headers = response.headers_mut();
    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("content-type, authorization"),
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    async fn get(path: &str) -> (StatusCode, String) {
        let response = router()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
            .await
            .unwrap();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn serves_the_recorded_practice() {
        let (status, today) = get("/today").await;
        assert_eq!(status, StatusCode::OK);
        assert!(today.contains("not_yet_read"));
        let (status, day) = get("/practice-day").await;
        assert_eq!(status, StatusCode::OK);
        assert!(day.contains("Mary Louis"));
        let (status, mary) =
            get("/patient-workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e").await;
        assert_eq!(status, StatusCode::OK);
        assert!(mary.contains("authored_text"));
        let (status, found) = get("/patient-roster?query=Mary").await;
        assert_eq!(status, StatusCode::OK);
        assert!(found.contains("1972-03-18"));
        let (status, updates) = get("/updates").await;
        assert_eq!(status, StatusCode::OK);
        assert!(updates.contains("\"rows\":[]"));
        let (status, page) = get("/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(page.contains(r#"<main id="main"></main>"#));
        let (status, snapshot) =
            get("/patient-workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e/chats/none/snapshot")
                .await;
        assert_eq!(status, StatusCode::OK);
        assert!(snapshot.contains("\"conversation\":[]"));
    }

    #[tokio::test]
    async fn an_unset_upstream_stays_recorded_and_a_set_one_forwards() {
        let (status, today) = request(router_with(None), "GET", "/today", Body::empty()).await;
        assert_eq!(status, StatusCode::OK);
        assert!(today.contains("not_yet_read"));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let app =
                Router::new().route("/health", axum::routing::get(|| async { "live-health" }));
            axum::serve(listener, app).await.unwrap();
        });
        let (status, health) = request(
            router_with(Some(format!("http://{address}"))),
            "GET",
            "/health",
            Body::empty(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(health, "live-health");
    }

    // Exercise real HTTP parsing in parallel, including vector-sized URIs.
    // An inherited ElyLD -B prefix once misaligned httparse's AVX constants;
    // an in-process oneshot-only proof would not catch that native link defect.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn proxy_preserves_long_queries_under_concurrent_http_parsing() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let app = Router::new().route(
                "/health",
                axum::routing::get(|uri: axum::http::Uri| async move { uri.to_string() }),
            );
            axum::serve(listener, app).await.unwrap();
        });
        let app = router_with(Some(format!("http://{address}")));
        let mut requests = tokio::task::JoinSet::new();
        for length in [0, 15, 16, 31, 32, 33, 63, 64, 65, 127, 128, 255] {
            let app = app.clone();
            requests.spawn(async move {
                let path = format!("/health?probe={}", "a".repeat(length));
                let (status, body) = request(app, "GET", &path, Body::empty()).await;
                assert_eq!(status, StatusCode::OK);
                assert_eq!(body, path);
            });
        }
        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            while let Some(result) = requests.join_next().await {
                result.unwrap();
            }
        })
        .await
        .expect("concurrent proxy requests finish");
        server.abort();
    }

    async fn request(router: Router, method: &str, path: &str, body: Body) -> (StatusCode, String) {
        let response = router
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(path)
                    .body(body)
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
            .await
            .unwrap();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn commands_are_rejected_without_a_model() {
        let response = router()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/commands")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"type":"work.resume"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4_000)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["outcome"], "rejected");
        assert_eq!(body["message"], "Coda couldn’t resume that work.");
    }

    #[tokio::test]
    async fn a_chat_message_is_answered_without_calling_openai() {
        let app = live_router(live::ChatModel::Fixed("Noted.".to_owned()))
            .layer(middleware::from_fn(cors));
        let submit = r#"{"type":"message.submit","idempotency_key":"k1","payload":{"patient_workspace_id":"pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e","chat_id":"chat-1","text":"Hello","physician_time_zone":"America/New_York"}}"#;
        let (status, accepted) =
            request(app.clone(), "POST", "/commands", Body::from(submit)).await;
        assert_eq!(status, StatusCode::OK);
        assert!(accepted.contains("\"outcome\":\"accepted\""));
        let snapshot_path =
            "/patient-workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e/chats/chat-1/snapshot";
        let mut snapshot = String::new();
        for _ in 0..20 {
            let (status, body) = request(app.clone(), "GET", snapshot_path, Body::empty()).await;
            assert_eq!(status, StatusCode::OK);
            snapshot = body;
            if snapshot.contains("Noted.") {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert!(snapshot.contains("Hello"), "{snapshot}");
        assert!(snapshot.contains("Noted."), "{snapshot}");
        let (status, again) = request(app.clone(), "POST", "/commands", Body::from(submit)).await;
        assert_eq!(status, StatusCode::OK);
        assert!(again.contains("\"outcome\":\"accepted\""));
        tokio::task::yield_now().await;
        let (_, snapshot) = request(app, "GET", snapshot_path, Body::empty()).await;
        assert_eq!(snapshot.matches("Noted.").count(), 1, "{snapshot}");
        let (status, rejected) = request(
            live_router(live::ChatModel::Fixed("Noted.".to_owned())),
            "POST",
            "/commands",
            Body::from(r#"{"type":"work.resume","idempotency_key":"k2"}"#),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(rejected.contains("Coda couldn’t resume that work."));
    }

    #[tokio::test]
    async fn replay_answers_chat_from_a_cassette_without_an_api_key() {
        let dir =
            std::env::temp_dir().join(format!("coda-physician-cassette-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("cassette.json");
        let roles = local_product::model_roles("codex-exec", &[]).unwrap();
        let mut session = local_product::ChatSession::new(roles.chat);
        let recorded = match session.submit("record-1", "pw", "chat", "Hello") {
            local_product::Submit::Accepted { request } => request,
            other => panic!("expected a recorded turn, got {other:?}"),
        };
        let mut recording = local_product::codex::CodexTransport::recording(
            Box::new(ScriptedReply("Noted from the cassette.")),
            "synthetic test",
            path.clone(),
        )
        .unwrap();
        assert_eq!(
            recording.complete(&recorded).unwrap(),
            "Noted from the cassette."
        );
        let app = model_router(
            local_product::ModelExecution::Replay,
            None,
            None,
            local_product::codex::CodexSettings {
                cassette_authorization: None,
                cassette_output: None,
                cassette_path: Some(path.to_str().unwrap()),
            },
            &[],
        )
        .unwrap()
        .layer(middleware::from_fn(cors));
        let live = model_router(
            local_product::ModelExecution::Live,
            None,
            None,
            local_product::codex::CodexSettings::default(),
            &[],
        )
        .unwrap();
        let (status, health) = request(live, "GET", "/health", Body::empty()).await;
        assert_eq!(status, StatusCode::OK, "{health}");
        assert!(
            model_router(
                local_product::ModelExecution::Api,
                None,
                None,
                local_product::codex::CodexSettings::default(),
                &[],
            )
            .unwrap_err()
            .contains("CODA_ALLOW_MODEL_CALLS=1")
        );
        let submit = r#"{"type":"message.submit","idempotency_key":"k-replay","payload":{"patient_workspace_id":"pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e","chat_id":"chat-1","text":"Hello","physician_time_zone":"America/New_York"}}"#;
        let (status, accepted) =
            request(app.clone(), "POST", "/commands", Body::from(submit)).await;
        assert_eq!(status, StatusCode::OK, "{accepted}");
        assert!(accepted.contains("\"outcome\":\"accepted\""));
        let snapshot_path =
            "/patient-workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e/chats/chat-1/snapshot";
        let mut snapshot = String::new();
        for _ in 0..50 {
            let (status, body) = request(app.clone(), "GET", snapshot_path, Body::empty()).await;
            assert_eq!(status, StatusCode::OK);
            snapshot = body;
            if snapshot.contains("Noted from the cassette.") {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        assert!(snapshot.contains("Hello"), "{snapshot}");
        assert!(snapshot.contains("Noted from the cassette."), "{snapshot}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    struct ScriptedReply(&'static str);

    impl local_product::codex::CodexExecutor for ScriptedReply {
        fn run(
            &mut self,
            _request: &local_product::codex::CodexRequest,
        ) -> Result<local_product::codex::CodexProcessResult, String> {
            Ok(local_product::codex::CodexProcessResult::authored(self.0))
        }
    }

    #[tokio::test]
    async fn connected_care_end_to_end_invite_consent_provenance_dedup_revocation_and_exact_text_plan()
     {
        let app = router();
        let patient_id = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";

        // 1. Practice explicitly invites roster patient.
        let (status, invite_body) = request(
            app.clone(),
            "POST",
            &format!("/patient-workspaces/{patient_id}/invite"),
            Body::empty(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{invite_body}");
        let invite_json: Value = serde_json::from_str(&invite_body).unwrap();
        assert_eq!(invite_json["practice_id"], "roger-synthetic-practice");
        assert_eq!(invite_json["patient_workspace_id"], patient_id);
        let token = invite_json["token"].as_str().unwrap();
        assert!(!token.is_empty());

        // 2. Patient accepts invitation and links Duet account, then grants versioned consent.
        let (status, accept_body) = request(
            app.clone(),
            "POST",
            "/patient-invitations/accept",
            Body::from(format!(
                r#"{{"token":"{token}","duet_account_id":"duet-acct-elena"}}"#
            )),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{accept_body}");
        let accept_json: Value = serde_json::from_str(&accept_body).unwrap();
        assert_eq!(accept_json["linked"], true);
        assert_eq!(accept_json["sharing_consented"], false);

        // Before consent, syncing Duet records is forbidden.
        let (status, _) = request(
            app.clone(),
            "POST",
            &format!("/patient-workspaces/{patient_id}/duet-sync"),
            Body::from(r#"{"items":[],"revision_cursor":null}"#),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, consent_body) = request(
            app.clone(),
            "PUT",
            &format!("/patient-workspaces/{patient_id}/consent"),
            Body::from(r#"{"duet_account_id":"duet-acct-elena","consent_version":"v1"}"#),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{consent_body}");
        let consent_json: Value = serde_json::from_str(&consent_body).unwrap();
        assert_eq!(consent_json["sharing_consented"], true);
        assert_eq!(consent_json["consent_version"], "v1");

        // 3. Cadence is the only scheduling signal exposed (`weekly` | `daily_when_changed`).
        let (status, cadence_body) = request(
            app.clone(),
            "PUT",
            &format!("/patient-workspaces/{patient_id}/cadence"),
            Body::from(r#"{"cadence":"daily_when_changed"}"#),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{cadence_body}");
        let cadence_json: Value = serde_json::from_str(&cadence_body).unwrap();
        assert_eq!(cadence_json["cadence"], "daily_when_changed");
        assert!(cadence_json.get("premium").is_none());

        // 4. Preview input cannot claim patient origin; deduplication remains intact.
        let sync_payload = serde_json::json!({
            "items": [
                {
                    "record_id": "rec-quest-ldl",
                    "revision_cursor": "cursor-sha256-1",
                    "content_hash": "hash-quest-ldl",
                    "category": "lab",
                    "title": "LDL Cholesterol",
                    "summary": "98 mg/dL",
                    "recorded_at": "2026-10-13T08:00:00Z",
                    "origin_label": "from Quest",
                    "source_identity": "quest:ldl:2026-10-13",
                    "patient_approved_note": false
                },
                {
                    "record_id": "rec-quest-ldl-duplicate",
                    "revision_cursor": "cursor-sha256-2",
                    "content_hash": "hash-quest-ldl",
                    "category": "lab",
                    "title": "LDL Cholesterol (duplicate)",
                    "summary": "98 mg/dL",
                    "recorded_at": "2026-10-13T08:00:00Z",
                    "origin_label": "from Quest",
                    "source_identity": "quest:ldl:2026-10-13",
                    "patient_approved_note": false
                },
                {
                    "record_id": "luna-note-1",
                    "revision_cursor": "cursor-sha256-3",
                    "content_hash": "hash-luna-note-1",
                    "category": "patient_approved_luna_note",
                    "title": "2026-10-13 · Walking tolerance",
                    "summary": "Walked 30 minutes daily without chest tightness.",
                    "recorded_at": "2026-10-13T09:00:00Z",
                    "origin_label": "",
                    "source_identity": "duet:luna-approved-note:1",
                    "patient_approved_note": true
                }
            ]
        });
        let (status, sync_body) = request(
            app.clone(),
            "POST",
            &format!("/patient-workspaces/{patient_id}/duet-sync"),
            Body::from(serde_json::to_vec(&sync_payload).unwrap()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{sync_body}");
        let sync_json: Value = serde_json::from_str(&sync_body).unwrap();
        assert_eq!(sync_json["ingested_count"], 2);
        let records = sync_json["records"].as_array().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["provenance"], "Physician-entered preview");
        assert_eq!(records[1]["provenance"], "Physician-entered preview");
        assert!(
            records
                .iter()
                .all(|record| record["patient_approved_note"] == false),
            "{records:?}"
        );

        // Workspace endpoint surfaces the patient link and consented Duet records with provenance.
        let (status, ws_body) = request(
            app.clone(),
            "GET",
            &format!("/patient-workspaces/{patient_id}"),
            Body::empty(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let ws_json: Value = serde_json::from_str(&ws_body).unwrap();
        assert_eq!(ws_json["patient_link"]["sharing_consented"], true);
        assert_eq!(ws_json["duet_records"].as_array().unwrap().len(), 2);
        assert!(
            ws_json["duet_records"]
                .as_array()
                .unwrap()
                .iter()
                .all(|record| {
                    record["provenance"] == "Physician-entered preview"
                        && record["patient_approved_note"] == false
                })
        );

        // 5. Physician drafts a plan, approves exact text (v1), edits (invalidating approval),
        //    and approves batched item diffs (v2).
        let draft_v1 = serde_json::json!({
            "items": [
                {
                    "id": "plan-1",
                    "title": "Continue statin",
                    "body": "Continue rosuvastatin 10 mg nightly."
                },
                {
                    "id": "plan-2",
                    "title": "Daily walking",
                    "body": "Walk 30 minutes daily after dinner."
                }
            ]
        });
        let (status, draft_body) = request(
            app.clone(),
            "PUT",
            &format!("/patient-workspaces/{patient_id}/plan/draft"),
            Body::from(serde_json::to_vec(&draft_v1).unwrap()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{draft_body}");
        let draft_json: Value = serde_json::from_str(&draft_body).unwrap();
        assert_eq!(draft_json["currently_approved"], false);
        assert_eq!(draft_json["approved_version"], 0);

        // Mismatched exact text is rejected with 400 BAD_REQUEST.
        let wrong_approve = serde_json::json!({
            "approved_items": [
                {
                    "id": "plan-1",
                    "title": "Continue statin",
                    "body": "Wrong text"
                }
            ]
        });
        let (status, _) = request(
            app.clone(),
            "POST",
            &format!("/patient-workspaces/{patient_id}/plan/approve"),
            Body::from(serde_json::to_vec(&wrong_approve).unwrap()),
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        // Exact-text approval of full first version succeeds -> v1.
        let approve_v1 = serde_json::json!({
            "approved_items": draft_v1["items"]
        });
        let (status, approved_v1_body) = request(
            app.clone(),
            "POST",
            &format!("/patient-workspaces/{patient_id}/plan/approve"),
            Body::from(serde_json::to_vec(&approve_v1).unwrap()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{approved_v1_body}");
        let approved_v1_json: Value = serde_json::from_str(&approved_v1_body).unwrap();
        assert_eq!(approved_v1_json["version"], 1);
        assert_eq!(approved_v1_json["label"], "your doctor's plan");

        // Any edit invalidates prior approval and marks only the edited item as needing approval.
        let draft_v2 = serde_json::json!({
            "items": [
                {
                    "id": "plan-1",
                    "title": "Continue statin",
                    "body": "Increase rosuvastatin to 20 mg nightly."
                },
                {
                    "id": "plan-2",
                    "title": "Daily walking",
                    "body": "Walk 30 minutes daily after dinner."
                }
            ]
        });
        let (status, draft_v2_body) = request(
            app.clone(),
            "PUT",
            &format!("/patient-workspaces/{patient_id}/plan/draft"),
            Body::from(serde_json::to_vec(&draft_v2).unwrap()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{draft_v2_body}");
        let draft_v2_json: Value = serde_json::from_str(&draft_v2_body).unwrap();
        assert_eq!(draft_v2_json["currently_approved"], false);
        assert_eq!(draft_v2_json["items"][0]["needs_approval"], true);
        assert_eq!(draft_v2_json["items"][1]["needs_approval"], false);

        // Approve v2 via batched item diff (only the changed item needs to be approved).
        let approve_v2 = serde_json::json!({
            "approved_items": [
                {
                    "id": "plan-1",
                    "title": "Continue statin",
                    "body": "Increase rosuvastatin to 20 mg nightly."
                }
            ]
        });
        let (status, approved_v2_body) = request(
            app.clone(),
            "POST",
            &format!("/patient-workspaces/{patient_id}/plan/approve"),
            Body::from(serde_json::to_vec(&approve_v2).unwrap()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{approved_v2_body}");
        let approved_v2_json: Value = serde_json::from_str(&approved_v2_body).unwrap();
        assert_eq!(approved_v2_json["version"], 2);
        assert_eq!(approved_v2_json["last_diff"].as_array().unwrap().len(), 1);
        assert_eq!(approved_v2_json["last_diff"][0]["item_id"], "plan-1");

        // 6. One-step stop-sharing revocation immediately blocks Duet sync and hides Duet records.
        let (status, stop_body) = request(
            app.clone(),
            "POST",
            &format!("/patient-workspaces/{patient_id}/stop-sharing"),
            Body::empty(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{stop_body}");
        let stop_json: Value = serde_json::from_str(&stop_body).unwrap();
        assert_eq!(stop_json["sharing_consented"], false);
        assert!(stop_json["revoked_at"].is_u64());

        let (status, restore_body) = request(
            app.clone(),
            "PUT",
            &format!("/patient-workspaces/{patient_id}/consent"),
            Body::from(r#"{"duet_account_id":"duet-acct-elena","consent_version":"v1"}"#),
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{restore_body}");

        let (status, _) = request(
            app.clone(),
            "POST",
            &format!("/patient-workspaces/{patient_id}/duet-sync"),
            Body::from(serde_json::to_vec(&sync_payload).unwrap()),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, ws_after_stop) = request(
            app,
            "GET",
            &format!("/patient-workspaces/{patient_id}"),
            Body::empty(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let ws_after_json: Value = serde_json::from_str(&ws_after_stop).unwrap();
        assert_eq!(ws_after_json["patient_link"]["sharing_consented"], false);
        assert_eq!(ws_after_json["duet_records"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn simulation_router_serves_scenarios_investor_metric_and_guards_invariants() {
        let app = simulation_router();

        // 1. Default connected-walk scenario exposes investor metric on /today and clears stale ED reply.
        let (status, today_body) = request(app.clone(), "GET", "/today", Body::empty()).await;
        assert_eq!(status, StatusCode::OK);
        let today: Value = serde_json::from_str(&today_body).unwrap();
        assert_eq!(today["simulation"]["active_scenario_key"], "connected-walk");
        assert_eq!(
            today["simulation"]["investor_metric"]["items_per_100_patients_per_week"],
            json!(20.0)
        );
        assert!(
            today["rows"].as_array().unwrap().is_empty(),
            "stale ED reply must be cleared after corrected BMP in connected-walk"
        );

        // 2. Jake Smith's workspace holds undecided Losartan out of active_plan_items and records superseded ED reply.
        let jake_path = "/patient-workspaces/pw-82c44e9b-58c0-4af1-bd21-771bf77e101d";
        let (status, jake_body) = request(app.clone(), "GET", jake_path, Body::empty()).await;
        assert_eq!(status, StatusCode::OK);
        let jake: Value = serde_json::from_str(&jake_body).unwrap();
        assert!(jake["replies"].as_array().unwrap().is_empty());
        assert_eq!(jake["superseded_replies"].as_array().unwrap().len(), 1);
        assert_eq!(
            jake["undecided_recommendations"].as_array().unwrap().len(),
            1
        );
        let plan_titles: Vec<&str> = jake["active_plan_items"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| item["title"].as_str())
            .collect();
        assert!(!plan_titles.iter().any(|title| title.contains("Losartan")));

        // 3. Running lab-arrives-ed-reply surfaces the urgent ED reply before correction, then admitting corrected labs clears it.
        let run_cmd = r#"{"type":"simulation.run","payload":{"scenario":"lab-arrives-ed-reply"}}"#;
        let (status, _) = request(app.clone(), "POST", "/commands", Body::from(run_cmd)).await;
        assert_eq!(status, StatusCode::OK);
        let (_, today_before) = request(app.clone(), "GET", "/today", Body::empty()).await;
        assert!(today_before.contains("Urgent potassium 6.3 mmol/L"));

        let correct_cmd = r#"{"type":"simulation.admit_corrected_labs","payload":{"patient_workspace_id":"pw-82c44e9b-58c0-4af1-bd21-771bf77e101d"}}"#;
        let (status, _) = request(app.clone(), "POST", "/commands", Body::from(correct_cmd)).await;
        assert_eq!(status, StatusCode::OK);
        let (_, today_after) = request(app.clone(), "GET", "/today", Body::empty()).await;
        assert!(!today_after.contains("Urgent potassium 6.3 mmol/L"));

        // 4. Physician approving an undecided recommendation moves it into active_plan_items and updates the investor metric.
        let decide_cmd = r#"{"type":"plan.decide_recommendation","payload":{"patient_workspace_id":"pw-82c44e9b-58c0-4af1-bd21-771bf77e101d","recommendation_id":"rec-jake-losartan-50"}}"#;
        let (status, decide_resp) =
            request(app.clone(), "POST", "/commands", Body::from(decide_cmd)).await;
        assert_eq!(status, StatusCode::OK);
        let decide_json: Value = serde_json::from_str(&decide_resp).unwrap();
        assert_eq!(
            decide_json["simulation"]["investor_metric"]["items_per_100_patients_per_week"],
            json!(40.0)
        );
    }
}
