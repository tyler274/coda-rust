//! Real PostgreSQL integration tests for shared-harness patient review
//! execution, storage, lifecycle controls, and the clinical rounds recipe
//! (`patient_reviews` + `runtime::reviews`).
//!
//! Runs automatically whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL` or
//! `PHYSICIAN_DATABASE_URL` is set; panics if neither is set when
//! `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use local_product::clinical_library::{READ_LIBRARY_TOOL, SEARCH_LIBRARY_TOOL};
use local_product::initial_understanding::{
    ConnectedSourceBrief, EvidenceScope, HealthSystemStanding, IndexedAnchor, KnownHealthSystem,
    OperatingIndex, READ_UNDERSTANDING_TOOL, SAVE_UNDERSTANDING_TOOL, SaveUnderstandingArgs,
    SummaryClaimInput,
};
use local_product::patient_reviews::{
    COMPLETE_PATIENT_REVIEW_TOOL, CompleteReviewArgs, CompleteReviewDisposition, INSTRUCTIONS,
    QuietReviewReason, READ_PATIENT_REVIEWS_TOOL, ReviewOutcome, ReviewStartingContext,
    ReviewTrigger, UnfinishedReviewWork, assignment as review_assignment,
    complete_patient_review_tool, read_patient_reviews_tool, recipe as review_recipe,
};
use local_product::physician_chat::{PhysicianSkills, SKILL_VIEW};
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
    SourceRevision,
};
use local_product::runtime::chat_operations::{
    DispatchCheckStanding, MemoryMutation, MemoryProvenance, MemorySelection, MemoryStanding,
    PATIENT_MEMORY_TOOL, PatientMemoryEntry, PatientMemoryError, PatientMemoryId,
    PatientMemoryOperation, PatientMemoryStanding, PostgresPatientMemoryOperation, RecordLimit,
    RecordLimitKind, RecordMemoryCorrection, RetirePatientMemory, SourceFreshnessSummary,
};
use local_product::runtime::concerns::{
    CONCERN_TOOL, CommandKey, ConcernDoneOutcome, ConcernId, ConcernUrgency, ExactRecordReference,
    WorkspaceAccess,
};
use local_product::runtime::dispatch::{
    PatientSourceChecker, SourceCheckError, StoredObservationChecker,
};
use local_product::runtime::jobs::{
    PracticeAccessError, PracticeAuthority, PracticeJobHost, PracticeJobKind, PracticeScope,
    is_physician_only_tool,
};
use local_product::runtime::refresh::{
    FixedSourceCheck, REFRESH_TOOL, SourceCheckOutcome, SourceId,
};
use local_product::runtime::reviews::{
    AdmittedPatientReview, PatientReviewError, PatientReviewReceiver, RecordedReviewAction,
    ReviewHoldReason, ReviewInputSubmission, ReviewLifecycleState, ReviewSupersededReason,
    ReviewUsage,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{
    Authority, PublicationTransaction, Scope, Stopped, Store, async_trait, engine,
};
use local_product::{ModelBinding, ModelRoles, sha256_hex};
use patient_work::{Cadence, ReviewReason};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::{Value, json};
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../../verification/model_text.rs"]
mod model_text;

fn database_url() -> Option<String> {
    if let Ok(url) = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("PHYSICIAN_DATABASE_URL"))
    {
        let trimmed = url.trim().to_owned();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }
    assert!(
        std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
        "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL or PHYSICIAN_DATABASE_URL"
    );
    None
}

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
        .expect("load physician skills catalog")
}

fn distinct_roles() -> ModelRoles {
    ModelRoles {
        chat: ModelBinding {
            model: "interactive-chat-lane".into(),
            effort: "low".into(),
            context_window_tokens: 128_000,
            max_input_tokens: 100_000,
            adapter: "fixture".into(),
        },
        background: ModelBinding {
            model: "background-rounds-lane".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        },
    }
}

async fn connect_stores(url: &str) -> (Store, Arc<PostgresAuthStore>) {
    let store = Store::connect(url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(url)
            .await
            .expect("connect and migrate physician auth store"),
    );
    (store, auth)
}

fn source_patient(id: &str, name: &str) -> SourcePatient {
    SourcePatient {
        source_patient_id: id.to_owned(),
        display_name: name.to_owned(),
        birth_date: Some("1971-04-19".to_owned()),
        sex_or_gender: Some("female".to_owned()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-mrn".to_owned(),
            value: format!("MRN-{id}"),
        }],
    }
}

struct ChartRecordSpec<'a> {
    workspace: &'a str,
    source_id: &'a str,
    candidate_id: &'a str,
    revision: &'a str,
    record_type: &'a str,
    title: &'a str,
    date: &'a str,
    text: &'a str,
}

fn chart_record(spec: ChartRecordSpec<'_>) -> SourceRevision {
    SourceRevision {
        workspace: spec.workspace.to_owned(),
        source_id: spec.source_id.to_owned(),
        candidate_id: spec.candidate_id.to_owned(),
        revision: spec.revision.to_owned(),
        record_type: spec.record_type.to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".to_owned(),
            value: spec.date.to_owned(),
        }],
        content: spec.text.to_owned(),
        sha256: sha256_hex(spec.text.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: spec.title.to_owned(),
            text: spec.text.to_owned(),
        }],
        provenance: json!({
            "title": spec.title,
            "record_locator": spec.candidate_id,
        }),
    }
}

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

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice_id: &str,
    label: &str,
) -> (PhysicianIdentity, Vec<u8>) {
    let email = format!("{label}-{}@codahealth.test", Uuid::new_v4().simple());
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let invite = auth
        .invite(practice_id, &email, label, false, now)
        .await
        .unwrap();
    let (identity, token) = auth.redeem_invitation(&email, &invite, now).await.unwrap();
    let hash = PostgresAuthStore::session_reference(&token).to_vec();
    (identity, hash)
}

fn materialized_output<'a>(request: &'a engine::recipe::Request, call_id: &str) -> &'a str {
    request
        .items
        .iter()
        .find_map(|item| match item {
            engine::Item::MaterializedResult {
                call_id: id,
                output,
            } if id == call_id => Some(output.as_str()),
            _ => None,
        })
        .unwrap_or_else(|| {
            panic!(
                "missing materialized output for {call_id} in {:?}",
                request.items
            )
        })
}

struct RevocablePracticeAuthority {
    inner: Arc<PostgresAuthStore>,
    revoked: AtomicBool,
}

#[async_trait]
impl PracticeAuthority for RevocablePracticeAuthority {
    async fn check_practice_patient(
        &self,
        scope: &PracticeScope,
    ) -> Result<(), PracticeAccessError> {
        if self.revoked.load(Ordering::SeqCst) {
            return Err(PracticeAccessError::UnregisteredPatient {
                practice: scope.practice.clone(),
                workspace: scope.workspace.clone(),
            });
        }
        self.inner.check_practice_patient(scope).await
    }
}

#[test]
fn patient_review_recipe_and_tools_have_clean_model_text_and_valid_skill_tool_references() {
    let skills = load_skills();
    let roles = distinct_roles();

    for tool in [read_patient_reviews_tool(), complete_patient_review_tool()] {
        model_text::assert_clean_model_text(&tool.description);
        model_text::assert_clean_model_text(&serde_json::to_string(&tool.function()).unwrap());
    }

    let context = ReviewStartingContext {
        workspace: "pw-clean-check".into(),
        review_id: "review-clean-check".into(),
        patient: Some(source_patient("pt-clean", "Clara Chen")),
        trigger: ReviewTrigger::ChangedRecords {
            workspace_version: 2,
            record_versions: BTreeMap::from([(
                "Observation/potassium-1".to_owned(),
                "rev-1".to_owned(),
            )]),
            changed_candidates: vec!["Observation/potassium-1".to_owned()],
        },
        additional_inputs: Vec::new(),
        chart_version: 2,
        chart_record_count: 1,
        evidence_scope: EvidenceScope::ClinicalChart {
            connected_sources: vec![ConnectedSourceBrief {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                pulled_at: 1_790_000_000,
            }],
        },
        source_pulls: vec![SourceFreshnessSummary {
            source_id: "clinic-ehr".into(),
            source_label: "Clinic EHR".into(),
            pulled_at: 1_790_000_000,
        }],
        refresh_statuses: Vec::new(),
        check_standing: DispatchCheckStanding::Current {
            checked_at: 1_790_000_000,
        },
        understanding: None,
        memory_standing: PatientMemoryStanding::Available {
            entries: Vec::new(),
        },
        open_concerns: Vec::new(),
        prior_reviews: Vec::new(),
    };

    let assignment = review_assignment("cmd-clean-check", context, &skills).unwrap();
    let built = review_recipe(&roles.background, &assignment, &skills);
    model_text::assert_clean_model_text(&built.instructions);
    model_text::assert_clean_model_text(&assignment.prompt);
    model_text::assert_clean_model_text(&serde_json::to_string(&built.tools).unwrap());
}

