//! Duet AI foundations wired to `crates/harness` and `crates/clinical-library`,
//! 30-day rolling private memory version retention, standing AI disclosure, separate
//! opt-in research consent, and parked-route quarantine policy.
//!
//! Spec: `intent/duet.md` (AI permission and Terms, What leaves Duet, Consent and data,
//! Scope) and `intent/harness.md`.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::{get, put};
use axum::{Json, Router};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use clinical_library::corpus::{DEFAULT_SEARCH_LIMIT, ReadItemOutcome, SearchResult};
use clinical_library::freshness::LibraryFreshnessReport;
use harness::PromptCacheKey;
use harness::engine::recipe::{
    Finish, Provider, Request, RequestBinding, RequestRecipe, ToolFuture,
};
use harness::engine::recovery::{Inputs, Messages, Records};
use harness::engine::runs::{HoldReason, RunHistory, RunOutcome};
use harness::engine::{
    Cancellation, Event, EventKind, Failure, Invocation, Item, LimitBackstop, NotCurrent, Product,
    ProductError, ProductMessage, RunLimits, ToolOutcome, Turn, WindowSeed,
};
use sea_orm::sea_query::{Expr, OnConflict, Query};
use sea_orm::{
    ColumnTrait, EntityName, EntityTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::ApiError;
use crate::assistant_entity::{
    ai_preference, harness_run, memory_version, private_memory, user_gate,
};
use crate::database::{AccountDatabase, orm};
use crate::durable::DurableState;
use crate::identity::{AccessLevel, AccountContext, check_current};

/// Exact fixed standing notice required whenever Duet's AI is surfaced (`intent/duet.md`).
pub const STANDING_AI_DISCLOSURE: &str = "Duet's AI isn't a doctor. It explains health information and doesn't diagnose or treat. In an emergency, call 911.";

/// Retention window for superseded versions of Duet's private patient memory (`intent/duet.md`).
pub const MEMORY_RETENTION_DAYS: i64 = 30;

/// Environment variable that explicitly enables parked legacy routes when set to `1` or `true`.
pub const PARKED_ROUTES_ENV: &str = "DUET_ENABLE_PARKED_ROUTES";

/// Tool name exposed to the harness for clinical library search.
pub const TOOL_CLINICAL_LIBRARY_SEARCH: &str = "clinical_library_search";
/// Tool name exposed to the harness for exact clinical library readback.
pub const TOOL_CLINICAL_LIBRARY_READ: &str = "clinical_library_read";
/// Tool name exposed to the harness for clinical library freshness inspection.
pub const TOOL_CLINICAL_LIBRARY_FRESHNESS: &str = "clinical_library_freshness";

/// Whether parked legacy routes (`/today-stats`, `/luna-consent`, `/health-context`) are mounted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParkedRoutesPolicy {
    /// Default production mode: parked legacy routes are quarantined (not mounted).
    Quarantined,
    /// Explicit opt-in for legacy compatibility tests only.
    Enabled,
}

impl ParkedRoutesPolicy {
    /// Parses the policy from a lookup closure; defaults to [`Self::Quarantined`].
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Self {
        match get(PARKED_ROUTES_ENV)
            .as_deref()
            .map(str::trim)
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("1" | "true") => Self::Enabled,
            _ => Self::Quarantined,
        }
    }

    /// Reports whether parked legacy routes may be mounted.
    pub const fn is_enabled(self) -> bool {
        matches!(self, Self::Enabled)
    }
}

/// Operator kill switch controlling whether Duet's AI may run model calls.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantOperatorSwitch {
    /// Operator switch is off (default for MVP).
    Off,
    /// Operator switch is enabled for consented accounts.
    On,
}

/// Reason why Duet's AI cannot execute a harness run for this account.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantBlockReason {
    /// Current Duet Terms of Service have not been accepted.
    TermsNotAccepted,
    /// Account has not explicitly granted AI permission (`intelligence_enabled` defaults to false).
    IntelligenceConsentRequired,
    /// Operator switch is off.
    OperatorSwitchOff,
}

/// Standing of the account's Terms of Service acceptance for AI admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TermsStanding {
    /// Current Duet Terms are accepted.
    Accepted,
    /// Current Duet Terms are not yet accepted.
    Required,
}

/// Standing of the account's explicit AI permission (`intelligence_enabled` + consent).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AiConsentStanding {
    /// Explicit AI consent is granted and `intelligence_enabled` is true.
    Granted,
    /// AI consent is not granted or `intelligence_enabled` is false.
    NotGranted,
}

/// Typed admission decision for Duet AI model calls (`intent/duet.md` AI.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum AssistantEligibility {
    /// Account satisfies accepted Terms, explicit AI consent, and operator switch.
    Ready,
    /// Model calls are blocked with an explicit cause.
    Blocked {
        /// Why model calls cannot start.
        reason: AssistantBlockReason,
    },
}

impl AssistantEligibility {
    /// Evaluates the single admission rule across Terms, explicit AI consent, and operator switch.
    pub const fn evaluate(
        terms: TermsStanding,
        ai_consent: AiConsentStanding,
        operator_switch: AssistantOperatorSwitch,
    ) -> Self {
        match (terms, ai_consent, operator_switch) {
            (TermsStanding::Required, _, _) => Self::Blocked {
                reason: AssistantBlockReason::TermsNotAccepted,
            },
            (TermsStanding::Accepted, AiConsentStanding::NotGranted, _) => Self::Blocked {
                reason: AssistantBlockReason::IntelligenceConsentRequired,
            },
            (TermsStanding::Accepted, AiConsentStanding::Granted, AssistantOperatorSwitch::Off) => {
                Self::Blocked {
                    reason: AssistantBlockReason::OperatorSwitchOff,
                }
            }
            (TermsStanding::Accepted, AiConsentStanding::Granted, AssistantOperatorSwitch::On) => {
                Self::Ready
            }
        }
    }

