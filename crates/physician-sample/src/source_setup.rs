//! Practice-facing clinical source connection, historical Spruce backlog gate,
//! lost-access recovery, and disconnect retention (`/connections/*`).
//!
//! Scoped strictly by authenticated `practice_id` (`AuthContext` / `ConnectedCare`)
//! and persisted durably in PostgreSQL (`physician_practice_source_setups`) when a
//! database is configured, falling back to process memory only on unauthenticated
//! fixture routers.
//!
//! Key domain invariants enforced here:
//! 1. `Connected`, `RecordsImported`, and `UpToDate` are distinct saved states (`SourceStandingWire`);
//!    a credential/session check (`Connected`) never implies records were imported or up to date.
//! 2. Synthetic simulation (`SimulatedPractice`, `POST /connections/{source_id}/simulate`,
//!    synthetic pass-through verification, and synthetic record/draft seeding) is gated strictly
//!    to non-production synthetic practices (`PracticeClock::Fixed(_)`). Real-time practices
//!    without attached vendor adapters offer `CodaAssisted` onboarding and fail closed rather than
//!    faking `Connected` or writing fabricated records into real patient charts.
//! 3. Catch-up and transient retries (`SpruceBackgroundActivity`) stay completely silent on
//!    `/today` and `/connections`.
//! 4. Only `AccessLost` (`RevokedGrant` or `AdministratorBlock`) asks for `Reconnect` (`Rule 10`);
//!    transient `ReadFailed` retains its reason and `last_successful_read_at` without asking to `Reconnect`.
//! 5. Pre-connection historical Spruce messages stay held behind `SpruceBacklogWire::Offered`
//!    until explicit physician acceptance (`Rule 14`).
//! 6. Disconnecting Spruce deletes all never-linked Spruce messages while retaining linked
//!    clinical records and physician reply drafts under the practice record policy (`Rule 11`).

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, UNIX_EPOCH};

use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Extensions, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{Authority, PublicationTransaction, Scope, Store, async_trait};
use physician_auth::migration::{
    PracticeSourceSetupActiveModel, PracticeSourceSetupRow, PracticeSourceSetups,
    practice_source_setup,
};
use physician_auth::{AuthStore, Membership, PracticeClock};
use physician_wire::TodayConnection;
use physician_wire::source_setup::{
    AccessLossCauseWire, ConnectSourceBody, PracticeConnectionsWire, PracticeSourceWire,
    RequestAssistedSetupBody, SimulateSourceEventBody, SourceConnectionMethodWire,
    SourceOperationResponseWire, SourceStandingWire, SpruceBacklogWire, SupportedSourceWire,
    TodayConnectionRecoveryWire,
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set, TransactionTrait,
    sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::auth::{self, AuthContext};
use crate::care::{self, ConnectedCare};
use crate::patients::{MemoryPatients, PatientSourceRecord, SourceDeliveryState};
use crate::timestamp::format_rfc3339;

const BODY_LIMIT: usize = 64 * 1024;
const SPRUCE_REPLY_ID: &str = "spruce-retained-reply-draft";
const SPRUCE_REPLY_HEADLINE: &str = "Spruce message: blood pressure log follow-up";
const SPRUCE_REPLY_TEXT: &str = "Thank you for sending your home blood pressure readings. Please continue your current dose and bring your log to next week's visit.";

/// Background Spruce activity that must remain completely quiet in `/today` and `/connections`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum SpruceBackgroundActivity {
    /// No background catch-up or retry running.
    #[default]
    Idle,
    /// Catching up after an interruption (quiet per `message-matching` spec).
    CatchingUp,
    /// Retrying a transient transport error before surfacing failure (quiet per `message-matching` spec).
    RetryingTransient {
        /// Retry attempt count.
        attempt: u32,
    },
}

