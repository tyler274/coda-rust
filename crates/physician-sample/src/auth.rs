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
//! clinic and invite a roster patient to Duet through the patient front door
//! ([`front_door`]). The auth routes themselves stay open (or resolve their own
//! bearer token), and `/health` stays open because a load balancer needs it
//! unauthenticated. The store is Postgres (SeaORM) when `PHYSICIAN_DATABASE_URL`
//! is set — schemas must already be migrated — and process memory otherwise. The
//! front door rides the same database when it is set, so a practice's
//! invitations survive a restart.
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
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use futures_util::{StreamExt, stream};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::timestamp::format_rfc3339;
use front_door::{FrontDoorStore, MemoryFrontDoorStore, RosterPatient};
use physician_auth::{
    AuthStore, AuthorityFeed, CachedAuthStore, Membership, MemoryAuthStore, PanelError,
    PhysicianIdentity, PracticeClock, PracticeTimeZone, SessionError, SignInError,
    StoreUnavailable,
};

use crate::patients::{MemoryPatients, PatientDirectory, enroll_in_memory, read_patient};

/// The staging practice the synthetic charts load into.
pub const STAGING_PRACTICE_ID: &str = "roger-synthetic-practice";

/// The clock the synthetic practice is created with: 2026-08-27 noon Eastern,
/// the moment its charts end.
pub(crate) const SYNTHETIC_PRACTICE_CLOCK: PracticeClock = PracticeClock::Fixed(1_787_846_400);

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
    /// What each source last delivered about the practice's patients.
    pub(crate) patients: Arc<dyn PatientDirectory>,
    /// Practice-scoped clinical source setup, recovery, and backlog state:
    /// PostgreSQL when configured, process memory otherwise.
    pub(crate) source_setup: Arc<crate::source_setup::SourceSetupStore>,
    /// Workspace invalidation bus shared with chat execution. `live::events`
    /// is the only SSE encoder; the guard filters that stream and does not
    /// subscribe again. Session revocation still arrives on the Postgres
    /// NOTIFY feed.
    pub(crate) events: Arc<dyn physician_bus::Bus>,
}

/// The physicians whose panels start with every patient of the staging practice:
/// the preview identity and each active admin.
pub(crate) async fn staging_panel(
    store: &dyn AuthStore,
) -> Result<Vec<PhysicianIdentity>, StoreUnavailable> {
    let mut identities = vec![PhysicianIdentity {
        practice_id: STAGING_PRACTICE_ID.to_owned(),
        physician_id: "staging-physician".to_owned(),
    }];
    for account in store.practice_accounts(STAGING_PRACTICE_ID).await? {
        if account.active
            && account.admin
            && !identities
                .iter()
                .any(|identity| identity == &account.identity)
        {
            identities.push(account.identity);
        }
    }
    Ok(identities)
}

