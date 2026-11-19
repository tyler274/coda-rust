//! Live Spruce channel reader integration, transient attachment intake, and
//! matching/review orchestration on top of [`crate::runtime::spruce_matching`].
//!
//! Bridges `spruce::changes::Reader` with Coda's durable Spruce channel state:
//! - Acquires the practice's exclusive [`ChannelReaderLease`] and checks the
//!   release-generation fence before polling or filing.
//! - Polls `Reader::changes` and, when accepted, `Reader::read_backlog_page`
//!   using the persisted cursor without retaining `signedUrl`s or call recordings.
//! - Distinguishes `401`/`403` revocation (`RevokedOrBlocked` + disconnected reply
//!   cards) from transient outages (`503`/timeout -> `TransientFailure`).
//! - Downloads readable attachment and fax bytes transiently for linked
//!   conversations and converts them via [`crate::attachments`].
//! - Runs unmatched message classification ([`crate::spruce_matching_recipe`]),
//!   settles 15-minute fallbacks, dispatches review requests, and records
//!   outbound practice messages observed in Spruce.

use std::collections::{BTreeMap, BTreeSet};

use physician_auth::AuthStore;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, TransactionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ModelRoles;
use crate::attachments::{
    AttachmentMediaKind, PagedDocumentInput, SavedAttachmentInput, ingest_attachment_batch,
    ingest_paged_document,
};
use crate::physician_chat::PhysicianSkills;
use crate::physician_sources::{
    Bucket, RecordDate, RecordEnvelope, RecordVersion, SourcePatient, SourceRecordKind,
    SourceRevision,
};
use crate::runtime::dispatch::ReviewReceiver;
use crate::runtime::entities::source_pull;
use crate::runtime::reply::proposals::{
    ChannelConnection, ObservePracticeMessage, PracticeMessageKind,
};
use crate::runtime::sources::{DeliveryScope, SourcePull, record_delivery_in_tx};
use crate::runtime::spruce_matching::{
    BacklogStanding, ChannelHealth, ChannelStanding, ChannelStopCause, ConversationLinkState,
    IncomingSpruceItem, OutsideParticipant, ReadableSpruceItem, SenderAddress,
    SpruceAttachmentSummary, SpruceContactPatient, SpruceContactRecord, SpruceIngestOutcome,
    SpruceMatchingError, UnreadableSpruceItem, entities as spruce_entities,
};
use crate::runtime::{Store, engine};
use crate::spruce_matching_recipe::{MatchingRunStatus, run_spruce_message_matching};

/// Options controlling one exclusive Spruce channel sync pass.
pub struct SpruceSyncPass<'a> {
    /// Practice owning the channel.
    pub practice: &'a str,
    /// Stable reader identity acquiring the exclusive channel lease.
    pub holder_id: &'a str,
    /// Reader lease duration in Unix seconds (`>= 1`).
    pub lease_ttl_seconds: i64,
    /// Current Unix timestamp for lease, fence, ingest, and settlement checks.
    pub now_ts: i64,
    /// Model roles used when classifying unmatched messages (`roles.background`).
    pub roles: &'a ModelRoles,
    /// Loaded skill catalog offered to classification runs.
    pub skills: &'a PhysicianSkills,
    /// Receiver admitting downstream patient reviews (`PracticeJob` / `PatientReview`).
    pub review_receiver: &'a dyn ReviewReceiver,
}

/// Summary counters returned by a completed [`Store::sync_spruce_channel_pass`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpruceSyncPassReport {
    /// Monotonic channel `place` after saving this pass's messages and cursor.
    pub channel_position_after: u64,
    /// Readable inbound items delivered or already kept by idempotent replay.
    pub ingested_count: usize,
    /// Unreadable or deleted items recorded cleanly without blocking sibling messages.
    pub unreadable_count: usize,
    /// Outbound practice messages observed on already-linked conversations.
    pub outbound_observed_count: usize,
    /// Attachment or fax revisions persisted transiently into linked workspaces.
    pub attachments_ingested_count: usize,
    /// Attachment fetches or decodes that failed (kept explicitly unreadable).
    pub attachments_failed_count: usize,
    /// Pre-connection backlog items traversed after backlog acceptance.
    pub backlog_items_read: usize,
    /// Conversations on Spruce that could not be decoded or read on this page.
    pub unread_conversations: usize,
    /// Unmatched messages evaluated by [`run_spruce_message_matching`].
    pub matched_run_count: usize,
    /// Messages settled via the 15-minute deterministic fallback rule.
    pub fallback_routed_count: usize,
    /// Patient review triggers handed to [`ReviewReceiver`].
    pub dispatched_review_count: usize,
    /// Resulting practice [`ChannelStanding`] after this pass.
    pub channel_standing: ChannelStanding,
}

