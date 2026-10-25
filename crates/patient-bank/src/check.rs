//! The authoring rules a unit's ledger must meet before it is accepted.
//!
//! Shape is the ledger types' job; these are the rules about content: seed
//! agreement, the catalog vocabulary, chronology (everything before today
//! except booked visits), pending results, orders and missed visits, lab arithmetic, vitals a device would produce, and note hygiene
//! (authors, roles not names, no em dashes).
//! Errors send the author back to the part files; warnings are kept with the
//! accepted ledger for review. None of this judges clinical reasoning.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use jiff::ToSpan;
use regex::Regex;

use crate::catalog::Catalog;
use crate::labs::DIFFERENTIAL;
use crate::ledger::{
    Appointment, ConditionAction, Event, MedicationAction, OrderAction, Patient, Stamp, VitalSet,
    num,
};
use crate::seeds::Unit;
use crate::time;
use crate::timeline::{ProblemStatus, Timeline};

static FORBIDDEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(synthetic|fictional|generated|template|simulation|simulated|test patient|answer key|placeholder|lorem)\b")
        .expect("regex")
});

/// A clinician or staff member named in a note: "Dr. Okafor", "Dr. C", "M. Lindqvist, MD",
/// "Colleen Fitzgerald, NP", "J. Ortiz, MA", or a signature line "-- B. Kowal".
/// A surname has a lowercase second letter, so "Per GI MD" is not a name. MA and PA
/// count only after an initial, so "Springfield, MA" and "State College, PA" stay
/// places.
static CLINICIAN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:Dr\.?|Doctor) (?:[A-Z][a-z][a-zA-Z'-]*|[A-Z]\b)",
        r"|\b[A-Z]\. [A-Z][a-z][a-zA-Z'-]*,? (?:MD|DO|NP|PA-C|PA|RN|APRN|FNP|DNP|LPN|LICSW|LCSW|PharmD|CNM|MA)\b",
        r"|\b[A-Z][a-z]+ [A-Z][a-z][a-zA-Z'-]*,? (?:MD|DO|NP|PA-C|RN|APRN|FNP|DNP|LPN|LICSW|LCSW|PharmD|CNM)\b",
        r"|(?m)(?:--|—) ?[A-Z]\. [A-Z][a-z][a-zA-Z'-]*(?:, [a-z][a-z ]+)?[ \t]*$",
    ))
    .expect("regex")
});

/// Phrasings no chart should carry: a role used as a name in a greeting or a
/// salutation ("Hi the PCP,", "The RN, I wanted to…"), and the em dash, which
/// clinicians do not type.
static PHRASING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(?:hi|hello|hey|dear) the (?:pcp|rn|ma|np|pa|doctor|physician|nurse)\b",
        r"|(?m-i)^The (?:PCP|RN|MA|NP|PA),",
        r"|—",
    ))
    .expect("regex")
});

/// With at least this many readings, a patient's vitals are judged as a series.
const SERIES: usize = 8;

/// Unseeded problems a chart may still carry as active at the bank time: real
/// lists pick up a stray entry or two beyond what the story planned.
const UNSEEDED_ACTIVE: usize = 2;

/// Real care organizations and record systems a chart must not name (the guide
/// asks for roles and generic places). Drug, vaccine and assay product names stay.
static REAL_NAMES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(MyChart|Epic|Cerner|athenahealth|",
        r"Quest Diagnostics|LabCorp|CVS|Walgreens|Walmart|Rite Aid|Stop & Shop|Big Y|",
        r"Baystate|Cooley Dickinson|Mercy Medical|Holyoke Medical|UMass|Mass General|Brigham|",
        r"Dartmouth|Hartford Hospital|Yale New Haven|Valley Medical Group|Kaiser|Mayo Clinic)\b",
    ))
    .expect("regex")
});

pub static AGE_IN_TEXT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(\d{2,3})[- ](?:year|yr)s?[- ]old\b").expect("regex"));

/// Words near a stated age that make it someone else's age.
pub static KIN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\b(mother|father|sister|brother|son|daughter|wife|husband|spouse|partner|grand\w*|aunt|uncle|cousin|friend|at age|in her|in his|niece|nephew)\b",
    )
    .expect("regex")
});

