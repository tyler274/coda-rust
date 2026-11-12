//! Disposable PostgreSQL test environments for `local-product` integration suites.
//!
//! [`DisposablePostgres::acquire`] creates an isolated per-test database on
//! `PHYSICIAN_RUNTIME_TEST_DATABASE_URL` or `PHYSICIAN_TEST_DATABASE_URL` when a
//! shared server is configured (quoting database identifiers with
//! `PostgresQueryBuilder`), and otherwise spins up an isolated cluster under `/tmp`
//! via `initdb` and `pg_ctl`. [`DisposablePostgres::acquire_restartable`] prefers a
//! local `/tmp` cluster whenever `initdb` and `pg_ctl` are on `PATH` so
//! [`DisposablePostgres::crash_restart`] exercises a real server stop and restart,
//! falling back to an isolated per-test database when those tools are absent. Local
//! clusters place their root directory and Unix domain socket in `/tmp` so long
//! `$TMPDIR` paths stay well below Unix `SUN_PATH` limits.
#![allow(
    dead_code,
    reason = "each test binary uses its own subset of this module"
)]

use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;

use local_product::runtime::Store;
use physician_auth::PostgresAuthStore;
use sea_orm::sea_query::{PostgresQueryBuilder, QuotedBuilder};
use sea_orm::{ConnectionTrait, Database};
use uuid::Uuid;

