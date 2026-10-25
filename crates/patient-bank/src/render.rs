//! Translate authored patient ledgers into strict FHIR R4 NDJSON.
//!
//! The ledger is the clinical source of truth; this module only encodes it, in the
//! shape of the Coda Agents Roger fixture and coda-gym's chart-build contract. It
//! takes an as-of instant, so the bank chart (as of the practice's current time) and
//! any earlier snapshot come from the same code: events and records after the
//! instant do not exist yet, and statuses are those true at the instant.
//!
//! Code here never chooses a diagnosis, drug or result that the ledger did not
//! state. It does fill unstated members of an ordered lab panel with values inside
//! the reference range, and it computes derived values (eGFR, anion gap, LDL,
//! non-HDL, BMI, indices) from stated ones; each such Observation is listed in the
//! patient's render receipt.
//!
//! This is a port of `scripts/patient-bank/render.py` that reproduces its output
//! byte for byte: the same random draws in the same order, Python's rounding,
//! string conversions, comparisons of ledger stamps as strings where Python
//! compared strings, and on the Eastern wall clock where it compared datetimes.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::LazyLock;

use base64::Engine as _;
use jiff::civil::Date;
use regex::{Captures, Regex};
use sha2::{Digest, Sha256};

use crate::json::{self, Json};
use crate::obj;
use crate::rng::PyRandom;
use crate::time::{self, Local};

pub const MRN_SYSTEM: &str = "urn:oid:2.16.840.1.113883.19.5";
pub const UCUM: &str = "http://unitsofmeasure.org";
pub const OBS_CAT: &str = "http://terminology.hl7.org/CodeSystem/observation-category";
pub const V2_0074: &str = "http://terminology.hl7.org/CodeSystem/v2-0074";
pub const FAMILIES: [&str; 8] = [
    "Patient",
    "Condition",
    "MedicationRequest",
    "Observation",
    "DiagnosticReport",
    "DocumentReference",
    "Procedure",
    "AllergyIntolerance",
];

/// A vital sign's (LOINC code, display, text, UCUM code, unit).
pub type Vital = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

/// key -> vital; coda-gym authoring/references/vital_fields.md
pub const VITALS: [(&str, Vital); 8] = [
    ("hr", ("8867-4", "Heart rate", "Heart Rate", "/min", "/min")),
    (
        "rr",
        (
            "9279-1",
            "Respiratory rate",
            "Respiratory Rate",
            "/min",
            "/min",
        ),
    ),
    (
        "temp_c",
        ("8310-5", "Body temperature", "Temperature", "Cel", "C"),
    ),
    (
        "spo2",
        (
            "59408-5",
            "Oxygen saturation in Arterial blood by Pulse oximetry",
            "Oxygen Saturation",
            "%",
            "%",
        ),
    ),
    (
        "weight_kg",
        ("29463-7", "Body weight", "Weight", "kg", "kg"),
    ),
    ("height_cm", ("8302-2", "Body height", "Height", "cm", "cm")),
    (
        "bmi",
        (
            "39156-5",
            "Body mass index (BMI) [Ratio]",
            "Body Mass Index",
            "kg/m2",
            "kg/m2",
        ),
    ),
    (
        "peak_flow",
        (
            "33452-4",
            "Maximum expiratory gas flow Respiratory system airway by Peak flow meter",
            "Peak Expiratory Flow",
            "L/min",
            "L/min",
        ),
    ),
];
pub const BP: (&str, &str, &str) = (
    "85354-9",
    "Blood pressure panel with all children optional",
    "Blood Pressure",
);
pub const SBP: (&str, &str, &str) = (
    "8480-6",
    "Systolic blood pressure",
    "Systolic Blood Pressure",
);
pub const DBP: (&str, &str, &str) = (
    "8462-4",
    "Diastolic blood pressure",
    "Diastolic Blood Pressure",
);

/// Accurate LOINC document classes only; others stay text-only.
pub const DOC_TYPE_CODES: [(&str, (&str, &str)); 3] = [
    (
        "History and Physical",
        ("34117-2", "History and physical note"),
    ),
    ("Discharge Summary", ("18842-5", "Discharge summary")),
    ("Consults", ("11488-4", "Consult note")),
];

pub const MARITAL: [(&str, (&str, &str)); 6] = [
    ("married", ("M", "Married")),
    ("single", ("S", "Never Married")),
    ("divorced", ("D", "Divorced")),
    ("widowed", ("W", "Widowed")),
    ("partnered", ("T", "Domestic partner")),
    ("separated", ("L", "Legally Separated")),
];
pub const LANG: [(&str, &str); 11] = [
    ("English", "en"),
    ("Spanish", "es"),
    ("Vietnamese", "vi"),
    ("Mandarin", "zh"),
    ("Korean", "ko"),
    ("Khmer", "km"),
    ("Hindi", "hi"),
    ("Portuguese", "pt"),
    ("Russian", "ru"),
    ("Haitian Creole", "ht"),
    ("Polish", "pl"),
];
pub const RACE: [(&str, &str); 4] = [
    ("White", "2106-3"),
    ("Black or African American", "2054-5"),
    ("Asian", "2028-9"),
    ("Other Race", "2131-1"),
];
pub const ETHNICITY: [(&str, &str); 2] = [
    ("Hispanic or Latino", "2135-2"),
    ("Not Hispanic or Latino", "2186-5"),
];
pub const US_CORE: &str = "http://hl7.org/fhir/us/core/StructureDefinition";
pub const CDC_RACE: &str = "urn:oid:2.16.840.1.113883.6.238";

/// A normal range used to fill an unstated panel member: one range for
/// everyone or separate male and female ranges, and the decimals it reports.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fill {
    pub male: (f64, f64),
    pub female: (f64, f64),
    pub decimals: usize,
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

/// Normal ranges used to fill unstated panel members.
pub const FILL: [(&str, Fill); 38] = [
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
pub const FILL_TEXT: [(&str, &str); 8] = [
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
pub const DERIVED: [&str; 10] = [
    "Estimated Glomerular Filtration Rate (Creatinine)",
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

/// The value for `key` in one of the constant tables above.
pub fn lookup<'a, V>(table: &'a [(&str, V)], key: &str) -> Option<&'a V> {
    table.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
}

pub fn load_catalog(path: &Path) -> Json {
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    json::parse(&text).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

pub fn last_updated(utc: &str) -> String {
    format!("{}T00:00:00+00:00", head(utc, 10))
}

fn sha256_hex(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

pub fn opaque(prefix: &str, parts: &[&str]) -> String {
    format!("{prefix}-{}", &sha256_hex(&parts.join("|"))[..12])
}

pub fn b64(text: &str) -> String {
    base64::engine::general_purpose::STANDARD.encode(text.as_bytes())
}

static PACK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(NDA\d+ )?(\d+ ACTUAT |[\d.]+ ML )?").expect("regex"));
static TOPICAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(Topical|Cream|Gel|Ointment|Lotion)\b").expect("regex"));
static STRENGTH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d*\.?\d+) (MG/MG|MG/ML)\b").expect("regex"));
static UNITS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(MG|MCG|ML|UNT|MEQ|ACTUAT|HR|GM)\b").expect("regex"));
static FORMS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(Oral|Tablet|Capsule|Extended|Release|Delayed|Topical|Cream|Ointment|Gel|Injector|Pen|Auto-Injector|Solution|Spray|Nasal|Inhaler|Metered|Dose|Dry|Powder|Sublingual|Film|Disintegrating|Ophthalmic|Vaginal|Intrauterine|System|Injection|Transdermal|Suspension|Inhalation|Pack)\b",
    )
    .expect("regex")
});

fn percent(m: &Captures) -> String {
    let scale = if m[2].to_uppercase() == "MG/MG" {
        100.0
    } else {
        0.1
    };
    let pct = m[1].parse::<f64>().expect("strength number") * scale;
    format!("{}%", fmt_g(pct))
}

