//! Verify the candidate terminology and write the authoring catalog.
//!
//! Reads `catalog_seed`, looks every code up in its official service, and writes
//! `catalog.json`: the canonical display, the clean label used as `code.text`, and a
//! receipt naming the service. Responses are cached in `terminology-cache.json` so a
//! rebuild is reproducible offline. Codes the service does not confirm are reported
//! and left out; nothing is guessed.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::time::Duration;

use regex::Regex;

use crate::catalog_seed as seed;
use crate::json::{self, Json, Layout};
use crate::obj;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub const ICD: &str = "http://hl7.org/fhir/sid/icd-10-cm";
pub const LOINC: &str = "http://loinc.org";
pub const RXNORM: &str = "http://www.nlm.nih.gov/research/umls/rxnorm";
pub const SNOMED: &str = "http://snomed.info/sct";
pub const CPT: &str = "http://www.ama-assn.org/go/cpt";
pub const HCPCS: &str = "https://www.cms.gov/Medicare/Coding/HCPCSReleaseCodeSets";

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

/// The Coda Agents Roger fixture's resources, relative to the repository root.
pub const ROGER_FIXTURE: &str =
    "submodules/coda-agents/docs/local-product-fixtures/internal_resources";

/// `scripts/patient-bank` in this checkout, where the shared terminology cache,
/// the coda-gym lab ranges and the catalog live beside the Python pipeline.
pub fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/patient-bank")
}

/// The shared terminology cache that authoring agents and builds all read and extend.
pub fn shared_cache() -> PathBuf {
    data_dir().join("terminology-cache.json")
}

/// Terminology service lookups, answered from the cache file when possible.
pub struct Lookup {
    path: PathBuf,
    offline: bool,
    cache: BTreeMap<String, Json>,
    client: Option<reqwest::blocking::Client>,
}

impl Lookup {
    /// Load the cache at `path` (empty when the file does not exist).
    pub fn new(path: impl Into<PathBuf>, offline: bool) -> Result<Lookup> {
        let path = path.into();
        let mut cache = BTreeMap::new();
        if path.exists() {
            cache = into_map(json::parse(&fs::read_to_string(&path)?)?)?;
        }
        Ok(Lookup {
            path,
            offline,
            cache,
            client: None,
        })
    }

    pub fn get(&mut self, url: &str, accept: &str) -> Result<Json> {
        if let Some(v) = self.cache.get(url) {
            return Ok(v.clone());
        }
        if self.offline {
            return Err(format!("offline and not cached: {url}").into());
        }
        if self.client.is_none() {
            self.client = Some(
                reqwest::blocking::Client::builder()
                    .timeout(Duration::from_secs(60))
                    .build()?,
            );
        }
        let client = self.client.as_ref().expect("client built above");
        let body = client
            .get(url)
            .header(reqwest::header::ACCEPT, accept)
            .header(reqwest::header::USER_AGENT, "coda-patient-bank")
            .send()?
            .error_for_status()?
            .bytes()?;
        // json.load reads bytes, so a UTF-8 byte-order mark is dropped.
        let body = body.strip_prefix(b"\xef\xbb\xbf").unwrap_or(&body);
        let value = json::parse(std::str::from_utf8(body)?)?;
        self.cache.insert(url.to_string(), value.clone());
        Ok(value)
    }