fn pg_command(program: &str) -> Command {
    let mut cmd = Command::new(program);
    for key in ["PGHOST", "PGPORT", "PGUSER", "PGPASSWORD", "PGDATABASE"] {
        cmd.env_remove(key);
    }
    cmd
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ClusterState {
    Running,
    Stopped,
}

struct LocalCluster {
    root: PathBuf,
    pgdata: PathBuf,
    logfile: PathBuf,
    port: u16,
    url: String,
    state: ClusterState,
}

impl LocalCluster {
    fn start() -> Self {
        let id = Uuid::new_v4().simple().to_string();
        let root = PathBuf::from("/tmp").join(format!("cpg-{}", &id[..12]));
        let pgdata = root.join("pgdata");
        let logfile = root.join("postgres.log");
        std::fs::create_dir_all(&root).expect("create disposable Postgres root");

        let init = pg_command("initdb")
            .args([
                "-D",
                pgdata.to_str().expect("utf8 pgdata"),
                "-A",
                "trust",
                "-U",
                "postgres",
                "--no-locale",
                "-E",
                "UTF8",
            ])
            .output()
            .expect("run initdb");
        assert!(
            init.status.success(),
            "initdb failed: {}",
            String::from_utf8_lossy(&init.stderr)
        );

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port for Postgres");
        let port = listener.local_addr().expect("ephemeral port").port();
        drop(listener);

        let mut cluster = Self {
            root,
            pgdata,
            logfile,
            port,
            url: format!("postgres://postgres@127.0.0.1:{port}/postgres"),
            state: ClusterState::Stopped,
        };
        cluster.start_server();
        cluster
    }

    fn options(&self) -> String {
        format!("-p {} -h 127.0.0.1 -k /tmp", self.port)
    }

    fn start_server(&mut self) {
        let out = pg_command("pg_ctl")
            .args([
                "-D",
                self.pgdata.to_str().expect("utf8 pgdata"),
                "-l",
                self.logfile.to_str().expect("utf8 logfile"),
                "-w",
                "-o",
                &self.options(),
                "start",
            ])
            .output()
            .expect("run pg_ctl start");
        assert!(
            out.status.success(),
            "pg_ctl start failed: {}\n{}\npostgres.log:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
            std::fs::read_to_string(&self.logfile).unwrap_or_default()
        );
        self.state = ClusterState::Running;
    }

    fn stop_server_immediate(&mut self) {
        if self.state == ClusterState::Running {
            let stop = pg_command("pg_ctl")
                .args([
                    "-D",
                    self.pgdata.to_str().expect("utf8 pgdata"),
                    "-m",
                    "immediate",
                    "stop",
                ])
                .output()
                .expect("run pg_ctl immediate stop");
            assert!(
                stop.status.success(),
                "pg_ctl stop failed: {}",
                String::from_utf8_lossy(&stop.stderr)
            );
            self.state = ClusterState::Stopped;
        }
    }

    fn crash_restart(&mut self) {
        self.stop_server_immediate();
        self.start_server();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExternalDbState {
    Present,
    Dropped,
}

enum Backend {
    LocalCluster(LocalCluster),
    ExternalDatabase {
        admin_url: String,
        quoted_db: String,
        url: String,
        state: ExternalDbState,
    },
}

/// Isolated PostgreSQL backing for one integration test.
pub struct DisposablePostgres {
    backend: Backend,
}

fn has_local_cluster_tools() -> bool {
    pg_command("initdb")
        .arg("--version")
        .output()
        .is_ok_and(|out| out.status.success())
        && pg_command("pg_ctl")
            .arg("--version")
            .output()
            .is_ok_and(|out| out.status.success())
}

fn external_server_url() -> Option<String> {
    for key in [
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
        "PHYSICIAN_TEST_DATABASE_URL",
    ] {
        if let Ok(raw) = std::env::var(key) {
            let trimmed = raw.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_owned());
            }
        }
    }
    None
}

fn split_database_url(raw: &str) -> (String, String) {
    let (without_query, query_suffix) = match raw.split_once('?') {
        Some((lhs, rhs)) => (lhs, format!("?{rhs}")),
        None => (raw, String::new()),
    };
    let (base, _db) = without_query
        .rsplit_once('/')
        .expect("database URL must contain a database path segment");
    (base.to_owned(), query_suffix)
}

fn unique_db_name(prefix: &str) -> String {
    let mut sanitized = String::with_capacity(28);
    for ch in prefix.chars() {
        if sanitized.len() >= 28 {
            break;
        }
        if ch.is_ascii_alphanumeric() {
            sanitized.push(ch.to_ascii_lowercase());
        } else {
            sanitized.push('_');
        }
    }
    if sanitized.is_empty() || sanitized.starts_with(|c: char| c.is_ascii_digit()) {
        sanitized.insert_str(0, "pg_");
        sanitized.truncate(28);
    }
    format!("{sanitized}_{}", Uuid::new_v4().simple())
}

fn quote_identifier(name: &str) -> String {
    let mut quoted = String::new();
    PostgresQueryBuilder.prepare_iden(&name.to_owned().into(), &mut quoted);
    quoted
}

impl DisposablePostgres {
    async fn acquire_external(raw_url: &str, prefix: &str) -> Self {
        let (base, query_suffix) = split_database_url(raw_url);
        let admin_url = format!("{base}/postgres{query_suffix}");
        let db_name = unique_db_name(prefix);
        let quoted_db = quote_identifier(&db_name);
        let admin = Database::connect(&admin_url)
            .await
            .expect("connect admin Postgres database");
        admin
            .execute_unprepared(&format!("DROP DATABASE IF EXISTS {quoted_db} WITH (FORCE)"))
            .await
            .expect("drop stale per-test database");
        admin
            .execute_unprepared(&format!("CREATE DATABASE {quoted_db}"))
            .await
            .expect("create per-test database");
        admin.close().await.expect("close admin connection");

        Self {
            backend: Backend::ExternalDatabase {
                admin_url,
                quoted_db,
                url: format!("{base}/{db_name}{query_suffix}"),
                state: ExternalDbState::Present,
            },
        }
    }

    fn fail_closed_or_skip() -> Option<Self> {
        assert!(
            std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
            "initdb/pg_ctl or PHYSICIAN_RUNTIME_TEST_DATABASE_URL / PHYSICIAN_TEST_DATABASE_URL required when CODA_REQUIRE_POSTGRES_TESTS is set"
        );
        None
    }

    /// Acquires an isolated PostgreSQL per-test database or local `/tmp` cluster.
    ///
    /// Prefers creating an isolated database on a configured test server URL and
    /// falls back to a local `/tmp` cluster via `initdb` and `pg_ctl`. Returns
    /// `None` only when neither is available and `CODA_REQUIRE_POSTGRES_TESTS` is
    /// unset.
    pub async fn acquire(prefix: &str) -> Option<Self> {
        if let Some(raw_url) = external_server_url() {
            return Some(Self::acquire_external(&raw_url, prefix).await);
        }

        if has_local_cluster_tools() {
            return Some(Self {
                backend: Backend::LocalCluster(LocalCluster::start()),
            });
        }

        Self::fail_closed_or_skip()
    }

    /// Acquires an isolated PostgreSQL instance suitable for crash-restart tests.
    ///
    /// Prefers a dedicated local `/tmp` cluster whenever `initdb` and `pg_ctl` are
    /// on `PATH` so [`Self::crash_restart`] stops and restarts a real server, and
    /// falls back to an isolated per-test database on a configured test server URL
    /// when local cluster tools are absent.
    pub async fn acquire_restartable(prefix: &str) -> Option<Self> {
        if has_local_cluster_tools() {
            return Some(Self {
                backend: Backend::LocalCluster(LocalCluster::start()),
            });
        }

        if let Some(raw_url) = external_server_url() {
            return Some(Self::acquire_external(&raw_url, prefix).await);
        }

        Self::fail_closed_or_skip()
    }

    /// Connection URL for this isolated test database.
    pub fn url(&self) -> &str {
        match &self.backend {
            Backend::LocalCluster(cluster) => &cluster.url,
            Backend::ExternalDatabase { url, .. } => url,
        }
    }

    /// Connects a fresh [`Store`] and runs runtime migrations.
    pub async fn store(&self) -> Store {
        let store = Store::connect(self.url()).await.expect("connect store");
        store.migrate().await.expect("migrate store");
        store
    }

    /// Connects a fresh [`PostgresAuthStore`] and runs auth migrations.
    pub async fn auth_store(&self) -> PostgresAuthStore {
        PostgresAuthStore::connect_with_migrations(self.url())
            .await
            .expect("connect and migrate auth store")
    }

    /// Crash-restarts a local cluster; no-op on a shared external server where
    /// the caller reconnects a fresh pool against persisted database rows.
    pub fn crash_restart(&mut self) {
        match &mut self.backend {
            Backend::LocalCluster(cluster) => cluster.crash_restart(),
            Backend::ExternalDatabase { state, .. } => {
                assert_eq!(
                    *state,
                    ExternalDbState::Present,
                    "cannot restart an already-dropped per-test database"
                );
            }
        }
    }

    /// Immediately terminates database availability for this test's store,
    /// either stopping the local cluster or force-dropping this test's isolated
    /// database on the shared server.
    pub async fn stop_immediate(&mut self) {
        match &mut self.backend {
            Backend::LocalCluster(cluster) => cluster.stop_server_immediate(),
            Backend::ExternalDatabase {
                admin_url,
                quoted_db,
                state,
                ..
            } => {
                if *state == ExternalDbState::Present {
                    let admin = Database::connect(admin_url.as_str())
                        .await
                        .expect("connect admin Postgres database for immediate stop");
                    admin
                        .execute_unprepared(&format!(
                            "DROP DATABASE IF EXISTS {quoted_db} WITH (FORCE)"
                        ))
                        .await
                        .expect("force-drop per-test database");
                    admin.close().await.expect("close admin connection");
                    *state = ExternalDbState::Dropped;
                }
            }
        }
    }
}

impl Drop for DisposablePostgres {
    fn drop(&mut self) {
        match &mut self.backend {
            Backend::LocalCluster(cluster) => {
                if cluster.state == ClusterState::Running {
                    let _ = pg_command("pg_ctl")
                        .args([
                            "-D",
                            cluster.pgdata.to_str().unwrap_or_default(),
                            "-m",
                            "immediate",
                            "stop",
                        ])
                        .output();
                    cluster.state = ClusterState::Stopped;
                }
                let _ = std::fs::remove_file(format!("/tmp/.s.PGSQL.{}", cluster.port));
                let _ = std::fs::remove_file(format!("/tmp/.s.PGSQL.{}.lock", cluster.port));
                let _ = std::fs::remove_dir_all(&cluster.root);
            }
            Backend::ExternalDatabase {
                admin_url,
                quoted_db,
                state,
                ..
            } => {
                if *state == ExternalDbState::Present {
                    let admin_url = admin_url.clone();
                    let quoted_db = quoted_db.clone();
                    let _ = std::thread::spawn(move || {
                        if let Ok(rt) = tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                        {
                            rt.block_on(async move {
                                if let Ok(admin) = Database::connect(&admin_url).await {
                                    let _ = admin
                                        .execute_unprepared(&format!(
                                            "DROP DATABASE IF EXISTS {quoted_db} WITH (FORCE)"
                                        ))
                                        .await;
                                    let _ = admin.close().await;
                                }
                            });
                        }
                    })
                    .join();
                    *state = ExternalDbState::Dropped;
                }
            }
        }
    }
}
