//! Shared domain types for filed source records, adapter mappings, and pull outcomes.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::{SourcePatient, SourceRevision};

/// An unknown chart bucket name supplied at an external boundary.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("unknown chart bucket `{0}`")]
pub struct UnknownBucketError(pub String);

/// The sixteen chart buckets defined by the physician product specification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Bucket {
    /// Patient demographics and identifiers.
    Patient,
    /// Active and historical problems or diagnoses.
    Problems,
    /// Prescriptions, medication statements, dispenses, and administrations.
    Medications,
    /// Allergies and intolerances.
    Allergies,
    /// Laboratory, imaging, and diagnostic report results.
    Results,
    /// Vital signs.
    Vitals,
    /// Clinical notes.
    Notes,
    /// Non-note clinical documents and attachments.
    Documents,
    /// Encounters and appointments.
    Visits,
    /// Procedures.
    Procedures,
    /// Immunizations.
    Immunizations,
    /// Service requests and orders.
    Orders,
    /// Social and family history.
    History,
    /// Questionnaires and survey observations.
    Forms,
    /// Patient and care-team communications.
    Messages,
    /// Source records whose kind has no more specific bucket row.
    Other,
}

impl Bucket {
    /// Every chart bucket in canonical order.
    pub const ALL: [Self; 16] = [
        Self::Patient,
        Self::Problems,
        Self::Medications,
        Self::Allergies,
        Self::Results,
        Self::Vitals,
        Self::Notes,
        Self::Documents,
        Self::Visits,
        Self::Procedures,
        Self::Immunizations,
        Self::Orders,
        Self::History,
        Self::Forms,
        Self::Messages,
        Self::Other,
    ];

    /// Canonical snake-case wire name for this bucket.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Patient => "patient",
            Self::Problems => "problems",
            Self::Medications => "medications",
            Self::Allergies => "allergies",
            Self::Results => "results",
            Self::Vitals => "vitals",
            Self::Notes => "notes",
            Self::Documents => "documents",
            Self::Visits => "visits",
            Self::Procedures => "procedures",
            Self::Immunizations => "immunizations",
            Self::Orders => "orders",
            Self::History => "history",
            Self::Forms => "forms",
            Self::Messages => "messages",
            Self::Other => "other",
        }
    }

    /// Parses a caller-supplied bucket name without silent fallback.
    pub fn parse(raw: &str) -> Result<Self, UnknownBucketError> {
        match raw {
            "patient" => Ok(Self::Patient),
            "problems" => Ok(Self::Problems),
            "medications" => Ok(Self::Medications),
            "allergies" => Ok(Self::Allergies),
            "results" => Ok(Self::Results),
            "vitals" => Ok(Self::Vitals),
            "notes" => Ok(Self::Notes),
            "documents" => Ok(Self::Documents),
            "visits" => Ok(Self::Visits),
            "procedures" => Ok(Self::Procedures),
            "immunizations" => Ok(Self::Immunizations),
            "orders" => Ok(Self::Orders),
            "history" => Ok(Self::History),
            "forms" => Ok(Self::Forms),
            "messages" => Ok(Self::Messages),
            "other" => Ok(Self::Other),
            other => Err(UnknownBucketError(other.to_owned())),
        }
    }
}

impl fmt::Display for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// How a source names a record's kind, including its category when the source labels one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRecordKind {
    /// Source-native record type name (such as `"Observation"` or `"DocumentReference"`).
    pub kind: String,
    /// Source-native category label when present (such as `"laboratory"` or `"clinical-note"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl SourceRecordKind {
    /// Canonical source kind string (`"Observation/laboratory"` when categorized, `"Condition"` otherwise).
    pub fn display_name(&self) -> String {
        match &self.category {
            Some(category) => format!("{}/{category}", self.kind),
            None => self.kind.clone(),
        }
    }
}

/// Semantic role a date field plays in its source record, declared by the adapter table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DateRole {
    /// Patient date of birth.
    BirthDate,
    /// Condition or allergy onset.
    Onset,
    /// Condition abatement or resolution.
    Abatement,
    /// When the record was recorded or documented.
    Recorded,
    /// When an order, request, or response was authored.
    Authored,
    /// Clinical effective time of an observation, medication, or report.
    Effective,
    /// When a report or observation was issued.
    Issued,
    /// When a procedure was performed.
    Performed,
    /// When an immunization or dispense occurred.
    Occurred,
    /// Start of a clinical period or appointment.
    Start,
    /// End of a clinical period or appointment.
    End,
    /// When a communication was sent.
    Sent,
    /// When a communication was received.
    Received,
}

