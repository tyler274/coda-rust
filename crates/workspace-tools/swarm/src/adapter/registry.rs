//! The model registry: which adapter runs each model id, with what
//! provider model name, key and endpoint. Parsed once from JSON, then run
//! by a [`RegistryAdapter`] that dispatches each turn on the member's model.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::cli::{Cli, CliAdapter, Sessions};
use super::http::{Api, Histories, HttpAdapter};
use super::{
    Cancel, Catalog, CommandAdapter, ForkError, KeyEnv, ModelInfo, TurnOutcome, TurnRequest,
    WorkerAdapter,
};
use crate::names::MemberName;

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
    ClaudeCli,
    CodexCli,
    GeminiCli,
    AnthropicHttp {
        #[serde(default = "anthropic_base_url")]
        base_url: String,
        #[serde(default = "default_max_tokens")]
        max_tokens: u32,
        #[serde(default = "default_max_calls")]
        max_calls: u32,
    },
    OpenaiHttp {
        base_url: String,
        #[serde(default)]
        max_tokens: Option<u32>,
        #[serde(default = "default_max_calls")]
        max_calls: u32,
    },
}

fn anthropic_base_url() -> String {
    "https://api.anthropic.com".to_owned()
}

const fn default_max_tokens() -> u32 {
    8192
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
    /// HTTP adapters keep this key in memory and send it as a header; CLI
    /// adapters get it in the child env. Absent for keyless endpoints such
    /// as a local Ollama, and for a CLI that fetches its own key: in the
    /// sandbox container the claude, codex and gemini wrappers ask the
    /// token broker themselves.
    #[serde(default)]
    pub key: Option<KeyEnv>,
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
                .map(|(id, entry)| ModelInfo {
                    name: id.to_string(),
                    display_name: entry.display_name.clone().unwrap_or_else(|| id.to_string()),
                    notes: entry.notes.clone(),
                })
                .collect(),
        )
    }
}

/// Runs each turn with the adapter its member's model is registered to.
/// Entries on the same CLI or API share member sessions, so a member can be
/// forked onto another model of the same kind.
pub struct RegistryAdapter {
    registry: Registry,
    adapters: BTreeMap<ModelId, Box<dyn WorkerAdapter>>,
}

impl RegistryAdapter {
    pub fn new(registry: Registry) -> Self {
        let claude = Arc::new(Sessions::default());
        let codex = Arc::new(Sessions::default());
        let gemini = Arc::new(Sessions::default());
        let anthropic = Arc::new(Histories::default());
        let openai = Arc::new(Histories::default());
        let adapters = registry
            .models
            .iter()
            .map(|(id, entry)| {
                let model = entry.model.clone();
                let key_env = entry.key.as_ref().map(|key| key.var.clone());
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
                    AdapterKind::GeminiCli => Box::new(CliAdapter::with_sessions(
                        Cli::Gemini,
                        model,
                        Arc::clone(&gemini),
                    )),
                    AdapterKind::AnthropicHttp {
                        base_url,
                        max_tokens,
                        max_calls,
                    } => {
                        let api = Api::Anthropic {
                            base_url: base_url.clone(),
                            max_tokens: *max_tokens,
                        };
                        let mut adapter = HttpAdapter::with_histories(
                            api,
                            model,
                            key_env,
                            Arc::clone(&anthropic),
                        );
                        adapter.max_calls = *max_calls;
                        Box::new(adapter)
                    }
                    AdapterKind::OpenaiHttp {
                        base_url,
                        max_tokens,
                        max_calls,
                    } => {
                        let api = Api::OpenAi {
                            base_url: base_url.clone(),
                            max_tokens: *max_tokens,
                        };
                        let mut adapter =
                            HttpAdapter::with_histories(api, model, key_env, Arc::clone(&openai));
                        adapter.max_calls = *max_calls;
                        Box::new(adapter)
                    }
                };
                (id.clone(), adapter)
            })
            .collect();
        Self { registry, adapters }
    }

    fn resolve(
        &self,
        model: Option<&str>,
    ) -> Result<(&ModelEntry, &dyn WorkerAdapter), ResolveError> {
        let model = model.ok_or(ResolveError::NoModel)?;
        let id = ModelId::new(model);
        match (self.registry.models.get(&id), self.adapters.get(&id)) {
            (Some(entry), Some(adapter)) => Ok((entry, adapter.as_ref())),
            _ => Err(ResolveError::Unknown(model.to_owned())),
        }
    }
}

impl WorkerAdapter for RegistryAdapter {
    fn run_turn(&self, request: TurnRequest, cancel: &Cancel) -> TurnOutcome {
        match self.resolve(request.model.as_deref()) {
            Ok((_, adapter)) => adapter.run_turn(request, cancel),
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
        self.resolve(model)?.1.fork(from, to, model)
    }

    fn keys(&self, model: Option<&str>) -> Vec<KeyEnv> {
        self.resolve(model)
            .ok()
            .and_then(|(entry, _)| entry.key.clone())
            .into_iter()
            .collect()
    }

    fn models(&self) -> Catalog {
        self.registry.catalog()
    }
}
