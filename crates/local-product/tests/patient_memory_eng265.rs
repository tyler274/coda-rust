//! Focused disposable-Postgres and structural proof for durable patient memory,
//! dated corrections, versioned search projections, and physician-facing summaries.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::OnceLock;

#[path = "common/disposable_postgres.rs"]
mod disposable_postgres;
use disposable_postgres::DisposablePostgres;

use local_product::ModelBinding;
use local_product::physician_chat::{self, PhysicianSkills};
use local_product::runtime::engine::recipe::{Observer, Provider, Request};
use local_product::runtime::memory::{
    ClinicalDate, CorrectionId, CorrectionStanding, JobMemoryCaller, MemoryAudience, MemoryAuthor,
    MemoryMutation, MemoryMutationOutcome, MemoryRead, MemorySearchDate, MemorySearchQuery,
    MemoryVersionNumber, OperatingIndexPatch, PatientAccess, PatientKey, PatientSummary,
    PermittedVersionsRead, PermittedVisibility, READ_MEMORY_TOOL, RECORD_CORRECTION_TOOL,
    RecordCorrection, RequiredAccess, RetractionDisposition, SAVE_MEMORY_TOOL, SAVE_SUMMARY_TOOL,
    SavedPatientSummary, SearchAudienceScope, SearchRead, SearchVersionScope,
    SearchableMemoryContent, SourceMessageRef, SummaryDates, SummaryRead, SummaryReference,
    TopicKey, VersionSelection, VersionStanding,
};
use local_product::runtime::{
    Accepted, Authority, Input, MessageRole, PhysicianHost, PublicationTransaction, Scope,
    async_trait, engine,
};
use serde_json::{Value, json};
use uuid::Uuid;

fn skills() -> &'static PhysicianSkills {
    static SKILLS: OnceLock<PhysicianSkills> = OnceLock::new();
    SKILLS.get_or_init(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../skills");
        PhysicianSkills::load(&root).expect("load workspace skills")
    })
}

fn chat_recipe() -> engine::recipe::RequestRecipe {
    let binding = ModelBinding {
        model: "fixture-resident".into(),
        effort: "high".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 250_000,
        adapter: "fixture".into(),
    };
    physician_chat::recipe(&binding, physician_chat::INSTRUCTIONS, skills())
}

struct AllowSession {
    expected_practice: String,
    expected_physician: String,
    allowed_workspaces: BTreeSet<String>,
}

impl AllowSession {
    fn single(scope: &Scope) -> Self {
        let mut allowed_workspaces = BTreeSet::new();
        allowed_workspaces.insert(scope.workspace.clone());
        Self {
            expected_practice: scope.practice.clone(),
            expected_physician: scope.physician.clone(),
            allowed_workspaces,
        }
    }

    fn verify(&self, scope: &Scope, session_hash: Option<&[u8]>) -> Result<(), String> {
        let Some(hash) = session_hash else {
            return Err("missing session reference".into());
        };
        if hash != [7_u8; 32] {
            return Err("revoked or unrecognized session".into());
        }
        if scope.practice != self.expected_practice {
            return Err("practice mismatch".into());
        }
        if scope.physician != self.expected_physician {
            return Err("physician mismatch".into());
        }
        if !self.allowed_workspaces.contains(&scope.workspace) {
            return Err("patient workspace not on physician panel".into());
        }
        Ok(())
    }
}

#[async_trait]
impl Authority for AllowSession {
    async fn check(
        &mut self,
        _tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.verify(scope, session_hash)
    }

    async fn lock_publication(
        &mut self,
        _tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.verify(scope, session_hash)
    }
}

struct ScriptedProvider {
    turns: Vec<engine::Turn>,
    seen_requests: Vec<Request>,
}

impl ScriptedProvider {
    fn new(turns: Vec<engine::Turn>) -> Self {
        Self {
            turns,
            seen_requests: Vec::new(),
        }
    }
}

impl Provider for ScriptedProvider {
    async fn exchange(
        &mut self,
        request: &Request,
        _cancel: &engine::Cancellation,
        _limits: &engine::RunLimits,
        _observations: &mut dyn Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        self.seen_requests.push(request.clone());
        if self.turns.is_empty() {
            return Err("scripted provider ran out of turns".into());
        }
        Ok(self.turns.remove(0))
    }
}

fn call_turn(calls: Vec<engine::Invocation>) -> engine::Turn {
    engine::Turn {
        items: calls.into_iter().map(engine::Item::Call).collect(),
        receipt: json!({ "turn": "tool_calls" }),
        usage: Default::default(),
    }
}

fn assistant_turn(text: &str) -> engine::Turn {
    engine::Turn {
        items: vec![engine::Item::assistant(text)],
        receipt: json!({ "turn": "assistant" }),
        usage: Default::default(),
    }
}

fn topic(name: &str) -> TopicKey {
    TopicKey::parse(name).expect("valid topic key")
}

fn date(value: &str) -> ClinicalDate {
    ClinicalDate::parse(value).expect("valid clinical date")
}