/// Outcome of calling [`Store::sync_spruce_channel_pass`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum SpruceSyncOutcome {
    /// Poll, ingest, attachments, settlement, and review dispatch completed cleanly.
    Completed(SpruceSyncPassReport),
    /// Spruce credentials were rejected (`401`/`403`); reconnect required without advancing cursor.
    ReconnectRequired {
        /// Plain-language failure detail.
        reason: String,
    },
    /// Transient transport or `503` failure; cursor preserved for quiet/retry recovery.
    TransientFailure {
        /// Plain-language failure detail.
        reason: String,
    },
    /// Channel is disconnected; no Spruce requests were issued.
    SkippedDisconnected,
    /// Another worker holds an active reader lease for this practice.
    ReaderLeaseBusy,
}

enum PollEntryKind {
    Change(spruce::changes::Observed),
    Backlog(spruce::changes::Observed),
}

impl PollEntryKind {
    fn observed(&self) -> &spruce::changes::Observed {
        match self {
            Self::Change(obs) | Self::Backlog(obs) => obs,
        }
    }
}

struct PendingAttachmentSync {
    conversation_id: String,
    item: Box<spruce::payload::Item>,
}

impl Store {
    /// Executes one exclusive Spruce polling, ingestion, attachment intake,
    /// matching, 15-minute fallback settlement, and review dispatch pass.
    pub async fn sync_spruce_channel_pass<P>(
        &self,
        reader: &spruce::changes::Reader,
        auth: &dyn AuthStore,
        pass: &SpruceSyncPass<'_>,
        matching_provider: Option<&mut P>,
    ) -> Result<SpruceSyncOutcome, SpruceMatchingError>
    where
        P: engine::recipe::Provider + Send,
    {
        let lease = match self
            .acquire_spruce_reader_lease(
                pass.practice,
                pass.holder_id,
                pass.lease_ttl_seconds,
                pass.now_ts,
            )
            .await
        {
            Ok(lease) => lease,
            Err(SpruceMatchingError::ReaderLeaseHeld { .. }) => {
                return Ok(SpruceSyncOutcome::ReaderLeaseBusy);
            }
            Err(err) => return Err(err),
        };

        let result = self
            .run_spruce_sync_with_lease(reader, auth, pass, &lease.lease_token, matching_provider)
            .await;

        let _ = self
            .release_spruce_reader_lease(pass.practice, &lease.lease_token, pass.now_ts)
            .await;

        result
    }

