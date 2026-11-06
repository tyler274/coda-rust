//! One authenticated `/readyz` pass over the hosted Rust services.
//!
//! Each run asks every configured target for `/readyz` once, classifies the
//! answer into a fixed outcome, and writes one Cloud Monitoring point per
//! target plus a heartbeat. Redirects are never followed: a 302 to the Google
//! sign-in page means the probe did not authenticate, never that the target is
//! up. Target names and outcome classes are fixed so the metric labels stay
//! low-cardinality.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use reqwest::{Client, StatusCode, header::HeaderValue, redirect::Policy};
use serde_json::{Value, json};

/// Metric written once per target per run: 1 when the target failed, 0 when it
/// answered ready. Labels: `target`, `outcome`.
pub const READINESS_METRIC: &str = "custom.googleapis.com/coda/availability/readiness_failed";
/// Metric written once per run, whatever the targets answered. Its absence
/// means the probe itself is not running or cannot publish.
pub const HEARTBEAT_METRIC: &str = "custom.googleapis.com/coda/availability/heartbeat";

const METADATA: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default";
const SIGN_JWT: &str = "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts";
const MONITORING: &str = "https://monitoring.googleapis.com/v3/projects";
const CREDENTIAL_TIMEOUT: Duration = Duration::from_secs(10);
const CREDENTIAL_LIMIT: usize = 64 * 1024;

/// The fixed set of monitored services.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Physician,
    Duet,
    PhysicianWorker,
}

impl Target {
    pub const ALL: [Target; 3] = [Target::Physician, Target::Duet, Target::PhysicianWorker];

    pub fn label(self) -> &'static str {
        match self {
            Target::Physician => "physician",
            Target::Duet => "duet",
            Target::PhysicianWorker => "physician-worker",
        }
    }

    fn url_key(self) -> &'static str {
        match self {
            Target::Physician => "AVAILABILITY_PROBE_PHYSICIAN_URL",
            Target::Duet => "AVAILABILITY_PROBE_DUET_URL",
            Target::PhysicianWorker => "AVAILABILITY_PROBE_WORKER_URL",
        }
    }

    /// Physician and Duet sit behind direct Cloud Run IAP, which accepts a
    /// service-account JWT whose audience is `<origin>/*`. The worker has no
    /// IAP; Cloud Run's invoker check accepts a Google ID token whose audience
    /// is the service URL.
    fn auth(self) -> AuthKind {
        match self {
            Target::Physician | Target::Duet => AuthKind::IapJwt,
            Target::PhysicianWorker => AuthKind::InvokerIdToken,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AuthKind {
    IapJwt,
    InvokerIdToken,
}

/// Fixed outcome classes. Only `Success` counts as ready.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// `/readyz` answered 2xx.
    Success,
    /// The probe was not let in: a redirect (IAP's sign-in 302), 401, 403, or
    /// the probe could not mint its own credential. Points at probe identity
    /// or IAM, not necessarily at the target.
    AuthFailure,
    /// The target answered with any other status, or the transport failed
    /// (DNS, TLS, reset) after the request was allowed.
    TargetFailure,
    /// No answer within the configured timeout. A cold start that exceeds the
    /// timeout lands here.
    Timeout,
    /// The connection was refused.
    Refused,
}

impl Outcome {
    pub fn label(self) -> &'static str {
        match self {
            Outcome::Success => "success",
            Outcome::AuthFailure => "auth_failure",
            Outcome::TargetFailure => "target_failure",
            Outcome::Timeout => "timeout",
            Outcome::Refused => "refused",
        }
    }

    pub fn failed(self) -> bool {
        self != Outcome::Success
    }
}

