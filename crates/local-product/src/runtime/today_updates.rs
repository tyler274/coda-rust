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
use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
    sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use super::briefings::ConcernOpenOutcome;
use super::concerns::{
    CommandKey, Concern, ConcernDoneOutcome, ConcernError, ConcernId, ConcernInputError,
    ConcernStanding, ConcernUrgency, ExactRecordReference, SourceWork, WorkspaceAccess,
};
use super::entities::{
    concern, conversation, patient_review, patient_understanding, practice_job, schedule_source,
    source, source_pull, source_status, today_updates_seen, today_work_dismissal, visit_briefing,
    visit_concern, visit_preparation,
};
use super::jobs::PracticeJobKind;
use super::receipt::{self, Receipt};
use super::reviews::{ReviewHoldReason, ReviewLifecycleState};
use super::understanding::{PatientSummary, SummaryCitation};
use super::{Authority, MessageRole, Store, WorkStatus, control, conversion_error};
use crate::patient_reviews::{ReviewCitation, ReviewOutcome, ReviewTrigger, UnfinishedReviewWork};
use crate::physician_sources::{SourcePatient, SourceRevision};
use crate::visit_preparation::VisitPreparationDecision;

const EXCERPT_CHARS: usize = 320;
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
    session_hash: Vec<u8>,
    clinical_now_unix: i64,
    allowed_workspaces: Vec<String>,
    practice_workspaces: Vec<String>,
}

fn sha256_bytes(token: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hasher.finalize().to_vec()
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

fn civil_from_days(days_since_unix_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_unix_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    (year as i32, m as u32, d as u32)
}

fn format_day_iso_and_label(day_number: i64, today_day_number: i64) -> (String, String) {
    let (year, month, day) = civil_from_days(day_number);
    let iso = format!("{year:04}-{month:02}-{day:02}");
    if day_number == today_day_number - 1 {
        return (iso, "Yesterday".to_owned());
    }
    let month_name = match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Day",
    };
    (iso, format!("{month_name} {day}, {year}"))
}

fn is_physician_requested_review(triggers: &[ReviewTrigger]) -> bool {
    triggers
        .iter()
        .any(|trigger| matches!(trigger, ReviewTrigger::PhysicianRequest { .. }))
}

/// Single-owner rule deciding whether a failed or held review creates an
/// unhandled Today row: quiet reviews and failed/held background reviews
/// without `unfinished_work` create no Today row.
fn review_creates_unhandled_today_row(
    triggers: &[ReviewTrigger],
    unfinished_work: &[UnfinishedReviewWork],
    state: &ReviewLifecycleState,
) -> bool {
    match state {
        ReviewLifecycleState::Failed { .. } | ReviewLifecycleState::Held { .. } => {
            !unfinished_work.is_empty() || is_physician_requested_review(triggers)
        }
        ReviewLifecycleState::Pending
        | ReviewLifecycleState::Running { .. }
        | ReviewLifecycleState::Completed { .. }
        | ReviewLifecycleState::Superseded { .. } => false,
    }
}