    async fn run_spruce_sync_with_lease<P>(
        &self,
        reader: &spruce::changes::Reader,
        auth: &dyn AuthStore,
        pass: &SpruceSyncPass<'_>,
        lease_token: &str,
        matching_provider: Option<&mut P>,
    ) -> Result<SpruceSyncOutcome, SpruceMatchingError>
    where
        P: engine::recipe::Provider + Send,
    {
        let overview = self
            .enforce_spruce_release_fence(pass.practice, pass.now_ts)
            .await?;

        let (place, start_boundary, saved_cursor_str, backlog, health) = match &overview.standing {
            ChannelStanding::Disconnected { .. } => {
                return Ok(SpruceSyncOutcome::SkippedDisconnected);
            }
            ChannelStanding::Connected {
                place,
                start_boundary,
                spruce_cursor,
                backlog,
                health,
            } => (
                *place,
                *start_boundary,
                spruce_cursor.clone(),
                backlog.clone(),
                health.clone(),
            ),
        };

        if let ChannelHealth::RevokedOrBlocked { reason, .. } = &health {
            return Ok(SpruceSyncOutcome::ReconnectRequired {
                reason: reason.clone(),
            });
        }

        let decoded_cursor = match saved_cursor_str.as_deref() {
            Some(raw) => match spruce::changes::Cursor::decode(raw) {
                Ok(c) => Some(c),
                Err(_) => {
                    let reason =
                        "Stored Spruce synchronization cursor could not be decoded.".to_owned();
                    self.record_spruce_channel_failure(
                        pass.practice,
                        lease_token,
                        ChannelStopCause::Transient {
                            reason: reason.clone(),
                        },
                        pass.now_ts,
                    )
                    .await?;
                    return Ok(SpruceSyncOutcome::TransientFailure { reason });
                }
            },
            None => None,
        };

        let change_outcome = match reader.changes(decoded_cursor.as_ref()).await {
            Ok(outcome) => outcome,
            Err(err) => {
                return self
                    .record_spruce_api_failure(pass.practice, lease_token, &err, pass.now_ts)
                    .await;
            }
        };

        let page = match change_outcome {
            spruce::changes::Outcome::ReconnectRequired => {
                let reason =
                    "Spruce rejected channel credentials (authorization revoked or blocked)."
                        .to_owned();
                self.record_spruce_revocation(
                    pass.practice,
                    lease_token,
                    reason.clone(),
                    pass.now_ts,
                )
                .await?;
                return Ok(SpruceSyncOutcome::ReconnectRequired { reason });
            }
            spruce::changes::Outcome::Unavailable => {
                let reason = "Spruce channel API is temporarily unavailable.".to_owned();
                self.record_spruce_channel_failure(
                    pass.practice,
                    lease_token,
                    ChannelStopCause::Transient {
                        reason: reason.clone(),
                    },
                    pass.now_ts,
                )
                .await?;
                return Ok(SpruceSyncOutcome::TransientFailure { reason });
            }
            spruce::changes::Outcome::Page(page) => page,
        };

        if matches!(health, ChannelHealth::TransientFailure { .. }) {
            let _ = self
                .recover_spruce_channel_reading(pass.practice, lease_token, pass.now_ts)
                .await?;
        }

        let mut unread_conversations = page.unread.len();
        let mut entries: Vec<PollEntryKind> = page
            .changes
            .into_iter()
            .rev()
            .map(PollEntryKind::Change)
            .collect();

        let mut backlog_items_read = 0usize;
        let mut backlog_completed_cleanly = false;
        if matches!(backlog, BacklogStanding::Accepted { .. }) {
            let mut next_page_token: Option<String> = None;
            loop {
                match reader
                    .read_backlog_page(&page.cursor, next_page_token.as_deref())
                    .await
                {
                    Ok(Ok(bp)) => {
                        unread_conversations += bp.unread.len();
                        backlog_items_read += bp.items.len();
                        for obs in bp.items.into_iter().rev() {
                            entries.push(PollEntryKind::Backlog(obs));
                        }
                        next_page_token = bp.next_page;
                        if next_page_token.is_none() {
                            backlog_completed_cleanly = true;
                            break;
                        }
                    }
                    Ok(Err(spruce::changes::Outcome::ReconnectRequired)) => {
                        let reason =
                            "Spruce rejected channel credentials while reading backlog.".to_owned();
                        self.record_spruce_revocation(
                            pass.practice,
                            lease_token,
                            reason.clone(),
                            pass.now_ts,
                        )
                        .await?;
                        return Ok(SpruceSyncOutcome::ReconnectRequired { reason });
                    }
                    Ok(Err(_)) => {
                        let reason =
                            "Spruce channel API is temporarily unavailable while reading backlog."
                                .to_owned();
                        self.record_spruce_channel_failure(
                            pass.practice,
                            lease_token,
                            ChannelStopCause::Transient {
                                reason: reason.clone(),
                            },
                            pass.now_ts,
                        )
                        .await?;
                        return Ok(SpruceSyncOutcome::TransientFailure { reason });
                    }
                    Err(err) => {
                        return self
                            .record_spruce_api_failure(
                                pass.practice,
                                lease_token,
                                &err,
                                pass.now_ts,
                            )
                            .await;
                    }
                }
            }
        }

        // Load highest existing channel position + conversation cache cleanly.
        let mut next_position = self
            .highest_spruce_message_position(pass.practice)
            .await?
            .max(place)
            .max(start_boundary);
        let mut conv_cache: BTreeMap<String, Option<spruce::payload::Conversation>> =
            BTreeMap::new();
        let mut touched_conversations: BTreeSet<String> = BTreeSet::new();
        let mut attachments_to_sync: Vec<PendingAttachmentSync> = Vec::new();
        let mut ingested_count = 0usize;
        let mut unreadable_count = 0usize;
        let mut outbound_items: Vec<(String, Box<spruce::payload::Item>)> = Vec::new();

        for entry in entries {
            match entry.observed() {
                spruce::changes::Observed::Message(item) => {
                    let item = item.as_ref();
                    let cid = item
                        .envelope
                        .conversation_id
                        .clone()
                        .unwrap_or_else(|| format!("spruce-conv-{}", item.id()));
                    touched_conversations.insert(cid.clone());
                    let participants = self
                        .resolve_and_record_spruce_conversation_participants(
                            reader,
                            pass.practice,
                            &cid,
                            &mut conv_cache,
                            pass.now_ts,
                        )
                        .await?;

                    if item.deleted {
                        unreadable_count += 1;
                        continue;
                    }
                    if item.envelope.is_internal_note == Some(true) {
                        continue;
                    }
                    if item.envelope.direction.as_deref() == Some("outbound") {
                        outbound_items.push((cid, Box::new(item.clone())));
                        continue;
                    }

                    let pos = match self
                        .existing_spruce_message_position(pass.practice, item.id())
                        .await?
                    {
                        Some(existing) => existing,
                        None => {
                            next_position = next_position.saturating_add(1);
                            next_position
                        }
                    };
                    let body_text = compose_spruce_item_body(item);
                    let sender_address = first_participant_sender_address(&participants);
                    let attachments = item
                        .attachments
                        .iter()
                        .map(|att| SpruceAttachmentSummary {
                            kind: att.kind.clone(),
                            title: att.title.clone(),
                            name: att.name.clone(),
                            mimetype: att.mimetype.clone(),
                        })
                        .collect();

                    let incoming = IncomingSpruceItem::Readable(ReadableSpruceItem {
                        message_id: item.id().to_owned(),
                        conversation_id: cid.clone(),
                        channel_position: pos,
                        content_version: item.version(),
                        sender_address,
                        author_name: item.envelope.author_name.clone(),
                        body_text,
                        attachments,
                        app_url: item.envelope.app_url.clone(),
                        created_at_spruce: item.envelope.created_at.clone(),
                        outside_participants: participants,
                    });

                    let outcome = self
                        .ingest_spruce_item(pass.practice, lease_token, &incoming, pass.now_ts)
                        .await?;
                    if !matches!(outcome, SpruceIngestOutcome::SkippedPreConnectionBacklog) {
                        ingested_count += 1;
                        if !item.attachments.is_empty() {
                            attachments_to_sync.push(PendingAttachmentSync {
                                conversation_id: cid,
                                item: Box::new(item.clone()),
                            });
                        }
                    }
                }
                spruce::changes::Observed::Unreadable(env) => {
                    let cid = env
                        .conversation_id
                        .clone()
                        .unwrap_or_else(|| format!("spruce-conv-{}", env.id));
                    let pos = match self
                        .existing_spruce_message_position(pass.practice, &env.id)
                        .await?
                    {
                        Some(existing) => existing,
                        None => {
                            next_position = next_position.saturating_add(1);
                            next_position
                        }
                    };
                    let incoming = IncomingSpruceItem::Unreadable(UnreadableSpruceItem {
                        message_id: env.id.clone(),
                        conversation_id: cid,
                        channel_position: pos,
                        content_version: format!("unreadable-{}", env.id),
                        app_url: env.app_url.clone(),
                        failure_reason: "Spruce message body could not be decoded cleanly."
                            .to_owned(),
                        outside_participants: Vec::new(),
                    });
                    let outcome = self
                        .ingest_spruce_item(pass.practice, lease_token, &incoming, pass.now_ts)
                        .await?;
                    if !matches!(outcome, SpruceIngestOutcome::SkippedPreConnectionBacklog) {
                        unreadable_count += 1;
                    }
                }
            }
        }

        // Persist the updated channel cursor & monotonic place after durable message writes.
        let cursor_encoded = page.cursor.encode();
        let channel_position_after = self
            .advance_spruce_channel_place(
                pass.practice,
                lease_token,
                Some(cursor_encoded),
                pass.now_ts,
            )
            .await?;
        if backlog_completed_cleanly {
            self.complete_spruce_backlog(pass.practice, lease_token, pass.now_ts)
                .await?;
        }

        // Run background classification & automatic matching when a provider is supplied.
        let mut matched_run_count = 0usize;
        if let Some(provider) = matching_provider {
            let deciding_ids = self.deciding_spruce_message_ids(pass.practice).await?;
            for message_id in deciding_ids {
                match run_spruce_message_matching(
                    self,
                    auth,
                    pass.practice,
                    &message_id,
                    pass.roles,
                    pass.skills,
                    provider,
                    Some(pass.review_receiver),
                    engine::RunLimits::default(),
                    &engine::Cancellation::default(),
                )
                .await
                {
                    Ok(done) if done.run.status == MatchingRunStatus::Completed => {
                        matched_run_count += 1;
                    }
                    Ok(_) | Err(_) => {}
                }
            }
        }

        // Settle any 15-minute undecided fallback items.
        let fallback_report = self
            .settle_undecided_spruce_messages(
                auth,
                pass.practice,
                Some(pass.review_receiver),
                pass.now_ts,
            )
            .await?;
        let fallback_routed_count = fallback_report.succeeded.len();

        // Sync linked Spruce messages & attachments into patient reply/chart state.
        let mut synced_workspaces = BTreeSet::new();
        for cid in &touched_conversations {
            if let Some(conv) = self.spruce_conversation(pass.practice, cid).await?
                && let ConversationLinkState::Linked { workspace, .. } = conv.link_state
            {
                synced_workspaces.insert(workspace);
            }
        }
        for workspace in &synced_workspaces {
            let _ = self
                .sync_workspace_spruce_conversations_to_reply(pass.practice, workspace)
                .await;
        }

        // Record observed outbound practice messages on linked reply conversations.
        let mut outbound_observed_count = 0usize;
        for (cid, item) in outbound_items {
            let Some(conv) = self.spruce_conversation(pass.practice, &cid).await? else {
                continue;
            };
            let ConversationLinkState::Linked { workspace, .. } = conv.link_state else {
                continue;
            };
            let _ = self
                .sync_workspace_spruce_conversations_to_reply(pass.practice, &workspace)
                .await;
            let text = compose_spruce_item_body(&item);
            if text.trim().is_empty() {
                continue;
            }
            let sent_at = item
                .envelope
                .created_at
                .as_deref()
                .and_then(|ts| chrono::DateTime::parse_from_rfc3339(ts).ok())
                .map(|dt| dt.timestamp())
                .unwrap_or(pass.now_ts);
            let kind = if item
                .envelope
                .author_name
                .as_deref()
                .is_some_and(|n| n.to_ascii_lowercase().contains("auto"))
            {
                PracticeMessageKind::Automatic
            } else {
                PracticeMessageKind::PracticeSent
            };
            if self
                .observe_practice_message(&ObservePracticeMessage {
                    practice: pass.practice.to_owned(),
                    conversation_id: cid,
                    practice_message_id: item.id().to_owned(),
                    kind,
                    text,
                    sent_at,
                })
                .await
                .is_ok()
            {
                outbound_observed_count += 1;
            }
        }

        // Download & ingest attachments transiently for linked conversations.
        let (attachments_ingested_count, attachments_failed_count) = self
            .ingest_linked_spruce_attachments(
                reader,
                pass.practice,
                &attachments_to_sync,
                pass.now_ts,
            )
            .await?;

        // Dispatch pending patient review triggers to ReviewReceiver.
        let dispatch_report = self
            .dispatch_pending_spruce_reviews(pass.practice, pass.review_receiver, pass.now_ts)
            .await?;
        let dispatched_review_count = dispatch_report.succeeded.len();

        let (channel_standing, _) = self.spruce_channel_standing(pass.practice).await?;
        Ok(SpruceSyncOutcome::Completed(SpruceSyncPassReport {
            channel_position_after,
            ingested_count,
            unreadable_count,
            outbound_observed_count,
            attachments_ingested_count,
            attachments_failed_count,
            backlog_items_read,
            unread_conversations,
            matched_run_count,
            fallback_routed_count,
            dispatched_review_count,
            channel_standing,
        }))
    }

