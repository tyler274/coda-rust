//! HTTP handlers and durable `Store` composition for Spruce `Messages to match`,
//! `Filtered`, unreadable channel items, and patient-scoped Spruce reply cards,
//! private drafts, Coda revisions, copy attributions, and conversation moves.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, UNIX_EPOCH};

use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::extract::{Extension, Path};
use axum::http::{Extensions, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::SourceRevision;
use local_product::runtime::concerns::{CommandKey, WorkspaceAccess};
use local_product::runtime::dispatch::{PracticeJobReviewReceiver, ReviewReceiver};
use local_product::runtime::history::format_cited_source_body;
use local_product::runtime::jobs::{PracticeAccessError, PracticeAuthority, PracticeScope};
use local_product::runtime::reply::RevisionInput;
use local_product::runtime::reply::proposals::{
    CardArrivalNotice, CardChannelBanner, CardReviewState, ChannelConnection, ConversationLink,
    DisplayedReplyText, ExpectedDisconnectedCard, ExpectedDraftBase, LinkedPatientMessage,
    MessageSender, ObservedPracticeMessage, PhysicianDraftKind, PhysicianDraftStanding,
    PhysicianReplySurface, ProposalRevisionAccepted, RecordPatientMessage, ReplyCardView,
    ReplyConversation, ReplyProposalBasis, ReplyProposalError, ReplyStanding,
};
use local_product::runtime::spruce_matching::{
    ChannelHealth, ChannelStanding, ConversationLinkState, MatchAssessment, MatchingDecision,
    MessageRoutingOutcome, MessageStanding, MessageUrgency, MoveDestination,
    MoveSpruceConversationCommand, NonPatientCategory, PendingMoveUndo, SavedSpruceConversation,
    SavedSpruceMessage, SenderConfirmationPolicy, SpruceAttachmentSummary, SpruceMatchingError,
    UnmatchedMessageRow,
};
use local_product::runtime::understanding::SummaryCitation;
use local_product::runtime::{Authority, PublicationTransaction, Scope, Store, async_trait};
use local_product::{ModelBinding, ModelRoles};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use physician_wire::matching_reply::{
    CandidatePatientWire, CardArrivalNoticeWire, CardChannelBannerWire, CardReviewStateWire,
    ChannelSenderRoleWire, ChooseCodaReplyUpdateBody, CodaUpdateOfferWire, ConfirmMessageLinkBody,
    ConfirmMessageLinkResponseWire, CopyAttributionWire, CopyReplyBody, CopyReplyResponseWire,
    DisconnectedCardStandingWire, DisconnectedReplyDoneBody, DisplayedReplyTextWire,
    ExpectedDraftBaseWire, FilteredMessageWire, FilteredMessagesResponseWire, MatchSuggestionWire,
    MatchedMessageStandingWire, MatchingActionCommandBody, MatchingMessageStatusResponseWire,
    MessagesToMatchResponseWire, MoveDestinationWire, MoveSpruceConversationBody,
    NeedsChoiceReasonWire, NoReplyNeededBody, NonPatientCategoryWire, PatientReplyConversationWire,
    PatientReplyListResponseWire, PendingMoveTargetDisplayWire, PendingSpruceMoveWire,
    PhysicianDraftKindWire, PracticeMessageKindWire, ReplyCardWire, ReplyCommandKeyBody,
    ReplyPatientMessageWire, ReplyPracticeMessageWire, RequestReplyProposalRevisionBody,
    RestoreMyReplyVersionBody, SaveReplyProposalDraftBody, SenderConfirmationPolicyWire,
    SpruceAttachmentWire, SpruceChannelBannerWire, ThisIsAPatientOutcomeWire, UnmatchedMessageWire,
    UnmatchedUrgencyWire, UnreadableSpruceMessageWire,
};
use physician_wire::today_updates::{FailedProducerWire, ItemCitationWire, ProducerSliceWire};
use serde_json::json;

use crate::auth::{self, AuthContext};
use crate::care::ConnectedCare;
use crate::live::Live;
use crate::patients::{Patient, PatientDirectory, read_patients};
use crate::timestamp::format_rfc3339;

/// Closed target slice for test-only fault injection across Spruce matching,
/// reply editing, patient search, and patient follow-ups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MatchingReplyFollowUpsFault {
    /// Fail `GET /spruce-matching/messages` and `GET /spruce-matching/filtered`.
    MatchingRead,
    /// Fail `POST /spruce-matching/*` mutations.
    MatchingCommand,
    /// Fail `GET /patient-roster` lookup inside `Choose another patient` / `Wrong patient?`.
    PatientLookup,
    /// Fail `GET /patient-workspaces/{id}/spruce-replies*` reads.
    ReplyRead,
    /// Fail `PUT`/`POST` `/patient-workspaces/{id}/spruce-replies/*` mutations.
    ReplyCommand,
    /// Fail `GET /patient-workspaces/{id}/follow-ups` reads.
    FollowUpRead,
    /// Fail `POST`/`PUT` `/patient-workspaces/{id}/follow-ups*` mutations.
    FollowUpCommand,
    /// Fail discussion-chat seeding inside `POST /patient-workspaces/{id}/follow-ups/{fid}/stop-and-discuss`.
    StopAndDiscussChat,
}

/// Test-only fault-injection controller for browser and integration qualification.
/// Production routers never attach this extension.
#[derive(Default)]
pub struct MatchingReplyFollowUpsFaultInjection {
    active: RwLock<BTreeSet<MatchingReplyFollowUpsFault>>,
}

impl MatchingReplyFollowUpsFaultInjection {
    /// Creates a healthy fault-injection controller with zero active faults.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Activates `fault` on subsequent requests.
    pub fn fail(&self, fault: MatchingReplyFollowUpsFault) {
        self.active.write().expect("fault lock").insert(fault);
    }

    /// Clears a single `fault`.
    pub fn clear_fault(&self, fault: MatchingReplyFollowUpsFault) {
        self.active.write().expect("fault lock").remove(&fault);
    }

    /// Clears all active faults.
    pub fn clear_all(&self) {
        self.active.write().expect("fault lock").clear();
    }

    fn contains(&self, fault: MatchingReplyFollowUpsFault) -> bool {
        self.active.read().expect("fault lock").contains(&fault)
    }
}

/// Installs `faults` as an Axum extension layer on a test router.
pub fn fault_layer(
    faults: Arc<MatchingReplyFollowUpsFaultInjection>,
) -> Extension<Arc<MatchingReplyFollowUpsFaultInjection>> {
    Extension(faults)
}

pub(crate) fn is_faulted(extensions: &Extensions, fault: MatchingReplyFollowUpsFault) -> bool {
    extensions
        .get::<Arc<MatchingReplyFollowUpsFaultInjection>>()
        .is_some_and(|f| f.contains(fault))
}

pub(crate) fn is_patient_lookup_faulted(extensions: &Extensions) -> bool {
    is_faulted(extensions, MatchingReplyFollowUpsFault::PatientLookup)
}

const DEFAULT_MOVE_UNDO_WINDOW_SECONDS: i64 = 30;

pub(crate) fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/spruce-matching/messages", get(list_messages_to_match))
        .route("/spruce-matching/filtered", get(list_filtered_messages))
        .route(
            "/spruce-matching/messages/{message_id}/confirm",
            post(confirm_message_link),
        )
        .route(
            "/spruce-matching/messages/{message_id}/leave-unlinked",
            post(leave_message_unlinked),
        )
        .route(
            "/spruce-matching/filtered/{message_id}/this-is-a-patient",
            post(mark_message_as_patient),
        )
        .route(
            "/spruce-matching/unreadable/{message_id}/done",
            post(mark_unreadable_done),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies",
            get(list_patient_replies),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}",
            get(get_patient_reply),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/draft",
            put(save_reply_draft),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/revise",
            post(revise_reply_draft),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/restore-my-version",
            post(restore_my_version),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/choose-update",
            post(choose_coda_update),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/copy",
            post(copy_reply),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/no-reply-needed",
            post(close_no_reply_needed),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/undo-no-reply-needed",
            post(undo_no_reply_needed),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/ask-coda",
            post(ask_coda_to_reply),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/move",
            post(move_conversation),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/undo-move",
            post(undo_move_conversation),
        )
        .route(
            "/patient-workspaces/{id}/spruce-replies/{conversation_id}/disconnected-done",
            post(mark_disconnected_done),
        )
}

pub(crate) struct SampleRuntimeAuthority<'a> {
    pub(crate) auth: &'a dyn AuthStore,
    pub(crate) token: &'a str,
    pub(crate) now_unix: u64,
}

#[async_trait]
impl Authority for SampleRuntimeAuthority<'_> {
    async fn check(
        &mut self,
        _tx: &PublicationTransaction,
        scope: &Scope,
        _session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        let resolved = self
            .auth
            .identity(self.token, self.now_unix)
            .await
            .map_err(|err| format!("session invalid: {err:?}"))?;
        if resolved.practice_id != scope.practice || resolved.physician_id != scope.physician {
            return Err("session scope mismatch".to_owned());
        }
        if self.auth.membership(&resolved, &scope.workspace).await != Membership::Allowed {
            return Err("workspace not on physician panel".to_owned());
        }
        Ok(())
    }

    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.check(tx, scope, session_hash).await
    }
}

pub(crate) struct AuthPractice<'a>(pub(crate) &'a dyn AuthStore);

#[async_trait]
impl PracticeAuthority for AuthPractice<'_> {
    async fn check_practice_patient(
        &self,
        scope: &PracticeScope,
    ) -> Result<(), PracticeAccessError> {
        <dyn AuthStore as PracticeAuthority>::check_practice_patient(self.0, scope).await
    }
}

fn default_fallback_roles() -> ModelRoles {
    let binding = ModelBinding {
        model: "gpt-5.4-mini".to_owned(),
        effort: "low".to_owned(),
        context_window_tokens: 128_000,
        max_input_tokens: 96_000,
        adapter: "responses".to_owned(),
    };
    ModelRoles {
        chat: binding.clone(),
        background: binding,
    }
}

pub(crate) fn runtime_store_from_extensions(extensions: &Extensions) -> Option<Store> {
    if let Some(store) = extensions.get::<Store>() {
        return Some(store.clone());
    }
    extensions
        .get::<Arc<Live>>()
        .map(|live| live.chat.store.clone())
}

pub(crate) fn unix_to_rfc3339(seconds: i64) -> String {
    format_rfc3339(UNIX_EPOCH + Duration::from_secs(seconds.max(0) as u64))
}

fn current_unix_i64() -> i64 {
    i64::try_from(auth::now()).unwrap_or(i64::MAX)
}

pub(crate) async fn publish_bus_resync(extensions: &Extensions) {
    if let Some(ctx) = extensions.get::<AuthContext>() {
        let _ = ctx.auth.events.publish(physician_bus::Event::Resync).await;
    } else if let Some(live) = extensions.get::<Arc<Live>>() {
        let _ = live.bus.publish(physician_bus::Event::Resync).await;
    }
}

fn spruce_error_response(err: SpruceMatchingError) -> Response {
    match err {
        SpruceMatchingError::AccessDenied { .. } | SpruceMatchingError::PracticeAccess(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "detail": "Patient or conversation access is not available." })),
        )
            .into_response(),
        SpruceMatchingError::AdminRequired { .. } => (
            StatusCode::FORBIDDEN,
            Json(json!({ "detail": "An active practice admin must resolve a Spruce rollback halt." })),
        )
            .into_response(),
        SpruceMatchingError::NotFound { .. } => (
            StatusCode::NOT_FOUND,
            Json(json!({ "detail": err.to_string() })),
        )
            .into_response(),
        SpruceMatchingError::InvalidIdentifier { .. }
        | SpruceMatchingError::InvalidReasonText { .. } => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "detail": err.to_string() })),
        )
            .into_response(),
        SpruceMatchingError::InvalidTransition { .. }
        | SpruceMatchingError::ChannelStateRefused { .. }
        | SpruceMatchingError::IdempotencyCollision { .. }
        | SpruceMatchingError::ReaderLeaseHeld { .. }
        | SpruceMatchingError::InvalidReaderLease { .. }
        | SpruceMatchingError::RollbackHalted(_) => (
            StatusCode::CONFLICT,
            Json(json!({ "detail": err.to_string() })),
        )
            .into_response(),
        SpruceMatchingError::AuthorityUnavailable(_)
        | SpruceMatchingError::ReviewHandoff { .. }
        | SpruceMatchingError::Database(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not save that change right now." })),
        )
            .into_response(),
    }
}

