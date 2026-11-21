//! Zero-sign-in public demo sessions and bounded-memory turn budgets.
//!
//! When the server runs under the `public-demo` profile
//! ([`crate::hosting::is_public_demo`]), `POST /auth/demo-session` mints an
//! isolated ephemeral demo physician in [`DEMO_PRACTICE_ID`], assigns the
//! practice's registered synthetic workspaces to that visitor's panel, and
//! returns a fixed-TTL bearer session. Turn submissions from [`DEMO_PRACTICE_ID`]
//! are additionally guarded by [`DemoTurnBudget`] before Postgres work is
//! accepted.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use http::ingress::{ClientKey, DEFAULT_MAX_BODY_BYTES, IngressGuardConfig};
use local_product::physician_sources::workspace_id;
use local_product::runtime::Store;
use physician_auth::{AuthStore, PhysicianIdentity, PostgresAuthStore};
use serde_json::json;

use crate::auth::SYNTHETIC_PRACTICE_CLOCK;

/// Dedicated synthetic practice serving public demo visitors.
pub const DEMO_PRACTICE_ID: &str = "coda-demo-practice";

/// Display name shown on ephemeral demo visitor accounts.
pub const DEMO_PHYSICIAN_DISPLAY_NAME: &str = "Dr. Hannah Alvarez (Demo)";

/// Default fixed lifetime for each public demo bearer session (2 hours).
pub const DEMO_SESSION_SECONDS: u64 = 2 * 3600;

/// Environment variable overriding [`DEMO_SESSION_SECONDS`].
pub const DEMO_SESSION_SECONDS_ENV: &str = "CODA_DEMO_SESSION_SECONDS";

/// Default maximum model-invoking turns per ephemeral demo physician session.
pub const DEFAULT_DEMO_SESSION_MAX_TURNS: u32 = 20;

/// Environment variable overriding [`DEFAULT_DEMO_SESSION_MAX_TURNS`].
pub const DEMO_SESSION_MAX_TURNS_ENV: &str = "CODA_DEMO_SESSION_MAX_TURNS";

/// Default maximum model-invoking demo turns per client IP per hour (`3600s`).
pub const DEFAULT_DEMO_IP_HOURLY_MAX_TURNS: u32 = 30;

/// Environment variable overriding [`DEFAULT_DEMO_IP_HOURLY_MAX_TURNS`].
pub const DEMO_IP_HOURLY_MAX_TURNS_ENV: &str = "CODA_DEMO_IP_HOURLY_MAX_TURNS";

/// Default global daily cap across all public demo visitors (`86400s`).
pub const DEFAULT_DEMO_DAILY_MAX_TURNS: u32 = 500;

/// Environment variable overriding [`DEFAULT_DEMO_DAILY_MAX_TURNS`].
pub const DEMO_DAILY_MAX_TURNS_ENV: &str = "CODA_DEMO_DAILY_MAX_TURNS";

const DEMO_SOURCE: &str = "demo-fixture";
const IP_WINDOW: Duration = Duration::from_secs(3600);
const DAILY_WINDOW: Duration = Duration::from_secs(86_400);
const MAX_TRACKED_SESSIONS: usize = 4_096;
const MAX_TRACKED_IPS: usize = 4_096;

/// Workspace identifiers for the synthetic fixture panel seeded into [`DEMO_PRACTICE_ID`].
pub fn demo_workspaces() -> Vec<String> {
    crate::WORKSPACE_IDS
        .iter()
        .map(|fixture| workspace_id(DEMO_SOURCE, fixture))
        .collect()
}