    pub fn save(&mut self) -> Result<()> {
        // authoring agents share the cache: keep their entries, never leave a partial file
        match fs::read_to_string(&self.path) {
            Ok(text) => {
                if let Ok(disk) = json::parse(&text) {
                    for (url, value) in into_map(disk)? {
                        self.cache.entry(url).or_insert(value);
                    }
                }
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
        let all = Json::Obj(
            self.cache
                .iter()
                .map(|(url, value)| (url.clone(), sort_keys(value)))
                .collect(),
        );
        let name = self
            .path
            .file_name()
            .ok_or("cache path has no file name")?
            .to_string_lossy();
        let tmp = self
            .path
            .with_file_name(format!("{name}.{}.tmp", std::process::id()));
        fs::write(&tmp, json::dumps(&all, Layout::Indent(0), true))?;
        fs::rename(&tmp, &self.path)?;
        Ok(())
    }

    pub fn hcpcs(&mut self, code: &str) -> Result<Option<String>> {
        let url = format!(
            "https://clinicaltables.nlm.nih.gov/api/hcpcs/v3/search?{}",
            urlencode(&[
                ("sf", "code"),
                ("terms", code),
                ("df", "code,display"),
                ("maxList", "10")
            ])
        );
        Ok(search_hit(&self.get(&url, "application/json")?, code))
    }

    pub fn icd10(&mut self, code: &str) -> Result<Option<String>> {
        let url = format!(
            "https://clinicaltables.nlm.nih.gov/api/icd10cm/v3/search?{}",
            urlencode(&[("sf", "code"), ("terms", code), ("maxList", "50")])
        );
        Ok(search_hit(&self.get(&url, "application/json")?, code))
    }

    pub fn loinc(&mut self, code: &str) -> Result<Option<String>> {
        let url = format!(
            "https://clinicaltables.nlm.nih.gov/api/loinc_items/v3/search?{}",
            urlencode(&[
                ("terms", code),
                ("df", "LOINC_NUM,LONG_COMMON_NAME"),
                ("maxList", "50")
            ])
        );
        Ok(search_hit(&self.get(&url, "application/json")?, code))
    }

    /// Any lookup failure, offline misses included, reads as "not found".
    pub fn snomed(&mut self, code: &str) -> Option<String> {
        let url = format!(
            "https://tx.fhir.org/r4/CodeSystem/$lookup?{}",
            urlencode(&[("system", SNOMED), ("code", code)])
        );
        let params = self.get(&url, "application/fhir+json").ok()?;
        for p in params.get("parameter").as_array() {
            if p.get("name").as_str() == Some("display") {
                return p.get("valueString").as_str().map(String::from);
            }
        }
        None
    }

    /// RxNav's clinical drugs and generic packs for an ingredient (concept properties).
    pub fn rxnorm_drugs(&mut self, ingredient: &str) -> Result<Vec<Json>> {
        let url = format!(
            "https://rxnav.nlm.nih.gov/REST/drugs.json?{}",
            urlencode(&[("name", ingredient)])
        );
        let found = self.get(&url, "application/json")?;
        let groups = found.get("drugGroup").get("conceptGroup").as_array();
        Ok(groups
            .iter()
            .filter(|g| matches!(g.get("tty").as_str(), Some("SCD" | "GPCK")))
            .flat_map(|g| g.get("conceptProperties").as_array().iter().cloned())
            .collect())
    }
}

/// The display of the NLM Clinical Tables row whose code is exactly `code`.
fn search_hit(found: &Json, code: &str) -> Option<String> {
    found.as_array().get(3)?.as_array().iter().find_map(|row| {
        let row = row.as_array();
        (row.first()?.as_str()? == code).then(|| row.get(1)?.as_str().map(String::from))?
    })
}

fn into_map(v: Json) -> Result<BTreeMap<String, Json>> {
    match v {
        Json::Obj(members) => Ok(members.into_iter().collect()),
        _ => Err("terminology cache is not a JSON object".into()),
    }
}

/// The value with every object's keys sorted, as `json.dumps(sort_keys=True)` writes it.
pub fn sort_keys(v: &Json) -> Json {
    match v {
        Json::Obj(members) => {
            let mut members: Vec<(String, Json)> = members
                .iter()
                .map(|(k, v)| (k.clone(), sort_keys(v)))
                .collect();
            members.sort_by(|a, b| a.0.cmp(&b.0));
            Json::Obj(members)
        }
        Json::Arr(items) => Json::Arr(items.iter().map(sort_keys).collect()),
        other => other.clone(),
    }
}

/// `urllib.parse.urlencode`: `quote_plus` on every key and value.
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

/// Python's `repr()` of a string, as f-strings with `!r` write it.
pub fn py_repr(s: &str) -> String {
    // str.isprintable() is false for the Other and Separator categories (space aside).
    static UNPRINTABLE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[\p{C}\p{Z}]$").expect("valid regex"));
    let quote = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::new();
    out.push(quote);
    let mut buf = [0u8; 4];
    for c in s.chars() {
        let n = c as u32;
        match c {
            _ if c == quote || c == '\\' => {
                out.push('\\');
                out.push(c);
            }
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            _ if n < 0x20 || n == 0x7f => {
                let _ = write!(out, "\\x{n:02x}");
            }
            _ if n < 0x7f => out.push(c),
            _ if !UNPRINTABLE.is_match(c.encode_utf8(&mut buf)) => out.push(c),
            _ if n <= 0xff => {
                let _ = write!(out, "\\x{n:02x}");
            }
            _ if n <= 0xffff => {
                let _ = write!(out, "\\u{n:04x}");
            }
            _ => {
                let _ = write!(out, "\\U{n:08x}");
            }
        }
    }
    out.push(quote);
    out
}

fn opt(s: Option<&str>) -> Json {
    s.map_or(Json::Null, Json::from)
}

/// `build_with` against the shared cache and this checkout's coda-gym ranges.
pub fn build(roger: &Path, offline: bool, out: &Path) -> Result<i32> {
    let mut look = Lookup::new(shared_cache(), offline)?;
    build_with(
        &mut look,
        &data_dir().join("gym-lab-ranges.json"),
        roger,
        out,
    )
}

/// Write the verified catalog to `out`, print the counts and every problem, and
/// return the exit status: 1 when any candidate was not confirmed.
pub fn build_with(look: &mut Lookup, gym_labs: &Path, roger: &Path, out: &Path) -> Result<i32> {
    let mut problems: Vec<String> = Vec::new();
    let mut catalog = obj![(
        "systems",
        obj![
            ("icd10cm", ICD),
            ("loinc", LOINC),
            ("rxnorm", RXNORM),
            ("snomed", SNOMED),
            ("cpt", CPT),
            ("hcpcs", HCPCS)
        ]
    )];
    let rows = if gym_labs.exists() {
        json::parse(&fs::read_to_string(gym_labs)?)?
    } else {
        Json::Arr(Vec::new())
    };
    let mut gym: HashMap<String, &Json> = HashMap::new();
    for row in rows.as_array() {
        let label = row.get("label").as_str().ok_or("gym row without a label")?;
        // first row wins: blood before urine, BMP before ABG
        gym.entry(label.to_string()).or_insert(row);
        if row.get("category").as_str() == Some("Urine Studies") {
            gym.entry(format!("Urine {label}")).or_insert(row);
        }
    }

    let mut conditions = Json::obj();
    for &(label, code) in seed::CONDITIONS {
        let Some(display) = look.icd10(code)? else {
            problems.push(format!(
                "condition {}: ICD-10-CM {code} not found",
                py_repr(label)
            ));
            continue;
        };
        conditions.set(
            label,
            obj![
                ("system", ICD),
                ("code", code),
                ("display", display),
                ("receipt", "NLM Clinical Tables ICD-10-CM")
            ],
        );
    }
    catalog.set("conditions", conditions);

    let mut meds = Json::obj();
    for &(ingredient, strengths, form) in seed::MEDICATIONS {
        let scds = look.rxnorm_drugs(ingredient)?;
        let wanted: Vec<Option<&str>> = if strengths.is_empty() {
            vec![None]
        } else {
            strengths.iter().copied().map(Some).collect()
        };
        let form_lower = form.to_lowercase();
        for strength in wanted {
            let mut matched: Vec<(&str, &Json)> = Vec::new();
            for c in &scds {
                let name = c.get("name").as_str().ok_or("SCD without a name")?;
                if !name.to_lowercase().contains(&form_lower) {
                    continue;
                }
                if let Some(s) = strength
                    && !format!(" {name} ").contains(&format!(" {s} "))
                {
                    continue;
                }
                matched.push((name, c));
            }
            // Prefer the shortest exact SCD name (no packaging qualifiers).
            matched.sort_by(|a, b| (a.0.chars().count(), a.0).cmp(&(b.0.chars().count(), b.0)));
            let Some(&(name, chosen)) = matched.first() else {
                problems.push(format!(
                    "medication {ingredient} {} {form}: no SCD",
                    strength.unwrap_or("None")
                ));
                continue;
            };
            meds.set(
                name,
                obj![
                    ("system", RXNORM),
                    ("code", chosen.get("rxcui").clone()),
                    ("display", name),
                    ("ingredient", ingredient),
                    ("receipt", "NLM RxNav drugs.json (SCD)")
                ],
            );
        }
    }
    for &(ingredient, exact) in seed::EXACT_MEDICATIONS {
        let drugs = look.rxnorm_drugs(ingredient)?;
        let Some(found) = drugs.iter().find(|c| c.get("name").as_str() == Some(exact)) else {
            problems.push(format!(
                "medication exact {}: not an RxNorm SCD/GPCK name",
                py_repr(exact)
            ));
            continue;
        };
        meds.set(
            exact,
            obj![
                ("system", RXNORM),
                ("code", found.get("rxcui").clone()),
                ("display", exact),
                ("ingredient", ingredient),
                ("receipt", "NLM RxNav drugs.json (exact name)")
            ],
        );
    }
    catalog.set("medications", meds);

    let mut labs = Json::obj();
    for &(label, code, unit) in seed::LABS {
        let Some(display) = look.loinc(code)? else {
            problems.push(format!("lab {}: LOINC {code} not found", py_repr(label)));
            continue;
        };
        let mut entry = obj![
            ("system", LOINC),
            ("code", code),
            ("display", display),
            ("unit", opt(unit)),
            ("receipt", "NLM Clinical Tables LOINC")
        ];
        if let Some(g) = gym.get(label).filter(|g| g.truthy()) {
            let mut range = Json::obj();
            for k in ["low", "high", "low_f", "high_f"] {
                let v = g.get(k);
                if !v.is_null() && v.as_str() != Some("None") {
                    range.set(k, v.clone());
                }
            }
            entry.set("range", range);
            entry.set("range_source", "coda-gym lab_reference_ranges.xlsx".into());
        }
        labs.set(label, entry);
    }

    let mut panels = Json::obj();
    for &(label, code, members) in seed::PANELS {
        let display = look.loinc(code)?;
        let missing: Vec<&str> = members
            .iter()
            .copied()
            .filter(|m| !labs.contains(m))
            .collect();
        match display {
            Some(display) if missing.is_empty() => panels.set(
                label,
                obj![
                    ("system", LOINC),
                    ("code", code),
                    ("display", display),
                    (
                        "members",
                        members.iter().map(|m| Json::from(*m)).collect::<Vec<_>>()
                    )
                ],
            ),
            display => {
                let missing: Vec<String> = missing.iter().map(|m| py_repr(m)).collect();
                problems.push(format!(
                    "panel {}: LOINC {code} found={} missing=[{}]",
                    py_repr(label),
                    if display.is_some() { "True" } else { "False" },
                    missing.join(", ")
                ));
            }
        }
    }
    catalog.set("labs", labs);
    catalog.set("panels", panels);

    let surveys: Vec<(&str, &str, Option<&str>)> = seed::SURVEYS
        .iter()
        .map(|&(label, code, unit)| (label, code, Some(unit)))
        .collect();
    let social: Vec<(&str, &str, Option<&str>)> = seed::SOCIAL
        .iter()
        .map(|&(label, code)| (label, code, None))
        .collect();
    for (name, table) in [("surveys", surveys), ("social", social)] {
        let mut out = Json::obj();
        for (label, code, unit) in table {
            let Some(display) = look.loinc(code)? else {
                problems.push(format!("{name} {}: LOINC {code} not found", py_repr(label)));
                continue;
            };
            out.set(
                label,
                obj![
                    ("system", LOINC),
                    ("code", code),
                    ("display", display),
                    ("unit", opt(unit))
                ],
            );
        }
        catalog.set(name, out);
    }

    let mut reports = Json::obj();
    for &(label, code, section) in seed::REPORTS {
        let Some(display) = look.loinc(code)? else {
            problems.push(format!("report {}: LOINC {code} not found", py_repr(label)));
            continue;
        };
        let (_, section_display) = V2_0074
            .iter()
            .find(|(s, _)| *s == section)
            .ok_or("unknown v2-0074 section")?;
        reports.set(
            label,
            obj![
                ("system", LOINC),
                ("code", code),
                ("display", display),
                (
                    "category",
                    obj![("code", section), ("display", *section_display)]
                )
            ],
        );
    }
    catalog.set("reports", reports);

    // CPT has no public verification service. Codes harvested from the Coda
    // Agents Roger fixture keep that fixture's display; recalled codes carry no
    // display rather than an unverified one.
    let fixture = roger.join("Procedure.ndjson");
    let mut fixture_display: HashMap<String, Json> = HashMap::new();
    if !fixture.exists() {
        problems.push(format!(
            "Roger fixture not found at {}; pass --roger-fixture",
            fixture.display()
        ));
    } else {
        for line in fs::read_to_string(&fixture)?.lines() {
            let resource = json::parse(line)?;
            let coding = &resource.get("code").get("coding").as_array()[0];
            let code = coding.get("code").as_str().ok_or("fixture code")?;
            fixture_display.insert(code.to_string(), coding.get("display").clone());
        }
    }
    let mut procedures = Json::obj();
    for &(label, code, prov) in seed::PROCEDURES {
        let mut entry = obj![("system", CPT), ("code", code), ("provenance", prov)];
        if prov == "hcpcs" {
            // HCPCS Level II is verifiable: NLM Clinical Tables
            let Some(display) = look.hcpcs(code)? else {
                problems.push(format!(
                    "procedure {}: HCPCS {code} not found",
                    py_repr(label)
                ));
                continue;
            };
            entry = obj![
                ("system", HCPCS),
                ("code", code),
                ("display", display),
                ("provenance", "NLM Clinical Tables HCPCS")
            ];
        } else if prov == "fixture" {
            let Some(display) = fixture_display.get(code) else {
                problems.push(format!(
                    "procedure {}: CPT {code} not in the Roger fixture",
                    py_repr(label)
                ));
                continue;
            };
            entry.set("display", display.clone());
        }
        procedures.set(label, entry);
    }
    catalog.set("procedures", procedures);

    let mut allergens = Json::obj();
    for &(label, code, category) in seed::ALLERGENS {
        let Some(display) = look.snomed(code) else {
            problems.push(format!(
                "allergen {}: SNOMED {code} not found",
                py_repr(label)
            ));
            continue;
        };
        allergens.set(
            label,
            obj![
                ("system", SNOMED),
                ("code", code),
                ("display", display),
                ("category", category)
            ],
        );
    }
    catalog.set("allergens", allergens);
    let mut reactions = Json::obj();
    for &(label, code) in seed::REACTIONS {
        let Some(display) = look.snomed(code) else {
            problems.push(format!(
                "reaction {}: SNOMED {code} not found",
                py_repr(label)
            ));
            continue;
        };
        reactions.set(
            label,
            obj![("system", SNOMED), ("code", code), ("display", display)],
        );
    }
    catalog.set("reactions", reactions);

    look.save()?;
    fs::write(
        out,
        json::dumps(&sort_keys(&catalog), Layout::Indent(1), false) + "\n",
    )?;
    let counts = Json::Obj(
        catalog
            .as_object()
            .iter()
            .filter(|(k, _)| k != "systems")
            .map(|(k, v)| (k.clone(), Json::Int(v.as_object().len() as i64)))
            .collect(),
    );
    println!("{}", json::dumps(&counts, Layout::Default, true));
    for p in &problems {
        println!("PROBLEM {p}");
    }
    Ok(if problems.is_empty() { 0 } else { 1 })
}
