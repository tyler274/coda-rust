//! Integration and unit tests for Spruce channel reading, conversation-to-patient
//! linking, classification storage, confirmation, wrong-patient moves, authorized
//! projections, and the background unmatched message matching recipe.
//!
//! Runs PostgreSQL tests automatically whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL`
//! or `PHYSICIAN_DATABASE_URL` is set; panics if neither is set when
//! `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use harness::provider::ResponsesProvider;
use local_product::physician_chat::{PhysicianSkills, SKILL_VIEW};
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
    SourceRevision,
};
use local_product::runtime::concerns::CONCERN_TOOL;
use local_product::runtime::dispatch::ReviewReceiver;
use local_product::runtime::jobs::{PracticeAccessError, PracticeJobError};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::spruce_matching::{
    BacklogStanding, ChannelHealth, ChannelStanding, ChannelStopCause, ChannelTieConflict,
    ChannelTieEvaluation, ConversationLinkState, FIFTEEN_MINUTES_SECONDS, IncomingSpruceItem,
    LinkCommandAcceptance, LinkOrigin, MatchAssessment, MatchingDecision, MessageRoutingOutcome,
    MessageStanding, MessageUrgency, MessageUrgencyChoice, MoveDestination,
    MoveSpruceConversationCommand, NonPatientCategory, OutsideParticipant, ReadableSpruceItem,
    RoutingOrigin, SenderAddress, SenderConfirmationPolicy, SpruceAttachmentSummary,
    SpruceContactPatient, SpruceContactRecord, SpruceEhrLink, SpruceIngestOutcome,
    SpruceMatchingError, TodayChannelBanner, UnreadableSpruceItem, evaluate_automatic_link,
    evaluate_channel_tie,
};
use local_product::runtime::{Store, async_trait, engine};
use local_product::spruce_matching_recipe::{
    GUIDANCE_VERSION, INSTRUCTIONS, MatchingInitialLoad, MatchingRunStatus,
    MatchingStartingContext, PATIENT_LOOKUP_TOOL, RECIPE_VERSION, SUBMIT_CLASSIFICATION_TOOL,
    ToolDiscoveryDisposition, instructions_sha256, patient_lookup_tool, recipe as matching_recipe,
    run_spruce_message_matching, submit_classification_tool, tools as matching_tools,
};
use local_product::{ModelBinding, ModelRoles, open_ai_api_key, sha256_hex};
use patient_work::{ReviewReason, ReviewRequest};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::{Value, json};
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../../verification/model_text.rs"]
mod model_text;

const SECRET_CHART_SENTINEL: &str = "CONFIDENTIAL_BIOPSY_SENTINEL_NEVER_IN_MATCHING_9941";

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
            model: "background-spruce-matching-lane".into(),
            effort: "medium".into(),
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

async fn invite_physician(
    auth: &PostgresAuthStore,
    practice: &str,
    label: &str,
    now_secs: u64,
) -> PhysicianIdentity {
    let email = format!("{label}-{}@codahealth.test", Uuid::new_v4().simple());
    let invite = auth
        .invite(practice, &email, label, false, now_secs)
        .await
        .expect("create physician invitation");
    let (identity, _) = auth
        .redeem_invitation(&email, &invite, now_secs)
        .await
        .expect("redeem physician invitation");
    identity
}

fn make_patient_demographics(id: &str, name: &str, dob: &str) -> SourcePatient {
    SourcePatient {
        source_patient_id: id.to_owned(),
        display_name: name.to_owned(),
        birth_date: Some(dob.to_owned()),
        sex_or_gender: Some("female".to_owned()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-mrn".to_owned(),
            value: format!("MRN-{id}"),
        }],
    }
}

fn make_chart_record(
    workspace: &str,
    candidate_id: &str,
    revision: &str,
    text: &str,
) -> SourceRevision {
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "clinic-ehr".to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: "DiagnosticReport".to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".to_owned(),
            value: "2026-11-01".to_owned(),
        }],
        content: text.to_owned(),
        sha256: sha256_hex(text.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "Report".to_owned(),
            text: text.to_owned(),
        }],
        provenance: json!({
            "system": "clinic-ehr",
            "native_id": candidate_id,
        }),
        envelope: None,
    }
}

#[allow(clippy::too_many_arguments)]
async fn seed_patient_with_chart(
    store: &Store,
    auth: &PostgresAuthStore,
    practice: &str,
    workspace: &str,
    ehr_patient_id: &str,
    display_name: &str,
    dob: &str,
    chart_text: &str,
    now_ts: i64,
) {
    auth.register_patient(practice, workspace)
        .await
        .expect("register patient workspace");
    let record = make_chart_record(workspace, "DiagnosticReport/lab-1", "rev-1", chart_text);
    let pull = SourcePull {
        source_id: "clinic-ehr".to_owned(),
        source_label: "Clinic EHR".to_owned(),
        patient: make_patient_demographics(ehr_patient_id, display_name, dob),
        pulled_at: Some(now_ts),
    };
    store
        .record_chart(workspace, &[record], &pull)
        .await
        .expect("record patient chart");
}

fn readable_item(
    pos: u64,
    message_id: &str,
    conversation_id: &str,
    sender: Option<&str>,
    author: Option<&str>,
    body: &str,
    participants: Vec<OutsideParticipant>,
) -> IncomingSpruceItem {
    IncomingSpruceItem::Readable(ReadableSpruceItem {
        message_id: message_id.to_owned(),
        conversation_id: conversation_id.to_owned(),
        channel_position: pos,
        content_version: format!("ver-{message_id}-{pos}"),
        sender_address: sender.map(|s| SenderAddress::parse(s).unwrap()),
        author_name: author.map(str::to_owned),
        body_text: body.to_owned(),
        attachments: Vec::new(),
        app_url: Some(format!(
            "https://app.sprucehealth.com/conversations/{conversation_id}"
        )),
        created_at_spruce: Some("2026-11-12T15:00:00Z".to_owned()),
        outside_participants: participants,
    })
}

/// Captures `ReviewRequest` submissions deduplicated by `(practice_id, command_key)`
/// just like the production practice review receiver, with optional per-workspace
/// fault injection to prove per-patient failure isolation.
#[derive(Default)]
struct RecordingReviewReceiver {
    accepted_by_key: Mutex<BTreeMap<(String, String), ReviewRequest>>,
    all_calls: Mutex<Vec<ReviewRequest>>,
    failing_workspaces: Mutex<BTreeSet<String>>,
}

impl RecordingReviewReceiver {
    fn fail_workspace(&self, workspace: &str) {
        self.failing_workspaces
            .lock()
            .unwrap()
            .insert(workspace.to_owned());
    }

    fn clear_failures(&self) {
        self.failing_workspaces.lock().unwrap().clear();
    }

    fn accepted_requests(&self) -> Vec<ReviewRequest> {
        self.accepted_by_key
            .lock()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }

    fn accepted_for_workspace(&self, workspace: &str) -> Vec<ReviewRequest> {
        self.accepted_by_key
            .lock()
            .unwrap()
            .values()
            .filter(|r| r.patient_workspace_id == workspace)
            .cloned()
            .collect()
    }
}

#[async_trait]
impl ReviewReceiver for RecordingReviewReceiver {
    async fn submit_review(&self, request: &ReviewRequest) -> Result<(), PracticeJobError> {
        self.all_calls.lock().unwrap().push(request.clone());
        if self
            .failing_workspaces
            .lock()
            .unwrap()
            .contains(&request.patient_workspace_id)
        {
            return Err(PracticeJobError::Access(
                PracticeAccessError::UnregisteredPatient {
                    practice: request.practice_id.clone(),
                    workspace: request.patient_workspace_id.clone(),
                },
            ));
        }
        self.accepted_by_key
            .lock()
            .unwrap()
            .entry((request.practice_id.clone(), request.command_key.clone()))
            .or_insert_with(|| request.clone());
        Ok(())
    }
}

struct ScriptedProvider {
    turns: VecDeque<Result<engine::Turn, String>>,
    requests: Vec<engine::recipe::Request>,
}

impl ScriptedProvider {
    fn new(turns: Vec<Result<engine::Turn, String>>) -> Self {
        Self {
            turns: VecDeque::from(turns),
            requests: Vec::new(),
        }
    }
}

impl engine::recipe::Provider for ScriptedProvider {
    async fn exchange(
        &mut self,
        request: &engine::recipe::Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn engine::recipe::Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        self.requests.push(request.clone());
        match self
            .turns
            .pop_front()
            .expect("scripted provider ran out of turns")
        {
            Ok(turn) => Ok(turn),
            Err(err) => Err(err.into()),
        }
    }
}

fn call_turn(id: &str, name: &str, args: Value) -> Result<engine::Turn, String> {
    Ok(engine::Turn {
        items: vec![engine::Item::Call(engine::Invocation {
            id: id.to_owned(),
            name: name.to_owned(),
            arguments: serde_json::to_string(&args).unwrap(),
        })],
        receipt: json!({ "call_id": id }),
        usage: Default::default(),
    })
}

fn done_turn(text: &str) -> Result<engine::Turn, String> {
    Ok(engine::Turn {
        items: vec![engine::Item::assistant(text)],
        receipt: json!({ "done": true }),
        usage: Default::default(),
    })
}

#[test]
fn matching_recipe_model_text_is_hygienic_and_excludes_all_chart_and_physician_tools() {
    model_text::assert_clean_model_text(INSTRUCTIONS);

    let lookup = patient_lookup_tool();
    let submit = submit_classification_tool();
    model_text::assert_clean_model_text(&serde_json::to_string(&lookup.function()).unwrap());
    model_text::assert_clean_model_text(&serde_json::to_string(&submit.function()).unwrap());

    let sample_ctx = MatchingStartingContext {
        practice: "practice-demo".into(),
        message_id: "msg-demo-1".into(),
        conversation_id: "conv-demo-1".into(),
        content_version: "ver-demo-1".into(),
        sender_address: Some(SenderAddress::parse("+14155550101").unwrap()),
        author_name: Some("Clara Oswald".into()),
        body_text: "Could I refill my blood pressure medication before Friday?".into(),
        attachments: vec![SpruceAttachmentSummary {
            kind: "image".into(),
            title: Some("Bottle label".into()),
            name: Some("bottle.jpg".into()),
            mimetype: Some("image/jpeg".into()),
        }],
        created_at_spruce: Some("2026-11-12T15:00:00Z".into()),
        outside_participants: vec![OutsideParticipant {
            display_name: Some("Clara Oswald".into()),
            endpoint_channel: Some("sms".into()),
            endpoint_address: Some(SenderAddress::parse("+14155550101").unwrap()),
            saved_contact_id: Some("spruce-contact-1".into()),
        }],
        confirmed_sender_workspaces: Vec::new(),
        channel_tie: ChannelTieEvaluation::Untied,
    };
    model_text::assert_clean_model_text(&sample_ctx.prompt_text());

    let skills = load_skills();
    let roles = distinct_roles();
    let built = matching_recipe(&roles, &skills);
    assert_eq!(built.model, roles.background.model);
    assert_eq!(
        built.context_window_tokens,
        roles.background.context_window_tokens
    );
    assert_eq!(built.max_input_tokens, roles.background.max_input_tokens);
    assert_eq!(built.options, json!({ "effort": roles.background.effort }));

    let tool_names: BTreeSet<String> = matching_tools().into_iter().map(|t| t.name).collect();
    assert!(tool_names.contains(PATIENT_LOOKUP_TOOL));
    assert!(tool_names.contains(SUBMIT_CLASSIFICATION_TOOL));
    assert!(tool_names.contains(SKILL_VIEW));

    for forbidden in [
        READ_TOOL,
        SEARCH_TOOL,
        CONCERN_TOOL,
        "list_patient_memory",
        "read_patient_memory",
        "search_patient_memory",
        "add_patient_memory_note",
        "edit_patient_memory",
        "read_patient_understanding",
        "save_patient_understanding",
        "read_patient_reviews",
        "complete_patient_review",
        "refresh_patient_sources",
        "retrieve_patient_work",
        "search_clinical_library",
        "read_clinical_library",
        "save_visit_briefing",
        "select_visits_for_preparation",
        "propose_patient_reply",
    ] {
        assert!(
            !tool_names.contains(forbidden),
            "matching recipe tools must never include clinical chart or physician tool `{forbidden}`"
        );
        let serialized_tools = serde_json::to_string(&built.tools).unwrap();
        assert!(
            !serialized_tools.contains(&format!("\"{forbidden}\"")),
            "built matching RequestRecipe must never advertise `{forbidden}`"
        );
    }
}