/// Seeds [`DEMO_PRACTICE_ID`] on [`SYNTHETIC_PRACTICE_CLOCK`] and enrolls the
/// synthetic fixture panel so isolated demo sessions can open and chat against
/// those patients without sharing data across practices.
pub async fn seed_demo_practice(
    auth: &PostgresAuthStore,
    runtime: &Store,
    _now: u64,
) -> Result<(), String> {
    auth.ensure_practice(DEMO_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
        .await
        .map_err(|_| "create the public demo practice".to_owned())?;
    for (fixture, workspace) in crate::WORKSPACE_IDS.iter().zip(demo_workspaces()) {
        let owner = auth
            .patient_practice(&workspace)
            .await
            .map_err(|_| format!("{workspace}: read the patient's practice"))?;
        if owner.as_deref().is_some_and(|id| id != DEMO_PRACTICE_ID) {
            return Err(format!("{workspace}: belongs to another practice"));
        }
        let mut pull = crate::patients::fixture_pull(fixture);
        pull.source_id = crate::synthetic_chart::SOURCE_ID.to_owned();
        pull.source_label = "Synthetic demo chart".to_owned();
        let pulled_at = pull.pulled_at.unwrap_or(1_787_846_400);
        runtime
            .record_chart(&workspace, &[], &pull)
            .await
            .map_err(|error| format!("{workspace}: {error}"))?;
        auth.register_patient(DEMO_PRACTICE_ID, &workspace)
            .await
            .map_err(|_| format!("{workspace}: register the patient"))?;
        runtime
            .register_patient_dispatch(auth, DEMO_PRACTICE_ID, &workspace, pulled_at)
            .await
            .map_err(|error| format!("{workspace}: {error}"))?;
    }
    Ok(())
}

/// Configuration for `POST /auth/demo-session`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DemoSessionConfig {
    /// Fixed session lifetime in seconds.
    pub session_seconds: u64,
}

impl Default for DemoSessionConfig {
    fn default() -> Self {
        Self {
            session_seconds: DEMO_SESSION_SECONDS,
        }
    }
}

impl DemoSessionConfig {
    /// Reads demo session configuration from `get`, validating positive lifetimes.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let session_seconds = match get(DEMO_SESSION_SECONDS_ENV)
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
        {
            Some(raw) => {
                let parsed = raw.parse::<u64>().map_err(|_| {
                    format!("{DEMO_SESSION_SECONDS_ENV} must be a positive integer")
                })?;
                if parsed == 0 {
                    return Err(format!("{DEMO_SESSION_SECONDS_ENV} must be greater than 0"));
                }
                parsed
            }
            None => DEMO_SESSION_SECONDS,
        };
        Ok(Self { session_seconds })
    }
}

struct DemoSignInState {
    auth: Arc<crate::auth::Auth>,
    session_seconds: u64,
}

/// Routes mounted only under the `public-demo` profile (`POST /auth/demo-session`).
pub(crate) fn sign_in_routes(
    auth: Arc<crate::auth::Auth>,
    config: DemoSessionConfig,
) -> axum::Router {
    let state = Arc::new(DemoSignInState {
        auth,
        session_seconds: config.session_seconds,
    });
    axum::Router::new()
        .route("/auth/demo-session", axum::routing::post(sign_in))
        .with_state(state)
}

