//! Practice-side connected-care intake from [`front_door::FrontDoorStore`]:
//!
//! 1. **Join / rejoin with consent:** when a linked patient has granted (or re-granted)
//!    sharing consent, existing consented Duet shared records are filed once into the
//!    patient's chart (`duet-patient-share`) with exact `"from <source>, via the patient's Duet"`
//!    origin attribution and date, and practice agents wake immediately with a priority
//!    [`PatientReview`] keyed to `(practice, workspace, consented_at)`.
//! 2. **EHR duplicate labeling (`Rule 14`):** incoming Duet records matching items the
//!    practice chart already holds are kept and explicitly labeled as duplicates in both
//!    content and provenance, never silently dropped or overwriting the EHR original.
//! 3. **Post-join source corrections & retractions (`Rule 13`):** revised or removed Duet
//!    records append a new version while preserving dated prior versions, retire stale ready
//!    replies, supersede unfinished reviews holding the replaced version, and admit a
//!    fresh correction wake.
//! 4. **`"For my doctor"` intake:** patient-prepared shares sent while sharing consent is on
//!    are filed once with explicit origin (`"Duet — For my doctor"`) and sent date, trigger
//!    a priority [`PatientReview`] (`ReviewReason::ForMyDoctor` / `ReviewTrigger::ForMyDoctor`),
//!    and report `"received"` (never `"physician looked"`) back to `front-door`.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, SecondsFormat, Utc};
use front_door::{ForMyDoctorError, FrontDoorStore, SharedPatientRecord};
use patient_work::{ForMyDoctorTrigger, ReviewDispatchInput, ReviewReason, ReviewRequest};
use physician_auth::{AuthStore, StoreUnavailable};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::dispatch::ReviewReceiver;
use super::entities::{connected_care_consent, connected_care_intake};
use super::jobs::{PracticeAuthority, PracticeJobError};
use super::reviews::{PatientReviewError, PatientReviewReceiver};
use super::sources::SourcePull;
use super::{Store, dispatch::DispatchError};
use crate::physician_chat::PhysicianSkills;
use crate::physician_sources::{RecordDate, SourcePatient, SourceRevision};
use crate::{ModelRoles, sha256_hex};

/// Connected source ID used for consented Duet patient-shared health records.
pub const DUET_PATIENT_SHARE_SOURCE_ID: &str = "duet-patient-share";

/// Connected source ID used for patient-prepared `"For my doctor"` shares.
pub const DUET_FOR_MY_DOCTOR_SOURCE_ID: &str = "duet-for-my-doctor";

/// Failure while synchronizing connected-care shares or consent wakes from `front-door`.
#[derive(Debug, thiserror::Error)]
pub enum ConnectedCareError {
    /// Practice identifier was blank or invalid.
    #[error("practice identifier must be non-empty")]
    InvalidPractice,
    /// Patient workspace identifier on a linked record was blank.
    #[error("linked patient workspace identifier must be non-empty")]
    InvalidWorkspace,
    /// Linked patient workspace already belongs to another practice in `physician-auth`.
    #[error(
        "linked patient workspace `{workspace}` belongs to `{owner_practice}`, not `{requested_practice}`"
    )]
    ForeignWorkspace {
        /// Practice requested by this sync pass.
        requested_practice: String,
        /// Authoritative practice owning the workspace.
        owner_practice: String,
        /// Patient workspace identifier.
        workspace: String,
    },
    /// Shared `front-door` operation failed.
    #[error("front-door connected-care read or write failed")]
    FrontDoor(#[from] ForMyDoctorError),
    /// Practice authorization / patient registration failed.
    #[error("practice auth store unavailable")]
    Auth(#[from] StoreUnavailable),
    /// Dispatch patient registration failed.
    #[error("dispatch registration failed")]
    Dispatch(#[from] DispatchError),
    /// Practice job error while admitting priority review.
    #[error("connected-care job admission failed")]
    Job(#[from] PracticeJobError),
    /// Patient review admission failed.
    #[error("connected-care review admission failed")]
    Review(#[from] PatientReviewError),
    /// Postgres persistence error.
    #[error("connected-care database failure")]
    Database(#[from] DbErr),
}

impl ConnectedCareError {
    /// Coarse, content-free error class for operations telemetry and alerts.
    pub const fn error_class(&self) -> &'static str {
        match self {
            Self::InvalidPractice | Self::InvalidWorkspace => "invalid_input",
            Self::ForeignWorkspace { .. } => "foreign_workspace",
            Self::FrontDoor(_) => "front_door",
            Self::Auth(_) => "auth",
            Self::Dispatch(_) => "dispatch",
            Self::Job(_) => "job",
            Self::Review(_) => "review",
            Self::Database(_) => "database",
        }
    }
}

/// Summary of one connected-care sync pass for a practice.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConnectedCareSyncReport {
    /// Consented patient workspaces whose shared Duet records were inspected/synced.
    pub consented_workspaces_checked: usize,
    /// Workspaces woken immediately because a new or renewed consent timestamp was observed.
    pub consent_wakes_started: usize,
    /// Workspaces woken because a post-join source correction or retraction revised a Duet record.
    pub correction_wakes_started: usize,
    /// Duet records explicitly labeled as duplicates of existing EHR chart records (`Rule 14`).
    pub duplicates_labeled: usize,
    /// `"For my doctor"` shares filed and admitted for priority review on this pass.
    pub shares_ingested: usize,
    /// `"For my doctor"` shares acknowledged back to `front-door` as `"received"`.
    pub shares_marked_received: usize,
}

/// Why an incoming Duet record matched an existing non-Duet EHR chart record (`Rule 14`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DuplicateMatchReason {
    /// Source external record ID matched an existing chart record identifier.
    ExternalRecordId,
    /// Record type, calendar date, and normalized title matched an existing chart record.
    TypeDateAndTitle,
}

/// Existing EHR chart record matched by an incoming Duet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EhrDuplicateMatch {
    /// Candidate ID of the existing chart record.
    pub candidate_id: String,
    /// Owning EHR source ID (e.g., `"elation"`, `"hint"`).
    pub source_id: String,
    /// Rule that matched the duplicate.
    pub match_reason: DuplicateMatchReason,
}

/// Structured metadata extracted from a `front-door` [`SharedPatientRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedSharedRecord {
    /// Underlying source system name (e.g., `"Epic"`, `"Apple Health"`, `"Duet"`).
    pub source_system: String,
    /// Exact Connected Care origin attribution (`"from <source>, via the patient's Duet"`).
    pub origin_label: String,
    /// Optional source-native external record identifier for duplicate correlation.
    pub source_record_id: Option<String>,
    /// Optional normalized clinical concept identifier.
    pub source_concept: Option<String>,
    /// Optional source lifecycle status (e.g., `"corrected"`, `"entered-in-error"`, `"removed"`).
    pub source_status: Option<String>,
    /// Literal clinical record body without header framing.
    pub body: String,
}

