//! The authored ledger: a patient's clinical source of truth.
//!
//! These types are the ledger format in `data/authoring-guide.md`, exactly. Parsing
//! is the only shape check: an unknown field, a misspelled action or a stamp that is
//! not `YYYY-MM-DDTHH:MM` fails with the line and column, so a wrongly shaped draft
//! can never lose content on its way to an accepted ledger.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use jiff::civil::{Date, DateTime};
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Number;

use crate::Result;

/// An Eastern wall-clock time, written `YYYY-MM-DDTHH:MM`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stamp(pub DateTime);

impl Stamp {
    pub fn parse(text: &str) -> Option<Stamp> {
        let b = text.as_bytes();
        let shaped = b.len() == 16
            && b.iter().enumerate().all(|(i, c)| match i {
                4 | 7 => *c == b'-',
                10 => *c == b'T',
                13 => *c == b':',
                _ => c.is_ascii_digit(),
            });
        if !shaped {
            return None;
        }
        text.parse::<DateTime>().ok().map(Stamp)
    }

    pub fn date(self) -> Date {
        self.0.date()
    }
}

impl fmt::Display for Stamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.strftime("%Y-%m-%dT%H:%M"))
    }
}

impl<'de> Deserialize<'de> for Stamp {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        Stamp::parse(&text)
            .ok_or_else(|| de::Error::custom(format!("{text:?} is not a 'YYYY-MM-DDTHH:MM' time")))
    }
}

impl Serialize for Stamp {
    fn serialize<S: Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

/// A calendar date, written `YYYY-MM-DD`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day(pub Date);

impl<'de> Deserialize<'de> for Day {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        let shaped = text.len() == 10 && text.as_bytes()[4] == b'-' && text.as_bytes()[7] == b'-';
        match text.parse::<Date>() {
            Ok(day) if shaped => Ok(Day(day)),
            _ => Err(de::Error::custom(format!(
                "{text:?} is not a 'YYYY-MM-DD' date"
            ))),
        }
    }
}

impl Serialize for Day {
    fn serialize<S: Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.collect_str(&self.0)
    }
}

/// An accepted ledger, `ledgers/<unit>.json`, written only by `check`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ledger {
    pub unit: String,
    pub model: String,
    #[serde(default)]
    pub warnings: Vec<String>,
    pub patients: Vec<Patient>,
    /// Out-of-catalog codes verified for this unit when it was accepted. Required:
    /// a ledger accepted before codes were stored here must be checked again.
    pub terms: Terms,
}

impl Ledger {
    pub fn read(path: &Path) -> Result<Ledger> {
        let text = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?)
    }

    pub fn patient(&self, mrn: &str) -> Option<&Patient> {
        self.patients.iter().find(|p| p.mrn == mrn)
    }
}

/// One patient's chart as authored.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Patient {
    pub mrn: String,
    pub given: String,
    pub family: String,
    pub birth_date: Day,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<MaritalStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub events: Vec<Event>,
}

/// One draft part file, `drafts/<unit>/part-NN.json`. `part-01.json` carries the
/// patient fields; later parts carry only `mrn` and `events`.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Part {
    pub patients: Vec<PartPatient>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartPatient {
    pub mrn: String,
    pub given: Option<String>,
    pub family: Option<String>,
    pub birth_date: Option<Day>,
    pub address_line: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub marital_status: Option<MaritalStatus>,
    pub summary: Option<String>,
    #[serde(default)]
    pub events: Vec<Event>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MaritalStatus {
    Married,
    Single,
    Divorced,
    Widowed,
    Partnered,
    Separated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    EstablishCare,
    OfficeVisit,
    AnnualPhysical,
    TelehealthVisit,
    NurseVisit,
    Telephone,
    PortalMessage,
    LabOnly,
    Imaging,
    SpecialistVisit,
    UrgentCare,
    EdVisit,
    HospitalAdmission,
    ProcedureVisit,
    OutsideRecords,
    Pharmacy,
}

impl Kind {
    /// A primary-care visit that must leave a note.
    pub fn needs_note(self) -> bool {
        matches!(
            self,
            Kind::OfficeVisit | Kind::AnnualPhysical | Kind::EstablishCare | Kind::TelehealthVisit
        )
    }
}

/// One contact with the practice and everything it produced.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub id: String,
    pub kind: Kind,
    pub start: Stamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<Stamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ConditionItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub medications: Vec<MedicationItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allergies: Vec<AllergyItem>,
    #[serde(default, skip_serializing_if = "Vitals::is_empty")]
    pub vitals: Vitals,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labs: Vec<Lab>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub surveys: Vec<Survey>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub social: Vec<Social>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reports: Vec<Report>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedures: Vec<ProcedureItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub immunizations: Vec<Immunization>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documents: Vec<Document>,
}

