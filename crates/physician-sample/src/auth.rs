//! Physician sign-in and access enforcement on the sample server.
//!
//! `PHYSICIAN_AUTH=required` resolves every physician route's session from
//! the request's `Authorization: Bearer` header through [`physician_auth`]:
//! unknown tokens never read, cross-practice or unknown workspaces answer 404,
//! physicians in the same clinic read basic demographic information for other
//! patients in their clinic, and full clinical workspace/command access
//! requires the patient to be on the physician's panel (`Membership::Allowed`).
//! Clinic admins, secretaries, and physicians holding panel-assignment (`admin`)
//! permission may assign or remove patients on a physician's panel within their
//! clinic, invite a roster patient to Duet through the patient front door
//! ([`front_door`]), and read one linked patient's account and premium/cadence
//! status there. The auth routes themselves stay open (or resolve their own bearer
//! token), and `/health` stays open because a load balancer needs it
//! unauthenticated. The store is Postgres (SeaORM) when `PHYSICIAN_DATABASE_URL`
//! is set — schemas must already be migrated — and process memory otherwise. The
//! front door rides the same database when it is set, so a practice's
//! invitations and premium reads survive a restart.
//!
//! Identity on the front door is email-based for now: whether Duet accounts
//! keep a shared email magic-link provider or move to Sign in with Apple is
//! an open "Discuss with Dev" item, not decided here.
//! Book: `book/src/physician-site.md`.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::body::{Body, Bytes};
use axum::extract::{Path, Request, State};
use axum::http::{Method, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use futures_util::{StreamExt, stream};
use serde::Deserialize;
use serde_json::{Value, json};

use front_door::{FrontDoorStore, MemoryFrontDoorStore, PostgresFrontDoorStore, RosterPatient};
use physician_auth::{
    AuthStore, AuthorityFeed, CachedAuthStore, Membership, MemoryAuthStore, PanelError,
    PhysicianIdentity, PostgresAuthStore, ReviewCadence, SessionError, SignInError,
};

/// The staging practice the recorded fixture workspaces are seeded into by default.
pub const STAGING_PRACTICE_ID: &str = "roger-synthetic-practice";

/// Unix seconds now.
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default()
}

/// The resolved per-request extension: the session's physician identity.
pub(crate) type ResolvedSession = PhysicianIdentity;

/// Per-request auth context attached to authenticated requests so background
/// or streaming handlers can re-verify session and panel access.
#[derive(Clone)]
pub(crate) struct AuthContext {
    pub(crate) auth: Arc<Auth>,
    pub(crate) token: String,
    pub(crate) identity: PhysicianIdentity,
}

/// The auth state shared by the routes and the guard.
pub(crate) struct Auth {
    /// The durable store: Postgres when configured, memory otherwise.
    pub(crate) store: Arc<dyn AuthStore>,
    /// Serializes local authority mutations with publication. PostgreSQL also
    /// holds row locks against mutations from other server instances.
    pub(crate) publication: tokio::sync::Mutex<()>,
    /// Proof-mode sign-in tokens, kept in memory instead of mail. Hosted
    /// mode leaves this empty.
    pub(crate) delivery: std::sync::Mutex<Vec<String>>,
    /// Proof mode returns tokens in JSON. Hosted mode uses the configured mail transport.
    pub(crate) mail: crate::gmail::Delivery,
    /// The Duet front door this practice invites patients through. One
    /// store serves every practice; isolation is enforced per request.
    pub(crate) front_door: Arc<dyn FrontDoorStore>,
    /// Workspace invalidation bus shared with chat execution. `live::events`
    /// is the only SSE encoder; the guard filters that stream and does not
    /// subscribe again. Session revocation still arrives on the Postgres
    /// NOTIFY feed.
    pub(crate) events: Arc<dyn physician_bus::Bus>,
}

/// Registers the recorded practice's five workspaces to `STAGING_PRACTICE_ID`
/// (if not already registered to a practice) and grants `"staging-physician"`
/// panel access. Runs before the router is built so a workspace check never
/// races its grant.
async fn seed_staging(store: &dyn AuthStore) {
    let identity = PhysicianIdentity {
        practice_id: STAGING_PRACTICE_ID.to_owned(),
        physician_id: "staging-physician".to_owned(),
    };
    for workspace in super::WORKSPACE_IDS {
        if store.patient_practice(workspace).await.is_none() {
            store.register_patient(STAGING_PRACTICE_ID, workspace).await;
        }
        if store.patient_practice(workspace).await.as_deref() == Some(STAGING_PRACTICE_ID) {
            store
                .set_membership(&identity, workspace, Membership::Allowed)
                .await;
        }
    }
}

impl Auth {
    /// Builds the auth state on one pair of stores after seeding the
    /// staging access. The front door is chosen beside the auth store:
    /// Postgres rides the same database URL, memory rides local runs.
    pub(crate) async fn on(
        store: Arc<dyn AuthStore>,
        front_door: Arc<dyn FrontDoorStore>,
    ) -> Arc<Self> {
        Self::with_bus(
            store,
            front_door,
            Arc::new(physician_bus::LocalBus::new(64)),
        )
        .await
    }

    /// Same stores as [`Self::on`], publishing on a caller-owned bus.
    /// Sign-in stays in proof mode.
    pub(crate) async fn with_bus(
        store: Arc<dyn AuthStore>,
        front_door: Arc<dyn FrontDoorStore>,
        events: Arc<dyn physician_bus::Bus>,
    ) -> Arc<Self> {
        Self::configured(store, front_door, events, crate::gmail::Delivery::Proof).await
    }

    /// Stores, a workspace bus, and one mail delivery.
    ///
    /// [`Delivery::Proof`](crate::gmail::Delivery::Proof) is what local runs
    /// and tests use. The process entrypoint passes
    /// [`Delivery::from_env`](crate::gmail::Delivery::from_env) so a configured
    /// mail transport switches the same routes to hosted mail.
    pub(crate) async fn configured(
        store: Arc<dyn AuthStore>,
        front_door: Arc<dyn FrontDoorStore>,
        events: Arc<dyn physician_bus::Bus>,
        mail: crate::gmail::Delivery,
    ) -> Arc<Self> {
        seed_staging(&*store).await;
        Arc::new(Self {
            store,
            publication: tokio::sync::Mutex::new(()),
            delivery: std::sync::Mutex::new(Vec::new()),
            front_door,
            events,
            mail,
        })
    }

    /// Builds the process-memory auth state for tests.
    #[cfg(test)]
    pub(crate) async fn memory() -> Arc<Self> {
        Self::memory_with(
            Arc::new(physician_bus::LocalBus::new(64)),
            crate::gmail::Delivery::Proof,
        )
        .await
    }

    async fn memory_with(
        events: Arc<dyn physician_bus::Bus>,
        mail: crate::gmail::Delivery,
    ) -> Arc<Self> {
        let store = Arc::new(MemoryAuthStore::new());
        store.seed_account(
            STAGING_PRACTICE_ID,
            "staging-physician",
            "physician@codahealth.test",
            "Staging Physician",
            true,
        );
        Self::configured(store, Arc::new(MemoryFrontDoorStore::new()), events, mail).await
    }

    /// Postgres auth on a caller-owned bus. Does not migrate.
    pub(crate) async fn postgres_with_bus(
        database_url: &str,
        events: Arc<dyn physician_bus::Bus>,
        mail: crate::gmail::Delivery,
    ) -> Result<Arc<Self>, String> {
        let store = PostgresAuthStore::connect_without_migrations(database_url)
            .await
            .map_err(|error| format!("physician auth store: {error}"))?;
        store
            .check_schema()
            .await
            .map_err(|error| format!("physician auth schema; run migration job: {error}"))?;
        let front_door = PostgresFrontDoorStore::connect_without_migrations(database_url)
            .await
            .map_err(|error| format!("front-door store: {error}"))?;
        front_door
            .check_schema()
            .await
            .map_err(|error| format!("front-door schema; run migration job: {error}"))?;
        // Authority reads are answered from memory between changes; the
        // database trigger feed invalidates them across replicas.
        let store = CachedAuthStore::postgres(store);
        Ok(Self::configured(Arc::new(store), Arc::new(front_door), events, mail).await)
    }
}

/// The presented bearer token, read synchronously before any await: the
/// request itself is not `Sync`, so no handler may hold it across one.
pub(crate) fn bearer(request: &Request<Body>) -> Option<String> {
    let header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())?;
    header
        .strip_prefix("Bearer ")
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
}

/// Why a presented session does not resolve. The page answers each one
/// differently: an ended session asks to sign in again over the open page,
/// and removed access returns to the sign-in page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Refused {
    /// No session, or it ended or expired.
    NoSession,
    /// The session's account no longer has access to the practice.
    AccessRemoved,
}

impl Refused {
    /// The `reason` a refused read carries beside its 401.
    fn reason(self) -> &'static str {
        match self {
            Self::NoSession => "no_session",
            Self::AccessRemoved => "access_removed",
        }
    }
}

/// Resolves the presented bearer token.
pub(crate) async fn resolve(
    auth: &Auth,
    token: Option<String>,
) -> Result<ResolvedSession, Refused> {
    let token = token.ok_or(Refused::NoSession)?;
    auth.store
        .session(&token, now())
        .await
        .map_err(|error| match error {
            SessionError::Invalid => Refused::NoSession,
            SessionError::Inactive => Refused::AccessRemoved,
        })
}

/// Whether this path is one the guard protects: exactly the physician
/// routes, the one list the proxy also forwards, minus `/health`, which the
/// load balancer needs unauthenticated. The auth routes establish the
/// session and the static site renders it, so both stay open.
fn guarded(path: &str) -> bool {
    crate::physician_route(path) && path != "/health"
}