/// Parses structured source metadata headers produced by Duet's uploader while remaining
/// backwards-compatible with plain `content_text`.
pub fn parse_shared_record(record: &SharedPatientRecord) -> ParsedSharedRecord {
    let mut source_system: Option<String> = None;
    let mut explicit_origin: Option<String> = None;
    let mut source_record_id: Option<String> = None;
    let mut source_concept: Option<String> = None;
    let mut source_status: Option<String> = None;

    let body = if let Some((header_block, rest)) = record.content_text.split_once("\n---\n") {
        let mut recognized_any = false;
        for line in header_block.lines() {
            if let Some(value) = line.strip_prefix("Source-Origin:") {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    source_system = Some(trimmed.to_owned());
                    recognized_any = true;
                }
            } else if let Some(value) = line.strip_prefix("Origin:") {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    explicit_origin = Some(trimmed.to_owned());
                    recognized_any = true;
                }
            } else if let Some(value) = line.strip_prefix("Source-Record-Id:") {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    source_record_id = Some(trimmed.to_owned());
                    recognized_any = true;
                }
            } else if let Some(value) = line.strip_prefix("Source-Concept:") {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    source_concept = Some(trimmed.to_owned());
                    recognized_any = true;
                }
            } else if let Some(value) = line.strip_prefix("Source-Status:") {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    source_status = Some(trimmed.to_owned());
                    recognized_any = true;
                }
            }
        }
        if recognized_any {
            rest.trim().to_owned()
        } else {
            record.content_text.trim().to_owned()
        }
    } else {
        record.content_text.trim().to_owned()
    };

    let source_system = source_system.unwrap_or_else(|| "Duet".to_owned());
    let origin_label = explicit_origin
        .filter(|value| value.starts_with("from ") && value.contains("via the patient's Duet"))
        .unwrap_or_else(|| format!("from {source_system}, via the patient's Duet"));

    ParsedSharedRecord {
        source_system,
        origin_label,
        source_record_id,
        source_concept,
        source_status,
        body,
    }
}

