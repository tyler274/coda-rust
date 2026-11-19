//! Durable Hint Health Chrome connector bridge state (`Scope.Now.EhrReadOnly`,
//! `P4.1`, `P4.Undecided.Scheduler`, `P4.Undecided.Roster`).
//!
//! Persists paired Chrome extension credentials (SHA-256 hashed, verified in
//! constant time), leased read jobs, delivered `hint::ChartCapture` snapshots,
//! and Hint practice appointments in Postgres so `physician-sample` (bridge HTTP
//! routes) and `physician-worker` (scheduled wakes & pre-visit prep) share one
//! durable source of truth without credentials ever leaving Chrome.

use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use physician_auth::AuthStore;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, Set,
    sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::refresh::{
    PatientSourceRefresh, SourceCheckOutcome, SourceId as RefreshSourceId,
    SourceRefreshAdapterError,
};
use super::schedule::{
    RosterSyncSummary, ScheduleDelivery, ScheduleError, ScheduleSource, ScheduleSourceError,
    ScheduleSyncOutcome, SourceId as ScheduleSourceId,
};
use super::sources::SourcePull;
use super::{Store, error, now};
use crate::physician_sources::hint::{
    ChartCapture, HINT_SOURCE_ID, HINT_SOURCE_LABEL, parse_hint_appointments, parse_hint_roster,
};
use crate::physician_sources::{SourcePullOutput, workspace_id};

pub(in crate::runtime) mod tables {
    pub(in crate::runtime) mod hint_pairing {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_hint_pairings")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub pair_id: String,
            #[sea_orm(column_type = "Text")]
            pub pair_key_hash: String,
            #[sea_orm(column_type = "Text")]
            pub source_label: String,
            pub created_at: i64,
            pub revoked_at: Option<i64>,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    pub(in crate::runtime) mod hint_capture {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_hint_captures")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub source_patient_id: String,
            #[sea_orm(column_type = "Text")]
            pub workspace: String,
            #[sea_orm(column_type = "Text")]
            pub capture_json: String,
            pub captured_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    pub(in crate::runtime) mod hint_schedule {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_hint_schedules")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(column_type = "Text")]
            pub appointments_json: String,
            pub delivered_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    pub(in crate::runtime) mod hint_job {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_hint_jobs")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub job_id: String,
            #[sea_orm(column_type = "Text")]
            pub read_kind: String,
            #[sea_orm(column_type = "Text")]
            pub patient_id: Option<String>,
            #[sea_orm(column_type = "Text")]
            pub state: String,
            #[sea_orm(column_type = "Text")]
            pub lease_id: Option<String>,
            pub leased_until: Option<i64>,
            pub created_at: i64,
            pub updated_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }
}

/// Kind of read job queued for a paired Hint Chrome connector extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HintBridgeJobKind {
    /// Practice-wide patient roster read (`GET /api/provider/patients`).
    Roster,
    /// Practice or patient appointment schedule read.
    Appointments,
    /// Single-patient 14-section `ChartCapture` read.
    ChartCapture,
}

impl HintBridgeJobKind {
    /// Stable database and wire slug.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Roster => "roster",
            Self::Appointments => "appointments",
            Self::ChartCapture => "chart_capture",
        }
    }

    fn parse(raw: &str) -> Result<Self, DbErr> {
        match raw {
            "roster" => Ok(Self::Roster),
            "appointments" => Ok(Self::Appointments),
            "chart_capture" => Ok(Self::ChartCapture),
            other => Err(error(&format!("unknown hint bridge job kind `{other}`"))),
        }
    }
}

/// Verified `X-Coda-Pair: <pair_id>:<pair_key>` header parsed at the HTTP boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HintPairHeader {
    /// Stable pairing identifier.
    pub pair_id: String,
    /// Secret pairing key compared via constant-time SHA-256 digest match.
    pub pair_key: String,
}

impl HintPairHeader {
    /// Parses `X-Coda-Pair` header value `<pair_id>:<pair_key>`.
    pub fn parse(raw: &str) -> Option<Self> {
        let (pair_id, pair_key) = raw.trim().split_once(':')?;
        let pair_id = pair_id.trim();
        let pair_key = pair_key.trim();
        if pair_id.is_empty() || pair_key.is_empty() {
            return None;
        }
        Some(Self {
            pair_id: pair_id.to_owned(),
            pair_key: pair_key.to_owned(),
        })
    }
}