/// The route guard: resolves the session before any physician read,
/// resolves workspace membership before any patient read or command dispatch,
/// filters multi-patient projections by the caller's practice and panel, and
/// terminates SSE streams when session or panel access is revoked.
pub(crate) async fn guard(
    State(auth): State<Arc<Auth>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    request.extensions_mut().insert(Arc::clone(&auth.store));
    if !guarded(request.uri().path()) {
        return next.run(request).await;
    }
    // The token is read before the await; the request moves into `next` after.
    let token = bearer(&request);
    let identity = match resolve(&auth, token.clone()).await {
        Ok(identity) => identity,
        Err(refused) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "detail": "Sign in to read the practice.",
                    "reason": refused.reason(),
                })),
            )
                .into_response();
        }
    };
    let token = token.unwrap_or_default();
    let (mut parts, body) = request.into_parts();
    let path = parts.uri.path().to_owned();
    let query = parts.uri.query().map(str::to_owned);
    parts.extensions.insert(AuthContext {
        auth: Arc::clone(&auth),
        token: token.clone(),
        identity: identity.clone(),
    });

    // Per-patient cadence read/write guard (`ENG-14`): Agents reads per-patient
    // cadence without payment status, and physicians cannot override it.
    if let Some(workspace) = cadence_workspace(&path) {
        if parts.method != Method::GET {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "detail": "Physicians cannot override patient review cadence."
                })),
            )
                .into_response();
        }
        if !allows_workspace(&auth, &identity, workspace).await {
            return not_found();
        }
        let cadence = auth.store.patient_cadence(workspace).await;
        return Json(json!({
            "patient_workspace_id": workspace,
            "cadence": cadence.as_str(),
        }))
        .into_response();
    }

    // A command names its patient workspace in the body, not the path. The
    // body is read only for `/commands`, bounded, so no physician command
    // dispatches into a workspace the session's membership denies or overrides
    // per-patient review cadence.
    let body = if path == "/commands" {
        match axum::body::to_bytes(body, COMMAND_BODY_LIMIT).await {
            Ok(bytes) => {
                if let Some(kind) = command_kind(&bytes)
                    && matches!(
                        kind.as_str(),
                        "cadence.set" | "cadence.override" | "review_cadence.set"
                    )
                {
                    return (
                        StatusCode::OK,
                        Json(json!({
                            "outcome": "rejected",
                            "message": "Physicians cannot override patient review cadence.",
                        })),
                    )
                        .into_response();
                }
                if let Some(workspace) = command_workspace(&bytes)
                    && !allows_workspace(&auth, &identity, &workspace).await
                {
                    // 404, not 403: the workspace's existence stays unknown.
                    return not_found();
                }
                Body::from(bytes)
            }
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "detail": "The command body was unreadable." })),
                )
                    .into_response();
            }
        }
    } else {
        body
    };

    if let Some(workspace) = direct_workspace(&path) {
        match auth.store.membership(&identity, workspace).await {
            Membership::Allowed => {
                let response = next.run(Request::from_parts(parts, body)).await;
                let cadence = auth.store.patient_cadence(workspace).await;
                return enrich_allowed_workspace(response, cadence).await;
            }
            Membership::ClinicDemographic => {
                if let Some(raw) = crate::workspace_body(workspace) {
                    return clinic_demographics(raw);
                }
                return not_found();
            }
            Membership::Denied => return not_found(),
        }
    }

    if let Some(workspace) = request_workspace(&path, query.as_deref())
        && !allows_workspace(&auth, &identity, &workspace).await
    {
        // 404, not 403: a denied physician cannot read clinical projections
        // for a patient outside their panel.
        return not_found();
    }

    if path == "/patient-workspaces/events" {
        let watched_workspace = query_param(query.as_deref(), "patient_workspace_id");
        if let Some(ref ws) = watched_workspace
            && !allows_workspace(&auth, &identity, ws).await
        {
            return not_found();
        }
        let response = next.run(Request::from_parts(parts, body)).await;
        return guard_event_stream(response, auth, token, identity, watched_workspace).await;
    }

    let response = next.run(Request::from_parts(parts, body)).await;
    filter_projection_response(&path, query.as_deref(), response, &auth, &identity).await
}

/// The largest command body the guard reads to resolve its workspace.
const COMMAND_BODY_LIMIT: usize = 1_000_000;

/// The top-level `/patient-workspaces/{id}` path, excluding sub-routes like
/// `/chats/...`, `/cadence`, and the practice-wide `/patient-workspaces/events` stream.
fn direct_workspace(path: &str) -> Option<&str> {
    let rest = path.strip_prefix("/patient-workspaces/")?;
    (!rest.is_empty() && rest != "events" && !rest.contains('/')).then_some(rest)
}

/// The `/patient-workspaces/{id}/cadence` path when present.
fn cadence_workspace(path: &str) -> Option<&str> {
    let rest = path.strip_prefix("/patient-workspaces/")?;
    let workspace = rest.strip_suffix("/cadence")?;
    (!workspace.is_empty() && !workspace.contains('/')).then_some(workspace)
}

fn query_param(query: Option<&str>, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    query?
        .split('&')
        .find_map(|pair| pair.strip_prefix(&prefix))
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

async fn enrich_allowed_workspace(response: Response, cadence: ReviewCadence) -> Response {
    if response.status() != StatusCode::OK {
        return response;
    }
    let (parts, body) = response.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, 2_000_000).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Ok(mut value) = serde_json::from_slice::<Value>(&bytes) else {
        return Response::from_parts(parts, Body::from(bytes));
    };
    if let Some(obj) = value.as_object_mut() {
        obj.insert("membership".to_owned(), json!("allowed"));
        obj.insert("review_cadence".to_owned(), json!(cadence.as_str()));
    }
    Json(value).into_response()
}

/// Basic demographic information for a patient in the caller's clinic who is
/// not on the caller's panel: identity demographics are preserved while
/// clinical synopsis, citations, chats, follow-ups, source handles, and
/// review cadence stay omitted.
fn clinic_demographics(raw: &str) -> Response {
    let value: Value = serde_json::from_str(raw).expect("recorded workspace fixture");
    let identity = value.get("identity").map(|id| {
        json!({
            "display_name": id.get("display_name").cloned().unwrap_or(Value::Null),
            "birth_date": id.get("birth_date").cloned().unwrap_or(Value::Null),
            "age_years": id.get("age_years").cloned().unwrap_or(Value::Null),
            "age_as_of": id.get("age_as_of").cloned().unwrap_or(Value::Null),
            "sex_or_gender": id.get("sex_or_gender").cloned().unwrap_or(Value::Null),
            "medical_record_numbers": id
                .get("medical_record_numbers")
                .cloned()
                .unwrap_or_else(|| json!([])),
            "state": id.get("state").cloned().unwrap_or(Value::Null),
        })
    });
    Json(json!({
        "patient_workspace_id": value["patient_workspace_id"],
        "display_name": value["display_name"],
        "membership": "clinic_demographic",
        "time_zone": value["time_zone"],
        "current_time": value["current_time"],
        "cursor": value["cursor"],
        "identity": identity,
        "chats": [],
        "patient_synopsis": null,
        "automation_run": null,
        "active_follow_ups": [],
        "has_past_work": false,
        "connected_sources": [],
    }))
    .into_response()
}

async fn filter_projection_response(
    path: &str,
    query: Option<&str>,
    response: Response,
    auth: &Auth,
    identity: &PhysicianIdentity,
) -> Response {
    if response.status() != StatusCode::OK {
        return response;
    }
    if path == "/today" && query_param(query, "patient_workspace_id").is_some() {
        return response;
    }
    if !matches!(
        path,
        "/today"
            | "/updates"
            | "/recent-conversations"
            | "/patient-workspaces"
            | "/practice-day"
            | "/patient-roster"
    ) {
        return response;
    }
    let (parts, body) = response.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, 2_000_000).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Ok(mut value) = serde_json::from_slice::<Value>(&bytes) else {
        return Response::from_parts(parts, Body::from(bytes));
    };
    let panel: HashSet<String> = auth.store.panel(identity).await.into_iter().collect();
    let practice_has_patients = !auth
        .store
        .practice_patients(&identity.practice_id)
        .await
        .is_empty();

    match path {
        "/today" => {
            filter_workspace_array(&mut value, "rows", &panel);
            filter_workspace_array(&mut value, "seen", &panel);
            if let Some(cursors) = value.get_mut("cursors").and_then(Value::as_object_mut) {
                cursors.retain(|workspace_id, _| panel.contains(workspace_id));
            }
            if !practice_has_patients {
                value["connections"] = json!([]);
            }
        }
        "/updates" => {
            filter_workspace_array(&mut value, "rows", &panel);
            filter_workspace_array(&mut value, "seen", &panel);
        }
        "/recent-conversations" => {
            filter_workspace_array(&mut value, "conversations", &panel);
        }
        "/patient-workspaces" => {
            filter_workspace_array(&mut value, "patients", &panel);
            if let Some(items) = value.get_mut("patients").and_then(Value::as_array_mut) {
                for item in items {
                    if let Some(ws_id) = item
                        .get("patient_workspace_id")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                    {
                        let cadence = auth.store.patient_cadence(&ws_id).await;
                        if let Some(obj) = item.as_object_mut() {
                            obj.insert("review_cadence".to_owned(), json!(cadence.as_str()));
                        }
                    }
                }
            }
        }
        "/practice-day" => {
            if !practice_has_patients {
                value["visits"] = json!([]);
            } else if let Some(visits) = value.get_mut("visits").and_then(Value::as_array_mut) {
                visits.retain(|visit| {
                    match visit.get("patient_workspace_id").and_then(Value::as_str) {
                        Some(ws_id) => panel.contains(ws_id),
                        None => true,
                    }
                });
            }
        }
        "/patient-roster" => {
            if let Some(patients) = value.get_mut("patients").and_then(Value::as_array_mut) {
                let mut retained = Vec::with_capacity(patients.len());
                for patient in std::mem::take(patients) {
                    let same_practice =
                        match patient.get("patient_workspace_id").and_then(Value::as_str) {
                            Some(ws_id) => {
                                auth.store.patient_practice(ws_id).await.as_deref()
                                    == Some(identity.practice_id.as_str())
                            }
                            None => false,
                        };
                    if same_practice {
                        retained.push(patient);
                    }
                }
                *patients = retained;
            }
        }
        _ => {}
    }

    Json(value).into_response()
}

fn filter_workspace_array(value: &mut Value, key: &str, panel: &HashSet<String>) {
    if let Some(items) = value.get_mut(key).and_then(Value::as_array_mut) {
        items.retain(|item| {
            item.get("patient_workspace_id")
                .and_then(Value::as_str)
                .is_some_and(|ws_id| panel.contains(ws_id))
        });
    }
}

async fn guard_event_stream(
    response: Response,
    auth: Arc<Auth>,
    token: String,
    identity: PhysicianIdentity,
    watched_workspace: Option<String>,
) -> Response {
    if response.status() != StatusCode::OK {
        return response;
    }
    let initial_panel: HashSet<String> = auth.store.panel(&identity).await.into_iter().collect();
    let (parts, body) = response.into_parts();
    let inner = body.into_data_stream();
    // Do not subscribe here. `live::events` already encodes the shared bus,
    // including resync. A second subscription would deliver every frame twice.
    let feed = auth.store.authority_changes();
    let state = (
        Box::pin(inner),
        feed,
        auth,
        token,
        identity,
        watched_workspace,
        initial_panel,
        String::new(),
    );
    let stream = stream::unfold(
        state,
        |(
            mut inner,
            mut feed,
            auth,
            token,
            identity,
            watched_workspace,
            initial_panel,
            mut pending,
        )| async move {
            loop {
                tokio::select! {
                    biased;
                    () = authority_moved(&mut feed) => {
                        if !stream_still_authorized(
                            &auth,
                            &token,
                            &identity,
                            watched_workspace.as_deref(),
                            &initial_panel,
                        )
                        .await
                        {
                            return None;
                        }
                    }
                    item = inner.next() => {
                        let chunk = item?.ok()?;
                        if !stream_still_authorized(
                            &auth,
                            &token,
                            &identity,
                            watched_workspace.as_deref(),
                            &initial_panel,
                        )
                        .await
                        {
                            return None;
                        }
                        let text = String::from_utf8_lossy(&chunk);
                        let events = take_sse_events(&mut pending, &text)?;
                        let mut filtered = String::new();
                        for event in events {
                            filtered.push_str(
                                &filter_sse_chunk(
                                    &format!("{event}\n\n"),
                                    &auth,
                                    &identity,
                                    watched_workspace.as_deref(),
                                )
                                .await,
                            );
                        }
                        if !filtered.is_empty() {
                            return Some((
                                Ok::<Bytes, axum::Error>(Bytes::from(filtered)),
                                (
                                    inner,
                                    feed,
                                    auth,
                                    token,
                                    identity,
                                    watched_workspace,
                                    initial_panel,
                                    pending,
                                ),
                            ));
                        }
                    }
                }
            }
        },
    );
    Response::from_parts(parts, Body::from_stream(stream))
}

