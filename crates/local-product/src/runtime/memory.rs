//! Durable patient memory per practice and patient workspace.
//!
//! Coda holds one revisable memory per patient, separate from per-conversation
//! harness working notes ([`super::working_notes`]) and separate from the
//! physician-facing summary shown on the patient page.
//!
//! - [`OperatingIndex`] holds Coda's private working understanding by
//!   model-chosen [`TopicKey`]s. Physicians never see it, and physician-facing
//!   types ([`PatientSummary`], [`SavedPatientSummary`], [`SummaryRead`]) and
//!   physician-scoped search ([`SearchAudienceScope::PhysicianFacing`])
//!   structurally exclude it.
//! - [`MemoryCorrection`] records physician corrections with an unambiguous
//!   [`ClinicalDate`] and exact [`SourceMessageRef`] while keeping unrelated
//!   understanding and unrelated corrections intact.
//! - Every accepted mutation consumes a once-only command receipt in
//!   `physician_runtime_commands` inside the same Postgres transaction and
//!   appends an auditable version in `physician_runtime_patient_memory_versions`.

use std::collections::{BTreeMap, BTreeSet};
use std::num::{NonZeroU64, TryFromIntError};

use harness::engine::{self, Invocation, Tool, ToolOutcome};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait, Unchanged, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::entities::{patient_memory, patient_memory_version};
use super::{Authority, Claim, MessageRole, Scope, Store, now, receipt};

/// Read tool for Coda's patient memory and permitted version history.
pub const READ_MEMORY_TOOL: &str = "read_patient_memory";
/// Mutation tool for patching Coda's private operating index.
pub const SAVE_MEMORY_TOOL: &str = "save_patient_memory";
/// Mutation tool for recording a dated physician correction with source message provenance.
pub const RECORD_CORRECTION_TOOL: &str = "record_patient_correction";
/// Mutation tool for saving the physician-facing patient summary with references and dates.
pub const SAVE_SUMMARY_TOOL: &str = "save_patient_summary";

const VISIBILITY_PERMITTED: &str = "permitted";
const VISIBILITY_RETRACTED: &str = "retracted";

/// Validation failure at a memory domain or tool boundary.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MemoryValidationError {
    /// Practice identifier was blank.
    #[error("practice identifier must not be empty")]
    EmptyPractice,
    /// Patient workspace identifier was blank.
    #[error("patient workspace identifier must not be empty")]
    EmptyWorkspace,
    /// Command receipt key was blank.
    #[error("command receipt key must not be empty")]
    EmptyKey,
    /// Session hash was not a 32-byte SHA-256 digest.
    #[error("session reference must be 32 bytes")]
    InvalidSessionReference,
    /// Operating-index topic identifier was blank or exceeded length bounds.
    #[error("operating-index topic key must be 1..=128 non-whitespace-padded characters")]
    InvalidTopicKey,
    /// Correction identifier was blank.
    #[error("correction identifier must not be empty")]
    EmptyCorrectionId,
    /// Correction statement was blank.
    #[error("correction statement must not be empty")]
    EmptyStatement,
    /// Summary content was blank.
    #[error("patient summary content must not be empty")]
    EmptySummaryContent,
    /// Clinical date did not start with a valid `YYYY-MM-DD` calendar date.
    #[error("clinical date `{value}` is invalid: {reason}")]
    InvalidClinicalDate {
        /// Rejected input string.
        value: String,
        /// Specific calendar rule that failed.
        reason: &'static str,
    },
    /// Source message reference omitted a required identifier.
    #[error("source message reference requires non-empty chat_id, message_id, and physician_id")]
    InvalidSourceMessage,
    /// Summary citation omitted a required identifier or label.
    #[error("summary reference requires non-empty identifiers: {0}")]
    InvalidSummaryReference(&'static str),
    /// Operating-index patch neither upserted nor removed any topic.
    #[error("operating-index patch must upsert or remove at least one topic")]
    EmptyUnderstandingPatch,
    /// Tool patch listed the same topic more than once in `upsert`.
    #[error("operating-index patch listed topic `{topic}` more than once")]
    DuplicatePatchTopic {
        /// Duplicated topic key.
        topic: String,
    },
    /// Operating-index patch both upserted and removed the same topic.
    #[error("operating-index patch cannot both upsert and remove topic `{topic}`")]
    OverlappingPatchTopic {
        /// Conflicting topic key.
        topic: String,
    },
    /// Version retraction omitted a reason.
    #[error("version retraction reason must not be empty")]
    EmptyRetractionReason,
    /// Author or job identity omitted a required field.
    #[error("memory author field `{0}` must not be empty")]
    EmptyAuthorField(&'static str),
}

/// Underlying cause of a corrupt persisted patient-memory row.
#[derive(Debug, thiserror::Error)]
pub enum CorruptRecordSource {
    /// Stored JSON did not match the owning Rust representation.
    #[error("{0}")]
    Json(#[source] serde_json::Error),
    /// Stored integer was negative, zero where forbidden, or out of range.
    #[error("{0}")]
    Integer(#[source] TryFromIntError),
    /// Stored domain value violated a boundary invariant.
    #[error("{0}")]
    Validation(#[source] MemoryValidationError),
    /// Stored relational state violated a cross-column invariant.
    #[error("{0}")]
    Invariant(&'static str),
}

/// Failure on a patient-memory store operation.
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    /// Storage query or transaction failed.
    #[error("{0}")]
    Database(#[source] DbErr),
    /// Stored record could not be decoded into valid domain state.
    #[error("corrupt patient memory record ({context}): {source}")]
    CorruptRecord {
        /// Column or projection being decoded.
        context: &'static str,
        /// Underlying decode or invariant cause.
        #[source]
        source: CorruptRecordSource,
    },
    /// Caller supplied invalid boundary input.
    #[error("{0}")]
    Validation(#[source] MemoryValidationError),
}

impl From<DbErr> for MemoryError {
    fn from(error: DbErr) -> Self {
        Self::Database(error)
    }
}

impl From<MemoryValidationError> for MemoryError {
    fn from(error: MemoryValidationError) -> Self {
        Self::Validation(error)
    }
}

fn corrupt_json(context: &'static str, error: serde_json::Error) -> MemoryError {
    MemoryError::CorruptRecord {
        context,
        source: CorruptRecordSource::Json(error),
    }
}

fn corrupt_int(context: &'static str, error: TryFromIntError) -> MemoryError {
    MemoryError::CorruptRecord {
        context,
        source: CorruptRecordSource::Integer(error),
    }
}

fn corrupt_invariant(context: &'static str, reason: &'static str) -> MemoryError {
    MemoryError::CorruptRecord {
        context,
        source: CorruptRecordSource::Invariant(reason),
    }
}

/// Validated practice and patient workspace identity owning one patient memory.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatientKey {
    practice: String,
    workspace: String,
}

impl PatientKey {
    /// Validates non-empty practice and patient workspace identifiers.
    pub fn new(
        practice: impl Into<String>,
        workspace: impl Into<String>,
    ) -> Result<Self, MemoryValidationError> {
        let practice = practice.into();
        let workspace = workspace.into();
        if practice.trim().is_empty() {
            return Err(MemoryValidationError::EmptyPractice);
        }
        if workspace.trim().is_empty() {
            return Err(MemoryValidationError::EmptyWorkspace);
        }
        Ok(Self {
            practice,
            workspace,
        })
    }

    /// Practice tenant identifier.
    pub fn practice(&self) -> &str {
        &self.practice
    }

    /// Patient workspace identifier.
    pub fn workspace(&self) -> &str {
        &self.workspace
    }
}

/// Stable identifier of one patient's memory within a practice.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MemoryId(String);

impl MemoryId {
    /// Deterministic memory identity for `patient`.
    pub fn for_patient(patient: &PatientKey) -> Self {
        let canonical =
            serde_json::to_string(&(patient.practice(), patient.workspace())).expect("tuple json");
        Self(format!("pmem_{}", crate::sha256_hex(canonical.as_bytes())))
    }

    /// Borrowed identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_stored(value: String) -> Result<Self, MemoryError> {
        if value.trim().is_empty() {
            return Err(corrupt_invariant("memory_id", "blank stored memory_id"));
        }
        Ok(Self(value))
    }
}

/// Positive 1-based version number of a patient memory snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MemoryVersionNumber(NonZeroU64);

impl MemoryVersionNumber {
    /// First version of a patient memory (`1`).
    pub const FIRST: Self = Self(NonZeroU64::MIN);

    /// Wraps `value` when `value >= 1`.
    pub fn new(value: u64) -> Option<Self> {
        NonZeroU64::new(value).map(Self)
    }

    /// Underlying 1-based integer.
    pub fn get(self) -> u64 {
        self.0.get()
    }

    fn next(self) -> Result<Self, MemoryError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or_else(|| corrupt_invariant("version", "memory version overflow"))
    }

    fn to_db(self) -> Result<i64, MemoryError> {
        i64::try_from(self.0.get()).map_err(|err| corrupt_int("version->i64", err))
    }

    fn from_db(context: &'static str, value: i64) -> Result<Self, MemoryError> {
        let unsigned = u64::try_from(value).map_err(|err| corrupt_int(context, err))?;
        Self::new(unsigned).ok_or_else(|| corrupt_invariant(context, "version must be >= 1"))
    }
}

/// Validated calendar date or RFC 3339 timestamp beginning with `YYYY-MM-DD`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct ClinicalDate(String);

impl ClinicalDate {
    /// Validates that `value` starts with a real `YYYY-MM-DD` calendar date.
    pub fn parse(value: impl Into<String>) -> Result<Self, MemoryValidationError> {
        let value = value.into();
        validate_clinical_date(&value)?;
        Ok(Self(value))
    }

    /// Full date or timestamp text as recorded.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Exact `YYYY-MM-DD` calendar date prefix.
    pub fn calendar_date(&self) -> &str {
        &self.0[..10]
    }
}

impl<'de> Deserialize<'de> for ClinicalDate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::parse(raw).map_err(serde::de::Error::custom)
    }
}

fn parse_digits(bytes: &[u8]) -> Option<u16> {
    if bytes.is_empty() || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let mut acc = 0_u16;
    for &byte in bytes {
        acc = acc.checked_mul(10)?.checked_add(u16::from(byte - b'0'))?;
    }
    Some(acc)
}

fn validate_rfc3339_time_suffix(suffix: &[u8]) -> Result<(), &'static str> {
    if suffix.first() != Some(&b'T') {
        return Err("timestamp suffix must start with 'T'");
    }
    if suffix.len() < 10 || suffix[3] != b':' || suffix[6] != b':' {
        return Err("timestamp suffix must match THH:MM:SS[.frac](Z|±HH:MM)");
    }
    let _hour = parse_digits(&suffix[1..3])
        .filter(|hour| *hour <= 23)
        .ok_or("timestamp hour must be in 00..=23")?;
    let _minute = parse_digits(&suffix[4..6])
        .filter(|minute| *minute <= 59)
        .ok_or("timestamp minute must be in 00..=59")?;
    let _second = parse_digits(&suffix[7..9])
        .filter(|second| *second <= 60)
        .ok_or("timestamp second must be in 00..=60")?;

    let mut rest = &suffix[9..];
    if let Some(after_dot) = rest.strip_prefix(b".") {
        let frac_len = after_dot
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        if !(1..=9).contains(&frac_len) {
            return Err("timestamp fractional seconds must be 1..=9 digits");
        }
        rest = &after_dot[frac_len..];
    }

    if rest == b"Z" {
        return Ok(());
    }
    if rest.len() == 6 && matches!(rest[0], b'+' | b'-') && rest[3] == b':' {
        let _tz_hour = parse_digits(&rest[1..3])
            .filter(|hour| *hour <= 23)
            .ok_or("timezone offset hour must be in 00..=23")?;
        let _tz_minute = parse_digits(&rest[4..6])
            .filter(|minute| *minute <= 59)
            .ok_or("timezone offset minute must be in 00..=59")?;
        return Ok(());
    }
    Err("timestamp timezone must be 'Z' or ±HH:MM")
}

fn validate_clinical_date(value: &str) -> Result<(), MemoryValidationError> {
    let invalid = |reason: &'static str| MemoryValidationError::InvalidClinicalDate {
        value: value.to_owned(),
        reason,
    };
    if value.trim() != value || value.len() < 10 {
        return Err(invalid("expected YYYY-MM-DD with four-digit year"));
    }
    let bytes = value.as_bytes();
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return Err(invalid("expected YYYY-MM-DD separators"));
    }
    let year = parse_digits(&bytes[0..4])
        .filter(|year| (1900..=9999).contains(year))
        .ok_or_else(|| invalid("year must be in 1900..=9999"))?;
    let month = parse_digits(&bytes[5..7])
        .filter(|month| (1..=12).contains(month))
        .ok_or_else(|| invalid("month must be in 01..=12"))?;
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap =
                (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400);
            if leap { 29 } else { 28 }
        }
        _ => return Err(invalid("month must be in 01..=12")),
    };
    let _day = parse_digits(&bytes[8..10])
        .filter(|day| (1..=max_day).contains(day))
        .ok_or_else(|| invalid("day is out of range for month"))?;
    if bytes.len() > 10 {
        validate_rfc3339_time_suffix(&bytes[10..]).map_err(invalid)?;
    }
    Ok(())
}

/// Model-chosen topic identifier inside Coda's private [`OperatingIndex`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct TopicKey(String);

impl TopicKey {
    /// Validates a non-empty topic key without leading or trailing whitespace.
    pub fn parse(value: impl Into<String>) -> Result<Self, MemoryValidationError> {
        let value = value.into();
        if value.is_empty() || value.trim() != value || value.chars().count() > 128 {
            return Err(MemoryValidationError::InvalidTopicKey);
        }
        Ok(Self(value))
    }

    /// Borrowed topic key text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for TopicKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::parse(raw).map_err(serde::de::Error::custom)
    }
}

/// Coda's private, revisable operating index for one patient.
///
/// Topics are model-chosen rather than a fixed clinical schema. Physicians
/// never see this type; it has no `Display` implementation and no conversion
/// into [`PatientSummary`] or [`SavedPatientSummary`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatingIndex {
    entries: BTreeMap<TopicKey, String>,
}

impl OperatingIndex {
    /// Builds an operating index from validated topic notes.
    pub fn from_entries(entries: BTreeMap<TopicKey, String>) -> Self {
        Self { entries }
    }

    /// Borrowed topic notes in deterministic key order, for internal agent use.
    pub fn entries(&self) -> &BTreeMap<TopicKey, String> {
        &self.entries
    }

