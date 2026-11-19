//! Spruce channel reading, patient conversation linking, classification storage,
//! confirmation, wrong-patient moves, and authorized message projections.
//!
//! Each Spruce conversation links to at most one Coda patient at a time. A
//! conversation that is not linked yet links automatically when, and only when,
//! its sender is confirmed for one patient or the channel ties its single
//! outside participant through one saved contact and one EHR record to one Coda
//! patient, and nothing points to a different patient. Ambiguous or urgent
//! unmatched messages wait for physician confirmation without reading any
//! patient's clinical chart and never enter patient reviews until linked.

use std::collections::{BTreeMap, BTreeSet};

use patient_work::{ReviewReason, ReviewRequest};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, StoreUnavailable};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use super::dispatch::ReviewReceiver;
use super::entities::{schedule_patient_link, source_pull};
use super::jobs::{PracticeAccessError, PracticeAuthority, PracticeJobError, PracticeScope};
use super::receipt::{self, Receipt};
use super::sources::Observation;
use super::{Store, conversion_error, error};
use crate::physician_sources::SourcePatient;

/// Maximum seconds a kept message may stay in [`MessageStanding::Deciding`]
/// before routing automatically so no patient message stays invisible (`15` minutes).
pub const FIFTEEN_MINUTES_SECONDS: i64 = 15 * 60;

/// Default seconds `Undo` remains available after moving a linked conversation.
pub const DEFAULT_MOVE_UNDO_SECONDS: i64 = 10;

/// Private SeaORM entities owned exclusively by [`super::spruce_matching`],
/// [`crate::spruce_matching_recipe`], and `m20261118_10_spruce_matching`.
pub(crate) mod entities {
    /// Practice-level Spruce channel standing, cursor boundaries, and single-reader lease.
    pub mod spruce_channel {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_spruce_channels")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(column_type = "Text")]
            pub standing_kind: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub standing: Json,
            #[sea_orm(column_type = "JsonBinary", nullable)]
            pub reader_lease: Option<Json>,
            pub updated_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Saved Spruce contact records mapping one contact ID to Spruce patient and EHR links.
    pub mod spruce_contact {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_spruce_contacts")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub contact_id: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub spruce_patients: Json,
            pub updated_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Practice EHR patient record links (`(source_id, source_patient_id) -> workspace`).
    /// Composite key includes `workspace` so conflicting multi-patient EHR mappings
    /// can be represented and detected rather than silently overwritten.
    pub mod spruce_ehr_link {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_spruce_ehr_links")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub source_id: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub source_patient_id: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub workspace: String,
            pub recorded_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Confirmed sender endpoints mapped to one or more Coda patient workspaces.
    pub mod spruce_confirmed_sender {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_spruce_confirmed_senders")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub sender_key: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub workspace: String,
            #[sea_orm(column_type = "Text")]
            pub confirmed_by: String,
            pub confirmed_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Spruce conversation link state, outside participants, active suggestion,
    /// open physician views, and wrong-patient move / Undo tracking.
    pub mod spruce_conversation {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_spruce_conversations")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub conversation_id: String,
            #[sea_orm(column_type = "Text")]
            pub link_kind: String,
            #[sea_orm(column_type = "Text", nullable)]
            pub linked_workspace: Option<String>,
            #[sea_orm(column_type = "JsonBinary")]
            pub link_state: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub outside_participants: Json,
            #[sea_orm(column_type = "JsonBinary", nullable)]
            pub active_suggestion: Option<Json>,
            #[sea_orm(column_type = "JsonBinary")]
            pub open_viewers: Json,
            #[sea_orm(column_type = "JsonBinary", nullable)]
            pub move_undo: Option<Json>,
            pub move_count: i64,
            pub last_move_acted: bool,
            pub updated_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Kept Spruce message versions, routing standing, urgency, classification,
    /// and monotonic `ever_linked` record-retention latch.
    pub mod spruce_message {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_spruce_messages")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub message_id: String,
            #[sea_orm(column_type = "Text")]
            pub conversation_id: String,
            pub channel_position: i64,
            #[sea_orm(column_type = "Text")]
            pub content_version: String,
            #[sea_orm(column_type = "Text")]
            pub standing_kind: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub standing: Json,
            #[sea_orm(column_type = "Text")]
            pub urgency_kind: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub urgency: Json,
            pub ever_linked: bool,
            #[sea_orm(column_type = "Text", nullable)]
            pub sender_address: Option<String>,
            #[sea_orm(column_type = "Text", nullable)]
            pub author_name: Option<String>,
            #[sea_orm(column_type = "Text")]
            pub body_text: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub attachments: Json,
            #[sea_orm(column_type = "Text", nullable)]
            pub app_url: Option<String>,
            #[sea_orm(column_type = "Text", nullable)]
            pub created_at_spruce: Option<String>,
            pub kept_at: i64,
            #[sea_orm(column_type = "JsonBinary", nullable)]
            pub saved_classification: Option<Json>,
            pub updated_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Permanent entered-in-error audit records written when a wrong-patient
    /// conversation move settles after `Undo` expires. Never deleted.
    pub mod spruce_entered_in_error {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_spruce_entered_in_error")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub from_workspace: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub message_id: String,
            #[sea_orm(primary_key, auto_increment = false)]
            pub move_ordinal: i64,
            #[sea_orm(column_type = "Text")]
            pub conversation_id: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub destination: Json,
            pub marked_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Durable once-only review requests emitted when Spruce messages link to a
    /// patient or when a settled move affects old/new patients.
    pub mod spruce_review_trigger {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_spruce_review_triggers")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub command_key: String,
            #[sea_orm(column_type = "Text")]
            pub conversation_id: String,
            #[sea_orm(column_type = "Text")]
            pub workspace: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub origin: Json,
            pub workspace_version: i64,
            #[sea_orm(column_type = "JsonBinary")]
            pub record_versions: Json,
            pub dispatched: bool,
            pub created_at: i64,
            pub dispatched_at: Option<i64>,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Harness run records for unmatched Spruce message classification and
    /// patient matching (`crate::spruce_matching_recipe`).
    pub mod spruce_matching_run {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_spruce_matching_runs")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub run_id: String,
            #[sea_orm(column_type = "Text")]
            pub message_id: String,
            #[sea_orm(column_type = "Text")]
            pub conversation_id: String,
            #[sea_orm(column_type = "Text")]
            pub content_version: String,
            #[sea_orm(column_type = "Text")]
            pub attempt_id: String,
            #[sea_orm(column_type = "Text")]
            pub status: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub provenance: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub starting_context: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub initial_loads: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub tool_discoveries: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub events: Json,
            #[sea_orm(column_type = "JsonBinary", nullable)]
            pub staged_decision: Option<Json>,
            #[sea_orm(column_type = "Text", nullable)]
            pub final_summary: Option<String>,
            pub started_at: i64,
            pub completed_at: Option<i64>,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }
}

/// Canonicalized non-empty sender address (`phone:+1...`, `email:...`, or `spruce:...`).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SenderAddress(String);

impl SenderAddress {
    /// Parses and normalizes a non-empty sender endpoint address.
    pub fn parse(raw: &str) -> Result<Self, SpruceMatchingError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
            return Err(SpruceMatchingError::InvalidIdentifier {
                field: "sender_address",
            });
        }
        let normalized = if trimmed.contains('@') || trimmed.starts_with("email:") {
            trimmed.to_ascii_lowercase()
        } else {
            let candidate = trimmed
                .strip_prefix("phone:")
                .or_else(|| trimmed.strip_prefix("sms:"))
                .or_else(|| trimmed.strip_prefix("fax:"))
                .map(str::trim)
                .unwrap_or(trimmed);
            let is_phone_shape = !candidate.is_empty()
                && candidate
                    .chars()
                    .all(|c| c.is_ascii_digit() || matches!(c, '+' | '-' | '(' | ')' | '.' | ' '));
            if is_phone_shape {
                let digits: String = candidate.chars().filter(char::is_ascii_digit).collect();
                if candidate.starts_with('+') && digits.len() >= 7 {
                    format!("+{digits}")
                } else if digits.len() == 10 {
                    format!("+1{digits}")
                } else if digits.len() == 11 && digits.starts_with('1') {
                    format!("+{digits}")
                } else {
                    trimmed.to_owned()
                }
            } else {
                trimmed.to_owned()
            }
        };
        Ok(Self(normalized))
    }

    /// Canonical string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Single active channel reader lease held on one practice's Spruce channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelReaderLease {
    /// Logical worker or reader holder identifier.
    pub holder_id: String,
    /// Unique secret lease token required to mutate cursor or ingest items.
    pub lease_token: String,
    /// Unix second when acquired.
    pub acquired_at: i64,
    /// Unix second when the lease expires unless renewed.
    pub expires_at: i64,
}

impl ChannelReaderLease {
    /// Whether this lease is still active at `now`.
    #[must_use]
    pub const fn is_active_at(&self, now: i64) -> bool {
        self.expires_at > now
    }
}

/// One-time backlog review offer state after connecting a practice channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum BacklogStanding {
    /// No pre-connection messages existed when the channel connected (`start_boundary == 0`).
    NoOffer,
    /// Pre-connection messages existed; offered once until a physician accepts.
    Offered {
        /// Unix second when offered at connection time.
        offered_at: i64,
    },
    /// A physician accepted the one-time offer to read pre-connection messages.
    Accepted {
        /// Unix second when accepted.
        accepted_at: i64,
        /// Physician who accepted the backlog offer.
        accepted_by: String,
    },
}

/// Why reading a connected Spruce channel stopped (`ChannelStops(r)` in `MessageMatching.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "cause", rename_all = "snake_case")]
pub enum ChannelStopCause {
    /// Temporary transport or upstream outage; Today says Coda cannot read the channel without `Reconnect`.
    Transient {
        /// Plain-language failure reason.
        reason: String,
    },
    /// Credential grant was revoked or blocked by an administrator; Today asks to `Reconnect`.
    RevokedOrBlocked {
        /// Plain-language revocation or block detail.
        reason: String,
    },
}

/// Operational health of a connected Spruce channel (`Rule 10`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ChannelHealth {
    /// Coda can read the channel normally.
    Reading {
        /// Unix second when Coda last read the channel cleanly, when any read occurred.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        last_read_at: Option<i64>,
    },
    /// Transient transport or upstream failure; Coda retries automatically without asking to `Reconnect`.
    TransientFailure {
        /// Plain-language failure summary.
        reason: String,
        /// Unix second when the failure began.
        failed_at: i64,
        /// Unix second when Coda last read the channel cleanly.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        last_read_at: Option<i64>,
    },
    /// Credential grant was revoked or blocked by an administrator; requires `Reconnect`.
    RevokedOrBlocked {
        /// Plain-language revocation or block detail.
        reason: String,
        /// Unix second when detected.
        failed_at: i64,
        /// Unix second when Coda last read the channel cleanly.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        last_read_at: Option<i64>,
    },
}

/// Practice Spruce channel lifecycle state (`Rules 10, 11, 12, 14`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ChannelStanding {
    /// Not connected (or disconnected by the practice). Today stays quiet.
    Disconnected {
        /// Unix second when most recently disconnected, if ever connected before.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        disconnected_at: Option<i64>,
    },
    /// Connected to the practice's Spruce organization.
    Connected {
        /// Monotonic channel position past which Coda has durably kept every message (`place`).
        place: u64,
        /// Channel position when Coda connected (`start`); items `<= start_boundary` wait for backlog acceptance.
        start_boundary: u64,
        /// Opaque encoded Spruce synchronization cursor, when stored.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        spruce_cursor: Option<String>,
        /// One-time backlog offer standing.
        backlog: BacklogStanding,
        /// Current channel read health.
        health: ChannelHealth,
    },
}

/// What Today says about the practice's Spruce channel (`Rule 10`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "banner", rename_all = "snake_case")]
pub enum TodayChannelBanner {
    /// Channel is reading normally or disconnected; Today shows no channel error.
    Quiet,
    /// Channel cannot be read temporarily; Today names why and when Coda last read it, with **no** `Reconnect`.
    CannotRead {
        /// Plain-language failure reason.
        reason: String,
        /// Unix second when the failure was recorded.
        failed_at: i64,
        /// Unix second when Coda last read the channel cleanly.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        last_read_at: Option<i64>,
    },
    /// Credential grant was revoked or blocked; Today says Coda cannot read the channel and offers `Reconnect`.
    ReconnectRequired {
        /// Plain-language revocation or block detail.
        reason: String,
        /// Unix second when the revocation or block was recorded.
        failed_at: i64,
        /// Unix second when Coda last read the channel cleanly.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        last_read_at: Option<i64>,
    },
}

impl ChannelStanding {
    pub(super) const fn kind_str(&self) -> &'static str {
        match self {
            Self::Disconnected { .. } => "disconnected",
            Self::Connected { .. } => "connected",
        }
    }

    /// Single rule owner deriving Today's channel notice from [`ChannelStanding`] (`Rule 10`).
    #[must_use]
    pub fn today_banner(&self) -> TodayChannelBanner {
        match self {
            Self::Disconnected { .. }
            | Self::Connected {
                health: ChannelHealth::Reading { .. },
                ..
            } => TodayChannelBanner::Quiet,
            Self::Connected {
                health:
                    ChannelHealth::TransientFailure {
                        reason,
                        failed_at,
                        last_read_at,
                    },
                ..
            } => TodayChannelBanner::CannotRead {
                reason: reason.clone(),
                failed_at: *failed_at,
                last_read_at: *last_read_at,
            },
            Self::Connected {
                health:
                    ChannelHealth::RevokedOrBlocked {
                        reason,
                        failed_at,
                        last_read_at,
                    },
                ..
            } => TodayChannelBanner::ReconnectRequired {
                reason: reason.clone(),
                failed_at: *failed_at,
                last_read_at: *last_read_at,
            },
        }
    }
}

/// Outside participant metadata on a Spruce conversation (`Rule 4`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutsideParticipant {
    /// Display name sent by Spruce, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Endpoint channel (`phone`, `sms`, `email`, `fax`, `secure`), when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_channel: Option<String>,
    /// Normalized address used by this outside participant, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_address: Option<SenderAddress>,
    /// Spruce saved-contact identifier (`None` when the participant is not a saved contact).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_contact_id: Option<String>,
}

/// One EHR patient record reference attached to a Spruce patient record.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpruceEhrLink {
    /// EHR source system identifier in Coda.
    pub source_id: String,
    /// EHR patient identifier within `source_id`.
    pub source_patient_id: String,
}

/// One Spruce patient record linked from a saved Spruce contact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpruceContactPatient {
    /// Spruce patient record identifier.
    pub spruce_patient_id: String,
    /// EHR patient records linked to this Spruce patient.
    #[serde(default)]
    pub ehr_links: Vec<SpruceEhrLink>,
}

/// Saved Spruce contact entry mapping `contact_id` to zero, one, or many Spruce patients.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpruceContactRecord {
    /// Practice owning the Spruce contact.
    pub practice: String,
    /// Spruce saved contact identifier.
    pub contact_id: String,
    /// Spruce patients naming this contact.
    pub spruce_patients: Vec<SpruceContactPatient>,
}

/// Why a conversation's Spruce channel metadata failed the `1 -> 1 -> 1 -> 1`
/// automatic-tie check with conflicting or ambiguous evidence (`Rule 4`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelTieConflict {
    /// Conversation has multiple outside participants and at least one ties to a patient.
    MultipleOutsideParticipants,
    /// The outside participant's saved contact belongs to more than one Spruce patient.
    ContactSharedAcrossSprucePatients,
    /// The Spruce patient record links to more than one EHR record.
    MultipleEhrRecordsOnSprucePatient,
    /// The EHR record maps to more than one Coda patient workspace in this practice.
    MultipleCodaPatientsForEhrRecord,
}

/// Outcome of evaluating whether the Spruce channel ties a conversation to one patient (`Rule 4`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "tie", rename_all = "snake_case")]
pub enum ChannelTieEvaluation {
    /// `1` outside participant -> `1` saved contact -> `1` Spruce patient -> `1` EHR record -> `1` Coda patient.
    UniquePatient {
        /// Unique matched Coda patient workspace.
        workspace: String,
    },
    /// Channel participants/contacts/EHR records point ambiguously or to multiple patients.
    Contradictory {
        /// Candidate Coda workspaces reachable from the channel participants.
        workspaces: BTreeSet<String>,
        /// Structural reason the `1 -> 1 -> 1 -> 1` rule was violated.
        reason: ChannelTieConflict,
    },
    /// Channel metadata establishes no patient tie (for example unsaved contact or unlinked EHR record).
    Untied,
}

/// Single rule owner evaluating the Spruce channel tie (`README.md` Rule 4):
/// *"in Spruce, its one outside participant is saved as the contact of 1 patient,
/// linked to 1 EHR record that matches 1 Coda patient."*
#[must_use]
pub fn evaluate_channel_tie(
    outside_participants: &[OutsideParticipant],
    contacts_by_id: &BTreeMap<String, Vec<SpruceContactPatient>>,
    ehr_to_workspaces: &BTreeMap<SpruceEhrLink, BTreeSet<String>>,
) -> ChannelTieEvaluation {
    let mut all_workspaces = BTreeSet::new();
    for participant in outside_participants {
        if let Some(contact_id) = &participant.saved_contact_id
            && let Some(patients) = contacts_by_id.get(contact_id)
        {
            for patient in patients {
                for ehr in &patient.ehr_links {
                    if let Some(ws_set) = ehr_to_workspaces.get(ehr) {
                        all_workspaces.extend(ws_set.iter().cloned());
                    }
                }
            }
        }
    }

    if outside_participants.len() > 1 {
        return ChannelTieEvaluation::Contradictory {
            workspaces: all_workspaces,
            reason: ChannelTieConflict::MultipleOutsideParticipants,
        };
    }
    if outside_participants.is_empty() {
        return ChannelTieEvaluation::Untied;
    }

    let Some(contact_id) = outside_participants[0].saved_contact_id.as_ref() else {
        return ChannelTieEvaluation::Untied;
    };
    let Some(spruce_patients) = contacts_by_id.get(contact_id) else {
        return ChannelTieEvaluation::Untied;
    };
    if spruce_patients.is_empty() {
        return ChannelTieEvaluation::Untied;
    }
    if spruce_patients.len() > 1 {
        return ChannelTieEvaluation::Contradictory {
            workspaces: all_workspaces,
            reason: ChannelTieConflict::ContactSharedAcrossSprucePatients,
        };
    }

    let ehr_links = &spruce_patients[0].ehr_links;
    if ehr_links.is_empty() {
        return ChannelTieEvaluation::Untied;
    }
    if ehr_links.len() > 1 {
        return ChannelTieEvaluation::Contradictory {
            workspaces: all_workspaces,
            reason: ChannelTieConflict::MultipleEhrRecordsOnSprucePatient,
        };
    }

    let Some(matched_workspaces) = ehr_to_workspaces.get(&ehr_links[0]) else {
        return ChannelTieEvaluation::Untied;
    };
    if matched_workspaces.is_empty() {
        return ChannelTieEvaluation::Untied;
    }
    if matched_workspaces.len() > 1 {
        return ChannelTieEvaluation::Contradictory {
            workspaces: all_workspaces,
            reason: ChannelTieConflict::MultipleCodaPatientsForEhrRecord,
        };
    }

    let workspace = matched_workspaces
        .iter()
        .next()
        .expect("len == 1 checked above")
        .clone();
    ChannelTieEvaluation::UniquePatient { workspace }
}

/// Urgency choice supplied by model classification on a patient message.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "urgency", rename_all = "snake_case")]
pub enum MessageUrgencyChoice {
    /// Routine message; waits in ordinary order.
    Ordinary,
    /// Appears clinically urgent; leads Today as `Now` while unmatched without auto-linking.
    Urgent {
        /// Plain-language clinical reason the message looks urgent.
        reason: String,
    },
}

/// Stored urgency state on a kept Spruce message.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "urgency", rename_all = "snake_case")]
pub enum MessageUrgency {
    /// Routine message.
    Ordinary,
    /// Flagged urgent; leads Today as `Now` for authorized physicians while waiting.
    Urgent {
        /// Plain-language clinical reason supplied when flagged.
        reason: String,
        /// Unix second when flagged urgent.
        flagged_at: i64,
    },
}

impl MessageUrgency {
    pub(super) const fn kind_str(&self) -> &'static str {
        match self {
            Self::Ordinary => "ordinary",
            Self::Urgent { .. } => "urgent",
        }
    }

    /// Whether this message is flagged urgent.
    #[must_use]
    pub const fn is_urgent(&self) -> bool {
        matches!(self, Self::Urgent { .. })
    }
}

/// Non-patient message category recorded when Coda filters non-patient mail.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonPatientCategory {
    /// Commercial vendor pitch, fax blast, or unsolicited marketing spam.
    VendorOrSpam,
    /// Administrative, payer, directory, or operational notice not from/about a patient.
    AdministrativeNotice,
    /// Other non-patient inquiry.
    NonPatientOutreach,
}

/// Identity-based match assessment produced by Coda's classification recipe.
/// Carries **no** numeric confidence scores.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum MatchAssessment {
    /// Permitted identity evidence names one plausible patient workspace.
    SuggestedPatient {
        /// Suggested Coda patient workspace in this practice.
        workspace: String,
        /// Concise identity basis (name, DOB, MRN, contact/relationship) without scores.
        evidence_summary: String,
    },
    /// Multiple patients could match; Coda withholds a single guess and explains why.
    AmbiguousCandidates {
        /// Plausible Coda patient workspaces (`>= 2`).
        candidate_workspaces: Vec<String>,
        /// Why the message is ambiguous across these patients.
        evidence_summary: String,
    },
    /// Sender/channel metadata and message content point to different patients.
    ConflictingEvidence {
        /// Conflicting Coda patient workspaces.
        candidate_workspaces: Vec<String>,
        /// Explanation of the contradiction (for example caregiver writing from own phone about another patient).
        evidence_summary: String,
    },
    /// No patient on the practice roster matches the permitted identity evidence.
    NoMatchFound {
        /// Why no roster patient matched.
        evidence_summary: String,
    },
}

/// Structured classification and matching decision submitted by Coda's matching recipe.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "classification", rename_all = "snake_case")]
pub enum MatchingDecision {
    /// Message is from or about a patient.
    PatientMail {
        /// Ordinary vs urgent unmatched flag.
        urgency: MessageUrgencyChoice,
        /// Identity-based candidate assessment (zero confidence scores).
        assessment: MatchAssessment,
        /// Concise physician-facing rationale.
        rationale: String,
    },
    /// Message is not from or about a patient and routes to `Filtered`.
    NotPatientMail {
        /// Kind of non-patient mail.
        category: NonPatientCategory,
        /// Concise explanation shown in `Filtered`.
        rationale: String,
    },
}

///Outcome of `Rule 4` automatic-link evaluation for an unlinked conversation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AutomaticLinkEvaluation {
    /// Exactly one patient is established by confirmed sender or `1 -> 1 -> 1 -> 1`
    /// channel tie, nothing points to any other patient, and at least one practice
    /// physician may access that patient.
    LinkAutomatically {
        /// Unique target patient workspace.
        workspace: String,
    },
    /// Zero positive ties, contradictory/ambiguous evidence, or no authorized physician;
    /// message must wait for physician confirmation.
    WaitForConfirmation,
}