const SSE_PENDING_LIMIT: usize = 64 * 1024;

/// Append `chunk` and return events that end in a blank line.
///
/// `None` means the buffer passed 64 KiB and the stream must end. A partial
/// tail stays in `pending` and is not forwarded.
fn take_sse_events(pending: &mut String, chunk: &str) -> Option<Vec<String>> {
    pending.push_str(chunk);
    if pending.len() > SSE_PENDING_LIMIT {
        return None;
    }
    let mut events = Vec::new();
    while let Some(end) = pending.find("\n\n") {
        let event: String = pending.drain(..end).collect();
        pending.drain(..2);
        if !event.trim().is_empty() {
            events.push(event);
        }
    }
    Some(events)
}

/// An event that names a workspace outside `allowed` is not forwarded.
/// An event that names no workspace is allowed.
#[cfg(test)]
fn sse_event_names_disallowed_workspace(event: &str, allowed: impl Fn(&str) -> bool) -> bool {
    let mut workspace_id = None;
    for line in event.lines() {
        if let Some(data) = line.strip_prefix("data:")
            && let Ok(val) = serde_json::from_str::<Value>(data.trim())
            && let Some(ws) = val.get("patient_workspace_id").and_then(Value::as_str)
        {
            workspace_id = Some(ws.to_owned());
        }
    }
    match workspace_id {
        Some(ws) => !allowed(&ws),
        None => false,
    }
}

/// How often an open stream re-checks its grant while the authority feed is
/// live. Changes wake it at once; this tick covers session expiry, which is a
/// clock edge rather than a write.
const LIVE_RECHECK: Duration = Duration::from_secs(30);
/// How often an open stream re-checks while no feed is following changes.
const OFFLINE_RECHECK: Duration = Duration::from_secs(1);

/// Resolves when an open stream should re-check its grant: on an authority
/// change, on reconnect of the feed, or on the fallback tick.
async fn authority_moved(feed: &mut Option<physician_auth::watch::Receiver<AuthorityFeed>>) {
    let live = feed.as_ref().is_some_and(|rx| rx.borrow().live);
    let Some(rx) = feed.as_mut() else {
        return tokio::time::sleep(OFFLINE_RECHECK).await;
    };
    let tick = if live { LIVE_RECHECK } else { OFFLINE_RECHECK };
    tokio::select! {
        changed = rx.changed() => {
            if changed.is_err() {
                *feed = None;
            }
        }
        () = tokio::time::sleep(tick) => {}
    }
}

async fn stream_still_authorized(
    auth: &Auth,
    token: &str,
    identity: &PhysicianIdentity,
    watched_workspace: Option<&str>,
    initial_panel: &HashSet<String>,
) -> bool {
    if auth.store.session(token, now()).await.is_err() {
        return false;
    }
    if let Some(ws) = watched_workspace {
        return allows_workspace(auth, identity, ws).await;
    }
    let current_panel: HashSet<String> = auth.store.panel(identity).await.into_iter().collect();
    initial_panel.iter().all(|ws| current_panel.contains(ws))
}

async fn filter_sse_chunk(
    text: &str,
    auth: &Auth,
    identity: &PhysicianIdentity,
    watched_workspace: Option<&str>,
) -> String {
    let mut out = String::new();
    for block in text.split("\n\n") {
        let trimmed = block.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut workspace_id: Option<String> = None;
        for line in trimmed.lines() {
            if let Some(data) = line.strip_prefix("data:")
                && let Ok(val) = serde_json::from_str::<Value>(data.trim())
                && let Some(ws) = val.get("patient_workspace_id").and_then(Value::as_str)
            {
                workspace_id = Some(ws.to_owned());
            }
        }
        let allow = match workspace_id {
            // `event: resync` and any other frame without a workspace id pass
            // through. Membership filtering applies only to workspace frames.
            None => true,
            Some(ws) => {
                if let Some(watched) = watched_workspace
                    && watched != ws
                {
                    false
                } else {
                    allows_workspace(auth, identity, &ws).await
                }
            }
        };
        if allow {
            out.push_str(trimmed);
            out.push_str("\n\n");
        }
    }
    out
}

fn command_kind(bytes: &[u8]) -> Option<String> {
    let body: Value = serde_json::from_slice(bytes).ok()?;
    body.get("type")?
        .as_str()
        .map(str::trim)
        .filter(|kind| !kind.is_empty())
        .map(str::to_owned)
}

/// The patient workspace a command body names, when it names one.
fn command_workspace(bytes: &[u8]) -> Option<String> {
    let body: Value = serde_json::from_slice(bytes).ok()?;
    body.get("payload")?
        .get("patient_workspace_id")?
        .as_str()
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_owned)
}

/// The patient workspace a request addresses: a `/patient-workspaces/{id}`
/// path segment, or `/today`'s `patient_workspace_id` query filter. The
/// exact path `/patient-workspaces/events` is the practice-level change
/// stream and names no single workspace. Query values compare unencoded, so
/// an encoded id fails closed to a 404 rather than wrongly passing.
fn request_workspace(path: &str, query: Option<&str>) -> Option<String> {
    if let Some(rest) = path.strip_prefix("/patient-workspaces/") {
        if rest == "events" {
            return None;
        }
        let id = rest.split('/').next()?;
        return (!id.is_empty()).then(|| id.to_owned());
    }
    if let Some(rest) = path.strip_prefix("/physician-state/drafts/") {
        let id = rest.split('/').next()?;
        return (!id.is_empty()).then(|| id.to_owned());
    }
    if path == "/today" {
        return query
            .and_then(|query| {
                query
                    .split('&')
                    .find_map(|pair| pair.strip_prefix("patient_workspace_id="))
            })
            .filter(|id| !id.is_empty())
            .map(str::to_owned);
    }
    None
}

#[derive(Deserialize)]
struct SignInBody {
    email: String,
}

/// Issues a sign-in token. Proof setups read it from the response. Hosted
/// mode mails the link and answers with the same body as an unknown address.
async fn sign_in(State(auth): State<Arc<Auth>>, body: axum::body::Bytes) -> Response {
    let Ok(body) = serde_json::from_slice::<SignInBody>(&body) else {
        return sign_in_unavailable();
    };
    match auth.store.begin_sign_in(&body.email, now()).await {
        Ok(token) => deliver_sign_in(&auth, &body.email, token).await,
        Err(_) => sign_in_unavailable(),
    }
}

async fn deliver_sign_in(auth: &Auth, email: &str, token: String) -> Response {
    match &auth.mail {
        crate::gmail::Delivery::Proof => {
            auth.delivery.lock().expect("delivery").push(token.clone());
            (
                StatusCode::OK,
                Json(json!({
                    "status": "token_issued",
                    "delivery": "stored",
                    "token": token,
                })),
            )
                .into_response()
        }
        crate::gmail::Delivery::Hosted(sender) => {
            if let Err(error) = sender.send_sign_in(email, &token).await {
                tracing::warn!(%error, "sign-in mail was not sent");
            }
            // The token stays out of the response when the send is missing
            // or fails, and the body matches an unknown address.
            sign_in_unavailable()
        }
    }
}

fn sign_in_unavailable() -> Response {
    (
        StatusCode::OK,
        Json(json!({
            "status": "unavailable",
            "message": "If that address has an account, a sign-in link is on its way.",
        })),
    )
        .into_response()
}

#[derive(Deserialize)]
struct VerifyBody {
    email: String,
    token: String,
}

/// Verifies a sign-in token and returns the bearer session.
async fn verify(State(auth): State<Arc<Auth>>, body: axum::body::Bytes) -> Response {
    let Ok(body) = serde_json::from_slice::<VerifyBody>(&body) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    match auth
        .store
        .complete_sign_in(&body.email, &body.token, now())
        .await
    {
        Ok((identity, session)) => (
            StatusCode::OK,
            Json(json!({
                "session": session,
                "physician_id": identity.physician_id,
                "practice_id": identity.practice_id,
            })),
        )
            .into_response(),
        Err(SignInError::Invalid) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "detail": "That sign-in link is no longer valid." })),
        )
            .into_response(),
        // Only a matching, unexpired token reaches the account check, so only
        // the link's holder learns that it no longer grants access.
        Err(SignInError::Inactive) => (
            StatusCode::FORBIDDEN,
            Json(json!({
                "detail": "This sign-in link no longer gives access to this practice.",
                "reason": "no_access",
            })),
        )
            .into_response(),
    }
}

/// Ends the presented session.
async fn sign_out(State(auth): State<Arc<Auth>>, request: Request<Body>) -> Response {
    let _publication = auth.publication.lock().await;
    if let Some(token) = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
    {
        auth.store.end_session(token).await;
    }
    StatusCode::NO_CONTENT.into_response()
}

/// The session's physician, or 401.
async fn whoami(State(auth): State<Arc<Auth>>, request: Request<Body>) -> Response {
    let token = bearer(&request);
    let Ok(identity) = resolve(&auth, token).await else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let account = auth.store.account_by_identity(&identity).await;
    let admin = account.as_ref().is_some_and(|account| account.admin);
    let practice_accounts: Vec<Value> = if admin {
        auth.store
            .practice_accounts(&identity.practice_id)
            .await
            .into_iter()
            .filter(|acct| acct.active)
            .map(|acct| {
                json!({
                    "physician_id": acct.identity.physician_id,
                    "display_name": acct.display_name,
                    "email": acct.email,
                    "admin": acct.admin,
                })
            })
            .collect()
    } else {
        Vec::new()
    };
    // The expired-session prompt emails its link to this address.
    let email = account
        .as_ref()
        .map(|account| account.email.clone())
        .unwrap_or_default();
    Json(json!({
        "physician_id": identity.physician_id,
        "practice_id": identity.practice_id,
        "email": email,
        "display_name": account.map(|account| account.display_name)
            .unwrap_or_else(|| identity.physician_id.clone()),
        "admin": admin,
        "practice_accounts": practice_accounts,
    }))
    .into_response()
}

/// Reads the patient workspaces on one physician's panel within the caller's
/// practice. Readable by the physician themselves or a clinic admin/secretary
/// (`admin`) in the same practice.
async fn get_panel(
    State(auth): State<Arc<Auth>>,
    Path(physician_id): Path<String>,
    request: Request<Body>,
) -> Response {
    let token = bearer(&request);
    let Ok(caller) = resolve(&auth, token).await else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let caller_admin = auth
        .store
        .account_by_identity(&caller)
        .await
        .is_some_and(|account| account.admin);
    if caller.physician_id != physician_id && !caller_admin {
        return StatusCode::FORBIDDEN.into_response();
    }
    let target = PhysicianIdentity {
        practice_id: caller.practice_id.clone(),
        physician_id,
    };
    if auth.store.account_by_identity(&target).await.is_none() {
        return not_found();
    }
    let workspaces = auth.store.panel(&target).await;
    Json(json!({
        "practice_id": target.practice_id,
        "physician_id": target.physician_id,
        "patient_workspace_ids": workspaces,
    }))
    .into_response()
}

#[derive(Deserialize)]
struct PanelAssignBody {
    #[serde(default = "default_allowed")]
    allowed: bool,
}

fn default_allowed() -> bool {
    true
}

