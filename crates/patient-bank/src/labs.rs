//! Lab draws as Observations and DiagnosticReports, and the lab facts code
//! measures with: each test's normal range, its usual qualitative result and the
//! eGFR formula. Every value is already in the measured ledger (see `draw`); a
//! draw renders as its results.

use std::collections::HashMap;

use serde_json::Number;

use crate::catalog::Catalog;
use crate::fhir::{
    CodeableConcept, Coding, DiagnosticReport, Meta, Observation, Quantity, Reference,
    ReferenceRange,
};
use crate::ledger::{Event, Lab, RefRange, Stamp, num};
use crate::render::{Receipt, opaque};
use crate::time;
use crate::timeline::Timeline;

pub const OBS_CAT: &str = "http://terminology.hl7.org/CodeSystem/observation-category";
pub const V2_0074: &str = "http://terminology.hl7.org/CodeSystem/v2-0074";
const INTERPRETATION: &str = "http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation";

/// A test's normal range in a healthy adult, by sex, and the decimals reported.
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

/// A qualitative member's usual result.
pub const FILL_TEXT: &[(&str, &str)] = &[
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

fn table<'a, V>(t: &'a [(&str, V)], key: &str) -> Option<&'a V> {
    t.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
}

/// `v` rounded to `decimals` places (half to even on the exact value); whole
/// numbers when `decimals` is 0.
/// A test's normal range for this sex and the decimals it is reported to.
pub fn normal(test: &str, female: bool) -> Option<((f64, f64), usize)> {
    let spec = table(FILL, test)?;
    Some((if female { spec.female } else { spec.male }, spec.decimals))
}

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

struct Value<'a> {
    number: Option<Number>,
    text: Option<&'a str>,
    released: Stamp,
    /// The range the author stated for this result (a test the catalog lacks).
    range: Option<&'a RefRange>,
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
pub fn render(cx: &LabContext, ev: &Event, i: usize, lab: &Lab, receipt: &mut Receipt) -> Rendered {
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
    let released = if lab.pending {
        time::never()
    } else {
        lab.resulted.unwrap_or(usual)
    };

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
            released: r.resulted.filter(|_| !lab.pending).unwrap_or(released),
            range: r.range.as_ref(),
        };
        values.insert(&r.test, v);
    }
    let members: Vec<&str> = match panel {
        Some(p) => p.members.iter().map(String::as_str).collect(),
        None => stated_order,
    };
    values.retain(|t, _| members.contains(t));

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
                let stated = (v.range).map(|r| (r.low.as_ref().map(num), r.high.as_ref().map(num)));
                let range = match stated {
                    Some((None, None)) => None,
                    Some(r) => Some(r),
                    None => entry.reference_range(cx.female),
                };
                if let Some((lo, hi)) = range {
                    let bound = |x: f64| {
                        Quantity::in_unit(
                            Number::from_f64(x).expect("finite"),
                            entry.unit.as_deref(),
                        )
                    };
                    obs.reference_range = vec![ReferenceRange {
                        low: lo.map(bound),
                        high: hi.map(bound),
                    }];
                    let x = num(n);
                    let flag = if hi.is_some_and(|hi| x > hi) {
                        Some(("H", "High"))
                    } else if lo.is_some_and(|lo| x < lo) {
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