    async fn record_spruce_revocation(
        &self,
        practice: &str,
        lease_token: &str,
        reason: String,
        now_ts: i64,
    ) -> Result<(), SpruceMatchingError> {
        self.record_spruce_channel_failure(
            practice,
            lease_token,
            ChannelStopCause::RevokedOrBlocked {
                reason: reason.clone(),
            },
            now_ts,
        )
        .await?;
        let _ = self
            .set_practice_reply_channels_connection(
                practice,
                ChannelConnection::Disconnected {
                    disconnected_at: now_ts,
                    note: format!("{reason} Open in Spruce to reply manually, or mark Done."),
                },
            )
            .await;
        Ok(())
    }

    async fn record_spruce_api_failure(
        &self,
        practice: &str,
        lease_token: &str,
        err: &spruce::api::Error,
        now_ts: i64,
    ) -> Result<SpruceSyncOutcome, SpruceMatchingError> {
        match err {
            spruce::api::Error::AuthorizationRejected
            | spruce::api::Error::Rejected { status: 403 } => {
                let reason = format!("Spruce authorization refused: {err}");
                self.record_spruce_revocation(practice, lease_token, reason.clone(), now_ts)
                    .await?;
                Ok(SpruceSyncOutcome::ReconnectRequired { reason })
            }
            _ => {
                let reason = format!("Spruce API request failed: {err}");
                self.record_spruce_channel_failure(
                    practice,
                    lease_token,
                    ChannelStopCause::Transient {
                        reason: reason.clone(),
                    },
                    now_ts,
                )
                .await?;
                Ok(SpruceSyncOutcome::TransientFailure { reason })
            }
        }
    }