/// An RxNorm name as a clinician reads it: no application number or pack size,
/// lower-case units and dose forms, topical strengths as percent.
pub fn friendly_drug(name: &str) -> String {
    let name = PACK.replace(name, ""); // application number, pack size, pen volume
    let name = if TOPICAL.is_match(&name) {
        // topical strengths read as percent
        STRENGTH.replace_all(&name, percent).into_owned()
    } else {
        name.into_owned()
    };
    let text = UNITS.replace_all(&name, |m: &Captures| m[1].to_lowercase());
    FORMS
        .replace_all(&text, |m: &Captures| m[1].to_lowercase())
        .into_owned()
}

pub fn ckd_epi_2021(scr: f64, age: i64, female: bool) -> i64 {
    let (k, a) = if female { (0.7, -0.241) } else { (0.9, -0.302) };
    let ratio = scr / k;
    let egfr = 142.0
        * py_min(ratio, 1.0).powf(a)
        * py_max(ratio, 1.0).powf(-1.200)
        * 0.9938f64.powf(age as f64)
        * if female { 1.012 } else { 1.0 };
    egfr.round_ties_even() as i64
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Receipt {
    pub filled: Vec<String>,
    pub derived: Vec<String>,
    pub uncoded: Vec<String>,
    pub pending: Vec<String>,
    pub skipped: Vec<String>,
}

/// One patient's resources by family, in `FAMILIES` order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Chart {
    rows: [Vec<Json>; 8],
}

impl Chart {
    fn slot(family: &str) -> usize {
        FAMILIES
            .iter()
            .position(|f| *f == family)
            .unwrap_or_else(|| panic!("not a chart family: {family}"))
    }

    pub fn get(&self, family: &str) -> &[Json] {
        &self.rows[Self::slot(family)]
    }

    /// Every family with its rows, in `FAMILIES` order.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &[Json])> {
        FAMILIES
            .iter()
            .copied()
            .zip(self.rows.iter().map(Vec::as_slice))
    }

    fn push(&mut self, family: &str, row: Json) {
        self.rows[Self::slot(family)].push(row);
    }
}

/// What every resource of one render shares.
struct Context<'a> {
    mrn: &'a str,
    subject: Json,
    female: bool,
    birth: Date,
    cutoff: Local,
    rng_base: u64,
}

pub struct Renderer {
    c: Json,
    extra: Json,
}

impl Renderer {
    pub fn new(catalog: &Json, extra: &Json) -> Renderer {
        let extra = if extra.truthy() {
            extra.clone()
        } else {
            obj![("conditions", Json::obj()), ("medications", Json::obj())]
        };
        Renderer {
            c: catalog.clone(),
            extra,
        }
    }

    // ---- terminology -------------------------------------------------------
    fn condition_code(&self, item: &Json, receipt: &mut Receipt) -> Json {
        let label = item.get("label");
        let entry = member(self.c.get("conditions"), label);
        if entry.truthy() {
            return obj![("coding", vec![coding(entry)]), ("text", label.clone())];
        }
        let icd10 = if item.contains("icd10") {
            py_str(item.get("icd10"))
        } else {
            String::new()
        };
        let key = format!("{}|{icd10}", py_str(label));
        let extra = self.extra.get("conditions").get(&key);
        if extra.truthy() {
            let c = obj![
                ("system", self.c.get("systems").get("icd10cm").clone()),
                ("code", extra.get("code").clone()),
                ("display", extra.get("display").clone()),
            ];
            return obj![("coding", vec![c]), ("text", label.clone())];
        }
        receipt.uncoded.push(format!("Condition:{}", py_str(label)));
        obj![("text", label.clone())]
    }

    /// The medication's concept. (Python also returned the ingredient, which
    /// rendering never used.)
    fn medication_concept(&self, item: &Json, receipt: &mut Receipt) -> Json {
        let name = item.get("drug");
        if let Some(n) = name.as_str()
            && !n.is_empty()
            && self.c.get("medications").contains(n)
        {
            let e = self.c.get("medications").get(n);
            return obj![("coding", vec![coding(e)]), ("text", friendly_drug(n))];
        }
        let unspecified = Json::from("unspecified medication");
        let text = or(or(name, item.get("drug_text")), &unspecified);
        let extra = member(self.extra.get("medications"), text);
        if extra.truthy() {
            let c = obj![
                ("system", self.c.get("systems").get("rxnorm").clone()),
                ("code", extra.get("code").clone()),
                ("display", extra.get("display").clone()),
            ];
            return obj![("coding", vec![c]), ("text", text.clone())];
        }
        receipt
            .uncoded
            .push(format!("MedicationRequest:{}", py_str(text)));
        obj![("text", text.clone())]
    }

    // ---- patient ------------------------------------------------------------
    fn patient(&self, seed: &Json, led: &Json) -> Json {
        let mrn = string(seed.get("mrn"));
        let mut telecom = vec![obj![
            ("system", "phone"),
            ("value", seed.get("phone").clone()),
            ("use", "mobile"),
        ]];
        if seed.get("email").truthy() {
            telecom.push(obj![
                ("system", "email"),
                ("value", seed.get("email").clone()),
                ("use", "home"),
            ]);
        }
        let main_street = Json::from("1 Main Street");
        let mut res = obj![
            ("resourceType", "Patient"),
            ("id", mrn),
            (
                "meta",
                obj![
                    ("versionId", "1"),
                    ("lastUpdated", "2026-08-27T00:00:00+00:00")
                ]
            ),
            (
                "identifier",
                vec![obj![("system", MRN_SYSTEM), ("value", tail(mrn, 3))]]
            ),
            (
                "name",
                vec![obj![
                    ("use", "official"),
                    ("family", seed.get("family").clone()),
                    ("given", vec![seed.get("given").clone()]),
                ]]
            ),
            ("gender", seed.get("sex").clone()),
            ("birthDate", seed.get("birth_date").clone()),
            ("telecom", telecom),
            (
                "address",
                vec![obj![
                    ("use", "home"),
                    (
                        "line",
                        vec![or(led.get("address_line"), &main_street).clone()]
                    ),
                    ("city", seed.get("town").clone()),
                    ("state", "MA"),
                    ("postalCode", seed.get("postal_code").clone()),
                    ("country", "US"),
                ]]
            ),
        ];
        let marital = led
            .get("marital_status")
            .as_str()
            .unwrap_or("")
            .to_lowercase();
        if let Some((code, display)) = lookup(&MARITAL, &marital) {
            let c = obj![
                (
                    "system",
                    "http://terminology.hl7.org/CodeSystem/v3-MaritalStatus"
                ),
                ("code", *code),
                ("display", *display),
            ];
            res.set(
                "maritalStatus",
                obj![("coding", vec![c]), ("text", *display)],
            );
        }
        // seed.get("preferred_language", "English")
        let lang = if seed.contains("preferred_language") {
            seed.get("preferred_language").as_str()
        } else {
            Some("English")
        };
        if let Some(lang) = lang.filter(|l| *l != "English")
            && let Some(code) = lookup(&LANG, lang)
        {
            let c = obj![
                ("system", "urn:ietf:bcp:47"),
                ("code", *code),
                ("display", lang)
            ];
            res.set(
                "communication",
                vec![obj![
                    ("language", obj![("coding", vec![c]), ("text", lang)]),
                    ("preferred", true),
                ]]
                .into(),
            );
        }
        let mut ext = Vec::new();
        let race = seed.get("race");
        if race.truthy() && race.as_str() != Some("Declined") {
            let mut sub = vec![obj![("url", "text"), ("valueString", race.clone())]];
            if let Some(code) = race.as_str().and_then(|r| lookup(&RACE, r)) {
                let c = obj![
                    ("system", CDC_RACE),
                    ("code", *code),
                    ("display", race.clone())
                ];
                sub.insert(0, obj![("url", "ombCategory"), ("valueCoding", c)]);
            }
            ext.push(obj![
                ("url", format!("{US_CORE}/us-core-race")),
                ("extension", sub)
            ]);
        }
        let eth = seed.get("ethnicity");
        if let Some(code) = eth.as_str().and_then(|e| lookup(&ETHNICITY, e)) {
            let c = obj![
                ("system", CDC_RACE),
                ("code", *code),
                ("display", eth.clone())
            ];
            ext.push(obj![
                ("url", format!("{US_CORE}/us-core-ethnicity")),
                (
                    "extension",
                    vec![
                        obj![("url", "ombCategory"), ("valueCoding", c)],
                        obj![("url", "text"), ("valueString", eth.clone())],
                    ]
                ),
            ]);
        }
        if !ext.is_empty() {
            res.set("extension", ext.into());
        }
        res
    }

