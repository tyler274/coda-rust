//! Real PostgreSQL integration tests for message and follow-up producers in the
//! Today and Updates read model: routing of every message and follow-up kind,
//! panel-gated match suggestions, `Done` clearance, partial-failure isolation,
//! and the joint reply + concern lifecycle on Today.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, Weak};

use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::concerns::{
    CommandKey, ConcernId, ConcernUrgency, ExactRecordReference, RaiseConcern, SourceWork,
    WorkspaceAccess,
};
use local_product::runtime::follow_ups::{
    FollowUpChangeNote, FollowUpCommandKey, FollowUpId, FollowUpReviewOutcome, FollowUpSummary,
    PhysicianChangeFollowUp, PhysicianCreateFollowUp, WithdrawWrongPatientRequest,
};
use local_product::runtime::reply::proposals::{
    AnsweredMessageClosure, ChannelConnection, CompleteReplyReview, ConversationLink,
    MessageSender, ObservePracticeMessage, PracticeMessageKind, RecordPatientMessage,
    ReplyProposalBasisInput, ReplyProposalInput, ReplyReviewDecision,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::spruce_matching::{
    ChannelStopCause, IncomingSpruceItem, MatchAssessment, MatchingDecision, MessageStanding,
    MessageUrgencyChoice, NonPatientCategory, OutsideParticipant, ReadableSpruceItem,
    SenderAddress, UnreadableSpruceItem,
};
use local_product::runtime::today_updates::{
    ClearItemDoneRequest, OpenItemRequest, PRACTICE_MESSAGING_WORKSPACE, ProducerKind,
    ReadCompleteness, SeenStanding, TodayDoneResult, TodayRowPayload, TodayUpdatesProbe,
    UnhandledWorkKind, UpdateEntryKind,
};
use local_product::runtime::{Authority, PublicationTransaction, Scope, Store, async_trait};
use local_product::sha256_hex;
use local_product::spruce_matching_recipe::{GUIDANCE_VERSION, RECIPE_VERSION};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::json;
use uuid::Uuid;

fn current_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_secs()
}

struct LocalCluster {
    cluster_dir: PathBuf,
    data_dir: PathBuf,
    admin_url: String,
}

impl Drop for LocalCluster {
    fn drop(&mut self) {
        let _ = Command::new("pg_ctl")
            .args([
                "-D",
                self.data_dir.to_str().unwrap_or_default(),
                "-m",
                "immediate",
                "stop",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = std::fs::remove_dir_all(&self.cluster_dir);
    }
}

static LOCAL_CLUSTER: StdMutex<Weak<LocalCluster>> = StdMutex::new(Weak::new());
static MIGRATE_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn acquire_database() -> Option<(String, Option<Arc<LocalCluster>>)> {
    for key in [
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
        "PHYSICIAN_DATABASE_URL",
        "PHYSICIAN_TEST_DATABASE_URL",
    ] {
        if let Ok(url) = std::env::var(key) {
            let trimmed = url.trim().to_owned();
            if !trimmed.is_empty() {
                return Some((trimmed, None));
            }
        }
    }

    let mut slot = LOCAL_CLUSTER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = slot.upgrade() {
        return Some((existing.admin_url.clone(), Some(existing)));
    }

    let port = {
        use std::net::TcpListener;
        (0..500)
            .find_map(|_| {
                let candidate = 32_000 + (Uuid::new_v4().as_u128() % 9_000) as u16;
                TcpListener::bind(("127.0.0.1", candidate))
                    .ok()
                    .map(|_| candidate)
            })
            .expect("free loopback port for local postgres")
    };
    let cluster_dir =
        std::env::temp_dir().join(format!("coda-tumf-pg-{}-{}", std::process::id(), port));
    let _ = std::fs::remove_dir_all(&cluster_dir);
    let data_dir = cluster_dir.join("data");
    let sock_dir = cluster_dir.join("sock");
    std::fs::create_dir_all(&data_dir).ok()?;
    std::fs::create_dir_all(&sock_dir).ok()?;

    let init = Command::new("initdb")
        .args([
            "-D",
            data_dir.to_str()?,
            "-U",
            "postgres",
            "-A",
            "trust",
            "--no-locale",
            "-E",
            "UTF8",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    if !init.success() {
        let _ = std::fs::remove_dir_all(&cluster_dir);
        return None;
    }

    let log_file = cluster_dir.join("postgres.log");
    let opts = format!("-F -p {port} -h 127.0.0.1 -k {}", sock_dir.to_str()?);
    let start = Command::new("pg_ctl")
        .args([
            "-D",
            data_dir.to_str()?,
            "-l",
            log_file.to_str()?,
            "-o",
            &opts,
            "-w",
            "start",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    if !start.success() {
        let _ = std::fs::remove_dir_all(&cluster_dir);
        return None;
    }

    let cluster = Arc::new(LocalCluster {
        cluster_dir,
        data_dir,
        admin_url: format!("postgresql://postgres@127.0.0.1:{port}/postgres"),
    });
    *slot = Arc::downgrade(&cluster);
    Some((cluster.admin_url.clone(), Some(cluster)))
}

async fn connect_stores() -> Option<(Store, Arc<PostgresAuthStore>, Option<Arc<LocalCluster>>)> {
    let Some((url, cluster)) = acquire_database() else {
        assert!(
            std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
            "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL"
        );
        return None;
    };
    let _guard = MIGRATE_LOCK.lock().await;
    let store = Store::connect(&url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("connect and migrate physician auth store"),
    );
    drop(_guard);
    Some((store, auth, cluster))
}

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

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice_id: &str,
    label: &str,
    now_unix: u64,
) -> (PhysicianIdentity, String, Vec<u8>) {
    let email = format!("{label}-{}@codahealth.test", Uuid::new_v4().simple());
    let invite = auth
        .invite(practice_id, &email, label, true, now_unix)
        .await
        .expect("invite physician");
    let (identity, token) = auth
        .redeem_invitation(&email, &invite, now_unix)
        .await
        .expect("redeem invitation");
    let hash = PostgresAuthStore::session_reference(&token).to_vec();
    (identity, token, hash)
}

async fn seed_chart_record(
    store: &Store,
    workspace: &str,
    patient_name: &str,
    candidate_id: &str,
    revision: &str,
    text: &str,
    pulled_at: i64,
) -> ExactRecordReference {
    let pull = SourcePull {
        source_id: "regional-ehr".to_owned(),
        source_label: "Regional EHR".to_owned(),
        patient: SourcePatient {
            source_patient_id: format!("pt-{workspace}"),
            display_name: patient_name.to_owned(),
            birth_date: Some("1968-05-14".to_owned()),
            sex_or_gender: Some("female".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "clinic-mrn".to_owned(),
                value: format!("MRN-{workspace}"),
            }],
        },
        pulled_at,
    };
    let rec = SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "regional-ehr".to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: "Observation".to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".to_owned(),
            value: "2026-11-12".to_owned(),
        }],
        content: text.to_owned(),
        sha256: sha256_hex(text.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "Lab Result".to_owned(),
            text: text.to_owned(),
        }],
        provenance: json!({
            "system": "regional-ehr",
            "id": candidate_id,
            "title": "Metabolic Panel",
        }),
    };
    store
        .record_chart(workspace, &[rec], &pull)
        .await
        .expect("record chart");
    ExactRecordReference::new(candidate_id, revision).expect("valid record reference")
}

struct FailingSliceProbe {
    fail_producers: BTreeSet<ProducerKind>,
}

impl TodayUpdatesProbe for FailingSliceProbe {
    fn check_producer(&self, producer: ProducerKind) -> Result<(), String> {
        if self.fail_producers.contains(&producer) {
            return Err(format!("simulated {} outage", producer.as_str()));
        }
        Ok(())
    }
}