    async fn resolve_and_record_spruce_conversation_participants(
        &self,
        reader: &spruce::changes::Reader,
        practice: &str,
        conversation_id: &str,
        cache: &mut BTreeMap<String, Option<spruce::payload::Conversation>>,
        now_ts: i64,
    ) -> Result<Vec<OutsideParticipant>, SpruceMatchingError> {
        let fetched = if let Some(entry) = cache.get(conversation_id) {
            entry.clone()
        } else {
            let conv = reader.client().get_conversation(conversation_id).await.ok();
            cache.insert(conversation_id.to_owned(), conv.clone());
            conv
        };
        let Some(conv) = fetched else {
            return Ok(Vec::new());
        };
        let mut participants = Vec::with_capacity(conv.external_participants.len());
        for ext in &conv.external_participants {
            let endpoint_channel = ext.endpoint.as_ref().and_then(|e| e.channel.clone());
            let endpoint_address = ext
                .endpoint
                .as_ref()
                .and_then(|e| parse_endpoint_address(e.channel.as_deref(), e.raw_value.as_deref()));
            if let Some(contact_id) = ext.contact.as_deref().filter(|s| !s.trim().is_empty()) {
                let existing = self
                    .spruce_contact_patient_links(practice, contact_id)
                    .await?;
                if existing.is_empty() {
                    let _ = self
                        .save_spruce_contact_record(
                            &SpruceContactRecord {
                                practice: practice.to_owned(),
                                contact_id: contact_id.to_owned(),
                                spruce_patients: vec![SpruceContactPatient {
                                    spruce_patient_id: contact_id.to_owned(),
                                    ehr_links: Vec::new(),
                                }],
                            },
                            now_ts,
                        )
                        .await;
                }
            }
            participants.push(OutsideParticipant {
                display_name: ext.display_name.clone(),
                endpoint_channel,
                endpoint_address,
                saved_contact_id: ext.contact.clone(),
            });
        }
        Ok(participants)
    }

