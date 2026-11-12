//! What `doctor` and `status` read from the machine, behind one trait so
//! their verdicts can be tested against a fake.

mod systemd;

use std::ffi::OsStr;
use std::io::{self, Read as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::{Duration, Instant};

use serde::Serialize;

pub use systemd::{ActiveState, LoadState, UnitScope, UnitState};

use crate::broker::error::BrokerError;
use crate::broker::wire::Report;
use crate::config::UnitName;
use crate::error::Error;

pub const ZFS_BCLONE_PARAMETER: &str = "/sys/module/zfs/parameters/zfs_bclone_enabled";

/// How long `ssh` gets to connect and run `true` before it is killed.
pub const SSH_DEADLINE: Duration = Duration::from_secs(10);

pub trait Probe {
    fn unit(&self, scope: UnitScope, unit: &UnitName) -> Result<UnitState, Error>;
    /// `Missing` when the path does not exist; an error when it cannot be told,
    /// such as a parent directory the user may not search.
    fn path_kind(&self, path: &Path) -> Result<PathKind, Error>;
    fn fs_type(&self, path: &Path) -> Result<FsType, Error>;
    /// The filesystem source mounted at or above `path`, such as
    /// `agents/coda/build` for a ZFS dataset.
    fn mount_source(&self, path: &Path) -> Result<String, Error>;
    /// The state of a ZFS pool's `feature@block_cloning`.
    fn pool_block_cloning(&self, pool: &str) -> Result<PoolFeature, Error>;
    /// `None` when the file does not exist.
    fn read_file(&self, path: &Path) -> Result<Option<String>, Error>;
    fn count_entries(&self, dir: &Path) -> Result<usize, Error>;
    fn ssh(&self, host: &str) -> Result<SshReach, Error>;
    fn home(&self) -> Option<PathBuf>;
    /// The token broker's report of what it can serve; never a secret.
    fn broker(&self, socket: &Path) -> Result<Report, BrokerError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PathKind {
    Missing,
    Directory,
    File,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FsType {
    Zfs,
    Btrfs,
    Xfs,
    Other,
}

/// A ZFS pool feature's state, as `zpool get` reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PoolFeature {
    Disabled,
    Enabled,
    Active,
}

impl PoolFeature {
    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "disabled" => Self::Disabled,
            "enabled" => Self::Enabled,
            "active" => Self::Active,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SshReach {
    Reachable,
    /// `ssh` exited non-zero: refused, unknown host, auth or host-key failure.
    /// `exit_code` is `None` when a signal ended it.
    Failed {
        exit_code: Option<i32>,
        stderr: String,
    },
    /// No answer within [`SSH_DEADLINE`]; `ssh` was killed.
    TimedOut,
}

/// Reads the real machine, without privileges.
pub struct SystemProbe;

fn run(
    program: &'static str,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
) -> Result<String, Error> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|source| Error::Spawn { program, source })?;
    if !output.status.success() {
        return Err(Error::CommandFailed {
            program,
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

impl Probe for SystemProbe {
    fn unit(&self, scope: UnitScope, unit: &UnitName) -> Result<UnitState, Error> {
        let mut args = scope.systemctl_flag().to_vec();
        args.extend([
            "show",
            unit.as_str(),
            "-p",
            "LoadState",
            "-p",
            "ActiveState",
        ]);
        UnitState::parse(unit, &run("systemctl", args)?)
    }

    fn path_kind(&self, path: &Path) -> Result<PathKind, Error> {
        match std::fs::metadata(path) {
            Ok(meta) if meta.is_dir() => Ok(PathKind::Directory),
            Ok(meta) if meta.is_file() => Ok(PathKind::File),
            Ok(_) => Ok(PathKind::Other),
            Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(PathKind::Missing),
            Err(source) => Err(Error::Inspect {
                path: path.to_owned(),
                source,
            }),
        }
    }

    fn fs_type(&self, path: &Path) -> Result<FsType, Error> {
        const ZFS: i128 = 0x2fc1_2fc1;
        const BTRFS: i128 = 0x9123_683e;
        const XFS: i128 = 0x5846_5342;
        let stat = rustix::fs::statfs(path).map_err(|errno| Error::Inspect {
            path: path.to_owned(),
            source: errno.into(),
        })?;
        Ok(match i128::from(stat.f_type) {
            ZFS => FsType::Zfs,
            BTRFS => FsType::Btrfs,
            XFS => FsType::Xfs,
            _ => FsType::Other,
        })
    }

    fn mount_source(&self, path: &Path) -> Result<String, Error> {
        let args = [
            OsStr::new("-n"),
            "-o".as_ref(),
            "SOURCE".as_ref(),
            "--target".as_ref(),
        ];
        let output = run("findmnt", args.into_iter().chain([path.as_os_str()]))?;
        Ok(output.trim().to_owned())
    }

    fn pool_block_cloning(&self, pool: &str) -> Result<PoolFeature, Error> {
        let output = run(
            "zpool",
            ["get", "-H", "-o", "value", "feature@block_cloning", pool],
        )?;
        PoolFeature::parse(output.trim()).ok_or_else(|| Error::UnexpectedOutput {
            what: "feature@block_cloning",
            source_name: format!("zpool get {pool}"),
            found: output,
        })
    }

    fn read_file(&self, path: &Path) -> Result<Option<String>, Error> {
        match std::fs::read_to_string(path) {
            Ok(text) => Ok(Some(text)),
            Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(source) => Err(Error::Inspect {
                path: path.to_owned(),
                source,
            }),
        }
    }

    fn count_entries(&self, dir: &Path) -> Result<usize, Error> {
        let inspect = |source| Error::Inspect {
            path: dir.to_owned(),
            source,
        };
        let mut count = 0;
        for entry in std::fs::read_dir(dir).map_err(inspect)? {
            entry.map_err(inspect)?;
            count += 1;
        }
        Ok(count)
    }

    fn ssh(&self, host: &str) -> Result<SshReach, Error> {
        ssh_true(host)
    }

    fn home(&self) -> Option<PathBuf> {
        std::env::var_os("HOME")
            .filter(|home| !home.is_empty())
            .map(PathBuf::from)
    }

    fn broker(&self, socket: &Path) -> Result<Report, BrokerError> {
        crate::broker::client::ping(socket)
    }
}

/// Runs `ssh <host> true` without forwarding anything and kills it at the
/// deadline, since `ConnectTimeout` does not bound a session that hangs.
fn ssh_true(host: &str) -> Result<SshReach, Error> {
    let options = [
        "BatchMode=yes",
        "ConnectTimeout=5",
        "ForwardAgent=no",
        "ClearAllForwardings=yes",
    ];
    let mut child = Command::new("ssh")
        .args(options.iter().flat_map(|option| ["-o", option]))
        .args([host, "true"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| Error::Spawn {
            program: "ssh",
            source,
        })?;
    let mut pipe = child.stderr.take().expect("stderr is piped");
    let (sender, stderr_read) = mpsc::channel();
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        // The receiver is gone only when ssh succeeded or timed out.
        let _ = sender.send(pipe.read_to_end(&mut bytes).map(|_| bytes));
    });
    let deadline = Instant::now() + SSH_DEADLINE;
    let wait_error = |source| Error::Wait {
        program: "ssh",
        source,
    };
    let status = loop {
        if let Some(status) = child.try_wait().map_err(wait_error)? {
            break status;
        }
        if Instant::now() >= deadline {
            // A kill error means ssh already exited; `wait` reaps it either way.
            let _ = child.kill();
            child.wait().map_err(wait_error)?;
            return Ok(SshReach::TimedOut);
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    if status.success() {
        return Ok(SshReach::Reachable);
    }
    // A process ssh forked, such as a ControlPersist master, may hold the
    // pipe open after ssh exits, so the read gets a short grace period.
    let stderr = match stderr_read.recv_timeout(Duration::from_secs(1)) {
        Ok(Ok(bytes)) => String::from_utf8_lossy(&bytes).trim().to_owned(),
        Ok(Err(error)) => format!("(stderr unreadable: {error})"),
        Err(RecvTimeoutError::Timeout) => "(stderr still open after ssh exited)".to_owned(),
        Err(RecvTimeoutError::Disconnected) => "(stderr reader stopped)".to_owned(),
    };
    Ok(SshReach::Failed {
        exit_code: status.code(),
        stderr,
    })
}
