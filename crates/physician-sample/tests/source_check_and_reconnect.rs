//! End-to-end Postgres + HTTP readback proof for source checks, reconnect commands,
//! retained last-good-read timestamps, and practice-wide outage summarization.

#[allow(dead_code)]
mod common;

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use local_product::physician_sources::workspace_id;
use local_product::runtime::Store;
use physician_auth::migration::PracticeActiveModel;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use physician_sample::chart_load::load_practice_from_source;
use physician_sample::synthetic_chart::{SOURCE_ID, SyntheticCharts};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ActiveValue::Unchanged, Database};
use serde_json::{Value, json};

const NOTE: &str = r#"{"resourceType":"DocumentReference","id":"note-1","date":"2026-08-12T12:05:00+00:00","content":[{"attachment":{"contentType":"text/plain; charset=utf-8","data":"U2VlbiBpbiBjbGluaWMu"}}]}"#;
const VISIT: &str = r#"{"resourceType":"Encounter","id":"visit-1","period":{"start":"2026-07-26T14:15:00+00:00","end":"2026-07-26T15:00:00+00:00"}}"#;

fn run_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("SRC{nanos:x}")
}

fn patient_ndjson(id: &str) -> String {
    format!(
        r#"{{"resourceType":"Patient","id":"{id}","name":[{{"family":"{id}","given":["Clara"]}}],"gender":"female","birthDate":"1980-04-10"}}"#
    )
}

fn build_bank_at(dir_suffix: &str, patient_prefix: &str, count: usize) -> PathBuf {
    let root = std::env::temp_dir().join(format!("coda-source-check-{dir_suffix}"));
    for index in 1..=count {
        let id = format!("{patient_prefix}-{index}");
        let chart = root.join("patients").join(&id);
        fs::create_dir_all(&chart).unwrap();
        fs::write(
            chart.join("Patient.ndjson"),
            format!("{}\n", patient_ndjson(&id)),
        )
        .unwrap();
        fs::write(chart.join("DocumentReference.ndjson"), format!("{NOTE}\n")).unwrap();
        fs::write(chart.join("Encounter.ndjson"), format!("{VISIT}\n")).unwrap();
    }
    root
}

fn build_bank(run: &str, count: usize) -> PathBuf {
    build_bank_at(run, run, count)
}

async fn stores() -> (PostgresAuthStore, Store, String) {
    let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL"))
        .expect("set PHYSICIAN_TEST_DATABASE_URL");
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    front_door::PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let runtime = Store::connect(&url).await.unwrap();
    runtime.migrate().await.unwrap();
    (auth, runtime, url)
}

async fn advance_practice_clock(url: &str, practice_id: &str, epoch: u64) {
    let db = Database::connect(url).await.unwrap();
    PracticeActiveModel {
        practice_id: Unchanged(practice_id.to_owned()),
        fixed_time: Set(Some(epoch as i64)),
        ..Default::default()
    }
    .update(&db)
    .await
    .unwrap();
    db.close().await.unwrap();
}

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

async fn get_json(base: &str, session: &str, path: &str) -> (u16, Value) {
    let response = reqwest::Client::new()
        .get(format!("{base}{path}"))
        .bearer_auth(session)
        .send()
        .await
        .unwrap();
    let status = response.status().as_u16();
    (status, response.json().await.unwrap_or(Value::Null))
}

