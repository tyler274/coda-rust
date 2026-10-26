//! The rules a unit's ledger must meet before it is accepted.
//!
//! Shape is the ledger types' job; these are the rules about content: the
//! patient matches the seed, every code is the catalog's, and the chronology holds
//! (everything before today except booked visits, nothing closed before it was
//! opened, results back only after they were drawn, lab arithmetic an analyzer
//! would produce). Errors send the author back to the part files; warnings are kept
//! with the accepted ledger. None of this judges how a chart reads.

use std::collections::{HashMap, HashSet};

use jiff::ToSpan;

use crate::catalog::Catalog;
use crate::labs::{DIFFERENTIAL, derived};
use crate::ledger::{
    Appointment, ConditionAction, Event, MedicationAction, OrderAction, Patient, Stamp, num,
};
use crate::seeds::Unit;
use crate::time;
use crate::timeline::Timeline;

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
                if d.body.trim().is_empty() {
                    errors.push(format!("{eid}: document {:?} has no body", d.title()));
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
            if !catalog.labs.contains_key(test) && r.loinc.is_none() {
                errors.push(format!(
                    "{eid}: unknown lab test {test:?}: use a label from the vocabulary, or give \"loinc\" (and \"unit\") for a test it lacks"
                ));
            } else if lab.pending && (r.value.is_some() || r.text.is_some()) {
                errors.push(format!(
                    "{eid}: lab {test:?} is pending; it has no value until it comes back"
                ));
            } else if !lab.pending
                && r.value.is_none()
                && r.text.is_none()
                // a panel's derived member is calculated from the panel when rendered
                && !(derived(test) && members.is_some_and(|m| m.iter().any(|m| m == test)))
            {
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
        if !catalog.reports.contains_key(&r.study) && r.loinc.is_none() {
            errors.push(format!(
                "{eid}: unknown report study {:?}: use a study from the vocabulary, or give \"loinc\" for one it lacks",
                r.study
            ));
        }
        if r.text.trim().is_empty() {
            errors.push(format!("{eid}: report {:?} has no text", r.study));
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
            warnings.push(format!(
                "{eid}: procedure {:?} not in catalog (text only, no CPT)",
                x.procedure
            ));
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
