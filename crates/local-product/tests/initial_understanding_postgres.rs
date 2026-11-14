//! Real PostgreSQL integration tests for initial and revised patient
//! understanding (`initial_understanding` + `runtime::understanding`).
//!
//! Runs automatically whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL` or
//! `PHYSICIAN_DATABASE_URL` is set; panics if neither is set when
//! `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

use local_product::initial_understanding::{
    ConnectedSourceBrief, EvidenceScope, HealthSystemStanding, IndexedAnchor, KnownHealthSystem,
    OperatingIndex, READ_UNDERSTANDING_TOOL, SAVE_UNDERSTANDING_TOOL, SaveUnderstandingArgs,
    SummaryClaimInput, UnderstandingMode,
};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
    SourceRevision,
};
use local_product::runtime::concerns::ExactRecordReference;
use local_product::runtime::jobs::{PracticeJobHost, PracticeJobKind, is_physician_only_tool};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::understanding::{
    PhysicianSummaryAccessError, RevisionOrigin, UnderstandingRunState, UnderstandingTriggerOutcome,
};
use local_product::runtime::{
    Accepted, AttemptStatus, Authority, Input, PhysicianHost, PublicationTransaction, Scope,
    ShownSteps, Store, WorkStatus, async_trait, engine,
};
use local_product::{ModelBinding, ModelRoles, sha256_hex};
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
            model: "background-worker-lane".into(),
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
        birth_date: Some("1972-08-27".to_owned()),
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
    let hash = PostgresAuthStore::session_reference(&token);
    (identity, hash)
}

/// AC1: A new patient link starts the intended assignment with its model
/// settings (`roles.background`), initial guidance (`patient-record-retrieval`
/// preloaded with sha256 + `initial_understanding::INSTRUCTIONS`), and
/// discoverable capabilities (`skill_view`, chart search/read, understanding
/// read/save, working notes, history, tool-result paging, zero physician-only tools).
#[tokio::test]
async fn ac1_new_patient_link_starts_intended_assignment_with_model_settings_guidance_and_capabilities()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-ac1-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-ac1-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    let pull = SourcePull {
        source_id: "clinic-ehr".to_owned(),
        source_label: "Clinic EHR".to_owned(),
        patient: source_patient("ac1-pt", "Elena Rostova"),
        pulled_at: Some(1_788_000_000),
    };
    let note = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "DocumentReference/intake-1",
        revision: "rev-1",
        record_type: "DocumentReference",
        title: "New Patient Intake Note",
        date: "2026-09-01",
        text: "Stage 2 hypertension on amlodipine 5 mg daily. Outside cardiology records from Mercy Heart Institute not yet transferred.",
    });
    store
        .record_chart(&workspace, &[note], &pull)
        .await
        .unwrap();

    let triggered = store
        .trigger_patient_understanding(&practice, &workspace, &roles, &skills, auth.as_ref())
        .await
        .expect("new patient link triggers initial understanding");
    let UnderstandingTriggerOutcome::Started {
        scope,
        mode,
        source_version,
    } = triggered
    else {
        panic!("expected Started, got {triggered:?}");
    };
    assert_eq!(mode, UnderstandingMode::Initial);
    assert_eq!(source_version, 1);

    let saved_job = store
        .practice_job(&scope)
        .await
        .unwrap()
        .expect("saved initial understanding job");
    assert_eq!(saved_job.kind, PracticeJobKind::InitialUnderstanding);
    assert_eq!(saved_job.status, WorkStatus::Pending);
    assert_eq!(saved_job.preloaded_skills.len(), 1);
    assert_eq!(
        saved_job.preloaded_skills[0].name,
        "patient-record-retrieval"
    );
    assert!(!saved_job.preloaded_skills[0].sha256.is_empty());
    assert!(
        saved_job.preloaded_skills[0]
            .content
            .contains("# Patient Record Retrieval"),
        "preloaded retrieval skill must carry full body"
    );
    let ctx = saved_job
        .assignment
        .understanding
        .as_ref()
        .expect("typed UnderstandingContext recorded on assignment");
    assert_eq!(ctx.mode, UnderstandingMode::Initial);
    assert_eq!(
        ctx.evidence_scope,
        EvidenceScope::ClinicalChart {
            connected_sources: vec![ConnectedSourceBrief {
                source_id: "clinic-ehr".to_owned(),
                source_label: "Clinic EHR".to_owned(),
                pulled_at: 1_788_000_000,
            }],
        }
    );
    assert_eq!(ctx.target_source_version, 1);
    assert_eq!(
        ctx.changed_candidates,
        vec!["DocumentReference/intake-1".to_owned()]
    );

    let binding = store
        .practice_job_binding(&scope, &saved_job.binding)
        .await
        .expect("frozen request binding");
    assert_eq!(binding.recipe.model, roles.background.model);
    assert_eq!(
        binding.recipe.options["effort"],
        json!(roles.background.effort)
    );
    assert_eq!(
        binding.recipe.context_window_tokens,
        roles.background.context_window_tokens
    );
    assert_eq!(
        binding.recipe.max_input_tokens,
        roles.background.max_input_tokens
    );
    assert_ne!(binding.recipe.model, roles.chat.model);

    assert!(
        binding
            .recipe
            .instructions
            .contains(local_product::initial_understanding::INSTRUCTIONS)
    );
    assert!(
        binding
            .recipe
            .instructions
            .contains("patient-record-retrieval")
    );
    assert!(
        binding
            .recipe
            .instructions
            .contains("patient-communication")
    );

    let tool_names: BTreeSet<String> = binding
        .recipe
        .tools
        .iter()
        .filter_map(|t| t.get("name").and_then(Value::as_str).map(str::to_owned))
        .collect();
    for expected in [
        harness::skills::SKILL_VIEW,
        "read_tool_result",
        SEARCH_TOOL,
        READ_TOOL,
        READ_UNDERSTANDING_TOOL,
        SAVE_UNDERSTANDING_TOOL,
    ] {
        assert!(
            tool_names.contains(expected),
            "missing expected capability {expected} in {tool_names:?}"
        );
    }
    for name in &tool_names {
        assert!(
            !is_physician_only_tool(name),
            "background understanding job must never offer physician-only tool {name}"
        );
    }

    model_text::assert_clean_model_text(
        &serde_json::to_string(&json!({
            "instructions": binding.recipe.instructions,
            "assignment_prompt": saved_job.assignment.prompt,
            "tools": binding.recipe.tools,
        }))
        .unwrap(),
    );

    // Prove on-demand capability discovery (`skill_view`) and full execution inside PracticeJobHost.
    let claim = store
        .claim_practice_job(&scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim initial understanding job");

    struct CapabilityDiscoveryProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for CapabilityDiscoveryProvider {
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
                            id: "discover-skill".into(),
                            name: harness::skills::SKILL_VIEW.into(),
                            arguments: json!({
                                "name": "patient-communication",
                                "file_path": null
                            })
                            .to_string(),
                        }),
                        engine::Item::Call(engine::Invocation {
                            id: "read-intake".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "DocumentReference/intake-1",
                                    "revision": "rev-1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let skill_out = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "discover-skill" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("skill_view output materialized");
                    assert!(
                        skill_out.contains("patient-communication"),
                        "on-demand skill_view did not load skill: {skill_out}"
                    );
                    let save_args = SaveUnderstandingArgs {
                        expected_revision: Some(0),
                        operating_index: OperatingIndex {
                            working_overview: "Stage 2 hypertension on amlodipine 5 mg daily; outside cardiology records pending.".into(),
                            known_health_systems: vec![
                                KnownHealthSystem {
                                    name: "Clinic EHR".into(),
                                    standing: HealthSystemStanding::Connected {
                                        source_id: "clinic-ehr".into(),
                                    },
                                    note: "Primary outpatient EHR".into(),
                                },
                                KnownHealthSystem {
                                    name: "Mercy Heart Institute".into(),
                                    standing: HealthSystemStanding::MentionedInRecords,
                                    note: "Mentioned in intake note; records not yet connected".into(),
                                },
                            ],
                            open_threads: vec![
                                "Mercy Heart Institute cardiology records not yet received".into(),
                            ],
                            anchors: vec![IndexedAnchor {
                                reference: ExactRecordReference {
                                    candidate_id: "DocumentReference/intake-1".into(),
                                    revision: "rev-1".into(),
                                },
                                source_id: "clinic-ehr".into(),
                                topic: "Hypertension intake".into(),
                            }],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "Stage 2 hypertension managed on amlodipine 5 mg daily".into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "DocumentReference/intake-1".into(),
                                revision: "rev-1".into(),
                            }],
                        }],
                        material_gaps: vec![
                            "Mercy Heart Institute cardiology records not yet received".into(),
                        ],
                        scope_note: None,
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "save-1".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: serde_json::to_string(&save_args).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Saved initial patient understanding.",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
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
        &mut CapabilityDiscoveryProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("AC1 run completes");
}

