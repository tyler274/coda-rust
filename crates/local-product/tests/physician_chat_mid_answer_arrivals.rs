//! Real Postgres and harness proof that physician chat checks chart currency at
//! finish, refuses stale finishes with an arrival note in the same run, lets
//! the model judge added records while requiring re-reads of corrected or
//! withdrawn records it relied on, adds zero extra requests when nothing
//! changed, and starts nothing when records arrive while idle.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use local_product::{
    ModelBinding, ModelRoles,
    physician_chat::{ChatAssignment, PhysicianSkills},
    physician_sources::{
        ChatArrivalOutcome, ChatChartArrivalNote, MedicalRecordNumber, READ_TOOL, ReadablePart,
        RecordDate, ReliedRecordRequirement, ReliedRereadAction, SEARCH_TOOL, SourcePatient,
        SourceRevision, WithdrawnShape, workspace_id,
    },
    runtime::{
        Accepted, AttemptStatus, Authority, Input, PhysicianHost, PublicationTransaction, Scope,
        Store, WorkStatus, async_trait,
        chat_operations::RecordLimitKind,
        engine::{
            self, Cancellation, EventKind, Invocation, Item, RunLimits, Turn,
            recipe::{Observer, Provider, Request},
        },
        history::HistoryOutcome,
        sources::SourcePull,
    },
    sha256_hex,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use serde_json::{Value, json};

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

async fn setup_stores() -> (Store, PostgresAuthStore) {
    let url = test_database_url();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    (store, auth)
}

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice_id: &str,
    label: &str,
) -> (PhysicianIdentity, Vec<u8>) {
    let email = format!("{label}-{}@codahealth.test", uuid::Uuid::new_v4().simple());
    let now = unix_seconds();
    let invite = auth
        .invite(practice_id, &email, label, false, now)
        .await
        .unwrap();
    let (identity, token) = auth.redeem_invitation(&email, &invite, now).await.unwrap();
    let hash = PostgresAuthStore::session_reference(&token);
    (identity, hash)
}

