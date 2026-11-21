//! `coda-sandbox credential <op>`: git's credential helper protocol in the
//! container, answered from the token broker.
//!
//! git writes `key=value` lines ending in a blank line. For `get`, a query
//! for `https://github.com` gets `username=x-access-token` and the token;
//! anything else gets no output, so git tries its next helper. `store` and
//! `erase` are acknowledged and ignored: the broker owns the token, and git
//! must neither cache nor wipe it.

use std::io::{BufRead, Write};
use std::path::Path;

use crate::broker::Secret;
use crate::broker::client::{self, Want};
use crate::error::{Error, chain};

/// The operation git asks a helper for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Operation {
    Get,
    Store,
    Erase,
}

/// The fields of git's query the helper reads. Other keys (`path`,
/// `wwwauth[]`, `capability[]`, ...) are read and ignored, as the protocol
/// requires of a helper that does not use them.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Query {
    pub protocol: Option<String>,
    pub host: Option<String>,
    pub username: Option<String>,
}

/// Whether the broker's token answers a query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// HTTPS to github.com.
    GitHub,
    /// Another protocol or host; no output, so git moves on.
    Other,
}

impl Query {
    /// Reads git's query up to the blank line or end of input.
    pub fn read(input: impl BufRead) -> std::io::Result<Self> {
        let mut query = Self::default();
        for line in input.lines() {
            let line = line?;
            if line.is_empty() {
                break;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let slot = match key {
                "protocol" => &mut query.protocol,
                "host" => &mut query.host,
                "username" => &mut query.username,
                _ => continue,
            };
            *slot = Some(value.to_owned());
        }
        Ok(query)
    }

    pub fn scope(&self) -> Scope {
        let https = self.protocol.as_deref() == Some("https");
        let github = self.host.as_deref() == Some("github.com");
        let our_user = self.username.as_deref().is_none_or(|name| name == USERNAME);
        if https && github && our_user {
            Scope::GitHub
        } else {
            Scope::Other
        }
    }
}

/// The username GitHub expects with a token over HTTPS.
pub const USERNAME: &str = "x-access-token";

/// git's answer for a granted token.
pub fn answer(secret: &Secret) -> String {
    format!("username={USERNAME}\npassword={}\n", secret.expose())
}

/// Runs one helper operation. A broker that cannot serve the token leaves
/// one line on stderr and no answer, so git reports its own authentication
/// failure rather than prompting; that is not an error of the helper.
pub fn run(
    operation: Operation,
    socket: &Path,
    input: impl BufRead,
    mut output: impl Write,
) -> Result<(), Error> {
    let query = Query::read(input).map_err(Error::CredentialQuery)?;
    if operation != Operation::Get || query.scope() == Scope::Other {
        return Ok(());
    }
    match client::secret(socket, Want::GitCredential) {
        Ok(secret) => output
            .write_all(answer(&secret).as_bytes())
            .map_err(Error::Output),
        Err(error) => {
            eprintln!("coda-sandbox: {}", chain(&error));
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(text: &str) -> Query {
        Query::read(text.as_bytes()).expect("reads")
    }

    #[test]
    fn github_https_is_ours_with_unknown_keys_ignored() {
        let q = query(
            "capability[]=authtype\nprotocol=https\nhost=github.com\npath=tyler274/coda-rust.git\nwwwauth[]=Basic realm=\"GitHub\"\n\n",
        );
        assert_eq!(q.protocol.as_deref(), Some("https"));
        assert_eq!(q.host.as_deref(), Some("github.com"));
        assert_eq!(q.scope(), Scope::GitHub);
    }

    #[test]
    fn other_hosts_protocols_and_users_are_not() {
        for text in [
            "protocol=https\nhost=gitlab.com\n",
            "protocol=http\nhost=github.com\n",
            "protocol=https\nhost=github.com:8443\n",
            "protocol=https\nhost=github.com\nusername=someone\n",
            "host=github.com\n",
            "",
        ] {
            assert_eq!(query(text).scope(), Scope::Other, "{text:?}");
        }
        let ours = query("protocol=https\nhost=github.com\nusername=x-access-token\n");
        assert_eq!(ours.scope(), Scope::GitHub);
    }

    #[test]
    fn reading_stops_at_the_blank_line() {
        let q = query("protocol=https\n\nhost=github.com\n");
        assert_eq!(q.host, None);
    }

    #[test]
    fn answer_is_git_lines() {
        let secret = Secret::new("tok\n").expect("non-empty");
        assert_eq!(answer(&secret), "username=x-access-token\npassword=tok\n");
    }
}
