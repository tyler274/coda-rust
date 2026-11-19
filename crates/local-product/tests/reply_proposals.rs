//! Unit and real PostgreSQL proof for reply proposals, conversation review
//! fencing, practice-message partial closures, patient/not-a-patient moves,
//! channel disconnects, copy attribution, undo windows, and private per-physician
//! drafts (`verification/physician/message-reply/`).

use local_product::{
    physician_chat::PhysicianSkills,
    physician_sources::{RecordDate, SourceRevision},
    runtime::{
        Authority, PublicationTransaction, Scope, Store, async_trait,
        concerns::{CommandKey, ExactRecordReference, WorkspaceAccess},
        delivery::Delivery,
        engine::{
            Event, EventKind, Invocation, Item, Turn,
            recipe::{RequestRecipe, Usage},
        },
        reply::{
            FINISH_TOOL, RevisionCommit, RevisionInput,
            proposals::{
                CompleteReplyReview, ConversationLink, DisplayedReplyText, ExpectedDraftBase,
                MessageIntakeOutcome, MessageSender, PhysicianDraftKind, ProposalRevisionAccepted,
                RecordPatientMessage, ReplyProposalBasisInput, ReplyProposalError,
                ReplyProposalInput, ReplyProposalInputError, ReplyReviewDecision,
                ReplyReviewOutcome, ReplyStanding, proposal_draft_scope,
            },
        },
    },
    sha256_hex,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use serde_json::json;
use std::collections::BTreeSet;
use std::error::Error as _;
use std::path::PathBuf;

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
}
