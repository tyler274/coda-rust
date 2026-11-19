//! Integration tests covering PR-D1 AC4–AC7:
//! - AC4: Server-persisted per-physician panel preferences across sessions.
//! - AC5: Elation FHIR R4 OAuth2/roster/schedule/chart-refresh + Hint parsers.
//! - AC6: Authenticated Hint Chrome connector bridge routes (`X-Coda-Pair`).
//! - AC7: `RefreshingSourceChecker` daily external source re-pull on dispatch wake.

#[path = "common/disposable.rs"]
mod disposable;

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::{Path as AxumPath, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::elation_fhir::{
    BASE_URL_ENV, CLIENT_ID_ENV, CLIENT_SECRET_ENV, ElationFhirClient, ElationFhirConfig,
    SOURCE_ID as ELATION_SOURCE_ID, TOKEN_URL_ENV,
};
use local_product::physician_sources::hint::{
    HINT_SOURCE_ID, parse_hint_appointments, parse_hint_roster,
};
use local_product::physician_sources::workspace_id;
use local_product::runtime::Store;
use local_product::runtime::dispatch::RefreshingSourceChecker;
use local_product::runtime::hint_bridge::{HintBridgeScheduleSource, HintBridgeSourceRefresh};
use local_product::runtime::jobs::PracticeScope;
use local_product::runtime::refresh::{
    CompositePatientSourceRefresh, PatientSourceRefresh, RefreshCommandKey, RefreshRequest,
    RefreshTarget, SourceRefreshOutcome,
};
use local_product::runtime::reviews::PatientReviewReceiver;
use local_product::runtime::schedule::{
    ScheduleDelivery, ScheduleFreshness, ScheduleSource, ScheduleStanding, ScheduleSyncOutcome,
    SourceId as ScheduleSourceId,
};
use local_product::{ModelBinding, ModelRoles};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use physician_bus::{Bus, Event, LocalBus};
use physician_sample::durable::DurableChat;
use physician_sample::hint_bridge::PAIR_HEADER;
use physician_sample::{authed_postgres_router, durable_site};
use physician_wire::{PanelPreferencesWire, PracticeDayResponse};
use sea_orm::Database;
use serde_json::{Value, json};

const PRACTICE_NOW_UNIX: u64 = 1_787_846_400; // 2026-08-27T16:00:00Z

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

async fn setup_postgres(tag: &str) -> Option<(String, String, String, Store, PostgresAuthStore)> {
    let gate = gate_database_url()?;
    let (server, _) = gate
        .rsplit_once('/')
        .expect("database URL must include database name");
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let db_name = format!("ehr_panels_{tag}_{}_{nanos:x}", std::process::id());
    let admin_url = format!("{server}/postgres");
    let admin = Database::connect(&admin_url)
        .await
        .expect("connect admin postgres");
    disposable::replace_database(&admin, &db_name).await;
    admin.close().await.expect("close admin postgres");
    let test_url = format!("{server}/{db_name}");

    let store = Store::connect(&test_url)
        .await
        .expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = PostgresAuthStore::from_connection(store.connection().clone());
    auth.migrate().await.expect("migrate auth store");
    let fd = front_door::PostgresFrontDoorStore::from_connection(store.connection().clone());
    fd.migrate().await.expect("migrate front door store");

    Some((admin_url, db_name, test_url, store, auth))
}

async fn teardown_postgres(admin_url: &str, db_name: &str) {
    let admin = Database::connect(admin_url)
        .await
        .expect("connect admin postgres for cleanup");
    disposable::drop_database(&admin, db_name).await;
    let _ = admin.close().await;
}

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"),
    )
    .expect("load physician skills")
}

fn test_roles() -> ModelRoles {
    let binding = ModelBinding {
        model: "test-ehr-model".into(),
        effort: "low".into(),
        context_window_tokens: 128_000,
        max_input_tokens: 100_000,
        adapter: "openai-responses".into(),
    };
    ModelRoles {
        chat: binding.clone(),
        background: binding,
    }
}

async fn spawn_router(app: Router) -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind loopback");
    let addr = listener.local_addr().expect("local addr");
    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    (addr, handle)
}

