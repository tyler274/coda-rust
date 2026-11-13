//! Runs one turn's program in its own process group and streams its output
//! lines to observers. Shared by every adapter that runs a CLI per turn.

use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use nix::sys::signal::{Signal, killpg};
use nix::sys::wait::{Id, WaitPidFlag, WaitStatus, waitid};
use nix::unistd::Pid;

use super::Cancel;

/// Longest output line an observer sees; the rest of the line is dropped.
const MAX_LINE: usize = 1024 * 1024;

#[derive(Debug, Clone)]
pub struct ProcessLimits {
    pub turn_timeout: Option<Duration>,
    pub kill_grace: Duration,
    pub poll: Duration,
}

impl Default for ProcessLimits {
    fn default() -> Self {
        Self {
            turn_timeout: None,
            kill_grace: Duration::from_secs(5),
            poll: Duration::from_millis(20),
        }
    }
}

pub enum Ended {
    Exited(ExitStatus),
    Cancelled,
    TimedOut(Duration),
}

/// What a finished run produced: how it ended and the observers back.
pub struct Ran<O, E> {
    pub ended: Ended,
    pub stdout: O,
    pub stderr: E,
}

/// Sees each output line of a turn process, without its line ending.
pub trait LineSink: Send + 'static {
    fn line(&mut self, line: &str);
}

/// Keeps the first `limit` bytes of output.
#[derive(Debug)]
pub struct Tail {
    text: String,
    limit: usize,
    truncated: bool,
}

impl Tail {
    pub fn new(limit: usize) -> Self {
        Self {
            text: String::new(),
            limit,
            truncated: false,
        }
    }

    pub fn push(&mut self, line: &str) {
        let room = self.limit.saturating_sub(self.text.len());
        if line.len() > room {
            self.truncated = true;
        }
        if room == 0 {
            return;
        }
        let mut end = line.len().min(room);
        while !line.is_char_boundary(end) {
            end -= 1;
        }
        self.text.push_str(&line[..end]);
        self.text.push('\n');
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }
}

impl LineSink for Tail {
    fn line(&mut self, line: &str) {
        self.push(line);
    }
}

/// Spawns `command` in a new process group with `input` on stdin, feeds
/// each stdout and stderr line to its observer, and supervises it until it
/// exits, is cancelled, or times out. Stragglers left in the group are
/// killed once the leader exits.
pub fn run<O, E>(
    limits: &ProcessLimits,
    mut command: Command,
    input: String,
    stdout: O,
    stderr: E,
    cancel: &Cancel,
) -> Result<Ran<O, E>, String>
where
    O: LineSink,
    E: LineSink,
{
    command
        .process_group(0)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let program = command.get_program().to_string_lossy().into_owned();
    let mut child = command
        .spawn()
        .map_err(|error| format!("start {program}: {error}"))?;
    let writer = child.stdin.take().map(|mut stdin| {
        // A program that never reads its stdin gets EPIPE; that is fine.
        thread::spawn(move || drop(stdin.write_all(input.as_bytes())))
    });
    let out = child.stdout.take().map(|pipe| observe(pipe, stdout));
    let err = child.stderr.take().map(|pipe| observe(pipe, stderr));
    let ended = supervise(limits, &mut child, cancel);
    if let Some(writer) = writer {
        drop(writer.join());
    }
    let (Some(out), Some(err)) = (out, err) else {
        return Err("turn process pipes were not captured".into());
    };
    let (Ok(stdout), Ok(stderr)) = (out.join(), err.join()) else {
        return Err("turn output reader panicked".into());
    };
    Ok(Ran {
        ended: ended?,
        stdout,
        stderr,
    })
}

fn observe<R, F>(pipe: R, mut observer: F) -> thread::JoinHandle<F>
where
    R: Read + Send + 'static,
    F: LineSink,
{
    thread::spawn(move || {
        let mut reader = BufReader::new(pipe);
        let mut line = Vec::new();
        loop {
            line.clear();
            match read_line(&mut reader, &mut line) {
                Ok(0) | Err(_) => return observer,
                Ok(_) => observer.line(String::from_utf8_lossy(&line).trim_end()),
            }
        }
    })
}

/// Reads through the next newline, keeping at most [`MAX_LINE`] bytes.
fn read_line(reader: &mut impl BufRead, line: &mut Vec<u8>) -> std::io::Result<usize> {
    let mut consumed = 0;
    loop {
        let buffer = reader.fill_buf()?;
        if buffer.is_empty() {
            return Ok(consumed);
        }
        let (chunk, done) = match buffer.iter().position(|&b| b == b'\n') {
            Some(at) => (&buffer[..=at], true),
            None => (buffer, false),
        };
        let room = MAX_LINE.saturating_sub(line.len());
        line.extend_from_slice(&chunk[..chunk.len().min(room)]);
        let used = chunk.len();
        consumed += used;
        reader.consume(used);
        if done {
            return Ok(consumed);
        }
    }
}

fn supervise(limits: &ProcessLimits, child: &mut Child, cancel: &Cancel) -> Result<Ended, String> {
    let pid = child_pid(child)?;
    let started = Instant::now();
    loop {
        if leader_exited(pid)? {
            return reap(child, pid).map(Ended::Exited);
        }
        if cancel.is_cancelled() {
            terminate(limits, child, pid)?;
            return Ok(Ended::Cancelled);
        }
        if let Some(limit) = limits.turn_timeout
            && started.elapsed() >= limit
        {
            terminate(limits, child, pid)?;
            return Ok(Ended::TimedOut(limit));
        }
        thread::sleep(limits.poll);
    }
}

/// SIGTERM the group, SIGKILL it after the grace period, then reap.
pub fn terminate(
    limits: &ProcessLimits,
    child: &mut Child,
    pid: Pid,
) -> Result<ExitStatus, String> {
    signal_group(pid, Signal::SIGTERM);
    wait_then_reap(limits, child, pid)
}

/// Waits up to the grace period for the leader to exit on its own, then
/// kills the group and reaps.
pub fn wait_then_reap(
    limits: &ProcessLimits,
    child: &mut Child,
    pid: Pid,
) -> Result<ExitStatus, String> {
    let deadline = Instant::now() + limits.kill_grace;
    while Instant::now() < deadline && !leader_exited(pid)? {
        thread::sleep(limits.poll);
    }
    reap(child, pid)
}

pub fn child_pid(child: &Child) -> Result<Pid, String> {
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
