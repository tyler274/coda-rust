//! Per-patient source-check cadence, review reasons, and dispatch state rules.
//!
//! Each registered patient has a [`Cadence`] governing how often sources are
//! checked for record changes and how often a periodic whole-patient review
//! runs when no record changes arrive. By default, sources are checked daily
//! and a periodic review runs weekly. Authorized operators inspect and adjust
//! cadence; payment and connection state never alter it.

#![deny(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::num::NonZeroU64;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Default interval between patient source checks: one day (`86_400` seconds).
pub const DEFAULT_CHECK_EVERY_SECONDS: u64 = 24 * 60 * 60;

/// Default interval between periodic whole-patient reviews: seven days (`604_800` seconds).
pub const DEFAULT_REVIEW_EVERY_SECONDS: u64 = 7 * 24 * 60 * 60;

/// Validation error when constructing a [`Cadence`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CadenceError {
    /// Source-check interval must be at least one second.
    #[error("check interval must be positive")]
    ZeroCheckInterval,
    /// Periodic-review interval must be at least one second.
    #[error("review interval must be positive")]
    ZeroReviewInterval,
}

/// Per-patient source-check and periodic-review intervals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cadence {
    check_every_seconds: NonZeroU64,
    review_every_seconds: NonZeroU64,
}

impl Cadence {
    /// Builds a validated cadence from positive check and periodic-review intervals in seconds.
    pub fn new(check_every_seconds: u64, review_every_seconds: u64) -> Result<Self, CadenceError> {
        let check_every_seconds =
            NonZeroU64::new(check_every_seconds).ok_or(CadenceError::ZeroCheckInterval)?;
        let review_every_seconds =
            NonZeroU64::new(review_every_seconds).ok_or(CadenceError::ZeroReviewInterval)?;
        Ok(Self {
            check_every_seconds,
            review_every_seconds,
        })
    }

    /// Seconds between scheduled source checks.
    #[must_use]
    pub const fn check_every_seconds(self) -> u64 {
        self.check_every_seconds.get()
    }

    /// Seconds between scheduled periodic reviews when records have not changed.
    #[must_use]
    pub const fn review_every_seconds(self) -> u64 {
        self.review_every_seconds.get()
    }
}

impl Default for Cadence {
    fn default() -> Self {
        Self {
            check_every_seconds: NonZeroU64::new(DEFAULT_CHECK_EVERY_SECONDS)
                .expect("default check interval is non-zero"),
            review_every_seconds: NonZeroU64::new(DEFAULT_REVIEW_EVERY_SECONDS)
                .expect("default review interval is non-zero"),
        }
    }
}

/// Why a patient review was started by the dispatcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewReason {
    /// One or more patient source records were added, updated, or withdrawn.
    #[serde(rename = "new records")]
    NewRecords,
    /// The periodic review window elapsed without record changes.
    #[serde(rename = "periodic")]
    Periodic,
    /// A linked patient message arrived or required review.
    #[serde(rename = "message")]
    Message,
    /// A patient follow-up responsibility came due at its current version.
    #[serde(rename = "follow-up")]
    FollowUp,
}

impl ReviewReason {
    /// Wire and storage label for this review reason.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NewRecords => "new records",
            Self::Periodic => "periodic",
            Self::Message => "message",
            Self::FollowUp => "follow-up",
        }
    }

    /// Parses a stored or wire review reason label.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim() {
            "new records" => Some(Self::NewRecords),
            "periodic" => Some(Self::Periodic),
            "message" => Some(Self::Message),
            "follow-up" => Some(Self::FollowUp),
            _ => None,
        }
    }

    /// Deterministically selects the primary headline reason from coalesced dispatch inputs.
    #[must_use]
    pub fn select_primary(inputs: &[ReviewDispatchInput]) -> Option<Self> {
        if inputs
            .iter()
            .any(|input| matches!(input, ReviewDispatchInput::LinkedMessage(_)))
        {
            return Some(Self::Message);
        }
        if inputs
            .iter()
            .any(|input| matches!(input, ReviewDispatchInput::ChangedRecords { .. }))
        {
            return Some(Self::NewRecords);
        }
        if inputs
            .iter()
            .any(|input| matches!(input, ReviewDispatchInput::DueFollowUp(_)))
        {
            return Some(Self::FollowUp);
        }
        if inputs
            .iter()
            .any(|input| matches!(input, ReviewDispatchInput::Periodic { .. }))
        {
            return Some(Self::Periodic);
        }
        None
    }
}

impl fmt::Display for ReviewReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Schedule timing for an active patient follow-up responsibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FollowUpScheduleState {
    /// Waiting until `due_at` (or due once `now >= due_at`).
    Scheduled {
        /// Unix timestamp when this follow-up comes due.
        due_at: i64,
    },
    /// Explicitly marked due immediately by a physician or resume action.
    DueNow {
        /// Effective due timestamp recorded when marked due now.
        due_at: i64,
    },
}

/// Dispatch-relevant lifecycle standing of a patient follow-up responsibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum FollowUpDispatchStanding {
    /// Live follow-up that can trigger a review when its schedule is due.
    Active {
        /// Due schedule for this active follow-up.
        schedule: FollowUpScheduleState,
    },
    /// Paused by a physician; starts no review while held.
    Held,
    /// Cannot restart automatically and awaits physician instruction; starts no review.
    CannotRestart,
    /// Terminal completed standing; starts no review.
    Completed,
    /// Terminal cancelled standing; starts no review.
    Cancelled,
    /// Terminal withdrawn wrong-patient standing; starts no review.
    WithdrawnWrongPatient,
}

