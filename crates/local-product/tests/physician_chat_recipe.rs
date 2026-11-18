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
        SourceAdapterError, SourcePatient, SourceRevision, workspace_id,
    },
    runtime::{
        Accepted, Authority, Input, PhysicianHost, PublicationTransaction, Scope, ShownSteps,
        Store, WorkStatus, async_trait,
        chat_operations::{
            ChatStartError, DispatchCheckStanding, PatientMemoryStanding, REFRESH_SOURCES_TOOL,
            RETRIEVE_PATIENT_WORK_TOOL, RecordLimit, RecordLimitKind,
            SourceAdapterRefreshOperation,
        },
        concerns::{CONCERN_TOOL, ConcernStanding, ConcernUrgency, WorkspaceAccess},
        dispatch::{PatientSourceChecker, SourceCheckError},
        engine::{
            self, Invocation, Item, Turn,
            recipe::{Observer, Provider, Request},
        },
        history::HistoryOutcome,
        memory::{self, MemoryPath, PatientKey},
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
        source_patient_id: format!("{id}-{}", uuid::Uuid::new_v4().simple()),
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
        envelope: None,
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

    async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
        Ok(vec![self.patient.source_patient_id.clone()])
    }

    async fn patient(&self, patient_id: &str) -> Result<SourcePatient, SourceAdapterError> {
        if patient_id == self.patient.source_patient_id {
            Ok(self.patient.clone())
        } else {
            Err(SourceAdapterError::Unavailable {
                reason: format!("unknown synthetic patient {patient_id}"),
            })
        }
    }

    async fn records(
        &self,
        _patient: &SourcePatient,
        _workspace: &str,
    ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
        self.state
            .lock()
            .unwrap()
            .clone()
            .map_err(|reason| SourceAdapterError::Unavailable { reason })
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
                pulled_at: Some(1_762_900_100),
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
        recipe
            .instructions
            .contains("Loaded skill: patient-record-retrieval"),
        "chat recipe loads patient-record-retrieval after the catalog"
    );
    assert!(
        !recipe
            .instructions
            .contains("Loaded skill: patient-communication"),
        "non-start skill bodies must open on demand via skill_view"
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
        "list_patient_memory",
        "read_patient_memory",
        "search_patient_memory",
        "add_patient_memory_note",
        "edit_patient_memory",
        RETRIEVE_PATIENT_WORK_TOOL,
        CONCERN_TOOL,
        SEARCH_LIBRARY_TOOL,
        READ_LIBRARY_TOOL,
        local_product::physician_sources::citations::FINISH_TOOL,
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
                            .contains("# Patient Record Retrieval")
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
                        items: vec![Item::Call(Invocation {
                            id: "finish-elena-1".into(),
                            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": "Elena's potassium is 4.6 mmol/L and creatinine is 1.18 mg/dL (eGFR 52 mL/min/1.73 m2) on November 4, 2026 [S1] while taking lisinopril 20 mg daily [S2]. Continuing lisinopril 20 mg daily is appropriate; if messaging her, reassure her that her kidney numbers and potassium remain stable without implying any prescription change.",
                                "quotes": [],
                                "descriptions": [],
                                "basis": "Synthetic Clinic EHR Basic Metabolic Panel (2026-11-04) [S1] and active lisinopril 20 mg order (2026-10-15) [S2], plus patient-record-retrieval and patient-communication guidance. No home blood pressure readings are in the chart.",
                                "cites_none": false
                            })
                            .to_string(),
                        })],
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
        shown_steps: ShownSteps::new(()),
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
        envelope: None,
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
        envelope: None,
    };

    base_store
        .record_chart(
            &workspace,
            &[readable_a1c, unreadable_scan, unreadable_fax],
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: Some(1_762_900_200),
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
                        items: vec![Item::Call(Invocation {
                            id: "finish-marcus-1".into(),
                            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": "Marcus's Hemoglobin A1c on November 2, 2026 is 7.4% [S1]. I cannot confirm his lipid panel because that record is missing from the chart, I cannot interpret the October 28, 2026 outside echocardiogram scan or the October 30, 2026 cardiology fax because both documents are unreadable without extracted clinical text, and recent Regional Hospital ADT discharge status is unavailable because the hospital feed did not respond.",
                                "quotes": [],
                                "descriptions": [],
                                "basis": "Synthetic Clinic EHR Hemoglobin A1c (2026-11-02) [S1]. Missing: 2026 lipid panel. Unreadable: outside echocardiogram PDF scan (doc-outside-echo-scan) and cardiology fax with failed OCR (doc-corrupt-cardiology-fax). Unavailable: Regional Hospital ADT live refresh and clinical library guideline lookup.",
                                "cites_none": false
                            })
                            .to_string(),
                        })],
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
        shown_steps: ShownSteps::new(()),
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
                pulled_at: Some(1_762_900_000),
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
                                name: "add_patient_memory_note".into(),
                                arguments: json!({
                                    "text": "Stopped oral potassium supplement 3 days ago per Dr. On-Panel.",
                                    "quote": null
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
                    assert_eq!(mem_out["note"], "N1");
                    assert!(
                        mem_out["path"].as_str().unwrap().starts_with("log/"),
                        "expected log path, got {mem_out}"
                    );

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
                                name: "list_patient_memory".into(),
                                arguments: json!({}).to_string(),
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
                    assert_eq!(mem_read["log"].as_array().unwrap().len(), 1);

                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "finish-priya-1".into(),
                            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": "Refreshed Synthetic Clinic EHR records show Priya's serum potassium is 5.9 mmol/L on November 12, 2026 [S1]. I noted your update that she stopped her potassium supplement 3 days ago, raised an urgent concern, and recommend a prompt 12-lead ECG, repeat non-hemolyzed potassium, and holding potassium-retaining agents.",
                                "quotes": [],
                                "descriptions": [],
                                "basis": "Refreshed Synthetic Clinic EHR Serum Potassium (2026-11-12) [S1], DailyMed hyperkalemia clinical monograph (2026-03-15), and physician-reported supplement stop.",
                                "cites_none": false
                            })
                            .to_string(),
                        })],
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
        shown_steps: ShownSteps::new(()),
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

    let memory_key = PatientKey::for_rounds(&practice, &workspace).unwrap();
    let listing_after = store.list_memory(&memory_key).await.unwrap();
    assert_eq!(listing_after.log_days.len(), 1);
    let log_path = MemoryPath::Log(listing_after.log_days[0]);
    let log_text = match store
        .read_memory(&memory_key, &log_path, None)
        .await
        .unwrap()
    {
        memory::FileRead::Log { markdown } | memory::FileRead::Version { markdown, .. } => markdown,
    };
    assert!(log_text.contains("Stopped oral potassium supplement 3 days ago per Dr. On-Panel."));

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

    // Now inject failures across refresh, refused memory note, and unverified concern and prove NOTHING new saves.
    *ehr_adapter.state.lock().unwrap() = Err("EHR endpoint connection reset".into());
    let pulled_before_failure = store
        .source_pulls(std::slice::from_ref(&workspace))
        .await
        .unwrap()[&workspace][0]
        .pulled_at;

    let fail_scope = Scope {
        practice: practice.clone(),
        physician: on_panel_doc.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-ops-failures-ac3".into(),
    };
    let mut unavailable_ctx = store.patient_starting_context(&fail_scope).await.unwrap();
    unavailable_ctx.memory_standing = PatientMemoryStanding::Unavailable {
        reason: "synthetic memory store outage".into(),
    };
    let unavailable_text = unavailable_ctx.developer_text();
    assert!(
        unavailable_text.contains("Memory limit (unavailable):"),
        "failed memory read must surface as an explicit unavailable limit, got: {unavailable_text}"
    );
    assert!(
        unavailable_text.contains("synthetic memory store outage"),
        "failed memory read must carry its operational reason, got: {unavailable_text}"
    );
    assert!(
        !unavailable_text.contains("Stopped oral potassium supplement"),
        "unavailable memory must not masquerade as available entries: {unavailable_text}"
    );

    let fail_starting = store.patient_starting_context(&fail_scope).await.unwrap();
    let fail_recipe =
        ChatAssignment::for_patient(fail_scope.clone(), fail_starting).recipe(&roles, &skills);
    store
        .start_chat_authorized(
            &fail_scope,
            &Input {
                key: "cmd-ac3-fail".into(),
                text:
                    "Try refreshing again, saving a blank note, and raising an unverified concern."
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
                1 => Ok(Turn {
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
                            name: "add_patient_memory_note".into(),
                            arguments: json!({
                                "text": " \n",
                                "quote": null
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
                }),
                2 => {
                    let outputs = materialized_outputs(request);
                    let refresh_fail: Value =
                        serde_json::from_str(outputs.get("fail-refresh").unwrap()).unwrap();
                    assert_eq!(refresh_fail["outcome"], "failed");
                    assert_eq!(refresh_fail["saved"], false);

                    let memory_fail = outputs.get("fail-memory").unwrap();
                    assert!(
                        memory_fail.contains("a note needs text"),
                        "expected refused blank memory note, got {memory_fail}"
                    );

                    let concern_refusal = outputs.get("fail-concern").unwrap();
                    assert!(
                        concern_refusal.contains("not read in this run")
                            || concern_refusal.contains("refused")
                            || concern_refusal.contains("rev-999"),
                        "expected unverified concern refusal, got {concern_refusal}"
                    );

                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "finish-fail-1".into(),
                            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": "Live EHR refresh and blank memory note both failed and were not saved, and no new concern was recorded without verified chart evidence.",
                                "quotes": [],
                                "descriptions": [],
                                "basis": "Unavailable live Synthetic Clinic EHR refresh and refused memory note; retained chart remains at the earlier November 12, 2026 pull.",
                                "cites_none": true
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let fail_claim = store.claim(&fail_scope, 60).await.unwrap().unwrap();
    let mut fail_host = PhysicianHost {
        store: store.clone(),
        claim: fail_claim,
        authority: PostgresWorkspaceAuthority,
        shown_steps: ShownSteps::new(()),
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
    let log_still = match store
        .read_memory(&memory_key, &log_path, None)
        .await
        .unwrap()
    {
        memory::FileRead::Log { markdown } | memory::FileRead::Version { markdown, .. } => markdown,
    };
    assert_eq!(log_still, log_text);
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
        shown_steps: ShownSteps::new(()),
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
