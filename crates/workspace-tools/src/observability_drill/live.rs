use std::process::Command;
use std::time::Duration;

use chrono::{DateTime, Utc};

use super::model::{DrillRoute, Signal, SignalError};

const DOPPLER_PROJECT: &str = "coda-rust";
const DOPPLER_CONFIG: &str = "stg";
const DATADOG_API_KEY: &str = "DD_API_KEY";
const DATADOG_SERIES_URL: &str = "https://api.datadoghq.com/api/v1/series";
pub(super) const SYNTHETIC_METRIC: &str = "coda.qualification.synthetic_readiness";
pub(super) const SYNTHETIC_TAG: &str = "env:staging";
const LOKI_PUSH_URL: &str = "http://127.0.0.1:3100/loki/api/v1/push";
pub(super) const PROBE_JOB: &str = "availability-probe";
pub(super) const STAGING_PROJECT: &str = "coda-rust-staging";
pub(super) const STAGING_REGION: &str = "us-east1";
pub(super) const QUALIFICATION_CRITICAL: f64 = 1.0;
pub(super) const QUALIFICATION_BELOW_RECOVERY: f64 = 0.0;

pub(super) struct Secret(String);

impl Secret {
    fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Secret(<redacted>)")
    }
}

impl std::fmt::Display for Secret {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Secret(<redacted>)")
    }
}

pub(super) struct Live {
    route: DrillRoute,
}

impl Live {
    pub(super) fn new(route: DrillRoute) -> Self {
        Self { route }
    }
}

impl Signal for Live {
    fn fail(&mut self) -> Result<(), SignalError> {
        match self.route {
            DrillRoute::Datadog => post_metric(QUALIFICATION_CRITICAL),
            DrillRoute::Grafana => push_loki("fail"),
            DrillRoute::Watchdog => reconcile_scheduler(SchedulerState::Paused, &mut ProcessShell),
        }
    }

