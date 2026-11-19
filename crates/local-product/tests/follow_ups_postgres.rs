//! Real Postgres proof for patient follow-ups: creating through `FOLLOW_UP_TOOL`,
//! changing through follow-up operations across restarts with one `Updates` entry
//! per change, once-only command idempotency, `Stop` / `Resume` / `Review now`
//! scheduling, terminal `Completed` / `Cancelled` irreversibility, `CannotRestart`
//! physician choices without automatic restart, durable commit probing, panel
//! authorization, and wrong-patient source withdrawal holdbacks.

use std::path::PathBuf;

use local_product::{
    ModelBinding, ModelRoles,
    patient_reviews::{
        COMPLETE_PATIENT_REVIEW_TOOL, CompleteReviewArgs, CompleteReviewDisposition,
    },
    physician_chat::{ChatAssignment, PhysicianSkills},
    physician_sources::{
        ChartRequest, ChartScope, MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate,
        SourcePatient, SourceRevision, read_tool,
    },
    runtime::{
        Accepted, Authority, Input, PhysicianHost, PublicationTransaction, Scope, Store,
        WorkStatus, async_trait,
        concerns::{ExactRecordReference, SourceWork, WorkspaceAccess},
        dispatch::StoredObservationChecker,
        engine::{
            self, AcceptedInput, Event, Invocation, Item, Product, ProductMessage, ToolOutcome,
            Turn, WindowSeed,
            recipe::{Finish, Observer, Provider, Request, RequestBinding, RequestRecipe},
        },
        follow_ups::{
            CannotRestartAction, CannotRestartResolution, FOLLOW_UP_TOOL, FollowUpActor,
            FollowUpChangeNote, FollowUpCommandKey, FollowUpCommitProbe, FollowUpError, FollowUpId,
            FollowUpMutation, FollowUpReviewOutcome, FollowUpReviewSchedule, FollowUpStanding,
            FollowUpSummary, FollowUpToolCall, FollowUpUpdateKind, PhysicianChangeFollowUp,
            PhysicianCreateFollowUp, SourceWithdrawalOutcome, WithdrawWrongPatientRequest,
            follow_up_tool, is_follow_up_tool,
        },
        jobs::PracticeJobHost,
        reviews::PatientReviewReceiver,
        sources::SourcePull,
    },
    sha256_hex,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use sea_orm::DbErr;
use serde_json::{Value, json};

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

fn unix_seconds() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice_id: &str,
    label: &str,
) -> (PhysicianIdentity, String, Vec<u8>) {
    let email = format!("{label}-{}@codahealth.test", uuid::Uuid::new_v4().simple());
    let now = unix_seconds();
    let invite = auth
        .invite(practice_id, &email, label, false, now)
        .await
        .unwrap();
    let (identity, token) = auth.redeem_invitation(&email, &invite, now).await.unwrap();
    let hash = PostgresAuthStore::session_reference(&token);
    (identity, token, hash)
}

fn seeded_record(
    workspace: &str,
    source_id: &str,
    candidate_id: &str,
    revision: &str,
    date: &str,
    detail: &str,
) -> SourceRevision {
    let content = json!({
        "resourceType": "Observation",
        "id": candidate_id,
        "date": date,
        "detail": detail,
    })
    .to_string();
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: "Observation".into(),
        dates: vec![RecordDate {
            name: "effectiveDate".into(),
            value: date.to_owned(),
        }],
        sha256: sha256_hex(content.as_bytes()),
        content,
        readable_parts: vec![],
        provenance: json!({"system": source_id, "id": candidate_id}),
        envelope: None,
    }
}

struct FollowUpJobHost {
    store: Store,
    practice: String,
    workspace: String,
    source_work: SourceWork,
    sources: Vec<SourceRevision>,
    events: Vec<Event>,
}

#[async_trait]
impl Product for FollowUpJobHost {
    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<ProductMessage>>, engine::ProductError> {
        Ok(Some(vec![]))
    }

    async fn admit_binding(&mut self, _: &RequestBinding) -> Result<(), engine::ProductError> {
        Ok(())
    }

    fn read_only(&self, name: &str) -> bool {
        name == READ_TOOL
    }

    fn read_result(&mut self, _: &str) -> engine::recipe::ToolFuture<Option<String>> {
        Box::pin(async { Ok(None) })
    }

    async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
        Ok(())
    }

    async fn persist(&mut self, event: Event) -> Result<(), engine::ProductError> {
        self.events.push(event);
        Ok(())
    }

    fn execute(&mut self, call: &Invocation, arguments: Value) -> engine::recipe::ToolFuture {
        let store = self.store.clone();
        let practice = self.practice.clone();
        let workspace = self.workspace.clone();
        let source_work = self.source_work.clone();
        let sources = self.sources.clone();
        let typed_call = Invocation {
            id: call.id.clone(),
            name: call.name.clone(),
            arguments: arguments.clone(),
        };
        Box::pin(async move {
            if is_follow_up_tool(&typed_call.name) {
                return store
                    .execute_follow_up_tool(&practice, &workspace, &source_work, &typed_call)
                    .await;
            }
            if typed_call.name == READ_TOOL {
                let request =
                    match ChartRequest::parse(&typed_call.name, &arguments, ChartScope::Chat) {
                        Ok(request) => request,
                        Err(reason) => return Ok(ToolOutcome::refused(reason.to_string())),
                    };
                return request.run(&workspace, 1, &sources).map_err(Into::into);
            }
            Ok(ToolOutcome::refused(format!(
                "unknown job tool {}",
                typed_call.name
            )))
        })
    }

    async fn finish(&mut self, _: &Turn) -> Result<Finish, engine::ProductError> {
        Ok(Finish::Committed)
    }

    async fn finish_partial(
        &mut self,
        _: &Turn,
        _: &[engine::LimitBackstop],
    ) -> Result<(), engine::ProductError> {
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<WindowSeed, engine::ProductError> {
        Ok(WindowSeed {
            items: vec![],
            note_references: vec![],
        })
    }
}