/// Creates the synthetic practice and registers the fixture patients on
/// `store`, on the staging panel, as a load does. A patient another practice
/// holds stays with that practice.
async fn register_fixture_patients(store: &dyn AuthStore) {
    store
        .ensure_practice(STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
        .await
        .expect("create the synthetic practice");
    let panel = staging_panel(store).await.expect("read the staging panel");
    for workspace in crate::WORKSPACE_IDS {
        store
            .register_patient(STAGING_PRACTICE_ID, workspace)
            .await
            .expect("register a fixture patient");
        let owner = store
            .patient_practice(workspace)
            .await
            .expect("read a fixture patient's practice");
        if owner.as_deref() == Some(STAGING_PRACTICE_ID) {
            for identity in &panel {
                store
                    .set_membership(identity, workspace, Membership::Allowed)
                    .await
                    .expect("place a fixture patient on the panel");
            }
        }
    }
}

impl Auth {
    /// Stores, a workspace bus, and one mail delivery. The front door and the
    /// patient directory are chosen beside the auth store: Postgres rides the
    /// same database URL, memory rides local runs.
    ///
    /// [`Delivery::Proof`](crate::gmail::Delivery::Proof) is what local runs
    /// and tests use. The process entrypoint passes
    /// [`Delivery::from_env`](crate::gmail::Delivery::from_env) so a configured
    /// mail transport switches the same routes to hosted mail.
    pub(crate) fn configured(
        store: Arc<dyn AuthStore>,
        front_door: Arc<dyn FrontDoorStore>,
        patients: Arc<dyn PatientDirectory>,
        events: Arc<dyn physician_bus::Bus>,
        mail: crate::gmail::Delivery,
    ) -> Arc<Self> {
        Self::configured_with_source_setup(
            store,
            front_door,
            patients,
            crate::source_setup::SourceSetupStore::new(),
            events,
            mail,
        )
    }

    pub(crate) fn configured_with_source_setup(
        store: Arc<dyn AuthStore>,
        front_door: Arc<dyn FrontDoorStore>,
        patients: Arc<dyn PatientDirectory>,
        source_setup: Arc<crate::source_setup::SourceSetupStore>,
        events: Arc<dyn physician_bus::Bus>,
        mail: crate::gmail::Delivery,
    ) -> Arc<Self> {
        Arc::new(Self {
            store,
            publication: tokio::sync::Mutex::new(()),
            delivery: std::sync::Mutex::new(Vec::new()),
            front_door,
            patients,
            source_setup,
            events,
            mail,
        })
    }

    /// Builds the process-memory auth state for tests.
    #[cfg(test)]
    pub(crate) async fn memory() -> Arc<Self> {
        Self::memory_with(
            Arc::default(),
            Arc::new(physician_bus::LocalBus::new(64)),
            crate::gmail::Delivery::Proof,
            false,
        )
        .await
    }

    async fn memory_with(
        patients: Arc<MemoryPatients>,
        events: Arc<dyn physician_bus::Bus>,
        mail: crate::gmail::Delivery,
        forbid_simulation: bool,
    ) -> Arc<Self> {
        let store = Arc::new(MemoryAuthStore::new());
        store.seed_account(
            STAGING_PRACTICE_ID,
            "staging-physician",
            "physician@codahealth.test",
            "Staging Physician",
            true,
        );
        enroll_in_memory(&store, &patients).await;
        Self::configured_with_source_setup(
            store,
            Arc::new(MemoryFrontDoorStore::new()),
            patients,
            crate::source_setup::SourceSetupStore::memory(forbid_simulation),
            events,
            mail,
        )
    }

    /// Auth over the fixture patients, registered on `store`.
    pub(crate) async fn over_fixture(
        store: Arc<dyn AuthStore>,
        front_door: Arc<dyn FrontDoorStore>,
    ) -> Arc<Self> {
        register_fixture_patients(&*store).await;
        Self::configured(
            store,
            front_door,
            MemoryPatients::fixture(),
            Arc::new(physician_bus::LocalBus::new(64)),
            crate::gmail::Delivery::Proof,
        )
    }

    /// Postgres auth on a caller-owned bus, on the `stores` that read through
    /// the process's `runtime` pool, which also reads patients. Does not
    /// migrate; refuses when either schema does not answer.
    pub(crate) async fn postgres_with_bus(
        runtime: local_product::runtime::Store,
        stores: &crate::hosting::Readiness,
        events: Arc<dyn physician_bus::Bus>,
        mail: crate::gmail::Delivery,
        forbid_simulation: bool,
    ) -> Result<Arc<Self>, crate::hosting::SchemaError> {
        stores.check().await?;
        let source_setup = crate::source_setup::SourceSetupStore::postgres(
            runtime.connection().clone(),
            forbid_simulation,
        );
        // Authority reads are answered from memory between changes; the
        // database trigger feed invalidates them across replicas.
        let store = CachedAuthStore::postgres(stores.auth.clone());
        Ok(Self::configured_with_source_setup(
            Arc::new(store),
            Arc::new(stores.front_door.clone()),
            Arc::new(runtime),
            source_setup,
            events,
            mail,
        ))
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
    let now = now();
    let identity = auth
        .store
        .identity(&token, now)
        .await
        .map_err(|error| match error {
            SessionError::Invalid => Refused::NoSession,
            SessionError::Inactive => Refused::AccessRemoved,
        })?;
    if let Err(error) = auth.store.note_physician_activity(&token, now).await {
        tracing::warn!(?error, "physician session activity was not recorded");
    }
    Ok(identity)
}

/// Whether this path is one the guard protects: exactly the physician
/// routes, the one list the proxy also forwards, minus `/health` (which the
/// load balancer needs unauthenticated) and Hint Chrome connector routes
/// (which authenticate via constant-time `X-Coda-Pair` verification). The auth
/// routes establish the session and the static site renders it, so both stay open.
fn guarded(path: &str) -> bool {
    crate::physician_route(path)
        && path != "/health"
        && !crate::hint_bridge::is_connector_route(path)
}

/// Marker extension inserted exclusively on routers running behind required physician auth.
#[derive(Clone, Copy, Debug)]
pub(crate) struct AuthRequiredMarker;

/// The route guard: resolves the session before any physician read,
/// resolves workspace membership before any patient read or command dispatch,
/// filters multi-patient projections by the caller's practice and panel, and
/// terminates SSE streams when session or panel access is revoked.
pub(crate) async fn guard(
    State(auth): State<Arc<Auth>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    request.extensions_mut().insert(AuthRequiredMarker);
    request.extensions_mut().insert(Arc::clone(&auth.store));
    request
        .extensions_mut()
        .insert(Arc::clone(&auth.front_door));
    request.extensions_mut().insert(Arc::clone(&auth.patients));
    request
        .extensions_mut()
        .insert(Arc::clone(&auth.source_setup));
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
        match auth.store.try_membership(&identity, workspace).await {
            Ok(Membership::Allowed) => {
                let response = next.run(Request::from_parts(parts, body)).await;
                return enrich_allowed_workspace(response).await;
            }
            Ok(Membership::ClinicDemographic) => {
                return clinic_demographics(&auth, &identity, workspace).await;
            }
            Ok(Membership::Denied) => return not_found(),
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        }
    }

    if let Some(workspace) = request_workspace(&path, query.as_deref()) {
        match try_allows_workspace(&auth, &identity, &workspace).await {
            Ok(true) => {}
            // 404, not 403: a denied physician cannot read clinical projections
            // for a patient outside their panel.
            Ok(false) => return not_found(),
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        }
    }

    if path == "/patient-workspaces/events" {
        let watched_workspace = query_param(query.as_deref(), "patient_workspace_id");
        if let Some(ref ws) = watched_workspace {
            match try_allows_workspace(&auth, &identity, ws).await {
                Ok(true) => {}
                Ok(false) => return not_found(),
                Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
            }
        }
        let response = next.run(Request::from_parts(parts, body)).await;
        return guard_event_stream(response, auth, token, identity, watched_workspace).await;
    }

    next.run(Request::from_parts(parts, body)).await
}

/// The largest command body the guard reads to resolve its workspace.
const COMMAND_BODY_LIMIT: usize = 1_000_000;

/// The top-level `/patient-workspaces/{id}` path, excluding sub-routes like
/// `/chats/...` and the practice-wide `/patient-workspaces/events` stream.
fn direct_workspace(path: &str) -> Option<&str> {
    let rest = path.strip_prefix("/patient-workspaces/")?;
    (!rest.is_empty() && rest != "events" && !rest.contains('/')).then_some(rest)
}

fn query_param(query: Option<&str>, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    query?
        .split('&')
        .find_map(|pair| pair.strip_prefix(&prefix))
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

async fn enrich_allowed_workspace(response: Response) -> Response {
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
    }
    Json(value).into_response()
}

/// Basic demographic information for a patient in the caller's clinic who is
/// not on the caller's panel: identity demographics are preserved while
/// clinical synopsis, citations, chats, follow-ups, source handles, and
/// review cadence stay omitted.
async fn clinic_demographics(
    auth: &Auth,
    identity: &PhysicianIdentity,
    workspace: &str,
) -> Response {
    let Ok(practice) = auth.store.practice(&identity.practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let (clock, time_zone) = match practice {
        Some(record) => (record.clock, record.time_zone),
        None => (PracticeClock::RealTime, PracticeTimeZone::default()),
    };
    match read_patient(&*auth.patients, workspace).await {
        Ok(Some(patient)) => {
            let at = UNIX_EPOCH + Duration::from_secs(clock.now(now()));
            Json(patient.clinic_demographics_in_zone(&format_rfc3339(at), time_zone.as_str()))
                .into_response()
        }
        Ok(None) => not_found(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
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
    let still_allowed = async {
        if let Some(ws) = watched_workspace {
            return allows_workspace(auth, identity, ws).await;
        }
        let current_panel: HashSet<String> = auth.store.panel(identity).await.into_iter().collect();
        initial_panel.iter().all(|ws| current_panel.contains(ws))
    };
    let (resolved_identity, still_allowed) =
        tokio::join!(auth.store.identity(token, now()), still_allowed);
    resolved_identity.is_ok() && still_allowed
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
    if let Some(rest) = path
        .strip_prefix("/physician-state/drafts/")
        .or_else(|| path.strip_prefix("/physician-state/feedback/"))
    {
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

/// Verifies a sign-in token (or redeems a pending physician invitation link)
/// and returns the bearer session.
async fn verify(State(auth): State<Arc<Auth>>, body: axum::body::Bytes) -> Response {
    let Ok(body) = serde_json::from_slice::<VerifyBody>(&body) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let verified = match auth
        .store
        .complete_sign_in(&body.email, &body.token, now())
        .await
    {
        Ok(session) => Ok(session),
        Err(SignInError::Invalid) => {
            auth.store
                .redeem_invitation(&body.email, &body.token, now())
                .await
        }
        Err(SignInError::Inactive) => Err(SignInError::Inactive),
    };
    match verified {
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
        && auth.store.end_session(token).await.is_err()
    {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
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
            .unwrap_or_default()
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
    let Ok(owner) = auth.store.patient_practice(&patient_workspace_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    if owner.as_deref() != Some(caller.practice_id.as_str()) {
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
        Err(PanelError::Unavailable) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
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

#[derive(Deserialize)]
struct PatientActionBody {
    /// Practice patient workspace targeted by resend or cancel.
    patient_workspace_id: String,
}

fn practice_display_name(practice_id: &str) -> String {
    if practice_id == STAGING_PRACTICE_ID {
        "Roger Synthetic Practice".to_owned()
    } else {
        practice_id.to_owned()
    }
}

fn invite_error_response(error: front_door::InviteError) -> Response {
    let (status, detail) = match error {
        front_door::InviteError::Conflict => (
            StatusCode::CONFLICT,
            "That patient is already connected to this practice.".to_owned(),
        ),
        front_door::InviteError::Locked => (
            StatusCode::LOCKED,
            "This invitation is locked after 5 wrong birth dates. Cancel it before inviting again."
                .to_owned(),
        ),
        front_door::InviteError::NotFound => (
            StatusCode::NOT_FOUND,
            "No open invitation exists for this patient.".to_owned(),
        ),
        front_door::InviteError::InvalidInput(reason) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("Invalid invitation details: {reason}."),
        ),
        front_door::InviteError::Storage(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "Invitation storage is unavailable.".to_owned(),
        ),
    };
    (status, Json(json!({ "detail": detail }))).into_response()
}

async fn authorize_practice_patient<T: serde::de::DeserializeOwned>(
    auth: &Auth,
    request: Request<Body>,
    workspace_of: impl FnOnce(&T) -> &str,
) -> Result<(PhysicianIdentity, String, T), Box<Response>> {
    let token = bearer(&request);
    let Ok(caller) = resolve(auth, token).await else {
        return Err(Box::new(StatusCode::UNAUTHORIZED.into_response()));
    };
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), 64 * 1024).await else {
        return Err(Box::new(StatusCode::BAD_REQUEST.into_response()));
    };
    let Ok(body) = serde_json::from_slice::<T>(&bytes) else {
        return Err(Box::new(StatusCode::BAD_REQUEST.into_response()));
    };
    let Some(account) = auth.store.account_by_identity(&caller).await else {
        return Err(Box::new(StatusCode::FORBIDDEN.into_response()));
    };
    if !account.active {
        return Err(Box::new(StatusCode::FORBIDDEN.into_response()));
    }
    let workspace_id = workspace_of(&body);
    let Ok(owner) = auth.store.patient_practice(workspace_id).await else {
        return Err(Box::new(StatusCode::SERVICE_UNAVAILABLE.into_response()));
    };
    if owner.as_deref() != Some(caller.practice_id.as_str()) {
        return Err(Box::new(not_found()));
    }
    Ok((caller, account.display_name, body))
}

/// Invites one roster patient to Duet on the front door. Any active practice
/// member may invite a patient from that patient's page. The patient must be on
/// the caller's practice's roster: a cross-practice or unknown patient answers
/// 404, so the roster's existence stays unknown outside the practice. Proof
/// mode returns the link token and short code. Hosted mode mails them and
/// drops them from JSON.
async fn invite(State(auth): State<Arc<Auth>>, request: Request<Body>) -> Response {
    let (caller, physician_name, body) =
        match authorize_practice_patient::<InviteBody>(&auth, request, |body| {
            &body.patient_workspace_id
        })
        .await
        {
            Ok(ok) => ok,
            Err(response) => return *response,
        };
    let Ok(pulls) = auth
        .patients
        .pulls(std::slice::from_ref(&body.patient_workspace_id))
        .await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let expected_birth_date = pulls
        .get(&body.patient_workspace_id)
        .and_then(|list| list.iter().find_map(|pull| pull.patient.birth_date.clone()));
    let patient = RosterPatient {
        practice_id: caller.practice_id.clone(),
        patient_id: body.patient_workspace_id.clone(),
    };
    match auth
        .front_door
        .invite_patient(front_door::InvitePatientParams {
            patient,
            email: body.email.clone(),
            display_name: body.display_name.clone(),
            practice_name: practice_display_name(&caller.practice_id),
            physician_name,
            expected_birth_date,
            now: now(),
        })
        .await
    {
        Ok(issued) => deliver_invite(&auth, &caller, &body.patient_workspace_id, issued).await,
        Err(error) => invite_error_response(error),
    }
}

/// Re-sends the open invitation for one roster patient unless it is locked after
/// repeated wrong birth-date entries (`COD-49`). Keeps all prior credentials open.
async fn resend_invite(State(auth): State<Arc<Auth>>, request: Request<Body>) -> Response {
    let (caller, _, body) =
        match authorize_practice_patient::<PatientActionBody>(&auth, request, |body| {
            &body.patient_workspace_id
        })
        .await
        {
            Ok(ok) => ok,
            Err(response) => return *response,
        };
    let patient = RosterPatient {
        practice_id: caller.practice_id.clone(),
        patient_id: body.patient_workspace_id.clone(),
    };
    match auth.front_door.resend_invitation(&patient, now()).await {
        Ok(issued) => deliver_invite(&auth, &caller, &body.patient_workspace_id, issued).await,
        Err(error) => invite_error_response(error),
    }
}

/// Cancels the open invitation for one roster patient (`Rule 5`), invalidating
/// all prior invitation links and short codes and returning the roster to `not_invited`.
async fn cancel_invite(State(auth): State<Arc<Auth>>, request: Request<Body>) -> Response {
    let (caller, _, body) =
        match authorize_practice_patient::<PatientActionBody>(&auth, request, |body| {
            &body.patient_workspace_id
        })
        .await
        {
            Ok(ok) => ok,
            Err(response) => return *response,
        };
    let patient = RosterPatient {
        practice_id: caller.practice_id.clone(),
        patient_id: body.patient_workspace_id.clone(),
    };
    match auth.front_door.cancel_invitation(&patient).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "practice_id": caller.practice_id,
                "patient_workspace_id": body.patient_workspace_id,
                "status": "not_invited",
            })),
        )
            .into_response(),
        Err(error) => invite_error_response(error),
    }
}

#[derive(Deserialize)]
struct ColleagueInviteBody {
    /// Colleague physician's work email address.
    email: String,
    /// Colleague physician's display name.
    display_name: String,
}

/// Invites a colleague physician into the caller's practice (`Sign-in.Journey.2`).
/// Any active practice member may invite non-admin colleagues.
async fn invite_colleague(State(auth): State<Arc<Auth>>, request: Request<Body>) -> Response {
    let token = bearer(&request);
    let Ok(caller) = resolve(&auth, token).await else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let caller_active = auth
        .store
        .account_by_identity(&caller)
        .await
        .is_some_and(|account| account.active);
    if !caller_active {
        return StatusCode::FORBIDDEN.into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), 64 * 1024).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(body) = serde_json::from_slice::<ColleagueInviteBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let email = body.email.trim().to_lowercase();
    let display_name = body.display_name.trim().to_owned();
    let valid_email = email
        .split_once('@')
        .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'))
        && !email.chars().any(char::is_whitespace);
    if !valid_email || display_name.is_empty() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "detail": "Invitation requires a valid email address and display name.",
                "code": "invalid_invitation",
            })),
        )
            .into_response();
    }
    if auth.store.account_by_email(&email).await.is_some() {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": "An account with that email address already exists.",
                "code": "account_exists",
            })),
        )
            .into_response();
    }
    let invite_token = match auth
        .store
        .invite(&caller.practice_id, &email, &display_name, false, now())
        .await
    {
        Ok(token) => token,
        Err(SignInError::Invalid | SignInError::Inactive) => {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "detail": "That email address already has a conflicting invitation.",
                    "code": "invitation_conflict",
                })),
            )
                .into_response();
        }
    };
    match &auth.mail {
        crate::gmail::Delivery::Proof => {
            auth.delivery
                .lock()
                .expect("delivery")
                .push(invite_token.clone());
            let mut sign_in_path = String::from("/?code=");
            for byte in invite_token.bytes() {
                if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
                    sign_in_path.push(char::from(byte));
                } else {
                    use std::fmt::Write as _;
                    let _ = write!(sign_in_path, "%{byte:02X}");
                }
            }
            sign_in_path.push_str("&email=");
            for byte in email.bytes() {
                if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
                    sign_in_path.push(char::from(byte));
                } else {
                    use std::fmt::Write as _;
                    let _ = write!(sign_in_path, "%{byte:02X}");
                }
            }
            (
                StatusCode::OK,
                Json(json!({
                    "practice_id": caller.practice_id,
                    "email": email,
                    "display_name": display_name,
                    "role": "physician",
                    "status": "invited",
                    "delivery": "stored",
                    "token": invite_token,
                    "sign_in_path": sign_in_path,
                })),
            )
                .into_response()
        }
        crate::gmail::Delivery::Hosted(sender) => {
            if let Err(error) = sender.send_physician_invite(&email, &invite_token).await {
                tracing::warn!(%error, "colleague invitation mail was not sent");
                return (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({
                        "detail": "The colleague invitation email could not be sent.",
                        "code": "mail_delivery_failed",
                    })),
                )
                    .into_response();
            }
            (
                StatusCode::OK,
                Json(json!({
                    "practice_id": caller.practice_id,
                    "email": email,
                    "display_name": display_name,
                    "role": "physician",
                    "status": "invited",
                    "delivery": "mailed",
                })),
            )
                .into_response()
        }
    }
}