/// Checks whether an incoming Duet record matches an existing non-Duet EHR chart record.
///
/// Duplicates are never dropped; callers keep both records and label the Duet copy with
/// the returned [`EhrDuplicateMatch`] (`intent/connected-care.md` Rule 14).
pub fn find_ehr_duplicate(
    existing_ehr: &[&SourceRevision],
    incoming_type: &str,
    incoming_title: &str,
    recorded_rfc: &str,
    parsed: &ParsedSharedRecord,
) -> Option<EhrDuplicateMatch> {
    let incoming_day = calendar_day(recorded_rfc);
    let norm_title = normalize_match_text(incoming_title);
    let norm_type = incoming_type.trim().to_ascii_lowercase();

    for ehr in existing_ehr {
        if let Some(ref ext_id) = parsed.source_record_id
            && ehr_matches_external_id(ehr, ext_id)
        {
            return Some(EhrDuplicateMatch {
                candidate_id: ehr.candidate_id.clone(),
                source_id: ehr.source_id.clone(),
                match_reason: DuplicateMatchReason::ExternalRecordId,
            });
        }

        let same_type = ehr.record_type.trim().eq_ignore_ascii_case(&norm_type);
        if !same_type {
            continue;
        }
        let same_day = incoming_day.is_some_and(|day| {
            ehr.dates
                .iter()
                .filter_map(|d| calendar_day(&d.value))
                .any(|ehr_day| ehr_day == day)
        });
        if !same_day {
            continue;
        }
        let ehr_title = ehr
            .provenance
            .get("title")
            .and_then(|v| v.as_str())
            .map(normalize_match_text)
            .unwrap_or_default();
        let ehr_body = normalize_match_text(&ehr.content);
        if !norm_title.is_empty() && (ehr_title == norm_title || ehr_body.contains(&norm_title)) {
            return Some(EhrDuplicateMatch {
                candidate_id: ehr.candidate_id.clone(),
                source_id: ehr.source_id.clone(),
                match_reason: DuplicateMatchReason::TypeDateAndTitle,
            });
        }
    }
    None
}

fn ehr_matches_external_id(ehr: &SourceRevision, ext_id: &str) -> bool {
    let needle = ext_id.trim();
    if needle.is_empty() {
        return false;
    }
    if ehr.candidate_id == needle
        || ehr
            .candidate_id
            .rsplit('/')
            .next()
            .is_some_and(|tail| tail == needle)
    {
        return true;
    }
    for key in ["source_record_id", "native_id", "external_id"] {
        if ehr
            .provenance
            .get(key)
            .and_then(|v| v.as_str())
            .is_some_and(|val| val.trim() == needle)
        {
            return true;
        }
    }
    false
}

fn calendar_day(timestamp: &str) -> Option<&str> {
    let trimmed = timestamp.trim();
    if trimmed.len() >= 10
        && trimmed.as_bytes()[4] == b'-'
        && trimmed.as_bytes()[7] == b'-'
        && trimmed[..4].bytes().all(|b| b.is_ascii_digit())
        && trimmed[5..7].bytes().all(|b| b.is_ascii_digit())
        && trimmed[8..10].bytes().all(|b| b.is_ascii_digit())
    {
        Some(&trimmed[..10])
    } else {
        None
    }
}

