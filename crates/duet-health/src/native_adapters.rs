//! Native acquisition adapters over the HealthKit and Health Connect bridges.
//!
//! The native bridges are thin: they answer one bounded query and return
//! what the store reported (records, deleted identifiers, a new opaque
//! sub-anchor or changes token, or a typed error). Every durable decision —
//! which anchor is committed, how failures map to coverage, an explicit
//! history window, the Health Connect scope gate — lives here and is tested
//! on Linux against fake bridges. Paging is the default history bound.
//!
//! * [`HealthKitAdapter`] keeps one `HKQueryAnchor` per data type inside a
//!   composite [`Anchor`]. A type whose read failed keeps its previous
//!   sub-anchor, so its changes are reread on the next pass.
//! * [`HealthConnectAdapter`] drives the type-independent Health Connect
//!   changes feed (token, pagination, expiry, errors). It is gated by
//!   [`HealthConnectScope`], which defaults to disabled until the Android type
//!   scope is agreed; a disabled adapter reports a typed
//!   [`CoverageError::NotEnabled`], never an empty read.

use crate::acquisition::{
    AcquisitionPage, AcquisitionRecord, AdapterError, Anchor, CoverageError, CoverageOutcome,
    DeletionRecord, ExactValue, HealthSourceAdapter, PageRequest, PhoneSource, Provenance,
    SourceTime, TypeCoverage,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

/// Typed failure reported by a native bridge for one query.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum BridgeError {
    /// Read authorization is not granted (where the platform reports it).
    PermissionDenied,
    /// The store cannot be read now (device locked, provider busy, ...).
    StoreUnavailable,
    /// The anchor or changes token is no longer accepted.
    AnchorInvalidated,
    /// The type is not supported on this OS or device.
    Unsupported,
    /// Any other failure; diagnostic only.
    Failed {
        /// Native error text.
        #[serde(default)]
        message: String,
    },
}

/// One sample exactly as a native bridge reported it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NativeSample {
    /// Native identifier (HealthKit object UUID, Health Connect record id).
    pub id: String,
    /// Literal value produced by the bridge; never reparsed.
    pub value: String,
    /// Literal unit string (e.g. `count/min`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// RFC 3339 start, with its original offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// RFC 3339 end, with its original offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// IANA time zone recorded with the sample (`HKMetadataKeyTimeZone`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Writer display name (`sourceRevision.source.name`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// Writer bundle id (iOS) or `dataOrigin.packageName` (Android).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub writer_id: Option<String>,
    /// Device description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Native metadata retained verbatim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl NativeSample {
    fn source_time(&self) -> SourceTime {
        match (&self.start, &self.end) {
            (None, None) => SourceTime::Unknown,
            (Some(start), Some(end)) if start == end => SourceTime::Instant { at: start.clone() },
            (start, end) => SourceTime::Interval {
                start: start.clone(),
                end: end.clone(),
            },
        }
    }

    fn into_record(
        self,
        native_id: String,
        spec: &TypeSpec,
        provenance: Provenance,
    ) -> AcquisitionRecord {
        let time = self.source_time();
        AcquisitionRecord {
            native_id,
            data_type: spec.data_type.clone(),
            concept_id: spec.concept_id.clone(),
            value: ExactValue {
                literal: self.value,
                unit: self.unit,
            },
            time,
            time_zone: self.time_zone,
            provenance,
            original: self.metadata,
        }
    }
}