async fn run_follow_up_job<P: Provider>(
    store: &Store,
    practice: &str,
    workspace: &str,
    source_work: SourceWork,
    sources: Vec<SourceRevision>,
    provider: &mut P,
) {
    let recipe = RequestRecipe {
        model: "controlled-follow-up-job".into(),
        context_window_tokens: 200_000,
        max_input_tokens: 150_000,
        options: json!({"effort": "high"}),
        instructions: "Review patient chart updates and manage clinical follow-ups.".into(),
        tools: vec![
            follow_up_tool().function(),
            read_tool(ChartScope::Chat).function(),
        ],
    };
    let mut request = recipe.bind(None).request(Vec::new());
    request.items.clear();
    let work = request
        .freeze()
        .unwrap()
        .recover(
            &source_work.attempt_id,
            engine::recovery::Records {
                events: vec![],
                allowance_after: 0,
            },
            engine::recovery::Messages {
                sources: &[],
                partial: None,
            },
            engine::recovery::Inputs {
                accepted: vec![AcceptedInput {
                    id: source_work.work_id.clone(),
                    items: vec![Item::ProductMessage(ProductMessage {
                        id: source_work.work_id.clone(),
                        role: engine::history::Role::User,
                        text: "Evaluate patient follow-up items.".into(),
                    })],
                }],
                source_feedback: None,
            },
        )
        .unwrap();
    let mut host = FollowUpJobHost {
        store: store.clone(),
        practice: practice.to_owned(),
        workspace: workspace.to_owned(),
        source_work,
        sources,
        events: Vec::new(),
    };
    work.run(
        provider,
        &mut host,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
}

#[test]
fn follow_up_tool_schema_and_model_text_are_clean() {
    let tool = follow_up_tool();
    assert_eq!(tool.name, FOLLOW_UP_TOOL);
    let serialized = serde_json::to_string(&tool.function()).unwrap();
    model_text::assert_clean_model_text(&serialized);

    let parsed = FollowUpToolCall::parse(
        FOLLOW_UP_TOOL,
        &json!({
            "operation": "create",
            "follow_up_id": "repeat-bmp-14d",
            "summary": "Recheck basic metabolic panel after lisinopril titration.",
            "due_at": 1_795_000_000,
            "records": [{"candidate_id": "bmp-1", "revision": "r1"}]
        }),
    )
    .expect("valid create call parses");
    assert!(matches!(parsed, FollowUpToolCall::Create { .. }));

    assert!(
        FollowUpToolCall::parse(
            FOLLOW_UP_TOOL,
            &json!({
                "operation": "create",
                "follow_up_id": "bad-due",
                "summary": "Zero due date must be rejected.",
                "due_at": 0
            }),
        )
        .is_err(),
        "non-positive due_at must fail validation"
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn create_through_tool_change_through_operation_restart_and_once_only_idempotency() {
    struct CreateFollowUpProvider {
        step: usize,
        due_at: i64,
    }
    impl Provider for CreateFollowUpProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "read-bmp-1".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{"candidate_id": "bmp-1", "revision": "r1", "cursor": null}],
                                "max_result_bytes": 8192
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "create-bad-ref".into(),
                            name: FOLLOW_UP_TOOL.into(),
                            arguments: json!({
                                "operation": "create",
                                "follow_up_id": "unverified-fu",
                                "summary": "Should be refused because revision does not exist.",
                                "due_at": self.due_at,
                                "records": [{"candidate_id": "bmp-1", "revision": "missing-rev"}]
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "create-bmp-followup".into(),
                            name: FOLLOW_UP_TOOL.into(),
                            arguments: json!({
                                "operation": "create",
                                "follow_up_id": "repeat-bmp-potassium",
                                "summary": "Verify repeat metabolic panel within 7 days after potassium 5.6 mmol/L.",
                                "due_at": self.due_at,
                                "records": [{"candidate_id": "bmp-1", "revision": "r1"}]
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"step": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let bad_output = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            Item::MaterializedResult { call_id, output }
                                if call_id == "create-bad-ref" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("refusal materialized");
                    assert!(
                        bad_output.contains("not in the workspace's current retained sources"),
                        "unverified citation must be refused: {bad_output}"
                    );
                    Ok(Turn {
                        items: vec![Item::assistant("Follow-up scheduled.")],
                        receipt: json!({"step": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected step {other}"),
            }
        }
    }

    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();

    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-patel").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let bmp_1 = seeded_record(
        &workspace,
        "clinic-lab",
        "bmp-1",
        "r1",
        "2026-11-10",
        "Potassium 5.6 mmol/L",
    );
    store.record_source(&bmp_1).await.unwrap();

    let initial_due = i64::try_from(unix_seconds()).unwrap() + 7 * 86_400;
    let work_1 = SourceWork::new("patient_review", "work-fu-1", "attempt-1").unwrap();
    run_follow_up_job(
        &store,
        &practice,
        &workspace,
        work_1.clone(),
        vec![bmp_1.clone()],
        &mut CreateFollowUpProvider {
            step: 0,
            due_at: initial_due,
        },
    )
    .await;

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &physician.physician_id,
        workspace: &workspace,
        session_hash: &session_hash,
    };

    // Change the created follow-up through the physician operation.
    let revised_due = initial_due + 7 * 86_400;
    let change_key = FollowUpCommandKey::new("cmd-change-bmp-fu-1").unwrap();
    let fu_id = FollowUpId::new("repeat-bmp-potassium").unwrap();
    let change_input = PhysicianChangeFollowUp {
        id: fu_id.clone(),
        summary: FollowUpSummary::new(
            "Verify repeat metabolic panel in 14 days after dietary counseling.",
        )
        .unwrap(),
        due_at: revised_due,
        records: vec![ExactRecordReference {
            candidate_id: "bmp-1".into(),
            revision: "r1".into(),
        }],
        note: FollowUpChangeNote::new("Extended follow-up window to 14 days per clinic call.")
            .unwrap(),
    };
    let changed = store
        .change_follow_up_for_physician(
            &access,
            &change_key,
            &change_input,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(changed, FollowUpMutation::Applied(_)));
    assert_eq!(changed.follow_up().version, 2);

    // Repeating the same command key makes one change (Duplicate, version stays 2).
    let duplicate = store
        .change_follow_up_for_physician(
            &access,
            &change_key,
            &change_input,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(duplicate, FollowUpMutation::Duplicate(_)));
    assert_eq!(duplicate.follow_up().version, 2);

    // Reusing the same command key with a different payload must fail closed.
    let collision = store
        .change_follow_up_for_physician(
            &access,
            &change_key,
            &PhysicianChangeFollowUp {
                id: fu_id.clone(),
                summary: FollowUpSummary::new("Conflicting payload under same key.").unwrap(),
                due_at: revised_due + 86_400,
                records: vec![],
                note: FollowUpChangeNote::new("Different note.").unwrap(),
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await;
    assert!(matches!(
        collision,
        Err(FollowUpError::IdempotencyCollision { .. })
    ));

    // Drop the store and reconnect a fresh pool to verify restart persistence.
    drop(store);
    let restarted = Store::connect(&url).await.unwrap();

    let items = restarted
        .physician_follow_ups(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(items.len(), 1, "expected exactly one durable follow-up");
    assert_eq!(items[0].id, fu_id);
    assert_eq!(items[0].version, 2);
    assert_eq!(items[0].due_at, revised_due);
    assert_eq!(
        items[0].summary.as_str(),
        "Verify repeat metabolic panel in 14 days after dietary counseling."
    );
    assert_eq!(
        items[0].origin,
        FollowUpActor::CodaWork {
            source_work: work_1
        }
    );

    let updates = restarted
        .physician_follow_up_updates(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(
        updates.len(),
        2,
        "expected exactly one Updates entry per committed change (Created + Changed)"
    );
    assert_eq!(updates[0].version, 1);
    assert_eq!(updates[0].kind, FollowUpUpdateKind::Created);
    assert_eq!(updates[1].version, 2);
    assert_eq!(updates[1].kind, FollowUpUpdateKind::Changed);
    assert_eq!(
        updates[1].detail,
        "Extended follow-up window to 14 days per clinic call."
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn stop_holds_follow_up_and_resume_or_review_now_marks_current_version_due() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();

    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-kim").await;
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

    let now_ts = i64::try_from(unix_seconds()).unwrap();
    let future_due = now_ts + 30 * 86_400;
    let fu_a = FollowUpId::new("fu-hold-resume").unwrap();
    let fu_b = FollowUpId::new("fu-review-now").unwrap();

    store
        .create_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("create-a").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_a.clone(),
                summary: FollowUpSummary::new("Watch outpatient echo report arrival.").unwrap(),
                due_at: future_due,
                records: vec![],
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    store
        .create_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("create-b").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_b.clone(),
                summary: FollowUpSummary::new("Watch ambulatory Holter monitor summary.").unwrap(),
                due_at: future_due + 86_400,
                records: vec![],
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    // Neither future-scheduled item is due right now.
    assert!(
        store
            .due_follow_ups(&practice, &workspace, now_ts)
            .await
            .unwrap()
            .is_empty()
    );

    // Stop holds `fu-hold-resume` so even past `future_due` it is not due for review.
    let stopped = store
        .stop_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("stop-a").unwrap(),
            &fu_a,
            Some(FollowUpChangeNote::new("Patient travelling abroad for two weeks.").unwrap()),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        stopped.follow_up().standing,
        FollowUpStanding::Held { paused_due_at, .. } if paused_due_at == future_due
    ));
    let due_at_future = store
        .due_follow_ups(&practice, &workspace, future_due + 100)
        .await
        .unwrap();
    assert!(
        due_at_future.iter().all(|item| item.id != fu_a),
        "held follow-up must never be due while stopped"
    );

    // Resume marks `fu-hold-resume`'s current version immediately due for review (`DueNow`),
    // even at `now_ts < future_due`.
    let resumed = store
        .resume_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("resume-a").unwrap(),
            &fu_a,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(resumed.follow_up().version, 3);
    assert!(matches!(
        resumed.follow_up().standing,
        FollowUpStanding::Active {
            schedule: FollowUpReviewSchedule::DueNow { .. }
        }
    ));

    // `Review now` on `fu-review-now` also marks its current version immediately due (`DueNow`).
    let reviewed_now = store
        .review_follow_up_now_for_physician(
            &access,
            &FollowUpCommandKey::new("review-now-b").unwrap(),
            &fu_b,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(reviewed_now.follow_up().version, 2);
    assert!(matches!(
        reviewed_now.follow_up().standing,
        FollowUpStanding::Active {
            schedule: FollowUpReviewSchedule::DueNow { .. }
        }
    ));

    let due_now_list = store
        .due_follow_ups(&practice, &workspace, now_ts)
        .await
        .unwrap();
    let due_ids: Vec<&str> = due_now_list.iter().map(|f| f.id.as_str()).collect();
    assert_eq!(due_ids, vec!["fu-hold-resume", "fu-review-now"]);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn terminal_follow_ups_never_reopen_and_cannot_restart_offers_physician_choices_without_auto_restart()
 {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();

    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-rivera").await;
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

    let due_ts = i64::try_from(unix_seconds()).unwrap() + 3600;
    let fu_completed = FollowUpId::new("fu-completed").unwrap();
    let fu_cancelled = FollowUpId::new("fu-cancelled").unwrap();
    let fu_stuck = FollowUpId::new("fu-cannot-restart").unwrap();

    for (key, id) in [
        ("c-1", &fu_completed),
        ("c-2", &fu_cancelled),
        ("c-3", &fu_stuck),
    ] {
        store
            .create_follow_up_for_physician(
                &access,
                &FollowUpCommandKey::new(key).unwrap(),
                &PhysicianCreateFollowUp {
                    id: id.clone(),
                    summary: FollowUpSummary::new(format!("Follow-up item {id}")).unwrap(),
                    due_at: due_ts,
                    records: vec![],
                },
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap();
    }

    // Complete `fu-completed` and cancel `fu-cancelled`.
    store
        .complete_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("done-1").unwrap(),
            &fu_completed,
            FollowUpChangeNote::new("Repeat lab confirmed normal potassium.").unwrap(),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    store
        .cancel_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("cancel-2").unwrap(),
            &fu_cancelled,
            FollowUpChangeNote::new("Order superseded by inpatient admission plan.").unwrap(),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    // Late review outcomes, physician changes, and late `FOLLOW_UP_TOOL` calls can NEVER reopen
    // a completed or cancelled follow-up.
    let late_work = SourceWork::new("patient_review", "late-review-work", "attempt-2").unwrap();
    for (idx, terminal_id) in [&fu_completed, &fu_cancelled].iter().enumerate() {
        let late_revise = store
            .apply_follow_up_review_outcome(
                &practice,
                &workspace,
                &FollowUpCommandKey::new(format!("late-revise-{idx}")).unwrap(),
                terminal_id,
                late_work.clone(),
                FollowUpReviewOutcome::Revise {
                    summary: FollowUpSummary::new("Late attempt trying to revive item.").unwrap(),
                    due_at: due_ts + 86_400,
                    records: vec![],
                    note: FollowUpChangeNote::new("Late revise attempt.").unwrap(),
                },
            )
            .await;
        assert!(matches!(late_revise, Err(FollowUpError::Terminal { .. })));

        let late_review_now = store
            .review_follow_up_now_for_physician(
                &access,
                &FollowUpCommandKey::new(format!("late-review-now-{idx}")).unwrap(),
                terminal_id,
                &mut PostgresWorkspaceAuthority,
            )
            .await;
        assert!(matches!(
            late_review_now,
            Err(FollowUpError::Terminal { .. })
        ));

        let late_tool_outcome = store
            .execute_follow_up_tool(
                &practice,
                &workspace,
                &late_work,
                &Invocation {
                    id: format!("tool-reopen-{idx}"),
                    name: FOLLOW_UP_TOOL.into(),
                    arguments: json!({
                        "operation": "change",
                        "follow_up_id": terminal_id.as_str(),
                        "summary": "Tool attempting to reopen terminal follow-up.",
                        "due_at": due_ts + 86_400,
                        "note": "Should be refused."
                    }),
                },
            )
            .await
            .unwrap();
        assert!(
            matches!(late_tool_outcome, ToolOutcome::Refused(_)),
            "tool must refuse reopening terminal follow-up: {late_tool_outcome:?}"
        );
    }

    // Mark `fu-cannot-restart` as `CannotRestart`: verify it offers [Complete, NewDate, Cancel]
    // and never auto-restarts even far past `due_ts`.
    let marked_stuck = store
        .apply_follow_up_review_outcome(
            &practice,
            &workspace,
            &FollowUpCommandKey::new("stuck-outcome").unwrap(),
            &fu_stuck,
            SourceWork::new("patient_review", "failed-review-work", "attempt-3").unwrap(),
            FollowUpReviewOutcome::CannotRestart {
                failed_work_id: "failed-review-work".into(),
                reason: "Provider allowance exhausted across all retry attempts.".into(),
            },
        )
        .await
        .unwrap();
    assert!(matches!(
        marked_stuck.follow_up().standing,
        FollowUpStanding::CannotRestart { .. }
    ));
    assert_eq!(
        marked_stuck.follow_up().standing.cannot_restart_choices(),
        &[
            CannotRestartAction::Complete,
            CannotRestartAction::NewDate,
            CannotRestartAction::Cancel,
        ]
    );
    assert!(
        store
            .due_follow_ups(&practice, &workspace, due_ts + 1_000_000)
            .await
            .unwrap()
            .is_empty(),
        "CannotRestart follow-up must never auto-restart in due_follow_ups"
    );

    // Ordinary `change` (both tool and physician) and `FollowUpReviewOutcome::Revise` MUST refuse
    // a `CannotRestart` follow-up so only explicit `resolve_cannot_restart_for_physician` can clear it.
    let refused_stuck_tool = store
        .execute_follow_up_tool(
            &practice,
            &workspace,
            &SourceWork::new("patient_review", "late-work-stuck", "attempt-1").unwrap(),
            &Invocation {
                id: "tool-change-cannot-restart".into(),
                name: FOLLOW_UP_TOOL.into(),
                arguments: json!({
                    "operation": "change",
                    "follow_up_id": fu_stuck.as_str(),
                    "summary": "Attempt to overwrite CannotRestart via ordinary change.",
                    "due_at": due_ts + 3600,
                    "note": "Should be refused."
                }),
            },
        )
        .await
        .unwrap();
    assert!(
        matches!(refused_stuck_tool, ToolOutcome::Refused(_)),
        "FOLLOW_UP_TOOL change must refuse CannotRestart standing: {refused_stuck_tool:?}"
    );
    assert!(matches!(
        store
            .change_follow_up_for_physician(
                &access,
                &FollowUpCommandKey::new("physician-change-stuck-refused").unwrap(),
                &PhysicianChangeFollowUp {
                    id: fu_stuck.clone(),
                    summary: FollowUpSummary::new("Attempt ordinary physician edit on stuck item.")
                        .unwrap(),
                    due_at: due_ts + 3600,
                    records: vec![],
                    note: FollowUpChangeNote::new("Must use resolve_cannot_restart.").unwrap(),
                },
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(FollowUpError::InvalidTransition { .. })
    ));
    assert!(matches!(
        store
            .apply_follow_up_review_outcome(
                &practice,
                &workspace,
                &FollowUpCommandKey::new("review-revise-stuck-refused").unwrap(),
                &fu_stuck,
                SourceWork::new("patient_review", "late-work-revise", "attempt-1").unwrap(),
                FollowUpReviewOutcome::Revise {
                    summary: FollowUpSummary::new("Attempt review revise on stuck item.").unwrap(),
                    due_at: due_ts + 3600,
                    records: vec![],
                    note: FollowUpChangeNote::new("Must be refused.").unwrap(),
                },
            )
            .await,
        Err(FollowUpError::InvalidTransition { .. })
    ));

    // Physician resolves `CannotRestart` with `NewDate`.
    let rescheduled_due = due_ts + 7 * 86_400;
    let rescheduled = store
        .resolve_cannot_restart_for_physician(
            &access,
            &FollowUpCommandKey::new("resolve-new-date").unwrap(),
            &fu_stuck,
            CannotRestartResolution::NewDate {
                due_at: rescheduled_due,
                note: FollowUpChangeNote::new("Rescheduled review for next week.").unwrap(),
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        rescheduled.follow_up().standing,
        FollowUpStanding::Active {
            schedule: FollowUpReviewSchedule::Scheduled { due_at }
        } if due_at == rescheduled_due
    ));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn review_outcomes_commit_durability_probe_and_unauthorized_physician_refusal() {
    struct FailingCommitProbe;
    impl FollowUpCommitProbe for FailingCommitProbe {
        fn before_commit(&self) -> Result<(), DbErr> {
            Err(DbErr::Custom(
                "simulated Postgres commit failure before durable save".into(),
            ))
        }
    }

    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();

    let practice = uuid::Uuid::new_v4().to_string();
    let other_practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let (authorized_doc, _tok_ok, ok_hash) =
        invite_and_sign_in(&auth, &practice, "dr-authorized").await;
    let (unlinked_doc, _tok_unlinked, unlinked_hash) =
        invite_and_sign_in(&auth, &practice, "dr-unlinked").await;
    let (outsider_doc, _tok_outsider, outsider_hash) =
        invite_and_sign_in(&auth, &other_practice, "dr-outsider").await;

    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&authorized_doc, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let ok_access = WorkspaceAccess {
        practice: &practice,
        physician: &authorized_doc.physician_id,
        workspace: &workspace,
        session_hash: &ok_hash,
    };
    let unlinked_access = WorkspaceAccess {
        practice: &practice,
        physician: &unlinked_doc.physician_id,
        workspace: &workspace,
        session_hash: &unlinked_hash,
    };
    let outsider_access = WorkspaceAccess {
        practice: &other_practice,
        physician: &outsider_doc.physician_id,
        workspace: &workspace,
        session_hash: &outsider_hash,
    };

    // 1. Inject DB commit failure via `FollowUpCommitProbe`: tool MUST fail and leave zero saved state.
    let due_ts = i64::try_from(unix_seconds()).unwrap() + 86_400;
    let work = SourceWork::new("patient_review", "work-durability-1", "attempt-1").unwrap();
    let failed_commit = store
        .execute_follow_up_tool_with_probe(
            &practice,
            &workspace,
            &work,
            &Invocation {
                id: "call-uncommitted".into(),
                name: FOLLOW_UP_TOOL.into(),
                arguments: json!({
                    "operation": "create",
                    "follow_up_id": "fu-never-committed",
                    "summary": "Must roll back completely when commit fails.",
                    "due_at": due_ts
                }),
            },
            &FailingCommitProbe,
        )
        .await;
    assert!(
        failed_commit.is_err(),
        "FOLLOW_UP_TOOL must report failure when Postgres commit fails"
    );
    assert!(
        store
            .workspace_follow_ups(&practice, &workspace)
            .await
            .unwrap()
            .is_empty(),
        "rolled-back tool call must leave zero follow-ups in Postgres"
    );
    assert!(
        store
            .physician_follow_up_updates(&ok_access, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap()
            .is_empty(),
        "rolled-back tool call must leave zero Updates entries in Postgres"
    );

    // 2. Commit cleanly, then revise and resolve the SAME follow-up through `apply_follow_up_review_outcome`.
    let fu_id = FollowUpId::new("fu-review-lifecycle").unwrap();
    let created_outcome = store
        .execute_follow_up_tool(
            &practice,
            &workspace,
            &work,
            &Invocation {
                id: "call-committed".into(),
                name: FOLLOW_UP_TOOL.into(),
                arguments: json!({
                    "operation": "create",
                    "follow_up_id": fu_id.as_str(),
                    "summary": "Check outside cardiology consult note.",
                    "due_at": due_ts
                }),
            },
        )
        .await
        .unwrap();
    assert!(matches!(created_outcome, ToolOutcome::Returned(_)));

    let revised = store
        .apply_follow_up_review_outcome(
            &practice,
            &workspace,
            &FollowUpCommandKey::new("review-revise-1").unwrap(),
            &fu_id,
            SourceWork::new("patient_review", "review-work-2", "attempt-1").unwrap(),
            FollowUpReviewOutcome::Revise {
                summary: FollowUpSummary::new(
                    "Cardiology appointment moved; recheck consult note in 5 days.",
                )
                .unwrap(),
                due_at: due_ts + 5 * 86_400,
                records: vec![],
                note: FollowUpChangeNote::new("Consult rescheduled by cardiology clinic.").unwrap(),
            },
        )
        .await
        .unwrap();
    assert_eq!(revised.follow_up().id, fu_id);
    assert_eq!(revised.follow_up().version, 2);

    // Unauthorized physicians (both unlinked same-practice and cross-practice) cannot read or change the follow-up.
    for bad_access in [&unlinked_access, &outsider_access] {
        assert!(matches!(
            store
                .physician_follow_ups(bad_access, &mut PostgresWorkspaceAuthority)
                .await,
            Err(FollowUpError::AccessDenied { .. })
        ));
        assert!(matches!(
            store
                .physician_follow_up_updates(bad_access, &mut PostgresWorkspaceAuthority)
                .await,
            Err(FollowUpError::AccessDenied { .. })
        ));
        assert!(matches!(
            store
                .change_follow_up_for_physician(
                    bad_access,
                    &FollowUpCommandKey::new("unauth-change").unwrap(),
                    &PhysicianChangeFollowUp {
                        id: fu_id.clone(),
                        summary: FollowUpSummary::new("Unauthorized edit").unwrap(),
                        due_at: due_ts + 100,
                        records: vec![],
                        note: FollowUpChangeNote::new("Unauthorized").unwrap(),
                    },
                    &mut PostgresWorkspaceAuthority,
                )
                .await,
            Err(FollowUpError::AccessDenied { .. })
        ));
    }

    let resolved = store
        .apply_follow_up_review_outcome(
            &practice,
            &workspace,
            &FollowUpCommandKey::new("review-resolve-2").unwrap(),
            &fu_id,
            SourceWork::new("patient_review", "review-work-3", "attempt-1").unwrap(),
            FollowUpReviewOutcome::Resolved {
                resolution: FollowUpChangeNote::new(
                    "Cardiology consult note received and reviewed.",
                )
                .unwrap(),
            },
        )
        .await
        .unwrap();
    assert_eq!(resolved.follow_up().id, fu_id);
    assert_eq!(resolved.follow_up().version, 3);
    assert!(matches!(
        resolved.follow_up().standing,
        FollowUpStanding::Completed { .. }
    ));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn removing_wrong_patient_message_holds_back_derived_follow_ups_and_pending_actions_without_touching_unrelated_follow_ups()
 {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();

    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-lin").await;
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

    let clean_record = seeded_record(
        &workspace,
        "clinic-lab",
        "cand-clean-a1c",
        "r1",
        "2026-11-10",
        "HbA1c 7.9%",
    );
    let wrong_msg_record = seeded_record(
        &workspace,
        "spruce",
        "cand-wrong-spruce-msg",
        "m1",
        "2026-11-11",
        "Wrong-patient message reporting warfarin INR 4.8",
    );
    store.record_source(&clean_record).await.unwrap();
    store.record_source(&wrong_msg_record).await.unwrap();

    let now_ts = i64::try_from(unix_seconds()).unwrap();
    let due_soon = now_ts - 60;

    // Create unrelated follow-up citing `cand-clean-a1c`.
    let clean_work = SourceWork::new("patient_review", "work-clean-fu", "attempt-1").unwrap();
    store
        .execute_follow_up_tool(
            &practice,
            &workspace,
            &clean_work,
            &Invocation {
                id: "call-clean".into(),
                name: FOLLOW_UP_TOOL.into(),
                arguments: json!({
                    "operation": "create",
                    "follow_up_id": "fu-clean-a1c",
                    "summary": "Recheck HbA1c in 90 days.",
                    "due_at": due_soon,
                    "records": [{"candidate_id": "cand-clean-a1c", "revision": "r1"}]
                }),
            },
        )
        .await
        .unwrap();

    // Create follow-up derived from the wrong-patient message AND mark it `DueNow`
    // so it has an active pending review action when withdrawal starts.
    let wrong_work = SourceWork::new("patient_review", "work-wrong-msg", "attempt-1").unwrap();
    let wrong_call = Invocation {
        id: "call-wrong".into(),
        name: FOLLOW_UP_TOOL.into(),
        arguments: json!({
            "operation": "create",
            "follow_up_id": "fu-wrong-inr",
            "summary": "Repeat INR within 48 hours for supratherapeutic reading.",
            "due_at": due_soon,
            "records": [{"candidate_id": "cand-wrong-spruce-msg", "revision": "m1"}]
        }),
    };
    store
        .execute_follow_up_tool(&practice, &workspace, &wrong_work, &wrong_call)
        .await
        .unwrap();
    store
        .review_follow_up_now_for_physician(
            &access,
            &FollowUpCommandKey::new("pending-review-wrong").unwrap(),
            &FollowUpId::new("fu-wrong-inr").unwrap(),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    assert_eq!(
        store
            .due_follow_ups(&practice, &workspace, now_ts)
            .await
            .unwrap()
            .len(),
        2
    );

    // Withdraw the wrong-patient message and verify derived follow-ups + pending review actions
    // are held back before completion, while `fu-clean-a1c` stays untouched.
    let withdrawal_req = WithdrawWrongPatientRequest {
        practice: practice.clone(),
        workspace: workspace.clone(),
        withdrawal_id: "withdraw-spruce-msg-1".into(),
        candidate_ids: vec!["cand-wrong-spruce-msg".into()],
        message_id: Some("spruce-msg-991".into()),
        fenced_work_ids: vec!["work-wrong-msg".into()],
        reason: "Inbound Spruce message belonged to another patient and was unlinked.".into(),
        actor: physician.physician_id.clone(),
    };
    let withdrawal = store
        .withdraw_wrong_patient_source(&withdrawal_req)
        .await
        .unwrap();
    assert!(matches!(withdrawal, SourceWithdrawalOutcome::Applied(_)));
    assert_eq!(
        withdrawal.receipt().withdrawn_follow_up_ids,
        vec![FollowUpId::new("fu-wrong-inr").unwrap()]
    );

    // Repeating the withdrawal request with identical payload is idempotent.
    let dup_withdrawal = store
        .withdraw_wrong_patient_source(&withdrawal_req)
        .await
        .unwrap();
    assert!(matches!(
        dup_withdrawal,
        SourceWithdrawalOutcome::Duplicate(_)
    ));

    // Reusing `withdrawal_id` with different `candidate_ids` or `reason` fails with `IdempotencyCollision`.
    let mut colliding_candidates = withdrawal_req.clone();
    colliding_candidates.candidate_ids = vec!["cand-clean-a1c".into()];
    assert!(matches!(
        store
            .withdraw_wrong_patient_source(&colliding_candidates)
            .await,
        Err(FollowUpError::IdempotencyCollision { .. })
    ));
    let mut colliding_reason = withdrawal_req.clone();
    colliding_reason.reason = "Mutated reason for existing withdrawal key.".into();
    assert!(matches!(
        store.withdraw_wrong_patient_source(&colliding_reason).await,
        Err(FollowUpError::IdempotencyCollision { .. })
    ));

    // Pending due review action on `fu-wrong-inr` is immediately cleared; `fu-clean-a1c` remains due.
    let due_after = store
        .due_follow_ups(&practice, &workspace, now_ts)
        .await
        .unwrap();
    assert_eq!(due_after.len(), 1);
    assert_eq!(due_after[0].id.as_str(), "fu-clean-a1c");

    // Physician-facing clinical follow-up list and Updates feed EXCLUDE `WithdrawnWrongPatient`
    // items (`fu-wrong-inr`), returning only `fu-clean-a1c`.
    let physician_items = store
        .physician_follow_ups(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(physician_items.len(), 1);
    assert_eq!(physician_items[0].id.as_str(), "fu-clean-a1c");
    assert!(matches!(
        physician_items[0].standing,
        FollowUpStanding::Active { .. }
    ));

    let physician_updates = store
        .physician_follow_up_updates(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert!(
        !physician_updates
            .iter()
            .any(|u| u.follow_up_id.as_str() == "fu-wrong-inr"),
        "physician Updates must exclude wrong-patient withdrawn follow-up history: {physician_updates:?}"
    );
    assert_eq!(physician_updates.len(), 1);
    assert_eq!(physician_updates[0].follow_up_id.as_str(), "fu-clean-a1c");

    // Internal/audit reads retain `fu-wrong-inr` in `WithdrawnWrongPatient` standing and its audit Updates log.
    let audit_items = store
        .workspace_follow_ups(&practice, &workspace)
        .await
        .unwrap();
    assert_eq!(audit_items.len(), 2);
    let wrong_after = audit_items
        .iter()
        .find(|f| f.id.as_str() == "fu-wrong-inr")
        .unwrap();
    assert!(matches!(
        &wrong_after.standing,
        FollowUpStanding::WithdrawnWrongPatient { withdrawal_id, .. }
            if withdrawal_id == "withdraw-spruce-msg-1"
    ));
    let audit_updates = store
        .workspace_follow_up_updates_audit(&practice, &workspace)
        .await
        .unwrap();
    assert!(audit_updates.iter().any(|u| {
        u.follow_up_id.as_str() == "fu-wrong-inr"
            && u.kind == FollowUpUpdateKind::WithdrawnWrongPatient
    }));

    // Replay of the earlier `FOLLOW_UP_TOOL` call or a new write citing `cand-wrong-spruce-msg` is refused.
    let replay_outcome = store
        .execute_follow_up_tool(&practice, &workspace, &wrong_work, &wrong_call)
        .await
        .unwrap();
    assert!(matches!(replay_outcome, ToolOutcome::Refused(_)));

    let fresh_work_citing_withdrawn = store
        .execute_follow_up_tool(
            &practice,
            &workspace,
            &SourceWork::new("patient_review", "work-fresh-after", "attempt-1").unwrap(),
            &Invocation {
                id: "call-cite-withdrawn".into(),
                name: FOLLOW_UP_TOOL.into(),
                arguments: json!({
                    "operation": "create",
                    "follow_up_id": "fu-revive-wrong",
                    "summary": "Attempt to cite withdrawn wrong-patient record.",
                    "due_at": now_ts + 3600,
                    "records": [{"candidate_id": "cand-wrong-spruce-msg", "revision": "m1"}]
                }),
            },
        )
        .await
        .unwrap();
    assert!(matches!(
        fresh_work_citing_withdrawn,
        ToolOutcome::Refused(_)
    ));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn production_patient_review_and_physician_chat_hosts_offer_and_execute_follow_up_tool() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();

    let skills =
        PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
            .unwrap();
    let roles = ModelRoles {
        chat: ModelBinding {
            model: "chat-lane-followup".into(),
            effort: "medium".into(),
            context_window_tokens: 192_000,
            max_input_tokens: 160_000,
            adapter: "fixture".into(),
        },
        background: ModelBinding {
            model: "background-lane-followup".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        },
    };

    let practice = format!("practice-prod-fu-{}", uuid::Uuid::new_v4().simple());
    let workspace = format!("pw-prod-fu-{}", uuid::Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-prod").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let reg_time = 1_790_000_000_i64;
    store
        .register_patient_dispatch(&auth, &practice, &workspace, reg_time)
        .await
        .unwrap();

    let lab_content = "Serum ferritin 9 ng/mL on oral iron supplementation.";
    let lab_rev = SourceRevision {
        workspace: workspace.clone(),
        source_id: "clinic-ehr".into(),
        candidate_id: "Observation/ferritin-1".into(),
        revision: "rev-1".into(),
        record_type: "Observation".into(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".into(),
            value: "2026-11-10".into(),
        }],
        content: lab_content.into(),
        sha256: sha256_hex(lab_content.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "Ferritin".into(),
            text: lab_content.into(),
        }],
        provenance: json!({"title": "Ferritin", "id": "Observation/ferritin-1"}),
        envelope: None,
    };
    let pull = SourcePull {
        source_id: "clinic-ehr".into(),
        source_label: "Clinic EHR".into(),
        patient: SourcePatient {
            source_patient_id: "pt-ferritin".into(),
            display_name: "Nadia Vance".into(),
            birth_date: Some("1982-09-12".into()),
            sex_or_gender: Some("female".into()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "clinic-mrn".into(),
                value: "MRN-8821".into(),
            }],
        },
        pulled_at: Some(reg_time + 3600),
    };
    store
        .record_chart(&workspace, std::slice::from_ref(&lab_rev), &pull)
        .await
        .unwrap();

    // 1. Dispatch a real `PatientReview` job and run it via production `PracticeJobHost`.
    let checker = StoredObservationChecker::new(store.clone());
    let receiver = PatientReviewReceiver::new(&store, &roles, &skills, &auth);
    let wake = store
        .wake_practice_dispatch(&auth, &practice, reg_time + 3600, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake.started_reviews.len(), 1);
    let reviews = store
        .patient_reviews(&practice, &workspace, &auth)
        .await
        .unwrap();
    let review_scope = reviews[0].practice_scope();
    let claim = store
        .claim_practice_job(&review_scope, 60, &auth)
        .await
        .unwrap()
        .unwrap();

    let due_initial = i64::try_from(unix_seconds()).unwrap() + 14 * 86_400;
    let due_revised = due_initial + 7 * 86_400;

    struct ReviewCreateFollowUpProvider {
        turn: usize,
        due_at: i64,
    }
    impl Provider for ReviewCreateFollowUpProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            let tool_names: Vec<&str> = request
                .tools
                .iter()
                .filter_map(|t| t["name"].as_str())
                .collect();
            assert!(
                tool_names.contains(&FOLLOW_UP_TOOL),
                "PatientReview recipe must offer FOLLOW_UP_TOOL: {tool_names:?}"
            );
            match self.turn {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "read-ferritin".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "Observation/ferritin-1",
                                    "revision": "rev-1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "create-ferritin-fu".into(),
                            name: FOLLOW_UP_TOOL.into(),
                            arguments: json!({
                                "operation": "create",
                                "follow_up_id": "fu-prod-ferritin",
                                "summary": "Recheck CBC and ferritin in 2 weeks on oral iron.",
                                "due_at": self.due_at,
                                "records": [{"candidate_id": "Observation/ferritin-1", "revision": "rev-1"}]
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let fu_out = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            Item::MaterializedResult { call_id, output }
                                if call_id == "create-ferritin-fu" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("FOLLOW_UP_TOOL result materialized in PracticeJobHost");
                    let parsed: Value = serde_json::from_str(fu_out).unwrap();
                    assert_eq!(parsed["outcome"], "applied");
                    assert_eq!(parsed["follow_up"]["id"], "fu-prod-ferritin");

                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Low ferritin 9 ng/mL; scheduled 2-week ferritin follow-up.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/ferritin-1".into(),
                            revision: "rev-1".into(),
                        }],
                        basis_and_limitations: "Clinic EHR ferritin lab 2026-11-10.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "complete-rev".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(Turn {
                    items: vec![Item::assistant("Completed ferritin patient review.")],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected review turn {other}"),
            }
        }
    }

    let mut review_host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: &auth,
    };
    review_host
        .run(
            &mut ReviewCreateFollowUpProvider {
                turn: 0,
                due_at: due_initial,
            },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    // 2. Now run an authorized `PhysicianHost` chat turn using `physician_chat::recipe` and
    // revise the SAME `fu-prod-ferritin` follow-up via `FOLLOW_UP_TOOL`!
    let chat_scope = Scope {
        practice: practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "chat-prod-fu-1".into(),
    };
    let starting = store.patient_starting_context(&chat_scope).await.unwrap();
    let recipe = ChatAssignment::for_patient(chat_scope.clone(), starting).recipe(&roles, &skills);
    let accepted = store
        .start_chat_authorized(
            &chat_scope,
            &Input {
                key: "chat-fu-cmd-1".into(),
                text: "Move Nadia's ferritin follow-up out by one more week.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session_hash.clone(),
            &skills,
            &recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(accepted, Accepted::New);

    struct ChatReviseFollowUpProvider {
        turn: usize,
        due_at: i64,
    }
    impl Provider for ChatReviseFollowUpProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            let tool_names: Vec<&str> = request
                .tools
                .iter()
                .filter_map(|t| t["name"].as_str())
                .collect();
            assert!(
                tool_names.contains(&FOLLOW_UP_TOOL),
                "physician_chat recipe must offer FOLLOW_UP_TOOL: {tool_names:?}"
            );
            match self.turn {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "chat-read-ferritin".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "Observation/ferritin-1",
                                    "revision": "rev-1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "chat-change-fu".into(),
                            name: FOLLOW_UP_TOOL.into(),
                            arguments: json!({
                                "operation": "change",
                                "follow_up_id": "fu-prod-ferritin",
                                "summary": "Recheck CBC and ferritin in 3 weeks on oral iron.",
                                "due_at": self.due_at,
                                "records": [{"candidate_id": "Observation/ferritin-1", "revision": "rev-1"}],
                                "note": "Extended by 1 week per physician chat instruction."
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let fu_out = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            Item::MaterializedResult { call_id, output }
                                if call_id == "chat-change-fu" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("FOLLOW_UP_TOOL result materialized in PhysicianHost");
                    let parsed: Value = serde_json::from_str(fu_out).unwrap();
                    assert_eq!(parsed["outcome"], "applied");
                    assert_eq!(parsed["follow_up"]["version"], 2);
                    Ok(Turn {
                        items: vec![Item::Call(local_product::runtime::engine::Invocation {
                            id: "finish-fu-revise".into(),
                            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": "Updated the ferritin follow-up to 3 weeks [S1].",
                                "basis": "Clinic EHR ferritin result dated 2026-11-10.",
                                "descriptions": [], "quotes": [{ "label": "S1", "quote": "Ferritin" }],
                                "cites_none": false,
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

    let chat_claim = store.claim(&chat_scope, 60).await.unwrap().unwrap();
    let mut chat_host = PhysicianHost {
        store: store.clone(),
        claim: chat_claim,
        authority: PostgresWorkspaceAuthority,
        shown_steps: local_product::runtime::ShownSteps::new(()),
    };
    chat_host
        .run(
            &mut ChatReviseFollowUpProvider {
                turn: 0,
                due_at: due_revised,
            },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(
        store.conversation(&chat_scope).await.unwrap().status,
        WorkStatus::Completed
    );

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &physician.physician_id,
        workspace: &workspace,
        session_hash: &session_hash,
    };
    let final_items = store
        .physician_follow_ups(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(final_items.len(), 1);
    assert_eq!(final_items[0].id.as_str(), "fu-prod-ferritin");
    assert_eq!(final_items[0].version, 2);

    let updates = store
        .physician_follow_up_updates(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(updates.len(), 2);
    assert_eq!(updates[0].kind, FollowUpUpdateKind::Created);
    assert_eq!(updates[1].kind, FollowUpUpdateKind::Changed);
    assert!(matches!(updates[0].actor, FollowUpActor::CodaWork { .. }));
    assert!(matches!(updates[1].actor, FollowUpActor::CodaWork { .. }));
}
