//! `observability-drill langfuse`: drives a rehearsal chat turn on staging as
//! the physician rehearsal identity and reads it back from Langfuse and the
//! operations backends.
//!
//! Every check writes `langfuse-<action>-<stamp>.json` with the deployed
//! revisions it ran against. `fail` and `recover` also write the alert ledger
//! `observations-langfuse-<stamp>.json` for `qualify-alerts.py`, under the
//! same 40-minute cap and restore-on-abort as the other routes.
mod chat;
mod cluster;
mod readback;

use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use super::live::{STAGING_PROJECT, Secret, fetch_secret};
use super::model::{DrillRoute, EvalKind, Exercise, SignalError, SignalKind};
use crate::failure::Failure;
use chat::{Physician, Turn};
use cluster::{Forward, Kubectl};
use readback::{LangfuseTurn, MonitorGroup, OpsSpan};

const USAGE: &str = "\
observability-drill langfuse ACTION --output-dir DIR [--scenario ID] [--backend datadog|grafana|both] [--confirm]

ACTION
  rehearse           send a canary turn; Langfuse must hold it as simulation,
                     with the scenario, session = chat id, coda.run_id and trace id
  alert-to-run       the same turn found from Datadog: trace id and coda.run_id
                     agree across the attempt span, the generation and Langfuse
  isolation          Langfuse holds the canary; the operations spans of the same
                     trace exist with model and usage and hold no canary
  fail               scale langfuse-web to 0 (needs --confirm); the chat still
                     answers, the ops trace arrives, the Langfuse exporter fails
                     and its monitor fires
  recover            scale langfuse-web back (needs --confirm); a new turn lands
                     in Langfuse and the monitor recovers
  signin-breakglass  scale langfuse-sso to 0 (needs --confirm) and check whether
                     Langfuse still offers its documented password sign-in

The operator runs it with gcloud signed in as a physician_rehearsal_signers
account and kubectl pointed at rust-observability. Langfuse, Tempo and Loki are
reached through port-forwards the drill opens itself. Doppler coda-rust/stg
supplies LANGFUSE_INIT_PROJECT_PUBLIC_KEY, LANGFUSE_INIT_PROJECT_SECRET_KEY,
DD_API_KEY and DD_APP_KEY; none is printed.";

const LANGFUSE_EXPORTER: &str = "otlphttp/langfuse";
const DATADOG_API: &str = "https://api.datadoghq.com";
const DATADOG_ENV: &str = "staging";
/// Langfuse's web image carries the `/langfuse` base path in staging.
const LANGFUSE_BASE_PATH: &str = "/langfuse";
const REPLY_POLLS: u32 = 60;
const READBACK_POLLS: u32 = 36;
const MONITOR_POLLS: u32 = 60;
const POLL: Duration = Duration::from_secs(10);

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
    #[error("{what} could not run")]
    Tool {
        what: &'static str,
        #[source]
        source: SignalError,
    },
    #[error("{0} needs --confirm: it scales a staging workload to zero")]
    Unconfirmed(&'static str),
    #[error("{what}: {}", gaps.join("; "))]
    Gaps {
        what: &'static str,
        gaps: Vec<String>,
    },
    #[error("the drill exceeded 40 minutes; langfuse-web was restored")]
    Cap,
    #[error("{0}")]
    Ledger(String),
    #[error("could not write {}", path.display())]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("{primary}; restoring {workload} also failed: {restore}")]
    Restore {
        primary: Box<DrillError>,
        workload: &'static str,
        restore: Box<DrillError>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Rehearse,
    AlertToRun,
    Isolation,
    Fail,
    Recover,
    SigninBreakglass,
}

impl Action {
    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "rehearse" => Self::Rehearse,
            "alert-to-run" => Self::AlertToRun,
            "isolation" => Self::Isolation,
            "fail" => Self::Fail,
            "recover" => Self::Recover,
            "signin-breakglass" => Self::SigninBreakglass,
            _ => return None,
        })
    }

    fn slug(self) -> &'static str {
        match self {
            Self::Rehearse => "rehearse",
            Self::AlertToRun => "alert-to-run",
            Self::Isolation => "isolation",
            Self::Fail => "fail",
            Self::Recover => "recover",
            Self::SigninBreakglass => "signin-breakglass",
        }
    }

    fn scales(self) -> bool {
        matches!(self, Self::Fail | Self::Recover | Self::SigninBreakglass)
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
    if request.action.scales() && !request.confirmed {
        return Err(failure(DrillError::Unconfirmed(request.action.slug())));
    }
    let mut report = Report::new(request.action, Utc::now());
    let outcome = run(&request, &mut report);
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
        let written = std::fs::create_dir_all(dir).and_then(|()| {
            let mut text = serde_json::to_string_pretty(&body).map_err(std::io::Error::other)?;
            text.push('\n');
            std::fs::write(&path, text)
        });
        written.map_err(|source| DrillError::Write {
            path: path.clone(),
            source,
        })?;
        Ok(path)
    }
}

