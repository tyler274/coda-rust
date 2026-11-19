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
    patient_reviews::{
        COMPLETE_PATIENT_REVIEW_TOOL, CompleteReviewArgs, CompleteReviewDisposition,
    },
    physician_chat::{ChatAssignment, PhysicianSkills},
    physician_sources::{
        MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
        SourceRevision,
    },
    runtime::{
        Accepted, Authority, Input, PhysicianHost, PublicationTransaction, Scope, Store,
        WorkStatus, async_trait,
        chat_operations::{PatientMemoryStanding, PriorWorkKind},
        concerns::{
            CONCERN_TOOL, CommandKey, ConcernError, ConcernId, ConcernStanding, ConcernUrgency,
            ExactRecordReference, RaiseConcern, ReviseConcern, SourceWork, WorkspaceAccess,
        },
        dispatch::StoredObservationChecker,
        engine::{
            self, Invocation, Item, ToolOutcome, Turn,
            recipe::{Observer, Provider, Request},
        },
        follow_ups::{
            FOLLOW_UP_TOOL, SourceWithdrawalOutcome, WithdrawWrongPatientRequest,
            WithdrawalFenceViolation, WorkspaceWithdrawalBoundary,
        },
        jobs::PracticeJobHost,
        memory::{
            Edit, FileRead, MemoryError, MemoryPath, NoteDraft, NoteOrigin, PatientKey,
            VersionedFile, Words,
        },
        reviews::PatientReviewReceiver,
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
        envelope: None,
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
        pulled_at: Some(pulled_at),
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
    let (physician, session_hash) = invite_and_sign_in(&auth, &practice, "dr-morales").await;
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

    let clean_mem_scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-clean-1".into(),
    };
    let clean_note = store
        .append_note(
            &clean_mem_scope,
            &NoteOrigin {
                work: "work-clean-chat".into(),
                call: "mem-call-clean-1".into(),
                message: "msg-clean-10".into(),
            },
            &NoteDraft {
                text: "Penicillin allergy: childhood rash to amoxicillin; tolerates cephalexin."
                    .into(),
                quote: None,
            },
        )
        .await
        .unwrap();

    let wrong_mem_scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-wrong-1".into(),
    };
    let wrong_note_origin = NoteOrigin {
        work: "work-contaminated-chat".into(),
        call: "mem-call-wrong-1".into(),
        message: "msg-wrong-42".into(),
    };
    let wrong_note_draft = NoteDraft {
        text: "Anticoagulation: takes warfarin 7.5 mg daily for mechanical valve.".into(),
        quote: None,
    };
    let wrong_note = store
        .append_note(&wrong_mem_scope, &wrong_note_origin, &wrong_note_draft)
        .await
        .unwrap();

    let wrong_summary_note = store
        .append_note(
            &wrong_mem_scope,
            &NoteOrigin {
                work: "work-contaminated-job".into(),
                call: "mem-call-wrong-2".into(),
                message: "cand-wrong-msg".into(),
            },
            &NoteDraft {
                text: "Valve history: mechanical valve reported in inbound Spruce message.".into(),
                quote: None,
            },
        )
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

    // Chart source withdrawal check (BEFORE any `record_chart` call):
    // `withdraw_wrong_patient_source` tombstones `cand-wrong-msg` in `physician_runtime_sources`
    // while keeping earlier historical source revision rows immutable in Postgres for audit!
    let post_withdraw_obs = store.sources(&workspace).await.unwrap();
    assert!(
        post_withdraw_obs
            .sources
            .iter()
            .all(|s| s.candidate_id != "cand-wrong-msg"),
        "withdrawn candidate must be excluded from active chart observation immediately upon withdrawal"
    );
    assert!(
        store
            .has_sources_outside(&workspace, "clinic-ehr")
            .await
            .unwrap(),
        "immutable historical `spruce` row must remain in `physician_runtime_sources` for audit"
    );

    // AC1: Active memory search & reads contain ONLY `clean_note` (`Penicillin allergy`) while
    // excluding both withdrawn notes; immutable `EnteredInError` audit snapshots remain preserved!
    let mem_key = PatientKey::for_rounds(&practice, &workspace).unwrap();
    let warfarin_hits = store.search_memory(&mem_key, "warfarin").await.unwrap();
    assert!(
        warfarin_hits.is_empty(),
        "withdrawn memory note must be excluded from memory search: {warfarin_hits:?}"
    );
    let allergy_hits = store.search_memory(&mem_key, "Penicillin").await.unwrap();
    assert_eq!(allergy_hits.len(), 1);
    let listing = store.list_memory(&mem_key).await.unwrap();
    assert_eq!(listing.log_days.len(), 1);
    let FileRead::Log { markdown: log_md } = store
        .read_memory(&mem_key, &MemoryPath::Log(listing.log_days[0]), None)
        .await
        .unwrap()
    else {
        panic!("expected Log markdown");
    };
    assert!(log_md.contains("Penicillin allergy"));
    assert!(!log_md.contains("warfarin"));

    let audit_snapshots = store
        .memory_withdrawal_snapshots(&practice, &workspace)
        .await
        .unwrap();
    assert_eq!(audit_snapshots.len(), 2);
    for expected_msg in ["msg-wrong-42", "cand-wrong-msg"] {
        let snap = audit_snapshots
            .iter()
            .find(|s| s.get("message").and_then(Value::as_str) == Some(expected_msg))
            .expect("withdrawn memory snapshot retained in explicit audit read");
        assert_eq!(snap["standing"]["kind"].as_str(), Some("entered_in_error"));
        assert_eq!(
            snap["standing"]["withdrawal_id"].as_str(),
            Some("withdraw-msg-42")
        );
    }
    assert_ne!(clean_note.id, wrong_note.id);
    assert_ne!(wrong_note.id, wrong_summary_note.id);

    // End-to-end `PhysicianHost` check: canonical memory search/read, `SEARCH_TOOL`, and `READ_TOOL`
    // exclude withdrawn notes and withdrawn chart sources!
    let verify_chat_scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-post-withdraw-verify".into(),
    };
    let verify_starting = store
        .patient_starting_context(&verify_chat_scope)
        .await
        .unwrap();
    let verify_recipe = ChatAssignment::for_patient(verify_chat_scope.clone(), verify_starting)
        .recipe(&model_roles, &skills);
    assert_eq!(
        store
            .start_chat_authorized(
                &verify_chat_scope,
                &Input {
                    key: "cmd-verify-post-wd".into(),
                    text:
                        "Check durable memory including retired items and check warfarin records."
                            .into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                session_hash.clone(),
                &skills,
                &verify_recipe,
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap(),
        Accepted::New
    );

    struct VerifyWithdrawnToolsChatProvider {
        turn: usize,
    }
    impl Provider for VerifyWithdrawnToolsChatProvider {
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
                            id: "mem-search-penicillin".into(),
                            name: "search_patient_memory".into(),
                            arguments: json!({
                                "query": "Penicillin"
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "mem-search-warfarin".into(),
                            name: "search_patient_memory".into(),
                            arguments: json!({
                                "query": "warfarin"
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "chart-search-warfarin".into(),
                            name: SEARCH_TOOL.into(),
                            arguments: json!({
                                "query": "warfarin"
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "chart-read-withdrawn".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "cand-wrong-msg",
                                    "revision": "rev-w1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "chart-read-clean".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "cand-clean-htn",
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
                    let out_for = |cid: &str| -> String {
                        request
                            .items
                            .iter()
                            .find_map(|item| match item {
                                Item::MaterializedResult { call_id, output } if call_id == cid => {
                                    Some(output.clone())
                                }
                                _ => None,
                            })
                            .unwrap_or_else(|| panic!("missing output for {cid}"))
                    };
                    let mem_clean_out = out_for("mem-search-penicillin");
                    assert!(
                        mem_clean_out.contains("Penicillin allergy"),
                        "search_patient_memory must retain clean memory note: {mem_clean_out}"
                    );
                    let mem_warfarin_out = out_for("mem-search-warfarin");
                    assert!(
                        !mem_warfarin_out.contains("warfarin")
                            && !mem_warfarin_out.contains("mechanical valve"),
                        "EnteredInError wrong-patient memory must never leak via search_patient_memory: {mem_warfarin_out}"
                    );

                    let search_out = out_for("chart-search-warfarin");
                    assert!(
                        !search_out.contains("cand-wrong-msg")
                            && !search_out.to_ascii_lowercase().contains("mechanical"),
                        "SEARCH_TOOL must exclude withdrawn wrong-patient source: {search_out}"
                    );

                    let read_wd_out = out_for("chart-read-withdrawn");
                    assert!(
                        !read_wd_out.to_ascii_lowercase().contains("warfarin"),
                        "READ_TOOL must not return clinical body of withdrawn wrong-patient source: {read_wd_out}"
                    );

                    let read_clean_out: Value =
                        serde_json::from_str(&out_for("chart-read-clean")).unwrap();
                    let clean_label = read_clean_out["results"][0]["cite_as"]
                        .as_str()
                        .expect("clean record carries a citation label");
                    assert_eq!(clean_label, "S2");

                    Ok(Turn {
                        items: vec![Item::Call(local_product::runtime::engine::Invocation {
                            id: "finish-wd-verify".into(),
                            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": format!("Active hypertension on amlodipine 5 mg daily [{clean_label}]; no warfarin records belong to this patient."),
                                "basis": format!("Clinic EHR note dated 2026-11-01 [{clean_label}]."),
                                "descriptions": [], "quotes": [{ "label": clean_label, "quote": "amlodipine" }],
                                "cites_none": false,
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected verify chat turn {other}"),
            }
        }
    }

    let verify_claim = store.claim(&verify_chat_scope, 60).await.unwrap().unwrap();
    let mut verify_host = PhysicianHost {
        store: store.clone(),
        claim: verify_claim,
        authority: PostgresWorkspaceAuthority,
        shown_steps: local_product::runtime::ShownSteps::new(()),
    };
    verify_host
        .run(
            &mut VerifyWithdrawnToolsChatProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(
        store.conversation(&verify_chat_scope).await.unwrap().status,
        WorkStatus::Completed
    );

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
    // (a) Replaying original wrong-patient note command key:
    assert!(matches!(
        store
            .append_note(&wrong_mem_scope, &wrong_note_origin, &wrong_note_draft)
            .await,
        Err(MemoryError::WithdrawnFence(_))
    ));
    // (b) Late write from fenced work ID (`work-manual-stale`) even with clean text:
    assert!(matches!(
        store
            .append_note(
                &clean_mem_scope,
                &NoteOrigin {
                    work: "work-manual-stale".into(),
                    call: "mem-call-late-fenced".into(),
                    message: "msg-clean-99".into(),
                },
                &NoteDraft {
                    text: "Written by a pre-withdrawal fenced work run.".into(),
                    quote: None,
                },
            )
            .await,
        Err(MemoryError::WithdrawnFence(
            WithdrawalFenceViolation::FencedWork { .. }
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

    let ctx_wrong_scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-ctx-1".into(),
    };
    store
        .append_note(
            &ctx_wrong_scope,
            &NoteOrigin {
                work: "work-ctx-1".into(),
                call: "ctx-mem-wrong".into(),
                message: "msg-wrong-ctx".into(),
            },
            &NoteDraft {
                text: "Lithium: takes lithium carbonate 600 mg nightly.".into(),
                quote: None,
            },
        )
        .await
        .unwrap();

    let ctx_clean_scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-ctx-2".into(),
    };
    store
        .append_note(
            &ctx_clean_scope,
            &NoteOrigin {
                work: "work-ctx-2".into(),
                call: "ctx-mem-clean".into(),
                message: "msg-clean-ctx".into(),
            },
            &NoteDraft {
                text: "Lifestyle: walks 30 minutes five mornings per week.".into(),
                quote: None,
            },
        )
        .await
        .unwrap();
    let ctx_mem_key = PatientKey::for_rounds(&practice, &workspace).unwrap();
    store
        .edit_memory(
            &ctx_mem_key,
            &VersionedFile::Summary,
            &[Ok(Edit::Create {
                text: Words::parse("Lifestyle: Walks 30 minutes five mornings per week.").unwrap(),
            })],
        )
        .await
        .unwrap();

    // Withdraw `cand-wrong-only` and `msg-wrong-ctx` directly; verify active chart observation
    // immediately excludes `cand-wrong-only` without any manual empty `spruce` pull!
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
    assert!(
        store.sources(&workspace).await.unwrap().sources.is_empty(),
        "withdrawing the only chart record must leave active chart observation empty"
    );

    // Register patient dispatch BEFORE delivering `clean_after` so `wake_practice_dispatch` wakes
    // a real `PatientReview` while active `understanding` is `None` and the withdrawal boundary is active!
    store
        .register_patient_dispatch(auth.as_ref(), &practice, &workspace, 1_791_025_000)
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

    // 0. Real `PatientReview` starting context (`ReviewStartingContext::developer_text()`) and
    // `PracticeJobHost` tools exclude withdrawn memory/chart items and surface the withdrawal boundary
    // even when active `understanding` is `None`!
    let checker = StoredObservationChecker::new(store.clone());
    let receiver = PatientReviewReceiver::new(&store, &model_roles, &skills, auth.as_ref());
    let wake = store
        .wake_practice_dispatch(auth.as_ref(), &practice, 1_791_050_000, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake.started_reviews.len(), 1);
    let reviews = store
        .patient_reviews(&practice, &workspace, auth.as_ref())
        .await
        .unwrap();
    let review_scope = reviews[0].practice_scope();
    let review_job = store
        .practice_job(&review_scope)
        .await
        .unwrap()
        .expect("saved review practice job");
    let review_ctx = review_job
        .assignment
        .review
        .as_ref()
        .expect("patient review starting context");
    assert!(
        review_ctx.starting_context.withdrawal_boundary.is_some(),
        "ReviewStartingContext must carry typed withdrawal_boundary"
    );
    assert!(
        review_ctx.starting_context.prior_reviews.is_empty(),
        "ReviewStartingContext.prior_reviews must never contain synthetic withdrawal entries"
    );
    let review_claim = store
        .claim_practice_job(&review_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct VerifyReviewContextAndToolsProvider {
        turn: usize,
    }
    impl Provider for VerifyReviewContextAndToolsProvider {
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
                    let starting_blob = serde_json::to_string(&request.items).unwrap();
                    assert!(
                        !starting_blob.to_ascii_lowercase().contains("lithium"),
                        "PatientReview starting context must exclude withdrawn wrong-patient claims and memory: {starting_blob}"
                    );
                    assert!(
                        starting_blob.contains("Walks 30 minutes"),
                        "PatientReview starting context must retain clean memory: {starting_blob}"
                    );
                    assert!(
                        starting_blob.contains("Entered-in-error withdrawal boundary:")
                            && starting_blob.contains("cand-wrong-only"),
                        "PatientReview starting context must surface withdrawal boundary even when understanding is None: {starting_blob}"
                    );
                    Ok(Turn {
                        items: vec![
                            Item::Call(Invocation {
                                id: "rev-mem-search-clean".into(),
                                name: "search_patient_memory".into(),
                                arguments: json!({
                                    "query": "Walks"
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "rev-mem-search-lithium".into(),
                                name: "search_patient_memory".into(),
                                arguments: json!({
                                    "query": "lithium"
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "rev-search-lithium".into(),
                                name: SEARCH_TOOL.into(),
                                arguments: json!({
                                    "query": "lithium"
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "rev-read-wrong".into(),
                                name: READ_TOOL.into(),
                                arguments: json!({
                                    "references": [{
                                        "candidate_id": "cand-wrong-only",
                                        "revision": "rev-w1",
                                        "cursor": null
                                    }],
                                    "max_result_bytes": 16_384
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "rev-read-lipid".into(),
                                name: READ_TOOL.into(),
                                arguments: json!({
                                    "references": [{
                                        "candidate_id": "cand-clean-lipid",
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
                    })
                }
                2 => {
                    let out_for = |cid: &str| -> String {
                        request
                            .items
                            .iter()
                            .find_map(|item| match item {
                                Item::MaterializedResult { call_id, output } if call_id == cid => {
                                    Some(output.clone())
                                }
                                _ => None,
                            })
                            .unwrap_or_else(|| panic!("missing output for {cid}"))
                    };
                    let mem_clean_out = out_for("rev-mem-search-clean");
                    assert!(
                        mem_clean_out.contains("walks 30 minutes"),
                        "PracticeJobHost search_patient_memory must return clean memory: {mem_clean_out}"
                    );
                    let mem_lithium_out = out_for("rev-mem-search-lithium");
                    assert!(
                        !mem_lithium_out.to_ascii_lowercase().contains("lithium"),
                        "PracticeJobHost search_patient_memory must never return EnteredInError memory: {mem_lithium_out}"
                    );
                    assert!(
                        !out_for("rev-search-lithium")
                            .to_ascii_lowercase()
                            .contains("lithium")
                    );
                    assert!(
                        !out_for("rev-read-wrong")
                            .to_ascii_lowercase()
                            .contains("lithium")
                    );

                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "LDL cholesterol 98 mg/dL at goal.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "cand-clean-lipid".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Clinic EHR lipid panel dated 2026-11-09.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "rev-complete".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(Turn {
                    items: vec![Item::assistant("Completed lipid review.")],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected review turn {other}"),
            }
        }
    }

    let mut review_host = PracticeJobHost {
        store: store.clone(),
        claim: review_claim,
        authority: auth.as_ref(),
    };
    review_host
        .run(
            &mut VerifyReviewContextAndToolsProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
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
    assert!(
        starting.withdrawal_boundary.is_some(),
        "PatientStartingContext must carry typed withdrawal_boundary"
    );
    assert!(
        !starting.recent_work.iter().any(|w| matches!(
            &w.target,
            PriorWorkKind::BackgroundJob { job_id, .. } if job_id.starts_with("withdrawal-epoch-")
        )),
        "PatientStartingContext.recent_work must never contain synthetic withdrawal pointers"
    );
    let PatientMemoryStanding::Available { log_days, .. } = &starting.memory_standing else {
        panic!("expected Available memory standing");
    };
    assert_eq!(log_days.len(), 1);

    let dev_text = starting.developer_text();
    assert!(!dev_text.contains("lithium"));
    assert!(dev_text.contains("Walks 30 minutes"));
    assert!(dev_text.contains("Entered-in-error withdrawal boundary:"));
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

    // Also seed a clean concern and clean follow-up that `wrong_work` retired/stopped before
    // withdrawal, so we can prove crash-recovery `saved_*_outcome` refuses fenced `work_id`s
    // even when the target item itself was never marked Withdrawn!
    store
        .raise_concern(
            &CommandKey::new("cmd-c-clean-retired-target").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: ConcernId::new("concern-clean-retired-target").unwrap(),
                urgency: ConcernUrgency::Today,
                reason: "Prior borderline potassium check.".into(),
                records: vec![ExactRecordReference {
                    candidate_id: "cand-clean-k".into(),
                    revision: "rev-1".into(),
                }],
                source_work: clean_work.clone(),
            },
        )
        .await
        .unwrap();
    let retire_by_fenced_call = Invocation {
        id: "call-retire-clean-by-fenced".into(),
        name: CONCERN_TOOL.into(),
        arguments: json!({
            "operation": "retire",
            "concern_id": "concern-clean-retired-target",
            "reason": "Superseded during pre-withdrawal run."
        }),
    };
    assert!(matches!(
        store
            .execute_concern_tool(&practice, &workspace, &wrong_work, &retire_by_fenced_call)
            .await
            .unwrap(),
        ToolOutcome::Returned(_)
    ));

    let create_clean_fu_call = Invocation {
        id: "call-create-clean-fu".into(),
        name: FOLLOW_UP_TOOL.into(),
        arguments: json!({
            "operation": "create",
            "follow_up_id": "fu-clean-completed-target",
            "summary": "Repeat basic metabolic panel in 2 weeks for potassium 5.5 mmol/L on lisinopril.",
            "due_at": 1_800_000_000_i64,
            "records": [{"candidate_id": "cand-clean-k", "revision": "rev-1"}]
        }),
    };
    assert!(matches!(
        store
            .execute_follow_up_tool(&practice, &workspace, &clean_work, &create_clean_fu_call)
            .await
            .unwrap(),
        ToolOutcome::Returned(_)
    ));
    let stop_by_fenced_call = Invocation {
        id: "call-complete-clean-fu-by-fenced".into(),
        name: FOLLOW_UP_TOOL.into(),
        arguments: json!({
            "operation": "complete",
            "follow_up_id": "fu-clean-completed-target",
            "resolution": "Completed during pre-withdrawal run."
        }),
    };
    assert!(matches!(
        store
            .execute_follow_up_tool(&practice, &workspace, &wrong_work, &stop_by_fenced_call)
            .await
            .unwrap(),
        ToolOutcome::Returned(_)
    ));

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

    // Today open concerns retain ONLY `concern-clean-k`; Updates retired list retains only the
    // non-withdrawn retirement while excluding both withdrawn concerns.
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
        retired_updates
            .iter()
            .all(|c| c.id.as_str() != "concern-wrong-chest-pain"
                && c.id.as_str() != "concern-from-fenced-run"),
        "withdrawn wrong-patient concerns must not appear as clinical retirements in Updates"
    );

    // Full audit history in `workspace_concerns` preserves all 4 concerns, with both wrong-patient
    // concerns marked `ConcernStanding::Withdrawn`.
    let audit_all = store
        .workspace_concerns(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(audit_all.len(), 4);
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

    // Replaying `CONCERN_TOOL`, crash-recovery `saved_concern_tool_outcome` /
    // `saved_follow_up_tool_outcome` for fenced `work-wrong-msg` (even on clean target items!),
    // revising a withdrawn concern, raising from fenced `work-wrong-msg`, or citing
    // `cand-wrong-troponin` are all refused!
    let replay_tool = store
        .execute_concern_tool(&practice, &workspace, &wrong_work, &raise_wrong_call)
        .await
        .unwrap();
    assert!(matches!(replay_tool, ToolOutcome::Refused(_)));

    let saved_fenced_concern = store
        .saved_concern_tool_outcome(
            &practice,
            &workspace,
            &wrong_work.work_id,
            &Invocation {
                id: retire_by_fenced_call.id.clone(),
                name: retire_by_fenced_call.name.clone(),
                arguments: retire_by_fenced_call.arguments.to_string(),
            },
        )
        .await
        .unwrap();
    assert!(
        matches!(saved_fenced_concern, Some(ToolOutcome::Refused(_))),
        "crash-recovery saved_concern_tool_outcome must refuse fenced work_id even when target concern is retired rather than Withdrawn: {saved_fenced_concern:?}"
    );

    let saved_fenced_follow_up = store
        .saved_follow_up_tool_outcome(
            &practice,
            &workspace,
            &wrong_work.work_id,
            &Invocation {
                id: stop_by_fenced_call.id.clone(),
                name: stop_by_fenced_call.name.clone(),
                arguments: stop_by_fenced_call.arguments.to_string(),
            },
        )
        .await
        .unwrap();
    assert!(
        matches!(saved_fenced_follow_up, Some(ToolOutcome::Refused(_))),
        "crash-recovery saved_follow_up_tool_outcome must refuse fenced work_id even when target follow-up is clean: {saved_fenced_follow_up:?}"
    );

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