#[tokio::test]
async fn each_message_and_follow_up_kind_reaches_the_right_list_with_item_open_and_done() {
    let Some((store, auth, _cluster)) = connect_stores().await else {
        return;
    };

    let now_u64 = current_unix();
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-tumf-ac1-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr_alpha, token_alpha, hash_alpha) =
        invite_and_sign_in(&auth, &practice, "Dr. Alpha", now_u64).await;
    let (dr_beta, token_beta, _hash_beta) =
        invite_and_sign_in(&auth, &practice, "Dr. Beta", now_u64).await;

    let ws_thread = format!("pw-thread-{}", Uuid::new_v4().simple());
    let ws_disconnected = format!("pw-disconn-{}", Uuid::new_v4().simple());
    let ws_suggest_only_alpha = format!("pw-sug-alpha-{}", Uuid::new_v4().simple());
    let ws_fu_active = format!("pw-fu-active-{}", Uuid::new_v4().simple());
    let ws_fu_cannot_restart = format!("pw-fu-cant-{}", Uuid::new_v4().simple());
    let ws_fu_withdrawn = format!("pw-fu-withdrawn-{}", Uuid::new_v4().simple());

    for (ws, name) in [
        (&ws_thread, "Clara Oswald"),
        (&ws_disconnected, "Danny Pink"),
        (&ws_suggest_only_alpha, "Martha Jones"),
        (&ws_fu_active, "Donna Noble"),
        (&ws_fu_cannot_restart, "Wilfred Mott"),
        (&ws_fu_withdrawn, "River Song"),
    ] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr_alpha, ws, Membership::Allowed)
            .await
            .unwrap();
        let _ = seed_chart_record(
            &store,
            ws,
            name,
            "obs-bmp-1",
            "rev-1",
            "Serum potassium 4.3 mmol/L, creatinine 0.95 mg/dL.",
            now_i64 - 1_800,
        )
        .await;
    }
    // `dr_beta` is ONLY allowed on `ws_thread` (NOT allowed on `ws_suggest_only_alpha`), proving Rule-2 suggestion gating!
    auth.set_membership(&dr_beta, &ws_thread, Membership::Allowed)
        .await
        .unwrap();

    let mut authority = PostgresWorkspaceAuthority;
    let bmp_ref = ExactRecordReference::new("obs-bmp-1", "rev-1").unwrap();

    // 1. Linked `MessageThread` with a saved `ReplyDrafted` proposal on `ws_thread` -> Today (`MessageThread`) + Updates (`ReplyDrafted`).
    store
        .link_reply_conversation(
            &practice,
            "conv-clara-thread",
            ConversationLink::LinkedPatient {
                workspace: ws_thread.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-clara-thread".into(),
            message_id: "msg-clara-1".into(),
            sender: MessageSender::Patient,
            text: "Hi Dr. Alpha, did my potassium lab come back normal?".into(),
            sent_at: now_i64 - 900,
        })
        .await
        .unwrap();
    let _ = store
        .start_or_read_reply_review(&practice, "conv-clara-thread", "rev-clara-1", "run-clara-1")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: "conv-clara-thread".into(),
            review_id: "rev-clara-1".into(),
            run_id: "run-clara-1".into(),
            completion_key: "comp-clara-1".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Your potassium is normal at 4.3 mmol/L.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Reviewed Regional EHR metabolic panel.".into(),
                    limits: vec![],
                    cited_records: vec![bmp_ref.clone()],
                },
                cited_records: vec![bmp_ref.clone()],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    // Also link a `NotAPatient` conversation in `reply_conversations` -> NEVER appears on Today or Updates!
    store
        .link_reply_conversation(
            &practice,
            "conv-vendor-nonpatient",
            ConversationLink::NotAPatient,
        )
        .await
        .unwrap();

    // 2. Disconnected linked `MessageThread` on `ws_disconnected` -> Today (`can_mark_done == true`), clearable with `Done`.
    store
        .link_reply_conversation(
            &practice,
            "conv-danny-disc",
            ConversationLink::LinkedPatient {
                workspace: ws_disconnected.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-danny-disc".into(),
            message_id: "msg-danny-1".into(),
            sender: MessageSender::Patient,
            text: "Question sent right before channel disconnect.".into(),
            sent_at: now_i64 - 1_200,
        })
        .await
        .unwrap();
    let _ = store
        .start_or_read_reply_review(&practice, "conv-danny-disc", "rev-danny-1", "run-danny-1")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: "conv-danny-disc".into(),
            review_id: "rev-danny-1".into(),
            run_id: "run-danny-1".into(),
            completion_key: "comp-danny-1".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Your lab panel remains normal; let us know if symptoms change.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Reviewed Regional EHR metabolic panel.".into(),
                    limits: vec![],
                    cited_records: vec![bmp_ref.clone()],
                },
                cited_records: vec![bmp_ref.clone()],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();
    store
        .set_reply_channel_connection(
            &practice,
            "conv-danny-disc",
            ChannelConnection::Disconnected {
                disconnected_at: now_i64 - 1_100,
                note: "Spruce channel disconnected.".into(),
            },
        )
        .await
        .unwrap();

    // 3. Spruce channel items:
    //    - Ordinary `MessageToMatch` suggesting `ws_suggest_only_alpha`
    //    - Urgent `MessageToMatch` (`leads_as_now: true`, sorts in `Now` ahead of ordinary rows)
    //    - Non-patient vendor spam (`FilteredNotPatient` -> NEITHER Today NOR Updates)
    //    - `UnreadableMessage` (`CantRead` -> Today `unreadable:{id}`, clearable with `Done`)
    //    - Transient `ChannelStopCause::Transient` -> Today `MessagingChannelProblem { reconnect_required: false }`
    store
        .connect_spruce_channel(auth.as_ref(), &dr_alpha, 0, None, now_i64 - 2_000)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-tumf", 3_600, now_i64 - 2_000)
        .await
        .unwrap();

    // 3a. Ordinary `MessageToMatch` suggesting `ws_suggest_only_alpha`.
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "spruce-match-ordinary".into(),
                conversation_id: "conv-match-ordinary".into(),
                channel_position: 1,
                content_version: "v1".into(),
                sender_address: Some(SenderAddress::parse("+14155550801").unwrap()),
                author_name: Some("Martha J.".into()),
                body_text: "Texting from my work phone about next month's refill.".into(),
                attachments: Vec::new(),
                app_url: Some(
                    "https://app.sprucehealth.com/conversations/conv-match-ordinary".into(),
                ),
                created_at_spruce: Some("2026-11-12T14:40:00Z".into()),
                outside_participants: vec![OutsideParticipant {
                    display_name: Some("Martha J.".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550801").unwrap()),
                    saved_contact_id: None,
                }],
            }),
            now_i64 - 1_000,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "spruce-match-ordinary",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: ws_suggest_only_alpha.clone(),
                    evidence_summary: "Self-identifies as Martha J.".into(),
                },
                rationale: "New phone needs physician confirmation.".into(),
            },
            None,
            now_i64 - 995,
        )
        .await
        .unwrap();

    // 3b. Urgent `MessageToMatch` (`MessageUrgencyChoice::Urgent` -> `Now`).
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "spruce-match-urgent".into(),
                conversation_id: "conv-match-urgent".into(),
                channel_position: 2,
                content_version: "v1".into(),
                sender_address: Some(SenderAddress::parse("+14155550802").unwrap()),
                author_name: Some("Unknown Urgent Caller".into()),
                body_text: "Sudden severe shortness of breath and chest tightness right now."
                    .into(),
                attachments: Vec::new(),
                app_url: Some(
                    "https://app.sprucehealth.com/conversations/conv-match-urgent".into(),
                ),
                created_at_spruce: Some("2026-11-12T14:58:00Z".into()),
                outside_participants: vec![OutsideParticipant {
                    display_name: Some("Unknown Urgent Caller".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(SenderAddress::parse("+14155550802").unwrap()),
                    saved_contact_id: None,
                }],
            }),
            now_i64 - 120,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "spruce-match-urgent",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Urgent {
                    reason: "Acute dyspnea and chest tightness".into(),
                },
                assessment: MatchAssessment::NoMatchFound {
                    evidence_summary: "Unrecognized sender phone.".into(),
                },
                rationale: "Urgent unmatched patient symptoms.".into(),
            },
            None,
            now_i64 - 115,
        )
        .await
        .unwrap();

    // 3c. Non-patient vendor spam (`FilteredNotPatient` -> never on Today or Updates).
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "spruce-spam-3".into(),
                conversation_id: "conv-spam-3".into(),
                channel_position: 3,
                content_version: "v1".into(),
                sender_address: Some(SenderAddress::parse("+18005550199").unwrap()),
                author_name: Some("Office Toner Promo".into()),
                body_text: "Save 40% on fax toner cartridges this week!".into(),
                attachments: Vec::new(),
                app_url: None,
                created_at_spruce: None,
                outside_participants: vec![],
            }),
            now_i64 - 800,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "spruce-spam-3",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::NotPatientMail {
                category: NonPatientCategory::VendorOrSpam,
                rationale: "Commercial office supply advertisement.".into(),
            },
            None,
            now_i64 - 795,
        )
        .await
        .unwrap();

    // 3d. Unreadable message (`CantRead` -> Today `unreadable:spruce-unreadable-4`).
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Unreadable(UnreadableSpruceItem {
                message_id: "spruce-unreadable-4".into(),
                conversation_id: "conv-unreadable-4".into(),
                channel_position: 4,
                content_version: "v-err-4".into(),
                failure_reason: "Corrupted fax TIFF page stream".into(),
                app_url: Some("https://app.sprucehealth.com/items/4".into()),
                outside_participants: vec![],
            }),
            now_i64 - 700,
        )
        .await
        .unwrap();

    // 3e. Transient channel failure -> Today `MessagingChannelProblem { reconnect_required: false }`.
    store
        .record_spruce_channel_failure(
            &practice,
            &lease.lease_token,
            ChannelStopCause::Transient {
                reason: "Spruce API gateway 503".into(),
            },
            now_i64 - 500,
        )
        .await
        .unwrap();

    // 4. Follow-ups across every standing:
    //    - `ws_fu_active`: `Created` -> `Changed` -> `Completed`, plus a second `Cancelled` follow-up,
    //      plus a third still-`Active` future follow-up (none of these appear on Today; all 4 transitions reach Updates!).
    let fu_lifecycle_id = FollowUpId::new("fu-donna-lifecycle").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr_alpha.practice_id,
                physician: &dr_alpha.physician_id,
                workspace: &ws_fu_active,
                session_hash: &hash_alpha,
            },
            &FollowUpCommandKey::new("cmd-fu-donna-create").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_lifecycle_id.clone(),
                summary: FollowUpSummary::new("Recheck thyroid panel in 6 weeks").unwrap(),
                due_at: now_i64 + 86_400 * 42,
                records: vec![bmp_ref.clone()],
            },
            &mut authority,
        )
        .await
        .unwrap();
    store
        .change_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr_alpha.practice_id,
                physician: &dr_alpha.physician_id,
                workspace: &ws_fu_active,
                session_hash: &hash_alpha,
            },
            &FollowUpCommandKey::new("cmd-fu-donna-change").unwrap(),
            &PhysicianChangeFollowUp {
                id: fu_lifecycle_id.clone(),
                summary: FollowUpSummary::new("Recheck TSH and free T4 in 4 weeks").unwrap(),
                due_at: now_i64 + 86_400 * 28,
                records: vec![bmp_ref.clone()],
                note: FollowUpChangeNote::new("Moved earlier after dose adjustment").unwrap(),
            },
            &mut authority,
        )
        .await
        .unwrap();
    store
        .complete_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr_alpha.practice_id,
                physician: &dr_alpha.physician_id,
                workspace: &ws_fu_active,
                session_hash: &hash_alpha,
            },
            &FollowUpCommandKey::new("cmd-fu-donna-complete").unwrap(),
            &fu_lifecycle_id,
            FollowUpChangeNote::new("TSH normalized at 1.9 mIU/L").unwrap(),
            &mut authority,
        )
        .await
        .unwrap();

    let fu_cancel_id = FollowUpId::new("fu-donna-cancel").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr_alpha.practice_id,
                physician: &dr_alpha.physician_id,
                workspace: &ws_fu_active,
                session_hash: &hash_alpha,
            },
            &FollowUpCommandKey::new("cmd-fu-donna-cancel-create").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_cancel_id.clone(),
                summary: FollowUpSummary::new("Check outsideHolter report").unwrap(),
                due_at: now_i64 + 86_400 * 14,
                records: vec![],
            },
            &mut authority,
        )
        .await
        .unwrap();
    store
        .cancel_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr_alpha.practice_id,
                physician: &dr_alpha.physician_id,
                workspace: &ws_fu_active,
                session_hash: &hash_alpha,
            },
            &FollowUpCommandKey::new("cmd-fu-donna-cancel").unwrap(),
            &fu_cancel_id,
            FollowUpChangeNote::new("Holter cancelled by cardiology").unwrap(),
            &mut authority,
        )
        .await
        .unwrap();

    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr_alpha.practice_id,
                physician: &dr_alpha.physician_id,
                workspace: &ws_fu_active,
                session_hash: &hash_alpha,
            },
            &FollowUpCommandKey::new("cmd-fu-donna-still-open").unwrap(),
            &PhysicianCreateFollowUp {
                id: FollowUpId::new("fu-donna-still-open").unwrap(),
                summary: FollowUpSummary::new("Annual lipid recheck").unwrap(),
                due_at: now_i64 + 86_400 * 90,
                records: vec![],
            },
            &mut authority,
        )
        .await
        .unwrap();

    //    - `ws_fu_cannot_restart`: `CannotRestart` follow-up -> Today `UnhandledWork { work_kind: FollowUpReview }`.
    let fu_cant_id = FollowUpId::new("fu-wilfred-cant").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr_alpha.practice_id,
                physician: &dr_alpha.physician_id,
                workspace: &ws_fu_cannot_restart,
                session_hash: &hash_alpha,
            },
            &FollowUpCommandKey::new("cmd-fu-wilfred-create").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_cant_id.clone(),
                summary: FollowUpSummary::new("Verify outside hospital discharge summary").unwrap(),
                due_at: now_i64 - 600,
                records: vec![bmp_ref.clone()],
            },
            &mut authority,
        )
        .await
        .unwrap();
    store
        .apply_follow_up_review_outcome(
            &practice,
            &ws_fu_cannot_restart,
            &FollowUpCommandKey::new("cmd-fu-wilfred-cant").unwrap(),
            &fu_cant_id,
            SourceWork {
                job: "patient_review".into(),
                work_id: "work-fu-wilfred".into(),
                attempt_id: "att-fu-wilfred".into(),
            },
            FollowUpReviewOutcome::CannotRestart {
                failed_work_id: "work-fu-wilfred".into(),
                reason: "Exhausted automatic retries fetching discharge summary".into(),
            },
        )
        .await
        .unwrap();

    //    - `ws_fu_withdrawn`: `WithdrawnWrongPatient` follow-up -> excluded from BOTH Today AND Updates!
    let fu_withdrawn_id = FollowUpId::new("fu-river-wrong-pt").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr_alpha.practice_id,
                physician: &dr_alpha.physician_id,
                workspace: &ws_fu_withdrawn,
                session_hash: &hash_alpha,
            },
            &FollowUpCommandKey::new("cmd-fu-river-create").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_withdrawn_id,
                summary: FollowUpSummary::new("Wrong-patient outside lab check").unwrap(),
                due_at: now_i64 + 3_600,
                records: vec![bmp_ref.clone()],
            },
            &mut authority,
        )
        .await
        .unwrap();
    store
        .withdraw_wrong_patient_source(&WithdrawWrongPatientRequest {
            practice: practice.clone(),
            workspace: ws_fu_withdrawn.clone(),
            withdrawal_id: "wd-river-1".into(),
            actor: dr_alpha.physician_id.clone(),
            candidate_ids: vec!["obs-bmp-1".into()],
            message_id: None,
            reason: "Misfiled chart record belonged to another patient".into(),
            fenced_work_ids: Vec::new(),
        })
        .await
        .unwrap();

    // --- QUERY READBACK: TODAY (Dr. Alpha vs Dr. Beta) ---
    let names = BTreeMap::new();
    let today_alpha = store
        .read_today(auth.as_ref(), &token_alpha, now_u64, &names)
        .await
        .expect("read today for Dr. Alpha");
    assert_eq!(today_alpha.completeness, ReadCompleteness::Complete);

    let alpha_keys: Vec<&str> = today_alpha
        .rows
        .iter()
        .map(|r| r.item_key.as_str())
        .collect();
    // Urgent `MessageToMatch` MUST lead at index 0 (`is_now() == true`)!
    assert_eq!(
        today_alpha.rows[0].item_key.as_str(),
        "match:spruce-match-urgent"
    );
    assert!(today_alpha.rows[0].is_now());

    // Check every expected Today row is present:
    assert!(alpha_keys.contains(&format!("reply:{ws_thread}:conv-clara-thread").as_str()));
    assert!(alpha_keys.contains(&format!("reply:{ws_disconnected}:conv-danny-disc").as_str()));
    assert!(alpha_keys.contains(&"match:spruce-match-ordinary"));
    assert!(alpha_keys.contains(&"unreadable:spruce-unreadable-4"));
    assert!(alpha_keys.contains(&"channel:spruce"));
    assert!(alpha_keys.contains(&format!("unhandled:{ws_fu_cannot_restart}").as_str()));

    // Verify excluded items NEVER appear on Today:
    for forbidden in [
        "spruce-spam-3",
        "conv-vendor-nonpatient",
        &ws_fu_active,
        &ws_fu_withdrawn,
    ] {
        assert!(
            !alpha_keys.iter().any(|k| k.contains(forbidden)),
            "forbidden item `{forbidden}` must never appear on Today: {alpha_keys:?}"
        );
    }

    // Verify payload shapes on Today:
    let ordinary_match_alpha = today_alpha
        .rows
        .iter()
        .find(|r| r.item_key.as_str() == "match:spruce-match-ordinary")
        .unwrap();
    assert_eq!(ordinary_match_alpha.workspace, PRACTICE_MESSAGING_WORKSPACE);
    assert!(matches!(
        &ordinary_match_alpha.payload,
        TodayRowPayload::MessageToMatch {
            suggested_workspace: Some(sug),
            leads_as_now: false,
            ..
        } if sug == &ws_suggest_only_alpha
    ));

    // Rule-2 check on `dr_beta`: `dr_beta` sees `match:spruce-match-ordinary`, but `suggested_workspace` is `None`!
    let today_beta = store
        .read_today(auth.as_ref(), &token_beta, now_u64, &names)
        .await
        .expect("read today for Dr. Beta");
    let ordinary_match_beta = today_beta
        .rows
        .iter()
        .find(|r| r.item_key.as_str() == "match:spruce-match-ordinary")
        .expect("practice-wide unmatched message visible to Dr. Beta");
    assert!(matches!(
        &ordinary_match_beta.payload,
        TodayRowPayload::MessageToMatch {
            suggested_workspace: None,
            ..
        }
    ));

    let channel_row_transient = today_alpha
        .rows
        .iter()
        .find(|r| r.item_key.as_str() == "channel:spruce")
        .unwrap();
    assert!(matches!(
        &channel_row_transient.payload,
        TodayRowPayload::MessagingChannelProblem {
            problem_kind,
            ..
        } if problem_kind == "cannot_read"
    ));

    // Workspace-scoped Today (`Some(&ws_thread)`) includes `ws_thread`'s `MessageThread` and omits practice-wide Spruce rows!
    let ws_scoped_today = store
        .read_today_with_probe(
            auth.as_ref(),
            &token_alpha,
            now_u64,
            Some(&ws_thread),
            &names,
            &FailingSliceProbe {
                fail_producers: BTreeSet::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(ws_scoped_today.rows.len(), 1);
    assert_eq!(
        ws_scoped_today.rows[0].item_key.as_str(),
        format!("reply:{ws_thread}:conv-clara-thread")
    );

    // Flip Spruce channel to `RevokedOrBlocked` -> `reconnect_required`!
    store
        .record_spruce_channel_failure(
            &practice,
            &lease.lease_token,
            ChannelStopCause::RevokedOrBlocked {
                reason: "Spruce API credential revoked".into(),
            },
            now_i64 - 400,
        )
        .await
        .unwrap();
    let today_revoked = store
        .read_today(auth.as_ref(), &token_alpha, now_u64, &names)
        .await
        .unwrap();
    let revoked_banner = today_revoked
        .rows
        .iter()
        .find(|r| r.item_key.as_str() == "channel:spruce")
        .expect("reconnect banner");
    assert!(matches!(
        &revoked_banner.payload,
        TodayRowPayload::MessagingChannelProblem {
            problem_kind,
            ..
        } if problem_kind == "reconnect_required"
    ));

    // Verify `CannotRestart` follow-up row payload, open detail, and `Done` clearance + handoff!
    let cant_restart_row = today_alpha
        .rows
        .iter()
        .find(|r| r.workspace == ws_fu_cannot_restart)
        .expect("CannotRestart follow-up Today row")
        .clone();
    assert!(matches!(
        cant_restart_row.payload,
        TodayRowPayload::UnhandledWork {
            work_kind: UnhandledWorkKind::FollowUpReview,
            ..
        }
    ));
    let cant_detail = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token_alpha,
                real_now_unix: now_u64,
                workspace: &ws_fu_cannot_restart,
                item_key: &cant_restart_row.item_key,
                command_key: &CommandKey::new("cmd-open-cant-fu").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .expect("open CannotRestart follow-up row");
    assert!(cant_detail.clinical_body.contains("discharge summary"));
    assert!(matches!(
        cant_detail.seen_standing,
        SeenStanding::Seen { .. }
    ));

    let done_cant = store
        .clear_today_item_done(
            auth.as_ref(),
            &mut authority,
            &ClearItemDoneRequest {
                token: &token_alpha,
                real_now_unix: now_u64,
                workspace: &ws_fu_cannot_restart,
                item_key: &cant_restart_row.item_key,
                version_id: &cant_restart_row.version_id,
                command_key: &CommandKey::new("cmd-done-cant-fu").unwrap(),
            },
        )
        .await
        .expect("clear CannotRestart follow-up row with Done");
    assert!(matches!(done_cant, TodayDoneResult::Cleared { .. }));

    // Open & clear `UnreadableMessage` (`unreadable:spruce-unreadable-4`) and disconnected `MessageThread` with `Done`!
    let unreadable_row = today_alpha
        .rows
        .iter()
        .find(|r| r.item_key.as_str() == "unreadable:spruce-unreadable-4")
        .unwrap()
        .clone();
    assert!(unreadable_row.can_mark_done);
    let _ = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token_alpha,
                real_now_unix: now_u64,
                workspace: PRACTICE_MESSAGING_WORKSPACE,
                item_key: &unreadable_row.item_key,
                command_key: &CommandKey::new("cmd-open-unreadable-4").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .expect("open unreadable message detail");
    let done_unreadable = store
        .clear_today_item_done(
            auth.as_ref(),
            &mut authority,
            &ClearItemDoneRequest {
                token: &token_alpha,
                real_now_unix: now_u64,
                workspace: PRACTICE_MESSAGING_WORKSPACE,
                item_key: &unreadable_row.item_key,
                version_id: &unreadable_row.version_id,
                command_key: &CommandKey::new("cmd-done-unreadable-4").unwrap(),
            },
        )
        .await
        .expect("clear unreadable message Done");
    assert!(matches!(done_unreadable, TodayDoneResult::Cleared { .. }));
    assert!(matches!(
        store
            .spruce_message(&practice, "spruce-unreadable-4")
            .await
            .unwrap()
            .unwrap()
            .standing,
        MessageStanding::MarkedDone { .. }
    ));

    let disc_row = today_alpha
        .rows
        .iter()
        .find(|r| r.workspace == ws_disconnected)
        .unwrap()
        .clone();
    assert!(disc_row.can_mark_done);
    let done_disc = store
        .clear_today_item_done(
            auth.as_ref(),
            &mut authority,
            &ClearItemDoneRequest {
                token: &token_alpha,
                real_now_unix: now_u64,
                workspace: &ws_disconnected,
                item_key: &disc_row.item_key,
                version_id: &disc_row.version_id,
                command_key: &CommandKey::new("cmd-done-disc-thread").unwrap(),
            },
        )
        .await
        .expect("clear disconnected reply row Done");
    assert!(matches!(done_disc, TodayDoneResult::Cleared { .. }));

    let today_after_clears = store
        .read_today(auth.as_ref(), &token_alpha, now_u64, &names)
        .await
        .unwrap();
    assert!(
        !today_after_clears
            .rows
            .iter()
            .any(|r| r.item_key == cant_restart_row.item_key
                || r.item_key == unreadable_row.item_key
                || r.item_key == disc_row.item_key),
        "Done items must leave Today"
    );

    // Cleanly disconnecting the Spruce channel silences `channel:spruce` and purges unlinked `match:...` items (`Rule 11`),
    // while keeping the linked patient `MessageThread` (`ws_thread`) intact!
    store
        .disconnect_spruce_channel(auth.as_ref(), &dr_alpha, now_i64 - 300)
        .await
        .unwrap();
    let today_clean_disconnect = store
        .read_today(auth.as_ref(), &token_alpha, now_u64, &names)
        .await
        .unwrap();
    assert!(
        !today_clean_disconnect
            .rows
            .iter()
            .any(|r| r.item_key.as_str() == "channel:spruce"
                || r.item_key.as_str().starts_with("match:")),
        "cleanly disconnected Spruce channel must silence banner and unlinked match rows"
    );

    // Open the active linked `MessageThread` row (`reply:{ws_thread}:conv-clara-thread`) and verify citations + draft body.
    let thread_row = today_after_clears
        .rows
        .iter()
        .find(|r| r.workspace == ws_thread)
        .expect("active linked MessageThread row")
        .clone();
    assert!(
        !thread_row.can_mark_done,
        "connected active MessageThread cannot be marked Done directly"
    );
    let thread_detail = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token_alpha,
                real_now_unix: now_u64,
                workspace: &ws_thread,
                item_key: &thread_row.item_key,
                command_key: &CommandKey::new("cmd-open-clara-thread").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .expect("open linked MessageThread detail");
    assert!(thread_detail.clinical_body.contains("4.3 mmol/L"));
    assert_eq!(thread_detail.citations.len(), 1);

    // --- QUERY READBACK: UPDATES ---
    let updates_alpha = store
        .read_updates(auth.as_ref(), &token_alpha, now_u64, &names)
        .await
        .expect("read updates for Dr. Alpha");
    assert_eq!(updates_alpha.completeness, ReadCompleteness::Complete);

    let all_updates: Vec<_> = updates_alpha
        .today_items
        .iter()
        .chain(
            updates_alpha
                .earlier_days
                .iter()
                .flat_map(|d| d.items.iter()),
        )
        .collect();

    // 1. `ReplyDrafted` reaches Updates and opens cleanly.
    let reply_update = all_updates
        .iter()
        .find(|u| {
            matches!(
                &u.kind,
                UpdateEntryKind::ReplyDrafted {
                    conversation_id,
                    proposal_version: 1,
                    ..
                } if conversation_id == "conv-clara-thread"
            )
        })
        .expect("ReplyDrafted update entry");
    let reply_update_detail = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token_alpha,
                real_now_unix: now_u64,
                workspace: &reply_update.workspace,
                item_key: &reply_update.item_key,
                command_key: &CommandKey::new("cmd-open-reply-update").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .expect("open ReplyDrafted update detail");
    assert!(reply_update_detail.clinical_body.contains("4.3 mmol/L"));

    // 2. `FollowUpChanged` reaches Updates for `created`, `changed`, `completed`, and `cancelled`,
    //    while `WithdrawnWrongPatient` (`ws_fu_withdrawn`) NEVER appears!
    for expected_kind in ["created", "changed", "completed", "cancelled"] {
        let entry = all_updates
            .iter()
            .find(|u| {
                matches!(
                    &u.kind,
                    UpdateEntryKind::FollowUpChanged { change_kind, .. }
                    if change_kind == expected_kind
                )
            })
            .unwrap_or_else(|| panic!("missing FollowUpChanged({expected_kind}) in Updates"));
        let opened = store
            .open_today_or_update_item(
                auth.as_ref(),
                &mut authority,
                &OpenItemRequest {
                    token: &token_alpha,
                    real_now_unix: now_u64,
                    workspace: &entry.workspace,
                    item_key: &entry.item_key,
                    command_key: &CommandKey::new(format!("cmd-open-fu-upd-{expected_kind}"))
                        .unwrap(),
                    patient_names: &names,
                },
            )
            .await
            .expect("open FollowUpChanged update detail");
        assert!(!opened.headline.is_empty());
    }
    assert!(
        !all_updates.iter().any(|u| u.workspace == ws_fu_withdrawn),
        "withdrawn wrong-patient follow-up must never appear in Updates"
    );

    // 3. Partial-failure isolation when `Messages` or `FollowUps` producer fails:
    let partial_today = store
        .read_today_with_probe(
            auth.as_ref(),
            &token_alpha,
            now_u64,
            None,
            &names,
            &FailingSliceProbe {
                fail_producers: BTreeSet::from([ProducerKind::Messages, ProducerKind::FollowUps]),
            },
        )
        .await
        .unwrap();
    assert!(matches!(
        partial_today.completeness,
        ReadCompleteness::Partial { ref failed_parts } if failed_parts.len() == 2
    ));
}

#[tokio::test]
async fn practice_message_does_not_clear_reply_row_until_both_reply_and_concern_are_handled() {
    let Some((store, auth, _cluster)) = connect_stores().await else {
        return;
    };

    let now_u64 = current_unix();
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-tumf-ac2-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr, token, hash) = invite_and_sign_in(&auth, &practice, "Dr. JointClose", now_u64).await;
    let ws = format!("pw-joint-close-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &ws).await.unwrap();
    auth.set_membership(&dr, &ws, Membership::Allowed)
        .await
        .unwrap();

    let k_ref = seed_chart_record(
        &store,
        &ws,
        "Elena Vance",
        "obs-k-joint",
        "rev-1",
        "Serum potassium 5.5 mmol/L on lisinopril 20 mg daily; repeat BMP advised.",
        now_i64 - 2_000,
    )
    .await;

    let conv_id = "conv-elena-joint";
    store
        .link_reply_conversation(
            &practice,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: ws.clone(),
            },
        )
        .await
        .unwrap();

    // --- Step 1: Before practice message receipt (`m1` and `m2` open) ---
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "m1-refill".into(),
            sender: MessageSender::Patient,
            text: "Did you send my amlodipine refill to Walgreens?".into(),
            sent_at: now_i64 - 1_500,
        })
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "m2-potassium".into(),
            sender: MessageSender::Patient,
            text: "Also I saw a high potassium alert in my lab portal — what should I do?".into(),
            sent_at: now_i64 - 1_400,
        })
        .await
        .unwrap();

    let names = BTreeMap::new();
    let today_before_receipt = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(today_before_receipt.rows.len(), 1);
    assert!(matches!(
        &today_before_receipt.rows[0].payload,
        TodayRowPayload::MessageThread {
            conversation_id,
            open_message_count: 2,
            reply_ready: false,
            open_concern_ids,
            ..
        } if conversation_id == conv_id && open_concern_ids.is_empty()
    ));

    // --- Step 2: Practice sends non-automatic message `pm1` answering ONLY `m1` ---
    // Receipt of `pm1` MUST NOT clear the Today reply row on its own, and `pm1` MUST NOT appear in Updates!
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            practice_message_id: "pm1-staff-refill".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "Hi Elena, we sent your 90-day amlodipine refill to Walgreens on Main St.".into(),
            sent_at: now_i64 - 1_200,
        })
        .await
        .unwrap();

    let today_after_receipt = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(
        today_after_receipt.rows.len(),
        1,
        "practice message receipt alone must NEVER clear the Today reply row"
    );
    assert!(matches!(
        &today_after_receipt.rows[0].payload,
        TodayRowPayload::MessageThread {
            conversation_id,
            open_message_count: 2,
            reply_ready: false,
            ..
        } if conversation_id == conv_id
    ));
    let updates_after_receipt = store
        .read_updates(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert!(
        updates_after_receipt.today_items.is_empty(),
        "observed practice messages must never create Updates entries: {:?}",
        updates_after_receipt.today_items
    );

    // --- Step 3: Review handles partial answer (`m1` closed by `pm1`), leaves `m2` open with a proposed draft reply, AND raises open concern `c1` ---
    let c1_id = ConcernId::new("concern-elena-k").unwrap();
    store
        .raise_concern(
            &CommandKey::new("cmd-raise-elena-k").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: ws.clone(),
                id: c1_id.clone(),
                urgency: ConcernUrgency::Today,
                reason: "Potassium 5.5 mmol/L on lisinopril 20 mg; patient asking for guidance."
                    .into(),
                records: vec![k_ref.clone()],
                source_work: SourceWork {
                    job: "patient_review".into(),
                    work_id: "rev-elena-1".into(),
                    attempt_id: "att-elena-1".into(),
                },
            },
        )
        .await
        .unwrap();

    let _ = store
        .start_or_read_reply_review(&practice, conv_id, "rev-elena-1", "run-elena-1")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            review_id: "rev-elena-1".into(),
            run_id: "run-elena-1".into(),
            completion_key: "comp-elena-partial".into(),
            answered_closures: vec![AnsweredMessageClosure {
                practice_message_id: "pm1-staff-refill".into(),
                answered_message_ids: BTreeSet::from(["m1-refill".to_owned()]),
            }],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Your potassium was mildly elevated at 5.5 mmol/L; please repeat a morning lab draw this week and avoid potassium supplements.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Reviewed metabolic panel obs-k-joint rev-1.".into(),
                    limits: vec![],
                    cited_records: vec![k_ref.clone()],
                },
                cited_records: vec![k_ref.clone()],
            }),
            raised_concern_ids: vec![c1_id.clone()],
        })
        .await
        .unwrap();

    let today_partial = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(
        today_partial.rows.len(),
        1,
        "associated open concern must stay attached to the single MessageThread row without duplicating: {:?}",
        today_partial.rows
    );
    assert!(matches!(
        &today_partial.rows[0].payload,
        TodayRowPayload::MessageThread {
            conversation_id,
            open_message_count: 1,
            reply_ready: true,
            open_concern_ids,
            ..
        } if conversation_id == conv_id && open_concern_ids == &vec![c1_id.clone()]
    ));

    // --- Step 4: Practice sends `pm2` answering `m2`, and review closes `m2` (`AllHandled`), while concern `c1` is STILL open ---
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            practice_message_id: "pm2-dr-potassium".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "Elena, please come in for a repeat potassium check tomorrow morning and hold potassium supplements.".into(),
            sent_at: now_i64 - 800,
        })
        .await
        .unwrap();
    let _ = store
        .start_or_read_reply_review(&practice, conv_id, "rev-elena-2", "run-elena-2")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            review_id: "rev-elena-2".into(),
            run_id: "run-elena-2".into(),
            completion_key: "comp-elena-all-msgs".into(),
            answered_closures: vec![AnsweredMessageClosure {
                practice_message_id: "pm2-dr-potassium".into(),
                answered_message_ids: BTreeSet::from(["m2-potassium".to_owned()]),
            }],
            decision: ReplyReviewDecision::AllHandled,
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    let today_concern_still_open = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(
        today_concern_still_open.rows.len(),
        1,
        "row must remain open while associated concern is still open even after all messages/reply are handled"
    );
    assert!(matches!(
        &today_concern_still_open.rows[0].payload,
        TodayRowPayload::MessageThread {
            conversation_id,
            open_message_count: 0,
            reply_ready: false,
            open_concern_ids,
            ..
        } if conversation_id == conv_id && open_concern_ids == &vec![c1_id.clone()]
    ));

    // --- Step 5: Physician marks concern `c1` Done -> BOTH reply and concern are now handled, so the row closes! ---
    let mut authority = PostgresWorkspaceAuthority;
    store
        .mark_concern_done(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws,
                session_hash: &hash,
            },
            &CommandKey::new("cmd-done-elena-k").unwrap(),
            &c1_id,
            &mut authority,
        )
        .await
        .unwrap();

    let today_both_handled = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert!(
        today_both_handled.rows.is_empty() && today_both_handled.nothing_needs_physician(),
        "once both reply and associated concern are handled, the Today row must close: {:?}",
        today_both_handled.rows
    );

    // --- Step 6: Symmetric order (concern closed FIRST while reply remains open, then reply handled) ---
    let c2_id = ConcernId::new("concern-elena-followup").unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "m3-fasting".into(),
            sender: MessageSender::Patient,
            text: "Do I need to fast before tomorrow's potassium blood draw?".into(),
            sent_at: now_i64 - 400,
        })
        .await
        .unwrap();
    store
        .raise_concern(
            &CommandKey::new("cmd-raise-elena-c2").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: ws.clone(),
                id: c2_id.clone(),
                urgency: ConcernUrgency::Today,
                reason: "Confirm standing lab order for tomorrow's repeat potassium.".into(),
                records: vec![k_ref.clone()],
                source_work: SourceWork {
                    job: "patient_review".into(),
                    work_id: "rev-elena-3".into(),
                    attempt_id: "att-elena-3".into(),
                },
            },
        )
        .await
        .unwrap();
    let _ = store
        .start_or_read_reply_review(&practice, conv_id, "rev-elena-3", "run-elena-3")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            review_id: "rev-elena-3".into(),
            run_id: "run-elena-3".into(),
            completion_key: "comp-elena-m3".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "No fasting is needed for a serum potassium check.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Reviewed lab order requirements.".into(),
                    limits: vec![],
                    cited_records: vec![k_ref],
                },
                cited_records: vec![],
            }),
            raised_concern_ids: vec![c2_id.clone()],
        })
        .await
        .unwrap();

    // Physician clears concern `c2` FIRST while reply draft for `m3` is still `Ready`:
    store
        .mark_concern_done(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws,
                session_hash: &hash,
            },
            &CommandKey::new("cmd-done-elena-c2").unwrap(),
            &c2_id,
            &mut authority,
        )
        .await
        .unwrap();

    let today_reply_still_open = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(
        today_reply_still_open.rows.len(),
        1,
        "row must remain open while reply is still open even after concern is closed"
    );
    assert!(matches!(
        &today_reply_still_open.rows[0].payload,
        TodayRowPayload::MessageThread {
            open_message_count: 1,
            reply_ready: true,
            open_concern_ids,
            ..
        } if open_concern_ids.is_empty()
    ));

    // Now practice sends `pm3` and review closes `m3` (`AllHandled`) -> row closes!
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            practice_message_id: "pm3-no-fasting".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "No fasting needed — you can eat and drink water normally.".into(),
            sent_at: now_i64 - 200,
        })
        .await
        .unwrap();
    let _ = store
        .start_or_read_reply_review(&practice, conv_id, "rev-elena-4", "run-elena-4")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            review_id: "rev-elena-4".into(),
            run_id: "run-elena-4".into(),
            completion_key: "comp-elena-m3-closed".into(),
            answered_closures: vec![AnsweredMessageClosure {
                practice_message_id: "pm3-no-fasting".into(),
                answered_message_ids: BTreeSet::from(["m3-fasting".to_owned()]),
            }],
            decision: ReplyReviewDecision::AllHandled,
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    let today_final = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert!(today_final.rows.is_empty());
}

