//! The physician connection budget, process by process.
//!
//! A site and a worker share a disposable Postgres, each tagged with its own
//! `application_name`. Signing in, inviting a patient through the front door,
//! a chat turn and a burst of `/readyz` all read through each process's one
//! pool, so neither ever holds more than
//! [`local_product::runtime::MAX_CONNECTIONS`]. When the worker's database
//! read fails, the log line keeps the error and says which failure it was:
//! a pool that ran out of connections, or Postgres refusing the query.

use std::time::{Duration, Instant};

use local_product::runtime::MAX_CONNECTIONS;
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

const SITE: &str = "pool-budget-site";
const WORKER: &str = "pool-budget-worker";

fn home(label: &str) -> std::path::PathBuf {
    let home = std::env::temp_dir().join(format!(
        "physician-pool-budget-{label}-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("home");
    home
}

/// The most sessions `application` held across `samples`.
fn peak(samples: &[std::collections::HashMap<String, i64>], application: &str) -> i64 {
    samples
        .iter()
        .map(|sample| sample.get(application).copied().unwrap_or(0))
        .max()
        .unwrap_or(0)
}

#[tokio::test]
async fn each_process_holds_one_bounded_pool() {
    let Some(database) = support::Database::create("pool_budget").await else {
        return;
    };
    support::staging_account(&database.url).await;
    let home = home("bound");
    let stub = support::ResponsesStub::start(&home, 0, 0, "[]").await;
    let worker_port = support::free_port();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
            ("PHYSICIAN_WORKER_POLL_MS", "100"),
            ("PGAPPNAME", WORKER),
        ],
        &home,
    );
    support::wait_ready(&mut worker, worker_port).await;
    let site_port = support::free_port();
    let site_listen = format!("127.0.0.1:{site_port}");
    let mut site = support::Process::spawn(
        "site",
        support::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            support::DURABLE,
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_LISTEN_ADDR", site_listen.as_str()),
            ("PGAPPNAME", SITE),
        ],
        &home,
    );
    support::wait_ready(&mut site, site_port).await;

    let mut samples = vec![database.connections_by_application().await];
    // Auth: sign in. Front door: invite a roster patient.
    let session = support::sign_in(&site_listen).await;
    let (status, body) = support::post(
        &site_listen,
        "/auth/invitations",
        Some(&session),
        &json!({
            "patient_workspace_id": physician_sample::WORKSPACE_IDS[0],
            "email": format!("{}@pool-budget.test", uuid::Uuid::new_v4().simple()),
            "display_name": "Pool Budget",
        }),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", site.tail());
    assert!(body["token"].is_string(), "{body}");
    // A chat turn, which the worker claims and runs.
    let chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let (status, body) = support::post(
        &site_listen,
        "/commands",
        Some(&session),
        &support::submit(physician_sample::WORKSPACE_IDS[0], &chat, "Hello"),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", site.tail());
    // Readiness on both processes, many at once, sampled while in flight.
    for _ in 0..5 {
        let probes = (0..16).map(|index| {
            let host = if index % 2 == 0 {
                site_listen.clone()
            } else {
                worker_listen.clone()
            };
            async move {
                support::exchange(&host, "GET", "/readyz", &[], None)
                    .await
                    .map(|(status, _)| status)
            }
        });
        let (statuses, sample) = tokio::join!(
            futures_util::future::join_all(probes),
            database.connections_by_application()
        );
        for status in statuses {
            assert_eq!(status.expect("readyz"), 200);
        }
        samples.push(sample);
    }
    // The turn finishes on the worker.
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        samples.push(database.connections_by_application().await);
        if !stub.requests().is_empty() {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the worker never ran the turn\n{}",
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    samples.push(database.connections_by_application().await);

    let bound = i64::from(MAX_CONNECTIONS);
    let site_peak = peak(&samples, SITE);
    let worker_peak = peak(&samples, WORKER);
    println!("pool bound {bound}; site peak {site_peak}; worker peak {worker_peak}");
    println!("samples {samples:?}");
    assert!(site_peak >= 1, "the site never connected: {samples:?}");
    assert!(worker_peak >= 1, "the worker never connected: {samples:?}");
    assert!(site_peak <= bound, "site held {site_peak}: {samples:?}");
    assert!(
        worker_peak <= bound,
        "worker held {worker_peak}: {samples:?}"
    );

    site.kill_now();
    worker.kill_now();
    let _ = std::fs::remove_dir_all(&home);
}

/// A login role with its own connection limit, dropped with its grants.
///
/// Role DDL and grants have no SeaQuery form and Postgres takes no bind
/// parameters in utility statements, so they live in `pool_budget/*.sql` and
/// run through psql, which quotes the role, password and limit passed to it
/// as `-v` variables. Queries over the catalog go through SeaQuery.
struct LimitedRole {
    name: String,
    database_url: String,
    admin: String,
    /// The database the role was granted on, as its owner.
    owner: String,
}

/// Runs one of this test's SQL files against `url` with `variables`, and
/// says why it failed when it did.
fn run_sql(url: &str, file: &str, variables: &[(&str, &str)]) -> Result<(), String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/pool_budget")
        .join(file);
    let mut psql = std::process::Command::new("psql");
    psql.args(["-X", "-q", url]);
    for (name, value) in variables {
        psql.arg("-v").arg(format!("{name}={value}"));
    }
    let output = psql
        .arg("-f")
        .arg(&path)
        .output()
        .map_err(|error| format!("psql for {file}: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    Err(format!(
        "{file}: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}

impl LimitedRole {
    async fn create(database: &support::Database) -> Self {
        let name = format!("pool_{}", uuid::Uuid::new_v4().simple());
        // CI's Postgres authenticates TCP logins by password, so the role
        // needs one; trust-auth clusters ignore it.
        let password = uuid::Uuid::new_v4().simple().to_string();
        run_sql(
            database.admin(),
            "create_role.sql",
            &[("role", &name), ("password", &password)],
        )
        .expect("create role");
        run_sql(&database.url, "grant_role.sql", &[("role", &name)]).expect("grant role");
        let mut url = reqwest::Url::parse(&database.url).expect("database url");
        url.set_username(&name).expect("role name");
        url.set_password(Some(&password)).expect("role password");
        Self {
            name,
            database_url: url.to_string(),
            admin: database.admin().to_owned(),
            owner: database.url.clone(),
        }
    }

    /// Sets how many sessions the role may hold; -1 lifts the limit.
    fn limit(&self, connections: i32) {
        run_sql(
            &self.admin,
            "limit_role.sql",
            &[
                ("role", &self.name),
                ("connections", &connections.to_string()),
            ],
        )
        .expect("limit role");
    }

    /// Takes away the role's read of due conversations, so the worker's next
    /// due read is one Postgres refuses.
    fn refuse_due_read(&self) {
        run_sql(&self.owner, "refuse_due_read.sql", &[("role", &self.name)])
            .expect("refuse the due read");
    }

    /// Ends the role's sessions on `database` that are not listening, as an
    /// idle reaper or a failover would, and returns how many it ended.
    async fn end_idle_sessions(&self, database: &support::Database) -> usize {
        use sea_orm::sea_query::extension::postgres::PgExpr;
        use sea_orm::sea_query::{Alias, Expr, ExprTrait, Func, Query};
        let admin = sea_orm::Database::connect(self.admin.as_str())
            .await
            .expect("admin connection");
        let query = Query::select()
            .expr_as(
                Func::cust(Alias::new("pg_terminate_backend")).arg(Expr::col(Alias::new("pid"))),
                Alias::new("ended"),
            )
            .from(Alias::new("pg_stat_activity"))
            .and_where(Expr::col(Alias::new("datname")).eq(database.name()))
            .and_where(Expr::col(Alias::new("usename")).eq(self.name.as_str()))
            .and_where(Expr::col(Alias::new("query")).not_ilike("LISTEN %"))
            .to_owned();
        let rows = admin.query_all(&query).await.expect("end sessions");
        admin.close().await.expect("close admin connection");
        rows.iter()
            .filter(|row| row.try_get::<bool>("", "ended").unwrap_or(false))
            .count()
    }
}

impl Drop for LimitedRole {
    fn drop(&mut self) {
        // Drop cannot await, so cleanup runs through psql. A failed cleanup
        // leaves a role in a disposable cluster, so it only warns.
        let role = [("role", self.name.as_str())];
        for (url, file) in [
            (&self.owner, "drop_owned.sql"),
            (&self.admin, "drop_role.sql"),
        ] {
            if let Err(error) = run_sql(url, file, &role) {
                eprintln!("pool_budget cleanup: {error}");
            }
        }
    }
}

/// The worker's `physician.queue.dispatch_failed` lines, parsed.
fn dispatch_failures(log: &str) -> Vec<Value> {
    log.lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|line| line["event"] == "physician.queue.dispatch_failed")
        .collect()
}

async fn wait_for_failure(worker: &support::Process, error_type: &str) -> Value {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        if let Some(line) = dispatch_failures(&worker.log())
            .into_iter()
            .find(|line| line["error.type"] == error_type)
        {
            return line;
        }
        assert!(
            Instant::now() < deadline,
            "no {error_type} dispatch failure\n{}",
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

#[tokio::test]
async fn the_worker_says_whether_the_pool_ran_out_or_the_query_failed() {
    let Some(database) = support::Database::create("pool_failure").await else {
        return;
    };
    let role = LimitedRole::create(&database).await;
    let home = home("failure");
    let stub = support::ResponsesStub::start(&home, 0, 0, "[]").await;
    let worker_port = support::free_port();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", role.database_url.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
            ("PHYSICIAN_WORKER_POLL_MS", "200"),
            ("CODA_LOG_FORMAT", "json"),
        ],
        &home,
    );
    support::wait_ready(&mut worker, worker_port).await;

    // The role's limit leaves the worker its listener and nothing else, as a
    // full `physician-runtime` role did on staging.
    role.limit(1);
    role.end_idle_sessions(&database).await;
    let timeout = wait_for_failure(&worker, "pool_timeout").await;
    println!("pool timeout: {timeout}");
    let cause = timeout["error"].as_str().expect("error field");
    assert!(cause.contains("timed out"), "{timeout}");
    assert!(timeout["db.response.status_code"].is_null(), "{timeout}");

    // With connections back, a due read Postgres refuses says so instead.
    role.refuse_due_read();
    role.limit(-1);
    let refused = wait_for_failure(&worker, "database").await;
    println!("query failure: {refused}");
    assert_eq!(refused["db.response.status_code"], "42501", "{refused}");
    let cause = refused["error"].as_str().expect("error field");
    assert!(
        cause.contains("physician_runtime_conversations"),
        "{refused}"
    );
    assert!(!cause.contains("timed out"), "{refused}");

    worker.kill_now();
    drop(role);
    let _ = std::fs::remove_dir_all(&home);
}

#[tokio::test]
async fn a_site_without_connections_does_not_ask_for_the_migration_job() {
    let Some(database) = support::Database::create("pool_site").await else {
        return;
    };
    support::staging_account(&database.url).await;
    let role = LimitedRole::create(&database).await;
    role.limit(0);
    let home = home("site");
    let site_port = support::free_port();
    let site_listen = format!("127.0.0.1:{site_port}");
    let mut site = support::Process::spawn(
        "site",
        support::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", role.database_url.as_str()),
            support::DURABLE,
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_LISTEN_ADDR", site_listen.as_str()),
        ],
        &home,
    );
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        assert!(
            site.exited().is_none(),
            "the site exited instead of staying degraded"
        );
        let ready = reqwest::get(format!("http://{site_listen}/readyz")).await;
        let log = site.log();
        if ready.is_ok_and(|response| response.status().as_u16() == 503)
            && log.contains("timed out")
        {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the site never reported degraded timed-out readiness: {log}"
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    let log = site.log();
    println!("site degraded log: {log}");
    assert!(log.contains("timed out"), "{log}");
    assert!(!log.contains("migration"), "{log}");
    role.limit(-1);
    support::wait_ready(&mut site, site_port).await;
    drop(role);
    let _ = std::fs::remove_dir_all(&home);
}