    async fn spruce_contact_patient_links(
        &self,
        practice: &str,
        contact_id: &str,
    ) -> Result<Vec<SpruceContactPatient>, SpruceMatchingError> {
        let row = spruce_entities::spruce_contact::Entity::find_by_id((
            practice.to_owned(),
            contact_id.to_owned(),
        ))
        .one(&self.db)
        .await?;
        let Some(row) = row else {
            return Ok(Vec::new());
        };
        serde_json::from_value(row.spruce_patients).map_err(|err| {
            SpruceMatchingError::Database(super::conversion_error(
                "spruce_contact.spruce_patients",
                "Vec<SpruceContactPatient>",
            )(err))
        })
    }

    async fn highest_spruce_message_position(
        &self,
        practice: &str,
    ) -> Result<u64, SpruceMatchingError> {
        let latest = spruce_entities::spruce_message::Entity::find()
            .filter(spruce_entities::spruce_message::Column::Practice.eq(practice))
            .order_by_desc(spruce_entities::spruce_message::Column::ChannelPosition)
            .one(&self.db)
            .await?;
        Ok(latest
            .and_then(|r| u64::try_from(r.channel_position).ok())
            .unwrap_or(0))
    }

    async fn existing_spruce_message_position(
        &self,
        practice: &str,
        message_id: &str,
    ) -> Result<Option<u64>, SpruceMatchingError> {
        let row = spruce_entities::spruce_message::Entity::find_by_id((
            practice.to_owned(),
            message_id.to_owned(),
        ))
        .one(&self.db)
        .await?;
        Ok(row.and_then(|r| u64::try_from(r.channel_position).ok()))
    }

    async fn deciding_spruce_message_ids(
        &self,
        practice: &str,
    ) -> Result<Vec<String>, SpruceMatchingError> {
        let rows = spruce_entities::spruce_message::Entity::find()
            .filter(spruce_entities::spruce_message::Column::Practice.eq(practice))
            .filter(spruce_entities::spruce_message::Column::StandingKind.eq("deciding"))
            .order_by_asc(spruce_entities::spruce_message::Column::ChannelPosition)
            .all(&self.db)
            .await?;
        Ok(rows.into_iter().map(|r| r.message_id).collect())
    }