    // ---- main ---------------------------------------------------------------
    /// Resources by family as of `as_of` (Eastern `YYYY-MM-DDTHH:MM`).
    pub fn render(&self, seed: &Json, led: &Json, as_of: &str) -> (Chart, Receipt) {
        let cutoff = time::local(as_of).unwrap_or_else(|| panic!("as_of: {as_of}"));
        let mrn = string(seed.get("mrn"));
        let birth: Date = string(seed.get("birth_date"))
            .parse()
            .unwrap_or_else(|e| panic!("{mrn} birth_date: {e}"));
        let cx = Context {
            mrn,
            subject: obj![("reference", format!("Patient/{mrn}"))],
            female: seed.get("sex").as_str() == Some("female"),
            birth,
            cutoff,
            rng_base: u64::from_str_radix(&sha256_hex(mrn)[..8], 16).expect("hex"),
        };
        let subject = &cx.subject;
        let mut receipt = Receipt::default();
        let mut out = Chart::default();
        out.push("Patient", self.patient(seed, led));
        let mut baseline: HashMap<String, f64> = HashMap::new();
        let mut events: Vec<&Json> = led.get("events").as_array().iter().collect();
        events.sort_by(|a, b| string(a.get("start")).cmp(string(b.get("start"))));
        events.retain(|e| local_dt(e.get("start")) < cutoff);

        // Problem list: diagnosis or history items, resolved by a later action.
        let mut resolved_at: HashMap<&str, &str> = HashMap::new();
        for ev in &events {
            for item in ev.get("conditions").as_array() {
                if item.get("action").as_str() == Some("resolved") {
                    let when = or(item.get("date"), ev.get("start"));
                    if local_dt(when) < cutoff {
                        resolved_at
                            .entry(string(item.get("label")))
                            .or_insert(string(when));
                    }
                }
            }
        }
        let mut seen_conditions: HashSet<&str> = HashSet::new();
        // Medication orders: started here or recorded as historical; stopped later.
        let mut stops: HashMap<Option<&str>, &str> = HashMap::new();
        for ev in &events {
            for item in ev.get("medications").as_array() {
                if item.get("action").as_str() == Some("stop") {
                    let key = or(item.get("drug"), item.get("drug_text")).as_str();
                    let when = or(item.get("date"), ev.get("start"));
                    if local_dt(when) < cutoff {
                        stops.entry(key).or_insert(string(when));
                    }
                }
            }
        }
        let mut height: Option<f64> = None;

        for ev in &events {
            let eid = string(ev.get("id"));
            let start = string(ev.get("start"));
            let start_utc = to_utc(ev.get("start"));
            for item in ev.get("conditions").as_array() {
                if !matches!(item.get("action").as_str(), Some("diagnosed" | "history")) {
                    continue;
                }
                let label = string(item.get("label"));
                if !seen_conditions.insert(label) {
                    continue; // one problem-list row per concept
                }
                let mut res = obj![
                    ("resourceType", "Condition"),
                    ("id", opaque("cond", &[mrn, eid, label])),
                    ("meta", meta(&start_utc)),
                    ("clinicalStatus", Json::Null),
                    (
                        "verificationStatus",
                        codings(
                            "http://terminology.hl7.org/CodeSystem/condition-ver-status",
                            "confirmed",
                            "Confirmed"
                        )
                    ),
                    (
                        "category",
                        vec![obj![
                            (
                                "coding",
                                vec![obj![
                                    (
                                        "system",
                                        "http://terminology.hl7.org/CodeSystem/condition-category"
                                    ),
                                    ("code", "problem-list-item"),
                                    ("display", "Problem List Item"),
                                ]]
                            ),
                            ("text", "Problem List Item"),
                        ]]
                    ),
                    ("code", self.condition_code(item, &mut receipt)),
                    ("subject", subject.clone()),
                ];
                let onset = item.get("onset");
                if onset.truthy() && head(string(onset), 10) < head(start, 10) {
                    let onset_s = string(onset);
                    let value = if onset_s.chars().count() <= 10 {
                        head(onset_s, 10).to_string()
                    } else {
                        to_utc(onset)
                    };
                    res.set("onsetDateTime", value.into());
                } else {
                    res.set("onsetDateTime", start_utc.clone().into());
                }
                res.set("recordedDate", start_utc.clone().into());
                let clinical = "http://terminology.hl7.org/CodeSystem/condition-clinical";
                match resolved_at.get(label) {
                    Some(when) if *when >= start => {
                        res.set("clinicalStatus", codings(clinical, "resolved", "Resolved"));
                        res.set("abatementDateTime", time_utc(when).into());
                    }
                    _ => res.set("clinicalStatus", codings(clinical, "active", "Active")),
                }
                if item.get("note").truthy() {
                    res.set(
                        "note",
                        vec![obj![("text", item.get("note").clone())]].into(),
                    );
                }
                out.push("Condition", res);
            }

            for (i, item) in ev.get("medications").as_array().iter().enumerate() {
                let action = item.get("action").as_str();
                if !matches!(action, Some("start" | "historical")) {
                    continue;
                }
                let key = or(item.get("drug"), item.get("drug_text"));
                let concept = self.medication_concept(item, &mut receipt);
                let authored = or(item.get("date"), ev.get("start"));
                let authored_utc = to_utc(authored);
                let mut res = obj![
                    ("resourceType", "MedicationRequest"),
                    (
                        "id",
                        opaque("medrx", &[mrn, eid, &i.to_string(), string(key)])
                    ),
                    ("meta", meta(&authored_utc)),
                    ("status", "active"),
                    ("intent", "order"),
                    ("medicationCodeableConcept", concept),
                    ("subject", subject.clone()),
                    ("authoredOn", authored_utc),
                ];
                if action == Some("historical") {
                    res.set("reportedBoolean", true.into());
                }
                let stop = stops
                    .get(&key.as_str())
                    .filter(|stop| !stop.is_empty() && **stop >= string(authored));
                if let Some(stop) = stop {
                    res.set("status", "stopped".into());
                    res.set(
                        "statusReason",
                        obj![("text", format!("Discontinued {}", head(stop, 10)))],
                    );
                } else if item.get("course").as_str() == Some("short")
                    && time::add(local_dt(authored), time::days(21)) < cutoff
                {
                    res.set("status", "completed".into());
                }
                let mut dose = Json::obj();
                let empty = Json::from("");
                let sig = or(or(item.get("sig"), item.get("frequency")), &empty);
                dose.set("text", sig.clone());
                if item.get("frequency").truthy() {
                    dose.set(
                        "timing",
                        obj![("code", obj![("text", item.get("frequency").clone())])],
                    );
                }
                let d = item.get("dose");
                if matches!(d, Json::Obj(_)) && number(d.get("value")).is_some() {
                    let unit = if d.contains("unit") {
                        d.get("unit").clone()
                    } else {
                        empty.clone()
                    };
                    dose.set(
                        "doseAndRate",
                        vec![obj![(
                            "doseQuantity",
                            obj![("value", d.get("value").clone()), ("unit", unit)]
                        )]]
                        .into(),
                    );
                }
                if item.get("route").truthy() {
                    dose.set("route", obj![("text", item.get("route").clone())]);
                }
                if dose.as_object().iter().any(|(_, v)| v.truthy()) {
                    res.set("dosageInstruction", vec![dose].into());
                }
                if item.get("reason").truthy() {
                    res.set(
                        "reasonCode",
                        vec![obj![("text", item.get("reason").clone())]].into(),
                    );
                }
                out.push("MedicationRequest", res);
            }

            for (i, item) in ev.get("allergies").as_array().iter().enumerate() {
                let substance = item.get("substance");
                let entry = member(self.c.get("allergens"), substance);
                let code = if entry.truthy() {
                    obj![("coding", vec![coding(entry)]), ("text", substance.clone())]
                } else {
                    let unknown = Json::from("Unknown");
                    obj![("text", get_or(item, "substance", &unknown).clone())]
                };
                if !entry.truthy() {
                    receipt
                        .uncoded
                        .push(format!("AllergyIntolerance:{}", py_str(substance)));
                }
                let kind = item
                    .get("type")
                    .as_str()
                    .filter(|t| matches!(*t, "allergy" | "intolerance"))
                    .unwrap_or("allergy");
                let criticality = item
                    .get("criticality")
                    .as_str()
                    .filter(|c| matches!(*c, "low" | "high" | "unable-to-assess"))
                    .unwrap_or("low");
                let category = if entry.truthy() {
                    entry.get("category").clone()
                } else {
                    "medication".into()
                };
                let mut res = obj![
                    ("resourceType", "AllergyIntolerance"),
                    ("id", opaque("allergy", &[mrn, eid, &i.to_string()])),
                    ("meta", meta(&start_utc)),
                    (
                        "clinicalStatus",
                        codings(
                            "http://terminology.hl7.org/CodeSystem/allergyintolerance-clinical",
                            "active",
                            "Active"
                        )
                    ),
                    (
                        "verificationStatus",
                        codings(
                            "http://terminology.hl7.org/CodeSystem/allergyintolerance-verification",
                            "confirmed",
                            "Confirmed"
                        )
                    ),
                    ("type", kind),
                    ("category", vec![category]),
                    ("criticality", criticality),
                    ("code", code),
                    ("patient", subject.clone()),
                    ("recordedDate", start_utc.clone()),
                ];
                let rx = member(self.c.get("reactions"), item.get("reaction"));
                if rx.truthy() || item.get("reaction").truthy() {
                    let man = if rx.truthy() {
                        obj![
                            ("coding", vec![coding(rx)]),
                            ("text", item.get("reaction").clone())
                        ]
                    } else {
                        obj![("text", item.get("reaction").clone())]
                    };
                    let mut reaction = obj![("manifestation", vec![man])];
                    if let Some(sev @ ("mild" | "moderate" | "severe")) =
                        item.get("severity").as_str()
                    {
                        reaction.set("severity", sev.into());
                    }
                    res.set("reaction", vec![reaction].into());
                }
                out.push("AllergyIntolerance", res);
            }

            let vitals = ev.get("vitals");
            let vit_sets: Vec<&Json> = if !vitals.truthy() {
                Vec::new()
            } else if matches!(vitals, Json::Obj(_)) {
                vec![vitals]
            } else {
                vitals.as_array().iter().collect()
            };
            for (vi, vit) in vit_sets.iter().enumerate() {
                // a stay records several timed sets
                let vi_s = vi.to_string();
                let tag: Vec<&str> = if vit_sets.len() == 1 && !vit.get("time").truthy() {
                    vec![eid]
                } else {
                    vec![eid, &vi_s]
                };
                let tagged = |key: &str| {
                    let mut parts = vec![mrn];
                    parts.extend(&tag);
                    parts.push(key);
                    opaque("obs", &parts)
                };
                let when = if vit.get("time").truthy() {
                    if local_dt(vit.get("time")) >= cutoff {
                        continue;
                    }
                    to_utc(vit.get("time"))
                } else {
                    let minutes = 4 + (cx.rng_base % 9) as i64;
                    time::to_utc(time::add(local_dt(ev.get("start")), time::minutes(minutes)))
                };
                let obs_cat = || {
                    vec![obj![(
                        "coding",
                        vec![obj![
                            ("system", OBS_CAT),
                            ("code", "vital-signs"),
                            ("display", "Vital Signs"),
                        ]]
                    )]]
                };
                if let Json::Arr(bp) = vit.get("bp")
                    && bp.len() == 2
                {
                    let part = |(code, display, text): (&str, &str, &str), value: &Json| {
                        obj![
                            ("code", loinc(code, display, text)),
                            (
                                "valueQuantity",
                                obj![
                                    ("value", value.clone()),
                                    ("unit", "mmHg"),
                                    ("system", UCUM),
                                    ("code", "mm[Hg]"),
                                ]
                            ),
                        ]
                    };
                    out.push(
                        "Observation",
                        obj![
                            ("resourceType", "Observation"),
                            ("id", tagged("bp")),
                            ("meta", meta(&when)),
                            ("status", "final"),
                            ("category", obs_cat()),
                            ("code", loinc(BP.0, BP.1, BP.2)),
                            ("subject", subject.clone()),
                            ("effectiveDateTime", when.clone()),
                            ("component", vec![part(SBP, &bp[0]), part(DBP, &bp[1])]),
                        ],
                    );
                }
                if let Some(h) = number(vit.get("height_cm")) {
                    height = Some(h);
                }
                let mut values = Json::Obj(
                    vit.as_object()
                        .iter()
                        .filter(|(k, _)| k != "time")
                        .cloned()
                        .collect(),
                );
                if !values.contains("bmi")
                    && let Some(weight) = number(values.get("weight_kg"))
                    && let Some(h) = height.filter(|h| *h != 0.0)
                {
                    values.set("bmi", py_round(weight / (h / 100.0).powf(2.0), 1).into());
                    receipt
                        .derived
                        .push(format!("{}:Body Mass Index", tag.join(":")));
                }
                for (key, (code, display, text, ucum, unit)) in VITALS {
                    let v = values.get(key);
                    if number(v).is_none() {
                        continue;
                    }
                    out.push(
                        "Observation",
                        obj![
                            ("resourceType", "Observation"),
                            ("id", tagged(key)),
                            ("meta", meta(&when)),
                            ("status", "final"),
                            ("category", obs_cat()),
                            ("code", loinc(code, display, text)),
                            ("subject", subject.clone()),
                            ("effectiveDateTime", when.clone()),
                            (
                                "valueQuantity",
                                obj![
                                    ("value", v.clone()),
                                    ("unit", unit),
                                    ("system", UCUM),
                                    ("code", ucum),
                                ]
                            ),
                        ],
                    );
                }
            }

            for (i, panel_item) in ev.get("labs").as_array().iter().enumerate() {
                self.labs(
                    &mut out,
                    &mut receipt,
                    &cx,
                    eid,
                    i,
                    panel_item,
                    ev,
                    &mut baseline,
                );
            }

            for (i, s) in ev.get("surveys").as_array().iter().enumerate() {
                let entry = member(self.c.get("surveys"), s.get("survey"));
                if !entry.truthy() || number(s.get("value")).is_none() {
                    receipt
                        .skipped
                        .push(format!("{eid}:survey:{}", py_str(s.get("survey"))));
                    continue;
                }
                let when = to_utc(or(s.get("date"), ev.get("start")));
                out.push(
                    "Observation",
                    obj![
                        ("resourceType", "Observation"),
                        ("id", opaque("obs", &[mrn, eid, "survey", &i.to_string()])),
                        ("meta", meta(&when)),
                        ("status", "final"),
                        (
                            "category",
                            vec![obj![(
                                "coding",
                                vec![obj![
                                    ("system", OBS_CAT),
                                    ("code", "survey"),
                                    ("display", "Survey"),
                                ]]
                            )]]
                        ),
                        ("code", loinc_entry(entry, s.get("survey"))),
                        ("subject", subject.clone()),
                        ("effectiveDateTime", when),
                        (
                            "valueQuantity",
                            obj![
                                ("value", s.get("value").clone()),
                                ("unit", "{score}"),
                                ("system", UCUM),
                                ("code", "{score}"),
                            ]
                        ),
                    ],
                );
            }
            for (i, s) in ev.get("social").as_array().iter().enumerate() {
                let entry = member(self.c.get("social"), s.get("item"));
                if !entry.truthy() || !s.get("value").truthy() {
                    receipt
                        .skipped
                        .push(format!("{eid}:social:{}", py_str(s.get("item"))));
                    continue;
                }
                out.push(
                    "Observation",
                    obj![
                        ("resourceType", "Observation"),
                        ("id", opaque("obs", &[mrn, eid, "social", &i.to_string()])),
                        ("meta", meta(&start_utc)),
                        ("status", "final"),
                        (
                            "category",
                            vec![obj![(
                                "coding",
                                vec![obj![
                                    ("system", OBS_CAT),
                                    ("code", "social-history"),
                                    ("display", "Social History"),
                                ]]
                            )]]
                        ),
                        ("code", loinc_entry(entry, s.get("item"))),
                        ("subject", subject.clone()),
                        ("effectiveDateTime", start_utc.clone()),
                        (
                            "valueCodeableConcept",
                            obj![("text", py_str(s.get("value")))]
                        ),
                    ],
                );
            }

            for (i, rep) in ev.get("reports").as_array().iter().enumerate() {
                self.report(&mut out, &mut receipt, &cx, eid, i, rep, ev);
            }

            for (i, p) in ev.get("procedures").as_array().iter().enumerate() {
                let entry = member(self.c.get("procedures"), p.get("procedure"));
                let when = or(p.get("performed"), ev.get("start"));
                if local_dt(when) >= cutoff {
                    continue;
                }
                let code = if entry.truthy() {
                    let mut c = obj![
                        ("system", entry.get("system").clone()),
                        ("code", entry.get("code").clone()),
                    ];
                    if entry.get("display").truthy() {
                        c.set("display", entry.get("display").clone());
                    }
                    obj![("coding", vec![c]), ("text", p.get("procedure").clone())]
                } else {
                    receipt
                        .uncoded
                        .push(format!("Procedure:{}", py_str(p.get("procedure"))));
                    let fallback = Json::from("Procedure");
                    obj![("text", get_or(p, "procedure", &fallback).clone())]
                };
                let when_utc = to_utc(when);
                out.push(
                    "Procedure",
                    obj![
                        ("resourceType", "Procedure"),
                        ("id", opaque("proc", &[mrn, eid, &i.to_string()])),
                        ("meta", meta(&when_utc)),
                        ("status", "completed"),
                        ("code", code),
                        ("subject", subject.clone()),
                        ("performedDateTime", when_utc),
                    ],
                );
            }
            for (i, imm) in ev.get("immunizations").as_array().iter().enumerate() {
                let when = or(imm.get("date"), ev.get("start"));
                if local_dt(when) >= cutoff {
                    continue;
                }
                let entry = self.c.get("procedures").get("Immunization administration");
                let vaccine = Json::from("Vaccine");
                let text = format!(
                    "{} vaccine administration",
                    py_str(get_or(imm, "vaccine", &vaccine))
                );
                let when_utc = to_utc(when);
                out.push(
                    "Procedure",
                    obj![
                        ("resourceType", "Procedure"),
                        ("id", opaque("proc", &[mrn, eid, "imm", &i.to_string()])),
                        ("meta", meta(&when_utc)),
                        ("status", "completed"),
                        (
                            "code",
                            obj![("coding", vec![coding(entry)]), ("text", text)]
                        ),
                        ("subject", subject.clone()),
                        ("performedDateTime", when_utc),
                    ],
                );
            }

            for (i, doc) in ev.get("documents").as_array().iter().enumerate() {
                let when = or(doc.get("date"), ev.get("start"));
                if local_dt(when) >= cutoff {
                    receipt.pending.push(format!("{eid}:document after cutoff"));
                    continue;
                }
                let d = local_dt(when).date();
                let service = Json::from("Internal Medicine — Primary Care");
                let header = format!(
                    "{}\nDate: {}\nPatient: {} {} | MRN: {} | Age: {}\nService: {}\n\n",
                    py_str(or(doc.get("title"), doc.get("type"))),
                    d.strftime("%m/%d/%Y"),
                    py_str(seed.get("given")),
                    py_str(seed.get("family")),
                    tail(mrn, 3),
                    time::age_on(birth, d),
                    py_str(or(ev.get("service"), &service)),
                );
                let text = format!("{header}{}\n", py_strip(string(doc.get("body"))));
                let progress = Json::from("Progress Notes");
                let dtype = or(doc.get("type"), &progress);
                let mut type_cc = obj![("text", dtype.clone())];
                for (key, (code, display)) in DOC_TYPE_CODES {
                    let t = string(dtype);
                    if t == key || t.ends_with(key) {
                        type_cc.set(
                            "coding",
                            vec![obj![
                                ("system", "http://loinc.org"),
                                ("code", code),
                                ("display", display),
                            ]]
                            .into(),
                        );
                    }
                }
                let when_utc = to_utc(when);
                let doc_status = if doc.get("status").as_str() == Some("preliminary") {
                    "preliminary"
                } else {
                    "final"
                };
                out.push(
                    "DocumentReference",
                    obj![
                        ("resourceType", "DocumentReference"),
                        ("id", opaque("docref", &[mrn, eid, &i.to_string()])),
                        ("meta", meta(&when_utc)),
                        ("status", "current"),
                        ("docStatus", doc_status),
                        ("type", type_cc),
                        (
                            "category",
                            vec![obj![(
                                "coding",
                                vec![obj![
                                    (
                                        "system",
                                        "http://hl7.org/fhir/us/core/CodeSystem/us-core-documentreference-category"
                                    ),
                                    ("code", "clinical-note"),
                                    ("display", "Clinical Note"),
                                ]]
                            )]]
                        ),
                        ("subject", subject.clone()),
                        ("date", when_utc),
                        ("description", or(doc.get("title"), dtype).clone()),
                        (
                            "content",
                            vec![obj![(
                                "attachment",
                                obj![
                                    ("contentType", "text/plain; charset=utf-8"),
                                    ("data", b64(&text)),
                                ]
                            )]]
                        ),
                    ],
                );
            }
        }
        (out, receipt)
    }

