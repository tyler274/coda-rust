//! Account rules the durable account routes share: the current terms version,
//! display-name normalization and Today stat validation.

use std::collections::HashSet;

use duet_wire::models::AccountIdentityRequest;

use crate::ApiError;

/// Current Duet terms version (`agreement_versions.CURRENT_TERMS_VERSION`).
pub(crate) const CURRENT_TERMS_VERSION: &str = "2026-09-23";

const MAX_DISPLAY_NAME_LENGTH: usize = 80;
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
