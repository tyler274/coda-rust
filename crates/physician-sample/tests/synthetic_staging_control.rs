//! End-to-end verification of staging synthetic event intake, rounds, visit
//! preparation, source failure, refusal, reset, production exclusion, and replay.

#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
#[path = "scale/mod.rs"]
mod scale;

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex as StdMutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use axum::Router;
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::response::Response;
use axum::routing::post;
use base64::Engine as _;
use devtools::Browser;
use local_product::initial_understanding::{
    HealthSystemStanding, IndexedAnchor, KnownHealthSystem, OperatingIndex, SaveUnderstandingArgs,
    SummaryClaimInput,
};
use local_product::physician_sources::workspace_id;
use local_product::runtime::Store;
use local_product::runtime::chat_operations::{
    MemoryProvenance, PatientMemoryId, RecordMemoryCorrection,
};
use local_product::runtime::concerns::ExactRecordReference;
use local_product::visit_preparation::{
    BriefingClaimInput, BriefingDecisionInput, SaveBriefingArgs, VisitSelectionContext,
};
use physician_sample::STAGING_PRACTICE_ID;
use physician_sample::hosting::{
    self, STAGING_SYNTHETIC_CONTROL_ENV, STAGING_SYNTHETIC_WORKDIR_ENV,
    StagingSyntheticControlConfig,
};
use physician_sample::synthetic_control::{
    ChartEventRecordKind, PRODUCTION_ROUTE_MANIFEST, PatientSelector, PracticeSyntheticStatus,
    RoundsExecutionReport, STAGING_SYNTHETIC_ROUTE_MANIFEST, SavedSyntheticEvent,
    SourceHealthDirective, StagingSyntheticSources, SyntheticEventSpec,
    SyntheticPatientListResponse, SyntheticSourceKind, VisitChangeStatus,
    VisitPreparationExecutionReport, published_route_manifest,
};
use serde_json::{Value, json};
use tokio::sync::Mutex;
use tower::ServiceExt;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
static BUILT_SITE: OnceLock<PathBuf> = OnceLock::new();

fn test_lock() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

const BASELINE_NOTE_TEXT: &str = "CLINIC NOTE (2026-08-12): Well-controlled essential hypertension on lisinopril 10 mg daily; baseline serum potassium 4.2 mmol/L.";

