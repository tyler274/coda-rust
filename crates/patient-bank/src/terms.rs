//! Verify out-of-catalog conditions and drugs that authors proposed.
//!
//! A proposed ICD-10-CM code is accepted only when NLM Clinical Tables returns it;
//! its official display replaces the author's. A drug written as text is accepted
//! only when RxNav resolves it to a clinical or branded drug whose name contains
//! the same ingredient word and every strength number in the text. Anything else
//! stays text-only and is counted as uncoded. Lookups share the catalog cache.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::catalog::{Lookup, Result, py_repr, shared_cache, urlencode};
use crate::json::{self, Json};
use crate::obj;

static NUMBER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d+(?:\.\d+)?").expect("valid regex"));
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z]+").expect("valid regex"));

fn numbers(text: &str) -> BTreeSet<&str> {
    NUMBER
        .find_iter(text)
        .map(|m| {
            let n = m.as_str();
            if n.contains('.') {
                n.trim_end_matches('0').trim_end_matches('.')
            } else {
                n
            }
        })
        .collect()
}

/// Python's `str.strip()`: `str.isspace` is Unicode White_Space plus the
/// information separators U+001C..U+001F, which Rust's `trim` keeps.
pub fn py_strip(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c))
}

/// Python's `str()` of the JSON scalars a ledger field can hold.
fn py_str(v: &Json) -> Result<String> {
    Ok(match v {
        Json::Null => "None".into(),
        Json::Str(s) => s.clone(),
        Json::Int(i) => i.to_string(),
        Json::Float(f) if f.is_nan() => "nan".into(),
        Json::Float(f) if f.is_infinite() => if *f > 0.0 { "inf" } else { "-inf" }.into(),
        Json::Float(f) => json::float_repr(*f),
        Json::Bool(b) => if *b { "True" } else { "False" }.into(),
        _ => return Err(format!("unexpected ledger value {v:?}").into()),
    })
}

pub fn resolve_drug(look: &mut Lookup, text: &str) -> Result<Option<Json>> {
    let url = format!(
        "https://rxnav.nlm.nih.gov/REST/approximateTerm.json?{}",
        urlencode(&[("term", text), ("maxEntries", "5")])
    );
    let Ok(found) = look.get(&url, "application/json") else {
        return Ok(None);
    };
    let lower = text.to_lowercase();
    let Some(first) = WORD.find(&lower).map(|m| m.as_str()) else {
        return Ok(None);
    };
    for c in found.get("approximateGroup").get("candidate").as_array() {
        let rxcui = c.get("rxcui");
        if !rxcui.truthy() {
            continue;
        }
        let url = format!(
            "https://rxnav.nlm.nih.gov/REST/rxcui/{}/properties.json",
            py_str(rxcui)?
        );
        let props = look.get(&url, "application/json")?;
        let props = props.get("properties");
        if !matches!(
            props.get("tty").as_str(),
            Some("SCD" | "SBD" | "GPCK" | "BPCK")
        ) {
            continue;
        }
        let name = props.get("name").as_str().unwrap_or("");
        if name.to_lowercase().contains(first) && numbers(text).is_subset(&numbers(name)) {
            return Ok(Some(obj![
                ("code", rxcui.clone()),
                ("display", name),
                ("ingredient", first),
                ("receipt", "NLM RxNav approximateTerm + properties")
            ]));
        }
    }
    Ok(None)
}

/// `resolve_extras_with` against the shared terminology cache.
pub fn resolve_extras(ledgers: &Path, catalog: &Json, offline: bool) -> Result<Json> {
    resolve_extras_with(&mut Lookup::new(shared_cache(), offline)?, ledgers, catalog)
}

pub fn resolve_extras_with(look: &mut Lookup, ledgers: &Path, catalog: &Json) -> Result<Json> {
    let mut conditions = Json::obj();
    let mut medications = Json::obj();
    let mut rejected: Vec<Json> = Vec::new();
    let mut paths: Vec<String> = Vec::new();
    for entry in fs::read_dir(ledgers)? {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if name.ends_with(".json") {
            paths.push(name);
        }
    }
    paths.sort();
    for name in paths {
        let led = json::parse(&fs::read_to_string(ledgers.join(&name))?)?;
        for p in led.get("patients").as_array() {
            for ev in p.get("events").as_array() {
                for c in ev.get("conditions").as_array() {
                    let (label, code) = (c.get("label"), c.get("icd10"));
                    let known = label
                        .as_str()
                        .is_some_and(|l| catalog.get("conditions").contains(l));
                    if known || !code.truthy() {
                        continue;
                    }
                    let code = code.as_str().ok_or("icd10 is not a string")?;
                    let key = format!("{}|{code}", py_str(label)?);
                    if conditions.contains(&key) {
                        continue;
                    }
                    let upper = py_strip(code).to_uppercase();
                    match look.icd10(&upper)? {
                        Some(display) if !display.is_empty() => conditions.set(
                            &key,
                            obj![
                                ("code", upper),
                                ("display", display),
                                ("receipt", "NLM Clinical Tables ICD-10-CM")
                            ],
                        ),
                        _ => {
                            let label = match label.as_str() {
                                Some(l) => py_repr(l),
                                None => py_str(label)?,
                            };
                            rejected.push(format!("condition {label} {code}").into());
                        }
                    }
                }
                for m in ev.get("medications").as_array() {
                    let text = m.get("drug_text");
                    if !text.truthy() {
                        continue;
                    }
                    let text = text.as_str().ok_or("drug_text is not a string")?;
                    if medications.contains(text) {
                        continue;
                    }
                    match resolve_drug(look, text)? {
                        Some(hit) => medications.set(text, hit),
                        None => rejected.push(format!("drug {}", py_repr(text)).into()),
                    }
                }
            }
        }
    }
    look.save()?;
    Ok(obj![
        ("conditions", conditions),
        ("medications", medications),
        ("rejected", rejected)
    ])
}
