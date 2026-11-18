//! Cross-producer Today and Updates read model, per-physician seen state,
//! practice-wide source outage collapsing, and item work-and-evidence readback.
//!
//! Today and Updates read only saved clinical outcomes and operational standing
//! across a physician's authorized panel patients, never running model calls on
//! read. Every producer executes independently so one failed part degrades into
//! [`ReadCompleteness::Partial`] while healthy producers still return their rows.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use physician_auth::{AuthStore, Membership, PhysicianIdentity, SessionError, StoreUnavailable};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::concerns::{
    CommandKey, ConcernError, ConcernId, ConcernInputError, ConcernUrgency, ExactRecordReference,
};
use super::entities::{
    concern, conversation, patient_review, patient_understanding, practice_job, schedule_source,
    source_pull, source_status, today_updates_seen, today_work_dismissal, visit_concern,
    visit_preparation,
};
use super::jobs::PracticeJobKind;
use super::reviews::{ReviewHoldReason, ReviewLifecycleState};
use super::{MessageRole, Store, WorkStatus, control, conversion_error};
use crate::patient_reviews::{ReviewOutcome, ReviewTrigger, UnfinishedReviewWork};
use crate::physician_sources::SourcePatient;

const SECONDS_PER_DAY: i64 = 86_400;

/// Which durable runtime producer owns one slice of Today or Updates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProducerKind {
    /// Open, done, and retired patient concerns and visit nudges.
    Concerns,
    /// Saved background and physician-requested patient chart reviews.
    PatientReviews,
    /// Visit preparation runs, saved briefings, and skipped/ended visit notes.
    VisitPreparation,
    /// Saved patient summaries and initial chart understanding jobs.
    PatientUnderstanding,
    /// Active physician chat responses currently running.
    RunningConversations,
    /// Per-source refresh statuses and practice schedule source standing.
    SourceStatuses,
}

impl ProducerKind {
    /// Canonical identifier serialized in wire responses.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Concerns => "concerns",
            Self::PatientReviews => "patient_reviews",
            Self::VisitPreparation => "visit_preparation",
            Self::PatientUnderstanding => "patient_understanding",
            Self::RunningConversations => "running_conversations",
            Self::SourceStatuses => "source_statuses",
        }
    }

    /// Physician-facing label naming the part that could not load.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Concerns => "patient concerns",
            Self::PatientReviews => "patient reviews",
            Self::VisitPreparation => "visit briefings",
            Self::PatientUnderstanding => "patient summaries",
            Self::RunningConversations => "active conversations",
            Self::SourceStatuses => "connected source statuses",
        }
    }
}

impl fmt::Display for ProducerKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// One producer failure recorded during a Today or Updates read without failing
/// healthy sibling producers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProducerFailure {
    /// Which producer failed to answer.
    pub producer: ProducerKind,
    /// Diagnostic detail describing why this part failed.
    pub message: String,
}

/// Whether every producer answered a Today or Updates read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum ReadCompleteness {
    /// Every producer answered cleanly.
    Complete,
    /// At least one producer failed; healthy producers' rows still accompany the response.
    Partial {
        /// Every producer that failed on this read, in deterministic order.
        failed_parts: Vec<ProducerFailure>,
    },
}

impl ReadCompleteness {
    /// Builds completeness from any producer failures recorded during a read.
    #[must_use]
    pub fn from_failures(mut failed_parts: Vec<ProducerFailure>) -> Self {
        if failed_parts.is_empty() {
            Self::Complete
        } else {
            failed_parts.sort_by_key(|part| part.producer);
            Self::Partial { failed_parts }
        }
    }

    /// Whether every producer answered without error.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Failed producer parts, or an empty slice when complete.
    #[must_use]
    pub fn failed_parts(&self) -> &[ProducerFailure] {
        match self {
            Self::Complete => &[],
            Self::Partial { failed_parts } => failed_parts,
        }
    }
}

/// Optional fault-injection probe for exercising single-producer and item-open
/// failures in integration and browser tests without synthetic branches in
/// production queries.
pub trait TodayUpdatesProbe: Send + Sync {
    /// Called before reading `producer`; returning `Err` fails only that producer.
    fn check_producer(&self, _producer: ProducerKind) -> Result<(), String> {
        Ok(())
    }

    /// Called before opening `item_key` in `workspace`; returning `Err` fails the open.
    fn check_open_item(&self, _workspace: &str, _item_key: &str) -> Result<(), String> {
        Ok(())
    }
}

/// Default no-op probe used by production read and open entry points.
pub struct NoopTodayUpdatesProbe;

impl TodayUpdatesProbe for NoopTodayUpdatesProbe {}

