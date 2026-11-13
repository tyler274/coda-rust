//! Real PostgreSQL integration tests for practice-owned background jobs and
//! patient source check / review dispatch.
//!
//! Runs automatically whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL` or
//! `PHYSICIAN_DATABASE_URL` is set; panics if neither is set when
//! `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::body::{Body, to_bytes};
use axum::http::{Request as HttpRequest, StatusCode};
use front_door::{FrontDoorStore, PostgresFrontDoorStore, RosterPatient};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
    SourceRevision,
};
use local_product::runtime::dispatch::{
    DispatchEndpointState, DispatchError, PatientSourceChecker, PracticeJobReviewReceiver,
    ReviewReceiver, SetCadenceBody, SourceCheckError, StoredObservationChecker, WakeSummary,
    operator_router,
};
use local_product::runtime::engine::Product;
use local_product::runtime::jobs::{
    Assignment, AssignmentError, PRACTICE_OWNER, PracticeAccessError, PracticeEffectAttribution,
    PracticeJobCompletion, PracticeJobError, PracticeJobHost, PracticeJobKind, PracticeScope,
    is_physician_only_tool,
};
use local_product::runtime::{
    Accepted, AttemptStatus, Authority, Input, PublicationTransaction, Resumed, Scope, Stopped,
    Store, WorkStatus, async_trait, engine,
};
use local_product::{ModelBinding, ModelRoles, sha256_hex};
use patient_work::{
    Cadence, CheckedSources, DEFAULT_CHECK_EVERY_SECONDS, DEFAULT_REVIEW_EVERY_SECONDS,
    OperatorCadenceView, ReviewReason, ReviewRequest,
};
use physician_auth::{AuthStore, PostgresAuthStore, PracticeClock};
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

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

fn chart_record(
    workspace: &str,
    candidate_id: &str,
    revision: &str,
    date: &str,
    content: &str,
) -> SourceRevision {
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "clinic-ehr".to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: "DiagnosticReport".to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".to_owned(),
            value: date.to_owned(),
        }],
        content: content.to_owned(),
        sha256: sha256_hex(content.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "Summary".to_owned(),
            text: content.to_owned(),
        }],
        provenance: json!({
            "system": "clinic-ehr",
            "native_id": candidate_id,
        }),
        envelope: None,
    }
}

fn source_patient(id: &str, name: &str) -> SourcePatient {
    SourcePatient {
        source_patient_id: id.to_owned(),
        display_name: name.to_owned(),
        birth_date: Some("1980-04-12".to_owned()),
        sex_or_gender: Some("female".to_owned()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-mrn".to_owned(),
            value: format!("MRN-{id}"),
        }],
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

struct ControlAllowed;

#[async_trait]
impl Authority for ControlAllowed {
    async fn check(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[tokio::test]
async fn practice_job_runs_without_physician_session_reads_only_practice_chart_and_saves_result() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice_a = format!("practice-a-{}", Uuid::new_v4().simple());
    let practice_b = format!("practice-b-{}", Uuid::new_v4().simple());
    let pw_a = format!("pw-a-{}", Uuid::new_v4().simple());
    let pw_b = format!("pw-b-{}", Uuid::new_v4().simple());

    auth.ensure_practice(&practice_a, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.ensure_practice(&practice_b, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice_a, &pw_a).await.unwrap();
    auth.register_patient(&practice_b, &pw_b).await.unwrap();

    // Confirm zero physician accounts exist in either practice: background runs
    // require no physician identity and no browser/API session.
    assert!(
        auth.practice_accounts(&practice_a)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        auth.practice_accounts(&practice_b)
            .await
            .unwrap()
            .is_empty()
    );

    store
        .import_sources(&[
            chart_record(
                &pw_a,
                "lab-a1c-2026",
                "rev-1",
                "2026-10-01",
                "Hemoglobin A1c 6.8% collected 2026-10-01.",
            ),
            chart_record(
                &pw_b,
                "foreign-secret-note",
                "rev-1",
                "2026-10-02",
                "Confidential chart note belonging to another practice.",
            ),
        ])
        .await
        .unwrap();

    let assignment = Assignment::synthetic_patient_stub(&pw_a, &skills).unwrap();

    // Cross-practice access and unregistered workspaces are refused before a job can enter the store.
    let foreign_scope = PracticeScope {
        practice: practice_a.clone(),
        workspace: pw_b.clone(),
        job_id: "job-foreign".into(),
    };
    assert!(matches!(
        store
            .accept_practice_job(&foreign_scope, &assignment, &roles, &skills, auth.as_ref())
            .await,
        Err(PracticeJobError::Access(PracticeAccessError::ForeignPractice {
            requested_practice,
            owner_practice,
            workspace,
        })) if requested_practice == practice_a && owner_practice == practice_b && workspace == pw_b
    ));

    let unknown_scope = PracticeScope {
        practice: practice_a.clone(),
        workspace: format!("pw-missing-{}", Uuid::new_v4().simple()),
        job_id: "job-missing".into(),
    };
    assert!(matches!(
        store
            .accept_practice_job(&unknown_scope, &assignment, &roles, &skills, auth.as_ref())
            .await,
        Err(PracticeJobError::Access(
            PracticeAccessError::UnregisteredPatient { .. }
        ))
    ));

    let scope = PracticeScope {
        practice: practice_a.clone(),
        workspace: pw_a.clone(),
        job_id: "job-initial-1".into(),
    };
    assert_eq!(
        store
            .accept_practice_job(&scope, &assignment, &roles, &skills, auth.as_ref())
            .await
            .unwrap(),
        Accepted::New
    );

    let claim = store
        .claim_practice_job(&scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("pending practice job is claimable");

    struct SyntheticChartProvider {
        turn_index: usize,
        foreign_workspace: String,
    }

    impl engine::recipe::Provider for SyntheticChartProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn_index += 1;
            match self.turn_index {
                1 => Ok(engine::Turn {
                    items: vec![
                        engine::Item::Call(engine::Invocation {
                            id: "search-own".into(),
                            name: SEARCH_TOOL.into(),
                            arguments: json!({ "query": "Hemoglobin A1c" }).to_string(),
                        }),
                        engine::Item::Call(engine::Invocation {
                            id: "read-foreign".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "workspace": self.foreign_workspace,
                                "references": ["foreign-secret-note"],
                                "max_result_bytes": 16_384,
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({ "turn": 1 }),
                    usage: Default::default(),
                }),
                2 => {
                    let own_search = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "search-own" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("own workspace search result materialized");
                    assert!(
                        own_search.contains("lab-a1c-2026"),
                        "expected own candidate in search output: {own_search}"
                    );

                    let foreign_read = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "read-foreign" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("foreign workspace read refusal materialized");
                    assert!(
                        !foreign_read.contains("Confidential chart note"),
                        "foreign practice chart leaked: {foreign_read}"
                    );
                    assert!(
                        foreign_read.contains("refused")
                            || foreign_read.contains("assigned patient workspace")
                            || foreign_read.contains("unknown field"),
                        "expected refusal for foreign workspace read: {foreign_read}"
                    );

                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-own".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "lab-a1c-2026",
                                    "revision": "rev-1",
                                }],
                                "max_result_bytes": 16_384,
                            })
                            .to_string(),
                        })],
                        receipt: json!({ "turn": 2 }),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let own_read = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "read-own" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("own workspace exact read materialized");
                    assert!(
                        own_read.contains("Hemoglobin A1c 6.8% collected 2026-10-01."),
                        "expected exact chart content: {own_read}"
                    );

                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant(
                            "Initial clinical understanding: Hemoglobin A1c was 6.8% on 2026-10-01 (DiagnosticReport lab-a1c-2026).",
                        )],
                        receipt: json!({ "turn": 3 }),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected provider turn {other}"),
            }
        }
    }

    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: auth.as_ref(),
    };
    let mut provider = SyntheticChartProvider {
        turn_index: 0,
        foreign_workspace: pw_b.clone(),
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("practice background run completes without physician session");

    let saved_job = store
        .practice_job(&scope)
        .await
        .unwrap()
        .expect("saved practice job");
    assert_eq!(saved_job.status, WorkStatus::Completed);
    assert_eq!(saved_job.sources_version, 1);
    let result = saved_job.result.expect("saved practice job result");
    assert_eq!(result.completion, PracticeJobCompletion::Complete);
    assert!(
        result
            .output_text
            .contains("Hemoglobin A1c was 6.8% on 2026-10-01")
    );
    assert_eq!(
        result.attribution,
        PracticeEffectAttribution {
            practice: practice_a,
            workspace: pw_a,
            job_id: "job-initial-1".into(),
            kind: PracticeJobKind::InitialUnderstanding,
        }
    );
}