fn reply_error_response(err: ReplyProposalError) -> Response {
    match err {
        ReplyProposalError::AccessDenied { .. } => (
            StatusCode::FORBIDDEN,
            Json(json!({ "code": "patient_unavailable", "detail": "Patient access is no longer available." })),
        )
            .into_response(),
        ReplyProposalError::ConversationNotFound { .. }
        | ReplyProposalError::ConversationNotLinkedToPatient { .. } => (
            StatusCode::NOT_FOUND,
            Json(json!({ "code": "conversation_unlinked", "detail": "Conversation is no longer linked to this patient." })),
        )
            .into_response(),
        ReplyProposalError::InvalidInput(_)
        | ReplyProposalError::MoveTargetUnchanged { .. }
        | ReplyProposalError::AutomaticPracticeMessageCannotAnswer { .. }
        | ReplyProposalError::UnreadPracticeMessage { .. }
        | ReplyProposalError::UnknownPatientMessage { .. }
        | ReplyProposalError::NoOpenMessagesForProposal
        | ReplyProposalError::OpenMessagesRemainUnhandled
        | ReplyProposalError::PhysicianRequestedReplyRequiresProposal => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "detail": format!("Not saved: {err}") })),
        )
            .into_response(),
        ReplyProposalError::StaleDraftVersion { .. }
        | ReplyProposalError::ConversationWithheld { .. }
        | ReplyProposalError::ReplyNotInRequiredState { .. }
        | ReplyProposalError::NoRevisionToRestore
        | ReplyProposalError::NoUpdateOffered
        | ReplyProposalError::ChannelDisconnected { .. }
        | ReplyProposalError::ChannelStillConnected { .. }
        | ReplyProposalError::IdempotencyCollision { .. }
        | ReplyProposalError::UndoNoLongerAvailable { .. }
        | ReplyProposalError::NoPendingReviewWake { .. }
        | ReplyProposalError::StaleReview { .. }
        | ReplyProposalError::StaleConversationRead { .. }
        | ReplyProposalError::StaleSourceObservation { .. }
        | ReplyProposalError::UnverifiedCitation { .. } => (
            StatusCode::CONFLICT,
            Json(json!({ "detail": format!("Not saved: {err}") })),
        )
            .into_response(),
        ReplyProposalError::Concern(_)
        | ReplyProposalError::Runtime(_)
        | ReplyProposalError::Database(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": format!("Not saved: {err}") })),
        )
            .into_response(),
    }
}

#[derive(Clone, Default)]
struct PatientIdentitySummary {
    display_name: String,
    birth_date: Option<String>,
    mrn: Option<String>,
}

/// Reads the display identity of every patient on `panel`. A failed read stays a failure:
/// a list that names candidates answers it with an explicit error rather than raw ids.
async fn authorized_patient_identities(
    directory: &dyn PatientDirectory,
    panel: &[String],
) -> Result<BTreeMap<String, PatientIdentitySummary>, crate::patients::DirectoryError> {
    let patients = read_patients(directory, panel).await?;
    Ok(patients
        .iter()
        .map(|patient: &Patient| {
            (
                patient.workspace().to_owned(),
                PatientIdentitySummary {
                    display_name: patient.display_name().to_owned(),
                    birth_date: patient.birth_date().map(str::to_owned),
                    mrn: patient.primary_mrn().map(str::to_owned),
                },
            )
        })
        .collect())
}

fn identities_unavailable_response() -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({ "detail": "Coda couldn’t read patient names." })),
    )
        .into_response()
}

fn map_channel_banner(standing: &ChannelStanding) -> SpruceChannelBannerWire {
    match standing {
        ChannelStanding::Disconnected {
            disconnected_at: None,
        } => SpruceChannelBannerWire::NotConfigured,
        ChannelStanding::Disconnected {
            disconnected_at: Some(ts),
        } => SpruceChannelBannerWire::Disconnected {
            reason: "Spruce channel was disconnected by the practice.".to_owned(),
            disconnected_at: unix_to_rfc3339(*ts),
        },
        ChannelStanding::Connected { health, .. } => match health {
            ChannelHealth::Reading { last_read_at } => SpruceChannelBannerWire::ConnectedReading {
                last_read_at: last_read_at.map(unix_to_rfc3339),
            },
            ChannelHealth::TransientFailure {
                reason,
                failed_at,
                last_read_at,
            } => SpruceChannelBannerWire::ReadFailed {
                last_read_at: last_read_at.map(unix_to_rfc3339),
                reason: reason.clone(),
                failed_at: unix_to_rfc3339(*failed_at),
            },
            ChannelHealth::RevokedOrBlocked {
                reason,
                failed_at,
                last_read_at,
            } => SpruceChannelBannerWire::ReconnectRequired {
                last_read_at: last_read_at.map(unix_to_rfc3339),
                reason: reason.clone(),
                lost_at: unix_to_rfc3339(*failed_at),
            },
        },
    }
}

fn map_attachments(items: &[SpruceAttachmentSummary]) -> Vec<SpruceAttachmentWire> {
    items
        .iter()
        .enumerate()
        .map(|(idx, item)| SpruceAttachmentWire {
            attachment_id: format!("att-{}", idx + 1),
            filename: item.name.clone().or_else(|| item.title.clone()),
            mime_type: item.mimetype.clone().unwrap_or_else(|| item.kind.clone()),
        })
        .collect()
}

fn sender_display_label(author_name: Option<&str>, sender_address: Option<&str>) -> String {
    author_name
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .or_else(|| sender_address.map(str::trim).filter(|s| !s.is_empty()))
        .unwrap_or("Unknown sender")
        .to_owned()
}

async fn map_unmatched_row(
    store: &Store,
    auth_store: &dyn AuthStore,
    viewer: &PhysicianIdentity,
    row: &UnmatchedMessageRow,
    identities: &BTreeMap<String, PatientIdentitySummary>,
) -> Result<UnmatchedMessageWire, SpruceMatchingError> {
    let urgency = match &row.urgency {
        MessageUrgency::Ordinary => UnmatchedUrgencyWire::Ordinary,
        MessageUrgency::Urgent { reason, flagged_at } => UnmatchedUrgencyWire::Urgent {
            reason: reason.clone(),
            flagged_at: unix_to_rfc3339(*flagged_at),
        },
    };

    let suggestion = if let Some(suggested) = &row.suggested_patient {
        let info = identities.get(&suggested.workspace);
        MatchSuggestionWire::SinglePatient {
            patient_workspace_id: suggested.workspace.clone(),
            patient_display_name: info
                .map(|i| i.display_name.clone())
                .unwrap_or_else(|| suggested.workspace.clone()),
            birth_date: info.and_then(|i| i.birth_date.clone()),
            mrn: info.and_then(|i| i.mrn.clone()),
            evidence_summary: suggested.evidence_summary.clone(),
        }
    } else {
        // Inspect stored classification (redacting any candidate workspace outside viewer's panel).
        let saved = store
            .spruce_message(&viewer.practice_id, &row.message_id)
            .await?;
        match saved.and_then(|m| m.saved_classification.map(|c| c.decision)) {
            Some(MatchingDecision::PatientMail { assessment, .. }) => match assessment {
                MatchAssessment::AmbiguousCandidates {
                    candidate_workspaces,
                    evidence_summary,
                } => {
                    let mut candidates = Vec::new();
                    for ws in candidate_workspaces {
                        if auth_store.membership(viewer, &ws).await == Membership::Allowed {
                            let info = identities.get(&ws);
                            candidates.push(CandidatePatientWire {
                                patient_workspace_id: ws.clone(),
                                patient_display_name: info
                                    .map(|i| i.display_name.clone())
                                    .unwrap_or(ws),
                                birth_date: info.and_then(|i| i.birth_date.clone()),
                                mrn: info.and_then(|i| i.mrn.clone()),
                            });
                        }
                    }
                    MatchSuggestionWire::NeedsPhysicianChoice {
                        reason: NeedsChoiceReasonWire::AmbiguousCandidates,
                        evidence_summary,
                        candidate_patients: candidates,
                    }
                }
                MatchAssessment::ConflictingEvidence {
                    candidate_workspaces,
                    evidence_summary,
                } => {
                    let mut candidates = Vec::new();
                    for ws in candidate_workspaces {
                        if auth_store.membership(viewer, &ws).await == Membership::Allowed {
                            let info = identities.get(&ws);
                            candidates.push(CandidatePatientWire {
                                patient_workspace_id: ws.clone(),
                                patient_display_name: info
                                    .map(|i| i.display_name.clone())
                                    .unwrap_or(ws),
                                birth_date: info.and_then(|i| i.birth_date.clone()),
                                mrn: info.and_then(|i| i.mrn.clone()),
                            });
                        }
                    }
                    MatchSuggestionWire::NeedsPhysicianChoice {
                        reason: NeedsChoiceReasonWire::ConflictingEvidence,
                        evidence_summary,
                        candidate_patients: candidates,
                    }
                }
                MatchAssessment::NoMatchFound { evidence_summary } => {
                    MatchSuggestionWire::NeedsPhysicianChoice {
                        reason: NeedsChoiceReasonWire::NoMatchFound,
                        evidence_summary,
                        candidate_patients: Vec::new(),
                    }
                }
                MatchAssessment::SuggestedPatient { .. } => {
                    // Redacted because the suggested patient is not on this physician's panel (`Rule 2`).
                    MatchSuggestionWire::NeedsPhysicianChoice {
                        reason: NeedsChoiceReasonWire::NoMatchFound,
                        evidence_summary:
                            "No patient on your panel matched automatically; choose a patient or leave unlinked."
                                .to_owned(),
                        candidate_patients: Vec::new(),
                    }
                }
            },
            _ => MatchSuggestionWire::NeedsPhysicianChoice {
                reason: NeedsChoiceReasonWire::ClassificationTimedOut,
                evidence_summary:
                    "Choose the patient this message belongs to, or leave it unlinked.".to_owned(),
                candidate_patients: Vec::new(),
            },
        }
    };

    let sender_addr = row.sender_address.as_ref().map(|s| s.as_str().to_owned());
    Ok(UnmatchedMessageWire {
        message_id: row.message_id.clone(),
        conversation_id: row.conversation_id.clone(),
        sender_display_name: sender_display_label(
            row.author_name.as_deref(),
            sender_addr.as_deref(),
        ),
        sender_address: sender_addr,
        body_text: row.body_text.clone(),
        attachments: map_attachments(&row.attachments),
        spruce_app_url: row.app_url.clone(),
        waiting_since: unix_to_rfc3339(row.waiting_since),
        urgency,
        suggestion,
    })
}

