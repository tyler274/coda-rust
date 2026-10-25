//! Lab draws as Observations and DiagnosticReports.
//!
//! A panel lists only the members the author had a reason to state. Unstated
//! members are filled with per-patient normal baselines that vary from draw to
//! draw; red-cell indices, the rest of the differential, eGFR (CKD-EPI 2021), anion
//! gap, cholesterol totals and transferrin saturation are derived from the values
//! in the same draw. Each filled or derived value is listed in the render receipt.

use std::collections::HashMap;

use serde_json::Number;

use crate::catalog::Catalog;
use crate::fhir::{
    CodeableConcept, Coding, DiagnosticReport, Meta, Observation, Quantity, Reference,
    ReferenceRange,
};
use crate::ledger::{Event, Lab, Stamp, num};
use crate::render::{Receipt, opaque};
use crate::rng::Rng;
use crate::time;
use crate::timeline::Timeline;

pub const OBS_CAT: &str = "http://terminology.hl7.org/CodeSystem/observation-category";
pub const V2_0074: &str = "http://terminology.hl7.org/CodeSystem/v2-0074";
const INTERPRETATION: &str = "http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation";

/// A normal range used to fill an unstated member, by sex, and the decimals reported.
struct Fill {
    male: (f64, f64),
    female: (f64, f64),
    decimals: usize,
}

const fn both(lo: f64, hi: f64, decimals: usize) -> Fill {
    Fill {
        male: (lo, hi),
        female: (lo, hi),
        decimals,
    }
}

const fn by_sex(male: (f64, f64), female: (f64, f64), decimals: usize) -> Fill {
    Fill {
        male,
        female,
        decimals,
    }
}

const FILL: &[(&str, Fill)] = &[
    ("White Blood Cells", both(4.5, 9.5, 1)),
    ("Hemoglobin", by_sex((13.8, 16.6), (12.2, 15.2), 1)),
    ("Red Cell Distribution Width", both(11.8, 14.0, 1)),
    ("Platelets", both(170.0, 360.0, 0)),
    ("Mean Platelet Volume", both(8.0, 11.0, 1)),
    ("Mean Corpuscular Volume", both(84.0, 96.0, 0)),
    ("Neutrophils %", both(48.0, 68.0, 0)),
    ("Lymphocytes %", both(22.0, 38.0, 0)),
    ("Monocytes %", both(4.0, 9.0, 0)),
    ("Eosinophils %", both(0.5, 5.0, 0)),
    ("Basophils %", both(0.0, 1.0, 0)),
    ("Sodium", both(137.0, 143.0, 0)),
    ("Potassium", both(3.7, 4.8, 1)),
    ("Chloride", both(99.0, 106.0, 0)),
    ("Bicarbonate", both(23.0, 29.0, 0)),
    ("Blood Urea Nitrogen", both(9.0, 20.0, 0)),
    ("Creatinine", by_sex((0.75, 1.15), (0.55, 0.95), 2)),
    ("Glucose", both(78.0, 104.0, 0)),
    ("Calcium", both(8.9, 10.1, 1)),
    ("Albumin", both(3.9, 4.8, 1)),
    ("Total Protein", both(6.4, 7.8, 1)),
    ("Aspartate Aminotransferase", both(14.0, 32.0, 0)),
    ("Alanine Aminotransferase", both(11.0, 34.0, 0)),
    ("Alkaline Phosphatase", both(48.0, 110.0, 0)),
    ("Bilirubin, Total", both(0.3, 1.0, 1)),
    ("Bilirubin, Direct", both(0.0, 0.3, 1)),
    (
        "High Density Lipoprotein",
        by_sex((40.0, 58.0), (50.0, 72.0), 0),
    ),
    ("Triglycerides", both(65.0, 145.0, 0)),
    ("Low Density Lipoprotein", both(85.0, 125.0, 0)),
    ("Iron", both(65.0, 150.0, 0)),
    ("Total Iron-Binding Capacity", both(260.0, 380.0, 0)),
    ("Ferritin", by_sex((40.0, 260.0), (20.0, 140.0), 0)),
    ("Prothrombin Time", both(11.2, 13.2, 1)),
    (
        "Prothrombin Time-International Normalized Ratio",
        both(0.9, 1.1, 1),
    ),
    ("Urine Specific Gravity", both(1.008, 1.024, 3)),
    ("Urine pH", both(5.0, 7.0, 1)),
    ("Urine White Blood Cells", both(0.0, 2.0, 0)),
    ("Urine Red Blood Cells", both(0.0, 2.0, 0)),
];

