//! Validate the patient bank in bulk and write a report.
//!
//! The validator reads the bank's NDJSON back as plain JSON, independent of the
//! renderer's types. Per patient (errors fail the bank; warnings are counted for review):
//!
//! ```text
//! schema      every resource against the official HL7 FHIR R4 4.0.1 JSON schema
//! ndjson      one resource type per file, FHIR ids, unique ids, UTC timestamps
//! references  subject/patient, DiagnosticReport.result, Observation.hasMember resolve
//!             to the same patient
//! terminology every coding is the catalog's verified code and display, or one the
//!             patient's ledger verified; text-only concepts are counted
//! timeline    clinical times after birth and before the bank time; issued >= effective;
//!             onset <= recorded; abatement after onset; stated note ages match birthDate
//! realism     no duplicate note bodies, no scaffold words, no repeated vital tuples
//! snapshots   the chart rendered as of three earlier moments contains nothing dated
//!             at or after the moment, every snapshot row exists in the full chart,
//!             and nothing is stopped or resolved before it was
//! future      note text naming a diagnosis or drug the chart only records later
//!             (lexical; review notes, read by a person)
//! ```

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

use base64::Engine as _;
use regex::Regex;
use serde::Serialize;
use serde_json::Value;

use crate::catalog::{CPT, Catalog};
use crate::check::wrong_ages;
use crate::fhir::{FAMILIES, LOINC};
use crate::ledger::{Day, Ledger, Stamp, Terms};
use crate::render::{DOC_TYPE_CODES, Renderer, VITAL_CODES};
use crate::rng::Rng;
use crate::seeds::{Seed, Seeds};
use crate::{Result, paths, time};

static ID: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z0-9\-.]{1,64}$").expect("regex"));
static UTC: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}\+00:00)?$").expect("regex")
});
static SCAFFOLD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(synthetic|fictional|generated|template|simulation|test patient|answer key|lorem)\b").expect("regex")
});
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z][a-z\-]+").expect("regex"));

const TIME_FIELDS: [&str; 8] = [
    "onsetDateTime",
    "recordedDate",
    "abatementDateTime",
    "authoredOn",
    "effectiveDateTime",
    "issued",
    "performedDateTime",
    "date",
];

/// Words too generic to tell one condition from another in note text.
const STOP: &[&str] = &[
    "disease",
    "disorder",
    "unspecified",
    "chronic",
    "of",
    "with",
    "without",
    "the",
    "and",
    "or",
    "in",
    "acute",
    "primary",
    "history",
    "due",
    "to",
    "status",
    "post",
    "type",
    "syndrome",
    "stage",
    "a",
    "mild",
    "moderate",
    "severe",
    "single",
    "episode",
    "recurrent",
    "other",
    "lower",
    "upper",
    "left",
    "right",
    "bilateral",
    "extremity",
    "extremities",
    "limb",
    "use",
    "current",
    "long-term",
    "term",
    "presence",
    "personal",
    "family",
    "gene",
    "positive",
    "encounter",
    "for",
    "general",
    "adult",
    "medical",
    "examination",
    "normal",
    "supervision",
    "pregnancy",
    "complications",
    "foot",
    "knee",
    "hip",
];
const SYNONYMS: &[(&str, &[&str])] = &[
    ("Essential hypertension", &["hypertension", "htn"]),
    ("Hyperlipidemia", &["hyperlipidemia", "dyslipidemia"]),
    ("Mixed hyperlipidemia", &["hyperlipidemia"]),
    (
        "Type 2 diabetes mellitus without complications",
        &["type 2 diabetes", "t2dm", "diabetes mellitus"],
    ),
    (
        "Type 2 diabetes mellitus with hyperglycemia",
        &["type 2 diabetes", "t2dm"],
    ),
    ("Prediabetes", &["prediabetes", "pre-diabetes"]),
    (
        "Paroxysmal atrial fibrillation",
        &["atrial fibrillation", "afib", "a-fib"],
    ),
    (
        "Persistent atrial fibrillation",
        &["atrial fibrillation", "afib"],
    ),
    ("Chronic obstructive pulmonary disease", &["copd"]),
    (
        "Coronary artery disease",
        &["coronary artery disease", "cad"],
    ),
    ("Obstructive sleep apnea", &["sleep apnea", "osa"]),
    ("Hypothyroidism", &["hypothyroidism"]),
    (
        "Gastroesophageal reflux disease without esophagitis",
        &["gerd", "reflux"],
    ),
    (
        "Generalized anxiety disorder",
        &["generalized anxiety", "gad"],
    ),
    (
        "Heart failure with reduced ejection fraction",
        &["heart failure", "hfref"],
    ),
    (
        "Heart failure with preserved ejection fraction",
        &["heart failure", "hfpef"],
    ),
];
const NO_TERMS: [&str; 7] = [
    "history of",
    "family history",
    "status post",
    "long-term",
    "presence",
    "encounter",
    "supervision",
];

