//! The token broker: a user service of the sandbox's host user that hands
//! the agent container and the CI controller their tokens over Unix
//! sockets, so no token sits in a file.
//!
//! The broker reads each [`Item`] from the user's Secret Service keyring
//! ([`keyring`]), keeps it in memory for a while ([`cache`]), and answers one
//! typed request per connection ([`wire`]). It listens on one socket per
//! [`peer::Door`]: the agent's, bound into the container, and the CI
//! controller's, which only the `coda-ci` user can reach. The kernel's peer
//! credentials, mapped through the container's user namespace ([`peer`]),
//! decide who is asking, and each door answers only its own callers. Every
//! request leaves one audit line that never holds a secret ([`audit`]).

pub mod audit;
pub mod cache;
pub mod client;
pub mod commands;
pub mod core;
pub mod error;
pub mod keyring;
pub mod limit;
pub mod peer;
pub mod serve;
pub mod wire;

#[cfg(test)]
mod tests;

use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

/// A secret the keyring holds, each under the attributes
/// `service=coda-sandbox token=<name>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Item {
    /// The fine-grained GitHub token for git and gh in the container.
    GithubAgent,
    /// The read-only Doppler service token for the container's doppler.
    Doppler,
    /// The fine-grained GitHub token the CI controller registers runners
    /// and reads the job queue with. It never enters the container.
    GithubCi,
    /// A throwaway item `broker self-test` writes, reads and deletes.
    #[value(skip)]
    #[serde(rename = "coda-sandbox-selftest")]
    SelfTest,
}

impl Item {
    /// The items the agent's door serves, in the order reports list them.
    pub const AGENT: [Self; 2] = [Self::GithubAgent, Self::Doppler];

    /// The value of the keyring item's `token` attribute.
    pub fn name(self) -> &'static str {
        match self {
            Self::GithubAgent => "github-agent",
            Self::Doppler => "doppler",
            Self::GithubCi => "github-ci",
            Self::SelfTest => "coda-sandbox-selftest",
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Where a served secret came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Source {
    Keyring,
    /// `gh auth token`: the user's general token, which reaches every
    /// repository the user can. Used only when `gh_fallback` is on.
    GhCli,
}

impl Source {
    pub fn name(self) -> &'static str {
        match self {
            Self::Keyring => "keyring",
            Self::GhCli => "gh-cli",
        }
    }
}

/// A token. `Debug` hides it; only [`Secret::expose`] and the wire's
/// serialization reveal it. Every token is printable ASCII with no spaces,
/// as GitHub's and Doppler's are, so one can never add a line to git's
/// credential answer or the wrappers' environment.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Secret(String);

/// Why some text is not a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SecretError {
    #[error("the token is empty")]
    Empty,
    #[error(
        "the token has a space, control or non-ASCII character at byte {at}; a token is printable ASCII"
    )]
    NotPrintable { at: usize },
}

impl Secret {
    /// Surrounding whitespace, such as the newline after a pasted token,
    /// is dropped; anything inside must be printable ASCII.
    pub fn new(text: &str) -> Result<Self, SecretError> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Err(SecretError::Empty);
        }
        if let Some(at) = trimmed.bytes().position(|byte| !byte.is_ascii_graphic()) {
            return Err(SecretError::NotPrintable { at });
        }
        Ok(Self(trimmed.to_owned()))
    }

    pub fn expose(&self) -> &str {
        &self.0
    }

    /// The first 8 hex digits of the secret's SHA-256, so the audit log
    /// shows a rotation without holding the token.
    pub fn fingerprint(&self) -> String {
        let digest = Sha256::digest(self.0.as_bytes());
        digest[..4]
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }
}

impl TryFrom<String> for Secret {
    type Error = SecretError;

    fn try_from(text: String) -> Result<Self, SecretError> {
        Self::new(&text)
    }
}

impl From<Secret> for String {
    fn from(secret: Secret) -> Self {
        secret.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(..)")
    }
}