    fn recover(&mut self) -> Result<(), SignalError> {
        match self.route {
            DrillRoute::Datadog => post_metric(QUALIFICATION_BELOW_RECOVERY),
            DrillRoute::Grafana => push_loki("ok"),
            DrillRoute::Watchdog => reconcile_scheduler(SchedulerState::Enabled, &mut ProcessShell),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SchedulerState {
    Enabled,
    Paused,
}

pub(super) struct Captured {
    pub(super) success: bool,
    pub(super) code: Option<i32>,
    pub(super) stdout: Vec<u8>,
    pub(super) stderr: Vec<u8>,
}

pub(super) trait Shell {
    fn run(&mut self, args: &[String]) -> Result<Captured, SignalError>;
}

pub(super) struct ProcessShell;

impl Shell for ProcessShell {
    fn run(&mut self, args: &[String]) -> Result<Captured, SignalError> {
        let output = Command::new("gcloud")
            .args(args)
            .output()
            .map_err(|error| {
                SignalError::new(format!("gcloud could not be started ({:?})", error.kind()))
            })?;
        Ok(Captured {
            success: output.status.success(),
            code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

pub(super) fn reconcile_scheduler(
    desired: SchedulerState,
    shell: &mut impl Shell,
) -> Result<(), SignalError> {
    let current = scheduler_state(shell)?;
    if current == desired {
        return Ok(());
    }
    let verb = match desired {
        SchedulerState::Paused => "pause",
        SchedulerState::Enabled => "resume",
    };
    let captured = shell.run(&mutate_args(verb))?;
    if captured.success {
        Ok(())
    } else {
        Err(SignalError::new(format!(
            "scheduler {verb} exited {:?}",
            captured.code
        )))
    }
}

fn scheduler_state(shell: &mut impl Shell) -> Result<SchedulerState, SignalError> {
    let captured = shell.run(&describe_args())?;
    if !captured.success {
        return Err(SignalError::new(format!(
            "scheduler describe exited {:?}",
            captured.code
        )));
    }
    match String::from_utf8_lossy(&captured.stdout).trim() {
        "ENABLED" => Ok(SchedulerState::Enabled),
        "PAUSED" => Ok(SchedulerState::Paused),
        _ => Err(SignalError::new("scheduler state was unrecognized")),
    }
}

pub(super) fn describe_args() -> Vec<String> {
    vec![
        "scheduler".to_string(),
        "jobs".to_string(),
        "describe".to_string(),
        PROBE_JOB.to_string(),
        format!("--project={STAGING_PROJECT}"),
        format!("--location={STAGING_REGION}"),
        "--format=value(state)".to_string(),
    ]
}

pub(super) fn mutate_args(verb: &str) -> Vec<String> {
    vec![
        "scheduler".to_string(),
        "jobs".to_string(),
        verb.to_string(),
        PROBE_JOB.to_string(),
        format!("--project={STAGING_PROJECT}"),
        format!("--location={STAGING_REGION}"),
    ]
}

pub(super) fn doppler_args() -> Vec<String> {
    vec![
        "secrets".to_string(),
        "get".to_string(),
        DATADOG_API_KEY.to_string(),
        "--plain".to_string(),
        "--project".to_string(),
        DOPPLER_PROJECT.to_string(),
        "--config".to_string(),
        DOPPLER_CONFIG.to_string(),
    ]
}

pub(super) fn series_body(value: f64, at: DateTime<Utc>) -> String {
    serde_json::json!({
        "series": [{
            "metric": SYNTHETIC_METRIC,
            "type": "gauge",
            "points": [[at.timestamp(), value]],
            "tags": [SYNTHETIC_TAG],
        }]
    })
    .to_string()
}

pub(super) fn loki_body(line: &str, at: DateTime<Utc>) -> String {
    let nanos = at.timestamp_nanos_opt().unwrap_or(0).to_string();
    serde_json::json!({
        "streams": [{
            "stream": {"qualification": "synthetic-notification-test"},
            "values": [[nanos, line]],
        }]
    })
    .to_string()
}

pub(super) fn secret_from_capture(
    success: bool,
    code: Option<i32>,
    stdout: &[u8],
) -> Result<Secret, SignalError> {
    if !success {
        return Err(SignalError::new(format!("Doppler exited {code:?}")));
    }
    let text = std::str::from_utf8(stdout)
        .map_err(|_error| SignalError::new("DD_API_KEY was not UTF-8"))?;
    let value = text.trim_end_matches(['\r', '\n']);
    if value.is_empty() {
        return Err(SignalError::new("DD_API_KEY is empty"));
    }
    if value.contains(['\n', '\r']) {
        return Err(SignalError::new("DD_API_KEY is not a single line"));
    }
    Ok(Secret::new(value))
}

fn fetch_api_key() -> Result<Secret, SignalError> {
    let output = Command::new("doppler")
        .args(doppler_args())
        .output()
        .map_err(|error| {
            SignalError::new(format!("Doppler could not be started ({:?})", error.kind()))
        })?;
    secret_from_capture(
        output.status.success(),
        output.status.code(),
        &output.stdout,
    )
}

fn client() -> Result<reqwest::blocking::Client, SignalError> {
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_error| SignalError::new("HTTP client could not be built"))
}

fn post_metric(value: f64) -> Result<(), SignalError> {
    let secret = fetch_api_key()?;
    let response = client()?
        .post(DATADOG_SERIES_URL)
        .header("DD-API-KEY", secret.expose())
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(series_body(value, Utc::now()))
        .send()
        .map_err(|_error| SignalError::new("Datadog series post did not connect"))?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(SignalError::new(format!(
            "Datadog series post was refused ({})",
            response.status().as_u16()
        )))
    }
}

fn push_loki(line: &str) -> Result<(), SignalError> {
    let response = client()?
        .post(LOKI_PUSH_URL)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(loki_body(line, Utc::now()))
        .send()
        .map_err(|_error| SignalError::new("Loki push did not connect"))?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(SignalError::new(format!(
            "Loki push was refused ({})",
            response.status().as_u16()
        )))
    }
}
