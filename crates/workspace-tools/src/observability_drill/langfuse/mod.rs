//! `observability-drill langfuse`: drives a rehearsal chat turn on staging as
//! the physician rehearsal identity and reads it back from Langfuse and the
//! operations backends.
//!
//! Every check writes `langfuse-<action>-<stamp>.json` with the deployed
//! revisions and the Langfuse host it ran against. `chat-failure` also writes
//! the alert ledger `observations-langfuse-<stamp>.json` for
//! `qualify-alerts.py`. Every wait runs under the exercise's 40-minute cap
//! and the operator's interrupt.
mod chat;
mod cluster;
mod contention;
mod readback;
mod watch;

use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use url::Url;

use super::live::{STAGING_PROJECT, Secret, fetch_secret};
use super::model::{DrillRoute, EvalKind, Exercise, SignalError, SignalKind};
use crate::failure::Failure;
use chat::{Physician, Turn};
use cluster::Forward;
use readback::{AttemptSpan, LangfuseTurn, LogIsolation, MonitorGroup, NamedMonitor, OpsSpan};
use watch::Watch;

const USAGE: &str = "\
observability-drill langfuse ACTION --output-dir DIR [--scenario ID] [--backend datadog|grafana|both] [--confirm]

ACTION
  rehearse           send a canary turn; Langfuse must hold it as simulation,
                     with the scenario, session = chat id, coda.run_id and trace id
  alert-to-run       the same turn found from Datadog: the snapshot's attempt id,
                     the attempt span, the generation and Langfuse agree on
                     coda.run_id and the trace id
  isolation          Langfuse holds the canary; the operations spans and logs of
                     the same trace exist and hold no canary
  chat-failure       send threshold + 1 canary turns marked [fail:1] (needs
                     --confirm, it pages): each answers on its retry, the
                     physician chat failure monitor fires, one failure is
                     followed from the monitor's query to the failed attempt,
                     its successor in the same work and its ERROR generation
                     in Langfuse, Datadog holds no canary, and the monitor
                     recovers
  burst              one unmarked turn for the idle baseline, then two
                     sequential waves of 12 turns marked [hold:20]; each turn's
                     submit, attempt, attempt-span start and duration, hold and
                     reply are recorded, a missing sample is listed as missing,
                     and each wave must drain within
                     ceil(12 / (4 x worker instances)) x 20 s + 10 s
  hard-stop          (needs --confirm, it aborts a worker instance) refuses
                     unless the queue gauges show nothing due, sends one turn
                     marked [hold:30][crash:1] and waits at most five minutes:
                     the answering attempt must succeed the crashed one in the
                     same work, the crashed attempt must have logged
                     physician.worker.rehearsed_crash, and the chat must hold
                     exactly one reply; a normal rehearse turn ends it

The operator runs it with gcloud signed in as a physician_rehearsal_signers
account and the rust-observability kubeconfig context; kubectl always names
that context. Tempo and Loki are reached through port-forwards the drill opens
itself; Langfuse is the hosted project at LANGFUSE_BASE_URL. Doppler
coda-rust/stg supplies LANGFUSE_BASE_URL, LANGFUSE_PUBLIC_KEY,
LANGFUSE_SECRET_KEY, DD_API_KEY and DD_APP_KEY; no key is printed.";

const DATADOG_API: &str = "https://api.datadoghq.com";
const DATADOG_ENV: &str = "staging";
const REPLY_POLLS: u32 = 60;
const READBACK_POLLS: u32 = 36;
const MONITOR_POLLS: u32 = 60;
const POLL: Duration = Duration::from_secs(10);
const REPLY_PAUSE: Duration = Duration::from_secs(5);
/// Turns in each `burst` wave.
const BURST_TURNS: u32 = 12;
/// What each `burst` wave turn is held for, and the marker that holds it.
const BURST_HOLD: Duration = Duration::from_secs(20);
const BURST_MARKER: &str = " [hold:20]";
/// What the `hard-stop` turn carries: the first attempt holds its lease for
/// 30 seconds, then the worker process aborts.
const HARD_STOP_MARKER: &str = " [hold:30][crash:1]";
/// `burst` reads each snapshot this often, so a reply's observed time stays
/// within a few seconds of the drain bound's 10 s slack.
const BURST_REPLY: (u32, Duration) = (150, Duration::from_secs(2));
/// How long `hard-stop` waits for the successor's reply: the 60-second lease
/// expiry, discovery and the successor's own hold, within five minutes.
const HARD_STOP_REPLY: (u32, Duration) = (60, REPLY_PAUSE);
const CLOUD_LOGGING: &str = "https://logging.googleapis.com/v2/entries:list";
const CLOUD_MONITORING: &str = "https://monitoring.googleapis.com/v3/projects";
/// The worker's queue gauge of turns runnable now
/// (`physician-worker/signals.rs`).
const DUE_METRIC: &str = "custom.googleapis.com/coda/physician/queue/due";
/// `chat_generation_errors` in `infra/gcp/observability/datadog.tf`.
const CHAT_FAILURE_MONITOR: &str = "[staging] physician chat generation errors";
/// The ledger group of a `chat-failure` exercise, apart from the exporter's.
const CHAT_FAILURE_GROUP: &str = "staging/physician-chat-failure";
/// What `chat-failure` appends to each turn: the worker fails the first
/// attempt of a rehearsal turn so marked, and its retry answers.
const FAIL_ONCE: &str = " [fail:1]";
/// The most marked turns `chat-failure` sends, whatever the threshold says.
const MAX_FAILING_TURNS: u32 = 20;
/// The chat failure monitor counts errors over 10 minutes, so its recovery
/// comes at least that long after the last failed attempt.
const CHAT_RECOVERY_POLLS: u32 = 120;

#[derive(Debug, thiserror::Error)]
pub(super) enum DrillError {
    #[error("{what} could not be read")]
    Request {
        what: &'static str,
        #[source]
        source: reqwest::Error,
    },
    #[error("{what} answered HTTP {status}")]
    Status { what: &'static str, status: u16 },
    #[error("{what} answered without {field}")]
    Shape {
        what: &'static str,
        field: &'static str,
    },
    #[error("the physician site rejected the turn ({0})")]
    Rejected(String),
    #[error("the rehearsal turn failed (run {run})")]
    TurnFailed { run: String },
    #[error("{what} did not arrive within {seconds} seconds")]
    Timeout { what: &'static str, seconds: u64 },
    #[error("all {polls} polls failed; the last could not reach its backend")]
    PollsFailed {
        polls: u32,
        #[source]
        last: Box<DrillError>,
    },
    #[error("{what} failed")]
    Io {
        what: &'static str,
        #[source]
        source: std::io::Error,
    },
    #[error("{what} exited {code:?}")]
    Exited {
        what: &'static str,
        code: Option<i32>,
    },
    #[error("{what}: gcloud exited {code:?} ({status})")]
    Gcloud {
        what: &'static str,
        code: Option<i32>,
        status: String,
    },
    #[error("kubectl port-forward to {0} printed no local port")]
    NotForwarded(&'static str),
    #[error("LANGFUSE_BASE_URL is not a bare https origin")]
    LangfuseHost(#[source] LangfuseHostError),
    #[error("{what} could not run")]
    Tool {
        what: &'static str,
        #[source]
        source: SignalError,
    },
    #[error("{action} needs --confirm: {why}")]
    Unconfirmed {
        action: &'static str,
        why: &'static str,
    },
    #[error("{what}: {}", gaps.join("; "))]
    Gaps {
        what: &'static str,
        gaps: Vec<String>,
    },
    #[error("the exercise passed its 40-minute cap")]
    Cap,
    #[error("the operator interrupted the drill")]
    Interrupted,
    #[error("the alert ledger could not be read: {0}")]
    Ledger(String),
    #[error("could not write {}", path.display())]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("{primary}; recording it in the alert ledger also failed: {ledger}")]
    Unrecorded {
        #[source]
        primary: Box<DrillError>,
        ledger: Box<DrillError>,
    },
}

impl DrillError {
    /// A request that timed out or could not connect, which the next poll
    /// may get through: a port-forward or a backend that was briefly slow.
    /// An HTTP status or an unreadable answer is not.
    fn is_transient(&self) -> bool {
        matches!(self, Self::Request { source, .. } if source.is_timeout() || source.is_connect())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Rehearse,
    AlertToRun,
    Isolation,
    ChatFailure,
    Burst,
    HardStop,
}

impl Action {
    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "rehearse" => Self::Rehearse,
            "alert-to-run" => Self::AlertToRun,
            "isolation" => Self::Isolation,
            "chat-failure" => Self::ChatFailure,
            "burst" => Self::Burst,
            "hard-stop" => Self::HardStop,
            _ => return None,
        })
    }

    fn slug(self) -> &'static str {
        match self {
            Self::Rehearse => "rehearse",
            Self::AlertToRun => "alert-to-run",
            Self::Isolation => "isolation",
            Self::ChatFailure => "chat-failure",
            Self::Burst => "burst",
            Self::HardStop => "hard-stop",
        }
    }

    /// Why the action needs `--confirm`, when it pages.
    fn needs_confirmation(self) -> Option<&'static str> {
        match self {
            Self::ChatFailure => Some("it pages #engineering-observability"),
            Self::HardStop => Some("it aborts a staging worker instance"),
            Self::Rehearse | Self::AlertToRun | Self::Isolation | Self::Burst => None,
        }
    }
}