/// `HKUnit.percent()` is 0–1. Shift a plain decimal literal two places so the
/// canonical value is a conventional percent. `0.98` becomes `98`.
fn scale_healthkit_percent(literal: &str) -> Option<String> {
    let (negative, body) = if let Some(rest) = literal.strip_prefix('-') {
        (true, rest)
    } else if let Some(rest) = literal.strip_prefix('+') {
        (false, rest)
    } else {
        (false, literal)
    };
    if body.is_empty() || body.chars().filter(|c| *c == '.').count() > 1 {
        return None;
    }
    let (whole, frac) = body.split_once('.').unwrap_or((body, ""));
    if whole.is_empty() && frac.is_empty() {
        return None;
    }
    if !whole.chars().all(|c| c.is_ascii_digit()) || !frac.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let whole = if whole.is_empty() { "0" } else { whole };
    let mut digits = String::with_capacity(whole.len() + frac.len().max(2));
    digits.push_str(whole);
    digits.push_str(frac);
    let pad = 2usize.saturating_sub(frac.len());
    digits.extend(std::iter::repeat_n('0', pad));
    let (head, tail) = digits.split_at(whole.len() + 2);
    let head = head.trim_start_matches('0');
    let head = if head.is_empty() { "0" } else { head };
    let mut scaled = String::with_capacity(head.len() + tail.len() + 2);
    if negative && !(head == "0" && tail.chars().all(|c| c == '0')) {
        scaled.push('-');
    }
    scaled.push_str(head);
    if !tail.is_empty() {
        scaled.push('.');
        scaled.push_str(tail);
    }
    Some(scaled)
}

/// Keep the bridge's 0–1 reading beside any metadata the sample already retained.
fn retain_unscaled_reading(original: Option<Value>, literal: &str, unit: &str) -> Value {
    let reading = json!({"literal": literal, "unit": unit});
    match original {
        Some(Value::Object(mut map)) => {
            map.insert("reading".into(), reading);
            Value::Object(map)
        }
        Some(other) => json!({"reading": reading, "metadata": other}),
        None => json!({"reading": reading}),
    }
}

/// A data type the adapter reads and the canonical concept it maps to.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeSpec {
    /// Native type identifier (e.g. `HKQuantityTypeIdentifierHeartRate`).
    pub data_type: String,
    /// Canonical concept id (e.g. `vital:heart_rate`).
    pub concept_id: String,
    /// Unit the bridge must express the exact value in (HealthKit unit string).
    pub unit: String,
}

impl TypeSpec {
    /// Builds a spec.
    pub fn new(data_type: &str, concept_id: &str, unit: &str) -> Self {
        Self {
            data_type: data_type.into(),
            concept_id: concept_id.into(),
            unit: unit.into(),
        }
    }
}

/// HealthKit quantity types acquired by default.
pub fn default_healthkit_types() -> Vec<TypeSpec> {
    [
        (
            "HKQuantityTypeIdentifierHeartRate",
            "vital:heart_rate",
            "count/min",
        ),
        (
            "HKQuantityTypeIdentifierRestingHeartRate",
            "vital:resting_heart_rate",
            "count/min",
        ),
        (
            "HKQuantityTypeIdentifierBloodPressureSystolic",
            "vital:blood_pressure_systolic",
            "mmHg",
        ),
        (
            "HKQuantityTypeIdentifierBloodPressureDiastolic",
            "vital:blood_pressure_diastolic",
            "mmHg",
        ),
        (
            "HKQuantityTypeIdentifierOxygenSaturation",
            "vital:oxygen_saturation",
            "%",
        ),
        (
            "HKQuantityTypeIdentifierBodyTemperature",
            "vital:body_temperature",
            "degC",
        ),
        ("HKQuantityTypeIdentifierBodyMass", "body:weight", "kg"),
        ("HKQuantityTypeIdentifierHeight", "body:height", "m"),
        (
            "HKQuantityTypeIdentifierBloodGlucose",
            "lab:glucose",
            "mg/dL",
        ),
        (
            "HKQuantityTypeIdentifierStepCount",
            "activity:steps",
            "count",
        ),
    ]
    .into_iter()
    .map(|(data_type, concept, unit)| TypeSpec::new(data_type, concept, unit))
    .collect()
}

// ---------------------------------------------------------------------------
// HealthKit
// ---------------------------------------------------------------------------

/// One anchored query for one HealthKit type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthKitQuery {
    /// HealthKit type identifier.
    pub data_type: String,
    /// Unit to express exact values in.
    pub unit: String,
    /// Base64 `NSKeyedArchiver` archive of the previous `HKQueryAnchor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// Lower bound (Unix seconds) of an explicit history predicate.
    /// Absent means the anchored query has no start predicate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_start_unix: Option<i64>,
    /// Maximum samples + deleted objects to return.
    pub limit: usize,
}