/// Failures that prevent authorizing or executing a Today/Updates operation.
#[derive(Debug, thiserror::Error)]
pub enum TodayUpdatesError {
    /// One of the required identifiers (`token`, `workspace`, `item_key`, or `command_key`) was blank.
    #[error("{field} must not be empty")]
    InvalidInput {
        /// Name of the empty parameter.
        field: &'static str,
    },
    /// Bearer session could not be verified.
    #[error("physician session is invalid: {0:?}")]
    Session(SessionError),
    /// Authority store could not answer practice clock or roster lookups.
    #[error("authority store unavailable: {0}")]
    AuthUnavailable(#[from] StoreUnavailable),
    /// Physician is not permitted to read or act on the requested workspace.
    #[error("workspace access denied: {reason}")]
    AccessDenied {
        /// Why workspace access was refused.
        reason: String,
    },
    /// Requested item does not exist in the authorized workspace.
    #[error("item `{item_key}` was not found in workspace `{workspace}`")]
    NotFound {
        /// Patient workspace identifier.
        workspace: String,
        /// Item key requested.
        item_key: String,
    },
    /// Requested item cannot be marked `Done` (for example a quiet review or running chat).
    #[error("item `{item_key}` cannot be cleared as Done")]
    CannotMarkDone {
        /// Item key requested.
        item_key: String,
    },
    /// Reused command key carried a different mutation payload.
    #[error("idempotency key `{key}` was already used for a different command")]
    IdempotencyCollision {
        /// Colliding command key.
        key: String,
    },
    /// Opening an item's detail failed (either storage error or injected probe failure).
    #[error("failed to open item `{item_key}`: {detail}")]
    ItemOpenFailed {
        /// Item key that failed to open.
        item_key: String,
        /// Failure detail.
        detail: String,
    },
    /// Underlying concern input validation failed.
    #[error("{0}")]
    ConcernInput(#[from] ConcernInputError),
    /// Underlying concern lifecycle operation failed.
    #[error("{0}")]
    Concern(#[source] ConcernError),
    /// Postgres refused or failed an operation outside partial producer reads.
    #[error("{0}")]
    Database(#[from] DbErr),
}

impl From<SessionError> for TodayUpdatesError {
    fn from(error: SessionError) -> Self {
        Self::Session(error)
    }
}

impl From<ConcernError> for TodayUpdatesError {
    fn from(error: ConcernError) -> Self {
        match error {
            ConcernError::AccessDenied { reason } => Self::AccessDenied { reason },
            ConcernError::IdempotencyCollision { key } => Self::IdempotencyCollision { key },
            other => Self::Concern(other),
        }
    }
}

/// Stable practice-within-physician seen-state key identifying a Today or Updates item.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SeenItemKey(String);

impl SeenItemKey {
    /// Validates a non-empty item key.
    pub fn new(raw: impl Into<String>) -> Result<Self, TodayUpdatesError> {
        let value = raw.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(TodayUpdatesError::InvalidInput { field: "item_key" });
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Borrowed string representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SeenItemKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Content snapshot persisted when a physician opens an item so later revisions
/// can report specifically what changed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeenItemSnapshot {
    /// Monotonic version token for the item (`rev:<n>` or status-timestamp version).
    pub version_id: String,
    /// Urgency when the item was seen, for concerns (`now` vs `today`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urgency: Option<ConcernUrgency>,
    /// Short clinical headline at the time the physician opened the item.
    pub headline: String,
    /// Clinical reason or summary body at the time the physician opened the item.
    pub detail: String,
}

/// Per-physician seen standing of a Today or Updates item.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum SeenStanding {
    /// This physician has not opened the item yet.
    Unseen,
    /// This physician opened the current version of the item at `seen_at`.
    Seen {
        /// Unix second when the physician opened this version.
        seen_at: i64,
    },
    /// This physician opened an earlier version at `seen_at`, and the item has
    /// changed since then; returns as new with `what_changed`.
    ChangedSinceSeen {
        /// Unix second when the physician last opened the earlier version.
        seen_at: i64,
        /// Physician-facing summary of what changed since `seen_at`.
        what_changed: String,
    },
}

impl SeenStanding {
    /// Whether this item has not yet been seen at its current version (`Unseen`
    /// or `ChangedSinceSeen`).
    #[must_use]
    pub const fn needs_physician_eye(&self) -> bool {
        match self {
            Self::Unseen | Self::ChangedSinceSeen { .. } => true,
            Self::Seen { .. } => false,
        }
    }

    /// Change note when the item changed after being seen.
    #[must_use]
    pub fn what_changed(&self) -> Option<&str> {
        match self {
            Self::ChangedSinceSeen { what_changed, .. } => Some(what_changed.as_str()),
            Self::Unseen | Self::Seen { .. } => None,
        }
    }
}

#[derive(Clone, Debug)]
struct StoredSeenEntry {
    seen_version: String,
    snapshot: SeenItemSnapshot,
    seen_at: i64,
}

/// Single-owner comparison deciding whether an item is `Unseen`, `Seen`, or
/// `ChangedSinceSeen` for one physician.
fn evaluate_seen_standing(
    stored: Option<&StoredSeenEntry>,
    current: &SeenItemSnapshot,
) -> SeenStanding {
    let Some(stored) = stored else {
        return SeenStanding::Unseen;
    };
    if stored.seen_version == current.version_id && stored.snapshot == *current {
        return SeenStanding::Seen {
            seen_at: stored.seen_at,
        };
    }

    let mut changes = Vec::new();
    if stored.snapshot.urgency != current.urgency
        && let Some(new_urgency) = current.urgency
    {
        let label = match new_urgency {
            ConcernUrgency::Now => "Urgency raised to Now",
            ConcernUrgency::Today => "Urgency changed to Today",
        };
        changes.push(label.to_owned());
    }
    if stored.snapshot.detail != current.detail {
        changes.push(format!(
            "Updated finding: {}",
            summarize_one_line(&current.detail, 140)
        ));
    } else if stored.snapshot.headline != current.headline {
        changes.push(format!(
            "Updated: {}",
            summarize_one_line(&current.headline, 140)
        ));
    }
    if changes.is_empty() {
        changes.push(format!("Updated ({})", current.version_id));
    }

    SeenStanding::ChangedSinceSeen {
        seen_at: stored.seen_at,
        what_changed: changes.join("; "),
    }
}

/// Pre-visit appointment slot attached to a visit-scoped Today concern.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodayVisitSlot {
    /// Schedule source identifier.
    pub source_id: String,
    /// Schedule appointment identifier.
    pub appointment_id: String,
    /// Visit start time in Unix seconds.
    pub starts_at_unix: i64,
    /// Visit end time in Unix seconds.
    pub ends_at_unix: i64,
}

/// Which kind of non-restartable background work left unhandled clinical items on Today.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnhandledWorkKind {
    /// Failed or held patient review with unresolved topics or physician request.
    PatientReview,
    /// Pre-visit preparation that could not finish before the upcoming visit.
    VisitPreparation,
    /// Initial patient understanding job that failed before producing a summary.
    InitialUnderstanding,
}

/// Which kind of physician-recognizable running work appears on Today.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecognizableRunKind {
    /// An active physician chat turn writing a response.
    PhysicianChatResponse,
    /// A patient review explicitly requested by a physician.
    PhysicianRequestedReview,
}

/// Kind-specific payload of a row on Today.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TodayRowPayload {
    /// An open clinical concern or pre-visit nudge (`Now` or `Today`).
    OpenConcern {
        /// Durable concern identifier.
        concern_id: ConcernId,
        /// `Now` or `Today` urgency.
        urgency: ConcernUrgency,
        /// Linked visit slot when raised by visit preparation.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        visit_slot: Option<TodayVisitSlot>,
        /// Number of cited chart records backing the concern.
        cited_record_count: usize,
    },
    /// Non-restartable failed or held work carrying unhandled clinical items.
    UnhandledWork {
        /// Producer kind of the failed work.
        work_kind: UnhandledWorkKind,
        /// Durable work or review identifier.
        work_id: String,
        /// Unfinished topics or failure details needing physician awareness.
        unfinished_topics: Vec<String>,
    },
    /// Running work initiated by a physician that has not yet settled.
    RunningWork {
        /// Whether this is an active chat response or physician-requested review.
        run_kind: RecognizableRunKind,
        /// Durable work identifier.
        work_id: String,
        /// Conversation identifier when `run_kind` is `PhysicianChatResponse`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        chat_id: Option<String>,
    },
}

/// One flat row on Today for an authorized patient workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodayRow {
    /// Seen-state item key (`concern:...`, `unhandled-...`, `running-...`).
    pub item_key: SeenItemKey,
    /// Current version token used for `SeenStanding` and `Done` dismissal fencing.
    pub version_id: String,
    /// Authorized patient workspace identifier.
    pub workspace: String,
    /// Physician-facing patient display name.
    pub patient_display_name: String,
    /// Concise row title naming what needs the physician.
    pub title: String,
    /// Supporting clinical reason or unhandled-work explanation.
    pub detail: String,
    /// Unix second since which this item has been waiting on Today.
    pub waiting_since_unix: i64,
    ///Unix second of the latest update on this item.
    pub updated_at_unix: i64,
    /// Per-physician seen state (`Unseen`, `Seen`, or `ChangedSinceSeen`).
    pub seen_standing: SeenStanding,
    /// Whether this row offers `Done` (`OpenConcern` and `UnhandledWork`).
    pub can_mark_done: bool,
    /// Typed Today row payload.
    pub payload: TodayRowPayload,
}

impl TodayRow {
    /// Whether this row has `Now` urgency.
    #[must_use]
    pub const fn is_now(&self) -> bool {
        matches!(
            &self.payload,
            TodayRowPayload::OpenConcern {
                urgency: ConcernUrgency::Now,
                ..
            }
        )
    }

    fn sort_rank(&self) -> u8 {
        match &self.payload {
            TodayRowPayload::OpenConcern {
                urgency: ConcernUrgency::Now,
                ..
            } => {
                if self.seen_standing.needs_physician_eye() {
                    0
                } else {
                    1
                }
            }
            TodayRowPayload::OpenConcern {
                urgency: ConcernUrgency::Today,
                ..
            }
            | TodayRowPayload::UnhandledWork { .. }
            | TodayRowPayload::RunningWork { .. } => 2,
        }
    }
}