    /// Looks up one topic's current note.
    pub fn get(&self, topic: &TopicKey) -> Option<&str> {
        self.entries.get(topic).map(String::as_str)
    }

    /// Whether no topics are currently recorded.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Number of recorded topics.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    fn apply_patch(&self, patch: &OperatingIndexPatch) -> Self {
        let mut entries = self.entries.clone();
        for removed in &patch.remove {
            entries.remove(removed);
        }
        for (topic, note) in &patch.upsert {
            entries.insert(topic.clone(), note.clone());
        }
        Self { entries }
    }
}

/// Incremental update to [`OperatingIndex`] that leaves untouched topics intact.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatingIndexPatch {
    upsert: BTreeMap<TopicKey, String>,
    remove: BTreeSet<TopicKey>,
}

impl OperatingIndexPatch {
    /// Validates that `upsert` and `remove` are disjoint and at least one is non-empty.
    pub fn new(
        upsert: BTreeMap<TopicKey, String>,
        remove: BTreeSet<TopicKey>,
    ) -> Result<Self, MemoryValidationError> {
        if upsert.is_empty() && remove.is_empty() {
            return Err(MemoryValidationError::EmptyUnderstandingPatch);
        }
        if let Some(overlap) = upsert.keys().find(|topic| remove.contains(*topic)) {
            return Err(MemoryValidationError::OverlappingPatchTopic {
                topic: overlap.as_str().to_owned(),
            });
        }
        Ok(Self { upsert, remove })
    }

    /// Validates a potentially empty optional patch (used alongside a correction).
    pub fn optional(
        upsert: BTreeMap<TopicKey, String>,
        remove: BTreeSet<TopicKey>,
    ) -> Result<Option<Self>, MemoryValidationError> {
        if upsert.is_empty() && remove.is_empty() {
            return Ok(None);
        }
        Self::new(upsert, remove).map(Some)
    }

    /// Topics being added or updated.
    pub fn upsert(&self) -> &BTreeMap<TopicKey, String> {
        &self.upsert
    }

    /// Topics being removed.
    pub fn remove(&self) -> &BTreeSet<TopicKey> {
        &self.remove
    }

    fn validate(&self) -> Result<(), MemoryValidationError> {
        if self.upsert.is_empty() && self.remove.is_empty() {
            return Err(MemoryValidationError::EmptyUnderstandingPatch);
        }
        if let Some(overlap) = self
            .upsert
            .keys()
            .find(|topic| self.remove.contains(*topic))
        {
            return Err(MemoryValidationError::OverlappingPatchTopic {
                topic: overlap.as_str().to_owned(),
            });
        }
        Ok(())
    }
}

/// Stable identifier of one physician correction inside a patient's memory.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct CorrectionId(String);

impl CorrectionId {
    /// Validates a non-empty correction identifier.
    pub fn parse(value: impl Into<String>) -> Result<Self, MemoryValidationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(MemoryValidationError::EmptyCorrectionId);
        }
        Ok(Self(value))
    }

    /// Deterministic correction identifier derived from a mutation receipt.
    pub fn from_receipt(practice: &str, actor: &str, key: &str) -> Self {
        let canonical = serde_json::to_string(&(practice, actor, key)).expect("tuple json");
        Self(format!(
            "corr_{}",
            &crate::sha256_hex(canonical.as_bytes())[..24]
        ))
    }

    /// Borrowed identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for CorrectionId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::parse(raw).map_err(serde::de::Error::custom)
    }
}

/// Exact conversation message where a physician stated or withdrew a correction.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceMessageRef {
    /// Conversation identifier holding the message.
    pub chat_id: String,
    /// Durable authored message identifier.
    pub message_id: String,
    /// Physician who authored the message.
    pub physician_id: String,
}

impl SourceMessageRef {
    /// Validates non-empty chat, message, and physician identifiers.
    pub fn new(
        chat_id: impl Into<String>,
        message_id: impl Into<String>,
        physician_id: impl Into<String>,
    ) -> Result<Self, MemoryValidationError> {
        let value = Self {
            chat_id: chat_id.into(),
            message_id: message_id.into(),
            physician_id: physician_id.into(),
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), MemoryValidationError> {
        if self.chat_id.trim().is_empty()
            || self.message_id.trim().is_empty()
            || self.physician_id.trim().is_empty()
        {
            return Err(MemoryValidationError::InvalidSourceMessage);
        }
        Ok(())
    }
}

/// Current lifecycle standing of a recorded [`MemoryCorrection`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum CorrectionStanding {
    /// Active correction governing Coda's current understanding.
    Active,
    /// Replaced by a newer correction while remaining auditable in history.
    Superseded {
        /// Newer correction that replaced this one.
        superseded_by: CorrectionId,
    },
    /// Explicitly withdrawn by the physician while remaining auditable in history.
    Withdrawn {
        /// Date the withdrawal was stated.
        withdrawn_on: ClinicalDate,
        /// Exact message that withdrew the correction.
        source_message: SourceMessageRef,
        /// Reason recorded for the withdrawal.
        reason: String,
    },
}

impl CorrectionStanding {
    fn validate(&self) -> Result<(), MemoryValidationError> {
        match self {
            Self::Active => Ok(()),
            Self::Superseded { superseded_by } => {
                CorrectionId::parse(superseded_by.as_str()).map(|_| ())
            }
            Self::Withdrawn {
                withdrawn_on,
                source_message,
                reason,
            } => {
                validate_clinical_date(withdrawn_on.as_str())?;
                source_message.validate()?;
                if reason.trim().is_empty() {
                    return Err(MemoryValidationError::EmptyStatement);
                }
                Ok(())
            }
        }
    }
}

/// One dated physician correction retained in patient memory.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryCorrection {
    /// Stable correction identifier.
    pub correction_id: CorrectionId,
    /// Calendar date or timestamp when the correction was stated.
    pub recorded_on: ClinicalDate,
    /// Exact message provenance of the correction.
    pub source_message: SourceMessageRef,
    /// Physician correction statement.
    pub statement: String,
    /// Whether the correction is active, superseded, or withdrawn.
    pub standing: CorrectionStanding,
}

impl MemoryCorrection {
    fn validate(&self) -> Result<(), MemoryValidationError> {
        CorrectionId::parse(self.correction_id.as_str())?;
        validate_clinical_date(self.recorded_on.as_str())?;
        self.source_message.validate()?;
        if self.statement.trim().is_empty() {
            return Err(MemoryValidationError::EmptyStatement);
        }
        self.standing.validate()
    }
}

/// Citation attached to a physician-facing [`PatientSummary`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SummaryReference {
    /// Exact retained chart record revision in this patient workspace.
    ChartRecord {
        /// Source adapter identifier.
        source_id: String,
        /// Retained chart candidate identifier.
        candidate_id: String,
        /// Immutable source revision identifier.
        revision: String,
        /// Physician-readable label for the cited record.
        label: String,
        /// Clinical date on the cited record, when known.
        record_date: Option<ClinicalDate>,
    },
    /// Authored message in a physician chat in this patient workspace.
    ConversationMessage {
        /// Conversation identifier.
        chat_id: String,
        /// Authored message identifier.
        message_id: String,
        /// Physician-readable label for the cited message.
        label: String,
        /// Date of the cited message, when known.
        stated_on: Option<ClinicalDate>,
    },
    /// External clinical guideline or literature citation.
    Literature {
        /// Citation text (title, journal/guideline, year).
        citation: String,
        /// Persistent identifier such as PMID or DOI, when available.
        identifier: Option<String>,
    },
}

impl SummaryReference {
    fn validate(&self) -> Result<(), MemoryValidationError> {
        match self {
            Self::ChartRecord {
                source_id,
                candidate_id,
                revision,
                label,
                record_date,
            } => {
                if source_id.trim().is_empty()
                    || candidate_id.trim().is_empty()
                    || revision.trim().is_empty()
                    || label.trim().is_empty()
                {
                    return Err(MemoryValidationError::InvalidSummaryReference(
                        "chart_record requires source_id, candidate_id, revision, and label",
                    ));
                }
                if let Some(date) = record_date {
                    validate_clinical_date(date.as_str())?;
                }
                Ok(())
            }
            Self::ConversationMessage {
                chat_id,
                message_id,
                label,
                stated_on,
            } => {
                if chat_id.trim().is_empty()
                    || message_id.trim().is_empty()
                    || label.trim().is_empty()
                {
                    return Err(MemoryValidationError::InvalidSummaryReference(
                        "conversation_message requires chat_id, message_id, and label",
                    ));
                }
                if let Some(date) = stated_on {
                    validate_clinical_date(date.as_str())?;
                }
                Ok(())
            }
            Self::Literature {
                citation,
                identifier,
            } => {
                if citation.trim().is_empty() {
                    return Err(MemoryValidationError::InvalidSummaryReference(
                        "literature requires citation",
                    ));
                }
                if identifier.as_ref().is_some_and(|id| id.trim().is_empty()) {
                    return Err(MemoryValidationError::InvalidSummaryReference(
                        "literature identifier must not be blank when present",
                    ));
                }
                Ok(())
            }
        }
    }
}

/// Dates carried on the physician-facing [`PatientSummary`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummaryDates {
    /// When Coda wrote this summary text.
    pub written_at: ClinicalDate,
    /// Latest periodic or clinical review date covered by this summary, when any.
    pub reviewed_at: Option<ClinicalDate>,
    /// Latest chart/source pull date reflected in this summary, when any.
    pub pulled_at: Option<ClinicalDate>,
}

impl SummaryDates {
    fn validate(&self) -> Result<(), MemoryValidationError> {
        validate_clinical_date(self.written_at.as_str())?;
        if let Some(reviewed) = &self.reviewed_at {
            validate_clinical_date(reviewed.as_str())?;
        }
        if let Some(pulled) = &self.pulled_at {
            validate_clinical_date(pulled.as_str())?;
        }
        Ok(())
    }
}

/// Physician-facing summary artifact for the patient page.
///
/// Structurally distinct from [`OperatingIndex`]: contains only physician-readable
/// prose, typed [`SummaryReference`]s, and [`SummaryDates`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatientSummary {
    /// Physician-readable summary text.
    pub content: String,
    /// Grounded citations backing the summary.
    pub references: Vec<SummaryReference>,
    /// Generation, review, and data-pull dates for the summary.
    pub dates: SummaryDates,
}

impl PatientSummary {
    /// Validates non-empty content, references, and clinical dates.
    pub fn validate(&self) -> Result<(), MemoryValidationError> {
        if self.content.trim().is_empty() {
            return Err(MemoryValidationError::EmptySummaryContent);
        }
        for reference in &self.references {
            reference.validate()?;
        }
        self.dates.validate()
    }
}

/// Saved physician-facing summary read back for the patient page.
///
/// Structurally carries no [`OperatingIndex`] and rejects unknown JSON fields.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedPatientSummary {
    /// Patient memory identity.
    pub memory_id: MemoryId,
    /// Practice tenant identifier.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Exact memory version that authored this summary.
    pub summary_version: MemoryVersionNumber,
    /// Current head version of the patient memory.
    pub memory_version: MemoryVersionNumber,
    /// Database-clock seconds when the current head snapshot was saved.
    pub saved_at: i64,
    /// Summary content, references, and dates.
    pub summary: PatientSummary,
}

/// Provenance of who authored a patient memory version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "actor", rename_all = "snake_case", deny_unknown_fields)]
pub enum MemoryAuthor {
    /// Direct authenticated physician action.
    PhysicianDirect {
        /// Signed-in physician identifier.
        physician_id: String,
    },
    /// Tool call inside an authorized physician chat job.
    ChatJob {
        /// Signed-in physician who owns the chat.
        physician_id: String,
        /// Conversation identifier.
        chat_id: String,
        /// Accepted chat work identifier.
        work_id: String,
        /// Tool invocation identifier within that work.
        call_id: String,
    },
    /// Practice-owned background job (such as periodic review or chart intake).
    PracticeJob {
        /// Kind of practice job.
        job_kind: String,
        /// Durable job run identifier.
        job_id: String,
    },
}

impl MemoryAuthor {
    fn validate(&self) -> Result<(), MemoryValidationError> {
        match self {
            Self::PhysicianDirect { physician_id } => {
                if physician_id.trim().is_empty() {
                    return Err(MemoryValidationError::EmptyAuthorField("physician_id"));
                }
                Ok(())
            }
            Self::ChatJob {
                physician_id,
                chat_id,
                work_id,
                call_id,
            } => {
                if physician_id.trim().is_empty() {
                    return Err(MemoryValidationError::EmptyAuthorField("physician_id"));
                }
                if chat_id.trim().is_empty() {
                    return Err(MemoryValidationError::EmptyAuthorField("chat_id"));
                }
                if work_id.trim().is_empty() {
                    return Err(MemoryValidationError::EmptyAuthorField("work_id"));
                }
                if call_id.trim().is_empty() {
                    return Err(MemoryValidationError::EmptyAuthorField("call_id"));
                }
                Ok(())
            }
            Self::PracticeJob { job_kind, job_id } => {
                if job_kind.trim().is_empty() {
                    return Err(MemoryValidationError::EmptyAuthorField("job_kind"));
                }
                if job_id.trim().is_empty() {
                    return Err(MemoryValidationError::EmptyAuthorField("job_id"));
                }
                Ok(())
            }
        }
    }
}

/// Identity of a practice-owned background job authorized to run memory tools.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobMemoryCaller {
    job_kind: String,
    job_id: String,
}

impl JobMemoryCaller {
    /// Validates non-empty job kind and job run identifiers.
    pub fn new(
        job_kind: impl Into<String>,
        job_id: impl Into<String>,
    ) -> Result<Self, MemoryValidationError> {
        let job_kind = job_kind.into();
        let job_id = job_id.into();
        if job_kind.trim().is_empty() {
            return Err(MemoryValidationError::EmptyAuthorField("job_kind"));
        }
        if job_id.trim().is_empty() {
            return Err(MemoryValidationError::EmptyAuthorField("job_id"));
        }
        Ok(Self { job_kind, job_id })
    }

    /// Job kind label (for example `"periodic_review"`).
    pub fn job_kind(&self) -> &str {
        &self.job_kind
    }

    /// Durable job execution identifier.
    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    fn receipt_actor(&self) -> String {
        serde_json::to_string(&("practice_job", &self.job_kind)).expect("tuple json")
    }

    fn receipt_key(&self, key: &str) -> String {
        serde_json::to_string(&(&self.job_id, key)).expect("tuple json")
    }
}