#[tokio::test]
async fn assignment_skills_are_preloaded_at_start_and_recorded_on_run_record() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-skills-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-skills-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    let assignment = Assignment::new(
        PracticeJobKind::PatientReview,
        "cmd-preload-1",
        "Review recent chart changes for this patient.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();
    assert_eq!(assignment.preloaded_skills.len(), 1);
    let expected_skill = assignment.preloaded_skills[0].clone();

    // Tampered preloaded skill content must be rejected at acceptance.
    let mut forged = assignment.clone();
    forged.key = "cmd-preload-forged".into();
    forged.preloaded_skills[0].content.push_str("\ntampered");
    let forged_scope = PracticeScope {
        practice: practice.clone(),
        workspace: workspace.clone(),
        job_id: "job-forged".into(),
    };
    assert!(matches!(
        store
            .accept_practice_job(&forged_scope, &forged, &roles, &skills, auth.as_ref())
            .await,
        Err(PracticeJobError::Assignment(
            AssignmentError::UnavailableSkill { .. }
        ))
    ));

    let scope = PracticeScope {
        practice: practice.clone(),
        workspace: workspace.clone(),
        job_id: "job-preload-1".into(),
    };
    assert_eq!(
        store
            .accept_practice_job(&scope, &assignment, &roles, &skills, auth.as_ref())
            .await
            .unwrap(),
        Accepted::New
    );

    let before_run = store.practice_job(&scope).await.unwrap().unwrap();
    assert_eq!(before_run.preloaded_skills, vec![expected_skill.clone()]);

    let claim = store
        .claim_practice_job(&scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    struct FirstRequestInspector {
        first_instructions: Option<String>,
    }

    impl engine::recipe::Provider for FirstRequestInspector {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            if self.first_instructions.is_none() {
                self.first_instructions = Some(request.instructions.clone());
            }
            Ok(engine::Turn {
                items: vec![engine::Item::assistant(
                    "No chart records are retained yet for this patient workspace.",
                )],
                receipt: json!({ "turn": 1 }),
                usage: Default::default(),
            })
        }
    }

    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: auth.as_ref(),
    };
    let mut inspector = FirstRequestInspector {
        first_instructions: None,
    };
    host.run(
        &mut inspector,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let instructions = inspector
        .first_instructions
        .expect("first provider request captured");
    let (_, preload_line) = instructions
        .split_once(
            "Preloaded assignment skills (already open in this run; do not reload via skill_view):\n",
        )
        .expect("preloaded skills header present in first request instructions");
    let preload_json_line = preload_line
        .lines()
        .next()
        .expect("preloaded skills JSON line");
    let preload_envelope: Value =
        serde_json::from_str(preload_json_line).expect("preloaded skills JSON parses");
    assert_eq!(preload_envelope["kind"], "preloaded_assignment_skills");
    assert_eq!(preload_envelope["skills"][0]["name"], expected_skill.name);
    assert_eq!(
        preload_envelope["skills"][0]["sha256"],
        expected_skill.sha256
    );
    assert_eq!(
        preload_envelope["skills"][0]["content"],
        expected_skill.content
    );

    let completed = store.practice_job(&scope).await.unwrap().unwrap();
    assert_eq!(completed.status, WorkStatus::Completed);
    assert_eq!(completed.preloaded_skills, vec![expected_skill]);
}