    /// Reports whether model execution is admitted.
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }
}

/// Separate, opt-in consent state for de-identified product improvement and research (`AI.5`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ResearchConsentState {
    /// Off by default; never bundled with Terms or core AI permission.
    Off,
    /// Person explicitly opted into separate research / product-improvement consent.
    Granted {
        /// When the person opted in.
        at: DateTime<Utc>,
    },
    /// Person withdrew research consent; core Duet features continue unaffected.
    Withdrawn {
        /// When the person withdrew.
        at: DateTime<Utc>,
    },
}

impl ResearchConsentState {
    fn from_row(row: Option<&ai_preference::Model>) -> Result<Self, ApiError> {
        let Some(row) = row else {
            return Ok(Self::Off);
        };
        match row.research_consent_state.as_str() {
            "off" => Ok(Self::Off),
            "granted" => Ok(Self::Granted {
                at: row.research_consent_at.unwrap_or(row.updated_at),
            }),
            "withdrawn" => Ok(Self::Withdrawn {
                at: row.research_consent_at.unwrap_or(row.updated_at),
            }),
            _ => Err(ApiError::Internal("stored research consent state invalid")),
        }
    }
}

/// Explicit action on the separate research / product-improvement consent surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchConsentAction {
    /// Explicitly opt in to separate de-identified product improvement / research.
    Grant,
    /// Withdraw separate research consent without affecting core Duet or AI permission.
    Withdraw,
}

/// Request body for `PUT /api/assistant/research-consent`.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResearchConsentRequest {
    /// Explicit choice (`grant` or `withdraw`).
    pub action: ResearchConsentAction,
    /// Expected monotonic revision for optimistic concurrency.
    pub expected_revision: i64,
}

/// Snapshot returned by `GET /api/assistant/foundation`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantFoundationSnapshot {
    /// Fixed standing AI disclosure shown at sign-up and kept in reach in settings/chat.
    pub standing_disclosure: String,
    /// Explicit reminder that AI services process information when enabled (`AI.4`).
    pub ai_processing_disclosed: bool,
    /// Whether `intelligence_enabled` is currently on (`false` by default before consent).
    pub intelligence_enabled: bool,
    /// Admission gate for running Duet assistant jobs on `crates/harness`.
    pub eligibility: AssistantEligibility,
    /// Separate opt-in research/product-improvement consent state (off by default).
    pub research_consent: ResearchConsentState,
    /// Current revision of the separate research consent control.
    pub research_consent_revision: i64,
    /// Rolling retention window (days) for superseded private memory versions.
    pub memory_version_retention_days: i64,
    /// Quarantine posture for parked legacy routes in this process.
    pub parked_routes: ParkedRoutesPolicy,
}

/// One historical version of Duet's private patient memory retained for 30 days.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedMemoryVersion {
    /// Monotonic memory revision number.
    pub revision: i64,
    /// Private memory JSON payload.
    pub content: Value,
    /// Timestamp when this superseded version entered retention.
    pub recorded_at: DateTime<Utc>,
    /// Timestamp (`recorded_at + 30 days`) after which this version is pruned.
    pub expires_at: DateTime<Utc>,
}

/// Error returned by the typed clinical library read boundary.
#[derive(Debug)]
pub enum ClinicalLibraryReadError {
    /// Clinical library service or database is unavailable.
    Unavailable(String),
    /// Caller supplied an invalid query or identifier.
    Invalid(String),
}

impl fmt::Display for ClinicalLibraryReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unavailable(detail) => write!(f, "clinical library unavailable: {detail}"),
            Self::Invalid(detail) => write!(f, "invalid clinical library request: {detail}"),
        }
    }
}

impl std::error::Error for ClinicalLibraryReadError {}

/// Product-neutral read boundary into `crates/clinical-library` (`intent/duet.md` What leaves Duet).
///
/// Duet uses the exact same clinical library as Chords and uses no web search.
#[async_trait]
pub trait ClinicalLibraryReader: Send + Sync {
    /// Searches the active clinical library snapshot by text query.
    async fn search(
        &self,
        query: &str,
        cursor: Option<&str>,
    ) -> Result<SearchResult, ClinicalLibraryReadError>;

    /// Reads exact evidence units for the given active candidate IDs.
    async fn read_items(
        &self,
        candidate_ids: &[String],
    ) -> Result<Vec<ReadItemOutcome>, ClinicalLibraryReadError>;

    /// Reads the clinical library's freshness standing report.
    async fn freshness(&self) -> Result<LibraryFreshnessReport, ClinicalLibraryReadError>;
}

/// In-process clinical library reader backed directly by `clinical_library::Store`.
#[derive(Clone)]
pub struct DirectClinicalLibraryReader {
    store: clinical_library::Store,
}

impl DirectClinicalLibraryReader {
    /// Wraps an already-connected `clinical_library::Store`.
    pub fn new(store: clinical_library::Store) -> Self {
        Self { store }
    }
}