    // ---- labs ---------------------------------------------------------------
    fn value(
        &self,
        label: &str,
        female: bool,
        rng: &mut PyRandom,
        baseline: &mut HashMap<String, f64>,
    ) -> Option<Json> {
        let spec = lookup(&FILL, label)?;
        let (lo, hi) = if female { spec.female } else { spec.male };
        let base = *baseline
            .entry(label.to_string())
            .or_insert_with(|| rng.uniform(lo + (hi - lo) * 0.2, hi - (hi - lo) * 0.2));
        let v = base + rng.gauss(0.0, (hi - lo) * 0.15);
        let v = py_min(py_max(v, lo), hi);
        Some(if spec.decimals > 0 {
            Json::Float(py_round(v, spec.decimals))
        } else {
            Json::Int(v.round_ties_even() as i64)
        })
    }

    fn quantity(&self, label: &str, value: &Json) -> Json {
        let unit = self.c.get("labs").get(label).get("unit");
        let mut q = obj![("value", value.clone())];
        if unit.truthy() {
            q.set("unit", unit.clone());
            q.set("system", UCUM.into());
            q.set("code", unit.clone());
        }
        q
    }

    /// The reference range and its numeric low and high.
    fn ref_range(&self, label: &str, female: bool) -> Option<(Json, f64, f64)> {
        let entry = self.c.get("labs").get(label);
        let r = entry.get("range");
        if !r.truthy() {
            return None;
        }
        let pick = |key: &str| {
            let f = r.get(&format!("{key}_f"));
            if female && !f.is_null() && f.as_str() != Some("") {
                f
            } else {
                r.get(key)
            }
        };
        let lo = py_float(pick("low"))?;
        let hi = py_float(pick("high"))?;
        let unit = entry.get("unit");
        let bound = |value: f64| {
            let mut q = obj![("value", value)];
            if unit.truthy() {
                q.set("unit", unit.clone());
                q.set("system", UCUM.into());
                q.set("code", unit.clone());
            }
            q
        };
        let rr = vec![obj![("low", bound(lo)), ("high", bound(hi))]];
        Some((rr.into(), lo, hi))
    }