async fn deliver_invite(
    auth: &Auth,
    caller: &PhysicianIdentity,
    patient_workspace_id: &str,
    issued: front_door::IssuedInvitation,
) -> Response {
    let invited = json!({
        "practice_id": caller.practice_id,
        "patient_workspace_id": patient_workspace_id,
        "email": issued.invitation.email,
        "display_name": issued.invitation.display_name,
        "status": "invited",
        "delivery_failed": false,
    });
    match &auth.mail {
        crate::gmail::Delivery::Proof => {
            let mut invited = invited;
            invited["token"] = json!(issued.link_token);
            invited["short_code"] = json!(issued.short_code);
            (StatusCode::OK, Json(invited)).into_response()
        }
        crate::gmail::Delivery::Hosted(sender) => {
            if let Err(error) = sender
                .send_invite(
                    &issued.invitation.email,
                    &issued.link_token,
                    &issued.short_code,
                )
                .await
            {
                tracing::warn!(%error, "invitation mail was not sent");
                if let Err(mark_err) = auth
                    .front_door
                    .mark_invitation_delivery_failed(
                        &issued.invitation.patient,
                        "The invitation email could not be sent.",
                        now(),
                    )
                    .await
                {
                    tracing::warn!(%mark_err, "could not record invitation mail delivery failure");
                }
                return (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({
                        "practice_id": caller.practice_id,
                        "patient_workspace_id": patient_workspace_id,
                        "email": issued.invitation.email,
                        "display_name": issued.invitation.display_name,
                        "status": "invited",
                        "delivery_failed": true,
                        "detail": "The invitation email could not be sent.",
                    })),
                )
                    .into_response();
            }
            (StatusCode::OK, Json(invited)).into_response()
        }
    }
}

