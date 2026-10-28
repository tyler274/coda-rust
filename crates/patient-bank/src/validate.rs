//! Validate the patient bank in bulk and write a report.
//!
//! The validator reads the bank's NDJSON back as plain JSON, independent of the
//! renderer's types. Per patient:
//!
//! ```text
//! schema      every resource against the official HL7 FHIR R4 4.0.1 JSON schema
//! ndjson      one resource type per file, FHIR ids, unique ids, UTC timestamps
//! references  subject/patient, DiagnosticReport.result, Observation.hasMember and
//!             encounter links resolve to the same patient
//! terminology every coding is the catalog's verified code and display, or one the
//!             patient's ledger verified; text-only concepts are counted
//! contract    every coded field has a code (a drug only when its author found no
//!             RxNorm concept), every referral a coded specialty, every measured lab
//!             a UCUM unit and a reference range unless its cited source gives none
//! timeline    clinical times after birth and before the bank time; issued >= effective;
//!             onset <= recorded; abatement after onset
//! one fact    one name is one LOINC code across results, reports and orders; one
//!             specimen has one glucose; a CPT code for an age band fits the age
//! ```
//!
//! The calendar is checked as a whole: valid Appointments for known patients from
//! the bank day on. The report also lists what the charts repeat from one another
//! (see `repeats`).

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

use jiff::civil::Date;
use regex::Regex;
use serde::Serialize;
use serde_json::Value;

use crate::catalog::{CPT, Catalog};
use crate::fhir::{FAMILIES, LOINC};
use crate::ledger::{Ledger, RefRange, Terms};
use crate::render::{DOC_TYPE_CODES, VITAL_CODES};
use crate::seeds::Seeds;
use crate::{Result, paths, repeats, time};

static ID: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z0-9\-.]{1,64}$").expect("regex"));
static UTC: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}\+00:00)?$").expect("regex")
});

const TIME_FIELDS: [&str; 9] = [
    "onsetDateTime",
    "recordedDate",
    "abatementDateTime",
    "authoredOn",
    "effectiveDateTime",
    "issued",
    "performedDateTime",
    "occurrenceDateTime",
    "date",
];

type Chart = BTreeMap<&'static str, Vec<Value>>;

#[derive(Default, Serialize)]
struct PatientReport {
    mrn: String,
    errors: Vec<String>,
    #[serde(skip)]
    stats: BTreeMap<String, usize>,
}

struct Shared {
    validators: Vec<jsonschema::Validator>,
    catalog: Catalog,
    known: HashSet<(String, String, Option<String>)>,
    cpt: HashSet<String>,
}

struct Work {
    mrn: String,
    unit: String,
}

