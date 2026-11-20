//! Typed client API for `"Doctors & hospitals"` portal search, Epic/MyChart
//! and Oracle Health OAuth connections, setup `"Not now"`, and phone-source
//! disconnect synchronization.

use crate::DuetClient;
use crate::health_upload::authentication_refused;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Active patient-portal vendors (`intent/duet.md` D2.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalVendor {
    /// Epic / MyChart.
    Epic,
    /// Oracle Health.
    Oracle,
}

/// Public vendor entry returned by `"Doctors & hospitals"`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalVendorView {
    /// Vendor identifier.
    pub vendor: PortalVendor,
    /// Patient-facing brand label.
    pub brand: String,
}

/// One healthcare organization returned by `"Doctors & hospitals"` search.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalOrganization {
    /// Stable directory identifier.
    pub id: String,
    /// Organization display name.
    pub name: String,
    /// Vendor resolved automatically by Duet.
    pub vendor: PortalVendor,
    /// Patient-facing brand label.
    pub brand: String,
    /// Optional location subtitle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Organization FHIR R4 base URL.
    pub fhir_base_url: String,
    /// SMART OAuth authorize endpoint.
    pub authorize_url: String,
    /// SMART OAuth token endpoint.
    pub token_url: String,
}

/// Lifecycle state of one connected organization.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum PortalConnectionState {
    /// Connected and latest refresh completed cleanly.
    Connected {
        /// Latest completed sync timestamp (RFC 3339).
        last_synced_at: Option<String>,
    },
    /// Latest import published readable items while some failed alone; retries next day.
    Partial {
        /// Latest partial sync timestamp (RFC 3339).
        last_synced_at: String,
        /// Number of failed items.
        failed_items: i32,
        /// Plain explanation.
        reason: String,
    },
    /// Connection stopped updating; shows `"Couldn't update since <day>"` with `Reconnect`.
    Broken {
        /// First failure timestamp (RFC 3339).
        failing_since: String,
        /// Plain reason.
        reason: String,
        /// Whether patient re-authorization is required.
        requires_reconnect: bool,
    },
    /// Disconnected; stops new imports while preserving imported history.
    Disconnected {
        /// Disconnect timestamp (RFC 3339).
        disconnected_at: String,
    },
}

/// Patient-facing view of one portal organization connection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalConnectionView {
    /// Stable connection UUID.
    pub id: Uuid,
    /// Directory organization id.
    pub organization_id: String,
    /// Organization display name.
    pub organization_name: String,
    /// Vendor identifier.
    pub vendor: PortalVendor,
    /// Vendor brand label.
    pub brand: String,
    /// Exact lifecycle state.
    pub status: PortalConnectionState,
    /// `"Couldn't update since <day>"` when broken.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_copy: Option<String>,
    /// Whether `Reconnect` should be offered.
    pub can_reconnect: bool,
    /// Whether `Disconnect` should be offered.
    pub can_disconnect: bool,
    /// Granted SMART scopes.
    pub granted_scopes: Vec<String>,
    /// Latest account evidence revision reached by this connection.
    pub evidence_revision: i64,
    /// Scheduled next daily refresh timestamp (RFC 3339).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_refresh_at: Option<String>,
}

/// Post-signup connector setup panel state (`intent/duet.md` D1.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectorSetupState {
    /// Setup connector step has not been finished or deferred yet.
    Pending,
    /// Person chose `"Not now"` during setup.
    NotNow,
    /// Person connected at least one source.
    Completed,
}

/// Phone-side health stores whose disconnect state synchronizes with `duet-api`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhoneSourceKind {
    /// Apple HealthKit (`appleHealth`).
    #[serde(rename = "appleHealth")]
    AppleHealth,
    /// Android Health Connect (`healthConnect`).
    #[serde(rename = "healthConnect")]
    HealthConnect,
}

impl PhoneSourceKind {
    /// Canonical path segment and wire source kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppleHealth => "appleHealth",
            Self::HealthConnect => "healthConnect",
        }
    }
}

/// Server-synced status for one phone health source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneSourceStatusView {
    /// Phone source kind.
    pub source: PhoneSourceKind,
    /// Current source generation.
    pub generation: i32,
    /// Explicit disconnect timestamp (`Some` while disconnected; history preserved).
    pub disconnected_at: Option<String>,
    /// Patient-facing platform guidance.
    pub guidance: String,
}

