//! The disposable Langfuse for ignored Docker tests
//! ([`dev/observability/test/langfuse.yaml`](/dev/observability/test/langfuse.yaml)).
//!
//! [`Langfuse::start`] brings one compose project up on loopback ports it
//! chooses and waits for its health checks; dropping it tears the project
//! down with its volumes, printing the named services' logs first when the
//! test is failing. Needs Docker with the fixture's images already pulled.

use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The Langfuse project keys the fixture seeds and its collector sends with.
pub const PUBLIC_KEY: &str = "pk-lf-coda-test";
/// See [`PUBLIC_KEY`].
pub const SECRET_KEY: &str = "sk-lf-coda-test";

/// A loopback port nothing holds right now.
pub fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .and_then(|listener| listener.local_addr())
        .map(|address| address.port())
        .expect("a free loopback port")
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../dev/observability/test")
        .join(name)
}

/// What the test needs from the fixture beyond Langfuse web.
pub enum Shape {
    /// Also langfuse-worker and the real collector (`collector.yaml` plus the
    /// test overlay), whose operations pipelines deliver to OTLP/HTTP on
    /// `ops_sink_port`.
    Ingest { ops_sink_port: u16 },
}

/// One running compose project.
pub struct Langfuse {
    project: String,
    files: Vec<PathBuf>,
    profiles: Vec<&'static str>,
    env: Vec<(&'static str, String)>,
    /// `http://127.0.0.1:<port>`.
    pub url: String,
    /// The Langfuse Postgres on loopback (user, password and database
    /// `langfuse`).
    pub postgres_port: u16,
    /// The collector's OTLP/HTTP receiver, under [`Shape::Ingest`].
    pub otlp_http: Option<String>,
}

impl Langfuse {
    /// Starts the project named `label` and waits until every service is
    /// healthy or running.
    pub fn start(label: &str, shape: Shape) -> Self {
        let langfuse_port = free_port();
        let postgres_port = free_port();
        let mut env = vec![
            ("POSTGRES_PORT", postgres_port.to_string()),
            ("VALKEY_PORT", free_port().to_string()),
            ("CLICKHOUSE_HTTP_PORT", free_port().to_string()),
            ("CLICKHOUSE_NATIVE_PORT", free_port().to_string()),
            ("MINIO_PORT", free_port().to_string()),
            ("LANGFUSE_PORT", langfuse_port.to_string()),
        ];
        let Shape::Ingest { ops_sink_port } = shape;
        let http = free_port();
        env.extend([
            ("LANGFUSE_WORKER_PORT", free_port().to_string()),
            ("COLLECTOR_GRPC_PORT", free_port().to_string()),
            ("COLLECTOR_HTTP_PORT", http.to_string()),
            ("COLLECTOR_HEALTH_PORT", free_port().to_string()),
            ("OPS_SINK_PORT", ops_sink_port.to_string()),
        ]);
        let langfuse = Self {
            project: format!("{label}-{}", std::process::id()),
            files: vec![fixture("langfuse.yaml")],
            profiles: vec!["ingest"],
            env,
            url: format!("http://127.0.0.1:{langfuse_port}"),
            postgres_port,
            otlp_http: Some(format!("http://127.0.0.1:{http}")),
        };
        langfuse.run(&["up", "-d", "--wait", "--wait-timeout", "600"]);
        langfuse
    }

    fn command(&self, args: &[&str]) -> Command {
        let mut command = Command::new("docker");
        command.args(["compose", "-p", &self.project]);
        for file in &self.files {
            command.arg("-f").arg(file);
        }
        for profile in &self.profiles {
            command.args(["--profile", profile]);
        }
        command
            .args(args)
            .envs(self.env.iter().map(|(name, value)| (name, value)));
        command
    }

    /// Runs `docker compose <args>` on this project and requires success.
    pub fn run(&self, args: &[&str]) -> Output {
        let output = self.command(args).output().expect("docker compose runs");
        assert!(
            output.status.success(),
            "docker compose {args:?}: {}{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            self.logs(),
        );
        output
    }

    /// The last lines each service logged.
    pub fn logs(&self) -> String {
        self.command(&["logs", "--tail", "200"])
            .output()
            .map(|logs| {
                format!(
                    "{}{}",
                    String::from_utf8_lossy(&logs.stdout),
                    String::from_utf8_lossy(&logs.stderr)
                )
            })
            .unwrap_or_else(|error| format!("docker compose logs: {error}"))
    }
}

impl Drop for Langfuse {
    fn drop(&mut self) {
        if std::thread::panicking() {
            eprintln!("compose logs:\n{}", self.logs());
        }
        let _ = self.command(&["down", "-v", "--remove-orphans"]).output();
    }
}
