//! Practice-side connected-care intake from [`front_door::FrontDoorStore`]:
//!
//! 1. **Join / rejoin with consent:** when a linked patient has granted (or re-granted)
//!    sharing consent, existing consented Duet shared records are filed once into the
//!    patient's chart (`duet-patient-share`) and practice agents wake immediately with a
//!    priority [`PatientReview`] keyed to `(practice, workspace, consented_at)`.
//! 2. **`"For my doctor"` intake:** patient-prepared shares sent while sharing consent is on
//!    are filed once with explicit origin (`"Duet — For my doctor"`) and sent date, trigger
//!    a priority [`PatientReview`] (`ReviewReason::ForMyDoctor` / `ReviewTrigger::ForMyDoctor`),
//!    and report `"received"` (never `"physician looked"`) back to `front-door`.

use std::collections::BTreeMap;

use chrono::{DateTime, SecondsFormat, Utc};
use front_door::{ForMyDoctorError, FrontDoorStore};
use patient_work::{ForMyDoctorTrigger, ReviewDispatchInput, ReviewReason, ReviewRequest};
use physician_auth::{AuthStore, StoreUnavailable};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set, sea_query::OnConflict,
};
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

/// Summary of one connected-care sync pass for a practice.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConnectedCareSyncReport {
    /// Consented patient workspaces whose shared Duet records were inspected/synced.
    pub consented_workspaces_checked: usize,
    /// Workspaces woken immediately because a new or renewed consent timestamp was observed.
    pub consent_wakes_started: usize,
    /// `"For my doctor"` shares filed and admitted for priority review on this pass.
    pub shares_ingested: usize,
    /// `"For my doctor"` shares acknowledged back to `front-door` as `"received"`.
    pub shares_marked_received: usize,
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
    /// Synchronizes consented Duet shared records, immediate join/rejoin consent wakes,
    /// and pending `"For my doctor"` shares from `front_door` into `practice_id`.
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

        // 1. Consented linked patients: file existing shared Duet records cleanly and wake
        //    practice agents immediately when a patient joins or rejoins with fresh consent.
        let consented_links = front_door.consented_links_for_practice(practice).await?;
        for link in consented_links {
            let workspace = link.patient.patient_id.trim().to_owned();
            if workspace.is_empty() {
                return Err(ConnectedCareError::InvalidWorkspace);
            }
            ensure_practice_patient_owner(auth, practice, &workspace).await?;

            let shared_records = front_door
                .shared_records_for_practice_patient(practice, &link.patient.patient_id)
                .await?;
            let mut shared_revisions = Vec::with_capacity(shared_records.len());
            for rec in shared_records {
                let recorded_rfc = format_unix_rfc3339(rec.recorded_at.min(i64::MAX as u64) as i64);
                let content = format!(
                    "Origin: Duet (patient-shared record)\nTitle: {}\nRecorded: {}\n\n{}",
                    rec.title, recorded_rfc, rec.content_text
                );
                let sha256 = sha256_hex(content.as_bytes());
                shared_revisions.push(SourceRevision {
                    workspace: workspace.clone(),
                    source_id: DUET_PATIENT_SHARE_SOURCE_ID.to_owned(),
                    candidate_id: format!("duet-shared:{}", rec.record_id),
                    revision: rec.revision,
                    record_type: rec.record_type,
                    dates: vec![RecordDate {
                        name: "recorded".to_owned(),
                        value: recorded_rfc,
                    }],
                    content,
                    sha256,
                    readable_parts: Vec::new(),
                    provenance: json!({
                        "origin": "Duet",
                        "patient_shared": true,
                        "title": rec.title,
                        "account_id": rec.account_id,
                    }),
                    envelope: None,
                });
            }

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
                self.record_chart(&workspace, &shared_revisions, &pull)
                    .await?;
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
