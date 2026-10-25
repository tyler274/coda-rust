//! Codex CLI transport for physician chat.
//!
//! `live` and `record` spawn `codex exec` against the subscription from
//! `codex login`. Neither mode reads `OPENAI_API_KEY`. `record` writes a
//! transport cassette beside the process, and `replay` returns those
//! exchanges without starting the CLI. This module is the physician-chat
//! adapter. It does not run the harness tool loop.
//!
//! The cassette schema is version 1. It stores one logical request and the
//! CLI's last message. It is not the coda-agents full-runtime cassette.

#[cfg(test)]
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::sha256_hex;

/// Cassette schema this adapter writes and reads.
pub const CASSETTE_SCHEMA_VERSION: u32 = 1;

/// Adapter identity stored on a cassette.
pub const ADAPTER_ID: &str = "codex-exec";

/// Instructions the CLI receives for a physician chat turn.
pub const PHYSICIAN_CHAT_INSTRUCTIONS: &str = "\
CLI transport: return exactly one final JSON object matching the output schema. \
The text field is the message the physician reads. \
Do not call CLI tools, inspect the workspace, or write any other text.";

const OUTPUT_SCHEMA: &str = r#"{"type":"object","properties":{"text":{"type":"string"}},"required":["text"],"additionalProperties":false}"#;

const MODEL_CATALOG: &str = include_str!("codex_model_catalog.json");

const ISOLATED_CONFIG: &[&str] = &[
    "agents.enabled=false",
    "skills.include_instructions=false",
    "skills.bundled.enabled=false",
    "orchestrator.skills.enabled=false",
    "orchestrator.mcp.enabled=false",
    "include_permissions_instructions=false",
    "include_apps_instructions=false",
    "include_collaboration_mode_instructions=false",
    "tools.experimental_request_user_input.enabled=false",
    "project_doc_max_bytes=0",
];

const DISABLED_CAPABILITIES: &[&str] = &[
    "shell_tool",
    "unified_exec",
    "shell_snapshot",
    "multi_agent",
    "multi_agent_v2",
    "apps",
    "plugins",
    "computer_use",
    "browser_use",
    "browser_use_external",
    "browser_use_full_cdp_access",
    "in_app_browser",
    "image_generation",
    "memories",
    "workspace_dependencies",
    "remote_plugin",
    "tool_suggest",
    "standalone_web_search",
    "hooks",
    "skill_mcp_dependency_install",
    "goals",
    "sleep_tool",
    "view_image",
];

const INHERITED_ENV: &[&str] = &[
    "LANG", "LC_ALL", "LC_CTYPE", "LOGNAME", "PATH", "SHELL", "TMPDIR", "USER",
];

/// Paths and authorization for `record` and `replay`.
///
/// `live` ignores every field. None of them is an API key.
#[derive(Debug, Clone, Copy, Default)]
pub struct CodexSettings<'a> {
    /// `CODA_CASSETTE_AUTHORIZATION`. Required for `record`.
    pub cassette_authorization: Option<&'a str>,
    /// `CODA_CASSETTE_OUTPUT`. Required for `record`.
    pub cassette_output: Option<&'a str>,
    /// `CODA_CASSETTE_PATH`. Required for `replay`.
    pub cassette_path: Option<&'a str>,
}

/// Opens the Codex transport for `execution`.
///
/// `api` is refused here. Live, record, and replay never read an API key.
pub fn open_codex(
    execution: crate::ModelExecution,
    settings: CodexSettings<'_>,
) -> Result<CodexTransport, String> {
    match execution {
        crate::ModelExecution::Live => Ok(CodexTransport::live()),
        crate::ModelExecution::Record => {
            let authorization = settings.cassette_authorization.unwrap_or("").trim();
            if authorization.is_empty() {
                return Err("Record mode requires CODA_CASSETTE_AUTHORIZATION".to_owned());
            }
            let output = settings.cassette_output.unwrap_or("").trim();
            if output.is_empty() {
                return Err("Record mode requires CODA_CASSETTE_OUTPUT".to_owned());
            }
            CodexTransport::recording(
                Box::new(CodexProcess::new()),
                authorization,
                PathBuf::from(output),
            )
        }
        crate::ModelExecution::Replay => {
            let path = settings.cassette_path.unwrap_or("").trim();
            if path.is_empty() {
                return Err("Replay mode requires CODA_CASSETTE_PATH".to_owned());
            }
            CodexTransport::replay_path(Path::new(path))
        }
        crate::ModelExecution::Api => {
            Err("api calls OpenAI Responses and does not use the Codex CLI".to_owned())
        }
    }
}

/// One physician or model turn inside a Codex request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodexTurn {
    /// `user` or `assistant`.
    pub role: String,
    /// The authored text.
    pub text: String,
}

/// The logical request a cassette exchange is keyed by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodexRequest {
    /// Catalog model slug.
    pub model: String,
    /// Reasoning effort.
    pub effort: String,
    /// The whole system prompt.
    pub instructions: String,
    /// Conversation so far, including the new physician turn.
    pub turns: Vec<CodexTurn>,
}

