//! Real PostgreSQL integration tests for practice-owned background jobs and
//! patient source check / review dispatch.
//!
//! Runs automatically whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL` or
//! `PHYSICIAN_DATABASE_URL` is set; panics if neither is set when
//! `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::path::PathBuf;
use std::sync::Arc;

use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourceRevision,
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
use physician_auth::{AuthStore, PostgresAuthStore, PracticeClock};
use serde_json::{Value, json};
use uuid::Uuid;

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
