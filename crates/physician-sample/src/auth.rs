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
//! is set — migrations apply on start — and process memory otherwise. The
//! front door rides the same database when it is set, so a practice's
//! invitations and premium reads survive a restart.
//!
//! Identity on the front door is email-based for now: whether Duet accounts
//! keep a shared email magic-link provider or move to Sign in with Apple is
//! an open "Discuss with Dev" item, not decided here.
//! Book: `book/src/physician-site.md`.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::body::Body;
use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use serde::Deserialize;
use serde_json::{Value, json};

use front_door::{FrontDoorStore, MemoryFrontDoorStore, PostgresFrontDoorStore, RosterPatient};
use physician_auth::{
    AuthStore, Membership, MemoryAuthStore, PanelError, PhysicianIdentity, PostgresAuthStore,
    SessionError, SignInError,
};

/// The staging practice every fixture belongs to.
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

/// The auth state shared by the routes and the guard.
pub(crate) struct Auth {
    /// The durable store: Postgres when configured, memory otherwise.
    pub(crate) store: Arc<dyn AuthStore>,
    /// Sign-in tokens are returned to the caller here instead of email.
    pub(crate) delivery: std::sync::Mutex<Vec<String>>,
    /// The Duet front door this practice invites patients through. One
    /// store serves every practice; isolation is enforced per request.
    pub(crate) front_door: Arc<dyn FrontDoorStore>,
}

/// Grants the staging physician access to the recorded practice's five
/// workspaces. Runs before the router is built, so a workspace check never
/// races its grant.
async fn seed_staging(store: &dyn AuthStore) {
    let identity = PhysicianIdentity {
        practice_id: STAGING_PRACTICE_ID.to_owned(),
        physician_id: "staging-physician".to_owned(),
    };
    // The staging physician may read the recorded practice's five workspaces,
    // the one list the recorded projections already own.
    for workspace in super::WORKSPACE_IDS {
        store
            .set_membership(&identity, workspace, Membership::Allowed)
            .await;
    }
}

impl Auth {
    /// Builds the auth state on one pair of stores after seeding the
    /// staging access. The front door is chosen beside the auth store:
    /// Postgres rides the same database URL, memory rides local runs.
    async fn on(store: Arc<dyn AuthStore>, front_door: Arc<dyn FrontDoorStore>) -> Arc<Self> {
        seed_staging(&*store).await;
        Arc::new(Self {
            store,
            delivery: std::sync::Mutex::new(Vec::new()),
            front_door,
        })
    }

    /// Builds the process-memory auth state for local runs and tests.
    pub(crate) async fn memory() -> Arc<Self> {
        let store = Arc::new(MemoryAuthStore::new());
        store.seed_account(
            STAGING_PRACTICE_ID,
            "staging-physician",
            "physician@codahealth.test",
            "Staging Physician",
            true,
        );
        Self::on(store, Arc::new(MemoryFrontDoorStore::new())).await
    }