/// Unified snapshot for the post-signup connector panel and `Sources & data` in `You`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalsSnapshot {
    /// Connector setup state.
    pub setup_state: ConnectorSetupState,
    /// Whether setup is complete or deferred via `"Not now"`.
    pub setup_complete: bool,
    /// Switched-on portal vendors.
    pub vendors: Vec<PortalVendorView>,
    /// Patient's organization connections.
    pub connections: Vec<PortalConnectionView>,
    /// Server-side phone health source statuses and guidance.
    pub phone_sources: Vec<PhoneSourceStatusView>,
}

/// Response from `GET /api/portals/organizations`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalOrganizationsSearchResponse {
    /// Currently switched-on portal vendors.
    pub vendors: Vec<PortalVendorView>,
    /// Matching organizations across switched-on vendors.
    pub organizations: Vec<PortalOrganization>,
}

/// Response from `POST /api/portals/setup/not-now`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupStatusResponse {
    /// Persisted setup state.
    pub setup_state: ConnectorSetupState,
    /// Always `true`.
    pub setup_complete: bool,
}

/// Response from `POST /api/portals/oauth/start`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OAuthStartResponse {
    /// Anti-CSRF state parameter bound to the signed-in account.
    pub state: String,
    /// SMART on FHIR authorize URL with PKCE `S256` challenge.
    pub authorize_url: String,
    /// Organization id.
    pub organization_id: String,
    /// Organization name.
    pub organization_name: String,
    /// Resolved vendor.
    pub vendor: PortalVendor,
    /// Vendor brand label.
    pub brand: String,
}

/// One record that failed alone during a portal import.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemImportFailure {
    /// Upstream record identity.
    pub record_id: String,
    /// Plain reason.
    pub reason: String,
}

/// Honest outcome of a portal import or refresh.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ImportOutcome {
    /// All items published cleanly.
    Complete {
        /// Accepted record count.
        accepted: usize,
    },
    /// Readable items published while some failed alone; retries next day.
    Partial {
        /// Accepted record count.
        accepted: usize,
        /// Per-item failure reasons.
        failed: Vec<ItemImportFailure>,
    },
    /// Whole-connection refresh failed.
    Failed {
        /// Plain reason.
        reason: String,
        /// Whether `Reconnect` is required.
        requires_reconnect: bool,
    },
}

/// Response from `POST /api/portals/oauth/complete` and `POST /api/portals/connections/{id}/refresh`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalSyncResponse {
    /// Updated connection view.
    pub connection: PortalConnectionView,
    /// Honest import outcome.
    pub import: ImportOutcome,
}

/// Typed failure from a portal or phone-source HTTP request.
#[derive(Debug, thiserror::Error)]
pub enum PortalClientError {
    /// Authentication was refused (401/403).
    #[error("session refused by Duet ({0})")]
    SessionRefused(u16),
    /// The target vendor is switched off for new sign-ins (`D2.12`).
    #[error("portal vendor is currently switched off for new sign-ins")]
    VendorDisabled,
    /// Conflict returned by the server (e.g. invalid OAuth state or disconnected link).
    #[error("portal request conflict: {0}")]
    Conflict(String),
    /// Target organization or connection was not found.
    #[error("portal resource not found")]
    NotFound,
    /// Network or transport failure.
    #[error("portal request failed: {0}")]
    Transport(#[from] reqwest::Error),
    /// Server returned an unexpected status code or undecodable payload.
    #[error("portal request failed ({status}): {detail}")]
    Http {
        /// HTTP status code.
        status: u16,
        /// Diagnostic body or message.
        detail: String,
    },
}

impl DuetClient {
    /// Read the unified setup + `Sources & data` connector snapshot.
    pub async fn portals(&self) -> Result<PortalsSnapshot, PortalClientError> {
        self.portal_get("/api/portals", &[]).await
    }

    /// Search `"Doctors & hospitals"` organizations across switched-on vendors.
    pub async fn search_portal_organizations(
        &self,
        query: &str,
    ) -> Result<PortalOrganizationsSearchResponse, PortalClientError> {
        self.portal_get("/api/portals/organizations", &[("q", query)])
            .await
    }

    /// Persist `"Not now"` on the post-signup connector panel.
    pub async fn skip_connector_setup_not_now(
        &self,
    ) -> Result<SetupStatusResponse, PortalClientError> {
        self.portal_post_empty("/api/portals/setup/not-now").await
    }

