//! Every number of one lab draw.
//!
//! A panel lists only the members the author had a reason to state. Code makes
//! the rest the way the analyzer would: a member with a level reads that level
//! plus the analyzer's variation, a member with none reads the patient's own
//! normal, a qualitative member reads its usual result, the open differential
//! shares what the stated members leave of 100%, and red-cell indices, eGFR
//! (CKD-EPI 2021), anion gap, cholesterol totals, transferrin saturation and
//! prothrombin time are calculated from the draw's other values. A value the
//! author wrote stays as written. Every value lands in the ledger, so the ledger
//! holds the whole chart.

use std::collections::HashMap;

use crate::labs::{DIFFERENTIAL, FILL_TEXT, ckd_epi_2021, normal, rounded};
use crate::ledger::{Lab, LabResult, Stamp, num};
use crate::rng::Rng;

/// Who and which draw, for seeds, error messages and age-dependent formulas.
pub struct Draw<'a> {
    pub mrn: &'a str,
    /// `MRN/event`, as errors name it.
    pub eid: &'a str,
    /// The draw's place in its event.
    pub k: usize,
    pub female: bool,
    pub age: i64,
    /// Lab levels the chart has set by now, and the decimals they were given in.
    pub levels: &'a HashMap<String, (f64, usize)>,
}

/// The patient's own normal for a test, the same every draw.
pub fn own_normal(mrn: &str, test: &str, female: bool) -> Option<(f64, usize)> {
    let ((lo, hi), dec) = normal(test, female)?;
    let mut rng = Rng::from_text(&format!("normal:{mrn}:{test}"));
    Some((rng.uniform(lo + (hi - lo) * 0.3, hi - (hi - lo) * 0.3), dec))
}

/// How many decimals a test reports: its normal's, else its formula's.
pub fn decimals(test: &str, female: bool) -> usize {
    normal(test, female)
        .map(|n| n.1)
        .or_else(|| {
            IDENTITIES
                .iter()
                .find(|i| i.test == test)
                .map(|i| i.decimals)
        })
        .unwrap_or(1)
}