/// Classify an HTTP answer. Redirects are failures of authentication: with
/// redirects disabled, IAP's sign-in redirect arrives here as a 3xx.
pub fn classify_status(status: StatusCode) -> Outcome {
    if status.is_success() {
        Outcome::Success
    } else if status.is_redirection()
        || status == StatusCode::UNAUTHORIZED
        || status == StatusCode::FORBIDDEN
    {
        Outcome::AuthFailure
    } else {
        Outcome::TargetFailure
    }
}

/// Classify a transport error.
pub fn classify_error(error: &reqwest::Error) -> Outcome {
    if error.is_timeout() {
        return Outcome::Timeout;
    }
    let mut source: Option<&(dyn std::error::Error + 'static)> = Some(error);
    while let Some(current) = source {
        if let Some(io) = current.downcast_ref::<std::io::Error>()
            && io.kind() == std::io::ErrorKind::ConnectionRefused
        {
            return Outcome::Refused;
        }
        source = current.source();
    }
    Outcome::TargetFailure
}

/// One target's result for this run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Observation {
    pub target: Target,
    pub outcome: Outcome,
    pub status: Option<u16>,
    pub latency_ms: u128,
}

/// Runtime configuration, read from the environment.
#[derive(Clone, Debug)]
pub struct Config {
    pub project: String,
    pub service_account: String,
    pub targets: Vec<(Target, String)>,
    pub timeout: Duration,
    pub endpoints: Endpoints,
}

/// Google endpoints. Fixed in production; tests point them at local servers.
#[derive(Clone, Debug)]
pub struct Endpoints {
    pub metadata: String,
    pub sign_jwt: String,
    pub monitoring: String,
}

impl Default for Endpoints {
    fn default() -> Self {
        Self {
            metadata: METADATA.into(),
            sign_jwt: SIGN_JWT.into(),
            monitoring: MONITORING.into(),
        }
    }
}

impl Config {
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let required = |key: &str| {
            get(key)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| format!("{key} is required"))
        };
        let project = required("AVAILABILITY_PROBE_PROJECT")?;
        let service_account = required("AVAILABILITY_PROBE_SERVICE_ACCOUNT")?;
        let mut targets = Vec::new();
        for target in Target::ALL {
            if let Some(url) = get(target.url_key()).filter(|value| !value.trim().is_empty()) {
                let url = url.trim().trim_end_matches('/').to_owned();
                if !url.starts_with("https://") {
                    return Err(format!("{} must be an https origin", target.url_key()));
                }
                targets.push((target, url));
            }
        }
        if targets.is_empty() {
            return Err("no AVAILABILITY_PROBE_*_URL target is configured".into());
        }
        let timeout = match get("AVAILABILITY_PROBE_TIMEOUT_SECONDS") {
            None => Duration::from_secs(30),
            Some(value) => {
                let seconds: u64 = value.trim().parse().map_err(|error| {
                    format!("AVAILABILITY_PROBE_TIMEOUT_SECONDS must be whole seconds: {error}")
                })?;
                if !(1..=45).contains(&seconds) {
                    return Err(
                        "AVAILABILITY_PROBE_TIMEOUT_SECONDS must be between 1 and 45".into(),
                    );
                }
                Duration::from_secs(seconds)
            }
        };
        Ok(Self {
            project,
            service_account,
            targets,
            timeout,
            endpoints: Endpoints::default(),
        })
    }
}

/// HTTP client for the probes: never follows redirects.
pub fn probe_client(timeout: Duration) -> Result<Client, String> {
    Client::builder()
        .redirect(Policy::none())
        .timeout(timeout)
        .connect_timeout(timeout)
        .build()
        .map_err(|error| format!("probe client: {error}"))
}

/// Client for Google credential and Monitoring calls.
fn api_client() -> Result<Client, String> {
    Client::builder()
        .redirect(Policy::none())
        .timeout(CREDENTIAL_TIMEOUT)
        .build()
        .map_err(|error| format!("api client: {error}"))
}