fn chat_roles() -> ModelRoles {
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

fn chart_record(
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

fn latest_refusal_note(request: &Request) -> Option<ChatChartArrivalNote> {
    request.items.iter().rev().find_map(|item| match item {
        Item::Message { role, text } if role == "developer" => {
            let envelope: Value = serde_json::from_str(text).ok()?;
            if envelope.get("executed") != Some(&Value::Bool(false)) {
                return None;
            }
            let error_text = envelope.get("error")?.as_str()?;
            serde_json::from_str::<ChatChartArrivalNote>(error_text).ok()
        }
        _ => None,
    })
}

fn saved_refusal_notes(
    conversation: &local_product::runtime::Conversation,
) -> Vec<ChatChartArrivalNote> {
    let mut notes = Vec::new();
    for attempt in &conversation.attempts {
        for event in &attempt.events {
            if let EventKind::ProviderStarted { request, .. } = &event.kind {
                for item in &request.added {
                    if let Item::FinishRefused(refusal) = item
                        && let Ok(note) =
                            serde_json::from_str::<ChatChartArrivalNote>(refusal.reason())
                    {
                        notes.push(note);
                    }
                }
            }
        }
    }
    notes
}

struct WorkspaceFixture {
    scope: Scope,
    patient: SourcePatient,
    session_hash: Vec<u8>,
}

async fn setup_workspace(
    store: &Store,
    auth: &PostgresAuthStore,
    label: &str,
    chat_id: &str,
) -> WorkspaceFixture {
    let practice = format!("practice-{label}-{}", uuid::Uuid::new_v4().simple());
    let (physician, session_hash) = invite_and_sign_in(auth, &practice, "dr-rivera").await;
    let patient = synthetic_patient(label, "Elena Morales", "1961-04-19", "MRN-44101");
    let workspace = workspace_id("synthetic-clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();
    let scope = Scope {
        practice,
        physician: physician.physician_id,
        workspace,
        chat: chat_id.to_owned(),
    };
    let _ = store;
    WorkspaceFixture {
        scope,
        patient,
        session_hash,
    }
}

async fn start_patient_chat(store: &Store, fixture: &WorkspaceFixture, key: &str, question: &str) {
    let starting_context = store
        .patient_starting_context(&fixture.scope)
        .await
        .unwrap();
    let assignment = ChatAssignment::for_patient(fixture.scope.clone(), starting_context);
    let skills = load_skills();
    let recipe = assignment.recipe(&chat_roles(), &skills);
    let accepted = store
        .start_chat_authorized(
            &fixture.scope,
            &Input {
                key: key.to_owned(),
                text: question.to_owned(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            fixture.session_hash.clone(),
            &skills,
            &recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(accepted, Accepted::New);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn corrected_result_mid_answer_refuses_finish_rereads_and_unchanged_chat_uses_no_extra_request()
 {
    let (store, auth) = setup_stores().await;

    // --- Part A (Unchanged chart): answer arrives with zero extra model requests ---
    let unchanged_fx = setup_workspace(&store, &auth, "unchanged", "chat-unchanged").await;
    let unchanged_bmp = chart_record(
        &unchanged_fx.scope.workspace,
        "rev-1",
        "2026-11-10",
        "Basic Metabolic Panel",
        "Potassium 4.5 mmol/L, Creatinine 1.02 mg/dL.",
        json!({
            "resourceType": "Observation",
            "id": "lab-potassium-2026-11",
            "status": "final",
            "valueString": "Potassium 4.5 mmol/L"
        }),
    );
    store
        .record_chart(
            &unchanged_fx.scope.workspace,
            std::slice::from_ref(&unchanged_bmp),
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: unchanged_fx.patient.clone(),
                pulled_at: 1_762_900_000,
            },
        )
        .await
        .unwrap();

    start_patient_chat(
        &store,
        &unchanged_fx,
        "msg-unchanged-1",
        "What is Elena's latest potassium?",
    )
    .await;

    struct UnchangedChatProvider {
        requests: Vec<Request>,
    }

    impl Provider for UnchangedChatProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.requests.push(request.clone());
            match self.requests.len() {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "search-1".into(),
                        name: SEARCH_TOOL.into(),
                        arguments: json!({
                            "query": "potassium",
                            "record_type": null,
                            "after": null
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "read-1".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "lab-potassium-2026-11",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 2}),
                    usage: Default::default(),
                }),
                3 => Ok(Turn {
                    items: vec![Item::assistant(
                        "Elena's potassium on November 10, 2026 is 4.5 mmol/L [1].\n\nBasis and limitations: Synthetic Clinic EHR Basic Metabolic Panel (2026-11-10).",
                    )],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected extra model request #{other} when nothing changed"),
            }
        }
    }

    let unchanged_claim = store.claim(&unchanged_fx.scope, 60).await.unwrap().unwrap();
    let mut unchanged_host = PhysicianHost {
        store: store.clone(),
        claim: unchanged_claim,
        authority: PostgresWorkspaceAuthority,
    };
    let mut unchanged_provider = UnchangedChatProvider {
        requests: Vec::new(),
    };
    unchanged_host
        .run(
            &mut unchanged_provider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        unchanged_provider.requests.len(),
        3,
        "unchanged chart must finish in exactly 3 requests (search -> read -> answer) with no extra model request"
    );
    let unchanged_saved = store.conversation(&unchanged_fx.scope).await.unwrap();
    assert_eq!(unchanged_saved.status, WorkStatus::Completed);
    assert!(
        saved_refusal_notes(&unchanged_saved).is_empty(),
        "unchanged chart must record zero finish refusals"
    );
    println!(
        "[unchanged-chat] provider_requests={} finish_refusals={} status={:?}",
        unchanged_provider.requests.len(),
        saved_refusal_notes(&unchanged_saved).len(),
        unchanged_saved.status
    );

    // --- Part B (Corrected mid-answer): finish is refused with note, model continues
    // in the same run, re-reads the corrected revision, and answers with the correction ---
    let corrected_fx = setup_workspace(&store, &auth, "corrected", "chat-corrected").await;
    let initial_hemolyzed_k = chart_record(
        &corrected_fx.scope.workspace,
        "rev-1",
        "2026-11-10",
        "Basic Metabolic Panel",
        "Potassium 6.1 mmol/L (specimen hemolyzed; repeat pending).",
        json!({
            "resourceType": "Observation",
            "id": "lab-potassium-2026-11",
            "status": "preliminary",
            "valueString": "Potassium 6.1 mmol/L (specimen hemolyzed)"
        }),
    );
    let corrected_repeat_k = chart_record(
        &corrected_fx.scope.workspace,
        "rev-2",
        "2026-11-10",
        "Basic Metabolic Panel (Corrected)",
        "Potassium 4.2 mmol/L (corrected non-hemolyzed repeat; initial 6.1 mmol/L was hemolysis artifact).",
        json!({
            "resourceType": "Observation",
            "id": "lab-potassium-2026-11",
            "status": "corrected",
            "valueString": "Potassium 4.2 mmol/L (corrected non-hemolyzed repeat)"
        }),
    );

    store
        .record_chart(
            &corrected_fx.scope.workspace,
            std::slice::from_ref(&initial_hemolyzed_k),
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: corrected_fx.patient.clone(),
                pulled_at: 1_762_900_100,
            },
        )
        .await
        .unwrap();

    start_patient_chat(
        &store,
        &corrected_fx,
        "msg-corrected-1",
        "What is Elena's latest potassium, and does she have hyperkalemia?",
    )
    .await;

    struct CorrectedMidAnswerProvider {
        store: Store,
        workspace: String,
        patient: SourcePatient,
        corrected_record: SourceRevision,
        requests: Vec<Request>,
    }

    impl Provider for CorrectedMidAnswerProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.requests.push(request.clone());
            match self.requests.len() {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "search-1".into(),
                        name: SEARCH_TOOL.into(),
                        arguments: json!({
                            "query": "potassium",
                            "record_type": null,
                            "after": null
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let outputs = materialized_outputs(request);
                    let search_json: Value =
                        serde_json::from_str(outputs.get("search-1").unwrap()).unwrap();
                    assert_eq!(search_json["workspace_version"], 1);
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "read-rev-1".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "lab-potassium-2026-11",
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
                    let outputs = materialized_outputs(request);
                    let read_r1 = outputs.get("read-rev-1").unwrap();
                    assert!(read_r1.contains("6.1 mmol/L"));

                    // While Coda is formulating its answer from `rev-1`, the lab
                    // delivers the corrected `rev-2` result (`4.2 mmol/L`).
                    self.store
                        .record_chart(
                            &self.workspace,
                            std::slice::from_ref(&self.corrected_record),
                            &SourcePull {
                                source_id: "synthetic-clinic-ehr".into(),
                                source_label: "Synthetic Clinic EHR".into(),
                                patient: self.patient.clone(),
                                pulled_at: 1_762_900_200,
                            },
                        )
                        .await
                        .unwrap();

                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's potassium on November 10, 2026 is elevated at 6.1 mmol/L [1], though the specimen was hemolyzed.\n\nBasis and limitations: Synthetic Clinic EHR preliminary Basic Metabolic Panel (2026-11-10).",
                        )],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => {
                    // Finish #1 was refused! Verify the developer refusal note names the correction
                    // and requires re-reading `rev-2`. Also verify that attempting to finish
                    // WITHOUT calling `read_patient_sources` on `rev-2` is refused too.
                    let note = latest_refusal_note(request)
                        .expect("refused finish must deliver ChatChartArrivalNote");
                    assert_eq!(note.outcome, ChatArrivalOutcome::SourcesChanged);
                    assert_eq!(note.workspace_version, 2);
                    assert!(note.added.is_empty());
                    assert!(note.withdrawn.is_empty());
                    assert_eq!(note.corrected.len(), 1);
                    assert_eq!(note.corrected[0].candidate_id, "lab-potassium-2026-11");
                    assert_eq!(note.corrected[0].previous_revision, "rev-1");
                    assert_eq!(note.corrected[0].current_revision, "rev-2");
                    assert_eq!(
                        note.must_read_again,
                        vec![ReliedRecordRequirement {
                            candidate_id: "lab-potassium-2026-11".into(),
                            action: ReliedRereadAction::ReadCurrentRevision {
                                revision: "rev-2".into(),
                            },
                        }]
                    );

                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Guessing without reading rev-2 is not allowed.",
                        )],
                        receipt: json!({"turn": 4}),
                        usage: Default::default(),
                    })
                }
                5 => {
                    let unread_note = latest_refusal_note(request)
                        .expect("skipping re-read on relied-on correction must refuse again");
                    assert_eq!(unread_note.outcome, ChatArrivalOutcome::SourcesUnread);
                    assert_eq!(
                        unread_note.must_read_again,
                        vec![ReliedRecordRequirement {
                            candidate_id: "lab-potassium-2026-11".into(),
                            action: ReliedRereadAction::ReadCurrentRevision {
                                revision: "rev-2".into(),
                            },
                        }]
                    );

                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "read-rev-2".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "lab-potassium-2026-11",
                                    "revision": "rev-2",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 5}),
                        usage: Default::default(),
                    })
                }
                6 => {
                    let outputs = materialized_outputs(request);
                    let read_r2 = outputs.get("read-rev-2").unwrap();
                    assert!(read_r2.contains("4.2 mmol/L"));
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's corrected potassium on November 10, 2026 is normal at 4.2 mmol/L on non-hemolyzed repeat testing (the initial 6.1 mmol/L value was a hemolysis artifact), so she does not have hyperkalemia [1].\n\nBasis and limitations: Synthetic Clinic EHR corrected Basic Metabolic Panel (2026-11-10, rev-2).",
                        )],
                        receipt: json!({"turn": 6}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn #{other}"),
            }
        }
    }

    let corrected_claim = store.claim(&corrected_fx.scope, 60).await.unwrap().unwrap();
    let mut corrected_host = PhysicianHost {
        store: store.clone(),
        claim: corrected_claim,
        authority: PostgresWorkspaceAuthority,
    };
    let mut corrected_provider = CorrectedMidAnswerProvider {
        store: store.clone(),
        workspace: corrected_fx.scope.workspace.clone(),
        patient: corrected_fx.patient.clone(),
        corrected_record: corrected_repeat_k,
        requests: Vec::new(),
    };
    corrected_host
        .run(
            &mut corrected_provider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();

    assert_eq!(corrected_provider.requests.len(), 6);

    let saved = store.conversation(&corrected_fx.scope).await.unwrap();
    assert_eq!(saved.status, WorkStatus::Completed);
    assert_eq!(
        saved.runs.len(),
        1,
        "mid-answer arrival continues in the exact same conversation work"
    );
    assert_eq!(
        saved.attempts.len(),
        1,
        "mid-answer arrival continues in the exact same attempt without starting another run"
    );
    assert_eq!(
        saved.messages.len(),
        2,
        "only the final current answer is published"
    );
    assert!(saved.messages[1].text.contains("4.2 mmol/L"));
    assert!(
        saved.messages[1]
            .text
            .contains("does not have hyperkalemia")
    );

    // Inspect the saved run events directly for the refused candidate and refusal notes.
    let candidates: Vec<&str> = saved.attempts[0]
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::Candidate { text } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(
        candidates.len(),
        3,
        "saved run must show the two refused finish candidates followed by the committed candidate"
    );
    assert!(candidates[0].contains("elevated at 6.1 mmol/L"));

    let notes = saved_refusal_notes(&saved);
    assert_eq!(
        notes.len(),
        2,
        "saved run must persist Item::FinishRefused notes"
    );
    assert_eq!(notes[0].outcome, ChatArrivalOutcome::SourcesChanged);
    assert_eq!(notes[0].corrected[0].candidate_id, "lab-potassium-2026-11");
    assert_eq!(notes[0].corrected[0].previous_revision, "rev-1");
    assert_eq!(notes[0].corrected[0].current_revision, "rev-2");
    assert_eq!(notes[1].outcome, ChatArrivalOutcome::SourcesUnread);

    let reopened = store
        .reopen_conversation(&corrected_fx.scope)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(reopened.outcome, HistoryOutcome::Answered);
    assert!(reopened.turns[1].raw_text.contains("4.2 mmol/L"));
    assert!(
        reopened
            .sources
            .values()
            .any(|src| src.candidate_id == "lab-potassium-2026-11" && src.revision == "rev-2"),
        "reopened citations must link to the corrected rev-2 record: {:?}",
        reopened.sources
    );
    println!(
        "[corrected-mid-answer] provider_requests={} runs={} attempts={} candidates={} refusal_note_1={} refusal_note_2={} final_answer={:?}",
        corrected_provider.requests.len(),
        saved.runs.len(),
        saved.attempts.len(),
        candidates.len(),
        notes[0].to_json_string(),
        notes[1].to_json_string(),
        saved.messages[1].text
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn added_records_mid_answer_notify_model_and_let_model_judge_whether_to_open_them() {
    let (store, auth) = setup_stores().await;

    // --- Subcase 1: Unrelated record arrives during tool execution / before finish.
    // Tool call is never interrupted; finish is refused once with `added`; model
    // judges the unrelated record itself and finishes without opening it. ---
    let unrelated_fx = setup_workspace(&store, &auth, "added-unrelated", "chat-added-1").await;
    let bmp = chart_record(
        &unrelated_fx.scope.workspace,
        "rev-1",
        "2026-11-10",
        "Basic Metabolic Panel",
        "Potassium 4.4 mmol/L.",
        json!({
            "resourceType": "Observation",
            "id": "lab-bmp-1",
            "status": "final",
            "valueString": "Potassium 4.4 mmol/L"
        }),
    );
    let unrelated_derm = chart_record(
        &unrelated_fx.scope.workspace,
        "rev-1",
        "2026-11-11",
        "Dermatology Clinic Note",
        "Benign seborrheic keratosis on left forearm; no intervention.",
        json!({
            "resourceType": "DocumentReference",
            "id": "note-derm-2026-11",
            "status": "current",
            "description": "Dermatology clinic note"
        }),
    );

    store
        .record_chart(
            &unrelated_fx.scope.workspace,
            std::slice::from_ref(&bmp),
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: unrelated_fx.patient.clone(),
                pulled_at: 1_762_900_000,
            },
        )
        .await
        .unwrap();

    start_patient_chat(
        &store,
        &unrelated_fx,
        "msg-added-unrelated",
        "What is Elena's potassium?",
    )
    .await;

    struct UnrelatedAdditionProvider {
        store: Store,
        added_record: SourceRevision,
        turn: usize,
    }

    impl Provider for UnrelatedAdditionProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => {
                    // Record arrives while Coda issues its read tool call; the tool
                    // call must NOT be interrupted or refused.
                    self.store.record_source(&self.added_record).await.unwrap();
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "read-bmp".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "lab-bmp-1",
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
                    let outputs = materialized_outputs(request);
                    assert!(
                        outputs.get("read-bmp").unwrap().contains("4.4 mmol/L"),
                        "chart arrival must never interrupt an in-flight tool call"
                    );
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's potassium on November 10, 2026 is 4.4 mmol/L [1].\n\nBasis and limitations: Synthetic Clinic EHR Basic Metabolic Panel (2026-11-10).",
                        )],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let note = latest_refusal_note(request)
                        .expect("added record must produce arrival note at finish");
                    assert_eq!(note.outcome, ChatArrivalOutcome::SourcesChanged);
                    assert_eq!(note.added.len(), 1);
                    assert_eq!(note.added[0].candidate_id, "note-derm-2026-11");
                    assert_eq!(note.added[0].revision, "rev-1");
                    assert!(note.corrected.is_empty());
                    assert!(note.withdrawn.is_empty());
                    assert!(
                        note.must_read_again.is_empty(),
                        "added records must not force read_patient_sources when unrelated"
                    );

                    // Model judges the dermatology note unrelated and answers directly
                    // without calling `read_patient_sources` on `note-derm-2026-11`.
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's potassium on November 10, 2026 is 4.4 mmol/L [1].\n\nBasis and limitations: Synthetic Clinic EHR Basic Metabolic Panel (2026-11-10).",
                        )],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn #{other}"),
            }
        }
    }

    let claim_1 = store.claim(&unrelated_fx.scope, 60).await.unwrap().unwrap();
    let mut host_1 = PhysicianHost {
        store: store.clone(),
        claim: claim_1,
        authority: PostgresWorkspaceAuthority,
    };
    let mut provider_1 = UnrelatedAdditionProvider {
        store: store.clone(),
        added_record: unrelated_derm,
        turn: 0,
    };
    host_1
        .run(
            &mut provider_1,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();

    assert_eq!(provider_1.turn, 3);
    let saved_1 = store.conversation(&unrelated_fx.scope).await.unwrap();
    assert_eq!(saved_1.status, WorkStatus::Completed);
    assert_eq!(saved_refusal_notes(&saved_1).len(), 1);

    // --- Subcase 2: Clinically relevant record arrives mid-answer; model opens it
    // via `read_patient_sources` and incorporates it into the published answer. ---
    let relevant_fx = setup_workspace(&store, &auth, "added-relevant", "chat-added-2").await;
    let earlier_bmp = chart_record(
        &relevant_fx.scope.workspace,
        "rev-1",
        "2026-11-01",
        "Basic Metabolic Panel",
        "Potassium 4.3 mmol/L on November 1, 2026.",
        json!({
            "resourceType": "Observation",
            "id": "lab-bmp-nov01",
            "status": "final",
            "valueString": "Potassium 4.3 mmol/L"
        }),
    );
    let stat_k_arrival = chart_record(
        &relevant_fx.scope.workspace,
        "rev-1",
        "2026-11-12",
        "STAT Serum Potassium",
        "Potassium 5.8 mmol/L (critical high, non-hemolyzed).",
        json!({
            "resourceType": "Observation",
            "id": "lab-stat-k-nov12",
            "status": "final",
            "valueString": "Potassium 5.8 mmol/L"
        }),
    );

    store
        .record_chart(
            &relevant_fx.scope.workspace,
            std::slice::from_ref(&earlier_bmp),
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: relevant_fx.patient.clone(),
                pulled_at: 1_762_900_000,
            },
        )
        .await
        .unwrap();

    start_patient_chat(
        &store,
        &relevant_fx,
        "msg-added-relevant",
        "What is Elena's most recent potassium?",
    )
    .await;

    struct RelevantAdditionProvider {
        store: Store,
        added_record: SourceRevision,
        turn: usize,
    }

    impl Provider for RelevantAdditionProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "read-nov01".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "lab-bmp-nov01",
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
                    self.store.record_source(&self.added_record).await.unwrap();
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's most recent potassium is 4.3 mmol/L on November 1, 2026 [1].\n\nBasis and limitations: Synthetic Clinic EHR Basic Metabolic Panel (2026-11-01).",
                        )],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let note = latest_refusal_note(request).expect("arrival note");
                    assert_eq!(note.added.len(), 1);
                    assert_eq!(note.added[0].candidate_id, "lab-stat-k-nov12");
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "read-nov12".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "lab-stat-k-nov12",
                                    "revision": "rev-1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => {
                    let outputs = materialized_outputs(request);
                    assert!(outputs.get("read-nov12").unwrap().contains("5.8 mmol/L"));
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's most recent potassium is elevated at 5.8 mmol/L on November 12, 2026 [2], up from 4.3 mmol/L on November 1, 2026 [1].\n\nBasis and limitations: Synthetic Clinic EHR STAT Serum Potassium (2026-11-12) and Basic Metabolic Panel (2026-11-01).",
                        )],
                        receipt: json!({"turn": 4}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn #{other}"),
            }
        }
    }

    let claim_2 = store.claim(&relevant_fx.scope, 60).await.unwrap().unwrap();
    let mut host_2 = PhysicianHost {
        store: store.clone(),
        claim: claim_2,
        authority: PostgresWorkspaceAuthority,
    };
    let mut provider_2 = RelevantAdditionProvider {
        store: store.clone(),
        added_record: stat_k_arrival,
        turn: 0,
    };
    host_2
        .run(
            &mut provider_2,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();

    let saved_2 = store.conversation(&relevant_fx.scope).await.unwrap();
    assert_eq!(saved_2.status, WorkStatus::Completed);
    assert!(saved_2.messages[1].text.contains("5.8 mmol/L"));
    println!(
        "[added-mid-answer] unrelated_note={} relevant_note={} relevant_answer={:?}",
        saved_refusal_notes(&saved_1)[0].to_json_string(),
        saved_refusal_notes(&saved_2)[0].to_json_string(),
        saved_2.messages[1].text
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn withdrawn_records_mid_answer_require_rereading_relied_record_for_both_withdrawal_shapes() {
    let (store, auth) = setup_stores().await;

    // --- Subcase 1: Whole-source withdrawal (`RemovedFromObservation`) ---
    let removed_fx = setup_workspace(&store, &auth, "withdrawn-removed", "chat-withdrawn-1").await;
    let baseline_med = chart_record(
        &removed_fx.scope.workspace,
        "rev-1",
        "2026-10-15",
        "Active Medication",
        "Lisinopril 20 mg oral daily.",
        json!({
            "resourceType": "MedicationRequest",
            "id": "med-lisinopril",
            "status": "active"
        }),
    );
    let misassigned_k = chart_record(
        &removed_fx.scope.workspace,
        "rev-1",
        "2026-11-12",
        "Serum Potassium",
        "Potassium 6.4 mmol/L.",
        json!({
            "resourceType": "Observation",
            "id": "lab-misassigned-k",
            "status": "final",
            "valueString": "Potassium 6.4 mmol/L"
        }),
    );

    store
        .record_chart(
            &removed_fx.scope.workspace,
            &[baseline_med.clone(), misassigned_k],
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: removed_fx.patient.clone(),
                pulled_at: 1_762_900_000,
            },
        )
        .await
        .unwrap();

    start_patient_chat(
        &store,
        &removed_fx,
        "msg-withdrawn-1",
        "Does Elena have a potassium lab from today?",
    )
    .await;

    struct RemovedWithdrawalProvider {
        store: Store,
        workspace: String,
        patient: SourcePatient,
        remaining_med: SourceRevision,
        turn: usize,
    }

    impl Provider for RemovedWithdrawalProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "read-k-1".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "lab-misassigned-k",
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
                    // Whole-source delivery omits `lab-misassigned-k`, withdrawing it.
                    self.store
                        .record_chart(
                            &self.workspace,
                            std::slice::from_ref(&self.remaining_med),
                            &SourcePull {
                                source_id: "synthetic-clinic-ehr".into(),
                                source_label: "Synthetic Clinic EHR".into(),
                                patient: self.patient.clone(),
                                pulled_at: 1_762_900_300,
                            },
                        )
                        .await
                        .unwrap();
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's potassium today is 6.4 mmol/L [1].\n\nBasis and limitations: Synthetic Clinic EHR Serum Potassium (2026-11-12).",
                        )],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let note = latest_refusal_note(request).expect("withdrawal note");
                    assert_eq!(note.outcome, ChatArrivalOutcome::SourcesChanged);
                    assert_eq!(note.withdrawn.len(), 1);
                    assert_eq!(note.withdrawn[0].candidate_id, "lab-misassigned-k");
                    assert_eq!(
                        note.withdrawn[0].shape,
                        WithdrawnShape::RemovedFromObservation {
                            withdrawn_revision: "rev-1".into(),
                        }
                    );
                    assert_eq!(
                        note.must_read_again,
                        vec![ReliedRecordRequirement {
                            candidate_id: "lab-misassigned-k".into(),
                            action: ReliedRereadAction::ConfirmWithdrawnFromChart {
                                withdrawn_revision: "rev-1".into(),
                            },
                        }]
                    );

                    // First attempt to finish without re-reading the withdrawn record must be refused.
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Skipping read_patient_sources after withdrawal.",
                        )],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => {
                    let unread = latest_refusal_note(request).expect("unread withdrawal note");
                    assert_eq!(unread.outcome, ChatArrivalOutcome::SourcesUnread);
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "confirm-withdrawn".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "lab-misassigned-k",
                                    "revision": "rev-1",
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
                    let outputs = materialized_outputs(request);
                    let receipt: Value =
                        serde_json::from_str(outputs.get("confirm-withdrawn").unwrap()).unwrap();
                    assert_eq!(receipt["results"][0]["outcome"], "unavailable");
                    assert_eq!(
                        receipt["results"][0]["reason"],
                        "not_in_recorded_observation"
                    );
                    assert_eq!(
                        receipt["limits"][0]["kind"],
                        serde_json::to_value(RecordLimitKind::Missing).unwrap()
                    );

                    Ok(Turn {
                        items: vec![Item::assistant(
                            "No valid potassium result from November 12, 2026 remains in Elena's chart; the earlier result was withdrawn from the chart observation.\n\nBasis and limitations: Synthetic Clinic EHR chart observation checked 2026-11-12. Missing: current serum potassium.",
                        )],
                        receipt: json!({"turn": 5}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn #{other}"),
            }
        }
    }

    let claim_1 = store.claim(&removed_fx.scope, 60).await.unwrap().unwrap();
    let mut host_1 = PhysicianHost {
        store: store.clone(),
        claim: claim_1,
        authority: PostgresWorkspaceAuthority,
    };
    let mut provider_1 = RemovedWithdrawalProvider {
        store: store.clone(),
        workspace: removed_fx.scope.workspace.clone(),
        patient: removed_fx.patient.clone(),
        remaining_med: baseline_med,
        turn: 0,
    };
    host_1
        .run(
            &mut provider_1,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();

    let saved_1 = store.conversation(&removed_fx.scope).await.unwrap();
    assert_eq!(saved_1.status, WorkStatus::Completed);
    assert!(saved_1.messages[1].text.contains("withdrawn"));
    assert!(!saved_1.messages[1].text.contains("6.4 mmol/L"));

    // --- Subcase 2: Retracted FHIR status (`"status": "entered-in-error"`) ---
    let retracted_fx =
        setup_workspace(&store, &auth, "withdrawn-retracted", "chat-withdrawn-2").await;
    let troponin_r1 = chart_record(
        &retracted_fx.scope.workspace,
        "rev-1",
        "2026-11-12",
        "Troponin I",
        "Troponin I 0.42 ng/mL (elevated).",
        json!({
            "resourceType": "Observation",
            "id": "lab-troponin-2026-11",
            "status": "final",
            "valueString": "0.42 ng/mL"
        }),
    );
    let troponin_retracted_r2 = chart_record(
        &retracted_fx.scope.workspace,
        "rev-2",
        "2026-11-12",
        "Troponin I (Entered in Error)",
        "Entered in error: wrong patient tube label; retracted by laboratory.",
        json!({
            "resourceType": "Observation",
            "id": "lab-troponin-2026-11",
            "status": "entered-in-error",
            "note": "Wrong patient tube label; retracted"
        }),
    );

    store
        .record_chart(
            &retracted_fx.scope.workspace,
            std::slice::from_ref(&troponin_r1),
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: retracted_fx.patient.clone(),
                pulled_at: 1_762_900_000,
            },
        )
        .await
        .unwrap();

    start_patient_chat(
        &store,
        &retracted_fx,
        "msg-withdrawn-2",
        "Is Elena's troponin elevated?",
    )
    .await;

    struct RetractedWithdrawalProvider {
        store: Store,
        retracted: SourceRevision,
        turn: usize,
    }

    impl Provider for RetractedWithdrawalProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "read-trop-r1".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "lab-troponin-2026-11",
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
                    self.store.record_source(&self.retracted).await.unwrap();
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's troponin I is elevated at 0.42 ng/mL [1].\n\nBasis and limitations: Synthetic Clinic EHR Troponin I (2026-11-12).",
                        )],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let note = latest_refusal_note(request).expect("retracted note");
                    assert_eq!(note.outcome, ChatArrivalOutcome::SourcesChanged);
                    assert_eq!(note.withdrawn.len(), 1);
                    assert_eq!(
                        note.withdrawn[0].shape,
                        WithdrawnShape::RetractedRevision {
                            previous_revision: "rev-1".into(),
                            current_revision: "rev-2".into(),
                        }
                    );
                    assert_eq!(
                        note.must_read_again,
                        vec![ReliedRecordRequirement {
                            candidate_id: "lab-troponin-2026-11".into(),
                            action: ReliedRereadAction::ReadCurrentRevision {
                                revision: "rev-2".into(),
                            },
                        }]
                    );

                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "read-trop-r2".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "lab-troponin-2026-11",
                                    "revision": "rev-2",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => {
                    let outputs = materialized_outputs(request);
                    assert!(
                        outputs
                            .get("read-trop-r2")
                            .unwrap()
                            .contains("entered-in-error")
                    );
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "The November 12, 2026 troponin I result was retracted by the laboratory as entered in error due to a specimen labeling mistake [1], so there is no valid elevated troponin result.\n\nBasis and limitations: Synthetic Clinic EHR retracted Troponin I record (2026-11-12, rev-2).",
                        )],
                        receipt: json!({"turn": 4}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn #{other}"),
            }
        }
    }

    let claim_2 = store.claim(&retracted_fx.scope, 60).await.unwrap().unwrap();
    let mut host_2 = PhysicianHost {
        store: store.clone(),
        claim: claim_2,
        authority: PostgresWorkspaceAuthority,
    };
    let mut provider_2 = RetractedWithdrawalProvider {
        store: store.clone(),
        retracted: troponin_retracted_r2,
        turn: 0,
    };
    host_2
        .run(
            &mut provider_2,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();

    let saved_2 = store.conversation(&retracted_fx.scope).await.unwrap();
    assert_eq!(saved_2.status, WorkStatus::Completed);
    assert!(saved_2.messages[1].text.contains("retracted"));
    println!(
        "[withdrawn-mid-answer] removed_note={} retracted_note={}",
        saved_refusal_notes(&saved_1)[0].to_json_string(),
        saved_refusal_notes(&saved_2)[0].to_json_string()
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn mid_answer_correction_across_crash_recovery_refuses_replayed_candidate_and_rereads() {
    let url = test_database_url();
    let (store, auth) = setup_stores().await;
    let fx = setup_workspace(&store, &auth, "crash-recovery", "chat-recovery").await;

    let lab_r1 = chart_record(
        &fx.scope.workspace,
        "rev-1",
        "2026-11-12",
        "Serum Potassium",
        "Potassium 5.9 mmol/L (preliminary).",
        json!({
            "resourceType": "Observation",
            "id": "lab-k-rec",
            "status": "preliminary",
            "valueString": "5.9 mmol/L"
        }),
    );
    let lab_r2 = chart_record(
        &fx.scope.workspace,
        "rev-2",
        "2026-11-12",
        "Serum Potassium (Corrected)",
        "Potassium 4.1 mmol/L (corrected repeat).",
        json!({
            "resourceType": "Observation",
            "id": "lab-k-rec",
            "status": "corrected",
            "valueString": "4.1 mmol/L"
        }),
    );

    store
        .record_chart(
            &fx.scope.workspace,
            std::slice::from_ref(&lab_r1),
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: fx.patient.clone(),
                pulled_at: 1_762_900_000,
            },
        )
        .await
        .unwrap();

    start_patient_chat(&store, &fx, "msg-recovery-1", "What is Elena's potassium?").await;

    let cancel_attempt_1 = Cancellation::default();
    let cancel_on_answer = cancel_attempt_1.clone();
    let phase = Arc::new(Mutex::new(0_usize));

    struct RecoveryProvider {
        phase: Arc<Mutex<usize>>,
        cancel_after_turn_2_return: Option<Cancellation>,
    }

    impl Provider for RecoveryProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            let current_phase = {
                let mut guard = self.phase.lock().unwrap();
                *guard += 1;
                *guard
            };
            match current_phase {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "read-r1".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "lab-k-rec",
                                "revision": "rev-1",
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"phase": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    // Return the candidate turn so `ProviderReturned` is persisted, then
                    // cancel before `finish` commits to simulate worker interruption.
                    if let Some(cancel) = self.cancel_after_turn_2_return.take() {
                        cancel.cancel();
                    }
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's potassium is 5.9 mmol/L [1].\n\nBasis and limitations: Synthetic Clinic EHR preliminary Serum Potassium (2026-11-12).",
                        )],
                        receipt: json!({"phase": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let note = latest_refusal_note(request)
                        .expect("recovered candidate must be refused with arrival note");
                    assert_eq!(note.outcome, ChatArrivalOutcome::SourcesChanged);
                    assert_eq!(note.corrected[0].current_revision, "rev-2");
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "read-r2".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "lab-k-rec",
                                    "revision": "rev-2",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"phase": 3}),
                        usage: Default::default(),
                    })
                }
                4 => {
                    let outputs = materialized_outputs(request);
                    assert!(outputs.get("read-r2").unwrap().contains("4.1 mmol/L"));
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Elena's corrected potassium is 4.1 mmol/L [1].\n\nBasis and limitations: Synthetic Clinic EHR corrected Serum Potassium (2026-11-12, rev-2).",
                        )],
                        receipt: json!({"phase": 4}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected recovery phase #{other}"),
            }
        }
    }

    let claim_1 = store.claim(&fx.scope, 60).await.unwrap().unwrap();
    let mut host_1 = PhysicianHost {
        store: store.clone(),
        claim: claim_1.clone(),
        authority: PostgresWorkspaceAuthority,
    };
    let mut provider_1 = RecoveryProvider {
        phase: Arc::clone(&phase),
        cancel_after_turn_2_return: Some(cancel_on_answer),
    };
    let err = host_1
        .run(&mut provider_1, RunLimits::default(), &cancel_attempt_1)
        .await
        .expect_err("cancelled before finish commits");
    assert!(matches!(err, engine::Failure::Cancelled));

    // Expire Attempt 1's lease (simulating lost worker) and deliver corrected `rev-2`
    // while no worker holds the lease.
    let raw_db = sea_orm::Database::connect(&url).await.unwrap();
    {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_chat_attempts")]
        struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            practice: String,
            #[sea_orm(primary_key, auto_increment = false)]
            physician: String,
            #[sea_orm(primary_key, auto_increment = false)]
            workspace: String,
            #[sea_orm(primary_key, auto_increment = false)]
            chat: String,
            #[sea_orm(primary_key, auto_increment = false)]
            id: String,
            expires: i64,
        }
        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}

        Entity::update_many()
            .col_expr(Column::Expires, Expr::value(0_i64))
            .filter(Column::Id.eq(claim_1.attempt.as_str()))
            .exec(&raw_db)
            .await
            .unwrap();
    }

    store.record_source(&lab_r2).await.unwrap();

    // Successor attempt claims the work, replays the uncommitted candidate turn
    // through `PhysicianHost::finish`, gets the arrival refusal note, re-reads `rev-2`,
    // and publishes the corrected answer!
    let claim_2 = store.claim(&fx.scope, 60).await.unwrap().unwrap();
    let mut host_2 = PhysicianHost {
        store: store.clone(),
        claim: claim_2,
        authority: PostgresWorkspaceAuthority,
    };
    let mut provider_2 = RecoveryProvider {
        phase: Arc::clone(&phase),
        cancel_after_turn_2_return: None,
    };
    host_2
        .run(
            &mut provider_2,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();

    let completed = store.conversation(&fx.scope).await.unwrap();
    assert_eq!(completed.status, WorkStatus::Completed);
    assert_eq!(completed.runs.len(), 1);
    assert_eq!(completed.attempts.len(), 2);
    assert_eq!(completed.attempts[0].status, AttemptStatus::Interrupted);
    assert_eq!(completed.attempts[1].status, AttemptStatus::Completed);
    assert_eq!(completed.messages.len(), 2);
    assert!(completed.messages[1].text.contains("4.1 mmol/L"));
    println!(
        "[crash-recovery-mid-answer] runs={} attempts={} attempt_0={:?} attempt_1={:?} refusal_note={} final_answer={:?}",
        completed.runs.len(),
        completed.attempts.len(),
        completed.attempts[0].status,
        completed.attempts[1].status,
        saved_refusal_notes(&completed)[0].to_json_string(),
        completed.messages[1].text
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn record_arriving_while_no_answer_runs_starts_nothing() {
    let (store, auth) = setup_stores().await;

    // 1. Workspace with zero conversations: arrivals add, correct, and withdraw
    // records without starting any chat conversation, delivery, or practice job.
    let untouched_fx = setup_workspace(&store, &auth, "idle-empty", "chat-never-started").await;
    let lab_v1 = chart_record(
        &untouched_fx.scope.workspace,
        "rev-1",
        "2026-11-12",
        "Basic Metabolic Panel",
        "Potassium 4.3 mmol/L.",
        json!({
            "resourceType": "Observation",
            "id": "lab-idle-1",
            "status": "final",
            "valueString": "4.3 mmol/L"
        }),
    );
    let lab_v2 = chart_record(
        &untouched_fx.scope.workspace,
        "rev-2",
        "2026-11-12",
        "Basic Metabolic Panel (Amended)",
        "Potassium 4.4 mmol/L.",
        json!({
            "resourceType": "Observation",
            "id": "lab-idle-1",
            "status": "amended",
            "valueString": "4.4 mmol/L"
        }),
    );

    store.record_source(&lab_v1).await.unwrap();
    store
        .import_sources(std::slice::from_ref(&lab_v2))
        .await
        .unwrap();
    store
        .record_chart(
            &untouched_fx.scope.workspace,
            &[],
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: untouched_fx.patient.clone(),
                pulled_at: 1_762_900_500,
            },
        )
        .await
        .unwrap();

    assert!(
        store.conversation(&untouched_fx.scope).await.is_err(),
        "record arrivals on an idle workspace must not create a conversation"
    );
    assert!(
        store.claim(&untouched_fx.scope, 60).await.is_err(),
        "no claimable chat work may exist when no conversation was started"
    );
    let untouched_past = store
        .workspace_past_work(
            &untouched_fx.scope.practice,
            &untouched_fx.scope.physician,
            &untouched_fx.scope.workspace,
        )
        .await
        .unwrap();
    assert!(
        untouched_past.is_empty(),
        "idle arrivals must not create past-work entries: {untouched_past:?}"
    );
    let due_for_untouched = store.dispatch_due(100, None).await.unwrap();
    assert!(
        !due_for_untouched
            .deliveries
            .iter()
            .any(|delivery| delivery.scope.workspace == untouched_fx.scope.workspace),
        "idle arrivals must not enqueue chat deliveries"
    );

    // 2. Workspace with an already-completed conversation: arrivals after the
    // answer finished start nothing (`News after the answer`).
    let completed_fx = setup_workspace(&store, &auth, "idle-after-answer", "chat-done").await;
    let initial_lab = chart_record(
        &completed_fx.scope.workspace,
        "rev-1",
        "2026-11-10",
        "Serum Potassium",
        "Potassium 4.5 mmol/L.",
        json!({
            "resourceType": "Observation",
            "id": "lab-done-1",
            "status": "final",
            "valueString": "4.5 mmol/L"
        }),
    );
    store
        .record_chart(
            &completed_fx.scope.workspace,
            std::slice::from_ref(&initial_lab),
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: completed_fx.patient.clone(),
                pulled_at: 1_762_900_000,
            },
        )
        .await
        .unwrap();

    start_patient_chat(
        &store,
        &completed_fx,
        "msg-done-1",
        "Summarize Elena's potassium.",
    )
    .await;

    struct OneTurnProvider;
    impl Provider for OneTurnProvider {
        async fn exchange(
            &mut self,
            _request: &Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            Ok(Turn {
                items: vec![Item::assistant(
                    "Elena's chart contains one serum potassium record from November 10, 2026.\n\nBasis and limitations: Starting chart orientation only.",
                )],
                receipt: json!({"turn": 1}),
                usage: Default::default(),
            })
        }
    }

    let done_claim = store.claim(&completed_fx.scope, 60).await.unwrap().unwrap();
    let mut done_host = PhysicianHost {
        store: store.clone(),
        claim: done_claim,
        authority: PostgresWorkspaceAuthority,
    };
    done_host
        .run(
            &mut OneTurnProvider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();

    let before_arrival = store.conversation(&completed_fx.scope).await.unwrap();
    assert_eq!(before_arrival.status, WorkStatus::Completed);
    assert_eq!(before_arrival.runs.len(), 1);
    assert_eq!(before_arrival.attempts.len(), 1);
    assert_eq!(before_arrival.messages.len(), 2);

    // Now deliver a new lab, a correction, and a withdrawal while NO answer runs.
    let post_answer_new = chart_record(
        &completed_fx.scope.workspace,
        "rev-1",
        "2026-11-12",
        "Repeat Potassium",
        "Potassium 5.6 mmol/L.",
        json!({
            "resourceType": "Observation",
            "id": "lab-after-answer",
            "status": "final",
            "valueString": "5.6 mmol/L"
        }),
    );
    let post_answer_corrected = chart_record(
        &completed_fx.scope.workspace,
        "rev-2",
        "2026-11-10",
        "Serum Potassium (Corrected)",
        "Potassium 4.2 mmol/L.",
        json!({
            "resourceType": "Observation",
            "id": "lab-done-1",
            "status": "corrected",
            "valueString": "4.2 mmol/L"
        }),
    );
    store.record_source(&post_answer_new).await.unwrap();
    store
        .record_chart(
            &completed_fx.scope.workspace,
            std::slice::from_ref(&post_answer_corrected),
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: completed_fx.patient.clone(),
                pulled_at: 1_762_900_900,
            },
        )
        .await
        .unwrap();

    let after_arrival = store.conversation(&completed_fx.scope).await.unwrap();
    assert_eq!(after_arrival.status, WorkStatus::Completed);
    assert_eq!(after_arrival.work, before_arrival.work);
    assert_eq!(after_arrival.revision, before_arrival.revision);
    assert_eq!(after_arrival.runs.len(), 1);
    assert_eq!(after_arrival.attempts.len(), 1);
    assert_eq!(after_arrival.messages.len(), 2);
    assert!(
        store
            .claim(&completed_fx.scope, 60)
            .await
            .unwrap()
            .is_none(),
        "arrivals while no answer runs must never make a completed conversation claimable"
    );
    let due_after = store.dispatch_due(100, None).await.unwrap();
    assert!(
        !due_after
            .deliveries
            .iter()
            .any(|delivery| delivery.scope.workspace == completed_fx.scope.workspace),
        "arrivals after completion must not enqueue any chat wakeup"
    );
    println!(
        "[idle-arrivals] untouched_conversations=0 post_answer_status={:?} post_answer_runs={} post_answer_attempts={} claimable=false due_deliveries=0",
        after_arrival.status,
        after_arrival.runs.len(),
        after_arrival.attempts.len()
    );
}
