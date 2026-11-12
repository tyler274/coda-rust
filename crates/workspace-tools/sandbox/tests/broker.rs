//! The broker on a real Unix socket with the in-memory keyring: the client,
//! the git credential helper and the audit line, end to end.

use std::sync::{Arc, Mutex};

use coda_sandbox::broker::audit::Record;
use coda_sandbox::broker::client::{self, Want};
use coda_sandbox::broker::core::Broker;
use coda_sandbox::broker::error::BrokerError;
use coda_sandbox::broker::keyring::{Locked, Memory};
use coda_sandbox::broker::peer::PeerMap;
use coda_sandbox::broker::wire::{Denied, GhFallback, Unavailable};
use coda_sandbox::broker::{Item, serve};
use coda_sandbox::credential::{self, Operation};

struct Running {
    dir: tempfile::TempDir,
    memory: Arc<Memory>,
    lines: Arc<Mutex<Vec<String>>>,
}

impl Running {
    fn socket(&self) -> std::path::PathBuf {
        self.dir.path().join("broker.sock")
    }

    fn audit(&self) -> String {
        self.lines.lock().expect("audit lock").join("\n")
    }
}

/// A broker that treats this test process as `as_peer`.
fn start(as_peer: fn(u32) -> PeerMap) -> Running {
    let dir = tempfile::tempdir().expect("temp dir");
    let memory = Arc::new(Memory::with([(Item::GithubAgent, "ghp-test-secret")]));
    let peers = as_peer(rustix::process::getuid().as_raw());
    let broker = Arc::new(Broker::new(
        peers,
        memory.clone(),
        memory.clone(),
        GhFallback::Off,
    ));
    let lines = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&lines);
    let audit: Arc<dyn Fn(&Record) + Send + Sync> =
        Arc::new(move |record| sink.lock().expect("audit lock").push(record.to_string()));
    let listener = serve::bind(&dir.path().join("broker.sock")).expect("binds");
    std::thread::spawn(move || serve::serve(&listener, &broker, &audit));
    Running { dir, memory, lines }
}

fn as_agent(uid: u32) -> PeerMap {
    PeerMap {
        agent_uid: uid,
        owner_uid: uid.wrapping_add(1),
    }
}

fn as_owner(uid: u32) -> PeerMap {
    PeerMap {
        agent_uid: uid.wrapping_add(1),
        owner_uid: uid,
    }
}

fn git_get(socket: &std::path::Path, query: &str) -> String {
    let mut out = Vec::new();
    credential::run(Operation::Get, socket, query.as_bytes(), &mut out).expect("helper runs");
    String::from_utf8(out).expect("utf-8")
}

#[test]
fn agent_gets_a_git_credential_and_the_audit_line_has_no_secret() {
    let broker = start(as_agent);
    let answer = git_get(&broker.socket(), "protocol=https\nhost=github.com\n\n");
    assert_eq!(
        answer,
        "username=x-access-token\npassword=ghp-test-secret\n"
    );
    let secret = client::secret(&broker.socket(), Want::Gh).expect("gh token");
    assert_eq!(secret.expose(), "ghp-test-secret");
    let audit = broker.audit();
    assert!(
        audit.contains("git_credential granted peer=agent"),
        "{audit}"
    );
    assert!(audit.contains("gh_token granted"), "{audit}");
    assert!(!audit.contains("ghp-test-secret"), "{audit}");
}

#[test]
fn helper_answers_nothing_for_other_hosts_and_for_store_and_erase() {
    let broker = start(as_agent);
    assert_eq!(
        git_get(&broker.socket(), "protocol=https\nhost=gitlab.com\n"),
        ""
    );
    for operation in [Operation::Store, Operation::Erase] {
        let mut out = Vec::new();
        let input = "protocol=https\nhost=github.com\nusername=x\npassword=y\n";
        credential::run(operation, &broker.socket(), input.as_bytes(), &mut out)
            .expect("helper runs");
        assert!(out.is_empty());
    }
    assert!(broker.audit().is_empty(), "no request reached the broker");
}

#[test]
fn locked_keyring_gives_git_no_answer_and_the_client_a_typed_reason() {
    let broker = start(as_agent);
    broker.memory.set_locked(Locked::Locked);
    assert_eq!(
        git_get(&broker.socket(), "protocol=https\nhost=github.com\n"),
        ""
    );
    let error = client::secret(&broker.socket(), Want::Doppler).expect_err("locked");
    assert!(matches!(
        error,
        BrokerError::Unavailable {
            item: Item::Doppler,
            reason: Unavailable::KeyringLocked
        }
    ));
}

#[test]
fn the_owner_pings_but_gets_no_secret() {
    let broker = start(as_owner);
    let report = client::ping(&broker.socket()).expect("ping");
    assert_eq!(report.items.len(), 2);
    assert!(!report.fallback_used);
    let error = client::secret(&broker.socket(), Want::Gh).expect_err("denied");
    assert!(matches!(error, BrokerError::Denied(Denied::WrongPeer)));
    client::reload(&broker.socket()).expect("reload");
    let audit = broker.audit();
    assert!(audit.contains("gh_token denied peer=owner"), "{audit}");
}

#[test]
fn no_broker_is_a_connect_error() {
    let dir = tempfile::tempdir().expect("temp dir");
    let error = client::ping(&dir.path().join("none.sock")).expect_err("nothing listens");
    assert!(matches!(error, BrokerError::Connect { .. }));
}