/// GET `<origin>/readyz` once with an optional bearer and classify the answer.
/// The body is never read or logged.
pub async fn check(
    client: &Client,
    target: Target,
    origin: &str,
    bearer: Option<&HeaderValue>,
    timeout: Duration,
) -> Observation {
    let started = Instant::now();
    let mut request = client.get(format!("{origin}/readyz"));
    if let Some(bearer) = bearer {
        request = request.header(reqwest::header::AUTHORIZATION, bearer.clone());
    }
    // reqwest's own timeout covers the request; this outer bound also covers
    // connection setup if a resolver stalls.
    let answer = tokio::time::timeout(timeout + Duration::from_secs(1), request.send()).await;
    let (outcome, status) = match answer {
        Err(_) => (Outcome::Timeout, None),
        Ok(Err(error)) => (classify_error(&error), None),
        Ok(Ok(response)) => (
            classify_status(response.status()),
            Some(response.status().as_u16()),
        ),
    };
    Observation {
        target,
        outcome,
        status,
        latency_ms: started.elapsed().as_millis(),
    }
}

/// Mints the bearer each target needs from the job's own identity. No key
/// files: the access token and ID token come from the metadata server and the
/// IAP JWT from IAM `signJwt` on the probe's own service account.
pub struct Credentials {
    client: Client,
    email: String,
    endpoints: Endpoints,
}

impl Credentials {
    pub fn new(email: String, endpoints: Endpoints) -> Result<Self, String> {
        Ok(Self {
            client: api_client()?,
            email,
            endpoints,
        })
    }

    pub async fn access_token(&self) -> Result<String, String> {
        let token = bounded_json(
            self.client
                .get(format!("{}/token", self.endpoints.metadata))
                .header("Metadata-Flavor", "Google"),
        )
        .await?;
        token
            .get("access_token")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .ok_or_else(|| "metadata token response has no access_token".into())
    }

    async fn bearer(&self, target: Target, origin: &str) -> Result<HeaderValue, String> {
        let token = match target.auth() {
            AuthKind::IapJwt => self.iap_jwt(origin).await?,
            AuthKind::InvokerIdToken => self.id_token(origin).await?,
        };
        let mut value = HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(|error| format!("credential is not a header value: {error}"))?;
        value.set_sensitive(true);
        Ok(value)
    }

    async fn iap_jwt(&self, origin: &str) -> Result<String, String> {
        let access = self.access_token().await?;
        let now = unix_now();
        let payload = json!({ "iss": self.email, "sub": self.email, "aud": format!("{origin}/*"), "iat": now, "exp": now + 600 }).to_string();
        let signed = bounded_json(
            self.client
                .post(format!(
                    "{}/{}:signJwt",
                    self.endpoints.sign_jwt, self.email
                ))
                .bearer_auth(access)
                .json(&json!({ "payload": payload })),
        )
        .await?;
        signed
            .get("signedJwt")
            .and_then(Value::as_str)
            .filter(|jwt| jwt.split('.').count() == 3)
            .map(str::to_owned)
            .ok_or_else(|| "signJwt response has no signedJwt".into())
    }

    async fn id_token(&self, audience: &str) -> Result<String, String> {
        let response = self
            .client
            .get(format!("{}/identity", self.endpoints.metadata))
            .query(&[("audience", audience), ("format", "full")])
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .map_err(|error| format!("metadata identity: {error}"))?;
        if !response.status().is_success() {
            return Err(format!(
                "metadata identity answered {}",
                response.status().as_u16()
            ));
        }
        let token = response
            .text()
            .await
            .map_err(|error| format!("metadata identity body: {error}"))?;
        let token = token.trim();
        if token.len() > CREDENTIAL_LIMIT || token.split('.').count() != 3 {
            return Err("metadata identity returned no JWT".into());
        }
        Ok(token.to_owned())
    }
}