/// Single rule owner implementing `MessageMatching.tla` `OwnTarget` (`Rule 4`):
/// links automatically when, and only when, `confirmed_sender_workspaces` or
/// `channel_tie` establishes **1** patient, **nothing** points to a different
/// patient (across confirmed senders, channel metadata, and model identity
/// assessment), and at least one physician in the practice may access that patient.
#[must_use]
pub fn evaluate_automatic_link(
    confirmed_sender_workspaces: &BTreeSet<String>,
    channel_tie: &ChannelTieEvaluation,
    model_assessment: Option<&MatchAssessment>,
    allowed_workspaces_with_physician: &BTreeSet<String>,
) -> AutomaticLinkEvaluation {
    let mut positive_ties = confirmed_sender_workspaces.clone();
    match channel_tie {
        ChannelTieEvaluation::UniquePatient { workspace } => {
            positive_ties.insert(workspace.clone());
        }
        ChannelTieEvaluation::Contradictory { .. } => {
            return AutomaticLinkEvaluation::WaitForConfirmation;
        }
        ChannelTieEvaluation::Untied => {}
    }

    if positive_ties.len() != 1 {
        return AutomaticLinkEvaluation::WaitForConfirmation;
    }
    let target = positive_ties.iter().next().expect("len == 1 checked above");

    if !allowed_workspaces_with_physician.contains(target) {
        return AutomaticLinkEvaluation::WaitForConfirmation;
    }

    if let Some(assessment) = model_assessment {
        match assessment {
            MatchAssessment::SuggestedPatient { workspace, .. } if workspace == target => {}
            MatchAssessment::SuggestedPatient { .. }
            | MatchAssessment::AmbiguousCandidates { .. }
            | MatchAssessment::ConflictingEvidence { .. }
            | MatchAssessment::NoMatchFound { .. } => {
                return AutomaticLinkEvaluation::WaitForConfirmation;
            }
        }
    }

    AutomaticLinkEvaluation::LinkAutomatically {
        workspace: target.clone(),
    }
}

/// Which step routed a patient message (`RouteSteps` in `MessageMatching.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "by", rename_all = "snake_case")]
pub enum RoutingOrigin {
    /// Routed when Coda submitted its classification (`CodaDecides`).
    ModelClassification,
    /// Routed when the 15-minute undecided limit elapsed (`FifteenMinutesPass`).
    FifteenMinuteFallback,
    /// Routed when a physician clicked `"This is a patient"` on `Filtered` mail (`ThisIsAPatient`).
    PhysicianMarkedPatient {
        /// Acting physician identifier.
        physician_id: String,
    },
}

/// How a message entered [`MessageStanding::Linked`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "by", rename_all = "snake_case")]
pub enum LinkOrigin {
    /// Linked automatically under `Rule 4` (`OwnTarget`).
    AutomaticRule4,
    /// Joined a conversation already linked to this patient.
    JoinedExistingConversationLink,
    /// Linked when an authorized physician confirmed the match (`LinkIt`).
    PhysicianConfirmed {
        /// Acting physician identifier.
        physician_id: String,
    },
    /// Restored to the original patient when the moving physician pressed `Undo` (`UndoIt`).
    RestoredByMoveUndo {
        /// Acting physician identifier.
        physician_id: String,
    },
}

/// Where a kept Spruce message currently stands in Coda (`MessageMatching.tla` `msg`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum MessageStanding {
    /// Envelope was kept, content could not be read (`CantRead`); waits on Today with `Done` + channel link.
    CantRead {
        /// Plain-language reason Coda could not read the message.
        failure_reason: String,
        /// Direct link to open the item in Spruce, when sent on the envelope.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        app_url: Option<String>,
    },
    /// Unreadable message cleared with `Done` by a physician (`Done`).
    MarkedDone {
        /// Retained unreadable reason.
        failure_reason: String,
        /// Direct link to open the item in Spruce, when available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        app_url: Option<String>,
        /// Physician who marked it `Done`.
        marked_by: String,
        /// Unix second when marked `Done`.
        marked_at: i64,
    },
    /// Kept readable message awaiting Coda's classification or the 15-minute deadline (`Deciding`).
    Deciding {
        /// Unix second when kept.
        kept_at: i64,
        /// Unix second (`kept_at + 900`) when Coda routes it automatically if still undecided.
        deadline_at: i64,
    },
    /// Coda classified the message as non-patient mail (`Filtered`); recoverable via `"This is a patient"`.
    Filtered {
        /// Non-patient category.
        category: NonPatientCategory,
        /// Why Coda filtered it.
        rationale: String,
        /// Unix second when filtered.
        filtered_at: i64,
    },
    /// Unmatched patient message waiting on `"Messages to match"` and Today (`Waiting`).
    Waiting {
        /// Unix second when placed on `"Messages to match"`.
        waiting_since: i64,
        /// Step that placed the message in `Waiting`.
        origin: RoutingOrigin,
    },
    /// Linked to its conversation's patient (`Linked`).
    Linked {
        /// Unix second when linked.
        linked_at: i64,
        /// How it joined the patient.
        origin: LinkOrigin,
    },
    /// Physician chose `Leave unlinked` for this message (`Unlinked`); later messages wait again.
    LeftUnlinked {
        /// Acting physician identifier.
        left_by: String,
        /// Unix second when left unlinked.
        left_at: i64,
    },
    /// Conversation was moved to `Not a patient` (`NotAPatient`).
    NotAPatient {
        /// Unix second when moved to `Not a patient`.
        moved_at: i64,
    },
}

impl MessageStanding {
    /// Canonical snake_case state tag for persistence and diagnostics.
    #[must_use]
    pub const fn kind_str(&self) -> &'static str {
        match self {
            Self::CantRead { .. } => "cant_read",
            Self::MarkedDone { .. } => "marked_done",
            Self::Deciding { .. } => "deciding",
            Self::Filtered { .. } => "filtered",
            Self::Waiting { .. } => "waiting",
            Self::Linked { .. } => "linked",
            Self::LeftUnlinked { .. } => "left_unlinked",
            Self::NotAPatient { .. } => "not_a_patient",
        }
    }
}

/// Physician override recorded when `"This is a patient"` returns a `Filtered` message to routing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicianPatientOverride {
    /// Acting physician identifier.
    pub physician_id: String,
    /// Unix second when overridden.
    pub overridden_at: i64,
}

/// Saved classification provenance and optional physician `"This is a patient"` override on a message.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedMessageClassification {
    /// Matching harness run identifier, when produced by a harness run (`None` for direct controlled tests).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// Recipe version string recorded with the classification.
    pub recipe_version: String,
    /// Guidance version string recorded with the classification.
    pub guidance_version: String,
    /// Model decision persisted on the message.
    pub decision: MatchingDecision,
    /// Unix second when saved.
    pub decided_at: i64,
    /// Present when a physician clicked `"This is a patient"` after `NotPatientMail`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physician_override: Option<PhysicianPatientOverride>,
}

/// Patient suggestion held on a waiting conversation (`shown` in `MessageMatching.tla`).
/// Contains **no** numeric confidence score and is shown only to physicians with
/// [`Membership::Allowed`] on `workspace` (`Rule 2`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SuggestedPatientMatch {
    /// Suggested Coda patient workspace.
    pub workspace: String,
    /// Identity evidence summary (name, DOB, MRN, or contact basis — never clinical chart content or scores).
    pub evidence_summary: String,
    /// Unix second when suggested.
    pub suggested_at: i64,
}

/// Single rule owner projecting a conversation's stored suggestion for `viewer` (`Rule 2`):
/// returns `None` whenever `viewer` lacks [`Membership::Allowed`] on the suggested patient.
pub async fn project_suggestion_for_viewer(
    auth: &dyn AuthStore,
    viewer: &PhysicianIdentity,
    suggestion: Option<&SuggestedPatientMatch>,
) -> Option<SuggestedPatientMatch> {
    let candidate = suggestion?;
    if auth.membership(viewer, &candidate.workspace).await == Membership::Allowed {
        Some(candidate.clone())
    } else {
        None
    }
}

/// Where a Spruce conversation is currently linked (`link` in `MessageMatching.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ConversationLinkState {
    /// Not linked to any patient yet (`Nobody`).
    Unlinked,
    /// Linked to `workspace` (`p \in Patients`).
    Linked {
        /// Linked Coda patient workspace.
        workspace: String,
        /// Unix second when linked (or moved to this patient).
        linked_at: i64,
    },
    /// Moved away from a patient to `Not a patient` (`Not a patient`).
    MovedNotAPatient {
        /// Unix second when moved.
        moved_at: i64,
    },
}

impl ConversationLinkState {
    pub(super) fn kind_and_workspace(&self) -> (&'static str, Option<String>) {
        match self {
            Self::Unlinked => ("unlinked", None),
            Self::Linked { workspace, .. } => ("linked", Some(workspace.clone())),
            Self::MovedNotAPatient { .. } => ("not_a_patient", None),
        }
    }
}

/// Destination chosen when moving a linked conversation away from its current patient.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "destination", rename_all = "snake_case")]
pub enum MoveDestination {
    /// Move the conversation to a different authorized Coda patient workspace.
    AnotherPatient {
        /// Target Coda patient workspace.
        workspace: String,
    },
    /// Move the conversation to `Not a patient`.
    NotAPatient,
}

/// Physician command moving a linked Spruce conversation to another patient or `Not a patient`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MoveSpruceConversationCommand {
    /// Practice Spruce conversation identifier.
    pub conversation_id: String,
    /// Chosen destination (`AnotherPatient` or `NotAPatient`).
    pub destination: MoveDestination,
    /// Duration in seconds during which `Undo` shows before Coda acts on the move.
    pub undo_window_seconds: i64,
    /// Client idempotency key for once-only receipt gating.
    pub command_key: String,
}

/// Active `Undo` window on a moved conversation (`undo` in `MessageMatching.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingMoveUndo {
    /// Physician who initiated the move and may press `Undo`.
    pub by_physician: String,
    /// Original patient workspace before the move began (`undo.from`).
    pub from_workspace: String,
    /// Unix second of the latest move gesture in this Undo window.
    pub moved_at: i64,
    /// Unix second when `Undo` goes away and Coda acts on the move.
    pub expires_at: i64,
    /// Message IDs that were linked on `from_workspace` when the move out of `from_workspace` began.
    /// Only these messages are marked `entered_in_error` on `from_workspace` if `Undo` expires,
    /// and any newer message linked while `Undo` showed triggers a patient review if `Undo` restores `from_workspace`.
    #[serde(default)]
    pub moved_message_ids: BTreeSet<String>,
}

impl PendingMoveUndo {
    /// Returns `true` strictly while the `Undo` window has not yet expired (`now_ts < self.expires_at`).
    #[must_use]
    pub fn is_active_at(&self, now_ts: i64) -> bool {
        now_ts < self.expires_at
    }
}

/// Attachment metadata retained on a kept Spruce message.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpruceAttachmentSummary {
    /// Spruce attachment type string.
    pub kind: String,
    /// Optional attachment title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional file name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional MIME type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
}

/// Readable Spruce message payload delivered during channel reading.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadableSpruceItem {
    /// Spruce item identifier.
    pub message_id: String,
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Monotonic 1-based position in the practice channel.
    pub channel_position: u64,
    /// Content version digest (such as `spruce::payload::Item::version()`).
    pub content_version: String,
    /// Normalized sender endpoint address when sent by an outside participant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_address: Option<SenderAddress>,
    /// Author display name when supplied by Spruce.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// Readable message body (including completed voicemail transcript when present).
    pub body_text: String,
    /// Attachment descriptors without signed URLs.
    #[serde(default)]
    pub attachments: Vec<SpruceAttachmentSummary>,
    /// Direct Spruce app URL for opening the item in Spruce.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    /// Spruce creation timestamp string when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at_spruce: Option<String>,
    /// Outside participants currently on the conversation.
    #[serde(default)]
    pub outside_participants: Vec<OutsideParticipant>,
}

/// Unreadable Spruce item envelope delivered when item content failed to decode (`Rule 13`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnreadableSpruceItem {
    /// Spruce item identifier.
    pub message_id: String,
    /// Spruce conversation identifier (or fallback synthetic envelope conversation key).
    pub conversation_id: String,
    /// Monotonic 1-based position in the practice channel.
    pub channel_position: u64,
    /// Version token for the unreadable envelope.
    pub content_version: String,
    /// Plain-language reason Coda could not read the item.
    pub failure_reason: String,
    /// Direct link to open the item in Spruce when readable on the envelope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    /// Outside participants when known on the conversation.
    #[serde(default)]
    pub outside_participants: Vec<OutsideParticipant>,
}

/// One channel delivery presented to [`Store::ingest_spruce_item`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum IncomingSpruceItem {
    /// Readable message content.
    Readable(ReadableSpruceItem),
    /// Unreadable item whose envelope was decoded (`Rule 13`).
    Unreadable(UnreadableSpruceItem),
}

impl IncomingSpruceItem {
    /// Spruce item identifier.
    #[must_use]
    pub fn message_id(&self) -> &str {
        match self {
            Self::Readable(item) => &item.message_id,
            Self::Unreadable(item) => &item.message_id,
        }
    }

    /// Monotonic 1-based channel position.
    #[must_use]
    pub const fn channel_position(&self) -> u64 {
        match self {
            Self::Readable(item) => item.channel_position,
            Self::Unreadable(item) => item.channel_position,
        }
    }
}

/// Whether confirming a message link also records the message's sender address
/// as confirmed for that patient workspace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SenderConfirmationPolicy {
    /// Link this conversation and its waiting messages without adding a global sender binding.
    ConversationOnly,
    /// Also record the message's `sender_address` (when present) as confirmed for the patient.
    ConfirmSenderForPatient,
}

/// Outcome of delivering one Spruce item to [`Store::ingest_spruce_item`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum SpruceIngestOutcome {
    /// Item was newly kept in [`MessageStanding::Deciding`] or [`MessageStanding::CantRead`].
    KeptNew {
        /// Saved standing of the kept message.
        standing: MessageStanding,
    },
    /// Previously `CantRead` item was re-read cleanly into [`MessageStanding::Deciding`] (`CodaReadsItNow`).
    RecoveredUnreadable {
        /// Updated [`MessageStanding::Deciding`] standing.
        standing: MessageStanding,
    },
    /// Item was already kept; duplicate/out-of-order redelivery changed nothing (`Rule 12`).
    AlreadyKept {
        /// Existing standing preserved unchanged.
        standing: MessageStanding,
    },
    /// Pre-connection item (`channel_position <= start_boundary`) ignored while backlog offer is unaccepted (`Rule 14`).
    SkippedPreConnectionBacklog,
}

/// Why Coda queued a patient review from Spruce message matching or a settled move.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "origin", rename_all = "snake_case")]
pub enum SpruceReviewTriggerOrigin {
    /// Conversation linked automatically under `Rule 4`.
    AutomaticLink,
    /// Message arrived in a conversation already linked to this patient.
    LinkedConversationArrival,
    /// Physician confirmed a waiting message onto this patient (`Confirm`).
    ConfirmedLink {
        /// Acting physician identifier.
        physician_id: String,
    },
    /// Settled conversation move left this patient (`Undo` expired).
    MoveSettledFrom,
    /// Settled conversation move arrived on this patient (`Undo` expired).
    MoveSettledTo,
}

/// Durable once-only review trigger record associated with Spruce matching/move settlement.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingSpruceReviewTrigger {
    /// Practice identifier.
    pub practice: String,
    /// Deterministic once-only `ReviewRequest.command_key`.
    pub command_key: String,
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Affected patient workspace.
    pub workspace: String,
    /// Trigger origin.
    pub origin: SpruceReviewTriggerOrigin,
    /// Workspace chart observation version recorded when queued.
    pub workspace_version: u64,
    /// Active source and linked Spruce message versions observed when queued.
    pub record_versions: BTreeMap<String, String>,
    /// Whether `ReviewReceiver::submit_review` has succeeded for this trigger.
    pub dispatched: bool,
    /// Unix second when queued.
    pub created_at: i64,
    /// Unix second when `submit_review` succeeded.
    pub dispatched_at: Option<i64>,
}

/// Permanent audit record of a message marked entered in error on `from_workspace`
/// after a wrong-patient conversation move settled (`README.md` non-TLC rule).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpruceEnteredInErrorRecord {
    /// Practice identifier.
    pub practice: String,
    /// Patient workspace the message was moved away from.
    pub from_workspace: String,
    /// Spruce message identifier (retained under record policy, never deleted).
    pub message_id: String,
    /// Move ordinal on the conversation when settled.
    pub move_ordinal: u64,
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Where the conversation moved to.
    pub destination: MoveDestination,
    /// Unix second when marked entered in error.
    pub marked_at: i64,
}

/// Saved Spruce message state returned by runtime inspection and projections.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedSpruceMessage {
    /// Practice identifier.
    pub practice: String,
    /// Spruce item identifier.
    pub message_id: String,
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Monotonic 1-based channel position.
    pub channel_position: u64,
    /// Content version digest.
    pub content_version: String,
    /// Current message standing.
    pub standing: MessageStanding,
    /// Current urgency state.
    pub urgency: MessageUrgency,
    /// Monotonic latch: `true` if this message was ever linked to any patient.
    pub ever_linked: bool,
    /// Sender endpoint address when present.
    pub sender_address: Option<SenderAddress>,
    /// Author display name when present.
    pub author_name: Option<String>,
    /// Readable body text (empty for `CantRead`).
    pub body_text: String,
    /// Attachment summaries.
    pub attachments: Vec<SpruceAttachmentSummary>,
    /// Direct link to open in Spruce.
    pub app_url: Option<String>,
    /// Spruce creation timestamp string.
    pub created_at_spruce: Option<String>,
    /// Unix second when kept.
    pub kept_at: i64,
    /// Saved model classification and optional physician override.
    pub saved_classification: Option<SavedMessageClassification>,
    /// Unix second of latest update.
    pub updated_at: i64,
}

/// Saved Spruce conversation link, suggestion, viewer, and move state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedSpruceConversation {
    /// Practice identifier.
    pub practice: String,
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Current link state (`Unlinked`, `Linked`, or `MovedNotAPatient`).
    pub link_state: ConversationLinkState,
    /// Outside participants recorded on the conversation.
    pub outside_participants: Vec<OutsideParticipant>,
    /// Internal raw suggestion while messages wait (filtered by viewer access before display).
    pub active_suggestion: Option<SuggestedPatientMatch>,
    /// Physician IDs who currently have this conversation open.
    pub open_viewers: BTreeSet<String>,
    /// Active `Undo` state during a move window.
    pub move_undo: Option<PendingMoveUndo>,
    /// Total moves executed on this conversation.
    pub move_count: u64,
    /// Whether Coda has acted on the latest move after `Undo` expired (`acted` in `MessageMatching.tla`).
    pub last_move_acted: bool,
    /// Unix second of latest update.
    pub updated_at: i64,
}

/// Physician-authorized row on `"Messages to match"` (and Today's unmatched queue).
/// Suggestion is filtered by [`Membership::Allowed`] (`Rule 2`) and carries zero confidence scores.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnmatchedMessageRow {
    /// Spruce message identifier.
    pub message_id: String,
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Author display name, when supplied.
    pub author_name: Option<String>,
    /// Sender endpoint address, when supplied.
    pub sender_address: Option<SenderAddress>,
    /// Message preview/body text.
    pub body_text: String,
    /// Attachments on the message.
    pub attachments: Vec<SpruceAttachmentSummary>,
    /// Direct Spruce app link.
    pub app_url: Option<String>,
    /// Urgency state (`Urgent` rows lead Today as `Now`).
    pub urgency: MessageUrgency,
    /// Whether this unmatched row leads Today as `Now`.
    pub leads_today_as_now: bool,
    /// Patient suggestion visible to this viewer (`None` when none suggested or viewer lacks access).
    pub suggested_patient: Option<SuggestedPatientMatch>,
    /// Unix second since waiting.
    pub waiting_since: i64,
}

/// Physician-visible `Filtered` non-patient message row offering `"This is a patient"`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilteredMessageRow {
    /// Spruce message identifier.
    pub message_id: String,
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Author display name, when supplied.
    pub author_name: Option<String>,
    /// Sender address, when supplied.
    pub sender_address: Option<SenderAddress>,
    /// Message text.
    pub body_text: String,
    /// Non-patient category.
    pub category: NonPatientCategory,
    /// Why Coda filtered the message.
    pub rationale: String,
    /// Unix second when filtered.
    pub filtered_at: i64,
}

/// Unreadable message row waiting on Today with a link to open in Spruce and `Done` (`Rule 13`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnreadableMessageRow {
    /// Spruce message identifier.
    pub message_id: String,
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Why Coda could not read the message.
    pub failure_reason: String,
    /// Link to open the message in Spruce.
    pub app_url: Option<String>,
    /// Unix second when kept.
    pub kept_at: i64,
}

/// Today's Spruce channel banner, backlog offer, urgent/waiting unmatched rows,
/// and unreadable messages for an authorized practice physician.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodaySpruceSummary {
    /// Today's channel health banner (`Quiet`, `CannotRead`, or `ReconnectRequired`).
    pub channel_banner: TodayChannelBanner,
    /// Whether the one-time pre-connection backlog import offer is currently open (`Rule 14`).
    pub backlog_offered: bool,
    /// Waiting unmatched messages (ordered urgent `Now` first, then chronological, with access-filtered suggestions).
    pub messages_to_match: Vec<UnmatchedMessageRow>,
    /// Unreadable messages waiting on Today until Coda reads them or a physician marks `Done` (`Rule 13`).
    pub unreadable_messages: Vec<UnreadableMessageRow>,
}

/// Authorized linked conversation projection for a physician with [`Membership::Allowed`] (`Rule 1` & `Rule 5`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizedSpruceConversation {
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Linked Coda patient workspace.
    pub workspace: String,
    /// Linked messages belonging to this conversation on `workspace`.
    pub messages: Vec<SavedSpruceMessage>,
    /// Active `Undo` state when a move is still inside its Undo window.
    pub move_undo: Option<PendingMoveUndo>,
    /// Whether this physician currently has the conversation open.
    pub is_open_for_viewer: bool,
}

/// Authorized demographic and contact roster entry exposed to `patient_lookup` during
/// unmatched message classification. Contains **zero** clinical chart content.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchingRosterPatient {
    /// Registered Coda patient workspace in this practice.
    pub workspace: String,
    /// Demographic identity (`display_name`, `birth_date`, `sex_or_gender`, `medical_record_numbers`) when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub demographics: Option<SourcePatient>,
    /// Sender addresses previously confirmed for this patient.
    #[serde(default)]
    pub confirmed_senders: Vec<SenderAddress>,
    /// Spruce contact IDs tied to this patient through EHR links.
    #[serde(default)]
    pub linked_spruce_contacts: Vec<String>,
    /// Linked `(source_id, source_patient_id)` EHR pairs for this workspace.
    #[serde(default)]
    pub ehr_links: Vec<SpruceEhrLink>,
}

/// Complete review input bundle for one patient workspace:
/// retained chart observation plus **only** Spruce messages currently `Linked`
/// to `workspace`. Unlinked, waiting, deciding, filtered, left-unlinked,
/// unreadable, and moved-away messages never appear here.
#[derive(Clone, Debug, PartialEq)]
pub struct PatientReviewInputs {
    /// Practice owning the patient workspace.
    pub practice: String,
    /// Patient workspace under review.
    pub workspace: String,
    /// Versioned chart source observation for `workspace`.
    pub chart: Observation,
    /// Linked Spruce messages currently on `workspace` (never unlinked or moved-away mail).
    pub linked_spruce_messages: Vec<SavedSpruceMessage>,
}