async fn list_messages_to_match(
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    if is_faulted(extensions, MatchingReplyFollowUpsFault::MatchingRead) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Messages to match could not be loaded right now." })),
        )
            .into_response();
    }
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return Json(MessagesToMatchResponseWire {
            channel_banner: SpruceChannelBannerWire::NotConfigured,
            messages: Vec::new(),
            unreadable_messages: Vec::new(),
            filtered_count: 0,
        })
        .into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let panel = auth_store.panel(&identity).await;
    let directory = connected.patients(extensions);
    let identities = match authorized_patient_identities(&*directory, &panel).await {
        Ok(identities) => identities,
        Err(_) => return identities_unavailable_response(),
    };

    let standing = match store.spruce_channel_standing(&identity.practice_id).await {
        Ok(standing) => standing,
        Err(err) => return spruce_error_response(err),
    };
    let summary = match store
        .today_spruce_summary_for_physician(auth_store.as_ref(), &identity)
        .await
    {
        Ok(summary) => summary,
        Err(err) => return spruce_error_response(err),
    };
    let filtered_count = match store
        .filtered_spruce_messages_for_physician(auth_store.as_ref(), &identity)
        .await
    {
        Ok(rows) => rows.len(),
        Err(err) => return spruce_error_response(err),
    };

    let mut messages = Vec::with_capacity(summary.messages_to_match.len());
    for row in &summary.messages_to_match {
        match map_unmatched_row(&store, auth_store.as_ref(), &identity, row, &identities).await {
            Ok(wire) => messages.push(wire),
            Err(err) => return spruce_error_response(err),
        }
    }
    let unreadable_messages = summary
        .unreadable_messages
        .iter()
        .map(|u| UnreadableSpruceMessageWire {
            message_id: u.message_id.clone(),
            conversation_id: u.conversation_id.clone(),
            failure_reason: u.failure_reason.clone(),
            spruce_app_url: u.app_url.clone(),
            kept_at: unix_to_rfc3339(u.kept_at),
        })
        .collect();

    Json(MessagesToMatchResponseWire {
        channel_banner: map_channel_banner(&standing.0),
        messages,
        unreadable_messages,
        filtered_count,
    })
    .into_response()
}

async fn list_filtered_messages(
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    if is_faulted(extensions, MatchingReplyFollowUpsFault::MatchingRead) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Filtered messages could not be loaded right now." })),
        )
            .into_response();
    }
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return Json(FilteredMessagesResponseWire {
            messages: Vec::new(),
        })
        .into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let rows = match store
        .filtered_spruce_messages_for_physician(auth_store.as_ref(), &identity)
        .await
    {
        Ok(rows) => rows,
        Err(err) => return spruce_error_response(err),
    };

    let mut messages = Vec::with_capacity(rows.len());
    for row in rows {
        let app_url = match store
            .spruce_message(&identity.practice_id, &row.message_id)
            .await
        {
            Ok(saved) => saved.and_then(|m| m.app_url),
            Err(err) => return spruce_error_response(err),
        };
        let sender_addr = row.sender_address.as_ref().map(|a| a.as_str().to_owned());
        messages.push(FilteredMessageWire {
            message_id: row.message_id,
            conversation_id: row.conversation_id,
            sender_display_name: sender_display_label(
                row.author_name.as_deref(),
                sender_addr.as_deref(),
            ),
            sender_address: sender_addr,
            body_text: row.body_text,
            spruce_app_url: app_url,
            category: match row.category {
                NonPatientCategory::VendorOrSpam => NonPatientCategoryWire::VendorOrSpam,
                NonPatientCategory::AdministrativeNotice => {
                    NonPatientCategoryWire::AdministrativeNotice
                }
                NonPatientCategory::NonPatientOutreach => {
                    NonPatientCategoryWire::NonPatientOutreach
                }
            },
            rationale: row.rationale,
            filtered_at: unix_to_rfc3339(row.filtered_at),
        });
    }

    Json(FilteredMessagesResponseWire { messages }).into_response()
}

/// Ensures that a linked Spruce conversation and its linked messages are also
/// registered in `physician_runtime_reply_conversations` so the reply editor
/// surface can inspect and act on them immediately.
pub(crate) async fn sync_spruce_conversation_into_reply_store(
    store: &Store,
    practice: &str,
    conversation_id: &str,
    workspace: &str,
    patient_display_name: Option<&str>,
) {
    if let Err(ReplyProposalError::IdempotencyCollision { .. }) = store
        .link_reply_conversation(
            practice,
            conversation_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.to_owned(),
            },
        )
        .await
        && let Ok(Some(sc)) = store.spruce_conversation(practice, conversation_id).await
        && sc.move_undo.is_none()
        && matches!(
            &sc.link_state,
            ConversationLinkState::Linked { workspace: sc_ws, .. } if sc_ws == workspace
        )
        && let Ok(cmd_key) = CommandKey::new(format!(
            "settle-reply-move:{}:{}:{}",
            practice, conversation_id, sc.move_count
        ))
    {
        let _ = store
            .act_on_conversation_move(
                &cmd_key,
                practice,
                conversation_id,
                ConversationLink::LinkedPatient {
                    workspace: workspace.to_owned(),
                },
            )
            .await;
    }

    let Ok(linked_messages) = store
        .patient_linked_spruce_messages(practice, workspace)
        .await
    else {
        return;
    };
    for msg in linked_messages
        .into_iter()
        .filter(|m| m.conversation_id == conversation_id)
    {
        if !matches!(msg.standing, MessageStanding::Linked { .. }) {
            continue;
        }
        let sender_addr = msg.sender_address.as_ref().map(|a| a.as_str());
        let label = sender_display_label(msg.author_name.as_deref(), sender_addr);
        let is_same_as_patient = patient_display_name.is_some_and(|pname| {
            msg.author_name
                .as_deref()
                .map(str::trim)
                .is_some_and(|author| author.eq_ignore_ascii_case(pname.trim()))
        });
        let sender = if is_same_as_patient {
            MessageSender::Patient
        } else {
            MessageSender::NamedSender { name: label }
        };
        let _ = store
            .record_linked_patient_message(&RecordPatientMessage {
                practice: practice.to_owned(),
                conversation_id: conversation_id.to_owned(),
                message_id: msg.message_id,
                sender,
                text: if msg.body_text.trim().is_empty() {
                    "(Attachment)".to_owned()
                } else {
                    msg.body_text
                },
                sent_at: msg.kept_at,
            })
            .await;
    }
}

pub(crate) async fn with_review_receiver_setup(
    store: &Store,
    extensions: &Extensions,
) -> (ModelRoles, PhysicianSkills) {
    if let Some(live) = extensions.get::<Arc<Live>>() {
        return (live.chat.roles().clone(), live.chat.skills().clone());
    }
    let roles = default_fallback_roles();
    let skills =
        PhysicianSkills::configured(None, false).expect("embedded physician skills catalog");
    let _ = store;
    (roles, skills)
}