fn build_synthetic_bank(tag: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("unix clock")
        .as_nanos();
    let dir = env::temp_dir().join(format!(
        "coda-synthetic-staging-bank-{tag}-{}-{nanos:x}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("calendar")).expect("create calendar dir");

    let encoded_note =
        base64::engine::general_purpose::STANDARD.encode(BASELINE_NOTE_TEXT.as_bytes());
    for (source_id, mrn, given, family, dob) in [
        ("MRN-ALICE", "00090001", "Alice", "Carter", "1972-03-18"),
        ("MRN-BOB", "00090002", "Bob", "Diaz", "1965-11-04"),
    ] {
        let chart_dir = dir.join("patients").join(source_id);
        fs::create_dir_all(&chart_dir).expect("create patient chart dir");
        let patient = json!({
            "resourceType": "Patient",
            "id": source_id,
            "identifier": [{ "system": "urn:oid:2.16.840.1.113883.19.5", "value": mrn }],
            "name": [{ "family": family, "given": [given] }],
            "gender": "female",
            "birthDate": dob,
        });
        let note = json!({
            "resourceType": "DocumentReference",
            "id": "note-baseline-1",
            "status": "current",
            "subject": { "reference": format!("Patient/{source_id}") },
            "description": "Routine Primary Care Note",
            "date": "2026-08-12T12:05:00Z",
            "content": [{
                "attachment": {
                    "contentType": "text/plain; charset=utf-8",
                    "data": encoded_note,
                }
            }],
        });
        fs::write(
            chart_dir.join("Patient.ndjson"),
            format!("{}\n", serde_json::to_string(&patient).unwrap()),
        )
        .expect("write Patient.ndjson");
        fs::write(
            chart_dir.join("DocumentReference.ndjson"),
            format!("{}\n", serde_json::to_string(&note).unwrap()),
        )
        .expect("write DocumentReference.ndjson");
    }

    let baseline_appt = json!({
        "resourceType": "Appointment",
        "id": "appt-alice-1",
        "status": "booked",
        "description": "Routine Hypertension Follow-up",
        "start": "2026-08-27T19:30:00Z",
        "end": "2026-08-27T20:00:00Z",
        "participant": [{
            "actor": { "reference": "Patient/MRN-ALICE" },
            "status": "accepted"
        }]
    });
    fs::write(
        dir.join("calendar").join("Appointment.ndjson"),
        format!("{}\n", serde_json::to_string(&baseline_appt).unwrap()),
    )
    .expect("write Appointment.ndjson");

    dir
}

fn snapshot_directory_bytes(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn walk(base: &Path, current: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        let mut entries: Vec<_> = fs::read_dir(current)
            .expect("read directory")
            .map(|entry| entry.expect("dir entry"))
            .collect();
        entries.sort_by_key(fs::DirEntry::file_name);
        for entry in entries {
            let path = entry.path();
            if entry.file_type().expect("file type").is_dir() {
                walk(base, &path, out);
            } else {
                let rel = path
                    .strip_prefix(base)
                    .expect("relative path")
                    .to_path_buf();
                out.insert(rel, fs::read(&path).expect("read file bytes"));
            }
        }
    }
    let mut snapshot = BTreeMap::new();
    if root.exists() {
        walk(root, root, &mut snapshot);
    }
    snapshot
}

#[tokio::test]
async fn ac4_source_refuses_unacceptable_events_without_saving() {
    let base_bank = build_synthetic_bank("ac4-unit");
    let work_bank = base_bank.with_file_name(format!(
        "{}-workdir",
        base_bank.file_name().unwrap().to_string_lossy()
    ));
    let config = StagingSyntheticControlConfig {
        base_bank_dir: base_bank.clone(),
        working_bank_dir: work_bank.clone(),
    };
    let sources = StagingSyntheticSources::new(&config, 1_787_846_400);
    sources
        .ensure_working_bank_initialized()
        .expect("initialize working bank");

    let before = snapshot_directory_bytes(&work_bank);
    assert!(!before.is_empty(), "working bank must contain base charts");

    // 1. Unknown patient selector.
    let err = sources
        .accept_event(
            SyntheticEventSpec::FreeFormChartRecord {
                patient: PatientSelector::MedicalRecordNumber("99999999".into()),
                record_id: Some("obs-unknown".into()),
                kind: ChartEventRecordKind::LabResult,
                title: "Potassium".into(),
                text: "Potassium 5.9 mmol/L".into(),
                effective_at: "2026-08-27T15:00:00Z".into(),
            },
            1_787_846_400,
        )
        .await
        .expect_err("unknown patient must be refused");
    assert_eq!(err.code(), "unknown_patient");
    assert!(err.to_string().contains("mrn=99999999"), "{err}");

    // 2. Blank title.
    let err = sources
        .accept_event(
            SyntheticEventSpec::FreeFormChartRecord {
                patient: PatientSelector::MedicalRecordNumber("00090001".into()),
                record_id: Some("obs-blank-title".into()),
                kind: ChartEventRecordKind::LabResult,
                title: "   ".into(),
                text: "Potassium 5.9 mmol/L".into(),
                effective_at: "2026-08-27T15:00:00Z".into(),
            },
            1_787_846_400,
        )
        .await
        .expect_err("blank title must be refused");
    assert_eq!(err.code(), "empty_record_title");

    // 3. Blank clinical text.
    let err = sources
        .accept_event(
            SyntheticEventSpec::FreeFormChartRecord {
                patient: PatientSelector::MedicalRecordNumber("00090001".into()),
                record_id: Some("obs-blank-text".into()),
                kind: ChartEventRecordKind::ClinicalNote,
                title: "Progress Note".into(),
                text: "\n\t  ".into(),
                effective_at: "2026-08-27T15:00:00Z".into(),
            },
            1_787_846_400,
        )
        .await
        .expect_err("blank clinical text must be refused");
    assert_eq!(err.code(), "empty_record_text");

    // 4. Invalid clinical date.
    let err = sources
        .accept_event(
            SyntheticEventSpec::FreeFormChartRecord {
                patient: PatientSelector::MedicalRecordNumber("00090001".into()),
                record_id: Some("obs-bad-date".into()),
                kind: ChartEventRecordKind::LabResult,
                title: "Potassium".into(),
                text: "Potassium 5.9 mmol/L".into(),
                effective_at: "27-Aug-2026".into(),
            },
            1_787_846_400,
        )
        .await
        .expect_err("non-ISO date must be refused");
    assert_eq!(err.code(), "invalid_timestamp");

    // 5. Non-text binary attachment on raw FHIR resource.
    let err = sources
        .accept_event(
            SyntheticEventSpec::RawFhirChartResource {
                patient: PatientSelector::SourcePatientId("MRN-ALICE".into()),
                resource: json!({
                    "resourceType": "DocumentReference",
                    "id": "binary-scan",
                    "date": "2026-08-27T15:00:00Z",
                    "subject": { "reference": "Patient/MRN-ALICE" },
                    "content": [{
                        "attachment": {
                            "contentType": "image/tiff",
                            "data": "AAECAwQ=",
                        }
                    }]
                }),
            },
            1_787_846_400,
        )
        .await
        .expect_err("binary attachment must be refused by synthetic chart adapter");
    assert_eq!(err.code(), "unsupported_attachment_media");

    // 6. Reversed appointment window (`ends_at < starts_at`).
    let err = sources
        .accept_event(
            SyntheticEventSpec::VisitChange {
                patient: PatientSelector::SourcePatientId("MRN-ALICE".into()),
                appointment_id: "appt-alice-1".into(),
                status: VisitChangeStatus::Booked,
                starts_at: "2026-08-27T19:00:00Z".into(),
                ends_at: "2026-08-27T18:30:00Z".into(),
                label: "Reversed Window".into(),
            },
            1_787_846_400,
        )
        .await
        .expect_err("reversed visit window must be refused");
    assert_eq!(err.code(), "invalid_appointment");

    let after_six_refusals = snapshot_directory_bytes(&work_bank);
    assert_eq!(
        before, after_six_refusals,
        "refused events must leave the working bank byte-for-byte unchanged"
    );

    // 7. Source offline / failing rejects intake with `source_unavailable` and persists across restarts.
    sources.set_source_health(
        SyntheticSourceKind::Chart,
        SourceHealthDirective::Failing {
            reason: "simulated upstream EHR maintenance".into(),
        },
    );
    let reopened = StagingSyntheticSources::new(&config, 1_787_846_400);
    assert_eq!(
        reopened.health_snapshot().chart,
        SourceHealthDirective::Failing {
            reason: "simulated upstream EHR maintenance".into(),
        }
    );
    let before_offline_attempt = snapshot_directory_bytes(&work_bank);
    let err = reopened
        .accept_event(
            SyntheticEventSpec::FreeFormChartRecord {
                patient: PatientSelector::MedicalRecordNumber("00090001".into()),
                record_id: Some("obs-while-offline".into()),
                kind: ChartEventRecordKind::LabResult,
                title: "Potassium".into(),
                text: "Potassium 5.9 mmol/L".into(),
                effective_at: "2026-08-27T15:00:00Z".into(),
            },
            1_787_846_400,
        )
        .await
        .expect_err("offline source must refuse intake");
    assert_eq!(err.code(), "source_unavailable");

    let after_offline_attempt = snapshot_directory_bytes(&work_bank);
    assert_eq!(
        before_offline_attempt, after_offline_attempt,
        "refused offline event must leave working bank byte-for-byte unchanged"
    );

    reopened
        .restore_working_bank_from_base()
        .expect("restore working bank from base");
    assert_eq!(
        reopened.health_snapshot().chart,
        SourceHealthDirective::Healthy
    );
    let after_restore = snapshot_directory_bytes(&work_bank);
    assert_eq!(
        before, after_restore,
        "restore_working_bank_from_base must return working bank byte-for-byte to base"
    );

    let _ = fs::remove_dir_all(&base_bank);
    let _ = fs::remove_dir_all(&work_bank);
}

#[test]
fn ac6_product_code_contains_zero_synthetic_checks() {
    fn collect_non_test_rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<_> = fs::read_dir(dir)
            .unwrap_or_else(|err| panic!("read {}: {err}", dir.display()))
            .map(|entry| entry.expect("dir entry"))
            .collect();
        entries.sort_by_key(fs::DirEntry::file_name);
        for entry in entries {
            let path = entry.path();
            if entry.file_type().expect("file type").is_dir() {
                if path.file_name().and_then(|n| n.to_str()) == Some("tests") {
                    continue;
                }
                collect_non_test_rust_files(&path, out);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs")
                && path.file_name().and_then(|n| n.to_str()) != Some("tests.rs")
            {
                out.push(path);
            }
        }
    }

    fn strip_cfg_test_tail(source: &str) -> &str {
        if let Some(idx) = source.find("\n#[cfg(test)]") {
            let rest = &source[idx..];
            if rest.contains("mod tests") {
                return &source[..idx];
            }
        }
        source
    }

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let product_crates = [
        "crates/local-product/src",
        "crates/harness/src",
        "crates/patient-work/src",
        "crates/physician-wire/src",
        "crates/physician-client/src",
        "crates/physician-app/src",
    ];
    let forbidden_tokens = [
        "synthetic_control",
        "StagingSyntheticControl",
        "StagingSyntheticSources",
        "CODA_STAGING_SYNTHETIC",
        "roger-synthetic-practice",
        "STAGING_PRACTICE_ID",
        "synthetic-charts",
        "synthetic-practice-calendar",
    ];

    let mut files = Vec::new();
    for rel in product_crates {
        collect_non_test_rust_files(&repo_root.join(rel), &mut files);
    }
    assert!(
        !files.is_empty(),
        "must scan non-empty set of product crate source files"
    );

    for file in files {
        let raw = fs::read_to_string(&file).expect("read product source file");
        let text = strip_cfg_test_tail(&raw);
        for token in forbidden_tokens {
            assert!(
                !text.contains(token),
                "product source {} must not branch on or reference synthetic token `{token}`",
                file.display()
            );
        }
    }

    for sample_rel in [
        "crates/physician-sample/src/patients.rs",
        "crates/physician-sample/src/care.rs",
        "crates/physician-sample/src/durable.rs",
        "crates/physician-sample/src/live.rs",
    ] {
        let path = repo_root.join(sample_rel);
        let raw = fs::read_to_string(&path).expect("read physician-sample clinical source");
        let text = strip_cfg_test_tail(&raw);
        for token in [
            "synthetic_control",
            "StagingSyntheticControl",
            "StagingSyntheticSources",
            "CODA_STAGING_SYNTHETIC",
        ] {
            assert!(
                !text.contains(token),
                "physician-sample clinical module {} must not branch on `{token}`",
                path.display()
            );
        }
    }
}

#[tokio::test]
async fn ac7_production_omits_staging_synthetic_routes_and_refuses_startup() {
    let prod_routes = published_route_manifest(false);
    assert_eq!(prod_routes.as_slice(), PRODUCTION_ROUTE_MANIFEST);
    for route in &prod_routes {
        assert!(
            !route.path.starts_with("/staging-synthetic"),
            "production route manifest must not contain staging synthetic path: {route:?}"
        );
    }

    let staging_routes = published_route_manifest(true);
    for expected in STAGING_SYNTHETIC_ROUTE_MANIFEST {
        assert!(
            staging_routes.contains(expected),
            "staging route manifest missing {expected:?}"
        );
    }

    let base_bank = build_synthetic_bank("ac7-unit");
    let bank_str = base_bank.display().to_string();

    // Production environment refuses CODA_STAGING_SYNTHETIC_CONTROL=enabled both in
    // StagingSyntheticControlConfig::from_env and in hosting::validate_startup.
    for env_label in ["prod", "production", "live"] {
        let get = |key: &str| match key {
            "K_SERVICE" => Some("physician".to_owned()),
            "CODA_ENVIRONMENT" => Some(env_label.to_owned()),
            "PHYSICIAN_AUTH" => Some("required".to_owned()),
            "PHYSICIAN_DATABASE_URL" => Some("postgres://u:p@127.0.0.1:5432/coda".to_owned()),
            "PHYSICIAN_PUBLIC_ORIGIN" => Some("https://chords.codahealth.com".to_owned()),
            "PHYSICIAN_GMAIL_FROM" => Some("signin@codahealth.com".to_owned()),
            "PHYSICIAN_GMAIL_CLIENT_ID" => Some("id".to_owned()),
            "PHYSICIAN_GMAIL_CLIENT_SECRET" => Some("secret".to_owned()),
            "PHYSICIAN_GMAIL_REFRESH_TOKEN" => Some("refresh".to_owned()),
            STAGING_SYNTHETIC_CONTROL_ENV => Some("enabled".to_owned()),
            "PHYSICIAN_SYNTHETIC_CHARTS" => Some(bank_str.clone()),
            _ => None,
        };
        let cfg_err = StagingSyntheticControlConfig::from_env(get)
            .expect_err("non-staging profile must refuse StagingSyntheticControlConfig");
        assert!(
            cfg_err.to_string().contains(STAGING_SYNTHETIC_CONTROL_ENV),
            "expected {STAGING_SYNTHETIC_CONTROL_ENV} in config error for {env_label}, got: {cfg_err}"
        );
        let startup_err = hosting::validate_startup(get)
            .expect_err("non-staging profile must refuse hosted startup");
        assert!(
            startup_err
                .to_string()
                .contains("production and every other profile refuse to start"),
            "expected hosted production refusal for CODA_ENVIRONMENT={env_label}, got: {startup_err}"
        );
    }

    // Standard production router returns 404 Not Found for every staging-synthetic endpoint.
    let prod_app = physician_sample::router();
    for entry in STAGING_SYNTHETIC_ROUTE_MANIFEST {
        let method = Method::from_bytes(entry.method.as_bytes()).expect("valid HTTP method");
        let response = prod_app
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(entry.path)
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .expect("build request"),
            )
            .await
            .expect("oneshot response");
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "production router must return 404 for {} {}",
            entry.method,
            entry.path
        );
    }

    // Staging profile (`CODA_ENVIRONMENT=stg`) accepts valid configuration.
    let work_dir = base_bank.with_file_name("ac7-unit-workdir");
    let work_str = work_dir.display().to_string();
    let stg_cfg = StagingSyntheticControlConfig::from_env(|key| match key {
        "CODA_ENVIRONMENT" => Some("stg".to_owned()),
        STAGING_SYNTHETIC_CONTROL_ENV => Some("enabled".to_owned()),
        "PHYSICIAN_SYNTHETIC_CHARTS" => Some(bank_str.clone()),
        STAGING_SYNTHETIC_WORKDIR_ENV => Some(work_str.clone()),
        _ => None,
    })
    .expect("staging profile accepts valid synthetic control config")
    .expect("staging synthetic config present");
    assert_eq!(stg_cfg.base_bank_dir, base_bank);
    assert_eq!(stg_cfg.working_bank_dir, work_dir);

    let _ = fs::remove_dir_all(&base_bank);
    let _ = fs::remove_dir_all(&work_dir);
}

