//! Runs one availability-probe slot as the probe's own identity, the only
//! caller IAP admits for rehearsals, and reads its published outcome back.
use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

use super::live::{ProcessShell, Shell};
use super::model::SignalError;

const PROJECT: &str = "coda-rust-staging";
const REGION: &str = "us-east1";
const JOB: &str = "availability-probe";
const READINESS_METRIC: &str = "custom.googleapis.com/coda/availability/readiness_failed";
const HEARTBEAT_METRIC: &str = "custom.googleapis.com/coda/availability/heartbeat";
const TIME_SERIES_URL: &str =
    "https://monitoring.googleapis.com/v3/projects/coda-rust-staging/timeSeries";
pub(super) const TARGETS: [&str; 3] = ["physician", "duet", "physician-worker"];
const READBACK_ATTEMPTS: u32 = 12;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub(super) struct TargetResult {
    pub(super) target: String,
    pub(super) outcome: String,
}

#[derive(Debug, Serialize)]
pub(super) struct ProbeReport {
    pub(super) project: &'static str,
    pub(super) execution: String,
    pub(super) slot: i64,
    pub(super) slot_utc: String,
    pub(super) targets: Vec<TargetResult>,
    pub(super) heartbeat: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Verdict {
    Reachable,
    AuthRefused(Vec<String>),
    Unhealthy(Vec<String>),
    Unobserved(Vec<String>),
    NoHeartbeat,
}

impl ProbeReport {
    pub(super) fn verdict(&self) -> Verdict {
        let outcome = |target: &str| {
            self.targets
                .iter()
                .find(|result| result.target == target)
                .map(|result| result.outcome.as_str())
        };
        let missing: Vec<String> = TARGETS
            .iter()
            .filter(|target| outcome(target).is_none())
            .map(|target| (*target).to_owned())
            .collect();
        let with = |wanted: &dyn Fn(&str) -> bool| -> Vec<String> {
            TARGETS
                .iter()
                .filter(|target| outcome(target).is_some_and(wanted))
                .map(|target| (*target).to_owned())
                .collect()
        };
        let refused = with(&|outcome| outcome == "auth_failure");
        let unhealthy = with(&|outcome| outcome != "success" && outcome != "auth_failure");
        if !refused.is_empty() {
            Verdict::AuthRefused(refused)
        } else if !unhealthy.is_empty() {
            Verdict::Unhealthy(unhealthy)
        } else if !missing.is_empty() {
            Verdict::Unobserved(missing)
        } else if !self.heartbeat {
            Verdict::NoHeartbeat
        } else {
            Verdict::Reachable
        }
    }
}

impl std::fmt::Display for Verdict {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reachable => formatter.write_str("Every target answered /readyz through IAP."),
            Self::AuthRefused(targets) => write!(
                formatter,
                "IAP or Cloud Run refused the probe identity for {}.",
                targets.join(", ")
            ),
            Self::Unhealthy(targets) => write!(
                formatter,
                "The probe reached {} but readiness failed.",
                targets.join(", ")
            ),
            Self::Unobserved(targets) => write!(
                formatter,
                "No readiness sample was published for {}.",
                targets.join(", ")
            ),
            Self::NoHeartbeat => formatter.write_str("No heartbeat was published for the slot."),
        }
    }
}

pub(super) fn slot_of(now: DateTime<Utc>) -> i64 {
    now.timestamp() - now.timestamp().rem_euclid(60)
}

pub(super) fn execute_args(slot: i64) -> Vec<String> {
    vec![
        "run".into(),
        "jobs".into(),
        "execute".into(),
        JOB.into(),
        format!("--project={PROJECT}"),
        format!("--region={REGION}"),
        format!("--update-env-vars=AVAILABILITY_PROBE_SLOT={slot}"),
        "--wait".into(),
        "--format=value(metadata.name)".into(),
    ]
}

pub(super) fn token_args() -> Vec<String> {
    vec!["auth".into(), "print-access-token".into()]
}

fn slot_stamp(slot: i64) -> Result<String, SignalError> {
    DateTime::<Utc>::from_timestamp(slot, 0)
        .map(|at| at.format("%Y-%m-%dT%H:%M:%SZ").to_string())
        .ok_or_else(|| SignalError::new("slot is out of range"))
}

fn points_at<'a>(series: &'a Value, stamp: &'a str) -> impl Iterator<Item = &'a Value> + 'a {
    series["points"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(move |point| point["interval"]["endTime"].as_str() == Some(stamp))
}

