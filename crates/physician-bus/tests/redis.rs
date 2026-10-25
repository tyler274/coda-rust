//! Real disposable Valkey, no shared service, credentials, or fixed ports.
//! Run from the Nix dev shell: cargo test -p physician-bus --test redis --locked
//!
//! Without `valkey-server` the suite returns. The Postgres CI job sets
//! `CODA_REQUIRE_POSTGRES_TESTS` and fails closed if the binary is missing.
use fred::prelude::*;
use physician_bus::{Bus, CHANNEL, Event, RedisBus, Subscription, Timing};
use std::{
    net::{TcpListener, TcpStream},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

const PASSWORD: &str = "synthetic-bus-test-password";
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct Server {
    child: Option<Child>,
    port: u16,
    directory: PathBuf,
    tls: bool,
}

fn valkey_available() -> bool {
    let found = Command::new("valkey-server")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    if found {
        return true;
    }
    assert!(
        std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
        "valkey-server from the Nix dev shell is required for the physician bus proof"
    );
    false
}

impl Server {
    async fn start(tls: bool) -> Option<Self> {
        if !valkey_available() {
            return None;
        }
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let directory = std::env::temp_dir().join(format!(
            "physician-bus-{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&directory).unwrap();
        let mut server = Self {
            child: None,
            port,
            directory,
            tls,
        };
        if tls {
            // A CA and a separate leaf it signs, as Memorystore presents them;
            // webpki rejects a CA certificate used as the server end entity.
            let dir = &server.directory;
            let openssl = |args: &[&str]| {
                let status = Command::new("openssl")
                    .args(args)
                    .current_dir(dir)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .expect("openssl in Nix shell");
                assert!(status.success(), "openssl {args:?}");
            };
            openssl(&[
                "req",
                "-x509",
                "-newkey",
                "rsa:2048",
                "-nodes",
                "-days",
                "1",
                "-subj",
                "/CN=physician-bus-test-ca",
                "-addext",
                "basicConstraints=critical,CA:TRUE",
                "-addext",
                "keyUsage=critical,keyCertSign,cRLSign",
                "-keyout",
                "ca-key.pem",
                "-out",
                "ca.pem",
            ]);
            openssl(&[
                "req",
                "-newkey",
                "rsa:2048",
                "-nodes",
                "-subj",
                "/CN=localhost",
                "-keyout",
                "key.pem",
                "-out",
                "server.csr",
            ]);
            std::fs::write(
                dir.join("server.ext"),
                "subjectAltName=DNS:localhost,IP:127.0.0.1\nbasicConstraints=critical,CA:FALSE\nextendedKeyUsage=serverAuth\n",
            )
            .unwrap();
            openssl(&[
                "x509",
                "-req",
                "-in",
                "server.csr",
                "-CA",
                "ca.pem",
                "-CAkey",
                "ca-key.pem",
                "-CAcreateserial",
                "-days",
                "1",
                "-extfile",
                "server.ext",
                "-out",
                "server.pem",
            ]);
        }
        drop(listener);
        for _ in 0..5 {
            if server.spawn().await {
                return Some(server);
            }
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            server.port = listener.local_addr().unwrap().port();
        }
        panic!("Valkey could not bind a free port");
    }

    /// Start Valkey on `port`. False when it exited, as when another test
    /// took the port between picking it and binding.
    async fn spawn(&mut self) -> bool {
        let mut command = Command::new("valkey-server");
        command
            .args([
                "--bind",
                "127.0.0.1",
                "--save",
                "",
                "--appendonly",
                "no",
                "--requirepass",
                PASSWORD,
                "--dir",
            ])
            .arg(&self.directory)
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        if self.tls {
            command
                .args(["--port", "0", "--tls-port"])
                .arg(self.port.to_string())
                .arg("--tls-cert-file")
                .arg(self.directory.join("server.pem"))
                .arg("--tls-key-file")
                .arg(self.directory.join("key.pem"))
                .arg("--tls-ca-cert-file")
                .arg(self.directory.join("ca.pem"))
                .args(["--tls-auth-clients", "no"]);
        } else {
            command.arg("--port").arg(self.port.to_string());
        }
        self.child = Some(command.spawn().expect("valkey-server in Nix shell"));
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if self.child.as_mut().unwrap().try_wait().unwrap().is_some() {
                self.child = None;
                return false;
            }
            if TcpStream::connect(("127.0.0.1", self.port)).is_ok_and(|stream| self.ours(stream)) {
                return true;
            }
            assert!(Instant::now() < deadline, "Valkey startup timeout");
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    async fn restart(&mut self) {
        assert!(self.spawn().await, "Valkey exited during startup");
    }

    /// The port was free when picked, but another test's proxy or socket can
    /// take it before Valkey binds. Accept only our own listener: plaintext
    /// Valkey must report this server's directory. A TLS port is ours when
    /// the child still runs a moment after the accept (a failed bind exits).
    fn ours(&mut self, mut stream: TcpStream) -> bool {
        use std::io::{Read, Write};
        if self.tls {
            std::thread::sleep(Duration::from_millis(100));
            return self.child.as_mut().unwrap().try_wait().unwrap().is_none();
        }
        let _ = stream.set_read_timeout(Some(Duration::from_secs(1)));
        if stream
            .write_all(format!("AUTH default {PASSWORD}\r\nCONFIG GET dir\r\n").as_bytes())
            .is_err()
        {
            return false;
        }
        // Valkey reports its working directory, which resolves symlinks.
        let directory = std::fs::canonicalize(&self.directory)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let mut reply = Vec::new();
        let mut chunk = [0u8; 512];
        while let Ok(read) = stream.read(&mut chunk) {
            if read == 0 {
                break;
            }
            reply.extend_from_slice(&chunk[..read]);
            if String::from_utf8_lossy(&reply).contains(&directory) {
                return true;
            }
        }
        false
    }

    fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    /// `STOP` freezes Valkey with its sockets open; `CONT` resumes it.
    fn signal(&self, signal: &str) {
        let pid = self.child.as_ref().expect("Valkey running").id();
        let status = Command::new("kill")
            .arg(format!("-{signal}"))
            .arg(pid.to_string())
            .status()
            .expect("kill in Nix shell");
        assert!(status.success(), "kill -{signal}");
    }

    fn url(&self) -> String {
        format!(
            "{}://default:{PASSWORD}@127.0.0.1:{}",
            if self.tls { "rediss" } else { "redis" },
            self.port
        )
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop();
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

async fn receive(receiver: &mut Subscription, expected: &Event) {
    tokio::time::timeout(Duration::from_secs(12), async {
        loop {
            let event = receiver.recv().await.unwrap();
            if &event == expected {
                break;
            }
            assert_eq!(event, Event::Resync, "unexpected workspace event");
        }
    })
    .await
    .expect("event delivery timeout");
}

async fn wait_live(bus: &RedisBus, expected: bool) {
    tokio::time::timeout(Duration::from_secs(12), async {
        while bus.live() != expected {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("bus liveness timeout");
}

#[tokio::test]
async fn independent_instances_auth_fanout_restart_resync_and_invalid_payload() {
    let Some(mut server) = Server::start(false).await else {
        return;
    };
    let bad_url = server
        .url()
        .replace(PASSWORD, "incorrect-synthetic-password");
    let error = RedisBus::connect(&bad_url, None)
        .await
        .err()
        .expect("AUTH must fail");
    assert!(!error.contains("incorrect-synthetic-password"));
    let first = RedisBus::connect(&server.url(), None).await.unwrap();
    let second = RedisBus::connect(&server.url(), None).await.unwrap();
    let mut a = first.subscribe();
    let mut b = second.subscribe();
    let changed = Event::WorkspaceChanged {
        workspace_id: "synthetic-workspace".into(),
    };
    first.publish(changed.clone()).await.unwrap();
    receive(&mut a, &changed).await;
    receive(&mut b, &changed).await;
    let settled = Event::WorkspaceSettled {
        workspace_id: "synthetic-workspace".into(),
    };
    second.publish(settled.clone()).await.unwrap();
    receive(&mut a, &settled).await;
    receive(&mut b, &settled).await;

    server.stop();
    wait_live(&first, false).await;
    wait_live(&second, false).await;
    receive(&mut a, &Event::Resync).await;
    receive(&mut b, &Event::Resync).await;
    assert!(first.publish(changed.clone()).await.is_err());
    // Drain outage notifications: the following resync must cover recovery.
    while a.try_recv().is_ok() {}
    while b.try_recv().is_ok() {}
    server.restart().await;
    wait_live(&first, true).await;
    wait_live(&second, true).await;
    receive(&mut a, &Event::Resync).await;
    receive(&mut b, &Event::Resync).await;
    first.publish(changed.clone()).await.unwrap();
    receive(&mut b, &changed).await;
    second.publish(settled.clone()).await.unwrap();
    receive(&mut a, &changed).await;
    receive(&mut a, &settled).await;
    receive(&mut b, &settled).await;

    // Malformed wire data cannot turn into a workspace event. It invalidates
    // projections conservatively rather than silently losing an unknown event.
    let raw = Builder::from_config(Config::from_url(&server.url()).unwrap())
        .build()
        .unwrap();
    let connection = raw.init().await.unwrap();
    raw.publish::<i64, _, _>(CHANNEL, r#"{"version":99,"event":{"type":"resync"}}"#)
        .await
        .unwrap();
    receive(&mut a, &Event::Resync).await;
    receive(&mut b, &Event::Resync).await;
    // Duet channel traffic must never arrive at a physician subscriber.
    while b.try_recv().is_ok() {}
    raw.publish::<i64, _, _>(
        "duet:workspace-events:v1",
        r#"{"version":1,"event":{"type":"workspace_changed","workspace_id":"foreign"}}"#,
    )
    .await
    .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(150), b.recv())
            .await
            .is_err()
    );
    raw.quit().await.unwrap();
    connection.await.unwrap().unwrap();
}

/// Collect the resyncs that arrive until `quiet` passes with no event. Any
/// other event fails: nothing but the bus itself publishes in these tests.
async fn resyncs_until_quiet(events: &mut Subscription, quiet: Duration) -> usize {
    let mut resyncs = 0;
    while let Ok(event) = tokio::time::timeout(quiet, events.recv()).await {
        assert_eq!(event.unwrap(), Event::Resync, "unexpected bus event");
        resyncs += 1;
    }
    resyncs
}

/// Wait until the bus reports live, then for a quiet spell. fred reports the
/// initial connection on its reconnect channel too, so a new bus resyncs (and
/// resubscribes) just after connect; a fixed sleep raced that under load.
async fn settle(bus: &RedisBus, events: &mut Subscription) {
    tokio::time::timeout(Duration::from_secs(12), async {
        loop {
            wait_live(bus, true).await;
            resyncs_until_quiet(events, Duration::from_millis(200)).await;
            if bus.live() {
                break;
            }
        }
    })
    .await
    .expect("bus did not settle");
}

/// A TCP hop between the bus and Valkey that the test opens and closes, so
/// an outage starts and ends exactly when the test says. Closing severs every
/// forwarded connection. While closed, new connections are accepted and held
/// unanswered; opening forwards them, so fred's pending reconnect completes
/// without any refused attempt in between.
struct Proxy {
    port: u16,
    state: Arc<Mutex<ProxyState>>,
    accept: tokio::task::JoinHandle<()>,
}

#[derive(Default)]
struct ProxyState {
    open: bool,
    forwarded: Vec<tokio::task::JoinHandle<()>>,
    held: Vec<tokio::net::TcpStream>,
}

fn forward(client: tokio::net::TcpStream, upstream: u16) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut client = client;
        let Ok(mut server) = tokio::net::TcpStream::connect(("127.0.0.1", upstream)).await else {
            return;
        };
        let _ = tokio::io::copy_bidirectional(&mut client, &mut server).await;
    })
}

impl Proxy {
    async fn start(upstream: u16) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let state = Arc::new(Mutex::new(ProxyState {
            open: true,
            ..ProxyState::default()
        }));
        let accepting = state.clone();
        let accept = tokio::spawn(async move {
            while let Ok((client, _)) = listener.accept().await {
                let mut state = accepting.lock().unwrap();
                if state.open {
                    state.forwarded.push(forward(client, upstream));
                } else {
                    state.held.push(client);
                }
            }
        });
        Self {
            port,
            state,
            accept,
        }
    }

    fn close(&self) {
        let mut state = self.state.lock().unwrap();
        state.open = false;
        for task in state.forwarded.drain(..) {
            task.abort();
        }
    }

    fn open(&self, upstream: u16) {
        let mut state = self.state.lock().unwrap();
        state.open = true;
        let held = std::mem::take(&mut state.held);
        for client in held {
            state.forwarded.push(forward(client, upstream));
        }
    }

    fn held(&self) -> usize {
        self.state.lock().unwrap().held.len()
    }

    fn url(&self) -> String {
        format!("redis://default:{PASSWORD}@127.0.0.1:{}", self.port)
    }
}

impl Drop for Proxy {
    fn drop(&mut self) {
        self.accept.abort();
        for task in self.state.lock().unwrap().forwarded.drain(..) {
            task.abort();
        }
    }
}

/// An outage shorter than one heartbeat: only the reconnect path can cover
/// it. The heartbeat is 60 s, so it never ticks after connect, and reconnects
/// have no jitter. The proxy drops both connections and holds fred's
/// reconnects; an event published then reaches no subscriber. When the proxy
/// reopens, the bus must resync once on reconnect and again once SUBSCRIBE is
/// acknowledged: a refetch for the first can still miss events published
/// before the ACK.
#[tokio::test]
async fn fast_restart_within_heartbeat_resyncs_from_reconnect_path() {
    let Some(server) = Server::start(false).await else {
        return;
    };
    let proxy = Proxy::start(server.port).await;
    let timing = Timing {
        heartbeat: Duration::from_secs(60),
        command_timeout: Duration::from_secs(3),
        reconnect_min: Duration::from_millis(50),
        reconnect_max: Duration::from_millis(200),
        jitter: Duration::ZERO,
    };
    let bus = RedisBus::connect_with(&proxy.url(), None, timing)
        .await
        .unwrap();
    let mut events = bus.subscribe();
    settle(&bus, &mut events).await;
    let changed = Event::WorkspaceChanged {
        workspace_id: "synthetic-fast-restart-workspace".into(),
    };
    bus.publish(changed.clone()).await.unwrap();
    receive(&mut events, &changed).await;
    assert_eq!(
        resyncs_until_quiet(&mut events, Duration::from_millis(200)).await,
        0,
        "a healthy bus must not resync"
    );

    proxy.close();
    // Publisher and subscriber are both reconnecting and held at the proxy.
    tokio::time::timeout(Duration::from_secs(10), async {
        while proxy.held() < 2 || bus.live() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("fred must reconnect to the closed proxy");
    // The drop's own resync is behind us: the consumer refetched for it while
    // still unsubscribed. Nothing else can fire while the proxy holds fred.
    resyncs_until_quiet(&mut events, Duration::from_millis(200)).await;
    let raw = Builder::from_config(Config::from_url(&server.url()).unwrap())
        .build()
        .unwrap();
    let connection = raw.init().await.unwrap();
    let missed = r#"{"version":1,"event":{"type":"workspace_changed","workspace_id":"missed"}}"#;
    let receivers: i64 = raw.publish(CHANNEL, missed).await.unwrap();
    assert_eq!(
        receivers, 0,
        "the closed proxy must keep the bus unsubscribed"
    );
    raw.quit().await.unwrap();
    connection.await.unwrap().unwrap();

    proxy.open(server.port);
    let first = tokio::time::timeout(Duration::from_secs(10), events.recv())
        .await
        .expect("the reconnect path must resync without a heartbeat tick")
        .unwrap();
    assert_eq!(first, Event::Resync, "unexpected event across restart");
    let resyncs = 1 + resyncs_until_quiet(&mut events, Duration::from_millis(300)).await;
    assert!(
        resyncs >= 2,
        "the reconnect path must resync again after resubscribing, saw {resyncs}"
    );
    wait_live(&bus, true).await;
    bus.publish(changed.clone()).await.unwrap();
    receive(&mut events, &changed).await;
}

/// A server that stops answering but keeps its sockets open (frozen process,
/// half-open link) produces no fred error or reconnect. Only the heartbeat's
/// failing ping can report it.
#[tokio::test]
async fn frozen_server_resyncs_from_heartbeat_path() {
    let Some(server) = Server::start(false).await else {
        return;
    };
    // Production's 2 s heartbeat and 3 s ping timeout. fred's own reconnect
    // to a frozen server would wait out its 10 s internal command timeout,
    // so only the heartbeat can resync within the 8 s below.
    let bus = RedisBus::connect_with(&server.url(), None, Timing::default())
        .await
        .unwrap();
    let mut events = bus.subscribe();
    settle(&bus, &mut events).await;

    server.signal("STOP");
    // Next tick within 2 s, then the ping's 3 s command timeout.
    let event = tokio::time::timeout(Duration::from_secs(8), events.recv())
        .await
        .expect("the heartbeat must resync when Valkey stops answering")
        .unwrap();
    assert_eq!(event, Event::Resync);
    assert!(!bus.live(), "a bus whose ping failed must not report live");

    server.signal("CONT");
    wait_live(&bus, true).await;
    receive(&mut events, &Event::Resync).await;
    let changed = Event::WorkspaceChanged {
        workspace_id: "synthetic-frozen-workspace".into(),
    };
    bus.publish(changed.clone()).await.unwrap();
    receive(&mut events, &changed).await;
}

#[tokio::test]
async fn tls_custom_ca_auth_and_untrusted_certificate_rejection() {
    let Some(server) = Server::start(true).await else {
        return;
    };
    assert!(
        RedisBus::connect(&server.url(), None).await.is_err(),
        "self-signed server must not be trusted by default"
    );
    let ca = std::fs::read(server.directory.join("ca.pem")).unwrap();
    let first = RedisBus::connect(&server.url(), Some(&ca)).await.unwrap();
    let second = RedisBus::connect(&server.url(), Some(&ca)).await.unwrap();
    let mut receiver = second.subscribe();
    let changed = Event::WorkspaceChanged {
        workspace_id: "synthetic-tls-workspace".into(),
    };
    first.publish(changed.clone()).await.unwrap();
    receive(&mut receiver, &changed).await;
    assert!(first.live() && second.live());
}