async fn sign_in_email(client: &reqwest::Client, base: &str, email: &str) -> String {
    let begun: Value = client
        .post(format!("{base}/auth/sign-in"))
        .json(&json!({ "email": email }))
        .send()
        .await
        .expect("sign-in")
        .json()
        .await
        .expect("decode sign-in");
    let verified: Value = client
        .post(format!("{base}/auth/verify"))
        .json(&json!({ "email": email, "token": begun["token"] }))
        .send()
        .await
        .expect("verify")
        .json()
        .await
        .expect("decode verify");
    verified["session"]
        .as_str()
        .expect("session token")
        .to_owned()
}

#[tokio::test]
async fn ac4_panel_preferences_persist_across_sessions_and_isolate_physicians() {
    let Some((admin_url, db_name, db_url, store, auth)) = setup_postgres("ac4").await else {
        return;
    };
    let practice = "practice-panels-ac4";
    auth.ensure_practice(practice, PracticeClock::Fixed(PRACTICE_NOW_UNIX))
        .await
        .expect("ensure practice");
    let alice = PhysicianIdentity {
        practice_id: practice.into(),
        physician_id: "dr-alice".into(),
    };
    let bob = PhysicianIdentity {
        practice_id: practice.into(),
        physician_id: "dr-bob".into(),
    };
    auth.ensure_account(
        &alice.practice_id,
        &alice.physician_id,
        "alice@clinic.test",
        "Dr. Alice",
        true,
        PRACTICE_NOW_UNIX,
    )
    .await
    .expect("seed alice");
    auth.ensure_account(
        &bob.practice_id,
        &bob.physician_id,
        "bob@clinic.test",
        "Dr. Bob",
        false,
        PRACTICE_NOW_UNIX,
    )
    .await
    .expect("seed bob");

    let bus: Arc<dyn Bus> = Arc::new(LocalBus::new(32));
    let chat = Arc::new(DurableChat::new(
        store.clone(),
        load_skills(),
        &test_roles(),
    ));
    let app = authed_postgres_router(
        durable_site(chat, Arc::clone(&bus)),
        &db_url,
        store.clone(),
        bus,
    )
    .await;
    let (addr, server) = spawn_router(app).await;
    let base = format!("http://{addr}");
    let client = reqwest::Client::new();

    // Sign in Dr. Alice -> defaults to both expanded.
    let alice_tok1 = sign_in_email(&client, &base, "alice@clinic.test").await;
    let initial: PanelPreferencesWire = client
        .get(format!("{base}/physician-state/panels"))
        .bearer_auth(&alice_tok1)
        .send()
        .await
        .expect("GET panels")
        .error_for_status()
        .expect("200 GET panels")
        .json()
        .await
        .expect("decode panels");
    assert_eq!(
        initial,
        PanelPreferencesWire {
            sidebar_collapsed: false,
            schedule_collapsed: false,
        }
    );

    // Save collapsed preferences for Dr. Alice.
    let saved: PanelPreferencesWire = client
        .put(format!("{base}/physician-state/panels"))
        .bearer_auth(&alice_tok1)
        .json(&PanelPreferencesWire {
            sidebar_collapsed: true,
            schedule_collapsed: true,
        })
        .send()
        .await
        .expect("PUT panels")
        .error_for_status()
        .expect("200 PUT panels")
        .json()
        .await
        .expect("decode saved panels");
    assert!(saved.sidebar_collapsed && saved.schedule_collapsed);

    // Sign out Dr. Alice -> old token rejected.
    client
        .post(format!("{base}/auth/sign-out"))
        .bearer_auth(&alice_tok1)
        .send()
        .await
        .expect("sign out")
        .error_for_status()
        .expect("200 sign out");
    let revoked_status = client
        .get(format!("{base}/physician-state/panels"))
        .bearer_auth(&alice_tok1)
        .send()
        .await
        .expect("GET revoked")
        .status();
    assert_eq!(revoked_status, StatusCode::UNAUTHORIZED);

    // Sign in Dr. Alice again -> preferences survived sign-out!
    let alice_tok2 = sign_in_email(&client, &base, "alice@clinic.test").await;
    let restored: PanelPreferencesWire = client
        .get(format!("{base}/physician-state/panels"))
        .bearer_auth(&alice_tok2)
        .send()
        .await
        .expect("GET restored panels")
        .error_for_status()
        .expect("200 GET restored panels")
        .json()
        .await
        .expect("decode restored panels");
    assert_eq!(
        restored,
        PanelPreferencesWire {
            sidebar_collapsed: true,
            schedule_collapsed: true,
        }
    );

    // Dr. Bob in the same practice still sees his own default preferences.
    let bob_tok = sign_in_email(&client, &base, "bob@clinic.test").await;
    let bob_prefs: PanelPreferencesWire = client
        .get(format!("{base}/physician-state/panels"))
        .bearer_auth(&bob_tok)
        .send()
        .await
        .expect("GET bob panels")
        .error_for_status()
        .expect("200 GET bob panels")
        .json()
        .await
        .expect("decode bob panels");
    assert_eq!(bob_prefs, PanelPreferencesWire::default());

    server.abort();
    drop(store);
    drop(auth);
    teardown_postgres(&admin_url, &db_name).await;
}

