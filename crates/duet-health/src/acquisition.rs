//! Durable acquisition records and the native source adapter contract.
//!
//! These types are distinct from the display-only [`crate::PhoneReading`]:
//! they carry the native identity, provenance, exact literal value and unit,
//! the declared instant or interval with its original time zone, and explicit
//! deletions. A failed read, a lost permission or an empty result is reported
//! as a per-type [`CoverageOutcome`] and never becomes a deletion.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Phone sources that may write canonical evidence from a device.
///
/// The names are the server's device source kinds. Android Health Connect has
/// no server source kind yet: [`PhoneSource::HealthConnect`] exists so the
/// gated adapter can name itself, but it stays out of the uploader until the
/// server accepts it and its type scope is agreed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PhoneSource {
    /// HealthKit samples (`appleHealth`).
    #[serde(rename = "appleHealth")]
    AppleHealth,
    /// HealthKit clinical records on the phone (`healthRecords`).
    #[serde(rename = "healthRecords")]
    HealthRecords,
    /// Android Health Connect (`healthConnect`). Not accepted by the server
    /// yet; acquisition is gated off by default.
    #[serde(rename = "healthConnect")]
    HealthConnect,
}

impl PhoneSource {
    /// The server source kind.
    pub fn kind(self) -> &'static str {
        match self {
            Self::AppleHealth => "appleHealth",
            Self::HealthRecords => "healthRecords",
            Self::HealthConnect => "healthConnect",
        }
    }
}

/// Where a record came from, exactly as the native store reported it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    /// Human-readable writer name reported by the store, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// Bundle identifier (iOS) or package name (Android) of the original writer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub writer_id: Option<String>,
    /// Device description reported with the sample (model, manufacturer, ...).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

/// An exact measurement. The literal is never reparsed or rounded.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactValue {
    /// Literal value as the store produced it, e.g. `"072"` or `"< 5.7"`.
    pub literal: String,
    /// Literal unit, e.g. `"count/min"`. Absent when the store reported none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// When the source says the record happened. Timestamps are RFC 3339 strings
/// preserved byte-for-byte; absent values stay absent (unknown), never "now".
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SourceTime {
    /// The source declared no date.
    Unknown,
    /// One instant.
    Instant {
        /// RFC 3339 instant with its original offset.
        at: String,
    },
    /// A start/end interval; either side may be unknown.
    Interval {
        /// RFC 3339 start, if declared.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        start: Option<String>,
        /// RFC 3339 end, if declared.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        end: Option<String>,
    },
}

/// One acquired record ready for durable upload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcquisitionRecord {
    /// Stable native identity (e.g. `HKQuantitySample/<uuid>`). Unique per source.
    pub native_id: String,
    /// Data type this record belongs to, matching a [`TypeCoverage::data_type`].
    pub data_type: String,
    /// Canonical concept identity, e.g. `vital:heart_rate`.
    pub concept_id: String,
    /// Exact value and unit.
    pub value: ExactValue,
    /// Declared instant or interval.
    pub time: SourceTime,
    /// IANA time zone recorded with the sample (e.g. `HKTimeZone`), if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Writer and device provenance.
    #[serde(default)]
    pub provenance: Provenance,
    /// Original source object (metadata, FHIR resource, ...) retained verbatim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<Value>,
}

impl AcquisitionRecord {
    /// The exact canonical payload sent for this record. Dates are included
    /// only when the source declared them.
    pub fn payload(&self, source: PhoneSource) -> Value {
        let mut provenance = Map::new();
        provenance.insert("kind".into(), source.kind().into());
        provenance.insert("record_id".into(), self.native_id.clone().into());
        provenance.insert("data_type".into(), self.data_type.clone().into());
        if let Some(name) = &self.provenance.source_name {
            provenance.insert("source_name".into(), name.clone().into());
        }
        if let Some(writer) = &self.provenance.writer_id {
            provenance.insert("writer_id".into(), writer.clone().into());
        }
        if let Some(device) = &self.provenance.device {
            provenance.insert("device".into(), device.clone().into());
        }
        let mut value = Map::new();
        value.insert("literal".into(), self.value.literal.clone().into());
        if let Some(unit) = &self.value.unit {
            value.insert("unit".into(), unit.clone().into());
        }
        let mut payload = Map::new();
        payload.insert("concept_id".into(), self.concept_id.clone().into());
        payload.insert("source".into(), Value::Object(provenance));
        payload.insert("value".into(), Value::Object(value));
        match &self.time {
            SourceTime::Unknown => {}
            SourceTime::Instant { at } => {
                payload.insert("measured_at".into(), at.clone().into());
            }
            SourceTime::Interval { start, end } => {
                let mut interval = Map::new();
                if let Some(start) = start {
                    interval.insert("start".into(), start.clone().into());
                }
                if let Some(end) = end {
                    interval.insert("end".into(), end.clone().into());
                }
                payload.insert("measured_interval".into(), Value::Object(interval));
            }
        }
        if let Some(zone) = &self.time_zone {
            payload.insert("timezone".into(), zone.clone().into());
        }
        if let Some(original) = &self.original {
            payload.insert("source_record".into(), original.clone());
        }
        Value::Object(payload)
    }
}