/// Candidate patient follow-up evaluated during a dispatch wake-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUpCandidate {
    /// Stable follow-up identifier.
    pub follow_up_id: String,
    /// Current monotonic version of the follow-up responsibility.
    pub version: u64,
    /// Plain clinical description of what Coda is watching for.
    pub summary: String,
    /// Current standing and due schedule.
    pub standing: FollowUpDispatchStanding,
}

impl FollowUpCandidate {
    /// Single decision rule determining whether this follow-up is eligible to start
    /// a review of its current version at `now`.
    ///
    /// Only [`FollowUpDispatchStanding::Active`] follow-ups whose schedule is
    /// [`FollowUpScheduleState::DueNow`] or has `due_at <= now` produce a trigger.
    /// Held, cannot-restart, completed, cancelled, and withdrawn follow-ups never
    /// start a review.
    #[must_use]
    pub fn due_trigger_at(&self, now: i64) -> Option<DueFollowUpTrigger> {
        match self.standing {
            FollowUpDispatchStanding::Active {
                schedule: FollowUpScheduleState::DueNow { due_at },
            } => Some(DueFollowUpTrigger {
                follow_up_id: self.follow_up_id.clone(),
                version: self.version,
                summary: self.summary.clone(),
                due_at,
            }),
            FollowUpDispatchStanding::Active {
                schedule: FollowUpScheduleState::Scheduled { due_at },
            } if due_at <= now => Some(DueFollowUpTrigger {
                follow_up_id: self.follow_up_id.clone(),
                version: self.version,
                summary: self.summary.clone(),
                due_at,
            }),
            FollowUpDispatchStanding::Active {
                schedule: FollowUpScheduleState::Scheduled { .. },
            }
            | FollowUpDispatchStanding::Held
            | FollowUpDispatchStanding::CannotRestart
            | FollowUpDispatchStanding::Completed
            | FollowUpDispatchStanding::Cancelled
            | FollowUpDispatchStanding::WithdrawnWrongPatient => None,
        }
    }
}

/// Linked patient message wake event eligible to start a patient review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedMessageTrigger {
    /// Stable deduplication key for this message wake event.
    pub trigger_key: String,
    /// Linked patient conversation identifier.
    pub conversation_id: String,
    /// Linked message identifiers carried by this wake event.
    pub message_ids: Vec<String>,
    /// Unix second when the linked message trigger was recorded.
    pub arrived_at: i64,
}

/// Due patient follow-up pinned to its current version for review execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DueFollowUpTrigger {
    /// Stable follow-up identifier.
    pub follow_up_id: String,
    /// Exact follow-up version targeted by this review.
    pub version: u64,
    /// Clinical summary of the follow-up responsibility at `version`.
    pub summary: String,
    /// Effective due timestamp of the follow-up.
    pub due_at: i64,
}

/// Typed motivating input carried inside a single coalesced patient review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReviewDispatchInput {
    /// Pulled or observed chart sources changed relative to held records.
    ChangedRecords {
        /// Observed workspace source version.
        workspace_version: u64,
        /// Source record keys that were added, modified, or withdrawn.
        changed_candidates: Vec<String>,
    },
    /// Periodic whole-patient review window elapsed without record changes.
    Periodic {
        /// Observed workspace source version.
        workspace_version: u64,
        /// Previous whole-patient review submission timestamp, if any.
        last_reviewed_at: Option<i64>,
    },
    /// One or more linked patient messages arrived on a conversation.
    LinkedMessage(LinkedMessageTrigger),
    /// A patient follow-up came due at its current version.
    DueFollowUp(DueFollowUpTrigger),
}

/// How source records were observed during a patient dispatch wake-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", content = "sources", rename_all = "snake_case")]
pub enum WakeSourceObservation {
    /// No source check ran on this wake-up.
    NotChecked,
    /// Scheduled cadence source check completed and advances check timestamps.
    ScheduledCheck(CheckedSources),
    /// Event-driven workspace snapshot taken alongside a message or follow-up wake
    /// so concurrent record changes coalesce into the same review without advancing
    /// scheduled source-check timers when unchanged.
    EventDrivenSnapshot(CheckedSources),
}

/// All candidate inputs observed for one patient during a dispatcher wake-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientWakeInputs {
    /// Source check or workspace snapshot observed on this wake-up.
    pub sources: WakeSourceObservation,
    /// Linked message wake events awaiting review dispatch.
    pub linked_messages: Vec<LinkedMessageTrigger>,
    /// Candidate follow-ups evaluated against `now`.
    pub follow_ups: Vec<FollowUpCandidate>,
    /// Current workspace source version for non-source-check triggers.
    pub workspace_version: u64,
    /// Current active source revision map for non-source-check triggers.
    pub record_versions: BTreeMap<String, String>,
}

/// Whether a source check observed record changes compared with what Coda held.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordChange {
    /// Pulled sources match Coda's held records.
    Unchanged,
    /// Pulled sources added, updated, or withdrew one or more records.
    Changed,
}