#[derive(Clone)]
struct StubElationState {
    base_url: String,
    token_requests: Arc<AtomicUsize>,
    everything_requests: Arc<AtomicUsize>,
    include_fresh_a1c: Arc<AtomicBool>,
}

async fn stub_oauth_token(State(state): State<StubElationState>, body: String) -> Response {
    state.token_requests.fetch_add(1, Ordering::SeqCst);
    if !body.contains("grant_type=client_credentials")
        || !body.contains("client_id=coda-client")
        || !body.contains("client_secret=coda-secret")
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "invalid_client" })),
        )
            .into_response();
    }
    Json(json!({
        "access_token": "elation-access-token-1",
        "token_type": "Bearer",
        "expires_in": 3600
    }))
    .into_response()
}

fn check_stub_bearer(headers: &HeaderMap) -> bool {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        == Some("Bearer elation-access-token-1")
}

async fn stub_fhir_patients(
    State(state): State<StubElationState>,
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<BTreeMap<String, String>>,
) -> Response {
    if !check_stub_bearer(&headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    if params.get("page").map(String::as_str) == Some("2") {
        return Json(json!({
            "resourceType": "Bundle",
            "type": "searchset",
            "entry": [{
                "resource": {
                    "resourceType": "Patient",
                    "id": "pat-elation-2",
                    "name": [{ "given": ["Leo"], "family": "Park" }],
                    "birthDate": "1981-04-12",
                    "gender": "male",
                    "identifier": [{ "system": "urn:elation:mrn", "value": "EL-2002" }]
                }
            }]
        }))
        .into_response();
    }
    Json(json!({
        "resourceType": "Bundle",
        "type": "searchset",
        "link": [
            { "relation": "next", "url": format!("{}/fhir/Patient?page=2", state.base_url) }
        ],
        "entry": [{
            "resource": {
                "resourceType": "Patient",
                "id": "pat-elation-1",
                "name": [{ "given": ["Maya"], "family": "Patel" }],
                "birthDate": "1974-09-02",
                "gender": "female",
                "identifier": [{ "system": "urn:elation:mrn", "value": "EL-1001" }]
            }
        }]
    }))
    .into_response()
}

async fn stub_fhir_appointments(headers: HeaderMap) -> Response {
    if !check_stub_bearer(&headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    Json(json!({
        "resourceType": "Bundle",
        "type": "searchset",
        "entry": [
            {
                "resource": {
                    "resourceType": "Appointment",
                    "id": "appt-elation-1",
                    "status": "booked",
                    "description": "Diabetes follow-up",
                    "start": "2026-08-27T17:00:00Z",
                    "end": "2026-08-27T17:30:00Z",
                    "participant": [{
                        "actor": { "reference": "Patient/pat-elation-1" },
                        "status": "accepted"
                    }]
                }
            },
            {
                "resource": {
                    "resourceType": "Appointment",
                    "id": "appt-elation-bad",
                    "status": "booked",
                    "description": "Broken window appointment",
                    "start": "not-a-timestamp",
                    "end": "2026-08-27T18:30:00Z"
                }
            }
        ]
    }))
    .into_response()
}

async fn stub_fhir_everything(
    State(state): State<StubElationState>,
    headers: HeaderMap,
    AxumPath(patient_id): AxumPath<String>,
) -> Response {
    if !check_stub_bearer(&headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    state.everything_requests.fetch_add(1, Ordering::SeqCst);
    if patient_id == "pat-elation-2" {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "resourceType": "OperationOutcome" })),
        )
            .into_response();
    }
    let mut entries = vec![
        json!({
            "resource": {
                "resourceType": "Patient",
                "id": patient_id,
                "name": [{ "given": ["Maya"], "family": "Patel" }],
                "birthDate": "1974-09-02",
                "gender": "female"
            }
        }),
        json!({
            "resource": {
                "resourceType": "Observation",
                "id": "obs-k-1",
                "status": "final",
                "subject": { "reference": format!("Patient/{patient_id}") },
                "code": { "text": "Serum Potassium" },
                "effectiveDateTime": "2026-08-26T12:00:00Z",
                "valueQuantity": { "value": 5.4, "unit": "mmol/L" }
            }
        }),
    ];
    if state.include_fresh_a1c.load(Ordering::SeqCst) {
        entries.push(json!({
            "resource": {
                "resourceType": "Observation",
                "id": "obs-new-a1c",
                "status": "final",
                "subject": { "reference": format!("Patient/{patient_id}") },
                "code": { "text": "Hemoglobin A1c" },
                "effectiveDateTime": "2026-08-27T15:00:00Z",
                "valueQuantity": { "value": 9.2, "unit": "%" }
            }
        }));
    }
    Json(json!({
        "resourceType": "Bundle",
        "type": "searchset",
        "entry": entries
    }))
    .into_response()
}