/// What one anchored query returned.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HealthKitChanges {
    /// New or changed samples.
    #[serde(default)]
    pub samples: Vec<NativeSample>,
    /// UUIDs of `HKDeletedObject`s.
    #[serde(default)]
    pub deleted: Vec<String>,
    /// Base64 archive of the new `HKQueryAnchor`.
    pub anchor: String,
    /// The query hit its limit; more changes are waiting.
    #[serde(default)]
    pub has_more: bool,
}

/// Low-level HealthKit bridge: one anchored object query per call.
pub trait HealthKitBridge {
    /// `HKHealthStore.isHealthDataAvailable()`.
    fn available(&self) -> bool;
    /// Run one `HKAnchoredObjectQuery`.
    fn read_type(&self, query: &HealthKitQuery) -> Result<HealthKitChanges, BridgeError>;
}

/// Durable shape of the composite HealthKit anchor.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct CompositeAnchor {
    v: u32,
    /// Fixed lower bound of an explicit initial window, set on the first read
    /// so later pages keep the same predicate. Absent when paging is the only bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    history_start_unix: Option<i64>,
    /// Data type → base64 `HKQueryAnchor`.
    #[serde(default)]
    types: BTreeMap<String, String>,
}

const COMPOSITE_VERSION: u32 = 1;

impl CompositeAnchor {
    fn decode(anchor: &Anchor) -> Result<Self, AdapterError> {
        let parsed: Self =
            serde_json::from_str(&anchor.0).map_err(|_| AdapterError::AnchorInvalidated)?;
        if parsed.v != COMPOSITE_VERSION {
            return Err(AdapterError::AnchorInvalidated);
        }
        Ok(parsed)
    }

    fn encode(&self) -> Anchor {
        Anchor(serde_json::to_string(self).unwrap_or_default())
    }
}

/// Identity used for both HealthKit samples and their `HKDeletedObject`s.
pub fn healthkit_native_id(uuid: &str) -> String {
    format!("HKObject/{uuid}")
}

/// [`HealthSourceAdapter`] over a [`HealthKitBridge`].
pub struct HealthKitAdapter<B> {
    bridge: B,
    types: Vec<TypeSpec>,
    initial_window_days: Option<u32>,
    now_unix: fn() -> i64,
}

fn system_now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() as i64)
        .unwrap_or(0)
}

impl<B: HealthKitBridge> HealthKitAdapter<B> {
    /// Adapter over `bridge` for the default types. History has no start
    /// predicate; each read is bounded by the page limit.
    pub fn new(bridge: B) -> Self {
        Self {
            bridge,
            types: default_healthkit_types(),
            initial_window_days: None,
            now_unix: system_now_unix,
        }
    }

    /// Replace the acquired types.
    pub fn with_types(mut self, types: Vec<TypeSpec>) -> Self {
        self.types = types;
        self
    }

    /// Bound the first read to the last `days` days and keep that predicate
    /// for later pages. The default does not set a window.
    pub fn with_initial_window_days(mut self, days: u32) -> Self {
        self.initial_window_days = Some(days);
        self
    }

    /// Replace the clock (tests).
    pub fn with_clock(mut self, now_unix: fn() -> i64) -> Self {
        self.now_unix = now_unix;
        self
    }

    /// Types this adapter reads.
    pub fn types(&self) -> &[TypeSpec] {
        &self.types
    }
}

fn coverage_error(error: &BridgeError) -> CoverageOutcome {
    match error {
        BridgeError::PermissionDenied => CoverageOutcome::PermissionLost,
        BridgeError::StoreUnavailable => CoverageOutcome::Error {
            error: CoverageError::StoreUnavailable,
        },
        BridgeError::Unsupported => CoverageOutcome::Error {
            error: CoverageError::Unsupported,
        },
        BridgeError::AnchorInvalidated => CoverageOutcome::Error {
            error: CoverageError::QueryFailed {
                message: "anchor_invalidated".into(),
            },
        },
        BridgeError::Failed { message } => CoverageOutcome::Error {
            error: CoverageError::QueryFailed {
                message: message.clone(),
            },
        },
    }
}

