//! One patient's chronology: when every record happened, and what was true at any
//! moment.
//!
//! This is the only owner of time in the bank. Each record has its own instant
//! (a medication's order date, a vital set's time, a result's release), and a
//! chart as of a moment holds exactly the records whose instant is before it.
//! Medication, problem and allergy status come from each concept's whole history,
//! so a restarted drug can stop again, a resolved problem can recur, and "no known
//! drug allergies" ends when a drug allergy is recorded.

use std::collections::HashMap;

use jiff::{Span, ToSpan};
use sha2::{Digest, Sha256};

use crate::catalog::Catalog;
use crate::ledger::{
    AllergyItem, ConditionAction, ConditionItem, Course, Event, Lab, MedicationAction,
    MedicationItem, OrderAction, OrderItem, Patient, Report, Stamp, VitalSet,
};
use crate::rng::Rng;
use crate::time;

/// The leading `hex` hex digits of SHA-256(`text`) as a number.
pub fn hash_prefix(text: &str, hex: usize) -> u64 {
    let digest = Sha256::digest(text.as_bytes());
    let full: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    u64::from_str_radix(&full[..hex], 16).expect("hex digits")
}

/// Cultures take about two days; other results come back the same day.
pub fn slow_lab(lab: &Lab) -> bool {
    lab.results
        .iter()
        .any(|r| r.test == "Urine Culture" || r.test == "Blood Culture")
}

/// Pathology and cytology take days; other studies are read the same day.
pub fn slow_report(report: &Report) -> bool {
    let study = report.study.to_lowercase();
    study.contains("pathology") || study.contains("cytology")
}

/// A medication order: a start or a reported historical medication.
pub struct Order<'a> {
    pub event: &'a Event,
    pub index: usize,
    pub item: &'a MedicationItem,
    pub authored: Stamp,
    /// The stop that ended this order, if one was ever documented.
    pub stopped: Option<Stamp>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderStatus {
    Active,
    Stopped(Stamp),
    /// A short course more than three weeks old.
    Completed,
}

impl Order<'_> {
    pub fn status(&self, at: Stamp) -> OrderStatus {
        match self.stopped {
            Some(stop) if stop < at => OrderStatus::Stopped(stop),
            _ if self.item.course == Some(Course::Short)
                && time::minutes_between(self.authored, at) > 21 * 24 * 60 =>
            {
                OrderStatus::Completed
            }
            _ => OrderStatus::Active,
        }
    }
}

/// One episode of a problem-list concept and every action on it.
pub struct Problem<'a> {
    pub label: &'a str,
    /// The event and item that first put it on the problem list.
    pub event: &'a Event,
    pub item: &'a ConditionItem,
    pub recorded: Stamp,
    actions: Vec<(Stamp, ConditionAction)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProblemStatus {
    Active,
    Resolved(Stamp),
}

impl Problem<'_> {
    pub fn status(&self, at: Stamp) -> ProblemStatus {
        self.actions
            .iter()
            .take_while(|(when, _)| *when < at)
            .find(|(_, action)| *action == ConditionAction::Resolved)
            .map_or(ProblemStatus::Active, |&(when, _)| {
                ProblemStatus::Resolved(when)
            })
    }
}

/// One allergy-list entry. An allergen stays on the list once recorded. A "no
/// known" entry says the patient has none of one category, so it stays true only
/// until an allergy of that category is recorded.
pub struct Allergy<'a> {
    pub event: &'a Event,
    pub index: usize,
    pub item: &'a AllergyItem,
    pub recorded: Stamp,
    /// The category a "no known" entry rules out.
    pub negates: Option<&'a str>,
    /// When an allergy this entry ruled out was recorded.
    pub refuted: Option<Stamp>,
}

impl Allergy<'_> {
    pub fn active(&self, at: Stamp) -> bool {
        self.refuted.is_none_or(|when| when >= at)
    }
}

/// A referral or test order and what became of it.
pub struct ServiceOrder<'a> {
    pub event: &'a Event,
    pub item: &'a OrderItem,
    pub placed: Stamp,
    /// When it was done or cancelled, if it ever was.
    pub closed: Option<(Stamp, OrderAction)>,
}

impl ServiceOrder<'_> {
    pub fn status(&self, at: Stamp) -> OrderAction {
        match self.closed {
            Some((when, action)) if when < at => action,
            _ => OrderAction::Placed,
        }
    }
}

pub struct Timeline<'a> {
    pub patient: &'a Patient,
    /// Events by start time (ties keep authored order).
    pub events: Vec<&'a Event>,
    pub orders: Vec<Order<'a>>,
    pub problems: Vec<Problem<'a>>,
    pub service_orders: Vec<ServiceOrder<'a>>,
    /// The per-patient number behind every seeded draw.
    pub base: u64,
}

