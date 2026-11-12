//! Runs a keyring helper with a deadline, feeding stdin and collecting both
//! output streams on their own threads so a full pipe cannot stall it.

use std::ffi::OsStr;
use std::io::Write as _;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use super::{DEADLINE, KeyringError};

pub struct Finished {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: String,
}

pub fn run(
    program: &'static str,
    path: &OsStr,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> Result<Finished, KeyringError> {
    let mut child = Command::new(path)
        .args(args)
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| KeyringError::Spawn { program, source })?;
    let io = |source| KeyringError::Io { program, source };
    if let Some(bytes) = stdin {
        let mut pipe = child.stdin.take().expect("stdin is piped");
        let written = pipe.write_all(bytes);
        drop(pipe);
        if let Err(source) = written {
            reap(&mut child);
            return Err(io(source));
        }
    }
    let stdout = drain(child.stdout.take().expect("stdout is piped"));
    let stderr = drain(child.stderr.take().expect("stderr is piped"));
    let deadline = Instant::now() + DEADLINE;
    let status = loop {
        if let Some(status) = child.try_wait().map_err(io)? {
            break status;
        }
        if Instant::now() >= deadline {
            reap(&mut child);
            return Err(KeyringError::TimedOut { program });
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let stdout = join(stdout).map_err(io)?;
    let stderr = join(stderr).map_err(io)?;
    Ok(Finished {
        status,
        stdout,
        stderr: String::from_utf8_lossy(&stderr).trim().to_owned(),
    })
}

/// Kills a child that is being abandoned. A kill error means it already
/// exited; `wait` reaps it either way, and a failed reap leaves a zombie
/// that matters to no one once the error is returned.
fn reap(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

fn drain(mut pipe: impl std::io::Read + Send + 'static) -> JoinHandle<std::io::Result<Vec<u8>>> {
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        pipe.read_to_end(&mut bytes).map(|_| bytes)
    })
}

fn join(handle: JoinHandle<std::io::Result<Vec<u8>>>) -> std::io::Result<Vec<u8>> {
    handle
        .join()
        .unwrap_or_else(|_| Err(std::io::Error::other("output reader panicked")))
}