impl CodexRequest {
    /// Builds the request the CLI and the cassette key share.
    pub fn from_responses(body: &Value) -> Result<Self, String> {
        let model = body
            .get("model")
            .and_then(Value::as_str)
            .ok_or_else(|| "Codex request is missing a model".to_owned())?;
        let effort = body
            .pointer("/reasoning/effort")
            .and_then(Value::as_str)
            .ok_or_else(|| "Codex request is missing a reasoning effort".to_owned())?;
        let input = body
            .get("input")
            .and_then(Value::as_array)
            .ok_or_else(|| "Codex request is missing input turns".to_owned())?;
        let mut turns = Vec::with_capacity(input.len());
        for item in input {
            let role = item
                .get("role")
                .and_then(Value::as_str)
                .ok_or_else(|| "Codex request input must be role and text".to_owned())?;
            let text = item
                .get("content")
                .and_then(Value::as_str)
                .ok_or_else(|| "Codex request input must be role and text".to_owned())?;
            turns.push(CodexTurn {
                role: role.to_owned(),
                text: text.to_owned(),
            });
        }
        Ok(Self {
            model: model.to_owned(),
            effort: effort.to_owned(),
            instructions: PHYSICIAN_CHAT_INSTRUCTIONS.to_owned(),
            turns,
        })
    }

    /// SHA-256 of the logical request. Occurrence ids stay out of the key.
    pub fn request_key(&self) -> String {
        #[derive(Serialize)]
        struct KeyBody<'a> {
            schema_version: u32,
            model: &'a str,
            effort: &'a str,
            instructions: &'a str,
            input: Vec<KeyTurn<'a>>,
        }
        #[derive(Serialize)]
        struct KeyTurn<'a> {
            role: &'a str,
            text: &'a str,
        }
        let body = KeyBody {
            schema_version: 1,
            model: &self.model,
            effort: &self.effort,
            instructions: &self.instructions,
            input: self
                .turns
                .iter()
                .map(|turn| KeyTurn {
                    role: &turn.role,
                    text: &turn.text,
                })
                .collect(),
        };
        harness::transport::request_key(&body).expect("request key")
    }

    fn prompt(&self) -> String {
        let mut prompt = String::new();
        for (index, turn) in self.turns.iter().enumerate() {
            if index > 0 {
                prompt.push_str("\n\n");
            }
            prompt.push_str(&turn.role);
            prompt.push_str(":\n");
            prompt.push_str(&turn.text);
        }
        prompt
    }
}

/// What one CLI invocation returned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodexProcessResult {
    /// Bytes the CLI wrote to `--output-last-message`.
    pub last_message: String,
    /// CLI stdout.
    pub stdout: String,
    /// CLI stderr.
    pub stderr: String,
    /// Process status, when the process exited.
    pub exit_code: Option<i32>,
    /// The call was cancelled.
    pub cancelled: bool,
    /// The call passed its deadline.
    pub timed_out: bool,
    /// Spawn or process failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Isolation check failure. Authored text is not admitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub violation: Option<String>,
}

impl CodexProcessResult {
    /// A clean reply whose last message is `{"text": text}`.
    pub fn authored(text: &str) -> Self {
        Self {
            last_message: json!({ "text": text }).to_string(),
            stdout: String::new(),
            stderr: String::new(),
            exit_code: Some(0),
            cancelled: false,
            timed_out: false,
            error: None,
            violation: None,
        }
    }
}

/// Runs one Codex request. Record and tests supply their own executor.
pub trait CodexExecutor: Send {
    /// Runs `request` once.
    fn run(&mut self, request: &CodexRequest) -> Result<CodexProcessResult, String>;
    /// Cancellable invocation. Controlled legacy executors may only check
    /// before dispatch; the live process overrides this to stop its group.
    fn run_cancellable(
        &mut self,
        request: &CodexRequest,
        cancel: &harness::execution::Cancellation,
    ) -> Result<CodexProcessResult, String> {
        cancel.check()?;
        self.run(request)
    }
}

/// Live, record, or replay execution for one physician chat.
pub struct CodexTransport {
    next_id: u64,
    kind: Kind,
}

enum Kind {
    Run {
        executor: Box<dyn CodexExecutor>,
        recording: Option<Recording>,
    },
    Replay(CassetteReplayer),
}

struct Recording {
    recorder: CassetteRecorder,
    output: PathBuf,
}

impl CodexTransport {
    fn live() -> Self {
        Self {
            next_id: 0,
            kind: Kind::Run {
                executor: Box::new(CodexProcess::new()),
                recording: None,
            },
        }
    }

    /// Records every `executor` result into `output`.
    ///
    /// An empty authorization is refused. The live CLI is not started unless
    /// `executor` starts it.
    pub fn recording(
        executor: Box<dyn CodexExecutor>,
        authorization: impl Into<String>,
        output: PathBuf,
    ) -> Result<Self, String> {
        let authorization = authorization.into();
        if authorization.trim().is_empty() {
            return Err("Record mode requires CODA_CASSETTE_AUTHORIZATION".to_owned());
        }
        if output.as_os_str().is_empty() {
            return Err("Record mode requires CODA_CASSETTE_OUTPUT".to_owned());
        }
        if output.is_dir() {
            return Err("CODA_CASSETTE_OUTPUT must be a file path".to_owned());
        }
        Ok(Self {
            next_id: 0,
            kind: Kind::Run {
                executor,
                recording: Some(Recording {
                    recorder: CassetteRecorder::new(authorization),
                    output,
                }),
            },
        })
    }

    fn replay_path(path: &Path) -> Result<Self, String> {
        let cassette = Cassette::load(path)?;
        Ok(Self::replay(cassette))
    }

    fn replay(cassette: Cassette) -> Self {
        Self {
            next_id: 0,
            kind: Kind::Replay(CassetteReplayer::new(cassette)),
        }
    }

    /// Which execution mode this transport implements.
    pub fn mode(&self) -> crate::ModelExecution {
        match &self.kind {
            Kind::Run {
                recording: None, ..
            } => crate::ModelExecution::Live,
            Kind::Run {
                recording: Some(_), ..
            } => crate::ModelExecution::Record,
            Kind::Replay(_) => crate::ModelExecution::Replay,
        }
    }

