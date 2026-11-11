//! A machine described by maps, for the doctor's verdict tests.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use crate::config::UnitName;
use crate::editor_settings::Editor;
use crate::error::Error;
use crate::probe::{
    ActiveState, FsType, LoadState, PathKind, PoolFeature, Probe, SshReach, UnitScope, UnitState,
    ZFS_BCLONE_PARAMETER,
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

    fn home(&self) -> Option<PathBuf> {
        Some(PathBuf::from("/home/me"))
    }
}

const SAFE_SETTINGS: &str = r#"{
  // forwarding stays off
  "remote.SSH.enableAgentForwarding": false,
  "git.terminalAuthentication": false,
  "github.gitAuthentication": false,
}"#;

/// Cyrene as it should be, with the legacy token directories gone.
pub(super) fn healthy() -> FakeProbe {
    let active = UnitState {
        load: LoadState::Loaded,
        active: ActiveState::Active,
    };
    let units = ["container@coda-agent.service", "coda-ci.timer"];
    let mut files = HashMap::from([(PathBuf::from(ZFS_BCLONE_PARAMETER), "1\n".to_owned())]);
    files.insert(
        Editor::Cursor.settings_path(Path::new("/home/me")),
        SAFE_SETTINGS.to_owned(),
    );
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
    }
}
