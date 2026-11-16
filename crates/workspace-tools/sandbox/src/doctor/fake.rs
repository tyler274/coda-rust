//! A machine described by maps, for the doctor's verdict tests.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use super::checks::ARP_IGNORE;
use crate::broker::error::BrokerError;
use crate::broker::wire::{GhFallback, ItemReport, ItemState, Report};
use crate::broker::{Item, Source};
use crate::config::UnitName;
use crate::editor_settings::{Editor, RemoteServer, ServerExtensions};
use crate::error::Error;
use crate::model_proxy::ProxyCheck;
use crate::probe::{
    ActiveState, FromContainer, FsType, LoadState, PathKind, PoolFeature, Probe, SshOptions,
    SshReach, UnitScope, UnitState, ZFS_BCLONE_PARAMETER,
};

/// Paths it does not list are missing; paths under `denied` fail with
/// permission denied, as root-owned paths do for the host user. A `None`
/// pool feature means `zpool` cannot read the pool.
pub(super) struct FakeProbe {
    pub units: HashMap<String, UnitState>,
    pub dirs: Vec<PathBuf>,
    pub files: HashMap<PathBuf, String>,
    pub denied: Vec<PathBuf>,
    pub fs: FsType,
    pub pool_feature: Option<PoolFeature>,
    pub ssh: SshReach,
    /// What `ssh -G` prints for the container's host.
    pub ssh_options: String,
    /// `None` when no broker listens on the socket.
    pub broker: Option<Report>,
    pub proxy_check: FromContainer<ProxyCheck>,
    /// What listing an editor server's extensions in the container gives.
    pub server_extensions: FromContainer<ServerExtensions>,
}

fn denied(path: &Path) -> Error {
    Error::Inspect {
        path: path.to_owned(),
        source: io::Error::from(io::ErrorKind::PermissionDenied),
    }
}

impl Probe for FakeProbe {
    fn unit(&self, _: UnitScope, unit: &UnitName) -> Result<UnitState, Error> {
        let missing = UnitState {
            load: LoadState::NotFound,
            active: ActiveState::Inactive,
        };
        Ok(self.units.get(unit.as_str()).copied().unwrap_or(missing))
    }

    fn path_kind(&self, path: &Path) -> Result<PathKind, Error> {
        if self.denied.iter().any(|d| path.starts_with(d)) {
            return Err(denied(path));
        }
        if self.dirs.iter().any(|d| d == path) {
            return Ok(PathKind::Directory);
        }
        Ok(if self.files.contains_key(path) {
            PathKind::File
        } else {
            PathKind::Missing
        })
    }

    fn fs_type(&self, _: &Path) -> Result<FsType, Error> {
        Ok(self.fs)
    }

    fn mount_source(&self, _: &Path) -> Result<String, Error> {
        Ok("agents/coda/build".to_owned())
    }

    fn pool_block_cloning(&self, pool: &str) -> Result<PoolFeature, Error> {
        assert_eq!(pool, "agents");
        self.pool_feature.ok_or_else(|| Error::UnexpectedOutput {
            what: "feature@block_cloning",
            source_name: format!("zpool get {pool}"),
            found: "no such pool available".to_owned(),
        })
    }

    fn read_file(&self, path: &Path) -> Result<Option<String>, Error> {
        if self.denied.iter().any(|d| path.starts_with(d)) {
            return Err(denied(path));
        }
        Ok(self.files.get(path).cloned())
    }

    fn count_entries(&self, _: &Path) -> Result<usize, Error> {
        Ok(0)
    }

    fn ssh(&self, _: &str) -> Result<SshReach, Error> {
        Ok(self.ssh.clone())
    }

    fn proxy_check(&self, _: &str) -> Result<FromContainer<ProxyCheck>, Error> {
        Ok(self.proxy_check.clone())
    }

    fn server_extensions(
        &self,
        _: &str,
        _: RemoteServer,
    ) -> Result<FromContainer<ServerExtensions>, Error> {
        Ok(self.server_extensions.clone())
    }

    fn ssh_config(&self, _: &str) -> Result<SshOptions, Error> {
        Ok(SshOptions::parse(&self.ssh_options))
    }

    fn home(&self) -> Option<PathBuf> {
        Some(PathBuf::from("/home/me"))
    }

    fn broker(&self, socket: &Path) -> Result<Report, BrokerError> {
        self.broker.clone().ok_or_else(|| BrokerError::Connect {
            socket: socket.to_owned(),
            source: io::Error::from(io::ErrorKind::NotFound),
        })
    }
}

/// The broker's report on a host with the CI and the model proxy when every
/// token is in the keyring.
pub(super) fn stored() -> Report {
    Report {
        items: [
            Item::GithubAgent,
            Item::Doppler,
            Item::ModelProxy,
            Item::GithubCi,
        ]
        .map(|item| ItemReport {
            item,
            state: ItemState::Available {
                source: Source::Keyring,
            },
        })
        .to_vec(),
        gh_fallback: GhFallback::Off,
        fallback_used: false,
    }
}

const SAFE_SETTINGS: &str = r#"{
  // forwarding stays off
  "remote.SSH.enableAgentForwarding": false,
  "git.terminalAuthentication": false,
  "github.gitAuthentication": false,
  "remote.SSH.defaultExtensions": ["mkhl.direnv", "rust-lang.rust-analyzer"],
}"#;

/// Cyrene as it should be, with the legacy token directories gone.
pub(super) fn healthy() -> FakeProbe {
    let active = UnitState {
        load: LoadState::Loaded,
        active: ActiveState::Active,
    };
    let units = ["container@coda-agent.service", "coda-ci.timer"];
    let mut files = HashMap::from([
        (PathBuf::from(ZFS_BCLONE_PARAMETER), "1\n".to_owned()),
        (PathBuf::from(ARP_IGNORE), "1\n".to_owned()),
    ]);
    files.insert(settings_path(Editor::Cursor), SAFE_SETTINGS.to_owned());
    FakeProbe {
        units: units.iter().map(|u| ((*u).to_owned(), active)).collect(),
        dirs: [
            "/agents/coda/build",
            "/agents/coda/ci/spool/requests",
            "/agents/coda/ci/spool/results",
        ]
        .map(PathBuf::from)
        .to_vec(),
        files,
        denied: Vec::new(),
        fs: FsType::Zfs,
        pool_feature: Some(PoolFeature::Active),
        ssh: SshReach::Reachable,
        ssh_options: "hostname 10.233.1.2\nforwardagent no\nuser agent\n".to_owned(),
        broker: Some(stored()),
        proxy_check: FromContainer::Answered(ProxyCheck::Accepted {
            client: Some("coda-agent".to_owned()),
        }),
        server_extensions: FromContainer::Answered(ServerExtensions::Missing(Vec::new())),
    }
}

/// An editor's settings file under the fake home.
pub(super) fn settings_path(editor: Editor) -> PathBuf {
    editor
        .settings()
        .expect("the editor has a settings file")
        .path(Path::new("/home/me"))
}
