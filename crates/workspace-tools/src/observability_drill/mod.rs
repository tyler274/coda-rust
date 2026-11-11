mod langfuse;
mod live;
mod model;
mod probe;
#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};

use crate::failure::Failure;
use live::Live;
use model::{
    Breach, DrillAction, DrillRequest, DrillRoute, Exercise, Signal, SignalError, SignalKind,
    parse_exercise,
};

const USAGE: &str = "\
observability-drill datadog|grafana|watchdog fail|recover|status --output-dir DIR
observability-drill probe --output-dir DIR
observability-drill langfuse rehearse|alert-to-run|isolation|chat-failure --output-dir DIR

Probe runs one availability-probe slot as the probe identity, which IAP admits,
reads each target's readiness outcome and the heartbeat back from Cloud
Monitoring, writes probe-SLOT.json, and fails unless every target succeeded.
Workstation identity tokens carry gcloud's audience, which IAP refuses.

Datadog reads DD_API_KEY from Doppler project coda-rust config stg and does not print it.
Grafana fail expects a port-forward to Loki on 127.0.0.1:3100.
Watchdog pauses or resumes the availability-probe scheduler in coda-rust-staging.
Langfuse drives a rehearsal chat as the physician rehearsal identity and reads
it back from Langfuse and the operations backends; see `langfuse --help`.
Each run writes observation JSON for dev/observability/qualify-alerts.py --observations.
A run stops at 40 minutes and 2 reminders, and restores the signal if it aborts.";

pub fn from_args(args: &[String]) -> Result<(), Failure> {
    if let Some(("probe", rest)) = args.split_first().map(|(head, tail)| (head.as_str(), tail)) {
        return probe::from_args(rest);
    }
    if let Some(("langfuse", rest)) = args.split_first().map(|(head, tail)| (head.as_str(), tail)) {
        return langfuse::from_args(rest);
    }
    let (request, output) = parse_args(args).map_err(|message| Failure::new(2, message))?;
    let mut live = Live::new(request.route);
    match run(request, &output, Utc::now(), &mut live) {
        Ok(Some(path)) => {
            println!("wrote {}", path.display());
            Ok(())
        }
        Ok(None) => {
            println!("no observations");
            Ok(())
        }
        Err(error) => Err(Failure::new(1, error.to_string())),
    }
}

fn parse_args(args: &[String]) -> Result<(DrillRequest, PathBuf), String> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Err(USAGE.to_string());
    }
    let mut route = None;
    let mut action = None;
    let mut output = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        if arg == "--output-dir" {
            if output.is_some() {
                return Err(format!("unexpected argument {arg}\n{USAGE}"));
            }
            let path = rest
                .next()
                .ok_or_else(|| format!("--output-dir needs a path\n{USAGE}"))?;
            output = Some(PathBuf::from(path));
            continue;
        }
        if route.is_none() {
            route = Some(
                DrillRoute::parse(arg).ok_or_else(|| format!("unknown route {arg}\n{USAGE}"))?,
            );
            continue;
        }
        if action.is_none() {
            action = Some(
                DrillAction::parse(arg).ok_or_else(|| format!("unknown action {arg}\n{USAGE}"))?,
            );
            continue;
        }
        return Err(format!("unexpected argument {arg}\n{USAGE}"));
    }
    let route = route.ok_or_else(|| USAGE.to_string())?;
    let action = action.ok_or_else(|| USAGE.to_string())?;
    let output = output.ok_or_else(|| format!("--output-dir is required\n{USAGE}"))?;
    Ok((DrillRequest { route, action }, output))
}

#[derive(Debug)]
struct RunError {
    kind: RunErrorKind,
    wrote: Option<PathBuf>,
}

#[derive(Debug)]
enum RunErrorKind {
    Cap(Breach),
    Effect(SignalError),
    Restore(SignalError),
    Ledger(String),
    Io(std::io::Error),
}

impl RunError {
    fn cap(breach: Breach) -> Self {
        Self {
            kind: RunErrorKind::Cap(breach),
            wrote: None,
        }
    }

    fn effect(error: SignalError) -> Self {
        Self {
            kind: RunErrorKind::Effect(error),
            wrote: None,
        }
    }

    fn restore(error: SignalError) -> Self {
        Self {
            kind: RunErrorKind::Restore(error),
            wrote: None,
        }
    }

    fn ledger(message: impl Into<String>) -> Self {
        Self {
            kind: RunErrorKind::Ledger(message.into()),
            wrote: None,
        }
    }

    fn io(error: std::io::Error) -> Self {
        Self {
            kind: RunErrorKind::Io(error),
            wrote: None,
        }
    }

    fn with_wrote(mut self, wrote: Option<PathBuf>) -> Self {
        self.wrote = wrote;
        self
    }
}