async fn sign_in(State(state): State<Arc<DemoSignInState>>) -> Response {
    let store = &state.auth.store;
    let now = crate::auth::now();
    let physician_id = format!("demo-{}", uuid::Uuid::new_v4().simple());
    let email = format!("{physician_id}@demo.codahealth.test");
    if let Err(error) = store
        .ensure_practice(DEMO_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
        .await
    {
        tracing::warn!(?error, "public demo practice unavailable");
        return demo_store_unavailable();
    }
    if let Err(error) = store
        .create_account(
            DEMO_PRACTICE_ID,
            Some(&physician_id),
            &email,
            DEMO_PHYSICIAN_DISPLAY_NAME,
            false,
            now,
        )
        .await
    {
        tracing::warn!(?error, "failed creating ephemeral demo physician account");
        return demo_store_unavailable();
    }
    let workspaces = match store.practice_patients(DEMO_PRACTICE_ID).await {
        Ok(workspaces) => workspaces,
        Err(error) => {
            tracing::warn!(?error, "failed listing demo practice patients");
            return demo_store_unavailable();
        }
    };
    let identity = PhysicianIdentity {
        practice_id: DEMO_PRACTICE_ID.to_owned(),
        physician_id: physician_id.clone(),
    };
    for workspace in workspaces {
        if let Err(error) = store.place_on_panel(&identity, &workspace).await {
            tracing::warn!(?error, %workspace, "failed assigning demo workspace to visitor panel");
            return demo_store_unavailable();
        }
    }
    let ends_at = now.saturating_add(state.session_seconds);
    match store.mint_fixed_session(&identity, now, ends_at).await {
        Ok(session) => {
            let mut response = (
                StatusCode::OK,
                Json(json!({
                    "session": session,
                    "physician_id": identity.physician_id,
                    "practice_id": identity.practice_id,
                    "ends_at": ends_at,
                })),
            )
                .into_response();
            response
                .headers_mut()
                .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
            response
        }
        Err(error) => {
            tracing::warn!(?error, "failed minting ephemeral demo session");
            demo_store_unavailable()
        }
    }
}

fn demo_store_unavailable() -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({
            "detail": "The public demo practice is temporarily unavailable.",
        })),
    )
        .into_response()
}

/// Turn-budget configuration for [`DEMO_PRACTICE_ID`] sessions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DemoTurnBudgetConfig {
    /// Maximum model-invoking turns allowed for one demo `physician_id`.
    pub session_max_turns: u32,
    /// Maximum model-invoking demo turns allowed per client IP per hour (`3600s`).
    pub ip_hourly_max_turns: u32,
    /// Global maximum model-invoking demo turns allowed per day (`86400s`).
    pub daily_max_turns: u32,
    /// Trusted proxy hop count for client IP extraction via [`ClientKey::from_request`].
    pub trusted_proxy_hops: usize,
    /// Positive `Retry-After` fallback reported when a per-session cap is reached.
    pub session_retry_after_seconds: u64,
}

impl Default for DemoTurnBudgetConfig {
    fn default() -> Self {
        Self {
            session_max_turns: DEFAULT_DEMO_SESSION_MAX_TURNS,
            ip_hourly_max_turns: DEFAULT_DEMO_IP_HOURLY_MAX_TURNS,
            daily_max_turns: DEFAULT_DEMO_DAILY_MAX_TURNS,
            trusted_proxy_hops: http::ingress::DEFAULT_TRUSTED_PROXY_HOPS,
            session_retry_after_seconds: DEMO_SESSION_SECONDS,
        }
    }
}

impl DemoTurnBudgetConfig {
    /// Reads demo turn-budget limits from `get`.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let parse_limit = |key: &str, default: u32| -> Result<u32, String> {
            match get(key)
                .map(|v| v.trim().to_owned())
                .filter(|v| !v.is_empty())
            {
                Some(raw) => raw
                    .parse::<u32>()
                    .map_err(|_| format!("{key} must be a non-negative integer")),
                None => Ok(default),
            }
        };
        let session_config = DemoSessionConfig::from_env(&get)?;
        let ingress = IngressGuardConfig::from_env(&get, DEFAULT_MAX_BODY_BYTES);
        Ok(Self {
            session_max_turns: parse_limit(
                DEMO_SESSION_MAX_TURNS_ENV,
                DEFAULT_DEMO_SESSION_MAX_TURNS,
            )?,
            ip_hourly_max_turns: parse_limit(
                DEMO_IP_HOURLY_MAX_TURNS_ENV,
                DEFAULT_DEMO_IP_HOURLY_MAX_TURNS,
            )?,
            daily_max_turns: parse_limit(DEMO_DAILY_MAX_TURNS_ENV, DEFAULT_DEMO_DAILY_MAX_TURNS)?,
            trusted_proxy_hops: ingress.trusted_proxy_hops,
            session_retry_after_seconds: session_config.session_seconds.max(1),
        })
    }
}