type Chart = BTreeMap<&'static str, Vec<Value>>;

fn decode(data: &Value) -> String {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(data.as_str().unwrap_or(""))
        .unwrap_or_default();
    String::from_utf8_lossy(&bytes).into_owned()
}

fn has_word(text: &str, term: &str) -> bool {
    Regex::new(&format!(r"\b{}\b", regex::escape(term))).is_ok_and(|re| re.is_match(text))
}

fn condition_terms(label: &str) -> Vec<String> {
    if let Some((_, terms)) = SYNONYMS.iter().find(|(l, _)| *l == label) {
        return terms.iter().map(|t| t.to_string()).collect();
    }
    let lower = label.to_lowercase();
    if NO_TERMS.iter().any(|p| lower.starts_with(p)) {
        return Vec::new();
    }
    let words: Vec<&str> = WORD
        .find_iter(&lower)
        .map(|m| m.as_str())
        .filter(|w| !STOP.contains(w) && w.chars().count() > 4)
        .collect();
    let mut out = vec![lower.clone()];
    if words.len() >= 2 {
        out.push(words.join(" "));
    }
    out
}

/// Note text naming a diagnosis or drug that the chart only records later (lexical).
pub fn future_mentions(chart: &Chart) -> Vec<(&'static str, String)> {
    let mut found = Vec::new();
    let mut docs: Vec<&Value> = chart["DocumentReference"].iter().collect();
    docs.sort_by(|a, b| a["date"].as_str().cmp(&b["date"].as_str()));
    let mut first_recorded: Vec<(String, String, Vec<String>)> = Vec::new();
    for r in &chart["Condition"] {
        let label = r["code"]["text"].as_str().unwrap_or("").to_string();
        let when = r["recordedDate"].as_str().unwrap_or("");
        let onset = r["onsetDateTime"].as_str().unwrap_or("");
        let known = if onset.is_empty() {
            when
        } else {
            when.min(onset)
        };
        if !first_recorded.iter().any(|(l, _, _)| *l == label) {
            first_recorded.push((label.clone(), known.to_string(), condition_terms(&label)));
        }
    }
    let mut first_drug: BTreeMap<String, String> = BTreeMap::new();
    for r in &chart["MedicationRequest"] {
        let text = r["medicationCodeableConcept"]["text"]
            .as_str()
            .unwrap_or("")
            .to_lowercase();
        let Some(ing) = WORD.find(&text).map(|m| m.as_str().to_string()) else {
            continue;
        };
        if ing.chars().count() > 4 {
            let authored = r["authoredOn"].as_str().unwrap_or("9999").to_string();
            let e = first_drug.entry(ing).or_insert_with(|| "9999".into());
            if authored < *e {
                *e = authored;
            }
        }
    }
    for doc in docs {
        let text = decode(&doc["content"][0]["attachment"]["data"]).to_lowercase();
        let date = doc["date"].as_str().unwrap_or("");
        let id = doc["id"].as_str().unwrap_or("");
        let day = &date[..date.len().min(10)];
        for (_, known, terms) in &first_recorded {
            if &known[..known.len().min(10)] > day
                && let Some(t) = terms.iter().find(|t| !t.is_empty() && has_word(&text, t))
            {
                found.push((
                    "future_condition_mention",
                    format!(
                        "future condition: {id} ({day}) mentions {t:?}, recorded {}",
                        &known[..10]
                    ),
                ));
            }
        }
        for (ing, started) in &first_drug {
            if &started[..10] > day && has_word(&text, ing) {
                found.push((
                    "future_drug_mention",
                    format!(
                        "future drug: {id} ({day}) mentions {ing:?}, started {}",
                        &started[..10]
                    ),
                ));
            }
        }
    }
    found
}