#[tokio::test]
async fn ac1_direct_and_tool_corrections_survive_postgres_restart_with_unrelated_understanding_intact()
 {
    let Some(mut pg) = DisposablePostgres::acquire_restartable("patient-memory-ac1").await else {
        return;
    };
    let store = pg.store().await;

    let practice = format!("prac-{}", Uuid::new_v4().simple());
    let workspace = "patient-claire".to_owned();
    let scope_seed = Scope {
        practice: practice.clone(),
        physician: "dr-nguyen".into(),
        workspace: workspace.clone(),
        chat: "chat-seed".into(),
    };
    let session = [7_u8; 32];
    let recipe = chat_recipe();

    // Turn 1: Seed private operating index across four topics AND a per-conversation
    // harness working note, proving patient memory and working_notes remain separate.
    assert_eq!(
        store
            .accept_authorized(
                &scope_seed,
                &Input {
                    key: "msg-seed-1".into(),
                    text: "Review initial hypertension and allergy history.".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                session.to_vec(),
                skills(),
                &recipe,
            )
            .await
            .unwrap(),
        Accepted::New
    );
    let claim_seed = store.claim(&scope_seed, 60).await.unwrap().unwrap();
    let mut seed_provider = ScriptedProvider::new(vec![
        call_turn(vec![
            engine::Invocation {
                id: "call-working-note-1".into(),
                name: "save_working_note".into(),
                arguments: json!({
                    "text": "WINDOW_SCRATCHPAD_ONLY: compare ambulatory readings next turn"
                })
                .to_string(),
            },
            engine::Invocation {
                id: "call-save-mem-1".into(),
                name: SAVE_MEMORY_TOOL.into(),
                arguments: json!({
                    "upsert": [
                        { "topic": "allergies", "note": "Chart lists penicillin rash (unverified)." },
                        { "topic": "cardiology", "note": "Stage 1 hypertension on amlodipine 5 mg daily." },
                        { "topic": "social_context", "note": "Night-shift nurse; prefers morning lab draws." },
                        { "topic": "uncertainties", "note": "Confirm whether home BP cuff was calibrated in 2026." }
                    ],
                    "remove": []
                })
                .to_string(),
            },
        ]),
        assistant_turn("Initial review complete."),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: claim_seed,
        authority: AllowSession::single(&scope_seed),
    }
    .run(
        &mut seed_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Direct physician operation: record a dated correction on allergies while patching
    // only the `allergies` topic.
    let mut patch_allergies = BTreeMap::new();
    patch_allergies.insert(
        topic("allergies"),
        "Confirmed anaphylaxis to amoxicillin in 2019; avoid penicillins.".into(),
    );
    let direct_mutation = MemoryMutation::RecordCorrection(RecordCorrection {
        correction_id: Some(CorrectionId::parse("corr-direct-allergy").unwrap()),
        recorded_on: date("2026-11-10"),
        source_message: SourceMessageRef::new("chat-direct", "msg-direct-101", "dr-nguyen")
            .unwrap(),
        statement: "Reaction to amoxicillin in 2019 was anaphylaxis, not a mild rash.".into(),
        supersedes: None,
        understanding_patch: Some(
            OperatingIndexPatch::new(patch_allergies, BTreeSet::new()).unwrap(),
        ),
    });
    let direct_receipt = match store
        .mutate_patient_memory_authorized(
            &scope_seed,
            "cmd-direct-corr-1",
            &session,
            direct_mutation.clone(),
            &mut AllowSession::single(&scope_seed),
        )
        .await
        .unwrap()
    {
        MemoryMutationOutcome::Applied(receipt) => receipt,
        other => panic!("expected Applied, got {other:?}"),
    };
    assert_eq!(direct_receipt.version.get(), 2);

    // Chat job tool operation: physician sends a correction in a second chat;
    // `record_patient_correction` captures the authored message ID (`msg-chat-corr-202`)
    // automatically when `source_message_id` is null and patches only `cardiology`.
    let scope_corr_chat = Scope {
        practice: practice.clone(),
        physician: "dr-nguyen".into(),
        workspace: workspace.clone(),
        chat: "chat-cardiology-followup".into(),
    };
    assert_eq!(
        store
            .accept_authorized(
                &scope_corr_chat,
                &Input {
                    key: "msg-chat-corr-202".into(),
                    text: "She stopped amlodipine due to ankle edema and takes losartan 50 mg daily as of 2026-11-11.".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                session.to_vec(),
                skills(),
                &recipe,
            )
            .await
            .unwrap(),
        Accepted::New
    );
    let claim_corr = store.claim(&scope_corr_chat, 60).await.unwrap().unwrap();
    let mut corr_provider = ScriptedProvider::new(vec![
        call_turn(vec![engine::Invocation {
            id: "call-tool-corr-2".into(),
            name: RECORD_CORRECTION_TOOL.into(),
            arguments: json!({
                "correction_id": "corr-tool-meds",
                "recorded_on": "2026-11-11",
                "statement": "Stopped amlodipine due to ankle edema; taking losartan 50 mg daily.",
                "source_message_id": null,
                "source_chat_id": null,
                "source_physician_id": null,
                "supersedes_correction_id": null,
                "upsert_topics": [
                    { "topic": "cardiology", "note": "Stage 1 hypertension treated with losartan 50 mg daily; prior amlodipine caused ankle edema." }
                ],
                "remove_topics": []
            })
            .to_string(),
        }]),
        assistant_turn("Noted: updated hypertension regimen to losartan 50 mg daily."),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: claim_corr,
        authority: AllowSession::single(&scope_corr_chat),
    }
    .run(
        &mut corr_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Practice-owned background job tool operation: records a third correction without
    // touching any operating-index topics.
    let patient = PatientKey::new(&practice, &workspace).unwrap();
    let job_caller = JobMemoryCaller::new("periodic_review", "job-review-42").unwrap();
    let job_call = engine::Invocation {
        id: "job-call-corr-3".into(),
        name: RECORD_CORRECTION_TOOL.into(),
        arguments: String::new(),
    };
    let job_args = json!({
        "correction_id": "corr-job-cuff",
        "recorded_on": "2026-11-12T09:30:00Z",
        "statement": "Home BP cuff was verified accurate in clinic on 2026-11-12.",
        "source_message_id": "msg-clinic-note-303",
        "source_chat_id": "chat-clinic-check",
        "source_physician_id": "dr-patel",
        "supersedes_correction_id": null,
        "upsert_topics": [],
        "remove_topics": []
    });
    let job_outcome = store
        .execute_job_memory_tool(&patient, &job_caller, &job_call, job_args.clone())
        .await
        .unwrap();
    let job_json: Value = serde_json::from_str(job_outcome.text().expect("tool text")).unwrap();
    assert_eq!(job_json["outcome"], "saved");
    assert_eq!(job_json["receipt"]["version"], 4);

    // Drop the store and crash-restart the disposable Postgres server.
    drop(store);
    pg.crash_restart();
    let restarted = pg.store().await;

    // Read back current memory through both the typed store API and `read_patient_memory`
    // in a brand-new chat (which has no working_notes).
    let MemoryRead::Available(snapshot) = restarted
        .patient_memory_authorized(
            &scope_corr_chat,
            &session,
            VersionSelection::Current,
            &mut AllowSession::single(&scope_corr_chat),
        )
        .await
        .unwrap()
    else {
        panic!("expected current patient memory after Postgres restart");
    };

    assert_eq!(snapshot.version.get(), 4);
    assert_eq!(snapshot.memory_id, expected_memory_id(&patient));
    // Unrelated understanding topics (`social_context` and `uncertainties`) survived untouched!
    assert_eq!(
        snapshot.operating_index.get(&topic("social_context")),
        Some("Night-shift nurse; prefers morning lab draws.")
    );
    assert_eq!(
        snapshot.operating_index.get(&topic("uncertainties")),
        Some("Confirm whether home BP cuff was calibrated in 2026.")
    );
    // Patched topics reflect their respective updates.
    assert_eq!(
        snapshot.operating_index.get(&topic("allergies")),
        Some("Confirmed anaphylaxis to amoxicillin in 2019; avoid penicillins.")
    );
    assert_eq!(
        snapshot.operating_index.get(&topic("cardiology")),
        Some(
            "Stage 1 hypertension treated with losartan 50 mg daily; prior amlodipine caused ankle edema."
        )
    );
    // Harness working_notes never leaked into patient memory.
    let serialized_snapshot = serde_json::to_string(&snapshot).unwrap();
    assert!(!serialized_snapshot.contains("WINDOW_SCRATCHPAD_ONLY"));

    // All three corrections survived restart with exact dates, source messages, and Active standing.
    assert_eq!(snapshot.corrections.len(), 3);
    assert_eq!(
        snapshot.corrections[0].correction_id.as_str(),
        "corr-direct-allergy"
    );
    assert_eq!(snapshot.corrections[0].recorded_on.as_str(), "2026-11-10");
    assert_eq!(
        snapshot.corrections[0].source_message,
        SourceMessageRef::new("chat-direct", "msg-direct-101", "dr-nguyen").unwrap()
    );
    assert_eq!(snapshot.corrections[0].standing, CorrectionStanding::Active);

    assert_eq!(
        snapshot.corrections[1].correction_id.as_str(),
        "corr-tool-meds"
    );
    assert_eq!(snapshot.corrections[1].recorded_on.as_str(), "2026-11-11");
    assert_eq!(
        snapshot.corrections[1].source_message,
        SourceMessageRef::new("chat-cardiology-followup", "msg-chat-corr-202", "dr-nguyen")
            .unwrap()
    );
    assert_eq!(snapshot.corrections[1].standing, CorrectionStanding::Active);

    assert_eq!(
        snapshot.corrections[2].correction_id.as_str(),
        "corr-job-cuff"
    );
    assert_eq!(
        snapshot.corrections[2].recorded_on.calendar_date(),
        "2026-11-12"
    );
    assert_eq!(
        snapshot.corrections[2].source_message,
        SourceMessageRef::new("chat-clinic-check", "msg-clinic-note-303", "dr-patel").unwrap()
    );

    // Compact all command receipts to salted digests, then verify both direct and tool
    // retries remain once-only idempotent and conflicting reuse is refused.
    let compacted = restarted.compact_receipts(0, 100).await.unwrap();
    assert!(
        compacted >= 3,
        "expected compacted receipts, got {compacted}"
    );

    assert_eq!(
        restarted
            .mutate_patient_memory_authorized(
                &scope_seed,
                "cmd-direct-corr-1",
                &session,
                direct_mutation,
                &mut AllowSession::single(&scope_seed),
            )
            .await
            .unwrap(),
        MemoryMutationOutcome::Duplicate(direct_receipt)
    );

    let duplicate_job = restarted
        .execute_job_memory_tool(&patient, &job_caller, &job_call, job_args)
        .await
        .unwrap();
    let duplicate_job_json: Value =
        serde_json::from_str(duplicate_job.text().expect("tool text")).unwrap();
    assert_eq!(duplicate_job_json["outcome"], "duplicate");
    assert_eq!(duplicate_job_json["receipt"]["version"], 4);

    let mut conflicting_upsert = BTreeMap::new();
    conflicting_upsert.insert(topic("allergies"), "different payload".into());
    assert_eq!(
        restarted
            .mutate_patient_memory_authorized(
                &scope_seed,
                "cmd-direct-corr-1",
                &session,
                MemoryMutation::UpdateUnderstanding(
                    OperatingIndexPatch::new(conflicting_upsert, BTreeSet::new()).unwrap()
                ),
                &mut AllowSession::single(&scope_seed),
            )
            .await
            .unwrap(),
        MemoryMutationOutcome::IdempotencyConflict
    );

    // Confirm version stayed at 4 after all retries/conflicts, and `read_patient_memory`
    // in a fresh chat sees the full understanding and corrections.
    let read_tool_out = restarted
        .execute_job_memory_tool(
            &patient,
            &job_caller,
            &engine::Invocation {
                id: "job-read-after-restart".into(),
                name: READ_MEMORY_TOOL.into(),
                arguments: String::new(),
            },
            json!({ "version": null }),
        )
        .await
        .unwrap();
    let read_json: Value = serde_json::from_str(read_tool_out.text().unwrap()).unwrap();
    assert_eq!(read_json["outcome"], "available");
    assert_eq!(read_json["snapshot"]["version"], 4);
    assert_eq!(read_json["permitted_versions"].as_array().unwrap().len(), 4);
}

fn expected_memory_id(patient: &PatientKey) -> local_product::runtime::memory::MemoryId {
    local_product::runtime::memory::MemoryId::for_patient(patient)
}

#[tokio::test]
async fn ac1_crash_between_tool_commit_and_harness_ack_reconciles_receipt_via_saved_outcome() {
    let Some(pg) = DisposablePostgres::acquire("patient-memory-recovery").await else {
        return;
    };
    let store = pg.store().await;

    let scope = Scope {
        practice: format!("prac-{}", Uuid::new_v4().simple()),
        physician: "dr-lin".into(),
        workspace: "patient-recovery".into(),
        chat: "chat-recovery".into(),
    };
    let session = [7_u8; 32];
    let recipe = chat_recipe();

    store
        .accept_authorized(
            &scope,
            &Input {
                key: "msg-rec-1".into(),
                text: "Record that penicillin tolerance was confirmed by oral challenge on 2026-10-04."
                    .into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            session.to_vec(),
            skills(),
            &recipe,
        )
        .await
        .unwrap();

    // Attempt 1 admits the recipe, persists `ProviderStarted`, `ProviderReturned`, and
    // `ToolStarted`, and commits the patient-memory mutation to Postgres, then crashes
    // BEFORE persisting `ToolReturned`.
    let claim_1 = store.claim(&scope, 60).await.unwrap().unwrap();
    let state = store.conversation(&scope).await.unwrap();
    let user_msg_id = state.messages[0].source().id;
    let input_id = state.input_id().expect("accepted input id").to_owned();
    let bound = recipe.bind(Some(scope.cache_key("chords-chat")));
    store.admit_binding(&claim_1, &bound).await.unwrap();

    let call_args = json!({
        "correction_id": "corr-oral-challenge",
        "recorded_on": "2026-10-04",
        "statement": "Oral amoxicillin challenge on 2026-10-04 showed no allergic reaction.",
        "source_message_id": null,
        "source_chat_id": null,
        "source_physician_id": null,
        "supersedes_correction_id": null,
        "upsert_topics": [
            { "topic": "allergies", "note": "Cleared of penicillin allergy by oral challenge on 2026-10-04." }
        ],
        "remove_topics": []
    });
    let call = engine::Invocation {
        id: "call-unacked-corr".into(),
        name: RECORD_CORRECTION_TOOL.into(),
        arguments: call_args.to_string(),
    };

    for (sequence, kind) in [
        (
            1,
            engine::EventKind::ProviderStarted {
                dispatched_at_unix_ms: 1_762_900_000_000,
                context: engine::DispatchContext {
                    carried_inputs: vec![input_id],
                    ..Default::default()
                },
                request: engine::SavedRequest {
                    added: Vec::new(),
                    items: vec![engine::RequestItem::Message { id: user_msg_id }],
                },
            },
        ),
        (
            2,
            engine::EventKind::ProviderReturned(call_turn(vec![call.clone()])),
        ),
        (3, engine::EventKind::ToolStarted(call.clone())),
    ] {
        store
            .persist(
                &claim_1,
                engine::Event {
                    attempt_id: claim_1.attempt.clone(),
                    sequence,
                    ordinal: 1,
                    kind,
                },
            )
            .await
            .unwrap();
    }

    // Before committing the mutation, `saved_chat_memory_outcome` must report `None`
    // without executing anything.
    assert!(
        store
            .saved_chat_memory_outcome(&claim_1, &call)
            .await
            .unwrap()
            .is_none()
    );

    // Commit the tool mutation under Attempt 1's active lease.
    let committed_out = store
        .execute_chat_memory_tool(&claim_1, &call, call_args)
        .await
        .unwrap();
    let committed_json: Value = serde_json::from_str(committed_out.text().unwrap()).unwrap();
    assert_eq!(committed_json["outcome"], "saved");
    assert_eq!(committed_json["receipt"]["version"], 1);

    // Simulate Attempt 1 crashing before `ToolReturned`: settle Attempt 1 as interrupted
    // and claim successor Attempt 2 for the same work.
    store
        .settle_chat_attempt(&claim_1, engine::AttemptFailure::Interrupted)
        .await
        .unwrap();
    let claim_2 = store.claim(&scope, 60).await.unwrap().unwrap();
    assert_ne!(claim_1.attempt, claim_2.attempt);

    // Successor Attempt 2 runs `PhysicianHost::run` end-to-end with ONLY the final
    // assistant turn in `successor_provider`. Harness `recover_tools` sees `ToolStarted`
    // without `ToolReturned`, invokes `PhysicianHost::saved_outcome`, recovers the exact
    // version 1 receipt without re-running the tool, and delivers it into the continuation
    // provider exchange.
    let mut host_2 = PhysicianHost {
        store: store.clone(),
        claim: claim_2,
        authority: AllowSession::single(&scope),
    };
    let mut successor_provider = ScriptedProvider::new(vec![assistant_turn(
        "Recorded the negative oral amoxicillin challenge from 2026-10-04.",
    )]);
    host_2
        .run(
            &mut successor_provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    assert_eq!(successor_provider.seen_requests.len(), 1);
    let delivered_tool_output = successor_provider.seen_requests[0]
        .items
        .iter()
        .find_map(|item| match item {
            engine::Item::MaterializedResult { call_id, output }
                if call_id == "call-unacked-corr" =>
            {
                Some(output.clone())
            }
            engine::Item::Result { call_id, output } if call_id == "call-unacked-corr" => {
                Some(serde_json::to_string(output).unwrap())
            }
            _ => None,
        })
        .expect("recovered continuation request must carry reconciled tool result");
    let delivered_json: Value = serde_json::from_str(&delivered_tool_output).unwrap();
    assert_eq!(delivered_json, committed_json);

    let conversation = store.conversation(&scope).await.unwrap();
    assert_eq!(conversation.responses().count(), 1);

    let patient = PatientKey::new(&scope.practice, &scope.workspace).unwrap();
    let PermittedVersionsRead::Permitted {
        current_version,
        versions,
    } = store
        .permitted_memory_versions(&patient, PatientAccess::ClinicalPanel)
        .await
        .unwrap()
    else {
        panic!("expected permitted versions");
    };
    assert_eq!(current_version, MemoryVersionNumber::new(1));
    assert_eq!(versions.len(), 1);
}

#[tokio::test]
async fn ac2_practice_patient_access_and_retraction_isolation_plus_search_metadata() {
    let Some(pg) = DisposablePostgres::acquire("patient-memory-ac2").await else {
        return;
    };
    let store = pg.store().await;

    let practice_a = format!("prac-a-{}", Uuid::new_v4().simple());
    let practice_b = format!("prac-b-{}", Uuid::new_v4().simple());

    let scope_a_p1 = Scope {
        practice: practice_a.clone(),
        physician: "dr-a".into(),
        workspace: "patient-shared-local-id".into(),
        chat: "chat-1".into(),
    };
    let scope_a_p2 = Scope {
        practice: practice_a.clone(),
        physician: "dr-a".into(),
        workspace: "patient-other".into(),
        chat: "chat-1".into(),
    };
    let scope_b_p1 = Scope {
        practice: practice_b.clone(),
        physician: "dr-b".into(),
        workspace: "patient-shared-local-id".into(),
        chat: "chat-1".into(),
    };
    let session = [7_u8; 32];

    let mut auth_a = AllowSession {
        expected_practice: practice_a.clone(),
        expected_physician: "dr-a".into(),
        allowed_workspaces: [
            "patient-shared-local-id".to_owned(),
            "patient-other".to_owned(),
            "patient-cleared-head".to_owned(),
        ]
        .into_iter()
        .collect(),
    };
    let mut auth_b = AllowSession::single(&scope_b_p1);

    // Populate (practice_a, patient-shared-local-id):
    // v1: initial understanding
    let mut v1_topics = BTreeMap::new();
    v1_topics.insert(
        topic("renal"),
        "CKD stage 3a, baseline Cr 1.4 mg/dL.".into(),
    );
    store
        .mutate_patient_memory_authorized(
            &scope_a_p1,
            "a-p1-v1",
            &session,
            MemoryMutation::UpdateUnderstanding(
                OperatingIndexPatch::new(v1_topics, BTreeSet::new()).unwrap(),
            ),
            &mut auth_a,
        )
        .await
        .unwrap();

    // v2: physician correction via practice job so we also verify multi-author search metadata
    let patient_a_p1 = PatientKey::new(&practice_a, "patient-shared-local-id").unwrap();
    let review_job = JobMemoryCaller::new("periodic_review", "review-run-9").unwrap();
    store
        .mutate_patient_memory_in_job(
            &patient_a_p1,
            &review_job,
            "a-p1-v2",
            MemoryMutation::RecordCorrection(RecordCorrection {
                correction_id: Some(CorrectionId::parse("corr-baseline-cr").unwrap()),
                recorded_on: date("2026-09-15"),
                source_message: SourceMessageRef::new("chat-1", "msg-cr-1", "dr-a").unwrap(),
                statement:
                    "Baseline creatinine is 1.2 mg/dL; 1.4 mg/dL was during acute dehydration."
                        .into(),
                supersedes: None,
                understanding_patch: None,
            }),
        )
        .await
        .unwrap();

    // v3: erroneous note accidentally written to this chart ("CONTAMINATED_WRONG_PATIENT_NOTE")
    let mut v3_wrong = BTreeMap::new();
    v3_wrong.insert(
        topic("erroneous_entry"),
        "CONTAMINATED_WRONG_PATIENT_NOTE: transplant rejection protocol".into(),
    );
    store
        .mutate_patient_memory_authorized(
            &scope_a_p1,
            "a-p1-v3-err",
            &session,
            MemoryMutation::UpdateUnderstanding(
                OperatingIndexPatch::new(v3_wrong, BTreeSet::new()).unwrap(),
            ),
            &mut auth_a,
        )
        .await
        .unwrap();

    // Asking to `KeepCurrent` when retracting the current version (v3) is structurally refused
    // and rolled back without consuming the receipt key or marking v3 retracted.
    assert!(matches!(
        store
            .mutate_patient_memory_authorized(
                &scope_a_p1,
                "a-p1-v4-retract",
                &session,
                MemoryMutation::RetractVersion {
                    target_version: MemoryVersionNumber::new(3).unwrap(),
                    retracted_on: date("2026-09-16"),
                    reason: "Entered on wrong patient workspace".into(),
                    disposition: RetractionDisposition::KeepCurrent,
                },
                &mut auth_a,
            )
            .await
            .unwrap(),
        MemoryMutationOutcome::InvalidTarget { .. }
    ));

    // v4: retract v3 with explicit `ReplaceCurrent` restoring only valid renal topic + correction!
    let mut clean_topics = BTreeMap::new();
    clean_topics.insert(
        topic("renal"),
        "CKD stage 3a, baseline Cr 1.2 mg/dL (1.4 mg/dL was during dehydration).".into(),
    );
    store
        .mutate_patient_memory_authorized(
            &scope_a_p1,
            "a-p1-v4-retract",
            &session,
            MemoryMutation::RetractVersion {
                target_version: MemoryVersionNumber::new(3).unwrap(),
                retracted_on: date("2026-09-16"),
                reason: "Entered on wrong patient workspace".into(),
                disposition: RetractionDisposition::ReplaceCurrent(
                    local_product::runtime::memory::ReplacementState {
                        operating_index:
                            local_product::runtime::memory::OperatingIndex::from_entries(
                                clean_topics,
                            ),
                        corrections: vec![local_product::runtime::memory::MemoryCorrection {
                            correction_id: CorrectionId::parse("corr-baseline-cr").unwrap(),
                            recorded_on: date("2026-09-15"),
                            source_message: SourceMessageRef::new("chat-1", "msg-cr-1", "dr-a")
                                .unwrap(),
                            statement:
                                "Baseline creatinine is 1.2 mg/dL; 1.4 mg/dL was during acute dehydration."
                                    .into(),
                            standing: CorrectionStanding::Active,
                        }],
                        summary: None,
                    },
                ),
            },
            &mut auth_a,
        )
        .await
        .unwrap();

    // Populate (practice_a, patient-other) and (practice_b, patient-shared-local-id)
    // to prove strict tenant and patient isolation.
    let mut other_patient_topics = BTreeMap::new();
    other_patient_topics.insert(
        topic("pulmonary"),
        "OTHER_PATIENT_SECRET_ASTHMA_NOTE".into(),
    );
    store
        .mutate_patient_memory_authorized(
            &scope_a_p2,
            "a-p2-v1",
            &session,
            MemoryMutation::UpdateUnderstanding(
                OperatingIndexPatch::new(other_patient_topics, BTreeSet::new()).unwrap(),
            ),
            &mut auth_a,
        )
        .await
        .unwrap();

    let mut other_practice_topics = BTreeMap::new();
    other_practice_topics.insert(
        topic("neurology"),
        "OTHER_PRACTICE_SECRET_MIGRAINE_NOTE".into(),
    );
    store
        .mutate_patient_memory_authorized(
            &scope_b_p1,
            "b-p1-v1",
            &session,
            MemoryMutation::UpdateUnderstanding(
                OperatingIndexPatch::new(other_practice_topics, BTreeSet::new()).unwrap(),
            ),
            &mut auth_b,
        )
        .await
        .unwrap();

    // 1. Right practice + right patient reads current clean v4 and permitted prior [v1, v2, v4];
    // retracted v3 is Withheld and absent from permitted versions!
    let MemoryRead::Available(curr) = store
        .patient_memory_for_access(
            &patient_a_p1,
            PatientAccess::ClinicalPanel,
            VersionSelection::Current,
        )
        .await
        .unwrap()
    else {
        panic!("expected current memory");
    };
    assert_eq!(curr.version.get(), 4);
    assert_eq!(curr.practice, practice_a);
    assert_eq!(curr.workspace, "patient-shared-local-id");
    assert!(
        curr.operating_index
            .get(&topic("erroneous_entry"))
            .is_none()
    );

    assert_eq!(
        store
            .patient_memory_for_access(
                &patient_a_p1,
                PatientAccess::ClinicalPanel,
                VersionSelection::Exact(MemoryVersionNumber::new(3).unwrap()),
            )
            .await
            .unwrap(),
        MemoryRead::Withheld {
            version: MemoryVersionNumber::new(3).unwrap()
        }
    );

    let PermittedVersionsRead::Permitted {
        current_version,
        versions,
    } = store
        .permitted_memory_versions(&patient_a_p1, PatientAccess::ClinicalPanel)
        .await
        .unwrap()
    else {
        panic!("expected permitted versions");
    };
    assert_eq!(current_version, MemoryVersionNumber::new(4));
    let version_numbers: Vec<u64> = versions.iter().map(|s| s.version.get()).collect();
    assert_eq!(version_numbers, vec![1, 2, 4]);
    let all_permitted_json = serde_json::to_string(&versions).unwrap();
    assert!(!all_permitted_json.contains("CONTAMINATED_WRONG_PATIENT_NOTE"));
    assert!(!all_permitted_json.contains("OTHER_PATIENT_SECRET_ASTHMA_NOTE"));
    assert!(!all_permitted_json.contains("OTHER_PRACTICE_SECRET_MIGRAINE_NOTE"));

    // 2. Clinic-demographic and denied callers, plus cross-practice/off-panel sessions,
    // are refused across every read surface.
    for non_panel in [PatientAccess::ClinicDemographic, PatientAccess::Denied] {
        assert!(matches!(
            store
                .patient_memory_for_access(&patient_a_p1, non_panel, VersionSelection::Current)
                .await
                .unwrap(),
            MemoryRead::Unauthorized { .. }
        ));
        assert!(matches!(
            store
                .permitted_memory_versions(&patient_a_p1, non_panel)
                .await
                .unwrap(),
            PermittedVersionsRead::Unauthorized { .. }
        ));
        assert!(matches!(
            store
                .patient_summary_for_access(&patient_a_p1, non_panel)
                .await
                .unwrap(),
            SummaryRead::Unauthorized { .. }
        ));
    }
    // Practice B's authority cannot read Practice A's scope even when workspace string matches.
    assert!(matches!(
        store
            .patient_memory_authorized(
                &scope_a_p1,
                &session,
                VersionSelection::Current,
                &mut auth_b,
            )
            .await
            .unwrap(),
        MemoryRead::Unauthorized { .. }
    ));
    assert!(matches!(
        store
            .patient_memory_search_records_authorized(
                &scope_a_p1,
                &session,
                SearchVersionScope::CurrentAndPermittedPrior,
                SearchAudienceScope::AgentJob,
                &mut auth_b,
            )
            .await
            .unwrap(),
        SearchRead::Unauthorized { .. }
    ));

    // 3. Search projection delivers exact identities, versions, standings, clinical dates,
    // authors, and access facts ONLY for the requested practice and allowed workspaces,
    // excluding retracted v3.
    let search_records = store
        .memory_search_records(&MemorySearchQuery {
            practice: practice_a.clone(),
            allowed_workspaces: ["patient-shared-local-id".to_owned()].into_iter().collect(),
            versions: SearchVersionScope::CurrentAndPermittedPrior,
            audience: SearchAudienceScope::AgentJob,
        })
        .await
        .unwrap();

    assert!(!search_records.is_empty());
    let search_json = serde_json::to_string(&search_records).unwrap();
    assert!(!search_json.contains("CONTAMINATED_WRONG_PATIENT_NOTE"));
    assert!(!search_json.contains("OTHER_PATIENT_SECRET_ASTHMA_NOTE"));
    assert!(!search_json.contains("OTHER_PRACTICE_SECRET_MIGRAINE_NOTE"));

    let expected_memory_id = local_product::runtime::memory::MemoryId::for_patient(&patient_a_p1);
    let v4_num = MemoryVersionNumber::new(4).unwrap();
    for record in &search_records {
        assert_eq!(record.memory_id, expected_memory_id);
        assert_ne!(
            record.version.get(),
            3,
            "retracted version 3 must never reach search"
        );
        assert_eq!(record.access.practice, practice_a);
        assert_eq!(record.access.workspace, "patient-shared-local-id");
        assert_eq!(record.access.required_access, RequiredAccess::ClinicalPanel);
        assert_eq!(record.access.visibility, PermittedVisibility::Permitted);
        if record.version == v4_num {
            assert_eq!(record.standing, VersionStanding::Current);
        } else {
            assert_eq!(
                record.standing,
                VersionStanding::Prior {
                    current_version: Some(v4_num)
                }
            );
        }
    }

    // Check v2's correction search record specifically for exact PracticeJob author + date + access facts.
    let v2_correction = search_records
        .iter()
        .find(|r| {
            r.version.get() == 2
                && matches!(
                    &r.content,
                    SearchableMemoryContent::Correction { correction_id, .. }
                        if correction_id.as_str() == "corr-baseline-cr"
                )
        })
        .expect("v2 correction search record");
    assert_eq!(
        v2_correction.author,
        MemoryAuthor::PracticeJob {
            job_kind: "periodic_review".into(),
            job_id: "review-run-9".into(),
        }
    );
    assert_eq!(
        v2_correction.clinical_dates,
        vec![MemorySearchDate::CorrectionRecordedOn(date("2026-09-15"))]
    );
    assert_eq!(
        v2_correction.access.audience,
        MemoryAudience::PhysicianVisible
    );

    // 4. Retracting a patient's current head version with `ClearCurrent` leaves `current_version`
    // as `None` while keeping permitted prior versions searchable under `CurrentAndPermittedPrior`.
    let scope_cleared = Scope {
        practice: practice_a.clone(),
        physician: "dr-a".into(),
        workspace: "patient-cleared-head".into(),
        chat: "chat-1".into(),
    };
    let patient_cleared = PatientKey::new(&practice_a, "patient-cleared-head").unwrap();
    let mut cleared_v1 = BTreeMap::new();
    cleared_v1.insert(
        topic("endocrine"),
        "Subclinical hypothyroidism, TSH 5.8.".into(),
    );
    store
        .mutate_patient_memory_authorized(
            &scope_cleared,
            "cleared-v1",
            &session,
            MemoryMutation::UpdateUnderstanding(
                OperatingIndexPatch::new(cleared_v1, BTreeSet::new()).unwrap(),
            ),
            &mut auth_a,
        )
        .await
        .unwrap();

    let mut cleared_v2 = BTreeMap::new();
    cleared_v2.insert(topic("erroneous"), "CLEARED_HEAD_CONTAMINATED_V2".into());
    store
        .mutate_patient_memory_authorized(
            &scope_cleared,
            "cleared-v2",
            &session,
            MemoryMutation::UpdateUnderstanding(
                OperatingIndexPatch::new(cleared_v2, BTreeSet::new()).unwrap(),
            ),
            &mut auth_a,
        )
        .await
        .unwrap();

    store
        .mutate_patient_memory_authorized(
            &scope_cleared,
            "cleared-v3-retract",
            &session,
            MemoryMutation::RetractVersion {
                target_version: MemoryVersionNumber::new(2).unwrap(),
                retracted_on: date("2026-09-17"),
                reason: "Contaminated head version cleared pending manual review".into(),
                disposition: RetractionDisposition::ClearCurrent,
            },
            &mut auth_a,
        )
        .await
        .unwrap();

    let PermittedVersionsRead::Permitted {
        current_version: cleared_current,
        versions: cleared_versions,
    } = store
        .permitted_memory_versions(&patient_cleared, PatientAccess::ClinicalPanel)
        .await
        .unwrap()
    else {
        panic!("expected permitted versions on cleared head");
    };
    assert_eq!(cleared_current, None);
    assert_eq!(cleared_versions.len(), 1);
    assert_eq!(cleared_versions[0].version.get(), 1);

    let cleared_current_only = store
        .memory_search_records(&MemorySearchQuery {
            practice: practice_a.clone(),
            allowed_workspaces: ["patient-cleared-head".to_owned()].into_iter().collect(),
            versions: SearchVersionScope::CurrentOnly,
            audience: SearchAudienceScope::AgentJob,
        })
        .await
        .unwrap();
    assert!(cleared_current_only.is_empty());

    let cleared_with_prior = store
        .memory_search_records(&MemorySearchQuery {
            practice: practice_a,
            allowed_workspaces: ["patient-cleared-head".to_owned()].into_iter().collect(),
            versions: SearchVersionScope::CurrentAndPermittedPrior,
            audience: SearchAudienceScope::AgentJob,
        })
        .await
        .unwrap();
    assert_eq!(cleared_with_prior.len(), 1);
    assert_eq!(cleared_with_prior[0].version.get(), 1);
    assert_eq!(
        cleared_with_prior[0].standing,
        VersionStanding::Prior {
            current_version: None
        }
    );
    assert!(
        !serde_json::to_string(&cleared_with_prior)
            .unwrap()
            .contains("CLEARED_HEAD_CONTAMINATED_V2")
    );
}

#[tokio::test]
async fn ac3_and_ac4_summary_survives_restart_and_private_operating_index_never_reaches_physician_responses()
 {
    let Some(mut pg) = DisposablePostgres::acquire_restartable("patient-memory-ac3-ac4").await
    else {
        return;
    };
    let store = pg.store().await;

    let practice = format!("prac-{}", Uuid::new_v4().simple());
    let workspace = "patient-marcus".to_owned();
    let scope = Scope {
        practice: practice.clone(),
        physician: "dr-okafor".into(),
        workspace: workspace.clone(),
        chat: "chat-summary-review".into(),
    };
    let session = [7_u8; 32];
    let recipe = chat_recipe();

    let private_sentinel_1 = "PRIVATE_OPERATING_INDEX_SENTINEL_7741_UNVERIFIED_RENAL_HYPOTHESIS";
    let private_sentinel_2 = "PRIVATE_OPERATING_INDEX_SENTINEL_8892_INTERNAL_FOLLOWUP_QUEUE";

    // Run a chat turn that writes private operating-index notes AND saves the physician-facing
    // patient summary, then answers the physician in clinical prose.
    store
        .accept_authorized(
            &scope,
            &Input {
                key: "msg-summary-req-1".into(),
                text: "Summarize Marcus's heart failure management and recent labs.".into(),
                timezone: "America/New_York".into(),
                scenario: None,
            },
            session.to_vec(),
            skills(),
            &recipe,
        )
        .await
        .unwrap();

    let expected_references = vec![
        SummaryReference::ChartRecord {
            source_id: "epic-fhir".into(),
            candidate_id: "obs-bnp-2026-11".into(),
            revision: "rev-3".into(),
            label: "NT-proBNP 410 pg/mL".into(),
            record_date: Some(date("2026-11-08")),
        },
        SummaryReference::ConversationMessage {
            chat_id: "chat-summary-review".into(),
            message_id: "msg-summary-req-1".into(),
            label: "Cardiology management review".into(),
            stated_on: Some(date("2026-11-12")),
        },
        SummaryReference::Literature {
            citation: "2022 AHA/ACC/HFSA Guideline for the Management of Heart Failure".into(),
            identifier: Some("PMID:35363499".into()),
        },
    ];
    let expected_dates = SummaryDates {
        written_at: date("2026-11-12"),
        reviewed_at: Some(date("2026-11-12")),
        pulled_at: Some(date("2026-11-08")),
    };
    let summary_prose = "Heart failure with reduced ejection fraction remains compensated on sacubitril-valsartan and carvedilol, with NT-proBNP 410 pg/mL on 2026-11-08.";

    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut provider = ScriptedProvider::new(vec![
        call_turn(vec![
            engine::Invocation {
                id: "call-private-index".into(),
                name: SAVE_MEMORY_TOOL.into(),
                arguments: json!({
                    "upsert": [
                        { "topic": "differential_scratch", "note": private_sentinel_1 }
                    ],
                    "remove": []
                })
                .to_string(),
            },
            engine::Invocation {
                id: "call-public-summary".into(),
                name: SAVE_SUMMARY_TOOL.into(),
                arguments: json!({
                    "content": summary_prose,
                    "references": expected_references,
                    "dates": expected_dates,
                })
                .to_string(),
            },
        ]),
        assistant_turn(
            "Marcus's HFrEF remains compensated on guideline-directed therapy; NT-proBNP on 2026-11-08 was 410 pg/mL.\n\nBasis: 2026-11-08 laboratory record and 2022 AHA/ACC/HFSA heart failure guideline.",
        ),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim,
        authority: AllowSession::single(&scope),
    }
    .run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Apply a subsequent background-job update to the private operating index (version 3)
    // to prove the saved summary (authored at version 2) survives later private index edits
    // AND that the new private note never leaks into summary or physician responses.
    let patient = PatientKey::new(&practice, &workspace).unwrap();
    let bg_job = JobMemoryCaller::new("chart_arrival", "arrival-19").unwrap();
    let mut bg_upsert = BTreeMap::new();
    bg_upsert.insert(topic("internal_queue"), private_sentinel_2.into());
    store
        .mutate_patient_memory_in_job(
            &patient,
            &bg_job,
            "bg-op-index-v3",
            MemoryMutation::UpdateUnderstanding(
                OperatingIndexPatch::new(bg_upsert, BTreeSet::new()).unwrap(),
            ),
        )
        .await
        .unwrap();

    // Crash-restart Postgres and reconnect a fresh Store.
    drop(store);
    pg.crash_restart();
    let restarted = pg.store().await;

    // AC3: Summary reads back after restart with exact content, references, and dates.
    let SummaryRead::Present(saved_summary) = restarted
        .patient_summary_authorized(&scope, &session, &mut AllowSession::single(&scope))
        .await
        .unwrap()
    else {
        panic!("expected saved patient summary after restart");
    };
    assert_eq!(saved_summary.practice, practice);
    assert_eq!(saved_summary.workspace, workspace);
    assert_eq!(saved_summary.summary_version.get(), 2);
    assert_eq!(saved_summary.memory_version.get(), 3);
    assert_eq!(saved_summary.summary.content, summary_prose);
    assert_eq!(saved_summary.summary.references, expected_references);
    assert_eq!(saved_summary.summary.dates, expected_dates);

    // AC4 (1): Structural schema & serialization check on physician-facing summary types.
    let summary_value = serde_json::to_value(&saved_summary).unwrap();
    let top_keys: BTreeSet<&str> = summary_value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        top_keys,
        [
            "memory_id",
            "memory_version",
            "practice",
            "saved_at",
            "summary",
            "summary_version",
            "workspace",
        ]
        .into_iter()
        .collect()
    );
    let inner_keys: BTreeSet<&str> = summary_value["summary"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        inner_keys,
        ["content", "dates", "references"].into_iter().collect()
    );

    let mut injected_inner = summary_value["summary"].clone();
    injected_inner["operating_index"] = json!({ "entries": { "leak": private_sentinel_1 } });
    assert!(
        serde_json::from_value::<PatientSummary>(injected_inner).is_err(),
        "PatientSummary must structurally reject any operating_index field"
    );

    let mut injected = summary_value.clone();
    injected["operating_index"] = json!({ "entries": { "leak": private_sentinel_1 } });
    assert!(
        serde_json::from_value::<SavedPatientSummary>(injected).is_err(),
        "SavedPatientSummary must structurally reject any operating_index field"
    );

    let summary_text = serde_json::to_string(&saved_summary).unwrap();
    assert!(!summary_text.contains("operating_index"));
    assert!(!summary_text.contains(private_sentinel_1));
    assert!(!summary_text.contains(private_sentinel_2));

    // AC4 (2): Physician-facing chat conversation & published responses inspection.
    let conversation = restarted.conversation(&scope).await.unwrap();
    assert_eq!(conversation.messages.len(), 2);
    assert!(matches!(
        conversation.messages[0].role,
        MessageRole::User { .. }
    ));
    assert!(matches!(
        conversation.messages[1].role,
        MessageRole::Assistant { partial: false, .. }
    ));
    for message in &conversation.messages {
        assert!(!message.text.contains("operating_index"));
        assert!(!message.text.contains(private_sentinel_1));
        assert!(!message.text.contains(private_sentinel_2));
    }
    let published_responses: Vec<_> = conversation.responses().collect();
    assert_eq!(published_responses.len(), 1);
    assert!(!published_responses[0].text.contains(private_sentinel_1));
    assert!(!published_responses[0].text.contains(private_sentinel_2));

    // AC4 (3): Physician-facing search audience structurally excludes OperatingTopic,
    // whereas internal AgentJob search still receives the private operating index.
    let physician_search = restarted
        .memory_search_records(&MemorySearchQuery {
            practice: practice.clone(),
            allowed_workspaces: [workspace.clone()].into_iter().collect(),
            versions: SearchVersionScope::CurrentAndPermittedPrior,
            audience: SearchAudienceScope::PhysicianFacing,
        })
        .await
        .unwrap();
    assert!(!physician_search.is_empty());
    for item in &physician_search {
        assert_eq!(item.access.audience, MemoryAudience::PhysicianVisible);
        assert!(
            !matches!(item.content, SearchableMemoryContent::OperatingTopic { .. }),
            "PhysicianFacing search must never emit OperatingTopic records"
        );
    }
    let physician_search_text = serde_json::to_string(&physician_search).unwrap();
    assert!(!physician_search_text.contains(private_sentinel_1));
    assert!(!physician_search_text.contains(private_sentinel_2));

    let agent_search = restarted
        .memory_search_records(&MemorySearchQuery {
            practice,
            allowed_workspaces: [workspace].into_iter().collect(),
            versions: SearchVersionScope::CurrentOnly,
            audience: SearchAudienceScope::AgentJob,
        })
        .await
        .unwrap();
    let agent_search_text = serde_json::to_string(&agent_search).unwrap();
    assert!(agent_search_text.contains(private_sentinel_1));
    assert!(agent_search_text.contains(private_sentinel_2));
}