/// The operations backends to read, matching the root's `telemetry_backend`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Backend {
    Datadog,
    Grafana,
    Both,
}

impl Backend {
    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "datadog" => Self::Datadog,
            "grafana" => Self::Grafana,
            "both" => Self::Both,
            _ => return None,
        })
    }

    fn datadog(self) -> bool {
        matches!(self, Self::Datadog | Self::Both)
    }

    fn grafana(self) -> bool {
        matches!(self, Self::Grafana | Self::Both)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Request {
    action: Action,
    output: PathBuf,
    scenario: Option<String>,
    backend: Backend,
    confirmed: bool,
}

fn parse_args(args: &[String]) -> Result<Request, String> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Err(USAGE.to_owned());
    }
    let mut action = None;
    let mut output = None;
    let mut scenario = None;
    let mut backend = Backend::Both;
    let mut confirmed = false;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        let mut value = |flag: &str| {
            rest.next()
                .cloned()
                .ok_or_else(|| format!("{flag} needs a value\n{USAGE}"))
        };
        match arg.as_str() {
            "--output-dir" => output = Some(PathBuf::from(value(arg)?)),
            "--scenario" => scenario = Some(value(arg)?),
            "--backend" => {
                let named = value(arg)?;
                backend = Backend::parse(&named)
                    .ok_or_else(|| format!("unknown backend {named}\n{USAGE}"))?;
            }
            "--confirm" => confirmed = true,
            other if action.is_none() => {
                action = Some(
                    Action::parse(other)
                        .ok_or_else(|| format!("unknown action {other}\n{USAGE}"))?,
                );
            }
            other => return Err(format!("unexpected argument {other}\n{USAGE}")),
        }
    }
    let action = action.ok_or_else(|| USAGE.to_owned())?;
    let output = output.ok_or_else(|| format!("--output-dir is required\n{USAGE}"))?;
    if let Some(scenario) = &scenario
        && !is_scenario(scenario)
    {
        return Err(format!(
            "--scenario must be [a-z0-9._-], at most 64 characters\n{USAGE}"
        ));
    }
    Ok(Request {
        action,
        output,
        scenario,
        backend,
        confirmed,
    })
}

/// The physician site's scenario rule (`SimulationPractices::admit_scenario`).
fn is_scenario(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"._-".contains(&b))
}

pub(super) fn from_args(args: &[String]) -> Result<(), Failure> {
    let request = parse_args(args).map_err(|message| Failure::new(2, message))?;
    if let Some(why) = request.action.needs_confirmation()
        && !request.confirmed
    {
        return Err(failure(DrillError::Unconfirmed {
            action: request.action.slug(),
            why,
        }));
    }
    let started = Utc::now();
    let mut report = Report::new(request.action, started);
    let outcome =
        Watch::install(started).and_then(|mut watch| run(&request, &mut report, &mut watch));
    report.outcome(&outcome);
    let path = report.write(&request.output).map_err(failure)?;
    match outcome {
        Ok(()) => {
            println!(
                "{} passed. Wrote {}.",
                request.action.slug(),
                path.display()
            );
            Ok(())
        }
        Err(error) => {
            let mut failed = failure(error);
            failed
                .message
                .push_str(&format!(" Wrote {}.", path.display()));
            Err(failed)
        }
    }
}

/// The operator sees the whole cause chain; the typed error stays the source.
fn failure(error: DrillError) -> Failure {
    let mut message = error.to_string();
    let mut cause = std::error::Error::source(&error);
    while let Some(inner) = cause {
        message.push_str(&format!(": {inner}"));
        cause = inner.source();
    }
    Failure::with_source(1, message, error)
}

/// Inspectable evidence for one run, written whatever the outcome.
struct Report {
    action: Action,
    started: DateTime<Utc>,
    body: serde_json::Map<String, Value>,
}

impl Report {
    fn new(action: Action, started: DateTime<Utc>) -> Self {
        Self {
            action,
            started,
            body: serde_json::Map::new(),
        }
    }

    fn put(&mut self, key: &str, value: impl serde::Serialize) {
        let value = serde_json::to_value(value).unwrap_or(Value::Null);
        self.body.insert(key.to_owned(), value);
    }

    fn outcome(&mut self, outcome: &Result<(), DrillError>) {
        match outcome {
            Ok(()) => self.put("outcome", "passed"),
            Err(error) => {
                self.put("outcome", "failed");
                self.put("failure", error.to_string());
            }
        }
    }

    fn write(&self, dir: &Path) -> Result<PathBuf, DrillError> {
        let stamp = self.started.format("%Y%m%dT%H%M%SZ");
        let path = dir.join(format!("langfuse-{}-{stamp}.json", self.action.slug()));
        let mut body = self.body.clone();
        body.insert("action".to_owned(), json!(self.action.slug()));
        body.insert("project".to_owned(), json!(STAGING_PROJECT));
        body.insert("started".to_owned(), json!(self.started.to_rfc3339()));
        body.insert("finished".to_owned(), json!(Utc::now().to_rfc3339()));
        write_json(&path, &Value::Object(body))?;
        Ok(path)
    }
}

fn write_json(path: &Path, body: &Value) -> Result<(), DrillError> {
    let written = path
        .parent()
        .map_or(Ok(()), std::fs::create_dir_all)
        .and_then(|()| {
            let mut text = serde_json::to_string_pretty(body).map_err(std::io::Error::other)?;
            text.push('\n');
            std::fs::write(path, text)
        });
    written.map_err(|source| DrillError::Write {
        path: path.to_owned(),
        source,
    })
}

fn run(request: &Request, report: &mut Report, watch: &mut Watch) -> Result<(), DrillError> {
    report.put("revisions", cluster::revisions()?);
    match request.action {
        Action::Rehearse => rehearse(request, report, watch),
        Action::AlertToRun => alert_to_run(request, report, watch),
        Action::Isolation => isolation(request, report, watch),
        Action::ChatFailure => chat_failure(request, report, watch),
        Action::Burst => burst(request, report, watch),
        Action::HardStop => hard_stop(request, report, watch),
    }
}