#[derive(Default, Serialize)]
struct PatientReport {
    mrn: String,
    errors: Vec<String>,
    #[serde(skip)]
    warnings: BTreeMap<&'static str, usize>,
    #[serde(skip)]
    stats: BTreeMap<String, usize>,
    #[serde(skip)]
    notes: Vec<String>,
    /// Problems active or recurring at the bank time, by label.
    #[serde(skip)]
    problems: Vec<String>,
}

struct Shared {
    validators: Vec<jsonschema::Validator>,
    catalog: Catalog,
    known: HashSet<(String, String, Option<String>)>,
    cpt: HashSet<String>,
}

struct Work {
    mrn: String,
    seed: Option<Seed>,
    unit: String,
}

fn load_chart(dir: &Path) -> Result<(Chart, Vec<String>)> {
    let mut chart = Chart::new();
    let mut errors = Vec::new();
    for fam in FAMILIES {
        let path = dir.join(format!("{fam}.ndjson"));
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut rows = Vec::new();
        for line in text.lines() {
            match serde_json::from_str(line) {
                Ok(v) => rows.push(v),
                Err(e) => errors.push(format!("{fam}: unreadable line ({e})")),
            }
        }
        chart.insert(fam, rows);
    }
    Ok((chart, errors))
}

fn str_of<'a>(v: &'a Value, key: &str) -> &'a str {
    v[key].as_str().unwrap_or("")
}