#[tokio::test]
async fn practice_run_refuses_physician_actions_and_attributes_effects_to_practice_and_job() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-access-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-access-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    let scope = PracticeScope {
        practice: practice.clone(),
        workspace: workspace.clone(),
        job_id: "job-access-1".into(),
    };
    let assignment = Assignment::new(
        PracticeJobKind::PatientReview,
        "cmd-access-1",
        "Review patient chart without performing physician actions.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();

    store
        .accept_practice_job(&scope, &assignment, &roles, &skills, auth.as_ref())
        .await
        .unwrap();

    // Every physician-scoped Store write entry point must refuse PRACTICE_OWNER.
    let control_scope = scope.as_control_scope();
    assert_eq!(control_scope.physician, PRACTICE_OWNER);
    let chat_err = store
        .accept_authorized(
            &control_scope,
            &Input {
                key: "forged-chat".into(),
                text: "Act as physician".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            vec![0_u8; 32],
            &skills,
            &local_product::physician_chat::recipe(&roles.chat, "instructions", &skills),
            None,
        )
        .await
        .expect_err("practice owner cannot open physician chat");
    assert!(
        chat_err
            .to_string()
            .contains("practice run cannot act as a physician"),
        "unexpected error: {chat_err}"
    );

    let draft_err = store
        .save_chat_draft(
            &control_scope,
            0,
            "unsent physician draft",
            &[0_u8; 32],
            &mut ControlAllowed,
        )
        .await
        .expect_err("practice owner cannot save physician composer draft");
    assert!(
        draft_err
            .to_string()
            .contains("practice run cannot act as a physician"),
        "unexpected error: {draft_err}"
    );

    let claim = store
        .claim_practice_job(&scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    let mut host = PracticeJobHost {
        store: store.clone(),
        claim: claim.clone(),
        authority: auth.as_ref(),
    };

    // Direct tool execution on PracticeJobHost refuses every physician-only tool.
    for tool_name in [
        "finish_reply_revision",
        "reply_draft",
        "open_reply_draft",
        "save_reply_draft",
        "chat_draft",
        "save_chat_draft",
        "response_feedback",
        "save_feedback",
        "send_patient_message",
        "approve_doctor_plan",
        "invite_patient",
    ] {
        assert!(is_physician_only_tool(tool_name));
        let outcome = host
            .execute(
                &engine::Invocation {
                    id: format!("call-{tool_name}"),
                    name: tool_name.to_owned(),
                    arguments: "{}".to_owned(),
                },
                json!({}),
            )
            .await
            .unwrap();
        match outcome {
            engine::ToolOutcome::Refused(refusal) => {
                assert!(
                    refusal
                        .reason()
                        .contains("practice background job cannot act as a physician"),
                    "unexpected refusal for {tool_name}: {}",
                    refusal.reason()
                );
            }
            other => panic!("expected Refused for physician tool {tool_name}, got {other:?}"),
        }
    }

    // Complete the run via Provider after attempting a physician tool in-band.
    struct AttemptPhysicianToolProvider {
        step: usize,
    }

    impl engine::recipe::Provider for AttemptPhysicianToolProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.step += 1;
            if self.step == 1 {
                return Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "illegal-send".into(),
                        name: "send_patient_message".into(),
                        arguments: json!({ "body": "Hello patient" }).to_string(),
                    })],
                    receipt: json!({ "step": 1 }),
                    usage: Default::default(),
                });
            }
            let refusal_output = request
                .items
                .iter()
                .find_map(|item| match item {
                    engine::Item::MaterializedResult { call_id, output }
                        if call_id == "illegal-send" =>
                    {
                        Some(output.as_str())
                    }
                    _ => None,
                })
                .expect("refusal returned to model");
            assert!(
                refusal_output.contains("\"executed\":false")
                    && (refusal_output.contains("does not exist")
                        || refusal_output.contains("cannot act as a physician")),
                "unexpected output for send_patient_message: {refusal_output}"
            );
            Ok(engine::Turn {
                items: vec![engine::Item::assistant(
                    "Background chart review recorded for practice review.",
                )],
                receipt: json!({ "step": 2 }),
                usage: Default::default(),
            })
        }
    }

    host.run(
        &mut AttemptPhysicianToolProvider { step: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let job = store.practice_job(&scope).await.unwrap().unwrap();
    assert_eq!(job.status, WorkStatus::Completed);
    assert!(!job.attempts[0].events.is_empty());
    let saved = job.result.expect("saved result");
    assert_eq!(
        saved.attribution,
        PracticeEffectAttribution {
            practice,
            workspace,
            job_id: "job-access-1".into(),
            kind: PracticeJobKind::PatientReview,
        }
    );
}

#[tokio::test]
async fn existing_stop_and_resume_controls_hold_and_resume_practice_run_on_background_lane() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-lane-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-lane-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    let scope = PracticeScope {
        practice: practice.clone(),
        workspace: workspace.clone(),
        job_id: "job-visit-prep-1".into(),
    };
    let assignment = Assignment::new(
        PracticeJobKind::VisitPreparation,
        "cmd-visit-prep-1",
        "Prepare pre-visit summary for upcoming appointment.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();

    store
        .accept_practice_job(&scope, &assignment, &roles, &skills, auth.as_ref())
        .await
        .unwrap();

    let accepted = store.practice_job(&scope).await.unwrap().unwrap();
    let binding = store
        .practice_job_binding(&scope, &accepted.binding)
        .await
        .unwrap();
    let probe_request = binding.request(Vec::new());
    assert_eq!(probe_request.model, "background-worker-lane");
    assert_ne!(probe_request.model, roles.chat.model);
    assert_eq!(probe_request.options, json!({ "effort": "high" }));

    let claim1 = store
        .claim_practice_job(&scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("first claim succeeds");
    let first_run_id = store
        .practice_job(&scope)
        .await
        .unwrap()
        .unwrap()
        .current_run()
        .unwrap()
        .id
        .to_owned();

    // Stop the active practice run through the existing Store::stop_authorized control!
    let control_scope = scope.as_control_scope();
    let stopped = store
        .stop_authorized(&control_scope, "stop-key-1", vec![0_u8; 32], &scope.job_id)
        .await
        .unwrap();
    assert_eq!(stopped, Stopped::Held);

    // Stopping withdraws currency from the in-flight attempt immediately.
    let withdrawn = store
        .assert_practice_job_current(&claim1)
        .await
        .expect_err("stopped attempt loses currency");
    assert!(matches!(
        withdrawn,
        engine::ProductError::NotCurrent(engine::NotCurrent::Withdrawn)
    ));

    let held_job = store.practice_job(&scope).await.unwrap().unwrap();
    assert_eq!(held_job.status, WorkStatus::Held);
    assert_eq!(held_job.attempts[0].status, AttemptStatus::Cancelled);
    assert_eq!(
        held_job.current_run().unwrap().outcome,
        Some(engine::runs::RunOutcome::Held {
            reason: engine::runs::HoldReason::Stopped,
        })
    );

    // Resume the held practice run through the existing Store::resume_authorized control
    // (including when invoked through a signed-in physician's control Scope: the
    // returned broker Delivery must normalize to DeliveryKind::PracticeJob and PRACTICE_OWNER).
    let physician_control_scope = Scope {
        practice: practice.clone(),
        physician: "dr-alice".into(),
        workspace: workspace.clone(),
        chat: scope.job_id.clone(),
    };
    let resumed = store
        .resume_authorized(
            &physician_control_scope,
            "resume-key-1",
            vec![0_u8; 32],
            &scope.job_id,
            &first_run_id,
            None,
        )
        .await
        .unwrap();
    let Resumed::Pending { delivery } = resumed else {
        panic!("expected Resumed::Pending, got {resumed:?}");
    };
    assert_eq!(
        delivery.kind,
        local_product::runtime::delivery::DeliveryKind::PracticeJob
    );
    assert_eq!(delivery.scope, control_scope);
    assert_eq!(delivery.input_id, assignment.key);

    // Before a worker claims it, the due sweep sees the resumed wakeup as due.
    let (due_deliveries, _) = store.practice_jobs_dispatch_due(100, None).await.unwrap();
    assert!(due_deliveries.contains(&delivery));

    assert_eq!(
        store
            .delivery_state(&delivery)
            .await
            .expect("resumed practice job delivery state resolves while pending"),
        local_product::runtime::delivery::DeliveryState::Due,
    );

    let pending_job = store.practice_job(&scope).await.unwrap().unwrap();
    assert_eq!(pending_job.status, WorkStatus::Pending);
    let resumed_run_id = pending_job.current_run().unwrap().id.to_owned();
    assert_ne!(resumed_run_id, first_run_id);

    // Claim the resumed run through the worker delivery entrypoint and execute it on the background lane.
    let claim2 = store
        .claim_practice_job_delivery(&delivery, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("resumed practice job delivery is claimable");
    assert_ne!(claim2.attempt, claim1.attempt);
    assert_eq!(
        store.delivery_state(&delivery).await.unwrap(),
        local_product::runtime::delivery::DeliveryState::Claimed,
    );
    let (due_while_claimed, _) = store.practice_jobs_dispatch_due(100, None).await.unwrap();
    assert!(!due_while_claimed.contains(&delivery));

    struct BackgroundLaneVerifier {
        observed_model: Option<String>,
    }

    impl engine::recipe::Provider for BackgroundLaneVerifier {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.observed_model = Some(request.model.clone());
            Ok(engine::Turn {
                items: vec![engine::Item::assistant(
                    "Visit preparation completed after resume on background lane.",
                )],
                receipt: json!({ "resumed": true }),
                usage: Default::default(),
            })
        }
    }

    let mut host = PracticeJobHost {
        store: store.clone(),
        claim: claim2,
        authority: auth.as_ref(),
    };
    let mut verifier = BackgroundLaneVerifier {
        observed_model: None,
    };
    host.run(
        &mut verifier,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    assert_eq!(
        verifier.observed_model.as_deref(),
        Some("background-worker-lane")
    );
    assert_eq!(
        store.delivery_state(&delivery).await.unwrap(),
        local_product::runtime::delivery::DeliveryState::Settled,
    );
    let finished = store.practice_job(&scope).await.unwrap().unwrap();
    assert_eq!(finished.status, WorkStatus::Completed);
    assert_eq!(finished.attempts.len(), 2);
    assert_eq!(finished.attempts[1].status, AttemptStatus::Completed);
    assert!(
        finished
            .result
            .unwrap()
            .output_text
            .contains("Visit preparation completed after resume")
    );
}

#[derive(Default)]
struct RecordingReceiver {
    requests: Mutex<Vec<ReviewRequest>>,
}

#[async_trait]
impl ReviewReceiver for RecordingReceiver {
    async fn submit_review(&self, request: &ReviewRequest) -> Result<(), PracticeJobError> {
        self.requests.lock().unwrap().push(request.clone());
        Ok(())
    }
}

struct TeeReceiver<'a> {
    recorder: &'a RecordingReceiver,
    practice_jobs: PracticeJobReviewReceiver<'a>,
}

#[async_trait]
impl ReviewReceiver for TeeReceiver<'_> {
    async fn submit_review(&self, request: &ReviewRequest) -> Result<(), PracticeJobError> {
        self.practice_jobs.submit_review(request).await?;
        self.recorder.submit_review(request).await
    }
}

#[tokio::test]
async fn wakeup_evaluates_unchanged_changed_and_periodic_reviews_with_daily_weekly_defaults() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-dispatch-rules-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-dispatch-rules-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    let t0 = 1_760_000_000_i64;
    let initial_view = store
        .patient_dispatch_status(auth.as_ref(), &practice, &workspace, t0)
        .await
        .unwrap();
    assert_eq!(
        initial_view.cadence.check_every_seconds(),
        DEFAULT_CHECK_EVERY_SECONDS
    );
    assert_eq!(
        initial_view.cadence.review_every_seconds(),
        DEFAULT_REVIEW_EVERY_SECONDS
    );
    assert_eq!(DEFAULT_CHECK_EVERY_SECONDS, 86_400);
    assert_eq!(DEFAULT_REVIEW_EVERY_SECONDS, 604_800);

    let checker = StoredObservationChecker::new(store.clone());
    let recorder = RecordingReceiver::default();
    let receiver = TeeReceiver {
        recorder: &recorder,
        practice_jobs: PracticeJobReviewReceiver::new(
            store.clone(),
            &roles,
            &skills,
            auth.as_ref(),
        ),
    };

    // 1. Initial wake-up with no chart changes -> check runs, zero reviews started.
    let wake0 = store
        .wake_practice_dispatch(auth.as_ref(), &practice, t0, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake0.checked_patients, vec![workspace.clone()]);
    assert!(wake0.started_reviews.is_empty());

    // 2. Wake-up 12 hours later -> not due yet (default daily check = 86_400s).
    let wake_early = store
        .wake_practice_dispatch(auth.as_ref(), &practice, t0 + 43_200, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake_early.skipped_not_due, vec![workspace.clone()]);
    assert!(wake_early.checked_patients.is_empty());
    assert!(wake_early.started_reviews.is_empty());

    // 3. New lab record arrives before Day 1 wake-up -> exactly one review, reason "new records".
    let pull1 = local_product::runtime::sources::SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: source_patient("pt-1", "Jordan Rivera"),
        pulled_at: Some(t0 + 80_000),
    };
    store
        .record_chart(
            &workspace,
            &[chart_record(
                &workspace,
                "cmp-2026-10",
                "rev-1",
                "2026-10-05",
                "Potassium 4.2 mmol/L",
            )],
            &pull1,
        )
        .await
        .unwrap();

    let day1 = t0 + 86_400;
    let wake1 = store
        .wake_practice_dispatch(auth.as_ref(), &practice, day1, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake1.checked_patients, vec![workspace.clone()]);
    assert_eq!(wake1.started_reviews.len(), 1);
    let rev1 = &wake1.started_reviews[0];
    assert_eq!(rev1.reason, ReviewReason::NewRecords);
    assert_eq!(rev1.reason.as_str(), "new records");
    assert_eq!(rev1.workspace_version, 1);
    assert_eq!(
        rev1.record_versions,
        BTreeMap::from([("cmp-2026-10".to_owned(), "rev-1".to_owned())])
    );

    // Confirm PracticeJobReviewReceiver created the PatientReview job in the run store.
    let review_job = store
        .practice_job(&PracticeScope {
            practice: practice.clone(),
            workspace: workspace.clone(),
            job_id: rev1.command_key.clone(),
        })
        .await
        .unwrap()
        .expect("patient review job created in run store");
    assert_eq!(review_job.kind, PracticeJobKind::PatientReview);

    // 4. Daily checks on Days 2..=7 with unchanged records -> no review until 7 days elapsed.
    for day_offset in 1..7 {
        let ts = day1 + day_offset * 86_400;
        let daily_wake = store
            .wake_practice_dispatch(auth.as_ref(), &practice, ts, &checker, &receiver)
            .await
            .unwrap();
        assert_eq!(daily_wake.checked_patients, vec![workspace.clone()]);
        assert!(
            daily_wake.started_reviews.is_empty(),
            "unexpected review before weekly window at day offset {day_offset}"
        );
    }

    // 5. Day 8 (+604_800s after Day 1 review) with unchanged records -> one periodic review!
    let day8 = day1 + 604_800;
    let wake_periodic = store
        .wake_practice_dispatch(auth.as_ref(), &practice, day8, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake_periodic.checked_patients, vec![workspace.clone()]);
    assert_eq!(wake_periodic.started_reviews.len(), 1);
    let rev_periodic = &wake_periodic.started_reviews[0];
    assert_eq!(rev_periodic.reason, ReviewReason::Periodic);
    assert_eq!(rev_periodic.reason.as_str(), "periodic");
    assert_eq!(rev_periodic.workspace_version, 1);
    assert_eq!(recorder.requests.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn stop_between_source_check_and_review_start_then_replay_creates_one_review_request() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-replay-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-replay-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    store
        .import_sources(&[chart_record(
            &workspace,
            "lipid-panel-1",
            "rev-1",
            "2026-10-08",
            "LDL cholesterol 98 mg/dL",
        )])
        .await
        .unwrap();

    let t0 = 1_760_100_000_i64;
    let checker = StoredObservationChecker::new(store.clone());

    // Phase 1 executes and commits durable check + pending_review; process stops BEFORE Phase 2!
    let phase1 = store
        .check_due_patients(auth.as_ref(), &practice, t0, &checker)
        .await
        .unwrap();
    assert_eq!(phase1.checked_patients, vec![workspace.clone()]);
    assert!(
        store
            .practice_jobs_due(&practice, 10)
            .await
            .unwrap()
            .is_empty(),
        "no review job should exist before Phase 2 starts"
    );
    drop(checker);
    drop(store);

    // Reconnect on a fresh Store instance and replay wake-up twice.
    let reconnected = Store::connect(&url).await.unwrap();
    let checker2 = StoredObservationChecker::new(reconnected.clone());
    let recorder = RecordingReceiver::default();
    let receiver = TeeReceiver {
        recorder: &recorder,
        practice_jobs: PracticeJobReviewReceiver::new(
            reconnected.clone(),
            &roles,
            &skills,
            auth.as_ref(),
        ),
    };

    let replay1 = reconnected
        .wake_practice_dispatch(auth.as_ref(), &practice, t0 + 30, &checker2, &receiver)
        .await
        .unwrap();
    assert_eq!(replay1.started_reviews.len(), 1);
    assert_eq!(replay1.started_reviews[0].reason, ReviewReason::NewRecords);

    let replay2 = reconnected
        .wake_practice_dispatch(auth.as_ref(), &practice, t0 + 60, &checker2, &receiver)
        .await
        .unwrap();
    assert!(replay2.started_reviews.is_empty());

    assert_eq!(
        recorder.requests.lock().unwrap().len(),
        1,
        "replay after interruption must emit exactly one review request"
    );
    let due_jobs = reconnected.practice_jobs_due(&practice, 10).await.unwrap();
    assert_eq!(
        due_jobs.len(),
        1,
        "exactly one PatientReview job exists in the run store"
    );
}