const FILL_TEXT: &[(&str, &str)] = &[
    ("Urine Color", "Yellow"),
    ("Urine Clarity", "Clear"),
    ("Urine Protein", "Negative"),
    ("Urine Glucose", "Negative"),
    ("Urine Ketones", "Negative"),
    ("Urine Blood", "Negative"),
    ("Urine Leukocyte Esterase", "Negative"),
    ("Urine Nitrite", "Negative"),
];

pub const DIFFERENTIAL: [&str; 5] = [
    "Neutrophils %",
    "Lymphocytes %",
    "Monocytes %",
    "Eosinophils %",
    "Basophils %",
];

const DERIVED: &[&str] = &[
    EGFR,
    "Anion Gap",
    "Non-HDL Cholesterol",
    "Total Cholesterol",
    "Hematocrit",
    "Red Blood Cells",
    "Mean Corpuscular Hemoglobin",
    "Mean Corpuscular Hemoglobin Concentration",
    "Absolute Neutrophil Count",
    "Transferrin Saturation",
];

const EGFR: &str = "Estimated Glomerular Filtration Rate (Creatinine)";

fn table<'a, V>(t: &'a [(&str, V)], key: &str) -> Option<&'a V> {
    t.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
}

/// `v` rounded to `decimals` places (half to even on the exact value); whole
/// numbers when `decimals` is 0.
pub fn rounded(v: f64, decimals: usize) -> Number {
    let s = format!("{v:.decimals$}");
    if decimals == 0 {
        Number::from(s.parse::<i64>().expect("whole number"))
    } else {
        Number::from_f64(s.parse().expect("decimal")).expect("finite")
    }
}

pub fn ckd_epi_2021(scr: f64, age: i64, female: bool) -> Number {
    let (k, a) = if female { (0.7, -0.241) } else { (0.9, -0.302) };
    let egfr = 142.0
        * (scr / k).min(1.0).powf(a)
        * (scr / k).max(1.0).powf(-1.200)
        * 0.9938f64.powf(age as f64)
        * if female { 1.012 } else { 1.0 };
    rounded(egfr, 0)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Origin {
    Stated,
    Filled,
    Derived,
}

struct Value<'a> {
    number: Option<Number>,
    text: Option<&'a str>,
    origin: Origin,
    released: Stamp,
}

/// Per-patient normal baselines, shared by every draw of one chart.
#[derive(Default)]
pub struct Baselines {
    by_test: HashMap<&'static str, f64>,
    mchc: Option<f64>,
}

impl Baselines {
    /// An unstated member's value: the patient's baseline plus draw-to-draw variation.
    fn fill(&mut self, test: &str, female: bool, rng: &mut Rng) -> Option<Number> {
        let (&name, spec) = FILL.iter().find(|(k, _)| *k == test).map(|(k, v)| (k, v))?;
        let (lo, hi) = if female { spec.female } else { spec.male };
        let base = *self
            .by_test
            .entry(name)
            .or_insert_with(|| rng.uniform(lo + (hi - lo) * 0.2, hi - (hi - lo) * 0.2));
        let v = (base + rng.gauss(0.0, (hi - lo) * 0.15)).clamp(lo, hi);
        Some(rounded(v, spec.decimals))
    }
}

pub struct LabContext<'a> {
    pub catalog: &'a Catalog,
    pub timeline: &'a Timeline<'a>,
    pub subject: Reference,
    pub female: bool,
    pub at: Stamp,
}

pub struct Rendered {
    pub observations: Vec<Observation>,
    pub report: Option<DiagnosticReport>,
}