impl<B: HealthKitBridge> HealthSourceAdapter for HealthKitAdapter<B> {
    fn source(&self) -> PhoneSource {
        PhoneSource::AppleHealth
    }

    fn read_page(&self, request: &PageRequest) -> Result<AcquisitionPage, AdapterError> {
        if !self.bridge.available() {
            return Err(AdapterError::StoreUnavailable);
        }
        let mut composite = match &request.anchor {
            Some(anchor) => CompositeAnchor::decode(anchor)?,
            None => CompositeAnchor {
                v: COMPOSITE_VERSION,
                history_start_unix: self
                    .initial_window_days
                    .map(|days| (self.now_unix)() - i64::from(days) * 86_400),
                types: BTreeMap::new(),
            },
        };
        let mut page = AcquisitionPage::default();
        let mut remaining = request.limit.max(1);
        let mut errors: Vec<BridgeError> = Vec::new();
        for spec in &self.types {
            if remaining == 0 {
                page.has_more = true;
                break;
            }
            let query = HealthKitQuery {
                data_type: spec.data_type.clone(),
                unit: spec.unit.clone(),
                anchor: composite.types.get(&spec.data_type).cloned(),
                history_start_unix: composite.history_start_unix,
                limit: remaining,
            };
            match self.bridge.read_type(&query) {
                Ok(changes) => {
                    let count = changes.samples.len() + changes.deleted.len();
                    if count > remaining {
                        // A bridge that ignores the limit cannot be committed
                        // partially; keep the old sub-anchor and report it.
                        let error = BridgeError::Failed {
                            message: "bridge exceeded the page limit".into(),
                        };
                        page.coverage.push(TypeCoverage {
                            data_type: spec.data_type.clone(),
                            outcome: coverage_error(&error),
                        });
                        errors.push(error);
                        continue;
                    }
                    remaining -= count;
                    page.coverage.push(TypeCoverage {
                        data_type: spec.data_type.clone(),
                        outcome: if count == 0 {
                            CoverageOutcome::Empty
                        } else {
                            CoverageOutcome::Data
                        },
                    });
                    for sample in changes.samples {
                        let provenance = Provenance {
                            source_name: sample.source_name.clone(),
                            writer_id: sample.writer_id.clone(),
                            device: sample.device.clone(),
                        };
                        let native_id = healthkit_native_id(&sample.id);
                        let mut record = sample.into_record(native_id, spec, provenance);
                        if record.value.unit.as_deref() == Some("%")
                            && let Some(scaled) = scale_healthkit_percent(&record.value.literal)
                        {
                            let unscaled = std::mem::replace(&mut record.value.literal, scaled);
                            record.original = Some(retain_unscaled_reading(
                                record.original.take(),
                                &unscaled,
                                "%",
                            ));
                        }
                        page.records.push(record);
                    }
                    for uuid in changes.deleted {
                        page.deletions.push(DeletionRecord {
                            native_id: healthkit_native_id(&uuid),
                            data_type: spec.data_type.clone(),
                        });
                    }
                    composite
                        .types
                        .insert(spec.data_type.clone(), changes.anchor);
                    if changes.has_more {
                        page.has_more = true;
                        // Drain this type before moving on so its anchor and
                        // the page budget stay consistent.
                        break;
                    }
                }
                Err(error) => {
                    if error == BridgeError::AnchorInvalidated {
                        // Only this type rereads; its predicate is unchanged.
                        composite.types.remove(&spec.data_type);
                    }
                    page.coverage.push(TypeCoverage {
                        data_type: spec.data_type.clone(),
                        outcome: coverage_error(&error),
                    });
                    errors.push(error);
                }
            }
        }
        if !self.types.is_empty() && errors.len() == self.types.len() {
            // Nothing was read: report the whole-read failure, commit nothing.
            if errors.iter().all(|e| *e == BridgeError::StoreUnavailable) {
                return Err(AdapterError::StoreUnavailable);
            }
            if errors.iter().all(|e| *e == BridgeError::PermissionDenied) {
                return Err(AdapterError::PermissionLost);
            }
        }
        page.next_anchor = Some(composite.encode());
        Ok(page)
    }
}

