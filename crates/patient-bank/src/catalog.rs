//! The verified terminology catalog, and the build that verifies it.
//!
//! `data/catalog.json` holds every code authors may use by label: ICD-10-CM
//! conditions, RxNorm drugs, LOINC labs, panels, surveys, social items and reports,
//! CPT and HCPCS procedures, SNOMED allergens, reactions, order categories and
//! referral specialties, and
//! CVX vaccines. `catalog` reads every entry back from its official service and
//! records the canonical display and a receipt; nothing is guessed. The catalog is
//! its own input: a concept is added by writing its label and code into it. Responses are kept in a local cache under `target/patient-bank/`, so a
//! service is asked each question once; the catalog build is that cache's only writer.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ledger::{AllergyCategory, AllergyItem, OrderCategory, OrderItem};
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
    /// Who a referral goes to: a specialty, coded as the SNOMED CT occupation that
    /// practices it.
    pub specialties: Section,
}

impl Catalog {
    /// This catalog plus the labs, studies, allergens and procedures a ledger's
    /// verified terms add.
    pub fn with_terms(&self, terms: &crate::ledger::Terms) -> Catalog {
        let mut c = self.clone();
        let system = |name: &str| c.systems.get(name).cloned().unwrap_or_default();
        let (loinc, snomed) = (system("loinc"), system("snomed"));
        let entry = |system: &str, v: &crate::ledger::VerifiedCode| Entry {
            system: system.to_string(),
            code: v.code.clone(),
            display: Some(v.display.clone()),
            receipt: Some(v.receipt.clone()),
            unit: v.unit.clone(),
            ..Entry::default()
        };
        for (section, system, verified) in [
            (&mut c.labs, &loinc, &terms.labs),
            (&mut c.reports, &loinc, &terms.reports),
            (&mut c.allergens, &snomed, &terms.allergens),
            (&mut c.procedures, &snomed, &terms.procedures),
        ] {
            for (label, v) in verified {
                section
                    .entry(label.clone())
                    .or_insert_with(|| entry(system, v));
            }
        }
        c
    }

    /// What an order orders: a lab, panel, study or procedure by its label, looked
    /// up in the order's own category first, since one label (spirometry) can be
    /// both a procedure and a study. A referral's specialty is its performer.
    pub fn orderable(&self, item: &OrderItem) -> Option<&Entry> {
        let what = item.order.as_deref()?;
        let sections = match item.category.unwrap_or(OrderCategory::Referral) {
            OrderCategory::Referral => return None,
            OrderCategory::Lab => [&self.labs, &self.panels, &self.reports, &self.procedures],
            OrderCategory::Imaging => [&self.reports, &self.labs, &self.panels, &self.procedures],
            OrderCategory::Procedure => [&self.procedures, &self.reports, &self.labs, &self.panels],
        };
        sections
            .into_iter()
            .find_map(|s| s.get(what))
            .filter(|e| e.display.is_some())
    }

    /// An allergy entry's FHIR category (the catalog's, else the entry's own), and
    /// the category it rules out when it is a "no known" entry.
    pub fn allergy_class(&self, item: &AllergyItem) -> (&str, Option<&str>) {
        let entry = self.allergens.get(&item.substance);
        let category = match entry.and_then(|e| e.category.as_ref()) {
            Some(Category::Text(c)) => c.as_str(),
            _ => item.category.map_or("medication", AllergyCategory::as_str),
        };
        (category, entry.and_then(|e| e.negates.as_deref()))
    }

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

    pub fn sections(&self) -> [(&'static str, &Section); 13] {
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
            ("specialties", &self.specialties),
        ]
    }
}

/// One verified concept. Which optional fields appear depends on the section.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    /// Set by the catalog build from the entry's section.
    #[serde(default)]
    pub system: String,
    /// A drug's is set by the catalog build from its exact name.
    #[serde(default)]
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
    /// A "no known" allergen: the allergen category it says the patient has none of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negates: Option<String>,
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
/// own lookups file in its preview folder over the shared cache, read-only, so
/// concurrent authoring never writes a file another process writes.
pub struct Lookup {
    shared: BTreeMap<String, Value>,
    own: BTreeMap<String, Value>,
    save_to: PathBuf,
    dirty: bool,
    client: Option<reqwest::blocking::Client>,
}