async fn confirm_message_link(
    Path(message_id): Path<String>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if is_faulted(extensions, MatchingReplyFollowUpsFault::MatchingCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not confirm that patient match right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<ConfirmMessageLinkBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (roles, skills) = with_review_receiver_setup(&store, extensions).await;
    let practice_auth = AuthPractice(auth_store.as_ref());
    let receiver = PracticeJobReviewReceiver::new(store.clone(), &roles, &skills, &practice_auth);
    let policy = match payload.sender_policy {
        SenderConfirmationPolicyWire::ConversationOnly => {
            SenderConfirmationPolicy::ConversationOnly
        }
        SenderConfirmationPolicyWire::AlsoConfirmSenderForPatient => {
            SenderConfirmationPolicy::ConfirmSenderForPatient
        }
    };
    let now_ts = current_unix_i64();

    let outcome = match store
        .confirm_spruce_message_link(
            auth_store.as_ref(),
            &identity,
            &message_id,
            &payload.patient_workspace_id,
            policy,
            &payload.idempotency_key,
            Some(&receiver as &dyn ReviewReceiver),
            now_ts,
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return spruce_error_response(err),
    };

    let panel = auth_store.panel(&identity).await;
    let directory = connected.patients(extensions);
    // The link is already saved; a failed name read must not make the saved
    // action read as failed, so the response names the workspace instead.
    let identities = authorized_patient_identities(&*directory, &panel)
        .await
        .unwrap_or_default();
    let patient_display_name = identities
        .get(&outcome.workspace)
        .map(|i| i.display_name.clone())
        .unwrap_or_else(|| outcome.workspace.clone());

    sync_spruce_conversation_into_reply_store(
        &store,
        &identity.practice_id,
        &outcome.conversation_id,
        &outcome.workspace,
        Some(&patient_display_name),
    )
    .await;
    publish_bus_resync(extensions).await;

    Json(ConfirmMessageLinkResponseWire {
        message_id,
        conversation_id: outcome.conversation_id,
        patient_workspace_id: outcome.workspace,
        patient_display_name,
        linked_message_ids: outcome.linked_message_ids,
    })
    .into_response()
}

async fn leave_message_unlinked(
    Path(message_id): Path<String>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if is_faulted(extensions, MatchingReplyFollowUpsFault::MatchingCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not update that message right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<MatchingActionCommandBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let now_ts = current_unix_i64();
    let saved = match store
        .leave_spruce_message_unlinked(
            auth_store.as_ref(),
            &identity,
            &message_id,
            &payload.idempotency_key,
            now_ts,
        )
        .await
    {
        Ok(saved) => saved,
        Err(err) => return spruce_error_response(err),
    };
    publish_bus_resync(extensions).await;

    Json(MatchingMessageStatusResponseWire {
        message_id: saved.message_id,
        standing: MatchedMessageStandingWire::Unlinked,
    })
    .into_response()
}

async fn mark_message_as_patient(
    Path(message_id): Path<String>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if is_faulted(extensions, MatchingReplyFollowUpsFault::MatchingCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not move that message right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<MatchingActionCommandBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (roles, skills) = with_review_receiver_setup(&store, extensions).await;
    let practice_auth = AuthPractice(auth_store.as_ref());
    let receiver = PracticeJobReviewReceiver::new(store.clone(), &roles, &skills, &practice_auth);
    let now_ts = current_unix_i64();

    let routing = match store
        .mark_spruce_message_as_patient(
            auth_store.as_ref(),
            &identity,
            &message_id,
            &payload.idempotency_key,
            Some(&receiver as &dyn ReviewReceiver),
            now_ts,
        )
        .await
    {
        Ok(routing) => routing,
        Err(err) => return spruce_error_response(err),
    };

    let conv_id = store
        .spruce_message(&identity.practice_id, &message_id)
        .await
        .ok()
        .flatten()
        .map(|m| m.conversation_id)
        .unwrap_or_default();

    let wire = match routing {
        MessageRoutingOutcome::Linked { workspace, .. } => {
            let panel = auth_store.panel(&identity).await;
            let directory = connected.patients(extensions);
            // The routing is already saved; a failed name read leaves the name absent
            // rather than making the saved action read as failed.
            let identities = authorized_patient_identities(&*directory, &panel)
                .await
                .unwrap_or_default();
            let pname = identities.get(&workspace).map(|i| i.display_name.clone());
            sync_spruce_conversation_into_reply_store(
                &store,
                &identity.practice_id,
                &conv_id,
                &workspace,
                pname.as_deref(),
            )
            .await;
            ThisIsAPatientOutcomeWire::Linked {
                message_id,
                conversation_id: conv_id,
                patient_workspace_id: Some(workspace),
                patient_display_name: pname,
            }
        }
        MessageRoutingOutcome::LinkedWithoutViewerAccess { .. } => {
            ThisIsAPatientOutcomeWire::Linked {
                message_id,
                conversation_id: conv_id,
                patient_workspace_id: None,
                patient_display_name: None,
            }
        }
        MessageRoutingOutcome::Waiting { .. }
        | MessageRoutingOutcome::Filtered { .. }
        | MessageRoutingOutcome::NotAPatient => {
            ThisIsAPatientOutcomeWire::WaitingInMessagesToMatch {
                message_id,
                conversation_id: conv_id,
            }
        }
    };
    publish_bus_resync(extensions).await;

    Json(wire).into_response()
}

async fn mark_unreadable_done(
    Path(message_id): Path<String>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if is_faulted(extensions, MatchingReplyFollowUpsFault::MatchingCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not mark that item done right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<MatchingActionCommandBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let now_ts = current_unix_i64();
    let saved = match store
        .mark_unreadable_spruce_message_done(
            auth_store.as_ref(),
            &identity,
            &message_id,
            &payload.idempotency_key,
            now_ts,
        )
        .await
    {
        Ok(saved) => saved,
        Err(err) => return spruce_error_response(err),
    };
    publish_bus_resync(extensions).await;

    Json(MatchingMessageStatusResponseWire {
        message_id: saved.message_id,
        standing: MatchedMessageStandingWire::Done,
    })
    .into_response()
}

/// Settles any expired `Wrong patient?` Undo windows in both `spruce_matching`
/// and `reply::proposals` so moved conversations transition to `Withheld` (on
/// `AnotherPatient`) or close & retire associated concerns (on `NotAPatient`).
pub(crate) async fn settle_expired_spruce_moves_and_replies(
    store: &Store,
    auth_store: &dyn AuthStore,
    directory: &dyn PatientDirectory,
    extensions: &Extensions,
    practice: &str,
    now_ts: i64,
) {
    let (roles, skills) = with_review_receiver_setup(store, extensions).await;
    let practice_auth = AuthPractice(auth_store);
    let receiver = PracticeJobReviewReceiver::new(store.clone(), &roles, &skills, &practice_auth);
    let Ok(report) = store
        .settle_expired_spruce_moves(practice, Some(&receiver as &dyn ReviewReceiver), now_ts)
        .await
    else {
        return;
    };

    let mut settled = report.succeeded;
    // `act_on_spruce_conversation_move` commits the settled `spruce_conversation` row before
    // dispatching queued review triggers; if trigger dispatch fails (for instance without a live
    // model), `move_undo` is already cleared, so reconcile `reply_conversation` as well.
    for failure in report.failures {
        if let Ok(Some(conv)) = store
            .spruce_conversation(practice, &failure.target_id)
            .await
            && conv.move_undo.is_none()
        {
            settled.push(conv);
        }
    }

    for conv in settled {
        let target = match &conv.link_state {
            ConversationLinkState::Linked { workspace, .. } => Some((
                ConversationLink::LinkedPatient {
                    workspace: workspace.clone(),
                },
                Some(workspace.clone()),
            )),
            ConversationLinkState::MovedNotAPatient { .. } => {
                Some((ConversationLink::NotAPatient, None))
            }
            ConversationLinkState::Unlinked => None,
        };
        if let Some((link_target, maybe_ws)) = target {
            if let Ok(cmd_key) = CommandKey::new(format!(
                "settle-reply-move:{}:{}:{}",
                practice, conv.conversation_id, conv.move_count
            )) {
                let _ = store
                    .act_on_conversation_move(
                        &cmd_key,
                        practice,
                        &conv.conversation_id,
                        link_target,
                    )
                    .await;
            }
            if let Some(ws) = maybe_ws {
                let dest_patient_name = read_patients(directory, std::slice::from_ref(&ws))
                    .await
                    .ok()
                    .and_then(|patients| {
                        patients
                            .iter()
                            .find(|p| p.workspace() == ws)
                            .map(|p| p.display_name().to_owned())
                    });
                sync_spruce_conversation_into_reply_store(
                    store,
                    practice,
                    &conv.conversation_id,
                    &ws,
                    dest_patient_name.as_deref(),
                )
                .await;
            }
        }
    }
}

fn format_basis(basis: Option<&ReplyProposalBasis>) -> String {
    let Some(basis) = basis else {
        return String::new();
    };
    if basis.limits.is_empty() {
        basis.summary.clone()
    } else {
        format!("{} Limits: {}", basis.summary, basis.limits.join("; "))
    }
}

fn map_citations(
    citations: &[SummaryCitation],
    sources_by_key: &BTreeMap<(String, String), SourceRevision>,
) -> Vec<ItemCitationWire> {
    citations
        .iter()
        .map(|c| {
            let body = sources_by_key
                .get(&(
                    c.reference.candidate_id.clone(),
                    c.reference.revision.clone(),
                ))
                .map(|src| {
                    format_cited_source_body(&c.display_label, &src.readable_parts, &src.content)
                })
                .unwrap_or_else(|| c.excerpt.clone());
            ItemCitationWire {
                candidate_id: c.reference.candidate_id.clone(),
                revision: c.reference.revision.clone(),
                source_id: c.source_id.clone(),
                source_label: c.source_label.clone(),
                label: c.display_label.clone(),
                as_of: c.as_of.clone(),
                excerpt: c.excerpt.clone(),
                body,
                native_uri: c.native_uri.clone(),
            }
        })
        .collect()
}

fn channel_participant_label(spruce_conv: Option<&SavedSpruceConversation>) -> Option<String> {
    let first_outside = spruce_conv?.outside_participants.first()?;
    first_outside
        .display_name
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .or_else(|| {
            first_outside
                .endpoint_address
                .as_ref()
                .map(|a| a.as_str().trim())
                .filter(|s| !s.is_empty())
        })
        .map(str::to_owned)
}

fn channel_message_sender_label(
    message_id: &str,
    spruce_msgs_by_id: &BTreeMap<String, SavedSpruceMessage>,
    spruce_conv: Option<&SavedSpruceConversation>,
) -> String {
    if let Some(sm) = spruce_msgs_by_id.get(message_id) {
        let addr = sm.sender_address.as_ref().map(|a| a.as_str());
        if sm
            .author_name
            .as_deref()
            .is_some_and(|s| !s.trim().is_empty())
            || addr.is_some_and(|s| !s.trim().is_empty())
        {
            return sender_display_label(sm.author_name.as_deref(), addr);
        }
    }
    channel_participant_label(spruce_conv).unwrap_or_else(|| "Patient".to_owned())
}

fn map_patient_messages(
    messages: &[LinkedPatientMessage],
    spruce_msgs_by_id: &BTreeMap<String, SavedSpruceMessage>,
    spruce_conv: Option<&SavedSpruceConversation>,
    proposed_at: Option<i64>,
    covered_message_ids: Option<&BTreeSet<String>>,
) -> Vec<ReplyPatientMessageWire> {
    messages
        .iter()
        .map(|m| {
            let (sender_display_name, sender_role) = match &m.sender {
                MessageSender::Patient => (
                    channel_message_sender_label(&m.message_id, spruce_msgs_by_id, spruce_conv),
                    ChannelSenderRoleWire::Patient,
                ),
                MessageSender::NamedSender { name } => {
                    (name.clone(), ChannelSenderRoleWire::NonPatient)
                }
            };
            let arrived_after_proposal = proposed_at.is_some_and(|prop_ts| m.sent_at > prop_ts)
                && covered_message_ids.is_none_or(|covered| !covered.contains(&m.message_id));
            ReplyPatientMessageWire {
                message_id: m.message_id.clone(),
                sequence: m.sequence,
                sender_display_name,
                sender_role,
                text: m.text.clone(),
                sent_at: unix_to_rfc3339(m.sent_at),
                arrived_after_proposal,
            }
        })
        .collect()
}

fn map_practice_messages(messages: &[ObservedPracticeMessage]) -> Vec<ReplyPracticeMessageWire> {
    messages
        .iter()
        .map(|m| ReplyPracticeMessageWire {
            practice_message_id: m.practice_message_id.clone(),
            sequence: m.sequence,
            kind: match m.kind {
                local_product::runtime::reply::proposals::PracticeMessageKind::PracticeSent => {
                    PracticeMessageKindWire::HumanReply
                }
                local_product::runtime::reply::proposals::PracticeMessageKind::Automatic => {
                    PracticeMessageKindWire::AutomatedAcknowledgement
                }
            },
            text: m.text.clone(),
            sent_at: unix_to_rfc3339(m.sent_at),
        })
        .collect()
}

fn derive_primary_channel_sender(
    messages: &[LinkedPatientMessage],
    spruce_msgs_by_id: &BTreeMap<String, SavedSpruceMessage>,
    spruce_conv: Option<&SavedSpruceConversation>,
    patient_display_name: &str,
) -> (String, ChannelSenderRoleWire) {
    if let Some(newest) = messages.last() {
        return match &newest.sender {
            MessageSender::NamedSender { name } => {
                (name.clone(), ChannelSenderRoleWire::NonPatient)
            }
            MessageSender::Patient => (
                channel_message_sender_label(&newest.message_id, spruce_msgs_by_id, spruce_conv),
                ChannelSenderRoleWire::Patient,
            ),
        };
    }
    if let Some(label) = channel_participant_label(spruce_conv) {
        let role = if label.eq_ignore_ascii_case(patient_display_name.trim()) {
            ChannelSenderRoleWire::Patient
        } else {
            ChannelSenderRoleWire::NonPatient
        };
        return (label, role);
    }
    ("Patient".to_owned(), ChannelSenderRoleWire::Patient)
}

fn map_pending_move(
    undo: &PendingMoveUndo,
    link_state: &ConversationLinkState,
    identities: &BTreeMap<String, PatientIdentitySummary>,
    now_ts: i64,
) -> Option<PendingSpruceMoveWire> {
    if !undo.is_active_at(now_ts) {
        return None;
    }
    let destination = match link_state {
        ConversationLinkState::Linked { workspace, .. } => {
            let patient_label = identities
                .get(workspace)
                .map(|i| i.display_name.clone())
                .unwrap_or_else(|| "another patient".to_owned());
            PendingMoveTargetDisplayWire::AnotherPatient {
                patient_workspace_id: identities
                    .contains_key(workspace)
                    .then(|| workspace.clone()),
                patient_label,
            }
        }
        ConversationLinkState::MovedNotAPatient { .. } | ConversationLinkState::Unlinked => {
            PendingMoveTargetDisplayWire::NotAPatient
        }
    };
    Some(PendingSpruceMoveWire {
        destination,
        moved_by_physician: undo.by_physician.clone(),
        expires_at: unix_to_rfc3339(undo.expires_at),
    })
}

async fn spruce_app_url_for_conversation(
    store: &Store,
    practice: &str,
    workspace: &str,
    conversation_id: &str,
) -> Option<String> {
    if let Ok(msgs) = store
        .patient_linked_spruce_messages(practice, workspace)
        .await
        && let Some(url) = msgs
            .into_iter()
            .filter(|m| m.conversation_id == conversation_id)
            .find_map(|m| m.app_url)
    {
        return Some(url);
    }
    None
}

#[expect(
    clippy::too_many_arguments,
    reason = "single projection owner combines reply surface, conversation metadata, Spruce move state, citations, and viewer draft status"
)]
async fn project_reply_surface_wire(
    store: &Store,
    practice: &str,
    physician_id: &str,
    workspace: &str,
    patient_display_name: &str,
    conversation_id: &str,
    surface: PhysicianReplySurface,
    raw_conv: Option<&ReplyConversation>,
    spruce_conv: Option<&SavedSpruceConversation>,
    spruce_msgs_by_id: &BTreeMap<String, SavedSpruceMessage>,
    identities: &BTreeMap<String, PatientIdentitySummary>,
    now_ts: i64,
) -> PatientReplyConversationWire {
    let pending_move = spruce_conv.and_then(|sc| {
        sc.move_undo
            .as_ref()
            .and_then(|u| map_pending_move(u, &sc.link_state, identities, now_ts))
    });
    let spruce_app_url = spruce_msgs_by_id
        .values()
        .filter(|m| m.conversation_id == conversation_id)
        .find_map(|m| m.app_url.clone())
        .or(spruce_app_url_for_conversation(store, practice, workspace, conversation_id).await);

    let sources_by_key: BTreeMap<(String, String), SourceRevision> = store
        .sources(workspace)
        .await
        .map(|obs| {
            obs.sources
                .into_iter()
                .map(|s| ((s.candidate_id.clone(), s.revision.clone()), s))
                .collect()
        })
        .unwrap_or_default();

    match surface {
        PhysicianReplySurface::OpenCard(card) => {
            project_open_card_wire(
                store,
                practice,
                physician_id,
                workspace,
                patient_display_name,
                *card,
                raw_conv,
                spruce_conv,
                spruce_msgs_by_id,
                spruce_app_url,
                pending_move,
                &sources_by_key,
                now_ts,
            )
            .await
        }
        PhysicianReplySurface::ClosedUndoBanner {
            conversation_id,
            workspace,
            closed_by,
            ..
        } => {
            let empty_msgs = Vec::new();
            let msgs = raw_conv.map_or(empty_msgs.as_slice(), |c| c.patient_messages.as_slice());
            let (primary_sender_name, primary_sender_role) = derive_primary_channel_sender(
                msgs,
                spruce_msgs_by_id,
                spruce_conv,
                patient_display_name,
            );
            PatientReplyConversationWire::ClosedUndoBanner {
                conversation_id,
                patient_workspace_id: workspace,
                primary_sender_name,
                primary_sender_role,
                spruce_app_url,
                closed_by_physician: closed_by,
                pending_move,
            }
        }
        PhysicianReplySurface::NoOpenCard {
            conversation_id,
            workspace,
            open_messages,
            practice_messages,
            non_patient_sender_names,
            review_state,
        } => {
            let all_msgs = raw_conv.map_or(open_messages.as_slice(), |c| {
                if open_messages.is_empty() {
                    c.patient_messages.as_slice()
                } else {
                    open_messages.as_slice()
                }
            });
            let (primary_sender_name, primary_sender_role) = derive_primary_channel_sender(
                all_msgs,
                spruce_msgs_by_id,
                spruce_conv,
                patient_display_name,
            );
            let started_at = raw_conv
                .and_then(|c| c.active_review.as_ref().map(|r| r.started_at))
                .or_else(|| raw_conv.map(|c| c.updated_at))
                .unwrap_or(now_ts);
            let review_wire = match review_state {
                CardReviewState::Idle => CardReviewStateWire::Idle,
                CardReviewState::Reviewing => CardReviewStateWire::Reviewing {
                    started_at: unix_to_rfc3339(started_at),
                },
            };
            let channel_connected =
                raw_conv.is_none_or(|c| matches!(c.channel, ChannelConnection::Connected));
            let has_any_patient_msg = raw_conv.is_some_and(|c| !c.patient_messages.is_empty())
                || !open_messages.is_empty();
            PatientReplyConversationWire::NoOpenCard {
                conversation_id,
                patient_workspace_id: workspace,
                primary_sender_name,
                primary_sender_role,
                non_patient_sender_names,
                spruce_app_url,
                patient_messages: map_patient_messages(
                    all_msgs,
                    spruce_msgs_by_id,
                    spruce_conv,
                    None,
                    None,
                ),
                practice_messages: map_practice_messages(&practice_messages),
                review_state: review_wire,
                can_ask_coda_to_reply: channel_connected && has_any_patient_msg,
                pending_move,
            }
        }
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "projects a complete ReplyCardWire from durable ReplyCardView and retained chart sources"
)]
async fn project_open_card_wire(
    store: &Store,
    practice: &str,
    physician_id: &str,
    workspace: &str,
    patient_display_name: &str,
    card: ReplyCardView,
    raw_conv: Option<&ReplyConversation>,
    spruce_conv: Option<&SavedSpruceConversation>,
    spruce_msgs_by_id: &BTreeMap<String, SavedSpruceMessage>,
    spruce_app_url: Option<String>,
    pending_move: Option<PendingSpruceMoveWire>,
    sources_by_key: &BTreeMap<(String, String), SourceRevision>,
    now_ts: i64,
) -> PatientReplyConversationWire {
    let (primary_sender_name, primary_sender_role) = derive_primary_channel_sender(
        &card.open_messages,
        spruce_msgs_by_id,
        spruce_conv,
        patient_display_name,
    );

    let (proposal_version, proposed_at, covered_ids) = match raw_conv.map(|c| &c.reply_standing) {
        Some(ReplyStanding::Ready { proposal }) => (
            proposal.version,
            Some(proposal.proposed_at),
            Some(&proposal.covered_message_ids),
        ),
        Some(ReplyStanding::Withheld {
            withheld_from_proposal_version,
            ..
        }) => (*withheld_from_proposal_version, None, None),
        _ => match &card.displayed_text {
            DisplayedReplyText::CodaProposal { version, .. } => (*version, None, None),
            DisplayedReplyText::PhysicianDraft { .. } => {
                let v = match &card.draft_standing {
                    PhysicianDraftStanding::Active {
                        based_on_proposal_version,
                        ..
                    } => *based_on_proposal_version,
                    PhysicianDraftStanding::NoDraft => 1,
                };
                (v, None, None)
            }
            DisplayedReplyText::Withheld => (0, None, None),
        },
    };

    let displayed_reply = match &card.displayed_text {
        DisplayedReplyText::CodaProposal { version, text, .. } => {
            DisplayedReplyTextWire::CodaProposal {
                proposal_version: *version,
                text: text.clone(),
            }
        }
        DisplayedReplyText::PhysicianDraft {
            draft_revision,
            kind,
            text,
            ..
        } => DisplayedReplyTextWire::PhysicianDraft {
            revision: *draft_revision,
            text: text.clone(),
            kind: match kind {
                PhysicianDraftKind::YourText => PhysicianDraftKindWire::YourText,
                PhysicianDraftKind::CodaRevision {
                    pre_revision_your_text,
                } => PhysicianDraftKindWire::CodaRevision {
                    pre_revision_your_text: pre_revision_your_text.clone(),
                },
            },
        },
        DisplayedReplyText::Withheld => DisplayedReplyTextWire::Withheld {
            proposal_version,
            reason:
                "Reply withheld while Coda reviews the moved conversation against this patient's chart."
                    .to_owned(),
        },
    };

    let revision_run = match &card.draft_standing {
        PhysicianDraftStanding::Active {
            draft_scope_chat, ..
        } => {
            let draft_scope = Scope {
                practice: practice.to_owned(),
                physician: physician_id.to_owned(),
                workspace: workspace.to_owned(),
                chat: draft_scope_chat.clone(),
            };
            store
                .draft(&draft_scope)
                .await
                .ok()
                .flatten()
                .and_then(|d| crate::reply::revision_run(&d, draft_scope_chat))
        }
        PhysicianDraftStanding::NoDraft => None,
    };

    let coda_update_offer = card
        .coda_update_offer
        .as_ref()
        .map(|offer| CodaUpdateOfferWire {
            offered_proposal_version: offer.offered_version,
            text: offer.offered_text.clone(),
            basis_and_limitations: format_basis(Some(&offer.offered_basis)),
            citations: map_citations(&offer.offered_citations, sources_by_key),
        });

    let notices = card
        .notices
        .iter()
        .map(|n| match n {
            CardArrivalNotice::NewerReplyReplacedPrevious {
                previous_version,
                current_version,
            } => CardArrivalNoticeWire::NewerReplyReplacedPrevious {
                previous_version: *previous_version,
                current_version: *current_version,
            },
            CardArrivalNotice::PatientWroteAgain => CardArrivalNoticeWire::PatientWroteAgain,
            CardArrivalNotice::OlderMessageReadLate => CardArrivalNoticeWire::OlderMessageReadLate,
        })
        .collect();

    let copied_by_me = if let Some(conv) = raw_conv {
        store
            .reply_copy_records(practice, &card.conversation_id)
            .await
            .map(|copies| {
                copies.iter().any(|c| {
                    c.physician == physician_id
                        && c.reply_id == card.reply_id
                        && c.practice_observation_seq == conv.last_practice_observation_seq
                })
            })
            .unwrap_or(false)
    } else {
        false
    };

    let other_physician_copies = card
        .other_physician_copies
        .iter()
        .enumerate()
        .map(|(idx, c)| CopyAttributionWire {
            copy_id: format!("copy-{}-{idx}", c.reply_id),
            copied_by_physician: c.physician.clone(),
            copied_at: unix_to_rfc3339(c.copied_at),
        })
        .collect();

    let started_at = raw_conv
        .and_then(|c| c.active_review.as_ref().map(|r| r.started_at))
        .or_else(|| raw_conv.map(|c| c.updated_at))
        .unwrap_or(now_ts);
    let review_state = match card.review_state {
        CardReviewState::Idle => CardReviewStateWire::Idle,
        CardReviewState::Reviewing => CardReviewStateWire::Reviewing {
            started_at: unix_to_rfc3339(started_at),
        },
    };

    let channel_banner = match card.channel_banner {
        CardChannelBanner::Connected => CardChannelBannerWire::Connected,
        CardChannelBanner::Disconnected {
            disconnected_at,
            note,
        } => CardChannelBannerWire::Disconnected {
            reason: note,
            disconnected_at: unix_to_rfc3339(disconnected_at),
            spruce_app_url: spruce_app_url.clone(),
        },
    };

    let has_open_concern = raw_conv.is_some_and(|c| !c.associated_concern_ids.is_empty());

    PatientReplyConversationWire::OpenCard(Box::new(ReplyCardWire {
        conversation_id: card.conversation_id,
        patient_workspace_id: workspace.to_owned(),
        patient_display_name: patient_display_name.to_owned(),
        primary_sender_name,
        primary_sender_role,
        non_patient_sender_names: card.non_patient_sender_names,
        spruce_app_url,
        reply_id: card.reply_id,
        conversation_version: raw_conv.map_or(1, |c| c.conversation_version),
        proposal_version,
        has_open_concern,
        patient_messages: map_patient_messages(
            &card.open_messages,
            spruce_msgs_by_id,
            spruce_conv,
            proposed_at,
            covered_ids,
        ),
        practice_messages: map_practice_messages(&card.practice_messages),
        displayed_reply,
        revision_run,
        coda_update_offer,
        notices,
        basis_and_limitations: format_basis(card.basis.as_ref()),
        citations: map_citations(&card.citations, sources_by_key),
        other_physician_copies,
        copied_by_me,
        review_state,
        channel_banner,
        pending_move,
    }))
}

fn token_and_hash(extensions: &Extensions) -> (String, Vec<u8>) {
    if let Some(ctx) = extensions.get::<AuthContext>() {
        let hash = PostgresAuthStore::session_reference(&ctx.token);
        (ctx.token.clone(), hash)
    } else {
        let fallback = "sample-session-token".to_owned();
        let hash = PostgresAuthStore::session_reference(&fallback);
        (fallback, hash)
    }
}

enum LoadConversationError {
    AbsentOnPatient,
    Http(Box<Response>),
}

impl LoadConversationError {
    fn into_boxed_response(self) -> Box<Response> {
        match self {
            Self::AbsentOnPatient => Box::new(reply_error_response(
                ReplyProposalError::ConversationNotFound {
                    conversation_id: String::new(),
                },
            )),
            Self::Http(resp) => resp,
        }
    }
}

async fn load_single_conversation_wire(
    store: &Store,
    connected: &ConnectedCare,
    extensions: &Extensions,
    workspace: &str,
    conversation_id: &str,
) -> Result<PatientReplyConversationWire, Box<Response>> {
    load_conversation_wire_outcome(store, connected, extensions, workspace, conversation_id)
        .await
        .map_err(LoadConversationError::into_boxed_response)
}

async fn load_conversation_wire_outcome(
    store: &Store,
    connected: &ConnectedCare,
    extensions: &Extensions,
    workspace: &str,
    conversation_id: &str,
) -> Result<PatientReplyConversationWire, LoadConversationError> {
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let directory = connected.patients(extensions);
    let now_ts = current_unix_i64();

    settle_expired_spruce_moves_and_replies(
        store,
        auth_store.as_ref(),
        &*directory,
        extensions,
        &identity.practice_id,
        now_ts,
    )
    .await;

    let panel = auth_store.panel(&identity).await;
    let identities = match authorized_patient_identities(&*directory, &panel).await {
        Ok(identities) => identities,
        Err(_) => {
            return Err(LoadConversationError::Http(Box::new(
                identities_unavailable_response(),
            )));
        }
    };
    let patient_display_name = identities
        .get(workspace)
        .map(|i| i.display_name.clone())
        .unwrap_or_else(|| workspace.to_owned());

    let spruce_conv = store
        .spruce_conversation(&identity.practice_id, conversation_id)
        .await
        .map_err(|err| LoadConversationError::Http(Box::new(spruce_error_response(err))))?;

    let ws_spruce_msgs = store
        .patient_linked_spruce_messages(&identity.practice_id, workspace)
        .await
        .map_err(|err| LoadConversationError::Http(Box::new(spruce_error_response(err))))?;
    let spruce_msgs_by_id: BTreeMap<String, SavedSpruceMessage> = ws_spruce_msgs
        .iter()
        .filter(|m| m.conversation_id == conversation_id)
        .map(|m| (m.message_id.clone(), m.clone()))
        .collect();

    // Handle active `Wrong patient?` Undo window without leaking origin details or hiding destination messages!
    if let Some(sc) = &spruce_conv
        && let Some(undo) = &sc.move_undo
        && undo.is_active_at(now_ts)
    {
        if undo.from_workspace == workspace {
            // Origin workspace: `message-matching` Rule 5 forbids retaining readable conversation
            // content, sender identity, or Spruce links on the patient the conversation left.
            let mut wire = PatientReplyConversationWire::NoOpenCard {
                conversation_id: conversation_id.to_owned(),
                patient_workspace_id: workspace.to_owned(),
                primary_sender_name: "Moved conversation".to_owned(),
                primary_sender_role: ChannelSenderRoleWire::Patient,
                non_patient_sender_names: Vec::new(),
                spruce_app_url: None,
                patient_messages: Vec::new(),
                practice_messages: Vec::new(),
                review_state: CardReviewStateWire::Idle,
                can_ask_coda_to_reply: false,
                pending_move: map_pending_move(undo, &sc.link_state, &identities, now_ts),
            };
            resolve_conversation_physician_names(
                &mut wire,
                auth_store.as_ref(),
                &identity.practice_id,
            )
            .await;
            return Ok(wire);
        }

        let linked_to_this_destination = matches!(
            &sc.link_state,
            ConversationLinkState::Linked { workspace: dest_ws, .. } if dest_ws == workspace
        );
        if !linked_to_this_destination {
            return Err(LoadConversationError::AbsentOnPatient);
        }

        // Destination workspace during active Undo window (`Rule 9`): show linked Spruce messages,
        // channel sender, and observed practice messages alongside `Coda is reviewing` and `Undo`,
        // without prematurely relinking `reply_conversation` before Undo settles!
        let mut sorted_msgs: Vec<&SavedSpruceMessage> = ws_spruce_msgs
            .iter()
            .filter(|m| {
                m.conversation_id == conversation_id
                    && matches!(m.standing, MessageStanding::Linked { .. })
            })
            .collect();
        sorted_msgs.sort_by_key(|m| (m.channel_position, m.kept_at));

        let mut non_patient_senders = BTreeSet::new();
        let mut dest_patient_messages = Vec::with_capacity(sorted_msgs.len());
        for (idx, sm) in sorted_msgs.iter().enumerate() {
            let addr = sm.sender_address.as_ref().map(|a| a.as_str());
            let label = sender_display_label(sm.author_name.as_deref(), addr);
            let is_patient = sm
                .author_name
                .as_deref()
                .map(str::trim)
                .is_some_and(|author| author.eq_ignore_ascii_case(patient_display_name.trim()));
            let role = if is_patient {
                ChannelSenderRoleWire::Patient
            } else {
                non_patient_senders.insert(label.clone());
                ChannelSenderRoleWire::NonPatient
            };
            dest_patient_messages.push(ReplyPatientMessageWire {
                message_id: sm.message_id.clone(),
                sequence: (idx + 1) as u64,
                sender_display_name: label,
                sender_role: role,
                text: if sm.body_text.trim().is_empty() {
                    "(Attachment)".to_owned()
                } else {
                    sm.body_text.clone()
                },
                sent_at: unix_to_rfc3339(sm.kept_at),
                arrived_after_proposal: false,
            });
        }
        let (primary_sender_name, primary_sender_role) = if let Some(last_pm) =
            dest_patient_messages.last()
        {
            (last_pm.sender_display_name.clone(), last_pm.sender_role)
        } else {
            derive_primary_channel_sender(&[], &spruce_msgs_by_id, Some(sc), &patient_display_name)
        };
        let spruce_app_url = sorted_msgs.iter().find_map(|m| m.app_url.clone());
        let raw_conv_for_practice = store
            .reply_conversation(&identity.practice_id, conversation_id)
            .await
            .map_err(|err| LoadConversationError::Http(Box::new(reply_error_response(err))))?;
        let dest_practice_messages = raw_conv_for_practice
            .as_ref()
            .map_or_else(Vec::new, |rc| map_practice_messages(&rc.practice_messages));

        let mut wire = PatientReplyConversationWire::NoOpenCard {
            conversation_id: conversation_id.to_owned(),
            patient_workspace_id: workspace.to_owned(),
            primary_sender_name,
            primary_sender_role,
            non_patient_sender_names: non_patient_senders.into_iter().collect(),
            spruce_app_url,
            patient_messages: dest_patient_messages,
            practice_messages: dest_practice_messages,
            review_state: CardReviewStateWire::Reviewing {
                started_at: unix_to_rfc3339(undo.moved_at),
            },
            can_ask_coda_to_reply: false,
            pending_move: map_pending_move(undo, &sc.link_state, &identities, now_ts),
        };
        resolve_conversation_physician_names(&mut wire, auth_store.as_ref(), &identity.practice_id)
            .await;
        return Ok(wire);
    }

    // Outside an active `Wrong patient?` Undo window, a Spruce conversation belongs on this
    // patient workspace if and only if its settled `link_state` points to `workspace`.
    if let Some(sc) = &spruce_conv {
        if matches!(
            &sc.link_state,
            ConversationLinkState::MovedNotAPatient { .. }
        ) && let Ok(cmd_key) = CommandKey::new(format!(
            "settle-reply-move:{}:{}:{}",
            identity.practice_id, conversation_id, sc.move_count
        )) {
            let _ = store
                .act_on_conversation_move(
                    &cmd_key,
                    &identity.practice_id,
                    conversation_id,
                    ConversationLink::NotAPatient,
                )
                .await;
        }
        if !matches!(
            &sc.link_state,
            ConversationLinkState::Linked { workspace: ws, .. } if ws == workspace
        ) {
            return Err(LoadConversationError::AbsentOnPatient);
        }
        sync_spruce_conversation_into_reply_store(
            store,
            &identity.practice_id,
            conversation_id,
            workspace,
            Some(&patient_display_name),
        )
        .await;
    }

    // Keep `reply_conversation.channel` synchronized when the practice Spruce channel was disconnected.
    if let Ok((
        ChannelStanding::Disconnected {
            disconnected_at: Some(_),
        },
        _,
    )) = store.spruce_channel_standing(&identity.practice_id).await
        && let Ok(Some(existing_rc)) = store
            .reply_conversation(&identity.practice_id, conversation_id)
            .await
        && matches!(existing_rc.channel, ChannelConnection::Connected)
    {
        let _ = store
            .set_reply_channel_connection(
                &identity.practice_id,
                conversation_id,
                ChannelConnection::Disconnected {
                    note:
                        "Spruce channel is disconnected. Open in Spruce to reply manually, or mark Done."
                            .to_owned(),
                    disconnected_at: now_ts,
                },
            )
            .await;
    }

    let (token, session_hash) = token_and_hash(extensions);
    let real_now_unix = auth::now();
    let access = WorkspaceAccess {
        session_hash: &session_hash,
        practice: &identity.practice_id,
        physician: &identity.physician_id,
        workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &token,
        now_unix: real_now_unix,
    };

    let raw_conv = store
        .reply_conversation(&identity.practice_id, conversation_id)
        .await
        .map_err(|err| match err {
            ReplyProposalError::ConversationNotFound { .. }
            | ReplyProposalError::ConversationNotLinkedToPatient { .. }
            | ReplyProposalError::ConversationWithheld { .. } => {
                LoadConversationError::AbsentOnPatient
            }
            other => LoadConversationError::Http(Box::new(reply_error_response(other))),
        })?;
    let Some(rc) = &raw_conv else {
        return Err(LoadConversationError::AbsentOnPatient);
    };
    if !matches!(
        &rc.link,
        ConversationLink::LinkedPatient { workspace: rc_ws } if rc_ws == workspace
    ) {
        return Err(LoadConversationError::AbsentOnPatient);
    }

    let surface = store
        .physician_reply_surface(&access, conversation_id, &mut authority)
        .await
        .map_err(|err| match err {
            ReplyProposalError::ConversationNotFound { .. }
            | ReplyProposalError::ConversationNotLinkedToPatient { .. }
            | ReplyProposalError::ConversationWithheld { .. } => {
                LoadConversationError::AbsentOnPatient
            }
            other => LoadConversationError::Http(Box::new(reply_error_response(other))),
        })?;

    let mut wire = project_reply_surface_wire(
        store,
        &identity.practice_id,
        &identity.physician_id,
        workspace,
        &patient_display_name,
        conversation_id,
        surface,
        raw_conv.as_ref(),
        spruce_conv.as_ref(),
        &spruce_msgs_by_id,
        &identities,
        now_ts,
    )
    .await;
    resolve_conversation_physician_names(&mut wire, auth_store.as_ref(), &identity.practice_id)
        .await;
    Ok(wire)
}

/// Replaces physician identifiers with display names across copy attributions,
/// pending `Wrong patient?` move banners, and `No reply needed` undo banners.
async fn resolve_conversation_physician_names(
    wire: &mut PatientReplyConversationWire,
    auth: &dyn AuthStore,
    practice: &str,
) {
    let Ok(accounts) = auth.practice_accounts(practice).await else {
        return;
    };
    let names: BTreeMap<&str, &str> = accounts
        .iter()
        .map(|a| (a.identity.physician_id.as_str(), a.display_name.as_str()))
        .collect();

    let resolve_pending = |pm: &mut Option<PendingSpruceMoveWire>| {
        if let Some(m) = pm
            && let Some(name) = names.get(m.moved_by_physician.as_str())
        {
            m.moved_by_physician = (*name).to_owned();
        }
    };

    match wire {
        PatientReplyConversationWire::OpenCard(card) => {
            for copy in &mut card.other_physician_copies {
                if let Some(name) = names.get(copy.copied_by_physician.as_str()) {
                    copy.copied_by_physician = (*name).to_owned();
                }
            }
            resolve_pending(&mut card.pending_move);
        }
        PatientReplyConversationWire::ClosedUndoBanner {
            closed_by_physician,
            pending_move,
            ..
        } => {
            if let Some(name) = names.get(closed_by_physician.as_str()) {
                *closed_by_physician = (*name).to_owned();
            }
            resolve_pending(pending_move);
        }
        PatientReplyConversationWire::NoOpenCard { pending_move, .. } => {
            resolve_pending(pending_move);
        }
    }
}

async fn list_patient_replies(
    Path(workspace): Path<String>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyRead) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Patient messages and reply drafts could not be loaded right now." })),
        )
            .into_response();
    }
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return Json(PatientReplyListResponseWire {
            patient_workspace_id: workspace,
            conversations: Vec::new(),
        })
        .into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    match store.spruce_channel_standing(&identity.practice_id).await {
        Ok((
            ChannelStanding::Disconnected {
                disconnected_at: None,
            },
            _,
        )) => {
            return Json(PatientReplyListResponseWire {
                patient_workspace_id: workspace,
                conversations: Vec::new(),
            })
            .into_response();
        }
        Ok(_) => {}
        Err(err) => return spruce_error_response(err),
    }

    let auth_store = connected.auth_store(extensions);
    let directory = connected.patients(extensions);
    let now_ts = current_unix_i64();
    settle_expired_spruce_moves_and_replies(
        &store,
        auth_store.as_ref(),
        &*directory,
        extensions,
        &identity.practice_id,
        now_ts,
    )
    .await;

    let mut conv_ids = BTreeSet::new();
    let spruce_convs = match store
        .open_spruce_conversations_for_physician(auth_store.as_ref(), &identity, now_ts)
        .await
    {
        Ok(rows) => rows,
        Err(err) => return spruce_error_response(err),
    };
    for sc in spruce_convs {
        if sc.workspace == workspace
            || sc
                .move_undo
                .as_ref()
                .is_some_and(|u| u.is_active_at(now_ts) && u.from_workspace == workspace)
        {
            conv_ids.insert(sc.conversation_id);
        }
    }

    let linked_msgs = match store
        .patient_linked_spruce_messages(&identity.practice_id, &workspace)
        .await
    {
        Ok(msgs) => msgs,
        Err(err) => return spruce_error_response(err),
    };
    for m in linked_msgs {
        conv_ids.insert(m.conversation_id);
    }

    // Also discover conversations from durable Spruce review triggers (including conversations
    // currently in their active `Undo` window after `Wrong patient?` -> `Not a patient` or
    // `AnotherPatient`, or after channel disconnect).
    let triggered_ids = match discover_reply_conversation_ids_for_workspace(
        &store,
        &identity.practice_id,
        &workspace,
        now_ts,
    )
    .await
    {
        Ok(ids) => ids,
        Err(err) => return spruce_error_response(err),
    };
    for cid in triggered_ids {
        conv_ids.insert(cid);
    }

    let mut conversations = Vec::new();
    for cid in conv_ids {
        match load_conversation_wire_outcome(&store, &connected, extensions, &workspace, &cid).await
        {
            Ok(wire) => conversations.push(wire),
            Err(LoadConversationError::AbsentOnPatient) => continue,
            Err(LoadConversationError::Http(resp)) => return *resp,
        }
    }

    Json(PatientReplyListResponseWire {
        patient_workspace_id: workspace,
        conversations,
    })
    .into_response()
}

