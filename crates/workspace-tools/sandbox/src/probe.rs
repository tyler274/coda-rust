//! What `doctor` and `status` read from the machine, behind one trait so
//! their verdicts can be tested against a fake.

mod systemd;

use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::{Duration, Instant};

use serde::Serialize;

pub use systemd::{ActiveState, LoadState, UnitScope, UnitState};

use crate::broker::error::BrokerError;
use crate::broker::wire::Report;
use crate::config::UnitName;
use crate::error::Error;
use crate::model_proxy::ProxyCheck;

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
    /// The model proxy as the container sees it, through [`PROXY_CHECK`].
    fn proxy_check(&self, host: &str) -> Result<FromContainer<ProxyCheck>, Error>;
    /// The options `ssh` resolves for `host` from the user's SSH config.
    fn ssh_config(&self, host: &str) -> Result<SshOptions, Error>;
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

/// What a command run in the container over SSH gave back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromContainer<T> {
    Answered(T),
    /// `ssh` or the command exited non-zero; `exit_code` is `None` when a signal ended it.
    Failed {
        exit_code: Option<i32>,
        stderr: String,
    },
    /// No answer within [`SSH_DEADLINE`]; `ssh` was killed.
    TimedOut,
}

/// The command `doctor` runs in the container to check the model proxy.
pub const PROXY_CHECK: &str = "coda-sandbox proxy-check";

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

/// What `ssh -G <host>` prints: one lowercase option name and its value per
/// line, with every option resolved.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SshOptions(Vec<(String, String)>);

impl SshOptions {
    pub fn parse(text: &str) -> Self {
        Self(
            text.lines()
                .filter_map(|line| line.split_once(' '))
                .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
                .collect(),
        )
    }

    /// The resolved `ForwardAgent`. `ssh -G` always prints it, so a missing
    /// or empty value means the output is not what `ssh -G` prints.
    pub fn forward_agent(&self) -> Result<ForwardAgent, Error> {
        let value = self
            .0
            .iter()
            .find(|(name, _)| name == "forwardagent")
            .map(|(_, value)| value.as_str());
        match value {
            Some("no") => Ok(ForwardAgent::No),
            Some("yes") => Ok(ForwardAgent::Yes),
            Some(socket) if !socket.is_empty() => Ok(ForwardAgent::Socket(socket.to_owned())),
            found => Err(Error::UnexpectedOutput {
                what: "forwardagent",
                source_name: "ssh -G".to_owned(),
                found: found.unwrap_or_default().to_owned(),
            }),
        }
    }
}

/// Whether SSH sessions to a host carry an agent, and whose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForwardAgent {
    No,
    /// The agent `SSH_AUTH_SOCK` names.
    Yes,
    /// An agent socket path, or `$VAR` naming one.
    Socket(String),
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

    fn proxy_check(&self, host: &str) -> Result<FromContainer<ProxyCheck>, Error> {
        ssh_proxy_check(host)
    }

    fn ssh_config(&self, host: &str) -> Result<SshOptions, Error> {
        run("ssh", ["-G", host]).map(|text| SshOptions::parse(&text))
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

/// Runs `ssh <host> true`.
fn ssh_true(host: &str) -> Result<SshReach, Error> {
    Ok(match ssh_run(host, "true")? {
        SshRun::Finished { status, .. } if status.success() => SshReach::Reachable,
        SshRun::Finished { status, stderr, .. } => SshReach::Failed {
            exit_code: status.code(),
            stderr,
        },
        SshRun::TimedOut => SshReach::TimedOut,
    })
}

/// Runs [`PROXY_CHECK`] in the container and reads its answer.
fn ssh_proxy_check(host: &str) -> Result<FromContainer<ProxyCheck>, Error> {
    match ssh_run(host, PROXY_CHECK)? {
        SshRun::Finished { status, stderr, .. } if !status.success() => Ok(FromContainer::Failed {
            exit_code: status.code(),
            stderr,
        }),
        SshRun::Finished { stdout, .. } => serde_json::from_str(&stdout)
            .map(FromContainer::Answered)
            .map_err(|source| Error::ProxyCheckOutput {
                host: host.to_owned(),
                command: PROXY_CHECK,
                found: stdout,
                source,
            }),
        SshRun::TimedOut => Ok(FromContainer::TimedOut),
    }
}

enum SshRun {
    Finished {
        status: ExitStatus,
        stdout: String,
        stderr: String,
    },
    /// Killed at [`SSH_DEADLINE`].
    TimedOut,
}

/// Runs `command` on `host` without forwarding anything and kills ssh at
/// the deadline, since `ConnectTimeout` does not bound a session that hangs.
fn ssh_run(host: &str, command: &str) -> Result<SshRun, Error> {
    let options = [
        "BatchMode=yes",
        "ConnectTimeout=5",
        "ForwardAgent=no",
        "ClearAllForwardings=yes",
    ];
    let mut child = Command::new("ssh")
        .args(options.iter().flat_map(|option| ["-o", option]))
        .args([host, command])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| Error::Spawn {
            program: "ssh",
            source,
        })?;
    let stdout_read = drain(child.stdout.take().expect("stdout is piped"));
    let stderr_read = drain(child.stderr.take().expect("stderr is piped"));
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
            return Ok(SshRun::TimedOut);
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    Ok(SshRun::Finished {
        status,
        stdout: collect(&stdout_read, "stdout"),
        stderr: collect(&stderr_read, "stderr"),
    })
}

type Drained = mpsc::Receiver<io::Result<Vec<u8>>>;

/// Reads a pipe to its end on its own thread.
fn drain(mut pipe: impl io::Read + Send + 'static) -> Drained {
    let (sender, read) = mpsc::channel();
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        // The receiver is gone only when ssh timed out or its pipe stayed open.
        let _ = sender.send(pipe.read_to_end(&mut bytes).map(|_| bytes));
    });
    read
}

/// A process ssh forked, such as a ControlPersist master, may hold a pipe
/// open after ssh exits, so each read gets a short grace period.
fn collect(read: &Drained, name: &str) -> String {
    match read.recv_timeout(Duration::from_secs(1)) {
        Ok(Ok(bytes)) => String::from_utf8_lossy(&bytes).trim().to_owned(),
        Ok(Err(error)) => format!("({name} unreadable: {error})"),
        Err(RecvTimeoutError::Timeout) => format!("({name} still open after ssh exited)"),
        Err(RecvTimeoutError::Disconnected) => format!("({name} reader stopped)"),
    }
}