/// One collapsed practice-wide outage line on Today when an entire connected
/// source is failing across the practice instead of repeating a row per patient.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticeOutageLine {
    /// Stable connected source identifier.
    pub source_id: String,
    /// Physician-facing source label.
    pub source_label: String,
    /// Number of workspaces on the physician's panel affected by this source outage.
    pub affected_workspace_count: usize,
    /// Earliest failure timestamp across the current outage, in Unix seconds.
    pub since_unix: i64,
    /// Concise failure summary naming what could not update.
    pub failure_detail: String,
}

/// Durable read-model projection for Today (either practice-wide across the
/// physician's panel or scoped to one patient workspace).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodayProjection {
    /// Practice clinical `"now"` in Unix seconds when the read ran.
    pub clinical_now_unix: i64,
    /// Flat ordered Today rows: unopened/changed `Now` first, then seen `Now`,
    /// then remaining Today rows ordered by how long they have waited (`waiting_since_unix` ascending).
    pub rows: Vec<TodayRow>,
    /// Practice-wide source outage lines (one per down practice source).
    pub practice_outages: Vec<PracticeOutageLine>,
    /// Distinct authorized patients Coda quietly reviewed today with nothing needing the physician.
    pub also_reviewed_today_count: usize,
    /// Whether every producer answered or one or more failed.
    pub completeness: ReadCompleteness,
}

impl TodayProjection {
    /// Single-owner predicate: `"Nothing needs you"` holds strictly when every
    /// producer answered cleanly and neither Today rows nor practice outages exist.
    #[must_use]
    pub fn nothing_needs_physician(&self) -> bool {
        self.completeness.is_complete() && self.rows.is_empty() && self.practice_outages.is_empty()
    }
}

/// One active background run surfaced in Updates and the collapsed orb preview.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveUpdateRun {
    /// Stable run identifier.
    pub run_id: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Patient display name.
    pub patient_display_name: String,
    /// Concise description of what Coda is currently reviewing or preparing.
    pub label: String,
    /// Unix second when the run started or was queued.
    pub started_at_unix: i64,
}

/// Typed kind of a completed entry in Updates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UpdateEntryKind {
    /// Completed patient review that recorded clinical findings or state updates.
    AssessedReview {
        /// Saved review identifier.
        review_id: String,
    },
    /// Completed patient review with controlled quiet disposition (`"nothing to do"`).
    QuietReview {
        /// Saved review identifier.
        review_id: String,
        /// Controlled quiet completion reason (`"nothing to do"`).
        quiet_reason: String,
    },
    /// Saved patient summary revision on the patient page.
    PatientSummary {
        /// Summary revision number.
        revision: u64,
    },
    /// Saved pre-visit briefing prepared ahead of an appointment.
    VisitBriefing {
        /// Durable briefing identifier.
        briefing_id: String,
        /// Scheduled appointment start in Unix seconds.
        starts_at_unix: i64,
    },
    /// Concern raised (and currently open or cleared `Done` by a physician).
    ConcernRaised {
        /// Durable concern identifier.
        concern_id: ConcernId,
        /// `Now` or `Today` urgency.
        urgency: ConcernUrgency,
        /// `"open"` or `"done"`.
        standing_label: String,
    },
    /// Concern retired by Coda with a single-line clinical reason.
    ConcernRetired {
        /// Durable concern identifier.
        concern_id: ConcernId,
        /// Single-line retirement reason.
        retirement_reason: String,
    },
    /// Background work (such as visit preparation) that ended with nothing to do and why.
    EndedNothingToDo {
        /// Short label of the work (`"Visit preparation"`).
        work_label: String,
        /// Clinical reason why nothing was needed.
        reason: String,
    },
}

/// One completed entry in Updates across any producer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateEntry {
    /// Seen-state item key.
    pub item_key: SeenItemKey,
    /// Current version token.
    pub version_id: String,
    /// Authorized patient workspace identifier.
    pub workspace: String,
    /// Physician-facing patient display name.
    pub patient_display_name: String,
    /// Concise update headline.
    pub title: String,
    /// Supporting clinical summary or one-line retirement/quiet reason.
    pub detail: String,
    /// Unix second when this update occurred.
    pub occurred_at_unix: i64,
    /// Per-physician seen standing (`Unseen`, `Seen`, or `ChangedSinceSeen`).
    pub seen_standing: SeenStanding,
    /// Producer-specific metadata.
    pub kind: UpdateEntryKind,
}

/// Earlier calendar day bucket in Updates, ordered newest day first.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EarlierDayUpdatesGroup {
    /// ISO calendar date (`YYYY-MM-DD`, UTC).
    pub day_iso: String,
    /// Physician-facing day heading (for example `"Yesterday"` or `"Nov 10, 2026"`).
    pub day_label: String,
    /// Updates recorded on `day_iso`, newest first.
    pub items: Vec<UpdateEntry>,
}

/// Durable read-model projection for Updates across a physician's authorized panel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdatesProjection {
    /// Practice clinical `"now"` in Unix seconds when the read ran.
    pub clinical_now_unix: i64,
    /// Active background reviews, visit preparations, and initial understanding runs.
    pub active_runs: Vec<ActiveUpdateRun>,
    /// Today's non-quiet updates (what Coda wrote, raised, retired, or skipped with reason), newest first.
    pub today_items: Vec<UpdateEntry>,
    /// Today's quiet patient reviews grouped under `"Reviewed, nothing for you"`, newest first.
    pub reviewed_nothing_for_you: Vec<UpdateEntry>,
    /// Updates from earlier calendar days, grouped newest day first.
    pub earlier_days: Vec<EarlierDayUpdatesGroup>,
    /// Whether every producer answered or one or more failed.
    pub completeness: ReadCompleteness,
}

/// Verified chart record citation attached to [`ItemWorkEvidence`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemEvidenceCitation {
    /// Exact `(candidate_id, revision)` record identity.
    pub reference: ExactRecordReference,
    /// Originating source identifier.
    pub source_id: String,
    /// Human-readable source label.
    pub source_label: String,
    /// Display title of the cited record.
    pub display_label: String,
    /// Primary record date (`YYYY-MM-DD`), when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub as_of: Option<String>,
    /// Bounded readable excerpt from the verified chart record.
    pub excerpt: String,
    /// Full formatted readable source record opened when inspecting the citation.
    #[serde(default)]
    pub body: String,
    /// Native or external EHR link when provenance supplies one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_uri: Option<String>,
}

/// Full clinical work and evidence returned when opening a Today row or Update item.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemWorkEvidence {
    /// Item key that was opened and recorded as seen for this physician.
    pub item_key: SeenItemKey,
    /// Current version token recorded as seen.
    pub version_id: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Physician-facing patient display name.
    pub patient_display_name: String,
    /// Human-readable category label (`"Urgent concern"`, `"Visit briefing"`, `"Quiet review"`, etc.).
    pub kind_label: String,
    /// Primary clinical headline.
    pub headline: String,
    /// Full clinical explanation, narrative, or unhandled work details.
    pub clinical_body: String,
    /// Dated basis and explicit limitations when recorded by the producer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basis_and_limitations: Option<String>,
    /// Verified chart citations backing this work.
    pub citations: Vec<ItemEvidenceCitation>,
    /// Originating work provenance label (for example `"Patient review rev-1"`).
    pub origin_summary: String,
    /// Linked conversation identifier when opening an active chat item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// Seen standing after recording this physician's open (`Seen`).
    pub seen_standing: SeenStanding,
}