    /// Exchanges still waiting in a replay cassette.
    pub fn unconsumed(&self) -> Option<usize> {
        match &self.kind {
            Kind::Replay(replayer) => Some(replayer.remaining()),
            Kind::Run { .. } => None,
        }
    }

    /// Answers one Responses-shaped physician turn.
    pub fn complete(&mut self, responses_body: &Value) -> Result<String, String> {
        self.complete_cancellable(responses_body, &harness::execution::Cancellation::default())
    }

    /// Stops a live process on cancellation while retaining its cassette result.
    pub fn complete_cancellable(
        &mut self,
        responses_body: &Value,
        cancel: &harness::execution::Cancellation,
    ) -> Result<String, String> {
        cancel.check()?;
        let request = CodexRequest::from_responses(responses_body)?;
        self.next_id += 1;
        let provider_call_id = format!("physician-chat-{}", self.next_id);
        let span = tracing::info_span!(
            "codex.exec",
            provider_call_id = %provider_call_id,
            coda.run_id = %provider_call_id,
            execution = ?self.mode(),
            duration_ms = tracing::field::Empty,
            exit_code = tracing::field::Empty,
            timed_out = tracing::field::Empty,
            otel.status_code = tracing::field::Empty,
        );
        let _entered = span.enter();
        let outcome = match &mut self.kind {
            Kind::Replay(replayer) => replayer.complete(&provider_call_id, &request),
            Kind::Run {
                executor,
                recording,
            } => {
                let started = Instant::now();
                let result = executor.run_cancellable(&request, cancel)?;
                let duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
                span.record("duration_ms", duration_ms);
                span.record("timed_out", result.timed_out);
                if let Some(code) = result.exit_code {
                    span.record("exit_code", code);
                }
                if let Some(recording) = recording {
                    recording
                        .recorder
                        .record(&provider_call_id, &request, &result, duration_ms)?;
                    recording.recorder.write(&recording.output)?;
                }
                admit(&result)
            }
        };
        if outcome.is_err() {
            span.record("otel.status_code", "ERROR");
        }
        outcome
    }
}

fn admit(result: &CodexProcessResult) -> Result<String, String> {
    if let Some(violation) = &result.violation {
        return Err(format!(
            "Codex exec was not one isolated model request ({violation}); no authored output was admitted"
        ));
    }
    if result.cancelled {
        return Err("Codex exec call was cancelled".to_owned());
    }
    if result.timed_out {
        return Err("Codex exec call exceeded its deadline".to_owned());
    }
    if let Some(error) = &result.error {
        return Err(error.clone());
    }
    if result.exit_code != Some(0) {
        let code = result
            .exit_code
            .map(|code| code.to_string())
            .unwrap_or_else(|| "without a status".to_owned());
        let detail: String = result.stderr.trim().chars().take(400).collect();
        if detail.is_empty() {
            return Err(format!("Codex exec exited {code}"));
        }
        return Err(format!("Codex exec exited {code}: {detail}"));
    }
    authored_codex_text(&result.last_message, &result.stdout)
        .ok_or_else(|| "Codex exec returned no final authored output".to_owned())
}

/// The physician-visible text from a CLI last message or stdout event.
pub fn authored_codex_text(last_message: &str, stdout: &str) -> Option<String> {
    if let Some(text) = text_field(last_message) {
        return Some(text);
    }
    for line in stdout.lines() {
        let Ok(event) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if event.get("type").and_then(Value::as_str) != Some("item.completed") {
            continue;
        }
        let Some(item) = event.get("item") else {
            continue;
        };
        if item.get("type").and_then(Value::as_str) != Some("agent_message") {
            continue;
        }
        let Some(text) = item.get("text").and_then(Value::as_str) else {
            continue;
        };
        if let Some(authored) = text_field(text) {
            return Some(authored);
        }
    }
    None
}

fn text_field(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(value) = serde_json::from_str::<Value>(trimmed) {
        let text = value.get("text").and_then(Value::as_str)?;
        if text.is_empty() {
            return None;
        }
        return Some(text.to_owned());
    }
    Some(trimmed.to_owned())
}

/// A physician-chat transport cassette.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cassette {
    /// Schema version. This adapter writes version 1.
    pub schema_version: u32,
    /// Identifier for this capture.
    pub cassette_id: String,
    /// UTC time the recorder was created.
    pub recorded_at: String,
    /// Who authorized the synthetic capture.
    pub provenance: Provenance,
    /// Exchanges in record order.
    pub exchanges: Vec<Exchange>,
}

/// Provenance required on every cassette.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    /// Always `codex-exec`.
    pub adapter_id: String,
    /// The `CODA_CASSETTE_AUTHORIZATION` value.
    pub authorization_reference: String,
    /// This capture is synthetic and not PHI.
    pub synthetic_non_phi: bool,
}

/// One captured CLI exchange.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exchange {
    /// 1-based position in the cassette.
    pub sequence: u64,
    /// Logical request key.
    pub request_key: String,
    /// Identity of the call that produced this exchange.
    pub provider_call_id: String,
    /// Wall time of the executor call, in milliseconds.
    pub duration_ms: u64,
    /// Process outcome.
    pub result: CodexProcessResult,
}

impl Cassette {
    fn load(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|error| {
            format!("Could not read Codex cassette {}: {error}", path.display())
        })?;
        let cassette: Self = serde_json::from_str(&text).map_err(|error| {
            format!("Codex cassette {} is not schema 1: {error}", path.display())
        })?;
        validate_cassette(&cassette)?;
        Ok(cassette)
    }
}

