//! One journal line per request. It names the caller, the request kind,
//! the outcome and, for a granted secret, its source and fingerprint. It
//! never holds a secret or a request body.

use std::fmt::{self, Write as _};

use super::Source;
use super::peer::{Peer, Principal};

/// What a request led to, as the audit line records it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub peer: Peer,
    pub comm: String,
    pub principal: Principal,
    /// The request's `kind`, or `malformed` when it did not parse.
    pub kind: &'static str,
    pub outcome: &'static str,
    /// The reason of an unavailable or denied reply.
    pub reason: Option<String>,
    pub served: Option<Served>,
}

/// The secret a granted request received, without the secret.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Served {
    pub source: Source,
    pub fingerprint: String,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut line = format!(
            "broker {} {} peer={} uid={} pid={} comm={:?}",
            self.kind,
            self.outcome,
            self.principal.name(),
            self.peer.uid,
            self.peer.pid,
            self.comm,
        );
        if let Some(reason) = &self.reason {
            let _ = write!(line, " reason={reason}");
        }
        if let Some(served) = &self.served {
            let _ = write!(
                line,
                " source={} sha256={}",
                served.source.name(),
                served.fingerprint
            );
        }
        f.write_str(&line)
    }
}