async fn bounded_json(request: reqwest::RequestBuilder) -> Result<Value, String> {
    let response = request
        .send()
        .await
        .map_err(|error| format!("credential request: {error}"))?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!("credential request answered {}", status.as_u16()));
    }
    let bytes = response
        .bytes()
        .await
        .map_err(|error| format!("credential body: {error}"))?;
    if bytes.len() > CREDENTIAL_LIMIT {
        return Err("credential body too large".into());
    }
    serde_json::from_slice(&bytes).map_err(|error| format!("credential body: {error}"))
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or_default()
}

/// Probe every configured target concurrently. A target whose credential
/// cannot be minted is an `AuthFailure` without a request being sent.
pub async fn probe_all(
    config: &Config,
    credentials: &Credentials,
    client: &Client,
) -> Vec<Observation> {
    let checks = config.targets.iter().map(|(target, origin)| async move {
        match credentials.bearer(*target, origin).await {
            Ok(bearer) => check(client, *target, origin, Some(&bearer), config.timeout).await,
            Err(error) => {
                eprintln!("{}", json!({ "severity": "WARNING", "message": "probe credential unavailable", "target": target.label(), "error": error }));
                Observation { target: *target, outcome: Outcome::AuthFailure, status: None, latency_ms: 0 }
            }
        }
    });
    // Polled together: a slow target never delays the others' clocks.
    futures::future::join_all(checks).await
}

/// The `timeSeries.create` body: one readiness point per observation and one
/// heartbeat. Resource `global` keeps one series per label set across runs.
pub fn time_series(project: &str, observations: &[Observation], end_time: &str) -> Value {
    let point = |value: i64| json!({ "interval": { "endTime": end_time }, "value": { "int64Value": value.to_string() } });
    let resource = json!({ "type": "global", "labels": { "project_id": project } });
    let mut series: Vec<Value> = observations
        .iter()
        .map(|observation| {
            json!({
                "metric": { "type": READINESS_METRIC, "labels": { "target": observation.target.label(), "outcome": observation.outcome.label() } },
                "resource": resource,
                "metricKind": "GAUGE",
                "valueType": "INT64",
                "points": [point(i64::from(observation.outcome.failed()))],
            })
        })
        .collect();
    series.push(json!({
        "metric": { "type": HEARTBEAT_METRIC, "labels": {} },
        "resource": resource,
        "metricKind": "GAUGE",
        "valueType": "INT64",
        "points": [point(1)],
    }));
    json!({ "timeSeries": series })
}

/// Write the run's points to Cloud Monitoring.
pub async fn publish(
    config: &Config,
    credentials: &Credentials,
    observations: &[Observation],
) -> Result<(), String> {
    let access = credentials.access_token().await?;
    let end_time = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let body = time_series(&config.project, observations, &end_time);
    let response = credentials
        .client
        .post(format!(
            "{}/{}/timeSeries",
            config.endpoints.monitoring, config.project
        ))
        .bearer_auth(access)
        .json(&body)
        .send()
        .await
        .map_err(|error| format!("timeSeries.create: {error}"))?;
    if !response.status().is_success() {
        return Err(format!(
            "timeSeries.create answered {}",
            response.status().as_u16()
        ));
    }
    Ok(())
}

/// One run: probe, log one structured line per target, publish.
pub async fn run(config: Config) -> Result<Vec<Observation>, String> {
    let client = probe_client(config.timeout)?;
    let credentials = Credentials::new(config.service_account.clone(), config.endpoints.clone())?;
    let observations = probe_all(&config, &credentials, &client).await;
    for observation in &observations {
        let severity = if observation.outcome.failed() {
            "WARNING"
        } else {
            "INFO"
        };
        println!(
            "{}",
            json!({
                "severity": severity,
                "message": "readiness probe",
                "target": observation.target.label(),
                "outcome": observation.outcome.label(),
                "status": observation.status,
                "latency_ms": observation.latency_ms,
            })
        );
    }
    publish(&config, &credentials, &observations).await?;
    Ok(observations)
}
