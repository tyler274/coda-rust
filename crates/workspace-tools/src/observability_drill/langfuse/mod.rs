//! `observability-drill langfuse`: drives a rehearsal chat turn on staging as
//! the physician rehearsal identity and reads it back from Langfuse and the
//! operations backends.
//!
//! Every check writes `langfuse-<action>-<stamp>.json` with the deployed
//! revisions it ran against. `fail` and `recover` also write the alert ledger
//! `observations-langfuse-<stamp>.json` for `qualify-alerts.py`. Every wait
//! runs under the exercise's 40-minute cap and the operator's interrupt, and
//! a scaled-down workload is restored on every exit but a passed `fail`.
mod chat;
mod cluster;
mod readback;
mod watch;

use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, SET_COOKIE};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use super::live::{STAGING_PROJECT, Secret, fetch_secret};
use super::model::{DrillRoute, EvalKind, Exercise, SignalError, SignalKind};
use crate::failure::Failure;
use chat::{Physician, Turn};
use cluster::{Forward, Kubectl, Workload};
use readback::{LangfuseTurn, MonitorGroup, OpsSpan, PasswordSignIn};
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
  fail               scale langfuse-web to 0 (needs --confirm); the chat still
                     answers, the ops trace arrives, the Langfuse exporter fails
                     and its monitor fires; langfuse-web stays down for recover
  recover            scale langfuse-web back to its count before fail (needs
                     --confirm); new turns land in Langfuse and the monitor recovers
  signin-breakglass  scale langfuse-sso to 0 (needs --confirm) and try the init
                     admin's password sign-in through a port-forward

The operator runs it with gcloud signed in as a physician_rehearsal_signers
account and the rust-observability kubeconfig context; kubectl always names
that context. Langfuse, Tempo and Loki are reached through port-forwards the
drill opens itself. Doppler coda-rust/stg supplies
LANGFUSE_INIT_PROJECT_PUBLIC_KEY, LANGFUSE_INIT_PROJECT_SECRET_KEY,
LANGFUSE_INIT_USER_EMAIL, LANGFUSE_INIT_USER_PASSWORD, DD_API_KEY and
DD_APP_KEY; none is printed. Ctrl-C restores what the drill scaled down.";

