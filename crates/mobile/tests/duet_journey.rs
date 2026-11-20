//! Suite 2: mobile Duet journeys, with separate stub and durable-service proofs.
//!
//! `durable_doctor` uses isolated PostgreSQL, real durable API routes, and the
//! compiled patient UI. Its identity is synthetic; it does not qualify hosted
//! login or live model generation.
//!
//! The client mode talks to `duet-stub` mounted in the same process under
//! `/api`: the You and Shared tabs render their live data, and For my doctor
//! saves through the stub. The mode chooser walks both shells in the same browser.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

#[path = "common/devtools.rs"]
mod devtools;

use devtools::{Browser, DESKTOP, PHONE};

#[path = "common/disposable.rs"]
mod disposable;

#[path = "common/durable_doctor.rs"]
mod durable_doctor;

#[path = "common/session_refusal.rs"]
mod session_refusal;

// Durable-service D2 proof (no browser): isolated PostgreSQL + real owners + duet-client.
#[path = "common/durable_d2.rs"]
mod durable_d2;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_duet_journey_runs_on_a_live_stub_backend() {
    let address = spawn_site().await;
    let base = format!("http://{address}");
    let http = reqwest::Client::new();
    for credential in [None, Some("invalid-synthetic-credential")] {
        for path in ["/api/auth/account", "/api/auth/account/context"] {
            let mut request = if path.ends_with("/context") {
                http.get(format!("{base}{path}"))
            } else {
                http.put(format!("{base}{path}"))
            };
            if let Some(credential) = credential {
                request = request.bearer_auth(credential);
            }
            assert_eq!(request.send().await.unwrap().status(), 401);
        }
    }
    let mut browser = Browser::launch();
    browser.resize(PHONE);

    // Client mode: the shell opens on the live stub, landing on You.
    let text = browser.text_until(&format!("{base}/?mode=client&api={base}"), "Shared");
    assert!(
        text.contains("You") && text.contains("Shared"),
        "client mode did not show the Duet tabs:\n{text}"
    );
    assert!(
        !text.contains("Find patient"),
        "client mode leaked the physician shell:\n{text}"
    );

