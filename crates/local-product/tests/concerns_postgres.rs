//! Real Postgres proof for patient concerns: raising, revising, and retiring
//! through the job tool across restarts, Updates readback, shared-patient
//! `Done` clearing + feedback capture, once-only command idempotency, and
//! panel/practice authority enforcement.

use local_product::{
    physician_sources::{
        ChartRequest, ChartScope, READ_TOOL, RecordDate, SourceRevision, read_tool,
    },
    runtime::{
        Authority, PublicationTransaction, Scope, Store, async_trait,
        concerns::{
            CONCERN_TOOL, CommandKey, ConcernDoneOutcome, ConcernError, ConcernFeedbackAction,
            ConcernId, ConcernMutation, ConcernStanding, ConcernUrgency, ExactRecordReference,
            RaiseConcern, RetireConcern, RetirementReason, ReviseConcern, SourceWork,
            WorkspaceAccess, concern_tool, is_concern_tool,
        },
        engine::{
            self, AcceptedInput, Event, Invocation, Item, Product, ProductMessage, ToolOutcome,
            Turn, WindowSeed,
            recipe::{Finish, Observer, Provider, Request, RequestBinding, RequestRecipe},
        },
    },
    sha256_hex,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use serde_json::{Value, json};

/// Real workspace authority delegating both read checks and write locks to
/// `PostgresAuthStore` on the caller's transaction.
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

fn seeded_lab(
    workspace: &str,
    candidate_id: &str,
    revision: &str,
    date: &str,
    potassium: &str,
) -> SourceRevision {
    let content = format!(
        r#"{{"resourceType":"Observation","id":"{candidate_id}","date":"{date}","potassium_mmol_L":{potassium}}}"#
    );
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "clinic-lab".into(),
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
        provenance: json!({"system": "clinic-lab", "id": candidate_id}),
        envelope: None,
    }
}

/// Minimal harness `Product` host for background patient-review jobs exercising
/// `CONCERN_TOOL` alongside exact chart reads.
struct PatientJobHost {
    store: Store,
    practice: String,
    workspace: String,
    source_work: SourceWork,
    sources: Vec<SourceRevision>,
    events: Vec<Event>,
}