    #[allow(clippy::too_many_arguments)]
    fn labs(
        &self,
        out: &mut Chart,
        receipt: &mut Receipt,
        cx: &Context,
        eid: &str,
        i: usize,
        item: &Json,
        ev: &Json,
        baseline: &mut HashMap<String, f64>,
    ) {
        let collected = or(item.get("collected"), ev.get("start"));
        if local_dt(collected) >= cx.cutoff {
            return;
        }
        let (mrn, female) = (cx.mrn, cx.female);
        let i_s = i.to_string();
        let panel_name = item.get("panel");
        let panel = if panel_name.truthy() {
            Some(member(self.c.get("panels"), panel_name)).filter(|p| p.truthy())
        } else {
            None
        };
        let labs = self.c.get("labs");
        // {r["test"]: r ...}: first position, last value
        let mut stated: Vec<(&str, &Json)> = Vec::new();
        for r in item.get("results").as_array() {
            if !r.get("test").truthy() {
                continue;
            }
            let t = string(r.get("test"));
            match stated.iter_mut().find(|(k, _)| *k == t) {
                Some(slot) => slot.1 = r,
                None => stated.push((t, r)),
            }
        }
        for (t, _) in &stated {
            if !labs.contains(t) {
                receipt.skipped.push(format!("{eid}:lab:{t}"));
            }
        }
        stated.retain(|(t, _)| labs.contains(t));
        let slow = stated
            .iter()
            .any(|(t, _)| matches!(*t, "Urine Culture" | "Blood Culture"));
        let seed = cx.rng_base + hex_prefix(&format!("{eid}{i}"), 6);
        let mut rng = PyRandom::new(u128::from(seed));
        let delay = if slow {
            time::hours(46 + rng.randint(0, 20))
        } else {
            time::minutes(95 + rng.randint(0, 300))
        };
        let issued_local = if item.get("resulted").truthy() {
            local_dt(item.get("resulted"))
        } else {
            time::add(local_dt(collected), delay)
        };
        let pending = issued_local >= cx.cutoff;
        let eff = to_utc(collected);
        let issued = time::to_utc(issued_local);
        let mut values: HashMap<&str, Json> = HashMap::new();
        let members: Vec<&str> = match panel {
            Some(p) => p.get("members").as_array().iter().map(string).collect(),
            None => stated.iter().map(|(t, _)| *t).collect(),
        };
        for t in &members {
            if let Some((_, r)) = stated.iter().find(|(k, _)| k == t) {
                values.insert(t, (*r).clone());
            }
        }
        if panel.is_some() {
            let age = time::age_on(cx.birth, local_dt(collected).date());
            let has = |t: &str| members.contains(&t);
            let diff_open: Vec<&str> = DIFFERENTIAL
                .into_iter()
                .filter(|t| has(t) && !values.contains_key(t))
                .collect();
            let mcv_t = "Mean Corpuscular Volume";
            if has(mcv_t)
                && !values.contains_key(mcv_t)
                && let (Some(hct), Some(rbc)) =
                    (num(&values, "Hematocrit"), num(&values, "Red Blood Cells"))
            {
                let v = (hct / rbc * 10.0).round_ties_even() as i64;
                values.insert(mcv_t, derived(v.into()));
            }
            for t in &members {
                if values.contains_key(t) || DERIVED.contains(t) || diff_open.contains(t) {
                    continue;
                }
                if let Some(text) = lookup(&FILL_TEXT, t) {
                    values.insert(t, obj![("text", *text), ("_filled", true)]);
                    continue;
                }
                if let Some(v) = self.value(t, female, &mut rng, baseline) {
                    values.insert(t, obj![("value", v), ("_filled", true)]);
                }
            }
            if !diff_open.is_empty() {
                // unstated differential members share what the stated ones leave of 100%
                let stated_diff: Vec<PyNum> = DIFFERENTIAL
                    .into_iter()
                    .filter_map(|t| values.get(t))
                    .map(|r| {
                        PyNum::of(r.get("value"))
                            .filter(|n| n.truthy())
                            .unwrap_or(PyNum::Int(0))
                    })
                    .collect();
                let room = 100.0 - py_sum(&stated_diff);
                let share: Vec<(&str, f64)> = diff_open
                    .iter()
                    .map(|t| {
                        let v = self
                            .value(t, female, &mut rng, baseline)
                            .and_then(|v| number(&v))
                            .unwrap_or(0.0);
                        (*t, v)
                    })
                    .collect();
                let total: f64 = share.iter().map(|(_, v)| v).sum();
                let total = if total == 0.0 { 1.0 } else { total };
                let mut fill: Vec<(&str, i64)> = share
                    .iter()
                    .map(|(t, v)| (*t, ((room * v / total).round_ties_even() as i64).max(0)))
                    .collect();
                let mut biggest = 0;
                for (n, (_, v)) in share.iter().enumerate() {
                    if *v > share[biggest].1 {
                        biggest = n;
                    }
                }
                let sum: i64 = fill.iter().map(|(_, v)| v).sum();
                fill[biggest].1 = (fill[biggest].1 + room.round_ties_even() as i64 - sum).max(0);
                for (t, v) in fill {
                    values.insert(t, obj![("value", v), ("_filled", true)]);
                }
            }
            // Derived members, computed only from values present in this draw. Red-cell
            // indices come from one consistent set: Hgb, MCV and MCHC give Hct and RBC.
            let (hgb, mcv) = (num(&values, "Hemoglobin"), num(&values, mcv_t));
            let (mut hct, rbc) = (
                num_or_zero(&values, "Hematocrit"),
                num_or_zero(&values, "Red Blood Cells"),
            );
            if let (Some(hgb), Some(mcv)) = (hgb, mcv) {
                if hct.is_none()
                    && let Some(rbc) = rbc
                {
                    hct = Some(py_round(rbc * mcv / 10.0, 1));
                }
                if hct.is_none() {
                    // baseline.setdefault("_mchc", rng.uniform(...)) draws every time
                    let first = rng.uniform(32.6, 34.2);
                    let mchc = *baseline.entry("_mchc".to_string()).or_insert(first)
                        + rng.uniform(-0.3, 0.3);
                    hct = Some(py_round(hgb / mchc * 100.0, 1));
                }
                let hct = hct.expect("set above");
                let rbc = rbc.unwrap_or_else(|| py_round(hct / mcv * 10.0, 2));
                for (t, v) in [("Hematocrit", hct), ("Red Blood Cells", rbc)] {
                    if has(t) && !values.contains_key(t) {
                        values.insert(t, derived(v.into()));
                    }
                }
            }
            let mut derive = |t: &'static str, inputs: &[&str], f: &dyn Fn(&[f64]) -> Json| {
                if has(t) && !values.contains_key(t) {
                    let got: Option<Vec<f64>> = inputs.iter().map(|x| num(&values, x)).collect();
                    if let Some(got) = got {
                        let v = f(&got);
                        values.insert(t, derived(v));
                    }
                }
            };
            derive(
                "Mean Corpuscular Hemoglobin",
                &["Hemoglobin", "Red Blood Cells"],
                &|x| py_round(x[0] / x[1] * 10.0, 1).into(),
            );
            derive(
                "Mean Corpuscular Hemoglobin Concentration",
                &["Hemoglobin", "Hematocrit"],
                &|x| py_round(x[0] / x[1] * 100.0, 1).into(),
            );
            derive(
                "Absolute Neutrophil Count",
                &["White Blood Cells", "Neutrophils %"],
                &|x| py_round(x[0] * x[1] / 100.0, 1).into(),
            );
            derive(
                "Estimated Glomerular Filtration Rate (Creatinine)",
                &["Creatinine"],
                &|x| ckd_epi_2021(x[0], age, female).into(),
            );
            derive("Anion Gap", &["Sodium", "Chloride", "Bicarbonate"], &|x| {
                round_int(x[0] - x[1] - x[2])
            });
            derive(
                "Total Cholesterol",
                &[
                    "Low Density Lipoprotein",
                    "High Density Lipoprotein",
                    "Triglycerides",
                ],
                &|x| round_int(x[0] + x[1] + x[2] / 5.0),
            );
            derive(
                "Non-HDL Cholesterol",
                &["Total Cholesterol", "High Density Lipoprotein"],
                &|x| round_int(x[0] - x[1]),
            );
            derive(
                "Transferrin Saturation",
                &["Iron", "Total Iron-Binding Capacity"],
                &|x| round_int(x[0] / x[1] * 100.0),
            );
        }
        let lab_cat = vec![obj![(
            "coding",
            vec![obj![
                ("system", OBS_CAT),
                ("code", "laboratory"),
                ("display", "Laboratory"),
            ]]
        )]];
        let status = if pending { "registered" } else { "final" };
        let subject = &cx.subject;
        let mut refs = Vec::new();
        for t in &members {
            let Some(r) = values.get(t) else {
                continue;
            };
            let entry = labs.get(t);
            assert!(labs.contains(t), "panel member {t} is not in the catalog");
            let oid = opaque("obs", &[mrn, eid, &i_s, t]);
            let mut res = obj![
                ("resourceType", "Observation"),
                ("id", oid.clone()),
                ("meta", meta(&eff)),
                ("status", status),
                ("category", lab_cat.clone()),
                ("code", obj![("coding", vec![coding(entry)]), ("text", *t)]),
                ("subject", subject.clone()),
                ("effectiveDateTime", eff.clone()),
            ];
            if !pending {
                res.set("issued", issued.clone().into());
                if let Some(value) = number(r.get("value")) {
                    res.set("valueQuantity", self.quantity(t, r.get("value")));
                    if let Some((rr, lo, hi)) = self.ref_range(t, female) {
                        res.set("referenceRange", rr);
                        let flag = if value > hi {
                            Some(("H", "High"))
                        } else if value < lo {
                            Some(("L", "Low"))
                        } else {
                            None
                        };
                        if let Some((code, display)) = flag {
                            let cc = codings(
                                "http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation",
                                code,
                                display,
                            );
                            res.set("interpretation", vec![cc].into());
                        }
                    }
                } else if r.get("text").truthy() {
                    res.set(
                        "valueCodeableConcept",
                        obj![("text", py_str(r.get("text")))],
                    );
                } else {
                    continue;
                }
            }
            if r.get("_filled").truthy() {
                receipt.filled.push(format!("{eid}:{t}"));
            }
            if r.get("_derived").truthy() {
                receipt.derived.push(format!("{eid}:{t}"));
            }
            out.push("Observation", res);
            refs.push(obj![("reference", format!("Observation/{oid}"))]);
        }
        if pending {
            let name = if panel_name.truthy() {
                py_str(panel_name)
            } else {
                "lab".to_string()
            };
            receipt
                .pending
                .push(format!("{eid}:{name} pending at cutoff"));
        }
        if let Some(panel) = panel
            && !refs.is_empty()
        {
            let code = obj![
                ("coding", vec![coding(panel)]),
                ("text", panel_name.clone())
            ];
            let mut dr = obj![
                ("resourceType", "DiagnosticReport"),
                ("id", opaque("diagrep", &[mrn, eid, &i_s])),
                ("meta", meta(&eff)),
                ("status", status),
                (
                    "category",
                    vec![obj![(
                        "coding",
                        vec![obj![
                            ("system", V2_0074),
                            ("code", "LAB"),
                            ("display", "Laboratory")
                        ]]
                    )]]
                ),
                ("code", code.clone()),
                ("subject", subject.clone()),
                ("effectiveDateTime", eff.clone()),
                ("result", refs.clone()),
            ];
            if !pending {
                dr.set("issued", issued.clone().into());
            }
            out.push("DiagnosticReport", dr);
            let mut po = obj![
                ("resourceType", "Observation"),
                ("id", opaque("obs-panel", &[mrn, eid, &i_s])),
                ("meta", meta(&eff)),
                ("status", status),
                ("category", lab_cat),
                ("code", code),
                ("subject", subject.clone()),
                ("effectiveDateTime", eff),
                ("hasMember", refs),
            ];
            if !pending {
                po.set("issued", issued.into());
            }
            out.push("Observation", po);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn report(
        &self,
        out: &mut Chart,
        receipt: &mut Receipt,
        cx: &Context,
        eid: &str,
        i: usize,
        rep: &Json,
        ev: &Json,
    ) {
        let performed = or(rep.get("performed"), ev.get("start"));
        if local_dt(performed) >= cx.cutoff {
            return;
        }
        let study = rep.get("study");
        let entry = member(self.c.get("reports"), study);
        let mut rng = PyRandom::new(u128::from(hex_prefix(&format!("{}{eid}{i}", cx.mrn), 8)));
        let lower = if study.truthy() {
            string(study).to_lowercase()
        } else {
            String::new()
        };
        let slow = ["pathology", "cytology"].iter().any(|w| lower.contains(w));
        let at = local_dt(performed);
        let delayed = if slow {
            let days = 3 + rng.randint(0, 5);
            let hours = rng.randint(0, 8);
            time::add(time::add(at, time::days(days)), time::hours(hours))
        } else {
            time::add(at, time::minutes(50 + rng.randint(0, 600)))
        };
        let issued_local = if rep.get("issued").truthy() {
            local_dt(rep.get("issued"))
        } else {
            delayed
        };
        let pending = issued_local >= cx.cutoff;
        let (code, cat) = if entry.truthy() {
            let category = entry.get("category");
            (
                obj![("coding", vec![coding(entry)]), ("text", study.clone())],
                obj![
                    ("system", V2_0074),
                    ("code", category.get("code").clone()),
                    ("display", category.get("display").clone()),
                ],
            )
        } else {
            receipt
                .uncoded
                .push(format!("DiagnosticReport:{}", py_str(study)));
            let fallback = Json::from("Diagnostic study");
            (
                obj![("text", get_or(rep, "study", &fallback).clone())],
                obj![("system", V2_0074), ("code", "OTH"), ("display", "Other")],
            )
        };
        let eff = to_utc(performed);
        let mut dr = obj![
            ("resourceType", "DiagnosticReport"),
            (
                "id",
                opaque("diagrep", &[cx.mrn, eid, "rep", &i.to_string()])
            ),
            ("meta", meta(&eff)),
            ("status", if pending { "registered" } else { "final" }),
            ("category", vec![obj![("coding", vec![cat])]]),
            ("code", code),
            ("subject", cx.subject.clone()),
            ("effectiveDateTime", eff),
        ];
        if pending {
            receipt
                .pending
                .push(format!("{eid}:{} pending at cutoff", py_str(study)));
        } else {
            dr.set("issued", time::to_utc(issued_local).into());
            if rep.get("conclusion").truthy() {
                dr.set("conclusion", rep.get("conclusion").clone());
            }
            let text = format!("{}\n", py_strip(string(rep.get("text"))));
            dr.set(
                "presentedForm",
                vec![obj![
                    ("contentType", "text/plain; charset=utf-8"),
                    ("data", b64(&text)),
                ]]
                .into(),
            );
        }
        out.push("DiagnosticReport", dr);
    }
}

// ---- resource pieces --------------------------------------------------------

fn meta(utc: &str) -> Json {
    obj![("versionId", "1"), ("lastUpdated", last_updated(utc))]
}

/// `{"coding": [{system, code, display}]}`
fn codings(system: &str, code: &str, display: &str) -> Json {
    obj![(
        "coding",
        vec![obj![
            ("system", system),
            ("code", code),
            ("display", display)
        ]]
    )]
}

/// A catalog entry's `{system, code, display}`.
fn coding(entry: &Json) -> Json {
    obj![
        ("system", entry.get("system").clone()),
        ("code", entry.get("code").clone()),
        ("display", entry.get("display").clone()),
    ]
}

fn loinc(code: &str, display: &str, text: &str) -> Json {
    obj![
        (
            "coding",
            vec![obj![
                ("system", "http://loinc.org"),
                ("code", code),
                ("display", display),
            ]]
        ),
        ("text", text),
    ]
}

/// A LOINC code from a catalog entry, with the ledger's label as text.
fn loinc_entry(entry: &Json, text: &Json) -> Json {
    obj![
        (
            "coding",
            vec![obj![
                ("system", "http://loinc.org"),
                ("code", entry.get("code").clone()),
                ("display", entry.get("display").clone()),
            ]]
        ),
        ("text", text.clone()),
    ]
}

fn derived(value: Json) -> Json {
    obj![("value", value), ("_derived", true)]
}

// ---- Python semantics ---------------------------------------------------------

static NONE: Json = Json::Null;

/// `table.get(key)` for a key that is a JSON value: only strings can match.
fn member<'a>(table: &'a Json, key: &Json) -> &'a Json {
    key.as_str().map_or(&NONE, |k| table.get(k))
}

