//! On-demand patient source refresh tool, saved per-source status, and atomic
//! once-only refresh receipts.
//!
//! During a background job or physician chat run, Coda can request fresh
//! records for a patient workspace's clinical sources (`refresh_patient_sources`)
//! and learn what happened for each source:
//! - [`SourceRefreshOutcome::Updated`] — fresh revisions were saved to the
//!   versioned source store and `physician_runtime_source_pulls` advanced.
//! - [`SourceRefreshOutcome::PartlyUpdated`] — readable revisions were saved to
//!   the versioned source store without withdrawing unmentioned records, while
//!   recording why part of the source could not be fetched.
//! - [`SourceRefreshOutcome::Failed`] — the failure reason and check timestamp
//!   were recorded in `physician_runtime_source_statuses` while leaving the
//!   source's last good records and prior `pulled_at` untouched.
//!
//! Each source settles atomically in one Postgres transaction alongside its
//! once-only receipt row in `physician_runtime_source_refresh_receipts`, and the
//! completed request records a command receipt in `physician_runtime_commands`.
//! Retrying after an interrupted source check rechecks patient access first and
//! saves each source result at most once.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use harness::engine::{self, Invocation, Tool, ToolOutcome};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::entities::{command, source_refresh_receipt, source_status};
use super::jobs::{PRACTICE_OWNER, PracticeAccessError, PracticeAuthority, PracticeScope};
use super::receipt::{self, Receipt};
use super::sources::{
    DeliveryScope, SourcePull, current_version_in_tx, last_source_pull_in_tx, record_delivery_in_tx,
};
use super::{Authority, Claim, Scope, Store, conversion_error, error, now};
use crate::physician_sources::{SourcePatient, SourceRevision};

/// Tool name offered to runs for fetching fresh patient source records.
pub const REFRESH_TOOL: &str = "refresh_patient_sources";

/// Maximum distinct source identifiers accepted in one selected refresh request.
pub const MAX_SELECTED_SOURCES: usize = 16;

/// Maximum character length for a source label or failure/partial reason.
pub const MAX_REASON_CHARS: usize = 1000;

/// Reserved receipt actor namespace for physician-scoped refresh commands so
/// refresh command keys never collide with chat message command keys.
const PHYSICIAN_REFRESH_ACTOR_PREFIX: &str = "coda:refresh:";

/// Reserved receipt actor for practice-owned background job refresh commands.
const PRACTICE_REFRESH_ACTOR: &str = PRACTICE_OWNER;

/// Whether `name` is the patient source refresh tool.
pub fn is_refresh_tool(name: &str) -> bool {
    name == REFRESH_TOOL
}

fn valid_token(value: &str) -> bool {
    let count = value.chars().count();
    (1..=256).contains(&count) && value.trim() == value && !value.chars().any(char::is_control)
}

fn validate_text_field(
    field: &'static str,
    value: &str,
    max_chars: usize,
) -> Result<String, RefreshInputError> {
    let trimmed = value.trim();
    let count = trimmed.chars().count();
    if !(1..=max_chars).contains(&count) || trimmed.chars().any(char::is_control) {
        return Err(RefreshInputError::InvalidField(field));
    }
    Ok(trimmed.to_owned())
}

/// Boundary validation failure on a refresh identifier, target, or tool call.
#[derive(Debug, thiserror::Error)]
pub enum RefreshInputError {
    /// A required scope, work, label, or reason field was blank or invalid.
    #[error("{0} must be non-empty without control characters")]
    InvalidField(&'static str),
    /// A source identifier or command key failed validation.
    #[error("invalid identifier `{0}`: supply 1 to 256 trimmed visible characters")]
    InvalidIdentifier(String),
    /// Selected `source_ids` was empty or exceeded [`MAX_SELECTED_SOURCES`].
    #[error(
        "supply between 1 and {MAX_SELECTED_SOURCES} source_ids, or omit source_ids to refresh all configured sources"
    )]
    EmptySourceSelection,
    /// Tool JSON arguments did not match the offered schema.
    #[error("supply optional non-empty `source_ids` and optional `request_key` ({0})")]
    MalformedArguments(#[source] serde_json::Error),
    /// Tool invocation named an unknown tool.
    #[error("unknown source refresh tool `{0}`")]
    UnknownTool(String),
}

/// Stable identifier of one clinical source adapter (such as `"ehr"` or `"labs"`).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SourceId(String);

impl SourceId {
    /// Parses a trimmed, non-empty source identifier of 1 to 256 characters.
    pub fn new(value: impl Into<String>) -> Result<Self, RefreshInputError> {
        let value = value.into();
        if !valid_token(&value) {
            return Err(RefreshInputError::InvalidIdentifier(value));
        }
        Ok(Self(value))
    }

