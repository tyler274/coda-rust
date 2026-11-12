//! One model turn for one member. The supervisor owns the loop around turns;
//! an adapter only runs a single turn and reports how it ended.

mod cli;
mod http;
mod mcp_client;
mod process;
mod registry;
#[cfg(test)]
mod tests;

use std::fmt;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

pub use cli::Sessions;
pub use cli::{Cli, CliAdapter};
pub use http::Histories;
pub use http::{Api, HttpAdapter};
pub use process::ProcessLimits;
pub use registry::{
    AdapterKind, ModelEntry, ModelId, Registry, RegistryAdapter, RegistryError, ResolveError,
};

use crate::names::MemberName;
use process::{Ended, Tail};

/// Everything an adapter needs to run one turn.
#[derive(Clone)]
pub struct TurnRequest {
    pub member: MemberName,
    pub model: Option<String>,
    pub prompt: String,
    pub cwd: PathBuf,
    /// The command a worker runs to reach the swarm MCP server.
    pub mcp_command: Vec<String>,
    /// Extra child environment. Values may hold keys, so `Debug` omits them.
    pub env: Vec<(String, String)>,
}

impl fmt::Debug for TurnRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TurnRequest")
            .field("member", &self.member)
            .field("model", &self.model)
            .field("prompt_bytes", &self.prompt.len())
            .field("cwd", &self.cwd)
            .field("mcp_command", &self.mcp_command)
            .field(
                "env",
                &self.env.iter().map(|(name, _)| name).collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl TurnRequest {
    /// Replaces every env value (each may be a key) in `text`.
    fn redact(&self, mut text: String) -> String {
        for (_, value) in &self.env {
            if !value.is_empty() {
                text = text.replace(value.as_str(), "<redacted>");
            }
        }
        text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnOutcome {
    Completed,
    Failed {
        reason: String,
    },
    /// The model refused for quota; try again after `retry_after`.
    QuotaExhausted {
        retry_after: Duration,
    },
    /// The turn was stopped by [`Cancel::cancel`].
    Cancelled,
}

/// Set once to stop a running turn. Shared between the supervisor and the
/// thread running the member.
#[derive(Debug, Clone, Default)]
pub struct Cancel(Arc<AtomicBool>);

impl Cancel {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ForkError {
    #[error("{adapter} cannot seed a member from another member's history")]
    Unsupported { adapter: &'static str },
    #[error("{0} has no history to fork yet")]
    NoHistory(MemberName),
    #[error(transparent)]
    Model(#[from] ResolveError),
}

/// A child env variable filled from a key source item before each turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyEnv {
    #[serde(rename = "env")]
    pub var: String,
    pub item: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelInfo {
    pub name: String,
    pub display_name: String,
    pub notes: String,
}

/// The models an adapter can run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Catalog {
    /// The adapter runs whatever model name it is given.
    Any,
    Listed(Vec<ModelInfo>),
}

impl Catalog {
    pub fn admits(&self, model: &str) -> bool {
        match self {
            Self::Any => true,
            Self::Listed(models) => models.iter().any(|m| m.name == model),
        }
    }
}

pub trait WorkerAdapter: Send + Sync {
    /// Runs one turn. Returns [`TurnOutcome::Cancelled`] promptly once
    /// `cancel` is set, after stopping everything the turn started.
    fn run_turn(&self, request: TurnRequest, cancel: &Cancel) -> TurnOutcome;

    /// Seeds `to`, which has not run a turn, with the history `from` has so
    /// far; `to`'s first turn continues it. `model` is `to`'s model.
    fn fork(
        &self,
        from: &MemberName,
        to: &MemberName,
        model: Option<&str>,
    ) -> Result<(), ForkError>;

    /// Keys a turn on `model` needs in its env, beyond the supervisor's.
    fn keys(&self, _model: Option<&str>) -> Vec<KeyEnv> {
        Vec::new()
    }

    fn models(&self) -> Catalog {
        Catalog::Any
    }
}

/// Exit status a [`CommandAdapter`] program uses to report quota exhaustion.
pub const QUOTA_EXIT: i32 = 75;
/// Stdout line prefix carrying the quota retry delay in whole seconds.
pub const RETRY_AFTER_PREFIX: &str = "retry_after_secs=";
const DEFAULT_RETRY_AFTER: Duration = Duration::from_secs(60);
const MAX_STDOUT: usize = 64 * 1024;
/// Longest error excerpt a failed outcome quotes.
const MAX_REASON: usize = 2000;

/// Runs a configured program for each turn, in its own process group.
///
/// The program gets the prompt on stdin and `CODA_SWARM_MEMBER`,
/// `CODA_SWARM_MODEL` and `CODA_SWARM_MCP_COMMAND` (a JSON array) plus the
/// request env. Exit 0 completes the turn; exit [`QUOTA_EXIT`] reports quota
/// exhaustion, with an optional `retry_after_secs=N` stdout line; anything
/// else fails it. When the program exits, stragglers left in its group are
/// killed. Cancellation and timeout send SIGTERM to the group, then SIGKILL
/// after `limits.kill_grace`.
#[derive(Debug, Clone)]
pub struct CommandAdapter {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub limits: ProcessLimits,
}

impl CommandAdapter {
    pub fn new(program: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            limits: ProcessLimits::default(),
        }
    }
}

impl WorkerAdapter for CommandAdapter {
    fn run_turn(&self, request: TurnRequest, cancel: &Cancel) -> TurnOutcome {
        let mcp = serde_json::to_string(&request.mcp_command).unwrap_or_else(|_| "[]".into());
        let mut command = Command::new(&self.program);
        command
            .args(&self.args)
            .current_dir(&request.cwd)
            .env("CODA_SWARM_MEMBER", request.member.as_str())
            .env("CODA_SWARM_MODEL", request.model.as_deref().unwrap_or(""))
            .env("CODA_SWARM_MCP_COMMAND", mcp)
            .envs(request.env.iter().map(|(name, value)| (name, value)));
        let ran = process::run(
            &self.limits,
            command,
            request.prompt,
            Tail::new(MAX_STDOUT),
            Tail::new(0),
            cancel,
        );
        let ran = match ran {
            Ok(ran) => ran,
            Err(reason) => return TurnOutcome::Failed { reason },
        };
        match ended_outcome(ran.ended) {
            Ok(status) => match status.code() {
                Some(0) => TurnOutcome::Completed,
                Some(QUOTA_EXIT) => TurnOutcome::QuotaExhausted {
                    retry_after: retry_after(ran.stdout.text()),
                },
                Some(code) => TurnOutcome::Failed {
                    reason: format!("exited with status {code}"),
                },
                None => TurnOutcome::Failed {
                    reason: format!("ended by {status}"),
                },
            },
            Err(outcome) => outcome,
        }
    }

    fn fork(&self, _: &MemberName, _: &MemberName, _: Option<&str>) -> Result<(), ForkError> {
        Err(ForkError::Unsupported {
            adapter: "the command adapter",
        })
    }
}

/// The exit status of a turn process that ran to its end, or the outcome
/// of one that was stopped.
fn ended_outcome(ended: Ended) -> Result<std::process::ExitStatus, TurnOutcome> {
    match ended {
        Ended::Exited(status) => Ok(status),
        Ended::Cancelled => Err(TurnOutcome::Cancelled),
        Ended::TimedOut(after) => Err(TurnOutcome::Failed {
            reason: format!("turn timed out after {}s", after.as_secs_f32()),
        }),
    }
}

fn retry_after(stdout: &str) -> Duration {
    stdout
        .lines()
        .rev()
        .find_map(|line| line.trim().strip_prefix(RETRY_AFTER_PREFIX))
        .and_then(|secs| secs.trim().parse().ok())
        .map_or(DEFAULT_RETRY_AFTER, Duration::from_secs)
}

/// Whether a provider error message reports rate limiting or exhausted
/// quota, and when to retry. Only error text is classified, never model
/// output, so a model talking about quotas does not pause a member.
fn quota_retry(error: &str) -> Option<Duration> {
    const SIGNS: [&str; 7] = [
        "usage limit",
        "rate limit",
        "rate_limit",
        "quota",
        "resource_exhausted",
        "too many requests",
        "429",
    ];
    let lower = error.to_ascii_lowercase();
    if !SIGNS.iter().any(|sign| lower.contains(sign)) {
        return None;
    }
    Some(reset_epoch(error).unwrap_or(DEFAULT_RETRY_AFTER))
}

/// Claude reports its usage limit as `...limit reached|<unix seconds>`.
fn reset_epoch(error: &str) -> Option<Duration> {
    let (_, tail) = error.rsplit_once('|')?;
    let digits: String = tail.chars().take_while(char::is_ascii_digit).collect();
    let reset = Duration::from_secs(digits.parse().ok()?);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    Some(reset.saturating_sub(now).max(Duration::from_secs(1)))
}

fn excerpt(text: &str) -> String {
    let text = text.trim();
    match text.char_indices().nth(MAX_REASON) {
        Some((at, _)) => format!("{}...", &text[..at]),
        None => text.to_owned(),
    }
}
