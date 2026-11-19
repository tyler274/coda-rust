//! Unit and real PostgreSQL proof for reply proposals, conversation review
//! fencing, practice-message partial closures, patient/not-a-patient moves,
//! channel disconnects, copy attribution, undo windows, and private per-physician
//! drafts (`verification/physician/message-reply/`).

use std::collections::BTreeSet;
use std::error::Error as _;
use std::path::PathBuf;

use local_product::{
    physician_chat::PhysicianSkills,
    physician_sources::{RecordDate, SourceRevision},
    runtime::{
        Authority, PublicationTransaction, Scope, Store, async_trait,
        concerns::{
            CommandKey, ConcernId, ConcernStanding, ConcernUrgency, ExactRecordReference,
            RaiseConcern, SourceWork, WorkspaceAccess,
        },
        delivery::Delivery,
        engine::{
            Event, EventKind, Invocation, Item, Turn,
            recipe::{RequestRecipe, Usage},
            runs::RunOutcome,
        },
        reply::{
            FINISH_TOOL, RevisionCommit, RevisionInput,
            proposals::{
                AnsweredMessageClosure, AskCodaToReplyOutcome, CardArrivalNotice,
                CardChannelBanner, CardReviewState, ChannelConnection, CompleteReplyReview,
                ConversationLink, ConversationMoveOutcome, CopiedReplySource, CopyReplyOutcome,
                DisplayedReplyText, ExpectedDisconnectedCard, ExpectedDraftBase,
                MessageIntakeOutcome, MessageSender, NoReplyReason, ObservePracticeMessage,
                PatientMessageStanding, PhysicianDraftKind, PhysicianDraftStanding,
                PhysicianReplySurface, PracticeMessageKind, ProposalRevisionAccepted,
                RecordPatientMessage, ReplyProposalBasisInput, ReplyProposalError,
                ReplyProposalInput, ReplyProposalInputError, ReplyReviewDecision,
                ReplyReviewOutcome, ReplyStanding, ReviewReadState, ReviewWakeEvent,
                proposal_draft_scope,
            },
        },
    },
    sha256_hex,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use serde_json::json;

/// Real workspace authority backed by `PostgresAuthStore` on the caller's transaction.
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

/// Authority that denies `check` (reads) while allowing `lock_publication` (writes),
/// proving read projections call `Authority::check` rather than taking publication locks.
struct ReadDeniedAuthority;

#[async_trait]
impl Authority for ReadDeniedAuthority {
    async fn check(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Err("read authority denied".into())
    }

    async fn lock_publication(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
}

fn unix_seconds() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

async fn setup_stores() -> (String, Store, PostgresAuthStore) {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    (url, store, auth)
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

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
        .expect("load physician skills catalog")
}

fn test_revision_recipe() -> RequestRecipe {
    RequestRecipe {
        model: "controlled-reply-revision".into(),
        context_window_tokens: 128_000,
        max_input_tokens: 100_000,
        options: json!({"effort": "low"}),
        instructions: "Revise the physician's reply draft concisely.".into(),
        tools: vec![],
    }
}

fn revision_turn(text: &str, basis: &str) -> Turn {
    Turn {
        items: vec![Item::Call(Invocation {
            id: "finish-rev".into(),
            name: FINISH_TOOL.into(),
            arguments: json!({
                "text": text,
                "review_basis": basis,
            })
            .to_string(),
        })],
        usage: Usage::default(),
        receipt: json!({"synthetic": true}),
    }
}

async fn record_revision_candidate(
    store: &Store,
    claim: &local_product::runtime::reply::RevisionClaim,
    turn: &Turn,
) {
    for (sequence, kind) in [
        (1, EventKind::ProviderReturned(turn.clone())),
        (2, EventKind::Candidate { text: turn.text() }),
    ] {
        store
            .record_revision(
                claim,
                Event {
                    attempt_id: claim.attempt.clone(),
                    sequence,
                    ordinal: 1,
                    kind,
                },
            )
            .await
            .unwrap();
    }
}

fn set_of(ids: &[&str]) -> BTreeSet<String> {
    ids.iter().map(|s| (*s).to_owned()).collect()
}

#[test]
fn typed_domain_helpers_and_error_sources_preserve_underlying_causes() {
    let scope = proposal_draft_scope("prac-1", "dr-1", "ws-1", "conv:abc", 3);
    assert_eq!(scope.chat, "reply-proposal:conv:abc:3");
    assert!(!ReplyStanding::None.is_open());
    assert!(
        ReplyStanding::Withheld {
            reply_id: 2,
            workspace: "ws-2".into(),
            covered_message_ids: set_of(&["m1"]),
            withheld_from_proposal_version: 1,
            move_epoch: 1,
            withheld_at: 100,
        }
        .is_open()
    );

    // Underlying validation cause stays attached via `std::error::Error::source()`.
    let bad_ref = ExactRecordReference::new("  ", "r1").unwrap_err();
    let input_err = ReplyProposalInputError::InvalidRecordReference {
        field: "cited_records",
        cause: bad_ref,
    };
    assert!(input_err.source().is_some());
    let proposal_err = ReplyProposalError::from(input_err);
    assert!(proposal_err.source().is_some());

    // Underlying database error stays attached via `std::error::Error::source()`.
    let db_err = ReplyProposalError::Database(sea_orm::DbErr::Custom("disk offline".into()));
    let cause = db_err.source().expect("DbErr preserved as source");
    assert!(cause.to_string().contains("disk offline"));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac1_saves_proposal_basis_verified_citations_and_survives_restart_without_duplicate() {
    let (url, store, auth) = setup_stores().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let conversation_id = "conv-ac1";

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac1").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let lab = seeded_lab(&workspace, "bmp-ac1", "rev-1", "2026-11-10", "4.3");
    let source_version = store.record_source(&lab).await.unwrap();
    assert_eq!(source_version, 1);

    let linked = store
        .link_reply_conversation(
            &practice,
            conversation_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    assert_eq!(linked.conversation_version, 1);
    // Linking again with the same patient is idempotent; conflicting patient link is refused.
    assert_eq!(
        store
            .link_reply_conversation(
                &practice,
                conversation_id,
                ConversationLink::LinkedPatient {
                    workspace: workspace.clone(),
                },
            )
            .await
            .unwrap(),
        linked
    );
    assert!(matches!(
        store
            .link_reply_conversation(&practice, conversation_id, ConversationLink::NotAPatient)
            .await,
        Err(ReplyProposalError::IdempotencyCollision { .. })
    ));

    let msg_in = RecordPatientMessage {
        practice: practice.clone(),
        conversation_id: conversation_id.into(),
        message_id: "pm-1".into(),
        sender: MessageSender::Patient,
        text: "Did my potassium lab come back normal?".into(),
        sent_at: 1_762_900_000,
    };
    assert!(matches!(
        store.record_linked_patient_message(&msg_in).await.unwrap(),
        MessageIntakeOutcome::Recorded { .. }
    ));

    let snap = store
        .start_or_read_reply_review(&practice, conversation_id, "rev-ac1", "run-ac1-1")
        .await
        .unwrap();
    assert_eq!(snap.source_version, 1);
    assert_eq!(snap.conversation.conversation_version, 2);

    // Conflicting revisions for the same record in basis vs claim citations are refused.
    let conflicting_complete = CompleteReplyReview {
        practice: practice.clone(),
        conversation_id: conversation_id.into(),
        review_id: "rev-ac1".into(),
        run_id: "run-ac1-1".into(),
        completion_key: "comp-conflict".into(),
        answered_closures: vec![],
        decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
            text: "Your potassium is normal at 4.3 mmol/L.".into(),
            basis: ReplyProposalBasisInput {
                summary: "Reviewed 2026-11-10 metabolic panel.".into(),
                limits: vec!["No external hospital labs connected.".into()],
                cited_records: vec![ExactRecordReference::new("bmp-ac1", "missing-rev").unwrap()],
            },
            cited_records: vec![ExactRecordReference::new("bmp-ac1", "rev-1").unwrap()],
        }),
        raised_concern_ids: vec![],
    };
    assert!(matches!(
        store.complete_reply_review(&conflicting_complete).await,
        Err(ReplyProposalError::InvalidInput(
            ReplyProposalInputError::ConflictingRecordRevision { ref candidate_id }
        )) if candidate_id == "bmp-ac1"
    ));

    // Unverified chart citation is refused and leaves the review active without saving a proposal.
    let bad_complete = CompleteReplyReview {
        practice: practice.clone(),
        conversation_id: conversation_id.into(),
        review_id: "rev-ac1".into(),
        run_id: "run-ac1-1".into(),
        completion_key: "comp-bad".into(),
        answered_closures: vec![],
        decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
            text: "Your potassium is normal at 4.3 mmol/L.".into(),
            basis: ReplyProposalBasisInput {
                summary: "Reviewed 2026-11-10 metabolic panel.".into(),
                limits: vec!["No external hospital labs connected.".into()],
                cited_records: vec![ExactRecordReference::new("bmp-ac1", "missing-rev").unwrap()],
            },
            cited_records: vec![ExactRecordReference::new("bmp-ac1", "missing-rev").unwrap()],
        }),
        raised_concern_ids: vec![],
    };
    assert!(matches!(
        store.complete_reply_review(&bad_complete).await,
        Err(ReplyProposalError::UnverifiedCitation {
            ref candidate_id,
            ref revision,
        }) if candidate_id == "bmp-ac1" && revision == "missing-rev"
    ));
    assert!(
        store
            .reply_proposals(&practice, conversation_id)
            .await
            .unwrap()
            .is_empty()
    );

    // Verified proposal saves basis, exact citations, workspace, conversation, review/run IDs, and versions.
    let good_complete = CompleteReplyReview {
        practice: practice.clone(),
        conversation_id: conversation_id.into(),
        review_id: "rev-ac1".into(),
        run_id: "run-ac1-1".into(),
        completion_key: "comp-ac1-1".into(),
        answered_closures: vec![],
        decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
            text: "Your 2026-11-10 metabolic panel shows a normal potassium of 4.3 mmol/L.".into(),
            basis: ReplyProposalBasisInput {
                summary: "Based on 2026-11-10 clinic BMP and today's patient message.".into(),
                limits: vec!["Outside urgent-care labs are not connected.".into()],
                cited_records: vec![ExactRecordReference::new("bmp-ac1", "rev-1").unwrap()],
            },
            cited_records: vec![ExactRecordReference::new("bmp-ac1", "rev-1").unwrap()],
        }),
        raised_concern_ids: vec![],
    };
    let applied = match store.complete_reply_review(&good_complete).await.unwrap() {
        ReplyReviewOutcome::Applied(done) => *done,
        other => panic!("expected Applied, got {other:?}"),
    };
    let saved_proposal = applied.proposal.clone().expect("saved proposal");
    assert_eq!(saved_proposal.reply_id, 1);
    assert_eq!(saved_proposal.version, 1);
    assert_eq!(saved_proposal.workspace, workspace);
    assert_eq!(saved_proposal.conversation_id, conversation_id);
    assert_eq!(saved_proposal.review_id, "rev-ac1");
    assert_eq!(saved_proposal.run_id, "run-ac1-1");
    assert_eq!(saved_proposal.conversation_version, 2);
    assert_eq!(saved_proposal.source_version, 1);
    assert_eq!(saved_proposal.covered_message_ids, set_of(&["pm-1"]));
    assert_eq!(saved_proposal.citations.len(), 1);
    assert_eq!(
        saved_proposal.citations[0].reference.candidate_id,
        "bmp-ac1"
    );
    assert_eq!(saved_proposal.citations[0].reference.revision, "rev-1");
    assert_eq!(
        saved_proposal.basis.limits,
        vec!["Outside urgent-care labs are not connected.".to_owned()]
    );

    // Drop Store, reconnect to Postgres, and verify exact durable readback.
    drop(store);
    let reopened = Store::connect(&url).await.unwrap();
    let proposals = reopened
        .reply_proposals(&practice, conversation_id)
        .await
        .unwrap();
    assert_eq!(proposals, vec![saved_proposal.clone()]);
    let conv_after = reopened
        .reply_conversation(&practice, conversation_id)
        .await
        .unwrap()
        .expect("saved conversation");
    assert_eq!(
        conv_after.reply_standing,
        ReplyStanding::Ready {
            proposal: Box::new(saved_proposal.clone()),
        }
    );

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let card = reopened
        .reply_card(&access, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .expect("open reply card");
    assert_eq!(
        card.displayed_text,
        DisplayedReplyText::CodaProposal {
            reply_id: 1,
            version: 1,
            text: saved_proposal.text.clone(),
        }
    );
    assert_eq!(card.basis, Some(saved_proposal.basis.clone()));
    assert_eq!(card.citations, saved_proposal.citations);

    // Read projection uses `Authority::check`, so ReadDeniedAuthority refuses it cleanly.
    assert!(matches!(
        reopened
            .reply_card(&access, conversation_id, &mut ReadDeniedAuthority)
            .await,
        Err(ReplyProposalError::AccessDenied { .. })
    ));

    // Repeated publication with the same completion key returns Duplicate and creates no second proposal.
    let replayed = reopened
        .complete_reply_review(&good_complete)
        .await
        .unwrap();
    assert_eq!(replayed, ReplyReviewOutcome::Duplicate(Box::new(applied)));
    assert_eq!(
        reopened
            .reply_proposals(&practice, conversation_id)
            .await
            .unwrap()
            .len(),
        1
    );

    // Reusing the same completion key with altered text fails with IdempotencyCollision.
    let mut mutated = good_complete;
    mutated.decision = ReplyReviewDecision::NoReplyWouldHelp {
        reason: "Changed mind".into(),
    };
    assert!(matches!(
        reopened.complete_reply_review(&mutated).await,
        Err(ReplyProposalError::IdempotencyCollision { .. })
    ));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac2_fences_stale_conversation_and_source_reads_and_runs_selected_draft_revision() {
    let (_url, store, auth) = setup_stores().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let conversation_id = "conv-ac2";

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac2").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    store
        .record_source(&seeded_lab(
            &workspace,
            "bmp-ac2",
            "r1",
            "2026-11-10",
            "5.8",
        ))
        .await
        .unwrap();
    store
        .link_reply_conversation(
            &practice,
            conversation_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            message_id: "m1".into(),
            sender: MessageSender::Patient,
            text: "Should I hold my lisinopril after that lab?".into(),
            sent_at: 1_762_900_100,
        })
        .await
        .unwrap();

    let first_read = store
        .start_or_read_reply_review(&practice, conversation_id, "rev-ac2", "run-1")
        .await
        .unwrap();
    assert_eq!(first_read.conversation.conversation_version, 2);
    assert_eq!(first_read.source_version, 1);

    // New patient input arrives mid-review -> stale publication is refused (`StaleConversationRead`).
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            message_id: "m2".into(),
            sender: MessageSender::Patient,
            text: "Also I took spironolactone this morning.".into(),
            sent_at: 1_762_900_120,
        })
        .await
        .unwrap();

    let attempt_publish = |rev_ref: &str, key: &str| CompleteReplyReview {
        practice: practice.clone(),
        conversation_id: conversation_id.into(),
        review_id: "rev-ac2".into(),
        run_id: "run-1".into(),
        completion_key: key.into(),
        answered_closures: vec![],
        decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
            text: "Please hold potassium-raising medications until we speak today.".into(),
            basis: ReplyProposalBasisInput {
                summary: "Elevated potassium with ACEi/MRA context.".into(),
                limits: vec![],
                cited_records: vec![ExactRecordReference::new("bmp-ac2", rev_ref).unwrap()],
            },
            cited_records: vec![ExactRecordReference::new("bmp-ac2", rev_ref).unwrap()],
        }),
        raised_concern_ids: vec![],
    };

    assert!(matches!(
        store
            .complete_reply_review(&attempt_publish("r1", "comp-stale-conv"))
            .await,
        Err(ReplyProposalError::StaleConversationRead {
            read_version: Some(2),
            current_version: 3,
            ..
        })
    ));

    // Review re-reads conversation (version 3, source 1), then chart lab is corrected to r2!
    let second_read = store
        .start_or_read_reply_review(&practice, conversation_id, "rev-ac2", "run-1")
        .await
        .unwrap();
    assert_eq!(second_read.conversation.conversation_version, 3);
    assert_eq!(second_read.source_version, 1);

    store
        .record_source(&seeded_lab(
            &workspace,
            "bmp-ac2",
            "r2",
            "2026-11-10",
            "4.5",
        ))
        .await
        .unwrap();

    // Publication pinned to source version 1 is refused (`StaleSourceObservation`).
    assert!(matches!(
        store
            .complete_reply_review(&attempt_publish("r1", "comp-stale-src"))
            .await,
        Err(ReplyProposalError::StaleSourceObservation {
            read_source_version: 1,
            current_source_version: 2,
            ..
        })
    ));
    assert!(
        store
            .reply_proposals(&practice, conversation_id)
            .await
            .unwrap()
            .is_empty()
    );

    // Re-reading pins source version 2 (`r2`) and publishes cleanly.
    let third_read = store
        .start_or_read_reply_review(&practice, conversation_id, "rev-ac2", "run-1")
        .await
        .unwrap();
    assert_eq!(third_read.source_version, 2);
    store
        .complete_reply_review(&attempt_publish("r2", "comp-fresh"))
        .await
        .unwrap();

    // Physician edits draft twice so selected draft revision is 2 (with specific wording).
    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let edit_1 = store
        .edit_reply_proposal_draft(
            &access,
            conversation_id,
            ExpectedDraftBase::ProposalVersion(1),
            "First physician edit.",
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        edit_1.displayed_text,
        DisplayedReplyText::PhysicianDraft {
            draft_revision: 1,
            kind: PhysicianDraftKind::YourText,
            ..
        }
    ));

    let edit_2 = store
        .edit_reply_proposal_draft(
            &access,
            conversation_id,
            ExpectedDraftBase::DraftRevision(1),
            "Corrected BMP potassium is 4.5 mmol/L (hemolyzed earlier sample); continue regimen.",
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        edit_2.displayed_text,
        DisplayedReplyText::PhysicianDraft {
            draft_revision: 2,
            kind: PhysicianDraftKind::YourText,
            ..
        }
    ));

    // Requesting a revision against stale draft revision 1 fails (`StaleDraftVersion`).
    let skills = load_skills();
    let recipe = test_revision_recipe();
    assert!(matches!(
        store
            .request_reply_draft_revision(
                &access,
                conversation_id,
                &RevisionInput {
                    key: "rev-stale".into(),
                    expected_revision: 1,
                    instruction: "Make warmer".into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                &skills,
                &recipe,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));

    // Requesting against selected revision 2 succeeds; repeating the same command key returns
    // `Duplicate` and preserves subsequent physician edits if replayed!
    let rev_input = RevisionInput {
        key: "rev-cmd-2".into(),
        expected_revision: 2,
        instruction: "Make warmer and ask them to message if dizzy.".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    let accepted = store
        .request_reply_draft_revision(
            &access,
            conversation_id,
            &rev_input,
            &skills,
            &recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    let draft_scope = match accepted {
        ProposalRevisionAccepted::New { scope } => scope,
        other => panic!("expected New, got {other:?}"),
    };
    assert_eq!(
        store
            .request_reply_draft_revision(
                &access,
                conversation_id,
                &rev_input,
                &skills,
                &recipe,
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap(),
        ProposalRevisionAccepted::Duplicate {
            scope: draft_scope.clone(),
        }
    );

    // Claim and commit the revision; verify `CodaRevision` displays and `Restore my version`
    // restores the physician's exact revision-2 `YourText`.
    let delivery = Delivery::reply_revision(draft_scope.clone(), "rev-cmd-2".into());
    let claim = store.claim_revision(&delivery, 60).await.unwrap().unwrap();
    assert_eq!(claim.accepted.input, rev_input);
    let saved_draft = store.draft(&draft_scope).await.unwrap().unwrap();
    assert_eq!(
        saved_draft.base_text,
        "Corrected BMP potassium is 4.5 mmol/L (hemolyzed earlier sample); continue regimen."
    );

    let turn = revision_turn(
        "Good news: repeat lab shows normal potassium (4.5 mmol/L). Continue your medications and message us if you feel dizzy.",
        "Applied physician warmth + dizziness guidance on corrected BMP r2.",
    );
    record_revision_candidate(&store, &claim, &turn).await;
    assert_eq!(
        store
            .commit_revision(&claim, &turn, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap(),
        RevisionCommit::Committed
    );

    let revised_card = store
        .reply_card(&access, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        revised_card.displayed_text,
        DisplayedReplyText::PhysicianDraft {
            reply_id: 1,
            draft_revision: 3,
            kind: PhysicianDraftKind::CodaRevision {
                pre_revision_your_text:
                    "Corrected BMP potassium is 4.5 mmol/L (hemolyzed earlier sample); continue regimen."
                        .into(),
            },
            text: "Good news: repeat lab shows normal potassium (4.5 mmol/L). Continue your medications and message us if you feel dizzy.".into(),
        }
    );

    // Replay of initial `rev_input` after commit still returns `Duplicate` and preserves the draft!
    assert_eq!(
        store
            .request_reply_draft_revision(
                &access,
                conversation_id,
                &rev_input,
                &skills,
                &recipe,
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap(),
        ProposalRevisionAccepted::Duplicate {
            scope: draft_scope.clone(),
        }
    );

    // Stale `restore_my_reply_version` fence fails without writing a receipt.
    assert!(matches!(
        store
            .restore_my_reply_version(
                &access,
                &CommandKey::new("restore-ac2").unwrap(),
                conversation_id,
                2,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));

    let restored_card = store
        .restore_my_reply_version(
            &access,
            &CommandKey::new("restore-ac2").unwrap(),
            conversation_id,
            3,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(
        restored_card.displayed_text,
        DisplayedReplyText::PhysicianDraft {
            reply_id: 1,
            draft_revision: 4,
            kind: PhysicianDraftKind::YourText,
            text: "Corrected BMP potassium is 4.5 mmol/L (hemolyzed earlier sample); continue regimen."
                .into(),
        }
    );
    // Lost-receipt retry with the same command key and expected revision succeeds idempotently
    // even after `draft_revision` advanced to 4 and `kind` returned to `YourText`.
    let replayed_restore = store
        .restore_my_reply_version(
            &access,
            &CommandKey::new("restore-ac2").unwrap(),
            conversation_id,
            3,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(
        replayed_restore.displayed_text,
        restored_card.displayed_text
    );

    // `Store::open_draft_authorized` refuses `reply-proposal:` scopes, while `Store::save_draft`
    // synchronizes `reply_draft_binding` and accepts CRLF text on an active `Ready` binding.
    assert!(
        store
            .open_draft_authorized(
                &draft_scope,
                "Bypass proposal draft owner",
                &dr_hash,
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .is_err()
    );
    let saved_rev = store
        .save_draft(
            &draft_scope,
            4,
            "Line 1 via save_draft.\r\nLine 2 via save_draft.",
            &dr_hash,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(saved_rev, 5);
    let synced_card = store
        .reply_card(&access, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        synced_card.displayed_text,
        DisplayedReplyText::PhysicianDraft {
            reply_id: 1,
            draft_revision: 5,
            kind: PhysicianDraftKind::YourText,
            text: "Line 1 via save_draft.\r\nLine 2 via save_draft.".into(),
        }
    );

    store
        .copy_reply(
            &access,
            &CommandKey::new("copy-eval-ac5").unwrap(),
            conversation_id,
            ExpectedDraftBase::DraftRevision(5),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    let exported = store.reply_feedback_for_evaluation().await.unwrap();
    let row = exported
        .iter()
        .find(|entry| {
            entry.practice == practice
                && entry.conversation_id == conversation_id
                && entry.physician == dr.physician_id
        })
        .expect("exported reply feedback row");
    assert!(row.proposed_text.is_some());
    assert_eq!(
        row.revised_text.as_deref(),
        Some("Line 1 via save_draft.\r\nLine 2 via save_draft.")
    );
    assert_eq!(row.draft_revision, Some(5));
    assert!(
        row.revision_instructions
            .contains(&"Make warmer and ask them to message if dizzy.".to_owned()),
        "{:?}",
        row.revision_instructions
    );
    assert_eq!(
        row.copied_texts,
        vec!["Line 1 via save_draft.\r\nLine 2 via save_draft."]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac3_practice_message_partial_closures_both_singletons_automatic_refusal_history_and_concerns()
 {
    let (_url, store, auth) = setup_stores().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac3").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    store
        .record_source(&seeded_lab(
            &workspace,
            "bmp-ac3",
            "r1",
            "2026-11-11",
            "6.1",
        ))
        .await
        .unwrap();

    // Raise an open patient concern and attach it to conversation A (`Rule 3: ConcernStays`).
    let concern_id = ConcernId::new("concern-ac3-k").unwrap();
    store
        .raise_concern(
            &CommandKey::new("raise-ac3-k").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: concern_id.clone(),
                urgency: ConcernUrgency::Now,
                reason: "Potassium 6.1 mmol/L requires clinical action.".into(),
                records: vec![ExactRecordReference::new("bmp-ac3", "r1").unwrap()],
                source_work: SourceWork::new("message_review", "rev-work", "att-1").unwrap(),
            },
        )
        .await
        .unwrap();

    // --- Scenario A: 2 patient messages arrive before 1 practice message; partial closure `{m1}` ---
    let conv_a = "conv-ac3-singleton-1";
    store
        .link_reply_conversation(
            &practice,
            conv_a,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .attach_reply_conversation_concern(&practice, conv_a, &concern_id)
        .await
        .unwrap();
    for (id, text, sent_at) in [
        ("m1", "Can you send my lab order to Quest?", 1_762_901_000),
        ("m2", "Also my chest feels fluttery tonight.", 1_762_901_030),
    ] {
        store
            .record_linked_patient_message(&RecordPatientMessage {
                practice: practice.clone(),
                conversation_id: conv_a.into(),
                message_id: id.into(),
                sender: MessageSender::Patient,
                text: text.into(),
                sent_at,
            })
            .await
            .unwrap();
    }

    // Observe an automatic responder AND a human practice message (`Rule 2`, `Rule 8`).
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_a.into(),
            practice_message_id: "auto-after-hours".into(),
            kind: PracticeMessageKind::Automatic,
            text: "Auto-reply: For emergencies call 911.".into(),
            sent_at: 1_762_901_040,
        })
        .await
        .unwrap();
    let observed = match store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_a.into(),
            practice_message_id: "prac-quest".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "Sent your Quest order over just now.".into(),
            sent_at: 1_762_901_060,
        })
        .await
        .unwrap()
    {
        MessageIntakeOutcome::Recorded { conversation } => *conversation,
        other => panic!("expected Recorded, got {other:?}"),
    };
    // Observing practice messages NEVER marks open messages Answered before review (`Rule 2`).
    assert!(
        observed
            .patient_messages
            .iter()
            .all(|m| m.standing.is_open())
    );

    let snap_a = store
        .start_or_read_reply_review(&practice, conv_a, "rev-a1", "run-a1")
        .await
        .unwrap();
    // `Rule 8: HistoryTrue` — recorded practice history holds exact channel words!
    assert_eq!(snap_a.conversation.recorded_practice_history.len(), 2);
    assert_eq!(
        snap_a.conversation.recorded_practice_history[1].text,
        "Sent your Quest order over just now."
    );

    // Automatic practice reply is refused as an answer (`AutomaticPracticeMessageCannotAnswer`).
    assert!(matches!(
        store
            .complete_reply_review(&CompleteReplyReview {
                practice: practice.clone(),
                conversation_id: conv_a.into(),
                review_id: "rev-a1".into(),
                run_id: "run-a1".into(),
                completion_key: "comp-auto-refused".into(),
                answered_closures: vec![AnsweredMessageClosure {
                    practice_message_id: "auto-after-hours".into(),
                    answered_message_ids: set_of(&["m1"]),
                }],
                decision: ReplyReviewDecision::NoReplyWouldHelp {
                    reason: "Done".into(),
                },
                raised_concern_ids: vec![],
            })
            .await,
        Err(ReplyProposalError::AutomaticPracticeMessageCannotAnswer { .. })
    ));

    // Unread/unknown practice message ID is refused (`UnreadPracticeMessage`).
    assert!(matches!(
        store
            .complete_reply_review(&CompleteReplyReview {
                practice: practice.clone(),
                conversation_id: conv_a.into(),
                review_id: "rev-a1".into(),
                run_id: "run-a1".into(),
                completion_key: "comp-unread-refused".into(),
                answered_closures: vec![AnsweredMessageClosure {
                    practice_message_id: "prac-never-sent".into(),
                    answered_message_ids: set_of(&["m1"]),
                }],
                decision: ReplyReviewDecision::NoReplyWouldHelp {
                    reason: "Done".into(),
                },
                raised_concern_ids: vec![],
            })
            .await,
        Err(ReplyProposalError::UnreadPracticeMessage { .. })
    ));

    // `AllHandled` when `m2` remains open is refused (`OpenMessagesRemainUnhandled`, `Rule 9`).
    assert!(matches!(
        store
            .complete_reply_review(&CompleteReplyReview {
                practice: practice.clone(),
                conversation_id: conv_a.into(),
                review_id: "rev-a1".into(),
                run_id: "run-a1".into(),
                completion_key: "comp-unhandled-refused".into(),
                answered_closures: vec![AnsweredMessageClosure {
                    practice_message_id: "prac-quest".into(),
                    answered_message_ids: set_of(&["m1"]),
                }],
                decision: ReplyReviewDecision::AllHandled,
                raised_concern_ids: vec![],
            })
            .await,
        Err(ReplyProposalError::OpenMessagesRemainUnhandled)
    ));

    // Partial closure of singleton `{m1}` while proposing an urgent reply for open `{m2}`!
    let outcome_a = match store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_a.into(),
            review_id: "rev-a1".into(),
            run_id: "run-a1".into(),
            completion_key: "comp-partial-m1".into(),
            answered_closures: vec![AnsweredMessageClosure {
                practice_message_id: "prac-quest".into(),
                answered_message_ids: set_of(&["m1"]),
            }],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Because your potassium was 6.1 mmol/L and your chest feels fluttery, please go to the emergency department now.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Potassium 6.1 mmol/L with palpitations; Quest request m1 already answered by prac-quest.".into(),
                    limits: vec![],
                    cited_records: vec![ExactRecordReference::new("bmp-ac3", "r1").unwrap()],
                },
                cited_records: vec![ExactRecordReference::new("bmp-ac3", "r1").unwrap()],
            }),
            raised_concern_ids: vec![concern_id.clone()],
        })
        .await
        .unwrap()
    {
        ReplyReviewOutcome::Applied(done) => *done,
        other => panic!("expected Applied, got {other:?}"),
    };
    assert_eq!(outcome_a.newly_answered_message_ids, set_of(&["m1"]));
    assert_eq!(
        outcome_a.proposal.as_ref().unwrap().covered_message_ids,
        set_of(&["m2"])
    );

    let conv_a_state = store
        .reply_conversation(&practice, conv_a)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        &conv_a_state.patient_messages[0].standing,
        PatientMessageStanding::Answered { practice_message_id, .. } if practice_message_id == "prac-quest"
    ));
    assert_eq!(
        conv_a_state.patient_messages[1].standing,
        PatientMessageStanding::Open
    );

    // Copy reply AND close reply via a second practice message answering `m2`:
    // verify `Rule 3: ConcernStays` (concern stays Open!) and `Rule 8: HistoryTrue`
    // (practice sent slightly different wording than Coda's proposal, and history records the channel's words!).
    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let card_a = store
        .reply_card(&access, conv_a, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(card_a.practice_messages.len(), 2);
    assert_eq!(
        card_a.practice_messages[1].practice_message_id,
        "prac-quest"
    );
    store
        .copy_reply(
            &access,
            &CommandKey::new("copy-ac3").unwrap(),
            conv_a,
            ExpectedDraftBase::ProposalVersion(1),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    // `Rule 8: HistoryTrue` + untrimmed `\r\n` channel words: exact words from the channel
    // (including CRLF and leading/trailing spaces) are preserved on `practice_messages`,
    // `NoOpenCard`, and `recorded_practice_history`!
    let exact_prac_words =
        "  Dr. AC3 here — head to the nearest ER right away.\r\nBring your medication list.  ";
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_a.into(),
            practice_message_id: "prac-ed-now".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: exact_prac_words.into(),
            sent_at: 1_762_901_200,
        })
        .await
        .unwrap();
    let card_a_before_review = store
        .reply_card(&access, conv_a, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        card_a_before_review.practice_messages.last().unwrap().text,
        exact_prac_words
    );
    let snap_a2 = store
        .start_or_read_reply_review(&practice, conv_a, "rev-a2", "run-a2")
        .await
        .unwrap();
    assert_eq!(
        snap_a2
            .conversation
            .recorded_practice_history
            .last()
            .unwrap()
            .text,
        exact_prac_words
    );
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_a.into(),
            review_id: "rev-a2".into(),
            run_id: "run-a2".into(),
            completion_key: "comp-close-m2".into(),
            answered_closures: vec![AnsweredMessageClosure {
                practice_message_id: "prac-ed-now".into(),
                answered_message_ids: set_of(&["m2"]),
            }],
            decision: ReplyReviewDecision::AllHandled,
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    let concerns_after = store
        .workspace_concerns(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(concerns_after.len(), 1);
    assert_eq!(concerns_after[0].standing, ConcernStanding::Open);

    // --- Scenario B: Symmetric singleton partial closure `{m2}` leaving `{m1}` open ---
    let conv_b = "conv-ac3-singleton-2";
    store
        .link_reply_conversation(
            &practice,
            conv_b,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    for (id, text, sent_at) in [
        ("m1", "Thanks for seeing me yesterday!", 1_762_902_000),
        ("m2", "Can I take ibuprofen with dinner?", 1_762_902_020),
    ] {
        store
            .record_linked_patient_message(&RecordPatientMessage {
                practice: practice.clone(),
                conversation_id: conv_b.into(),
                message_id: id.into(),
                sender: MessageSender::Patient,
                text: text.into(),
                sent_at,
            })
            .await
            .unwrap();
    }
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_b.into(),
            practice_message_id: "prac-ibuprofen".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "Please avoid ibuprofen given your kidney labs; use acetaminophen instead."
                .into(),
            sent_at: 1_762_902_050,
        })
        .await
        .unwrap();
    store
        .start_or_read_reply_review(&practice, conv_b, "rev-b1", "run-b1")
        .await
        .unwrap();
    let outcome_b = match store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_b.into(),
            review_id: "rev-b1".into(),
            run_id: "run-b1".into(),
            completion_key: "comp-partial-m2".into(),
            answered_closures: vec![AnsweredMessageClosure {
                practice_message_id: "prac-ibuprofen".into(),
                answered_message_ids: set_of(&["m2"]),
            }],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "It was great seeing you yesterday too! Let us know how acetaminophen works."
                    .into(),
                basis: ReplyProposalBasisInput {
                    summary:
                        "Practice message prac-ibuprofen answered m2; proposing reply for open m1."
                            .into(),
                    limits: vec![],
                    cited_records: vec![],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap()
    {
        ReplyReviewOutcome::Applied(done) => *done,
        other => panic!("expected Applied, got {other:?}"),
    };
    assert_eq!(outcome_b.newly_answered_message_ids, set_of(&["m2"]));
    assert_eq!(
        outcome_b.proposal.as_ref().unwrap().covered_message_ids,
        set_of(&["m1"])
    );
    let conv_b_state = store
        .reply_conversation(&practice, conv_b)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        conv_b_state.patient_messages[0].standing,
        PatientMessageStanding::Open
    );
    assert!(matches!(
        &conv_b_state.patient_messages[1].standing,
        PatientMessageStanding::Answered { practice_message_id, .. } if practice_message_id == "prac-ibuprofen"
    ));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac4_moves_withhold_reply_fence_old_work_retire_not_patient_concerns_and_handle_disconnect()
{
    let (_url, store, auth) = setup_stores().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let ws_a = format!("pw-a-{}", uuid::Uuid::new_v4().simple());
    let ws_b = format!("pw-b-{}", uuid::Uuid::new_v4().simple());

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac4").await;
    let (dr_peer, _peer_tok, peer_hash) = invite_and_sign_in(&auth, &practice, "dr-ac4-peer").await;
    for ws in [&ws_a, &ws_b] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
        auth.set_membership(&dr_peer, ws, Membership::Allowed)
            .await
            .unwrap();
    }
    store
        .record_source(&seeded_lab(&ws_a, "bmp-a", "r1", "2026-11-10", "4.1"))
        .await
        .unwrap();
    store
        .record_source(&seeded_lab(&ws_b, "bmp-b", "r1", "2026-11-11", "4.9"))
        .await
        .unwrap();

    // Conversation 1 starts on `ws_a`: `m0` gets answered, `m1` gets an open Ready reply (`reply_id = 1`).
    let conv_move = "conv-ac4-patient-move";
    store
        .link_reply_conversation(
            &practice,
            conv_move,
            ConversationLink::LinkedPatient {
                workspace: ws_a.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_move.into(),
            message_id: "m0".into(),
            sender: MessageSender::Patient,
            text: "What are clinic hours?".into(),
            sent_at: 1_762_903_000,
        })
        .await
        .unwrap();
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_move.into(),
            practice_message_id: "p0".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "8am to 5pm Monday through Friday.".into(),
            sent_at: 1_762_903_010,
        })
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_move.into(),
            message_id: "m1".into(),
            sender: MessageSender::Patient,
            text: "How were my morning labs?".into(),
            sent_at: 1_762_903_020,
        })
        .await
        .unwrap();

    store
        .start_or_read_reply_review(&practice, conv_move, "rev-pre-move-1", "run-pre-1")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_move.into(),
            review_id: "rev-pre-move-1".into(),
            run_id: "run-pre-1".into(),
            completion_key: "comp-pre-1".into(),
            answered_closures: vec![AnsweredMessageClosure {
                practice_message_id: "p0".into(),
                answered_message_ids: set_of(&["m0"]),
            }],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Patient A labs look normal (4.1 mmol/L).".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Patient A chart BMP.".into(),
                    limits: vec![],
                    cited_records: vec![ExactRecordReference::new("bmp-a", "r1").unwrap()],
                },
                cited_records: vec![ExactRecordReference::new("bmp-a", "r1").unwrap()],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    // Physician edits private draft on `(ws_a, reply_id = 1)` and starts an in-flight revision;
    // meanwhile a second message triggers an in-flight pre-move review on `ws_a`.
    let access_a = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &ws_a,
        session_hash: &dr_hash,
    };
    store
        .edit_reply_proposal_draft(
            &access_a,
            conv_move,
            ExpectedDraftBase::ProposalVersion(1),
            "Dr private draft written for Patient A.",
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    // `dr` copies `reply_id = 1` on `ws_a` before moving to `ws_b`; that pre-move copy must
    // NEVER leak into `other_physician_copies` on `ws_b`'s `Withheld` or `Ready` cards!
    store
        .copy_reply(
            &access_a,
            &CommandKey::new("copy-pre-move-a").unwrap(),
            conv_move,
            ExpectedDraftBase::DraftRevision(1),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    let skills = load_skills();
    let recipe = test_revision_recipe();
    let pre_move_rev = match store
        .request_reply_draft_revision(
            &access_a,
            conv_move,
            &RevisionInput {
                key: "rev-before-move".into(),
                expected_revision: 1,
                instruction: "Shorten for Patient A".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            &skills,
            &recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap()
    {
        ProposalRevisionAccepted::New { scope } => scope,
        other => panic!("expected New, got {other:?}"),
    };
    let pre_move_delivery =
        Delivery::reply_revision(pre_move_rev.clone(), "rev-before-move".into());
    let pre_move_claim = store
        .claim_revision(&pre_move_delivery, 60)
        .await
        .unwrap()
        .unwrap();
    let pre_move_turn = revision_turn("Shortened Patient A draft.", "Pre-move basis");
    record_revision_candidate(&store, &pre_move_claim, &pre_move_turn).await;

    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_move.into(),
            message_id: "m2".into(),
            sender: MessageSender::Patient,
            text: "Checking in again.".into(),
            sent_at: 1_762_903_050,
        })
        .await
        .unwrap();
    store
        .start_or_read_reply_review(&practice, conv_move, "rev-inflight-a", "run-inflight-a")
        .await
        .unwrap();

    // Move conversation from `ws_a` -> `ws_b`!
    let moved = match store
        .act_on_conversation_move(
            &CommandKey::new("move-a-to-b").unwrap(),
            &practice,
            conv_move,
            ConversationLink::LinkedPatient {
                workspace: ws_b.clone(),
            },
        )
        .await
        .unwrap()
    {
        ConversationMoveOutcome::MovedToPatient { conversation } => *conversation,
        other => panic!("expected MovedToPatient, got {other:?}"),
    };
    assert_eq!(moved.move_epoch, 1);
    assert!(moved.active_review.is_none());
    assert_eq!(
        moved.woke_events,
        BTreeSet::from([ReviewWakeEvent::MovedToPatient { move_epoch: 1 }])
    );
    // Already-handled `m0` stays `Answered`; open messages stay `Open`; reply is `Withheld` (`reply_id = 2`).
    assert!(matches!(
        moved.patient_messages[0].standing,
        PatientMessageStanding::Answered { .. }
    ));
    assert_eq!(
        moved.patient_messages[1].standing,
        PatientMessageStanding::Open
    );
    assert!(matches!(
        moved.reply_standing,
        ReplyStanding::Withheld {
            reply_id: 2,
            ref workspace,
            withheld_from_proposal_version: 1,
            move_epoch: 1,
            ..
        } if workspace == &ws_b
    ));

    // Late pre-move review completion (`rev-inflight-a`) is fenced and rejected (`StaleReview`).
    assert!(matches!(
        store
            .complete_reply_review(&CompleteReplyReview {
                practice: practice.clone(),
                conversation_id: conv_move.into(),
                review_id: "rev-inflight-a".into(),
                run_id: "run-inflight-a".into(),
                completion_key: "comp-stale-after-move".into(),
                answered_closures: vec![],
                decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                    text: "Wrong-patient late proposal".into(),
                    basis: ReplyProposalBasisInput {
                        summary: "Stale".into(),
                        limits: vec![],
                        cited_records: vec![],
                    },
                    cited_records: vec![],
                }),
                raised_concern_ids: vec![],
            })
            .await,
        Err(ReplyProposalError::StaleReview { .. })
    ));

    // Late pre-move draft revision commit (`pre_move_claim`) is cancelled and fenced!
    assert!(
        store
            .commit_revision(
                &pre_move_claim,
                &pre_move_turn,
                &mut PostgresWorkspaceAuthority
            )
            .await
            .is_err()
    );
    let old_draft = store.draft(&pre_move_rev).await.unwrap().unwrap();
    // Private draft on `(ws_a, reply_id = 1)` keeps the physician's text untouched (`Rule 4`)
    // and its cancelled run outcome is recorded cleanly.
    assert_eq!(old_draft.text, "Dr private draft written for Patient A.");
    assert_eq!(
        old_draft.runs.current().unwrap().outcome,
        Some(RunOutcome::Cancelled)
    );

    // `Store::save_draft` and `Store::accept_revision` on the old `ws_a` proposal scope are fenced after move.
    assert!(
        store
            .save_draft(
                &pre_move_rev,
                1,
                "Late save after move",
                &dr_hash,
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .is_err()
    );
    assert_eq!(
        store
            .accept_revision(
                &pre_move_rev,
                &RevisionInput {
                    key: "rev-after-move-fenced".into(),
                    expected_revision: 1,
                    instruction: "Late revise after move".into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                dr_hash.clone(),
                &skills,
                &recipe,
                None,
            )
            .await
            .unwrap(),
        local_product::runtime::reply::RevisionAccepted::Refused
    );

    // On `ws_b`, the card shows `DisplayedReplyText::Withheld` (never Patient A's text, draft, or copy!).
    let access_b = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &ws_b,
        session_hash: &dr_hash,
    };
    let peer_access_b = WorkspaceAccess {
        practice: &practice,
        physician: &dr_peer.physician_id,
        workspace: &ws_b,
        session_hash: &peer_hash,
    };
    let withheld_card = store
        .reply_card(&peer_access_b, conv_move, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(withheld_card.reply_id, 2);
    assert_eq!(withheld_card.displayed_text, DisplayedReplyText::Withheld);
    assert!(withheld_card.other_physician_copies.is_empty());
    assert_eq!(
        withheld_card.draft_standing,
        PhysicianDraftStanding::NoDraft
    );
    assert_eq!(withheld_card.review_state, CardReviewState::Reviewing);

    // `Rule 11: NoteWhileDisconnected` also holds while a reply is `Withheld`!
    store
        .set_reply_channel_connection(
            &practice,
            conv_move,
            ChannelConnection::Disconnected {
                disconnected_at: 1_762_903_080,
                note: "Disconnected during withheld review.".into(),
            },
        )
        .await
        .unwrap();
    let withheld_disconnected = store
        .reply_card(&access_b, conv_move, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        withheld_disconnected.channel_banner,
        CardChannelBanner::Disconnected {
            disconnected_at: 1_762_903_080,
            note: "Disconnected during withheld review.".into(),
        }
    );
    store
        .set_reply_channel_connection(&practice, conv_move, ChannelConnection::Connected)
        .await
        .unwrap();

    // New review for `ws_b` publishes `Ready` `reply_id = 2, version = 2` grounded in Patient B's chart!
    store
        .start_or_read_reply_review(&practice, conv_move, "rev-b-after-move", "run-b-1")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_move.into(),
            review_id: "rev-b-after-move".into(),
            run_id: "run-b-1".into(),
            completion_key: "comp-b-fresh".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Patient B morning potassium is 4.9 mmol/L (within normal range).".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Patient B chart BMP.".into(),
                    limits: vec![],
                    cited_records: vec![ExactRecordReference::new("bmp-b", "r1").unwrap()],
                },
                cited_records: vec![ExactRecordReference::new("bmp-b", "r1").unwrap()],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    let ready_b_card = store
        .reply_card(&peer_access_b, conv_move, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        ready_b_card.displayed_text,
        DisplayedReplyText::CodaProposal {
            reply_id: 2,
            version: 2,
            text: "Patient B morning potassium is 4.9 mmol/L (within normal range).".into(),
        }
    );
    assert!(ready_b_card.other_physician_copies.is_empty());

    // --- Channel disconnect, retain draft, reconnect, and `Done` while disconnected (`Rule 11`) ---
    // Marking `Done` while still connected is refused (`ChannelStillConnected`).
    assert!(matches!(
        store
            .mark_disconnected_reply_done(
                &access_b,
                &CommandKey::new("done-too-early").unwrap(),
                conv_move,
                ExpectedDisconnectedCard::Ready {
                    reply_id: 2,
                    proposal_version: 2,
                },
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::ChannelStillConnected { .. })
    ));

    store
        .edit_reply_proposal_draft(
            &access_b,
            conv_move,
            ExpectedDraftBase::ProposalVersion(2),
            "Patient B custom physician draft kept across disconnect.",
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    store
        .set_reply_channel_connection(
            &practice,
            conv_move,
            ChannelConnection::Disconnected {
                disconnected_at: 1_762_903_200,
                note: "Spruce webhook disconnected; reconnect in Settings.".into(),
            },
        )
        .await
        .unwrap();
    let disconnected_card = store
        .reply_card(&access_b, conv_move, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        disconnected_card.channel_banner,
        CardChannelBanner::Disconnected {
            disconnected_at: 1_762_903_200,
            note: "Spruce webhook disconnected; reconnect in Settings.".into(),
        }
    );
    assert!(matches!(
        disconnected_card.displayed_text,
        DisplayedReplyText::PhysicianDraft { ref text, .. }
            if text == "Patient B custom physician draft kept across disconnect."
    ));

    // Reconnecting clears the banner while retaining the reply and draft; disconnecting again allows `Done`.
    store
        .set_reply_channel_connection(&practice, conv_move, ChannelConnection::Connected)
        .await
        .unwrap();
    let reconnected_card = store
        .reply_card(&access_b, conv_move, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        reconnected_card.channel_banner,
        CardChannelBanner::Connected
    );

    store
        .set_reply_channel_connection(
            &practice,
            conv_move,
            ChannelConnection::Disconnected {
                disconnected_at: 1_762_903_300,
                note: String::new(),
            },
        )
        .await
        .unwrap();
    // Stale `Withheld` or wrong `proposal_version` fences on `mark_disconnected_reply_done` fail cleanly!
    assert!(matches!(
        store
            .mark_disconnected_reply_done(
                &access_b,
                &CommandKey::new("done-while-disconnected").unwrap(),
                conv_move,
                ExpectedDisconnectedCard::Withheld { reply_id: 2 },
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));
    assert!(matches!(
        store
            .mark_disconnected_reply_done(
                &access_b,
                &CommandKey::new("done-while-disconnected").unwrap(),
                conv_move,
                ExpectedDisconnectedCard::Ready {
                    reply_id: 2,
                    proposal_version: 1,
                },
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));

    let done_conv = store
        .mark_disconnected_reply_done(
            &access_b,
            &CommandKey::new("done-while-disconnected").unwrap(),
            conv_move,
            ExpectedDisconnectedCard::Ready {
                reply_id: 2,
                proposal_version: 2,
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    // Lost-receipt retry with the same key & fence returns `Ok` idempotently even after standing became `None`.
    let done_conv_replayed = store
        .mark_disconnected_reply_done(
            &access_b,
            &CommandKey::new("done-while-disconnected").unwrap(),
            conv_move,
            ExpectedDisconnectedCard::Ready {
                reply_id: 2,
                proposal_version: 2,
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(done_conv_replayed.reply_standing, ReplyStanding::None);
    assert_eq!(done_conv.reply_standing, ReplyStanding::None);
    assert!(matches!(
        done_conv.patient_messages[1].standing,
        PatientMessageStanding::MarkedDoneWhileDisconnected { .. }
    ));
    assert!(matches!(
        done_conv.patient_messages[2].standing,
        PatientMessageStanding::MarkedDoneWhileDisconnected { .. }
    ));
    assert!(
        store
            .reply_card(&access_b, conv_move, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap()
            .is_none()
    );

    // --- Move to `NotAPatient` closes open messages and retires open conversation concerns (`Rule 3`) ---
    let conv_spam = "conv-ac4-not-patient";
    let spam_concern = ConcernId::new("concern-spam-thread").unwrap();
    let unrelated_concern = ConcernId::new("concern-unrelated-chart").unwrap();
    for (id, key) in [
        (&spam_concern, "raise-spam-c"),
        (&unrelated_concern, "raise-unrelated-c"),
    ] {
        store
            .raise_concern(
                &CommandKey::new(key).unwrap(),
                &RaiseConcern {
                    practice: practice.clone(),
                    workspace: ws_a.clone(),
                    id: id.clone(),
                    urgency: ConcernUrgency::Today,
                    reason: format!("Concern {id} on ws_a."),
                    records: vec![ExactRecordReference::new("bmp-a", "r1").unwrap()],
                    source_work: SourceWork::new("message_review", "w-spam", key).unwrap(),
                },
            )
            .await
            .unwrap();
    }
    store
        .link_reply_conversation(
            &practice,
            conv_spam,
            ConversationLink::LinkedPatient {
                workspace: ws_a.clone(),
            },
        )
        .await
        .unwrap();
    store
        .attach_reply_conversation_concern(&practice, conv_spam, &spam_concern)
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_spam.into(),
            message_id: "spam-1".into(),
            sender: MessageSender::Patient,
            text: "Vendor fax advertisement.".into(),
            sent_at: 1_762_903_400,
        })
        .await
        .unwrap();

    let not_patient_outcome = store
        .act_on_conversation_move(
            &CommandKey::new("move-to-not-patient").unwrap(),
            &practice,
            conv_spam,
            ConversationLink::NotAPatient,
        )
        .await
        .unwrap();
    match not_patient_outcome {
        ConversationMoveOutcome::MovedToNotAPatient {
            conversation,
            retired_concern_ids,
        } => {
            assert_eq!(conversation.link, ConversationLink::NotAPatient);
            assert_eq!(conversation.reply_standing, ReplyStanding::None);
            assert!(conversation.woke_events.is_empty());
            assert!(matches!(
                conversation.patient_messages[0].standing,
                PatientMessageStanding::MovedToNotPatient { move_epoch: 1, .. }
            ));
            assert_eq!(retired_concern_ids, vec![spam_concern.clone()]);
        }
        other => panic!("expected MovedToNotAPatient, got {other:?}"),
    }

    let ws_a_concerns = store
        .workspace_concerns(&access_a, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    let by_id: std::collections::BTreeMap<_, _> = ws_a_concerns
        .into_iter()
        .map(|c| (c.id.as_str().to_owned(), c.standing))
        .collect();
    assert!(matches!(
        by_id.get("concern-spam-thread"),
        Some(ConcernStanding::Retired { .. })
    ));
    assert_eq!(
        by_id.get("concern-unrelated-chart"),
        Some(&ConcernStanding::Open)
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac5_no_reply_would_help_ask_coda_waiting_or_queuing_undo_window_and_copy_attribution() {
    let (_url, store, auth) = setup_stores().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let conversation_id = "conv-ac5";

    let (dr_1, _t1, hash_1) = invite_and_sign_in(&auth, &practice, "dr-ac5-1").await;
    let (dr_2, _t2, hash_2) = invite_and_sign_in(&auth, &practice, "dr-ac5-2").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    for dr in [&dr_1, &dr_2] {
        auth.set_membership(dr, &workspace, Membership::Allowed)
            .await
            .unwrap();
    }
    store
        .record_source(&seeded_lab(
            &workspace,
            "bmp-ac5",
            "r1",
            "2026-11-11",
            "4.2",
        ))
        .await
        .unwrap();

    store
        .link_reply_conversation(
            &practice,
            conversation_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            message_id: "m-thanks".into(),
            sender: MessageSender::Patient,
            text: "Got it, thank you!".into(),
            sent_at: 1_762_904_000,
        })
        .await
        .unwrap();

    // Review concludes `NoReplyWouldHelp` and saves its reason on the message.
    store
        .start_or_read_reply_review(&practice, conversation_id, "rev-1", "run-1")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            review_id: "rev-1".into(),
            run_id: "run-1".into(),
            completion_key: "comp-no-reply-1".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::NoReplyWouldHelp {
                reason: "Closing courtesy acknowledgement; no clinical question asked.".into(),
            },
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    let settled = store
        .reply_conversation(&practice, conversation_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(settled.reply_standing, ReplyStanding::None);
    assert!(matches!(
        &settled.patient_messages[0].standing,
        PatientMessageStanding::NeedsNoReply {
            reason: NoReplyReason::ReviewJudgedNoReplyWouldHelp { reason, .. },
            ..
        } if reason == "Closing courtesy acknowledgement; no clinical question asked."
    ));

    let access_1 = WorkspaceAccess {
        practice: &practice,
        physician: &dr_1.physician_id,
        workspace: &workspace,
        session_hash: &hash_1,
    };
    let access_2 = WorkspaceAccess {
        practice: &practice,
        physician: &dr_2.physician_id,
        workspace: &workspace,
        session_hash: &hash_2,
    };

    // `dr_1` clicks `Ask Coda to reply`: reopens `m-thanks` and queues ONE review.
    let ask_1 = store
        .ask_coda_to_reply(
            &access_1,
            &CommandKey::new("ask-1").unwrap(),
            conversation_id,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        ask_1,
        AskCodaToReplyOutcome::QueuedReview { ref conversation, .. }
            if conversation.patient_messages[0].standing == PatientMessageStanding::Open
                && conversation.woke_events.len() == 1
    ));

    // Start that review; while it is active, `dr_2` also clicks `Ask Coda to reply`:
    // waits behind the active review (`WaitingBehindActiveReview`) without queuing a second wake!
    let active_snap = store
        .start_or_read_reply_review(&practice, conversation_id, "rev-asked", "run-asked")
        .await
        .unwrap();
    assert!(matches!(
        active_snap.review.read_state,
        ReviewReadState::Current {
            saw_physician_reply_request: true,
            ..
        }
    ));

    let ask_2 = store
        .ask_coda_to_reply(
            &access_2,
            &CommandKey::new("ask-2").unwrap(),
            conversation_id,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        ask_2,
        AskCodaToReplyOutcome::WaitingBehindActiveReview { ref conversation, .. }
            if conversation.woke_events.is_empty() && conversation.active_review.is_some()
    ));

    // While a physician reply request is pending, `NoReplyWouldHelp` is refused!
    assert!(matches!(
        store
            .complete_reply_review(&CompleteReplyReview {
                practice: practice.clone(),
                conversation_id: conversation_id.into(),
                review_id: "rev-asked".into(),
                run_id: "run-asked".into(),
                completion_key: "comp-refuse-no-reply".into(),
                answered_closures: vec![],
                decision: ReplyReviewDecision::NoReplyWouldHelp {
                    reason: "Still just thanks".into(),
                },
                raised_concern_ids: vec![],
            })
            .await,
        Err(ReplyProposalError::PhysicianRequestedReplyRequiresProposal)
    ));

    // Active review proposes a reply -> satisfies `pending_reply_request` in one run!
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            review_id: "rev-asked".into(),
            run_id: "run-asked".into(),
            completion_key: "comp-satisfy-ask".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "You're very welcome — reach out anytime if anything changes.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Physician-requested warm closing note.".into(),
                    limits: vec![],
                    cited_records: vec![],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    // `Copy reply` (`Rule 5: CopiesWhatYouSee`): stale copy fence fails without writing a receipt;
    // `dr_1` copies Coda's proposal; `dr_2` sees attribution, edits private draft, and copies their private draft!
    assert!(matches!(
        store
            .copy_reply(
                &access_1,
                &CommandKey::new("copy-dr1").unwrap(),
                conversation_id,
                ExpectedDraftBase::ProposalVersion(99),
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));
    let copy_1 = match store
        .copy_reply(
            &access_1,
            &CommandKey::new("copy-dr1").unwrap(),
            conversation_id,
            ExpectedDraftBase::ProposalVersion(1),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap()
    {
        CopyReplyOutcome::Recorded(rec) => rec,
        other => panic!("expected Recorded, got {other:?}"),
    };
    assert_eq!(
        copy_1.copied_source,
        CopiedReplySource::CodaProposal {
            reply_id: 1,
            version: 1,
        }
    );
    assert_eq!(
        copy_1.copied_text,
        "You're very welcome — reach out anytime if anything changes."
    );
    // Same-key retry returns `Duplicate` with the original copy record.
    assert!(matches!(
        store
            .copy_reply(
                &access_1,
                &CommandKey::new("copy-dr1").unwrap(),
                conversation_id,
                ExpectedDraftBase::ProposalVersion(1),
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap(),
        CopyReplyOutcome::Duplicate(ref rec) if rec.copy_id == copy_1.copy_id
    ));

    let card_for_dr2 = store
        .reply_card(&access_2, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(card_for_dr2.other_physician_copies.len(), 1);
    assert_eq!(
        card_for_dr2.other_physician_copies[0].physician,
        dr_1.physician_id
    );

    store
        .edit_reply_proposal_draft(
            &access_2,
            conversation_id,
            ExpectedDraftBase::ProposalVersion(1),
            "Glad to help! See you at your December follow-up.",
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    // Stale `ProposalVersion(1)` when `dr_2` now displays `DraftRevision(1)` fails cleanly!
    assert!(matches!(
        store
            .copy_reply(
                &access_2,
                &CommandKey::new("copy-dr2").unwrap(),
                conversation_id,
                ExpectedDraftBase::ProposalVersion(1),
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));
    let copy_2 = match store
        .copy_reply(
            &access_2,
            &CommandKey::new("copy-dr2").unwrap(),
            conversation_id,
            ExpectedDraftBase::DraftRevision(1),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap()
    {
        CopyReplyOutcome::Recorded(rec) => rec,
        other => panic!("expected Recorded, got {other:?}"),
    };
    assert_eq!(
        copy_2.copied_source,
        CopiedReplySource::PhysicianDraft {
            reply_id: 1,
            draft_revision: 1,
            kind: PhysicianDraftKind::YourText,
        }
    );
    assert_eq!(
        copy_2.copied_text,
        "Glad to help! See you at your December follow-up."
    );
    assert_eq!(
        store
            .reply_copy_records(&practice, conversation_id)
            .await
            .unwrap()
            .len(),
        2
    );

    // Close with `No reply needed`, verify `ClosedUndoBanner` & `Undo` (`Rule 12: UndoBringsBack`),
    // then close again and verify `expire_no_reply_undo` AND new message arrival end `Undo`!
    assert!(matches!(
        store
            .close_reply_no_reply_needed(
                &access_1,
                &CommandKey::new("close-nrn-1").unwrap(),
                conversation_id,
                99,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));
    store
        .close_reply_no_reply_needed(
            &access_1,
            &CommandKey::new("close-nrn-1").unwrap(),
            conversation_id,
            1,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    let surface_closed = store
        .physician_reply_surface(&access_1, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert!(matches!(
        surface_closed,
        PhysicianReplySurface::ClosedUndoBanner {
            reply_id: 1,
            proposal_version: 1,
            ..
        }
    ));

    let undone_card = store
        .undo_close_no_reply_needed(
            &access_1,
            &CommandKey::new("undo-nrn-1").unwrap(),
            conversation_id,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(
        undone_card.displayed_text,
        DisplayedReplyText::CodaProposal {
            reply_id: 1,
            version: 1,
            text: "You're very welcome — reach out anytime if anything changes.".into(),
        }
    );
    assert_eq!(undone_card.open_messages.len(), 1);

    // Close, undo, close again, and verify BOTH `PatientWrites` AND `expire_no_reply_undo` end `Undo`!
    store
        .close_reply_no_reply_needed(
            &access_1,
            &CommandKey::new("close-nrn-2").unwrap(),
            conversation_id,
            1,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    // Same-key retry after state became `ClosedWithUndo` succeeds idempotently.
    store
        .close_reply_no_reply_needed(
            &access_1,
            &CommandKey::new("close-nrn-2").unwrap(),
            conversation_id,
            1,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    let expired = store
        .expire_no_reply_undo(&practice, conversation_id)
        .await
        .unwrap();
    assert_eq!(expired.reply_standing, ReplyStanding::None);
    assert!(matches!(
        store
            .undo_close_no_reply_needed(
                &access_1,
                &CommandKey::new("undo-after-expire").unwrap(),
                conversation_id,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::UndoNoLongerAvailable { .. })
    ));

    // Even BEFORE any practice message is observed (`practice_observation_seq == 0`), a new
    // patient message opening `reply_id = 2` MUST NOT inherit `reply_id = 1`'s copy attributions!
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            message_id: "m-between-replies".into(),
            sender: MessageSender::Patient,
            text: "Quick question before December: is coffee okay?".into(),
            sent_at: 1_762_904_150,
        })
        .await
        .unwrap();
    store
        .start_or_read_reply_review(&practice, conversation_id, "rev-between", "run-between")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            review_id: "rev-between".into(),
            run_id: "run-between".into(),
            completion_key: "comp-between".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Black coffee or water is fine; avoid cream and sugar if we add lipids."
                    .into(),
                basis: ReplyProposalBasisInput {
                    summary: "New reply_id = 2 before any practice message.".into(),
                    limits: vec![],
                    cited_records: vec![],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();
    let reply_2_card = store
        .reply_card(&access_2, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(reply_2_card.reply_id, 2);
    assert!(reply_2_card.other_physician_copies.is_empty());

    // Observing a practice message clears copy attributions on future open cards while keeping copy audit rows.
    store
        .copy_reply(
            &access_1,
            &CommandKey::new("copy-reply2-dr1").unwrap(),
            conversation_id,
            ExpectedDraftBase::ProposalVersion(2),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            practice_message_id: "prac-sent-closing".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "Glad to help! See you at your December follow-up.".into(),
            sent_at: 1_762_904_200,
        })
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            message_id: "m-followup-q".into(),
            sender: MessageSender::Patient,
            text: "Should I fast before December labs?".into(),
            sent_at: 1_762_904_300,
        })
        .await
        .unwrap();
    store
        .start_or_read_reply_review(&practice, conversation_id, "rev-2", "run-2")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            review_id: "rev-2".into(),
            run_id: "run-2".into(),
            completion_key: "comp-followup-q".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "No fasting is needed for your routine BMP.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "BMP requires no fasting.".into(),
                    limits: vec![],
                    cited_records: vec![],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();
    let fresh_card = store
        .reply_card(&access_2, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert!(fresh_card.other_physician_copies.is_empty());
    assert_eq!(
        store
            .reply_copy_records(&practice, conversation_id)
            .await
            .unwrap()
            .len(),
        3
    );

    // Closing `fresh_card` (`version = 3`) with stale `expected_proposal_version = 2` fails cleanly;
    // closing with `expected_proposal_version = 3` succeeds and a new patient message immediately ends `Undo`.
    assert!(matches!(
        store
            .close_reply_no_reply_needed(
                &access_1,
                &CommandKey::new("close-nrn-3").unwrap(),
                conversation_id,
                2,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));
    store
        .close_reply_no_reply_needed(
            &access_1,
            &CommandKey::new("close-nrn-3").unwrap(),
            conversation_id,
            3,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            message_id: "m-ends-undo".into(),
            sender: MessageSender::Patient,
            text: "One more question about vitamin D.".into(),
            sent_at: 1_762_904_400,
        })
        .await
        .unwrap();
    assert!(matches!(
        store
            .physician_reply_surface(&access_1, conversation_id, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap(),
        PhysicianReplySurface::NoOpenCard {
            review_state: CardReviewState::Reviewing,
            ..
        }
    ));
    assert!(matches!(
        store
            .undo_close_no_reply_needed(
                &access_1,
                &CommandKey::new("undo-after-patient-writes").unwrap(),
                conversation_id,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::UndoNoLongerAvailable { .. })
    ));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac6_two_physicians_isolated_drafts_update_offer_named_senders_late_older_msg_and_access() {
    let (_url, store, auth) = setup_stores().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let other_practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let conversation_id = "conv-ac6";

    let (dr_1, _t1, hash_1) = invite_and_sign_in(&auth, &practice, "dr-shared-1").await;
    let (dr_2, _t2, hash_2) = invite_and_sign_in(&auth, &practice, "dr-shared-2").await;
    let (dr_demo, _td, hash_demo) = invite_and_sign_in(&auth, &practice, "dr-demo").await;
    let (dr_denied, _tn, hash_denied) = invite_and_sign_in(&auth, &practice, "dr-denied").await;
    let (dr_revoked, token_rev, hash_revoked) =
        invite_and_sign_in(&auth, &practice, "dr-revoked").await;
    let (dr_foreign, _tf, hash_foreign) =
        invite_and_sign_in(&auth, &other_practice, "dr-foreign").await;

    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr_1, &workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&dr_2, &workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&dr_demo, &workspace, Membership::ClinicDemographic)
        .await
        .unwrap();
    auth.set_membership(&dr_denied, &workspace, Membership::Denied)
        .await
        .unwrap();
    auth.set_membership(&dr_revoked, &workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.end_session(&token_rev).await.unwrap();

    store
        .link_reply_conversation(
            &practice,
            conversation_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();

    // Non-patient sender (`NamedSender`) alongside the patient (`README.md` line 72).
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            message_id: "m-caregiver-1".into(),
            sender: MessageSender::NamedSender {
                name: "Jordan Park (Daughter)".into(),
            },
            text: "Mom's home BP this morning was 126/76.".into(),
            sent_at: 1_700_000_000,
        })
        .await
        .unwrap();

    store
        .start_or_read_reply_review(&practice, conversation_id, "rev-v1", "run-v1")
        .await
        .unwrap();
    let v1_done = match store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            review_id: "rev-v1".into(),
            run_id: "run-v1".into(),
            completion_key: "comp-v1".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Proposal v1: Thank you Jordan — 126/76 is right on target.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Home BP report from daughter Jordan Park.".into(),
                    limits: vec![],
                    cited_records: vec![],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap()
    {
        ReplyReviewOutcome::Applied(done) => *done,
        other => panic!("expected Applied, got {other:?}"),
    };
    let v1_proposed_at = v1_done.proposal.unwrap().proposed_at;

    // `Rule 1: OnlyWithAccess` — every unauthorized physician is refused across reads and writes!
    for (prac, phys, hash) in [
        (&practice, &dr_demo.physician_id, &hash_demo[..]),
        (&practice, &dr_denied.physician_id, &hash_denied[..]),
        (&practice, &dr_revoked.physician_id, &hash_revoked[..]),
        (&other_practice, &dr_foreign.physician_id, &hash_foreign[..]),
    ] {
        let unauth = WorkspaceAccess {
            practice: prac,
            physician: phys,
            workspace: &workspace,
            session_hash: hash,
        };
        assert!(matches!(
            store
                .reply_card(&unauth, conversation_id, &mut PostgresWorkspaceAuthority)
                .await,
            Err(ReplyProposalError::AccessDenied { .. })
        ));
        assert!(matches!(
            store
                .physician_reply_surface(&unauth, conversation_id, &mut PostgresWorkspaceAuthority)
                .await,
            Err(ReplyProposalError::AccessDenied { .. })
        ));
        assert!(matches!(
            store
                .edit_reply_proposal_draft(
                    &unauth,
                    conversation_id,
                    ExpectedDraftBase::ProposalVersion(1),
                    "Unauthorized edit",
                    &mut PostgresWorkspaceAuthority,
                )
                .await,
            Err(ReplyProposalError::AccessDenied { .. })
        ));
        assert!(matches!(
            store
                .copy_reply(
                    &unauth,
                    &CommandKey::new("unauth-copy").unwrap(),
                    conversation_id,
                    ExpectedDraftBase::ProposalVersion(1),
                    &mut PostgresWorkspaceAuthority,
                )
                .await,
            Err(ReplyProposalError::AccessDenied { .. })
        ));
    }

    let access_1 = WorkspaceAccess {
        practice: &practice,
        physician: &dr_1.physician_id,
        workspace: &workspace,
        session_hash: &hash_1,
    };
    let access_2 = WorkspaceAccess {
        practice: &practice,
        physician: &dr_2.physician_id,
        workspace: &workspace,
        session_hash: &hash_2,
    };

    // Both authorized physicians share Coda's proposal initially and see non-patient sender name.
    let initial_1 = store
        .reply_card(&access_1, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        initial_1.non_patient_sender_names,
        vec!["Jordan Park (Daughter)".to_owned()]
    );

    // `dr_1` saves a private draft; `dr_2` saves a DIFFERENT private draft (`Rule 4: YourDraft`).
    store
        .edit_reply_proposal_draft(
            &access_1,
            conversation_id,
            ExpectedDraftBase::ProposalVersion(1),
            "Dr 1 private draft: Great BP, keep current amlodipine dose.",
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    store
        .edit_reply_proposal_draft(
            &access_2,
            conversation_id,
            ExpectedDraftBase::ProposalVersion(1),
            "Dr 2 private draft: Thanks Jordan, please log evening BP too.",
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    // Late-arrived OLDER message (`sent_at <= v1_proposed_at`) arrives first:
    // card MUST show `OlderMessageReadLate` and MUST NOT say `PatientWroteAgain`!
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            message_id: "m-older-late".into(),
            sender: MessageSender::Patient,
            text: "Sent yesterday before webhook caught up.".into(),
            sent_at: v1_proposed_at - 3600,
        })
        .await
        .unwrap();
    let card_with_late_older = store
        .reply_card(&access_1, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        card_with_late_older.notices,
        vec![CardArrivalNotice::OlderMessageReadLate]
    );
    assert!(
        !card_with_late_older
            .notices
            .contains(&CardArrivalNotice::PatientWroteAgain)
    );

    // Now a genuinely NEWER message (`sent_at > v1_proposed_at`) arrives: adds `PatientWroteAgain`.
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            message_id: "m-newer".into(),
            sender: MessageSender::Patient,
            text: "Evening BP is 124/74 and pulse is 68.".into(),
            sent_at: v1_proposed_at + 120,
        })
        .await
        .unwrap();
    let card_with_both = store
        .reply_card(&access_1, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        card_with_both.notices,
        vec![
            CardArrivalNotice::PatientWroteAgain,
            CardArrivalNotice::OlderMessageReadLate,
        ]
    );

    // Coda finishes review for the new messages and publishes `version = 2` on `reply_id = 1`:
    // BOTH `dr_1` and `dr_2` keep their isolated private drafts and receive `coda_update_offer`!
    store
        .start_or_read_reply_review(&practice, conversation_id, "rev-v2", "run-v2")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            review_id: "rev-v2".into(),
            run_id: "run-v2".into(),
            completion_key: "comp-v2".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Proposal v2: Both morning (126/76) and evening (124/74, HR 68) readings look great.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Updated with evening BP and pulse.".into(),
                    limits: vec![],
                    cited_records: vec![],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    let dr1_after_v2 = store
        .reply_card(&access_1, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    let dr2_after_v2 = store
        .reply_card(&access_2, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        dr1_after_v2.displayed_text,
        DisplayedReplyText::PhysicianDraft { ref text, .. }
            if text == "Dr 1 private draft: Great BP, keep current amlodipine dose."
    ));
    assert_eq!(
        dr1_after_v2
            .coda_update_offer
            .as_ref()
            .unwrap()
            .offered_version,
        2
    );
    assert!(matches!(
        dr2_after_v2.displayed_text,
        DisplayedReplyText::PhysicianDraft { ref text, .. }
            if text == "Dr 2 private draft: Thanks Jordan, please log evening BP too."
    ));

    // Stale `No reply needed` click against `v1` after `v2` published is refused (`StaleDraftVersion`),
    // protecting newly arrived `m-newer` from being closed accidentally!
    assert!(matches!(
        store
            .close_reply_no_reply_needed(
                &access_1,
                &CommandKey::new("stale-close-v1").unwrap(),
                conversation_id,
                1,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));

    // Stale `choose_coda_reply_update` fences fail without writing a receipt.
    assert!(matches!(
        store
            .choose_coda_reply_update(
                &access_1,
                &CommandKey::new("choose-v2-dr1").unwrap(),
                conversation_id,
                99,
                2,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));
    assert!(matches!(
        store
            .choose_coda_reply_update(
                &access_1,
                &CommandKey::new("choose-v2-dr1").unwrap(),
                conversation_id,
                1,
                1,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::StaleDraftVersion { .. })
    ));

    // `dr_1` chooses Coda's update (`choose_coda_reply_update`), switching to `CodaProposal` v2
    // with `NewerReplyReplacedPrevious`, while `dr_2`'s private draft stays completely untouched!
    let dr1_chose = store
        .choose_coda_reply_update(
            &access_1,
            &CommandKey::new("choose-v2-dr1").unwrap(),
            conversation_id,
            1,
            2,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    // Lost-receipt retry with the same command key and fences returns `Ok(dr1_chose)` idempotently!
    let dr1_chose_replayed = store
        .choose_coda_reply_update(
            &access_1,
            &CommandKey::new("choose-v2-dr1").unwrap(),
            conversation_id,
            1,
            2,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(dr1_chose_replayed.displayed_text, dr1_chose.displayed_text);

    // Once `dr_1`'s draft was detached via `choose_coda_reply_update`, `Store::save_draft` and
    // `Store::accept_revision` on `dr_1`'s `reply-proposal:` scope are refused until `dr_1` edits again.
    let detached_scope = proposal_draft_scope(
        &practice,
        &dr_1.physician_id,
        &workspace,
        conversation_id,
        1,
    );
    assert!(
        store
            .save_draft(
                &detached_scope,
                1,
                "Save on detached draft",
                &hash_1,
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .is_err()
    );
    assert_eq!(
        store
            .accept_revision(
                &detached_scope,
                &RevisionInput {
                    key: "rev-on-detached".into(),
                    expected_revision: 1,
                    instruction: "Revise detached".into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                hash_1.clone(),
                &load_skills(),
                &test_revision_recipe(),
                None,
            )
            .await
            .unwrap(),
        local_product::runtime::reply::RevisionAccepted::Refused
    );
    assert_eq!(
        dr1_chose.displayed_text,
        DisplayedReplyText::CodaProposal {
            reply_id: 1,
            version: 2,
            text: "Proposal v2: Both morning (126/76) and evening (124/74, HR 68) readings look great.".into(),
        }
    );
    assert!(dr1_chose.coda_update_offer.is_none());
    assert_eq!(
        dr1_chose.notices,
        vec![CardArrivalNotice::NewerReplyReplacedPrevious {
            previous_version: 1,
            current_version: 2,
        }]
    );

    let dr2_still_private = store
        .reply_card(&access_2, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        dr2_still_private.displayed_text,
        DisplayedReplyText::PhysicianDraft { ref text, .. }
            if text == "Dr 2 private draft: Thanks Jordan, please log evening BP too."
    ));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac7_deduplicates_wakes_and_restart_requires_full_reread_without_duplicate_review() {
    let (_url, store, auth) = setup_stores().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let conversation_id = "conv-ac7";

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac7").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    store
        .link_reply_conversation(
            &practice,
            conversation_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();

    // Replaying the same patient message or practice message queues at most ONE wake (`Rule 7`).
    let patient_event = RecordPatientMessage {
        practice: practice.clone(),
        conversation_id: conversation_id.into(),
        message_id: "m-dedup-1".into(),
        sender: MessageSender::Patient,
        text: "Can I get a refill on my inhaler?".into(),
        sent_at: 1_762_905_000,
    };
    assert!(matches!(
        store
            .record_linked_patient_message(&patient_event)
            .await
            .unwrap(),
        MessageIntakeOutcome::Recorded { .. }
    ));
    let dup_patient = match store
        .record_linked_patient_message(&patient_event)
        .await
        .unwrap()
    {
        MessageIntakeOutcome::Duplicate { conversation } => *conversation,
        other => panic!("expected Duplicate, got {other:?}"),
    };
    assert_eq!(dup_patient.woke_events.len(), 1);
    assert_eq!(dup_patient.conversation_version, 2);

    // Before a card exists, `physician_reply_surface` surfaces open messages with `Reviewing` (`Rule 9`).
    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let waiting_surface = store
        .physician_reply_surface(&access, conversation_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert!(matches!(
        waiting_surface,
        PhysicianReplySurface::NoOpenCard {
            ref open_messages,
            review_state: CardReviewState::Reviewing,
            ..
        } if open_messages.len() == 1
    ));

    // Start review (`run-1`) -> consumes wake queue (`woke_events` empty).
    store
        .start_or_read_reply_review(&practice, conversation_id, "rev-ac7", "run-1")
        .await
        .unwrap();
    // Another review ID trying to start concurrently is refused (`StaleReview`, `Rule 7`).
    assert!(matches!(
        store
            .start_or_read_reply_review(&practice, conversation_id, "rev-competing", "run-comp")
            .await,
        Err(ReplyProposalError::StaleReview { .. })
    ));

    // Simulate worker interruption/restart (`restart_reply_review` / `CodaRestarts`):
    // sets `ReviewReadState::NeedsRead` on `run-2` WITHOUT requeuing already-consumed wakes!
    let restarted = store
        .restart_reply_review(&practice, conversation_id, "run-2")
        .await
        .unwrap();
    assert!(restarted.woke_events.is_empty());
    assert_eq!(
        restarted.active_review.as_ref().unwrap().read_state,
        ReviewReadState::NeedsRead
    );

    // Completing `run-2` BEFORE re-reading the full conversation is refused (`StaleConversationRead`).
    assert!(matches!(
        store
            .complete_reply_review(&CompleteReplyReview {
                practice: practice.clone(),
                conversation_id: conversation_id.into(),
                review_id: "rev-ac7".into(),
                run_id: "run-2".into(),
                completion_key: "comp-without-reread".into(),
                answered_closures: vec![],
                decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                    text: "Premature proposal without re-reading.".into(),
                    basis: ReplyProposalBasisInput {
                        summary: "Premature".into(),
                        limits: vec![],
                        cited_records: vec![],
                    },
                    cited_records: vec![],
                }),
                raised_concern_ids: vec![],
            })
            .await,
        Err(ReplyProposalError::StaleConversationRead {
            read_version: None,
            ..
        })
    ));
    // Old `run-1` is also fenced (`StaleReview`).
    assert!(matches!(
        store
            .complete_reply_review(&CompleteReplyReview {
                practice: practice.clone(),
                conversation_id: conversation_id.into(),
                review_id: "rev-ac7".into(),
                run_id: "run-1".into(),
                completion_key: "comp-old-run1".into(),
                answered_closures: vec![],
                decision: ReplyReviewDecision::NoReplyWouldHelp {
                    reason: "Old run".into(),
                },
                raised_concern_ids: vec![],
            })
            .await,
        Err(ReplyProposalError::StaleReview { .. })
    ));

    // Re-reading the full conversation on `run-2` (`start_or_read_reply_review`) restores currency
    // and allows `run-2` to publish cleanly; afterward `woke_events` is empty (`NoPendingReviewWake`).
    let reread = store
        .start_or_read_reply_review(&practice, conversation_id, "rev-ac7", "run-2")
        .await
        .unwrap();
    assert!(matches!(
        reread.review.read_state,
        ReviewReadState::Current { .. }
    ));
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conversation_id.into(),
            review_id: "rev-ac7".into(),
            run_id: "run-2".into(),
            completion_key: "comp-after-reread".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "We sent your albuterol inhaler refill to your pharmacy today.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Re-read full conversation after worker restart.".into(),
                    limits: vec![],
                    cited_records: vec![],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    assert!(matches!(
        store
            .start_or_read_reply_review(&practice, conversation_id, "rev-extra", "run-extra")
            .await,
        Err(ReplyProposalError::NoPendingReviewWake { .. })
    ));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ac5_ask_coda_edge_cases_empty_thread_active_review_handoff_and_answered_only_thread() {
    let (_url, store, auth) = setup_stores().await;
    let practice = uuid::Uuid::new_v4().to_string();
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac5-edges").await;
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };

    // 1. Empty conversation (`patient_messages.is_empty()`): `ask_coda_to_reply` refuses upfront
    // with `NoOpenMessagesForProposal` and leaves no receipt so the same key can succeed later.
    let conv_handoff = "conv-ac5-handoff";
    store
        .link_reply_conversation(
            &practice,
            conv_handoff,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    assert!(matches!(
        store
            .ask_coda_to_reply(
                &access,
                &CommandKey::new("ask-reusable-after-empty").unwrap(),
                conv_handoff,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(ReplyProposalError::NoOpenMessagesForProposal)
    ));

    // 2. Untrimmed CRLF patient words are preserved verbatim (`Rule 8` / channel fidelity).
    let exact_patient_words = "  Thanks!\r\nSee you next month.  ";
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_handoff.into(),
            message_id: "m-crlf-1".into(),
            sender: MessageSender::Patient,
            text: exact_patient_words.into(),
            sent_at: 1_762_906_000,
        })
        .await
        .unwrap();
    match store
        .physician_reply_surface(&access, conv_handoff, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
    {
        PhysicianReplySurface::NoOpenCard { open_messages, .. } => {
            assert_eq!(open_messages.len(), 1);
            assert_eq!(open_messages[0].text, exact_patient_words);
        }
        other => panic!("expected NoOpenCard, got {other:?}"),
    }

    // 3. Active review starts BEFORE `ask_coda_to_reply` (`saw_physician_reply_request == false`),
    // then physician clicks `Ask Coda to reply` (reusing `"ask-reusable-after-empty"`!), and the
    // running review finishes with `NoReplyWouldHelp`: it succeeds cleanly and queues a follow-up wake!
    let pre_ask_snap = store
        .start_or_read_reply_review(&practice, conv_handoff, "rev-pre-ask", "run-pre-ask")
        .await
        .unwrap();
    assert!(matches!(
        pre_ask_snap.review.read_state,
        ReviewReadState::Current {
            saw_physician_reply_request: false,
            ..
        }
    ));

    let ask_mid_run = store
        .ask_coda_to_reply(
            &access,
            &CommandKey::new("ask-reusable-after-empty").unwrap(),
            conv_handoff,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        ask_mid_run,
        AskCodaToReplyOutcome::WaitingBehindActiveReview { .. }
    ));

    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_handoff.into(),
            review_id: "rev-pre-ask".into(),
            run_id: "run-pre-ask".into(),
            completion_key: "comp-pre-ask-no-reply".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::NoReplyWouldHelp {
                reason: "Initial review judged courtesy note needed no reply.".into(),
            },
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    let after_first_run = store
        .reply_conversation(&practice, conv_handoff)
        .await
        .unwrap()
        .unwrap();
    assert!(after_first_run.pending_reply_request.is_some());
    assert_eq!(after_first_run.woke_events.len(), 1);
    assert_eq!(
        after_first_run.patient_messages[0].standing,
        PatientMessageStanding::Open
    );

    let followup_snap = store
        .start_or_read_reply_review(&practice, conv_handoff, "rev-followup", "run-followup")
        .await
        .unwrap();
    assert!(matches!(
        followup_snap.review.read_state,
        ReviewReadState::Current {
            saw_physician_reply_request: true,
            ..
        }
    ));
    assert!(matches!(
        store
            .complete_reply_review(&CompleteReplyReview {
                practice: practice.clone(),
                conversation_id: conv_handoff.into(),
                review_id: "rev-followup".into(),
                run_id: "run-followup".into(),
                completion_key: "comp-followup-refuse".into(),
                answered_closures: vec![],
                decision: ReplyReviewDecision::NoReplyWouldHelp {
                    reason: "Refused because physician asked".into(),
                },
                raised_concern_ids: vec![],
            })
            .await,
        Err(ReplyProposalError::PhysicianRequestedReplyRequiresProposal)
    ));
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_handoff.into(),
            review_id: "rev-followup".into(),
            run_id: "run-followup".into(),
            completion_key: "comp-followup-propose".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Looking forward to seeing you next month!".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Follow-up review satisfied pending physician request.".into(),
                    limits: vec![],
                    cited_records: vec![],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();
    let satisfied = store
        .reply_conversation(&practice, conv_handoff)
        .await
        .unwrap()
        .unwrap();
    assert!(satisfied.pending_reply_request.is_none());
    assert!(satisfied.woke_events.is_empty());

    // 4. Conversely, when an active review (`saw_physician_reply_request == false`) finishes with
    // `ProposeReply`, it satisfies and clears `pending_reply_request` in that single run!
    let conv_one_run = "conv-ac5-single-run-satisfy";
    store
        .link_reply_conversation(
            &practice,
            conv_one_run,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_one_run.into(),
            message_id: "m-one-run".into(),
            sender: MessageSender::Patient,
            text: "Should I bring my blood pressure cuff?".into(),
            sent_at: 1_762_906_100,
        })
        .await
        .unwrap();
    store
        .start_or_read_reply_review(&practice, conv_one_run, "rev-one-run", "run-one-run")
        .await
        .unwrap();
    store
        .ask_coda_to_reply(
            &access,
            &CommandKey::new("ask-one-run").unwrap(),
            conv_one_run,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_one_run.into(),
            review_id: "rev-one-run".into(),
            run_id: "run-one-run".into(),
            completion_key: "comp-one-run".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Yes, please bring your home cuff so we can check its calibration.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Satisfied in one run.".into(),
                    limits: vec![],
                    cited_records: vec![],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();
    let one_run_after = store
        .reply_conversation(&practice, conv_one_run)
        .await
        .unwrap()
        .unwrap();
    assert!(one_run_after.pending_reply_request.is_none());
    assert!(one_run_after.woke_events.is_empty());

    // 5. Thread whose ONLY patient message is closed as `Answered` (`AllHandled` mid-run while
    // `ask_coda_to_reply` arrived): `AllHandled` succeeds cleanly without reverting `Answered` to `Open`,
    // queues `PhysicianAskedForReply`, and the follow-up `ProposeReply` covers that message while
    // keeping its `Answered` standing intact!
    let conv_answered = "conv-ac5-answered-only";
    store
        .link_reply_conversation(
            &practice,
            conv_answered,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_answered.into(),
            message_id: "m-only-answered".into(),
            sender: MessageSender::Patient,
            text: "Did you get my pharmacy update?".into(),
            sent_at: 1_762_906_200,
        })
        .await
        .unwrap();
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_answered.into(),
            practice_message_id: "pm-staff-ack".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "Front desk updated your preferred pharmacy.".into(),
            sent_at: 1_762_906_210,
        })
        .await
        .unwrap();
    store
        .start_or_read_reply_review(&practice, conv_answered, "rev-ans-1", "run-ans-1")
        .await
        .unwrap();
    store
        .ask_coda_to_reply(
            &access,
            &CommandKey::new("ask-on-answered").unwrap(),
            conv_answered,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_answered.into(),
            review_id: "rev-ans-1".into(),
            run_id: "run-ans-1".into(),
            completion_key: "comp-ans-1".into(),
            answered_closures: vec![AnsweredMessageClosure {
                practice_message_id: "pm-staff-ack".into(),
                answered_message_ids: set_of(&["m-only-answered"]),
            }],
            decision: ReplyReviewDecision::AllHandled,
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    let mid_answered = store
        .reply_conversation(&practice, conv_answered)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        mid_answered.patient_messages[0].standing,
        PatientMessageStanding::Answered { .. }
    ));
    assert_eq!(mid_answered.woke_events.len(), 1);

    store
        .start_or_read_reply_review(&practice, conv_answered, "rev-ans-2", "run-ans-2")
        .await
        .unwrap();
    assert!(matches!(
        store
            .complete_reply_review(&CompleteReplyReview {
                practice: practice.clone(),
                conversation_id: conv_answered.into(),
                review_id: "rev-ans-2".into(),
                run_id: "run-ans-2".into(),
                completion_key: "comp-ans-refuse".into(),
                answered_closures: vec![],
                decision: ReplyReviewDecision::AllHandled,
                raised_concern_ids: vec![],
            })
            .await,
        Err(ReplyProposalError::PhysicianRequestedReplyRequiresProposal)
    ));
    let ans_applied = match store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_answered.into(),
            review_id: "rev-ans-2".into(),
            run_id: "run-ans-2".into(),
            completion_key: "comp-ans-propose".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text:
                    "Dr. Edges here as well — confirming your new pharmacy is active for refills."
                        .into(),
                basis: ReplyProposalBasisInput {
                    summary: "Physician requested follow-up confirmation on answered thread."
                        .into(),
                    limits: vec![],
                    cited_records: vec![],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap()
    {
        ReplyReviewOutcome::Applied(done) => *done,
        other => panic!("expected Applied, got {other:?}"),
    };
    assert_eq!(
        ans_applied.proposal.as_ref().unwrap().covered_message_ids,
        set_of(&["m-only-answered"])
    );
    let final_answered = store
        .reply_conversation(&practice, conv_answered)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        final_answered.patient_messages[0].standing,
        PatientMessageStanding::Answered { .. }
    ));
    assert!(final_answered.pending_reply_request.is_none());
}