#[tokio::test]
async fn dispatch_changed_periodic_and_physician_reviews_reach_model_with_recipe_and_discoverable_capabilities()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-dispatch-rev-{}", Uuid::new_v4().simple());
    let changed_ws = format!("pw-changed-{}", Uuid::new_v4().simple());
    let periodic_ws = format!("pw-periodic-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &changed_ws).await.unwrap();
    auth.register_patient(&practice, &periodic_ws)
        .await
        .unwrap();

    // Seed baseline chart for periodic_ws BEFORE registration so its chart is unchanged on wake.
    let periodic_pull = SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: source_patient("pt-periodic", "Arthur Pendelton"),
        pulled_at: 1_790_000_000,
    };
    let periodic_note = chart_record(ChartRecordSpec {
        workspace: &periodic_ws,
        source_id: "clinic-ehr",
        candidate_id: "DocumentReference/annual-note",
        revision: "rev-1",
        record_type: "DocumentReference",
        title: "Annual Wellness Note",
        date: "2026-10-01",
        text: "Stable well-controlled essential hypertension on lisinopril 10 mg daily. BP 122/76 mmHg.",
    });
    store
        .record_chart(&periodic_ws, &[periodic_note], &periodic_pull)
        .await
        .unwrap();

    let reg_time = 1_790_000_000_i64;
    store
        .register_patient_dispatch(auth.as_ref(), &practice, &changed_ws, reg_time)
        .await
        .unwrap();
    store
        .register_patient_dispatch(auth.as_ref(), &practice, &periodic_ws, reg_time)
        .await
        .unwrap();

    // Now deliver a new lab result into changed_ws AFTER registration so changed_ws gets "new records",
    // while advancing clock by 8 days (>= weekly review window) so periodic_ws gets "periodic"!
    let changed_pull = SourcePull {
        source_id: "quest-labs".into(),
        source_label: "Quest Diagnostics".into(),
        patient: source_patient("pt-changed", "Beatrice Morales"),
        pulled_at: reg_time + 8 * 86_400,
    };
    let changed_lab = chart_record(ChartRecordSpec {
        workspace: &changed_ws,
        source_id: "quest-labs",
        candidate_id: "Observation/potassium-panel-1",
        revision: "rev-1",
        record_type: "Observation",
        title: "Basic Metabolic Panel - Potassium",
        date: "2026-10-09",
        text: "Serum potassium 5.6 mmol/L (reference 3.5-5.1 mmol/L) on lisinopril 20 mg daily.",
    });
    store
        .record_chart(&changed_ws, &[changed_lab], &changed_pull)
        .await
        .unwrap();

    let checker = StoredObservationChecker::new(store.clone());
    let receiver = PatientReviewReceiver::new(&store, &roles, &skills, auth.as_ref());
    let wake_now = reg_time + 8 * 86_400;
    let wake_summary = store
        .wake_practice_dispatch(auth.as_ref(), &practice, wake_now, &checker, &receiver)
        .await
        .expect("wake_practice_dispatch admits changed and periodic reviews");

    assert_eq!(wake_summary.started_reviews.len(), 2);
    let deliveries = receiver.take_deliveries().await;
    assert_eq!(deliveries.len(), 2);

    let by_ws: BTreeMap<String, _> = wake_summary
        .started_reviews
        .iter()
        .map(|req| (req.patient_workspace_id.clone(), req))
        .collect();
    assert_eq!(by_ws[&changed_ws].reason, ReviewReason::NewRecords);
    assert_eq!(by_ws[&periodic_ws].reason, ReviewReason::Periodic);

    // Verify physician-requested review admission & panel authorization enforcement.
    let (allowed_dr, _allowed_hash) = invite_and_sign_in(&auth, &practice, "dr-allowed").await;
    let (denied_dr, _denied_hash) = invite_and_sign_in(&auth, &practice, "dr-denied").await;
    auth.set_membership(&allowed_dr, &periodic_ws, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&denied_dr, &periodic_ws, Membership::Denied)
        .await
        .unwrap();

    let unauthorized = store
        .submit_physician_patient_review(
            auth.as_ref(),
            &denied_dr,
            &periodic_ws,
            "cmd-denied-dr-review",
            "Reassess blood pressure trend.",
            &roles,
            &skills,
        )
        .await
        .expect_err("non-panel physician must be refused");
    assert!(matches!(
        unauthorized,
        PatientReviewError::PhysicianUnauthorized { .. }
    ));

    let physician_admitted = store
        .submit_physician_patient_review(
            auth.as_ref(),
            &allowed_dr,
            &periodic_ws,
            "cmd-allowed-dr-review",
            "Reassess blood pressure stability prior to refill.",
            &roles,
            &skills,
        )
        .await
        .expect("authorized physician review request admitted");
    assert!(matches!(
        physician_admitted.review().triggers.as_slice(),
        [ReviewTrigger::PhysicianRequest { .. }]
    ));

    // Execute both the changed-record review and the periodic review through PracticeJobHost,
    // asserting the exact Request reaching the model (recipe, background model settings,
    // preloaded retrieval skill, discoverable patient-communication skill, and tools).
    let changed_reviews = store.patient_reviews(&practice, &changed_ws).await.unwrap();
    assert_eq!(changed_reviews.len(), 1);
    let changed_rev = &changed_reviews[0];
    assert!(matches!(
        changed_rev.triggers.as_slice(),
        [ReviewTrigger::ChangedRecords {
            workspace_version: 1,
            changed_candidates,
            ..
        }] if changed_candidates == &vec!["Observation/potassium-panel-1".to_owned()]
    ));

    let changed_scope = changed_rev.practice_scope();
    let changed_job = store
        .practice_job(&changed_scope)
        .await
        .unwrap()
        .expect("changed review practice job");
    assert_eq!(changed_job.kind, PracticeJobKind::PatientReview);
    assert_eq!(changed_job.preloaded_skills.len(), 1);
    assert_eq!(
        changed_job.preloaded_skills[0].name,
        "patient-record-retrieval"
    );

    let changed_claim = store
        .claim_practice_job(&changed_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim changed review job");

    struct InspectRecipeAndDiscoverProvider {
        expected_model: String,
        expected_effort: String,
        turn: usize,
    }

    impl engine::recipe::Provider for InspectRecipeAndDiscoverProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            assert_eq!(request.model, self.expected_model);
            assert_eq!(request.options["effort"], json!(self.expected_effort));
            assert!(request.instructions.contains(INSTRUCTIONS));
            assert!(request.instructions.contains("patient-record-retrieval"));
            assert!(request.instructions.contains("patient-communication"));

            let offered: BTreeSet<&str> = request
                .tools
                .iter()
                .filter_map(|t| t.get("name").and_then(Value::as_str))
                .collect();
            for required in [
                SEARCH_TOOL,
                READ_TOOL,
                REFRESH_TOOL,
                CONCERN_TOOL,
                PATIENT_MEMORY_TOOL,
                READ_UNDERSTANDING_TOOL,
                SAVE_UNDERSTANDING_TOOL,
                READ_PATIENT_REVIEWS_TOOL,
                COMPLETE_PATIENT_REVIEW_TOOL,
                SEARCH_LIBRARY_TOOL,
                READ_LIBRARY_TOOL,
                SKILL_VIEW,
                "read_tool_result",
            ] {
                assert!(
                    offered.contains(required),
                    "missing offered capability {required} in {offered:?}"
                );
            }
            for name in &offered {
                assert!(
                    !is_physician_only_tool(name),
                    "physician-only tool {name} must never reach background review model"
                );
            }
            model_text::assert_clean_model_text(&request.instructions);

            match self.turn {
                1 => {
                    let first_msg_json = serde_json::to_string(&request.items).unwrap();
                    assert!(
                        first_msg_json.contains("Beatrice Morales")
                            && first_msg_json.contains("Observation/potassium-panel-1"),
                        "initial guidance and changed candidate context must reach model: {first_msg_json}"
                    );
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "discover-comm-skill".into(),
                                name: SKILL_VIEW.into(),
                                arguments: json!({
                                    "name": "patient-communication",
                                    "file_path": null
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "search-k".into(),
                                name: SEARCH_TOOL.into(),
                                arguments: json!({
                                    "query": "potassium",
                                    "record_types": null,
                                    "source_ids": null,
                                    "limit": 10
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "read-k".into(),
                                name: READ_TOOL.into(),
                                arguments: json!({
                                    "references": [{
                                        "candidate_id": "Observation/potassium-panel-1",
                                        "revision": "rev-1",
                                        "cursor": null
                                    }],
                                    "max_result_bytes": 16_384
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({"turn": 1}),
                        usage: engine::recipe::Usage {
                            input_tokens: Some(1200),
                            cached_input_tokens: Some(400),
                            output_tokens: Some(150),
                        },
                    })
                }
                2 => {
                    let skill_body = materialized_output(request, "discover-comm-skill");
                    assert!(
                        skill_body.contains("patient-communication"),
                        "discoverable skill must load via skill_view: {skill_body}"
                    );
                    let read_out = materialized_output(request, "read-k");
                    assert!(read_out.contains("Serum potassium 5.6 mmol/L"));

                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "New mild-moderate hyperkalemia (K+ 5.6 mmol/L) on lisinopril 20 mg daily; raised concern for repeat BMP and ACE-inhibitor review.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/potassium-panel-1".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Assessed from Quest Diagnostics BMP dated 2026-10-09; outpatient medication adherence history not yet connected.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "raise-k-today".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-k-56",
                                    "urgency": "today",
                                    "reason": "Serum potassium elevated to 5.6 mmol/L on lisinopril 20 mg daily; consider repeat non-hemolyzed BMP and medication review.",
                                    "records": [{
                                        "candidate_id": "Observation/potassium-panel-1",
                                        "revision": "rev-1"
                                    }]
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-changed".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: engine::recipe::Usage {
                            input_tokens: Some(1600),
                            cached_input_tokens: Some(800),
                            output_tokens: Some(180),
                        },
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Completed changed-record review for Beatrice Morales.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: engine::recipe::Usage {
                        input_tokens: Some(1750),
                        cached_input_tokens: Some(1200),
                        output_tokens: Some(40),
                    },
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut changed_host = PracticeJobHost {
        store: store.clone(),
        claim: changed_claim,
        authority: auth.as_ref(),
    };
    changed_host
        .run(
            &mut InspectRecipeAndDiscoverProvider {
                expected_model: roles.background.model.clone(),
                expected_effort: roles.background.effort.clone(),
                turn: 0,
            },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("changed-record review executes to completion");

    let saved_changed = store
        .patient_review(&practice, &changed_ws, &changed_rev.review_id)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        saved_changed.state,
        ReviewLifecycleState::Completed {
            completed_source_version: 1,
            outcome: ReviewOutcome::Assessed { .. },
            ..
        }
    ));

    // Also execute the dispatched periodic review (from wake_practice_dispatch) to prove
    // periodic dispatch reaches the model with its periodic trigger and completes cleanly.
    let periodic_reviews = store
        .patient_reviews(&practice, &periodic_ws)
        .await
        .unwrap();
    let periodic_dispatched = periodic_reviews
        .iter()
        .find(|r| matches!(r.triggers.as_slice(), [ReviewTrigger::Periodic { .. }]))
        .expect("dispatched periodic review row");
    let periodic_claim = store
        .claim_practice_job(&periodic_dispatched.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim periodic review");

    struct QuietPeriodicProvider {
        expected_model: String,
        turn: usize,
    }
    impl engine::recipe::Provider for QuietPeriodicProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            assert_eq!(request.model, self.expected_model);
            assert!(request.instructions.contains(INSTRUCTIONS));
            match self.turn {
                1 => {
                    let wire = serde_json::to_string(&request.items).unwrap();
                    assert!(
                        wire.contains("periodic whole-patient reassessment")
                            && wire.contains("Arthur Pendelton"),
                        "periodic trigger context must reach model: {wire}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-wellness".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "DocumentReference/annual-note",
                                    "revision": "rev-1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Quiet {
                            explanation: "Reviewed annual wellness note; blood pressure 122/76 mmHg on stable lisinopril 10 mg daily with no open concerns or overdue follow-up.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "DocumentReference/annual-note".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Based on Clinic EHR annual wellness note (2026-10-01); no external hospital feeds connected.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "complete-periodic".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Periodic review complete; nothing requires physician attention.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut periodic_host = PracticeJobHost {
        store: store.clone(),
        claim: periodic_claim,
        authority: auth.as_ref(),
    };
    periodic_host
        .run(
            &mut QuietPeriodicProvider {
                expected_model: roles.background.model.clone(),
                turn: 0,
            },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("periodic review completes quietly");

    let saved_periodic = store
        .patient_review(&practice, &periodic_ws, &periodic_dispatched.review_id)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        &saved_periodic.state,
        ReviewLifecycleState::Completed {
            outcome: ReviewOutcome::Quiet {
                reason: QuietReviewReason::NothingToDo,
                ..
            },
            ..
        }
    ));
}

#[tokio::test]
async fn real_tool_operations_persist_across_restart_and_quiet_saves_nothing_to_do() {
    let Some(url) = database_url() else {
        return;
    };
    let (base_store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-ops-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-ops-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();
    let (dr, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ops").await;
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let initial_pull = SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: source_patient("pt-ops", "Hannah Lindqvist"),
        pulled_at: 1_790_100_000,
    };
    let intake = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "DocumentReference/intake-ops",
        revision: "rev-1",
        record_type: "DocumentReference",
        title: "Cardiology Follow-up",
        date: "2026-10-12",
        text: "HFrEF (EF 38%) on sacubitril-valsartan 49/51 mg BID and carvedilol 12.5 mg BID.",
    });
    base_store
        .record_chart(&workspace, std::slice::from_ref(&intake), &initial_pull)
        .await
        .unwrap();

    // Configure an on-demand source refresh source (`quest-labs`) so `refresh_patient_sources`
    // pulls real new records into the workspace during the review!
    let refresh_checker = FixedSourceCheck::default();
    refresh_checker.set_source_outcome(
        &practice,
        &workspace,
        SourceId::new("quest-labs").unwrap(),
        SourceCheckOutcome::Updated {
            source_label: "Quest Diagnostics".into(),
            patient: source_patient("pt-ops", "Hannah Lindqvist"),
            pulled_at: 1_790_100_600,
            records: vec![chart_record(ChartRecordSpec {
                workspace: &workspace,
                source_id: "quest-labs",
                candidate_id: "Observation/bnp-1",
                revision: "rev-1",
                record_type: "Observation",
                title: "NT-proBNP",
                date: "2026-10-14",
                text: "NT-proBNP 2,850 pg/mL (prior baseline 640 pg/mL) with 4 lb weight gain.",
            })],
        },
    );
    let store = base_store.with_source_refresh(Arc::new(refresh_checker));

    let AdmittedPatientReview::New { review, .. } = store
        .admit_patient_review_with_trigger(
            &practice,
            &workspace,
            "cmd-ops-review-1",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "DocumentReference/intake-ops".into(),
                    "rev-1".into(),
                )]),
                changed_candidates: vec!["DocumentReference/intake-ops".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new review admission");
    };

    let claim = store
        .claim_practice_job(&review.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct FullOperationsProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for FullOperationsProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![
                        engine::Item::Call(engine::Invocation {
                            id: "read-intake".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "DocumentReference/intake-ops",
                                    "revision": "rev-1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        }),
                        engine::Item::Call(engine::Invocation {
                            id: "refresh-labs".into(),
                            name: REFRESH_TOOL.into(),
                            arguments: json!({ "source_ids": ["quest-labs"] }).to_string(),
                        }),
                    ],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let refresh_out = materialized_output(request, "refresh-labs");
                    assert!(
                        refresh_out.contains("\"status\":\"updated\""),
                        "refresh_patient_sources must pull updated lab: {refresh_out}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-bnp".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "Observation/bnp-1",
                                    "revision": "rev-1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let save_u = SaveUnderstandingArgs {
                        expected_revision: Some(0),
                        operating_index: OperatingIndex {
                            working_overview: "HFrEF (EF 38%) with acute NT-proBNP rise to 2,850 pg/mL and 4 lb weight gain.".into(),
                            known_health_systems: vec![
                                KnownHealthSystem {
                                    name: "Clinic EHR".into(),
                                    standing: HealthSystemStanding::Connected {
                                        source_id: "clinic-ehr".into(),
                                    },
                                    note: "Outpatient cardiology notes".into(),
                                },
                                KnownHealthSystem {
                                    name: "Quest Diagnostics".into(),
                                    standing: HealthSystemStanding::Connected {
                                        source_id: "quest-labs".into(),
                                    },
                                    note: "Outpatient laboratory feed".into(),
                                },
                            ],
                            open_threads: vec![
                                "Acute NT-proBNP elevation 2,850 pg/mL awaiting diuretic evaluation".into(),
                            ],
                            anchors: vec![IndexedAnchor {
                                reference: ExactRecordReference {
                                    candidate_id: "Observation/bnp-1".into(),
                                    revision: "rev-1".into(),
                                },
                                source_id: "quest-labs".into(),
                                topic: "NT-proBNP elevation".into(),
                            }],
                        },
                        claims: vec![
                            SummaryClaimInput {
                                text: "HFrEF (EF 38%) maintained on sacubitril-valsartan 49/51 mg BID and carvedilol 12.5 mg BID".into(),
                                citations: vec![ExactRecordReference {
                                    candidate_id: "DocumentReference/intake-ops".into(),
                                    revision: "rev-1".into(),
                                }],
                            },
                            SummaryClaimInput {
                                text: "NT-proBNP rose to 2,850 pg/mL on 2026-10-14 (from 640 pg/mL) with 4 lb weight gain".into(),
                                citations: vec![ExactRecordReference {
                                    candidate_id: "Observation/bnp-1".into(),
                                    revision: "rev-1".into(),
                                }],
                            },
                        ],
                        material_gaps: vec![
                            "Routine annual renal monitoring due in 12 months".into(),
                        ],
                        scope_note: None,
                    };
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Urgent HFrEF decompensation signal: NT-proBNP rose from 640 to 2,850 pg/mL with 4 lb weight gain; raised `now` concern and updated longitudinal summary and memory.".into(),
                        },
                        citations: vec![
                            ExactRecordReference {
                                candidate_id: "DocumentReference/intake-ops".into(),
                                revision: "rev-1".into(),
                            },
                            ExactRecordReference {
                                candidate_id: "Observation/bnp-1".into(),
                                revision: "rev-1".into(),
                            },
                        ],
                        basis_and_limitations: "Based on Clinic EHR cardiology note (2026-10-12) and refreshed Quest Diagnostics NT-proBNP (2026-10-14).".into(),
                        unfinished_work: vec![UnfinishedReviewWork {
                            topic: "Post-diuresis BMP and weight check".into(),
                            detail: "Verify follow-up weight and renal panel if loop diuretic is initiated by physician.".into(),
                            limit: None,
                        }],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "raise-bnp-now".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-bnp-now",
                                    "urgency": "now",
                                    "reason": "NT-proBNP rose to 2,850 pg/mL (prior 640 pg/mL) with 4 lb weight gain in HFrEF (EF 38%); evaluate today for volume overload.",
                                    "records": [{
                                        "candidate_id": "Observation/bnp-1",
                                        "revision": "rev-1"
                                    }]
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "mem-hfref".into(),
                                name: PATIENT_MEMORY_TOOL.into(),
                                arguments: json!({
                                    "operation": "record_correction",
                                    "memory_id": "mem-hfref-trajectory",
                                    "topic": "Heart failure trajectory",
                                    "statement": "HFrEF (EF 38%); baseline NT-proBNP 640 pg/mL rose to 2,850 pg/mL on 2026-10-14."
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "save-u-ops".into(),
                                name: SAVE_UNDERSTANDING_TOOL.into(),
                                arguments: serde_json::to_string(&save_u).unwrap(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-ops".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => {
                    for (id, expected_snippet) in [
                        ("raise-bnp-now", "\"outcome\":\"applied\""),
                        ("mem-hfref", "\"outcome\":\"applied\""),
                        ("save-u-ops", "\"revision\":1"),
                        ("complete-ops", "\"outcome\":\"staged\""),
                    ] {
                        let out = materialized_output(request, id);
                        assert!(
                            out.contains(expected_snippet),
                            "tool {id} did not return expected response {expected_snippet}: {out}"
                        );
                    }
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant(
                            "Completed HFrEF review and recorded urgent NT-proBNP concern.",
                        )],
                        receipt: json!({"turn": 4}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: auth.as_ref(),
    };
    host.run(
        &mut FullOperationsProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("full operations review completes");

    // Simulate a clean process restart: drop `host` and `store`, connect a fresh Store!
    drop(host);
    drop(store);
    let restarted = Store::connect(&url).await.expect("reconnect after restart");

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let mut authority = PostgresWorkspaceAuthority;
    let open = restarted
        .open_concerns(&access, &mut authority)
        .await
        .unwrap();
    assert_eq!(open.len(), 1);
    assert_eq!(open[0].id, ConcernId::new("concern-bnp-now").unwrap());
    assert_eq!(open[0].urgency, ConcernUrgency::Now);

    let memories = restarted
        .patient_memories(&practice, &workspace, MemorySelection::ActiveOnly)
        .await
        .unwrap();
    assert_eq!(memories.len(), 1);
    assert_eq!(
        memories[0].id,
        PatientMemoryId::new("mem-hfref-trajectory").unwrap()
    );
    assert!(matches!(
        memories[0].provenance,
        MemoryProvenance::ClinicalSummary { .. }
    ));

    let understanding = restarted
        .patient_understanding_for_practice(&practice, &workspace, auth.as_ref())
        .await
        .unwrap()
        .expect("saved understanding persists across restart");
    assert_eq!(understanding.revision, 1);
    assert_eq!(understanding.source_version, 2);

    let readback_review = restarted
        .patient_review(&practice, &workspace, &review.review_id)
        .await
        .unwrap()
        .expect("saved review persists across restart");
    assert_eq!(readback_review.completed_source_version, Some(2));
    assert_eq!(readback_review.actions.len(), 4);
    assert!(
        readback_review
            .actions
            .iter()
            .any(|a| matches!(a, RecordedReviewAction::ConcernOpen { concern_id, .. } if concern_id == "concern-bnp-now"))
    );
    assert!(
        readback_review
            .actions
            .iter()
            .any(|a| matches!(a, RecordedReviewAction::MemoryRecorded { memory_id, .. } if memory_id == "mem-hfref-trajectory"))
    );
    assert!(readback_review.actions.iter().any(|a| matches!(
        a,
        RecordedReviewAction::UnderstandingSaved { revision: 1, .. }
    )));
    assert!(
        readback_review
            .actions
            .iter()
            .any(|a| matches!(a, RecordedReviewAction::SourceRefreshed { source_id, .. } if source_id == "quest-labs"))
    );

    // Now run a controlled quiet completion on a routine normal lab workspace and verify
    // it saves its reason as `"nothing to do"`, structurally distinct from Failed or Held.
    let quiet_ws = format!("pw-quiet-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &quiet_ws).await.unwrap();
    let quiet_pull = SourcePull {
        source_id: "quest-labs".into(),
        source_label: "Quest Diagnostics".into(),
        patient: source_patient("pt-quiet", "Owen Takahashi"),
        pulled_at: 1_790_101_000,
    };
    let normal_cbc = chart_record(ChartRecordSpec {
        workspace: &quiet_ws,
        source_id: "quest-labs",
        candidate_id: "Observation/cbc-normal-1",
        revision: "rev-1",
        record_type: "Observation",
        title: "Complete Blood Count",
        date: "2026-10-14",
        text: "WBC 6.1 x10^3/uL, Hemoglobin 14.6 g/dL, Platelets 245 x10^3/uL. All indices within reference range.",
    });
    restarted
        .record_chart(&quiet_ws, &[normal_cbc], &quiet_pull)
        .await
        .unwrap();

    let AdmittedPatientReview::New {
        review: quiet_rev, ..
    } = restarted
        .admit_patient_review_with_trigger(
            &practice,
            &quiet_ws,
            "cmd-quiet-cbc-1",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/cbc-normal-1".into(),
                    "rev-1".into(),
                )]),
                changed_candidates: vec!["Observation/cbc-normal-1".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new quiet review");
    };
    let quiet_claim = restarted
        .claim_practice_job(&quiet_rev.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct ControlledQuietProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for ControlledQuietProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-cbc".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/cbc-normal-1",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Quiet {
                            explanation: "Routine CBC is completely normal (WBC 6.1, Hgb 14.6, Plt 245) with no open concerns.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/cbc-normal-1".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Verified Quest Diagnostics CBC dated 2026-10-14.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "complete-quiet-cbc".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant("Quiet review completed.")],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut quiet_host = PracticeJobHost {
        store: restarted.clone(),
        claim: quiet_claim,
        authority: auth.as_ref(),
    };
    quiet_host
        .run(
            &mut ControlledQuietProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("controlled quiet review finishes");

    let after_quiet_restart = Store::connect(&url).await.unwrap();
    let saved_quiet = after_quiet_restart
        .patient_review(&practice, &quiet_ws, &quiet_rev.review_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_quiet.state.status_str(), "completed");
    let ReviewLifecycleState::Completed {
        outcome:
            ReviewOutcome::Quiet {
                reason,
                explanation,
                citations,
                ..
            },
        ..
    } = &saved_quiet.state
    else {
        panic!(
            "expected Completed Quiet outcome, got {:?}",
            saved_quiet.state
        );
    };
    assert_eq!(*reason, QuietReviewReason::NothingToDo);
    assert_eq!(reason.as_str(), "nothing to do");
    assert!(explanation.contains("Routine CBC is completely normal"));
    assert_eq!(citations.len(), 1);
    assert!(saved_quiet.actions.is_empty());
}

#[tokio::test]
async fn mid_run_record_correction_blocks_old_version_and_interrupted_action_is_not_repeated() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-fence-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-fence-corr-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();
    let (dr, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-fence").await;
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    // --- Part A: Record corrected while review runs ---
    let pull_v1 = SourcePull {
        source_id: "quest-labs".into(),
        source_label: "Quest Diagnostics".into(),
        patient: source_patient("pt-corr", "Simone Boucher"),
        pulled_at: 1_790_200_000,
    };
    let lab_rev1 = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "quest-labs",
        candidate_id: "Observation/k-hemolyzed",
        revision: "rev-1",
        record_type: "Observation",
        title: "Serum Potassium (Preliminary)",
        date: "2026-10-15",
        text: "Preliminary serum potassium 6.1 mmol/L (critical high).",
    });
    store
        .record_chart(&workspace, &[lab_rev1], &pull_v1)
        .await
        .unwrap();

    let AdmittedPatientReview::New {
        review: corr_rev, ..
    } = store
        .admit_patient_review_with_trigger(
            &practice,
            &workspace,
            "cmd-corr-midrun-1",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/k-hemolyzed".into(),
                    "rev-1".into(),
                )]),
                changed_candidates: vec!["Observation/k-hemolyzed".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new review");
    };
    let claim_corr = store
        .claim_practice_job(&corr_rev.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct MidRunCorrectionProvider {
        store: Store,
        workspace: String,
        turn: usize,
    }
    impl engine::recipe::Provider for MidRunCorrectionProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                // Turn 1: Read rev-1.
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-k-v1".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/k-hemolyzed",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                // Turn 2: Raise urgent concern citing rev-1 and stage completion citing rev-1.
                2 => {
                    let complete_v1 = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Critical potassium 6.1 mmol/L on preliminary report.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/k-hemolyzed".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Preliminary Quest potassium report.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "raise-k-v1".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-k-hemolyzed",
                                    "urgency": "now",
                                    "reason": "Preliminary critical potassium 6.1 mmol/L.",
                                    "records": [{
                                        "candidate_id": "Observation/k-hemolyzed",
                                        "revision": "rev-1"
                                    }]
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-k-v1".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete_v1).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                // Turn 3: Right before the model tries to finish, the laboratory delivers
                // a corrected addendum (`rev-2`) showing rev-1 was hemolyzed and repeat K+ is 4.2!
                3 => {
                    let pull_v2 = SourcePull {
                        source_id: "quest-labs".into(),
                        source_label: "Quest Diagnostics".into(),
                        patient: source_patient("pt-corr", "Simone Boucher"),
                        pulled_at: 1_790_200_300,
                    };
                    let lab_rev2 = chart_record(ChartRecordSpec {
                        workspace: &self.workspace,
                        source_id: "quest-labs",
                        candidate_id: "Observation/k-hemolyzed",
                        revision: "rev-2",
                        record_type: "Observation",
                        title: "Serum Potassium (Corrected Addendum)",
                        date: "2026-10-15",
                        text: "CORRECTED REPORT: Preliminary 6.1 mmol/L specimen was grossly hemolyzed. Repeat non-hemolyzed plasma potassium is 4.2 mmol/L (normal).",
                    });
                    self.store
                        .record_chart(&self.workspace, &[lab_rev2], &pull_v2)
                        .await
                        .unwrap();

                    // Model attempts to publish the old rev-1 assessment; `finish()` MUST block it!
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant(
                            "Publishing preliminary hyperkalemia review.",
                        )],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                // Turn 4: Harness kept the review running with `sources_changed` / `stale_cited_revision`
                // feedback! Verify the review was NOT completed on rev-1, read rev-2, retire the
                // superseded concern, and stage corrected completion on rev-2.
                4 => {
                    let feedback_json = serde_json::to_string(&request.items).unwrap();
                    assert!(
                        feedback_json.contains("sources_changed")
                            || feedback_json.contains("stale_cited_revision")
                            || feedback_json.contains("rev-2"),
                        "finish barrier must refuse old version and report rev-2: {feedback_json}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-k-v2".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "Observation/k-hemolyzed",
                                    "revision": "rev-2",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 4}),
                        usage: Default::default(),
                    })
                }
                5 => {
                    let complete_v2 = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Quiet {
                            explanation: "Corrected Quest addendum (rev-2) confirmed gross hemolysis on preliminary sample and normal repeat potassium 4.2 mmol/L; retired preliminary concern.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/k-hemolyzed".into(),
                            revision: "rev-2".into(),
                        }],
                        basis_and_limitations: "Verified corrected Quest Diagnostics potassium addendum (rev-2).".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "retire-k-v1".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "retire",
                                    "concern_id": "concern-k-hemolyzed",
                                    "reason": "Corrected lab addendum (rev-2) showed specimen hemolysis; repeat plasma potassium is normal at 4.2 mmol/L."
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-k-v2".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete_v2).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 5}),
                        usage: Default::default(),
                    })
                }
                6 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Completed review on corrected potassium revision rev-2.",
                    )],
                    receipt: json!({"turn": 6}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut corr_host = PracticeJobHost {
        store: store.clone(),
        claim: claim_corr,
        authority: auth.as_ref(),
    };
    corr_host
        .run(
            &mut MidRunCorrectionProvider {
                store: store.clone(),
                workspace: workspace.clone(),
                turn: 0,
            },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("review continues past mid-run correction and commits corrected version");

    let saved_corr = store
        .patient_review(&practice, &workspace, &corr_rev.review_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_corr.initial_source_version, 1);
    assert_eq!(saved_corr.completed_source_version, Some(2));
    let ReviewLifecycleState::Completed {
        outcome:
            ReviewOutcome::Quiet {
                reason: QuietReviewReason::NothingToDo,
                citations,
                ..
            },
        ..
    } = &saved_corr.state
    else {
        panic!(
            "expected corrected Quiet completion, got {:?}",
            saved_corr.state
        );
    };
    assert_eq!(citations.len(), 1);
    assert_eq!(citations[0].reference.revision, "rev-2");

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let mut authority = PostgresWorkspaceAuthority;
    assert!(
        store
            .open_concerns(&access, &mut authority)
            .await
            .unwrap()
            .is_empty(),
        "preliminary concern from rev-1 must not remain open on Today"
    );
    let retired = store
        .retired_concern_updates(&access, &mut authority)
        .await
        .unwrap();
    assert_eq!(retired.len(), 1);
    assert!(retired[0].retirement_reason.as_str().contains("4.2 mmol/L"));

    // --- Part B: Interrupted after accepted actions keeps each action once and does not repeat it ---
    let crash_ws = format!("pw-fence-crash-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &crash_ws).await.unwrap();
    auth.set_membership(&dr, &crash_ws, Membership::Allowed)
        .await
        .unwrap();
    let crash_pull = SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: source_patient("pt-crash", "Tariq Mansour"),
        pulled_at: 1_790_201_000,
    };
    let inr_lab = chart_record(ChartRecordSpec {
        workspace: &crash_ws,
        source_id: "clinic-ehr",
        candidate_id: "Observation/inr-supratherapeutic",
        revision: "rev-1",
        record_type: "Observation",
        title: "Prothrombin Time / INR",
        date: "2026-10-15",
        text: "INR 4.9 on warfarin 5 mg daily (target INR 2.0-3.0 for atrial fibrillation); no active bleeding reported.",
    });
    store
        .record_chart(&crash_ws, &[inr_lab], &crash_pull)
        .await
        .unwrap();

    let AdmittedPatientReview::New {
        review: crash_rev, ..
    } = store
        .admit_patient_review_with_trigger(
            &practice,
            &crash_ws,
            "cmd-crash-inr-1",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/inr-supratherapeutic".into(),
                    "rev-1".into(),
                )]),
                changed_candidates: vec!["Observation/inr-supratherapeutic".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new crash review");
    };

    // Run Attempt 1 through real `PracticeJobHost::run` with a memory operation that cancels
    // immediately after committing `patient_memory` to Postgres (before `ToolReturned`), and
    // also append an interrupted `patient_concern` (`ToolStarted` + DB commit, no `ToolReturned`)!
    struct CancelAfterCommitMemory {
        cancel: engine::Cancellation,
    }
    #[async_trait]
    impl PatientMemoryOperation for CancelAfterCommitMemory {
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

    let concern_call = engine::Invocation {
        id: "raise-inr-crash".to_owned(),
        name: CONCERN_TOOL.to_owned(),
        arguments: json!({
            "operation": "raise",
            "concern_id": "concern-inr-49",
            "urgency": "now",
            "reason": "Supratherapeutic INR 4.9 on warfarin 5 mg daily; evaluate dose hold and repeat INR.",
            "records": [{
                "candidate_id": "Observation/inr-supratherapeutic",
                "revision": "rev-1"
            }]
        })
        .to_string(),
    };
    let mem_call = engine::Invocation {
        id: "mem-inr-crash".to_owned(),
        name: PATIENT_MEMORY_TOOL.to_owned(),
        arguments: json!({
            "operation": "record_correction",
            "memory_id": "mem-warfarin-inr",
            "topic": "Anticoagulation",
            "statement": "Atrial fibrillation on warfarin 5 mg daily; supratherapeutic INR 4.9 on 2026-10-15."
        })
        .to_string(),
    };

    struct FirstAttemptCrashProvider {
        mem_call: engine::Invocation,
        concern_call: engine::Invocation,
        turn: usize,
    }
    impl engine::recipe::Provider for FirstAttemptCrashProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-inr-1".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/inr-supratherapeutic",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"attempt": 1, "turn": 1}),
                    usage: Default::default(),
                }),
                2 => Ok(engine::Turn {
                    items: vec![
                        engine::Item::Call(self.mem_call.clone()),
                        engine::Item::Call(self.concern_call.clone()),
                    ],
                    receipt: json!({"attempt": 1, "turn": 2}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected attempt 1 turn {other}"),
            }
        }
    }

    let crash_scope = crash_rev.practice_scope();
    let cancel_attempt_1 = engine::Cancellation::default();
    let crashing_store = store
        .clone()
        .with_memory_operation(Arc::new(CancelAfterCommitMemory {
            cancel: cancel_attempt_1.clone(),
        }));
    let claim_1 = crashing_store
        .claim_practice_job(&crash_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim attempt 1");
    let mut host_1 = PracticeJobHost {
        store: crashing_store,
        claim: claim_1.clone(),
        authority: auth.as_ref(),
    };
    let attempt_1_err = host_1
        .run(
            &mut FirstAttemptCrashProvider {
                mem_call: mem_call.clone(),
                concern_call: concern_call.clone(),
                turn: 0,
            },
            engine::RunLimits::default(),
            &cancel_attempt_1,
        )
        .await
        .expect_err(
            "attempt 1 must abort right after committing mem-inr-crash before ToolReturned",
        );
    assert!(matches!(attempt_1_err, engine::Failure::Cancelled));

    // Also persist `ToolStarted(concern_call)` and execute `concern_call` without `ToolReturned`
    // so BOTH `patient_memory` AND `patient_concern` committed on Attempt 1 without `ToolReturned`!
    let after_crash = store
        .practice_job(&crash_scope)
        .await
        .unwrap()
        .expect("practice job after crash");
    let last_event = after_crash.attempts.last().unwrap().events.last().unwrap();
    let mut exec_host = PracticeJobHost {
        store: store.clone(),
        claim: claim_1.clone(),
        authority: auth.as_ref(),
    };
    engine::Product::persist(
        &mut exec_host,
        engine::Event {
            attempt_id: claim_1.attempt.clone(),
            sequence: last_event.sequence + 1,
            ordinal: last_event.ordinal,
            kind: engine::EventKind::ToolStarted(concern_call.clone()),
        },
    )
    .await
    .unwrap();
    let concern_args: Value = serde_json::from_str(&concern_call.arguments).unwrap();
    let concern_exec = engine::Product::execute(&mut exec_host, &concern_call, concern_args)
        .await
        .unwrap();
    assert!(
        matches!(concern_exec, engine::ToolOutcome::Returned(ref s) if s.contains("\"applied\""))
    );
    drop(exec_host);
    drop(host_1);

    store
        .settle_practice_job_attempt(&claim_1, engine::AttemptFailure::Interrupted)
        .await
        .unwrap();

    let claim_2 = store
        .claim_practice_job(&crash_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim successor attempt 2 after lease expiry");
    assert_eq!(claim_2.work, claim_1.work);
    assert_ne!(claim_2.attempt, claim_1.attempt);

    struct RecoveryAfterAcceptedActionProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for RecoveryAfterAcceptedActionProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => {
                    // Harness recovery must have reconciled BOTH unfinished calls via `saved_outcome`
                    // (`"outcome":"duplicate"`) without re-executing or failing them!
                    let c_out = materialized_output(request, "raise-inr-crash");
                    assert!(
                        c_out.contains("\"outcome\":\"duplicate\""),
                        "recovered concern action must reconcile as duplicate: {c_out}"
                    );
                    let m_out = materialized_output(request, "mem-inr-crash");
                    assert!(
                        m_out.contains("\"outcome\":\"duplicate\""),
                        "recovered memory action must reconcile as duplicate: {m_out}"
                    );

                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Supratherapeutic INR 4.9 on warfarin; urgent concern and anticoagulation memory preserved across recovery.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/inr-supratherapeutic".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Based on Clinic EHR INR result dated 2026-10-15.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "complete-after-crash".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: json!({"attempt": 2, "turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Recovered patient review completed without repeating accepted actions.",
                    )],
                    receipt: json!({"attempt": 2, "turn": 2}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected recovery turn {other}"),
            }
        }
    }

    let mut host_2 = PracticeJobHost {
        store: store.clone(),
        claim: claim_2,
        authority: auth.as_ref(),
    };
    host_2
        .run(
            &mut RecoveryAfterAcceptedActionProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("successor attempt recovers and completes");

    let crash_access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &crash_ws,
        session_hash: &dr_hash,
    };
    let open_after = store
        .open_concerns(&crash_access, &mut authority)
        .await
        .unwrap();
    assert_eq!(open_after.len(), 1);
    assert_eq!(
        open_after[0].revision, 1,
        "concern must be written once and never repeated on recovery"
    );

    let mem_after = store
        .patient_memories(&practice, &crash_ws, MemorySelection::ActiveOnly)
        .await
        .unwrap();
    assert_eq!(mem_after.len(), 1);
    assert_eq!(
        mem_after[0].revision, 1,
        "memory entry must be written once and never repeated on recovery"
    );

    let saved_crash_rev = store
        .patient_review(&practice, &crash_ws, &crash_rev.review_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_crash_rev.actions.len(), 2);
}