#[async_trait]
impl ClinicalLibraryReader for DirectClinicalLibraryReader {
    async fn search(
        &self,
        query: &str,
        cursor: Option<&str>,
    ) -> Result<SearchResult, ClinicalLibraryReadError> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Err(ClinicalLibraryReadError::Invalid(
                "search query cannot be blank".into(),
            ));
        }
        self.store
            .search_active(trimmed, cursor, DEFAULT_SEARCH_LIMIT)
            .await
            .map_err(|error| ClinicalLibraryReadError::Unavailable(error.to_string()))
    }

    async fn read_items(
        &self,
        candidate_ids: &[String],
    ) -> Result<Vec<ReadItemOutcome>, ClinicalLibraryReadError> {
        self.store
            .read_active(candidate_ids)
            .await
            .map_err(|error| ClinicalLibraryReadError::Unavailable(error.to_string()))
    }

    async fn freshness(&self) -> Result<LibraryFreshnessReport, ClinicalLibraryReadError> {
        self.store
            .freshness_report()
            .await
            .map_err(|error| ClinicalLibraryReadError::Unavailable(error.to_string()))
    }
}

/// Bounded HTTP client for the standalone clinical library service (`/v1/search`, `/v1/read`, `/v1/freshness`).
#[derive(Clone)]
pub struct HttpClinicalLibraryReader {
    client: reqwest::Client,
    base_url: reqwest::Url,
}

impl HttpClinicalLibraryReader {
    /// Connects to a clinical library service base URL (HTTPS required except loopback HTTP).
    pub fn new(base_url: &str) -> Result<Self, ApiError> {
        let parsed = reqwest::Url::parse(base_url.trim())
            .map_err(|_| ApiError::Unprocessable("invalid clinical library URL"))?;
        let loopback = matches!(parsed.host_str(), Some("127.0.0.1" | "localhost" | "::1"));
        if parsed.scheme() != "https" && !(parsed.scheme() == "http" && loopback) {
            return Err(ApiError::Unprocessable(
                "clinical library URL must use HTTPS",
            ));
        }
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(|_| ApiError::Unavailable("clinical library HTTP client unavailable"))?;
        Ok(Self {
            client,
            base_url: parsed,
        })
    }

    fn endpoint(&self, path: &str) -> Result<reqwest::Url, ClinicalLibraryReadError> {
        self.base_url
            .join(path)
            .map_err(|error| ClinicalLibraryReadError::Invalid(error.to_string()))
    }
}

#[derive(Deserialize)]
struct ClinicalLibraryReadResponse {
    results: Vec<ReadItemOutcome>,
}

#[async_trait]
impl ClinicalLibraryReader for HttpClinicalLibraryReader {
    async fn search(
        &self,
        query: &str,
        cursor: Option<&str>,
    ) -> Result<SearchResult, ClinicalLibraryReadError> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Err(ClinicalLibraryReadError::Invalid(
                "search query cannot be blank".into(),
            ));
        }
        let response = self
            .client
            .post(self.endpoint("/v1/search")?)
            .json(&json!({
                "mode": "query",
                "query": trimmed,
                "cursor": cursor,
            }))
            .send()
            .await
            .map_err(|error| {
                ClinicalLibraryReadError::Unavailable(error.without_url().to_string())
            })?;
        if !response.status().is_success() {
            return Err(ClinicalLibraryReadError::Unavailable(format!(
                "clinical library search returned HTTP {}",
                response.status().as_u16()
            )));
        }
        response
            .json::<SearchResult>()
            .await
            .map_err(|error| ClinicalLibraryReadError::Unavailable(error.without_url().to_string()))
    }

    async fn read_items(
        &self,
        candidate_ids: &[String],
    ) -> Result<Vec<ReadItemOutcome>, ClinicalLibraryReadError> {
        let response = self
            .client
            .post(self.endpoint("/v1/read")?)
            .json(&json!({ "candidate_ids": candidate_ids }))
            .send()
            .await
            .map_err(|error| {
                ClinicalLibraryReadError::Unavailable(error.without_url().to_string())
            })?;
        if !response.status().is_success() {
            return Err(ClinicalLibraryReadError::Unavailable(format!(
                "clinical library read returned HTTP {}",
                response.status().as_u16()
            )));
        }
        let body = response
            .json::<ClinicalLibraryReadResponse>()
            .await
            .map_err(|error| {
                ClinicalLibraryReadError::Unavailable(error.without_url().to_string())
            })?;
        Ok(body.results)
    }

    async fn freshness(&self) -> Result<LibraryFreshnessReport, ClinicalLibraryReadError> {
        let response = self
            .client
            .get(self.endpoint("/v1/freshness")?)
            .send()
            .await
            .map_err(|error| {
                ClinicalLibraryReadError::Unavailable(error.without_url().to_string())
            })?;
        if !response.status().is_success() {
            return Err(ClinicalLibraryReadError::Unavailable(format!(
                "clinical library freshness returned HTTP {}",
                response.status().as_u16()
            )));
        }
        response
            .json::<LibraryFreshnessReport>()
            .await
            .map_err(|error| ClinicalLibraryReadError::Unavailable(error.without_url().to_string()))
    }
}

/// Inputs describing one Duet assistant turn submitted to `crates/harness`.
#[derive(Clone, Copy, Debug)]
pub struct AssistantTurnInput<'a> {
    /// Stable run identity persisted in `duet_harness_runs`.
    pub run_id: &'a str,
    /// Model selected by the product host.
    pub model: &'a str,
    /// Patient message initiating the turn.
    pub prompt: &'a str,
}