const LANGFUSE_EXPORTER: &str = "otlphttp/langfuse";
const DATADOG_API: &str = "https://api.datadoghq.com";
const DATADOG_ENV: &str = "staging";
/// Langfuse's web image carries the `/langfuse` base path in staging.
const LANGFUSE_BASE_PATH: &str = "/langfuse";
/// What `fail` leaves for `recover`: the replica count to restore.
const HANDOFF: &str = "langfuse-web-down.json";
const REPLY_POLLS: u32 = 60;
const READBACK_POLLS: u32 = 36;
const MONITOR_POLLS: u32 = 60;
const POLL: Duration = Duration::from_secs(10);
/// How often `recover` sends a turn while it waits for the monitor, so the
/// exporter keeps reporting healthy delivery through the 5-minute window.
const RECOVERY_TRAFFIC: u32 = 6;

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
    #[error("{what} printed {text:?}, not a count")]
    Count {
        what: &'static str,
        text: String,
        #[source]
        source: std::num::ParseIntError,
    },
    #[error("kubectl port-forward to {0} printed no local port")]
    NotForwarded(&'static str),
    #[error("{0} is already at 0 replicas; restore it before drilling")]
    AlreadyDown(&'static str),
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
    #[error("the exercise passed its 40-minute cap")]
    Cap,
    #[error("the operator interrupted the drill")]
    Interrupted,
    #[error("recover could not read {HANDOFF}, which fail leaves in the output directory")]
    NoHandoff {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("the alert ledger could not be read: {0}")]
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
    #[error("{primary}; recording it in the alert ledger also failed: {ledger}")]
    Unrecorded {
        #[source]
        primary: Box<DrillError>,
        ledger: Box<DrillError>,
    },
    #[error(
        "the Langfuse exporter monitor resolved at {resolved} before langfuse-web was restored \
         at {restored}, so its recovery cannot be read back; run recover promptly after fail"
    )]
    ResolvedEarly {
        resolved: DateTime<Utc>,
        restored: DateTime<Utc>,
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
    let mut kubectl = Kubectl;
    report.put("revisions", cluster::revisions()?);
    match request.action {
        Action::Rehearse => rehearse(request, report, watch),
        Action::AlertToRun => alert_to_run(request, report, watch),
        Action::Isolation => isolation(request, report, watch),
        Action::Fail => fail(request, report, &mut kubectl, watch),
        Action::Recover => recover(request, report, &mut kubectl, watch),
        Action::SigninBreakglass => signin_breakglass(report, &mut kubectl, watch),
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
}

/// Signs in as the rehearsal physician and sends one canary turn on a new
/// chat, under a scenario unique to the run unless one was named.
fn send_turn(request: &Request, report: &mut Report, watch: &Watch) -> Result<Sent, DrillError> {
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
    physician.submit(&turn)?;
    let answer = physician.await_reply(&turn, REPLY_POLLS, Duration::from_secs(5), watch)?;
    let reply_chars = answer.text.chars().count();
    report.put(
        "turn",
        json!({
            "origin": origin,
            "patient_workspace_id": turn.workspace,
            "chat_id": turn.chat,
            "scenario": turn.scenario,
            "canary": canary,
            "attempt_id": answer.attempt,
            "reply_chars": reply_chars,
        }),
    );
    Ok(Sent {
        turn,
        canary,
        attempt: answer.attempt,
        reply_chars,
    })
}

/// Langfuse over a drill-owned port-forward.
struct Langfuse {
    client: Client,
    base: String,
    _forward: Forward,
}

impl Langfuse {
    fn open() -> Result<Self, DrillError> {
        let forward = Forward::open("langfuse-web", 3000)?;
        Ok(Self {
            client: client()?,
            base: format!("{}{LANGFUSE_BASE_PATH}", forward.url()),
            _forward: forward,
        })
    }

    /// The public API with the staging project keys.
    fn api(&self) -> Result<LangfuseApi<'_>, DrillError> {
        Ok(LangfuseApi {
            langfuse: self,
            public: secret("LANGFUSE_INIT_PROJECT_PUBLIC_KEY")?,
            secret: secret("LANGFUSE_INIT_PROJECT_SECRET_KEY")?,
        })
    }

    /// Tries the init admin's password sign-in the way a browser does: CSRF
    /// token, credentials callback, then the session it left. Cookies are
    /// carried by hand because NextAuth marks them `Secure` and the
    /// port-forward is plain HTTP.
    fn password_sign_in(&self) -> Result<PasswordSignIn, DrillError> {
        let email = secret("LANGFUSE_INIT_USER_EMAIL")?;
        let password = secret("LANGFUSE_INIT_USER_PASSWORD")?;
        let mut jar = Jar::default();
        let response = self
            .client
            .get(format!("{}/api/auth/csrf", self.base))
            .send()
            .map_err(|source| DrillError::Request {
                what: "Langfuse CSRF token",
                source,
            })?;
        jar.take(&response);
        let csrf: Value = json_body(response, "Langfuse CSRF token")?;
        let token = csrf["csrfToken"].as_str().ok_or(DrillError::Shape {
            what: "Langfuse CSRF token",
            field: "csrfToken",
        })?;
        let callback_url = format!("{}/", self.base);
        let response = self
            .client
            .post(format!("{}/api/auth/callback/credentials", self.base))
            .header(COOKIE, jar.header())
            .form(&[
                ("csrfToken", token),
                ("email", email.expose()),
                ("password", password.expose()),
                ("callbackUrl", callback_url.as_str()),
                ("json", "true"),
            ])
            .send()
            .map_err(|source| DrillError::Request {
                what: "Langfuse credentials sign-in",
                source,
            })?;
        jar.take(&response);
        // NextAuth answers a refused sign-in with 401 and an error URL, so the
        // status alone decides nothing; the URL and the session do.
        let callback: Value = response.json().map_err(|source| DrillError::Request {
            what: "Langfuse credentials sign-in",
            source,
        })?;
        let session = chat::send_json(
            self.client
                .get(format!("{}/api/auth/session", self.base))
                .header(COOKIE, jar.header()),
            "Langfuse session",
        )?;
        Ok(readback::password_sign_in(&callback, &session))
    }
}

/// The NextAuth cookies one sign-in attempt sets, sent back by name.
#[derive(Default)]
struct Jar(Vec<(String, String)>);

impl Jar {
    fn take(&mut self, response: &reqwest::blocking::Response) {
        for value in response.headers().get_all(SET_COOKIE) {
            let Some(pair) = value.to_str().ok().and_then(|v| v.split(';').next()) else {
                continue;
            };
            let Some((name, value)) = pair.split_once('=') else {
                continue;
            };
            self.0.retain(|(known, _)| known != name);
            self.0
                .push((name.trim().to_owned(), value.trim().to_owned()));
        }
    }