/// How retracting a version settled the patient's current head version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetractionOutcome {
    /// Retracted an earlier non-current version and carried forward the clean current state.
    KeptCurrent,
    /// Retracted a version and installed an explicit clean replacement state as current.
    ReplacedCurrent,
    /// Retracted a version without installing a current replacement, leaving no current permitted version.
    ClearedCurrent,
}

/// Audit summary of what a memory version changed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum MemoryChange {
    /// Patched topics in Coda's private operating index.
    UnderstandingUpdated {
        /// Topics added or updated in this version.
        upserted_topics: Vec<TopicKey>,
        /// Topics removed in this version.
        removed_topics: Vec<TopicKey>,
    },
    /// Recorded a dated physician correction (and any paired topic patch).
    CorrectionRecorded {
        /// Identifier of the new correction.
        correction_id: CorrectionId,
        /// Earlier correction superseded by this one, if any.
        superseded: Option<CorrectionId>,
        /// Topics updated alongside this correction.
        upserted_topics: Vec<TopicKey>,
        /// Topics removed alongside this correction.
        removed_topics: Vec<TopicKey>,
    },
    /// Withdrew an earlier correction while retaining it in audit history.
    CorrectionWithdrawn {
        /// Identifier of the withdrawn correction.
        correction_id: CorrectionId,
    },
    /// Wrote a new physician-facing patient summary.
    SummarySaved,
    /// Retracted an erroneous version from permitted readback and search.
    VersionRetracted {
        /// Version marked retracted.
        retracted_version: MemoryVersionNumber,
        /// How the patient's current head version settled after retraction.
        disposition: RetractionOutcome,
    },
}

/// Audit record stored when a memory version is retracted (for example entered-in-error).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionRetraction {
    /// Version whose commit performed the retraction.
    pub retracted_by_version: MemoryVersionNumber,
    /// Date of the retraction.
    pub retracted_on: ClinicalDate,
    /// Reason the version was withheld.
    pub reason: String,
    /// Author of the retraction.
    pub author: MemoryAuthor,
}

/// Whether a persisted version may be served to clinical readers and search.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum VersionVisibility {
    /// Permitted for clinical readback and search within the patient's practice.
    Permitted,
    /// Withheld from readback and search (such as entered-in-error).
    Retracted(VersionRetraction),
}

/// Physician-facing summary paired with the exact memory version that authored it.
///
/// Bundling `summary_version` and `summary` into one struct prevents a snapshot
/// from carrying a summary without its authoring version or vice versa.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotSummary {
    /// Exact memory version that authored `summary`.
    pub summary_version: MemoryVersionNumber,
    /// Physician-facing summary content, references, and dates.
    pub summary: PatientSummary,
}

/// Complete internal snapshot of one permitted patient memory version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemorySnapshot {
    /// Stable patient memory identity.
    pub memory_id: MemoryId,
    /// Practice tenant identifier.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// 1-based version number of this snapshot.
    pub version: MemoryVersionNumber,
    /// Author of this version.
    pub author: MemoryAuthor,
    /// What this version changed.
    pub change: MemoryChange,
    /// Database-clock seconds when this version was committed.
    pub saved_at: i64,
    /// Coda's private operating index at this version (internal/agent only).
    pub operating_index: OperatingIndex,
    /// Recorded physician corrections at this version.
    pub corrections: Vec<MemoryCorrection>,
    /// Physician-facing summary carried at this version and its authoring version, if any.
    pub summary: Option<SnapshotSummary>,
}

/// Lightweight metadata for one permitted memory version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionHeader {
    /// 1-based version number.
    pub version: MemoryVersionNumber,
    /// Author of this version.
    pub author: MemoryAuthor,
    /// What this version changed.
    pub change: MemoryChange,
    /// Database-clock seconds when saved.
    pub saved_at: i64,
    /// Version that authored the summary carried on this snapshot, if any.
    pub summary_version: Option<MemoryVersionNumber>,
}

impl From<&MemorySnapshot> for VersionHeader {
    fn from(snapshot: &MemorySnapshot) -> Self {
        Self {
            version: snapshot.version,
            author: snapshot.author.clone(),
            change: snapshot.change.clone(),
            saved_at: snapshot.saved_at,
            summary_version: snapshot.summary.as_ref().map(|s| s.summary_version),
        }
    }
}

/// Request to record a new physician correction.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordCorrection {
    /// Optional explicit correction identifier; derived deterministically from the receipt when `None`.
    pub correction_id: Option<CorrectionId>,
    /// Date when the physician stated the correction.
    pub recorded_on: ClinicalDate,
    /// Exact message provenance of the correction.
    pub source_message: SourceMessageRef,
    /// Correction statement in clear clinical language.
    pub statement: String,
    /// Earlier active correction replaced by this one, if any.
    pub supersedes: Option<CorrectionId>,
    /// Optional targeted topic patch so unrelated understanding stays untouched.
    pub understanding_patch: Option<OperatingIndexPatch>,
}

impl RecordCorrection {
    fn validate(&self) -> Result<(), MemoryValidationError> {
        if let Some(id) = &self.correction_id {
            CorrectionId::parse(id.as_str())?;
        }
        validate_clinical_date(self.recorded_on.as_str())?;
        self.source_message.validate()?;
        if self.statement.trim().is_empty() {
            return Err(MemoryValidationError::EmptyStatement);
        }
        if let Some(prior) = &self.supersedes {
            CorrectionId::parse(prior.as_str())?;
        }
        if let Some(patch) = &self.understanding_patch {
            patch.validate()?;
        }
        Ok(())
    }
}

/// Request to withdraw an existing active correction.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WithdrawCorrection {
    /// Correction identifier to withdraw.
    pub correction_id: CorrectionId,
    /// Date when the withdrawal was stated.
    pub withdrawn_on: ClinicalDate,
    /// Exact message provenance of the withdrawal.
    pub source_message: SourceMessageRef,
    /// Reason for withdrawing the correction.
    pub reason: String,
    /// Optional targeted topic patch to restore or revise understanding.
    pub understanding_patch: Option<OperatingIndexPatch>,
}

impl WithdrawCorrection {
    fn validate(&self) -> Result<(), MemoryValidationError> {
        CorrectionId::parse(self.correction_id.as_str())?;
        validate_clinical_date(self.withdrawn_on.as_str())?;
        self.source_message.validate()?;
        if self.reason.trim().is_empty() {
            return Err(MemoryValidationError::EmptyStatement);
        }
        if let Some(patch) = &self.understanding_patch {
            patch.validate()?;
        }
        Ok(())
    }
}

/// Clean replacement state supplied when retracting a contaminated head version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplacementState {
    /// Clean operating index.
    pub operating_index: OperatingIndex,
    /// Clean correction list.
    pub corrections: Vec<MemoryCorrection>,
    /// Clean physician-facing summary, if any.
    pub summary: Option<PatientSummary>,
}

impl ReplacementState {
    fn validate(&self) -> Result<(), MemoryValidationError> {
        for correction in &self.corrections {
            correction.validate()?;
        }
        if let Some(summary) = &self.summary {
            summary.validate()?;
        }
        Ok(())
    }
}

/// Explicit head-state disposition when retracting a patient memory version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "mode",
    content = "state",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum RetractionDisposition {
    /// Retract a non-current prior version while keeping the existing current state
    /// (dropping the carried summary if the retracted version authored it).
    KeepCurrent,
    /// Write an explicit clean replacement snapshot as the new permitted current version.
    ReplaceCurrent(ReplacementState),
    /// Clear the current head version without installing a replacement snapshot,
    /// recording a retracted audit tombstone and leaving `current_version` as `None`.
    ClearCurrent,
}

impl RetractionDisposition {
    fn validate(&self) -> Result<(), MemoryValidationError> {
        match self {
            Self::KeepCurrent | Self::ClearCurrent => Ok(()),
            Self::ReplaceCurrent(clean) => clean.validate(),
        }
    }
}

/// Validated state transition on a patient's memory.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum MemoryMutation {
    /// Patches topics in Coda's private operating index without touching unrelated topics, corrections, or summary.
    UpdateUnderstanding(OperatingIndexPatch),
    /// Records a dated physician correction (and optional topic patch) while preserving unrelated state.
    RecordCorrection(RecordCorrection),
    /// Marks an earlier active correction withdrawn while preserving audit history.
    WithdrawCorrection(WithdrawCorrection),
    /// Saves a new physician-facing summary with references and dates while preserving Coda's operating index and corrections.
    SaveSummary(PatientSummary),
    /// Retracts a version from permitted clinical readback and search.
    RetractVersion {
        /// Version number to mark retracted.
        target_version: MemoryVersionNumber,
        /// Clinical date of the retraction.
        retracted_on: ClinicalDate,
        /// Reason the version is withheld.
        reason: String,
        /// Explicit disposition for the patient's current head version.
        disposition: RetractionDisposition,
    },
}

impl MemoryMutation {
    /// Validates all domain fields of the mutation before storage execution.
    pub fn validate(&self) -> Result<(), MemoryValidationError> {
        match self {
            Self::UpdateUnderstanding(patch) => patch.validate(),
            Self::RecordCorrection(record) => record.validate(),
            Self::WithdrawCorrection(withdraw) => withdraw.validate(),
            Self::SaveSummary(summary) => summary.validate(),
            Self::RetractVersion {
                target_version: _,
                retracted_on,
                reason,
                disposition,
            } => {
                validate_clinical_date(retracted_on.as_str())?;
                if reason.trim().is_empty() {
                    return Err(MemoryValidationError::EmptyRetractionReason);
                }
                disposition.validate()
            }
        }
    }
}

/// Durable receipt returned when a memory mutation commits (or replays idempotently).
///
/// Structurally carries no [`OperatingIndex`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryWriteReceipt {
    /// Patient memory identity.
    pub memory_id: MemoryId,
    /// Practice tenant identifier.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Version number committed by this receipt.
    pub version: MemoryVersionNumber,
    /// Version that authored the summary carried on this snapshot, if any.
    pub summary_version: Option<MemoryVersionNumber>,
    /// Database-clock seconds when committed.
    pub saved_at: i64,
}

/// Outcome of a fenced patient-memory mutation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemoryMutationOutcome {
    /// Newly committed memory version.
    Applied(MemoryWriteReceipt),
    /// Exact retry of an already-committed command; no second version was written.
    Duplicate(MemoryWriteReceipt),
    /// Another command was already accepted under the same receipt key.
    IdempotencyConflict,
    /// Target correction or version could not be mutated as requested; nothing was written.
    InvalidTarget {
        /// Explanation of why the target state refused the mutation.
        reason: String,
    },
    /// Caller lacked authorization for the practice and patient workspace.
    Unauthorized {
        /// Refusal reason from the authorization owner.
        reason: String,
    },
}

/// Access standing of a caller relative to a patient workspace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatientAccess {
    /// Authorized clinical panel access for this practice and patient workspace.
    ClinicalPanel,
    /// Practice-wide clinic/demographic lookup only; never grants clinical memory access.
    ClinicDemographic,
    /// No access to this practice or patient workspace.
    Denied,
}

impl PatientAccess {
    fn permits_clinical_memory(self) -> Result<(), &'static str> {
        match self {
            Self::ClinicalPanel => Ok(()),
            Self::ClinicDemographic => {
                Err("clinic demographic access does not permit patient memory")
            }
            Self::Denied => Err("patient workspace access denied"),
        }
    }
}

/// Which version of a patient's memory to read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "selection", content = "version", rename_all = "snake_case")]
pub enum VersionSelection {
    /// Current permitted head snapshot.
    Current,
    /// Exact 1-based version number.
    Exact(MemoryVersionNumber),
}

/// Result of reading one patient memory snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemoryRead {
    /// Permitted snapshot exists and caller is authorized.
    Available(Box<MemorySnapshot>),
    /// Caller is authorized, and no permitted memory exists yet for this patient/selection.
    Absent,
    /// Requested version exists in the audit ledger, and was retracted from clinical readback.
    Withheld {
        /// Retracted version number.
        version: MemoryVersionNumber,
    },
    /// Caller is not authorized for this practice and patient workspace.
    Unauthorized {
        /// Authorization refusal reason.
        reason: String,
    },
}

/// Result of reading all permitted versions of one patient's memory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PermittedVersionsRead {
    /// Authorized list of permitted snapshots in ascending version order (`1..=N`).
    Permitted {
        /// Current permitted head version, if any.
        current_version: Option<MemoryVersionNumber>,
        /// Permitted snapshots, oldest first.
        versions: Vec<MemorySnapshot>,
    },
    /// Caller is not authorized for this practice and patient workspace.
    Unauthorized {
        /// Authorization refusal reason.
        reason: String,
    },
}

/// Result of reading the physician-facing summary for the patient page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SummaryRead {
    /// Authorized and a permitted summary is saved.
    Present(SavedPatientSummary),
    /// Authorized and no permitted summary is currently saved.
    Absent,
    /// Caller is not authorized for this practice and patient workspace.
    Unauthorized {
        /// Authorization refusal reason.
        reason: String,
    },
}

/// Audience allowed to view a searchable memory projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryAudience {
    /// Coda's private operating index; restricted to internal agent jobs.
    AgentOnly,
    /// Physician corrections and physician-facing summaries.
    PhysicianVisible,
}

/// Audience scope supplied by a search caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchAudienceScope {
    /// Internal Coda job search; may receive both `AgentOnly` and `PhysicianVisible` items.
    AgentJob,
    /// Physician-facing search; structurally excludes `AgentOnly` operating-index items.
    PhysicianFacing,
}

impl SearchAudienceScope {
    fn allows(self, audience: MemoryAudience) -> bool {
        match (self, audience) {
            (Self::AgentJob, MemoryAudience::AgentOnly | MemoryAudience::PhysicianVisible) => true,
            (Self::PhysicianFacing, MemoryAudience::PhysicianVisible) => true,
            (Self::PhysicianFacing, MemoryAudience::AgentOnly) => false,
        }
    }
}

/// Which versions patient search should inspect.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchVersionScope {
    /// Only the current permitted head snapshot of each authorized patient.
    CurrentOnly,
    /// Both the current permitted snapshot and permitted prior versions.
    CurrentAndPermittedPrior,
}

/// Standing of a memory version relative to the patient's current head version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case", deny_unknown_fields)]
pub enum VersionStanding {
    /// Latest permitted head version of the patient's memory.
    Current,
    /// Permitted earlier version superseded by `current_version`, or retained when
    /// the head version was retracted without replacement (`current_version: None`).
    Prior {
        /// Current permitted head version for the patient, if any.
        current_version: Option<MemoryVersionNumber>,
    },
}

