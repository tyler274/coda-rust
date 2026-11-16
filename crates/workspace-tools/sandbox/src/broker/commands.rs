//! The host's `broker` subcommands: serve, store, mint, delete and self-test.

use std::io::{BufRead as _, IsTerminal as _, Write as _};
use std::sync::Arc;

use super::audit::Record;
use super::core::Broker;
use super::error::BrokerError;
use super::keyring::{GhCli, Keyring, KeyringError, SecretTool};
use super::peer::{Door, PeerMap};
use super::{Item, Secret, client, serve};
use crate::config::{Config, ModelProxy};
use crate::model_proxy::{Admin, ClientView};

/// Runs the broker on the config's sockets until a listener fails.
pub fn serve(config: &Config) -> Result<(), BrokerError> {
    // No core dump can write a cached token to disk, and other processes
    // of the same user cannot ptrace the broker or read its memory.
    rustix::process::set_dumpable_behavior(rustix::process::DumpableBehavior::NotDumpable)
        .map_err(|errno| BrokerError::NotDumpable(errno.into()))?;
    let ci = config
        .broker
        .ci
        .as_ref()
        .map(|ci| uid_of(&ci.user).map(|uid| (ci, uid)))
        .transpose()?;
    let peers = PeerMap {
        agent_uid: config.container.agent_uid,
        owner_uid: rustix::process::getuid().as_raw(),
        ci_uid: ci.map(|(_, uid)| uid),
    };
    let broker = Arc::new(Broker::new(
        peers,
        Arc::new(SecretTool::default()),
        Arc::new(GhCli::default()),
        config.broker.gh_fallback,
    ));
    let mut doors = vec![(Door::Agent, config.broker.socket.clone())];
    if let Some((ci, _)) = ci {
        doors.push((Door::Ci, ci.socket.clone()));
    }
    // Every socket is bound before any is served, so a broker that cannot
    // open one of its doors serves none.
    let listeners = doors
        .into_iter()
        .map(|(door, socket)| serve::bind(door, &socket).map(|listener| (listener, socket)))
        .collect::<Result<Vec<_>, _>>()?;
    serve::reload_on_sighup(&broker)?;
    println!(
        "broker listening on {} for agent uid {} (gh fallback {:?})",
        config.broker.socket.display(),
        peers.agent_uid,
        config.broker.gh_fallback
    );
    if let Some((ci, uid)) = ci {
        println!(
            "broker listening on {} for CI user {} uid {uid}",
            ci.socket.display(),
            ci.user
        );
    }
    let audit: Arc<dyn Fn(&Record) + Send + Sync> = Arc::new(|record| println!("{record}"));
    // The first door to fail ends the broker, and systemd restarts it with
    // all of them.
    let (failed, failure) = std::sync::mpsc::channel();
    for (listener, socket) in listeners {
        let broker = Arc::clone(&broker);
        let audit = Arc::clone(&audit);
        let failed = failed.clone();
        std::thread::spawn(move || {
            let source = serve::serve(&listener, &broker, &audit);
            // The receiver outlives every sender, so this send lands.
            let _ = failed.send(BrokerError::Listen { socket, source });
        });
    }
    drop(failed);
    Err(failure
        .recv()
        .expect("a door's thread sends its failure before it ends"))
}

fn uid_of(user: &str) -> Result<u32, BrokerError> {
    let found = nix::unistd::User::from_name(user).map_err(|errno| BrokerError::UserLookup {
        user: user.to_owned(),
        source: errno.into(),
    })?;
    match found {
        Some(found) => Ok(found.uid.as_raw()),
        None => Err(BrokerError::NoSuchUser {
            user: user.to_owned(),
        }),
    }
}

/// What `store` did besides writing the keyring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stored {
    /// The running broker dropped its cache and reads the new token next.
    Reloaded,
    /// No broker is listening; it reads the keyring when it starts.
    BrokerNotRunning,
}

/// Reads a secret from standard input, without echo on a terminal, and
/// writes it to the keyring as `item`.
pub fn store(config: &Config, item: Item) -> Result<Stored, BrokerError> {
    let secret = read_secret(item)?;
    SecretTool::default()
        .store(item, &secret)
        .map_err(|source| BrokerError::Keyring {
            op: "store",
            item,
            source,
        })?;
    reload(config)
}

/// What `mint model-proxy` did.
#[derive(Debug, PartialEq, Eq)]
pub struct Minted {
    /// The new key's client at the proxy.
    pub client: ClientView,
    pub stored: Stored,
    /// The client of the key it replaced, now revoked.
    pub revoked: Option<ClientView>,
}

