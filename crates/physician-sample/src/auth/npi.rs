//! Public NPPES lookup and self-asserted profile association, never verification.
#[cfg(test)]
mod tests;
use super::*;
use npi_registry::{Clinician, Npi, RegistryError, SearchQuery};
use physician_auth::{NpiAssociation, NpiAssociationError};
use physician_wire::{ClinicianSearch, ClinicianSummary, NpiClaim, NpiClaimRequest};

pub(super) fn summary(value: &Clinician) -> ClinicianSummary {
    ClinicianSummary {
        npi: value.npi.to_string(),
        name: value.name.clone(),
        credential: value.credential.clone(),
        specialty: value
            .primary_taxonomy
            .as_ref()
            .map(|t| t.description.clone()),
        practice_address: value.practice_address.as_ref().map(|a| {
            [
                Some(a.line1.clone()),
                a.line2.clone(),
                Some(format!("{}, {} {}", a.city, a.state, a.postal_code)),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(", ")
        }),
        individual: value.kind == npi_registry::ClinicianKind::Individual,
    }
}
pub(super) fn claim(value: &NpiAssociation) -> NpiClaim {
    NpiClaim {
        clinician: summary(&value.clinician),
        lookup_checked_at: value.lookup_checked_at,
        name_matched: value.name_matched,
    }
}
fn error(status: StatusCode, detail: &str) -> Response {
    (status, Json(json!({"detail": detail}))).into_response()
}
fn registry_error(value: RegistryError) -> Response {
    match value {
        RegistryError::InvalidQuery(_) | RegistryError::Rejected(_) => error(
            StatusCode::BAD_REQUEST,
            "Invalid directory search criteria.",
        ),
        _ => error(
            StatusCode::BAD_GATEWAY,
            "The public NPI directory is unavailable. Try again later.",
        ),
    }
}
fn store_error(value: NpiAssociationError) -> Response {
    let (status, detail) = match value {
        NpiAssociationError::Inactive => {
            (StatusCode::UNAUTHORIZED, "Sign in to an active account.")
        }
        NpiAssociationError::NotIndividual => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "Choose an individual NPI, not an organization.",
        ),
        NpiAssociationError::NameMismatch => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "The directory family name does not match your display name.",
        ),
        NpiAssociationError::Forbidden => (
            StatusCode::FORBIDDEN,
            "Only an administrator can bypass the name consistency check.",
        ),
        NpiAssociationError::Conflict => (
            StatusCode::CONFLICT,
            "This NPI is already associated with an account. Association does not prove ownership.",
        ),
        NpiAssociationError::Unavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "The association could not be saved. Try again later.",
        ),
    };
    error(status, detail)
}
async fn identity(
    auth: &Auth,
    headers: axum::http::HeaderMap,
) -> Result<PhysicianIdentity, StatusCode> {
    let token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|v| v.trim().to_owned());
    resolve(auth, token).await
}
pub(super) async fn associate(
    State(auth): State<Arc<Auth>>,
    headers: axum::http::HeaderMap,
    Json(input): Json<NpiClaimRequest>,
) -> Response {
    let caller = match identity(&auth, headers).await {
        Ok(v) => v,
        Err(status) => return error(status, "Sign in to manage your profile."),
    };
    let npi = match Npi::parse(&input.npi) {
        Ok(v) => v,
        Err(_) => return error(StatusCode::BAD_REQUEST, "Enter a valid ten-digit NPI."),
    };
    let record = match auth.directory.by_number(&npi).await {
        Ok(Some(value)) => value,
        Ok(None) => {
            return error(
                StatusCode::NOT_FOUND,
                "This NPI was not found in the public directory.",
            );
        }
        Err(value) => return registry_error(value),
    };
    match auth
        .store
        .claim_npi(&caller, record, now(), input.override_name)
        .await
    {
        Ok(value) => Json(claim(&value)).into_response(),
        Err(value) => store_error(value),
    }
}
pub(super) async fn clear(
    State(auth): State<Arc<Auth>>,
    headers: axum::http::HeaderMap,
) -> Response {
    let caller = match identity(&auth, headers).await {
        Ok(v) => v,
        Err(status) => return error(status, "Sign in to manage your profile."),
    };
    match auth.store.clear_npi(&caller).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(v) => store_error(v),
    }
}
pub(super) async fn search(
    State(auth): State<Arc<Auth>>,
    headers: axum::http::HeaderMap,
    Json(input): Json<ClinicianSearch>,
) -> Response {
    if let Err(status) = identity(&auth, headers).await {
        return error(status, "Sign in to manage your profile.");
    }
    let query = SearchQuery {
        first_name: input.first_name,
        last_name: input.last_name,
        city: input.city,
        state: input.state,
        ..Default::default()
    };
    match auth.directory.search(&query).await {
        Ok(values) => Json(values.iter().map(summary).collect::<Vec<_>>()).into_response(),
        Err(value) => registry_error(value),
    }
}