/// Access level required by every patient-memory search record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredAccess {
    /// Full clinical panel access for the practice and patient workspace.
    ClinicalPanel,
}

/// Proof tag that a search record comes from a permitted (non-retracted) version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermittedVisibility {
    /// Version visibility verified as permitted.
    Permitted,
}

/// Clinical date carried on a searchable memory item.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "role",
    content = "date",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum MemorySearchDate {
    /// Date a physician correction was stated.
    CorrectionRecordedOn(ClinicalDate),
    /// Date a physician correction was withdrawn.
    CorrectionWithdrawnOn(ClinicalDate),
    /// Date a physician-facing summary was written.
    SummaryWrittenAt(ClinicalDate),
    /// Date of the latest review covered by a summary.
    SummaryReviewedAt(ClinicalDate),
    /// Date of the latest chart pull covered by a summary.
    SummaryPulledAt(ClinicalDate),
}

/// Exact access-control and visibility facts delivered to patient search.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryAccessFacts {
    /// Practice tenant owning this memory.
    pub practice: String,
    /// Patient workspace owning this memory.
    pub workspace: String,
    /// Clinical access required to read or surface this record.
    pub required_access: RequiredAccess,
    /// Verified non-retracted visibility.
    pub visibility: PermittedVisibility,
    /// Whether the record is agent-only or physician-visible.
    pub audience: MemoryAudience,
}

/// Typed searchable content projected from one permitted memory version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "entry_kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SearchableMemoryContent {
    /// Private operating-index topic (`MemoryAudience::AgentOnly`).
    OperatingTopic {
        /// Topic key.
        topic: TopicKey,
        /// Topic note.
        note: String,
    },
    /// Dated physician correction (`MemoryAudience::PhysicianVisible`).
    Correction {
        /// Correction identifier.
        correction_id: CorrectionId,
        /// Correction text.
        statement: String,
        /// Date stated.
        recorded_on: ClinicalDate,
        /// Source message provenance.
        source_message: SourceMessageRef,
        /// Lifecycle standing in this version.
        standing: CorrectionStanding,
    },
    /// Physician-facing summary (`MemoryAudience::PhysicianVisible`).
    Summary {
        /// Exact version that authored this summary.
        summary_version: MemoryVersionNumber,
        /// Summary content, references, and dates.
        summary: PatientSummary,
    },
}

/// One versioned, access-annotated patient memory item for search indexing and retrieval.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemorySearchRecord {
    /// Deterministic unique identifier for this versioned item.
    pub record_id: String,
    /// Patient memory identity.
    pub memory_id: MemoryId,
    /// Snapshot version number.
    pub version: MemoryVersionNumber,
    /// Whether this snapshot is current or a permitted prior version.
    pub standing: VersionStanding,
    /// Database-clock seconds when the snapshot was committed.
    pub saved_at: i64,
    /// Clinical dates attached to this item.
    pub clinical_dates: Vec<MemorySearchDate>,
    /// Author of this snapshot version.
    pub author: MemoryAuthor,
    /// Practice, workspace, required access, visibility, and audience facts.
    pub access: MemoryAccessFacts,
    /// Typed item payload.
    pub content: SearchableMemoryContent,
}

/// Scoped query parameters for reading [`MemorySearchRecord`] items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemorySearchQuery {
    /// Authenticated practice tenant.
    pub practice: String,
    /// Patient workspaces authorized at `ClinicalPanel` standing for the caller.
    pub allowed_workspaces: BTreeSet<String>,
    /// Whether to project current snapshots only or permitted prior versions too.
    pub versions: SearchVersionScope,
    /// Caller audience (`AgentJob` vs `PhysicianFacing`).
    pub audience: SearchAudienceScope,
}

/// Result of an authorized single-patient search projection read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SearchRead {
    /// Authorized search records for the patient workspace.
    Permitted(Vec<MemorySearchRecord>),
    /// Caller lacked authorization for the practice and patient workspace.
    Unauthorized {
        /// Authorization refusal reason.
        reason: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ToolWriteDisposition {
    Saved,
    Duplicate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MemoryToolWriteResult {
    outcome: ToolWriteDisposition,
    receipt: MemoryWriteReceipt,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadMemoryArgs {
    version: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TopicEntryWire {
    topic: String,
    note: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SaveMemoryArgs {
    upsert: Vec<TopicEntryWire>,
    remove: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RecordCorrectionArgs {
    correction_id: Option<String>,
    recorded_on: String,
    statement: String,
    source_message_id: Option<String>,
    source_chat_id: Option<String>,
    source_physician_id: Option<String>,
    supersedes_correction_id: Option<String>,
    upsert_topics: Vec<TopicEntryWire>,
    remove_topics: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SaveSummaryArgs {
    content: String,
    references: Vec<SummaryReference>,
    dates: SummaryDates,
}

impl SaveMemoryArgs {
    fn into_patch(self) -> Result<OperatingIndexPatch, MemoryValidationError> {
        patch_from_wire(self.upsert, self.remove)
    }
}

fn patch_from_wire(
    upsert_wire: Vec<TopicEntryWire>,
    remove_wire: Vec<String>,
) -> Result<OperatingIndexPatch, MemoryValidationError> {
    let mut upsert = BTreeMap::new();
    for entry in upsert_wire {
        let key = TopicKey::parse(entry.topic)?;
        if upsert.contains_key(&key) {
            return Err(MemoryValidationError::DuplicatePatchTopic {
                topic: key.as_str().to_owned(),
            });
        }
        upsert.insert(key, entry.note);
    }
    let mut remove = BTreeSet::new();
    for raw in remove_wire {
        remove.insert(TopicKey::parse(raw)?);
    }
    OperatingIndexPatch::new(upsert, remove)
}

fn optional_patch_from_wire(
    upsert_wire: Vec<TopicEntryWire>,
    remove_wire: Vec<String>,
) -> Result<Option<OperatingIndexPatch>, MemoryValidationError> {
    if upsert_wire.is_empty() && remove_wire.is_empty() {
        return Ok(None);
    }
    patch_from_wire(upsert_wire, remove_wire).map(Some)
}

/// Whether `name` is one of the patient-memory job tools.
pub fn is_tool(name: &str) -> bool {
    matches!(
        name,
        READ_MEMORY_TOOL | SAVE_MEMORY_TOOL | RECORD_CORRECTION_TOOL | SAVE_SUMMARY_TOOL
    )
}

/// Whether `name` is a read-only patient-memory tool safe to run without a barrier.
pub fn is_read_only(name: &str) -> bool {
    name == READ_MEMORY_TOOL
}

/// Whether `name` is a receipt-fenced patient-memory mutation tool.
pub fn is_mutation_tool(name: &str) -> bool {
    matches!(
        name,
        SAVE_MEMORY_TOOL | RECORD_CORRECTION_TOOL | SAVE_SUMMARY_TOOL
    )
}

/// Job tools for reading and updating durable patient memory and the patient summary.
pub fn tools() -> [Tool; 4] {
    let topic_entry = json!({
        "type": "object",
        "properties": {
            "topic": { "type": "string" },
            "note": { "type": "string" }
        },
        "required": ["topic", "note"],
        "additionalProperties": false
    });
    let summary_reference = json!({
        "anyOf": [
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "enum": ["chart_record"] },
                    "source_id": { "type": "string" },
                    "candidate_id": { "type": "string" },
                    "revision": { "type": "string" },
                    "label": { "type": "string" },
                    "record_date": { "type": ["string", "null"] }
                },
                "required": ["kind", "source_id", "candidate_id", "revision", "label", "record_date"],
                "additionalProperties": false
            },
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "enum": ["conversation_message"] },
                    "chat_id": { "type": "string" },
                    "message_id": { "type": "string" },
                    "label": { "type": "string" },
                    "stated_on": { "type": ["string", "null"] }
                },
                "required": ["kind", "chat_id", "message_id", "label", "stated_on"],
                "additionalProperties": false
            },
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "enum": ["literature"] },
                    "citation": { "type": "string" },
                    "identifier": { "type": ["string", "null"] }
                },
                "required": ["kind", "citation", "identifier"],
                "additionalProperties": false
            }
        ]
    });
    [
        Tool {
            name: READ_MEMORY_TOOL.into(),
            description: "Read durable patient memory for this patient workspace: current understanding, recorded physician corrections, saved summary, and permitted version headers. Pass version as null for current memory or a positive integer for a specific permitted version.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "version": { "type": ["integer", "null"], "minimum": 1 }
                },
                "required": ["version"],
                "additionalProperties": false
            }),
        },
        Tool {
            name: SAVE_MEMORY_TOOL.into(),
            description: "Patch Coda's private operating index for this patient workspace. Topics not named in upsert or remove stay unchanged, and physician corrections and the patient-page summary stay intact.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "upsert": { "type": "array", "items": topic_entry.clone() },
                    "remove": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["upsert", "remove"],
                "additionalProperties": false
            }),
        },
        Tool {
            name: RECORD_CORRECTION_TOOL.into(),
            description: "Record a dated physician correction for this patient workspace with source message provenance, optionally superseding an earlier correction and patching affected operating-index topics while keeping unrelated understanding intact. Use unambiguous YYYY-MM-DD dates.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "correction_id": { "type": ["string", "null"] },
                    "recorded_on": { "type": "string" },
                    "statement": { "type": "string" },
                    "source_message_id": { "type": ["string", "null"] },
                    "source_chat_id": { "type": ["string", "null"] },
                    "source_physician_id": { "type": ["string", "null"] },
                    "supersedes_correction_id": { "type": ["string", "null"] },
                    "upsert_topics": { "type": "array", "items": topic_entry },
                    "remove_topics": { "type": "array", "items": { "type": "string" } }
                },
                "required": [
                    "correction_id",
                    "recorded_on",
                    "statement",
                    "source_message_id",
                    "source_chat_id",
                    "source_physician_id",
                    "supersedes_correction_id",
                    "upsert_topics",
                    "remove_topics"
                ],
                "additionalProperties": false
            }),
        },
        Tool {
            name: SAVE_SUMMARY_TOOL.into(),
            description: "Save the physician-facing summary for the patient page with clinical prose, exact source references, and unambiguous YYYY-MM-DD dates. Never place private operating-index notes into the physician-facing summary.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "content": { "type": "string" },
                    "references": { "type": "array", "items": summary_reference },
                    "dates": {
                        "type": "object",
                        "properties": {
                            "written_at": { "type": "string" },
                            "reviewed_at": { "type": ["string", "null"] },
                            "pulled_at": { "type": ["string", "null"] }
                        },
                        "required": ["written_at", "reviewed_at", "pulled_at"],
                        "additionalProperties": false
                    }
                },
                "required": ["content", "references", "dates"],
                "additionalProperties": false
            }),
        },
    ]
}

fn chat_tool_receipt_key(scope: &Scope, work_id: &str, call_id: &str) -> String {
    serde_json::to_string(&(
        "memory_chat_tool",
        &scope.workspace,
        &scope.chat,
        work_id,
        call_id,
    ))
    .expect("tuple json")
}

fn command_envelope(patient: &PatientKey, mutation: &MemoryMutation) -> Value {
    json!({
        "patient": {
            "practice": patient.practice(),
            "workspace": patient.workspace(),
        },
        "memory_mutation": mutation,
    })
}

#[derive(Clone, Default)]
struct PriorSnapshot {
    operating_index: OperatingIndex,
    corrections: Vec<MemoryCorrection>,
    summary: Option<SnapshotSummary>,
}

struct PreparedSnapshot {
    operating_index: OperatingIndex,
    corrections: Vec<MemoryCorrection>,
    summary: Option<SnapshotSummary>,
    change: MemoryChange,
}

fn sorted_topics<'a>(topics: impl IntoIterator<Item = &'a TopicKey>) -> Vec<TopicKey> {
    let mut list: Vec<_> = topics.into_iter().cloned().collect();
    list.sort();
    list
}

fn next_ordinary_snapshot(
    prior: &PriorSnapshot,
    new_version: MemoryVersionNumber,
    default_correction_id: CorrectionId,
    mutation: &MemoryMutation,
) -> Result<PreparedSnapshot, String> {
    match mutation {
        MemoryMutation::UpdateUnderstanding(patch) => Ok(PreparedSnapshot {
            operating_index: prior.operating_index.apply_patch(patch),
            corrections: prior.corrections.clone(),
            summary: prior.summary.clone(),
            change: MemoryChange::UnderstandingUpdated {
                upserted_topics: sorted_topics(patch.upsert.keys()),
                removed_topics: sorted_topics(patch.remove.iter()),
            },
        }),
        MemoryMutation::RecordCorrection(record) => {
            let correction_id = record
                .correction_id
                .clone()
                .unwrap_or(default_correction_id);
            if prior
                .corrections
                .iter()
                .any(|existing| existing.correction_id == correction_id)
            {
                return Err(format!(
                    "correction `{}` already exists in patient memory",
                    correction_id.as_str()
                ));
            }
            let mut corrections = prior.corrections.clone();
            if let Some(target_id) = &record.supersedes {
                let Some(target) = corrections
                    .iter_mut()
                    .find(|existing| &existing.correction_id == target_id)
                else {
                    return Err(format!(
                        "cannot supersede unknown correction `{}`",
                        target_id.as_str()
                    ));
                };
                match &target.standing {
                    CorrectionStanding::Active => {
                        target.standing = CorrectionStanding::Superseded {
                            superseded_by: correction_id.clone(),
                        };
                    }
                    CorrectionStanding::Superseded { .. } => {
                        return Err(format!(
                            "correction `{}` is already superseded",
                            target_id.as_str()
                        ));
                    }
                    CorrectionStanding::Withdrawn { .. } => {
                        return Err(format!(
                            "correction `{}` is already withdrawn",
                            target_id.as_str()
                        ));
                    }
                }
            }
            corrections.push(MemoryCorrection {
                correction_id: correction_id.clone(),
                recorded_on: record.recorded_on.clone(),
                source_message: record.source_message.clone(),
                statement: record.statement.clone(),
                standing: CorrectionStanding::Active,
            });
            let (operating_index, upserted_topics, removed_topics) =
                match &record.understanding_patch {
                    Some(patch) => (
                        prior.operating_index.apply_patch(patch),
                        sorted_topics(patch.upsert.keys()),
                        sorted_topics(patch.remove.iter()),
                    ),
                    None => (prior.operating_index.clone(), Vec::new(), Vec::new()),
                };
            Ok(PreparedSnapshot {
                operating_index,
                corrections,
                summary: prior.summary.clone(),
                change: MemoryChange::CorrectionRecorded {
                    correction_id,
                    superseded: record.supersedes.clone(),
                    upserted_topics,
                    removed_topics,
                },
            })
        }
        MemoryMutation::WithdrawCorrection(withdraw) => {
            let mut corrections = prior.corrections.clone();
            let Some(target) = corrections
                .iter_mut()
                .find(|existing| existing.correction_id == withdraw.correction_id)
            else {
                return Err(format!(
                    "cannot withdraw unknown correction `{}`",
                    withdraw.correction_id.as_str()
                ));
            };
            match &target.standing {
                CorrectionStanding::Active => {
                    target.standing = CorrectionStanding::Withdrawn {
                        withdrawn_on: withdraw.withdrawn_on.clone(),
                        source_message: withdraw.source_message.clone(),
                        reason: withdraw.reason.clone(),
                    };
                }
                CorrectionStanding::Superseded { .. } => {
                    return Err(format!(
                        "correction `{}` is already superseded",
                        withdraw.correction_id.as_str()
                    ));
                }
                CorrectionStanding::Withdrawn { .. } => {
                    return Err(format!(
                        "correction `{}` is already withdrawn",
                        withdraw.correction_id.as_str()
                    ));
                }
            }
            let operating_index = match &withdraw.understanding_patch {
                Some(patch) => prior.operating_index.apply_patch(patch),
                None => prior.operating_index.clone(),
            };
            Ok(PreparedSnapshot {
                operating_index,
                corrections,
                summary: prior.summary.clone(),
                change: MemoryChange::CorrectionWithdrawn {
                    correction_id: withdraw.correction_id.clone(),
                },
            })
        }
        MemoryMutation::SaveSummary(summary) => Ok(PreparedSnapshot {
            operating_index: prior.operating_index.clone(),
            corrections: prior.corrections.clone(),
            summary: Some(SnapshotSummary {
                summary_version: new_version,
                summary: summary.clone(),
            }),
            change: MemoryChange::SummarySaved,
        }),
        MemoryMutation::RetractVersion { .. } => {
            Err("cannot retract a version when patient memory does not exist".into())
        }
    }
}