/// Issued pairing credentials returned once to the pairing operator/extension.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatedHintPairing {
    /// Owning practice identifier.
    pub practice: String,
    /// Pairing identifier.
    pub pair_id: String,
    /// Plaintext pairing secret (stored only as SHA-256 hash in Postgres).
    pub pair_key: String,
    /// Physician-facing source label.
    pub source_label: String,
}

/// One leased Hint connector job handed to the Chrome extension.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeasedHintJob {
    /// Job identifier.
    pub id: String,
    /// Lease identifier required on delivery.
    pub lease: String,
    /// Source identifier (`"hint"`).
    pub source: String,
    /// Declared read kind (`roster`, `appointments`, `chart_capture`).
    pub read: HintBridgeJobKind,
    /// Patient identifier when `read` is patient-scoped.
    pub patient_id: Option<String>,
}

/// Summary of ingesting one Hint `ChartCapture` payload through [`ChartCapture::file`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HintCaptureIngestSummary {
    /// Derived patient workspace identifier.
    pub workspace: String,
    /// Hint-native patient identifier.
    pub source_patient_id: String,
    /// Number of chart records filed into the evidence store.
    pub filed_records: usize,
    /// Number of non-fatal section/item failures recorded.
    pub failed_parts: usize,
    /// Whether the capture completed cleanly without failed parts.
    pub complete: bool,
}