/// An explicit deletion reported by the native store (e.g. `HKDeletedObject`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletionRecord {
    /// Native identity of the deleted record.
    pub native_id: String,
    /// Data type the deletion was observed for.
    pub data_type: String,
}

/// Why a data type could not be read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum CoverageError {
    /// The store is temporarily unavailable (locked device, busy, ...).
    StoreUnavailable,
    /// The query failed; the message is diagnostic only.
    QueryFailed {
        /// Native error text.
        message: String,
    },
    /// The data type is not supported on this device or OS version.
    Unsupported,
    /// Acquisition for this source is deliberately disabled in this build
    /// (e.g. Health Connect pending its agreed type scope). Never "empty".
    NotEnabled,
}

/// Per-type result of one read. Only [`CoverageOutcome::Data`] and
/// [`CoverageOutcome::Empty`] are successful reads; neither the empty nor the
/// failure outcomes ever imply that a record was deleted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum CoverageOutcome {
    /// Read succeeded and returned no changes.
    Empty,
    /// Read succeeded and returned changes.
    Data,
    /// Read failed with a typed error.
    Error {
        /// Typed failure.
        error: CoverageError,
    },
    /// Read permission is no longer granted.
    PermissionLost,
}

impl CoverageOutcome {
    /// Whether a successful read happened, so explicit deletions are credible.
    pub fn read_succeeded(&self) -> bool {
        matches!(self, Self::Empty | Self::Data)
    }
}

/// Coverage for one data type in a page.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeCoverage {
    /// Native data type identifier (e.g. `HKQuantityTypeIdentifierHeartRate`).
    pub data_type: String,
    /// Read outcome.
    pub outcome: CoverageOutcome,
}

/// Opaque change token. Adapters encode per-type anchors inside it; the
/// outbox stores it verbatim and never interprets it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Anchor(pub String);

/// One bounded read request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageRequest {
    /// Anchor committed with the previous page; `None` reads from the start.
    pub anchor: Option<Anchor>,
    /// Maximum `records.len() + deletions.len()` the adapter may return.
    pub limit: usize,
}

/// One bounded page of changes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AcquisitionPage {
    /// New or changed records.
    pub records: Vec<AcquisitionRecord>,
    /// Explicit deletions observed by a successful read.
    pub deletions: Vec<DeletionRecord>,
    /// Outcome for every data type covered by this read.
    pub coverage: Vec<TypeCoverage>,
    /// Anchor to commit atomically with this page's records. An adapter must
    /// not advance the part of the anchor for a type that failed.
    pub next_anchor: Option<Anchor>,
    /// Whether another page is available immediately.
    pub has_more: bool,
}

/// Whole-read failures. None of these imply deletions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdapterError {
    /// All read permission is gone.
    PermissionLost,
    /// The store cannot be read right now (locked device, not installed, ...).
    StoreUnavailable,
    /// The store no longer accepts the anchor; reread from the start.
    AnchorInvalidated,
    /// Any other failure; diagnostic only.
    Failed(String),
}

/// Native bridge contract implemented by the HealthKit and Health Connect
/// adapters. Implementations hold no durable state: the outbox owns anchors,
/// records, retry and upload. Reads must be side-effect free so a page can be
/// re-read after a crash before its anchor was committed.
pub trait HealthSourceAdapter {
    /// The source this adapter reads.
    fn source(&self) -> PhoneSource;
    /// Read at most `request.limit` changes after `request.anchor`.
    fn read_page(&self, request: &PageRequest) -> Result<AcquisitionPage, AdapterError>;
}