    async fn ingest_linked_spruce_attachments(
        &self,
        reader: &spruce::changes::Reader,
        practice: &str,
        items: &[PendingAttachmentSync],
        now_ts: i64,
    ) -> Result<(usize, usize), SpruceMatchingError> {
        let mut ingested = 0usize;
        let mut failed = 0usize;
        for pending in items {
            let Some(conv) = self
                .spruce_conversation(practice, &pending.conversation_id)
                .await?
            else {
                continue;
            };
            let ConversationLinkState::Linked { workspace, .. } = conv.link_state else {
                continue;
            };

            let existing_sources = self.sources(&workspace).await?;
            let existing_candidates: BTreeSet<String> = existing_sources
                .sources
                .iter()
                .map(|s| s.candidate_id.clone())
                .collect();

            let msg_id = pending.item.id();
            let all_already_ingested = (0..pending.item.attachments.len())
                .all(|idx| existing_candidates.contains(&format!("spruce-att-{msg_id}-{idx}")));
            if all_already_ingested {
                continue;
            }

            let outcomes = match reader.fetch_item_attachments(pending.item.as_ref()).await {
                Ok(outcomes) => outcomes,
                Err(_) => {
                    failed += pending.item.attachments.len();
                    continue;
                }
            };

            let authored_date = pending
                .item
                .envelope
                .created_at
                .as_deref()
                .and_then(|s| s.get(..10))
                .filter(|s| s.len() == 10)
                .unwrap_or("1970-01-01")
                .to_owned();

            let mut revisions: Vec<SourceRevision> = Vec::new();
            let mut batch_inputs: Vec<SavedAttachmentInput> = Vec::new();

            for (idx, outcome) in outcomes.into_iter().enumerate() {
                let candidate_id = format!("spruce-att-{msg_id}-{idx}");
                if existing_candidates.contains(&candidate_id) {
                    continue;
                }
                match outcome {
                    spruce::changes::AttachmentBytesOutcome::Fetched { attachment, bytes } => {
                        let content_type = attachment
                            .mimetype
                            .as_deref()
                            .unwrap_or("application/octet-stream");
                        let filename = attachment.name.as_deref();
                        let title = attachment
                            .title
                            .clone()
                            .or_else(|| attachment.name.clone())
                            .unwrap_or_else(|| format!("Spruce attachment {}", idx + 1));
                        match AttachmentMediaKind::parse(content_type, filename) {
                            Some(
                                kind @ (AttachmentMediaKind::PdfDocument
                                | AttachmentMediaKind::SpruceFaxPdf
                                | AttachmentMediaKind::ChartScanPdf),
                            ) => {
                                let effective_kind = if attachment.kind.eq_ignore_ascii_case("fax")
                                {
                                    AttachmentMediaKind::SpruceFaxPdf
                                } else {
                                    kind
                                };
                                match ingest_paged_document(&PagedDocumentInput {
                                    document_id: candidate_id.clone(),
                                    version: 1,
                                    workspace: workspace.clone(),
                                    source_id: "spruce".to_owned(),
                                    authored_date: authored_date.clone(),
                                    title: title.clone(),
                                    media_kind: effective_kind,
                                    pdf_bytes: bytes,
                                    expected_sha256: None,
                                }) {
                                    Ok(doc) => {
                                        let mut rev = doc.source_revision;
                                        enrich_spruce_attachment_provenance(
                                            &mut rev.provenance,
                                            &pending.conversation_id,
                                            msg_id,
                                        );
                                        revisions.push(rev);
                                        ingested += 1;
                                    }
                                    Err(err) => {
                                        failed += 1;
                                        revisions.push(unreadable_spruce_attachment_revision(
                                            &workspace,
                                            &candidate_id,
                                            &authored_date,
                                            &title,
                                            &pending.conversation_id,
                                            msg_id,
                                            &err.to_string(),
                                        ));
                                    }
                                }
                            }
                            Some(kind) => {
                                batch_inputs.push(SavedAttachmentInput {
                                    attachment_id: candidate_id,
                                    revision: "v1".to_owned(),
                                    workspace: workspace.clone(),
                                    source_id: "spruce".to_owned(),
                                    authored_date: authored_date.clone(),
                                    title,
                                    media_kind: kind,
                                    bytes,
                                    expected_sha256: None,
                                });
                            }
                            None => {
                                failed += 1;
                                revisions.push(unreadable_spruce_attachment_revision(
                                    &workspace,
                                    &candidate_id,
                                    &authored_date,
                                    &title,
                                    &pending.conversation_id,
                                    msg_id,
                                    &format!("unsupported attachment media type `{content_type}`"),
                                ));
                            }
                        }
                    }
                    spruce::changes::AttachmentBytesOutcome::NoDownloadUrl { attachment } => {
                        let title = attachment
                            .title
                            .or(attachment.name)
                            .unwrap_or_else(|| format!("Spruce attachment {}", idx + 1));
                        failed += 1;
                        revisions.push(unreadable_spruce_attachment_revision(
                            &workspace,
                            &candidate_id,
                            &authored_date,
                            &title,
                            &pending.conversation_id,
                            msg_id,
                            "attachment had no download URL in Spruce response",
                        ));
                    }
                    spruce::changes::AttachmentBytesOutcome::Unavailable { attachment, reason } => {
                        let title = attachment
                            .title
                            .or(attachment.name)
                            .unwrap_or_else(|| format!("Spruce attachment {}", idx + 1));
                        failed += 1;
                        revisions.push(unreadable_spruce_attachment_revision(
                            &workspace,
                            &candidate_id,
                            &authored_date,
                            &title,
                            &pending.conversation_id,
                            msg_id,
                            &reason,
                        ));
                    }
                }
            }

            if !batch_inputs.is_empty() {
                let batch = ingest_attachment_batch(&batch_inputs);
                for rec in batch.records {
                    if rec.outcome.is_ok() {
                        ingested += 1;
                    } else {
                        failed += 1;
                    }
                    let mut rev = rec.source_revision;
                    enrich_spruce_attachment_provenance(
                        &mut rev.provenance,
                        &pending.conversation_id,
                        msg_id,
                    );
                    revisions.push(rev);
                }
            }

            if !revisions.is_empty() {
                let tx = self.db.begin().await?;
                let existing_pull = source_pull::Entity::find()
                    .filter(source_pull::Column::Workspace.eq(&workspace))
                    .one(&tx)
                    .await?;
                let patient = existing_pull
                    .and_then(|row| serde_json::from_value::<SourcePatient>(row.patient).ok())
                    .unwrap_or_else(|| SourcePatient {
                        source_patient_id: workspace.clone(),
                        display_name: "Spruce Patient".to_owned(),
                        birth_date: None,
                        sex_or_gender: None,
                        medical_record_numbers: Vec::new(),
                    });
                let pull = SourcePull {
                    source_id: "spruce".to_owned(),
                    source_label: "Spruce Health".to_owned(),
                    patient,
                    pulled_at: Some(now_ts),
                };
                let _ = record_delivery_in_tx(
                    &tx,
                    &workspace,
                    DeliveryScope::ChangedRecords,
                    &revisions,
                    &pull,
                )
                .await?;
                tx.commit().await?;
            }
        }
        Ok((ingested, failed))
    }
}