impl Event {
    /// The department that wrote this event's documents.
    pub fn service(&self) -> &str {
        self.service
            .as_deref()
            .unwrap_or("Internal Medicine — Primary Care")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConditionAction {
    Diagnosed,
    History,
    Resolved,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionItem {
    pub label: String,
    pub action: ConditionAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onset: Option<Day>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The author's ICD-10-CM code for a label outside the catalog, verified at check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icd10: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MedicationAction {
    Start,
    Stop,
    Historical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Course {
    Chronic,
    Short,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MedicationItem {
    /// A catalog RxNorm name, exactly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drug: Option<String>,
    /// Generic name, strength and form for a drug outside the catalog, looked up at check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drug_text: Option<String>,
    pub action: MedicationAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dose: Option<Dose>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub course: Option<Course>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// When the order was written, if not at the event's start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<Stamp>,
}

impl MedicationItem {
    /// What identifies the drug across start and stop actions.
    pub fn key(&self) -> Option<&str> {
        self.drug.as_deref().or(self.drug_text.as_deref())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dose {
    pub value: Number,
    pub unit: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Mild,
    Moderate,
    Severe,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Criticality {
    Low,
    High,
    UnableToAssess,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AllergyType {
    Allergy,
    Intolerance,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AllergyItem {
    pub substance: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reaction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criticality: Option<Criticality>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<AllergyType>,
}

/// Vitals: one set at rooming, or several timed sets during an ED visit or a stay.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Vitals {
    #[default]
    None,
    One(VitalSet),
    Many(Vec<VitalSet>),
}

impl Vitals {
    pub fn is_empty(&self) -> bool {
        matches!(self, Vitals::None)
    }

    pub fn sets(&self) -> &[VitalSet] {
        match self {
            Vitals::None => &[],
            Vitals::One(set) => std::slice::from_ref(set),
            Vitals::Many(sets) => sets,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VitalSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Stamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bp: Option<[Number; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hr: Option<Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rr: Option<Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temp_c: Option<Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spo2: Option<Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height_cm: Option<Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peak_flow: Option<Number>,
}

/// One lab draw: a catalog panel with the members worth stating, or standalone tests.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Lab {
    pub panel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collected: Option<Stamp>,
    /// When the whole draw was released, if not the usual turnaround.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resulted: Option<Stamp>,
    pub results: Vec<LabResult>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LabResult {
    pub test: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// When this result was released, if it came back apart from the rest of the draw.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resulted: Option<Stamp>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Survey {
    pub survey: String,
    pub value: Number,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<Stamp>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Social {
    pub item: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub study: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed: Option<Stamp>,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    /// When the reader released it, if not the usual turnaround.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued: Option<Stamp>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcedureItem {
    pub procedure: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed: Option<Stamp>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Immunization {
    pub vaccine: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<Stamp>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DocStatus {
    Final,
    Preliminary,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Document {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<Stamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DocStatus>,
    pub body: String,
}

impl Document {
    pub fn kind(&self) -> &str {
        self.kind.as_deref().unwrap_or("Progress Notes")
    }

    pub fn title(&self) -> &str {
        self.title.as_deref().unwrap_or_else(|| self.kind())
    }
}

/// Codes verified for one unit's out-of-catalog conditions and drugs.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Terms {
    /// `label|icd10` to the confirmed ICD-10-CM code and its official display.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub conditions: BTreeMap<String, VerifiedCode>,
    /// `drug_text` to the RxNorm drug with an equivalent ingredient and strength.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub medications: BTreeMap<String, VerifiedCode>,
    /// What could not be verified and stays text-only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rejected: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VerifiedCode {
    pub code: String,
    pub display: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingredient: Option<String>,
    pub receipt: String,
}

/// A number as `f64`, for arithmetic on ledger values.
pub fn num(n: &Number) -> f64 {
    n.as_f64().unwrap_or(f64::NAN)
}