/// Assigns or removes a patient on a physician's panel. Held by clinic admins,
/// secretaries, or physicians granted panel-assignment (`admin`) permission.
async fn put_panel(
    State(auth): State<Arc<Auth>>,
    Path((physician_id, patient_workspace_id)): Path<(String, String)>,
    request: Request<Body>,
) -> Response {
    let token = bearer(&request);
    let Ok(caller) = resolve(&auth, token).await else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), 64 * 1024).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let allowed = if bytes.is_empty() {
        true
    } else {
        let Ok(body) = serde_json::from_slice::<PanelAssignBody>(&bytes) else {
            return StatusCode::BAD_REQUEST.into_response();
        };
        body.allowed
    };
    apply_panel_change(auth, caller, physician_id, patient_workspace_id, allowed).await
}

/// Removes a patient from a physician's panel via `DELETE`.
async fn delete_panel(
    State(auth): State<Arc<Auth>>,
    Path((physician_id, patient_workspace_id)): Path<(String, String)>,
    request: Request<Body>,
) -> Response {
    let token = bearer(&request);
    let Ok(caller) = resolve(&auth, token).await else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    apply_panel_change(auth, caller, physician_id, patient_workspace_id, false).await
}

async fn apply_panel_change(
    auth: Arc<Auth>,
    caller: PhysicianIdentity,
    physician_id: String,
    patient_workspace_id: String,
    allowed: bool,
) -> Response {
    let _publication = auth.publication.lock().await;
    let caller_admin = auth
        .store
        .account_by_identity(&caller)
        .await
        .is_some_and(|account| account.active && account.admin);
    if !caller_admin {
        return StatusCode::FORBIDDEN.into_response();
    }
    let owner_practice = auth.store.patient_practice(&patient_workspace_id).await;
    if owner_practice.as_deref() != Some(caller.practice_id.as_str()) {
        return not_found();
    }
    let membership = if allowed {
        Membership::Allowed
    } else {
        Membership::Denied
    };
    match auth
        .store
        .assign_panel(&caller, &physician_id, &patient_workspace_id, membership)
        .await
    {
        Ok(()) => {
            let _ = auth
                .events
                .publish(physician_bus::Event::WorkspaceChanged {
                    workspace_id: patient_workspace_id.clone(),
                })
                .await;
            Json(json!({
                "practice_id": caller.practice_id,
                "physician_id": physician_id,
                "patient_workspace_id": patient_workspace_id,
                "allowed": allowed,
            }))
            .into_response()
        }
        Err(PanelError::Forbidden) => StatusCode::FORBIDDEN.into_response(),
        Err(PanelError::UnknownTarget) => not_found(),
    }
}

#[derive(Deserialize)]
struct InviteBody {
    /// The patient on this practice's roster to invite to Duet.
    patient_workspace_id: String,
    /// The email the patient will redeem the invitation with.
    email: String,
    /// The invitee's display name, as the practice wrote it.
    display_name: String,
}

/// Invites one roster patient to Duet on the front door. Held by clinic
/// admins, secretaries, or physicians granted panel-assignment (`admin`)
/// permission, like the panel changes. The patient must be on the caller's
/// practice's roster: a cross-practice or unknown patient answers 404, so
/// the roster's existence stays unknown outside the practice. Proof mode
/// returns the redeem token. Hosted mode mails it and drops it from the JSON.
async fn invite(State(auth): State<Arc<Auth>>, request: Request<Body>) -> Response {
    let token = bearer(&request);
    let Ok(caller) = resolve(&auth, token).await else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), 64 * 1024).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(body) = serde_json::from_slice::<InviteBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let caller_admin = auth
        .store
        .account_by_identity(&caller)
        .await
        .is_some_and(|account| account.active && account.admin);
    if !caller_admin {
        return StatusCode::FORBIDDEN.into_response();
    }
    if caller.practice_id != STAGING_PRACTICE_ID
        || !crate::WORKSPACE_IDS.contains(&body.patient_workspace_id.as_str())
    {
        return not_found();
    }
    let patient = RosterPatient {
        practice_id: caller.practice_id.clone(),
        patient_id: body.patient_workspace_id.clone(),
    };
    match auth
        .front_door
        .invite(&patient, &body.email, &body.display_name, now())
        .await
    {
        Ok(token) => deliver_invite(&auth, &caller, &body, token).await,
        Err(front_door::InviteError::Conflict) => (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": "That email is already invited or has an account.",
            })),
        )
            .into_response(),
    }
}

async fn deliver_invite(
    auth: &Auth,
    caller: &PhysicianIdentity,
    body: &InviteBody,
    token: String,
) -> Response {
    let invited = json!({
        "practice_id": caller.practice_id,
        "patient_workspace_id": body.patient_workspace_id,
        "email": body.email,
        "display_name": body.display_name,
        "status": "invited",
    });
    match &auth.mail {
        crate::gmail::Delivery::Proof => {
            let mut invited = invited;
            invited["token"] = json!(token);
            (StatusCode::OK, Json(invited)).into_response()
        }
        crate::gmail::Delivery::Hosted(sender) => {
            if let Err(error) = sender.send_invite(&body.email, &token).await {
                tracing::warn!(%error, "invitation mail was not sent");
                return (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({
                        "detail": "The invitation email could not be sent.",
                    })),
                )
                    .into_response();
            }
            (StatusCode::OK, Json(invited)).into_response()
        }
    }
}

/// Reads one linked roster patient's Duet account and premium status.
///
/// Held by the same admins as the invite: the front door owns who the
/// patient is and whether they pay, and the practice reads it per patient.
/// Identity is email-based for now — whether Duet accounts stay a shared
/// email magic-link provider or move to Sign in with Apple is an open
/// "Discuss with Dev" item, not decided here.
/// Nothing here stores billing: the practice reads only the premium flag
/// and the cadence Agents runs (`weekly`, or `weekly-daily-check` when the
/// patient pays). An unlinked or unknown patient answers 404.
async fn patient_front_door(
    State(auth): State<Arc<Auth>>,
    Path(patient_workspace_id): Path<String>,
    request: Request<Body>,
) -> Response {
    let token = bearer(&request);
    let Ok(caller) = resolve(&auth, token).await else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let caller_admin = auth
        .store
        .account_by_identity(&caller)
        .await
        .is_some_and(|account| account.active && account.admin);
    if !caller_admin {
        return StatusCode::FORBIDDEN.into_response();
    }
    if caller.practice_id != STAGING_PRACTICE_ID
        || !crate::WORKSPACE_IDS.contains(&patient_workspace_id.as_str())
    {
        return not_found();
    }
    let patient = RosterPatient {
        practice_id: caller.practice_id.clone(),
        patient_id: patient_workspace_id.clone(),
    };
    // The practice reads only its own roster: unknown or cross-practice
    // patients are indistinguishable from unlinked ones here.
    let linked = auth
        .front_door
        .linked_accounts(&caller.practice_id)
        .await
        .into_iter()
        .find(|(_, linked)| *linked == patient);
    let Some((account, patient)) = linked else {
        return not_found();
    };
    let premium = auth.front_door.premium(&patient).await;
    (
        StatusCode::OK,
        Json(json!({
            "practice_id": caller.practice_id,
            "patient_workspace_id": patient_workspace_id,
            "account_id": account.id,
            "email": account.email,
            "display_name": account.display_name,
            "active": account.active,
            "premium": premium.premium,
            // The cadence Agents runs; the practice never sees billing.
            "cadence": premium.cadence,
        })),
    )
        .into_response()
}

/// The workspace membership check a recorded handler uses: a denied or
/// missing membership answers 404 so clinical projections stay panel-scoped.
pub(crate) async fn allows_workspace(
    auth: &Auth,
    identity: &PhysicianIdentity,
    patient_workspace_id: &str,
) -> bool {
    auth.store.membership(identity, patient_workspace_id).await == Membership::Allowed
}

/// The 404 a denied workspace check returns. It matches what the recorded
/// handler returns for an unknown workspace, so "denied" and "absent" are
/// indistinguishable to a probing physician.
pub(crate) fn not_found() -> Response {
    StatusCode::NOT_FOUND.into_response()
}

/// Builds the auth routes for one store.
pub(crate) fn auth_routes(auth: Arc<Auth>) -> axum::Router {
    axum::Router::new()
        .route("/auth/sign-in", post(sign_in))
        .route("/auth/verify", post(verify))
        .route("/auth/sign-out", post(sign_out))
        .route("/auth/whoami", get(whoami))
        .route("/auth/panels/{physician_id}", get(get_panel))
        .route(
            "/auth/panels/{physician_id}/{patient_workspace_id}",
            put(put_panel).delete(delete_panel),
        )
        .route("/auth/invitations", post(invite))
        .route(
            "/auth/front-door/{patient_workspace_id}",
            get(patient_front_door),
        )
        .with_state(auth)
}

/// Builds the router with auth required on `base` through one explicit
/// store: the guard resolves the session before any physician read, over
/// the whole merged router, so no physician route bypasses it.
pub(crate) fn required_router(base: axum::Router, auth: Arc<Auth>) -> axum::Router {
    base.merge(auth_routes(Arc::clone(&auth)))
        .layer(axum::middleware::from_fn_with_state(auth, guard))
}

/// Builds the router with auth required on `base` using the provided `store`.
pub async fn authed_router_with_store(
    base: axum::Router,
    store: Arc<dyn AuthStore>,
) -> axum::Router {
    let auth = Auth::on(store, Arc::new(MemoryFrontDoorStore::new())).await;
    required_router(base, auth)
}

async fn inject_auth_store(
    State(auth): State<Arc<Auth>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    request.extensions_mut().insert(Arc::clone(&auth.store));
    next.run(request).await
}

/// Builds the router with auth required.
///
/// `PHYSICIAN_AUTH=required` guards every physician route of `base` through
/// one store — Postgres when `PHYSICIAN_DATABASE_URL` is set (the migration
/// job applies schema changes; this connects without migrating), memory
/// otherwise. Any other value serves `base` with the auth routes still
/// present. Workspace events stay on an in-process bus. The process entrypoint
/// uses [`authed_router_with_bus`] when Redis is configured. Sign-in mail comes
/// from the environment.
pub async fn authed_router(base: axum::Router) -> axum::Router {
    authed_router_with_bus(base, Arc::new(physician_bus::LocalBus::new(64))).await
}

/// Builds the router with auth on `base`, publishing panel changes on `bus`.
///
/// `PHYSICIAN_AUTH=required` guards every physician route through one store —
/// Postgres when `PHYSICIAN_DATABASE_URL` is set, memory otherwise. Any other
/// value serves `base` with the auth routes still present. Neither store is
/// migrated. Sign-in mail comes from the environment.
pub(crate) async fn authed_router_with_bus(
    base: axum::Router,
    bus: Arc<dyn physician_bus::Bus>,
) -> axum::Router {
    let mail = crate::gmail::Delivery::from_env(|key| std::env::var(key).ok());
    let auth = match (auth_mode(), database_url()) {
        (AuthMode::Required, Some(url)) => match Auth::postgres_with_bus(&url, bus, mail).await {
            Ok(auth) => auth,
            Err(error) => {
                tracing::error!(%error, "physician auth store unavailable");
                std::process::exit(1);
            }
        },
        _ => Auth::memory_with(bus, mail).await,
    };
    match auth_mode() {
        AuthMode::Required => required_router(base, auth),
        AuthMode::Open => {
            base.merge(auth_routes(Arc::clone(&auth)))
                .layer(axum::middleware::from_fn_with_state(
                    auth,
                    inject_auth_store,
                ))
        }
    }
}