#[tokio::test]
async fn rule_4_auto_links_only_on_uncontradicted_unique_identity_and_enforces_access_without_scores()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts: i64 = 1_762_960_000;
    let practice = format!("practice-r4-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();

    let dr_alpha = invite_physician(&auth, &practice, "DrAlpha", now_ts as u64).await;
    let dr_beta = invite_physician(&auth, &practice, "DrBeta", now_ts as u64).await;

    let pw_clara = format!("pw-clara-{}", Uuid::new_v4().simple());
    let pw_danny = format!("pw-danny-{}", Uuid::new_v4().simple());
    let pw_no_duet = format!("pw-noduet-{}", Uuid::new_v4().simple());
    let pw_unassigned = format!("pw-unassigned-{}", Uuid::new_v4().simple());

    seed_patient_with_chart(
        &store,
        &auth,
        &practice,
        &pw_clara,
        "ehr-clara",
        "Clara Oswald",
        "1986-11-23",
        SECRET_CHART_SENTINEL,
        now_ts,
    )
    .await;
    seed_patient_with_chart(
        &store,
        &auth,
        &practice,
        &pw_danny,
        "ehr-danny",
        "Danny Pink",
        "1985-05-14",
        "Routine annual exam note.",
        now_ts,
    )
    .await;
    // Patient without Duet (registered in physician practice only, zero Duet connection):
    seed_patient_with_chart(
        &store,
        &auth,
        &practice,
        &pw_no_duet,
        "ehr-noduet",
        "Martha Jones",
        "1984-03-09",
        "Outpatient clinic chart without Duet.",
        now_ts,
    )
    .await;
    seed_patient_with_chart(
        &store,
        &auth,
        &practice,
        &pw_unassigned,
        "ehr-unassigned",
        "Rory Williams",
        "1983-08-02",
        "No physician assigned yet.",
        now_ts,
    )
    .await;

    // Dr. Alpha has access to Clara, Danny, and Martha (no-Duet); Dr. Beta only has access to Danny.
    for ws in [&pw_clara, &pw_danny, &pw_no_duet] {
        auth.set_membership(&dr_alpha, ws, Membership::Allowed)
            .await
            .unwrap();
    }
    auth.set_membership(&dr_beta, &pw_danny, Membership::Allowed)
        .await
        .unwrap();

    // Connect practice Spruce channel and acquire single reader lease.
    store
        .connect_spruce_channel(auth.as_ref(), &dr_alpha, 0, None, now_ts)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-a", 120, now_ts)
        .await
        .unwrap();

    let receiver = RecordingReviewReceiver::default();

    // --- 1a. Positive Auto-Link via Confirmed Sender -> 1 Patient ---
    let clara_phone = SenderAddress::parse("(415) 555-0111").unwrap();
    assert_eq!(clara_phone.as_str(), "+14155550111");
    store
        .record_spruce_confirmed_sender(
            &practice,
            &clara_phone,
            &pw_clara,
            &dr_alpha.physician_id,
            now_ts,
        )
        .await
        .unwrap();

    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                1,
                "msg-auto-sender",
                "conv-auto-sender",
                Some("415-555-0111"),
                Some("Clara Oswald"),
                "Checking on my blood pressure log.",
                vec![OutsideParticipant {
                    display_name: Some("Clara Oswald".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(clara_phone.clone()),
                    saved_contact_id: None,
                }],
            ),
            now_ts,
        )
        .await
        .unwrap();

    let outcome_sender = store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-auto-sender",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: pw_clara.clone(),
                    evidence_summary: "Confirmed sender +14155550111 matches Clara Oswald.".into(),
                },
                rationale: "Routine patient check-in from confirmed phone.".into(),
            },
            Some(&receiver),
            now_ts + 1,
        )
        .await
        .unwrap();
    assert!(matches!(
        outcome_sender,
        MessageRoutingOutcome::Linked {
            ref workspace,
            origin: LinkOrigin::AutomaticRule4,
            ..
        } if workspace == &pw_clara
    ));
    assert_eq!(receiver.accepted_for_workspace(&pw_clara).len(), 1);
    assert_eq!(
        receiver.accepted_for_workspace(&pw_clara)[0].reason,
        ReviewReason::NewRecords
    );

    // --- 1b. Positive Auto-Link via 1 -> 1 -> 1 -> 1 Channel Tie on Patient WITHOUT Duet ---
    store
        .register_spruce_ehr_patient_link(
            &practice,
            "clinic-ehr",
            "ehr-noduet",
            &pw_no_duet,
            now_ts,
        )
        .await
        .unwrap();
    store
        .save_spruce_contact_record(
            &SpruceContactRecord {
                practice: practice.clone(),
                contact_id: "contact-martha".into(),
                spruce_patients: vec![SpruceContactPatient {
                    spruce_patient_id: "spruce-pt-martha".into(),
                    ehr_links: vec![SpruceEhrLink {
                        source_id: "clinic-ehr".into(),
                        source_patient_id: "ehr-noduet".into(),
                    }],
                }],
            },
            now_ts,
        )
        .await
        .unwrap();

    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                2,
                "msg-auto-noduet",
                "conv-auto-noduet",
                Some("+14155550122"),
                Some("Martha Jones"),
                "Following up after yesterday's clinic visit.",
                vec![OutsideParticipant {
                    display_name: Some("Martha Jones".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550122").unwrap()),
                    saved_contact_id: Some("contact-martha".into()),
                }],
            ),
            now_ts + 2,
        )
        .await
        .unwrap();

    let outcome_noduet = store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-auto-noduet",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: pw_no_duet.clone(),
                    evidence_summary: "Channel tie uniquely links Martha Jones.".into(),
                },
                rationale: "Patient follow-up message.".into(),
            },
            Some(&receiver),
            now_ts + 3,
        )
        .await
        .unwrap();
    assert!(matches!(
        outcome_noduet,
        MessageRoutingOutcome::Linked {
            ref workspace,
            origin: LinkOrigin::AutomaticRule4,
            ..
        } if workspace == &pw_no_duet
    ));
    assert_eq!(receiver.accepted_for_workspace(&pw_no_duet).len(), 1);

    // --- 1c. Every Rule 4 Contradiction & Ambiguity MUST Wait for Confirmation ---
    // Direct unit checks of the single rule owners (`evaluate_channel_tie` & `evaluate_automatic_link`):
    let mut contacts_map = BTreeMap::new();
    contacts_map.insert(
        "shared-contact".to_owned(),
        vec![
            SpruceContactPatient {
                spruce_patient_id: "sp-1".into(),
                ehr_links: vec![SpruceEhrLink {
                    source_id: "clinic-ehr".into(),
                    source_patient_id: "ehr-clara".into(),
                }],
            },
            SpruceContactPatient {
                spruce_patient_id: "sp-2".into(),
                ehr_links: vec![SpruceEhrLink {
                    source_id: "clinic-ehr".into(),
                    source_patient_id: "ehr-danny".into(),
                }],
            },
        ],
    );
    contacts_map.insert(
        "multi-ehr-contact".to_owned(),
        vec![SpruceContactPatient {
            spruce_patient_id: "sp-multi".into(),
            ehr_links: vec![
                SpruceEhrLink {
                    source_id: "clinic-ehr".into(),
                    source_patient_id: "ehr-clara".into(),
                },
                SpruceEhrLink {
                    source_id: "clinic-ehr".into(),
                    source_patient_id: "ehr-danny".into(),
                },
            ],
        }],
    );
    contacts_map.insert(
        "dup-coda-contact".to_owned(),
        vec![SpruceContactPatient {
            spruce_patient_id: "sp-dup".into(),
            ehr_links: vec![SpruceEhrLink {
                source_id: "clinic-ehr".into(),
                source_patient_id: "ehr-dup".into(),
            }],
        }],
    );
    let mut ehr_map: BTreeMap<SpruceEhrLink, BTreeSet<String>> = BTreeMap::new();
    ehr_map.insert(
        SpruceEhrLink {
            source_id: "clinic-ehr".into(),
            source_patient_id: "ehr-clara".into(),
        },
        BTreeSet::from([pw_clara.clone()]),
    );
    ehr_map.insert(
        SpruceEhrLink {
            source_id: "clinic-ehr".into(),
            source_patient_id: "ehr-danny".into(),
        },
        BTreeSet::from([pw_danny.clone()]),
    );
    ehr_map.insert(
        SpruceEhrLink {
            source_id: "clinic-ehr".into(),
            source_patient_id: "ehr-dup".into(),
        },
        BTreeSet::from([pw_clara.clone(), pw_danny.clone()]),
    );

    assert!(matches!(
        evaluate_channel_tie(
            &[
                OutsideParticipant {
                    display_name: None,
                    endpoint_channel: None,
                    endpoint_address: None,
                    saved_contact_id: Some("shared-contact".into()),
                },
                OutsideParticipant {
                    display_name: None,
                    endpoint_channel: None,
                    endpoint_address: None,
                    saved_contact_id: None,
                }
            ],
            &contacts_map,
            &ehr_map,
        ),
        ChannelTieEvaluation::Contradictory {
            reason: ChannelTieConflict::MultipleOutsideParticipants,
            ..
        }
    ));
    assert!(matches!(
        evaluate_channel_tie(
            &[OutsideParticipant {
                display_name: None,
                endpoint_channel: None,
                endpoint_address: None,
                saved_contact_id: Some("shared-contact".into()),
            }],
            &contacts_map,
            &ehr_map,
        ),
        ChannelTieEvaluation::Contradictory {
            reason: ChannelTieConflict::ContactSharedAcrossSprucePatients,
            ..
        }
    ));
    assert!(matches!(
        evaluate_channel_tie(
            &[OutsideParticipant {
                display_name: None,
                endpoint_channel: None,
                endpoint_address: None,
                saved_contact_id: Some("multi-ehr-contact".into()),
            }],
            &contacts_map,
            &ehr_map,
        ),
        ChannelTieEvaluation::Contradictory {
            reason: ChannelTieConflict::MultipleEhrRecordsOnSprucePatient,
            ..
        }
    ));
    assert!(matches!(
        evaluate_channel_tie(
            &[OutsideParticipant {
                display_name: None,
                endpoint_channel: None,
                endpoint_address: None,
                saved_contact_id: Some("dup-coda-contact".into()),
            }],
            &contacts_map,
            &ehr_map,
        ),
        ChannelTieEvaluation::Contradictory {
            reason: ChannelTieConflict::MultipleCodaPatientsForEhrRecord,
            ..
        }
    ));

    let allowed_set = BTreeSet::from([pw_clara.clone(), pw_danny.clone(), pw_no_duet.clone()]);
    // Sender confirmed for Clara while channel ties to Martha -> WaitForConfirmation:
    assert_eq!(
        evaluate_automatic_link(
            &BTreeSet::from([pw_clara.clone()]),
            &ChannelTieEvaluation::UniquePatient {
                workspace: pw_no_duet.clone(),
            },
            None,
            &allowed_set,
        ),
        local_product::runtime::spruce_matching::AutomaticLinkEvaluation::WaitForConfirmation
    );
    // Patient with zero allowed physicians -> WaitForConfirmation:
    assert_eq!(
        evaluate_automatic_link(
            &BTreeSet::from([pw_unassigned.clone()]),
            &ChannelTieEvaluation::Untied,
            None,
            &allowed_set,
        ),
        local_product::runtime::spruce_matching::AutomaticLinkEvaluation::WaitForConfirmation
    );

    // End-to-end DB contradiction: sender confirmed for Clara, but model suggests Clara on an untied
    // unknown phone (`+14155550199`) -> MUST wait for confirmation (model suggestion alone NEVER auto-links!)
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                3,
                "msg-suggest-clara",
                "conv-suggest-clara",
                Some("+14155550199"),
                Some("Clara O."),
                "Hi doctor, this is Clara texting from my work phone.",
                vec![OutsideParticipant {
                    display_name: Some("Clara O.".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550199").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 4,
        )
        .await
        .unwrap();

    let outcome_suggest = store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-suggest-clara",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: pw_clara.clone(),
                    evidence_summary: "Message self-identifies as Clara O., matching Clara Oswald."
                        .into(),
                },
                rationale: "New work phone requires physician confirmation.".into(),
            },
            Some(&receiver),
            now_ts + 5,
        )
        .await
        .unwrap();
    assert_eq!(
        outcome_suggest,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: Some(pw_clara.clone()),
        }
    );

    // --- 1d. Access Enforcement (Rules 1, 2, 9) & Zero Confidence Scores ---
    // Dr. Alpha (allowed on Clara) sees `suggested_patient == Some(pw_clara)` with zero confidence fields:
    let alpha_waiting = store
        .messages_to_match_for_physician(auth.as_ref(), &dr_alpha)
        .await
        .unwrap();
    assert_eq!(alpha_waiting.len(), 1);
    assert_eq!(
        alpha_waiting[0]
            .suggested_patient
            .as_ref()
            .map(|s| s.workspace.as_str()),
        Some(pw_clara.as_str())
    );
    let alpha_json = serde_json::to_string(&alpha_waiting[0]).unwrap();
    assert!(
        !alpha_json.to_ascii_lowercase().contains("confidence")
            && !alpha_json.to_ascii_lowercase().contains("score"),
        "waiting row JSON must contain zero confidence scores: {alpha_json}"
    );

    // Dr. Beta (NOT allowed on Clara) sees the unmatched message row, but `suggested_patient` is hidden (`None`)!
    let beta_waiting = store
        .messages_to_match_for_physician(auth.as_ref(), &dr_beta)
        .await
        .unwrap();
    assert_eq!(beta_waiting.len(), 1);
    assert!(
        beta_waiting[0].suggested_patient.is_none(),
        "Rule 2: viewer without access to suggested patient must see suggested_patient == None"
    );

    // Dr. Beta cannot confirm onto Clara (`Rule 9`), cannot open Clara's linked conversation (`Rule 1`),
    // and cannot move Clara's linked conversation (`Rule 9`):
    assert!(matches!(
        store
            .confirm_spruce_message_link(
                auth.as_ref(),
                &dr_beta,
                "msg-suggest-clara",
                &pw_clara,
                SenderConfirmationPolicy::ConversationOnly,
                "cmd-beta-illegal-confirm",
                Some(&receiver),
                now_ts + 6,
            )
            .await,
        Err(SpruceMatchingError::AccessDenied { .. })
    ));
    assert!(matches!(
        store
            .open_spruce_conversation(auth.as_ref(), &dr_beta, "conv-auto-sender", now_ts + 6)
            .await,
        Err(SpruceMatchingError::AccessDenied { .. })
    ));
    assert!(matches!(
        store
            .move_spruce_conversation(
                auth.as_ref(),
                &dr_beta,
                MoveSpruceConversationCommand {
                    conversation_id: "conv-auto-sender".into(),
                    destination: MoveDestination::AnotherPatient {
                        workspace: pw_danny.clone(),
                    },
                    undo_window_seconds: 10,
                    command_key: "cmd-beta-illegal-move".into(),
                },
                None,
                now_ts + 6,
            )
            .await,
        Err(SpruceMatchingError::AccessDenied { .. })
    ));

    // Dr. Alpha opens `conv-auto-sender`, then Clara's panel access is revoked for Dr. Alpha ->
    // `open_spruce_conversations_for_physician` and `authorized_spruce_conversation_for_physician`
    // hide it immediately (`Rule 1`)!
    let opened = store
        .open_spruce_conversation(auth.as_ref(), &dr_alpha, "conv-auto-sender", now_ts + 7)
        .await
        .unwrap();
    assert_eq!(opened.messages.len(), 1);
    assert_eq!(
        store
            .open_spruce_conversations_for_physician(auth.as_ref(), &dr_alpha, now_ts + 7)
            .await
            .unwrap()
            .len(),
        1
    );
    auth.set_membership(&dr_alpha, &pw_clara, Membership::ClinicDemographic)
        .await
        .unwrap();
    assert!(
        store
            .open_spruce_conversations_for_physician(auth.as_ref(), &dr_alpha, now_ts + 8)
            .await
            .unwrap()
            .is_empty(),
        "revoking Membership::Allowed must immediately hide open linked conversations"
    );
    assert!(matches!(
        store
            .authorized_spruce_conversation_for_physician(
                auth.as_ref(),
                &dr_alpha,
                "conv-auto-sender",
                now_ts + 8,
            )
            .await,
        Err(SpruceMatchingError::AccessDenied { .. })
    ));
}

