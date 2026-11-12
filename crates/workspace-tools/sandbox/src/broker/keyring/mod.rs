//! Where the broker's secrets live: the user's Secret Service keyring.
//!
//! [`SecretTool`] reaches it through libsecret's `secret-tool`, with fixed
//! arguments and the secret on stdin, never on argv. [`Memory`] is the fake
//! the tests use. `gh auth token` is the optional fallback for a
//! `github-agent` the keyring does not hold ([`GhCli`]).

mod command;
mod memory;
mod secret_tool;

use std::io;
use std::process::ExitStatus;
use std::time::Duration;

pub use memory::{Locked, Memory, MemoryState};
pub use secret_tool::{GhCli, SecretTool};

use super::{Item, Secret, SecretError};

/// How long a keyring or `gh` call may take. A locked keyring with no
/// session can wait forever on a prompt nobody sees.
pub const DEADLINE: Duration = Duration::from_secs(5);

/// The user's secret store.
pub trait Keyring: Send + Sync {
    /// `Ok(None)` when the keyring answered and holds no such item.
    fn lookup(&self, item: Item) -> Result<Option<Secret>, KeyringError>;
    /// Writes or replaces the item.
    fn store(&self, item: Item, secret: &Secret) -> Result<(), KeyringError>;
    /// Removes the item; removing an absent item succeeds.
    fn delete(&self, item: Item) -> Result<(), KeyringError>;
}

/// The general token `gh` holds, used only when the fallback is on.
pub trait Fallback: Send + Sync {
    fn gh_token(&self) -> Result<Secret, KeyringError>;
}

/// A keyring or `gh` call that did not give an answer. The broker reports
/// each of these as the keyring being unavailable, never as "not stored".
#[derive(Debug, thiserror::Error)]
pub enum KeyringError {
    #[error("cannot run {program}")]
    Spawn {
        program: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("cannot exchange data with {program}")]
    Io {
        program: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("{program} did not answer within {} s", DEADLINE.as_secs())]
    TimedOut { program: &'static str },
    #[error("{program} {status}: {stderr}")]
    Failed {
        program: &'static str,
        status: ExitStatus,
        stderr: String,
    },
    /// It answered, with something that is not a token.
    #[error("{program} printed something that is not a token")]
    NotAToken {
        program: &'static str,
        #[source]
        source: SecretError,
    },
    /// The memory fake's stand-in for a locked keyring.
    #[error("the keyring is locked")]
    Locked,
}