struct SelectiveFailureChecker {
    failing_workspace: String,
    changed_workspace: String,
}

#[async_trait]
impl PatientSourceChecker for SelectiveFailureChecker {
    async fn check_patient_sources(
        &self,
        _practice_id: &str,
        workspace_id: &str,
        _now: i64,
    ) -> Result<CheckedSources, SourceCheckError> {
        if workspace_id == self.failing_workspace {
            return Err(SourceCheckError::Upstream(std::io::Error::other(
                "source adapter upstream 503",
            )));
        }
        if workspace_id == self.changed_workspace {
            return Ok(CheckedSources::changed(
                2,
                BTreeMap::from([("note-1".to_owned(), "rev-2".to_owned())]),
            ));
        }
        Ok(CheckedSources::unchanged(0, BTreeMap::new()))
    }
}

#[tokio::test]
async fn operator_endpoints_read_and_update_cadence_and_failed_check_isolates_per_patient() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;

    let practice = format!("practice-op-{}", Uuid::new_v4().simple());
    let pw_failing = format!("pw-a-fail-{}", Uuid::new_v4().simple());
    let pw_healthy = format!("pw-b-ok-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &pw_failing).await.unwrap();
    auth.register_patient(&practice, &pw_healthy).await.unwrap();

    let recorder = Arc::new(RecordingReceiver::default());
    let app = operator_router(DispatchEndpointState {
        store: store.clone(),
        auth: auth.clone(),
        checker: Arc::new(SelectiveFailureChecker {
            failing_workspace: pw_failing.clone(),
            changed_workspace: pw_healthy.clone(),
        }),
        receiver: recorder.clone(),
    });

    let t0 = 1_760_200_000_i64;

    // 1. Operator GET reads default cadence and initial due timestamp.
    let get_resp = app
        .clone()
        .oneshot(
            HttpRequest::builder()
                .method("GET")
                .uri(format!(
                    "/operator/practices/{practice}/patients/{pw_healthy}/cadence?now={t0}"
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_resp.status(), StatusCode::OK);
    let initial: OperatorCadenceView =
        serde_json::from_slice(&to_bytes(get_resp.into_body(), 64 * 1024).await.unwrap()).unwrap();
    assert_eq!(initial.cadence, Cadence::default());
    assert_eq!(initial.next_due_at, t0);
    assert_eq!(initial.last_attempt_at, None);
    assert_eq!(initial.last_success_at, None);

    // 2. Invalid cadence (zero check interval) is rejected with 400.
    let bad_put = app
        .clone()
        .oneshot(
            HttpRequest::builder()
                .method("PUT")
                .uri(format!(
                    "/operator/practices/{practice}/patients/{pw_healthy}/cadence?now={t0}"
                ))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&SetCadenceBody {
                        check_every_seconds: 0,
                        review_every_seconds: 3_600,
                    })
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bad_put.status(), StatusCode::BAD_REQUEST);

    // 3. Operator PUT updates cadence for pw_healthy to hourly check / 2-day periodic review.
    let put_resp = app
        .clone()
        .oneshot(
            HttpRequest::builder()
                .method("PUT")
                .uri(format!(
                    "/operator/practices/{practice}/patients/{pw_healthy}/cadence?now={t0}"
                ))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&SetCadenceBody {
                        check_every_seconds: 3_600,
                        review_every_seconds: 172_800,
                    })
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(put_resp.status(), StatusCode::OK);
    let updated: OperatorCadenceView =
        serde_json::from_slice(&to_bytes(put_resp.into_body(), 64 * 1024).await.unwrap()).unwrap();
    assert_eq!(updated.cadence.check_every_seconds(), 3_600);
    assert_eq!(updated.cadence.review_every_seconds(), 172_800);

    // 4. Wake-up at t0 + 120: pw_failing fails check, pw_healthy still checks & starts review!
    let wake_ts = t0 + 120;
    let wake_resp = app
        .clone()
        .oneshot(
            HttpRequest::builder()
                .method("POST")
                .uri(format!(
                    "/internal/practices/{practice}/dispatch/wake?now={wake_ts}"
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(wake_resp.status(), StatusCode::OK);
    let summary: WakeSummary =
        serde_json::from_slice(&to_bytes(wake_resp.into_body(), 64 * 1024).await.unwrap()).unwrap();
    assert_eq!(summary.failed_checks.len(), 1);
    assert_eq!(summary.failed_checks[0].patient_workspace_id, pw_failing);
    assert_eq!(summary.checked_patients, vec![pw_healthy.clone()]);
    assert_eq!(summary.started_reviews.len(), 1);
    assert_eq!(summary.started_reviews[0].patient_workspace_id, pw_healthy);

    // 5. Operator readback proves failed check recorded attempt, NOT success, and stays due;
    //    healthy patient recorded both attempt and success and moved next_due_at by 3600s.
    let fail_view = store
        .patient_dispatch_status(auth.as_ref(), &practice, &pw_failing, wake_ts + 10)
        .await
        .unwrap();
    assert_eq!(fail_view.last_attempt_at, Some(wake_ts));
    assert_eq!(fail_view.last_success_at, None);
    assert_eq!(fail_view.next_due_at, wake_ts);

    let ok_view_resp = app
        .oneshot(
            HttpRequest::builder()
                .method("GET")
                .uri(format!(
                    "/operator/practices/{practice}/patients/{pw_healthy}/cadence?now={}",
                    wake_ts + 10
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(ok_view_resp.status(), StatusCode::OK);
    let ok_view: OperatorCadenceView =
        serde_json::from_slice(&to_bytes(ok_view_resp.into_body(), 64 * 1024).await.unwrap())
            .unwrap();
    assert_eq!(ok_view.last_attempt_at, Some(wake_ts));
    assert_eq!(ok_view.last_success_at, Some(wake_ts));
    assert_eq!(ok_view.next_due_at, wake_ts + 3_600);
}

#[tokio::test]
async fn single_cadence_owner_unaffected_by_billing_and_duet_connection_changes() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let front_door = PostgresFrontDoorStore::connect_with_migrations(&url)
        .await
        .expect("connect and migrate front-door store");

    let practice = format!("practice-billing-{}", Uuid::new_v4().simple());
    let pw_with_duet = format!("pw-duet-{}", Uuid::new_v4().simple());
    let pw_clinic_only = format!("pw-noduet-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &pw_with_duet)
        .await
        .unwrap();
    auth.register_patient(&practice, &pw_clinic_only)
        .await
        .unwrap();

    let recorder = Arc::new(RecordingReceiver::default());
    let app = operator_router(DispatchEndpointState {
        store: store.clone(),
        auth: auth.clone(),
        checker: Arc::new(StoredObservationChecker::new(store.clone())),
        receiver: recorder,
    });

    let t0 = 1_760_300_000_i64;
    let custom_cadence = Cadence::new(7_200, 172_800).unwrap();
    store
        .set_patient_cadence(auth.as_ref(), &practice, &pw_with_duet, custom_cadence, t0)
        .await
        .unwrap();

    // Link pw_with_duet to Duet, grant consent, toggle premium ON and OFF, then withdraw consent.
    let now_u64 = t0 as u64;
    let roster_patient = RosterPatient {
        practice_id: practice.clone(),
        patient_id: pw_with_duet.clone(),
    };
    let email = format!("patient-{}@codahealth.test", Uuid::new_v4().simple());
    let fd_invite = front_door
        .invite(&roster_patient, &email, "Alex Patient", now_u64)
        .await
        .unwrap();
    let (duet_account_id, _session) = front_door
        .redeem_invitation(&email, &fd_invite, now_u64 + 1)
        .await
        .unwrap();
    front_door
        .grant_consent(duet_account_id, now_u64 + 2)
        .await
        .unwrap();

    let auth_invite = auth
        .invite_patient(&practice, &pw_with_duet, now_u64 + 3)
        .await
        .unwrap();
    let duet_id_str = duet_account_id.to_string();
    auth.accept_patient_invitation(&auth_invite, &duet_id_str, now_u64 + 4)
        .await
        .unwrap();
    auth.grant_patient_consent(&practice, &pw_with_duet, &duet_id_str, "v1", now_u64 + 5)
        .await
        .unwrap();

    // Toggle billing/premium ON, verify operator readback for both Duet and non-Duet patients.
    front_door
        .set_premium(&roster_patient, true, now_u64 + 6)
        .await
        .unwrap();
    let premium_on = front_door.premium(&roster_patient).await;
    assert!(premium_on.premium);
    let premium_json: Value = serde_json::to_value(&premium_on).unwrap();
    assert!(
        premium_json.get("cadence").is_none(),
        "Premium wire state must not carry cadence: {premium_json}"
    );

    for (pw, expected) in [
        (&pw_with_duet, custom_cadence),
        (&pw_clinic_only, Cadence::default()),
    ] {
        let resp = app
            .clone()
            .oneshot(
                HttpRequest::builder()
                    .method("GET")
                    .uri(format!(
                        "/operator/practices/{practice}/patients/{pw}/cadence?now={}",
                        t0 + 10
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let view: OperatorCadenceView =
            serde_json::from_slice(&to_bytes(resp.into_body(), 64 * 1024).await.unwrap()).unwrap();
        assert_eq!(view.cadence, expected);
    }

    // Run initial scheduled wake-up at t0 + 20: both patients check and set their schedules.
    let wake1_resp = app
        .clone()
        .oneshot(
            HttpRequest::builder()
                .method("POST")
                .uri(format!(
                    "/internal/practices/{practice}/dispatch/wake?now={}",
                    t0 + 20
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(wake1_resp.status(), StatusCode::OK);
    let wake1: WakeSummary =
        serde_json::from_slice(&to_bytes(wake1_resp.into_body(), 64 * 1024).await.unwrap())
            .unwrap();
    assert_eq!(wake1.checked_patients.len(), 2);

    // Toggle premium OFF and disconnect sharing on both front-door and physician-auth.
    front_door
        .set_premium(&roster_patient, false, now_u64 + 30)
        .await
        .unwrap();
    let premium_off = front_door.premium(&roster_patient).await;
    assert!(!premium_off.premium);
    front_door
        .withdraw_consent(duet_account_id, now_u64 + 31)
        .await
        .unwrap();
    auth.revoke_patient_consent(&practice, &pw_with_duet, now_u64 + 32)
        .await
        .unwrap();

    // Operator readback and scheduled wake-up at t0 + 20 + 7_200 confirm neither patient's
    // cadence nor due schedule changed after billing and connection transitions!
    let duet_after = store
        .patient_dispatch_status(auth.as_ref(), &practice, &pw_with_duet, t0 + 40)
        .await
        .unwrap();
    assert_eq!(duet_after.cadence, custom_cadence);
    assert_eq!(duet_after.next_due_at, t0 + 20 + 7_200);

    let clinic_after = store
        .patient_dispatch_status(auth.as_ref(), &practice, &pw_clinic_only, t0 + 40)
        .await
        .unwrap();
    assert_eq!(clinic_after.cadence, Cadence::default());
    assert_eq!(clinic_after.next_due_at, t0 + 20 + 86_400);

    let wake2_resp = app
        .oneshot(
            HttpRequest::builder()
                .method("POST")
                .uri(format!(
                    "/internal/practices/{practice}/dispatch/wake?now={}",
                    t0 + 20 + 7_200
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(wake2_resp.status(), StatusCode::OK);
    let wake2: WakeSummary =
        serde_json::from_slice(&to_bytes(wake2_resp.into_body(), 64 * 1024).await.unwrap())
            .unwrap();
    assert_eq!(wake2.checked_patients, vec![pw_with_duet]);
    assert_eq!(wake2.skipped_not_due, vec![pw_clinic_only]);
}

#[tokio::test]
async fn baseline_chart_registered_patient_skips_first_review_until_records_change_or_periodic_due()
{
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-baseline-{}", Uuid::new_v4().simple());
    let pw_changes = format!("pw-base-chg-{}", Uuid::new_v4().simple());
    let pw_periodic = format!("pw-base-per-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();

    let t0 = 1_760_400_000_i64;

    // Load non-empty baseline charts BEFORE joining the practice, then register
    // and seed registration dispatch baselines (matching `load_synthetic_practice`).
    for (workspace, patient_id, name, note_id, summary) in [
        (
            &pw_changes,
            "pt-base-1",
            "Amara Singh",
            "baseline-labs-1",
            "Hemoglobin A1c 6.4%",
        ),
        (
            &pw_periodic,
            "pt-base-2",
            "Devon Brooks",
            "baseline-labs-2",
            "Blood pressure 122/76 mmHg",
        ),
    ] {
        let baseline_pull = local_product::runtime::sources::SourcePull {
            source_id: "clinic-ehr".into(),
            source_label: "Clinic EHR".into(),
            patient: source_patient(patient_id, name),
            pulled_at: Some(t0),
        };
        store
            .record_chart(
                workspace,
                &[chart_record(
                    workspace,
                    note_id,
                    "rev-1",
                    "2026-10-01",
                    summary,
                )],
                &baseline_pull,
            )
            .await
            .unwrap();
        auth.register_patient(&practice, workspace).await.unwrap();
        store
            .register_patient_dispatch(auth.as_ref(), &practice, workspace, t0)
            .await
            .unwrap();
    }

    let checker = StoredObservationChecker::new(store.clone());
    let recorder = RecordingReceiver::default();
    let receiver = TeeReceiver {
        recorder: &recorder,
        practice_jobs: PracticeJobReviewReceiver::new(
            store.clone(),
            &roles,
            &skills,
            auth.as_ref(),
        ),
    };

    // 1. First wake-up at `t0`: both pre-loaded charts are unchanged since registration -> no review.
    let wake0 = store
        .wake_practice_dispatch(auth.as_ref(), &practice, t0, &checker, &receiver)
        .await
        .unwrap();
    let mut expected_checked = vec![pw_changes.clone(), pw_periodic.clone()];
    expected_checked.sort();
    assert_eq!(wake0.checked_patients, expected_checked);
    assert!(wake0.failed_checks.is_empty());
    assert!(
        wake0.started_reviews.is_empty(),
        "baseline chart unchanged since registration must not start a spurious review: {wake0:?}"
    );
    assert!(recorder.requests.lock().unwrap().is_empty());

    // 2. Re-running chart load on `pw_changes` with an updated record + idempotent
    //    `register_patient_dispatch` preserves the baseline, so Day 1 starts `"new records"`.
    let day1 = t0 + DEFAULT_CHECK_EVERY_SECONDS.cast_signed();
    let updated_pull = local_product::runtime::sources::SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: source_patient("pt-base-1", "Amara Singh"),
        pulled_at: Some(day1 - 60),
    };
    store
        .record_chart(
            &pw_changes,
            &[chart_record(
                &pw_changes,
                "baseline-labs-1",
                "rev-2",
                "2026-10-02",
                "Hemoglobin A1c 7.1%",
            )],
            &updated_pull,
        )
        .await
        .unwrap();
    store
        .register_patient_dispatch(auth.as_ref(), &practice, &pw_changes, day1 - 60)
        .await
        .unwrap();

    let wake1 = store
        .wake_practice_dispatch(auth.as_ref(), &practice, day1, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake1.checked_patients, expected_checked);
    assert_eq!(wake1.started_reviews.len(), 1);
    assert_eq!(wake1.started_reviews[0].patient_workspace_id, pw_changes);
    assert_eq!(wake1.started_reviews[0].reason, ReviewReason::NewRecords);
    assert_eq!(wake1.started_reviews[0].reason.as_str(), "new records");

    // 3. Days 2..6 with no further changes start no reviews; Day 7 starts `"periodic"` for `pw_periodic`.
    for day_idx in 2..7 {
        let day_t = t0 + day_idx * DEFAULT_CHECK_EVERY_SECONDS.cast_signed();
        let mid_wake = store
            .wake_practice_dispatch(auth.as_ref(), &practice, day_t, &checker, &receiver)
            .await
            .unwrap();
        assert!(
            mid_wake.started_reviews.is_empty(),
            "day {day_idx} must not start any review before weekly window: {mid_wake:?}"
        );
    }

    let day7 = t0 + DEFAULT_REVIEW_EVERY_SECONDS.cast_signed();
    let wake7 = store
        .wake_practice_dispatch(auth.as_ref(), &practice, day7, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake7.started_reviews.len(), 1);
    assert_eq!(wake7.started_reviews[0].patient_workspace_id, pw_periodic);
    assert_eq!(wake7.started_reviews[0].reason, ReviewReason::Periodic);
    assert_eq!(wake7.started_reviews[0].reason.as_str(), "periodic");
}

#[tokio::test]
async fn unavailable_practice_patients_fails_dispatch_reads_and_wakeups_instead_of_reading_empty() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, _shared_auth) = connect_stores(&url).await;
    let lost_auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .expect("connect disposable PostgresAuthStore");

    let practice = format!("practice-lost-auth-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-lost-auth-{}", Uuid::new_v4().simple());
    lost_auth
        .ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    lost_auth
        .register_patient(&practice, &workspace)
        .await
        .unwrap();
    lost_auth.pool().close().await;

    let t0 = 1_760_500_000_i64;
    let checker = StoredObservationChecker::new(store.clone());
    let recorder = RecordingReceiver::default();

    assert!(matches!(
        store
            .practice_dispatch_statuses(&lost_auth, &practice, t0)
            .await,
        Err(DispatchError::Access(
            PracticeAccessError::StoreUnavailable(_)
        ))
    ));
    assert!(matches!(
        store
            .check_due_patients(&lost_auth, &practice, t0, &checker)
            .await,
        Err(DispatchError::Access(
            PracticeAccessError::StoreUnavailable(_)
        ))
    ));
    assert!(matches!(
        store
            .deliver_pending_reviews(&lost_auth, &practice, &recorder)
            .await,
        Err(DispatchError::Access(
            PracticeAccessError::StoreUnavailable(_)
        ))
    ));
    assert!(matches!(
        store
            .wake_practice_dispatch(&lost_auth, &practice, t0, &checker, &recorder)
            .await,
        Err(DispatchError::Access(
            PracticeAccessError::StoreUnavailable(_)
        ))
    ));
}

#[test]
fn practice_job_recipe_keeps_clean_model_text_and_valid_skill_tools() {
    let skills =
        PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
            .expect("physician skills catalog loads");
    let roles = distinct_roles();
    let assignment = Assignment::synthetic_patient_stub("pw-synthetic-model-text", &skills)
        .expect("synthetic patient assignment loads");
    let built = local_product::runtime::jobs::recipe(&roles, &assignment, &skills);

    model_text::assert_clean_model_text(&built.instructions);
    model_text::assert_clean_model_text(&serde_json::to_string(&built.tools).unwrap());

    let preloaded_package = json!({
        "skills": assignment
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