/// A short random hex string, from the clock and the process.
fn nonce() -> String {
    let seed = format!("{:?}-{}", std::time::SystemTime::now(), std::process::id());
    let digest = Sha256::digest(seed.as_bytes());
    digest[..8]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn client() -> Result<Client, DrillError> {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|source| DrillError::Request {
            what: "the HTTP client",
            source,
        })
}

fn secret(name: &'static str) -> Result<Secret, DrillError> {
    fetch_secret(name).map_err(|source| DrillError::Tool {
        what: "Doppler",
        source,
    })
}

/// One sent turn and what it is identified by.
struct Sent {
    turn: Turn,
    canary: String,
    /// The scoped attempt that answered, from the chat snapshot.
    attempt: String,
    reply_chars: usize,
    /// The model replies the answering snapshot holds.
    replies: usize,
    answered_at: DateTime<Utc>,
}

/// The rehearsal physician, signed in, and the patient its turns go to.
struct Rehearsal {
    physician: Physician,
    origin: String,
    workspace: String,
}

impl Rehearsal {
    fn sign_in() -> Result<Self, DrillError> {
        let client = client()?;
        let origin = cluster::physician_origin()?;
        let token = cluster::access_token()?;
        let now = u64::try_from(Utc::now().timestamp()).unwrap_or_default();
        let physician = Physician::sign_in(client, &origin, &token, now)?;
        let workspace = physician.first_workspace()?;
        Ok(Self {
            physician,
            origin,
            workspace,
        })
    }

    /// Submits one canary turn on a new chat, under a scenario unique to the
    /// turn unless one was named. `marker` is appended to the text as is.
    fn submit(&self, request: &Request, marker: &str) -> Result<(Turn, String), DrillError> {
        let run = nonce();
        let canary = format!("coda-canary-{run}");
        let turn = Turn {
            workspace: self.workspace.clone(),
            chat: format!("langfuse-drill-{run}"),
            text: format!("Langfuse drill {canary}. Please answer in one short sentence.{marker}"),
            scenario: request
                .scenario
                .clone()
                .unwrap_or_else(|| format!("langfuse-drill-{run}")),
            key: format!("langfuse-drill-{run}"),
        };
        self.physician.submit(&turn)?;
        Ok((turn, canary))
    }

    fn answer(&self, turn: Turn, canary: String, watch: &Watch) -> Result<Sent, DrillError> {
        self.answer_within(turn, canary, (REPLY_POLLS, REPLY_PAUSE), watch)
    }

    /// [`Self::answer`], reading the snapshot `polls` times `pause` apart.
    fn answer_within(
        &self,
        turn: Turn,
        canary: String,
        (polls, pause): (u32, Duration),
        watch: &Watch,
    ) -> Result<Sent, DrillError> {
        let (answer, snapshot) = self.physician.await_reply(&turn, polls, pause, watch)?;
        Ok(Sent {
            reply_chars: answer.text.chars().count(),
            replies: chat::model_replies(&snapshot),
            answered_at: Utc::now(),
            turn,
            canary,
            attempt: answer.attempt,
        })
    }

    fn evidence(&self, sent: &Sent) -> Value {
        json!({
            "origin": self.origin,
            "patient_workspace_id": sent.turn.workspace,
            "chat_id": sent.turn.chat,
            "scenario": sent.turn.scenario,
            "canary": sent.canary,
            "attempt_id": sent.attempt,
            "reply_chars": sent.reply_chars,
        })
    }
}

/// Signs in as the rehearsal physician and sends one canary turn on a new
/// chat, under a scenario unique to the run unless one was named.
fn send_turn(request: &Request, report: &mut Report, watch: &Watch) -> Result<Sent, DrillError> {
    let rehearsal = Rehearsal::sign_in()?;
    let (turn, canary) = rehearsal.submit(request, "")?;
    let sent = rehearsal.answer(turn, canary, watch)?;
    report.put("turn", rehearsal.evidence(&sent));
    Ok(sent)
}

/// The hosted Langfuse project the collector exports to, at
/// `LANGFUSE_BASE_URL`.
struct Langfuse {
    client: Client,
    base: Url,
}

impl Langfuse {
    fn open(report: &mut Report) -> Result<Self, DrillError> {
        let base = secret("LANGFUSE_BASE_URL")?;
        let base = langfuse_origin(base.expose()).map_err(DrillError::LangfuseHost)?;
        report.put("langfuse_host", base.origin().ascii_serialization());
        Ok(Self {
            client: client()?,
            base,
        })
    }

    /// The public API with the staging project keys.
    fn api(&self) -> Result<LangfuseApi<'_>, DrillError> {
        Ok(LangfuseApi {
            langfuse: self,
            public: secret("LANGFUSE_PUBLIC_KEY")?,
            secret: secret("LANGFUSE_SECRET_KEY")?,
        })
    }
}

/// Why `LANGFUSE_BASE_URL` is not a usable Langfuse origin.
#[derive(Debug, thiserror::Error)]
pub(super) enum LangfuseHostError {
    #[error("it does not parse as a URL")]
    Parse(#[source] url::ParseError),
    #[error("it carries more than an https scheme and a domain")]
    NotBare,
}

/// `value` as a bare https origin, the shape the observability root's
/// `langfuse_host` takes; a trailing slash is allowed.
fn langfuse_origin(value: &str) -> Result<Url, LangfuseHostError> {
    let url = Url::parse(value.trim()).map_err(LangfuseHostError::Parse)?;
    let bare = url.scheme() == "https"
        && url.domain().is_some()
        && url.port().is_none()
        && url.username().is_empty()
        && url.password().is_none()
        && url.path() == "/"
        && url.query().is_none()
        && url.fragment().is_none();
    if bare {
        Ok(url)
    } else {
        Err(LangfuseHostError::NotBare)
    }
}

fn json_body(
    response: reqwest::blocking::Response,
    what: &'static str,
) -> Result<Value, DrillError> {
    let status = response.status();
    if !status.is_success() {
        return Err(DrillError::Status {
            what,
            status: status.as_u16(),
        });
    }
    response
        .json()
        .map_err(|source| DrillError::Request { what, source })
}

struct LangfuseApi<'a> {
    langfuse: &'a Langfuse,
    public: Secret,
    secret: Secret,
}

impl LangfuseApi<'_> {
    fn get(
        &self,
        path: &'static str,
        query: &[(&str, &str)],
        what: &'static str,
    ) -> Result<Value, DrillError> {
        let mut url = self.langfuse.base.clone();
        url.set_path(path);
        chat::send_json(
            self.langfuse
                .client
                .get(url)
                .basic_auth(self.public.expose(), Some(self.secret.expose()))
                .query(query),
            what,
        )
    }

    /// The chat's generations, newest first, from the observations API: the
    /// real-time read path, where the trace endpoints lag by minutes.
    fn session_generations(&self, sent: &Sent) -> Result<Vec<LangfuseTurn>, DrillError> {
        let observations = self.get(
            "/api/public/v2/observations",
            &[
                ("sessionId", sent.turn.chat.as_str()),
                ("name", readback::GENERATION),
                ("fields", "core,basic,io,metadata,model"),
                ("expandMetadata", "attributes"),
                ("limit", "20"),
            ],
            "Langfuse observations",
        )?;
        readback::session_generations(&observations, &sent.canary).ok_or(DrillError::Shape {
            what: "Langfuse observations",
            field: "a data list whose generations carry traceId",
        })
    }

    /// Waits for the chat's newest generation.
    fn turn(&self, sent: &Sent, watch: &Watch) -> Result<LangfuseTurn, DrillError> {
        watch
            .poll(READBACK_POLLS, POLL, |_| {
                Ok(self.session_generations(sent)?.into_iter().next())
            })?
            .ok_or(readback_timeout("the Langfuse generation"))
    }

    /// Waits for the generation of attempt `run_id` among the chat's
    /// generations: each attempt has a trace of its own.
    fn attempt_generation(
        &self,
        sent: &Sent,
        run_id: &str,
        watch: &Watch,
    ) -> Result<LangfuseTurn, DrillError> {
        watch
            .poll(READBACK_POLLS, POLL, |_| {
                Ok(self
                    .session_generations(sent)?
                    .into_iter()
                    .find(|turn| turn.run_id.as_deref() == Some(run_id)))
            })?
            .ok_or(readback_timeout("the failed attempt's Langfuse generation"))
    }
}