fn check_patient(
    sh: &Shared,
    bank: &Path,
    ledgers: &HashMap<String, Ledger>,
    w: &Work,
) -> Result<PatientReport> {
    let mrn = w.mrn.as_str();
    let (chart, mut errors) = load_chart(&bank.join("patients").join(mrn))?;
    let mut rep = PatientReport {
        mrn: mrn.into(),
        ..PatientReport::default()
    };
    rep.problems = chart["Condition"]
        .iter()
        .filter(|c| {
            matches!(
                str_of(&c["clinicalStatus"]["coding"][0], "code"),
                "active" | "recurrence"
            )
        })
        .filter_map(|c| c["code"]["text"].as_str().map(String::from))
        .collect();
    let mut ids: HashMap<String, &str> = HashMap::new();
    for (slot, fam) in FAMILIES.iter().enumerate() {
        for r in &chart[fam] {
            *rep.stats.entry(fam.to_string()).or_default() += 1;
            let id = str_of(r, "id");
            if str_of(r, "resourceType") != *fam {
                errors.push(format!("{fam}: contains {}", str_of(r, "resourceType")));
            }
            if !ID.is_match(id) {
                errors.push(format!("{fam}/{id}: bad id"));
            }
            if ids.insert(id.to_string(), fam).is_some() {
                errors.push(format!("duplicate id {id}"));
            }
            if let Some(e) = sh.validators[slot].iter_errors(r).next() {
                let message: String = e.to_string().chars().take(160).collect();
                let path: Vec<String> = e.instance_path().iter().map(|s| s.to_string()).collect();
                errors.push(format!(
                    "schema {fam}/{id}: {message} at {}",
                    path.join("/")
                ));
            }
            if *fam != "Patient" {
                for k in TIME_FIELDS {
                    if let Some(v) = r[k].as_str()
                        && !UTC.is_match(v)
                    {
                        errors.push(format!("{fam}/{id}: {k} {v} not UTC +00:00"));
                    }
                }
            }
        }
    }
    let Some(seed) = &w.seed else {
        // a Roger fixture patient: format checks only
        rep.errors = errors;
        return Ok(rep);
    };
    let birth = str_of(&chart["Patient"][0], "birthDate").to_string();
    let as_of_utc = time::utc(time::as_of());

    for (fam, rows) in &chart {
        for r in rows {
            let id = str_of(r, "id");
            if *fam != "Patient" {
                let at = if r["subject"].is_object() {
                    &r["subject"]
                } else {
                    &r["patient"]
                };
                let reference = str_of(at, "reference");
                if reference != format!("Patient/{mrn}") {
                    errors.push(format!("{fam}/{id}: subject {reference}"));
                }
            }
            for key in ["result", "hasMember"] {
                for x in r[key].as_array().into_iter().flatten() {
                    let reference = str_of(x, "reference");
                    let (kind, target) = reference.split_once('/').unwrap_or(("", ""));
                    if ids.get(target).copied() != Some(kind) {
                        errors.push(format!("{fam}/{id}: {key} {reference} unresolved"));
                    }
                }
            }
            for field in ["code", "medicationCodeableConcept", "type"] {
                let Some(cc) = r[field].as_object() else {
                    continue;
                };
                let codings: Vec<&Value> = cc
                    .get("coding")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .collect();
                if field != "type" && *fam != "Patient" && *fam != "DocumentReference" {
                    let kind = if codings.is_empty() {
                        "text-only"
                    } else {
                        "coded"
                    };
                    *rep.stats.entry(format!("{fam}:{kind}")).or_default() += 1;
                }
                for c in codings {
                    let system = str_of(c, "system");
                    let code = str_of(c, "code");
                    if system == CPT {
                        if !sh.cpt.contains(code) {
                            errors.push(format!("{fam}/{id}: CPT {code} not in catalog"));
                        }
                        continue;
                    }
                    let display = c["display"].as_str().map(String::from);
                    let ledger_terms = ledgers.get(&w.unit).map(|l| &l.terms);
                    let known = sh
                        .known
                        .contains(&(system.into(), code.into(), display.clone()))
                        || ledger_terms.is_some_and(|t| {
                            verified(t, &sh.catalog, system, code, display.as_deref())
                        });
                    if !known {
                        errors.push(format!(
                            "{fam}/{id}: unverified coding {system}|{code}|{}",
                            display.as_deref().unwrap_or("None")
                        ));
                    }
                }
            }
            if *fam == "Patient" {
                continue;
            }
            for k in TIME_FIELDS {
                let Some(v) = r[k].as_str() else { continue };
                if v >= as_of_utc.as_str() {
                    errors.push(format!("{fam}/{id}: {k} {v} not before bank time"));
                }
                if v[..v.len().min(10)] < *birth {
                    errors.push(format!("{fam}/{id}: {k} {v} before birth"));
                }
            }
            if let (Some(issued), Some(eff)) =
                (r["issued"].as_str(), r["effectiveDateTime"].as_str())
                && issued < eff
            {
                errors.push(format!("{fam}/{id}: issued before effective"));
            }
            if *fam == "Condition" {
                let onset = str_of(r, "onsetDateTime");
                if onset[..onset.len().min(10)]
                    > str_of(r, "recordedDate")[..10.min(str_of(r, "recordedDate").len())]
                {
                    errors.push(format!("Condition/{id}: onset after recorded"));
                }
                if let Some(a) = r["abatementDateTime"].as_str()
                    && a < onset
                {
                    errors.push(format!("Condition/{id}: abatement before onset"));
                }
            }
        }
    }

    let mut docs: Vec<&Value> = chart["DocumentReference"].iter().collect();
    docs.sort_by(|a, b| str_of(a, "date").cmp(str_of(b, "date")));
    let mut bodies: HashMap<String, &str> = HashMap::new();
    let birth_day: Day = serde_json::from_value(Value::String(birth.clone()))
        .map_err(|e| format!("{mrn}: birthDate {e}"))?;
    for doc in docs {
        let id = str_of(doc, "id");
        let text = decode(&doc["content"][0]["attachment"]["data"]);
        let body = text
            .split_once("\n\n")
            .map_or(text.as_str(), |(_, b)| b)
            .to_string();
        if let Some(first) = bodies.get(&body) {
            errors.push(format!("DocumentReference/{id}: duplicate body of {first}"));
        }
        if let Some(m) = SCAFFOLD.find(&body) {
            errors.push(format!(
                "DocumentReference/{id}: scaffold word {:?}",
                m.as_str()
            ));
        }
        let date = str_of(doc, "date");
        if let Ok(on) = date[..date.len().min(10)].parse::<jiff::civil::Date>() {
            let actual = time::age_on(birth_day.0, on);
            for stated in wrong_ages(&body, actual) {
                *rep.warnings.entry("note_age_mismatch").or_default() += 1;
                rep.notes
                    .push(format!("age: {id} says {stated}, is {actual} ({on})"));
            }
        }
        bodies.insert(body, id);
    }
    let mut tuples: HashMap<(String, String), usize> = HashMap::new();
    for r in &chart["Observation"] {
        if r["code"]["coding"][0]["code"] == "85354-9" {
            let v = |i: usize| r["component"][i]["valueQuantity"]["value"].to_string();
            *tuples.entry((v(0), v(1))).or_default() += 1;
        }
    }
    let repeated = tuples.values().filter(|n| **n >= 4).count();
    if repeated > 0 {
        *rep.warnings.entry("repeated_bp_tuple").or_default() += repeated;
    }
    for (kind, msg) in future_mentions(&chart) {
        *rep.warnings.entry(kind).or_default() += 1;
        rep.notes.push(msg);
    }

    // Snapshots at three earlier moments chosen from the patient's own event times.
    let ledger = ledgers
        .get(&w.unit)
        .ok_or_else(|| format!("{mrn}: no ledger {}", w.unit))?;
    let patient = ledger
        .patient(mrn)
        .ok_or_else(|| format!("{mrn}: not in {}", w.unit))?;
    let renderer = Renderer {
        catalog: &sh.catalog,
        terms: &ledger.terms,
    };
    let mut starts: Vec<Stamp> = patient.events.iter().map(|e| e.start).collect();
    starts.sort();
    let full: HashMap<&str, &Value> = chart
        .values()
        .flatten()
        .map(|x| (str_of(x, "id"), x))
        .collect();
    let mut differs = 0;
    if starts.len() >= 3 {
        let mut rng = Rng::from_text(mrn);
        for _ in 0..3 {
            let cut = starts[rng.randint(1, starts.len() as i64 - 1) as usize];
            let cut_utc = time::utc(cut);
            let (snap, _) = renderer.render(seed, patient, cut);
            for (fam, lines) in snap.lines() {
                for line in lines {
                    let x: Value = serde_json::from_str(&line)?;
                    let id = str_of(&x, "id");
                    let Some(whole) = full.get(id) else {
                        errors.push(format!("snapshot {cut}: {fam}/{id} absent from full chart"));
                        continue;
                    };
                    if fam != "Patient" {
                        for k in TIME_FIELDS {
                            if let Some(v) = x[k].as_str()
                                && v >= cut_utc.as_str()
                            {
                                errors.push(format!(
                                    "snapshot {cut}: {fam}/{id} {k} {v} not before cutoff"
                                ));
                            }
                        }
                    }
                    if fam == "MedicationRequest" && x["status"] != whole["status"] {
                        differs += 1;
                        if x["status"] == "stopped" {
                            errors.push(format!("snapshot {cut}: {id} stopped before its stop"));
                        }
                    }
                    if fam == "Condition" && x["abatementDateTime"] != whole["abatementDateTime"] {
                        differs += 1;
                        // once resolved, a problem stays resolved unless it recurs
                        if x["abatementDateTime"].is_string()
                            && whole["clinicalStatus"]["coding"][0]["code"] == "active"
                        {
                            errors.push(format!("snapshot {cut}: {id} resolved before it was"));
                        }
                    }
                    if matches!(fam, "Observation" | "DiagnosticReport")
                        && x["status"] != whole["status"]
                    {
                        differs += 1;
                    }
                }
            }
            *rep.stats.entry("snapshots".into()).or_default() += 1;
        }
    }
    *rep.stats
        .entry("snapshot_status_differences".into())
        .or_default() += differs;
    rep.errors = errors;
    rep.notes.truncate(50);
    Ok(rep)
}

