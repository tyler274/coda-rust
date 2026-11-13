//! The model registry: which adapter runs each model id, with what
//! provider model name, key and endpoint. Parsed once from JSON, then run
//! by a [`RegistryAdapter`] that dispatches each turn on the member's model.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::cli::{Cli, CliAdapter, Sessions};
use super::http::{Histories, HttpAdapter, HttpKey, served_models};
use super::keys::{KeyItem, KeySource};
use super::{
    Availability, Cancel, Catalog, CommandAdapter, ForkError, ModelInfo, TurnOutcome, TurnRequest,
    WorkerAdapter,
};
use crate::names::MemberName;

const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

/// A model id as members and the coordinator name it, e.g. `claude-opus`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ModelId(String);

impl ModelId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The adapter that runs a model, with the settings only that adapter uses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "adapter", rename_all = "kebab-case")]
pub enum AdapterKind {
    /// Any program speaking the [`CommandAdapter`] protocol.
    Command {
        program: PathBuf,
        #[serde(default)]
        args: Vec<String>,
    },
    /// In the sandbox container the CLI authenticates through the host
    /// model proxy itself, so the entry carries no key.
    ClaudeCli,
    CodexCli,
    OpenaiHttp {
        base_url: String,
        /// Absent for keyless endpoints such as a local Ollama.
        #[serde(default)]
        key: Option<KeyItem>,
        #[serde(default)]
        max_tokens: Option<u32>,
        #[serde(default = "default_max_calls")]
        max_calls: u32,
    },
}