    /// Builds the Postgres auth state, applying the migration chain first.
    /// The front door rides the same database, under its own tables.
    pub(crate) async fn postgres(database_url: &str) -> Result<Arc<Self>, String> {
        let store = PostgresAuthStore::connect_with_migrations(database_url)
            .await
            .map_err(|error| format!("physician auth store: {error}"))?;
        let front_door = PostgresFrontDoorStore::connect_with_migrations(database_url)
            .await
            .map_err(|error| format!("front-door store: {error}"))?;
        Ok(Self::on(Arc::new(store), Arc::new(front_door)).await)
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

/// Resolves the presented bearer token.
pub(crate) async fn resolve(
    auth: &Auth,
    token: Option<String>,
) -> Result<ResolvedSession, StatusCode> {
    let token = token.ok_or(StatusCode::UNAUTHORIZED)?;
    let identity = auth
        .store
        .session(&token, now())
        .await
        .map_err(|error| match error {
            SessionError::Invalid | SessionError::Inactive => StatusCode::UNAUTHORIZED,
        })?;
    Ok(identity)
}

/// Whether this path is one the guard protects: exactly the physician
/// routes, the one list the proxy also forwards, minus `/health`, which the
/// load balancer needs unauthenticated. The auth routes establish the
/// session and the static site renders it, so both stay open.
fn guarded(path: &str) -> bool {
    crate::physician_route(path) && path != "/health"
}

/// The route guard: resolves the session before any physician read, and
/// resolves the path's workspace membership before any patient read or
/// workspace-directed command dispatch.
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
    let identity = match resolve(&auth, token).await {
        Ok(identity) => identity,
        Err(status) => {
            return (
                status,
                Json(json!({ "detail": "Sign in to read the practice." })),
            )
                .into_response();
        }
    };
    let (mut parts, body) = request.into_parts();
    parts.extensions.insert(identity.clone());
    // A command names its patient workspace in the body, not the path. The
    // body is read only for `/commands`, bounded, so no physician command
    // dispatches into a workspace the session's membership denies.
    let body = if parts.uri.path() == "/commands" {
        match axum::body::to_bytes(body, COMMAND_BODY_LIMIT).await {
            Ok(bytes) => {
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
    if let Some(workspace) = direct_workspace(parts.uri.path()) {
        if allows_workspace(&auth, &identity, workspace).await {
            return next.run(Request::from_parts(parts, body)).await;
        }
        // Within the same clinic, physicians can access basic demographic
        // information about other patients in their clinic; unknown or
        // cross-clinic patients still answer 404.
        if identity.practice_id == STAGING_PRACTICE_ID
            && let Some(raw) = crate::workspace_body(workspace)
        {
            return clinic_demographics(raw);
        }
        return not_found();
    }
    if let Some(workspace) = request_workspace(parts.uri.path(), parts.uri.query())
        && !allows_workspace(&auth, &identity, &workspace).await
    {
        // 404, not 403: a denied physician cannot read clinical projections
        // for a patient outside their panel.
        return not_found();
    }
    if parts.uri.path() == "/patient-roster" && identity.practice_id != STAGING_PRACTICE_ID {
        return Json(json!({ "patients": [], "next_after": null })).into_response();
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

/// Basic demographic information for a patient in the caller's clinic who is
/// not on the caller's panel: identity demographics are preserved while
/// clinical synopsis, citations, chats, follow-ups, and source handles stay omitted.
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

/// Issues a sign-in token. Proof setups read it from the response; a real
/// deployment delivers it by email instead.
async fn sign_in(State(auth): State<Arc<Auth>>, body: axum::body::Bytes) -> Response {
    let Ok(body) = serde_json::from_slice::<SignInBody>(&body) else {
        return sign_in_unavailable();
    };
    match auth.store.begin_sign_in(&body.email, now()).await {
        Ok(token) => {
            // Proof delivery: the token lands in memory instead of mail.
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
        Err(_) => sign_in_unavailable(),
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
        Err(SignInError::Invalid | SignInError::Inactive) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "detail": "That sign-in link is no longer valid." })),
        )
            .into_response(),
    }
}

/// Ends the presented session.
async fn sign_out(State(auth): State<Arc<Auth>>, request: Request<Body>) -> Response {
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
    Json(json!({
        "physician_id": identity.physician_id,
        "practice_id": identity.practice_id,
        "display_name": account.map(|account| account.display_name)
            .unwrap_or_else(|| identity.physician_id.clone()),
        "admin": admin,
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
    allowed: bool,
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
    let Ok(body) = serde_json::from_slice::<PanelAssignBody>(&bytes) else {
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
        || !crate::WORKSPACE_IDS.contains(&patient_workspace_id.as_str())
    {
        return not_found();
    }
    let membership = if body.allowed {
        Membership::Allowed
    } else {
        Membership::Denied
    };
    match auth
        .store
        .assign_panel(&caller, &physician_id, &patient_workspace_id, membership)
        .await
    {
        Ok(()) => Json(json!({
            "practice_id": caller.practice_id,
            "physician_id": physician_id,
            "patient_workspace_id": patient_workspace_id,
            "allowed": body.allowed,
        }))
        .into_response(),
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
/// the roster's existence stays unknown outside the practice. The token
/// rides the response here; a real deployment delivers it by email.
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
        Ok(token) => (
            StatusCode::OK,
            Json(json!({
                "practice_id": caller.practice_id,
                "patient_workspace_id": body.patient_workspace_id,
                "email": body.email,
                "display_name": body.display_name,
                "status": "invited",
                // Proof delivery: the token lands in the response instead
                // of mail; the Duet side redeems it at the front door.
                "token": token,
            })),
        )
            .into_response(),
        Err(front_door::InviteError::Conflict) => (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": "That email is already invited or has an account.",
            })),
        )
            .into_response(),
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
            put(put_panel),
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

async fn inject_auth_store(
    State(auth): State<Arc<Auth>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    request.extensions_mut().insert(Arc::clone(&auth.store));
    next.run(request).await
}

/// Builds the router with auth required on the recorded practice.
///
/// `PHYSICIAN_AUTH=required` guards every physician route of `base` through
/// one store — Postgres when `PHYSICIAN_DATABASE_URL` is set (migrations
/// apply on start), memory otherwise; unset or any other value serves `base`
/// with the auth routes still present for the client.
pub async fn authed_router(base: axum::Router) -> axum::Router {
    let auth = match (auth_mode(), database_url()) {
        (AuthMode::Required, Some(url)) => match Auth::postgres(&url).await {
            Ok(auth) => auth,
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        },
        _ => Auth::memory().await,
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
    use physician_auth::{Membership, MemoryAuthStore, PhysicianIdentity};
    use tower::ServiceExt;

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
}
