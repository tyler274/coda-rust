//! In-memory account and session store for `duet-api` (`ENG-25`).
//!
//! The dev bearer `Bearer dev:<uuid>` names the user. This store keeps that
//! user's account, terms receipt, Today stat selection, and sign-out fence.
//! It does not call Identity Platform, Postgres, or an Epic sweep.

use std::collections::{HashMap, HashSet};

use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use chrono::{DateTime, Utc};
use duet_wire::models::session_revocation_response::Status as RevocationStatus;
use duet_wire::models::{
    AccountIdentityRequest, AccountState, LunaConsentDecision, SessionRevocationResponse,
    TermsAcceptanceRequest, TodayStatSelectionRequest, TodayStatSelectionState,
};
use uuid::Uuid;

use crate::{ApiError, AppState};

/// Current Duet terms version (`agreement_versions.CURRENT_TERMS_VERSION`).
pub(crate) const CURRENT_TERMS_VERSION: &str = "2026-09-23";

/// Current Luna consent version advertised on account state.
const CURRENT_LUNA_CONSENT_VERSION: &str = "2026-09-24";

const MAX_DISPLAY_NAME_LENGTH: usize = 80;
const MAX_TERMS_VERSION_LENGTH: usize = 40;
const MAX_TODAY_STATS: usize = 4;
const MAX_STAT_ID_LENGTH: usize = 120;

/// Metric ids the Today screen can pin. A `metric:` id outside this set is rejected.
const TODAY_METRIC_IDS: &[&str] = &[
    "steps",
    "active_energy",
    "exercise_minutes",
    "sleep_minutes",
    "resting_heart_rate",
    "distance",
    "body_mass",
    "dietary_energy",
    "water",
];

#[derive(Debug, Default, Clone)]
struct Account {
    display_name: Option<String>,
    today_stat_ids: Option<Vec<String>>,
    terms_accepted_at: Option<DateTime<Utc>>,
}

impl Account {
    fn to_state(&self) -> AccountState {
        let mut state = AccountState::new();
        state.display_name = Some(self.display_name.clone());
        // This process does not run a model, so the flag matches an API
        // without `DUET_OPENAI_PHI_READY`.
        state.intelligence_available = Some(false);
        state.intelligence_enabled = Some(true);
        state.luna_consent = Some(LunaConsentDecision::Undecided);
        state.luna_consent_at = Some(None);
        state.luna_consent_version = Some(CURRENT_LUNA_CONSENT_VERSION.to_owned());
        state.setup_complete = Some(false);
        state.terms_accepted = Some(self.terms_accepted_at.is_some());
        state.terms_accepted_at = Some(self.terms_accepted_at.map(|at| at.fixed_offset()));
        state.terms_version = Some(CURRENT_TERMS_VERSION.to_owned());
        state.today_stat_ids = Some(self.today_stat_ids.clone());
        state
    }
}

/// Accounts keyed by the dev-bearer user id, plus the sign-out fence.
#[derive(Debug, Default)]
pub(crate) struct Accounts {
    by_user: HashMap<Uuid, Account>,
    revoked: HashSet<Uuid>,
}

impl Accounts {
    pub(crate) fn is_revoked(&self, user_id: Uuid) -> bool {
        self.revoked.contains(&user_id)
    }

    fn refuse_revoked(&self, user_id: Uuid) -> Result<(), ApiError> {
        if self.is_revoked(user_id) {
            Err(ApiError::SessionRevoked)
        } else {
            Ok(())
        }
    }

    /// Creates the account or returns the one already stored for this user.
    ///
    /// An explicit display name renames the account. A missing name leaves
    /// the stored name alone, including on the first ensure.
    pub(crate) fn ensure(
        &mut self,
        user_id: Uuid,
        display_name: Option<String>,
    ) -> Result<AccountState, ApiError> {
        self.refuse_revoked(user_id)?;
        let account = self.by_user.entry(user_id).or_default();
        if let Some(name) = display_name {
            account.display_name = Some(name);
        }
        Ok(account.to_state())
    }

    /// Records acceptance of the current terms version.
    ///
    /// A different version is a conflict even when the account is absent,
    /// matching the Python store's version check. The current version against
    /// an account that was never ensured is the absent-account refusal.
    /// Accepting the same version again keeps the original timestamp.
    pub(crate) fn accept_terms(
        &mut self,
        user_id: Uuid,
        version: &str,
    ) -> Result<AccountState, ApiError> {
        self.refuse_revoked(user_id)?;
        let len = version.chars().count();
        if len == 0 || len > MAX_TERMS_VERSION_LENGTH {
            return Err(ApiError::Unprocessable("invalid terms version"));
        }
        if version != CURRENT_TERMS_VERSION {
            return Err(ApiError::AgreementChanged);
        }
        let Some(account) = self.by_user.get_mut(&user_id) else {
            return Err(ApiError::AbsentAccount);
        };
        if account.terms_accepted_at.is_none() {
            account.terms_accepted_at = Some(Utc::now());
        }
        Ok(account.to_state())
    }