/// Whether a physician-authored link confirmation was newly applied or replayed from receipt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkCommandAcceptance {
    /// First execution of `command_key`.
    New,
    /// Idempotent replay of an identical prior `command_key`.
    Duplicate,
}

/// Outcome of confirming a waiting message link (`Store::confirm_spruce_message_link`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfirmSpruceLinkOutcome {
    /// Whether newly accepted or replayed from a prior receipt under the same `command_key`.
    pub acceptance: LinkCommandAcceptance,
    /// Linked conversation identifier.
    pub conversation_id: String,
    /// Linked Coda patient workspace.
    pub workspace: String,
    /// Message IDs joined onto `workspace` by that confirmation (the confirmed message + any sibling `Waiting` messages).
    pub linked_message_ids: Vec<String>,
    /// Deterministic once-only `ReviewRequest.command_key` created for `workspace`.
    pub review_command_key: String,
}

/// Outcome of routing a message (`CodaDecides`, `FifteenMinutesPass`, or `"This is a patient"`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum MessageRoutingOutcome {
    /// Message (and any sibling `Waiting` messages) linked to `workspace`.
    Linked {
        /// Linked patient workspace.
        workspace: String,
        /// Origin of the link.
        origin: LinkOrigin,
        /// Joined message IDs.
        joined_message_ids: Vec<String>,
        /// Optional queued `ReviewRequest.command_key` (`None` while an active `Undo` window holds).
        review_command_key: Option<String>,
    },
    /// Message linked to a patient workspace that the acting physician is not authorized to view (`Rule 1` & `Rule 2`).
    LinkedWithoutViewerAccess {
        /// Origin of the link.
        origin: LinkOrigin,
        /// Joined message IDs.
        joined_message_ids: Vec<String>,
    },
    /// Message entered `Waiting` on `"Messages to match"`.
    Waiting {
        /// Active conversation suggestion (`None` if none suggested or redacted for viewer access).
        suggested_workspace: Option<String>,
    },
    /// Message was classified as `Filtered` non-patient mail.
    Filtered {
        /// Non-patient category.
        category: NonPatientCategory,
    },
    /// Message followed a conversation already moved to `Not a patient`.
    NotAPatient,
}

/// Per-item or per-patient failure captured during a batch sweep so one patient's
/// failure never blocks other patients' messages while keeping the failure explicit.
#[derive(Debug)]
pub struct PatientIsolationFailure {
    /// Conversation, message, or patient workspace identifier that failed.
    pub target_id: String,
    /// Retained error with full `#[source]` chain.
    pub error: SpruceMatchingError,
}

/// Result of a multi-message or multi-patient sweep (`ingest_spruce_batch`,
/// `settle_undecided_spruce_messages`, `settle_expired_spruce_moves`, `dispatch_pending_spruce_reviews`).
#[derive(Debug)]
pub struct BatchStepReport<T> {
    /// Operations that succeeded independently.
    pub succeeded: Vec<T>,
    /// Operations that failed with their retained causes (non-empty when any item failed).
    pub failures: Vec<PatientIsolationFailure>,
}

impl<T> BatchStepReport<T> {
    /// True only when zero items failed.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.failures.is_empty()
    }
}

/// Errors produced by Spruce channel, matching, move, and review-handoff operations.
#[derive(Debug, thiserror::Error)]
pub enum SpruceMatchingError {
    /// Practice, physician, conversation, message, workspace, or key identifier was empty or invalid.
    #[error("invalid spruce matching identifier for `{field}`")]
    InvalidIdentifier {
        /// Name of the invalid field.
        field: &'static str,
    },
    /// Free-text failure or status reason was empty or contained NUL bytes.
    #[error("invalid spruce matching reason text for `{field}`")]
    InvalidReasonText {
        /// Name of the invalid reason field.
        field: &'static str,
    },
    /// Viewer is not an active physician in the practice or lacks [`Membership::Allowed`] on the patient.
    #[error("access denied for physician `{physician}` in practice `{practice}`: {reason}")]
    AccessDenied {
        /// Practice identifier.
        practice: String,
        /// Physician identifier.
        physician: String,
        /// Why access was refused.
        reason: &'static str,
    },
    /// Practice patient authority check refused the workspace.
    #[error("practice workspace access refused: {0}")]
    PracticeAccess(#[from] PracticeAccessError),
    /// Another active reader holds the practice's Spruce channel lease.
    #[error(
        "spruce channel in practice `{practice}` is already leased by `{holder_id}` until {expires_at}"
    )]
    ReaderLeaseHeld {
        /// Practice identifier.
        practice: String,
        /// Active lease holder.
        holder_id: String,
        /// Lease expiry Unix second.
        expires_at: i64,
    },
    /// Supplied lease token is missing, expired, or belongs to a different reader.
    #[error("spruce channel reader lease is not active for practice `{practice}`")]
    InvalidReaderLease {
        /// Practice identifier.
        practice: String,
    },
    /// Operation requires the channel to be in a different state (for example `Reading` or `RevokedOrBlocked`).
    #[error("spruce channel in practice `{practice}` cannot `{action}` while `{state}`")]
    ChannelStateRefused {
        /// Practice identifier.
        practice: String,
        /// Current channel state description.
        state: String,
        /// Attempted operation.
        action: &'static str,
    },
    /// Requested message or conversation does not exist in `practice`.
    #[error("spruce {kind} `{id}` not found in practice `{practice}`")]
    NotFound {
        /// Practice identifier.
        practice: String,
        /// `"message"`, `"conversation"`, or `"matching_run"`.
        kind: &'static str,
        /// Missing identifier.
        id: String,
    },
    /// State transition is invalid for the message's or conversation's current standing.
    #[error("cannot `{action}` on `{id}` while in state `{current_state}`")]
    InvalidTransition {
        /// Target message or conversation identifier.
        id: String,
        /// Current state label.
        current_state: String,
        /// Requested action name.
        action: &'static str,
    },
    /// Command key was already used for a different command payload.
    #[error("idempotency key `{key}` was already used for a different spruce matching command")]
    IdempotencyCollision {
        /// Colliding command key.
        key: String,
    },
    /// Authority store could not answer an access or practice query.
    #[error("physician authority store unavailable: {0}")]
    AuthorityUnavailable(#[from] StoreUnavailable),
    /// Handing off a patient review request failed before `ReviewReceiver` accepted it.
    #[error("patient review handoff failed for workspace `{workspace}`: {source}")]
    ReviewHandoff {
        /// Affected patient workspace.
        workspace: String,
        /// Underlying practice job / review receiver failure.
        #[source]
        source: PracticeJobError,
    },
    /// Database error from SeaORM.
    #[error("spruce matching database failure: {0}")]
    Database(#[from] DbErr),
}

fn require_id<'a>(value: &'a str, field: &'static str) -> Result<&'a str, SpruceMatchingError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
        return Err(SpruceMatchingError::InvalidIdentifier { field });
    }
    Ok(trimmed)
}

fn require_reason_text<'a>(
    value: &'a str,
    field: &'static str,
) -> Result<&'a str, SpruceMatchingError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.contains('\0') {
        return Err(SpruceMatchingError::InvalidReasonText { field });
    }
    Ok(trimmed)
}

/// Single rule owner redacting a [`MessageRoutingOutcome`] returned to a physician actor (`Rule 1` & `Rule 2`)
/// so a physician never sees a patient workspace ID or suggestion for a patient they may not access.
async fn project_routing_outcome_for_viewer(
    auth: &dyn AuthStore,
    viewer: &PhysicianIdentity,
    outcome: MessageRoutingOutcome,
) -> MessageRoutingOutcome {
    match outcome {
        MessageRoutingOutcome::Linked {
            workspace,
            origin,
            joined_message_ids,
            review_command_key,
        } => {
            if auth.membership(viewer, &workspace).await == Membership::Allowed {
                MessageRoutingOutcome::Linked {
                    workspace,
                    origin,
                    joined_message_ids,
                    review_command_key,
                }
            } else {
                MessageRoutingOutcome::LinkedWithoutViewerAccess {
                    origin,
                    joined_message_ids,
                }
            }
        }
        MessageRoutingOutcome::Waiting {
            suggested_workspace: Some(ws),
        } => {
            let suggested_workspace = if auth.membership(viewer, &ws).await == Membership::Allowed {
                Some(ws)
            } else {
                None
            };
            MessageRoutingOutcome::Waiting {
                suggested_workspace,
            }
        }
        other => other,
    }
}

async fn require_active_physician(
    auth: &dyn AuthStore,
    viewer: &PhysicianIdentity,
) -> Result<(), SpruceMatchingError> {
    require_id(&viewer.practice_id, "practice_id")?;
    require_id(&viewer.physician_id, "physician_id")?;
    let Some(account) = auth.account_by_identity(viewer).await else {
        return Err(SpruceMatchingError::AccessDenied {
            practice: viewer.practice_id.clone(),
            physician: viewer.physician_id.clone(),
            reason: "physician account does not exist in practice",
        });
    };
    if !account.active {
        return Err(SpruceMatchingError::AccessDenied {
            practice: viewer.practice_id.clone(),
            physician: viewer.physician_id.clone(),
            reason: "physician account is inactive",
        });
    }
    Ok(())
}

async fn require_patient_access(
    auth: &dyn AuthStore,
    viewer: &PhysicianIdentity,
    workspace: &str,
) -> Result<(), SpruceMatchingError> {
    require_active_physician(auth, viewer).await?;
    let workspace = require_id(workspace, "workspace")?;
    let owner = auth.patient_practice(workspace).await?;
    if owner.as_deref() != Some(viewer.practice_id.as_str()) {
        return Err(SpruceMatchingError::AccessDenied {
            practice: viewer.practice_id.clone(),
            physician: viewer.physician_id.clone(),
            reason: "patient workspace does not belong to practice",
        });
    }
    if auth.membership(viewer, workspace).await != Membership::Allowed {
        return Err(SpruceMatchingError::AccessDenied {
            practice: viewer.practice_id.clone(),
            physician: viewer.physician_id.clone(),
            reason: "physician is not allowed to access patient workspace",
        });
    }
    Ok(())
}

pub(crate) async fn allowed_workspaces_in_practice(
    auth: &dyn AuthStore,
    practice: &str,
) -> Result<BTreeSet<String>, SpruceMatchingError> {
    let registered: BTreeSet<String> = auth
        .practice_patients(practice)
        .await?
        .into_iter()
        .collect();
    let accounts = auth.practice_accounts(practice).await?;
    let mut allowed = BTreeSet::new();
    for account in accounts {
        if !account.active {
            continue;
        }
        for ws in auth.panel(&account.identity).await {
            if registered.contains(&ws) {
                allowed.insert(ws);
            }
        }
    }
    Ok(allowed)
}

async fn still_open_viewers(
    auth: &dyn AuthStore,
    practice: &str,
    viewers: &BTreeSet<String>,
    target_workspace: Option<&str>,
) -> BTreeSet<String> {
    let Some(workspace) = target_workspace else {
        return BTreeSet::new();
    };
    let mut retained = BTreeSet::new();
    for physician_id in viewers {
        let identity = PhysicianIdentity {
            practice_id: practice.to_owned(),
            physician_id: physician_id.clone(),
        };
        let Some(account) = auth.account_by_identity(&identity).await else {
            continue;
        };
        if account.active && auth.membership(&identity, workspace).await == Membership::Allowed {
            retained.insert(physician_id.clone());
        }
    }
    retained
}

fn decode_channel_row(
    row: entities::spruce_channel::Model,
) -> Result<(ChannelStanding, Option<ChannelReaderLease>), DbErr> {
    let standing: ChannelStanding = serde_json::from_value(row.standing).map_err(
        conversion_error("spruce_channel.standing", "ChannelStanding"),
    )?;
    let lease: Option<ChannelReaderLease> = row
        .reader_lease
        .map(serde_json::from_value)
        .transpose()
        .map_err(conversion_error(
            "spruce_channel.reader_lease",
            "ChannelReaderLease",
        ))?;
    Ok((standing, lease))
}

fn decode_conversation_row(
    row: entities::spruce_conversation::Model,
) -> Result<SavedSpruceConversation, DbErr> {
    let link_state: ConversationLinkState = serde_json::from_value(row.link_state).map_err(
        conversion_error("spruce_conversation.link_state", "ConversationLinkState"),
    )?;
    let outside_participants: Vec<OutsideParticipant> =
        serde_json::from_value(row.outside_participants).map_err(conversion_error(
            "spruce_conversation.outside_participants",
            "Vec<OutsideParticipant>",
        ))?;
    let active_suggestion: Option<SuggestedPatientMatch> = row
        .active_suggestion
        .map(serde_json::from_value)
        .transpose()
        .map_err(conversion_error(
            "spruce_conversation.active_suggestion",
            "SuggestedPatientMatch",
        ))?;
    let open_viewers: BTreeSet<String> = serde_json::from_value(row.open_viewers).map_err(
        conversion_error("spruce_conversation.open_viewers", "BTreeSet<String>"),
    )?;
    let move_undo: Option<PendingMoveUndo> = row
        .move_undo
        .map(serde_json::from_value)
        .transpose()
        .map_err(conversion_error(
            "spruce_conversation.move_undo",
            "PendingMoveUndo",
        ))?;
    let move_count = u64::try_from(row.move_count)
        .map_err(conversion_error("spruce_conversation.move_count", "u64"))?;
    Ok(SavedSpruceConversation {
        practice: row.practice,
        conversation_id: row.conversation_id,
        link_state,
        outside_participants,
        active_suggestion,
        open_viewers,
        move_undo,
        move_count,
        last_move_acted: row.last_move_acted,
        updated_at: row.updated_at,
    })
}

fn decode_message_row(row: entities::spruce_message::Model) -> Result<SavedSpruceMessage, DbErr> {
    let channel_position = u64::try_from(row.channel_position)
        .map_err(conversion_error("spruce_message.channel_position", "u64"))?;
    let standing: MessageStanding = serde_json::from_value(row.standing).map_err(
        conversion_error("spruce_message.standing", "MessageStanding"),
    )?;
    let urgency: MessageUrgency = serde_json::from_value(row.urgency)
        .map_err(conversion_error("spruce_message.urgency", "MessageUrgency"))?;
    let sender_address = row
        .sender_address
        .map(|raw| SenderAddress::parse(&raw).map_err(|e| error(&e.to_string())))
        .transpose()?;
    let attachments: Vec<SpruceAttachmentSummary> = serde_json::from_value(row.attachments)
        .map_err(conversion_error(
            "spruce_message.attachments",
            "Vec<SpruceAttachmentSummary>",
        ))?;
    let saved_classification: Option<SavedMessageClassification> = row
        .saved_classification
        .map(serde_json::from_value)
        .transpose()
        .map_err(conversion_error(
            "spruce_message.saved_classification",
            "SavedMessageClassification",
        ))?;
    Ok(SavedSpruceMessage {
        practice: row.practice,
        message_id: row.message_id,
        conversation_id: row.conversation_id,
        channel_position,
        content_version: row.content_version,
        standing,
        urgency,
        ever_linked: row.ever_linked,
        sender_address,
        author_name: row.author_name,
        body_text: row.body_text,
        attachments,
        app_url: row.app_url,
        created_at_spruce: row.created_at_spruce,
        kept_at: row.kept_at,
        saved_classification,
        updated_at: row.updated_at,
    })
}

fn decode_trigger_row(
    row: entities::spruce_review_trigger::Model,
) -> Result<PendingSpruceReviewTrigger, DbErr> {
    let origin: SpruceReviewTriggerOrigin = serde_json::from_value(row.origin).map_err(
        conversion_error("spruce_review_trigger.origin", "SpruceReviewTriggerOrigin"),
    )?;
    let workspace_version = u64::try_from(row.workspace_version).map_err(conversion_error(
        "spruce_review_trigger.workspace_version",
        "u64",
    ))?;
    let record_versions: BTreeMap<String, String> = serde_json::from_value(row.record_versions)
        .map_err(conversion_error(
            "spruce_review_trigger.record_versions",
            "BTreeMap<String, String>",
        ))?;
    Ok(PendingSpruceReviewTrigger {
        practice: row.practice,
        command_key: row.command_key,
        conversation_id: row.conversation_id,
        workspace: row.workspace,
        origin,
        workspace_version,
        record_versions,
        dispatched: row.dispatched,
        created_at: row.created_at,
        dispatched_at: row.dispatched_at,
    })
}

async fn ensure_channel_row(
    tx: &impl ConnectionTrait,
    practice: &str,
    now_ts: i64,
) -> Result<(), DbErr> {
    let default_standing = ChannelStanding::Disconnected {
        disconnected_at: None,
    };
    let standing_json = serde_json::to_value(&default_standing)
        .map_err(conversion_error("ChannelStanding", "json"))?;
    entities::spruce_channel::Entity::insert(entities::spruce_channel::ActiveModel {
        practice: Set(practice.to_owned()),
        standing_kind: Set(default_standing.kind_str().to_owned()),
        standing: Set(standing_json),
        reader_lease: Set(None),
        updated_at: Set(now_ts),
    })
    .on_conflict(
        OnConflict::column(entities::spruce_channel::Column::Practice)
            .do_nothing()
            .to_owned(),
    )
    .exec_without_returning(tx)
    .await?;
    Ok(())
}

async fn lock_channel_row(
    tx: &sea_orm::DatabaseTransaction,
    practice: &str,
    now_ts: i64,
) -> Result<entities::spruce_channel::Model, DbErr> {
    ensure_channel_row(tx, practice, now_ts).await?;
    entities::spruce_channel::Entity::find_by_id(practice.to_owned())
        .lock_exclusive()
        .one(tx)
        .await?
        .ok_or_else(|| error("missing spruce channel row"))
}

async fn save_channel_state_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    practice: &str,
    standing: &ChannelStanding,
    lease: Option<&ChannelReaderLease>,
    now_ts: i64,
) -> Result<(), DbErr> {
    let standing_json =
        serde_json::to_value(standing).map_err(conversion_error("ChannelStanding", "json"))?;
    let lease_json = lease
        .map(serde_json::to_value)
        .transpose()
        .map_err(conversion_error("ChannelReaderLease", "json"))?;
    let active = entities::spruce_channel::ActiveModel {
        practice: Set(practice.to_owned()),
        standing_kind: Set(standing.kind_str().to_owned()),
        standing: Set(standing_json),
        reader_lease: Set(lease_json),
        updated_at: Set(now_ts),
    };
    active.update(tx).await?;
    Ok(())
}

async fn ensure_conversation_row(
    tx: &impl ConnectionTrait,
    practice: &str,
    conversation_id: &str,
    outside_participants: &[OutsideParticipant],
    now_ts: i64,
) -> Result<(), DbErr> {
    let link_state = ConversationLinkState::Unlinked;
    let link_json = serde_json::to_value(&link_state)
        .map_err(conversion_error("ConversationLinkState", "json"))?;
    let participants_json = serde_json::to_value(outside_participants)
        .map_err(conversion_error("Vec<OutsideParticipant>", "json"))?;
    entities::spruce_conversation::Entity::insert(entities::spruce_conversation::ActiveModel {
        practice: Set(practice.to_owned()),
        conversation_id: Set(conversation_id.to_owned()),
        link_kind: Set("unlinked".to_owned()),
        linked_workspace: Set(None),
        link_state: Set(link_json),
        outside_participants: Set(participants_json),
        active_suggestion: Set(None),
        open_viewers: Set(json!([])),
        move_undo: Set(None),
        move_count: Set(0),
        last_move_acted: Set(false),
        updated_at: Set(now_ts),
    })
    .on_conflict(
        OnConflict::columns([
            entities::spruce_conversation::Column::Practice,
            entities::spruce_conversation::Column::ConversationId,
        ])
        .do_nothing()
        .to_owned(),
    )
    .exec_without_returning(tx)
    .await?;
    Ok(())
}

async fn lock_conversation_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    practice: &str,
    conversation_id: &str,
) -> Result<Option<SavedSpruceConversation>, DbErr> {
    entities::spruce_conversation::Entity::find_by_id((
        practice.to_owned(),
        conversation_id.to_owned(),
    ))
    .lock_exclusive()
    .one(tx)
    .await?
    .map(decode_conversation_row)
    .transpose()
}

async fn save_conversation_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    conv: &SavedSpruceConversation,
) -> Result<(), DbErr> {
    let (link_kind, linked_workspace) = conv.link_state.kind_and_workspace();
    let link_json = serde_json::to_value(&conv.link_state)
        .map_err(conversion_error("ConversationLinkState", "json"))?;
    let participants_json = serde_json::to_value(&conv.outside_participants)
        .map_err(conversion_error("Vec<OutsideParticipant>", "json"))?;
    let suggestion_json = conv
        .active_suggestion
        .as_ref()
        .map(serde_json::to_value)
        .transpose()
        .map_err(conversion_error("SuggestedPatientMatch", "json"))?;
    let viewers_json = serde_json::to_value(&conv.open_viewers)
        .map_err(conversion_error("BTreeSet<String>", "json"))?;
    let undo_json = conv
        .move_undo
        .as_ref()
        .map(serde_json::to_value)
        .transpose()
        .map_err(conversion_error("PendingMoveUndo", "json"))?;
    let move_count =
        i64::try_from(conv.move_count).map_err(conversion_error("move_count", "i64"))?;
    entities::spruce_conversation::ActiveModel {
        practice: Set(conv.practice.clone()),
        conversation_id: Set(conv.conversation_id.clone()),
        link_kind: Set(link_kind.to_owned()),
        linked_workspace: Set(linked_workspace),
        link_state: Set(link_json),
        outside_participants: Set(participants_json),
        active_suggestion: Set(suggestion_json),
        open_viewers: Set(viewers_json),
        move_undo: Set(undo_json),
        move_count: Set(move_count),
        last_move_acted: Set(conv.last_move_acted),
        updated_at: Set(conv.updated_at),
    }
    .update(tx)
    .await?;
    Ok(())
}

async fn save_message_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    msg: &SavedSpruceMessage,
) -> Result<(), DbErr> {
    let pos =
        i64::try_from(msg.channel_position).map_err(conversion_error("channel_position", "i64"))?;
    let standing_json =
        serde_json::to_value(&msg.standing).map_err(conversion_error("MessageStanding", "json"))?;
    let urgency_json =
        serde_json::to_value(&msg.urgency).map_err(conversion_error("MessageUrgency", "json"))?;
    let attachments_json = serde_json::to_value(&msg.attachments)
        .map_err(conversion_error("Vec<SpruceAttachmentSummary>", "json"))?;
    let classification_json = msg
        .saved_classification
        .as_ref()
        .map(serde_json::to_value)
        .transpose()
        .map_err(conversion_error("SavedMessageClassification", "json"))?;
    let ever_linked = msg.ever_linked || matches!(msg.standing, MessageStanding::Linked { .. });
    entities::spruce_message::ActiveModel {
        practice: Set(msg.practice.clone()),
        message_id: Set(msg.message_id.clone()),
        conversation_id: Set(msg.conversation_id.clone()),
        channel_position: Set(pos),
        content_version: Set(msg.content_version.clone()),
        standing_kind: Set(msg.standing.kind_str().to_owned()),
        standing: Set(standing_json),
        urgency_kind: Set(msg.urgency.kind_str().to_owned()),
        urgency: Set(urgency_json),
        ever_linked: Set(ever_linked),
        sender_address: Set(msg.sender_address.as_ref().map(|a| a.as_str().to_owned())),
        author_name: Set(msg.author_name.clone()),
        body_text: Set(msg.body_text.clone()),
        attachments: Set(attachments_json),
        app_url: Set(msg.app_url.clone()),
        created_at_spruce: Set(msg.created_at_spruce.clone()),
        kept_at: Set(msg.kept_at),
        saved_classification: Set(classification_json),
        updated_at: Set(msg.updated_at),
    }
    .update(tx)
    .await?;
    Ok(())
}

