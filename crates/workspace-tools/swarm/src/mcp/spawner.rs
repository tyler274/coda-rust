use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::adapter::Catalog;
use crate::names::{ChannelName, MemberName};

/// Where a new member works: the caller's checkout, or its own worktree or
/// clone that is deleted when the member is killed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Workspace {
    #[default]
    Inherit,
    Share,
    Branch,
}

/// Everything a spawner needs to start one member. The roster entry and the
/// brief post are the server's job; the spawner only runs the member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnSpec {
    pub name: MemberName,
    pub brief: String,
    pub model: Option<String>,
    pub workspace: Workspace,
    pub channels: Vec<ChannelName>,
}

#[derive(Debug, thiserror::Error)]
pub enum SpawnError {
    #[error("model {0} is not available")]
    UnknownModel(String),
    #[error("{action} {member}")]
    Failed {
        action: &'static str,
        member: MemberName,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
}

/// Runs member processes. The server checks authority and models and records
/// every roster change; a spawner starts, stops and reconfigures what runs.
pub trait Spawner {
    fn models(&self) -> Catalog;
    fn spawn(&self, spec: &SpawnSpec) -> Result<(), SpawnError>;
    fn kill(&self, name: &MemberName) -> Result<(), SpawnError>;
    fn set_model(&self, name: &MemberName, model: &str) -> Result<(), SpawnError>;
}

/// Runs nothing. Members exist only on the roster, so the board works before
/// a supervisor does.
#[derive(Debug, Clone, Copy, Default)]
pub struct RosterOnlySpawner;

impl Spawner for RosterOnlySpawner {
    fn models(&self) -> Catalog {
        Catalog::Any
    }

    fn spawn(&self, _spec: &SpawnSpec) -> Result<(), SpawnError> {
        Ok(())
    }

    fn kill(&self, _name: &MemberName) -> Result<(), SpawnError> {
        Ok(())
    }

    fn set_model(&self, _name: &MemberName, _model: &str) -> Result<(), SpawnError> {
        Ok(())
    }
}