/// The workspace membership check a handler uses, propagating store
/// outages instead of masquerading them as absent patients.
pub(crate) async fn try_allows_workspace(
    auth: &Auth,
    identity: &PhysicianIdentity,
    patient_workspace_id: &str,
) -> Result<bool, StoreUnavailable> {
    Ok(auth
        .store
        .try_membership(identity, patient_workspace_id)
        .await?
        == Membership::Allowed)
}

/// Checks each candidate patient's panel access individually. A patient whose
/// access check fails with [`StoreUnavailable`] is skipped without hiding the
/// remaining patients, setting `may_be_missing_someone` (`A1.3`).
pub(crate) async fn check_panel_candidates(
    store: &dyn AuthStore,
    identity: &PhysicianIdentity,
    candidates: &[String],
) -> (std::collections::BTreeSet<String>, bool) {
    let mut allowed = std::collections::BTreeSet::new();
    let mut may_be_missing_someone = false;
    for ws in candidates {
        match store.try_membership(identity, ws).await {
            Ok(Membership::Allowed) => {
                allowed.insert(ws.clone());
            }
            Ok(Membership::ClinicDemographic | Membership::Denied) => {}
            Err(_) => {
                may_be_missing_someone = true;
            }
        }
    }
    (allowed, may_be_missing_someone)
}

/// The workspace membership check a handler uses: a denied or
/// missing membership answers 404 so clinical projections stay panel-scoped.
pub(crate) async fn allows_workspace(
    auth: &Auth,
    identity: &PhysicianIdentity,
    patient_workspace_id: &str,
) -> bool {
    auth.store.membership(identity, patient_workspace_id).await == Membership::Allowed
}

/// The 404 a denied workspace check returns. It matches what the site
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
        .route("/auth/invitations/resend", post(resend_invite))
        .route("/auth/invitations/cancel", post(cancel_invite))
        .route("/auth/colleagues/invitations", post(invite_colleague))
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
    let auth = Auth::over_fixture(store, Arc::new(MemoryFrontDoorStore::new())).await;
    required_router(base, auth)
}

async fn inject_auth_store(
    State(auth): State<Arc<Auth>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    request.extensions_mut().insert(Arc::clone(&auth.store));
    request
        .extensions_mut()
        .insert(Arc::clone(&auth.front_door));
    request.extensions_mut().insert(Arc::clone(&auth.patients));
    request
        .extensions_mut()
        .insert(Arc::clone(&auth.source_setup));
    next.run(request).await
}

/// Builds the router with auth required over the in-memory store.
///
/// `PHYSICIAN_AUTH=required` guards every physician route of `base`. Any other
/// value serves `base` with the auth routes still present. Workspace events
/// stay on an in-process bus. The process entrypoint uses
/// [`authed_router_configured`] for Postgres and Redis. Sign-in mail comes from
/// the environment.
pub async fn authed_router(base: axum::Router) -> axum::Router {
    let required = matches!(auth_mode(), AuthMode::Required);
    let mail = crate::gmail::Delivery::from_env(|key| std::env::var(key).ok());
    let staff = crate::hosting::StaffSetupAuth::from_env(|key| std::env::var(key).ok())
        .ok()
        .flatten();
    let forbid_simulation = std::env::var("CODA_ENVIRONMENT")
        .ok()
        .is_some_and(|env| env.trim() == "prd");
    authed_router_configured(
        base,
        Arc::new(physician_bus::LocalBus::new(64)),
        MemoryPatients::fixture(),
        None,
        required,
        mail,
        None,
        staff,
        forbid_simulation,
    )
    .await
}