/// Snapshot returned by a successful patient source check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckedSources {
    /// Whether the source check pulled record changes relative to held records.
    pub change: RecordChange,
    /// Monotonic workspace source version observed after the check.
    pub workspace_version: u64,
    /// Active source identifiers mapped to their observed revision tokens.
    pub record_versions: BTreeMap<String, String>,
}

impl CheckedSources {
    /// Constructs a snapshot where pulled sources match what Coda already held.
    #[must_use]
    pub fn unchanged(workspace_version: u64, record_versions: BTreeMap<String, String>) -> Self {
        Self {
            change: RecordChange::Unchanged,
            workspace_version,
            record_versions,
        }
    }

    /// Constructs a snapshot where pulled sources changed at least one record.
    #[must_use]
    pub fn changed(workspace_version: u64, record_versions: BTreeMap<String, String>) -> Self {
        Self {
            change: RecordChange::Changed,
            workspace_version,
            record_versions,
        }
    }
}

/// Durable intent for a review decided by the dispatcher and awaiting once-only submission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingReview {
    /// Deterministic once-only command key for submitting this review request.
    pub command_key: String,
    /// Headline reason the review was triggered.
    pub reason: ReviewReason,
    /// Workspace source version observed when the review was queued.
    pub workspace_version: u64,
    /// Per-source revision map observed when the review was queued.
    pub record_versions: BTreeMap<String, String>,
    /// Unix second when the review was queued.
    pub queued_at: i64,
    /// All coalesced dispatch inputs motivating this single review.
    #[serde(default)]
    pub inputs: Vec<ReviewDispatchInput>,
}

impl PendingReview {
    /// Builds the [`ReviewRequest`] delivered to the patient review receiver.
    #[must_use]
    pub fn to_request(&self, practice_id: &str, patient_workspace_id: &str) -> ReviewRequest {
        ReviewRequest {
            practice_id: practice_id.to_owned(),
            patient_workspace_id: patient_workspace_id.to_owned(),
            command_key: self.command_key.clone(),
            reason: self.reason,
            workspace_version: self.workspace_version,
            record_versions: self.record_versions.clone(),
        }
    }

    /// Whether this pending review includes a whole-chart (`ChangedRecords` or `Periodic`) input.
    #[must_use]
    pub fn has_chart_review_input(&self) -> bool {
        self.inputs.is_empty()
            || self.inputs.iter().any(|input| {
                matches!(
                    input,
                    ReviewDispatchInput::ChangedRecords { .. }
                        | ReviewDispatchInput::Periodic { .. }
                )
            })
    }
}

/// Review request emitted by dispatch for the patient review background job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewRequest {
    /// Owning practice identifier.
    pub practice_id: String,
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Deterministic once-only command key.
    pub command_key: String,
    /// Why this review was requested (`"new records"`, `"periodic"`, `"message"`, or `"follow-up"`).
    pub reason: ReviewReason,
    /// Observed workspace source version at the time of the check.
    pub workspace_version: u64,
    /// Active source identifiers mapped to their observed revisions.
    pub record_versions: BTreeMap<String, String>,
}

/// Operator-facing cadence and dispatch timing view for one patient.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorCadenceView {
    /// Owning practice identifier.
    pub practice_id: String,
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Configured source-check and periodic-review cadence.
    pub cadence: Cadence,
    /// Next Unix second when this patient is due for dispatcher attention.
    pub next_due_at: i64,
    /// Unix second of the most recent source-check attempt, whether it succeeded or failed.
    pub last_attempt_at: Option<i64>,
    /// Unix second of the most recent successful source check.
    pub last_success_at: Option<i64>,
}

/// Operator-facing practice-level cadence default and affected patient views.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticeCadenceView {
    /// Practice identifier.
    pub practice_id: String,
    /// Practice default check and periodic-review cadence.
    pub cadence: Cadence,
    /// Updated operator cadence views across registered patients in `practice_id`.
    pub patients: Vec<OperatorCadenceView>,
}

/// Saved dispatch state for one registered patient.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispatchState {
    /// Configured check and periodic-review cadence.
    pub cadence: Cadence,
    /// Unix second when dispatch tracking began for this patient.
    pub registered_at: i64,
    /// Unix second of the most recent source-check attempt.
    pub last_attempt_at: Option<i64>,
    /// Unix second of the most recent successful source check.
    pub last_success_at: Option<i64>,
    /// Workspace source version observed at the most recent successful source check.
    pub last_checked_version: Option<u64>,
    /// Per-source revisions observed at the most recent successful source check.
    pub last_checked_records: BTreeMap<String, String>,
    /// Unix second when a review was most recently submitted.
    pub last_reviewed_at: Option<i64>,
    /// Workspace source version covered by the most recently submitted review.
    pub last_reviewed_version: Option<u64>,
    /// Monotonic counter of queued reviews for deterministic once-only command keys.
    pub review_sequence: u64,
    /// Review decided after a wake-up that has not yet completed submission.
    pub pending_review: Option<PendingReview>,
    /// Message trigger keys already handed off to review execution.
    #[serde(default)]
    pub dispatched_message_keys: BTreeSet<String>,
    /// Highest follow-up version already handed off per `follow_up_id`.
    #[serde(default)]
    pub dispatched_follow_up_versions: BTreeMap<String, u64>,
}

