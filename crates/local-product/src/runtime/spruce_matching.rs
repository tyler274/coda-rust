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

use super::jobs::{PracticeAccessError, PracticeJobError};
use super::sources::Observation;
use crate::physician_sources::SourcePatient;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, StoreUnavailable};
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

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