/// Summary of one completed or held Duet assistant harness execution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantRunSummary {
    /// Stable run identity persisted in `duet_harness_runs`.
    pub run_id: String,
    /// Terminal outcome produced by `crates/harness`.
    pub outcome: RunOutcome,
    /// Final assistant reply text when completed.
    pub reply_text: Option<String>,
    /// Number of ordered harness events recorded for this run.
    pub event_count: usize,
    /// Standing AI disclosure attached to this run's recipe.
    pub standing_disclosure: String,
}

/// Durable owner for Duet AI consent defaults, separate research consent, 30-day memory
/// version retention, and `crates/harness` + `crates/clinical-library` execution.
#[derive(Clone)]
pub struct AssistantFoundationStore {
    database: AccountDatabase,
    operator_switch: AssistantOperatorSwitch,
    parked_routes: ParkedRoutesPolicy,
    clinical_library: Option<Arc<dyn ClinicalLibraryReader>>,
}

impl AssistantFoundationStore {
    /// Creates an assistant foundation store with explicit operator switch and optional
    /// clinical library reader.
    pub fn new(
        database: AccountDatabase,
        operator_switch: AssistantOperatorSwitch,
        parked_routes: ParkedRoutesPolicy,
        clinical_library: Option<Arc<dyn ClinicalLibraryReader>>,
    ) -> Self {
        Self {
            database,
            operator_switch,
            parked_routes,
            clinical_library,
        }
    }