impl<'a> Timeline<'a> {
    pub fn new(patient: &'a Patient) -> Timeline<'a> {
        let mut events: Vec<&Event> = patient.events.iter().collect();
        events.sort_by_key(|e| e.start);

        // Medication actions in time order; a stop ends every open order of its drug.
        let mut actions: Vec<(Stamp, usize, usize, &Event, &MedicationItem)> = Vec::new();
        for (pos, ev) in events.iter().enumerate() {
            for (i, m) in ev.medications.iter().enumerate() {
                actions.push((m.date.unwrap_or(ev.start), pos, i, ev, m));
            }
        }
        actions.sort_by_key(|a| (a.0, a.1, a.2));
        let mut orders: Vec<(usize, Order)> = Vec::new();
        let mut open: HashMap<&str, Vec<usize>> = HashMap::new();
        for (when, pos, i, ev, m) in actions {
            let Some(key) = m.key() else { continue };
            match m.action {
                MedicationAction::Start | MedicationAction::Historical => {
                    open.entry(key).or_default().push(orders.len());
                    let order = Order {
                        event: ev,
                        index: i,
                        item: m,
                        authored: when,
                        stopped: None,
                    };
                    orders.push((pos, order));
                }
                MedicationAction::Stop => {
                    for o in open.remove(key).unwrap_or_default() {
                        orders[o].1.stopped = Some(when);
                    }
                }
            }
        }
        // Orders in chart order: by event, then as listed.
        orders.sort_by_key(|(pos, o)| (*pos, o.index));
        let orders = orders.into_iter().map(|(_, o)| o).collect();

        // A diagnosis after the concept resolved opens a new episode with its own onset.
        let mut problems: Vec<Problem> = Vec::new();
        for ev in &events {
            for item in &ev.conditions {
                let open = problems
                    .iter()
                    .rposition(|p| p.label == item.label)
                    .filter(|&i| {
                        item.action == ConditionAction::Resolved
                            || !matches!(
                                problems[i].actions.last(),
                                Some((_, ConditionAction::Resolved))
                            )
                    });
                match (open, item.action) {
                    (Some(i), action) => problems[i].actions.push((ev.start, action)),
                    (None, ConditionAction::Diagnosed | ConditionAction::History) => {
                        problems.push(Problem {
                            label: &item.label,
                            event: ev,
                            item,
                            recorded: ev.start,
                            actions: vec![(ev.start, item.action)],
                        })
                    }
                    // resolved before it was ever recorded: nothing to resolve
                    (None, _) => {}
                }
            }
        }

        let mut service_orders: Vec<ServiceOrder> = Vec::new();
        for ev in &events {
            for item in &ev.orders {
                let when = item.date.unwrap_or(ev.start);
                match item.action {
                    OrderAction::Placed => service_orders.push(ServiceOrder {
                        event: ev,
                        item,
                        placed: when,
                        closed: None,
                    }),
                    action => {
                        if let Some(o) = service_orders
                            .iter_mut()
                            .rev()
                            .find(|o| o.item.id == item.id && o.closed.is_none())
                        {
                            o.closed = Some((when, action));
                        }
                    }
                }
            }
        }

        Timeline {
            patient,
            events,
            orders,
            problems,
            service_orders,
            base: hash_prefix(&patient.mrn, 8),
        }
    }

    /// The allergy list's entries in the order recorded, each concept once.
    pub fn allergies(&self, catalog: &'a Catalog) -> Vec<Allergy<'a>> {
        let mut list: Vec<Allergy> = Vec::new();
        for ev in &self.events {
            for (index, item) in ev.allergies.iter().enumerate() {
                let (category, negates) = catalog.allergy_class(item);
                let again = list
                    .iter()
                    .any(|a| a.item.substance == item.substance && a.refuted.is_none());
                if again {
                    continue;
                }
                if negates.is_none() {
                    for a in list.iter_mut().filter(|a| a.refuted.is_none()) {
                        if a.negates == Some(category) {
                            a.refuted = Some(ev.start);
                        }
                    }
                }
                list.push(Allergy {
                    event: ev,
                    index,
                    item,
                    recorded: ev.start,
                    negates,
                    refuted: None,
                });
            }
        }
        list
    }

    /// When an untimed rooming set was taken: a few minutes after the visit started.
    pub fn rooming(&self, ev: &Event) -> Stamp {
        time::plus(ev.start, (4 + (self.base % 9) as i64).minutes())
    }

    pub fn vitals_time(&self, ev: &Event, set: &VitalSet) -> Stamp {
        set.time.unwrap_or_else(|| self.rooming(ev))
    }

    pub fn collected(ev: &Event, lab: &Lab) -> Stamp {
        lab.collected.unwrap_or(ev.start)
    }

    /// The seeded draws for lab `i` of an event: turnaround first, then filled values.
    pub fn lab_draws(&self, ev: &Event, i: usize) -> Rng {
        Rng::new(u128::from(
            self.base + hash_prefix(&format!("{}{i}", ev.id), 6),
        ))
    }

    /// The usual turnaround for a draw, taken from its seeded draws.
    pub fn lab_turnaround(rng: &mut Rng, lab: &Lab) -> Span {
        if slow_lab(lab) {
            (46 + rng.randint(0, 20)).hours()
        } else {
            (95 + rng.randint(0, 300)).minutes()
        }
    }

    /// When a draw's results were released, unless a result says otherwise.
    pub fn lab_released(&self, ev: &Event, i: usize, lab: &Lab) -> Stamp {
        let usual = time::plus(
            Self::collected(ev, lab),
            Self::lab_turnaround(&mut self.lab_draws(ev, i), lab),
        );
        if lab.pending {
            return time::never();
        }
        lab.resulted.unwrap_or(usual)
    }

    pub fn performed(ev: &Event, report: &Report) -> Stamp {
        report.performed.unwrap_or(ev.start)
    }

    /// When a study was released by its reader.
    pub fn report_issued(&self, ev: &Event, i: usize, report: &Report) -> Stamp {
        if let Some(issued) = report.issued {
            return issued;
        }
        let mut rng = Rng::new(u128::from(hash_prefix(
            &format!("{}{}{i}", self.patient.mrn, ev.id),
            8,
        )));
        let delay = if slow_report(report) {
            let days = 3 + rng.randint(0, 5);
            days.days().hours(rng.randint(0, 8))
        } else {
            (50 + rng.randint(0, 600)).minutes()
        };
        time::plus(Self::performed(ev, report), delay)
    }

    /// The first moment the practice had this patient: the earliest event start.
    pub fn first(&self) -> Option<Stamp> {
        self.events.first().map(|e| e.start)
    }
}
