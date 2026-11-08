//! Runs one availability-probe slot as the probe's own identity, the only
//! caller IAP admits for rehearsals, and reads its published outcome back.
use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

use super::live::{PROBE_JOB, ProcessShell, STAGING_PROJECT, STAGING_REGION, Shell};
use super::model::SignalError;
use crate::failure::{Failure, api_status};

// The availability-probe crate publishes these names (its lib.rs). The
// readback fails closed on any drift: an unknown outcome counts as unhealthy
// and a missing series as unobserved.
const READINESS_METRIC: &str = "custom.googleapis.com/coda/availability/readiness_failed";
const HEARTBEAT_METRIC: &str = "custom.googleapis.com/coda/availability/heartbeat";
pub(super) const TARGETS: [&str; 3] = ["physician", "duet", "physician-worker"];
const SUCCESS: &str = "success";
const AUTH_FAILURE: &str = "auth_failure";
const READBACK_ATTEMPTS: u32 = 12;
const USAGE: &str = "observability-drill probe --output-dir DIR";

#[derive(Debug, thiserror::Error)]
pub(super) enum ProbeError {
    #[error("gcloud {what} could not run")]
    Shell {
        what: &'static str,
        #[source]
        source: SignalError,
    },
    #[error("gcloud {what} exited {code:?} ({})", status.unwrap_or("no API status"))]
    Exited {
        what: &'static str,
        code: Option<i32>,
        status: Option<&'static str>,
    },
    #[error("gcloud {what} printed no single value")]
    NoValue { what: &'static str },
    #[error("slot {0} is out of range")]
    Slot(i64),
    #[error("Cloud Monitoring read failed")]
    Read(#[source] reqwest::Error),
    #[error("Cloud Monitoring refused the read ({0})")]
    Refused(u16),
    #[error("Cloud Monitoring answer was not JSON")]
    Decode(#[source] serde_json::Error),
    #[error("could not write {}", path.display())]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

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
    fn outcome(&self, target: &str) -> Option<&str> {
        self.targets
            .iter()
            .find(|result| result.target == target)
            .map(|result| result.outcome.as_str())
    }

    pub(super) fn verdict(&self) -> Verdict {
        let mut refused = Vec::new();
        let mut unhealthy = Vec::new();
        let mut missing = Vec::new();
        for target in TARGETS {
            match self.outcome(target) {
                None => missing.push(target.to_owned()),
                Some(SUCCESS) => {}
                Some(AUTH_FAILURE) => refused.push(target.to_owned()),
                Some(outcome) => unhealthy.push(format!("{target}: {outcome}")),
            }
        }
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
            Self::Unhealthy(outcomes) => {
                write!(formatter, "Readiness failed for {}.", outcomes.join(", "))
            }
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
        PROBE_JOB.into(),
        format!("--project={STAGING_PROJECT}"),
        format!("--region={STAGING_REGION}"),
        format!("--update-env-vars=AVAILABILITY_PROBE_SLOT={slot}"),
        "--wait".into(),
        "--format=value(metadata.name)".into(),
    ]
}

fn token_args() -> Vec<String> {
    vec!["auth".into(), "print-access-token".into()]
}

fn slot_stamp(slot: i64) -> Result<String, ProbeError> {
    DateTime::<Utc>::from_timestamp(slot, 0)
        .map(|at| at.format("%Y-%m-%dT%H:%M:%SZ").to_string())
        .ok_or(ProbeError::Slot(slot))
}

fn points_at<'a>(series: &'a Value, stamp: &'a str) -> impl Iterator<Item = &'a Value> + 'a {
    series["points"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(move |point| point["interval"]["endTime"].as_str() == Some(stamp))
}

fn parse_readiness(body: &Value, stamp: &str) -> Vec<TargetResult> {
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

fn parse_heartbeat(body: &Value, stamp: &str) -> bool {
    body["timeSeries"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|series| points_at(series, stamp))
        .any(|point| point["value"]["int64Value"].as_str() == Some("1"))
}

fn stdout_line(
    shell: &mut impl Shell,
    args: &[String],
    what: &'static str,
) -> Result<String, ProbeError> {
    let captured = shell
        .run(args)
        .map_err(|source| ProbeError::Shell { what, source })?;
    if !captured.success {
        return Err(ProbeError::Exited {
            what,
            code: captured.code,
            status: api_status(&captured.stderr),
        });
    }
    let text = String::from_utf8_lossy(&captured.stdout).trim().to_owned();
    if text.is_empty() || text.contains('\n') {
        return Err(ProbeError::NoValue { what });
    }
    Ok(text)
}

fn read_series(token: &str, metric: &str, slot: i64) -> Result<Value, ProbeError> {
    let start = slot_stamp(slot - 60)?;
    let end = slot_stamp(slot + 120)?;
    let filter = format!("metric.type=\"{metric}\"");
    let response = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(ProbeError::Read)?
        .get(format!(
            "https://monitoring.googleapis.com/v3/projects/{STAGING_PROJECT}/timeSeries"
        ))
        .bearer_auth(token)
        .query(&[
            ("filter", filter.as_str()),
            ("interval.startTime", start.as_str()),
            ("interval.endTime", end.as_str()),
        ])
        .send()
        .map_err(ProbeError::Read)?;
    if !response.status().is_success() {
        return Err(ProbeError::Refused(response.status().as_u16()));
    }
    let body = response.bytes().map_err(ProbeError::Read)?;
    serde_json::from_slice(&body).map_err(ProbeError::Decode)
}

/// Reads both series for the slot. A read error counts as an unfinished
/// attempt, because the slot is already spent; the last error surfaces only
/// when every attempt failed to read.
pub(super) fn probe(
    now: DateTime<Utc>,
    shell: &mut impl Shell,
    read: &mut impl FnMut(&str, &str, i64) -> Result<Value, ProbeError>,
    pause: &mut impl FnMut(),
) -> Result<ProbeReport, ProbeError> {
    let slot = slot_of(now);
    let execution = stdout_line(shell, &execute_args(slot), "probe job")?;
    let token = stdout_line(shell, &token_args(), "access token")?;
    let stamp = slot_stamp(slot)?;
    let mut read_slot = || -> Result<(Vec<TargetResult>, bool), ProbeError> {
        let readiness = read(&token, READINESS_METRIC, slot)?;
        let heartbeat = read(&token, HEARTBEAT_METRIC, slot)?;
        Ok((
            parse_readiness(&readiness, &stamp),
            parse_heartbeat(&heartbeat, &stamp),
        ))
    };
    let mut latest = read_slot();
    for _attempt in 1..READBACK_ATTEMPTS {
        if let Ok((targets, true)) = &latest
            && targets.len() >= TARGETS.len()
        {
            break;
        }
        pause();
        // A later read error keeps the last observation as the evidence.
        latest = match (read_slot(), latest) {
            (Ok(observed), _) | (Err(_), Ok(observed)) => Ok(observed),
            (Err(error), Err(_)) => Err(error),
        };
    }
    let (mut targets, heartbeat) = latest?;
    targets.sort_by(|left, right| left.target.cmp(&right.target));
    Ok(ProbeReport {
        project: STAGING_PROJECT,
        execution,
        slot,
        slot_utc: stamp,
        targets,
        heartbeat,
    })
}

pub(super) fn from_args(args: &[String]) -> Result<(), Failure> {
    let [flag, dir] = args else {
        return Err(Failure::new(2, USAGE));
    };
    if flag != "--output-dir" {
        return Err(Failure::new(2, USAGE));
    }
    let report = probe(
        Utc::now(),
        &mut ProcessShell,
        &mut |token, metric, slot| read_series(token, metric, slot),
        &mut || std::thread::sleep(Duration::from_secs(10)),
    )
    .map_err(failure)?;
    let path = write_report(Path::new(dir), &report).map_err(failure)?;
    match report.verdict() {
        Verdict::Reachable => {
            println!("{} Wrote {}.", Verdict::Reachable, path.display());
            Ok(())
        }
        verdict => Err(Failure::new(
            1,
            format!("{verdict} Wrote {}.", path.display()),
        )),
    }
}

/// The operator sees the whole cause chain; the typed error stays the source.
fn failure(error: ProbeError) -> Failure {
    let mut message = error.to_string();
    let mut cause = std::error::Error::source(&error);
    while let Some(inner) = cause {
        message.push_str(&format!(": {inner}"));
        cause = inner.source();
    }
    Failure::with_source(1, message, error)
}

fn write_report(dir: &Path, report: &ProbeReport) -> Result<PathBuf, ProbeError> {
    let path = dir.join(format!("probe-{}.json", report.slot));
    let written = std::fs::create_dir_all(dir).and_then(|()| {
        let mut body = serde_json::to_string_pretty(report).map_err(std::io::Error::other)?;
        body.push('\n');
        std::fs::write(&path, body)
    });
    written.map_err(|source| ProbeError::Write {
        path: path.clone(),
        source,
    })?;
    Ok(path)
}