/// Datadog's read APIs with the operator keys.
struct Datadog {
    client: Client,
    api: Secret,
    app: Secret,
}

/// One turn's trace as Datadog holds it.
struct DatadogTrace {
    trace_id: String,
    body: Value,
}

impl DatadogTrace {
    /// `coda.run_id` on the `physician.worker.attempt` span.
    fn attempt_run_id(&self) -> Option<String> {
        self.body["data"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|span| span["attributes"]["resource_name"] == "physician.worker.attempt")
            .and_then(|span| readback::attribute(&span["attributes"], "coda.run_id"))
            .map(str::to_owned)
    }
}

impl Datadog {
    fn open() -> Result<Self, DrillError> {
        Ok(Self {
            client: client()?,
            api: secret("DD_API_KEY")?,
            app: secret("DD_APP_KEY")?,
        })
    }

    fn authorized(
        &self,
        request: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        request
            .header("DD-API-KEY", self.api.expose())
            .header("DD-APPLICATION-KEY", self.app.expose())
    }

    fn spans(&self, query: &str) -> Result<Value, DrillError> {
        let body = json!({ "data": { "type": "search_request", "attributes": {
            "filter": { "query": query, "from": "now-2h", "to": "now" },
            "page": { "limit": 100 },
        }}});
        chat::send_json(
            self.authorized(
                self.client
                    .post(format!("{DATADOG_API}/api/v2/spans/events/search"))
                    .json(&body),
            ),
            "Datadog span search",
        )
    }

    /// Waits for the attempt span the snapshot named, then reads its whole
    /// trace: the turn is found by its attempt id, never by a nearby trace.
    fn attempt_trace(&self, attempt: &str, watch: &Watch) -> Result<DatadogTrace, DrillError> {
        let query = format!(
            "env:{DATADOG_ENV} service:physician-worker resource_name:physician.worker.attempt @coda.run_id:{attempt}"
        );
        watch
            .poll(READBACK_POLLS, POLL, |_| {
                let Some(trace_id) = first_trace_id(&self.spans(&query)?) else {
                    return Ok(None);
                };
                let body = self.spans(&format!("trace_id:{trace_id}"))?;
                Ok(readback::datadog_span(&body, "")
                    .found
                    .then_some(DatadogTrace { trace_id, body }))
            })?
            .ok_or(readback_timeout("the Datadog attempt and generation spans"))
    }

    /// Waits for the physician logs of the turn's trace, then judges every
    /// physician log event since `since` whole: message, attributes and
    /// error fields alike may not carry the canary.
    fn log_isolation(
        &self,
        trace_id: &str,
        canary: &str,
        since: DateTime<Utc>,
        watch: &Watch,
    ) -> Result<LogIsolation, DrillError> {
        let what = "Datadog log search";
        let body = json!({
            "filter": {
                "query": format!("env:{DATADOG_ENV} service:physician*"),
                "from": since.to_rfc3339(),
                "to": "now",
            },
            "page": { "limit": 1000 },
        });
        let read = || -> Result<LogIsolation, DrillError> {
            let found = chat::send_json(
                self.authorized(
                    self.client
                        .post(format!("{DATADOG_API}/api/v2/logs/events/search"))
                        .json(&body),
                ),
                what,
            )?;
            readback::datadog_logs(&found, trace_id, canary).ok_or(DrillError::Shape {
                what,
                field: "one complete page of log events",
            })
        };
        let traced = watch.poll(READBACK_POLLS, POLL, |_| {
            read().map(|logs| (logs.trace_lines > 0).then_some(logs))
        })?;
        match traced {
            Some(logs) => Ok(logs),
            None => read(),
        }
    }

    /// The physician chat failure monitor (`chat_generation_errors` in
    /// `infra/gcp/observability/datadog.tf`), which must exist.
    fn chat_failure_monitor(&self) -> Result<NamedMonitor, DrillError> {
        let what = "Datadog chat failure monitor";
        let monitors = chat::send_json(
            self.authorized(
                self.client
                    .get(format!("{DATADOG_API}/api/v1/monitor"))
                    .query(&[("name", CHAT_FAILURE_MONITOR), ("group_states", "all")]),
            ),
            what,
        )?;
        readback::named_monitor(&monitors, CHAT_FAILURE_MONITOR).ok_or(DrillError::Shape {
            what,
            field: "a monitor named [staging] physician chat generation errors",
        })
    }

    /// Waits for attempt `run_id`'s `physician.worker.attempt` span, then
    /// reads its whole trace.
    fn attempt(
        &self,
        run_id: &str,
        watch: &Watch,
    ) -> Result<(AttemptSpan, DatadogTrace), DrillError> {
        let query = format!(
            "env:{DATADOG_ENV} service:physician-worker resource_name:physician.worker.attempt @coda.run_id:{run_id}"
        );
        watch
            .poll(READBACK_POLLS, POLL, |_| {
                let found = self.spans(&query)?;
                let (Some(span), Some(trace_id)) = (
                    readback::attempt_span(&found, run_id),
                    first_trace_id(&found),
                ) else {
                    return Ok(None);
                };
                let body = self.spans(&format!("trace_id:{trace_id}"))?;
                Ok(Some((span, DatadogTrace { trace_id, body })))
            })?
            .ok_or(readback_timeout("the Datadog attempt span"))
    }

    /// The trace id of attempt `run_id`'s failed generation, found by the
    /// chat failure monitor's own query narrowed to that attempt.
    fn failed_generation(&self, run_id: &str, watch: &Watch) -> Result<String, DrillError> {
        let query = format!(
            "service:physician-worker env:{DATADOG_ENV} status:error @coda.job:physician_* @gen_ai.system:* @coda.run_id:{run_id}"
        );
        watch
            .poll(READBACK_POLLS, POLL, |_| {
                Ok(first_trace_id(&self.spans(&query)?))
            })?
            .ok_or(readback_timeout(
                "the Datadog error generation the monitor counts",
            ))
    }
}

/// The trace id of the first span in a Datadog span search answer.
fn first_trace_id(found: &Value) -> Option<String> {
    found["data"]
        .as_array()
        .into_iter()
        .flatten()
        .find_map(|span| span["attributes"]["trace_id"].as_str().map(str::to_owned))
}

/// What a readback that ran out of polls without finding `what` answers.
fn readback_timeout(what: &'static str) -> DrillError {
    DrillError::Timeout {
        what,
        seconds: u64::from(READBACK_POLLS) * POLL.as_secs(),
    }
}

fn gaps(what: &'static str, gaps: Vec<String>) -> Result<(), DrillError> {
    if gaps.is_empty() {
        Ok(())
    } else {
        Err(DrillError::Gaps { what, gaps })
    }
}

/// Where the snapshot's attempt id and Langfuse's `coda.run_id` part.
fn attempt_gaps(sent: &Sent, turn: &LangfuseTurn) -> Vec<String> {
    if turn.run_id.as_deref() == Some(sent.attempt.as_str()) {
        return Vec::new();
    }
    vec![format!(
        "Langfuse coda.run_id {:?} is not the snapshot's attempt {}",
        turn.run_id, sent.attempt
    )]
}

