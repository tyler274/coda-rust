//! The patient-work dispatcher's grouping window.
//!
//! The coda-agents checkout is the reference (`patientWorkDispatcher.ts`,
//! `sourceChangeGrouping.ts`) and is not modified. An arming read reports
//! only a future close. A window already closed when that read finishes is
//! delivered by the pass after it, once. A refusal's record is not a fact
//! that makes work due, so it does not offer the window again.
#![deny(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

/// One day in milliseconds (`86_400_000`).
pub const DAY_MS: u64 = 86_400_000;

/// One week in milliseconds (`604_800_000`).
pub const WEEK_MS: u64 = 7 * DAY_MS;

/// Default interval between patient source checks: one day (`86_400` seconds).
pub const DEFAULT_CHECK_EVERY_SECONDS: u64 = 24 * 60 * 60;

/// Default interval between periodic whole-patient reviews: seven days (`604_800` seconds).
pub const DEFAULT_REVIEW_EVERY_SECONDS: u64 = 7 * 24 * 60 * 60;

/// Per-patient review cadence read by Agents without knowing payment status.
///
/// Every patient receives a settled weekly whole-patient review. A patient with
/// [`ReviewCadence::WeeklyAndDailyWhenChanged`] also receives a daily check
/// that starts a review only when the patient's record or linked health state
/// changed since the previous review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ReviewCadence {
    /// Weekly whole-patient review only.
    #[default]
    Weekly,
    /// Weekly whole-patient review plus a daily review when changed.
    WeeklyAndDailyWhenChanged,
}

impl ReviewCadence {
    /// Wire and storage string for this cadence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Weekly => "weekly",
            Self::WeeklyAndDailyWhenChanged => "weekly_and_daily_when_changed",
        }
    }

    /// Parses a stored or wire cadence identifier.
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim() {
            "weekly" => Some(Self::Weekly),
            "weekly_and_daily_when_changed"
            | "weekly-daily-check"
            | "weekly_daily_when_changed" => Some(Self::WeeklyAndDailyWhenChanged),
            _ => None,
        }
    }

    /// Whether this cadence includes a daily check when the patient changed.
    pub const fn includes_daily_when_changed(self) -> bool {
        matches!(self, Self::WeeklyAndDailyWhenChanged)
    }
}

impl std::fmt::Display for ReviewCadence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The reason [`Dispatch`] started a patient review.
///
/// One scheduling owner drives source-arrival grouping windows, the settled
/// weekly whole-patient review (replacing separate census and memory
/// reconciliation schedules), and the daily-when-changed check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduledReviewKind {
    /// A source-arrival grouping window closed with these source references.
    SourceChanges(Vec<String>),
    /// The patient's settled weekly whole-patient review came due.
    WeeklyWholePatient {
        /// Changes accumulated since the previous review that are folded into
        /// this whole-patient review.
        folded_changes: Vec<String>,
    },
    /// The daily check ran for a [`ReviewCadence::WeeklyAndDailyWhenChanged`]
    /// patient who had changes since the previous review.
    DailyWhenChanged {
        /// Change references accumulated since the previous review.
        changes: Vec<String>,
    },
}

/// A review started by the single patient-work scheduling owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledReview {
    /// Patient identifier.
    pub patient: String,
    /// What triggered this review.
    pub kind: ScheduledReviewKind,
    /// Timestamp when the review started.
    pub started_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PatientSchedule {
    cadence: ReviewCadence,
    last_weekly_at: u64,
    last_daily_check_at: u64,
    daily_changes: Vec<String>,
}

/// One process's grouping-window delivery and per-patient cadence scheduler.
///
/// Time is milliseconds. The caller advances it and then drains.
#[derive(Debug)]
pub struct Dispatch {
    window_ms: u64,
    now: u64,
    arming_delay: u64,
    deaf: bool,
    refusing: bool,
    next_position: u64,
    armed_at: Option<u64>,
    close_attempts: u32,
    facts: Vec<Fact>,
    notices: Vec<Notice>,
    patients: BTreeMap<String, Patient>,
    schedules: BTreeMap<String, PatientSchedule>,
    busy: BTreeSet<String>,
    accepted: Vec<Vec<String>>,
    scheduled: Vec<ScheduledReview>,
}

#[derive(Debug)]
struct Patient {
    pending: Vec<Pending>,
}

