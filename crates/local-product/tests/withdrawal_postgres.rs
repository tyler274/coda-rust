//! Real Postgres proof for withdrawing durable memory, patient understanding
//! revisions, and concerns derived from a wrong-patient message: marking
//! affected memory and summary revisions `EnteredInError` while keeping audit
//! rows immutable, fencing late or replayed writes from restoring withdrawn
//! state, excluding withdrawn items from starting contexts while recording the
//! withdrawal boundary, and holding back affected concerns while preserving
//! unrelated concerns on the patient.

use std::path::PathBuf;
use std::sync::Arc;

use local_product::{
    ModelBinding, ModelRoles,
    initial_understanding::{
        HealthSystemStanding, IndexedAnchor, KnownHealthSystem, OperatingIndex,
        READ_UNDERSTANDING_TOOL, SAVE_UNDERSTANDING_TOOL, SaveUnderstandingArgs, SummaryClaimInput,
    },
    physician_chat::PhysicianSkills,
    physician_sources::{
        MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SourcePatient, SourceRevision,
    },
    runtime::{
        Authority, PublicationTransaction, Scope, Store, async_trait,
        chat_operations::{
            MemoryProvenance, MemorySelection, MemoryStanding, PatientMemoryError, PatientMemoryId,
            PatientMemoryStanding, RecordMemoryCorrection,
        },
        concerns::{
            CONCERN_TOOL, CommandKey, ConcernError, ConcernId, ConcernStanding, ConcernUrgency,
            ExactRecordReference, RaiseConcern, ReviseConcern, SourceWork, WorkspaceAccess,
        },
        engine::{self, Invocation, ToolOutcome},
        follow_ups::{
            SourceWithdrawalOutcome, WithdrawWrongPatientRequest, WithdrawalFenceViolation,
            WorkspaceWithdrawalBoundary,
        },
        jobs::PracticeJobHost,
        sources::SourcePull,
        understanding::{UnderstandingStanding, UnderstandingTriggerOutcome},
    },
    sha256_hex,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::{Value, json};
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../../verification/model_text.rs"]
mod model_text;

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

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
        .expect("load physician skills catalog")
}

fn roles() -> ModelRoles {
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

fn chart_record(
    workspace: &str,
    source_id: &str,
    candidate_id: &str,
    revision: &str,
    date: &str,
    title: &str,
    text: &str,
) -> SourceRevision {
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: "DocumentReference".into(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".into(),
            value: date.to_owned(),
        }],
        content: text.to_owned(),
        sha256: sha256_hex(text.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: title.to_owned(),
            text: text.to_owned(),
        }],
        provenance: json!({
            "title": title,
            "record_locator": candidate_id,
        }),
    }
}

fn source_pull(source_id: &str, label: &str, pt_id: &str, pulled_at: i64) -> SourcePull {
    SourcePull {
        source_id: source_id.to_owned(),
        source_label: label.to_owned(),
        patient: SourcePatient {
            source_patient_id: pt_id.to_owned(),
            display_name: "Clara Morales".to_owned(),
            birth_date: Some("1968-04-14".to_owned()),
            sex_or_gender: Some("female".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "clinic-mrn".to_owned(),
                value: format!("MRN-{pt_id}"),
            }],
        },
        pulled_at,
    }
}

async fn run_understanding_save_job(
    store: &Store,
    auth: &PostgresAuthStore,
    scope: &local_product::runtime::jobs::PracticeScope,
    refs_to_read: Vec<(&str, &str)>,
    save_args: SaveUnderstandingArgs,
) -> String {
    struct SaveJobProvider {
        turn: usize,
        refs_to_read: Vec<(String, String)>,
        save_args: SaveUnderstandingArgs,
        last_save_output: String,
    }
    impl engine::recipe::Provider for SaveJobProvider {
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
                    let references: Vec<Value> = self
                        .refs_to_read
                        .iter()
                        .map(|(cid, rev)| {
                            json!({
                                "candidate_id": cid,
                                "revision": rev,
                                "cursor": null
                            })
                        })
                        .collect();
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-sources".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": references,
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "save-understanding".into(),
                        name: SAVE_UNDERSTANDING_TOOL.into(),
                        arguments: serde_json::to_string(&self.save_args).unwrap(),
                    })],
                    receipt: json!({"turn": 2}),
                    usage: Default::default(),
                }),
                3 => {
                    self.last_save_output = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "save-understanding" =>
                            {
                                Some(output.clone())
                            }
                            _ => None,
                        })
                        .unwrap_or_default();
                    if !self.last_save_output.contains("\"outcome\":\"saved\"") {
                        return Err("understanding save refused".into());
                    }
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant("Completed understanding turn.")],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let claim = store
        .claim_practice_job(scope, 60, auth)
        .await
        .unwrap()
        .expect("claim understanding job");
    let mut provider = SaveJobProvider {
        turn: 0,
        refs_to_read: refs_to_read
            .into_iter()
            .map(|(c, r)| (c.to_owned(), r.to_owned()))
            .collect(),
        save_args,
        last_save_output: String::new(),
    };
    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: auth,
    };
    let run_res = host
        .run(
            &mut provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await;
    if provider.last_save_output.contains("\"outcome\":\"saved\"") {
        run_res.expect("understanding job run finishes");
    } else {
        assert!(
            run_res.is_err(),
            "refused save must not complete the understanding job"
        );
    }
    provider.last_save_output
}