#[tokio::test]
async fn matching_recipe_runs_on_background_lane_records_provenance_and_handles_synthetic_cases_and_this_is_a_patient()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let now_ts: i64 = 1_762_970_000;
    let practice = format!("practice-recipe-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();

    let dr = invite_physician(&auth, &practice, "DrMatcher", now_ts as u64).await;
    let pw_alice = format!("pw-alice-{}", Uuid::new_v4().simple());
    let pw_ben_twin = format!("pw-ben-{}", Uuid::new_v4().simple());
    let pw_maya_twin = format!("pw-maya-{}", Uuid::new_v4().simple());
    let pw_spouse_husband = format!("pw-husband-{}", Uuid::new_v4().simple());
    let pw_spouse_wife = format!("pw-wife-{}", Uuid::new_v4().simple());

    for (ws, ehr_id, name, dob) in [
        (&pw_alice, "ehr-alice", "Alice Nguyen", "1979-02-14"),
        (&pw_ben_twin, "ehr-ben", "Ben Alvarez", "2018-06-10"),
        (&pw_maya_twin, "ehr-maya", "Maya Alvarez", "2018-06-10"),
        (
            &pw_spouse_husband,
            "ehr-husband",
            "David Park",
            "1972-09-01",
        ),
        (&pw_spouse_wife, "ehr-wife", "Hannah Park", "1974-12-19"),
    ] {
        seed_patient_with_chart(
            &store,
            &auth,
            &practice,
            ws,
            ehr_id,
            name,
            dob,
            SECRET_CHART_SENTINEL,
            now_ts,
        )
        .await;
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    // Confirm `+14155550204` for David Park (`pw_spouse_husband`) so we can prove caregiver/spouse
    // contradiction when David texts from his confirmed phone about his wife Hannah (`pw_spouse_wife`)!
    let david_phone = SenderAddress::parse("+14155550204").unwrap();
    store
        .record_spruce_confirmed_sender(
            &practice,
            &david_phone,
            &pw_spouse_husband,
            &dr.physician_id,
            now_ts,
        )
        .await
        .unwrap();

    // Shared parent phone `+14155550202` confirmed for BOTH siblings Ben and Maya Alvarez:
    let parent_phone = SenderAddress::parse("+14155550202").unwrap();
    store
        .record_spruce_confirmed_sender(
            &practice,
            &parent_phone,
            &pw_ben_twin,
            &dr.physician_id,
            now_ts,
        )
        .await
        .unwrap();
    store
        .record_spruce_confirmed_sender(
            &practice,
            &parent_phone,
            &pw_maya_twin,
            &dr.physician_id,
            now_ts,
        )
        .await
        .unwrap();

    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, now_ts)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-recipe", 300, now_ts)
        .await
        .unwrap();
    let receiver = RecordingReviewReceiver::default();

    // --- Synthetic Case 1: Unique patient match + provenance, initial_loads, skill_view, patient_lookup, finish barrier ---
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                1,
                "msg-syn-unique",
                "conv-syn-unique",
                Some("+14155550201"),
                Some("Alice N."),
                "Hi, this is Alice Nguyen (DOB 1979-02-14). Could we check whether my lisinopril refill went through?",
                vec![OutsideParticipant {
                    display_name: Some("Alice N.".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550201").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 1,
        )
        .await
        .unwrap();

    let first_skill_name = "patient-record-retrieval".to_owned();
    let mut provider_unique = ScriptedProvider::new(vec![
        // Turn 1: model attempts to finish prematurely before submitting classification -> Finish::Continue barrier!
        done_turn("Premature finish before classification."),
        // Turn 2: model opens a skill via `skill_view` and searches `patient_lookup` for `"Nguyen"`
        Ok(engine::Turn {
            items: vec![
                engine::Item::Call(engine::Invocation {
                    id: "call-skill-1".into(),
                    name: SKILL_VIEW.into(),
                    arguments: json!({ "name": first_skill_name }).to_string(),
                }),
                engine::Item::Call(engine::Invocation {
                    id: "call-lookup-alice".into(),
                    name: PATIENT_LOOKUP_TOOL.into(),
                    arguments: json!({ "query": "Nguyen" }).to_string(),
                }),
            ],
            receipt: json!({ "turn": 2 }),
            usage: Default::default(),
        }),
        // Turn 3: model submits `SuggestedPatient` for Alice
        call_turn(
            "call-submit-alice",
            SUBMIT_CLASSIFICATION_TOOL,
            json!({
                "classification": "patient_mail",
                "urgency": "ordinary",
                "match_outcome": "suggested_patient",
                "suggested_workspace": pw_alice,
                "evidence_summary": "Message states full name Alice Nguyen and birth date 1979-02-14 matching roster.",
                "rationale": "Routine medication refill inquiry from Alice Nguyen on an unconfirmed number."
            }),
        ),
        done_turn("Classified as patient mail with suggested match Alice Nguyen."),
    ]);

    let completed_unique = run_spruce_message_matching(
        &store,
        auth.as_ref(),
        &practice,
        "msg-syn-unique",
        &roles,
        &skills,
        &mut provider_unique,
        Some(&receiver),
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Verify background lane binding (never chat lane) & complete provenance:
    assert_eq!(completed_unique.run.status, MatchingRunStatus::Completed);
    assert_eq!(
        completed_unique.run.provenance.model,
        roles.background.model
    );
    assert_ne!(completed_unique.run.provenance.model, roles.chat.model);
    assert_eq!(
        completed_unique.run.provenance.effort,
        roles.background.effort
    );
    assert_eq!(
        completed_unique.run.provenance.context_window_tokens,
        roles.background.context_window_tokens
    );
    assert_eq!(
        completed_unique.run.provenance.max_input_tokens,
        roles.background.max_input_tokens
    );
    assert_eq!(
        completed_unique.run.provenance.recipe_version,
        RECIPE_VERSION
    );
    assert_eq!(
        completed_unique.run.provenance.guidance_version,
        GUIDANCE_VERSION
    );
    assert_eq!(
        completed_unique.run.provenance.guidance_sha256,
        instructions_sha256()
    );
    assert_eq!(provider_unique.requests[0].model, roles.background.model);

    // Verify StartingContext & patient_lookup never exposed SECRET_CHART_SENTINEL:
    let all_requests_wire = serde_json::to_string(&provider_unique.requests).unwrap();
    assert!(
        !all_requests_wire.contains(SECRET_CHART_SENTINEL),
        "matching run requests and tool outputs must never contain clinical chart content"
    );
    assert!(
        all_requests_wire.contains("classification_not_submitted"),
        "premature finish without submit_message_classification must trigger Finish::Continue feedback"
    );

    // Verify initial_loads & tool_discoveries & events recorded on run:
    assert_eq!(completed_unique.run.initial_loads.len(), 4);
    assert!(matches!(
        &completed_unique.run.initial_loads[0],
        MatchingInitialLoad::ResidentGuidance { version, .. } if version == GUIDANCE_VERSION
    ));
    assert!(matches!(
        &completed_unique.run.initial_loads[1],
        MatchingInitialLoad::SkillCatalogMetadata { .. }
    ));
    assert!(matches!(
        &completed_unique.run.initial_loads[2],
        MatchingInitialLoad::SavedMessageEnvelope { message_id, .. } if message_id == "msg-syn-unique"
    ));
    assert!(matches!(
        &completed_unique.run.initial_loads[3],
        MatchingInitialLoad::ConversationParticipants {
            outside_participant_count: 1,
            ..
        }
    ));

    let discovered_tools: Vec<&str> = completed_unique
        .run
        .tool_discoveries
        .iter()
        .map(|d| d.tool_name.as_str())
        .collect();
    assert_eq!(
        discovered_tools,
        vec![SKILL_VIEW, PATIENT_LOOKUP_TOOL, SUBMIT_CLASSIFICATION_TOOL]
    );
    assert!(
        completed_unique
            .run
            .tool_discoveries
            .iter()
            .all(|d| d.disposition == ToolDiscoveryDisposition::Returned)
    );
    assert!(!completed_unique.run.events.is_empty());

    // `spruce_matching` retains sole auto-link authority: unconfirmed phone waits with suggestion!
    assert_eq!(
        completed_unique.routing_outcome,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: Some(pw_alice.clone()),
        }
    );

    // --- Synthetic Case 2: Ambiguous sibling match (`AmbiguousCandidates` >= 2) ---
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                2,
                "msg-syn-siblings",
                "conv-syn-siblings",
                Some("+14155550202"),
                Some("Alvarez Parent"),
                "Our child woke up with a mild earache after school; do you have a sick visit tomorrow?",
                vec![OutsideParticipant {
                    display_name: Some("Alvarez Parent".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(parent_phone.clone()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 2,
        )
        .await
        .unwrap();

    let mut provider_siblings = ScriptedProvider::new(vec![
        call_turn(
            "lookup-siblings",
            PATIENT_LOOKUP_TOOL,
            json!({ "query": "Alvarez" }),
        ),
        call_turn(
            "submit-siblings",
            SUBMIT_CLASSIFICATION_TOOL,
            json!({
                "classification": "patient_mail",
                "urgency": "ordinary",
                "match_outcome": "ambiguous_candidates",
                "candidate_workspaces": [pw_ben_twin, pw_maya_twin],
                "evidence_summary": "Parent phone +14155550202 is shared by siblings Ben Alvarez and Maya Alvarez, and the message does not name which child has the earache.",
                "rationale": "Caregiver message about one of two siblings; withhold single guess."
            }),
        ),
        done_turn("Waiting for physician to choose between Ben Alvarez and Maya Alvarez."),
    ]);
    let completed_siblings = run_spruce_message_matching(
        &store,
        auth.as_ref(),
        &practice,
        "msg-syn-siblings",
        &roles,
        &skills,
        &mut provider_siblings,
        Some(&receiver),
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(
        completed_siblings.routing_outcome,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: None,
        }
    );

    // --- Synthetic Case 3: Conflicting caregiver/spouse message from confirmed phone (`ConflictingEvidence`) ---
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                3,
                "msg-syn-conflict",
                "conv-syn-conflict",
                Some("+14155550204"),
                Some("David Park"),
                "Hi doctor, this is David texting for my wife Hannah Park (DOB 1974-12-19) — she is having migraine nausea.",
                vec![OutsideParticipant {
                    display_name: Some("David Park".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(david_phone.clone()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 3,
        )
        .await
        .unwrap();

    let mut provider_conflict = ScriptedProvider::new(vec![
        call_turn(
            "lookup-park",
            PATIENT_LOOKUP_TOOL,
            json!({ "query": "Park" }),
        ),
        call_turn(
            "submit-conflict",
            SUBMIT_CLASSIFICATION_TOOL,
            json!({
                "classification": "patient_mail",
                "urgency": "ordinary",
                "match_outcome": "conflicting_evidence",
                "candidate_workspaces": [pw_spouse_husband, pw_spouse_wife],
                "evidence_summary": "Sender +14155550204 is confirmed for David Park, while the message explicitly concerns Hannah Park (DOB 1974-12-19).",
                "rationale": "Spouse writing from his own confirmed number about another patient; do not auto-link to David Park."
            }),
        ),
        done_turn("Held for confirmation due to conflicting sender vs named patient."),
    ]);
    let completed_conflict = run_spruce_message_matching(
        &store,
        auth.as_ref(),
        &practice,
        "msg-syn-conflict",
        &roles,
        &skills,
        &mut provider_conflict,
        Some(&receiver),
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    // Crucial Rule 4 check: even though `+14155550204` is confirmed for David Park, `ConflictingEvidence`
    // prevents wrong-patient auto-linking to David Park!
    assert_eq!(
        completed_conflict.routing_outcome,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: None,
        }
    );
    assert!(
        receiver
            .accepted_for_workspace(&pw_spouse_husband)
            .is_empty()
    );

    // --- Synthetic Case 4: Unknown sender (`NoMatchFound`) ---
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                4,
                "msg-syn-nomatch",
                "conv-syn-nomatch",
                Some("+14155550209"),
                None,
                "Hi, I am a prospective patient hoping to transfer my records next month.",
                vec![OutsideParticipant {
                    display_name: None,
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550209").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 4,
        )
        .await
        .unwrap();

    let mut provider_nomatch = ScriptedProvider::new(vec![
        call_turn("lookup-all", PATIENT_LOOKUP_TOOL, json!({ "query": null })),
        call_turn(
            "submit-nomatch",
            SUBMIT_CLASSIFICATION_TOOL,
            json!({
                "classification": "patient_mail",
                "urgency": "ordinary",
                "match_outcome": "no_match_found",
                "evidence_summary": "No registered patient matches +14155550209 and no patient name or birth date is stated.",
                "rationale": "Patient inquiry with no roster match."
            }),
        ),
        done_turn("Unmatched patient inquiry with no roster match."),
    ]);
    let completed_nomatch = run_spruce_message_matching(
        &store,
        auth.as_ref(),
        &practice,
        "msg-syn-nomatch",
        &roles,
        &skills,
        &mut provider_nomatch,
        Some(&receiver),
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(
        completed_nomatch.routing_outcome,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: None,
        }
    );

    // --- Synthetic Case 5: Vendor Fax Spam over Real Local `ResponsesProvider` HTTP SSE Wire + `"This is a patient"` Override ---
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                5,
                "msg-syn-spam",
                "conv-syn-spam",
                Some("+18005559999"),
                Some("MedSupply Deals"),
                "LIMITED TIME FAX PROMO: Save 40% on exam table paper rolls this quarter!",
                vec![OutsideParticipant {
                    display_name: Some("MedSupply Deals".into()),
                    endpoint_channel: Some("fax".into()),
                    endpoint_address: Some(SenderAddress::parse("+18005559999").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 5,
        )
        .await
        .unwrap();

    // Spin up a local `axum` SSE server speaking OpenAI `/v1/responses` streaming protocol:
    #[derive(Clone, Default)]
    struct MockSseState {
        bodies: Arc<Mutex<Vec<Value>>>,
    }
    let sse_state = MockSseState::default();
    let app = Router::new()
        .route(
            "/v1/responses",
            post(|State(state): State<MockSseState>, Json(body): Json<Value>| async move {
                let call_idx = {
                    let mut guard = state.bodies.lock().unwrap();
                    guard.push(body);
                    guard.len()
                };
                let (done_item, response_obj) = if call_idx == 1 {
                    let item = json!({
                        "type": "function_call",
                        "status": "completed",
                        "call_id": "sse-submit-spam",
                        "name": SUBMIT_CLASSIFICATION_TOOL,
                        "arguments": json!({
                            "classification": "not_patient_mail",
                            "non_patient_category": "vendor_or_spam",
                            "rationale": "Commercial medical supply fax marketing blast, not from or about a patient."
                        }).to_string()
                    });
                    let resp = json!({
                        "id": "resp-sse-1",
                        "model": "background-spruce-matching-lane",
                        "status": "completed",
                        "output": [item],
                        "usage": { "input_tokens": 320, "output_tokens": 45 }
                    });
                    (item, resp)
                } else {
                    let item = json!({
                        "type": "message",
                        "role": "assistant",
                        "status": "completed",
                        "content": [{
                            "type": "output_text",
                            "text": "Filtered commercial vendor fax blast."
                        }]
                    });
                    let resp = json!({
                        "id": "resp-sse-2",
                        "model": "background-spruce-matching-lane",
                        "status": "completed",
                        "output": [item],
                        "usage": { "input_tokens": 380, "output_tokens": 12 }
                    });
                    (item, resp)
                };
                let sse_payload = format!(
                    "event: response.output_item.done\ndata: {}\n\nevent: response.completed\ndata: {}\n\n",
                    json!({
                        "type": "response.output_item.done",
                        "output_index": 0,
                        "item": done_item,
                    }),
                    json!({
                        "type": "response.completed",
                        "response": response_obj,
                    })
                );
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "text/event-stream")],
                    sse_payload,
                )
                    .into_response()
            }),
        )
        .with_state(sse_state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server_task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let mut wire_provider = ResponsesProvider::new(
        format!("http://{addr}/v1/responses"),
        "local-sse-key".into(),
        Arc::new(store.clone()),
    )
    .unwrap();

    let completed_spam = run_spruce_message_matching(
        &store,
        auth.as_ref(),
        &practice,
        "msg-syn-spam",
        &roles,
        &skills,
        &mut wire_provider,
        Some(&receiver),
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    server_task.abort();

    assert_eq!(
        completed_spam.routing_outcome,
        MessageRoutingOutcome::Filtered {
            category: NonPatientCategory::VendorOrSpam,
        }
    );
    assert_eq!(sse_state.bodies.lock().unwrap().len(), 2);
    assert_eq!(
        sse_state.bodies.lock().unwrap()[0]["model"],
        json!(roles.background.model)
    );

    // Filtered list shows `msg-syn-spam`; physician clicks `"This is a patient"` (`mark_spruce_message_as_patient`) ->
    // message leaves `Filtered`, records `physician_override`, and routes to `Waiting`!
    let filtered_before = store
        .filtered_spruce_messages_for_physician(auth.as_ref(), &dr)
        .await
        .unwrap();
    assert_eq!(filtered_before.len(), 1);
    assert_eq!(filtered_before[0].message_id, "msg-syn-spam");

    let rerouted = store
        .mark_spruce_message_as_patient(
            auth.as_ref(),
            &dr,
            "msg-syn-spam",
            "cmd-this-is-patient-1",
            Some(&receiver),
            now_ts + 10,
        )
        .await
        .unwrap();
    assert_eq!(
        rerouted,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: None,
        }
    );
    // Idempotent replay of `"This is a patient"` with same `command_key` returns same `Waiting` outcome:
    let rerouted_replay = store
        .mark_spruce_message_as_patient(
            auth.as_ref(),
            &dr,
            "msg-syn-spam",
            "cmd-this-is-patient-1",
            Some(&receiver),
            now_ts + 11,
        )
        .await
        .unwrap();
    assert_eq!(rerouted_replay, rerouted);

    assert!(
        store
            .filtered_spruce_messages_for_physician(auth.as_ref(), &dr)
            .await
            .unwrap()
            .is_empty()
    );
    let saved_rerouted = store
        .spruce_message(&practice, "msg-syn-spam")
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        saved_rerouted.standing,
        MessageStanding::Waiting {
            origin: RoutingOrigin::PhysicianMarkedPatient { .. },
            ..
        }
    ));
    assert!(
        saved_rerouted
            .saved_classification
            .as_ref()
            .and_then(|c| c.physician_override.as_ref())
            .is_some()
    );

    // Also verify `"This is a patient"` on a message whose conversation IS uniquely tied via Rule 4
    // routes straight to `Linked`!
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                6,
                "msg-admin-notice",
                "conv-admin-david",
                Some("+14155550204"),
                Some("Payer Directory Bot"),
                "Automated quarterly fax test for David Park's line.",
                vec![OutsideParticipant {
                    display_name: Some("Payer Directory Bot".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(david_phone),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 12,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-admin-notice",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::NotPatientMail {
                category: NonPatientCategory::AdministrativeNotice,
                rationale: "Directory verification notice.".into(),
            },
            Some(&receiver),
            now_ts + 13,
        )
        .await
        .unwrap();
    let rerouted_linked = store
        .mark_spruce_message_as_patient(
            auth.as_ref(),
            &dr,
            "msg-admin-notice",
            "cmd-this-is-patient-david",
            Some(&receiver),
            now_ts + 14,
        )
        .await
        .unwrap();
    assert!(matches!(
        rerouted_linked,
        MessageRoutingOutcome::Linked {
            ref workspace,
            origin: LinkOrigin::AutomaticRule4,
            ..
        } if workspace == &pw_spouse_husband
    ));
}

#[tokio::test]
async fn urgent_unmatched_message_stays_unlinked_cannot_read_charts_and_confirm_replay_emits_one_review()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let now_ts: i64 = 1_762_980_000;
    let practice = format!("practice-urgent-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();

    let dr = invite_physician(&auth, &practice, "DrUrgent", now_ts as u64).await;
    let pw_target = format!("pw-urgent-target-{}", Uuid::new_v4().simple());
    seed_patient_with_chart(
        &store,
        &auth,
        &practice,
        &pw_target,
        "ehr-urgent-1",
        "Elena Vance",
        "1965-07-22",
        SECRET_CHART_SENTINEL,
        now_ts,
    )
    .await;
    auth.set_membership(&dr, &pw_target, Membership::Allowed)
        .await
        .unwrap();

    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, now_ts)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-urgent", 300, now_ts)
        .await
        .unwrap();
    let receiver = RecordingReviewReceiver::default();

    // Seed an earlier ordinary waiting message at `now_ts + 1` so we can verify urgent `Now` ordering!
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                1,
                "msg-routine-first",
                "conv-routine-first",
                Some("+14155550301"),
                Some("Routine Sender"),
                "Can I get a copy of last year's immunization form?",
                vec![OutsideParticipant {
                    display_name: Some("Routine Sender".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550301").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 1,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-routine-first",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::NoMatchFound {
                    evidence_summary: "Unknown sender.".into(),
                },
                rationale: "Routine administrative form request.".into(),
            },
            Some(&receiver),
            now_ts + 2,
        )
        .await
        .unwrap();

    // Ingest an URGENT unmatched chest-pain message at `now_ts + 10`:
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                2,
                "msg-urgent-chest",
                "conv-urgent-chest",
                Some("+14155550399"),
                Some("Elena V."),
                "Having crushing chest pressure and shortness of breath for the past 20 minutes — Elena.",
                vec![OutsideParticipant {
                    display_name: Some("Elena V.".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550399").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 10,
        )
        .await
        .unwrap();

    // Adversarial provider attempts `read_patient_sources`, `search_patient_information`,
    // `patient_concerns`, and `patient_memory` before calling `patient_lookup` and `submit_message_classification`!
    let mut adversarial_provider = ScriptedProvider::new(vec![
        Ok(engine::Turn {
            items: vec![
                engine::Item::Call(engine::Invocation {
                    id: "illegal-read-chart".into(),
                    name: READ_TOOL.into(),
                    arguments: json!({
                        "references": [{ "candidate_id": "DiagnosticReport/lab-1", "revision": "rev-1", "cursor": null }],
                        "max_result_bytes": 4096
                    })
                    .to_string(),
                }),
                engine::Item::Call(engine::Invocation {
                    id: "illegal-search-chart".into(),
                    name: SEARCH_TOOL.into(),
                    arguments: json!({ "query": "biopsy" }).to_string(),
                }),
                engine::Item::Call(engine::Invocation {
                    id: "illegal-concern".into(),
                    name: CONCERN_TOOL.into(),
                    arguments: json!({ "operation": "list" }).to_string(),
                }),
                engine::Item::Call(engine::Invocation {
                    id: "illegal-memory".into(),
                    name: "list_patient_memory".into(),
                    arguments: json!({}).to_string(),
                }),
            ],
            receipt: json!({ "turn": 1 }),
            usage: Default::default(),
        }),
        call_turn(
            "lookup-elena",
            PATIENT_LOOKUP_TOOL,
            json!({ "query": "Elena" }),
        ),
        call_turn(
            "submit-urgent",
            SUBMIT_CLASSIFICATION_TOOL,
            json!({
                "classification": "patient_mail",
                "urgency": "urgent",
                "urgent_reason": "Acute crushing chest pressure with dyspnea.",
                "match_outcome": "suggested_patient",
                "suggested_workspace": pw_target,
                "evidence_summary": "First name Elena matches registered patient Elena Vance; sender phone is unconfirmed.",
                "rationale": "Acute cardiopulmonary symptoms from unconfirmed phone; flag urgent Now without auto-linking."
            }),
        ),
        done_turn("Flagged urgent unmatched message as Now with suggested patient Elena Vance."),
    ]);

    let completed_urgent = run_spruce_message_matching(
        &store,
        auth.as_ref(),
        &practice,
        "msg-urgent-chest",
        &roles,
        &skills,
        &mut adversarial_provider,
        Some(&receiver),
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // 1. Every clinical chart and physician-only tool call was REFUSED and recorded on `tool_discoveries`:
    let refused: Vec<_> = completed_urgent
        .run
        .tool_discoveries
        .iter()
        .filter(|d| matches!(d.disposition, ToolDiscoveryDisposition::Refused { .. }))
        .collect();
    assert_eq!(refused.len(), 4);
    for expected_refused in [READ_TOOL, SEARCH_TOOL, CONCERN_TOOL, "list_patient_memory"] {
        assert!(
            refused.iter().any(|d| d.tool_name == expected_refused),
            "expected `{expected_refused}` to be recorded as refused"
        );
    }

    // 2. Neither provider requests nor run trace contain `SECRET_CHART_SENTINEL`:
    let urgent_wire = serde_json::to_string(&adversarial_provider.requests).unwrap();
    let urgent_run_json = serde_json::to_string(&completed_urgent.run).unwrap();
    assert!(!urgent_wire.contains(SECRET_CHART_SENTINEL));
    assert!(!urgent_run_json.contains(SECRET_CHART_SENTINEL));

    // 3. Urgent message stays UNLINKED (`Waiting`), zero reviews triggered while unlinked,
    // and leads Today's `"Messages to match"` as `Now` ahead of the earlier routine message!
    assert_eq!(
        completed_urgent.routing_outcome,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: Some(pw_target.clone()),
        }
    );
    assert!(receiver.accepted_requests().is_empty());

    let conv_before_confirm = store
        .spruce_conversation(&practice, "conv-urgent-chest")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        conv_before_confirm.link_state,
        ConversationLinkState::Unlinked
    );

    let today = store
        .today_spruce_summary_for_physician(auth.as_ref(), &dr)
        .await
        .unwrap();
    assert_eq!(today.messages_to_match.len(), 2);
    assert_eq!(today.messages_to_match[0].message_id, "msg-urgent-chest");
    assert!(today.messages_to_match[0].leads_today_as_now);
    assert!(matches!(
        &today.messages_to_match[0].urgency,
        MessageUrgency::Urgent { reason, .. } if reason.contains("chest pressure")
    ));
    assert_eq!(today.messages_to_match[1].message_id, "msg-routine-first");
    assert!(!today.messages_to_match[1].leads_today_as_now);

    // 4. Confirming `msg-urgent-chest` creates ONE link and ONE review request; replaying the same
    // `command_key` (including after simulated receiver outage + recovery) produces ONE link and ONE review!
    let confirm_1 = store
        .confirm_spruce_message_link(
            auth.as_ref(),
            &dr,
            "msg-urgent-chest",
            &pw_target,
            SenderConfirmationPolicy::ConfirmSenderForPatient,
            "cmd-confirm-urgent-elena",
            Some(&receiver),
            now_ts + 20,
        )
        .await
        .unwrap();
    assert_eq!(confirm_1.acceptance, LinkCommandAcceptance::New);
    assert_eq!(confirm_1.linked_message_ids, vec!["msg-urgent-chest"]);
    assert_eq!(receiver.accepted_for_workspace(&pw_target).len(), 1);
    assert!(
        receiver.accepted_for_workspace(&pw_target)[0]
            .record_versions
            .contains_key("spruce:msg-urgent-chest"),
        "ReviewRequest.record_versions must include the linked Spruce message version"
    );

    let confirm_replay = store
        .confirm_spruce_message_link(
            auth.as_ref(),
            &dr,
            "msg-urgent-chest",
            &pw_target,
            SenderConfirmationPolicy::ConfirmSenderForPatient,
            "cmd-confirm-urgent-elena",
            Some(&receiver),
            now_ts + 21,
        )
        .await
        .unwrap();
    assert_eq!(confirm_replay.acceptance, LinkCommandAcceptance::Duplicate);
    assert_eq!(
        confirm_replay.review_command_key,
        confirm_1.review_command_key
    );
    assert_eq!(confirm_replay.linked_message_ids, vec!["msg-urgent-chest"]);
    assert_eq!(
        receiver.accepted_for_workspace(&pw_target).len(),
        1,
        "confirm command replay must produce at most one ReviewRequest"
    );

    // Different payload under same `command_key` collides cleanly (`IdempotencyCollision`):
    assert!(matches!(
        store
            .confirm_spruce_message_link(
                auth.as_ref(),
                &dr,
                "msg-routine-first",
                &pw_target,
                SenderConfirmationPolicy::ConversationOnly,
                "cmd-confirm-urgent-elena",
                Some(&receiver),
                now_ts + 22,
            )
            .await,
        Err(SpruceMatchingError::IdempotencyCollision { .. })
    ));
}

#[tokio::test]
async fn fifteen_minute_fallback_join_siblings_leave_unlinked_channel_lifecycle_moves_isolation_and_review_inputs()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts: i64 = 1_762_990_000;
    let practice = format!("practice-lifecycle-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();

    let dr_all = invite_physician(&auth, &practice, "DrAll", now_ts as u64).await;
    let dr_only_a = invite_physician(&auth, &practice, "DrOnlyA", now_ts as u64).await;

    let pw_a = format!("pw-a-{}", Uuid::new_v4().simple());
    let pw_b = format!("pw-b-{}", Uuid::new_v4().simple());
    let pw_c = format!("pw-c-{}", Uuid::new_v4().simple());

    for (ws, ehr_id, name) in [
        (&pw_a, "ehr-a", "Patient Alpha"),
        (&pw_b, "ehr-b", "Patient Bravo"),
        (&pw_c, "ehr-c", "Patient Charlie"),
    ] {
        seed_patient_with_chart(
            &store,
            &auth,
            &practice,
            ws,
            ehr_id,
            name,
            "1980-01-01",
            "Baseline chart note.",
            now_ts,
        )
        .await;
        auth.set_membership(&dr_all, ws, Membership::Allowed)
            .await
            .unwrap();
    }
    auth.set_membership(&dr_only_a, &pw_a, Membership::Allowed)
        .await
        .unwrap();

    let receiver = RecordingReviewReceiver::default();

    // --- 1. Connect at `channel_head_position = 5`: Backlog Offer (`Rule 14`) & Single Reader Lease ---
    let connected = store
        .connect_spruce_channel(auth.as_ref(), &dr_all, 5, Some("cursor-5".into()), now_ts)
        .await
        .unwrap();
    assert!(matches!(
        connected,
        ChannelStanding::Connected {
            place: 5,
            start_boundary: 5,
            backlog: BacklogStanding::Offered { .. },
            ..
        }
    ));

    let _lease_1 = store
        .acquire_spruce_reader_lease(&practice, "reader-1", 60, now_ts)
        .await
        .unwrap();
    // Second reader while `lease_1` is active is refused (`ReaderLeaseHeld`):
    assert!(matches!(
        store
            .acquire_spruce_reader_lease(&practice, "reader-2", 60, now_ts + 10)
            .await,
        Err(SpruceMatchingError::ReaderLeaseHeld { .. })
    ));
    // Once `lease_1` expires (`now_ts + 61`), `reader-2` can take over cleanly:
    let lease = store
        .acquire_spruce_reader_lease(&practice, "reader-2", 3_600, now_ts + 61)
        .await
        .unwrap();

    // Pre-connection item (`pos = 3 <= 5`) is skipped while backlog offer is unaccepted (`Rule 14`):
    let skipped = store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                3,
                "msg-backlog-3",
                "conv-backlog",
                Some("+14155550400"),
                Some("Backlog Patient"),
                "Pre-connection question from yesterday.",
                vec![],
            ),
            now_ts + 62,
        )
        .await
        .unwrap();
    assert_eq!(skipped, SpruceIngestOutcome::SkippedPreConnectionBacklog);

    // Physician accepts backlog offer -> `msg-backlog-3` is now kept (`KeptNew`), and backlog offer cannot be accepted twice!
    store
        .accept_spruce_backlog(auth.as_ref(), &dr_all, now_ts + 63)
        .await
        .unwrap();
    assert!(matches!(
        store
            .accept_spruce_backlog(auth.as_ref(), &dr_all, now_ts + 64)
            .await,
        Err(SpruceMatchingError::ChannelStateRefused { .. })
    ));
    let kept_backlog = store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                3,
                "msg-backlog-3",
                "conv-backlog",
                Some("+14155550400"),
                Some("Backlog Patient"),
                "Pre-connection question from yesterday.",
                vec![],
            ),
            now_ts + 65,
        )
        .await
        .unwrap();
    assert!(matches!(kept_backlog, SpruceIngestOutcome::KeptNew { .. }));

    // --- 2. Duplicate Delivery Idempotency & Contiguous `advance_spruce_channel_place` (`Rule 12`) ---
    // Deliver pos 7 before pos 6: `place` stays at 5 until pos 6 is kept, then advances to 7!
    let t_base = now_ts + 100;
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                7,
                "msg-15min-7",
                "conv-15min",
                Some("+14155550407"),
                Some("Undecided Patient"),
                "Waiting for 15-minute fallback.",
                vec![],
            ),
            t_base,
        )
        .await
        .unwrap();
    // Duplicate redelivery of `msg-15min-7` returns `AlreadyKept`:
    let dup = store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                7,
                "msg-15min-7",
                "conv-15min",
                Some("+14155550407"),
                Some("Undecided Patient"),
                "Waiting for 15-minute fallback.",
                vec![],
            ),
            t_base + 1,
        )
        .await
        .unwrap();
    assert!(matches!(dup, SpruceIngestOutcome::AlreadyKept { .. }));
    assert_eq!(
        store
            .advance_spruce_channel_place(&practice, &lease.lease_token, None, t_base + 1)
            .await
            .unwrap(),
        5,
        "place must not advance past missing gap at position 6"
    );

    // Deliver unreadable item at pos 6 (`Rule 13`):
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Unreadable(UnreadableSpruceItem {
                message_id: "msg-unreadable-6".into(),
                conversation_id: "conv-unreadable-6".into(),
                channel_position: 6,
                content_version: "ver-err-6".into(),
                failure_reason: "Voicemail audio still transcribing".into(),
                app_url: Some("https://app.sprucehealth.com/items/6".into()),
                outside_participants: vec![],
            }),
            t_base + 2,
        )
        .await
        .unwrap();
    assert_eq!(
        store
            .advance_spruce_channel_place(
                &practice,
                &lease.lease_token,
                Some("cursor-7".into()),
                t_base + 3
            )
            .await
            .unwrap(),
        7,
        "once position 6 is kept, place advances contiguously through 7"
    );

    // --- 3. Unreadable Items (`Rule 13`) & Channel Failures (`Rule 10`) ---
    // Add second unreadable item at pos 8 (`msg-unreadable-8`) to test `Done`, while `msg-unreadable-6` tests `CodaReadsItNow`:
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Unreadable(UnreadableSpruceItem {
                message_id: "msg-unreadable-8".into(),
                conversation_id: "conv-unreadable-8".into(),
                channel_position: 8,
                content_version: "ver-err-8".into(),
                failure_reason: "Unsupported fax attachment encoding".into(),
                app_url: Some("https://app.sprucehealth.com/items/8".into()),
                outside_participants: vec![],
            }),
            t_base + 4,
        )
        .await
        .unwrap();
    let today_unreadable = store
        .today_spruce_summary_for_physician(auth.as_ref(), &dr_all)
        .await
        .unwrap();
    assert_eq!(today_unreadable.unreadable_messages.len(), 2);

    // `CodaReadsItNow`: `msg-unreadable-6` becomes readable -> `RecoveredUnreadable` (`Deciding`) and leaves `unreadable_messages`!
    let recovered = store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                6,
                "msg-unreadable-6",
                "conv-unreadable-6",
                Some("+14155550406"),
                Some("Recovered Voicemail"),
                "Transcribed voicemail body.",
                vec![],
            ),
            t_base + 5,
        )
        .await
        .unwrap();
    assert!(matches!(
        recovered,
        SpruceIngestOutcome::RecoveredUnreadable { .. }
    ));

    // Physician marks `msg-unreadable-8` `Done` -> leaves `unreadable_messages`!
    let marked_done = store
        .mark_unreadable_spruce_message_done(
            auth.as_ref(),
            &dr_all,
            "msg-unreadable-8",
            "cmd-done-8",
            t_base + 6,
        )
        .await
        .unwrap();
    assert!(matches!(
        marked_done.standing,
        MessageStanding::MarkedDone { .. }
    ));
    assert!(
        store
            .today_spruce_summary_for_physician(auth.as_ref(), &dr_all)
            .await
            .unwrap()
            .unreadable_messages
            .is_empty()
    );

    // Transient failure (`CannotRead`, no `Reconnect`) vs Revoked (`ReconnectRequired` -> `reconnect_spruce_channel` preserving `place == 7`):
    store
        .record_spruce_channel_failure(
            &practice,
            &lease.lease_token,
            ChannelStopCause::Transient {
                reason: "Upstream Spruce 503".into(),
            },
            t_base + 7,
        )
        .await
        .unwrap();
    assert!(matches!(
        store
            .today_spruce_summary_for_physician(auth.as_ref(), &dr_all)
            .await
            .unwrap()
            .channel_banner,
        TodayChannelBanner::CannotRead { .. }
    ));
    // `reconnect_spruce_channel` is refused on a transient failure (Coda never asks to reconnect on transient errors!):
    assert!(matches!(
        store
            .reconnect_spruce_channel(auth.as_ref(), &dr_all, t_base + 8)
            .await,
        Err(SpruceMatchingError::ChannelStateRefused { .. })
    ));
    store
        .recover_spruce_channel_reading(&practice, &lease.lease_token, t_base + 9)
        .await
        .unwrap();
    assert_eq!(
        store
            .today_spruce_summary_for_physician(auth.as_ref(), &dr_all)
            .await
            .unwrap()
            .channel_banner,
        TodayChannelBanner::Quiet
    );

    store
        .record_spruce_channel_failure(
            &practice,
            &lease.lease_token,
            ChannelStopCause::RevokedOrBlocked {
                reason: "API key revoked in Spruce admin".into(),
            },
            t_base + 10,
        )
        .await
        .unwrap();
    assert!(matches!(
        store
            .today_spruce_summary_for_physician(auth.as_ref(), &dr_all)
            .await
            .unwrap()
            .channel_banner,
        TodayChannelBanner::ReconnectRequired { .. }
    ));
    let reconnected = store
        .reconnect_spruce_channel(auth.as_ref(), &dr_all, t_base + 11)
        .await
        .unwrap();
    assert!(matches!(
        reconnected,
        ChannelStanding::Connected {
            place: 7,
            health: ChannelHealth::Reading { .. },
            ..
        }
    ));

    // --- 4. 15-Minute Undecided Fallback (`Rule 6`) ---
    // At `t_base + 899`, `msg-15min-7` (kept at `t_base`) stays `Deciding`; at `t_base + 900`, it routes to `Waiting`!
    let before_deadline = store
        .settle_undecided_spruce_messages(
            auth.as_ref(),
            &practice,
            Some(&receiver),
            t_base + FIFTEEN_MINUTES_SECONDS - 1,
        )
        .await
        .unwrap();
    assert!(
        !before_deadline
            .succeeded
            .iter()
            .any(|(id, _)| id == "msg-15min-7"),
        "msg-15min-7 must remain Deciding before its 15-minute deadline"
    );

    let at_deadline = store
        .settle_undecided_spruce_messages(
            auth.as_ref(),
            &practice,
            Some(&receiver),
            t_base + FIFTEEN_MINUTES_SECONDS,
        )
        .await
        .unwrap();
    assert!(at_deadline.succeeded.iter().any(
        |(id, out)| id == "msg-15min-7" && matches!(out, MessageRoutingOutcome::Waiting { .. })
    ));

    // --- 5. `LeaveUnlinked` vs Sibling `Waiting` Join (`Join`) + Subsequent `Rule 3` Auto-Join ---
    let t_sib = t_base + 1_000;
    for (pos, mid, text) in [
        (9, "msg-sib-1", "First message left unlinked."),
        (10, "msg-sib-2", "Second message waits again."),
        (11, "msg-sib-3", "Third message also waits."),
    ] {
        store
            .ingest_spruce_item(
                &practice,
                &lease.lease_token,
                &readable_item(
                    pos,
                    mid,
                    "conv-siblings-join",
                    Some("+14155550450"),
                    Some("Alpha Thread"),
                    text,
                    vec![],
                ),
                t_sib + pos as i64,
            )
            .await
            .unwrap();
        store
            .record_spruce_message_classification(
                auth.as_ref(),
                &practice,
                mid,
                None,
                RECIPE_VERSION,
                GUIDANCE_VERSION,
                MatchingDecision::PatientMail {
                    urgency: MessageUrgencyChoice::Ordinary,
                    assessment: MatchAssessment::SuggestedPatient {
                        workspace: pw_a.clone(),
                        evidence_summary: "Thread candidate Alpha.".into(),
                    },
                    rationale: "Patient message.".into(),
                },
                Some(&receiver),
                t_sib + pos as i64 + 10,
            )
            .await
            .unwrap();
    }

    // Leave `msg-sib-1` unlinked: `msg-sib-1` becomes `LeftUnlinked`, while `conv-siblings-join` stays `Unlinked`
    // and `msg-sib-2` + `msg-sib-3` stay `Waiting`!
    let left_m1 = store
        .leave_spruce_message_unlinked(
            auth.as_ref(),
            &dr_all,
            "msg-sib-1",
            "cmd-leave-sib-1",
            t_sib + 30,
        )
        .await
        .unwrap();
    assert!(matches!(
        left_m1.standing,
        MessageStanding::LeftUnlinked { .. }
    ));

    // Confirming `msg-sib-2` onto `pw_a` links BOTH waiting siblings (`msg-sib-2` & `msg-sib-3`) together,
    // while `msg-sib-1` remains `LeftUnlinked` (`ever_linked == false`)!
    let confirmed_sibs = store
        .confirm_spruce_message_link(
            auth.as_ref(),
            &dr_all,
            "msg-sib-2",
            &pw_a,
            SenderConfirmationPolicy::ConversationOnly,
            "cmd-confirm-sib-2",
            Some(&receiver),
            t_sib + 31,
        )
        .await
        .unwrap();
    assert_eq!(
        confirmed_sibs.linked_message_ids,
        vec!["msg-sib-2", "msg-sib-3"]
    );
    assert!(
        !store
            .spruce_message(&practice, "msg-sib-1")
            .await
            .unwrap()
            .unwrap()
            .ever_linked
    );

    // Later message `msg-sib-4` arriving in `conv-siblings-join` joins `pw_a` automatically (`Rule 3`):
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                12,
                "msg-sib-4",
                "conv-siblings-join",
                Some("+14155550450"),
                Some("Alpha Thread"),
                "Fourth message in already-linked conversation.",
                vec![],
            ),
            t_sib + 32,
        )
        .await
        .unwrap();
    let sib4_out = store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-sib-4",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::NoMatchFound {
                    evidence_summary: "Already linked conversation.".into(),
                },
                rationale: "Follow-up in linked conversation.".into(),
            },
            Some(&receiver),
            t_sib + 33,
        )
        .await
        .unwrap();
    assert!(matches!(
        sib4_out,
        MessageRoutingOutcome::Linked {
            ref workspace,
            origin: LinkOrigin::JoinedExistingConversationLink,
            ..
        } if workspace == &pw_a
    ));

    // --- 6. Wrong-Patient `Move`, Multi-Hop `Undo`, `NotAPatient` + `Undo`, Settlement, & Isolation (`Rules 5, 7, 8`) ---
    // `dr_only_a` opens `conv-siblings-join` on `pw_a`:
    store
        .open_spruce_conversation(auth.as_ref(), &dr_only_a, "conv-siblings-join", t_sib + 40)
        .await
        .unwrap();

    // Move `A -> B`: immediately disappears from `pw_a` (`Rule 5`), drops `dr_only_a` from `open_viewers` (`Rule 1`),
    // shows on `pw_b` with `move_undo.from_workspace == pw_a`, and emits ZERO reviews while `Undo` holds!
    let reviews_before_move = receiver.accepted_requests().len();
    let moved_b = store
        .move_spruce_conversation(
            auth.as_ref(),
            &dr_all,
            MoveSpruceConversationCommand {
                conversation_id: "conv-siblings-join".into(),
                destination: MoveDestination::AnotherPatient {
                    workspace: pw_b.clone(),
                },
                undo_window_seconds: 10,
                command_key: "cmd-move-a-to-b".into(),
            },
            Some(&receiver),
            t_sib + 41,
        )
        .await
        .unwrap();
    assert_eq!(
        moved_b
            .move_undo
            .as_ref()
            .map(|u| u.from_workspace.as_str()),
        Some(pw_a.as_str())
    );
    assert!(!moved_b.open_viewers.contains(&dr_only_a.physician_id));
    assert!(
        store
            .patient_linked_spruce_messages(&practice, &pw_a)
            .await
            .unwrap()
            .is_empty(),
        "Rule 5: moved conversation must vanish from origin patient immediately"
    );
    assert_eq!(
        store
            .patient_linked_spruce_messages(&practice, &pw_b)
            .await
            .unwrap()
            .len(),
        3
    );
    assert_eq!(receiver.accepted_requests().len(), reviews_before_move);

    // Move again `B -> C` while `Undo` holds: `from_workspace` stays `pw_a`! Then move `C -> A`: `move_undo` clears with ZERO reviews!
    let moved_c = store
        .move_spruce_conversation(
            auth.as_ref(),
            &dr_all,
            MoveSpruceConversationCommand {
                conversation_id: "conv-siblings-join".into(),
                destination: MoveDestination::AnotherPatient {
                    workspace: pw_c.clone(),
                },
                undo_window_seconds: 10,
                command_key: "cmd-move-b-to-c".into(),
            },
            Some(&receiver),
            t_sib + 42,
        )
        .await
        .unwrap();
    assert_eq!(
        moved_c
            .move_undo
            .as_ref()
            .map(|u| u.from_workspace.as_str()),
        Some(pw_a.as_str())
    );
    let moved_back_a = store
        .move_spruce_conversation(
            auth.as_ref(),
            &dr_all,
            MoveSpruceConversationCommand {
                conversation_id: "conv-siblings-join".into(),
                destination: MoveDestination::AnotherPatient {
                    workspace: pw_a.clone(),
                },
                undo_window_seconds: 10,
                command_key: "cmd-move-c-to-a".into(),
            },
            Some(&receiver),
            t_sib + 43,
        )
        .await
        .unwrap();
    assert!(moved_back_a.move_undo.is_none());
    assert_eq!(receiver.accepted_requests().len(), reviews_before_move);

    // Move `A -> NotAPatient`, receive `msg-sib-5` while `Undo` shows (`NotAPatient`), then press `Undo` ->
    // ALL messages (`msg-sib-2`, `msg-sib-3`, `msg-sib-4`, AND `msg-sib-5`) restore onto `pw_a` (`Rule 7`),
    // AND pressing `Undo` queues one review on `pw_a` because `msg-sib-5` arrived during `Undo`!
    let a_reviews_before_np_undo = receiver.accepted_for_workspace(&pw_a).len();
    store
        .move_spruce_conversation(
            auth.as_ref(),
            &dr_all,
            MoveSpruceConversationCommand {
                conversation_id: "conv-siblings-join".into(),
                destination: MoveDestination::NotAPatient,
                undo_window_seconds: 10,
                command_key: "cmd-move-not-patient".into(),
            },
            Some(&receiver),
            t_sib + 50,
        )
        .await
        .unwrap();
    assert!(
        store
            .patient_linked_spruce_messages(&practice, &pw_a)
            .await
            .unwrap()
            .is_empty()
    );
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                13,
                "msg-sib-5",
                "conv-siblings-join",
                Some("+14155550450"),
                Some("Alpha Thread"),
                "Arrived during NotAPatient Undo window.",
                vec![],
            ),
            t_sib + 51,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-sib-5",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::NoMatchFound {
                    evidence_summary: "Mid-undo arrival.".into(),
                },
                rationale: "Patient message.".into(),
            },
            Some(&receiver),
            t_sib + 52,
        )
        .await
        .unwrap();
    assert_eq!(
        receiver.accepted_for_workspace(&pw_a).len(),
        a_reviews_before_np_undo
    );

    store
        .undo_spruce_conversation_move(
            auth.as_ref(),
            &dr_all,
            "conv-siblings-join",
            "cmd-undo-not-patient",
            Some(&receiver),
            t_sib + 53,
        )
        .await
        .unwrap();
    assert_eq!(
        store
            .patient_linked_spruce_messages(&practice, &pw_a)
            .await
            .unwrap()
            .len(),
        4,
        "Undo must restore every message including mid-Undo arrival back onto pw_a"
    );
    assert_eq!(
        receiver.accepted_for_workspace(&pw_a).len(),
        a_reviews_before_np_undo + 1,
        "Undo after mid-Undo arrival must trigger one review on origin patient pw_a"
    );

    // Now test real `Move` settlement (`A -> B` after `Undo` expires) + per-patient failure isolation (`Rule 8`):
    // Also create a second conversation on `pw_c` moving `C -> B` so when `pw_c`'s review receiver fails,
    // `conv-siblings-join` (`A -> B`) STILL settles cleanly without being blocked!
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                14,
                "msg-c-1",
                "conv-c-move",
                Some("+14155550499"),
                Some("Charlie Thread"),
                "Message originally linked to Charlie.",
                vec![],
            ),
            t_sib + 60,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-c-1",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::NoMatchFound {
                    evidence_summary: "Wait.".into(),
                },
                rationale: "Patient message.".into(),
            },
            None,
            t_sib + 61,
        )
        .await
        .unwrap();
    store
        .confirm_spruce_message_link(
            auth.as_ref(),
            &dr_all,
            "msg-c-1",
            &pw_c,
            SenderConfirmationPolicy::ConversationOnly,
            "cmd-link-c",
            Some(&receiver),
            t_sib + 62,
        )
        .await
        .unwrap();

    store
        .move_spruce_conversation(
            auth.as_ref(),
            &dr_all,
            MoveSpruceConversationCommand {
                conversation_id: "conv-siblings-join".into(),
                destination: MoveDestination::AnotherPatient {
                    workspace: pw_b.clone(),
                },
                undo_window_seconds: 10,
                command_key: "cmd-final-move-a-to-b".into(),
            },
            Some(&receiver),
            t_sib + 70,
        )
        .await
        .unwrap();
    store
        .move_spruce_conversation(
            auth.as_ref(),
            &dr_all,
            MoveSpruceConversationCommand {
                conversation_id: "conv-c-move".into(),
                destination: MoveDestination::NotAPatient,
                undo_window_seconds: 10,
                command_key: "cmd-final-move-c-to-np".into(),
            },
            Some(&receiver),
            t_sib + 70,
        )
        .await
        .unwrap();

    // Inject failure on `pw_c` only, then settle expired moves at `t_sib + 85`:
    let a_reviews_before = receiver.accepted_for_workspace(&pw_a).len();
    let b_reviews_before = receiver.accepted_for_workspace(&pw_b).len();
    let c_reviews_before = receiver.accepted_for_workspace(&pw_c).len();
    receiver.fail_workspace(&pw_c);

    let settle_report = store
        .settle_expired_spruce_moves(&practice, Some(&receiver), t_sib + 85)
        .await
        .unwrap();
    assert_eq!(
        settle_report.succeeded.len(),
        1,
        "A -> B move settlement must succeed even when pw_c fails"
    );
    assert_eq!(settle_report.failures.len(), 1);
    assert_eq!(
        receiver.accepted_for_workspace(&pw_a).len(),
        a_reviews_before + 1,
        "Rule 8: settling A -> B starts exactly 1 review for origin patient A"
    );
    assert_eq!(
        receiver.accepted_for_workspace(&pw_b).len(),
        b_reviews_before + 1,
        "Rule 8: settling A -> B starts exactly 1 review for destination patient B"
    );

    // Clear `pw_c` failure and retry `dispatch_pending_spruce_reviews` -> `pw_c` review dispatches once!
    receiver.clear_failures();
    let retry_report = store
        .dispatch_pending_spruce_reviews(&practice, &receiver, t_sib + 86)
        .await
        .unwrap();
    assert!(!retry_report.succeeded.is_empty());
    assert_eq!(
        receiver.accepted_for_workspace(&pw_c).len(),
        c_reviews_before + 1
    );

    // Verify permanent `entered_in_error` records on `pw_a` (`4` ever-linked messages from `conv-siblings-join`):
    let entered_in_error_a = store
        .entered_in_error_spruce_messages(&practice, &pw_a)
        .await
        .unwrap();
    assert_eq!(entered_in_error_a.len(), 4);

    // --- 7. `patient_review_inputs_with_spruce` Excludes Every Unlinked / Moved-Away Message ---
    let inputs_a = store
        .patient_review_inputs_with_spruce(&practice, &pw_a, auth.as_ref())
        .await
        .unwrap();
    assert!(
        inputs_a.linked_spruce_messages.is_empty(),
        "moved-away and left-unlinked messages must never appear in pw_a's review inputs"
    );
    let inputs_b = store
        .patient_review_inputs_with_spruce(&practice, &pw_b, auth.as_ref())
        .await
        .unwrap();
    let b_msg_ids: Vec<&str> = inputs_b
        .linked_spruce_messages
        .iter()
        .map(|m| m.message_id.as_str())
        .collect();
    assert_eq!(
        b_msg_ids,
        vec!["msg-sib-2", "msg-sib-3", "msg-sib-4", "msg-sib-5"],
        "pw_b review inputs must include only messages currently Linked to pw_b"
    );

    // --- 8. Disconnect Channel (`Rule 11`): Deletes `!ever_linked` Messages, Keeps `ever_linked` & `entered_in_error` ---
    let deleted = store
        .disconnect_spruce_channel(auth.as_ref(), &dr_all, t_sib + 100)
        .await
        .unwrap();
    assert!(
        deleted >= 5,
        "all never-linked messages must be deleted on disconnect"
    );

    for never_linked_id in [
        "msg-backlog-3",
        "msg-unreadable-6",
        "msg-15min-7",
        "msg-unreadable-8",
        "msg-sib-1",
    ] {
        assert!(
            store
                .spruce_message(&practice, never_linked_id)
                .await
                .unwrap()
                .is_none(),
            "Rule 11: never-linked message `{never_linked_id}` must be deleted when channel disconnects"
        );
    }
    for ever_linked_id in [
        "msg-sib-2",
        "msg-sib-3",
        "msg-sib-4",
        "msg-sib-5",
        "msg-c-1",
    ] {
        assert!(
            store
                .spruce_message(&practice, ever_linked_id)
                .await
                .unwrap()
                .is_some(),
            "Rule 11: ever-linked message `{ever_linked_id}` (including moved to NotAPatient) must be retained under record policy"
        );
    }
    assert_eq!(
        store
            .entered_in_error_spruce_messages(&practice, &pw_a)
            .await
            .unwrap()
            .len(),
        4,
        "entered_in_error audit records must remain after channel disconnect"
    );
    assert_eq!(
        store
            .today_spruce_summary_for_physician(auth.as_ref(), &dr_all)
            .await
            .unwrap()
            .channel_banner,
        TodayChannelBanner::Quiet
    );
}