/// Decodes one HealthKit bridge reply: `{"ok": HealthKitChanges}` or
/// `{"error": BridgeError}`. A malformed body is a typed failure.
pub fn decode_healthkit_reply(body: &str) -> Result<HealthKitChanges, BridgeError> {
    decode_reply(body)
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Reply<T> {
    Ok(T),
    Error(BridgeError),
}

fn decode_reply<T: serde::de::DeserializeOwned>(body: &str) -> Result<T, BridgeError> {
    match serde_json::from_str::<Reply<T>>(body) {
        Ok(Reply::Ok(value)) => Ok(value),
        Ok(Reply::Error(error)) => Err(error),
        Err(error) => Err(BridgeError::Failed {
            message: format!("malformed bridge reply: {error}"),
        }),
    }
}

// ---------------------------------------------------------------------------
// Health Connect
// ---------------------------------------------------------------------------

/// Which Health Connect types may be acquired. Disabled until the Android
/// type scope (and its permissions and background work) is agreed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum HealthConnectScope {
    /// No acquisition; every read reports [`CoverageError::NotEnabled`].
    #[default]
    Disabled,
    /// Acquire these record types (Health Connect record type names).
    Enabled {
        /// Record types in scope.
        types: Vec<TypeSpec>,
    },
}

/// Coverage key for the type-independent Health Connect changes feed.
/// Deletions carry it because `DeletionChange` has no record type.
pub const HEALTH_CONNECT_FEED: &str = "healthConnect:changes";

/// One Health Connect record from the changes feed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthConnectRecord {
    /// Record type name (e.g. `HeartRateRecord`).
    pub record_type: String,
    /// Sample content; `writer_id` carries `metadata.dataOrigin.packageName`.
    #[serde(flatten)]
    pub sample: NativeSample,
}

/// One page from `getChanges(token)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HealthConnectChanges {
    /// `UpsertionChange` records.
    #[serde(default)]
    pub upserts: Vec<HealthConnectRecord>,
    /// `DeletionChange.recordId`s.
    #[serde(default)]
    pub deleted: Vec<String>,
    /// `nextChangesToken`.
    pub next_token: String,
    /// `hasMore`.
    #[serde(default)]
    pub has_more: bool,
    /// `changesTokenExpired`.
    #[serde(default)]
    pub token_expired: bool,
}

/// Low-level, type-independent Health Connect bridge.
pub trait HealthConnectBridge {
    /// `getChangesToken(ChangesTokenRequest(types))`.
    fn changes_token(&self, record_types: &[String]) -> Result<String, BridgeError>;
    /// `getChanges(token)`.
    fn changes(&self, token: &str) -> Result<HealthConnectChanges, BridgeError>;
}

#[derive(Debug, Serialize, Deserialize)]
struct HealthConnectAnchor {
    v: u32,
    token: String,
}

/// Identity of a Health Connect record.
pub fn health_connect_native_id(id: &str) -> String {
    format!("HealthConnect/{id}")
}

/// [`HealthSourceAdapter`] over a [`HealthConnectBridge`], gated by scope.
pub struct HealthConnectAdapter<B> {
    bridge: B,
    scope: HealthConnectScope,
}

impl<B: HealthConnectBridge> HealthConnectAdapter<B> {
    /// Adapter with the default (disabled) scope.
    pub fn new(bridge: B) -> Self {
        Self {
            bridge,
            scope: HealthConnectScope::default(),
        }
    }

    /// Replace the scope.
    pub fn with_scope(mut self, scope: HealthConnectScope) -> Self {
        self.scope = scope;
        self
    }

    /// Current scope.
    pub fn scope(&self) -> &HealthConnectScope {
        &self.scope
    }
}

fn adapter_error(error: BridgeError) -> AdapterError {
    match error {
        BridgeError::PermissionDenied => AdapterError::PermissionLost,
        BridgeError::StoreUnavailable => AdapterError::StoreUnavailable,
        BridgeError::AnchorInvalidated => AdapterError::AnchorInvalidated,
        BridgeError::Unsupported => AdapterError::Failed("unsupported".into()),
        BridgeError::Failed { message } => AdapterError::Failed(message),
    }
}

