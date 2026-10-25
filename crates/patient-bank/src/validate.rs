//! Validate the patient bank in bulk and write a report.
//!
//! Checks, per patient (errors fail the bank; warnings are counted for review):
//!   schema      every resource against the official HL7 FHIR R4 4.0.1 JSON schema
//!   ndjson      one resource type per file, FHIR ids, unique ids, UTC timestamps
//!   references  subject/patient, DiagnosticReport.result, Observation.hasMember resolve
//!               to the same patient
//!   terminology every coding is the catalog's verified code and display, or a
//!               render-time verified extra; text-only concepts are counted
//!   timeline    clinical times after birth and before the bank time; issued >= effective;
//!               onset <= recorded; abatement after onset; stated note ages match birthDate
//!   realism     no duplicate note bodies, no scaffold words, no repeated vital tuples
//!   snapshots   the chart rendered as of three earlier instants contains nothing dated
//!               at or after the instant, every snapshot row exists in the full chart,
//!               and medication/problem statuses are the ones true at that instant
//!   future      note text mentioning a diagnosis or drug that the chart only records
//!               later (lexical; warning, reviewed by reading)
//!
//! A port of `validate.py` with the same report. Schema errors name the same
//! resource and instance path; their message is the Rust `jsonschema` crate's
//! wording, not Python `jsonschema`'s.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

use base64::Engine as _;
use jiff::civil::Date;
use regex::Regex;

use crate::bank::{Counter, py_str, read_ndjson};
use crate::catalog::{Result, pathlib_str, py_repr};
use crate::json::{self, Json, Layout};
use crate::ledger::{age_in_text, boundary, head, is_decimal, is_word, match_at, search_words};
use crate::render::{self, Chart, FAMILIES, Renderer};
use crate::rng::PyRandom;
use crate::{obj, paths, terms, time};

pub const AS_OF_UTC: &str = "2026-08-27T16:00:00+00:00";
const CPT: &str = "http://www.ama-assn.org/go/cpt";
// Python's `$` also matches before a final newline.
static ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[A-Za-z0-9\-.]{1,64}\n?\z").expect("ID_RE"));
static UTC_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\A\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}\+00:00)?\n?\z").expect("UTC_RE")
});
static LOWER_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z][a-z\-]+").expect("word"));
/// `\b(...)\b`, case-insensitive.
const SCAFFOLD: [&str; 8] = [
    "synthetic",
    "fictional",
    "generated",
    "template",
    "simulation",
    "test patient",
    "answer key",
    "lorem",
];
/// `\b(...|grand\w*|...)\b`, case-insensitive; `grand\w*` is checked on its own.
const KIN: &[&str] = &[
    "mother", "father", "sister", "brother", "son", "daughter", "wife", "husband", "spouse",
    "partner", "aunt", "uncle", "cousin", "friend", "at age", "in her", "in his", "niece",
    "nephew",
];
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

/// A patient's resources by family: a rendered `Chart`, or one read back from NDJSON.
pub trait Families {
    fn family(&self, name: &str) -> &[Json];
}

impl Families for Chart {
    fn family(&self, name: &str) -> &[Json] {
        self.get(name)
    }
}

/// A chart as read from a patient directory, in `FAMILIES` order.
pub struct Loaded(Vec<Vec<Json>>);

impl Families for Loaded {
    fn family(&self, name: &str) -> &[Json] {
        let at = FAMILIES
            .iter()
            .position(|f| *f == name)
            .unwrap_or_else(|| panic!("not a chart family: {name}"));
        &self.0[at]
    }
}

pub fn load_chart(d: &Path) -> Result<Loaded> {
    let mut rows = Vec::new();
    for fam in FAMILIES {
        let p = d.join(format!("{fam}.ndjson"));
        rows.push(if p.exists() {
            read_ndjson(&p)?
        } else {
            Vec::new()
        });
    }
    Ok(Loaded(rows))
}

/// `base64.b64decode(data).decode("utf-8", "replace")`
fn b64text(data: &Json) -> Result<String> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(data.as_str().unwrap_or(""))?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

/// `re.search(r"\b" + re.escape(term) + r"\b", text)` is not None.
fn has_word(text: &str, term: &str) -> bool {
    let word_at = |c: Option<char>| c.is_some_and(is_word);
    let mut from = 0;
    while let Some(off) = text[from..].find(term) {
        let at = from + off;
        let end = at + term.len();
        let first = text[at..].chars().next();
        let last = text[..end].chars().next_back();
        if word_at(text[..at].chars().next_back()) != word_at(first)
            && word_at(last) != word_at(text[end..].chars().next())
        {
            return true;
        }
        from = at + first.map_or(1, char::len_utf8);
    }
    false
}