/// Whether a coding is one the ledger's own verified terms produced.
fn verified(
    terms: &Terms,
    catalog: &Catalog,
    system: &str,
    code: &str,
    display: Option<&str>,
) -> bool {
    let icd = catalog.systems.get("icd10cm").map(String::as_str);
    let rx = catalog.systems.get("rxnorm").map(String::as_str);
    (Some(system) == icd
        && terms
            .conditions
            .values()
            .any(|v| v.code == code && Some(v.display.as_str()) == display))
        || (Some(system) == rx
            && terms
                .medications
                .values()
                .any(|v| v.code == code && Some(v.display.as_str()) == display))
}

fn check_all(
    sh: &Shared,
    bank: &Path,
    ledgers: &HashMap<String, Ledger>,
    work: &[Work],
    jobs: usize,
) -> Result<Vec<PatientReport>> {
    let next = AtomicUsize::new(0);
    let mut done: Vec<(usize, Result<PatientReport>)> = std::thread::scope(|s| {
        let workers: Vec<_> = (0..jobs.max(1))
            .map(|_| {
                s.spawn(|| {
                    let mut out = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        let Some(w) = work.get(i) else { break };
                        out.push((i, check_patient(sh, bank, ledgers, w)));
                    }
                    out
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|h| h.join().expect("validation worker panicked"))
            .collect()
    });
    done.sort_by_key(|(i, _)| *i);
    done.into_iter().map(|(_, r)| r).collect()
}

#[derive(Serialize)]
struct Report {
    bank: String,
    patients: usize,
    patients_with_errors: usize,
    errors: usize,
    panel_errors: Vec<String>,
    panels: BTreeMap<String, usize>,
    warnings: BTreeMap<&'static str, usize>,
    stats: BTreeMap<String, usize>,
    terminology_extras: Extras,
    problem_list: ProblemList,
    failures: Vec<PatientReport>,
    review_notes: BTreeMap<String, Vec<String>>,
}

/// How varied the panel is: problems active at the bank time across all patients.
#[derive(Serialize)]
struct ProblemList {
    distinct: usize,
    /// Labels carried by only one patient.
    unique: usize,
    /// The most common labels, as a percent of patients.
    top: Vec<(String, f64)>,
}

impl ProblemList {
    fn of(results: &[PatientReport]) -> ProblemList {
        let mut carriers: BTreeMap<&str, usize> = BTreeMap::new();
        for r in results {
            let mine: HashSet<&str> = r.problems.iter().map(String::as_str).collect();
            for label in mine {
                *carriers.entry(label).or_default() += 1;
            }
        }
        let mut top: Vec<(&str, usize)> = carriers.iter().map(|(l, n)| (*l, *n)).collect();
        top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        let percent = |n: usize| (1000.0 * n as f64 / results.len().max(1) as f64).round() / 10.0;
        ProblemList {
            distinct: carriers.len(),
            unique: carriers.values().filter(|n| **n == 1).count(),
            top: top
                .iter()
                .take(25)
                .map(|(l, n)| (l.to_string(), percent(*n)))
                .collect(),
        }
    }
}

#[derive(Serialize)]
struct Extras {
    conditions: usize,
    medications: usize,
    rejected: Vec<String>,
}

/// Validate the bank, write the report and print its summary; returns 1 when
/// anything failed.
pub fn run(
    bank: &Path,
    seeds: &Path,
    schema: &Path,
    jobs: usize,
    report: Option<&Path>,
) -> Result<i32> {
    let catalog = Catalog::load(&paths::catalog())?;
    let seeds = Seeds::read(seeds)?;
    let manifest: Value = serde_json::from_str(&fs::read_to_string(bank.join("manifest.json"))?)?;
    let root: Value = serde_json::from_str(
        &fs::read_to_string(schema).map_err(|e| format!("{}: {e}", schema.display()))?,
    )?;
    let mut validators = Vec::new();
    for fam in FAMILIES {
        let mut sub = root.clone();
        let obj = sub
            .as_object_mut()
            .ok_or("the FHIR schema is not an object")?;
        obj.remove("oneOf");
        obj.remove("discriminator");
        obj.insert("$ref".into(), Value::String(format!("#/definitions/{fam}")));
        let v = jsonschema::draft6::options()
            .should_validate_formats(false)
            .build(&sub)
            .map_err(|e| format!("{fam} schema: {e}"))?;
        validators.push(v);
    }
    let mut known: HashSet<(String, String, Option<String>)> = HashSet::new();
    for (_, section) in catalog.sections() {
        for e in section.values() {
            known.insert((e.system.clone(), e.code.clone(), e.display.clone()));
        }
    }
    for (code, display) in VITAL_CODES
        .iter()
        .copied()
        .chain(DOC_TYPE_CODES.iter().map(|(_, c)| *c))
    {
        known.insert((LOINC.into(), code.into(), Some(display.into())));
    }
    let cpt = catalog
        .procedures
        .values()
        .map(|e| e.code.clone())
        .collect();

    let mut ledgers: HashMap<String, Ledger> = HashMap::new();
    let mut extras = Extras {
        conditions: 0,
        medications: 0,
        rejected: Vec::new(),
    };
    let mut work = Vec::new();
    for p in manifest["patients"].as_array().into_iter().flatten() {
        let mrn = str_of(p, "mrn").to_string();
        let unit = str_of(p, "unit").to_string();
        if unit == "roger" {
            work.push(Work {
                mrn,
                seed: None,
                unit,
            });
            continue;
        }
        if !ledgers.contains_key(&unit) {
            let l = Ledger::read(&bank.join("ledgers").join(format!("{unit}.json")))?;
            extras.conditions += l.terms.conditions.len();
            extras.medications += l.terms.medications.len();
            extras.rejected.extend(l.terms.rejected.iter().cloned());
            ledgers.insert(unit.clone(), l);
        }
        let (_, seed) = seeds
            .member(&mrn)
            .ok_or_else(|| format!("{mrn} has no seed"))?;
        work.push(Work {
            mrn,
            seed: Some(seed.clone()),
            unit,
        });
    }
    let sh = Shared {
        validators,
        catalog,
        known,
        cpt,
    };
    let results = check_all(&sh, bank, &ledgers, &work, jobs)?;

    // Panels: requested size, known members, nested, families whole.
    let mut panel_errors = Vec::new();
    let mut sizes = BTreeMap::new();
    let mrns: HashSet<&str> = work.iter().map(|w| w.mrn.as_str()).collect();
    let family_of: HashMap<&str, &str> = work
        .iter()
        .filter_map(|w| Some((w.mrn.as_str(), w.seed.as_ref()?.family_id.as_deref()?)))
        .collect();
    let mut files: Vec<(usize, std::path::PathBuf)> = fs::read_dir(bank.join("panels"))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter_map(|p| {
            let n = p
                .file_stem()?
                .to_str()?
                .strip_prefix("panel-")?
                .parse()
                .ok()?;
            Some((n, p))
        })
        .collect();
    files.sort();
    let mut prev: Option<HashSet<String>> = None;
    for (_, path) in files {
        let pan: Value = serde_json::from_str(&fs::read_to_string(&path)?)?;
        let stem = path
            .file_stem()
            .expect("stem")
            .to_string_lossy()
            .to_string();
        let mem: HashSet<String> = pan["members"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|m| m.as_str().map(String::from))
            .collect();
        sizes.insert(stem.clone(), mem.len());
        if Some(mem.len() as u64) != pan["requested"].as_u64() {
            panel_errors.push(format!(
                "{stem}: {} members, requested {}",
                mem.len(),
                pan["requested"]
            ));
        }
        if !mem.iter().all(|m| mrns.contains(m.as_str())) {
            panel_errors.push(format!("{stem}: unknown members"));
        }
        if prev.as_ref().is_some_and(|p| !p.is_subset(&mem)) {
            panel_errors.push(format!("{stem}: does not contain the smaller panel"));
        }
        let fams: HashSet<&str> = mem
            .iter()
            .filter_map(|m| family_of.get(m.as_str()).copied())
            .collect();
        let mut fams: Vec<&str> = fams.into_iter().collect();
        fams.sort_unstable();
        for fid in fams {
            if !family_of
                .iter()
                .filter(|(_, f)| **f == fid)
                .all(|(m, _)| mem.contains(*m))
            {
                panel_errors.push(format!("{stem}: splits {fid}"));
            }
        }
        prev = Some(mem);
    }

    let mut warnings: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut stats: BTreeMap<String, usize> = BTreeMap::new();
    let mut review_notes = BTreeMap::new();
    for r in &results {
        for (k, n) in &r.warnings {
            *warnings.entry(k).or_default() += n;
        }
        for (k, n) in &r.stats {
            *stats.entry(k.clone()).or_default() += n;
        }
        if !r.notes.is_empty() {
            review_notes.insert(r.mrn.clone(), r.notes.clone());
        }
    }
    let error_count = results.iter().map(|r| r.errors.len()).sum::<usize>() + panel_errors.len();
    let patients_with_errors = results.iter().filter(|r| !r.errors.is_empty()).count();
    let out = report.map(Path::to_path_buf).unwrap_or_else(|| {
        bank.parent()
            .unwrap_or(Path::new("."))
            .join("validation.json")
    });
    let report = Report {
        bank: bank.display().to_string(),
        patients: results.len(),
        patients_with_errors,
        errors: error_count,
        panel_errors,
        panels: sizes,
        warnings,
        stats,
        terminology_extras: extras,
        problem_list: ProblemList::of(&results),
        failures: results
            .into_iter()
            .filter(|r| !r.errors.is_empty())
            .take(200)
            .collect(),
        review_notes,
    };
    fs::write(&out, serde_json::to_string_pretty(&report)? + "\n")?;
    let summary = serde_json::json!({
        "patients": report.patients,
        "patients_with_errors": report.patients_with_errors,
        "errors": report.errors,
        "panels": report.panels,
        "warnings": report.warnings,
        "stats": report.stats,
        "panel_errors": report.panel_errors,
        "problem_list": report.problem_list,
    });
    println!("{}", serde_json::to_string_pretty(&summary)?);
    println!("report: {}", out.display());
    Ok(i32::from(report.errors > 0))
}