fn rehearse(request: &Request, report: &mut Report, watch: &Watch) -> Result<(), DrillError> {
    let langfuse = Langfuse::open(report)?;
    let api = langfuse.api()?;
    let sent = send_turn(request, report, watch)?;
    let turn = api.turn(&sent, watch)?;
    report.put("langfuse", &turn);
    let mut found = readback::rehearsal_gaps(&turn, &sent.turn.chat, &sent.turn.scenario);
    found.extend(attempt_gaps(&sent, &turn));
    gaps("the Langfuse generation", found)
}

fn alert_to_run(request: &Request, report: &mut Report, watch: &Watch) -> Result<(), DrillError> {
    let langfuse = Langfuse::open(report)?;
    let api = langfuse.api()?;
    let datadog = Datadog::open()?;
    let sent = send_turn(request, report, watch)?;
    let trace = datadog.attempt_trace(&sent.attempt, watch)?;
    let turn = api.turn(&sent, watch)?;
    let generation = readback::datadog_span(&trace.body, &sent.canary);
    let attempt_run = trace.attempt_run_id();
    report.put(
        "datadog",
        json!({ "trace_id": trace.trace_id, "attempt_run_id": attempt_run, "generation": generation }),
    );
    report.put("langfuse", &turn);
    let mut found = attempt_gaps(&sent, &turn);
    if !readback::same_trace(&trace.trace_id, &turn.trace_id) {
        found.push(format!(
            "Datadog trace {} is not Langfuse trace {}",
            trace.trace_id, turn.trace_id
        ));
    }
    // A missing attempt span is the `coda.run_id` gap below.
    if let Some(span) = readback::attempt_span(&trace.body, &sent.attempt) {
        found.extend(readback::link_gaps(&span));
    }
    let attempt = Some(sent.attempt.as_str());
    if generation.run_id.as_deref() != attempt {
        found.push(format!(
            "Datadog generation coda.run_id {:?} is not the attempt {}",
            generation.run_id, sent.attempt
        ));
    }
    if attempt_run.as_deref() != attempt {
        found.push(format!(
            "Datadog attempt coda.run_id {attempt_run:?} is not the attempt {}",
            sent.attempt
        ));
    }
    gaps("alert to run", found)
}

fn isolation(request: &Request, report: &mut Report, watch: &Watch) -> Result<(), DrillError> {
    let langfuse = Langfuse::open(report)?;
    let api = langfuse.api()?;
    let datadog = request.backend.datadog().then(Datadog::open).transpose()?;
    let tempo = request
        .backend
        .grafana()
        .then(|| Forward::open("tempo", 3200))
        .transpose()?;
    let loki = request
        .backend
        .grafana()
        .then(|| Forward::open("loki", 3100))
        .transpose()?;
    let started = Utc::now();
    let sent = send_turn(request, report, watch)?;
    let turn = api.turn(&sent, watch)?;
    report.put("langfuse", &turn);
    let mut found = attempt_gaps(&sent, &turn);
    if !turn.canary_in_content {
        found.push("Langfuse does not hold the canary; the positive control failed".to_owned());
    }
    let attempt = Some(sent.attempt.as_str());
    let mut spans: Vec<OpsSpan> = Vec::new();
    let mut logs: Vec<LogIsolation> = Vec::new();
    if let Some(datadog) = &datadog {
        let trace = datadog.attempt_trace(&sent.attempt, watch)?;
        if !readback::same_trace(&trace.trace_id, &turn.trace_id) {
            found.push(format!(
                "Datadog trace {} is not Langfuse trace {}",
                trace.trace_id, turn.trace_id
            ));
        }
        spans.push(readback::datadog_span(&trace.body, &sent.canary));
        logs.push(datadog.log_isolation(&turn.trace_id, &sent.canary, started, watch)?);
    }
    if let Some(tempo) = &tempo {
        spans.push(tempo_span(tempo, &turn.trace_id, &sent.canary, watch)?);
    }
    for span in &spans {
        found.extend(readback::isolation_gaps(span, attempt));
    }
    report.put("operations", &spans);
    if let Some(loki) = &loki {
        logs.push(loki_isolation(loki, &sent, &turn.trace_id, started, watch)?);
    }
    for log in &logs {
        found.extend(log.gaps());
    }
    report.put("logs", &logs);
    gaps("content isolation", found)
}

fn tempo_span(
    tempo: &Forward,
    trace_id: &str,
    canary: &str,
    watch: &Watch,
) -> Result<OpsSpan, DrillError> {
    let client = client()?;
    let found = watch.poll(READBACK_POLLS, POLL, |_| {
        let response = client
            .get(format!("{}/api/traces/{trace_id}", tempo.url()))
            .send()
            .map_err(|source| DrillError::Request {
                what: "Tempo trace",
                source,
            })?;
        if response.status().as_u16() == 404 {
            return Ok(None);
        }
        let span = readback::tempo_span(&json_body(response, "Tempo trace")?, canary);
        Ok(span.found.then_some(span))
    })?;
    Ok(found.unwrap_or_else(|| readback::tempo_span(&Value::Null, canary)))
}

/// Waits for the physician logs of the turn's trace, then counts canary lines in
/// every physician stream over the same window.
fn loki_isolation(
    loki: &Forward,
    sent: &Sent,
    trace_id: &str,
    since: DateTime<Utc>,
    watch: &Watch,
) -> Result<LogIsolation, DrillError> {
    let client = client()?;
    let start = since.timestamp_nanos_opt().unwrap_or_default().to_string();
    let count = |query: &str| -> Result<usize, DrillError> {
        let body = chat::send_json(
            client
                .get(format!("{}/loki/api/v1/query_range", loki.url()))
                .query(&[
                    ("query", query),
                    ("start", start.as_str()),
                    ("limit", "1000"),
                ]),
            "Loki query",
        )?;
        readback::loki_lines(&body).ok_or(DrillError::Shape {
            what: "Loki query",
            field: "a streams result",
        })
    };
    // The turn's trace carries at least the site's `http.server` response log
    // for its POST /commands: the first delivery's attempt continues that
    // trace. Loki keeps `trace_id` as structured metadata.
    let traced = format!("{{service_name=~\"physician.*\"}} | trace_id=\"{trace_id}\"");
    let trace_lines = watch
        .poll(READBACK_POLLS, POLL, |_| {
            count(&traced).map(|lines| (lines > 0).then_some(lines))
        })?
        .unwrap_or(0);
    let canary_lines = count(&format!(
        "{{service_name=~\"physician.*\"}} |= \"{}\"",
        sent.canary
    ))?;
    Ok(LogIsolation {
        backend: "loki",
        trace_lines,
        canary_lines,
    })
}

/// Whether a monitor group fired after `since`, or every group has resolved
/// after it.
fn monitor_reached(
    groups: &[MonitorGroup],
    alerting: bool,
    since: DateTime<Utc>,
) -> Option<(MonitorGroup, DateTime<Utc>)> {
    let at = |stamp: Option<i64>| {
        stamp
            .and_then(|seconds| DateTime::<Utc>::from_timestamp(seconds, 0))
            .filter(|at| *at >= since)
    };
    if alerting {
        return groups.iter().find_map(|group| {
            let fired = at(group.last_triggered_ts).filter(|_| group.status == "Alert")?;
            Some((group.clone(), fired))
        });
    }
    // Recovered only once no group is alerting and one resolved after the
    // restore; the latest resolution is the evaluator's recovery time.
    if groups.is_empty() || groups.iter().any(|group| group.status != "OK") {
        return None;
    }
    groups
        .iter()
        .filter_map(|group| Some((group.clone(), at(group.last_resolved_ts)?)))
        .max_by_key(|(_, at)| *at)
}