fn compose_spruce_item_body(item: &spruce::payload::Item) -> String {
    let mut parts: Vec<String> = Vec::new();
    let trimmed = item.text.trim();
    if !trimmed.is_empty() {
        parts.push(trimmed.to_owned());
    }
    if let Some(tr) = &item.transcription {
        match tr {
            spruce::payload::TranscriptionState::Ready { text, summary, .. } => {
                if !text.trim().is_empty() {
                    parts.push(format!("[Voicemail transcript] {}", text.trim()));
                } else if let Some(s) = summary.as_deref().filter(|s| !s.trim().is_empty()) {
                    parts.push(format!("[Voicemail summary] {}", s.trim()));
                }
            }
            spruce::payload::TranscriptionState::Unfinished { status, text, .. } => {
                if !text.trim().is_empty() {
                    parts.push(format!("[Voicemail transcript ({status})] {}", text.trim()));
                }
            }
            spruce::payload::TranscriptionState::NotFound { .. }
            | spruce::payload::TranscriptionState::Unavailable { .. } => {}
        }
    }
    parts.join("\n")
}

fn parse_endpoint_address(channel: Option<&str>, raw_value: Option<&str>) -> Option<SenderAddress> {
    let raw = raw_value?.trim();
    if raw.is_empty() {
        return None;
    }
    let formatted = if raw.contains('@')
        || raw.starts_with("phone:")
        || raw.starts_with("email:")
        || raw.starts_with("fax:")
        || raw.starts_with("spruce:")
    {
        raw.to_owned()
    } else {
        match channel {
            Some("email") => format!("email:{raw}"),
            Some("fax") => format!("fax:{raw}"),
            _ => format!("phone:{raw}"),
        }
    };
    SenderAddress::parse(&formatted).ok()
}

fn first_participant_sender_address(participants: &[OutsideParticipant]) -> Option<SenderAddress> {
    participants.iter().find_map(|p| p.endpoint_address.clone())
}

fn enrich_spruce_attachment_provenance(
    provenance: &mut Value,
    conversation_id: &str,
    message_id: &str,
) {
    if let Some(obj) = provenance.as_object_mut() {
        obj.insert(
            "conversation_id".to_owned(),
            Value::String(conversation_id.to_owned()),
        );
        obj.insert(
            "message_id".to_owned(),
            Value::String(message_id.to_owned()),
        );
    }
}

fn unreadable_spruce_attachment_revision(
    workspace: &str,
    candidate_id: &str,
    authored_date: &str,
    title: &str,
    conversation_id: &str,
    message_id: &str,
    reason: &str,
) -> SourceRevision {
    let content = format!("Unreadable Spruce attachment `{title}`: {reason}");
    let sha256 = crate::sha256_hex(content.as_bytes());
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "spruce".to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: "v1".to_owned(),
        record_type: "DocumentReference".to_owned(),
        dates: vec![RecordDate {
            name: "authored".to_owned(),
            value: authored_date.to_owned(),
        }],
        content,
        sha256,
        readable_parts: Vec::new(),
        provenance: serde_json::json!({
            "document_id": candidate_id,
            "attachment_id": candidate_id,
            "conversation_id": conversation_id,
            "message_id": message_id,
            "unreadable": true,
            "reason": reason,
        }),
        envelope: Some(RecordEnvelope {
            bucket: Bucket::Documents,
            source_kind: SourceRecordKind {
                kind: "DocumentReference".to_owned(),
                category: Some("spruce-attachment".to_owned()),
            },
            native_id: candidate_id.to_owned(),
            version: RecordVersion::Source("v1".to_owned()),
            dates: Vec::new(),
            status: Some("unreadable".to_owned()),
            codes: Vec::new(),
            title: Some(title.to_owned()),
            native_link: None,
            unavailable_parts: Vec::new(),
        }),
    }
}
