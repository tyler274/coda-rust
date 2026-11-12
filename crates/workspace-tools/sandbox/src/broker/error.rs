use std::io;
use std::path::PathBuf;

use super::keyring::KeyringError;
use super::wire::{Denied, Unavailable};
use super::{Item, SecretError};

/// What can go wrong talking to, running or filling the broker.
#[derive(Debug, thiserror::Error)]
pub enum BrokerError {
    #[error(
        "cannot reach the token broker at {socket}; is coda-sandbox-broker running on the host?"
    )]
    Connect {
        socket: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the exchange with the token broker at {socket} failed")]
    Exchange {
        socket: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the token broker at {socket} sent a reply this binary cannot read")]
    Decode {
        socket: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("{item} is unavailable")]
    Unavailable {
        item: Item,
        #[source]
        reason: Unavailable,
    },
    #[error("the token broker refused")]
    Denied(#[source] Denied),
    #[error("the token broker answered {got} to a {expected} request")]
    UnexpectedReply {
        expected: &'static str,
        got: &'static str,
    },
    #[error("the keyring could not {op} {item}")]
    Keyring {
        op: &'static str,
        item: Item,
        #[source]
        source: KeyringError,
    },
    #[error("cannot listen on {socket}")]
    Listen {
        socket: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("{socket} exists and is not a socket; move it away")]
    NotASocket { socket: PathBuf },
    #[error("cannot read the secret from standard input")]
    ReadSecret(#[source] io::Error),
    #[error("standard input did not hold a token")]
    NotAToken(#[source] SecretError),
    #[error("refusing to print a token to a terminal; pipe it, or pass --force")]
    Terminal,
    #[error("the keyring returned something other than the secret just stored")]
    SelfTestMismatch,
    #[error("the keyring still holds {item} after deleting it")]
    SelfTestNotDeleted { item: Item },
    #[error("cannot look up the CI controller's user {user}")]
    UserLookup {
        user: String,
        #[source]
        source: io::Error,
    },
    #[error("the CI controller's user {user} does not exist on this host")]
    NoSuchUser { user: String },
    #[error("cannot handle SIGHUP")]
    Signals(#[source] io::Error),
    #[error("cannot turn off core dumps for the broker")]
    NotDumpable(#[source] io::Error),
}
