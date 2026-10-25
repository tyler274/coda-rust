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

use crate::labs::{normal, rounded};
use crate::ledger::{Patient, State, Vital, VitalSet, Vitals, num};
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

/// `p` with every measured value filled in, and what could not be measured.
pub fn apply(seed: &Seed, p: &Patient) -> (Patient, Vec<String>) {
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
            for r in lab
                .results
                .iter_mut()
                .filter(|r| r.value.is_none() && r.text.is_none())
            {
                let range = normal(&r.test, female);
                let level = levels.labs.get(&r.test).copied().or_else(|| {
                    // the patient's own normal, the same every draw
                    let ((lo, hi), dec) = range?;
                    let mut rng = Rng::from_text(&format!("normal:{}:{}", p.mrn, r.test));
                    Some((rng.uniform(lo + (hi - lo) * 0.3, hi - (hi - lo) * 0.3), dec))
                });
                let Some((level, given)) = level else {
                    errors.push(format!(
                        "{eid}: {:?} has no value and no level; set its level in `state`",
                        r.test
                    ));
                    continue;
                };
                // a normal level strays a tenth of its range; an abnormal one
                // also strays in proportion to itself
                let (sd, dec) = match range {
                    Some(((lo, hi), dec)) if (lo..=hi).contains(&level) => ((hi - lo) * 0.1, dec),
                    Some(((lo, hi), dec)) => (((hi - lo) * 0.1).max(level.abs() * 0.03), dec),
                    None => (level.abs() * 0.04, given),
                };
                let mut rng = Rng::from_text(&format!("measure:{eid}:{k}:{}", r.test));
                r.value = Some(rounded(rng.gauss(level, sd).max(0.0), dec.max(given)));
            }
        }
    }
    (out, errors)
}
