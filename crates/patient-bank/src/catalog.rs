//! The verified terminology catalog, and the build that verifies it.
//!
//! `data/catalog.json` holds every code authors may use by label: ICD-10-CM
//! conditions, RxNorm drugs, LOINC labs, panels, surveys, social items and reports,
//! CPT and HCPCS procedures, SNOMED allergens, reactions and order categories, and
//! CVX vaccines. `catalog` reads each candidate in `catalog_seed` back from its
//! official service and records the canonical display and a receipt; nothing is
//! guessed. Responses are kept in `data/terminology-cache.json`, so a rebuild is
//! reproducible offline. The catalog build is the only writer of that shared cache.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::catalog_seed as seed;
use crate::{Result, paths};

pub const ICD: &str = "http://hl7.org/fhir/sid/icd-10-cm";
pub const LOINC: &str = "http://loinc.org";
pub const RXNORM: &str = "http://www.nlm.nih.gov/research/umls/rxnorm";
pub const SNOMED: &str = "http://snomed.info/sct";
pub const CPT: &str = "http://www.ama-assn.org/go/cpt";
pub const HCPCS: &str = "https://www.cms.gov/Medicare/Coding/HCPCSReleaseCodeSets";
pub const CVX: &str = "http://hl7.org/fhir/sid/cvx";

const V2_0074: &[(&str, &str)] = &[
    ("RAD", "Radiology"),
    ("EC", "Electrocardiac (e.g., EKG, EEC, Holter)"),
    ("CUS", "Cardiac Ultrasound"),
    ("GE", "Genetics"),
    ("SP", "Surgical Pathology"),
    ("CP", "Cytopathology"),
    ("PF", "Pulmonary Function"),
    ("EN", "Electroneuro (EEG, EMG,EP,PSG)"),
    ("OUS", "OB Ultrasound"),
    ("AU", "Audiology"),
    ("LAB", "Laboratory"),
];

type Section = BTreeMap<String, Entry>;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub systems: BTreeMap<String, String>,
    pub conditions: Section,
    pub medications: Section,
    pub labs: Section,
    pub panels: Section,
    pub surveys: Section,
    pub social: Section,
    pub reports: Section,
    pub procedures: Section,
    pub allergens: Section,
    pub reactions: Section,
    pub vaccines: Section,
    pub order_categories: Section,
}

impl Catalog {
    pub fn load(path: &Path) -> Result<Catalog> {
        let text = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?)
    }

    /// Every lab test that belongs to no panel.
    pub fn standalone_labs(&self) -> Vec<&str> {
        self.labs
            .keys()
            .filter(|t| !self.panels.values().any(|p| p.members.contains(t)))
            .map(String::as_str)
            .collect()
    }

    pub fn sections(&self) -> [(&'static str, &Section); 12] {
        [
            ("conditions", &self.conditions),
            ("medications", &self.medications),
            ("labs", &self.labs),
            ("panels", &self.panels),
            ("surveys", &self.surveys),
            ("social", &self.social),
            ("reports", &self.reports),
            ("procedures", &self.procedures),
            ("allergens", &self.allergens),
            ("reactions", &self.reactions),
            ("vaccines", &self.vaccines),
            ("order_categories", &self.order_categories),
        ]
    }
}

/// One verified concept. Which optional fields appear depends on the section.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub system: String,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
    /// CPT: `fixture` (display from the Roger fixture) or `recalled` (no display).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingredient: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Lab reference range from coda-gym: `low`, `high`, `low_f`, `high_f`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<BTreeMap<String, Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    /// Panel members, in reporting order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
}

/// An allergen's FHIR category, or a report's v2-0074 diagnostic section.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Category {
    Text(String),
    Section { code: String, display: String },
}

impl Entry {
    /// A lab's reference range for this sex, when both ends are numbers.
    pub fn reference_range(&self, female: bool) -> Option<(f64, f64)> {
        let range = self.range.as_ref()?;
        let end = |plain: &str, f: &str| {
            let v = if female {
                range.get(f).filter(|v| !blank(v))
            } else {
                None
            };
            number(v.or_else(|| range.get(plain))?)
        };
        Some((end("low", "low_f")?, end("high", "high_f")?))
    }
}

fn blank(v: &Value) -> bool {
    v.is_null() || v.as_str() == Some("")
}

