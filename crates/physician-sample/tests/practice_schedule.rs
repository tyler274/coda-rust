//! End-to-end Postgres + HTTP integration tests for the synthetic bank practice
//! schedule (`physician-migrate load-charts` and `physician-sample` readback).

#[path = "common/disposable.rs"]
mod disposable;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use local_product::physician_sources::workspace_id;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use physician_sample::STAGING_PRACTICE_ID;
use physician_sample::hosting::staging_physician_id;
use physician_wire::PracticeDayResponse;
use reqwest::StatusCode;
use sea_orm::Database;
use serde_json::{Value, json};

const NOTE: &str = r#"{"resourceType":"DocumentReference","id":"note-1","date":"2026-08-12T12:05:00+00:00","content":[{"attachment":{"contentType":"text/plain; charset=utf-8","data":"U2VlbiBpbiBjbGluaWMu"}}]}"#;

struct RunningSite(Child);

impl Drop for RunningSite {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn gate_database_url() -> Option<String> {
    for key in [
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
        "PHYSICIAN_TEST_DATABASE_URL",
        "PHYSICIAN_DATABASE_URL",
    ] {
        if let Ok(url) = std::env::var(key) {
            let trimmed = url.trim().to_owned();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
    assert!(
        std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
        "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL"
    );
    None
}

async fn fresh_database_url(tag: &str) -> Option<(String, String, String)> {
    let gate = gate_database_url()?;
    let (server, _) = gate
        .rsplit_once('/')
        .expect("database URL must include a database name");
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_name = format!("practice_sched_{tag}_{}_{nanos:x}", std::process::id());
    let admin_url = format!("{server}/postgres");
    let admin = Database::connect(&admin_url)
        .await
        .expect("connect admin postgres");
    disposable::replace_database(&admin, &db_name).await;
    admin.close().await.expect("close admin postgres");
    let test_url = format!("{server}/{db_name}");
    Some((admin_url, db_name, test_url))
}

async fn drop_fresh_database(admin_url: &str, db_name: &str) {
    let admin = Database::connect(admin_url)
        .await
        .expect("connect admin postgres for cleanup");
    disposable::drop_database(&admin, db_name).await;
    let _ = admin.close().await;
}

fn temp_bank(tag: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "coda-practice-schedule-{tag}-{}-{nanos:x}",
        std::process::id()
    ));
    fs::create_dir_all(dir.join("calendar")).unwrap();
    for (mrn, given, family) in [("MRN-ALICE", "Alice", "Carter"), ("MRN-BOB", "Bob", "Diaz")] {
        let chart_dir = dir.join("patients").join(mrn);
        fs::create_dir_all(&chart_dir).unwrap();
        let patient_json = format!(
            r#"{{"resourceType":"Patient","id":"{mrn}","name":[{{"family":"{family}","given":["{given}"]}}],"birthDate":"1972-03-18"}}"#
        );
        fs::write(
            chart_dir.join("Patient.ndjson"),
            format!("{patient_json}\n"),
        )
        .unwrap();
        fs::write(
            chart_dir.join("DocumentReference.ndjson"),
            format!("{NOTE}\n"),
        )
        .unwrap();
    }
    dir
}

fn fhir_appointment(
    id: &str,
    status: &str,
    description: &str,
    start: &str,
    end: &str,
    patient_mrn: Option<&str>,
) -> String {
    let participant = match patient_mrn {
        Some(mrn) => {
            format!(r#"[{{"actor":{{"reference":"Patient/{mrn}"}},"status":"accepted"}}]"#)
        }
        None => "[]".to_owned(),
    };
    format!(
        r#"{{"resourceType":"Appointment","id":"{id}","meta":{{"versionId":"1","lastUpdated":"2026-08-27T00:00:00+00:00"}},"status":"{status}","serviceType":[{{"text":"Follow-up"}}],"appointmentType":{{"text":"Follow-up visit"}},"description":"{description}","start":"{start}","end":"{end}","minutesDuration":30,"participant":{participant}}}"#
    )
}

fn run_migrate_schema(db_url: &str, staging_accounts: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_physician-migrate"))
        .env("PHYSICIAN_DATABASE_URL", db_url)
        .env("CODA_HOSTED_PROFILE", "staging-preview")
        .env("PHYSICIAN_STAGING_ACCOUNTS", staging_accounts)
        .env_remove("PHYSICIAN_SIMULATION_PRACTICES")
        .output()
        .expect("run physician-migrate")
}

fn run_load_charts(db_url: &str, bank: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_physician-migrate"))
        .arg("load-charts")
        .env("PHYSICIAN_DATABASE_URL", db_url)
        .env("CODA_HOSTED_PROFILE", "staging-preview")
        .env("PHYSICIAN_SYNTHETIC_CHARTS", bank)
        .output()
        .expect("run physician-migrate load-charts")
}

async fn spawn_site(db_url: &str, synthetic_calendar: Option<&Path>) -> (RunningSite, String) {
    let address = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_physician-sample"));
    cmd.env("PHYSICIAN_DATABASE_URL", db_url)
        .env("PHYSICIAN_AUTH", "required")
        .env("PHYSICIAN_LISTEN_ADDR", address.to_string())
        .env_remove("PHYSICIAN_SYNTHETIC_CHARTS")
        .env_remove("PHYSICIAN_SYNTHETIC_CALENDAR")
        .env_remove("CODA_LOCAL_MODEL_EXECUTION")
        .env_remove("PHYSICIAN_CHAT_TOPIC")
        .env_remove("PHYSICIAN_UPSTREAM")
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if let Some(calendar) = synthetic_calendar {
        cmd.env("PHYSICIAN_SYNTHETIC_CALENDAR", calendar);
    }
    let mut site = RunningSite(cmd.spawn().expect("spawn physician-sample"));
    let base = format!("http://{address}");
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        assert!(
            site.0.try_wait().unwrap().is_none(),
            "physician-sample exited unexpectedly"
        );
        let ready = reqwest::get(format!("{base}/readyz")).await;
        if ready.is_ok_and(|resp| resp.status().is_success()) {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "physician-sample never became ready"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    (site, base)
}

async fn sign_in_http(client: &reqwest::Client, base: &str, email: &str) -> String {
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

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn load_charts_syncs_synthetic_bank_calendar_for_runtime_site_without_bank_mount() {
    let Some((admin_url, db_name, db_url)) = fresh_database_url("load_charts").await else {
        return;
    };

    let admin_email = "admin@codahealth.test";
    let migrate = run_migrate_schema(&db_url, &format!("{admin_email}|Admin Doc|admin"));
    assert!(
        migrate.status.success(),
        "schema migrate failed: {}",
        String::from_utf8_lossy(&migrate.stderr)
    );

    // Start physician-sample WITHOUT PHYSICIAN_SYNTHETIC_CHARTS or PHYSICIAN_SYNTHETIC_CALENDAR,
    // matching the hosted GCP staging runtime image where only physician-migration-image holds the bank.
    let (site, base) = spawn_site(&db_url, None).await;
    let client = reqwest::Client::new();
    let admin_session = sign_in_http(&client, &base, admin_email).await;

    let before_load: Value = client
        .get(format!("{base}/practice-day"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(before_load["source_state"]["state"], "absent");
    assert_eq!(before_load["visits"], json!([]));

    let bank = temp_bank("load-charts");
    let calendar_path = bank.join("calendar/Appointment.ndjson");

    // 1. Corrupt appointment in calendar/Appointment.ndjson fails load-charts and names what failed,
    //    while saving valid delivered visits as a partial schedule.
    let corrupt_calendar = [
        fhir_appointment(
            "appt-alice",
            "booked",
            "Medication follow-up",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-ALICE"),
        ),
        r#"{"resourceType":"Appointment","id":"appt-corrupt","status":"unsupported_code"}"#
            .to_owned(),
    ]
    .join("\n");
    fs::write(&calendar_path, format!("{corrupt_calendar}\n")).unwrap();

    let failed_load = run_load_charts(&db_url, &bank);
    let failed_stderr = String::from_utf8_lossy(&failed_load.stderr);
    assert!(!failed_load.status.success(), "{failed_stderr}");
    assert!(
        failed_stderr.contains("Appointment/appt-corrupt"),
        "{failed_stderr}"
    );

    let partial_day: Value = client
        .get(format!("{base}/practice-day"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(partial_day["source_state"]["state"], "usable");
    assert_eq!(partial_day["source_state"]["partial"], true);
    assert_eq!(
        partial_day["source_state"]["failures"][0]["target"],
        "Appointment/appt-corrupt"
    );
    assert_eq!(partial_day["visits"].as_array().unwrap().len(), 1);

    // 2. Repaired synthetic bank calendar loads cleanly; runtime site serves matched + unmatched visits
    //    strictly from Postgres without adding unlinked appointment MRNs to the roster.
    let clean_calendar = [
        fhir_appointment(
            "appt-alice",
            "booked",
            "Medication follow-up",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-ALICE"),
        ),
        fhir_appointment(
            "appt-bob",
            "booked",
            "Annual physical",
            "2026-08-27T18:00:00+00:00",
            "2026-08-27T18:30:00+00:00",
            Some("MRN-BOB"),
        ),
        fhir_appointment(
            "appt-new-patient",
            "booked",
            "New patient visit",
            "2026-08-27T19:00:00+00:00",
            "2026-08-27T19:30:00+00:00",
            None,
        ),
        fhir_appointment(
            "appt-unlinked-mrn",
            "booked",
            "Walk-in consult",
            "2026-08-27T20:00:00+00:00",
            "2026-08-27T20:30:00+00:00",
            Some("MRN-NOT-ON-ROSTER"),
        ),
    ]
    .join("\n");
    fs::write(&calendar_path, format!("{clean_calendar}\n")).unwrap();

    let ok_load = run_load_charts(&db_url, &bank);
    let ok_stdout = String::from_utf8_lossy(&ok_load.stdout);
    let ok_stderr = String::from_utf8_lossy(&ok_load.stderr);
    assert!(ok_load.status.success(), "{ok_stderr}");
    assert!(
        ok_stdout.contains(
            "synthetic schedule: 4 appointments (2 matched, 2 unmatched), 3 changed, 0 failed"
        ),
        "{ok_stdout}"
    );

    let day_json: Value = client
        .get(format!("{base}/practice-day"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let wire_day: PracticeDayResponse =
        serde_json::from_value(day_json.clone()).expect("PracticeDayResponse wire contract");
    assert_eq!(wire_day.source_state.state, "usable");
    assert_eq!(day_json["source_state"]["partial"], false);
    assert_eq!(
        day_json["source_state"]["snapshot_as_of"],
        "2026-08-27T16:00:00.000Z"
    );
    assert_eq!(wire_day.visits.len(), 4);

    let alice_ws = workspace_id("synthetic-charts", "MRN-ALICE");
    let bob_ws = workspace_id("synthetic-charts", "MRN-BOB");
    assert_eq!(
        wire_day.visits[0].patient_workspace_id.as_deref(),
        Some(alice_ws.as_str())
    );
    assert_eq!(
        wire_day.visits[0].patient_display_name.as_deref(),
        Some("Alice Carter")
    );
    assert_eq!(
        wire_day.visits[1].patient_workspace_id.as_deref(),
        Some(bob_ws.as_str())
    );
    assert_eq!(
        wire_day.visits[1].patient_display_name.as_deref(),
        Some("Bob Diaz")
    );
    assert!(wire_day.visits[2].patient_workspace_id.is_none());
    assert_eq!(wire_day.visits[2].label, "New patient visit");
    assert!(wire_day.visits[3].patient_workspace_id.is_none());
    assert_eq!(
        day_json["visits"][3]["unmatched_source_patient_id"],
        "MRN-NOT-ON-ROSTER"
    );

    let workspaces: Value = client
        .get(format!("{base}/patient-workspaces"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(workspaces["patients"].as_array().unwrap().len(), 2);

    drop(site);
    let _ = fs::remove_dir_all(&bank);
    drop_fresh_database(&admin_url, &db_name).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_readback_matches_linked_patients_keeps_unmatched_visible_and_preserves_last_good_on_failure()
 {
    let Some((admin_url, db_name, db_url)) = fresh_database_url("http_sync").await else {
        return;
    };

    let admin_email = "admin@codahealth.test";
    let peer_email = "peer@codahealth.test";
    let migrate = run_migrate_schema(
        &db_url,
        &format!("{admin_email}|Admin Doc|admin,{peer_email}|Peer Doc|physician"),
    );
    assert!(
        migrate.status.success(),
        "schema migrate failed: {}",
        String::from_utf8_lossy(&migrate.stderr)
    );

    // Load charts first WITHOUT calendar/Appointment.ndjson so /practice-day starts "absent".
    let bank = temp_bank("http-sync");
    let calendar_path = bank.join("calendar/Appointment.ndjson");
    let chart_only_load = run_load_charts(&db_url, &bank);
    assert!(
        chart_only_load.status.success(),
        "{}",
        String::from_utf8_lossy(&chart_only_load.stderr)
    );

    let alice_ws = workspace_id("synthetic-charts", "MRN-ALICE");
    let bob_ws = workspace_id("synthetic-charts", "MRN-BOB");
    let auth = PostgresAuthStore::connect_without_migrations(&db_url)
        .await
        .unwrap();
    let admin_id = PhysicianIdentity {
        practice_id: STAGING_PRACTICE_ID.to_owned(),
        physician_id: staging_physician_id(admin_email),
    };
    let peer_physician_id = staging_physician_id(peer_email);
    // Restrict peer physician's panel to Alice only (remove Bob).
    auth.assign_panel(&admin_id, &peer_physician_id, &bob_ws, Membership::Denied)
        .await
        .unwrap();
    auth.assign_panel(
        &admin_id,
        &peer_physician_id,
        &alice_ws,
        Membership::Allowed,
    )
    .await
    .unwrap();

    let initial_calendar = [
        fhir_appointment(
            "appt-alice",
            "booked",
            "Medication follow-up",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-ALICE"),
        ),
        fhir_appointment(
            "appt-bob",
            "booked",
            "Annual physical",
            "2026-08-27T18:00:00+00:00",
            "2026-08-27T18:30:00+00:00",
            Some("MRN-BOB"),
        ),
        fhir_appointment(
            "appt-new-patient",
            "booked",
            "New patient visit",
            "2026-08-27T19:00:00+00:00",
            "2026-08-27T19:30:00+00:00",
            None,
        ),
        fhir_appointment(
            "appt-unlinked-mrn",
            "booked",
            "Walk-in consult",
            "2026-08-27T20:00:00+00:00",
            "2026-08-27T20:30:00+00:00",
            Some("MRN-NOT-ON-ROSTER"),
        ),
    ]
    .join("\n");
    fs::write(&calendar_path, format!("{initial_calendar}\n")).unwrap();

    let (site, base) = spawn_site(&db_url, Some(&calendar_path)).await;
    let client = reqwest::Client::new();
    let admin_session = sign_in_http(&client, &base, admin_email).await;
    let peer_session = sign_in_http(&client, &base, peer_email).await;

    // Before first schedule sync, /practice-day reports "absent".
    let pre_sync: Value = client
        .get(format!("{base}/practice-day"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(pre_sync["source_state"]["state"], "absent");
    assert_eq!(pre_sync["visits"], json!([]));

    // Non-admin physician cannot trigger /director/schedule/sync.
    let forbidden = client
        .post(format!("{base}/director/schedule/sync"))
        .bearer_auth(&peer_session)
        .send()
        .await
        .unwrap();
    assert_eq!(forbidden.status(), StatusCode::FORBIDDEN);

    // Admin triggers schedule sync over HTTP.
    let sync_resp = client
        .post(format!("{base}/director/schedule/sync"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap();
    assert_eq!(sync_resp.status(), StatusCode::OK);
    let sync_body: Value = sync_resp.json().await.unwrap();
    assert_eq!(sync_body["outcome"], "applied");
    assert_eq!(sync_body["saved_appointments"], 4);
    assert_eq!(sync_body["matched_appointments"], 2);
    assert_eq!(sync_body["unmatched_appointments"], 2);

    // Roster invariant: unlinked MRN-NOT-ON-ROSTER was NOT added to the practice roster!
    let roster_after = auth.practice_patients(STAGING_PRACTICE_ID).await.unwrap();
    assert_eq!(roster_after.len(), 2);
    assert!(roster_after.contains(&alice_ws));
    assert!(roster_after.contains(&bob_ws));

    // Admin HTTP readback on /practice-day: 2 matched + 2 unmatched visits.
    let day_resp = client
        .get(format!("{base}/practice-day"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap();
    assert_eq!(day_resp.status(), StatusCode::OK);
    let day_json: Value = day_resp.json().await.unwrap();
    let wire_day: PracticeDayResponse =
        serde_json::from_value(day_json.clone()).expect("PracticeDayResponse wire contract");
    assert_eq!(wire_day.source_state.state, "usable");
    assert_eq!(wire_day.visits.len(), 4);
    assert_eq!(
        day_json["source_state"]["snapshot_as_of"],
        "2026-08-27T16:00:00.000Z"
    );
    assert_eq!(day_json["source_state"]["partial"], false);

    // Peer physician HTTP readback: sees Alice (on panel) + 2 unmatched, NOT Bob (off panel).
    let peer_day: Value = client
        .get(format!("{base}/practice-day"))
        .bearer_auth(&peer_session)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let peer_visits = peer_day["visits"].as_array().unwrap();
    assert_eq!(peer_visits.len(), 3);
    assert!(
        !peer_visits
            .iter()
            .any(|v| v["patient_workspace_id"] == bob_ws.as_str())
    );

    // Reschedule Alice + cancel Bob, plus inject one corrupt line (partial read).
    let updated_calendar = [
        fhir_appointment(
            "appt-alice",
            "booked",
            "Medication follow-up",
            "2026-08-27T21:00:00+00:00",
            "2026-08-27T21:30:00+00:00",
            Some("MRN-ALICE"),
        ),
        fhir_appointment(
            "appt-bob",
            "cancelled",
            "Annual physical",
            "2026-08-27T18:00:00+00:00",
            "2026-08-27T18:30:00+00:00",
            Some("MRN-BOB"),
        ),
        r#"{"resourceType":"Appointment","id":"appt-broken","status":"invalid_code"}"#.to_owned(),
        fhir_appointment(
            "appt-new-patient",
            "booked",
            "New patient visit",
            "2026-08-27T19:00:00+00:00",
            "2026-08-27T19:30:00+00:00",
            None,
        ),
    ]
    .join("\n");
    fs::write(&calendar_path, format!("{updated_calendar}\n")).unwrap();

    let partial_sync = client
        .post(format!("{base}/director/schedule/sync"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap();
    assert_eq!(partial_sync.status(), StatusCode::OK);
    let partial_day: Value = client
        .get(format!("{base}/practice-day"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(partial_day["source_state"]["partial"], true);
    assert_eq!(
        partial_day["source_state"]["failures"][0]["target"],
        "Appointment/appt-broken"
    );
    let visits_after_partial = partial_day["visits"].as_array().unwrap();
    let bob_visit = visits_after_partial
        .iter()
        .find(|v| v["encounter"]["native_event_id"] == "appt-bob")
        .unwrap();
    assert_eq!(bob_visit["status"], "cancelled");
    assert_eq!(bob_visit["encounter"]["saved_version"], 2);
    let alice_visit = visits_after_partial
        .iter()
        .find(|v| v["encounter"]["native_event_id"] == "appt-alice")
        .unwrap();
    assert_eq!(alice_visit["starts_at"], "2026-08-27T21:00:00.000Z");
    assert_eq!(alice_visit["encounter"]["saved_version"], 2);

    // Remove calendar file -> failed read returns 502 while /practice-day keeps last good schedule + as_of!
    fs::remove_file(&calendar_path).unwrap();
    let failed_sync = client
        .post(format!("{base}/director/schedule/sync"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap();
    assert_eq!(failed_sync.status(), StatusCode::BAD_GATEWAY);

    let preserved_day: Value = client
        .get(format!("{base}/practice-day"))
        .bearer_auth(&admin_session)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(preserved_day["source_state"]["state"], "usable");
    assert_eq!(preserved_day["source_state"]["refresh_status"], "failed");
    assert_eq!(
        preserved_day["source_state"]["snapshot_as_of"],
        "2026-08-27T16:00:00.000Z"
    );
    assert_eq!(preserved_day["visits"].as_array().unwrap().len(), 4);

    drop(site);
    let _ = fs::remove_dir_all(&bank);
    drop_fresh_database(&admin_url, &db_name).await;
}