fn validate_cassette(cassette: &Cassette) -> Result<(), String> {
    if cassette.schema_version != CASSETTE_SCHEMA_VERSION {
        return Err(format!(
            "Codex cassette schema is {}, expected {CASSETTE_SCHEMA_VERSION}",
            cassette.schema_version
        ));
    }
    if cassette.cassette_id.is_empty() {
        return Err("Codex cassette is missing an id".to_owned());
    }
    if cassette.provenance.adapter_id != ADAPTER_ID {
        return Err("Codex cassette adapter must be codex-exec".to_owned());
    }
    if cassette
        .provenance
        .authorization_reference
        .trim()
        .is_empty()
    {
        return Err("Codex cassette is missing an authorization reference".to_owned());
    }
    if !cassette.provenance.synthetic_non_phi {
        return Err("Codex cassette must be marked synthetic_non_phi".to_owned());
    }
    let mut seen_calls = HashSet::new();
    for (index, exchange) in cassette.exchanges.iter().enumerate() {
        let expected = (index as u64) + 1;
        if exchange.sequence != expected {
            return Err("Codex exec cassette sequence must be contiguous and ordered".to_owned());
        }
        if exchange.request_key.len() != 64
            || !exchange
                .request_key
                .chars()
                .all(|char| char.is_ascii_hexdigit())
        {
            return Err("Codex cassette request key must be 64 hex characters".to_owned());
        }
        if exchange.provider_call_id.is_empty()
            || !seen_calls.insert(exchange.provider_call_id.clone())
        {
            return Err("Cassette repeats a provider call identity".to_owned());
        }
    }
    Ok(())
}

struct CassetteRecorder {
    cassette: Cassette,
}

impl CassetteRecorder {
    fn new(authorization: String) -> Self {
        Self {
            cassette: Cassette {
                schema_version: CASSETTE_SCHEMA_VERSION,
                cassette_id: random_id(),
                recorded_at: format_rfc3339(SystemTime::now()),
                provenance: Provenance {
                    adapter_id: ADAPTER_ID.to_owned(),
                    authorization_reference: authorization,
                    synthetic_non_phi: true,
                },
                exchanges: Vec::new(),
            },
        }
    }

    fn record(
        &mut self,
        provider_call_id: &str,
        request: &CodexRequest,
        result: &CodexProcessResult,
        duration_ms: u64,
    ) -> Result<(), String> {
        let sequence = (self.cassette.exchanges.len() as u64) + 1;
        self.cassette.exchanges.push(Exchange {
            sequence,
            request_key: request.request_key(),
            provider_call_id: provider_call_id.to_owned(),
            duration_ms,
            result: result.clone(),
        });
        validate_cassette(&self.cassette)
    }

    fn write(&self, path: &Path) -> Result<(), String> {
        validate_cassette(&self.cassette)?;
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "Could not create cassette directory {}: {error}",
                    parent.display()
                )
            })?;
        }
        let body = serde_json::to_string_pretty(&self.cassette)
            .map_err(|error| format!("Could not encode Codex cassette: {error}"))?;
        harness::transport::write_atomic(path, format!("{body}\n").as_bytes())
    }
}

struct CassetteReplayer {
    queue: harness::transport::ReplayQueue<Exchange>,
    seen_calls: HashSet<String>,
}

impl CassetteReplayer {
    fn new(cassette: Cassette) -> Self {
        Self {
            queue: harness::transport::ReplayQueue::new(
                cassette
                    .exchanges
                    .into_iter()
                    .map(|exchange| (exchange.request_key.clone(), exchange)),
            ),
            seen_calls: HashSet::new(),
        }
    }

    fn remaining(&self) -> usize {
        self.queue.remaining()
    }

    fn complete(
        &mut self,
        provider_call_id: &str,
        request: &CodexRequest,
    ) -> Result<String, String> {
        if !self.seen_calls.insert(provider_call_id.to_owned()) {
            return Err("Replay repeats a current provider call identity".to_owned());
        }
        let key = request.request_key();
        let exchange = self.queue.take(provider_call_id, &key).map_err(|_| {
            "Codex exec cassette has no exchange for this logical request".to_owned()
        })?;
        admit(&exchange.result)
    }
}

struct CodexProcess {
    executable: String,
    timeout: Duration,
    auth_file: PathBuf,
}

impl CodexProcess {
    fn new() -> Self {
        Self {
            executable: "codex".to_owned(),
            timeout: Duration::from_secs(10 * 60),
            auth_file: default_auth_file(),
        }
    }

    #[cfg(test)]
    fn with_auth_file(auth_file: PathBuf) -> Self {
        Self {
            auth_file,
            ..Self::new()
        }
    }
}

impl CodexExecutor for CodexProcess {
    fn run(&mut self, request: &CodexRequest) -> Result<CodexProcessResult, String> {
        self.run_cancellable(request, &harness::execution::Cancellation::default())
    }
    fn run_cancellable(
        &mut self,
        request: &CodexRequest,
        cancel: &harness::execution::Cancellation,
    ) -> Result<CodexProcessResult, String> {
        cancel.check()?;
        ensure_catalog(request)?;
        if !self.auth_file.is_file() {
            return Err(
                "Codex CLI subscription is not signed in. Run `codex login`, then send the message again."
                    .to_owned(),
            );
        }
        let cwd = call_directory()?;
        let cleanup = RemoveDir(cwd.clone());
        let result = self.run_in(&cwd, request, cancel);
        drop(cleanup);
        Ok(result)
    }
}

