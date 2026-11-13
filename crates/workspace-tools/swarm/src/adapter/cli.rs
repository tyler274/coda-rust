//! Agent CLIs run headless for one turn each: `claude -p` and `codex exec`.
//! Each turn reaches the board through the request's MCP command and
//! continues the member's previous CLI session. In the sandbox container
//! both CLIs authenticate through the host model proxy on their own.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex, PoisonError};

use serde_json::{Value, json};

use super::process::{self, LineSink, ProcessLimits, Tail};
use super::{
    Cancel, ForkError, TurnOutcome, TurnRequest, WorkerAdapter, ended_outcome, excerpt, quota_retry,
};
use crate::names::MemberName;

/// The name the swarm MCP server is registered under in each CLI.
pub const MCP_SERVER: &str = "coda-swarm";
const MAX_ERRORS: usize = 16 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cli {
    Claude,
    Codex,
}

impl Cli {
    pub fn program(self) -> &'static str {
        match self {
            Self::Claude => "claude",
            Self::Codex => "codex",
        }
    }
}

/// Where a member's next turn starts.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Session {
    /// Continue the member's own session.
    Resume(String),
    /// Branch a new session off another member's.
    Fork(String),
}

/// Member sessions of one CLI, shared by every model entry that runs it so
/// a member can fork across models.
#[derive(Debug, Default)]
pub struct Sessions(Mutex<BTreeMap<MemberName, Session>>);

impl Sessions {
    fn get(&self, member: &MemberName) -> Option<Session> {
        self.lock().get(member).cloned()
    }