/// Whether a practice runs synthetic source rehearsal or real-world assisted onboarding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PracticeSourceMode {
    /// Synthetic fixed-clock practice in a non-production environment.
    SyntheticRehearsal,
    /// Real-time practice (or production environment) where live vendor connectors are not attached.
    LiveAssistedOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct UnlinkedSpruceMessage {
    message_id: String,
    sender_handle: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct LinkedSourceRecord {
    workspace: String,
    candidate_id: String,
    summary_sentence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RetainedReplyDraft {
    workspace: String,
    reply_work_id: String,
    headline: String,
    draft_text: String,
    created_at: String,
    revision: u64,
}

#[derive(Debug, Clone)]
struct SourceEntry {
    standing: SourceStandingWire,
    backlog: SpruceBacklogWire,
    activity: SpruceBackgroundActivity,
    unlinked_spruce: Vec<UnlinkedSpruceMessage>,
    linked_records: Vec<LinkedSourceRecord>,
    retained_drafts: Vec<RetainedReplyDraft>,
    import_generation: u64,
}

impl SourceEntry {
    fn new(source: SupportedSourceWire) -> Self {
        Self {
            standing: SourceStandingWire::Disconnected,
            backlog: match source {
                SupportedSourceWire::Spruce => SpruceBacklogWire::NonePending,
                SupportedSourceWire::Hint | SupportedSourceWire::ElationFhir => {
                    SpruceBacklogWire::NotApplicable
                }
            },
            activity: SpruceBackgroundActivity::Idle,
            unlinked_spruce: Vec::new(),
            linked_records: Vec::new(),
            retained_drafts: Vec::new(),
            import_generation: 0,
        }
    }
}

/// Errors reading or writing durable clinical source setup state.
#[derive(Debug, thiserror::Error)]
pub(crate) enum SourcePersistError {
    #[error("failed to read or write practice source setup in database")]
    Database(#[source] Box<sea_orm::DbErr>),
    #[error("corrupt practice source setup row for `{practice_id}` / `{source_id}`")]
    CorruptRow {
        practice_id: String,
        source_id: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("failed to serialize practice source setup row")]
    Serialize(#[source] serde_json::Error),
}

impl From<sea_orm::DbErr> for SourcePersistError {
    fn from(error: sea_orm::DbErr) -> Self {
        Self::Database(Box::new(error))
    }
}

#[derive(Debug)]
enum MutateError<E> {
    Persist(SourcePersistError),
    Domain(E),
}

impl<E> From<SourcePersistError> for MutateError<E> {
    fn from(error: SourcePersistError) -> Self {
        Self::Persist(error)
    }
}

fn persist_failure(error: SourcePersistError) -> Response {
    tracing::error!(?error, %error, "practice source setup persistence failed");
    StatusCode::SERVICE_UNAVAILABLE.into_response()
}

enum SourceSetupBackend {
    Memory(Mutex<BTreeMap<(String, SupportedSourceWire), SourceEntry>>),
    Postgres(DatabaseConnection),
}

/// Fallback store attached to unauthenticated fixture routers; overridden by [`Auth`]'s
/// request extension whenever auth or PostgreSQL is wired.
#[derive(Clone)]
pub(crate) struct FallbackSourceSetup(pub(crate) Arc<SourceSetupStore>);

/// Shared practice-scoped store for clinical source setup and recovery state.
pub struct SourceSetupStore {
    backend: SourceSetupBackend,
    forbid_simulation: bool,
}

impl Default for SourceSetupStore {
    fn default() -> Self {
        Self {
            backend: SourceSetupBackend::Memory(Mutex::new(BTreeMap::new())),
            forbid_simulation: false,
        }
    }
}

impl SourceSetupStore {
    /// Creates an empty in-memory practice source setup store for fixture runs.
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Creates an in-memory practice source setup store with explicit simulation policy.
    pub fn memory(forbid_simulation: bool) -> Arc<Self> {
        Arc::new(Self {
            backend: SourceSetupBackend::Memory(Mutex::new(BTreeMap::new())),
            forbid_simulation,
        })
    }

    /// Creates a PostgreSQL-backed practice source setup store sharing `db`.
    pub fn postgres(db: DatabaseConnection, forbid_simulation: bool) -> Arc<Self> {
        Arc::new(Self {
            backend: SourceSetupBackend::Postgres(db),
            forbid_simulation,
        })
    }

    /// Reads the background Spruce activity for `(practice_id, Spruce)`.
    pub async fn spruce_activity(&self, practice_id: &str) -> SpruceBackgroundActivity {
        self.read_entry(practice_id, SupportedSourceWire::Spruce)
            .await
            .map(|entry| entry.activity)
            .unwrap_or_default()
    }

    async fn load_practice_map(
        &self,
        practice_id: &str,
    ) -> Result<BTreeMap<SupportedSourceWire, SourceEntry>, SourcePersistError> {
        match &self.backend {
            SourceSetupBackend::Memory(entries) => {
                let guard = entries.lock().expect("source setup lock");
                let mut out = BTreeMap::new();
                for kind in SupportedSourceWire::ALL {
                    if let Some(entry) = guard.get(&(practice_id.to_owned(), kind)) {
                        out.insert(kind, entry.clone());
                    }
                }
                Ok(out)
            }
            SourceSetupBackend::Postgres(db) => {
                let rows = PracticeSourceSetups::find()
                    .filter(practice_source_setup::Column::PracticeId.eq(practice_id))
                    .all(db)
                    .await?;
                let mut out = BTreeMap::new();
                for row in rows {
                    if let Some(kind) = SupportedSourceWire::from_slug(&row.source_id) {
                        out.insert(kind, row_to_entry(row)?);
                    }
                }
                Ok(out)
            }
        }
    }

    async fn read_entry(
        &self,
        practice_id: &str,
        kind: SupportedSourceWire,
    ) -> Result<SourceEntry, SourcePersistError> {
        match &self.backend {
            SourceSetupBackend::Memory(entries) => {
                let guard = entries.lock().expect("source setup lock");
                Ok(guard
                    .get(&(practice_id.to_owned(), kind))
                    .cloned()
                    .unwrap_or_else(|| SourceEntry::new(kind)))
            }
            SourceSetupBackend::Postgres(db) => {
                let row = PracticeSourceSetups::find_by_id((
                    practice_id.to_owned(),
                    kind.slug().to_owned(),
                ))
                .one(db)
                .await?;
                match row {
                    Some(row) => row_to_entry(row),
                    None => Ok(SourceEntry::new(kind)),
                }
            }
        }
    }

    async fn mutate_entry<T, E>(
        &self,
        practice_id: &str,
        kind: SupportedSourceWire,
        now_unix: i64,
        f: impl FnOnce(&mut SourceEntry) -> Result<T, E>,
    ) -> Result<(SourceEntry, T), MutateError<E>> {
        match &self.backend {
            SourceSetupBackend::Memory(entries) => {
                let mut guard = entries.lock().expect("source setup lock");
                let mut working = guard
                    .get(&(practice_id.to_owned(), kind))
                    .cloned()
                    .unwrap_or_else(|| SourceEntry::new(kind));
                let out = f(&mut working).map_err(MutateError::Domain)?;
                guard.insert((practice_id.to_owned(), kind), working.clone());
                Ok((working, out))
            }
            SourceSetupBackend::Postgres(db) => {
                let txn = db.begin().await.map_err(SourcePersistError::from)?;
                let existing = PracticeSourceSetups::find_by_id((
                    practice_id.to_owned(),
                    kind.slug().to_owned(),
                ))
                .lock_exclusive()
                .one(&txn)
                .await
                .map_err(SourcePersistError::from)?;
                let mut working = match existing {
                    Some(row) => row_to_entry(row)?,
                    None => SourceEntry::new(kind),
                };
                let out = match f(&mut working) {
                    Ok(val) => val,
                    Err(err) => {
                        let _ = txn.rollback().await;
                        return Err(MutateError::Domain(err));
                    }
                };
                let active = entry_to_active_model(practice_id, kind, &working, now_unix)?;
                PracticeSourceSetups::insert(active)
                    .on_conflict(
                        OnConflict::columns([
                            practice_source_setup::Column::PracticeId,
                            practice_source_setup::Column::SourceId,
                        ])
                        .update_columns([
                            practice_source_setup::Column::StandingJson,
                            practice_source_setup::Column::BacklogJson,
                            practice_source_setup::Column::ActivityJson,
                            practice_source_setup::Column::UnlinkedSpruceJson,
                            practice_source_setup::Column::LinkedRecordsJson,
                            practice_source_setup::Column::RetainedDraftsJson,
                            practice_source_setup::Column::ImportGeneration,
                            practice_source_setup::Column::UpdatedAt,
                        ])
                        .to_owned(),
                    )
                    .exec(&txn)
                    .await
                    .map_err(SourcePersistError::from)?;
                txn.commit().await.map_err(SourcePersistError::from)?;
                Ok((working, out))
            }
        }
    }

    async fn list_practice(
        &self,
        practice_id: &str,
        mode: PracticeSourceMode,
    ) -> Result<PracticeConnectionsWire, SourcePersistError> {
        let saved = self.load_practice_map(practice_id).await?;
        let sources = SupportedSourceWire::ALL
            .into_iter()
            .map(|kind| {
                let default_entry;
                let entry = match saved.get(&kind) {
                    Some(existing) => existing,
                    None => {
                        default_entry = SourceEntry::new(kind);
                        &default_entry
                    }
                };
                project_source(kind, mode, entry)
            })
            .collect();
        Ok(PracticeConnectionsWire {
            practice_id: practice_id.to_owned(),
            sources,
        })
    }

    async fn today_connections(
        &self,
        practice_id: &str,
    ) -> Result<Vec<TodayConnection>, SourcePersistError> {
        let saved = self.load_practice_map(practice_id).await?;
        let mut out = Vec::new();
        for kind in SupportedSourceWire::ALL {
            let Some(entry) = saved.get(&kind) else {
                continue;
            };
            // Quiet catch-up and transient retries never surface on Today.
            let _quiet = entry.activity;
            match &entry.standing {
                SourceStandingWire::ReadFailed {
                    last_successful_read_at,
                    reason,
                    ..
                } => {
                    out.push(TodayConnection {
                        condition: "source_read_failed".to_owned(),
                        channel: Some(kind.slug().to_owned()),
                        source_id: Some(kind),
                        source_label: Some(kind.label().to_owned()),
                        reason: Some(reason.clone()),
                        last_successful_read_at: last_successful_read_at.clone(),
                        recovery: Some(TodayConnectionRecoveryWire::ReadFailed),
                    });
                }
                SourceStandingWire::AccessLost {
                    last_successful_read_at,
                    cause,
                    reason,
                    ..
                } => {
                    let condition = match cause {
                        AccessLossCauseWire::RevokedGrant => "reconnect_required",
                        AccessLossCauseWire::AdministratorBlock => "blocked_by_administrator",
                    };
                    out.push(TodayConnection {
                        condition: condition.to_owned(),
                        channel: Some(kind.slug().to_owned()),
                        source_id: Some(kind),
                        source_label: Some(kind.label().to_owned()),
                        reason: Some(reason.clone()),
                        last_successful_read_at: last_successful_read_at.clone(),
                        recovery: Some(TodayConnectionRecoveryWire::ReconnectRequired),
                    });
                }
                SourceStandingWire::Disconnected
                | SourceStandingWire::CodaAssistedRequested { .. }
                | SourceStandingWire::Connected { .. }
                | SourceStandingWire::RecordsImported { .. }
                | SourceStandingWire::UpToDate { .. } => {}
            }
        }
        Ok(out)
    }
}

fn row_to_entry(row: PracticeSourceSetupRow) -> Result<SourceEntry, SourcePersistError> {
    let practice_id = row.practice_id;
    let source_id = row.source_id;
    let corrupt = |source: serde_json::Error| SourcePersistError::CorruptRow {
        practice_id: practice_id.clone(),
        source_id: source_id.clone(),
        source,
    };
    Ok(SourceEntry {
        standing: serde_json::from_str(&row.standing_json).map_err(corrupt)?,
        backlog: serde_json::from_str(&row.backlog_json).map_err(corrupt)?,
        activity: serde_json::from_str(&row.activity_json).map_err(corrupt)?,
        unlinked_spruce: serde_json::from_str(&row.unlinked_spruce_json).map_err(corrupt)?,
        linked_records: serde_json::from_str(&row.linked_records_json).map_err(corrupt)?,
        retained_drafts: serde_json::from_str(&row.retained_drafts_json).map_err(corrupt)?,
        import_generation: row.import_generation.max(0) as u64,
    })
}

fn entry_to_active_model(
    practice_id: &str,
    kind: SupportedSourceWire,
    entry: &SourceEntry,
    now_unix: i64,
) -> Result<PracticeSourceSetupActiveModel, SourcePersistError> {
    Ok(PracticeSourceSetupActiveModel {
        practice_id: Set(practice_id.to_owned()),
        source_id: Set(kind.slug().to_owned()),
        standing_json: Set(
            serde_json::to_string(&entry.standing).map_err(SourcePersistError::Serialize)?
        ),
        backlog_json: Set(
            serde_json::to_string(&entry.backlog).map_err(SourcePersistError::Serialize)?
        ),
        activity_json: Set(
            serde_json::to_string(&entry.activity).map_err(SourcePersistError::Serialize)?
        ),
        unlinked_spruce_json: Set(
            serde_json::to_string(&entry.unlinked_spruce).map_err(SourcePersistError::Serialize)?
        ),
        linked_records_json: Set(
            serde_json::to_string(&entry.linked_records).map_err(SourcePersistError::Serialize)?
        ),
        retained_drafts_json: Set(
            serde_json::to_string(&entry.retained_drafts).map_err(SourcePersistError::Serialize)?
        ),
        import_generation: Set(entry.import_generation as i64),
        updated_at: Set(now_unix),
    })
}

pub(crate) fn store_from(extensions: &Extensions) -> Option<Arc<SourceSetupStore>> {
    extensions
        .get::<Arc<SourceSetupStore>>()
        .cloned()
        .or_else(|| {
            extensions
                .get::<FallbackSourceSetup>()
                .map(|fallback| Arc::clone(&fallback.0))
        })
}

async fn resolve_practice_mode(
    connected: &ConnectedCare,
    setup: &SourceSetupStore,
    extensions: &Extensions,
    practice_id: &str,
) -> Result<PracticeSourceMode, physician_auth::StoreUnavailable> {
    if setup.forbid_simulation {
        return Ok(PracticeSourceMode::LiveAssistedOnly);
    }
    let auth_store = connected.auth_store(extensions);
    let record = auth_store.practice(practice_id).await?;
    match record.map(|rec| rec.clock) {
        Some(PracticeClock::Fixed(_)) => Ok(PracticeSourceMode::SyntheticRehearsal),
        Some(PracticeClock::RealTime) | None => Ok(PracticeSourceMode::LiveAssistedOnly),
    }
}

fn source_metadata(
    kind: SupportedSourceWire,
    mode: PracticeSourceMode,
) -> (
    &'static str,
    &'static str,
    Vec<String>,
    Vec<SourceConnectionMethodWire>,
) {
    let methods = match (kind, mode) {
        (_, PracticeSourceMode::LiveAssistedOnly) => {
            vec![SourceConnectionMethodWire::CodaAssisted]
        }
        (SupportedSourceWire::Hint, PracticeSourceMode::SyntheticRehearsal) => vec![
            SourceConnectionMethodWire::BrowserSession,
            SourceConnectionMethodWire::SimulatedPractice,
            SourceConnectionMethodWire::CodaAssisted,
        ],
        (
            SupportedSourceWire::ElationFhir | SupportedSourceWire::Spruce,
            PracticeSourceMode::SyntheticRehearsal,
        ) => vec![
            SourceConnectionMethodWire::CredentialSignIn,
            SourceConnectionMethodWire::SimulatedPractice,
            SourceConnectionMethodWire::CodaAssisted,
        ],
    };
    match kind {
        SupportedSourceWire::Hint => (
            "Direct primary care membership, demographics, encounters, labs, and whole-chart notes.",
            match mode {
                PracticeSourceMode::SyntheticRehearsal => {
                    "Sign in to Hint Health in Chrome with the Coda Hint connector installed so Coda can verify whole-chart access, or request Coda-assisted onboarding."
                }
                PracticeSourceMode::LiveAssistedOnly => {
                    "Live Hint Health browser-connector verification is coordinated with Coda staff for real-time practices. Request Coda-assisted onboarding below."
                }
            },
            vec![
                "Chrome browser with the Coda Hint connector extension enabled.".to_owned(),
                "Active Hint Health practice session with whole-chart and full-note read access."
                    .to_owned(),
            ],
            methods,
        ),
        SupportedSourceWire::ElationFhir => (
            "EHR clinical chart records, problem list, medications, allergies, labs, and signed clinical notes.",
            match mode {
                PracticeSourceMode::SyntheticRehearsal => {
                    "Enter your practice's Elation / FHIR endpoint credentials authorized by your practice administrator, or request Coda-assisted setup."
                }
                PracticeSourceMode::LiveAssistedOnly => {
                    "Live Elation / FHIR R4 credential onboarding is coordinated with Coda staff for real-time practices. Request Coda-assisted setup below."
                }
            },
            vec![
                "Elation / FHIR R4 base URL and practice client credentials.".to_owned(),
                "Administrator approval for patient demographics, encounters, labs, and clinical notes.".to_owned(),
            ],
            methods,
        ),
        SupportedSourceWire::Spruce => (
            "Patient messaging threads, clinical photo and text attachments, and inbound faxes.",
            match mode {
                PracticeSourceMode::SyntheticRehearsal => {
                    "Connect your practice's Spruce Health API grant. New messages are read after connection; earlier message history is read only if you accept backlog review."
                }
                PracticeSourceMode::LiveAssistedOnly => {
                    "Live Spruce Health API grant onboarding is coordinated with Coda staff for real-time practices. Request Coda-assisted setup below."
                }
            },
            vec![
                "Spruce Health organization administrator API token and webhook grant.".to_owned(),
                "Agreement on whether to review pre-connection message backlog after connecting."
                    .to_owned(),
            ],
            methods,
        ),
    }
}

fn project_source(
    kind: SupportedSourceWire,
    mode: PracticeSourceMode,
    entry: &SourceEntry,
) -> PracticeSourceWire {
    let (summary, guidance, prerequisites, supported_methods) = source_metadata(kind, mode);
    PracticeSourceWire {
        source_id: kind,
        label: kind.label().to_owned(),
        summary: summary.to_owned(),
        guidance: guidance.to_owned(),
        prerequisites,
        supported_methods,
        standing: entry.standing.clone(),
        backlog: entry.backlog.clone(),
        unlinked_spruce_messages: entry.unlinked_spruce.len(),
        retained_linked_records: entry.linked_records.len(),
        retained_reply_drafts: entry.retained_drafts.len(),
    }
}

/// Builds the `/connections` routes attached to `site()`.
pub(crate) fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/connections", get(list_connections))
        .route("/connections/{source_id}/connect", post(connect_source))
        .route(
            "/connections/{source_id}/assist",
            post(request_assisted_setup),
        )
        .route("/connections/{source_id}/import", post(import_records))
        .route("/connections/{source_id}/refresh", post(refresh_source))
        .route(
            "/connections/{source_id}/accept-backlog",
            post(accept_backlog),
        )
        .route("/connections/{source_id}/reconnect", post(reconnect_source))
        .route(
            "/connections/{source_id}/disconnect",
            post(disconnect_source),
        )
        .route("/connections/{source_id}/simulate", post(simulate_event))
}

/// Reads structured Today connection notices for `practice_id` when a [`SourceSetupStore`] is present.
pub(crate) async fn today_connections(
    extensions: &Extensions,
    practice_id: &str,
) -> Result<Vec<TodayConnection>, SourcePersistError> {
    match store_from(extensions) {
        Some(store) => store.today_connections(practice_id).await,
        None => Ok(Vec::new()),
    }
}

/// Enriches a workspace response with retained source-setup clinical summary, per-source freshness,
/// and retained Spruce reply drafts so patient records remain readable across `ReadFailed`,
/// `AccessLost`, and Spruce `Disconnect`.
pub(crate) async fn enrich_workspace(
    enriched: &mut Value,
    workspace: &str,
    now_str: &str,
    extensions: &Extensions,
) -> Result<(), SourcePersistError> {
    let Some(setup) = store_from(extensions) else {
        return Ok(());
    };
    let identity = ConnectedCare::identity(extensions);
    let saved = setup.load_practice_map(&identity.practice_id).await?;
    if saved.is_empty() {
        return Ok(());
    }

    let mut summary_lines: Vec<String> = Vec::new();
    let mut extra_replies: Vec<Value> = Vec::new();

    for kind in SupportedSourceWire::ALL {
        let Some(entry) = saved.get(&kind) else {
            continue;
        };
        for rec in entry
            .linked_records
            .iter()
            .filter(|rec| rec.workspace == workspace)
        {
            summary_lines.push(rec.summary_sentence.clone());
        }

        if let Some(connected_sources) = enriched
            .get_mut("connected_sources")
            .and_then(Value::as_array_mut)
        {
            let state_json = match &entry.standing {
                SourceStandingWire::UpToDate { last_read_at, .. } => Some(json!({
                    "state": "usable",
                    "synchronized_at": last_read_at,
                })),
                SourceStandingWire::RecordsImported {
                    last_imported_at, ..
                } => Some(json!({
                    "state": "usable",
                    "synchronized_at": last_imported_at,
                })),
                SourceStandingWire::ReadFailed {
                    last_successful_read_at: Some(ts),
                    ..
                }
                | SourceStandingWire::AccessLost {
                    last_successful_read_at: Some(ts),
                    ..
                } => Some(json!({
                    "state": "failed",
                    "usable_snapshot": {
                        "synchronized_at": ts,
                    },
                })),
                SourceStandingWire::ReadFailed {
                    last_successful_read_at: None,
                    ..
                }
                | SourceStandingWire::AccessLost {
                    last_successful_read_at: None,
                    ..
                } => Some(json!({ "state": "failed" })),
                SourceStandingWire::Connected { .. } => Some(json!({ "state": "not_yet_pulled" })),
                SourceStandingWire::Disconnected
                | SourceStandingWire::CodaAssistedRequested { .. } => None,
            };
            if let Some(state_val) = state_json {
                let slug = kind.slug();
                if let Some(existing) = connected_sources
                    .iter_mut()
                    .find(|item| item.get("source_id").and_then(Value::as_str) == Some(slug))
                {
                    existing["state"] = state_val;
                } else {
                    connected_sources.push(json!({
                        "source_id": slug,
                        "source_label": kind.label(),
                        "state": state_val,
                    }));
                }
            }
        }

        for draft in entry
            .retained_drafts
            .iter()
            .filter(|draft| draft.workspace == workspace)
        {
            let display_name = enriched["display_name"]
                .as_str()
                .unwrap_or(workspace)
                .to_owned();
            let (text, revision) = if let (Some(store), Some(ctx)) =
                (extensions.get::<Store>(), extensions.get::<AuthContext>())
            {
                let scope = Scope {
                    practice: ctx.identity.practice_id.clone(),
                    physician: ctx.identity.physician_id.clone(),
                    workspace: workspace.to_owned(),
                    chat: draft.reply_work_id.clone(),
                };
                match store.draft(&scope).await {
                    Ok(Some(saved)) => (saved.text, saved.revision),
                    _ => (draft.draft_text.clone(), draft.revision),
                }
            } else {
                (draft.draft_text.clone(), draft.revision)
            };
            extra_replies.push(json!({
                "reply_work_id": draft.reply_work_id,
                "patient_workspace_id": workspace,
                "patient_display_name": display_name,
                "channel": "spruce",
                "headline": draft.headline,
                "draft_text": text,
                "urgency": "routine",
                "created_at": draft.created_at,
                "state": "ready",
                "draft_revision": revision,
            }));
        }
    }

    if !summary_lines.is_empty() && enriched["patient_synopsis"].is_null() {
        let joined = summary_lines.join(" ");
        enriched["patient_synopsis"] = json!({
            "version_id": "source-setup-v1",
            "authored_text": joined,
            "segments": [
                {
                    "kind": "text",
                    "text": joined,
                }
            ],
            "authored_at": now_str,
        });
    }

    if !extra_replies.is_empty() {
        match enriched.get_mut("replies").and_then(Value::as_array_mut) {
            Some(existing) => {
                for reply in extra_replies {
                    let id = reply["reply_work_id"].as_str().unwrap_or_default();
                    if !existing
                        .iter()
                        .any(|item| item["reply_work_id"].as_str() == Some(id))
                    {
                        existing.push(reply);
                    }
                }
            }
            None => {
                enriched["replies"] = Value::Array(extra_replies);
            }
        }
    }

    Ok(())
}

async fn list_connections(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    let Some(setup) = store_from(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let Ok(mode) =
        resolve_practice_mode(&connected, &setup, extensions, &identity.practice_id).await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    match setup.list_practice(&identity.practice_id, mode).await {
        Ok(list) => Json(list).into_response(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
enum SourceInputError {
    #[error("unknown clinical source")]
    UnknownSource,
    #[error("Select a supported connection method for this source.")]
    UnsupportedMethod,
    #[error(
        "Live vendor verification is not attached on this server; request Coda-assisted setup."
    )]
    LiveConnectorUnavailable,
    #[error("Enter the practice account, connector session, or endpoint identifier.")]
    MissingAccountLabel,
    #[error("Enter the credential secret or connector verification token.")]
    MissingSecret,
    #[error("Confirm the Hint Chrome connector prerequisites before connecting.")]
    PrerequisitesUnconfirmed,
}

impl SourceInputError {
    const fn code(self) -> &'static str {
        match self {
            Self::UnknownSource => "unknown_source",
            Self::UnsupportedMethod => "unsupported_method",
            Self::LiveConnectorUnavailable => "live_connector_unavailable",
            Self::MissingAccountLabel => "missing_account_label",
            Self::MissingSecret => "missing_secret",
            Self::PrerequisitesUnconfirmed => "prerequisites_unconfirmed",
        }
    }
}

impl IntoResponse for SourceInputError {
    fn into_response(self) -> Response {
        match self {
            Self::UnknownSource => StatusCode::NOT_FOUND.into_response(),
            Self::LiveConnectorUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "detail": self.to_string(),
                    "code": self.code(),
                })),
            )
                .into_response(),
            other => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": other.to_string(),
                    "code": other.code(),
                })),
            )
                .into_response(),
        }
    }
}