    /// Reads the account's current AI foundation state, standing disclosure, eligibility,
    /// and separate research consent.
    pub async fn snapshot(
        &self,
        context: &AccountContext,
    ) -> Result<AssistantFoundationSnapshot, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::RightsOnly).await?;

        let user = orm::model::<user_gate::Model>(
            &mut tx,
            &user_gate::Entity::find_by_id(context.account_id).into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;

        let pref = orm::model::<ai_preference::Model>(
            &mut tx,
            &ai_preference::Entity::find_by_id(context.account_id).into_query(),
        )
        .await?;
        tx.commit().await?;

        let terms = if user.terms_version.as_deref() == Some(crate::session::CURRENT_TERMS_VERSION)
        {
            TermsStanding::Accepted
        } else {
            TermsStanding::Required
        };
        let ai_consent = if user.intelligence_enabled
            && user.luna_consent_decision.as_deref() == Some("accepted")
        {
            AiConsentStanding::Granted
        } else {
            AiConsentStanding::NotGranted
        };
        let eligibility = AssistantEligibility::evaluate(terms, ai_consent, self.operator_switch);
        let research_consent = ResearchConsentState::from_row(pref.as_ref())?;
        let research_consent_revision =
            pref.as_ref().map_or(0, |row| row.research_consent_revision);

        Ok(AssistantFoundationSnapshot {
            standing_disclosure: STANDING_AI_DISCLOSURE.to_owned(),
            ai_processing_disclosed: true,
            intelligence_enabled: user.intelligence_enabled,
            eligibility,
            research_consent,
            research_consent_revision,
            memory_version_retention_days: MEMORY_RETENTION_DAYS,
            parked_routes: self.parked_routes,
        })
    }

    /// Updates the separate opt-in research/product-improvement consent without touching
    /// Terms acceptance or core `intelligence_enabled` state.
    pub async fn set_research_consent(
        &self,
        context: &AccountContext,
        request: ResearchConsentRequest,
        now: DateTime<Utc>,
    ) -> Result<AssistantFoundationSnapshot, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::RightsOnly).await?;

        let existing = orm::model::<ai_preference::Model>(
            &mut tx,
            &ai_preference::Entity::find_by_id(context.account_id)
                .lock_exclusive()
                .into_query(),
        )
        .await?;
        let current_revision = existing
            .as_ref()
            .map_or(0, |row| row.research_consent_revision);
        if current_revision != request.expected_revision {
            return Err(ApiError::Conflict("research consent revision changed"));
        }
        let next_state = match request.action {
            ResearchConsentAction::Grant => "granted",
            ResearchConsentAction::Withdraw => "withdrawn",
        };
        let next_revision = current_revision.saturating_add(1);
        let insert = Query::insert()
            .into_table(ai_preference::Entity.table_ref())
            .columns([
                ai_preference::Column::UserId,
                ai_preference::Column::ResearchConsentState,
                ai_preference::Column::ResearchConsentAt,
                ai_preference::Column::ResearchConsentRevision,
                ai_preference::Column::UpdatedAt,
            ])
            .values_panic([
                context.account_id.into(),
                next_state.into(),
                Some(now).into(),
                next_revision.into(),
                now.into(),
            ])
            .on_conflict(
                OnConflict::column(ai_preference::Column::UserId)
                    .update_columns([
                        ai_preference::Column::ResearchConsentState,
                        ai_preference::Column::ResearchConsentAt,
                        ai_preference::Column::ResearchConsentRevision,
                        ai_preference::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .to_owned();
        orm::execute(&mut tx, &insert).await?;
        tx.commit().await?;

        self.snapshot(context).await
    }

    /// Writes a new current version of `duet_private_memory` and archives any superseded
    /// version into `duet_private_memory_versions` with a 30-day rolling expiration.
    pub async fn write_memory(
        &self,
        context: &AccountContext,
        content: Value,
        now: DateTime<Utc>,
    ) -> Result<i64, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;

        let prior = orm::model::<private_memory::Model>(
            &mut tx,
            &private_memory::Entity::find_by_id(context.account_id)
                .lock_exclusive()
                .into_query(),
        )
        .await?;

        let next_revision = if let Some(existing) = prior {
            let expires_at = now + ChronoDuration::days(MEMORY_RETENTION_DAYS);
            let archive = Query::insert()
                .into_table(memory_version::Entity.table_ref())
                .columns([
                    memory_version::Column::UserId,
                    memory_version::Column::Revision,
                    memory_version::Column::Content,
                    memory_version::Column::RecordedAt,
                    memory_version::Column::ExpiresAt,
                ])
                .values_panic([
                    context.account_id.into(),
                    existing.revision.into(),
                    existing.content.into(),
                    now.into(),
                    expires_at.into(),
                ])
                .on_conflict(
                    OnConflict::columns([
                        memory_version::Column::UserId,
                        memory_version::Column::Revision,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .to_owned();
            orm::execute(&mut tx, &archive).await?;
            existing.revision.saturating_add(1)
        } else {
            1
        };

        let upsert = Query::insert()
            .into_table(private_memory::Entity.table_ref())
            .columns([
                private_memory::Column::UserId,
                private_memory::Column::Revision,
                private_memory::Column::Content,
            ])
            .values_panic([
                context.account_id.into(),
                next_revision.into(),
                content.into(),
            ])
            .on_conflict(
                OnConflict::column(private_memory::Column::UserId)
                    .update_columns([
                        private_memory::Column::Revision,
                        private_memory::Column::Content,
                    ])
                    .to_owned(),
            )
            .to_owned();
        orm::execute(&mut tx, &upsert).await?;
        tx.commit().await?;
        Ok(next_revision)
    }

    /// Reads the current private memory document and revision for the account.
    pub async fn current_memory(
        &self,
        context: &AccountContext,
    ) -> Result<Option<(i64, Value)>, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;
        let current = orm::model::<private_memory::Model>(
            &mut tx,
            &private_memory::Entity::find_by_id(context.account_id).into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(current.map(|row| (row.revision, row.content)))
    }

    /// Lists non-expired historical memory versions (`expires_at > now`) for the account.
    pub async fn retained_memory_versions(
        &self,
        context: &AccountContext,
        now: DateTime<Utc>,
    ) -> Result<Vec<RetainedMemoryVersion>, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;
        let rows = orm::models::<memory_version::Model>(
            &mut tx,
            &memory_version::Entity::find()
                .filter(memory_version::Column::UserId.eq(context.account_id))
                .filter(memory_version::Column::ExpiresAt.gt(now))
                .order_by_asc(memory_version::Column::Revision)
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(rows
            .into_iter()
            .map(|row| RetainedMemoryVersion {
                revision: row.revision,
                content: row.content,
                recorded_at: row.recorded_at,
                expires_at: row.expires_at,
            })
            .collect())
    }

    /// Prunes superseded private memory versions older than 30 days (`expires_at <= now`)
    /// without touching the current `duet_private_memory` row or versions still within 30 days.
    pub async fn prune_expired_memory_versions(
        &self,
        account_id: Uuid,
        now: DateTime<Utc>,
    ) -> Result<u64, ApiError> {
        let mut tx = self.database.begin(account_id).await?;
        let deleted = orm::execute(
            &mut tx,
            &memory_version::Entity::delete_many()
                .filter(memory_version::Column::UserId.eq(account_id))
                .filter(memory_version::Column::ExpiresAt.lte(now))
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(deleted)
    }

    /// Runs one Duet assistant job through `crates/harness` with `crates/clinical-library`
    /// read-only tools and persists the run record in `public.duet_harness_runs`.
    ///
    /// Fails closed with [`ApiError::Forbidden`] if Terms are not accepted,
    /// `intelligence_enabled` is false, AI consent is not granted, or the operator switch is off.
    pub async fn run_harness_turn<P: Provider + ?Sized>(
        &self,
        context: &AccountContext,
        turn_input: AssistantTurnInput<'_>,
        provider: &mut P,
        limits: RunLimits,
        cancel: &Cancellation,
    ) -> Result<AssistantRunSummary, ApiError> {
        let snapshot = self.snapshot(context).await?;
        if !snapshot.eligibility.is_ready() {
            return Err(ApiError::Forbidden);
        }
        let run_id = turn_input.run_id.trim();
        if run_id.is_empty() {
            return Err(ApiError::Unprocessable("run_id must not be empty"));
        }
        let model = turn_input.model;
        let prompt = turn_input.prompt;

        let cache_key = PromptCacheKey::scoped(
            "duet-assistant",
            &[&context.account_id.to_string(), "conversation"],
        );
        let recipe = duet_assistant_recipe(model);
        let binding = recipe.bind(Some(cache_key.clone()));
        let request: Request = binding.request(vec![Item::Message {
            role: "user".to_owned(),
            text: prompt.to_owned(),
        }]);

        let mut history = RunHistory::default();
        history
            .start(run_id.to_owned())
            .map_err(|_| ApiError::Internal("invalid harness run start"))?;

        let started_at = Utc::now();
        let cache_key_hex = serde_json::to_string(&cache_key)
            .map_err(|_| ApiError::Internal("prompt cache key serialization failed"))?;
        {
            let mut tx = self.database.begin(context.account_id).await?;
            check_current(&mut tx, context, AccessLevel::AcceptedLuna).await?;
            let insert = Query::insert()
                .into_table(harness_run::Entity.table_ref())
                .columns([
                    harness_run::Column::UserId,
                    harness_run::Column::RunId,
                    harness_run::Column::RecipeName,
                    harness_run::Column::Model,
                    harness_run::Column::PromptCacheKey,
                    harness_run::Column::Outcome,
                    harness_run::Column::Events,
                    harness_run::Column::StartedAt,
                ])
                .values_panic([
                    context.account_id.into(),
                    run_id.into(),
                    "duet-assistant-v1".into(),
                    model.into(),
                    cache_key_hex.into(),
                    "running".into(),
                    json!([]).into(),
                    started_at.into(),
                ])
                .to_owned();
            orm::execute(&mut tx, &insert).await?;
            tx.commit().await?;
        }

        let work = request
            .freeze()
            .map_err(|_| ApiError::Unprocessable("invalid harness request recipe"))?;
        let recovered = work
            .recover(
                "attempt-1",
                Records {
                    events: Vec::new(),
                    allowance_after: 0,
                },
                Messages {
                    sources: &[],
                    partial: None,
                },
                Inputs::default(),
            )
            .map_err(|_| ApiError::Internal("harness work recovery failed"))?;

        let mut product = DuetHarnessProduct::new(
            self.database.clone(),
            context.clone(),
            self.operator_switch,
            self.clinical_library.clone(),
            binding,
        );

        let result = recovered.run(provider, &mut product, limits, cancel).await;
        let (outcome, reply_text) = match result {
            Ok(turn) => (RunOutcome::Completed, extract_reply_text(&turn)),
            Err(Failure::NotCurrent(NotCurrent::Withdrawn)) => (RunOutcome::Cancelled, None),
            Err(Failure::NotCurrent(NotCurrent::Superseded | NotCurrent::Lost)) => {
                (RunOutcome::Cancelled, None)
            }
            Err(Failure::Cancelled) => (
                RunOutcome::Held {
                    reason: HoldReason::Stopped,
                },
                None,
            ),
            Err(Failure::Limit(_) | Failure::ContextRefused) => (
                RunOutcome::Held {
                    reason: HoldReason::Limit,
                },
                product.partial_reply.clone(),
            ),
            Err(
                Failure::Final(_)
                | Failure::Protocol(_)
                | Failure::Provider(_)
                | Failure::AfterOutput(_)
                | Failure::Host(_)
                | Failure::Encoding(_),
            ) => (RunOutcome::Failed, None),
        };

        history
            .end(outcome)
            .map_err(|_| ApiError::Internal("invalid harness run end"))?;

        let hold_reason = match outcome {
            RunOutcome::Held { reason } => Some(match reason {
                HoldReason::Stopped => "stopped",
                HoldReason::Limit => "limit",
                HoldReason::ProviderBlocked => "provider_blocked",
            }),
            RunOutcome::Completed | RunOutcome::Failed | RunOutcome::Cancelled => None,
        };
        let events_json = serde_json::to_value(&product.events)
            .map_err(|_| ApiError::Internal("failed to serialize harness events"))?;
        let (input_tokens, output_tokens) = sum_usage(&product.events);

        let mut tx = self.database.begin(context.account_id).await?;
        orm::execute(
            &mut tx,
            &harness_run::Entity::update_many()
                .col_expr(harness_run::Column::Outcome, Expr::val(outcome.status()))
                .col_expr(harness_run::Column::HoldReason, Expr::val(hold_reason))
                .col_expr(harness_run::Column::Events, Expr::val(events_json))
                .col_expr(harness_run::Column::InputTokens, Expr::val(input_tokens))
                .col_expr(harness_run::Column::OutputTokens, Expr::val(output_tokens))
                .col_expr(harness_run::Column::EndedAt, Expr::val(Some(Utc::now())))
                .filter(harness_run::Column::UserId.eq(context.account_id))
                .filter(harness_run::Column::RunId.eq(run_id))
                .into_query(),
        )
        .await?;
        tx.commit().await?;

        Ok(AssistantRunSummary {
            run_id: run_id.to_owned(),
            outcome,
            reply_text,
            event_count: product.events.len(),
            standing_disclosure: STANDING_AI_DISCLOSURE.to_owned(),
        })
    }

    /// Reads the stored harness run record for one account-scoped `run_id`.
    pub async fn read_harness_run(
        &self,
        context: &AccountContext,
        run_id: &str,
    ) -> Result<Option<(String, usize)>, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;
        let row = orm::model::<harness_run::Model>(
            &mut tx,
            &harness_run::Entity::find()
                .filter(harness_run::Column::UserId.eq(context.account_id))
                .filter(harness_run::Column::RunId.eq(run_id))
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(row.map(|r| {
            let count = r.events.as_array().map_or(0, Vec::len);
            (r.outcome, count)
        }))
    }
}

/// Builds the frozen `crates/harness` recipe for Duet's assistant, including the standing
/// disclosure and read-only `crates/clinical-library` tool schemas (never web search).
pub fn duet_assistant_recipe(model: &str) -> RequestRecipe {
    RequestRecipe {
        model: model.to_owned(),
        context_window_tokens: 65_536,
        max_input_tokens: 65_536,
        options: json!({}),
        instructions: format!(
            "{STANDING_AI_DISCLOSURE}\nUse only authorized account records and the Coda clinical library tools; never use web search."
        ),
        tools: vec![
            json!({
                "type": "function",
                "name": TOOL_CLINICAL_LIBRARY_SEARCH,
                "description": "Search the active Coda clinical library snapshot.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": { "type": "string" },
                        "cursor": { "type": "string" }
                    },
                    "required": ["query"],
                    "additionalProperties": false
                }
            }),
            json!({
                "type": "function",
                "name": TOOL_CLINICAL_LIBRARY_READ,
                "description": "Read exact evidence units from the active Coda clinical library snapshot by candidate_id.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "candidate_ids": {
                            "type": "array",
                            "items": { "type": "string" }
                        }
                    },
                    "required": ["candidate_ids"],
                    "additionalProperties": false
                }
            }),
            json!({
                "type": "function",
                "name": TOOL_CLINICAL_LIBRARY_FRESHNESS,
                "description": "Inspect the active Coda clinical library freshness standing.",
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }
            }),
        ],
    }
}