    fn set(&self, member: MemberName, session: Session) {
        self.lock().insert(member, session);
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, BTreeMap<MemberName, Session>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[derive(Debug, Clone)]
pub struct CliAdapter {
    pub cli: Cli,
    pub program: PathBuf,
    /// The provider model name passed to the CLI.
    pub model: String,
    pub limits: ProcessLimits,
    sessions: Arc<Sessions>,
}

impl CliAdapter {
    pub fn new(cli: Cli, model: impl Into<String>) -> Self {
        Self::with_sessions(cli, model, Arc::default())
    }

    pub fn with_sessions(cli: Cli, model: impl Into<String>, sessions: Arc<Sessions>) -> Self {
        Self {
            cli,
            program: PathBuf::from(cli.program()),
            model: model.into(),
            limits: ProcessLimits::default(),
            sessions,
        }
    }

    /// The CLI arguments for one turn. The prompt goes on stdin.
    fn args(&self, request: &TurnRequest, session: Option<&Session>) -> Vec<String> {
        let (program, args) = request
            .mcp_command
            .split_first()
            .map_or(("", &[][..]), |(program, args)| (program.as_str(), args));
        let mut out: Vec<String> = Vec::new();
        let mut push = |items: &[&str]| out.extend(items.iter().map(|item| (*item).to_owned()));
        match self.cli {
            Cli::Claude => {
                let config = json!({
                    "mcpServers": {
                        MCP_SERVER: { "type": "stdio", "command": program, "args": args }
                    }
                });
                push(&["-p", "--output-format", "stream-json", "--verbose"]);
                push(&["--model", &self.model]);
                push(&["--mcp-config", &config.to_string(), "--strict-mcp-config"]);
                push(&["--permission-mode", "bypassPermissions"]);
                match session {
                    Some(Session::Resume(id)) => push(&["--resume", id]),
                    Some(Session::Fork(id)) => push(&["--resume", id, "--fork-session"]),
                    None => {}
                }
            }
            Cli::Codex => {
                push(&["exec"]);
                match session {
                    Some(Session::Resume(_)) => push(&["resume"]),
                    Some(Session::Fork(_)) => push(&["fork"]),
                    None => {}
                }
                push(&["--json", "--skip-git-repo-check", "--ignore-user-config"]);
                push(&["--dangerously-bypass-approvals-and-sandbox"]);
                push(&["-m", &self.model]);
                let command = toml_string(program);
                let command_args = toml_array(args);
                push(&["-c", &format!("mcp_servers.{MCP_SERVER}.command={command}")]);
                push(&[
                    "-c",
                    &format!("mcp_servers.{MCP_SERVER}.args={command_args}"),
                ]);
                if let Some(Session::Resume(id) | Session::Fork(id)) = session {
                    push(&[id]);
                }
                push(&["-"]);
            }
        }
        out
    }
}

/// A TOML basic string. A JSON string literal is one.
fn toml_string(text: &str) -> String {
    Value::from(text).to_string()
}

fn toml_array(items: &[String]) -> String {
    Value::from(items.to_vec()).to_string()
}

impl WorkerAdapter for CliAdapter {
    fn run_turn(&self, request: TurnRequest, cancel: &Cancel) -> TurnOutcome {
        let session = self.sessions.get(&request.member);
        let mut command = Command::new(&self.program);
        command
            .args(self.args(&request, session.as_ref()))
            .current_dir(&request.cwd)
            .envs(request.env.iter().map(|(name, value)| (name, value)));
        let ran = process::run(
            &self.limits,
            command,
            request.prompt.clone(),
            Events::new(self.cli),
            Tail::new(MAX_ERRORS),
            cancel,
        );
        let ran = match ran {
            Ok(ran) => ran,
            Err(reason) => return TurnOutcome::Failed { reason },
        };
        if let Some(id) = ran.stdout.session.clone() {
            self.sessions
                .set(request.member.clone(), Session::Resume(id));
        }
        let status = match ended_outcome(ran.ended) {
            Ok(status) => status,
            Err(outcome) => return outcome,
        };
        if status.success() && !ran.stdout.failed {
            return TurnOutcome::Completed;
        }
        let errors = format!("{}{}", ran.stdout.errors.text(), ran.stderr.text());
        if let Some(retry_after) = quota_retry(&errors) {
            return TurnOutcome::QuotaExhausted { retry_after };
        }
        let detail = if errors.trim().is_empty() {
            status.to_string()
        } else {
            excerpt(&errors)
        };
        TurnOutcome::Failed {
            reason: format!("{} turn failed: {detail}", self.cli.program()),
        }
    }

    fn fork(
        &self,
        from: &MemberName,
        to: &MemberName,
        _model: Option<&str>,
    ) -> Result<(), ForkError> {
        let Some(Session::Resume(id)) = self.sessions.get(from) else {
            return Err(ForkError::NoHistory(from.clone()));
        };
        self.sessions.set(to.clone(), Session::Fork(id));
        Ok(())
    }
}

/// Reads a CLI's JSON event stream for the session id and failures.
#[derive(Debug)]
struct Events {
    cli: Cli,
    session: Option<String>,
    failed: bool,
    errors: Tail,
}

impl Events {
    fn new(cli: Cli) -> Self {
        Self {
            cli,
            session: None,
            failed: false,
            errors: Tail::new(MAX_ERRORS),
        }
    }
}

impl LineSink for Events {
    fn line(&mut self, line: &str) {
        let Ok(event) = serde_json::from_str::<Value>(line) else {
            return;
        };
        let text = |value: &Value| value.as_str().map(str::to_owned);
        match self.cli {
            Cli::Claude => {
                if let Some(id) = event.get("session_id").and_then(text) {
                    self.session = Some(id);
                }
                if event["type"] == "result" && event["is_error"] == true {
                    self.failed = true;
                    let detail = text(&event["result"]).unwrap_or_else(|| event.to_string());
                    self.errors.push(&detail);
                }
            }
            Cli::Codex => match event["type"].as_str() {
                Some("thread.started") => self.session = text(&event["thread_id"]),
                Some("turn.failed") => {
                    self.failed = true;
                    let detail =
                        text(&event["error"]["message"]).unwrap_or_else(|| event.to_string());
                    self.errors.push(&detail);
                }
                Some("error") => {
                    let detail = text(&event["message"]).unwrap_or_else(|| event.to_string());
                    self.errors.push(&detail);
                }
                _ => {}
            },
        }
    }
}

#[cfg(test)]
pub(super) fn golden_args(
    adapter: &CliAdapter,
    request: &TurnRequest,
    session: Option<(&str, bool)>,
) -> Vec<String> {
    let session = session.map(|(id, fork)| {
        if fork {
            Session::Fork(id.to_owned())
        } else {
            Session::Resume(id.to_owned())
        }
    });
    adapter.args(request, session.as_ref())
}