    /// Identifier text as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SourceId {
    type Error = RefreshInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<SourceId> for String {
    fn from(id: SourceId) -> Self {
        id.0
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Once-only idempotency key for one patient source refresh request.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct RefreshCommandKey(String);

impl RefreshCommandKey {
    /// Parses a trimmed, non-empty command key of 1 to 256 characters.
    pub fn new(value: impl Into<String>) -> Result<Self, RefreshInputError> {
        let value = value.into();
        if !valid_token(&value) {
            return Err(RefreshInputError::InvalidIdentifier(value));
        }
        Ok(Self(value))
    }

    /// Command key text as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for RefreshCommandKey {
    type Error = RefreshInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<RefreshCommandKey> for String {
    fn from(key: RefreshCommandKey) -> Self {
        key.0
    }
}

impl fmt::Display for RefreshCommandKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Which configured sources a [`RefreshRequest`] checks.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "selection", content = "source_ids", rename_all = "snake_case")]
pub enum RefreshTarget {
    /// Check every configured source for the patient workspace.
    AllSources,
    /// Check a specific non-empty, deduplicated list of source identifiers.
    Selected(Vec<SourceId>),
}

impl RefreshTarget {
    /// Validates and canonicalizes a non-empty list of [`SourceId`] values.
    pub fn selected(ids: impl IntoIterator<Item = SourceId>) -> Result<Self, RefreshInputError> {
        let unique: BTreeSet<SourceId> = ids.into_iter().collect();
        if !(1..=MAX_SELECTED_SOURCES).contains(&unique.len()) {
            return Err(RefreshInputError::EmptySourceSelection);
        }
        Ok(Self::Selected(unique.into_iter().collect()))
    }

    fn fingerprint(&self) -> String {
        match self {
            Self::AllSources => "all".to_owned(),
            Self::Selected(ids) => {
                let joined = ids
                    .iter()
                    .map(SourceId::as_str)
                    .collect::<Vec<_>>()
                    .join(",");
                format!("selected:{}", &crate::sha256_hex(joined.as_bytes())[..24])
            }
        }
    }
}

/// Validated once-only request to refresh a patient's sources.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefreshRequest {
    /// Once-only command key scoped to the caller and patient workspace.
    pub command_key: RefreshCommandKey,
    /// Whether to check all configured sources or a selected subset.
    pub target: RefreshTarget,
}

impl RefreshRequest {
    /// Constructs a validated refresh request.
    pub fn new(command_key: RefreshCommandKey, target: RefreshTarget) -> Self {
        Self {
            command_key,
            target,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRefreshArguments {
    #[serde(default)]
    source_ids: Option<Vec<String>>,
    #[serde(default)]
    request_key: Option<String>,
}

/// Parsed call to [`REFRESH_TOOL`] before binding to a work item's once-only key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefreshToolCall {
    /// Requested source selection (`AllSources` when `source_ids` was omitted).
    pub target: RefreshTarget,
    /// Optional caller-supplied key distinguishing multiple refreshes in one run.
    pub request_key: Option<RefreshCommandKey>,
}

impl RefreshToolCall {
    /// Validates tool name and JSON arguments before consulting Postgres or any adapter.
    pub fn parse(name: &str, arguments: &Value) -> Result<Self, RefreshInputError> {
        if !is_refresh_tool(name) {
            return Err(RefreshInputError::UnknownTool(name.to_owned()));
        }
        let raw: RawRefreshArguments = serde_json::from_value(arguments.clone())
            .map_err(RefreshInputError::MalformedArguments)?;
        let target = match raw.source_ids {
            None => RefreshTarget::AllSources,
            Some(raw_ids) => {
                if raw_ids.is_empty() || raw_ids.len() > MAX_SELECTED_SOURCES {
                    return Err(RefreshInputError::EmptySourceSelection);
                }
                let mut parsed_ids = Vec::with_capacity(raw_ids.len());
                for raw_id in raw_ids {
                    parsed_ids.push(SourceId::new(raw_id)?);
                }
                RefreshTarget::selected(parsed_ids)?
            }
        };
        let request_key = raw.request_key.map(RefreshCommandKey::new).transpose()?;
        Ok(Self {
            target,
            request_key,
        })
    }

    /// Derives the deterministic [`RefreshRequest`] for `work_id`.
    ///
    /// Retrying the same target within `work_id` after an interrupted source
    /// check resolves to the same [`RefreshCommandKey`], so already-settled
    /// sources are not repeated.
    pub fn into_request(self, work_id: &str) -> Result<RefreshRequest, RefreshInputError> {
        let work_trimmed = validate_text_field("work_id", work_id, 256)?;
        let work_digest = &crate::sha256_hex(work_trimmed.as_bytes())[..24];
        let raw_key = match &self.request_key {
            Some(explicit) => {
                let explicit_digest = &crate::sha256_hex(explicit.as_str().as_bytes())[..24];
                format!("refresh:{work_digest}:key:{explicit_digest}")
            }
            None => format!("refresh:{work_digest}:{}", self.target.fingerprint()),
        };
        let command_key = RefreshCommandKey::new(raw_key)?;
        Ok(RefreshRequest {
            command_key,
            target: self.target,
        })
    }
}

/// Tool definition offered to Coda runs so jobs and chats can request fresh
/// patient source records and inspect each source's refresh outcome.
pub fn refresh_tool() -> Tool {
    Tool {
        name: REFRESH_TOOL.into(),
        description: "Fetch fresh records for this patient's clinical sources and report each source's status (`updated`, `partly_updated`, or `failed`). Omit `source_ids` to check every configured source for this patient, or pass specific `source_ids` to refresh selected sources. After an `updated` or `partly_updated` result, discover and read the new record revisions through `search_patient_information` and `read_patient_sources`. When one source fails, its last good records remain readable while other sources still update.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "source_ids": {
                    "type": "array",
                    "items": { "type": "string" },
                    "minItems": 1,
                    "maxItems": MAX_SELECTED_SOURCES,
                    "description": "Optional list of source identifiers to refresh. Omit to refresh every configured source for this patient."
                },
                "request_key": {
                    "type": "string",
                    "description": "Optional key distinguishing an intentional second refresh later in the same run."
                }
            },
            "additionalProperties": false
        }),
    }
}

/// Result of checking one patient source through a [`PatientSourceRefresh`] adapter.
#[derive(Clone, Debug, PartialEq)]
pub enum SourceCheckOutcome {
    /// The source check succeeded completely and delivered fresh or current revisions.
    Updated {
        /// Physician-facing source name.
        source_label: String,
        /// Patient identity reported by this source.
        patient: SourcePatient,
        /// Practice-clock Unix seconds when the pull completed.
        pulled_at: i64,
        /// New or updated records delivered by this source.
        records: Vec<SourceRevision>,
    },
    /// Some records were fetched and saved while part of the source could not be read.
    PartlyUpdated {
        /// Physician-facing source name.
        source_label: String,
        /// Patient identity reported by this source.
        patient: SourcePatient,
        /// Practice-clock Unix seconds when the partial pull completed.
        pulled_at: i64,
        /// Readable records delivered by the available portion of this source.
        records: Vec<SourceRevision>,
        /// Clinical/operational explanation of what could not be fetched.
        reason: String,
    },
    /// The source check failed; prior retained records and last good pull stay intact.
    Failed {
        /// Physician-facing source name, when known by the adapter.
        source_label: Option<String>,
        /// Practice-clock Unix seconds when the failed check settled.
        checked_at: i64,
        /// Explanation recorded in the saved source status.
        reason: String,
    },
}