struct DuetHarnessProduct {
    database: AccountDatabase,
    context: AccountContext,
    operator_switch: AssistantOperatorSwitch,
    clinical_library: Option<Arc<dyn ClinicalLibraryReader>>,
    expected_binding: RequestBinding,
    events: Vec<Event>,
    tool_results: BTreeMap<String, String>,
    partial_reply: Option<String>,
}

impl DuetHarnessProduct {
    fn new(
        database: AccountDatabase,
        context: AccountContext,
        operator_switch: AssistantOperatorSwitch,
        clinical_library: Option<Arc<dyn ClinicalLibraryReader>>,
        expected_binding: RequestBinding,
    ) -> Self {
        Self {
            database,
            context,
            operator_switch,
            clinical_library,
            expected_binding,
            events: Vec::new(),
            tool_results: BTreeMap::new(),
            partial_reply: None,
        }
    }
}

#[async_trait]
impl Product for DuetHarnessProduct {
    async fn assert_current(&mut self) -> Result<(), ProductError> {
        if self.operator_switch != AssistantOperatorSwitch::On {
            return Err(NotCurrent::Withdrawn.into());
        }
        let mut tx = self
            .database
            .begin(self.context.account_id)
            .await
            .map_err(|error| ProductError::Failed(error.to_string().into()))?;
        match check_current(&mut tx, &self.context, AccessLevel::AcceptedLuna).await {
            Ok(()) => Ok(()),
            Err(ApiError::Forbidden | ApiError::SessionRevoked | ApiError::AbsentAccount) => {
                Err(NotCurrent::Withdrawn.into())
            }
            Err(error) => Err(ProductError::Failed(format!("{error:?}").into())),
        }
    }