/// Lab `i` of `ev` as of `cx.at`: nothing before collection, `registered`
/// observations until each result is released.
pub fn render(
    cx: &LabContext,
    ev: &Event,
    i: usize,
    lab: &Lab,
    baselines: &mut Baselines,
    receipt: &mut Receipt,
) -> Rendered {
    let none = Rendered {
        observations: Vec::new(),
        report: None,
    };
    let collected = Timeline::collected(ev, lab);
    if collected >= cx.at {
        return none;
    }
    let mrn = &cx.timeline.patient.mrn;
    let panel_name = lab.panel.as_deref();
    let panel = panel_name.and_then(|p| cx.catalog.panels.get(p));
    let mut rng = cx.timeline.lab_draws(ev, i);
    let usual = time::plus(collected, Timeline::lab_turnaround(&mut rng, lab));
    let released = lab.resulted.unwrap_or(usual);

    let mut values: HashMap<&str, Value> = HashMap::new();
    let mut stated_order: Vec<&str> = Vec::new();
    for r in &lab.results {
        if !cx.catalog.labs.contains_key(&r.test) {
            receipt.skipped.push(format!("{}:lab:{}", ev.id, r.test));
            continue;
        }
        if values.contains_key(r.test.as_str()) {
            continue;
        }
        stated_order.push(&r.test);
        let v = Value {
            number: r.value.clone(),
            text: r.text.as_deref(),
            origin: Origin::Stated,
            released: r.resulted.unwrap_or(released),
        };
        values.insert(&r.test, v);
    }
    let members: Vec<&str> = match panel {
        Some(p) => p.members.iter().map(String::as_str).collect(),
        None => stated_order,
    };
    values.retain(|t, _| members.contains(t));

    if panel.is_some() {
        derive_and_fill(
            cx,
            collected,
            released,
            &members,
            &mut values,
            baselines,
            &mut rng,
        );
    }

    let lab_cat = CodeableConcept::coding_only(Coding::new(OBS_CAT, "laboratory", "Laboratory"));
    let effective = time::utc(collected);
    let mut out = Rendered {
        observations: Vec::new(),
        report: None,
    };
    let mut refs = Vec::new();
    let mut latest = collected;
    let mut all_released = true;
    let mut any_released = false;
    for t in &members {
        let Some(v) = values.get(t) else { continue };
        let entry = &cx.catalog.labs[*t];
        let pending = v.released >= cx.at;
        let code = CodeableConcept::coded(
            Coding::new(
                &entry.system,
                &entry.code,
                entry.display.as_deref().unwrap_or(t),
            ),
            t,
        );
        let id = opaque("obs", &[mrn, &ev.id, &i.to_string(), t]);
        let mut obs = Observation::new(
            id.clone(),
            if pending { "registered" } else { "final" },
            lab_cat.clone(),
            code,
            &cx.subject,
            effective.clone(),
        );
        if !pending {
            obs.issued = Some(time::utc(v.released));
            if let Some(n) = &v.number {
                obs.value_quantity = Some(Quantity::in_unit(n.clone(), entry.unit.as_deref()));
                if let Some((lo, hi)) = entry.reference_range(cx.female) {
                    let bound = |x: f64| {
                        Quantity::in_unit(
                            Number::from_f64(x).expect("finite"),
                            entry.unit.as_deref(),
                        )
                    };
                    obs.reference_range = vec![ReferenceRange {
                        low: bound(lo),
                        high: bound(hi),
                    }];
                    let x = num(n);
                    let flag = if x > hi {
                        Some(("H", "High"))
                    } else if x < lo {
                        Some(("L", "Low"))
                    } else {
                        None
                    };
                    if let Some((code, display)) = flag {
                        obs.interpretation = vec![CodeableConcept::coding_only(Coding::new(
                            INTERPRETATION,
                            code,
                            display,
                        ))];
                    }
                }
            } else if let Some(text) = v.text {
                obs.value_codeable_concept = Some(CodeableConcept::text(text));
            } else {
                continue;
            }
        }
        match v.origin {
            Origin::Filled => receipt.filled.push(format!("{}:{t}", ev.id)),
            Origin::Derived => receipt.derived.push(format!("{}:{t}", ev.id)),
            Origin::Stated => {}
        }
        if pending {
            all_released = false;
        } else {
            any_released = true;
            latest = latest.max(v.released);
        }
        out.observations.push(obs);
        refs.push(Reference::to("Observation", &id));
    }
    if !all_released {
        receipt.pending.push(format!(
            "{}:{} pending at cutoff",
            ev.id,
            panel_name.unwrap_or("lab")
        ));
    }
    if let (Some(p), false) = (panel, refs.is_empty()) {
        let name = panel_name.expect("panel has a name");
        let code = CodeableConcept::coded(
            Coding::new(&p.system, &p.code, p.display.as_deref().unwrap_or(name)),
            name,
        );
        let status = match (all_released, any_released) {
            (true, _) => "final",
            (false, true) => "partial",
            (false, false) => "registered",
        };
        let issued = all_released.then(|| time::utc(latest));
        out.report = Some(DiagnosticReport {
            resource_type: "DiagnosticReport",
            id: opaque("diagrep", &[mrn, &ev.id, &i.to_string()]),
            meta: Meta::on(&effective),
            status,
            category: vec![CodeableConcept::coding_only(Coding::new(
                V2_0074,
                "LAB",
                "Laboratory",
            ))],
            code: code.clone(),
            subject: cx.subject.clone(),
            encounter: None,
            effective_date_time: effective.clone(),
            issued: issued.clone(),
            result: refs.clone(),
            conclusion: None,
            presented_form: Vec::new(),
        });
        let id = opaque("obs-panel", &[mrn, &ev.id, &i.to_string()]);
        let mut po = Observation::new(
            id,
            if all_released { "final" } else { "registered" },
            lab_cat,
            code,
            &cx.subject,
            effective,
        );
        po.issued = issued;
        po.has_member = refs;
        out.observations.push(po);
    }
    out
}