/// Typed request parameters for opening a Today row or Updates item.
pub struct OpenItemRequest<'a> {
    /// Bearer session token of the signed-in physician.
    pub token: &'a str,
    /// Current operational wall-clock Unix second.
    pub real_now_unix: u64,
    /// Target patient workspace identifier.
    pub workspace: &'a str,
    /// Item key to open and mark seen.
    pub item_key: &'a SeenItemKey,
    /// Once-only idempotency key for any visit-concern open reconciliation.
    pub command_key: &'a CommandKey,
    /// Optional workspace-to-display-name directory overrides.
    pub patient_names: &'a BTreeMap<String, String>,
}

/// Typed request parameters for clearing a Today row as `Done`.
pub struct ClearItemDoneRequest<'a> {
    /// Bearer session token of the signed-in physician.
    pub token: &'a str,
    /// Current operational wall-clock Unix second.
    pub real_now_unix: u64,
    /// Target patient workspace identifier.
    pub workspace: &'a str,
    /// Item key to clear (`concern:...` or `unhandled-...`).
    pub item_key: &'a SeenItemKey,
    /// Current version token of the item being cleared.
    pub version_id: &'a str,
    /// Once-only idempotency key for the `Done` command.
    pub command_key: &'a CommandKey,
}

/// Outcome of clearing a Today row as `Done`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum TodayDoneResult {
    /// Newly cleared (either an open concern or an unhandled non-restartable work item).
    Cleared {
        /// Cleared item key.
        item_key: SeenItemKey,
    },
    /// Exact idempotent command key was already accepted earlier.
    Duplicate {
        /// Cleared item key.
        item_key: SeenItemKey,
    },
    /// Item was already cleared by an earlier command.
    AlreadyCleared {
        /// Cleared item key.
        item_key: SeenItemKey,
    },
}

struct AuthorizedReadContext {
    identity: PhysicianIdentity,
    clinical_now_unix: i64,
    allowed_workspaces: Vec<String>,
    practice_workspaces: Vec<String>,
}

fn summarize_one_line(text: &str, max_chars: usize) -> String {
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() <= max_chars {
        return collapsed;
    }
    let prefix: String = collapsed.chars().take(max_chars).collect();
    format!("{}…", prefix.trim_end())
}

fn utc_day_number(unix_seconds: i64) -> i64 {
    unix_seconds.div_euclid(SECONDS_PER_DAY)
}

fn is_physician_requested_review(triggers: &[ReviewTrigger]) -> bool {
    triggers
        .iter()
        .any(|trigger| matches!(trigger, ReviewTrigger::PhysicianRequest { .. }))
}

/// Single-owner rule deciding whether a failed or held review creates an
/// unhandled Today row: quiet reviews, physician-stopped (`ReviewHoldReason::Stopped`)
/// reviews, and failed/limit-held background reviews without `unfinished_work`
/// create no Today row.
fn review_creates_unhandled_today_row(
    triggers: &[ReviewTrigger],
    unfinished_work: &[UnfinishedReviewWork],
    state: &ReviewLifecycleState,
) -> bool {
    match state {
        ReviewLifecycleState::Failed { .. }
        | ReviewLifecycleState::Held {
            reason: ReviewHoldReason::Limit,
            ..
        } => !unfinished_work.is_empty() || is_physician_requested_review(triggers),
        ReviewLifecycleState::Held {
            reason: ReviewHoldReason::Stopped,
            ..
        }
        | ReviewLifecycleState::Pending
        | ReviewLifecycleState::Running { .. }
        | ReviewLifecycleState::Completed { .. }
        | ReviewLifecycleState::Superseded { .. } => false,
    }
}

fn compute_unhandled_group_version(constituents: &[TodayRow]) -> String {
    let mut buf = Vec::new();
    for row in constituents {
        buf.extend_from_slice(row.item_key.as_str().as_bytes());
        buf.push(b'@');
        buf.extend_from_slice(row.version_id.as_bytes());
        buf.push(b'\n');
    }
    let hex = crate::sha256_hex(&buf);
    format!("unhandled-v1:{}", &hex[..16])
}

/// Collapses constituent non-restartable unhandled items to at most one Today
/// row per patient workspace (`unhandled:<workspace>`), ordered by earliest
/// `waiting_since_unix` with a deterministic version over active constituents.
fn collapse_unhandled_today_rows(
    rows: Vec<TodayRow>,
    seen_map: &BTreeMap<String, StoredSeenEntry>,
    dismissals: &BTreeSet<(String, String, String)>,
) -> Vec<TodayRow> {
    let mut out = Vec::with_capacity(rows.len());
    let mut unhandled_by_ws: BTreeMap<String, Vec<TodayRow>> = BTreeMap::new();
    for row in rows {
        if matches!(row.payload, TodayRowPayload::UnhandledWork { .. }) {
            unhandled_by_ws
                .entry(row.workspace.clone())
                .or_default()
                .push(row);
        } else {
            out.push(row);
        }
    }

    for (workspace, mut group) in unhandled_by_ws {
        group.sort_by(|left, right| {
            left.waiting_since_unix
                .cmp(&right.waiting_since_unix)
                .then_with(|| left.item_key.cmp(&right.item_key))
        });
        let combined_key = format!("unhandled:{workspace}");
        let version_id = compute_unhandled_group_version(&group);
        if dismissals.contains(&(workspace.clone(), combined_key.clone(), version_id.clone())) {
            continue;
        }
        let waiting_since_unix = group[0].waiting_since_unix;
        let updated_at_unix = group
            .iter()
            .map(|r| r.updated_at_unix)
            .max()
            .unwrap_or(waiting_since_unix);
        let patient_display_name = group[0].patient_display_name.clone();

        let (title, detail, payload) = if group.len() == 1 {
            let single = group.into_iter().next().expect("single constituent");
            (single.title, single.detail, single.payload)
        } else {
            let title = format!("{} unhandled items couldn't finish", group.len());
            let detail = group
                .iter()
                .map(|r| r.detail.as_str())
                .collect::<Vec<_>>()
                .join(" · ");
            let mut combined_topics = Vec::new();
            let mut first_kind = UnhandledWorkKind::PatientReview;
            let mut first_work_id = String::new();
            for (idx, item) in group.into_iter().enumerate() {
                if let TodayRowPayload::UnhandledWork {
                    work_kind,
                    work_id,
                    mut unfinished_topics,
                } = item.payload
                {
                    if idx == 0 {
                        first_kind = work_kind;
                        first_work_id = work_id;
                    }
                    combined_topics.append(&mut unfinished_topics);
                }
            }
            (
                title,
                detail,
                TodayRowPayload::UnhandledWork {
                    work_kind: first_kind,
                    work_id: first_work_id,
                    unfinished_topics: combined_topics,
                },
            )
        };

        let snapshot = SeenItemSnapshot {
            version_id: version_id.clone(),
            urgency: None,
            headline: title.clone(),
            detail: detail.clone(),
        };
        let seen_standing = evaluate_seen_standing(seen_map.get(&combined_key), &snapshot);
        out.push(TodayRow {
            item_key: SeenItemKey(combined_key),
            version_id,
            workspace,
            patient_display_name,
            title,
            detail,
            waiting_since_unix,
            updated_at_unix,
            seen_standing,
            can_mark_done: true,
            payload,
        });
    }

    out
}