impl CodexProcess {
    fn run_in(
        &self,
        cwd: &Path,
        request: &CodexRequest,
        cancel: &harness::execution::Cancellation,
    ) -> CodexProcessResult {
        let codex_home = cwd.join("codex-home");
        let instructions = cwd.join("instructions.md");
        let schema = cwd.join("agent-message.schema.json");
        let last_message = cwd.join("agent-message.json");
        let catalog = cwd.join("codex-model-catalog.json");
        if let Err(error) = prepare_call_dir(
            &codex_home,
            &instructions,
            &schema,
            &catalog,
            &self.auth_file,
            request,
        ) {
            return errored(error);
        }
        let args = codex_exec_args(
            &request.model,
            &request.effort,
            &instructions,
            &catalog,
            &schema,
            &last_message,
        );
        let mut command = Command::new(&self.executable);
        command
            .args(&args)
            .current_dir(cwd)
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (key, value) in subscription_environment(&codex_home) {
            command.env(key, value);
        }
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(error) => {
                return errored(format!(
                    "Codex CLI executable `{}` could not be started ({error}). Install the CLI and run `codex login`.",
                    self.executable
                ));
            }
        };
        let stdin = child.stdin.take();
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let prompt = request.prompt();
        let stdin_thread = thread::spawn(move || {
            if let Some(mut stdin) = stdin {
                let _ = stdin.write_all(prompt.as_bytes());
            }
        });
        let stdout_thread = thread::spawn(move || read_pipe(stdout));
        let stderr_thread = thread::spawn(move || read_pipe(stderr));
        let pid = child.id();
        let started = Instant::now();
        let mut timed_out = false;
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) if cancel.is_cancelled() || started.elapsed() >= self.timeout => {
                    timed_out = started.elapsed() >= self.timeout;
                    signal_group(pid, 15);
                    thread::sleep(Duration::from_millis(250));
                    if child.try_wait().ok().flatten().is_none() {
                        signal_group(pid, 9);
                    }
                    match child.wait() {
                        Ok(status) => break status,
                        Err(error) => {
                            return errored(format!("Codex exec wait failed: {error}"));
                        }
                    }
                }
                Ok(None) => thread::sleep(Duration::from_millis(20)),
                Err(error) => return errored(format!("Codex exec wait failed: {error}")),
            }
        };
        let _ = stdin_thread.join();
        let stdout = stdout_thread.join().unwrap_or_default();
        let stderr = stderr_thread.join().unwrap_or_default();
        let last = fs::read_to_string(&last_message).unwrap_or_default();
        let exit_code = status.code();
        let mut result = CodexProcessResult {
            last_message: last,
            stdout,
            stderr,
            exit_code,
            cancelled: cancel.is_cancelled(),
            timed_out,
            error: None,
            violation: None,
        };
        if exit_code == Some(0) && !timed_out {
            let session = check_session(&codex_home, &result.stdout, request);
            if let Some(violation) = session.violation {
                result.violation = Some(violation);
            }
        }
        result
    }
}

fn errored(message: String) -> CodexProcessResult {
    CodexProcessResult {
        last_message: String::new(),
        stdout: String::new(),
        stderr: String::new(),
        exit_code: None,
        cancelled: false,
        timed_out: false,
        error: Some(message),
        violation: None,
    }
}

fn prepare_call_dir(
    codex_home: &Path,
    instructions: &Path,
    schema: &Path,
    catalog: &Path,
    auth_file: &Path,
    request: &CodexRequest,
) -> Result<(), String> {
    fs::create_dir_all(codex_home)
        .map_err(|error| format!("Could not create Codex home: {error}"))?;
    write_private(instructions, request.instructions.as_bytes())?;
    write_private(schema, OUTPUT_SCHEMA.as_bytes())?;
    write_private(catalog, MODEL_CATALOG.as_bytes())?;
    let link = codex_home.join("auth.json");
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(auth_file, &link)
            .map_err(|error| format!("Could not link Codex subscription login: {error}"))?;
    }
    #[cfg(not(unix))]
    {
        let _ = (auth_file, link);
        return Err("Codex exec requires a Unix process".to_owned());
    }
    Ok(())
}

fn write_private(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("Could not write {}: {error}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = file.set_permissions(fs::Permissions::from_mode(0o600));
    }
    file.write_all(bytes)
        .map_err(|error| format!("Could not write {}: {error}", path.display()))?;
    Ok(())
}

struct CallArgs<'a> {
    model: &'a str,
    effort: &'a str,
    instructions: &'a Path,
    catalog: &'a Path,
    schema: &'a Path,
    last_message: &'a Path,
}

fn codex_exec_args(
    model: &str,
    effort: &str,
    instructions: &Path,
    catalog: &Path,
    schema: &Path,
    last_message: &Path,
) -> Vec<String> {
    let paths = CallArgs {
        model,
        effort,
        instructions,
        catalog,
        schema,
        last_message,
    };
    let mut args = vec![
        "exec".to_owned(),
        "-".to_owned(),
        "--model".to_owned(),
        paths.model.to_owned(),
        "--config".to_owned(),
        format!("model_reasoning_effort=\"{}\"", paths.effort),
        "--config".to_owned(),
        format!(
            "model_instructions_file={}",
            serde_json::to_string(&paths.instructions).expect("path")
        ),
        "--config".to_owned(),
        format!(
            "model_catalog_json={}",
            serde_json::to_string(&paths.catalog).expect("path")
        ),
        "--config".to_owned(),
        "web_search=\"disabled\"".to_owned(),
        "--config".to_owned(),
        "include_environment_context=false".to_owned(),
    ];
    // Codex's own OTel exporter is off by default; Coda's `codex.exec` span is the record of the call.
    for setting in ISOLATED_CONFIG {
        args.push("--config".to_owned());
        args.push((*setting).to_owned());
    }
    args.push("--sandbox".to_owned());
    args.push("read-only".to_owned());
    args.push("--strict-config".to_owned());
    for capability in DISABLED_CAPABILITIES {
        args.push("--disable".to_owned());
        args.push((*capability).to_owned());
    }
    args.extend([
        "--skip-git-repo-check".to_owned(),
        "--ignore-user-config".to_owned(),
        "--ignore-rules".to_owned(),
        "--output-schema".to_owned(),
        paths.schema.display().to_string(),
        "--output-last-message".to_owned(),
        paths.last_message.display().to_string(),
        "--json".to_owned(),
        "--color".to_owned(),
        "never".to_owned(),
    ]);
    args
}

