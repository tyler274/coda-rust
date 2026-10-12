//! The patient-work dispatcher's grouping window.
//!
//! The coda-agents checkout is the reference (`patientWorkDispatcher.ts`,
//! `sourceChangeGrouping.ts`) and is not modified. An arming read reports
//! only a future close. A window already closed when that read finishes is
//! delivered by the pass after it, once. A refusal's record is not a fact
//! that makes work due, so it does not offer the window again.
#![deny(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

/// One process's grouping-window delivery.
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
    busy: BTreeSet<String>,
    accepted: Vec<Vec<String>>,
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
            busy: BTreeSet::new(),
            accepted: Vec::new(),
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

    /// Hears every notice waiting. Arming reads come first. A pass then delivers a window that read already found closed.
    pub fn drain(&mut self) {
        for _ in 0..8 {
            if self.notices.is_empty() {
                return;
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
        panic!("a refused window was offered again");
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
        self.accepted.push(refs);
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
}