#[derive(Debug)]
struct Pending {
    source_ref: String,
    admitted_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FactKind {
    /// An admitted change. This can make work due.
    Admission,
    /// A refused delivery. This cannot make work due.
    Refusal,
}

#[derive(Debug)]
struct Fact {
    patient: String,
    position: u64,
    kind: FactKind,
}

#[derive(Debug)]
struct Notice {
    patient: String,
    /// Absent when the notice stands for commits this dispatcher never heard.
    range: Option<(u64, u64)>,
}

impl Dispatch {
    /// A dispatcher whose grouping window is `window_ms` long. The clock starts at zero.
    pub fn new(window_ms: u64) -> Self {
        Self {
            window_ms,
            now: 0,
            arming_delay: 0,
            deaf: false,
            refusing: false,
            next_position: 1,
            armed_at: None,
            close_attempts: 0,
            facts: Vec::new(),
            notices: Vec::new(),
            patients: BTreeMap::new(),
            schedules: BTreeMap::new(),
            busy: BTreeSet::new(),
            accepted: Vec::new(),
            scheduled: Vec::new(),
        }
    }

    /// Enrolls a patient in the cadence scheduler or updates their [`ReviewCadence`].
    pub fn enroll_patient(&mut self, patient: &str, cadence: ReviewCadence) {
        let now = self.now;
        self.schedules
            .entry(patient.to_owned())
            .and_modify(|schedule| schedule.cadence = cadence)
            .or_insert_with(|| PatientSchedule {
                cadence,
                last_weekly_at: now,
                last_daily_check_at: now,
                daily_changes: Vec::new(),
            });
    }

    /// Sets the per-patient [`ReviewCadence`].
    pub fn set_cadence(&mut self, patient: &str, cadence: ReviewCadence) {
        self.enroll_patient(patient, cadence);
    }

    /// Returns the patient's [`ReviewCadence`], defaulting to [`ReviewCadence::Weekly`].
    pub fn cadence(&self, patient: &str) -> ReviewCadence {
        self.schedules
            .get(patient)
            .map(|schedule| schedule.cadence)
            .unwrap_or_default()
    }

    /// Records that a patient's record or linked health state changed since the last review.
    pub fn record_daily_change(&mut self, patient: &str, change_ref: &str) {
        let now = self.now;
        let schedule =
            self.schedules
                .entry(patient.to_owned())
                .or_insert_with(|| PatientSchedule {
                    cadence: ReviewCadence::Weekly,
                    last_weekly_at: now,
                    last_daily_check_at: now,
                    daily_changes: Vec::new(),
                });
        if !schedule
            .daily_changes
            .iter()
            .any(|existing| existing == change_ref)
        {
            schedule.daily_changes.push(change_ref.to_owned());
        }
    }

    /// All reviews started by the single scheduling owner in chronological order.
    pub fn scheduled_reviews(&self) -> &[ScheduledReview] {
        &self.scheduled
    }

    /// The next scheduled cadence check timestamp for an enrolled patient.
    pub fn next_scheduled_at(&self, patient: &str) -> Option<u64> {
        let schedule = self.schedules.get(patient)?;
        let next_weekly = schedule.last_weekly_at.saturating_add(WEEK_MS);
        match schedule.cadence {
            ReviewCadence::Weekly => Some(next_weekly),
            ReviewCadence::WeeklyAndDailyWhenChanged => {
                let next_daily = schedule.last_daily_check_at.saturating_add(DAY_MS);
                Some(next_daily.min(next_weekly))
            }
        }
    }

    /// Moves the clock forward. A time already past is left where it is.
    pub fn advance_to(&mut self, now: u64) {
        self.now = self.now.max(now);
    }

    /// Each arming read lands this many milliseconds later, before it looks at the window.
    pub fn delay_arming_reads(&mut self, delay_ms: u64) {
        self.arming_delay = delay_ms;
    }

    /// While deaf, an admission is committed and not heard. A later catch-up can stand for it.
    pub fn set_deaf(&mut self, deaf: bool) {
        self.deaf = deaf;
    }

    /// While refusing, a due window is tried and its refusal is recorded.
    pub fn set_refusing(&mut self, refusing: bool) {
        self.refusing = refusing;
    }

    /// Admits one source change at the current time and, unless deaf, notices it.
    pub fn admit(&mut self, patient: &str, source_ref: &str) {
        self.patients
            .entry(patient.to_owned())
            .or_insert_with(|| Patient {
                pending: Vec::new(),
            })
            .pending
            .push(Pending {
                source_ref: source_ref.to_owned(),
                admitted_at: self.now,
            });
        let position = self.push_fact(patient, FactKind::Admission);
        if !self.deaf {
            self.notices.push(Notice {
                patient: patient.to_owned(),
                range: Some((position, position)),
            });
        }
    }

