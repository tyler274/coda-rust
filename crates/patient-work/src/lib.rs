//! Per-patient source-check cadence, review reasons, and dispatch state rules.
//!
//! Each registered patient has a [`Cadence`] governing how often sources are
//! checked for record changes and how often a periodic whole-patient review
//! runs when no record changes arrive. By default, sources are checked daily
//! and a periodic review runs weekly. Authorized operators inspect and adjust
//! cadence; payment and connection state never alter it.

#![deny(missing_docs)]

use std::collections::BTreeMap;
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

/// Why a patient review was started after checking sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewReason {
    /// One or more patient source records were added, updated, or withdrawn.
    #[serde(rename = "new records")]
    NewRecords,
    /// The periodic review window elapsed without record changes.
    #[serde(rename = "periodic")]
    Periodic,
}

impl ReviewReason {
    /// Wire and storage label for this review reason.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NewRecords => "new records",
            Self::Periodic => "periodic",
        }
    }

    /// Parses a stored or wire review reason label.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim() {
            "new records" => Some(Self::NewRecords),
            "periodic" => Some(Self::Periodic),
            _ => None,
        }
    }
}

impl fmt::Display for ReviewReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
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

/// Durable intent for a review decided by a source check and awaiting once-only submission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingReview {
    /// Deterministic once-only command key for submitting this review request.
    pub command_key: String,
    /// Reason the review was triggered.
    pub reason: ReviewReason,
    /// Workspace source version observed when the review was queued.
    pub workspace_version: u64,
    /// Per-source revision map observed when the review was queued.
    pub record_versions: BTreeMap<String, String>,
    /// Unix second when the review was queued.
    pub queued_at: i64,
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
    /// Why this review was requested (`"new records"` or `"periodic"`).
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
    /// Review decided after a source check that has not yet completed submission.
    pub pending_review: Option<PendingReview>,
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
        let reason = evaluate_source_check(self, &checked, now);
        self.last_attempt_at = Some(now);
        self.last_success_at = Some(now);
        self.last_checked_version = Some(checked.workspace_version);
        self.last_checked_records = checked.record_versions.clone();
        let reason = reason?;
        self.review_sequence = self.review_sequence.saturating_add(1);
        let pending = PendingReview {
            command_key: format!(
                "dispatch-review:{practice_id}:{patient_workspace_id}:{}",
                self.review_sequence
            ),
            reason,
            workspace_version: checked.workspace_version,
            record_versions: checked.record_versions,
            queued_at: now,
        };
        self.pending_review = Some(pending.clone());
        Some(pending)
    }

    /// Marks the currently pending review submitted at `now`.
    pub fn complete_pending_review(&mut self, now: i64) {
        if let Some(pending) = self.pending_review.take() {
            self.last_reviewed_at = Some(now);
            self.last_reviewed_version = Some(pending.workspace_version);
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
    let records_changed = checked.change == RecordChange::Changed
        || state.last_checked_version.is_some_and(|previous_version| {
            previous_version != checked.workspace_version
                || state.last_checked_records != checked.record_versions
        });
    if records_changed {
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
}