    async fn message_sources(&mut self) -> Result<Option<Vec<ProductMessage>>, ProductError> {
        Ok(None)
    }

    async fn admit_binding(&mut self, binding: &RequestBinding) -> Result<(), ProductError> {
        if *binding != self.expected_binding {
            return Err(ProductError::Failed(
                "harness request binding drifted from accepted Duet recipe".into(),
            ));
        }
        Ok(())
    }

    async fn persist(&mut self, event: Event) -> Result<(), ProductError> {
        if let EventKind::ToolReturned {
            ref call_id,
            ref result,
        } = event.kind
            && let Some(original) = result.original()
        {
            self.tool_results
                .insert(call_id.clone(), original.to_owned());
        }
        self.events.push(event);
        Ok(())
    }

    fn read_only(&self, name: &str) -> bool {
        matches!(
            name,
            TOOL_CLINICAL_LIBRARY_SEARCH
                | TOOL_CLINICAL_LIBRARY_READ
                | TOOL_CLINICAL_LIBRARY_FRESHNESS
        )
    }

    fn execute(&mut self, call: &Invocation, arguments: Value) -> ToolFuture {
        let name = call.name.clone();
        let reader = self.clinical_library.clone();
        Box::pin(async move {
            let Some(reader) = reader else {
                return Ok(ToolOutcome::refused(
                    "clinical library reader is not configured",
                ));
            };
            match name.as_str() {
                TOOL_CLINICAL_LIBRARY_SEARCH => {
                    let query = arguments
                        .get("query")
                        .and_then(Value::as_str)
                        .unwrap_or_default();
                    let cursor = arguments.get("cursor").and_then(Value::as_str);
                    match reader.search(query, cursor).await {
                        Ok(result) => {
                            let encoded = serde_json::to_string(&result)
                                .map_err(|e| ProductError::Failed(e.to_string().into()))?;
                            Ok(ToolOutcome::Returned(encoded))
                        }
                        Err(error) => Ok(ToolOutcome::refused(error)),
                    }
                }
                TOOL_CLINICAL_LIBRARY_READ => {
                    let ids: Vec<String> = arguments
                        .get("candidate_ids")
                        .and_then(Value::as_array)
                        .map(|items| {
                            items
                                .iter()
                                .filter_map(Value::as_str)
                                .map(str::to_owned)
                                .collect()
                        })
                        .unwrap_or_default();
                    match reader.read_items(&ids).await {
                        Ok(items) => {
                            let encoded = serde_json::to_string(&items)
                                .map_err(|e| ProductError::Failed(e.to_string().into()))?;
                            Ok(ToolOutcome::Returned(encoded))
                        }
                        Err(error) => Ok(ToolOutcome::refused(error)),
                    }
                }
                TOOL_CLINICAL_LIBRARY_FRESHNESS => match reader.freshness().await {
                    Ok(report) => {
                        let encoded = serde_json::to_string(&report)
                            .map_err(|e| ProductError::Failed(e.to_string().into()))?;
                        Ok(ToolOutcome::Returned(encoded))
                    }
                    Err(error) => Ok(ToolOutcome::refused(error)),
                },
                _ => Ok(ToolOutcome::refused(format!(
                    "tool `{name}` is not authorized by Duet"
                ))),
            }
        })
    }

    fn read_result(&mut self, call_id: &str) -> ToolFuture<Option<String>> {
        let found = self.tool_results.get(call_id).cloned();
        Box::pin(async move { Ok(found) })
    }

    async fn finish(&mut self, _turn: &Turn) -> Result<Finish, ProductError> {
        Ok(Finish::Committed)
    }

