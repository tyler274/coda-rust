//! The controller's GitHub token, asked of the token broker at the start of
//! each pass. The token lives in the host user's keyring and reaches the
//! controller over the broker's CI socket; it is in no file.
//!
//! A broker that cannot serve the token leaves the pass with the local
//! queue. A broker that refuses, or answers something this binary cannot
//! read, ends the pass with an error: that is a misconfiguration, not a
//! missing token.

use std::path::Path;

use crate::broker::Secret;
use crate::broker::client;
use crate::broker::error::BrokerError;

/// Whether this pass can talk to GitHub.
#[derive(Debug)]
pub enum Access {
    Granted(Secret),
    LocalOnly(NoToken),
}

/// Why a pass has no GitHub token.
#[derive(Debug, thiserror::Error)]
pub enum NoToken {
    /// The unit names no broker socket, as on a host that runs only local
    /// requests.
    #[error("CODA_CI_BROKER_SOCKET is not set")]
    NoSocket,
    /// No broker listens: the host user's session has not started one.
    #[error("the token broker is not running")]
    NotRunning(#[source] BrokerError),
    /// The broker answered, and has no token to give now.
    #[error("the token broker has no token to serve")]
    NotServed(#[source] BrokerError),
}

/// Asks the broker at `socket` for the controller's token.
pub fn fetch(socket: Option<&Path>) -> Result<Access, BrokerError> {
    let Some(socket) = socket else {
        return Ok(Access::LocalOnly(NoToken::NoSocket));
    };
    match client::ci_token(socket) {
        Ok(secret) => Ok(Access::Granted(secret)),
        Err(error @ BrokerError::Connect { .. }) => {
            Ok(Access::LocalOnly(NoToken::NotRunning(error)))
        }
        Err(error @ BrokerError::Unavailable { .. }) => {
            Ok(Access::LocalOnly(NoToken::NotServed(error)))
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::broker::Item;
    use crate::broker::audit::Record;
    use crate::broker::core::Broker;
    use crate::broker::keyring::{Locked, Memory};
    use crate::broker::peer::{Door, PeerMap};
    use crate::broker::serve;
    use crate::broker::wire::{Denied, GhFallback, Unavailable};

    /// A broker on a CI socket in a temporary directory, with this test
    /// process as the CI user unless `ci_uid` says otherwise.
    fn broker(memory: &Arc<Memory>, ci_uid: u32) -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("temp dir");
        let peers = PeerMap {
            agent_uid: ci_uid.wrapping_add(1),
            owner_uid: ci_uid.wrapping_add(2),
            ci_uid: Some(ci_uid),
        };
        let broker = Arc::new(Broker::new(
            peers,
            memory.clone(),
            memory.clone(),
            GhFallback::Off,
        ));
        let listener = serve::bind(Door::Ci, &dir.path().join("broker.sock")).expect("binds");
        let audit: Arc<dyn Fn(&Record) + Send + Sync> = Arc::new(|_| {});
        std::thread::spawn(move || serve::serve(&listener, &broker, &audit));
        dir
    }

    fn me() -> u32 {
        rustix::process::getuid().as_raw()
    }

    #[test]
    fn a_stored_token_is_granted() {
        let memory = Arc::new(Memory::with([(Item::GithubCi, "ci-token")]));
        let dir = broker(&memory, me());
        let access = fetch(Some(&dir.path().join("broker.sock"))).expect("fetches");
        let Access::Granted(secret) = access else {
            panic!("expected a token, got {access:?}");
        };
        assert_eq!(secret.expose(), "ci-token");
    }

    #[test]
    fn no_socket_no_broker_and_no_token_each_mean_local_only() {
        assert!(matches!(
            fetch(None).expect("no socket"),
            Access::LocalOnly(NoToken::NoSocket)
        ));

        let empty = tempfile::tempdir().expect("temp dir");
        assert!(matches!(
            fetch(Some(&empty.path().join("broker.sock"))).expect("no broker"),
            Access::LocalOnly(NoToken::NotRunning(BrokerError::Connect { .. }))
        ));

        let memory = Arc::new(Memory::default());
        let dir = broker(&memory, me());
        let socket = dir.path().join("broker.sock");
        assert!(matches!(
            fetch(Some(&socket)).expect("not stored"),
            Access::LocalOnly(NoToken::NotServed(BrokerError::Unavailable {
                item: Item::GithubCi,
                reason: Unavailable::NotStored,
            }))
        ));
        memory.set_locked(Locked::Locked);
        assert!(matches!(
            fetch(Some(&socket)).expect("locked"),
            Access::LocalOnly(NoToken::NotServed(BrokerError::Unavailable {
                reason: Unavailable::KeyringLocked,
                ..
            }))
        ));
    }

    #[test]
    fn a_refusal_fails_the_pass_instead_of_reading_as_no_token() {
        let memory = Arc::new(Memory::with([(Item::GithubCi, "ci-token")]));
        let dir = broker(&memory, me().wrapping_add(7));
        let error = fetch(Some(&dir.path().join("broker.sock"))).expect_err("refused");
        assert!(
            matches!(error, BrokerError::Denied(Denied::WrongPeer)),
            "{error:?}"
        );
    }
}