fn ledger(dir: &Path) -> Result<Exercise, DrillError> {
    super::load_open(dir, DrillRoute::Langfuse).map_err(DrillError::Ledger)
}

fn save_ledger(dir: &Path, exercise: &Exercise, now: DateTime<Utc>) -> Result<(), DrillError> {
    let name = super::observation_file_name(DrillRoute::Langfuse, now);
    super::write_observations(dir, &name, exercise)
        .map(|_| ())
        .map_err(|error| DrillError::Ledger(error.to_string()))
}

/// Keeps both errors when the ledger write fails after `outcome`.
fn with_ledger(
    outcome: Result<(), DrillError>,
    saved: Result<(), DrillError>,
) -> Result<(), DrillError> {
    match (outcome, saved) {
        (outcome, Ok(())) => outcome,
        (Ok(()), Err(saved)) => Err(saved),
        (Err(primary), Err(saved)) => Err(DrillError::Unrecorded {
            primary: Box::new(primary),
            ledger: Box::new(saved),
        }),
    }
}

/// How many marked turns cross a monitor alerting above `threshold`.
fn failing_turns(threshold: Option<f64>) -> Result<u32, DrillError> {
    let what = "Datadog chat failure monitor";
    let threshold = threshold.filter(|value| value.is_finite() && *value >= 0.0);
    let Some(threshold) = threshold else {
        return Err(DrillError::Shape {
            what,
            field: "options.thresholds.critical",
        });
    };
    // The monitor alerts on `> threshold`: the fewest failures above it.
    (1..=MAX_FAILING_TURNS)
        .find(|turns| f64::from(*turns) > threshold)
        .ok_or_else(|| DrillError::Gaps {
            what,
            gaps: vec![format!(
                "its threshold {threshold} needs more than {MAX_FAILING_TURNS} failing turns"
            )],
        })
}

/// Waits for the chat failure monitor to fire (`alerting`) or recover after
/// `since`.
fn await_chat_monitor(
    datadog: &Datadog,
    alerting: bool,
    since: DateTime<Utc>,
    polls: u32,
    watch: &Watch,
) -> Result<(MonitorGroup, DateTime<Utc>), DrillError> {
    let reached = watch.poll(polls, POLL, |_| {
        let monitor = datadog.chat_failure_monitor()?;
        Ok(monitor_reached(&monitor.groups, alerting, since))
    })?;
    reached.ok_or(DrillError::Timeout {
        what: if alerting {
            "the chat failure monitor alert"
        } else {
            "the chat failure monitor recovery"
        },
        seconds: u64::from(polls) * POLL.as_secs(),
    })
}

/// Sends threshold + 1 rehearsal turns whose first attempt fails on purpose,
/// so the physician chat failure monitor pages, then follows one failure from
/// the monitor's own query to the failed attempt, its answering successor and
/// its Langfuse generation, and waits for the monitor to recover.
fn chat_failure(
    request: &Request,
    report: &mut Report,
    watch: &mut Watch,
) -> Result<(), DrillError> {
    let datadog = Datadog::open()?;
    let langfuse = Langfuse::open(report)?;
    let api = langfuse.api()?;
    if ledger(&request.output)?.is_open() {
        return Err(DrillError::Ledger(
            "an exercise on the Langfuse route is still open in the output directory".to_owned(),
        ));
    }
    let monitor = datadog.chat_failure_monitor()?;
    report.put("monitor_before", &monitor);
    if monitor.groups.iter().any(|group| group.status != "OK") {
        return Err(DrillError::Gaps {
            what: "the chat failure monitor",
            gaps: vec!["it is not OK before the drill, so a firing would prove nothing".to_owned()],
        });
    }
    let count = failing_turns(monitor.threshold)?;
    let rehearsal = Rehearsal::sign_in()?;
    let mut exercise = Exercise::idle_in(DrillRoute::Langfuse, CHAT_FAILURE_GROUP);
    let failed_at = Utc::now();
    watch.opened_at(failed_at);
    let file = super::observation_file_name(DrillRoute::Langfuse, failed_at);
    exercise.record_signal(SignalKind::Failure, failed_at, &file);
    let outcome = chat_failure_checks(
        request,
        report,
        &ChatFailureReaders {
            datadog: &datadog,
            api: &api,
            rehearsal: &rehearsal,
        },
        count,
        failed_at,
        &mut exercise,
        watch,
    );
    // Nothing stays failing once the drill stops sending marked turns, so an
    // aborted run closes its exercise rather than block the next drill.
    if exercise.is_open() {
        exercise.record_signal(SignalKind::Restored, Utc::now(), &file);
    }
    with_ledger(outcome, save_ledger(&request.output, &exercise, failed_at))
}

/// What `chat-failure` reads and writes through.
struct ChatFailureReaders<'a> {
    datadog: &'a Datadog,
    api: &'a LangfuseApi<'a>,
    rehearsal: &'a Rehearsal,
}

fn chat_failure_checks(
    request: &Request,
    report: &mut Report,
    readers: &ChatFailureReaders<'_>,
    count: u32,
    failed_at: DateTime<Utc>,
    exercise: &mut Exercise,
    watch: &Watch,
) -> Result<(), DrillError> {
    let ChatFailureReaders {
        datadog,
        api,
        rehearsal,
    } = readers;
    // Every turn is submitted before any is awaited, so the failures land
    // well inside the monitor's 10-minute window.
    let mut submitted = Vec::new();
    for _turn in 0..count {
        submitted.push(rehearsal.submit(request, FAIL_ONCE)?);
    }
    let mut answered = Vec::new();
    for (turn, canary) in submitted {
        answered.push(rehearsal.answer(turn, canary, watch)?);
    }
    report.put(
        "turns",
        answered
            .iter()
            .map(|sent| rehearsal.evidence(sent))
            .collect::<Vec<_>>(),
    );
    let (group, fired) = await_chat_monitor(datadog, true, failed_at, MONITOR_POLLS, watch)?;
    report.put("monitor_firing", &group);
    exercise.record_evaluator(
        EvalKind::Firing,
        fired,
        &format!("Datadog monitor {CHAT_FAILURE_MONITOR:?}"),
    );
    // Every marked turn answered before the firing, so the failures have
    // stopped by now and the monitor's 10-minute window drains from here,
    // whatever the readback below takes.
    let restored = Utc::now();
    exercise.record_signal(
        SignalKind::Restored,
        restored,
        &super::observation_file_name(DrillRoute::Langfuse, failed_at),
    );
    let followed = answered.first().ok_or(DrillError::Shape {
        what: "chat-failure",
        field: "an answered turn",
    })?;
    let mut found = follow_failure(report, datadog, api, followed, watch)?;
    for sent in &answered[1..] {
        let (successor, _) = datadog.attempt(&sent.attempt, watch)?;
        if successor.predecessor_run_id.is_none() {
            found.push(format!(
                "turn {} answered on its first attempt; the marker did not fail it",
                sent.turn.chat
            ));
        }
    }
    gaps("chat failure to run", found)?;

    let (group, recovered) = await_chat_monitor(datadog, false, fired, CHAT_RECOVERY_POLLS, watch)?;
    report.put("monitor_recovered", &group);
    if recovered < restored {
        return Err(DrillError::Gaps {
            what: "the chat failure monitor",
            gaps: vec![format!(
                "it recovered at {recovered}, before the failures were seen to stop at {restored}"
            )],
        });
    }
    exercise.record_evaluator(
        EvalKind::Recovered,
        recovered,
        &format!("Datadog monitor {CHAT_FAILURE_MONITOR:?}"),
    );
    Ok(())
}

