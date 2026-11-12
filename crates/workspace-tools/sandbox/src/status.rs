//! `coda-sandbox status`: a compact snapshot, and with `--json` the stable
//! schema an editor extension or script reads.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::broker::wire::Report;
use crate::config::{Config, ModelKeys, UnitName};
use crate::doctor::LegacyTokenDir;
use crate::error::Error;
use crate::probe::{LoadState, PathKind, Probe, SshReach, UnitScope, UnitState};

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Serialize)]
pub struct Status {
    pub schema_version: u32,
    pub config: PathBuf,
    pub container: ContainerStatus,
    pub build_root: Reading<PathKind>,
    pub ci: CiStatus,
    pub broker: BrokerStatus,
    pub legacy_token_dirs: Vec<TokenDir>,
}

/// A value read from the machine, or why it could not be read.
#[derive(Debug, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Reading<T> {
    Known { value: T },
    Unknown { error: String },
}

impl<T> From<Result<T, Error>> for Reading<T> {
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(value) => Self::Known { value },
            Err(error) => Self::Unknown {
                error: error.chain(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UnitStatus {
    pub unit: UnitName,
    pub scope: UnitScope,
    pub state: Reading<UnitState>,
}

/// The outcome of `ssh <host> true`.
#[derive(Debug, Serialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum Reach {
    Reachable,
    /// `exit_code` is null when a signal ended ssh.
    Failed {
        exit_code: Option<i32>,
        stderr: String,
    },
    TimedOut,
}

impl From<SshReach> for Reach {
    fn from(reach: SshReach) -> Self {
        match reach {
            SshReach::Reachable => Self::Reachable,
            SshReach::Failed { exit_code, stderr } => Self::Failed { exit_code, stderr },
            SshReach::TimedOut => Self::TimedOut,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ContainerStatus {
    pub name: String,
    pub unit: UnitStatus,
    pub ssh_host: String,
    pub ssh: Reading<Reach>,
}

/// CI controller state the host user can read. The request queue is not
/// listed: its directory is write-only to requesters by design.
#[derive(Debug, Serialize)]
pub struct CiStatus {
    pub timer: UnitStatus,
    pub service: UnitStatus,
    /// Entries in the results directory.
    pub results: Reading<usize>,
}

/// The token broker's socket and its report of what it can serve.
#[derive(Debug, Serialize)]
pub struct BrokerStatus {
    pub socket: PathBuf,
    /// Which agent CLIs read their API key from the broker.
    pub model_keys: ModelKeys,
    pub report: Reading<Report>,
}

/// A directory that holds a root-owned GitHub token in a file.
#[derive(Debug, Serialize)]
pub struct TokenDir {
    pub path: PathBuf,
    pub kind: Reading<PathKind>,
}

fn unit(probe: &dyn Probe, scope: UnitScope, unit: &UnitName) -> UnitStatus {
    UnitStatus {
        unit: unit.clone(),
        scope,
        state: probe.unit(scope, unit).into(),
    }
}

pub fn collect(probe: &dyn Probe, config_path: &Path, config: &Config) -> Status {
    Status {
        schema_version: SCHEMA_VERSION,
        config: config_path.to_owned(),
        container: ContainerStatus {
            name: config.container.name.clone(),
            unit: unit(probe, UnitScope::System, &config.container.unit),
            ssh_host: config.container.ssh_host.clone(),
            ssh: probe
                .ssh(&config.container.ssh_host)
                .map(Reach::from)
                .into(),
        },
        build_root: probe.path_kind(&config.build.root).into(),
        ci: CiStatus {
            timer: unit(probe, UnitScope::System, &config.ci.timer),
            service: unit(probe, UnitScope::System, &config.ci.service),
            results: probe.count_entries(&config.ci.results()).into(),
        },
        broker: BrokerStatus {
            socket: config.broker.socket.clone(),
            model_keys: config.broker.model_keys,
            report: probe
                .broker(&config.broker.socket)
                .map_err(Error::from)
                .into(),
        },
        legacy_token_dirs: LegacyTokenDir::ALL
            .into_iter()
            .map(|dir| TokenDir {
                path: dir.path().to_owned(),
                kind: probe.path_kind(dir.path()).into(),
            })
            .collect(),
    }
}

fn show<T>(reading: &Reading<T>, known: impl Fn(&T) -> String) -> String {
    match reading {
        Reading::Known { value } => known(value),
        Reading::Unknown { error } => format!("unknown ({error})"),
    }
}

fn show_unit(status: &UnitStatus) -> String {
    show(&status.state, |state| match (state.load, state.active) {
        (LoadState::Loaded, active) => format!("{active:?}").to_lowercase(),
        (load, _) => format!("{load:?}").to_lowercase(),
    })
}

impl Status {
    pub fn render(&self) -> String {
        let mut out = String::new();
        let ssh = show(&self.container.ssh, |reach| match reach {
            Reach::Reachable => "reachable".to_owned(),
            Reach::Failed {
                exit_code: Some(code),
                ..
            } => format!("failed (exit {code})"),
            Reach::Failed {
                exit_code: None, ..
            } => "failed (killed)".to_owned(),
            Reach::TimedOut => "timed out".to_owned(),
        });
        let _ = writeln!(
            out,
            "container  {} {}, ssh {} {ssh}",
            self.container.unit.unit,
            show_unit(&self.container.unit),
            self.container.ssh_host
        );
        let _ = writeln!(
            out,
            "build root {}",
            show(&self.build_root, |kind| format!("{kind:?}").to_lowercase())
        );
        let _ = writeln!(
            out,
            "ci         timer {}, service {}, {} results",
            show_unit(&self.ci.timer),
            show_unit(&self.ci.service),
            show(&self.ci.results, usize::to_string)
        );
        let _ = writeln!(
            out,
            "broker     {}",
            show(&self.broker.report, |report| {
                crate::doctor::broker::describe(report, self.broker.model_keys)
            })
        );
        let (mut absent, mut present, mut unreadable) = (0, 0, 0);
        for dir in &self.legacy_token_dirs {
            match dir.kind {
                Reading::Known {
                    value: PathKind::Missing,
                } => absent += 1,
                Reading::Known { .. } => present += 1,
                Reading::Unknown { .. } => unreadable += 1,
            }
        }
        let _ = writeln!(
            out,
            "tokens     legacy token dirs: {present} present, {absent} absent, {unreadable} unreadable"
        );
        out
    }
}