impl Lookup {
    /// The shared cache, which `save` writes back.
    pub fn shared() -> Result<Lookup> {
        let path = paths::terminology_cache();
        Ok(Lookup {
            shared: BTreeMap::new(),
            own: read_cache(&path)?,
            save_to: path,
            dirty: false,
            client: None,
        })
    }

    /// The shared cache read-only, with new answers kept in `own`.
    pub fn layered(own: &Path) -> Result<Lookup> {
        Ok(Lookup {
            shared: read_cache(&paths::terminology_cache())?,
            own: read_cache(own)?,
            save_to: own.to_path_buf(),
            dirty: false,
            client: None,
        })
    }

    pub fn get(&mut self, url: &str, accept: &str) -> Result<Value> {
        if let Some(v) = self.own.get(url).or_else(|| self.shared.get(url)) {
            return Ok(v.clone());
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

    /// Real codes near a rejected one, as "code display", so the author can pick:
    /// ICD-10-CM codes under the code's prefix, LOINC codes matching the label.
    /// Any lookup failure reads as no candidates.
    pub fn near(&mut self, loinc: bool, terms: &str) -> Vec<String> {
        let (api, q) = if loinc {
            let df = ("df", "LOINC_NUM,LONG_COMMON_NAME");
            ("loinc_items", [("terms", terms), df, ("maxList", "5")])
        } else {
            (
                "icd10cm",
                [("sf", "code"), ("terms", terms), ("maxList", "5")],
            )
        };
        let url = format!(
            "https://clinicaltables.nlm.nih.gov/api/{api}/v3/search?{}",
            urlencode(&q)
        );
        let Ok(found) = self.get(&url, "application/json") else {
            return Vec::new();
        };
        found[3]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|row| Some(format!("{} {}", row[0].as_str()?, row[1].as_str()?)))
            .collect()
    }

    /// Real codes whose names match `words`, as "code display", from the service
    /// `check` verifies that system against: ICD-10-CM and LOINC from NLM Clinical
    /// Tables, SNOMED CT from tx.fhir.org (with each concept's kind), RxNorm drugs
    /// from RxNav.
    pub fn find(&mut self, system: &str, words: &str) -> Result<Vec<String>> {
        let rows = |found: &Value| -> Vec<String> {
            found[3]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|r| Some(format!("{} {}", r[0].as_str()?, r[1].as_str()?)))
                .collect()
        };
        let clinical = |api: &str, pairs: &[(&str, &str)]| {
            let mut q = vec![("terms", words), ("maxList", "15")];
            q.extend_from_slice(pairs);
            format!(
                "https://clinicaltables.nlm.nih.gov/api/{api}/v3/search?{}",
                urlencode(&q)
            )
        };
        Ok(match system {
            "icd10" => {
                let q = [("sf", "code,name"), ("df", "code,name")];
                rows(&self.get(&clinical("icd10cm", &q), "application/json")?)
            }
            "loinc" => {
                let q = [("df", "LOINC_NUM,LONG_COMMON_NAME")];
                rows(&self.get(&clinical("loinc_items", &q), "application/json")?)
            }
            "snomed" => {
                let url = format!(
                    "https://tx.fhir.org/r4/ValueSet/$expand?{}",
                    urlencode(&[
                        ("url", "http://snomed.info/sct?fhir_vs"),
                        ("filter", words),
                        ("count", "15"),
                        ("includeDesignations", "true"),
                    ])
                );
                let found = self.get(&url, "application/fhir+json")?;
                let fsn = |c: &Value| -> Option<String> {
                    c["designation"].as_array()?.iter().find_map(|d| {
                        (d["use"]["code"] == "900000000000003001")
                            .then(|| d["value"].as_str().map(String::from))?
                    })
                };
                found["expansion"]["contains"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|c| {
                        let name = fsn(c).or_else(|| c["display"].as_str().map(String::from))?;
                        Some(format!("{} {name}", c["code"].as_str()?))
                    })
                    .collect()
            }
            "rxnorm" => {
                let mut out = Vec::new();
                for rxcui in self.rxnorm_candidates(words)? {
                    if let Some((tty, name)) = self.rxnorm_properties(&rxcui)?
                        && !out.iter().any(|o: &String| o.ends_with(&name))
                    {
                        out.push(format!("{rxcui} [{tty}] {name}"));
                    }
                }
                out
            }
            _ => {
                return Err(
                    format!("find takes icd10, loinc, snomed or rxnorm, not {system:?}").into(),
                );
            }
        })
    }

