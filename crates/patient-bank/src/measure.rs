//! Measurements made from the patient's state.
//!
//! An author records where the patient's body is (`state`: blood pressure, weight,
//! an A1c level) and what was taken (`measure` on a contact, a test named in a draw
//! with no value). Code makes the numbers, so no author picks a reading: each is
//! the current level plus the variation a cuff, a scale or an analyzer gives, and a
//! level the author never set is the patient's own normal. Every draw is seeded by
//! the patient, event and test, so the same ledger always measures the same values.
//! A value the author did write stays as written.

use std::collections::HashMap;

use serde_json::Number;

use crate::catalog::Catalog;
use crate::labs::{derived, normal, rounded};
use crate::ledger::{LabResult, Patient, State, Vital, VitalSet, Vitals, num};
use crate::rng::Rng;
use crate::seeds::{Seed, Sex};

/// The patient's current levels while walking the chart forward.
struct Levels {
    vitals: HashMap<Vital, f64>,
    diastolic: f64,
    /// Lab levels and the decimals they were given in.
    labs: HashMap<String, (f64, usize)>,
}

fn decimals(n: &Number) -> usize {
    n.to_string().split_once('.').map_or(0, |(_, d)| d.len())
}

impl Levels {
    /// A healthy adult's resting levels, varied by patient. Weight and height have
    /// no default: the author sets them.
    fn new(rng: &mut Rng) -> Levels {
        let vitals = [
            (Vital::Bp, rng.uniform(112.0, 128.0)),
            (Vital::Hr, rng.uniform(62.0, 80.0)),
            (Vital::Rr, rng.uniform(14.0, 17.0)),
            (Vital::TempC, rng.uniform(36.5, 36.9)),
            (Vital::Spo2, rng.uniform(96.6, 98.4)),
        ]
        .into_iter()
        .collect();
        Levels {
            vitals,
            diastolic: rng.uniform(70.0, 80.0),
            labs: HashMap::new(),
        }
    }

    fn apply(&mut self, s: &State) {
        for (v, n) in [
            (Vital::Hr, &s.hr),
            (Vital::Rr, &s.rr),
            (Vital::TempC, &s.temp_c),
            (Vital::Spo2, &s.spo2),
            (Vital::WeightKg, &s.weight_kg),
            (Vital::HeightCm, &s.height_cm),
        ] {
            if let Some(n) = n {
                self.vitals.insert(v, num(n));
            }
        }
        if let Some([sys, dia]) = &s.bp {
            self.vitals.insert(Vital::Bp, num(sys));
            self.diastolic = num(dia);
        }
        for (t, n) in &s.labs {
            self.labs.insert(t.clone(), (num(n), decimals(n)));
        }
    }
}

/// How far a reading strays from the level, and the decimals it is shown to.
fn spread(v: Vital) -> (f64, usize) {
    match v {
        Vital::Bp => (6.5, 0),
        Vital::Hr => (6.0, 0),
        Vital::Rr => (1.2, 0),
        Vital::TempC => (0.18, 1),
        Vital::Spo2 => (0.8, 0),
        Vital::WeightKg => (0.5, 1),
        Vital::HeightCm => (0.6, 1),
    }
}

/// A derived test's level, when an author sets it, moves the input it is
/// calculated from, so the rendered derivation lands on that level: (derived
/// test, input, the other inputs, the input's level from the derived level and
/// the others' levels). The other inputs are pinned at their levels too, so the
/// renderer derives from the same numbers.
type Solve = fn(f64, &[f64]) -> f64;
const SOLVE: &[(&str, &str, &[&str], Solve)] = &[
    (
        "Total Cholesterol",
        "Low Density Lipoprotein",
        &["High Density Lipoprotein", "Triglycerides"],
        |tc, o| tc - o[0] - o[1] / 5.0,
    ),
    (
        "Non-HDL Cholesterol",
        "Low Density Lipoprotein",
        &["Triglycerides"],
        |n, o| n - o[0] / 5.0,
    ),
    (
        "Transferrin Saturation",
        "Iron",
        &["Total Iron-Binding Capacity"],
        |sat, o| sat * o[0] / 100.0,
    ),
    ("Hematocrit", "Hemoglobin", &[], |hct, _| hct * 0.334),
    (
        "Absolute Neutrophil Count",
        "Neutrophils %",
        &["White Blood Cells"],
        |anc, o| anc / o[0] * 100.0,
    ),
];

/// The patient's own normal for a test, the same every draw.
fn own_normal(mrn: &str, test: &str, female: bool) -> Option<(f64, usize)> {
    let ((lo, hi), dec) = normal(test, female)?;
    let mut rng = Rng::from_text(&format!("normal:{mrn}:{test}"));
    Some((rng.uniform(lo + (hi - lo) * 0.3, hi - (hi - lo) * 0.3), dec))
}