impl<B: HealthConnectBridge> HealthSourceAdapter for HealthConnectAdapter<B> {
    fn source(&self) -> PhoneSource {
        PhoneSource::HealthConnect
    }

    fn read_page(&self, request: &PageRequest) -> Result<AcquisitionPage, AdapterError> {
        let types = match &self.scope {
            HealthConnectScope::Disabled => {
                return Ok(AcquisitionPage {
                    coverage: vec![TypeCoverage {
                        data_type: HEALTH_CONNECT_FEED.into(),
                        outcome: CoverageOutcome::Error {
                            error: CoverageError::NotEnabled,
                        },
                    }],
                    next_anchor: request.anchor.clone(),
                    ..AcquisitionPage::default()
                });
            }
            HealthConnectScope::Enabled { types } => types,
        };
        let Some(anchor) = &request.anchor else {
            // Establish the changes token. Initial history needs type-specific
            // reads, which wait on the agreed scope: report it, never "empty".
            let names: Vec<String> = types.iter().map(|t| t.data_type.clone()).collect();
            let token = self.bridge.changes_token(&names).map_err(adapter_error)?;
            let mut coverage: Vec<TypeCoverage> = types
                .iter()
                .map(|spec| TypeCoverage {
                    data_type: spec.data_type.clone(),
                    outcome: CoverageOutcome::Error {
                        error: CoverageError::QueryFailed {
                            message: "initial_history_not_read".into(),
                        },
                    },
                })
                .collect();
            coverage.push(TypeCoverage {
                data_type: HEALTH_CONNECT_FEED.into(),
                outcome: CoverageOutcome::Empty,
            });
            return Ok(AcquisitionPage {
                coverage,
                next_anchor: Some(encode_hc_anchor(token)),
                has_more: true,
                ..AcquisitionPage::default()
            });
        };
        let token = serde_json::from_str::<HealthConnectAnchor>(&anchor.0)
            .ok()
            .filter(|parsed| parsed.v == COMPOSITE_VERSION)
            .ok_or(AdapterError::AnchorInvalidated)?
            .token;
        let changes = self.bridge.changes(&token).map_err(adapter_error)?;
        if changes.token_expired {
            return Err(AdapterError::AnchorInvalidated);
        }
        // Health Connect fixes its own page size; one token page is committed
        // whole even when it is larger than `request.limit`.
        let by_type: BTreeMap<&str, &TypeSpec> =
            types.iter().map(|t| (t.data_type.as_str(), t)).collect();
        let mut page = AcquisitionPage {
            has_more: changes.has_more,
            next_anchor: Some(encode_hc_anchor(changes.next_token)),
            ..AcquisitionPage::default()
        };
        let mut with_data: BTreeSet<String> = BTreeSet::new();
        let mut unextracted: BTreeSet<String> = BTreeSet::new();
        for upsert in changes.upserts {
            let Some(spec) = by_type.get(upsert.record_type.as_str()) else {
                continue; // Out of scope: never uploaded.
            };
            if upsert.sample.value.is_empty() {
                // The bridge has no exact-value extraction for this type yet.
                // Never upload an empty literal; report the gap instead.
                unextracted.insert(spec.data_type.clone());
                continue;
            }
            with_data.insert(spec.data_type.clone());
            let sample = upsert.sample;
            let provenance = Provenance {
                source_name: sample
                    .writer_id
                    .as_deref()
                    .map(crate::source_name)
                    .filter(|name| !name.is_empty())
                    .map(str::to_owned)
                    .or(sample.source_name.clone()),
                writer_id: sample.writer_id.clone(),
                device: sample.device.clone(),
            };
            let native_id = health_connect_native_id(&sample.id);
            page.records
                .push(sample.into_record(native_id, spec, provenance));
        }
        for spec in types {
            page.coverage.push(TypeCoverage {
                data_type: spec.data_type.clone(),
                outcome: if unextracted.contains(&spec.data_type) {
                    CoverageOutcome::Error {
                        error: CoverageError::QueryFailed {
                            message: "value_not_extracted".into(),
                        },
                    }
                } else if with_data.contains(&spec.data_type) {
                    CoverageOutcome::Data
                } else {
                    CoverageOutcome::Empty
                },
            });
        }
        page.coverage.push(TypeCoverage {
            data_type: HEALTH_CONNECT_FEED.into(),
            outcome: if changes.deleted.is_empty() && page.records.is_empty() {
                CoverageOutcome::Empty
            } else {
                CoverageOutcome::Data
            },
        });
        for id in changes.deleted {
            page.deletions.push(DeletionRecord {
                native_id: health_connect_native_id(&id),
                data_type: HEALTH_CONNECT_FEED.into(),
            });
        }
        Ok(page)
    }
}