    /// Any lookup failure reads as "not found".
    pub fn snomed(&mut self, code: &str) -> Option<String> {
        self.tx(SNOMED, code)
    }

    /// An active SNOMED CT concept's display, when its fully specified name's
    /// semantic tag is one of `tags` (a procedure for a procedure, a substance for
    /// an allergen); any failure reads as "not found".
    pub fn snomed_of(&mut self, code: &str, tags: &[&str]) -> Option<String> {
        let url = format!(
            "https://tx.fhir.org/r4/CodeSystem/$lookup?{}",
            urlencode(&[("system", SNOMED), ("code", code)])
        );
        let params = self.get(&url, "application/fhir+json").ok()?;
        let params = params["parameter"].as_array()?;
        let part = |p: &Value, name: &str| -> Option<Value> {
            p["part"]
                .as_array()?
                .iter()
                .find(|x| x["name"] == name)
                .cloned()
        };
        let inactive = params.iter().any(|p| {
            p["name"] == "property"
                && part(p, "code").is_some_and(|c| c["valueCode"] == "inactive")
                && part(p, "value").is_some_and(|v| v["valueBoolean"] == true)
        });
        let fsn = params.iter().find_map(|p| {
            let fully = part(p, "use")?["valueCoding"]["code"] == "900000000000003001";
            (p["name"] == "designation" && fully)
                .then(|| part(p, "value")?["valueString"].as_str().map(String::from))?
        })?;
        let tag = fsn.rsplit_once('(')?.1.trim_end_matches(')');
        if inactive || !tags.contains(&tag) {
            return None;
        }
        params.iter().find(|p| p["name"] == "display")?["valueString"]
            .as_str()
            .map(String::from)
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

    /// The clinical and branded drugs RxNav relates to a concept (a drug component
    /// such as "penicillin V potassium 250 MG" relates to its tablets and capsules).
    pub fn rxnorm_related_drugs(&mut self, rxcui: &str) -> Result<Vec<(String, String)>> {
        let url = format!("https://rxnav.nlm.nih.gov/REST/rxcui/{rxcui}/related.json?tty=SCD+SBD");
        let found = self.get(&url, "application/json")?;
        Ok(found["relatedGroup"]["conceptGroup"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(|g| {
                g["conceptProperties"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default()
            })
            .filter_map(|c| {
                Some((
                    c["rxcui"].as_str()?.to_string(),
                    c["name"].as_str()?.to_string(),
                ))
            })
            .collect())
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

/// Record a service's display for `e`, or the problem when it has none.
fn confirm(problems: &mut Vec<String>, what: &str, label: &str, e: &mut Entry, d: Option<String>) {
    match d {
        Some(display) => e.display = Some(display),
        None => problems.push(format!(
            "{what} {label:?}: {} {} not found",
            e.system, e.code
        )),
    }
}

/// Verify every entry of `data/catalog.json` against its official service again and
/// write it back with the services' displays. A concept is added by writing its
/// label and code (a drug: its exact RxNorm name and ingredient; a report: its
/// v2-0074 section; a procedure: `hcpcs`, `fixture` or `recalled`) and running
/// this. Prints the counts and every problem; when any entry is not confirmed it
/// writes nothing and returns 1.
pub fn build(roger: &Path) -> Result<i32> {
    let mut look = Lookup::shared()?;
    let mut problems: Vec<String> = Vec::new();
    let mut c = Catalog::load(&paths::catalog())?;
    c.systems = [
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
    .collect();

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

    for (label, e) in &mut c.conditions {
        e.system = ICD.into();
        e.receipt = Some("NLM Clinical Tables ICD-10-CM".into());
        let d = look.icd10(&e.code)?;
        confirm(&mut problems, "condition", label, e, d);
    }

    // a drug is its exact RxNorm name, found among its ingredient's drugs
    for (label, e) in &mut c.medications {
        let Some(ingredient) = e.ingredient.clone() else {
            problems.push(format!("medication {label:?}: no ingredient"));
            continue;
        };
        let drugs = look.rxnorm_drugs(&ingredient)?;
        let Some(d) = drugs.iter().find(|d| d["name"] == label.as_str()) else {
            problems.push(format!(
                "medication {label:?}: not an RxNorm drug of {ingredient}"
            ));
            continue;
        };
        e.system = RXNORM.into();
        e.code = d["rxcui"].as_str().unwrap_or_default().into();
        e.display = Some(label.clone());
        e.receipt
            .get_or_insert_with(|| "NLM RxNav drugs.json (exact name)".into());
    }

    for (label, e) in &mut c.labs {
        e.system = LOINC.into();
        e.receipt = Some("NLM Clinical Tables LOINC".into());
        let d = look.loinc(&e.code)?;
        confirm(&mut problems, "lab", label, e, d);
        if let Some(g) = gym.get(label.as_str()) {
            let range: BTreeMap<String, Value> = ["low", "high", "low_f", "high_f"]
                .into_iter()
                .filter(|k| !g[*k].is_null() && g[*k] != "None")
                .map(|k| (k.to_string(), g[k].clone()))
                .collect();
            e.range = Some(range);
            e.range_source = Some("coda-gym lab_reference_ranges.xlsx".into());
        }
    }
    for (label, e) in &mut c.panels {
        e.system = LOINC.into();
        let missing: Vec<&String> = (e.members.iter())
            .filter(|m| !c.labs.contains_key(*m))
            .collect();
        if !missing.is_empty() {
            problems.push(format!("panel {label:?}: members not in labs {missing:?}"));
        }
        let d = look.loinc(&e.code)?;
        confirm(&mut problems, "panel", label, e, d);
    }
    for (what, section) in [("survey", &mut c.surveys), ("social", &mut c.social)] {
        for (label, e) in section {
            e.system = LOINC.into();
            let d = look.loinc(&e.code)?;
            confirm(&mut problems, what, label, e, d);
        }
    }
    for (label, e) in &mut c.reports {
        e.system = LOINC.into();
        let d = look.loinc(&e.code)?;
        confirm(&mut problems, "report", label, e, d);
        let section = match &e.category {
            Some(Category::Section { code, .. }) => code.clone(),
            Some(Category::Text(code)) => code.clone(),
            None => String::new(),
        };
        match V2_0074.iter().find(|(s, _)| *s == section) {
            Some((code, display)) => {
                e.category = Some(Category::Section {
                    code: (*code).into(),
                    display: (*display).into(),
                });
            }
            None => problems.push(format!("report {label:?}: no v2-0074 section {section:?}")),
        }
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
    for (label, e) in &mut c.procedures {
        match e.provenance.as_deref() {
            // HCPCS Level II is verifiable: NLM Clinical Tables
            Some("hcpcs" | "NLM Clinical Tables HCPCS") => {
                e.system = HCPCS.into();
                e.provenance = Some("NLM Clinical Tables HCPCS".into());
                let d = look.hcpcs(&e.code)?;
                confirm(&mut problems, "procedure", label, e, d);
            }
            Some("fixture") => {
                e.system = CPT.into();
                let d = fixture_display.get(&e.code).cloned();
                confirm(&mut problems, "procedure", label, e, d);
            }
            Some("recalled") => {
                e.system = CPT.into();
                e.display = None;
            }
            other => problems.push(format!(
                "procedure {label:?}: provenance {other:?} is not hcpcs, fixture or recalled"
            )),
        }
    }

    for (what, section) in [
        ("allergen", &mut c.allergens),
        ("reaction", &mut c.reactions),
        ("order category", &mut c.order_categories),
    ] {
        for (label, e) in section {
            e.system = SNOMED.into();
            let d = look.snomed(&e.code);
            confirm(&mut problems, what, label, e, d);
        }
    }
    for (label, e) in &mut c.specialties {
        e.system = SNOMED.into();
        let d = look.snomed_of(&e.code, &["occupation"]);
        confirm(&mut problems, "specialty", label, e, d);
    }
    for (label, e) in &mut c.vaccines {
        e.system = CVX.into();
        e.receipt = Some("tx.fhir.org CVX".into());
        let d = look.tx(CVX, &e.code);
        confirm(&mut problems, "vaccine", label, e, d);
    }

    look.save()?;
    let counts: Vec<String> = c
        .sections()
        .iter()
        .map(|(name, s)| format!("{name} {}", s.len()))
        .collect();
    println!("{}", counts.join(", "));
    for p in &problems {
        println!("PROBLEM {p}");
    }
    if problems.is_empty() {
        write_atomic(
            &paths::catalog(),
            &(serde_json::to_string_pretty(&c)? + "\n"),
        )?;
    }
    Ok(i32::from(!problems.is_empty()))
}