/// Follows one marked turn from the monitor's query to the failed attempt,
/// its successor and the failed generation in Langfuse. Returns the gaps.
fn follow_failure(
    report: &mut Report,
    datadog: &Datadog,
    api: &LangfuseApi<'_>,
    sent: &Sent,
    watch: &Watch,
) -> Result<Vec<String>, DrillError> {
    let (successor, successor_trace) = datadog.attempt(&sent.attempt, watch)?;
    let Some(failed_run) = successor.predecessor_run_id.clone() else {
        return Ok(vec![format!(
            "the answering attempt {} names no predecessor; the turn never failed",
            sent.attempt
        )]);
    };
    let (failed, failed_trace) = datadog.attempt(&failed_run, watch)?;
    let alerted_trace = datadog.failed_generation(&failed_run, watch)?;
    let langfuse = api.attempt_generation(sent, &failed_run, watch)?;
    let failed_generation = readback::datadog_span(&failed_trace.body, &sent.canary);
    let successor_generation = readback::datadog_span(&successor_trace.body, &sent.canary);
    report.put(
        "followed",
        json!({
            "chat_id": sent.turn.chat,
            "monitor_query_trace_id": alerted_trace,
            "failed_attempt": failed,
            "failed_trace_id": failed_trace.trace_id,
            "failed_generation": failed_generation,
            "successor_attempt": successor,
            "successor_trace_id": successor_trace.trace_id,
            "successor_generation": successor_generation,
            "langfuse": langfuse,
        }),
    );
    let mut found = readback::retry_gaps(&failed, &successor);
    found.extend(readback::failed_generation_gaps(&langfuse, &failed_run));
    if alerted_trace != failed_trace.trace_id {
        found.push(format!(
            "the monitor's error generation is in trace {alerted_trace}, not the failed attempt's {}",
            failed_trace.trace_id
        ));
    }
    if !readback::same_trace(&failed_trace.trace_id, &langfuse.trace_id) {
        found.push(format!(
            "Datadog trace {} of the failed attempt is not Langfuse trace {}",
            failed_trace.trace_id, langfuse.trace_id
        ));
    }
    if !failed_generation.found {
        found.push(
            "Datadog has no generation span in the failed attempt's trace; an empty search is not proof"
                .to_owned(),
        );
    }
    if failed_generation.canary {
        found.push("Datadog carries the canary in the failed attempt's trace".to_owned());
    }
    found.extend(readback::link_gaps(&failed));
    found.extend(readback::link_gaps(&successor));
    found.extend(readback::isolation_gaps(
        &successor_generation,
        Some(&sent.attempt),
    ));
    Ok(found)
}

/// The operator's Google Cloud reads `hard-stop` and `burst` need: the queue
/// gauges and the worker's crash receipts.
struct Google {
    client: Client,
    token: String,
}

impl Google {
    fn open() -> Result<Self, DrillError> {
        Ok(Self {
            client: client()?,
            token: cluster::access_token()?,
        })
    }

    /// The newest `due` gauge point of every worker instance from the last
    /// five minutes.
    fn queue_due(&self) -> Result<Option<contention::QueueDue>, DrillError> {
        let now = Utc::now();
        let start = (now - chrono::Duration::minutes(5)).to_rfc3339();
        let end = now.to_rfc3339();
        let filter = format!("metric.type=\"{DUE_METRIC}\"");
        let body = chat::send_json(
            self.client
                .get(format!("{CLOUD_MONITORING}/{STAGING_PROJECT}/timeSeries"))
                .bearer_auth(&self.token)
                .query(&[
                    ("filter", filter.as_str()),
                    ("interval.startTime", start.as_str()),
                    ("interval.endTime", end.as_str()),
                ]),
            "the physician queue gauges",
        )?;
        Ok(contention::queue_due(&body))
    }

    /// Waits for attempt `run_id`'s rehearsed-crash log line in Cloud
    /// Logging, which holds the worker's stdout whatever the telemetry
    /// backend; `None` once the readback polls run out.
    fn crash_receipt(
        &self,
        run_id: &str,
        since: DateTime<Utc>,
        watch: &Watch,
    ) -> Result<Option<Value>, DrillError> {
        let filter = format!(
            "resource.type=\"cloud_run_revision\" resource.labels.service_name=\"physician-worker\" \
             jsonPayload.event=\"{}\" timestamp>=\"{}\"",
            readback::CRASH_EVENT,
            since.to_rfc3339()
        );
        let body = json!({
            "resourceNames": [format!("projects/{STAGING_PROJECT}")],
            "filter": filter,
            "orderBy": "timestamp desc",
            "pageSize": 50,
        });
        watch.poll(READBACK_POLLS, POLL, |_| {
            let found = chat::send_json(
                self.client
                    .post(CLOUD_LOGGING)
                    .bearer_auth(&self.token)
                    .json(&body),
                "Cloud Logging",
            )?;
            Ok(readback::crash_receipt(&found, run_id))
        })
    }
}

/// `burst`: an idle baseline turn, then two waves of held turns, each judged
/// against its drain bound.
fn burst(request: &Request, report: &mut Report, watch: &Watch) -> Result<(), DrillError> {
    let datadog = Datadog::open()?;
    let google = Google::open()?;
    let rehearsal = Rehearsal::sign_in()?;
    let queue = google.queue_due()?;
    report.put("queue_before", queue);
    let instances = queue.map_or(1, |queue| queue.instances);
    let bound = contention::drain_bound(BURST_TURNS, instances, BURST_HOLD);
    report.put("drain_bound_seconds", bound.as_secs_f64());

    let baseline = burst_wave(&datadog, &rehearsal, request, 1, "", watch)?;
    let baseline_summary = contention::WaveSummary::of(&baseline, bound, false);
    report.put(
        "baseline",
        json!({ "turn": baseline, "summary": baseline_summary }),
    );
    let mut found = Vec::new();
    for wave in ["wave 1", "wave 2"] {
        let samples = burst_wave(
            &datadog,
            &rehearsal,
            request,
            BURST_TURNS,
            BURST_MARKER,
            watch,
        )?;
        let summary = contention::WaveSummary::of(&samples, bound, true);
        found.extend(summary.gaps(wave));
        report.put(
            &wave.replace(' ', "_"),
            json!({ "turns": samples, "summary": summary }),
        );
    }
    gaps("the burst", found)
}

/// Submits `count` turns marked `marker` at once, awaits each reply, then
/// reads each answering attempt's span. A reply or span the drill could not
/// read stays missing in its sample.
fn burst_wave(
    datadog: &Datadog,
    rehearsal: &Rehearsal,
    request: &Request,
    count: u32,
    marker: &str,
    watch: &Watch,
) -> Result<Vec<contention::TurnSample>, DrillError> {
    let mut submitted = Vec::new();
    for _turn in 0..count {
        let at = Utc::now();
        let (turn, canary) = rehearsal.submit(request, marker)?;
        submitted.push((
            contention::TurnSample::submitted(turn.chat.clone(), at),
            turn,
            canary,
        ));
    }
    let mut samples = Vec::new();
    for (mut sample, turn, canary) in submitted {
        match rehearsal.answer_within(turn, canary, BURST_REPLY, watch) {
            Ok(sent) => {
                sample.completed = Some(sent.answered_at);
                sample.attempt_id = Some(sent.attempt);
            }
            Err(DrillError::Timeout { .. } | DrillError::TurnFailed { .. }) => {}
            Err(error) => return Err(error),
        }
        samples.push(sample);
    }
    for sample in &mut samples {
        let Some(attempt) = sample.attempt_id.clone() else {
            continue;
        };
        match datadog.attempt(&attempt, watch) {
            Ok((span, _trace)) => sample.spanned(&span),
            Err(DrillError::Timeout { .. }) => {}
            Err(error) => return Err(error),
        }
    }
    Ok(samples)
}