/// Builds the router with auth on `base`, publishing panel changes on `bus`.
///
/// `required` guards every physician route through one store — Postgres when
/// `postgres` holds the process's runtime store and the stores over its pool,
/// memory otherwise. Any other value serves `base` with the auth routes still
/// present. Neither store is migrated. Without a database the practice holds
/// `memory_patients`.
///
/// `rehearsal` adds `POST /auth/rehearsal` ([`crate::rehearsal::sign_in_routes`]),
/// and only behind required sign-in: with open auth there is no session to mint.
#[expect(
    clippy::too_many_arguments,
    reason = "process router composition wiring"
)]
pub(crate) async fn authed_router_configured(
    base: axum::Router,
    bus: Arc<dyn physician_bus::Bus>,
    memory_patients: Arc<MemoryPatients>,
    postgres: Option<(local_product::runtime::Store, &crate::hosting::Readiness)>,
    required: bool,
    mail: crate::gmail::Delivery,
    rehearsal: Option<crate::hosting::RehearsalSignIn>,
    staff: Option<crate::hosting::StaffSetupAuth>,
    forbid_simulation: bool,
) -> axum::Router {
    let mode = if required {
        AuthMode::Required
    } else {
        AuthMode::Open
    };
    let auth = match (&mode, postgres) {
        (AuthMode::Required, Some((runtime, stores))) => {
            match Auth::postgres_with_bus(runtime, stores, bus, mail, forbid_simulation).await {
                Ok(auth) => auth,
                Err(error) => {
                    tracing::error!(
                        error.type = error.failure().error_type(),
                        db.response.status_code =
                            local_product::runtime::sqlstate(error.cause()).as_deref(),
                        error = %error,
                        error.cause = %error.cause(),
                        "physician auth store unavailable"
                    );
                    std::process::exit(1);
                }
            }
        }
        _ => Auth::memory_with(memory_patients, bus, mail, forbid_simulation).await,
    };
    match mode {
        AuthMode::Required => {
            let base = match rehearsal {
                Some(config) => {
                    base.merge(crate::rehearsal::sign_in_routes(Arc::clone(&auth), config))
                }
                None => base,
            };
            let base = match staff {
                Some(staff_auth) => {
                    base.merge(crate::staff_setup::routes(Arc::clone(&auth), staff_auth))
                }
                None => base,
            };
            required_router(base, auth)
        }
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
/// migrated, over the fixture patients. The Postgres integration tests use it
/// to prove the invite → redeem → link journey the deployment wiring builds.
pub async fn authed_router_on(
    base: axum::Router,
    store: Arc<dyn AuthStore>,
    front_door: Arc<dyn FrontDoorStore>,
) -> axum::Router {
    required_router(base, Auth::over_fixture(store, front_door).await)
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Method, Request, StatusCode};
    use base64::Engine;
    use physician_auth::{Membership, MemoryAuthStore, PhysicianIdentity};
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

    /// The fixture site behind a required session on one memory store,
    /// plus the concrete stores for seeding a second physician's access and
    /// redeeming an invitation the practice endpoint minted.
    async fn authed_practice() -> (
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
        let auth = Auth::over_fixture(
            Arc::clone(&store) as Arc<dyn AuthStore>,
            Arc::clone(&front_door) as Arc<dyn FrontDoorStore>,
        )
        .await;
        (
            required_router(crate::fixture_site(), auth),
            store,
            front_door,
        )
    }

    #[tokio::test]
    async fn active_staging_admins_see_the_sample_patients() {
        let store = Arc::new(MemoryAuthStore::new());
        store.seed_account(
            STAGING_PRACTICE_ID,
            "ops-admin",
            "ops@codahealth.ai",
            "Ops",
            true,
        );
        store.seed_account(
            STAGING_PRACTICE_ID,
            "eli-admin",
            "eli@codahealth.ai",
            "Eli",
            true,
        );
        store.seed_account(
            STAGING_PRACTICE_ID,
            "member",
            "member@codahealth.ai",
            "Member",
            false,
        );
        store.seed_account(
            STAGING_PRACTICE_ID,
            "former-admin",
            "former@codahealth.ai",
            "Former",
            true,
        );
        assert!(store.deactivate("former@codahealth.ai").await);
        let _auth = Auth::over_fixture(
            Arc::clone(&store) as Arc<dyn AuthStore>,
            Arc::new(MemoryFrontDoorStore::new()),
        )
        .await;
        let mut expected: Vec<String> = crate::WORKSPACE_IDS
            .iter()
            .map(|id| (*id).to_owned())
            .collect();
        expected.sort();
        for physician_id in ["ops-admin", "eli-admin"] {
            let panel = store
                .panel(&PhysicianIdentity {
                    practice_id: STAGING_PRACTICE_ID.to_owned(),
                    physician_id: physician_id.to_owned(),
                })
                .await;
            assert_eq!(panel, expected, "{physician_id}");
        }
        for physician_id in ["member", "former-admin"] {
            let panel = store
                .panel(&PhysicianIdentity {
                    practice_id: STAGING_PRACTICE_ID.to_owned(),
                    physician_id: physician_id.to_owned(),
                })
                .await;
            assert!(
                panel.is_empty(),
                "{physician_id} should not receive the sample panel: {panel:?}"
            );
        }
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
        let (app, _, _) = authed_practice().await;
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
        let (app, _, _) = authed_practice().await;
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
        let (app, _, _) = authed_practice().await;
        let (status, body) = send(&app, Method::GET.as_str(), "/health", None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("\"status\":\"ok\""), "{body}");
        let (status, page) = send(&app, Method::GET.as_str(), "/", None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert!(page.contains(r#"<main id="main"></main>"#), "page: {page}");
    }

    #[tokio::test]
    async fn whoami_names_the_session_physician() {
        let (app, _, _) = authed_practice().await;
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
        let (app, store, _) = authed_practice().await;
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
        let (app, store, _) = authed_practice().await;
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
        let (app, _, _) = authed_practice().await;
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
        let (app, _, _) = authed_practice().await;
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
        let (app, _, _) = authed_practice().await;
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
        let (app, store, _) = authed_practice().await;
        let colleague = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: "colleague".to_owned(),
        };
        store
            .set_membership(&colleague, crate::WORKSPACE_IDS[0], Membership::Allowed)
            .await
            .unwrap();
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

        // The granted workspace reads in full; practice-level lists still read.
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{}", crate::WORKSPACE_IDS[0]),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert!(body.contains("synchronized_at"), "{body}");
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
        let (app, store, _) = authed_practice().await;
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
        assert!(ws_body.contains("synchronized_at"), "{ws_body}");

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
    async fn a_patient_exists_once_registered_to_the_practice() {
        let (app, store, _) = authed_practice().await;
        let session = session_for(&app, STAGING_EMAIL).await;
        let physician = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: "staging-physician".to_owned(),
        };
        store
            .set_membership(&physician, "pw-chart", Membership::Allowed)
            .await
            .unwrap();
        store
            .set_membership(&physician, "pw-elsewhere", Membership::Allowed)
            .await
            .unwrap();
        store
            .register_patient("northside-clinic", "pw-elsewhere")
            .await
            .unwrap();
        let snapshot = async |workspace: &str| {
            let path = format!("/patient-workspaces/{workspace}/chats/none/snapshot");
            send(&app, Method::GET.as_str(), &path, Some(&session), None)
                .await
                .0
        };

        assert_eq!(snapshot("pw-chart").await, StatusCode::NOT_FOUND);
        assert_eq!(snapshot("pw-elsewhere").await, StatusCode::NOT_FOUND);
        store
            .register_patient(STAGING_PRACTICE_ID, "pw-chart")
            .await
            .unwrap();
        assert_eq!(snapshot("pw-chart").await, StatusCode::OK);
    }

    #[tokio::test]
    async fn clinical_now_follows_the_clock_each_practice_was_created_with() {
        const FIXED: &str = "2026-08-27T16:00:00.000Z";
        let (app, store, _) = authed_practice().await;
        let mary = crate::WORKSPACE_IDS[4];
        store.seed_account(
            "northside-clinic",
            "north-doc",
            "doc@northside.test",
            "Northside Doc",
            false,
        );
        let colleague = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: "colleague".to_owned(),
        };
        store
            .set_membership(&colleague, mary, Membership::Allowed)
            .await
            .unwrap();
        let synthetic = session_for(&app, COLLEAGUE_EMAIL).await;
        let real = session_for(&app, "doc@northside.test").await;
        let current_time = async |path: &str, session: &str| {
            let (status, body) = send(&app, Method::GET.as_str(), path, Some(session), None).await;
            assert_eq!(status, StatusCode::OK, "{path}: {body}");
            let value: Value = serde_json::from_str(&body).unwrap();
            value["current_time"].as_str().unwrap().to_owned()
        };

        assert_eq!(current_time("/practice-day", &synthetic).await, FIXED);
        let workspace = format!("/patient-workspaces/{mary}");
        assert_eq!(current_time(&workspace, &synthetic).await, FIXED);

        let before = UNIX_EPOCH + Duration::from_secs(now());
        let before = crate::timestamp::format_rfc3339(before);
        assert!(current_time("/practice-day", &real).await >= before);
    }

    #[tokio::test]
    async fn projection_lists_filter_by_practice_and_panel_without_staging_practice_assumptions() {
        let (app, store, _) = authed_practice().await;
        // Move Jake Smith (WORKSPACE_IDS[1]) to a second clinic ("northside-clinic")
        // with its own admin and physician, proving no STAGING_PRACTICE_ID hard-coding.
        let jake = crate::WORKSPACE_IDS[1];
        let mary = crate::WORKSPACE_IDS[4];
        store.seed_patient("northside-clinic", jake);
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
            .await
            .unwrap();

        let colleague_session = session_for(&app, COLLEAGUE_EMAIL).await;
        let north_admin_session = session_for(&app, "admin@northside.test").await;
        let north_doc_session = session_for(&app, "doc@northside.test").await;

        // Colleague in STAGING_PRACTICE_ID sees only Mary on /patient-workspaces and /today?patient_workspace_id=...
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
        assert!(north_full.contains("synchronized_at"), "{north_full}");
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
        let auth = Auth::over_fixture(
            Arc::clone(&store) as Arc<dyn AuthStore>,
            Arc::new(MemoryFrontDoorStore::new()),
        )
        .await;
        let bus: Arc<dyn physician_bus::Bus> = Arc::new(physician_bus::LocalBus::new(64));
        let live_base = axum::Router::new().route(
            "/patient-workspaces/events",
            get(crate::live::events).with_state(Arc::clone(&bus)),
        );
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
            .await
            .unwrap();

        let colleague_session = session_for(&app, COLLEAGUE_EMAIL).await;
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

        for workspace in [other_ws, allowed_ws] {
            bus.publish(physician_bus::Event::WorkspaceChanged {
                workspace_id: workspace.to_owned(),
            })
            .await
            .unwrap();
        }

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
            .await
            .unwrap();
        let closed = tokio::time::timeout(Duration::from_secs(2), body_stream.next())
            .await
            .expect("SSE termination timeout");
        assert!(
            closed.is_none(),
            "SSE stream must terminate when panel membership is revoked"
        );
    }

    #[tokio::test]
    async fn a_physician_cannot_change_review_cadence() {
        let (app, _store, _) = authed_practice().await;
        let session = session_for(&app, STAGING_EMAIL).await;
        let workspace = crate::WORKSPACE_IDS[0];

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
        assert!(ws_json.get("payment_status").is_none());

        // The site has no cadence route, and a physician command cannot set it.
        for method in [Method::GET, Method::PUT] {
            let (status, _) = send(
                &app,
                method.as_str(),
                &format!("/patient-workspaces/{workspace}/cadence"),
                Some(&session),
                Some(json!({ "cadence": "weekly" }).to_string()),
            )
            .await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{method}");
        }

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
    }

    #[tokio::test]
    async fn a_command_into_a_denied_workspace_returns_404() {
        let (app, store, _) = authed_practice().await;
        let colleague = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: "colleague".to_owned(),
        };
        store
            .set_membership(&colleague, crate::WORKSPACE_IDS[0], Membership::Allowed)
            .await
            .unwrap();
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
        // fixture site answers its own rejection, not the guard's 404.
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

    /// Verifies DOB and accepts via Share at the front door behind the practice server.
    async fn accept_with_dob(
        front_door: &MemoryFrontDoorStore,
        email: &str,
        credential: &front_door::InvitationCredential,
        birth_date: &str,
    ) -> bool {
        if front_door
            .verify_invitation_dob(credential, birth_date, 1_760_000_000)
            .await
            .is_err()
        {
            return false;
        }
        front_door
            .accept_invitation_share(None, Some(email), credential, birth_date, 1_760_000_000)
            .await
            .is_ok()
    }

    #[tokio::test]
    async fn an_admin_invitation_redeems_once_at_the_front_door() {
        let (app, _, front_door) = authed_practice().await;
        let session = session_for(&app, STAGING_EMAIL).await;
        let workspace = crate::WORKSPACE_IDS[0];
        let (status, body) =
            invite_patient(&app, Some(&session), "luna@example.test", workspace).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let parsed: Value = serde_json::from_str(&body).expect("invitation body");
        let token = parsed["token"].as_str().expect("invite token").to_owned();
        let short_code = parsed["short_code"]
            .as_str()
            .expect("invite short_code")
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
        // The patient verifies chart DOB (1989-11-08) and accepts once via Share.
        let cred = front_door::InvitationCredential::LinkToken(token.clone());
        assert!(accept_with_dob(&front_door, "luna@example.test", &cred, "1989-11-08").await);
        let (account_id, connections) = {
            let account = front_door
                .account_by_email("luna@example.test")
                .await
                .expect("account");
            (account.id, account.connections)
        };
        assert_eq!(connections.len(), 1);
        assert_eq!(
            connections[0].patient,
            RosterPatient {
                practice_id: STAGING_PRACTICE_ID.to_owned(),
                patient_id: workspace.to_owned(),
            }
        );
        assert_eq!(
            front_door.connections(account_id).await.unwrap()[0].patient,
            RosterPatient {
                practice_id: STAGING_PRACTICE_ID.to_owned(),
                patient_id: workspace.to_owned(),
            }
        );
        // Neither the same token nor the paired short code accepts again.
        assert!(
            !accept_with_dob(&front_door, "luna@example.test", &cred, "1989-11-08").await,
            "an invitation redeems once"
        );
        let code_cred = front_door::InvitationCredential::ShortCode(short_code);
        assert!(
            !accept_with_dob(&front_door, "luna@example.test", &code_cred, "1989-11-08").await,
            "paired short code also closes on accept"
        );
    }

    #[tokio::test]
    async fn resend_cancel_and_lock_enforce_practice_scope_and_keep_invited_state() {
        let (app, _, front_door) = authed_practice().await;
        let session = session_for(&app, STAGING_EMAIL).await;
        let workspace = crate::WORKSPACE_IDS[0];
        let (status, body) =
            invite_patient(&app, Some(&session), "luna@example.test", workspace).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let v1: Value = serde_json::from_str(&body).unwrap();
        let tok1 = v1["token"].as_str().unwrap().to_owned();

        // Resend mints new credentials while keeping old token open (Rule 4).
        let (status, resend_body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/invitations/resend",
            Some(&session),
            Some(json!({ "patient_workspace_id": workspace }).to_string()),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{resend_body}");
        let v2: Value = serde_json::from_str(&resend_body).unwrap();
        assert_ne!(v2["token"], v1["token"]);
        assert_ne!(v2["short_code"], v1["short_code"]);
        assert!(
            front_door
                .inspect_invitation(&front_door::InvitationCredential::LinkToken(tok1.clone()))
                .await
                .is_ok()
        );

        // Cross-practice / unknown workspace answers 404.
        let (unknown_status, _) = send(
            &app,
            Method::POST.as_str(),
            "/auth/invitations/cancel",
            Some(&session),
            Some(json!({ "patient_workspace_id": "pw-unknown" }).to_string()),
        )
        .await;
        assert_eq!(unknown_status, StatusCode::NOT_FOUND);

        // 5 wrong DOBs lock the invitation, refusing Resend with 423 LOCKED (COD-49).
        let tok2_cred =
            front_door::InvitationCredential::LinkToken(v2["token"].as_str().unwrap().to_owned());
        for _ in 0..5 {
            let _ = front_door
                .verify_invitation_dob(&tok2_cred, "1901-01-01", now())
                .await;
        }
        let (locked_status, _) = send(
            &app,
            Method::POST.as_str(),
            "/auth/invitations/resend",
            Some(&session),
            Some(json!({ "patient_workspace_id": workspace }).to_string()),
        )
        .await;
        assert_eq!(locked_status, StatusCode::LOCKED);

        // Cancel invalidates all credentials and clears lock so re-invite succeeds.
        let (cancel_status, cancel_body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/invitations/cancel",
            Some(&session),
            Some(json!({ "patient_workspace_id": workspace }).to_string()),
        )
        .await;
        assert_eq!(cancel_status, StatusCode::OK, "{cancel_body}");
        assert!(
            front_door
                .inspect_invitation(&front_door::InvitationCredential::LinkToken(tok1))
                .await
                .is_err()
        );
        let (reinvite_status, _) =
            invite_patient(&app, Some(&session), "luna@example.test", workspace).await;
        assert_eq!(reinvite_status, StatusCode::OK);
    }

    #[tokio::test]
    async fn a_cross_practice_or_unknown_patient_never_gets_invited() {
        let (app, store, front_door) = authed_practice().await;
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
    async fn any_active_practice_member_can_invite_and_a_conflict_answers_409() {
        let (app, _, front_door) = authed_practice().await;
        let colleague = session_for(&app, COLLEAGUE_EMAIL).await;
        let (status, body) = invite_patient(
            &app,
            Some(&colleague),
            "luna@example.test",
            crate::WORKSPACE_IDS[0],
        )
        .await;
        assert_eq!(
            status,
            StatusCode::OK,
            "active non-admin practice member must be allowed to invite: {body}"
        );
        assert!(front_door.invitation("luna@example.test").await.is_some());

        // An existing Duet account can be invited for another roster patient without conflict,
        // whereas inviting an already-Connected roster patient answers 409 Conflict.
        let session = session_for(&app, STAGING_EMAIL).await;
        front_door.seed_linked_account(
            &RosterPatient {
                practice_id: STAGING_PRACTICE_ID.to_owned(),
                patient_id: crate::WORKSPACE_IDS[1].to_owned(),
            },
            "linked@example.test",
            "Already Linked",
        );
        let second_workspace = RosterPatient {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            patient_id: crate::WORKSPACE_IDS[0].to_owned(),
        };
        let (ok_status, ok_body) = invite_patient(
            &app,
            Some(&session),
            "linked@example.test",
            crate::WORKSPACE_IDS[0],
        )
        .await;
        assert_eq!(
            ok_status,
            StatusCode::OK,
            "existing Duet account may connect to another practice/patient: {ok_body}"
        );
        assert!(front_door.invitation_for(&second_workspace).await.is_some());

        let (status, body) = invite_patient(
            &app,
            Some(&session),
            "linked@example.test",
            crate::WORKSPACE_IDS[1],
        )
        .await;
        assert_eq!(status, StatusCode::CONFLICT, "{body}");
        assert!(body.contains("already connected"), "{body}");
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
        register_fixture_patients(&*store).await;
        let auth = Auth::configured(
            Arc::clone(&store) as Arc<dyn AuthStore>,
            Arc::clone(&front_door) as Arc<dyn FrontDoorStore>,
            MemoryPatients::fixture(),
            Arc::new(physician_bus::LocalBus::new(64)),
            crate::gmail::Delivery::Hosted(Box::new(sender)),
        );
        (
            required_router(crate::fixture_site(), auth),
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
        assert!(!message.contains("expires"), "{message}");
        let token = query_value(&message, "token");
        assert!(
            message.contains(&format!("https://physician.example/invite?token={token}")),
            "{message}"
        );
        assert!(
            message.contains("Or enter this code in Duet:"),
            "hosted mail must carry the short code too: {message}"
        );
        assert!(
            !body.contains(token),
            "the response included the mailed token: {body}"
        );
        let cred = front_door::InvitationCredential::LinkToken(token.to_owned());
        assert!(
            front_door
                .verify_invitation_dob(&cred, "1989-11-08", now())
                .await
                .is_ok(),
            "the mailed token should unlock with the chart DOB"
        );
        assert!(
            front_door
                .accept_invitation_share(
                    None,
                    Some("luna@example.test"),
                    &cred,
                    "1989-11-08",
                    now()
                )
                .await
                .is_ok(),
            "the mailed invite token should accept via Share once"
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
                    ..gmail::Endpoints::google()
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

    #[tokio::test]
    async fn active_practice_member_can_invite_non_admin_colleague_and_sign_in() {
        let (app, store, _) = authed_practice().await;
        let member = session_for(&app, COLLEAGUE_EMAIL).await;
        let (status, body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/colleagues/invitations",
            Some(&member),
            Some(
                json!({
                    "email": "new-doc@codahealth.test",
                    "display_name": "Dr. New Colleague",
                })
                .to_string(),
            ),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let value: Value = serde_json::from_str(&body).expect("colleague invite body");
        assert_eq!(value["practice_id"], STAGING_PRACTICE_ID);
        assert_eq!(value["email"], "new-doc@codahealth.test");
        assert_eq!(value["display_name"], "Dr. New Colleague");
        assert_eq!(value["role"], "physician");
        assert_eq!(value["status"], "invited");
        let token = value["token"].as_str().expect("proof mode token");
        assert!(
            store
                .account_by_email("new-doc@codahealth.test")
                .await
                .is_none(),
            "pending invitation must not create an active account until redeemed"
        );

        // Colleague can verify their invitation token into a working session.
        let (verify_status, verify_body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/verify",
            None,
            Some(
                json!({
                    "email": "new-doc@codahealth.test",
                    "token": token,
                })
                .to_string(),
            ),
        )
        .await;
        assert_eq!(verify_status, StatusCode::OK, "{verify_body}");
        let invited = store
            .account_by_email("new-doc@codahealth.test")
            .await
            .expect("created colleague account after redeem");
        assert_eq!(invited.identity.practice_id, STAGING_PRACTICE_ID);
        assert!(!invited.admin, "invited colleague must be non-admin");
        assert!(invited.active);
        let verified: Value = serde_json::from_str(&verify_body).expect("verify body");
        let new_session = verified["session"].as_str().expect("colleague session");
        let (who_status, who_body) = send(
            &app,
            Method::GET.as_str(),
            "/auth/whoami",
            Some(new_session),
            None,
        )
        .await;
        assert_eq!(who_status, StatusCode::OK, "{who_body}");
        let who: Value = serde_json::from_str(&who_body).expect("whoami body");
        assert_eq!(who["admin"], false);

        // Duplicate colleague email is refused with 409 Conflict.
        let (dup_status, _) = send(
            &app,
            Method::POST.as_str(),
            "/auth/colleagues/invitations",
            Some(&member),
            Some(
                json!({
                    "email": "new-doc@codahealth.test",
                    "display_name": "Duplicate",
                })
                .to_string(),
            ),
        )
        .await;
        assert_eq!(dup_status, StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn hosted_colleague_invite_mails_link_and_omits_token() {
        let (sender, fake) = gmail_sender(false).await;
        let (app, _, session) = hosted_practice(sender).await;
        let (status, body) = send(
            &app,
            Method::POST.as_str(),
            "/auth/colleagues/invitations",
            Some(&session),
            Some(
                json!({
                    "email": "colleague-hosted@codahealth.test",
                    "display_name": "Dr. Hosted Colleague",
                })
                .to_string(),
            ),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let value: Value = serde_json::from_str(&body).expect("hosted colleague invite");
        assert!(value.get("token").is_none(), "{body}");
        assert_eq!(value["delivery"], "mailed");
        let sent = fake.sent.lock().expect("sent");
        assert_eq!(sent.len(), 1);
        let message = decode_gmail_raw(&sent[0]);
        assert!(
            message.contains("To: colleague-hosted@codahealth.test"),
            "{message}"
        );
        let code = query_value(&message, "code");
        assert!(!body.contains(code), "{body}");
    }

    struct PartialFailAuthStore {
        inner: Arc<MemoryAuthStore>,
        failing_workspace: String,
    }

    #[async_trait::async_trait]
    impl AuthStore for PartialFailAuthStore {
        async fn begin_sign_in(
            &self,
            email: &str,
            now: u64,
        ) -> Result<String, physician_auth::SignInError> {
            self.inner.begin_sign_in(email, now).await
        }
        async fn complete_sign_in(
            &self,
            email: &str,
            token: &str,
            now: u64,
        ) -> Result<(PhysicianIdentity, String), physician_auth::SignInError> {
            self.inner.complete_sign_in(email, token, now).await
        }
        async fn mint_fixed_session(
            &self,
            identity: &PhysicianIdentity,
            now: u64,
            ends_at: u64,
        ) -> Result<String, physician_auth::SignInError> {
            self.inner.mint_fixed_session(identity, now, ends_at).await
        }
        async fn identity(
            &self,
            token: &str,
            now: u64,
        ) -> Result<PhysicianIdentity, physician_auth::SessionError> {
            self.inner.identity(token, now).await
        }
        async fn verified_session(
            &self,
            token: &str,
            now: u64,
        ) -> Result<physician_auth::Session, physician_auth::SessionError> {
            self.inner.verified_session(token, now).await
        }
        async fn note_physician_activity(
            &self,
            token: &str,
            now: u64,
        ) -> Result<physician_auth::SessionActivity, physician_auth::StoreUnavailable> {
            self.inner.note_physician_activity(token, now).await
        }
        async fn publish_authorized(
            &self,
            token: &str,
            identity: &PhysicianIdentity,
            workspace: &str,
            now: u64,
            publish: &mut (dyn FnMut() -> Result<(), String> + Send),
        ) -> Result<(), String> {
            self.inner
                .publish_authorized(token, identity, workspace, now, publish)
                .await
        }
        async fn end_session(&self, token: &str) -> Result<(), physician_auth::StoreUnavailable> {
            self.inner.end_session(token).await
        }
        async fn account_by_email(&self, email: &str) -> Option<physician_auth::Account> {
            self.inner.account_by_email(email).await
        }
        async fn try_account_by_identity(
            &self,
            identity: &PhysicianIdentity,
        ) -> Result<Option<physician_auth::Account>, physician_auth::StoreUnavailable> {
            self.inner.try_account_by_identity(identity).await
        }
        async fn account_by_identity(
            &self,
            identity: &PhysicianIdentity,
        ) -> Option<physician_auth::Account> {
            self.inner.account_by_identity(identity).await
        }
        async fn try_membership(
            &self,
            identity: &PhysicianIdentity,
            patient_workspace_id: &str,
        ) -> Result<Membership, physician_auth::StoreUnavailable> {
            if patient_workspace_id == self.failing_workspace {
                return Err(physician_auth::StoreUnavailable);
            }
            self.inner
                .try_membership(identity, patient_workspace_id)
                .await
        }
        async fn membership(
            &self,
            identity: &PhysicianIdentity,
            patient_workspace_id: &str,
        ) -> Membership {
            self.inner.membership(identity, patient_workspace_id).await
        }
        async fn set_membership(
            &self,
            identity: &PhysicianIdentity,
            patient_workspace_id: &str,
            membership: Membership,
        ) -> Result<(), physician_auth::StoreUnavailable> {
            self.inner
                .set_membership(identity, patient_workspace_id, membership)
                .await
        }
        async fn place_on_panel(
            &self,
            identity: &PhysicianIdentity,
            patient_workspace_id: &str,
        ) -> Result<(), physician_auth::StoreUnavailable> {
            self.inner
                .place_on_panel(identity, patient_workspace_id)
                .await
        }
        async fn try_panel(
            &self,
            identity: &PhysicianIdentity,
        ) -> Result<Vec<String>, physician_auth::StoreUnavailable> {
            self.inner.try_panel(identity).await
        }
        async fn panel(&self, identity: &PhysicianIdentity) -> Vec<String> {
            self.inner.panel(identity).await
        }
        async fn assign_panel(
            &self,
            actor: &PhysicianIdentity,
            physician_id: &str,
            patient_workspace_id: &str,
            membership: Membership,
        ) -> Result<(), physician_auth::PanelError> {
            self.inner
                .assign_panel(actor, physician_id, patient_workspace_id, membership)
                .await
        }
        async fn ensure_practice(
            &self,
            practice_id: &str,
            clock: physician_auth::PracticeClock,
        ) -> Result<(), physician_auth::StoreUnavailable> {
            self.inner.ensure_practice(practice_id, clock).await
        }
        async fn upsert_practice(
            &self,
            record: &physician_auth::PracticeRecord,
        ) -> Result<physician_auth::PracticeSetupOutcome, physician_auth::PracticeSetupError>
        {
            self.inner.upsert_practice(record).await
        }
        async fn practice(
            &self,
            practice_id: &str,
        ) -> Result<Option<physician_auth::PracticeRecord>, physician_auth::StoreUnavailable>
        {
            self.inner.practice(practice_id).await
        }
        async fn practice_clock(
            &self,
            practice_id: &str,
        ) -> Result<physician_auth::PracticeClock, physician_auth::StoreUnavailable> {
            self.inner.practice_clock(practice_id).await
        }
        async fn set_synthetic_practice_clock(
            &self,
            practice_id: &str,
            update: physician_auth::SyntheticClockUpdate,
        ) -> Result<physician_auth::PracticeClock, physician_auth::ClockUpdateError> {
            self.inner
                .set_synthetic_practice_clock(practice_id, update)
                .await
        }
        async fn register_patient(
            &self,
            practice_id: &str,
            patient_workspace_id: &str,
        ) -> Result<(), physician_auth::StoreUnavailable> {
            self.inner
                .register_patient(practice_id, patient_workspace_id)
                .await
        }
        async fn patient_practice(
            &self,
            patient_workspace_id: &str,
        ) -> Result<Option<String>, physician_auth::StoreUnavailable> {
            self.inner.patient_practice(patient_workspace_id).await
        }
        async fn practice_patients(
            &self,
            practice_id: &str,
        ) -> Result<Vec<String>, physician_auth::StoreUnavailable> {
            self.inner.practice_patients(practice_id).await
        }
        async fn practice_accounts(
            &self,
            practice_id: &str,
        ) -> Result<Vec<physician_auth::Account>, physician_auth::StoreUnavailable> {
            AuthStore::practice_accounts(self.inner.as_ref(), practice_id).await
        }
        async fn create_account(
            &self,
            practice_id: &str,
            physician_id: Option<&str>,
            email: &str,
            display_name: &str,
            admin: bool,
            now: u64,
        ) -> Result<
            (
                physician_auth::Account,
                physician_auth::AccountCreateOutcome,
            ),
            physician_auth::AccountSetupError,
        > {
            self.inner
                .create_account(practice_id, physician_id, email, display_name, admin, now)
                .await
        }
        async fn invite(
            &self,
            practice_id: &str,
            email: &str,
            display_name: &str,
            admin: bool,
            now: u64,
        ) -> Result<String, physician_auth::SignInError> {
            self.inner
                .invite(practice_id, email, display_name, admin, now)
                .await
        }
        async fn redeem_invitation(
            &self,
            email: &str,
            token: &str,
            now: u64,
        ) -> Result<(PhysicianIdentity, String), physician_auth::SignInError> {
            self.inner.redeem_invitation(email, token, now).await
        }
        async fn deactivate(&self, email: &str) -> bool {
            self.inner.deactivate(email).await
        }
        async fn reactivate(&self, email: &str) -> bool {
            self.inner.reactivate(email).await
        }
        async fn remove_account(&self, email: &str) -> bool {
            self.inner.remove_account(email).await
        }
    }

    #[tokio::test]
    async fn unchecked_patient_marks_cross_patient_lists_incomplete_and_single_patient_503() {
        let inner = Arc::new(MemoryAuthStore::new());
        inner.seed_account(
            STAGING_PRACTICE_ID,
            "staging-physician",
            STAGING_EMAIL,
            "Staging Physician",
            true,
        );
        let failing_ws = crate::WORKSPACE_IDS[1].to_owned();
        let wrapper = Arc::new(PartialFailAuthStore {
            inner: Arc::clone(&inner),
            failing_workspace: failing_ws.clone(),
        });
        let auth = Auth::over_fixture(
            wrapper as Arc<dyn AuthStore>,
            Arc::new(MemoryFrontDoorStore::new()),
        )
        .await;
        let app = required_router(crate::fixture_site(), auth);
        let session = session_for(&app, STAGING_EMAIL).await;

        for path in [
            "/patient-workspaces",
            "/today",
            "/recent-conversations",
            "/updates",
        ] {
            let (status, body) = send(&app, Method::GET.as_str(), path, Some(&session), None).await;
            assert_eq!(status, StatusCode::OK, "{path}: {body}");
            let value: Value = serde_json::from_str(&body).expect("json body");
            assert_eq!(
                value["may_be_missing_someone"],
                json!(true),
                "{path} must surface may_be_missing_someone=true: {body}"
            );
            assert!(
                !body.contains(&failing_ws),
                "{path} must exclude unchecked patient {failing_ws}: {body}"
            );
        }

        // Single-patient endpoint for the unchecked patient fails loudly with 503, never 404.
        let (single_status, _) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{failing_ws}/briefings"),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(single_status, StatusCode::SERVICE_UNAVAILABLE);
    }
}
