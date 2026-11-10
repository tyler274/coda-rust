//! Authenticated, bounded readiness checks. Only fixed operational data leaves
//! the process. Redirects are authentication failures, never application success.
pub mod delivery;
pub mod dispatch;
pub mod error;
pub mod state;
pub use error::Error;
use reqwest::{Client, StatusCode, header::HeaderValue, redirect::Policy};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const READINESS_METRIC: &str = "custom.googleapis.com/coda/availability/readiness_failed";
pub const HEARTBEAT_METRIC: &str = "custom.googleapis.com/coda/availability/heartbeat";
pub const ALERT_METRIC: &str = "custom.googleapis.com/coda/availability/firing";
pub const LATENCY_METRIC: &str = "custom.googleapis.com/coda/availability/latency_ms";
pub const FRESHNESS_METRIC: &str = "custom.googleapis.com/coda/availability/observed_at";
const CREDENTIAL_LIMIT: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Target {
    Physician,
    Duet,
    PhysicianWorker,
}
impl Target {
    pub const ALL: [Self; 3] = [Self::Physician, Self::Duet, Self::PhysicianWorker];
    pub fn label(self) -> &'static str {
        match self {
            Self::Physician => "physician",
            Self::Duet => "duet",
            Self::PhysicianWorker => "physician-worker",
        }
    }
    fn url_key(self) -> &'static str {
        match self {
            Self::Physician => "AVAILABILITY_PROBE_PHYSICIAN_URL",
            Self::Duet => "AVAILABILITY_PROBE_DUET_URL",
            Self::PhysicianWorker => "AVAILABILITY_PROBE_WORKER_URL",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Success,
    AuthFailure,
    TargetFailure,
    Timeout,
    Refused,
}
impl Outcome {
    pub fn label(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::AuthFailure => "auth_failure",
            Self::TargetFailure => "target_failure",
            Self::Timeout => "timeout",
            Self::Refused => "refused",
        }
    }
    pub fn failed(self) -> bool {
        self != Self::Success
    }
}
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    pub target: Target,
    pub outcome: Outcome,
    pub status: Option<u16>,
    pub latency_ms: u128,
}
#[derive(Clone, Debug)]
pub struct Config {
    pub project: String,
    pub state_bucket: Option<String>,
    pub service_account: String,
    pub targets: Vec<(Target, String)>,
    pub timeout: Duration,
    pub endpoints: Endpoints,
}
#[derive(Clone, Debug)]
pub struct Endpoints {
    pub metadata: String,
    pub sign_jwt: String,
    pub monitoring: String,
    pub storage: String,
}
impl Default for Endpoints {
    fn default() -> Self {
        Self {
            metadata: "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default".into(),
            sign_jwt: http::signjwt::IAM_CREDENTIALS.into(),
            monitoring: "https://monitoring.googleapis.com/v3/projects".into(),
            storage: "https://storage.googleapis.com".into(),
        }
    }
}
impl Config {
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, Error> {
        let required = |key: &str| {
            get(key)
                .filter(|v| !v.trim().is_empty())
                .ok_or_else(|| Error::from(format!("{key} is required")))
        };
        let project = required("AVAILABILITY_PROBE_PROJECT")?;
        let service_account = required("AVAILABILITY_PROBE_SERVICE_ACCOUNT")?;
        let mut targets = Vec::new();
        for target in Target::ALL {
            if let Some(url) = get(target.url_key()).filter(|v| !v.trim().is_empty()) {
                let url = url.trim().trim_end_matches('/').to_owned();
                let parsed = reqwest::Url::parse(&url)
                    .map_err(|e| Error::caused("invalid target origin", e))?;
                if parsed.scheme() != "https"
                    || parsed.host_str().is_none()
                    || !parsed.username().is_empty()
                    || parsed.password().is_some()
                    || parsed.path() != "/"
                    || parsed.query().is_some()
                    || parsed.fragment().is_some()
                {
                    return Err(Error::from(format!(
                        "{} must be an https origin",
                        target.url_key()
                    )));
                }
                targets.push((target, url));
            }
        }
        if targets.is_empty() {
            return Err("no AVAILABILITY_PROBE_*_URL target is configured".into());
        }
        let seconds = get("AVAILABILITY_PROBE_TIMEOUT_SECONDS")
            .map(|v| v.parse::<u64>())
            .transpose()
            .map_err(|e| Error::caused("timeout must be whole seconds", e))?
            .unwrap_or(30);
        if !(1..=45).contains(&seconds) {
            return Err("timeout must be between 1 and 45 seconds".into());
        }
        Ok(Self {
            project,
            state_bucket: Some(required("AVAILABILITY_PROBE_STATE_BUCKET")?),
            service_account,
            targets,
            timeout: Duration::from_secs(seconds),
            endpoints: Endpoints::default(),
        })
    }
}
pub fn probe_client(timeout: Duration) -> Result<Client, Error> {
    Client::builder()
        .redirect(Policy::none())
        .timeout(timeout)
        .connect_timeout(timeout)
        .build()
        .map_err(|e| Error::caused("HTTP client construction failed", e))
}
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
    let answer = tokio::time::timeout(timeout + Duration::from_secs(1), request.send()).await;
    let (outcome, status) = match answer {
        Err(_) => (Outcome::Timeout, None),
        Ok(Err(e)) => (classify_error(&e), None),
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
pub struct Credentials {
    pub(crate) client: Client,
    email: String,
    endpoints: Endpoints,
}
impl Credentials {
    pub fn new(email: String, endpoints: Endpoints) -> Result<Self, Error> {
        Ok(Self {
            client: probe_client(Duration::from_secs(10))?,
            email,
            endpoints,
        })
    }
    pub async fn access_token(&self) -> Result<String, Error> {
        let value = bounded_json(
            self.client
                .get(format!("{}/token", self.endpoints.metadata))
                .header("Metadata-Flavor", "Google"),
        )
        .await?;
        value["access_token"]
            .as_str()
            .filter(|v| !v.is_empty())
            .map(str::to_owned)
            .ok_or_else(|| "metadata token response has no access_token".into())
    }
    async fn bearer(&self, target: Target, origin: &str) -> Result<HeaderValue, Error> {
        let token = if target == Target::PhysicianWorker {
            self.id_token(origin).await?
        } else {
            self.iap_jwt(origin).await?
        };
        let mut value = HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(|e| Error::caused("invalid credential header", e))?;
        value.set_sensitive(true);
        Ok(value)
    }
    async fn iap_jwt(&self, origin: &str) -> Result<String, Error> {
        let access = self.access_token().await?;
        let call =
            http::signjwt::iap_request(&self.endpoints.sign_jwt, &self.email, origin, unix_now());
        let signed = bounded_json(
            self.client
                .post(call.url)
                .bearer_auth(access)
                .json(&call.body),
        )
        .await?;
        http::signjwt::signed_jwt(&signed)
            .map(str::to_owned)
            .ok_or_else(|| "signJwt response has no signedJwt".into())
    }
    async fn id_token(&self, origin: &str) -> Result<String, Error> {
        let response = self
            .client
            .get(format!("{}/identity", self.endpoints.metadata))
            .query(&[("audience", origin), ("format", "full")])
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .map_err(|e| Error::caused("metadata identity request failed", e))?;
        let bytes = bounded_body(response, CREDENTIAL_LIMIT).await?;
        let token = std::str::from_utf8(&bytes)
            .map_err(|e| Error::caused("identity decoding failed", e))?
            .trim();
        if token.split('.').count() != 3 {
            return Err("metadata identity returned no JWT".into());
        }
        Ok(token.into())
    }
}
pub(crate) async fn bounded_body(
    mut response: reqwest::Response,
    limit: usize,
) -> Result<Vec<u8>, Error> {
    if !response.status().is_success() {
        return Err(Error::status("API request", response.status()));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|e| Error::caused("API body read failed", e))?
    {
        if bytes.len().saturating_add(chunk.len()) > limit {
            return Err("API body too large".into());
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}
pub(crate) async fn bounded_json(request: reqwest::RequestBuilder) -> Result<Value, Error> {
    let response = request
        .send()
        .await
        .map_err(|e| Error::caused("API request failed", e))?;
    let bytes = bounded_body(response, CREDENTIAL_LIMIT).await?;
    serde_json::from_slice(&bytes).map_err(|e| Error::caused("API JSON decoding failed", e))
}
pub(crate) fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default()
}
pub async fn probe_all(
    config: &Config,
    credentials: &Credentials,
    client: &Client,
) -> Vec<Observation> {
    futures::future::join_all(config.targets.iter().map(|(target, origin)| async move {
        match credentials.bearer(*target, origin).await {
            Ok(bearer) => check(client, *target, origin, Some(&bearer), config.timeout).await,
            Err(error) => {
                eprintln!("{}", json!({ "severity": "WARNING", "message": "probe credential unavailable", "target": target.label(), "error": error.to_string() }));
                Observation { target: *target, outcome: Outcome::AuthFailure, status: None, latency_ms: 0 }
            }
        }
    })).await
}
pub fn time_series(project: &str, observations: &[Observation], end_time: &str) -> Value {
    let mut series: Vec<Value> = observations
        .iter()
        .map(|o| {
            metric_point(
                project,
                READINESS_METRIC,
                json!({ "target": o.target.label(), "outcome": o.outcome.label() }),
                i64::from(o.outcome.failed()),
                end_time,
            )
        })
        .collect();
    series.push(metric_point(
        project,
        HEARTBEAT_METRIC,
        json!({}),
        1,
        end_time,
    ));
    json!({ "timeSeries": series })
}
pub(crate) fn metric_point(
    project: &str,
    metric: &str,
    labels: Value,
    value: i64,
    end_time: &str,
) -> Value {
    json!({ "metric": { "type": metric, "labels": labels }, "resource": { "type": "global", "labels": { "project_id": project } }, "metricKind": "GAUGE", "valueType": "INT64", "points": [{ "interval": { "endTime": end_time }, "value": { "int64Value": value.to_string() } }] })
}
/// Synthetic/stateless harness only. Production uses run_slot and durable state.
pub async fn run(config: Config) -> Result<Vec<Observation>, Error> {
    if config.state_bucket.is_some() {
        return Err("durable execution requires an explicit scheduler slot".into());
    }
    let client = probe_client(config.timeout)?;
    let credentials = Credentials::new(config.service_account.clone(), config.endpoints.clone())?;
    let observations = probe_all(&config, &credentials, &client).await;
    let end_time = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let body = time_series(&config.project, &observations, &end_time);
    let access = credentials.access_token().await?;
    for series in body["timeSeries"].as_array().ok_or("missing time series")? {
        delivery::send_point(&config, &credentials, &access, series).await?;
    }
    Ok(observations)
}