fn parse_visibility(row: &patient_memory_version::Model) -> Result<VersionVisibility, MemoryError> {
    match (row.visibility.as_str(), &row.retraction) {
        (VISIBILITY_PERMITTED, None) => Ok(VersionVisibility::Permitted),
        (VISIBILITY_RETRACTED, Some(value)) => {
            let retraction: VersionRetraction = serde_json::from_value(value.clone())
                .map_err(|err| corrupt_json("version.retraction", err))?;
            Ok(VersionVisibility::Retracted(retraction))
        }
        (VISIBILITY_PERMITTED, Some(_)) => Err(corrupt_invariant(
            "version.visibility",
            "permitted version has retraction payload",
        )),
        (VISIBILITY_RETRACTED, None) => Err(corrupt_invariant(
            "version.visibility",
            "retracted version lacks retraction payload",
        )),
        _ => Err(corrupt_invariant(
            "version.visibility",
            "unknown visibility tag",
        )),
    }
}

fn decode_snapshot(row: &patient_memory_version::Model) -> Result<MemorySnapshot, MemoryError> {
    let version = MemoryVersionNumber::from_db("version.version", row.version)?;
    let author: MemoryAuthor = serde_json::from_value(row.author.clone())
        .map_err(|e| corrupt_json("version.author", e))?;
    author.validate().map_err(|e| MemoryError::CorruptRecord {
        context: "version.author",
        source: CorruptRecordSource::Validation(e),
    })?;
    let change: MemoryChange = serde_json::from_value(row.change_kind.clone())
        .map_err(|e| corrupt_json("version.change_kind", e))?;
    let operating_index: OperatingIndex = serde_json::from_value(row.operating_index.clone())
        .map_err(|e| corrupt_json("version.operating_index", e))?;
    let corrections: Vec<MemoryCorrection> = serde_json::from_value(row.corrections.clone())
        .map_err(|e| corrupt_json("version.corrections", e))?;
    for correction in &corrections {
        correction
            .validate()
            .map_err(|e| MemoryError::CorruptRecord {
                context: "version.corrections",
                source: CorruptRecordSource::Validation(e),
            })?;
    }
    let summary = match (&row.summary, row.summary_version) {
        (None, None) => None,
        (Some(raw_summary), Some(raw_version)) => {
            let summary: PatientSummary = serde_json::from_value(raw_summary.clone())
                .map_err(|e| corrupt_json("version.summary", e))?;
            summary.validate().map_err(|e| MemoryError::CorruptRecord {
                context: "version.summary",
                source: CorruptRecordSource::Validation(e),
            })?;
            let sv = MemoryVersionNumber::from_db("version.summary_version", raw_version)?;
            if sv > version {
                return Err(corrupt_invariant(
                    "version.summary_version",
                    "summary_version exceeds snapshot version",
                ));
            }
            Some(SnapshotSummary {
                summary_version: sv,
                summary,
            })
        }
        (None, Some(_)) | (Some(_), None) => {
            return Err(corrupt_invariant(
                "version.summary",
                "summary and summary_version nullability mismatch",
            ));
        }
    };
    Ok(MemorySnapshot {
        memory_id: MemoryId::from_stored(row.memory_id.clone())?,
        practice: row.practice.clone(),
        workspace: row.workspace.clone(),
        version,
        author,
        change,
        saved_at: row.saved_at,
        operating_index,
        corrections,
        summary,
    })
}

fn decode_receipt(row: &patient_memory_version::Model) -> Result<MemoryWriteReceipt, MemoryError> {
    let version = MemoryVersionNumber::from_db("version.version", row.version)?;
    let summary_version = row
        .summary_version
        .map(|sv| MemoryVersionNumber::from_db("version.summary_version", sv))
        .transpose()?;
    Ok(MemoryWriteReceipt {
        memory_id: MemoryId::from_stored(row.memory_id.clone())?,
        practice: row.practice.clone(),
        workspace: row.workspace.clone(),
        version,
        summary_version,
        saved_at: row.saved_at,
    })
}

async fn find_receipt_version(
    db: &impl ConnectionTrait,
    practice: &str,
    receipt_physician: &str,
    receipt_key: &str,
) -> Result<Option<MemoryWriteReceipt>, MemoryError> {
    use patient_memory_version::Column as C;
    let row = patient_memory_version::Entity::find()
        .filter(C::Practice.eq(practice))
        .filter(C::ReceiptPhysician.eq(receipt_physician))
        .filter(C::ReceiptKey.eq(receipt_key))
        .one(db)
        .await?;
    row.as_ref().map(decode_receipt).transpose()
}

struct VersionRowSpec<'a> {
    patient: &'a PatientKey,
    memory_id: &'a MemoryId,
    version: MemoryVersionNumber,
    receipt_physician: &'a str,
    receipt_key: &'a str,
    author: &'a MemoryAuthor,
    visibility: &'a VersionVisibility,
    prepared: &'a PreparedSnapshot,
    saved_at: i64,
}

fn insert_version_model(
    spec: VersionRowSpec<'_>,
) -> Result<patient_memory_version::ActiveModel, MemoryError> {
    let (visibility_tag, retraction_json) = match spec.visibility {
        VersionVisibility::Permitted => (VISIBILITY_PERMITTED.to_owned(), None),
        VersionVisibility::Retracted(retraction) => (
            VISIBILITY_RETRACTED.to_owned(),
            Some(
                serde_json::to_value(retraction)
                    .map_err(|e| corrupt_json("encode.retraction", e))?,
            ),
        ),
    };
    let summary_json = spec
        .prepared
        .summary
        .as_ref()
        .map(|s| serde_json::to_value(&s.summary))
        .transpose()
        .map_err(|e| corrupt_json("encode.summary", e))?;
    let summary_version_i64 = spec
        .prepared
        .summary
        .as_ref()
        .map(|s| s.summary_version.to_db())
        .transpose()?;
    Ok(patient_memory_version::ActiveModel {
        practice: Set(spec.patient.practice().to_owned()),
        workspace: Set(spec.patient.workspace().to_owned()),
        version: Set(spec.version.to_db()?),
        memory_id: Set(spec.memory_id.as_str().to_owned()),
        receipt_physician: Set(spec.receipt_physician.to_owned()),
        receipt_key: Set(spec.receipt_key.to_owned()),
        author: Set(
            serde_json::to_value(spec.author).map_err(|e| corrupt_json("encode.author", e))?
        ),
        change_kind: Set(serde_json::to_value(&spec.prepared.change)
            .map_err(|e| corrupt_json("encode.change", e))?),
        visibility: Set(visibility_tag),
        retraction: Set(retraction_json),
        operating_index: Set(serde_json::to_value(&spec.prepared.operating_index)
            .map_err(|e| corrupt_json("encode.operating_index", e))?),
        corrections: Set(serde_json::to_value(&spec.prepared.corrections)
            .map_err(|e| corrupt_json("encode.corrections", e))?),
        summary: Set(summary_json),
        summary_version: Set(summary_version_i64),
        saved_at: Set(spec.saved_at),
    })
}

enum TxMutationResult {
    Commit(MemoryMutationOutcome),
    Rollback(MemoryMutationOutcome),
}

