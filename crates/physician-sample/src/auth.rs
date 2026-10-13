//! Physician sign-in and access enforcement on the sample server.
//!
//! `PHYSICIAN_AUTH=required` resolves every physician route's session from
//! the request's `Authorization: Bearer` header through [`physician_auth`]:
//! unknown tokens never read, and a workspace check answers 404 so responses
//! never disclose a patient's existence. The auth routes themselves stay
//! open, and `/health` stays open, because a load balancer needs it
//! unauthenticated. The store is Postgres (SeaORM) when `PHYSICIAN_DATABASE_URL`
//! is set — migrations apply on start — and process memory otherwise.
//! Book: `book/src/physician-site.md`.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde::Deserialize;
use serde_json::{Value, json};

use physician_auth::{
    AuthStore, Membership, MemoryAuthStore, PhysicianIdentity, PostgresAuthStore, SessionError,
    SignInError,
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
    /// Builds the auth state on one store after seeding the staging access.
    async fn on(store: Arc<dyn AuthStore>) -> Arc<Self> {
        seed_staging(&*store).await;
        Arc::new(Self {
            store,
            delivery: std::sync::Mutex::new(Vec::new()),
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
        Self::on(store).await
    }

    /// Builds the Postgres auth state, applying the migration chain first.
    pub(crate) async fn postgres(database_url: &str) -> Result<Arc<Self>, String> {
        let store = PostgresAuthStore::connect_with_migrations(database_url)
            .await
            .map_err(|error| format!("physician auth store: {error}"))?;
        Ok(Self::on(Arc::new(store)).await)
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
    request: Request<Body>,
    next: Next,
) -> Response {
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
    let (parts, body) = request.into_parts();
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
    if let Some(workspace) = request_workspace(parts.uri.path(), parts.uri.query())
        && !allows_workspace(&auth, &identity, &workspace).await
    {
        // 404, not 403: a denied physician cannot learn the patient exists.
        return not_found();
    }
    next.run(Request::from_parts(parts, body)).await
}

/// The largest command body the guard reads to resolve its workspace.
const COMMAND_BODY_LIMIT: usize = 1_000_000;

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
    Json(json!({
        "physician_id": identity.physician_id,
        "practice_id": identity.practice_id,
        "display_name": account.map(|account| account.display_name)
            .unwrap_or_else(|| identity.physician_id.clone()),
    }))
    .into_response()
}

/// The workspace membership check a recorded handler uses: a denied or
/// missing membership answers 404 so the patient's existence stays unknown.
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
        .with_state(auth)
}

/// Builds the router with auth required on `base` through one explicit
/// store: the guard resolves the session before any physician read, over
/// the whole merged router, so no physician route bypasses it.
pub(crate) fn required_router(base: axum::Router, auth: Arc<Auth>) -> axum::Router {
    base.merge(auth_routes(Arc::clone(&auth)))
        .layer(axum::middleware::from_fn_with_state(auth, guard))
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
        AuthMode::Open => base.merge(auth_routes(auth)),
    }
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
    /// plus the concrete store for seeding a second physician's access.
    async fn authed_recorded() -> (axum::Router, Arc<MemoryAuthStore>) {
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
        let auth = Auth::on(Arc::clone(&store) as Arc<dyn AuthStore>).await;
        (required_router(crate::recorded(), auth), store)
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
        let (app, _) = authed_recorded().await;
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
        let (app, _) = authed_recorded().await;
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
        let (app, _) = authed_recorded().await;
        let (status, body) = send(&app, Method::GET.as_str(), "/health", None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("\"status\":\"ok\""), "{body}");
        let (status, page) = send(&app, Method::GET.as_str(), "/", None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert!(page.contains(r#"<main id="main"></main>"#), "page: {page}");
    }

    #[tokio::test]
    async fn whoami_names_the_session_physician() {
        let (app, _) = authed_recorded().await;
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
        let (app, _) = authed_recorded().await;
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
        let (app, _) = authed_recorded().await;
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
        let (app, _) = authed_recorded().await;
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
    async fn a_denied_workspace_is_indistinguishable_from_an_absent_one() {
        let (app, store) = authed_recorded().await;
        let colleague = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: "colleague".to_owned(),
        };
        store
            .set_membership(&colleague, crate::WORKSPACE_IDS[0], Membership::Allowed)
            .await;
        // WORKSPACE_IDS[1] is left unset: a missing row denies, like an
        // explicit Denied row.
        let session = session_for(&app, COLLEAGUE_EMAIL).await;
        let denied = crate::WORKSPACE_IDS[1];
        let absent = "pw-00000000-0000-0000-0000-000000000000";
        for path in [
            &format!("/patient-workspaces/{denied}"),
            &format!("/patient-workspaces/{absent}"),
            &format!("/today?patient_workspace_id={denied}"),
        ] {
            let (status, body) = send(&app, Method::GET.as_str(), path, Some(&session), None).await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{path}: {body}");
            assert!(body.is_empty(), "a 404 carried a body: {body}");
        }
        // The granted workspace reads; practice-level lists still read.
        let (status, body) = send(
            &app,
            Method::GET.as_str(),
            &format!("/patient-workspaces/{}", crate::WORKSPACE_IDS[0]),
            Some(&session),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
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
    async fn a_command_into_a_denied_workspace_returns_404() {
        let (app, store) = authed_recorded().await;
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
}