fn parse_source(slug: &str) -> Result<SupportedSourceWire, SourceInputError> {
    SupportedSourceWire::from_slug(slug).ok_or(SourceInputError::UnknownSource)
}

fn validate_connect_body(
    kind: SupportedSourceWire,
    mode: PracticeSourceMode,
    body: &ConnectSourceBody,
) -> Result<(SourceConnectionMethodWire, String), SourceInputError> {
    if body.method == SourceConnectionMethodWire::CodaAssisted {
        return Err(SourceInputError::UnsupportedMethod);
    }
    if mode == PracticeSourceMode::LiveAssistedOnly {
        if body.method == SourceConnectionMethodWire::SimulatedPractice {
            return Err(SourceInputError::UnsupportedMethod);
        }
        return Err(SourceInputError::LiveConnectorUnavailable);
    }
    let (_, _, _, supported) = source_metadata(kind, mode);
    if !supported.contains(&body.method) {
        return Err(SourceInputError::UnsupportedMethod);
    }
    let label = body.account_label.trim().to_owned();
    if label.is_empty() {
        return Err(SourceInputError::MissingAccountLabel);
    }
    if matches!(
        body.method,
        SourceConnectionMethodWire::CredentialSignIn | SourceConnectionMethodWire::BrowserSession
    ) {
        let secret = body
            .secret_or_code
            .as_deref()
            .map(str::trim)
            .unwrap_or_default();
        if secret.is_empty() {
            return Err(SourceInputError::MissingSecret);
        }
    }
    if kind == SupportedSourceWire::Hint
        && body.method == SourceConnectionMethodWire::BrowserSession
        && !body.prerequisites_confirmed
    {
        return Err(SourceInputError::PrerequisitesUnconfirmed);
    }
    Ok((body.method, label))
}