fn source_work_summary(work: &SourceWork) -> String {
    match work.job.as_str() {
        "patient_review" => "chart review".to_owned(),
        "visit_briefing" | "visit_preparation" => "visit preparation".to_owned(),
        "initial_understanding" | "patient_summary" => "patient summary review".to_owned(),
        "physician_chat" => "physician conversation".to_owned(),
        _ => "clinical review".to_owned(),
    }
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
            session_hash: sha256_bytes(trimmed_token),
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
            let headline = if visit_slot.is_some() {
                format!("Pre-visit: {}", summarize_one_line(&row.reason, 96))
            } else {
                summarize_one_line(&row.reason, 96)
            };
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: Some(urgency),
                headline: headline.clone(),
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
                            ReviewHoldReason::Stopped => "paused",
                            ReviewHoldReason::Limit => "reached run limit",
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
                    source_label: source_label.clone(),
                    affected_workspace_count: affected_allowed_count,
                    since_unix,
                    failure_detail: format!(
                        "{source_label} is unavailable across the practice ({reason})."
                    ),
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
                source_label: sched.source_label.clone(),
                affected_workspace_count: ctx.allowed_workspaces.len(),
                since_unix: sched.as_of.unwrap_or(ctx.clinical_now_unix),
                failure_detail: format!(
                    "{} is unavailable across the practice ({detail}).",
                    sched.source_label
                ),
            });
        }

        outages.sort_by(|left, right| left.source_id.cmp(&right.source_id));
        Ok(outages)
    }

    /// Reads the Updates projection across the signed-in physician's authorized
    /// panel patients, including active runs, today's items, `"Reviewed, nothing
    /// for you"`, and earlier calendar days.
    pub async fn read_updates(
        &self,
        auth: &dyn AuthStore,
        token: &str,
        real_now_unix: u64,
        patient_names: &BTreeMap<String, String>,
    ) -> Result<UpdatesProjection, TodayUpdatesError> {
        self.read_updates_with_probe(
            auth,
            token,
            real_now_unix,
            patient_names,
            &NoopTodayUpdatesProbe,
        )
        .await
    }

    /// Reads Updates using `probe` so tests can force individual producer failures.
    pub async fn read_updates_with_probe(
        &self,
        auth: &dyn AuthStore,
        token: &str,
        real_now_unix: u64,
        patient_names: &BTreeMap<String, String>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<UpdatesProjection, TodayUpdatesError> {
        let ctx = self
            .authorize_physician_read(auth, token, real_now_unix, None)
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

        let mut active_runs = Vec::new();
        let mut completed_entries = Vec::new();
        let mut failed_parts = Vec::new();

        // 1. Concerns (raised + retired with one-line reason).
        match self
            .read_updates_concerns_slice(&ctx, &names, &seen_map, probe)
            .await
        {
            Ok(mut entries) => completed_entries.append(&mut entries),
            Err(message) => failed_parts.push(ProducerFailure {
                producer: ProducerKind::Concerns,
                message,
            }),
        }

        // 2. Patient reviews (active runs + assessed reviews + quiet reviews).
        match self
            .read_updates_reviews_slice(&ctx, &names, &seen_map, probe)
            .await
        {
            Ok((mut runs, mut entries)) => {
                active_runs.append(&mut runs);
                completed_entries.append(&mut entries);
            }
            Err(message) => failed_parts.push(ProducerFailure {
                producer: ProducerKind::PatientReviews,
                message,
            }),
        }

        // 3. Visit preparation (preparing runs + saved briefings + skipped/ended with reason).
        match self
            .read_updates_visit_slice(&ctx, &names, &seen_map, probe)
            .await
        {
            Ok((mut runs, mut entries)) => {
                active_runs.append(&mut runs);
                completed_entries.append(&mut entries);
            }
            Err(message) => failed_parts.push(ProducerFailure {
                producer: ProducerKind::VisitPreparation,
                message,
            }),
        }

        // 4. Patient summaries (active initial understanding + saved summary revisions).
        match self
            .read_updates_understanding_slice(&ctx, &names, &seen_map, probe)
            .await
        {
            Ok((mut runs, mut entries)) => {
                active_runs.append(&mut runs);
                completed_entries.append(&mut entries);
            }
            Err(message) => failed_parts.push(ProducerFailure {
                producer: ProducerKind::PatientUnderstanding,
                message,
            }),
        }

        active_runs.sort_by(|left, right| {
            right
                .started_at_unix
                .cmp(&left.started_at_unix)
                .then_with(|| left.run_id.cmp(&right.run_id))
        });

        completed_entries.sort_by(|left, right| {
            right
                .occurred_at_unix
                .cmp(&left.occurred_at_unix)
                .then_with(|| left.item_key.cmp(&right.item_key))
        });

        let today_day = utc_day_number(ctx.clinical_now_unix);
        let mut today_items = Vec::new();
        let mut reviewed_nothing_for_you = Vec::new();
        let mut earlier_by_day: BTreeMap<i64, Vec<UpdateEntry>> = BTreeMap::new();

        for entry in completed_entries {
            let day = utc_day_number(entry.occurred_at_unix);
            if day >= today_day {
                if matches!(entry.kind, UpdateEntryKind::QuietReview { .. }) {
                    reviewed_nothing_for_you.push(entry);
                } else {
                    today_items.push(entry);
                }
            } else {
                earlier_by_day.entry(day).or_default().push(entry);
            }
        }

        let earlier_days = earlier_by_day
            .into_iter()
            .rev()
            .map(|(day_number, items)| {
                let (day_iso, day_label) = format_day_iso_and_label(day_number, today_day);
                EarlierDayUpdatesGroup {
                    day_iso,
                    day_label,
                    items,
                }
            })
            .collect();

        Ok(UpdatesProjection {
            clinical_now_unix: ctx.clinical_now_unix,
            active_runs,
            today_items,
            reviewed_nothing_for_you,
            earlier_days,
            completeness: ReadCompleteness::from_failures(failed_parts),
        })
    }

    async fn read_updates_concerns_slice(
        &self,
        ctx: &AuthorizedReadContext,
        names: &BTreeMap<String, String>,
        seen_map: &BTreeMap<String, StoredSeenEntry>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<Vec<UpdateEntry>, String> {
        probe.check_producer(ProducerKind::Concerns)?;
        if ctx.allowed_workspaces.is_empty() {
            return Ok(Vec::new());
        }
        self.reconcile_visit_concerns(&ctx.identity.practice_id, ctx.clinical_now_unix)
            .await
            .map_err(|err| err.to_string())?;

        let rows = concern::Entity::find()
            .filter(concern::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                concern::Column::Workspace.is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        let mut entries = Vec::with_capacity(rows.len());
        for row in rows {
            let urgency = match row.urgency.as_str() {
                "now" => ConcernUrgency::Now,
                "today" => ConcernUrgency::Today,
                other => return Err(format!("invalid concern urgency `{other}`")),
            };
            let concern_id = ConcernId::new(row.id.clone())
                .map_err(|err| format!("invalid concern id: {err}"))?;
            let patient_display_name = names
                .get(&row.workspace)
                .cloned()
                .unwrap_or_else(|| row.workspace.clone());

            match row.status.as_str() {
                "retired" => {
                    let retirement_reason = row
                        .retirement_reason
                        .clone()
                        .unwrap_or_else(|| "No longer needed".to_owned());
                    let retired_at = row.retired_at.unwrap_or(row.updated_at);
                    let key_str = format!("concern-retired:{}:{}", row.workspace, row.id);
                    let version_id = format!("rev:{}", row.revision);
                    let title = format!("Retired concern: {}", summarize_one_line(&row.reason, 84));
                    let detail = retirement_reason.clone();
                    let snapshot = SeenItemSnapshot {
                        version_id: version_id.clone(),
                        urgency: None,
                        headline: title.clone(),
                        detail: detail.clone(),
                    };
                    let seen_standing = evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
                    entries.push(UpdateEntry {
                        item_key: SeenItemKey(key_str),
                        version_id,
                        workspace: row.workspace,
                        patient_display_name,
                        title,
                        detail,
                        occurred_at_unix: retired_at,
                        seen_standing,
                        kind: UpdateEntryKind::ConcernRetired {
                            concern_id,
                            retirement_reason,
                        },
                    });
                }
                "open" | "done" => {
                    let key_str = format!("concern:{}:{}", row.workspace, row.id);
                    let version_id = format!("rev:{}", row.revision);
                    let headline = summarize_one_line(&row.reason, 96);
                    let snapshot = SeenItemSnapshot {
                        version_id: version_id.clone(),
                        urgency: Some(urgency),
                        headline: headline.clone(),
                        detail: row.reason.clone(),
                    };
                    let seen_standing = evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
                    let title = if row.status == "done" {
                        format!("Concern (cleared Done): {headline}")
                    } else {
                        format!("Concern raised: {headline}")
                    };
                    entries.push(UpdateEntry {
                        item_key: SeenItemKey(key_str),
                        version_id,
                        workspace: row.workspace,
                        patient_display_name,
                        title,
                        detail: row.reason,
                        occurred_at_unix: row.updated_at,
                        seen_standing,
                        kind: UpdateEntryKind::ConcernRaised {
                            concern_id,
                            urgency,
                            standing_label: row.status,
                        },
                    });
                }
                other => return Err(format!("invalid concern status `{other}`")),
            }
        }
        Ok(entries)
    }

    async fn read_updates_reviews_slice(
        &self,
        ctx: &AuthorizedReadContext,
        names: &BTreeMap<String, String>,
        seen_map: &BTreeMap<String, StoredSeenEntry>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<(Vec<ActiveUpdateRun>, Vec<UpdateEntry>), String> {
        probe.check_producer(ProducerKind::PatientReviews)?;
        if ctx.allowed_workspaces.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }
        let rows = patient_review::Entity::find()
            .filter(patient_review::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                patient_review::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        let mut active_runs = Vec::new();
        let mut entries = Vec::new();
        for row in rows {
            let state: ReviewLifecycleState = serde_json::from_value(row.state)
                .map_err(|err| format!("decode review state: {err}"))?;
            let patient_display_name = names
                .get(&row.workspace)
                .cloned()
                .unwrap_or_else(|| row.workspace.clone());

            match state {
                ReviewLifecycleState::Pending => {
                    active_runs.push(ActiveUpdateRun {
                        run_id: row.review_id,
                        workspace: row.workspace,
                        patient_display_name,
                        label: "Queued chart review".to_owned(),
                        started_at_unix: row.queued_at,
                    });
                }
                ReviewLifecycleState::Running { started_at, .. } => {
                    active_runs.push(ActiveUpdateRun {
                        run_id: row.review_id,
                        workspace: row.workspace,
                        patient_display_name,
                        label: "Reviewing new chart records".to_owned(),
                        started_at_unix: started_at,
                    });
                }
                ReviewLifecycleState::Completed {
                    outcome,
                    completed_at,
                    completed_source_version,
                } => {
                    let key_str = format!("review:{}:{}", row.workspace, row.review_id);
                    let version_id = format!("completed:{completed_source_version}:{completed_at}");
                    match outcome {
                        ReviewOutcome::Quiet {
                            reason,
                            explanation,
                            ..
                        } => {
                            let title = "Reviewed chart — nothing for you".to_owned();
                            let detail = summarize_one_line(&explanation, 160);
                            let snapshot = SeenItemSnapshot {
                                version_id: version_id.clone(),
                                urgency: None,
                                headline: title.clone(),
                                detail: explanation,
                            };
                            let seen_standing =
                                evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
                            entries.push(UpdateEntry {
                                item_key: SeenItemKey(key_str),
                                version_id,
                                workspace: row.workspace,
                                patient_display_name,
                                title,
                                detail,
                                occurred_at_unix: completed_at,
                                seen_standing,
                                kind: UpdateEntryKind::QuietReview {
                                    review_id: row.review_id,
                                    quiet_reason: reason.as_str().to_owned(),
                                },
                            });
                        }
                        ReviewOutcome::Assessed { summary, .. } => {
                            let title =
                                format!("Chart review: {}", summarize_one_line(&summary, 84));
                            let snapshot = SeenItemSnapshot {
                                version_id: version_id.clone(),
                                urgency: None,
                                headline: title.clone(),
                                detail: summary.clone(),
                            };
                            let seen_standing =
                                evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
                            entries.push(UpdateEntry {
                                item_key: SeenItemKey(key_str),
                                version_id,
                                workspace: row.workspace,
                                patient_display_name,
                                title,
                                detail: summarize_one_line(&summary, 180),
                                occurred_at_unix: completed_at,
                                seen_standing,
                                kind: UpdateEntryKind::AssessedReview {
                                    review_id: row.review_id,
                                },
                            });
                        }
                    }
                }
                ReviewLifecycleState::Held { .. }
                | ReviewLifecycleState::Failed { .. }
                | ReviewLifecycleState::Superseded { .. } => {}
            }
        }
        Ok((active_runs, entries))
    }

    async fn read_updates_visit_slice(
        &self,
        ctx: &AuthorizedReadContext,
        names: &BTreeMap<String, String>,
        seen_map: &BTreeMap<String, StoredSeenEntry>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<(Vec<ActiveUpdateRun>, Vec<UpdateEntry>), String> {
        probe.check_producer(ProducerKind::VisitPreparation)?;
        if ctx.allowed_workspaces.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }
        let prep_rows = visit_preparation::Entity::find()
            .filter(visit_preparation::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                visit_preparation::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        let mut active_runs = Vec::new();
        let mut entries = Vec::new();

        for row in prep_rows {
            let patient_display_name = names
                .get(&row.workspace)
                .cloned()
                .unwrap_or_else(|| row.workspace.clone());

            match row.state.as_str() {
                "preparing" => {
                    let run_id = row
                        .briefing_job_id
                        .unwrap_or_else(|| row.selection_job_id.clone());
                    active_runs.push(ActiveUpdateRun {
                        run_id,
                        workspace: row.workspace,
                        patient_display_name,
                        label: "Preparing visit briefing".to_owned(),
                        started_at_unix: row.updated_at,
                    });
                }
                "skipped" => {
                    let decision: VisitPreparationDecision =
                        serde_json::from_value(row.decision)
                            .map_err(|err| format!("decode visit decision: {err}"))?;
                    let reason = decision.reason().to_owned();
                    let key_str = format!(
                        "visit-skipped:{}:{}:{}:{}",
                        row.workspace, row.source_id, row.appointment_id, row.starts_at_unix
                    );
                    let version_id = format!("skipped:{}", row.updated_at);
                    let title = "Visit preparation — nothing to prepare".to_owned();
                    let snapshot = SeenItemSnapshot {
                        version_id: version_id.clone(),
                        urgency: None,
                        headline: title.clone(),
                        detail: reason.clone(),
                    };
                    let seen_standing = evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
                    entries.push(UpdateEntry {
                        item_key: SeenItemKey(key_str),
                        version_id,
                        workspace: row.workspace,
                        patient_display_name,
                        title,
                        detail: reason.clone(),
                        occurred_at_unix: row.updated_at,
                        seen_standing,
                        kind: UpdateEntryKind::EndedNothingToDo {
                            work_label: "Visit preparation".to_owned(),
                            reason,
                        },
                    });
                }
                "ended_visit_changed" => {
                    let reason = row.failed_reason.unwrap_or_else(|| {
                        "Appointment ended or changed before preparation.".to_owned()
                    });
                    let key_str = format!(
                        "visit-ended:{}:{}:{}:{}",
                        row.workspace, row.source_id, row.appointment_id, row.starts_at_unix
                    );
                    let version_id = format!("ended:{}", row.updated_at);
                    let title = "Visit preparation ended without briefing".to_owned();
                    let snapshot = SeenItemSnapshot {
                        version_id: version_id.clone(),
                        urgency: None,
                        headline: title.clone(),
                        detail: reason.clone(),
                    };
                    let seen_standing = evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
                    entries.push(UpdateEntry {
                        item_key: SeenItemKey(key_str),
                        version_id,
                        workspace: row.workspace,
                        patient_display_name,
                        title,
                        detail: reason.clone(),
                        occurred_at_unix: row.updated_at,
                        seen_standing,
                        kind: UpdateEntryKind::EndedNothingToDo {
                            work_label: "Visit preparation".to_owned(),
                            reason,
                        },
                    });
                }
                _ => {}
            }
        }

        let briefing_rows = visit_briefing::Entity::find()
            .filter(visit_briefing::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                visit_briefing::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        for row in briefing_rows {
            let patient_display_name = names
                .get(&row.workspace)
                .cloned()
                .unwrap_or_else(|| row.workspace.clone());
            let key_str = format!("briefing:{}:{}", row.workspace, row.briefing_id);
            let version_id = format!("v{}:{}", row.source_version, row.updated_at);
            let title = format!(
                "Visit briefing prepared: {}",
                summarize_one_line(&row.visit_card_orientation, 84)
            );
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: None,
                headline: title.clone(),
                detail: row.visit_card_orientation.clone(),
            };
            let seen_standing = evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
            entries.push(UpdateEntry {
                item_key: SeenItemKey(key_str),
                version_id,
                workspace: row.workspace,
                patient_display_name,
                title,
                detail: row.visit_card_orientation,
                occurred_at_unix: row.prepared_at,
                seen_standing,
                kind: UpdateEntryKind::VisitBriefing {
                    briefing_id: row.briefing_id,
                    starts_at_unix: row.starts_at_unix,
                },
            });
        }

        Ok((active_runs, entries))
    }

    async fn read_updates_understanding_slice(
        &self,
        ctx: &AuthorizedReadContext,
        names: &BTreeMap<String, String>,
        seen_map: &BTreeMap<String, StoredSeenEntry>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<(Vec<ActiveUpdateRun>, Vec<UpdateEntry>), String> {
        probe.check_producer(ProducerKind::PatientUnderstanding)?;
        if ctx.allowed_workspaces.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }

        let running_jobs = practice_job::Entity::find()
            .filter(practice_job::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                practice_job::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .filter(practice_job::Column::Kind.eq(PracticeJobKind::InitialUnderstanding))
            .filter(practice_job::Column::Status.is_in([WorkStatus::Pending, WorkStatus::Active]))
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        let mut active_runs = Vec::with_capacity(running_jobs.len());
        for job in running_jobs {
            let patient_display_name = names
                .get(&job.workspace)
                .cloned()
                .unwrap_or_else(|| job.workspace.clone());
            active_runs.push(ActiveUpdateRun {
                run_id: job.job_id,
                workspace: job.workspace,
                patient_display_name,
                label: "Preparing patient summary".to_owned(),
                started_at_unix: job.created_at,
            });
        }

        // Select ONLY summary columns — never OperatingIndex.
        let summary_rows = patient_understanding::Entity::find()
            .select_only()
            .column(patient_understanding::Column::Workspace)
            .column(patient_understanding::Column::Revision)
            .column(patient_understanding::Column::Summary)
            .column(patient_understanding::Column::UpdatedAt)
            .filter(patient_understanding::Column::Practice.eq(ctx.identity.practice_id.as_str()))
            .filter(
                patient_understanding::Column::Workspace
                    .is_in(ctx.allowed_workspaces.iter().map(String::as_str)),
            )
            .into_tuple::<(String, i64, Value, i64)>()
            .all(&self.db)
            .await
            .map_err(|err| err.to_string())?;

        let mut entries = Vec::with_capacity(summary_rows.len());
        for (workspace, revision_i64, summary_json, updated_at) in summary_rows {
            let revision =
                u64::try_from(revision_i64).map_err(|err| format!("summary revision: {err}"))?;
            let summary: PatientSummary = serde_json::from_value(summary_json)
                .map_err(|err| format!("decode patient summary: {err}"))?;
            let patient_display_name = names
                .get(&workspace)
                .cloned()
                .unwrap_or_else(|| workspace.clone());
            let key_str = format!("summary:{workspace}");
            let version_id = format!("rev:{revision}");
            let title = if revision <= 1 {
                "Prepared patient summary".to_owned()
            } else {
                format!("Updated patient summary (v{revision})")
            };
            let detail = summarize_one_line(&summary.narrative, 160);
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: None,
                headline: title.clone(),
                detail: summary.narrative,
            };
            let seen_standing = evaluate_seen_standing(seen_map.get(&key_str), &snapshot);
            entries.push(UpdateEntry {
                item_key: SeenItemKey(key_str),
                version_id,
                workspace,
                patient_display_name,
                title,
                detail,
                occurred_at_unix: updated_at,
                seen_standing,
                kind: UpdateEntryKind::PatientSummary { revision },
            });
        }

        Ok((active_runs, entries))
    }

    /// Opens a Today row or Updates entry on behalf of the signed-in physician,
    /// recording per-physician seen state in `physician_runtime_today_updates_seen`,
    /// reconciling any opened visit concern via [`Store::open_concern`], and
    /// returning the item's clinical work and cited chart evidence.
    pub async fn open_today_or_update_item(
        &self,
        auth: &dyn AuthStore,
        authority: &mut dyn Authority,
        request: &OpenItemRequest<'_>,
    ) -> Result<ItemWorkEvidence, TodayUpdatesError> {
        self.open_today_or_update_item_with_probe(auth, authority, request, &NoopTodayUpdatesProbe)
            .await
    }

    /// Opens a Today or Updates item using `probe` so forced item-open failures
    /// can be exercised cleanly.
    pub async fn open_today_or_update_item_with_probe(
        &self,
        auth: &dyn AuthStore,
        authority: &mut dyn Authority,
        request: &OpenItemRequest<'_>,
        probe: &dyn TodayUpdatesProbe,
    ) -> Result<ItemWorkEvidence, TodayUpdatesError> {
        let workspace = request.workspace;
        let item_key = request.item_key;
        let ctx = self
            .authorize_physician_read(auth, request.token, request.real_now_unix, Some(workspace))
            .await?;
        if let Err(detail) = probe.check_open_item(workspace, item_key.as_str()) {
            return Err(TodayUpdatesError::ItemOpenFailed {
                item_key: item_key.to_string(),
                detail,
            });
        }
        let names = self
            .resolve_patient_names(&ctx.allowed_workspaces, request.patient_names)
            .await;
        let patient_display_name = names
            .get(workspace)
            .cloned()
            .unwrap_or_else(|| workspace.to_owned());

        let (mut detail, snapshot, open_concern_id) = self
            .load_item_work_evidence_and_snapshot(&ctx, workspace, item_key, &patient_display_name)
            .await?;

        if let Some(concern_id) = open_concern_id {
            let access = WorkspaceAccess {
                practice: &ctx.identity.practice_id,
                physician: &ctx.identity.physician_id,
                workspace,
                session_hash: &ctx.session_hash,
            };
            let concern_cmd =
                CommandKey::new(format!("open-concern:{}", request.command_key.as_str()))?;
            let outcome = self
                .open_concern(
                    &access,
                    &concern_cmd,
                    &concern_id,
                    authority,
                    ctx.clinical_now_unix,
                )
                .await?;
            let (ConcernOpenOutcome::Opened { concern, .. }
            | ConcernOpenOutcome::Duplicate { concern }) = outcome;
            detail.version_id = format!("rev:{}", concern.revision);
        }

        let final_snapshot = SeenItemSnapshot {
            version_id: detail.version_id.clone(),
            ..snapshot
        };
        let seen_at = ctx.clinical_now_unix;
        self.save_physician_seen_snapshot(
            &ctx.identity.practice_id,
            &ctx.identity.physician_id,
            workspace,
            item_key,
            &final_snapshot,
            seen_at,
        )
        .await?;

        detail.seen_standing = SeenStanding::Seen { seen_at };
        Ok(detail)
    }

    async fn save_physician_seen_snapshot(
        &self,
        practice: &str,
        physician: &str,
        workspace: &str,
        item_key: &SeenItemKey,
        snapshot: &SeenItemSnapshot,
        seen_at: i64,
    ) -> Result<(), DbErr> {
        use today_updates_seen::Column as S;

        let snapshot_json =
            serde_json::to_value(snapshot).map_err(conversion_error("SeenItemSnapshot", "json"))?;
        today_updates_seen::Entity::insert(today_updates_seen::ActiveModel {
            practice: Set(practice.to_owned()),
            physician: Set(physician.to_owned()),
            item_key: Set(item_key.as_str().to_owned()),
            workspace: Set(workspace.to_owned()),
            seen_version: Set(snapshot.version_id.clone()),
            seen_snapshot: Set(snapshot_json),
            seen_at: Set(seen_at),
            updated_at: Set(seen_at),
        })
        .on_conflict(
            OnConflict::columns([S::Practice, S::Physician, S::ItemKey])
                .update_columns([
                    S::Workspace,
                    S::SeenVersion,
                    S::SeenSnapshot,
                    S::SeenAt,
                    S::UpdatedAt,
                ])
                .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await
        .map(|_| ())
    }

    async fn resolve_exact_record_citations(
        &self,
        workspace: &str,
        refs: &[ExactRecordReference],
    ) -> Result<Vec<ItemEvidenceCitation>, DbErr> {
        if refs.is_empty() {
            return Ok(Vec::new());
        }
        let candidate_ids: Vec<&str> = refs.iter().map(|r| r.candidate_id.as_str()).collect();
        let rows = source::Entity::find()
            .filter(source::Column::Workspace.eq(workspace))
            .filter(source::Column::Candidate.is_in(candidate_ids))
            .order_by_desc(source::Column::ObservedAtVersion)
            .all(&self.db)
            .await?;

        let mut by_ref: BTreeMap<(String, String), SourceRevision> = BTreeMap::new();
        for row in rows {
            let Some(rev_json) = row.revision else {
                continue;
            };
            let decoded: SourceRevision = serde_json::from_value(rev_json)
                .map_err(conversion_error("source revision", "SourceRevision"))?;
            by_ref
                .entry((row.candidate, decoded.revision.clone()))
                .or_insert(decoded);
        }

        let source_labels: BTreeMap<String, String> = source_pull::Entity::find()
            .filter(source_pull::Column::Workspace.eq(workspace))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| (row.source_id, row.source_label))
            .collect();

        let mut citations = Vec::with_capacity(refs.len());
        for reference in refs {
            if let Some(found) =
                by_ref.get(&(reference.candidate_id.clone(), reference.revision.clone()))
            {
                let display_label = super::history::cited_record_title(
                    &found.provenance,
                    &found.record_type,
                    &reference.candidate_id,
                );
                let body = super::history::format_cited_source_body(
                    &display_label,
                    &found.readable_parts,
                    &found.content,
                );
                let excerpt = if body.trim().is_empty() {
                    "Chart record on file.".to_owned()
                } else {
                    summarize_one_line(&body, EXCERPT_CHARS)
                };
                let as_of = found
                    .dates
                    .first()
                    .map(|date| date.value.trim())
                    .filter(|val| !val.is_empty())
                    .map(|raw| raw.split('T').next().unwrap_or(raw).trim().to_owned());
                let source_label = source_labels
                    .get(&found.source_id)
                    .cloned()
                    .filter(|label| !label.trim().is_empty())
                    .unwrap_or_else(|| found.source_id.clone());
                citations.push(ItemEvidenceCitation {
                    reference: reference.clone(),
                    source_id: found.source_id.clone(),
                    source_label,
                    display_label,
                    as_of,
                    excerpt,
                });
            } else {
                citations.push(ItemEvidenceCitation {
                    reference: reference.clone(),
                    source_id: "chart".to_owned(),
                    source_label: "Chart record".to_owned(),
                    display_label: reference.candidate_id.clone(),
                    as_of: None,
                    excerpt: "Referenced chart record.".to_owned(),
                });
            }
        }
        Ok(citations)
    }

    async fn load_item_work_evidence_and_snapshot(
        &self,
        ctx: &AuthorizedReadContext,
        workspace: &str,
        item_key: &SeenItemKey,
        patient_display_name: &str,
    ) -> Result<(ItemWorkEvidence, SeenItemSnapshot, Option<ConcernId>), TodayUpdatesError> {
        let key = item_key.as_str();
        let practice = ctx.identity.practice_id.as_str();

        if let Some(rest) = key
            .strip_prefix("concern:")
            .or_else(|| key.strip_prefix("concern-retired:"))
        {
            let is_retired_key = key.starts_with("concern-retired:");
            let (ws_part, id_part) =
                rest.split_once(':')
                    .ok_or_else(|| TodayUpdatesError::NotFound {
                        workspace: workspace.to_owned(),
                        item_key: key.to_owned(),
                    })?;
            if ws_part != workspace {
                return Err(TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                });
            }
            let row = concern::Entity::find_by_id((
                practice.to_owned(),
                workspace.to_owned(),
                id_part.to_owned(),
            ))
            .one(&self.db)
            .await?
            .ok_or_else(|| TodayUpdatesError::NotFound {
                workspace: workspace.to_owned(),
                item_key: key.to_owned(),
            })?;

            let urgency = match row.urgency.as_str() {
                "now" => ConcernUrgency::Now,
                _ => ConcernUrgency::Today,
            };
            let concern_id = ConcernId::new(row.id.clone())?;
            let refs: Vec<ExactRecordReference> = serde_json::from_value(row.records).map_err(
                conversion_error("concern records", "Vec<ExactRecordReference>"),
            )?;
            let source_work: SourceWork = serde_json::from_value(row.source_work)
                .map_err(conversion_error("concern source_work", "SourceWork"))?;
            let citations = self
                .resolve_exact_record_citations(workspace, &refs)
                .await?;
            let version_id = format!("rev:{}", row.revision);

            let visit_link = visit_concern::Entity::find_by_id((
                practice.to_owned(),
                workspace.to_owned(),
                row.id.clone(),
            ))
            .one(&self.db)
            .await?;

            if is_retired_key || row.status == "retired" {
                let retirement_reason = row
                    .retirement_reason
                    .clone()
                    .unwrap_or_else(|| "No longer needed".to_owned());
                let title = format!("Retired concern: {}", summarize_one_line(&row.reason, 84));
                let clinical_body = format!(
                    "Retired reason: {retirement_reason}\n\nEarlier clinical concern: {}",
                    row.reason
                );
                let snapshot = SeenItemSnapshot {
                    version_id: version_id.clone(),
                    urgency: None,
                    headline: title.clone(),
                    detail: retirement_reason,
                };
                return Ok((
                    ItemWorkEvidence {
                        item_key: item_key.clone(),
                        version_id,
                        workspace: workspace.to_owned(),
                        patient_display_name: patient_display_name.to_owned(),
                        kind_label: "Retired concern".to_owned(),
                        headline: title,
                        clinical_body,
                        basis_and_limitations: Some(format!(
                            "Grounded in {} cited chart {}.",
                            citations.len(),
                            if citations.len() == 1 {
                                "record"
                            } else {
                                "records"
                            }
                        )),
                        citations,
                        origin_summary: source_work_summary(&source_work),
                        chat_id: None,
                        seen_standing: SeenStanding::Unseen,
                    },
                    snapshot,
                    None,
                ));
            }

            let headline = if visit_link.is_some() {
                format!("Pre-visit: {}", summarize_one_line(&row.reason, 96))
            } else {
                summarize_one_line(&row.reason, 96)
            };
            let kind_label = match urgency {
                ConcernUrgency::Now => "Now concern".to_owned(),
                ConcernUrgency::Today => {
                    if visit_link.is_some() {
                        "Pre-visit nudge".to_owned()
                    } else {
                        "Today concern".to_owned()
                    }
                }
            };
            let mut clinical_body = row.reason.clone();
            let mut basis = Some(format!(
                "Backed by {} verified chart {} from {}.",
                citations.len(),
                if citations.len() == 1 {
                    "record"
                } else {
                    "records"
                },
                source_work_summary(&source_work)
            ));
            if let Some(vlink) = &visit_link
                && let Some(briefing_row) = visit_briefing::Entity::find_by_id((
                    practice.to_owned(),
                    vlink.source_id.clone(),
                    vlink.appointment_id.clone(),
                    vlink.starts_at_unix,
                ))
                .one(&self.db)
                .await?
            {
                clinical_body = format!(
                    "{}\n\nVisit briefing:\n{}",
                    row.reason, briefing_row.narrative
                );
                basis = Some(briefing_row.basis_and_limitations);
            }

            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: Some(urgency),
                headline: headline.clone(),
                detail: row.reason,
            };
            let open_concern_target = (row.status == "open").then_some(concern_id);
            return Ok((
                ItemWorkEvidence {
                    item_key: item_key.clone(),
                    version_id,
                    workspace: workspace.to_owned(),
                    patient_display_name: patient_display_name.to_owned(),
                    kind_label,
                    headline,
                    clinical_body,
                    basis_and_limitations: basis,
                    citations,
                    origin_summary: source_work_summary(&source_work),
                    chat_id: None,
                    seen_standing: SeenStanding::Unseen,
                },
                snapshot,
                open_concern_target,
            ));
        }

        if let Some(rest) = key
            .strip_prefix("review:")
            .or_else(|| key.strip_prefix("unhandled-review:"))
            .or_else(|| key.strip_prefix("running-review:"))
        {
            let (ws_part, review_id) =
                rest.split_once(':')
                    .ok_or_else(|| TodayUpdatesError::NotFound {
                        workspace: workspace.to_owned(),
                        item_key: key.to_owned(),
                    })?;
            if ws_part != workspace {
                return Err(TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                });
            }
            let row = patient_review::Entity::find_by_id((
                practice.to_owned(),
                workspace.to_owned(),
                review_id.to_owned(),
            ))
            .one(&self.db)
            .await?
            .ok_or_else(|| TodayUpdatesError::NotFound {
                workspace: workspace.to_owned(),
                item_key: key.to_owned(),
            })?;
            let state: ReviewLifecycleState = serde_json::from_value(row.state)
                .map_err(conversion_error("review state", "ReviewLifecycleState"))?;
            let unfinished: Vec<UnfinishedReviewWork> = serde_json::from_value(row.unfinished_work)
                .map_err(conversion_error(
                    "unfinished_work",
                    "Vec<UnfinishedReviewWork>",
                ))?;
            let evidence_refs: Vec<ExactRecordReference> =
                serde_json::from_value(row.evidence_read).map_err(conversion_error(
                    "evidence_read",
                    "Vec<ExactRecordReference>",
                ))?;

            match state {
                ReviewLifecycleState::Completed {
                    outcome,
                    completed_at,
                    completed_source_version,
                } => {
                    let version_id = format!("completed:{completed_source_version}:{completed_at}");
                    let citations = outcome
                        .citations()
                        .iter()
                        .map(review_citation_to_evidence)
                        .collect();
                    let basis = Some(outcome.basis_and_limitations().to_owned());
                    let (kind_label, headline, detail) = match &outcome {
                        ReviewOutcome::Quiet { explanation, .. } => (
                            "Reviewed, nothing for you".to_owned(),
                            "Reviewed chart — nothing for you".to_owned(),
                            explanation.clone(),
                        ),
                        ReviewOutcome::Assessed { summary, .. } => (
                            "Patient review".to_owned(),
                            format!("Chart review: {}", summarize_one_line(summary, 84)),
                            summary.clone(),
                        ),
                    };
                    let snapshot = SeenItemSnapshot {
                        version_id: version_id.clone(),
                        urgency: None,
                        headline: headline.clone(),
                        detail: detail.clone(),
                    };
                    return Ok((
                        ItemWorkEvidence {
                            item_key: item_key.clone(),
                            version_id,
                            workspace: workspace.to_owned(),
                            patient_display_name: patient_display_name.to_owned(),
                            kind_label,
                            headline,
                            clinical_body: detail,
                            basis_and_limitations: basis,
                            citations,
                            origin_summary: format!("Patient review ({})", row.review_id),
                            chat_id: None,
                            seen_standing: SeenStanding::Unseen,
                        },
                        snapshot,
                        None,
                    ));
                }
                ReviewLifecycleState::Failed {
                    reason, failed_at, ..
                } => {
                    let version_id = format!("failed:{failed_at}:{}", row.updated_at);
                    let topics: Vec<String> = unfinished
                        .iter()
                        .map(|item| format!("{}: {}", item.topic, item.detail))
                        .collect();
                    let headline = "Patient review couldn't finish".to_owned();
                    let detail = if topics.is_empty() {
                        format!("Couldn't finish requested review: {reason}")
                    } else {
                        format!("Unhandled follow-up: {}", topics.join("; "))
                    };
                    let citations = self
                        .resolve_exact_record_citations(workspace, &evidence_refs)
                        .await?;
                    let snapshot = SeenItemSnapshot {
                        version_id: version_id.clone(),
                        urgency: None,
                        headline: headline.clone(),
                        detail: detail.clone(),
                    };
                    return Ok((
                        ItemWorkEvidence {
                            item_key: item_key.clone(),
                            version_id,
                            workspace: workspace.to_owned(),
                            patient_display_name: patient_display_name.to_owned(),
                            kind_label: "Unhandled review work".to_owned(),
                            headline,
                            clinical_body: format!("{detail}\n\nFailure detail: {reason}"),
                            basis_and_limitations: Some(
                                "Review stopped before saving a complete assessment.".to_owned(),
                            ),
                            citations,
                            origin_summary: format!("Patient review ({})", row.review_id),
                            chat_id: None,
                            seen_standing: SeenStanding::Unseen,
                        },
                        snapshot,
                        None,
                    ));
                }
                ReviewLifecycleState::Held {
                    reason, held_at, ..
                } => {
                    let reason_label = match reason {
                        ReviewHoldReason::Stopped => "paused",
                        ReviewHoldReason::Limit => "reached run limit",
                    };
                    let version_id = format!("held:{held_at}:{}", row.updated_at);
                    let topics: Vec<String> = unfinished
                        .iter()
                        .map(|item| format!("{}: {}", item.topic, item.detail))
                        .collect();
                    let headline = format!("Patient review {reason_label} with unhandled items");
                    let detail = if topics.is_empty() {
                        format!("Requested review {reason_label} before finishing.")
                    } else {
                        format!("Unhandled follow-up: {}", topics.join("; "))
                    };
                    let citations = self
                        .resolve_exact_record_citations(workspace, &evidence_refs)
                        .await?;
                    let snapshot = SeenItemSnapshot {
                        version_id: version_id.clone(),
                        urgency: None,
                        headline: headline.clone(),
                        detail: detail.clone(),
                    };
                    return Ok((
                        ItemWorkEvidence {
                            item_key: item_key.clone(),
                            version_id,
                            workspace: workspace.to_owned(),
                            patient_display_name: patient_display_name.to_owned(),
                            kind_label: "Unhandled review work".to_owned(),
                            headline,
                            clinical_body: detail,
                            basis_and_limitations: Some(
                                "Review held before completing assessment.".to_owned(),
                            ),
                            citations,
                            origin_summary: format!("Patient review ({})", row.review_id),
                            chat_id: None,
                            seen_standing: SeenStanding::Unseen,
                        },
                        snapshot,
                        None,
                    ));
                }
                other => {
                    let version_id = format!("{}:{}", other.status_str(), row.updated_at);
                    let headline = "Reviewing patient chart as requested".to_owned();
                    let detail =
                        "Coda is reviewing the requested records for this patient.".to_owned();
                    let citations = self
                        .resolve_exact_record_citations(workspace, &evidence_refs)
                        .await?;
                    let snapshot = SeenItemSnapshot {
                        version_id: version_id.clone(),
                        urgency: None,
                        headline: headline.clone(),
                        detail: detail.clone(),
                    };
                    return Ok((
                        ItemWorkEvidence {
                            item_key: item_key.clone(),
                            version_id,
                            workspace: workspace.to_owned(),
                            patient_display_name: patient_display_name.to_owned(),
                            kind_label: "Running review".to_owned(),
                            headline,
                            clinical_body: detail,
                            basis_and_limitations: None,
                            citations,
                            origin_summary: format!("Patient review ({})", row.review_id),
                            chat_id: None,
                            seen_standing: SeenStanding::Unseen,
                        },
                        snapshot,
                        None,
                    ));
                }
            }
        }

        if let Some(rest) = key.strip_prefix("briefing:") {
            let (ws_part, briefing_id) =
                rest.split_once(':')
                    .ok_or_else(|| TodayUpdatesError::NotFound {
                        workspace: workspace.to_owned(),
                        item_key: key.to_owned(),
                    })?;
            if ws_part != workspace {
                return Err(TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                });
            }
            let row = visit_briefing::Entity::find()
                .filter(visit_briefing::Column::Practice.eq(practice))
                .filter(visit_briefing::Column::Workspace.eq(workspace))
                .filter(visit_briefing::Column::BriefingId.eq(briefing_id))
                .one(&self.db)
                .await?
                .ok_or_else(|| TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                })?;
            let raw_citations: Vec<SummaryCitation> = serde_json::from_value(row.citations)
                .map_err(conversion_error(
                    "briefing citations",
                    "Vec<SummaryCitation>",
                ))?;
            let citations = raw_citations
                .iter()
                .map(summary_citation_to_evidence)
                .collect();
            let version_id = format!("v{}:{}", row.source_version, row.updated_at);
            let headline = format!(
                "Visit briefing prepared: {}",
                summarize_one_line(&row.visit_card_orientation, 84)
            );
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: None,
                headline: headline.clone(),
                detail: row.visit_card_orientation,
            };
            return Ok((
                ItemWorkEvidence {
                    item_key: item_key.clone(),
                    version_id,
                    workspace: workspace.to_owned(),
                    patient_display_name: patient_display_name.to_owned(),
                    kind_label: "Visit briefing".to_owned(),
                    headline,
                    clinical_body: row.narrative,
                    basis_and_limitations: Some(row.basis_and_limitations),
                    citations,
                    origin_summary: format!("Visit briefing ({})", row.briefing_id),
                    chat_id: None,
                    seen_standing: SeenStanding::Unseen,
                },
                snapshot,
                None,
            ));
        }

        if let Some(ws_part) = key.strip_prefix("summary:") {
            if ws_part != workspace {
                return Err(TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                });
            }
            let row = patient_understanding::Entity::find()
                .select_only()
                .column(patient_understanding::Column::Revision)
                .column(patient_understanding::Column::SourceVersion)
                .column(patient_understanding::Column::Summary)
                .filter(patient_understanding::Column::Practice.eq(practice))
                .filter(patient_understanding::Column::Workspace.eq(workspace))
                .into_tuple::<(i64, i64, Value)>()
                .one(&self.db)
                .await?
                .ok_or_else(|| TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                })?;
            let (revision_i64, source_version, summary_json) = row;
            let summary: PatientSummary = serde_json::from_value(summary_json)
                .map_err(conversion_error("patient summary", "PatientSummary"))?;
            let citations = summary
                .citations
                .iter()
                .map(summary_citation_to_evidence)
                .collect();
            let version_id = format!("rev:{revision_i64}");
            let headline = if revision_i64 <= 1 {
                "Prepared patient summary".to_owned()
            } else {
                format!("Updated patient summary (v{revision_i64})")
            };
            let basis = if summary.material_gaps.is_empty() {
                summary
                    .scope_note
                    .clone()
                    .or_else(|| Some(format!("Grounded in chart version {source_version}.")))
            } else {
                Some(format!(
                    "Chart version {source_version}. Gaps: {}",
                    summary.material_gaps.join("; ")
                ))
            };
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: None,
                headline: headline.clone(),
                detail: summary.narrative.clone(),
            };
            return Ok((
                ItemWorkEvidence {
                    item_key: item_key.clone(),
                    version_id,
                    workspace: workspace.to_owned(),
                    patient_display_name: patient_display_name.to_owned(),
                    kind_label: "Patient summary".to_owned(),
                    headline,
                    clinical_body: summary.narrative,
                    basis_and_limitations: basis,
                    citations,
                    origin_summary: format!("Patient summary v{revision_i64}"),
                    chat_id: None,
                    seen_standing: SeenStanding::Unseen,
                },
                snapshot,
                None,
            ));
        }

        if let Some(rest) = key
            .strip_prefix("unhandled-visit:")
            .or_else(|| key.strip_prefix("visit-skipped:"))
            .or_else(|| key.strip_prefix("visit-ended:"))
        {
            let parts: Vec<&str> = rest.splitn(4, ':').collect();
            if parts.len() != 4 || parts[0] != workspace {
                return Err(TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                });
            }
            let starts_at_unix: i64 =
                parts[3].parse().map_err(|_| TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                })?;
            let row = visit_preparation::Entity::find_by_id((
                practice.to_owned(),
                parts[1].to_owned(),
                parts[2].to_owned(),
                starts_at_unix,
            ))
            .one(&self.db)
            .await?
            .ok_or_else(|| TodayUpdatesError::NotFound {
                workspace: workspace.to_owned(),
                item_key: key.to_owned(),
            })?;

            let (kind_label, headline, detail, version_id) = match row.state.as_str() {
                "could_not_finish" => {
                    let reason = row
                        .failed_reason
                        .unwrap_or_else(|| "Pre-visit briefing could not finish.".to_owned());
                    let detail = format!("Upcoming visit: {reason}");
                    (
                        "Unhandled visit preparation".to_owned(),
                        "Visit briefing couldn't finish".to_owned(),
                        detail,
                        format!("could_not_finish:{}", row.updated_at),
                    )
                }
                "skipped" => {
                    let decision: VisitPreparationDecision = serde_json::from_value(row.decision)
                        .map_err(conversion_error(
                        "visit decision",
                        "VisitPreparationDecision",
                    ))?;
                    let reason = decision.reason().to_owned();
                    (
                        "Visit preparation".to_owned(),
                        "Visit preparation — nothing to prepare".to_owned(),
                        reason,
                        format!("skipped:{}", row.updated_at),
                    )
                }
                _ => {
                    let reason = row.failed_reason.unwrap_or_else(|| {
                        "Appointment ended or changed before preparation.".to_owned()
                    });
                    (
                        "Visit preparation".to_owned(),
                        "Visit preparation ended without briefing".to_owned(),
                        reason,
                        format!("ended:{}", row.updated_at),
                    )
                }
            };
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: None,
                headline: headline.clone(),
                detail: detail.clone(),
            };
            return Ok((
                ItemWorkEvidence {
                    item_key: item_key.clone(),
                    version_id,
                    workspace: workspace.to_owned(),
                    patient_display_name: patient_display_name.to_owned(),
                    kind_label,
                    headline,
                    clinical_body: detail,
                    basis_and_limitations: None,
                    citations: Vec::new(),
                    origin_summary: format!("Visit preparation ({})", row.appointment_id),
                    chat_id: None,
                    seen_standing: SeenStanding::Unseen,
                },
                snapshot,
                None,
            ));
        }

        if let Some(rest) = key.strip_prefix("unhandled-understanding:") {
            let (ws_part, job_id) =
                rest.split_once(':')
                    .ok_or_else(|| TodayUpdatesError::NotFound {
                        workspace: workspace.to_owned(),
                        item_key: key.to_owned(),
                    })?;
            if ws_part != workspace {
                return Err(TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                });
            }
            let job = practice_job::Entity::find_by_id((
                practice.to_owned(),
                workspace.to_owned(),
                job_id.to_owned(),
            ))
            .one(&self.db)
            .await?
            .ok_or_else(|| TodayUpdatesError::NotFound {
                workspace: workspace.to_owned(),
                item_key: key.to_owned(),
            })?;
            let status_tag = match job.status {
                WorkStatus::Failed => "failed",
                WorkStatus::Held => "held",
                _ => "incomplete",
            };
            let version_id = format!("{status_tag}:{}:{}", job.failures, job.updated_at);
            let headline = "Initial patient summary couldn't finish".to_owned();
            let detail = format!(
                "Chart summary for version {} could not complete automatically.",
                job.sources_version
            );
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: None,
                headline: headline.clone(),
                detail: detail.clone(),
            };
            return Ok((
                ItemWorkEvidence {
                    item_key: item_key.clone(),
                    version_id,
                    workspace: workspace.to_owned(),
                    patient_display_name: patient_display_name.to_owned(),
                    kind_label: "Unhandled summary work".to_owned(),
                    headline,
                    clinical_body: detail,
                    basis_and_limitations: Some(
                        "Summary job exhausted automatic retry attempts.".to_owned(),
                    ),
                    citations: Vec::new(),
                    origin_summary: format!("Initial understanding ({})", job.job_id),
                    chat_id: None,
                    seen_standing: SeenStanding::Unseen,
                },
                snapshot,
                None,
            ));
        }

        if let Some(rest) = key.strip_prefix("running-chat:") {
            let (ws_part, chat_id) =
                rest.split_once(':')
                    .ok_or_else(|| TodayUpdatesError::NotFound {
                        workspace: workspace.to_owned(),
                        item_key: key.to_owned(),
                    })?;
            if ws_part != workspace {
                return Err(TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                });
            }
            let row = conversation::Entity::find_by_id((
                practice.to_owned(),
                ctx.identity.physician_id.clone(),
                workspace.to_owned(),
                chat_id.to_owned(),
            ))
            .one(&self.db)
            .await?
            .ok_or_else(|| TodayUpdatesError::NotFound {
                workspace: workspace.to_owned(),
                item_key: key.to_owned(),
            })?;
            let revision = row.revision;
            let state = control::conversation(row, Vec::new())?;
            let latest_question = state
                .messages
                .iter()
                .rev()
                .find(|msg| matches!(msg.role, MessageRole::User { .. }))
                .map(|msg| msg.text.clone())
                .unwrap_or_else(|| "Writing response".to_owned());
            let version_id = format!("rev:{revision}");
            let headline = format!(
                "Writing response: {}",
                summarize_one_line(&latest_question, 80)
            );
            let detail = "Coda is answering your question in this conversation.".to_owned();
            let snapshot = SeenItemSnapshot {
                version_id: version_id.clone(),
                urgency: None,
                headline: headline.clone(),
                detail: detail.clone(),
            };
            return Ok((
                ItemWorkEvidence {
                    item_key: item_key.clone(),
                    version_id,
                    workspace: workspace.to_owned(),
                    patient_display_name: patient_display_name.to_owned(),
                    kind_label: "Active conversation".to_owned(),
                    headline,
                    clinical_body: format!("Question: {latest_question}\n\n{detail}"),
                    basis_and_limitations: None,
                    citations: Vec::new(),
                    origin_summary: format!("Conversation ({chat_id})"),
                    chat_id: Some(chat_id.to_owned()),
                    seen_standing: SeenStanding::Unseen,
                },
                snapshot,
                None,
            ));
        }

        Err(TodayUpdatesError::NotFound {
            workspace: workspace.to_owned(),
            item_key: key.to_owned(),
        })
    }

    /// Clears a Today item as `Done`: delegates open concerns to
    /// [`Store::mark_concern_done`] (clearing for every physician sharing the
    /// patient and recording evaluation feedback) and records durable once-only
    /// dismissals for non-restartable unhandled work.
    pub async fn clear_today_item_done(
        &self,
        auth: &dyn AuthStore,
        authority: &mut dyn Authority,
        request: &ClearItemDoneRequest<'_>,
    ) -> Result<TodayDoneResult, TodayUpdatesError> {
        let workspace = request.workspace;
        let item_key = request.item_key;
        let command_key = request.command_key;
        let ctx = self
            .authorize_physician_read(auth, request.token, request.real_now_unix, Some(workspace))
            .await?;
        let key = item_key.as_str();

        if let Some(rest) = key.strip_prefix("concern:") {
            let (ws_part, id_part) =
                rest.split_once(':')
                    .ok_or_else(|| TodayUpdatesError::NotFound {
                        workspace: workspace.to_owned(),
                        item_key: key.to_owned(),
                    })?;
            if ws_part != workspace {
                return Err(TodayUpdatesError::NotFound {
                    workspace: workspace.to_owned(),
                    item_key: key.to_owned(),
                });
            }
            let concern_id = ConcernId::new(id_part.to_owned())?;
            let access = WorkspaceAccess {
                practice: &ctx.identity.practice_id,
                physician: &ctx.identity.physician_id,
                workspace,
                session_hash: &ctx.session_hash,
            };
            return match self
                .mark_concern_done(&access, command_key, &concern_id, authority)
                .await?
            {
                ConcernDoneOutcome::Cleared { .. } => Ok(TodayDoneResult::Cleared {
                    item_key: item_key.clone(),
                }),
                ConcernDoneOutcome::Duplicate { .. } => Ok(TodayDoneResult::Duplicate {
                    item_key: item_key.clone(),
                }),
                ConcernDoneOutcome::AlreadyCleared { .. } => Ok(TodayDoneResult::AlreadyCleared {
                    item_key: item_key.clone(),
                }),
            };
        }

        if !key.starts_with("unhandled-") {
            return Err(TodayUpdatesError::CannotMarkDone {
                item_key: key.to_owned(),
            });
        }
        let trimmed_version = request.version_id.trim();
        if trimmed_version.is_empty() {
            return Err(TodayUpdatesError::InvalidInput {
                field: "version_id",
            });
        }

        let access = WorkspaceAccess {
            practice: &ctx.identity.practice_id,
            physician: &ctx.identity.physician_id,
            workspace,
            session_hash: &ctx.session_hash,
        };
        let scope = access.authority_scope()?;
        let tx = self.db.begin().await?;
        if let Err(reason) = authority
            .lock_publication(&tx, &scope, Some(access.session_hash))
            .await
        {
            tx.rollback().await?;
            return Err(TodayUpdatesError::AccessDenied { reason });
        }

        let command_json = json!({
            "kind": "today_work_done",
            "practice": &scope.practice,
            "physician": &scope.physician,
            "workspace": &scope.workspace,
            "item_key": key,
            "version_id": trimmed_version,
        });
        match receipt::record(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command_json,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                tx.commit().await?;
                return Ok(TodayDoneResult::Duplicate {
                    item_key: item_key.clone(),
                });
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(TodayUpdatesError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        let existing = today_work_dismissal::Entity::find_by_id((
            scope.practice.clone(),
            scope.workspace.clone(),
            key.to_owned(),
            trimmed_version.to_owned(),
        ))
        .one(&tx)
        .await?;
        if existing.is_some() {
            tx.commit().await?;
            return Ok(TodayDoneResult::AlreadyCleared {
                item_key: item_key.clone(),
            });
        }

        today_work_dismissal::Entity::insert(today_work_dismissal::ActiveModel {
            practice: Set(scope.practice),
            workspace: Set(scope.workspace),
            item_key: Set(key.to_owned()),
            version_id: Set(trimmed_version.to_owned()),
            cleared_by_physician: Set(scope.physician),
            cleared_at: Set(ctx.clinical_now_unix),
        })
        .exec_without_returning(&tx)
        .await?;
        tx.commit().await?;

        Ok(TodayDoneResult::Cleared {
            item_key: item_key.clone(),
        })
    }
}

fn review_citation_to_evidence(citation: &ReviewCitation) -> ItemEvidenceCitation {
    ItemEvidenceCitation {
        reference: citation.reference.clone(),
        source_id: citation.source_id.clone(),
        source_label: citation.source_label.clone(),
        display_label: citation.display_label.clone(),
        as_of: citation.as_of.clone(),
        excerpt: citation.excerpt.clone(),
    }
}

fn summary_citation_to_evidence(citation: &SummaryCitation) -> ItemEvidenceCitation {
    ItemEvidenceCitation {
        reference: citation.reference.clone(),
        source_id: citation.source_id.clone(),
        source_label: citation.source_label.clone(),
        display_label: citation.display_label.clone(),
        as_of: citation.as_of.clone(),
        excerpt: citation.excerpt.clone(),
    }
}

/// Typed SeaORM seeding helpers for exercising every Today and Updates
/// producer state at deterministic timestamps in integration and browser proof
/// tests without raw SQL.
pub mod fixture {
    use sea_orm::{ActiveModelTrait, EntityTrait, Set, Unchanged, sea_query::OnConflict};
    use serde_json::json;

    use super::*;
    use crate::initial_understanding::{EvidenceScope, OperatingIndex};
    use crate::runtime::reviews::ReviewUsage;

    /// Lifecycle state when seeding a fixture concern.
    #[derive(Clone, Debug)]
    pub enum SeedConcernStanding {
        /// Active on Today.
        Open,
        /// Retired by Coda with a one-line clinical reason at `retired_at`.
        Retired {
            /// One-line retirement reason.
            reason: String,
            /// Unix second when retired.
            retired_at: i64,
        },
    }

    /// Typed specification for seeding a concern (and optional visit-slot link).
    #[derive(Clone, Debug)]
    pub struct SeedConcern {
        /// Practice identifier.
        pub practice: String,
        /// Patient workspace identifier.
        pub workspace: String,
        /// Concern identifier.
        pub concern_id: String,
        /// `Now` or `Today`.
        pub urgency: ConcernUrgency,
        /// `Open` or `Retired`.
        pub standing: SeedConcernStanding,
        /// Clinical statement.
        pub reason: String,
        /// Cited chart record references.
        pub records: Vec<ExactRecordReference>,
        /// Originating background work.
        pub source_work: SourceWork,
        /// Unix second when raised.
        pub created_at: i64,
        /// Unix second of latest update.
        pub updated_at: i64,
        /// Monotonic revision (>= 1).
        pub revision: i64,
        /// Optional pre-visit appointment link.
        pub visit_slot: Option<TodayVisitSlot>,
    }

    /// Inserts a typed concern (and optional `visit_concern` link) into Postgres.
    pub async fn seed_concern(store: &Store, spec: SeedConcern) -> Result<(), DbErr> {
        let records_json =
            serde_json::to_value(&spec.records).map_err(conversion_error("records", "json"))?;
        let work_json = serde_json::to_value(&spec.source_work)
            .map_err(conversion_error("source_work", "json"))?;
        let (status, ret_reason, ret_work, ret_at) = match &spec.standing {
            SeedConcernStanding::Open => ("open".to_owned(), None, None, None),
            SeedConcernStanding::Retired { reason, retired_at } => (
                "retired".to_owned(),
                Some(reason.clone()),
                Some(work_json.clone()),
                Some(*retired_at),
            ),
        };

        concern::ActiveModel {
            practice: Set(spec.practice.clone()),
            workspace: Set(spec.workspace.clone()),
            id: Set(spec.concern_id.clone()),
            urgency: Set(spec.urgency.as_str().to_owned()),
            status: Set(status),
            reason: Set(spec.reason),
            records: Set(records_json),
            source_work: Set(work_json),
            cleared_by_physician: Set(None),
            cleared_at: Set(None),
            feedback_revision: Set(None),
            retirement_reason: Set(ret_reason),
            retired_by_work: Set(ret_work),
            retired_at: Set(ret_at),
            created_at: Set(spec.created_at),
            updated_at: Set(spec.updated_at),
            revision: Set(spec.revision),
        }
        .insert(&store.db)
        .await?;

        if let Some(slot) = spec.visit_slot {
            use super::super::entities::schedule_appointment::{self, Column as A};
            use super::super::schedule::AppointmentStatus;
            schedule_appointment::Entity::insert(schedule_appointment::ActiveModel {
                practice: Set(spec.practice.clone()),
                source_id: Set(slot.source_id.clone()),
                appointment_id: Set(slot.appointment_id.clone()),
                saved_version: Set(1),
                source_revision: Set("rev-1".to_owned()),
                starts_at: Set(slot.starts_at_unix.to_string()),
                starts_at_unix: Set(slot.starts_at_unix),
                ends_at: Set(slot.ends_at_unix.to_string()),
                ends_at_unix: Set(slot.ends_at_unix),
                status: Set(AppointmentStatus::Scheduled),
                source_status: Set("booked".to_owned()),
                label: Set("Follow-up visit".to_owned()),
                match_state: Set("matched".to_owned()),
                workspace: Set(Some(spec.workspace.clone())),
                source_patient_id: Set(Some(spec.workspace.clone())),
                dates: Set(json!([])),
                content: Set("{}".to_owned()),
                sha256: Set("fixture-appt".to_owned()),
                provenance: Set(json!({})),
                as_of: Set(spec.updated_at),
                updated_at: Set(spec.updated_at),
            })
            .on_conflict(
                OnConflict::columns([A::Practice, A::SourceId, A::AppointmentId])
                    .update_columns([A::StartsAtUnix, A::EndsAtUnix, A::Status, A::UpdatedAt])
                    .to_owned(),
            )
            .exec_without_returning(&store.db)
            .await?;

            visit_concern::ActiveModel {
                practice: Set(spec.practice),
                workspace: Set(spec.workspace),
                concern_id: Set(spec.concern_id),
                source_id: Set(slot.source_id),
                appointment_id: Set(slot.appointment_id),
                starts_at_unix: Set(slot.starts_at_unix),
                ends_at_unix: Set(slot.ends_at_unix),
                urgency: Set(spec.urgency.as_str().to_owned()),
                opened_at: Set(None),
                opened_by_physician: Set(None),
                created_at: Set(spec.created_at),
                updated_at: Set(spec.updated_at),
            }
            .insert(&store.db)
            .await?;
        }
        Ok(())
    }

    /// Revises an open seeded concern in place so `SeenStanding::ChangedSinceSeen`
    /// can be verified deterministically across restarts and browser tabs.
    pub async fn revise_seeded_concern(
        store: &Store,
        practice: &str,
        workspace: &str,
        concern_id: &str,
        new_urgency: ConcernUrgency,
        new_reason: &str,
        updated_at: i64,
    ) -> Result<(), DbErr> {
        let row = concern::Entity::find_by_id((
            practice.to_owned(),
            workspace.to_owned(),
            concern_id.to_owned(),
        ))
        .one(&store.db)
        .await?
        .ok_or_else(|| DbErr::Custom(format!("concern `{concern_id}` not found")))?;

        let next_rev = row.revision + 1;
        concern::ActiveModel {
            practice: Unchanged(row.practice),
            workspace: Unchanged(row.workspace),
            id: Unchanged(row.id),
            urgency: Set(new_urgency.as_str().to_owned()),
            status: Unchanged(row.status),
            reason: Set(new_reason.to_owned()),
            records: Unchanged(row.records),
            source_work: Unchanged(row.source_work),
            cleared_by_physician: Unchanged(row.cleared_by_physician),
            cleared_at: Unchanged(row.cleared_at),
            feedback_revision: Unchanged(row.feedback_revision),
            retirement_reason: Unchanged(row.retirement_reason),
            retired_by_work: Unchanged(row.retired_by_work),
            retired_at: Unchanged(row.retired_at),
            created_at: Unchanged(row.created_at),
            updated_at: Set(updated_at),
            revision: Set(next_rev),
        }
        .update(&store.db)
        .await?;
        Ok(())
    }

    /// Typed specification for seeding a saved patient review row.
    #[derive(Clone, Debug)]
    pub struct SeedReview {
        /// Practice identifier.
        pub practice: String,
        /// Patient workspace identifier.
        pub workspace: String,
        /// Review identifier.
        pub review_id: String,
        /// Triggers that started the review.
        pub triggers: Vec<ReviewTrigger>,
        /// Unfinished follow-up items recorded on the review.
        pub unfinished_work: Vec<UnfinishedReviewWork>,
        /// Lifecycle state (`Pending`, `Running`, `Held`, `Completed`, `Failed`, `Superseded`).
        pub state: ReviewLifecycleState,
        /// Exact chart references read during the review.
        pub evidence_read: Vec<ExactRecordReference>,
        /// Unix second when queued.
        pub queued_at: i64,
        /// Unix second of latest update.
        pub updated_at: i64,
    }

    /// Inserts a typed patient review row into Postgres.
    pub async fn seed_review(store: &Store, spec: SeedReview) -> Result<(), DbErr> {
        let completed_at = match &spec.state {
            ReviewLifecycleState::Completed { completed_at, .. } => Some(*completed_at),
            _ => None,
        };
        let completed_source_version = match &spec.state {
            ReviewLifecycleState::Completed {
                completed_source_version,
                ..
            } => Some(
                i64::try_from(*completed_source_version)
                    .map_err(conversion_error("completed_source_version", "i64"))?,
            ),
            _ => None,
        };
        let started_at = match &spec.state {
            ReviewLifecycleState::Running { started_at, .. } => Some(*started_at),
            ReviewLifecycleState::Completed { .. } => Some(spec.queued_at),
            _ => None,
        };

        patient_review::ActiveModel {
            practice: Set(spec.practice),
            workspace: Set(spec.workspace),
            review_id: Set(spec.review_id.clone()),
            command_key: Set(format!("cmd-{}", spec.review_id)),
            job_id: Set(format!("job-{}", spec.review_id)),
            work_id: Set(format!("work-{}", spec.review_id)),
            status: Set(spec.state.status_str().to_owned()),
            patient_snapshot: Set(None),
            triggers: Set(serde_json::to_value(&spec.triggers)
                .map_err(conversion_error("triggers", "json"))?),
            accepted_inputs: Set(json!([])),
            recipe_version: Set(json!({"model":"fixture","effort":"high"})),
            initial_source_version: Set(1),
            initial_sources: Set(json!({})),
            completed_source_version: Set(completed_source_version),
            completed_sources: Set(completed_source_version.map(|_| json!({}))),
            evidence_read: Set(serde_json::to_value(&spec.evidence_read)
                .map_err(conversion_error("evidence_read", "json"))?),
            actions: Set(json!([])),
            unfinished_work: Set(serde_json::to_value(&spec.unfinished_work)
                .map_err(conversion_error("unfinished_work", "json"))?),
            state: Set(
                serde_json::to_value(&spec.state).map_err(conversion_error("state", "json"))?
            ),
            usage: Set(serde_json::to_value(ReviewUsage::Unknown)
                .map_err(conversion_error("usage", "json"))?),
            duration_ms: Set(completed_at.map(|_| 1_200)),
            staged_completion: Set(None),
            queued_at: Set(spec.queued_at),
            started_at: Set(started_at),
            completed_at: Set(completed_at),
            updated_at: Set(spec.updated_at),
        }
        .insert(&store.db)
        .await?;
        Ok(())
    }

    /// Standing when seeding a visit preparation row.
    #[derive(Clone, Debug)]
    pub enum SeedVisitPrepState {
        /// Actively preparing a visit briefing.
        Preparing {
            /// Briefing job identifier.
            job_id: String,
        },
        /// Skipped with a clinical reason.
        Skipped {
            /// Why no briefing was needed.
            reason: String,
        },
        /// Non-restartable failure ahead of the visit.
        CouldNotFinish {
            /// Failure summary.
            reason: String,
        },
        /// Appointment ended or moved before preparation ran.
        EndedVisitChanged {
            /// Explanation recorded on the visit preparation.
            reason: String,
        },
    }

    /// Typed specification for seeding a visit preparation row.
    #[derive(Clone, Debug)]
    pub struct SeedVisitPreparation {
        /// Practice identifier.
        pub practice: String,
        /// Patient workspace identifier.
        pub workspace: String,
        /// Schedule source identifier.
        pub source_id: String,
        /// Appointment identifier.
        pub appointment_id: String,
        /// Scheduled start in Unix seconds.
        pub starts_at_unix: i64,
        /// Scheduled end in Unix seconds.
        pub ends_at_unix: i64,
        /// Execution standing.
        pub state: SeedVisitPrepState,
        /// Unix second of latest update.
        pub updated_at: i64,
    }

    /// Inserts a typed visit preparation row into Postgres.
    pub async fn seed_visit_preparation(
        store: &Store,
        spec: SeedVisitPreparation,
    ) -> Result<(), DbErr> {
        let (state_str, decision, briefing_job_id, failed_reason) = match spec.state {
            SeedVisitPrepState::Preparing { job_id } => (
                "preparing".to_owned(),
                VisitPreparationDecision::PrepareNow {
                    reason: "Upcoming complex follow-up".to_owned(),
                },
                Some(job_id),
                None,
            ),
            SeedVisitPrepState::Skipped { reason } => (
                "skipped".to_owned(),
                VisitPreparationDecision::Skip { reason },
                None,
                None,
            ),
            SeedVisitPrepState::CouldNotFinish { reason } => (
                "could_not_finish".to_owned(),
                VisitPreparationDecision::PrepareNow {
                    reason: "Upcoming visit preparation".to_owned(),
                },
                Some(format!("brief-job-{}", spec.appointment_id)),
                Some(reason),
            ),
            SeedVisitPrepState::EndedVisitChanged { reason } => (
                "ended_visit_changed".to_owned(),
                VisitPreparationDecision::PrepareNow {
                    reason: "Scheduled visit".to_owned(),
                },
                None,
                Some(reason),
            ),
        };

        visit_preparation::ActiveModel {
            practice: Set(spec.practice),
            source_id: Set(spec.source_id),
            appointment_id: Set(spec.appointment_id.clone()),
            starts_at_unix: Set(spec.starts_at_unix),
            workspace: Set(spec.workspace),
            ends_at_unix: Set(spec.ends_at_unix),
            appointment_saved_version: Set(1),
            observed_source_version: Set(1),
            decision: Set(
                serde_json::to_value(&decision).map_err(conversion_error("decision", "json"))?
            ),
            decided_at: Set(spec.updated_at),
            selection_job_id: Set(format!("sel-job-{}", spec.appointment_id)),
            selection_work_id: Set(format!("sel-work-{}", spec.appointment_id)),
            selection_idempotency_key: Set(format!("sel-idem-{}", spec.appointment_id)),
            state: Set(state_str),
            briefing_job_id: Set(briefing_job_id),
            briefing_id: Set(None),
            failed_reason: Set(failed_reason),
            updated_at: Set(spec.updated_at),
        }
        .insert(&store.db)
        .await?;
        Ok(())
    }

    /// Typed specification for seeding a saved visit briefing.
    #[derive(Clone, Debug)]
    pub struct SeedVisitBriefing {
        /// Practice identifier.
        pub practice: String,
        /// Patient workspace identifier.
        pub workspace: String,
        /// Schedule source identifier.
        pub source_id: String,
        /// Appointment identifier.
        pub appointment_id: String,
        /// Scheduled start in Unix seconds.
        pub starts_at_unix: i64,
        /// Scheduled end in Unix seconds.
        pub ends_at_unix: i64,
        /// Stable briefing identifier.
        pub briefing_id: String,
        /// Concise orientation shown on schedule cards and Updates.
        pub orientation: String,
        /// Full narrative markdown.
        pub narrative: String,
        /// Dated basis and limitations.
        pub basis_and_limitations: String,
        /// Verified chart citations.
        pub citations: Vec<SummaryCitation>,
        /// Unix second when prepared.
        pub prepared_at: i64,
        /// Chart source version covered.
        pub source_version: i64,
    }

    /// Inserts a typed visit briefing row into Postgres.
    pub async fn seed_visit_briefing(store: &Store, spec: SeedVisitBriefing) -> Result<(), DbErr> {
        visit_briefing::ActiveModel {
            practice: Set(spec.practice),
            source_id: Set(spec.source_id),
            appointment_id: Set(spec.appointment_id),
            starts_at_unix: Set(spec.starts_at_unix),
            briefing_id: Set(spec.briefing_id.clone()),
            workspace: Set(spec.workspace),
            idempotency_key: Set(format!("idem-{}", spec.briefing_id)),
            starts_at: Set("2026-11-12T16:00:00Z".to_owned()),
            ends_at: Set("2026-11-12T16:30:00Z".to_owned()),
            ends_at_unix: Set(spec.ends_at_unix),
            appointment_saved_version: Set(1),
            prepared_at: Set(spec.prepared_at),
            source_version: Set(spec.source_version),
            source_versions: Set(json!({})),
            source_freshness: Set(json!([])),
            limits: Set(json!([])),
            visit_card_orientation: Set(spec.orientation),
            trajectory: Set(json!([])),
            meaningful_changes: Set(json!([])),
            unresolved_issues: Set(json!([])),
            visit_decisions: Set(json!([])),
            basis_and_limitations: Set(spec.basis_and_limitations),
            basis_citations: Set(json!([])),
            citations: Set(serde_json::to_value(&spec.citations)
                .map_err(conversion_error("citations", "json"))?),
            narrative: Set(spec.narrative),
            raised_concern_id: Set(None),
            visit_standing: Set("current".to_owned()),
            visit_standing_detail: Set(None),
            origin: Set(json!({
                "job_id": format!("job-{}", spec.briefing_id),
                "work_id": format!("work-{}", spec.briefing_id),
                "attempt_id": "attempt-1"
            })),
            updated_at: Set(spec.prepared_at),
        }
        .insert(&store.db)
        .await?;
        Ok(())
    }

    /// Inserts or updates a saved patient summary (`patient_understanding`) for `workspace`.
    pub async fn seed_patient_summary(
        store: &Store,
        practice: &str,
        workspace: &str,
        revision: i64,
        source_version: i64,
        summary: &PatientSummary,
        updated_at: i64,
    ) -> Result<(), DbErr> {
        use patient_understanding::Column as U;

        let empty_index = OperatingIndex {
            working_overview: "INTERNAL-OPERATING-INDEX-NEVER-SHOWN".to_owned(),
            known_health_systems: Vec::new(),
            open_threads: Vec::new(),
            anchors: Vec::new(),
        };
        let scope = EvidenceScope::ClinicalChart {
            connected_sources: Vec::new(),
        };

        patient_understanding::Entity::insert(patient_understanding::ActiveModel {
            practice: Set(practice.to_owned()),
            workspace: Set(workspace.to_owned()),
            revision: Set(revision),
            source_version: Set(source_version),
            evidence_scope: Set(
                serde_json::to_value(&scope).map_err(conversion_error("scope", "json"))?
            ),
            operating_index: Set(serde_json::to_value(&empty_index)
                .map_err(conversion_error("operating_index", "json"))?),
            summary: Set(
                serde_json::to_value(summary).map_err(conversion_error("summary", "json"))?
            ),
            origin: Set(json!({
                "kind": "practice_run",
                "job_id": format!("understanding-{workspace}"),
                "work_id": format!("work-{workspace}"),
                "attempt_id": "attempt-1",
                "mode": "initial"
            })),
            updated_at: Set(updated_at),
        })
        .on_conflict(
            OnConflict::columns([U::Practice, U::Workspace])
                .update_columns([U::Revision, U::SourceVersion, U::Summary, U::UpdatedAt])
                .to_owned(),
        )
        .exec_without_returning(&store.db)
        .await?;
        Ok(())
    }

    /// Inserts a failed `InitialUnderstanding` practice job row so non-restartable
    /// summary failures can be exercised on Today.
    pub async fn seed_failed_initial_understanding(
        store: &Store,
        practice: &str,
        workspace: &str,
        job_id: &str,
        sources_version: i64,
        updated_at: i64,
    ) -> Result<(), DbErr> {
        practice_job::ActiveModel {
            practice: Set(practice.to_owned()),
            workspace: Set(workspace.to_owned()),
            job_id: Set(job_id.to_owned()),
            kind: Set(PracticeJobKind::InitialUnderstanding),
            status: Set(WorkStatus::Failed),
            work: Set(format!("work-{job_id}")),
            revision: Set(1),
            assignment: Set(json!({"prompt":"Prepare initial patient summary"})),
            preloaded_skills: Set(json!([])),
            skills: Set("pkg-fixture".to_owned()),
            binding: Set(json!({"model":"fixture"})),
            runs: Set(json!({})),
            failures: Set(3),
            allowance_after: Set(0),
            sources_version: Set(sources_version),
            sources: Set(json!({})),
            result: Set(None),
            dispatch_after: Set(updated_at),
            created_at: Set(updated_at),
            updated_at: Set(updated_at),
        }
        .insert(&store.db)
        .await?;
        Ok(())
    }

    /// Outcome when seeding a connected source status row.
    #[derive(Clone, Debug)]
    pub enum SeedSourceOutcome {
        /// Connected source refreshed cleanly at `pulled_at`.
        Updated {
            /// Last successful pull timestamp.
            pulled_at: i64,
        },
        /// Connected source failed to refresh with `reason`.
        Failed {
            /// Optional prior successful pull timestamp.
            last_good_pulled_at: Option<i64>,
            /// Failure detail.
            reason: String,
        },
    }

    /// Inserts or updates a connected source refresh status row in `physician_runtime_source_statuses`.
    pub async fn seed_source_status(
        store: &Store,
        practice: &str,
        workspace: &str,
        source_id: &str,
        source_label: &str,
        outcome: SeedSourceOutcome,
        checked_at: i64,
    ) -> Result<(), DbErr> {
        use source_status::Column as S;

        let (outcome_str, last_good, reason) = match outcome {
            SeedSourceOutcome::Updated { pulled_at } => {
                ("updated".to_owned(), Some(pulled_at), None)
            }
            SeedSourceOutcome::Failed {
                last_good_pulled_at,
                reason,
            } => ("failed".to_owned(), last_good_pulled_at, Some(reason)),
        };

        source_status::Entity::insert(source_status::ActiveModel {
            practice: Set(practice.to_owned()),
            workspace: Set(workspace.to_owned()),
            source_id: Set(source_id.to_owned()),
            source_label: Set(source_label.to_owned()),
            outcome: Set(outcome_str),
            checked_at: Set(checked_at),
            last_good_pulled_at: Set(last_good),
            workspace_version: Set(Some(1)),
            changed_records: Set(0),
            reason: Set(reason),
        })
        .on_conflict(
            OnConflict::columns([S::Practice, S::Workspace, S::SourceId])
                .update_columns([
                    S::SourceLabel,
                    S::Outcome,
                    S::CheckedAt,
                    S::LastGoodPulledAt,
                    S::Reason,
                ])
                .to_owned(),
        )
        .exec_without_returning(&store.db)
        .await?;
        Ok(())
    }

    /// Parameters for seeding a running physician conversation row.
    #[derive(Clone, Debug)]
    pub struct SeedRunningConversation {
        /// Practice identifier.
        pub practice: String,
        /// Physician identifier.
        pub physician: String,
        /// Patient workspace identifier.
        pub workspace: String,
        /// Conversation identifier.
        pub chat_id: String,
        /// Latest physician question text.
        pub question: String,
        /// Unix timestamp when the question was accepted.
        pub activity_at: i64,
    }

    /// Inserts an active physician conversation row in `physician_runtime_conversations`
    /// using the canonical conversation state encoding.
    pub async fn seed_running_conversation(
        store: &Store,
        seed: SeedRunningConversation,
    ) -> Result<(), DbErr> {
        use super::super::{Conversation, Message, MessageRole, SkillPackageId, control};
        let skills: SkillPackageId = serde_json::from_value(json!("pkg-fixture"))
            .map_err(conversion_error("pkg", "SkillPackageId"))?;
        let work_id = format!("work-{}", seed.chat_id);
        let conv = Conversation {
            messages: vec![Message {
                role: MessageRole::User {
                    id: format!("cmd-{}", seed.chat_id),
                    work: work_id.clone(),
                    revision: 1,
                },
                text: seed.question,
            }],
            revision: 1,
            status: WorkStatus::Active,
            work: work_id,
            runs: BTreeMap::new(),
            skills,
            attempts: Vec::new(),
            session_hash: None,
            dispatch_after: seed.activity_at,
            failures: 0,
            budget_resume_after: 0,
            bindings: BTreeMap::new(),
            opened_at: seed.activity_at,
            last_activity_at: seed.activity_at,
            scenario: None,
        };
        let state = control::conversation_state(&conv)?;
        conversation::ActiveModel {
            practice: Set(seed.practice),
            physician: Set(seed.physician),
            workspace: Set(seed.workspace),
            chat: Set(seed.chat_id),
            status: Set(WorkStatus::Active),
            revision: Set(1),
            session_hash: Set(None),
            state: Set(state),
        }
        .insert(&store.db)
        .await?;
        Ok(())
    }
}

#[allow(dead_code)]
fn _assert_concern_standing_used(_: &Concern, _: &ConcernStanding) {}