fn subscription_environment(codex_home: &Path) -> Vec<(String, String)> {
    let mut env = Vec::new();
    for name in INHERITED_ENV {
        if let Ok(value) = std::env::var(name) {
            env.push(((*name).to_owned(), value));
        }
    }
    let home = codex_home.display().to_string();
    env.push(("CODEX_HOME".to_owned(), home.clone()));
    env.push(("HOME".to_owned(), home));
    env.push(("CODEX_EXEC_SERVER_URL".to_owned(), "none".to_owned()));
    env
}

fn ensure_catalog(request: &CodexRequest) -> Result<(), String> {
    let catalog = model_catalog();
    let Some(efforts) = catalog.get(&request.model) else {
        return Err(format!(
            "{} is not in Coda's Codex model catalog",
            request.model
        ));
    };
    if !efforts.contains(&request.effort) {
        return Err(format!(
            "{} does not support effort {}",
            request.model, request.effort
        ));
    }
    Ok(())
}

fn model_catalog() -> &'static HashMap<String, HashSet<String>> {
    static CATALOG: OnceLock<HashMap<String, HashSet<String>>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let value: Value = serde_json::from_str(MODEL_CATALOG).expect("codex model catalog");
        let mut models = HashMap::new();
        for model in value["models"].as_array().into_iter().flatten() {
            let Some(slug) = model.get("slug").and_then(Value::as_str) else {
                continue;
            };
            let efforts = model
                .get("supported_reasoning_levels")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|level| level.get("effort").and_then(Value::as_str))
                .map(str::to_owned)
                .collect();
            models.insert(slug.to_owned(), efforts);
        }
        models
    })
}

struct SessionCheck {
    violation: Option<String>,
}

fn check_session(codex_home: &Path, stdout: &str, request: &CodexRequest) -> SessionCheck {
    let Some(thread_id) = started_thread_id(stdout) else {
        return SessionCheck {
            violation: Some("Codex exec reported no thread".to_owned()),
        };
    };
    let records = session_records(&codex_home.join("sessions"));
    let expected_suffix = format!("-{thread_id}.jsonl");
    let name_matches = records.first().is_some_and(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(&expected_suffix))
    });
    if records.len() != 1 || !name_matches {
        return SessionCheck {
            violation: Some(format!(
                "expected one session record for thread {thread_id}, found {}",
                records.len()
            )),
        };
    }
    let text = match fs::read_to_string(&records[0]) {
        Ok(text) => text,
        Err(error) => {
            return SessionCheck {
                violation: Some(format!("unreadable session record: {error}")),
            };
        }
    };
    let _ = fs::remove_file(&records[0]);
    let mut violations = Vec::new();
    let mut items: HashMap<String, usize> = HashMap::new();
    let mut session_meta = false;
    let mut turns = 0;
    let mut requests = 0;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let Ok(entry) = serde_json::from_str::<Value>(line) else {
            violations.push("unreadable session record line".to_owned());
            continue;
        };
        let Some(payload) = entry.get("payload").filter(|value| value.is_object()) else {
            continue;
        };
        match entry.get("type").and_then(Value::as_str) {
            Some("session_meta") => {
                session_meta = true;
                let text = payload
                    .pointer("/base_instructions/text")
                    .and_then(Value::as_str);
                if text != Some(request.instructions.trim()) {
                    violations.push("base instructions are not Coda's".to_owned());
                }
            }
            Some("turn_context") => {
                turns += 1;
                let model = payload.get("model").and_then(Value::as_str);
                let effort = payload.get("effort").and_then(Value::as_str);
                if model != Some(request.model.as_str()) || effort != Some(request.effort.as_str())
                {
                    violations.push(format!(
                        "turn ran {model:?}/{effort:?}, bound {}/{}",
                        request.model, request.effort
                    ));
                }
            }
            Some("token_usage_record") => requests += 1,
            Some("compacted") => violations.push("context compaction".to_owned()),
            Some("event_msg")
                if payload.get("type").and_then(Value::as_str) == Some("context_compacted") =>
            {
                violations.push("context compaction".to_owned());
            }
            Some("response_item") => {
                let kind = if payload.get("type").and_then(Value::as_str) == Some("message") {
                    format!(
                        "message:{}",
                        payload.get("role").and_then(Value::as_str).unwrap_or("")
                    )
                } else {
                    payload
                        .get("type")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_owned()
                };
                *items.entry(kind.clone()).or_insert(0) += 1;
                if kind != "message:user" && kind != "message:assistant" && kind != "reasoning" {
                    violations.push(format!("native item {kind}"));
                }
            }
            _ => {}
        }
    }
    if !session_meta {
        violations.push("no session metadata".to_owned());
    }
    if turns == 0 {
        violations.push("no turn context".to_owned());
    }
    if requests != 1 {
        violations.push(format!("{requests} model requests"));
    }
    if items.get("message:user").copied().unwrap_or(0) != 1 {
        violations.push(format!(
            "{} user messages",
            items.get("message:user").copied().unwrap_or(0)
        ));
    }
    SessionCheck {
        violation: if violations.is_empty() {
            None
        } else {
            Some(violations.join("; "))
        },
    }
}