impl DispatchState {
    /// Initializes dispatch state for a newly tracked patient at `registered_at`.
    #[must_use]
    pub fn new(cadence: Cadence, registered_at: i64) -> Self {
        Self {
            cadence,
            registered_at,
            last_attempt_at: None,
            last_success_at: None,
            last_checked_version: None,
            last_checked_records: BTreeMap::new(),
            last_reviewed_at: None,
            last_reviewed_version: None,
            review_sequence: 0,
            pending_review: None,
            dispatched_message_keys: BTreeSet::new(),
            dispatched_follow_up_versions: BTreeMap::new(),
        }
    }

    /// Computes the next Unix second when this patient requires dispatcher work.
    ///
    /// A pending review from an interrupted wake-up is due immediately at its
    /// queue timestamp. A patient who has never completed a successful check is
    /// due at `registered_at`. Otherwise the patient is due at the earlier of
    /// the next scheduled source check (`last_success_at + check_every`) and
    /// the next periodic review (`review_anchor + review_every`).
    #[must_use]
    pub fn next_due_at(&self) -> i64 {
        if let Some(pending) = &self.pending_review {
            return pending.queued_at;
        }
        let Some(last_success) = self.last_success_at else {
            return self.registered_at;
        };
        let next_check =
            last_success.saturating_add(self.cadence.check_every_seconds().cast_signed());
        let review_anchor = self.last_reviewed_at.unwrap_or(self.registered_at);
        let next_periodic =
            review_anchor.saturating_add(self.cadence.review_every_seconds().cast_signed());
        next_check.min(next_periodic)
    }

    /// Whether this patient is due for a source check or pending-review replay at `now`.
    #[must_use]
    pub fn is_due_at(&self, now: i64) -> bool {
        now >= self.next_due_at()
    }

    /// Computes the next Unix second when a scheduled source fetch is due,
    /// anchored on `source_last_good_read_at` (or `self.last_success_at`),
    /// or `self.registered_at` before any successful check.
    #[must_use]
    pub fn next_source_check_at(&self, source_last_good_read_at: Option<i64>) -> i64 {
        match source_last_good_read_at.or(self.last_success_at) {
            None => self.registered_at,
            Some(last_good) => {
                last_good.saturating_add(self.cadence.check_every_seconds().cast_signed())
            }
        }
    }