    /// Ends every session for this user. A later request with that bearer is refused.
    pub(crate) fn revoke(&mut self, user_id: Uuid) -> Result<SessionRevocationResponse, ApiError> {
        self.refuse_revoked(user_id)?;
        if !self.by_user.contains_key(&user_id) {
            return Err(ApiError::AbsentAccount);
        }
        self.revoked.insert(user_id);
        Ok(SessionRevocationResponse::new(RevocationStatus::Confirmed))
    }

    /// Stores the selected Today stat ids for an account that has accepted terms.
    pub(crate) fn update_today_stats(
        &mut self,
        user_id: Uuid,
        stat_ids: Vec<String>,
    ) -> Result<Vec<String>, ApiError> {
        self.refuse_revoked(user_id)?;
        let Some(account) = self.by_user.get_mut(&user_id) else {
            return Err(ApiError::AbsentAccount);
        };
        if account.terms_accepted_at.is_none() {
            return Err(ApiError::TermsRequired);
        }
        account.today_stat_ids = Some(stat_ids.clone());
        Ok(stat_ids)
    }
}

pub(crate) fn explicit_display_name(
    body: &AccountIdentityRequest,
) -> Result<Option<String>, ApiError> {
    let Some(Some(name)) = &body.display_name else {
        return Ok(None);
    };
    if name.chars().count() > MAX_DISPLAY_NAME_LENGTH {
        return Err(ApiError::Unprocessable("display name is too long"));
    }
    Ok(normalize_display_name(name))
}

fn normalize_display_name(value: &str) -> Option<String> {
    let normalized = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

pub(crate) fn validate_stat_ids(values: &[String]) -> Result<Vec<String>, ApiError> {
    if values.is_empty() || values.len() > MAX_TODAY_STATS {
        return Err(ApiError::Unprocessable("invalid Today stat identifier"));
    }
    let normalized: Vec<String> = values.iter().map(|value| value.trim().to_owned()).collect();
    if normalized.iter().any(|value| !stat_id_ok(value)) {
        return Err(ApiError::Unprocessable("invalid Today stat identifier"));
    }
    let unique: HashSet<&str> = normalized.iter().map(String::as_str).collect();
    if unique.len() != normalized.len() {
        return Err(ApiError::Unprocessable(
            "Today stat identifiers must be unique",
        ));
    }
    Ok(normalized)
}

fn stat_id_ok(value: &str) -> bool {
    if value.is_empty() || value.chars().count() > MAX_STAT_ID_LENGTH {
        return false;
    }
    if let Some(metric) = value.strip_prefix("metric:") {
        return TODAY_METRIC_IDS.contains(&metric);
    }
    if let Some(signal) = value.strip_prefix("signal:") {
        let body: String = signal.chars().filter(|c| *c != '_').collect();
        return !body.is_empty() && body.chars().all(char::is_alphanumeric);
    }
    false
}

pub(crate) async fn put_ensure_account(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<AccountIdentityRequest>,
) -> Result<Json<AccountState>, ApiError> {
    let user_id = crate::account_id(&state, &headers)?;
    let name = explicit_display_name(&body)?;
    let mut accounts = state.accounts.lock().expect("accounts lock");
    Ok(Json(accounts.ensure(user_id, name)?))
}

pub(crate) async fn put_accept_terms(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<TermsAcceptanceRequest>,
) -> Result<Json<AccountState>, ApiError> {
    let user_id = crate::account_id(&state, &headers)?;
    let mut accounts = state.accounts.lock().expect("accounts lock");
    Ok(Json(accounts.accept_terms(user_id, &body.version)?))
}

pub(crate) async fn post_revoke_sessions(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<SessionRevocationResponse>, ApiError> {
    let user_id = crate::account_id(&state, &headers)?;
    let mut accounts = state.accounts.lock().expect("accounts lock");
    Ok(Json(accounts.revoke(user_id)?))
}

pub(crate) async fn put_today_stats(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<TodayStatSelectionRequest>,
) -> Result<Json<TodayStatSelectionState>, ApiError> {
    let user_id = crate::account_id(&state, &headers)?;
    let stat_ids = validate_stat_ids(&body.stat_ids)?;
    let mut accounts = state.accounts.lock().expect("accounts lock");
    let stored = accounts.update_today_stats(user_id, stat_ids)?;
    Ok(Json(TodayStatSelectionState::new(stored)))
}