#[tokio::test]
async fn ac5_and_ac7_elation_fhir_oauth_roster_schedule_refresh_and_daily_repull() {
    let Some((admin_url, db_name, _db_url, store, auth)) = setup_postgres("ac5_ac7").await else {
        return;
    };
    let practice = "practice-elation-ac5";
    auth.ensure_practice(practice, PracticeClock::Fixed(PRACTICE_NOW_UNIX))
        .await
        .expect("ensure practice");

    // Start stub Elation FHIR R4 + OAuth2 server on ephemeral port.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind stub fhir");
    let addr = listener.local_addr().expect("stub fhir addr");
    let base_url = format!("http://{addr}");
    let stub_state = StubElationState {
        base_url: base_url.clone(),
        token_requests: Arc::new(AtomicUsize::new(0)),
        everything_requests: Arc::new(AtomicUsize::new(0)),
        include_fresh_a1c: Arc::new(AtomicBool::new(false)),
    };
    let stub_app = Router::new()
        .route("/oauth2/token", post(stub_oauth_token))
        .route("/fhir/Patient", get(stub_fhir_patients))
        .route("/fhir/Appointment", get(stub_fhir_appointments))
        .route("/fhir/Patient/{id}/$everything", get(stub_fhir_everything))
        .with_state(stub_state.clone());
    let stub_handle = tokio::spawn(async move {
        let _ = axum::serve(listener, stub_app).await;
    });

    let env_map: BTreeMap<&str, String> = BTreeMap::from([
        (BASE_URL_ENV, format!("{base_url}/fhir")),
        (TOKEN_URL_ENV, format!("{base_url}/oauth2/token")),
        (CLIENT_ID_ENV, "coda-client".to_owned()),
        (CLIENT_SECRET_ENV, "coda-secret".to_owned()),
    ]);
    let config = ElationFhirConfig::from_env(|key| env_map.get(key).cloned())
        .expect("valid elation env config")
        .expect("elation config present");
    let elation = Arc::new(
        ElationFhirClient::new(config)
            .expect("build elation client")
            .with_store(store.clone()),
    );

    let mut composite = CompositePatientSourceRefresh::new();
    let elation_refresh: Arc<dyn PatientSourceRefresh> = elation.clone();
    composite.push(elation_refresh);
    let store = store.with_source_refresh(Arc::new(composite));

    // 1. AC5: Read paginated FHIR Patient roster & sync into practice roster + schedule links + dispatch.
    let roster = elation.read_roster().await.expect("read paginated roster");
    assert_eq!(
        roster.patients.len(),
        2,
        "must follow FHIR Bundle next link across pages"
    );
    assert_eq!(
        stub_state.token_requests.load(Ordering::SeqCst),
        1,
        "OAuth2 bearer token must be cached across paginated requests"
    );
    let roster_summary = store
        .sync_practice_roster(
            &auth,
            practice,
            elation.source_id(),
            elation.source_label(),
            &roster.patients,
            PRACTICE_NOW_UNIX as i64,
        )
        .await
        .expect("sync elation roster");
    assert_eq!(roster_summary.roster_patients, 2);
    assert_eq!(roster_summary.newly_registered, 2);
    assert_eq!(roster_summary.seeded_links, 2);

    // 2. AC5: Read FHIR Appointment schedule -> partial failure on malformed entry does not lose valid appointment.
    let sync_outcome = store
        .sync_practice_schedule(&auth, practice, &*elation, PRACTICE_NOW_UNIX as i64)
        .await
        .expect("sync elation schedule");
    match sync_outcome {
        ScheduleSyncOutcome::Applied(summary) => {
            assert_eq!(summary.saved_appointments, 1);
            assert_eq!(summary.matched_appointments, 1);
            assert_eq!(summary.unmatched_appointments, 0);
            match summary.standing {
                ScheduleStanding::Usable {
                    source_id,
                    freshness,
                    ..
                } => {
                    assert_eq!(source_id.as_str(), ELATION_SOURCE_ID);
                    match freshness {
                        ScheduleFreshness::Partial { failures, .. } => {
                            assert_eq!(failures.as_slice().len(), 1);
                        }
                        other => panic!("expected Partial schedule freshness, got {other:?}"),
                    }
                }
                other => panic!("expected Usable standing, got {other:?}"),
            }
        }
        other => panic!("expected Applied schedule outcome, got {other:?}"),
    }

    // 3. AC5: Refresh pat-elation-1 ($everything succeeds) and pat-elation-2 ($everything 503 fails gracefully).
    let ws1 = workspace_id(ELATION_SOURCE_ID, "pat-elation-1");
    let ws2 = workspace_id(ELATION_SOURCE_ID, "pat-elation-2");
    let r1 = store
        .refresh_patient_sources_for_practice(
            &PracticeScope {
                practice: practice.into(),
                workspace: ws1.clone(),
                job_id: "elation-refresh-1".into(),
            },
            &auth,
            &RefreshRequest {
                command_key: RefreshCommandKey::new("cmd-elation-1").unwrap(),
                target: RefreshTarget::AllSources,
            },
            store.source_refresh(),
        )
        .await
        .expect("refresh ws1");
    assert!(
        r1.sources
            .iter()
            .any(|s| s.source_id.as_str() == ELATION_SOURCE_ID
                && matches!(s.outcome, SourceRefreshOutcome::Updated { .. }))
    );
    let sources_ws1 = store.sources(&ws1).await.expect("ws1 sources");
    assert!(
        sources_ws1
            .sources
            .iter()
            .any(|r| r.candidate_id == format!("{ELATION_SOURCE_ID}/Observation/obs-k-1")),
        "ws1 must store fetched Elation Observation/obs-k-1: {:?}",
        sources_ws1
            .sources
            .iter()
            .map(|r| &r.candidate_id)
            .collect::<Vec<_>>()
    );

    let r2 = store
        .refresh_patient_sources_for_practice(
            &PracticeScope {
                practice: practice.into(),
                workspace: ws2.clone(),
                job_id: "elation-refresh-2".into(),
            },
            &auth,
            &RefreshRequest {
                command_key: RefreshCommandKey::new("cmd-elation-2").unwrap(),
                target: RefreshTarget::AllSources,
            },
            store.source_refresh(),
        )
        .await
        .expect("refresh ws2 returns recorded failure rather than aborting");
    assert!(
        r2.sources
            .iter()
            .any(|s| s.source_id.as_str() == ELATION_SOURCE_ID
                && matches!(s.outcome, SourceRefreshOutcome::Failed { .. })),
        "failed Elation $everything for ws2 must record SourceRefreshOutcome::Failed: {r2:?}"
    );

    // 4. AC7 (`P2.1` / `Rule.DailyRePull`): New HbA1c arrives in Elation; scheduled dispatch wake
    // using `RefreshingSourceChecker` re-pulls from external FHIR endpoint and dispatches a review!
    stub_state.include_fresh_a1c.store(true, Ordering::SeqCst);
    let calls_before = stub_state.everything_requests.load(Ordering::SeqCst);
    let checker = RefreshingSourceChecker::new(store.clone(), &auth);
    let skills = load_skills();
    let roles = test_roles();
    let receiver = PatientReviewReceiver::new(&store, &roles, &skills, &auth);
    let wake_now = (PRACTICE_NOW_UNIX as i64) + 86_400; // next daily cadence check
    let dispatch = store
        .wake_practice_dispatch(&auth, practice, wake_now, &checker, &receiver)
        .await
        .expect("wake_practice_dispatch with RefreshingSourceChecker");
    assert!(
        stub_state.everything_requests.load(Ordering::SeqCst) > calls_before,
        "RefreshingSourceChecker must invoke live external Elation $everything re-pull on wake"
    );
    let updated_sources = store.sources(&ws1).await.expect("updated ws1 sources");
    assert!(
        updated_sources
            .sources
            .iter()
            .any(|r| r.candidate_id == format!("{ELATION_SOURCE_ID}/Observation/obs-new-a1c")),
        "daily re-pull must persist newly appeared Observation/obs-new-a1c into workspace sources"
    );
    assert!(
        dispatch
            .started_reviews
            .iter()
            .any(|s| s.patient_workspace_id == ws1),
        "new externally pulled observation must dispatch a patient review for ws1: {dispatch:?}"
    );

    // 5. AC5: Verify Hint Health roster & appointment parsers directly as well.
    let (hint_roster, roster_issues) = parse_hint_roster(&json!({
        "patients": [{
            "id": "hint-99",
            "first_name": "Nadia",
            "last_name": "Chen",
            "dob": "1968-11-03",
            "gender": "female"
        }]
    }))
    .expect("parse hint roster");
    assert!(roster_issues.is_empty());
    assert_eq!(hint_roster.len(), 1);
    assert_eq!(hint_roster[0].display_name, "Nadia Chen");

    let hint_source_id = ScheduleSourceId::new(HINT_SOURCE_ID).unwrap();
    let hint_appts = parse_hint_appointments(
        &hint_source_id,
        &json!({
            "appointments": [{
                "id": "hint-appt-1",
                "patient_id": "hint-99",
                "status": "scheduled",
                "reason": "Annual physical",
                "start_at": "2026-08-27T18:00:00Z",
                "end_at": "2026-08-27T18:45:00Z"
            }]
        }),
    )
    .expect("parse hint appointments");
    match hint_appts {
        ScheduleDelivery::Complete { appointments } => {
            assert_eq!(appointments.len(), 1);
            assert_eq!(appointments[0].label(), "Annual physical");
        }
        other => panic!("expected Complete hint schedule delivery, got {other:?}"),
    }

    stub_handle.abort();
    drop(store);
    drop(auth);
    teardown_postgres(&admin_url, &db_name).await;
}

