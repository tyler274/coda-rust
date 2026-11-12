//! Real Postgres proof for the physician chat recipe:
//! 1. Synthetic-patient chat assignment starts small on `ModelRoles.chat`, places
//!    `PatientStartingContext` into the initial request, and discovers skills and
//!    tools on demand.
//! 2. Missing, unreadable, and unavailable evidence produce distinct, named
//!    `RecordLimit` values, and the saved answer cites readable evidence without
//!    overclaiming.
//! 3. Read, search, refresh, memory, concern, and clinical library calls reach
//!    their backing operations; failed calls never read as saved; and physicians
//!    without workspace panel access cannot start or run chat work.
//! 4. A later conversation uses a durable physician memory correction from an
//!    earlier conversation without overwriting unrelated memory, and retrieves
//!    earlier conversations and background work.
//! 5. End-to-end synthetic patient question saves a complete cited answer whose
//!    numbered citations reopen with readable source details across a fresh pool.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use local_product::{
    ModelBinding, ModelRoles,
    clinical_library::{
        ClinicalLibraryClient, READ_LIBRARY_TOOL, SEARCH_LIBRARY_TOOL, SearchWireResponse,
        WireCandidate, WireEvidenceUnit, WireIdentifier, WireLifecycleState,
        WireReadUnavailableCause, WireSearchPage,
    },
    physician_chat::{ChatAssignment, INSTRUCTIONS, PhysicianSkills, SKILL_VIEW},
    physician_sources::{
        MedicalRecordNumber, PatientSource, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL,
        SourcePatient, SourceRevision, workspace_id,
    },
    runtime::{
        Accepted, Authority, Input, PhysicianHost, PublicationTransaction, Scope, Store,
        WorkStatus, async_trait,
        chat_operations::{
            ChatStartError, DispatchCheckStanding, MemoryMutation, MemoryProvenance,
            MemorySelection, MemoryStanding, PATIENT_MEMORY_TOOL, PatientMemoryEntry,
            PatientMemoryError, PatientMemoryId, PatientMemoryOperation, PatientMemoryStanding,
            PostgresPatientMemoryOperation, PriorWorkKind, REFRESH_SOURCES_TOOL,
            RETRIEVE_PATIENT_WORK_TOOL, RecordLimit, RecordLimitKind, RecordMemoryCorrection,
            RetirePatientMemory, SourceAdapterRefreshOperation,
        },
        concerns::{CONCERN_TOOL, ConcernStanding, ConcernUrgency, WorkspaceAccess},
        dispatch::{PatientSourceChecker, SourceCheckError},
        engine::{
            self, Event, EventKind, Invocation, Item, Product, Turn,
            recipe::{Observer, Provider, Request},
        },
        history::{HistoryOutcome, HistorySegment, TurnAuthor},
        jobs::{Assignment, PracticeJobHost, PracticeJobKind, PracticeScope},
        sources::SourcePull,
    },
    sha256_hex,
};
use patient_work::CheckedSources;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use sea_orm::{ConnectionTrait, Database};
use serde_json::{Value, json};

const LIBRARY_UNIT_ID: &str = "11111111-1111-4111-8111-111111111254";

struct PostgresWorkspaceAuthority;

#[async_trait]
impl Authority for PostgresWorkspaceAuthority {
    async fn check(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::check_runtime_authority(
            tx,
            session_hash.ok_or("missing session reference")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|cause| cause.to_string())
    }

    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::lock_runtime_authority(
            tx,
            session_hash.ok_or("missing session reference")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|cause| cause.to_string())
    }
}

fn unix_seconds() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn test_database_url() -> String {
    std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .expect("explicit local Postgres URL required in PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
}

async fn setup_stores() -> (Store, PostgresAuthStore, String) {
    let url = test_database_url();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    (store, auth, url)
}

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice_id: &str,
    label: &str,
) -> (PhysicianIdentity, String, Vec<u8>) {
    let email = format!("{label}-{}@codahealth.test", uuid::Uuid::new_v4().simple());
    let now = unix_seconds();
    let invite = auth
        .invite(practice_id, &email, label, false, now)
        .await
        .unwrap();
    let (identity, token) = auth.redeem_invitation(&email, &invite, now).await.unwrap();
    let hash = PostgresAuthStore::session_reference(&token);
    (identity, token, hash)
}

fn distinct_model_roles() -> ModelRoles {
    ModelRoles {
        chat: ModelBinding {
            model: "gpt-5.4-chat-lane".into(),
            effort: "medium".into(),
            context_window_tokens: 192_000,
            max_input_tokens: 160_000,
            adapter: "responses".into(),
        },
        background: ModelBinding {
            model: "gpt-5.4-pro-background-lane".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "responses".into(),
        },
    }
}

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::configured(None, false).expect("workspace skills load")
}

fn synthetic_patient(id: &str, name: &str, dob: &str, mrn: &str) -> SourcePatient {
    SourcePatient {
        source_patient_id: id.to_owned(),
        display_name: name.to_owned(),
        birth_date: Some(dob.to_owned()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "synthetic-clinic-ehr".into(),
            value: mrn.to_owned(),
        }],
    }
}

fn readable_chart_record(
    workspace: &str,
    revision: &str,
    date: &str,
    title: &str,
    summary_text: &str,
    payload: Value,
) -> SourceRevision {
    let source_id = "synthetic-clinic-ehr";
    let candidate_id = payload["id"]
        .as_str()
        .expect("chart fixture payload must include string id");
    let record_type = payload["resourceType"]
        .as_str()
        .expect("chart fixture payload must include string resourceType");
    let content = serde_json::to_string(&payload).unwrap();
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: record_type.to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDate".into(),
            value: date.to_owned(),
        }],
        sha256: sha256_hex(content.as_bytes()),
        content,
        readable_parts: vec![ReadablePart {
            label: title.to_owned(),
            text: summary_text.to_owned(),
        }],
        provenance: json!({
            "system": source_id,
            "id": candidate_id,
            "title": title,
        }),
    }
}

fn materialized_outputs(request: &Request) -> BTreeMap<String, String> {
    request
        .items
        .iter()
        .filter_map(|item| match item {
            Item::MaterializedResult { call_id, output } => Some((call_id.clone(), output.clone())),
            _ => None,
        })
        .collect()
}

/// Controllable synthetic `PatientSource` adapter for testing live refresh and
/// upstream source failures without advancing `pulled_at`.
struct ControllablePatientSource {
    source_id: String,
    source_label: String,
    patient: SourcePatient,
    state: Mutex<Result<Vec<SourceRevision>, String>>,
}

#[async_trait]
impl PatientSource for ControllablePatientSource {
    fn source_id(&self) -> &str {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        &self.source_label
    }

    async fn patient_ids(&self) -> Result<Vec<String>, String> {
        Ok(vec![self.patient.source_patient_id.clone()])
    }

    async fn patient(&self, patient_id: &str) -> Result<SourcePatient, String> {
        if patient_id == self.patient.source_patient_id {
            Ok(self.patient.clone())
        } else {
            Err(format!("unknown synthetic patient {patient_id}"))
        }
    }

    async fn records(
        &self,
        _patient: &SourcePatient,
        _workspace: &str,
    ) -> Result<Vec<SourceRevision>, String> {
        self.state.lock().unwrap().clone()
    }
}

/// Failing memory operation proving a failed `patient_memory` call never reads as saved.
struct UnavailableMemoryOperation;

#[async_trait]
impl PatientMemoryOperation for UnavailableMemoryOperation {
    async fn list_memories(
        &self,
        _store: &Store,
        _practice: &str,
        _workspace: &str,
        _selection: MemorySelection,
    ) -> Result<Vec<PatientMemoryEntry>, PatientMemoryError> {
        Err(PatientMemoryError::Unavailable {
            reason: "synthetic memory store outage".into(),
        })
    }

    async fn record_correction(
        &self,
        _store: &Store,
        _command_key: &str,
        _input: &RecordMemoryCorrection,
    ) -> Result<MemoryMutation, PatientMemoryError> {
        Err(PatientMemoryError::Unavailable {
            reason: "synthetic memory store write outage".into(),
        })
    }

    async fn retire_memory(
        &self,
        _store: &Store,
        _command_key: &str,
        _input: &RetirePatientMemory,
    ) -> Result<MemoryMutation, PatientMemoryError> {
        Err(PatientMemoryError::Unavailable {
            reason: "synthetic memory store retirement outage".into(),
        })
    }
}

struct FailingSourceChecker;

#[async_trait]
impl PatientSourceChecker for FailingSourceChecker {
    async fn check_patient_sources(
        &self,
        _practice_id: &str,
        _workspace_id: &str,
        _now: i64,
    ) -> Result<CheckedSources, SourceCheckError> {
        Err(SourceCheckError::Upstream(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "external hospital source timed out",
        )))
    }
}