    async fn finish_partial(
        &mut self,
        turn: &Turn,
        _backstops: &[LimitBackstop],
    ) -> Result<(), ProductError> {
        self.partial_reply = extract_reply_text(turn);
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<WindowSeed, ProductError> {
        Ok(WindowSeed {
            items: Vec::new(),
            note_references: Vec::new(),
        })
    }
}

fn extract_reply_text(turn: &Turn) -> Option<String> {
    let mut parts = Vec::new();
    for item in &turn.items {
        if let Item::Message { role, text } = item
            && role == "assistant"
            && !text.is_empty()
        {
            parts.push(text.as_str());
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("\n"))
    }
}

fn sum_usage(events: &[Event]) -> (Option<i64>, Option<i64>) {
    let mut input: i64 = 0;
    let mut output: i64 = 0;
    let mut saw_input = false;
    let mut saw_output = false;
    for event in events {
        if let EventKind::ProviderReturned(turn) = &event.kind {
            if let Some(tokens) = turn.usage.input_tokens {
                saw_input = true;
                input = input.saturating_add(i64::try_from(tokens).unwrap_or(i64::MAX));
            }
            if let Some(tokens) = turn.usage.output_tokens {
                saw_output = true;
                output = output.saturating_add(i64::try_from(tokens).unwrap_or(i64::MAX));
            }
        }
    }
    (saw_input.then_some(input), saw_output.then_some(output))
}

/// Router state for `/api/assistant/*` endpoints.
#[derive(Clone)]
pub struct AssistantState {
    /// Authenticated identity, privacy, and evidence owners.
    pub durable: DurableState,
    /// Assistant foundation owner.
    pub assistant: AssistantFoundationStore,
}

/// Builds the authenticated Duet assistant foundation router.
pub fn router(state: AssistantState) -> Router {
    Router::new()
        .route("/api/assistant/foundation", get(foundation_route))
        .route(
            "/api/assistant/research-consent",
            put(research_consent_route),
        )
        .with_state(state)
}

async fn foundation_route(
    State(state): State<AssistantState>,
    headers: HeaderMap,
) -> Result<Json<AssistantFoundationSnapshot>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(state.assistant.snapshot(&context).await?))
}

async fn research_consent_route(
    State(state): State<AssistantState>,
    headers: HeaderMap,
    Json(body): Json<ResearchConsentRequest>,
) -> Result<Json<AssistantFoundationSnapshot>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(
        state
            .assistant
            .set_research_consent(&context, body, Utc::now())
            .await?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parked_routes_policy_is_quarantined_by_default() {
        assert_eq!(
            ParkedRoutesPolicy::from_env(|_| None),
            ParkedRoutesPolicy::Quarantined
        );
        assert_eq!(
            ParkedRoutesPolicy::from_env(|_| Some("false".into())),
            ParkedRoutesPolicy::Quarantined
        );
        assert_eq!(
            ParkedRoutesPolicy::from_env(|_| Some("1".into())),
            ParkedRoutesPolicy::Enabled
        );
        assert_eq!(
            ParkedRoutesPolicy::from_env(|_| Some("TRUE".into())),
            ParkedRoutesPolicy::Enabled
        );
    }

    #[test]
    fn assistant_eligibility_enforces_terms_consent_and_operator_switch() {
        assert_eq!(
            AssistantEligibility::evaluate(
                TermsStanding::Required,
                AiConsentStanding::Granted,
                AssistantOperatorSwitch::On
            ),
            AssistantEligibility::Blocked {
                reason: AssistantBlockReason::TermsNotAccepted,
            }
        );
        assert_eq!(
            AssistantEligibility::evaluate(
                TermsStanding::Accepted,
                AiConsentStanding::NotGranted,
                AssistantOperatorSwitch::On
            ),
            AssistantEligibility::Blocked {
                reason: AssistantBlockReason::IntelligenceConsentRequired,
            }
        );
        assert_eq!(
            AssistantEligibility::evaluate(
                TermsStanding::Accepted,
                AiConsentStanding::Granted,
                AssistantOperatorSwitch::Off
            ),
            AssistantEligibility::Blocked {
                reason: AssistantBlockReason::OperatorSwitchOff,
            }
        );
        assert_eq!(
            AssistantEligibility::evaluate(
                TermsStanding::Accepted,
                AiConsentStanding::Granted,
                AssistantOperatorSwitch::On
            ),
            AssistantEligibility::Ready
        );
    }

    #[test]
    fn recipe_embeds_standing_disclosure_and_clinical_library_tools_only() {
        let recipe = duet_assistant_recipe("gpt-5.4");
        assert!(recipe.instructions.contains(STANDING_AI_DISCLOSURE));
        let names: Vec<&str> = recipe
            .tools
            .iter()
            .filter_map(|tool| tool.get("name").and_then(Value::as_str))
            .collect();
        assert_eq!(
            names,
            vec![
                TOOL_CLINICAL_LIBRARY_SEARCH,
                TOOL_CLINICAL_LIBRARY_READ,
                TOOL_CLINICAL_LIBRARY_FRESHNESS,
            ]
        );
    }

    #[test]
    fn http_clinical_library_reader_requires_https_except_loopback() {
        assert!(HttpClinicalLibraryReader::new("https://library.coda.test").is_ok());
        assert!(HttpClinicalLibraryReader::new("http://127.0.0.1:8081").is_ok());
        assert!(HttpClinicalLibraryReader::new("http://library.coda.test").is_err());
    }
}
