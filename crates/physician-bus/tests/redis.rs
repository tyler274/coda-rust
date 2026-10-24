//! Real disposable Valkey, no shared service, credentials, or fixed ports.
//! Run from the Nix dev shell: cargo test -p physician-bus --test redis --locked
//!
//! Without `valkey-server` the suite returns. The Postgres CI job sets
//! `CODA_REQUIRE_POSTGRES_TESTS` and fails closed if the binary is missing.
use fred::prelude::*;
use physician_bus::{Bus, CHANNEL, Event, RedisBus, Subscription};
use std::{
    net::{TcpListener, TcpStream},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
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
        server.restart().await;
        Some(server)
    }

    async fn restart(&mut self) {
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
            assert!(
                self.child.as_mut().unwrap().try_wait().unwrap().is_none(),
                "Valkey exited during startup"
            );
            if TcpStream::connect(("127.0.0.1", self.port)).is_ok() {
                break;
            }
            assert!(Instant::now() < deadline, "Valkey startup timeout");
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
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