async fn spawn_healthy_clinical_library() -> String {
    let content = "For adults with acute serum potassium >= 5.8 mmol/L, repeat a non-hemolyzed potassium measurement promptly, obtain a 12-lead ECG, and hold potassium-retaining agents.";
    let unit = WireEvidenceUnit {
        bibliographic_observations: None,
        unit_id: LIBRARY_UNIT_ID.into(),
        version_id: "11111111-1111-4111-8111-000000000254".into(),
        source_code: "dailymed".into(),
        source_display_name: "DailyMed".into(),
        native_document_id: "hyperkalemia-guidance-2026".into(),
        native_version_id: Some("v1".into()),
        source_version_label: None,
        title: "Management of Hyperkalemia in Renin-Angiotensin Blockade".into(),
        document_type: "clinical_guidance".into(),
        publication_date: Some("2026-03-15".into()),
        version_date: Some("2026-03-15".into()),
        acquired_at: "2026-11-10T04:00:00Z".into(),
        lifecycle_state: WireLifecycleState::Current,
        source_status_code: None,
        native_unit_id: "hk-sec-1".into(),
        native_locator: "Section 1".into(),
        unit_kind: "section".into(),
        ordinal: 0,
        label: Some("Acute Hyperkalemia Monitoring and Medication Hold".into()),
        content: content.into(),
        content_sha256: sha256_hex(content.as_bytes()),
        native_destination_uri: "https://clinical.example.org/hyperkalemia-guidance-2026".into(),
        version_destination_uri: "https://clinical.example.org/hyperkalemia-guidance-2026/v1"
            .into(),
        rights_decision_id: "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".into(),
        license_identifier: Some("public-domain".into()),
        attribution_text: Some("DailyMed Clinical Monograph 2026; public domain".into()),
        may_display_excerpt: true,
        identifiers: vec![WireIdentifier {
            scheme: "native".into(),
            raw_value: "hyperkalemia-guidance-2026".into(),
            normalized_value: Some("hyperkalemia-guidance-2026".into()),
        }],
        relationships: vec![],
    };

    #[derive(Clone)]
    struct AppState {
        unit: WireEvidenceUnit,
    }

    async fn search(State(state): State<AppState>) -> impl IntoResponse {
        let candidate = WireCandidate {
            candidate_id: state.unit.unit_id.clone(),
            source_code: state.unit.source_code.clone(),
            source_display_name: state.unit.source_display_name.clone(),
            native_document_id: state.unit.native_document_id.clone(),
            title: state.unit.title.clone(),
        };
        (
            StatusCode::OK,
            Json(SearchWireResponse {
                candidates: vec![candidate],
                page: WireSearchPage {
                    examined_candidate_count: 1,
                    returned_candidate_count: 1,
                    unavailable_candidate_count: 0,
                    truncated: false,
                },
                next_cursor: None,
            }),
        )
    }

    async fn read(State(state): State<AppState>, Json(body): Json<Value>) -> impl IntoResponse {
        let ids = body
            .get("candidate_ids")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let mut results = Vec::with_capacity(ids.len());
        for raw_id in ids {
            let id = raw_id.as_str().unwrap_or_default();
            if id == state.unit.unit_id {
                results.push(json!({
                    "candidate_id": id,
                    "outcome": "available",
                    "unit": state.unit,
                }));
            } else {
                results.push(json!({
                    "candidate_id": id,
                    "outcome": "unavailable",
                    "cause": WireReadUnavailableCause::NotFoundOrNotAuthorized,
                }));
            }
        }
        (StatusCode::OK, Json(json!({ "results": results })))
    }

    let app = Router::new()
        .route("/v1/search", post(search))
        .route("/v1/read", post(read))
        .with_state(AppState { unit });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    format!("http://{addr}")
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn synthetic_patient_question_starts_chat_recipe_small_and_discovers_skills_and_tools() {
    let (store, auth, _url) = setup_stores().await;
    let practice = format!("practice-recipe-{}", uuid::Uuid::new_v4().simple());
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-rivera").await;
    let patient = synthetic_patient("syn-elena-1", "Elena Morales", "1961-04-19", "MRN-10482");
    let workspace = workspace_id("synthetic-clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let bmp = readable_chart_record(
        &workspace,
        "rev-1",
        "2026-11-04",
        "Basic Metabolic Panel",
        "Potassium 4.6 mmol/L, Creatinine 1.18 mg/dL, eGFR 52 mL/min/1.73 m2 on 2026-11-04.",
        json!({
            "resourceType": "Observation",
            "id": "lab-bmp-2026-11",
            "potassium_mmol_L": 4.6,
            "creatinine_mg_dL": 1.18,
            "egfr_mL_min": 52
        }),
    );
    let med = readable_chart_record(
        &workspace,
        "rev-1",
        "2026-10-15",
        "Lisinopril 20 mg tablet",
        "Lisinopril 20 mg orally once daily (active).",
        json!({
            "resourceType": "MedicationRequest",
            "id": "med-lisinopril-2026-10",
            "medication": "Lisinopril 20 mg oral tablet",
            "status": "active"
        }),
    );
    store
        .record_chart(
            &workspace,
            &[bmp, med],
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: 1_762_900_100,
            },
        )
        .await
        .unwrap();

    let scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-synthetic-ac1".into(),
    };
    let skills = load_skills();
    let roles = distinct_model_roles();
    let starting_context = store.patient_starting_context(&scope).await.unwrap();
    assert_eq!(starting_context.workspace, workspace);
    assert_eq!(
        starting_context
            .patient
            .as_ref()
            .map(|p| p.display_name.as_str()),
        Some("Elena Morales")
    );
    assert_eq!(starting_context.chart_version, 1);
    assert_eq!(starting_context.chart_record_count, 2);

    let assignment = ChatAssignment::for_patient(scope.clone(), starting_context);
    assert!(
        assignment.preloaded_skills.is_empty(),
        "chat starts small without eagerly preloading skill bodies"
    );
    let recipe = assignment.recipe(&roles, &skills);
    assert_eq!(recipe.model, "gpt-5.4-chat-lane");
    assert_eq!(recipe.context_window_tokens, 192_000);
    assert_eq!(recipe.max_input_tokens, 160_000);
    assert_eq!(recipe.options, json!({ "effort": "medium" }));
    assert!(recipe.instructions.starts_with(INSTRUCTIONS));
    assert!(
        recipe
            .instructions
            .contains("\"kind\":\"approved_skill_catalog\"")
    );
    assert!(!recipe.instructions.contains("preloaded_assignment_skills"));
    assert!(
        !recipe.instructions.contains("Retrieval Discipline"),
        "skill body must not be eagerly embedded in the starting prompt"
    );

    let offered_names: Vec<&str> = recipe
        .tools
        .iter()
        .filter_map(|tool| tool["name"].as_str())
        .collect();
    for required in [
        SKILL_VIEW,
        "read_tool_result",
        "save_working_note",
        SEARCH_TOOL,
        READ_TOOL,
        REFRESH_SOURCES_TOOL,
        PATIENT_MEMORY_TOOL,
        RETRIEVE_PATIENT_WORK_TOOL,
        CONCERN_TOOL,
        SEARCH_LIBRARY_TOOL,
        READ_LIBRARY_TOOL,
    ] {
        assert!(
            offered_names.contains(&required),
            "expected authorized chat tool {required} in {offered_names:?}"
        );
    }
    for forbidden in [
        "write_patient_memory",
        "send_patient_message",
        "finish_reply_revision",
    ] {
        assert!(
            !offered_names.contains(&forbidden),
            "unexpected tool {forbidden} in {offered_names:?}"
        );
    }

    let question = Input {
        key: "cmd-ac1-1".into(),
        text: "How are Elena's renal function and potassium on lisinopril, and how should we frame a portal update if she asks?".into(),
        timezone: "America/Los_Angeles".into(),
        scenario: None,
    };
    let accepted = store
        .start_chat_authorized(
            &scope,
            &question,
            session_hash,
            &skills,
            &recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(accepted, Accepted::New);

    struct InspectingProvider {
        requests: Vec<Request>,
    }

    impl Provider for InspectingProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.requests.push(request.clone());
            match self.requests.len() {
                1 => {
                    let Item::Message { role, text } = &request.items[0] else {
                        panic!(
                            "expected first request item to be developer patient starting context, got {:?}",
                            request.items.first()
                        );
                    };
                    assert_eq!(role, "developer");
                    assert!(text.contains("Elena Morales"), "{text}");
                    assert!(text.contains("MRN-10482"), "{text}");
                    assert!(text.contains("Synthetic Clinic EHR"), "{text}");
                    assert!(request.items.iter().any(|item| matches!(
                        item,
                        Item::Message { role, text }
                            if role == "user"
                                && text.contains("How are Elena's renal function and potassium on lisinopril, and how should we frame a portal update if she asks?")
                    )));
                    Ok(Turn {
                        items: vec![
                            Item::Call(Invocation {
                                id: "load-retrieval-skill".into(),
                                name: SKILL_VIEW.into(),
                                arguments: json!({
                                    "name": "patient-record-retrieval",
                                    "file_path": null
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "discover-chart".into(),
                                name: SEARCH_TOOL.into(),
                                arguments: json!({
                                    "query": "",
                                    "record_type": null,
                                    "after": null
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({"turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let outputs = materialized_outputs(request);
                    let skill_json: Value =
                        serde_json::from_str(outputs.get("load-retrieval-skill").unwrap()).unwrap();
                    assert_eq!(skill_json["status"], "available");
                    assert_eq!(skill_json["name"], "patient-record-retrieval");
                    assert!(
                        skill_json["content"]
                            .as_str()
                            .unwrap()
                            .contains("search_patient_information")
                    );

                    let search_json: Value =
                        serde_json::from_str(outputs.get("discover-chart").unwrap()).unwrap();
                    assert_eq!(search_json["workspace_version"], 1);
                    assert_eq!(search_json["results"].as_array().unwrap().len(), 2);

                    Ok(Turn {
                        items: vec![
                            Item::Call(Invocation {
                                id: "load-comm-skill".into(),
                                name: SKILL_VIEW.into(),
                                arguments: json!({
                                    "name": "patient-communication",
                                    "file_path": null
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "read-bmp-and-med".into(),
                                name: READ_TOOL.into(),
                                arguments: json!({
                                    "references": [
                                        {
                                            "candidate_id": "lab-bmp-2026-11",
                                            "revision": "rev-1",
                                            "cursor": null
                                        },
                                        {
                                            "candidate_id": "med-lisinopril-2026-10",
                                            "revision": "rev-1",
                                            "cursor": null
                                        }
                                    ],
                                    "max_result_bytes": 16384
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let outputs = materialized_outputs(request);
                    let comm_json: Value =
                        serde_json::from_str(outputs.get("load-comm-skill").unwrap()).unwrap();
                    assert_eq!(comm_json["status"], "available");
                    assert_eq!(comm_json["name"], "patient-communication");
                    let read_json: Value =
                        serde_json::from_str(outputs.get("read-bmp-and-med").unwrap()).unwrap();
                    assert_eq!(read_json["results"].as_array().unwrap().len(), 2);

                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's potassium is 4.6 mmol/L and creatinine is 1.18 mg/dL (eGFR 52 mL/min/1.73 m2) on November 4, 2026 [1] while taking lisinopril 20 mg daily [2]. Continuing lisinopril 20 mg daily is appropriate; if messaging her, reassure her that her kidney numbers and potassium remain stable without implying any prescription change.\n\nBasis and limitations: Synthetic Clinic EHR Basic Metabolic Panel (2026-11-04) and active lisinopril 20 mg order (2026-10-15), plus patient-record-retrieval and patient-communication guidance. No home blood pressure readings are in the chart.",
                        )],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: PostgresWorkspaceAuthority,
    };
    let mut provider = InspectingProvider {
        requests: Vec::new(),
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    assert_eq!(provider.requests.len(), 3);
    assert_eq!(provider.requests[0].model, "gpt-5.4-chat-lane");
    let saved = store.conversation(&scope).await.unwrap();
    assert_eq!(saved.status, WorkStatus::Completed);
    assert_eq!(saved.messages.len(), 2);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn missing_unreadable_and_unavailable_evidence_produce_distinct_named_limits_without_overclaiming()
 {
    let (base_store, auth, url) = setup_stores().await;
    let practice = format!("practice-limits-{}", uuid::Uuid::new_v4().simple());
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-patel").await;
    let patient = synthetic_patient("syn-marcus-2", "Marcus Chen", "1954-09-12", "MRN-20911");
    let workspace = workspace_id("synthetic-clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    // 1. Readable lab result.
    let readable_a1c = readable_chart_record(
        &workspace,
        "rev-1",
        "2026-11-02",
        "Hemoglobin A1c",
        "Hemoglobin A1c 7.4% on 2026-11-02.",
        json!({
            "resourceType": "Observation",
            "id": "lab-a1c-2026-11",
            "a1c_percent": 7.4
        }),
    );

    // 2. Unreadable scanned PDF DocumentReference with binary data and no extracted text.
    let scanned_pdf_payload = json!({
        "resourceType": "DocumentReference",
        "id": "doc-outside-echo-scan",
        "status": "current",
        "content": [{
            "attachment": {
                "contentType": "application/pdf",
                "data": "JVBERi0xLjQKJcfsj6IKNSAwIG9iago8PC9MZW5ndGggNiAwIFIvRmlsdGVyIC9GbGF0ZURlY29kZT4+CnN0cmVhbQp4nDPQM1Qo5ypUMFAwALJMLUAAAwhYBHUKZW5kc3RyZWFtCmVuZG9iago="
            }
        }]
    })
    .to_string();
    let unreadable_scan = SourceRevision {
        workspace: workspace.clone(),
        source_id: "synthetic-clinic-ehr".into(),
        candidate_id: "doc-outside-echo-scan".into(),
        revision: "rev-1".into(),
        record_type: "DocumentReference".into(),
        dates: vec![RecordDate {
            name: "indexed".into(),
            value: "2026-10-28".into(),
        }],
        sha256: sha256_hex(scanned_pdf_payload.as_bytes()),
        content: scanned_pdf_payload,
        readable_parts: vec![],
        provenance: json!({
            "system": "synthetic-clinic-ehr",
            "id": "doc-outside-echo-scan",
            "title": "Outside Transthoracic Echocardiogram Scan"
        }),
    };

    // 3. Unreadable fax whose source reported explicit OCR failure in provenance.
    let fax_payload = json!({
        "resourceType": "DocumentReference",
        "id": "doc-corrupt-cardiology-fax",
        "status": "current"
    })
    .to_string();
    let unreadable_fax = SourceRevision {
        workspace: workspace.clone(),
        source_id: "synthetic-clinic-ehr".into(),
        candidate_id: "doc-corrupt-cardiology-fax".into(),
        revision: "rev-1".into(),
        record_type: "DocumentReference".into(),
        dates: vec![RecordDate {
            name: "indexed".into(),
            value: "2026-10-30".into(),
        }],
        sha256: sha256_hex(fax_payload.as_bytes()),
        content: fax_payload,
        readable_parts: vec![],
        provenance: json!({
            "system": "synthetic-clinic-ehr",
            "id": "doc-corrupt-cardiology-fax",
            "unreadable_reason": "OCR failed on skewed fax pages 2-4"
        }),
    };

    base_store
        .record_chart(
            &workspace,
            &[readable_a1c, unreadable_scan, unreadable_fax],
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: 1_762_900_200,
            },
        )
        .await
        .unwrap();

    // Record a failed background source check so dispatch standing reflects an unavailable newer pull.
    let wake = base_store
        .check_due_patients(&auth, &practice, 1_762_905_000, &FailingSourceChecker)
        .await
        .unwrap();
    assert_eq!(wake.failed_checks.len(), 1);

    let failing_hospital_adapter = Arc::new(ControllablePatientSource {
        source_id: "regional-hospital-adt".into(),
        source_label: "Regional Hospital ADT".into(),
        patient: patient.clone(),
        state: Mutex::new(Err("TLS upstream gateway 503".into())),
    });
    let unavailable_library = ClinicalLibraryClient::http("http://127.0.0.1:9")
        .unwrap()
        .with_bearer_token("unused");
    let store = base_store
        .with_refresh_operation(Arc::new(SourceAdapterRefreshOperation::new(vec![
            failing_hospital_adapter,
        ])))
        .with_clinical_library(unavailable_library);

    let scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-limits-ac2".into(),
    };
    let starting_context = store.patient_starting_context(&scope).await.unwrap();
    assert!(matches!(
        starting_context.check_standing,
        DispatchCheckStanding::FailedSinceLastSuccess {
            last_attempt_at: 1_762_905_000,
            last_success_at: None
        }
    ));

    let skills = load_skills();
    let roles = distinct_model_roles();
    let recipe =
        ChatAssignment::for_patient(scope.clone(), starting_context).recipe(&roles, &skills);
    let question = Input {
        key: "cmd-ac2-1".into(),
        text: "Summarize Marcus's A1c, lipid panel, outside echo scan, cardiology fax, and hospital discharge status.".into(),
        timezone: "America/Los_Angeles".into(),
        scenario: None,
    };
    store
        .start_chat_authorized(
            &scope,
            &question,
            session_hash,
            &skills,
            &recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    struct LimitsProvider {
        turn: usize,
        observed_limits: Vec<RecordLimit>,
    }

    impl Provider for LimitsProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "read-records".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [
                                    {"candidate_id": "lab-a1c-2026-11", "revision": "rev-1", "cursor": null},
                                    {"candidate_id": "lab-missing-lipid-2026", "revision": "rev-1", "cursor": null},
                                    {"candidate_id": "doc-outside-echo-scan", "revision": "rev-1", "cursor": null},
                                    {"candidate_id": "doc-corrupt-cardiology-fax", "revision": "rev-1", "cursor": null}
                                ],
                                "max_result_bytes": 16384
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "refresh-hospital".into(),
                            name: REFRESH_SOURCES_TOOL.into(),
                            arguments: json!({
                                "source_id": "regional-hospital-adt",
                                "reason": "Check for recent hospital discharge summary"
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "check-library".into(),
                            name: SEARCH_LIBRARY_TOOL.into(),
                            arguments: json!({
                                "query": "glycemic targets older adults chronic disease",
                                "collection": null,
                                "limit": 5
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let outputs = materialized_outputs(request);
                    let read_payload: Value =
                        serde_json::from_str(outputs.get("read-records").unwrap()).unwrap();
                    assert_eq!(read_payload["results"][0]["outcome"], "available");
                    assert_eq!(read_payload["results"][0]["reference"], "lab-a1c-2026-11");
                    assert_eq!(read_payload["results"][1]["outcome"], "unavailable");
                    let read_limits: Vec<RecordLimit> =
                        serde_json::from_value(read_payload["limits"].clone()).unwrap();
                    self.observed_limits.extend(read_limits);

                    let refresh_payload: Value =
                        serde_json::from_str(outputs.get("refresh-hospital").unwrap()).unwrap();
                    assert_eq!(refresh_payload["outcome"], "failed");
                    assert_eq!(refresh_payload["saved"], false);
                    let refresh_limits: Vec<RecordLimit> =
                        serde_json::from_value(refresh_payload["limits"].clone()).unwrap();
                    self.observed_limits.extend(refresh_limits);

                    let library_payload: Value =
                        serde_json::from_str(outputs.get("check-library").unwrap()).unwrap();
                    assert_eq!(library_payload["outcome"], "unavailable");
                    let library_limits: Vec<RecordLimit> =
                        serde_json::from_value(library_payload["limits"].clone()).unwrap();
                    self.observed_limits.extend(library_limits);

                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Marcus's Hemoglobin A1c on November 2, 2026 is 7.4% [1]. I cannot confirm his lipid panel because that record is missing from the chart, I cannot interpret the October 28, 2026 outside echocardiogram scan or the October 30, 2026 cardiology fax because both documents are unreadable without extracted clinical text, and recent Regional Hospital ADT discharge status is unavailable because the hospital feed did not respond.\n\nBasis and limitations: Synthetic Clinic EHR Hemoglobin A1c (2026-11-02). Missing: 2026 lipid panel. Unreadable: outside echocardiogram PDF scan (doc-outside-echo-scan) and cardiology fax with failed OCR (doc-corrupt-cardiology-fax). Unavailable: Regional Hospital ADT live refresh and clinical library guideline lookup.",
                        )],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: PostgresWorkspaceAuthority,
    };
    let mut provider = LimitsProvider {
        turn: 0,
        observed_limits: Vec::new(),
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    assert!(provider.observed_limits.iter().any(|limit| {
        limit.kind == RecordLimitKind::Missing && limit.subject == "lab-missing-lipid-2026"
    }));
    assert!(provider.observed_limits.iter().any(|limit| {
        limit.kind == RecordLimitKind::Unreadable && limit.subject == "doc-outside-echo-scan"
    }));
    assert!(provider.observed_limits.iter().any(|limit| {
        limit.kind == RecordLimitKind::Unreadable && limit.subject == "doc-corrupt-cardiology-fax"
    }));
    assert!(provider.observed_limits.iter().any(|limit| {
        limit.kind == RecordLimitKind::Unavailable && limit.subject == "regional-hospital-adt"
    }));
    assert!(provider.observed_limits.iter().any(|limit| {
        limit.kind == RecordLimitKind::Unavailable && limit.subject == "clinical_library"
    }));

    let reopened = store.reopen_conversation(&scope).await.unwrap().unwrap();
    assert_eq!(reopened.outcome, HistoryOutcome::Answered);
    let answer_turn = &reopened.turns[1];
    assert!(answer_turn.raw_text.contains("missing"));
    assert!(answer_turn.raw_text.contains("unreadable"));
    assert!(answer_turn.raw_text.contains("unavailable"));
    assert!(
        !answer_turn
            .raw_text
            .to_lowercase()
            .contains("read everything")
    );
    assert!(answer_turn.basis.is_some());
    assert!(
        reopened
            .sources
            .values()
            .any(|src| src.candidate_id == "lab-a1c-2026-11")
    );

    // Also verify corrupt stored JSON in `physician_runtime_sources` surfaces as `Unreadable`.
    // Typed SeaQuery insert is used here because `Store::record_source` validates `SourceRevision`
    // before writing, so malformed revision JSON cannot enter through the normal typed API.
    let raw_db = Database::connect(&url).await.unwrap();
    let corrupt_workspace = format!("pw-corrupt-{}", uuid::Uuid::new_v4().simple());
    let insert_version = sea_orm::sea_query::Query::insert()
        .into_table(sea_orm::sea_query::Alias::new(
            "physician_runtime_source_versions",
        ))
        .columns([
            sea_orm::sea_query::Alias::new("workspace"),
            sea_orm::sea_query::Alias::new("version"),
        ])
        .values_panic([corrupt_workspace.clone().into(), 1_i64.into()])
        .to_owned();
    raw_db.execute(&insert_version).await.unwrap();
    let insert_corrupt_source = sea_orm::sea_query::Query::insert()
        .into_table(sea_orm::sea_query::Alias::new("physician_runtime_sources"))
        .columns([
            sea_orm::sea_query::Alias::new("workspace"),
            sea_orm::sea_query::Alias::new("candidate"),
            sea_orm::sea_query::Alias::new("observed_at_version"),
            sea_orm::sea_query::Alias::new("revision"),
        ])
        .values_panic([
            corrupt_workspace.clone().into(),
            "corrupt-lab-1".into(),
            1_i64.into(),
            json!({"broken": true}).into(),
        ])
        .to_owned();
    raw_db.execute(&insert_corrupt_source).await.unwrap();
    let corrupt_scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: corrupt_workspace.clone(),
        chat: "chat-corrupt".into(),
    };
    let corrupt_read = store
        .execute_chat_chart_tool(
            &corrupt_scope,
            READ_TOOL,
            &json!({
                "references": [{"candidate_id": "corrupt-lab-1", "revision": "rev-1", "cursor": null}],
                "max_result_bytes": 16384
            }),
        )
        .await
        .unwrap();
    let engine::ToolOutcome::Returned(corrupt_json) = corrupt_read else {
        panic!("expected structured unreadable tool result, got {corrupt_read:?}");
    };
    let parsed_corrupt: Value = serde_json::from_str(&corrupt_json).unwrap();
    assert_eq!(parsed_corrupt["limits"][0]["kind"], "unreadable");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn chat_operations_execute_persist_refuse_failed_saves_and_enforce_workspace_access() {
    let (base_store, auth, _url) = setup_stores().await;
    let practice = format!("practice-ops-{}", uuid::Uuid::new_v4().simple());
    let (on_panel_doc, _token, on_panel_hash) =
        invite_and_sign_in(&auth, &practice, "dr-on-panel").await;
    let (off_panel_doc, _off_token, off_panel_hash) =
        invite_and_sign_in(&auth, &practice, "dr-off-panel").await;
    let (demographic_doc, _demo_token, demographic_hash) =
        invite_and_sign_in(&auth, &practice, "dr-demographic").await;
    let (revoked_doc, revoked_token, revoked_hash) =
        invite_and_sign_in(&auth, &practice, "dr-revoked").await;

    let other_practice = format!("practice-foreign-{}", uuid::Uuid::new_v4().simple());
    let (foreign_doc, _foreign_token, foreign_hash) =
        invite_and_sign_in(&auth, &other_practice, "dr-foreign").await;

    let patient = synthetic_patient("syn-priya-3", "Priya Nair", "1972-02-18", "MRN-33019");
    let workspace = workspace_id("synthetic-clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&on_panel_doc, &workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&off_panel_doc, &workspace, Membership::Denied)
        .await
        .unwrap();
    auth.set_membership(&demographic_doc, &workspace, Membership::ClinicDemographic)
        .await
        .unwrap();
    auth.set_membership(&revoked_doc, &workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.end_session(&revoked_token).await.unwrap();

    // Seed initial chart v1 (potassium 4.4 mmol/L) at pulled_at = 1_762_900_000.
    let initial_k = readable_chart_record(
        &workspace,
        "rev-1",
        "2026-11-01",
        "Serum Potassium",
        "Serum potassium 4.4 mmol/L on 2026-11-01.",
        json!({"resourceType": "Observation", "id": "lab-k-2026-11", "potassium_mmol_L": 4.4}),
    );
    base_store
        .record_chart(
            &workspace,
            std::slice::from_ref(&initial_k),
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: 1_762_900_000,
            },
        )
        .await
        .unwrap();

    // Prepare refreshed chart record v2 (potassium 5.9 mmol/L) in the live source adapter.
    let refreshed_k = readable_chart_record(
        &workspace,
        "rev-2",
        "2026-11-12",
        "Serum Potassium",
        "Critical serum potassium 5.9 mmol/L on 2026-11-12.",
        json!({"resourceType": "Observation", "id": "lab-k-2026-11", "potassium_mmol_L": 5.9}),
    );
    let ehr_adapter = Arc::new(ControllablePatientSource {
        source_id: "synthetic-clinic-ehr".into(),
        source_label: "Synthetic Clinic EHR".into(),
        patient: patient.clone(),
        state: Mutex::new(Ok(vec![refreshed_k])),
    });
    let library_url = spawn_healthy_clinical_library().await;
    let store = base_store
        .with_refresh_operation(Arc::new(SourceAdapterRefreshOperation::new(vec![
            Arc::clone(&ehr_adapter) as Arc<dyn PatientSource>,
        ])))
        .with_clinical_library(ClinicalLibraryClient::http(&library_url).unwrap());

    let skills = load_skills();
    let roles = distinct_model_roles();

    // Access refusals BEFORE any job or conversation row is created.
    for (label, doc_id, doc_practice, hash) in [
        (
            "off-panel physician",
            off_panel_doc.physician_id.clone(),
            practice.clone(),
            off_panel_hash,
        ),
        (
            "clinic-demographic-only physician",
            demographic_doc.physician_id.clone(),
            practice.clone(),
            demographic_hash,
        ),
        (
            "foreign-practice physician",
            foreign_doc.physician_id.clone(),
            other_practice.clone(),
            foreign_hash,
        ),
        (
            "revoked-session physician",
            revoked_doc.physician_id.clone(),
            practice.clone(),
            revoked_hash,
        ),
    ] {
        let denied_scope = Scope {
            practice: doc_practice,
            physician: doc_id,
            workspace: workspace.clone(),
            chat: format!("denied-chat-{}", uuid::Uuid::new_v4().simple()),
        };
        let dummy_recipe =
            local_product::physician_chat::recipe(&roles.chat, INSTRUCTIONS, &skills);
        let attempt = store
            .start_chat_authorized(
                &denied_scope,
                &Input {
                    key: format!("denied-key-{}", uuid::Uuid::new_v4().simple()),
                    text: "Attempt unauthorized question".into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                hash,
                &skills,
                &dummy_recipe,
                &mut PostgresWorkspaceAuthority,
            )
            .await;
        assert!(
            matches!(attempt, Err(ChatStartError::AccessDenied { .. })),
            "{label} must be refused at chat start, got {attempt:?}"
        );
        assert!(
            store.read(&denied_scope).await.unwrap().is_none(),
            "{label} must leave zero conversation rows in Postgres"
        );
    }

    // Authorized physician runs search, refresh, read, memory, concern, and clinical library.
    let scope = Scope {
        practice: practice.clone(),
        physician: on_panel_doc.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-ops-ac3".into(),
    };
    let starting = store.patient_starting_context(&scope).await.unwrap();
    let recipe = ChatAssignment::for_patient(scope.clone(), starting).recipe(&roles, &skills);
    store
        .start_chat_authorized(
            &scope,
            &Input {
                key: "cmd-ac3-ok".into(),
                text: "Refresh Priya's labs, note that she stopped her potassium supplement 3 days ago, and flag any urgent hyperkalemia.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            on_panel_hash.clone(),
            &skills,
            &recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    struct OperationsProvider {
        turn: usize,
    }

    impl Provider for OperationsProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "search-1".into(),
                            name: SEARCH_TOOL.into(),
                            arguments: json!({"query": "potassium", "record_type": null, "after": null})
                                .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "refresh-1".into(),
                            name: REFRESH_SOURCES_TOOL.into(),
                            arguments: json!({
                                "source_id": "synthetic-clinic-ehr",
                                "reason": "Verify today's serum potassium before recommending action"
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "lib-search-1".into(),
                            name: SEARCH_LIBRARY_TOOL.into(),
                            arguments: json!({
                                "query": "acute hyperkalemia renin angiotensin",
                                "collection": null,
                                "limit": 5
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let outputs = materialized_outputs(request);
                    let refresh_out: Value =
                        serde_json::from_str(outputs.get("refresh-1").unwrap()).unwrap();
                    assert_eq!(refresh_out["outcome"], "refreshed");
                    assert_eq!(refresh_out["saved"], true);
                    assert_eq!(refresh_out["workspace_version"], 2);

                    let lib_search_out: Value =
                        serde_json::from_str(outputs.get("lib-search-1").unwrap()).unwrap();
                    assert_eq!(lib_search_out["outcome"], "results");
                    assert_eq!(lib_search_out["passages"][0]["passage_id"], LIBRARY_UNIT_ID);

                    Ok(Turn {
                        items: vec![
                            Item::Call(Invocation {
                                id: "read-k-v2".into(),
                                name: READ_TOOL.into(),
                                arguments: json!({
                                    "references": [{
                                        "candidate_id": "lab-k-2026-11",
                                        "revision": "rev-2",
                                        "cursor": null
                                    }],
                                    "max_result_bytes": 16384
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "lib-read-1".into(),
                                name: READ_LIBRARY_TOOL.into(),
                                arguments: json!({"passage_id": LIBRARY_UNIT_ID}).to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "mem-save-1".into(),
                                name: PATIENT_MEMORY_TOOL.into(),
                                arguments: json!({
                                    "operation": "record_correction",
                                    "memory_id": "mem-stopped-kcl",
                                    "topic": "Potassium supplement",
                                    "statement": "Stopped oral potassium supplement 3 days ago per Dr. On-Panel."
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let outputs = materialized_outputs(request);
                    let mem_out: Value =
                        serde_json::from_str(outputs.get("mem-save-1").unwrap()).unwrap();
                    assert_eq!(mem_out["outcome"], "applied");
                    assert_eq!(mem_out["saved"], true);

                    let lib_read_out: Value =
                        serde_json::from_str(outputs.get("lib-read-1").unwrap()).unwrap();
                    assert_eq!(lib_read_out["outcome"], "passage");
                    assert_eq!(lib_read_out["passage"]["passage_id"], LIBRARY_UNIT_ID);

                    Ok(Turn {
                        items: vec![
                            Item::Call(Invocation {
                                id: "raise-k-concern".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-hyperkalemia",
                                    "urgency": "now",
                                    "reason": "Serum potassium rose to 5.9 mmol/L on 2026-11-12; obtain ECG and repeat non-hemolyzed potassium.",
                                    "records": [{
                                        "candidate_id": "lab-k-2026-11",
                                        "revision": "rev-2"
                                    }]
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "mem-read-1".into(),
                                name: PATIENT_MEMORY_TOOL.into(),
                                arguments: json!({
                                    "operation": "read",
                                    "include_retired": false
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => {
                    let outputs = materialized_outputs(request);
                    let concern_out: Value =
                        serde_json::from_str(outputs.get("raise-k-concern").unwrap()).unwrap();
                    assert_eq!(concern_out["outcome"], "applied");

                    let mem_read: Value =
                        serde_json::from_str(outputs.get("mem-read-1").unwrap()).unwrap();
                    assert_eq!(mem_read["entries"].as_array().unwrap().len(), 1);
                    assert_eq!(mem_read["entries"][0]["id"], "mem-stopped-kcl");

                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Refreshed Synthetic Clinic EHR records show Priya's serum potassium is 5.9 mmol/L on November 12, 2026 [1]. I noted your update that she stopped her potassium supplement 3 days ago, raised an urgent concern, and recommend a prompt 12-lead ECG, repeat non-hemolyzed potassium, and holding potassium-retaining agents.\n\nBasis and limitations: Refreshed Synthetic Clinic EHR Serum Potassium (2026-11-12), DailyMed hyperkalemia clinical monograph (2026-03-15), and physician-reported supplement stop.",
                        )],
                        receipt: json!({"turn": 4}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: PostgresWorkspaceAuthority,
    };
    host.run(
        &mut OperationsProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Verify Postgres readback of all persisted operations.
    let chart_after = store.sources(&workspace).await.unwrap();
    assert_eq!(chart_after.version, 2);
    assert_eq!(chart_after.sources[0].revision, "rev-2");

    let memories_after = store
        .patient_memories(&practice, &workspace, MemorySelection::All)
        .await
        .unwrap();
    assert_eq!(memories_after.len(), 1);
    assert_eq!(memories_after[0].id.as_str(), "mem-stopped-kcl");
    assert_eq!(memories_after[0].standing, MemoryStanding::Active);

    let open_concerns = store
        .open_concerns(
            &WorkspaceAccess {
                practice: &practice,
                physician: &on_panel_doc.physician_id,
                workspace: &workspace,
                session_hash: &on_panel_hash,
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(open_concerns.len(), 1);
    assert_eq!(open_concerns[0].id.as_str(), "concern-hyperkalemia");
    assert_eq!(open_concerns[0].urgency, ConcernUrgency::Now);
    assert_eq!(open_concerns[0].standing, ConcernStanding::Open);

    // Now inject failures across refresh, memory, and unverified concern and prove NOTHING new saves.
    *ehr_adapter.state.lock().unwrap() = Err("EHR endpoint connection reset".into());
    let pulled_before_failure = store
        .source_pulls(std::slice::from_ref(&workspace))
        .await
        .unwrap()[&workspace][0]
        .pulled_at;
    let failing_store = store
        .clone()
        .with_memory_operation(Arc::new(UnavailableMemoryOperation));

    let fail_scope = Scope {
        practice: practice.clone(),
        physician: on_panel_doc.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-ops-failures-ac3".into(),
    };
    let fail_starting = failing_store
        .patient_starting_context(&fail_scope)
        .await
        .unwrap();
    assert_eq!(
        fail_starting.memory_standing,
        PatientMemoryStanding::Unavailable {
            reason: "synthetic memory store outage".into(),
        }
    );

    let fail_recipe =
        ChatAssignment::for_patient(fail_scope.clone(), fail_starting).recipe(&roles, &skills);
    failing_store
        .start_chat_authorized(
            &fail_scope,
            &Input {
                key: "cmd-ac3-fail".into(),
                text:
                    "Try refreshing again, saving a second note, and raising an unverified concern."
                        .into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            on_panel_hash.clone(),
            &skills,
            &fail_recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    struct FailureCheckProvider {
        turn: usize,
    }

    impl Provider for FailureCheckProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => {
                    let Item::Message { role, text } = &request.items[0] else {
                        panic!("expected developer starting context item");
                    };
                    assert_eq!(role, "developer");
                    assert!(
                        text.contains("Memory limit (unavailable):"),
                        "failed memory read must surface as an explicit unavailable limit, got: {text}"
                    );
                    assert!(
                        text.contains("synthetic memory store outage"),
                        "failed memory read must carry its operational reason, got: {text}"
                    );
                    assert!(
                        !text.contains("mem-stopped-kcl"),
                        "unavailable memory must not masquerade as available entries: {text}"
                    );
                    Ok(Turn {
                        items: vec![
                            Item::Call(Invocation {
                                id: "fail-refresh".into(),
                                name: REFRESH_SOURCES_TOOL.into(),
                                arguments: json!({
                                    "source_id": "synthetic-clinic-ehr",
                                    "reason": "Check for repeat potassium"
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "fail-memory".into(),
                                name: PATIENT_MEMORY_TOOL.into(),
                                arguments: json!({
                                    "operation": "record_correction",
                                    "memory_id": "mem-unsaved",
                                    "topic": "Diet",
                                    "statement": "Should not persist during outage."
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "fail-concern".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-unverified",
                                    "urgency": "today",
                                    "reason": "Fabricated citation never read in this turn.",
                                    "records": [{
                                        "candidate_id": "lab-k-2026-11",
                                        "revision": "rev-999"
                                    }]
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({"turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let outputs = materialized_outputs(request);
                    let refresh_fail: Value =
                        serde_json::from_str(outputs.get("fail-refresh").unwrap()).unwrap();
                    assert_eq!(refresh_fail["outcome"], "failed");
                    assert_eq!(refresh_fail["saved"], false);

                    let memory_fail: Value =
                        serde_json::from_str(outputs.get("fail-memory").unwrap()).unwrap();
                    assert_eq!(memory_fail["outcome"], "failed");
                    assert_eq!(memory_fail["saved"], false);

                    let concern_refusal = outputs.get("fail-concern").unwrap();
                    assert!(
                        concern_refusal.contains("not read in this run")
                            || concern_refusal.contains("refused")
                            || concern_refusal.contains("rev-999"),
                        "expected unverified concern refusal, got {concern_refusal}"
                    );

                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Live EHR refresh and patient memory update both failed and were not saved, and no new concern was recorded without verified chart evidence.\n\nBasis and limitations: Unavailable live Synthetic Clinic EHR refresh and patient memory service; retained chart remains at the earlier November 12, 2026 pull.",
                        )],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let fail_claim = failing_store.claim(&fail_scope, 60).await.unwrap().unwrap();
    let mut fail_host = PhysicianHost {
        store: failing_store,
        claim: fail_claim,
        authority: PostgresWorkspaceAuthority,
    };
    fail_host
        .run(
            &mut FailureCheckProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    // Verify Postgres retained state was NOT mutated by any failed call!
    assert_eq!(store.sources(&workspace).await.unwrap().version, 2);
    let pulled_after_failure = store
        .source_pulls(std::slice::from_ref(&workspace))
        .await
        .unwrap()[&workspace][0]
        .pulled_at;
    assert_eq!(pulled_after_failure, pulled_before_failure);
    let memories_still = store
        .patient_memories(&practice, &workspace, MemorySelection::All)
        .await
        .unwrap();
    assert_eq!(memories_still.len(), 1);
    assert_eq!(memories_still[0].id.as_str(), "mem-stopped-kcl");
    let concerns_still = store
        .open_concerns(
            &WorkspaceAccess {
                practice: &practice,
                physician: &on_panel_doc.physician_id,
                workspace: &workspace,
                session_hash: &on_panel_hash,
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(concerns_still.len(), 1);
    assert_eq!(concerns_still[0].id.as_str(), "concern-hyperkalemia");

    // Finally, revoking panel access after acceptance blocks worker execution before any model call.
    let midflight_scope = Scope {
        practice: practice.clone(),
        physician: on_panel_doc.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-revoked-after-accept".into(),
    };
    store
        .start_chat_authorized(
            &midflight_scope,
            &Input {
                key: "cmd-ac3-revoked-midflight".into(),
                text: "Question accepted right before panel removal".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            on_panel_hash,
            &skills,
            &recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    let midflight_claim = store.claim(&midflight_scope, 60).await.unwrap().unwrap();
    auth.set_membership(&on_panel_doc, &workspace, Membership::Denied)
        .await
        .unwrap();

    struct PanicIfDispatched;
    impl Provider for PanicIfDispatched {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            panic!("provider must never be called after panel access is revoked");
        }
    }

    let mut revoked_host = PhysicianHost {
        store: store.clone(),
        claim: midflight_claim,
        authority: PostgresWorkspaceAuthority,
    };
    let run_err = revoked_host
        .run(
            &mut PanicIfDispatched,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect_err("revoked panel access must fail assert_current");
    assert!(matches!(run_err, engine::Failure::Host(_)), "{run_err}");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn later_conversation_uses_earlier_memory_correction_and_retrieves_earlier_work() {
    let (store, auth, _url) = setup_stores().await;
    let practice = format!("practice-seq-{}", uuid::Uuid::new_v4().simple());
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-lin").await;
    let patient = synthetic_patient("syn-elena-4", "Elena Morales", "1961-04-19", "MRN-10482");
    let workspace = workspace_id("synthetic-clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let rx_metformin = readable_chart_record(
        &workspace,
        "rev-1",
        "2026-09-10",
        "Metformin 500 mg oral tablet",
        "Chart prescription: Metformin 500 mg orally twice daily.",
        json!({
            "resourceType": "MedicationRequest",
            "id": "med-metformin-2026-09",
            "dosage": "500 mg twice daily"
        }),
    );
    store
        .record_chart(
            &workspace,
            &[rx_metformin],
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: 1_762_900_300,
            },
        )
        .await
        .unwrap();

    // Seed an unrelated clinical summary memory entry to verify non-destructive updates.
    store
        .record_patient_memory(
            "seed-allergy-summary",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: PatientMemoryId::new("mem-amoxicillin-rash").unwrap(),
                topic: "Beta-lactam allergy".into(),
                statement: "Childhood maculopapular rash to amoxicillin; tolerates cefazolin."
                    .into(),
                provenance: MemoryProvenance::ClinicalSummary {
                    job_id: "job-initial-understanding".into(),
                    work_id: "work-bg-1".into(),
                    attempt_id: "attempt-bg-1".into(),
                    records: Vec::new(),
                },
            },
        )
        .await
        .unwrap();

    // Also run a real completed practice background job so Conversation 2 can retrieve it.
    let skills = load_skills();
    let roles = distinct_model_roles();
    let bg_scope = PracticeScope {
        practice: practice.clone(),
        workspace: workspace.clone(),
        job_id: "job-initial-understanding".into(),
    };
    let bg_assignment = Assignment::synthetic_patient_stub(&workspace, &skills).unwrap();
    store
        .accept_practice_job(&bg_scope, &bg_assignment, &roles, &skills, &auth)
        .await
        .unwrap();
    let bg_claim = store
        .claim_practice_job(&bg_scope, 60, &auth)
        .await
        .unwrap()
        .unwrap();

    struct BackgroundSummaryProvider;
    impl Provider for BackgroundSummaryProvider {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            Ok(Turn {
                items: vec![Item::assistant(
                    "Initial chart understanding: Type 2 diabetes with chart prescription for metformin 500 mg twice daily (2026-09-10).",
                )],
                receipt: json!({"bg": 1}),
                usage: Default::default(),
            })
        }
    }
    let mut bg_host = PracticeJobHost {
        store: store.clone(),
        claim: bg_claim,
        authority: &auth,
    };
    bg_host
        .run(
            &mut BackgroundSummaryProvider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    // Conversation 1 (`chat-1`): Physician supplies dosing correction -> saved in `patient_memory`.
    let chat1_scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-1-metformin-correction".into(),
    };
    let chat1_recipe = ChatAssignment::for_patient(
        chat1_scope.clone(),
        store.patient_starting_context(&chat1_scope).await.unwrap(),
    )
    .recipe(&roles, &skills);
    store
        .start_chat_authorized(
            &chat1_scope,
            &Input {
                key: "cmd-chat1-1".into(),
                text: "Elena dropped her morning metformin because of GI upset and is taking 500 mg once daily with dinner now. Please note that and check her chart prescription.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session_hash.clone(),
            &skills,
            &chat1_recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    struct Chat1Provider {
        turn: usize,
    }
    impl Provider for Chat1Provider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "read-rx".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "med-metformin-2026-09",
                                    "revision": "rev-1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16384
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "record-metformin-correction".into(),
                            name: PATIENT_MEMORY_TOOL.into(),
                            arguments: json!({
                                "operation": "record_correction",
                                "memory_id": "mem-metformin-evening-only",
                                "topic": "Metformin regimen",
                                "statement": "Taking metformin 500 mg once daily with dinner after stopping morning dose for GI upset (reported by physician)."
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"chat": 1, "turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let outputs = materialized_outputs(request);
                    let mem_saved: Value = serde_json::from_str(
                        outputs.get("record-metformin-correction").unwrap(),
                    )
                    .unwrap();
                    assert_eq!(mem_saved["outcome"], "applied");
                    assert_eq!(mem_saved["saved"], true);
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Noted that Elena is taking metformin 500 mg once daily with dinner due to GI upset, whereas the September 10, 2026 chart prescription still lists 500 mg twice daily [1]. Consider switching to extended-release metformin if glycemic control requires uptitration.\n\nBasis and limitations: Synthetic Clinic EHR Metformin prescription (2026-09-10) and physician-reported dosing update saved to patient memory.",
                        )],
                        receipt: json!({"chat": 1, "turn": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected chat1 turn {other}"),
            }
        }
    }

    let chat1_claim = store.claim(&chat1_scope, 60).await.unwrap().unwrap();
    let mut chat1_host = PhysicianHost {
        store: store.clone(),
        claim: chat1_claim,
        authority: PostgresWorkspaceAuthority,
    };
    chat1_host
        .run(
            &mut Chat1Provider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    // Verify saved memory rows between Conversation 1 and Conversation 2.
    let saved_memories = store
        .patient_memories(&practice, &workspace, MemorySelection::ActiveOnly)
        .await
        .unwrap();
    assert_eq!(saved_memories.len(), 2);
    let correction = saved_memories
        .iter()
        .find(|entry| entry.id.as_str() == "mem-metformin-evening-only")
        .expect("saved metformin correction");
    assert!(matches!(
        &correction.provenance,
        MemoryProvenance::PhysicianCorrection {
            physician: saved_doc,
            chat_id,
            message_id,
            ..
        } if saved_doc == &physician.physician_id
            && chat_id == "chat-1-metformin-correction"
            && message_id == "cmd-chat1-1"
    ));
    assert!(
        saved_memories
            .iter()
            .any(|entry| entry.id.as_str() == "mem-amoxicillin-rash"),
        "unrelated allergy memory must remain intact"
    );

    // Conversation 2 (`chat-2`): Starts with the saved memory correction in starting context
    // and retrieves both `chat-1` and `job-initial-understanding`.
    let chat2_scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-2-followup".into(),
    };
    let chat2_starting = store.patient_starting_context(&chat2_scope).await.unwrap();
    match &chat2_starting.memory_standing {
        PatientMemoryStanding::Available { entries } => assert_eq!(entries.len(), 2),
        other => panic!("expected available memories, got {other:?}"),
    }
    assert!(chat2_starting.recent_work.iter().any(|w| matches!(
        &w.target,
        PriorWorkKind::Conversation { chat_id } if chat_id == "chat-1-metformin-correction"
    )));
    assert!(chat2_starting.recent_work.iter().any(|w| matches!(
        &w.target,
        PriorWorkKind::BackgroundJob { job_id, job_kind }
            if job_id == "job-initial-understanding"
                && *job_kind == PracticeJobKind::InitialUnderstanding
    )));

    let chat2_recipe =
        ChatAssignment::for_patient(chat2_scope.clone(), chat2_starting).recipe(&roles, &skills);
    store
        .start_chat_authorized(
            &chat2_scope,
            &Input {
                key: "cmd-chat2-1".into(),
                text:
                    "Where did we land on Elena's metformin dose compared with our initial review?"
                        .into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session_hash,
            &skills,
            &chat2_recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    struct Chat2Provider {
        turn: usize,
    }
    impl Provider for Chat2Provider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => {
                    let Item::Message { role, text } = &request.items[0] else {
                        panic!(
                            "expected developer starting context item in chat-2, got {:?}",
                            request.items.first()
                        );
                    };
                    assert_eq!(role, "developer");
                    assert!(
                        text.contains("mem-metformin-evening-only"),
                        "chat-2 starting context must carry the saved memory correction: {text}"
                    );
                    assert!(
                        text.contains("500 mg once daily with dinner"),
                        "chat-2 starting context must include correction text: {text}"
                    );
                    assert!(
                        text.contains("mem-amoxicillin-rash"),
                        "chat-2 starting context must preserve unrelated memory: {text}"
                    );
                    assert!(
                        text.contains("chat-1-metformin-correction"),
                        "chat-2 starting context must reference earlier conversation: {text}"
                    );

                    Ok(Turn {
                        items: vec![
                            Item::Call(Invocation {
                                id: "list-work".into(),
                                name: RETRIEVE_PATIENT_WORK_TOOL.into(),
                                arguments: json!({
                                    "operation": "list",
                                    "limit": 10
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "open-chat-1".into(),
                                name: RETRIEVE_PATIENT_WORK_TOOL.into(),
                                arguments: json!({
                                    "operation": "read_conversation",
                                    "chat_id": "chat-1-metformin-correction"
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "open-bg-job".into(),
                                name: RETRIEVE_PATIENT_WORK_TOOL.into(),
                                arguments: json!({
                                    "operation": "read_background_job",
                                    "job_id": "job-initial-understanding"
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({"chat": 2, "turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let outputs = materialized_outputs(request);
                    let list_out: Value =
                        serde_json::from_str(outputs.get("list-work").unwrap()).unwrap();
                    assert_eq!(list_out["outcome"], "available");
                    assert!(!list_out["past_work"].as_array().unwrap().is_empty());
                    assert_eq!(list_out["background_jobs"].as_array().unwrap().len(), 1);

                    let prev_chat: Value =
                        serde_json::from_str(outputs.get("open-chat-1").unwrap()).unwrap();
                    assert_eq!(prev_chat["outcome"], "available");
                    assert_eq!(prev_chat["chat_id"], "chat-1-metformin-correction");
                    assert!(
                        prev_chat["turns"][1]["text"]
                            .as_str()
                            .unwrap()
                            .contains("extended-release metformin")
                    );

                    let bg_job: Value =
                        serde_json::from_str(outputs.get("open-bg-job").unwrap()).unwrap();
                    assert_eq!(bg_job["outcome"], "available");
                    assert_eq!(bg_job["job_id"], "job-initial-understanding");
                    assert!(
                        bg_job["result"]["output_text"]
                            .as_str()
                            .unwrap()
                            .contains("Initial chart understanding")
                    );

                    Ok(Turn {
                        items: vec![Item::assistant(
                            "In our earlier conversation, you reported that Elena is taking metformin 500 mg once daily with dinner after stopping the morning dose for GI upset (saved physician update), whereas the initial chart review reflected the September 10, 2026 EHR prescription for 500 mg twice daily, and we discussed extended-release metformin if uptitration is needed.\n\nBasis and limitations: Saved physician memory correction (mem-metformin-evening-only), earlier conversation chat-1-metformin-correction, and initial chart understanding job-initial-understanding.",
                        )],
                        receipt: json!({"chat": 2, "turn": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected chat2 turn {other}"),
            }
        }
    }

    let chat2_claim = store.claim(&chat2_scope, 60).await.unwrap().unwrap();
    let mut chat2_host = PhysicianHost {
        store: store.clone(),
        claim: chat2_claim,
        authority: PostgresWorkspaceAuthority,
    };
    chat2_host
        .run(
            &mut Chat2Provider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    let chat2_saved = store
        .reopen_conversation(&chat2_scope)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(chat2_saved.outcome, HistoryOutcome::Answered);
    assert!(
        chat2_saved.turns[1]
            .raw_text
            .contains("500 mg once daily with dinner")
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn synthetic_patient_end_to_end_question_saves_answer_and_opens_citations() {
    let (store, auth, url) = setup_stores().await;
    let practice = format!("practice-e2e-{}", uuid::Uuid::new_v4().simple());
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-chen").await;
    let patient = synthetic_patient("syn-samuel-5", "Samuel Okafor", "1958-07-23", "MRN-55204");
    let workspace = workspace_id("synthetic-clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let renal_lab = readable_chart_record(
        &workspace,
        "rev-3",
        "2026-11-08",
        "Renal Function Panel",
        "Serum creatinine 1.42 mg/dL, eGFR 44 mL/min/1.73 m2, Urine ACR 180 mg/g on 2026-11-08.",
        json!({
            "resourceType": "Observation",
            "id": "lab-renal-2026-11-08",
            "creatinine_mg_dL": 1.42,
            "egfr_mL_min": 44,
            "uacr_mg_g": 180
        }),
    );
    let empagliflozin = readable_chart_record(
        &workspace,
        "rev-1",
        "2026-10-01",
        "Empagliflozin 10 mg oral tablet",
        "Empagliflozin 10 mg orally once daily (active, started 2026-10-01).",
        json!({
            "resourceType": "MedicationRequest",
            "id": "med-empagliflozin-2026-10-01",
            "medication": "Empagliflozin 10 mg oral tablet",
            "status": "active"
        }),
    );
    store
        .record_chart(
            &workspace,
            &[renal_lab, empagliflozin],
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient,
                pulled_at: 1_762_901_000,
            },
        )
        .await
        .unwrap();

    let scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-synthetic-e2e-ac5".into(),
    };
    let skills = load_skills();
    let roles = distinct_model_roles();
    let recipe = ChatAssignment::for_patient(
        scope.clone(),
        store.patient_starting_context(&scope).await.unwrap(),
    )
    .recipe(&roles, &skills);

    store
        .start_chat_authorized(
            &scope,
            &Input {
                key: "cmd-ac5-1".into(),
                text: "Should Samuel continue empagliflozin given his November 2026 kidney labs?"
                    .into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session_hash,
            &skills,
            &recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    struct EndToEndSyntheticProvider {
        turn: usize,
    }
    impl Provider for EndToEndSyntheticProvider {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "search-samuel".into(),
                        name: SEARCH_TOOL.into(),
                        arguments: json!({"query": "", "record_type": null, "after": null})
                            .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "read-samuel".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [
                                {
                                    "candidate_id": "lab-renal-2026-11-08",
                                    "revision": "rev-3",
                                    "cursor": null
                                },
                                {
                                    "candidate_id": "med-empagliflozin-2026-10-01",
                                    "revision": "rev-1",
                                    "cursor": null
                                }
                            ],
                            "max_result_bytes": 16384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 2}),
                    usage: Default::default(),
                }),
                3 => Ok(Turn {
                    items: vec![Item::assistant(
                        "Yes — continue Samuel's empagliflozin 10 mg once daily [2]. His November 8, 2026 renal panel shows eGFR 44 mL/min/1.73 m2 with persistent albuminuria (urine ACR 180 mg/g) [1], well above the continuation threshold for renoprotection.\n\nBasis and limitations: Synthetic Clinic EHR Renal Function Panel (2026-11-08) and active empagliflozin 10 mg order (2026-10-01). No pre-initiation baseline creatinine prior to October 2026 is present in the chart.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store,
        claim,
        authority: PostgresWorkspaceAuthority,
    };
    host.run(
        &mut EndToEndSyntheticProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    drop(host);

    // Reopen from a brand-new database connection pool and inspect the saved answer + opened citations.
    let fresh_store = Store::connect(&url).await.unwrap();
    let reopened = fresh_store
        .reopen_conversation(&scope)
        .await
        .unwrap()
        .expect("saved synthetic conversation reopens");
    assert_eq!(reopened.outcome, HistoryOutcome::Answered);
    assert_eq!(reopened.turns.len(), 2);
    assert_eq!(reopened.turns[0].author, TurnAuthor::Physician);
    assert_eq!(
        reopened.turns[1].author,
        TurnAuthor::Assistant { partial: false }
    );
    assert!(!reopened.turns[1].newer_records);
    assert!(reopened.turns[1].basis.is_some());

    let citations: Vec<_> = reopened.turns[1]
        .segments
        .iter()
        .filter_map(|segment| match segment {
            HistorySegment::Citation {
                source_key,
                number,
                label,
                quote,
            } => Some((source_key.clone(), *number, label.clone(), quote.clone())),
            HistorySegment::Text { .. } => None,
        })
        .collect();
    assert_eq!(citations.len(), 2, "both [2] and [1] citations resolve");

    for (source_key, number, label, quote) in &citations {
        let opened = reopened
            .sources
            .get(source_key)
            .unwrap_or_else(|| panic!("missing opened citation {source_key}"));
        assert_eq!(opened.number, *number);
        assert_eq!(opened.source_label, "Synthetic Clinic EHR");
        assert!(!label.trim().is_empty());
        assert!(!quote.trim().is_empty());
        assert!(!opened.body.trim().is_empty());
        assert!(!opened.raw_content.trim().is_empty());
    }

    let opened_lab = reopened
        .sources
        .values()
        .find(|source| source.candidate_id == "lab-renal-2026-11-08")
        .expect("opened renal lab citation");
    assert_eq!(opened_lab.number, 1);
    assert_eq!(opened_lab.revision, "rev-3");
    assert_eq!(opened_lab.title, "Renal Function Panel");
    assert_eq!(opened_lab.recorded_at.as_deref(), Some("2026-11-08"));
    assert!(opened_lab.body.contains("eGFR 44 mL/min/1.73 m2"));

    let opened_med = reopened
        .sources
        .values()
        .find(|source| source.candidate_id == "med-empagliflozin-2026-10-01")
        .expect("opened empagliflozin citation");
    assert_eq!(opened_med.number, 2);
    assert_eq!(opened_med.revision, "rev-1");
    assert_eq!(opened_med.title, "Empagliflozin 10 mg oral tablet");
    assert!(
        opened_med
            .body
            .contains("Empagliflozin 10 mg orally once daily")
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn interrupted_chat_attempt_reconciles_saved_memory_and_concern_receipts_without_unknown_or_reexecution()
 {
    let (store, auth, _url) = setup_stores().await;
    let practice = format!("practice-reconcile-{}", uuid::Uuid::new_v4().simple());
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-shah").await;
    let patient = synthetic_patient("syn-nora-6", "Nora Alvarez", "1968-02-14", "MRN-66019");
    let workspace = workspace_id("synthetic-clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let k_lab = readable_chart_record(
        &workspace,
        "rev-1",
        "2026-11-11",
        "Serum Potassium",
        "Serum potassium 5.7 mmol/L on 2026-11-11.",
        json!({
            "resourceType": "Observation",
            "id": "lab-k-recovery",
            "potassium_mmol_L": 5.7
        }),
    );
    store
        .record_chart(
            &workspace,
            &[k_lab],
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient,
                pulled_at: 1_762_902_000,
            },
        )
        .await
        .unwrap();

    // Seed an active memory entry that the interrupted attempt will retire.
    store
        .record_patient_memory(
            "seed-prior-kcl",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: PatientMemoryId::new("mem-seeded-kcl").unwrap(),
                topic: "Potassium supplement".into(),
                statement: "Previously taking potassium chloride 20 mEq daily.".into(),
                provenance: MemoryProvenance::PhysicianCorrection {
                    physician: physician.physician_id.clone(),
                    chat_id: "chat-seed".into(),
                    message_id: "msg-seed".into(),
                    work_id: "work-seed".into(),
                    attempt_id: "attempt-seed".into(),
                },
            },
        )
        .await
        .unwrap();

    let scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-crash-reconcile".into(),
    };
    let skills = load_skills();
    let roles = distinct_model_roles();
    let recipe = ChatAssignment::for_patient(
        scope.clone(),
        store.patient_starting_context(&scope).await.unwrap(),
    )
    .recipe(&roles, &skills);

    store
        .start_chat_authorized(
            &scope,
            &Input {
                key: "cmd-reconcile-1".into(),
                text: "Nora stopped potassium chloride yesterday and switched to a low-potassium diet; flag her potassium of 5.7 on Today.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session_hash.clone(),
            &skills,
            &recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    let call_mem_record = Invocation {
        id: "call-mem-record".into(),
        name: PATIENT_MEMORY_TOOL.into(),
        arguments: json!({
            "operation": "record_correction",
            "memory_id": "mem-low-k-diet",
            "topic": "Dietary potassium",
            "statement": "Switched to a low-potassium diet on November 10, 2026 per physician."
        })
        .to_string(),
    };
    let call_mem_retire = Invocation {
        id: "call-mem-retire".into(),
        name: PATIENT_MEMORY_TOOL.into(),
        arguments: json!({
            "operation": "retire",
            "memory_id": "mem-seeded-kcl",
            "reason": "Physician confirmed potassium chloride 20 mEq daily was stopped."
        })
        .to_string(),
    };
    let call_concern_raise = Invocation {
        id: "call-concern-raise".into(),
        name: CONCERN_TOOL.into(),
        arguments: json!({
            "operation": "raise",
            "concern_id": "concern-k-5-7",
            "urgency": "now",
            "reason": "Serum potassium 5.7 mmol/L on November 11, 2026 requires prompt repeat check.",
            "records": [{
                "candidate_id": "lab-k-recovery",
                "revision": "rev-1"
            }]
        })
        .to_string(),
    };
    let call_concern_uncommitted = Invocation {
        id: "call-concern-uncommitted".into(),
        name: CONCERN_TOOL.into(),
        arguments: json!({
            "operation": "revise",
            "concern_id": "concern-k-5-7",
            "urgency": "today",
            "reason": "Uncommitted revision started just before worker crash.",
            "records": [{
                "candidate_id": "lab-k-recovery",
                "revision": "rev-1"
            }]
        })
        .to_string(),
    };

    // Memory operation wrapper that commits `call-mem-record` to real Postgres and then
    // cancels `Attempt 1` before `ToolReturned` can be persisted.
    struct CancelAfterFirstRecordMemory {
        cancel: engine::Cancellation,
    }

    #[async_trait]
    impl PatientMemoryOperation for CancelAfterFirstRecordMemory {
        async fn list_memories(
            &self,
            store: &Store,
            practice: &str,
            workspace: &str,
            selection: MemorySelection,
        ) -> Result<Vec<PatientMemoryEntry>, PatientMemoryError> {
            PostgresPatientMemoryOperation
                .list_memories(store, practice, workspace, selection)
                .await
        }

        async fn record_correction(
            &self,
            store: &Store,
            command_key: &str,
            input: &RecordMemoryCorrection,
        ) -> Result<MemoryMutation, PatientMemoryError> {
            let committed = PostgresPatientMemoryOperation
                .record_correction(store, command_key, input)
                .await?;
            self.cancel.cancel();
            std::future::pending::<()>().await;
            Ok(committed)
        }

        async fn retire_memory(
            &self,
            store: &Store,
            command_key: &str,
            input: &RetirePatientMemory,
        ) -> Result<MemoryMutation, PatientMemoryError> {
            PostgresPatientMemoryOperation
                .retire_memory(store, command_key, input)
                .await
        }

        async fn saved_mutation(
            &self,
            store: &Store,
            practice: &str,
            workspace: &str,
            command_key: &str,
            id: &PatientMemoryId,
        ) -> Result<Option<PatientMemoryEntry>, PatientMemoryError> {
            PostgresPatientMemoryOperation
                .saved_mutation(store, practice, workspace, command_key, id)
                .await
        }
    }

    struct FirstAttemptProvider {
        calls: Vec<Invocation>,
    }

    impl Provider for FirstAttemptProvider {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            Ok(Turn {
                items: self.calls.iter().cloned().map(Item::Call).collect(),
                receipt: json!({"attempt": 1, "turn": 1}),
                usage: Default::default(),
            })
        }
    }

    let cancel_attempt_1 = engine::Cancellation::default();
    let crashing_store =
        store
            .clone()
            .with_memory_operation(Arc::new(CancelAfterFirstRecordMemory {
                cancel: cancel_attempt_1.clone(),
            }));

    // Claim Attempt 1 with a 1-second lease so a successor worker can claim after abandonment.
    let claim_1 = crashing_store.claim(&scope, 1).await.unwrap().unwrap();
    let mut host_1 = PhysicianHost {
        store: crashing_store,
        claim: claim_1.clone(),
        authority: PostgresWorkspaceAuthority,
    };
    let attempt_1_err = host_1
        .run(
            &mut FirstAttemptProvider {
                calls: vec![
                    call_mem_record.clone(),
                    call_mem_retire.clone(),
                    call_concern_raise.clone(),
                    call_concern_uncommitted.clone(),
                ],
            },
            engine::RunLimits::default(),
            &cancel_attempt_1,
        )
        .await
        .expect_err("attempt 1 must abort after committing call-mem-record before ToolReturned");
    assert!(
        matches!(attempt_1_err, engine::Failure::Cancelled),
        "expected cancellation failure, got {attempt_1_err}"
    );

    // Inspect Attempt 1's persisted events so we append `ToolStarted` for the remaining 3 calls:
    // `call-mem-retire` and `call-concern-raise` execute through real `PhysicianHost::execute`
    // without persisting `ToolReturned`, while `call-concern-uncommitted` never executes at all.
    let after_crash = store.conversation(&scope).await.unwrap();
    let last_seq = after_crash
        .attempts
        .last()
        .unwrap()
        .events
        .last()
        .unwrap()
        .sequence;
    let last_ordinal = after_crash
        .attempts
        .last()
        .unwrap()
        .events
        .last()
        .unwrap()
        .ordinal;

    let mut exec_host = PhysicianHost {
        store: store.clone(),
        claim: claim_1.clone(),
        authority: PostgresWorkspaceAuthority,
    };
    for (offset, (call, should_execute)) in [
        (&call_mem_retire, true),
        (&call_concern_raise, true),
        (&call_concern_uncommitted, false),
    ]
    .into_iter()
    .enumerate()
    {
        let sequence = last_seq + (offset as u64) + 1;
        store
            .persist(
                &claim_1,
                Event {
                    attempt_id: claim_1.attempt.clone(),
                    ordinal: last_ordinal,
                    sequence,
                    kind: EventKind::ToolStarted(call.clone()),
                },
            )
            .await
            .unwrap();
        if should_execute {
            let parsed_args: Value = serde_json::from_str(&call.arguments).unwrap();
            let outcome = Product::execute(&mut exec_host, call, parsed_args)
                .await
                .unwrap();
            assert!(
                matches!(outcome, engine::ToolOutcome::Returned(_)),
                "expected committed tool outcome before simulated crash"
            );
        }
    }

    // Wait for Attempt 1's 1-second lease to expire, then claim Successor Attempt 2.
    tokio::time::sleep(std::time::Duration::from_millis(1_150)).await;
    let claim_2 = store
        .claim(&scope, 60)
        .await
        .unwrap()
        .expect("successor attempt claims abandoned chat turn");
    assert_ne!(claim_2.attempt, claim_1.attempt);

    struct SuccessorRecoveryProvider {
        turn: usize,
    }

    impl Provider for SuccessorRecoveryProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            assert_eq!(self.turn, 1, "successor completes in a single resumed turn");
            let outputs = materialized_outputs(request);

            // 1. `call-mem-record` reconciled via `saved_outcome` as `"duplicate"` (NOT `"unknown"`).
            let mem_record_out: Value =
                serde_json::from_str(outputs.get("call-mem-record").unwrap()).unwrap();
            assert_eq!(mem_record_out["outcome"], "duplicate", "{mem_record_out}");
            assert_eq!(mem_record_out["saved"], true);
            assert_eq!(mem_record_out["entry"]["id"], "mem-low-k-diet");
            assert_eq!(mem_record_out["entry"]["revision"], 1);

            // 2. `call-mem-retire` reconciled via `saved_outcome` as `"duplicate"` (NOT `"unknown"`).
            let mem_retire_out: Value =
                serde_json::from_str(outputs.get("call-mem-retire").unwrap()).unwrap();
            assert_eq!(mem_retire_out["outcome"], "duplicate", "{mem_retire_out}");
            assert_eq!(mem_retire_out["saved"], true);
            assert_eq!(mem_retire_out["entry"]["id"], "mem-seeded-kcl");
            assert_eq!(mem_retire_out["entry"]["revision"], 2);
            assert_eq!(mem_retire_out["entry"]["standing"]["state"], "retired");

            // 3. `call-concern-raise` reconciled via `saved_outcome` as `"duplicate"` (NOT `"unknown"`).
            let concern_raise_out: Value =
                serde_json::from_str(outputs.get("call-concern-raise").unwrap()).unwrap();
            assert_eq!(
                concern_raise_out["outcome"], "duplicate",
                "{concern_raise_out}"
            );
            assert_eq!(concern_raise_out["concern"]["id"], "concern-k-5-7");
            assert_eq!(concern_raise_out["concern"]["urgency"], "now");
            assert_eq!(concern_raise_out["concern"]["revision"], 1);

            // 4. `call-concern-uncommitted` had no committed receipt, so it MUST remain `"unknown"`,
            // even though `concern-k-5-7` exists in Postgres from `call-concern-raise`!
            let uncommitted_out: Value =
                serde_json::from_str(outputs.get("call-concern-uncommitted").unwrap()).unwrap();
            assert_eq!(uncommitted_out["outcome"], "unknown", "{uncommitted_out}");

            Ok(Turn {
                items: vec![Item::assistant(
                    "Recorded Nora's switch to a low-potassium diet, retired her prior potassium chloride note, and flagged her November 11, 2026 potassium of 5.7 mmol/L as an urgent concern.\n\nBasis and limitations: Synthetic Clinic EHR Serum Potassium (2026-11-11) and physician-reported medication/diet updates.",
                )],
                receipt: json!({"attempt": 2, "turn": 1}),
                usage: Default::default(),
            })
        }
    }

    let mut host_2 = PhysicianHost {
        store: store.clone(),
        claim: claim_2,
        authority: PostgresWorkspaceAuthority,
    };
    host_2
        .run(
            &mut SuccessorRecoveryProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    // Verify none of the reconciled mutations double-incremented or mutated further.
    let all_memories = store
        .patient_memories(&practice, &workspace, MemorySelection::All)
        .await
        .unwrap();
    assert_eq!(all_memories.len(), 2);
    let low_k = all_memories
        .iter()
        .find(|entry| entry.id.as_str() == "mem-low-k-diet")
        .unwrap();
    assert_eq!(low_k.revision, 1);
    assert_eq!(low_k.standing, MemoryStanding::Active);

    let retired_kcl = all_memories
        .iter()
        .find(|entry| entry.id.as_str() == "mem-seeded-kcl")
        .unwrap();
    assert_eq!(retired_kcl.revision, 2);
    assert!(matches!(
        retired_kcl.standing,
        MemoryStanding::Retired { .. }
    ));

    let open_concerns = store
        .open_concerns(
            &WorkspaceAccess {
                practice: &practice,
                physician: &physician.physician_id,
                workspace: &workspace,
                session_hash: &session_hash,
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(open_concerns.len(), 1);
    assert_eq!(open_concerns[0].id.as_str(), "concern-k-5-7");
    assert_eq!(open_concerns[0].urgency, ConcernUrgency::Now);
    assert_eq!(open_concerns[0].revision, 1);

    let saved_conv = store.conversation(&scope).await.unwrap();
    assert_eq!(saved_conv.status, WorkStatus::Completed);
    assert_eq!(saved_conv.messages.len(), 2);
}