async fn post_json(base: &str, session: &str, path: &str, body: Value) -> (u16, Value) {
    let response = reqwest::Client::new()
        .post(format!("{base}{path}"))
        .bearer_auth(session)
        .json(&body)
        .send()
        .await
        .unwrap();
    let status = response.status().as_u16();
    (status, response.json().await.unwrap_or(Value::Null))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn check_and_reconnect_flow_preserves_last_good_read_and_collapses_practice_outage_over_http()
{
    let (auth, runtime, db_url) = stores().await;
    let run = run_id();
    let practice_id = format!("practice-{}", run.to_lowercase());
    let email = format!("{}@codahealth.test", run.to_lowercase());
    let colleague_id = format!("{run}-colleague");
    let colleague_email = format!("{}-colleague@codahealth.test", run.to_lowercase());
    let initial_clock: u64 = 1_787_846_400; // 2026-08-27T16:00:00.000Z
    let initial_iso = "2026-08-27T16:00:00.000Z";

    auth.ensure_practice(&practice_id, PracticeClock::Fixed(initial_clock))
        .await
        .unwrap();
    auth.ensure_account(&practice_id, &run, &email, "Dr. Check", true, 1)
        .await
        .unwrap();

    let bank = build_bank(&run, 3);
    let charts = SyntheticCharts::open(&bank);
    let loaded = load_practice_from_source(&practice_id, &charts, &auth, &runtime)
        .await
        .unwrap();
    assert_eq!(loaded.patients, 3);
    assert!(loaded.failed.is_empty(), "{:?}", loaded.failed);

    auth.ensure_account(
        &practice_id,
        &colleague_id,
        &colleague_email,
        "Dr. Colleague",
        false,
        1,
    )
    .await
    .unwrap();

    let admin = PhysicianIdentity {
        practice_id: practice_id.clone(),
        physician_id: run.clone(),
    };
    let ws1 = workspace_id(SOURCE_ID, &format!("{run}-1"));
    let ws2 = workspace_id(SOURCE_ID, &format!("{run}-2"));
    let ws3 = workspace_id(SOURCE_ID, &format!("{run}-3"));
    for ws in [&ws1, &ws2, &ws3] {
        auth.place_on_panel(&admin, ws).await.unwrap();
    }

    let bank_for_env = bank.clone();
    let url_for_env = db_url.clone();
    let server = common::serve_isolated(move || {
        // SAFETY: `serve_isolated` serializes server startup under its mutex.
        unsafe {
            std::env::set_var("PHYSICIAN_DATABASE_URL", &url_for_env);
            std::env::set_var("PHYSICIAN_AUTH", "required");
            std::env::set_var("PHYSICIAN_SYNTHETIC_CHARTS", &bank_for_env);
            std::env::set_var("PHYSICIAN_LOCAL_PROOF", "1");
        }
    })
    .await;
    let base = format!("http://{}", server.address);
    let session = sign_in(&base, &email).await;
    let colleague_session = sign_in(&base, &colleague_email).await;

    // A same-clinic colleague without panel assignment receives 404 on patient-targeted source checks.
    let (denied_ws_status, _) = post_json(
        &base,
        &colleague_session,
        "/connections/synthetic-charts/check",
        json!({ "patient_workspace_id": ws1 }),
    )
    .await;
    assert_eq!(denied_ws_status, 404);
    let (denied_spid_status, _) = post_json(
        &base,
        &colleague_session,
        "/connections/synthetic-charts/check",
        json!({ "source_patient_id": format!("{run}-1") }),
    )
    .await;
    assert_eq!(denied_spid_status, 404);

    // 1. Healthy baseline readback on /connections, /today, and /patient-workspaces/{ws1}.
    let (status, connections) = get_json(&base, &session, "/connections").await;
    assert_eq!(status, 200, "{connections}");
    assert_eq!(connections["outages"], json!([]));
    assert_eq!(connections["patient_issues"], json!([]));
    assert_eq!(connections["sources"][0]["source_id"], SOURCE_ID);
    assert_eq!(connections["sources"][0]["summary"]["scope"], "healthy");
    assert_eq!(connections["sources"][0]["summary"]["total_patients"], 3);

    let (status, today) = get_json(&base, &session, "/today").await;
    assert_eq!(status, 200, "{today}");
    assert_eq!(today["connections"], json!([]));
    assert_eq!(today["source_outages"], json!([]));

    let (status, page1) = get_json(&base, &session, &format!("/patient-workspaces/{ws1}")).await;
    assert_eq!(status, 200, "{page1}");
    assert_eq!(page1["connected_sources"][0]["standing"], "up_to_date");
    assert_eq!(page1["connected_sources"][0]["state"]["state"], "usable");
    assert_eq!(
        page1["connected_sources"][0]["state"]["synchronized_at"],
        initial_iso
    );

    // Advance the practice clock so subsequent check/reconnect timestamps are distinct from initial load.
    let partial_clock: u64 = 1_787_850_000; // 2026-08-27T17:00:00.000Z
    let partial_iso = "2026-08-27T17:00:00.000Z";
    advance_practice_clock(&db_url, &practice_id, partial_clock).await;

    // 2. Inject Partial failure for 1 of 3 patients (minority -> PatientIssues, NOT PracticeWideOutage).
    let (status, mode_resp) = post_json(
        &base,
        &session,
        "/local-proof/sources/synthetic-charts/mode",
        json!({
            "source_patient_id": format!("{run}-1"),
            "mode": "partial",
            "failed_families": ["DocumentReference"],
            "reason": "DocumentReference endpoint 502"
        }),
    )
    .await;
    assert_eq!(status, 200, "{mode_resp}");

    let (status, check_one) = post_json(
        &base,
        &session,
        "/connections/synthetic-charts/check",
        json!({ "patient_workspace_id": ws1 }),
    )
    .await;
    assert_eq!(status, 200, "{check_one}");
    assert_eq!(check_one["patient"]["standing"]["standing"], "partial");
    assert_eq!(
        check_one["patient"]["standing"]["last_good_read_at"],
        initial_clock as i64
    );
    assert_eq!(
        check_one["patient"]["standing"]["failed_parts"],
        json!([{ "part": "DocumentReference", "reason": "DocumentReference endpoint 502" }])
    );
    assert_eq!(check_one["summary"]["scope"], "patient_issues");
    assert_eq!(check_one["outages"], json!([]));
    assert_eq!(check_one["patient_issues"].as_array().unwrap().len(), 1);

    // Panel-scoped filtering on /connections: Dr. Check sees ws1's issue, whereas Dr. Colleague
    // (same practice, not assigned ws1 on panel) receives zero per-patient issue details.
    let (status, check_conns) = get_json(&base, &session, "/connections").await;
    assert_eq!(status, 200, "{check_conns}");
    assert_eq!(check_conns["patient_issues"].as_array().unwrap().len(), 1);
    let (status, colleague_conns) = get_json(&base, &colleague_session, "/connections").await;
    assert_eq!(status, 200, "{colleague_conns}");
    assert_eq!(colleague_conns["patient_issues"], json!([]));
    assert_eq!(colleague_conns["sources"][0]["patient_issues"], json!([]));
    assert_eq!(
        colleague_conns["sources"][0]["summary"]["issues"],
        json!([])
    );
    let (status, colleague_check) = post_json(
        &base,
        &colleague_session,
        "/connections/synthetic-charts/check",
        json!({}),
    )
    .await;
    assert_eq!(status, 200, "{colleague_check}");
    assert_eq!(colleague_check["patient_issues"], json!([]));
    assert_eq!(colleague_check["summary"]["issues"], json!([]));

    // Patient page preserves `usable_snapshot.synchronized_at == initial_iso` while naming failed_parts.
    let (status, page1_partial) =
        get_json(&base, &session, &format!("/patient-workspaces/{ws1}")).await;
    assert_eq!(status, 200, "{page1_partial}");
    assert_eq!(page1_partial["connected_sources"][0]["standing"], "partial");
    assert_eq!(
        page1_partial["connected_sources"][0]["last_good_read_at"],
        initial_iso
    );
    assert_eq!(
        page1_partial["connected_sources"][0]["attempted_at"],
        partial_iso
    );
    assert_eq!(
        page1_partial["connected_sources"][0]["state"]["usable_snapshot"]["synchronized_at"],
        initial_iso
    );
    assert_eq!(
        page1_partial["connected_sources"][0]["failed_parts"][0]["part"],
        "DocumentReference"
    );

    // 3. Inject practice-wide Outage -> ONE PracticeWideOutage and ZERO per-patient rows on /connections & /today.
    let outage_clock: u64 = 1_787_853_600; // 2026-08-27T18:00:00.000Z
    advance_practice_clock(&db_url, &practice_id, outage_clock).await;

    let (status, _) = post_json(
        &base,
        &session,
        "/local-proof/sources/synthetic-charts/mode",
        json!({
            "mode": "outage",
            "reason": "upstream FHIR gateway 503"
        }),
    )
    .await;
    assert_eq!(status, 200);

    let (status, outage_check) = post_json(
        &base,
        &session,
        "/connections/synthetic-charts/check",
        json!({}),
    )
    .await;
    assert_eq!(status, 200, "{outage_check}");
    assert_eq!(outage_check["summary"]["scope"], "practice_wide_outage");
    assert_eq!(outage_check["summary"]["affected_patients"], 3);
    assert_eq!(outage_check["summary"]["total_patients"], 3);
    assert_eq!(outage_check["outages"].as_array().unwrap().len(), 1);
    assert_eq!(
        outage_check["patient_issues"],
        json!([]),
        "practice-wide outage must not emit one row per patient"
    );

    let (status, conns_outage) = get_json(&base, &session, "/connections").await;
    assert_eq!(status, 200, "{conns_outage}");
    assert_eq!(conns_outage["outages"].as_array().unwrap().len(), 1);
    assert_eq!(conns_outage["outages"][0]["kind"]["kind"], "source_down");
    assert_eq!(conns_outage["patient_issues"], json!([]));

    let (status, today_outage) = get_json(&base, &session, "/today").await;
    assert_eq!(status, 200, "{today_outage}");
    assert_eq!(today_outage["connections"].as_array().unwrap().len(), 1);
    assert_eq!(
        today_outage["connections"][0]["condition"],
        "temporarily_unavailable"
    );
    assert_eq!(
        today_outage["connections"][0]["scope"],
        "practice_wide_outage"
    );
    assert_eq!(today_outage["rows"], json!([]));

    // 4. Inject LapsedAccess -> check shows ReconnectNeeded + preserved last_good_read_at ->
    //    reconnect command through /commands restores Connected & UpToDate!
    let lapsed_clock: u64 = 1_787_857_200; // 2026-08-27T19:00:00.000Z
    advance_practice_clock(&db_url, &practice_id, lapsed_clock).await;

    let (status, _) = post_json(
        &base,
        &session,
        "/local-proof/sources/synthetic-charts/mode",
        json!({
            "mode": "lapsed_access",
            "reason": "practice OAuth token expired"
        }),
    )
    .await;
    assert_eq!(status, 200);

    let (status, lapsed_check) = post_json(
        &base,
        &session,
        "/connections/synthetic-charts/check",
        json!({}),
    )
    .await;
    assert_eq!(status, 200, "{lapsed_check}");
    assert_eq!(lapsed_check["summary"]["scope"], "practice_wide_outage");
    assert_eq!(lapsed_check["summary"]["kind"]["kind"], "reconnect_needed");
    assert_eq!(
        lapsed_check["outages"][0]["kind"]["kind"],
        "reconnect_needed"
    );
    assert_eq!(lapsed_check["patient_issues"], json!([]));

    let (status, today_lapsed) = get_json(&base, &session, "/today").await;
    assert_eq!(status, 200, "{today_lapsed}");
    assert_eq!(
        today_lapsed["connections"][0]["condition"],
        "reconnect_required"
    );

    let (status, page1_lapsed) =
        get_json(&base, &session, &format!("/patient-workspaces/{ws1}")).await;
    assert_eq!(status, 200, "{page1_lapsed}");
    assert_eq!(
        page1_lapsed["connected_sources"][0]["standing"],
        "reconnect_needed"
    );
    assert_eq!(
        page1_lapsed["connected_sources"][0]["reconnect_needed"],
        true
    );
    assert_eq!(
        page1_lapsed["connected_sources"][0]["state"]["usable_snapshot"]["synchronized_at"],
        initial_iso
    );

    // Reconnect via `/commands` `reconnect_source` at reconnect_clock.
    let reconnect_clock: u64 = 1_787_860_800; // 2026-08-27T20:00:00.000Z
    let reconnect_iso = "2026-08-27T20:00:00.000Z";
    advance_practice_clock(&db_url, &practice_id, reconnect_clock).await;

    let (status, cmd_resp) = post_json(
        &base,
        &session,
        "/commands",
        json!({
            "type": "reconnect_source",
            "idempotency_key": format!("reconn-{run}"),
            "payload": {
                "source_id": SOURCE_ID
            }
        }),
    )
    .await;
    assert_eq!(status, 200, "{cmd_resp}");
    assert_eq!(cmd_resp["outcome"], "accepted");
    assert_eq!(cmd_resp["receipt"]["source_id"], SOURCE_ID);
    assert_eq!(cmd_resp["receipt"]["practice_id"], practice_id);
    assert_eq!(cmd_resp["access"]["state"], "connected");
    assert_eq!(cmd_resp["summary"]["scope"], "healthy");

    // Also verify `/connections/{source_id}/reconnect` directly after another lapse.
    let (status, _) = post_json(
        &base,
        &session,
        "/local-proof/sources/synthetic-charts/mode",
        json!({
            "mode": "lapsed_access",
            "reason": "token rotated again"
        }),
    )
    .await;
    assert_eq!(status, 200);
    let _ = post_json(
        &base,
        &session,
        "/connections/synthetic-charts/check",
        json!({}),
    )
    .await;
    let (status, direct_reconn) = post_json(
        &base,
        &session,
        "/connections/synthetic-charts/reconnect",
        json!({ "idempotency_key": format!("idem-direct-{run}") }),
    )
    .await;
    assert_eq!(status, 200, "{direct_reconn}");
    assert_eq!(direct_reconn["outcome"], "reconnected");
    assert_eq!(direct_reconn["access"]["state"], "connected");
    assert_eq!(direct_reconn["summary"]["scope"], "healthy");

    // Final healthy readback across /connections, /today, and /patient-workspaces/{ws1}.
    let (status, conns_recovered) = get_json(&base, &session, "/connections").await;
    assert_eq!(status, 200, "{conns_recovered}");
    assert_eq!(conns_recovered["outages"], json!([]));
    assert_eq!(conns_recovered["patient_issues"], json!([]));
    assert_eq!(
        conns_recovered["sources"][0]["access"]["state"],
        "connected"
    );
    assert_eq!(conns_recovered["sources"][0]["summary"]["scope"], "healthy");

    let (status, today_recovered) = get_json(&base, &session, "/today").await;
    assert_eq!(status, 200, "{today_recovered}");
    assert_eq!(today_recovered["connections"], json!([]));
    assert_eq!(today_recovered["source_outages"], json!([]));

    let (status, page1_recovered) =
        get_json(&base, &session, &format!("/patient-workspaces/{ws1}")).await;
    assert_eq!(status, 200, "{page1_recovered}");
    assert_eq!(
        page1_recovered["connected_sources"][0]["standing"],
        "up_to_date"
    );
    assert_eq!(
        page1_recovered["connected_sources"][0]["reconnect_needed"],
        false
    );
    assert_eq!(
        page1_recovered["connected_sources"][0]["state"]["state"],
        "usable"
    );
    assert_eq!(
        page1_recovered["connected_sources"][0]["state"]["synchronized_at"],
        reconnect_iso
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn local_proof_mode_endpoint_is_absent_when_local_proof_is_disabled() {
    let (auth, _runtime, db_url) = stores().await;
    let run = run_id();
    let practice_id = format!("practice-{}", run.to_lowercase());
    let email = format!("{}-guard@codahealth.test", run.to_lowercase());

    auth.ensure_practice(&practice_id, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.ensure_account(&practice_id, &run, &email, "Dr. Guard", true, 1)
        .await
        .unwrap();

    let server = common::serve_isolated(move || {
        // SAFETY: `serve_isolated` serializes server startup under its mutex.
        unsafe {
            std::env::set_var("PHYSICIAN_DATABASE_URL", &db_url);
            std::env::set_var("PHYSICIAN_AUTH", "required");
            std::env::remove_var("PHYSICIAN_LOCAL_PROOF");
            std::env::remove_var("PHYSICIAN_SYNTHETIC_CHARTS");
        }
    })
    .await;
    let base = format!("http://{}", server.address);
    let session = sign_in(&base, &email).await;

    let (status, _) = post_json(
        &base,
        &session,
        "/local-proof/sources/synthetic-charts/mode",
        json!({ "mode": "outage", "reason": "blocked" }),
    )
    .await;
    assert_eq!(status, 404);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn off_panel_colleague_sees_practice_outage_without_patient_failure_strings_and_reconnect_command_is_idempotent()
 {
    let (auth, runtime, db_url) = stores().await;
    let run = run_id();
    let practice_id = format!("practice-priv-{}", run.to_lowercase());
    let owner_email = format!("{}-owner@codahealth.test", run.to_lowercase());
    let colleague_email = format!("{}-offpanel@codahealth.test", run.to_lowercase());
    let colleague_id = format!("{run}-offpanel");
    let initial_clock: u64 = 1_787_846_400;

    auth.ensure_practice(&practice_id, PracticeClock::Fixed(initial_clock))
        .await
        .unwrap();
    auth.ensure_account(&practice_id, &run, &owner_email, "Dr. Owner", true, 1)
        .await
        .unwrap();
    auth.ensure_account(
        &practice_id,
        &colleague_id,
        &colleague_email,
        "Dr. OffPanel",
        false,
        2,
    )
    .await
    .unwrap();

    let bank = build_bank(&run, 3);
    let charts = SyntheticCharts::open(&bank);
    let summary = load_practice_from_source(&practice_id, &charts, &auth, &runtime)
        .await
        .unwrap();
    assert_eq!(summary.patients, 3);

    let owner_identity = PhysicianIdentity {
        practice_id: practice_id.clone(),
        physician_id: run.clone(),
    };
    for suffix in ["1", "2", "3"] {
        let ws = workspace_id(SOURCE_ID, &format!("{run}-{suffix}"));
        auth.assign_panel(&owner_identity, &run, &ws, Membership::Allowed)
            .await
            .unwrap();
    }

    let bank_for_server = bank.clone();
    let db_for_server = db_url.clone();
    let server = common::serve_isolated(move || {
        // SAFETY: `serve_isolated` serializes server startup under its mutex.
        unsafe {
            std::env::set_var("PHYSICIAN_DATABASE_URL", &db_for_server);
            std::env::set_var("PHYSICIAN_AUTH", "required");
            std::env::set_var("PHYSICIAN_LOCAL_PROOF", "1");
            std::env::set_var("PHYSICIAN_SYNTHETIC_CHARTS", &bank_for_server);
        }
    })
    .await;
    let base = format!("http://{}", server.address);
    let owner_session = sign_in(&base, &owner_email).await;
    let colleague_session = sign_in(&base, &colleague_email).await;

    // Fail 2 of 3 (>50%) off-panel patients with patient-identifying diagnostic strings.
    let secret_p1 = "SECRET-MRN-88412-corrupt-bundle";
    let secret_p2 = "SECRET-MRN-99501-locked-chart";
    for (suffix, secret) in [("1", secret_p1), ("2", secret_p2)] {
        let (status, _) = post_json(
            &base,
            &owner_session,
            "/local-proof/sources/synthetic-charts/mode",
            json!({
                "practice_id": practice_id,
                "source_patient_id": format!("{run}-{suffix}"),
                "mode": "outage",
                "reason": secret,
            }),
        )
        .await;
        assert_eq!(status, 200);
    }

    advance_practice_clock(&db_url, &practice_id, initial_clock + 3_600).await;
    let (status, _) = post_json(
        &base,
        &owner_session,
        "/connections/synthetic-charts/check",
        json!({}),
    )
    .await;
    assert_eq!(status, 200);

    // Off-panel colleague sees PracticeWideOutage on /connections and /today, with ZERO
    // occurrences of off-panel patient failure strings anywhere in either response body.
    let (status, offpanel_conns) = get_json(&base, &colleague_session, "/connections").await;
    assert_eq!(status, 200, "{offpanel_conns}");
    assert_eq!(offpanel_conns["outages"].as_array().unwrap().len(), 1);
    let conns_text = offpanel_conns.to_string();
    assert!(!conns_text.contains(secret_p1), "{conns_text}");
    assert!(!conns_text.contains(secret_p2), "{conns_text}");

    let (status, offpanel_today) = get_json(&base, &colleague_session, "/today").await;
    assert_eq!(status, 200, "{offpanel_today}");
    assert_eq!(offpanel_today["connections"].as_array().unwrap().len(), 1);
    let today_text = offpanel_today.to_string();
    assert!(!today_text.contains(secret_p1), "{today_text}");
    assert!(!today_text.contains(secret_p2), "{today_text}");

    // Clear per-patient outages via practice LapsedAccess, then verify `/commands` `reconnect_source`
    // enforces durable `idempotency_key` replay and conflict rules.
    let (status, _) = post_json(
        &base,
        &owner_session,
        "/local-proof/sources/synthetic-charts/mode",
        json!({
            "practice_id": practice_id,
            "mode": "lapsed_access",
            "reason": "practice token expired",
        }),
    )
    .await;
    assert_eq!(status, 200);

    // Missing `idempotency_key` is rejected.
    let (status, missing_key_resp) = post_json(
        &base,
        &owner_session,
        "/commands",
        json!({
            "type": "reconnect_source",
            "payload": { "source_id": SOURCE_ID }
        }),
    )
    .await;
    assert_eq!(status, 200, "{missing_key_resp}");
    assert_eq!(missing_key_resp["outcome"], "rejected");

    // First reconnect command with `idem_key` executes reconnect + practice check at t1.
    let t1 = initial_clock + 7_200;
    advance_practice_clock(&db_url, &practice_id, t1).await;
    let idem_key = format!("idem-reconn-{run}");
    let (status, first_cmd) = post_json(
        &base,
        &owner_session,
        "/commands",
        json!({
            "type": "reconnect_source",
            "idempotency_key": idem_key,
            "payload": { "source_id": SOURCE_ID }
        }),
    )
    .await;
    assert_eq!(status, 200, "{first_cmd}");
    assert_eq!(first_cmd["outcome"], "accepted");
    assert_eq!(first_cmd["receipt"]["reconnected_at"], t1 as i64);

    // Put adapter into practice Outage and advance clock to t2: a duplicate replay of the same
    // `idempotency_key` MUST NOT re-invoke `source.reconnect` (which would fail under Outage!)
    // or advance `reconnected_at` to t2; it returns the accepted receipt from t1.
    let (status, _) = post_json(
        &base,
        &owner_session,
        "/local-proof/sources/synthetic-charts/mode",
        json!({
            "practice_id": practice_id,
            "mode": "outage",
            "reason": "adapter refuses new reconnects right now",
        }),
    )
    .await;
    assert_eq!(status, 200);
    advance_practice_clock(&db_url, &practice_id, t1 + 3_600).await;

    let (status, replay_cmd) = post_json(
        &base,
        &owner_session,
        "/commands",
        json!({
            "type": "reconnect_source",
            "idempotency_key": idem_key,
            "payload": { "source_id": SOURCE_ID }
        }),
    )
    .await;
    assert_eq!(status, 200, "{replay_cmd}");
    assert_eq!(replay_cmd["outcome"], "accepted");
    assert_eq!(
        replay_cmd["receipt"]["reconnected_at"], t1 as i64,
        "duplicate idempotency_key replay must not re-run reconnect side effects"
    );

    // Restore healthy mode and run a SECOND reconnect under a fresh key at `t3` (advancing
    // `practice_source_access.connected_at` to `t3`), then verify replaying `idem_key` STILL
    // returns its original `reconnected_at == t1` rather than reconstructing `t3`!
    let t3 = t1 + 7_200;
    let (status, _) = post_json(
        &base,
        &owner_session,
        "/local-proof/sources/synthetic-charts/mode",
        json!({
            "practice_id": practice_id,
            "mode": "healthy",
        }),
    )
    .await;
    assert_eq!(status, 200);
    advance_practice_clock(&db_url, &practice_id, t3).await;

    let idem_key_2 = format!("idem-reconn-2-{run}");
    let (status, second_cmd) = post_json(
        &base,
        &owner_session,
        "/commands",
        json!({
            "type": "reconnect_source",
            "idempotency_key": idem_key_2,
            "payload": { "source_id": SOURCE_ID }
        }),
    )
    .await;
    assert_eq!(status, 200, "{second_cmd}");
    assert_eq!(second_cmd["outcome"], "accepted");
    assert_eq!(second_cmd["receipt"]["reconnected_at"], t3 as i64);

    let (status, replay_after_later) = post_json(
        &base,
        &owner_session,
        "/commands",
        json!({
            "type": "reconnect_source",
            "idempotency_key": idem_key,
            "payload": { "source_id": SOURCE_ID }
        }),
    )
    .await;
    assert_eq!(status, 200, "{replay_after_later}");
    assert_eq!(replay_after_later["outcome"], "accepted");
    assert_eq!(
        replay_after_later["receipt"]["reconnected_at"], t1 as i64,
        "replaying an older key after a later reconnect must return its original persisted receipt"
    );

    // Record a durable command receipt for a second source (`clinic-b-ehr`) under `other_source_key`,
    // then reuse `other_source_key` over HTTP `/commands` targeting the VALID, REGISTERED `SOURCE_ID`
    // (`synthetic-charts`) while `synthetic-charts` is healthy: must reject via receipt comparison!
    struct SecondEhr;
    #[async_trait::async_trait]
    impl local_product::physician_sources::PatientSource for SecondEhr {
        fn source_id(&self) -> &str {
            "clinic-b-ehr"
        }
        fn source_label(&self) -> &str {
            "Clinic B EHR"
        }
        async fn patient_ids(
            &self,
        ) -> Result<Vec<String>, local_product::physician_sources::SourceAdapterError> {
            Ok(Vec::new())
        }
        async fn patient(
            &self,
            id: &str,
        ) -> Result<
            local_product::physician_sources::SourcePatient,
            local_product::physician_sources::SourceAdapterError,
        > {
            Ok(local_product::physician_sources::SourcePatient {
                source_patient_id: id.to_owned(),
                display_name: id.to_owned(),
                birth_date: None,
                sex_or_gender: None,
                medical_record_numbers: Vec::new(),
            })
        }
        async fn records(
            &self,
            _patient: &local_product::physician_sources::SourcePatient,
            _workspace: &str,
        ) -> Result<
            Vec<local_product::physician_sources::SourceRevision>,
            local_product::physician_sources::SourceAdapterError,
        > {
            Ok(Vec::new())
        }
    }

    let other_source_key = format!("idem-other-src-{run}");
    let seeded = local_product::source_status::SourceCoordinator::reconnect_source_command(
        &runtime,
        local_product::source_status::ReconnectCommand {
            practice_id: &practice_id,
            physician_id: &run,
            idempotency_key: &other_source_key,
            source_id: "clinic-b-ehr",
            source: Some(&SecondEhr),
            now: t3 as i64,
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        seeded,
        local_product::source_status::ReconnectCommandOutcome::Accepted { .. }
    ));

    let (status, conflict_valid_source) = post_json(
        &base,
        &owner_session,
        "/commands",
        json!({
            "type": "reconnect_source",
            "idempotency_key": other_source_key,
            "payload": { "source_id": SOURCE_ID }
        }),
    )
    .await;
    assert_eq!(status, 200, "{conflict_valid_source}");
    assert_eq!(
        conflict_valid_source["outcome"], "rejected",
        "reusing an existing command key against a valid registered source must reject via receipt comparison"
    );

    // Also verify reusing `idem_key` against an unregistered source rejects via receipt comparison.
    let (status, conflict_unregistered) = post_json(
        &base,
        &owner_session,
        "/commands",
        json!({
            "type": "reconnect_source",
            "idempotency_key": idem_key,
            "payload": { "source_id": "other-ehr" }
        }),
    )
    .await;
    assert_eq!(status, 200, "{conflict_unregistered}");
    assert_eq!(conflict_unregistered["outcome"], "rejected");
}

struct FailingEnrollmentAuth {
    inner: PostgresAuthStore,
    fail_register: std::sync::atomic::AtomicBool,
}

#[async_trait::async_trait]
impl AuthStore for FailingEnrollmentAuth {
    fn authority_changes(
        &self,
    ) -> Option<physician_auth::watch::Receiver<physician_auth::AuthorityFeed>> {
        self.inner.authority_changes()
    }
    async fn begin_sign_in(
        &self,
        email: &str,
        now: u64,
    ) -> Result<String, physician_auth::SignInError> {
        self.inner.begin_sign_in(email, now).await
    }
    async fn complete_sign_in(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(PhysicianIdentity, String), physician_auth::SignInError> {
        self.inner.complete_sign_in(email, token, now).await
    }
    async fn mint_fixed_session(
        &self,
        identity: &PhysicianIdentity,
        now: u64,
        ends_at: u64,
    ) -> Result<String, physician_auth::SignInError> {
        self.inner.mint_fixed_session(identity, now, ends_at).await
    }
    async fn verified_session(
        &self,
        token: &str,
        now: u64,
    ) -> Result<physician_auth::Session, physician_auth::SessionError> {
        self.inner.verified_session(token, now).await
    }
    async fn note_physician_activity(
        &self,
        token: &str,
        now: u64,
    ) -> Result<physician_auth::SessionActivity, physician_auth::StoreUnavailable> {
        self.inner.note_physician_activity(token, now).await
    }
    async fn publish_authorized(
        &self,
        token: &str,
        identity: &PhysicianIdentity,
        workspace: &str,
        now: u64,
        publish: &mut (dyn FnMut() -> Result<(), String> + Send),
    ) -> Result<(), String> {
        self.inner
            .publish_authorized(token, identity, workspace, now, publish)
            .await
    }
    async fn end_session(&self, token: &str) -> Result<(), physician_auth::StoreUnavailable> {
        self.inner.end_session(token).await
    }
    async fn account_by_email(&self, email: &str) -> Option<physician_auth::Account> {
        self.inner.account_by_email(email).await
    }
    async fn account_by_identity(
        &self,
        identity: &PhysicianIdentity,
    ) -> Option<physician_auth::Account> {
        self.inner.account_by_identity(identity).await
    }
    async fn membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Membership {
        self.inner.membership(identity, patient_workspace_id).await
    }
    async fn set_membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
        membership: Membership,
    ) -> Result<(), physician_auth::StoreUnavailable> {
        self.inner
            .set_membership(identity, patient_workspace_id, membership)
            .await
    }
    async fn place_on_panel(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Result<(), physician_auth::StoreUnavailable> {
        self.inner
            .place_on_panel(identity, patient_workspace_id)
            .await
    }
    async fn panel(&self, identity: &PhysicianIdentity) -> Vec<String> {
        self.inner.panel(identity).await
    }
    async fn assign_panel(
        &self,
        actor: &PhysicianIdentity,
        physician_id: &str,
        patient_workspace_id: &str,
        membership: Membership,
    ) -> Result<(), physician_auth::PanelError> {
        self.inner
            .assign_panel(actor, physician_id, patient_workspace_id, membership)
            .await
    }
    async fn ensure_practice(
        &self,
        practice_id: &str,
        clock: PracticeClock,
    ) -> Result<(), physician_auth::StoreUnavailable> {
        self.inner.ensure_practice(practice_id, clock).await
    }
    async fn practice_clock(
        &self,
        practice_id: &str,
    ) -> Result<PracticeClock, physician_auth::StoreUnavailable> {
        self.inner.practice_clock(practice_id).await
    }
    async fn register_patient(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Result<(), physician_auth::StoreUnavailable> {
        if self.fail_register.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(physician_auth::StoreUnavailable);
        }
        self.inner
            .register_patient(practice_id, patient_workspace_id)
            .await
    }
    async fn patient_practice(
        &self,
        patient_workspace_id: &str,
    ) -> Result<Option<String>, physician_auth::StoreUnavailable> {
        self.inner.patient_practice(patient_workspace_id).await
    }
    async fn practice_patients(
        &self,
        practice_id: &str,
    ) -> Result<Vec<String>, physician_auth::StoreUnavailable> {
        self.inner.practice_patients(practice_id).await
    }
    async fn practice_accounts(
        &self,
        practice_id: &str,
    ) -> Result<Vec<physician_auth::Account>, physician_auth::StoreUnavailable> {
        self.inner.practice_accounts(practice_id).await
    }
    async fn invite(
        &self,
        practice_id: &str,
        email: &str,
        display_name: &str,
        admin: bool,
        now: u64,
    ) -> Result<String, physician_auth::SignInError> {
        self.inner
            .invite(practice_id, email, display_name, admin, now)
            .await
    }
    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(PhysicianIdentity, String), physician_auth::SignInError> {
        self.inner.redeem_invitation(email, token, now).await
    }
    async fn deactivate(&self, email: &str) -> bool {
        self.inner.deactivate(email).await
    }
    async fn reactivate(&self, email: &str) -> bool {
        self.inner.reactivate(email).await
    }
    async fn remove_account(&self, email: &str) -> bool {
        self.inner.remove_account(email).await
    }
    async fn invite_patient(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        now: u64,
    ) -> Result<String, physician_auth::SignInError> {
        self.inner
            .invite_patient(practice_id, patient_workspace_id, now)
            .await
    }
    async fn accept_patient_invitation(
        &self,
        token: &str,
        duet_account_id: &str,
        now: u64,
    ) -> Result<physician_auth::PatientLink, physician_auth::SignInError> {
        self.inner
            .accept_patient_invitation(token, duet_account_id, now)
            .await
    }
    async fn grant_patient_consent(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        duet_account_id: &str,
        consent_version: &str,
        now: u64,
    ) -> Result<physician_auth::PatientLink, physician_auth::SignInError> {
        self.inner
            .grant_patient_consent(
                practice_id,
                patient_workspace_id,
                duet_account_id,
                consent_version,
                now,
            )
            .await
    }
    async fn revoke_patient_consent(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        now: u64,
    ) -> Result<physician_auth::PatientLink, physician_auth::SignInError> {
        self.inner
            .revoke_patient_consent(practice_id, patient_workspace_id, now)
            .await
    }
    async fn patient_link(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Option<physician_auth::PatientLink> {
        self.inner
            .patient_link(practice_id, patient_workspace_id)
            .await
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn check_and_reconnect_routes_propagate_enrollment_failures_as_500_and_repair_on_retry() {
    use std::sync::Arc;
    use std::sync::atomic::Ordering::SeqCst;

    let (auth, runtime, db_url) = stores().await;
    let run = run_id();
    let practice_id = format!("practice-enr-{}", run.to_lowercase());
    let email = format!("enr-{}@codahealth.test", run.to_lowercase());
    let fixed_clock: u64 = 1_788_100_000;

    auth.ensure_practice(&practice_id, PracticeClock::Fixed(fixed_clock))
        .await
        .unwrap();
    auth.ensure_account(&practice_id, &run, &email, "Dr. Enroll", true, 1)
        .await
        .unwrap();

    let bank = build_bank(&run, 1);
    let pat_id = format!("{run}-1");
    let ws = workspace_id(SOURCE_ID, &pat_id);
    let admin = PhysicianIdentity {
        practice_id: practice_id.clone(),
        physician_id: run.clone(),
    };
    // Grant panel membership without pre-registering `ws` in `physician_patients` so the HTTP
    // check/reconnect handlers exercise `enroll_identified_patients` -> `auth.register_patient`.
    auth.set_membership(&admin, &ws, Membership::Allowed)
        .await
        .unwrap();
    assert_eq!(auth.patient_practice(&ws).await.unwrap(), None);

    let failing_auth = Arc::new(FailingEnrollmentAuth {
        inner: auth.clone(),
        fail_register: std::sync::atomic::AtomicBool::new(false),
    });
    let front_door = front_door::PostgresFrontDoorStore::connect_with_migrations(&db_url)
        .await
        .unwrap();
    let registry = physician_sample::sources::SourceRegistry::new(Some(runtime.clone()));
    registry.register_synthetic(Arc::new(SyntheticCharts::open(&bank)));

    let app = physician_sample::authed_router_on(
        physician_sample::fixture_site().layer(axum::Extension(registry)),
        Arc::clone(&failing_auth) as Arc<dyn AuthStore>,
        Arc::new(front_door),
    )
    .await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server_task = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    let base = format!("http://{addr}");
    let session = sign_in(&base, &email).await;

    // Induce `EnrollmentError::RegisterPatient` on all four check/reconnect HTTP paths:
    // none may return 200 OK while leaving the newly identified patient unenrolled!
    failing_auth.fail_register.store(true, SeqCst);

    let (status_single_check, _) = post_json(
        &base,
        &session,
        "/connections/synthetic-charts/check",
        json!({ "patient_workspace_id": ws, "source_patient_id": pat_id }),
    )
    .await;
    assert_eq!(status_single_check, 500);
    assert_eq!(auth.patient_practice(&ws).await.unwrap(), None);

    let (status_practice_check, _) = post_json(
        &base,
        &session,
        "/connections/synthetic-charts/check",
        json!({}),
    )
    .await;
    assert_eq!(status_practice_check, 500);

    let idem_direct = format!("idem-direct-enr-{run}");
    let (status_reconnect, _) = post_json(
        &base,
        &session,
        "/connections/synthetic-charts/reconnect",
        json!({ "idempotency_key": idem_direct }),
    )
    .await;
    assert_eq!(status_reconnect, 500);
    assert_eq!(auth.patient_practice(&ws).await.unwrap(), None);

    let idem = format!("idem-enr-{run}");
    let (status_command_fail, _) = post_json(
        &base,
        &session,
        "/commands",
        json!({
            "type": "reconnect_source",
            "idempotency_key": idem,
            "payload": { "source_id": SOURCE_ID }
        }),
    )
    .await;
    assert_eq!(status_command_fail, 500);
    assert_eq!(auth.patient_practice(&ws).await.unwrap(), None);

    // Clear the enrollment failure and retry BOTH keys after advancing the practice clock:
    // each idempotent replay repairs directory enrollment and returns its original receipt timestamp!
    failing_auth.fail_register.store(false, SeqCst);
    advance_practice_clock(&db_url, &practice_id, fixed_clock + 600).await;

    let (status_direct_ok, direct_ok_body) = post_json(
        &base,
        &session,
        "/connections/synthetic-charts/reconnect",
        json!({ "idempotency_key": idem_direct }),
    )
    .await;
    assert_eq!(status_direct_ok, 200, "{direct_ok_body}");
    assert_eq!(direct_ok_body["outcome"], "reconnected");
    assert_eq!(
        direct_ok_body["receipt"]["reconnected_at"],
        fixed_clock as i64
    );
    assert_eq!(
        auth.patient_practice(&ws).await.unwrap(),
        Some(practice_id.clone())
    );

    let (status_command_ok, ok_body) = post_json(
        &base,
        &session,
        "/commands",
        json!({
            "type": "reconnect_source",
            "idempotency_key": idem,
            "payload": { "source_id": SOURCE_ID }
        }),
    )
    .await;
    assert_eq!(status_command_ok, 200, "{ok_body}");
    assert_eq!(ok_body["outcome"], "accepted");
    assert_eq!(ok_body["receipt"]["reconnected_at"], fixed_clock as i64);
    assert_eq!(auth.patient_practice(&ws).await.unwrap(), Some(practice_id));

    server_task.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn pre_checkpoint_interrupted_reconnect_recovers_original_receipt_across_fresh_synthetic_charts_instance()
 {
    use std::sync::Arc;

    use local_product::physician_sources::{
        PatientSource, ReconnectReceipt, SourceAdapterError, SourceCommandKey, SourcePatient,
        SourcePullOutput, SourceRevision,
    };
    use local_product::source_status::{ReconnectCommand, SourceAccessState, SourceCoordinator};
    use physician_sample::synthetic_chart::SyntheticSourceMode;

    struct SiteACrashBeforeCheckpoint {
        inner: SyntheticCharts,
        committed: tokio::sync::Notify,
    }

    #[async_trait::async_trait]
    impl PatientSource for SiteACrashBeforeCheckpoint {
        fn source_id(&self) -> &str {
            self.inner.source_id()
        }

        fn source_label(&self) -> &str {
            self.inner.source_label()
        }

        async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
            self.inner.patient_ids().await
        }

        async fn patient(
            &self,
            source_patient_id: &str,
        ) -> Result<SourcePatient, SourceAdapterError> {
            self.inner.patient(source_patient_id).await
        }

        async fn records(
            &self,
            patient: &SourcePatient,
            workspace: &str,
        ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
            self.inner.records(patient, workspace).await
        }

        async fn pull_patient(
            &self,
            practice_id: &str,
            source_patient_id: &str,
            workspace: &str,
        ) -> SourcePullOutput {
            self.inner
                .pull_patient(practice_id, source_patient_id, workspace)
                .await
        }

        async fn reconnect(
            &self,
            practice_id: &str,
            now: i64,
            command_key: Option<SourceCommandKey<'_>>,
        ) -> Result<ReconnectReceipt, SourceAdapterError> {
            let receipt = self.inner.reconnect(practice_id, now, command_key).await?;
            self.committed.notify_waiters();
            std::future::pending::<()>().await;
            Ok(receipt)
        }
    }

    let (auth, runtime, db_url) = stores().await;
    let run = run_id();
    let practice_id = format!("practice-failover-{}", run.to_lowercase());
    let email = format!("failover-{}@codahealth.test", run.to_lowercase());
    let t_initial: u64 = 1_788_200_000;
    let t_site_a: i64 = 1_788_201_000;
    let t_site_b: u64 = 1_788_205_000;

    auth.ensure_practice(&practice_id, PracticeClock::Fixed(t_initial))
        .await
        .unwrap();
    auth.ensure_account(&practice_id, &run, &email, "Dr. Failover", true, 1)
        .await
        .unwrap();

    // Give Site A and Site B completely separate bank directories on disk so they share ONLY Postgres!
    let bank_dir_a = build_bank_at(&format!("{run}-site-a"), &run, 2);
    let bank_dir_b = build_bank_at(&format!("{run}-site-b"), &run, 2);
    assert_ne!(bank_dir_a, bank_dir_b);

    {
        let site_a_charts = SyntheticCharts::open_with_store(&bank_dir_a, runtime.clone());
        let loaded = load_practice_from_source(&practice_id, &site_a_charts, &auth, &runtime)
            .await
            .unwrap();
        assert_eq!(loaded.patients, 2);

        let admin = PhysicianIdentity {
            practice_id: practice_id.clone(),
            physician_id: run.clone(),
        };
        for suffix in ["1", "2"] {
            let ws = workspace_id(SOURCE_ID, &format!("{run}-{suffix}"));
            auth.assign_panel(&admin, &run, &ws, Membership::Allowed)
                .await
                .unwrap();
        }

        site_a_charts
            .set_practice_mode(
                &practice_id,
                SyntheticSourceMode::LapsedAccess {
                    reason: "site A refresh token expired".into(),
                },
            )
            .await
            .unwrap();
        runtime
            .set_practice_source_access(
                &practice_id,
                SOURCE_ID,
                &SourceAccessState::ReconnectNeeded {
                    since: t_initial as i64 + 500,
                    reason: "site A refresh token expired".into(),
                },
                t_initial as i64 + 500,
            )
            .await
            .unwrap();

        let idem = format!("idem-failover-{run}");
        let site_a_adapter = Arc::new(SiteACrashBeforeCheckpoint {
            inner: site_a_charts,
            committed: tokio::sync::Notify::new(),
        });
        let committed = site_a_adapter.committed.notified();
        let runtime_bg = runtime.clone();
        let practice_bg = practice_id.clone();
        let physician_bg = run.clone();
        let idem_bg = idem.clone();
        let adapter_bg = Arc::clone(&site_a_adapter);

        let site_a_task = tokio::spawn(async move {
            SourceCoordinator::reconnect_source_command(
                &runtime_bg,
                ReconnectCommand {
                    practice_id: &practice_bg,
                    physician_id: &physician_bg,
                    idempotency_key: &idem_bg,
                    source_id: SOURCE_ID,
                    source: Some(&*adapter_bg),
                    now: t_site_a,
                },
            )
            .await
        });

        committed.await;
        assert_eq!(
            site_a_adapter
                .inner
                .reconnect_effect_count(&practice_id)
                .await
                .unwrap(),
            1,
            "Site A performed exactly one reconnect effect before crashing"
        );
        site_a_task.abort();
        let _ = site_a_task.await;
    }

    // Delete Site A's bank directory completely so Site B cannot observe anything from Site A's filesystem.
    fs::remove_dir_all(&bank_dir_a).unwrap();
    assert!(!bank_dir_a.exists());

    // Site A's adapter instance has been dropped and the DB checkpoint never ran (`ReconnectNeeded`
    // is still stored in Postgres).
    let pre_retry_report = runtime
        .practice_source_report(&practice_id, SOURCE_ID)
        .await
        .unwrap();
    assert!(matches!(
        pre_retry_report.access,
        Some(SourceAccessState::ReconnectNeeded { .. })
    ));

    // Boot Site B with its own isolated `bank_dir_b` (`bank_dir_b != bank_dir_a`), sharing ONLY
    // `db_url`, advance the practice clock to `t_site_b`, and retry the same `idempotency_key`
    // over HTTP `/commands`.
    let bank_for_site_b = bank_dir_b.clone();
    let db_for_site_b = db_url.clone();
    let site_b = common::serve_isolated(move || {
        // SAFETY: `serve_isolated` serializes server startup under its mutex.
        unsafe {
            std::env::set_var("PHYSICIAN_DATABASE_URL", &db_for_site_b);
            std::env::set_var("PHYSICIAN_AUTH", "required");
            std::env::set_var("PHYSICIAN_LOCAL_PROOF", "1");
            std::env::set_var("PHYSICIAN_SYNTHETIC_CHARTS", &bank_for_site_b);
        }
    })
    .await;
    let base_b = format!("http://{}", site_b.address);
    let session_b = sign_in(&base_b, &email).await;
    advance_practice_clock(&db_url, &practice_id, t_site_b).await;

    let idem = format!("idem-failover-{run}");
    let (status, site_b_resp) = post_json(
        &base_b,
        &session_b,
        "/commands",
        json!({
            "type": "reconnect_source",
            "idempotency_key": idem,
            "payload": { "source_id": SOURCE_ID }
        }),
    )
    .await;
    assert_eq!(status, 200, "{site_b_resp}");
    assert_eq!(site_b_resp["outcome"], "accepted");
    assert_eq!(
        site_b_resp["receipt"]["reconnected_at"], t_site_a,
        "Site B retry must recover Site A's original receipt timestamp rather than minting t_site_b"
    );
    assert_eq!(
        SyntheticCharts::open_with_store(&bank_dir_b, runtime.clone())
            .reconnect_effect_count(&practice_id)
            .await
            .unwrap(),
        1,
        "Site B retry must not repeat the underlying reconnect effect"
    );

    let (conns_status, conns_body) = get_json(&base_b, &session_b, "/connections").await;
    assert_eq!(conns_status, 200, "{conns_body}");
    assert_eq!(conns_body["outages"], json!([]));
    assert_eq!(conns_body["sources"][0]["summary"]["scope"], "healthy");
    assert_eq!(conns_body["sources"][0]["summary"]["total_patients"], 2);

    let _ = fs::remove_dir_all(&bank_dir_b);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn newly_linked_pull_less_source_standing_projects_on_identified_workspace_without_enrolling_unidentified()
 {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use local_product::physician_sources::{
        FailedPart, PatientSource, ReconnectReceipt, SourceAdapterError, SourceCommandKey,
        SourcePatient, SourcePullOutput, SourceRevision,
    };
    use physician_sample::patients::PatientDirectory;

    #[derive(Clone)]
    enum SecondSourceBehavior {
        Failed(String),
        PartialWithoutIdentity(String),
        Lapsed(String),
    }

    struct ConfigurableSecondSource {
        source_id: String,
        source_label: String,
        behaviors: Mutex<HashMap<String, SecondSourceBehavior>>,
    }

    impl ConfigurableSecondSource {
        fn new(source_id: &str, source_label: &str) -> Arc<Self> {
            Arc::new(Self {
                source_id: source_id.to_owned(),
                source_label: source_label.to_owned(),
                behaviors: Mutex::new(HashMap::new()),
            })
        }

        fn set_behavior(&self, source_patient_id: &str, behavior: SecondSourceBehavior) {
            self.behaviors
                .lock()
                .unwrap()
                .insert(source_patient_id.to_owned(), behavior);
        }
    }

    #[async_trait::async_trait]
    impl PatientSource for ConfigurableSecondSource {
        fn source_id(&self) -> &str {
            &self.source_id
        }

        fn source_label(&self) -> &str {
            &self.source_label
        }

        async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
            Ok(Vec::new())
        }

        async fn patient(
            &self,
            source_patient_id: &str,
        ) -> Result<SourcePatient, SourceAdapterError> {
            Err(SourceAdapterError::Unavailable {
                reason: format!("unknown patient {source_patient_id}"),
            })
        }

        async fn records(
            &self,
            _patient: &SourcePatient,
            _workspace: &str,
        ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
            Ok(Vec::new())
        }

        async fn pull_patient(
            &self,
            _practice_id: &str,
            source_patient_id: &str,
            _workspace: &str,
        ) -> SourcePullOutput {
            let behavior = self
                .behaviors
                .lock()
                .unwrap()
                .get(source_patient_id)
                .cloned()
                .unwrap_or_else(|| SecondSourceBehavior::Failed("unconfigured".to_owned()));
            match behavior {
                SecondSourceBehavior::Failed(msg) => SourcePullOutput::Failed {
                    error: SourceAdapterError::Unavailable { reason: msg },
                },
                SecondSourceBehavior::PartialWithoutIdentity(msg) => SourcePullOutput::Partial {
                    patient: None,
                    records: Vec::new(),
                    succeeded_kinds: Vec::new(),
                    failed_parts: vec![FailedPart {
                        part: "Observation".to_owned(),
                        reason: msg,
                    }],
                },
                SecondSourceBehavior::Lapsed(reason) => SourcePullOutput::LapsedAccess {
                    error: SourceAdapterError::LapsedAccess { reason },
                },
            }
        }

        async fn reconnect(
            &self,
            practice_id: &str,
            now: i64,
            _command_key: Option<SourceCommandKey<'_>>,
        ) -> Result<ReconnectReceipt, SourceAdapterError> {
            Ok(ReconnectReceipt {
                source_id: self.source_id.clone(),
                practice_id: practice_id.to_owned(),
                reconnected_at: now,
            })
        }
    }

    let (auth, runtime, db_url) = stores().await;
    let run = run_id();
    let practice_id = format!("practice-multi-{}", run.to_lowercase());
    let email = format!("multi-{}@codahealth.test", run.to_lowercase());
    let t0: u64 = 1_788_300_000;

    auth.ensure_practice(&practice_id, PracticeClock::Fixed(t0))
        .await
        .unwrap();
    auth.ensure_account(&practice_id, &run, &email, "Dr. Multi", true, 1)
        .await
        .unwrap();

    // Load two identified patients from Source A (`synthetic-charts`).
    let bank = build_bank(&run, 2);
    let charts = Arc::new(SyntheticCharts::open_with_store(&bank, runtime.clone()));
    let loaded = load_practice_from_source(&practice_id, &*charts, &auth, &runtime)
        .await
        .unwrap();
    assert_eq!(loaded.patients, 2);

    let ws1 = workspace_id(SOURCE_ID, &format!("{run}-1"));
    let ws2 = workspace_id(SOURCE_ID, &format!("{run}-2"));
    let ws_unidentified = workspace_id("regional-labs", "lab-only-99");
    let admin = PhysicianIdentity {
        practice_id: practice_id.clone(),
        physician_id: run.clone(),
    };
    for ws in [&ws1, &ws2, &ws_unidentified] {
        auth.set_membership(&admin, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    let second_source = ConfigurableSecondSource::new("regional-labs", "Regional Reference Labs");
    let registry = physician_sample::sources::SourceRegistry::new(Some(runtime.clone()));
    registry.register_synthetic(Arc::clone(&charts));
    registry.register(Arc::clone(&second_source) as Arc<dyn PatientSource>);

    let front_door = front_door::PostgresFrontDoorStore::connect_with_migrations(&db_url)
        .await
        .unwrap();
    let directory: Arc<dyn PatientDirectory> = Arc::new(runtime.clone());
    let app = physician_sample::authed_router_on(
        physician_sample::fixture_site()
            .layer(axum::Extension(registry))
            .layer(axum::Extension(directory)),
        Arc::new(auth.clone()) as Arc<dyn AuthStore>,
        Arc::new(front_door),
    )
    .await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server_task = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    let base = format!("http://{addr}");
    let session = sign_in(&base, &email).await;

    // 1. Brand-new workspace whose ONLY source fails (no pull row anywhere) stays unenrolled and 404!
    second_source.set_behavior(
        "lab-only-99",
        SecondSourceBehavior::Failed("initial lab feed down".to_owned()),
    );
    let (chk_unid_status, chk_unid_body) = post_json(
        &base,
        &session,
        "/connections/regional-labs/check",
        json!({ "source_patient_id": "lab-only-99" }),
    )
    .await;
    assert_eq!(chk_unid_status, 200, "{chk_unid_body}");
    assert_eq!(
        auth.patient_practice(&ws_unidentified).await.unwrap(),
        None,
        "unidentified workspace whose first source check fails must remain unenrolled"
    );
    let (unid_ws_status, _) = get_json(
        &base,
        &session,
        &format!("/patient-workspaces/{ws_unidentified}"),
    )
    .await;
    assert_eq!(unid_ws_status, 404);

    // 2. Already-identified `ws1` links Source B (`regional-labs`), whose first pull fails (`Failed`).
    advance_practice_clock(&db_url, &practice_id, t0 + 120).await;
    second_source.set_behavior(
        "lab-p1",
        SecondSourceBehavior::Failed("upstream lab gateway timeout".to_owned()),
    );
    let (chk_fail_status, chk_fail_body) = post_json(
        &base,
        &session,
        "/connections/regional-labs/check",
        json!({
            "patient_workspace_id": ws1,
            "source_patient_id": "lab-p1"
        }),
    )
    .await;
    assert_eq!(chk_fail_status, 200, "{chk_fail_body}");
    let pulls_after_fail = runtime
        .source_pulls(std::slice::from_ref(&ws1))
        .await
        .unwrap();
    assert_eq!(
        pulls_after_fail[&ws1].len(),
        1,
        "failed first check of newly linked source B writes no source_pull row"
    );

    let (ws1_status, ws1_failed) =
        get_json(&base, &session, &format!("/patient-workspaces/{ws1}")).await;
    assert_eq!(ws1_status, 200, "{ws1_failed}");
    let sources_failed = ws1_failed["connected_sources"].as_array().unwrap();
    assert_eq!(sources_failed.len(), 2, "{ws1_failed}");
    let lab_failed = sources_failed
        .iter()
        .find(|s| s["source_id"] == "regional-labs")
        .expect("newly linked failing source B must appear in connected_sources");
    assert_eq!(lab_failed["source_label"], "Regional Reference Labs");
    assert_eq!(lab_failed["standing"], "failed");
    assert_eq!(lab_failed["reconnect_needed"], false);
    assert_eq!(lab_failed["last_good_read_at"], Value::Null);
    assert!(
        lab_failed["reason"]
            .as_str()
            .unwrap()
            .contains("upstream lab gateway timeout")
    );
    assert_eq!(lab_failed["state"]["state"], "failed");

    // 3. Partial pull without patient identity (`Partial` with no pull row) also projects on `ws1`.
    advance_practice_clock(&db_url, &practice_id, t0 + 240).await;
    second_source.set_behavior(
        "lab-p1",
        SecondSourceBehavior::PartialWithoutIdentity("chemistry panel feed timeout".to_owned()),
    );
    let (chk_part_status, chk_part_body) = post_json(
        &base,
        &session,
        "/connections/regional-labs/check",
        json!({ "patient_workspace_id": ws1 }),
    )
    .await;
    assert_eq!(chk_part_status, 200, "{chk_part_body}");
    assert_eq!(
        runtime
            .source_pulls(std::slice::from_ref(&ws1))
            .await
            .unwrap()[&ws1]
            .len(),
        1,
        "partial pull without patient identity still writes no source_pull row"
    );
    let (ws1_status, ws1_partial) =
        get_json(&base, &session, &format!("/patient-workspaces/{ws1}")).await;
    assert_eq!(ws1_status, 200, "{ws1_partial}");
    let lab_partial = ws1_partial["connected_sources"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["source_id"] == "regional-labs")
        .unwrap();
    assert_eq!(lab_partial["source_label"], "Regional Reference Labs");
    assert_eq!(lab_partial["standing"], "partial");
    assert_eq!(lab_partial["reconnect_needed"], false);
    assert_eq!(lab_partial["last_good_read_at"], Value::Null);
    assert_eq!(lab_partial["failed_parts"].as_array().unwrap().len(), 1);

    // 4. Lapsed access (`ReconnectNeeded` with no pull row) projects on `ws1` while `ws2` (never
    // linked to `regional-labs`) stays strictly scoped to its own linked source (`synthetic-charts`).
    advance_practice_clock(&db_url, &practice_id, t0 + 360).await;
    second_source.set_behavior(
        "lab-p1",
        SecondSourceBehavior::Lapsed("regional lab token revoked".to_owned()),
    );
    let (chk_lapsed_status, chk_lapsed_body) = post_json(
        &base,
        &session,
        "/connections/regional-labs/check",
        json!({ "patient_workspace_id": ws1 }),
    )
    .await;
    assert_eq!(chk_lapsed_status, 200, "{chk_lapsed_body}");

    let (ws1_status, ws1_lapsed) =
        get_json(&base, &session, &format!("/patient-workspaces/{ws1}")).await;
    assert_eq!(ws1_status, 200, "{ws1_lapsed}");
    let lab_lapsed = ws1_lapsed["connected_sources"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["source_id"] == "regional-labs")
        .unwrap();
    assert_eq!(lab_lapsed["source_label"], "Regional Reference Labs");
    assert_eq!(lab_lapsed["standing"], "reconnect_needed");
    assert_eq!(lab_lapsed["reconnect_needed"], true);
    assert_eq!(lab_lapsed["reason"], "regional lab token revoked");
    assert_eq!(lab_lapsed["last_good_read_at"], Value::Null);

    let (ws2_status, ws2_body) =
        get_json(&base, &session, &format!("/patient-workspaces/{ws2}")).await;
    assert_eq!(ws2_status, 200, "{ws2_body}");
    let ws2_sources = ws2_body["connected_sources"].as_array().unwrap();
    assert_eq!(
        ws2_sources.len(),
        1,
        "unlinked patient ws2 must not inherit practice-level lapsed standing of regional-labs: {ws2_body}"
    );
    assert_eq!(ws2_sources[0]["source_id"], SOURCE_ID);
    assert_eq!(ws2_sources[0]["state"]["state"], "usable");

    server_task.abort();
    let _ = fs::remove_dir_all(&bank);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn post_connections_reconnect_requires_key_replays_after_500_and_deduplicates_across_instances()
 {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

    use local_product::physician_sources::{
        PatientSource, ReconnectReceipt, SourceAdapterError, SourceCommandKey, SourcePatient,
        SourcePullOutput, SourceRevision,
    };
    use physician_sample::synthetic_chart::SyntheticSourceMode;

    struct CountingSecondSource {
        source_id: String,
        reconnects: AtomicUsize,
    }

    #[async_trait::async_trait]
    impl PatientSource for CountingSecondSource {
        fn source_id(&self) -> &str {
            &self.source_id
        }

        fn source_label(&self) -> &str {
            "Secondary Hospital Feed"
        }

        async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
            Ok(Vec::new())
        }

        async fn patient(
            &self,
            source_patient_id: &str,
        ) -> Result<SourcePatient, SourceAdapterError> {
            Err(SourceAdapterError::Unavailable {
                reason: format!("unknown patient {source_patient_id}"),
            })
        }

        async fn records(
            &self,
            _patient: &SourcePatient,
            _workspace: &str,
        ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
            Ok(Vec::new())
        }

        async fn pull_patient(
            &self,
            _practice_id: &str,
            source_patient_id: &str,
            _workspace: &str,
        ) -> SourcePullOutput {
            SourcePullOutput::Failed {
                error: SourceAdapterError::Unavailable {
                    reason: format!("unknown patient {source_patient_id}"),
                },
            }
        }

        async fn reconnect(
            &self,
            practice_id: &str,
            now: i64,
            _command_key: Option<SourceCommandKey<'_>>,
        ) -> Result<ReconnectReceipt, SourceAdapterError> {
            self.reconnects.fetch_add(1, SeqCst);
            Ok(ReconnectReceipt {
                source_id: self.source_id.clone(),
                practice_id: practice_id.to_owned(),
                reconnected_at: now,
            })
        }
    }

    let (auth, runtime, db_url) = stores().await;
    let run = run_id();
    let practice_id = format!("practice-direct-reconn-{}", run.to_lowercase());
    let email = format!("direct-reconn-{}@codahealth.test", run.to_lowercase());
    let t1: u64 = 1_788_400_000;
    let t2: u64 = 1_788_400_600;
    let t3: u64 = 1_788_401_200;

    auth.ensure_practice(&practice_id, PracticeClock::Fixed(t1))
        .await
        .unwrap();
    auth.ensure_account(&practice_id, &run, &email, "Dr. Reconnect", true, 1)
        .await
        .unwrap();

    let bank_a = build_bank_at(&format!("{run}-reconn-a"), &run, 1);
    let bank_b = build_bank_at(&format!("{run}-reconn-b"), &run, 1);
    let pat_id = format!("{run}-1");
    let ws = workspace_id(SOURCE_ID, &pat_id);
    let admin = PhysicianIdentity {
        practice_id: practice_id.clone(),
        physician_id: run.clone(),
    };
    auth.set_membership(&admin, &ws, Membership::Allowed)
        .await
        .unwrap();

    let failing_auth = Arc::new(FailingEnrollmentAuth {
        inner: auth.clone(),
        fail_register: std::sync::atomic::AtomicBool::new(false),
    });
    let charts_a = Arc::new(SyntheticCharts::open_with_store(&bank_a, runtime.clone()));
    charts_a
        .set_practice_mode(
            &practice_id,
            SyntheticSourceMode::LapsedAccess {
                reason: "token expired".into(),
            },
        )
        .await
        .unwrap();

    let secondary_a = Arc::new(CountingSecondSource {
        source_id: "secondary-feed".to_owned(),
        reconnects: AtomicUsize::new(0),
    });
    let registry_a = physician_sample::sources::SourceRegistry::new(Some(runtime.clone()));
    registry_a.register_synthetic(Arc::clone(&charts_a));
    registry_a.register(Arc::clone(&secondary_a) as Arc<dyn PatientSource>);
    let front_door_a = front_door::PostgresFrontDoorStore::connect_with_migrations(&db_url)
        .await
        .unwrap();
    let app_a = physician_sample::authed_router_on(
        physician_sample::fixture_site().layer(axum::Extension(registry_a)),
        Arc::clone(&failing_auth) as Arc<dyn AuthStore>,
        Arc::new(front_door_a),
    )
    .await;
    let listener_a = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr_a = listener_a.local_addr().unwrap();
    let task_a = tokio::spawn(async move {
        let _ = axum::serve(listener_a, app_a).await;
    });
    let base_a = format!("http://{addr_a}");
    let session_a = sign_in(&base_a, &email).await;

    // Seed the durable patient link for `pat_id -> ws` via patient-scoped check while access is
    // lapsed so the reconnect catch-up pull delivers `pat_id`'s identity while `ws` is unenrolled.
    let (st_seed_lapsed, _) = post_json(
        &base_a,
        &session_a,
        "/connections/synthetic-charts/check",
        json!({ "patient_workspace_id": ws, "source_patient_id": pat_id }),
    )
    .await;
    assert_eq!(st_seed_lapsed, 200);
    assert_eq!(auth.patient_practice(&ws).await.unwrap(), None);

    // (a) Missing, blank, or header/body-conflicting `idempotency_key` -> 400 with 0 reconnect effects.
    let (st_missing, _) = post_json(
        &base_a,
        &session_a,
        "/connections/synthetic-charts/reconnect",
        json!({}),
    )
    .await;
    assert_eq!(st_missing, 400);

    let (st_blank, _) = post_json(
        &base_a,
        &session_a,
        "/connections/synthetic-charts/reconnect",
        json!({ "idempotency_key": "   " }),
    )
    .await;
    assert_eq!(st_blank, 400);

    let mismatch_resp = reqwest::Client::new()
        .post(format!("{base_a}/connections/synthetic-charts/reconnect"))
        .bearer_auth(&session_a)
        .header("Idempotency-Key", "header-key-1")
        .json(&json!({ "idempotency_key": "body-key-2" }))
        .send()
        .await
        .unwrap();
    assert_eq!(mismatch_resp.status().as_u16(), 400);
    assert_eq!(
        charts_a.reconnect_effect_count(&practice_id).await.unwrap(),
        0,
        "rejected 400 requests must execute zero adapter reconnect effects"
    );

    let (st_not_found, _) = post_json(
        &base_a,
        &session_a,
        "/connections/unknown-source/reconnect",
        json!({ "idempotency_key": "idem-unknown" }),
    )
    .await;
    assert_eq!(st_not_found, 404);

    // (b) Post-effect enrollment 500 -> retry with the SAME key via `Idempotency-Key` header (empty body)
    // repairs enrollment, returns original `reconnected_at == t1`, and keeps `reconnect_effect_count == 1`.
    failing_auth.fail_register.store(true, SeqCst);
    let idem_retry = format!("idem-route-500-{run}");
    let first_attempt = reqwest::Client::new()
        .post(format!("{base_a}/connections/synthetic-charts/reconnect"))
        .bearer_auth(&session_a)
        .header("Idempotency-Key", &idem_retry)
        .send()
        .await
        .unwrap();
    assert_eq!(first_attempt.status().as_u16(), 500);
    assert_eq!(auth.patient_practice(&ws).await.unwrap(), None);
    assert_eq!(
        charts_a.reconnect_effect_count(&practice_id).await.unwrap(),
        1,
        "initial attempt executed the adapter reconnect once before enrollment failed"
    );

    failing_auth.fail_register.store(false, SeqCst);
    advance_practice_clock(&db_url, &practice_id, t2).await;

    let retry_resp = reqwest::Client::new()
        .post(format!("{base_a}/connections/synthetic-charts/reconnect"))
        .bearer_auth(&session_a)
        .header("Idempotency-Key", &idem_retry)
        .send()
        .await
        .unwrap();
    assert_eq!(retry_resp.status().as_u16(), 200);
    let retry_body: Value = retry_resp.json().await.unwrap();
    assert_eq!(retry_body["outcome"], "reconnected");
    assert_eq!(retry_body["receipt"]["reconnected_at"], t1 as i64);
    assert_eq!(
        auth.patient_practice(&ws).await.unwrap(),
        Some(practice_id.clone())
    );
    assert_eq!(
        charts_a.reconnect_effect_count(&practice_id).await.unwrap(),
        1,
        "retrying POST /connections/{{id}}/reconnect after 500 must perform zero additional adapter effects"
    );

    // (c) Cross-instance duplicate `POST /connections/{id}/reconnect` (Site A -> Site B) replays the
    // exact receipt with 1 adapter effect, and reusing the same key for another source returns 409 Conflict.
    let charts_b = Arc::new(SyntheticCharts::open_with_store(&bank_b, runtime.clone()));
    let secondary_b = Arc::new(CountingSecondSource {
        source_id: "secondary-feed".to_owned(),
        reconnects: AtomicUsize::new(0),
    });
    let registry_b = physician_sample::sources::SourceRegistry::new(Some(runtime.clone()));
    registry_b.register_synthetic(Arc::clone(&charts_b));
    registry_b.register(Arc::clone(&secondary_b) as Arc<dyn PatientSource>);
    let front_door_b = front_door::PostgresFrontDoorStore::connect_with_migrations(&db_url)
        .await
        .unwrap();
    let app_b = physician_sample::authed_router_on(
        physician_sample::fixture_site().layer(axum::Extension(registry_b)),
        Arc::new(auth.clone()) as Arc<dyn AuthStore>,
        Arc::new(front_door_b),
    )
    .await;
    let listener_b = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr_b = listener_b.local_addr().unwrap();
    let task_b = tokio::spawn(async move {
        let _ = axum::serve(listener_b, app_b).await;
    });
    let base_b = format!("http://{addr_b}");
    let session_b = sign_in(&base_b, &email).await;

    advance_practice_clock(&db_url, &practice_id, t3).await;
    let (st_dup_b, dup_b_body) = post_json(
        &base_b,
        &session_b,
        "/connections/synthetic-charts/reconnect",
        json!({ "idempotency_key": idem_retry }),
    )
    .await;
    assert_eq!(st_dup_b, 200, "{dup_b_body}");
    assert_eq!(dup_b_body["outcome"], "reconnected");
    assert_eq!(dup_b_body["receipt"], retry_body["receipt"]);
    assert_eq!(
        charts_b.reconnect_effect_count(&practice_id).await.unwrap(),
        1,
        "cross-instance duplicate POST /connections/{{id}}/reconnect must keep total adapter effects at 1"
    );

    let (st_conflict, _) = post_json(
        &base_b,
        &session_b,
        "/connections/secondary-feed/reconnect",
        json!({ "idempotency_key": idem_retry }),
    )
    .await;
    assert_eq!(
        st_conflict, 409,
        "reusing an idempotency_key for a different source_id must return 409 Conflict"
    );
    assert_eq!(secondary_a.reconnects.load(SeqCst), 0);
    assert_eq!(secondary_b.reconnects.load(SeqCst), 0);

    task_a.abort();
    task_b.abort();
    let _ = fs::remove_dir_all(&bank_a);
    let _ = fs::remove_dir_all(&bank_b);
}
