//! One model turn for one member. The supervisor owns the loop around turns;
//! an adapter only runs a single turn and reports how it ended.

use std::fmt;
use std::io::{Read, Write};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use nix::sys::signal::{Signal, killpg};
use nix::sys::wait::{Id, WaitPidFlag, WaitStatus, waitid};
use nix::unistd::Pid;

use crate::names::MemberName;

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

pub trait WorkerAdapter: Send + Sync {
    /// Runs one turn. Returns [`TurnOutcome::Cancelled`] promptly once
    /// `cancel` is set, after stopping everything the turn started.
    fn run_turn(&self, request: TurnRequest, cancel: &Cancel) -> TurnOutcome;
}

/// Exit status a [`CommandAdapter`] program uses to report quota exhaustion.
pub const QUOTA_EXIT: i32 = 75;
/// Stdout line prefix carrying the quota retry delay in whole seconds.
pub const RETRY_AFTER_PREFIX: &str = "retry_after_secs=";
const DEFAULT_RETRY_AFTER: Duration = Duration::from_secs(60);
const MAX_STDOUT: u64 = 64 * 1024;

/// Runs a configured program for each turn, in its own process group.
///
/// The program gets the prompt on stdin and `CODA_SWARM_MEMBER`,
/// `CODA_SWARM_MODEL` and `CODA_SWARM_MCP_COMMAND` (a JSON array) plus the
/// request env. Exit 0 completes the turn; exit [`QUOTA_EXIT`] reports quota
/// exhaustion, with an optional `retry_after_secs=N` stdout line; anything
/// else fails it. When the program exits, stragglers left in its group are
/// killed. Cancellation and timeout send SIGTERM to the group, then SIGKILL
/// after `kill_grace`.
#[derive(Debug, Clone)]
pub struct CommandAdapter {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub turn_timeout: Option<Duration>,
    pub kill_grace: Duration,
    pub poll: Duration,
}

impl CommandAdapter {
    pub fn new(program: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            turn_timeout: None,
            kill_grace: Duration::from_secs(5),
            poll: Duration::from_millis(20),
        }
    }
}

enum Ended {
    Exited(ExitStatus),
    Cancelled,
    TimedOut(Duration),
}

impl WorkerAdapter for CommandAdapter {
    fn run_turn(&self, request: TurnRequest, cancel: &Cancel) -> TurnOutcome {
        let mcp = serde_json::to_string(&request.mcp_command).unwrap_or_else(|_| "[]".into());
        let mut command = Command::new(&self.program);
        command
            .args(&self.args)
            .current_dir(&request.cwd)
            .process_group(0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .env("CODA_SWARM_MEMBER", request.member.as_str())
            .env("CODA_SWARM_MODEL", request.model.as_deref().unwrap_or(""))
            .env("CODA_SWARM_MCP_COMMAND", mcp)
            .envs(request.env.iter().map(|(name, value)| (name, value)));
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(error) => {
                return TurnOutcome::Failed {
                    reason: format!("start {}: {error}", self.program.display()),
                };
            }
        };
        let stdin = child.stdin.take().map(|mut stdin| {
            let prompt = request.prompt;
            // A program that never reads its stdin gets EPIPE; that is fine.
            thread::spawn(move || drop(stdin.write_all(prompt.as_bytes())))
        });
        let stdout = child.stdout.take().map(|stdout| {
            thread::spawn(move || {
                let mut text = String::new();
                drop(stdout.take(MAX_STDOUT).read_to_string(&mut text));
                text
            })
        });
        let ended = self.supervise(&mut child, cancel);
        if let Some(stdin) = stdin {
            drop(stdin.join());
        }
        let stdout = stdout
            .and_then(|reader| reader.join().ok())
            .unwrap_or_default();
        match ended {
            Ok(Ended::Cancelled) => TurnOutcome::Cancelled,
            Ok(Ended::TimedOut(after)) => TurnOutcome::Failed {
                reason: format!("turn timed out after {}s", after.as_secs_f32()),
            },
            Ok(Ended::Exited(status)) => match status.code() {
                Some(0) => TurnOutcome::Completed,
                Some(QUOTA_EXIT) => TurnOutcome::QuotaExhausted {
                    retry_after: retry_after(&stdout),
                },
                Some(code) => TurnOutcome::Failed {
                    reason: format!("exited with status {code}"),
                },
                None => TurnOutcome::Failed {
                    reason: format!("ended by {status}"),
                },
            },
            Err(reason) => TurnOutcome::Failed { reason },
        }
    }
}

impl CommandAdapter {
    fn supervise(&self, child: &mut Child, cancel: &Cancel) -> Result<Ended, String> {
        let pid = child_pid(child)?;
        let started = Instant::now();
        loop {
            if leader_exited(pid)? {
                return reap(child, pid).map(Ended::Exited);
            }
            if cancel.is_cancelled() {
                self.terminate(child, pid)?;
                return Ok(Ended::Cancelled);
            }
            if let Some(limit) = self.turn_timeout
                && started.elapsed() >= limit
            {
                self.terminate(child, pid)?;
                return Ok(Ended::TimedOut(limit));
            }
            thread::sleep(self.poll);
        }
    }

    /// SIGTERM the group, SIGKILL it after the grace period, then reap.
    fn terminate(&self, child: &mut Child, pid: Pid) -> Result<ExitStatus, String> {
        signal_group(pid, Signal::SIGTERM);
        let deadline = Instant::now() + self.kill_grace;
        while Instant::now() < deadline && !leader_exited(pid)? {
            thread::sleep(self.poll);
        }
        reap(child, pid)
    }
}

fn child_pid(child: &Child) -> Result<Pid, String> {
    i32::try_from(child.id())
        .map(Pid::from_raw)
        .map_err(|error| format!("child pid {}: {error}", child.id()))
}

/// Whether the group leader has exited, without reaping it: while it is an
/// unreaped zombie its pid, and so the group id, cannot be reused.
fn leader_exited(pid: Pid) -> Result<bool, String> {
    let flags = WaitPidFlag::WEXITED | WaitPidFlag::WNOHANG | WaitPidFlag::WNOWAIT;
    match waitid(Id::Pid(pid), flags) {
        Ok(WaitStatus::StillAlive) => Ok(false),
        Ok(_) => Ok(true),
        Err(error) => Err(format!("wait for turn process: {error}")),
    }
}

/// Kills whatever is left in the group while the leader still holds its id,
/// then reaps the leader.
fn reap(child: &mut Child, pid: Pid) -> Result<ExitStatus, String> {
    signal_group(pid, Signal::SIGKILL);
    child
        .wait()
        .map_err(|error| format!("reap turn process: {error}"))
}

fn signal_group(pid: Pid, signal: Signal) {
    // ESRCH means the group is already empty.
    let _ = killpg(pid, signal);
}

fn retry_after(stdout: &str) -> Duration {
    stdout
        .lines()
        .rev()
        .find_map(|line| line.trim().strip_prefix(RETRY_AFTER_PREFIX))
        .and_then(|secs| secs.trim().parse().ok())
        .map_or(DEFAULT_RETRY_AFTER, Duration::from_secs)
}