fn normalize_match_text(raw: &str) -> String {
    raw.split_whitespace()
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Per-practice operational health of Connected Care sync (`intent/connected-care.md` P15).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum PracticeSyncStanding {
    /// Latest sync pass succeeded.
    Healthy {
        /// Unix second when sync last succeeded.
        last_synced_at: i64,
    },
    /// Latest sync pass failed; surfaced to operations alerts instead of silent warnings.
    Failing {
        /// Unix second when sync last succeeded, if ever.
        last_synced_at: Option<i64>,
        /// Unix second of the most recent failure.
        failed_at: i64,
        /// Consecutive failing poll cycles.
        consecutive_failures: u32,
        /// Content-free coarse failure category.
        error_class: String,
    },
}

/// Thread-safe tracker of per-practice Connected Care sync health and failure alerts (`P15`).
#[derive(Clone, Default)]
pub struct ConnectedCareSyncHealth {
    practices: Arc<Mutex<BTreeMap<String, PracticeSyncStanding>>>,
}

impl ConnectedCareSyncHealth {
    /// Records a successful sync pass for `practice_id`.
    pub fn record_success(&self, practice_id: &str, now_ts: i64) {
        if let Ok(mut map) = self.practices.lock() {
            map.insert(
                practice_id.to_owned(),
                PracticeSyncStanding::Healthy {
                    last_synced_at: now_ts,
                },
            );
        }
    }

    /// Records a failed sync pass for `practice_id` and emits an explicit `ERROR` telemetry alert.
    pub fn record_failure(
        &self,
        practice_id: &str,
        error_class: &'static str,
        now_ts: i64,
    ) -> PracticeSyncStanding {
        let updated = if let Ok(mut map) = self.practices.lock() {
            let (last_synced_at, consecutive_failures) = match map.get(practice_id) {
                Some(PracticeSyncStanding::Healthy { last_synced_at }) => {
                    (Some(*last_synced_at), 1)
                }
                Some(PracticeSyncStanding::Failing {
                    last_synced_at,
                    consecutive_failures,
                    ..
                }) => (*last_synced_at, consecutive_failures.saturating_add(1)),
                None => (None, 1),
            };
            let standing = PracticeSyncStanding::Failing {
                last_synced_at,
                failed_at: now_ts,
                consecutive_failures,
                error_class: error_class.to_owned(),
            };
            map.insert(practice_id.to_owned(), standing.clone());
            standing
        } else {
            PracticeSyncStanding::Failing {
                last_synced_at: None,
                failed_at: now_ts,
                consecutive_failures: 1,
                error_class: error_class.to_owned(),
            }
        };
        let consecutive = match &updated {
            PracticeSyncStanding::Failing {
                consecutive_failures,
                ..
            } => *consecutive_failures,
            PracticeSyncStanding::Healthy { .. } => 0,
        };
        tracing::error!(
            event = "physician.connected_care_sync.failed",
            otel.status_code = "ERROR",
            practice = %practice_id,
            error_class,
            consecutive_failures = consecutive,
            "connected-care sync degraded; operational attention required"
        );
        updated
    }

    /// Reads the current standing for `practice_id`, if observed.
    pub fn standing(&self, practice_id: &str) -> Option<PracticeSyncStanding> {
        self.practices
            .lock()
            .ok()
            .and_then(|map| map.get(practice_id).cloned())
    }

    /// Lists all practices currently in [`PracticeSyncStanding::Failing`].
    pub fn failing_practices(&self) -> Vec<(String, PracticeSyncStanding)> {
        let Ok(map) = self.practices.lock() else {
            return Vec::new();
        };
        map.iter()
            .filter(|(_, standing)| matches!(standing, PracticeSyncStanding::Failing { .. }))
            .map(|(practice, standing)| (practice.clone(), standing.clone()))
            .collect()
    }
}

fn format_unix_rfc3339(ts: i64) -> String {
    DateTime::<Utc>::from_timestamp(ts, 0)
        .unwrap_or(DateTime::<Utc>::UNIX_EPOCH)
        .to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn record_versions_map(sources: &[SourceRevision]) -> BTreeMap<String, String> {
    sources
        .iter()
        .map(|src| (src.candidate_id.clone(), src.revision.clone()))
        .collect()
}

fn for_my_doctor_candidate_id(share_id: &str) -> String {
    format!("for-my-doctor:{share_id}")
}

fn consent_wake_command_key(practice_id: &str, workspace: &str, consented_at: u64) -> String {
    format!("connected-care-join:{practice_id}:{workspace}:{consented_at}")
}

fn correction_wake_command_key(practice_id: &str, workspace: &str, version: u64) -> String {
    format!("connected-care-correction:{practice_id}:{workspace}:v{version}")
}

fn for_my_doctor_command_key(practice_id: &str, share_id: &str) -> String {
    format!("for-my-doctor:{practice_id}:{share_id}")
}

/// Ensures `workspace` belongs to `practice` in `auth` (registering an unclaimed workspace
/// under `practice`, and failing closed if another practice already owns it) before any
/// practice-triggered chart write touches `physician_runtime_sources`.
async fn ensure_practice_patient_owner(
    auth: &dyn AuthStore,
    practice: &str,
    workspace: &str,
) -> Result<(), ConnectedCareError> {
    match auth.patient_practice(workspace).await? {
        None => {
            auth.register_patient(practice, workspace).await?;
            Ok(())
        }
        Some(owner) if owner == practice => Ok(()),
        Some(owner) => Err(ConnectedCareError::ForeignWorkspace {
            requested_practice: practice.to_owned(),
            owner_practice: owner,
            workspace: workspace.to_owned(),
        }),
    }
}

impl Store {
    /// Synchronizes consented Duet shared records, EHR duplicate labels, post-join source
    /// correction wakes/fences, immediate join/rejoin consent wakes, and pending
    /// `"For my doctor"` shares from `front_door` into `practice_id`.
    #[allow(clippy::too_many_arguments)]
    pub async fn sync_connected_care_from_front_door(
        &self,
        auth: &dyn AuthStore,
        front_door: &dyn FrontDoorStore,
        practice_id: &str,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
        practice_authority: &dyn PracticeAuthority,
        now_ts: i64,
    ) -> Result<ConnectedCareSyncReport, ConnectedCareError> {
        let practice = practice_id.trim();
        if practice.is_empty() {
            return Err(ConnectedCareError::InvalidPractice);
        }

        let mut report = ConnectedCareSyncReport::default();
        let receiver = PatientReviewReceiver::new(self, roles, skills, practice_authority);

        // 1. Consented linked patients: file shared Duet records with exact origin attribution
        //    and EHR duplicate labels, wake practice agents on join/rejoin, and wake/supersede
        //    unfinished work on post-join source corrections (`Rules 12-14`).
        let consented_links = front_door.consented_links_for_practice(practice).await?;
        for link in consented_links {
            let workspace = link.patient.patient_id.trim().to_owned();
            if workspace.is_empty() {
                return Err(ConnectedCareError::InvalidWorkspace);
            }
            ensure_practice_patient_owner(auth, practice, &workspace).await?;

            let prior_observation = self.sources(&workspace).await?;
            let existing_ehr: Vec<&SourceRevision> = prior_observation
                .sources
                .iter()
                .filter(|src| {
                    src.source_id != DUET_PATIENT_SHARE_SOURCE_ID
                        && src.source_id != DUET_FOR_MY_DOCTOR_SOURCE_ID
                })
                .collect();
            let prior_duet_sources: BTreeMap<String, &SourceRevision> = prior_observation
                .sources
                .iter()
                .filter(|src| src.source_id == DUET_PATIENT_SHARE_SOURCE_ID)
                .map(|src| (src.candidate_id.clone(), src))
                .collect();

            let shared_records = front_door
                .shared_records_for_practice_patient(practice, &link.patient.patient_id)
                .await?;
            let mut shared_revisions = Vec::with_capacity(shared_records.len());
            let mut corrected_candidates: Vec<String> = Vec::new();
            let mut changed_duet_candidates: Vec<String> = Vec::new();

            for rec in shared_records {
                let candidate_id = format!("duet-shared:{}", rec.record_id);
                if let Some(existing) = prior_duet_sources.get(&candidate_id)
                    && existing.revision == rec.revision
                {
                    // Already recorded at this exact revision: keep the immutable revision bytes
                    // and provenance intact so idempotent replays never conflict on identity.
                    if existing
                        .provenance
                        .get("is_duplicate")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false)
                    {
                        report.duplicates_labeled += 1;
                    }
                    shared_revisions.push((*existing).clone());
                    continue;
                }

                let recorded_rfc = format_unix_rfc3339(rec.recorded_at.min(i64::MAX as u64) as i64);
                let parsed = parse_shared_record(&rec);
                let duplicate = find_ehr_duplicate(
                    &existing_ehr,
                    &rec.record_type,
                    &rec.title,
                    &recorded_rfc,
                    &parsed,
                );
                if duplicate.is_some() {
                    report.duplicates_labeled += 1;
                }

                let prior_revision = prior_duet_sources
                    .get(&candidate_id)
                    .map(|existing| existing.revision.clone());
                if prior_revision.is_some() {
                    corrected_candidates.push(candidate_id.clone());
                }
                changed_duet_candidates.push(candidate_id.clone());

                let mut content = format!(
                    "Origin: {}\nTitle: {}\nRecorded: {}",
                    parsed.origin_label, rec.title, recorded_rfc
                );
                if let Some(ref dup) = duplicate {
                    content.push_str(&format!(
                        "\nDuplicate of chart record: {} (source: {})",
                        dup.candidate_id, dup.source_id
                    ));
                }
                if let Some(ref status) = parsed.source_status {
                    content.push_str(&format!("\nStatus: {status}"));
                }
                if let Some(ref old_rev) = prior_revision {
                    content.push_str(&format!("\nSupersedes prior version: {old_rev}"));
                }
                content.push_str("\n\n");
                content.push_str(&parsed.body);

                let sha256 = sha256_hex(content.as_bytes());
                let supersedes_revision = prior_revision;
                shared_revisions.push(SourceRevision {
                    workspace: workspace.clone(),
                    source_id: DUET_PATIENT_SHARE_SOURCE_ID.to_owned(),
                    candidate_id,
                    revision: rec.revision,
                    record_type: rec.record_type,
                    dates: vec![RecordDate {
                        name: "recorded".to_owned(),
                        value: recorded_rfc.clone(),
                    }],
                    content,
                    sha256,
                    readable_parts: Vec::new(),
                    provenance: json!({
                        "origin": parsed.origin_label,
                        "source_system": parsed.source_system,
                        "via_duet": true,
                        "patient_shared": true,
                        "title": rec.title,
                        "recorded_at": recorded_rfc,
                        "account_id": rec.account_id,
                        "source_record_id": parsed.source_record_id,
                        "source_concept": parsed.source_concept,
                        "source_status": parsed.source_status,
                        "is_duplicate": duplicate.is_some(),
                        "duplicate_of_chart_record": duplicate,
                        "supersedes_revision": supersedes_revision,
                    }),
                    envelope: None,
                });
            }

            let mut chart_changed = 0usize;
            if !shared_revisions.is_empty() {
                let pull = SourcePull {
                    source_id: DUET_PATIENT_SHARE_SOURCE_ID.to_owned(),
                    source_label: "Duet".to_owned(),
                    patient: SourcePatient {
                        source_patient_id: link.account.id.to_string(),
                        display_name: link.account.display_name.clone(),
                        birth_date: None,
                        sex_or_gender: None,
                        medical_record_numbers: Vec::new(),
                    },
                    pulled_at: Some(now_ts),
                };
                // `record_chart_with_receipt` preserves dated prior versions, bumps `source_version`,
                // and retires stale ready replies citing any replaced revision.
                chart_changed = self
                    .record_chart_with_receipt(&workspace, &shared_revisions, &pull)
                    .await?
                    .changed;
            }

            self.register_patient_dispatch(practice_authority, practice, &workspace, now_ts)
                .await?;
            report.consented_workspaces_checked += 1;

            let consent_i64 = i64::try_from(link.consented_at).unwrap_or(i64::MAX);
            let existing_consent = connected_care_consent::Entity::find_by_id((
                practice.to_owned(),
                workspace.clone(),
            ))
            .one(&self.db)
            .await?;
            let needs_wake = existing_consent
                .as_ref()
                .is_none_or(|row| row.consented_at < consent_i64);

            if needs_wake {
                let observation = self.sources(&workspace).await?;
                let record_versions = record_versions_map(&observation.sources);
                let changed_candidates: Vec<String> = record_versions.keys().cloned().collect();
                let command_key = consent_wake_command_key(practice, &workspace, link.consented_at);
                let request = ReviewRequest {
                    practice_id: practice.to_owned(),
                    patient_workspace_id: workspace.clone(),
                    command_key: command_key.clone(),
                    reason: ReviewReason::NewRecords,
                    workspace_version: observation.version,
                    record_versions,
                };
                let inputs = vec![ReviewDispatchInput::ChangedRecords {
                    workspace_version: observation.version,
                    changed_candidates,
                }];
                receiver.submit_coalesced_review(&request, &inputs).await?;
                connected_care_consent::Entity::insert(connected_care_consent::ActiveModel {
                    practice: Set(practice.to_owned()),
                    workspace: Set(workspace),
                    consented_at: Set(consent_i64),
                    command_key: Set(command_key),
                    synced_at: Set(now_ts),
                })
                .on_conflict(
                    OnConflict::columns([
                        connected_care_consent::Column::Practice,
                        connected_care_consent::Column::Workspace,
                    ])
                    .update_columns([
                        connected_care_consent::Column::ConsentedAt,
                        connected_care_consent::Column::CommandKey,
                        connected_care_consent::Column::SyncedAt,
                    ])
                    .to_owned(),
                )
                .exec_without_returning(&self.db)
                .await?;
                report.consent_wakes_started += 1;
            } else if chart_changed > 0 && !changed_duet_candidates.is_empty() {
                // Rule 12 / Rule 13: post-join live updates and source corrections arrive as
                // distinct events, wake review, and supersede any unfinished review holding
                // a replaced version.
                let observation = self.sources(&workspace).await?;
                let record_versions = record_versions_map(&observation.sources);
                let command_key =
                    correction_wake_command_key(practice, &workspace, observation.version);
                let request = ReviewRequest {
                    practice_id: practice.to_owned(),
                    patient_workspace_id: workspace.clone(),
                    command_key,
                    reason: ReviewReason::NewRecords,
                    workspace_version: observation.version,
                    record_versions,
                };
                let inputs = vec![ReviewDispatchInput::ChangedRecords {
                    workspace_version: observation.version,
                    changed_candidates: changed_duet_candidates,
                }];
                receiver.submit_coalesced_review(&request, &inputs).await?;
                if !corrected_candidates.is_empty() {
                    report.correction_wakes_started += 1;
                }
            }
        }

        // 2. Pending `"For my doctor"` shares: ingest once with explicit origin & sent date,
        //    admit priority PatientReview (`ForMyDoctor`), and report `"received"` back to front-door.
        let pending_shares = front_door
            .pending_for_my_doctor_shares_for_practice(practice)
            .await?;
        for share in pending_shares {
            let workspace = share.patient.patient_id.trim().to_owned();
            if workspace.is_empty() {
                return Err(ConnectedCareError::InvalidWorkspace);
            }
            ensure_practice_patient_owner(auth, practice, &workspace).await?;

            let existing = connected_care_intake::Entity::find_by_id((
                practice.to_owned(),
                share.share_id.clone(),
            ))
            .one(&self.db)
            .await?;

            if existing.is_none() {
                let candidate_id = for_my_doctor_candidate_id(&share.share_id);
                let sent_i64 = i64::try_from(share.sent_at).unwrap_or(i64::MAX);
                let sent_rfc = format_unix_rfc3339(sent_i64);
                let content = format!(
                    "Origin: {}\nTitle: {}\nSent: {}\n\n{}",
                    share.origin_label, share.title, sent_rfc, share.summary_text
                );
                let sha256 = sha256_hex(content.as_bytes());
                let revision = SourceRevision {
                    workspace: workspace.clone(),
                    source_id: DUET_FOR_MY_DOCTOR_SOURCE_ID.to_owned(),
                    candidate_id: candidate_id.clone(),
                    revision: format!("sent-{}", share.sent_at),
                    record_type: "DocumentReference".to_owned(),
                    dates: vec![RecordDate {
                        name: "sent".to_owned(),
                        value: sent_rfc,
                    }],
                    content,
                    sha256,
                    readable_parts: Vec::new(),
                    provenance: json!({
                        "origin": share.origin_label,
                        "patient_prepared": true,
                        "title": share.title,
                        "share_id": share.share_id,
                        "sent_at": share.sent_at,
                    }),
                    envelope: None,
                };
                self.record_source(&revision).await?;
                let observation = self.sources(&workspace).await?;
                let record_versions = record_versions_map(&observation.sources);
                let command_key = for_my_doctor_command_key(practice, &share.share_id);
                let request = ReviewRequest {
                    practice_id: practice.to_owned(),
                    patient_workspace_id: workspace.clone(),
                    command_key: command_key.clone(),
                    reason: ReviewReason::ForMyDoctor,
                    workspace_version: observation.version,
                    record_versions,
                };
                let inputs = vec![ReviewDispatchInput::ForMyDoctor(ForMyDoctorTrigger {
                    share_id: share.share_id.clone(),
                    title: share.title.clone(),
                    origin_label: share.origin_label.clone(),
                    sent_at: sent_i64,
                    candidate_id: candidate_id.clone(),
                })];
                receiver.submit_coalesced_review(&request, &inputs).await?;
                connected_care_intake::Entity::insert(connected_care_intake::ActiveModel {
                    practice: Set(practice.to_owned()),
                    share_id: Set(share.share_id.clone()),
                    workspace: Set(workspace),
                    candidate_id: Set(candidate_id),
                    sent_at: Set(sent_i64),
                    command_key: Set(command_key),
                    ingested_at: Set(now_ts),
                    received_written_at: Set(None),
                })
                .on_conflict(
                    OnConflict::columns([
                        connected_care_intake::Column::Practice,
                        connected_care_intake::Column::ShareId,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .exec_without_returning(&self.db)
                .await?;
                report.shares_ingested += 1;
            }

            let needs_received_writeback = existing
                .as_ref()
                .is_none_or(|row| row.received_written_at.is_none());
            if needs_received_writeback {
                front_door
                    .mark_for_my_doctor_received(practice, &share.share_id, now_ts.max(0) as u64)
                    .await?;
                if let Some(row) = connected_care_intake::Entity::find_by_id((
                    practice.to_owned(),
                    share.share_id.clone(),
                ))
                .one(&self.db)
                .await?
                {
                    let mut active: connected_care_intake::ActiveModel = row.into();
                    active.received_written_at = Set(Some(now_ts));
                    active.update(&self.db).await?;
                }
                report.shares_marked_received += 1;
            }
        }

        let _ = receiver.take_deliveries().await;
        Ok(report)
    }

    /// Reads all synced `"For my doctor"` share intake records for one patient workspace
    /// in `practice_id`, ordered oldest-sent first.
    pub async fn synced_for_my_doctor_shares(
        &self,
        practice_id: &str,
        workspace_id: &str,
    ) -> Result<Vec<SyncedForMyDoctorShare>, DbErr> {
        use sea_orm::QueryOrder;
        let rows = connected_care_intake::Entity::find()
            .filter(connected_care_intake::Column::Practice.eq(practice_id))
            .filter(connected_care_intake::Column::Workspace.eq(workspace_id))
            .order_by_asc(connected_care_intake::Column::SentAt)
            .order_by_asc(connected_care_intake::Column::ShareId)
            .all(&self.db)
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| SyncedForMyDoctorShare {
                share_id: row.share_id,
                candidate_id: row.candidate_id,
                sent_at: row.sent_at,
                command_key: row.command_key,
                ingested_at: row.ingested_at,
                received_written_at: row.received_written_at,
            })
            .collect())
    }
}

/// Summary of one `"For my doctor"` share synced into the physician runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncedForMyDoctorShare {
    /// Front-door share identifier.
    pub share_id: String,
    /// Chart candidate identifier under `duet-for-my-doctor`.
    pub candidate_id: String,
    /// Unix second when the patient sent the share.
    pub sent_at: i64,
    /// Once-only priority review command key.
    pub command_key: String,
    /// Unix second when the share was ingested into the patient's chart.
    pub ingested_at: i64,
    /// Unix second when `"received"` was acknowledged back to `front-door`.
    pub received_written_at: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use front_door::RosterPatient;
    use uuid::Uuid;

    fn sample_shared(content_text: &str) -> SharedPatientRecord {
        SharedPatientRecord {
            account_id: Uuid::new_v4(),
            patient: RosterPatient {
                practice_id: "practice-a".into(),
                patient_id: "ws-1".into(),
            },
            record_id: "rec-1".into(),
            revision: "rev-1".into(),
            record_type: "Observation".into(),
            title: "Hemoglobin A1c".into(),
            content_text: content_text.to_owned(),
            recorded_at: 1_763_217_000,
            updated_at: 1_763_217_010,
        }
    }

    #[test]
    fn parses_exact_via_duet_origin_and_falls_back_cleanly() {
        let structured = parse_shared_record(&sample_shared(
            "Source-Origin: Epic\nOrigin: from Epic, via the patient's Duet\nSource-Record-Id: epic-obs-99\nSource-Concept: hba1c\n---\nHemoglobin A1c: 6.4 %",
        ));
        assert_eq!(structured.source_system, "Epic");
        assert_eq!(structured.origin_label, "from Epic, via the patient's Duet");
        assert_eq!(structured.source_record_id.as_deref(), Some("epic-obs-99"));
        assert_eq!(structured.source_concept.as_deref(), Some("hba1c"));
        assert_eq!(structured.body, "Hemoglobin A1c: 6.4 %");

        let plain = parse_shared_record(&sample_shared("Morning blood pressure 130/80 mmHg."));
        assert_eq!(plain.source_system, "Duet");
        assert_eq!(plain.origin_label, "from Duet, via the patient's Duet");
        assert_eq!(plain.body, "Morning blood pressure 130/80 mmHg.");
    }

    #[test]
    fn duplicate_detection_matches_external_id_or_type_date_and_title() {
        let ehr_record = SourceRevision {
            workspace: "ws-1".into(),
            source_id: "elation".into(),
            candidate_id: "elation/Observation/epic-obs-99".into(),
            revision: "v1".into(),
            record_type: "Observation".into(),
            dates: vec![RecordDate {
                name: "recorded".into(),
                value: "2025-11-15T14:30:00Z".into(),
            }],
            content: "Hemoglobin A1c: 6.4 %".into(),
            sha256: sha256_hex(b"Hemoglobin A1c: 6.4 %"),
            readable_parts: Vec::new(),
            provenance: json!({ "title": "Hemoglobin A1c" }),
            envelope: None,
        };
        let parsed_with_id = parse_shared_record(&sample_shared(
            "Source-Origin: Epic\nOrigin: from Epic, via the patient's Duet\nSource-Record-Id: epic-obs-99\n---\nHemoglobin A1c: 6.4 %",
        ));
        let by_id = find_ehr_duplicate(
            &[&ehr_record],
            "Observation",
            "Hemoglobin A1c",
            "2025-11-15T14:30:00Z",
            &parsed_with_id,
        )
        .expect("match by external id");
        assert_eq!(by_id.match_reason, DuplicateMatchReason::ExternalRecordId);
        assert_eq!(by_id.source_id, "elation");

        let parsed_no_id = parse_shared_record(&sample_shared(
            "Source-Origin: Epic\nOrigin: from Epic, via the patient's Duet\n---\nHemoglobin A1c: 6.4 %",
        ));
        let by_date_title = find_ehr_duplicate(
            &[&ehr_record],
            "Observation",
            "Hemoglobin A1c",
            "2025-11-15T18:00:00Z",
            &parsed_no_id,
        )
        .expect("match by type, date, and title");
        assert_eq!(
            by_date_title.match_reason,
            DuplicateMatchReason::TypeDateAndTitle
        );
    }

    #[test]
    fn sync_health_tracks_consecutive_failures_and_recovers_on_success() {
        let health = ConnectedCareSyncHealth::default();
        assert!(health.failing_practices().is_empty());

        health.record_success("practice-a", 1_000);
        let first_fail = health.record_failure("practice-a", "front_door", 1_060);
        assert_eq!(
            first_fail,
            PracticeSyncStanding::Failing {
                last_synced_at: Some(1_000),
                failed_at: 1_060,
                consecutive_failures: 1,
                error_class: "front_door".into(),
            }
        );
        let second_fail = health.record_failure("practice-a", "database", 1_120);
        assert_eq!(
            second_fail,
            PracticeSyncStanding::Failing {
                last_synced_at: Some(1_000),
                failed_at: 1_120,
                consecutive_failures: 2,
                error_class: "database".into(),
            }
        );
        assert_eq!(health.failing_practices().len(), 1);

        health.record_success("practice-a", 1_180);
        assert!(health.failing_practices().is_empty());
        assert_eq!(
            health.standing("practice-a"),
            Some(PracticeSyncStanding::Healthy {
                last_synced_at: 1_180,
            })
        );
    }
}