async fn load_practice_channel_evidence_in_tx(
    tx: &impl ConnectionTrait,
    practice: &str,
    registered_workspaces: &BTreeSet<String>,
) -> Result<
    (
        BTreeMap<String, Vec<SpruceContactPatient>>,
        BTreeMap<SpruceEhrLink, BTreeSet<String>>,
    ),
    DbErr,
> {
    let contact_rows = entities::spruce_contact::Entity::find()
        .filter(entities::spruce_contact::Column::Practice.eq(practice))
        .all(tx)
        .await?;
    let mut contacts = BTreeMap::new();
    for row in contact_rows {
        let patients: Vec<SpruceContactPatient> = serde_json::from_value(row.spruce_patients)
            .map_err(conversion_error(
                "spruce_contact.spruce_patients",
                "Vec<SpruceContactPatient>",
            ))?;
        contacts.insert(row.contact_id, patients);
    }

    let mut ehr_map: BTreeMap<SpruceEhrLink, BTreeSet<String>> = BTreeMap::new();
    let ehr_rows = entities::spruce_ehr_link::Entity::find()
        .filter(entities::spruce_ehr_link::Column::Practice.eq(practice))
        .all(tx)
        .await?;
    for row in ehr_rows {
        if registered_workspaces.contains(&row.workspace) {
            ehr_map
                .entry(SpruceEhrLink {
                    source_id: row.source_id,
                    source_patient_id: row.source_patient_id,
                })
                .or_default()
                .insert(row.workspace);
        }
    }

    let schedule_rows = schedule_patient_link::Entity::find()
        .filter(schedule_patient_link::Column::Practice.eq(practice))
        .all(tx)
        .await?;
    for row in schedule_rows {
        if registered_workspaces.contains(&row.workspace) {
            ehr_map
                .entry(SpruceEhrLink {
                    source_id: row.source_id,
                    source_patient_id: row.source_patient_id,
                })
                .or_default()
                .insert(row.workspace);
        }
    }

    if !registered_workspaces.is_empty() {
        let pulls = source_pull::Entity::find()
            .filter(source_pull::Column::Workspace.is_in(registered_workspaces.iter().cloned()))
            .all(tx)
            .await?;
        for row in pulls {
            let patient: SourcePatient = serde_json::from_value(row.patient)
                .map_err(conversion_error("source_pull.patient", "SourcePatient"))?;
            ehr_map
                .entry(SpruceEhrLink {
                    source_id: row.source_id,
                    source_patient_id: patient.source_patient_id,
                })
                .or_default()
                .insert(row.workspace);
        }
    }

    Ok((contacts, ehr_map))
}

async fn confirmed_workspaces_for_conversation_in_tx(
    tx: &impl ConnectionTrait,
    practice: &str,
    sender: Option<&SenderAddress>,
    outside_participants: &[OutsideParticipant],
    registered_workspaces: &BTreeSet<String>,
) -> Result<BTreeSet<String>, DbErr> {
    let mut keys = BTreeSet::new();
    if let Some(addr) = sender {
        keys.insert(addr.as_str().to_owned());
    }
    for participant in outside_participants {
        if let Some(addr) = &participant.endpoint_address {
            keys.insert(addr.as_str().to_owned());
        }
    }
    if keys.is_empty() {
        return Ok(BTreeSet::new());
    }
    let rows = entities::spruce_confirmed_sender::Entity::find()
        .filter(entities::spruce_confirmed_sender::Column::Practice.eq(practice))
        .filter(entities::spruce_confirmed_sender::Column::SenderKey.is_in(keys))
        .all(tx)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| row.workspace)
        .filter(|ws| registered_workspaces.contains(ws))
        .collect())
}

async fn queue_review_trigger_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    practice: &str,
    conversation_id: &str,
    workspace: &str,
    command_key: &str,
    origin: SpruceReviewTriggerOrigin,
    now_ts: i64,
) -> Result<PendingSpruceReviewTrigger, DbErr> {
    let observation = super::sources::observed(tx, workspace).await?;
    let workspace_version = observation.version;
    let mut record_versions: BTreeMap<String, String> = observation
        .sources
        .into_iter()
        .map(|src| (src.candidate_id, src.revision))
        .collect();
    let linked_convs = entities::spruce_conversation::Entity::find()
        .filter(entities::spruce_conversation::Column::Practice.eq(practice))
        .filter(entities::spruce_conversation::Column::LinkKind.eq("linked"))
        .filter(entities::spruce_conversation::Column::LinkedWorkspace.eq(workspace))
        .all(tx)
        .await?;
    let linked_conv_ids: Vec<String> = linked_convs
        .into_iter()
        .map(|c| c.conversation_id)
        .collect();
    if !linked_conv_ids.is_empty() {
        let linked_msgs = entities::spruce_message::Entity::find()
            .filter(entities::spruce_message::Column::Practice.eq(practice))
            .filter(entities::spruce_message::Column::ConversationId.is_in(linked_conv_ids))
            .filter(entities::spruce_message::Column::StandingKind.eq("linked"))
            .all(tx)
            .await?;
        for m in linked_msgs {
            record_versions.insert(format!("spruce:{}", m.message_id), m.content_version);
        }
    }

    let ws_ver_i64 =
        i64::try_from(workspace_version).map_err(conversion_error("workspace_version", "i64"))?;
    let origin_json = serde_json::to_value(&origin)
        .map_err(conversion_error("SpruceReviewTriggerOrigin", "json"))?;
    let record_versions_json = serde_json::to_value(&record_versions)
        .map_err(conversion_error("BTreeMap<String, String>", "json"))?;
    entities::spruce_review_trigger::Entity::insert(entities::spruce_review_trigger::ActiveModel {
        practice: Set(practice.to_owned()),
        command_key: Set(command_key.to_owned()),
        conversation_id: Set(conversation_id.to_owned()),
        workspace: Set(workspace.to_owned()),
        origin: Set(origin_json),
        workspace_version: Set(ws_ver_i64),
        record_versions: Set(record_versions_json),
        dispatched: Set(false),
        created_at: Set(now_ts),
        dispatched_at: Set(None),
    })
    .on_conflict(
        OnConflict::columns([
            entities::spruce_review_trigger::Column::Practice,
            entities::spruce_review_trigger::Column::CommandKey,
        ])
        .do_nothing()
        .to_owned(),
    )
    .exec_without_returning(tx)
    .await?;

    let saved = entities::spruce_review_trigger::Entity::find_by_id((
        practice.to_owned(),
        command_key.to_owned(),
    ))
    .one(tx)
    .await?
    .ok_or_else(|| error("missing spruce review trigger"))?;
    decode_trigger_row(saved)
}

pub(crate) async fn dispatch_single_trigger(
    store: &Store,
    trigger: &PendingSpruceReviewTrigger,
    review_receiver: &dyn ReviewReceiver,
    now_ts: i64,
) -> Result<(), SpruceMatchingError> {
    if trigger.dispatched {
        return Ok(());
    }
    let request = ReviewRequest {
        practice_id: trigger.practice.clone(),
        patient_workspace_id: trigger.workspace.clone(),
        command_key: trigger.command_key.clone(),
        reason: ReviewReason::NewRecords,
        workspace_version: trigger.workspace_version,
        record_versions: trigger.record_versions.clone(),
    };
    review_receiver
        .submit_review(&request)
        .await
        .map_err(|source| SpruceMatchingError::ReviewHandoff {
            workspace: trigger.workspace.clone(),
            source,
        })?;

    let tx = store.db.begin().await?;
    if let Some(row) = entities::spruce_review_trigger::Entity::find_by_id((
        trigger.practice.clone(),
        trigger.command_key.clone(),
    ))
    .lock_exclusive()
    .one(&tx)
    .await?
        && !row.dispatched
    {
        let mut active: entities::spruce_review_trigger::ActiveModel = row.into();
        active.dispatched = Set(true);
        active.dispatched_at = Set(Some(now_ts));
        active.update(&tx).await?;
    }
    tx.commit().await?;
    Ok(())
}

/// Single rule owner implementing `Join(i, p)` (`MessageMatching.tla` line 162):
/// links `conv` to `workspace`, links `target_message_id` **and** every sibling
/// `Waiting` message in `conv` to `workspace`, latches `ever_linked = true` on
/// each joined message, and clears `conv.active_suggestion`.
async fn join_conversation_to_patient_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    conv: &mut SavedSpruceConversation,
    target_msg: &mut SavedSpruceMessage,
    workspace: &str,
    origin: LinkOrigin,
    now_ts: i64,
) -> Result<Vec<String>, DbErr> {
    conv.link_state = ConversationLinkState::Linked {
        workspace: workspace.to_owned(),
        linked_at: now_ts,
    };
    conv.active_suggestion = None;
    conv.updated_at = now_ts;
    save_conversation_in_tx(tx, conv).await?;

    target_msg.standing = MessageStanding::Linked {
        linked_at: now_ts,
        origin: origin.clone(),
    };
    target_msg.ever_linked = true;
    target_msg.updated_at = now_ts;
    save_message_in_tx(tx, target_msg).await?;

    let mut joined_ids = vec![target_msg.message_id.clone()];
    let sibling_rows = entities::spruce_message::Entity::find()
        .filter(entities::spruce_message::Column::Practice.eq(&conv.practice))
        .filter(entities::spruce_message::Column::ConversationId.eq(&conv.conversation_id))
        .order_by_asc(entities::spruce_message::Column::ChannelPosition)
        .lock_exclusive()
        .all(tx)
        .await?;
    for row in sibling_rows {
        if row.message_id == target_msg.message_id {
            continue;
        }
        let mut sibling = decode_message_row(row)?;
        if matches!(sibling.standing, MessageStanding::Waiting { .. }) {
            sibling.standing = MessageStanding::Linked {
                linked_at: now_ts,
                origin: origin.clone(),
            };
            sibling.ever_linked = true;
            sibling.updated_at = now_ts;
            joined_ids.push(sibling.message_id.clone());
            save_message_in_tx(tx, &sibling).await?;
        }
    }
    Ok(joined_ids)
}

/// Single rule owner implementing `Route(i, name, who)` (`MessageMatching.tla` line 180)
/// shared across `CodaDecides`, `FifteenMinutesPass`, and `"This is a patient"`.
#[expect(
    clippy::too_many_arguments,
    reason = "single routing owner takes explicit locked conversation, message, authority sets, and timestamp"
)]
async fn route_patient_message_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    conv: &mut SavedSpruceConversation,
    msg: &mut SavedSpruceMessage,
    routing_origin: RoutingOrigin,
    model_assessment: Option<&MatchAssessment>,
    registered_workspaces: &BTreeSet<String>,
    allowed_workspaces_with_physician: &BTreeSet<String>,
    now_ts: i64,
) -> Result<(MessageRoutingOutcome, Option<PendingSpruceReviewTrigger>), DbErr> {
    match conv.link_state.clone() {
        ConversationLinkState::Linked { workspace, .. } => {
            let origin = LinkOrigin::JoinedExistingConversationLink;
            msg.standing = MessageStanding::Linked {
                linked_at: now_ts,
                origin: origin.clone(),
            };
            msg.ever_linked = true;
            msg.updated_at = now_ts;
            save_message_in_tx(tx, msg).await?;

            let trigger = if !conv
                .move_undo
                .as_ref()
                .is_some_and(|u| u.is_active_at(now_ts))
            {
                let cmd_key = format!(
                    "spruce-arrival:{}:{}:{}:{}",
                    conv.practice, conv.conversation_id, msg.message_id, workspace
                );
                Some(
                    queue_review_trigger_in_tx(
                        tx,
                        &conv.practice,
                        &conv.conversation_id,
                        &workspace,
                        &cmd_key,
                        SpruceReviewTriggerOrigin::LinkedConversationArrival,
                        now_ts,
                    )
                    .await?,
                )
            } else {
                None
            };
            let review_command_key = trigger.as_ref().map(|t| t.command_key.clone());
            Ok((
                MessageRoutingOutcome::Linked {
                    workspace,
                    origin,
                    joined_message_ids: vec![msg.message_id.clone()],
                    review_command_key,
                },
                trigger,
            ))
        }
        ConversationLinkState::MovedNotAPatient { .. } => {
            msg.standing = MessageStanding::NotAPatient { moved_at: now_ts };
            msg.updated_at = now_ts;
            save_message_in_tx(tx, msg).await?;
            Ok((MessageRoutingOutcome::NotAPatient, None))
        }
        ConversationLinkState::Unlinked => {
            let confirmed_sender = confirmed_workspaces_for_conversation_in_tx(
                tx,
                &conv.practice,
                msg.sender_address.as_ref(),
                &conv.outside_participants,
                registered_workspaces,
            )
            .await?;
            let (contacts, ehr_map) =
                load_practice_channel_evidence_in_tx(tx, &conv.practice, registered_workspaces)
                    .await?;
            let channel_tie = evaluate_channel_tie(&conv.outside_participants, &contacts, &ehr_map);

            match evaluate_automatic_link(
                &confirmed_sender,
                &channel_tie,
                model_assessment,
                allowed_workspaces_with_physician,
            ) {
                AutomaticLinkEvaluation::LinkAutomatically { workspace } => {
                    let origin = LinkOrigin::AutomaticRule4;
                    let joined = join_conversation_to_patient_in_tx(
                        tx,
                        conv,
                        msg,
                        &workspace,
                        origin.clone(),
                        now_ts,
                    )
                    .await?;
                    let cmd_key = format!(
                        "spruce-autolink:{}:{}:{}:{}",
                        conv.practice, conv.conversation_id, msg.message_id, workspace
                    );
                    let trigger = queue_review_trigger_in_tx(
                        tx,
                        &conv.practice,
                        &conv.conversation_id,
                        &workspace,
                        &cmd_key,
                        SpruceReviewTriggerOrigin::AutomaticLink,
                        now_ts,
                    )
                    .await?;
                    let review_command_key = Some(trigger.command_key.clone());
                    Ok((
                        MessageRoutingOutcome::Linked {
                            workspace,
                            origin,
                            joined_message_ids: joined,
                            review_command_key,
                        },
                        Some(trigger),
                    ))
                }
                AutomaticLinkEvaluation::WaitForConfirmation => {
                    let already_waiting = entities::spruce_message::Entity::find()
                        .filter(entities::spruce_message::Column::Practice.eq(&conv.practice))
                        .filter(
                            entities::spruce_message::Column::ConversationId
                                .eq(&conv.conversation_id),
                        )
                        .filter(entities::spruce_message::Column::MessageId.ne(&msg.message_id))
                        .filter(entities::spruce_message::Column::StandingKind.eq("waiting"))
                        .one(tx)
                        .await?
                        .is_some();

                    if (!already_waiting || conv.active_suggestion.is_none())
                        && let Some(MatchAssessment::SuggestedPatient {
                            workspace,
                            evidence_summary,
                        }) = model_assessment
                        && registered_workspaces.contains(workspace)
                    {
                        conv.active_suggestion = Some(SuggestedPatientMatch {
                            workspace: workspace.clone(),
                            evidence_summary: evidence_summary.clone(),
                            suggested_at: now_ts,
                        });
                        conv.updated_at = now_ts;
                        save_conversation_in_tx(tx, conv).await?;
                    }

                    msg.standing = MessageStanding::Waiting {
                        waiting_since: now_ts,
                        origin: routing_origin,
                    };
                    msg.updated_at = now_ts;
                    save_message_in_tx(tx, msg).await?;

                    let suggested_workspace =
                        conv.active_suggestion.as_ref().map(|s| s.workspace.clone());
                    Ok((
                        MessageRoutingOutcome::Waiting {
                            suggested_workspace,
                        },
                        None,
                    ))
                }
            }
        }
    }
}