fn encode_hc_anchor(token: String) -> Anchor {
    Anchor(
        serde_json::to_string(&HealthConnectAnchor {
            v: COMPOSITE_VERSION,
            token,
        })
        .unwrap_or_default(),
    )
}

/// Decodes a Health Connect changes-token reply (`{"ok": "<token>"}`).
pub fn decode_changes_token_reply(body: &str) -> Result<String, BridgeError> {
    decode_reply(body)
}

/// Decodes a Health Connect changes reply (`{"ok": HealthConnectChanges}`).
pub fn decode_changes_reply(body: &str) -> Result<HealthConnectChanges, BridgeError> {
    decode_reply(body)
}

// ---------------------------------------------------------------------------
// Access honesty
// ---------------------------------------------------------------------------

/// What the app may claim about read access. Completing the OS permission
/// sheet is not evidence of readable data: HealthKit never reports a denied
/// read, so only a successful read that returned data proves readability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadAccess {
    /// Access was never requested.
    NotRequested,
    /// The request flow finished; nothing has been read yet (or reads were empty).
    RequestCompleted,
    /// At least one type returned data in a successful read.
    ReadableData,
    /// Reads are failing or permission was lost.
    Unreadable,
}

/// Derive [`ReadAccess`] from the request flag and recorded coverage.
pub fn read_access<'a>(
    request_completed: bool,
    coverage: impl IntoIterator<Item = &'a CoverageOutcome>,
) -> ReadAccess {
    let outcomes: Vec<&CoverageOutcome> = coverage.into_iter().collect();
    if outcomes.iter().any(|o| **o == CoverageOutcome::Data) {
        return ReadAccess::ReadableData;
    }
    if !request_completed {
        return ReadAccess::NotRequested;
    }
    if !outcomes.is_empty() && outcomes.iter().all(|o| !o.read_succeeded()) {
        return ReadAccess::Unreadable;
    }
    ReadAccess::RequestCompleted
}

// ---------------------------------------------------------------------------
// Background delivery hand-off
// ---------------------------------------------------------------------------

/// What a background delivery pass returned to the native observer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum BackgroundSignal {
    /// New changes (or none) were durably written to the outbox.
    Durable = 0,
    /// A typed pause (locked storage, account change, ...): nothing lost.
    Paused = 1,
    /// No signed-in session has registered a handler.
    NoSession = 2,
}

type BackgroundHandler = Box<dyn Fn(&str) -> BackgroundSignal + Send + Sync>;

static BACKGROUND_HANDLER: std::sync::Mutex<Option<BackgroundHandler>> =
    std::sync::Mutex::new(None);

/// Install the handler the native observer calls. The app installs one per
/// signed-in session and clears it on sign-out. The handler must return only
/// after its outbox write (or a typed pause); it must not wait for upload.
pub fn set_background_handler(handler: Option<BackgroundHandler>) {
    if let Ok(mut slot) = BACKGROUND_HANDLER.lock() {
        *slot = handler;
    }
}

/// Run the background handler for `data_type`. Called by the native observer
/// before it calls its completion handler.
pub fn background_delivery(data_type: &str) -> BackgroundSignal {
    match BACKGROUND_HANDLER.lock() {
        Ok(slot) => slot
            .as_ref()
            .map_or(BackgroundSignal::NoSession, |handler| handler(data_type)),
        Err(_) => BackgroundSignal::Paused,
    }
}