impl DateRole {
    /// Canonical snake-case name for this date role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BirthDate => "birth_date",
            Self::Onset => "onset",
            Self::Abatement => "abatement",
            Self::Recorded => "recorded",
            Self::Authored => "authored",
            Self::Effective => "effective",
            Self::Issued => "issued",
            Self::Performed => "performed",
            Self::Occurred => "occurred",
            Self::Start => "start",
            Self::End => "end",
            Self::Sent => "sent",
            Self::Received => "received",
        }
    }
}

/// One date extracted from a source record without altering the value sent by the source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FiledDate {
    /// Declared role of this date field.
    pub role: DateRole,
    /// Exact field path in the source record (such as `"effectiveDateTime"` or `"period.start"`).
    pub field: String,
    /// Exact value string sent by the source.
    pub value: String,
}

/// One coded concept extracted from a source record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordCode {
    /// Source field path (such as `"code"`, `"medicationCodeableConcept"`, or `"vaccineCode"`).
    pub field: String,
    /// Coding system URI or identifier when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Code token when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Coding display string when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// Concept text when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// How a filed record's version identity was established.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "origin", content = "value", rename_all = "snake_case")]
pub enum RecordVersion {
    /// Explicit version supplied by the source (such as FHIR `meta.versionId`).
    Source(String),
    /// Content SHA-256 hex used when the source supplies no version.
    ContentHash(String),
}

impl RecordVersion {
    /// The version token string regardless of origin.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Source(value) | Self::ContentHash(value) => value.as_str(),
        }
    }
}

/// Shared envelope carried by every record filed through a declared adapter table.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordEnvelope {
    /// Target chart bucket declared by the adapter's mapping row.
    pub bucket: Bucket,
    /// Source's own name for this record's kind (including category when labeled).
    pub source_kind: SourceRecordKind,
    /// Source's native record identifier.
    pub native_id: String,
    /// Source-supplied version or content-hash version.
    pub version: RecordVersion,
    /// Dates present on the record, with their declared roles and source field names.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dates: Vec<FiledDate>,
    /// Status in the source's exact words; absent when the source sent none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Coded concepts present on the record.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub codes: Vec<RecordCode>,
    /// Display title present on or derived from the record's declared title fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Direct link into the source system when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_link: Option<String>,
}

/// Receipt returned when recording a source chart delivery.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChartPullReceipt {
    /// Number of candidate observations added or updated in this delivery.
    pub changed: usize,
    /// Previously active candidate IDs from this source that were absent from this pull.
    pub missing_candidates: Vec<String>,
}

/// One part of a source pull (such as a resource family) that failed during a partial read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FailedPart {
    /// Source part or resource family name.
    pub part: String,
    /// Diagnostic reason reported by the adapter.
    pub reason: String,
}

/// Outcome of a single source pull attempt for one patient.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PullOutcome {
    /// Every part of the pull succeeded.
    Succeeded {
        /// Previously saved candidate IDs from this source that were absent on this pull.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        missing_candidates: Vec<String>,
    },
    /// Some parts loaded and were saved, while other parts failed.
    Partial {
        /// Parts of the pull that failed.
        failed_parts: Vec<FailedPart>,
        /// Previously saved candidate IDs absent among the parts that succeeded.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        missing_candidates: Vec<String>,
    },
    /// The pull failed before a usable chart could be read.
    Failed {
        /// Failure cause reported by the source adapter.
        reason: String,
    },
    /// Source credentials or authorization have lapsed and require reconnecting.
    LapsedAccess {
        /// Access refusal reported by the source adapter.
        reason: String,
    },
}

/// Connector-level output of pulling one patient's chart from a [`super::PatientSource`].
#[derive(Clone, Debug, PartialEq)]
pub enum SourcePullOutput {
    /// Complete patient identity and chart records.
    Complete {
        /// Patient identity read from the source.
        patient: SourcePatient,
        /// Admitted source revisions for the workspace.
        records: Vec<SourceRevision>,
    },
    /// Partial pull: records that succeeded alongside the parts that failed.
    Partial {
        /// Patient identity if the identity part succeeded.
        patient: Option<SourcePatient>,
        /// Admitted source revisions from the parts that succeeded.
        records: Vec<SourceRevision>,
        /// Parts that failed during the pull.
        failed_parts: Vec<FailedPart>,
    },
    /// Whole-patient pull failure.
    Failed {
        /// Failure reason.
        reason: String,
    },
    /// Source access has lapsed and needs reconnect.
    LapsedAccess {
        /// Access failure reason.
        reason: String,
    },
}

/// Confirmation returned when a practice reconnects a source adapter.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReconnectReceipt {
    /// Stable source identifier.
    pub source_id: String,
    /// Practice whose source access was reconnected.
    pub practice_id: String,
    /// Practice-clock Unix timestamp in seconds when reconnected.
    pub reconnected_at: i64,
}