impl Store {
    /// Reads the practice's current [`ChannelStanding`] and optional reader lease.
    pub async fn spruce_channel_standing(
        &self,
        practice: &str,
    ) -> Result<(ChannelStanding, Option<ChannelReaderLease>), SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let row = entities::spruce_channel::Entity::find_by_id(practice.to_owned())
            .one(&self.db)
            .await?;
        match row {
            Some(model) => Ok(decode_channel_row(model)?),
            None => Ok((
                ChannelStanding::Disconnected {
                    disconnected_at: None,
                },
                None,
            )),
        }
    }

    /// Acquires the practice's exclusive Spruce channel reader lease when free
    /// or expired (`README.md`: *"Each practice's channel has one reader at a time"*).
    pub async fn acquire_spruce_reader_lease(
        &self,
        practice: &str,
        holder_id: &str,
        ttl_seconds: i64,
        now_ts: i64,
    ) -> Result<ChannelReaderLease, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let holder_id = require_id(holder_id, "holder_id")?;
        let ttl = ttl_seconds.max(1);

        let tx = self.db.begin().await?;
        let row = lock_channel_row(&tx, practice, now_ts).await?;
        let (standing, existing) = decode_channel_row(row)?;
        if let Some(active) = existing
            && active.is_active_at(now_ts)
            && active.holder_id != holder_id
        {
            return Err(SpruceMatchingError::ReaderLeaseHeld {
                practice: practice.to_owned(),
                holder_id: active.holder_id,
                expires_at: active.expires_at,
            });
        }
        let lease = ChannelReaderLease {
            holder_id: holder_id.to_owned(),
            lease_token: Uuid::new_v4().to_string(),
            acquired_at: now_ts,
            expires_at: now_ts.saturating_add(ttl),
        };
        save_channel_state_in_tx(&tx, practice, &standing, Some(&lease), now_ts).await?;
        tx.commit().await?;
        Ok(lease)
    }

    /// Releases a held reader lease when `lease_token` matches.
    pub async fn release_spruce_reader_lease(
        &self,
        practice: &str,
        lease_token: &str,
        now_ts: i64,
    ) -> Result<(), SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let lease_token = require_id(lease_token, "lease_token")?;
        let tx = self.db.begin().await?;
        let row = lock_channel_row(&tx, practice, now_ts).await?;
        let (standing, existing) = decode_channel_row(row)?;
        if existing
            .as_ref()
            .is_some_and(|l| l.lease_token == lease_token)
        {
            save_channel_state_in_tx(&tx, practice, &standing, None, now_ts).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Connects a disconnected practice Spruce channel (`YouConnect` in `MessageMatching.tla`):
    /// sets `place = channel_head_position` and `start_boundary = channel_head_position`,
    /// offering one-time backlog review when `channel_head_position > 0` (`Rule 14`).
    pub async fn connect_spruce_channel(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        channel_head_position: u64,
        spruce_cursor: Option<String>,
        now_ts: i64,
    ) -> Result<ChannelStanding, SpruceMatchingError> {
        require_active_physician(auth, actor).await?;
        let practice = actor.practice_id.as_str();
        let tx = self.db.begin().await?;
        let row = lock_channel_row(&tx, practice, now_ts).await?;
        let (current, lease) = decode_channel_row(row)?;
        if !matches!(current, ChannelStanding::Disconnected { .. }) {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: current.kind_str().to_owned(),
                action: "connect",
            });
        }
        let backlog = if channel_head_position > 0 {
            BacklogStanding::Offered { offered_at: now_ts }
        } else {
            BacklogStanding::NoOffer
        };
        let connected = ChannelStanding::Connected {
            place: channel_head_position,
            start_boundary: channel_head_position,
            spruce_cursor,
            backlog,
            health: ChannelHealth::Reading {
                last_read_at: Some(now_ts),
            },
        };
        save_channel_state_in_tx(&tx, practice, &connected, lease.as_ref(), now_ts).await?;
        tx.commit().await?;
        Ok(connected)
    }

    /// Accepts the one-time offer to read messages from before Coda connected (`YouAcceptBacklog`, `Rule 14`).
    pub async fn accept_spruce_backlog(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        now_ts: i64,
    ) -> Result<ChannelStanding, SpruceMatchingError> {
        require_active_physician(auth, actor).await?;
        let practice = actor.practice_id.as_str();
        let tx = self.db.begin().await?;
        let row = lock_channel_row(&tx, practice, now_ts).await?;
        let (current, lease) = decode_channel_row(row)?;
        let ChannelStanding::Connected {
            place,
            start_boundary,
            spruce_cursor,
            backlog,
            health,
        } = current
        else {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: "disconnected".to_owned(),
                action: "accept_backlog",
            });
        };
        if !matches!(backlog, BacklogStanding::Offered { .. }) {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: "backlog_not_offered".to_owned(),
                action: "accept_backlog",
            });
        }
        let updated = ChannelStanding::Connected {
            place,
            start_boundary,
            spruce_cursor,
            backlog: BacklogStanding::Accepted {
                accepted_at: now_ts,
                accepted_by: actor.physician_id.clone(),
            },
            health,
        };
        save_channel_state_in_tx(&tx, practice, &updated, lease.as_ref(), now_ts).await?;
        tx.commit().await?;
        Ok(updated)
    }

    /// Records a transient read failure (`ChannelStops(Failing)`) or revoked/blocked
    /// credential (`ChannelStops(Revoked)`) on the practice channel (`Rule 10`).
    pub async fn record_spruce_channel_failure(
        &self,
        practice: &str,
        lease_token: &str,
        cause: ChannelStopCause,
        now_ts: i64,
    ) -> Result<ChannelStanding, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let lease_token = require_id(lease_token, "lease_token")?;
        let raw_reason = match &cause {
            ChannelStopCause::Transient { reason }
            | ChannelStopCause::RevokedOrBlocked { reason } => reason.as_str(),
        };
        let reason = require_reason_text(raw_reason, "reason")?.to_owned();

        let tx = self.db.begin().await?;
        let row = lock_channel_row(&tx, practice, now_ts).await?;
        let (current, lease) = decode_channel_row(row)?;
        let Some(active_lease) = lease
            .as_ref()
            .filter(|l| l.is_active_at(now_ts) && l.lease_token == lease_token)
        else {
            return Err(SpruceMatchingError::InvalidReaderLease {
                practice: practice.to_owned(),
            });
        };
        let ChannelStanding::Connected {
            place,
            start_boundary,
            spruce_cursor,
            backlog,
            health,
        } = current
        else {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: "disconnected".to_owned(),
                action: "record_channel_failure",
            });
        };
        let previous_last_read = match health {
            ChannelHealth::Reading { last_read_at }
            | ChannelHealth::TransientFailure { last_read_at, .. }
            | ChannelHealth::RevokedOrBlocked { last_read_at, .. } => last_read_at,
        };
        let next_health = match cause {
            ChannelStopCause::RevokedOrBlocked { .. } => ChannelHealth::RevokedOrBlocked {
                reason,
                failed_at: now_ts,
                last_read_at: previous_last_read,
            },
            ChannelStopCause::Transient { .. } => ChannelHealth::TransientFailure {
                reason,
                failed_at: now_ts,
                last_read_at: previous_last_read,
            },
        };
        let updated = ChannelStanding::Connected {
            place,
            start_boundary,
            spruce_cursor,
            backlog,
            health: next_health,
        };
        save_channel_state_in_tx(&tx, practice, &updated, Some(active_lease), now_ts).await?;
        tx.commit().await?;
        Ok(updated)
    }

    /// Clears a transient channel failure when Coda can read the channel again (`ChannelComesBack`, `Rule 10`).
    pub async fn recover_spruce_channel_reading(
        &self,
        practice: &str,
        lease_token: &str,
        now_ts: i64,
    ) -> Result<ChannelStanding, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let lease_token = require_id(lease_token, "lease_token")?;
        let tx = self.db.begin().await?;
        let row = lock_channel_row(&tx, practice, now_ts).await?;
        let (current, lease) = decode_channel_row(row)?;
        let Some(active_lease) = lease
            .as_ref()
            .filter(|l| l.is_active_at(now_ts) && l.lease_token == lease_token)
        else {
            return Err(SpruceMatchingError::InvalidReaderLease {
                practice: practice.to_owned(),
            });
        };
        let ChannelStanding::Connected {
            place,
            start_boundary,
            spruce_cursor,
            backlog,
            health,
        } = current
        else {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: "disconnected".to_owned(),
                action: "recover_channel_reading",
            });
        };
        if matches!(health, ChannelHealth::RevokedOrBlocked { .. }) {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: "revoked_or_blocked".to_owned(),
                action: "recover_channel_reading",
            });
        }
        let updated = ChannelStanding::Connected {
            place,
            start_boundary,
            spruce_cursor,
            backlog,
            health: ChannelHealth::Reading {
                last_read_at: Some(now_ts),
            },
        };
        save_channel_state_in_tx(&tx, practice, &updated, Some(active_lease), now_ts).await?;
        tx.commit().await?;
        Ok(updated)
    }

    /// Reconnects a revoked or blocked channel (`YouReconnect` in `MessageMatching.tla`),
    /// resuming from Coda's saved `place` without losing or duplicating kept messages (`Rule 10` & `Rule 12`).
    pub async fn reconnect_spruce_channel(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        now_ts: i64,
    ) -> Result<ChannelStanding, SpruceMatchingError> {
        require_active_physician(auth, actor).await?;
        let practice = actor.practice_id.as_str();
        let tx = self.db.begin().await?;
        let row = lock_channel_row(&tx, practice, now_ts).await?;
        let (current, lease) = decode_channel_row(row)?;
        let ChannelStanding::Connected {
            place,
            start_boundary,
            spruce_cursor,
            backlog,
            health,
        } = current
        else {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: "disconnected".to_owned(),
                action: "reconnect",
            });
        };
        if !matches!(health, ChannelHealth::RevokedOrBlocked { .. }) {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: "not_revoked_or_blocked".to_owned(),
                action: "reconnect",
            });
        }
        let updated = ChannelStanding::Connected {
            place,
            start_boundary,
            spruce_cursor,
            backlog,
            health: ChannelHealth::Reading {
                last_read_at: Some(now_ts),
            },
        };
        save_channel_state_in_tx(&tx, practice, &updated, lease.as_ref(), now_ts).await?;
        tx.commit().await?;
        Ok(updated)
    }

    /// Disconnects the practice's Spruce channel (`YouDisconnect`, `Rule 11`):
    /// deletes **every** message in `practice` that was never linked to a patient
    /// (`!ever_linked`, along with its matching runs), clears waiting suggestions,
    /// retains every message ever linked (`ever_linked == true`) and entered-in-error
    /// records under record policy, and sets Today's channel banner to `Quiet`.
    pub async fn disconnect_spruce_channel(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        now_ts: i64,
    ) -> Result<u64, SpruceMatchingError> {
        require_active_physician(auth, actor).await?;
        let practice = actor.practice_id.as_str();
        let tx = self.db.begin().await?;
        let row = lock_channel_row(&tx, practice, now_ts).await?;
        let (current, _) = decode_channel_row(row)?;
        if matches!(current, ChannelStanding::Disconnected { .. }) {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: "disconnected".to_owned(),
                action: "disconnect",
            });
        }

        let never_linked_rows = entities::spruce_message::Entity::find()
            .filter(entities::spruce_message::Column::Practice.eq(practice))
            .filter(entities::spruce_message::Column::EverLinked.eq(false))
            .lock_exclusive()
            .all(&tx)
            .await?;
        let never_linked_ids: Vec<String> = never_linked_rows
            .into_iter()
            .map(|m| m.message_id)
            .collect();
        let deleted_count = never_linked_ids.len() as u64;

        if !never_linked_ids.is_empty() {
            entities::spruce_matching_run::Entity::delete_many()
                .filter(entities::spruce_matching_run::Column::Practice.eq(practice))
                .filter(
                    entities::spruce_matching_run::Column::MessageId
                        .is_in(never_linked_ids.iter().cloned()),
                )
                .exec(&tx)
                .await?;
            entities::spruce_message::Entity::delete_many()
                .filter(entities::spruce_message::Column::Practice.eq(practice))
                .filter(entities::spruce_message::Column::EverLinked.eq(false))
                .exec(&tx)
                .await?;
        }

        let conv_rows = entities::spruce_conversation::Entity::find()
            .filter(entities::spruce_conversation::Column::Practice.eq(practice))
            .lock_exclusive()
            .all(&tx)
            .await?;
        for conv_row in conv_rows {
            let mut conv = decode_conversation_row(conv_row)?;
            if conv.active_suggestion.is_some()
                || (matches!(conv.link_state, ConversationLinkState::Unlinked)
                    && !conv.open_viewers.is_empty())
            {
                conv.active_suggestion = None;
                if matches!(conv.link_state, ConversationLinkState::Unlinked) {
                    conv.open_viewers.clear();
                }
                conv.updated_at = now_ts;
                save_conversation_in_tx(&tx, &conv).await?;
            }
        }

        let disconnected = ChannelStanding::Disconnected {
            disconnected_at: Some(now_ts),
        };
        save_channel_state_in_tx(&tx, practice, &disconnected, None, now_ts).await?;
        tx.commit().await?;
        Ok(deleted_count)
    }

    /// Saves or updates a Spruce contact record mapping `contact_id` to Spruce patients and EHR links.
    pub async fn save_spruce_contact_record(
        &self,
        record: &SpruceContactRecord,
        now_ts: i64,
    ) -> Result<(), SpruceMatchingError> {
        let practice = require_id(&record.practice, "practice")?;
        let contact_id = require_id(&record.contact_id, "contact_id")?;
        let patients_json = serde_json::to_value(&record.spruce_patients)
            .map_err(conversion_error("Vec<SpruceContactPatient>", "json"))?;
        entities::spruce_contact::Entity::insert(entities::spruce_contact::ActiveModel {
            practice: Set(practice.to_owned()),
            contact_id: Set(contact_id.to_owned()),
            spruce_patients: Set(patients_json.clone()),
            updated_at: Set(now_ts),
        })
        .on_conflict(
            OnConflict::columns([
                entities::spruce_contact::Column::Practice,
                entities::spruce_contact::Column::ContactId,
            ])
            .update_columns([
                entities::spruce_contact::Column::SprucePatients,
                entities::spruce_contact::Column::UpdatedAt,
            ])
            .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;
        Ok(())
    }

    /// Registers an EHR patient link `(source_id, source_patient_id) -> workspace` in `practice`.
    pub async fn register_spruce_ehr_patient_link(
        &self,
        practice: &str,
        source_id: &str,
        source_patient_id: &str,
        workspace: &str,
        now_ts: i64,
    ) -> Result<(), SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let source_id = require_id(source_id, "source_id")?;
        let source_patient_id = require_id(source_patient_id, "source_patient_id")?;
        let workspace = require_id(workspace, "workspace")?;
        entities::spruce_ehr_link::Entity::insert(entities::spruce_ehr_link::ActiveModel {
            practice: Set(practice.to_owned()),
            source_id: Set(source_id.to_owned()),
            source_patient_id: Set(source_patient_id.to_owned()),
            workspace: Set(workspace.to_owned()),
            recorded_at: Set(now_ts),
        })
        .on_conflict(
            OnConflict::columns([
                entities::spruce_ehr_link::Column::Practice,
                entities::spruce_ehr_link::Column::SourceId,
                entities::spruce_ehr_link::Column::SourcePatientId,
                entities::spruce_ehr_link::Column::Workspace,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;
        Ok(())
    }

    /// Records a sender address as confirmed for `workspace` in `practice`.
    pub async fn record_spruce_confirmed_sender(
        &self,
        practice: &str,
        sender: &SenderAddress,
        workspace: &str,
        confirmed_by: &str,
        now_ts: i64,
    ) -> Result<(), SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let workspace = require_id(workspace, "workspace")?;
        let confirmed_by = require_id(confirmed_by, "confirmed_by")?;
        entities::spruce_confirmed_sender::Entity::insert(
            entities::spruce_confirmed_sender::ActiveModel {
                practice: Set(practice.to_owned()),
                sender_key: Set(sender.as_str().to_owned()),
                workspace: Set(workspace.to_owned()),
                confirmed_by: Set(confirmed_by.to_owned()),
                confirmed_at: Set(now_ts),
            },
        )
        .on_conflict(
            OnConflict::columns([
                entities::spruce_confirmed_sender::Column::Practice,
                entities::spruce_confirmed_sender::Column::SenderKey,
                entities::spruce_confirmed_sender::Column::Workspace,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;
        Ok(())
    }

    /// Revokes a sender address confirmation for `workspace` in `practice`.
    pub async fn revoke_spruce_confirmed_sender(
        &self,
        practice: &str,
        sender: &SenderAddress,
        workspace: &str,
    ) -> Result<(), SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let workspace = require_id(workspace, "workspace")?;
        entities::spruce_confirmed_sender::Entity::delete_by_id((
            practice.to_owned(),
            sender.as_str().to_owned(),
            workspace.to_owned(),
        ))
        .exec(&self.db)
        .await?;
        Ok(())
    }

    /// Ingests one delivered Spruce item under the active channel reader lease
    /// (`CodaReads` & `CodaReadsItNow` in `MessageMatching.tla`, `Rules 12, 13, 14`).
    pub async fn ingest_spruce_item(
        &self,
        practice: &str,
        lease_token: &str,
        item: &IncomingSpruceItem,
        now_ts: i64,
    ) -> Result<SpruceIngestOutcome, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let lease_token = require_id(lease_token, "lease_token")?;
        let message_id = require_id(item.message_id(), "message_id")?;
        if item.channel_position() == 0 {
            return Err(SpruceMatchingError::InvalidIdentifier {
                field: "channel_position",
            });
        }

        let tx = self.db.begin().await?;
        let ch_row = lock_channel_row(&tx, practice, now_ts).await?;
        let (current_channel, lease) = decode_channel_row(ch_row)?;
        let Some(active_lease) = lease
            .as_ref()
            .filter(|l| l.is_active_at(now_ts) && l.lease_token == lease_token)
        else {
            return Err(SpruceMatchingError::InvalidReaderLease {
                practice: practice.to_owned(),
            });
        };

        let ChannelStanding::Connected {
            place,
            start_boundary,
            spruce_cursor,
            backlog,
            health: ChannelHealth::Reading { .. },
        } = &current_channel
        else {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: current_channel.kind_str().to_owned(),
                action: "ingest_item",
            });
        };

        let pos = item.channel_position();
        let allowed_by_cursor = pos > *place
            || (matches!(backlog, BacklogStanding::Accepted { .. }) && pos <= *start_boundary);

        if let Some(existing_row) = entities::spruce_message::Entity::find_by_id((
            practice.to_owned(),
            message_id.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        {
            let mut existing = decode_message_row(existing_row)?;
            if matches!(existing.standing, MessageStanding::CantRead { .. })
                && let IncomingSpruceItem::Readable(readable) = item
            {
                let cid = require_id(&readable.conversation_id, "conversation_id")?;
                let ver = require_id(&readable.content_version, "content_version")?;
                let previous_cid = existing.conversation_id.clone();

                ensure_conversation_row(&tx, practice, cid, &readable.outside_participants, now_ts)
                    .await?;
                if !readable.outside_participants.is_empty()
                    && let Some(mut conv) = lock_conversation_in_tx(&tx, practice, cid).await?
                {
                    conv.outside_participants = readable.outside_participants.clone();
                    conv.updated_at = now_ts;
                    save_conversation_in_tx(&tx, &conv).await?;
                }

                let recovered_standing = MessageStanding::Deciding {
                    kept_at: now_ts,
                    deadline_at: now_ts.saturating_add(FIFTEEN_MINUTES_SECONDS),
                };
                existing.conversation_id = cid.to_owned();
                existing.content_version = ver.to_owned();
                existing.standing = recovered_standing.clone();
                existing.sender_address = readable.sender_address.clone();
                existing.author_name = readable.author_name.clone();
                existing.body_text = readable.body_text.clone();
                existing.attachments = readable.attachments.clone();
                existing.app_url = readable.app_url.clone();
                existing.created_at_spruce = readable.created_at_spruce.clone();
                existing.updated_at = now_ts;
                save_message_in_tx(&tx, &existing).await?;

                if previous_cid != cid
                    && let Some(old_conv) =
                        lock_conversation_in_tx(&tx, practice, &previous_cid).await?
                    && matches!(old_conv.link_state, ConversationLinkState::Unlinked)
                {
                    let remaining_on_old = entities::spruce_message::Entity::find()
                        .filter(entities::spruce_message::Column::Practice.eq(practice))
                        .filter(entities::spruce_message::Column::ConversationId.eq(&previous_cid))
                        .one(&tx)
                        .await?
                        .is_some();
                    if !remaining_on_old {
                        entities::spruce_conversation::Entity::delete_by_id((
                            practice.to_owned(),
                            previous_cid,
                        ))
                        .exec(&tx)
                        .await?;
                    }
                }

                tx.commit().await?;
                return Ok(SpruceIngestOutcome::RecoveredUnreadable {
                    standing: recovered_standing,
                });
            }
            tx.commit().await?;
            return Ok(SpruceIngestOutcome::AlreadyKept {
                standing: existing.standing,
            });
        }

        if !allowed_by_cursor {
            tx.commit().await?;
            return Ok(SpruceIngestOutcome::SkippedPreConnectionBacklog);
        }

        let (
            conversation_id,
            content_version,
            standing,
            sender_address,
            author_name,
            body_text,
            attachments,
            app_url,
            created_at_spruce,
            outside_participants,
        ) = match item {
            IncomingSpruceItem::Readable(r) => {
                let cid = require_id(&r.conversation_id, "conversation_id")?;
                let ver = require_id(&r.content_version, "content_version")?;
                (
                    cid.to_owned(),
                    ver.to_owned(),
                    MessageStanding::Deciding {
                        kept_at: now_ts,
                        deadline_at: now_ts.saturating_add(FIFTEEN_MINUTES_SECONDS),
                    },
                    r.sender_address.clone(),
                    r.author_name.clone(),
                    r.body_text.clone(),
                    r.attachments.clone(),
                    r.app_url.clone(),
                    r.created_at_spruce.clone(),
                    r.outside_participants.clone(),
                )
            }
            IncomingSpruceItem::Unreadable(u) => {
                let cid = require_id(&u.conversation_id, "conversation_id")?;
                let ver = require_id(&u.content_version, "content_version")?;
                let reason = require_reason_text(&u.failure_reason, "failure_reason")?;
                (
                    cid.to_owned(),
                    ver.to_owned(),
                    MessageStanding::CantRead {
                        failure_reason: reason.to_owned(),
                        app_url: u.app_url.clone(),
                    },
                    None,
                    None,
                    String::new(),
                    Vec::new(),
                    u.app_url.clone(),
                    None,
                    u.outside_participants.clone(),
                )
            }
        };

        ensure_conversation_row(
            &tx,
            practice,
            &conversation_id,
            &outside_participants,
            now_ts,
        )
        .await?;
        if !outside_participants.is_empty()
            && let Some(mut conv) = lock_conversation_in_tx(&tx, practice, &conversation_id).await?
        {
            conv.outside_participants = outside_participants;
            conv.updated_at = now_ts;
            save_conversation_in_tx(&tx, &conv).await?;
        }

        let pos_i64 = i64::try_from(pos).map_err(conversion_error("channel_position", "i64"))?;
        let standing_json =
            serde_json::to_value(&standing).map_err(conversion_error("MessageStanding", "json"))?;
        let urgency = MessageUrgency::Ordinary;
        let urgency_json =
            serde_json::to_value(&urgency).map_err(conversion_error("MessageUrgency", "json"))?;
        let attachments_json = serde_json::to_value(&attachments)
            .map_err(conversion_error("Vec<SpruceAttachmentSummary>", "json"))?;

        entities::spruce_message::Entity::insert(entities::spruce_message::ActiveModel {
            practice: Set(practice.to_owned()),
            message_id: Set(message_id.to_owned()),
            conversation_id: Set(conversation_id),
            channel_position: Set(pos_i64),
            content_version: Set(content_version),
            standing_kind: Set(standing.kind_str().to_owned()),
            standing: Set(standing_json),
            urgency_kind: Set(urgency.kind_str().to_owned()),
            urgency: Set(urgency_json),
            ever_linked: Set(false),
            sender_address: Set(sender_address.as_ref().map(|a| a.as_str().to_owned())),
            author_name: Set(author_name),
            body_text: Set(body_text),
            attachments: Set(attachments_json),
            app_url: Set(app_url),
            created_at_spruce: Set(created_at_spruce),
            kept_at: Set(now_ts),
            saved_classification: Set(None),
            updated_at: Set(now_ts),
        })
        .exec_without_returning(&tx)
        .await?;

        let updated_channel = ChannelStanding::Connected {
            place: *place,
            start_boundary: *start_boundary,
            spruce_cursor: spruce_cursor.clone(),
            backlog: backlog.clone(),
            health: ChannelHealth::Reading {
                last_read_at: Some(now_ts),
            },
        };
        save_channel_state_in_tx(&tx, practice, &updated_channel, Some(active_lease), now_ts)
            .await?;
        tx.commit().await?;

        Ok(SpruceIngestOutcome::KeptNew { standing })
    }

    /// Ingests a batch of Spruce deliveries independently so one malformed or
    /// failing item never stops other conversations' messages.
    pub async fn ingest_spruce_batch(
        &self,
        practice: &str,
        lease_token: &str,
        items: &[IncomingSpruceItem],
        now_ts: i64,
    ) -> BatchStepReport<(String, SpruceIngestOutcome)> {
        let mut succeeded = Vec::new();
        let mut failures = Vec::new();
        for item in items {
            let id = item.message_id().to_owned();
            match self
                .ingest_spruce_item(practice, lease_token, item, now_ts)
                .await
            {
                Ok(outcome) => succeeded.push((id, outcome)),
                Err(error) => failures.push(PatientIsolationFailure {
                    target_id: id,
                    error,
                }),
            }
        }
        BatchStepReport {
            succeeded,
            failures,
        }
    }

    /// Advances the practice channel's `place` monotonically past contiguous
    /// durably kept message positions (`CodaMovesOn` in `MessageMatching.tla`, `Rule 12`).
    pub async fn advance_spruce_channel_place(
        &self,
        practice: &str,
        lease_token: &str,
        spruce_cursor: Option<String>,
        now_ts: i64,
    ) -> Result<u64, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let lease_token = require_id(lease_token, "lease_token")?;
        let tx = self.db.begin().await?;
        let row = lock_channel_row(&tx, practice, now_ts).await?;
        let (current, lease) = decode_channel_row(row)?;
        let Some(active_lease) = lease
            .as_ref()
            .filter(|l| l.is_active_at(now_ts) && l.lease_token == lease_token)
        else {
            return Err(SpruceMatchingError::InvalidReaderLease {
                practice: practice.to_owned(),
            });
        };
        let ChannelStanding::Connected {
            mut place,
            start_boundary,
            spruce_cursor: prev_cursor,
            backlog,
            health,
        } = current
        else {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: "disconnected".to_owned(),
                action: "advance_place",
            });
        };
        if !matches!(health, ChannelHealth::Reading { .. }) {
            return Err(SpruceMatchingError::ChannelStateRefused {
                practice: practice.to_owned(),
                state: "not_reading".to_owned(),
                action: "advance_place",
            });
        }

        let kept_positions: BTreeSet<u64> = entities::spruce_message::Entity::find()
            .filter(entities::spruce_message::Column::Practice.eq(practice))
            .filter(
                entities::spruce_message::Column::ChannelPosition
                    .gt(i64::try_from(place).unwrap_or(i64::MAX)),
            )
            .all(&tx)
            .await?
            .into_iter()
            .filter_map(|r| u64::try_from(r.channel_position).ok())
            .collect();
        while kept_positions.contains(&(place + 1)) {
            place += 1;
        }

        let updated = ChannelStanding::Connected {
            place,
            start_boundary,
            spruce_cursor: spruce_cursor.or(prev_cursor),
            backlog,
            health,
        };
        save_channel_state_in_tx(&tx, practice, &updated, Some(active_lease), now_ts).await?;
        tx.commit().await?;
        Ok(place)
    }

    /// Marks an unreadable (`CantRead`) message `Done` (`YouMarkDone`, `Rule 13`).
    pub async fn mark_unreadable_spruce_message_done(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        message_id: &str,
        command_key: &str,
        now_ts: i64,
    ) -> Result<SavedSpruceMessage, SpruceMatchingError> {
        require_active_physician(auth, actor).await?;
        let practice = actor.practice_id.as_str();
        let message_id = require_id(message_id, "message_id")?;
        let command_key = require_id(command_key, "command_key")?;
        let command_body = json!({
            "kind": "spruce_mark_unreadable_done",
            "practice": practice,
            "physician": actor.physician_id,
            "message_id": message_id,
        });

        let tx = self.db.begin().await?;
        match receipt::check(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?
        {
            Some(Receipt::Same) => {
                let row = entities::spruce_message::Entity::find_by_id((
                    practice.to_owned(),
                    message_id.to_owned(),
                ))
                .one(&tx)
                .await?
                .ok_or_else(|| SpruceMatchingError::NotFound {
                    practice: practice.to_owned(),
                    kind: "message",
                    id: message_id.to_owned(),
                })?;
                let decoded = decode_message_row(row)?;
                tx.commit().await?;
                return Ok(decoded);
            }
            Some(Receipt::Different) => {
                return Err(SpruceMatchingError::IdempotencyCollision {
                    key: command_key.to_owned(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let row = entities::spruce_message::Entity::find_by_id((
            practice.to_owned(),
            message_id.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        .ok_or_else(|| SpruceMatchingError::NotFound {
            practice: practice.to_owned(),
            kind: "message",
            id: message_id.to_owned(),
        })?;
        let mut msg = decode_message_row(row)?;
        let MessageStanding::CantRead {
            failure_reason,
            app_url,
        } = msg.standing
        else {
            return Err(SpruceMatchingError::InvalidTransition {
                id: message_id.to_owned(),
                current_state: msg.standing.kind_str().to_owned(),
                action: "mark_done",
            });
        };
        msg.standing = MessageStanding::MarkedDone {
            failure_reason,
            app_url,
            marked_by: actor.physician_id.clone(),
            marked_at: now_ts,
        };
        msg.updated_at = now_ts;
        save_message_in_tx(&tx, &msg).await?;
        let _ = receipt::record(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?;
        tx.commit().await?;
        Ok(msg)
    }

    /// Persists Coda's classification and match assessment on a `Deciding` message
    /// (`CodaDecides` / `CodaFilters` in `MessageMatching.tla`) through explicit
    /// typed operations, routing patient mail via `Rule 4` and dispatching any
    /// resulting review request once.
    #[expect(
        clippy::too_many_arguments,
        reason = "persists classification provenance, decision, authority sets, and optional review receiver together"
    )]
    pub async fn record_spruce_message_classification(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
        message_id: &str,
        run_id: Option<&str>,
        recipe_version: &str,
        guidance_version: &str,
        decision: MatchingDecision,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<MessageRoutingOutcome, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let message_id = require_id(message_id, "message_id")?;
        let recipe_version = require_id(recipe_version, "recipe_version")?;
        let guidance_version = require_id(guidance_version, "guidance_version")?;

        let registered: BTreeSet<String> = auth
            .practice_patients(practice)
            .await?
            .into_iter()
            .collect();
        let allowed = allowed_workspaces_in_practice(auth, practice).await?;

        let tx = self.db.begin().await?;
        let (outcome, trigger) = self
            .apply_classification_in_tx(
                &tx,
                practice,
                message_id,
                run_id,
                recipe_version,
                guidance_version,
                decision,
                &registered,
                &allowed,
                now_ts,
            )
            .await?;
        tx.commit().await?;

        if let (Some(trig), Some(receiver)) = (trigger, review_receiver) {
            dispatch_single_trigger(self, &trig, receiver, now_ts).await?;
        }
        Ok(outcome)
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "shared transactional helper for live harness commit and direct classification persistence"
    )]
    pub(crate) async fn apply_classification_in_tx(
        &self,
        tx: &sea_orm::DatabaseTransaction,
        practice: &str,
        message_id: &str,
        run_id: Option<&str>,
        recipe_version: &str,
        guidance_version: &str,
        decision: MatchingDecision,
        registered_workspaces: &BTreeSet<String>,
        allowed_workspaces_with_physician: &BTreeSet<String>,
        now_ts: i64,
    ) -> Result<(MessageRoutingOutcome, Option<PendingSpruceReviewTrigger>), SpruceMatchingError>
    {
        let msg_row = entities::spruce_message::Entity::find_by_id((
            practice.to_owned(),
            message_id.to_owned(),
        ))
        .lock_exclusive()
        .one(tx)
        .await?
        .ok_or_else(|| SpruceMatchingError::NotFound {
            practice: practice.to_owned(),
            kind: "message",
            id: message_id.to_owned(),
        })?;
        let mut msg = decode_message_row(msg_row)?;
        let mut conv = lock_conversation_in_tx(tx, practice, &msg.conversation_id)
            .await?
            .ok_or_else(|| SpruceMatchingError::NotFound {
                practice: practice.to_owned(),
                kind: "conversation",
                id: msg.conversation_id.clone(),
            })?;

        if !matches!(msg.standing, MessageStanding::Deciding { .. }) {
            return Err(SpruceMatchingError::InvalidTransition {
                id: message_id.to_owned(),
                current_state: msg.standing.kind_str().to_owned(),
                action: "record_classification",
            });
        }

        msg.saved_classification = Some(SavedMessageClassification {
            run_id: run_id.map(str::to_owned),
            recipe_version: recipe_version.to_owned(),
            guidance_version: guidance_version.to_owned(),
            decision: decision.clone(),
            decided_at: now_ts,
            physician_override: None,
        });

        match decision {
            MatchingDecision::NotPatientMail {
                category,
                rationale,
            } => {
                msg.standing = MessageStanding::Filtered {
                    category,
                    rationale,
                    filtered_at: now_ts,
                };
                msg.updated_at = now_ts;
                save_message_in_tx(tx, &msg).await?;
                Ok((MessageRoutingOutcome::Filtered { category }, None))
            }
            MatchingDecision::PatientMail {
                urgency,
                assessment,
                ..
            } => {
                msg.urgency = match urgency {
                    MessageUrgencyChoice::Ordinary => MessageUrgency::Ordinary,
                    MessageUrgencyChoice::Urgent { reason } => MessageUrgency::Urgent {
                        reason,
                        flagged_at: now_ts,
                    },
                };
                let (outcome, trigger) = route_patient_message_in_tx(
                    tx,
                    &mut conv,
                    &mut msg,
                    RoutingOrigin::ModelClassification,
                    Some(&assessment),
                    registered_workspaces,
                    allowed_workspaces_with_physician,
                    now_ts,
                )
                .await?;
                Ok((outcome, trigger))
            }
        }
    }

    /// Marks a `Filtered` message `"This is a patient"` (`ThisIsAPatient` in
    /// `MessageMatching.tla`), returning it to conversation routing.
    pub async fn mark_spruce_message_as_patient(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        message_id: &str,
        command_key: &str,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<MessageRoutingOutcome, SpruceMatchingError> {
        require_active_physician(auth, actor).await?;
        let practice = actor.practice_id.as_str();
        let message_id = require_id(message_id, "message_id")?;
        let command_key = require_id(command_key, "command_key")?;
        let command_body = json!({
            "kind": "spruce_this_is_a_patient",
            "practice": practice,
            "physician": actor.physician_id,
            "message_id": message_id,
        });

        let registered: BTreeSet<String> = auth
            .practice_patients(practice)
            .await?
            .into_iter()
            .collect();
        let allowed = allowed_workspaces_in_practice(auth, practice).await?;

        let tx = self.db.begin().await?;
        match receipt::check(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?
        {
            Some(Receipt::Same) => {
                let row = entities::spruce_message::Entity::find_by_id((
                    practice.to_owned(),
                    message_id.to_owned(),
                ))
                .one(&tx)
                .await?
                .ok_or_else(|| SpruceMatchingError::NotFound {
                    practice: practice.to_owned(),
                    kind: "message",
                    id: message_id.to_owned(),
                })?;
                let msg = decode_message_row(row)?;
                let conv = lock_conversation_in_tx(&tx, practice, &msg.conversation_id)
                    .await?
                    .ok_or_else(|| SpruceMatchingError::NotFound {
                        practice: practice.to_owned(),
                        kind: "conversation",
                        id: msg.conversation_id.clone(),
                    })?;
                let (raw_outcome, replay_trigger) = match (&msg.standing, &conv.link_state) {
                    (
                        MessageStanding::Linked { origin, .. },
                        ConversationLinkState::Linked { workspace, .. },
                    ) => {
                        let expected_cmd_key = match origin {
                            LinkOrigin::AutomaticRule4 => format!(
                                "spruce-autolink:{}:{}:{}:{}",
                                practice, conv.conversation_id, msg.message_id, workspace
                            ),
                            _ => format!(
                                "spruce-arrival:{}:{}:{}:{}",
                                practice, conv.conversation_id, msg.message_id, workspace
                            ),
                        };
                        let trig = entities::spruce_review_trigger::Entity::find_by_id((
                            practice.to_owned(),
                            expected_cmd_key,
                        ))
                        .one(&tx)
                        .await?
                        .map(decode_trigger_row)
                        .transpose()?;
                        let joined_ids = if matches!(origin, LinkOrigin::AutomaticRule4) {
                            let linked_rows = entities::spruce_message::Entity::find()
                                .filter(entities::spruce_message::Column::Practice.eq(practice))
                                .filter(
                                    entities::spruce_message::Column::ConversationId
                                        .eq(&conv.conversation_id),
                                )
                                .filter(entities::spruce_message::Column::StandingKind.eq("linked"))
                                .order_by_asc(entities::spruce_message::Column::ChannelPosition)
                                .all(&tx)
                                .await?;
                            linked_rows.into_iter().map(|r| r.message_id).collect()
                        } else {
                            vec![msg.message_id]
                        };
                        let review_command_key = trig.as_ref().map(|t| t.command_key.clone());
                        (
                            MessageRoutingOutcome::Linked {
                                workspace: workspace.clone(),
                                origin: origin.clone(),
                                joined_message_ids: joined_ids,
                                review_command_key,
                            },
                            trig,
                        )
                    }
                    (MessageStanding::NotAPatient { .. }, _) => {
                        (MessageRoutingOutcome::NotAPatient, None)
                    }
                    _ => (
                        MessageRoutingOutcome::Waiting {
                            suggested_workspace: conv
                                .active_suggestion
                                .as_ref()
                                .map(|s| s.workspace.clone()),
                        },
                        None,
                    ),
                };
                tx.commit().await?;
                if let (Some(trig), Some(receiver)) = (replay_trigger, review_receiver) {
                    dispatch_single_trigger(self, &trig, receiver, now_ts).await?;
                }
                return Ok(project_routing_outcome_for_viewer(auth, actor, raw_outcome).await);
            }
            Some(Receipt::Different) => {
                return Err(SpruceMatchingError::IdempotencyCollision {
                    key: command_key.to_owned(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let msg_row = entities::spruce_message::Entity::find_by_id((
            practice.to_owned(),
            message_id.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        .ok_or_else(|| SpruceMatchingError::NotFound {
            practice: practice.to_owned(),
            kind: "message",
            id: message_id.to_owned(),
        })?;
        let mut msg = decode_message_row(msg_row)?;
        if !matches!(msg.standing, MessageStanding::Filtered { .. }) {
            return Err(SpruceMatchingError::InvalidTransition {
                id: message_id.to_owned(),
                current_state: msg.standing.kind_str().to_owned(),
                action: "this_is_a_patient",
            });
        }
        if let Some(saved) = &mut msg.saved_classification {
            saved.physician_override = Some(PhysicianPatientOverride {
                physician_id: actor.physician_id.clone(),
                overridden_at: now_ts,
            });
        }

        let mut conv = lock_conversation_in_tx(&tx, practice, &msg.conversation_id)
            .await?
            .ok_or_else(|| SpruceMatchingError::NotFound {
                practice: practice.to_owned(),
                kind: "conversation",
                id: msg.conversation_id.clone(),
            })?;

        let (outcome, trigger) = route_patient_message_in_tx(
            &tx,
            &mut conv,
            &mut msg,
            RoutingOrigin::PhysicianMarkedPatient {
                physician_id: actor.physician_id.clone(),
            },
            None,
            &registered,
            &allowed,
            now_ts,
        )
        .await?;
        let _ = receipt::record(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?;
        tx.commit().await?;

        if let (Some(trig), Some(receiver)) = (trigger, review_receiver) {
            dispatch_single_trigger(self, &trig, receiver, now_ts).await?;
        }
        Ok(project_routing_outcome_for_viewer(auth, actor, outcome).await)
    }

    /// Routes every `Deciding` message in `practice` whose 15-minute deadline
    /// (`kept_at + 900 <= now_ts`) has elapsed (`FifteenMinutesPass`, `Rule 6`),
    /// isolating per-message/per-patient failures so one failure never blocks others.
    pub async fn settle_undecided_spruce_messages(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<BatchStepReport<(String, MessageRoutingOutcome)>, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let registered: BTreeSet<String> = auth
            .practice_patients(practice)
            .await?
            .into_iter()
            .collect();
        let allowed = allowed_workspaces_in_practice(auth, practice).await?;

        let deciding_rows = entities::spruce_message::Entity::find()
            .filter(entities::spruce_message::Column::Practice.eq(practice))
            .filter(entities::spruce_message::Column::StandingKind.eq("deciding"))
            .order_by_asc(entities::spruce_message::Column::ChannelPosition)
            .all(&self.db)
            .await?;

        let mut succeeded = Vec::new();
        let mut failures = Vec::new();
        for row in deciding_rows {
            let message_id = row.message_id.clone();
            let decoded = match decode_message_row(row) {
                Ok(m) => m,
                Err(err) => {
                    failures.push(PatientIsolationFailure {
                        target_id: message_id,
                        error: SpruceMatchingError::Database(err),
                    });
                    continue;
                }
            };
            let MessageStanding::Deciding { deadline_at, .. } = decoded.standing else {
                continue;
            };
            if now_ts < deadline_at {
                continue;
            }

            let step = async {
                let tx = self.db.begin().await?;
                let locked_row = entities::spruce_message::Entity::find_by_id((
                    practice.to_owned(),
                    message_id.clone(),
                ))
                .lock_exclusive()
                .one(&tx)
                .await?
                .ok_or_else(|| SpruceMatchingError::NotFound {
                    practice: practice.to_owned(),
                    kind: "message",
                    id: message_id.clone(),
                })?;
                let mut msg = decode_message_row(locked_row)?;
                if !matches!(
                    msg.standing,
                    MessageStanding::Deciding { deadline_at, .. } if now_ts >= deadline_at
                ) {
                    tx.commit().await?;
                    return Ok(None);
                }
                let mut conv = lock_conversation_in_tx(&tx, practice, &msg.conversation_id)
                    .await?
                    .ok_or_else(|| SpruceMatchingError::NotFound {
                        practice: practice.to_owned(),
                        kind: "conversation",
                        id: msg.conversation_id.clone(),
                    })?;
                let (outcome, trigger) = route_patient_message_in_tx(
                    &tx,
                    &mut conv,
                    &mut msg,
                    RoutingOrigin::FifteenMinuteFallback,
                    None,
                    &registered,
                    &allowed,
                    now_ts,
                )
                .await?;
                tx.commit().await?;
                if let (Some(trig), Some(receiver)) = (trigger, review_receiver) {
                    dispatch_single_trigger(self, &trig, receiver, now_ts).await?;
                }
                Ok::<Option<MessageRoutingOutcome>, SpruceMatchingError>(Some(outcome))
            }
            .await;

            match step {
                Ok(Some(outcome)) => succeeded.push((message_id, outcome)),
                Ok(None) => {}
                Err(error) => failures.push(PatientIsolationFailure {
                    target_id: message_id,
                    error,
                }),
            }
        }
        Ok(BatchStepReport {
            succeeded,
            failures,
        })
    }

    /// Confirms a `Waiting` message onto `workspace` (`LinkIt` in `MessageMatching.tla`),
    /// enforcing [`Membership::Allowed`] (`Rule 9`), linking the conversation and
    /// **all** sibling `Waiting` messages together, and creating **exactly one**
    /// patient review request across command replays.
    #[expect(
        clippy::too_many_arguments,
        reason = "physician-authorized confirmation takes explicit actor, message, target workspace, sender policy, idempotency key, and review receiver"
    )]
    pub async fn confirm_spruce_message_link(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        message_id: &str,
        workspace: &str,
        sender_policy: SenderConfirmationPolicy,
        command_key: &str,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<ConfirmSpruceLinkOutcome, SpruceMatchingError> {
        require_patient_access(auth, actor, workspace).await?;
        let practice = actor.practice_id.as_str();
        let message_id = require_id(message_id, "message_id")?;
        let workspace = require_id(workspace, "workspace")?;
        let command_key = require_id(command_key, "command_key")?;
        let review_cmd_key = format!("spruce-confirm:{practice}:{command_key}");

        let command_body = json!({
            "kind": "spruce_confirm_message_link",
            "practice": practice,
            "physician": actor.physician_id,
            "message_id": message_id,
            "workspace": workspace,
            "sender_policy": sender_policy,
        });

        let tx = self.db.begin().await?;
        match receipt::check(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?
        {
            Some(Receipt::Same) => {
                let msg_row = entities::spruce_message::Entity::find_by_id((
                    practice.to_owned(),
                    message_id.to_owned(),
                ))
                .one(&tx)
                .await?
                .ok_or_else(|| SpruceMatchingError::NotFound {
                    practice: practice.to_owned(),
                    kind: "message",
                    id: message_id.to_owned(),
                })?;
                let msg = decode_message_row(msg_row)?;
                let linked_rows = entities::spruce_message::Entity::find()
                    .filter(entities::spruce_message::Column::Practice.eq(practice))
                    .filter(
                        entities::spruce_message::Column::ConversationId.eq(&msg.conversation_id),
                    )
                    .filter(entities::spruce_message::Column::StandingKind.eq("linked"))
                    .order_by_asc(entities::spruce_message::Column::ChannelPosition)
                    .all(&tx)
                    .await?;
                let linked_ids: Vec<String> =
                    linked_rows.into_iter().map(|r| r.message_id).collect();
                let trigger_row = entities::spruce_review_trigger::Entity::find_by_id((
                    practice.to_owned(),
                    review_cmd_key.clone(),
                ))
                .one(&tx)
                .await?
                .map(decode_trigger_row)
                .transpose()?;
                tx.commit().await?;

                if let (Some(trig), Some(receiver)) = (trigger_row, review_receiver) {
                    dispatch_single_trigger(self, &trig, receiver, now_ts).await?;
                }
                return Ok(ConfirmSpruceLinkOutcome {
                    acceptance: LinkCommandAcceptance::Duplicate,
                    conversation_id: msg.conversation_id,
                    workspace: workspace.to_owned(),
                    linked_message_ids: linked_ids,
                    review_command_key: review_cmd_key,
                });
            }
            Some(Receipt::Different) => {
                return Err(SpruceMatchingError::IdempotencyCollision {
                    key: command_key.to_owned(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let msg_row = entities::spruce_message::Entity::find_by_id((
            practice.to_owned(),
            message_id.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        .ok_or_else(|| SpruceMatchingError::NotFound {
            practice: practice.to_owned(),
            kind: "message",
            id: message_id.to_owned(),
        })?;
        let mut msg = decode_message_row(msg_row)?;
        if !matches!(msg.standing, MessageStanding::Waiting { .. }) {
            return Err(SpruceMatchingError::InvalidTransition {
                id: message_id.to_owned(),
                current_state: msg.standing.kind_str().to_owned(),
                action: "confirm_link",
            });
        }
        let mut conv = lock_conversation_in_tx(&tx, practice, &msg.conversation_id)
            .await?
            .ok_or_else(|| SpruceMatchingError::NotFound {
                practice: practice.to_owned(),
                kind: "conversation",
                id: msg.conversation_id.clone(),
            })?;
        if !matches!(conv.link_state, ConversationLinkState::Unlinked) {
            return Err(SpruceMatchingError::InvalidTransition {
                id: conv.conversation_id,
                current_state: "already_linked".to_owned(),
                action: "confirm_link",
            });
        }

        let joined_ids = join_conversation_to_patient_in_tx(
            &tx,
            &mut conv,
            &mut msg,
            workspace,
            LinkOrigin::PhysicianConfirmed {
                physician_id: actor.physician_id.clone(),
            },
            now_ts,
        )
        .await?;

        if sender_policy == SenderConfirmationPolicy::ConfirmSenderForPatient
            && let Some(sender) = &msg.sender_address
        {
            entities::spruce_confirmed_sender::Entity::insert(
                entities::spruce_confirmed_sender::ActiveModel {
                    practice: Set(practice.to_owned()),
                    sender_key: Set(sender.as_str().to_owned()),
                    workspace: Set(workspace.to_owned()),
                    confirmed_by: Set(actor.physician_id.clone()),
                    confirmed_at: Set(now_ts),
                },
            )
            .on_conflict(
                OnConflict::columns([
                    entities::spruce_confirmed_sender::Column::Practice,
                    entities::spruce_confirmed_sender::Column::SenderKey,
                    entities::spruce_confirmed_sender::Column::Workspace,
                ])
                .do_nothing()
                .to_owned(),
            )
            .exec_without_returning(&tx)
            .await?;
        }

        let trigger = queue_review_trigger_in_tx(
            &tx,
            practice,
            &conv.conversation_id,
            workspace,
            &review_cmd_key,
            SpruceReviewTriggerOrigin::ConfirmedLink {
                physician_id: actor.physician_id.clone(),
            },
            now_ts,
        )
        .await?;
        let _ = receipt::record(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?;
        tx.commit().await?;

        if let Some(receiver) = review_receiver {
            dispatch_single_trigger(self, &trigger, receiver, now_ts).await?;
        }
        Ok(ConfirmSpruceLinkOutcome {
            acceptance: LinkCommandAcceptance::New,
            conversation_id: conv.conversation_id,
            workspace: workspace.to_owned(),
            linked_message_ids: joined_ids,
            review_command_key: review_cmd_key,
        })
    }

    /// Leaves a single `Waiting` message unlinked (`LeaveUnlinked` in `MessageMatching.tla`):
    /// marks **only** that message [`MessageStanding::LeftUnlinked`] so the conversation
    /// remains `Unlinked` and the next message in that conversation waits again.
    pub async fn leave_spruce_message_unlinked(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        message_id: &str,
        command_key: &str,
        now_ts: i64,
    ) -> Result<SavedSpruceMessage, SpruceMatchingError> {
        require_active_physician(auth, actor).await?;
        let practice = actor.practice_id.as_str();
        let message_id = require_id(message_id, "message_id")?;
        let command_key = require_id(command_key, "command_key")?;
        let command_body = json!({
            "kind": "spruce_leave_message_unlinked",
            "practice": practice,
            "physician": actor.physician_id,
            "message_id": message_id,
        });

        let tx = self.db.begin().await?;
        match receipt::check(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?
        {
            Some(Receipt::Same) => {
                let row = entities::spruce_message::Entity::find_by_id((
                    practice.to_owned(),
                    message_id.to_owned(),
                ))
                .one(&tx)
                .await?
                .ok_or_else(|| SpruceMatchingError::NotFound {
                    practice: practice.to_owned(),
                    kind: "message",
                    id: message_id.to_owned(),
                })?;
                let decoded = decode_message_row(row)?;
                tx.commit().await?;
                return Ok(decoded);
            }
            Some(Receipt::Different) => {
                return Err(SpruceMatchingError::IdempotencyCollision {
                    key: command_key.to_owned(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let row = entities::spruce_message::Entity::find_by_id((
            practice.to_owned(),
            message_id.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        .ok_or_else(|| SpruceMatchingError::NotFound {
            practice: practice.to_owned(),
            kind: "message",
            id: message_id.to_owned(),
        })?;
        let mut msg = decode_message_row(row)?;
        if !matches!(msg.standing, MessageStanding::Waiting { .. }) {
            return Err(SpruceMatchingError::InvalidTransition {
                id: message_id.to_owned(),
                current_state: msg.standing.kind_str().to_owned(),
                action: "leave_unlinked",
            });
        }
        let mut conv = lock_conversation_in_tx(&tx, practice, &msg.conversation_id)
            .await?
            .ok_or_else(|| SpruceMatchingError::NotFound {
                practice: practice.to_owned(),
                kind: "conversation",
                id: msg.conversation_id.clone(),
            })?;

        msg.standing = MessageStanding::LeftUnlinked {
            left_by: actor.physician_id.clone(),
            left_at: now_ts,
        };
        msg.updated_at = now_ts;
        save_message_in_tx(&tx, &msg).await?;

        let remaining_waiting = entities::spruce_message::Entity::find()
            .filter(entities::spruce_message::Column::Practice.eq(practice))
            .filter(entities::spruce_message::Column::ConversationId.eq(&conv.conversation_id))
            .filter(entities::spruce_message::Column::StandingKind.eq("waiting"))
            .one(&tx)
            .await?
            .is_some();
        if !remaining_waiting && conv.active_suggestion.is_some() {
            conv.active_suggestion = None;
            conv.updated_at = now_ts;
            save_conversation_in_tx(&tx, &conv).await?;
        }

        let _ = receipt::record(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?;
        tx.commit().await?;
        Ok(msg)
    }

    /// Opens a linked Spruce conversation for `viewer` (`OpenIt` in `MessageMatching.tla`),
    /// enforcing [`Membership::Allowed`] on its linked patient (`Rule 1`).
    pub async fn open_spruce_conversation(
        &self,
        auth: &dyn AuthStore,
        viewer: &PhysicianIdentity,
        conversation_id: &str,
        now_ts: i64,
    ) -> Result<AuthorizedSpruceConversation, SpruceMatchingError> {
        require_active_physician(auth, viewer).await?;
        let practice = viewer.practice_id.as_str();
        let conversation_id = require_id(conversation_id, "conversation_id")?;

        let tx = self.db.begin().await?;
        let mut conv = lock_conversation_in_tx(&tx, practice, conversation_id)
            .await?
            .ok_or_else(|| SpruceMatchingError::NotFound {
                practice: practice.to_owned(),
                kind: "conversation",
                id: conversation_id.to_owned(),
            })?;
        let ConversationLinkState::Linked { workspace, .. } = conv.link_state.clone() else {
            return Err(SpruceMatchingError::InvalidTransition {
                id: conversation_id.to_owned(),
                current_state: "unlinked_or_not_patient".to_owned(),
                action: "open_conversation",
            });
        };
        require_patient_access(auth, viewer, &workspace).await?;

        conv.open_viewers.insert(viewer.physician_id.clone());
        conv.updated_at = now_ts;
        save_conversation_in_tx(&tx, &conv).await?;
        let messages =
            linked_messages_for_conversation_in_tx(&tx, practice, conversation_id).await?;
        tx.commit().await?;

        Ok(AuthorizedSpruceConversation {
            conversation_id: conv.conversation_id,
            workspace,
            messages,
            move_undo: conv.move_undo.filter(|u| u.is_active_at(now_ts)),
            is_open_for_viewer: true,
        })
    }

    /// Closes an open Spruce conversation for `viewer` (`CloseIt` in `MessageMatching.tla`).
    pub async fn close_spruce_conversation(
        &self,
        auth: &dyn AuthStore,
        viewer: &PhysicianIdentity,
        conversation_id: &str,
        now_ts: i64,
    ) -> Result<(), SpruceMatchingError> {
        require_active_physician(auth, viewer).await?;
        let practice = viewer.practice_id.as_str();
        let conversation_id = require_id(conversation_id, "conversation_id")?;
        let tx = self.db.begin().await?;
        let mut conv = lock_conversation_in_tx(&tx, practice, conversation_id)
            .await?
            .ok_or_else(|| SpruceMatchingError::NotFound {
                practice: practice.to_owned(),
                kind: "conversation",
                id: conversation_id.to_owned(),
            })?;
        if conv.open_viewers.remove(&viewer.physician_id) {
            conv.updated_at = now_ts;
            save_conversation_in_tx(&tx, &conv).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Moves a linked conversation to another authorized patient or `Not a patient`
    /// (`Move` in `MessageMatching.tla`, `Rules 1, 5, 7, 8, 9`):
    /// - Immediately shows only in its new place (`on = {q}`) and drops open viewers
    ///   lacking access (`StillOpen(q)`).
    /// - Settles any prior expired `Undo` window (`now_ts >= undo.expires_at`) before
    ///   starting a new move (`chart-move-again`).
    /// - Shows `Undo` without acting on the move yet (`acted = false`).
    /// - Moving again while `Undo` shows preserves `undo.from` and `undo.moved_message_ids`;
    ///   moving back to `undo.from` ends the move (`undo = None`), doing nothing for the move
    ///   itself while queueing one review if new messages arrived during `Undo`.
    pub async fn move_spruce_conversation(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        command: MoveSpruceConversationCommand,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<SavedSpruceConversation, SpruceMatchingError> {
        require_active_physician(auth, actor).await?;
        let practice = actor.practice_id.as_str();
        let conversation_id = require_id(&command.conversation_id, "conversation_id")?;
        let command_key = require_id(&command.command_key, "command_key")?;
        let destination = command.destination;
        let undo_window_seconds = command.undo_window_seconds;
        if let MoveDestination::AnotherPatient { workspace } = &destination {
            require_patient_access(auth, actor, workspace).await?;
        }

        let command_body = json!({
            "kind": "spruce_move_conversation",
            "practice": practice,
            "physician": actor.physician_id,
            "conversation_id": conversation_id,
            "destination": destination,
        });

        let tx = self.db.begin().await?;
        match receipt::check(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?
        {
            Some(Receipt::Same) => {
                let conv = lock_conversation_in_tx(&tx, practice, conversation_id)
                    .await?
                    .ok_or_else(|| SpruceMatchingError::NotFound {
                        practice: practice.to_owned(),
                        kind: "conversation",
                        id: conversation_id.to_owned(),
                    })?;
                let undispatched =
                    undispatched_triggers_for_conversation_in_tx(&tx, practice, conversation_id)
                        .await?;
                tx.commit().await?;
                dispatch_trigger_list(self, &undispatched, review_receiver, now_ts).await?;
                return Ok(conv);
            }
            Some(Receipt::Different) => {
                return Err(SpruceMatchingError::IdempotencyCollision {
                    key: command_key.to_owned(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let mut conv = lock_conversation_in_tx(&tx, practice, conversation_id)
            .await?
            .ok_or_else(|| SpruceMatchingError::NotFound {
                practice: practice.to_owned(),
                kind: "conversation",
                id: conversation_id.to_owned(),
            })?;
        let ConversationLinkState::Linked {
            workspace: current_workspace,
            ..
        } = conv.link_state.clone()
        else {
            return Err(SpruceMatchingError::InvalidTransition {
                id: conversation_id.to_owned(),
                current_state: "not_linked_to_patient".to_owned(),
                action: "move_conversation",
            });
        };
        require_patient_access(auth, actor, &current_workspace).await?;

        if let MoveDestination::AnotherPatient { workspace } = &destination
            && workspace == &current_workspace
        {
            return Err(SpruceMatchingError::InvalidTransition {
                id: conversation_id.to_owned(),
                current_state: "same_patient".to_owned(),
                action: "move_conversation",
            });
        }

        let mut pending_triggers = Vec::new();
        if let Some(existing_undo) = conv.move_undo.clone() {
            if !existing_undo.is_active_at(now_ts) {
                let settled =
                    settle_expired_move_in_tx(&tx, &mut conv, &existing_undo, now_ts).await?;
                pending_triggers.extend(settled);
            } else if existing_undo.by_physician != actor.physician_id {
                return Err(SpruceMatchingError::InvalidTransition {
                    id: conversation_id.to_owned(),
                    current_state: "move_undo_held_by_other_physician".to_owned(),
                    action: "move_conversation",
                });
            }
        }

        let current_linked_msgs =
            linked_messages_for_conversation_in_tx(&tx, practice, conversation_id).await?;
        let current_linked_ids: BTreeSet<String> = current_linked_msgs
            .iter()
            .map(|m| m.message_id.clone())
            .collect();

        let target_ws_opt = match &destination {
            MoveDestination::AnotherPatient { workspace } => Some(workspace.as_str()),
            MoveDestination::NotAPatient => None,
        };
        conv.open_viewers =
            still_open_viewers(auth, practice, &conv.open_viewers, target_ws_opt).await;

        let undo_window = undo_window_seconds.max(1);
        let active_prior_undo = conv.move_undo.clone();
        let returned_to_origin = active_prior_undo.as_ref().is_some_and(|u| {
            u.by_physician == actor.physician_id
                && matches!(
                    &destination,
                    MoveDestination::AnotherPatient { workspace } if workspace == &u.from_workspace
                )
        });

        conv.move_undo = if returned_to_origin {
            None
        } else if let Some(prior) = &active_prior_undo {
            Some(PendingMoveUndo {
                by_physician: actor.physician_id.clone(),
                from_workspace: prior.from_workspace.clone(),
                moved_at: now_ts,
                expires_at: now_ts.saturating_add(undo_window),
                moved_message_ids: prior.moved_message_ids.clone(),
            })
        } else {
            Some(PendingMoveUndo {
                by_physician: actor.physician_id.clone(),
                from_workspace: current_workspace.clone(),
                moved_at: now_ts,
                expires_at: now_ts.saturating_add(undo_window),
                moved_message_ids: current_linked_ids.clone(),
            })
        };

        conv.link_state = match &destination {
            MoveDestination::AnotherPatient { workspace } => ConversationLinkState::Linked {
                workspace: workspace.clone(),
                linked_at: now_ts,
            },
            MoveDestination::NotAPatient => {
                ConversationLinkState::MovedNotAPatient { moved_at: now_ts }
            }
        };
        conv.move_count += 1;
        conv.last_move_acted = false;
        conv.updated_at = now_ts;
        save_conversation_in_tx(&tx, &conv).await?;

        if matches!(destination, MoveDestination::NotAPatient) {
            let msg_rows = entities::spruce_message::Entity::find()
                .filter(entities::spruce_message::Column::Practice.eq(practice))
                .filter(entities::spruce_message::Column::ConversationId.eq(conversation_id))
                .lock_exclusive()
                .all(&tx)
                .await?;
            for row in msg_rows {
                let mut msg = decode_message_row(row)?;
                if matches!(msg.standing, MessageStanding::Linked { .. }) {
                    msg.standing = MessageStanding::NotAPatient { moved_at: now_ts };
                    msg.updated_at = now_ts;
                    save_message_in_tx(&tx, &msg).await?;
                }
            }
        } else if returned_to_origin
            && let Some(prior_undo) = active_prior_undo
            && current_linked_ids
                .iter()
                .any(|id| !prior_undo.moved_message_ids.contains(id))
        {
            let arrival_key = format!(
                "spruce-move-return-arrivals:{practice}:{conversation_id}:move-{}:{}",
                conv.move_count, prior_undo.from_workspace
            );
            pending_triggers.push(
                queue_review_trigger_in_tx(
                    &tx,
                    practice,
                    conversation_id,
                    &prior_undo.from_workspace,
                    &arrival_key,
                    SpruceReviewTriggerOrigin::LinkedConversationArrival,
                    now_ts,
                )
                .await?,
            );
        }

        let _ = receipt::record(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?;
        tx.commit().await?;

        dispatch_trigger_list(self, &pending_triggers, review_receiver, now_ts).await?;
        Ok(conv)
    }

    /// Presses `Undo` on an active move (`UndoIt` in `MessageMatching.tla`, `Rule 7` & `Rule 8`):
    /// refuses expired `Undo` windows (`now_ts >= undo.expires_at`), restores the conversation
    /// and every message (including `NotAPatient` messages and messages that arrived while `Undo`
    /// showed) onto `undo.from`, prunes viewers lacking access to `undo.from`, and triggers one
    /// patient review on `undo.from` if new messages arrived during `Undo`.
    pub async fn undo_spruce_conversation_move(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        conversation_id: &str,
        command_key: &str,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<SavedSpruceConversation, SpruceMatchingError> {
        require_active_physician(auth, actor).await?;
        let practice = actor.practice_id.as_str();
        let conversation_id = require_id(conversation_id, "conversation_id")?;
        let command_key = require_id(command_key, "command_key")?;
        let command_body = json!({
            "kind": "spruce_undo_conversation_move",
            "practice": practice,
            "physician": actor.physician_id,
            "conversation_id": conversation_id,
        });

        let tx = self.db.begin().await?;
        match receipt::check(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?
        {
            Some(Receipt::Same) => {
                let conv = lock_conversation_in_tx(&tx, practice, conversation_id)
                    .await?
                    .ok_or_else(|| SpruceMatchingError::NotFound {
                        practice: practice.to_owned(),
                        kind: "conversation",
                        id: conversation_id.to_owned(),
                    })?;
                let undispatched =
                    undispatched_triggers_for_conversation_in_tx(&tx, practice, conversation_id)
                        .await?;
                tx.commit().await?;
                dispatch_trigger_list(self, &undispatched, review_receiver, now_ts).await?;
                return Ok(conv);
            }
            Some(Receipt::Different) => {
                return Err(SpruceMatchingError::IdempotencyCollision {
                    key: command_key.to_owned(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let mut conv = lock_conversation_in_tx(&tx, practice, conversation_id)
            .await?
            .ok_or_else(|| SpruceMatchingError::NotFound {
                practice: practice.to_owned(),
                kind: "conversation",
                id: conversation_id.to_owned(),
            })?;
        let Some(undo) = conv.move_undo.clone() else {
            return Err(SpruceMatchingError::InvalidTransition {
                id: conversation_id.to_owned(),
                current_state: "no_active_undo".to_owned(),
                action: "undo_move",
            });
        };
        if !undo.is_active_at(now_ts) {
            return Err(SpruceMatchingError::InvalidTransition {
                id: conversation_id.to_owned(),
                current_state: "undo_expired".to_owned(),
                action: "undo_move",
            });
        }
        if undo.by_physician != actor.physician_id {
            return Err(SpruceMatchingError::AccessDenied {
                practice: practice.to_owned(),
                physician: actor.physician_id.clone(),
                reason: "only the physician who moved the conversation may press Undo",
            });
        }
        require_patient_access(auth, actor, &undo.from_workspace).await?;

        conv.link_state = ConversationLinkState::Linked {
            workspace: undo.from_workspace.clone(),
            linked_at: now_ts,
        };
        conv.open_viewers = still_open_viewers(
            auth,
            practice,
            &conv.open_viewers,
            Some(&undo.from_workspace),
        )
        .await;
        conv.move_undo = None;
        conv.updated_at = now_ts;
        save_conversation_in_tx(&tx, &conv).await?;

        let msg_rows = entities::spruce_message::Entity::find()
            .filter(entities::spruce_message::Column::Practice.eq(practice))
            .filter(entities::spruce_message::Column::ConversationId.eq(conversation_id))
            .lock_exclusive()
            .all(&tx)
            .await?;
        let mut saw_mid_undo_arrival = false;
        for row in msg_rows {
            let mut msg = decode_message_row(row)?;
            if matches!(msg.standing, MessageStanding::NotAPatient { .. }) {
                msg.standing = MessageStanding::Linked {
                    linked_at: now_ts,
                    origin: LinkOrigin::RestoredByMoveUndo {
                        physician_id: actor.physician_id.clone(),
                    },
                };
                msg.ever_linked = true;
                msg.updated_at = now_ts;
                save_message_in_tx(&tx, &msg).await?;
            } else if matches!(msg.standing, MessageStanding::Linked { .. }) && !msg.ever_linked {
                msg.ever_linked = true;
                msg.updated_at = now_ts;
                save_message_in_tx(&tx, &msg).await?;
            }
            if matches!(msg.standing, MessageStanding::Linked { .. })
                && !undo.moved_message_ids.contains(&msg.message_id)
            {
                saw_mid_undo_arrival = true;
            }
        }

        let mut triggers = Vec::new();
        if saw_mid_undo_arrival {
            let undo_arrival_key = format!(
                "spruce-undo-arrivals:{practice}:{conversation_id}:move-{}:{}",
                conv.move_count, undo.from_workspace
            );
            triggers.push(
                queue_review_trigger_in_tx(
                    &tx,
                    practice,
                    conversation_id,
                    &undo.from_workspace,
                    &undo_arrival_key,
                    SpruceReviewTriggerOrigin::LinkedConversationArrival,
                    now_ts,
                )
                .await?,
            );
        }

        let _ = receipt::record(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?;
        tx.commit().await?;

        dispatch_trigger_list(self, &triggers, review_receiver, now_ts).await?;
        Ok(conv)
    }

    /// Settles every conversation in `practice` whose `Undo` window has expired
    /// (`now_ts >= undo.expires_at` — `UndoGoesAway` in `MessageMatching.tla`, `Rule 8`):
    /// - Marks moved messages (`undo.moved_message_ids`) entered in error on `undo.from` (never deleting them).
    /// - Sets `last_move_acted = true` and clears `move_undo`.
    /// - Starts **one** patient review for each affected patient (`undo.from` and, when
    ///   moved to another patient, the destination `workspace`), isolating per-patient
    ///   review failures so one patient's failure never blocks another's.
    pub async fn settle_expired_spruce_moves(
        &self,
        practice: &str,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<BatchStepReport<SavedSpruceConversation>, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let rows = entities::spruce_conversation::Entity::find()
            .filter(entities::spruce_conversation::Column::Practice.eq(practice))
            .filter(entities::spruce_conversation::Column::MoveUndo.is_not_null())
            .all(&self.db)
            .await?;

        let mut succeeded = Vec::new();
        let mut failures = Vec::new();
        for row in rows {
            let cid = row.conversation_id.clone();
            let decoded = match decode_conversation_row(row) {
                Ok(c) => c,
                Err(err) => {
                    failures.push(PatientIsolationFailure {
                        target_id: cid,
                        error: SpruceMatchingError::Database(err),
                    });
                    continue;
                }
            };
            let Some(undo) = decoded.move_undo else {
                continue;
            };
            if undo.is_active_at(now_ts) {
                continue;
            }
            match self
                .act_on_spruce_conversation_move(practice, &cid, review_receiver, now_ts)
                .await
            {
                Ok(settled) => succeeded.push(settled),
                Err(error) => failures.push(PatientIsolationFailure {
                    target_id: cid,
                    error,
                }),
            }
        }
        Ok(BatchStepReport {
            succeeded,
            failures,
        })
    }

    /// Settles `UndoGoesAway` for one conversation (`Rule 8`), refusing early settlement
    /// while `Undo` is still active (`now_ts < undo.expires_at`), writing permanent
    /// `entered_in_error` audit rows strictly for `undo.moved_message_ids` on `from_workspace`,
    /// and submitting one review per affected patient.
    pub async fn act_on_spruce_conversation_move(
        &self,
        practice: &str,
        conversation_id: &str,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<SavedSpruceConversation, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let conversation_id = require_id(conversation_id, "conversation_id")?;

        let tx = self.db.begin().await?;
        let mut conv = lock_conversation_in_tx(&tx, practice, conversation_id)
            .await?
            .ok_or_else(|| SpruceMatchingError::NotFound {
                practice: practice.to_owned(),
                kind: "conversation",
                id: conversation_id.to_owned(),
            })?;
        let Some(undo) = conv.move_undo.clone() else {
            return Err(SpruceMatchingError::InvalidTransition {
                id: conversation_id.to_owned(),
                current_state: "no_active_undo".to_owned(),
                action: "act_on_move",
            });
        };
        let triggers = settle_expired_move_in_tx(&tx, &mut conv, &undo, now_ts).await?;
        tx.commit().await?;

        dispatch_trigger_list(self, &triggers, review_receiver, now_ts).await?;
        Ok(conv)
    }

    /// Retries any queued Spruce review triggers in `practice` that have not yet
    /// been accepted by `review_receiver`, isolating per-patient failures.
    pub async fn dispatch_pending_spruce_reviews(
        &self,
        practice: &str,
        review_receiver: &dyn ReviewReceiver,
        now_ts: i64,
    ) -> Result<BatchStepReport<PendingSpruceReviewTrigger>, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let rows = entities::spruce_review_trigger::Entity::find()
            .filter(entities::spruce_review_trigger::Column::Practice.eq(practice))
            .filter(entities::spruce_review_trigger::Column::Dispatched.eq(false))
            .order_by_asc(entities::spruce_review_trigger::Column::CreatedAt)
            .all(&self.db)
            .await?;

        let mut succeeded = Vec::new();
        let mut failures = Vec::new();
        for row in rows {
            let key = row.command_key.clone();
            let ws = row.workspace.clone();
            let mut trig = match decode_trigger_row(row) {
                Ok(t) => t,
                Err(err) => {
                    failures.push(PatientIsolationFailure {
                        target_id: key,
                        error: SpruceMatchingError::Database(err),
                    });
                    continue;
                }
            };
            match dispatch_single_trigger(self, &trig, review_receiver, now_ts).await {
                Ok(()) => {
                    trig.dispatched = true;
                    trig.dispatched_at = Some(now_ts);
                    succeeded.push(trig);
                }
                Err(error) => failures.push(PatientIsolationFailure {
                    target_id: ws,
                    error,
                }),
            }
        }
        Ok(BatchStepReport {
            succeeded,
            failures,
        })
    }

    /// Reads one saved Spruce message in `practice`.
    pub async fn spruce_message(
        &self,
        practice: &str,
        message_id: &str,
    ) -> Result<Option<SavedSpruceMessage>, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let message_id = require_id(message_id, "message_id")?;
        entities::spruce_message::Entity::find_by_id((practice.to_owned(), message_id.to_owned()))
            .one(&self.db)
            .await?
            .map(decode_message_row)
            .transpose()
            .map_err(Into::into)
    }

    /// Reads one saved Spruce conversation in `practice`.
    pub async fn spruce_conversation(
        &self,
        practice: &str,
        conversation_id: &str,
    ) -> Result<Option<SavedSpruceConversation>, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let conversation_id = require_id(conversation_id, "conversation_id")?;
        entities::spruce_conversation::Entity::find_by_id((
            practice.to_owned(),
            conversation_id.to_owned(),
        ))
        .one(&self.db)
        .await?
        .map(decode_conversation_row)
        .transpose()
        .map_err(Into::into)
    }

    /// Projects `"Messages to match"` for `viewer` (`Rules 2, 6`):
    /// - Urgent rows lead first (`leads_today_as_now == true`), then chronological.
    /// - Suggestions are filtered through [`project_suggestion_for_viewer`] (`Membership::Allowed` only).
    /// - Zero numeric confidence scores exist on any row.
    pub async fn messages_to_match_for_physician(
        &self,
        auth: &dyn AuthStore,
        viewer: &PhysicianIdentity,
    ) -> Result<Vec<UnmatchedMessageRow>, SpruceMatchingError> {
        require_active_physician(auth, viewer).await?;
        let practice = viewer.practice_id.as_str();

        let waiting_rows = entities::spruce_message::Entity::find()
            .filter(entities::spruce_message::Column::Practice.eq(practice))
            .filter(entities::spruce_message::Column::StandingKind.eq("waiting"))
            .order_by_asc(entities::spruce_message::Column::ChannelPosition)
            .all(&self.db)
            .await?;

        let conv_ids: BTreeSet<String> = waiting_rows
            .iter()
            .map(|r| r.conversation_id.clone())
            .collect();
        let mut conv_map = BTreeMap::new();
        if !conv_ids.is_empty() {
            let conv_rows = entities::spruce_conversation::Entity::find()
                .filter(entities::spruce_conversation::Column::Practice.eq(practice))
                .filter(
                    entities::spruce_conversation::Column::ConversationId
                        .is_in(conv_ids.iter().cloned()),
                )
                .all(&self.db)
                .await?;
            for c_row in conv_rows {
                let c = decode_conversation_row(c_row)?;
                conv_map.insert(c.conversation_id.clone(), c);
            }
        }

        let mut rows = Vec::with_capacity(waiting_rows.len());
        for row in waiting_rows {
            let msg = decode_message_row(row)?;
            let MessageStanding::Waiting { waiting_since, .. } = msg.standing else {
                continue;
            };
            let raw_suggestion = conv_map
                .get(&msg.conversation_id)
                .and_then(|c| c.active_suggestion.as_ref());
            let suggested_patient =
                project_suggestion_for_viewer(auth, viewer, raw_suggestion).await;
            let leads_today_as_now = msg.urgency.is_urgent();
            rows.push(UnmatchedMessageRow {
                message_id: msg.message_id,
                conversation_id: msg.conversation_id,
                author_name: msg.author_name,
                sender_address: msg.sender_address,
                body_text: msg.body_text,
                attachments: msg.attachments,
                app_url: msg.app_url,
                urgency: msg.urgency,
                leads_today_as_now,
                suggested_patient,
                waiting_since,
            });
        }
        rows.sort_by(|a, b| {
            b.leads_today_as_now
                .cmp(&a.leads_today_as_now)
                .then_with(|| a.waiting_since.cmp(&b.waiting_since))
                .then_with(|| a.message_id.cmp(&b.message_id))
        });
        Ok(rows)
    }

    /// Projects `Filtered` non-patient messages for an active physician in `viewer.practice_id`
    /// so they can inspect filtered items and click `"This is a patient"`.
    pub async fn filtered_spruce_messages_for_physician(
        &self,
        auth: &dyn AuthStore,
        viewer: &PhysicianIdentity,
    ) -> Result<Vec<FilteredMessageRow>, SpruceMatchingError> {
        require_active_physician(auth, viewer).await?;
        let practice = viewer.practice_id.as_str();
        let rows = entities::spruce_message::Entity::find()
            .filter(entities::spruce_message::Column::Practice.eq(practice))
            .filter(entities::spruce_message::Column::StandingKind.eq("filtered"))
            .order_by_asc(entities::spruce_message::Column::ChannelPosition)
            .all(&self.db)
            .await?;

        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let msg = decode_message_row(row)?;
            if let MessageStanding::Filtered {
                category,
                rationale,
                filtered_at,
            } = msg.standing
            {
                out.push(FilteredMessageRow {
                    message_id: msg.message_id,
                    conversation_id: msg.conversation_id,
                    author_name: msg.author_name,
                    sender_address: msg.sender_address,
                    body_text: msg.body_text,
                    category,
                    rationale,
                    filtered_at,
                });
            }
        }
        Ok(out)
    }

    /// Projects Today's Spruce channel banner, backlog offer, waiting unmatched rows,
    /// and unreadable messages for `viewer` (`Rules 2, 6, 10, 13, 14`).
    pub async fn today_spruce_summary_for_physician(
        &self,
        auth: &dyn AuthStore,
        viewer: &PhysicianIdentity,
    ) -> Result<TodaySpruceSummary, SpruceMatchingError> {
        require_active_physician(auth, viewer).await?;
        let practice = viewer.practice_id.as_str();
        let (standing, _) = self.spruce_channel_standing(practice).await?;
        let channel_banner = standing.today_banner();
        let backlog_offered = matches!(
            standing,
            ChannelStanding::Connected {
                backlog: BacklogStanding::Offered { .. },
                ..
            }
        );
        let messages_to_match = self.messages_to_match_for_physician(auth, viewer).await?;

        let cant_read_rows = entities::spruce_message::Entity::find()
            .filter(entities::spruce_message::Column::Practice.eq(practice))
            .filter(entities::spruce_message::Column::StandingKind.eq("cant_read"))
            .order_by_asc(entities::spruce_message::Column::ChannelPosition)
            .all(&self.db)
            .await?;
        let mut unreadable_messages = Vec::with_capacity(cant_read_rows.len());
        for row in cant_read_rows {
            let msg = decode_message_row(row)?;
            if let MessageStanding::CantRead {
                failure_reason,
                app_url,
            } = msg.standing
            {
                unreadable_messages.push(UnreadableMessageRow {
                    message_id: msg.message_id,
                    conversation_id: msg.conversation_id,
                    failure_reason,
                    app_url,
                    kept_at: msg.kept_at,
                });
            }
        }

        Ok(TodaySpruceSummary {
            channel_banner,
            backlog_offered,
            messages_to_match,
            unreadable_messages,
        })
    }

    /// Reads an authorized linked conversation for `viewer` (`Rule 1: OnlyWithAccess` & `Rule 5: RightPatient`).
    /// Refuses access when the conversation is unlinked, moved to `Not a patient`, or
    /// linked to a patient `viewer` may not access.
    pub async fn authorized_spruce_conversation_for_physician(
        &self,
        auth: &dyn AuthStore,
        viewer: &PhysicianIdentity,
        conversation_id: &str,
        now_ts: i64,
    ) -> Result<AuthorizedSpruceConversation, SpruceMatchingError> {
        require_active_physician(auth, viewer).await?;
        let practice = viewer.practice_id.as_str();
        let conversation_id = require_id(conversation_id, "conversation_id")?;

        let conv = self
            .spruce_conversation(practice, conversation_id)
            .await?
            .ok_or_else(|| SpruceMatchingError::NotFound {
                practice: practice.to_owned(),
                kind: "conversation",
                id: conversation_id.to_owned(),
            })?;
        let ConversationLinkState::Linked { workspace, .. } = conv.link_state else {
            return Err(SpruceMatchingError::AccessDenied {
                practice: practice.to_owned(),
                physician: viewer.physician_id.clone(),
                reason: "conversation is not linked to an accessible patient",
            });
        };
        require_patient_access(auth, viewer, &workspace).await?;
        let messages =
            linked_messages_for_conversation_in_tx(&self.db, practice, conversation_id).await?;
        let is_open_for_viewer = conv.open_viewers.contains(&viewer.physician_id);
        Ok(AuthorizedSpruceConversation {
            conversation_id: conv.conversation_id,
            workspace,
            messages,
            move_undo: conv.move_undo.filter(|u| u.is_active_at(now_ts)),
            is_open_for_viewer,
        })
    }

    /// Lists conversation IDs currently open for `viewer`, filtering dynamically by
    /// live [`Membership::Allowed`] so panel revocation closes visibility at once (`Rule 1`).
    pub async fn open_spruce_conversations_for_physician(
        &self,
        auth: &dyn AuthStore,
        viewer: &PhysicianIdentity,
        now_ts: i64,
    ) -> Result<Vec<AuthorizedSpruceConversation>, SpruceMatchingError> {
        require_active_physician(auth, viewer).await?;
        let practice = viewer.practice_id.as_str();
        let rows = entities::spruce_conversation::Entity::find()
            .filter(entities::spruce_conversation::Column::Practice.eq(practice))
            .filter(entities::spruce_conversation::Column::LinkKind.eq("linked"))
            .order_by_asc(entities::spruce_conversation::Column::ConversationId)
            .all(&self.db)
            .await?;

        let mut open = Vec::new();
        for row in rows {
            let conv = decode_conversation_row(row)?;
            if !conv.open_viewers.contains(&viewer.physician_id) {
                continue;
            }
            let ConversationLinkState::Linked { workspace, .. } = conv.link_state else {
                continue;
            };
            if auth.membership(viewer, &workspace).await != Membership::Allowed {
                continue;
            }
            let messages =
                linked_messages_for_conversation_in_tx(&self.db, practice, &conv.conversation_id)
                    .await?;
            open.push(AuthorizedSpruceConversation {
                conversation_id: conv.conversation_id,
                workspace,
                messages,
                move_undo: conv.move_undo.filter(|u| u.is_active_at(now_ts)),
                is_open_for_viewer: true,
            });
        }
        Ok(open)
    }

    /// Returns **only** Spruce messages currently `Linked` to `workspace` in `practice`.
    /// Unlinked, waiting, deciding, filtered, left-unlinked, unreadable, and moved-away
    /// messages are **excluded** (`Rule 5` & non-TLC review-input rule).
    pub async fn patient_linked_spruce_messages(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<Vec<SavedSpruceMessage>, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let workspace = require_id(workspace, "workspace")?;

        let linked_convs = entities::spruce_conversation::Entity::find()
            .filter(entities::spruce_conversation::Column::Practice.eq(practice))
            .filter(entities::spruce_conversation::Column::LinkKind.eq("linked"))
            .filter(entities::spruce_conversation::Column::LinkedWorkspace.eq(workspace))
            .order_by_asc(entities::spruce_conversation::Column::ConversationId)
            .all(&self.db)
            .await?;
        let conv_ids: Vec<String> = linked_convs
            .into_iter()
            .map(|c| c.conversation_id)
            .collect();
        if conv_ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = entities::spruce_message::Entity::find()
            .filter(entities::spruce_message::Column::Practice.eq(practice))
            .filter(entities::spruce_message::Column::ConversationId.is_in(conv_ids))
            .filter(entities::spruce_message::Column::StandingKind.eq("linked"))
            .order_by_asc(entities::spruce_message::Column::ChannelPosition)
            .all(&self.db)
            .await?;

        let mut messages = Vec::with_capacity(rows.len());
        for row in rows {
            let decoded = decode_message_row(row)?;
            if matches!(decoded.standing, MessageStanding::Linked { .. }) {
                messages.push(decoded);
            }
        }
        Ok(messages)
    }

    /// Assembles authorized review inputs for `(practice, workspace)`:
    /// checks `authority`, reads the retained chart [`Observation`], and attaches
    /// [`Store::patient_linked_spruce_messages`] so unlinked or moved-away mail
    /// **never** enters a patient review.
    pub async fn patient_review_inputs_with_spruce(
        &self,
        practice: &str,
        workspace: &str,
        authority: &dyn PracticeAuthority,
    ) -> Result<PatientReviewInputs, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let workspace = require_id(workspace, "workspace")?;
        authority
            .check_practice_patient(&PracticeScope {
                practice: practice.to_owned(),
                workspace: workspace.to_owned(),
                job_id: "spruce_review_inputs".to_owned(),
            })
            .await?;

        let chart = self.sources(workspace).await?;
        let linked_spruce_messages = self
            .patient_linked_spruce_messages(practice, workspace)
            .await?;
        Ok(PatientReviewInputs {
            practice: practice.to_owned(),
            workspace: workspace.to_owned(),
            chart,
            linked_spruce_messages,
        })
    }

    /// Reads permanent `entered_in_error` records retained on `from_workspace`
    /// after wrong-patient conversation moves settled (`README.md` non-TLC rule).
    pub async fn entered_in_error_spruce_messages(
        &self,
        practice: &str,
        from_workspace: &str,
    ) -> Result<Vec<SpruceEnteredInErrorRecord>, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let from_workspace = require_id(from_workspace, "from_workspace")?;
        let rows = entities::spruce_entered_in_error::Entity::find()
            .filter(entities::spruce_entered_in_error::Column::Practice.eq(practice))
            .filter(entities::spruce_entered_in_error::Column::FromWorkspace.eq(from_workspace))
            .order_by_asc(entities::spruce_entered_in_error::Column::MoveOrdinal)
            .order_by_asc(entities::spruce_entered_in_error::Column::MessageId)
            .all(&self.db)
            .await?;
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let destination: MoveDestination = serde_json::from_value(row.destination).map_err(
                conversion_error("spruce_entered_in_error.destination", "MoveDestination"),
            )?;
            let move_ordinal = u64::try_from(row.move_ordinal).map_err(conversion_error(
                "spruce_entered_in_error.move_ordinal",
                "u64",
            ))?;
            out.push(SpruceEnteredInErrorRecord {
                practice: row.practice,
                from_workspace: row.from_workspace,
                message_id: row.message_id,
                move_ordinal,
                conversation_id: row.conversation_id,
                destination,
                marked_at: row.marked_at,
            });
        }
        Ok(out)
    }

    /// Reads queued Spruce review trigger rows for `practice` (for replay/proof inspection).
    pub async fn spruce_review_triggers(
        &self,
        practice: &str,
    ) -> Result<Vec<PendingSpruceReviewTrigger>, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let rows = entities::spruce_review_trigger::Entity::find()
            .filter(entities::spruce_review_trigger::Column::Practice.eq(practice))
            .order_by_asc(entities::spruce_review_trigger::Column::CreatedAt)
            .order_by_asc(entities::spruce_review_trigger::Column::CommandKey)
            .all(&self.db)
            .await?;
        rows.into_iter()
            .map(decode_trigger_row)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Builds the authorized practice demographic & contact roster for `patient_lookup`
    /// during unmatched message classification. Reads **only** registered patient IDs, `source_pull` demographics,
    /// confirmed sender addresses, and Spruce contact/EHR links — **never** clinical
    /// chart records, notes, labs, patient understanding, or patient memory.
    pub async fn authorized_practice_roster_for_matching(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
    ) -> Result<Vec<MatchingRosterPatient>, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let workspaces = auth.practice_patients(practice).await?;
        self.practice_roster_for_workspaces(practice, &workspaces)
            .await
    }

    pub(crate) async fn practice_roster_for_workspaces(
        &self,
        practice: &str,
        workspaces: &[String],
    ) -> Result<Vec<MatchingRosterPatient>, SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let registered_set: BTreeSet<String> = workspaces.iter().cloned().collect();

        let pulls_by_ws = if workspaces.is_empty() {
            BTreeMap::new()
        } else {
            self.source_pulls(workspaces).await?
        };
        let (contacts, ehr_map) =
            load_practice_channel_evidence_in_tx(&self.db, practice, &registered_set).await?;

        let sender_rows = entities::spruce_confirmed_sender::Entity::find()
            .filter(entities::spruce_confirmed_sender::Column::Practice.eq(practice))
            .order_by_asc(entities::spruce_confirmed_sender::Column::SenderKey)
            .all(&self.db)
            .await?;
        let mut senders_by_ws: BTreeMap<String, Vec<SenderAddress>> = BTreeMap::new();
        for s_row in sender_rows {
            if let Ok(addr) = SenderAddress::parse(&s_row.sender_key) {
                senders_by_ws.entry(s_row.workspace).or_default().push(addr);
            }
        }

        let mut ehrs_by_ws: BTreeMap<String, Vec<SpruceEhrLink>> = BTreeMap::new();
        for (ehr, ws_set) in &ehr_map {
            for ws in ws_set {
                ehrs_by_ws.entry(ws.clone()).or_default().push(ehr.clone());
            }
        }

        let mut contacts_by_ws: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for (contact_id, patients) in &contacts {
            for sp in patients {
                for ehr in &sp.ehr_links {
                    if let Some(ws_set) = ehr_map.get(ehr) {
                        for ws in ws_set {
                            contacts_by_ws
                                .entry(ws.clone())
                                .or_default()
                                .insert(contact_id.clone());
                        }
                    }
                }
            }
        }

        let mut roster = Vec::with_capacity(workspaces.len());
        for ws in workspaces {
            let demographics = pulls_by_ws
                .get(ws)
                .and_then(|pulls| pulls.first())
                .map(|pull| pull.patient.clone());
            roster.push(MatchingRosterPatient {
                demographics,
                confirmed_senders: senders_by_ws.remove(ws).unwrap_or_default(),
                linked_spruce_contacts: contacts_by_ws
                    .remove(ws)
                    .map(|s| s.into_iter().collect())
                    .unwrap_or_default(),
                ehr_links: ehrs_by_ws.remove(ws).unwrap_or_default(),
                workspace: ws.clone(),
            });
        }
        Ok(roster)
    }

    pub(crate) fn spruce_db(&self) -> &sea_orm::DatabaseConnection {
        &self.db
    }

    pub(crate) async fn spruce_now(&self) -> Result<i64, DbErr> {
        super::now(&self.db).await
    }

    pub(crate) async fn keep_spruce_matching_frozen(
        &self,
        tx: &impl ConnectionTrait,
        practice: &str,
        binding: &harness::engine::recipe::RequestBinding,
        skills: &crate::physician_chat::PhysicianSkills,
    ) -> Result<(super::BindingRef, super::SkillPackageId), DbErr> {
        let binding_ref =
            super::frozen::keep_owner_binding(tx, practice, super::jobs::PRACTICE_OWNER, binding)
                .await?;
        let skills_id =
            super::frozen::keep_owner_skills(tx, practice, super::jobs::PRACTICE_OWNER, skills)
                .await?;
        Ok((binding_ref, skills_id))
    }

    pub(crate) async fn load_spruce_matching_frozen(
        &self,
        practice: &str,
        binding_ref: &super::BindingRef,
        skills_id: &super::SkillPackageId,
    ) -> Result<
        (
            harness::engine::recipe::RequestBinding,
            crate::physician_chat::PhysicianSkills,
        ),
        DbErr,
    > {
        let binding = super::frozen::owner_binding(
            &self.db,
            practice,
            super::jobs::PRACTICE_OWNER,
            binding_ref,
        )
        .await?;
        let skills =
            super::frozen::owner_skills(&self.db, practice, super::jobs::PRACTICE_OWNER, skills_id)
                .await?;
        Ok((binding, skills))
    }

    pub(crate) async fn evaluate_conversation_identity_for_matching(
        &self,
        practice: &str,
        sender_address: Option<&SenderAddress>,
        outside_participants: &[OutsideParticipant],
        registered_workspaces: &BTreeSet<String>,
    ) -> Result<(BTreeSet<String>, ChannelTieEvaluation), SpruceMatchingError> {
        let practice = require_id(practice, "practice")?;
        let confirmed_sender = confirmed_workspaces_for_conversation_in_tx(
            &self.db,
            practice,
            sender_address,
            outside_participants,
            registered_workspaces,
        )
        .await?;
        let (contacts, ehr_map) =
            load_practice_channel_evidence_in_tx(&self.db, practice, registered_workspaces).await?;
        let channel_tie = evaluate_channel_tie(outside_participants, &contacts, &ehr_map);
        Ok((confirmed_sender, channel_tie))
    }
}

async fn settle_expired_move_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    conv: &mut SavedSpruceConversation,
    undo: &PendingMoveUndo,
    now_ts: i64,
) -> Result<Vec<PendingSpruceReviewTrigger>, SpruceMatchingError> {
    if undo.is_active_at(now_ts) {
        return Err(SpruceMatchingError::InvalidTransition {
            id: conv.conversation_id.clone(),
            current_state: "undo_still_active".to_owned(),
            action: "act_on_move",
        });
    }
    let destination = match &conv.link_state {
        ConversationLinkState::Linked { workspace, .. } => MoveDestination::AnotherPatient {
            workspace: workspace.clone(),
        },
        ConversationLinkState::MovedNotAPatient { .. } => MoveDestination::NotAPatient,
        ConversationLinkState::Unlinked => {
            return Err(SpruceMatchingError::InvalidTransition {
                id: conv.conversation_id.clone(),
                current_state: "unlinked".to_owned(),
                action: "act_on_move",
            });
        }
    };

    let move_ordinal =
        i64::try_from(conv.move_count).map_err(conversion_error("move_count", "i64"))?;
    let destination_json =
        serde_json::to_value(&destination).map_err(conversion_error("MoveDestination", "json"))?;

    for message_id in &undo.moved_message_ids {
        entities::spruce_entered_in_error::Entity::insert(
            entities::spruce_entered_in_error::ActiveModel {
                practice: Set(conv.practice.clone()),
                from_workspace: Set(undo.from_workspace.clone()),
                message_id: Set(message_id.clone()),
                move_ordinal: Set(move_ordinal),
                conversation_id: Set(conv.conversation_id.clone()),
                destination: Set(destination_json.clone()),
                marked_at: Set(now_ts),
            },
        )
        .on_conflict(
            OnConflict::columns([
                entities::spruce_entered_in_error::Column::Practice,
                entities::spruce_entered_in_error::Column::FromWorkspace,
                entities::spruce_entered_in_error::Column::MessageId,
                entities::spruce_entered_in_error::Column::MoveOrdinal,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec_without_returning(tx)
        .await?;
    }

    conv.move_undo = None;
    conv.last_move_acted = true;
    conv.updated_at = now_ts;
    save_conversation_in_tx(tx, conv).await?;

    let mut triggers = Vec::with_capacity(2);
    let from_key = format!(
        "spruce-move-settled-from:{}:{}:move-{}:{}",
        conv.practice, conv.conversation_id, conv.move_count, undo.from_workspace
    );
    triggers.push(
        queue_review_trigger_in_tx(
            tx,
            &conv.practice,
            &conv.conversation_id,
            &undo.from_workspace,
            &from_key,
            SpruceReviewTriggerOrigin::MoveSettledFrom,
            now_ts,
        )
        .await?,
    );

    if let MoveDestination::AnotherPatient { workspace } = &destination {
        let to_key = format!(
            "spruce-move-settled-to:{}:{}:move-{}:{}",
            conv.practice, conv.conversation_id, conv.move_count, workspace
        );
        triggers.push(
            queue_review_trigger_in_tx(
                tx,
                &conv.practice,
                &conv.conversation_id,
                workspace,
                &to_key,
                SpruceReviewTriggerOrigin::MoveSettledTo,
                now_ts,
            )
            .await?,
        );
    }

    Ok(triggers)
}

async fn undispatched_triggers_for_conversation_in_tx(
    tx: &impl ConnectionTrait,
    practice: &str,
    conversation_id: &str,
) -> Result<Vec<PendingSpruceReviewTrigger>, DbErr> {
    let rows = entities::spruce_review_trigger::Entity::find()
        .filter(entities::spruce_review_trigger::Column::Practice.eq(practice))
        .filter(entities::spruce_review_trigger::Column::ConversationId.eq(conversation_id))
        .filter(entities::spruce_review_trigger::Column::Dispatched.eq(false))
        .order_by_asc(entities::spruce_review_trigger::Column::CreatedAt)
        .order_by_asc(entities::spruce_review_trigger::Column::CommandKey)
        .all(tx)
        .await?;
    rows.into_iter().map(decode_trigger_row).collect()
}

async fn dispatch_trigger_list(
    store: &Store,
    triggers: &[PendingSpruceReviewTrigger],
    review_receiver: Option<&dyn ReviewReceiver>,
    now_ts: i64,
) -> Result<(), SpruceMatchingError> {
    let Some(receiver) = review_receiver else {
        return Ok(());
    };
    let mut first_err: Option<SpruceMatchingError> = None;
    for trig in triggers {
        if let Err(err) = dispatch_single_trigger(store, trig, receiver, now_ts).await
            && first_err.is_none()
        {
            first_err = Some(err);
        }
    }
    if let Some(err) = first_err {
        return Err(err);
    }
    Ok(())
}

async fn linked_messages_for_conversation_in_tx(
    db: &impl ConnectionTrait,
    practice: &str,
    conversation_id: &str,
) -> Result<Vec<SavedSpruceMessage>, DbErr> {
    let rows = entities::spruce_message::Entity::find()
        .filter(entities::spruce_message::Column::Practice.eq(practice))
        .filter(entities::spruce_message::Column::ConversationId.eq(conversation_id))
        .filter(entities::spruce_message::Column::StandingKind.eq("linked"))
        .order_by_asc(entities::spruce_message::Column::ChannelPosition)
        .all(db)
        .await?;
    rows.into_iter().map(decode_message_row).collect()
}

pub(crate) fn spruce_db_error(message: &str) -> DbErr {
    error(message)
}

pub(crate) fn spruce_conversion_error<E>(
    from: &'static str,
    to: &'static str,
) -> impl FnOnce(E) -> DbErr
where
    E: std::error::Error + Send + Sync + 'static,
{
    conversion_error(from, to)
}

pub(crate) fn is_spruce_working_note_tool(name: &str) -> bool {
    super::working_notes::is_tool(name)
}

pub(crate) fn execute_spruce_working_note<'a>(
    work: &str,
    call: &harness::engine::Invocation<serde_json::Value>,
    records: impl IntoIterator<Item = (&'a str, &'a harness::engine::Event)>,
) -> Result<Option<harness::engine::ToolOutcome>, harness::engine::Cause> {
    super::working_notes::execute(work, call, records)
}

pub(crate) fn spruce_working_note_references<'a>(
    records: impl IntoIterator<Item = (&'a str, &'a harness::engine::Event)>,
) -> Result<Vec<String>, harness::engine::Cause> {
    super::working_notes::references(records)
}