fn number(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

/// Terminology service lookups, answered from the cache when possible.
///
/// The catalog build reads and writes the shared cache. A unit's check layers its
/// own lookups file over the shared cache, read-only, so concurrent authoring
/// never writes a file another process writes.
pub struct Lookup {
    shared: BTreeMap<String, Value>,
    own: BTreeMap<String, Value>,
    save_to: PathBuf,
    offline: bool,
    dirty: bool,
    client: Option<reqwest::blocking::Client>,
}

impl Lookup {
    /// The shared cache, which `save` writes back.
    pub fn shared(offline: bool) -> Result<Lookup> {
        let path = paths::terminology_cache();
        Ok(Lookup {
            shared: BTreeMap::new(),
            own: read_cache(&path)?,
            save_to: path,
            offline,
            dirty: false,
            client: None,
        })
    }

    /// The shared cache read-only, with new answers kept in `own`.
    pub fn layered(own: &Path, offline: bool) -> Result<Lookup> {
        Ok(Lookup {
            shared: read_cache(&paths::terminology_cache())?,
            own: read_cache(own)?,
            save_to: own.to_path_buf(),
            offline,
            dirty: false,
            client: None,
        })
    }

    pub fn get(&mut self, url: &str, accept: &str) -> Result<Value> {
        if let Some(v) = self.own.get(url).or_else(|| self.shared.get(url)) {
            return Ok(v.clone());
        }
        if self.offline {
            return Err(format!("offline and not cached: {url}").into());
        }
        let client = match &self.client {
            Some(c) => c,
            None => self.client.insert(
                reqwest::blocking::Client::builder()
                    .timeout(Duration::from_secs(60))
                    .build()?,
            ),
        };
        let body = client
            .get(url)
            .header(reqwest::header::ACCEPT, accept)
            .header(reqwest::header::USER_AGENT, "coda-patient-bank")
            .send()?
            .error_for_status()?
            .bytes()?;
        let body = body.strip_prefix(b"\xef\xbb\xbf").unwrap_or(&body);
        let value: Value = serde_json::from_slice(body)?;
        self.own.insert(url.to_string(), value.clone());
        self.dirty = true;
        Ok(value)
    }

    /// Write new answers, if any, atomically.
    pub fn save(&mut self) -> Result<()> {
        if !self.dirty {
            return Ok(());
        }
        write_atomic(&self.save_to, &serde_json::to_string_pretty(&self.own)?)?;
        self.dirty = false;
        Ok(())
    }

    fn search(&mut self, api: &str, pairs: &[(&str, &str)], code: &str) -> Result<Option<String>> {
        let url = format!(
            "https://clinicaltables.nlm.nih.gov/api/{api}/v3/search?{}",
            urlencode(pairs)
        );
        Ok(search_hit(&self.get(&url, "application/json")?, code))
    }

    pub fn hcpcs(&mut self, code: &str) -> Result<Option<String>> {
        let q = [
            ("sf", "code"),
            ("terms", code),
            ("df", "code,display"),
            ("maxList", "10"),
        ];
        self.search("hcpcs", &q, code)
    }

    pub fn icd10(&mut self, code: &str) -> Result<Option<String>> {
        self.search(
            "icd10cm",
            &[("sf", "code"), ("terms", code), ("maxList", "50")],
            code,
        )
    }

    pub fn loinc(&mut self, code: &str) -> Result<Option<String>> {
        let q = [
            ("terms", code),
            ("df", "LOINC_NUM,LONG_COMMON_NAME"),
            ("maxList", "50"),
        ];
        self.search("loinc_items", &q, code)
    }

    /// Any lookup failure, offline misses included, reads as "not found".
    pub fn snomed(&mut self, code: &str) -> Option<String> {
        self.tx(SNOMED, code)
    }

    /// A code's display from tx.fhir.org; any failure reads as "not found".
    pub fn tx(&mut self, system: &str, code: &str) -> Option<String> {
        let url = format!(
            "https://tx.fhir.org/r4/CodeSystem/$lookup?{}",
            urlencode(&[("system", system), ("code", code)])
        );
        let params = self.get(&url, "application/fhir+json").ok()?;
        params["parameter"]
            .as_array()?
            .iter()
            .find(|p| p["name"] == "display")?["valueString"]
            .as_str()
            .map(String::from)
    }

    /// RxNav's clinical drugs and generic packs for an ingredient (concept properties).
    pub fn rxnorm_drugs(&mut self, ingredient: &str) -> Result<Vec<Value>> {
        let url = format!(
            "https://rxnav.nlm.nih.gov/REST/drugs.json?{}",
            urlencode(&[("name", ingredient)])
        );
        let found = self.get(&url, "application/json")?;
        let groups = found["drugGroup"]["conceptGroup"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        Ok(groups
            .into_iter()
            .filter(|g| matches!(g["tty"].as_str(), Some("SCD" | "GPCK")))
            .flat_map(|g| {
                g["conceptProperties"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default()
            })
            .collect())
    }

    /// RxNav's approximate matches for a free-text drug, best first (rxcuis).
    pub fn rxnorm_candidates(&mut self, text: &str) -> Result<Vec<String>> {
        let url = format!(
            "https://rxnav.nlm.nih.gov/REST/approximateTerm.json?{}",
            urlencode(&[("term", text), ("maxEntries", "5")])
        );
        let found = self.get(&url, "application/json")?;
        Ok(found["approximateGroup"]["candidate"]
            .as_array()
            .map(|cands| {
                cands
                    .iter()
                    .filter_map(|c| c["rxcui"].as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    /// RxNav's term type and name for a concept.
    pub fn rxnorm_properties(&mut self, rxcui: &str) -> Result<Option<(String, String)>> {
        let url = format!("https://rxnav.nlm.nih.gov/REST/rxcui/{rxcui}/properties.json");
        let props = &self.get(&url, "application/json")?["properties"];
        Ok(match (props["tty"].as_str(), props["name"].as_str()) {
            (Some(tty), Some(name)) => Some((tty.to_string(), name.to_string())),
            _ => None,
        })
    }
}

fn read_cache(path: &Path) -> Result<BTreeMap<String, Value>> {
    match fs::read_to_string(path) {
        Ok(text) => {
            Ok(serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(BTreeMap::new()),
        Err(e) => Err(e.into()),
    }
}

/// Write `text` to `path` through a temporary file in the same directory.
pub fn write_atomic(path: &Path, text: &str) -> Result<()> {
    let name = path
        .file_name()
        .ok_or("path has no file name")?
        .to_string_lossy();
    let tmp = path.with_file_name(format!(".{name}.{}.tmp", std::process::id()));
    fs::write(&tmp, text)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

/// The display of the first NLM Clinical Tables row whose code is exactly `code`.
fn search_hit(found: &Value, code: &str) -> Option<String> {
    found[3].as_array()?.iter().find(|row| row[0] == code)?[1]
        .as_str()
        .map(String::from)
}

/// Query-string encoding: unreserved characters kept, spaces as `+`.
pub fn urlencode(pairs: &[(&str, &str)]) -> String {
    let quote = |s: &str| {
        let mut out = String::new();
        for b in s.bytes() {
            match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'.' | b'-' | b'~' => {
                    out.push(b as char)
                }
                b' ' => out.push('+'),
                _ => {
                    let _ = write!(out, "%{b:02X}");
                }
            }
        }
        out
    };
    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", quote(k), quote(v)))
        .collect::<Vec<_>>()
        .join("&")
}

fn entry(system: &str, code: &str, display: &str) -> Entry {
    Entry {
        system: system.into(),
        code: code.into(),
        display: Some(display.into()),
        ..Entry::default()
    }
}

/// Verify every candidate and write `data/catalog.json`. Prints the counts and every
/// problem; returns 1 when any candidate was not confirmed.
pub fn build(roger: &Path, offline: bool) -> Result<i32> {
    let mut look = Lookup::shared(offline)?;
    let mut problems: Vec<String> = Vec::new();
    let mut c = Catalog {
        systems: [
            ("icd10cm", ICD),
            ("loinc", LOINC),
            ("rxnorm", RXNORM),
            ("snomed", SNOMED),
            ("cpt", CPT),
            ("hcpcs", HCPCS),
            ("cvx", CVX),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect(),
        ..Catalog::default()
    };

    // coda-gym's lab labels and ranges; the first row wins (blood before urine,
    // BMP before ABG), and urine rows also answer to "Urine <label>".
    let gym_rows: Vec<Value> = serde_json::from_str(&fs::read_to_string(paths::gym_lab_ranges())?)?;
    let mut gym: HashMap<String, &Value> = HashMap::new();
    for row in &gym_rows {
        let label = row["label"].as_str().ok_or("gym row without a label")?;
        gym.entry(label.to_string()).or_insert(row);
        if row["category"] == "Urine Studies" {
            gym.entry(format!("Urine {label}")).or_insert(row);
        }
    }

    for &(label, code) in seed::CONDITIONS {
        match look.icd10(code)? {
            Some(display) => {
                let e = Entry {
                    receipt: Some("NLM Clinical Tables ICD-10-CM".into()),
                    ..entry(ICD, code, &display)
                };
                c.conditions.insert(label.into(), e);
            }
            None => problems.push(format!("condition {label:?}: ICD-10-CM {code} not found")),
        }
    }

    for &(ingredient, strengths, form) in seed::MEDICATIONS {
        let scds = look.rxnorm_drugs(ingredient)?;
        let wanted: Vec<Option<&str>> = if strengths.is_empty() {
            vec![None]
        } else {
            strengths.iter().copied().map(Some).collect()
        };
        for strength in wanted {
            // The shortest exact SCD name (no packaging qualifiers) of this form and strength.
            let chosen = scds
                .iter()
                .filter_map(|d| Some((d["name"].as_str()?, d)))
                .filter(|(name, _)| name.to_lowercase().contains(&form.to_lowercase()))
                .filter(|(name, _)| {
                    strength.is_none_or(|s| format!(" {name} ").contains(&format!(" {s} ")))
                })
                .min_by(|a, b| (a.0.chars().count(), a.0).cmp(&(b.0.chars().count(), b.0)));
            let Some((name, d)) = chosen else {
                problems.push(format!(
                    "medication {ingredient} {} {form}: no SCD",
                    strength.unwrap_or("-")
                ));
                continue;
            };
            let e = Entry {
                ingredient: Some(ingredient.into()),
                receipt: Some("NLM RxNav drugs.json (SCD)".into()),
                ..entry(RXNORM, d["rxcui"].as_str().unwrap_or_default(), name)
            };
            c.medications.insert(name.into(), e);
        }
    }
    for &(ingredient, exact) in seed::EXACT_MEDICATIONS {
        let drugs = look.rxnorm_drugs(ingredient)?;
        let Some(d) = drugs.iter().find(|d| d["name"] == exact) else {
            problems.push(format!(
                "medication exact {exact:?}: not an RxNorm SCD/GPCK name"
            ));
            continue;
        };
        let e = Entry {
            ingredient: Some(ingredient.into()),
            receipt: Some("NLM RxNav drugs.json (exact name)".into()),
            ..entry(RXNORM, d["rxcui"].as_str().unwrap_or_default(), exact)
        };
        c.medications.insert(exact.into(), e);
    }

    for &(label, code, unit) in seed::LABS {
        let Some(display) = look.loinc(code)? else {
            problems.push(format!("lab {label:?}: LOINC {code} not found"));
            continue;
        };
        let mut e = Entry {
            unit: unit.map(String::from),
            receipt: Some("NLM Clinical Tables LOINC".into()),
            ..entry(LOINC, code, &display)
        };
        if let Some(g) = gym.get(label) {
            let range: BTreeMap<String, Value> = ["low", "high", "low_f", "high_f"]
                .into_iter()
                .filter(|k| !g[*k].is_null() && g[*k] != "None")
                .map(|k| (k.to_string(), g[k].clone()))
                .collect();
            e.range = Some(range);
            e.range_source = Some("coda-gym lab_reference_ranges.xlsx".into());
        }
        c.labs.insert(label.into(), e);
    }
    for &(label, code, members) in seed::PANELS {
        let display = look.loinc(code)?;
        let missing: Vec<&str> = members
            .iter()
            .copied()
            .filter(|m| !c.labs.contains_key(*m))
            .collect();
        match display {
            Some(display) if missing.is_empty() => {
                let e = Entry {
                    members: members.iter().map(|m| m.to_string()).collect(),
                    ..entry(LOINC, code, &display)
                };
                c.panels.insert(label.into(), e);
            }
            display => problems.push(format!(
                "panel {label:?}: LOINC {code} found={} missing={missing:?}",
                display.is_some()
            )),
        }
    }

    for &(label, code, unit) in seed::SURVEYS {
        match look.loinc(code)? {
            Some(display) => {
                let e = Entry {
                    unit: Some(unit.into()),
                    ..entry(LOINC, code, &display)
                };
                c.surveys.insert(label.into(), e);
            }
            None => problems.push(format!("surveys {label:?}: LOINC {code} not found")),
        }
    }
    for &(label, code) in seed::SOCIAL {
        match look.loinc(code)? {
            Some(display) => {
                c.social.insert(label.into(), entry(LOINC, code, &display));
            }
            None => problems.push(format!("social {label:?}: LOINC {code} not found")),
        }
    }

    for &(label, code, section) in seed::REPORTS {
        let Some(display) = look.loinc(code)? else {
            problems.push(format!("report {label:?}: LOINC {code} not found"));
            continue;
        };
        let (_, section_display) = V2_0074
            .iter()
            .find(|(s, _)| *s == section)
            .ok_or("unknown v2-0074 section")?;
        let e = Entry {
            category: Some(Category::Section {
                code: section.into(),
                display: (*section_display).into(),
            }),
            ..entry(LOINC, code, &display)
        };
        c.reports.insert(label.into(), e);
    }

    // CPT has no public verification service. Codes harvested from the Coda Agents
    // Roger fixture keep that fixture's display; recalled codes carry no display
    // rather than an unverified one.
    let fixture = roger.join("Procedure.ndjson");
    let mut fixture_display: HashMap<String, String> = HashMap::new();
    match fs::read_to_string(&fixture) {
        Ok(text) => {
            for line in text.lines() {
                let r: Value = serde_json::from_str(line)?;
                let coding = &r["code"]["coding"][0];
                if let (Some(code), Some(display)) =
                    (coding["code"].as_str(), coding["display"].as_str())
                {
                    fixture_display.insert(code.into(), display.into());
                }
            }
        }
        Err(_) => problems.push(format!(
            "Roger fixture not found at {}; pass --roger-fixture",
            fixture.display()
        )),
    }
    for &(label, code, prov) in seed::PROCEDURES {
        let e = match prov {
            // HCPCS Level II is verifiable: NLM Clinical Tables
            "hcpcs" => match look.hcpcs(code)? {
                Some(display) => Entry {
                    provenance: Some("NLM Clinical Tables HCPCS".into()),
                    ..entry(HCPCS, code, &display)
                },
                None => {
                    problems.push(format!("procedure {label:?}: HCPCS {code} not found"));
                    continue;
                }
            },
            "fixture" => match fixture_display.get(code) {
                Some(display) => Entry {
                    provenance: Some(prov.into()),
                    ..entry(CPT, code, display)
                },
                None => {
                    problems.push(format!(
                        "procedure {label:?}: CPT {code} not in the Roger fixture"
                    ));
                    continue;
                }
            },
            _ => Entry {
                system: CPT.into(),
                code: code.into(),
                provenance: Some(prov.into()),
                ..Entry::default()
            },
        };
        c.procedures.insert(label.into(), e);
    }

    for &(label, code, category) in seed::ALLERGENS {
        match look.snomed(code) {
            Some(display) => {
                let e = Entry {
                    category: Some(Category::Text(category.into())),
                    ..entry(SNOMED, code, &display)
                };
                c.allergens.insert(label.into(), e);
            }
            None => problems.push(format!("allergen {label:?}: SNOMED {code} not found")),
        }
    }
    for &(label, code) in seed::REACTIONS {
        match look.snomed(code) {
            Some(display) => {
                c.reactions
                    .insert(label.into(), entry(SNOMED, code, &display));
            }
            None => problems.push(format!("reaction {label:?}: SNOMED {code} not found")),
        }
    }

    for &(label, code) in seed::VACCINES {
        match look.tx(CVX, code) {
            Some(display) => {
                let e = Entry {
                    receipt: Some("tx.fhir.org CVX".into()),
                    ..entry(CVX, code, &display)
                };
                c.vaccines.insert(label.into(), e);
            }
            None => problems.push(format!("vaccine {label:?}: CVX {code} not found")),
        }
    }
    for &(label, code) in seed::ORDER_CATEGORIES {
        match look.snomed(code) {
            Some(display) => {
                c.order_categories
                    .insert(label.into(), entry(SNOMED, code, &display));
            }
            None => problems.push(format!("order category {label:?}: SNOMED {code} not found")),
        }
    }

    look.save()?;
    write_atomic(
        &paths::catalog(),
        &(serde_json::to_string_pretty(&c)? + "\n"),
    )?;
    let counts: Vec<String> = c
        .sections()
        .iter()
        .map(|(name, s)| format!("{name} {}", s.len()))
        .collect();
    println!("{}", counts.join(", "));
    for p in &problems {
        println!("PROBLEM {p}");
    }
    Ok(i32::from(!problems.is_empty()))
}