#[tokio::test]
async fn ac6_hint_chrome_bridge_pair_lease_capture_roster_appointments_and_auth() {
    let Some((admin_url, db_name, db_url, store, auth)) = setup_postgres("ac6").await else {
        return;
    };
    let practice = "practice-hint-ac6";
    auth.ensure_practice(practice, PracticeClock::Fixed(PRACTICE_NOW_UNIX))
        .await
        .expect("ensure practice");
    let admin_id = PhysicianIdentity {
        practice_id: practice.into(),
        physician_id: "dr-admin".into(),
    };
    let staff_id = PhysicianIdentity {
        practice_id: practice.into(),
        physician_id: "dr-nonadmin".into(),
    };
    auth.ensure_account(
        &admin_id.practice_id,
        &admin_id.physician_id,
        "admin@hintclinic.test",
        "Dr. Admin",
        true,
        PRACTICE_NOW_UNIX,
    )
    .await
    .expect("seed admin");
    auth.ensure_account(
        &staff_id.practice_id,
        &staff_id.physician_id,
        "doc@hintclinic.test",
        "Dr. Staff",
        false,
        PRACTICE_NOW_UNIX,
    )
    .await
    .expect("seed non-admin");

    let bus: Arc<dyn Bus> = Arc::new(LocalBus::new(64));
    let mut bus_rx = bus.subscribe();
    let chat = Arc::new(DurableChat::new(
        store.clone(),
        load_skills(),
        &test_roles(),
    ));
    let app = authed_postgres_router(
        durable_site(chat, Arc::clone(&bus)),
        &db_url,
        store.clone(),
        Arc::clone(&bus),
    )
    .await;
    let (addr, server) = spawn_router(app).await;
    let base = format!("http://{addr}");
    let client = reqwest::Client::new();

    let staff_tok = sign_in_email(&client, &base, "doc@hintclinic.test").await;
    let admin_tok = sign_in_email(&client, &base, "admin@hintclinic.test").await;

    // Non-admin cannot pair a Hint bridge.
    let forbidden = client
        .post(format!("{base}/connections/hint/bridge/pair"))
        .bearer_auth(&staff_tok)
        .json(&json!({}))
        .send()
        .await
        .expect("non-admin pair")
        .status();
    assert_eq!(forbidden, StatusCode::FORBIDDEN);

    // Admin creates pairing -> receives pair_id + pair_key once.
    let pair_resp: Value = client
        .post(format!("{base}/connections/hint/bridge/pair"))
        .bearer_auth(&admin_tok)
        .json(&json!({ "source_label": "Hint Health (Clinic)" }))
        .send()
        .await
        .expect("admin pair")
        .error_for_status()
        .expect("201 pair")
        .json()
        .await
        .expect("decode pair");
    let pair_id = pair_resp["pair_id"].as_str().expect("pair_id");
    let pair_key = pair_resp["pair_key"].as_str().expect("pair_key");
    let pair_header = format!("{pair_id}:{pair_key}");

    // Connector without X-Coda-Pair or with wrong key receives 401.
    assert_eq!(
        client
            .post(format!("{base}/connections/hint/bridge/lease"))
            .json(&json!({}))
            .send()
            .await
            .expect("missing pair header")
            .status(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        client
            .post(format!("{base}/connections/hint/bridge/lease"))
            .header(PAIR_HEADER, format!("{pair_id}:wrong-key"))
            .json(&json!({}))
            .send()
            .await
            .expect("bad pair header")
            .status(),
        StatusCode::UNAUTHORIZED
    );

    // Admin queues a roster job -> connector leases -> wrong lease returns 409 -> right lease delivers.
    let queued: Value = client
        .post(format!("{base}/connections/hint/bridge/jobs"))
        .bearer_auth(&admin_tok)
        .json(&json!({ "read": "roster" }))
        .send()
        .await
        .expect("queue roster job")
        .error_for_status()
        .expect("202 queue job")
        .json()
        .await
        .expect("decode queued job");
    let job_id = queued["job_id"].as_str().expect("job_id");

    let leased: Value = client
        .post(format!("{base}/connections/hint/bridge/lease"))
        .header(PAIR_HEADER, &pair_header)
        .json(&json!({ "lease_seconds": 120 }))
        .send()
        .await
        .expect("lease job")
        .error_for_status()
        .expect("200 lease")
        .json()
        .await
        .expect("decode leased");
    assert_eq!(leased["job"]["id"].as_str(), Some(job_id));
    let lease_id = leased["job"]["lease"].as_str().expect("lease id");

    let roster_payload = json!({
        "patients": [{
            "id": "hint-pat-1",
            "first_name": "Nadia",
            "last_name": "Chen",
            "dob": "1968-11-03",
            "gender": "female"
        }]
    });
    assert_eq!(
        client
            .post(format!("{base}/connections/hint/bridge/deliver"))
            .header(PAIR_HEADER, &pair_header)
            .json(&json!({
                "job_id": job_id,
                "lease": "wrong-lease-id",
                "read": "roster",
                "payload": roster_payload,
            }))
            .send()
            .await
            .expect("deliver wrong lease")
            .status(),
        StatusCode::CONFLICT
    );

    let delivered: Value = client
        .post(format!("{base}/connections/hint/bridge/deliver"))
        .header(PAIR_HEADER, &pair_header)
        .json(&json!({
            "job_id": job_id,
            "lease": lease_id,
            "read": "roster",
            "payload": roster_payload,
        }))
        .send()
        .await
        .expect("deliver right lease")
        .error_for_status()
        .expect("200 deliver")
        .json()
        .await
        .expect("decode deliver");
    assert_eq!(delivered["outcome"], "delivered");
    assert_eq!(delivered["newly_registered"], 1);

    // Connector delivers a real Hint ChartCapture -> filed through `ChartCapture::file(...)`.
    let capture_payload = json!({
        "request": { "patient_id": "hint-pat-1" },
        "sections": [
            {
                "section": "patient",
                "state": "done",
                "result": {
                    "body": {
                        "items": [{
                            "body": {
                                "id": "hint-pat-1",
                                "first_name": "Nadia",
                                "last_name": "Chen",
                                "dob": "1968-11-03",
                                "gender": "female"
                            },
                            "unavailable_parts": []
                        }],
                        "failed_parts": []
                    }
                }
            },
            {
                "section": "notes",
                "state": "done",
                "result": {
                    "body": {
                        "items": [{
                            "body": {
                                "id": "note-101",
                                "noteType": "NOTE",
                                "createdAt": "2026-08-20T14:00:00Z",
                                "title": "Hypertension follow-up",
                                "body": "BP 134/82 on lisinopril 20 mg daily."
                            },
                            "unavailable_parts": []
                        }],
                        "failed_parts": []
                    }
                }
            }
        ]
    });
    let capture_summary: Value = client
        .post(format!("{base}/connections/hint/bridge/capture"))
        .header(PAIR_HEADER, &pair_header)
        .json(&json!({
            "patient_id": "hint-pat-1",
            "capture": capture_payload,
        }))
        .send()
        .await
        .expect("ingest capture")
        .error_for_status()
        .expect("200 capture")
        .json()
        .await
        .expect("decode capture");
    let ws = workspace_id(HINT_SOURCE_ID, "hint-pat-1");
    assert_eq!(capture_summary["workspace"].as_str(), Some(ws.as_str()));
    assert!(capture_summary["filed_records"].as_u64().unwrap() >= 1);
    assert!(
        !store
            .sources(&ws)
            .await
            .expect("hint sources")
            .sources
            .is_empty()
    );

    // Place patient on admin's panel, ingest Hint appointments, and confirm `GET /practice-day` shows it!
    auth.set_membership(&admin_id, &ws, Membership::Allowed)
        .await
        .expect("add hint patient to panel");
    let appts_resp: Value = client
        .post(format!("{base}/connections/hint/bridge/appointments"))
        .header(PAIR_HEADER, &pair_header)
        .json(&json!({
            "payload": {
                "appointments": [{
                    "id": "hint-appt-today",
                    "patient_id": "hint-pat-1",
                    "status": "scheduled",
                    "reason": "Hypertension follow-up",
                    "start_at": "2026-08-27T18:00:00Z",
                    "end_at": "2026-08-27T18:30:00Z"
                }]
            }
        }))
        .send()
        .await
        .expect("ingest appointments")
        .error_for_status()
        .expect("200 appointments")
        .json()
        .await
        .expect("decode appointments");
    assert_eq!(appts_resp["outcome"], "applied");
    assert_eq!(appts_resp["matched_appointments"], 1);

    let day: PracticeDayResponse = client
        .get(format!("{base}/practice-day"))
        .bearer_auth(&admin_tok)
        .send()
        .await
        .expect("GET /practice-day")
        .error_for_status()
        .expect("200 /practice-day")
        .json()
        .await
        .expect("decode /practice-day");
    assert_eq!(day.visits.len(), 1);
    assert_eq!(
        day.visits[0].patient_display_name.as_deref(),
        Some("Nadia Chen")
    );
    assert_eq!(day.visits[0].label, "Hypertension follow-up");

    // Verify durable Hint bridge schedule & refresh adapters read directly from Postgres.
    let hint_schedule = HintBridgeScheduleSource::new(store.clone(), practice.to_owned());
    let delivery = hint_schedule
        .read_schedule()
        .await
        .expect("HintBridgeScheduleSource read");
    match delivery {
        ScheduleDelivery::Complete { appointments } => assert_eq!(appointments.len(), 1),
        other => panic!("expected Complete HintBridgeScheduleSource delivery, got {other:?}"),
    }
    let hint_refresh = HintBridgeSourceRefresh::new(store.clone());
    let configured = hint_refresh
        .configured_sources(practice, &ws)
        .await
        .expect("HintBridgeSourceRefresh configured_sources");
    assert_eq!(configured.len(), 1);
    assert_eq!(configured[0].as_str(), HINT_SOURCE_ID);

    // Verify bus received both WorkspaceChanged and Resync events.
    let mut saw_ws_changed = false;
    let mut saw_resync = false;
    while let Ok(Ok(ev)) = tokio::time::timeout(Duration::from_millis(200), bus_rx.recv()).await {
        match ev {
            Event::WorkspaceChanged { workspace_id } if workspace_id == ws => {
                saw_ws_changed = true;
            }
            Event::Resync => saw_resync = true,
            _ => {}
        }
    }
    assert!(
        saw_ws_changed,
        "Hint capture must publish Event::WorkspaceChanged"
    );
    assert!(saw_resync, "Hint bridge ingests must publish Event::Resync");

    server.abort();
    drop(store);
    drop(auth);
    teardown_postgres(&admin_url, &db_name).await;
}