async fn connect_source(
    Path(source_slug): Path<String>,
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let kind = match parse_source(&source_slug) {
        Ok(kind) => kind,
        Err(err) => return err.into_response(),
    };
    let (parts, body) = request.into_parts();
    let Some(setup) = store_from(&parts.extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Ok(bytes) = axum::body::to_bytes(body, BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<ConnectSourceBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let identity = ConnectedCare::identity(&parts.extensions);
    let Ok(mode) =
        resolve_practice_mode(&connected, &setup, &parts.extensions, &identity.practice_id).await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let (method, account_label) = match validate_connect_body(kind, mode, &payload) {
        Ok(valid) => valid,
        Err(err) => return err.into_response(),
    };
    let Ok(moment) = connected.practice_moment(&parts.extensions).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let now_str = moment.now_rfc3339;
    let updated = setup
        .mutate_entry(
            &identity.practice_id,
            kind,
            moment.now_unix as i64,
            |entry| -> Result<(), std::convert::Infallible> {
                entry.standing = SourceStandingWire::Connected {
                    connected_at: now_str,
                    method,
                    account_label,
                };
                entry.activity = SpruceBackgroundActivity::Idle;
                if kind == SupportedSourceWire::Spruce {
                    entry.backlog = SpruceBacklogWire::NonePending;
                }
                Ok(())
            },
        )
        .await;
    let Ok((entry, ())) = updated else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    notify_practice(&parts.extensions).await;
    Json(SourceOperationResponseWire {
        practice_id: identity.practice_id,
        source: project_source(kind, mode, &entry),
    })
    .into_response()
}

async fn request_assisted_setup(
    Path(source_slug): Path<String>,
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let kind = match parse_source(&source_slug) {
        Ok(kind) => kind,
        Err(err) => return err.into_response(),
    };
    let (parts, body) = request.into_parts();
    let Some(setup) = store_from(&parts.extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Ok(bytes) = axum::body::to_bytes(body, BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<RequestAssistedSetupBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let contact_email = payload.contact_email.trim().to_lowercase();
    if !contact_email.contains('@') || contact_email.starts_with('@') {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "detail": "Enter a valid practice contact email for Coda-assisted setup.",
                "code": "invalid_contact_email",
            })),
        )
            .into_response();
    }
    let notes = payload
        .notes
        .as_deref()
        .map(str::trim)
        .filter(|notes| !notes.is_empty())
        .map(str::to_owned);
    let identity = ConnectedCare::identity(&parts.extensions);
    let Ok(mode) =
        resolve_practice_mode(&connected, &setup, &parts.extensions, &identity.practice_id).await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Ok(moment) = connected.practice_moment(&parts.extensions).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let now_str = moment.now_rfc3339;
    let updated = setup
        .mutate_entry(
            &identity.practice_id,
            kind,
            moment.now_unix as i64,
            |entry| -> Result<(), std::convert::Infallible> {
                entry.standing = SourceStandingWire::CodaAssistedRequested {
                    requested_at: now_str,
                    contact_email,
                    notes,
                };
                Ok(())
            },
        )
        .await;
    let Ok((entry, ())) = updated else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    Json(SourceOperationResponseWire {
        practice_id: identity.practice_id,
        source: project_source(kind, mode, &entry),
    })
    .into_response()
}