/// AC2: A synthetic chart gives a saved memory and summary that state material
/// gaps and keep verified source support (refusing unread citations and refusing
/// completion before saving), plus explicit Duet-only scope labeling.
#[tokio::test]
async fn ac2_synthetic_chart_saves_verified_memory_and_summary_with_material_gaps_and_duet_only_scope()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-ac2-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-ac2-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    let pull = SourcePull {
        source_id: "clinic-ehr".to_owned(),
        source_label: "Clinic EHR".to_owned(),
        patient: source_patient("ac2-pt", "Marcus Vance"),
        pulled_at: Some(1_788_100_000),
    };
    let problem_rec = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "Condition/afib-1",
        revision: "rev-afib-1",
        record_type: "Condition",
        title: "Paroxysmal Atrial Fibrillation",
        date: "2026-08-14",
        text: "Paroxysmal atrial fibrillation on apixaban 5 mg BID; 2025 transthoracic echocardiogram performed at Metro Cardiology Health System is not in this chart.",
    });
    let lab_rec = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "DiagnosticReport/bmp-1",
        revision: "rev-bmp-1",
        record_type: "DiagnosticReport",
        title: "Basic Metabolic Panel",
        date: "2026-09-10",
        text: "Creatinine 1.18 mg/dL, Potassium 4.3 mmol/L, eGFR 68 mL/min/1.73m2.",
    });
    let expected_afib_sha = problem_rec.sha256.clone();
    let expected_bmp_sha = lab_rec.sha256.clone();

    store
        .record_chart(&workspace, &[problem_rec, lab_rec], &pull)
        .await
        .unwrap();

    let UnderstandingTriggerOutcome::Started { scope, .. } = store
        .trigger_patient_understanding(&practice, &workspace, &roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected Started");
    };
    let claim = store
        .claim_practice_job(&scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct SyntheticChartUnderstandingProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for SyntheticChartUnderstandingProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            let draft_args = SaveUnderstandingArgs {
                expected_revision: None,
                operating_index: OperatingIndex {
                    working_overview: "Paroxysmal atrial fibrillation on apixaban with mild CKD; Metro Cardiology echo missing.".into(),
                    known_health_systems: vec![
                        KnownHealthSystem {
                            name: "Clinic EHR".into(),
                            standing: HealthSystemStanding::Connected {
                                source_id: "clinic-ehr".into(),
                            },
                            note: "Primary connected outpatient EHR".into(),
                        },
                        KnownHealthSystem {
                            name: "Metro Cardiology Health System".into(),
                            standing: HealthSystemStanding::MentionedInRecords,
                            note: "2025 echocardiogram referenced in Condition/afib-1".into(),
                        },
                    ],
                    open_threads: vec![
                        "2025 Metro Cardiology transthoracic echocardiogram report missing".into(),
                        "Recent body weight for apixaban dose check not documented".into(),
                    ],
                    anchors: vec![
                        IndexedAnchor {
                            reference: ExactRecordReference {
                                candidate_id: "Condition/afib-1".into(),
                                revision: "rev-afib-1".into(),
                            },
                            source_id: "clinic-ehr".into(),
                            topic: "Atrial fibrillation diagnosis and outside echo note".into(),
                        },
                        IndexedAnchor {
                            reference: ExactRecordReference {
                                candidate_id: "DiagnosticReport/bmp-1".into(),
                                revision: "rev-bmp-1".into(),
                            },
                            source_id: "clinic-ehr".into(),
                            topic: "Baseline renal function".into(),
                        },
                    ],
                },
                claims: vec![
                    SummaryClaimInput {
                        text: "Paroxysmal atrial fibrillation managed with apixaban 5 mg twice daily".into(),
                        citations: vec![ExactRecordReference {
                            candidate_id: "Condition/afib-1".into(),
                            revision: "rev-afib-1".into(),
                        }],
                    },
                    SummaryClaimInput {
                        text: "Creatinine was 1.18 mg/dL (eGFR 68) with potassium 4.3 mmol/L on 2026-09-10".into(),
                        citations: vec![ExactRecordReference {
                            candidate_id: "DiagnosticReport/bmp-1".into(),
                            revision: "rev-bmp-1".into(),
                        }],
                    },
                ],
                material_gaps: vec![
                    "2025 Metro Cardiology echocardiogram report not yet transferred".into(),
                    "Current body weight not recorded in connected chart".into(),
                ],
                scope_note: None,
            };

            match self.turn {
                // Turn 1: Browse chart metadata, and prematurely try to save BEFORE reading exact sources!
                1 => Ok(engine::Turn {
                    items: vec![
                        engine::Item::Call(engine::Invocation {
                            id: "search-1".into(),
                            name: SEARCH_TOOL.into(),
                            arguments: json!({
                                "query": "",
                                "record_type": null,
                                "after": null
                            })
                            .to_string(),
                        }),
                        engine::Item::Call(engine::Invocation {
                            id: "premature-save".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: serde_json::to_string(&draft_args).unwrap(),
                        }),
                    ],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                // Turn 2: Verify premature save was refused (`unread_source_reference`), then read ONLY `Condition/afib-1`
                // and verify an unread `operating_index.anchors` entry (`DiagnosticReport/bmp-1`) is ALSO refused!
                2 => {
                    let refusal = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "premature-save" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("premature save refusal materialized");
                    assert!(
                        refusal.contains("unread_source_reference"),
                        "citing unread record must be refused: {refusal}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-afib-only".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [
                                    {
                                        "candidate_id": "Condition/afib-1",
                                        "revision": "rev-afib-1",
                                        "cursor": null
                                    }
                                ],
                                "max_result_bytes": 32_768
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                // Turn 3: Cite ONLY the read `Condition/afib-1` in `claims`, while smuggling unread `DiagnosticReport/bmp-1`
                // into `operating_index.anchors` -> must be refused (`unread_source_reference`), then read `DiagnosticReport/bmp-1`!
                3 => {
                    let unread_anchor_args = SaveUnderstandingArgs {
                        expected_revision: None,
                        operating_index: draft_args.operating_index.clone(),
                        claims: vec![SummaryClaimInput {
                            text: "Paroxysmal atrial fibrillation managed with apixaban 5 mg twice daily"
                                .into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "Condition/afib-1".into(),
                                revision: "rev-afib-1".into(),
                            }],
                        }],
                        material_gaps: draft_args.material_gaps.clone(),
                        scope_note: None,
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "unread-anchor-save".into(),
                                name: SAVE_UNDERSTANDING_TOOL.into(),
                                arguments: serde_json::to_string(&unread_anchor_args).unwrap(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "read-bmp".into(),
                                name: READ_TOOL.into(),
                                arguments: json!({
                                    "references": [
                                        {
                                            "candidate_id": "DiagnosticReport/bmp-1",
                                            "revision": "rev-bmp-1",
                                            "cursor": null
                                        }
                                    ],
                                    "max_result_bytes": 32_768
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                // Turn 4: Confirm `unread-anchor-save` was refused, then try finishing WITHOUT calling `save_patient_understanding` -> PracticeJobHost forces Continue!
                4 => {
                    let anchor_refusal = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "unread-anchor-save" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("unread anchor refusal materialized");
                    assert!(
                        anchor_refusal.contains("unread_source_reference")
                            && anchor_refusal.contains("DiagnosticReport/bmp-1"),
                        "unread operating_index anchor must be refused: {anchor_refusal}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant(
                            "Attempting to finish before saving structured understanding.",
                        )],
                        receipt: json!({"turn": 4}),
                        usage: Default::default(),
                    })
                }
                // Turn 5: Save verified understanding!
                5 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "verified-save".into(),
                        name: SAVE_UNDERSTANDING_TOOL.into(),
                        arguments: serde_json::to_string(&draft_args).unwrap(),
                    })],
                    receipt: json!({"turn": 5}),
                    usage: Default::default(),
                }),
                // Turn 6: Conclude cleanly after saving.
                6 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Initial patient understanding saved with verified citations and material gaps.",
                    )],
                    receipt: json!({"turn": 6}),
                    usage: Default::default(),
                }),
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
        &mut SyntheticChartUnderstandingProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("AC2 synthetic chart run completes");

    // Reconnect on a fresh Store to verify durable Postgres persistence.
    let reopened = Store::connect(&url).await.unwrap();
    let saved = reopened
        .patient_understanding_for_practice(&practice, &workspace, auth.as_ref())
        .await
        .unwrap()
        .expect("saved patient understanding persisted");
    assert_eq!(saved.revision, 1);
    assert_eq!(saved.source_version, 1);
    assert!(matches!(
        saved.evidence_scope,
        EvidenceScope::ClinicalChart { .. }
    ));
    assert_eq!(saved.summary.material_gaps.len(), 2);
    assert!(saved.summary.narrative.contains(
        "Material gaps: 2025 Metro Cardiology echocardiogram report not yet transferred"
    ));
    assert_eq!(saved.summary.citations.len(), 2);
    assert_eq!(saved.summary.citations[0].sha256, expected_afib_sha);
    assert_eq!(
        saved.summary.citations[0].as_of.as_deref(),
        Some("2026-08-14")
    );
    assert_eq!(saved.summary.citations[1].sha256, expected_bmp_sha);
    assert_eq!(
        saved.summary.citations[1].as_of.as_deref(),
        Some("2026-09-10")
    );
    assert!(
        saved
            .operating_index
            .known_health_systems
            .iter()
            .any(|hs| hs.name == "Metro Cardiology Health System"
                && hs.standing == HealthSystemStanding::MentionedInRecords)
    );

    // Also verify Duet-only scope (`EvidenceScope::DuetPatientSharedOnly`): requires explicit `scope_note`.
    let duet_ws = format!("pw-duet-only-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &duet_ws).await.unwrap();
    let duet_pull = SourcePull {
        source_id: "duet-patient-shared".to_owned(),
        source_label: "Duet Patient-Shared".to_owned(),
        patient: source_patient("duet-pt", "Nadia Lin"),
        pulled_at: Some(1_788_100_500),
    };
    let duet_note = chart_record(ChartRecordSpec {
        workspace: &duet_ws,
        source_id: "duet-patient-shared",
        candidate_id: "PatientSummary/duet-1",
        revision: "rev-duet-1",
        record_type: "DocumentReference",
        title: "Patient-Shared Intake",
        date: "2026-09-12",
        text: "Patient reports intermittent migraine headaches and sumatriptan 50 mg PRN; clinic EHR not yet linked.",
    });
    reopened
        .record_chart(&duet_ws, &[duet_note], &duet_pull)
        .await
        .unwrap();

    let UnderstandingTriggerOutcome::Started {
        scope: duet_scope, ..
    } = reopened
        .trigger_patient_understanding(&practice, &duet_ws, &roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected Started for Duet-only workspace");
    };
    let duet_claim = reopened
        .claim_practice_job(&duet_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct DuetOnlyProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for DuetOnlyProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            let make_args = |scope_note: Option<String>| {
                SaveUnderstandingArgs {
                expected_revision: None,
                operating_index: OperatingIndex {
                    working_overview: "Patient-shared migraine history via Duet; no clinic EHR connected yet.".into(),
                    known_health_systems: vec![KnownHealthSystem {
                        name: "Duet Patient-Shared".into(),
                        standing: HealthSystemStanding::Connected {
                            source_id: "duet-patient-shared".into(),
                        },
                        note: "Patient-shared Duet connection".into(),
                    }],
                    open_threads: vec!["No clinical EHR records connected yet".into()],
                    anchors: vec![IndexedAnchor {
                        reference: ExactRecordReference {
                            candidate_id: "PatientSummary/duet-1".into(),
                            revision: "rev-duet-1".into(),
                        },
                        source_id: "duet-patient-shared".into(),
                        topic: "Patient-shared migraine summary".into(),
                    }],
                },
                claims: vec![SummaryClaimInput {
                    text: "Patient-reported intermittent migraines treated with sumatriptan 50 mg as needed".into(),
                    citations: vec![ExactRecordReference {
                        candidate_id: "PatientSummary/duet-1".into(),
                        revision: "rev-duet-1".into(),
                    }],
                }],
                material_gaps: vec!["Clinical EHR records not yet connected".into()],
                scope_note,
            }
            };
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-duet".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "PatientSummary/duet-1",
                                "revision": "rev-duet-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                // Attempt without scope_note -> refused!
                2 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "save-missing-scope".into(),
                        name: SAVE_UNDERSTANDING_TOOL.into(),
                        arguments: serde_json::to_string(&make_args(None)).unwrap(),
                    })],
                    receipt: json!({"turn": 2}),
                    usage: Default::default(),
                }),
                3 => {
                    let out = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "save-missing-scope" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .unwrap();
                    assert!(
                        out.contains("missing_scope_note"),
                        "expected Duet-only scope_note refusal, got: {out}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "save-with-scope".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: serde_json::to_string(&make_args(Some(
                                "Summary reflects patient-shared Duet information only; clinical chart records are not yet connected.".into(),
                            )))
                            .unwrap(),
                        })],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Saved Duet-scoped initial summary.",
                    )],
                    receipt: json!({"turn": 4}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut duet_host = PracticeJobHost {
        store: reopened.clone(),
        claim: duet_claim,
        authority: auth.as_ref(),
    };
    duet_host
        .run(
            &mut DuetOnlyProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    let duet_saved = reopened
        .patient_understanding_for_practice(&practice, &duet_ws, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        duet_saved.evidence_scope,
        EvidenceScope::DuetPatientSharedOnly
    );
    assert!(
        duet_saved
            .summary
            .narrative
            .contains("patient-shared Duet information only")
    );
}

/// AC3: A later source or correction updates the same patient's understanding
/// without losing unrelated information or creating a second patient
/// relationship. A subsequent question uses the revised understanding (and a
/// conversational correction in chat updates revision 3 for later questions).
#[tokio::test]
async fn ac3_later_source_and_chat_correction_update_same_patient_and_inform_subsequent_question() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-ac3-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-ac3-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();
    let (physician, session_hash) = invite_and_sign_in(auth.as_ref(), &practice, "Dr. Chen").await;
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let pull1 = SourcePull {
        source_id: "clinic-ehr".to_owned(),
        source_label: "Clinic EHR".to_owned(),
        patient: source_patient("ac3-pt", "Hannah Alverez"),
        pulled_at: Some(1_788_200_000),
    };
    let asthma = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "Condition/asthma-1",
        revision: "rev-asthma-1",
        record_type: "Condition",
        title: "Moderate Persistent Asthma",
        date: "2026-06-15",
        text: "Moderate persistent asthma well controlled on budesonide-formoterol inhaler; followed annually at Valley Pulmonology.",
    });
    let bmp_v1 = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "DiagnosticReport/bmp-1",
        revision: "rev-bmp-1",
        record_type: "DiagnosticReport",
        title: "Basic Metabolic Panel",
        date: "2026-09-01",
        text: "Potassium 3.9 mmol/L, Creatinine 0.88 mg/dL.",
    });
    store
        .record_chart(&workspace, &[asthma.clone(), bmp_v1], &pull1)
        .await
        .unwrap();

    // Run 1: Initial understanding (revision 1).
    let UnderstandingTriggerOutcome::Started { scope: scope1, .. } = store
        .trigger_patient_understanding(&practice, &workspace, &roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected initial run Started");
    };
    let claim1 = store
        .claim_practice_job(&scope1, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct InitialRunProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for InitialRunProvider {
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
                        id: "read-init".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [
                                {"candidate_id": "Condition/asthma-1", "revision": "rev-asthma-1", "cursor": null},
                                {"candidate_id": "DiagnosticReport/bmp-1", "revision": "rev-bmp-1", "cursor": null}
                            ],
                            "max_result_bytes": 32_768
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let args = SaveUnderstandingArgs {
                        expected_revision: Some(0),
                        operating_index: OperatingIndex {
                            working_overview: "PRIVATE_INDEX_SENTINEL_V1: Moderate persistent asthma; normal baseline potassium 3.9 mmol/L.".into(),
                            known_health_systems: vec![
                                KnownHealthSystem {
                                    name: "Clinic EHR".into(),
                                    standing: HealthSystemStanding::Connected {
                                        source_id: "clinic-ehr".into(),
                                    },
                                    note: "Primary connected outpatient EHR".into(),
                                },
                                KnownHealthSystem {
                                    name: "Valley Pulmonology".into(),
                                    standing: HealthSystemStanding::MentionedInRecords,
                                    note: "Annual pulmonology follow-up".into(),
                                },
                            ],
                            open_threads: vec![
                                "Pulmonary function test spirometry report from Valley Pulmonology missing".into(),
                            ],
                            anchors: vec![
                                IndexedAnchor {
                                    reference: ExactRecordReference {
                                        candidate_id: "Condition/asthma-1".into(),
                                        revision: "rev-asthma-1".into(),
                                    },
                                    source_id: "clinic-ehr".into(),
                                    topic: "Asthma baseline".into(),
                                },
                                IndexedAnchor {
                                    reference: ExactRecordReference {
                                        candidate_id: "DiagnosticReport/bmp-1".into(),
                                        revision: "rev-bmp-1".into(),
                                    },
                                    source_id: "clinic-ehr".into(),
                                    topic: "Metabolic panel".into(),
                                },
                            ],
                        },
                        claims: vec![
                            SummaryClaimInput {
                                text: "Moderate persistent asthma controlled on budesonide-formoterol".into(),
                                citations: vec![ExactRecordReference {
                                    candidate_id: "Condition/asthma-1".into(),
                                    revision: "rev-asthma-1".into(),
                                }],
                            },
                            SummaryClaimInput {
                                text: "Potassium was 3.9 mmol/L with creatinine 0.88 mg/dL on 2026-09-01".into(),
                                citations: vec![ExactRecordReference {
                                    candidate_id: "DiagnosticReport/bmp-1".into(),
                                    revision: "rev-bmp-1".into(),
                                }],
                            },
                        ],
                        material_gaps: vec![
                            "Valley Pulmonology spirometry report not in connected chart".into(),
                        ],
                        scope_note: None,
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "save-init".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: serde_json::to_string(&args).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant("Initial understanding saved.")],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut host1 = PracticeJobHost {
        store: store.clone(),
        claim: claim1,
        authority: auth.as_ref(),
    };
    host1
        .run(
            &mut InitialRunProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    // Deliver a corrected laboratory result (`rev-bmp-2`) on the SAME workspace!
    let pull2 = SourcePull {
        pulled_at: Some(1_788_300_000),
        ..pull1.clone()
    };
    let bmp_v2_corrected = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "DiagnosticReport/bmp-1",
        revision: "rev-bmp-2",
        record_type: "DiagnosticReport",
        title: "Basic Metabolic Panel (Corrected)",
        date: "2026-09-18",
        text: "Corrected laboratory result: Potassium 5.6 mmol/L (elevated, non-hemolyzed), Creatinine 0.90 mg/dL.",
    });
    store
        .record_chart(&workspace, &[asthma, bmp_v2_corrected], &pull2)
        .await
        .unwrap();

    // Trigger revision: must update the SAME patient relationship (zero duplicate workspaces).
    assert_eq!(
        auth.practice_patients(&practice).await.unwrap(),
        vec![workspace.clone()],
        "later source must never create a second patient relationship"
    );

    let triggered_rev = store
        .trigger_patient_understanding(&practice, &workspace, &roles, &skills, auth.as_ref())
        .await
        .unwrap();
    let UnderstandingTriggerOutcome::Started {
        scope: scope2,
        mode: mode2,
        source_version: sv2,
    } = triggered_rev
    else {
        panic!("expected Started for revised source, got {triggered_rev:?}");
    };
    assert_eq!(
        mode2,
        UnderstandingMode::Revision {
            prior_revision: 1,
            prior_source_version: 1,
        }
    );
    assert_eq!(sv2, 2);

    let job2 = store.practice_job(&scope2).await.unwrap().unwrap();
    assert_eq!(
        job2.assignment
            .understanding
            .as_ref()
            .unwrap()
            .changed_candidates,
        vec!["DiagnosticReport/bmp-1".to_owned()],
        "delta candidate list should name only the changed lab record"
    );

    let claim2 = store
        .claim_practice_job(&scope2, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    // Run 2 reads prior understanding + ONLY the changed record `DiagnosticReport/bmp-1@rev-bmp-2`,
    // carrying forward `Condition/asthma-1@rev-asthma-1` without re-reading the unchanged chart!
    struct RevisionRunProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for RevisionRunProvider {
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
                            id: "read-prior-memory".into(),
                            name: READ_UNDERSTANDING_TOOL.into(),
                            arguments: json!({}).to_string(),
                        }),
                        engine::Item::Call(engine::Invocation {
                            id: "read-delta-lab".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "DiagnosticReport/bmp-1",
                                    "revision": "rev-bmp-2",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let prior_json = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "read-prior-memory" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("prior understanding read back");
                    assert!(prior_json.contains("Valley Pulmonology"));
                    assert!(prior_json.contains("budesonide-formoterol"));

                    // Verify BOTH citing the stale superseded `rev-bmp-1` in `claims` AND keeping a stale
                    // `rev-bmp-1` anchor in `operating_index.anchors` are refused!
                    let stale_attempt = SaveUnderstandingArgs {
                        expected_revision: Some(1),
                        operating_index: OperatingIndex {
                            working_overview: "Attempt with stale citation".into(),
                            known_health_systems: vec![],
                            open_threads: vec![],
                            anchors: vec![],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "Stale potassium 3.9 mmol/L".into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "DiagnosticReport/bmp-1".into(),
                                revision: "rev-bmp-1".into(),
                            }],
                        }],
                        material_gaps: vec!["Valley Pulmonology spirometry missing".into()],
                        scope_note: None,
                    };
                    let stale_anchor_attempt = SaveUnderstandingArgs {
                        expected_revision: Some(1),
                        operating_index: OperatingIndex {
                            working_overview: "Attempt with stale operating_index anchor".into(),
                            known_health_systems: vec![],
                            open_threads: vec![],
                            anchors: vec![IndexedAnchor {
                                reference: ExactRecordReference {
                                    candidate_id: "DiagnosticReport/bmp-1".into(),
                                    revision: "rev-bmp-1".into(),
                                },
                                source_id: "clinic-ehr".into(),
                                topic: "Superseded lab anchor".into(),
                            }],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "Moderate persistent asthma controlled on budesonide-formoterol"
                                .into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "Condition/asthma-1".into(),
                                revision: "rev-asthma-1".into(),
                            }],
                        }],
                        material_gaps: vec!["Valley Pulmonology spirometry missing".into()],
                        scope_note: None,
                    };
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "save-stale".into(),
                                name: SAVE_UNDERSTANDING_TOOL.into(),
                                arguments: serde_json::to_string(&stale_attempt).unwrap(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "save-stale-anchor".into(),
                                name: SAVE_UNDERSTANDING_TOOL.into(),
                                arguments: serde_json::to_string(&stale_anchor_attempt).unwrap(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let stale_refusal = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "save-stale" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .unwrap();
                    assert!(
                        stale_refusal.contains("stale_source_revision"),
                        "superseded claim citation must be refused: {stale_refusal}"
                    );
                    let stale_anchor_refusal = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "save-stale-anchor" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .unwrap();
                    assert!(
                        stale_anchor_refusal.contains("stale_source_revision")
                            && stale_anchor_refusal.contains("DiagnosticReport/bmp-1"),
                        "superseded operating_index anchor must be refused: {stale_anchor_refusal}"
                    );

                    let revised_args = SaveUnderstandingArgs {
                        expected_revision: Some(1),
                        operating_index: OperatingIndex {
                            working_overview: "PRIVATE_INDEX_SENTINEL_V2: Moderate persistent asthma preserved; corrected potassium elevated at 5.6 mmol/L.".into(),
                            known_health_systems: vec![
                                KnownHealthSystem {
                                    name: "Clinic EHR".into(),
                                    standing: HealthSystemStanding::Connected {
                                        source_id: "clinic-ehr".into(),
                                    },
                                    note: "Primary connected outpatient EHR".into(),
                                },
                                KnownHealthSystem {
                                    name: "Valley Pulmonology".into(),
                                    standing: HealthSystemStanding::MentionedInRecords,
                                    note: "Annual pulmonology follow-up".into(),
                                },
                            ],
                            open_threads: vec![
                                "Valley Pulmonology spirometry report not in connected chart".into(),
                                "Repeat potassium after 2026-09-18 correction not yet resulted".into(),
                            ],
                            anchors: vec![
                                IndexedAnchor {
                                    reference: ExactRecordReference {
                                        candidate_id: "Condition/asthma-1".into(),
                                        revision: "rev-asthma-1".into(),
                                    },
                                    source_id: "clinic-ehr".into(),
                                    topic: "Asthma baseline".into(),
                                },
                                IndexedAnchor {
                                    reference: ExactRecordReference {
                                        candidate_id: "DiagnosticReport/bmp-1".into(),
                                        revision: "rev-bmp-2".into(),
                                    },
                                    source_id: "clinic-ehr".into(),
                                    topic: "Corrected hyperkalemia".into(),
                                },
                            ],
                        },
                        claims: vec![
                            SummaryClaimInput {
                                text: "Moderate persistent asthma controlled on budesonide-formoterol".into(),
                                citations: vec![ExactRecordReference {
                                    candidate_id: "Condition/asthma-1".into(),
                                    revision: "rev-asthma-1".into(),
                                }],
                            },
                            SummaryClaimInput {
                                text: "Corrected basic metabolic panel on 2026-09-18 showed hyperkalemia with potassium 5.6 mmol/L and creatinine 0.90 mg/dL".into(),
                                citations: vec![ExactRecordReference {
                                    candidate_id: "DiagnosticReport/bmp-1".into(),
                                    revision: "rev-bmp-2".into(),
                                }],
                            },
                        ],
                        material_gaps: vec![
                            "Valley Pulmonology spirometry report not in connected chart".into(),
                            "Repeat potassium confirmation not yet resulted".into(),
                        ],
                        scope_note: None,
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "save-rev-2".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: serde_json::to_string(&revised_args).unwrap(),
                        })],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Updated patient understanding to revision 2.",
                    )],
                    receipt: json!({"turn": 4}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut host2 = PracticeJobHost {
        store: store.clone(),
        claim: claim2,
        authority: auth.as_ref(),
    };
    host2
        .run(
            &mut RevisionRunProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    let rev2 = store
        .patient_understanding_for_practice(&practice, &workspace, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(rev2.revision, 2);
    assert_eq!(rev2.source_version, 2);
    assert!(
        rev2.summary.narrative.contains("budesonide-formoterol"),
        "unrelated asthma information must be preserved across lab update"
    );
    assert!(rev2.summary.narrative.contains("potassium 5.6 mmol/L"));
    assert!(!rev2.summary.narrative.contains("3.9 mmol/L"));
    assert!(
        rev2.operating_index
            .known_health_systems
            .iter()
            .any(|hs| hs.name == "Valley Pulmonology")
    );

    let history = store
        .patient_understanding_revisions(&practice, &workspace, auth.as_ref())
        .await
        .unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].revision, 1);
    assert_eq!(history[1].revision, 2);

    // Subsequent physician question uses the revised understanding (revision 2) AND applies a chat correction (revision 3)!
    let chat_scope_1 = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-subsequent-1".to_owned(),
    };
    let chat_recipe_1 = store
        .chat_recipe_with_understanding(
            &roles.chat,
            local_product::physician_chat::INSTRUCTIONS,
            &skills,
            &practice,
            &workspace,
        )
        .await
        .unwrap();
    model_text::assert_clean_model_text(
        &serde_json::to_string(&json!({
            "instructions": chat_recipe_1.instructions,
            "tools": chat_recipe_1.tools,
        }))
        .unwrap(),
    );

    assert_eq!(
        store
            .accept_authorized(
                &chat_scope_1,
                &Input {
                    key: "q-after-rev2".into(),
                    text: "What is Hannah's latest potassium, and note that her Valley Pulmonology spirometry was normal per phone report today.".into(),
                    timezone: "America/New_York".into(),
                    scenario: None,
                },
                session_hash,
                &skills,
                &chat_recipe_1,
                None,
            )
            .await
            .unwrap(),
        Accepted::New
    );
    let chat_claim_1 = store.claim(&chat_scope_1, 60).await.unwrap().unwrap();

    struct SubsequentQuestionAndCorrectionProvider {
        turn: usize,
        store: Store,
        claim: local_product::runtime::Claim,
    }
    impl engine::recipe::Provider for SubsequentQuestionAndCorrectionProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            // Verify chat receives revised understanding (revision 2: 5.6 mmol/L + asthma)!
            assert!(
                request.instructions.contains("\"revision\":2")
                    && request.instructions.contains("potassium 5.6 mmol/L")
                    && request.instructions.contains("budesonide-formoterol"),
                "subsequent chat question did not receive revised understanding: {}",
                request.instructions
            );
            match self.turn {
                1 => {
                    let corrected = SaveUnderstandingArgs {
                        expected_revision: Some(2),
                        operating_index: OperatingIndex {
                            working_overview: "PRIVATE_INDEX_SENTINEL_V3: Moderate persistent asthma (Valley Pulmonology spirometry normal per clinician phone note); corrected potassium 5.6 mmol/L.".into(),
                            known_health_systems: vec![
                                KnownHealthSystem {
                                    name: "Clinic EHR".into(),
                                    standing: HealthSystemStanding::Connected {
                                        source_id: "clinic-ehr".into(),
                                    },
                                    note: "Primary connected outpatient EHR".into(),
                                },
                                KnownHealthSystem {
                                    name: "Valley Pulmonology".into(),
                                    standing: HealthSystemStanding::MentionedInRecords,
                                    note: "Normal spirometry reported by physician on phone".into(),
                                },
                            ],
                            open_threads: vec![
                                "Repeat potassium confirmation not yet resulted".into(),
                            ],
                            anchors: vec![
                                IndexedAnchor {
                                    reference: ExactRecordReference {
                                        candidate_id: "Condition/asthma-1".into(),
                                        revision: "rev-asthma-1".into(),
                                    },
                                    source_id: "clinic-ehr".into(),
                                    topic: "Asthma baseline".into(),
                                },
                                IndexedAnchor {
                                    reference: ExactRecordReference {
                                        candidate_id: "DiagnosticReport/bmp-1".into(),
                                        revision: "rev-bmp-2".into(),
                                    },
                                    source_id: "clinic-ehr".into(),
                                    topic: "Corrected hyperkalemia".into(),
                                },
                            ],
                        },
                        claims: vec![
                            SummaryClaimInput {
                                text: "Moderate persistent asthma controlled on budesonide-formoterol, with normal Valley Pulmonology spirometry noted by Dr. Chen".into(),
                                citations: vec![ExactRecordReference {
                                    candidate_id: "Condition/asthma-1".into(),
                                    revision: "rev-asthma-1".into(),
                                }],
                            },
                            SummaryClaimInput {
                                text: "Corrected basic metabolic panel on 2026-09-18 showed potassium 5.6 mmol/L and creatinine 0.90 mg/dL".into(),
                                citations: vec![ExactRecordReference {
                                    candidate_id: "DiagnosticReport/bmp-1".into(),
                                    revision: "rev-bmp-2".into(),
                                }],
                            },
                        ],
                        material_gaps: vec![
                            "Repeat potassium confirmation not yet resulted".into(),
                        ],
                        scope_note: None,
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "chat-save-rev3".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: serde_json::to_string(&corrected).unwrap(),
                        })],
                        receipt: json!({"turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let mut recovery_chat_host = PhysicianHost {
                        store: self.store.clone(),
                        claim: self.claim.clone(),
                        authority: PostgresWorkspaceAuthority,
                        shown_steps: ShownSteps::new(()),
                    };
                    let saved_chat_receipt = engine::recipe::Product::saved_outcome(
                        &mut recovery_chat_host,
                        &engine::Invocation {
                            id: "chat-save-rev3".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: "{}".into(),
                        },
                    )
                    .await
                    .expect("PhysicianHost::saved_outcome reconciles committed chat save");
                    let Some(engine::ToolOutcome::Returned(saved_chat_json)) = saved_chat_receipt
                    else {
                        panic!("expected saved chat receipt, got {saved_chat_receipt:?}");
                    };
                    assert!(
                        saved_chat_json.contains("\"outcome\":\"saved\"")
                            && saved_chat_json.contains("\"duplicate\":true")
                            && saved_chat_json.contains("\"revision\":3"),
                        "unexpected PhysicianHost::saved_outcome payload: {saved_chat_json}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "finish-chat-rev3".into(),
                            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": "Hannah's corrected BMP from 2026-09-18 shows potassium 5.6 mmol/L (creatinine 0.90 mg/dL), and I updated her summary with the normal Valley Pulmonology spirometry report while keeping her asthma history.",
                                "quotes": [],
                                "descriptions": [],
                                "basis": "Updated patient understanding (revision 3) and physician-reported Valley Pulmonology spirometry report.",
                                "cites_none": true
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected chat turn {other}"),
            }
        }
    }

    let mut chat_host_1 = PhysicianHost {
        store: store.clone(),
        claim: chat_claim_1.clone(),
        authority: PostgresWorkspaceAuthority,
        shown_steps: ShownSteps::new(()),
    };
    chat_host_1
        .run(
            &mut SubsequentQuestionAndCorrectionProvider {
                turn: 0,
                store: store.clone(),
                claim: chat_claim_1,
            },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    let convo_1 = store.conversation(&chat_scope_1).await.unwrap();
    let answer_1 = convo_1.messages.last().unwrap().text.as_str();
    assert!(answer_1.contains("5.6 mmol/L"));
    assert!(!answer_1.contains("PRIVATE_INDEX_SENTINEL"));

    // Confirm revision 3 is saved with `RevisionOrigin::PhysicianConversation` and fed to a follow-up chat!
    let rev3 = store
        .patient_understanding_for_practice(&practice, &workspace, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(rev3.revision, 3);
    assert!(
        matches!(rev3.origin, RevisionOrigin::PhysicianConversation { .. }),
        "expected PhysicianConversation origin, got {:?}",
        rev3.origin
    );
    let follow_up_context = store
        .understanding_prompt_context(&practice, &workspace)
        .await
        .unwrap()
        .unwrap();
    assert!(follow_up_context.contains("\"revision\":3"));
    assert!(follow_up_context.contains("normal Valley Pulmonology spirometry noted by Dr. Chen"));
}

/// AC4: A failed first attempt retries through the existing run policy before
/// showing unable to finish (`WorkStatus::Failed` / `UnderstandingRunState::Failed`).
/// Repeated trigger events, interruption, and replayed `save_patient_understanding`
/// calls do not duplicate accepted jobs or revisions.
#[tokio::test]
async fn ac4_failed_first_attempt_retries_before_failed_standing_and_idempotency_prevents_duplicates()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-ac4-{}", Uuid::new_v4().simple());
    let ws_recover = format!("pw-ac4-recover-{}", Uuid::new_v4().simple());
    let ws_exhaust = format!("pw-ac4-exhaust-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &ws_recover).await.unwrap();
    auth.register_patient(&practice, &ws_exhaust).await.unwrap();

    for (ws, name) in [(&ws_recover, "Owen Harper"), (&ws_exhaust, "Tessa Gray")] {
        let pull = SourcePull {
            source_id: "clinic-ehr".to_owned(),
            source_label: "Clinic EHR".to_owned(),
            patient: source_patient(ws, name),
            pulled_at: Some(1_788_400_000),
        };
        let rec = chart_record(ChartRecordSpec {
            workspace: ws,
            source_id: "clinic-ehr",
            candidate_id: "Observation/bp-1",
            revision: "rev-bp-1",
            record_type: "Observation",
            title: "Blood Pressure",
            date: "2026-09-20",
            text: "Sitting blood pressure 148/92 mmHg.",
        });
        store.record_chart(ws, &[rec], &pull).await.unwrap();
    }

    // 1. Trigger ws_recover, and repeat the trigger event twice while pending: zero duplicate jobs!
    let UnderstandingTriggerOutcome::Started {
        scope: recover_scope,
        ..
    } = store
        .trigger_patient_understanding(&practice, &ws_recover, &roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected Started");
    };
    for _ in 0..2 {
        let dup = store
            .trigger_patient_understanding(&practice, &ws_recover, &roles, &skills, auth.as_ref())
            .await
            .unwrap();
        assert_eq!(
            dup,
            UnderstandingTriggerOutcome::AlreadyQueuedOrActive {
                scope: recover_scope.clone(),
                source_version: 1,
            }
        );
    }

    // 2. Claim attempt 1 and fail it (`AttemptFailure::Failed`): must retry (`Pending` / `Running`), NOT `Failed`!
    let attempt1 = store
        .claim_practice_job(&recover_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    store
        .settle_practice_job_attempt(
            &attempt1,
            engine::AttemptFailure::Failed,
            local_product::runtime::delivery::Backoff::IMMEDIATE,
        )
        .await
        .unwrap();

    let after_fail_1 = store.practice_job(&recover_scope).await.unwrap().unwrap();
    assert_eq!(after_fail_1.failures, 1);
    assert_eq!(after_fail_1.status, WorkStatus::Pending);
    assert_eq!(after_fail_1.attempts[0].status, AttemptStatus::Failed);

    let runs_after_1 = store
        .active_or_failed_understanding_runs(std::slice::from_ref(&ws_recover))
        .await
        .unwrap();
    assert_eq!(
        runs_after_1.get(&ws_recover).map(|r| r.state),
        Some(UnderstandingRunState::Running),
        "failed first attempt must stay Running while retries remain, never showing unable to finish early"
    );

    // 3. Claim attempt 2 and interrupt it (`AttemptFailure::Interrupted`): does not even increment failure count!
    let attempt2 = store
        .claim_practice_job(&recover_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert_ne!(attempt2.attempt, attempt1.attempt);
    store
        .settle_practice_job_attempt(
            &attempt2,
            engine::AttemptFailure::Interrupted,
            local_product::runtime::delivery::Backoff::IMMEDIATE,
        )
        .await
        .unwrap();
    let after_interrupt = store.practice_job(&recover_scope).await.unwrap().unwrap();
    assert_eq!(after_interrupt.failures, 1);
    assert_eq!(after_interrupt.status, WorkStatus::Pending);
    assert_eq!(
        after_interrupt.attempts[1].status,
        AttemptStatus::Interrupted
    );

    // 4. Claim attempt 3, execute save, and ALSO replay the exact same tool call idempotently -> one revision saved!
    let attempt3 = store
        .claim_practice_job(&recover_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct RetrySaveAndReplayProvider {
        turn: usize,
        store: Store,
        claim: local_product::runtime::jobs::PracticeJobClaim,
        auth: Arc<PostgresAuthStore>,
        replayed_call: Option<(engine::Invocation, Value)>,
    }
    impl engine::recipe::Provider for RetrySaveAndReplayProvider {
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
                        id: "read-bp".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Observation/bp-1",
                                "revision": "rev-bp-1",
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
                    let args = SaveUnderstandingArgs {
                        expected_revision: Some(0),
                        operating_index: OperatingIndex {
                            working_overview: "Elevated clinic BP 148/92 mmHg.".into(),
                            known_health_systems: vec![KnownHealthSystem {
                                name: "Clinic EHR".into(),
                                standing: HealthSystemStanding::Connected {
                                    source_id: "clinic-ehr".into(),
                                },
                                note: "Primary connected outpatient EHR".into(),
                            }],
                            open_threads: vec!["Ambulatory blood pressure log unavailable".into()],
                            anchors: vec![IndexedAnchor {
                                reference: ExactRecordReference {
                                    candidate_id: "Observation/bp-1".into(),
                                    revision: "rev-bp-1".into(),
                                },
                                source_id: "clinic-ehr".into(),
                                topic: "Clinic blood pressure".into(),
                            }],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "Sitting clinic blood pressure was 148/92 mmHg on 2026-09-20"
                                .into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/bp-1".into(),
                                revision: "rev-bp-1".into(),
                            }],
                        }],
                        material_gaps: vec!["Home blood pressure log unavailable".into()],
                        scope_note: None,
                    };
                    let val = serde_json::to_value(&args).unwrap();
                    let inv = engine::Invocation {
                        id: "idempotent-save-call".into(),
                        name: SAVE_UNDERSTANDING_TOOL.into(),
                        arguments: serde_json::to_string(&args).unwrap(),
                    };
                    self.replayed_call = Some((inv.clone(), val));
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(inv)],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let (inv, val) = self
                        .replayed_call
                        .as_ref()
                        .expect("save call recorded in turn 2");
                    let replay_outcome = self
                        .store
                        .execute_practice_understanding_tool(
                            &self.claim,
                            self.auth.as_ref(),
                            inv,
                            val,
                        )
                        .await
                        .expect("replayed save call succeeds idempotently");
                    let engine::ToolOutcome::Returned(replay_json) = replay_outcome else {
                        panic!("expected Returned on idempotent replay, got {replay_outcome:?}");
                    };
                    assert!(
                        replay_json.contains("\"duplicate\":true")
                            && replay_json.contains("\"revision\":1"),
                        "expected duplicate=true idempotent replay receipt, got: {replay_json}"
                    );
                    let mut recovery_host = PracticeJobHost {
                        store: self.store.clone(),
                        claim: self.claim.clone(),
                        authority: self.auth.as_ref(),
                    };
                    let recovered_outcome =
                        engine::recipe::Product::saved_outcome(&mut recovery_host, inv)
                            .await
                            .expect("PracticeJobHost::saved_outcome reconciles committed save");
                    let Some(engine::ToolOutcome::Returned(recovered_json)) = recovered_outcome
                    else {
                        panic!(
                            "expected saved_outcome Returned receipt, got {recovered_outcome:?}"
                        );
                    };
                    assert_eq!(recovered_json, replay_json);
                    let unstarted = engine::Invocation {
                        id: "never-committed-call".into(),
                        name: SAVE_UNDERSTANDING_TOOL.into(),
                        arguments: inv.arguments.clone(),
                    };
                    assert!(
                        engine::recipe::Product::saved_outcome(&mut recovery_host, &unstarted)
                            .await
                            .unwrap()
                            .is_none(),
                        "uncommitted call must return None from saved_outcome"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant("Recovered and saved on retry.")],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut provider = RetrySaveAndReplayProvider {
        turn: 0,
        store: store.clone(),
        claim: attempt3.clone(),
        auth: auth.clone(),
        replayed_call: None,
    };
    let mut host = PracticeJobHost {
        store: store.clone(),
        claim: attempt3.clone(),
        authority: auth.as_ref(),
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let completed_job = store.practice_job(&recover_scope).await.unwrap().unwrap();
    assert_eq!(completed_job.status, WorkStatus::Completed);
    assert_eq!(completed_job.attempts.len(), 3);
    assert_eq!(completed_job.attempts[0].status, AttemptStatus::Failed);
    assert_eq!(completed_job.attempts[1].status, AttemptStatus::Interrupted);
    assert_eq!(completed_job.attempts[2].status, AttemptStatus::Completed);

    let revisions = store
        .patient_understanding_revisions(&practice, &ws_recover, auth.as_ref())
        .await
        .unwrap();
    assert_eq!(revisions.len(), 1);
    assert!(
        store
            .active_or_failed_understanding_runs(std::slice::from_ref(&ws_recover))
            .await
            .unwrap()
            .is_empty(),
        "completed understanding run must clear active_or_failed_understanding_runs"
    );
    assert_eq!(
        store
            .trigger_patient_understanding(&practice, &ws_recover, &roles, &skills, auth.as_ref())
            .await
            .unwrap(),
        UnderstandingTriggerOutcome::AlreadyCurrent {
            revision: 1,
            source_version: 1,
        }
    );

    // 5. On `ws_exhaust`, fail all 5 attempts under the run policy -> only after attempt 5 does it report `Failed`!
    let UnderstandingTriggerOutcome::Started {
        scope: exhaust_scope,
        ..
    } = store
        .trigger_patient_understanding(&practice, &ws_exhaust, &roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected Started for ws_exhaust");
    };
    for attempt_no in 1..=5 {
        let claim = store
            .claim_practice_job(&exhaust_scope, 60, auth.as_ref())
            .await
            .unwrap()
            .expect("claimable while retries remain");
        store
            .settle_practice_job_attempt(
                &claim,
                engine::AttemptFailure::Failed,
                local_product::runtime::delivery::Backoff::IMMEDIATE,
            )
            .await
            .unwrap();
        let projected = store
            .active_or_failed_understanding_runs(std::slice::from_ref(&ws_exhaust))
            .await
            .unwrap();
        let run_status = projected.get(&ws_exhaust).expect("projected run status");
        if attempt_no < 5 {
            assert_eq!(
                run_status.state,
                UnderstandingRunState::Running,
                "attempt {attempt_no} must stay Running before retry budget is exhausted"
            );
        } else {
            assert_eq!(
                run_status.state,
                UnderstandingRunState::Failed,
                "attempt 5 exhausts retry policy and transitions to Failed (unable to finish)"
            );
            let wire = run_status.to_wire_automation_run("2026-09-20T12:00:00Z".into());
            assert_eq!(wire["kind"], "initial_understanding");
            assert_eq!(wire["state"], "failed");
        }
    }
    let exhausted_job = store.practice_job(&exhaust_scope).await.unwrap().unwrap();
    assert_eq!(exhausted_job.status, WorkStatus::Failed);
    assert_eq!(exhausted_job.failures, 5);
}

/// AC5 (runtime store boundary): Physician summary projections require panel
/// membership (`Membership::Allowed`) and physically omit every private
/// [`OperatingIndex`] field from both `PatientSummaryView` and `to_wire_synopsis`.
#[tokio::test]
async fn ac5_physician_projections_enforce_panel_access_and_exclude_private_operating_index() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-ac5-{}", Uuid::new_v4().simple());
    let foreign_practice = format!("practice-foreign-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-ac5-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.ensure_practice(&foreign_practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    let on_panel = PhysicianIdentity {
        practice_id: practice.clone(),
        physician_id: "dr-panel".to_owned(),
    };
    let clinic_colleague = PhysicianIdentity {
        practice_id: practice.clone(),
        physician_id: "dr-colleague".to_owned(),
    };
    let outsider = PhysicianIdentity {
        practice_id: foreign_practice.clone(),
        physician_id: "dr-outsider".to_owned(),
    };
    auth.set_membership(&on_panel, &workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&clinic_colleague, &workspace, Membership::ClinicDemographic)
        .await
        .unwrap();

    let pull = SourcePull {
        source_id: "clinic-ehr".to_owned(),
        source_label: "Clinic EHR".to_owned(),
        patient: source_patient("ac5-pt", "claire bennett"),
        pulled_at: Some(1_788_500_000),
    };
    let rec = chart_record(ChartRecordSpec {
        workspace: &workspace,
        source_id: "clinic-ehr",
        candidate_id: "Condition/hypothyroid-1",
        revision: "rev-hypo-1",
        record_type: "Condition",
        title: "Primary Hypothyroidism",
        date: "2026-08-02",
        text: "Primary hypothyroidism stable on levothyroxine 75 mcg daily; TSH 2.1 mIU/L.",
    });
    store.record_chart(&workspace, &[rec], &pull).await.unwrap();

    let UnderstandingTriggerOutcome::Started { scope, .. } = store
        .trigger_patient_understanding(&practice, &workspace, &roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected Started");
    };
    let claim = store
        .claim_practice_job(&scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct SentinelOperatingIndexProvider {
        turn: usize,
    }
    impl engine::recipe::Provider for SentinelOperatingIndexProvider {
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
                        id: "read-1".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "Condition/hypothyroid-1",
                                "revision": "rev-hypo-1",
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
                    let args = SaveUnderstandingArgs {
                        expected_revision: Some(0),
                        operating_index: OperatingIndex {
                            working_overview: "SECRET_OPERATING_ORIENTATION_9912".into(),
                            known_health_systems: vec![KnownHealthSystem {
                                name: "SECRET_OUTSIDE_SYSTEM_9912".into(),
                                standing: HealthSystemStanding::MentionedInRecords,
                                note: "SECRET_SYSTEM_NOTES_9912".into(),
                            }],
                            open_threads: vec![
                                "SECRET_INTERNAL_GAP_9912".into(),
                                "SECRET_ANCHOR_WHY_9912".into(),
                                "SECRET_MED_CUE_9912".into(),
                            ],
                            anchors: vec![IndexedAnchor {
                                reference: ExactRecordReference {
                                    candidate_id: "Condition/hypothyroid-1".into(),
                                    revision: "rev-hypo-1".into(),
                                },
                                source_id: "clinic-ehr".into(),
                                topic: "SECRET_ANCHOR_TOPIC_9912".into(),
                            }],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "Primary hypothyroidism stable on levothyroxine 75 mcg daily (TSH 2.1 mIU/L)".into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "Condition/hypothyroid-1".into(),
                                revision: "rev-hypo-1".into(),
                            }],
                        }],
                        material_gaps: vec!["Prior thyroid ultrasound report not in connected chart".into()],
                        scope_note: None,
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "save-sentinel".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: serde_json::to_string(&args).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant("Saved.")],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
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
        &mut SentinelOperatingIndexProvider { turn: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Off-panel clinic colleague (`ClinicDemographic`) and foreign physician (`Denied`) are refused!
    assert!(matches!(
        store
            .patient_summary_for_physician(auth.as_ref(), &clinic_colleague, &workspace)
            .await,
        Err(PhysicianSummaryAccessError::Unauthorized {
            membership: Membership::ClinicDemographic,
            ..
        })
    ));
    assert!(matches!(
        store
            .patient_summary_for_physician(auth.as_ref(), &outsider, &workspace)
            .await,
        Err(PhysicianSummaryAccessError::Unauthorized {
            membership: Membership::Denied,
            ..
        })
    ));

    // On-panel physician receives summary + citations, and ZERO private OperatingIndex sentinels!
    let view = store
        .patient_summary_for_physician(auth.as_ref(), &on_panel, &workspace)
        .await
        .unwrap()
        .expect("on-panel physician reads summary");
    let wire = view.to_wire_synopsis("2026-08-27T12:00:00Z".into());
    let serialized_view = serde_json::to_string(&view).unwrap();
    let serialized_wire = serde_json::to_string(&wire).unwrap();

    for forbidden in [
        "operating_index",
        "SECRET_OPERATING_ORIENTATION_9912",
        "SECRET_OUTSIDE_SYSTEM_9912",
        "SECRET_SYSTEM_NOTES_9912",
        "SECRET_ANCHOR_TOPIC_9912",
        "SECRET_ANCHOR_WHY_9912",
        "SECRET_INTERNAL_GAP_9912",
        "SECRET_MED_CUE_9912",
    ] {
        assert!(
            !serialized_view.contains(forbidden),
            "PatientSummaryView leaked private operating index token {forbidden}: {serialized_view}"
        );
        assert!(
            !serialized_wire.contains(forbidden),
            "wire patient_synopsis leaked private operating index token {forbidden}: {serialized_wire}"
        );
    }
    assert!(serialized_wire.contains("levothyroxine 75 mcg daily"));
    assert!(serialized_wire.contains("Prior thyroid ultrasound report not in connected chart"));
}

#[test]
fn recipe_instructions_tools_and_skill_catalog_pass_model_text_audit() {
    let skills = load_skills();
    let roles = distinct_roles();
    let context = local_product::initial_understanding::UnderstandingContext {
        workspace: "pw-audit-1".into(),
        mode: UnderstandingMode::Initial,
        target_source_version: 1,
        evidence_scope: EvidenceScope::ClinicalChart {
            connected_sources: vec![ConnectedSourceBrief {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                pulled_at: 1_787_846_400,
            }],
        },
        changed_candidates: vec!["DocumentReference/intake-1".into()],
    };
    let created = local_product::initial_understanding::assignment(context, &skills)
        .expect("build initial understanding assignment");
    let built = local_product::initial_understanding::recipe(&roles.background, &created, &skills);

    model_text::assert_clean_model_text(local_product::initial_understanding::INSTRUCTIONS);
    model_text::assert_clean_model_text(&built.instructions);
    model_text::assert_clean_model_text(&created.prompt);
    for tool in &built.tools {
        model_text::assert_clean_model_text(
            tool["description"].as_str().expect("tool description"),
        );
        let schema_text = serde_json::to_string(&tool["parameters"]).expect("schema json");
        model_text::assert_clean_model_text(&schema_text);
    }
    let preloaded_package = json!({
        "skills": created
            .preloaded_skills
            .iter()
            .map(|skill| json!({
                "name": skill.name,
                "content": skill.content,
            }))
            .collect::<Vec<_>>(),
    });
    model_text::assert_skill_tools(&preloaded_package, &built.tools, &[]);
}