impl Store {
    async fn authorize_physician_read(
        &self,
        auth: &dyn AuthStore,
        token: &str,
        real_now_unix: u64,
        workspace_filter: Option<&str>,
    ) -> Result<AuthorizedReadContext, TodayUpdatesError> {
        let trimmed_token = token.trim();
        if trimmed_token.is_empty() {
            return Err(TodayUpdatesError::InvalidInput { field: "token" });
        }
        let session = auth.verified_session(trimmed_token, real_now_unix).await?;
        let identity = session.identity;
        let practice_clock = auth.practice_clock(&identity.practice_id).await?;
        let clinical_now_u64 = practice_clock.now(real_now_unix);
        let clinical_now_unix =
            i64::try_from(clinical_now_u64).map_err(conversion_error("clinical now", "i64"))?;
        let practice_workspaces = auth.practice_patients(&identity.practice_id).await?;

        let allowed_workspaces = if let Some(target_ws) = workspace_filter {
            let trimmed_ws = target_ws.trim();
            if trimmed_ws.is_empty() {
                return Err(TodayUpdatesError::InvalidInput { field: "workspace" });
            }
            match auth.membership(&identity, trimmed_ws).await {
                Membership::Allowed => vec![trimmed_ws.to_owned()],
                Membership::ClinicDemographic => {
                    return Err(TodayUpdatesError::AccessDenied {
                        reason: format!(
                            "patient workspace `{trimmed_ws}` is outside physician `{}`'s panel",
                            identity.physician_id
                        ),
                    });
                }
                Membership::Denied => {
                    return Err(TodayUpdatesError::AccessDenied {
                        reason: format!(
                            "physician `{}` may not access workspace `{trimmed_ws}`",
                            identity.physician_id
                        ),
                    });
                }
            }
        } else {
            let panel = auth.panel(&identity).await;
            let mut verified = Vec::with_capacity(panel.len());
            for ws in panel {
                if auth.membership(&identity, &ws).await == Membership::Allowed {
                    verified.push(ws);
                }
            }
            verified
        };

        Ok(AuthorizedReadContext {
            identity,
            clinical_now_unix,
            allowed_workspaces,
            practice_workspaces,
        })
    }

    async fn resolve_patient_names(
        &self,
        workspaces: &[String],
        supplied_names: &BTreeMap<String, String>,
    ) -> BTreeMap<String, String> {
        let mut names = BTreeMap::new();
        for ws in workspaces {
            if let Some(name) = supplied_names.get(ws)
                && !name.trim().is_empty()
            {
                names.insert(ws.clone(), name.clone());
            }
        }
        let missing: Vec<String> = workspaces
            .iter()
            .filter(|ws| !names.contains_key(*ws))
            .cloned()
            .collect();
        if !missing.is_empty()
            && let Ok(rows) = source_pull::Entity::find()
                .filter(source_pull::Column::Workspace.is_in(missing.iter().map(String::as_str)))
                .order_by_desc(source_pull::Column::PulledAt)
                .all(&self.db)
                .await
        {
            for row in rows {
                if names.contains_key(&row.workspace) {
                    continue;
                }
                if let Ok(patient) = serde_json::from_value::<SourcePatient>(row.patient)
                    && !patient.display_name.trim().is_empty()
                {
                    names.insert(row.workspace, patient.display_name);
                }
            }
        }
        for ws in workspaces {
            names.entry(ws.clone()).or_insert_with(|| ws.clone());
        }
        names
    }