/// Ephemeral loopback PostgreSQL cluster started automatically when neither
/// `PHYSICIAN_DATABASE_URL` nor `PHYSICIAN_RUNTIME_TEST_DATABASE_URL` is set.
struct EphemeralPostgres {
    data_dir: PathBuf,
    admin_url: String,
}

impl EphemeralPostgres {
    fn start() -> Self {
        let port = scale::free_port();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unix clock")
            .as_nanos();
        let data_dir =
            PathBuf::from("/tmp").join(format!("coda-syn-stg-pg-{}-{nanos:x}", std::process::id()));
        fs::create_dir_all(&data_dir).expect("create ephemeral PGDATA");

        let init = Command::new("initdb")
            .args([
                "-U",
                "postgres",
                "-A",
                "trust",
                "--no-locale",
                "--encoding=UTF8",
                "-D",
            ])
            .arg(&data_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .expect("run initdb");
        assert!(
            init.status.success(),
            "initdb failed: {}",
            String::from_utf8_lossy(&init.stderr)
        );

        let log_file = data_dir.join("postgres.log");
        let pg_opts = format!("-p {port} -h 127.0.0.1 -k /tmp");
        let start = Command::new("pg_ctl")
            .arg("-D")
            .arg(&data_dir)
            .arg("-l")
            .arg(&log_file)
            .args(["-o", &pg_opts, "-w", "start"])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .expect("run pg_ctl start");
        assert!(
            start.status.success(),
            "pg_ctl start failed: {}",
            String::from_utf8_lossy(&start.stderr)
        );

        Self {
            data_dir,
            admin_url: format!("postgresql://postgres@127.0.0.1:{port}/postgres"),
        }
    }
}

impl Drop for EphemeralPostgres {
    fn drop(&mut self) {
        if self.data_dir.join("postmaster.pid").exists() {
            let _ = Command::new("pg_ctl")
                .arg("-D")
                .arg(&self.data_dir)
                .args(["-m", "fast", "-w", "stop"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        let _ = fs::remove_dir_all(&self.data_dir);
    }
}

#[derive(Clone, Default)]
struct ScriptedResponsesState {
    requests: Arc<StdMutex<Vec<Value>>>,
    seq: Arc<std::sync::atomic::AtomicU64>,
}

impl ScriptedResponsesState {
    fn recorded_requests(&self) -> Vec<Value> {
        self.requests.lock().expect("requests mutex").clone()
    }
}

async fn start_scripted_responses_server()
-> (u16, ScriptedResponsesState, tokio::task::JoinHandle<()>) {
    let state = ScriptedResponsesState::default();
    let app = Router::new()
        .route("/responses", post(handle_responses_post))
        .with_state(state.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind scripted responses server");
    let port = listener.local_addr().expect("local addr").port();
    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    (port, state, handle)
}

async fn handle_responses_post(
    State(state): State<ScriptedResponsesState>,
    body: axum::body::Bytes,
) -> Response {
    let payload: Value = serde_json::from_slice(&body).expect("Responses request JSON");
    state
        .requests
        .lock()
        .expect("requests mutex")
        .push(payload.clone());

    let seq = state.seq.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
    let resp_id = format!("resp_syn_stg_{seq}");
    let model = payload
        .get("model")
        .and_then(Value::as_str)
        .unwrap_or("gpt-6-luna");
    let tool_names: Vec<&str> = payload
        .get("tools")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|t| t.get("name").and_then(Value::as_str))
        .collect();

    let items = if tool_names.contains(&"complete_patient_review") {
        script_patient_review_turn(&payload, seq)
    } else if tool_names.contains(&"select_visit_preparations") {
        script_visit_selection_turn(&payload, seq)
    } else if tool_names.contains(&"save_visit_briefing") {
        script_visit_briefing_turn(&payload, seq)
    } else {
        vec![assistant_message_item(seq, "Acknowledged.")]
    };

    encode_responses_sse(&resp_id, model, &items)
}

fn tool_outputs_by_name(payload: &Value) -> Vec<(String, Value)> {
    let input = payload
        .get("input")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut call_names: BTreeMap<String, String> = BTreeMap::new();
    for item in &input {
        if item.get("type").and_then(Value::as_str) == Some("function_call")
            && let (Some(call_id), Some(name)) = (
                item.get("call_id").and_then(Value::as_str),
                item.get("name").and_then(Value::as_str),
            )
        {
            call_names.insert(call_id.to_owned(), name.to_owned());
        }
    }
    let mut outputs = Vec::new();
    for item in &input {
        if item.get("type").and_then(Value::as_str) == Some("function_call_output")
            && let Some(call_id) = item.get("call_id").and_then(Value::as_str)
            && let Some(tool_name) = call_names.get(call_id)
        {
            let raw_out = item.get("output").and_then(Value::as_str).unwrap_or("{}");
            let parsed = serde_json::from_str::<Value>(raw_out)
                .unwrap_or_else(|_| json!({ "raw": raw_out }));
            outputs.push((tool_name.clone(), parsed));
        }
    }
    outputs
}

fn function_call_item(seq: u64, slot: usize, name: &str, args: &Value) -> Value {
    json!({
        "id": format!("fc_{seq}_{slot}"),
        "type": "function_call",
        "status": "completed",
        "call_id": format!("call_{seq}_{slot}"),
        "name": name,
        "arguments": serde_json::to_string(args).expect("serialize tool arguments"),
    })
}

fn assistant_message_item(seq: u64, text: &str) -> Value {
    json!({
        "id": format!("msg_{seq}"),
        "type": "message",
        "role": "assistant",
        "status": "completed",
        "content": [{
            "type": "output_text",
            "text": text,
            "annotations": [],
        }],
    })
}

fn extract_search_references(search_output: &Value) -> Vec<(String, String)> {
    search_output
        .get("results")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let reference = entry.get("reference")?;
            let cid = reference.get("candidate_id")?.as_str()?.to_owned();
            let rev = reference.get("revision")?.as_str()?.to_owned();
            Some((cid, rev))
        })
        .collect()
}

fn script_patient_review_turn(payload: &Value, seq: u64) -> Vec<Value> {
    let outputs = tool_outputs_by_name(payload);
    let completed = outputs
        .iter()
        .any(|(name, _)| name == "complete_patient_review");
    if completed {
        return vec![assistant_message_item(
            seq,
            "Completed longitudinal chart review for elevated serum potassium (6.1 mmol/L).",
        )];
    }

    let read_done = outputs
        .iter()
        .any(|(name, _)| name == "read_patient_sources");
    let search_out = outputs
        .iter()
        .rev()
        .find(|(name, _)| name == "search_patient_information")
        .map(|(_, val)| val);

    match (search_out, read_done) {
        (None, _) => vec![function_call_item(
            seq,
            0,
            "search_patient_information",
            &json!({
                "query": "",
                "record_type": null,
                "after": null,
            }),
        )],
        (Some(search_val), false) => {
            let refs = extract_search_references(search_val);
            let references: Vec<Value> = refs
                .iter()
                .map(|(cid, rev)| {
                    json!({
                        "candidate_id": cid,
                        "revision": rev,
                        "cursor": null,
                    })
                })
                .collect();
            vec![function_call_item(
                seq,
                0,
                "read_patient_sources",
                &json!({
                    "references": references,
                    "max_result_bytes": 32_768,
                }),
            )]
        }
        (Some(search_val), true) => {
            let refs = extract_search_references(search_val);
            let (target_cid, target_rev) = refs
                .iter()
                .find(|(cid, _)| cid.starts_with("Observation/"))
                .or_else(|| refs.first())
                .cloned()
                .expect("at least one chart record reference");
            let exact_ref = ExactRecordReference {
                candidate_id: target_cid.clone(),
                revision: target_rev.clone(),
            };
            let cite = json!([&exact_ref]);
            let understanding_args = SaveUnderstandingArgs {
                expected_revision: Some(0),
                operating_index: OperatingIndex {
                    working_overview:
                        "Established hypertension patient with interval hyperkalemia (K+ 6.1 mmol/L)."
                            .into(),
                    known_health_systems: vec![KnownHealthSystem {
                        name: "Synthetic chart".into(),
                        standing: HealthSystemStanding::Connected {
                            source_id: "synthetic-charts".into(),
                        },
                        note: "Primary outpatient chart connected.".into(),
                    }],
                    open_threads: vec![
                        "Verify non-hemolyzed repeat serum potassium and 12-lead ECG.".into(),
                    ],
                    anchors: vec![IndexedAnchor {
                        reference: exact_ref.clone(),
                        source_id: "synthetic-charts".into(),
                        topic: "Hyperkalemia 6.1 mmol/L on 2026-08-27".into(),
                    }],
                },
                claims: vec![SummaryClaimInput {
                    text: "Basic metabolic panel on 2026-08-27 shows serum potassium 6.1 mmol/L requiring prompt clinical review".into(),
                    citations: vec![exact_ref],
                }],
                material_gaps: vec![
                    "Repeat non-hemolyzed basic metabolic panel and 12-lead ECG not yet on file"
                        .into(),
                ],
                scope_note: None,
            };

            vec![
                function_call_item(
                    seq,
                    0,
                    "patient_concern",
                    &json!({
                        "operation": "raise",
                        "concern_id": "critical-potassium-elevated",
                        "urgency": "now",
                        "reason": "Critical potassium 6.1 mmol/L on basic metabolic panel requires prompt repeat electrolyte check and ECG evaluation.",
                        "records": cite,
                    }),
                ),
                function_call_item(
                    seq,
                    1,
                    "save_patient_understanding",
                    &serde_json::to_value(&understanding_args)
                        .expect("serialize SaveUnderstandingArgs"),
                ),
                function_call_item(
                    seq,
                    2,
                    "complete_patient_review",
                    &json!({
                        "disposition": "assessed",
                        "summary": "Assessed newly resulted serum potassium 6.1 mmol/L on 2026-08-27; raised immediate Today concern and updated patient summary.",
                        "citations": cite,
                        "basis_and_limitations": "Grounded in Observation/bmp-k-2026-08-27 (2026-08-27) from Synthetic chart; external emergency or ECG records are not connected.",
                        "unfinished_work": [],
                    }),
                ),
            ]
        }
    }
}

fn script_visit_selection_turn(payload: &Value, seq: u64) -> Vec<Value> {
    let outputs = tool_outputs_by_name(payload);
    if outputs
        .iter()
        .any(|(name, _)| name == "select_visit_preparations")
    {
        return vec![assistant_message_item(
            seq,
            "Selected upcoming visit for immediate pre-visit briefing preparation.",
        )];
    }

    let prompt_text = payload
        .get("input")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .flat_map(|item| {
            let mut texts = Vec::new();
            if let Some(s) = item.get("content").and_then(Value::as_str) {
                texts.push(s);
            }
            if let Some(parts) = item.get("content").and_then(Value::as_array) {
                for part in parts {
                    if let Some(s) = part.get("text").and_then(Value::as_str) {
                        texts.push(s);
                    }
                }
            }
            texts
        })
        .find(|text| text.contains("Scheduled visit selection context:\n"))
        .expect("visit selection prompt carries serialized VisitSelectionContext");
    let (_, context_json) = prompt_text
        .split_once("Scheduled visit selection context:\n")
        .expect("selection context marker");
    let parsed: VisitSelectionContext =
        serde_json::from_str(context_json.trim()).expect("deserialize VisitSelectionContext");

    let decisions: Vec<Value> = parsed
        .candidates
        .iter()
        .map(|cand| {
            json!({
                "source_id": cand.source_id,
                "appointment_id": cand.appointment_id,
                "starts_at_unix": cand.starts_at_unix,
                "action": "prepare_now",
                "reason": "Rescheduled potassium follow-up visit warrants immediate pre-visit synthesis.",
            })
        })
        .collect();

    vec![function_call_item(
        seq,
        0,
        "select_visit_preparations",
        &json!({ "decisions": decisions }),
    )]
}

fn script_visit_briefing_turn(payload: &Value, seq: u64) -> Vec<Value> {
    let outputs = tool_outputs_by_name(payload);
    if outputs
        .iter()
        .any(|(name, _)| name == "save_visit_briefing")
    {
        return vec![assistant_message_item(
            seq,
            "Saved pre-visit briefing for rescheduled potassium follow-up visit.",
        )];
    }

    let read_done = outputs
        .iter()
        .any(|(name, _)| name == "read_patient_sources");
    let search_out = outputs
        .iter()
        .rev()
        .find(|(name, _)| name == "search_patient_information")
        .map(|(_, val)| val);

    match (search_out, read_done) {
        (None, _) => vec![function_call_item(
            seq,
            0,
            "search_patient_information",
            &json!({
                "query": "",
                "record_type": null,
                "after": null,
            }),
        )],
        (Some(search_val), false) => {
            let refs = extract_search_references(search_val);
            let references: Vec<Value> = refs
                .iter()
                .map(|(cid, rev)| {
                    json!({
                        "candidate_id": cid,
                        "revision": rev,
                        "cursor": null,
                    })
                })
                .collect();
            vec![function_call_item(
                seq,
                0,
                "read_patient_sources",
                &json!({
                    "references": references,
                    "max_result_bytes": 32_768,
                }),
            )]
        }
        (Some(search_val), true) => {
            let refs = extract_search_references(search_val);
            let (target_cid, target_rev) = refs
                .iter()
                .find(|(cid, _)| cid.starts_with("Observation/"))
                .or_else(|| refs.first())
                .cloned()
                .expect("at least one chart reference for briefing");
            let exact_ref = ExactRecordReference {
                candidate_id: target_cid,
                revision: target_rev,
            };
            let briefing_args = SaveBriefingArgs {
                visit_card_orientation:
                    "Urgent potassium follow-up after K+ 6.1 mmol/L on 2026-08-27; review repeat BMP and lisinopril."
                        .into(),
                trajectory: vec![BriefingClaimInput {
                    text: "Established primary-care hypertension patient previously stable on lisinopril 10 mg daily.".into(),
                    uncertainty: None,
                    citations: vec![exact_ref.clone()],
                }],
                meaningful_changes: vec![BriefingClaimInput {
                    text: "Visit moved earlier to 2026-08-27 18:30 UTC as Urgent Potassium Follow-up after serum potassium 6.1 mmol/L.".into(),
                    uncertainty: None,
                    citations: vec![exact_ref.clone()],
                }],
                unresolved_issues: vec![BriefingClaimInput {
                    text: "Repeat non-hemolyzed electrolyte panel and 12-lead ECG confirmation remain pending ahead of the visit.".into(),
                    uncertainty: None,
                    citations: vec![exact_ref.clone()],
                }],
                visit_decisions: vec![BriefingDecisionInput {
                    decision: "Assess hyperkalemia etiology and antihypertensive regimen during the encounter.".into(),
                    clinical_rationale: "ACE-inhibitor therapy can contribute to elevated serum potassium.".into(),
                    options_or_questions: vec![
                        "Whether a repeat non-hemolyzed potassium and ECG have been completed.".into(),
                        "Whether temporary ACE-inhibitor hold or alternative blood-pressure therapy is warranted.".into(),
                    ],
                    uncertainty: None,
                    citations: vec![exact_ref.clone()],
                }],
                basis_and_limitations:
                    "Prepared from Synthetic chart records through 2026-08-27; external emergency department records are not connected."
                        .into(),
                basis_citations: vec![exact_ref],
                limits: Vec::new(),
                visit_concern: None,
            };

            vec![function_call_item(
                seq,
                0,
                "save_visit_briefing",
                &serde_json::to_value(&briefing_args).expect("serialize SaveBriefingArgs"),
            )]
        }
    }
}

fn encode_responses_sse(resp_id: &str, model: &str, items: &[Value]) -> Response {
    let mut events: Vec<Value> = Vec::new();
    let mut seq = 0_u64;
    let mut push_event = |mut obj: Value| {
        obj["sequence_number"] = json!(seq);
        seq += 1;
        events.push(obj);
    };

    push_event(json!({
        "type": "response.created",
        "response": { "id": resp_id },
    }));

    for (output_index, item) in items.iter().enumerate() {
        push_event(json!({
            "type": "response.output_item.added",
            "output_index": output_index,
            "item": item,
        }));
        if item.get("type").and_then(Value::as_str) == Some("function_call") {
            let item_id = item["id"].as_str().unwrap_or_default();
            let args = item["arguments"].as_str().unwrap_or_default();
            push_event(json!({
                "type": "response.function_call_arguments.delta",
                "output_index": output_index,
                "item_id": item_id,
                "delta": args,
            }));
            push_event(json!({
                "type": "response.function_call_arguments.done",
                "output_index": output_index,
                "item_id": item_id,
                "arguments": args,
            }));
        } else if item.get("type").and_then(Value::as_str) == Some("message") {
            let item_id = item["id"].as_str().unwrap_or_default();
            let part = &item["content"][0];
            let text = part["text"].as_str().unwrap_or_default();
            push_event(json!({
                "type": "response.content_part.added",
                "output_index": output_index,
                "content_index": 0,
                "item_id": item_id,
                "part": part,
            }));
            push_event(json!({
                "type": "response.output_text.delta",
                "output_index": output_index,
                "content_index": 0,
                "item_id": item_id,
                "delta": text,
            }));
            push_event(json!({
                "type": "response.output_text.done",
                "output_index": output_index,
                "content_index": 0,
                "item_id": item_id,
                "text": text,
            }));
            push_event(json!({
                "type": "response.content_part.done",
                "output_index": output_index,
                "content_index": 0,
                "item_id": item_id,
                "part": part,
            }));
        }
        push_event(json!({
            "type": "response.output_item.done",
            "output_index": output_index,
            "item": item,
        }));
    }

    push_event(json!({
        "type": "response.completed",
        "response": {
            "id": resp_id,
            "status": "completed",
            "model": model,
            "output": items,
            "usage": {
                "input_tokens": 180,
                "output_tokens": 64,
                "total_tokens": 244,
            }
        }
    }));

    let mut body = String::new();
    for event in events {
        body.push_str("data: ");
        body.push_str(&serde_json::to_string(&event).expect("serialize SSE event"));
        body.push_str("\n\n");
    }

    let mut response = Response::new(Body::from(body));
    response.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("text/event-stream"),
    );
    response
}

fn build_site() -> PathBuf {
    BUILT_SITE
        .get_or_init(|| {
            let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            let target = workspace.join("target/coda-physician-wasm");
            let wasm = target.join("wasm32-unknown-unknown/debug/physician-app.wasm");
            let mut cargo = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
            cargo.env_clear();
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
                if let Some(val) = env::var_os(key) {
                    cargo.env(key, val);
                }
            }
            let status = cargo
                .current_dir(&workspace)
                .env("CARGO_TARGET_DIR", &target)
                .args([
                    "build",
                    "-p",
                    "physician-app",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--features",
                    "web",
                ])
                .status()
                .expect("cargo build physician-app wasm");
            assert!(status.success(), "physician-app wasm build failed");

            let out_dir = workspace.join("target/physician-web-staging-synthetic");
            fs::create_dir_all(&out_dir).expect("create target/physician-web-staging-synthetic");
            let bindgen = env::var_os("WASM_BINDGEN")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("wasm-bindgen"));
            let status = Command::new(bindgen)
                .args([
                    "--target",
                    "web",
                    "--out-dir",
                    out_dir.to_str().unwrap(),
                    wasm.to_str().unwrap(),
                ])
                .status()
                .expect("wasm-bindgen");
            assert!(status.success(), "wasm-bindgen failed");
            out_dir
        })
        .clone()
}