/// Interruption or transport abort while checking a patient source before any
/// [`SourceCheckOutcome`] settled.
#[derive(Debug, thiserror::Error)]
pub enum SourceRefreshAdapterError {
    /// Execution was interrupted mid-check before settling a source outcome.
    #[error("source check for `{source_id}` was interrupted: {reason}")]
    Interrupted {
        /// Source whose check was interrupted.
        source_id: SourceId,
        /// Interruption detail.
        reason: String,
    },
    /// Underlying transport aborted before settling a source outcome.
    #[error("source check transport failed for `{source_id}`: {source}")]
    Transport {
        /// Source whose transport failed.
        source_id: SourceId,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
}

/// Pluggable "check this patient's sources" operation consumed by [`REFRESH_TOOL`].
#[async_trait]
pub trait PatientSourceRefresh: Send + Sync {
    /// Lists the configured source identifiers for `(practice, workspace)` in deterministic order.
    async fn configured_sources(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<Vec<SourceId>, SourceRefreshAdapterError>;

    /// Checks one configured source for `(practice, workspace)` and returns its settled outcome.
    async fn check_source(
        &self,
        practice: &str,
        workspace: &str,
        source_id: &SourceId,
    ) -> Result<SourceCheckOutcome, SourceRefreshAdapterError>;
}

/// Saved refresh status outcome for one `(practice, workspace, source_id)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SourceRefreshOutcome {
    /// Source refreshed cleanly; new/updated records are in the versioned evidence store.
    Updated {
        /// Workspace source version after committing this source's delivery.
        workspace_version: u64,
        /// Number of new or changed record revisions appended.
        changed_records: usize,
        /// Practice-clock Unix seconds recorded on `physician_runtime_source_pulls`.
        pulled_at: i64,
    },
    /// Source partly refreshed; available records were saved and omitted records kept.
    PartlyUpdated {
        /// Workspace source version after committing the available records.
        workspace_version: u64,
        /// Number of new or changed record revisions appended.
        changed_records: usize,
        /// Practice-clock Unix seconds recorded on `physician_runtime_source_pulls`.
        pulled_at: i64,
        /// Reason part of the source could not be refreshed.
        reason: String,
    },
    /// Source failed to refresh; last good records in the evidence store are unchanged.
    Failed {
        /// Prior `pulled_at` from `physician_runtime_source_pulls`, if any good pull existed.
        last_good_pulled_at: Option<i64>,
        /// Practice-clock Unix seconds when the failed check was recorded.
        checked_at: i64,
        /// Failure reason saved on the source status.
        reason: String,
    },
}

/// Persisted status of one clinical source for one patient workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedSourceStatus {
    /// Owning practice tenant.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Source identifier.
    pub source_id: SourceId,
    /// Physician-facing source name.
    pub source_label: String,
    /// Outcome (`updated`, `partly_updated`, or `failed`) and its state-specific fields.
    #[serde(flatten)]
    pub outcome: SourceRefreshOutcome,
}

/// Combined tool and API result of refreshing a patient workspace's sources.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshReport {
    /// Refreshed patient workspace identifier.
    pub workspace: String,
    /// Workspace source version after all settled source updates in this report.
    pub workspace_version: u64,
    /// Per-source outcomes in deterministic source order.
    pub sources: Vec<SavedSourceStatus>,
}

/// Errors produced when refreshing patient sources or reading saved source statuses.
#[derive(Debug, thiserror::Error)]
pub enum RefreshError {
    /// Request or tool arguments failed validation.
    #[error("{0}")]
    InvalidInput(#[from] RefreshInputError),
    /// Practice patient membership check refused the workspace.
    #[error("{0}")]
    PracticeAccess(#[from] PracticeAccessError),
    /// Physician session or patient workspace access was refused.
    #[error("patient workspace access refused: {0}")]
    PhysicianAccess(#[source] DbErr),
    /// Command key was already used for a different refresh target or workspace.
    #[error("refresh command key `{key}` already recorded a different request")]
    IdempotencyCollision {
        /// Colliding command key.
        key: RefreshCommandKey,
    },
    /// No clinical sources are configured for this patient workspace.
    #[error("no clinical sources are configured for patient workspace `{workspace}`")]
    NoConfiguredSources {
        /// Workspace with no configured sources.
        workspace: String,
    },
    /// Requested `source_id` is not configured for this patient workspace.
    #[error(
        "source `{source_id}` is not configured for patient workspace `{workspace}` (configured: {configured})"
    )]
    UnknownSource {
        /// Patient workspace identifier.
        workspace: String,
        /// Unrecognized source identifier.
        source_id: SourceId,
        /// Comma-separated configured source identifiers.
        configured: String,
    },
    /// Source check was interrupted before settling an outcome.
    #[error("{0}")]
    Interrupted(#[from] SourceRefreshAdapterError),
    /// Report JSON encoding failed.
    #[error("failed to encode source refresh report: {0}")]
    Encoding(#[source] serde_json::Error),
    /// Runtime database failure.
    #[error("{0}")]
    Database(#[from] DbErr),
}

/// Step executed by [`FixedSourceCheck`] when a source is checked.
#[derive(Clone)]
pub enum FixedCheckStep {
    /// Return a settled [`SourceCheckOutcome`].
    Outcome(SourceCheckOutcome),
    /// Simulate an interrupted check before any outcome settles, optionally
    /// firing an external hook (such as a run [`engine::Cancellation`] handle).
    Interrupt {
        /// Reason reported on [`SourceRefreshAdapterError::Interrupted`].
        reason: String,
        /// Optional hook invoked right when the interruption triggers.
        on_interrupt: Option<Arc<dyn Fn() + Send + Sync>>,
    },
}

impl fmt::Debug for FixedCheckStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Outcome(outcome) => f.debug_tuple("Outcome").field(outcome).finish(),
            Self::Interrupt { reason, .. } => {
                f.debug_struct("Interrupt").field("reason", reason).finish()
            }
        }
    }
}

#[derive(Default)]
struct FixedSourceEntry {
    queue: VecDeque<FixedCheckStep>,
    steady: Option<SourceCheckOutcome>,
    calls: usize,
}

type FixedMap = BTreeMap<(String, String), BTreeMap<SourceId, FixedSourceEntry>>;

/// Deterministic fake source check stub that delivers configured record
/// versions, partial updates, failures, or mid-check interruptions while
/// persisting into the real [`Store`] evidence tables.
#[derive(Clone, Default)]
pub struct FixedSourceCheck {
    inner: Arc<Mutex<FixedMap>>,
}

impl FixedSourceCheck {
    /// Creates an empty stub with no configured sources.
    pub fn new() -> Self {
        Self::default()
    }

    /// Configures the steady [`SourceCheckOutcome`] returned for `(practice, workspace, source_id)`
    /// whenever no queued step precedes it.
    pub fn set_source_outcome(
        &self,
        practice: impl Into<String>,
        workspace: impl Into<String>,
        source_id: SourceId,
        outcome: SourceCheckOutcome,
    ) {
        let mut guard = self
            .inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let entry = guard
            .entry((practice.into(), workspace.into()))
            .or_default()
            .entry(source_id)
            .or_default();
        entry.steady = Some(outcome);
    }

    /// Queues one scripted [`FixedCheckStep`] ahead of any steady outcome for
    /// `(practice, workspace, source_id)`.
    pub fn push_step(
        &self,
        practice: impl Into<String>,
        workspace: impl Into<String>,
        source_id: SourceId,
        step: FixedCheckStep,
    ) {
        let mut guard = self
            .inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let entry = guard
            .entry((practice.into(), workspace.into()))
            .or_default()
            .entry(source_id)
            .or_default();
        entry.queue.push_back(step);
    }

