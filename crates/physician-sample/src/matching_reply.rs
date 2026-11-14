//! HTTP handlers and durable `Store` composition for Spruce `Messages to match`,
//! `Filtered`, unreadable channel items, and patient-scoped Spruce reply cards,
//! private drafts, Coda revisions, copy attributions, and conversation moves.

#![expect(
    dead_code,
    reason = "patient reply card, draft, and revision routes land in companion slice"
)]
#![expect(
    unused_imports,
    reason = "patient reply card, draft, and revision imports are exercised in companion slice"
)]

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
        | SpruceMatchingError::InvalidReaderLease { .. } => (
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