/// Mints a model-proxy key labelled with the container's name from the
/// proxy's loopback admin route, stores it in `keyring` without showing
/// it, and revokes the client of the key it replaces. The old key is
/// checked before anything is minted, so a proxy that cannot answer
/// changes nothing.
pub fn mint_model_proxy(
    config: &Config,
    proxy: &ModelProxy,
    keyring: &dyn Keyring,
) -> Result<Minted, BrokerError> {
    let item = Item::ModelProxy;
    let keyring_error = |op| move |source| BrokerError::Keyring { op, item, source };
    let admin = Admin::new(proxy);
    // A stored value that is not a token names no client, so there is
    // nothing to revoke and the new key replaces it.
    let previous = match keyring.lookup(item) {
        Ok(Some(key)) => admin.client_of(&key)?,
        Ok(None) | Err(KeyringError::NotAToken { .. }) => None,
        Err(source) => return Err(keyring_error("read")(source)),
    };
    let (client, key) = admin.create(&config.container.name)?;
    keyring.store(item, &key).map_err(keyring_error("store"))?;
    let stored = reload(config)?;
    let revoked = match previous {
        Some(old) if old.id != client.id => {
            admin
                .revoke(&old.id)
                .map_err(|source| BrokerError::PreviousNotRevoked {
                    label: old.label.clone(),
                    id: old.id.clone(),
                    source: Box::new(source),
                })?;
            Some(old)
        }
        Some(_) | None => None,
    };
    Ok(Minted {
        client,
        stored,
        revoked,
    })
}

pub fn delete(config: &Config, item: Item) -> Result<Stored, BrokerError> {
    SecretTool::default()
        .delete(item)
        .map_err(|source| BrokerError::Keyring {
            op: "delete",
            item,
            source,
        })?;
    reload(config)
}

fn reload(config: &Config) -> Result<Stored, BrokerError> {
    match client::reload(&config.broker.socket) {
        Ok(()) => Ok(Stored::Reloaded),
        Err(BrokerError::Connect { .. }) => Ok(Stored::BrokerNotRunning),
        Err(error) => Err(error),
    }
}

/// Writes, reads back and deletes a throwaway item, proving the keyring
/// round trip without touching a real token.
pub fn self_test(keyring: &dyn Keyring) -> Result<String, BrokerError> {
    let item = Item::SelfTest;
    let keyring_error = |op| move |source| BrokerError::Keyring { op, item, source };
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let secret = Secret::new(&format!("selftest-{nanos:x}")).expect("printable ASCII");
    keyring
        .store(item, &secret)
        .map_err(keyring_error("store"))?;
    let read = keyring.lookup(item).map_err(keyring_error("read"))?;
    let matched = read.as_ref() == Some(&secret);
    keyring.delete(item).map_err(keyring_error("delete"))?;
    if !matched {
        return Err(BrokerError::SelfTestMismatch);
    }
    if keyring
        .lookup(item)
        .map_err(keyring_error("read"))?
        .is_some()
    {
        return Err(BrokerError::SelfTestNotDeleted { item });
    }
    Ok(format!(
        "keyring self-test passed: stored, read back (sha256 {}) and deleted {item}\n",
        secret.fingerprint()
    ))
}

fn read_secret(item: Item) -> Result<Secret, BrokerError> {
    let stdin = std::io::stdin();
    let text = if stdin.is_terminal() {
        eprint!("{item} token (input hidden): ");
        let _ = std::io::stderr().flush();
        let line = without_echo(&stdin, || {
            let mut line = String::new();
            stdin.lock().read_line(&mut line).map(|_| line)
        });
        eprintln!();
        line
    } else {
        std::io::read_to_string(stdin.lock())
    }
    .map_err(BrokerError::ReadSecret)?;
    Secret::new(&text).map_err(BrokerError::NotAToken)
}

/// Runs `read` with the terminal's echo off, restoring it afterwards.
fn without_echo<T>(
    terminal: &std::io::Stdin,
    read: impl FnOnce() -> std::io::Result<T>,
) -> std::io::Result<T> {
    use rustix::termios::{LocalModes, OptionalActions, tcgetattr, tcsetattr};
    let saved = tcgetattr(terminal)?;
    let mut quiet = saved.clone();
    quiet.local_modes.remove(LocalModes::ECHO);
    tcsetattr(terminal, OptionalActions::Now, &quiet)?;
    let result = read();
    tcsetattr(terminal, OptionalActions::Now, &saved)?;
    result
}