fn started_thread_id(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        let Ok(event) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if event.get("type").and_then(Value::as_str) != Some("thread.started") {
            continue;
        }
        if let Some(thread_id) = event.get("thread_id").and_then(Value::as_str)
            && !thread_id.is_empty()
            && thread_id
                .chars()
                .all(|char| char.is_ascii_alphanumeric() || char == '-')
        {
            return Some(thread_id.to_owned());
        }
    }
    None
}

fn session_records(directory: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![directory.to_path_buf()];
    while let Some(dir) = pending.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("jsonl") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

fn default_auth_file() -> PathBuf {
    if let Some(home) = std::env::var_os("CODEX_HOME") {
        return PathBuf::from(home).join("auth.json");
    }
    let mut home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.push(".codex");
    home.push("auth.json");
    home
}

fn call_directory() -> Result<PathBuf, String> {
    let dir = std::env::temp_dir().join(format!("coda-codex-exec-{}", random_id()));
    fs::create_dir_all(&dir)
        .map_err(|error| format!("Could not create Codex call directory: {error}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&dir, fs::Permissions::from_mode(0o700));
    }
    Ok(dir)
}

struct RemoveDir(PathBuf);

impl Drop for RemoveDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn read_pipe(pipe: Option<impl Read>) -> String {
    let Some(mut pipe) = pipe else {
        return String::new();
    };
    let mut bytes = Vec::new();
    let _ = pipe.read_to_end(&mut bytes);
    String::from_utf8_lossy(&bytes).into_owned()
}

fn signal_group(pid: u32, signal: i32) {
    #[cfg(unix)]
    unsafe {
        libc_kill(-(pid as i32), signal);
    }
    #[cfg(not(unix))]
    {
        let _ = (pid, signal);
    }
}

unsafe extern "C" {
    #[cfg(unix)]
    fn kill(pid: i32, sig: i32) -> i32;
}

#[cfg(unix)]
unsafe fn libc_kill(pid: i32, sig: i32) -> i32 {
    unsafe { kill(pid, sig) }
}

fn random_id() -> String {
    let mut bytes = [0_u8; 16];
    if let Ok(mut file) = File::open("/dev/urandom")
        && file.read_exact(&mut bytes).is_ok()
    {
        return sha256_hex(&bytes)[..32].to_owned();
    }
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{nanos:x}{:x}", std::process::id())
}

/// UTC RFC 3339 with milliseconds, e.g. `1970-01-01T00:00:00.000Z`.
pub fn format_rfc3339(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let days = secs / 86_400;
    let tod = secs % 86_400;
    let (year, month, day) = civil_from_unix_days(days);
    let hour = tod / 3_600;
    let minute = (tod % 3_600) / 60;
    let second = tod % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z")
}

fn civil_from_unix_days(days: u64) -> (i32, u32, u32) {
    let z = days as i64 + 719_468;
    let era = z / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    (year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChatSession, ModelExecution, Submit, model_roles};

    #[test]
    fn live_record_and_replay_do_not_read_an_api_key() {
        let live = match open_codex(ModelExecution::Live, CodexSettings::default()) {
            Ok(live) => live,
            Err(error) => panic!("live does not need an API key: {error}"),
        };
        assert_eq!(live.mode(), ModelExecution::Live);
        let error = expect_error(open_codex(ModelExecution::Record, CodexSettings::default()));
        assert!(error.contains("CODA_CASSETTE_AUTHORIZATION"), "{error}");
        let error = expect_error(open_codex(
            ModelExecution::Record,
            CodexSettings {
                cassette_authorization: Some("Dev approval"),
                cassette_output: None,
                cassette_path: None,
            },
        ));
        assert!(error.contains("CODA_CASSETTE_OUTPUT"), "{error}");
        let error = expect_error(open_codex(ModelExecution::Replay, CodexSettings::default()));
        assert!(error.contains("CODA_CASSETTE_PATH"), "{error}");
        let error = expect_error(open_codex(ModelExecution::Api, CodexSettings::default()));
        assert!(error.contains("does not use the Codex CLI"), "{error}");
        assert!(!error.contains("OPENAI_API_KEY"), "{error}");
    }

    #[test]
    fn record_and_replay_round_trip_without_the_cli() {
        let dir = scratch_dir();
        let path = dir.join("cassette.json");
        let mut recording = CodexTransport::recording(
            Box::new(Scripted::replies(["Noted.", "Again."])),
            "synthetic test",
            path.clone(),
        )
        .unwrap();
        assert_eq!(recording.mode(), ModelExecution::Record);
        assert_eq!(recording.complete(&responses("Hello")).unwrap(), "Noted.");
        assert_eq!(recording.complete(&responses("Hello")).unwrap(), "Again.");
        let mut replay = CodexTransport::replay_path(&path).unwrap();
        assert_eq!(replay.mode(), ModelExecution::Replay);
        assert_eq!(replay.complete(&responses("Hello")).unwrap(), "Noted.");
        assert_eq!(replay.complete(&responses("Hello")).unwrap(), "Again.");
        assert_eq!(replay.unconsumed(), Some(0));
        let error = replay.complete(&responses("Hello")).unwrap_err();
        assert!(
            error.contains("no exchange for this logical request"),
            "{error}"
        );

        let mut replay = CodexTransport::replay_path(&path).unwrap();
        let error = replay
            .complete(&responses("A different question"))
            .unwrap_err();
        assert!(
            error.contains("no exchange for this logical request"),
            "{error}"
        );
        assert_eq!(replay.unconsumed(), Some(2));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_missing_subscription_does_not_spawn_the_cli() {
        let mut process =
            CodexProcess::with_auth_file(PathBuf::from("/nonexistent/coda-codex-auth.json"));
        let request = CodexRequest::from_responses(&responses("Hello")).unwrap();
        let error = process.run(&request).unwrap_err();
        assert!(error.contains("codex login"), "{error}");
    }

    #[test]
    fn an_unknown_model_does_not_spawn_the_cli() {
        let mut process =
            CodexProcess::with_auth_file(PathBuf::from("/nonexistent/coda-codex-auth.json"));
        let request = CodexRequest {
            model: "not-a-model".to_owned(),
            effort: "high".to_owned(),
            instructions: PHYSICIAN_CHAT_INSTRUCTIONS.to_owned(),
            turns: vec![CodexTurn {
                role: "user".to_owned(),
                text: "Hello".to_owned(),
            }],
        };
        let error = process.run(&request).unwrap_err();
        assert!(
            error.contains("not in Coda's Codex model catalog"),
            "{error}"
        );
        assert!(!error.contains("codex login"), "{error}");
    }

    #[test]
    fn the_cli_invocation_keeps_the_subscription_and_drops_the_api_key() {
        let args = codex_exec_args(
            "gpt-6-sol",
            "high",
            Path::new("/tmp/instructions.md"),
            Path::new("/tmp/catalog.json"),
            Path::new("/tmp/schema.json"),
            Path::new("/tmp/last.json"),
        );
        assert_eq!(args[0], "exec");
        assert!(args.iter().any(|arg| arg == "--ignore-user-config"));
        assert!(args.iter().any(|arg| arg == "--sandbox"));
        assert!(args.iter().any(|arg| arg == "read-only"));
        assert!(args.iter().any(|arg| arg == "gpt-6-sol"));
        assert!(
            args.iter()
                .any(|arg| arg.contains("model_reasoning_effort=\"high\""))
        );
        let joined = args.join(" ");
        assert!(!joined.contains("OPENAI_API_KEY"));
        let env = subscription_environment(Path::new("/tmp/codex-home"));
        assert!(
            env.iter()
                .any(|(key, value)| key == "CODEX_HOME" && value == "/tmp/codex-home")
        );
        assert!(
            env.iter()
                .any(|(key, value)| key == "CODEX_EXEC_SERVER_URL" && value == "none")
        );
        assert!(env.iter().all(|(key, _)| key != "OPENAI_API_KEY"));
    }

    #[test]
    fn session_check_admits_one_isolated_turn_and_rejects_a_native_tool() {
        let dir = scratch_dir();
        let request = CodexRequest::from_responses(&responses("Hello")).unwrap();
        let stdout = "{\"type\":\"thread.started\",\"thread_id\":\"thread-1\"}\n";
        let record = dir.join("sessions").join("rollout-thread-1.jsonl");
        fs::create_dir_all(record.parent().unwrap()).unwrap();
        fs::write(&record, clean_session(&request.instructions)).unwrap();
        assert!(check_session(&dir, stdout, &request).violation.is_none());

        fs::write(&record, native_tool_session(&request.instructions)).unwrap();
        let violation = check_session(&dir, stdout, &request).violation.unwrap();
        assert!(
            violation.contains("native item function_call"),
            "{violation}"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn authored_text_reads_the_json_text_field() {
        assert_eq!(
            authored_codex_text(r#"{"text":"Noted."}"#, ""),
            Some("Noted.".to_owned())
        );
        let stdout = concat!(
            r#"{"type":"item.completed","item":{"type":"agent_message","text":"{\"text\":\"From stdout.\"}"}}"#,
            "\n"
        );
        assert_eq!(
            authored_codex_text("", stdout),
            Some("From stdout.".to_owned())
        );
    }

    #[test]
    fn rfc3339_formats_the_unix_epoch() {
        assert_eq!(format_rfc3339(UNIX_EPOCH), "1970-01-01T00:00:00.000Z");
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::from_secs(86_400)),
            "1970-01-02T00:00:00.000Z"
        );
    }

    fn expect_error(result: Result<CodexTransport, String>) -> String {
        match result {
            Ok(_) => panic!("expected Codex setup to fail"),
            Err(error) => error,
        }
    }

    struct Scripted {
        replies: VecDeque<String>,
    }

    impl Scripted {
        fn replies(replies: impl IntoIterator<Item = &'static str>) -> Self {
            Self {
                replies: replies.into_iter().map(str::to_owned).collect(),
            }
        }
    }

    impl CodexExecutor for Scripted {
        fn run(&mut self, _request: &CodexRequest) -> Result<CodexProcessResult, String> {
            let text = self.replies.pop_front().expect("scripted reply");
            Ok(CodexProcessResult::authored(&text))
        }
    }

    fn responses(text: &str) -> Value {
        let roles = model_roles("codex-exec", &[]).unwrap();
        let mut session = ChatSession::new(roles.chat);
        match session.submit("k", "pw", "chat", text) {
            Submit::Accepted { request } => request,
            other => panic!("expected an accepted request, got {other:?}"),
        }
    }

    fn scratch_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("coda-codex-test-{}", random_id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn clean_session(instructions: &str) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n",
            json!({"type":"session_meta","payload":{"base_instructions":{"text":instructions}}}),
            json!({"type":"turn_context","payload":{"model":"gpt-6-sol","effort":"high"}}),
            json!({"type":"token_usage_record","payload":{"usage":{"input_tokens":1}}}),
            json!({"type":"response_item","payload":{"type":"message","role":"user"}}),
            json!({"type":"response_item","payload":{"type":"message","role":"assistant"}})
        )
    }

    fn native_tool_session(instructions: &str) -> String {
        format!(
            "{}\n{}\n",
            clean_session(instructions).trim_end(),
            json!({"type":"response_item","payload":{"type":"function_call"}})
        )
    }
}