const fn default_max_calls() -> u32 {
    32
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelEntry {
    #[serde(flatten)]
    pub kind: AdapterKind,
    /// The provider's model name, passed to the CLI or API.
    pub model: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    pub models: BTreeMap<ModelId, ModelEntry>,
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("read model registry {path}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("parse model registry")]
    Parse(#[from] serde_json::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ResolveError {
    #[error("the member has no model; pick one from the model registry")]
    NoModel,
    #[error("model {0} is not in the model registry")]
    Unknown(String),
}

impl Registry {
    pub fn parse(text: &str) -> Result<Self, RegistryError> {
        Ok(serde_json::from_str(text)?)
    }

    pub fn load(path: &Path) -> Result<Self, RegistryError> {
        let text = std::fs::read_to_string(path).map_err(|source| RegistryError::Read {
            path: path.to_owned(),
            source,
        })?;
        Self::parse(&text)
    }

    pub fn catalog(&self) -> Catalog {
        Catalog::Listed(
            self.models
                .iter()
                .map(|(id, entry)| entry.info(id))
                .collect(),
        )
    }
}

impl ModelEntry {
    fn info(&self, id: &ModelId) -> ModelInfo {
        ModelInfo {
            name: id.to_string(),
            display_name: self.display_name.clone().unwrap_or_else(|| id.to_string()),
            notes: self.notes.clone(),
            availability: None,
        }
    }
}

/// An HTTP endpoint as one key reaches it: the unit a probe lists once.
type Endpoint = (String, Option<KeyItem>);

/// Runs each turn with the adapter its member's model is registered to.
/// Entries on the same CLI or API share member sessions, so a member can be
/// forked onto another model of the same kind.
pub struct RegistryAdapter {
    registry: Registry,
    adapters: BTreeMap<ModelId, Box<dyn WorkerAdapter>>,
    keys: Arc<dyn KeySource>,
    pub probe_timeout: Duration,
}

impl RegistryAdapter {
    pub fn new(registry: Registry, keys: Arc<dyn KeySource>) -> Self {
        let claude = Arc::new(Sessions::default());
        let codex = Arc::new(Sessions::default());
        let openai = Arc::new(Histories::default());
        let adapters = registry
            .models
            .iter()
            .map(|(id, entry)| {
                let model = entry.model.clone();
                let adapter: Box<dyn WorkerAdapter> = match &entry.kind {
                    AdapterKind::Command { program, args } => {
                        let mut adapter = CommandAdapter::new(program);
                        adapter.args.clone_from(args);
                        Box::new(adapter)
                    }
                    AdapterKind::ClaudeCli => Box::new(CliAdapter::with_sessions(
                        Cli::Claude,
                        model,
                        Arc::clone(&claude),
                    )),
                    AdapterKind::CodexCli => Box::new(CliAdapter::with_sessions(
                        Cli::Codex,
                        model,
                        Arc::clone(&codex),
                    )),
                    AdapterKind::OpenaiHttp {
                        base_url,
                        key,
                        max_tokens,
                        max_calls,
                    } => {
                        let key = key.map(|item| HttpKey {
                            item,
                            source: Arc::clone(&keys),
                        });
                        let mut adapter = HttpAdapter::with_histories(
                            base_url.clone(),
                            model,
                            key,
                            Arc::clone(&openai),
                        );
                        adapter.max_tokens = *max_tokens;
                        adapter.max_calls = *max_calls;
                        Box::new(adapter)
                    }
                };
                (id.clone(), adapter)
            })
            .collect();
        Self {
            registry,
            adapters,
            keys,
            probe_timeout: PROBE_TIMEOUT,
        }
    }

    fn resolve(&self, model: Option<&str>) -> Result<&dyn WorkerAdapter, ResolveError> {
        let model = model.ok_or(ResolveError::NoModel)?;
        self.adapters
            .get(&ModelId::new(model))
            .map(AsRef::as_ref)
            .ok_or_else(|| ResolveError::Unknown(model.to_owned()))
    }

    /// Lists each HTTP endpoint's models once, in parallel.
    fn probe(&self) -> BTreeMap<Endpoint, Result<BTreeSet<String>, String>> {
        let endpoints: BTreeSet<Endpoint> = self
            .registry
            .models
            .values()
            .filter_map(|entry| match &entry.kind {
                AdapterKind::OpenaiHttp { base_url, key, .. } => Some((base_url.clone(), *key)),
                _ => None,
            })
            .collect();
        thread::scope(|scope| {
            let probes: Vec<_> = endpoints
                .into_iter()
                .map(|endpoint| {
                    let (base_url, item) = endpoint.clone();
                    let probe = scope.spawn(move || {
                        let key = item
                            .map(|item| self.keys.get(item))
                            .transpose()
                            .map_err(|error| format!("key: {error}"))?;
                        let key = key.as_ref().map(|key| key.expose());
                        served_models(&base_url, key, self.probe_timeout)
                    });
                    (endpoint, probe)
                })
                .collect();
            probes
                .into_iter()
                .map(|(endpoint, probe)| {
                    let served = probe
                        .join()
                        .unwrap_or_else(|_| Err("model probe panicked".to_owned()));
                    (endpoint, served)
                })
                .collect()
        })
    }
}

impl WorkerAdapter for RegistryAdapter {
    fn run_turn(&self, request: TurnRequest, cancel: &Cancel) -> TurnOutcome {
        match self.resolve(request.model.as_deref()) {
            Ok(adapter) => adapter.run_turn(request, cancel),
            Err(error) => TurnOutcome::Failed {
                reason: error.to_string(),
            },
        }
    }

    fn fork(
        &self,
        from: &MemberName,
        to: &MemberName,
        model: Option<&str>,
    ) -> Result<(), ForkError> {
        self.resolve(model)?.fork(from, to, model)
    }

    fn models(&self) -> Catalog {
        self.registry.catalog()
    }

    fn probe_models(&self) -> Catalog {
        let probed = self.probe();
        let availability = |entry: &ModelEntry| {
            let AdapterKind::OpenaiHttp { base_url, key, .. } = &entry.kind else {
                return None;
            };
            Some(match &probed[&(base_url.clone(), *key)] {
                Ok(ids) if ids.contains(&entry.model) => Availability::Available,
                Ok(_) => Availability::NotServed,
                Err(reason) => Availability::Unreachable {
                    reason: reason.clone(),
                },
            })
        };
        Catalog::Listed(
            self.registry
                .models
                .iter()
                .map(|(id, entry)| ModelInfo {
                    availability: availability(entry),
                    ..entry.info(id)
                })
                .collect(),
        )
    }
}