    async fn load_physician_seen_map(
        &self,
        practice: &str,
        physician: &str,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, StoredSeenEntry>, DbErr> {
        if workspaces.is_empty() {
            return Ok(BTreeMap::new());
        }
        let rows = today_updates_seen::Entity::find()
            .filter(today_updates_seen::Column::Practice.eq(practice))
            .filter(today_updates_seen::Column::Physician.eq(physician))
            .filter(
                today_updates_seen::Column::Workspace.is_in(workspaces.iter().map(String::as_str)),
            )
            .all(&self.db)
            .await?;
        let mut map = BTreeMap::new();
        for row in rows {
            let snapshot: SeenItemSnapshot = serde_json::from_value(row.seen_snapshot)
                .map_err(conversion_error("seen_snapshot", "SeenItemSnapshot"))?;
            map.insert(
                row.item_key,
                StoredSeenEntry {
                    seen_version: row.seen_version,
                    snapshot,
                    seen_at: row.seen_at,
                },
            );
        }
        Ok(map)
    }

    async fn load_work_dismissals(
        &self,
        practice: &str,
        workspaces: &[String],
    ) -> Result<BTreeSet<(String, String, String)>, DbErr> {
        if workspaces.is_empty() {
            return Ok(BTreeSet::new());
        }
        let rows = today_work_dismissal::Entity::find()
            .filter(today_work_dismissal::Column::Practice.eq(practice))
            .filter(
                today_work_dismissal::Column::Workspace
                    .is_in(workspaces.iter().map(String::as_str)),
            )
            .all(&self.db)
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| (row.workspace, row.item_key, row.version_id))
            .collect())
    }

    /// Reads the Today projection across every patient workspace currently on
    /// the signed-in physician's panel.
    pub async fn read_today(
        &self,
        auth: &dyn AuthStore,
        token: &str,
        real_now_unix: u64,
        patient_names: &BTreeMap<String, String>,
    ) -> Result<TodayProjection, TodayUpdatesError> {
        self.read_today_with_probe(
            auth,
            token,
            real_now_unix,
            None,
            patient_names,
            &NoopTodayUpdatesProbe,
        )
        .await
    }

    /// Reads the Today projection scoped strictly to one authorized patient
    /// workspace for that patient's page.
    pub async fn read_today_for_workspace(
        &self,
        auth: &dyn AuthStore,
        token: &str,
        real_now_unix: u64,
        workspace: &str,
        patient_names: &BTreeMap<String, String>,
    ) -> Result<TodayProjection, TodayUpdatesError> {
        self.read_today_with_probe(
            auth,
            token,
            real_now_unix,
            Some(workspace),
            patient_names,
            &NoopTodayUpdatesProbe,
        )
        .await
    }

    /// Reads Today across the physician's panel (or one workspace) using `probe`
    /// so tests can force individual producer failures cleanly.
    pub async fn read_today_with_probe(
        &self,
        auth: &dyn AuthStore,
        token: &str,
        real_now_unix: u64,
        workspace_filter: Option<&str>,
        patient_names: &BTreeMap<String, String>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<TodayProjection, TodayUpdatesError> {
        let ctx = self
            .authorize_physician_read(auth, token, real_now_unix, workspace_filter)
            .await?;
        let names = self
            .resolve_patient_names(&ctx.allowed_workspaces, patient_names)
            .await;
        let seen_map = self
            .load_physician_seen_map(
                &ctx.identity.practice_id,
                &ctx.identity.physician_id,
                &ctx.allowed_workspaces,
            )
            .await?;
        let dismissals = self
            .load_work_dismissals(&ctx.identity.practice_id, &ctx.allowed_workspaces)
            .await?;

        let mut rows = Vec::new();
        let mut failed_parts = Vec::new();
        let mut quiet_reviewed_workspaces_today = BTreeSet::new();
        let mut practice_outages = Vec::new();

        // 1. Concerns & pre-visit nudges.
        match self
            .read_today_concerns_slice(&ctx, &names, &seen_map, probe)
            .await
        {
            Ok(mut concern_rows) => rows.append(&mut concern_rows),
            Err(message) => failed_parts.push(ProducerFailure {
                producer: ProducerKind::Concerns,
                message,
            }),
        }

        // 2. Patient reviews (quiet count, unhandled failures/holds, physician-requested runs).
        match self
            .read_today_reviews_slice(&ctx, &names, &seen_map, &dismissals, probe)
            .await
        {
            Ok((mut review_rows, quiet_workspaces)) => {
                rows.append(&mut review_rows);
                quiet_reviewed_workspaces_today.extend(quiet_workspaces);
            }
            Err(message) => failed_parts.push(ProducerFailure {
                producer: ProducerKind::PatientReviews,
                message,
            }),
        }

        // 3. Non-restartable visit preparation (`CouldNotFinish` before visit end).
        match self
            .read_today_visit_prep_slice(&ctx, &names, &seen_map, &dismissals, probe)
            .await
        {
            Ok(mut prep_rows) => rows.append(&mut prep_rows),
            Err(message) => failed_parts.push(ProducerFailure {
                producer: ProducerKind::VisitPreparation,
                message,
            }),
        }

        // 4. Exhausted initial understanding runs without a saved summary.
        match self
            .read_today_understanding_slice(&ctx, &names, &seen_map, &dismissals, probe)
            .await
        {
            Ok(mut understanding_rows) => rows.append(&mut understanding_rows),
            Err(message) => failed_parts.push(ProducerFailure {
                producer: ProducerKind::PatientUnderstanding,
                message,
            }),
        }

        // 5. Recognizable physician-initiated active chat responses.
        match self
            .read_today_running_chats_slice(&ctx, &names, &seen_map, probe)
            .await
        {
            Ok(mut chat_rows) => rows.append(&mut chat_rows),
            Err(message) => failed_parts.push(ProducerFailure {
                producer: ProducerKind::RunningConversations,
                message,
            }),
        }

        // 6. Practice-wide source outages (only on practice-wide Today or when workspace is affected).
        match self
            .read_today_source_outages_slice(&ctx, workspace_filter, probe)
            .await
        {
            Ok(outages) => practice_outages = outages,
            Err(message) => failed_parts.push(ProducerFailure {
                producer: ProducerKind::SourceStatuses,
                message,
            }),
        }

        let mut rows = collapse_unhandled_today_rows(rows, &seen_map, &dismissals);
        rows.sort_by(|left, right| {
            left.sort_rank()
                .cmp(&right.sort_rank())
                .then_with(|| left.waiting_since_unix.cmp(&right.waiting_since_unix))
                .then_with(|| left.item_key.cmp(&right.item_key))
        });

        Ok(TodayProjection {
            clinical_now_unix: ctx.clinical_now_unix,
            rows,
            practice_outages,
            also_reviewed_today_count: quiet_reviewed_workspaces_today.len(),
            completeness: ReadCompleteness::from_failures(failed_parts),
        })
    }

    async fn read_today_concerns_slice(
        &self,
        ctx: &AuthorizedReadContext,
        names: &BTreeMap<String, String>,
        seen_map: &BTreeMap<String, StoredSeenEntry>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<Vec<TodayRow>, String> {
        probe.check_producer(ProducerKind::Concerns)?;
        if ctx.allowed_workspaces.is_empty() {
            return Ok(Vec::new());
        }
        self.reconcile_visit_concerns(&ctx.identity.practice_id, ctx.clinical_now_unix)
            .await
            .map_err(|err| err.to_string())?;

        let concern_rows = concern::Entity::find()
            .filter(concern::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                concern::Column::Workspace.is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .filter(concern::Column::Status.eq("open"))
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        let visit_links = visit_concern::Entity::find()
            .filter(visit_concern::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                visit_concern::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;
        let visit_slot_by_concern: BTreeMap<(String, String), TodayVisitSlot> = visit_links
            .into_iter()
            .map(|row| {
                (
                    (row.workspace, row.concern_id),
                    TodayVisitSlot {
                        source_id: row.source_id,
                        appointment_id: row.appointment_id,
                        starts_at_unix: row.starts_at_unix,
                        ends_at_unix: row.ends_at_unix,
                    },
                )
            })
            .collect();

        let mut out = Vec::with_capacity(concern_rows.len());
        for row in concern_rows {
            let urgency = match row.urgency.as_str() {
                "now" => ConcernUrgency::Now,
                "today" => ConcernUrgency::Today,
                other => return Err(format!("invalid concern urgency `{other}`")),
            };
            let concern_id = ConcernId::new(row.id.clone())
                .map_err(|err| format!("invalid concern id: {err}"))?;
            let records: Vec<ExactRecordReference> = serde_json::from_value(row.records)
                .map_err(|err| format!("decode concern records: {err}"))?;
            let visit_slot = visit_slot_by_concern
                .get(&(row.workspace.clone(), row.id.clone()))
                .cloned();
            let key_str = format!("concern:{}:{}", row.workspace, row.id);
            let item_key = SeenItemKey(key_str.clone());
            let version_id = format!("rev:{}", row.revision);
            let base_headline = summarize_one_line(&row.reason, 96);
            let headline = if visit_slot.is_some() {
                format!("Pre-visit: {base_headline}")
            } else {
                base_headline.clone()
            };
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: Some(urgency),
                headline: base_headline,
                detail: row.reason.clone(),
            };
            let seen_standing = evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
            let patient_display_name = names
                .get(&row.workspace)
                .cloned()
                .unwrap_or_else(|| row.workspace.clone());

            out.push(TodayRow {
                item_key,
                version_id,
                workspace: row.workspace,
                patient_display_name,
                title: headline,
                detail: row.reason,
                waiting_since_unix: row.created_at,
                updated_at_unix: row.updated_at,
                seen_standing,
                can_mark_done: true,
                payload: TodayRowPayload::OpenConcern {
                    concern_id,
                    urgency,
                    visit_slot,
                    cited_record_count: records.len(),
                },
            });
        }
        Ok(out)
    }

    async fn read_today_reviews_slice(
        &self,
        ctx: &AuthorizedReadContext,
        names: &BTreeMap<String, String>,
        seen_map: &BTreeMap<String, StoredSeenEntry>,
        dismissals: &BTreeSet<(String, String, String)>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<(Vec<TodayRow>, BTreeSet<String>), String> {
        probe.check_producer(ProducerKind::PatientReviews)?;
        if ctx.allowed_workspaces.is_empty() {
            return Ok((Vec::new(), BTreeSet::new()));
        }
        let review_rows = patient_review::Entity::find()
            .filter(patient_review::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                patient_review::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        let today_day = utc_day_number(ctx.clinical_now_unix);
        let mut out = Vec::new();
        let mut quiet_workspaces = BTreeSet::new();

        for row in review_rows {
            let triggers: Vec<ReviewTrigger> = serde_json::from_value(row.triggers)
                .map_err(|err| format!("decode review triggers: {err}"))?;
            let unfinished_work: Vec<UnfinishedReviewWork> =
                serde_json::from_value(row.unfinished_work)
                    .map_err(|err| format!("decode review unfinished_work: {err}"))?;
            let state: ReviewLifecycleState = serde_json::from_value(row.state)
                .map_err(|err| format!("decode review state: {err}"))?;
            let patient_display_name = names
                .get(&row.workspace)
                .cloned()
                .unwrap_or_else(|| row.workspace.clone());

            match &state {
                ReviewLifecycleState::Completed {
                    outcome: ReviewOutcome::Quiet { .. },
                    completed_at,
                    ..
                } => {
                    if utc_day_number(*completed_at) == today_day {
                        quiet_workspaces.insert(row.workspace);
                    }
                }
                ReviewLifecycleState::Pending | ReviewLifecycleState::Running { .. } => {
                    if is_physician_requested_review(&triggers) {
                        let key_str = format!("running-review:{}:{}", row.workspace, row.review_id);
                        let version_id = format!("{}:{}", state.status_str(), row.updated_at);
                        let title = "Reviewing patient chart as requested".to_owned();
                        let detail =
                            "Coda is reviewing the requested records for this patient.".to_owned();
                        let snapshot = SeenItemSnapshot {
                            version_id: version_id.clone(),
                            urgency: None,
                            headline: title.clone(),
                            detail: detail.clone(),
                        };
                        let seen_standing =
                            evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
                        let started = row.started_at.unwrap_or(row.queued_at);
                        out.push(TodayRow {
                            item_key: SeenItemKey(key_str),
                            version_id,
                            workspace: row.workspace,
                            patient_display_name,
                            title,
                            detail,
                            waiting_since_unix: started,
                            updated_at_unix: row.updated_at,
                            seen_standing,
                            can_mark_done: false,
                            payload: TodayRowPayload::RunningWork {
                                run_kind: RecognizableRunKind::PhysicianRequestedReview,
                                work_id: row.review_id,
                                chat_id: None,
                            },
                        });
                    }
                }
                ReviewLifecycleState::Failed {
                    reason, failed_at, ..
                } => {
                    if review_creates_unhandled_today_row(&triggers, &unfinished_work, &state) {
                        let key_str =
                            format!("unhandled-review:{}:{}", row.workspace, row.review_id);
                        let version_id = format!("failed:{failed_at}:{}", row.updated_at);
                        if dismissals.contains(&(
                            row.workspace.clone(),
                            key_str.clone(),
                            version_id.clone(),
                        )) {
                            continue;
                        }
                        let topics: Vec<String> = unfinished_work
                            .iter()
                            .map(|item| format!("{}: {}", item.topic, item.detail))
                            .collect();
                        let title = "Patient review couldn't finish".to_owned();
                        let detail = if topics.is_empty() {
                            format!("Couldn't finish requested review: {reason}")
                        } else {
                            format!("Unhandled follow-up: {}", topics.join("; "))
                        };
                        let snapshot = SeenItemSnapshot {
                            version_id: version_id.clone(),
                            urgency: None,
                            headline: title.clone(),
                            detail: detail.clone(),
                        };
                        let seen_standing =
                            evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
                        out.push(TodayRow {
                            item_key: SeenItemKey(key_str),
                            version_id,
                            workspace: row.workspace,
                            patient_display_name,
                            title,
                            detail,
                            waiting_since_unix: *failed_at,
                            updated_at_unix: row.updated_at,
                            seen_standing,
                            can_mark_done: true,
                            payload: TodayRowPayload::UnhandledWork {
                                work_kind: UnhandledWorkKind::PatientReview,
                                work_id: row.review_id,
                                unfinished_topics: topics,
                            },
                        });
                    }
                }
                ReviewLifecycleState::Held {
                    reason, held_at, ..
                } => {
                    if review_creates_unhandled_today_row(&triggers, &unfinished_work, &state) {
                        let key_str =
                            format!("unhandled-review:{}:{}", row.workspace, row.review_id);
                        let reason_label = match reason {
                            ReviewHoldReason::Limit => "reached run limit",
                            ReviewHoldReason::Stopped => continue,
                        };
                        let version_id = format!("held:{held_at}:{}", row.updated_at);
                        if dismissals.contains(&(
                            row.workspace.clone(),
                            key_str.clone(),
                            version_id.clone(),
                        )) {
                            continue;
                        }
                        let topics: Vec<String> = unfinished_work
                            .iter()
                            .map(|item| format!("{}: {}", item.topic, item.detail))
                            .collect();
                        let title = format!("Patient review {reason_label} with unhandled items");
                        let detail = if topics.is_empty() {
                            format!("Requested review {reason_label} before finishing.")
                        } else {
                            format!("Unhandled follow-up: {}", topics.join("; "))
                        };
                        let snapshot = SeenItemSnapshot {
                            version_id: version_id.clone(),
                            urgency: None,
                            headline: title.clone(),
                            detail: detail.clone(),
                        };
                        let seen_standing =
                            evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
                        out.push(TodayRow {
                            item_key: SeenItemKey(key_str),
                            version_id,
                            workspace: row.workspace,
                            patient_display_name,
                            title,
                            detail,
                            waiting_since_unix: *held_at,
                            updated_at_unix: row.updated_at,
                            seen_standing,
                            can_mark_done: true,
                            payload: TodayRowPayload::UnhandledWork {
                                work_kind: UnhandledWorkKind::PatientReview,
                                work_id: row.review_id,
                                unfinished_topics: topics,
                            },
                        });
                    }
                }
                ReviewLifecycleState::Completed {
                    outcome: ReviewOutcome::Assessed { .. },
                    ..
                }
                | ReviewLifecycleState::Superseded { .. } => {}
            }
        }
        Ok((out, quiet_workspaces))
    }

    async fn read_today_visit_prep_slice(
        &self,
        ctx: &AuthorizedReadContext,
        names: &BTreeMap<String, String>,
        seen_map: &BTreeMap<String, StoredSeenEntry>,
        dismissals: &BTreeSet<(String, String, String)>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<Vec<TodayRow>, String> {
        probe.check_producer(ProducerKind::VisitPreparation)?;
        if ctx.allowed_workspaces.is_empty() {
            return Ok(Vec::new());
        }
        let prep_rows = visit_preparation::Entity::find()
            .filter(visit_preparation::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                visit_preparation::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .filter(visit_preparation::Column::State.eq("could_not_finish"))
            .filter(visit_preparation::Column::EndsAtUnix.gt(ctx.clinical_now_unix))
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        let mut out = Vec::with_capacity(prep_rows.len());
        for row in prep_rows {
            let key_str = format!(
                "unhandled-visit:{}:{}:{}:{}",
                row.workspace, row.source_id, row.appointment_id, row.starts_at_unix
            );
            let version_id = format!("could_not_finish:{}", row.updated_at);
            if dismissals.contains(&(row.workspace.clone(), key_str.clone(), version_id.clone())) {
                continue;
            }
            let reason = row
                .failed_reason
                .clone()
                .unwrap_or_else(|| "Pre-visit briefing could not finish.".to_owned());
            let title = "Visit briefing couldn't finish".to_owned();
            let detail = format!("Upcoming visit: {reason}");
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: None,
                headline: title.clone(),
                detail: detail.clone(),
            };
            let seen_standing = evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
            let patient_display_name = names
                .get(&row.workspace)
                .cloned()
                .unwrap_or_else(|| row.workspace.clone());
            let work_id = row
                .briefing_job_id
                .clone()
                .unwrap_or_else(|| row.selection_job_id.clone());

            out.push(TodayRow {
                item_key: SeenItemKey(key_str),
                version_id,
                workspace: row.workspace,
                patient_display_name,
                title,
                detail: detail.clone(),
                waiting_since_unix: row.updated_at,
                updated_at_unix: row.updated_at,
                seen_standing,
                can_mark_done: true,
                payload: TodayRowPayload::UnhandledWork {
                    work_kind: UnhandledWorkKind::VisitPreparation,
                    work_id,
                    unfinished_topics: vec![reason],
                },
            });
        }
        Ok(out)
    }

    async fn read_today_understanding_slice(
        &self,
        ctx: &AuthorizedReadContext,
        names: &BTreeMap<String, String>,
        seen_map: &BTreeMap<String, StoredSeenEntry>,
        dismissals: &BTreeSet<(String, String, String)>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<Vec<TodayRow>, String> {
        probe.check_producer(ProducerKind::PatientUnderstanding)?;
        if ctx.allowed_workspaces.is_empty() {
            return Ok(Vec::new());
        }
        let failed_jobs = practice_job::Entity::find()
            .filter(practice_job::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                practice_job::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .filter(practice_job::Column::Kind.eq(PracticeJobKind::InitialUnderstanding))
            .filter(practice_job::Column::Status.is_in([WorkStatus::Failed, WorkStatus::Held]))
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;
        if failed_jobs.is_empty() {
            return Ok(Vec::new());
        }

        let covered_versions: BTreeMap<String, i64> = patient_understanding::Entity::find()
            .select_only()
            .column(patient_understanding::Column::Workspace)
            .column(patient_understanding::Column::SourceVersion)
            .filter(patient_understanding::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                patient_understanding::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .into_tuple::<(String, i64)>()
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?
            .into_iter()
            .collect();

        let mut out = Vec::new();
        for job in failed_jobs {
            if job.sources_version <= 0 {
                continue;
            }
            if let Some(saved_version) = covered_versions.get(&job.workspace)
                && *saved_version >= job.sources_version
            {
                continue;
            }
            let key_str = format!("unhandled-understanding:{}:{}", job.workspace, job.job_id);
            let status_tag = match job.status {
                WorkStatus::Failed => "failed",
                WorkStatus::Held => "held",
                _ => "incomplete",
            };
            let version_id = format!("{status_tag}:{}:{}", job.failures, job.updated_at);
            if dismissals.contains(&(job.workspace.clone(), key_str.clone(), version_id.clone())) {
                continue;
            }
            let title = "Initial patient summary couldn't finish".to_owned();
            let detail = format!(
                "Chart summary for version {} could not complete automatically.",
                job.sources_version
            );
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: None,
                headline: title.clone(),
                detail: detail.clone(),
            };
            let seen_standing = evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
            let patient_display_name = names
                .get(&job.workspace)
                .cloned()
                .unwrap_or_else(|| job.workspace.clone());

            out.push(TodayRow {
                item_key: SeenItemKey(key_str),
                version_id,
                workspace: job.workspace,
                patient_display_name,
                title,
                detail: detail.clone(),
                waiting_since_unix: job.updated_at,
                updated_at_unix: job.updated_at,
                seen_standing,
                can_mark_done: true,
                payload: TodayRowPayload::UnhandledWork {
                    work_kind: UnhandledWorkKind::InitialUnderstanding,
                    work_id: job.job_id,
                    unfinished_topics: vec![detail],
                },
            });
        }
        Ok(out)
    }

    async fn read_today_running_chats_slice(
        &self,
        ctx: &AuthorizedReadContext,
        names: &BTreeMap<String, String>,
        seen_map: &BTreeMap<String, StoredSeenEntry>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<Vec<TodayRow>, String> {
        probe.check_producer(ProducerKind::RunningConversations)?;
        if ctx.allowed_workspaces.is_empty() {
            return Ok(Vec::new());
        }
        let conv_rows = conversation::Entity::find()
            .filter(conversation::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(conversation::Column::Physician.eq(ctx.identity.physician_id.as_str()))
            .filter(
                conversation::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .filter(conversation::Column::Status.is_in([WorkStatus::Pending, WorkStatus::Active]))
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        let mut out = Vec::with_capacity(conv_rows.len());
        for row in conv_rows {
            let chat_id = row.chat.clone();
            let workspace = row.workspace.clone();
            let revision = row.revision;
            let state = control::conversation(row, Vec::new())
                .map_err(|err| format!("decode running conversation: {err}"))?;
            let latest_question = state
                .messages
                .iter()
                .rev()
                .find(|msg| matches!(msg.role, MessageRole::User { .. }))
                .map(|msg| summarize_one_line(&msg.text, 80))
                .unwrap_or_else(|| "Writing response".to_owned());
            let key_str = format!("running-chat:{workspace}:{chat_id}");
            let version_id = format!("rev:{revision}");
            let title = format!("Writing response: {latest_question}");
            let detail = "Coda is answering your question in this conversation.".to_owned();
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: None,
                headline: title.clone(),
                detail: detail.clone(),
            };
            let seen_standing = evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
            let patient_display_name = names
                .get(&workspace)
                .cloned()
                .unwrap_or_else(|| workspace.clone());

            out.push(TodayRow {
                item_key: SeenItemKey(key_str),
                version_id,
                workspace,
                patient_display_name,
                title,
                detail,
                waiting_since_unix: state.last_activity_at,
                updated_at_unix: state.last_activity_at,
                seen_standing,
                can_mark_done: false,
                payload: TodayRowPayload::RunningWork {
                    run_kind: RecognizableRunKind::PhysicianChatResponse,
                    work_id: state.work,
                    chat_id: Some(chat_id),
                },
            });
        }
        Ok(out)
    }

    async fn read_today_source_outages_slice(
        &self,
        ctx: &AuthorizedReadContext,
        workspace_filter: Option<&str>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<Vec<PracticeOutageLine>, String> {
        probe.check_producer(ProducerKind::SourceStatuses)?;
        if ctx.allowed_workspaces.is_empty() || workspace_filter.is_some() {
            // Practice-wide outage lines belong on the practice-wide Today list;
            // individual patient pages already render per-source freshness directly.
            return Ok(Vec::new());
        }

        let allowed_set: BTreeSet<&str> =
            ctx.allowed_workspaces.iter().map(String::as_str).collect();
        let practice_set: BTreeSet<&str> =
            ctx.practice_workspaces.iter().map(String::as_str).collect();

        let status_rows = source_status::Entity::find()
            .filter(source_status::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        let mut by_source: BTreeMap<String, Vec<source_status::Model>> = BTreeMap::new();
        for row in status_rows {
            if practice_set.is_empty() || practice_set.contains(row.workspace.as_str()) {
                by_source
                    .entry(row.source_id.clone())
                    .or_default()
                    .push(row);
            }
        }

        let mut outages = Vec::new();
        for (source_id, entries) in by_source {
            let all_failed =
                !entries.is_empty() && entries.iter().all(|entry| entry.outcome == "failed");
            let practice_wide_coverage = entries.len() >= 2
                || (!practice_set.is_empty() && entries.len() >= practice_set.len());
            let affected_allowed_count = entries
                .iter()
                .filter(|entry| allowed_set.contains(entry.workspace.as_str()))
                .count();

            if all_failed && practice_wide_coverage && affected_allowed_count > 0 {
                let source_label = entries[0].source_label.clone();
                let since_unix = entries
                    .iter()
                    .map(|entry| entry.checked_at)
                    .min()
                    .unwrap_or(ctx.clinical_now_unix);
                let reason = entries
                    .iter()
                    .find_map(|entry| entry.reason.as_deref())
                    .unwrap_or("source connection unavailable");
                outages.push(PracticeOutageLine {
                    source_id,
                    source_label,
                    affected_workspace_count: affected_allowed_count,
                    since_unix,
                    failure_detail: reason.to_owned(),
                });
            }
        }

        let sched_sources = schedule_source::Entity::find()
            .filter(schedule_source::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(schedule_source::Column::Standing.eq("failed"))
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;
        for sched in sched_sources {
            if outages.iter().any(|item| item.source_id == sched.source_id) {
                continue;
            }
            let detail = sched
                .last_failure
                .as_ref()
                .and_then(|val| val.get("message").and_then(Value::as_str))
                .unwrap_or("schedule feed could not refresh");
            outages.push(PracticeOutageLine {
                source_id: sched.source_id,
                source_label: sched.source_label,
                affected_workspace_count: ctx.allowed_workspaces.len(),
                since_unix: sched.as_of.unwrap_or(ctx.clinical_now_unix),
                failure_detail: detail.to_owned(),
            });
        }

        outages.sort_by(|left, right| left.source_id.cmp(&right.source_id));
        Ok(outages)
    }
}
