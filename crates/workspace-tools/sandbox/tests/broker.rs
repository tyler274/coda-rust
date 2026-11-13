//! The broker on a real Unix socket with the in-memory keyring: the client,
//! the git credential helper and the audit line, end to end.

use std::sync::{Arc, Mutex};

use coda_sandbox::broker::audit::Record;
use coda_sandbox::broker::client::{self, Want};
use coda_sandbox::broker::core::Broker;
use coda_sandbox::broker::error::BrokerError;
use coda_sandbox::broker::keyring::{Locked, Memory};
use coda_sandbox::broker::peer::{Door, PeerMap};
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
    let listener = serve::bind(Door::Agent, &dir.path().join("broker.sock")).expect("binds");
    std::thread::spawn(move || serve::serve(&listener, &broker, &audit));
    Running { dir, memory, lines }
}

fn as_agent(uid: u32) -> PeerMap {
    PeerMap {
        agent_uid: uid,
        owner_uid: uid.wrapping_add(1),
        ci_uid: None,
    }
}

fn as_owner(uid: u32) -> PeerMap {
    PeerMap {
        agent_uid: uid.wrapping_add(1),
        owner_uid: uid,
        ci_uid: None,
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
        audit.contains("git_credential granted socket=agent peer=agent"),
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
    assert_eq!(
        report
            .items
            .iter()
            .map(|entry| entry.item)
            .collect::<Vec<_>>(),
        Item::AGENT
    );
    assert!(!report.fallback_used);
    let error = client::secret(&broker.socket(), Want::Gh).expect_err("denied");
    assert!(matches!(error, BrokerError::Denied(Denied::WrongPeer)));
    client::reload(&broker.socket()).expect("reload");
    let audit = broker.audit();
    assert!(
        audit.contains("gh_token denied socket=agent peer=owner"),
        "{audit}"
    );
}

#[test]
fn no_broker_is_a_connect_error() {
    let dir = tempfile::tempdir().expect("temp dir");
    let error = client::ping(&dir.path().join("none.sock")).expect_err("nothing listens");
    assert!(matches!(error, BrokerError::Connect { .. }));
}

/// Raw connections that send nothing, as a stalled or hostile client would.
fn hold(socket: &std::path::Path, count: usize) -> Vec<std::os::unix::net::UnixStream> {
    (0..count)
        .map(|_| std::os::unix::net::UnixStream::connect(socket).expect("connects"))
        .collect()
}

#[test]
fn a_full_door_refuses_more_connections_and_recovers_at_the_deadline() {
    let broker = start(as_agent);
    let held = hold(&broker.socket(), serve::MAX_CONNECTIONS);
    // The accept loop takes connections in order, so the next one finds
    // every slot held.
    let error = client::secret(&broker.socket(), Want::Gh).expect_err("busy");
    assert!(
        matches!(error, BrokerError::Denied(Denied::Busy)),
        "{error:?}"
    );
    // Silent clients are cut off at the deadline, without closing. Each one
    // spent a request from the caller's rate limit, so the door is open again
    // before the limit has refilled.
    std::thread::sleep(serve::REQUEST_DEADLINE + std::time::Duration::from_millis(500));
    let give_up = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let secret = loop {
        match client::secret(&broker.socket(), Want::Gh) {
            Ok(secret) => break secret,
            Err(BrokerError::Denied(Denied::RateLimited)) => {
                assert!(std::time::Instant::now() < give_up, "never served again");
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            Err(error) => panic!("the door stayed shut: {error:?}"),
        }
    };
    assert_eq!(secret.expose(), "ghp-test-secret");
    drop(held);
}

#[test]
fn a_client_that_trickles_bytes_is_cut_off_at_the_deadline() {
    use std::io::{Read as _, Write as _};
    let broker = start(as_agent);
    let mut stream = std::os::unix::net::UnixStream::connect(broker.socket()).expect("connects");
    let started = std::time::Instant::now();
    let writer = stream.try_clone().expect("clones");
    // One byte every 300 ms would reset a per-read timeout forever.
    std::thread::spawn(move || {
        let mut writer = writer;
        for _ in 0..40 {
            if writer.write_all(b" ").is_err() {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    });
    let mut reply = String::new();
    stream.read_to_string(&mut reply).expect("reads the reply");
    let took = started.elapsed();
    assert!(reply.contains(r#""reason":"malformed""#), "{reply}");
    assert!(
        took < serve::REQUEST_DEADLINE + std::time::Duration::from_secs(2),
        "answered after {took:?}"
    );
    assert!(
        broker.audit().contains("malformed denied"),
        "{}",
        broker.audit()
    );
}