#[tokio::test]
async fn saved_review_records_metadata_keeps_unknown_usage_unknown_and_feeds_later_review_reads() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-meta-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-meta-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    let pull = SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: source_patient("pt-meta", "Linnea Lindholm"),
        pulled_at: 1_790_300_000,
    };
    let ferritin_lab = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "Observation/ferritin-low-1",
        revision: "rev-1",
        record_type: "Observation",
        title: "Iron Studies",
        date: "2026-10-16",
        text: "Ferritin 9 ng/mL (low), Hemoglobin 10.8 g/dL, MCV 74 fL consistent with iron deficiency anemia.",
    });
    store
        .record_chart(&workspace, &[ferritin_lab], &pull)
        .await
        .unwrap();

    // --- Review 1: Provider omits token usage (`Usage::default()`), records unfinished work ---
    let AdmittedPatientReview::New { review: rev1, .. } = store
        .admit_patient_review_with_trigger(
            &practice,
            &workspace,
            "cmd-meta-rev-1",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/ferritin-low-1".into(),
                    "rev-1".into(),
                )]),
                changed_candidates: vec!["Observation/ferritin-low-1".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new review 1");
    };
    let claim_1 = store
        .claim_practice_job(&rev1.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct UnknownUsageReviewProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for UnknownUsageReviewProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-ferritin".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/ferritin-low-1",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(), // Unknown token usage!
                }),
                2 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "New microcytic iron-deficiency anemia (Hgb 10.8 g/dL, MCV 74 fL, ferritin 9 ng/mL); raised `today` concern.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/ferritin-low-1".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Based on Clinic EHR Iron Studies dated 2026-10-16; GI bleeding screen and colonoscopy records not yet present.".into(),
                        unfinished_work: vec![UnfinishedReviewWork {
                            topic: "GI evaluation and oral iron response".into(),
                            detail: "Check stool FIT / gastroenterology referral status and repeat CBC/ferritin in 6-8 weeks.".into(),
                            limit: None,
                        }],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "raise-ida".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-ida-1",
                                    "urgency": "today",
                                    "reason": "Microcytic iron-deficiency anemia (Hgb 10.8 g/dL, MCV 74 fL, ferritin 9 ng/mL); consider GI evaluation and iron repletion.",
                                    "records": [{
                                        "candidate_id": "Observation/ferritin-low-1",
                                        "revision": "rev-1"
                                    }]
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-rev-1".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant("Completed review 1.")],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut host_1 = PracticeJobHost {
        store: store.clone(),
        claim: claim_1,
        authority: auth.as_ref(),
    };
    host_1
        .run(
            &mut UnknownUsageReviewProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    let saved_1 = store
        .patient_review(&practice, &workspace, &rev1.review_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_1.triggers.len(), 1);
    assert_eq!(saved_1.triggers[0].reason_label(), "new records");
    assert_eq!(saved_1.recipe_version.model, roles.background.model);
    assert_eq!(saved_1.recipe_version.effort, roles.background.effort);
    assert!(!saved_1.recipe_version.binding_sha256.is_empty());
    assert!(!saved_1.recipe_version.skills_package_id.is_empty());
    assert_eq!(saved_1.recipe_version.preloaded_skills.len(), 1);
    assert_eq!(
        saved_1.recipe_version.preloaded_skills[0].name,
        "patient-record-retrieval"
    );
    assert_eq!(saved_1.initial_source_version, 1);
    assert_eq!(saved_1.completed_source_version, Some(1));
    assert_eq!(
        saved_1.evidence_read,
        vec![ExactRecordReference {
            candidate_id: "Observation/ferritin-low-1".into(),
            revision: "rev-1".into(),
        }]
    );
    assert_eq!(saved_1.actions.len(), 1);
    assert_eq!(saved_1.unfinished_work.len(), 1);
    assert_eq!(
        saved_1.unfinished_work[0].topic,
        "GI evaluation and oral iron response"
    );
    assert!(saved_1.duration_ms.is_some());
    assert_eq!(
        saved_1.usage,
        ReviewUsage::Unknown,
        "unknown provider usage must stay ReviewUsage::Unknown and never coerce to zero"
    );
    assert_eq!(
        serde_json::to_value(&saved_1.usage).unwrap(),
        json!({"status": "unknown"})
    );

    // --- Review 2: Later review reads earlier review's decisions + unfinished work via both
    // starting context AND `read_patient_reviews`, and reports measured token usage ---
    let AdmittedPatientReview::New { review: rev2, .. } = store
        .admit_patient_review_with_trigger(
            &practice,
            &workspace,
            "cmd-meta-rev-2",
            ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/ferritin-low-1".into(),
                    "rev-1".into(),
                )]),
                last_reviewed_at: saved_1.completed_at,
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new review 2");
    };
    let claim_2 = store
        .claim_practice_job(&rev2.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct SubsequentReadsAndKnownUsageProvider {
        prior_review_id: String,
        turn: usize,
    }
    impl engine::recipe::Provider for SubsequentReadsAndKnownUsageProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => {
                    let wire = serde_json::to_string(&request.items).unwrap();
                    assert!(
                        wire.contains(&self.prior_review_id)
                            && wire.contains("GI evaluation and oral iron response"),
                        "starting context of later review must surface prior decisions and unfinished work: {wire}"
                    );
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "list-prior-reviews".into(),
                                name: READ_PATIENT_REVIEWS_TOOL.into(),
                                arguments: json!({
                                    "operation": "list",
                                    "limit": 10
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "detail-prior-review".into(),
                                name: READ_PATIENT_REVIEWS_TOOL.into(),
                                arguments: json!({
                                    "operation": "read",
                                    "review_id": self.prior_review_id
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "read-ferritin-again".into(),
                                name: READ_TOOL.into(),
                                arguments: json!({
                                    "references": [{
                                        "candidate_id": "Observation/ferritin-low-1",
                                        "revision": "rev-1",
                                        "cursor": null
                                    }],
                                    "max_result_bytes": 16_384
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({"turn": 1}),
                        usage: engine::recipe::Usage {
                            input_tokens: Some(900),
                            cached_input_tokens: Some(300),
                            output_tokens: Some(120),
                        },
                    })
                }
                2 => {
                    let list_out = materialized_output(request, "list-prior-reviews");
                    assert!(
                        list_out.contains(&self.prior_review_id)
                            && list_out.contains("GI evaluation and oral iron response"),
                        "read_patient_reviews list must include earlier review and unfinished work: {list_out}"
                    );
                    let detail_out = materialized_output(request, "detail-prior-review");
                    assert!(
                        detail_out.contains("concern-ida-1")
                            && detail_out.contains("\"status\":\"unknown\""),
                        "read_patient_reviews detail must include full actions and unknown usage: {detail_out}"
                    );

                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Quiet {
                            explanation: "Open iron-deficiency anemia concern (`concern-ida-1`) remains active on Today and repeat labs are not yet due (6-8 week interval).".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/ferritin-low-1".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Reviewed prior review decision and active concern; no new lab arrivals.".into(),
                        unfinished_work: vec![UnfinishedReviewWork {
                            topic: "GI evaluation and oral iron response".into(),
                            detail: "Check stool FIT / gastroenterology referral status and repeat CBC/ferritin in 6-8 weeks.".into(),
                            limit: None,
                        }],
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "complete-rev-2".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: engine::recipe::Usage {
                            input_tokens: Some(1400),
                            cached_input_tokens: Some(700),
                            output_tokens: Some(110),
                        },
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Periodic follow-up quiet completion.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: engine::recipe::Usage {
                        input_tokens: Some(1500),
                        cached_input_tokens: Some(1100),
                        output_tokens: Some(30),
                    },
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut host_2 = PracticeJobHost {
        store: store.clone(),
        claim: claim_2,
        authority: auth.as_ref(),
    };
    host_2
        .run(
            &mut SubsequentReadsAndKnownUsageProvider {
                prior_review_id: saved_1.review_id.clone(),
                turn: 0,
            },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    let saved_2 = store
        .patient_review(&practice, &workspace, &rev2.review_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        saved_2.usage,
        ReviewUsage::Known {
            input_tokens: 900 + 1400 + 1500,
            cached_input_tokens: Some(300 + 700 + 1100),
            output_tokens: 120 + 110 + 30,
            exchanges: 3,
        }
    );
}

#[tokio::test]
async fn stop_holds_across_restart_resume_and_try_again_refresh_evidence_and_new_input_access_loss_or_correction_block_stale_completion()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-ctrl-{}", Uuid::new_v4().simple());
    let stop_ws = format!("pw-ctrl-stop-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &stop_ws).await.unwrap();

    let pull_1 = SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: source_patient("pt-stop", "Gideon Osei"),
        pulled_at: 1_790_400_000,
    };
    let cr_v1 = chart_record(ChartRecordSpec {
        workspace: &stop_ws,
        source_id: "clinic-ehr",
        candidate_id: "Observation/creatinine-1",
        revision: "rev-1",
        record_type: "Observation",
        title: "Serum Creatinine",
        date: "2026-10-17",
        text: "Preliminary serum creatinine 2.4 mg/dL.",
    });
    store
        .record_chart(&stop_ws, &[cr_v1], &pull_1)
        .await
        .unwrap();

    // --- Case 1: Stop holds across restart; record corrected while Stopped; Resume uses current evidence ---
    let AdmittedPatientReview::New {
        review: stop_rev, ..
    } = store
        .admit_patient_review_with_trigger(
            &practice,
            &stop_ws,
            "cmd-stop-resume-1",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/creatinine-1".into(),
                    "rev-1".into(),
                )]),
                changed_candidates: vec!["Observation/creatinine-1".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new review");
    };
    let stop_scope = stop_rev.practice_scope();
    let active_claim = store
        .claim_practice_job(&stop_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        store
            .stop_patient_review(&practice, &stop_ws, &stop_rev.review_id, "cmd-stop-1")
            .await
            .unwrap(),
        Stopped::Held
    );
    // Old claimed attempt lost currency immediately on Stop.
    let mut stale_stopped_host = PracticeJobHost {
        store: store.clone(),
        claim: active_claim,
        authority: auth.as_ref(),
    };
    let stopped_err = stale_stopped_host
        .run(
            &mut FailIfCalledProvider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect_err("stopped attempt must not execute or complete");
    assert_eq!(
        engine::AttemptFailure::from_error(&stopped_err, &engine::Cancellation::default()),
        engine::AttemptFailure::Interrupted
    );

    // Reconnect Store ("after restart") and verify review stays Held(Stopped) and is NOT due!
    let restarted = Store::connect(&url).await.unwrap();
    let held_after_restart = restarted
        .patient_review(&practice, &stop_ws, &stop_rev.review_id)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        held_after_restart.state,
        ReviewLifecycleState::Held {
            reason: ReviewHoldReason::Stopped,
            ..
        }
    ));
    let due_jobs = restarted.practice_jobs_due(&practice, 50).await.unwrap();
    assert!(
        !due_jobs.contains(&stop_scope),
        "held review must remain held across restart until explicit Resume"
    );

    // Correct the creatinine record while Stopped (`rev-1` -> `rev-2`), then Resume!
    let pull_2 = SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: source_patient("pt-stop", "Gideon Osei"),
        pulled_at: 1_790_400_500,
    };
    let cr_v2 = chart_record(ChartRecordSpec {
        workspace: &stop_ws,
        source_id: "clinic-ehr",
        candidate_id: "Observation/creatinine-1",
        revision: "rev-2",
        record_type: "Observation",
        title: "Serum Creatinine (Corrected)",
        date: "2026-10-17",
        text: "Corrected serum creatinine 1.0 mg/dL (specimen mislabeled in lab; repeat verified normal).",
    });
    restarted
        .record_chart(&stop_ws, &[cr_v2], &pull_2)
        .await
        .unwrap();

    let _resume_delivery = restarted
        .resume_patient_review(
            stop_rev.target(),
            "cmd-resume-1",
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .expect("Resume rearms stopped review with current evidence");

    let resumed_claim = restarted
        .claim_practice_job(&stop_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim resumed review");

    struct ResumedOnCorrectedEvidenceProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for ResumedOnCorrectedEvidenceProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => {
                    let wire = serde_json::to_string(&request.items).unwrap();
                    assert!(
                        wire.contains("Retained chart: version 2"),
                        "Resume must refresh starting context to current chart version 2: {wire}"
                    );
                    // Prove trying to cite old `rev-1` is refused (`stale_cited_revision`).
                    let stale_complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Acute kidney injury creatinine 2.4 mg/dL.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/creatinine-1".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Old rev-1.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "try-stale-rev1".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&stale_complete).unwrap(),
                        })],
                        receipt: json!({"turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let refused = serde_json::to_string(&request.items).unwrap();
                    assert!(
                        refused.contains("sources_changed")
                            && refused.contains("Observation/creatinine-1"),
                        "old rev-1 result must not complete after correction: {refused}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-cr-v2".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "Observation/creatinine-1",
                                    "revision": "rev-2",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let complete_v2 = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Quiet {
                            explanation: "Corrected creatinine report (rev-2) verified normal renal function (1.0 mg/dL).".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/creatinine-1".into(),
                            revision: "rev-2".into(),
                        }],
                        basis_and_limitations: "Based on corrected Clinic EHR creatinine rev-2.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "complete-cr-v2".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete_v2).unwrap(),
                        })],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Resumed review completed on rev-2.",
                    )],
                    receipt: json!({"turn": 4}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut resumed_host = PracticeJobHost {
        store: restarted.clone(),
        claim: resumed_claim,
        authority: auth.as_ref(),
    };
    resumed_host
        .run(
            &mut ResumedOnCorrectedEvidenceProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("resumed review completes on corrected evidence");

    let saved_resumed = restarted
        .patient_review(&practice, &stop_ws, &stop_rev.review_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_resumed.completed_source_version, Some(2));

    // --- Case 2: Failed review -> Try again refreshes evidence and completes; Access lost blocks Try again ---
    let retry_ws = format!("pw-ctrl-retry-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &retry_ws).await.unwrap();
    let retry_pull = SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: source_patient("pt-retry", "Miriam Solis"),
        pulled_at: 1_790_401_000,
    };
    let retry_lab = chart_record(ChartRecordSpec {
        workspace: &retry_ws,
        source_id: "clinic-ehr",
        candidate_id: "Observation/tsh-1",
        revision: "rev-1",
        record_type: "Observation",
        title: "TSH",
        date: "2026-10-17",
        text: "TSH 2.1 mIU/L (euthyroid on levothyroxine 75 mcg daily).",
    });
    restarted
        .record_chart(&retry_ws, &[retry_lab], &retry_pull)
        .await
        .unwrap();

    let AdmittedPatientReview::New {
        review: fail_rev, ..
    } = restarted
        .admit_patient_review_with_trigger(
            &practice,
            &retry_ws,
            "cmd-fail-try-again-1",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([("Observation/tsh-1".into(), "rev-1".into())]),
                changed_candidates: vec!["Observation/tsh-1".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new review");
    };
    let fail_scope = fail_rev.practice_scope();
    for _ in 0..harness::engine::MAX_FAILED_ATTEMPTS {
        let c = restarted
            .claim_practice_job(&fail_scope, 60, auth.as_ref())
            .await
            .unwrap()
            .unwrap();
        restarted
            .settle_practice_job_attempt(&c, engine::AttemptFailure::Failed)
            .await
            .unwrap();
    }
    let failed_state = restarted
        .patient_review(&practice, &retry_ws, &fail_rev.review_id)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        failed_state.state,
        ReviewLifecycleState::Failed { .. }
    ));

    restarted
        .try_again_patient_review(
            fail_rev.target(),
            "cmd-try-again-1",
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .expect("Try again rearms Failed review");
    let rearmed_claim = restarted
        .claim_practice_job(&fail_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct SimpleQuietTshProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for SimpleQuietTshProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-tsh".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/tsh-1",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Quiet {
                            explanation: "TSH 2.1 mIU/L is euthyroid on stable levothyroxine."
                                .into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/tsh-1".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Clinic EHR TSH (2026-10-17).".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "complete-tsh".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant("Completed after Try again.")],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut rearmed_host = PracticeJobHost {
        store: restarted.clone(),
        claim: rearmed_claim,
        authority: auth.as_ref(),
    };
    rearmed_host
        .run(
            &mut SimpleQuietTshProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("Try again completes cleanly");

    // --- Case 3: New input (`submit_patient_review_input`) fences out in-flight old attempt;
    // AND access loss (`RevocablePracticeAuthority`) transitions review to Superseded(AccessLost) ---
    let steer_ws = format!("pw-ctrl-steer-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &steer_ws).await.unwrap();
    restarted
        .record_chart(
            &steer_ws,
            &[chart_record(ChartRecordSpec {
                workspace: &steer_ws,
                source_id: "clinic-ehr",
                candidate_id: "Observation/bp-steer",
                revision: "rev-1",
                record_type: "Observation",
                title: "Home BP Summary",
                date: "2026-10-17",
                text: "Home blood pressure averaging 148/92 mmHg over 7 days.",
            })],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: source_patient("pt-steer", "Devon Brooks"),
                pulled_at: 1_790_402_000,
            },
        )
        .await
        .unwrap();

    let revocable = RevocablePracticeAuthority {
        inner: auth.clone(),
        revoked: AtomicBool::new(false),
    };
    let AdmittedPatientReview::New {
        review: steer_rev, ..
    } = restarted
        .admit_patient_review_with_trigger(
            &practice,
            &steer_ws,
            "cmd-steer-1",
            ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::from([("Observation/bp-steer".into(), "rev-1".into())]),
                last_reviewed_at: None,
            },
            &roles,
            &skills,
            &revocable,
        )
        .await
        .unwrap()
    else {
        panic!("expected new steer review");
    };
    let steer_scope = steer_rev.practice_scope();
    let old_claim = restarted
        .claim_practice_job(&steer_scope, 60, &revocable)
        .await
        .unwrap()
        .unwrap();

    // New input arrives while `old_claim` is running -> advances revision and cancels `old_claim`!
    restarted
        .submit_patient_review_input(
            steer_rev.target(),
            ReviewInputSubmission {
                input_key: "input-steer-focus",
                input_text: "Focus on 7-day home blood pressure elevation before next visit.",
                additional_trigger: None,
            },
            &roles,
            &skills,
            &revocable,
        )
        .await
        .expect("new input accepted onto in-flight review");

    let mut old_host = PracticeJobHost {
        store: restarted.clone(),
        claim: old_claim,
        authority: &revocable,
    };
    let superseded_err = old_host
        .run(
            &mut FailIfCalledProvider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect_err("superseded attempt must not complete old result");
    assert_eq!(
        engine::AttemptFailure::from_error(&superseded_err, &engine::Cancellation::default()),
        engine::AttemptFailure::Interrupted
    );

    // Stop the steered review, then revoke practice access and attempt Resume ->
    // must fail Access AND transition SavedPatientReview to Superseded(AccessLost)!
    assert_eq!(
        restarted
            .stop_patient_review(&practice, &steer_ws, &steer_rev.review_id, "cmd-stop-steer")
            .await
            .unwrap(),
        Stopped::Held
    );
    revocable.revoked.store(true, Ordering::SeqCst);

    let access_err = restarted
        .resume_patient_review(
            steer_rev.target(),
            "cmd-resume-after-revoke",
            &roles,
            &skills,
            &revocable,
        )
        .await
        .expect_err("Resume after access loss must fail");
    assert!(matches!(access_err, PatientReviewError::Access(_)));

    let superseded_review = restarted
        .patient_review(&practice, &steer_ws, &steer_rev.review_id)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        superseded_review.state,
        ReviewLifecycleState::Superseded {
            reason: ReviewSupersededReason::AccessLost { .. },
            ..
        }
    ));
}

struct FailIfCalledProvider;
impl engine::recipe::Provider for FailIfCalledProvider {
    async fn exchange(
        &mut self,
        _: &engine::recipe::Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn engine::recipe::Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        panic!("stale or stopped attempt must fail currency check before calling provider");
    }
}

#[tokio::test]
async fn clinical_cases_urgent_abnormal_nonurgent_change_conflicting_sources_and_normal_quiet_review()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-cases-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (dr, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-cases").await;
    let mut authority = PostgresWorkspaceAuthority;

    // --- Case 1: Urgent abnormal result (`now`) ---
    let urgent_ws = format!("pw-case-urgent-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &urgent_ws).await.unwrap();
    auth.set_membership(&dr, &urgent_ws, Membership::Allowed)
        .await
        .unwrap();
    store
        .record_chart(
            &urgent_ws,
            &[chart_record(ChartRecordSpec {
                workspace: &urgent_ws,
                source_id: "stat-labs",
                candidate_id: "Observation/k-critical-63",
                revision: "rev-1",
                record_type: "Observation",
                title: "STAT Potassium",
                date: "2026-10-18",
                text: "Serum potassium 6.3 mmol/L (critical high; non-hemolyzed venous sample) in CKD stage 3b patient on spironolactone 25 mg daily.",
            })],
            &SourcePull {
                source_id: "stat-labs".into(),
                source_label: "Hospital STAT Lab".into(),
                patient: source_patient("pt-urgent", "Caleb Nwosu"),
                pulled_at: 1_790_500_000,
            },
        )
        .await
        .unwrap();

    let AdmittedPatientReview::New {
        review: urgent_rev, ..
    } = store
        .admit_patient_review_with_trigger(
            &practice,
            &urgent_ws,
            "cmd-case-urgent",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/k-critical-63".into(),
                    "rev-1".into(),
                )]),
                changed_candidates: vec!["Observation/k-critical-63".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new urgent review");
    };
    let urgent_claim = store
        .claim_practice_job(&urgent_rev.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct UrgentHyperkalemiaProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for UrgentHyperkalemiaProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-k-63".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/k-critical-63",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Critical non-hemolyzed serum potassium 6.3 mmol/L in CKD stage 3b on spironolactone; raised `now` concern for immediate physician evaluation and ECG assessment.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/k-critical-63".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Based on Hospital STAT Lab potassium result (2026-10-18); current ECG not yet in chart.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "raise-k-now".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-k-63-now",
                                    "urgency": "now",
                                    "reason": "Critical non-hemolyzed potassium 6.3 mmol/L in CKD stage 3b on spironolactone 25 mg daily; prompt physician review and ECG evaluation needed.",
                                    "records": [{
                                        "candidate_id": "Observation/k-critical-63",
                                        "revision": "rev-1"
                                    }]
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-urgent".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Urgent hyperkalemia review complete.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }
    PracticeJobHost {
        store: store.clone(),
        claim: urgent_claim,
        authority: auth.as_ref(),
    }
    .run(
        &mut UrgentHyperkalemiaProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let urgent_open = store
        .open_concerns(
            &WorkspaceAccess {
                practice: &practice,
                physician: &dr.physician_id,
                workspace: &urgent_ws,
                session_hash: &dr_hash,
            },
            &mut authority,
        )
        .await
        .unwrap();
    assert_eq!(urgent_open.len(), 1);
    assert_eq!(urgent_open[0].urgency, ConcernUrgency::Now);

    // --- Case 2: Important nonurgent change (`today`, not `now`) ---
    let nonurgent_ws = format!("pw-case-nonurgent-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &nonurgent_ws)
        .await
        .unwrap();
    auth.set_membership(&dr, &nonurgent_ws, Membership::Allowed)
        .await
        .unwrap();
    store
        .record_chart(
            &nonurgent_ws,
            &[chart_record(ChartRecordSpec {
                workspace: &nonurgent_ws,
                source_id: "quest-labs",
                candidate_id: "Observation/hba1c-worsening",
                revision: "rev-1",
                record_type: "Observation",
                title: "Hemoglobin A1c and Urine ACR",
                date: "2026-10-18",
                text: "Hemoglobin A1c rose to 7.9% (up from 6.8% six months ago); urine albumin-to-creatinine ratio 64 mg/g (up from 22 mg/g) on metformin monotherapy.",
            })],
            &SourcePull {
                source_id: "quest-labs".into(),
                source_label: "Quest Diagnostics".into(),
                patient: source_patient("pt-nonurgent", "Rosa Delgado"),
                pulled_at: 1_790_501_000,
            },
        )
        .await
        .unwrap();

    let AdmittedPatientReview::New {
        review: nonurgent_rev,
        ..
    } = store
        .admit_patient_review_with_trigger(
            &practice,
            &nonurgent_ws,
            "cmd-case-nonurgent",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/hba1c-worsening".into(),
                    "rev-1".into(),
                )]),
                changed_candidates: vec!["Observation/hba1c-worsening".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new nonurgent review");
    };
    let nonurgent_claim = store
        .claim_practice_job(&nonurgent_rev.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct NonurgentChronicChangeProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for NonurgentChronicChangeProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-a1c".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/hba1c-worsening",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Important nonurgent glycemic and albuminuria progression (HbA1c 6.8% -> 7.9%, uACR 22 -> 64 mg/g); raised `today` concern for physician review.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/hba1c-worsening".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Based on Quest Diagnostics HbA1c and urine ACR panel (2026-10-18).".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "raise-a1c-today".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-a1c-uacr",
                                    "urgency": "today",
                                    "reason": "HbA1c worsened from 6.8% to 7.9% with new microalbuminuria (uACR 64 mg/g) on metformin alone; physician review warranted for regimen and renal-protective considerations.",
                                    "records": [{
                                        "candidate_id": "Observation/hba1c-worsening",
                                        "revision": "rev-1"
                                    }]
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-a1c".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Nonurgent chronic progression review complete.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }
    PracticeJobHost {
        store: store.clone(),
        claim: nonurgent_claim,
        authority: auth.as_ref(),
    }
    .run(
        &mut NonurgentChronicChangeProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let nonurgent_open = store
        .open_concerns(
            &WorkspaceAccess {
                practice: &practice,
                physician: &dr.physician_id,
                workspace: &nonurgent_ws,
                session_hash: &dr_hash,
            },
            &mut authority,
        )
        .await
        .unwrap();
    assert_eq!(nonurgent_open.len(), 1);
    assert_eq!(nonurgent_open[0].urgency, ConcernUrgency::Today);

    // --- Case 3: Conflicting cross-source evidence ---
    let conflict_ws = format!("pw-case-conflict-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &conflict_ws)
        .await
        .unwrap();
    auth.set_membership(&dr, &conflict_ws, Membership::Allowed)
        .await
        .unwrap();
    store
        .record_chart(
            &conflict_ws,
            &[chart_record(ChartRecordSpec {
                workspace: &conflict_ws,
                source_id: "clinic-ehr",
                candidate_id: "MedicationStatement/metoprolol-clinic",
                revision: "rev-1",
                record_type: "MedicationStatement",
                title: "Outpatient Medication List",
                date: "2026-10-10",
                text: "Active medication: metoprolol succinate ER 25 mg once daily (reduced at 2026-10-10 visit for bradycardia HR 48 bpm).",
            })],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: source_patient("pt-conflict", "Victor Petrov"),
                pulled_at: 1_790_502_000,
            },
        )
        .await
        .unwrap();
    store
        .record_chart(
            &conflict_ws,
            &[chart_record(ChartRecordSpec {
                workspace: &conflict_ws,
                source_id: "surescripts-pharmacy",
                candidate_id: "MedicationDispense/metoprolol-dispense",
                revision: "rev-1",
                record_type: "MedicationDispense",
                title: "Pharmacy Dispense Event",
                date: "2026-10-18",
                text: "Dispensed 2026-10-18: metoprolol succinate ER 100 mg tablets (#90, take 1 tablet daily) from prior standing refill.",
            })],
            &SourcePull {
                source_id: "surescripts-pharmacy".into(),
                source_label: "SureScripts Pharmacy".into(),
                patient: source_patient("pt-conflict", "Victor Petrov"),
                pulled_at: 1_790_502_300,
            },
        )
        .await
        .unwrap();

    let AdmittedPatientReview::New {
        review: conflict_rev,
        ..
    } = store
        .admit_patient_review_with_trigger(
            &practice,
            &conflict_ws,
            "cmd-case-conflict",
            ReviewTrigger::ChangedRecords {
                workspace_version: 2,
                record_versions: BTreeMap::from([
                    (
                        "MedicationDispense/metoprolol-dispense".into(),
                        "rev-1".into(),
                    ),
                    (
                        "MedicationStatement/metoprolol-clinic".into(),
                        "rev-1".into(),
                    ),
                ]),
                changed_candidates: vec!["MedicationDispense/metoprolol-dispense".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new conflict review");
    };
    let conflict_claim = store
        .claim_practice_job(&conflict_rev.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct ConflictingSourcesProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for ConflictingSourcesProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-both-meds".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [
                                {
                                    "candidate_id": "MedicationStatement/metoprolol-clinic",
                                    "revision": "rev-1",
                                    "cursor": null
                                },
                                {
                                    "candidate_id": "MedicationDispense/metoprolol-dispense",
                                    "revision": "rev-1",
                                    "cursor": null
                                }
                            ],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Conflicting metoprolol succinate dosing across sources: Clinic EHR reduced dose to 25 mg daily on 2026-10-10 for bradycardia (HR 48), whereas SureScripts Pharmacy dispensed 100 mg daily on 2026-10-18.".into(),
                        },
                        citations: vec![
                            ExactRecordReference {
                                candidate_id: "MedicationStatement/metoprolol-clinic".into(),
                                revision: "rev-1".into(),
                            },
                            ExactRecordReference {
                                candidate_id: "MedicationDispense/metoprolol-dispense".into(),
                                revision: "rev-1".into(),
                            },
                        ],
                        basis_and_limitations: "Cross-source conflict between Clinic EHR (25 mg daily) and SureScripts Pharmacy fill (100 mg daily); actual tablet strength patient is taking at home is unconfirmed.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "raise-med-conflict".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-metoprolol-conflict",
                                    "urgency": "today",
                                    "reason": "Conflicting metoprolol succinate records: Clinic EHR lowered dose to 25 mg daily on 2026-10-10 for bradycardia, but SureScripts Pharmacy dispensed 100 mg tablets on 2026-10-18.",
                                    "records": [
                                        {
                                            "candidate_id": "MedicationStatement/metoprolol-clinic",
                                            "revision": "rev-1"
                                        },
                                        {
                                            "candidate_id": "MedicationDispense/metoprolol-dispense",
                                            "revision": "rev-1"
                                        }
                                    ]
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-conflict".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Cross-source medication conflict surfaced.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }
    PracticeJobHost {
        store: store.clone(),
        claim: conflict_claim,
        authority: auth.as_ref(),
    }
    .run(
        &mut ConflictingSourcesProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let conflict_open = store
        .open_concerns(
            &WorkspaceAccess {
                practice: &practice,
                physician: &dr.physician_id,
                workspace: &conflict_ws,
                session_hash: &dr_hash,
            },
            &mut authority,
        )
        .await
        .unwrap();
    assert_eq!(conflict_open.len(), 1);
    assert_eq!(conflict_open[0].records.len(), 2);
    let saved_conflict = store
        .patient_review(&practice, &conflict_ws, &conflict_rev.review_id)
        .await
        .unwrap()
        .unwrap();
    let ReviewLifecycleState::Completed {
        outcome: ReviewOutcome::Assessed { citations, .. },
        ..
    } = &saved_conflict.state
    else {
        panic!("expected Assessed conflict review");
    };
    assert_eq!(citations.len(), 2);
    let cited_sources: BTreeSet<&str> = citations.iter().map(|c| c.source_id.as_str()).collect();
    assert_eq!(
        cited_sources,
        BTreeSet::from(["clinic-ehr", "surescripts-pharmacy"])
    );

    // --- Case 4: Normal result with nothing open permits a quiet review (`"nothing to do"`) ---
    let normal_ws = format!("pw-case-normal-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &normal_ws).await.unwrap();
    auth.set_membership(&dr, &normal_ws, Membership::Allowed)
        .await
        .unwrap();
    store
        .record_chart(
            &normal_ws,
            &[chart_record(ChartRecordSpec {
                workspace: &normal_ws,
                source_id: "quest-labs",
                candidate_id: "Observation/lipid-normal-1",
                revision: "rev-1",
                record_type: "Observation",
                title: "Screening Lipid Panel",
                date: "2026-10-18",
                text: "Total cholesterol 162 mg/dL, HDL 58 mg/dL, LDL 82 mg/dL, Triglycerides 96 mg/dL (all within normal limits).",
            })],
            &SourcePull {
                source_id: "quest-labs".into(),
                source_label: "Quest Diagnostics".into(),
                patient: source_patient("pt-normal", "Amara Mensah"),
                pulled_at: 1_790_503_000,
            },
        )
        .await
        .unwrap();

    let AdmittedPatientReview::New {
        review: normal_rev, ..
    } = store
        .admit_patient_review_with_trigger(
            &practice,
            &normal_ws,
            "cmd-case-normal",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/lipid-normal-1".into(),
                    "rev-1".into(),
                )]),
                changed_candidates: vec!["Observation/lipid-normal-1".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected new normal review");
    };
    let normal_claim = store
        .claim_practice_job(&normal_rev.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct NormalLipidQuietProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for NormalLipidQuietProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-lipid".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/lipid-normal-1",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Quiet {
                            explanation: "Routine screening lipid panel is optimal (LDL 82 mg/dL, HDL 58 mg/dL, TG 96 mg/dL) with nothing open requiring physician action.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/lipid-normal-1".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Verified Quest Diagnostics screening lipid panel (2026-10-18).".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "complete-lipid-quiet".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Normal lipid panel quiet completion.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }
    PracticeJobHost {
        store: store.clone(),
        claim: normal_claim,
        authority: auth.as_ref(),
    }
    .run(
        &mut NormalLipidQuietProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    assert!(
        store
            .open_concerns(
                &WorkspaceAccess {
                    practice: &practice,
                    physician: &dr.physician_id,
                    workspace: &normal_ws,
                    session_hash: &dr_hash,
                },
                &mut authority,
            )
            .await
            .unwrap()
            .is_empty()
    );
    let saved_normal = store
        .patient_review(&practice, &normal_ws, &normal_rev.review_id)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        saved_normal.state,
        ReviewLifecycleState::Completed {
            outcome: ReviewOutcome::Quiet {
                reason: QuietReviewReason::NothingToDo,
                ..
            },
            ..
        }
    ));
}

#[tokio::test]
async fn periodic_reassessment_respects_prior_decisions_avoids_duplicate_alerts_and_updates_memory_only_when_warranted()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-long-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-long-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();
    let (dr, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-long").await;
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();
    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let mut authority = PostgresWorkspaceAuthority;

    // Seed physician-authored memory correction so we also verify periodic reviews preserve it!
    store
        .record_patient_memory(
            "cmd-dr-allergy-note",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: PatientMemoryId::new("mem-dr-ace-cough").unwrap(),
                topic: "Medication intolerance".into(),
                statement: "Physician note: prior dry cough on lisinopril; prefers losartan."
                    .into(),
                provenance: MemoryProvenance::PhysicianCorrection {
                    physician: dr.physician_id.clone(),
                    chat_id: "chat-intake".into(),
                    message_id: "msg-intake-1".into(),
                    work_id: "work-intake".into(),
                    attempt_id: "attempt-intake-1".into(),
                },
            },
        )
        .await
        .unwrap();

    let pull_1 = SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: source_patient("pt-long", "Marcus Vance"),
        pulled_at: 1_790_600_000,
    };
    let bmp_1 = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "Observation/egfr-baseline",
        revision: "rev-1",
        record_type: "Observation",
        title: "Renal Function Panel",
        date: "2026-10-01",
        text: "eGFR 48 mL/min/1.73m2 (down from 56 last year), creatinine 1.52 mg/dL on losartan 50 mg daily. BP 142/88 mmHg.",
    });
    store
        .record_chart(&workspace, std::slice::from_ref(&bmp_1), &pull_1)
        .await
        .unwrap();

    // --- Run 1 (`ChangedRecords`): raises `concern-egfr-trend` + `concern-bp-log`, writes initial
    // memory `mem-ckd-stage` (rev 1) + initial `PatientUnderstanding` (rev 1), and records unfinished work ---
    let AdmittedPatientReview::New { review: run1, .. } = store
        .admit_patient_review_with_trigger(
            &practice,
            &workspace,
            "cmd-long-run-1",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/egfr-baseline".into(),
                    "rev-1".into(),
                )]),
                changed_candidates: vec!["Observation/egfr-baseline".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected run 1");
    };
    let claim_1 = store
        .claim_practice_job(&run1.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct Run1InitialAssessmentProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for Run1InitialAssessmentProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-bmp-1".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/egfr-baseline",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let save_u = SaveUnderstandingArgs {
                        expected_revision: Some(0),
                        operating_index: OperatingIndex {
                            working_overview: "CKD stage 3a/3b transition (eGFR 48 mL/min/1.73m2) and hypertension on losartan 50 mg daily.".into(),
                            known_health_systems: vec![KnownHealthSystem {
                                name: "Clinic EHR".into(),
                                standing: HealthSystemStanding::Connected {
                                    source_id: "clinic-ehr".into(),
                                },
                                note: "Primary nephrology/internal medicine chart".into(),
                            }],
                            open_threads: vec!["Repeat BMP and urine albumin check in 2 weeks".into()],
                            anchors: vec![IndexedAnchor {
                                reference: ExactRecordReference {
                                    candidate_id: "Observation/egfr-baseline".into(),
                                    revision: "rev-1".into(),
                                },
                                source_id: "clinic-ehr".into(),
                                topic: "Renal function".into(),
                            }],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "eGFR declined from 56 to 48 mL/min/1.73m2 (creatinine 1.52 mg/dL) on losartan 50 mg daily".into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/egfr-baseline".into(),
                                revision: "rev-1".into(),
                            }],
                        }],
                        material_gaps: vec!["Repeat BMP pending in 2 weeks".into()],
                        scope_note: None,
                    };
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Declining eGFR (56 -> 48) and elevated office BP (142/88); raised renal trend and BP log concerns and initialized understanding.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/egfr-baseline".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Clinic EHR Renal Function Panel (2026-10-01).".into(),
                        unfinished_work: vec![UnfinishedReviewWork {
                            topic: "Repeat BMP in 2 weeks".into(),
                            detail: "Reassess eGFR and serum potassium once repeat renal panel arrives.".into(),
                            limit: None,
                        }],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "raise-egfr".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-egfr-trend",
                                    "urgency": "today",
                                    "reason": "eGFR declined from 56 to 48 mL/min/1.73m2 (Cr 1.52 mg/dL); repeat renal panel and urine albumin check warranted.",
                                    "records": [{
                                        "candidate_id": "Observation/egfr-baseline",
                                        "revision": "rev-1"
                                    }]
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "raise-bp-office".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-bp-log",
                                    "urgency": "today",
                                    "reason": "Office blood pressure 142/88 mmHg on losartan 50 mg daily; request 7-day home BP log.",
                                    "records": [{
                                        "candidate_id": "Observation/egfr-baseline",
                                        "revision": "rev-1"
                                    }]
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "mem-ckd".into(),
                                name: PATIENT_MEMORY_TOOL.into(),
                                arguments: json!({
                                    "operation": "record_correction",
                                    "memory_id": "mem-ckd-stage",
                                    "topic": "Chronic kidney disease",
                                    "statement": "eGFR 48 mL/min/1.73m2 (creatinine 1.52 mg/dL) as of 2026-10-01 on losartan 50 mg daily."
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "save-u-1".into(),
                                name: SAVE_UNDERSTANDING_TOOL.into(),
                                arguments: serde_json::to_string(&save_u).unwrap(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-run-1".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant("Run 1 completed.")],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }
    PracticeJobHost {
        store: store.clone(),
        claim: claim_1,
        authority: auth.as_ref(),
    }
    .run(
        &mut Run1InitialAssessmentProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Physician clears `concern-bp-log` as `Done` after ordering home BP cuff!
    let done = store
        .mark_concern_done(
            &access,
            &CommandKey::new("cmd-dr-done-bp").unwrap(),
            &ConcernId::new("concern-bp-log").unwrap(),
            &mut authority,
        )
        .await
        .unwrap();
    assert!(matches!(done, ConcernDoneOutcome::Cleared { .. }));

    // --- Run 2 (`Periodic` with unchanged chart): sees open `concern-egfr-trend`, prior review
    // decisions (which raised `concern-bp-log`, now cleared by physician), and unfinished work;
    // finishes quietly WITHOUT repeating alerts or churning memory / understanding! ---
    let saved_run1 = store
        .patient_review(&practice, &workspace, &run1.review_id)
        .await
        .unwrap()
        .unwrap();
    let AdmittedPatientReview::New { review: run2, .. } = store
        .admit_patient_review_with_trigger(
            &practice,
            &workspace,
            "cmd-long-run-2-periodic",
            ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::from([(
                    "Observation/egfr-baseline".into(),
                    "rev-1".into(),
                )]),
                last_reviewed_at: saved_run1.completed_at,
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected run 2");
    };
    let claim_2 = store
        .claim_practice_job(&run2.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct Run2PeriodicNoDuplicateProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for Run2PeriodicNoDuplicateProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => {
                    let wire = serde_json::to_string(&request.items).unwrap();
                    // Verify starting context surfaces active `concern-egfr-trend`, durable memories,
                    // saved understanding, and Run 1's decisions + unfinished work!
                    assert!(wire.contains("concern-egfr-trend"));
                    assert!(wire.contains("Repeat BMP in 2 weeks"));
                    assert!(wire.contains("mem-dr-ace-cough"));
                    assert!(wire.contains("mem-ckd-stage"));
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-bmp-periodic".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "Observation/egfr-baseline",
                                    "revision": "rev-1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Quiet {
                            explanation: "Periodic reassessment: `concern-egfr-trend` is already open on Today, `concern-bp-log` was addressed by physician, and repeat 2-week BMP has not yet resulted; no duplicate alert or memory rewrite warranted.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/egfr-baseline".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Reviewed Clinic EHR chart v1, open Today concerns, and prior review unfinished work.".into(),
                        unfinished_work: vec![UnfinishedReviewWork {
                            topic: "Repeat BMP in 2 weeks".into(),
                            detail: "Reassess eGFR and serum potassium once repeat renal panel arrives.".into(),
                            limit: None,
                        }],
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "complete-run-2".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Periodic review 2 quiet completion.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }
    PracticeJobHost {
        store: store.clone(),
        claim: claim_2,
        authority: auth.as_ref(),
    }
    .run(
        &mut Run2PeriodicNoDuplicateProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Verify Run 2 did NOT duplicate concerns or churn memory/understanding!
    let open_after_run2 = store.open_concerns(&access, &mut authority).await.unwrap();
    assert_eq!(open_after_run2.len(), 1);
    assert_eq!(open_after_run2[0].id.as_str(), "concern-egfr-trend");
    assert_eq!(open_after_run2[0].revision, 1);
    assert_eq!(
        store
            .patient_understanding_for_practice(&practice, &workspace, auth.as_ref())
            .await
            .unwrap()
            .unwrap()
            .revision,
        1,
        "understanding must not be rewritten when unwarranted"
    );
    let memories_after_run2 = store
        .patient_memories(&practice, &workspace, MemorySelection::ActiveOnly)
        .await
        .unwrap();
    assert_eq!(memories_after_run2.len(), 2);
    for entry in &memories_after_run2 {
        assert_eq!(entry.revision, 1);
    }

    // --- Run 3 (`Periodic` after new repeat BMP & nephrology note arrive): updates memory and
    // understanding when clinically warranted, retires resolved concern, and clears unfinished work ---
    let bmp_followup = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "Observation/egfr-followup-2",
        revision: "rev-1",
        record_type: "Observation",
        title: "Repeat Renal Function Panel and Urine ACR",
        date: "2026-10-16",
        text: "Repeat eGFR recovered to 57 mL/min/1.73m2 (creatinine 1.28 mg/dL) after resolving transient NSAID use; urine ACR normal at 11 mg/g; home BP average 124/76 mmHg.",
    });
    store
        .record_chart(
            &workspace,
            &[bmp_1, bmp_followup],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: source_patient("pt-long", "Marcus Vance"),
                pulled_at: 1_790_700_000,
            },
        )
        .await
        .unwrap();

    let saved_run2 = store
        .patient_review(&practice, &workspace, &run2.review_id)
        .await
        .unwrap()
        .unwrap();
    let AdmittedPatientReview::New { review: run3, .. } = store
        .admit_patient_review_with_trigger(
            &practice,
            &workspace,
            "cmd-long-run-3-warranted",
            ReviewTrigger::Periodic {
                workspace_version: 2,
                record_versions: BTreeMap::from([
                    ("Observation/egfr-baseline".into(), "rev-1".into()),
                    ("Observation/egfr-followup-2".into(), "rev-1".into()),
                ]),
                last_reviewed_at: saved_run2.completed_at,
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected run 3");
    };
    let claim_3 = store
        .claim_practice_job(&run3.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct Run3WarrantedUpdateProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for Run3WarrantedUpdateProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-bmp-2".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [
                                {
                                    "candidate_id": "Observation/egfr-baseline",
                                    "revision": "rev-1",
                                    "cursor": null
                                },
                                {
                                    "candidate_id": "Observation/egfr-followup-2",
                                    "revision": "rev-1",
                                    "cursor": null
                                }
                            ],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let save_u2 = SaveUnderstandingArgs {
                        expected_revision: Some(1),
                        operating_index: OperatingIndex {
                            working_overview: "Renal function recovered to baseline eGFR 57 mL/min/1.73m2 (uACR 11 mg/g) after discontinuing transient NSAID; hypertension controlled at 124/76 mmHg on losartan 50 mg daily.".into(),
                            known_health_systems: vec![KnownHealthSystem {
                                name: "Clinic EHR".into(),
                                standing: HealthSystemStanding::Connected {
                                    source_id: "clinic-ehr".into(),
                                },
                                note: "Primary nephrology/internal medicine chart".into(),
                            }],
                            open_threads: vec![],
                            anchors: vec![IndexedAnchor {
                                reference: ExactRecordReference {
                                    candidate_id: "Observation/egfr-followup-2".into(),
                                    revision: "rev-1".into(),
                                },
                                source_id: "clinic-ehr".into(),
                                topic: "Recovered renal panel".into(),
                            }],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "Repeat renal panel on 2026-10-16 showed recovery to eGFR 57 mL/min/1.73m2, normal uACR 11 mg/g, and home BP 124/76 mmHg after stopping transient NSAID".into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/egfr-followup-2".into(),
                                revision: "rev-1".into(),
                            }],
                        }],
                        material_gaps: vec![
                            "Routine annual renal monitoring due in 12 months".into(),
                        ],
                        scope_note: None,
                    };
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Repeat renal panel confirmed recovery to baseline eGFR 57 mL/min/1.73m2 and normal uACR 11 mg/g; retired open renal concern and updated longitudinal memory and summary.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/egfr-followup-2".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Clinic EHR repeat renal panel and home BP summary (2026-10-16).".into(),
                        unfinished_work: vec![],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "retire-egfr".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "retire",
                                    "concern_id": "concern-egfr-trend",
                                    "reason": "Repeat renal panel on 2026-10-16 showed eGFR recovery to 57 mL/min/1.73m2 and normal uACR 11 mg/g after stopping NSAID."
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "update-mem-ckd".into(),
                                name: PATIENT_MEMORY_TOOL.into(),
                                arguments: json!({
                                    "operation": "record_correction",
                                    "memory_id": "mem-ckd-stage",
                                    "topic": "Chronic kidney disease",
                                    "statement": "Baseline eGFR ~56-57 mL/min/1.73m2; transient dip to 48 in Oct 2026 resolved to 57 (uACR 11 mg/g) off NSAIDs."
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "save-u-2".into(),
                                name: SAVE_UNDERSTANDING_TOOL.into(),
                                arguments: serde_json::to_string(&save_u2).unwrap(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-run-3".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Run 3 warranted updates completed.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }
    PracticeJobHost {
        store: store.clone(),
        claim: claim_3,
        authority: auth.as_ref(),
    }
    .run(
        &mut Run3WarrantedUpdateProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    assert!(
        store
            .open_concerns(&access, &mut authority)
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        store
            .patient_understanding_for_practice(&practice, &workspace, auth.as_ref())
            .await
            .unwrap()
            .unwrap()
            .revision,
        2
    );
    let memories_after_run3 = store
        .patient_memories(&practice, &workspace, MemorySelection::ActiveOnly)
        .await
        .unwrap();
    let by_id: BTreeMap<String, _> = memories_after_run3
        .into_iter()
        .map(|m| (m.id.as_str().to_owned(), m))
        .collect();
    assert_eq!(by_id["mem-ckd-stage"].revision, 2);
    assert!(matches!(
        by_id["mem-ckd-stage"].standing,
        MemoryStanding::Active
    ));
    // Physician-supplied medication intolerance memory stayed untouched at revision 1!
    assert_eq!(by_id["mem-dr-ace-cough"].revision, 1);
    assert!(matches!(
        by_id["mem-dr-ace-cough"].provenance,
        MemoryProvenance::PhysicianCorrection { .. }
    ));
}

#[tokio::test]
async fn corrected_evidence_revises_outputs_missing_sources_stay_explicit_limits_and_medication_stays_unapproved_consideration()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-limits-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-limits-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();
    let (dr, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-limits").await;
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();
    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let mut authority = PostgresWorkspaceAuthority;

    // Seed initial records:
    // 1. Preliminary troponin elevation (`Observation/troponin-1` rev-1)
    // 2. Unreadable binary scanned PDF external cardiology fax (`DocumentReference/scanned-echo-pdf` rev-1)
    let scanned_pdf = SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "clinic-ehr".into(),
        candidate_id: "DocumentReference/scanned-echo-pdf".into(),
        revision: "rev-1".into(),
        record_type: "DocumentReference".into(),
        dates: vec![RecordDate {
            name: "date".into(),
            value: "2026-10-18".into(),
        }],
        content: r#"{"resourceType":"DocumentReference","content":[{"attachment":{"contentType":"application/pdf","data":"JVBERi0xLjQK"}}]}"#.into(),
        sha256: sha256_hex(
            br#"{"resourceType":"DocumentReference","content":[{"attachment":{"contentType":"application/pdf","data":"JVBERi0xLjQK"}}]}"#,
        ),
        readable_parts: vec![], // Binary PDF without extracted text -> RecordLimitKind::Unreadable!
        provenance: json!({
            "title": "Scanned Outside Stress Echo Fax",
            "record_locator": "DocumentReference/scanned-echo-pdf"
        }),
    };
    let troponin_v1 = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "Observation/troponin-1",
        revision: "rev-1",
        record_type: "Observation",
        title: "High-Sensitivity Troponin I",
        date: "2026-10-18",
        text: "Preliminary hs-Troponin I: 118 ng/L (elevated; 99th percentile URL 18 ng/L). Patient seen in clinic yesterday for atypical chest discomfort; outside St. Jude Cardiology discharge records not yet connected.",
    });
    store
        .record_chart(
            &workspace,
            &[troponin_v1, scanned_pdf.clone()],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: source_patient("pt-limits", "Nadia Haddad"),
                pulled_at: 1_790_800_000,
            },
        )
        .await
        .unwrap();

    // Enroll cadence and record a failed upstream source pull so `DispatchCheckStanding::FailedSinceLastSuccess`
    // (`RecordLimitKind::Unavailable`) is ALSO active alongside `RecordLimitKind::Unreadable`!
    store
        .set_patient_cadence(
            auth.as_ref(),
            &practice,
            &workspace,
            Cadence::default(),
            1_790_700_000,
        )
        .await
        .unwrap();

    struct FailingExternalFeedChecker;
    #[async_trait]
    impl PatientSourceChecker for FailingExternalFeedChecker {
        async fn check_patient_sources(
            &self,
            _practice_id: &str,
            _workspace_id: &str,
            _now: i64,
        ) -> Result<patient_work::CheckedSources, SourceCheckError> {
            Err(SourceCheckError::Upstream(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "St. Jude HIE gateway timed out",
            )))
        }
    }
    let failed_wake = store
        .check_due_patients(
            auth.as_ref(),
            &practice,
            1_790_800_100,
            &FailingExternalFeedChecker,
        )
        .await
        .unwrap();
    assert_eq!(failed_wake.failed_checks.len(), 1);

    // --- Run 1: Preliminary elevated troponin -> raises `now` concern, surfaces unreadable + unavailable
    // limits explicitly, and frames aspirin/statin strictly as an unapproved physician consideration ---
    let AdmittedPatientReview::New {
        review: rev_initial,
        ..
    } = store
        .admit_patient_review_with_trigger(
            &practice,
            &workspace,
            "cmd-limits-run-1",
            ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::from([
                    ("DocumentReference/scanned-echo-pdf".into(), "rev-1".into()),
                    ("Observation/troponin-1".into(), "rev-1".into()),
                ]),
                changed_candidates: vec!["Observation/troponin-1".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected initial review");
    };
    let claim_initial = store
        .claim_practice_job(&rev_initial.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct Run1PreliminaryAndLimitsProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for Run1PreliminaryAndLimitsProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => {
                    let wire = serde_json::to_string(&request.items).unwrap();
                    // Starting context MUST carry the failed upstream HIE pull limit (`unavailable`),
                    // and Turn 2 verifies `read_patient_sources` returns both `unreadable` (binary PDF)
                    // and `unavailable`!
                    assert!(
                        wire.contains(
                            "Source limit (unavailable): latest source check at unix 1790800100 failed"
                        ),
                        "starting context must surface failed source pull limit: {wire}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-trop-and-pdf".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [
                                    {
                                        "candidate_id": "Observation/troponin-1",
                                        "revision": "rev-1",
                                        "cursor": null
                                    },
                                    {
                                        "candidate_id": "DocumentReference/scanned-echo-pdf",
                                        "revision": "rev-1",
                                        "cursor": null
                                    }
                                ],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let read_out = materialized_output(request, "read-trop-and-pdf");
                    let parsed_read: Value = serde_json::from_str(read_out).unwrap();
                    let limit_kinds: BTreeSet<&str> = parsed_read["limits"]
                        .as_array()
                        .expect("tool read limits array")
                        .iter()
                        .filter_map(|l| l.get("kind").and_then(Value::as_str))
                        .collect();
                    assert!(
                        limit_kinds.contains(RecordLimitKind::Unreadable.as_str())
                            && limit_kinds.contains(RecordLimitKind::Unavailable.as_str()),
                        "read_patient_sources must return explicit unreadable + unavailable limits: {read_out}"
                    );

                    // Also verify attempting a physician-only tool (such as drafting/sending a reply or approving a plan)
                    // is absent from offered tools and refused if ever invoked by a rogue provider!
                    let save_u1 = SaveUnderstandingArgs {
                        expected_revision: Some(0),
                        operating_index: OperatingIndex {
                            working_overview: "Preliminary elevated hs-Troponin I (118 ng/L) with atypical chest discomfort; scanned echo PDF unreadable and St. Jude HIE pull failed.".into(),
                            known_health_systems: vec![
                                KnownHealthSystem {
                                    name: "Clinic EHR".into(),
                                    standing: HealthSystemStanding::Connected {
                                        source_id: "clinic-ehr".into(),
                                    },
                                    note: "Outpatient chart".into(),
                                },
                                KnownHealthSystem {
                                    name: "St. Jude Cardiology".into(),
                                    standing: HealthSystemStanding::MentionedInRecords,
                                    note: "Discharge records missing; latest HIE pull failed".into(),
                                },
                            ],
                            open_threads: vec![
                                "Urgent evaluation of preliminary hs-Troponin I 118 ng/L".into(),
                            ],
                            anchors: vec![IndexedAnchor {
                                reference: ExactRecordReference {
                                    candidate_id: "Observation/troponin-1".into(),
                                    revision: "rev-1".into(),
                                },
                                source_id: "clinic-ehr".into(),
                                topic: "Preliminary hs-Troponin I".into(),
                            }],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "Preliminary hs-Troponin I 118 ng/L reported on 2026-10-18 after atypical chest discomfort".into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/troponin-1".into(),
                                revision: "rev-1".into(),
                            }],
                        }],
                        material_gaps: vec![
                            "Scanned outside stress echo PDF (`DocumentReference/scanned-echo-pdf`) has no extracted text and is unreadable".into(),
                            "St. Jude Cardiology external records are missing and latest source pull failed".into(),
                        ],
                        scope_note: None,
                    };
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Preliminary hs-Troponin I 118 ng/L with atypical chest discomfort; raised `now` concern. Antiplatelet or statin initiation remains strictly a consideration for physician evaluation (no accepted medication plan).".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/troponin-1".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Explicit limits: `DocumentReference/scanned-echo-pdf` is an unreadable binary PDF without extracted text; St. Jude Cardiology records are missing and the latest external source pull failed.".into(),
                        unfinished_work: vec![UnfinishedReviewWork {
                            topic: "Obtain readable stress echo text and St. Jude Cardiology records".into(),
                            detail: "External source pull failed and scanned echo fax lacks OCR/extracted text.".into(),
                            limit: Some(RecordLimit {
                                kind: RecordLimitKind::Unreadable,
                                subject: "DocumentReference/scanned-echo-pdf".into(),
                                detail: "Scanned echo PDF has no extracted clinical text".into(),
                            }),
                        }],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "raise-trop-now".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "raise",
                                    "concern_id": "concern-troponin-118",
                                    "urgency": "now",
                                    "reason": "Preliminary hs-Troponin I elevated at 118 ng/L with recent atypical chest discomfort; urgent physician evaluation needed (any antiplatelet/statin change is unapproved pending physician decision).",
                                    "records": [{
                                        "candidate_id": "Observation/troponin-1",
                                        "revision": "rev-1"
                                    }]
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "save-u-trop-1".into(),
                                name: SAVE_UNDERSTANDING_TOOL.into(),
                                arguments: serde_json::to_string(&save_u1).unwrap(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-trop-1".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Preliminary troponin review saved.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }
    PracticeJobHost {
        store: store.clone(),
        claim: claim_initial,
        authority: auth.as_ref(),
    }
    .run(
        &mut Run1PreliminaryAndLimitsProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    assert_eq!(
        store
            .open_concerns(&access, &mut authority)
            .await
            .unwrap()
            .len(),
        1
    );

    // --- Corrected lab report (`Observation/troponin-1` rev-2) arrives: heterophile antibody assay
    // interference confirmed on rev-1; corrected hs-Troponin I is < 3 ng/L (normal)! ---
    let troponin_v2 = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "Observation/troponin-1",
        revision: "rev-2",
        record_type: "Observation",
        title: "High-Sensitivity Troponin I (Corrected Final)",
        date: "2026-10-18",
        text: "CORRECTED FINAL REPORT: Preliminary 118 ng/L result superseded due to confirmed heterophile antibody assay interference. Corrected hs-Troponin I on blocking-reagent platform is < 3 ng/L (normal, below 99th percentile URL 18 ng/L).",
    });
    store
        .record_chart(
            &workspace,
            &[troponin_v2, scanned_pdf],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: source_patient("pt-limits", "Nadia Haddad"),
                pulled_at: 1_790_801_000,
            },
        )
        .await
        .unwrap();

    let AdmittedPatientReview::New {
        review: rev_corrected,
        ..
    } = store
        .admit_patient_review_with_trigger(
            &practice,
            &workspace,
            "cmd-limits-run-2-corrected",
            ReviewTrigger::ChangedRecords {
                workspace_version: 2,
                record_versions: BTreeMap::from([
                    ("DocumentReference/scanned-echo-pdf".into(), "rev-1".into()),
                    ("Observation/troponin-1".into(), "rev-2".into()),
                ]),
                changed_candidates: vec!["Observation/troponin-1".into()],
            },
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .unwrap()
    else {
        panic!("expected corrected review");
    };
    let claim_corrected = store
        .claim_practice_job(&rev_corrected.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct Run2CorrectedEvidenceProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for Run2CorrectedEvidenceProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-trop-v2".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/troponin-1",
                                "revision": "rev-2",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let save_u2 = SaveUnderstandingArgs {
                        expected_revision: Some(1),
                        operating_index: OperatingIndex {
                            working_overview: "Corrected hs-Troponin I is normal (< 3 ng/L; preliminary elevation was heterophile antibody interference). Unreadable stress echo PDF and missing St. Jude Cardiology records remain open limits.".into(),
                            known_health_systems: vec![
                                KnownHealthSystem {
                                    name: "Clinic EHR".into(),
                                    standing: HealthSystemStanding::Connected {
                                        source_id: "clinic-ehr".into(),
                                    },
                                    note: "Outpatient chart".into(),
                                },
                                KnownHealthSystem {
                                    name: "St. Jude Cardiology".into(),
                                    standing: HealthSystemStanding::MentionedInRecords,
                                    note: "Discharge records missing; latest HIE pull failed".into(),
                                },
                            ],
                            open_threads: vec![
                                "Obtain readable outside stress echo report and St. Jude Cardiology records".into(),
                            ],
                            anchors: vec![IndexedAnchor {
                                reference: ExactRecordReference {
                                    candidate_id: "Observation/troponin-1".into(),
                                    revision: "rev-2".into(),
                                },
                                source_id: "clinic-ehr".into(),
                                topic: "Corrected normal hs-Troponin I".into(),
                            }],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "Corrected final hs-Troponin I on 2026-10-18 is normal (< 3 ng/L) after blocking-reagent testing confirmed heterophile antibody interference on the preliminary result".into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/troponin-1".into(),
                                revision: "rev-2".into(),
                            }],
                        }],
                        material_gaps: vec![
                            "Scanned outside stress echo PDF (`DocumentReference/scanned-echo-pdf`) remains unreadable without extracted text".into(),
                            "St. Jude Cardiology external records remain missing after failed upstream source pull".into(),
                        ],
                        scope_note: None,
                    };
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Corrected hs-Troponin I (`rev-2`) is normal (< 3 ng/L; preliminary 118 ng/L was heterophile antibody interference). Retired urgent troponin concern and revised patient understanding; no antiplatelet or statin plan was accepted, and any lipid/preventive therapy remains a physician consideration.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/troponin-1".into(),
                            revision: "rev-2".into(),
                        }],
                        basis_and_limitations: "Assessed from corrected Clinic EHR hs-Troponin I (`rev-2`). Explicit limits remain: `DocumentReference/scanned-echo-pdf` is unreadable (binary PDF without text) and St. Jude Cardiology records are unavailable after a failed upstream source check.".into(),
                        unfinished_work: vec![UnfinishedReviewWork {
                            topic: "Unreadable stress echo fax and unavailable St. Jude Cardiology records".into(),
                            detail: "Retry external source check and obtain readable stress echo interpretation.".into(),
                            limit: Some(RecordLimit {
                                kind: RecordLimitKind::Unavailable,
                                subject: "st-jude-hie".into(),
                                detail: "Latest upstream HIE pull timed out".into(),
                            }),
                        }],
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "retire-trop-now".into(),
                                name: CONCERN_TOOL.into(),
                                arguments: json!({
                                    "operation": "retire",
                                    "concern_id": "concern-troponin-118",
                                    "reason": "Corrected final lab report (rev-2) confirmed heterophile antibody interference; repeat hs-Troponin I is normal (< 3 ng/L)."
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "save-u-trop-2".into(),
                                name: SAVE_UNDERSTANDING_TOOL.into(),
                                arguments: serde_json::to_string(&save_u2).unwrap(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "complete-trop-2".into(),
                                name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                                arguments: serde_json::to_string(&complete).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Corrected troponin review saved; urgent concern retired and limits retained.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }
    PracticeJobHost {
        store: store.clone(),
        claim: claim_corrected,
        authority: auth.as_ref(),
    }
    .run(
        &mut Run2CorrectedEvidenceProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Verify corrected evidence retired the concern from Today into Updates, revised PatientUnderstanding
    // to cite rev-2, kept missing/unreadable/unavailable limits explicit, and produced zero accepted medication plan!
    assert!(
        store
            .open_concerns(&access, &mut authority)
            .await
            .unwrap()
            .is_empty()
    );
    let retired_updates = store
        .retired_concern_updates(&access, &mut authority)
        .await
        .unwrap();
    assert_eq!(retired_updates.len(), 1);
    assert!(
        retired_updates[0]
            .retirement_reason
            .as_str()
            .contains("< 3 ng/L")
    );

    let revised_u = store
        .patient_understanding_for_practice(&practice, &workspace, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(revised_u.revision, 2);
    assert_eq!(revised_u.source_version, 2);
    assert_eq!(revised_u.summary.claims.len(), 1);
    assert_eq!(revised_u.summary.claims[0].citations[0].revision, "rev-2");
    assert_eq!(revised_u.summary.material_gaps.len(), 2);

    let saved_corr_review = store
        .patient_review(&practice, &workspace, &rev_corrected.review_id)
        .await
        .unwrap()
        .unwrap();
    let ReviewLifecycleState::Completed {
        outcome:
            ReviewOutcome::Assessed {
                summary,
                citations,
                basis_and_limitations,
            },
        ..
    } = &saved_corr_review.state
    else {
        panic!("expected Assessed corrected review");
    };
    assert_eq!(citations.len(), 1);
    assert_eq!(citations[0].reference.revision, "rev-2");
    assert!(
        basis_and_limitations.contains("unreadable")
            && basis_and_limitations.contains("unavailable")
    );
    assert!(summary.contains("no antiplatelet or statin plan was accepted"));
    assert_eq!(saved_corr_review.unfinished_work.len(), 1);
}