/// The text around a match: `before` characters before it and `after` after it.
pub fn around(text: &str, start: usize, end: usize, before: usize, after: usize) -> &str {
    let from = text[..start]
        .char_indices()
        .rev()
        .nth(before.saturating_sub(1))
        .map_or(0, |(i, _)| i);
    let to = text[end..]
        .char_indices()
        .nth(after)
        .map_or(text.len(), |(i, _)| end + i);
    &text[from..to]
}

/// The ages a note body states for the patient that disagree with `actual`.
pub fn wrong_ages(body: &str, actual: i64) -> Vec<i64> {
    AGE_IN_TEXT
        .captures_iter(body)
        .filter_map(|m| {
            let whole = m.get(0).expect("match");
            let stated: i64 = m[1].parse().ok()?;
            let ctx = around(body, whole.start(), whole.end(), 40, 10);
            (stated != actual && stated < 120 && !KIN.is_match(ctx)).then_some(stated)
        })
        .collect()
}

fn fmt_num(x: f64) -> String {
    let s = format!("{x:.4}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// Check a unit's merged charts. Returns (errors, warnings).
pub fn check(unit: &Unit, patients: &[Patient], catalog: &Catalog) -> (Vec<String>, Vec<String>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let as_of = time::as_of();
    let wanted: Vec<&str> = unit.members.iter().map(|m| m.mrn.as_str()).collect();
    let mut got: Vec<&str> = patients.iter().map(|p| p.mrn.as_str()).collect();
    got.sort_unstable();
    let mut want_sorted = wanted.clone();
    want_sorted.sort_unstable();
    if got != want_sorted {
        errors.push(format!(
            "patients must be exactly {want_sorted:?}; got {got:?}"
        ));
    }
    let mut bodies: HashMap<&str, String> = HashMap::new();
    for p in patients {
        let Some(seed) = unit.members.iter().find(|m| m.mrn == p.mrn) else {
            continue;
        };
        let mrn = &seed.mrn;
        for (field, have, want) in [
            ("given", p.given.as_str(), seed.given.as_str()),
            ("family", p.family.as_str(), seed.family.as_str()),
        ] {
            if have != want {
                errors.push(format!(
                    "{mrn}: {field} must be {want:?} (seed), got {have:?}"
                ));
            }
        }
        if p.birth_date != seed.birth_date {
            errors.push(format!(
                "{mrn}: birth_date must be {} (seed), got {}",
                seed.birth_date.0, p.birth_date.0
            ));
        }
        if p.events.is_empty() {
            errors.push(format!("{mrn}: no events"));
            continue;
        }
        let mut ids = HashSet::new();
        if !p.events.iter().all(|e| ids.insert(e.id.as_str())) {
            errors.push(format!("{mrn}: duplicate event ids"));
        }
        let birth = seed.birth_date.0;
        let tl = Timeline::new(p);
        let mut recorded: HashSet<&str> = HashSet::new();
        let mut started: HashSet<&str> = HashSet::new();
        for ev in &tl.events {
            let eid = format!("{mrn}/{}", ev.id);
            let booked = ev.appointment == Some(Appointment::Booked);
            for (what, stamp) in stamps(ev) {
                // an admission still under way ends after today
                if stamp >= as_of && what != "end" && !booked {
                    errors.push(format!(
                        "{eid}: {what} {stamp} is not before today {}; nothing after today is \
                         written except a booked visit",
                        time::AS_OF
                    ));
                }
                if stamp.date() < birth {
                    errors.push(format!("{eid}: {what} {stamp} precedes birth {birth}"));
                }
            }
            check_event(
                ev,
                &eid,
                catalog,
                &mut recorded,
                &mut started,
                &mut errors,
                &mut warnings,
            );
            for d in &ev.documents {
                let body = d.body.trim();
                let title = d.title();
                if body.chars().count() < 40 {
                    errors.push(format!("{eid}: document {title:?} body too short"));
                }
                match d.author.as_deref().map(str::trim) {
                    None | Some("") => errors.push(format!(
                        "{eid}: document {title:?} has no author; name the writer's role"
                    )),
                    Some(a) if CLINICIAN.is_match(a) => errors.push(format!(
                        "{eid}: document {title:?} author {a:?} names a person; use the role"
                    )),
                    Some(_) => {}
                }
                if let Some(m) = PHRASING
                    .find(body)
                    .or_else(|| d.title.as_deref().and_then(|t| PHRASING.find(t)))
                {
                    errors.push(format!(
                        "{eid}: document {title:?} uses {:?}; write the way the author would",
                        m.as_str()
                    ));
                }
                if let Some(m) = REAL_NAMES.find(body) {
                    errors.push(format!(
                        "{eid}: document {title:?} names a real organization or product ({}); use a role or generic description",
                        m.as_str()
                    ));
                }
                if let Some(m) = CLINICIAN
                    .find(body)
                    .or_else(|| d.title.as_deref().and_then(|t| CLINICIAN.find(t)))
                {
                    errors.push(format!(
                        "{eid}: document {title:?} names a clinician ({}); refer to clinicians and staff by role",
                        m.as_str()
                    ));
                }
                if let Some(m) = FORBIDDEN.find(body) {
                    errors.push(format!(
                        "{eid}: document {title:?} uses a forbidden word ({})",
                        m.as_str()
                    ));
                }
                match bodies.get(body) {
                    Some(first) => errors.push(format!("{eid}: document body duplicates {first}")),
                    None => {
                        bodies.insert(body, eid.clone());
                    }
                }
                let on = d.date.unwrap_or(ev.start).date();
                let actual = time::age_on(birth, on);
                for stated in wrong_ages(body, actual) {
                    errors.push(format!(
                        "{eid}: note says {stated}-year-old but patient is {actual} on {on}"
                    ));
                }
            }
            if ev.appointment.is_none() && ev.kind.needs_note() && ev.documents.is_empty() {
                errors.push(format!("{eid}: visit without a note"));
            }
            if ev.appointment.is_some() {
                if !ev.booked() {
                    errors.push(format!(
                        "{eid}: only a visit on the practice calendar can be booked, cancelled \
                         or missed"
                    ));
                }
                let written = !ev.documents.is_empty()
                    || !ev.conditions.is_empty()
                    || !ev.medications.is_empty()
                    || !ev.allergies.is_empty()
                    || !ev.social.is_empty()
                    || !ev.orders.is_empty();
                if booked && ev.start < as_of {
                    errors.push(format!(
                        "{eid}: a visit booked before today either happened, or was cancelled \
                         or missed"
                    ));
                }
                if booked && written {
                    errors.push(format!("{eid}: a booked visit holds nothing yet"));
                }
                let happened = !ev.vitals.is_empty()
                    || !ev.labs.is_empty()
                    || !ev.reports.is_empty()
                    || !ev.procedures.is_empty()
                    || !ev.immunizations.is_empty()
                    || !ev.surveys.is_empty();
                if happened {
                    errors.push(format!(
                        "{eid}: a cancelled or missed visit records nothing clinical"
                    ));
                }
            }
        }
        // What comes back after today is decided later: results drawn before today
        // and not back are `pending`, with no values.
        for ev in &tl.events {
            let eid = format!("{mrn}/{}", ev.id);
            for (i, lab) in ev.labs.iter().enumerate() {
                let collected = Timeline::collected(ev, lab);
                if lab.pending && collected < time::plus(as_of, (-14).days()) {
                    errors.push(format!(
                        "{eid}: a draw from {collected} would be back by today; give its results"
                    ));
                }
                let late = lab.results.iter().filter_map(|r| r.resulted);
                if !lab.pending
                    && (tl.lab_released(ev, i, lab) >= as_of
                        || late.into_iter().any(|t| t >= as_of))
                {
                    errors.push(format!(
                        "{eid}: a draw on {collected} is not back by today; mark it \
                         `\"pending\": true` and leave its values out"
                    ));
                }
            }
            for (i, r) in ev.reports.iter().enumerate() {
                if ev.start < as_of && tl.report_issued(ev, i, r) >= as_of {
                    errors.push(format!(
                        "{eid}: {:?} would be read after today; date it earlier, or leave the \
                         study out and keep an open order for it",
                        r.study
                    ));
                }
            }
        }
        check_orders(mrn, &tl, &mut errors);
        check_vitals(mrn, &tl, &mut errors);
        check_weekdays(mrn, &tl, &mut errors);
        // A stop that finds no open order of its drug documents nothing.
        for ev in &tl.events {
            for m in ev
                .medications
                .iter()
                .filter(|m| m.action == MedicationAction::Stop)
            {
                let when = m.date.unwrap_or(ev.start);
                let key = m.key().unwrap_or("");
                let ended = tl
                    .orders
                    .iter()
                    .any(|o| o.item.key() == Some(key) && o.stopped == Some(when));
                if !ended && started.contains(key) {
                    warnings.push(format!(
                        "{mrn}/{}: stop of {key:?} finds no active order",
                        ev.id
                    ));
                }
            }
        }
        if let Some(first) = tl
            .events
            .iter()
            .filter(|e| e.kind != crate::ledger::Kind::OutsideRecords)
            .map(|e| e.start)
            .min()
        {
            let expected = time::as_of().date().year() as i64 - i64::from(seed.years_with_practice);
            if (i64::from(first.date().year()) - expected).abs() > 2 {
                warnings.push(format!(
                    "{mrn}: first practice event {} vs about {expected}",
                    first.date()
                ));
            }
        }
        if seed.coded_anchors {
            problem_list(mrn, &seed.anchors, &tl, catalog, &mut errors);
        }
    }
    (errors, warnings)
}

/// Each order is placed once with what it is, and closed only after it was placed.
fn check_orders(mrn: &str, tl: &Timeline, errors: &mut Vec<String>) {
    let mut placed: HashMap<&str, Stamp> = HashMap::new();
    for ev in &tl.events {
        for o in &ev.orders {
            let when = o.date.unwrap_or(ev.start);
            let eid = format!("{mrn}/{}", ev.id);
            match o.action {
                OrderAction::Placed => {
                    if o.order.as_deref().is_none_or(|t| t.trim().is_empty()) {
                        errors.push(format!("{eid}: order {:?} placed without `order`", o.id));
                    }
                    if o.category.is_none() {
                        errors.push(format!("{eid}: order {:?} placed without `category`", o.id));
                    }
                    if placed.insert(&o.id, when).is_some() {
                        errors.push(format!("{eid}: order id {:?} placed twice", o.id));
                    }
                }
                _ => match placed.get(o.id.as_str()) {
                    Some(at) if *at <= when => {}
                    _ => errors.push(format!(
                        "{eid}: order {:?} closed before it was placed",
                        o.id
                    )),
                },
            }
        }
    }
}

/// With at least this many dated contacts, their weekdays are judged as a spread.
const WEEKDAY_SERIES: usize = 15;

/// Contacts spread over the week the way a practice's do: no one weekday carries
/// more than about a third of the visits or of everything else, and the practice
/// does not book Sunday visits.
fn check_weekdays(mrn: &str, tl: &Timeline, errors: &mut Vec<String>) {
    let weekday = |s: &Stamp| s.date().weekday().to_sunday_zero_offset();
    const NAMES: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    let visits: Vec<i8> = tl
        .events
        .iter()
        .filter(|e| e.booked())
        .map(|e| weekday(&e.start))
        .collect();
    let all: Vec<i8> = tl.events.iter().map(|e| weekday(&e.start)).collect();
    for (what, days) in [("practice visits", &visits), ("events", &all)] {
        if days.len() < WEEKDAY_SERIES {
            continue;
        }
        let mut counts = [0usize; 7];
        for d in days.iter() {
            counts[*d as usize] += 1;
        }
        let (top, n) = counts
            .iter()
            .enumerate()
            .max_by_key(|(_, n)| **n)
            .expect("seven days");
        if *n as f64 > 0.35 * days.len() as f64 {
            errors.push(format!(
                "{mrn}: {n} of {} {what} fall on a {}; spread them over the week",
                days.len(),
                NAMES[top]
            ));
        }
    }
    for e in tl
        .events
        .iter()
        .filter(|e| e.booked() && weekday(&e.start) == 0)
    {
        errors.push(format!(
            "{mrn}/{}: the practice books no Sunday visits",
            e.id
        ));
    }
}

/// Vitals a real cuff, monitor and scale would produce: not every systolic even,
/// not nearly every heart rate even, few exact repeats of a BP pair, and
/// temperatures that vary.
fn check_vitals(mrn: &str, tl: &Timeline, errors: &mut Vec<String>) {
    let sets: Vec<&VitalSet> = tl.events.iter().flat_map(|e| e.vitals.sets()).collect();
    let bps: Vec<(i64, i64)> = sets
        .iter()
        .filter_map(|v| v.bp.as_ref())
        .map(|[s, d]| (num(s).round() as i64, num(d).round() as i64))
        .collect();
    let share = |n: usize, of: usize| n as f64 / of.max(1) as f64;
    if bps.len() >= SERIES {
        let odd = bps.iter().filter(|(s, _)| s % 2 != 0).count();
        if share(odd, bps.len()) < 0.15 {
            errors.push(format!(
                "{mrn}: {odd} of {} systolic readings are odd; automated cuffs read any digit",
                bps.len()
            ));
        }
        let mut seen: HashSet<(i64, i64)> = HashSet::new();
        let repeats = bps.iter().filter(|bp| !seen.insert(**bp)).count();
        if share(repeats, bps.len()) > 0.25 {
            errors.push(format!(
                "{mrn}: {repeats} of {} BP readings repeat an earlier pair exactly",
                bps.len()
            ));
        }
    }
    let hrs: Vec<i64> = sets
        .iter()
        .filter_map(|v| v.hr.as_ref())
        .map(|h| num(h).round() as i64)
        .collect();
    if hrs.len() >= SERIES {
        let even = hrs.iter().filter(|h| *h % 2 == 0).count();
        if share(even, hrs.len()) > 0.8 {
            errors.push(format!(
                "{mrn}: {even} of {} heart rates are even; pulse oximeters read any digit",
                hrs.len()
            ));
        }
    }
    let temps: Vec<String> = sets
        .iter()
        .filter_map(|v| v.temp_c.as_ref())
        .map(|t| format!("{:.1}", num(t)))
        .collect();
    if temps.len() >= SERIES {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for t in &temps {
            *counts.entry(t).or_default() += 1;
        }
        let (top, n) = counts
            .iter()
            .max_by_key(|(t, n)| (**n, **t))
            .expect("readings");
        if share(*n, temps.len()) > 0.4 {
            errors.push(format!(
                "{mrn}: {n} of {} temperatures are {top} C; let them vary",
                temps.len()
            ));
        }
    }
}

/// A seed with coded anchors fixes the problem list: every anchor is recorded, and
/// at most a couple of other problems are still active at the bank time.
fn problem_list(
    mrn: &str,
    anchors: &[String],
    tl: &Timeline,
    catalog: &Catalog,
    errors: &mut Vec<String>,
) {
    for a in anchors {
        if !catalog.conditions.contains_key(a) {
            errors.push(format!(
                "{mrn}: seed anchor {a:?} is not a catalog condition"
            ));
        } else if !tl.problems.iter().any(|p| p.label == a) {
            errors.push(format!(
                "{mrn}: seed anchor {a:?} is never recorded on the problem list"
            ));
        }
    }
    let extra: Vec<&str> = tl
        .problems
        .iter()
        .filter(|p| !anchors.iter().any(|a| a == p.label))
        .filter(|p| !matches!(p.status(time::as_of()), ProblemStatus::Resolved(_)))
        .map(|p| p.label)
        .collect();
    if extra.len() > UNSEEDED_ACTIVE {
        errors.push(format!(
            "{mrn}: {} problems the seed did not plan are still active at the bank time ({}); resolve the acute ones and keep at most {UNSEEDED_ACTIVE} others",
            extra.len(),
            extra.join(", ")
        ));
    }
}

/// Every time an event's records state, by what they are.
fn stamps(ev: &Event) -> Vec<(&'static str, Stamp)> {
    let mut out = vec![("start", ev.start)];
    out.extend(ev.end.map(|e| ("end", e)));
    out.extend(
        ev.medications
            .iter()
            .filter_map(|m| m.date)
            .map(|d| ("medications.date", d)),
    );
    out.extend(
        ev.vitals
            .sets()
            .iter()
            .filter_map(|v| v.time)
            .map(|t| ("vitals.time", t)),
    );
    out.extend(
        ev.labs
            .iter()
            .filter_map(|l| l.collected)
            .map(|t| ("labs.collected", t)),
    );
    out.extend(
        ev.surveys
            .iter()
            .filter_map(|s| s.date)
            .map(|t| ("surveys.date", t)),
    );
    out.extend(
        ev.reports
            .iter()
            .filter_map(|r| r.performed)
            .map(|t| ("reports.performed", t)),
    );
    out.extend(
        ev.procedures
            .iter()
            .filter_map(|p| p.performed)
            .map(|t| ("procedures.performed", t)),
    );
    out.extend(
        ev.immunizations
            .iter()
            .filter_map(|i| i.date)
            .map(|t| ("immunizations.date", t)),
    );
    out.extend(
        ev.documents
            .iter()
            .filter_map(|d| d.date)
            .map(|t| ("documents.date", t)),
    );
    out.extend(
        ev.orders
            .iter()
            .filter_map(|o| o.date)
            .map(|t| ("orders.date", t)),
    );
    out
}

fn check_event<'a>(
    ev: &'a Event,
    eid: &str,
    catalog: &Catalog,
    recorded: &mut HashSet<&'a str>,
    started: &mut HashSet<&'a str>,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    for c in &ev.conditions {
        let label = c.label.as_str();
        if !catalog.conditions.contains_key(label) && c.icd10.is_none() {
            errors.push(format!(
                "{eid}: condition {label:?} is not a catalog label and has no icd10"
            ));
        }
        match c.action {
            ConditionAction::Diagnosed | ConditionAction::History => {
                recorded.insert(label);
                if let Some(onset) = c.onset
                    && onset.0 > ev.start.date()
                {
                    errors.push(format!(
                        "{eid}: condition {label:?} onset {} after recording event {}",
                        onset.0,
                        ev.start.date()
                    ));
                }
            }
            ConditionAction::Resolved if !recorded.contains(label) => {
                errors.push(format!(
                    "{eid}: condition {label:?} resolved before being diagnosed"
                ));
            }
            ConditionAction::Resolved => {}
        }
    }
    for m in &ev.medications {
        if let Some(drug) = &m.drug
            && !catalog.medications.contains_key(drug)
        {
            errors.push(format!(
                "{eid}: drug {drug:?} is not a catalog name (use drug_text instead)"
            ));
        }
        let Some(key) = m.key() else {
            errors.push(format!("{eid}: medication without drug or drug_text"));
            continue;
        };
        match m.action {
            MedicationAction::Start | MedicationAction::Historical => {
                started.insert(key);
            }
            MedicationAction::Stop if !started.contains(key) => {
                errors.push(format!(
                    "{eid}: medication {key:?} stopped before it was started"
                ));
            }
            MedicationAction::Stop => {}
        }
    }
    for lab in &ev.labs {
        let panel = lab.panel.as_deref();
        let members = match panel {
            Some(p) => match catalog.panels.get(p) {
                Some(entry) => Some(&entry.members),
                None => {
                    errors.push(format!("{eid}: unknown panel {p:?}"));
                    None
                }
            },
            None => None,
        };
        let collected = Timeline::collected(ev, lab);
        let mut seen = HashSet::new();
        for r in &lab.results {
            let test = r.test.as_str();
            if !seen.insert(test) {
                errors.push(format!("{eid}: {test:?} is stated twice in one draw"));
            }
            if !catalog.labs.contains_key(test) {
                errors.push(format!("{eid}: unknown lab test {test:?}"));
            } else if lab.pending && (r.value.is_some() || r.text.is_some()) {
                errors.push(format!(
                    "{eid}: lab {test:?} is pending; it has no value until it comes back"
                ));
            } else if !lab.pending && r.value.is_none() && r.text.is_none() {
                errors.push(format!("{eid}: lab {test:?} needs value or text"));
            }
            if let (Some(members), Some(p)) = (members, panel)
                && !members.iter().any(|m| m == test)
            {
                errors.push(format!("{eid}: {test:?} is not a member of {p:?}"));
            }
            if let Some(when) = r.resulted.or(lab.resulted)
                && when < collected
            {
                errors.push(format!(
                    "{eid}: {test:?} resulted {when} before it was collected {collected}"
                ));
            }
        }
        let vals: HashMap<&str, f64> = lab
            .results
            .iter()
            .filter_map(|r| Some((r.test.as_str(), num(r.value.as_ref()?))))
            .collect();
        let stated: Vec<f64> = DIFFERENTIAL
            .iter()
            .filter_map(|t| vals.get(t).copied())
            .collect();
        let total: f64 = stated.iter().sum();
        if total > 101.0 {
            errors.push(format!(
                "{eid}: stated differential totals {}% (over 100); state only the members you mean and code fills the rest to 100",
                fmt_num(total)
            ));
        } else if stated.len() == DIFFERENTIAL.len() && !(98.0..=102.0).contains(&total) {
            errors.push(format!(
                "{eid}: differential totals {}%, must total 100",
                fmt_num(total)
            ));
        }
        let get = |t: &str| vals.get(t).copied().filter(|v| *v != 0.0);
        let (hgb, hct, rbc, mcv) = (
            get("Hemoglobin"),
            get("Hematocrit"),
            get("Red Blood Cells"),
            get("Mean Corpuscular Volume"),
        );
        if rbc.is_some() && mcv.is_none() {
            errors.push(format!(
                "{eid}: Red Blood Cells stated without Mean Corpuscular Volume; state Hgb and MCV (code derives Hct, RBC, MCH and MCHC) or state the indices together"
            ));
        }
        if let (Some(hct), Some(rbc), Some(mcv)) = (hct, rbc, mcv)
            && (hct / rbc * 10.0 - mcv).abs() > 3.0
        {
            errors.push(format!(
                "{eid}: MCV {} disagrees with Hct/RBC ({:.0})",
                fmt_num(mcv),
                hct / rbc * 10.0
            ));
        }
        let implied_hct = hct.or_else(|| Some(rbc? * mcv? / 10.0));
        if let (Some(hgb), Some(h)) = (hgb, implied_hct)
            && !(30.0..=37.0).contains(&(hgb / h * 100.0))
        {
            errors.push(format!(
                "{eid}: Hgb {} with Hct {h:.1} gives MCHC {:.1}, outside what an analyzer reports (30-37)",
                fmt_num(hgb),
                hgb / h * 100.0
            ));
        }
    }
    for s in &ev.surveys {
        if !catalog.surveys.contains_key(&s.survey) {
            errors.push(format!("{eid}: unknown survey {:?}", s.survey));
        }
    }
    for s in &ev.social {
        if !catalog.social.contains_key(&s.item) {
            errors.push(format!("{eid}: unknown social item {:?}", s.item));
        }
    }
    for r in &ev.reports {
        if !catalog.reports.contains_key(&r.study) {
            errors.push(format!("{eid}: unknown report study {:?}", r.study));
        }
        if r.text.trim().is_empty() {
            errors.push(format!("{eid}: report {:?} has no text", r.study));
        }
        if let Some(m) = REAL_NAMES.find(&r.text) {
            errors.push(format!(
                "{eid}: report {:?} names a real organization or product ({})",
                r.study,
                m.as_str()
            ));
        }
        if let Some(issued) = r.issued
            && issued < Timeline::performed(ev, r)
        {
            errors.push(format!(
                "{eid}: report {:?} issued {issued} before it was performed",
                r.study
            ));
        }
    }
    for x in &ev.procedures {
        if !catalog.procedures.contains_key(&x.procedure) {
            errors.push(format!("{eid}: unknown procedure {:?}", x.procedure));
        }
    }
    for i in &ev.immunizations {
        if !catalog.vaccines.contains_key(&i.vaccine) {
            warnings.push(format!(
                "{eid}: vaccine {:?} not in catalog (text only, no CVX)",
                i.vaccine
            ));
        }
    }
    for a in &ev.allergies {
        if !catalog.allergens.contains_key(&a.substance) {
            warnings.push(format!(
                "{eid}: allergen {:?} not in catalog (text only)",
                a.substance
            ));
        }
        if let Some(r) = &a.reaction
            && !catalog.reactions.contains_key(r)
        {
            warnings.push(format!("{eid}: reaction {r:?} not in catalog (text only)"));
        }
    }
}
