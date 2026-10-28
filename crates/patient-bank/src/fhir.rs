//! The FHIR R4 resources the bank writes, as types.
//!
//! Only the fields the bank emits exist here. Field order is the order a reader
//! sees in each NDJSON line, starting with `resourceType`; absent optional fields
//! are left out.

use serde::Serialize;
use serde_json::Number;

pub const UCUM: &str = "http://unitsofmeasure.org";
pub const LOINC: &str = "http://loinc.org";

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Coding {
    pub system: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

impl Coding {
    pub fn new(system: &str, code: &str, display: &str) -> Coding {
        Coding {
            system: system.into(),
            code: code.into(),
            display: Some(display.into()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, PartialEq)]
pub struct CodeableConcept {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub coding: Vec<Coding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl CodeableConcept {
    pub fn text(text: &str) -> CodeableConcept {
        CodeableConcept {
            coding: Vec::new(),
            text: Some(text.into()),
        }
    }

    pub fn coded(coding: Coding, text: &str) -> CodeableConcept {
        CodeableConcept {
            coding: vec![coding],
            text: Some(text.into()),
        }
    }

    pub fn coding_only(coding: Coding) -> CodeableConcept {
        CodeableConcept {
            coding: vec![coding],
            text: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Reference {
    pub reference: String,
}

impl Reference {
    pub fn to(kind: &str, id: &str) -> Reference {
        Reference {
            reference: format!("{kind}/{id}"),
        }
    }
}

/// A reference by name only: who wrote a note, by role.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Named {
    pub display: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Period {
    pub start: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub version_id: &'static str,
    pub last_updated: String,
}

impl Meta {
    /// Version 1, last updated at midnight UTC of the resource's day.
    pub fn on(utc: &str) -> Meta {
        Meta {
            version_id: "1",
            last_updated: format!("{}T00:00:00+00:00", &utc[..10]),
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Quantity {
    pub value: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl Quantity {
    /// A UCUM quantity whose display unit may differ from its code.
    pub fn ucum(value: Number, unit: &str, code: &str) -> Quantity {
        Quantity {
            value,
            unit: Some(unit.into()),
            system: Some(UCUM),
            code: Some(code.into()),
        }
    }

    /// A quantity in a catalog unit (display and UCUM code alike), or a bare value.
    pub fn in_unit(value: Number, unit: Option<&str>) -> Quantity {
        match unit {
            Some(u) => Quantity::ucum(value, u, u),
            None => Quantity {
                value,
                unit: None,
                system: None,
                code: None,
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Annotation {
    pub text: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub content_type: &'static str,
    pub data: String,
}

// ---------------------------------------------------------------- Patient

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub identifier: Vec<Identifier>,
    pub name: Vec<HumanName>,
    pub gender: &'static str,
    pub birth_date: String,
    pub telecom: Vec<ContactPoint>,
    pub address: Vec<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<CodeableConcept>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<Communication>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<Extension>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Identifier {
    pub system: &'static str,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct HumanName {
    #[serde(rename = "use")]
    pub use_: &'static str,
    pub family: String,
    pub given: Vec<String>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ContactPoint {
    pub system: &'static str,
    pub value: String,
    #[serde(rename = "use")]
    pub use_: &'static str,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(rename = "use")]
    pub use_: &'static str,
    pub line: Vec<String>,
    pub city: String,
    pub state: &'static str,
    pub postal_code: String,
    pub country: &'static str,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Communication {
    pub language: CodeableConcept,
    pub preferred: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    pub url: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<Extension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_coding: Option<Coding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_string: Option<String>,
}

// ---------------------------------------------------------------- Condition

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub clinical_status: CodeableConcept,
    pub verification_status: CodeableConcept,
    pub category: Vec<CodeableConcept>,
    pub code: CodeableConcept,
    pub subject: Reference,
    pub onset_date_time: String,
    pub recorded_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abatement_date_time: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
}

// ---------------------------------------------------------------- MedicationRequest

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequest {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub status: &'static str,
    pub intent: &'static str,
    pub medication_codeable_concept: CodeableConcept,
    pub subject: Reference,
    pub authored_on: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_boolean: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<CodeableConcept>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dosage_instruction: Vec<Dosage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<CodeableConcept>,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dosage {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dose_and_rate: Vec<DoseAndRate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<CodeableConcept>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Timing {
    pub code: CodeableConcept,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoseAndRate {
    pub dose_quantity: Quantity,
}

// ---------------------------------------------------------------- AllergyIntolerance

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AllergyIntolerance {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub clinical_status: CodeableConcept,
    pub verification_status: CodeableConcept,
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub category: Vec<String>,
    pub criticality: &'static str,
    pub code: CodeableConcept,
    pub patient: Reference,
    pub recorded_date: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<Reaction>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Reaction {
    pub manifestation: Vec<CodeableConcept>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<&'static str>,
}

// ---------------------------------------------------------------- Observation

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub status: &'static str,
    pub category: Vec<CodeableConcept>,
    pub code: CodeableConcept,
    pub subject: Reference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,
    pub effective_date_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_codeable_concept: Option<CodeableConcept>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reference_range: Vec<ReferenceRange>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interpretation: Vec<CodeableConcept>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<Component>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_member: Vec<Reference>,
}

impl Observation {
    /// An observation with no value yet.
    pub fn new(
        id: String,
        status: &'static str,
        category: CodeableConcept,
        code: CodeableConcept,
        subject: &Reference,
        effective: String,
    ) -> Observation {
        Observation {
            resource_type: "Observation",
            id,
            meta: Meta::on(&effective),
            status,
            category: vec![category],
            code,
            subject: subject.clone(),
            encounter: None,
            effective_date_time: effective,
            issued: None,
            value_quantity: None,
            value_codeable_concept: None,
            reference_range: Vec::new(),
            interpretation: Vec::new(),
            component: Vec::new(),
            has_member: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ReferenceRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<Quantity>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub code: CodeableConcept,
    pub value_quantity: Quantity,
}

// ---------------------------------------------------------------- DiagnosticReport

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReport {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub status: &'static str,
    pub category: Vec<CodeableConcept>,
    pub code: CodeableConcept,
    pub subject: Reference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,
    pub effective_date_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<Reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub presented_form: Vec<Attachment>,
}

// ---------------------------------------------------------------- DocumentReference

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReference {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub status: &'static str,
    pub doc_status: &'static str,
    #[serde(rename = "type")]
    pub kind: CodeableConcept,
    pub category: Vec<CodeableConcept>,
    pub subject: Reference,
    pub date: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<Named>,
    pub description: String,
    pub content: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<DocContext>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Content {
    pub attachment: Attachment,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct DocContext {
    pub encounter: Vec<Reference>,
}

// ---------------------------------------------------------------- Procedure

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Procedure {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub status: &'static str,
    pub code: CodeableConcept,
    pub subject: Reference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,
    pub performed_date_time: String,
}

// ---------------------------------------------------------------- Encounter

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub status: &'static str,
    pub class: Coding,
    #[serde(rename = "type")]
    pub kind: Vec<CodeableConcept>,
    pub service_type: CodeableConcept,
    pub subject: Reference,
    pub period: Period,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<CodeableConcept>,
}

// ---------------------------------------------------------------- Immunization

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Immunization {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub status: &'static str,
    pub vaccine_code: CodeableConcept,
    pub patient: Reference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,
    pub occurrence_date_time: String,
    pub primary_source: bool,
}

// ---------------------------------------------------------------- ServiceRequest

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequest {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub status: &'static str,
    pub intent: &'static str,
    pub category: Vec<CodeableConcept>,
    pub code: CodeableConcept,
    pub subject: Reference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,
    pub authored_on: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub performer_type: Vec<CodeableConcept>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<CodeableConcept>,
}

// ---------------------------------------------------------------- Appointment

/// One visit on the practice calendar.
#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Appointment {
    pub resource_type: &'static str,
    pub id: String,
    pub meta: Meta,
    pub status: &'static str,
    pub service_type: Vec<CodeableConcept>,
    pub appointment_type: CodeableConcept,
    pub description: String,
    pub start: String,
    pub end: String,
    pub minutes_duration: i64,
    pub participant: Vec<Participant>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Participant {
    pub actor: Reference,
    pub status: &'static str,
}

// ---------------------------------------------------------------- one chart

/// The resource families: the Roger fixture's eight in its file order, then the
/// encounters, immunizations and orders it does not carry.
pub const FAMILIES: [&str; 11] = [
    "Patient",
    "Condition",
    "MedicationRequest",
    "Observation",
    "DiagnosticReport",
    "DocumentReference",
    "Procedure",
    "AllergyIntolerance",
    "Encounter",
    "Immunization",
    "ServiceRequest",
];

/// One patient's chart, by family.
#[derive(Clone, Debug, PartialEq)]
pub struct Chart {
    pub patient: Patient,
    pub conditions: Vec<Condition>,
    pub medications: Vec<MedicationRequest>,
    pub observations: Vec<Observation>,
    pub reports: Vec<DiagnosticReport>,
    pub documents: Vec<DocumentReference>,
    pub procedures: Vec<Procedure>,
    pub allergies: Vec<AllergyIntolerance>,
    pub encounters: Vec<Encounter>,
    pub immunizations: Vec<Immunization>,
    pub service_requests: Vec<ServiceRequest>,
}

impl Chart {
    /// Every family's resources as NDJSON lines, in `FAMILIES` order.
    pub fn lines(&self) -> [(&'static str, Vec<String>); 11] {
        fn each<T: Serialize>(rows: &[T]) -> Vec<String> {
            rows.iter()
                .map(|r| serde_json::to_string(r).expect("FHIR types serialize"))
                .collect()
        }
        [
            ("Patient", each(std::slice::from_ref(&self.patient))),
            ("Condition", each(&self.conditions)),
            ("MedicationRequest", each(&self.medications)),
            ("Observation", each(&self.observations)),
            ("DiagnosticReport", each(&self.reports)),
            ("DocumentReference", each(&self.documents)),
            ("Procedure", each(&self.procedures)),
            ("AllergyIntolerance", each(&self.allergies)),
            ("Encounter", each(&self.encounters)),
            ("Immunization", each(&self.immunizations)),
            ("ServiceRequest", each(&self.service_requests)),
        ]
    }

    pub fn counts(&self) -> [(&'static str, usize); 11] {
        [
            ("Patient", 1),
            ("Condition", self.conditions.len()),
            ("MedicationRequest", self.medications.len()),
            ("Observation", self.observations.len()),
            ("DiagnosticReport", self.reports.len()),
            ("DocumentReference", self.documents.len()),
            ("Procedure", self.procedures.len()),
            ("AllergyIntolerance", self.allergies.len()),
            ("Encounter", self.encounters.len()),
            ("Immunization", self.immunizations.len()),
            ("ServiceRequest", self.service_requests.len()),
        ]
    }
}
