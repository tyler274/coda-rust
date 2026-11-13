//! What crosses the socket: one JSON line from the client, one JSON line
//! back, then the broker closes the connection.

use serde::{Deserialize, Serialize};

use super::{Item, Secret, Source};

/// The longest request line the broker reads; anything longer is malformed.
pub const MAX_REQUEST: u64 = 4096;

/// A request. Each kind belongs to one [`Audience`], and the broker answers
/// it only for a peer of that audience, on a door that serves it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Request {
    /// git's `get` for an HTTPS GitHub remote.
    GitCredential { protocol: Protocol, host: GitHost },
    /// The token the container's `gh` wrapper puts in `GH_TOKEN`.
    GhToken,
    /// The token the container's `doppler` wrapper puts in `DOPPLER_TOKEN`.
    DopplerToken,
    /// The model proxy's client key, which Claude Code's `apiKeyHelper` and
    /// Codex's provider `auth.command` print.
    ModelProxyKey,
    /// The token the CI controller talks to GitHub with.
    CiToken,
    /// Which items the broker can serve and from where; never a secret.
    Ping,
    /// Drop the cache, so a stored or rotated token is read on next use.
    Reload,
}

/// Who may send a request kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
    /// The agent in the container.
    Agent,
    /// The CI controller, the host's `coda-ci` user.
    Ci,
    /// The host user the broker runs as: `doctor`, `status`, `broker store`.
    Owner,
}

/// A secret the container's agent may ask for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentSecret {
    Github,
    Doppler,
    ModelProxy,
}

impl AgentSecret {
    pub fn item(self) -> Item {
        match self {
            Self::Github => Item::GithubAgent,
            Self::Doppler => Item::Doppler,
            Self::ModelProxy => Item::ModelProxy,
        }
    }
}

/// What a request asks the broker to do, with each kind's data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Asked {
    /// One of the agent's secrets.
    Agent(AgentSecret),
    /// The CI controller's GitHub token.
    CiToken,
    /// The owner's report.
    Ping,
    /// The owner's cache drop.
    Reload,
}

impl Asked {
    pub fn audience(self) -> Audience {
        match self {
            Self::Agent(_) => Audience::Agent,
            Self::CiToken => Audience::Ci,
            Self::Ping | Self::Reload => Audience::Owner,
        }
    }
}

impl Request {
    pub fn asked(&self) -> Asked {
        match self {
            Self::GitCredential { .. } | Self::GhToken => Asked::Agent(AgentSecret::Github),
            Self::DopplerToken => Asked::Agent(AgentSecret::Doppler),
            Self::ModelProxyKey => Asked::Agent(AgentSecret::ModelProxy),
            Self::CiToken => Asked::CiToken,
            Self::Ping => Asked::Ping,
            Self::Reload => Asked::Reload,
        }
    }

    /// The `kind` tag, for the audit log.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::GitCredential { .. } => "git_credential",
            Self::GhToken => "gh_token",
            Self::DopplerToken => "doppler_token",
            Self::ModelProxyKey => "model_proxy_key",
            Self::CiToken => "ci_token",
            Self::Ping => "ping",
            Self::Reload => "reload",
        }
    }
}

/// The git protocols the broker serves a credential for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Https,
}

/// The git hosts the broker serves a credential for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitHost {
    #[serde(rename = "github.com")]
    GithubCom,
}

/// The broker's answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum Reply {
    Granted { secret: Secret },
    Unavailable { reason: Unavailable },
    Denied { reason: Denied },
    Report { report: Report },
    Reloaded,
}

impl Reply {
    /// The `outcome` tag, for the audit log.
    pub fn outcome(&self) -> &'static str {
        match self {
            Self::Granted { .. } => "granted",
            Self::Unavailable { .. } => "unavailable",
            Self::Denied { .. } => "denied",
            Self::Report { .. } => "report",
            Self::Reloaded => "reloaded",
        }
    }
}

/// The item exists in principle but cannot be served now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum Unavailable {
    /// The keyring did not answer or is locked, and nothing is cached.
    #[error(
        "the keyring on the host is locked or not running, and no token is cached; log in on the host"
    )]
    KeyringLocked,
    /// The keyring has no such item.
    #[error("no token is stored; run `coda-sandbox broker store <item>` on the host")]
    NotStored,
    /// `gh auth token` failed while it was the fallback.
    #[error("the `gh auth token` fallback failed; see the broker's journal")]
    FallbackFailed,
    /// The keyring holds something that is not a token: empty, or with a
    /// space, control or non-ASCII character.
    #[error(
        "the stored token is not printable ASCII; run `coda-sandbox broker store <item>` on the host"
    )]
    NotAToken,
}

/// The broker refused the request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum Denied {
    #[error("this user may not ask the broker for that")]
    WrongPeer,
    #[error("too many requests; the broker is rate limiting this caller")]
    RateLimited,
    #[error("the broker could not read the request")]
    Malformed,
    /// Too many connections are open on this socket at once.
    #[error("the broker has too many connections open on this socket; try again")]
    Busy,
}

/// Ping's answer: each served item's state and the fallback's.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub items: Vec<ItemReport>,
    pub gh_fallback: GhFallback,
    /// Whether any token was served from `gh auth token` since the broker
    /// started.
    pub fallback_used: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemReport {
    pub item: Item,
    pub state: ItemState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ItemState {
    Available {
        source: Source,
    },
    /// Served from the cache because the keyring is locked or unreachable;
    /// the last good read was `read_secs_ago` seconds ago.
    Stale {
        source: Source,
        read_secs_ago: u64,
    },
    Unavailable {
        reason: Unavailable,
    },
}

/// Whether `gh auth token` may stand in for a `github-agent` the keyring
/// does not hold. It never stands in for a locked keyring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GhFallback {
    #[default]
    Off,
    On,
}
