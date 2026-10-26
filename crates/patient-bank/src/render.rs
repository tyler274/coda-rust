//! Translate a patient's ledger into strict FHIR R4, as of a moment.
//!
//! The ledger is the clinical source of truth; this module only encodes it, in the
//! shape of the Coda Agents Roger fixture and coda-gym's chart-build contract. The
//! bank chart (as of the practice's current time) and any earlier snapshot come
//! from the same code: `Timeline` says when each record happened and what was true
//! at the moment, and a record exists only once its own time has passed.
//!
//! Code here never chooses a diagnosis, drug or result the ledger did not state:
//! every lab value is already in the measured ledger (see `draw`). BMI and
//! anything left uncoded are listed in the render receipt.

use std::sync::LazyLock;

use base64::Engine as _;
use regex::{Captures, Regex};
use serde::Serialize;
use serde_json::Number;
use sha2::{Digest, Sha256};

use crate::catalog::{Catalog, Category, Entry, RXNORM};
use crate::fhir::*;
use crate::labs::{self, LabContext, OBS_CAT, V2_0074};
use crate::ledger::{
    self, AllergyType, ConditionItem, Criticality, DocStatus, Event, Kind, MaritalStatus,
    MedicationAction, OrderAction, OrderCategory, Severity, Stamp, Terms, num,
};
use crate::seeds::Seed;
use crate::time;
use crate::timeline::{OrderStatus, ProblemStatus, Timeline};

pub const MRN_SYSTEM: &str = "urn:oid:2.16.840.1.113883.19.5";
const US_CORE: &str = "http://hl7.org/fhir/us/core/StructureDefinition";
const CDC_RACE: &str = "urn:oid:2.16.840.1.113883.6.238";
const CLINICAL: &str = "http://terminology.hl7.org/CodeSystem/condition-clinical";

/// A vital sign: (LOINC code, display, text, UCUM code, unit).
type Vital = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);
const HR: Vital = ("8867-4", "Heart rate", "Heart Rate", "/min", "/min");
const RR: Vital = (
    "9279-1",
    "Respiratory rate",
    "Respiratory Rate",
    "/min",
    "/min",
);
const TEMP: Vital = ("8310-5", "Body temperature", "Temperature", "Cel", "C");
const SPO2: Vital = (
    "59408-5",
    "Oxygen saturation in Arterial blood by Pulse oximetry",
    "Oxygen Saturation",
    "%",
    "%",
);
const WEIGHT: Vital = ("29463-7", "Body weight", "Weight", "kg", "kg");
const HEIGHT: Vital = ("8302-2", "Body height", "Height", "cm", "cm");
const BMI: Vital = (
    "39156-5",
    "Body mass index (BMI) [Ratio]",
    "Body Mass Index",
    "kg/m2",
    "kg/m2",
);
const PEAK_FLOW: Vital = (
    "33452-4",
    "Maximum expiratory gas flow Respiratory system airway by Peak flow meter",
    "Peak Expiratory Flow",
    "L/min",
    "L/min",
);
/// Every vital-sign coding, for terminology checks.
pub const VITAL_CODES: [(&str, &str); 11] = [
    (HR.0, HR.1),
    (RR.0, RR.1),
    (TEMP.0, TEMP.1),
    (SPO2.0, SPO2.1),
    (WEIGHT.0, WEIGHT.1),
    (HEIGHT.0, HEIGHT.1),
    (BMI.0, BMI.1),
    (PEAK_FLOW.0, PEAK_FLOW.1),
    BP,
    SBP,
    DBP,
];
const BP: (&str, &str) = ("85354-9", "Blood pressure panel with all children optional");
const SBP: (&str, &str) = ("8480-6", "Systolic blood pressure");
const DBP: (&str, &str) = ("8462-4", "Diastolic blood pressure");

/// Accurate LOINC document classes only; other note types stay text-only.
pub const DOC_TYPE_CODES: [(&str, (&str, &str)); 3] = [
    (
        "History and Physical",
        ("34117-2", "History and physical note"),
    ),
    ("Discharge Summary", ("18842-5", "Discharge summary")),
    ("Consults", ("11488-4", "Consult note")),
];