#[tokio::test]
async fn live_openai_spruce_matching_smoke_when_authorized() {
    let Some(url) = database_url() else {
        return;
    };
    let allow = std::env::var("CODA_ALLOW_MODEL_CALLS").ok();
    let key = std::env::var("OPENAI_API_KEY").ok();
    let Ok(api_key) = open_ai_api_key(allow.as_deref(), key.as_deref()) else {
        return;
    };

    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let model = std::env::var("CODA_BACKGROUND_MODEL").unwrap_or_else(|_| "gpt-5.4-mini".into());
    let roles = ModelRoles {
        chat: ModelBinding {
            model: model.clone(),
            effort: "low".into(),
            context_window_tokens: 128_000,
            max_input_tokens: 100_000,
            adapter: "openai-responses".into(),
        },
        background: ModelBinding {
            model,
            effort: "low".into(),
            context_window_tokens: 128_000,
            max_input_tokens: 100_000,
            adapter: "openai-responses".into(),
        },
    };

    let now_ts: i64 = 1_763_000_000;
    let practice = format!("practice-live-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let dr = invite_physician(&auth, &practice, "DrLive", now_ts as u64).await;
    let pw = format!("pw-live-{}", Uuid::new_v4().simple());
    seed_patient_with_chart(
        &store,
        &auth,
        &practice,
        &pw,
        "ehr-live-1",
        "Nadia Patel",
        "1988-09-15",
        SECRET_CHART_SENTINEL,
        now_ts,
    )
    .await;
    auth.set_membership(&dr, &pw, Membership::Allowed)
        .await
        .unwrap();

    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, now_ts)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-live", 300, now_ts)
        .await
        .unwrap();
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                1,
                "msg-live-1",
                "conv-live-1",
                Some("+14155550888"),
                Some("Nadia Patel"),
                "Hi Dr. Live, this is Nadia Patel (DOB 1988-09-15) texting from a new phone about scheduling my follow-up.",
                vec![OutsideParticipant {
                    display_name: Some("Nadia Patel".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550888").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 1,
        )
        .await
        .unwrap();

    let mut provider = ResponsesProvider::new(
        harness::responses::ENDPOINT.to_owned(),
        api_key,
        Arc::new(store.clone()),
    )
    .unwrap();
    let completed = run_spruce_message_matching(
        &store,
        auth.as_ref(),
        &practice,
        "msg-live-1",
        &roles,
        &skills,
        &mut provider,
        None,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(completed.run.status, MatchingRunStatus::Completed);
    assert_eq!(
        completed.routing_outcome,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: Some(pw),
        }
    );
}

#[tokio::test]
async fn wrong_patient_undo_boundaries_access_redaction_multi_participant_recovery_no_match_and_event_replay()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let now_ts: i64 = 1_763_100_000;
    let practice = format!("practice-reg-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();

    let dr_full = invite_physician(&auth, &practice, "DrFull", now_ts as u64).await;
    let dr_limited = invite_physician(&auth, &practice, "DrLimited", now_ts as u64).await;

    let pw_a = format!("pw-reg-a-{}", Uuid::new_v4().simple());
    let pw_b = format!("pw-reg-b-{}", Uuid::new_v4().simple());
    let pw_c = format!("pw-reg-c-{}", Uuid::new_v4().simple());

    for (ws, ehr_id, name) in [
        (&pw_a, "ehr-reg-a", "Patient Alpha"),
        (&pw_b, "ehr-reg-b", "Patient Bravo"),
        (&pw_c, "ehr-reg-c", "Patient Charlie"),
    ] {
        seed_patient_with_chart(
            &store,
            &auth,
            &practice,
            ws,
            ehr_id,
            name,
            "1982-04-10",
            "Clinical chart content.",
            now_ts,
        )
        .await;
        auth.set_membership(&dr_full, ws, Membership::Allowed)
            .await
            .unwrap();
    }
    // `dr_limited` has access ONLY to `pw_b`, NOT `pw_a` or `pw_c` (`ClinicDemographic` on `pw_a`):
    auth.set_membership(&dr_limited, &pw_a, Membership::ClinicDemographic)
        .await
        .unwrap();
    auth.set_membership(&dr_limited, &pw_b, Membership::Allowed)
        .await
        .unwrap();

    store
        .connect_spruce_channel(auth.as_ref(), &dr_full, 0, None, now_ts)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-reg", 7_200, now_ts)
        .await
        .unwrap();
    let receiver = RecordingReviewReceiver::default();
    let mut pos_seq: u64 = 1;
    let mut next_pos = || {
        let p = pos_seq;
        pos_seq += 1;
        p
    };

    // --- 1. Multi-line / tab failure reasons succeed; empty and NUL reasons fail (`InvalidReasonText`) ---
    store
        .record_spruce_channel_failure(
            &practice,
            &lease.lease_token,
            ChannelStopCause::Transient {
                reason: "Spruce 502 Bad Gateway\r\n\tUpstream trace: req-991".into(),
            },
            now_ts + 1,
        )
        .await
        .unwrap();
    assert!(matches!(
        store
            .record_spruce_channel_failure(
                &practice,
                &lease.lease_token,
                ChannelStopCause::Transient {
                    reason: " \r\n\t ".into(),
                },
                now_ts + 2,
            )
            .await,
        Err(SpruceMatchingError::InvalidReasonText { field: "reason" })
    ));
    assert!(matches!(
        store
            .record_spruce_channel_failure(
                &practice,
                &lease.lease_token,
                ChannelStopCause::Transient {
                    reason: "bad\0nul".into(),
                },
                now_ts + 2,
            )
            .await,
        Err(SpruceMatchingError::InvalidReasonText { field: "reason" })
    ));
    store
        .recover_spruce_channel_reading(&practice, &lease.lease_token, now_ts + 3)
        .await
        .unwrap();

    let unreadable_multiline_pos = next_pos();
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Unreadable(UnreadableSpruceItem {
                message_id: "msg-multiline-err".into(),
                conversation_id: "unreadable-env-multiline".into(),
                channel_position: unreadable_multiline_pos,
                content_version: "ver-unreadable-init".into(),
                failure_reason: "Fax TIFF decode failed:\n\tpage 2 corrupt strip offset".into(),
                app_url: Some("https://app.sprucehealth.com/items/multiline".into()),
                outside_participants: vec![],
            }),
            now_ts + 4,
        )
        .await
        .unwrap();
    assert!(matches!(
        store
            .ingest_spruce_item(
                &practice,
                &lease.lease_token,
                &IncomingSpruceItem::Unreadable(UnreadableSpruceItem {
                    message_id: "msg-nul-err".into(),
                    conversation_id: "unreadable-env-nul".into(),
                    channel_position: next_pos(),
                    content_version: "ver-nul".into(),
                    failure_reason: "contains\0nul".into(),
                    app_url: None,
                    outside_participants: vec![],
                }),
                now_ts + 4,
            )
            .await,
        Err(SpruceMatchingError::InvalidReasonText {
            field: "failure_reason"
        })
    ));

    // --- 2. `CantRead -> Readable` (`CodaReadsItNow`) migrates synthetic conversation onto real linked conversation (`Rule 3`) ---
    // First link `"conv-real-linked-a"` onto `pw_a`:
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                next_pos(),
                "msg-real-anchor",
                "conv-real-linked-a",
                Some("+14155550601"),
                Some("Patient Alpha"),
                "Initial message in real conversation.",
                vec![OutsideParticipant {
                    display_name: Some("Patient Alpha".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550601").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 5,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-real-anchor",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: pw_a.clone(),
                    evidence_summary: "Alpha.".into(),
                },
                rationale: "Patient mail.".into(),
            },
            None,
            now_ts + 6,
        )
        .await
        .unwrap();
    store
        .confirm_spruce_message_link(
            auth.as_ref(),
            &dr_full,
            "msg-real-anchor",
            &pw_a,
            SenderConfirmationPolicy::ConversationOnly,
            "cmd-confirm-real-anchor",
            Some(&receiver),
            now_ts + 7,
        )
        .await
        .unwrap();

    // Invalid `content_version` on recovery must fail with `InvalidIdentifier`:
    assert!(matches!(
        store
            .ingest_spruce_item(
                &practice,
                &lease.lease_token,
                &IncomingSpruceItem::Readable(ReadableSpruceItem {
                    message_id: "msg-multiline-err".into(),
                    conversation_id: "conv-real-linked-a".into(),
                    channel_position: unreadable_multiline_pos,
                    content_version: "bad\0version".into(),
                    sender_address: Some(SenderAddress::parse("+14155550601").unwrap()),
                    author_name: Some("Patient Alpha".into()),
                    body_text: "Decoded fax body.".into(),
                    attachments: vec![],
                    app_url: None,
                    created_at_spruce: None,
                    outside_participants: vec![],
                }),
                now_ts + 8,
            )
            .await,
        Err(SpruceMatchingError::InvalidIdentifier {
            field: "content_version"
        })
    ));

    // Valid recovery migrates `msg-multiline-err` from `"unreadable-env-multiline"` to `"conv-real-linked-a"`,
    // cleans up the orphaned synthetic conversation row, updates `content_version`, and joins `pw_a` on classification!
    let recovered = store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "msg-multiline-err".into(),
                conversation_id: "conv-real-linked-a".into(),
                channel_position: unreadable_multiline_pos,
                content_version: "ver-recovered-clean".into(),
                sender_address: Some(SenderAddress::parse("+14155550601").unwrap()),
                author_name: Some("Patient Alpha".into()),
                body_text: "Decoded fax body for Alpha.".into(),
                attachments: vec![],
                app_url: Some(
                    "https://app.sprucehealth.com/conversations/conv-real-linked-a".into(),
                ),
                created_at_spruce: None,
                outside_participants: vec![OutsideParticipant {
                    display_name: Some("Patient Alpha".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550601").unwrap()),
                    saved_contact_id: None,
                }],
            }),
            now_ts + 9,
        )
        .await
        .unwrap();
    assert!(matches!(
        recovered,
        SpruceIngestOutcome::RecoveredUnreadable { .. }
    ));
    assert!(
        store
            .spruce_conversation(&practice, "unreadable-env-multiline")
            .await
            .unwrap()
            .is_none(),
        "orphaned synthetic fallback conversation row should be removed upon recovery"
    );
    let saved_recovered = store
        .spruce_message(&practice, "msg-multiline-err")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_recovered.conversation_id, "conv-real-linked-a");
    assert_eq!(saved_recovered.content_version, "ver-recovered-clean");

    let recovered_route = store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-multiline-err",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::NoMatchFound {
                    evidence_summary: "Follows linked conversation.".into(),
                },
                rationale: "Patient fax.".into(),
            },
            Some(&receiver),
            now_ts + 10,
        )
        .await
        .unwrap();
    assert!(matches!(
        recovered_route,
        MessageRoutingOutcome::Linked {
            ref workspace,
            origin: LinkOrigin::JoinedExistingConversationLink,
            ..
        } if workspace == &pw_a
    ));

    // --- 3. Multi-outside-participant conversation & participant confirmed-sender conflict withhold `Rule 4` auto-link ---
    let confirmed_a_phone = SenderAddress::parse("+14155550611").unwrap();
    let confirmed_b_phone = SenderAddress::parse("+14155550622").unwrap();
    store
        .record_spruce_confirmed_sender(
            &practice,
            &confirmed_a_phone,
            &pw_a,
            &dr_full.physician_id,
            now_ts + 11,
        )
        .await
        .unwrap();
    store
        .record_spruce_confirmed_sender(
            &practice,
            &confirmed_b_phone,
            &pw_b,
            &dr_full.physician_id,
            now_ts + 11,
        )
        .await
        .unwrap();

    // 3a. Group thread (`outside_participants.len() == 2` without `saved_contact_id`) from `confirmed_a_phone`
    // even when model suggests `pw_a` MUST wait for confirmation (`Rule 4: one outside participant`)!
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                next_pos(),
                "msg-group-2p",
                "conv-group-2p",
                Some("+14155550611"),
                Some("Patient Alpha"),
                "Group text with caregiver on thread.",
                vec![
                    OutsideParticipant {
                        display_name: Some("Patient Alpha".into()),
                        endpoint_channel: Some("sms".into()),
                        endpoint_address: Some(confirmed_a_phone.clone()),
                        saved_contact_id: None,
                    },
                    OutsideParticipant {
                        display_name: Some("Second Outside Person".into()),
                        endpoint_channel: Some("sms".into()),
                        endpoint_address: Some(SenderAddress::parse("+14155550699").unwrap()),
                        saved_contact_id: None,
                    },
                ],
            ),
            now_ts + 12,
        )
        .await
        .unwrap();
    let group_outcome = store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-group-2p",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: pw_a.clone(),
                    evidence_summary: "Alpha is one participant in 2-person group thread.".into(),
                },
                rationale: "Group thread.".into(),
            },
            Some(&receiver),
            now_ts + 13,
        )
        .await
        .unwrap();
    assert_eq!(
        group_outcome,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: Some(pw_a.clone()),
        },
        "Rule 4: conversation with >1 outside participant must never auto-link"
    );

    // 3b. Single outside participant confirmed for `pw_b` while `msg.sender_address` is confirmed for `pw_a`
    // unions `{pw_a, pw_b}` and withholds auto-link!
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                next_pos(),
                "msg-endpoint-mismatch",
                "conv-endpoint-mismatch",
                Some("+14155550611"),
                Some("Mixed Sender"),
                "Forwarded message.",
                vec![OutsideParticipant {
                    display_name: Some("Bravo Endpoint".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(confirmed_b_phone.clone()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 14,
        )
        .await
        .unwrap();
    let mismatch_outcome = store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-endpoint-mismatch",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: pw_a.clone(),
                    evidence_summary: "Conflicting endpoints.".into(),
                },
                rationale: "Conflicting endpoints.".into(),
            },
            Some(&receiver),
            now_ts + 15,
        )
        .await
        .unwrap();
    assert!(matches!(
        mismatch_outcome,
        MessageRoutingOutcome::Waiting { .. }
    ));

    // --- 4. Confirmed sender writing about an unregistered relative (`NoMatchFound`) withholds `Rule 4` auto-link
    //        AND `persist_spruce_matching_run_event` enforces replay idempotency, collision check, sequence order, and currency! ---
    let a_reviews_before_rel = receiver.accepted_for_workspace(&pw_a).len();
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                next_pos(),
                "msg-unregistered-nephew",
                "conv-unregistered-nephew",
                Some("+14155550611"),
                Some("Patient Alpha"),
                "Hi doctor, my visiting 6-year-old nephew Leo has a fever — do you see pediatric walk-ins?",
                vec![OutsideParticipant {
                    display_name: Some("Patient Alpha".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(confirmed_a_phone.clone()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 16,
        )
        .await
        .unwrap();

    struct ReplayProbeProvider {
        store: Store,
        practice: String,
        message_id: String,
        turns: VecDeque<engine::Turn>,
        probed_tool_replay: bool,
    }

    impl engine::recipe::Provider for ReplayProbeProvider {
        async fn exchange(
            &mut self,
            _request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            let runs = self
                .store
                .spruce_matching_runs_for_message(&self.practice, &self.message_id)
                .await
                .unwrap();
            assert_eq!(runs.len(), 1);
            let current_run = &runs[0];
            assert_eq!(current_run.status, MatchingRunStatus::Running);

            // Wrong attempt ID is rejected:
            let mut wrong_attempt_ev = current_run.events[0].clone();
            wrong_attempt_ev.attempt_id = "foreign-attempt-id".into();
            assert!(
                self.store
                    .persist_spruce_matching_run_event(
                        &self.practice,
                        &current_run.run_id,
                        &current_run.attempt_id,
                        wrong_attempt_ev,
                    )
                    .await
                    .is_err()
            );

            // Sequence gap (`last + 5`) is rejected:
            let mut gap_ev = current_run.events.last().unwrap().clone();
            gap_ev.sequence += 5;
            assert!(
                self.store
                    .persist_spruce_matching_run_event(
                        &self.practice,
                        &current_run.run_id,
                        &current_run.attempt_id,
                        gap_ev,
                    )
                    .await
                    .is_err()
            );

            // Colliding ordinal at existing sequence is rejected:
            let mut colliding_ev = current_run.events[0].clone();
            colliding_ev.ordinal += 999;
            assert!(
                self.store
                    .persist_spruce_matching_run_event(
                        &self.practice,
                        &current_run.run_id,
                        &current_run.attempt_id,
                        colliding_ev,
                    )
                    .await
                    .is_err()
            );

            // Once `ToolReturned` has been recorded (turn 2+), replay that exact `ToolReturned` event
            // and verify neither `events` nor `tool_discoveries` duplicates!
            if let Some(tool_ev) = current_run
                .events
                .iter()
                .find(|e| matches!(e.kind, engine::EventKind::ToolReturned { .. }))
                .cloned()
            {
                let events_len_before = current_run.events.len();
                let discoveries_len_before = current_run.tool_discoveries.len();
                self.store
                    .persist_spruce_matching_run_event(
                        &self.practice,
                        &current_run.run_id,
                        &current_run.attempt_id,
                        tool_ev,
                    )
                    .await
                    .expect("idempotent ToolReturned replay must succeed");
                let after = &self
                    .store
                    .spruce_matching_runs_for_message(&self.practice, &self.message_id)
                    .await
                    .unwrap()[0];
                assert_eq!(after.events.len(), events_len_before);
                assert_eq!(after.tool_discoveries.len(), discoveries_len_before);
                self.probed_tool_replay = true;
            }

            Ok(self.turns.pop_front().expect("probe provider turn"))
        }
    }

    let mut probe_provider = ReplayProbeProvider {
        store: store.clone(),
        practice: practice.clone(),
        message_id: "msg-unregistered-nephew".into(),
        turns: VecDeque::from(vec![
            call_turn("lookup-leo", PATIENT_LOOKUP_TOOL, json!({ "query": "Leo" })).unwrap(),
            call_turn(
                "submit-nephew",
                SUBMIT_CLASSIFICATION_TOOL,
                json!({
                    "classification": "patient_mail",
                    "urgency": "ordinary",
                    "match_outcome": "no_match_found",
                    "evidence_summary": "Sender +14155550611 belongs to Patient Alpha, while the message asks about an unregistered nephew Leo not on the practice roster.",
                    "rationale": "Caregiver message about an unregistered child; keep waiting for physician review rather than linking to Patient Alpha."
                }),
            )
            .unwrap(),
            done_turn("Held unmatched inquiry about unregistered relative Leo.").unwrap(),
        ]),
        probed_tool_replay: false,
    };

    let completed_nephew = run_spruce_message_matching(
        &store,
        auth.as_ref(),
        &practice,
        "msg-unregistered-nephew",
        &roles,
        &skills,
        &mut probe_provider,
        Some(&receiver),
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(probe_provider.probed_tool_replay);
    assert_eq!(
        completed_nephew.routing_outcome,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: None,
        },
        "confirmed sender writing about an unregistered relative (NoMatchFound) must NOT auto-link to the sender's workspace"
    );
    assert_eq!(
        receiver.accepted_for_workspace(&pw_a).len(),
        a_reviews_before_rel,
        "zero reviews may fire on pw_a when NoMatchFound withholds auto-link"
    );

    // Post-completion currency fence: appending a new non-surviving event after `run` completed is rejected!
    let mut stale_ev = completed_nephew.run.events[0].clone();
    stale_ev.sequence = completed_nephew.run.events.last().unwrap().sequence + 1;
    stale_ev.ordinal = completed_nephew.run.events.last().unwrap().ordinal + 1;
    assert!(
        store
            .persist_spruce_matching_run_event(
                &practice,
                &completed_nephew.run.run_id,
                &completed_nephew.run.attempt_id,
                stale_ev,
            )
            .await
            .is_err(),
        "stale non-surviving event after run completion must be refused"
    );

    // --- 5. `Rule 2` Access Redaction on `mark_spruce_message_as_patient` (Live + Replay) & Undispatched Trigger Retry ---
    // 5a. Conversation `conv-group-2p` currently has `active_suggestion == Some(pw_a)`.
    // Add a `Filtered` message `msg-filt-in-suggested` in `conv-group-2p`; `dr_limited` (not allowed on `pw_a`)
    // clicks `"This is a patient"` -> both live and replay MUST redact `suggested_workspace` to `None`!
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                next_pos(),
                "msg-filt-in-suggested",
                "conv-group-2p",
                Some("+14155550699"),
                Some("Second Outside Person"),
                "Follow-up attachment.",
                vec![],
            ),
            now_ts + 20,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-filt-in-suggested",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::NotPatientMail {
                category: NonPatientCategory::NonPatientOutreach,
                rationale: "Filtered initially.".into(),
            },
            None,
            now_ts + 21,
        )
        .await
        .unwrap();

    let limited_wait_live = store
        .mark_spruce_message_as_patient(
            auth.as_ref(),
            &dr_limited,
            "msg-filt-in-suggested",
            "cmd-limited-mark-wait",
            Some(&receiver),
            now_ts + 22,
        )
        .await
        .unwrap();
    assert_eq!(
        limited_wait_live,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: None,
        },
        "Rule 2: mark_spruce_message_as_patient must redact unauthorized suggested_workspace"
    );
    let limited_wait_replay = store
        .mark_spruce_message_as_patient(
            auth.as_ref(),
            &dr_limited,
            "msg-filt-in-suggested",
            "cmd-limited-mark-wait",
            Some(&receiver),
            now_ts + 23,
        )
        .await
        .unwrap();
    assert_eq!(
        limited_wait_replay,
        MessageRoutingOutcome::Waiting {
            suggested_workspace: None,
        },
        "Rule 2: Receipt::Same replay of mark_spruce_message_as_patient must also redact unauthorized suggested_workspace"
    );

    // 5b. Filtered message `msg-filt-autolink-a` in a conversation uniquely tied to `pw_a` via `confirmed_a_phone`:
    // `dr_limited` calls `mark_spruce_message_as_patient` first with `review_receiver = None` (leaving trigger undispatched),
    // then replays with `Some(&receiver)`:
    // - Both live & replay return `MessageRoutingOutcome::LinkedWithoutViewerAccess` (never exposing `pw_a`!)
    // - Replay dispatches the previously undispatched review trigger for `pw_a`!
    let a_reviews_before_limited_link = receiver.accepted_for_workspace(&pw_a).len();
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                next_pos(),
                "msg-filt-autolink-a",
                "conv-filt-autolink-a",
                Some("+14155550611"),
                Some("Patient Alpha"),
                "Short note initially misfiltered.",
                vec![OutsideParticipant {
                    display_name: Some("Patient Alpha".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(confirmed_a_phone.clone()),
                    saved_contact_id: None,
                }],
            ),
            now_ts + 24,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-filt-autolink-a",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::NotPatientMail {
                category: NonPatientCategory::AdministrativeNotice,
                rationale: "Misfiltered.".into(),
            },
            None,
            now_ts + 25,
        )
        .await
        .unwrap();

    let limited_link_live = store
        .mark_spruce_message_as_patient(
            auth.as_ref(),
            &dr_limited,
            "msg-filt-autolink-a",
            "cmd-limited-mark-link",
            None,
            now_ts + 26,
        )
        .await
        .unwrap();
    assert_eq!(
        limited_link_live,
        MessageRoutingOutcome::LinkedWithoutViewerAccess {
            origin: LinkOrigin::AutomaticRule4,
            joined_message_ids: vec!["msg-filt-autolink-a".into()],
        },
        "Rule 2: mark_spruce_message_as_patient must return LinkedWithoutViewerAccess when actor lacks access to linked patient"
    );
    assert_eq!(
        receiver.accepted_for_workspace(&pw_a).len(),
        a_reviews_before_limited_link
    );

    let limited_link_replay = store
        .mark_spruce_message_as_patient(
            auth.as_ref(),
            &dr_limited,
            "msg-filt-autolink-a",
            "cmd-limited-mark-link",
            Some(&receiver),
            now_ts + 27,
        )
        .await
        .unwrap();
    assert_eq!(limited_link_replay, limited_link_live);
    assert_eq!(
        receiver.accepted_for_workspace(&pw_a).len(),
        a_reviews_before_limited_link + 1,
        "Receipt::Same replay on mark_spruce_message_as_patient must retry undispatched review trigger"
    );

    // --- 6. Wrong-Patient `Move` / `Undo` Window Lifecycle Boundaries (`Rules 7 & 8`) ---
    // Seed a clean linked conversation `conv-move-bounds` on `pw_a` with `msg-mb-1`:
    let t_mv = now_ts + 100;
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                next_pos(),
                "msg-mb-1",
                "conv-move-bounds",
                Some("+14155550680"),
                Some("Bounds Thread"),
                "Original message on Alpha.",
                vec![],
            ),
            t_mv,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-mb-1",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::NoMatchFound {
                    evidence_summary: "Bounds.".into(),
                },
                rationale: "Patient mail.".into(),
            },
            None,
            t_mv + 1,
        )
        .await
        .unwrap();
    store
        .confirm_spruce_message_link(
            auth.as_ref(),
            &dr_full,
            "msg-mb-1",
            &pw_a,
            SenderConfirmationPolicy::ConversationOnly,
            "cmd-confirm-mb-1",
            Some(&receiver),
            t_mv + 2,
        )
        .await
        .unwrap();

    // 6a. Move `A -> B` (`t_mv + 10`, `expires_at = t_mv + 20`), receive `msg-mb-2` on `B` during `Undo`,
    // then move back `B -> A` (`returned_to_origin`) -> triggers 1 review on `pw_a` for `msg-mb-2`!
    let a_reviews_before_return = receiver.accepted_for_workspace(&pw_a).len();
    store
        .move_spruce_conversation(
            auth.as_ref(),
            &dr_full,
            MoveSpruceConversationCommand {
                conversation_id: "conv-move-bounds".into(),
                destination: MoveDestination::AnotherPatient {
                    workspace: pw_b.clone(),
                },
                undo_window_seconds: 10,
                command_key: "cmd-mb-a-to-b-1".into(),
            },
            Some(&receiver),
            t_mv + 10,
        )
        .await
        .unwrap();
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                next_pos(),
                "msg-mb-2",
                "conv-move-bounds",
                Some("+14155550680"),
                Some("Bounds Thread"),
                "Arrived on B during Undo before moving back to A.",
                vec![],
            ),
            t_mv + 11,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-mb-2",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::NoMatchFound {
                    evidence_summary: "Mid-undo.".into(),
                },
                rationale: "Patient mail.".into(),
            },
            Some(&receiver),
            t_mv + 12,
        )
        .await
        .unwrap();
    assert_eq!(
        receiver.accepted_for_workspace(&pw_a).len(),
        a_reviews_before_return
    );

    store
        .move_spruce_conversation(
            auth.as_ref(),
            &dr_full,
            MoveSpruceConversationCommand {
                conversation_id: "conv-move-bounds".into(),
                destination: MoveDestination::AnotherPatient {
                    workspace: pw_a.clone(),
                },
                undo_window_seconds: 10,
                command_key: "cmd-mb-return-b-to-a".into(),
            },
            Some(&receiver),
            t_mv + 13,
        )
        .await
        .unwrap();
    assert_eq!(
        receiver.accepted_for_workspace(&pw_a).len(),
        a_reviews_before_return + 1,
        "returning B -> A during Undo after a mid-Undo arrival must trigger one review on pw_a"
    );

    // 6b. Move `A -> B` at `t_mv + 30` (`expires_at = t_mv + 40`, `moved_message_ids = {msg-mb-1, msg-mb-2}`).
    // - Early `act_on_spruce_conversation_move` at `t_mv + 35` (`< expires_at`) MUST fail (`InvalidTransition`).
    // - `msg-mb-never-a` arrives on `B` at `t_mv + 36` during active `Undo`.
    // - Projections show `move_undo` at `t_mv + 38`, and hide `move_undo` at `t_mv + 40` even before sweep!
    // - `undo_spruce_conversation_move` at `t_mv + 40` (`>= expires_at`) MUST fail (`InvalidTransition`).
    // - Move `B -> C` at `t_mv + 41` (`>= expires_at`, BEFORE sweep!) settles Move 1 (`A -> B`) first:
    //   * `entered_in_error` on `pw_a` records ONLY `{msg-mb-1, msg-mb-2}` and NEVER `msg-mb-never-a`!
    //   * Move 1 reviews fire for `pw_a` and `pw_b`!
    //   * New `move_undo` has `from_workspace == pw_b` (not `pw_a`)!
    store
        .move_spruce_conversation(
            auth.as_ref(),
            &dr_full,
            MoveSpruceConversationCommand {
                conversation_id: "conv-move-bounds".into(),
                destination: MoveDestination::AnotherPatient {
                    workspace: pw_b.clone(),
                },
                undo_window_seconds: 10,
                command_key: "cmd-mb-a-to-b-2".into(),
            },
            Some(&receiver),
            t_mv + 30,
        )
        .await
        .unwrap();
    store
        .open_spruce_conversation(auth.as_ref(), &dr_full, "conv-move-bounds", t_mv + 31)
        .await
        .unwrap();

    assert!(matches!(
        store
            .act_on_spruce_conversation_move(
                &practice,
                "conv-move-bounds",
                Some(&receiver),
                t_mv + 35,
            )
            .await,
        Err(SpruceMatchingError::InvalidTransition {
            action: "act_on_move",
            ..
        })
    ));

    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &readable_item(
                next_pos(),
                "msg-mb-never-a",
                "conv-move-bounds",
                Some("+14155550680"),
                Some("Bounds Thread"),
                "Arrived on B while Undo active; was NEVER linked to A.",
                vec![],
            ),
            t_mv + 36,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "msg-mb-never-a",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::NoMatchFound {
                    evidence_summary: "Destination arrival.".into(),
                },
                rationale: "Patient mail.".into(),
            },
            Some(&receiver),
            t_mv + 37,
        )
        .await
        .unwrap();

    assert!(
        store
            .authorized_spruce_conversation_for_physician(
                auth.as_ref(),
                &dr_full,
                "conv-move-bounds",
                t_mv + 38,
            )
            .await
            .unwrap()
            .move_undo
            .is_some()
    );
    assert!(
        store
            .authorized_spruce_conversation_for_physician(
                auth.as_ref(),
                &dr_full,
                "conv-move-bounds",
                t_mv + 40,
            )
            .await
            .unwrap()
            .move_undo
            .is_none(),
        "expired move_undo must not be displayed by authorized_spruce_conversation_for_physician"
    );
    assert!(
        store
            .open_spruce_conversations_for_physician(auth.as_ref(), &dr_full, t_mv + 40)
            .await
            .unwrap()[0]
            .move_undo
            .is_none(),
        "expired move_undo must not be displayed by open_spruce_conversations_for_physician"
    );

    assert!(matches!(
        store
            .undo_spruce_conversation_move(
                auth.as_ref(),
                &dr_full,
                "conv-move-bounds",
                "cmd-expired-undo",
                Some(&receiver),
                t_mv + 40,
            )
            .await,
        Err(SpruceMatchingError::InvalidTransition {
            action: "undo_move",
            ..
        })
    ));

    let a_reviews_pre_chain = receiver.accepted_for_workspace(&pw_a).len();
    let b_reviews_pre_chain = receiver.accepted_for_workspace(&pw_b).len();
    let moved_post_expiry = store
        .move_spruce_conversation(
            auth.as_ref(),
            &dr_full,
            MoveSpruceConversationCommand {
                conversation_id: "conv-move-bounds".into(),
                destination: MoveDestination::AnotherPatient {
                    workspace: pw_c.clone(),
                },
                undo_window_seconds: 10,
                command_key: "cmd-mb-post-expiry-b-to-c".into(),
            },
            Some(&receiver),
            t_mv + 41,
        )
        .await
        .unwrap();
    assert_eq!(
        moved_post_expiry
            .move_undo
            .as_ref()
            .map(|u| u.from_workspace.as_str()),
        Some(pw_b.as_str()),
        "moving B -> C after A -> B Undo expired must settle A -> B first and start new Undo from B"
    );
    assert_eq!(
        receiver.accepted_for_workspace(&pw_a).len(),
        a_reviews_pre_chain + 1
    );
    assert_eq!(
        receiver.accepted_for_workspace(&pw_b).len(),
        b_reviews_pre_chain + 1
    );

    let eie_a: BTreeSet<String> = store
        .entered_in_error_spruce_messages(&practice, &pw_a)
        .await
        .unwrap()
        .into_iter()
        .map(|r| r.message_id)
        .collect();
    assert_eq!(
        eie_a,
        BTreeSet::from(["msg-mb-1".to_owned(), "msg-mb-2".to_owned()]),
        "entered_in_error on pw_a must include ONLY messages that were linked to pw_a, never mid-Undo arrivals on pw_b"
    );
    assert!(!eie_a.contains("msg-mb-never-a"));

    // 6c. Per-patient failure isolation inside a single 2-patient settlement (`B -> C` where `pw_b` fails):
    // `pw_c`'s review STILL dispatches immediately even when `pw_b`'s review fails!
    let c_reviews_pre_settle = receiver.accepted_for_workspace(&pw_c).len();
    receiver.fail_workspace(&pw_b);
    assert!(matches!(
        store
            .act_on_spruce_conversation_move(
                &practice,
                "conv-move-bounds",
                Some(&receiver),
                t_mv + 55,
            )
            .await,
        Err(SpruceMatchingError::ReviewHandoff { .. })
    ));
    assert_eq!(
        receiver.accepted_for_workspace(&pw_c).len(),
        c_reviews_pre_settle + 1,
        "failure of origin patient pw_b review handoff must not block destination patient pw_c review handoff"
    );
    receiver.clear_failures();
}