impl std::fmt::Display for RunError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            RunErrorKind::Cap(Breach::Duration) => {
                formatter.write_str("The drill exceeded 40 minutes. The signal was restored.")?
            }
            RunErrorKind::Cap(Breach::Reminders) => {
                formatter.write_str("The drill exceeded 2 reminders. The signal was restored.")?
            }
            RunErrorKind::Effect(error) => write!(formatter, "{error}")?,
            RunErrorKind::Restore(error) => {
                write!(formatter, "The signal could not be restored. {error}")?
            }
            RunErrorKind::Ledger(message) => formatter.write_str(message)?,
            RunErrorKind::Io(error) => {
                write!(formatter, "Could not write the observation file. {error}")?
            }
        }
        if let Some(path) = &self.wrote {
            write!(formatter, " Wrote {}.", path.display())?;
        }
        Ok(())
    }
}

fn run(
    request: DrillRequest,
    output_dir: &Path,
    now: DateTime<Utc>,
    signal: &mut impl Signal,
) -> Result<Option<PathBuf>, RunError> {
    let mut exercise = match load_open(output_dir, request.route) {
        Ok(exercise) => exercise,
        Err(error) => {
            signal.recover().map_err(RunError::restore)?;
            return Err(RunError::ledger(error));
        }
    };
    let file_name = observation_file_name(request.route, now);
    let acted = act(&mut exercise, request.action, now, &file_name, signal);
    let wrote = write_observations(output_dir, &file_name, &exercise)?;
    match acted {
        Ok(()) => Ok(wrote),
        Err(error) => Err(error.with_wrote(wrote)),
    }
}

fn act(
    exercise: &mut Exercise,
    action: DrillAction,
    now: DateTime<Utc>,
    file_name: &str,
    signal: &mut impl Signal,
) -> Result<(), RunError> {
    if matches!(action, DrillAction::Fail | DrillAction::Status)
        && let Some(breach) = exercise.breach(now)
    {
        abort(exercise, now, file_name, signal)?;
        return Err(RunError::cap(breach));
    }
    match action {
        DrillAction::Fail => match signal.fail() {
            Ok(()) => {
                if !exercise.is_open() {
                    exercise.record_signal(SignalKind::Failure, now, file_name);
                }
                Ok(())
            }
            Err(error) => {
                abort(exercise, now, file_name, signal)?;
                Err(RunError::effect(error))
            }
        },
        DrillAction::Recover => {
            signal.recover().map_err(RunError::restore)?;
            if exercise.is_open() {
                exercise.record_signal(SignalKind::Restored, now, file_name);
            }
            Ok(())
        }
        DrillAction::Status => Ok(()),
    }
}

fn abort(
    exercise: &mut Exercise,
    now: DateTime<Utc>,
    file_name: &str,
    signal: &mut impl Signal,
) -> Result<(), RunError> {
    signal.recover().map_err(RunError::restore)?;
    if exercise.is_open() {
        exercise.record_signal(SignalKind::Restored, now, file_name);
    }
    Ok(())
}

fn load_open(dir: &Path, route: DrillRoute) -> Result<Exercise, String> {
    let exercise = load_latest(dir, route)?;
    if exercise.is_open() {
        Ok(exercise)
    } else {
        Ok(Exercise::idle(route))
    }
}

/// The route's latest exercise, open or closed.
fn load_latest(dir: &Path, route: DrillRoute) -> Result<Exercise, String> {
    let Some(path) = latest_observation(dir, route).map_err(|error| error.to_string())? else {
        return Ok(Exercise::idle(route));
    };
    let text = std::fs::read_to_string(&path).map_err(|error| error.to_string())?;
    parse_exercise(route, &text)
}

fn latest_observation(dir: &Path, route: DrillRoute) -> Result<Option<PathBuf>, std::io::Error> {
    if !dir.exists() {
        return Ok(None);
    }
    let prefix = format!("observations-{}-", route.slug());
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with(&prefix) && name.ends_with(".json") {
            paths.push(entry.path());
        }
    }
    paths.sort();
    Ok(paths.pop())
}

fn observation_file_name(route: DrillRoute, now: DateTime<Utc>) -> String {
    format!(
        "observations-{}-{}.json",
        route.slug(),
        now.format("%Y%m%dT%H%M%SZ")
    )
}

fn write_observations(
    dir: &Path,
    file_name: &str,
    exercise: &Exercise,
) -> Result<Option<PathBuf>, RunError> {
    if exercise.is_empty() {
        return Ok(None);
    }
    std::fs::create_dir_all(dir).map_err(RunError::io)?;
    let path = dir.join(file_name);
    let mut body = exercise
        .to_json()
        .map_err(|error| RunError::ledger(error.to_string()))?;
    if !body.ends_with('\n') {
        body.push('\n');
    }
    std::fs::write(&path, body).map_err(RunError::io)?;
    Ok(Some(path))
}