pub(crate) async fn discover_reply_conversation_ids_for_workspace(
    store: &Store,
    practice: &str,
    workspace: &str,
    now_ts: i64,
) -> Result<Vec<String>, SpruceMatchingError> {
    let mut ids = BTreeSet::new();
    let triggers = store.spruce_review_triggers(practice).await?;
    let candidate_convs: BTreeSet<String> =
        triggers.into_iter().map(|t| t.conversation_id).collect();
    for cid in candidate_convs {
        if let Some(sc) = store.spruce_conversation(practice, &cid).await? {
            let linked_here = matches!(
                &sc.link_state,
                ConversationLinkState::Linked { workspace: ws, .. } if ws == workspace
            );
            let undo_from_here = sc
                .move_undo
                .as_ref()
                .is_some_and(|u| u.is_active_at(now_ts) && u.from_workspace == workspace);
            if linked_here || undo_from_here {
                ids.insert(cid);
            }
        }
    }
    Ok(ids.into_iter().collect())
}

async fn get_patient_reply(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyRead) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Reply card could not be loaded right now." })),
        )
            .into_response();
    }
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let _ = store
        .open_spruce_conversation(
            connected.auth_store(extensions).as_ref(),
            &identity,
            &conversation_id,
            current_unix_i64(),
        )
        .await;

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