fn make_save_args(
    expected_revision: u64,
    overview: &str,
    source_id: &str,
    candidate_id: &str,
    revision: &str,
    claim_text: &str,
) -> SaveUnderstandingArgs {
    SaveUnderstandingArgs {
        expected_revision: Some(expected_revision),
        operating_index: OperatingIndex {
            working_overview: overview.to_owned(),
            known_health_systems: vec![KnownHealthSystem {
                name: "Clinic EHR".into(),
                standing: HealthSystemStanding::Connected {
                    source_id: source_id.to_owned(),
                },
                note: "Primary care chart".into(),
            }],
            open_threads: vec!["Routine longitudinal care".into()],
            anchors: vec![IndexedAnchor {
                reference: ExactRecordReference {
                    candidate_id: candidate_id.to_owned(),
                    revision: revision.to_owned(),
                },
                source_id: source_id.to_owned(),
                topic: "Primary anchor".into(),
            }],
        },
        claims: vec![SummaryClaimInput {
            text: claim_text.to_owned(),
            citations: vec![ExactRecordReference {
                candidate_id: candidate_id.to_owned(),
                revision: revision.to_owned(),
            }],
        }],
        material_gaps: vec!["No outside specialty gaps identified".into()],
        scope_note: None,
    }
}

#[test]
fn withdrawal_boundary_instruction_is_clean_model_text() {
    let boundary = WorkspaceWithdrawalBoundary {
        latest_epoch: 2,
        candidate_ids: vec!["spruce-msg-wrong-1".into()],
        message_ids: vec!["msg-wrong-42".into()],
        withdrawn_understanding_revisions: vec![2],
    };
    let line = boundary.instruction_line();
    assert!(line.contains("spruce-msg-wrong-1"));
    assert!(line.contains("message:msg-wrong-42"));
    model_text::assert_clean_model_text(&line);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac1_and_ac2_wrong_patient_withdrawal_marks_memory_and_summary_entered_in_error_preserves_audit_and_fences_late_or_replayed_writes()
 {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .unwrap(),
    );
    let skills = load_skills();
    let model_roles = roles();

    let practice = format!("practice-wd-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-wd-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (physician, _session_hash) = invite_and_sign_in(&auth, &practice, "dr-morales").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    // 1. Seed clean record (`cand-clean-htn`) and save clean initial understanding `v1`.
    let clean_rec = chart_record(
        &workspace,
        "clinic-ehr",
        "cand-clean-htn",
        "rev-1",
        "2026-11-01",
        "Hypertension Follow-Up Note",
        "Essential hypertension controlled on amlodipine 5 mg daily.",
    );
    store
        .record_chart(
            &workspace,
            std::slice::from_ref(&clean_rec),
            &source_pull("clinic-ehr", "Clinic EHR", "pt-1", 1_790_000_000),
        )
        .await
        .unwrap();

    let UnderstandingTriggerOutcome::Started {
        scope: scope_v1, ..
    } = store
        .trigger_patient_understanding(&practice, &workspace, &model_roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected initial understanding job Started");
    };
    let save_v1_out = run_understanding_save_job(
        &store,
        auth.as_ref(),
        &scope_v1,
        vec![("cand-clean-htn", "rev-1")],
        make_save_args(
            0,
            "Essential hypertension on amlodipine 5 mg daily.",
            "clinic-ehr",
            "cand-clean-htn",
            "rev-1",
            "Essential hypertension controlled on amlodipine 5 mg daily",
        ),
    )
    .await;
    assert!(
        save_v1_out.contains("\"outcome\":\"saved\""),
        "{save_v1_out}"
    );

    // 2. Wrong-patient message (`cand-wrong-msg` / `msg-wrong-42`) arrives and contaminates
    // understanding `v2` plus two durable memory entries (`mem-wrong-msg` & `mem-wrong-summary`),
    // alongside an unrelated clean memory (`mem-clean-allergy`).
    let wrong_rec = chart_record(
        &workspace,
        "spruce",
        "cand-wrong-msg",
        "rev-w1",
        "2026-11-05",
        "Misrouted Inbound Message",
        "Wrong-patient report of mechanical aortic valve on warfarin 7.5 mg daily.",
    );
    store
        .record_chart(
            &workspace,
            std::slice::from_ref(&wrong_rec),
            &source_pull("spruce", "Spruce Messaging", "pt-1", 1_790_100_000),
        )
        .await
        .unwrap();

    let UnderstandingTriggerOutcome::Started {
        scope: scope_v2, ..
    } = store
        .trigger_patient_understanding(&practice, &workspace, &model_roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected revision understanding job Started");
    };
    let save_v2_out = run_understanding_save_job(
        &store,
        auth.as_ref(),
        &scope_v2,
        vec![("cand-clean-htn", "rev-1"), ("cand-wrong-msg", "rev-w1")],
        make_save_args(
            1,
            "Mechanical aortic valve on warfarin 7.5 mg daily (contaminated).",
            "spruce",
            "cand-wrong-msg",
            "rev-w1",
            "Mechanical aortic valve managed with warfarin 7.5 mg daily",
        ),
    )
    .await;
    assert!(
        save_v2_out.contains("\"outcome\":\"saved\""),
        "{save_v2_out}"
    );

    store
        .record_patient_memory(
            "mem-cmd-clean-1",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: PatientMemoryId::new("mem-clean-allergy").unwrap(),
                topic: "Penicillin allergy".into(),
                statement: "Childhood rash to amoxicillin; tolerates cephalexin.".into(),
                provenance: MemoryProvenance::PhysicianCorrection {
                    physician: physician.physician_id.clone(),
                    chat_id: "chat-clean-1".into(),
                    message_id: "msg-clean-10".into(),
                    work_id: "work-clean-chat".into(),
                    attempt_id: "attempt-1".into(),
                },
            },
        )
        .await
        .unwrap();

    let wrong_mem_corr = RecordMemoryCorrection {
        practice: practice.clone(),
        workspace: workspace.clone(),
        id: PatientMemoryId::new("mem-wrong-msg").unwrap(),
        topic: "Anticoagulation".into(),
        statement: "Takes warfarin 7.5 mg daily for mechanical valve.".into(),
        provenance: MemoryProvenance::PhysicianCorrection {
            physician: physician.physician_id.clone(),
            chat_id: "chat-wrong-1".into(),
            message_id: "msg-wrong-42".into(),
            work_id: "work-contaminated-chat".into(),
            attempt_id: "attempt-1".into(),
        },
    };
    store
        .record_patient_memory("mem-cmd-wrong-1", &wrong_mem_corr)
        .await
        .unwrap();

    let wrong_mem_summary = RecordMemoryCorrection {
        practice: practice.clone(),
        workspace: workspace.clone(),
        id: PatientMemoryId::new("mem-wrong-summary").unwrap(),
        topic: "Valve history".into(),
        statement: "Mechanical valve reported in inbound Spruce message.".into(),
        provenance: MemoryProvenance::ClinicalSummary {
            job_id: "job-contaminated".into(),
            work_id: "work-contaminated-job".into(),
            attempt_id: "attempt-1".into(),
            records: vec![ExactRecordReference {
                candidate_id: "cand-wrong-msg".into(),
                revision: "rev-w1".into(),
            }],
        },
    };
    store
        .record_patient_memory("mem-cmd-wrong-2", &wrong_mem_summary)
        .await
        .unwrap();

    // Queue a pre-withdrawal in-flight background job (`scope_inflight`) after a minor chart bump
    // so it is `Pending` when withdrawal runs and must be automatically fenced!
    let pre_withdraw_note = chart_record(
        &workspace,
        "clinic-ehr",
        "cand-clean-vitals",
        "rev-1",
        "2026-11-06",
        "Nurse Blood Pressure Check",
        "Seated blood pressure 126/78 mmHg.",
    );
    store
        .record_chart(
            &workspace,
            &[clean_rec.clone(), pre_withdraw_note.clone()],
            &source_pull("clinic-ehr", "Clinic EHR", "pt-1", 1_790_200_000),
        )
        .await
        .unwrap();
    let UnderstandingTriggerOutcome::Started {
        scope: scope_inflight,
        ..
    } = store
        .trigger_patient_understanding(&practice, &workspace, &model_roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected pre-withdrawal in-flight job Started");
    };

    // 3. Withdraw the wrong-patient message (`cand-wrong-msg` + `msg-wrong-42`).
    let withdrawal = store
        .withdraw_wrong_patient_source(&WithdrawWrongPatientRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            withdrawal_id: "withdraw-msg-42".into(),
            candidate_ids: vec!["cand-wrong-msg".into()],
            message_id: Some("msg-wrong-42".into()),
            fenced_work_ids: vec!["work-manual-stale".into()],
            reason: "Spruce message msg-wrong-42 belonged to a different patient.".into(),
            actor: physician.physician_id.clone(),
        })
        .await
        .unwrap();
    let SourceWithdrawalOutcome::Applied(receipt) = withdrawal else {
        panic!("expected Applied withdrawal receipt");
    };
    assert_eq!(receipt.withdrawal_epoch, 1);
    assert_eq!(receipt.withdrawn_understanding_revisions, vec![2]);
    assert_eq!(receipt.active_understanding_revision, Some(1));
    assert_eq!(receipt.withdrawn_memory_ids.len(), 2);

    // AC1: Active memory contains ONLY `mem-clean-allergy`; `All` preserves immutable
    // `EnteredInError` audit snapshots for both withdrawn memories.
    let active_memories = store
        .patient_memories(&practice, &workspace, MemorySelection::ActiveOnly)
        .await
        .unwrap();
    assert_eq!(active_memories.len(), 1);
    assert_eq!(active_memories[0].id.as_str(), "mem-clean-allergy");

    let all_memories = store
        .patient_memories(&practice, &workspace, MemorySelection::All)
        .await
        .unwrap();
    assert_eq!(all_memories.len(), 3);
    for id in ["mem-wrong-msg", "mem-wrong-summary"] {
        let entry = all_memories
            .iter()
            .find(|m| m.id.as_str() == id)
            .expect("withdrawn memory retained in audit read");
        assert!(matches!(
            &entry.standing,
            MemoryStanding::EnteredInError { withdrawal_id, .. }
                if withdrawal_id == "withdraw-msg-42"
        ));
    }

    // AC1: Immutable revision rows `v1` and `v2` are both preserved in audit, with `v1` Active
    // and `v2` EnteredInError, while active physician summary rolled back to `v1`.
    let rev_audits = store
        .patient_understanding_revision_audits(&practice, &workspace, auth.as_ref())
        .await
        .unwrap();
    assert_eq!(rev_audits.len(), 2);
    assert_eq!(rev_audits[0].record.revision, 1);
    assert_eq!(rev_audits[0].standing, UnderstandingStanding::Active);
    assert_eq!(rev_audits[1].record.revision, 2);
    assert!(matches!(
        &rev_audits[1].standing,
        UnderstandingStanding::EnteredInError {
            withdrawal_id,
            withdrawal_epoch: 1,
            ..
        } if withdrawal_id == "withdraw-msg-42"
    ));

    let active_summary = store
        .patient_summary_for_physician(auth.as_ref(), &physician, &workspace)
        .await
        .unwrap()
        .expect("rolled-back v1 summary remains active");
    assert_eq!(active_summary.revision, 1);
    assert!(
        !active_summary.summary.narrative.contains("warfarin"),
        "active summary must exclude withdrawn wrong-patient claim"
    );
    assert!(active_summary.summary.narrative.contains("amlodipine"));

    // AC2: Late or repeated writes CANNOT restore withdrawn memory or summary claims:
    // (a) Replaying original memory command key:
    assert!(matches!(
        store
            .record_patient_memory("mem-cmd-wrong-1", &wrong_mem_corr)
            .await,
        Err(PatientMemoryError::WithdrawnFence(_))
    ));
    // (b) Late write from fenced work ID (`work-manual-stale`) even with clean text:
    assert!(matches!(
        store
            .record_patient_memory(
                "mem-cmd-late-fenced",
                &RecordMemoryCorrection {
                    practice: practice.clone(),
                    workspace: workspace.clone(),
                    id: PatientMemoryId::new("mem-late-fenced").unwrap(),
                    topic: "Late write".into(),
                    statement: "Written by a pre-withdrawal fenced work run.".into(),
                    provenance: MemoryProvenance::PhysicianCorrection {
                        physician: physician.physician_id.clone(),
                        chat_id: "chat-clean-1".into(),
                        message_id: "msg-clean-99".into(),
                        work_id: "work-manual-stale".into(),
                        attempt_id: "attempt-2".into(),
                    },
                },
            )
            .await,
        Err(PatientMemoryError::WithdrawnFence(
            WithdrawalFenceViolation::FencedWork { .. }
        ))
    ));
    // (c) New memory citing `cand-wrong-msg` or `msg-wrong-42`:
    assert!(matches!(
        store
            .record_patient_memory(
                "mem-cmd-cite-withdrawn",
                &RecordMemoryCorrection {
                    practice: practice.clone(),
                    workspace: workspace.clone(),
                    id: PatientMemoryId::new("mem-fresh-bad-cite").unwrap(),
                    topic: "Bad cite".into(),
                    statement: "Attempting to cite withdrawn wrong-patient record.".into(),
                    provenance: MemoryProvenance::ClinicalSummary {
                        job_id: "job-fresh".into(),
                        work_id: "work-fresh".into(),
                        attempt_id: "attempt-1".into(),
                        records: vec![ExactRecordReference {
                            candidate_id: "cand-wrong-msg".into(),
                            revision: "rev-w1".into(),
                        }],
                    },
                },
            )
            .await,
        Err(PatientMemoryError::WithdrawnFence(
            WithdrawalFenceViolation::WithdrawnCandidate { .. }
        ))
    ));

    // (d) Pre-withdrawal in-flight understanding job (`scope_inflight`) is automatically fenced
    // even if it tries to save clean records!
    let fenced_save_output = run_understanding_save_job(
        &store,
        auth.as_ref(),
        &scope_inflight,
        vec![("cand-clean-htn", "rev-1"), ("cand-clean-vitals", "rev-1")],
        make_save_args(
            1,
            "Attempt from pre-withdrawal job to overwrite understanding.",
            "clinic-ehr",
            "cand-clean-vitals",
            "rev-1",
            "Seated blood pressure 126/78 mmHg",
        ),
    )
    .await;
    assert!(
        fenced_save_output.contains("withdrawn_fence"),
        "pre-withdrawal in-flight job must be fenced: {fenced_save_output}"
    );
    assert_eq!(
        store
            .patient_summary_for_physician(auth.as_ref(), &physician, &workspace)
            .await
            .unwrap()
            .unwrap()
            .revision,
        1,
        "fenced job must leave active understanding untouched at v1"
    );

    // (e) Fresh post-withdrawal run citing a clean record (`cand-clean-bmp`) succeeds cleanly
    // as revision `3` while keeping `v2` immutable as `EnteredInError`!
    let clean_bmp = chart_record(
        &workspace,
        "clinic-ehr",
        "cand-clean-bmp",
        "rev-1",
        "2026-11-07",
        "Basic Metabolic Panel",
        "Serum creatinine 0.92 mg/dL, potassium 4.2 mmol/L.",
    );
    store
        .record_chart(
            &workspace,
            &[],
            &source_pull("spruce", "Spruce Messaging", "pt-1", 1_790_250_000),
        )
        .await
        .unwrap();
    store
        .record_chart(
            &workspace,
            &[clean_rec, pre_withdraw_note, clean_bmp],
            &source_pull("clinic-ehr", "Clinic EHR", "pt-1", 1_790_300_000),
        )
        .await
        .unwrap();
    let UnderstandingTriggerOutcome::Started {
        scope: scope_clean_v3,
        ..
    } = store
        .trigger_patient_understanding(&practice, &workspace, &model_roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected post-withdrawal clean revision job Started");
    };
    let clean_v3_out = run_understanding_save_job(
        &store,
        auth.as_ref(),
        &scope_clean_v3,
        vec![("cand-clean-bmp", "rev-1")],
        make_save_args(
            1,
            "Hypertension controlled; BMP creatinine 0.92 mg/dL and potassium 4.2 mmol/L.",
            "clinic-ehr",
            "cand-clean-bmp",
            "rev-1",
            "Basic metabolic panel shows creatinine 0.92 mg/dL and potassium 4.2 mmol/L",
        ),
    )
    .await;
    assert!(
        clean_v3_out.contains("\"outcome\":\"saved\"") && clean_v3_out.contains("\"revision\":3"),
        "clean post-withdrawal save must advance monotonically to revision 3: {clean_v3_out}"
    );

    let audits_after_v3 = store
        .patient_understanding_revision_audits(&practice, &workspace, auth.as_ref())
        .await
        .unwrap();
    assert_eq!(audits_after_v3.len(), 3);
    assert_eq!(audits_after_v3[0].standing, UnderstandingStanding::Active);
    assert!(matches!(
        audits_after_v3[1].standing,
        UnderstandingStanding::EnteredInError { .. }
    ));
    assert_eq!(audits_after_v3[2].record.revision, 3);
    assert_eq!(audits_after_v3[2].standing, UnderstandingStanding::Active);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac3_starting_contexts_exclude_withdrawn_memory_and_summary_and_record_withdrawal_boundary_including_when_only_revision_withdrawn()
 {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .unwrap(),
    );
    let skills = load_skills();
    let model_roles = roles();

    let practice = format!("practice-ctx-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-ctx-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (physician, _hash) = invite_and_sign_in(&auth, &practice, "dr-chen").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    // Workspace where `v1` itself was derived from `cand-wrong-only` so withdrawing it clears
    // active understanding (`None`) while still recording the explicit withdrawal boundary!
    let wrong_only = chart_record(
        &workspace,
        "spruce",
        "cand-wrong-only",
        "rev-w1",
        "2026-11-08",
        "Wrong Patient Message",
        "Wrong-patient lithium carbonate 600 mg nightly report.",
    );
    let clean_after = chart_record(
        &workspace,
        "clinic-ehr",
        "cand-clean-lipid",
        "rev-1",
        "2026-11-09",
        "Lipid Panel",
        "LDL cholesterol 98 mg/dL.",
    );
    store
        .record_chart(
            &workspace,
            &[wrong_only],
            &source_pull("spruce", "Spruce Messaging", "pt-ctx", 1_791_000_000),
        )
        .await
        .unwrap();

    let UnderstandingTriggerOutcome::Started {
        scope: scope_v1, ..
    } = store
        .trigger_patient_understanding(&practice, &workspace, &model_roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected initial understanding job");
    };
    run_understanding_save_job(
        &store,
        auth.as_ref(),
        &scope_v1,
        vec![("cand-wrong-only", "rev-w1")],
        make_save_args(
            0,
            "Wrong-patient lithium carbonate 600 mg nightly.",
            "spruce",
            "cand-wrong-only",
            "rev-w1",
            "Lithium carbonate 600 mg nightly",
        ),
    )
    .await;

    store
        .record_patient_memory(
            "ctx-mem-wrong",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: PatientMemoryId::new("mem-lithium-wrong").unwrap(),
                topic: "Lithium".into(),
                statement: "Takes lithium carbonate 600 mg nightly.".into(),
                provenance: MemoryProvenance::PhysicianCorrection {
                    physician: physician.physician_id.clone(),
                    chat_id: "chat-ctx-1".into(),
                    message_id: "msg-wrong-ctx".into(),
                    work_id: "work-ctx-1".into(),
                    attempt_id: "attempt-1".into(),
                },
            },
        )
        .await
        .unwrap();

    store
        .record_patient_memory(
            "ctx-mem-clean",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: PatientMemoryId::new("mem-exercise-clean").unwrap(),
                topic: "Lifestyle".into(),
                statement: "Walks 30 minutes five mornings per week.".into(),
                provenance: MemoryProvenance::PhysicianCorrection {
                    physician: physician.physician_id.clone(),
                    chat_id: "chat-ctx-2".into(),
                    message_id: "msg-clean-ctx".into(),
                    work_id: "work-ctx-2".into(),
                    attempt_id: "attempt-1".into(),
                },
            },
        )
        .await
        .unwrap();

    // Withdraw `cand-wrong-only` and `msg-wrong-ctx`, then update retained chart to clean record only.
    store
        .withdraw_wrong_patient_source(&WithdrawWrongPatientRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            withdrawal_id: "withdraw-ctx-1".into(),
            candidate_ids: vec!["cand-wrong-only".into()],
            message_id: Some("msg-wrong-ctx".into()),
            fenced_work_ids: vec![],
            reason: "Wrong-patient message removed from workspace.".into(),
            actor: physician.physician_id.clone(),
        })
        .await
        .unwrap();
    store
        .record_chart(
            &workspace,
            &[clean_after],
            &source_pull("clinic-ehr", "Clinic EHR", "pt-ctx", 1_791_050_000),
        )
        .await
        .unwrap();

    // 1. `understanding_prompt_context` excludes `lithium` from summary/memory, keeps `Walks 30 minutes`,
    // and records `withdrawal_boundary`.
    let prompt_ctx = store
        .understanding_prompt_context(&practice, &workspace)
        .await
        .unwrap()
        .expect("prompt context present with clean memory + withdrawal boundary");
    assert!(!prompt_ctx.contains("lithium"));
    assert!(!prompt_ctx.contains("Lithium"));
    assert!(prompt_ctx.contains("Walks 30 minutes"));
    assert!(prompt_ctx.contains("withdrawal_boundary"));
    assert!(prompt_ctx.contains("cand-wrong-only"));
    model_text::assert_clean_model_text(&prompt_ctx);

    // 2. `READ_UNDERSTANDING_TOOL` inside a subsequent understanding run excludes withdrawn claims
    // and includes `withdrawal_boundary`.
    let UnderstandingTriggerOutcome::Started {
        scope: scope_next, ..
    } = store
        .trigger_patient_understanding(&practice, &workspace, &model_roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected subsequent understanding job");
    };
    let next_claim = store
        .claim_practice_job(&scope_next, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    let ToolOutcome::Returned(read_u_json) = store
        .execute_practice_understanding_tool(
            &next_claim,
            auth.as_ref(),
            &Invocation {
                id: "read-u-after".into(),
                name: READ_UNDERSTANDING_TOOL.into(),
                arguments: "{}".into(),
            },
            &json!({}),
        )
        .await
        .unwrap()
    else {
        panic!("expected Returned from read_patient_understanding");
    };
    assert!(!read_u_json.contains("lithium"));
    assert!(read_u_json.contains("Walks 30 minutes"));
    assert!(read_u_json.contains("\"withdrawal_boundary\""));
    assert!(read_u_json.contains("cand-wrong-only"));

    // 3. `patient_starting_context` and its `developer_text()` exclude withdrawn memory and summary,
    // retain clean memory, and inject the explicit withdrawal boundary instruction.
    let chat_scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-post-withdrawal".into(),
    };
    let starting = store.patient_starting_context(&chat_scope).await.unwrap();
    assert!(starting.understanding.is_none());
    let PatientMemoryStanding::Available { entries } = &starting.memory_standing else {
        panic!("expected Available memory standing");
    };
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id.as_str(), "mem-exercise-clean");

    let dev_text = starting.developer_text();
    assert!(!dev_text.contains("lithium"));
    assert!(dev_text.contains("Walks 30 minutes"));
    assert!(
        dev_text.contains("Withdrawn wrong-patient boundary (epoch 1)"),
        "starting context must surface withdrawal boundary even when active understanding is None: {dev_text}"
    );
    assert!(dev_text.contains("cand-wrong-only"));
    model_text::assert_clean_model_text(&dev_text);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac4_wrong_patient_withdrawal_holds_back_affected_concerns_fences_stale_runs_and_preserves_unrelated_concerns_and_audit()
 {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();

    let practice = Uuid::new_v4().to_string();
    let workspace = format!("pw-conc-{}", Uuid::new_v4().simple());
    let (physician, session_hash) = invite_and_sign_in(&auth, &practice, "dr-okafor").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &physician.physician_id,
        workspace: &workspace,
        session_hash: &session_hash,
    };

    let clean_rec = chart_record(
        &workspace,
        "clinic-ehr",
        "cand-clean-k",
        "rev-1",
        "2026-11-10",
        "Chemistry Panel",
        "Potassium 5.5 mmol/L on lisinopril.",
    );
    let wrong_rec = chart_record(
        &workspace,
        "spruce",
        "cand-wrong-troponin",
        "rev-w1",
        "2026-11-10",
        "Misrouted Message",
        "Wrong-patient chest pain message.",
    );
    store.record_source(&clean_rec).await.unwrap();
    store.record_source(&wrong_rec).await.unwrap();

    let clean_work = SourceWork::new("patient_review", "work-clean-k", "attempt-1").unwrap();
    let wrong_work = SourceWork::new("patient_review", "work-wrong-msg", "attempt-1").unwrap();

    store
        .raise_concern(
            &CommandKey::new("cmd-c-clean").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: ConcernId::new("concern-clean-k").unwrap(),
                urgency: ConcernUrgency::Today,
                reason: "Mild hyperkalemia 5.5 mmol/L on lisinopril.".into(),
                records: vec![ExactRecordReference {
                    candidate_id: "cand-clean-k".into(),
                    revision: "rev-1".into(),
                }],
                source_work: clean_work.clone(),
            },
        )
        .await
        .unwrap();

    let raise_wrong_call = Invocation {
        id: "call-raise-wrong".into(),
        name: CONCERN_TOOL.into(),
        arguments: json!({
            "operation": "raise",
            "concern_id": "concern-wrong-chest-pain",
            "urgency": "now",
            "reason": "Urgent chest pain reported in misrouted message.",
            "records": [{"candidate_id": "cand-wrong-troponin", "revision": "rev-w1"}]
        }),
    };
    store
        .execute_concern_tool(&practice, &workspace, &wrong_work, &raise_wrong_call)
        .await
        .unwrap();

    store
        .raise_concern(
            &CommandKey::new("cmd-c-fenced-work").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: ConcernId::new("concern-from-fenced-run").unwrap(),
                urgency: ConcernUrgency::Today,
                reason: "Secondary concern emitted by contaminated run.".into(),
                records: vec![ExactRecordReference {
                    candidate_id: "cand-clean-k".into(),
                    revision: "rev-1".into(),
                }],
                source_work: wrong_work.clone(),
            },
        )
        .await
        .unwrap();

    assert_eq!(
        store
            .open_concerns(&access, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap()
            .len(),
        3
    );

    // Withdraw `cand-wrong-troponin` and fence `work-wrong-msg`.
    let outcome = store
        .withdraw_wrong_patient_source(&WithdrawWrongPatientRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            withdrawal_id: "withdraw-conc-1".into(),
            candidate_ids: vec!["cand-wrong-troponin".into()],
            message_id: Some("msg-wrong-chest".into()),
            fenced_work_ids: vec!["work-wrong-msg".into()],
            reason: "Inbound chest pain message belonged to a different patient.".into(),
            actor: physician.physician_id.clone(),
        })
        .await
        .unwrap();
    assert_eq!(outcome.receipt().withdrawn_concern_ids.len(), 2);

    // Today open concerns retain ONLY `concern-clean-k`; Updates retired list excludes withdrawn concerns.
    let open_after = store
        .open_concerns(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(open_after.len(), 1);
    assert_eq!(open_after[0].id.as_str(), "concern-clean-k");

    let retired_updates = store
        .retired_concern_updates(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert!(
        retired_updates.is_empty(),
        "withdrawn wrong-patient concerns must not appear as clinical retirements in Updates"
    );

    // Full audit history in `workspace_concerns` preserves both withdrawn concerns as `ConcernStanding::Withdrawn`.
    let audit_all = store
        .workspace_concerns(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(audit_all.len(), 3);
    for withdrawn_id in ["concern-wrong-chest-pain", "concern-from-fenced-run"] {
        let item = audit_all
            .iter()
            .find(|c| c.id.as_str() == withdrawn_id)
            .expect("withdrawn concern preserved in workspace audit");
        assert!(matches!(
            &item.standing,
            ConcernStanding::Withdrawn { withdrawal_id, .. }
                if withdrawal_id == "withdraw-conc-1"
        ));
    }

    // Replaying `CONCERN_TOOL`, revising a withdrawn concern, raising from fenced `work-wrong-msg`,
    // or raising a new concern citing `cand-wrong-troponin` are all refused!
    let replay_tool = store
        .execute_concern_tool(&practice, &workspace, &wrong_work, &raise_wrong_call)
        .await
        .unwrap();
    assert!(matches!(replay_tool, ToolOutcome::Refused(_)));

    assert!(matches!(
        store
            .revise_concern(
                &CommandKey::new("cmd-revise-withdrawn").unwrap(),
                &ReviseConcern {
                    practice: practice.clone(),
                    workspace: workspace.clone(),
                    id: ConcernId::new("concern-wrong-chest-pain").unwrap(),
                    urgency: ConcernUrgency::Now,
                    reason: "Attempt to restore withdrawn concern.".into(),
                    records: vec![ExactRecordReference {
                        candidate_id: "cand-clean-k".into(),
                        revision: "rev-1".into(),
                    }],
                    source_work: clean_work.clone(),
                },
            )
            .await,
        Err(ConcernError::WithdrawnFence(_))
    ));

    assert!(matches!(
        store
            .raise_concern(
                &CommandKey::new("cmd-late-from-fenced-run").unwrap(),
                &RaiseConcern {
                    practice: practice.clone(),
                    workspace: workspace.clone(),
                    id: ConcernId::new("concern-late-fenced").unwrap(),
                    urgency: ConcernUrgency::Today,
                    reason: "Late concern from fenced pre-withdrawal run.".into(),
                    records: vec![ExactRecordReference {
                        candidate_id: "cand-clean-k".into(),
                        revision: "rev-1".into(),
                    }],
                    source_work: wrong_work,
                },
            )
            .await,
        Err(ConcernError::WithdrawnFence(
            WithdrawalFenceViolation::FencedWork { .. }
        ))
    ));

    assert!(matches!(
        store
            .raise_concern(
                &CommandKey::new("cmd-cite-withdrawn").unwrap(),
                &RaiseConcern {
                    practice: practice.clone(),
                    workspace: workspace.clone(),
                    id: ConcernId::new("concern-fresh-bad-cite").unwrap(),
                    urgency: ConcernUrgency::Today,
                    reason: "Fresh run citing withdrawn candidate.".into(),
                    records: vec![ExactRecordReference {
                        candidate_id: "cand-wrong-troponin".into(),
                        revision: "rev-w1".into(),
                    }],
                    source_work: clean_work,
                },
            )
            .await,
        Err(ConcernError::WithdrawnFence(
            WithdrawalFenceViolation::WithdrawnCandidate { .. }
        ))
    ));
}