#[tokio::test]
async fn channel_sender_display_disconnected_done_with_open_concern_and_message_seen_snapshots() {
    let Some((store, auth, _cluster)) = connect_stores().await else {
        return;
    };

    let now_u64 = current_unix();
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-tumf-reg-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr, token, hash) = invite_and_sign_in(&auth, &practice, "Dr. SenderSeen", now_u64).await;
    let ws_linked = format!("pw-linked-sender-{}", Uuid::new_v4().simple());
    let ws_suggested = format!("pw-sug-sender-{}", Uuid::new_v4().simple());
    let ws_disc_concern = format!("pw-disc-conc-{}", Uuid::new_v4().simple());

    for (ws, chart_name) in [
        (&ws_linked, "Clara Oswald"),
        (&ws_suggested, "Martha Jones"),
        (&ws_disc_concern, "Danny Pink"),
    ] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
        let _ = seed_chart_record(
            &store,
            ws,
            chart_name,
            "obs-bmp-reg",
            "rev-1",
            "Serum potassium 4.4 mmol/L, creatinine 0.90 mg/dL.",
            now_i64 - 1_800,
        )
        .await;
    }

    let bmp_ref = ExactRecordReference::new("obs-bmp-reg", "rev-1").unwrap();
    let mut authority = PostgresWorkspaceAuthority;

    // Pass explicit chart patient names so any accidental lookup of `names.get(ws)` would return
    // `"Clara Oswald"` / `"Martha Jones"` / `"Danny Pink"` instead of the channel's sender!
    let names = BTreeMap::from([
        (ws_linked.clone(), "Clara Oswald".to_owned()),
        (ws_suggested.clone(), "Martha Jones".to_owned()),
        (ws_disc_concern.clone(), "Danny Pink".to_owned()),
    ]);

    // 1. Connect Spruce channel and ingest:
    //    (a) Linked caregiver message on `ws_linked` from `"Rory Williams (Spouse)"` + draft reply proposal
    //    (b) Unmatched message suggesting `ws_suggested` from `"+14155550922"` (no `author_name`)
    //    (c) Unreadable message `spruce-unreadable-reg`
    //    (d) Transient channel problem `channel:spruce`
    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, now_i64 - 1_500)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-tumf-reg", 3_600, now_i64 - 1_500)
        .await
        .unwrap();
    let rory_phone = SenderAddress::parse("+14155550911").unwrap();
    store
        .record_spruce_confirmed_sender(
            &practice,
            &rory_phone,
            &ws_linked,
            &dr.physician_id,
            now_i64 - 1_500,
        )
        .await
        .unwrap();
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "spruce-rory-1".into(),
                conversation_id: "conv-rory-clara".into(),
                channel_position: 1,
                content_version: "v1".into(),
                sender_address: Some(rory_phone.clone()),
                author_name: Some("Rory Williams (Spouse)".into()),
                body_text: "Clara asked me to check whether her lab panel looked okay.".into(),
                attachments: Vec::new(),
                app_url: None,
                created_at_spruce: None,
                outside_participants: vec![OutsideParticipant {
                    display_name: Some("Rory Williams (Spouse)".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(rory_phone),
                    saved_contact_id: None,
                }],
            }),
            now_i64 - 900,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "spruce-rory-1",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: ws_linked.clone(),
                    evidence_summary: "Confirmed family thread for Clara Oswald.".into(),
                },
                rationale: "Spouse lab question.".into(),
            },
            None,
            now_i64 - 895,
        )
        .await
        .unwrap();

    // Sync via `read_today` once so `conv-rory-clara` exists in `reply_conversation`, then save a reply proposal!
    let _ = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    let _ = store
        .start_or_read_reply_review(&practice, "conv-rory-clara", "rev-rory-1", "run-rory-1")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: "conv-rory-clara".into(),
            review_id: "rev-rory-1".into(),
            run_id: "run-rory-1".into(),
            completion_key: "comp-rory-1".into(),
            answered_closures: Vec::new(),
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Clara's metabolic panel is normal with potassium at 4.4 mmol/L.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Reviewed Regional EHR metabolic panel.".into(),
                    limits: Vec::new(),
                    cited_records: vec![bmp_ref.clone()],
                },
                cited_records: vec![bmp_ref.clone()],
            }),
            raised_concern_ids: Vec::new(),
        })
        .await
        .unwrap();

    // (b) Unmatched message suggesting `ws_suggested` ("Martha Jones"), sent from `+14155550922` with `author_name: None`.
    let martha_work_phone = SenderAddress::parse("+14155550922").unwrap();
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "spruce-match-phone-only".into(),
                conversation_id: "conv-match-phone-only".into(),
                channel_position: 2,
                content_version: "v1".into(),
                sender_address: Some(martha_work_phone.clone()),
                author_name: None,
                body_text: "Texting from a new number — can we refill my inhaler?".into(),
                attachments: Vec::new(),
                app_url: None,
                created_at_spruce: None,
                outside_participants: vec![OutsideParticipant {
                    display_name: None,
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(martha_work_phone),
                    saved_contact_id: None,
                }],
            }),
            now_i64 - 800,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "spruce-match-phone-only",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: ws_suggested.clone(),
                    evidence_summary: "Matches inhaler prescription history.".into(),
                },
                rationale: "Unconfirmed phone number.".into(),
            },
            None,
            now_i64 - 795,
        )
        .await
        .unwrap();

    // (c) Unreadable message + (d) transient channel error.
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Unreadable(UnreadableSpruceItem {
                message_id: "spruce-unreadable-reg".into(),
                conversation_id: "conv-unreadable-reg".into(),
                channel_position: 3,
                content_version: "v-err-reg".into(),
                failure_reason: "Encrypted MMS attachment decode failed".into(),
                app_url: Some("https://app.sprucehealth.com/items/reg-3".into()),
                outside_participants: Vec::new(),
            }),
            now_i64 - 700,
        )
        .await
        .unwrap();
    store
        .record_spruce_channel_failure(
            &practice,
            &lease.lease_token,
            ChannelStopCause::Transient {
                reason: "Spruce webhook timeout".into(),
            },
            now_i64 - 600,
        )
        .await
        .unwrap();

    // 2. Disconnected conversation on `ws_disc_concern` with BOTH an open `Ready` reply AND an open `ConcernId`!
    let disc_concern_id = ConcernId::new("concern-danny-disc-open").unwrap();
    store
        .raise_concern(
            &CommandKey::new("cmd-raise-danny-disc-open").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: ws_disc_concern.clone(),
                id: disc_concern_id.clone(),
                urgency: ConcernUrgency::Today,
                reason: "Follow up blood pressure cuff reading after channel disconnect.".into(),
                records: vec![bmp_ref.clone()],
                source_work: SourceWork {
                    job: "patient_review".into(),
                    work_id: "rev-danny-disc-1".into(),
                    attempt_id: "att-danny-disc-1".into(),
                },
            },
        )
        .await
        .unwrap();
    store
        .link_reply_conversation(
            &practice,
            "conv-danny-disc-concern",
            ConversationLink::LinkedPatient {
                workspace: ws_disc_concern.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-danny-disc-concern".into(),
            message_id: "msg-danny-disc-1".into(),
            sender: MessageSender::NamedSender {
                name: "Danny Pink (Mobile)".into(),
            },
            text: "My home cuff read 152/94 this evening.".into(),
            sent_at: now_i64 - 500,
        })
        .await
        .unwrap();
    let _ = store
        .start_or_read_reply_review(
            &practice,
            "conv-danny-disc-concern",
            "rev-danny-disc-1",
            "run-danny-disc-1",
        )
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: "conv-danny-disc-concern".into(),
            review_id: "rev-danny-disc-1".into(),
            run_id: "run-danny-disc-1".into(),
            completion_key: "comp-danny-disc-1".into(),
            answered_closures: Vec::new(),
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Please recheck your blood pressure after resting 10 minutes.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Reviewed chart baseline.".into(),
                    limits: Vec::new(),
                    cited_records: vec![bmp_ref],
                },
                cited_records: Vec::new(),
            }),
            raised_concern_ids: vec![disc_concern_id.clone()],
        })
        .await
        .unwrap();
    store
        .set_reply_channel_connection(
            &practice,
            "conv-danny-disc-concern",
            ChannelConnection::Disconnected {
                disconnected_at: now_i64 - 450,
                note: "Spruce disconnected.".into(),
            },
        )
        .await
        .unwrap();

    // --- VERIFY 1: Channel sender display names on `read_today` AND `open_today_or_update_item` ---
    let today1 = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();

    let linked_row = today1
        .rows
        .iter()
        .find(|r| r.item_key.as_str() == format!("reply:{ws_linked}:conv-rory-clara"))
        .expect("linked caregiver MessageThread row")
        .clone();
    assert_eq!(
        linked_row.patient_display_name, "Rory Williams (Spouse)",
        "MessageThread row must display channel sender, never linked patient chart name `Clara Oswald`"
    );

    let match_row = today1
        .rows
        .iter()
        .find(|r| r.item_key.as_str() == "match:spruce-match-phone-only")
        .expect("unmatched phone-only MessageToMatch row")
        .clone();
    assert_eq!(
        match_row.patient_display_name, "+14155550922",
        "MessageToMatch row must display channel sender number, never suggested patient chart name `Martha Jones`"
    );

    let unreadable_row = today1
        .rows
        .iter()
        .find(|r| r.item_key.as_str() == "unreadable:spruce-unreadable-reg")
        .unwrap()
        .clone();
    let channel_row = today1
        .rows
        .iter()
        .find(|r| r.item_key.as_str() == "channel:spruce")
        .unwrap()
        .clone();

    // Open every message item kind and verify BOTH `patient_display_name` AND subsequent `SeenStanding::Seen`!
    let opened_linked = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &ws_linked,
                item_key: &linked_row.item_key,
                command_key: &CommandKey::new("cmd-open-reg-linked").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .unwrap();
    assert_eq!(opened_linked.patient_display_name, "Rory Williams (Spouse)");

    let opened_match = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: PRACTICE_MESSAGING_WORKSPACE,
                item_key: &match_row.item_key,
                command_key: &CommandKey::new("cmd-open-reg-match").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .unwrap();
    assert_eq!(opened_match.patient_display_name, "+14155550922");

    let opened_unreadable = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: PRACTICE_MESSAGING_WORKSPACE,
                item_key: &unreadable_row.item_key,
                command_key: &CommandKey::new("cmd-open-reg-unreadable").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .unwrap();
    assert_eq!(opened_unreadable.patient_display_name, "Practice messaging");

    let opened_channel = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: PRACTICE_MESSAGING_WORKSPACE,
                item_key: &channel_row.item_key,
                command_key: &CommandKey::new("cmd-open-reg-channel").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .unwrap();
    assert_eq!(opened_channel.patient_display_name, "Practice messaging");

    // Also open `reply-proposal:` in `Updates`!
    let updates1 = store
        .read_updates(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    let reply_upd = updates1
        .today_items
        .iter()
        .chain(updates1.earlier_days.iter().flat_map(|d| d.items.iter()))
        .find(|u| {
            matches!(
                &u.kind,
                UpdateEntryKind::ReplyDrafted { conversation_id, .. }
                if conversation_id == "conv-rory-clara"
            )
        })
        .expect("ReplyDrafted update entry")
        .clone();
    assert!(matches!(reply_upd.seen_standing, SeenStanding::Unseen));
    let _ = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &ws_linked,
                item_key: &reply_upd.item_key,
                command_key: &CommandKey::new("cmd-open-reg-reply-upd").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .unwrap();

    // --- VERIFY 2: Subsequent `read_today` and `read_updates` report `SeenStanding::Seen` (never `Unseen` or `UpdatedSinceSeen`)! ---
    let today_after_open = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    for expected_seen_key in [
        linked_row.item_key.as_str(),
        match_row.item_key.as_str(),
        unreadable_row.item_key.as_str(),
        channel_row.item_key.as_str(),
    ] {
        let row = today_after_open
            .rows
            .iter()
            .find(|r| r.item_key.as_str() == expected_seen_key)
            .unwrap_or_else(|| panic!("missing row `{expected_seen_key}` after open"));
        assert!(
            matches!(row.seen_standing, SeenStanding::Seen { .. }),
            "opened Today item `{expected_seen_key}` must transition to SeenStanding::Seen, got: {:?}",
            row.seen_standing
        );
    }

    let updates_after_open = store
        .read_updates(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    let reply_upd_after = updates_after_open
        .today_items
        .iter()
        .chain(
            updates_after_open
                .earlier_days
                .iter()
                .flat_map(|d| d.items.iter()),
        )
        .find(|u| u.item_key == reply_upd.item_key)
        .expect("ReplyDrafted update entry after open");
    assert!(
        matches!(reply_upd_after.seen_standing, SeenStanding::Seen { .. }),
        "opened ReplyDrafted Updates item must transition to SeenStanding::Seen, got: {:?}",
        reply_upd_after.seen_standing
    );

    // --- VERIFY 3: Disconnected open reply `Done` (`MessageReply.tla` Rule 3) closes ONLY the reply, leaving open concerns open! ---
    let disc_row_step1 = today_after_open
        .rows
        .iter()
        .find(|r| r.workspace == ws_disc_concern)
        .expect("disconnected reply + open concern row")
        .clone();
    assert!(disc_row_step1.can_mark_done);
    assert_eq!(disc_row_step1.patient_display_name, "Danny Pink (Mobile)");
    assert!(matches!(
        &disc_row_step1.payload,
        TodayRowPayload::MessageThread {
            reply_ready: true,
            channel_disconnected: true,
            open_concern_ids,
            ..
        } if open_concern_ids == &vec![disc_concern_id.clone()]
    ));

    // First `Done` click closes ONLY the disconnected open reply!
    let done_step1 = store
        .clear_today_item_done(
            auth.as_ref(),
            &mut authority,
            &ClearItemDoneRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &ws_disc_concern,
                item_key: &disc_row_step1.item_key,
                version_id: &disc_row_step1.version_id,
                command_key: &CommandKey::new("cmd-done-disc-step1-reply-only").unwrap(),
            },
        )
        .await
        .expect("first Done closes disconnected reply only");
    assert!(matches!(done_step1, TodayDoneResult::Cleared { .. }));

    let conv_after_step1 = store
        .reply_conversation(&practice, "conv-danny-disc-concern")
        .await
        .unwrap()
        .unwrap();
    assert!(
        !conv_after_step1.reply_standing.is_open(),
        "disconnected reply must be closed by first Done"
    );

    let today_after_step1 = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    let disc_row_step2 = today_after_step1
        .rows
        .iter()
        .find(|r| r.workspace == ws_disc_concern)
        .expect("open concern must keep row on Today after disconnected reply Done")
        .clone();
    assert_eq!(disc_row_step2.title, "Message concern still open");
    assert!(disc_row_step2.can_mark_done);
    assert!(matches!(
        &disc_row_step2.payload,
        TodayRowPayload::MessageThread {
            reply_ready: false,
            open_message_count: 0,
            open_concern_ids,
            ..
        } if open_concern_ids == &vec![disc_concern_id.clone()]
    ));

    // Second `Done` click (now that reply is closed and only open concern remains) marks the concern `Done` and clears the row!
    let done_step2 = store
        .clear_today_item_done(
            auth.as_ref(),
            &mut authority,
            &ClearItemDoneRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &ws_disc_concern,
                item_key: &disc_row_step2.item_key,
                version_id: &disc_row_step2.version_id,
                command_key: &CommandKey::new("cmd-done-disc-step2-concern").unwrap(),
            },
        )
        .await
        .expect("second Done closes remaining open concern");
    assert!(matches!(done_step2, TodayDoneResult::Cleared { .. }));
    let _ = hash;

    let today_after_step2 = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert!(
        !today_after_step2
            .rows
            .iter()
            .any(|r| r.workspace == ws_disc_concern),
        "row must clear once both disconnected reply and open concern are handled"
    );
}