/// Python's `a or b`.
fn or<'a>(a: &'a Json, b: &'a Json) -> &'a Json {
    if a.truthy() { a } else { b }
}

/// Python's `d.get(key, default)`: the default only when the key is absent.
fn get_or<'a>(d: &'a Json, key: &str, default: &'a Json) -> &'a Json {
    if d.contains(key) { d.get(key) } else { default }
}

/// A value the ledger must give as a string (Python would raise otherwise).
fn string(v: &Json) -> &str {
    v.as_str()
        .unwrap_or_else(|| panic!("expected a string, got {v:?}"))
}

fn local_dt(stamp: &Json) -> Local {
    time::local(string(stamp)).unwrap_or_else(|| panic!("not a ledger time: {stamp:?}"))
}

fn time_utc(stamp: &str) -> String {
    time::stamp_to_utc(stamp).unwrap_or_else(|| panic!("not a ledger time: {stamp}"))
}

/// Eastern wall-clock ledger stamp -> `YYYY-MM-DDTHH:MM:SS+00:00`.
fn to_utc(stamp: &Json) -> String {
    time_utc(string(stamp))
}

/// `int(sha256(text).hexdigest()[:n], 16)`
fn hex_prefix(text: &str, n: usize) -> u64 {
    u64::from_str_radix(&sha256_hex(text)[..n], 16).expect("hex")
}