const LANG: &[(&str, &str)] = &[
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
const RACE: &[(&str, &str)] = &[
    ("White", "2106-3"),
    ("Black or African American", "2054-5"),
    ("Asian", "2028-9"),
    ("Other Race", "2131-1"),
];
const ETHNICITY: &[(&str, &str)] = &[
    ("Hispanic or Latino", "2135-2"),
    ("Not Hispanic or Latino", "2186-5"),
];

fn table<'a>(t: &'a [(&str, &str)], key: &str) -> Option<&'a str> {
    t.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

/// A stable resource id: the prefix and 12 hex digits of SHA-256 over the parts.
pub fn opaque(prefix: &str, parts: &[&str]) -> String {
    let digest = Sha256::digest(parts.join("|").as_bytes());
    let hex: String = digest.iter().take(6).map(|b| format!("{b:02x}")).collect();
    format!("{prefix}-{hex}")
}

pub fn b64(text: &str) -> String {
    base64::engine::general_purpose::STANDARD.encode(text.as_bytes())
}

fn attachment(text: &str) -> Attachment {
    Attachment {
        content_type: "text/plain; charset=utf-8",
        data: b64(text),
    }
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

/// An RxNorm name as a clinician reads it: no application number or pack size,
/// lower-case units and dose forms, topical strengths as percent.
pub fn friendly_drug(name: &str) -> String {
    let name = PACK.replace(name, "");
    let name = if TOPICAL.is_match(&name) {
        STRENGTH
            .replace_all(&name, |m: &Captures| {
                let scale = if m[2].eq_ignore_ascii_case("MG/MG") {
                    100.0
                } else {
                    0.1
                };
                let pct = m[1].parse::<f64>().expect("strength number") * scale;
                let pct = format!("{pct:.6}");
                format!("{}%", pct.trim_end_matches('0').trim_end_matches('.'))
            })
            .into_owned()
    } else {
        name.into_owned()
    };
    let text = UNITS.replace_all(&name, |m: &Captures| m[1].to_lowercase());
    FORMS
        .replace_all(&text, |m: &Captures| m[1].to_lowercase())
        .into_owned()
}

/// What one render filled, derived, could not code, left pending or skipped.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Receipt {
    pub derived: Vec<String>,
    pub uncoded: Vec<String>,
    pub pending: Vec<String>,
    pub skipped: Vec<String>,
}

fn coding(e: &Entry) -> Coding {
    Coding {
        system: e.system.clone(),
        code: e.code.clone(),
        display: e.display.clone(),
    }
}

fn status(system: &str, code: &str, display: &str) -> CodeableConcept {
    CodeableConcept::coding_only(Coding::new(system, code, display))
}

pub struct Renderer<'a> {
    pub catalog: &'a Catalog,
    /// The unit's verified out-of-catalog codes.
    pub terms: &'a Terms,
}