/// `hard-stop`: one turn whose first attempt aborts its worker, recovered
/// by another worker once the lease expires, then a normal turn.
fn hard_stop(request: &Request, report: &mut Report, watch: &Watch) -> Result<(), DrillError> {
    let datadog = Datadog::open()?;
    let google = Google::open()?;
    let queue = google.queue_due()?;
    report.put("queue_before", queue);
    match queue {
        Some(queue) if queue.due == 0 => {}
        Some(queue) => {
            return Err(DrillError::Gaps {
                what: "the physician queue",
                gaps: vec![format!(
                    "{} turns are due; a crash would also drop theirs, so the drill needs an empty queue",
                    queue.due
                )],
            });
        }
        None => {
            return Err(DrillError::Gaps {
                what: "the physician queue",
                gaps: vec!["no worker wrote a due gauge in the last five minutes".to_owned()],
            });
        }
    }
    let rehearsal = Rehearsal::sign_in()?;
    let crashed_from = Utc::now();
    let (turn, canary) = rehearsal.submit(request, HARD_STOP_MARKER)?;
    let sent = rehearsal.answer_within(turn, canary, HARD_STOP_REPLY, watch)?;
    let (successor, _trace) = datadog.attempt(&sent.attempt, watch)?;
    let receipt = match successor.predecessor_run_id.as_deref() {
        Some(crashed) => google.crash_receipt(crashed, crashed_from, watch)?,
        None => None,
    };
    // The answering snapshot may predate a second reply; read the chat again
    // once the readbacks are in.
    let replies = chat::model_replies(
        &rehearsal
            .physician
            .snapshot(&sent.turn.workspace, &sent.turn.chat)?,
    );
    report.put(
        "hard_stop",
        json!({
            "turn": rehearsal.evidence(&sent),
            "submitted": crashed_from.to_rfc3339(),
            "answered": sent.answered_at.to_rfc3339(),
            "replies_when_answered": sent.replies,
            "replies_after_readback": replies,
            "successor_attempt": successor,
            "crash_receipt": receipt,
        }),
    );
    gaps(
        "the hard stop",
        contention::hard_stop_gaps(&successor, receipt.as_ref(), replies),
    )?;
    let (turn, canary) = rehearsal.submit(request, "")?;
    let after = rehearsal.answer(turn, canary, watch)?;
    report.put("rehearse_after", rehearsal.evidence(&after));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn actions_and_flags_parse() {
        let request = parse_args(&args(&[
            "chat-failure",
            "--output-dir",
            "out",
            "--confirm",
            "--backend",
            "datadog",
            "--scenario",
            "s-1",
        ]))
        .unwrap();
        assert_eq!(request.action, Action::ChatFailure);
        assert!(request.confirmed);
        assert_eq!(request.backend, Backend::Datadog);
        assert_eq!(request.scenario.as_deref(), Some("s-1"));
    }

    #[test]
    fn bad_input_is_refused_with_usage() {
        assert!(
            parse_args(&args(&["rehearse"]))
                .unwrap_err()
                .contains("--output-dir")
        );
        assert!(parse_args(&args(&["nope", "--output-dir", "o"])).is_err());
        assert!(
            parse_args(&args(&[
                "rehearse",
                "--output-dir",
                "o",
                "--scenario",
                "Bad Id"
            ]))
            .is_err()
        );
        assert!(parse_args(&args(&["rehearse", "--output-dir", "o", "--backend", "x"])).is_err());
        assert!(parse_args(&args(&["rehearse", "--output-dir", "o", "extra"])).is_err());
        for retired in ["fail", "recover", "signin-breakglass"] {
            assert!(parse_args(&args(&[retired, "--output-dir", "o"])).is_err());
        }
    }

    #[test]
    fn only_the_paging_and_crashing_actions_need_confirmation() {
        for action in ["chat-failure", "hard-stop"] {
            let refused =
                from_args(&args(&[action, "--output-dir", "/nonexistent/x"])).unwrap_err();
            assert!(refused.message.contains("--confirm"), "{}", refused.message);
        }
        for action in [
            Action::Rehearse,
            Action::AlertToRun,
            Action::Isolation,
            Action::Burst,
        ] {
            assert!(action.needs_confirmation().is_none());
        }
    }

    #[test]
    fn the_contention_actions_parse() {
        for (named, action) in [("burst", Action::Burst), ("hard-stop", Action::HardStop)] {
            let request = parse_args(&args(&[named, "--output-dir", "o"])).unwrap();
            assert_eq!(request.action, action);
            assert_eq!(action.slug(), named);
        }
    }

    #[test]
    fn the_langfuse_host_is_a_bare_https_origin() {
        let origin = |value| {
            langfuse_origin(value)
                .ok()
                .map(|url| url.origin().ascii_serialization())
        };
        assert_eq!(
            origin("https://hipaa.cloud.langfuse.com").as_deref(),
            Some("https://hipaa.cloud.langfuse.com")
        );
        assert_eq!(
            origin("https://hipaa.cloud.langfuse.com/\n").as_deref(),
            Some("https://hipaa.cloud.langfuse.com")
        );
        assert!(matches!(
            langfuse_origin("https://"),
            Err(LangfuseHostError::Parse(_))
        ));
        for refused in [
            "http://hipaa.cloud.langfuse.com",
            "https://hipaa.cloud.langfuse.com/langfuse",
            "https://127.0.0.1:3000",
            "https://hipaa.cloud.langfuse.com:8443",
            "https://user@hipaa.cloud.langfuse.com",
            "https://hipaa.cloud.langfuse.com/?x=1",
        ] {
            assert!(
                matches!(langfuse_origin(refused), Err(LangfuseHostError::NotBare)),
                "{refused}"
            );
        }
    }

    #[test]
    fn chat_failure_sends_one_more_turn_than_the_threshold() {
        assert_eq!(failing_turns(Some(5.0)).unwrap(), 6);
        assert_eq!(failing_turns(Some(0.0)).unwrap(), 1);
        assert_eq!(failing_turns(Some(2.5)).unwrap(), 3);
        assert!(failing_turns(None).is_err());
        assert!(failing_turns(Some(-1.0)).is_err());
        assert!(failing_turns(Some(f64::from(MAX_FAILING_TURNS))).is_err());
    }

    fn group(status: &str, triggered: Option<i64>, resolved: Option<i64>) -> MonitorGroup {
        MonitorGroup {
            monitor: "m".into(),
            status: status.into(),
            last_triggered_ts: triggered,
            last_resolved_ts: resolved,
        }
    }

    #[test]
    fn firing_needs_an_alert_after_the_failure() {
        let since = DateTime::<Utc>::from_timestamp(100, 0).unwrap();
        assert!(monitor_reached(&[group("Alert", Some(50), None)], true, since).is_none());
        let (_, at) = monitor_reached(&[group("Alert", Some(150), None)], true, since).unwrap();
        assert_eq!(at.timestamp(), 150);
    }

    #[test]
    fn recovery_needs_every_group_ok_and_a_resolution_after_the_restore() {
        let since = DateTime::<Utc>::from_timestamp(100, 0).unwrap();
        assert!(monitor_reached(&[], false, since).is_none());
        let mixed = [
            group("OK", None, Some(150)),
            group("Alert", Some(120), None),
        ];
        assert!(monitor_reached(&mixed, false, since).is_none());
        assert!(monitor_reached(&[group("OK", None, Some(90))], false, since).is_none());
        let both = [group("OK", None, Some(150)), group("OK", None, Some(170))];
        let (_, at) = monitor_reached(&both, false, since).unwrap();
        assert_eq!(at.timestamp(), 170);
    }

    #[test]
    fn nonces_differ() {
        assert_ne!(nonce(), nonce());
        assert_eq!(nonce().len(), 16);
    }
}