fn map_expected_base(base: ExpectedDraftBaseWire) -> ExpectedDraftBase {
    match base {
        ExpectedDraftBaseWire::ProposalVersion { version } => {
            ExpectedDraftBase::ProposalVersion(version)
        }
        ExpectedDraftBaseWire::PhysicianDraftRevision { revision } => {
            ExpectedDraftBase::DraftRevision(revision)
        }
    }
}

async fn save_reply_draft(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Your draft edits are still on screen." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 128_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<SaveReplyProposalDraftBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (token, session_hash) = token_and_hash(extensions);
    let real_now_unix = auth::now();
    let access = WorkspaceAccess {
        session_hash: &session_hash,
        practice: &identity.practice_id,
        physician: &identity.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &token,
        now_unix: real_now_unix,
    };

    if let Err(err) = store
        .edit_reply_proposal_draft(
            &access,
            &conversation_id,
            map_expected_base(payload.expected_base),
            &payload.text,
            &mut authority,
        )
        .await
    {
        return reply_error_response(err);
    }

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

async fn revise_reply_draft(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not start that revision right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 128_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<RequestReplyProposalRevisionBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (token, session_hash) = token_and_hash(extensions);
    let real_now_unix = auth::now();
    let access = WorkspaceAccess {
        session_hash: &session_hash,
        practice: &identity.practice_id,
        physician: &identity.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &token,
        now_unix: real_now_unix,
    };

    // Ensure the physician holds an active private draft at the expected revision
    // before calling `request_reply_draft_revision`.
    let target_draft_revision = match payload.expected_base {
        ExpectedDraftBaseWire::ProposalVersion { version } => {
            let card = match store
                .edit_reply_proposal_draft(
                    &access,
                    &conversation_id,
                    ExpectedDraftBase::ProposalVersion(version),
                    &payload.current_text,
                    &mut authority,
                )
                .await
            {
                Ok(card) => card,
                Err(err) => return reply_error_response(err),
            };
            match card.displayed_text {
                DisplayedReplyText::PhysicianDraft { draft_revision, .. } => draft_revision,
                _ => 1,
            }
        }
        ExpectedDraftBaseWire::PhysicianDraftRevision { revision } => {
            let surface = match store
                .physician_reply_surface(&access, &conversation_id, &mut authority)
                .await
            {
                Ok(s) => s,
                Err(err) => return reply_error_response(err),
            };
            let needs_save = match &surface {
                PhysicianReplySurface::OpenCard(c) => match &c.displayed_text {
                    DisplayedReplyText::PhysicianDraft {
                        draft_revision,
                        text,
                        ..
                    } => *draft_revision == revision && text != &payload.current_text,
                    _ => false,
                },
                _ => false,
            };
            if needs_save {
                let card = match store
                    .edit_reply_proposal_draft(
                        &access,
                        &conversation_id,
                        ExpectedDraftBase::DraftRevision(revision),
                        &payload.current_text,
                        &mut authority,
                    )
                    .await
                {
                    Ok(card) => card,
                    Err(err) => return reply_error_response(err),
                };
                match card.displayed_text {
                    DisplayedReplyText::PhysicianDraft { draft_revision, .. } => draft_revision,
                    _ => revision + 1,
                }
            } else {
                revision
            }
        }
    };

    let (roles, skills) = with_review_receiver_setup(&store, extensions).await;
    let recipe = local_product::runtime::reply::recipe(&roles.background, &skills);
    let rev_input = RevisionInput {
        key: payload.idempotency_key,
        expected_revision: target_draft_revision,
        instruction: payload.instruction,
        timezone: payload.physician_time_zone,
        scenario: None,
    };

    let accepted = match store
        .request_reply_draft_revision(
            &access,
            &conversation_id,
            &rev_input,
            &skills,
            &recipe,
            &mut authority,
        )
        .await
    {
        Ok(accepted) => accepted,
        Err(err) => return reply_error_response(err),
    };

    if matches!(accepted, ProposalRevisionAccepted::New { .. })
        && let Some(live) = extensions.get::<Arc<Live>>()
    {
        let _ = live
            .bus
            .publish(physician_bus::Event::WorkspaceChanged {
                workspace_id: workspace.clone(),
            })
            .await;
    }

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

async fn restore_my_version(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not restore your earlier wording right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<RestoreMyReplyVersionBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(cmd_key) = CommandKey::new(&payload.idempotency_key) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (token, session_hash) = token_and_hash(extensions);
    let real_now_unix = auth::now();
    let access = WorkspaceAccess {
        session_hash: &session_hash,
        practice: &identity.practice_id,
        physician: &identity.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &token,
        now_unix: real_now_unix,
    };

    if let Err(err) = store
        .restore_my_reply_version(
            &access,
            &cmd_key,
            &conversation_id,
            payload.expected_draft_revision,
            &mut authority,
        )
        .await
    {
        return reply_error_response(err);
    }

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

async fn choose_coda_update(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Your draft was preserved." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<ChooseCodaReplyUpdateBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(cmd_key) = CommandKey::new(&payload.idempotency_key) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (token, session_hash) = token_and_hash(extensions);
    let real_now_unix = auth::now();
    let access = WorkspaceAccess {
        session_hash: &session_hash,
        practice: &identity.practice_id,
        physician: &identity.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &token,
        now_unix: real_now_unix,
    };

    if let Err(err) = store
        .choose_coda_reply_update(
            &access,
            &cmd_key,
            &conversation_id,
            payload.expected_draft_revision,
            payload.expected_proposal_version,
            &mut authority,
        )
        .await
    {
        return reply_error_response(err);
    }

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

async fn copy_reply(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not record Copy reply right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<CopyReplyBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(cmd_key) = CommandKey::new(&payload.idempotency_key) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (token, session_hash) = token_and_hash(extensions);
    let real_now_unix = auth::now();
    let access = WorkspaceAccess {
        session_hash: &session_hash,
        practice: &identity.practice_id,
        physician: &identity.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &token,
        now_unix: real_now_unix,
    };

    let outcome = match store
        .copy_reply(
            &access,
            &cmd_key,
            &conversation_id,
            map_expected_base(payload.expected_base),
            &mut authority,
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return reply_error_response(err),
    };
    let copied_text = match outcome {
        local_product::runtime::reply::proposals::CopyReplyOutcome::Recorded(record)
        | local_product::runtime::reply::proposals::CopyReplyOutcome::Duplicate(record) => {
            record.copied_text
        }
    };
    publish_bus_resync(extensions).await;

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(surface) => Json(CopyReplyResponseWire {
            copied_text,
            surface,
        })
        .into_response(),
        Err(resp) => *resp,
    }
}

async fn close_no_reply_needed(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not close that reply right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<NoReplyNeededBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(cmd_key) = CommandKey::new(&payload.idempotency_key) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (token, session_hash) = token_and_hash(extensions);
    let real_now_unix = auth::now();
    let access = WorkspaceAccess {
        session_hash: &session_hash,
        practice: &identity.practice_id,
        physician: &identity.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &token,
        now_unix: real_now_unix,
    };

    if let Err(err) = store
        .close_reply_no_reply_needed(
            &access,
            &cmd_key,
            &conversation_id,
            payload.expected_proposal_version,
            &mut authority,
        )
        .await
    {
        return reply_error_response(err);
    }
    publish_bus_resync(extensions).await;

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

async fn undo_no_reply_needed(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not undo that closure right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<ReplyCommandKeyBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(cmd_key) = CommandKey::new(&payload.idempotency_key) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (token, session_hash) = token_and_hash(extensions);
    let real_now_unix = auth::now();
    let access = WorkspaceAccess {
        session_hash: &session_hash,
        practice: &identity.practice_id,
        physician: &identity.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &token,
        now_unix: real_now_unix,
    };

    if let Err(err) = store
        .undo_close_no_reply_needed(&access, &cmd_key, &conversation_id, &mut authority)
        .await
    {
        return reply_error_response(err);
    }
    publish_bus_resync(extensions).await;

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

async fn ask_coda_to_reply(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not request a reply right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<ReplyCommandKeyBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(cmd_key) = CommandKey::new(&payload.idempotency_key) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (token, session_hash) = token_and_hash(extensions);
    let real_now_unix = auth::now();
    let access = WorkspaceAccess {
        session_hash: &session_hash,
        practice: &identity.practice_id,
        physician: &identity.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &token,
        now_unix: real_now_unix,
    };

    if let Err(err) = store
        .ask_coda_to_reply(&access, &cmd_key, &conversation_id, &mut authority)
        .await
    {
        return reply_error_response(err);
    }
    publish_bus_resync(extensions).await;

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

async fn move_conversation(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(
                json!({ "detail": "Not saved. Coda could not move that conversation right now." }),
            ),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<MoveSpruceConversationBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (roles, skills) = with_review_receiver_setup(&store, extensions).await;
    let practice_auth = AuthPractice(auth_store.as_ref());
    let receiver = PracticeJobReviewReceiver::new(store.clone(), &roles, &skills, &practice_auth);
    let now_ts = current_unix_i64();

    let destination = match payload.destination {
        MoveDestinationWire::AnotherPatient {
            patient_workspace_id,
        } => MoveDestination::AnotherPatient {
            workspace: patient_workspace_id,
        },
        MoveDestinationWire::NotAPatient => MoveDestination::NotAPatient,
    };
    let undo_seconds = payload
        .undo_window_seconds
        .unwrap_or(DEFAULT_MOVE_UNDO_WINDOW_SECONDS)
        .max(1);

    if let Err(err) = store
        .move_spruce_conversation(
            auth_store.as_ref(),
            &identity,
            MoveSpruceConversationCommand {
                conversation_id: conversation_id.clone(),
                destination,
                undo_window_seconds: undo_seconds,
                command_key: payload.idempotency_key,
            },
            Some(&receiver as &dyn ReviewReceiver),
            now_ts,
        )
        .await
    {
        return spruce_error_response(err);
    }
    publish_bus_resync(extensions).await;

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

async fn undo_move_conversation(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not undo that move right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<ReplyCommandKeyBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (roles, skills) = with_review_receiver_setup(&store, extensions).await;
    let practice_auth = AuthPractice(auth_store.as_ref());
    let receiver = PracticeJobReviewReceiver::new(store.clone(), &roles, &skills, &practice_auth);
    let now_ts = current_unix_i64();

    if let Err(err) = store
        .undo_spruce_conversation_move(
            auth_store.as_ref(),
            &identity,
            &conversation_id,
            &payload.idempotency_key,
            Some(&receiver as &dyn ReviewReceiver),
            now_ts,
        )
        .await
    {
        return spruce_error_response(err);
    }
    publish_bus_resync(extensions).await;

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

async fn mark_disconnected_done(
    Path((workspace, conversation_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not mark that disconnected reply done right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<DisconnectedReplyDoneBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(cmd_key) = CommandKey::new(&payload.idempotency_key) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (token, session_hash) = token_and_hash(extensions);
    let real_now_unix = auth::now();
    let access = WorkspaceAccess {
        session_hash: &session_hash,
        practice: &identity.practice_id,
        physician: &identity.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &token,
        now_unix: real_now_unix,
    };

    let expected = match payload.standing {
        DisconnectedCardStandingWire::Ready { proposal_version } => {
            ExpectedDisconnectedCard::Ready {
                reply_id: payload.expected_reply_id,
                proposal_version,
            }
        }
        DisconnectedCardStandingWire::Withheld => ExpectedDisconnectedCard::Withheld {
            reply_id: payload.expected_reply_id,
        },
    };

    if let Err(err) = store
        .mark_disconnected_reply_done(
            &access,
            &cmd_key,
            &conversation_id,
            expected,
            &mut authority,
        )
        .await
    {
        return reply_error_response(err);
    }
    publish_bus_resync(extensions).await;

    match load_single_conversation_wire(
        &store,
        &connected,
        extensions,
        &workspace,
        &conversation_id,
    )
    .await
    {
        Ok(wire) => Json(wire).into_response(),
        Err(resp) => *resp,
    }
}

fn spruce_slice_failed(err: impl std::fmt::Display) -> FailedProducerWire {
    FailedProducerWire {
        producer: ProducerSliceWire::SpruceMessages,
        label: "Spruce messages".to_owned(),
        message: err.to_string(),
    }
}

fn truncate_today_preview(text: &str) -> String {
    let trimmed = text.trim();
    const MAX_CHARS: usize = 140;
    if trimmed.chars().count() <= MAX_CHARS {
        trimmed.to_owned()
    } else {
        let head: String = trimmed.chars().take(MAX_CHARS).collect();
        format!("{head}…")
    }
}

/// Builds supplemental Today rows and `messages_to_match_count` from durable
/// Spruce matching and open reply cards for the signed-in physician.
pub(crate) async fn spruce_today_rows_for_physician(
    store: &Store,
    ctx: &AuthContext,
    directory: &dyn PatientDirectory,
    extensions: &Extensions,
    patient_names: &BTreeMap<String, String>,
    target_workspace: Option<&str>,
) -> Result<(usize, Vec<serde_json::Value>), FailedProducerWire> {
    if is_faulted(extensions, MatchingReplyFollowUpsFault::MatchingRead)
        || is_faulted(extensions, MatchingReplyFollowUpsFault::ReplyRead)
    {
        return Err(spruce_slice_failed(
            "Spruce messages could not be loaded right now.",
        ));
    }
    match store
        .spruce_channel_standing(&ctx.identity.practice_id)
        .await
    {
        Ok((
            ChannelStanding::Disconnected {
                disconnected_at: None,
            },
            _,
        )) => return Ok((0, Vec::new())),
        Ok(_) => {}
        Err(err) => return Err(spruce_slice_failed(err)),
    }

    let now_ts = current_unix_i64();
    settle_expired_spruce_moves_and_replies(
        store,
        ctx.auth.store.as_ref(),
        directory,
        extensions,
        &ctx.identity.practice_id,
        now_ts,
    )
    .await;

    let mut rows = Vec::new();
    let summary = store
        .today_spruce_summary_for_physician(ctx.auth.store.as_ref(), &ctx.identity)
        .await
        .map_err(spruce_slice_failed)?;
    let waiting_count = summary.messages_to_match.len();
    if target_workspace.is_none() {
        for unmatched in &summary.messages_to_match {
            let sender_addr = unmatched.sender_address.as_ref().map(|a| a.as_str());
            let sender_name = sender_display_label(unmatched.author_name.as_deref(), sender_addr);
            let (urgency, title, reason) = match &unmatched.urgency {
                MessageUrgency::Urgent { reason, .. } => (
                    "now",
                    "Urgent unmatched message",
                    format!("{reason} — {}", unmatched.body_text),
                ),
                MessageUrgency::Ordinary => {
                    ("today", "Message to match", unmatched.body_text.clone())
                }
            };
            rows.push(json!({
                "patient_workspace_id": unmatched
                    .suggested_patient
                    .as_ref()
                    .map(|s| s.workspace.as_str())
                    .unwrap_or(""),
                "patient_display_name": sender_name,
                "item_id": format!("spruce-match:{}", unmatched.message_id),
                "version_id": format!("waiting:{}", unmatched.waiting_since),
                "waiting_since": unix_to_rfc3339(unmatched.waiting_since),
                "urgency": urgency,
                "kind": "unmatched_message",
                "title": title,
                "reason": reason,
                "seen": false,
                "can_mark_done": false,
                "matching_message_id": unmatched.message_id,
            }));
        }
        for unreadable in &summary.unreadable_messages {
            rows.push(json!({
                "patient_workspace_id": "",
                "patient_display_name": "Spruce message",
                "item_id": format!("spruce-unreadable:{}", unreadable.message_id),
                "version_id": format!("cant-read:{}", unreadable.kept_at),
                "waiting_since": unix_to_rfc3339(unreadable.kept_at),
                "urgency": "today",
                "kind": "unreadable_message",
                "title": "Couldn't read Spruce message",
                "reason": unreadable.failure_reason,
                "seen": false,
                "can_mark_done": false,
                "matching_message_id": unreadable.message_id,
            }));
        }
    }

    // Discover active `(workspace, conversation_id)` pairs from practice-level Spruce tables
    // rather than scanning every workspace in the practice roster.
    let mut convs_by_workspace: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let spruce_convs = store
        .open_spruce_conversations_for_physician(ctx.auth.store.as_ref(), &ctx.identity, now_ts)
        .await
        .map_err(spruce_slice_failed)?;
    for sc in spruce_convs {
        if target_workspace.is_none_or(|target| target == sc.workspace) {
            convs_by_workspace
                .entry(sc.workspace)
                .or_default()
                .insert(sc.conversation_id);
        }
    }
    let triggers = store
        .spruce_review_triggers(&ctx.identity.practice_id)
        .await
        .map_err(spruce_slice_failed)?;
    for tr in triggers {
        if target_workspace.is_none_or(|target| target == tr.workspace) {
            convs_by_workspace
                .entry(tr.workspace)
                .or_default()
                .insert(tr.conversation_id);
        }
    }
    if let Some(ws) = target_workspace {
        let msgs = store
            .patient_linked_spruce_messages(&ctx.identity.practice_id, ws)
            .await
            .map_err(spruce_slice_failed)?;
        for m in msgs {
            convs_by_workspace
                .entry(ws.to_owned())
                .or_default()
                .insert(m.conversation_id);
        }
    }

    let hash = PostgresAuthStore::session_reference(&ctx.token);
    let real_now_unix = auth::now();

    for (ws, conv_ids) in convs_by_workspace {
        if ctx.auth.store.membership(&ctx.identity, &ws).await != Membership::Allowed {
            continue;
        }
        let patient_name = patient_names
            .get(&ws)
            .cloned()
            .unwrap_or_else(|| ws.clone());
        let ws_spruce_msgs = store
            .patient_linked_spruce_messages(&ctx.identity.practice_id, &ws)
            .await
            .map_err(spruce_slice_failed)?;
        let ws_msgs_by_id: BTreeMap<String, SavedSpruceMessage> = ws_spruce_msgs
            .into_iter()
            .map(|m| (m.message_id.clone(), m))
            .collect();

        for cid in conv_ids {
            let spruce_conv = store
                .spruce_conversation(&ctx.identity.practice_id, &cid)
                .await
                .map_err(spruce_slice_failed)?;
            // Never emit a Today reply row during an active `Wrong patient?` Undo window or on a
            // workspace that does not match the conversation's settled link.
            if let Some(sc) = &spruce_conv {
                if sc
                    .move_undo
                    .as_ref()
                    .is_some_and(|u| u.is_active_at(now_ts))
                {
                    continue;
                }
                if !matches!(
                    &sc.link_state,
                    ConversationLinkState::Linked { workspace: linked_ws, .. } if linked_ws == &ws
                ) {
                    continue;
                }
            }

            let raw_conv = store
                .reply_conversation(&ctx.identity.practice_id, &cid)
                .await
                .map_err(spruce_slice_failed)?;
            let Some(rc) = &raw_conv else {
                continue;
            };
            if !matches!(
                &rc.link,
                ConversationLink::LinkedPatient { workspace: rc_ws } if rc_ws == &ws
            ) {
                continue;
            }

            let access = WorkspaceAccess {
                session_hash: &hash,
                practice: &ctx.identity.practice_id,
                physician: &ctx.identity.physician_id,
                workspace: &ws,
            };
            let mut authority = SampleRuntimeAuthority {
                auth: ctx.auth.store.as_ref(),
                token: &ctx.token,
                now_unix: real_now_unix,
            };
            let card = match store.reply_card(&access, &cid, &mut authority).await {
                Ok(Some(card)) => card,
                Ok(None)
                | Err(
                    ReplyProposalError::ConversationNotFound { .. }
                    | ReplyProposalError::ConversationNotLinkedToPatient { .. }
                    | ReplyProposalError::ConversationWithheld { .. },
                ) => continue,
                Err(err) => return Err(spruce_slice_failed(err)),
            };
            let (sender_name, _) = derive_primary_channel_sender(
                &card.open_messages,
                &ws_msgs_by_id,
                spruce_conv.as_ref(),
                &patient_name,
            );
            let flagged = raw_conv
                .as_ref()
                .is_some_and(|c| !c.associated_concern_ids.is_empty());
            let waiting_ts = raw_conv.as_ref().map_or(now_ts, |c| c.updated_at);
            // `message-reply/README.md:69-70`: Today row shows the start of the sender's newest words,
            // never the drafted reply text or the linked patient's chart name.
            let newest_sender_words = card
                .open_messages
                .last()
                .map(|m| truncate_today_preview(&m.text))
                .unwrap_or_default();
            let title = match &card.displayed_text {
                DisplayedReplyText::Withheld => {
                    if flagged {
                        "Reply withheld · Flagged"
                    } else {
                        "Reply withheld while Coda reviews"
                    }
                }
                DisplayedReplyText::CodaProposal { .. }
                | DisplayedReplyText::PhysicianDraft { .. } => {
                    if flagged {
                        "Drafted a reply · Flagged"
                    } else {
                        "Drafted a reply"
                    }
                }
            };
            rows.push(json!({
                "patient_workspace_id": ws,
                "patient_display_name": sender_name,
                "item_id": format!("spruce-reply:{cid}:{}", card.reply_id),
                "version_id": format!("reply:{}:{}", card.reply_id, raw_conv.map_or(1, |c| c.conversation_version)),
                "waiting_since": unix_to_rfc3339(waiting_ts),
                "urgency": if flagged { "now" } else { "today" },
                "kind": "drafted_reply",
                "title": title,
                "reason": newest_sender_words,
                "seen": false,
                "can_mark_done": false,
                "spruce_conversation_id": cid,
            }));
        }
    }

    Ok((waiting_count, rows))
}