/// Builds the required router on one explicit pair of stores, both already
/// migrated and seeded. The Postgres integration tests use it to prove the
/// invite → redeem → link → premium journey the deployment wiring builds.
pub async fn authed_router_on(
    base: axum::Router,
    store: Arc<dyn AuthStore>,
    front_door: Arc<dyn FrontDoorStore>,
) -> axum::Router {
    required_router(base, Auth::on(store, front_door).await)
}

enum AuthMode {
    Required,
    Open,
}

fn auth_mode() -> AuthMode {
    match std::env::var("PHYSICIAN_AUTH").as_deref() {
        Ok("required") => AuthMode::Required,
        _ => AuthMode::Open,
    }
}

fn database_url() -> Option<String> {
    let url = std::env::var("PHYSICIAN_DATABASE_URL").ok()?;
    let url = url.trim().to_owned();
    (!url.is_empty()).then_some(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Method, Request, StatusCode};
    use base64::Engine;
    use physician_auth::{Membership, MemoryAuthStore, PhysicianIdentity, ReviewCadence};
    use tower::ServiceExt;

    #[test]
    fn a_disallowed_event_split_across_chunks_is_not_forwarded() {
        let mut pending = String::new();
        let first = take_sse_events(
            &mut pending,
            "event: workspace_changed\ndata: {\"patient_workspace_id\":",
        )
        .expect("partial event stays buffered");
        assert!(first.is_empty());
        assert!(!pending.is_empty());
        let second = take_sse_events(&mut pending, "\"pw-secret\"}\n\n").expect("complete event");
        assert!(pending.is_empty());
        let forwarded: Vec<_> = second
            .iter()
            .filter(|event| !sse_event_names_disallowed_workspace(event, |_| false))
            .collect();
        assert!(forwarded.is_empty());
    }

    const STAGING_EMAIL: &str = "physician@codahealth.test";
    const COLLEAGUE_EMAIL: &str = "colleague@codahealth.test";

    /// The recorded practice behind a required session on one memory store,
    /// plus the concrete stores for seeding a second physician's access and
    /// redeeming an invitation the practice endpoint minted.
    async fn authed_recorded() -> (
        axum::Router,
        Arc<MemoryAuthStore>,
        Arc<MemoryFrontDoorStore>,
    ) {
        let store = Arc::new(MemoryAuthStore::new());
        store.seed_account(
            STAGING_PRACTICE_ID,
            "staging-physician",
            STAGING_EMAIL,
            "Staging Physician",
            true,
        );
        store.seed_account(
            STAGING_PRACTICE_ID,
            "colleague",
            COLLEAGUE_EMAIL,
            "Colleague Physician",
            false,
        );
        let front_door = Arc::new(MemoryFrontDoorStore::new());
        let auth = Auth::on(
            Arc::clone(&store) as Arc<dyn AuthStore>,
            Arc::clone(&front_door) as Arc<dyn FrontDoorStore>,
        )
        .await;
        (required_router(crate::recorded(), auth), store, front_door)
    }

    async fn send(
        app: &axum::Router,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: Option<String>,
    ) -> (StatusCode, String) {
        let mut builder = Request::builder().method(method).uri(path);
        if let Some(token) = token {
            builder = builder.header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"));
        }
        if body.is_some() {
            builder = builder.header(axum::http::header::CONTENT_TYPE, "application/json");
        }
        let request = builder
            .body(Body::from(body.unwrap_or_default()))
            .expect("request");
        let response = app.clone().oneshot(request).await.expect("response");
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
            .await
            .unwrap();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    /// Signs one seeded account in over HTTP and returns its bearer session.
    async fn session_for(app: &axum::Router, email: &str) -> String {
        let (status, body) = send(
            app,
            Method::POST.as_str(),
            "/auth/sign-in",
            None,
            Some(json!({ "email": email }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "sign-in: {body}");
        let token = serde_json::from_str::<Value>(&body).expect("sign-in body")["token"]
            .as_str()
            .expect("issued token")
            .to_owned();
        let (status, body) = send(
            app,
            Method::POST.as_str(),
            "/auth/verify",
            None,
            Some(json!({ "email": email, "token": token }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "verify: {body}");
        serde_json::from_str::<Value>(&body).expect("verify body")["session"]
            .as_str()
            .expect("bearer session")
            .to_owned()
    }

    #[tokio::test]
    async fn reads_without_a_session_never_serialize() {
        let (app, _, _) = authed_recorded().await;
        for path in [
            "/today",
            "/patient-roster?query=Mary",
            "/patient-workspaces",
            &format!("/patient-workspaces/{}", crate::WORKSPACE_IDS[0]),
            "/commands",
            "/patient-workspaces/events",
        ] {
            let (status, body) = send(&app, Method::GET.as_str(), path, None, None).await;
            assert_eq!(status, StatusCode::UNAUTHORIZED, "{path}: {body}");
        }
    }

    #[tokio::test]
    async fn an_unknown_or_expired_token_never_serializes() {
        let (app, _, _) = authed_recorded().await;
        let (status, _) = send(
            &app,
            Method::GET.as_str(),
            "/today",
            Some("codas-unknown"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        // A minted session that was ended stops resolving.
        let session = session_for(&app, STAGING_EMAIL).await;
        let (status, _) = send(
            &app,
            Method::POST.as_str(),
            "/auth/sign-out",
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NO_CONTENT);
        let (status, _) = send(&app, Method::GET.as_str(), "/today", Some(&session), None).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn health_and_the_signed_out_page_stay_open() {
        let (app, _, _) = authed_recorded().await;
        let (status, body) = send(&app, Method::GET.as_str(), "/health", None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("\"status\":\"ok\""), "{body}");
        let (status, page) = send(&app, Method::GET.as_str(), "/", None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert!(page.contains(r#"<main id="main"></main>"#), "page: {page}");
    }

    #[tokio::test]
    async fn whoami_names_the_session_physician() {
        let (app, _, _) = authed_recorded().await;
        let (status, _) = send(&app, Method::GET.as_str(), "/auth/whoami", None, None).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        let session = session_for(&app, STAGING_EMAIL).await;
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            "/auth/whoami",
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert!(body.contains("Staging Physician"), "{body}");
        assert!(
            body.contains(&format!("\"email\":\"{STAGING_EMAIL}\"")),
            "{body}"
        );
    }

    fn reason(body: &str) -> String {
        serde_json::from_str::<Value>(body).expect("refusal body")["reason"]
            .as_str()
            .unwrap_or_default()
            .to_owned()
    }

    #[tokio::test]
    async fn a_refused_read_says_whether_the_session_ended_or_access_was_removed() {
        let (app, store, _) = authed_recorded().await;
        let (status, body) = send(&app, Method::GET.as_str(), "/today", None, None).await;
        assert_eq!(
            (status, reason(&body).as_str()),
            (StatusCode::UNAUTHORIZED, "no_session")
        );
        let ended = session_for(&app, STAGING_EMAIL).await;
        send(
            &app,
            Method::POST.as_str(),
            "/auth/sign-out",
            Some(&ended),
            None,
        )
        .await;
        let (status, body) = send(&app, Method::GET.as_str(), "/today", Some(&ended), None).await;
        assert_eq!(
            (status, reason(&body).as_str()),
            (StatusCode::UNAUTHORIZED, "no_session")
        );
        let held = session_for(&app, COLLEAGUE_EMAIL).await;
        store.deactivate(COLLEAGUE_EMAIL).await;
        let (status, body) = send(&app, Method::GET.as_str(), "/today", Some(&held), None).await;
        assert_eq!(
            (status, reason(&body).as_str()),
            (StatusCode::UNAUTHORIZED, "access_removed")
        );
    }

    #[tokio::test]
    async fn a_held_link_without_access_says_so_and_a_guessed_one_does_not() {
        let (app, store, _) = authed_recorded().await;
        let (_, body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/sign-in",
            None,
            Some(json!({ "email": COLLEAGUE_EMAIL }).to_string()),
        )
        .await;
        let token = serde_json::from_str::<Value>(&body).expect("sign-in body")["token"]
            .as_str()
            .expect("token")
            .to_owned();
        store.deactivate(COLLEAGUE_EMAIL).await;
        let verify = |token: &str| {
            let body = json!({ "email": COLLEAGUE_EMAIL, "token": token }).to_string();
            let app = app.clone();
            async move {
                send(
                    &app,
                    Method::POST.as_str(),
                    "/auth/verify",
                    None,
                    Some(body),
                )
                .await
            }
        };
        let (status, body) = verify(&token).await;
        assert_eq!(
            (status, reason(&body).as_str()),
            (StatusCode::FORBIDDEN, "no_access")
        );
        let (status, body) = verify("codas-guessed").await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{body}");
        assert!(reason(&body).is_empty(), "a guess learns nothing: {body}");
    }

    #[tokio::test]
    async fn an_unknown_email_never_confirms_an_account() {
        let (app, _, _) = authed_recorded().await;
        let (status, body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/sign-in",
            None,
            Some(json!({ "email": "nobody@codahealth.test" }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("\"unavailable\""), "{body}");
    }

    #[tokio::test]
    async fn a_sign_in_token_verifies_once() {
        let (app, _, _) = authed_recorded().await;
        let (_, body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/sign-in",
            None,
            Some(json!({ "email": STAGING_EMAIL }).to_string()),
        )
        .await;
        let token = serde_json::from_str::<Value>(&body).expect("sign-in body")["token"]
            .as_str()
            .expect("token")
            .to_owned();
        let verify = |token: String| {
            let app = app.clone();
            async move {
                send(
                    &app,
                    Method::POST.as_str(),
                    "/auth/verify",
                    None,
                    Some(json!({ "email": STAGING_EMAIL, "token": token }).to_string()),
                )
                .await
            }
        };
        let (status, _) = verify(token.clone()).await;
        assert_eq!(status, StatusCode::OK);
        let (status, body) = verify(token).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{body}");
    }

    #[tokio::test]
    async fn the_staging_physician_reads_their_granted_workspaces() {
        let (app, _, _) = authed_recorded().await;
        let session = session_for(&app, STAGING_EMAIL).await;
        let workspace = crate::WORKSPACE_IDS[0];
        let (status, today) = send(
            &app,
            Method::GET.as_str(),
            &format!("/today?patient_workspace_id={workspace}"),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{today}");
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{workspace}"),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let (status, body) = send(&app, Method::GET.as_str(), "/today", Some(&session), None).await;
        assert_eq!(status, StatusCode::OK, "{body}");
    }

    #[tokio::test]
    async fn same_clinic_physicians_read_basic_demographics_while_clinical_routes_require_panel_membership()
     {
        let (app, store, _) = authed_recorded().await;
        let colleague = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: "colleague".to_owned(),
        };
        store
            .set_membership(&colleague, crate::WORKSPACE_IDS[0], Membership::Allowed)
            .await;
        // WORKSPACE_IDS[1] is in the same clinic, not on `colleague`'s panel.
        let session = session_for(&app, COLLEAGUE_EMAIL).await;
        let other_clinic_patient = crate::WORKSPACE_IDS[1];
        let absent = "pw-00000000-0000-0000-0000-000000000000";

        // Same-clinic non-panel patient returns basic demographics only, no synopsis/chats/sources.
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{other_clinic_patient}"),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let json: Value = serde_json::from_str(&body).expect("demographic workspace");
        assert_eq!(json["display_name"], "Jake Smith");
        assert_eq!(json["membership"], "clinic_demographic");
        assert_eq!(json["identity"]["birth_date"], "1950-03-12");
        assert!(json["patient_synopsis"].is_null(), "{body}");
        assert_eq!(json["chats"], json!([]));
        assert_eq!(json["connected_sources"], json!([]));
        assert!(json["identity"].get("source_handle").is_none(), "{body}");

        // Clinical routes for a non-panel patient and any absent patient still return 404.
        for path in [
            &format!("/patient-workspaces/{absent}"),
            &format!("/patient-workspaces/{other_clinic_patient}/chats/none/snapshot"),
            &format!("/physician-state/drafts/{other_clinic_patient}/none"),
            &format!("/today?patient_workspace_id={other_clinic_patient}"),
        ] {
            let (status, body) = send(&app, Method::GET.as_str(), path, Some(&session), None).await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{path}: {body}");
            assert!(body.is_empty(), "a 404 carried a body: {body}");
        }

        // Cross-clinic physician gets 404 even on /patient-workspaces/{id} and empty roster.
        store.seed_account(
            "other-practice",
            "outsider",
            "outsider@other.test",
            "Outsider",
            false,
        );
        let outsider_session = session_for(&app, "outsider@other.test").await;
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{other_clinic_patient}"),
            Some(&outsider_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
        assert!(body.is_empty(), "{body}");
        let (status, roster_body) = send(
            &app,
            Method::GET.as_str(),
            "/patient-roster?query=Jake",
            Some(&outsider_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(roster_body.contains("\"patients\":[]"), "{roster_body}");

        // The granted workspace reads its full clinical synopsis; practice-level lists still read.
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{}", crate::WORKSPACE_IDS[0]),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert!(body.contains("authored_text"), "{body}");
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            "/patient-workspaces",
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
    }

    #[tokio::test]
    async fn clinic_admin_or_secretary_assigns_patient_to_physician_panel_over_http() {
        let (app, store, _) = authed_recorded().await;
        store.seed_account(
            STAGING_PRACTICE_ID,
            "secretary",
            "secretary@codahealth.test",
            "Clinic Secretary",
            true,
        );
        let secretary_session = session_for(&app, "secretary@codahealth.test").await;
        let colleague_session = session_for(&app, COLLEAGUE_EMAIL).await;
        let workspace = crate::WORKSPACE_IDS[1];

        // Non-admin physician cannot self-assign a patient to their panel.
        let (status, _) = send(
            &app,
            Method::PUT.as_str(),
            &format!("/auth/panels/colleague/{workspace}"),
            Some(&colleague_session),
            Some(json!({ "allowed": true }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);

        // Secretary (or admin physician) assigns the patient to colleague's panel.
        let (status, body) = send(
            &app,
            Method::PUT.as_str(),
            &format!("/auth/panels/colleague/{workspace}"),
            Some(&secretary_session),
            Some(json!({ "allowed": true }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");

        // Colleague now reads their panel and the full clinical workspace for that patient.
        let (status, panel_body) = send(
            &app,
            Method::GET.as_str(),
            "/auth/panels/colleague",
            Some(&colleague_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{panel_body}");
        assert!(panel_body.contains(workspace), "{panel_body}");

        let (status, ws_body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{workspace}"),
            Some(&colleague_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{ws_body}");
        assert!(ws_body.contains("authored_text"), "{ws_body}");

        // Removing the patient via DELETE returns colleague to the clinic-demographic view.
        let (status, del_body) = send(
            &app,
            Method::DELETE.as_str(),
            &format!("/auth/panels/colleague/{workspace}"),
            Some(&secretary_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{del_body}");
        let (status, demo_body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{workspace}"),
            Some(&colleague_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{demo_body}");
        let json: Value = serde_json::from_str(&demo_body).expect("demographic body");
        assert_eq!(json["membership"], "clinic_demographic");
        assert!(json["patient_synopsis"].is_null());
    }

    #[tokio::test]
    async fn projection_lists_filter_by_practice_and_panel_without_staging_practice_assumptions() {
        let (app, store, _) = authed_recorded().await;
        // Move Jake Smith (WORKSPACE_IDS[1]) to a second clinic ("northside-clinic")
        // with its own admin and physician, proving no STAGING_PRACTICE_ID hard-coding.
        let jake = crate::WORKSPACE_IDS[1];
        let mary = crate::WORKSPACE_IDS[4];
        store.register_patient("northside-clinic", jake).await;
        store.seed_account(
            "northside-clinic",
            "north-admin",
            "admin@northside.test",
            "Northside Admin",
            true,
        );
        store.seed_account(
            "northside-clinic",
            "north-doc",
            "doc@northside.test",
            "Northside Doc",
            false,
        );
        // Grant colleague in STAGING_PRACTICE_ID only Mary Louis on their panel.
        let colleague = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: "colleague".to_owned(),
        };
        store
            .set_membership(&colleague, mary, Membership::Allowed)
            .await;

        let colleague_session = session_for(&app, COLLEAGUE_EMAIL).await;
        let north_admin_session = session_for(&app, "admin@northside.test").await;
        let north_doc_session = session_for(&app, "doc@northside.test").await;

        // Colleague in STAGING_PRACTICE_ID sees only Mary on /patient-workspaces, /practice-day, and /today?patient_workspace_id=...
        let (status, mary_today_body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/today?patient_workspace_id={mary}"),
            Some(&colleague_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{mary_today_body}");
        let (status, _) = send(
            &app,
            Method::GET.as_str(),
            &format!("/today?patient_workspace_id={}", crate::WORKSPACE_IDS[0]),
            Some(&colleague_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND);

        let (_, ws_list_body) = send(
            &app,
            Method::GET.as_str(),
            "/patient-workspaces",
            Some(&colleague_session),
            None,
        )
        .await;
        let ws_list_json: Value = serde_json::from_str(&ws_list_body).unwrap();
        let patients = ws_list_json["patients"].as_array().unwrap();
        assert_eq!(patients.len(), 1);
        assert_eq!(patients[0]["patient_workspace_id"], mary);

        let (_, day_body) = send(
            &app,
            Method::GET.as_str(),
            "/practice-day",
            Some(&colleague_session),
            None,
        )
        .await;
        let day_json: Value = serde_json::from_str(&day_body).unwrap();
        let visits = day_json["visits"].as_array().unwrap();
        assert!(!visits.is_empty());
        assert!(
            visits
                .iter()
                .filter_map(|v| v.get("patient_workspace_id").and_then(Value::as_str))
                .all(|ws_id| ws_id == mary),
            "practice-day leaked non-panel visits: {day_body}"
        );

        // Colleague gets 404 on Jake (now in northside-clinic) and cannot find Jake in roster.
        let (status, _) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{jake}"),
            Some(&colleague_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let (_, roster_body) = send(
            &app,
            Method::GET.as_str(),
            "/patient-roster?query=Jake",
            Some(&colleague_session),
            None,
        )
        .await;
        assert!(roster_body.contains("\"patients\":[]"), "{roster_body}");

        // Northside Doc sees Jake in northside-clinic's roster and gets Jake's clinic-demographic card.
        let (_, north_roster) = send(
            &app,
            Method::GET.as_str(),
            "/patient-roster?query=Jake",
            Some(&north_doc_session),
            None,
        )
        .await;
        assert!(north_roster.contains(jake), "{north_roster}");
        let (status, north_demo) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{jake}"),
            Some(&north_doc_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let demo_json: Value = serde_json::from_str(&north_demo).unwrap();
        assert_eq!(demo_json["membership"], "clinic_demographic");

        // Northside Admin assigns Jake to Northside Doc's panel via PUT /auth/panels/...
        let (status, assign_body) = send(
            &app,
            Method::PUT.as_str(),
            &format!("/auth/panels/north-doc/{jake}"),
            Some(&north_admin_session),
            Some(json!({ "allowed": true }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{assign_body}");

        // Northside Doc now reads Jake's full clinical workspace and sees only Jake on /patient-workspaces!
        let (status, north_full) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{jake}"),
            Some(&north_doc_session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(north_full.contains("authored_text"), "{north_full}");
        let (_, north_ws_list) = send(
            &app,
            Method::GET.as_str(),
            "/patient-workspaces",
            Some(&north_doc_session),
            None,
        )
        .await;
        let north_ws_json: Value = serde_json::from_str(&north_ws_list).unwrap();
        let north_patients = north_ws_json["patients"].as_array().unwrap();
        assert_eq!(north_patients.len(), 1);
        assert_eq!(north_patients[0]["patient_workspace_id"], jake);
    }

    #[tokio::test]
    async fn event_guard_forwards_resync_frames() {
        let auth = Auth::memory().await;
        let identity = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: "staging-physician".to_owned(),
        };
        let filtered =
            filter_sse_chunk("event: resync\ndata: {}\n\n", &auth, &identity, None).await;
        assert_eq!(filtered, "event: resync\ndata: {}\n\n");
    }

    #[tokio::test]
    async fn sse_events_filter_to_panel_and_terminate_when_session_or_panel_access_is_revoked() {
        let store = Arc::new(MemoryAuthStore::new());
        store.seed_account(
            STAGING_PRACTICE_ID,
            "staging-physician",
            STAGING_EMAIL,
            "Staging Physician",
            true,
        );
        store.seed_account(
            STAGING_PRACTICE_ID,
            "colleague",
            COLLEAGUE_EMAIL,
            "Colleague Physician",
            false,
        );
        let auth = Auth::on(
            Arc::clone(&store) as Arc<dyn AuthStore>,
            Arc::new(MemoryFrontDoorStore::new()),
        )
        .await;
        let live_base = crate::live_router(crate::live::ChatModel::Fixed("Noted.".to_owned()));
        let app = required_router(live_base, Arc::clone(&auth));

        // Grant colleague only WORKSPACE_IDS[0]; WORKSPACE_IDS[1] is not on colleague's panel.
        let colleague = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: "colleague".to_owned(),
        };
        let allowed_ws = crate::WORKSPACE_IDS[0];
        let other_ws = crate::WORKSPACE_IDS[1];
        store
            .set_membership(&colleague, allowed_ws, Membership::Allowed)
            .await;

        let colleague_session = session_for(&app, COLLEAGUE_EMAIL).await;
        let staging_session = session_for(&app, STAGING_EMAIL).await;

        let req = Request::builder()
            .method(Method::GET)
            .uri("/patient-workspaces/events")
            .header(
                axum::http::header::AUTHORIZATION,
                format!("Bearer {colleague_session}"),
            )
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let mut body_stream = response.into_body().into_data_stream();

        // Trigger a live chat event on `other_ws` (not on colleague's panel) and then on `allowed_ws`.
        let submit_other = json!({
            "type": "message.submit",
            "idempotency_key": "sse-other",
            "payload": {
                "patient_workspace_id": other_ws,
                "chat_id": "chat-1",
                "text": "Other",
                "physician_time_zone": "America/New_York"
            }
        })
        .to_string();
        let _ = send(
            &app,
            Method::POST.as_str(),
            "/commands",
            Some(&staging_session),
            Some(submit_other),
        )
        .await;

        let submit_allowed = json!({
            "type": "message.submit",
            "idempotency_key": "sse-allowed",
            "payload": {
                "patient_workspace_id": allowed_ws,
                "chat_id": "chat-1",
                "text": "Allowed",
                "physician_time_zone": "America/New_York"
            }
        })
        .to_string();
        let _ = send(
            &app,
            Method::POST.as_str(),
            "/commands",
            Some(&staging_session),
            Some(submit_allowed),
        )
        .await;

        let next_frame = tokio::time::timeout(Duration::from_secs(2), body_stream.next())
            .await
            .expect("SSE frame timeout")
            .expect("SSE stream open")
            .expect("SSE frame bytes");
        let text = String::from_utf8_lossy(&next_frame);
        assert!(text.contains(allowed_ws), "expected allowed_ws in {text}");
        assert!(
            !text.contains(other_ws),
            "SSE leaked non-panel workspace event: {text}"
        );

        // Now revoke colleague's panel access to `allowed_ws`: the SSE stream must terminate!
        store
            .set_membership(&colleague, allowed_ws, Membership::Denied)
            .await;
        let closed = tokio::time::timeout(Duration::from_secs(2), body_stream.next())
            .await
            .expect("SSE termination timeout");
        assert!(
            closed.is_none(),
            "SSE stream must terminate when panel membership is revoked"
        );
    }

    #[tokio::test]
    async fn agents_reads_per_patient_review_cadence_without_payment_status_or_physician_override()
    {
        let (app, store, _) = authed_recorded().await;
        let session = session_for(&app, STAGING_EMAIL).await;
        let workspace = crate::WORKSPACE_IDS[0];

        // Default cadence is `weekly` and contains no payment/billing fields.
        let (status, cadence_body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{workspace}/cadence"),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{cadence_body}");
        let cadence_json: Value = serde_json::from_str(&cadence_body).unwrap();
        assert_eq!(cadence_json["cadence"], "weekly");
        assert!(cadence_json.get("payment_status").is_none());
        assert!(cadence_json.get("subscription").is_none());

        // Front door / practice bridge updates cadence to `weekly_and_daily_when_changed`.
        store
            .set_patient_cadence(workspace, ReviewCadence::WeeklyAndDailyWhenChanged)
            .await;
        let (status, ws_body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{workspace}"),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let ws_json: Value = serde_json::from_str(&ws_body).unwrap();
        assert_eq!(ws_json["review_cadence"], "weekly_and_daily_when_changed");

        // Physician cannot override cadence via PUT /patient-workspaces/{id}/cadence or /commands.
        let (status, _) = send(
            &app,
            Method::PUT.as_str(),
            &format!("/patient-workspaces/{workspace}/cadence"),
            Some(&session),
            Some(json!({ "cadence": "weekly" }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, cmd_body) = send(
            &app,
            Method::POST.as_str(),
            "/commands",
            Some(&session),
            Some(
                json!({
                    "type": "cadence.override",
                    "payload": {
                        "patient_workspace_id": workspace,
                        "cadence": "weekly"
                    }
                })
                .to_string(),
            ),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(
            cmd_body.contains("Physicians cannot override patient review cadence."),
            "{cmd_body}"
        );
        assert_eq!(
            store.patient_cadence(workspace).await,
            ReviewCadence::WeeklyAndDailyWhenChanged
        );
    }

    #[tokio::test]
    async fn a_command_into_a_denied_workspace_returns_404() {
        let (app, store, _) = authed_recorded().await;
        let colleague = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: "colleague".to_owned(),
        };
        store
            .set_membership(&colleague, crate::WORKSPACE_IDS[0], Membership::Allowed)
            .await;
        let session = session_for(&app, COLLEAGUE_EMAIL).await;
        let command = |workspace: &str| {
            json!({
                "type": "message.submit",
                "idempotency_key": "guard-proof",
                "payload": {
                    "patient_workspace_id": workspace,
                    "chat_id": "chat-1",
                    "text": "hello",
                    "physician_time_zone": "America/New_York",
                },
            })
            .to_string()
        };
        let (status, body) = send(
            &app,
            Method::POST.as_str(),
            "/commands",
            Some(&session),
            Some(command(crate::WORKSPACE_IDS[1])),
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
        assert!(
            body.is_empty(),
            "a denied command 404 carried a body: {body}"
        );
        // A command into the granted workspace reaches the handler: the
        // recorded server answers its own rejection, not the guard's 404.
        let (status, body) = send(
            &app,
            Method::POST.as_str(),
            "/commands",
            Some(&session),
            Some(command(crate::WORKSPACE_IDS[0])),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert!(body.contains("\"rejected\""), "{body}");
    }

    #[test]
    fn workspace_resolution_names_only_patient_reads() {
        let workspace = crate::WORKSPACE_IDS[0];
        assert_eq!(
            request_workspace("/patient-workspaces/events", None),
            None,
            "the change stream names no single workspace"
        );
        assert_eq!(
            request_workspace(&format!("/patient-workspaces/{workspace}"), None),
            Some(workspace.to_owned())
        );
        assert_eq!(
            request_workspace(
                &format!("/patient-workspaces/{workspace}/chats/c-1/snapshot"),
                None
            ),
            Some(workspace.to_owned())
        );
        assert_eq!(
            request_workspace("/today", Some(&format!("patient_workspace_id={workspace}"))),
            Some(workspace.to_owned())
        );
        assert_eq!(
            request_workspace("/today", Some("patient_workspace_id=")),
            None
        );
        assert_eq!(
            request_workspace("/patient-roster", Some("query=Mary")),
            None
        );
        // An encoded id cannot match a stored membership: it fails closed.
        assert_eq!(
            request_workspace("/today", Some("patient_workspace_id=pw%2Dabc")),
            Some("pw%2Dabc".to_owned())
        );
    }

    #[test]
    fn command_resolution_names_only_bodied_workspaces() {
        let command =
            |payload: Value| json!({ "type": "chat.open", "payload": payload }).to_string();
        assert_eq!(
            command_workspace(command(json!({ "patient_workspace_id": "pw-x" })).as_bytes()),
            Some("pw-x".to_owned())
        );
        assert_eq!(command_workspace(command(json!({})).as_bytes()), None);
        assert_eq!(command_workspace(b"not json"), None);
        assert_eq!(
            command_workspace(command(json!({ "patient_workspace_id": "  " })).as_bytes()),
            None
        );
    }

    /// Invites one patient over HTTP, returning the status and body.
    async fn invite_patient(
        app: &axum::Router,
        token: Option<&str>,
        email: &str,
        workspace: &str,
    ) -> (StatusCode, String) {
        send(
            app,
            Method::POST.as_str(),
            "/auth/invitations",
            token,
            Some(
                json!({
                    "patient_workspace_id": workspace,
                    "email": email,
                    "display_name": "Luna Park",
                })
                .to_string(),
            ),
        )
        .await
    }

    /// Redeems one token at the front door behind the practice server.
    async fn redeem(front_door: &MemoryFrontDoorStore, email: &str, token: &str) -> bool {
        front_door
            .redeem_invitation(email, token, 1_760_000_000)
            .await
            .is_ok()
    }

    #[tokio::test]
    async fn an_admin_invitation_redeems_once_at_the_front_door() {
        let (app, _, front_door) = authed_recorded().await;
        let session = session_for(&app, STAGING_EMAIL).await;
        let workspace = crate::WORKSPACE_IDS[0];
        let (status, body) =
            invite_patient(&app, Some(&session), "luna@example.test", workspace).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let token = serde_json::from_str::<Value>(&body).expect("invitation body")["token"]
            .as_str()
            .expect("invite token")
            .to_owned();
        let patient = front_door
            .invitation("luna@example.test")
            .await
            .expect("pending invitation");
        assert_eq!(
            patient.patient,
            RosterPatient {
                practice_id: STAGING_PRACTICE_ID.to_owned(),
                patient_id: workspace.to_owned(),
            }
        );
        // The patient redeems once: the account links to the roster patient.
        assert!(redeem(&front_door, "luna@example.test", &token).await);
        let (account_id, _) = {
            let account = front_door
                .account_by_email("luna@example.test")
                .await
                .expect("account");
            (account.id, account.link)
        };
        assert_eq!(
            front_door.link(account_id).await.unwrap(),
            RosterPatient {
                practice_id: STAGING_PRACTICE_ID.to_owned(),
                patient_id: workspace.to_owned(),
            }
        );
        // The same token never redeems again.
        assert!(
            !redeem(&front_door, "luna@example.test", &token).await,
            "an invitation redeems once"
        );
    }

    #[tokio::test]
    async fn a_cross_practice_or_unknown_patient_never_gets_invited() {
        let (app, store, front_door) = authed_recorded().await;
        store.seed_account(
            "other-practice",
            "outsider-admin",
            "outsider-admin@other.test",
            "Outsider Admin",
            true,
        );
        let outsider = session_for(&app, "outsider-admin@other.test").await;
        let (status, body) = invite_patient(
            &app,
            Some(&outsider),
            "luna@example.test",
            crate::WORKSPACE_IDS[0],
        )
        .await;
        assert_eq!(
            status,
            StatusCode::NOT_FOUND,
            "a cross-practice invite stays 404: {body}"
        );
        assert!(
            front_door.invitation("luna@example.test").await.is_none(),
            "no invitation was recorded for another practice's patient"
        );
        // An unknown workspace on the practice's own caller also 404s.
        let session = session_for(&app, STAGING_EMAIL).await;
        let (status, body) =
            invite_patient(&app, Some(&session), "other@example.test", "pw-unknown").await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
        assert!(front_door.invitation("other@example.test").await.is_none());
    }

    #[tokio::test]
    async fn a_non_admin_physician_cannot_invite_and_a_conflict_answers_409() {
        let (app, _, front_door) = authed_recorded().await;
        let colleague = session_for(&app, COLLEAGUE_EMAIL).await;
        let (status, body) = invite_patient(
            &app,
            Some(&colleague),
            "luna@example.test",
            crate::WORKSPACE_IDS[0],
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
        assert!(front_door.invitation("luna@example.test").await.is_none());

        // An admin invite that collides with an existing front-door account
        // answers conflict, and the roster patient stays unlinked.
        let session = session_for(&app, STAGING_EMAIL).await;
        front_door.seed_linked_account(
            &RosterPatient {
                practice_id: STAGING_PRACTICE_ID.to_owned(),
                patient_id: crate::WORKSPACE_IDS[1].to_owned(),
            },
            "linked@example.test",
            "Already Linked",
        );
        let (status, body) = invite_patient(
            &app,
            Some(&session),
            "linked@example.test",
            crate::WORKSPACE_IDS[0],
        )
        .await;
        assert_eq!(status, StatusCode::CONFLICT, "{body}");
        assert!(body.contains("already invited"), "{body}");
    }

    /// Seeds one front-door account linked to the staging practice's
    /// `workspace`, or finds one the fixture already seeded.
    async fn linked_account(
        front_door: &MemoryFrontDoorStore,
        email: &str,
        workspace: &str,
    ) -> front_door::Account {
        if let Some(account) = front_door.account_by_email(email).await {
            return account;
        }
        let id = front_door.seed_linked_account(
            &RosterPatient {
                practice_id: STAGING_PRACTICE_ID.to_owned(),
                patient_id: workspace.to_owned(),
            },
            email,
            "Linked Patient",
        );
        front_door.account(id).await.expect("seeded account")
    }

    #[tokio::test]
    async fn the_practice_reads_one_linked_patient_account_and_premium() {
        let (app, _, front_door) = authed_recorded().await;
        let session = session_for(&app, STAGING_EMAIL).await;
        let workspace = crate::WORKSPACE_IDS[2];
        let account = linked_account(&front_door, "reader@example.test", workspace).await;
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/auth/front-door/{workspace}"),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let json: Value = serde_json::from_str(&body).expect("front-door read");
        assert_eq!(json["account_id"], account.id.to_string());
        assert_eq!(json["email"], "reader@example.test");
        assert_eq!(json["active"], true);
        // Nothing here stores billing: the practice reads only the premium
        // flag and the cadence Agents runs, which starts free.
        assert_eq!(json["premium"], false);
        assert_eq!(json["cadence"], front_door::CADENCE_WEEKLY);

        // Paying flips the cadence Agents runs, still without billing.
        let patient = RosterPatient {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            patient_id: workspace.to_owned(),
        };
        front_door
            .set_premium(&patient, true, 1_760_000_000)
            .await
            .unwrap();
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/auth/front-door/{workspace}"),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let json: Value = serde_json::from_str(&body).expect("front-door read");
        assert_eq!(json["premium"], true);
        assert_eq!(json["cadence"], front_door::CADENCE_WEEKLY_DAILY_CHECK);
    }

    #[tokio::test]
    async fn an_unlinked_or_cross_practice_front_door_read_answers_404() {
        let (app, store, front_door) = authed_recorded().await;
        let session = session_for(&app, STAGING_EMAIL).await;
        // A roster patient nobody invited yet answers 404, like the
        // workspace route's unknown patient.
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/auth/front-door/{}", crate::WORKSPACE_IDS[3]),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
        // An account linked to another practice is invisible here, even
        // when the caller asks under a staging workspace id.
        front_door.seed_linked_account(
            &RosterPatient {
                practice_id: "other-practice".to_owned(),
                patient_id: crate::WORKSPACE_IDS[3].to_owned(),
            },
            "elsewhere@example.test",
            "Elsewhere Patient",
        );
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/auth/front-door/{}", crate::WORKSPACE_IDS[3]),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
        // A cross-practice admin reads only their own roster's patients.
        store.seed_account(
            "other-practice",
            "outsider-admin",
            "outsider-admin@other.test",
            "Outsider Admin",
            true,
        );
        let outsider = session_for(&app, "outsider-admin@other.test").await;
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/auth/front-door/{}", crate::WORKSPACE_IDS[2]),
            Some(&outsider),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
    }

    #[tokio::test]
    async fn a_non_admin_cannot_read_the_front_door_panel() {
        let (app, _, front_door) = authed_recorded().await;
        linked_account(&front_door, "reader@example.test", crate::WORKSPACE_IDS[0]).await;
        let colleague = session_for(&app, COLLEAGUE_EMAIL).await;
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/auth/front-door/{}", crate::WORKSPACE_IDS[0]),
            Some(&colleague),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
    }

    fn decode_gmail_raw(sent: &Value) -> String {
        let raw = sent["raw"].as_str().expect("raw field");
        assert!(
            sent.as_object().is_some_and(|object| object.len() == 1),
            "messages.send body should be only raw: {sent}"
        );
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(raw)
            .expect("base64url raw");
        String::from_utf8(bytes).expect("rfc2822")
    }

    fn query_value<'a>(message: &'a str, key: &str) -> &'a str {
        let marker = format!("{key}=");
        message
            .split(&marker)
            .nth(1)
            .unwrap_or_else(|| panic!("missing {key} in {message}"))
            .split(|c: char| c.is_whitespace() || c == '&')
            .next()
            .expect("query value")
    }

    async fn gmail_sender(fail_send: bool) -> (crate::gmail::Outbound, gmail::testing::FakeGmail) {
        let fake = gmail::testing::spawn(fail_send).await;
        let sender = gmail::Sender::refresh(
            "sign-in@coda.health",
            fake.endpoints(),
            "client-id",
            "client-secret",
            "refresh-token",
        );
        (
            crate::gmail::Outbound::new("https://physician.example".into(), sender),
            fake,
        )
    }

    async fn hosted_practice(
        sender: crate::gmail::Outbound,
    ) -> (axum::Router, Arc<MemoryFrontDoorStore>, String) {
        let store = Arc::new(MemoryAuthStore::new());
        store.seed_account(
            STAGING_PRACTICE_ID,
            "staging-physician",
            STAGING_EMAIL,
            "Staging Physician",
            true,
        );
        let issued = store
            .begin_sign_in(STAGING_EMAIL, now())
            .await
            .expect("session sign-in");
        let (_, session) = store
            .complete_sign_in(STAGING_EMAIL, &issued, now())
            .await
            .expect("session");
        let front_door = Arc::new(MemoryFrontDoorStore::new());
        let auth = Auth::configured(
            Arc::clone(&store) as Arc<dyn AuthStore>,
            Arc::clone(&front_door) as Arc<dyn FrontDoorStore>,
            Arc::new(physician_bus::LocalBus::new(64)),
            crate::gmail::Delivery::Hosted(Box::new(sender)),
        )
        .await;
        (
            required_router(crate::recorded(), auth),
            front_door,
            session,
        )
    }

    #[tokio::test]
    async fn hosted_sign_in_mails_the_link_and_omits_the_token() {
        let (sender, fake) = gmail_sender(false).await;
        let (app, _, _) = hosted_practice(sender).await;
        let (status, body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/sign-in",
            None,
            Some(json!({ "email": STAGING_EMAIL }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let value: Value = serde_json::from_str(&body).expect("sign-in body");
        assert_eq!(value["status"], "unavailable");
        assert_ne!(value["status"], "token_issued");
        assert!(value.get("token").is_none(), "{body}");
        assert!(
            body.contains("If that address has an account, a sign-in link is on its way."),
            "{body}"
        );
        let sent = fake.sent.lock().expect("sent").clone();
        let message = decode_gmail_raw(sent.first().expect("one message"));
        assert!(message.contains("From: sign-in@coda.health"), "{message}");
        assert!(
            message.contains(&format!("To: {STAGING_EMAIL}")),
            "{message}"
        );
        let parsed = mailparse::parse_mail(message.as_bytes()).expect("valid MIME email");
        assert_eq!(parsed.ctype.mimetype, "multipart/alternative");
        let text = parsed
            .subparts
            .iter()
            .find(|part| part.ctype.mimetype == "text/plain")
            .expect("plain-text alternative")
            .get_body()
            .expect("decoded text");
        let html = parsed
            .subparts
            .iter()
            .find(|part| part.ctype.mimetype == "text/html")
            .expect("HTML alternative")
            .get_body()
            .expect("decoded HTML");
        assert!(text.contains("expires in 24 hours"), "{text}");
        let code = query_value(&text, "code");
        let link =
            format!("https://physician.example/?code={code}&email=physician%40codahealth.test");
        assert!(text.contains(&link), "{text}");
        assert_eq!(html.matches("href=").count(), 1, "one sign-in action");
        assert!(html.contains(&format!("href=\"{}\"", link.replace('&', "&amp;"))));
        assert!(html.contains(">Sign in</a>"));
        assert!(!html.contains("Sign-in code") && !text.contains("sign-in code"));
        assert!(
            !body.contains(code),
            "the response included the mailed code: {body}"
        );
        let exchange = fake.token_bodies.lock().expect("tokens").clone();
        assert!(
            exchange
                .first()
                .is_some_and(|form| form.contains("grant_type=refresh_token")),
            "token exchange was not a refresh grant: {exchange:?}"
        );
        let (status, verified) = send(
            &app,
            Method::POST.as_str(),
            "/auth/verify",
            None,
            Some(json!({ "email": STAGING_EMAIL, "token": code }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{verified}");
        let (status, unknown) = send(
            &app,
            Method::POST.as_str(),
            "/auth/sign-in",
            None,
            Some(json!({ "email": "nobody@codahealth.test" }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{unknown}");
        assert_eq!(unknown, body, "an unknown address must match a mailed one");
        assert_eq!(fake.sent.lock().expect("sent").len(), 1);
    }

    #[tokio::test]
    async fn hosted_invite_mails_the_redeem_link_and_omits_the_token() {
        let (sender, fake) = gmail_sender(false).await;
        let (app, front_door, session) = hosted_practice(sender).await;
        let workspace = crate::WORKSPACE_IDS[0];
        let (status, body) =
            invite_patient(&app, Some(&session), "luna@example.test", workspace).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let value: Value = serde_json::from_str(&body).expect("invitation body");
        assert_eq!(value["status"], "invited");
        assert_eq!(value["email"], "luna@example.test");
        assert!(value.get("token").is_none(), "{body}");
        let message = decode_gmail_raw(&fake.sent.lock().expect("sent")[0]);
        assert!(message.contains("To: luna@example.test"), "{message}");
        assert!(message.contains("expires in 14 days"), "{message}");
        let token = query_value(&message, "token");
        assert!(
            message.contains(&format!("https://physician.example/invite?token={token}")),
            "{message}"
        );
        assert!(
            !body.contains(token),
            "the response included the mailed token: {body}"
        );
        assert!(
            front_door
                .redeem_invitation("luna@example.test", token, now())
                .await
                .is_ok(),
            "the mailed redeem token should work once"
        );
    }

    #[tokio::test]
    async fn a_failed_or_missing_gmail_send_does_not_return_the_token() {
        let (sender, _) = gmail_sender(true).await;
        let (app, _, session) = hosted_practice(sender).await;
        let (status, body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/sign-in",
            None,
            Some(json!({ "email": STAGING_EMAIL }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let value: Value = serde_json::from_str(&body).expect("sign-in body");
        assert_eq!(value["status"], "unavailable");
        assert!(value.get("token").is_none(), "{body}");
        assert!(!body.contains("codas-"), "{body}");
        let (status, body) = invite_patient(
            &app,
            Some(&session),
            "luna@example.test",
            crate::WORKSPACE_IDS[0],
        )
        .await;
        assert_eq!(status, StatusCode::BAD_GATEWAY, "{body}");
        assert!(body.contains("could not be sent"), "{body}");
        assert!(
            serde_json::from_str::<Value>(&body)
                .unwrap()
                .get("token")
                .is_none()
        );
        assert!(!body.contains("codaf-"), "{body}");

        let missing = crate::gmail::Outbound::new(
            "https://physician.example".into(),
            gmail::Sender::refresh(
                "sign-in@coda.health",
                gmail::Endpoints {
                    send: "http://127.0.0.1:9/gmail/v1/users/me/messages/send".into(),
                    token: "http://127.0.0.1:9/token".into(),
                },
                "",
                "",
                "",
            ),
        );
        let (app, _, session) = hosted_practice(missing).await;
        let (status, body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/sign-in",
            None,
            Some(json!({ "email": STAGING_EMAIL }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert!(
            serde_json::from_str::<Value>(&body)
                .unwrap()
                .get("token")
                .is_none()
        );
        assert!(!body.contains("codas-"), "{body}");
        let (status, body) = invite_patient(
            &app,
            Some(&session),
            "other@example.test",
            crate::WORKSPACE_IDS[1],
        )
        .await;
        assert_eq!(status, StatusCode::BAD_GATEWAY, "{body}");
        assert!(!body.contains("codaf-"), "{body}");
        assert!(
            serde_json::from_str::<Value>(&body)
                .unwrap()
                .get("token")
                .is_none()
        );
    }
}