/// `p` with every measured value filled in, and what could not be measured.
pub fn apply(seed: &Seed, p: &Patient, catalog: &Catalog) -> (Patient, Vec<String>) {
    let mut out = p.clone();
    let mut errors = Vec::new();
    let female = seed.sex == Sex::Female;
    let mut levels = Levels::new(&mut Rng::from_text(&format!("levels:{}", p.mrn)));
    let mut order: Vec<usize> = (0..out.events.len()).collect();
    order.sort_by_key(|i| out.events[*i].start);
    for i in order {
        let ev = &mut out.events[i];
        let eid = format!("{}/{}", p.mrn, ev.id);
        if let Some(s) = &ev.state {
            levels.apply(s);
            for (derived, input, others, solve) in SOLVE {
                if s.labs.contains_key(*input) {
                    continue;
                }
                let Some(target) = s.labs.get(*derived).map(num) else {
                    continue;
                };
                let mut got = Vec::new();
                for t in *others {
                    let level = levels.labs.get(*t).copied();
                    let Some(level) = level.or_else(|| own_normal(&p.mrn, t, female)) else {
                        break;
                    };
                    levels.labs.insert(t.to_string(), level);
                    got.push(level.0);
                }
                if got.len() == others.len() {
                    let dec = normal(input, female).map_or(0, |n| n.1);
                    let v = solve(target, &got).max(0.0);
                    levels.labs.insert(input.to_string(), (v, dec));
                }
            }
        }
        if !ev.measure.is_empty() {
            let mut set = match std::mem::take(&mut ev.vitals) {
                Vitals::One(set) => set,
                Vitals::None => VitalSet::default(),
                Vitals::Many(sets) => {
                    errors.push(format!(
                        "{eid}: measure takes one rooming set, not timed sets"
                    ));
                    ev.vitals = Vitals::Many(sets);
                    continue;
                }
            };
            for &v in &ev.measure {
                let mut rng = Rng::from_text(&format!("measure:{eid}:{v:?}"));
                let Some(&level) = levels.vitals.get(&v) else {
                    errors.push(format!("{eid}: {v:?} is measured but no state sets it"));
                    continue;
                };
                let (sd, dec) = spread(v);
                let read = |rng: &mut Rng, level: f64| rounded(rng.gauss(level, sd), dec);
                match v {
                    Vital::Bp if set.bp.is_none() => {
                        let sys = read(&mut rng, level);
                        let dia = rounded(rng.gauss(levels.diastolic, 4.5), 0);
                        set.bp = Some([sys, dia]);
                    }
                    Vital::Hr if set.hr.is_none() => set.hr = Some(read(&mut rng, level)),
                    Vital::Rr if set.rr.is_none() => set.rr = Some(read(&mut rng, level)),
                    Vital::TempC if set.temp_c.is_none() => {
                        set.temp_c = Some(read(&mut rng, level))
                    }
                    Vital::Spo2 if set.spo2.is_none() => {
                        set.spo2 = Some(rounded(rng.gauss(level, sd).min(100.0), 0))
                    }
                    Vital::WeightKg if set.weight_kg.is_none() => {
                        set.weight_kg = Some(read(&mut rng, level))
                    }
                    Vital::HeightCm if set.height_cm.is_none() => {
                        set.height_cm = Some(read(&mut rng, level))
                    }
                    _ => {}
                }
            }
            ev.vitals = Vitals::One(set);
        }
        for (k, lab) in ev.labs.iter_mut().enumerate() {
            if lab.pending {
                continue;
            }
            // a member whose level the author set is measured even when not named
            let members = lab.panel.as_ref().and_then(|p| catalog.panels.get(p));
            for m in members.map(|e| &e.members).into_iter().flatten() {
                if levels.labs.contains_key(m)
                    && !derived(m)
                    && !lab.results.iter().any(|r| &r.test == m)
                {
                    lab.results.push(LabResult {
                        test: m.clone(),
                        loinc: None,
                        unit: None,
                        value: None,
                        text: None,
                        resulted: None,
                    });
                }
            }
            // a derived member is left for the renderer to calculate
            for r in lab
                .results
                .iter_mut()
                .filter(|r| r.value.is_none() && r.text.is_none() && !derived(&r.test))
            {
                let range = normal(&r.test, female);
                let level = (levels.labs.get(&r.test).copied())
                    .or_else(|| own_normal(&p.mrn, &r.test, female));
                let Some((level, given)) = level else {
                    errors.push(format!(
                        "{eid}: {:?} has no value and no level; set its level in `state`",
                        r.test
                    ));
                    continue;
                };
                // a level strays a tenth of its range, but never more than 15% of
                // itself (a ferritin of 5 stays near 5); an abnormal one strays at
                // least 3% of itself
                let (sd, dec) = match range {
                    Some(((lo, hi), dec)) => {
                        let sd = ((hi - lo) * 0.1).min(level.abs() * 0.15);
                        let abnormal = !(lo..=hi).contains(&level);
                        (
                            if abnormal {
                                sd.max(level.abs() * 0.03)
                            } else {
                                sd
                            },
                            dec,
                        )
                    }
                    None => (level.abs() * 0.04, given),
                };
                let mut rng = Rng::from_text(&format!("measure:{eid}:{k}:{}", r.test));
                r.value = Some(rounded(rng.gauss(level, sd).max(0.0), dec.max(given)));
            }
        }
    }
    (out, errors)
}