/// Fill unstated panel members and derive the computed ones, in a fixed order so
/// the same ledger always draws the same values.
fn derive_and_fill<'a>(
    cx: &LabContext,
    collected: Stamp,
    released: Stamp,
    members: &[&'a str],
    values: &mut HashMap<&'a str, Value<'a>>,
    baselines: &mut Baselines,
    rng: &mut Rng,
) {
    let female = cx.female;
    let age = time::age_on(cx.timeline.patient.birth_date.0, collected.date());
    let has = |t: &str| members.contains(&t);
    let num_of = |values: &HashMap<&str, Value>, t: &str| -> Option<f64> {
        values
            .get(t)
            .and_then(|v| v.number.as_ref())
            .map(num)
            .filter(|x| *x != 0.0)
    };
    let put = |values: &mut HashMap<&'a str, Value<'a>>,
               t: &'a str,
               n: Number,
               origin: Origin,
               released: Stamp| {
        values.insert(
            t,
            Value {
                number: Some(n),
                text: None,
                origin,
                released,
            },
        );
    };
    // A computed value is released with the last of its inputs.
    let after = |values: &HashMap<&str, Value>, inputs: &[&str]| -> Stamp {
        inputs
            .iter()
            .filter_map(|i| values.get(i).map(|v| v.released))
            .fold(released, Stamp::max)
    };
    let member = |t: &str| members.iter().copied().find(|m| *m == t);

    let diff_open: Vec<&str> = DIFFERENTIAL
        .iter()
        .copied()
        .filter(|t| has(t) && !values.contains_key(t))
        .collect();
    if let Some(t) = member("Mean Corpuscular Volume").filter(|t| !values.contains_key(t))
        && let (Some(hct), Some(rbc)) = (
            num_of(values, "Hematocrit"),
            num_of(values, "Red Blood Cells"),
        )
    {
        let when = after(values, &["Hematocrit", "Red Blood Cells"]);
        put(
            values,
            t,
            rounded(hct / rbc * 10.0, 0),
            Origin::Derived,
            when,
        );
    }
    for &t in members {
        if values.contains_key(t) || DERIVED.contains(&t) || diff_open.contains(&t) {
            continue;
        }
        if let Some(text) = table(FILL_TEXT, t) {
            values.insert(
                t,
                Value {
                    number: None,
                    text: Some(text),
                    origin: Origin::Filled,
                    released,
                },
            );
        } else if let Some(n) = baselines.fill(t, female, rng) {
            put(values, t, n, Origin::Filled, released);
        }
    }
    if !diff_open.is_empty() {
        // unstated differential members share what the stated ones leave of 100%
        let stated: f64 = DIFFERENTIAL.iter().filter_map(|t| num_of(values, t)).sum();
        let room = 100.0 - stated;
        let share: Vec<(&str, f64)> = diff_open
            .iter()
            .map(|t| (*t, baselines.fill(t, female, rng).map_or(0.0, |n| num(&n))))
            .collect();
        let total = Some(share.iter().map(|s| s.1).sum::<f64>())
            .filter(|t| *t != 0.0)
            .unwrap_or(1.0);
        let mut fill: Vec<(&str, i64)> = share
            .iter()
            .map(|(t, v)| (*t, (room * v / total).round_ties_even().max(0.0) as i64))
            .collect();
        let biggest = share
            .iter()
            .enumerate()
            .fold(0, |best, (k, s)| if s.1 > share[best].1 { k } else { best });
        let sum: i64 = fill.iter().map(|f| f.1).sum();
        fill[biggest].1 = (fill[biggest].1 + room.round_ties_even() as i64 - sum).max(0);
        let when = after(values, &DIFFERENTIAL);
        for (t, v) in fill {
            let t = member(t).expect("differential member");
            put(values, t, Number::from(v), Origin::Filled, when);
        }
    }
    // Red-cell indices from one consistent set: Hgb, MCV and MCHC give Hct and RBC.
    if let (Some(hgb), Some(mcv)) = (
        num_of(values, "Hemoglobin"),
        num_of(values, "Mean Corpuscular Volume"),
    ) {
        let mut hct = num_of(values, "Hematocrit");
        let mut rbc = num_of(values, "Red Blood Cells");
        if hct.is_none()
            && let Some(rbc) = rbc
        {
            hct = Some(num(&rounded(rbc * mcv / 10.0, 1)));
        }
        let hct = hct.unwrap_or_else(|| {
            let base = *baselines
                .mchc
                .get_or_insert_with(|| rng.uniform(32.6, 34.2));
            let mchc = base + rng.uniform(-0.3, 0.3);
            num(&rounded(hgb / mchc * 100.0, 1))
        });
        let rbc = *rbc.get_or_insert_with(|| num(&rounded(hct / mcv * 10.0, 2)));
        let when = after(
            values,
            &[
                "Hemoglobin",
                "Mean Corpuscular Volume",
                "Hematocrit",
                "Red Blood Cells",
            ],
        );
        for (t, v, dec) in [("Hematocrit", hct, 1), ("Red Blood Cells", rbc, 2)] {
            if let Some(t) = member(t).filter(|t| !values.contains_key(t)) {
                put(values, t, rounded(v, dec), Origin::Derived, when);
            }
        }
    }
    let derive = |values: &mut HashMap<&'a str, Value<'a>>,
                  t: &str,
                  inputs: &[&str],
                  f: &dyn Fn(&[f64]) -> Number| {
        let Some(t) = member(t).filter(|t| !values.contains_key(t)) else {
            return;
        };
        let got: Option<Vec<f64>> = inputs.iter().map(|i| num_of(values, i)).collect();
        if let Some(got) = got {
            let when = after(values, inputs);
            put(values, t, f(&got), Origin::Derived, when);
        }
    };
    derive(
        values,
        "Mean Corpuscular Hemoglobin",
        &["Hemoglobin", "Red Blood Cells"],
        &|v| rounded(v[0] / v[1] * 10.0, 1),
    );
    derive(
        values,
        "Mean Corpuscular Hemoglobin Concentration",
        &["Hemoglobin", "Hematocrit"],
        &|v| rounded(v[0] / v[1] * 100.0, 1),
    );
    derive(
        values,
        "Absolute Neutrophil Count",
        &["White Blood Cells", "Neutrophils %"],
        &|v| rounded(v[0] * v[1] / 100.0, 1),
    );
    derive(values, EGFR, &["Creatinine"], &|v| {
        ckd_epi_2021(v[0], age, female)
    });
    derive(
        values,
        "Anion Gap",
        &["Sodium", "Chloride", "Bicarbonate"],
        &|v| rounded(v[0] - v[1] - v[2], 0),
    );
    derive(
        values,
        "Total Cholesterol",
        &[
            "Low Density Lipoprotein",
            "High Density Lipoprotein",
            "Triglycerides",
        ],
        &|v| rounded(v[0] + v[1] + v[2] / 5.0, 0),
    );
    derive(
        values,
        "Non-HDL Cholesterol",
        &["Total Cholesterol", "High Density Lipoprotein"],
        &|v| rounded(v[0] - v[1], 0),
    );
    derive(
        values,
        "Transferrin Saturation",
        &["Iron", "Total Iron-Binding Capacity"],
        &|v| rounded(v[0] / v[1] * 100.0, 0),
    );
}