/// Which quota caused [`DemoTurnBudget`] to reject a demo turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DemoBudgetLimit {
    /// Per-demo-session lifetime turn cap.
    Session,
    /// Per-client-IP hourly window cap.
    IpHourly,
    /// Global daily window cap across all demo visitors.
    Daily,
}

/// Rejection details when a demo turn budget is exhausted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DemoBudgetExhausted {
    /// Quota that triggered the rejection.
    pub limit: DemoBudgetLimit,
    /// Positive `Retry-After` interval in seconds.
    pub retry_after_seconds: u64,
}

impl DemoBudgetExhausted {
    fn detail(self) -> &'static str {
        match self.limit {
            DemoBudgetLimit::Session => {
                "This demo session has reached its turn limit. Start a new demo session to continue."
            }
            DemoBudgetLimit::IpHourly => {
                "Hourly public demo turn limit reached for this network address. Please try again later."
            }
            DemoBudgetLimit::Daily => {
                "The public demo has reached its daily capacity. Please try again later."
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SessionBucket {
    count: u32,
    last_seen: Instant,
}

#[derive(Clone, Copy, Debug)]
struct WindowBucket {
    window_start: Instant,
    count: u32,
}

#[derive(Debug)]
struct BudgetState {
    sessions: HashMap<String, SessionBucket>,
    ips: HashMap<ClientKey, WindowBucket>,
    daily: WindowBucket,
}

/// Bounded-memory per-session, per-IP hourly, and global daily turn budget for
/// model-invoking commands in [`DEMO_PRACTICE_ID`].
#[derive(Clone, Debug)]
pub struct DemoTurnBudget {
    config: DemoTurnBudgetConfig,
    state: Arc<Mutex<BudgetState>>,
}

impl DemoTurnBudget {
    /// Builds a budget enforcer for `config`.
    pub fn new(config: DemoTurnBudgetConfig) -> Self {
        Self {
            config,
            state: Arc::new(Mutex::new(BudgetState {
                sessions: HashMap::new(),
                ips: HashMap::new(),
                daily: WindowBucket {
                    window_start: Instant::now(),
                    count: 0,
                },
            })),
        }
    }

    /// Builds a budget enforcer configured from `get`.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        DemoTurnBudgetConfig::from_env(get).map(Self::new)
    }

    /// Checks and consumes one turn for `identity` if `identity.practice_id`
    /// is [`DEMO_PRACTICE_ID`]. Non-demo practices are unaffected.
    pub fn check_and_record(
        &self,
        identity: &PhysicianIdentity,
        headers: &HeaderMap,
        peer_addr: Option<SocketAddr>,
    ) -> Result<(), DemoBudgetExhausted> {
        if identity.practice_id != DEMO_PRACTICE_ID {
            return Ok(());
        }
        let client = ClientKey::from_request(headers, peer_addr, self.config.trusted_proxy_hops);
        self.consume_at(&identity.physician_id, client, Instant::now())
    }

    fn consume_at(
        &self,
        physician_id: &str,
        client: ClientKey,
        now: Instant,
    ) -> Result<(), DemoBudgetExhausted> {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        let daily_elapsed = now.saturating_duration_since(state.daily.window_start);
        if daily_elapsed >= DAILY_WINDOW {
            state.daily = WindowBucket {
                window_start: now,
                count: 0,
            };
        }
        if state.daily.count >= self.config.daily_max_turns {
            let remaining = DAILY_WINDOW.saturating_sub(daily_elapsed);
            return Err(DemoBudgetExhausted {
                limit: DemoBudgetLimit::Daily,
                retry_after_seconds: remaining.as_secs().max(1),
            });
        }

        if let Some(bucket) = state.ips.get(&client).copied() {
            let ip_elapsed = now.saturating_duration_since(bucket.window_start);
            if ip_elapsed < IP_WINDOW && bucket.count >= self.config.ip_hourly_max_turns {
                let remaining = IP_WINDOW.saturating_sub(ip_elapsed);
                return Err(DemoBudgetExhausted {
                    limit: DemoBudgetLimit::IpHourly,
                    retry_after_seconds: remaining.as_secs().max(1),
                });
            }
        } else if self.config.ip_hourly_max_turns == 0 {
            return Err(DemoBudgetExhausted {
                limit: DemoBudgetLimit::IpHourly,
                retry_after_seconds: IP_WINDOW.as_secs().max(1),
            });
        }

        if let Some(session_bucket) = state.sessions.get(physician_id) {
            if session_bucket.count >= self.config.session_max_turns {
                return Err(DemoBudgetExhausted {
                    limit: DemoBudgetLimit::Session,
                    retry_after_seconds: self.config.session_retry_after_seconds.max(1),
                });
            }
        } else if self.config.session_max_turns == 0 {
            return Err(DemoBudgetExhausted {
                limit: DemoBudgetLimit::Session,
                retry_after_seconds: self.config.session_retry_after_seconds.max(1),
            });
        }

        state.daily.count = state.daily.count.saturating_add(1);

        let reset_ip = state
            .ips
            .get(&client)
            .is_some_and(|bucket| now.saturating_duration_since(bucket.window_start) >= IP_WINDOW);
        if reset_ip || !state.ips.contains_key(&client) {
            if !state.ips.contains_key(&client) && state.ips.len() >= MAX_TRACKED_IPS {
                state.ips.retain(|_, bucket| {
                    now.saturating_duration_since(bucket.window_start) < IP_WINDOW
                });
                if state.ips.len() >= MAX_TRACKED_IPS
                    && let Some(oldest_key) = state
                        .ips
                        .iter()
                        .min_by_key(|(_, bucket)| bucket.window_start)
                        .map(|(k, _)| *k)
                {
                    state.ips.remove(&oldest_key);
                }
            }
            state.ips.insert(
                client,
                WindowBucket {
                    window_start: now,
                    count: 1,
                },
            );
        } else if let Some(bucket) = state.ips.get_mut(&client) {
            bucket.count = bucket.count.saturating_add(1);
        }

        if !state.sessions.contains_key(physician_id)
            && state.sessions.len() >= MAX_TRACKED_SESSIONS
            && let Some(oldest_id) = state
                .sessions
                .iter()
                .min_by_key(|(_, bucket)| bucket.last_seen)
                .map(|(k, _)| k.clone())
        {
            state.sessions.remove(&oldest_id);
        }
        let session = state
            .sessions
            .entry(physician_id.to_owned())
            .or_insert(SessionBucket {
                count: 0,
                last_seen: now,
            });
        session.count = session.count.saturating_add(1);
        session.last_seen = now;

        Ok(())
    }
}

/// Fallback router extension used by [`crate::site`] when no process-wide
/// [`DemoTurnBudget`] override was installed by [`crate::site_router_from_env`].
#[derive(Clone, Debug)]
pub(crate) struct FallbackDemoTurnBudget(pub(crate) Arc<DemoTurnBudget>);

/// Enforces the active [`DemoTurnBudget`] from router extensions for handlers
/// outside `/commands` (such as follow-up review/resume and dictation),
/// returning `Some(response)` (`429`) when a cap is reached.
pub(crate) fn check_request_demo_budget(
    extensions: &axum::http::Extensions,
    headers: &HeaderMap,
    identity: &PhysicianIdentity,
) -> Option<Response> {
    if identity.practice_id != DEMO_PRACTICE_ID {
        return None;
    }
    let budget = extensions
        .get::<Arc<DemoTurnBudget>>()
        .or_else(|| extensions.get::<FallbackDemoTurnBudget>().map(|f| &f.0))?;
    let peer_addr = extensions
        .get::<axum::extract::ConnectInfo<SocketAddr>>()
        .map(|axum::extract::ConnectInfo(addr)| *addr);
    budget
        .check_and_record(identity, headers, peer_addr)
        .err()
        .map(demo_budget_exhausted_response)
}

/// Renders the `429 Too Many Requests` response when [`DemoTurnBudget`] rejects a turn.
pub fn demo_budget_exhausted_response(exhausted: DemoBudgetExhausted) -> Response {
    let retry_after = exhausted.retry_after_seconds.max(1);
    let mut response = (
        StatusCode::TOO_MANY_REQUESTS,
        Json(json!({
            "code": "demo_turn_budget_exhausted",
            "detail": exhausted.detail(),
            "retry_after_seconds": retry_after,
        })),
    )
        .into_response();
    if let Ok(value) = HeaderValue::from_str(&retry_after.to_string()) {
        response.headers_mut().insert(header::RETRY_AFTER, value);
    }
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    fn demo_id(physician_id: &str) -> PhysicianIdentity {
        PhysicianIdentity {
            practice_id: DEMO_PRACTICE_ID.to_owned(),
            physician_id: physician_id.to_owned(),
        }
    }

    #[test]
    fn turn_budget_enforces_session_ip_hourly_and_daily_caps_without_leaking_counts() {
        let budget = DemoTurnBudget::new(DemoTurnBudgetConfig {
            session_max_turns: 2,
            ip_hourly_max_turns: 3,
            daily_max_turns: 4,
            trusted_proxy_hops: 1,
            session_retry_after_seconds: 120,
        });
        let ip_a = ClientKey::Ip(IpAddr::V4(Ipv4Addr::new(198, 51, 100, 10)));
        let ip_b = ClientKey::Ip(IpAddr::V4(Ipv4Addr::new(198, 51, 100, 20)));
        let t0 = Instant::now();

        assert_eq!(budget.consume_at("demo-1", ip_a, t0), Ok(()));
        assert_eq!(budget.consume_at("demo-1", ip_a, t0), Ok(()));
        let session_hit = budget
            .consume_at("demo-1", ip_a, t0)
            .expect_err("third turn in demo-1 should hit session cap");
        assert_eq!(session_hit.limit, DemoBudgetLimit::Session);
        assert_eq!(session_hit.retry_after_seconds, 120);

        // Switching visitor identity on the same IP still shares the hourly IP cap (3),
        // and the rejected demo-1 attempt must not have consumed ip_a's 3rd slot.
        assert_eq!(budget.consume_at("demo-2", ip_a, t0), Ok(()));
        let ip_hit = budget
            .consume_at("demo-2", ip_a, t0 + Duration::from_secs(10))
            .expect_err("fourth turn from ip_a must hit hourly IP cap");
        assert_eq!(ip_hit.limit, DemoBudgetLimit::IpHourly);
        assert_eq!(ip_hit.retry_after_seconds, 3590);

        // After an hour passes, ip_a resets and can spend the 4th global daily turn,
        // then the 5th hits the global daily cap.
        let t_later = t0 + Duration::from_secs(3600);
        assert_eq!(budget.consume_at("demo-3", ip_a, t_later), Ok(()));
        let daily_hit = budget
            .consume_at("demo-4", ip_b, t_later)
            .expect_err("fifth total turn must hit daily cap");
        assert_eq!(daily_hit.limit, DemoBudgetLimit::Daily);

        // Non-demo practices are exempt from demo turn budgets.
        let real_id = PhysicianIdentity {
            practice_id: crate::STAGING_PRACTICE_ID.to_owned(),
            physician_id: "staging-physician".to_owned(),
        };
        assert_eq!(
            budget.check_and_record(&real_id, &HeaderMap::new(), None),
            Ok(())
        );
    }

    #[test]
    fn turn_budget_respects_rightmost_trusted_proxy_ip() {
        let budget = DemoTurnBudget::new(DemoTurnBudgetConfig {
            session_max_turns: 10,
            ip_hourly_max_turns: 1,
            daily_max_turns: 10,
            trusted_proxy_hops: 1,
            session_retry_after_seconds: 60,
        });
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("1.1.1.1, 203.0.113.9"),
        );
        assert!(
            budget
                .check_and_record(&demo_id("demo-a"), &headers, None)
                .is_ok()
        );

        // Spoofing a different leftmost IP cannot bypass the rightmost proxy hop cap.
        let mut spoofed = HeaderMap::new();
        spoofed.insert(
            "x-forwarded-for",
            HeaderValue::from_static("8.8.8.8, 203.0.113.9"),
        );
        let err = budget
            .check_and_record(&demo_id("demo-b"), &spoofed, None)
            .expect_err("same proxy hop must share IP quota");
        assert_eq!(err.limit, DemoBudgetLimit::IpHourly);
    }

    #[tokio::test]
    async fn demo_session_route_mints_isolated_sessions_only_when_mounted() {
        use axum::body::Body;
        use axum::http::Request;
        use physician_auth::MemoryAuthStore;
        use tower::ServiceExt;

        let store = Arc::new(MemoryAuthStore::new());
        store
            .ensure_practice(DEMO_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
            .await
            .unwrap();
        let workspace = demo_workspaces().into_iter().next().unwrap();
        store
            .register_patient(DEMO_PRACTICE_ID, &workspace)
            .await
            .unwrap();
        let auth = crate::auth::Auth::over_fixture(
            Arc::clone(&store) as Arc<dyn AuthStore>,
            Arc::new(front_door::MemoryFrontDoorStore::new()),
        )
        .await;

        let unmounted = crate::auth::required_router(crate::fixture_site(), Arc::clone(&auth));
        let not_found = unmounted
            .oneshot(
                Request::post("/auth/demo-session")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(not_found.status(), StatusCode::NOT_FOUND);

        let mounted = crate::auth::required_router(
            crate::fixture_site().merge(sign_in_routes(
                Arc::clone(&auth),
                DemoSessionConfig {
                    session_seconds: 7200,
                },
            )),
            Arc::clone(&auth),
        );
        let first_resp = mounted
            .clone()
            .oneshot(
                Request::post("/auth/demo-session")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(first_resp.status(), StatusCode::OK);
        let first_bytes = axum::body::to_bytes(first_resp.into_body(), 1024 * 64)
            .await
            .unwrap();
        let first: serde_json::Value = serde_json::from_slice(&first_bytes).unwrap();
        let p1 = first["physician_id"].as_str().unwrap();
        assert!(p1.starts_with("demo-"));
        assert_eq!(first["practice_id"], DEMO_PRACTICE_ID);
        let id1 = PhysicianIdentity {
            practice_id: DEMO_PRACTICE_ID.to_owned(),
            physician_id: p1.to_owned(),
        };
        assert_eq!(store.try_panel(&id1).await.unwrap(), vec![workspace]);

        let second_resp = mounted
            .oneshot(
                Request::post("/auth/demo-session")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(second_resp.status(), StatusCode::OK);
        let second_bytes = axum::body::to_bytes(second_resp.into_body(), 1024 * 64)
            .await
            .unwrap();
        let second: serde_json::Value = serde_json::from_slice(&second_bytes).unwrap();
        assert_ne!(second["physician_id"], first["physician_id"]);
        assert_ne!(second["session"], first["session"]);
    }

    #[tokio::test]
    async fn demo_budget_exhausted_response_sets_429_retry_after_and_code() {
        let response = demo_budget_exhausted_response(DemoBudgetExhausted {
            limit: DemoBudgetLimit::Session,
            retry_after_seconds: 90,
        });
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(
            response
                .headers()
                .get(header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok()),
            Some("90")
        );
        let body = axum::body::to_bytes(response.into_body(), 1024 * 64)
            .await
            .unwrap();
        let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(parsed["code"], "demo_turn_budget_exhausted");
        assert_eq!(parsed["retry_after_seconds"], 90);
    }

    #[tokio::test]
    async fn demo_session_forbids_invites_and_source_mutations_and_budgets_dictation() {
        use axum::body::Body;
        use axum::http::Request;
        use base64::Engine as _;
        use physician_auth::MemoryAuthStore;
        use tower::ServiceExt;

        let store = Arc::new(MemoryAuthStore::new());
        store
            .ensure_practice(DEMO_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
            .await
            .unwrap();
        let workspace = demo_workspaces().into_iter().next().unwrap();
        store
            .register_patient(DEMO_PRACTICE_ID, &workspace)
            .await
            .unwrap();
        let auth = crate::auth::Auth::over_fixture(
            Arc::clone(&store) as Arc<dyn AuthStore>,
            Arc::new(front_door::MemoryFrontDoorStore::new()),
        )
        .await;
        let budget = Arc::new(DemoTurnBudget::new(DemoTurnBudgetConfig {
            session_max_turns: 1,
            ip_hourly_max_turns: 10,
            daily_max_turns: 10,
            trusted_proxy_hops: 1,
            session_retry_after_seconds: 60,
        }));
        let base = crate::fixture_site()
            .merge(sign_in_routes(
                Arc::clone(&auth),
                DemoSessionConfig::default(),
            ))
            .layer(axum::Extension(
                crate::dictation::DictationBackend::LocalTest {
                    fixture: Some("Synthetic dictation".to_owned()),
                },
            ))
            .layer(axum::Extension(budget));
        let app = crate::auth::required_router(base, Arc::clone(&auth));

        let sign_in_resp = app
            .clone()
            .oneshot(
                Request::post("/auth/demo-session")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(sign_in_resp.status(), StatusCode::OK);
        let sign_in_bytes = axum::body::to_bytes(sign_in_resp.into_body(), 1024 * 64)
            .await
            .unwrap();
        let sign_in: serde_json::Value = serde_json::from_slice(&sign_in_bytes).unwrap();
        let token = sign_in["session"].as_str().unwrap().to_owned();

        let audio = base64::engine::general_purpose::STANDARD.encode(b"TEXT:check potassium");
        let dictation_body = serde_json::json!({
            "mime_type": "audio/webm",
            "audio_base64": audio,
        })
        .to_string();
        let first_dictation = app
            .clone()
            .oneshot(
                Request::post("/dictation")
                    .header(header::AUTHORIZATION, format!("Bearer {token}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(dictation_body.clone()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(first_dictation.status(), StatusCode::OK);

        let second_dictation = app
            .clone()
            .oneshot(
                Request::post("/dictation")
                    .header(header::AUTHORIZATION, format!("Bearer {token}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(dictation_body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(second_dictation.status(), StatusCode::TOO_MANY_REQUESTS);
        let dict_err_bytes = axum::body::to_bytes(second_dictation.into_body(), 1024 * 64)
            .await
            .unwrap();
        let dict_err: serde_json::Value = serde_json::from_slice(&dict_err_bytes).unwrap();
        assert_eq!(dict_err["code"], "demo_turn_budget_exhausted");

        for (path, payload) in [
            (
                "/auth/invitations",
                json!({
                    "patient_workspace_id": workspace,
                    "email": "abuse@example.test",
                    "display_name": "Abuse"
                }),
            ),
            (
                "/auth/colleagues/invitations",
                json!({
                    "email": "colleague@example.test",
                    "display_name": "Colleague"
                }),
            ),
            (
                "/connections/spruce/connect",
                json!({ "method": "simulated_practice" }),
            ),
            ("/connections/spruce/simulate", json!({ "event": "outage" })),
        ] {
            let res = app
                .clone()
                .oneshot(
                    Request::post(path)
                        .header(header::AUTHORIZATION, format!("Bearer {token}"))
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(payload.to_string()))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(res.status(), StatusCode::FORBIDDEN, "{path}");
        }

        let get_conn = app
            .oneshot(
                Request::get("/connections")
                    .header(header::AUTHORIZATION, format!("Bearer {token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(get_conn.status(), StatusCode::OK);
    }
}
