//! systemd unit state as `systemctl show` reports it.

use serde::Serialize;

use crate::config::UnitName;
use crate::error::Error;

/// Which service manager owns a unit: the system's, or the login user's
/// (`systemctl --user`), where the token broker runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitScope {
    System,
    User,
}

impl UnitScope {
    pub(super) fn systemctl_flag(self) -> &'static [&'static str] {
        match self {
            Self::System => &[],
            Self::User => &["--user"],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct UnitState {
    pub load: LoadState,
    pub active: ActiveState,
}

/// systemd's `LoadState` property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoadState {
    Stub,
    Loaded,
    NotFound,
    BadSetting,
    Error,
    Merged,
    Masked,
}

/// systemd's `ActiveState` property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActiveState {
    Active,
    Reloading,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Maintenance,
    Refreshing,
}

impl LoadState {
    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "stub" => Self::Stub,
            "loaded" => Self::Loaded,
            "not-found" => Self::NotFound,
            "bad-setting" => Self::BadSetting,
            "error" => Self::Error,
            "merged" => Self::Merged,
            "masked" => Self::Masked,
            _ => return None,
        })
    }
}

impl ActiveState {
    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "active" => Self::Active,
            "reloading" => Self::Reloading,
            "inactive" => Self::Inactive,
            "failed" => Self::Failed,
            "activating" => Self::Activating,
            "deactivating" => Self::Deactivating,
            "maintenance" => Self::Maintenance,
            "refreshing" => Self::Refreshing,
            _ => return None,
        })
    }
}

impl UnitState {
    /// Parses `systemctl show -p LoadState -p ActiveState` output.
    pub fn parse(unit: &UnitName, output: &str) -> Result<Self, Error> {
        let unexpected = |what: &'static str, found: &str| Error::UnexpectedOutput {
            what,
            source_name: format!("systemctl show {unit}"),
            found: found.to_owned(),
        };
        let property = |name: &'static str| {
            output
                .lines()
                .find_map(|line| line.strip_prefix(name)?.strip_prefix('='))
                .ok_or_else(|| unexpected(name, output))
        };
        let load = property("LoadState")?;
        let active = property("ActiveState")?;
        Ok(Self {
            load: LoadState::parse(load).ok_or_else(|| unexpected("LoadState", load))?,
            active: ActiveState::parse(active).ok_or_else(|| unexpected("ActiveState", active))?,
        })
    }
}