/// Register `HKObserverQuery` + background delivery for `types`. Each update
/// calls [`background_delivery`] and completes the observer afterwards. A
/// no-op off iOS.
pub fn start_healthkit_observers(types: &[TypeSpec]) {
    #[cfg(target_os = "ios")]
    {
        unsafe extern "C" {
            fn duet_healthkit_start_observers(
                types: *const std::ffi::c_char,
                deliver: unsafe extern "C" fn(*const std::ffi::c_char) -> i32,
            );
        }
        let names: Vec<&str> = types.iter().map(|t| t.data_type.as_str()).collect();
        if let Ok(json) = std::ffi::CString::new(serde_json::to_string(&names).unwrap_or_default())
        {
            unsafe {
                duet_healthkit_start_observers(json.as_ptr(), duet_healthkit_background_delivery)
            }
        }
    }
    #[cfg(not(target_os = "ios"))]
    let _ = types;
}

/// C entry for `HKObserverQuery` updates. Blocks until the durable outbox
/// hand-off (or a typed pause) finishes, then returns a [`BackgroundSignal`].
///
/// # Safety
/// `data_type` must be null or a valid NUL-terminated string.
#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn duet_healthkit_background_delivery(
    data_type: *const std::ffi::c_char,
) -> i32 {
    let name = if data_type.is_null() {
        String::new()
    } else {
        unsafe { std::ffi::CStr::from_ptr(data_type) }
            .to_string_lossy()
            .into_owned()
    };
    background_delivery(&name) as i32
}

// ---------------------------------------------------------------------------
// Platform bridges
// ---------------------------------------------------------------------------

/// HealthKit bridge over the Swift `@_cdecl` functions in `platform/ios`.
#[cfg(target_os = "ios")]
#[derive(Clone, Copy, Debug, Default)]
pub struct IosHealthKitBridge;

#[cfg(target_os = "ios")]
impl HealthKitBridge for IosHealthKitBridge {
    fn available(&self) -> bool {
        unsafe extern "C" {
            fn duet_healthkit_available() -> i32;
        }
        unsafe { duet_healthkit_available() != 0 }
    }

    fn read_type(&self, query: &HealthKitQuery) -> Result<HealthKitChanges, BridgeError> {
        unsafe extern "C" {
            fn duet_healthkit_read_type(request: *const std::ffi::c_char) -> *mut std::ffi::c_char;
            fn duet_healthkit_string_free(pointer: *mut std::ffi::c_char);
        }
        let request = serde_json::to_string(query).map_err(|e| BridgeError::Failed {
            message: e.to_string(),
        })?;
        let request = std::ffi::CString::new(request).map_err(|e| BridgeError::Failed {
            message: e.to_string(),
        })?;
        let pointer = unsafe { duet_healthkit_read_type(request.as_ptr()) };
        if pointer.is_null() {
            return Err(BridgeError::Failed {
                message: "bridge returned no reply".into(),
            });
        }
        let body = unsafe { std::ffi::CStr::from_ptr(pointer) }
            .to_string_lossy()
            .into_owned();
        unsafe { duet_healthkit_string_free(pointer) };
        decode_healthkit_reply(&body)
    }
}

/// Health Connect bridge over the Kotlin `HealthConnectBridge` object.
#[cfg(target_os = "android")]
#[derive(Clone, Copy, Debug, Default)]
pub struct AndroidHealthConnectBridge;

#[cfg(target_os = "android")]
impl HealthConnectBridge for AndroidHealthConnectBridge {
    fn changes_token(&self, record_types: &[String]) -> Result<String, BridgeError> {
        let request = serde_json::to_string(record_types).unwrap_or_else(|_| "[]".into());
        let body = crate::android_call_string(jni::jni_str!("getChangesToken"), &request)
            .map_err(|message| BridgeError::Failed { message })?;
        decode_changes_token_reply(&body)
    }

    fn changes(&self, token: &str) -> Result<HealthConnectChanges, BridgeError> {
        let body = crate::android_call_string(jni::jni_str!("getChanges"), token)
            .map_err(|message| BridgeError::Failed { message })?;
        decode_changes_reply(&body)
    }
}
