//! Loading a source into a practice, against real Postgres stores.

#[allow(dead_code)]
mod common;
#[path = "common/disposable.rs"]
mod disposable;

use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use local_product::physician_sources::workspace_id;
use local_product::runtime::Store;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use physician_sample::STAGING_PRACTICE_ID;
use physician_sample::chart_load::load_synthetic_practice;
use serde_json::{Value, json};

const NOTE: &str = r#"{"resourceType":"DocumentReference","id":"note-1","date":"2026-08-12T12:05:00+00:00","content":[{"attachment":{"contentType":"text/plain; charset=utf-8","data":"U2VlbiBpbiBjbGluaWMu"}}]}"#;

fn patient(id: &str) -> String {
    format!(
        r#"{{"resourceType":"Patient","id":"{id}","name":[{{"family":"{id}","given":["Mary"]}}],"birthDate":"1972-03-18"}}"#
    )
}

/// A bank of patients `{run}-a` and `{run}-b`, where `a_note` is a's only note.
/// Charts load in order, so a failure at a comes before b.
fn bank(run: &str, a_note: &str) -> PathBuf {
    let bank = std::env::temp_dir().join(format!("coda-chart-load-{run}"));
    for (suffix, note) in [("a", a_note), ("b", NOTE)] {
        let id = format!("{run}-{suffix}");
        let chart = bank.join("patients").join(&id);
        fs::create_dir_all(&chart).unwrap();
        fs::write(chart.join("Patient.ndjson"), format!("{}\n", patient(&id))).unwrap();
        fs::write(chart.join("DocumentReference.ndjson"), format!("{note}\n")).unwrap();
    }
    bank
}

fn run_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("MRN{nanos:x}")
}