    /// A notice that names no facts, as a reconnect's catch-up does.
    pub fn catch_up(&mut self, patient: &str) {
        self.notices.push(Notice {
            patient: patient.to_owned(),
            range: None,
        });
    }

    /// The review that took this patient's slot has ended. That release offers the patient once.
    pub fn finish_review(&mut self, patient: &str) {
        self.busy.remove(patient);
        self.catch_up(patient);
    }

    /// The earliest future close currently armed, if one is.
    pub fn armed_at(&self) -> Option<u64> {
        self.armed_at
    }

    /// How many times a pass asked the window to close.
    pub fn close_attempts(&self) -> u32 {
        self.close_attempts
    }

    /// Deliveries that started a review, each one's source refs in order.
    pub fn accepted(&self) -> &[Vec<String>] {
        &self.accepted
    }

    /// Hears every notice waiting. Arming reads come first. A pass then delivers a window that read already found closed, followed by due cadence reviews.
    pub fn drain(&mut self) {
        let mut completed = false;
        for _ in 0..8 {
            if self.notices.is_empty() {
                completed = true;
                break;
            }
            let notices = coalesce(std::mem::take(&mut self.notices));
            for notice in &notices {
                self.arming_read(&notice.patient);
            }
            let due: Vec<String> = notices
                .into_iter()
                .filter(|notice| self.makes_work_due(notice))
                .map(|notice| notice.patient)
                .collect();
            for patient in due {
                self.close_one(&patient);
            }
        }
        if !completed {
            panic!("a refused window was offered again");
        }
        self.drain_cadence();
    }

    fn push_fact(&mut self, patient: &str, kind: FactKind) -> u64 {
        let position = self.next_position;
        self.next_position += 1;
        self.facts.push(Fact {
            patient: patient.to_owned(),
            position,
            kind,
        });
        position
    }

    fn arming_read(&mut self, patient: &str) {
        self.now = self.now.saturating_add(self.arming_delay);
        let Some(closes) = self.closes_at(patient) else {
            return;
        };
        if closes <= self.now {
            return;
        }
        if self.armed_at.is_none_or(|at| closes < at) {
            self.armed_at = Some(closes);
        }
    }

    fn makes_work_due(&self, notice: &Notice) -> bool {
        let Some((first, through)) = notice.range else {
            return true;
        };
        self.facts.iter().any(|fact| {
            fact.patient == notice.patient
                && fact.kind == FactKind::Admission
                && fact.position >= first
                && fact.position <= through
        })
    }

    fn closes_at(&self, patient: &str) -> Option<u64> {
        let pending = &self.patients.get(patient)?.pending;
        pending
            .iter()
            .map(|change| change.admitted_at.saturating_add(self.window_ms))
            .min()
    }

    fn close_one(&mut self, patient: &str) {
        self.close_attempts += 1;
        let Some(closes) = self.closes_at(patient) else {
            return;
        };
        if self.now < closes || self.busy.contains(patient) {
            return;
        }
        if self.refusing {
            let position = self.push_fact(patient, FactKind::Refusal);
            self.notices.push(Notice {
                patient: patient.to_owned(),
                range: Some((position, position)),
            });
            return;
        }
        let mut refs: Vec<String> = self
            .patients
            .get(patient)
            .map(|state| {
                state
                    .pending
                    .iter()
                    .map(|change| change.source_ref.clone())
                    .collect()
            })
            .unwrap_or_default();
        refs.sort();
        if let Some(state) = self.patients.get_mut(patient) {
            state.pending.clear();
        }
        self.busy.insert(patient.to_owned());
        self.accepted.push(refs.clone());
        self.scheduled.push(ScheduledReview {
            patient: patient.to_owned(),
            kind: ScheduledReviewKind::SourceChanges(refs),
            started_at: self.now,
        });
    }