    /// Begin a SMART on FHIR OAuth sign-in for one organization.
    pub async fn start_portal_oauth(
        &self,
        organization_id: &str,
        redirect_uri: &str,
    ) -> Result<OAuthStartResponse, PortalClientError> {
        self.portal_post_json(
            "/api/portals/oauth/start",
            &serde_json::json!({
                "organization_id": organization_id,
                "redirect_uri": redirect_uri,
            }),
        )
        .await
    }

    /// Complete a SMART on FHIR OAuth sign-in and run the initial history import.
    pub async fn complete_portal_oauth(
        &self,
        state: &str,
        code: &str,
        redirect_uri: &str,
    ) -> Result<PortalSyncResponse, PortalClientError> {
        self.portal_post_json(
            "/api/portals/oauth/complete",
            &serde_json::json!({
                "state": state,
                "code": code,
                "redirect_uri": redirect_uri,
            }),
        )
        .await
    }

    /// Trigger an immediate refresh for one connected portal organization.
    pub async fn refresh_portal_connection(
        &self,
        id: Uuid,
    ) -> Result<PortalSyncResponse, PortalClientError> {
        self.portal_post_empty(&format!("/api/portals/connections/{id}/refresh"))
            .await
    }

    /// Disconnect one portal organization without erasing imported history.
    pub async fn disconnect_portal_connection(
        &self,
        id: Uuid,
    ) -> Result<PortalConnectionView, PortalClientError> {
        self.portal_post_empty(&format!("/api/portals/connections/{id}/disconnect"))
            .await
    }

    /// Read the current generation for a phone health source (`appleHealth` or `healthConnect`).
    pub async fn phone_source_generation(
        &self,
        source: PhoneSourceKind,
    ) -> Result<i32, PortalClientError> {
        self.portal_get(
            &format!("/api/phone-sources/{}/generation", source.as_str()),
            &[],
        )
        .await
    }

    /// Notify `duet-api` that a phone health source is connected (clears `disconnected_at`).
    pub async fn connect_phone_source(
        &self,
        source: PhoneSourceKind,
    ) -> Result<PhoneSourceStatusView, PortalClientError> {
        self.portal_post_empty(&format!("/api/phone-sources/{}/connect", source.as_str()))
            .await
    }

    /// Synchronize a phone health source disconnect to `duet-api` while keeping imported history.
    pub async fn disconnect_phone_source(
        &self,
        source: PhoneSourceKind,
    ) -> Result<PhoneSourceStatusView, PortalClientError> {
        self.portal_post_empty(&format!(
            "/api/phone-sources/{}/disconnect",
            source.as_str()
        ))
        .await
    }

    async fn portal_get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, PortalClientError> {
        let mut req = self
            .configuration
            .client
            .get(format!("{}{path}", self.configuration.base_path));
        if !query.is_empty() {
            req = req.query(query);
        }
        if let Some(token) = &self.configuration.bearer_access_token {
            req = req.bearer_auth(token);
        }
        let response = req.send().await?;
        self.decode_portal_response(response).await
    }

    async fn portal_post_empty<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, PortalClientError> {
        let mut req = self
            .configuration
            .client
            .post(format!("{}{path}", self.configuration.base_path));
        if let Some(token) = &self.configuration.bearer_access_token {
            req = req.bearer_auth(token);
        }
        let response = req.send().await?;
        self.decode_portal_response(response).await
    }

    async fn portal_post_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<T, PortalClientError> {
        let mut req = self
            .configuration
            .client
            .post(format!("{}{path}", self.configuration.base_path))
            .json(body);
        if let Some(token) = &self.configuration.bearer_access_token {
            req = req.bearer_auth(token);
        }
        let response = req.send().await?;
        self.decode_portal_response(response).await
    }

    async fn decode_portal_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, PortalClientError> {
        let status = response.status();
        let text = response.text().await?;
        if status.is_success() {
            return serde_json::from_str(&text).map_err(|error| PortalClientError::Http {
                status: status.as_u16(),
                detail: format!("invalid response JSON: {error}"),
            });
        }
        self.observe_refusal(status, &text);
        if authentication_refused(status.as_u16()) {
            return Err(PortalClientError::SessionRefused(status.as_u16()));
        }
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(PortalClientError::NotFound);
        }
        if status == reqwest::StatusCode::CONFLICT {
            if text.contains("portal_vendor_disabled") {
                return Err(PortalClientError::VendorDisabled);
            }
            return Err(PortalClientError::Conflict(text));
        }
        Err(PortalClientError::Http {
            status: status.as_u16(),
            detail: text,
        })
    }
}
