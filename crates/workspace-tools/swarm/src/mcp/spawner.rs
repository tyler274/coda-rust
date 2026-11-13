use std::error::Error;
use std::sync::Arc;

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
    /// The `add` call's name: members of one group share a `share` worktree.
    pub group: String,
    pub brief: String,
    pub model: Option<String>,
    pub workspace: Workspace,
    pub channels: Vec<ChannelName>,
    /// A member whose session the new member continues.
    pub fork: Option<MemberName>,
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
    /// [`Self::models`] with endpoint availability, for the `models` action.
    fn probe_models(&self) -> Catalog {
        self.models()
    }
    fn spawn(&self, spec: &SpawnSpec) -> Result<(), SpawnError>;
    fn kill(&self, name: &MemberName) -> Result<(), SpawnError>;
    fn set_model(&self, name: &MemberName, model: &str) -> Result<(), SpawnError>;
}

impl<T: Spawner + ?Sized> Spawner for Arc<T> {
    fn models(&self) -> Catalog {
        (**self).models()
    }

    fn probe_models(&self) -> Catalog {
        (**self).probe_models()
    }

    fn spawn(&self, spec: &SpawnSpec) -> Result<(), SpawnError> {
        (**self).spawn(spec)
    }

    fn kill(&self, name: &MemberName) -> Result<(), SpawnError> {
        (**self).kill(name)
    }

    fn set_model(&self, name: &MemberName, model: &str) -> Result<(), SpawnError> {
        (**self).set_model(name, model)
    }
}

/// Runs nothing. Members exist only on the roster, so the board works without
/// a model registry and in every worker's server.
#[derive(Debug, Clone, Copy, Default)]
pub struct RosterOnlySpawner;

impl Spawner for RosterOnlySpawner {
    fn models(&self) -> Catalog {
        Catalog::Unregistered
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