fn run(request: &Request, report: &mut Report) -> Result<(), DrillError> {
    let mut kubectl = Kubectl;
    report.put("revisions", cluster::revisions()?);
    match request.action {
        Action::Rehearse => rehearse(request, report),
        Action::AlertToRun => alert_to_run(request, report),
        Action::Isolation => isolation(request, report),
        Action::Fail => fail(request, report, &mut kubectl),
        Action::Recover => recover(request, report, &mut kubectl),
        Action::SigninBreakglass => signin_breakglass(report, &mut kubectl),
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
    reply_chars: usize,
}

/// Signs in as the rehearsal physician and sends one canary turn on a new
/// chat, under a scenario unique to the run unless one was named.
fn send_turn(request: &Request, report: &mut Report) -> Result<Sent, DrillError> {
    let client = client()?;
    let origin = cluster::physician_origin()?;
    let token = cluster::access_token()?;
    let now = u64::try_from(Utc::now().timestamp()).unwrap_or_default();
    let physician = Physician::sign_in(client, &origin, &token, now)?;
    let workspace = physician.first_workspace()?;
    let run = nonce();
    let canary = format!("coda-canary-{run}");
    let turn = Turn {
        workspace,
        chat: format!("langfuse-drill-{run}"),
        text: format!("Langfuse drill {canary}. Please answer in one short sentence."),
        scenario: request
            .scenario
            .clone()
            .unwrap_or_else(|| format!("langfuse-drill-{run}")),
        key: format!("langfuse-drill-{run}"),
    };
    report.put(
        "turn",
        json!({
            "origin": origin,
            "patient_workspace_id": turn.workspace,
            "chat_id": turn.chat,
            "scenario": turn.scenario,
            "canary": canary,
        }),
    );
    physician.submit(&turn)?;
    let reply = physician.await_reply(&turn, REPLY_POLLS, Duration::from_secs(5))?;
    report.put("reply_chars", reply.chars().count());
    Ok(Sent {
        turn,
        canary,
        reply_chars: reply.chars().count(),
    })
}

/// Langfuse over a drill-owned port-forward, with the staging project keys.
struct Langfuse {
    client: Client,
    base: String,
    public: Secret,
    secret: Secret,
    _forward: Forward,
}

impl Langfuse {
    fn open() -> Result<Self, DrillError> {
        let forward = Forward::open("langfuse-web", 3000)?;
        Ok(Self {
            client: client()?,
            base: format!("{}{LANGFUSE_BASE_PATH}", forward.url()),
            public: secret("LANGFUSE_INIT_PROJECT_PUBLIC_KEY")?,
            secret: secret("LANGFUSE_INIT_PROJECT_SECRET_KEY")?,
            _forward: forward,
        })
    }

    fn get(
        &self,
        path: &str,
        query: &[(&str, &str)],
        what: &'static str,
    ) -> Result<Value, DrillError> {
        chat::send_json(
            self.client
                .get(format!("{}{path}", self.base))
                .basic_auth(self.public.expose(), Some(self.secret.expose()))
                .query(query),
            what,
        )
    }

    /// Waits for the chat's trace and its generation.
    fn turn(&self, sent: &Sent) -> Result<LangfuseTurn, DrillError> {
        for _poll in 0..READBACK_POLLS {
            let traces = self.get(
                "/api/public/traces",
                &[("sessionId", sent.turn.chat.as_str()), ("limit", "10")],
                "Langfuse traces",
            )?;
            if let Some(id) = readback::session_trace(&traces) {
                let trace = self.get(&format!("/api/public/traces/{id}"), &[], "Langfuse trace")?;
                if let Some(turn) = readback::langfuse_turn(&trace, &sent.canary) {
                    return Ok(turn);
                }
            }
            std::thread::sleep(POLL);
        }
        Err(DrillError::Timeout {
            what: "the Langfuse generation",
            seconds: u64::from(READBACK_POLLS) * POLL.as_secs(),
        })
    }

    fn providers(&self) -> Result<Value, DrillError> {
        chat::send_json(
            self.client.get(format!("{}/api/auth/providers", self.base)),
            "Langfuse sign-in providers",
        )
    }
}

/// Datadog's read APIs with the operator keys.
struct Datadog {
    client: Client,
    api: Secret,
    app: Secret,
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

    /// Waits for the scenario's generation span, then reads its whole trace.
    fn generation_trace(&self, scenario: &str) -> Result<(String, Value), DrillError> {
        let query =
            format!("env:{DATADOG_ENV} service:physician-worker @coda.scenario_id:{scenario}");
        for _poll in 0..READBACK_POLLS {
            let found = self.spans(&query)?;
            let trace_id = found["data"]
                .as_array()
                .into_iter()
                .flatten()
                .find_map(|span| span["attributes"]["trace_id"].as_str().map(str::to_owned));
            if let Some(trace_id) = trace_id {
                let trace = self.spans(&format!("trace_id:{trace_id}"))?;
                return Ok((trace_id, trace));
            }
            std::thread::sleep(POLL);
        }
        Err(DrillError::Timeout {
            what: "the Datadog generation span",
            seconds: u64::from(READBACK_POLLS) * POLL.as_secs(),
        })
    }

    fn query(&self, query: &str, from: i64, to: i64) -> Result<Value, DrillError> {
        let (from, to) = (from.to_string(), to.to_string());
        chat::send_json(
            self.authorized(
                self.client
                    .get(format!("{DATADOG_API}/api/v1/query"))
                    .query(&[("query", query), ("from", &from), ("to", &to)]),
            ),
            "Datadog metric query",
        )
    }

    fn exporter_groups(&self) -> Result<Vec<MonitorGroup>, DrillError> {
        let monitors = chat::send_json(
            self.authorized(
                self.client
                    .get(format!("{DATADOG_API}/api/v1/monitor"))
                    .query(&[
                        ("monitor_tags", "service:otel-collector"),
                        ("group_states", "all"),
                    ]),
            ),
            "Datadog monitors",
        )?;
        Ok(readback::exporter_groups(&monitors, LANGFUSE_EXPORTER))
    }
}

fn gaps(what: &'static str, gaps: Vec<String>) -> Result<(), DrillError> {
    if gaps.is_empty() {
        Ok(())
    } else {
        Err(DrillError::Gaps { what, gaps })
    }
}

fn rehearse(request: &Request, report: &mut Report) -> Result<(), DrillError> {
    let langfuse = Langfuse::open()?;
    let sent = send_turn(request, report)?;
    let turn = langfuse.turn(&sent)?;
    report.put("langfuse", &turn);
    gaps(
        "the Langfuse generation",
        readback::rehearsal_gaps(&turn, &sent.turn.chat, &sent.turn.scenario),
    )
}

fn alert_to_run(request: &Request, report: &mut Report) -> Result<(), DrillError> {
    let langfuse = Langfuse::open()?;
    let datadog = Datadog::open()?;
    let sent = send_turn(request, report)?;
    let (dd_trace, trace) = datadog.generation_trace(&sent.turn.scenario)?;
    let turn = langfuse.turn(&sent)?;
    let generation = readback::datadog_span(&trace, &sent.canary);
    let attempt_run = trace["data"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|span| span["attributes"]["resource_name"] == "physician.worker.attempt")
        .and_then(|span| readback::attribute(&span["attributes"], "coda.run_id"))
        .map(str::to_owned);
    report.put(
        "datadog",
        json!({ "trace_id": dd_trace, "attempt_run_id": attempt_run, "generation": generation }),
    );
    report.put("langfuse", &turn);
    let mut found = Vec::new();
    if readback::datadog_trace_id(&turn.trace_id).as_deref() != Some(dd_trace.as_str())
        && !dd_trace.eq_ignore_ascii_case(&turn.trace_id)
    {
        found.push(format!(
            "Datadog trace {dd_trace} is not Langfuse trace {}",
            turn.trace_id
        ));
    }
    let langfuse_run = turn.run_id.as_deref();
    if langfuse_run.is_none() {
        found.push("Langfuse has no coda.run_id".to_owned());
    }
    if generation.run_id.as_deref() != langfuse_run {
        found.push(format!(
            "Datadog generation coda.run_id {:?} is not Langfuse's {langfuse_run:?}",
            generation.run_id
        ));
    }
    if attempt_run.as_deref() != langfuse_run {
        found.push(format!(
            "Datadog attempt coda.run_id {attempt_run:?} is not Langfuse's {langfuse_run:?}"
        ));
    }
    gaps("alert to run", found)
}

fn isolation(request: &Request, report: &mut Report) -> Result<(), DrillError> {
    let langfuse = Langfuse::open()?;
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
    let sent = send_turn(request, report)?;
    let turn = langfuse.turn(&sent)?;
    report.put("langfuse", &turn);
    let mut found = Vec::new();
    if !turn.canary_in_content {
        found.push("Langfuse does not hold the canary; the positive control failed".to_owned());
    }
    let run_id = turn.run_id.as_deref();
    let mut spans: Vec<OpsSpan> = Vec::new();
    if let Some(datadog) = &datadog {
        let (_, trace) = datadog.generation_trace(&sent.turn.scenario)?;
        spans.push(readback::datadog_span(&trace, &sent.canary));
    }
    if let Some(tempo) = &tempo {
        spans.push(tempo_span(tempo, &turn.trace_id, &sent.canary)?);
    }
    for span in &spans {
        found.extend(readback::isolation_gaps(span, run_id));
    }
    report.put("operations", &spans);
    if let Some(loki) = &loki {
        let lines = loki_canary_lines(loki, &sent.canary, started)?;
        report.put("loki_canary_lines", lines);
        if lines > 0 {
            found.push(format!("Loki holds {lines} line(s) with the canary"));
        }
    }
    gaps("content isolation", found)
}

fn tempo_span(tempo: &Forward, trace_id: &str, canary: &str) -> Result<OpsSpan, DrillError> {
    let client = client()?;
    for _poll in 0..READBACK_POLLS {
        let response = client
            .get(format!("{}/api/traces/{trace_id}", tempo.url()))
            .send()
            .map_err(|source| DrillError::Request {
                what: "Tempo trace",
                source,
            })?;
        if response.status().is_success() {
            let body: Value = response.json().map_err(|source| DrillError::Request {
                what: "Tempo trace",
                source,
            })?;
            let span = readback::tempo_span(&body, canary);
            if span.found {
                return Ok(span);
            }
        } else if response.status().as_u16() != 404 {
            return Err(DrillError::Status {
                what: "Tempo trace",
                status: response.status().as_u16(),
            });
        }
        std::thread::sleep(POLL);
    }
    Ok(readback::tempo_span(&Value::Null, canary))
}

fn loki_canary_lines(
    loki: &Forward,
    canary: &str,
    since: DateTime<Utc>,
) -> Result<usize, DrillError> {
    let query = format!("{{service_name=~\"physician.*\"}} |= \"{canary}\"");
    let start = since.timestamp_nanos_opt().unwrap_or_default().to_string();
    let body = chat::send_json(
        client()?
            .get(format!("{}/loki/api/v1/query_range", loki.url()))
            .query(&[
                ("query", query.as_str()),
                ("start", start.as_str()),
                ("limit", "100"),
            ]),
        "Loki query",
    )?;
    Ok(readback::loki_lines(&body))
}

/// The Langfuse exporter's failures and peak queue over the window.
fn exporter_pressure(datadog: &Datadog, from: i64, to: i64) -> Result<Value, DrillError> {
    let scope = format!("env:{DATADOG_ENV},service:otel-collector,exporter:{LANGFUSE_EXPORTER}");
    let mut measured = serde_json::Map::new();
    for metric in ["send_failed_spans", "enqueue_failed_spans"] {
        let body = datadog.query(
            &format!("sum:coda.collector.exporter.{metric}.count{{{scope}}}.as_count()"),
            from,
            to,
        )?;
        measured.insert(metric.to_owned(), json!(readback::series_total(&body)));
    }
    let queue = datadog.query(
        &format!("max:coda.collector.exporter.queue_size{{{scope}}}"),
        from,
        to,
    )?;
    let peak = queue["series"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|series| series["pointlist"].as_array().into_iter().flatten())
        .filter_map(|point| point.get(1).and_then(Value::as_f64))
        .fold(0.0_f64, f64::max);
    measured.insert("queue_size_peak".to_owned(), json!(peak));
    Ok(Value::Object(measured))
}

/// Waits for a Langfuse exporter monitor group to reach `status`, and returns
/// it with the evaluator's own transition time.
fn await_monitor(
    datadog: &Datadog,
    alerting: bool,
    since: DateTime<Utc>,
) -> Result<Option<(MonitorGroup, DateTime<Utc>)>, DrillError> {
    for _poll in 0..MONITOR_POLLS {
        for group in datadog.exporter_groups()? {
            let stamp = if alerting {
                group.last_triggered_ts
            } else {
                group.last_resolved_ts
            };
            let at = stamp.and_then(|seconds| DateTime::<Utc>::from_timestamp(seconds, 0));
            let reached = if alerting {
                group.status == "Alert"
            } else {
                group.status == "OK"
            };
            if let Some(at) = at.filter(|at| reached && *at >= since) {
                return Ok(Some((group, at)));
            }
        }
        std::thread::sleep(POLL);
    }
    Ok(None)
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

/// Restores `workload` after `primary` failed, keeping both errors.
fn restored(
    primary: DrillError,
    workload: &'static str,
    kubectl: &mut Kubectl,
    replicas: u32,
) -> DrillError {
    match kubectl.scale(workload, replicas) {
        Ok(()) => primary,
        Err(restore) => DrillError::Restore {
            primary: Box::new(primary),
            workload,
            restore: Box::new(restore),
        },
    }
}

fn fail(request: &Request, report: &mut Report, kubectl: &mut Kubectl) -> Result<(), DrillError> {
    let datadog = Datadog::open()?;
    let mut exercise = ledger(&request.output)?;
    let now = Utc::now();
    if exercise.breach(now).is_some() {
        let restored = kubectl.scale("langfuse-web", 1);
        exercise.record_signal(
            SignalKind::Restored,
            now,
            "cap reached; langfuse-web scaled to 1",
        );
        save_ledger(&request.output, &exercise, now)?;
        restored?;
        return Err(DrillError::Cap);
    }
    let replicas = kubectl.replicas("langfuse-web")?;
    report.put("langfuse_web_replicas_before", replicas);
    kubectl.scale("langfuse-web", 0)?;
    let failed_at = Utc::now();
    let file = super::observation_file_name(DrillRoute::Langfuse, failed_at);
    if !exercise.is_open() {
        exercise.record_signal(SignalKind::Failure, failed_at, &file);
    }
    let firing = fail_checks(request, report, &datadog, failed_at).and_then(|firing| {
        firing.ok_or(DrillError::Timeout {
            what: "the Langfuse exporter monitor alert",
            seconds: u64::from(MONITOR_POLLS) * POLL.as_secs(),
        })
    });
    let result = match firing {
        Ok((group, at)) => {
            let evidence = format!(
                "Datadog monitor {:?} group exporter:{LANGFUSE_EXPORTER}",
                group.monitor
            );
            exercise.record_evaluator(EvalKind::Firing, at, &evidence);
            Ok(())
        }
        Err(error) => {
            let error = restored(error, "langfuse-web", kubectl, replicas.max(1));
            exercise.record_signal(SignalKind::Restored, Utc::now(), &file);
            Err(error)
        }
    };
    save_ledger(&request.output, &exercise, failed_at)?;
    result
}

/// With Langfuse down: the chat answers, the ops trace arrives, the exporter
/// reports failures or queue, and the monitor fires.
fn fail_checks(
    request: &Request,
    report: &mut Report,
    datadog: &Datadog,
    failed_at: DateTime<Utc>,
) -> Result<Option<(MonitorGroup, DateTime<Utc>)>, DrillError> {
    let sent = send_turn(request, report)?;
    report.put("reply_chars_while_down", sent.reply_chars);
    let (trace_id, trace) = datadog.generation_trace(&sent.turn.scenario)?;
    let span = readback::datadog_span(&trace, &sent.canary);
    report.put(
        "datadog",
        json!({ "trace_id": trace_id, "generation": span }),
    );
    gaps(
        "the ops trace while Langfuse is down",
        readback::isolation_gaps(&span, None),
    )?;
    let mut pressure = Value::Null;
    for _poll in 0..READBACK_POLLS {
        pressure = exporter_pressure(datadog, failed_at.timestamp() - 60, Utc::now().timestamp())?;
        let rose = [
            "send_failed_spans",
            "enqueue_failed_spans",
            "queue_size_peak",
        ]
        .iter()
        .any(|key| pressure[*key].as_f64().unwrap_or_default() > 0.0);
        if rose {
            break;
        }
        std::thread::sleep(POLL);
    }
    report.put("exporter", &pressure);
    let rose = [
        "send_failed_spans",
        "enqueue_failed_spans",
        "queue_size_peak",
    ]
    .iter()
    .any(|key| pressure[*key].as_f64().unwrap_or_default() > 0.0);
    if !rose {
        return Err(DrillError::Gaps {
            what: "the Langfuse exporter",
            gaps: vec!["no send failure, enqueue failure or queue was reported".to_owned()],
        });
    }
    let firing = await_monitor(datadog, true, failed_at)?;
    report.put("monitor_firing", firing.as_ref().map(|(group, _)| group));
    Ok(firing)
}

fn recover(
    request: &Request,
    report: &mut Report,
    kubectl: &mut Kubectl,
) -> Result<(), DrillError> {
    let datadog = Datadog::open()?;
    let mut exercise = ledger(&request.output)?;
    kubectl.scale("langfuse-web", 1)?;
    kubectl.rollout("langfuse-web")?;
    let restored_at = Utc::now();
    let file = super::observation_file_name(DrillRoute::Langfuse, restored_at);
    if exercise.is_open() {
        exercise.record_signal(SignalKind::Restored, restored_at, &file);
    }
    let landed = (|| {
        let langfuse = Langfuse::open()?;
        let sent = send_turn(request, report)?;
        let turn = langfuse.turn(&sent)?;
        report.put("langfuse", &turn);
        gaps(
            "the Langfuse generation after recovery",
            readback::rehearsal_gaps(&turn, &sent.turn.chat, &sent.turn.scenario),
        )?;
        let recovered = await_monitor(&datadog, false, restored_at)?;
        report.put(
            "monitor_recovered",
            recovered.as_ref().map(|(group, _)| group),
        );
        let (group, at) = recovered.ok_or(DrillError::Timeout {
            what: "the Langfuse exporter monitor recovery",
            seconds: u64::from(MONITOR_POLLS) * POLL.as_secs(),
        })?;
        exercise.record_evaluator(
            EvalKind::Recovered,
            at,
            &format!(
                "Datadog monitor {:?} group exporter:{LANGFUSE_EXPORTER}",
                group.monitor
            ),
        );
        Ok(())
    })();
    save_ledger(&request.output, &exercise, restored_at)?;
    landed
}

/// Under the SSO issuer, passwords are off on every route
/// (`AUTH_DISABLE_USERNAME_PASSWORD`). With the issuer down, the break-glass
/// is real only if Langfuse still offers password sign-in.
fn signin_breakglass(report: &mut Report, kubectl: &mut Kubectl) -> Result<(), DrillError> {
    let replicas = kubectl.replicas("langfuse-sso")?;
    report.put("langfuse_sso_replicas_before", replicas);
    kubectl.scale("langfuse-sso", 0)?;
    let checked = (|| {
        let langfuse = Langfuse::open()?;
        let providers = langfuse.providers()?;
        let names: Vec<&String> = providers
            .as_object()
            .map(|map| map.keys().collect())
            .unwrap_or_default();
        report.put("providers_with_issuer_down", &names);
        if readback::offers_password(&providers) {
            Ok(())
        } else {
            Err(DrillError::Gaps {
                what: "break-glass sign-in",
                gaps: vec![
                    "with langfuse-sso down Langfuse offers no password sign-in on port-forward"
                        .to_owned(),
                ],
            })
        }
    })();
    let restore = kubectl.scale("langfuse-sso", replicas.max(1));
    match (checked, restore) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) => Err(error),
        (Ok(()), Err(error)) => Err(error),
        (Err(primary), Err(restore)) => Err(DrillError::Restore {
            primary: Box::new(primary),
            workload: "langfuse-sso",
            restore: Box::new(restore),
        }),
    }
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
            "fail",
            "--output-dir",
            "out",
            "--confirm",
            "--backend",
            "datadog",
            "--scenario",
            "s-1",
        ]))
        .unwrap();
        assert_eq!(request.action, Action::Fail);
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
    }

    #[test]
    fn scaling_actions_need_confirmation() {
        let refused = from_args(&args(&["fail", "--output-dir", "/nonexistent/x"])).unwrap_err();
        assert!(refused.message.contains("--confirm"), "{}", refused.message);
        assert!(Action::SigninBreakglass.scales() && !Action::Isolation.scales());
    }

    #[test]
    fn nonces_differ() {
        assert_ne!(nonce(), nonce());
        assert_eq!(nonce().len(), 16);
    }
}