async fn import_records(
    Path(source_slug): Path<String>,
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let kind = match parse_source(&source_slug) {
        Ok(kind) => kind,
        Err(err) => return err.into_response(),
    };
    let extensions = request.extensions();
    let Some(setup) = store_from(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let Ok(mode) =
        resolve_practice_mode(&connected, &setup, extensions, &identity.practice_id).await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    if mode == PracticeSourceMode::LiveAssistedOnly {
        return SourceInputError::LiveConnectorUnavailable.into_response();
    }
    let Ok(moment) = connected.practice_moment(extensions).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let now_str = moment.now_rfc3339;
    let pulled_unix = moment.now_unix;

    let Ok(current) = setup.read_entry(&identity.practice_id, kind).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let (connected_at, method, account_label, next_generation) = match &current.standing {
        SourceStandingWire::Connected {
            connected_at,
            method,
            account_label,
        }
        | SourceStandingWire::RecordsImported {
            connected_at,
            method,
            account_label,
            ..
        } => (
            connected_at.clone(),
            *method,
            account_label.clone(),
            current.import_generation + 1,
        ),
        _ => {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "detail": "Connect and verify this source before importing initial records.",
                    "code": "source_not_connected",
                })),
            )
                .into_response();
        }
    };

    let auth_store = connected.auth_store(extensions);
    let Ok(practice_patients) = auth_store.practice_patients(&identity.practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let target_workspace = practice_patients
        .first()
        .cloned()
        .unwrap_or_else(|| format!("pw-{}-primary", identity.practice_id));

    let linked_records = build_initial_linked_records(kind, &target_workspace, next_generation);
    if !practice_patients.is_empty()
        && persist_source_delivery(
            extensions,
            &connected,
            SourceDelivery {
                workspace: &target_workspace,
                kind,
                linked_records: &linked_records,
                pulled_unix,
                now_str: &now_str,
            },
        )
        .await
        .is_err()
    {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    let has_patients = !practice_patients.is_empty();
    let updated = setup
        .mutate_entry(
            &identity.practice_id,
            kind,
            pulled_unix as i64,
            |entry| -> Result<(), StatusCode> {
                if !matches!(
                    entry.standing,
                    SourceStandingWire::Connected { .. }
                        | SourceStandingWire::RecordsImported { .. }
                ) {
                    return Err(StatusCode::CONFLICT);
                }
                entry.import_generation += 1;
                let generation = entry.import_generation;
                for rec in linked_records {
                    if !entry
                        .linked_records
                        .iter()
                        .any(|existing| existing.candidate_id == rec.candidate_id)
                    {
                        entry.linked_records.push(rec);
                    }
                }
                if kind == SupportedSourceWire::Spruce {
                    entry.unlinked_spruce = vec![
                        UnlinkedSpruceMessage {
                            message_id: format!("spruce-unlinked-1-g{generation}"),
                            sender_handle: "+15550199011".to_owned(),
                        },
                        UnlinkedSpruceMessage {
                            message_id: format!("spruce-unlinked-2-g{generation}"),
                            sender_handle: "+15550199012".to_owned(),
                        },
                    ];
                    if !matches!(entry.backlog, SpruceBacklogWire::Accepted { .. }) {
                        let earliest = format_rfc3339(
                            UNIX_EPOCH
                                + Duration::from_secs(pulled_unix.saturating_sub(5 * 86_400)),
                        );
                        entry.backlog = SpruceBacklogWire::Offered {
                            message_count: 3,
                            earliest_message_at: earliest,
                        };
                    }
                    if has_patients
                        && !entry
                            .retained_drafts
                            .iter()
                            .any(|draft| draft.workspace == target_workspace)
                    {
                        entry.retained_drafts.push(RetainedReplyDraft {
                            workspace: target_workspace.clone(),
                            reply_work_id: SPRUCE_REPLY_ID.to_owned(),
                            headline: SPRUCE_REPLY_HEADLINE.to_owned(),
                            draft_text: SPRUCE_REPLY_TEXT.to_owned(),
                            created_at: now_str.clone(),
                            revision: 1,
                        });
                    }
                }
                let imported_records = entry.linked_records.len();
                entry.standing = SourceStandingWire::RecordsImported {
                    connected_at,
                    method,
                    account_label,
                    imported_records,
                    last_imported_at: now_str,
                };
                Ok(())
            },
        )
        .await;

    let entry = match updated {
        Ok((entry, ())) => entry,
        Err(MutateError::Domain(status)) => return status.into_response(),
        Err(MutateError::Persist(error)) => return persist_failure(error),
    };

    notify_practice(extensions).await;
    Json(SourceOperationResponseWire {
        practice_id: identity.practice_id,
        source: project_source(kind, mode, &entry),
    })
    .into_response()
}

async fn refresh_source(
    Path(source_slug): Path<String>,
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let kind = match parse_source(&source_slug) {
        Ok(kind) => kind,
        Err(err) => return err.into_response(),
    };
    let extensions = request.extensions();
    let Some(setup) = store_from(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let Ok(mode) =
        resolve_practice_mode(&connected, &setup, extensions, &identity.practice_id).await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    if mode == PracticeSourceMode::LiveAssistedOnly {
        return SourceInputError::LiveConnectorUnavailable.into_response();
    }
    let Ok(moment) = connected.practice_moment(extensions).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let now_str = moment.now_rfc3339;
    let pulled_unix = moment.now_unix;

    let Ok(current) = setup.read_entry(&identity.practice_id, kind).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let (connected_at, method, account_label, linked_snapshot) = match &current.standing {
        SourceStandingWire::RecordsImported {
            connected_at,
            method,
            account_label,
            ..
        }
        | SourceStandingWire::UpToDate {
            connected_at,
            method,
            account_label,
            ..
        }
        | SourceStandingWire::ReadFailed {
            connected_at,
            method,
            account_label,
            ..
        } => (
            connected_at.clone(),
            *method,
            account_label.clone(),
            current.linked_records.clone(),
        ),
        SourceStandingWire::AccessLost { .. } => {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "detail": "Access was lost; reconnect this source before refreshing records.",
                    "code": "reconnect_required",
                })),
            )
                .into_response();
        }
        _ => {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "detail": "Import initial records before refreshing this source.",
                    "code": "initial_import_required",
                })),
            )
                .into_response();
        }
    };

    let auth_store = connected.auth_store(extensions);
    let Ok(practice_patients) = auth_store.practice_patients(&identity.practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    if let Some(target_workspace) = practice_patients.first()
        && persist_source_delivery(
            extensions,
            &connected,
            SourceDelivery {
                workspace: target_workspace,
                kind,
                linked_records: &linked_snapshot,
                pulled_unix,
                now_str: &now_str,
            },
        )
        .await
        .is_err()
    {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    let updated = setup
        .mutate_entry(
            &identity.practice_id,
            kind,
            pulled_unix as i64,
            |entry| -> Result<(), StatusCode> {
                entry.activity = SpruceBackgroundActivity::Idle;
                let imported_records = entry.linked_records.len();
                entry.standing = SourceStandingWire::UpToDate {
                    connected_at,
                    method,
                    account_label,
                    imported_records,
                    last_read_at: now_str,
                };
                Ok(())
            },
        )
        .await;
    let Ok((entry, ())) = updated else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    notify_practice(extensions).await;
    Json(SourceOperationResponseWire {
        practice_id: identity.practice_id,
        source: project_source(kind, mode, &entry),
    })
    .into_response()
}

async fn accept_backlog(
    Path(source_slug): Path<String>,
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let kind = match parse_source(&source_slug) {
        Ok(kind) => kind,
        Err(err) => return err.into_response(),
    };
    if kind != SupportedSourceWire::Spruce {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": "Only Spruce connections hold a pre-connection message backlog.",
                "code": "backlog_not_applicable",
            })),
        )
            .into_response();
    }
    let extensions = request.extensions();
    let Some(setup) = store_from(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let Ok(mode) =
        resolve_practice_mode(&connected, &setup, extensions, &identity.practice_id).await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    if mode == PracticeSourceMode::LiveAssistedOnly {
        return SourceInputError::LiveConnectorUnavailable.into_response();
    }
    let Ok(moment) = connected.practice_moment(extensions).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let now_str = moment.now_rfc3339;
    let pulled_unix = moment.now_unix;

    let auth_store = connected.auth_store(extensions);
    let Ok(practice_patients) = auth_store.practice_patients(&identity.practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let target_workspace = practice_patients
        .first()
        .cloned()
        .unwrap_or_else(|| format!("pw-{}-primary", identity.practice_id));

    let Ok(current) = setup.read_entry(&identity.practice_id, kind).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let SpruceBacklogWire::Offered { message_count, .. } = current.backlog else {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": "No pending Spruce message backlog is awaiting acceptance.",
                "code": "no_pending_backlog",
            })),
        )
            .into_response();
    };

    let mut preview_records = current.linked_records.clone();
    for idx in 1..=message_count {
        let candidate_id = format!("spruce-backlog-msg-{idx}");
        if !preview_records
            .iter()
            .any(|existing| existing.candidate_id == candidate_id)
        {
            preview_records.push(LinkedSourceRecord {
                workspace: target_workspace.clone(),
                candidate_id,
                summary_sentence: format!(
                    "Accepted historical Spruce message #{idx} from prior clinic thread."
                ),
            });
        }
    }

    if !practice_patients.is_empty()
        && persist_source_delivery(
            extensions,
            &connected,
            SourceDelivery {
                workspace: &target_workspace,
                kind,
                linked_records: &preview_records,
                pulled_unix,
                now_str: &now_str,
            },
        )
        .await
        .is_err()
    {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    let updated = setup
        .mutate_entry(
            &identity.practice_id,
            kind,
            pulled_unix as i64,
            |entry| -> Result<(), StatusCode> {
                let SpruceBacklogWire::Offered { message_count, .. } = entry.backlog else {
                    return Err(StatusCode::CONFLICT);
                };
                for idx in 1..=message_count {
                    let candidate_id = format!("spruce-backlog-msg-{idx}");
                    if !entry
                        .linked_records
                        .iter()
                        .any(|existing| existing.candidate_id == candidate_id)
                    {
                        entry.linked_records.push(LinkedSourceRecord {
                            workspace: target_workspace.clone(),
                            candidate_id,
                            summary_sentence: format!(
                                "Accepted historical Spruce message #{idx} from prior clinic thread."
                            ),
                        });
                    }
                }
                entry.backlog = SpruceBacklogWire::Accepted {
                    accepted_at: now_str.clone(),
                    imported_message_count: message_count,
                };
                let total = entry.linked_records.len();
                match &mut entry.standing {
                    SourceStandingWire::RecordsImported {
                        imported_records, ..
                    }
                    | SourceStandingWire::UpToDate {
                        imported_records, ..
                    }
                    | SourceStandingWire::ReadFailed {
                        imported_records, ..
                    }
                    | SourceStandingWire::AccessLost {
                        imported_records, ..
                    } => {
                        *imported_records = total;
                    }
                    SourceStandingWire::Disconnected
                    | SourceStandingWire::CodaAssistedRequested { .. }
                    | SourceStandingWire::Connected { .. } => {}
                }
                Ok(())
            },
        )
        .await;

    let entry = match updated {
        Ok((entry, ())) => entry,
        Err(MutateError::Domain(_)) => {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "detail": "No pending Spruce message backlog is awaiting acceptance.",
                    "code": "no_pending_backlog",
                })),
            )
                .into_response();
        }
        Err(MutateError::Persist(error)) => return persist_failure(error),
    };

    notify_practice(extensions).await;
    Json(SourceOperationResponseWire {
        practice_id: identity.practice_id,
        source: project_source(kind, mode, &entry),
    })
    .into_response()
}