    fn header(&self) -> String {
        self.0
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ")
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
        path: &str,
        query: &[(&str, &str)],
        what: &'static str,
    ) -> Result<Value, DrillError> {
        chat::send_json(
            self.langfuse
                .client
                .get(format!("{}{path}", self.langfuse.base))
                .basic_auth(self.public.expose(), Some(self.secret.expose()))
                .query(query),
            what,
        )
    }

    /// Waits for the chat's trace and its generation.
    fn turn(&self, sent: &Sent, watch: &Watch) -> Result<LangfuseTurn, DrillError> {
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
            watch.pause(POLL)?;
        }
        Err(DrillError::Timeout {
            what: "the Langfuse generation",
            seconds: u64::from(READBACK_POLLS) * POLL.as_secs(),
        })
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
        for _poll in 0..READBACK_POLLS {
            let found = self.spans(&query)?;
            let trace_id = found["data"]
                .as_array()
                .into_iter()
                .flatten()
                .find_map(|span| span["attributes"]["trace_id"].as_str().map(str::to_owned));
            if let Some(trace_id) = trace_id {
                let body = self.spans(&format!("trace_id:{trace_id}"))?;
                if readback::datadog_span(&body, "").found {
                    return Ok(DatadogTrace { trace_id, body });
                }
            }
            watch.pause(POLL)?;
        }
        Err(DrillError::Timeout {
            what: "the Datadog attempt and generation spans",
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
    let langfuse = Langfuse::open()?;
    let api = langfuse.api()?;
    let sent = send_turn(request, report, watch)?;
    let turn = api.turn(&sent, watch)?;
    report.put("langfuse", &turn);
    let mut found = readback::rehearsal_gaps(&turn, &sent.turn.chat, &sent.turn.scenario);
    found.extend(attempt_gaps(&sent, &turn));
    gaps("the Langfuse generation", found)
}

fn alert_to_run(request: &Request, report: &mut Report, watch: &Watch) -> Result<(), DrillError> {
    let langfuse = Langfuse::open()?;
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
    let langfuse = Langfuse::open()?;
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
    if let Some(datadog) = &datadog {
        let trace = datadog.attempt_trace(&sent.attempt, watch)?;
        if !readback::same_trace(&trace.trace_id, &turn.trace_id) {
            found.push(format!(
                "Datadog trace {} is not Langfuse trace {}",
                trace.trace_id, turn.trace_id
            ));
        }
        spans.push(readback::datadog_span(&trace.body, &sent.canary));
    }
    if let Some(tempo) = &tempo {
        spans.push(tempo_span(tempo, &turn.trace_id, &sent.canary, watch)?);
    }
    for span in &spans {
        found.extend(readback::isolation_gaps(span, attempt));
    }
    report.put("operations", &spans);
    if let Some(loki) = &loki {
        let lines = loki_isolation(loki, &sent, &turn.trace_id, started, watch)?;
        report.put("loki", &lines);
        found.extend(lines.gaps());
    }
    gaps("content isolation", found)
}

fn tempo_span(
    tempo: &Forward,
    trace_id: &str,
    canary: &str,
    watch: &Watch,
) -> Result<OpsSpan, DrillError> {
    let client = client()?;
    for _poll in 0..READBACK_POLLS {
        let response = client
            .get(format!("{}/api/traces/{trace_id}", tempo.url()))
            .send()
            .map_err(|source| DrillError::Request {
                what: "Tempo trace",
                source,
            })?;
        if response.status().as_u16() != 404 {
            let body = json_body(response, "Tempo trace")?;
            let span = readback::tempo_span(&body, canary);
            if span.found {
                return Ok(span);
            }
        }
        watch.pause(POLL)?;
    }
    Ok(readback::tempo_span(&Value::Null, canary))
}

/// Loki's answer for one turn: the physician logs of its trace (the positive
/// control that its logs arrive at all) and any line with the canary.
#[derive(serde::Serialize)]
struct LokiIsolation {
    trace_lines: usize,
    canary_lines: usize,
}

impl LokiIsolation {
    fn gaps(&self) -> Vec<String> {
        let mut gaps = Vec::new();
        if self.trace_lines == 0 {
            gaps.push(
                "Loki holds no physician log for the turn's trace; an empty answer is not proof"
                    .to_owned(),
            );
        }
        if self.canary_lines > 0 {
            gaps.push(format!(
                "Loki holds {} line(s) with the canary",
                self.canary_lines
            ));
        }
        gaps
    }
}

/// Waits for the physician logs of the turn's trace, then counts canary lines in
/// every physician stream over the same window.
fn loki_isolation(
    loki: &Forward,
    sent: &Sent,
    trace_id: &str,
    since: DateTime<Utc>,
    watch: &Watch,
) -> Result<LokiIsolation, DrillError> {
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
    let mut trace_lines = 0;
    for _poll in 0..READBACK_POLLS {
        trace_lines = count(&traced)?;
        if trace_lines > 0 {
            break;
        }
        watch.pause(POLL)?;
    }
    let canary_lines = count(&format!(
        "{{service_name=~\"physician.*\"}} |= \"{}\"",
        sent.canary
    ))?;
    Ok(LokiIsolation {
        trace_lines,
        canary_lines,
    })
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

fn pressure_rose(pressure: &Value) -> bool {
    [
        "send_failed_spans",
        "enqueue_failed_spans",
        "queue_size_peak",
    ]
    .iter()
    .any(|key| pressure[*key].as_f64().unwrap_or_default() > 0.0)
}

/// Whether a Langfuse exporter monitor group fired after `since`, or every
/// group has resolved after it.
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

/// When every group is already OK but none resolved after `since`, the
/// monitor resolved while langfuse-web was still down (on no data, say) and no
/// later transition will come; returns that resolution.
fn resolved_before(groups: &[MonitorGroup], since: DateTime<Utc>) -> Option<DateTime<Utc>> {
    if groups.is_empty() || groups.iter().any(|group| group.status != "OK") {
        return None;
    }
    let latest = groups
        .iter()
        .filter_map(|group| DateTime::<Utc>::from_timestamp(group.last_resolved_ts?, 0))
        .max()?;
    (latest < since).then_some(latest)
}

/// Waits for the monitor. `between` runs before each poll, so recovery can
/// keep traffic flowing while it waits.
fn await_monitor(
    datadog: &Datadog,
    alerting: bool,
    since: DateTime<Utc>,
    watch: &Watch,
    mut between: impl FnMut(u32) -> Result<(), DrillError>,
) -> Result<Option<(MonitorGroup, DateTime<Utc>)>, DrillError> {
    for poll in 0..MONITOR_POLLS {
        between(poll)?;
        let groups = datadog.exporter_groups()?;
        if let Some(reached) = monitor_reached(&groups, alerting, since) {
            return Ok(Some(reached));
        }
        if let Some(resolved) = resolved_before(&groups, since).filter(|_| !alerting) {
            return Err(DrillError::ResolvedEarly {
                resolved,
                restored: since,
            });
        }
        watch.pause(POLL)?;
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

/// Scales langfuse-web down and requires, with it down, the chat answer, the
/// ops trace, exporter pressure and the monitor firing. On success
/// langfuse-web stays down for `recover`, with its count in [`HANDOFF`]; on
/// any failure, the cap or Ctrl-C, it is restored first.
fn fail(
    request: &Request,
    report: &mut Report,
    kubectl: &mut Kubectl,
    watch: &mut Watch,
) -> Result<(), DrillError> {
    let datadog = Datadog::open()?;
    let mut exercise = ledger(&request.output)?;
    if exercise.is_open() {
        return Err(DrillError::Ledger(
            "an exercise is already open; run recover first".to_owned(),
        ));
    }
    let down = kubectl.scale_down(Workload::LangfuseWeb)?;
    let failed_at = Utc::now();
    watch.opened_at(failed_at);
    report.put("langfuse_web_replicas_before", down.replicas());
    let file = super::observation_file_name(DrillRoute::Langfuse, failed_at);
    exercise.record_signal(SignalKind::Failure, failed_at, &file);
    let handoff = request.output.join(HANDOFF);
    let checked = write_json(
        &handoff,
        &json!({ "replicas": down.replicas(), "failed_at": failed_at.to_rfc3339() }),
    )
    .and_then(|()| fail_checks(request, report, &datadog, failed_at, watch));
    // langfuse-web stays down only once the ledger holds the open exercise;
    // `recover` needs both it and the handoff.
    let checked = checked.and_then(|(group, at)| {
        exercise.record_evaluator(
            EvalKind::Firing,
            at,
            &format!(
                "Datadog monitor {:?} group exporter:{LANGFUSE_EXPORTER}",
                group.monitor
            ),
        );
        save_ledger(&request.output, &exercise, failed_at)
    });
    let Err(error) = checked else {
        down.leave_down();
        return Ok(());
    };
    let (error, back) = down.restore_after(error, kubectl);
    if back {
        exercise.record_signal(SignalKind::Restored, Utc::now(), &file);
        if let Err(stale) = remove_handoff(&handoff) {
            report.put("stale_handoff", stale.to_string());
        }
    }
    report.put("langfuse_web_restored", back);
    with_ledger(
        Err(error),
        save_ledger(&request.output, &exercise, failed_at),
    )
}

/// A time `fail` or `recover` wrote into the handoff.
fn stamp(handoff: &Value, key: &str) -> Option<DateTime<Utc>> {
    let at = DateTime::parse_from_rfc3339(handoff[key].as_str()?).ok()?;
    Some(at.with_timezone(&Utc))
}

/// Removes the handoff once langfuse-web is back; a missing one is fine.
fn remove_handoff(handoff: &Path) -> Result<(), DrillError> {
    match std::fs::remove_file(handoff) {
        Err(source) if source.kind() != std::io::ErrorKind::NotFound => Err(DrillError::Write {
            path: handoff.to_owned(),
            source,
        }),
        _ => Ok(()),
    }
}

fn fail_checks(
    request: &Request,
    report: &mut Report,
    datadog: &Datadog,
    failed_at: DateTime<Utc>,
    watch: &Watch,
) -> Result<(MonitorGroup, DateTime<Utc>), DrillError> {
    let sent = send_turn(request, report, watch)?;
    report.put("reply_chars_while_down", sent.reply_chars);
    let trace = datadog.attempt_trace(&sent.attempt, watch)?;
    let span = readback::datadog_span(&trace.body, &sent.canary);
    report.put(
        "datadog",
        json!({ "trace_id": trace.trace_id, "generation": span }),
    );
    gaps(
        "the ops trace while Langfuse is down",
        readback::isolation_gaps(&span, Some(&sent.attempt)),
    )?;
    let mut pressure = Value::Null;
    for _poll in 0..READBACK_POLLS {
        pressure = exporter_pressure(datadog, failed_at.timestamp() - 60, Utc::now().timestamp())?;
        if pressure_rose(&pressure) {
            break;
        }
        watch.pause(POLL)?;
    }
    report.put("exporter", &pressure);
    if !pressure_rose(&pressure) {
        return Err(DrillError::Gaps {
            what: "the Langfuse exporter",
            gaps: vec!["no send failure, enqueue failure or queue was reported".to_owned()],
        });
    }
    let firing = await_monitor(datadog, true, failed_at, watch, |_| Ok(()))?;
    report.put("monitor_firing", firing.as_ref().map(|(group, _)| group));
    firing.ok_or(DrillError::Timeout {
        what: "the Langfuse exporter monitor alert",
        seconds: u64::from(MONITOR_POLLS) * POLL.as_secs(),
    })
}

/// Restores langfuse-web to the count `fail` recorded, first of all, then
/// requires new turns in Langfuse and the monitor's recovery while rehearsal
/// traffic keeps the exporter reporting.
fn recover(
    request: &Request,
    report: &mut Report,
    kubectl: &mut Kubectl,
    watch: &mut Watch,
) -> Result<(), DrillError> {
    let datadog = Datadog::open()?;
    let handoff = request.output.join(HANDOFF);
    let handoff_error =
        |source: Box<dyn std::error::Error + Send + Sync>| DrillError::NoHandoff { source };
    let text = std::fs::read_to_string(&handoff).map_err(|error| handoff_error(error.into()))?;
    let recorded: Value =
        serde_json::from_str(&text).map_err(|error| handoff_error(error.into()))?;
    let replicas = recorded["replicas"]
        .as_u64()
        .and_then(|count| u32::try_from(count).ok())
        .filter(|count| *count > 0)
        .ok_or_else(|| handoff_error("no positive replicas count".into()))?;
    // A retried recover verifies against the first restore, which the
    // handoff keeps until verification passes, and continues the exercise
    // that restore closed in the ledger.
    let earlier = stamp(&recorded, "restored_at");
    let mut exercise = match earlier {
        Some(_) => {
            super::load_latest(&request.output, DrillRoute::Langfuse).map_err(DrillError::Ledger)?
        }
        None => ledger(&request.output)?,
    };
    if let Some(opened) = exercise
        .open_since()
        .or_else(|| stamp(&recorded, "failed_at"))
    {
        watch.opened_at(opened);
    }
    kubectl.scale(Workload::LangfuseWeb, replicas)?;
    report.put("langfuse_web_replicas_restored", replicas);
    let restored_at = match earlier {
        Some(at) => at,
        None => {
            let now = Utc::now();
            let mut updated = recorded.clone();
            updated["restored_at"] = json!(now.to_rfc3339());
            write_json(&handoff, &updated)?;
            let file = super::observation_file_name(DrillRoute::Langfuse, now);
            exercise.record_signal(SignalKind::Restored, now, &file);
            save_ledger(&request.output, &exercise, now)?;
            now
        }
    };
    report.put("langfuse_web_restored_at", restored_at.to_rfc3339());
    let (group, at) = recover_checks(request, report, &datadog, restored_at, watch)?;
    exercise.record_evaluator(
        EvalKind::Recovered,
        at,
        &format!(
            "Datadog monitor {:?} group exporter:{LANGFUSE_EXPORTER}",
            group.monitor
        ),
    );
    save_ledger(&request.output, &exercise, Utc::now())?;
    remove_handoff(&handoff)
}

fn recover_checks(
    request: &Request,
    report: &mut Report,
    datadog: &Datadog,
    restored_at: DateTime<Utc>,
    watch: &Watch,
) -> Result<(MonitorGroup, DateTime<Utc>), DrillError> {
    let langfuse = Langfuse::open()?;
    let api = langfuse.api()?;
    let sent = send_turn(request, report, watch)?;
    let turn = api.turn(&sent, watch)?;
    report.put("langfuse", &turn);
    let mut found = readback::rehearsal_gaps(&turn, &sent.turn.chat, &sent.turn.scenario);
    found.extend(attempt_gaps(&sent, &turn));
    gaps("the Langfuse generation after recovery", found)?;
    let mut traffic = 0_u32;
    let recovered = await_monitor(datadog, false, restored_at, watch, |poll| {
        if poll > 0 && poll % RECOVERY_TRAFFIC == 0 {
            let mut scratch = Report::new(Action::Recover, Utc::now());
            send_turn(request, &mut scratch, watch)?;
            traffic += 1;
        }
        Ok(())
    })?;
    report.put("recovery_traffic_turns", traffic);
    report.put(
        "monitor_recovered",
        recovered.as_ref().map(|(group, _)| group),
    );
    recovered.ok_or(DrillError::Timeout {
        what: "the Langfuse exporter monitor recovery",
        seconds: u64::from(MONITOR_POLLS) * POLL.as_secs(),
    })
}

/// Under the SSO issuer, passwords are off on every route
/// (`AUTH_DISABLE_USERNAME_PASSWORD`). With the issuer down, the break-glass
/// is real only if the init admin can still sign in by password; the drill
/// tries it, and restores langfuse-sso on every exit.
fn signin_breakglass(
    report: &mut Report,
    kubectl: &mut Kubectl,
    watch: &Watch,
) -> Result<(), DrillError> {
    let down = kubectl.scale_down(Workload::LangfuseSso)?;
    report.put("langfuse_sso_replicas_before", down.replicas());
    let checked = watch.check().and_then(|()| {
        let langfuse = Langfuse::open()?;
        let outcome = langfuse.password_sign_in()?;
        report.put("password_sign_in_with_issuer_down", outcome);
        match outcome {
            PasswordSignIn::SignedIn => Ok(()),
            PasswordSignIn::Refused => Err(DrillError::Gaps {
                what: "break-glass sign-in",
                gaps: vec![
                    "with langfuse-sso down the init admin's password sign-in is refused"
                        .to_owned(),
                ],
            }),
        }
    });
    match checked {
        Ok(()) => down.restore(kubectl),
        Err(error) => Err(down.restore_after(error, kubectl).0),
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
    fn a_resolution_before_the_restore_is_reported_not_awaited() {
        let since = DateTime::<Utc>::from_timestamp(100, 0).unwrap();
        let early = resolved_before(&[group("OK", Some(40), Some(90))], since);
        assert_eq!(early.map(|at| at.timestamp()), Some(90));
        assert!(resolved_before(&[group("OK", None, Some(150))], since).is_none());
        assert!(resolved_before(&[group("Alert", Some(40), Some(90))], since).is_none());
        assert!(resolved_before(&[], since).is_none());
    }

    #[test]
    fn nonces_differ() {
        assert_ne!(nonce(), nonce());
        assert_eq!(nonce().len(), 16);
    }
}
