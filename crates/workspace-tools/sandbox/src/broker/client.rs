//! One request to the broker and its typed reply.

use std::io::{BufRead as _, BufReader, Write as _};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

use super::error::BrokerError;
use super::wire::{GitHost, Protocol, Reply, Report, Request};
use super::{Item, Secret};

/// The broker's agent socket on the host and, through the container's bind
/// of its directory, inside the container.
pub const SOCKET: &str = "/run/coda-sandbox/agent/broker.sock";

/// How long the client waits for the broker; a keyring read may take up to
/// its own 5 s deadline.
const TIMEOUT: Duration = Duration::from_secs(10);

pub fn exchange(socket: &Path, request: &Request) -> Result<Reply, BrokerError> {
    let exchange = |source| BrokerError::Exchange {
        socket: socket.to_owned(),
        source,
    };
    let mut stream = UnixStream::connect(socket).map_err(|source| BrokerError::Connect {
        socket: socket.to_owned(),
        source,
    })?;
    stream.set_read_timeout(Some(TIMEOUT)).map_err(exchange)?;
    stream.set_write_timeout(Some(TIMEOUT)).map_err(exchange)?;
    let mut line = serde_json::to_vec(request).expect("a request always serializes");
    line.push(b'\n');
    stream.write_all(&line).map_err(exchange)?;
    let mut reply = String::new();
    BufReader::new(&stream)
        .read_line(&mut reply)
        .map_err(exchange)?;
    serde_json::from_str(&reply).map_err(|source| BrokerError::Decode {
        socket: socket.to_owned(),
        source,
    })
}

/// A secret the container asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Want {
    /// git's HTTPS credential for github.com.
    #[value(skip)]
    GitCredential,
    /// The GitHub token for `gh`.
    Gh,
    /// The Doppler service token for `doppler`.
    Doppler,
    /// The model proxy's client key, for Claude Code's `apiKeyHelper` and
    /// Codex's provider `auth.command`.
    ModelProxy,
}

impl Want {
    pub fn request(self) -> Request {
        match self {
            Self::GitCredential => Request::GitCredential {
                protocol: Protocol::Https,
                host: GitHost::GithubCom,
            },
            Self::Gh => Request::GhToken,
            Self::Doppler => Request::DopplerToken,
            Self::ModelProxy => Request::ModelProxyKey,
        }
    }

    pub fn item(self) -> Item {
        match self {
            Self::GitCredential | Self::Gh => Item::GithubAgent,
            Self::Doppler => Item::Doppler,
            Self::ModelProxy => Item::ModelProxy,
        }
    }
}

/// Asks for a secret; every reply but `granted` is an error naming why.
pub fn secret(socket: &Path, want: Want) -> Result<Secret, BrokerError> {
    let item = want.item();
    let request = want.request();
    match exchange(socket, &request)? {
        Reply::Granted { secret } => Ok(secret),
        Reply::Unavailable { reason } => Err(BrokerError::Unavailable { item, reason }),
        Reply::Denied { reason } => Err(BrokerError::Denied(reason)),
        other @ (Reply::Report { .. } | Reply::Reloaded) => Err(BrokerError::UnexpectedReply {
            expected: request.kind(),
            got: other.outcome(),
        }),
    }
}

/// Asks the CI door for the controller's GitHub token; every reply but
/// `granted` is an error naming why.
pub fn ci_token(socket: &Path) -> Result<Secret, BrokerError> {
    let request = Request::CiToken;
    match exchange(socket, &request)? {
        Reply::Granted { secret } => Ok(secret),
        Reply::Unavailable { reason } => Err(BrokerError::Unavailable {
            item: Item::GithubCi,
            reason,
        }),
        Reply::Denied { reason } => Err(BrokerError::Denied(reason)),
        other @ (Reply::Report { .. } | Reply::Reloaded) => Err(BrokerError::UnexpectedReply {
            expected: request.kind(),
            got: other.outcome(),
        }),
    }
}

pub fn ping(socket: &Path) -> Result<Report, BrokerError> {
    match exchange(socket, &Request::Ping)? {
        Reply::Report { report } => Ok(report),
        Reply::Denied { reason } => Err(BrokerError::Denied(reason)),
        other @ (Reply::Granted { .. } | Reply::Unavailable { .. } | Reply::Reloaded) => {
            Err(BrokerError::UnexpectedReply {
                expected: "ping",
                got: other.outcome(),
            })
        }
    }
}

pub fn reload(socket: &Path) -> Result<(), BrokerError> {
    match exchange(socket, &Request::Reload)? {
        Reply::Reloaded => Ok(()),
        Reply::Denied { reason } => Err(BrokerError::Denied(reason)),
        other @ (Reply::Granted { .. } | Reply::Unavailable { .. } | Reply::Report { .. }) => {
            Err(BrokerError::UnexpectedReply {
                expected: "reload",
                got: other.outcome(),
            })
        }
    }
}
