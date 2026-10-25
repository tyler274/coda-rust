//! The authoring rules a unit's ledger must meet before it is accepted.
//!
//! Shape is the ledger types' job; these are the rules about content: seed
//! agreement, the catalog vocabulary, chronology, lab arithmetic and note hygiene.
//! Errors send the author back to the part files; warnings are kept with the
//! accepted ledger for review. None of this judges clinical reasoning.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use regex::Regex;

use crate::catalog::Catalog;
use crate::labs::DIFFERENTIAL;
use crate::ledger::{ConditionAction, Event, MedicationAction, Patient, Stamp, num};
use crate::seeds::Unit;
use crate::time;
use crate::timeline::Timeline;

static FORBIDDEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(synthetic|fictional|generated|template|simulation|simulated|test patient|answer key|placeholder|lorem)\b")
        .expect("regex")
});

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
            for (what, stamp) in stamps(ev) {
                if stamp >= as_of && what != "end" {
                    errors.push(format!(
                        "{eid}: {what} {stamp} is not before {}",
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
                if let Some(m) = REAL_NAMES.find(body) {
                    errors.push(format!(
                        "{eid}: document {title:?} names a real organization or product ({}); use a role or generic description",
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
            if ev.kind.needs_note() && ev.documents.is_empty() {
                errors.push(format!("{eid}: visit without a note"));
            }
        }
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
            let expected = as_of.date().year() as i64 - i64::from(seed.years_with_practice);
            if (i64::from(first.date().year()) - expected).abs() > 2 {
                warnings.push(format!(
                    "{mrn}: first practice event {} vs about {expected}",
                    first.date()
                ));
            }
        }
    }
    (errors, warnings)
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
            } else if r.value.is_none() && r.text.is_none() {
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