fn fmt(x: f64) -> String {
    let s = format!("{x:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// A calculated test: the formula an analyzer or the lab applies to its inputs.
/// It holds on every draw. Code calculates the test from its inputs; when the test
/// is stated and an input is not, code solves that input so the calculation lands
/// on the stated value. A level set on the test moves `moves`, the input the
/// patient's condition moves, with the other inputs pinned at their levels.
pub struct Identity {
    pub test: &'static str,
    pub inputs: &'static [&'static str],
    /// The test from the inputs' values, for a patient of this age and sex.
    pub formula: fn(&[f64], i64, bool) -> f64,
    pub decimals: usize,
    pub moves: Option<usize>,
    /// For a formula with a lab-chosen constant, the lowest and highest test value
    /// any real lab's choice gives; `formula` is the one this practice's lab uses.
    pub band: Option<Band>,
}

/// The lowest and highest test value from the inputs' values.
pub type Band = fn(&[f64]) -> (f64, f64);

impl Identity {
    /// The value of input `k` that makes the formula give `target`, the other
    /// inputs held at `values`; `None` when no positive value does. Every formula
    /// is monotonic in each input, so bisection finds the one root.
    pub fn solve(
        &self,
        k: usize,
        target: f64,
        values: &[f64],
        age: i64,
        female: bool,
    ) -> Option<f64> {
        let at = |x: f64| {
            let mut v = values.to_vec();
            v[k] = x;
            (self.formula)(&v, age, female) - target
        };
        let (mut lo, mut hi) = (1e-6, 1.0);
        while at(lo).signum() == at(hi).signum() {
            hi *= 2.0;
            if hi > 1e7 {
                return None;
            }
        }
        for _ in 0..200 {
            let mid = (lo + hi) / 2.0;
            if at(mid).signum() == at(lo).signum() {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Some((lo + hi) / 2.0)
    }
}

/// Every calculated test, each after the tests it is calculated from (total
/// cholesterol before non-HDL), so calculating runs forward through the table and
/// solving runs backward.
pub const IDENTITIES: &[Identity] = &[
    Identity {
        test: "Mean Corpuscular Hemoglobin",
        inputs: &["Hemoglobin", "Red Blood Cells"],
        formula: |v, _, _| v[0] / v[1] * 10.0,
        decimals: 1,
        moves: None,
        band: None,
    },
    Identity {
        test: "Mean Corpuscular Hemoglobin Concentration",
        inputs: &["Hemoglobin", "Hematocrit"],
        formula: |v, _, _| v[0] / v[1] * 100.0,
        decimals: 1,
        moves: None,
        band: None,
    },
    Identity {
        test: "Absolute Neutrophil Count",
        inputs: &["White Blood Cells", "Neutrophils %"],
        formula: |v, _, _| v[0] * v[1] / 100.0,
        decimals: 1,
        moves: Some(1),
        band: None,
    },
    Identity {
        test: "Estimated Glomerular Filtration Rate (Creatinine)",
        inputs: &["Creatinine"],
        formula: |v, age, female| num(&ckd_epi_2021(v[0], age, female)),
        decimals: 0,
        moves: Some(0),
        band: None,
    },
    Identity {
        test: "Anion Gap",
        inputs: &["Sodium", "Chloride", "Bicarbonate"],
        formula: |v, _, _| v[0] - v[1] - v[2],
        decimals: 0,
        moves: None,
        band: None,
    },
    Identity {
        test: "Total Cholesterol",
        inputs: &[
            "Low Density Lipoprotein",
            "High Density Lipoprotein",
            "Triglycerides",
        ],
        // LDL is calculated as TC - HDL - TG/f: Friedewald's f is 5, Martin-Hopkins
        // picks f from 3.1 to 11.9 by triglycerides and non-HDL
        formula: |v, _, _| v[0] + v[1] + v[2] / 5.0,
        decimals: 0,
        moves: Some(0),
        band: Some(|v| (v[0] + v[1] + v[2] / 11.9, v[0] + v[1] + v[2] / 3.1)),
    },
    Identity {
        test: "Non-HDL Cholesterol",
        inputs: &["Total Cholesterol", "High Density Lipoprotein"],
        formula: |v, _, _| v[0] - v[1],
        decimals: 0,
        moves: Some(0),
        band: None,
    },
    Identity {
        test: "Transferrin Saturation",
        inputs: &["Iron", "Total Iron-Binding Capacity"],
        formula: |v, _, _| v[0] / v[1] * 100.0,
        decimals: 0,
        moves: Some(0),
        band: None,
    },
    Identity {
        // INR is (PT / the lab's normal PT) ^ ISI; with ISI 1 and a normal PT of
        // 12.2 s, PT follows the INR the chart is managed by. Reagents run ISI 1.0
        // to 1.3 and normal PT 11.2 to 13.2 s
        test: "Prothrombin Time",
        inputs: &["Prothrombin Time-International Normalized Ratio"],
        formula: |v, _, _| v[0] * 12.2,
        decimals: 1,
        moves: Some(0),
        band: Some(|v| {
            let (a, b) = (11.2 * v[0].powf(1.0 / 1.3), 13.2 * v[0]);
            (a.min(b), a.max(b))
        }),
    },
];

/// Hematocrit and red cells are calculated too, from hemoglobin and MCV.
pub fn derived(test: &str) -> bool {
    matches!(test, "Hematocrit" | "Red Blood Cells") || IDENTITIES.iter().any(|d| d.test == test)
}

/// A reading of `level`: a tenth of the test's range, but never more than 15% of
/// the level (a ferritin of 5 stays near 5); an abnormal level strays at least 3%
/// of itself.
fn read(test: &str, (level, given): (f64, usize), female: bool, rng: &mut Rng) -> f64 {
    let (sd, dec) = match normal(test, female) {
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
    num(&rounded(rng.gauss(level, sd).max(0.0), dec.max(given)))
}

/// Fill every member of `lab` that has no value; `members` is its panel's.
pub fn fill(lab: &mut Lab, members: &[String], d: &Draw, errors: &mut Vec<String>) {
    let named: Vec<String> = lab.results.iter().map(|r| r.test.clone()).collect();
    for m in members.iter().filter(|m| !named.contains(m)) {
        lab.results.push(LabResult {
            test: m.clone(),
            loinc: None,
            unit: None,
            range: None,
            value: None,
            text: None,
            resulted: None,
        });
    }
    let index: HashMap<String, usize> = (lab.results.iter().enumerate())
        .map(|(i, r)| (r.test.clone(), i))
        .collect();
    let at = |t: &str| index.get(t).copied();
    let value = |lab: &Lab, t: &str| {
        at(t)
            .and_then(|i| lab.results[i].value.as_ref())
            .map(num)
            .filter(|x| *x != 0.0)
    };
    // a result is answered by a value or by text ("not calculated")
    let open = |lab: &Lab, t: &str| {
        at(t).is_some_and(|i| lab.results[i].value.is_none() && lab.results[i].text.is_none())
    };
    // A calculated value is released with the last of its inputs.
    let released = |lab: &Lab, inputs: &[&str]| -> Option<Stamp> {
        inputs
            .iter()
            .filter_map(|t| at(t).and_then(|i| lab.results[i].resulted))
            .max()
    };
    let set = |lab: &mut Lab, t: &str, v: f64, dec: usize, inputs: &[&str]| {
        let when = released(lab, inputs);
        let r = &mut lab.results[at(t).expect("member")];
        r.value = Some(rounded(v, dec));
        r.resulted = r.resulted.or(when);
    };
    let rng = |t: &str| Rng::from_text(&format!("measure:{}:{}:{t}", d.eid, d.k));

    // MCV from a stated hematocrit and red cell count
    if open(lab, "Mean Corpuscular Volume")
        && let (Some(hct), Some(rbc)) = (value(lab, "Hematocrit"), value(lab, "Red Blood Cells"))
    {
        set(
            lab,
            "Mean Corpuscular Volume",
            hct / rbc * 10.0,
            0,
            &["Hematocrit", "Red Blood Cells"],
        );
    }
    // A stated calculated test fixes one of its open inputs: the one its level
    // moves, else the last. Plan which before anything is filled, so that input is
    // solved from the stated test, not read from its own level.
    // Only a number solves; a formula with an input answered in text ("not
    // calculated") cannot, so it holds back none of its inputs.
    let mut numbers: Vec<&str> = (lab.results.iter())
        .filter(|r| r.value.is_some())
        .map(|r| r.test.as_str())
        .collect();
    let answered: Vec<&str> = (lab.results.iter())
        .filter(|r| r.text.is_some())
        .map(|r| r.test.as_str())
        .collect();
    let mut plan: Vec<(&Identity, usize)> = Vec::new();
    for id in IDENTITIES.iter().rev() {
        if !numbers.contains(&id.test)
            || (id.inputs.iter()).any(|t| at(t).is_none() || answered.contains(t))
        {
            continue;
        }
        let unknown: Vec<usize> = (0..id.inputs.len())
            .filter(|&k| !numbers.contains(&id.inputs[k]))
            .collect();
        let Some(&last) = unknown.last() else {
            continue;
        };
        plan.push((id, id.moves.filter(|m| unknown.contains(m)).unwrap_or(last)));
        numbers.extend(id.inputs);
    }
    let solved: Vec<&str> = plan.iter().map(|(id, k)| id.inputs[*k]).collect();
    let diff_open: Vec<&str> = DIFFERENTIAL
        .iter()
        .copied()
        .filter(|t| open(lab, t) && !d.levels.contains_key(*t) && !solved.contains(t))
        .collect();
    for i in 0..lab.results.len() {
        let r = &lab.results[i];
        if r.value.is_some()
            || r.text.is_some()
            || derived(&r.test)
            || diff_open.contains(&r.test.as_str())
            || solved.contains(&r.test.as_str())
        {
            continue;
        }
        let test = r.test.clone();
        if let Some(text) = FILL_TEXT.iter().find(|(k, _)| *k == test).map(|(_, v)| *v) {
            lab.results[i].text = Some(text.to_string());
            continue;
        }
        let level = (d.levels.get(&test).copied()).or_else(|| own_normal(d.mrn, &test, d.female));
        match level {
            Some(level) => {
                let dec = normal(&test, d.female).map_or(level.1, |n| n.1.max(level.1));
                let v = read(&test, level, d.female, &mut rng(&test));
                lab.results[i].value = Some(rounded(v, dec));
            }
            None if named.contains(&test) => errors.push(format!(
                "{}: {test:?} has no value and no level; set its level in `state`",
                d.eid
            )),
            None => {}
        }
    }
    for (id, k) in plan {
        let input = id.inputs[k];
        let values: Option<Vec<f64>> = (id.inputs.iter().enumerate())
            .map(|(j, t)| if j == k { Some(0.0) } else { value(lab, t) })
            .collect();
        let (Some(target), Some(values)) = (value(lab, id.test), values) else {
            continue;
        };
        match id.solve(k, target, &values, d.age, d.female) {
            Some(v) => {
                let others: Vec<&str> = (id.inputs.iter().copied())
                    .filter(|t| *t != input)
                    .chain([id.test])
                    .collect();
                set(lab, input, v, decimals(input, d.female), &others);
            }
            None => errors.push(format!(
                "{}: {} {} cannot come from this draw's other values: no {input} gives it",
                d.eid,
                id.test,
                fmt(target)
            )),
        }
    }
    if !diff_open.is_empty() {
        // the open differential shares what the stated members leave of 100%
        let stated: f64 = DIFFERENTIAL.iter().filter_map(|t| value(lab, t)).sum();
        let room = 100.0 - stated;
        let share: Vec<f64> = (diff_open.iter())
            .map(|t| {
                own_normal(d.mrn, t, d.female).map_or(0.0, |l| read(t, l, d.female, &mut rng(t)))
            })
            .collect();
        let total = Some(share.iter().sum::<f64>())
            .filter(|t| *t != 0.0)
            .unwrap_or(1.0);
        let mut parts: Vec<i64> = (share.iter())
            .map(|v| (room * v / total).round_ties_even().max(0.0) as i64)
            .collect();
        let biggest =
            (0..share.len()).fold(0, |best, k| if share[k] > share[best] { k } else { best });
        let sum: i64 = parts.iter().sum();
        parts[biggest] = (parts[biggest] + room.round_ties_even() as i64 - sum).max(0);
        for (t, v) in diff_open.iter().zip(parts) {
            set(lab, t, v as f64, 0, &DIFFERENTIAL);
        }
    }
    // red-cell indices from one consistent set: Hgb, MCV and MCHC give Hct and RBC
    if let (Some(hgb), Some(mcv)) = (
        value(lab, "Hemoglobin"),
        value(lab, "Mean Corpuscular Volume"),
    ) {
        let inputs = ["Hemoglobin", "Mean Corpuscular Volume"];
        let rbc = value(lab, "Red Blood Cells");
        let hct = value(lab, "Hematocrit").unwrap_or_else(|| match rbc {
            Some(rbc) => num(&rounded(rbc * mcv / 10.0, 1)),
            None => {
                let base = Rng::from_text(&format!("normal:{}:mchc", d.mrn)).uniform(32.6, 34.2);
                let mchc = base + rng("mchc").uniform(-0.3, 0.3);
                num(&rounded(hgb / mchc * 100.0, 1))
            }
        });
        if open(lab, "Hematocrit") {
            set(lab, "Hematocrit", hct, 1, &inputs);
        }
        if open(lab, "Red Blood Cells") {
            set(lab, "Red Blood Cells", hct / mcv * 10.0, 2, &inputs);
        }
    }
    for id in IDENTITIES {
        if !open(lab, id.test) {
            continue;
        }
        let got: Option<Vec<f64>> = id.inputs.iter().map(|i| value(lab, i)).collect();
        if let Some(got) = got {
            let v = (id.formula)(&got, d.age, d.female);
            set(lab, id.test, v, id.decimals, id.inputs);
        }
    }
    // a member code could not make is left out, unless the author named it
    lab.results
        .retain(|r| r.value.is_some() || r.text.is_some() || named.contains(&r.test));
}
