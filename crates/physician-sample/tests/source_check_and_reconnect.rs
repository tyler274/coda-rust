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

fn build_bank(run: &str, count: usize) -> PathBuf {
    let root = std::env::temp_dir().join(format!("coda-source-check-{run}"));
    for index in 1..=count {
        let id = format!("{run}-{index}");
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
        json!({}),
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

    // Reusing the same `idempotency_key` with a different `source_id` returns `rejected`.
    let (status, conflict_cmd) = post_json(
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
    assert_eq!(status, 200, "{conflict_cmd}");
    assert_eq!(conflict_cmd["outcome"], "rejected");
}