    /// Builds the operator inspection projection for `(practice_id, patient_workspace_id)`.
    #[must_use]
    pub fn operator_view(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> OperatorCadenceView {
        OperatorCadenceView {
            practice_id: practice_id.to_owned(),
            patient_workspace_id: patient_workspace_id.to_owned(),
            cadence: self.cadence,
            next_due_at: self.next_due_at(),
            last_attempt_at: self.last_attempt_at,
            last_success_at: self.last_success_at,
        }
    }

    /// Records a failed source check at `now` without advancing `last_success_at`
    /// or queueing a review.
    pub fn record_failed_check(&mut self, now: i64) {
        self.last_attempt_at = Some(now);
    }

    /// Evaluates and applies a successful source check at `now`, queueing a
    /// [`PendingReview`] when records changed or a periodic review came due.
    pub fn record_successful_check(
        &mut self,
        practice_id: &str,
        patient_workspace_id: &str,
        checked: CheckedSources,
        now: i64,
    ) -> Option<PendingReview> {
        let workspace_version = checked.workspace_version;
        let record_versions = checked.record_versions.clone();
        self.evaluate_patient_wakeup(
            practice_id,
            patient_workspace_id,
            PatientWakeInputs {
                sources: WakeSourceObservation::ScheduledCheck(checked),
                linked_messages: Vec::new(),
                follow_ups: Vec::new(),
                workspace_version,
                record_versions,
            },
            now,
        )
    }

    /// Evaluates all wake-up triggers for one patient — scheduled or event-driven
    /// source changes, linked messages, and due follow-ups — and coalesces every
    /// eligible input for this patient into a single [`PendingReview`].
    ///
    /// Replay-safe: triggers already recorded in `dispatched_message_keys` or
    /// `dispatched_follow_up_versions` (or already queued in `pending_review`) are
    /// never duplicated. If a new trigger arrives before an earlier `pending_review`
    /// finishes handoff, it merges into that single `PendingReview`.
    pub fn evaluate_patient_wakeup(
        &mut self,
        practice_id: &str,
        patient_workspace_id: &str,
        wake: PatientWakeInputs,
        now: i64,
    ) -> Option<PendingReview> {
        let mut new_inputs = Vec::new();
        let mut effective_version = wake.workspace_version;
        let mut effective_records = wake.record_versions;

        match wake.sources {
            WakeSourceObservation::NotChecked => {}
            WakeSourceObservation::ScheduledCheck(checked) => {
                let changed_candidates =
                    diff_record_keys(&self.last_checked_records, &checked.record_versions);
                let reason = evaluate_source_check(self, &checked, now);
                self.last_attempt_at = Some(now);
                self.last_success_at = Some(now);
                self.last_checked_version = Some(checked.workspace_version);
                self.last_checked_records = checked.record_versions.clone();
                effective_version = checked.workspace_version;
                effective_records = checked.record_versions;
                match reason {
                    Some(ReviewReason::NewRecords) => {
                        new_inputs.push(ReviewDispatchInput::ChangedRecords {
                            workspace_version: effective_version,
                            changed_candidates,
                        });
                    }
                    Some(ReviewReason::Periodic) => {
                        new_inputs.push(ReviewDispatchInput::Periodic {
                            workspace_version: effective_version,
                            last_reviewed_at: self.last_reviewed_at,
                        });
                    }
                    Some(ReviewReason::Message | ReviewReason::FollowUp) | None => {}
                }
            }
            WakeSourceObservation::EventDrivenSnapshot(checked) => {
                let records_changed = has_record_change(self, &checked);
                if records_changed {
                    let changed_candidates =
                        diff_record_keys(&self.last_checked_records, &checked.record_versions);
                    self.last_checked_version = Some(checked.workspace_version);
                    self.last_checked_records = checked.record_versions.clone();
                    effective_version = checked.workspace_version;
                    effective_records = checked.record_versions;
                    new_inputs.push(ReviewDispatchInput::ChangedRecords {
                        workspace_version: effective_version,
                        changed_candidates,
                    });
                } else {
                    effective_version = checked.workspace_version;
                    effective_records = checked.record_versions;
                }
            }
        }

        for msg in wake.linked_messages {
            if self.dispatched_message_keys.contains(&msg.trigger_key) {
                continue;
            }
            new_inputs.push(ReviewDispatchInput::LinkedMessage(msg));
        }

        for candidate in &wake.follow_ups {
            let Some(due) = candidate.due_trigger_at(now) else {
                continue;
            };
            let dispatched_version = self
                .dispatched_follow_up_versions
                .get(&due.follow_up_id)
                .copied()
                .unwrap_or(0);
            if due.version <= dispatched_version {
                continue;
            }
            new_inputs.push(ReviewDispatchInput::DueFollowUp(due));
        }

        if let Some(existing) = &mut self.pending_review {
            for input in new_inputs {
                merge_dispatch_input(&mut existing.inputs, input);
            }
            existing.workspace_version = existing.workspace_version.max(effective_version);
            if !effective_records.is_empty() {
                existing.record_versions = effective_records;
            }
            if let Some(primary) = ReviewReason::select_primary(&existing.inputs) {
                existing.reason = primary;
            }
            return Some(existing.clone());
        }

        let mut coalesced = Vec::new();
        for input in new_inputs {
            merge_dispatch_input(&mut coalesced, input);
        }
        let reason = ReviewReason::select_primary(&coalesced)?;
        self.review_sequence = self.review_sequence.saturating_add(1);
        let pending = PendingReview {
            command_key: format!(
                "dispatch-review:{practice_id}:{patient_workspace_id}:{}",
                self.review_sequence
            ),
            reason,
            workspace_version: effective_version,
            record_versions: effective_records,
            queued_at: now,
            inputs: coalesced,
        };
        self.pending_review = Some(pending.clone());
        Some(pending)
    }

    /// Marks the currently pending review submitted at `now` and records its
    /// message and follow-up version watermarks so replay wakes never duplicate it.
    pub fn complete_pending_review(&mut self, now: i64) {
        if let Some(pending) = self.pending_review.take() {
            let updates_chart_anchor = pending.has_chart_review_input();
            for input in &pending.inputs {
                match input {
                    ReviewDispatchInput::LinkedMessage(msg) => {
                        self.dispatched_message_keys.insert(msg.trigger_key.clone());
                    }
                    ReviewDispatchInput::DueFollowUp(follow_up) => {
                        let entry = self
                            .dispatched_follow_up_versions
                            .entry(follow_up.follow_up_id.clone())
                            .or_insert(0);
                        *entry = (*entry).max(follow_up.version);
                    }
                    ReviewDispatchInput::ChangedRecords { .. }
                    | ReviewDispatchInput::Periodic { .. } => {}
                }
            }
            if updates_chart_anchor {
                self.last_reviewed_at = Some(now);
                self.last_reviewed_version = Some(pending.workspace_version);
            }
        }
    }
}

fn has_record_change(state: &DispatchState, checked: &CheckedSources) -> bool {
    checked.change == RecordChange::Changed
        || state.last_checked_version.is_some_and(|previous_version| {
            previous_version != checked.workspace_version
                || state.last_checked_records != checked.record_versions
        })
}

fn diff_record_keys(
    previous: &BTreeMap<String, String>,
    current: &BTreeMap<String, String>,
) -> Vec<String> {
    if previous.is_empty() {
        return current.keys().cloned().collect();
    }
    let mut keys = BTreeSet::new();
    for (key, rev) in current {
        if previous.get(key) != Some(rev) {
            keys.insert(key.clone());
        }
    }
    for key in previous.keys() {
        if !current.contains_key(key) {
            keys.insert(key.clone());
        }
    }
    if keys.is_empty() {
        current.keys().cloned().collect()
    } else {
        keys.into_iter().collect()
    }
}

fn merge_dispatch_input(target: &mut Vec<ReviewDispatchInput>, candidate: ReviewDispatchInput) {
    match candidate {
        ReviewDispatchInput::ChangedRecords {
            workspace_version,
            changed_candidates,
        } => {
            if let Some(ReviewDispatchInput::ChangedRecords {
                workspace_version: existing_ver,
                changed_candidates: existing_keys,
            }) = target
                .iter_mut()
                .find(|item| matches!(item, ReviewDispatchInput::ChangedRecords { .. }))
            {
                *existing_ver = (*existing_ver).max(workspace_version);
                for key in changed_candidates {
                    if !existing_keys.contains(&key) {
                        existing_keys.push(key);
                    }
                }
                existing_keys.sort();
            } else {
                target.push(ReviewDispatchInput::ChangedRecords {
                    workspace_version,
                    changed_candidates,
                });
            }
            target.retain(|item| !matches!(item, ReviewDispatchInput::Periodic { .. }));
        }
        ReviewDispatchInput::Periodic {
            workspace_version,
            last_reviewed_at,
        } => {
            let already_has_chart_input = target.iter().any(|item| {
                matches!(
                    item,
                    ReviewDispatchInput::ChangedRecords { .. }
                        | ReviewDispatchInput::Periodic { .. }
                )
            });
            if !already_has_chart_input {
                target.push(ReviewDispatchInput::Periodic {
                    workspace_version,
                    last_reviewed_at,
                });
            }
        }
        ReviewDispatchInput::LinkedMessage(msg) => {
            let duplicate = target.iter().any(|item| {
                matches!(
                    item,
                    ReviewDispatchInput::LinkedMessage(existing)
                        if existing.trigger_key == msg.trigger_key
                )
            });
            if !duplicate {
                target.push(ReviewDispatchInput::LinkedMessage(msg));
            }
        }
        ReviewDispatchInput::DueFollowUp(follow_up) => {
            if let Some(ReviewDispatchInput::DueFollowUp(existing)) =
                target.iter_mut().find(|item| {
                    matches!(
                        item,
                        ReviewDispatchInput::DueFollowUp(existing)
                            if existing.follow_up_id == follow_up.follow_up_id
                    )
                })
            {
                if follow_up.version >= existing.version {
                    *existing = follow_up;
                }
            } else {
                target.push(ReviewDispatchInput::DueFollowUp(follow_up));
            }
        }
    }
}

/// Single decision rule mapping a successful source check to an optional [`ReviewReason`].
///
/// Changed records always request one review with [`ReviewReason::NewRecords`].
/// When records did not change, a periodic review is requested with
/// [`ReviewReason::Periodic`] once `review_every_seconds` has elapsed since the
/// last submitted review (or `registered_at` before the first review).
#[must_use]
pub fn evaluate_source_check(
    state: &DispatchState,
    checked: &CheckedSources,
    now: i64,
) -> Option<ReviewReason> {
    if has_record_change(state, checked) {
        return Some(ReviewReason::NewRecords);
    }
    let review_anchor = state.last_reviewed_at.unwrap_or(state.registered_at);
    let periodic_due_at =
        review_anchor.saturating_add(state.cadence.review_every_seconds().cast_signed());
    if now >= periodic_due_at {
        return Some(ReviewReason::Periodic);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const START: i64 = 1_760_000_000;
    const DAY: i64 = DEFAULT_CHECK_EVERY_SECONDS as i64;
    const WEEK: i64 = DEFAULT_REVIEW_EVERY_SECONDS as i64;

    fn sample_records(revision: &str) -> BTreeMap<String, String> {
        BTreeMap::from([("ehr:chart-1".to_owned(), revision.to_owned())])
    }

    #[test]
    fn default_cadence_checks_daily_and_reviews_weekly() {
        let cadence = Cadence::default();
        assert_eq!(cadence.check_every_seconds(), 86_400);
        assert_eq!(cadence.review_every_seconds(), 604_800);
        assert_eq!(
            Cadence::new(0, 604_800).unwrap_err(),
            CadenceError::ZeroCheckInterval
        );
        assert_eq!(
            Cadence::new(86_400, 0).unwrap_err(),
            CadenceError::ZeroReviewInterval
        );
    }

    #[test]
    fn review_reason_serializes_exact_wire_labels() {
        assert_eq!(
            serde_json::to_string(&ReviewReason::NewRecords).unwrap(),
            "\"new records\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewReason::Periodic).unwrap(),
            "\"periodic\""
        );
        assert_eq!(
            ReviewReason::parse("new records"),
            Some(ReviewReason::NewRecords)
        );
        assert_eq!(
            ReviewReason::parse("periodic"),
            Some(ReviewReason::Periodic)
        );
        assert_eq!(
            serde_json::to_string(&ReviewReason::Message).unwrap(),
            "\"message\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewReason::FollowUp).unwrap(),
            "\"follow-up\""
        );
        assert_eq!(ReviewReason::parse("message"), Some(ReviewReason::Message));
        assert_eq!(
            ReviewReason::parse("follow-up"),
            Some(ReviewReason::FollowUp)
        );
        assert_eq!(ReviewReason::parse("unknown"), None);
    }

    #[test]
    fn unchanged_sources_within_weekly_window_start_no_review() {
        let mut state = DispatchState::new(Cadence::default(), START);
        assert!(state.is_due_at(START));

        let pending = state.record_successful_check(
            "practice-1",
            "patient-1",
            CheckedSources::unchanged(1, sample_records("r1")),
            START + DAY,
        );
        assert_eq!(pending, None);
        assert_eq!(state.last_attempt_at, Some(START + DAY));
        assert_eq!(state.last_success_at, Some(START + DAY));
        assert_eq!(state.next_due_at(), START + 2 * DAY);
    }

    #[test]
    fn changed_records_queue_one_new_records_review() {
        let mut state = DispatchState::new(Cadence::default(), START);
        assert_eq!(
            state.record_successful_check(
                "practice-1",
                "patient-1",
                CheckedSources::unchanged(1, sample_records("r1")),
                START + DAY,
            ),
            None
        );

        let pending = state
            .record_successful_check(
                "practice-1",
                "patient-1",
                CheckedSources::changed(2, sample_records("r2")),
                START + 2 * DAY,
            )
            .expect("changed records must queue one review");
        assert_eq!(pending.reason, ReviewReason::NewRecords);
        assert_eq!(pending.reason.as_str(), "new records");
        assert_eq!(pending.workspace_version, 2);
        assert_eq!(pending.record_versions, sample_records("r2"));
        assert!(
            state.is_due_at(START + 2 * DAY),
            "pending review remains due until submitted"
        );

        state.complete_pending_review(START + 2 * DAY);
        assert!(state.pending_review.is_none());
        assert_eq!(state.next_due_at(), START + 3 * DAY);
    }

    #[test]
    fn elapsed_weekly_window_without_changes_queues_one_periodic_review() {
        let mut state = DispatchState::new(Cadence::default(), START);
        for day in 1..7 {
            assert_eq!(
                state.record_successful_check(
                    "practice-1",
                    "patient-1",
                    CheckedSources::unchanged(1, sample_records("r1")),
                    START + day * DAY,
                ),
                None
            );
        }

        let pending = state
            .record_successful_check(
                "practice-1",
                "patient-1",
                CheckedSources::unchanged(1, sample_records("r1")),
                START + WEEK,
            )
            .expect("weekly window must trigger periodic review");
        assert_eq!(pending.reason, ReviewReason::Periodic);
        assert_eq!(pending.reason.as_str(), "periodic");
        state.complete_pending_review(START + WEEK);

        assert_eq!(
            state.record_successful_check(
                "practice-1",
                "patient-1",
                CheckedSources::unchanged(1, sample_records("r1")),
                START + WEEK + DAY,
            ),
            None,
            "completed periodic review resets the weekly window"
        );
    }

    #[test]
    fn failed_check_records_attempt_without_advancing_last_success_or_next_due() {
        let mut state = DispatchState::new(Cadence::default(), START);
        state.record_successful_check(
            "practice-1",
            "patient-1",
            CheckedSources::unchanged(1, sample_records("r1")),
            START,
        );
        assert_eq!(state.next_due_at(), START + DAY);

        state.record_failed_check(START + DAY);
        assert_eq!(state.last_attempt_at, Some(START + DAY));
        assert_eq!(state.last_success_at, Some(START));
        assert_eq!(state.next_due_at(), START + DAY);
        assert!(state.is_due_at(START + DAY));
    }

    #[test]
    fn linked_message_starts_one_review_with_message_reason() {
        let mut state = DispatchState::new(Cadence::default(), START);
        state.record_successful_check(
            "practice-1",
            "patient-1",
            CheckedSources::unchanged(1, sample_records("r1")),
            START,
        );

        let trigger = LinkedMessageTrigger {
            trigger_key: "msg:conv-1:m-1".to_owned(),
            conversation_id: "conv-1".to_owned(),
            message_ids: vec!["m-1".to_owned()],
            arrived_at: START + 120,
        };
        let pending = state
            .evaluate_patient_wakeup(
                "practice-1",
                "patient-1",
                PatientWakeInputs {
                    sources: WakeSourceObservation::EventDrivenSnapshot(CheckedSources::unchanged(
                        1,
                        sample_records("r1"),
                    )),
                    linked_messages: vec![trigger.clone()],
                    follow_ups: Vec::new(),
                    workspace_version: 1,
                    record_versions: sample_records("r1"),
                },
                START + 120,
            )
            .expect("linked message must queue one review");
        assert_eq!(pending.reason, ReviewReason::Message);
        assert_eq!(pending.reason.as_str(), "message");
        assert_eq!(
            pending.inputs,
            vec![ReviewDispatchInput::LinkedMessage(trigger.clone())]
        );

        state.complete_pending_review(START + 120);
        let replay = state.evaluate_patient_wakeup(
            "practice-1",
            "patient-1",
            PatientWakeInputs {
                sources: WakeSourceObservation::EventDrivenSnapshot(CheckedSources::unchanged(
                    1,
                    sample_records("r1"),
                )),
                linked_messages: vec![trigger],
                follow_ups: Vec::new(),
                workspace_version: 1,
                record_versions: sample_records("r1"),
            },
            START + 180,
        );
        assert_eq!(replay, None, "settled message trigger must not repeat");
    }

    #[test]
    fn due_follow_up_starts_one_review_of_current_version_and_inactive_start_none() {
        let mut state = DispatchState::new(Cadence::default(), START);
        state.record_successful_check(
            "practice-1",
            "patient-1",
            CheckedSources::unchanged(1, sample_records("r1")),
            START,
        );

        let inactive_candidates = vec![
            FollowUpCandidate {
                follow_up_id: "fu-held".to_owned(),
                version: 2,
                summary: "Held potassium recheck".to_owned(),
                standing: FollowUpDispatchStanding::Held,
            },
            FollowUpCandidate {
                follow_up_id: "fu-completed".to_owned(),
                version: 3,
                summary: "Completed referral".to_owned(),
                standing: FollowUpDispatchStanding::Completed,
            },
            FollowUpCandidate {
                follow_up_id: "fu-cancelled".to_owned(),
                version: 4,
                summary: "Cancelled check".to_owned(),
                standing: FollowUpDispatchStanding::Cancelled,
            },
            FollowUpCandidate {
                follow_up_id: "fu-cannot-restart".to_owned(),
                version: 1,
                summary: "Stopped during review".to_owned(),
                standing: FollowUpDispatchStanding::CannotRestart,
            },
            FollowUpCandidate {
                follow_up_id: "fu-future".to_owned(),
                version: 1,
                summary: "Future A1c".to_owned(),
                standing: FollowUpDispatchStanding::Active {
                    schedule: FollowUpScheduleState::Scheduled {
                        due_at: START + 3 * DAY,
                    },
                },
            },
        ];

        assert_eq!(
            state.evaluate_patient_wakeup(
                "practice-1",
                "patient-1",
                PatientWakeInputs {
                    sources: WakeSourceObservation::NotChecked,
                    linked_messages: Vec::new(),
                    follow_ups: inactive_candidates.clone(),
                    workspace_version: 1,
                    record_versions: sample_records("r1"),
                },
                START + 600,
            ),
            None,
            "held, completed, cancelled, cannot-restart, and future follow-ups must start no review"
        );

        let mut with_due = inactive_candidates;
        with_due.push(FollowUpCandidate {
            follow_up_id: "fu-active".to_owned(),
            version: 5,
            summary: "Repeat BMP after lisinopril increase".to_owned(),
            standing: FollowUpDispatchStanding::Active {
                schedule: FollowUpScheduleState::Scheduled {
                    due_at: START + 300,
                },
            },
        });

        let pending = state
            .evaluate_patient_wakeup(
                "practice-1",
                "patient-1",
                PatientWakeInputs {
                    sources: WakeSourceObservation::NotChecked,
                    linked_messages: Vec::new(),
                    follow_ups: with_due.clone(),
                    workspace_version: 1,
                    record_versions: sample_records("r1"),
                },
                START + 600,
            )
            .expect("due active follow-up must start one review");
        assert_eq!(pending.reason, ReviewReason::FollowUp);
        assert_eq!(pending.reason.as_str(), "follow-up");
        assert_eq!(
            pending.inputs,
            vec![ReviewDispatchInput::DueFollowUp(DueFollowUpTrigger {
                follow_up_id: "fu-active".to_owned(),
                version: 5,
                summary: "Repeat BMP after lisinopril increase".to_owned(),
                due_at: START + 300,
            })]
        );
        state.complete_pending_review(START + 600);

        assert_eq!(
            state.evaluate_patient_wakeup(
                "practice-1",
                "patient-1",
                PatientWakeInputs {
                    sources: WakeSourceObservation::NotChecked,
                    linked_messages: Vec::new(),
                    follow_ups: with_due,
                    workspace_version: 1,
                    record_versions: sample_records("r1"),
                },
                START + 660,
            ),
            None,
            "same follow-up version must not start a second review"
        );

        let updated_version = vec![FollowUpCandidate {
            follow_up_id: "fu-active".to_owned(),
            version: 6,
            summary: "Repeat BMP and creatinine after lisinopril increase".to_owned(),
            standing: FollowUpDispatchStanding::Active {
                schedule: FollowUpScheduleState::DueNow {
                    due_at: START + 700,
                },
            },
        }];
        let next_pending = state
            .evaluate_patient_wakeup(
                "practice-1",
                "patient-1",
                PatientWakeInputs {
                    sources: WakeSourceObservation::NotChecked,
                    linked_messages: Vec::new(),
                    follow_ups: updated_version,
                    workspace_version: 1,
                    record_versions: sample_records("r1"),
                },
                START + 700,
            )
            .expect("updated follow-up version must start one review of the new current version");
        assert_eq!(
            next_pending.inputs,
            vec![ReviewDispatchInput::DueFollowUp(DueFollowUpTrigger {
                follow_up_id: "fu-active".to_owned(),
                version: 6,
                summary: "Repeat BMP and creatinine after lisinopril increase".to_owned(),
                due_at: START + 700,
            })]
        );
    }

    #[test]
    fn linked_message_and_record_change_coalesce_into_one_review() {
        let mut state = DispatchState::new(Cadence::default(), START);
        state.record_successful_check(
            "practice-1",
            "patient-1",
            CheckedSources::unchanged(1, sample_records("r1")),
            START,
        );

        let msg = LinkedMessageTrigger {
            trigger_key: "msg:conv-9:m-22".to_owned(),
            conversation_id: "conv-9".to_owned(),
            message_ids: vec!["m-22".to_owned()],
            arrived_at: START + DAY,
        };
        let pending = state
            .evaluate_patient_wakeup(
                "practice-1",
                "patient-1",
                PatientWakeInputs {
                    sources: WakeSourceObservation::ScheduledCheck(CheckedSources::changed(
                        2,
                        sample_records("r2"),
                    )),
                    linked_messages: vec![msg.clone()],
                    follow_ups: Vec::new(),
                    workspace_version: 2,
                    record_versions: sample_records("r2"),
                },
                START + DAY,
            )
            .expect("linked message and record change must coalesce into one review");

        assert_eq!(state.review_sequence, 1, "only one review may be queued");
        assert_eq!(pending.reason, ReviewReason::Message);
        assert_eq!(pending.workspace_version, 2);
        assert_eq!(
            pending.inputs,
            vec![
                ReviewDispatchInput::ChangedRecords {
                    workspace_version: 2,
                    changed_candidates: vec!["ehr:chart-1".to_owned()],
                },
                ReviewDispatchInput::LinkedMessage(msg),
            ]
        );
    }
}