/// Errors produced by Hint Chrome bridge operations.
#[derive(Debug, thiserror::Error)]
pub enum HintBridgeError {
    /// Scope or identifier was empty.
    #[error("hint bridge identifier must not be empty")]
    InvalidInput,
    /// Hint chart capture failed to file into the source owner.
    #[error("hint chart capture filing failed: {0}")]
    FilingFailed(String),
    /// Hint roster or appointment payload failed validation.
    #[error("hint payload invalid: {0}")]
    InvalidPayload(String),
    /// Schedule or roster persistence error.
    #[error("{0}")]
    Schedule(#[from] ScheduleError),
    /// Auth store error.
    #[error("auth store unavailable: {0}")]
    AuthStore(#[from] physician_auth::StoreUnavailable),
    /// Runtime database error.
    #[error("{0}")]
    Database(#[from] DbErr),
}

fn constant_time_eq_hex(a: &str, b: &str) -> bool {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    if ab.len() != bb.len() {
        return false;
    }
    let mut diff = 0_u8;
    for (&x, &y) in ab.iter().zip(bb.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

impl Store {
    /// Registers a new paired Hint Chrome connector for `practice` and returns
    /// its one-time `(pair_id, pair_key)` credentials (storing only SHA-256 hash).
    pub async fn create_hint_pairing(
        &self,
        practice: &str,
        source_label: Option<&str>,
    ) -> Result<CreatedHintPairing, HintBridgeError> {
        let practice = practice.trim();
        if practice.is_empty() {
            return Err(HintBridgeError::InvalidInput);
        }
        let pair_id = uuid::Uuid::new_v4().to_string();
        let pair_key = format!(
            "{}{}",
            uuid::Uuid::new_v4().simple(),
            uuid::Uuid::new_v4().simple()
        );
        let pair_key_hash = crate::sha256_hex(pair_key.as_bytes());
        let label = source_label
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or(HINT_SOURCE_LABEL)
            .to_owned();
        let created_at = now(&self.db).await?;

        tables::hint_pairing::Entity::insert(tables::hint_pairing::ActiveModel {
            practice: Set(practice.to_owned()),
            pair_id: Set(pair_id.clone()),
            pair_key_hash: Set(pair_key_hash),
            source_label: Set(label.clone()),
            created_at: Set(created_at),
            revoked_at: Set(None),
        })
        .exec_without_returning(&self.db)
        .await?;

        Ok(CreatedHintPairing {
            practice: practice.to_owned(),
            pair_id,
            pair_key,
            source_label: label,
        })
    }

    /// Verifies `header` (`X-Coda-Pair`) in constant time and returns the owning
    /// `practice_id` when active.
    pub async fn verify_hint_pairing(
        &self,
        header: &HintPairHeader,
    ) -> Result<Option<String>, DbErr> {
        let row = tables::hint_pairing::Entity::find()
            .filter(tables::hint_pairing::Column::PairId.eq(header.pair_id.as_str()))
            .filter(tables::hint_pairing::Column::RevokedAt.is_null())
            .one(&self.db)
            .await?;
        let Some(row) = row else {
            return Ok(None);
        };
        let presented_hash = crate::sha256_hex(header.pair_key.as_bytes());
        if constant_time_eq_hex(&row.pair_key_hash, &presented_hash) {
            Ok(Some(row.practice))
        } else {
            Ok(None)
        }
    }

    /// Queues a read job for `practice`'s paired Hint Chrome connector.
    pub async fn queue_hint_job(
        &self,
        practice: &str,
        kind: HintBridgeJobKind,
        patient_id: Option<&str>,
    ) -> Result<String, HintBridgeError> {
        let practice = practice.trim();
        if practice.is_empty() {
            return Err(HintBridgeError::InvalidInput);
        }
        let patient_id = patient_id
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned);
        if kind == HintBridgeJobKind::ChartCapture && patient_id.is_none() {
            return Err(HintBridgeError::InvalidInput);
        }
        let job_id = uuid::Uuid::new_v4().to_string();
        let ts = now(&self.db).await?;
        tables::hint_job::Entity::insert(tables::hint_job::ActiveModel {
            practice: Set(practice.to_owned()),
            job_id: Set(job_id.clone()),
            read_kind: Set(kind.as_str().to_owned()),
            patient_id: Set(patient_id),
            state: Set("pending".to_owned()),
            lease_id: Set(None),
            leased_until: Set(None),
            created_at: Set(ts),
            updated_at: Set(ts),
        })
        .exec_without_returning(&self.db)
        .await?;
        Ok(job_id)
    }

    /// Leases the oldest pending (or lease-expired) Hint connector job for `practice`.
    pub async fn lease_hint_job(
        &self,
        practice: &str,
        lease_seconds: i64,
    ) -> Result<Option<LeasedHintJob>, DbErr> {
        let ts = now(&self.db).await?;
        let candidates = tables::hint_job::Entity::find()
            .filter(tables::hint_job::Column::Practice.eq(practice))
            .filter(tables::hint_job::Column::State.is_in(["pending", "leased"]))
            .order_by_asc(tables::hint_job::Column::CreatedAt)
            .all(&self.db)
            .await?;

        let Some(row) = candidates
            .into_iter()
            .find(|r| r.state == "pending" || r.leased_until.is_none_or(|until| until <= ts))
        else {
            return Ok(None);
        };
        let lease_id = uuid::Uuid::new_v4().to_string();
        let kind = HintBridgeJobKind::parse(&row.read_kind)?;
        let job_id = row.job_id.clone();
        let patient_id = row.patient_id.clone();

        tables::hint_job::ActiveModel {
            practice: sea_orm::Unchanged(row.practice),
            job_id: sea_orm::Unchanged(row.job_id),
            read_kind: sea_orm::Unchanged(row.read_kind),
            patient_id: sea_orm::Unchanged(row.patient_id),
            state: Set("leased".to_owned()),
            lease_id: Set(Some(lease_id.clone())),
            leased_until: Set(Some(ts + lease_seconds.max(15))),
            created_at: sea_orm::Unchanged(row.created_at),
            updated_at: Set(ts),
        }
        .update(&self.db)
        .await?;

        Ok(Some(LeasedHintJob {
            id: job_id,
            lease: lease_id,
            source: HINT_SOURCE_ID.to_owned(),
            read: kind,
            patient_id,
        }))
    }

    /// Returns whether `(practice, job_id)` currently holds an active lease matching `lease_id`.
    pub async fn verify_hint_lease(
        &self,
        practice: &str,
        job_id: &str,
        lease_id: &str,
    ) -> Result<bool, DbErr> {
        let Some(row) =
            tables::hint_job::Entity::find_by_id((practice.to_owned(), job_id.to_owned()))
                .one(&self.db)
                .await?
        else {
            return Ok(false);
        };
        Ok(row.state == "leased" && row.lease_id.as_deref() == Some(lease_id))
    }

    /// Marks a leased Hint connector job completed or failed if the lease matches.
    pub async fn settle_hint_job(
        &self,
        practice: &str,
        job_id: &str,
        lease_id: &str,
        succeeded: bool,
    ) -> Result<bool, DbErr> {
        let Some(row) =
            tables::hint_job::Entity::find_by_id((practice.to_owned(), job_id.to_owned()))
                .one(&self.db)
                .await?
        else {
            return Ok(false);
        };
        if row.lease_id.as_deref() != Some(lease_id) {
            return Ok(false);
        }
        let ts = now(&self.db).await?;
        tables::hint_job::ActiveModel {
            practice: sea_orm::Unchanged(row.practice),
            job_id: sea_orm::Unchanged(row.job_id),
            read_kind: sea_orm::Unchanged(row.read_kind),
            patient_id: sea_orm::Unchanged(row.patient_id),
            state: Set(if succeeded { "completed" } else { "failed" }.to_owned()),
            lease_id: Set(None),
            leased_until: Set(None),
            created_at: sea_orm::Unchanged(row.created_at),
            updated_at: Set(ts),
        }
        .update(&self.db)
        .await?;
        Ok(true)
    }

    /// Files one delivered Hint [`ChartCapture`] payload into Postgres, registers
    /// the patient on `practice`'s roster and dispatch ledger, seeds the Hint
    /// schedule link, and retains the raw capture for future source checks.
    pub async fn ingest_hint_chart_capture(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
        requested_patient_id: &str,
        capture_value: Value,
        captured_at: i64,
    ) -> Result<HintCaptureIngestSummary, HintBridgeError> {
        let practice = practice.trim();
        let requested_patient_id = requested_patient_id.trim();
        if practice.is_empty() || requested_patient_id.is_empty() {
            return Err(HintBridgeError::InvalidInput);
        }
        let capture_json = serde_json::to_string(&capture_value)
            .map_err(|err| HintBridgeError::InvalidPayload(err.to_string()))?;
        let capture: ChartCapture = serde_json::from_value(capture_value)
            .map_err(|err| HintBridgeError::InvalidPayload(err.to_string()))?;
        let workspace = workspace_id(HINT_SOURCE_ID, requested_patient_id);

        let output = capture.file(&workspace, HINT_SOURCE_ID, requested_patient_id);
        let (patient, records, failed_parts, complete) = match output {
            SourcePullOutput::Complete { patient, records } => (patient, records, 0, true),
            SourcePullOutput::Partial {
                patient: Some(patient),
                records,
                failed_parts,
                ..
            } => (patient, records, failed_parts.len(), false),
            SourcePullOutput::Partial {
                patient: None,
                failed_parts,
                ..
            } => {
                let detail = failed_parts
                    .iter()
                    .map(|p| format!("{}: {}", p.part, p.reason))
                    .collect::<Vec<_>>()
                    .join("; ");
                return Err(HintBridgeError::FilingFailed(detail));
            }
            SourcePullOutput::Failed { error } | SourcePullOutput::LapsedAccess { error } => {
                return Err(HintBridgeError::FilingFailed(error.to_string()));
            }
        };

        let pull = SourcePull {
            source_id: HINT_SOURCE_ID.to_owned(),
            source_label: HINT_SOURCE_LABEL.to_owned(),
            patient: patient.clone(),
            pulled_at: Some(captured_at),
        };
        let filed_records = records.len();
        self.record_chart(&workspace, &records, &pull).await?;

        let source_id = ScheduleSourceId::new(HINT_SOURCE_ID).expect("static hint source_id");
        self.sync_practice_roster(
            auth,
            practice,
            &source_id,
            HINT_SOURCE_LABEL,
            std::slice::from_ref(&patient),
            captured_at,
        )
        .await?;

        tables::hint_capture::Entity::insert(tables::hint_capture::ActiveModel {
            practice: Set(practice.to_owned()),
            source_patient_id: Set(requested_patient_id.to_owned()),
            workspace: Set(workspace.clone()),
            capture_json: Set(capture_json),
            captured_at: Set(captured_at),
        })
        .on_conflict(
            OnConflict::columns([
                tables::hint_capture::Column::Practice,
                tables::hint_capture::Column::SourcePatientId,
            ])
            .update_columns([
                tables::hint_capture::Column::Workspace,
                tables::hint_capture::Column::CaptureJson,
                tables::hint_capture::Column::CapturedAt,
            ])
            .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;

        Ok(HintCaptureIngestSummary {
            workspace,
            source_patient_id: requested_patient_id.to_owned(),
            filed_records,
            failed_parts,
            complete,
        })
    }

    /// Reconciles a Hint `READS.roster` payload delivered by the Chrome connector
    /// into `practice`'s roster and Hint schedule links.
    pub async fn ingest_hint_roster(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
        roster_payload: &Value,
        synced_at: i64,
    ) -> Result<(RosterSyncSummary, Vec<String>), HintBridgeError> {
        let (patients, failures) = parse_hint_roster(roster_payload)
            .map_err(|err| HintBridgeError::InvalidPayload(err.to_string()))?;
        let source_id = ScheduleSourceId::new(HINT_SOURCE_ID).expect("static hint source_id");
        let summary = self
            .sync_practice_roster(
                auth,
                practice,
                &source_id,
                HINT_SOURCE_LABEL,
                &patients,
                synced_at,
            )
            .await?;
        Ok((summary, failures))
    }

    /// Saves a Hint `READS.appointments` payload delivered by the Chrome connector
    /// and syncs it through [`HintBridgeScheduleSource`] into `practice`'s schedule.
    pub async fn ingest_hint_appointments(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
        appointments_payload: &Value,
        delivered_at: i64,
    ) -> Result<ScheduleSyncOutcome, HintBridgeError> {
        let practice = practice.trim();
        if practice.is_empty() {
            return Err(HintBridgeError::InvalidInput);
        }
        let appointments_json = serde_json::to_string(appointments_payload)
            .map_err(|err| HintBridgeError::InvalidPayload(err.to_string()))?;
        tables::hint_schedule::Entity::insert(tables::hint_schedule::ActiveModel {
            practice: Set(practice.to_owned()),
            appointments_json: Set(appointments_json),
            delivered_at: Set(delivered_at),
        })
        .on_conflict(
            OnConflict::column(tables::hint_schedule::Column::Practice)
                .update_columns([
                    tables::hint_schedule::Column::AppointmentsJson,
                    tables::hint_schedule::Column::DeliveredAt,
                ])
                .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;

        let source = HintBridgeScheduleSource::new(self.clone(), practice.to_owned());
        self.seed_schedule_links_from_pulls(auth, practice, source.source_id(), delivered_at)
            .await?;
        Ok(self
            .sync_practice_schedule(auth, practice, &source, delivered_at)
            .await?)
    }

    /// Lists practices that currently hold a delivered Hint bridge schedule snapshot.
    pub async fn hint_bridge_schedule_practices(&self) -> Result<Vec<String>, DbErr> {
        let rows = tables::hint_schedule::Entity::find().all(&self.db).await?;
        Ok(rows.into_iter().map(|r| r.practice).collect())
    }
}

/// [`ScheduleSource`] backed by the durable Hint Chrome connector schedule
/// deliveries stored in `physician_runtime_hint_schedules`.
#[derive(Clone)]
pub struct HintBridgeScheduleSource {
    store: Store,
    practice: String,
    source_id: ScheduleSourceId,
}

impl HintBridgeScheduleSource {
    /// Creates a schedule source reading durable Hint deliveries for `practice`.
    pub fn new(store: Store, practice: impl Into<String>) -> Self {
        Self {
            store,
            practice: practice.into(),
            source_id: ScheduleSourceId::new(HINT_SOURCE_ID).expect("static hint source_id"),
        }
    }
}

#[async_trait]
impl ScheduleSource for HintBridgeScheduleSource {
    fn source_id(&self) -> &ScheduleSourceId {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        HINT_SOURCE_LABEL
    }

    async fn read_schedule(&self) -> Result<ScheduleDelivery, ScheduleSourceError> {
        let row = tables::hint_schedule::Entity::find_by_id(self.practice.clone())
            .one(&self.store.db)
            .await
            .map_err(|err| ScheduleSourceError::Unavailable {
                source_id: self.source_id.clone(),
                reason: err.to_string(),
            })?;
        let Some(row) = row else {
            return Err(ScheduleSourceError::Unavailable {
                source_id: self.source_id.clone(),
                reason: "No Hint Chrome connector appointment delivery has been received yet."
                    .to_owned(),
            });
        };
        let payload: Value = serde_json::from_str(&row.appointments_json).map_err(|err| {
            ScheduleSourceError::Unavailable {
                source_id: self.source_id.clone(),
                reason: format!("stored Hint appointments JSON invalid: {err}"),
            }
        })?;
        parse_hint_appointments(&self.source_id, &payload)
    }
}

/// [`PatientSourceRefresh`] backed by durable Hint Chrome connector captures
/// (`physician_runtime_hint_captures`).
#[derive(Clone)]
pub struct HintBridgeSourceRefresh {
    store: Store,
}

impl HintBridgeSourceRefresh {
    /// Wraps `store` to check Hint bridge captures during scheduled wakes and pre-visit prep.
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}

#[async_trait]
impl PatientSourceRefresh for HintBridgeSourceRefresh {
    async fn configured_sources(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<Vec<RefreshSourceId>, SourceRefreshAdapterError> {
        let found = tables::hint_capture::Entity::find()
            .filter(tables::hint_capture::Column::Practice.eq(practice))
            .filter(tables::hint_capture::Column::Workspace.eq(workspace))
            .one(&self.store.db)
            .await
            .map_err(|err| SourceRefreshAdapterError::Interrupted {
                source_id: RefreshSourceId::new(HINT_SOURCE_ID).expect("valid hint source_id"),
                reason: err.to_string(),
            })?;
        if found.is_some() {
            Ok(vec![
                RefreshSourceId::new(HINT_SOURCE_ID).expect("valid hint source_id"),
            ])
        } else {
            Ok(Vec::new())
        }
    }

    async fn check_source(
        &self,
        practice: &str,
        workspace: &str,
        _source_id: &RefreshSourceId,
    ) -> Result<SourceCheckOutcome, SourceRefreshAdapterError> {
        let now_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs() as i64);
        let row = tables::hint_capture::Entity::find()
            .filter(tables::hint_capture::Column::Practice.eq(practice))
            .filter(tables::hint_capture::Column::Workspace.eq(workspace))
            .one(&self.store.db)
            .await
            .map_err(|err| SourceRefreshAdapterError::Interrupted {
                source_id: RefreshSourceId::new(HINT_SOURCE_ID).expect("valid hint source_id"),
                reason: err.to_string(),
            })?;
        let Some(row) = row else {
            return Ok(SourceCheckOutcome::Failed {
                source_label: Some(HINT_SOURCE_LABEL.to_owned()),
                checked_at: now_unix,
                reason: "No Hint Chrome connector capture is stored for this patient.".to_owned(),
            });
        };
        let capture: ChartCapture = match serde_json::from_str(&row.capture_json) {
            Ok(parsed) => parsed,
            Err(err) => {
                return Ok(SourceCheckOutcome::Failed {
                    source_label: Some(HINT_SOURCE_LABEL.to_owned()),
                    checked_at: now_unix,
                    reason: format!("stored Hint capture invalid: {err}"),
                });
            }
        };
        match capture.file(workspace, HINT_SOURCE_ID, &row.source_patient_id) {
            SourcePullOutput::Complete { patient, records } => Ok(SourceCheckOutcome::Updated {
                source_label: HINT_SOURCE_LABEL.to_owned(),
                patient,
                records,
                pulled_at: row.captured_at,
            }),
            SourcePullOutput::Partial {
                patient: Some(patient),
                records,
                failed_parts,
                ..
            } => Ok(SourceCheckOutcome::PartlyUpdated {
                source_label: HINT_SOURCE_LABEL.to_owned(),
                patient,
                records,
                pulled_at: row.captured_at,
                reason: failed_parts
                    .iter()
                    .map(|p| format!("{}: {}", p.part, p.reason))
                    .collect::<Vec<_>>()
                    .join("; "),
            }),
            SourcePullOutput::Partial {
                patient: None,
                failed_parts,
                ..
            } => Ok(SourceCheckOutcome::Failed {
                source_label: Some(HINT_SOURCE_LABEL.to_owned()),
                checked_at: now_unix,
                reason: failed_parts
                    .iter()
                    .map(|p| format!("{}: {}", p.part, p.reason))
                    .collect::<Vec<_>>()
                    .join("; "),
            }),
            SourcePullOutput::Failed { error } | SourcePullOutput::LapsedAccess { error } => {
                Ok(SourceCheckOutcome::Failed {
                    source_label: Some(HINT_SOURCE_LABEL.to_owned()),
                    checked_at: now_unix,
                    reason: error.to_string(),
                })
            }
        }
    }
}
