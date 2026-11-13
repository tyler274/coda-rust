//! The physician site for the synthetic practice.
//!
//! Patients come from [`patients`]: the practice's registered patients and what
//! their sources delivered. Postgres owns chat and revisions; the worker selects
//! local subscription or API inference. The site never runs a model.
//! `PHYSICIAN_UPSTREAM` forwards fixture routes to another server.
//! The same server serves the Dioxus page, which mounts
//! into `#main`.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

mod auth;
mod care;
pub mod chart_load;
mod chat_draft;
pub mod durable;
mod feedback;
mod gmail;
pub mod hosting;
mod live;
#[cfg(test)]
mod live_tests;
pub mod patients;
pub mod rehearsal;
mod reply;
pub mod synthetic_chart;
pub mod synthetic_chart_fhir;
mod synthetic_reply;
mod timestamp;

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

/// The fixture patients the tests read.
pub const WORKSPACE_IDS: [&str; 5] = [
    "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee",
    "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d",
    "pw-f5404d89-acde-4bb5-8c42-2fa191766811",
    "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971",
    "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e",
];

/// Builds the HTTP router. `PHYSICIAN_UPSTREAM` forwards physician routes.
pub fn router() -> Router {
    router_with(upstream_from_env())
}

fn router_with(upstream: Option<String>) -> Router {
    let routes = match upstream {
        Some(base) => proxied(base),
        None => fixture_site(),
    };
    routes.layer(middleware::from_fn(cors))
}

/// The site over five fixture patients, for integration tests. It runs no
/// chat, so a command is refused and a draft save answers 409.
pub fn fixture_site() -> Router {
    site()
        .route(
            "/physician-state/drafts/{workspace}/{chat}",
            get(empty_draft).put(draft_not_saved),
        )
        .route("/commands", post(reject_command))
        .route(
            "/patient-workspaces/{id}/chats/{chat}/snapshot",
            get(empty_snapshot),
        )
        .route("/patient-workspaces/events", get(quiet_events))
        .layer(axum::Extension(care::ConnectedCare::fixture()))
}

/// The physician routes over the durable chat owner and its invalidation bus.
/// The caller supplies the same authenticated stores used to accept work.
pub fn durable_site(
    chat: std::sync::Arc<durable::DurableChat>,
    bus: std::sync::Arc<dyn physician_bus::Bus>,
) -> Router {
    let live = live::Live::new(std::sync::Arc::clone(&bus), chat);
    site()
        .route("/commands", post(live::submit))
        .route(
            "/physician-state/drafts/{workspace}/{chat}",
            get(chat_draft::read).put(chat_draft::save),
        )
        .route(
            "/physician-state/reply-drafts/{workspace}/{reply}",
            get(reply::read).put(reply::save),
        )
        .route(
            "/physician-state/feedback/{workspace}/{chat}/{response}",
            axum::routing::put(feedback::save),
        )
        .route(
            "/patient-workspaces/{id}/chats/{chat}/snapshot",
            get(live::snapshot),
        )
        // The bus carries invalidations; chat state remains in Postgres.
        .route(
            "/patient-workspaces/events",
            get(live::events).with_state(bus),
        )
        // The shared workspace route lists this server's chats through it.
        .layer(axum::Extension(std::sync::Arc::clone(&live)))
        .layer(axum::Extension(care::ConnectedCare::new()))
        .with_state(live)
}