    /// Returns how many times `check_source` was invoked for `(practice, workspace, source_id)`.
    pub fn call_count(&self, practice: &str, workspace: &str, source_id: &SourceId) -> usize {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        guard
            .get(&(practice.to_owned(), workspace.to_owned()))
            .and_then(|sources| sources.get(source_id))
            .map_or(0, |entry| entry.calls)
    }
}

#[async_trait]
impl PatientSourceRefresh for FixedSourceCheck {
    async fn configured_sources(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<Vec<SourceId>, SourceRefreshAdapterError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        Ok(guard
            .get(&(practice.to_owned(), workspace.to_owned()))
            .map(|sources| sources.keys().cloned().collect())
            .unwrap_or_default())
    }

    async fn check_source(
        &self,
        practice: &str,
        workspace: &str,
        source_id: &SourceId,
    ) -> Result<SourceCheckOutcome, SourceRefreshAdapterError> {
        let step = {
            let mut guard = self
                .inner
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let entry = guard
                .get_mut(&(practice.to_owned(), workspace.to_owned()))
                .and_then(|sources| sources.get_mut(source_id))
                .ok_or_else(|| SourceRefreshAdapterError::Interrupted {
                    source_id: source_id.clone(),
                    reason: "source is not configured in FixedSourceCheck".to_owned(),
                })?;
            entry.calls += 1;
            if let Some(queued) = entry.queue.pop_front() {
                queued
            } else if let Some(steady) = entry.steady.clone() {
                FixedCheckStep::Outcome(steady)
            } else {
                return Err(SourceRefreshAdapterError::Interrupted {
                    source_id: source_id.clone(),
                    reason: "queued source steps exhausted with no steady outcome".to_owned(),
                });
            }
        };
        match step {
            FixedCheckStep::Outcome(outcome) => Ok(outcome),
            FixedCheckStep::Interrupt {
                reason,
                on_interrupt,
            } => {
                if let Some(hook) = on_interrupt {
                    hook();
                }
                Err(SourceRefreshAdapterError::Interrupted {
                    source_id: source_id.clone(),
                    reason,
                })
            }
        }
    }
}

fn command_envelope(workspace: &str, target: &RefreshTarget) -> Value {
    json!({
        "kind": "refresh_patient_sources",
        "workspace": workspace,
        "target": target,
    })
}

fn request_digest(workspace: &str, target: &RefreshTarget) -> Result<String, DbErr> {
    let encoded = serde_json::to_vec(&command_envelope(workspace, target))
        .map_err(conversion_error("RefreshTarget", "json bytes"))?;
    Ok(crate::sha256_hex(&encoded))
}

fn physician_actor(physician: &str) -> String {
    format!("{PHYSICIAN_REFRESH_ACTOR_PREFIX}{physician}")
}

fn decode_saved_status(row: source_status::Model) -> Result<SavedSourceStatus, DbErr> {
    let source_id = SourceId::new(row.source_id)
        .map_err(conversion_error("source_status.source_id", "SourceId"))?;
    let outcome = match row.outcome.as_str() {
        "updated" => {
            let version_raw = row
                .workspace_version
                .ok_or_else(|| error("corrupt updated source_status missing workspace_version"))?;
            let workspace_version = u64::try_from(version_raw)
                .map_err(conversion_error("source_status.workspace_version", "u64"))?;
            let changed_records = usize::try_from(row.changed_records)
                .map_err(conversion_error("source_status.changed_records", "usize"))?;
            let pulled_at = row.last_good_pulled_at.unwrap_or(row.checked_at);
            SourceRefreshOutcome::Updated {
                workspace_version,
                changed_records,
                pulled_at,
            }
        }
        "partly_updated" => {
            let version_raw = row.workspace_version.ok_or_else(|| {
                error("corrupt partly_updated source_status missing workspace_version")
            })?;
            let workspace_version = u64::try_from(version_raw)
                .map_err(conversion_error("source_status.workspace_version", "u64"))?;
            let changed_records = usize::try_from(row.changed_records)
                .map_err(conversion_error("source_status.changed_records", "usize"))?;
            let pulled_at = row.last_good_pulled_at.unwrap_or(row.checked_at);
            let reason = row
                .reason
                .ok_or_else(|| error("corrupt partly_updated source_status missing reason"))?;
            SourceRefreshOutcome::PartlyUpdated {
                workspace_version,
                changed_records,
                pulled_at,
                reason,
            }
        }
        "failed" => {
            let reason = row
                .reason
                .ok_or_else(|| error("corrupt failed source_status missing reason"))?;
            SourceRefreshOutcome::Failed {
                last_good_pulled_at: row.last_good_pulled_at,
                checked_at: row.checked_at,
                reason,
            }
        }
        _ => return Err(error("corrupt source_status outcome")),
    };
    Ok(SavedSourceStatus {
        practice: row.practice,
        workspace: row.workspace,
        source_id,
        source_label: row.source_label,
        outcome,
    })
}

async fn upsert_source_status_in_tx(
    tx: &impl ConnectionTrait,
    status: &SavedSourceStatus,
) -> Result<(), DbErr> {
    let (outcome_col, checked_at, last_good_pulled_at, workspace_version, changed_records, reason) =
        match &status.outcome {
            SourceRefreshOutcome::Updated {
                workspace_version,
                changed_records,
                pulled_at,
            } => (
                "updated",
                *pulled_at,
                Some(*pulled_at),
                Some(
                    i64::try_from(*workspace_version)
                        .map_err(conversion_error("workspace_version", "i64"))?,
                ),
                i64::try_from(*changed_records)
                    .map_err(conversion_error("changed_records", "i64"))?,
                None,
            ),
            SourceRefreshOutcome::PartlyUpdated {
                workspace_version,
                changed_records,
                pulled_at,
                reason,
            } => (
                "partly_updated",
                *pulled_at,
                Some(*pulled_at),
                Some(
                    i64::try_from(*workspace_version)
                        .map_err(conversion_error("workspace_version", "i64"))?,
                ),
                i64::try_from(*changed_records)
                    .map_err(conversion_error("changed_records", "i64"))?,
                Some(reason.clone()),
            ),
            SourceRefreshOutcome::Failed {
                last_good_pulled_at,
                checked_at,
                reason,
            } => (
                "failed",
                *checked_at,
                *last_good_pulled_at,
                None,
                0_i64,
                Some(reason.clone()),
            ),
        };

    source_status::Entity::insert(source_status::ActiveModel {
        practice: Set(status.practice.clone()),
        workspace: Set(status.workspace.clone()),
        source_id: Set(status.source_id.as_str().to_owned()),
        source_label: Set(status.source_label.clone()),
        outcome: Set(outcome_col.to_owned()),
        checked_at: Set(checked_at),
        last_good_pulled_at: Set(last_good_pulled_at),
        workspace_version: Set(workspace_version),
        changed_records: Set(changed_records),
        reason: Set(reason),
    })
    .on_conflict(
        OnConflict::columns([
            source_status::Column::Practice,
            source_status::Column::Workspace,
            source_status::Column::SourceId,
        ])
        .update_columns([
            source_status::Column::SourceLabel,
            source_status::Column::Outcome,
            source_status::Column::CheckedAt,
            source_status::Column::LastGoodPulledAt,
            source_status::Column::WorkspaceVersion,
            source_status::Column::ChangedRecords,
            source_status::Column::Reason,
        ])
        .to_owned(),
    )
    .exec_without_returning(tx)
    .await?;
    Ok(())
}

async fn load_receipt_rows(
    db: &impl ConnectionTrait,
    practice: &str,
    actor: &str,
    workspace: &str,
    command_key: &RefreshCommandKey,
) -> Result<Vec<source_refresh_receipt::Model>, DbErr> {
    use source_refresh_receipt::Column as C;
    source_refresh_receipt::Entity::find()
        .filter(C::Practice.eq(practice))
        .filter(C::Actor.eq(actor))
        .filter(C::Workspace.eq(workspace))
        .filter(C::CommandKey.eq(command_key.as_str()))
        .order_by_asc(C::Ordinal)
        .order_by_asc(C::SourceId)
        .all(db)
        .await
}

fn report_from_receipts(
    workspace: &str,
    rows: Vec<source_refresh_receipt::Model>,
    fallback_version: u64,
) -> Result<RefreshReport, DbErr> {
    let mut workspace_version = fallback_version;
    let mut sources = Vec::with_capacity(rows.len());
    for row in rows {
        let row_version = u64::try_from(row.workspace_version_after).map_err(conversion_error(
            "source_refresh_receipt.workspace_version_after",
            "u64",
        ))?;
        workspace_version = workspace_version.max(row_version);
        let status: SavedSourceStatus = serde_json::from_value(row.status).map_err(
            conversion_error("source_refresh_receipt.status", "SavedSourceStatus"),
        )?;
        sources.push(status);
    }
    Ok(RefreshReport {
        workspace: workspace.to_owned(),
        workspace_version,
        sources,
    })
}

fn map_tool_outcome(
    result: Result<RefreshReport, RefreshError>,
) -> Result<ToolOutcome, engine::ProductError> {
    match result {
        Ok(report) => {
            let encoded = serde_json::to_string(&report).map_err(RefreshError::Encoding)?;
            Ok(ToolOutcome::Returned(encoded))
        }
        Err(
            err @ (RefreshError::InvalidInput(_)
            | RefreshError::NoConfiguredSources { .. }
            | RefreshError::UnknownSource { .. }
            | RefreshError::IdempotencyCollision { .. }),
        ) => Ok(ToolOutcome::refused(err.to_string())),
        Err(RefreshError::Database(db_err)) => Err(super::persistence_failure(db_err)),
        Err(other) => Err(other.into()),
    }
}

impl Store {
    /// Reads all saved per-source refresh statuses for `(practice, workspace)`,
    /// ordered by `source_id`.
    pub async fn saved_source_statuses(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<Vec<SavedSourceStatus>, RefreshError> {
        let practice = validate_text_field("practice", practice, 256)?;
        let workspace = validate_text_field("workspace", workspace, 256)?;
        let rows = source_status::Entity::find()
            .filter(source_status::Column::Practice.eq(&practice))
            .filter(source_status::Column::Workspace.eq(&workspace))
            .order_by_asc(source_status::Column::SourceId)
            .all(&self.db)
            .await?;
        let mut statuses = Vec::with_capacity(rows.len());
        for row in rows {
            statuses.push(decode_saved_status(row)?);
        }
        Ok(statuses)
    }

    /// Refreshes patient sources for an authorized physician conversation or work
    /// item, rechecking physician/panel access first on every call and retry.
    pub async fn refresh_patient_sources_for_physician(
        &self,
        scope: &Scope,
        authority: &mut dyn Authority,
        request: &RefreshRequest,
        checker: &dyn PatientSourceRefresh,
    ) -> Result<RefreshReport, RefreshError> {
        self.authorize(scope, authority)
            .await
            .map_err(RefreshError::PhysicianAccess)?;
        let actor = physician_actor(&scope.physician);
        self.refresh_sources_core(&scope.practice, &actor, &scope.workspace, request, checker)
            .await
    }

    /// Refreshes patient sources for a practice-owned background job, rechecking
    /// practice patient registration first on every call and retry.
    pub async fn refresh_patient_sources_for_practice(
        &self,
        scope: &PracticeScope,
        authority: &dyn PracticeAuthority,
        request: &RefreshRequest,
        checker: &dyn PatientSourceRefresh,
    ) -> Result<RefreshReport, RefreshError> {
        authority.check_practice_patient(scope).await?;
        self.refresh_sources_core(
            &scope.practice,
            PRACTICE_REFRESH_ACTOR,
            &scope.workspace,
            request,
            checker,
        )
        .await
    }

    /// Executes a [`REFRESH_TOOL`] call for an authorized physician run,
    /// rechecking physician/panel access first.
    pub async fn execute_physician_refresh_tool(
        &self,
        scope: &Scope,
        authority: &mut dyn Authority,
        work_id: &str,
        call: &Invocation<Value>,
        checker: &dyn PatientSourceRefresh,
    ) -> Result<ToolOutcome, engine::ProductError> {
        if let Err(err) = self.authorize(scope, authority).await {
            return Err(RefreshError::PhysicianAccess(err).into());
        }
        let request = match RefreshToolCall::parse(&call.name, &call.arguments)
            .and_then(|parsed| parsed.into_request(work_id))
        {
            Ok(req) => req,
            Err(err) => return Ok(ToolOutcome::refused(err.to_string())),
        };
        let actor = physician_actor(&scope.physician);
        let result = self
            .refresh_sources_core(&scope.practice, &actor, &scope.workspace, &request, checker)
            .await;
        map_tool_outcome(result)
    }

    /// Executes a [`REFRESH_TOOL`] call for a practice-owned background job,
    /// rechecking practice patient registration first.
    pub async fn execute_practice_refresh_tool(
        &self,
        scope: &PracticeScope,
        authority: &dyn PracticeAuthority,
        work_id: &str,
        call: &Invocation<Value>,
        checker: &dyn PatientSourceRefresh,
    ) -> Result<ToolOutcome, engine::ProductError> {
        authority.check_practice_patient(scope).await?;
        let request = match RefreshToolCall::parse(&call.name, &call.arguments)
            .and_then(|parsed| parsed.into_request(work_id))
        {
            Ok(req) => req,
            Err(err) => return Ok(ToolOutcome::refused(err.to_string())),
        };
        let result = self
            .refresh_sources_core(
                &scope.practice,
                PRACTICE_REFRESH_ACTOR,
                &scope.workspace,
                &request,
                checker,
            )
            .await;
        map_tool_outcome(result)
    }

    /// Executes [`REFRESH_TOOL`] inside an already-fenced [`super::PhysicianHost`]
    /// turn after [`Product::assert_current`] verified session and panel access.
    pub(super) async fn execute_claimed_physician_refresh_tool(
        &self,
        claim: &Claim,
        work_id: &str,
        call: &Invocation<Value>,
    ) -> Result<ToolOutcome, engine::ProductError> {
        self.assert_current(claim).await?;
        let request = match RefreshToolCall::parse(&call.name, &call.arguments)
            .and_then(|parsed| parsed.into_request(work_id))
        {
            Ok(req) => req,
            Err(err) => return Ok(ToolOutcome::refused(err.to_string())),
        };
        let actor = physician_actor(&claim.scope.physician);
        let result = self
            .refresh_sources_core(
                &claim.scope.practice,
                &actor,
                &claim.scope.workspace,
                &request,
                self.source_refresh(),
            )
            .await;
        map_tool_outcome(result)
    }

    /// Executes [`REFRESH_TOOL`] inside an already-fenced [`super::jobs::PracticeJobHost`]
    /// turn after [`Product::assert_current`] verified practice patient access.
    pub(super) async fn execute_claimed_practice_refresh_tool(
        &self,
        claim: &super::jobs::PracticeJobClaim,
        work_id: &str,
        call: &Invocation<Value>,
    ) -> Result<ToolOutcome, engine::ProductError> {
        self.assert_practice_job_current(claim).await?;
        let request = match RefreshToolCall::parse(&call.name, &call.arguments)
            .and_then(|parsed| parsed.into_request(work_id))
        {
            Ok(req) => req,
            Err(err) => return Ok(ToolOutcome::refused(err.to_string())),
        };
        let result = self
            .refresh_sources_core(
                &claim.scope.practice,
                PRACTICE_REFRESH_ACTOR,
                &claim.scope.workspace,
                &request,
                self.source_refresh(),
            )
            .await;
        map_tool_outcome(result)
    }

    /// Read-only reconciliation of a completed [`REFRESH_TOOL`] call for
    /// [`engine::Product::saved_outcome`] during Chords recovery.
    ///
    /// Never invokes a source check; returns `Ok(None)` when the command had not
    /// yet settled every requested source before interruption.
    pub async fn saved_physician_refresh_outcome(
        &self,
        scope: &Scope,
        work_id: &str,
        call: &Invocation,
    ) -> Result<Option<ToolOutcome>, engine::ProductError> {
        let actor = physician_actor(&scope.physician);
        self.saved_refresh_outcome_inner(&scope.practice, &actor, &scope.workspace, work_id, call)
            .await
    }

    /// Read-only reconciliation of a completed [`REFRESH_TOOL`] call for a
    /// practice background job during Chords recovery. Never executes a source check.
    pub async fn saved_practice_refresh_outcome(
        &self,
        scope: &PracticeScope,
        work_id: &str,
        call: &Invocation,
    ) -> Result<Option<ToolOutcome>, engine::ProductError> {
        self.saved_refresh_outcome_inner(
            &scope.practice,
            PRACTICE_REFRESH_ACTOR,
            &scope.workspace,
            work_id,
            call,
        )
        .await
    }

    async fn saved_refresh_outcome_inner(
        &self,
        practice: &str,
        actor: &str,
        workspace: &str,
        work_id: &str,
        call: &Invocation,
    ) -> Result<Option<ToolOutcome>, engine::ProductError> {
        if !is_refresh_tool(&call.name) {
            return Ok(None);
        }
        let Ok(arguments) = serde_json::from_str::<Value>(&call.arguments) else {
            return Ok(None);
        };
        let Ok(parsed) = RefreshToolCall::parse(&call.name, &arguments) else {
            return Ok(None);
        };
        let Ok(request) = parsed.into_request(work_id) else {
            return Ok(None);
        };
        let envelope = command_envelope(workspace, &request.target);
        let tx = self.snapshot().await.map_err(super::persistence_failure)?;
        let Some(saved_command) = command::Entity::find_by_id((
            practice.to_owned(),
            actor.to_owned(),
            request.command_key.as_str().to_owned(),
        ))
        .one(&tx)
        .await
        .map_err(super::persistence_failure)?
        else {
            tx.commit().await.map_err(super::persistence_failure)?;
            return Ok(None);
        };
        if !receipt::matches(&saved_command.command, &envelope) {
            tx.commit().await.map_err(super::persistence_failure)?;
            return Ok(Some(ToolOutcome::refused(
                RefreshError::IdempotencyCollision {
                    key: request.command_key,
                }
                .to_string(),
            )));
        }
        let rows = load_receipt_rows(&tx, practice, actor, workspace, &request.command_key)
            .await
            .map_err(super::persistence_failure)?;
        let fallback_version = super::sources::snapshot_version_in_tx(&tx, workspace)
            .await
            .map_err(super::persistence_failure)?;
        tx.commit().await.map_err(super::persistence_failure)?;
        let report = report_from_receipts(workspace, rows, fallback_version)?;
        let encoded = serde_json::to_string(&report).map_err(RefreshError::Encoding)?;
        Ok(Some(ToolOutcome::Returned(encoded)))
    }

    async fn refresh_sources_core(
        &self,
        practice: &str,
        actor: &str,
        workspace: &str,
        request: &RefreshRequest,
        checker: &dyn PatientSourceRefresh,
    ) -> Result<RefreshReport, RefreshError> {
        let practice = validate_text_field("practice", practice, 256)?;
        let workspace = validate_text_field("workspace", workspace, 256)?;
        let envelope = command_envelope(&workspace, &request.target);
        let sha256 = request_digest(&workspace, &request.target)?;

        // 1. Check whether the entire refresh command already settled in the run
        // store's command receipt table, or collided with a different request.
        {
            let tx = self.db.begin().await?;
            if let Some(existing_cmd) = command::Entity::find_by_id((
                practice.clone(),
                actor.to_owned(),
                request.command_key.as_str().to_owned(),
            ))
            .one(&tx)
            .await?
            {
                if !receipt::matches(&existing_cmd.command, &envelope) {
                    return Err(RefreshError::IdempotencyCollision {
                        key: request.command_key.clone(),
                    });
                }
                let rows =
                    load_receipt_rows(&tx, &practice, actor, &workspace, &request.command_key)
                        .await?;
                let current_version = current_version_in_tx(&tx, &workspace).await?;
                tx.commit().await?;
                return Ok(report_from_receipts(&workspace, rows, current_version)?);
            }

            let prior_rows =
                load_receipt_rows(&tx, &practice, actor, &workspace, &request.command_key).await?;
            if prior_rows
                .iter()
                .any(|row| row.request_sha256 != sha256 || row.workspace != workspace)
            {
                return Err(RefreshError::IdempotencyCollision {
                    key: request.command_key.clone(),
                });
            }
            // Also detect a same-key collision against another workspace under `(practice, actor)`.
            let foreign_workspace = source_refresh_receipt::Entity::find()
                .filter(source_refresh_receipt::Column::Practice.eq(&practice))
                .filter(source_refresh_receipt::Column::Actor.eq(actor))
                .filter(source_refresh_receipt::Column::CommandKey.eq(request.command_key.as_str()))
                .filter(source_refresh_receipt::Column::Workspace.ne(&workspace))
                .one(&tx)
                .await?;
            if foreign_workspace.is_some() {
                return Err(RefreshError::IdempotencyCollision {
                    key: request.command_key.clone(),
                });
            }
            tx.commit().await?;
        }

        // 2. Resolve configured sources for `(practice, workspace)`.
        let configured_raw = checker.configured_sources(&practice, &workspace).await?;
        let configured_set: BTreeSet<SourceId> = configured_raw.into_iter().collect();
        if configured_set.is_empty() {
            return Err(RefreshError::NoConfiguredSources { workspace });
        }
        let configured_list = configured_set
            .iter()
            .map(SourceId::as_str)
            .collect::<Vec<_>>()
            .join(", ");

        let target_sources: Vec<SourceId> = match &request.target {
            RefreshTarget::AllSources => configured_set.into_iter().collect(),
            RefreshTarget::Selected(selected) => {
                for source_id in selected {
                    if !configured_set.contains(source_id) {
                        return Err(RefreshError::UnknownSource {
                            workspace,
                            source_id: source_id.clone(),
                            configured: configured_list,
                        });
                    }
                }
                selected.clone()
            }
        };

        // 3. Process each target source in order, reusing any per-source receipt
        // already committed before an earlier interruption.
        let mut settled_statuses = Vec::with_capacity(target_sources.len());
        let mut latest_version = 0_u64;

        for (ordinal, source_id) in target_sources.iter().enumerate() {
            let ordinal_i64 =
                i64::try_from(ordinal).map_err(conversion_error("source ordinal", "i64"))?;
            if let Some(existing) = source_refresh_receipt::Entity::find_by_id((
                practice.clone(),
                actor.to_owned(),
                workspace.clone(),
                request.command_key.as_str().to_owned(),
                source_id.as_str().to_owned(),
            ))
            .one(&self.db)
            .await?
            {
                if existing.request_sha256 != sha256 {
                    return Err(RefreshError::IdempotencyCollision {
                        key: request.command_key.clone(),
                    });
                }
                let version_after = u64::try_from(existing.workspace_version_after).map_err(
                    conversion_error("source_refresh_receipt.workspace_version_after", "u64"),
                )?;
                let saved_status: SavedSourceStatus = serde_json::from_value(existing.status)
                    .map_err(conversion_error(
                        "source_refresh_receipt.status",
                        "SavedSourceStatus",
                    ))?;
                latest_version = latest_version.max(version_after);
                settled_statuses.push(saved_status);
                continue;
            }

            let outcome = checker
                .check_source(&practice, &workspace, source_id)
                .await?;

            let (saved_status, version_after) = self
                .commit_single_source_refresh(
                    &practice,
                    actor,
                    &workspace,
                    &request.command_key,
                    &sha256,
                    ordinal_i64,
                    source_id,
                    outcome,
                )
                .await?;
            latest_version = latest_version.max(version_after);
            settled_statuses.push(saved_status);
        }

        // 4. Record the completed command receipt in `physician_runtime_commands`.
        let tx = self.db.begin().await?;
        match receipt::record(
            &tx,
            &practice,
            actor,
            request.command_key.as_str(),
            &envelope,
        )
        .await?
        {
            Receipt::New | Receipt::Same => {}
            Receipt::Different => {
                return Err(RefreshError::IdempotencyCollision {
                    key: request.command_key.clone(),
                });
            }
        }
        let current_version = current_version_in_tx(&tx, &workspace).await?;
        tx.commit().await?;

        Ok(RefreshReport {
            workspace,
            workspace_version: latest_version.max(current_version),
            sources: settled_statuses,
        })
    }

    #[allow(clippy::too_many_arguments)]
    async fn commit_single_source_refresh(
        &self,
        practice: &str,
        actor: &str,
        workspace: &str,
        command_key: &RefreshCommandKey,
        request_sha256: &str,
        ordinal: i64,
        source_id: &SourceId,
        outcome: SourceCheckOutcome,
    ) -> Result<(SavedSourceStatus, u64), RefreshError> {
        let tx = self.db.begin().await?;

        // Re-check inside the transaction in case a concurrent worker settled this
        // exact `(practice, actor, workspace, command_key, source_id)` first.
        if let Some(existing) = source_refresh_receipt::Entity::find_by_id((
            practice.to_owned(),
            actor.to_owned(),
            workspace.to_owned(),
            command_key.as_str().to_owned(),
            source_id.as_str().to_owned(),
        ))
        .one(&tx)
        .await?
        {
            if existing.request_sha256 != request_sha256 {
                return Err(RefreshError::IdempotencyCollision {
                    key: command_key.clone(),
                });
            }
            let version_after = u64::try_from(existing.workspace_version_after).map_err(
                conversion_error("source_refresh_receipt.workspace_version_after", "u64"),
            )?;
            let saved_status: SavedSourceStatus = serde_json::from_value(existing.status).map_err(
                conversion_error("source_refresh_receipt.status", "SavedSourceStatus"),
            )?;
            tx.commit().await?;
            return Ok((saved_status, version_after));
        }

        let (saved_status, version_after) = match outcome {
            SourceCheckOutcome::Updated {
                source_label,
                patient,
                pulled_at,
                records,
            } => {
                let label = validate_text_field("source_label", &source_label, 256)?;
                let pull = SourcePull {
                    source_id: source_id.as_str().to_owned(),
                    source_label: label.clone(),
                    patient,
                    pulled_at: Some(pulled_at),
                };
                let (version_after, changed_records) = record_delivery_in_tx(
                    &tx,
                    workspace,
                    DeliveryScope::ChangedRecords,
                    &records,
                    &pull,
                )
                .await?;
                (
                    SavedSourceStatus {
                        practice: practice.to_owned(),
                        workspace: workspace.to_owned(),
                        source_id: source_id.clone(),
                        source_label: label,
                        outcome: SourceRefreshOutcome::Updated {
                            workspace_version: version_after,
                            changed_records,
                            pulled_at,
                        },
                    },
                    version_after,
                )
            }
            SourceCheckOutcome::PartlyUpdated {
                source_label,
                patient,
                pulled_at,
                records,
                reason,
            } => {
                let label = validate_text_field("source_label", &source_label, 256)?;
                let reason = validate_text_field("reason", &reason, MAX_REASON_CHARS)?;
                let pull = SourcePull {
                    source_id: source_id.as_str().to_owned(),
                    source_label: label.clone(),
                    patient,
                    pulled_at: Some(pulled_at),
                };
                let (version_after, changed_records) = record_delivery_in_tx(
                    &tx,
                    workspace,
                    DeliveryScope::ChangedRecords,
                    &records,
                    &pull,
                )
                .await?;
                (
                    SavedSourceStatus {
                        practice: practice.to_owned(),
                        workspace: workspace.to_owned(),
                        source_id: source_id.clone(),
                        source_label: label,
                        outcome: SourceRefreshOutcome::PartlyUpdated {
                            workspace_version: version_after,
                            changed_records,
                            pulled_at,
                            reason,
                        },
                    },
                    version_after,
                )
            }
            SourceCheckOutcome::Failed {
                source_label,
                checked_at,
                reason,
            } => {
                let reason = validate_text_field("reason", &reason, MAX_REASON_CHARS)?;
                let prior_pull = last_source_pull_in_tx(&tx, workspace, source_id.as_str()).await?;
                let prior_status = source_status::Entity::find_by_id((
                    practice.to_owned(),
                    workspace.to_owned(),
                    source_id.as_str().to_owned(),
                ))
                .one(&tx)
                .await?;
                let resolved_label = match source_label {
                    Some(explicit) => validate_text_field("source_label", &explicit, 256)?,
                    None => prior_pull
                        .as_ref()
                        .map(|pull| pull.source_label.clone())
                        .or_else(|| prior_status.as_ref().map(|row| row.source_label.clone()))
                        .unwrap_or_else(|| source_id.as_str().to_owned()),
                };
                let last_good_pulled_at = prior_pull
                    .and_then(|pull| pull.pulled_at)
                    .or_else(|| prior_status.and_then(|row| row.last_good_pulled_at));
                let version_after = current_version_in_tx(&tx, workspace).await?;
                (
                    SavedSourceStatus {
                        practice: practice.to_owned(),
                        workspace: workspace.to_owned(),
                        source_id: source_id.clone(),
                        source_label: resolved_label,
                        outcome: SourceRefreshOutcome::Failed {
                            last_good_pulled_at,
                            checked_at,
                            reason,
                        },
                    },
                    version_after,
                )
            }
        };

        upsert_source_status_in_tx(&tx, &saved_status).await?;

        let recorded_at = now(&tx).await?;
        let status_json = serde_json::to_value(&saved_status)
            .map_err(conversion_error("SavedSourceStatus", "json"))?;
        let version_after_i64 = i64::try_from(version_after)
            .map_err(conversion_error("workspace_version_after", "i64"))?;

        source_refresh_receipt::Entity::insert(source_refresh_receipt::ActiveModel {
            practice: Set(practice.to_owned()),
            actor: Set(actor.to_owned()),
            workspace: Set(workspace.to_owned()),
            command_key: Set(command_key.as_str().to_owned()),
            source_id: Set(source_id.as_str().to_owned()),
            ordinal: Set(ordinal),
            request_sha256: Set(request_sha256.to_owned()),
            workspace_version_after: Set(version_after_i64),
            status: Set(status_json),
            recorded_at: Set(recorded_at),
        })
        .exec_without_returning(&tx)
        .await?;

        tx.commit().await?;
        Ok((saved_status, version_after))
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn refresh_tool_schema_validates_arguments() {
        let tool = refresh_tool();
        assert_eq!(tool.name, REFRESH_TOOL);
        assert!(is_refresh_tool(&tool.name));
        assert!(!is_refresh_tool("read_patient_sources"));

        let all = RefreshToolCall::parse(REFRESH_TOOL, &json!({})).expect("omit source_ids");
        assert_eq!(all.target, RefreshTarget::AllSources);
        assert_eq!(all.request_key, None);

        let selected = RefreshToolCall::parse(
            REFRESH_TOOL,
            &json!({
                "source_ids": ["labs", "ehr", "labs"],
                "request_key": "pass-2"
            }),
        )
        .expect("selected sources deduplicate and sort");
        assert_eq!(
            selected.target,
            RefreshTarget::Selected(vec![
                SourceId::new("ehr").unwrap(),
                SourceId::new("labs").unwrap(),
            ])
        );
        assert_eq!(
            selected.request_key,
            Some(RefreshCommandKey::new("pass-2").unwrap())
        );

        assert!(matches!(
            RefreshToolCall::parse(REFRESH_TOOL, &json!({ "source_ids": [] })),
            Err(RefreshInputError::EmptySourceSelection)
        ));
        assert!(matches!(
            RefreshToolCall::parse(REFRESH_TOOL, &json!({ "workspace": "pw-foreign" })),
            Err(RefreshInputError::MalformedArguments(_))
        ));
    }

    #[test]
    fn request_keys_are_deterministic_per_work_and_target() {
        let call_a =
            RefreshToolCall::parse(REFRESH_TOOL, &json!({ "source_ids": ["labs", "ehr"] }))
                .unwrap();
        let call_b =
            RefreshToolCall::parse(REFRESH_TOOL, &json!({ "source_ids": ["ehr", "labs"] }))
                .unwrap();
        let req_a = call_a.into_request("work-1").unwrap();
        let req_b = call_b.into_request("work-1").unwrap();
        assert_eq!(req_a.command_key, req_b.command_key);

        let other_work =
            RefreshToolCall::parse(REFRESH_TOOL, &json!({ "source_ids": ["ehr", "labs"] }))
                .unwrap()
                .into_request("work-2")
                .unwrap();
        assert_ne!(req_a.command_key, other_work.command_key);
    }
}
