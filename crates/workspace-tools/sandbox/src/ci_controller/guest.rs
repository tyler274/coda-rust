//! Booting one guest and reading the job's status from its serial console.

use std::fs::File;
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use rustix::process::{Pid, Signal, kill_process_group};

use super::error::Error;
use super::termination::Termination;

/// How long a guest may take, and how often the controller checks on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timers {
    /// The longest a guest may run.
    pub deadline: Duration,
    /// How often the deadline and the GitHub job are checked.
    pub check_every: Duration,
    /// A JIT runner that has not picked up a job by then never will.
    pub idle_grace: Duration,
    /// How long the guest gets to stop after SIGTERM before SIGKILL.
    pub stop_grace: Duration,
}

impl Timers {
    pub const PRODUCTION: Self = Self {
        deadline: Duration::from_secs(3 * 3600),
        check_every: Duration::from_secs(30),
        idle_grace: Duration::from_secs(600),
        stop_grace: Duration::from_secs(20),
    };
}

/// How often the controller looks for the guest's exit or SIGTERM.
const POLL_EVERY: Duration = Duration::from_millis(200);

/// The guest ended without reporting the job's exit status.
#[derive(Debug, thiserror::Error)]
pub enum GuestFailure {
    #[error("guest exceeded its three-hour limit")]
    DeadlineExceeded,
    #[error("the GitHub job ended without this runner")]
    RunnerAbandoned,
    #[error("guest exited ({0}) without reporting a status")]
    NoStatus(ExitStatus),
    #[error("seeding exited {status}; see {log}")]
    SeedFailed { status: u32, log: String },
}

/// Whether a running guest still has work to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Watch {
    KeepGoing,
    Abandoned,
}

/// Decides each check whether the guest keeps running.
pub type Watcher<'a> = &'a mut dyn FnMut() -> Result<Watch, Error>;

/// The job's status: the last `CODA_JOB_EXIT=<digits>` in the console log.
/// A last marker whose digits do not fit a status reports none, rather than
/// falling back to an earlier marker.
pub fn exit_marker(log: &[u8]) -> Option<u32> {
    const MARKER: &[u8] = b"CODA_JOB_EXIT=";
    let mut last = None;
    let mut rest = log;
    while let Some(at) = rest
        .windows(MARKER.len())
        .position(|window| window == MARKER)
    {
        rest = &rest[at + MARKER.len()..];
        let digits = rest.iter().take_while(|b| b.is_ascii_digit()).count();
        if digits > 0 {
            last = Some(&rest[..digits]);
        }
    }
    std::str::from_utf8(last?).ok()?.parse().ok()
}

/// Boot the guest in `job_dir` and return the job's exit status.
///
/// The guest runs in its own process group, which is always signalled when
/// this returns, so nothing it started outlives the job.
pub fn run_guest(
    runner: &Path,
    job_dir: &Path,
    log_path: &Path,
    termination: &Termination,
    timers: &Timers,
    mut watch: Option<Watcher<'_>>,
) -> Result<u32, Error> {
    let log = File::create(log_path).map_err(Error::io("create", log_path))?;
    let stderr = log.try_clone().map_err(Error::io("duplicate", log_path))?;
    let mut command = Command::new(runner);
    command
        .current_dir(job_dir)
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .env("HOME", job_dir)
        .env("TMPDIR", job_dir)
        .stdin(Stdio::null())
        .stdout(log)
        .stderr(stderr);
    std::os::unix::process::CommandExt::process_group(&mut command, 0);
    let mut child = command.spawn().map_err(|source| Error::Spawn {
        program: runner.display().to_string(),
        source,
    })?;
    let waited = wait(&mut child, termination, timers, &mut watch);
    let stopped = stop(&mut child, timers.stop_grace);
    let status = Error::with_cleanup(waited, stopped)?;
    let console = std::fs::read(log_path).map_err(Error::io("read", log_path))?;
    exit_marker(&console).ok_or_else(|| GuestFailure::NoStatus(status).into())
}

fn wait(
    child: &mut Child,
    termination: &Termination,
    timers: &Timers,
    watch: &mut Option<Watcher<'_>>,
) -> Result<ExitStatus, Error> {
    let started = Instant::now();
    let mut next_check = started + timers.check_every;
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(Error::io("wait for", "the guest"))?
        {
            return Ok(status);
        }
        if termination.requested() {
            return Err(Error::Terminated);
        }
        if Instant::now() >= next_check {
            next_check += timers.check_every;
            if started.elapsed() >= timers.deadline {
                return Err(GuestFailure::DeadlineExceeded.into());
            }
            if let Some(watch) = watch.as_mut()
                && watch()? == Watch::Abandoned
            {
                return Err(GuestFailure::RunnerAbandoned.into());
            }
        }
        std::thread::sleep(POLL_EVERY);
    }
}

/// SIGTERM the guest's group, give the leader `grace` to exit, then SIGKILL
/// whatever is left of the group, even after the leader exited.
fn stop(child: &mut Child, grace: Duration) -> Result<(), Error> {
    let Some(group) = Pid::from_raw(child.id().cast_signed()) else {
        return Ok(());
    };
    if signal_group(group, Signal::TERM)? == Group::Signalled {
        let until = Instant::now() + grace;
        while Instant::now() < until
            && child
                .try_wait()
                .map_err(Error::io("wait for", "the guest"))?
                .is_none()
        {
            std::thread::sleep(POLL_EVERY);
        }
        signal_group(group, Signal::KILL)?;
    }
    child.wait().map_err(Error::io("wait for", "the guest"))?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Group {
    Signalled,
    Gone,
}

fn signal_group(group: Pid, signal: Signal) -> Result<Group, Error> {
    match kill_process_group(group, signal) {
        Ok(()) => Ok(Group::Signalled),
        Err(rustix::io::Errno::SRCH) => Ok(Group::Gone),
        Err(errno) => Err(Error::io("signal", "the guest's process group")(
            errno.into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_last_marker_wins_and_noise_is_ignored() {
        assert_eq!(
            exit_marker(b"boot\nCODA_JOB_EXIT=1\nCODA_JOB_EXIT=3\n"),
            Some(3)
        );
        assert_eq!(exit_marker(b"\x1b[0mCODA_JOB_EXIT=0\r\n"), Some(0));
        assert_eq!(exit_marker(b"CODA_JOB_EXIT=\nbooting"), None);
        assert_eq!(exit_marker(b""), None);
        assert_eq!(
            exit_marker(b"CODA_JOB_EXIT=0\nCODA_JOB_EXIT=99999999999\n"),
            None
        );
        assert_eq!(exit_marker(b"CODA_JOB_EXIT=2\nCODA_JOB_EXIT=\n"), Some(2));
    }
}