/// `s[:n]` by characters.
fn head(s: &str, n: usize) -> &str {
    s.char_indices().nth(n).map_or(s, |(at, _)| &s[..at])
}

/// `s[n:]` by characters.
fn tail(s: &str, n: usize) -> &str {
    s.char_indices().nth(n).map_or("", |(at, _)| &s[at..])
}

/// `str.strip()`: Python's whitespace is Unicode White_Space plus the
/// information separators U+001C..U+001F.
fn py_strip(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c))
}

/// `isinstance(v, (int, float))` as a float; a bool is an int in Python.
fn number(v: &Json) -> Option<f64> {
    match v {
        Json::Bool(b) => Some(f64::from(u8::from(*b))),
        _ => v.as_f64(),
    }
}

/// The lab's `num(t)` lambda: the member's numeric value.
fn num(values: &HashMap<&str, Json>, t: &str) -> Option<f64> {
    values
        .get(t)
        .and_then(|r| number(r.get("value")))
        .filter(|v| *v != 0.0) // used only where Python tested its truthiness
}

/// `num(t)` where Python compared it with None rather than testing truthiness.
fn num_or_zero(values: &HashMap<&str, Json>, t: &str) -> Option<f64> {
    values.get(t).and_then(|r| number(r.get("value")))
}