/// Signs `email` in on the running site; returns the bearer.
async fn sign_in(base: &str, email: &str) -> String {
    let client = reqwest::Client::new();
    let begun: Value = client
        .post(format!("{base}/auth/sign-in"))
        .json(&json!({ "email": email }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let verified: Value = client
        .post(format!("{base}/auth/verify"))
        .json(&json!({ "email": email, "token": begun["token"] }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    verified["session"].as_str().unwrap().to_owned()
}

async fn read(base: &str, session: &str, path: &str) -> (u16, Value) {
    let response = reqwest::Client::new()
        .get(format!("{base}{path}"))
        .bearer_auth(session)
        .send()
        .await
        .unwrap();
    let status = response.status().as_u16();
    (status, response.json().await.unwrap_or(Value::Null))
}

async fn stores() -> (PostgresAuthStore, Store) {
    stores_at(&std::env::var("PHYSICIAN_TEST_DATABASE_URL").unwrap()).await
}

async fn stores_at(url: &str) -> (PostgresAuthStore, Store) {
    let auth = PostgresAuthStore::connect_with_migrations(url)
        .await
        .unwrap();
    front_door::PostgresFrontDoorStore::connect_with_migrations(url)
        .await
        .unwrap();
    let runtime = Store::connect(url).await.unwrap();
    runtime.migrate().await.unwrap();
    (auth, runtime)
}

/// Runs the job a deploy runs, under the staging profile with `account` on its
/// sign-in list and `charts` within reach.
fn migration_job(charts: &std::path::Path, account: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_physician-migrate"))
        .env(
            "PHYSICIAN_DATABASE_URL",
            std::env::var("PHYSICIAN_TEST_DATABASE_URL").unwrap(),
        )
        .env("CODA_HOSTED_PROFILE", "staging-preview")
        .env("PHYSICIAN_SYNTHETIC_CHARTS", charts)
        .env(
            "PHYSICIAN_STAGING_ACCOUNTS",
            format!("{account}|Staging Physician|admin"),
        )
        .output()
        .unwrap()
}

/// Runs the chart load over `charts` against the database at `url`.
fn load_charts(url: &str, charts: &std::path::Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_physician-migrate"))
        .arg("load-charts")
        .env("PHYSICIAN_DATABASE_URL", url)
        .env("CODA_HOSTED_PROFILE", "staging-preview")
        .env("PHYSICIAN_SYNTHETIC_CHARTS", charts)
        .output()
        .unwrap()
}

/// Stops the site when the test ends, however it ends.
struct Site(Child);

impl Drop for Site {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_loaded_chart_is_a_patient_of_the_practice_and_loading_again_changes_nothing() {
    let (auth, runtime) = stores().await;
    let run = run_id();
    let email = format!("{}@chart-load.test", run.to_lowercase());
    auth.ensure_account(STAGING_PRACTICE_ID, &run, &email, "Admin", true, 1)
        .await
        .unwrap();
    let bank = bank(&run, NOTE);

    let first = load_synthetic_practice(&bank, &auth, &runtime)
        .await
        .unwrap();
    assert_eq!(first.changed, 4);

    let workspace = workspace_id("synthetic-charts", &format!("{run}-a"));
    assert_eq!(
        auth.patient_practice(&workspace).await.unwrap().as_deref(),
        Some(STAGING_PRACTICE_ID)
    );
    let admin = PhysicianIdentity {
        practice_id: STAGING_PRACTICE_ID.to_owned(),
        physician_id: run.clone(),
    };
    assert_eq!(
        auth.membership(&admin, &workspace).await,
        Membership::Allowed
    );
    let observation = runtime.sources(&workspace).await.unwrap();
    assert_eq!(observation.sources.len(), 2);
    let pulls = runtime
        .source_pulls(std::slice::from_ref(&workspace))
        .await
        .unwrap();
    assert_eq!(pulls[&workspace].len(), 1);
    assert_eq!(
        pulls[&workspace][0].patient.display_name,
        format!("Mary {run}-a")
    );
    assert_eq!(pulls[&workspace][0].pulled_at, 1_787_846_400);

    let again = load_synthetic_practice(&bank, &auth, &runtime)
        .await
        .unwrap();
    assert_eq!(again.changed, 0);
    assert_eq!(runtime.sources(&workspace).await.unwrap(), observation);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_load_keeps_panel_choices_and_fills_only_what_is_blank() {
    let (auth, runtime) = stores().await;
    let run = run_id();
    let admin_email = format!("{}@chart-load.test", run.to_lowercase());
    auth.ensure_account(STAGING_PRACTICE_ID, &run, &admin_email, "Admin", true, 1)
        .await
        .unwrap();
    let bank = bank(&run, NOTE);
    load_synthetic_practice(&bank, &auth, &runtime)
        .await
        .unwrap();

    let removed = workspace_id("synthetic-charts", &format!("{run}-a"));
    let kept = workspace_id("synthetic-charts", &format!("{run}-b"));
    let admin = PhysicianIdentity {
        practice_id: STAGING_PRACTICE_ID.to_owned(),
        physician_id: run.clone(),
    };
    auth.assign_panel(&admin, &run, &removed, Membership::Denied)
        .await
        .unwrap();
    let later = format!("{run}-later");
    let later_email = format!("{}@chart-load.test", later.to_lowercase());
    auth.ensure_account(STAGING_PRACTICE_ID, &later, &later_email, "Later", true, 2)
        .await
        .unwrap();

    load_synthetic_practice(&bank, &auth, &runtime)
        .await
        .unwrap();

    assert_eq!(
        auth.membership(&admin, &removed).await,
        Membership::ClinicDemographic
    );
    assert_eq!(auth.membership(&admin, &kept).await, Membership::Allowed);
    let later_admin = PhysicianIdentity {
        practice_id: STAGING_PRACTICE_ID.to_owned(),
        physician_id: later,
    };
    assert_eq!(
        auth.membership(&later_admin, &removed).await,
        Membership::Allowed
    );
    assert_eq!(
        auth.membership(&later_admin, &kept).await,
        Membership::Allowed
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_loaded_patient_is_found_and_opened_on_the_signed_in_site() {
    let (auth, runtime) = stores().await;
    let run = run_id();
    let email = format!("{}@chart-load.test", run.to_lowercase());
    auth.ensure_account(STAGING_PRACTICE_ID, &run, &email, "Admin", true, 1)
        .await
        .unwrap();
    load_synthetic_practice(&bank(&run, NOTE), &auth, &runtime)
        .await
        .unwrap();
    let server = common::serve_isolated(|| {
        let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL").unwrap();
        // SAFETY: `serve_isolated` holds its start lock around this call.
        unsafe {
            std::env::set_var("PHYSICIAN_DATABASE_URL", url);
            std::env::set_var("PHYSICIAN_AUTH", "required");
        }
    })
    .await;
    let base = format!("http://{}", server.address);
    let session = sign_in(&base, &email).await;
    let workspace = workspace_id("synthetic-charts", &format!("{run}-a"));

    let (status, listed) = read(&base, &session, "/patient-workspaces").await;
    assert_eq!(status, 200, "{listed}");
    let on_panel = listed["patients"].as_array().unwrap();
    assert!(
        on_panel
            .iter()
            .any(|row| row["patient_workspace_id"] == workspace.as_str()),
        "{listed}"
    );

    let (status, found) = read(
        &base,
        &session,
        &format!("/patient-roster?query={run}-a%201972"),
    )
    .await;
    assert_eq!(status, 200, "{found}");
    assert!(found.to_string().contains(&workspace), "{found}");

    let (status, page) = read(&base, &session, &format!("/patient-workspaces/{workspace}")).await;
    assert_eq!(status, 200, "{page}");
    assert_eq!(page["display_name"], format!("Mary {run}-a"));
    assert_eq!(page["identity"]["birth_date"], "1972-03-18");
    assert_eq!(page["identity"]["age_years"], 54);
    assert_eq!(
        page["connected_sources"][0]["source_label"],
        "Synthetic chart"
    );
    assert_eq!(
        page["connected_sources"][0]["state"]["synchronized_at"],
        "2026-08-27T16:00:00.000Z"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_chart_that_cannot_load_is_reported_and_the_rest_still_load() {
    let (auth, runtime) = stores().await;
    let scan = NOTE.replace("text/plain; charset=utf-8", "image/tiff");
    let unreadable_note = (run_id(), Some(scan), None);
    let unreadable_patient = (run_id(), None, Some("not json"));
    for (run, note, patient_line) in [unreadable_note, unreadable_patient] {
        let bank = bank(&run, note.as_deref().unwrap_or(NOTE));
        if let Some(line) = patient_line {
            let chart = bank.join("patients").join(format!("{run}-a"));
            fs::write(chart.join("Patient.ndjson"), line).unwrap();
        }

        let summary = load_synthetic_practice(&bank, &auth, &runtime)
            .await
            .unwrap();

        let refused = workspace_id("synthetic-charts", &format!("{run}-a"));
        assert_eq!(summary.failed.len(), 1, "{:?}", summary.failed);
        assert!(summary.failed[0].contains(&refused), "{:?}", summary.failed);
        assert_eq!(summary.patients, 1);
        assert!(auth.patient_practice(&refused).await.unwrap().is_none());
        let loaded = workspace_id("synthetic-charts", &format!("{run}-b"));
        assert!(auth.patient_practice(&loaded).await.unwrap().is_some());
    }
}

/// Has Postgres refuse every new row for `workspace` in `table`.
async fn refuse_writes(auth: &PostgresAuthStore, name: &str, table: &str, workspace: &str) {
    let sql = format!(
        "CREATE FUNCTION {name}() RETURNS trigger AS $$ BEGIN
           IF NEW.patient_workspace_id = '{workspace}' THEN RAISE EXCEPTION 'refused'; END IF;
           RETURN NEW;
         END $$ LANGUAGE plpgsql;
         CREATE TRIGGER {name} BEFORE INSERT ON {table}
           FOR EACH ROW EXECUTE FUNCTION {name}();"
    );
    sea_orm::sqlx::raw_sql(sea_orm::sqlx::AssertSqlSafe(sql))
        .execute(auth.pool())
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_patient_the_practice_could_not_take_is_reported_and_the_next_load_takes_them() {
    let (auth, runtime) = stores().await;
    for table in ["physician_patients", "physician_memberships"] {
        let run = run_id();
        let email = format!("{}@chart-load.test", run.to_lowercase());
        auth.ensure_account(STAGING_PRACTICE_ID, &run, &email, "Admin", true, 1)
            .await
            .unwrap();
        let refused = workspace_id("synthetic-charts", &format!("{run}-a"));
        let name = format!("refuse_{}", run.to_lowercase());
        refuse_writes(&auth, &name, table, &refused).await;
        let bank = bank(&run, NOTE);

        let summary = load_synthetic_practice(&bank, &auth, &runtime)
            .await
            .unwrap();
        assert_eq!(summary.failed.len(), 1, "{table}: {:?}", summary.failed);
        assert!(summary.failed[0].contains(&refused), "{table}");
        assert_eq!(summary.changed, 4, "{table}: a's saved chart still counts");
        let loaded = workspace_id("synthetic-charts", &format!("{run}-b"));
        assert!(
            auth.patient_practice(&loaded).await.unwrap().is_some(),
            "{table}"
        );

        sea_orm::sqlx::raw_sql(sea_orm::sqlx::AssertSqlSafe(format!(
            "DROP TRIGGER {name} ON {table}"
        )))
        .execute(auth.pool())
        .await
        .unwrap();
        let again = load_synthetic_practice(&bank, &auth, &runtime)
            .await
            .unwrap();
        assert!(again.failed.is_empty(), "{table}: {:?}", again.failed);
        let admin = PhysicianIdentity {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            physician_id: run.clone(),
        };
        assert_eq!(
            auth.membership(&admin, &refused).await,
            Membership::Allowed,
            "{table}"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_deploy_leaves_the_charts_alone_and_a_failed_account_fails_it() {
    let (auth, _runtime) = stores().await;
    let run = run_id();
    let account = format!("{}@chart-load.test", run.to_lowercase());
    let refuse = format!("refuse_{}", run.to_lowercase());
    sea_orm::sqlx::raw_sql(sea_orm::sqlx::AssertSqlSafe(format!(
        "CREATE FUNCTION {refuse}() RETURNS trigger AS $$ BEGIN
           IF NEW.email = '{account}' THEN RAISE EXCEPTION 'refused'; END IF;
           RETURN NEW;
         END $$ LANGUAGE plpgsql;
         CREATE TRIGGER {refuse} BEFORE INSERT ON physician_accounts
           FOR EACH ROW EXECUTE FUNCTION {refuse}();"
    )))
    .execute(auth.pool())
    .await
    .unwrap();

    let job = migration_job(&bank(&run, NOTE), &account);

    let stderr = String::from_utf8_lossy(&job.stderr);
    assert!(!job.status.success(), "{stderr}");
    assert!(stderr.contains(&account), "{stderr}");
    let unread = workspace_id("synthetic-charts", &format!("{run}-b"));
    assert!(auth.patient_practice(&unread).await.unwrap().is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_load_tries_every_patient_and_fails_when_one_failed() {
    let (auth, _runtime) = stores().await;
    let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL").unwrap();
    let run = run_id();
    let scan = NOTE.replace("text/plain; charset=utf-8", "image/tiff");

    let load = load_charts(&url, &bank(&run, &scan));

    let stderr = String::from_utf8_lossy(&load.stderr);
    assert!(!load.status.success(), "{stderr}");
    assert!(stderr.contains("image/tiff"), "{stderr}");
    let stdout = String::from_utf8_lossy(&load.stdout);
    assert!(
        stdout.contains("1 patients") && stdout.trim_end().ends_with("1 failed"),
        "{stdout}"
    );
    let loaded = workspace_id("synthetic-charts", &format!("{run}-b"));
    assert!(auth.patient_practice(&loaded).await.unwrap().is_some());

    let repaired = load_charts(&url, &bank(&run, NOTE));
    let stderr = String::from_utf8_lossy(&repaired.stderr);
    assert!(repaired.status.success(), "{stderr}");

    let missing = std::env::temp_dir().join(format!("coda-chart-load-{run}-missing"));
    let load = load_charts(&url, &missing);
    assert!(!load.status.success());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_site_with_no_load_is_ready_and_empty_until_a_load_fills_it() {
    let (shared, _runtime) = stores().await;
    let database = format!("practice_{}", run_id().to_lowercase());
    let admin = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(shared.pool().clone());
    disposable::create_database(&admin, &database).await;
    let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL").unwrap();
    let (server, _) = url.rsplit_once('/').unwrap();
    let url = format!("{server}/{database}");
    let (auth, _runtime) = stores_at(&url).await;
    let run = run_id();
    let email = format!("{}@chart-load.test", run.to_lowercase());
    auth.ensure_account(STAGING_PRACTICE_ID, &run, &email, "Admin", true, 1)
        .await
        .unwrap();

    let address = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap();
    let state = std::env::temp_dir().join(format!("coda-site-start-{database}"));
    fs::create_dir_all(&state).unwrap();
    let mut site = Site(
        Command::new(env!("CARGO_BIN_EXE_physician-sample"))
            .env("PHYSICIAN_DATABASE_URL", &url)
            .env("PHYSICIAN_AUTH", "required")
            .env("PHYSICIAN_LISTEN_ADDR", address.to_string())
            .env_remove("CODA_LOCAL_MODEL_EXECUTION")
            .env_remove("PHYSICIAN_CHAT_TOPIC")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );

    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        assert!(site.0.try_wait().unwrap().is_none(), "the site exited");
        let health = reqwest::get(format!("http://{address}/health")).await;
        if health.is_ok_and(|response| response.status().is_success()) {
            break;
        }
        assert!(Instant::now() < deadline, "the site never answered");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let base = format!("http://{address}");
    assert_eq!(
        reqwest::get(format!("{base}/readyz"))
            .await
            .unwrap()
            .status(),
        200
    );
    assert_eq!(
        auth.practice_clock(STAGING_PRACTICE_ID).await.unwrap(),
        PracticeClock::RealTime,
        "only a load creates the practice"
    );
    let session = sign_in(&base, &email).await;
    let (status, listed) = read(&base, &session, "/patient-workspaces").await;
    assert_eq!(status, 200, "{listed}");
    assert_eq!(listed["patients"], json!([]), "{listed}");

    let load = load_charts(&url, &bank(&run, NOTE));
    let stderr = String::from_utf8_lossy(&load.stderr);
    assert!(load.status.success(), "{stderr}");

    assert_eq!(
        auth.practice_clock(STAGING_PRACTICE_ID).await.unwrap(),
        PracticeClock::Fixed(1_787_846_400)
    );
    let (status, listed) = read(&base, &session, "/patient-workspaces").await;
    assert_eq!(status, 200, "{listed}");
    assert_eq!(listed["patients"].as_array().unwrap().len(), 2, "{listed}");
}
