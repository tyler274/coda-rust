//! Private care-team and release-template client. Never logs bodies or response prose.
use crate::DuetClient;
pub use duet_wire::models::{
    CareTeamAddRequest, CareTeamMember, CareTeamPatchRequest, ClinicianSearchRequest,
    DirectoryClinician, ReleaseAuthorizationRequest, ReleaseAuthorizationView, ReleaseParty,
    ReleasePatient, ReleaseRecipient, ReleaseScope, ReleaseSensitiveDisclosure,
};
/// Closed errors; a transport failure does not prove a mutation failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CareTeamError {
    /// Reauthenticate without automatically replaying writes.
    Unauthorized,
    /// Current account authority is required.
    Forbidden,
    /// No resource belongs to this account at that identifier.
    NotFound,
    /// Review the current resource before retrying.
    Conflict,
    /// Invalid input or text cannot be faithfully rendered.
    Invalid,
    /// Outcome unavailable or uncertain; do not blindly create a second draft.
    Uncertain,
}
impl std::fmt::Display for CareTeamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unauthorized => "Sign in again; writes were not replayed.",
            Self::Forbidden => "Current account permission is required.",
            Self::NotFound => "Care-team resource unavailable.",
            Self::Conflict => "Resource changed; review before retrying.",
            Self::Invalid => "Review the directory query or release template.",
            Self::Uncertain => {
                "The outcome could not be confirmed. Do not create a duplicate release template."
            }
        })
    }
}
impl std::error::Error for CareTeamError {}
async fn response(request: reqwest::RequestBuilder) -> Result<reqwest::Response, CareTeamError> {
    let response = request.send().await.map_err(|_| CareTeamError::Uncertain)?;
    match response.status().as_u16() {
        200..=299 => Ok(response),
        401 => Err(CareTeamError::Unauthorized),
        403 => Err(CareTeamError::Forbidden),
        404 => Err(CareTeamError::NotFound),
        409 => Err(CareTeamError::Conflict),
        400 | 413 | 422 => Err(CareTeamError::Invalid),
        _ => Err(CareTeamError::Uncertain),
    }
}
async fn json<T: serde::de::DeserializeOwned>(
    request: reqwest::RequestBuilder,
) -> Result<T, CareTeamError> {
    response(request)
        .await?
        .json()
        .await
        .map_err(|_| CareTeamError::Uncertain)
}
impl DuetClient {
    fn care_request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.configuration
            .client
            .request(method, format!("{}{}", self.configuration.base_path, path))
            .bearer_auth(
                self.configuration
                    .bearer_access_token
                    .as_deref()
                    .unwrap_or_default(),
            )
            .header(reqwest::header::CACHE_CONTROL, "no-store")
    }
    /// Search public NPPES records using POST, never search terms in URLs.
    pub async fn search_clinicians(
        &self,
        query: &ClinicianSearchRequest,
    ) -> Result<Vec<DirectoryClinician>, CareTeamError> {
        json(
            self.care_request(reqwest::Method::POST, "/api/clinicians/search")
                .json(query),
        )
        .await
    }
    /// List the signed-in account's private care team.
    pub async fn care_team(&self) -> Result<Vec<CareTeamMember>, CareTeamError> {
        json(self.care_request(reqwest::Method::GET, "/api/care-team")).await
    }
    /// Add a server-fetched clinician snapshot; repeated NPI returns the same member.
    pub async fn add_care_team_member(&self, npi: &str) -> Result<CareTeamMember, CareTeamError> {
        json(
            self.care_request(reqwest::Method::POST, "/api/care-team")
                .json(&CareTeamAddRequest::new(npi.into())),
        )
        .await
    }
    /// Patch labels; empty text clears, absent fields remain unchanged.
    pub async fn patch_care_team_member(
        &self,
        id: uuid::Uuid,
        patch: &CareTeamPatchRequest,
    ) -> Result<CareTeamMember, CareTeamError> {
        json(
            self.care_request(reqwest::Method::PATCH, &format!("/api/care-team/{id}"))
                .json(patch),
        )
        .await
    }
    /// Delete a member and the release templates for which it is the discloser.
    pub async fn delete_care_team_member(&self, id: uuid::Uuid) -> Result<(), CareTeamError> {
        response(self.care_request(reqwest::Method::DELETE, &format!("/api/care-team/{id}")))
            .await?;
        Ok(())
    }
    /// Refresh the public snapshot while preserving patient labels.
    pub async fn refresh_care_team_member(
        &self,
        id: uuid::Uuid,
    ) -> Result<CareTeamMember, CareTeamError> {
        json(self.care_request(
            reqwest::Method::POST,
            &format!("/api/care-team/{id}/refresh"),
        ))
        .await
    }
    /// Create a private review template, not a legal-compliance or e-signature guarantee.
    pub async fn create_release_authorization(
        &self,
        body: &ReleaseAuthorizationRequest,
    ) -> Result<ReleaseAuthorizationView, CareTeamError> {
        json(
            self.care_request(reqwest::Method::POST, "/api/release-authorizations")
                .json(body),
        )
        .await
    }
    /// Replace editable content; omitting signed_name clears the prior typed name.
    pub async fn update_release_authorization(
        &self,
        id: uuid::Uuid,
        body: &ReleaseAuthorizationRequest,
    ) -> Result<ReleaseAuthorizationView, CareTeamError> {
        json(
            self.care_request(
                reqwest::Method::PUT,
                &format!("/api/release-authorizations/{id}"),
            )
            .json(body),
        )
        .await
    }
    /// Fetch the complete SVG review template; keep bytes out of persistent browser storage.
    pub async fn release_authorization_preview(
        &self,
        id: uuid::Uuid,
    ) -> Result<String, CareTeamError> {
        response(self.care_request(
            reqwest::Method::GET,
            &format!("/api/release-authorizations/{id}/preview"),
        ))
        .await?
        .text()
        .await
        .map_err(|_| CareTeamError::Uncertain)
    }
    /// Download an authenticated private PDF, never a public capability URL.
    pub async fn release_authorization_pdf(
        &self,
        id: uuid::Uuid,
    ) -> Result<Vec<u8>, CareTeamError> {
        Ok(response(self.care_request(
            reqwest::Method::GET,
            &format!("/api/release-authorizations/{id}/pdf"),
        ))
        .await?
        .bytes()
        .await
        .map_err(|_| CareTeamError::Uncertain)?
        .to_vec())
    }
}