    fn drain_cadence(&mut self) {
        if self.refusing {
            return;
        }
        let now = self.now;
        let patients: Vec<String> = self.schedules.keys().cloned().collect();
        for patient in patients {
            if self.busy.contains(&patient) {
                continue;
            }
            let Some(schedule) = self.schedules.get_mut(&patient) else {
                continue;
            };
            let next_weekly = schedule.last_weekly_at.saturating_add(WEEK_MS);
            if now >= next_weekly {
                let folded_changes = std::mem::take(&mut schedule.daily_changes);
                schedule.last_weekly_at = now;
                schedule.last_daily_check_at = now;
                self.busy.insert(patient.clone());
                self.scheduled.push(ScheduledReview {
                    patient,
                    kind: ScheduledReviewKind::WeeklyWholePatient { folded_changes },
                    started_at: now,
                });
                continue;
            }
            if schedule.cadence == ReviewCadence::WeeklyAndDailyWhenChanged {
                let next_daily = schedule.last_daily_check_at.saturating_add(DAY_MS);
                if now >= next_daily {
                    schedule.last_daily_check_at = now;
                    if !schedule.daily_changes.is_empty() {
                        let changes = std::mem::take(&mut schedule.daily_changes);
                        self.busy.insert(patient.clone());
                        self.scheduled.push(ScheduledReview {
                            patient,
                            kind: ScheduledReviewKind::DailyWhenChanged { changes },
                            started_at: now,
                        });
                    }
                }
            }
        }
    }
}

fn coalesce(notices: Vec<Notice>) -> Vec<Notice> {
    let mut order = Vec::new();
    let mut grouped: BTreeMap<String, Notice> = BTreeMap::new();
    for notice in notices {
        if let Some(existing) = grouped.get_mut(&notice.patient) {
            existing.range = match (existing.range, notice.range) {
                (Some((first, through)), Some((next_first, next_through))) => {
                    Some((first.min(next_first), through.max(next_through)))
                }
                _ => None,
            };
        } else {
            order.push(notice.patient.clone());
            grouped.insert(notice.patient.clone(), notice);
        }
    }
    order
        .into_iter()
        .filter_map(|patient| grouped.remove(&patient))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATIENT: &str = "maya";
    const WINDOW_MS: u64 = 200;

    fn closed_before_the_read() -> Dispatch {
        let mut dispatch = Dispatch::new(WINDOW_MS);
        dispatch.delay_arming_reads(300);
        dispatch
    }

    #[test]
    fn an_open_window_is_armed_and_not_delivered() {
        let mut dispatch = Dispatch::new(WINDOW_MS);
        dispatch.admit(PATIENT, "result");
        dispatch.drain();
        assert_eq!(dispatch.armed_at(), Some(WINDOW_MS));
        assert!(dispatch.accepted().is_empty());
    }

    #[test]
    fn a_window_closed_before_the_arming_read_is_delivered_once() {
        let mut dispatch = closed_before_the_read();
        dispatch.admit(PATIENT, "result");
        dispatch.drain();
        assert_eq!(dispatch.accepted(), &[vec!["result".to_owned()]]);
        dispatch.finish_review(PATIENT);
        dispatch.drain();
        assert_eq!(dispatch.accepted(), &[vec!["result".to_owned()]]);
    }

    #[test]
    fn a_window_found_closed_behind_a_running_review_is_tried_once_and_delivered_when_it_ends() {
        let mut dispatch = closed_before_the_read();
        dispatch.admit(PATIENT, "result");
        dispatch.drain();
        dispatch.admit(PATIENT, "visit");
        dispatch.drain();
        let attempts = dispatch.close_attempts();
        dispatch.drain();
        assert_eq!(dispatch.close_attempts(), attempts);
        assert_eq!(dispatch.accepted(), &[vec!["result".to_owned()]]);
        dispatch.finish_review(PATIENT);
        dispatch.drain();
        assert_eq!(
            dispatch.accepted(),
            &[vec!["result".to_owned()], vec!["visit".to_owned()]]
        );
    }

    #[test]
    fn a_refused_window_is_tried_once_and_delivered_with_the_next_change() {
        let mut dispatch = closed_before_the_read();
        dispatch.set_refusing(true);
        dispatch.admit(PATIENT, "result");
        dispatch.drain();
        assert_eq!(dispatch.close_attempts(), 1);
        assert!(dispatch.accepted().is_empty());
        dispatch.drain();
        assert_eq!(dispatch.close_attempts(), 1);
        dispatch.set_refusing(false);
        dispatch.admit(PATIENT, "visit");
        dispatch.drain();
        assert_eq!(
            dispatch.accepted(),
            &[vec!["result".to_owned(), "visit".to_owned()]]
        );
    }

    #[test]
    fn a_notice_with_no_facts_delivers_a_window_the_dispatcher_never_heard() {
        let mut dispatch = Dispatch::new(WINDOW_MS);
        dispatch.set_deaf(true);
        dispatch.admit(PATIENT, "result");
        dispatch.advance_to(500);
        dispatch.drain();
        assert!(dispatch.accepted().is_empty());
        dispatch.set_deaf(false);
        dispatch.catch_up(PATIENT);
        dispatch.drain();
        assert_eq!(dispatch.accepted(), &[vec!["result".to_owned()]]);
    }

    #[test]
    fn weekly_cadence_runs_settled_weekly_review_and_ignores_daily_ticks() {
        let mut dispatch = Dispatch::new(WINDOW_MS);
        dispatch.enroll_patient(PATIENT, ReviewCadence::Weekly);
        assert_eq!(dispatch.cadence(PATIENT), ReviewCadence::Weekly);
        assert_eq!(dispatch.next_scheduled_at(PATIENT), Some(WEEK_MS));

        dispatch.record_daily_change(PATIENT, "vitals-bp");
        dispatch.advance_to(DAY_MS);
        dispatch.drain();
        assert!(
            dispatch.scheduled_reviews().is_empty(),
            "weekly cadence must not run a daily review"
        );

        dispatch.advance_to(WEEK_MS);
        dispatch.drain();
        assert_eq!(
            dispatch.scheduled_reviews(),
            &[ScheduledReview {
                patient: PATIENT.to_owned(),
                kind: ScheduledReviewKind::WeeklyWholePatient {
                    folded_changes: vec!["vitals-bp".to_owned()],
                },
                started_at: WEEK_MS,
            }]
        );
    }

    #[test]
    fn weekly_and_daily_when_changed_runs_daily_only_when_changes_exist_and_folds_into_weekly() {
        let mut dispatch = Dispatch::new(WINDOW_MS);
        dispatch.enroll_patient(PATIENT, ReviewCadence::WeeklyAndDailyWhenChanged);
        assert_eq!(
            dispatch.cadence(PATIENT),
            ReviewCadence::WeeklyAndDailyWhenChanged
        );
        assert_eq!(dispatch.next_scheduled_at(PATIENT), Some(DAY_MS));

        // Day 1 with no changes: quiet check, no review started.
        dispatch.advance_to(DAY_MS);
        dispatch.drain();
        assert!(dispatch.scheduled_reviews().is_empty());
        assert_eq!(dispatch.next_scheduled_at(PATIENT), Some(2 * DAY_MS));

        // Day 2 with changes: starts DailyWhenChanged review.
        dispatch.record_daily_change(PATIENT, "duet-home-bp");
        dispatch.record_daily_change(PATIENT, "duet-home-bp");
        dispatch.record_daily_change(PATIENT, "lab-bmp");
        dispatch.advance_to(2 * DAY_MS);
        dispatch.drain();
        assert_eq!(
            dispatch.scheduled_reviews(),
            &[ScheduledReview {
                patient: PATIENT.to_owned(),
                kind: ScheduledReviewKind::DailyWhenChanged {
                    changes: vec!["duet-home-bp".to_owned(), "lab-bmp".to_owned()],
                },
                started_at: 2 * DAY_MS,
            }]
        );
        dispatch.finish_review(PATIENT);

        // Day 7: settled weekly whole-patient review runs once (folding any new changes).
        dispatch.record_daily_change(PATIENT, "med-reconciliation");
        dispatch.advance_to(WEEK_MS);
        dispatch.drain();
        assert_eq!(dispatch.scheduled_reviews().len(), 2);
        assert_eq!(
            dispatch.scheduled_reviews()[1],
            ScheduledReview {
                patient: PATIENT.to_owned(),
                kind: ScheduledReviewKind::WeeklyWholePatient {
                    folded_changes: vec!["med-reconciliation".to_owned()],
                },
                started_at: WEEK_MS,
            }
        );
    }

    #[test]
    fn due_cadence_review_waits_for_active_review_slot_to_finish() {
        let mut dispatch = closed_before_the_read();
        dispatch.enroll_patient(PATIENT, ReviewCadence::WeeklyAndDailyWhenChanged);
        dispatch.admit(PATIENT, "lab-1");
        dispatch.drain();
        assert_eq!(dispatch.scheduled_reviews().len(), 1);

        // While source-change review is still running, day 1 arrives with a change.
        dispatch.record_daily_change(PATIENT, "symptom-log");
        dispatch.advance_to(DAY_MS);
        dispatch.drain();
        assert_eq!(
            dispatch.scheduled_reviews().len(),
            1,
            "daily review must wait while patient slot is busy"
        );

        dispatch.finish_review(PATIENT);
        dispatch.drain();
        assert_eq!(dispatch.scheduled_reviews().len(), 2);
        assert_eq!(
            dispatch.scheduled_reviews()[1],
            ScheduledReview {
                patient: PATIENT.to_owned(),
                kind: ScheduledReviewKind::DailyWhenChanged {
                    changes: vec!["symptom-log".to_owned()],
                },
                started_at: DAY_MS + 300,
            }
        );
    }
}
