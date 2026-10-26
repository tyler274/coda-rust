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

/// Calculated members: (test, inputs, formula from the inputs' values).
type Formula = fn(&[f64], &Draw) -> f64;
const DERIVED: &[(&str, &[&str], Formula, usize)] = &[
    (
        "Mean Corpuscular Hemoglobin",
        &["Hemoglobin", "Red Blood Cells"],
        |v, _| v[0] / v[1] * 10.0,
        1,
    ),
    (
        "Mean Corpuscular Hemoglobin Concentration",
        &["Hemoglobin", "Hematocrit"],
        |v, _| v[0] / v[1] * 100.0,
        1,
    ),
    (
        "Absolute Neutrophil Count",
        &["White Blood Cells", "Neutrophils %"],
        |v, _| v[0] * v[1] / 100.0,
        1,
    ),
    (
        "Estimated Glomerular Filtration Rate (Creatinine)",
        &["Creatinine"],
        |v, d| num(&ckd_epi_2021(v[0], d.age, d.female)),
        0,
    ),
    (
        "Anion Gap",
        &["Sodium", "Chloride", "Bicarbonate"],
        |v, _| v[0] - v[1] - v[2],
        0,
    ),
    (
        "Total Cholesterol",
        &[
            "Low Density Lipoprotein",
            "High Density Lipoprotein",
            "Triglycerides",
        ],
        |v, _| v[0] + v[1] + v[2] / 5.0,
        0,
    ),
    (
        "Non-HDL Cholesterol",
        &["Total Cholesterol", "High Density Lipoprotein"],
        |v, _| v[0] - v[1],
        0,
    ),
    (
        "Transferrin Saturation",
        &["Iron", "Total Iron-Binding Capacity"],
        |v, _| v[0] / v[1] * 100.0,
        0,
    ),
    (
        // INR is (PT / the lab's normal PT) ^ ISI; with ISI 1 and a normal PT of
        // 12.2 s, PT follows the INR the chart is managed by
        "Prothrombin Time",
        &["Prothrombin Time-International Normalized Ratio"],
        |v, _| v[0] * 12.2,
        1,
    ),
];

/// Hematocrit and red cells are calculated too, from hemoglobin and MCV.
pub fn derived(test: &str) -> bool {
    matches!(test, "Hematocrit" | "Red Blood Cells") || DERIVED.iter().any(|d| d.0 == test)
}

/// A level set on a calculated test moves the input it is calculated from, so the
/// calculation lands on that level: (calculated test, input, the other inputs,
/// the input's level from the set level and the others' levels). The other
/// inputs are pinned at their levels, so the draw calculates from the same ones.
type Solve = fn(f64, &[f64]) -> f64;
pub const SOLVE: &[(&str, &str, &[&str], Solve)] = &[
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
    let open = |lab: &Lab, t: &str| at(t).is_some_and(|i| lab.results[i].value.is_none());
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
    let diff_open: Vec<&str> = DIFFERENTIAL
        .iter()
        .copied()
        .filter(|t| open(lab, t) && !d.levels.contains_key(*t))
        .collect();
    for i in 0..lab.results.len() {
        let r = &lab.results[i];
        if r.value.is_some()
            || r.text.is_some()
            || derived(&r.test)
            || diff_open.contains(&r.test.as_str())
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
    for (t, inputs, f, dec) in DERIVED {
        if !open(lab, t) {
            continue;
        }
        let got: Option<Vec<f64>> = inputs.iter().map(|i| value(lab, i)).collect();
        if let Some(got) = got {
            set(lab, t, f(&got, d), *dec, inputs);
        }
    }
    // a member code could not make is left out, unless the author named it
    lab.results
        .retain(|r| r.value.is_some() || r.text.is_some() || named.contains(&r.test));
}