pub(super) fn parse_readiness(body: &Value, stamp: &str) -> Vec<TargetResult> {
    body["timeSeries"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|series| points_at(series, stamp).next().is_some())
        .filter_map(|series| {
            let labels = &series["metric"]["labels"];
            Some(TargetResult {
                target: labels["target"].as_str()?.to_owned(),
                outcome: labels["outcome"].as_str()?.to_owned(),
            })
        })
        .collect()
}

pub(super) fn parse_heartbeat(body: &Value, stamp: &str) -> bool {
    body["timeSeries"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|series| points_at(series, stamp))
        .any(|point| point["value"]["int64Value"].as_str() == Some("1"))
}

fn stdout_line(shell: &mut impl Shell, args: &[String], what: &str) -> Result<String, SignalError> {
    let captured = shell.run(args)?;
    if !captured.success {
        return Err(SignalError::new(format!(
            "{what} exited {:?}",
            captured.code
        )));
    }
    let text = String::from_utf8_lossy(&captured.stdout).trim().to_owned();
    if text.is_empty() || text.contains('\n') {
        return Err(SignalError::new(format!("{what} printed no single value")));
    }
    Ok(text)
}

fn read_series(token: &str, metric: &str, slot: i64) -> Result<Value, SignalError> {
    let start = slot_stamp(slot - 60)?;
    let end = slot_stamp(slot + 120)?;
    let filter = format!("metric.type=\"{metric}\"");
    let response = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|_error| SignalError::new("HTTP client could not be built"))?
        .get(TIME_SERIES_URL)
        .bearer_auth(token)
        .query(&[
            ("filter", filter.as_str()),
            ("interval.startTime", start.as_str()),
            ("interval.endTime", end.as_str()),
        ])
        .send()
        .map_err(|_error| SignalError::new("Cloud Monitoring read did not connect"))?;
    if !response.status().is_success() {
        return Err(SignalError::new(format!(
            "Cloud Monitoring read was refused ({})",
            response.status().as_u16()
        )));
    }
    let body = response
        .bytes()
        .map_err(|_error| SignalError::new("Cloud Monitoring answer was cut off"))?;
    serde_json::from_slice(&body)
        .map_err(|_error| SignalError::new("Cloud Monitoring answer was not JSON"))
}

pub(super) fn probe(
    now: DateTime<Utc>,
    shell: &mut impl Shell,
    read: &mut impl FnMut(&str, &str, i64) -> Result<Value, SignalError>,
    pause: &mut impl FnMut(),
) -> Result<ProbeReport, SignalError> {
    let slot = slot_of(now);
    let execution = stdout_line(shell, &execute_args(slot), "probe job")?;
    let token = stdout_line(shell, &token_args(), "gcloud access token")?;
    let stamp = slot_stamp(slot)?;
    let mut targets = Vec::new();
    let mut heartbeat = false;
    for _attempt in 0..READBACK_ATTEMPTS {
        targets = parse_readiness(&read(&token, READINESS_METRIC, slot)?, &stamp);
        heartbeat = parse_heartbeat(&read(&token, HEARTBEAT_METRIC, slot)?, &stamp);
        if heartbeat && targets.len() >= TARGETS.len() {
            break;
        }
        pause();
    }
    targets.sort_by(|left, right| left.target.cmp(&right.target));
    Ok(ProbeReport {
        project: PROJECT,
        execution,
        slot,
        slot_utc: stamp,
        targets,
        heartbeat,
    })
}

pub(super) fn from_args(args: &[String]) -> Result<PathBuf, String> {
    let [flag, dir] = args else {
        return Err("observability-drill probe --output-dir DIR".into());
    };
    if flag != "--output-dir" {
        return Err("observability-drill probe --output-dir DIR".into());
    }
    let report = probe(
        Utc::now(),
        &mut ProcessShell,
        &mut |token, metric, slot| read_series(token, metric, slot),
        &mut || std::thread::sleep(Duration::from_secs(10)),
    )
    .map_err(|error| error.to_string())?;
    let path = write_report(Path::new(dir), &report)?;
    match report.verdict() {
        Verdict::Reachable => Ok(path),
        verdict => Err(format!("{verdict} Wrote {}.", path.display())),
    }
}

fn write_report(dir: &Path, report: &ProbeReport) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dir).map_err(|error| error.to_string())?;
    let path = dir.join(format!("probe-{}.json", report.slot));
    let mut body = serde_json::to_string_pretty(report).map_err(|error| error.to_string())?;
    body.push('\n');
    std::fs::write(&path, body).map_err(|error| error.to_string())?;
    Ok(path)
}