fn load_chart(dir: &Path) -> Result<(Chart, Vec<String>)> {
    let mut chart = Chart::new();
    let mut errors = Vec::new();
    for fam in FAMILIES {
        let path = dir.join(format!("{fam}.ndjson"));
        // a family with no rows may have no file
        if fam != "Patient" && !path.exists() {
            chart.insert(fam, Vec::new());
            continue;
        }
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

/// A resource's clinical times: the dated fields, and an encounter's period.
fn times(r: &Value) -> impl Iterator<Item = (&'static str, &str)> {
    TIME_FIELDS
        .into_iter()
        .filter_map(|k| r[k].as_str().map(|v| (k, v)))
        .chain(r["period"]["start"].as_str().map(|v| ("period.start", v)))
        .chain(r["period"]["end"].as_str().map(|v| ("period.end", v)))
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
                for (k, v) in times(r) {
                    if !UTC.is_match(v) {
                        errors.push(format!("{fam}/{id}: {k} {v} not UTC +00:00"));
                    }
                }
            }
        }
    }
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
            let single = [&r["encounter"], &r["context"]["encounter"]]
                .into_iter()
                .filter(|x| x.is_object())
                .map(|x| ("encounter", x));
            let many = ["result", "hasMember"]
                .into_iter()
                .flat_map(|k| r[k].as_array().into_iter().flatten().map(move |x| (k, x)));
            for (key, x) in single.chain(many) {
                let reference = str_of(x, "reference");
                let (kind, target) = reference.split_once('/').unwrap_or(("", ""));
                if ids.get(target).copied() != Some(kind) {
                    errors.push(format!("{fam}/{id}: {key} {reference} unresolved"));
                }
            }
            for field in ["code", "medicationCodeableConcept", "vaccineCode", "type"] {
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
            contract(sh, ledgers.get(&w.unit), fam, id, r, &mut errors);
            for (k, v) in times(r) {
                // an encounter under way at the bank time ends after it
                if v >= as_of_utc.as_str() && k != "period.end" {
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

    one_fact(sh, &chart, &birth, &mut errors);
    rep.errors = errors;
    Ok(rep)
}

/// Facts the chart must state once: a name's code, a specimen's glucose, and the
/// age a procedure code is for.
fn one_fact(sh: &Shared, chart: &Chart, birth: &str, errors: &mut Vec<String>) {
    let loinc = |r: &Value| {
        (r["code"]["coding"].as_array().into_iter().flatten())
            .find(|c| c["system"] == LOINC)
            .map(|c| str_of(c, "code").to_string())
    };
    let mut codes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for fam in ["Observation", "DiagnosticReport", "ServiceRequest"] {
        for r in &chart[fam] {
            if let Some(code) = loinc(r) {
                let name = r["code"]["text"].as_str().unwrap_or_default().to_string();
                codes.entry(name).or_default().insert(code);
            }
        }
    }
    for (name, set) in codes.iter().filter(|(_, s)| s.len() > 1) {
        errors.push(format!("{name:?} is coded {set:?}; one name is one test"));
    }
    // only results that are back carry a value; a registered one has none to compare
    let mut glucose: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    for r in &chart["Observation"] {
        let value = &r["valueQuantity"]["value"];
        if value.is_number() && matches!(loinc(r).as_deref(), Some("1558-6" | "2345-7")) {
            glucose
                .entry(str_of(r, "effectiveDateTime"))
                .or_default()
                .insert(value.to_string());
        }
    }
    for (at, values) in glucose.iter().filter(|(_, v)| v.len() > 1) {
        errors.push(format!("specimen {at} gives glucose as {values:?}"));
    }
    let Ok(born) = birth.parse::<Date>() else {
        return;
    };
    for r in &chart["Procedure"] {
        let code = (r["code"]["coding"].as_array().into_iter().flatten())
            .find(|c| c["system"] == CPT)
            .map(|c| str_of(c, "code"));
        let ages = (sh.catalog.procedures.values())
            .find(|e| Some(e.code.as_str()) == code)
            .and_then(|e| e.ages);
        let on = str_of(r, "performedDateTime")
            .get(..10)
            .and_then(|d| d.parse::<Date>().ok());
        if let (Some([low, high]), Some(on)) = (ages, on) {
            let age = time::age_on(born, on);
            if !(low..=high).contains(&age) {
                errors.push(format!(
                    "Procedure/{}: CPT {} is for ages {low} to {high}; the patient was {age}",
                    str_of(r, "id"),
                    code.unwrap_or_default()
                ));
            }
        }
    }
}

/// Check the practice calendar and count its visits by status.
fn check_calendar(
    bank: &Path,
    validator: &jsonschema::Validator,
    mrns: &HashSet<&str>,
    errors: &mut Vec<String>,
) -> Result<BTreeMap<String, usize>> {
    let text = fs::read_to_string(bank.join("calendar").join("Appointment.ndjson"))?;
    let day = time::utc(time::as_of())[..10].to_string();
    let mut ids = HashSet::new();
    let mut counts = BTreeMap::new();
    for line in text.lines() {
        let r: Value = serde_json::from_str(line)?;
        let id = str_of(&r, "id");
        if !ids.insert(id.to_string()) {
            errors.push(format!("calendar: duplicate id {id}"));
        }
        if let Some(e) = validator.iter_errors(&r).next() {
            let message: String = e.to_string().chars().take(160).collect();
            errors.push(format!("calendar schema Appointment/{id}: {message}"));
        }
        let (start, end) = (str_of(&r, "start"), str_of(&r, "end"));
        if !UTC.is_match(start) || start < day.as_str() || end <= start {
            errors.push(format!("calendar Appointment/{id}: {start} to {end}"));
        }
        let who = str_of(&r["participant"][0]["actor"], "reference");
        if !who
            .strip_prefix("Patient/")
            .is_some_and(|m| mrns.contains(m))
        {
            errors.push(format!("calendar Appointment/{id}: unknown patient {who}"));
        }
        *counts.entry(str_of(&r, "status").to_string()).or_default() += 1;
    }
    Ok(counts)
}

/// What the tool promises of every resource beyond the schema: each coded field
/// carries a code (only a drug its author marked as having no RxNorm concept stays
/// text), each referral its coded specialty, and each measured lab result its UCUM
/// unit and reference range (unless the catalog's cited source, or the author for a
/// test the catalog lacks, gives none).
fn contract(
    sh: &Shared,
    ledger: Option<&Ledger>,
    fam: &str,
    id: &str,
    r: &Value,
    errors: &mut Vec<String>,
) {
    let coded = |cc: &Value| cc["coding"].as_array().is_some_and(|c| !c.is_empty());
    let text = |cc: &Value| cc["text"].as_str().unwrap_or_default().to_string();
    let field = match fam {
        "MedicationRequest" => "medicationCodeableConcept",
        "Immunization" => "vaccineCode",
        "DocumentReference" | "Encounter" => return,
        _ => "code",
    };
    let cc = &r[field];
    if !cc.is_null() && !coded(cc) {
        let what = text(cc);
        let allowed = match fam {
            "MedicationRequest" => ledger.is_some_and(|l| {
                (l.patients.iter().flat_map(|p| &p.events))
                    .flat_map(|e| &e.medications)
                    .any(|m| m.uncoded && m.drug_text.as_deref() == Some(what.as_str()))
            }),
            // a referral is coded by its specialty
            "ServiceRequest" => r["performerType"].is_array(),
            _ => false,
        };
        if !allowed {
            errors.push(format!("{fam}/{id}: {what:?} has no code"));
        }
    }
    if fam == "ServiceRequest" {
        for p in r["performerType"].as_array().into_iter().flatten() {
            if !coded(p) {
                errors.push(format!("{fam}/{id}: specialty {:?} has no code", text(p)));
            }
        }
    }
    let lab = r["category"].as_array().into_iter().flatten().any(|c| {
        c["coding"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|x| x["code"] == "laboratory")
    });
    if fam == "Observation" && lab && r["valueQuantity"].is_object() {
        let test = text(&r["code"]);
        if r["valueQuantity"]["code"].is_null() {
            errors.push(format!("{fam}/{id}: {test:?} has no UCUM unit"));
        }
        // check holds a name to one answer on whether a range applies
        let none_given = sh.catalog.labs.get(&test).is_some_and(|e| {
            e.range.as_ref().is_some_and(|r| r.is_empty()) && e.range_source.is_some()
        }) || ledger.is_some_and(|l| {
            (l.terms.labs.get(&test)).is_some_and(|v| v.range == Some(RefRange::default()))
        });
        if r["referenceRange"].is_null() && !none_given {
            errors.push(format!("{fam}/{id}: {test:?} has no reference range"));
        }
    }
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
    let loinc = catalog.systems.get("loinc").map(String::as_str);
    let snomed = catalog.systems.get("snomed").map(String::as_str);
    let ours = |m: &std::collections::BTreeMap<String, crate::ledger::VerifiedCode>| {
        m.values()
            .any(|v| v.code == code && Some(v.display.as_str()) == display)
    };
    (Some(system) == loinc && (ours(&terms.labs) || ours(&terms.reports)))
        || (Some(system) == snomed && (ours(&terms.allergens) || ours(&terms.procedures)))
        || (Some(system) == icd
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
    calendar_errors: Vec<String>,
    stats: BTreeMap<String, usize>,
    calendar: BTreeMap<String, usize>,
    repeats: repeats::Repeats,
    failures: Vec<PatientReport>,
}

/// Validate the bank, write the report and print its summary; returns 1 when
/// anything failed.
pub fn run(
    bank: &Path,
    patients: &Path,
    schema: &Path,
    jobs: usize,
    report: Option<&Path>,
) -> Result<i32> {
    let catalog = Catalog::load(&paths::catalog())?;
    let manifest: Value = serde_json::from_str(&fs::read_to_string(bank.join("manifest.json"))?)?;
    let root: Value = serde_json::from_str(
        &fs::read_to_string(schema).map_err(|e| format!("{}: {e}", schema.display()))?,
    )?;
    let mut validators = Vec::new();
    for fam in FAMILIES.into_iter().chain(["Appointment"]) {
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
    let mut work = Vec::new();
    for p in manifest["patients"].as_array().into_iter().flatten() {
        let unit = str_of(p, "unit").to_string();
        if !ledgers.contains_key(&unit) {
            let l = Ledger::read(&bank.join("ledgers").join(format!("{unit}.json")))?;
            ledgers.insert(unit.clone(), l);
        }
        work.push(Work {
            mrn: str_of(p, "mrn").to_string(),
            unit,
        });
    }
    let appointment = validators.pop().expect("the Appointment validator");
    let sh = Shared {
        validators,
        catalog,
        known,
        cpt,
    };
    let results = check_all(&sh, bank, &ledgers, &work, jobs)?;

    let mut calendar_errors = Vec::new();
    let mrns: HashSet<&str> = work.iter().map(|w| w.mrn.as_str()).collect();
    let calendar = check_calendar(bank, &appointment, &mrns, &mut calendar_errors)?;
    let repeats = repeats::find(ledgers.values(), &repeats::own(&Seeds::read(patients)?), 2);
    let mut stats: BTreeMap<String, usize> = BTreeMap::new();
    for r in &results {
        for (k, n) in &r.stats {
            *stats.entry(k.clone()).or_default() += n;
        }
    }
    let error_count = results.iter().map(|r| r.errors.len()).sum::<usize>() + calendar_errors.len();
    let out = report.map(Path::to_path_buf).unwrap_or_else(|| {
        bank.parent()
            .unwrap_or(Path::new("."))
            .join("validation.json")
    });
    let report = Report {
        bank: bank.display().to_string(),
        patients: results.len(),
        patients_with_errors: results.iter().filter(|r| !r.errors.is_empty()).count(),
        errors: error_count,
        calendar_errors,
        stats,
        calendar,
        repeats,
        failures: results
            .into_iter()
            .filter(|r| !r.errors.is_empty())
            .take(200)
            .collect(),
    };
    fs::write(&out, serde_json::to_string_pretty(&report)? + "\n")?;
    let summary = serde_json::json!({
        "patients": report.patients,
        "patients_with_errors": report.patients_with_errors,
        "errors": report.errors,
        "calendar_errors": report.calendar_errors,
        "calendar": report.calendar,
        "stats": report.stats,
        "repeats": report.repeats,
    });
    println!("{}", serde_json::to_string_pretty(&summary)?);
    println!("report: {}", out.display());
    Ok(i32::from(report.errors > 0))
}