fn run_cli(base_url: &str, token: &str, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_coda-staging-events"))
        .env("CODA_STAGING_URL", base_url)
        .env("CODA_STAGING_TOKEN", token)
        .args(args)
        .output()
        .expect("execute coda-staging-events")
}

fn run_cli_ok<T: serde::de::DeserializeOwned>(base_url: &str, token: &str, args: &[&str]) -> T {
    let out = run_cli(base_url, token, args);
    assert!(
        out.status.success(),
        "coda-staging-events {:?} failed with {:?}\nstdout:\n{}\nstderr:\n{}",
        args,
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    serde_json::from_slice(&out.stdout).unwrap_or_else(|err| {
        panic!(
            "parse JSON from coda-staging-events {:?}: {err}\n{}",
            args,
            String::from_utf8_lossy(&out.stdout)
        )
    })
}

fn save_browser_screenshot(browser: &mut Browser, dir: &Path, name: &str) -> PathBuf {
    fs::create_dir_all(dir).expect("create screenshot directory");
    let shot = browser
        .page()
        .call_session("Page.captureScreenshot", json!({ "format": "png" }));
    let encoded = shot["result"]["data"]
        .as_str()
        .expect("base64 PNG data from Page.captureScreenshot");
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .expect("decode PNG screenshot");
    assert!(
        bytes.starts_with(b"\x89PNG\r\n\x1a\n") && bytes.len() > 1_000,
        "captured screenshot {name} must be a non-trivial PNG"
    );
    let path = dir.join(format!("{name}.png"));
    fs::write(&path, bytes).expect("write PNG screenshot");
    path
}

fn set_browser_viewport(browser: &mut Browser, width: u32, height: u32) {
    browser.page().call_session(
        "Emulation.setDeviceMetricsOverride",
        json!({
            "width": width,
            "height": height,
            "deviceScaleFactor": 1,
            "mobile": width < 600,
        }),
    );
    browser.page().call_session(
        "Emulation.setEmulatedMedia",
        json!({
            "features": [{ "name": "prefers-reduced-motion", "value": "reduce" }],
        }),
    );
}

fn navigate_and_wait(browser: &mut Browser, url: &str, needle: &str) -> String {
    let _ = browser.page().evaluate("window.stop(); true");
    browser.text_until(url, needle)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
#[ignore = "browser and Postgres qualification; run explicitly with --ignored"]
async fn staging_synthetic_end_to_end_walk_browser_and_cli() {
    let _guard = test_lock().lock().await;

    // Spin up an ephemeral loopback Postgres cluster automatically when no external
    // test database URL is supplied in the shell.
    let mut ephemeral_pg = None;
    if scale::gate_url().is_none() {
        let cluster = EphemeralPostgres::start();
        // SAFETY: `test_lock()` serializes environment mutation in this test binary.
        unsafe {
            env::set_var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL", &cluster.admin_url);
        }
        ephemeral_pg = Some(cluster);
    }

    let database = scale::Database::create("staging_synthetic_e2e")
        .await
        .expect("disposable Postgres database");
    {
        let auth = physician_auth::PostgresAuthStore::connect_without_migrations(&database.url)
            .await
            .expect("connect PostgresAuthStore");
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        auth.ensure_account(
            STAGING_PRACTICE_ID,
            "staging-physician",
            "physician@codahealth.test",
            "Staging Physician",
            true,
            now_secs,
        )
        .await
        .expect("ensure staging physician account without sample fixture patients");
    }

    let site_dir = build_site();
    let (stub_port, responses_state, stub_handle) = start_scripted_responses_server().await;

    let base_bank = build_synthetic_bank("e2e");
    let work_bank = base_bank.with_file_name(format!(
        "{}-workdir",
        base_bank.file_name().unwrap().to_string_lossy()
    ));
    let saved_event_file = base_bank.with_file_name(format!(
        "{}-saved-k-event.json",
        base_bank.file_name().unwrap().to_string_lossy()
    ));

    let mut env_map: BTreeMap<String, String> = scale::models::environment().collect();
    env_map.insert("CODA_ENVIRONMENT".into(), "stg".into());
    env_map.insert(STAGING_SYNTHETIC_CONTROL_ENV.into(), "enabled".into());
    env_map.insert(
        "PHYSICIAN_SYNTHETIC_CHARTS".into(),
        base_bank.display().to_string(),
    );
    env_map.insert(
        STAGING_SYNTHETIC_WORKDIR_ENV.into(),
        work_bank.display().to_string(),
    );
    env_map.insert("PHYSICIAN_DATABASE_URL".into(), database.url.clone());
    env_map.insert("PHYSICIAN_AUTH".into(), "required".into());
    env_map.insert("PHYSICIAN_AUTH_PROOF_TOKENS".into(), "1".into());
    env_map.insert(
        "PHYSICIAN_RESPONSES_STUB_PORT".into(),
        stub_port.to_string(),
    );
    env_map.insert("PHYSICIAN_SITE_DIR".into(), site_dir.display().to_string());

    // Ensure `physician_sample::site_dir()` and `auth::authed_router_with_bus` read
    // our built site bundle and required proof-token auth mode.
    // SAFETY: `test_lock()` is held for the entire test.
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site_dir);
        env::set_var("PHYSICIAN_AUTH", "required");
        env::set_var("PHYSICIAN_AUTH_PROOF_TOKENS", "1");
        env::remove_var("PHYSICIAN_UPSTREAM");
    }

    let (router, _recovery) =
        physician_sample::site_router_from_env(|key| env_map.get(key).cloned())
            .await
            .expect("initialize staging synthetic site router");
    let std_listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind staging server");
    std_listener
        .set_nonblocking(true)
        .expect("nonblocking staging listener");
    let addr = std_listener.local_addr().expect("local server addr");
    let base_url = format!("http://{addr}");
    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel::<()>();
    let server_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .expect("staging server runtime");
        rt.block_on(async move {
            let listener = tokio::net::TcpListener::from_std(std_listener).expect("tokio listener");
            let _ = axum::serve(listener, physician_sample::traced(router))
                .with_graceful_shutdown(async move {
                    let _ = stop_rx.await;
                })
                .await;
        });
    });

    // Wait for /health before launching Chromium.
    let http = reqwest::Client::new();
    let health_deadline = Instant::now() + Duration::from_secs(15);
    loop {
        if let Ok(resp) = http.get(format!("{base_url}/health")).send().await
            && resp.status().is_success()
        {
            break;
        }
        assert!(
            Instant::now() < health_deadline,
            "staging server did not answer /health"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Unauthenticated callers to `/staging-synthetic/*` must be refused with 401!
    let unauth_resp = http
        .get(format!("{base_url}/staging-synthetic/status"))
        .send()
        .await
        .expect("unauthenticated status check");
    assert_eq!(unauth_resp.status(), reqwest::StatusCode::UNAUTHORIZED);

    let shots_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/verification/screenshots/staging-synthetic");
    fs::create_dir_all(&shots_dir).expect("create screenshots output dir");

    // Sign in inside headless Chromium as `physician@codahealth.test`.
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    set_browser_viewport(&mut browser, 1440, 900);
    browser.text_until(&base_url, "Work email");
    browser.type_into("input[type=\"email\"]", "physician@codahealth.test");
    std::thread::sleep(Duration::from_millis(50));
    browser.click("button.control-primary");
    browser.wait_for("Nothing needs you right now.");
    // Clear expected pre-sign-in 401 probe entries so post-sign-in errors fail strictly.
    let _ = browser.take_page_errors();

    let token = browser
        .page()
        .evaluate("localStorage.getItem('coda.physician.session')")
        .as_str()
        .expect("physician session bearer token in localStorage")
        .to_owned();
    assert!(!token.is_empty());

    let alice_ws = workspace_id(physician_sample::synthetic_chart::SOURCE_ID, "MRN-ALICE");
    let bob_ws = workspace_id(physician_sample::synthetic_chart::SOURCE_ID, "MRN-BOB");

    // Baseline check via `coda-staging-events list` and `status`.
    let listed: SyntheticPatientListResponse = run_cli_ok(&base_url, &token, &["list"]);
    assert_eq!(listed.practice_id, STAGING_PRACTICE_ID);
    assert_eq!(listed.patients.len(), 2);
    assert_eq!(listed.patients[0].display_name, "Alice Carter");
    assert_eq!(listed.patients[0].patient_workspace_id, alice_ws);
    assert_eq!(listed.patients[0].chart_version, 1);
    assert_eq!(listed.patients[1].display_name, "Bob Diaz");
    assert_eq!(listed.patients[1].patient_workspace_id, bob_ws);
    assert_eq!(listed.patients[1].chart_version, 1);

    // =========================================================================
    // AC1 & AC6: Send a new lab result for Alice, verify Postgres is untouched
    // before rounds, run rounds, verify exactly 1 review executes, and verify
    // its result shows on Today and on Alice's Patient page!
    // =========================================================================
    let saved_ac1: SavedSyntheticEvent = run_cli_ok(
        &base_url,
        &token,
        &[
            "send-chart",
            "--patient",
            "mrn:00090001",
            "--id",
            "bmp-k-2026-08-27",
            "--kind",
            "lab_result",
            "--title",
            "Basic Metabolic Panel - Serum Potassium",
            "--text",
            "Potassium 6.1 mmol/L (Reference 3.5-5.1 mmol/L), Critically High.",
            "--effective-at",
            "2026-08-27T15:10:00Z",
            "--save-event",
            saved_event_file.to_str().unwrap(),
        ],
    );
    assert_eq!(saved_ac1.target_source, SyntheticSourceKind::Chart);
    assert_eq!(
        saved_ac1.resolved_workspace_id.as_deref(),
        Some(alice_ws.as_str())
    );
    assert_eq!(saved_ac1.native_locator, "Observation/bmp-k-2026-08-27");
    assert!(
        saved_event_file.is_file(),
        "--save-event must write JSON receipt"
    );

    // AC6 check: before `rounds`, PostgreSQL still has chart_version == 1 and 0 reviews!
    let pre_rounds_status: PracticeSyntheticStatus = run_cli_ok(&base_url, &token, &["status"]);
    let alice_pre = pre_rounds_status
        .patients
        .iter()
        .find(|p| p.patient_workspace_id == alice_ws)
        .expect("Alice in status");
    assert_eq!(
        alice_pre.chart_version, 1,
        "sending a synthetic event must write only to the working bank, never directly to PostgreSQL"
    );
    assert_eq!(pre_rounds_status.counts.reviews, 0);
    assert_eq!(pre_rounds_status.counts.open_concerns, 0);

    // Run rounds via CLI!
    let rounds_ac1: RoundsExecutionReport = run_cli_ok(&base_url, &token, &["rounds"]);
    assert_eq!(rounds_ac1.dispatch.checked_patients.len(), 2);
    assert!(rounds_ac1.dispatch.failed_checks.is_empty());
    assert_eq!(
        rounds_ac1.executed_reviews.len(),
        1,
        "only Alice changed, so exactly one patient review must run"
    );
    let rev_ac1 = &rounds_ac1.executed_reviews[0];
    assert_eq!(rev_ac1.patient_workspace_id, alice_ws);
    assert_eq!(rev_ac1.trigger_reason, "new records");
    assert_eq!(rev_ac1.status, "completed");
    assert_eq!(rev_ac1.completed_source_version, Some(2));
    assert_eq!(rev_ac1.action_count, 2);
    assert_eq!(rounds_ac1.open_concerns.len(), 1);
    assert_eq!(
        rounds_ac1.open_concerns[0].id.as_str(),
        "critical-potassium-elevated"
    );

    // Verify Today in Chromium (`1440x900` desktop and `390x844` phone).
    set_browser_viewport(&mut browser, 1440, 900);
    let today_text = navigate_and_wait(
        &mut browser,
        &base_url,
        "Critical potassium 6.1 mmol/L on basic metabolic panel",
    );
    assert!(today_text.contains("Alice Carter"), "{today_text}");
    std::thread::sleep(Duration::from_millis(250));
    save_browser_screenshot(&mut browser, &shots_dir, "ac1-today-desktop");

    set_browser_viewport(&mut browser, 390, 844);
    let today_phone_text = browser.wait_for("Critical potassium 6.1 mmol/L");
    assert!(
        today_phone_text.contains("Alice Carter"),
        "{today_phone_text}"
    );
    std::thread::sleep(Duration::from_millis(250));
    save_browser_screenshot(&mut browser, &shots_dir, "ac1-today-phone");

    // Verify Alice's Patient page in Chromium (`1440x900`).
    set_browser_viewport(&mut browser, 1440, 900);
    let alice_url = format!("{base_url}/?patient={alice_ws}");
    let patient_text = navigate_and_wait(
        &mut browser,
        &alice_url,
        "Basic metabolic panel on 2026-08-27 shows serum potassium 6.1 mmol/L requiring prompt clinical review",
    );
    assert!(patient_text.contains("Alice Carter"), "{patient_text}");
    assert!(patient_text.contains("Patient summary"), "{patient_text}");
    // Open citation chip 1 on Alice's summary to verify exact source record linkage!
    browser.click("section[aria-label=\"Patient summary\"] button[aria-label*=\"Source 1:\"]");
    browser.wait_for("Potassium 6.1 mmol/L");
    std::thread::sleep(Duration::from_millis(250));
    save_browser_screenshot(&mut browser, &shots_dir, "ac1-patient-desktop");

    // =========================================================================
    // AC2: Change Alice's upcoming visit and prepare it. Verify the briefing
    // is saved for that visit and shows on Today and on the Patient page!
    // =========================================================================
    let saved_visit: SavedSyntheticEvent = run_cli_ok(
        &base_url,
        &token,
        &[
            "send-visit",
            "--patient",
            "mrn:00090001",
            "--appointment-id",
            "appt-alice-1",
            "--status",
            "booked",
            "--starts-at",
            "2026-08-27T18:30:00Z",
            "--ends-at",
            "2026-08-27T19:00:00Z",
            "--label",
            "Urgent Potassium Follow-up",
        ],
    );
    assert_eq!(saved_visit.target_source, SyntheticSourceKind::Calendar);
    assert_eq!(saved_visit.native_locator, "Appointment/appt-alice-1");

    let prep_report: VisitPreparationExecutionReport =
        run_cli_ok(&base_url, &token, &["prepare-visits"]);
    assert_eq!(prep_report.wake_summary.completed_briefings.len(), 1);
    assert_eq!(prep_report.briefings.len(), 1);
    assert_eq!(prep_report.briefings[0].slot.appointment_id, "appt-alice-1");
    assert_eq!(
        prep_report.briefings[0].starts_at,
        "2026-08-27T18:30:00.000Z"
    );
    assert!(
        prep_report.briefings[0]
            .visit_card_orientation
            .contains("Urgent potassium follow-up after K+ 6.1 mmol/L")
    );

    // Check `/practice-day` wire JSON carries `state: "prepared"` and orientation!
    let day_json: Value = http
        .get(format!("{base_url}/practice-day"))
        .bearer_auth(&token)
        .send()
        .await
        .expect("GET /practice-day")
        .json()
        .await
        .expect("parse /practice-day JSON");
    assert_eq!(day_json["visits"][0]["label"], "Urgent Potassium Follow-up");
    assert_eq!(day_json["visits"][0]["preparation"]["state"], "prepared");
    assert!(
        day_json["visits"][0]["preparation"]["visit_card_orientation"]
            .as_str()
            .unwrap_or_default()
            .contains("Urgent potassium follow-up after K+ 6.1 mmol/L"),
        "{day_json}"
    );

    // Check Today (rescheduled 2:30 PM EDT slot) and Alice's Patient screen
    // (`Urgent Potassium Follow-up` + `Briefing ready` cue) in Chromium.
    let ac2_today = navigate_and_wait(&mut browser, &base_url, "2:30 PM");
    assert!(ac2_today.contains("Alice Carter"), "{ac2_today}");
    std::thread::sleep(Duration::from_millis(250));
    save_browser_screenshot(&mut browser, &shots_dir, "ac2-today-briefing-desktop");

    let ac2_patient = navigate_and_wait(&mut browser, &alice_url, "Urgent Potassium Follow-up");
    assert!(ac2_patient.contains("2:30 PM"), "{ac2_patient}");
    assert!(ac2_patient.contains("Briefing ready"), "{ac2_patient}");
    std::thread::sleep(Duration::from_millis(250));
    save_browser_screenshot(&mut browser, &shots_dir, "ac2-patient-briefing-desktop");

    // =========================================================================
    // AC3: Make the synthetic chart source fail. Verify the patient page says
    // the records could not be updated and keeps the last good pull time!
    // =========================================================================
    let before_fail_ws: Value = http
        .get(format!("{base_url}/patient-workspaces/{alice_ws}"))
        .bearer_auth(&token)
        .send()
        .await
        .expect("GET Alice workspace before failure")
        .json()
        .await
        .expect("parse Alice workspace JSON");
    let last_good_iso = before_fail_ws["connected_sources"][0]["state"]["synchronized_at"]
        .as_str()
        .expect("usable synchronized_at before source failure")
        .to_owned();

    let _: Value = run_cli_ok(
        &base_url,
        &token,
        &[
            "source-failure",
            "--source",
            "chart",
            "--mode",
            "failing",
            "--reason",
            "Synthetic EHR TLS handshake timed out",
            "--workspace",
            &alice_ws,
        ],
    );

    let after_fail_ws: Value = http
        .get(format!("{base_url}/patient-workspaces/{alice_ws}"))
        .bearer_auth(&token)
        .send()
        .await
        .expect("GET Alice workspace after failure")
        .json()
        .await
        .expect("parse Alice workspace JSON");
    assert_eq!(
        after_fail_ws["connected_sources"][0]["state"]["state"],
        "failed"
    );
    assert_eq!(
        after_fail_ws["connected_sources"][0]["state"]["usable_snapshot"]["synchronized_at"]
            .as_str(),
        Some(last_good_iso.as_str()),
        "failed refresh must preserve exact last_good_pulled_at snapshot timestamp"
    );

    set_browser_viewport(&mut browser, 1440, 900);
    let ac3_desktop = navigate_and_wait(&mut browser, &alice_url, "couldn't update");
    assert!(
        ac3_desktop.contains("Records as of") && ac3_desktop.contains(", couldn't update"),
        "expected failed refresh banner with preserved last good time:\n{ac3_desktop}"
    );
    assert!(
        ac3_desktop.contains("Synthetic chart · As of ")
            && ac3_desktop.contains("serum potassium 6.1 mmol/L"),
        "failed source refresh must preserve both last good read time and retained patient summary:\n{ac3_desktop}"
    );
    std::thread::sleep(Duration::from_millis(250));
    save_browser_screenshot(&mut browser, &shots_dir, "ac3-source-failure-desktop");

    set_browser_viewport(&mut browser, 390, 844);
    let ac3_phone = browser.wait_for("couldn't update");
    assert!(ac3_phone.contains("Records as of"), "{ac3_phone}");
    std::thread::sleep(Duration::from_millis(250));
    save_browser_screenshot(&mut browser, &shots_dir, "ac3-source-failure-phone");

    // Restore healthy chart adapter.
    let _: Value = run_cli_ok(
        &base_url,
        &token,
        &[
            "source-failure",
            "--source",
            "chart",
            "--mode",
            "healthy",
            "--workspace",
            &alice_ws,
        ],
    );

    // =========================================================================
    // AC4 (over live CLI + HTTP): Send events the source cannot accept; verify
    // non-zero exit + structured refusal reason and zero saved mutation!
    // =========================================================================
    let status_before_refusal: PracticeSyntheticStatus = run_cli_ok(&base_url, &token, &["status"]);
    let refused_chart = run_cli(
        &base_url,
        &token,
        &[
            "send-chart",
            "--patient",
            "mrn:99999999",
            "--kind",
            "lab_result",
            "--title",
            "Sodium",
            "--text",
            "Sodium 140 mmol/L",
            "--effective-at",
            "2026-08-27T15:20:00Z",
        ],
    );
    assert_eq!(refused_chart.status.code(), Some(2));
    let refused_chart_json: Value =
        serde_json::from_slice(&refused_chart.stdout).expect("structured refusal JSON");
    assert_eq!(refused_chart_json["status"], "refused");
    assert_eq!(refused_chart_json["reason_code"], "unknown_patient");

    let refused_visit = run_cli(
        &base_url,
        &token,
        &[
            "send-visit",
            "--patient",
            "mrn:00090001",
            "--appointment-id",
            "appt-bad-window",
            "--status",
            "booked",
            "--starts-at",
            "2026-08-27T21:00:00Z",
            "--ends-at",
            "2026-08-27T20:00:00Z",
            "--label",
            "Invalid Window",
        ],
    );
    assert_eq!(refused_visit.status.code(), Some(2));
    let refused_visit_json: Value =
        serde_json::from_slice(&refused_visit.stdout).expect("structured refusal JSON");
    assert_eq!(refused_visit_json["status"], "refused");
    assert_eq!(refused_visit_json["reason_code"], "invalid_appointment");

    let status_after_refusal: PracticeSyntheticStatus = run_cli_ok(&base_url, &token, &["status"]);
    assert_eq!(status_before_refusal.counts, status_after_refusal.counts);
    assert_eq!(
        status_before_refusal.accepted_events.len(),
        status_after_refusal.accepted_events.len()
    );

    // =========================================================================
    // AC5: Seed a physician conversation and durable patient memory, verify
    // non-zero counts across conversations, reviews, concerns, memories, and
    // briefings, then run `reset` and verify clean base-chart readback!
    // =========================================================================
    {
        let direct_store = Store::connect(&database.url)
            .await
            .expect("connect direct store for conversation/memory seed");
        scale::answered_chat(
            &direct_store,
            &alice_ws,
            "chat-syn-before-reset",
            &[(
                "Review potassium trend before calling Alice.",
                "Reviewed K+ 6.1 mmol/L; recommend holding lisinopril pending repeat BMP.",
            )],
        )
        .await;
        let physician_id = "staging-physician".to_owned();

        direct_store
            .record_patient_memory(
                "cmd-mem-before-reset",
                &RecordMemoryCorrection {
                    practice: STAGING_PRACTICE_ID.to_owned(),
                    workspace: alice_ws.clone(),
                    id: PatientMemoryId::new("mem-dietary-k").unwrap(),
                    topic: "Potassium supplement history".into(),
                    statement: "Patient stopped over-the-counter salt substitute last month."
                        .into(),
                    provenance: MemoryProvenance::PhysicianCorrection {
                        physician: physician_id,
                        chat_id: "chat-syn-before-reset".into(),
                        message_id: "msg-chat-before-reset".into(),
                        work_id: "work-mem-seed".into(),
                        attempt_id: "attempt-mem-seed".into(),
                    },
                },
            )
            .await
            .expect("seed durable patient memory before reset");
    }

    let dirty_status: PracticeSyntheticStatus = run_cli_ok(&base_url, &token, &["status"]);
    assert_eq!(dirty_status.counts.conversations, 1);
    assert_eq!(dirty_status.counts.reviews, 1);
    assert_eq!(dirty_status.counts.open_concerns, 1);
    assert_eq!(dirty_status.counts.memories, 1);
    assert_eq!(dirty_status.counts.understandings, 1);
    assert_eq!(dirty_status.counts.briefings, 1);

    let reset_status: PracticeSyntheticStatus = run_cli_ok(&base_url, &token, &["reset"]);
    assert_eq!(reset_status.practice_id, STAGING_PRACTICE_ID);
    assert_eq!(reset_status.counts.patients, 2);
    assert_eq!(reset_status.counts.conversations, 0);
    assert_eq!(reset_status.counts.reviews, 0);
    assert_eq!(reset_status.counts.open_concerns, 0);
    assert_eq!(reset_status.counts.memories, 0);
    assert_eq!(reset_status.counts.understandings, 0);
    assert_eq!(reset_status.counts.visit_preparations, 0);
    assert_eq!(reset_status.counts.briefings, 0);
    assert_eq!(reset_status.counts.appointments, 1);
    assert!(reset_status.accepted_events.is_empty());
    for patient in &reset_status.patients {
        assert_eq!(
            patient.chart_version, 1,
            "reset must restore {} to base chart_version 1",
            patient.display_name
        );
        assert_eq!(
            patient.chart_record_count, 2,
            "reset must restore {} to base 2 records (Patient + baseline DocumentReference)",
            patient.display_name
        );
    }

    set_browser_viewport(&mut browser, 1440, 900);
    let reset_today = navigate_and_wait(&mut browser, &base_url, "Nothing needs you right now.");
    assert!(
        reset_today.contains("3:30 PM")
            && reset_today.contains("Alice Carter")
            && !reset_today.contains("2:30 PM")
            && !reset_today.contains("Critical potassium 6.1 mmol/L"),
        "Today after reset must show clean base 3:30 PM schedule and zero concerns:\n{reset_today}"
    );
    std::thread::sleep(Duration::from_millis(250));
    save_browser_screenshot(&mut browser, &shots_dir, "ac5-reset-today-desktop");

    let reset_patient =
        navigate_and_wait(&mut browser, &alice_url, "Routine Hypertension Follow-up");
    assert!(
        reset_patient.contains("3:30 PM")
            && !reset_patient.contains("Briefing ready")
            && !reset_patient.contains("Patient summary")
            && !reset_patient.contains("Critical potassium 6.1 mmol/L"),
        "Patient page after reset must show base schedule without briefing, summary, or concern:\n{reset_patient}"
    );

    // =========================================================================
    // AC6 (call-path inspection): Confirm recorded harness provider requests
    // executed the real PatientReview & VisitPreparation recipes and tools!
    // =========================================================================
    let recorded = responses_state.recorded_requests();
    assert!(
        recorded.len() >= 9,
        "expected multi-round harness provider calls for review + selection + briefing, got {}",
        recorded.len()
    );
    let saw_real_review_recipe = recorded.iter().any(|req| {
        let tools: Vec<&str> = req["tools"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|t| t.get("name").and_then(Value::as_str))
            .collect();
        tools.contains(&"search_patient_information")
            && tools.contains(&"read_patient_sources")
            && tools.contains(&"patient_concern")
            && tools.contains(&"save_patient_understanding")
            && tools.contains(&"complete_patient_review")
    });
    assert!(
        saw_real_review_recipe,
        "rounds must execute through the real PracticeJobHost + patient_reviews recipe"
    );

    // =========================================================================
    // AC8: After reset, replay the saved event from AC1 and run rounds again.
    // Verify the exact same path runs and produces matching results!
    // =========================================================================
    let replayed: SavedSyntheticEvent = run_cli_ok(
        &base_url,
        &token,
        &["replay", "--file", saved_event_file.to_str().unwrap()],
    );
    assert_eq!(replayed.target_source, saved_ac1.target_source);
    assert_eq!(
        replayed.resolved_workspace_id,
        saved_ac1.resolved_workspace_id
    );
    assert_eq!(
        replayed.resolved_source_patient_id,
        saved_ac1.resolved_source_patient_id
    );
    assert_eq!(replayed.native_locator, saved_ac1.native_locator);
    assert_eq!(replayed.event, saved_ac1.event);

    let rounds_ac8: RoundsExecutionReport = run_cli_ok(&base_url, &token, &["rounds"]);
    assert_eq!(
        rounds_ac8.executed_reviews.len(),
        rounds_ac1.executed_reviews.len()
    );
    let rev_ac8 = &rounds_ac8.executed_reviews[0];
    assert_eq!(rev_ac8.patient_workspace_id, rev_ac1.patient_workspace_id);
    assert_eq!(rev_ac8.trigger_reason, rev_ac1.trigger_reason);
    assert_eq!(rev_ac8.status, rev_ac1.status);
    assert_eq!(
        rev_ac8.completed_source_version,
        rev_ac1.completed_source_version
    );
    assert_eq!(rev_ac8.action_count, rev_ac1.action_count);
    assert_eq!(rounds_ac8.open_concerns.len(), 1);
    assert_eq!(
        rounds_ac8.open_concerns[0].id,
        rounds_ac1.open_concerns[0].id
    );
    assert_eq!(
        rounds_ac8.open_concerns[0].reason,
        rounds_ac1.open_concerns[0].reason
    );

    let replay_today = navigate_and_wait(
        &mut browser,
        &base_url,
        "Critical potassium 6.1 mmol/L on basic metabolic panel",
    );
    assert!(replay_today.contains("Alice Carter"), "{replay_today}");
    std::thread::sleep(Duration::from_millis(250));
    save_browser_screenshot(&mut browser, &shots_dir, "ac8-replay-today-desktop");

    let page_errors = browser.take_page_errors();
    assert!(
        page_errors.is_empty(),
        "Chromium reported uncaught runtime/console errors: {page_errors:?}"
    );

    drop(browser);
    let _ = stop_tx.send(());
    let _ = server_thread.join();
    stub_handle.abort();
    drop(database);
    // SAFETY: `test_lock()` is held.
    unsafe {
        env::remove_var("PHYSICIAN_AUTH");
        env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
        if ephemeral_pg.is_some() {
            env::remove_var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL");
        }
    }
    drop(ephemeral_pg);
    let _ = fs::remove_dir_all(&base_bank);
    let _ = fs::remove_dir_all(&work_bank);
    let _ = fs::remove_file(&saved_event_file);
}