    assert_eq!(
        browser
            .page()
            .evaluate("Object.keys(localStorage).filter(k => k !== 'coda-mode')"),
        serde_json::json!([])
    );
    // For my doctor (`Shared`): mark a record in You, add a free-text note in Shared, review disclosure, and send.
    browser.click(r#"button[aria-label="Mark Lisinopril"]"#);
    browser.wait_for("Unmark");
    browser.click(r#"nav.tabs button:nth-of-type(2)"#);
    browser.wait_for("TO SEND");
    let editor_text = browser.body_text().unwrap();
    for retired in [
        "Name",
        "Date of birth",
        "Cover note",
        "Saved page preview",
        "Check and share PDF",
    ] {
        assert!(
            !editor_text.contains(retired),
            "retired field or control {retired} shown:\n{editor_text}"
        );
    }
    browser.wait_for("Lisinopril");
    browser.click(r#"button[aria-label="Add note"]"#);
    wait_for_selector(&mut browser, "textarea");
    browser.type_into("textarea", "What changed this week?");
    browser.click(r#"button[aria-label="Save note"]"#);
    let text = browser.wait_for("What changed this week?");
    assert!(
        !text.contains("Saved page preview") && !text.contains("Check and share PDF"),
        "retired PDF/preview controls shown:\n{text}"
    );
    browser.click(r#"button[aria-label="Send to Northside Primary Care"]"#);
    browser.wait_for("A send can't be taken back.");
    // Toggle disclosure checkbox (last checkbox on screen) and send.
    browser.page().evaluate(
        "(() => { const boxes = [...document.querySelectorAll('input[type=\"checkbox\"]')]; const d = boxes[boxes.length - 1]; if (d && !d.checked) d.click(); return true; })()",
    );
    browser.click(r#"button[aria-label="Send now"]"#);
    let after_send = browser.wait_for("Sent to Northside Primary Care.");
    assert!(
        after_send.contains("Sent") && after_send.contains("What changed this week?"),
        "expected Sent entry:\n{after_send}"
    );
    let saved: serde_json::Value = reqwest::get(format!("{base}/api/for-my-doctor"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(saved["marks"].as_array().unwrap().len(), 0);
    assert_eq!(saved["notes"].as_array().unwrap().len(), 0);
    assert_eq!(saved["sends"].as_array().unwrap().len(), 1);

    // D2.6 + You features: external records stay read-only, Search You filters,
    // trends open without lab range bands, and patient-entered medications can be corrected/deleted.
    browser.click(r#"nav.tabs button:nth-of-type(1)"#);
    browser.wait_for("Lisinopril");
    assert_eq!(
        browser.page().run(
            "has",
            serde_json::json!([r#"button[aria-label="Correct Lisinopril"]"#])
        ),
        false,
        "external Pharmacy record must not offer Correct"
    );
    assert_eq!(
        browser.page().run(
            "has",
            serde_json::json!([r#"button[aria-label="Delete LDL cholesterol"]"#])
        ),
        false,
        "external Quest lab must not offer Delete"
    );

    // Search filters You records live and clears cleanly.
    browser.type_into(r#"input[aria-label="Search You"]"#, "Pharmacy");
    let text = wait_until(&mut browser, |text| {
        text.contains("Lisinopril") && !text.contains("LDL cholesterol")
    });
    assert!(text.contains("Lisinopril"));
    browser.click(r#"button[aria-label="Clear search"]"#);
    browser.wait_for("LDL cholesterol");

    // Lab trend opens from a single source series with no printed reference band/flag.
    browser.click(r#"button[aria-label="View trend for LDL cholesterol"]"#);
    let text = wait_until(&mut browser, |text| {
        text.contains("Quest") && text.contains("Close trend")
    });
    assert!(!text.contains("reference") && !text.contains("flag"));
    browser.click(r#"button[aria-label="Close trend"]"#);
    wait_until(&mut browser, |text| !text.contains("Close trend"));

    // Patient-entered medication offers Correct and Delete.
    assert_eq!(
        browser
            .page()
            .run("clickButton", serde_json::json!(["Add health information"])),
        true
    );
    assert_eq!(
        browser
            .page()
            .run("clickButton", serde_json::json!(["New medication"])),
        true
    );
    browser.type_into(r#"input[aria-label="Medication name"]"#, "Magnesium 200 mg");
    assert_eq!(
        browser
            .page()
            .run("clickButton", serde_json::json!(["Add medication"])),
        true
    );
    browser.wait_for("Magnesium 200 mg");
    browser.click(r#"button[aria-label="Correct Magnesium 200 mg"]"#);
    browser.type_into(r#"input[aria-label="Corrected value"]"#, "Magnesium 400 mg");
    browser.click(r#"button[aria-label="Save correction"]"#);
    let text = wait_until(&mut browser, |text| {
        text.contains("Saved the correction.") && text.contains("Magnesium 400 mg")
    });
    assert!(text.contains("Lisinopril"));
    browser.click(r#"button[aria-label="Delete Magnesium 400 mg"]"#);
    let text = wait_until(&mut browser, |text| {
        text.contains("Removed from your records.") && !text.contains("Magnesium 400 mg")
    });
    assert!(text.contains("Lisinopril"));

    // Stub/browser D2 proof: You lists each record as a person would read it.
    browser.click("details.record-browser summary");
    let text = wait_until(&mut browser, |text| text.contains("Showing "));
    assert!(
        text.contains("Source: Pharmacy") && text.contains("2026-10-01T15:00:00+00:00"),
        "the record browser did not show the source and date:\n{text}"
    );
    assert!(
        !text.contains("Version 1") && !text.contains("Show original"),
        "the record browser still shows a technical inspector:\n{text}"
    );
    // The retired person-to-person endpoint stays absent. The Shared tab does not
    // expose physician consent controls.
    assert_eq!(
        http.get(format!("{base}/api/sharing/overview"))
            .send()
            .await
            .unwrap()
            .status(),
        404
    );
    browser.click(r#"nav.tabs button:nth-of-type(2)"#);
    wait_for_selector(
        &mut browser,
        r#"nav.tabs button:nth-of-type(2)[aria-current="page"]"#,
    );
    let text = browser.wait_for("Shared");
    assert!(
        !text.contains("People")
            && !text.contains("Share with my physician")
            && !text.contains("Stop sharing")
            && !text.contains("Awaiting acceptance")
            && !text.contains("not sharing yet")
            && !text.contains("Saved values stay visible"),
        "{text}"
    );

    // The journey holds at the desktop viewport.
    browser.resize(DESKTOP);
    let text = browser.wait_for("Shared");
    assert!(
        text.contains("You") && text.contains("Shared"),
        "the desktop viewport lost the Duet tabs:\n{text}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_mode_switch_walks_the_tabs_in_the_browser() {
    let address = spawn_site().await;
    let base = format!("http://{address}");

    let mut browser = Browser::launch();

    // Duet launches straight into the Duet shell with no mode chooser or physician shell.
    let text = browser.text_until(&base, "Shared");
    assert!(
        text.contains("You") && text.contains("Shared"),
        "Duet did not open the patient tabs:\n{text}"
    );
    assert!(
        !text.contains("Choose a mode")
            && !text.contains("Switch mode")
            && !text.contains("Find patient"),
        "Duet leaked retired mode chooser or physician chrome:\n{text}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn shared_tries_a_failed_workspace_read_again() {
    use axum::response::IntoResponse;
    use std::sync::atomic::{AtomicBool, Ordering};
    use tower::ServiceExt;

    let fail = Arc::new(AtomicBool::new(true));
    let route_fail = fail.clone();
    let stub = duet_stub::router(Arc::new(Mutex::new(
        duet_stub::Store::open(
            std::env::temp_dir().join(format!("shared-retry-{}.json", uuid::Uuid::new_v4())),
        )
        .unwrap(),
    )));
    let context = serde_json::json!({"account_id":uuid::Uuid::new_v4(),"incarnation":uuid::Uuid::new_v4(),"permission_revision":0,"authenticated_at":"2026-10-20T00:00:00Z"});
    let api = axum::Router::new()
        .route(
            "/api/auth/account",
            axum::routing::put(|| async {
                axum::Json(serde_json::json!({"terms_accepted": true, "terms_version": "2026-09-23"}))
            }),
        )
        .route(
            "/api/auth/account/profile",
            axum::routing::get(|| async {
                axum::Json(serde_json::json!({"appearance": "system", "terms_accepted": true, "terms_version": "2026-09-23"}))
            }),
        )
        .route(
            "/api/auth/account/context",
            axum::routing::get(move || {
                let context = context.clone();
                async move { axum::Json(context) }
            }),
        )
        .fallback(move |request: axum::http::Request<axum::body::Body>| {
            let stub = stub.clone();
            let failing = route_fail.load(Ordering::SeqCst)
                && request.method() == "GET"
                && request.uri().path() == "/api/for-my-doctor";
            async move {
                if failing {
                    axum::http::StatusCode::SERVICE_UNAVAILABLE.into_response()
                } else {
                    stub.oneshot(request).await.unwrap()
                }
            }
        });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let site = site_router(shared_site(), api);
    let server = tokio::spawn(async move { axum::serve(listener, site).await.unwrap() });
    let base = format!("http://{address}");
    let mut browser = Browser::launch();
    browser.resize(PHONE);
    browser.text_until(&format!("{base}/?mode=client&api={base}"), "Shared");

    // The first For my doctor read fails; Shared says so and offers to try again.
    browser.click(r#"nav.tabs button:nth-of-type(2)"#);
    let text = browser.wait_for("For my doctor couldn't load");
    assert!(!text.contains("To send"), "{text}");
    fail.store(false, Ordering::SeqCst);
    assert_eq!(
        browser
            .page()
            .run("clickButton", serde_json::json!(["Try again"])),
        true,
        "Shared offered no way to load For my doctor again:\n{}",
        browser.body_text().unwrap_or_default()
    );

    // For my doctor loads; the person writes a free-text note and saves.
    browser.wait_for("TO SEND");
    browser.click(r#"button[aria-label="Add note"]"#);
    wait_for_selector(&mut browser, "textarea");
    browser.type_into("textarea", "Discuss sleep and morning readings.");
    browser.click(r#"button[aria-label="Save note"]"#);
    browser.wait_for("Discuss sleep and morning readings.");
    let saved: serde_json::Value = reqwest::get(format!("{base}/api/for-my-doctor"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        saved["notes"][0]["body"],
        "Discuss sleep and morning readings."
    );
    server.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn you_search_trends_duplicates_and_conditions_in_browser() {
    use axum::{
        Json, Router,
        http::StatusCode,
        routing::{get, post},
    };

    let library_payload = serde_json::json!({
        "catalog_revision": null,
        "counts": {"currentCondition": 1, "pastCondition": 1, "lab": 2, "vital": 1, "procedure": 1},
        "entries": [
            {
                "category": "currentCondition",
                "coding_identities": [{"system": "http://snomed.info/sct", "code": "44054006", "version": null}],
                "condition_status_conflict": false,
                "date": "2026-10-01T00:00:00Z",
                "entity": "health_evidence",
                "group_kind": null,
                "id": "cond-now",
                "member_count": 1,
                "preview": null,
                "record_id": "00000000-0000-0000-0000-000000000101",
                "source_label": "Epic",
                "status_conflict": false,
                "summary": null,
                "title": "Type 2 diabetes"
            },
            {
                "category": "pastCondition",
                "coding_identities": [{"system": "http://snomed.info/sct", "code": "44054006", "version": null}],
                "condition_status_conflict": false,
                "date": "2022-05-01T00:00:00Z",
                "entity": "health_evidence",
                "group_kind": null,
                "id": "cond-old",
                "member_count": 1,
                "preview": null,
                "record_id": "00000000-0000-0000-0000-000000000102",
                "source_label": "Old Clinic",
                "status_conflict": false,
                "summary": null,
                "title": "Type 2 diabetes mellitus"
            },
            {
                "category": "lab",
                "coding_identities": [],
                "condition_status_conflict": false,
                "date": "2026-09-12T15:00:00Z",
                "entity": "health_evidence",
                "group_kind": null,
                "id": "a1c-quest",
                "member_count": 1,
                "preview": {"status": "entered-in-error"},
                "record_id": "00000000-0000-0000-0000-000000000103",
                "source_label": "Quest",
                "status_conflict": false,
                "summary": "6.8 %",
                "title": "Hemoglobin A1c"
            },
            {
                "category": "lab",
                "coding_identities": [],
                "condition_status_conflict": false,
                "date": "2026-09-12T18:00:00Z",
                "entity": "health_evidence",
                "group_kind": null,
                "id": "a1c-regional",
                "member_count": 1,
                "preview": null,
                "record_id": "00000000-0000-0000-0000-000000000104",
                "source_label": "Regional Lab",
                "status_conflict": false,
                "summary": "6.8 %",
                "title": "Hemoglobin A1c"
            },
            {
                "category": "vital",
                "coding_identities": [],
                "condition_status_conflict": false,
                "date": "2026-10-10T08:00:00Z",
                "entity": "health_evidence",
                "group_kind": null,
                "id": "bp-1",
                "member_count": 1,
                "preview": null,
                "record_id": "00000000-0000-0000-0000-000000000105",
                "source_label": "Apple Health",
                "status_conflict": false,
                "summary": "122/78 mmHg",
                "title": "Blood Pressure"
            },
            {
                "category": "procedure",
                "coding_identities": [],
                "condition_status_conflict": false,
                "date": "2026-08-20T09:00:00Z",
                "entity": "health_evidence",
                "group_kind": null,
                "id": "proc-1",
                "member_count": 1,
                "preview": {"status": "cancelled"},
                "record_id": "00000000-0000-0000-0000-000000000106",
                "source_label": "Surgery Center",
                "status_conflict": false,
                "summary": null,
                "title": "Cancelled visit"
            }
        ],
        "health_records_generation": 0,
        "next_cursor": null,
        "revision": 1,
        "total_count": 6
    });
    let browse_payload = serde_json::json!({
        "revision": 1,
        "health_records_generation": 0,
        "total_count": 4,
        "removed_count": 0,
        "next_cursor": null,
        "records": [
            {
                "address": {"entity": "health_evidence", "record_id": "q1", "canonical_record_id": "00000000-0000-0000-0000-000000000103", "revision": 1},
                "status": "current",
                "title": "Hemoglobin A1c",
                "date": "2026-06-12T15:00:00Z",
                "value": "7.1",
                "unit": "%",
                "provenance": {"source_kind": "epic", "writer": "connector", "source_label": "Quest"},
                "payload": null
            },
            {
                "address": {"entity": "health_evidence", "record_id": "q2", "canonical_record_id": "00000000-0000-0000-0000-000000000103", "revision": 2},
                "status": "current",
                "title": "Hemoglobin A1c",
                "date": "2026-09-12T15:00:00Z",
                "value": "6.8",
                "unit": "%",
                "provenance": {"source_kind": "epic", "writer": "connector", "source_label": "Quest"},
                "payload": null
            },
            {
                "address": {"entity": "health_evidence", "record_id": "r1", "canonical_record_id": "00000000-0000-0000-0000-000000000104", "revision": 1},
                "status": "current",
                "title": "Hemoglobin A1c",
                "date": "2026-09-12T18:00:00Z",
                "value": "6.8",
                "unit": "%",
                "provenance": {"source_kind": "epic", "writer": "connector", "source_label": "Regional Lab"},
                "payload": null
            },
            {
                "address": {"entity": "health_evidence", "record_id": "bp1", "canonical_record_id": "00000000-0000-0000-0000-000000000105", "revision": 1},
                "status": "current",
                "title": "Blood Pressure",
                "date": "2026-10-10T08:00:00Z",
                "value": "122/78",
                "unit": "mmHg",
                "provenance": {"source_kind": "appleHealth", "writer": "device", "source_label": "Apple Health"},
                "payload": null
            }
        ]
    });
    let empty_browse = serde_json::json!({
        "revision": 1,
        "health_records_generation": 0,
        "total_count": 0,
        "removed_count": 0,
        "next_cursor": null,
        "records": []
    });
    let account_id = uuid::Uuid::new_v4();
    let api = Router::new()
        .route(
            "/api/auth/account",
            axum::routing::put(|| async {
                Json(serde_json::json!({
                    "terms_accepted": true,
                    "terms_version": "2026-09-23"
                }))
            }),
        )
        .route(
            "/api/auth/account/profile",
            get(|| async {
                Json(serde_json::json!({
                    "appearance": "system",
                    "terms_accepted": true,
                    "terms_version": "2026-09-23"
                }))
            }),
        )
        .route(
            "/api/auth/account/context",
            get(move || async move {
                Json(serde_json::json!({
                    "account_id": account_id,
                    "incarnation": account_id,
                    "permission_revision": 0,
                    "authenticated_at": "2026-10-20T00:00:00Z"
                }))
            }),
        )
        .route(
            "/api/evidence/library",
            post({
                let payload = library_payload.clone();
                move || {
                    let payload = payload.clone();
                    async move { Json(payload) }
                }
            }),
        )
        .route(
            "/api/evidence/browse",
            post({
                let browse_payload = browse_payload.clone();
                let empty_browse = empty_browse.clone();
                move |Json(q): Json<serde_json::Value>| {
                    let browse_payload = browse_payload.clone();
                    let empty_browse = empty_browse.clone();
                    async move {
                        if q.get("cursor").is_some_and(|c| !c.is_null()) {
                            Json(empty_browse)
                        } else {
                            Json(browse_payload)
                        }
                    }
                }
            }),
        )
        .route("/api/evidence", post(|| async { StatusCode::CONFLICT }))
        .route(
            "/api/doctor-collection",
            get(|| async { Json(serde_json::json!({"note_generation": 0, "items": []})) }),
        )
        .route(
            "/api/for-my-doctor",
            get(|| async {
                Json(serde_json::json!({
                    "marks": [],
                    "notes": [],
                    "sends": [],
                    "notices": [],
                    "practices": []
                }))
            }),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let site = site_router(shared_site(), api);
    let server = tokio::spawn(async move { axum::serve(listener, site).await.unwrap() });
    let base = format!("http://{address}");
    let mut browser = Browser::launch();
    browser.resize(PHONE);
    browser.text_until(
        &format!("{base}/?mode=client&api={base}"),
        "Type 2 diabetes",
    );
    let text = browser.body_text().unwrap();
    assert!(
        !text.contains("Type 2 diabetes mellitus"),
        "covered pastCondition leaked:\n{text}"
    );
    assert!(
        text.contains("recorded by mistake"),
        "missing entered-in-error explanation:\n{text}"
    );
    assert!(
        text.contains("did not happen"),
        "missing cancelled explanation:\n{text}"
    );

    // Likely-duplicate review sheet shows cross-source match and hides chosen duplicate locally.
    browser.click(r#"button[aria-label="Review possible duplicates"]"#);
    let text = browser.wait_for("looks like the same result as");
    assert!(text.contains("Regional Lab") && text.contains("Quest"));
    browser.click(r#"button[aria-label="Keep Quest"]"#);
    browser.click(r#"button[aria-label="Done reviewing duplicates"]"#);
    wait_until(&mut browser, |text| !text.contains("Regional Lab"));

    // Lab trend shows per-source switcher without splicing or reference ranges.
    browser.click(r#"button[aria-label="View trend for Hemoglobin A1c"]"#);
    let text = wait_until(&mut browser, |text| {
        text.contains("Hemoglobin A1c")
            && text.contains("Regional Lab")
            && text.contains("Close trend")
    });
    assert!(!text.contains("reference") && !text.contains("flag"));
    browser.click(r#"button[aria-label="Close trend"]"#);

    // Blood pressure trend shows both numbers together.
    browser.click(r#"button[aria-label="View trend for Blood Pressure"]"#);
    wait_until(&mut browser, |text| {
        text.contains("122/78") && text.contains("systolic and diastolic")
    });
    server.abort();
}

fn wait_for_selector(browser: &mut Browser, selector: &str) {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    while std::time::Instant::now() < deadline {
        let found = browser.page().run("has", serde_json::json!([selector]));
        if found.as_bool() == Some(true) {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    panic!(
        "missing {selector}:\n{}",
        browser.body_text().unwrap_or_default()
    );
}

fn wait_until(browser: &mut Browser, ready: impl Fn(&str) -> bool) -> String {
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    let mut last = String::new();
    while std::time::Instant::now() < deadline {
        if let Some(text) = browser.body_text() {
            last = text;
            if ready(&last) {
                return last;
            }
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    panic!("the page did not reach the expected state:\n{last}");
}

/// One site per spawn: the mobile page at `/` and the stub router plus the
/// in-process front door mounted under `/api` in this same process. The
/// browser uses an explicitly verified synthetic standalone account adapter.
/// The WASM build is shared through the process-wide cell; the two servers
/// stay independent.
async fn spawn_site() -> std::net::SocketAddr {
    spawn_site_with_patient(false).await.0
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn for_my_doctor_uncertain_retry_cod50_and_cod51_in_browser() {
    use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};
    use std::sync::atomic::{AtomicU8, Ordering};
    use tower::ServiceExt;

    let stub_store = Arc::new(Mutex::new(
        duet_stub::Store::open(
            std::env::temp_dir().join(format!("fmd-browser-{}.json", uuid::Uuid::new_v4())),
        )
        .unwrap(),
    ));
    let stub = duet_stub::router(stub_store.clone());
    // 0 => normal, 1 => fail next POST /api/for-my-doctor/send with 503 (uncertain),
    // 2 => fail next POST /api/for-my-doctor/send with 409 connection_ended (COD-51).
    let send_mode = Arc::new(AtomicU8::new(1));
    let captured_keys = Arc::new(Mutex::new(Vec::<String>::new()));

    let context = serde_json::json!({
        "account_id": uuid::Uuid::new_v4(),
        "incarnation": uuid::Uuid::new_v4(),
        "permission_revision": 0,
        "authenticated_at": "2026-11-20T00:00:00Z"
    });
    let api = Router::new()
        .route(
            "/api/auth/account",
            axum::routing::put(|| async {
                Json(serde_json::json!({"terms_accepted": true, "terms_version": "2026-09-23"}))
            }),
        )
        .route(
            "/api/auth/account/profile",
            axum::routing::get(|| async {
                Json(serde_json::json!({"appearance": "system", "terms_accepted": true, "terms_version": "2026-09-23"}))
            }),
        )
        .route(
            "/api/auth/account/context",
            axum::routing::get(move || {
                let context = context.clone();
                async move { Json(context) }
            }),
        )
        .route(
            "/api/for-my-doctor/send",
            post({
                let stub = stub.clone();
                let mode = send_mode.clone();
                let keys = captured_keys.clone();
                move |req: axum::http::Request<axum::body::Body>| {
                    let stub = stub.clone();
                    let mode = mode.clone();
                    let keys = keys.clone();
                    async move {
                        let (parts, body) = req.into_parts();
                        let bytes = axum::body::to_bytes(body, 2_000_000).await.unwrap();
                        let parsed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
                        if let Some(key) = parsed["share_key"].as_str() {
                            keys.lock().unwrap().push(key.to_owned());
                        }
                        match mode.swap(0, Ordering::SeqCst) {
                            1 => (
                                StatusCode::SERVICE_UNAVAILABLE,
                                Json(serde_json::json!({"code": "uncertain", "detail": "Temporary timeout"})),
                            )
                                .into_response(),
                            2 => (
                                StatusCode::CONFLICT,
                                Json(serde_json::json!({"code": "connection_ended", "detail": "Patient left practice."})),
                            )
                                .into_response(),
                            _ => stub
                                .oneshot(axum::http::Request::from_parts(
                                    parts,
                                    axum::body::Body::from(bytes),
                                ))
                                .await
                                .unwrap(),
                        }
                    }
                }
            }),
        )
        .fallback(move |request: axum::http::Request<axum::body::Body>| {
            let stub = stub.clone();
            async move { stub.oneshot(request).await.unwrap() }
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let site = site_router(shared_site(), api);
    let server = tokio::spawn(async move { axum::serve(listener, site).await.unwrap() });
    let base = format!("http://{address}");

    let mut browser = Browser::launch();
    browser.resize(PHONE);
    browser.text_until(&format!("{base}/?mode=client&api={base}"), "Lisinopril");

    // Single open record -> Send -> Choosing pre-selects that record alone, then 503 -> Couldn't confirm + Try again.
    browser.click(r#"button[aria-label="Open Lisinopril"]"#);
    browser.wait_for("Add note");
    assert_eq!(
        browser
            .page()
            .run("clickButton", serde_json::json!(["Send"])),
        true
    );
    browser.wait_for("Choose what to send");
    browser.page().evaluate(
        "(() => { const boxes = [...document.querySelectorAll('input[type=\"checkbox\"]')]; const d = boxes[boxes.length - 1]; if (d && !d.checked) d.click(); return true; })()",
    );
    browser.click(r#"button[aria-label="Send now"]"#);
    browser.wait_for("Couldn't confirm the send");

    // Retrying with Try again sends exact same share_key and transitions cleanly to Sent.
    assert_eq!(
        browser
            .page()
            .run("clickButton", serde_json::json!(["Try again"])),
        true
    );
    let after_retry = browser.wait_for("Sent to Northside Primary Care.");
    assert!(after_retry.contains("Lisinopril"), "{after_retry}");
    let keys = captured_keys.lock().unwrap().clone();
    assert_eq!(keys.len(), 2);
    assert_eq!(
        keys[0], keys[1],
        "Try again must reuse the exact same share_key"
    );

    // Now test COD-51: connection ends while "Couldn't confirm the send" -> Try again disappears,
    // "Couldn't confirm before connection ended" shown, To send kept.
    browser.click(r#"button[aria-label="Add note"]"#);
    wait_for_selector(&mut browser, "textarea");
    browser.type_into("textarea", "Keep my note if ended.");
    browser.click(r#"button[aria-label="Save note"]"#);
    browser.wait_for("Keep my note if ended.");
    send_mode.store(1, Ordering::SeqCst);
    browser.click(r#"button[aria-label="Send to Northside Primary Care"]"#);
    browser.wait_for("Before you send");
    browser.page().evaluate(
        "(() => { const boxes = [...document.querySelectorAll('input[type=\"checkbox\"]')]; const d = boxes[boxes.length - 1]; if (d && !d.checked) d.click(); return true; })()",
    );
    browser.click(r#"button[aria-label="Send now"]"#);
    browser.wait_for("Couldn't confirm the send");
    send_mode.store(2, Ordering::SeqCst);
    assert_eq!(
        browser
            .page()
            .run("clickButton", serde_json::json!(["Try again"])),
        true
    );
    let cod51_text = browser.wait_for("Couldn't confirm before connection ended");
    assert!(
        !cod51_text.contains("Try again"),
        "Try again must disappear on COD-51:\n{cod51_text}"
    );
    assert!(
        cod51_text.contains("Keep my note if ended."),
        "To send note must remain:\n{cod51_text}"
    );

    server.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn patient_tabs_do_not_offer_or_request_physician_sharing() {
    let (address, session) = spawn_site_with_patient(true).await;
    let base = format!("http://{address}");
    let mut browser = Browser::launch();
    browser.resize(PHONE);
    browser
        .page()
        .call_session("Network.enable", serde_json::json!({}));
    browser.page().call_session(
        "Network.setExtraHTTPHeaders",
        serde_json::json!({"headers":{"Authorization":format!("Bearer {session}")}}),
    );
    browser.text_until(&format!("{base}/?mode=client"), "Shared");
    let evidence = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/verification/physician-sharing");
    std::fs::create_dir_all(&evidence).unwrap();
    for (tab, loaded, absent, image) in [
        (
            1,
            "Add health information",
            "Sharing with my physician",
            "you.json",
        ),
        (2, "Shared", "Share with my physician", "shared.json"),
    ] {
        browser.click(&format!("nav.tabs button:nth-of-type({tab})"));
        wait_for_selector(
            &mut browser,
            &format!(r#"nav.tabs button:nth-of-type({tab})[aria-current="page"]"#),
        );
        let text = browser.wait_for(loaded);
        assert!(!text.contains(absent), "{text}");
        let screenshot = browser.page().call_session(
            "Page.captureScreenshot",
            serde_json::json!({"format":"png"}),
        );
        std::fs::write(
            evidence.join(image),
            serde_json::to_vec(&screenshot).unwrap(),
        )
        .unwrap();
    }
    let paths: Vec<String> = serde_json::from_value(browser.page().evaluate(
        "performance.getEntriesByType('resource').map(entry => new URL(entry.name).pathname)",
    ))
    .unwrap();
    assert!(
        paths.iter().all(|path| path != "/api/consent"
            && !path.starts_with("/api/physician-sharing/")
            && !path.starts_with("/api/sharing/")),
        "{paths:?}"
    );
}

async fn spawn_site_with_patient(linked: bool) -> (std::net::SocketAddr, String) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let store = Arc::new(Mutex::new(
        duet_stub::Store::open(
            std::env::temp_dir().join(format!("duet-stub-browser-{}.json", uuid::Uuid::new_v4())),
        )
        .expect("stub store"),
    ));
    let front_door_store = Arc::new(front_door::MemoryFrontDoorStore::new());
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix clock")
        .as_secs();
    let patient = front_door::RosterPatient {
        practice_id: "roger-synthetic-practice".to_owned(),
        patient_id: "pat-elena".to_owned(),
    };
    let invite_token = front_door::FrontDoorStore::invite(
        front_door_store.as_ref(),
        &patient,
        "elena@codahealth.test",
        "Elena Rostova",
        now,
    )
    .await
    .expect("front-door invite");
    let (account_id, session) = front_door::FrontDoorStore::redeem_invitation(
        front_door_store.as_ref(),
        "elena@codahealth.test",
        &invite_token,
        now,
    )
    .await
    .expect("front-door redeem");
    let site = shared_site();
    let expected = format!(
        "Bearer {}",
        if linked {
            session.as_str()
        } else {
            "synthetic-stub-browser-only"
        }
    );
    let account_header = expected.clone();
    // Test-only identity adapter. It verifies the synthetic build credential, then
    // supplies a stable account/incarnation. It never modifies production routing.
    let context = serde_json::json!({"account_id":account_id,"incarnation":uuid::Uuid::new_v4(),"permission_revision":0,"authenticated_at":"2026-10-20T00:00:00Z"});
    let identity = axum::Router::new()
        .route(
            "/api/auth/account",
            axum::routing::put(move |headers: axum::http::HeaderMap| {
                let expected = account_header.clone();
                async move {
                    if headers.get("authorization").and_then(|h| h.to_str().ok())
                        != Some(expected.as_str())
                    {
                        return Err(axum::http::StatusCode::UNAUTHORIZED);
                    }
                    Ok(axum::Json(serde_json::json!({"terms_accepted": true, "terms_version": "2026-09-23"})))
                }
            }),
        )
        .route(
            "/api/auth/account/profile",
            axum::routing::get(|| async {
                axum::Json(serde_json::json!({"appearance": "system", "terms_accepted": true, "terms_version": "2026-09-23"}))
            }),
        )
        .route(
            "/api/auth/account/context",
            axum::routing::get(move |headers: axum::http::HeaderMap| {
                let context = context.clone();
                let expected = expected.clone();
                async move {
                    if headers.get("authorization").and_then(|h| h.to_str().ok())
                        != Some(expected.as_str())
                    {
                        return Err(axum::http::StatusCode::UNAUTHORIZED);
                    }
                    Ok(axum::Json(context))
                }
            }),
        );
    let api = duet_stub::router(store)
        .merge(front_door::http::router_with(
            front_door::http::FrontDoorState::new(front_door_store),
            true,
        ))
        .merge(identity);
    let app = site_router(site, api);
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(address).await;
    (address, session)
}

/// The mobile page at `/` and the stub under `/api`. The stub keeps its own
/// `/api/...` route paths, so the request is forwarded unchanged.
fn site_router(site_root: PathBuf, api: axum::Router) -> axum::Router {
    axum::Router::new().fallback(move |request: axum::http::Request<axum::body::Body>| {
        let site_root = site_root.clone();
        let mut api = api.clone();
        async move {
            if request.uri().path().starts_with("/api/")
                || request.uri().path().starts_with("/front-door/")
            {
                use tower::Service as _;
                return api
                    .call(request)
                    .await
                    .unwrap_or_else(|never| match never {});
            }
            match static_file(&site_root, request.uri().path(), "").await {
                Ok(response) => response,
                Err(_) => axum::http::Response::builder()
                    .status(axum::http::StatusCode::NOT_FOUND)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            }
        }
    })
}

#[tokio::test]
async fn static_site_serves_assets_without_escaping_its_root() {
    use tower::ServiceExt;

    let directory =
        std::env::temp_dir().join(format!("coda-site-boundary-{}", uuid::Uuid::new_v4()));
    let root = directory.join("site");
    std::fs::create_dir_all(root.join("assets")).unwrap();
    std::fs::write(root.join("index.html"), "test index").unwrap();
    std::fs::write(root.join("assets/app.js"), "test script").unwrap();
    std::fs::write(directory.join("outside.txt"), "outside the site root").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(directory.join("outside.txt"), root.join("escaped.txt")).unwrap();
    let app = site_router(root, axum::Router::new());
    let mut observed = Vec::new();
    for path in [
        "/",
        "/assets/app.js",
        "/../outside.txt",
        "/assets/../../outside.txt",
        "/%2e%2e/outside.txt",
        "/escaped.txt",
    ] {
        // Construct the raw URI: browsers and URL clients normalize dot segments.
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri(path)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        observed.push((path, status, body));
    }
    std::fs::remove_dir_all(directory).unwrap();
    for (path, status, body) in observed {
        match path {
            "/" => {
                assert_eq!(status, 200);
                assert_eq!(&body[..], b"test index");
            }
            "/assets/app.js" => {
                assert_eq!(status, 200);
                assert_eq!(&body[..], b"test script");
            }
            _ => assert_eq!(status, 404, "{path} escaped the site root"),
        }
    }
}

async fn static_file(
    root: &std::path::Path,
    path: &str,
    _session: &str,
) -> Result<axum::response::Response, std::io::Error> {
    use tokio::io::AsyncReadExt;

    let relative = path.trim_start_matches('/');
    let relative = if relative.is_empty() {
        "index.html"
    } else {
        relative
    };
    // Compare resolved paths so dot segments and symlinks cannot leave the
    // compiled site tree. Both journey routers use this same boundary.
    let root = root.canonicalize()?;
    let file = root.join(relative).canonicalize()?;
    if !file.starts_with(&root) || !file.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no such site file",
        ));
    }
    let mut bytes = Vec::new();
    let mut opened = tokio::fs::File::open(&file).await?;
    opened.read_to_end(&mut bytes).await?;
    let mime = match file.extension().and_then(|ext| ext.to_str()) {
        Some("js") => "text/javascript",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css",
        Some("html") => "text/html; charset=utf-8",
        _ => "application/octet-stream",
    };
    Ok(axum::response::Response::builder()
        .header("content-type", mime)
        .body(axum::body::Body::from(bytes))
        .unwrap())
}

/// Builds the WASM site once for this test binary.
fn shared_site() -> PathBuf {
    static SITE: OnceLock<PathBuf> = OnceLock::new();
    SITE.get_or_init(build_site).clone()
}

fn build_site() -> PathBuf {
    build_site_with_bearer(None)
}

fn build_site_with_bearer(bearer: Option<&str>) -> PathBuf {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let target = workspace.join("target").join(if bearer.is_some() {
        "coda-mobile-durable-wasm"
    } else {
        "coda-mobile-wasm"
    });
    let output = workspace.join(if bearer.is_some() {
        "target/mobile-durable-web"
    } else {
        "target/mobile-web"
    });
    let wasm = target.join("wasm32-unknown-unknown/debug/mobile.wasm");
    let mut cargo =
        std::process::Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    cargo_without_parent_package(&mut cargo);
    cargo.env(
        "DUET_BEARER",
        bearer.unwrap_or("synthetic-stub-browser-only"),
    );
    let status = cargo
        .current_dir(&workspace)
        .env("CARGO_TARGET_DIR", &target)
        .args([
            "build",
            "--locked",
            "--offline",
            "-p",
            "mobile",
            "--target",
            "wasm32-unknown-unknown",
            "--features",
            "web",
        ])
        .status()
        .expect("cargo build");
    assert!(status.success(), "mobile wasm build failed");
    std::fs::create_dir_all(output.clone()).unwrap();
    std::fs::copy(
        workspace.join("crates/mobile/index.html"),
        output.join("index.html"),
    )
    .unwrap();
    let status = std::process::Command::new(wasm_bindgen())
        .args([
            "--target",
            "web",
            "--out-dir",
            output.clone().to_str().unwrap(),
            wasm.to_str().unwrap(),
        ])
        .status()
        .expect("wasm-bindgen");
    assert!(status.success(), "wasm-bindgen failed");
    output.clone()
}

fn cargo_without_parent_package(command: &mut std::process::Command) {
    command.env_clear();
    // LD_LIBRARY_PATH rides along: the tools on PATH (for example the lld
    // that links wasm) resolve their own shared libraries through it.
    for key in [
        "PATH",
        "HOME",
        "USER",
        "RUSTUP_HOME",
        "CARGO_HOME",
        "RUSTUP_TOOLCHAIN",
        "TMPDIR",
        "LD_LIBRARY_PATH",
    ] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
}

fn wasm_bindgen() -> PathBuf {
    if let Some(path) = std::env::var_os("WASM_BINDGEN") {
        return PathBuf::from(path);
    }
    if let Some(path) = find_on_path("wasm-bindgen") {
        return path;
    }
    let local =
        std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/bin/wasm-bindgen"));
    if let Some(path) = local.filter(|path| path.is_file()) {
        return path;
    }
    panic!("wasm-bindgen 0.2.129 is required to build the mobile page");
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    std::env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}

async fn wait_until_ready(address: std::net::SocketAddr) {
    let started = std::time::Instant::now();
    loop {
        if started.elapsed() > Duration::from_secs(10) {
            panic!("mobile site did not accept connections on {address}");
        }
        if tokio::net::TcpStream::connect(address).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