async fn reconnect_source(
    Path(source_slug): Path<String>,
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let kind = match parse_source(&source_slug) {
        Ok(kind) => kind,
        Err(err) => return err.into_response(),
    };
    let (parts, body) = request.into_parts();
    let Some(setup) = store_from(&parts.extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Ok(bytes) = axum::body::to_bytes(body, BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<ConnectSourceBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let identity = ConnectedCare::identity(&parts.extensions);
    let Ok(mode) =
        resolve_practice_mode(&connected, &setup, &parts.extensions, &identity.practice_id).await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let (method, account_label) = match validate_connect_body(kind, mode, &payload) {
        Ok(valid) => valid,
        Err(err) => return err.into_response(),
    };
    let Ok(moment) = connected.practice_moment(&parts.extensions).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let now_str = moment.now_rfc3339;
    let pulled_unix = moment.now_unix;

    let Ok(current) = setup.read_entry(&identity.practice_id, kind).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    if !current.standing.requires_reconnect() {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": "Reconnect applies only when authorization was revoked or blocked.",
                "code": "reconnect_not_required",
            })),
        )
            .into_response();
    }

    let auth_store = connected.auth_store(&parts.extensions);
    let Ok(practice_patients) = auth_store.practice_patients(&identity.practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    if let Some(target_workspace) = practice_patients.first()
        && !current.linked_records.is_empty()
        && persist_source_delivery(
            &parts.extensions,
            &connected,
            SourceDelivery {
                workspace: target_workspace,
                kind,
                linked_records: &current.linked_records,
                pulled_unix,
                now_str: &now_str,
            },
        )
        .await
        .is_err()
    {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    let updated = setup
        .mutate_entry(
            &identity.practice_id,
            kind,
            pulled_unix as i64,
            |entry| -> Result<(), StatusCode> {
                if !entry.standing.requires_reconnect() {
                    return Err(StatusCode::CONFLICT);
                }
                let imported_records = entry.linked_records.len();
                entry.activity = SpruceBackgroundActivity::Idle;
                entry.standing = if imported_records > 0 {
                    SourceStandingWire::UpToDate {
                        connected_at: now_str.clone(),
                        method,
                        account_label,
                        imported_records,
                        last_read_at: now_str,
                    }
                } else {
                    SourceStandingWire::Connected {
                        connected_at: now_str,
                        method,
                        account_label,
                    }
                };
                Ok(())
            },
        )
        .await;

    let entry = match updated {
        Ok((entry, ())) => entry,
        Err(MutateError::Domain(status)) => return status.into_response(),
        Err(MutateError::Persist(error)) => return persist_failure(error),
    };

    notify_practice(&parts.extensions).await;
    Json(SourceOperationResponseWire {
        practice_id: identity.practice_id,
        source: project_source(kind, mode, &entry),
    })
    .into_response()
}

async fn disconnect_source(
    Path(source_slug): Path<String>,
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let kind = match parse_source(&source_slug) {
        Ok(kind) => kind,
        Err(err) => return err.into_response(),
    };
    let extensions = request.extensions();
    let Some(setup) = store_from(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let Ok(mode) =
        resolve_practice_mode(&connected, &setup, extensions, &identity.practice_id).await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Ok(moment) = connected.practice_moment(extensions).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let updated = setup
        .mutate_entry(
            &identity.practice_id,
            kind,
            moment.now_unix as i64,
            |entry| -> Result<(), std::convert::Infallible> {
                entry.standing = SourceStandingWire::Disconnected;
                entry.activity = SpruceBackgroundActivity::Idle;
                if kind == SupportedSourceWire::Spruce {
                    // Rule 11: purge all never-linked inbound Spruce messages while keeping
                    // accepted/linked patient chart records and physician reply drafts intact.
                    entry.unlinked_spruce.clear();
                    entry.backlog = SpruceBacklogWire::NonePending;
                }
                Ok(())
            },
        )
        .await;
    let Ok((entry, ())) = updated else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    notify_practice(extensions).await;
    Json(SourceOperationResponseWire {
        practice_id: identity.practice_id,
        source: project_source(kind, mode, &entry),
    })
    .into_response()
}

async fn simulate_event(
    Path(source_slug): Path<String>,
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let kind = match parse_source(&source_slug) {
        Ok(kind) => kind,
        Err(err) => return err.into_response(),
    };
    let (parts, body) = request.into_parts();
    let Some(setup) = store_from(&parts.extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let identity = ConnectedCare::identity(&parts.extensions);
    let Ok(mode) =
        resolve_practice_mode(&connected, &setup, &parts.extensions, &identity.practice_id).await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    // Principle 12: synthetic fault-injection simulation is unmounted (404) outside synthetic practices.
    if mode == PracticeSourceMode::LiveAssistedOnly {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(event) = serde_json::from_slice::<SimulateSourceEventBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(moment) = connected.practice_moment(&parts.extensions).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let now_str = moment.now_rfc3339;
    let updated = setup
        .mutate_entry(
            &identity.practice_id,
            kind,
            moment.now_unix as i64,
            |entry| -> Result<(), StatusCode> {
                match event {
                    SimulateSourceEventBody::QuietCatchUp => {
                        entry.activity = SpruceBackgroundActivity::CatchingUp;
                    }
                    SimulateSourceEventBody::QuietTransientRetry { attempt } => {
                        entry.activity = SpruceBackgroundActivity::RetryingTransient { attempt };
                    }
                    SimulateSourceEventBody::ClearQuietActivity => {
                        entry.activity = SpruceBackgroundActivity::Idle;
                    }
                    SimulateSourceEventBody::TransientReadFailure { reason } => {
                        let Some((connected_at, method, account_label, imported, last_ok)) =
                            connection_anchor(&entry.standing)
                        else {
                            return Err(StatusCode::CONFLICT);
                        };
                        entry.activity = SpruceBackgroundActivity::Idle;
                        entry.standing = SourceStandingWire::ReadFailed {
                            connected_at,
                            method,
                            account_label,
                            imported_records: imported,
                            last_successful_read_at: last_ok,
                            failed_at: now_str,
                            reason,
                        };
                    }
                    SimulateSourceEventBody::RevokedGrant { reason } => {
                        let Some((connected_at, method, account_label, imported, last_ok)) =
                            connection_anchor(&entry.standing)
                        else {
                            return Err(StatusCode::CONFLICT);
                        };
                        entry.activity = SpruceBackgroundActivity::Idle;
                        entry.standing = SourceStandingWire::AccessLost {
                            connected_at,
                            method,
                            account_label,
                            imported_records: imported,
                            last_successful_read_at: last_ok,
                            lost_at: now_str,
                            cause: AccessLossCauseWire::RevokedGrant,
                            reason,
                        };
                    }
                    SimulateSourceEventBody::AdministratorBlock { reason } => {
                        let Some((connected_at, method, account_label, imported, last_ok)) =
                            connection_anchor(&entry.standing)
                        else {
                            return Err(StatusCode::CONFLICT);
                        };
                        entry.activity = SpruceBackgroundActivity::Idle;
                        entry.standing = SourceStandingWire::AccessLost {
                            connected_at,
                            method,
                            account_label,
                            imported_records: imported,
                            last_successful_read_at: last_ok,
                            lost_at: now_str,
                            cause: AccessLossCauseWire::AdministratorBlock,
                            reason,
                        };
                    }
                }
                Ok(())
            },
        )
        .await;

    let entry = match updated {
        Ok((entry, ())) => entry,
        Err(MutateError::Domain(status)) => return status.into_response(),
        Err(MutateError::Persist(error)) => return persist_failure(error),
    };
    notify_practice(&parts.extensions).await;
    Json(SourceOperationResponseWire {
        practice_id: identity.practice_id,
        source: project_source(kind, mode, &entry),
    })
    .into_response()
}

fn connection_anchor(
    standing: &SourceStandingWire,
) -> Option<(
    String,
    SourceConnectionMethodWire,
    String,
    usize,
    Option<String>,
)> {
    match standing {
        SourceStandingWire::Connected {
            connected_at,
            method,
            account_label,
        } => Some((
            connected_at.clone(),
            *method,
            account_label.clone(),
            0,
            None,
        )),
        SourceStandingWire::RecordsImported {
            connected_at,
            method,
            account_label,
            imported_records,
            last_imported_at,
        } => Some((
            connected_at.clone(),
            *method,
            account_label.clone(),
            *imported_records,
            Some(last_imported_at.clone()),
        )),
        SourceStandingWire::UpToDate {
            connected_at,
            method,
            account_label,
            imported_records,
            last_read_at,
        } => Some((
            connected_at.clone(),
            *method,
            account_label.clone(),
            *imported_records,
            Some(last_read_at.clone()),
        )),
        SourceStandingWire::ReadFailed {
            connected_at,
            method,
            account_label,
            imported_records,
            last_successful_read_at,
            ..
        }
        | SourceStandingWire::AccessLost {
            connected_at,
            method,
            account_label,
            imported_records,
            last_successful_read_at,
            ..
        } => Some((
            connected_at.clone(),
            *method,
            account_label.clone(),
            *imported_records,
            last_successful_read_at.clone(),
        )),
        SourceStandingWire::Disconnected | SourceStandingWire::CodaAssistedRequested { .. } => None,
    }
}

fn build_initial_linked_records(
    kind: SupportedSourceWire,
    workspace: &str,
    generation: u64,
) -> Vec<LinkedSourceRecord> {
    let _ = generation;
    match kind {
        SupportedSourceWire::Hint => vec![
            LinkedSourceRecord {
                workspace: workspace.to_owned(),
                candidate_id: "hint-membership-summary".to_owned(),
                summary_sentence:
                    "Hint Health whole-chart membership and primary care note imported.".to_owned(),
            },
            LinkedSourceRecord {
                workspace: workspace.to_owned(),
                candidate_id: "hint-clinical-note-latest".to_owned(),
                summary_sentence: "Hint Health latest office visit note and problem list verified."
                    .to_owned(),
            },
        ],
        SupportedSourceWire::ElationFhir => vec![
            LinkedSourceRecord {
                workspace: workspace.to_owned(),
                candidate_id: "elation-fhir-medications".to_owned(),
                summary_sentence:
                    "Elation / FHIR active medication list and laboratory panel imported."
                        .to_owned(),
            },
            LinkedSourceRecord {
                workspace: workspace.to_owned(),
                candidate_id: "elation-fhir-encounter".to_owned(),
                summary_sentence: "Elation / FHIR signed encounter note retained.".to_owned(),
            },
        ],
        SupportedSourceWire::Spruce => vec![
            LinkedSourceRecord {
                workspace: workspace.to_owned(),
                candidate_id: "spruce-post-connect-msg-1".to_owned(),
                summary_sentence:
                    "Spruce post-connection patient blood pressure log message linked to chart."
                        .to_owned(),
            },
            LinkedSourceRecord {
                workspace: workspace.to_owned(),
                candidate_id: "spruce-post-connect-msg-2".to_owned(),
                summary_sentence: "Spruce post-connection home cuff photo note linked to chart."
                    .to_owned(),
            },
        ],
    }
}

struct SourceDelivery<'a> {
    workspace: &'a str,
    kind: SupportedSourceWire,
    linked_records: &'a [LinkedSourceRecord],
    pulled_unix: u64,
    now_str: &'a str,
}

async fn persist_source_delivery(
    extensions: &Extensions,
    connected: &ConnectedCare,
    delivery: SourceDelivery<'_>,
) -> Result<(), SourcePersistError> {
    let identity = ConnectedCare::identity(extensions);
    let workspace = delivery.workspace;
    let kind = delivery.kind;
    let patient_identity = existing_source_patient(extensions, connected, workspace).await;
    let pull = SourcePull {
        source_id: kind.slug().to_owned(),
        source_label: kind.label().to_owned(),
        patient: patient_identity,
        pulled_at: delivery.pulled_unix as i64,
    };

    if let Some(store) = extensions.get::<Store>() {
        let revisions: Vec<SourceRevision> = delivery
            .linked_records
            .iter()
            .filter(|rec| rec.workspace == workspace)
            .map(|rec| {
                let content = json!({
                    "source": kind.slug(),
                    "candidate_id": rec.candidate_id,
                    "summary": rec.summary_sentence,
                })
                .to_string();
                let sha256 = local_product::sha256_hex(content.as_bytes());
                SourceRevision {
                    workspace: workspace.to_owned(),
                    source_id: kind.slug().to_owned(),
                    candidate_id: rec.candidate_id.clone(),
                    revision: format!("rev-{}", &sha256[..12]),
                    record_type: "source_record".to_owned(),
                    dates: vec![RecordDate {
                        name: "recorded_at".to_owned(),
                        value: delivery.now_str.to_owned(),
                    }],
                    content,
                    sha256,
                    readable_parts: vec![ReadablePart {
                        label: kind.label().to_owned(),
                        text: rec.summary_sentence.clone(),
                    }],
                    provenance: json!({
                        "source_id": kind.slug(),
                        "native_id": rec.candidate_id,
                    }),
                }
            })
            .collect();
        store.record_chart(workspace, &revisions, &pull).await?;

        if kind == SupportedSourceWire::Spruce
            && let Some(ctx) = extensions.get::<AuthContext>()
            && ctx.auth.store.membership(&identity, workspace).await == Membership::Allowed
        {
            let scope = Scope {
                practice: identity.practice_id,
                physician: identity.physician_id,
                workspace: workspace.to_owned(),
                chat: SPRUCE_REPLY_ID.to_owned(),
            };
            if store.draft(&scope).await?.is_none() {
                let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
                let mut authority = SourceDraftAuthority {
                    auth: ctx.auth.store.as_ref(),
                    token: &ctx.token,
                    now_unix: auth::now(),
                };
                store
                    .open_draft_authorized(&scope, SPRUCE_REPLY_TEXT, &hash, &mut authority)
                    .await?;
            }
        }
    } else if let Some(memory) = extensions.get::<Arc<MemoryPatients>>() {
        memory.add_source_record(
            workspace,
            PatientSourceRecord {
                source_id: pull.source_id,
                source_label: pull.source_label,
                patient: pull.patient,
                delivery: SourceDeliveryState::Usable {
                    pulled_at: pull.pulled_at,
                },
            },
        );
    }
    let _ = care::on_roster;
    Ok(())
}

async fn existing_source_patient(
    extensions: &Extensions,
    connected: &ConnectedCare,
    workspace: &str,
) -> SourcePatient {
    let directory = connected.patients(extensions);
    if let Ok(mut pulls) = directory.pulls(&[workspace.to_owned()]).await
        && let Some(first) = pulls
            .remove(workspace)
            .and_then(|mut list| (!list.is_empty()).then(|| list.remove(0)))
    {
        return first.patient;
    }
    SourcePatient {
        source_patient_id: workspace.to_owned(),
        display_name: "Practice Patient".to_owned(),
        birth_date: Some("1982-04-14".to_owned()),
        sex_or_gender: Some("female".to_owned()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "Practice MRN".to_owned(),
            value: format!("MRN-{}", &workspace[..workspace.len().min(8)]),
        }],
    }
}

struct SourceDraftAuthority<'a> {
    auth: &'a dyn AuthStore,
    token: &'a str,
    now_unix: u64,
}

#[async_trait]
impl Authority for SourceDraftAuthority<'_> {
    async fn check(
        &mut self,
        _tx: &PublicationTransaction,
        scope: &Scope,
        _session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        let resolved = self
            .auth
            .identity(self.token, self.now_unix)
            .await
            .map_err(|err| format!("session invalid: {err:?}"))?;
        if resolved.practice_id != scope.practice || resolved.physician_id != scope.physician {
            return Err("session scope mismatch".to_owned());
        }
        if self.auth.membership(&resolved, &scope.workspace).await != Membership::Allowed {
            return Err("workspace not on physician panel".to_owned());
        }
        Ok(())
    }

    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.check(tx, scope, session_hash).await
    }
}

async fn notify_practice(extensions: &Extensions) {
    if let Some(ctx) = extensions.get::<AuthContext>() {
        let _ = ctx.auth.events.publish(physician_bus::Event::Resync).await;
    }
}
