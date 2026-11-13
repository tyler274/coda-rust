//! Minimal production routes on one durable account context.
use crate::{
    ApiError,
    evidence::{
        AddressedRecordRevision, EvidenceBatch, EvidenceBatchResult, EvidenceMutation,
        EvidenceSnapshot, EvidenceStore, ExactEvidence, MAX_EVIDENCE_BATCH_BYTES,
    },
    identity::{AccountContext, IdentityStore, check_current},
    privacy::{PrivacyStore, Receipt, Restriction},
};
use axum::{
    Json, Router,
    extract::{Query, State},
    http::HeaderMap,
    routing::{get, post, put},
};
use duet_wire::models::{
    AccountIdentityRequest, AccountState, LunaConsentDecision, TermsAcceptanceRequest,
    TodayStatSelectionRequest, TodayStatSelectionState,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

/// State shared by authenticated account/evidence routes and later workspace/publication owners.
#[derive(Clone)]
pub struct DurableState {
    /// Verified Identity Platform mapping and account fences.
    pub identity: IdentityStore,
    /// Independent privacy restriction/receipt owner.
    pub privacy: PrivacyStore,
    /// Exact canonical evidence reader/mutation owner.
    pub evidence: EvidenceStore,
}
impl DurableState {
    /// One verified account context for all route families. Never accept caller-selected account IDs.
    pub async fn authenticate(&self, headers: &HeaderMap) -> Result<AccountContext, ApiError> {
        self.identity
            .authenticate(
                &crate::bearer(headers).ok_or(ApiError::Unauthorized)?,
                false,
            )
            .await
    }
}
/// Production router. Legacy process-global shares and unauthenticated mutations are deliberately absent.
pub fn router(state: DurableState) -> Router {
    Router::new()
        .route("/api/auth/account", get(account).put(ensure))
        .route("/api/auth/account/context", get(context))
        .route("/api/apple-health/generation", get(apple_health_generation))
        .route("/api/auth/account/terms", put(terms))
        .route("/api/auth/account/today-stats", put(stats))
        .route("/api/auth/account/luna-consent", put(luna))
        .route("/api/auth/privacy-restrictions", post(restrict))
        .route("/api/canonical-evidence", post(ingest).get(snapshot))
        .route("/api/canonical-evidence/exact", post(exact))
        .route(
            "/api/canonical-evidence/batch",
            post(ingest_batch).layer(axum::extract::DefaultBodyLimit::max(
                MAX_EVIDENCE_BATCH_BYTES,
            )),
        )
        .with_state(state)
}
async fn state_in(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: Uuid,
) -> Result<AccountState, ApiError> {
    let row=sqlx::query("select display_name,today_stat_ids,terms_version,terms_accepted_at,luna_consent_decision,luna_consent_at,intelligence_enabled,setup_complete from public.users where id=$1").bind(id).fetch_one(&mut **tx).await?;
    let mut s = AccountState::new();
    s.display_name = Some(row.get("display_name"));
    s.today_stat_ids = Some(row.get("today_stat_ids"));
    s.terms_version = Some(crate::session::CURRENT_TERMS_VERSION.to_owned());
    s.terms_accepted = Some(
        row.get::<Option<String>, _>("terms_version").as_deref()
            == Some(crate::session::CURRENT_TERMS_VERSION),
    );
    s.terms_accepted_at = Some(
        row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("terms_accepted_at")
            .map(|x| x.fixed_offset()),
    );
    s.luna_consent = Some(
        match row
            .get::<Option<String>, _>("luna_consent_decision")
            .as_deref()
        {
            Some("accepted") => LunaConsentDecision::Accepted,
            Some("declined") => LunaConsentDecision::Declined,
            _ => LunaConsentDecision::Undecided,
        },
    );
    s.luna_consent_version = Some("2026-09-24".into());
    s.luna_consent_at = Some(
        row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("luna_consent_at")
            .map(|x| x.fixed_offset()),
    );
    s.intelligence_enabled = Some(row.get("intelligence_enabled"));
    s.intelligence_available = Some(false);
    s.setup_complete = Some(row.get("setup_complete"));
    Ok(s)
}
async fn apple_health_generation(
    State(s): State<DurableState>,
    headers: HeaderMap,
) -> Result<Json<i32>, ApiError> {
    Ok(Json(
        s.evidence
            .apple_health_generation(&s.authenticate(&headers).await?)
            .await?,
    ))
}
async fn context(
    State(s): State<DurableState>,
    headers: HeaderMap,
) -> Result<Json<AccountContext>, ApiError> {
    Ok(Json(s.authenticate(&headers).await?))
}
async fn account(
    State(s): State<DurableState>,
    headers: HeaderMap,
) -> Result<Json<AccountState>, ApiError> {
    let c = s.authenticate(&headers).await?;
    let mut tx = s.identity.database.begin(c.account_id).await?;
    check_current(&mut tx, &c, false).await?;
    Ok(Json(state_in(&mut tx, c.account_id).await?))
}
async fn ensure(
    State(s): State<DurableState>,
    headers: HeaderMap,
    Json(body): Json<AccountIdentityRequest>,
) -> Result<Json<AccountState>, ApiError> {
    let c = s
        .identity
        .authenticate(
            &crate::bearer(&headers).ok_or(ApiError::Unauthorized)?,
            true,
        )
        .await?;
    let name = crate::session::explicit_display_name(&body)?;
    let mut tx = s.identity.database.begin(c.account_id).await?;
    check_current(&mut tx, &c, false).await?;
    if let Some(name) = name {
        sqlx::query("update public.users set display_name=$2 where id=$1")
            .bind(c.account_id)
            .bind(name)
            .execute(&mut *tx)
            .await?;
    }
    let result = state_in(&mut tx, c.account_id).await?;
    tx.commit().await?;
    Ok(Json(result))
}
async fn terms(
    State(s): State<DurableState>,
    headers: HeaderMap,
    Json(body): Json<TermsAcceptanceRequest>,
) -> Result<Json<AccountState>, ApiError> {
    if body.version != crate::session::CURRENT_TERMS_VERSION {
        return Err(ApiError::AgreementChanged);
    }
    let c = s.authenticate(&headers).await?;
    let mut tx = s.identity.database.begin(c.account_id).await?;
    check_current(&mut tx, &c, false).await?;
    sqlx::query("update public.users set terms_accepted_at=case when terms_version=$2 then terms_accepted_at else now() end,terms_version=$2 where id=$1").bind(c.account_id).bind(body.version).execute(&mut *tx).await?;
    let result = state_in(&mut tx, c.account_id).await?;
    tx.commit().await?;
    Ok(Json(result))
}
async fn stats(
    State(s): State<DurableState>,
    headers: HeaderMap,
    Json(body): Json<TodayStatSelectionRequest>,
) -> Result<Json<TodayStatSelectionState>, ApiError> {
    let ids = crate::session::validate_stat_ids(&body.stat_ids)?;
    let c = s.authenticate(&headers).await?;
    let mut tx = s.identity.database.begin(c.account_id).await?;
    check_current(&mut tx, &c, false).await?;
    let updated =
        sqlx::query("update public.users set today_stat_ids=$2 where id=$1 and terms_version=$3")
            .bind(c.account_id)
            .bind(&ids)
            .bind(crate::session::CURRENT_TERMS_VERSION)
            .execute(&mut *tx)
            .await?;
    if updated.rows_affected() != 1 {
        return Err(ApiError::TermsRequired);
    }
    tx.commit().await?;
    Ok(Json(TodayStatSelectionState::new(ids)))
}
/// Explicit acceptance; withdrawal uses a metadata-only privacy operation with stable retry identity.
#[derive(Deserialize)]
struct LunaAcceptance {
    version: String,
    expected_revision: i64,
}
async fn luna(
    State(s): State<DurableState>,
    headers: HeaderMap,
    Json(body): Json<LunaAcceptance>,
) -> Result<Json<AccountState>, ApiError> {
    if body.version != "2026-09-24" {
        return Err(ApiError::AgreementChanged);
    }
    let c = s.authenticate(&headers).await?;
    let mut tx = s.identity.database.begin(c.account_id).await?;
    check_current(&mut tx, &c, false).await?;
    let updated=sqlx::query("update public.users set luna_consent_decision='accepted',luna_consent_version=$2,luna_consent_at=now(),intelligence_enabled=true,intelligence_control_revision=intelligence_control_revision+1 where id=$1 and intelligence_control_revision=$3 and terms_version=$4")
        .bind(c.account_id).bind(body.version).bind(body.expected_revision).bind(crate::session::CURRENT_TERMS_VERSION).execute(&mut *tx).await?;
    if updated.rows_affected() != 1 {
        return Err(ApiError::Conflict("permission or terms changed"));
    }
    let result = state_in(&mut tx, c.account_id).await?;
    tx.commit().await?;
    Ok(Json(result))
}
/// Stable privacy command body; account is always resolved from the bearer.
#[derive(Serialize, Deserialize)]
pub struct RestrictionRequest {
    /// Reuse on every retry, including uncertain acknowledgement.
    pub event_id: Uuid,
    /// Closed restriction operation.
    pub operation: Restriction,
    /// Resource UUID; omitted means the verified account.
    pub scope_id: Option<Uuid>,
    /// Expected permission/resource incarnation revision.
    pub scope_version: i64,
}
async fn restrict(
    State(s): State<DurableState>,
    headers: HeaderMap,
    Json(body): Json<RestrictionRequest>,
) -> Result<Json<Receipt>, ApiError> {
    let c = s
        .identity
        .authenticate_receipt_retry(&crate::bearer(&headers).ok_or(ApiError::Unauthorized)?)
        .await?;
    Ok(Json(
        s.privacy
            .restrict(
                &c,
                body.event_id,
                body.operation,
                body.scope_id.unwrap_or(c.account_id),
                body.scope_version,
            )
            .await?,
    ))
}
async fn ingest(
    State(s): State<DurableState>,
    headers: HeaderMap,
    Json(body): Json<EvidenceMutation>,
) -> Result<Json<AddressedRecordRevision>, ApiError> {
    let c = s.authenticate(&headers).await?;
    Ok(Json(s.evidence.ingest(&c, &body).await?))
}
/// Device batch: each item commits or is rejected on its own; the HTTP route never
/// carries connector authority.
async fn ingest_batch(
    State(s): State<DurableState>,
    headers: HeaderMap,
    Json(body): Json<EvidenceBatch>,
) -> Result<Json<EvidenceBatchResult>, ApiError> {
    let c = s.authenticate(&headers).await?;
    Ok(Json(s.evidence.ingest_batch(&c, &body).await?))
}
#[derive(Deserialize)]
struct Cursor {
    #[serde(default)]
    after: i64,
}
async fn snapshot(
    State(s): State<DurableState>,
    headers: HeaderMap,
    Query(query): Query<Cursor>,
) -> Result<Json<EvidenceSnapshot>, ApiError> {
    let c = s.authenticate(&headers).await?;
    Ok(Json(s.evidence.snapshot(&c, query.after).await?))
}
#[derive(Deserialize)]
struct ExactRequest {
    address: AddressedRecordRevision,
    #[serde(default)]
    require_current: bool,
}
async fn exact(
    State(s): State<DurableState>,
    headers: HeaderMap,
    Json(body): Json<ExactRequest>,
) -> Result<Json<ExactEvidence>, ApiError> {
    let c = s.authenticate(&headers).await?;
    Ok(Json(
        s.evidence
            .exact(&c, &body.address, body.require_current)
            .await?,
    ))
}