fn site<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/health", get(|| async { Json(json!({"status": "ok"})) }))
        .route("/today", get(today))
        .route("/practice-day", get(practice_day))
        .route("/updates", get(updates))
        .route(
            "/recent-conversations",
            get(|| async { Json(json!({ "conversations": [] })) }),
        )
        .route("/patient-workspaces", get(patient_list))
        .route("/patient-workspaces/{id}", get(workspace))
        .route("/patient-roster", get(roster))
        .route("/dictation", post(|| async { dictation_unavailable() }))
        .route("/", get(site_index))
        .route("/index.html", get(site_index))
        .route("/assets/coda-health-logo-transparent.png", get(logo))
        .route("/favicon.svg", get(favicon))
        .fallback(site_file)
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
    request: Request<Body>,
) -> Response {
    if !care::on_roster(request.extensions(), &id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    Json(physician_wire::ChatSnapshotResponse {
        snapshot: physician_wire::ChatSnapshot {
            transcript: Vec::new(),
            feedback: physician_wire::SnapshotFeedback::Loaded,
        },
    })
    .into_response()
}

async fn quiet_events() -> Response {
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
///
/// One process-wide bus is shared by chat execution and auth. With durable
/// chat (`PHYSICIAN_DURABLE_CHAT=1`) and `PHYSICIAN_DATABASE_URL`, `message.submit` is
/// accepted in Postgres, which wakes `physician-worker`; this process does
/// not run the model. A hosted site requires durable chat. Receipt
/// compaction is aborted after graceful shutdown.
pub async fn serve(listener: tokio::net::TcpListener) {
    serve_holding(listener, std::sync::Arc::default()).await;
}

/// [`serve`], with `memory_patients` in the practice when there is no database.
#[doc(hidden)]
pub async fn serve_holding(
    listener: tokio::net::TcpListener,
    memory_patients: std::sync::Arc<patients::MemoryPatients>,
) {
    let site = site_from_env(|key| env::var(key).ok(), memory_patients).await;
    let (router, compaction) = match site {
        Ok(ready) => ready,
        Err(error) => {
            let database = error.database();
            tracing::error!(
                error.type = database.map(|cause| local_product::runtime::DbFailure::of(cause).error_type()),
                db.response.status_code = database.and_then(local_product::runtime::sqlstate).as_deref(),
                error = %error,
                error.cause = database.map(tracing::field::display),
                "physician site cannot start"
            );
            std::process::exit(1);
        }
    };
    let app = traced(router);
    let served = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await;
    if let Some(compaction) = compaction {
        compaction.abort();
    }
    served.expect("sample server");
}

fn configured(get: &impl Fn(&str) -> Option<String>, key: &str) -> Option<String> {
    get(key)
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

async fn site_from_env(
    get: impl Fn(&str) -> Option<String>,
    memory_patients: std::sync::Arc<patients::MemoryPatients>,
) -> Result<(Router, Option<tokio::task::JoinHandle<()>>), hosting::StartupError> {
    let hosted = hosting::is_hosted(&get);
    let mode = ChatMode::from_env(&get)?;
    let database = configured(&get, "PHYSICIAN_DATABASE_URL");
    if mode == ChatMode::DurableByTopic {
        tracing::warn!(
            "PHYSICIAN_DURABLE_CHAT is unset and PHYSICIAN_CHAT_TOPIC is set; physician chat runs on the worker"
        );
    }
    let durable = mode.is_durable();
    if hosted && !durable {
        return Err("hosted physician chat requires PHYSICIAN_DURABLE_CHAT=1".into());
    }
    if durable && database.is_none() {
        return Err("PHYSICIAN_DURABLE_CHAT requires PHYSICIAN_DATABASE_URL".into());
    }
    if !durable
        && local_product::model_execution(
            configured(&get, "CODA_LOCAL_MODEL_EXECUTION").as_deref(),
        )? != local_product::ModelExecution::Disabled
    {
        return Err(
            "model execution requires the Postgres chat worker; start scripts/local-practice.sh"
                .into(),
        );
    }
    let required = configured(&get, "PHYSICIAN_AUTH").as_deref() == Some("required");
    if durable && !required {
        return Err("durable physician chat requires PHYSICIAN_AUTH=required".into());
    }
    // Their patients are not the practice's, so access has nothing to enforce.
    if required && configured(&get, "PHYSICIAN_UPSTREAM").is_some() {
        return Err("PHYSICIAN_UPSTREAM cannot run with required sign-in".into());
    }
    let skills = durable
        .then(|| {
            local_product::physician_chat::PhysicianSkills::configured(
                configured(&get, "PHYSICIAN_SKILLS_DIR").as_deref(),
                hosted,
            )
            .map_err(|error| error.to_string())
        })
        .transpose()?;
    let rehearsal = hosting::RehearsalSignIn::from_env(&get)?;
    if rehearsal.is_some() && !required {
        return Err("the rehearsal sign-in needs PHYSICIAN_AUTH=required".into());
    }
    let bus = physician_bus::from_env(&get, hosted).await?;
    // One pool per process: durable chat, the patient directory, auth, the
    // front door and `/readyz` all read through it.
    // A site that serves from the database refuses to start without it; one
    // that only reports readiness from it starts and answers 503.
    let runtime = match &database {
        Some(url) => match local_product::runtime::Store::connect(url).await {
            Ok(store) => Some(store),
            Err(error) if required || durable => {
                return Err(hosting::StartupError::RuntimeStore(error));
            }
            Err(error) => {
                tracing::warn!(%error, "physician readiness store unavailable");
                None
            }
        },
        None => None,
    };
    let readiness = runtime.as_ref().map(hosting::Readiness::shared);
    let (base, chat) = if let (Some(skills), Some(store)) = (skills, runtime.clone()) {
        let settings: Vec<_> = local_product::MODEL_SETTING_KEYS
            .into_iter()
            .filter_map(|key| configured(&get, key).map(|value| (key, value)))
            .collect();
        let environment: Vec<_> = settings
            .iter()
            .map(|(key, value)| (*key, value.as_str()))
            .collect();
        let roles = local_product::model_roles("responses", &environment)
            .map_err(|error| error.to_string())?;
        // The retained snapshot is the stand-in source owner's only import
        // path. Importing it again changes nothing, so every instance can.
        store
            .import_sources(&live::source_snapshot()?)
            .await
            .map_err(|_| "physician source import failed".to_string())?;
        let chat = std::sync::Arc::new(
            durable::DurableChat::new(store, skills, &roles)
                .with_simulation_practices(hosting::SimulationPractices::from_env(&get)?),
        );
        tracing::info!(
            "physician chat accepts turns into Postgres; this process does not run the model"
        );
        (
            durable_site(std::sync::Arc::clone(&chat), std::sync::Arc::clone(&bus))
                .layer(middleware::from_fn(cors)),
            Some(chat),
        )
    } else {
        (router(), None)
    };
    let postgres = runtime.zip(readiness.as_ref());
    let router =
        auth::authed_router_with_bus(base, bus, memory_patients, postgres, rehearsal).await;
    let router = route_readiness(router, readiness);
    let compaction = chat.map(|chat| chat.start_compaction());
    Ok((router, compaction))
}

/// How this site runs physician chat.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChatMode {
    /// In this process; only for a local site without model execution.
    Inline,
    /// Accepted into Postgres, which wakes `physician-worker`.
    Durable,
    /// Durable because `PHYSICIAN_CHAT_TOPIC` is set and
    /// `PHYSICIAN_DURABLE_CHAT` is not. A deployed environment may still
    /// carry only the topic until the `rust-environments` root is applied;
    /// once applied, that root sets `PHYSICIAN_DURABLE_CHAT=1` and this
    /// fallback is inert. The topic itself is no longer read.
    DurableByTopic,
}

impl ChatMode {
    /// `PHYSICIAN_DURABLE_CHAT=1` selects durable chat and `0` inline chat.
    /// Unset, a set `PHYSICIAN_CHAT_TOPIC` selects durable chat (see
    /// [`Self::DurableByTopic`]), and otherwise chat runs inline. Any other
    /// value refuses to start.
    fn from_env(get: &impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        match configured(get, "PHYSICIAN_DURABLE_CHAT").as_deref() {
            Some("1") => Ok(Self::Durable),
            Some("0") => Ok(Self::Inline),
            Some(_) => Err("PHYSICIAN_DURABLE_CHAT must be 0 or 1".into()),
            None if configured(get, "PHYSICIAN_CHAT_TOPIC").is_some() => Ok(Self::DurableByTopic),
            None => Ok(Self::Inline),
        }
    }

    fn is_durable(self) -> bool {
        match self {
            Self::Durable | Self::DurableByTopic => true,
            Self::Inline => false,
        }
    }
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

/// Adds `/readyz`, which checks the stores the process serves with, over its
/// one pool; it never opens a connection of its own. Without
/// `PHYSICIAN_DATABASE_URL` there is nothing to check, so it stays 503.
fn route_readiness(app: Router, readiness: Option<hosting::Readiness>) -> Router {
    app.route(
        "/readyz",
        get(move || {
            let readiness = readiness.clone();
            async move { hosting::ready(readiness.as_ref()).await }
        }),
    )
}

/// Wraps the whole router in the `http.server` tracing layer.
pub fn traced(router: Router) -> Router {
    router.layer(telemetry::http_layer())
}

/// Serves the fixture site on `listener` with authentication required
/// against the given `store`.
pub async fn serve_with_store(
    listener: tokio::net::TcpListener,
    store: std::sync::Arc<dyn physician_auth::AuthStore>,
) {
    let base = router();
    let app = traced(auth::authed_router_with_store(base, store).await);
    axum::serve(listener, app).await.expect("sample server");
}

/// The practice's calendar day, as `/today` and `/updates` carry it. It is the
/// clock's UTC date, which a fixed clock at noon never shows; a real-time
/// practice needs its own zone here.
fn practice_date(now: &str) -> &str {
    now.get(..10).unwrap_or_default()
}

async fn today(
    axum::Extension(connected): axum::Extension<care::ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let Ok(now) = connected.practice_now(request.extensions()).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    Json(json!({
        "date": practice_date(&now),
        "rows": [],
        "seen": [],
        "connections": [],
        "cursors": {},
        "as_of": 0,
        "also_reviewed": 0,
    }))
    .into_response()
}

async fn updates(
    axum::Extension(connected): axum::Extension<care::ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let Ok(now) = connected.practice_now(request.extensions()).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    Json(json!({
        "date": practice_date(&now),
        "rows": [],
        "seen": [],
        "as_of": 0,
        "also_reviewed": 0,
        "next_cursor": null,
        "previous_date": null,
    }))
    .into_response()
}

/// The practice has no calendar source yet, so the day says so.
async fn practice_day(
    axum::Extension(connected): axum::Extension<care::ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let Ok(now) = connected.practice_now(request.extensions()).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    Json(json!({
        "date": practice_date(&now),
        "time_zone": patients::PRACTICE_TIME_ZONE,
        "current_time": now,
        "source_state": { "state": "absent" },
        "visits": [],
    }))
    .into_response()
}

async fn patient_list(
    axum::Extension(connected): axum::Extension<care::ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    let store = connected.auth_store(extensions);
    let panel = store
        .panel(&care::ConnectedCare::identity(extensions))
        .await;
    let directory = connected.patients(extensions);
    let Ok(found) = patients::read_patients(&*directory, &panel).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let rows: Vec<Value> = found.iter().map(patients::Patient::list_row).collect();
    Json(json!({ "patients": rows })).into_response()
}

async fn workspace(
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::Extension(connected): axum::Extension<care::ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let Ok(now) = connected.practice_now(request.extensions()).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let patient = match connected.patient(request.extensions(), &id).await {
        Ok(Some(patient)) => patient,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let mut enriched =
        care::enrich_workspace(patient.workspace_body(&now), &id, request.extensions()).await;
    // A live server lists the signed-in physician's chats. The fixture site
    // has no `Live`.
    if let Some(live) = request.extensions().get::<std::sync::Arc<live::Live>>() {
        let ctx = request.extensions().get::<auth::AuthContext>();
        let Ok(chats) = live.chats(ctx, &id).await else {
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        };
        enriched["chats"] = Value::Array(chats);
        let name = enriched["display_name"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        match synthetic_reply::project(live, ctx, &id, &name).await {
            Ok(Some(reply)) => enriched["replies"] = json!([reply]),
            Ok(None) => {}
            Err(()) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        }
    }
    Json(enriched).into_response()
}

async fn roster(
    axum::Extension(connected): axum::Extension<care::ConnectedCare>,
    Query(query): Query<RosterQuery>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    let store = connected.auth_store(extensions);
    let practice = care::ConnectedCare::identity(extensions).practice_id;
    let registered = store.practice_patients(&practice).await;
    let directory = connected.patients(extensions);
    let query = query.query.unwrap_or_default();
    let Ok(found) = patients::search(&*directory, &registered, &query).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let mut rows: Vec<Value> = found.iter().map(|patient| patient.roster_row()).collect();
    let (connected, store, practice_id) = (&connected, &*store, practice.as_str());
    let links = auth::per_workspace(&rows, |ws_id| async move {
        connected.link_wire(store, practice_id, &ws_id).await
    })
    .await;
    for (row, link) in rows.iter_mut().zip(links) {
        if let Some(link) = link {
            row["duet_linked"] = json!(link.linked);
            row["sharing_consented"] = json!(link.sharing_consented);
            row["cadence"] = json!(link.cadence);
        }
    }
    Json(json!({ "patients": rows, "next_after": null })).into_response()
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

#[derive(Deserialize)]
pub(crate) struct RosterQuery {
    pub(crate) query: Option<String>,
}

/// Returns why this server rejects the command.
pub fn rejection_message(command_type: &str) -> &'static str {
    match command_type {
        "message.submit" => "Coda couldn’t accept that message.",
        "message.withdraw" => "Coda couldn’t remove that queued message.",
        "work.resume" => "Coda couldn’t resume that work.",
        "work.stop" => "Coda couldn’t stop that work.",
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

    fn environment(
        pairs: &'static [(&'static str, &'static str)],
    ) -> impl Fn(&str) -> Option<String> {
        |name: &str| {
            pairs
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned())
        }
    }

    #[test]
    fn chat_mode_follows_the_switch_and_falls_back_to_the_topic() {
        const TOPIC: (&str, &str) = (
            "PHYSICIAN_CHAT_TOPIC",
            "projects/p/topics/physician-chat-turns",
        );
        type Case = (
            &'static [(&'static str, &'static str)],
            Result<ChatMode, &'static str>,
        );
        let cases: [Case; 8] = [
            (&[], Ok(ChatMode::Inline)),
            (&[("PHYSICIAN_DURABLE_CHAT", "1")], Ok(ChatMode::Durable)),
            (&[("PHYSICIAN_DURABLE_CHAT", "0")], Ok(ChatMode::Inline)),
            (&[TOPIC], Ok(ChatMode::DurableByTopic)),
            (
                &[TOPIC, ("PHYSICIAN_DURABLE_CHAT", "1")],
                Ok(ChatMode::Durable),
            ),
            // An explicit 0 turns durable chat off even while the topic is set.
            (
                &[TOPIC, ("PHYSICIAN_DURABLE_CHAT", "0")],
                Ok(ChatMode::Inline),
            ),
            (
                &[("PHYSICIAN_DURABLE_CHAT", "true")],
                Err("PHYSICIAN_DURABLE_CHAT must be 0 or 1"),
            ),
            (
                &[TOPIC, ("PHYSICIAN_DURABLE_CHAT", "yes")],
                Err("PHYSICIAN_DURABLE_CHAT must be 0 or 1"),
            ),
        ];
        for (pairs, expected) in cases {
            assert_eq!(
                ChatMode::from_env(&environment(pairs)),
                expected.map_err(str::to_owned),
                "{pairs:?}"
            );
        }
        assert!(ChatMode::Durable.is_durable());
        assert!(ChatMode::DurableByTopic.is_durable());
        assert!(!ChatMode::Inline.is_durable());
    }

    #[tokio::test]
    async fn hosted_site_requires_durable_chat() {
        for pairs in [
            &[("K_SERVICE", "physician")][..],
            &[("K_SERVICE", "physician"), ("PHYSICIAN_DURABLE_CHAT", "0")],
        ] {
            let error = site_from_env(environment(pairs), std::sync::Arc::default())
                .await
                .expect_err("hosted chat requires its durable owner");
            assert_eq!(
                error.to_string(),
                "hosted physician chat requires PHYSICIAN_DURABLE_CHAT=1"
            );
        }
    }

    #[tokio::test]
    async fn durable_chat_requires_the_physician_database() {
        for pairs in [
            &[("PHYSICIAN_DURABLE_CHAT", "1")][..],
            &[(
                "PHYSICIAN_CHAT_TOPIC",
                "projects/p/topics/physician-chat-turns",
            )],
        ] {
            let error = site_from_env(environment(pairs), std::sync::Arc::default())
                .await
                .expect_err("durable chat without a database");
            assert_eq!(
                error.to_string(),
                "PHYSICIAN_DURABLE_CHAT requires PHYSICIAN_DATABASE_URL"
            );
        }
        let (_, compaction) = site_from_env(
            environment(&[("PHYSICIAN_DURABLE_CHAT", "0")]),
            std::sync::Arc::default(),
        )
        .await
        .expect("durable chat off starts locally");
        assert!(compaction.is_none(), "no durable chat, so no compaction");
    }

    #[tokio::test]
    async fn required_sign_in_serves_only_the_practice() {
        let environment = |name: &str| {
            matches!(name, "PHYSICIAN_AUTH" | "PHYSICIAN_UPSTREAM").then(|| "required".to_owned())
        };
        let error = site_from_env(environment, std::sync::Arc::default())
            .await
            .expect_err("required sign-in over other routes");
        assert!(error.to_string().contains("PHYSICIAN_UPSTREAM"), "{error}");
    }

    #[tokio::test]
    async fn serves_patients_and_no_work_it_does_not_hold() {
        let (status, today) = get("/today").await;
        assert_eq!(status, StatusCode::OK);
        assert!(today.contains("\"rows\":[]"), "{today}");
        let (status, day) = get("/practice-day").await;
        assert_eq!(status, StatusCode::OK);
        let day: Value = serde_json::from_str(&day).unwrap();
        assert_eq!(day["source_state"]["state"], "absent");
        assert_eq!(day["visits"], json!([]));
        assert_eq!(day["current_time"], "2026-08-27T16:00:00.000Z");
        let (status, mary) =
            get("/patient-workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e").await;
        assert_eq!(status, StatusCode::OK);
        let mary: Value = serde_json::from_str(&mary).unwrap();
        assert_eq!(mary["display_name"], "Mary Louis");
        assert_eq!(mary["identity"]["age_years"], 54);
        assert_eq!(
            mary["connected_sources"][0]["state"]["synchronized_at"],
            "2026-08-27T16:00:00.000Z"
        );
        let (status, listed) = get("/patient-workspaces").await;
        assert_eq!(status, StatusCode::OK);
        assert!(listed.contains("Mary Louis"), "{listed}");
        let (status, found) = get("/patient-roster?query=Mary").await;
        assert_eq!(status, StatusCode::OK);
        assert!(found.contains("1972-03-18"), "{found}");
        let (status, blank) = get("/patient-roster?query=").await;
        assert_eq!(status, StatusCode::OK);
        assert!(blank.contains("\"patients\":[]"), "{blank}");
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
        let snapshot: physician_wire::ChatSnapshotResponse =
            serde_json::from_str(&snapshot).expect("the page's snapshot shape");
        assert!(snapshot.snapshot.transcript.is_empty());
    }

    #[tokio::test]
    async fn a_registered_patient_no_source_holds_is_not_found() {
        let (status, _) = get("/patient-workspaces/pw-nobody").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn an_unset_upstream_serves_the_site_and_a_set_one_forwards() {
        let (status, today) = request(router_with(None), "GET", "/today", Body::empty()).await;
        assert_eq!(status, StatusCode::OK);
        assert!(today.contains("\"rows\":[]"), "{today}");
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
    async fn connected_care_patient_link() {
        use physician_auth::{AuthStore, MemoryAuthStore};
        let patient_id = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
        let store = MemoryAuthStore::new();
        store.seed_patient("roger-synthetic-practice", patient_id);
        let store: std::sync::Arc<dyn AuthStore> = std::sync::Arc::new(store);
        let app = router().layer(axum::Extension(store.clone()));

        // 1. The practice site hosts neither patient acceptance nor consent.
        for (method, path, body) in [
            (
                "POST",
                "/patient-invitations/accept".to_owned(),
                r#"{"token":"t","duet_account_id":"duet-acct-elena"}"#,
            ),
            (
                "PUT",
                format!("/patient-workspaces/{patient_id}/consent"),
                r#"{"duet_account_id":"duet-acct-elena","consent_version":"v1"}"#,
            ),
        ] {
            let (status, body) = request(app.clone(), method, &path, Body::from(body)).await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{path}: {body}");
        }

        // 2. Fixture setup stands in for the patient's invitation and consent.
        let token = store
            .invite_patient("roger-synthetic-practice", patient_id, 0)
            .await
            .unwrap();
        store
            .accept_patient_invitation(&token, "duet-acct-elena", 0)
            .await
            .unwrap();
        store
            .grant_patient_consent(
                "roger-synthetic-practice",
                patient_id,
                "duet-acct-elena",
                "v1",
                1,
            )
            .await
            .unwrap();

        // 3. The patient page carries the link's consent state.
        let (status, ws_body) = request(
            app.clone(),
            "GET",
            &format!("/patient-workspaces/{patient_id}"),
            Body::empty(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let ws_json: Value = serde_json::from_str(&ws_body).unwrap();
        assert_eq!(ws_json["patient_link"]["linked"], true);
        assert_eq!(ws_json["patient_link"]["sharing_consented"], true);
        assert!(ws_json.get("duet_records").is_none());
    }
}