impl Renderer<'_> {
    fn condition_code(&self, item: &ConditionItem, receipt: &mut Receipt) -> CodeableConcept {
        if let Some(e) = self.catalog.conditions.get(&item.label) {
            return CodeableConcept::coded(coding(e), &item.label);
        }
        let key = format!("{}|{}", item.label, item.icd10.as_deref().unwrap_or(""));
        if let Some(v) = self.terms.conditions.get(&key) {
            let system = &self.catalog.systems["icd10cm"];
            return CodeableConcept::coded(Coding::new(system, &v.code, &v.display), &item.label);
        }
        receipt.uncoded.push(format!("Condition:{}", item.label));
        CodeableConcept::text(&item.label)
    }

    fn medication_concept(
        &self,
        item: &ledger::MedicationItem,
        receipt: &mut Receipt,
    ) -> CodeableConcept {
        if let Some(name) = &item.drug
            && let Some(e) = self.catalog.medications.get(name)
        {
            return CodeableConcept::coded(coding(e), &friendly_drug(name));
        }
        let text = item.key().unwrap_or("unspecified medication");
        if let Some(v) = self.terms.medications.get(text) {
            return CodeableConcept::coded(Coding::new(RXNORM, &v.code, &v.display), text);
        }
        receipt.uncoded.push(format!("MedicationRequest:{text}"));
        CodeableConcept::text(text)
    }

    fn patient(&self, seed: &Seed, p: &ledger::Patient) -> Patient {
        let mut telecom = vec![ContactPoint {
            system: "phone",
            value: seed.phone.clone(),
            use_: "mobile",
        }];
        if let Some(email) = &seed.email {
            telecom.push(ContactPoint {
                system: "email",
                value: email.clone(),
                use_: "home",
            });
        }
        let marital = p.marital_status.map(|m| {
            let (code, display) = match m {
                MaritalStatus::Married => ("M", "Married"),
                MaritalStatus::Single => ("S", "Never Married"),
                MaritalStatus::Divorced => ("D", "Divorced"),
                MaritalStatus::Widowed => ("W", "Widowed"),
                MaritalStatus::Partnered => ("T", "Domestic partner"),
                MaritalStatus::Separated => ("L", "Legally Separated"),
            };
            CodeableConcept::coded(
                Coding::new(
                    "http://terminology.hl7.org/CodeSystem/v3-MaritalStatus",
                    code,
                    display,
                ),
                display,
            )
        });
        let lang = &seed.preferred_language;
        let communication = table(LANG, lang)
            .map(|code| Communication {
                language: CodeableConcept::coded(Coding::new("urn:ietf:bcp:47", code, lang), lang),
                preferred: true,
            })
            .into_iter()
            .collect();
        let sub = |url: &str, coding: Option<Coding>, text: Option<&str>| Extension {
            url: url.into(),
            extension: Vec::new(),
            value_coding: coding,
            value_string: text.map(String::from),
        };
        let mut extension = Vec::new();
        if seed.race != "Declined" {
            let mut parts = Vec::new();
            if let Some(code) = table(RACE, &seed.race) {
                parts.push(sub(
                    "ombCategory",
                    Some(Coding::new(CDC_RACE, code, &seed.race)),
                    None,
                ));
            }
            parts.push(sub("text", None, Some(&seed.race)));
            extension.push(Extension {
                extension: parts,
                ..sub(&format!("{US_CORE}/us-core-race"), None, None)
            });
        }
        if let Some(code) = table(ETHNICITY, &seed.ethnicity) {
            let parts = vec![
                sub(
                    "ombCategory",
                    Some(Coding::new(CDC_RACE, code, &seed.ethnicity)),
                    None,
                ),
                sub("text", None, Some(&seed.ethnicity)),
            ];
            extension.push(Extension {
                extension: parts,
                ..sub(&format!("{US_CORE}/us-core-ethnicity"), None, None)
            });
        }
        Patient {
            resource_type: "Patient",
            id: seed.mrn.clone(),
            meta: Meta::on(&time::utc(time::as_of())),
            identifier: vec![Identifier {
                system: MRN_SYSTEM,
                value: seed.mrn[3..].to_string(),
            }],
            name: vec![HumanName {
                use_: "official",
                family: seed.family.clone(),
                given: vec![seed.given.clone()],
            }],
            gender: seed.sex.as_str(),
            birth_date: seed.birth_date.0.to_string(),
            telecom,
            address: vec![Address {
                use_: "home",
                line: vec![
                    p.address_line
                        .clone()
                        .unwrap_or_else(|| "1 Main Street".into()),
                ],
                city: seed.town.clone(),
                state: "MA",
                postal_code: seed.postal_code.clone(),
                country: "US",
            }],
            marital_status: marital,
            communication,
            extension,
        }
    }

    /// The chart as of `at` (Eastern wall clock).
    pub fn render(&self, seed: &Seed, p: &ledger::Patient, at: Stamp) -> (Chart, Receipt) {
        let tl = Timeline::new(p);
        let mrn = seed.mrn.as_str();
        let subject = Reference::to("Patient", mrn);
        let female = seed.female();
        let mut receipt = Receipt::default();
        let mut chart = Chart {
            patient: self.patient(seed, p),
            conditions: Vec::new(),
            medications: Vec::new(),
            observations: Vec::new(),
            reports: Vec::new(),
            documents: Vec::new(),
            procedures: Vec::new(),
            allergies: Vec::new(),
            encounters: Vec::new(),
            immunizations: Vec::new(),
            service_requests: Vec::new(),
        };
        // The encounter an event's records belong to, once it has begun.
        let encounter_of = |ev: &Event| {
            (ev.start < at && ev.appointment.is_none() && ev.kind.encounter().is_some())
                .then(|| Reference::to("Encounter", &opaque("enc", &[mrn, &ev.id])))
        };

        // Problem list: one row per episode, its status from that episode's history.
        for prob in tl.problems.iter().filter(|p| p.recorded < at) {
            let recorded = time::utc(prob.recorded);
            let onset = match prob.item.onset {
                Some(day) if day.0 < prob.recorded.date() => day.0.to_string(),
                _ => recorded.clone(),
            };
            let (clinical, abatement) = match prob.status(at) {
                ProblemStatus::Active => (status(CLINICAL, "active", "Active"), None),
                ProblemStatus::Resolved(when) => (
                    status(CLINICAL, "resolved", "Resolved"),
                    Some(time::utc(when)),
                ),
            };
            chart.conditions.push(Condition {
                resource_type: "Condition",
                id: opaque("cond", &[mrn, &prob.event.id, prob.label]),
                meta: Meta::on(&recorded),
                clinical_status: clinical,
                verification_status: status(
                    "http://terminology.hl7.org/CodeSystem/condition-ver-status",
                    "confirmed",
                    "Confirmed",
                ),
                category: vec![CodeableConcept::coded(
                    Coding::new(
                        "http://terminology.hl7.org/CodeSystem/condition-category",
                        "problem-list-item",
                        "Problem List Item",
                    ),
                    "Problem List Item",
                )],
                code: self.condition_code(prob.item, &mut receipt),
                subject: subject.clone(),
                onset_date_time: onset,
                recorded_date: recorded,
                abatement_date_time: abatement,
                note: prob
                    .item
                    .note
                    .iter()
                    .filter(|t| !t.is_empty())
                    .map(|t| Annotation { text: t.clone() })
                    .collect(),
            });
        }

        // Medication orders: each start or reported medication, stopped by a later stop.
        for o in tl
            .orders
            .iter()
            .filter(|o| o.event.start < at && o.authored < at)
        {
            let item = o.item;
            let authored = time::utc(o.authored);
            let (state, reason) = match o.status(at) {
                OrderStatus::Active => ("active", None),
                OrderStatus::Completed => ("completed", None),
                OrderStatus::Stopped(when) => (
                    "stopped",
                    Some(CodeableConcept::text(&format!(
                        "Discontinued {}",
                        when.date()
                    ))),
                ),
            };
            let mut dose = Dosage {
                text: [&item.sig, &item.frequency]
                    .into_iter()
                    .flatten()
                    .find(|s| !s.is_empty())
                    .cloned()
                    .unwrap_or_default(),
                ..Dosage::default()
            };
            dose.timing = item.frequency.as_ref().map(|f| Timing {
                code: CodeableConcept::text(f),
            });
            if let Some(d) = &item.dose {
                let q = Quantity {
                    value: d.value.clone(),
                    unit: Some(d.unit.clone()),
                    system: None,
                    code: None,
                };
                dose.dose_and_rate = vec![DoseAndRate { dose_quantity: q }];
            }
            dose.route = item.route.as_ref().map(|r| CodeableConcept::text(r));
            let has_dose = !dose.text.is_empty()
                || dose.timing.is_some()
                || !dose.dose_and_rate.is_empty()
                || dose.route.is_some();
            chart.medications.push(MedicationRequest {
                resource_type: "MedicationRequest",
                id: opaque(
                    "medrx",
                    &[
                        mrn,
                        &o.event.id,
                        &o.index.to_string(),
                        item.key().unwrap_or(""),
                    ],
                ),
                meta: Meta::on(&authored),
                status: state,
                intent: "order",
                medication_codeable_concept: self.medication_concept(item, &mut receipt),
                subject: subject.clone(),
                authored_on: authored,
                reported_boolean: (item.action == MedicationAction::Historical).then_some(true),
                status_reason: reason,
                dosage_instruction: if has_dose { vec![dose] } else { Vec::new() },
                reason_code: item
                    .reason
                    .iter()
                    .filter(|r| !r.is_empty())
                    .map(|r| CodeableConcept::text(r))
                    .collect(),
            });
        }

        let mut height: Option<f64> = None;
        let vital_cat =
            CodeableConcept::coding_only(Coding::new(OBS_CAT, "vital-signs", "Vital Signs"));
        for ev in tl.events.iter().copied().filter(|e| e.start < at) {
            let eid = ev.id.as_str();
            let start = time::utc(ev.start);
            let enc = encounter_of(ev);
            if let (Some(r), Some((class, display, label))) = (&enc, ev.kind.encounter()) {
                let ended = ev.end.filter(|e| *e < at);
                chart.encounters.push(Encounter {
                    resource_type: "Encounter",
                    id: r.reference["Encounter/".len()..].to_string(),
                    meta: Meta::on(&start),
                    status: if ev.end.is_some_and(|e| e >= at) {
                        "in-progress"
                    } else {
                        "finished"
                    },
                    class: Coding::new(ACT_CODE, class, display),
                    kind: vec![CodeableConcept::text(label)],
                    service_type: CodeableConcept::text(ev.service()),
                    subject: subject.clone(),
                    period: Period {
                        start: start.clone(),
                        end: ended.map(time::utc),
                    },
                    reason_code: ev
                        .reason
                        .iter()
                        .filter(|r| !r.is_empty())
                        .map(|r| CodeableConcept::text(r))
                        .collect(),
                });
            }

            for (i, a) in ev.allergies.iter().enumerate() {
                let entry = self.catalog.allergens.get(&a.substance);
                let code = match entry {
                    Some(e) => CodeableConcept::coded(coding(e), &a.substance),
                    None => {
                        receipt
                            .uncoded
                            .push(format!("AllergyIntolerance:{}", a.substance));
                        CodeableConcept::text(&a.substance)
                    }
                };
                let category = match entry.and_then(|e| e.category.as_ref()) {
                    Some(Category::Text(c)) => c.clone(),
                    _ => "medication".into(),
                };
                let reaction = a.reaction.as_ref().map(|r| Reaction {
                    manifestation: vec![match self.catalog.reactions.get(r) {
                        Some(e) => CodeableConcept::coded(coding(e), r),
                        None => CodeableConcept::text(r),
                    }],
                    severity: a.severity.map(|s| match s {
                        Severity::Mild => "mild",
                        Severity::Moderate => "moderate",
                        Severity::Severe => "severe",
                    }),
                });
                chart.allergies.push(AllergyIntolerance {
                    resource_type: "AllergyIntolerance",
                    id: opaque("allergy", &[mrn, eid, &i.to_string()]),
                    meta: Meta::on(&start),
                    clinical_status: status(
                        "http://terminology.hl7.org/CodeSystem/allergyintolerance-clinical",
                        "active",
                        "Active",
                    ),
                    verification_status: status(
                        "http://terminology.hl7.org/CodeSystem/allergyintolerance-verification",
                        "confirmed",
                        "Confirmed",
                    ),
                    kind: match a.kind {
                        Some(AllergyType::Intolerance) => "intolerance",
                        _ => "allergy",
                    },
                    category: vec![category],
                    criticality: match a.criticality {
                        Some(Criticality::High) => "high",
                        Some(Criticality::UnableToAssess) => "unable-to-assess",
                        _ => "low",
                    },
                    code,
                    patient: subject.clone(),
                    recorded_date: start.clone(),
                    reaction: reaction.into_iter().collect(),
                });
            }

            let sets = ev.vitals.sets();
            for (vi, set) in sets.iter().enumerate() {
                let when = tl.vitals_time(ev, set);
                if when >= at {
                    continue;
                }
                let vi = vi.to_string();
                let tag: Vec<&str> = if sets.len() == 1 && set.time.is_none() {
                    vec![eid]
                } else {
                    vec![eid, &vi]
                };
                let when = time::utc(when);
                let id = |what: &str| {
                    let mut parts = vec![mrn];
                    parts.extend(&tag);
                    parts.push(what);
                    opaque("obs", &parts)
                };
                let loinc = |code: &str, display: &str, text: &str| {
                    CodeableConcept::coded(Coding::new(LOINC, code, display), text)
                };
                if let Some([sys, dia]) = &set.bp {
                    let mut o = Observation::new(
                        id("bp"),
                        "final",
                        vital_cat.clone(),
                        loinc(BP.0, BP.1, "Blood Pressure"),
                        &subject,
                        when.clone(),
                    );
                    o.encounter = enc.clone();
                    o.component = vec![
                        Component {
                            code: loinc(SBP.0, SBP.1, "Systolic Blood Pressure"),
                            value_quantity: Quantity::ucum(sys.clone(), "mmHg", "mm[Hg]"),
                        },
                        Component {
                            code: loinc(DBP.0, DBP.1, "Diastolic Blood Pressure"),
                            value_quantity: Quantity::ucum(dia.clone(), "mmHg", "mm[Hg]"),
                        },
                    ];
                    chart.observations.push(o);
                }
                if let Some(h) = &set.height_cm {
                    height = Some(num(h));
                }
                let bmi = match (&set.weight_kg, height) {
                    (Some(w), Some(h)) if h != 0.0 => {
                        receipt
                            .derived
                            .push(format!("{}:Body Mass Index", tag.join(":")));
                        Some(labs::rounded(num(w) / (h / 100.0).powi(2), 1))
                    }
                    _ => None,
                };
                let readings: [(&str, Vital, Option<Number>); 8] = [
                    ("hr", HR, set.hr.clone()),
                    ("rr", RR, set.rr.clone()),
                    ("temp_c", TEMP, set.temp_c.clone()),
                    ("spo2", SPO2, set.spo2.clone()),
                    ("weight_kg", WEIGHT, set.weight_kg.clone()),
                    ("height_cm", HEIGHT, set.height_cm.clone()),
                    ("bmi", BMI, bmi),
                    ("peak_flow", PEAK_FLOW, set.peak_flow.clone()),
                ];
                for (key, (code, display, text, ucum, unit), value) in readings {
                    let Some(value) = value else { continue };
                    let mut o = Observation::new(
                        id(key),
                        "final",
                        vital_cat.clone(),
                        loinc(code, display, text),
                        &subject,
                        when.clone(),
                    );
                    o.encounter = enc.clone();
                    o.value_quantity = Some(Quantity::ucum(value, unit, ucum));
                    chart.observations.push(o);
                }
            }

            let cx = LabContext {
                catalog: self.catalog,
                timeline: &tl,
                subject: subject.clone(),
                female,
                at,
            };
            for (i, lab) in ev.labs.iter().enumerate() {
                let out = labs::render(&cx, ev, i, lab, &mut receipt);
                chart
                    .observations
                    .extend(out.observations.into_iter().map(|mut o| {
                        o.encounter = enc.clone();
                        o
                    }));
                chart.reports.extend(out.report.map(|mut r| {
                    r.encounter = enc.clone();
                    r
                }));
            }

            for (i, s) in ev.surveys.iter().enumerate() {
                let Some(entry) = self.catalog.surveys.get(&s.survey) else {
                    receipt.skipped.push(format!("{eid}:survey:{}", s.survey));
                    continue;
                };
                let when = s.date.unwrap_or(ev.start);
                if when >= at {
                    continue;
                }
                let cat = CodeableConcept::coding_only(Coding::new(OBS_CAT, "survey", "Survey"));
                let code = CodeableConcept::coded(coding(entry), &s.survey);
                let mut o = Observation::new(
                    opaque("obs", &[mrn, eid, "survey", &i.to_string()]),
                    "final",
                    cat,
                    code,
                    &subject,
                    time::utc(when),
                );
                o.encounter = enc.clone();
                o.value_quantity = Some(Quantity::ucum(s.value.clone(), "{score}", "{score}"));
                chart.observations.push(o);
            }
            for (i, s) in ev.social.iter().enumerate() {
                let Some(entry) = self
                    .catalog
                    .social
                    .get(&s.item)
                    .filter(|_| !s.value.is_empty())
                else {
                    receipt.skipped.push(format!("{eid}:social:{}", s.item));
                    continue;
                };
                let cat = CodeableConcept::coding_only(Coding::new(
                    OBS_CAT,
                    "social-history",
                    "Social History",
                ));
                let code = CodeableConcept::coded(coding(entry), &s.item);
                let mut o = Observation::new(
                    opaque("obs", &[mrn, eid, "social", &i.to_string()]),
                    "final",
                    cat,
                    code,
                    &subject,
                    start.clone(),
                );
                o.encounter = enc.clone();
                o.value_codeable_concept = Some(CodeableConcept::text(&s.value));
                chart.observations.push(o);
            }

            for (i, rep) in ev.reports.iter().enumerate() {
                let performed = Timeline::performed(ev, rep);
                if performed >= at {
                    continue;
                }
                let issued = tl.report_issued(ev, i, rep);
                let pending = issued >= at;
                let (code, cat) = match self.catalog.reports.get(&rep.study) {
                    Some(e) => {
                        let section = match &e.category {
                            Some(Category::Section { code, display }) => {
                                Coding::new(V2_0074, code, display)
                            }
                            _ => Coding::new(V2_0074, "OTH", "Other"),
                        };
                        (CodeableConcept::coded(coding(e), &rep.study), section)
                    }
                    None => {
                        receipt
                            .uncoded
                            .push(format!("DiagnosticReport:{}", rep.study));
                        (
                            CodeableConcept::text(&rep.study),
                            Coding::new(V2_0074, "OTH", "Other"),
                        )
                    }
                };
                if pending {
                    receipt
                        .pending
                        .push(format!("{eid}:{} pending at cutoff", rep.study));
                }
                let effective = time::utc(performed);
                chart.reports.push(DiagnosticReport {
                    resource_type: "DiagnosticReport",
                    id: opaque("diagrep", &[mrn, eid, "rep", &i.to_string()]),
                    meta: Meta::on(&effective),
                    status: if pending { "registered" } else { "final" },
                    category: vec![CodeableConcept::coding_only(cat)],
                    code,
                    subject: subject.clone(),
                    encounter: enc.clone(),
                    effective_date_time: effective,
                    issued: (!pending).then(|| time::utc(issued)),
                    result: Vec::new(),
                    conclusion: rep.conclusion.clone().filter(|_| !pending),
                    presented_form: if pending {
                        Vec::new()
                    } else {
                        vec![attachment(&format!("{}\n", rep.text.trim()))]
                    },
                });
            }

            for (i, x) in ev.procedures.iter().enumerate() {
                let when = x.performed.unwrap_or(ev.start);
                if when >= at {
                    continue;
                }
                let code = match self.catalog.procedures.get(&x.procedure) {
                    Some(e) => CodeableConcept::coded(coding(e), &x.procedure),
                    None => {
                        receipt.uncoded.push(format!("Procedure:{}", x.procedure));
                        CodeableConcept::text(&x.procedure)
                    }
                };
                chart.procedures.push(procedure(
                    opaque("proc", &[mrn, eid, &i.to_string()]),
                    code,
                    &subject,
                    enc.clone(),
                    when,
                ));
            }
            for (i, imm) in ev.immunizations.iter().enumerate() {
                let when = imm.date.unwrap_or(ev.start);
                if when >= at {
                    continue;
                }
                // History from elsewhere is reported; this practice billed only what it gave.
                let given_here = ev.kind != Kind::OutsideRecords;
                let vaccine_code = match self.catalog.vaccines.get(&imm.vaccine) {
                    Some(e) => CodeableConcept::coded(coding(e), &imm.vaccine),
                    None => {
                        receipt
                            .uncoded
                            .push(format!("Immunization:{}", imm.vaccine));
                        CodeableConcept::text(&imm.vaccine)
                    }
                };
                let occurred = time::utc(when);
                chart.immunizations.push(Immunization {
                    resource_type: "Immunization",
                    id: opaque("imm", &[mrn, eid, &i.to_string()]),
                    meta: Meta::on(&occurred),
                    status: "completed",
                    vaccine_code,
                    patient: subject.clone(),
                    encounter: enc.clone(),
                    occurrence_date_time: occurred,
                    primary_source: given_here,
                });
                if given_here {
                    let e = &self.catalog.procedures["Immunization administration"];
                    let code = CodeableConcept::coded(
                        coding(e),
                        &format!("{} vaccine administration", imm.vaccine),
                    );
                    chart.procedures.push(procedure(
                        opaque("proc", &[mrn, eid, "imm", &i.to_string()]),
                        code,
                        &subject,
                        enc.clone(),
                        when,
                    ));
                }
            }

            for (i, doc) in ev.documents.iter().enumerate() {
                let when = doc.date.unwrap_or(ev.start);
                if when >= at {
                    receipt.pending.push(format!("{eid}:document after cutoff"));
                    continue;
                }
                let day = when.date();
                let header = format!(
                    "{}\nDate: {}\nPatient: {} {} | MRN: {} | Age: {}\nService: {}\n\n",
                    doc.title(),
                    day.strftime("%m/%d/%Y"),
                    seed.given,
                    seed.family,
                    &mrn[3..],
                    time::age_on(seed.birth_date.0, day),
                    ev.service(),
                );
                let text = format!("{header}{}\n", doc.body.trim());
                let kind = doc.kind();
                let mut type_cc = CodeableConcept::text(kind);
                for (key, (code, display)) in DOC_TYPE_CODES {
                    if kind.ends_with(key) {
                        type_cc.coding = vec![Coding::new(LOINC, code, display)];
                    }
                }
                let date = time::utc(when);
                chart.documents.push(DocumentReference {
                    resource_type: "DocumentReference",
                    id: opaque("docref", &[mrn, eid, &i.to_string()]),
                    meta: Meta::on(&date),
                    status: "current",
                    doc_status: if doc.status == Some(DocStatus::Preliminary) {
                        "preliminary"
                    } else {
                        "final"
                    },
                    kind: type_cc,
                    category: vec![CodeableConcept::coding_only(Coding::new(
                        "http://hl7.org/fhir/us/core/CodeSystem/us-core-documentreference-category",
                        "clinical-note",
                        "Clinical Note",
                    ))],
                    subject: subject.clone(),
                    date,
                    author: doc
                        .author
                        .iter()
                        .map(|a| Named { display: a.clone() })
                        .collect(),
                    description: doc.title().to_string(),
                    content: vec![Content {
                        attachment: attachment(&text),
                    }],
                    context: enc.clone().map(|e| DocContext { encounter: vec![e] }),
                });
            }
        }

        // Referrals and tests ordered: open until done or cancelled.
        for o in tl.service_orders.iter().filter(|o| o.placed < at) {
            let item = o.item;
            let what = item.order.as_deref().unwrap_or("Unspecified order");
            let category = match item.category.unwrap_or(OrderCategory::Referral) {
                OrderCategory::Referral => "referral",
                OrderCategory::Lab => "lab",
                OrderCategory::Imaging => "imaging",
                OrderCategory::Procedure => "procedure",
            };
            let known = self
                .catalog
                .procedures
                .get(what)
                .or_else(|| self.catalog.reports.get(what))
                .filter(|e| e.display.is_some());
            let code = match known {
                Some(e) => CodeableConcept::coded(coding(e), what),
                None => CodeableConcept::text(what),
            };
            let authored = time::utc(o.placed);
            chart.service_requests.push(ServiceRequest {
                resource_type: "ServiceRequest",
                id: opaque("sr", &[mrn, &o.event.id, &item.id]),
                meta: Meta::on(&authored),
                status: match o.status(at) {
                    OrderAction::Placed => "active",
                    OrderAction::Done => "completed",
                    OrderAction::Cancelled => "revoked",
                },
                intent: "order",
                category: self
                    .catalog
                    .order_categories
                    .get(category)
                    .map(|e| CodeableConcept::coding_only(coding(e)))
                    .into_iter()
                    .collect(),
                code,
                subject: subject.clone(),
                encounter: encounter_of(o.event),
                authored_on: authored,
                performer_type: item.to.iter().map(|t| CodeableConcept::text(t)).collect(),
                reason_code: item
                    .reason
                    .iter()
                    .filter(|r| !r.is_empty())
                    .map(|r| CodeableConcept::text(r))
                    .collect(),
            });
        }
        (chart, receipt)
    }
}

const ACT_CODE: &str = "http://terminology.hl7.org/CodeSystem/v3-ActCode";

fn procedure(
    id: String,
    code: CodeableConcept,
    subject: &Reference,
    encounter: Option<Reference>,
    when: Stamp,
) -> Procedure {
    let performed = time::utc(when);
    Procedure {
        resource_type: "Procedure",
        id,
        meta: Meta::on(&performed),
        status: "completed",
        code,
        subject: subject.clone(),
        encounter,
        performed_date_time: performed,
    }
}