#[async_trait]
impl Product for PatientJobHost {
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
            if is_concern_tool(&typed_call.name) {
                return store
                    .execute_concern_tool(&practice, &workspace, &source_work, &typed_call)
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

async fn run_background_job<P: Provider>(
    store: &Store,
    practice: &str,
    workspace: &str,
    source_work: SourceWork,
    sources: Vec<SourceRevision>,
    provider: &mut P,
) {
    let recipe = RequestRecipe {
        model: "controlled-job".into(),
        context_window_tokens: 200_000,
        max_input_tokens: 150_000,
        options: json!({"effort": "high"}),
        instructions: "Review patient chart updates and keep concerns accurate.".into(),
        tools: vec![
            concern_tool().function(),
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
                        text: "Run patient concern review.".into(),
                    })],
                }],
                source_feedback: None,
            },
        )
        .unwrap();
    let mut host = PatientJobHost {
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

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn raise_revise_and_retire_through_concern_tool_survives_restart_and_populates_updates() {
    struct RaiseJob {
        step: usize,
    }
    impl Provider for RaiseJob {
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
                        // Read seeded exact chart record first, and attempt an unverified record ref to verify refusal.
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
                            id: "raise-bad-ref".into(),
                            name: CONCERN_TOOL.into(),
                            arguments: json!({
                                "operation": "raise",
                                "concern_id": "bad-ref",
                                "urgency": "now",
                                "reason": "Citing missing revision.",
                                "records": [{"candidate_id": "bmp-1", "revision": "nonexistent-rev"}]
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "raise-k-now".into(),
                            name: CONCERN_TOOL.into(),
                            arguments: json!({
                                "operation": "raise",
                                "concern_id": "hyperkalemia-bmp",
                                "urgency": "now",
                                "reason": "Serum potassium 6.2 mmol/L on 2026-11-10 metabolic panel.",
                                "records": [{"candidate_id": "bmp-1", "revision": "r1"}]
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "raise-renal-today".into(),
                            name: CONCERN_TOOL.into(),
                            arguments: json!({
                                "operation": "raise",
                                "concern_id": "acei-renal-followup",
                                "urgency": "today",
                                "reason": "Recheck chemistry panel after lisinopril dose increase.",
                                "records": [{"candidate_id": "bmp-1", "revision": "r1"}]
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"job": "raise", "step": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    // Unverified citation must have been refused without persisting `bad-ref`.
                    let bad_ref_output = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            Item::MaterializedResult { call_id, output }
                                if call_id == "raise-bad-ref" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("bad-ref feedback delivered");
                    assert!(bad_ref_output.contains("not in the workspace's current retained sources"));
                    Ok(Turn {
                        items: vec![Item::assistant("Initial concerns recorded.")],
                        receipt: json!({"job": "raise", "step": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected RaiseJob step {other}"),
            }
        }
    }

    struct ReviseAndRetireJob {
        step: usize,
    }
    impl Provider for ReviseAndRetireJob {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "revise-renal".into(),
                            name: CONCERN_TOOL.into(),
                            arguments: json!({
                                "operation": "revise",
                                "concern_id": "acei-renal-followup",
                                "urgency": "today",
                                "reason": "Repeat chemistry panel confirms potassium normalized; verify creatinine stability in 2 weeks.",
                                "records": [
                                    {"candidate_id": "bmp-1", "revision": "r1"},
                                    {"candidate_id": "bmp-2", "revision": "r2"}
                                ]
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "retire-k".into(),
                            name: CONCERN_TOOL.into(),
                            arguments: json!({
                                "operation": "retire",
                                "concern_id": "hyperkalemia-bmp",
                                "reason": "Repeat serum potassium normalized to 4.3 mmol/L on 2026-11-11 chemistry panel."
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "raise-a1c".into(),
                            name: CONCERN_TOOL.into(),
                            arguments: json!({
                                "operation": "raise",
                                "concern_id": "a1c-routine-recheck",
                                "urgency": "today",
                                "reason": "Schedule routine quarterly chemistry and glycemic follow-up.",
                                "records": [
                                    {"candidate_id": "bmp-2", "revision": "r2"}
                                ]
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"job": "revise-retire", "step": 1}),
                    usage: Default::default(),
                }),
                2 => Ok(Turn {
                    items: vec![Item::assistant("Updated and retired concerns.")],
                    receipt: json!({"job": "revise-retire", "step": 2}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected ReviseAndRetireJob step {other}"),
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
    let (physician, _token, session_hash) = invite_and_sign_in(&auth, &practice, "dr-chen").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&physician, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let lab_1 = seeded_lab(&workspace, "bmp-1", "r1", "2026-11-10", "6.2");
    store.record_source(&lab_1).await.unwrap();

    let work_1 = SourceWork::new("lab_arrival_review", "work-bmp-1", "attempt-1").unwrap();
    run_background_job(
        &store,
        &practice,
        &workspace,
        work_1.clone(),
        vec![lab_1.clone()],
        &mut RaiseJob { step: 0 },
    )
    .await;

    // Before retirement, urgent `Now` concern sorts ahead of `Today` concern.
    let access = WorkspaceAccess {
        practice: &practice,
        physician: &physician.physician_id,
        workspace: &workspace,
        session_hash: &session_hash,
    };
    let open_before = store
        .open_concerns(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(open_before.len(), 2);
    assert_eq!(open_before[0].id.as_str(), "hyperkalemia-bmp");
    assert_eq!(open_before[0].urgency, ConcernUrgency::Now);
    assert_eq!(open_before[1].id.as_str(), "acei-renal-followup");
    assert_eq!(open_before[1].urgency, ConcernUrgency::Today);

    // Follow-up lab arrives on the next clock second (`now()` uses integer epoch seconds);
    // second job revises `acei-renal-followup`, retires `hyperkalemia-bmp`, and raises a newer
    // `Today` concern (`a1c-routine-recheck`, whose ID sorts alphabetically before `acei-renal-followup`).
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    let lab_2 = seeded_lab(&workspace, "bmp-2", "r2", "2026-11-11", "4.3");
    store.record_source(&lab_2).await.unwrap();
    let work_2 = SourceWork::new("lab_arrival_review", "work-bmp-2", "attempt-1").unwrap();
    run_background_job(
        &store,
        &practice,
        &workspace,
        work_2.clone(),
        vec![lab_1, lab_2],
        &mut ReviseAndRetireJob { step: 0 },
    )
    .await;

    // Drop the store completely and reconnect a fresh pool to prove restart persistence.
    drop(store);
    let restarted = Store::connect(&url).await.unwrap();

    let open_after = restarted
        .open_concerns(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(open_after.len(), 2);
    assert!(
        open_after[0].created_at < open_after[1].created_at,
        "Today must order same-urgency concerns by how long each has waited (oldest created_at first)"
    );
    assert_eq!(open_after[1].id.as_str(), "a1c-routine-recheck");
    let revised = &open_after[0];
    assert_eq!(revised.id.as_str(), "acei-renal-followup");
    assert_eq!(revised.urgency, ConcernUrgency::Today);
    assert_eq!(revised.revision, 2);
    assert_eq!(
        revised.reason,
        "Repeat chemistry panel confirms potassium normalized; verify creatinine stability in 2 weeks."
    );
    assert_eq!(
        revised.records,
        vec![
            ExactRecordReference::new("bmp-1", "r1").unwrap(),
            ExactRecordReference::new("bmp-2", "r2").unwrap(),
        ]
    );
    assert_eq!(revised.source_work, work_2);

    // Retired concern no longer appears on Today and surfaces its one-line reason in Updates.
    let updates = restarted
        .retired_concern_updates(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].id.as_str(), "hyperkalemia-bmp");
    assert_eq!(
        updates[0].retirement_reason.as_str(),
        "Repeat serum potassium normalized to 4.3 mmol/L on 2026-11-11 chemistry panel."
    );
    assert_eq!(
        updates[0].concern_reason,
        "Serum potassium 6.2 mmol/L on 2026-11-10 metabolic panel."
    );
    assert_eq!(
        updates[0].records,
        vec![ExactRecordReference::new("bmp-1", "r1").unwrap()]
    );
    assert_eq!(updates[0].raised_by_work, work_1);
    assert_eq!(updates[0].retired_by_work, work_2);
    assert_eq!(updates[0].revision, 2);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn shared_patient_done_clears_concern_for_both_physicians_and_saves_evaluation_feedback() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();

    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let (dr_alpha, _token_a, hash_a) = invite_and_sign_in(&auth, &practice, "dr-alpha").await;
    let (dr_beta, _token_b, hash_b) = invite_and_sign_in(&auth, &practice, "dr-beta").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr_alpha, &workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&dr_beta, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let lab = seeded_lab(&workspace, "bmp-shared-1", "r1", "2026-11-11", "5.9");
    store.record_source(&lab).await.unwrap();

    let origin = SourceWork::new("morning_briefing", "work-shared-1", "attempt-1").unwrap();
    let concern_id = ConcernId::new("shared-hyperkalemia").unwrap();
    store
        .raise_concern(
            &CommandKey::new("raise-shared-k-1").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: concern_id.clone(),
                urgency: ConcernUrgency::Now,
                reason: "Potassium 5.9 mmol/L while taking spironolactone.".into(),
                records: vec![ExactRecordReference::new("bmp-shared-1", "r1").unwrap()],
                source_work: origin.clone(),
            },
        )
        .await
        .unwrap();

    let access_a = WorkspaceAccess {
        practice: &practice,
        physician: &dr_alpha.physician_id,
        workspace: &workspace,
        session_hash: &hash_a,
    };
    let access_b = WorkspaceAccess {
        practice: &practice,
        physician: &dr_beta.physician_id,
        workspace: &workspace,
        session_hash: &hash_b,
    };

    // Both sharing physicians see the open `Now` concern on Today.
    assert_eq!(
        store
            .open_concerns(&access_a, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        store
            .open_concerns(&access_b, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap()
            .len(),
        1
    );

    // Physician A marks the concern Done.
    let done = store
        .mark_concern_done(
            &access_a,
            &CommandKey::new("done-shared-1").unwrap(),
            &concern_id,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    let ConcernDoneOutcome::Cleared { concern, feedback } = done else {
        panic!("expected Cleared outcome, got {done:?}");
    };
    assert_eq!(concern.revision, 2);
    assert!(matches!(
        &concern.standing,
        ConcernStanding::Done { physician, feedback_revision: 1, .. }
            if physician == &dr_alpha.physician_id
    ));
    assert_eq!(feedback.physician, dr_alpha.physician_id);
    assert_eq!(feedback.action, ConcernFeedbackAction::Done);
    assert_eq!(feedback.urgency, ConcernUrgency::Now);
    assert_eq!(feedback.concern_revision, 2);
    assert_eq!(
        feedback.concern_reason,
        "Potassium 5.9 mmol/L while taking spironolactone."
    );
    assert_eq!(
        feedback.records,
        vec![ExactRecordReference::new("bmp-shared-1", "r1").unwrap()]
    );
    assert_eq!(feedback.source_work, origin);

    // Cleared for BOTH physicians on Today, and persisted in evaluation feedback across reconnect.
    drop(store);
    let store = Store::connect(&url).await.unwrap();
    assert!(
        store
            .open_concerns(&access_a, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        store
            .open_concerns(&access_b, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap()
            .is_empty()
    );

    // If Physician B presses Done from a stale view, it reports AlreadyCleared without duplicating feedback.
    let stale_second_done = store
        .mark_concern_done(
            &access_b,
            &CommandKey::new("done-shared-stale-b").unwrap(),
            &concern_id,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        stale_second_done,
        ConcernDoneOutcome::AlreadyCleared { .. }
    ));

    let all_feedback: Vec<_> = store
        .concern_feedback_for_evaluation()
        .await
        .unwrap()
        .into_iter()
        .filter(|entry| entry.practice == practice && entry.workspace == workspace)
        .collect();
    assert_eq!(all_feedback, vec![*feedback]);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn once_only_commands_and_unauthorized_physicians_cannot_read_or_change_concerns() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();

    let practice = uuid::Uuid::new_v4().to_string();
    let other_practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());

    let (authorized, _auth_token, auth_hash) =
        invite_and_sign_in(&auth, &practice, "dr-panel").await;
    let (demographic_only, _demo_token, demo_hash) =
        invite_and_sign_in(&auth, &practice, "dr-demographic").await;
    let (denied_doc, _denied_token, denied_hash) =
        invite_and_sign_in(&auth, &practice, "dr-denied").await;
    let (revoked_doc, revoked_token, revoked_hash) =
        invite_and_sign_in(&auth, &practice, "dr-revoked").await;
    let (foreign_doc, _foreign_token, foreign_hash) =
        invite_and_sign_in(&auth, &other_practice, "dr-foreign").await;

    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&authorized, &workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&demographic_only, &workspace, Membership::ClinicDemographic)
        .await
        .unwrap();
    auth.set_membership(&denied_doc, &workspace, Membership::Denied)
        .await
        .unwrap();
    auth.set_membership(&revoked_doc, &workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.end_session(&revoked_token).await.unwrap();

    let lab = seeded_lab(&workspace, "bmp-guard-1", "r1", "2026-11-11", "6.0");
    store.record_source(&lab).await.unwrap();

    let work = SourceWork::new("chart_review", "work-guard-1", "attempt-1").unwrap();
    let concern_id = ConcernId::new("guarded-concern").unwrap();
    let raise_key = CommandKey::new("cmd-raise-guard").unwrap();
    let raise_payload = RaiseConcern {
        practice: practice.clone(),
        workspace: workspace.clone(),
        id: concern_id.clone(),
        urgency: ConcernUrgency::Now,
        reason: "Potassium 6.0 mmol/L requires review.".into(),
        records: vec![ExactRecordReference::new("bmp-guard-1", "r1").unwrap()],
        source_work: work.clone(),
    };

    let first_raise = store
        .raise_concern(&raise_key, &raise_payload)
        .await
        .unwrap();
    assert!(matches!(first_raise, ConcernMutation::Applied(ref c) if c.revision == 1));

    // Repeated raise command makes no second change; reusing key with different payload collides.
    let second_raise = store
        .raise_concern(&raise_key, &raise_payload)
        .await
        .unwrap();
    assert!(matches!(second_raise, ConcernMutation::Duplicate(ref c) if c.revision == 1));

    let mut colliding_raise = raise_payload.clone();
    colliding_raise.reason = "Different statement under same key.".into();
    assert!(matches!(
        store.raise_concern(&raise_key, &colliding_raise).await,
        Err(ConcernError::IdempotencyCollision { .. })
    ));

    // Repeated revise command increments revision once (1 -> 2, never 3).
    let revise_key = CommandKey::new("cmd-revise-guard").unwrap();
    let revise_payload = ReviseConcern {
        practice: practice.clone(),
        workspace: workspace.clone(),
        id: concern_id.clone(),
        urgency: ConcernUrgency::Now,
        reason: "Potassium 6.0 mmol/L confirmed without hemolysis.".into(),
        records: vec![ExactRecordReference::new("bmp-guard-1", "r1").unwrap()],
        source_work: work.clone(),
    };
    assert!(matches!(
        store
            .revise_concern(&revise_key, &revise_payload)
            .await
            .unwrap(),
        ConcernMutation::Applied(ref c) if c.revision == 2
    ));
    assert!(matches!(
        store
            .revise_concern(&revise_key, &revise_payload)
            .await
            .unwrap(),
        ConcernMutation::Duplicate(ref c) if c.revision == 2
    ));

    // Every unauthorized physician is refused on both read and Done mutation.
    let unauthorized_cases = [
        (
            "clinic-demographic",
            &practice,
            &demographic_only.physician_id,
            &demo_hash[..],
        ),
        (
            "panel-denied",
            &practice,
            &denied_doc.physician_id,
            &denied_hash[..],
        ),
        (
            "revoked-session",
            &practice,
            &revoked_doc.physician_id,
            &revoked_hash[..],
        ),
        (
            "other-practice",
            &other_practice,
            &foreign_doc.physician_id,
            &foreign_hash[..],
        ),
    ];

    for (label, prac, phys, hash) in unauthorized_cases {
        let access = WorkspaceAccess {
            practice: prac,
            physician: phys,
            workspace: &workspace,
            session_hash: hash,
        };
        assert!(
            matches!(
                store
                    .open_concerns(&access, &mut PostgresWorkspaceAuthority)
                    .await,
                Err(ConcernError::AccessDenied { .. })
            ),
            "{label}: open_concerns must refuse unauthorized physician"
        );
        assert!(
            matches!(
                store
                    .retired_concern_updates(&access, &mut PostgresWorkspaceAuthority)
                    .await,
                Err(ConcernError::AccessDenied { .. })
            ),
            "{label}: retired_concern_updates must refuse unauthorized physician"
        );
        assert!(
            matches!(
                store
                    .mark_concern_done(
                        &access,
                        &CommandKey::new(format!("unauth-done-{label}")).unwrap(),
                        &concern_id,
                        &mut PostgresWorkspaceAuthority,
                    )
                    .await,
                Err(ConcernError::AccessDenied { .. })
            ),
            "{label}: mark_concern_done must refuse unauthorized physician"
        );
    }

    // Authorized physician confirm: concern was untouched by unauthorized attempts (still Open at revision 2).
    let auth_access = WorkspaceAccess {
        practice: &practice,
        physician: &authorized.physician_id,
        workspace: &workspace,
        session_hash: &auth_hash,
    };
    let still_open = store
        .open_concerns(&auth_access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(still_open.len(), 1);
    assert_eq!(still_open[0].revision, 2);

    // Authorized physician Done command is once-only: repeating the key keeps revision 3 and feedback revision 1.
    let done_key = CommandKey::new("cmd-done-guard").unwrap();
    let first_done = store
        .mark_concern_done(
            &auth_access,
            &done_key,
            &concern_id,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        first_done,
        ConcernDoneOutcome::Cleared { ref concern, ref feedback }
            if concern.revision == 3 && feedback.revision == 1
    ));

    let repeated_done = store
        .mark_concern_done(
            &auth_access,
            &done_key,
            &concern_id,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        repeated_done,
        ConcernDoneOutcome::Duplicate { ref concern } if concern.revision == 3
    ));

    let other_id = ConcernId::new("different-concern").unwrap();
    assert!(matches!(
        store
            .mark_concern_done(
                &auth_access,
                &done_key,
                &other_id,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ConcernError::IdempotencyCollision { .. })
    ));

    // Repeated retire command is also once-only.
    store
        .raise_concern(
            &CommandKey::new("cmd-raise-retire-guard").unwrap(),
            &RaiseConcern {
                id: other_id.clone(),
                ..raise_payload
            },
        )
        .await
        .unwrap();
    let retire_key = CommandKey::new("cmd-retire-guard").unwrap();
    let retire_payload = RetireConcern {
        practice: practice.clone(),
        workspace: workspace.clone(),
        id: other_id,
        reason: RetirementReason::new("Normalized on repeat chemistry panel.").unwrap(),
        source_work: work,
    };
    assert!(matches!(
        store
            .retire_concern(&retire_key, &retire_payload)
            .await
            .unwrap(),
        ConcernMutation::Applied(ref c) if c.revision == 2
    ));
    assert!(matches!(
        store
            .retire_concern(&retire_key, &retire_payload)
            .await
            .unwrap(),
        ConcernMutation::Duplicate(ref c) if c.revision == 2
    ));
}