fn condition_terms(label: &str) -> Vec<String> {
    if let Some((_, terms)) = SYNONYMS.iter().find(|(l, _)| *l == label) {
        return terms.iter().map(|t| t.to_string()).collect();
    }
    let lower = label.to_lowercase();
    if NO_TERMS.iter().any(|p| lower.starts_with(p)) {
        return Vec::new();
    }
    let words: Vec<&str> = LOWER_WORD
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

fn str_or<'a>(v: &'a Json, default: &'a str) -> &'a str {
    match v {
        Json::Str(s) => s,
        _ => default,
    }
}

/// Note text naming a diagnosis or drug the chart only records later (lexical):
/// (warning kind, message).
pub fn future_mentions(chart: &impl Families) -> Result<Vec<(&'static str, String)>> {
    let mut found = Vec::new();
    let mut docs: Vec<&Json> = chart.family("DocumentReference").iter().collect();
    docs.sort_by(|a, b| str_or(a.get("date"), "").cmp(str_or(b.get("date"), "")));
    let mut first_recorded: Vec<(String, String, Vec<String>)> = Vec::new();
    for r in chart.family("Condition") {
        let label = str_or(r.get("code").get("text"), "").to_string();
        let when = str_or(r.get("recordedDate"), "");
        let onset = str_or(r.get("onsetDateTime"), "");
        let known = if onset.is_empty() {
            when
        } else {
            when.min(onset)
        };
        if !first_recorded.iter().any(|(l, _, _)| *l == label) {
            let terms = condition_terms(&label);
            first_recorded.push((label, known.to_string(), terms));
        }
    }
    let mut first_drug: Vec<(String, String)> = Vec::new();
    for r in chart.family("MedicationRequest") {
        let text = str_or(r.get("medicationCodeableConcept").get("text"), "").to_lowercase();
        let ing = LOWER_WORD.find(&text).map_or("", |m| m.as_str());
        if ing.chars().count() > 4 {
            let authored = str_or(r.get("authoredOn"), "");
            match first_drug.iter_mut().find(|(i, _)| i == ing) {
                Some(slot) => {
                    if authored < slot.1.as_str() {
                        slot.1 = authored.to_string();
                    }
                }
                None => first_drug.push((ing.to_string(), "9999".min(authored).to_string())),
            }
        }
    }
    for doc in docs {
        let text = b64text(
            doc.get("content").as_array()[0]
                .get("attachment")
                .get("data"),
        )?
        .to_lowercase();
        let id = py_str(doc.get("id"));
        let day = head(str_or(doc.get("date"), ""), 10);
        for (_, known, terms) in &first_recorded {
            if head(known, 10) > day {
                for t in terms {
                    if !t.is_empty() && has_word(&text, t) {
                        found.push((
                            "future_condition_mention",
                            format!(
                                "future condition: {id} ({day}) mentions {}, recorded {}",
                                py_repr(t),
                                head(known, 10)
                            ),
                        ));
                        break;
                    }
                }
            }
        }
        for (ing, started) in &first_drug {
            if head(started, 10) > day && has_word(&text, ing) {
                found.push((
                    "future_drug_mention",
                    format!(
                        "future drug: {id} ({day}) mentions {}, started {}",
                        py_repr(ing),
                        head(started, 10)
                    ),
                ));
            }
        }
    }
    Ok(found)
}

// ---------------------------------------------------------------- schema

fn to_serde(v: &Json) -> serde_json::Value {
    use serde_json::Value;
    match v {
        Json::Null => Value::Null,
        Json::Bool(b) => Value::Bool(*b),
        Json::Int(i) => Value::from(*i),
        Json::Float(f) => serde_json::Number::from_f64(*f).map_or(Value::Null, Value::Number),
        Json::Str(s) => Value::String(s.clone()),
        Json::Arr(a) => Value::Array(a.iter().map(to_serde).collect()),
        Json::Obj(m) => Value::Object(m.iter().map(|(k, v)| (k.clone(), to_serde(v))).collect()),
    }
}

/// One Draft 6 validator per family: the root schema without its `oneOf`
/// resource switch, pointed at the family's definition.
fn validators(root: &Json) -> Result<(Vec<jsonschema::Validator>, Vec<Json>)> {
    let mut out = Vec::new();
    let mut subs = Vec::new();
    for fam in FAMILIES {
        let mut sub = Json::Obj(
            root.as_object()
                .iter()
                .filter(|(k, _)| k != "oneOf" && k != "discriminator")
                .cloned()
                .collect(),
        );
        sub.set("$ref", format!("#/definitions/{fam}").into());
        let v = jsonschema::draft6::options()
            .should_validate_formats(false)
            .build(&to_serde(&sub))
            .map_err(|e| format!("{fam} schema: {e}"))?;
        out.push(v);
        subs.push(sub);
    }
    Ok((out, subs))
}

// ---------------------------------------------------------------- per patient

/// What every worker shares (`init_worker`).
struct Shared {
    schema: Json,
    /// Per family: the schema each validator was built from, and the validator.
    subs: Vec<Json>,
    validators: Vec<jsonschema::Validator>,
    catalog: Json,
    renderer: Renderer,
    known: HashSet<String>,
}

/// A key for Python tuple membership: strings and None compare by value.
fn key(v: &Json) -> String {
    match v {
        Json::Null => "n".into(),
        Json::Str(s) => format!("s{s}"),
        other => format!("j{}", json::dumps(other, Layout::Compact, false)),
    }
}

fn coding_key(system: &Json, code: &Json, display: &Json) -> String {
    format!("{}\u{0}{}\u{0}{}", key(system), key(code), key(display))
}

impl Shared {
    fn new(schema: &Path, catalog: &Json, extra: &Json) -> Result<Shared> {
        let mut known = HashSet::new();
        for sec in [
            "conditions",
            "medications",
            "labs",
            "panels",
            "surveys",
            "social",
            "reports",
            "procedures",
            "allergens",
            "reactions",
        ] {
            for (_, e) in catalog.get(sec).as_object() {
                known.insert(coding_key(e.get("system"), e.get("code"), e.get("display")));
            }
        }
        for (sec, system) in [("conditions", "icd10cm"), ("medications", "rxnorm")] {
            let system = catalog.get("systems").get(system);
            for (_, e) in extra.get(sec).as_object() {
                known.insert(coding_key(system, e.get("code"), e.get("display")));
            }
        }
        let loinc = Json::from("http://loinc.org");
        let mut add = |code: &str, display: &str| {
            known.insert(coding_key(&loinc, &code.into(), &display.into()));
        };
        for (code, display, _) in [render::BP, render::SBP, render::DBP] {
            add(code, display);
        }
        for (_, (code, display, ..)) in render::VITALS {
            add(code, display);
        }
        for (_, (code, display)) in render::DOC_TYPE_CODES {
            add(code, display);
        }
        let root = json::parse(&fs::read_to_string(schema)?)?;
        let (validators, subs) = validators(&root)?;
        Ok(Shared {
            schema: root,
            subs,
            validators,
            catalog: catalog.clone(),
            renderer: Renderer::new(catalog, extra),
            known,
        })
    }
}

struct Work {
    mrn: String,
    seed: Json,
    ledger: Option<PathBuf>,
}

#[derive(Debug, Default)]
pub struct PatientReport {
    pub mrn: String,
    pub errors: Vec<String>,
    pub warnings: Counter,
    pub stats: Counter,
    pub notes: Vec<String>,
    pub external: bool,
}

impl PatientReport {
    pub fn to_json(&self) -> Json {
        let mut out = obj![
            ("mrn", self.mrn.as_str()),
            (
                "errors",
                self.errors
                    .iter()
                    .map(|e| Json::from(e.as_str()))
                    .collect::<Vec<_>>()
            ),
            ("warnings", self.warnings.to_json()),
            ("stats", self.stats.to_json()),
            (
                "notes",
                self.notes
                    .iter()
                    .map(|e| Json::from(e.as_str()))
                    .collect::<Vec<_>>()
            ),
        ];
        if self.external {
            out.set("external", Json::Bool(true));
        }
        out
    }
}

/// Where an error sits in the order Python's `iter_errors` reaches it: keywords
/// and properties in the schema file's order, array items in instance order. The
/// Rust validator visits properties alphabetically, so the first error Python
/// reports is the least of these keys.
fn python_order(root: &Json, sub: &Json, e: &jsonschema::ValidationError) -> Vec<usize> {
    let segs: Vec<String> = e.evaluation_path().iter().map(|s| s.to_string()).collect();
    let inst: Vec<String> = e.instance_path().iter().map(|s| s.to_string()).collect();
    let position = |obj: &Json, key: &str| {
        obj.as_object()
            .iter()
            .position(|(k, _)| k == key)
            .unwrap_or(usize::MAX)
    };
    let mut key = Vec::new();
    let mut cur = sub;
    let (mut i, mut at) = (0, 0);
    while let Some(kw) = segs.get(i) {
        key.push(position(cur, kw));
        i += 1;
        match kw.as_str() {
            "$ref" => {
                let target = cur.get("$ref").as_str().unwrap_or("");
                cur = target
                    .strip_prefix("#/")
                    .unwrap_or("")
                    .split('/')
                    .fold(root, |node, part| node.get(part));
            }
            "properties" => {
                let name = segs.get(i).map_or("", String::as_str);
                key.push(position(cur.get("properties"), name));
                cur = cur.get("properties").get(name);
                (i, at) = (i + 1, at + 1);
            }
            "items" if matches!(cur.get("items"), Json::Obj(_)) => {
                key.push(
                    inst.get(at)
                        .and_then(|n| n.parse().ok())
                        .unwrap_or(usize::MAX),
                );
                cur = cur.get("items");
                at += 1;
            }
            "items" | "oneOf" | "anyOf" | "allOf" => {
                let n: usize = segs
                    .get(i)
                    .and_then(|n| n.parse().ok())
                    .unwrap_or(usize::MAX);
                key.push(n);
                cur = cur.get(kw).as_array().get(n).unwrap_or(&Json::Null);
                i += 1;
                if kw == "items" {
                    at += 1;
                }
            }
            "additionalProperties" if matches!(cur.get(kw), Json::Obj(_)) => {
                cur = cur.get(kw);
                at += 1;
            }
            _ => break,
        }
    }
    key
}

/// The first schema error Python reports, as
/// `f"{e.message[:160]} at {'/'.join(e.absolute_path)}"`.
fn schema_error(sh: &Shared, slot: usize, r: &Json) -> Option<String> {
    let instance = to_serde(r);
    let errors: Vec<_> = sh.validators[slot].iter_errors(&instance).collect();
    let e = errors
        .iter()
        .min_by_key(|e| python_order(&sh.schema, &sh.subs[slot], e))?;
    let message: String = e.to_string().chars().take(160).collect();
    let path: Vec<String> = e.instance_path().iter().map(|s| s.to_string()).collect();
    Some(format!("{message} at {}", path.join("/")))
}

fn check_patient(sh: &Shared, bank: &Path, w: &Work) -> Result<PatientReport> {
    let mrn = w.mrn.as_str();
    let d = bank.join("patients").join(mrn);
    let chart = load_chart(&d)?;
    let mut out = PatientReport {
        mrn: mrn.to_string(),
        ..Default::default()
    };
    let errors = &mut out.errors;
    let stats = &mut out.stats;
    let mut ids: HashMap<String, &str> = HashMap::new();
    // ndjson + schema (lines and rows come from the same splitlines, so they always agree)
    for (slot, fam) in FAMILIES.iter().enumerate() {
        for r in &chart.0[slot] {
            stats.add(fam, 1);
            if r.get("resourceType").as_str() != Some(fam) {
                errors.push(format!("{fam}: contains {}", py_str(r.get("resourceType"))));
            }
            let id = py_str(r.get("id"));
            if !r.get("id").as_str().is_some_and(|i| ID_RE.is_match(i)) {
                errors.push(format!("{fam}/{id}: bad id"));
            }
            if ids.contains_key(&id) {
                errors.push(format!("duplicate id {id}"));
            }
            ids.insert(id.clone(), fam);
            if let Some(e) = schema_error(sh, slot, r) {
                errors.push(format!("schema {fam}/{id}: {e}"));
            }
            for k in TIME_FIELDS {
                if let Some(v) = r.get(k).as_str()
                    && !UTC_RE.is_match(v)
                    && *fam != "Patient"
                {
                    errors.push(format!("{fam}/{id}: {k} {v} not UTC +00:00"));
                }
            }
        }
    }
    if w.ledger.is_none() {
        out.external = true;
        return Ok(out);
    }
    let patient = &chart.0[0][0];
    let birth_date = str_or(patient.get("birthDate"), "");
    let birth: Date = birth_date
        .parse()
        .map_err(|e| format!("{mrn} birthDate {birth_date:?}: {e}"))?;
    // references
    for (slot, fam) in FAMILIES.iter().enumerate() {
        for r in &chart.0[slot] {
            let id = py_str(r.get("id"));
            let at = [r.get("subject"), r.get("patient")]
                .into_iter()
                .find(|v| v.truthy())
                .unwrap_or(&Json::Null);
            let reference = at.get("reference");
            if *fam != "Patient" && reference.as_str() != Some(&format!("Patient/{mrn}")) {
                errors.push(format!("{fam}/{id}: subject {}", py_str(reference)));
            }
            for k in ["result", "hasMember"] {
                for x in r.get(k).as_array() {
                    let reference = str_or(x.get("reference"), "");
                    let (t, i) = reference.split_once('/').unwrap_or((reference, ""));
                    if ids.get(i).copied() != Some(t) {
                        errors.push(format!("{fam}/{id}: {k} {reference} unresolved"));
                    }
                }
            }
        }
    }
    // terminology
    let procedures = sh.catalog.get("procedures").as_object();
    for (slot, fam) in FAMILIES.iter().enumerate() {
        for r in &chart.0[slot] {
            let id = py_str(r.get("id"));
            for field in ["code", "medicationCodeableConcept", "type"] {
                let cc = r.get(field);
                if !matches!(cc, Json::Obj(_)) {
                    continue;
                }
                let cs = cc.get("coding").as_array();
                if matches!(
                    *fam,
                    "Condition"
                        | "MedicationRequest"
                        | "Observation"
                        | "Procedure"
                        | "AllergyIntolerance"
                        | "DiagnosticReport"
                ) && field != "type"
                {
                    let kind = if cs.is_empty() { "text-only" } else { "coded" };
                    stats.add(&format!("{fam}:{kind}"), 1);
                }
                for c in cs {
                    if c.get("system").as_str() == Some(CPT) {
                        if !procedures
                            .iter()
                            .any(|(_, e)| e.get("code") == c.get("code"))
                        {
                            errors.push(format!(
                                "{fam}/{id}: CPT {} not in catalog",
                                py_str(c.get("code"))
                            ));
                        }
                        continue;
                    }
                    if !sh.known.contains(&coding_key(
                        c.get("system"),
                        c.get("code"),
                        c.get("display"),
                    )) {
                        errors.push(format!(
                            "{fam}/{id}: unverified coding {}|{}|{}",
                            py_str(c.get("system")),
                            py_str(c.get("code")),
                            py_str(c.get("display"))
                        ));
                    }
                }
            }
        }
    }
    // timeline
    for (slot, fam) in FAMILIES.iter().enumerate().skip(1) {
        for r in &chart.0[slot] {
            let id = py_str(r.get("id"));
            for k in TIME_FIELDS {
                let Some(v) = r.get(k).as_str() else {
                    continue;
                };
                if v >= AS_OF_UTC {
                    errors.push(format!("{fam}/{id}: {k} {v} not before bank time"));
                }
                if head(v, 10) < birth_date {
                    errors.push(format!("{fam}/{id}: {k} {v} before birth"));
                }
            }
            let (issued, effective) = (r.get("issued"), r.get("effectiveDateTime"));
            if issued.truthy() && effective.truthy() && str_or(issued, "") < str_or(effective, "") {
                errors.push(format!("{fam}/{id}: issued before effective"));
            }
            if *fam == "Condition" {
                let onset = str_or(r.get("onsetDateTime"), "");
                if head(onset, 10) > head(str_or(r.get("recordedDate"), "9"), 10) {
                    errors.push(format!("Condition/{id}: onset after recorded"));
                }
                let abatement = r.get("abatementDateTime");
                if abatement.truthy() && str_or(abatement, "") < onset {
                    errors.push(format!("Condition/{id}: abatement before onset"));
                }
            }
        }
    }
    // documents
    let mut docs: Vec<&Json> = chart.family("DocumentReference").iter().collect();
    docs.sort_by(|a, b| str_or(a.get("date"), "").cmp(str_or(b.get("date"), "")));
    let mut bodies: HashMap<String, String> = HashMap::new();
    for doc in docs {
        let id = py_str(doc.get("id"));
        let text = b64text(
            doc.get("content").as_array()[0]
                .get("attachment")
                .get("data"),
        )?;
        let body = text.split_once("\n\n").map_or(text.as_str(), |(_, b)| b);
        if let Some(first) = bodies.get(body) {
            errors.push(format!("DocumentReference/{id}: duplicate body of {first}"));
        }
        bodies.insert(body.to_string(), id.clone());
        let chars: Vec<char> = body.chars().collect();
        if let Some((a, b)) = search_words(&chars, &SCAFFOLD, true) {
            let word: String = chars[a..b].iter().collect();
            errors.push(format!(
                "DocumentReference/{id}: scaffold word {}",
                py_repr(&word)
            ));
        }
        let date = str_or(doc.get("date"), "");
        let on: Date = head(date, 10)
            .parse()
            .map_err(|e| format!("DocumentReference/{id} date {date:?}: {e}"))?;
        let actual = time::age_on(birth, on);
        for (at, end, stated) in age_in_text(&chars) {
            let ctx = &chars[at.saturating_sub(40)..(end + 10).min(chars.len())];
            if stated != actual && !kin(ctx) {
                let digits: String = chars[at..].iter().take_while(|c| is_decimal(**c)).collect();
                out.warnings.add("note_age_mismatch", 1);
                out.notes.push(format!(
                    "age: {id} says {digits}, is {actual} ({})",
                    head(date, 10)
                ));
            }
        }
    }
    // repeated vital tuples
    let mut tuples: Vec<((u64, u64), i64)> = Vec::new();
    for r in chart.family("Observation") {
        let coding = r.get("code").get("coding").as_array();
        if coding
            .first()
            .is_some_and(|c| c.get("code").as_str() == Some("85354-9"))
        {
            let value = |i: usize| {
                let v = r
                    .get("component")
                    .as_array()
                    .get(i)
                    .unwrap_or(&Json::Null)
                    .get("valueQuantity")
                    .get("value")
                    .as_f64()
                    .unwrap_or(f64::NAN);
                (v + 0.0).to_bits() // -0.0 == 0.0, as Python's tuple keys
            };
            let t = (value(0), value(1));
            match tuples.iter_mut().find(|(k, _)| *k == t) {
                Some(slot) => slot.1 += 1,
                None => tuples.push((t, 1)),
            }
        }
    }
    for (_, n) in tuples {
        if n >= 4 {
            out.warnings.add("repeated_bp_tuple", 1);
        }
    }
    // future facts in note text
    for (kind, msg) in future_mentions(&chart)? {
        out.warnings.add(kind, 1);
        out.notes.push(msg);
    }
    // snapshots
    let ledger_path = w.ledger.as_ref().expect("authored patient");
    let led = json::parse(&fs::read_to_string(ledger_path)?)?;
    let pled = led
        .get("patients")
        .as_array()
        .iter()
        .find(|p| p.get("mrn").as_str() == Some(mrn))
        .ok_or_else(|| format!("StopIteration: {mrn} not in {}", ledger_path.display()))?;
    let mut starts: Vec<&str> = pled
        .get("events")
        .as_array()
        .iter()
        .map(|e| str_or(e.get("start"), ""))
        .collect();
    starts.sort();
    let mut rng = PyRandom::from_text(mrn);
    let mut full_ids: HashMap<String, &Json> = HashMap::new();
    for rows in &chart.0 {
        for x in rows {
            full_ids.insert(py_str(x.get("id")), x);
        }
    }
    let mut differs = 0;
    if starts.len() >= 3 {
        for _ in 0..3 {
            let cut = starts[rng.randint(1, starts.len() as i64 - 1) as usize];
            let cut_utc =
                time::stamp_to_utc(cut).ok_or_else(|| format!("{mrn}: bad start {cut:?}"))?;
            let (snap, _) = sh.renderer.render(&w.seed, pled, cut);
            for (fam, rows) in snap.iter() {
                for x in rows {
                    let id = py_str(x.get("id"));
                    let Some(full) = full_ids.get(&id) else {
                        errors.push(format!("snapshot {cut}: {fam}/{id} absent from full chart"));
                        continue;
                    };
                    for k in TIME_FIELDS {
                        if let Some(v) = x.get(k).as_str()
                            && fam != "Patient"
                            && v >= cut_utc.as_str()
                        {
                            errors.push(format!(
                                "snapshot {cut}: {fam}/{id} {k} {v} not before cutoff"
                            ));
                        }
                    }
                    if fam == "MedicationRequest" && x.get("status") != full.get("status") {
                        differs += 1;
                        if x.get("status").as_str() == Some("stopped") {
                            errors.push(format!("snapshot {cut}: {id} stopped before its stop"));
                        }
                    }
                    if fam == "Condition"
                        && x.get("abatementDateTime") != full.get("abatementDateTime")
                    {
                        differs += 1;
                        if x.get("abatementDateTime").truthy() {
                            errors
                                .push(format!("snapshot {cut}: {id} abated before its abatement"));
                        }
                    }
                    if matches!(fam, "Observation" | "DiagnosticReport")
                        && x.get("status") != full.get("status")
                    {
                        differs += 1;
                    }
                }
            }
            stats.add("snapshots", 1);
        }
    }
    stats.set("snapshot_status_differences", differs);
    out.notes.truncate(50);
    Ok(out)
}

/// `KIN.search(ctx)`: a kin or past-age phrase near a stated age.
fn kin(ctx: &[char]) -> bool {
    search_words(ctx, KIN, true).is_some()
        || (0..ctx.len()).any(|at| boundary(ctx, at) && match_at(ctx, at, "grand", true).is_some())
}

/// Check every patient on `jobs` threads; results keep the manifest's order.
fn check_all(sh: &Shared, bank: &Path, work: &[Work], jobs: usize) -> Result<Vec<PatientReport>> {
    let next = AtomicUsize::new(0);
    let mut done: Vec<(usize, Result<PatientReport>)> = std::thread::scope(|s| {
        let workers: Vec<_> = (0..jobs.max(1))
            .map(|_| {
                s.spawn(|| {
                    let mut out = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        let Some(w) = work.get(i) else {
                            return out;
                        };
                        out.push((i, check_patient(sh, bank, w)));
                    }
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

// ---------------------------------------------------------------- the bank

/// Validate the bank, write the report and print its summary; the exit status is 1
/// when anything failed.
pub fn run(
    bank: &Path,
    seeds: &Path,
    schema: &Path,
    jobs: usize,
    report_path: Option<&Path>,
) -> Result<i32> {
    let catalog = render::load_catalog(&paths::catalog());
    let manifest = json::parse(&fs::read_to_string(bank.join("manifest.json"))?)?;
    let extra = terms::resolve_extras(&bank.join("ledgers"), &catalog, false)?;
    let seeds = json::parse(&fs::read_to_string(seeds)?)?;
    let mut by_mrn: HashMap<String, (&Json, String)> = HashMap::new();
    for u in seeds.get("units").as_array() {
        for m in u.get("members").as_array() {
            by_mrn.insert(py_str(m.get("mrn")), (m, py_str(u.get("unit"))));
        }
    }
    let mut work = Vec::new();
    for p in manifest.get("patients").as_array() {
        let mrn = py_str(p.get("mrn"));
        if p.get("unit").as_str() == Some("roger") {
            work.push(Work {
                mrn,
                seed: Json::Null,
                ledger: None,
            });
        } else {
            let (seed, unit) = by_mrn
                .get(&mrn)
                .ok_or_else(|| format!("KeyError: {mrn} not in the seeds"))?;
            work.push(Work {
                mrn,
                seed: (*seed).clone(),
                ledger: Some(bank.join("ledgers").join(format!("{unit}.json"))),
            });
        }
    }
    let shared = Shared::new(schema, &catalog, &extra)?;
    let results = check_all(&shared, bank, &work, jobs)?;
    let errors: usize = results.iter().map(|r| r.errors.len()).sum();
    let mut warn = Counter::default();
    let mut stats = Counter::default();
    for r in &results {
        warn.update(&r.warnings);
        stats.update(&r.stats);
    }
    // panel checks
    let mut panel_errors: Vec<String> = Vec::new();
    let mut sizes: Vec<(String, Json)> = Vec::new();
    let mut prev: Option<HashSet<String>> = None;
    let mrns: HashSet<String> = manifest
        .get("patients")
        .as_array()
        .iter()
        .map(|p| py_str(p.get("mrn")))
        .collect();
    let fam_of: Vec<(String, &Json)> = manifest
        .get("patients")
        .as_array()
        .iter()
        .map(|p| (py_str(p.get("mrn")), p.get("family_id")))
        .collect();
    let mut files: Vec<(i64, PathBuf, String)> = Vec::new();
    for entry in fs::read_dir(bank.join("panels"))? {
        let path = entry?.path();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if let Some(stem) = name
            .strip_prefix("panel-")
            .and_then(|n| n.strip_suffix(".json"))
        {
            let size: i64 = stem
                .split('-')
                .next()
                .unwrap_or("")
                .parse()
                .map_err(|_| format!("ValueError: panel file {name}"))?;
            files.push((size, path.clone(), format!("panel-{stem}")));
        }
    }
    files.sort_by_key(|(size, _, _)| *size);
    for (_, path, stem) in files {
        let pan = json::parse(&fs::read_to_string(&path)?)?;
        let mem: HashSet<String> = pan.get("members").as_array().iter().map(py_str).collect();
        sizes.push((stem.clone(), Json::Int(mem.len() as i64)));
        if pan.get("requested").as_i64() != Some(mem.len() as i64) {
            panel_errors.push(format!(
                "{stem}: {} members, requested {}",
                mem.len(),
                py_str(pan.get("requested"))
            ));
        }
        if !mem.is_subset(&mrns) {
            panel_errors.push(format!("{stem}: unknown members"));
        }
        if let Some(prev) = &prev
            && !prev.is_subset(&mem)
        {
            panel_errors.push(format!("{stem}: does not contain the smaller panel"));
        }
        // Python walks a set here, in hash order; sorted is the stable reading of it.
        let fids: BTreeSet<String> = fam_of
            .iter()
            .filter(|(m, f)| mem.contains(m) && f.truthy())
            .map(|(_, f)| py_str(f))
            .collect();
        for fid in fids {
            let together = fam_of
                .iter()
                .filter(|(_, f)| f.as_str() == Some(fid.as_str()))
                .all(|(m, _)| mem.contains(m));
            if !together {
                panel_errors.push(format!("{stem}: splits {fid}"));
            }
        }
        prev = Some(mem);
    }
    let failures: Vec<Json> = results
        .iter()
        .filter(|r| !r.errors.is_empty())
        .take(200)
        .map(PatientReport::to_json)
        .collect();
    let review_notes = Json::Obj(
        results
            .iter()
            .filter(|r| !r.notes.is_empty())
            .map(|r| {
                (
                    r.mrn.clone(),
                    Json::Arr(r.notes.iter().map(|n| Json::from(n.as_str())).collect()),
                )
            })
            .collect(),
    );
    let strings = |v: &[String]| Json::Arr(v.iter().map(|s| Json::from(s.as_str())).collect());
    let total_errors = (errors + panel_errors.len()) as i64;
    let report = obj![
        ("bank", pathlib_str(bank)),
        ("patients", results.len() as i64),
        (
            "patients_with_errors",
            results.iter().filter(|r| !r.errors.is_empty()).count() as i64
        ),
        ("errors", total_errors),
        ("panel_errors", strings(&panel_errors)),
        ("panels", Json::Obj(sizes)),
        ("warnings", warn.to_json()),
        ("stats", stats.to_json()),
        (
            "terminology_extras",
            obj![
                (
                    "conditions",
                    extra.get("conditions").as_object().len() as i64
                ),
                (
                    "medications",
                    extra.get("medications").as_object().len() as i64
                ),
                ("rejected", extra.get("rejected").clone()),
            ]
        ),
        ("failures", failures),
        ("review_notes", review_notes),
    ];
    let out = report_path.map_or_else(
        || {
            bank.parent()
                .unwrap_or(Path::new(""))
                .join("validation.json")
        },
        Path::to_path_buf,
    );
    fs::write(&out, json::dumps(&report, Layout::Indent(1), false) + "\n")?;
    let summary = Json::Obj(
        [
            "patients",
            "patients_with_errors",
            "errors",
            "panels",
            "warnings",
            "stats",
            "panel_errors",
        ]
        .iter()
        .map(|k| (k.to_string(), report.get(k).clone()))
        .collect(),
    );
    println!("{}", json::dumps(&summary, Layout::Indent(1), true));
    println!("report: {}", pathlib_str(&out));
    Ok(i32::from(total_errors > 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The first error Python `jsonschema` 4.26 (`Draft6Validator`) reports for each
    /// instance of a small FHIR-shaped schema whose property order is not alphabetical.
    #[test]
    fn first_schema_error_is_pythons() {
        let root = json::parse(
            r##"{"$schema": "http://json-schema.org/draft-06/schema#", "definitions": {
             "R": {"properties": {"status": {"enum": ["a"]},
                                  "category": {"type": "array", "items": {"$ref": "#/definitions/C"}},
                                  "id": {"$ref": "#/definitions/id"}},
                   "additionalProperties": false},
             "C": {"properties": {"text": {"type": "string"}, "code": {"pattern": "^x$", "type": "string"}}},
             "id": {"pattern": "^[a-z]+$", "type": "string"}}}"##,
        )
        .unwrap();
        let mut sub = root.clone();
        sub.set("$ref", "#/definitions/R".into());
        let v = jsonschema::draft6::options()
            .should_validate_formats(false)
            .build(&to_serde(&sub))
            .unwrap();
        for (instance, path) in [
            (
                r#"{"category": [{"code": "y", "text": 1}], "status": "b", "zzz": 1, "id": "A"}"#,
                "status",
            ),
            (
                r#"{"category": [{"code": "y"}, {"code": "y", "text": 1}], "zzz": 1, "id": "A"}"#,
                "category/0/code",
            ),
            (r#"{"zzz": 1, "id": 5}"#, "id"),
            (r#"{"zzz": 1}"#, ""),
        ] {
            let instance = to_serde(&json::parse(instance).unwrap());
            let errors: Vec<_> = v.iter_errors(&instance).collect();
            let first = errors
                .iter()
                .min_by_key(|e| python_order(&root, &sub, e))
                .unwrap();
            let got: Vec<String> = first
                .instance_path()
                .iter()
                .map(|s| s.to_string())
                .collect();
            assert_eq!(got.join("/"), path);
        }
    }
}