/// `round(x, n)`: correctly rounded from the exact binary value, ties to even.
fn py_round(x: f64, n: usize) -> f64 {
    format!("{x:.n$}").parse().expect("formatted float")
}

/// `int(round(x))`
fn round_int(x: f64) -> Json {
    Json::Int(x.round_ties_even() as i64)
}

/// `min(a, b)`: `b` only when it is smaller.
fn py_min(a: f64, b: f64) -> f64 {
    if b < a { b } else { a }
}

/// `max(a, b)`: `b` only when it is larger.
fn py_max(a: f64, b: f64) -> f64 {
    if b > a { b } else { a }
}

/// `float(v)`, or None where Python raised TypeError or ValueError.
fn py_float(v: &Json) -> Option<f64> {
    match v {
        Json::Str(s) => py_strip(s).parse().ok(),
        _ => number(v),
    }
}

/// A Python number kept as int or float, for `sum`.
#[derive(Clone, Copy)]
enum PyNum {
    Int(i64),
    Float(f64),
}

impl PyNum {
    fn of(v: &Json) -> Option<PyNum> {
        match v {
            Json::Int(i) => Some(PyNum::Int(*i)),
            Json::Bool(b) => Some(PyNum::Int(i64::from(*b))),
            Json::Float(f) => Some(PyNum::Float(*f)),
            _ => None,
        }
    }

    fn truthy(self) -> bool {
        match self {
            PyNum::Int(i) => i != 0,
            PyNum::Float(f) => f != 0.0,
        }
    }
}

/// CPython 3.14 `sum()`: exact integer addition until the first float, then
/// Neumaier-compensated float addition for the rest.
fn py_sum(items: &[PyNum]) -> f64 {
    let mut int_total = 0i64;
    let mut rest = items.iter();
    let (mut hi, mut lo) = loop {
        match rest.next() {
            None => return int_total as f64,
            Some(PyNum::Int(i)) => int_total += i,
            Some(PyNum::Float(f)) => break (int_total as f64 + f, 0.0f64),
        }
    };
    for item in rest {
        let x = match *item {
            PyNum::Int(i) => i as f64,
            PyNum::Float(f) => f,
        };
        let t = hi + x;
        if hi.abs() >= x.abs() {
            lo += (hi - t) + x;
        } else {
            lo += (x - t) + hi;
        }
        hi = t;
    }
    if lo != 0.0 && lo.is_finite() {
        hi + lo
    } else {
        hi
    }
}

/// `f"{x:g}"`: six significant digits, trailing zeros removed, scientific
/// notation outside 1e-4 <= |x| < 1e6.
fn fmt_g(x: f64) -> String {
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf" } else { "-inf" }.into();
    }
    if x == 0.0 {
        return if x.is_sign_negative() { "-0" } else { "0" }.into();
    }
    let sci = format!("{x:.5e}");
    let (mantissa, exp) = sci.split_once('e').expect("exponent");
    let exp: i32 = exp.parse().expect("exponent digits");
    let trim = |s: &str| s.trim_end_matches('0').trim_end_matches('.').to_string();
    if (-4..6).contains(&exp) {
        trim(&format!("{x:.*}", (5 - exp) as usize))
    } else {
        let sign = if exp < 0 { '-' } else { '+' };
        format!("{}e{sign}{:02}", trim(mantissa), exp.abs())
    }
}

/// `str(v)` / an f-string field.
fn py_str(v: &Json) -> String {
    match v {
        Json::Str(s) => s.clone(),
        Json::Null => "None".into(),
        Json::Bool(b) => if *b { "True" } else { "False" }.into(),
        Json::Int(i) => i.to_string(),
        Json::Float(f) if f.is_nan() => "nan".into(),
        Json::Float(f) if f.is_infinite() => if *f > 0.0 { "inf" } else { "-inf" }.into(),
        Json::Float(f) => json::float_repr(*f),
        Json::Arr(_) | Json::Obj(_) => py_repr(v),
    }
}

/// `repr(v)` for containers; strings are quoted the way Python quotes
/// printable text.
fn py_repr(v: &Json) -> String {
    match v {
        Json::Str(s) => {
            let quote = if s.contains('\'') && !s.contains('"') {
                '"'
            } else {
                '\''
            };
            let mut out = String::from(quote);
            for c in s.chars() {
                match c {
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c if c == quote => {
                        out.push('\\');
                        out.push(c);
                    }
                    c if (c as u32) < 0x20 || c as u32 == 0x7f => {
                        out.push_str(&format!("\\x{:02x}", c as u32));
                    }
                    c => out.push(c),
                }
            }
            out.push(quote);
            out
        }
        Json::Arr(items) => format!(
            "[{}]",
            items.iter().map(py_repr).collect::<Vec<_>>().join(", ")
        ),
        Json::Obj(members) => format!(
            "{{{}}}",
            members
                .iter()
                .map(|(k, v)| format!("{}: {}", py_repr(&Json::from(k.as_str())), py_repr(v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => py_str(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn python_formatting_helpers() {
        // values checked against CPython 3.14
        assert_eq!(fmt_g(0.0005), "0.0005");
        assert_eq!(fmt_g(1234565.0), "1.23456e+06");
        assert_eq!(fmt_g(5e-05), "5e-05");
        assert_eq!(fmt_g(123456.5), "123456");
        assert_eq!(fmt_g(-0.0), "-0");
        assert_eq!(fmt_g(0.05 * 100.0), "5");
        assert_eq!(py_round(0.125, 2), 0.12);
        assert_eq!(py_round(2.675, 2), 2.67);
        assert_eq!(
            py_sum(&[PyNum::Float(1e16), PyNum::Int(1), PyNum::Int(1)]),
            1e16 + 2.0
        );
        assert_eq!(py_sum(&[PyNum::Float(0.1); 10]), 1.0);
        assert_eq!(py_strip("\u{1c} a b\u{a0}\n"), "a b");
        assert_eq!(head("2026-08-27T12:00", 10), "2026-08-27");
        assert_eq!(tail("MRN00000006", 3), "00000006");
    }
}