async fn apply_mutation_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    patient: &PatientKey,
    receipt_physician: &str,
    receipt_key: &str,
    author: MemoryAuthor,
    mutation: MemoryMutation,
) -> Result<TxMutationResult, MemoryError> {
    let command = command_envelope(patient, &mutation);
    match receipt::record(
        tx,
        patient.practice(),
        receipt_physician,
        receipt_key,
        &command,
    )
    .await?
    {
        receipt::Receipt::New => {}
        receipt::Receipt::Same => {
            let saved =
                find_receipt_version(tx, patient.practice(), receipt_physician, receipt_key)
                    .await?
                    .ok_or_else(|| {
                        corrupt_invariant(
                            "receipt_lookup",
                            "accepted memory receipt has no version row",
                        )
                    })?;
            return Ok(TxMutationResult::Commit(MemoryMutationOutcome::Duplicate(
                saved,
            )));
        }
        receipt::Receipt::Different => {
            return Ok(TxMutationResult::Commit(
                MemoryMutationOutcome::IdempotencyConflict,
            ));
        }
    }

    let head_id = (
        patient.practice().to_owned(),
        patient.workspace().to_owned(),
    );
    let mut existing_head = patient_memory::Entity::find_by_id(head_id.clone())
        .lock_exclusive()
        .one(tx)
        .await?;

    if existing_head.is_none() {
        let first_version = MemoryVersionNumber::FIRST;
        let default_corr =
            CorrectionId::from_receipt(patient.practice(), receipt_physician, receipt_key);
        let prepared = match next_ordinary_snapshot(
            &PriorSnapshot::default(),
            first_version,
            default_corr,
            &mutation,
        ) {
            Ok(prepared) => prepared,
            Err(reason) => {
                return Ok(TxMutationResult::Rollback(
                    MemoryMutationOutcome::InvalidTarget { reason },
                ));
            }
        };
        let saved_at = now(tx).await?;
        let memory_id = MemoryId::for_patient(patient);
        let summary_version = prepared.summary.as_ref().map(|s| s.summary_version);
        let summary_version_i64 = summary_version
            .map(MemoryVersionNumber::to_db)
            .transpose()?;
        let inserted = patient_memory::Entity::insert(patient_memory::ActiveModel {
            practice: Set(patient.practice().to_owned()),
            workspace: Set(patient.workspace().to_owned()),
            memory_id: Set(memory_id.as_str().to_owned()),
            latest_version: Set(1),
            current_version: Set(Some(1)),
            latest_summary_version: Set(summary_version_i64),
            created_at: Set(saved_at),
            updated_at: Set(saved_at),
        })
        .on_conflict(
            OnConflict::columns([
                patient_memory::Column::Practice,
                patient_memory::Column::Workspace,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec_without_returning(tx)
        .await?;

        if inserted == 1 {
            let version_model = insert_version_model(VersionRowSpec {
                patient,
                memory_id: &memory_id,
                version: first_version,
                receipt_physician,
                receipt_key,
                author: &author,
                visibility: &VersionVisibility::Permitted,
                prepared: &prepared,
                saved_at,
            })?;
            patient_memory_version::Entity::insert(version_model)
                .exec_without_returning(tx)
                .await?;
            return Ok(TxMutationResult::Commit(MemoryMutationOutcome::Applied(
                MemoryWriteReceipt {
                    memory_id,
                    practice: patient.practice().to_owned(),
                    workspace: patient.workspace().to_owned(),
                    version: first_version,
                    summary_version,
                    saved_at,
                },
            )));
        }

        existing_head = patient_memory::Entity::find_by_id(head_id)
            .lock_exclusive()
            .one(tx)
            .await?;
    }

    let head = existing_head.ok_or_else(|| {
        corrupt_invariant(
            "patient_memory",
            "head row disappeared under exclusive lock",
        )
    })?;
    let memory_id = MemoryId::from_stored(head.memory_id.clone())?;
    let latest_version =
        MemoryVersionNumber::from_db("patient_memory.latest_version", head.latest_version)?;
    let new_version = latest_version.next()?;
    let current_version = head
        .current_version
        .map(|v| MemoryVersionNumber::from_db("patient_memory.current_version", v))
        .transpose()?;
    let saved_at = now(tx).await?;

    let prior = match current_version {
        Some(cv) => {
            let row = patient_memory_version::Entity::find_by_id((
                patient.practice().to_owned(),
                patient.workspace().to_owned(),
                cv.to_db()?,
            ))
            .one(tx)
            .await?
            .ok_or_else(|| {
                corrupt_invariant("current_version", "current_version row is missing")
            })?;
            let decoded = decode_snapshot(&row)?;
            PriorSnapshot {
                operating_index: decoded.operating_index,
                corrections: decoded.corrections,
                summary: decoded.summary,
            }
        }
        None => PriorSnapshot::default(),
    };

    let (prepared, new_visibility, next_current_version) = match &mutation {
        MemoryMutation::UpdateUnderstanding(_)
        | MemoryMutation::RecordCorrection(_)
        | MemoryMutation::WithdrawCorrection(_)
        | MemoryMutation::SaveSummary(_) => {
            let default_corr =
                CorrectionId::from_receipt(patient.practice(), receipt_physician, receipt_key);
            let prepared =
                match next_ordinary_snapshot(&prior, new_version, default_corr, &mutation) {
                    Ok(prepared) => prepared,
                    Err(reason) => {
                        return Ok(TxMutationResult::Rollback(
                            MemoryMutationOutcome::InvalidTarget { reason },
                        ));
                    }
                };
            (prepared, VersionVisibility::Permitted, Some(new_version))
        }
        MemoryMutation::RetractVersion {
            target_version,
            retracted_on,
            reason,
            disposition,
        } => {
            let target_key = (
                patient.practice().to_owned(),
                patient.workspace().to_owned(),
                target_version.to_db()?,
            );
            let Some(target_row) = patient_memory_version::Entity::find_by_id(target_key)
                .lock_exclusive()
                .one(tx)
                .await?
            else {
                return Ok(TxMutationResult::Rollback(
                    MemoryMutationOutcome::InvalidTarget {
                        reason: format!("memory version {} does not exist", target_version.get()),
                    },
                ));
            };
            match parse_visibility(&target_row)? {
                VersionVisibility::Permitted => {}
                VersionVisibility::Retracted(_) => {
                    return Ok(TxMutationResult::Rollback(
                        MemoryMutationOutcome::InvalidTarget {
                            reason: format!(
                                "memory version {} is already retracted",
                                target_version.get()
                            ),
                        },
                    ));
                }
            }
            match disposition {
                RetractionDisposition::KeepCurrent => match current_version {
                    None => {
                        return Ok(TxMutationResult::Rollback(
                            MemoryMutationOutcome::InvalidTarget {
                                reason: "cannot keep current state when patient memory has no current version; choose ReplaceCurrent or ClearCurrent".into(),
                            },
                        ));
                    }
                    Some(cv) if cv == *target_version => {
                        return Ok(TxMutationResult::Rollback(
                            MemoryMutationOutcome::InvalidTarget {
                                reason: format!(
                                    "cannot keep current state when retracting current version {}; choose ReplaceCurrent or ClearCurrent",
                                    target_version.get()
                                ),
                            },
                        ));
                    }
                    Some(_) => {}
                },
                RetractionDisposition::ClearCurrent => {
                    if let Some(cv) = current_version
                        && cv != *target_version
                    {
                        return Ok(TxMutationResult::Rollback(
                            MemoryMutationOutcome::InvalidTarget {
                                reason: format!(
                                    "cannot clear current version {} while retracting prior version {}; choose KeepCurrent or ReplaceCurrent",
                                    cv.get(),
                                    target_version.get()
                                ),
                            },
                        ));
                    }
                }
                RetractionDisposition::ReplaceCurrent(_) => {}
            }

            let retraction = VersionRetraction {
                retracted_by_version: new_version,
                retracted_on: retracted_on.clone(),
                reason: reason.clone(),
                author: author.clone(),
            };
            let retraction_json = serde_json::to_value(&retraction)
                .map_err(|e| corrupt_json("encode.retraction", e))?;
            patient_memory_version::ActiveModel {
                practice: Unchanged(target_row.practice),
                workspace: Unchanged(target_row.workspace),
                version: Unchanged(target_row.version),
                memory_id: Unchanged(target_row.memory_id),
                receipt_physician: Unchanged(target_row.receipt_physician),
                receipt_key: Unchanged(target_row.receipt_key),
                author: Unchanged(target_row.author),
                change_kind: Unchanged(target_row.change_kind),
                visibility: Set(VISIBILITY_RETRACTED.to_owned()),
                retraction: Set(Some(retraction_json)),
                operating_index: Unchanged(target_row.operating_index),
                corrections: Unchanged(target_row.corrections),
                summary: Unchanged(target_row.summary),
                summary_version: Unchanged(target_row.summary_version),
                saved_at: Unchanged(target_row.saved_at),
            }
            .update(tx)
            .await?;

            match disposition {
                RetractionDisposition::KeepCurrent => {
                    let carried_summary = prior
                        .summary
                        .filter(|s| s.summary_version != *target_version);
                    (
                        PreparedSnapshot {
                            operating_index: prior.operating_index,
                            corrections: prior.corrections,
                            summary: carried_summary,
                            change: MemoryChange::VersionRetracted {
                                retracted_version: *target_version,
                                disposition: RetractionOutcome::KeptCurrent,
                            },
                        },
                        VersionVisibility::Permitted,
                        Some(new_version),
                    )
                }
                RetractionDisposition::ReplaceCurrent(clean) => {
                    let replacement_summary =
                        clean.summary.clone().map(|summary| SnapshotSummary {
                            summary_version: new_version,
                            summary,
                        });
                    (
                        PreparedSnapshot {
                            operating_index: clean.operating_index.clone(),
                            corrections: clean.corrections.clone(),
                            summary: replacement_summary,
                            change: MemoryChange::VersionRetracted {
                                retracted_version: *target_version,
                                disposition: RetractionOutcome::ReplacedCurrent,
                            },
                        },
                        VersionVisibility::Permitted,
                        Some(new_version),
                    )
                }
                RetractionDisposition::ClearCurrent => (
                    PreparedSnapshot {
                        operating_index: OperatingIndex::default(),
                        corrections: Vec::new(),
                        summary: None,
                        change: MemoryChange::VersionRetracted {
                            retracted_version: *target_version,
                            disposition: RetractionOutcome::ClearedCurrent,
                        },
                    },
                    VersionVisibility::Retracted(retraction),
                    None,
                ),
            }
        }
    };

    let version_model = insert_version_model(VersionRowSpec {
        patient,
        memory_id: &memory_id,
        version: new_version,
        receipt_physician,
        receipt_key,
        author: &author,
        visibility: &new_visibility,
        prepared: &prepared,
        saved_at,
    })?;
    patient_memory_version::Entity::insert(version_model)
        .exec_without_returning(tx)
        .await?;

    let summary_version = prepared.summary.as_ref().map(|s| s.summary_version);
    let next_current_i64 = next_current_version
        .map(MemoryVersionNumber::to_db)
        .transpose()?;
    let next_summary_i64 = summary_version
        .map(MemoryVersionNumber::to_db)
        .transpose()?;
    patient_memory::ActiveModel {
        practice: Unchanged(head.practice),
        workspace: Unchanged(head.workspace),
        memory_id: Unchanged(head.memory_id),
        latest_version: Set(new_version.to_db()?),
        current_version: Set(next_current_i64),
        latest_summary_version: Set(next_summary_i64),
        created_at: Unchanged(head.created_at),
        updated_at: Set(saved_at),
    }
    .update(tx)
    .await?;

    Ok(TxMutationResult::Commit(MemoryMutationOutcome::Applied(
        MemoryWriteReceipt {
            memory_id,
            practice: patient.practice().to_owned(),
            workspace: patient.workspace().to_owned(),
            version: new_version,
            summary_version,
            saved_at,
        },
    )))
}

async fn read_snapshot_in_tx(
    db: &impl ConnectionTrait,
    patient: &PatientKey,
    selection: VersionSelection,
) -> Result<MemoryRead, MemoryError> {
    let head_id = (
        patient.practice().to_owned(),
        patient.workspace().to_owned(),
    );
    let Some(head) = patient_memory::Entity::find_by_id(head_id).one(db).await? else {
        return Ok(MemoryRead::Absent);
    };
    let target_version = match selection {
        VersionSelection::Current => {
            let Some(current) = head.current_version else {
                return Ok(MemoryRead::Absent);
            };
            MemoryVersionNumber::from_db("patient_memory.current_version", current)?
        }
        VersionSelection::Exact(version) => version,
    };
    let Some(row) = patient_memory_version::Entity::find_by_id((
        patient.practice().to_owned(),
        patient.workspace().to_owned(),
        target_version.to_db()?,
    ))
    .one(db)
    .await?
    else {
        return Ok(MemoryRead::Absent);
    };
    match parse_visibility(&row)? {
        VersionVisibility::Permitted => Ok(MemoryRead::Available(Box::new(decode_snapshot(&row)?))),
        VersionVisibility::Retracted(_) => Ok(MemoryRead::Withheld {
            version: target_version,
        }),
    }
}

async fn read_permitted_versions_in_tx(
    db: &impl ConnectionTrait,
    patient: &PatientKey,
) -> Result<(Option<MemoryVersionNumber>, Vec<MemorySnapshot>), MemoryError> {
    use patient_memory_version::Column as C;
    let head_id = (
        patient.practice().to_owned(),
        patient.workspace().to_owned(),
    );
    let Some(head) = patient_memory::Entity::find_by_id(head_id).one(db).await? else {
        return Ok((None, Vec::new()));
    };
    let current_version = head
        .current_version
        .map(|v| MemoryVersionNumber::from_db("patient_memory.current_version", v))
        .transpose()?;
    let rows = patient_memory_version::Entity::find()
        .filter(C::Practice.eq(patient.practice()))
        .filter(C::Workspace.eq(patient.workspace()))
        .filter(C::Visibility.eq(VISIBILITY_PERMITTED))
        .order_by_asc(C::Version)
        .all(db)
        .await?;
    let mut versions = Vec::with_capacity(rows.len());
    for row in rows {
        versions.push(decode_snapshot(&row)?);
    }
    Ok((current_version, versions))
}

/// Reads ONLY summary columns (`memory_id`, `version`, `visibility`, `summary`,
/// `summary_version`, `saved_at`) — never selecting `operating_index`.
async fn read_summary_in_tx(
    db: &impl ConnectionTrait,
    patient: &PatientKey,
) -> Result<SummaryRead, MemoryError> {
    use patient_memory_version::Column as C;
    let head_id = (
        patient.practice().to_owned(),
        patient.workspace().to_owned(),
    );
    let Some(head) = patient_memory::Entity::find_by_id(head_id).one(db).await? else {
        return Ok(SummaryRead::Absent);
    };
    let Some(current_raw) = head.current_version else {
        return Ok(SummaryRead::Absent);
    };
    let current_version =
        MemoryVersionNumber::from_db("patient_memory.current_version", current_raw)?;
    let selected = patient_memory_version::Entity::find()
        .select_only()
        .columns([
            C::MemoryId,
            C::Version,
            C::Visibility,
            C::Summary,
            C::SummaryVersion,
            C::SavedAt,
        ])
        .filter(C::Practice.eq(patient.practice()))
        .filter(C::Workspace.eq(patient.workspace()))
        .filter(C::Version.eq(current_version.to_db()?))
        .into_tuple::<(String, i64, String, Option<Value>, Option<i64>, i64)>()
        .one(db)
        .await?;
    let Some((memory_id_raw, version_raw, visibility, summary_val, summary_ver_raw, saved_at)) =
        selected
    else {
        return Err(corrupt_invariant(
            "patient_summary",
            "current_version row missing during summary projection",
        ));
    };
    if visibility != VISIBILITY_PERMITTED {
        return Ok(SummaryRead::Absent);
    }
    let memory_version = MemoryVersionNumber::from_db("summary.memory_version", version_raw)?;
    match (summary_val, summary_ver_raw) {
        (None, None) => Ok(SummaryRead::Absent),
        (Some(raw_summary), Some(raw_sv)) => {
            let summary: PatientSummary =
                serde_json::from_value(raw_summary).map_err(|e| corrupt_json("summary.json", e))?;
            summary.validate().map_err(|e| MemoryError::CorruptRecord {
                context: "summary.validation",
                source: CorruptRecordSource::Validation(e),
            })?;
            let summary_version = MemoryVersionNumber::from_db("summary.summary_version", raw_sv)?;
            Ok(SummaryRead::Present(SavedPatientSummary {
                memory_id: MemoryId::from_stored(memory_id_raw)?,
                practice: patient.practice().to_owned(),
                workspace: patient.workspace().to_owned(),
                summary_version,
                memory_version,
                saved_at,
                summary,
            }))
        }
        (None, Some(_)) | (Some(_), None) => Err(corrupt_invariant(
            "summary.nullability",
            "summary and summary_version nullability mismatch",
        )),
    }
}

fn project_snapshot_search_records(
    snapshot: &MemorySnapshot,
    current_version: Option<MemoryVersionNumber>,
    audience_scope: SearchAudienceScope,
    out: &mut Vec<MemorySearchRecord>,
) {
    let standing = if current_version == Some(snapshot.version) {
        VersionStanding::Current
    } else {
        VersionStanding::Prior { current_version }
    };
    let version_num = snapshot.version.get();

    if audience_scope.allows(MemoryAudience::AgentOnly) {
        for (topic, note) in snapshot.operating_index.entries() {
            out.push(MemorySearchRecord {
                record_id: format!(
                    "{}:v{version_num}:topic:{}",
                    snapshot.memory_id.as_str(),
                    topic.as_str()
                ),
                memory_id: snapshot.memory_id.clone(),
                version: snapshot.version,
                standing,
                saved_at: snapshot.saved_at,
                clinical_dates: Vec::new(),
                author: snapshot.author.clone(),
                access: MemoryAccessFacts {
                    practice: snapshot.practice.clone(),
                    workspace: snapshot.workspace.clone(),
                    required_access: RequiredAccess::ClinicalPanel,
                    visibility: PermittedVisibility::Permitted,
                    audience: MemoryAudience::AgentOnly,
                },
                content: SearchableMemoryContent::OperatingTopic {
                    topic: topic.clone(),
                    note: note.clone(),
                },
            });
        }
    }

    if audience_scope.allows(MemoryAudience::PhysicianVisible) {
        for correction in &snapshot.corrections {
            let mut dates = vec![MemorySearchDate::CorrectionRecordedOn(
                correction.recorded_on.clone(),
            )];
            if let CorrectionStanding::Withdrawn { withdrawn_on, .. } = &correction.standing {
                dates.push(MemorySearchDate::CorrectionWithdrawnOn(
                    withdrawn_on.clone(),
                ));
            }
            out.push(MemorySearchRecord {
                record_id: format!(
                    "{}:v{version_num}:correction:{}",
                    snapshot.memory_id.as_str(),
                    correction.correction_id.as_str()
                ),
                memory_id: snapshot.memory_id.clone(),
                version: snapshot.version,
                standing,
                saved_at: snapshot.saved_at,
                clinical_dates: dates,
                author: snapshot.author.clone(),
                access: MemoryAccessFacts {
                    practice: snapshot.practice.clone(),
                    workspace: snapshot.workspace.clone(),
                    required_access: RequiredAccess::ClinicalPanel,
                    visibility: PermittedVisibility::Permitted,
                    audience: MemoryAudience::PhysicianVisible,
                },
                content: SearchableMemoryContent::Correction {
                    correction_id: correction.correction_id.clone(),
                    statement: correction.statement.clone(),
                    recorded_on: correction.recorded_on.clone(),
                    source_message: correction.source_message.clone(),
                    standing: correction.standing.clone(),
                },
            });
        }

        if let Some(snapshot_summary) = &snapshot.summary {
            let summary = &snapshot_summary.summary;
            let mut dates = vec![MemorySearchDate::SummaryWrittenAt(
                summary.dates.written_at.clone(),
            )];
            if let Some(reviewed) = &summary.dates.reviewed_at {
                dates.push(MemorySearchDate::SummaryReviewedAt(reviewed.clone()));
            }
            if let Some(pulled) = &summary.dates.pulled_at {
                dates.push(MemorySearchDate::SummaryPulledAt(pulled.clone()));
            }
            out.push(MemorySearchRecord {
                record_id: format!("{}:v{version_num}:summary", snapshot.memory_id.as_str()),
                memory_id: snapshot.memory_id.clone(),
                version: snapshot.version,
                standing,
                saved_at: snapshot.saved_at,
                clinical_dates: dates,
                author: snapshot.author.clone(),
                access: MemoryAccessFacts {
                    practice: snapshot.practice.clone(),
                    workspace: snapshot.workspace.clone(),
                    required_access: RequiredAccess::ClinicalPanel,
                    visibility: PermittedVisibility::Permitted,
                    audience: MemoryAudience::PhysicianVisible,
                },
                content: SearchableMemoryContent::Summary {
                    summary_version: snapshot_summary.summary_version,
                    summary: summary.clone(),
                },
            });
        }
    }
}

async fn read_search_records_in_tx(
    db: &impl ConnectionTrait,
    query: &MemorySearchQuery,
) -> Result<Vec<MemorySearchRecord>, MemoryError> {
    use patient_memory::Column as H;
    use patient_memory_version::Column as V;

    if query.practice.trim().is_empty() {
        return Err(MemoryValidationError::EmptyPractice.into());
    }
    let workspaces: Vec<String> = query
        .allowed_workspaces
        .iter()
        .filter(|ws| !ws.trim().is_empty())
        .cloned()
        .collect();
    if workspaces.is_empty() {
        return Ok(Vec::new());
    }

    let heads = patient_memory::Entity::find()
        .filter(H::Practice.eq(query.practice.as_str()))
        .filter(H::Workspace.is_in(workspaces.clone()))
        .order_by_asc(H::Workspace)
        .all(db)
        .await?;

    let mut current_by_workspace: BTreeMap<String, Option<MemoryVersionNumber>> = BTreeMap::new();
    for head in heads {
        let current = head
            .current_version
            .map(|current_raw| {
                MemoryVersionNumber::from_db("patient_memory.current_version", current_raw)
            })
            .transpose()?;
        if matches!(query.versions, SearchVersionScope::CurrentOnly) && current.is_none() {
            continue;
        }
        current_by_workspace.insert(head.workspace, current);
    }
    if current_by_workspace.is_empty() {
        return Ok(Vec::new());
    }

    let permitted_workspaces: Vec<String> = current_by_workspace.keys().cloned().collect();
    let rows = patient_memory_version::Entity::find()
        .filter(V::Practice.eq(query.practice.as_str()))
        .filter(V::Workspace.is_in(permitted_workspaces))
        .filter(V::Visibility.eq(VISIBILITY_PERMITTED))
        .order_by_asc(V::Workspace)
        .order_by_asc(V::Version)
        .all(db)
        .await?;

    let mut out = Vec::new();
    for row in rows {
        let Some(&current_version) = current_by_workspace.get(&row.workspace) else {
            continue;
        };
        let row_version = MemoryVersionNumber::from_db("version.version", row.version)?;
        match query.versions {
            SearchVersionScope::CurrentOnly if Some(row_version) != current_version => continue,
            SearchVersionScope::CurrentOnly | SearchVersionScope::CurrentAndPermittedPrior => {}
        }
        let snapshot = decode_snapshot(&row)?;
        project_snapshot_search_records(&snapshot, current_version, query.audience, &mut out);
    }
    Ok(out)
}

fn validate_direct_scope(scope: &Scope, session_hash: &[u8]) -> Result<PatientKey, MemoryError> {
    if session_hash.len() != 32 {
        return Err(MemoryValidationError::InvalidSessionReference.into());
    }
    if scope.physician.trim().is_empty() {
        return Err(MemoryValidationError::EmptyAuthorField("physician_id").into());
    }
    Ok(PatientKey::new(&scope.practice, &scope.workspace)?)
}

fn parse_tool_mutation(
    name: &str,
    arguments: Value,
    default_chat_source: Option<(&str, &str, Option<&str>)>,
) -> Result<MemoryMutation, String> {
    match name {
        SAVE_MEMORY_TOOL => {
            let args: SaveMemoryArgs = serde_json::from_value(arguments)
                .map_err(|err| format!("invalid {SAVE_MEMORY_TOOL} arguments: {err}"))?;
            let patch = args.into_patch().map_err(|err| err.to_string())?;
            Ok(MemoryMutation::UpdateUnderstanding(patch))
        }
        RECORD_CORRECTION_TOOL => {
            let args: RecordCorrectionArgs = serde_json::from_value(arguments)
                .map_err(|err| format!("invalid {RECORD_CORRECTION_TOOL} arguments: {err}"))?;
            let recorded_on =
                ClinicalDate::parse(args.recorded_on).map_err(|err| err.to_string())?;
            let correction_id = args
                .correction_id
                .map(CorrectionId::parse)
                .transpose()
                .map_err(|err| err.to_string())?;
            let supersedes = args
                .supersedes_correction_id
                .map(CorrectionId::parse)
                .transpose()
                .map_err(|err| err.to_string())?;
            let understanding_patch =
                optional_patch_from_wire(args.upsert_topics, args.remove_topics)
                    .map_err(|err| err.to_string())?;

            let (chat_id, message_id, physician_id) = match default_chat_source {
                Some((default_chat, default_physician, default_message)) => {
                    let chat_id = args
                        .source_chat_id
                        .unwrap_or_else(|| default_chat.to_owned());
                    let physician_id = args
                        .source_physician_id
                        .unwrap_or_else(|| default_physician.to_owned());
                    let message_id = match args.source_message_id {
                        Some(explicit) => explicit,
                        None => default_message
                            .map(str::to_owned)
                            .ok_or_else(|| "source_message_id is required".to_owned())?,
                    };
                    (chat_id, message_id, physician_id)
                }
                None => {
                    let chat_id = args
                        .source_chat_id
                        .ok_or_else(|| "source_chat_id is required in practice jobs".to_owned())?;
                    let message_id = args.source_message_id.ok_or_else(|| {
                        "source_message_id is required in practice jobs".to_owned()
                    })?;
                    let physician_id = args.source_physician_id.ok_or_else(|| {
                        "source_physician_id is required in practice jobs".to_owned()
                    })?;
                    (chat_id, message_id, physician_id)
                }
            };

            let source_message = SourceMessageRef::new(chat_id, message_id, physician_id)
                .map_err(|err| err.to_string())?;
            let record = RecordCorrection {
                correction_id,
                recorded_on,
                source_message,
                statement: args.statement,
                supersedes,
                understanding_patch,
            };
            record.validate().map_err(|err| err.to_string())?;
            Ok(MemoryMutation::RecordCorrection(record))
        }
        SAVE_SUMMARY_TOOL => {
            let args: SaveSummaryArgs = serde_json::from_value(arguments)
                .map_err(|err| format!("invalid {SAVE_SUMMARY_TOOL} arguments: {err}"))?;
            let summary = PatientSummary {
                content: args.content,
                references: args.references,
                dates: args.dates,
            };
            summary.validate().map_err(|err| err.to_string())?;
            Ok(MemoryMutation::SaveSummary(summary))
        }
        other => Err(format!("unknown patient memory mutation tool {other}")),
    }
}

fn memory_error_to_cause(error: MemoryError) -> engine::Cause {
    match error {
        MemoryError::Database(db_err) => super::persistence_failure(db_err),
        MemoryError::CorruptRecord { .. } | MemoryError::Validation(_) => error.into(),
    }
}

async fn execute_read_tool_in_db(
    store: &Store,
    patient: &PatientKey,
    arguments: Value,
) -> Result<ToolOutcome, engine::Cause> {
    let args: ReadMemoryArgs = match serde_json::from_value(arguments) {
        Ok(args) => args,
        Err(err) => {
            return Ok(ToolOutcome::refused(format!(
                "invalid {READ_MEMORY_TOOL} arguments: {err}"
            )));
        }
    };
    let selection = match args.version {
        None => VersionSelection::Current,
        Some(raw) => {
            let Some(version) = MemoryVersionNumber::new(raw) else {
                return Ok(ToolOutcome::refused("version must be >= 1 or null"));
            };
            VersionSelection::Exact(version)
        }
    };

    let tx = store.snapshot().await.map_err(super::persistence_failure)?;
    let snapshot_read = read_snapshot_in_tx(&tx, patient, selection)
        .await
        .map_err(memory_error_to_cause)?;
    let (_, permitted_snapshots) = read_permitted_versions_in_tx(&tx, patient)
        .await
        .map_err(memory_error_to_cause)?;
    tx.commit().await.map_err(super::persistence_failure)?;

    let headers: Vec<VersionHeader> = permitted_snapshots
        .iter()
        .map(VersionHeader::from)
        .collect();
    let payload = match snapshot_read {
        MemoryRead::Available(snapshot) => json!({
            "outcome": "available",
            "snapshot": snapshot,
            "permitted_versions": headers,
        }),
        MemoryRead::Absent => json!({
            "outcome": "absent",
            "permitted_versions": headers,
        }),
        MemoryRead::Withheld { version } => json!({
            "outcome": "withheld",
            "version": version,
            "permitted_versions": headers,
        }),
        MemoryRead::Unauthorized { reason } => return Ok(ToolOutcome::refused(reason)),
    };
    Ok(ToolOutcome::Returned(payload.to_string()))
}

fn outcome_to_tool(outcome: MemoryMutationOutcome) -> Result<ToolOutcome, engine::Cause> {
    match outcome {
        MemoryMutationOutcome::Applied(receipt) => {
            let body = serde_json::to_string(&MemoryToolWriteResult {
                outcome: ToolWriteDisposition::Saved,
                receipt,
            })?;
            Ok(ToolOutcome::Returned(body))
        }
        MemoryMutationOutcome::Duplicate(receipt) => {
            let body = serde_json::to_string(&MemoryToolWriteResult {
                outcome: ToolWriteDisposition::Duplicate,
                receipt,
            })?;
            Ok(ToolOutcome::Returned(body))
        }
        MemoryMutationOutcome::IdempotencyConflict => Ok(ToolOutcome::refused(
            "a different patient memory mutation already holds this receipt key",
        )),
        MemoryMutationOutcome::InvalidTarget { reason } => Ok(ToolOutcome::refused(reason)),
        MemoryMutationOutcome::Unauthorized { reason } => Ok(ToolOutcome::refused(reason)),
    }
}

impl Store {
    /// Applies a direct physician-authorized mutation to a patient's memory,
    /// holding the authorization owner's publication locks and recording a
    /// once-only command receipt in the same Postgres transaction.
    pub async fn mutate_patient_memory_authorized(
        &self,
        scope: &Scope,
        key: &str,
        session_hash: &[u8],
        mutation: MemoryMutation,
        authority: &mut dyn Authority,
    ) -> Result<MemoryMutationOutcome, MemoryError> {
        let patient = validate_direct_scope(scope, session_hash)?;
        if key.trim().is_empty() {
            return Err(MemoryValidationError::EmptyKey.into());
        }
        mutation.validate()?;
        let author = MemoryAuthor::PhysicianDirect {
            physician_id: scope.physician.clone(),
        };
        author.validate()?;

        let tx = self.db.begin().await?;
        if let Err(reason) = authority
            .lock_publication(&tx, scope, Some(session_hash))
            .await
        {
            tx.rollback().await?;
            return Ok(MemoryMutationOutcome::Unauthorized { reason });
        }

        match apply_mutation_in_tx(&tx, &patient, &scope.physician, key, author, mutation).await? {
            TxMutationResult::Commit(outcome) => {
                tx.commit().await?;
                Ok(outcome)
            }
            TxMutationResult::Rollback(outcome) => {
                tx.rollback().await?;
                Ok(outcome)
            }
        }
    }

    /// Applies a practice-owned job mutation to a patient's memory, consuming a
    /// once-only command receipt in the same Postgres transaction.
    pub async fn mutate_patient_memory_in_job(
        &self,
        patient: &PatientKey,
        caller: &JobMemoryCaller,
        key: &str,
        mutation: MemoryMutation,
    ) -> Result<MemoryMutationOutcome, MemoryError> {
        if key.trim().is_empty() {
            return Err(MemoryValidationError::EmptyKey.into());
        }
        mutation.validate()?;
        let author = MemoryAuthor::PracticeJob {
            job_kind: caller.job_kind().to_owned(),
            job_id: caller.job_id().to_owned(),
        };
        author.validate()?;
        let receipt_actor = caller.receipt_actor();
        let receipt_key = caller.receipt_key(key);

        let tx = self.db.begin().await?;
        match apply_mutation_in_tx(&tx, patient, &receipt_actor, &receipt_key, author, mutation)
            .await?
        {
            TxMutationResult::Commit(outcome) => {
                tx.commit().await?;
                Ok(outcome)
            }
            TxMutationResult::Rollback(outcome) => {
                tx.rollback().await?;
                Ok(outcome)
            }
        }
    }

    /// Reads current or exact patient memory under a revalidated physician session.
    pub async fn patient_memory_authorized(
        &self,
        scope: &Scope,
        session_hash: &[u8],
        selection: VersionSelection,
        authority: &mut dyn Authority,
    ) -> Result<MemoryRead, MemoryError> {
        let patient = validate_direct_scope(scope, session_hash)?;
        let tx = self.snapshot().await?;
        if let Err(reason) = authority.check(&tx, scope, Some(session_hash)).await {
            tx.rollback().await?;
            return Ok(MemoryRead::Unauthorized { reason });
        }
        let result = read_snapshot_in_tx(&tx, &patient, selection).await?;
        tx.commit().await?;
        Ok(result)
    }

    /// Reads current or exact patient memory gated by explicit [`PatientAccess`].
    pub async fn patient_memory_for_access(
        &self,
        patient: &PatientKey,
        access: PatientAccess,
        selection: VersionSelection,
    ) -> Result<MemoryRead, MemoryError> {
        if let Err(reason) = access.permits_clinical_memory() {
            return Ok(MemoryRead::Unauthorized {
                reason: reason.to_owned(),
            });
        }
        let tx = self.snapshot().await?;
        let result = read_snapshot_in_tx(&tx, patient, selection).await?;
        tx.commit().await?;
        Ok(result)
    }

    /// Reads all permitted versions of one patient's memory under a revalidated physician session.
    pub async fn permitted_memory_versions_authorized(
        &self,
        scope: &Scope,
        session_hash: &[u8],
        authority: &mut dyn Authority,
    ) -> Result<PermittedVersionsRead, MemoryError> {
        let patient = validate_direct_scope(scope, session_hash)?;
        let tx = self.snapshot().await?;
        if let Err(reason) = authority.check(&tx, scope, Some(session_hash)).await {
            tx.rollback().await?;
            return Ok(PermittedVersionsRead::Unauthorized { reason });
        }
        let (current_version, versions) = read_permitted_versions_in_tx(&tx, &patient).await?;
        tx.commit().await?;
        Ok(PermittedVersionsRead::Permitted {
            current_version,
            versions,
        })
    }

    /// Reads all permitted versions of one patient's memory gated by explicit [`PatientAccess`].
    pub async fn permitted_memory_versions(
        &self,
        patient: &PatientKey,
        access: PatientAccess,
    ) -> Result<PermittedVersionsRead, MemoryError> {
        if let Err(reason) = access.permits_clinical_memory() {
            return Ok(PermittedVersionsRead::Unauthorized {
                reason: reason.to_owned(),
            });
        }
        let tx = self.snapshot().await?;
        let (current_version, versions) = read_permitted_versions_in_tx(&tx, patient).await?;
        tx.commit().await?;
        Ok(PermittedVersionsRead::Permitted {
            current_version,
            versions,
        })
    }

    /// Reads the physician-facing summary for the patient page under a
    /// revalidated physician session, without selecting `operating_index`.
    pub async fn patient_summary_authorized(
        &self,
        scope: &Scope,
        session_hash: &[u8],
        authority: &mut dyn Authority,
    ) -> Result<SummaryRead, MemoryError> {
        let patient = validate_direct_scope(scope, session_hash)?;
        let tx = self.snapshot().await?;
        if let Err(reason) = authority.check(&tx, scope, Some(session_hash)).await {
            tx.rollback().await?;
            return Ok(SummaryRead::Unauthorized { reason });
        }
        let result = read_summary_in_tx(&tx, &patient).await?;
        tx.commit().await?;
        Ok(result)
    }

    /// Reads the physician-facing summary for the patient page gated by
    /// [`PatientAccess`], without selecting `operating_index`.
    pub async fn patient_summary_for_access(
        &self,
        patient: &PatientKey,
        access: PatientAccess,
    ) -> Result<SummaryRead, MemoryError> {
        if let Err(reason) = access.permits_clinical_memory() {
            return Ok(SummaryRead::Unauthorized {
                reason: reason.to_owned(),
            });
        }
        let tx = self.snapshot().await?;
        let result = read_summary_in_tx(&tx, patient).await?;
        tx.commit().await?;
        Ok(result)
    }

    /// Projects versioned, access-annotated [`MemorySearchRecord`] items for
    /// authorized patient workspaces in one practice.
    pub async fn memory_search_records(
        &self,
        query: &MemorySearchQuery,
    ) -> Result<Vec<MemorySearchRecord>, MemoryError> {
        let tx = self.snapshot().await?;
        let records = read_search_records_in_tx(&tx, query).await?;
        tx.commit().await?;
        Ok(records)
    }

    /// Projects versioned, access-annotated [`MemorySearchRecord`] items for
    /// one patient workspace under a revalidated physician session.
    pub async fn patient_memory_search_records_authorized(
        &self,
        scope: &Scope,
        session_hash: &[u8],
        versions: SearchVersionScope,
        audience: SearchAudienceScope,
        authority: &mut dyn Authority,
    ) -> Result<SearchRead, MemoryError> {
        let patient = validate_direct_scope(scope, session_hash)?;
        let tx = self.snapshot().await?;
        if let Err(reason) = authority.check(&tx, scope, Some(session_hash)).await {
            tx.rollback().await?;
            return Ok(SearchRead::Unauthorized { reason });
        }
        let mut allowed_workspaces = BTreeSet::new();
        allowed_workspaces.insert(patient.workspace().to_owned());
        let query = MemorySearchQuery {
            practice: patient.practice().to_owned(),
            allowed_workspaces,
            versions,
            audience,
        };
        let records = read_search_records_in_tx(&tx, &query).await?;
        tx.commit().await?;
        Ok(SearchRead::Permitted(records))
    }

    /// Executes a patient-memory tool inside a fenced chat attempt.
    pub async fn execute_chat_memory_tool(
        &self,
        claim: &Claim,
        call: &Invocation,
        arguments: Value,
    ) -> Result<ToolOutcome, engine::Cause> {
        let patient = PatientKey::new(&claim.scope.practice, &claim.scope.workspace)?;
        if call.name == READ_MEMORY_TOOL {
            self.assert_current(claim).await?;
            return execute_read_tool_in_db(self, &patient, arguments).await;
        }

        let tx = self.db.begin().await.map_err(super::persistence_failure)?;
        Self::lock_claim(&tx, claim).await?.require()?;
        let state = Self::read_conversation(&tx, &claim.scope).await?;
        let work_id = state.work.clone();
        if work_id.trim().is_empty() {
            return Err(super::error("missing accepted chat work identity").into());
        }
        let default_message = state.messages.iter().rev().find_map(|m| match &m.role {
            MessageRole::User { id, work, .. } if work == &work_id => Some(id.as_str()),
            MessageRole::User { .. } | MessageRole::Assistant { .. } => None,
        });
        let mutation = match parse_tool_mutation(
            &call.name,
            arguments,
            Some((
                &claim.scope.chat,
                &claim.scope.physician,
                default_message.or_else(|| state.input_id()),
            )),
        ) {
            Ok(mutation) => mutation,
            Err(reason) => {
                tx.rollback().await.map_err(super::persistence_failure)?;
                return Ok(ToolOutcome::refused(reason));
            }
        };

        let receipt_key = chat_tool_receipt_key(&claim.scope, &work_id, &call.id);
        let author = MemoryAuthor::ChatJob {
            physician_id: claim.scope.physician.clone(),
            chat_id: claim.scope.chat.clone(),
            work_id,
            call_id: call.id.clone(),
        };
        let tx_result = apply_mutation_in_tx(
            &tx,
            &patient,
            &claim.scope.physician,
            &receipt_key,
            author,
            mutation,
        )
        .await
        .map_err(memory_error_to_cause)?;

        let outcome = match tx_result {
            TxMutationResult::Commit(outcome) => {
                tx.commit().await.map_err(super::persistence_failure)?;
                outcome
            }
            TxMutationResult::Rollback(outcome) => {
                tx.rollback().await.map_err(super::persistence_failure)?;
                outcome
            }
        };
        outcome_to_tool(outcome)
    }

    /// Reconciles an unacknowledged chat memory mutation tool call across crash recovery.
    /// Never executes the mutation if no committed receipt exists.
    pub async fn saved_chat_memory_outcome(
        &self,
        claim: &Claim,
        call: &Invocation,
    ) -> Result<Option<ToolOutcome>, engine::Cause> {
        if !is_mutation_tool(&call.name) {
            return Ok(None);
        }
        self.assert_current(claim).await?;
        let tx = self.snapshot().await.map_err(super::persistence_failure)?;
        let state = Self::read_conversation(&tx, &claim.scope)
            .await
            .map_err(super::persistence_failure)?;
        let receipt_key = chat_tool_receipt_key(&claim.scope, &state.work, &call.id);
        let saved = find_receipt_version(
            &tx,
            &claim.scope.practice,
            &claim.scope.physician,
            &receipt_key,
        )
        .await
        .map_err(memory_error_to_cause)?;
        tx.commit().await.map_err(super::persistence_failure)?;
        match saved {
            Some(receipt) => {
                let body = serde_json::to_string(&MemoryToolWriteResult {
                    outcome: ToolWriteDisposition::Saved,
                    receipt,
                })?;
                Ok(Some(ToolOutcome::Returned(body)))
            }
            None => Ok(None),
        }
    }

    /// Executes a patient-memory tool inside a practice-owned background job.
    pub async fn execute_job_memory_tool(
        &self,
        patient: &PatientKey,
        caller: &JobMemoryCaller,
        call: &Invocation,
        arguments: Value,
    ) -> Result<ToolOutcome, engine::Cause> {
        if call.id.trim().is_empty() {
            return Ok(ToolOutcome::refused("tool call id must not be empty"));
        }
        if call.name == READ_MEMORY_TOOL {
            return execute_read_tool_in_db(self, patient, arguments).await;
        }
        let mutation = match parse_tool_mutation(&call.name, arguments, None) {
            Ok(mutation) => mutation,
            Err(reason) => return Ok(ToolOutcome::refused(reason)),
        };
        let outcome = self
            .mutate_patient_memory_in_job(patient, caller, &call.id, mutation)
            .await
            .map_err(memory_error_to_cause)?;
        outcome_to_tool(outcome)
    }

    /// Reconciles an unacknowledged practice-job memory mutation tool call across crash recovery.
    pub async fn saved_job_memory_outcome(
        &self,
        patient: &PatientKey,
        caller: &JobMemoryCaller,
        call: &Invocation,
    ) -> Result<Option<ToolOutcome>, engine::Cause> {
        if !is_mutation_tool(&call.name) || call.id.trim().is_empty() {
            return Ok(None);
        }
        let receipt_actor = caller.receipt_actor();
        let receipt_key = caller.receipt_key(&call.id);
        let tx = self.snapshot().await.map_err(super::persistence_failure)?;
        let saved = find_receipt_version(&tx, patient.practice(), &receipt_actor, &receipt_key)
            .await
            .map_err(memory_error_to_cause)?;
        tx.commit().await.map_err(super::persistence_failure)?;
        match saved {
            Some(receipt) => {
                let body = serde_json::to_string(&MemoryToolWriteResult {
                    outcome: ToolWriteDisposition::Saved,
                    receipt,
                })?;
                Ok(Some(ToolOutcome::Returned(body)))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn clinical_date_accepts_full_dates_and_rejects_ambiguous_or_impossible_dates() {
        for valid in [
            ("2026-11-12", "2026-11-12"),
            ("2024-02-29T08:15:00Z", "2024-02-29"),
            ("2026-11-12T14:30:05.123456-07:00", "2026-11-12"),
            ("2026-11-12T23:59:60+05:30", "2026-11-12"),
        ] {
            assert_eq!(
                ClinicalDate::parse(valid.0).unwrap().calendar_date(),
                valid.1
            );
        }
        for invalid in [
            "",
            "2026-11",
            "11-12",
            "Nov 12",
            "2026/11/12",
            "2025-02-29",
            "2026-02-30",
            "2026-13-01",
            "2026-00-10",
            "2026-+1-05",
            "2026-11-12extra",
            "2026-11-12Tnot-a-timestamp",
            "2026-11-12T24:00:00Z",
            "2026-11-12T12:60:00Z",
            "2026-11-12T12:00:61Z",
            "2026-11-12T12:00:00",
            "2026-11-12T12:00:00.Z",
            "2026-11-12T12:00:00.1234567890Z",
            "2026-11-12T12:00:00Zextra",
            "2026-11-12T12:00:00+24:00",
            "2026-11-12T12:00:00-07:60",
            " 2026-11-12",
        ] {
            assert!(
                ClinicalDate::parse(invalid).is_err(),
                "expected `{invalid}` to fail ClinicalDate::parse"
            );
        }
    }

    #[test]
    fn retraction_mutation_round_trips_all_dispositions() {
        let mut clean_entries = BTreeMap::new();
        clean_entries.insert(
            TopicKey::parse("renal").unwrap(),
            "Baseline Cr 1.2 mg/dL".into(),
        );
        for disposition in [
            RetractionDisposition::KeepCurrent,
            RetractionDisposition::ClearCurrent,
            RetractionDisposition::ReplaceCurrent(ReplacementState {
                operating_index: OperatingIndex::from_entries(clean_entries),
                corrections: Vec::new(),
                summary: None,
            }),
        ] {
            let mutation = MemoryMutation::RetractVersion {
                target_version: MemoryVersionNumber::FIRST,
                retracted_on: ClinicalDate::parse("2026-11-12").unwrap(),
                reason: "Entered on wrong workspace".into(),
                disposition,
            };
            mutation.validate().unwrap();
            let encoded = serde_json::to_value(&mutation).unwrap();
            let decoded: MemoryMutation = serde_json::from_value(encoded).unwrap();
            assert_eq!(decoded, mutation);
        }
    }

    #[test]
    fn operating_index_patch_rejects_empty_and_overlapping_topics() {
        assert_eq!(
            OperatingIndexPatch::new(BTreeMap::new(), BTreeSet::new()),
            Err(MemoryValidationError::EmptyUnderstandingPatch)
        );
        let topic = TopicKey::parse("medications").unwrap();
        let mut upsert = BTreeMap::new();
        upsert.insert(topic.clone(), "lisinopril 10 mg daily".into());
        let mut remove = BTreeSet::new();
        remove.insert(topic);
        assert_eq!(
            OperatingIndexPatch::new(upsert, remove),
            Err(MemoryValidationError::OverlappingPatchTopic {
                topic: "medications".into()
            })
        );
    }

    #[test]
    fn physician_facing_types_reject_operating_index_fields_at_deserialization() {
        let with_private = json!({
            "content": "Stable blood pressure on lisinopril.",
            "references": [],
            "dates": {
                "written_at": "2026-11-12",
                "reviewed_at": null,
                "pulled_at": null
            },
            "operating_index": {
                "entries": {
                    "uncertainties": "verify home cuff calibration"
                }
            }
        });
        assert!(serde_json::from_value::<PatientSummary>(with_private).is_err());

        let saved_with_private = json!({
            "memory_id": "pmem_123",
            "practice": "prac-1",
            "workspace": "pat-1",
            "summary_version": 1,
            "memory_version": 1,
            "saved_at": 1700000000,
            "summary": {
                "content": "Stable blood pressure on lisinopril.",
                "references": [],
                "dates": {
                    "written_at": "2026-11-12",
                    "reviewed_at": null,
                    "pulled_at": null
                }
            },
            "operating_index": {}
        });
        assert!(serde_json::from_value::<SavedPatientSummary>(saved_with_private).is_err());
    }
}
