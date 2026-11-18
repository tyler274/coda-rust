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
    DispatchCheckStanding, PatientMemoryStanding, SourceFreshnessSummary,
};
use local_product::runtime::concerns::{
    CONCERN_TOOL, ConcernId, ConcernUrgency, ExactRecordReference, WorkspaceAccess,
};
use local_product::runtime::dispatch::StoredObservationChecker;
use local_product::runtime::jobs::{PracticeJobHost, PracticeJobKind, is_physician_only_tool};
use local_product::runtime::memory::{self, MemoryPath, PatientKey};
use local_product::runtime::refresh::{
    FixedSourceCheck, PatientSourceRefresh, REFRESH_TOOL, SourceCheckOutcome, SourceId,
    SourceRefreshAdapterError,
};
use local_product::runtime::reviews::{
    AdmittedPatientReview, PatientReviewError, PatientReviewReceiver, RecordedReviewAction,
    ReviewLifecycleState, ReviewUsage,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{
    Authority, PublicationTransaction, Scope, Store, async_trait, engine,
};
use local_product::{ModelBinding, ModelRoles, sha256_hex};
use patient_work::ReviewReason;
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
        envelope: None,
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
            summary_markdown: None,
            direction_markdown: None,
            has_person_page: false,
            page_paths: Vec::new(),
            log_days: Vec::new(),
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
        pulled_at: Some(1_790_000_000),
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
        pulled_at: Some(reg_time + 8 * 86_400),
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

    // Execute both the changed-record review and the periodic review through PracticeJobHost,
    // asserting the exact Request reaching the model (recipe, background model settings,
    // preloaded retrieval skill, discoverable patient-communication skill, and tools).
    let changed_reviews = store
        .patient_reviews(&practice, &changed_ws, auth.as_ref())
        .await
        .unwrap();
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
                "list_patient_memory",
                "read_patient_memory",
                "search_patient_memory",
                "add_patient_memory_note",
                "edit_patient_memory",
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
        .patient_review(
            &practice,
            &changed_ws,
            &changed_rev.review_id,
            auth.as_ref(),
        )
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
        .patient_reviews(&practice, &periodic_ws, auth.as_ref())
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
        .patient_review(
            &practice,
            &periodic_ws,
            &periodic_dispatched.review_id,
            auth.as_ref(),
        )
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

    // Verify physician-requested review admission & panel authorization enforcement
    // after periodic review completes (and verify completed periodic review stays Completed).
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
        pulled_at: Some(1_790_100_000),
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
                                name: "add_patient_memory_note".into(),
                                arguments: json!({
                                    "text": "HFrEF (EF 38%); baseline NT-proBNP 640 pg/mL rose to 2,850 pg/mL on 2026-10-14.",
                                    "quote": null
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
                        ("mem-hfref", "\"note\":\"N1\""),
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

    let mem_key = PatientKey::for_rounds(&practice, &workspace).unwrap();
    let mem_listing = restarted.list_memory(&mem_key).await.unwrap();
    assert_eq!(mem_listing.log_days.len(), 1);
    let log_text = match restarted
        .read_memory(&mem_key, &MemoryPath::Log(mem_listing.log_days[0]), None)
        .await
        .unwrap()
    {
        memory::FileRead::Log { markdown } | memory::FileRead::Version { markdown, .. } => markdown,
    };
    assert!(log_text.contains("HFrEF (EF 38%); baseline NT-proBNP 640 pg/mL rose to 2,850 pg/mL"));

    let understanding = restarted
        .patient_understanding_for_practice(&practice, &workspace, auth.as_ref())
        .await
        .unwrap()
        .expect("saved understanding persists across restart");
    assert_eq!(understanding.revision, 1);
    assert_eq!(understanding.source_version, 2);

    let readback_review = restarted
        .patient_review(&practice, &workspace, &review.review_id, auth.as_ref())
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
    assert!(readback_review.actions.iter().any(|a| matches!(
        a,
        RecordedReviewAction::MemoryRecorded {
            topic,
            revision: 1,
            ..
        } if topic == "patient_memory_note"
    )));
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
        pulled_at: Some(1_790_101_000),
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
        .patient_review(&practice, &quiet_ws, &quiet_rev.review_id, auth.as_ref())
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
        pulled_at: Some(1_790_200_000),
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
                        pulled_at: Some(1_790_200_300),
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
        .patient_review(&practice, &workspace, &corr_rev.review_id, auth.as_ref())
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
        pulled_at: Some(1_790_201_000),
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

    // Run Attempt 1 through real `PracticeJobHost::run` with a source refresh operation that cancels
    // Attempt 1 before `ToolReturned`, then append `ToolStarted` + DB commit (no `ToolReturned`) for
    // BOTH `add_patient_memory_note` and `patient_concern`!
    struct CancelOnRefresh {
        cancel: engine::Cancellation,
    }
    #[async_trait]
    impl PatientSourceRefresh for CancelOnRefresh {
        async fn configured_sources(
            &self,
            _practice: &str,
            _workspace: &str,
        ) -> Result<Vec<SourceId>, SourceRefreshAdapterError> {
            Ok(vec![SourceId::new("clinic-ehr").unwrap()])
        }

        async fn check_source(
            &self,
            _practice: &str,
            _workspace: &str,
            _source_id: &SourceId,
        ) -> Result<SourceCheckOutcome, SourceRefreshAdapterError> {
            self.cancel.cancel();
            std::future::pending::<()>().await;
            unreachable!()
        }
    }

    let refresh_cancel_call = engine::Invocation {
        id: "refresh-crash".to_owned(),
        name: REFRESH_TOOL.to_owned(),
        arguments: json!({}).to_string(),
    };
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
        name: "add_patient_memory_note".to_owned(),
        arguments: json!({
            "text": "Atrial fibrillation on warfarin 5 mg daily; supratherapeutic INR 4.9 on 2026-10-15.",
            "quote": null
        })
        .to_string(),
    };

    struct FirstAttemptCrashProvider {
        refresh_call: engine::Invocation,
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
                        engine::Item::Call(self.refresh_call.clone()),
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
    let crashing_store = store.clone().with_source_refresh(Arc::new(CancelOnRefresh {
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
                refresh_call: refresh_cancel_call.clone(),
                mem_call: mem_call.clone(),
                concern_call: concern_call.clone(),
                turn: 0,
            },
            engine::RunLimits::default(),
            &cancel_attempt_1,
        )
        .await
        .expect_err("attempt 1 must abort during refresh before ToolReturned");
    assert!(matches!(attempt_1_err, engine::Failure::Cancelled));

    // Also persist `ToolStarted` and execute BOTH `mem_call` and `concern_call` without `ToolReturned`
    // so BOTH `patient_memory` note AND `patient_concern` committed on Attempt 1 without `ToolReturned`!
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
    for (offset, call) in [&mem_call, &concern_call].into_iter().enumerate() {
        engine::Product::persist(
            &mut exec_host,
            engine::Event {
                attempt_id: claim_1.attempt.clone(),
                sequence: last_event.sequence + (offset as u64) + 1,
                ordinal: last_event.ordinal,
                kind: engine::EventKind::ToolStarted(call.clone()),
            },
        )
        .await
        .unwrap();
        let args: Value = serde_json::from_str(&call.arguments).unwrap();
        let exec_out = engine::Product::execute(&mut exec_host, call, args)
            .await
            .unwrap();
        assert!(
            matches!(exec_out, engine::ToolOutcome::Returned(_)),
            "expected committed tool outcome for {}: {exec_out:?}",
            call.id
        );
    }
    drop(exec_host);
    drop(host_1);

    store
        .settle_practice_job_attempt(
            &claim_1,
            engine::AttemptFailure::Interrupted,
            local_product::runtime::delivery::Backoff::IMMEDIATE,
        )
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
                        m_out.contains("\"note\":\"N1\""),
                        "recovered memory action must reconcile idempotently without unknown: {m_out}"
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

    let crash_key = PatientKey::for_rounds(&practice, &crash_ws).unwrap();
    let crash_listing = store.list_memory(&crash_key).await.unwrap();
    assert_eq!(crash_listing.log_days.len(), 1);
    let crash_log = match store
        .read_memory(
            &crash_key,
            &MemoryPath::Log(crash_listing.log_days[0]),
            None,
        )
        .await
        .unwrap()
    {
        memory::FileRead::Log { markdown } | memory::FileRead::Version { markdown, .. } => markdown,
    };
    assert_eq!(
        crash_log
            .matches("Atrial fibrillation on warfarin 5 mg daily; supratherapeutic INR 4.9")
            .count(),
        1,
        "memory note must be written once and never repeated on recovery: {crash_log}"
    );

    let saved_crash_rev = store
        .patient_review(&practice, &crash_ws, &crash_rev.review_id, auth.as_ref())
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
        pulled_at: Some(1_790_300_000),
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
        .patient_review(&practice, &workspace, &rev1.review_id, auth.as_ref())
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
        .patient_review(&practice, &workspace, &rev2.review_id, auth.as_ref())
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
