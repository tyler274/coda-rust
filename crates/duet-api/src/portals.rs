//! Epic/MyChart and Oracle Health portal connectors, `"Doctors & hospitals"`
//! organization search, independent daily refresh, vendor kill-switch, setup
//! `"Not now"` persistence, and phone-source disconnect synchronization.
//!
//! Every account-owned query runs through SeaORM entities ([`crate::portal_entity`])
//! inside [`AccountDatabase::begin`] so PostgreSQL row-level security stays
//! enforced. Portal records publish only through [`EvidenceStore::ingest_batch_with_authority`]
//! under [`IngestAuthority::Connector`]; one unreadable record fails alone in
//! its own savepoint while sibling records commit (`intent/duet.md` D2.4).

use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::Engine;
use chrono::{DateTime, Datelike, Duration, Utc};
use duet_wire::models::addressed_record_revision::Entity as EvidenceEntity;
use rand::Rng;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QueryTrait};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::ApiError;
use crate::database::{AccountDatabase, PortalRefreshQueueRow, orm};
use crate::durable::DurableState;
use crate::evidence::{
    EvidenceBatch, EvidenceItemOutcome, EvidenceMutation, EvidenceOperation, EvidenceStore,
    IngestAuthority, MAX_EVIDENCE_BATCH_ITEMS,
};
use crate::identity::{AccountContext, check_current};
use crate::portal_entity::{connection, oauth_state, user_connectors};

/// Default daily refresh cadence for connected portals (`intent/duet.md` D2.7).
pub const DAILY_REFRESH_INTERVAL: Duration = Duration::hours(24);
/// Lifetime of a single-use OAuth PKCE state handshake.
pub const OAUTH_STATE_TTL: Duration = Duration::minutes(15);

/// Active patient-portal vendors (`intent/duet.md` D2.1; `healow` stays parked).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalVendor {
    /// Epic MyChart SMART on FHIR portal.
    Epic,
    /// Oracle Health (Cerner) SMART on FHIR portal.
    Oracle,
}

impl PortalVendor {
    /// Patient-facing brand label shown in `"Doctors & hospitals"`.
    pub const fn brand(self) -> &'static str {
        match self {
            Self::Epic => "Epic / MyChart",
            Self::Oracle => "Oracle Health",
        }
    }

    /// Canonical `health_evidence.source_kind` owned by this connector.
    pub const fn source_kind(self) -> &'static str {
        match self {
            Self::Epic => "epic",
            Self::Oracle => "oracleHealth",
        }
    }

    /// Stable wire and database token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Epic => "epic",
            Self::Oracle => "oracle",
        }
    }

    /// Parse a stored or wire vendor token once at the boundary.
    pub fn parse(raw: &str) -> Result<Self, ApiError> {
        match raw {
            "epic" => Ok(Self::Epic),
            "oracle" => Ok(Self::Oracle),
            _ => Err(ApiError::Unprocessable("unsupported portal vendor")),
        }
    }

    /// All supported portal vendors in display order.
    pub const fn all() -> [Self; 2] {
        [Self::Epic, Self::Oracle]
    }
}

/// Explicit operator switch for one portal vendor (`intent/duet.md` D2.12).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VendorToggle {
    /// Appears in `"Doctors & hospitals"` search and allows new sign-ins.
    Enabled,
    /// Hidden from search and blocks new sign-ins; existing connections keep
    /// refreshing and can always disconnect.
    Disabled,
}

/// Operator kill-switch configuration across supported portal vendors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VendorSwitches {
    /// Epic / MyChart availability for search and new sign-ins.
    pub epic: VendorToggle,
    /// Oracle Health availability for search and new sign-ins.
    pub oracle: VendorToggle,
}

impl Default for VendorSwitches {
    fn default() -> Self {
        Self {
            epic: VendorToggle::Enabled,
            oracle: VendorToggle::Enabled,
        }
    }
}

impl VendorSwitches {
    /// Read vendor switches from environment (`DUET_PORTAL_EPIC_ENABLED`,
    /// `DUET_PORTAL_ORACLE_ENABLED`). Unset defaults to enabled.
    pub fn from_env(read: impl Fn(&str) -> Option<String>) -> Self {
        let toggle = |key: &str| match read(key).as_deref().map(str::trim) {
            Some("0" | "false" | "FALSE" | "off" | "OFF" | "disabled") => VendorToggle::Disabled,
            _ => VendorToggle::Enabled,
        };
        Self {
            epic: toggle("DUET_PORTAL_EPIC_ENABLED"),
            oracle: toggle("DUET_PORTAL_ORACLE_ENABLED"),
        }
    }

    /// Single-owner check for whether `vendor` is switched on for search and new sign-ins.
    pub const fn is_enabled(self, vendor: PortalVendor) -> bool {
        match vendor {
            PortalVendor::Epic => matches!(self.epic, VendorToggle::Enabled),
            PortalVendor::Oracle => matches!(self.oracle, VendorToggle::Enabled),
        }
    }

    /// Vendors currently switched on, in display order.
    pub fn enabled_vendors(self) -> Vec<PortalVendorView> {
        PortalVendor::all()
            .into_iter()
            .filter(|vendor| self.is_enabled(*vendor))
            .map(|vendor| PortalVendorView {
                vendor,
                brand: vendor.brand().to_owned(),
            })
            .collect()
    }
}

/// Public vendor entry returned to `"Doctors & hospitals"`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalVendorView {
    /// Vendor identifier.
    pub vendor: PortalVendor,
    /// Patient-facing vendor brand.
    pub brand: String,
}

/// One healthcare organization discoverable in `"Doctors & hospitals"`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalOrganization {
    /// Stable directory identifier.
    pub id: String,
    /// Organization display name.
    pub name: String,
    /// Portal vendor resolved automatically by Duet (`intent/duet.md` D2.1).
    pub vendor: PortalVendor,
    /// Patient-facing vendor brand label.
    pub brand: String,
    /// City/state or regional subtitle shown in search results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// FHIR R4 base URL (`aud` parameter for SMART on FHIR).
    pub fhir_base_url: String,
    /// SMART OAuth authorization endpoint.
    pub authorize_url: String,
    /// SMART OAuth token endpoint.
    pub token_url: String,
}

/// Directory owner for `"Doctors & hospitals"` search.
pub trait PortalDirectory: Send + Sync {
    /// Search organizations by name or location, restricted strictly to switched-on vendors.
    fn search(&self, query: &str, switches: VendorSwitches) -> Vec<PortalOrganization>;
    /// Look up one organization by its stable directory id regardless of vendor switch.
    fn get(&self, organization_id: &str) -> Option<PortalOrganization>;
}

/// Built-in directory covering multiple Epic/MyChart and Oracle Health organizations.
#[derive(Clone, Debug)]
pub struct StaticPortalDirectory {
    organizations: Vec<PortalOrganization>,
}

impl Default for StaticPortalDirectory {
    fn default() -> Self {
        Self::new(default_portal_organizations())
    }
}

impl StaticPortalDirectory {
    /// Construct a directory from a caller-provided organization list.
    pub fn new(mut organizations: Vec<PortalOrganization>) -> Self {
        organizations.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.id.cmp(&b.id)));
        Self { organizations }
    }
}

impl PortalDirectory for StaticPortalDirectory {
    fn search(&self, query: &str, switches: VendorSwitches) -> Vec<PortalOrganization> {
        let tokens: Vec<String> = query
            .split_whitespace()
            .map(str::to_ascii_lowercase)
            .filter(|t| !t.is_empty())
            .collect();
        self.organizations
            .iter()
            .filter(|org| switches.is_enabled(org.vendor))
            .filter(|org| {
                if tokens.is_empty() {
                    return true;
                }
                let haystack = format!(
                    "{} {} {}",
                    org.name.to_ascii_lowercase(),
                    org.location
                        .as_deref()
                        .unwrap_or_default()
                        .to_ascii_lowercase(),
                    org.brand.to_ascii_lowercase()
                );
                tokens.iter().all(|token| haystack.contains(token))
            })
            .cloned()
            .collect()
    }

    fn get(&self, organization_id: &str) -> Option<PortalOrganization> {
        self.organizations
            .iter()
            .find(|org| org.id == organization_id)
            .cloned()
    }
}

/// Built-in representative Epic/MyChart and Oracle Health organizations.
pub fn default_portal_organizations() -> Vec<PortalOrganization> {
    let epic = |id: &str, name: &str, location: &str, base: &str| PortalOrganization {
        id: id.to_owned(),
        name: name.to_owned(),
        vendor: PortalVendor::Epic,
        brand: PortalVendor::Epic.brand().to_owned(),
        location: Some(location.to_owned()),
        fhir_base_url: format!("{base}/api/FHIR/R4"),
        authorize_url: format!("{base}/oauth2/authorize"),
        token_url: format!("{base}/oauth2/token"),
    };
    let oracle = |id: &str, name: &str, location: &str, base: &str| PortalOrganization {
        id: id.to_owned(),
        name: name.to_owned(),
        vendor: PortalVendor::Oracle,
        brand: PortalVendor::Oracle.brand().to_owned(),
        location: Some(location.to_owned()),
        fhir_base_url: format!("{base}/r4"),
        authorize_url: format!("{base}/tenants/oauth/authorize"),
        token_url: format!("{base}/tenants/oauth/token"),
    };
    vec![
        epic(
            "epic-ucsf-health",
            "UCSF Health",
            "San Francisco, CA",
            "https://unified-api.ucsf.edu/clinical/apex",
        ),
        epic(
            "epic-stanford-health-care",
            "Stanford Health Care",
            "Palo Alto, CA",
            "https://sfd.stanfordmed.org/FHIR",
        ),
        epic(
            "epic-mayo-clinic",
            "Mayo Clinic",
            "Rochester, MN",
            "https://fhir.mayoclinic.org/interconnect",
        ),
        epic(
            "epic-mass-general-brigham",
            "Mass General Brigham",
            "Boston, MA",
            "https://ws-interconnect-fhir.partners.org/Interconnect-FHIR-PRD",
        ),
        epic(
            "epic-nyu-langone",
            "NYU Langone Health",
            "New York, NY",
            "https://epicfhir.nyulangone.org/FHIRPRD",
        ),
        oracle(
            "oracle-adventhealth",
            "AdventHealth",
            "Altamonte Springs, FL",
            "https://fhir-myrecord.cerner.com/r4/adventhealth",
        ),
        oracle(
            "oracle-intermountain-health",
            "Intermountain Health",
            "Salt Lake City, UT",
            "https://fhir-myrecord.cerner.com/r4/intermountain",
        ),
        oracle(
            "oracle-banner-health",
            "Banner Health",
            "Phoenix, AZ",
            "https://fhir-myrecord.cerner.com/r4/banner",
        ),
        oracle(
            "oracle-baylor-scott-white",
            "Baylor Scott & White Health",
            "Dallas, TX",
            "https://fhir-myrecord.cerner.com/r4/bswh",
        ),
    ]
}

/// Lifecycle state of one connected healthcare organization (`intent/duet.md` D2.2, D2.4, D2.8, D2.11).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum PortalConnectionState {
    /// Connected and latest refresh completed cleanly.
    Connected {
        /// Latest completed sync timestamp.
        last_synced_at: Option<DateTime<Utc>>,
    },
    /// Connected, latest import finished partially; readable records are published
    /// and the next daily refresh retries.
    Partial {
        /// Latest partial sync timestamp.
        last_synced_at: DateTime<Utc>,
        /// Number of unreadable/rejected records.
        failed_items: i32,
        /// Plain reason describing the partial gap.
        reason: String,
    },
    /// Connection stopped working; shows `"Couldn't update since <day>"` with `Reconnect`.
    Broken {
        /// First failure timestamp for this broken stretch (preserved across retries).
        failing_since: DateTime<Utc>,
        /// Plain reason why updating failed.
        reason: String,
        /// Whether patient re-authorization is needed.
        requires_reconnect: bool,
    },
    /// Explicitly disconnected by the person; imports stopped while history stays.
    Disconnected {
        /// When the person disconnected this organization.
        disconnected_at: DateTime<Utc>,
    },
}

/// Format `"Couldn't update since <day>"` from a UTC timestamp (e.g. `"Nov 19"`).
pub fn format_update_failure_day(at: DateTime<Utc>) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let month = MONTHS[usize::try_from(at.month0()).unwrap_or(0)];
    format!("Couldn't update since {month} {}", at.day())
}

/// Patient-facing view of one organization connection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalConnectionView {
    /// Stable connection UUID reused across disconnect/reconnect.
    pub id: Uuid,
    /// Directory organization id.
    pub organization_id: String,
    /// Organization display name.
    pub organization_name: String,
    /// Resolved vendor (`epic` or `oracle`).
    pub vendor: PortalVendor,
    /// Vendor brand label (`Epic / MyChart` or `Oracle Health`).
    pub brand: String,
    /// Exact lifecycle state.
    pub status: PortalConnectionState,
    /// Patient-facing failure status copy (`"Couldn't update since <day>"` when broken).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_copy: Option<String>,
    /// Whether the row offers `Reconnect`.
    pub can_reconnect: bool,
    /// Whether the row offers `Disconnect` (true for every non-disconnected link).
    pub can_disconnect: bool,
    /// Granted SMART scopes.
    pub granted_scopes: Vec<String>,
    /// Latest account evidence revision reached by this connection.
    pub evidence_revision: i64,
    /// Next scheduled daily refresh timestamp (`None` when disconnected).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_refresh_at: Option<DateTime<Utc>>,
}

impl PortalConnectionView {
    fn from_model(model: connection::Model) -> Result<Self, ApiError> {
        let vendor = PortalVendor::parse(&model.vendor)?;
        let status = match model.state.as_str() {
            "connected" => PortalConnectionState::Connected {
                last_synced_at: model.last_synced_at,
            },
            "partial" => PortalConnectionState::Partial {
                last_synced_at: model.last_synced_at.unwrap_or(model.updated_at),
                failed_items: model.partial_failed_count,
                reason: model
                    .failure_reason
                    .unwrap_or_else(|| "Some records could not be read.".to_owned()),
            },
            "broken" => PortalConnectionState::Broken {
                failing_since: model.failing_since.unwrap_or(model.updated_at),
                reason: model
                    .failure_reason
                    .unwrap_or_else(|| "Portal connection could not update.".to_owned()),
                requires_reconnect: model.requires_reconnect,
            },
            "disconnected" => PortalConnectionState::Disconnected {
                disconnected_at: model.disconnected_at.unwrap_or(model.updated_at),
            },
            _ => {
                return Err(ApiError::Storage(
                    "invalid stored portal connection state".into(),
                ));
            }
        };
        let (status_copy, can_reconnect, can_disconnect) = match &status {
            PortalConnectionState::Broken { failing_since, .. } => {
                (Some(format_update_failure_day(*failing_since)), true, true)
            }
            PortalConnectionState::Disconnected { .. } => (None, true, false),
            PortalConnectionState::Connected { .. } | PortalConnectionState::Partial { .. } => {
                (None, false, true)
            }
        };
        Ok(Self {
            id: model.id,
            organization_id: model.organization_id,
            organization_name: model.organization_name,
            vendor,
            brand: model.brand,
            status,
            status_copy,
            can_reconnect,
            can_disconnect,
            granted_scopes: model.granted_scopes,
            evidence_revision: model.evidence_revision,
            next_refresh_at: model.next_refresh_at,
        })
    }
}

/// Post-signup connector setup panel state (`intent/duet.md` D1.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectorSetupState {
    /// Setup connector step has not yet been completed or deferred.
    Pending,
    /// Person chose `"Not now"` during setup.
    NotNow,
    /// Person connected at least one portal or phone health source.
    Completed,
}

impl ConnectorSetupState {
    fn from_db(raw: &str) -> Result<Self, ApiError> {
        match raw {
            "pending" => Ok(Self::Pending),
            "skipped_not_now" => Ok(Self::NotNow),
            "completed" => Ok(Self::Completed),
            _ => Err(ApiError::Storage("invalid connector setup state".into())),
        }
    }

    const fn as_db(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::NotNow => "skipped_not_now",
            Self::Completed => "completed",
        }
    }
}

/// Phone-side health stores whose disconnect state synchronizes with `duet-api`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhoneSourceKind {
    /// iOS Apple Health (`appleHealth`).
    #[serde(rename = "appleHealth")]
    AppleHealth,
    /// Android Health Connect (`healthConnect`).
    #[serde(rename = "healthConnect")]
    HealthConnect,
}

impl PhoneSourceKind {
    /// Parse a path segment into a typed phone source.
    pub fn parse(raw: &str) -> Result<Self, ApiError> {
        match raw {
            "appleHealth" | "apple-health" => Ok(Self::AppleHealth),
            "healthConnect" | "health-connect" => Ok(Self::HealthConnect),
            _ => Err(ApiError::Unprocessable("unsupported phone health source")),
        }
    }

    /// Canonical wire source kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppleHealth => "appleHealth",
            Self::HealthConnect => "healthConnect",
        }
    }
}

/// Server-synced status of one phone health source (`intent/duet.md` D2.3, D2.11, D2.14).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneSourceStatusView {
    /// Source kind (`appleHealth` or `healthConnect`).
    pub source: PhoneSourceKind,
    /// Current source generation (unchanged on plain disconnect; bumped only on privacy withdraw).
    pub generation: i32,
    /// Whether the source is explicitly disconnected on the server (`disconnected_at.is_some()`).
    pub disconnected_at: Option<DateTime<Utc>>,
    /// Patient-facing platform guidance (wearable-app linking or Health Connect background limit).
    pub guidance: String,
}

/// Unified snapshot backing both the post-signup setup panel (`"Not now"`) and `Sources & data` in `You`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalsSnapshot {
    /// Setup panel status (`pending`, `not_now`, `completed`).
    pub setup_state: ConnectorSetupState,
    /// Whether post-signup setup has been completed or deferred via `"Not now"`.
    pub setup_complete: bool,
    /// Currently switched-on portal vendors shown in `"Doctors & hospitals"`.
    pub vendors: Vec<PortalVendorView>,
    /// Patient's connected, broken, partial, and disconnected organizations.
    pub connections: Vec<PortalConnectionView>,
    /// Server-side status and guidance for Apple Health and Health Connect.
    pub phone_sources: Vec<PhoneSourceStatusView>,
}

/// Search response for `"Doctors & hospitals"`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalOrganizationsSearchResponse {
    /// Currently switched-on vendors.
    pub vendors: Vec<PortalVendorView>,
    /// Matching organizations across switched-on vendors.
    pub organizations: Vec<PortalOrganization>,
}

/// Response after persisting `"Not now"` on the post-signup connector panel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupStatusResponse {
    /// Updated setup status (`not_now`).
    pub setup_state: ConnectorSetupState,
    /// Always `true` once `"Not now"` or a connection succeeds.
    pub setup_complete: bool,
}

/// Request to start a SMART on FHIR OAuth authorization flow for one organization.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OAuthStartRequest {
    /// Directory organization id selected from `"Doctors & hospitals"`.
    pub organization_id: String,
    /// App redirect URI registered for the OAuth callback.
    pub redirect_uri: String,
}

/// Response returned to open the organization's SMART OAuth sign-in sheet.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OAuthStartResponse {
    /// Single-use anti-CSRF state bound to this authenticated account.
    pub state: String,
    /// Full SMART on FHIR authorize URL including PKCE `S256` challenge and `aud`.
    pub authorize_url: String,
    /// Target organization id.
    pub organization_id: String,
    /// Target organization display name.
    pub organization_name: String,
    /// Resolved vendor.
    pub vendor: PortalVendor,
    /// Patient-facing vendor brand.
    pub brand: String,
}

/// Request to complete a SMART on FHIR OAuth authorization flow.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OAuthCompleteRequest {
    /// Single-use state returned by `POST /api/portals/oauth/start`.
    pub state: String,
    /// Authorization code returned by the portal's OAuth callback.
    pub code: String,
    /// Exact redirect URI presented in `start`.
    pub redirect_uri: String,
}

/// One record that could not be read or ingested during a portal import (`intent/duet.md` D2.4).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemImportFailure {
    /// Upstream record identifier.
    pub record_id: String,
    /// Plain reason this item failed alone.
    pub reason: String,
}

/// Honest outcome of one portal import or daily refresh (`intent/duet.md` D2.4).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ImportOutcome {
    /// Every readable item published with zero unreadable or rejected items.
    Complete {
        /// Count of accepted records.
        accepted: usize,
    },
    /// Readable items published while one or more items failed alone; next daily refresh retries.
    Partial {
        /// Count of accepted records.
        accepted: usize,
        /// Per-item failure reasons.
        failed: Vec<ItemImportFailure>,
    },
    /// Whole-connection refresh failed (never treated as deletion).
    Failed {
        /// Plain reason why the refresh failed.
        reason: String,
        /// Whether patient re-authorization (`Reconnect`) is needed.
        requires_reconnect: bool,
    },
}

/// Response returned after completing OAuth or triggering a connection refresh.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalSyncResponse {
    /// Updated connection view.
    pub connection: PortalConnectionView,
    /// Honest outcome of the import pass.
    pub import: ImportOutcome,
}

/// OAuth token set returned by a portal vendor token endpoint.
#[derive(Clone, PartialEq, Eq)]
pub struct PortalTokenSet {
    /// Bearer access token for FHIR R4 requests.
    pub access_token: String,
    /// Optional refresh token for daily background refresh.
    pub refresh_token: Option<String>,
    /// Access token lifetime in seconds, if stated.
    pub expires_in_seconds: Option<i64>,
    /// Granted SMART scopes.
    pub granted_scopes: Vec<String>,
}

impl std::fmt::Debug for PortalTokenSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PortalTokenSet")
            .field("access_token", &"<redacted>")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "<redacted>"),
            )
            .field("expires_in_seconds", &self.expires_in_seconds)
            .field("granted_scopes", &self.granted_scopes)
            .finish()
    }
}

/// One record yielded by a portal FHIR fetch page.
#[derive(Clone, Debug)]
pub enum PortalFetchedItem {
    /// Valid canonical mutation ready for `EvidenceStore::ingest_batch_with_authority`.
    Readable(EvidenceMutation),
    /// Unreadable upstream item that must fail alone without aborting siblings (`D2.4`).
    Unreadable {
        /// Upstream record identity.
        record_id: String,
        /// Plain reason the record could not be parsed.
        reason: String,
    },
}

/// One bounded page of portal FHIR records.
#[derive(Clone, Debug, Default)]
pub struct PortalFetchPage {
    /// Readable and unreadable items on this page.
    pub items: Vec<PortalFetchedItem>,
    /// Continuation cursor for the next page, if any.
    pub next_cursor: Option<String>,
    /// Whether more pages remain.
    pub has_more: bool,
}

/// Typed failure from a portal OAuth or FHIR HTTP exchange.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PortalFetchError {
    /// Token was revoked, expired without refresh, or refused with 401/403.
    ReauthorizationRequired {
        /// Plain failure reason.
        reason: String,
    },
    /// Upstream portal timed out, returned 5xx, or was unreachable.
    UpstreamUnavailable {
        /// Plain failure reason.
        reason: String,
    },
    /// Upstream returned a malformed OAuth or FHIR payload.
    InvalidResponse {
        /// Plain failure reason.
        reason: String,
    },
}

impl std::fmt::Display for PortalFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReauthorizationRequired { reason } => {
                write!(f, "portal reauthorization required: {reason}")
            }
            Self::UpstreamUnavailable { reason } => {
                write!(f, "portal temporarily unavailable: {reason}")
            }
            Self::InvalidResponse { reason } => {
                write!(f, "portal returned an unreadable response: {reason}")
            }
        }
    }
}

impl std::error::Error for PortalFetchError {}

/// Pluggable SMART on FHIR OAuth and record-fetch client for Epic/MyChart and Oracle Health.
#[async_trait]
pub trait PortalFhirClient: Send + Sync {
    /// Exchange a SMART on FHIR authorization code + PKCE verifier for tokens.
    async fn exchange_code(
        &self,
        organization: &PortalOrganization,
        code: &str,
        code_verifier: &str,
        redirect_uri: &str,
    ) -> Result<PortalTokenSet, PortalFetchError>;

    /// Refresh an expiring access token using the stored refresh token.
    async fn refresh_token(
        &self,
        organization: &PortalOrganization,
        refresh_token: &str,
    ) -> Result<PortalTokenSet, PortalFetchError>;

    /// Fetch one bounded page of FHIR records for this connection.
    async fn fetch_records(
        &self,
        organization: &PortalOrganization,
        connection_id: Uuid,
        access_token: &str,
        cursor: Option<&str>,
    ) -> Result<PortalFetchPage, PortalFetchError>;
}

/// Production HTTP SMART on FHIR client for Epic/MyChart and Oracle Health.
#[derive(Clone)]
pub struct HttpPortalFhirClient {
    http: reqwest::Client,
    epic_client_id: String,
    oracle_client_id: String,
}

/// Request timeout for outbound SMART on FHIR OAuth and FHIR R4 HTTP calls.
const PORTAL_HTTP_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// Validate that an initial FHIR query or pagination `next` link stays strictly
/// on HTTPS and on the exact origin `(scheme, host, port)` of `fhir_base_url`
/// with no embedded credentials, preventing SSRF and bearer token exfiltration.
pub(crate) fn validated_fhir_page_url(
    fhir_base_url: &str,
    cursor: Option<&str>,
) -> Result<reqwest::Url, PortalFetchError> {
    let base =
        reqwest::Url::parse(fhir_base_url).map_err(|_| PortalFetchError::InvalidResponse {
            reason: "invalid organization FHIR base URL".to_owned(),
        })?;
    if base.scheme() != "https" || base.host_str().is_none() {
        return Err(PortalFetchError::InvalidResponse {
            reason: "organization FHIR base URL must use HTTPS".to_owned(),
        });
    }
    match cursor {
        None => {
            let trimmed = fhir_base_url.trim_end_matches('/');
            reqwest::Url::parse(&format!("{trimmed}/Observation?_count=50")).map_err(|_| {
                PortalFetchError::InvalidResponse {
                    reason: "invalid initial FHIR Observation URL".to_owned(),
                }
            })
        }
        Some(next) => {
            let parsed =
                reqwest::Url::parse(next).map_err(|_| PortalFetchError::InvalidResponse {
                    reason: "portal pagination link was not a valid URL".to_owned(),
                })?;
            let same_origin = parsed.scheme() == "https"
                && parsed.scheme() == base.scheme()
                && parsed.host_str() == base.host_str()
                && parsed.port_or_known_default() == base.port_or_known_default()
                && parsed.username().is_empty()
                && parsed.password().is_none();
            if !same_origin {
                return Err(PortalFetchError::InvalidResponse {
                    reason: "portal pagination link origin mismatch".to_owned(),
                });
            }
            Ok(parsed)
        }
    }
}

impl HttpPortalFhirClient {
    /// Construct an HTTP portal client with vendor SMART client ids, disabled
    /// redirects (preventing cross-origin bearer forwarding), and a 30s timeout.
    pub fn new(epic_client_id: impl Into<String>, oracle_client_id: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(PORTAL_HTTP_TIMEOUT)
            .build()
            .expect("static reqwest client builder configuration is valid");
        Self {
            http,
            epic_client_id: epic_client_id.into(),
            oracle_client_id: oracle_client_id.into(),
        }
    }

    fn client_id(&self, vendor: PortalVendor) -> &str {
        match vendor {
            PortalVendor::Epic => &self.epic_client_id,
            PortalVendor::Oracle => &self.oracle_client_id,
        }
    }
}

#[derive(Deserialize)]
struct OAuthTokenWire {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    expires_in: Option<i64>,
    #[serde(default)]
    scope: Option<String>,
}

#[async_trait]
impl PortalFhirClient for HttpPortalFhirClient {
    async fn exchange_code(
        &self,
        organization: &PortalOrganization,
        code: &str,
        code_verifier: &str,
        redirect_uri: &str,
    ) -> Result<PortalTokenSet, PortalFetchError> {
        let client_id = self.client_id(organization.vendor);
        if client_id.trim().is_empty() {
            return Err(PortalFetchError::UpstreamUnavailable {
                reason: "portal OAuth client is not configured".to_owned(),
            });
        }
        let response = self
            .http
            .post(&organization.token_url)
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", redirect_uri),
                ("client_id", client_id),
                ("code_verifier", code_verifier),
            ])
            .send()
            .await
            .map_err(|_| PortalFetchError::UpstreamUnavailable {
                reason: "could not reach portal token endpoint".to_owned(),
            })?;
        parse_token_response(response).await
    }

    async fn refresh_token(
        &self,
        organization: &PortalOrganization,
        refresh_token: &str,
    ) -> Result<PortalTokenSet, PortalFetchError> {
        let client_id = self.client_id(organization.vendor);
        if client_id.trim().is_empty() {
            return Err(PortalFetchError::UpstreamUnavailable {
                reason: "portal OAuth client is not configured".to_owned(),
            });
        }
        let response = self
            .http
            .post(&organization.token_url)
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
                ("client_id", client_id),
            ])
            .send()
            .await
            .map_err(|_| PortalFetchError::UpstreamUnavailable {
                reason: "could not reach portal token endpoint".to_owned(),
            })?;
        parse_token_response(response).await
    }

    async fn fetch_records(
        &self,
        organization: &PortalOrganization,
        connection_id: Uuid,
        access_token: &str,
        cursor: Option<&str>,
    ) -> Result<PortalFetchPage, PortalFetchError> {
        let url = validated_fhir_page_url(&organization.fhir_base_url, cursor)?;
        let response = self
            .http
            .get(url)
            .bearer_auth(access_token)
            .header("Accept", "application/fhir+json")
            .send()
            .await
            .map_err(|_| PortalFetchError::UpstreamUnavailable {
                reason: "could not reach portal FHIR endpoint".to_owned(),
            })?;
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
            return Err(PortalFetchError::ReauthorizationRequired {
                reason: "portal authorization expired or was revoked".to_owned(),
            });
        }
        if !status.is_success() {
            return Err(PortalFetchError::UpstreamUnavailable {
                reason: format!("portal returned HTTP {}", status.as_u16()),
            });
        }
        let bundle: Value =
            response
                .json()
                .await
                .map_err(|_| PortalFetchError::InvalidResponse {
                    reason: "portal FHIR bundle was not valid JSON".to_owned(),
                })?;
        let next_cursor = bundle["link"].as_array().and_then(|links| {
            links.iter().find_map(|link| {
                (link["relation"].as_str() == Some("next"))
                    .then(|| link["url"].as_str().map(str::to_owned))
                    .flatten()
            })
        });
        let mut items = Vec::new();
        if let Some(entries) = bundle["entry"].as_array() {
            for entry in entries {
                let resource = &entry["resource"];
                let Some(resource_type) = resource["resourceType"].as_str() else {
                    items.push(PortalFetchedItem::Unreadable {
                        record_id: "unknown".to_owned(),
                        reason: "missing FHIR resourceType".to_owned(),
                    });
                    continue;
                };
                let Some(resource_id) = resource["id"].as_str().filter(|id| !id.trim().is_empty())
                else {
                    items.push(PortalFetchedItem::Unreadable {
                        record_id: format!("{resource_type}/missing-id"),
                        reason: "missing FHIR resource id".to_owned(),
                    });
                    continue;
                };
                let source_record_id = format!("{resource_type}/{resource_id}");
                let canonical_record_id =
                    stable_portal_uuid(organization.vendor, connection_id, &source_record_id);
                let operation_id = Uuid::new_v4();
                let concept_id = resource
                    .pointer("/code/coding/0/code")
                    .and_then(Value::as_str)
                    .map(|code| format!("fhir:{resource_type}:{code}"))
                    .unwrap_or_else(|| format!("fhir:{resource_type}"));
                let display_name = resource
                    .pointer("/code/text")
                    .or_else(|| resource.pointer("/code/coding/0/display"))
                    .and_then(Value::as_str)
                    .unwrap_or(resource_type);
                let measured_at = resource["effectiveDateTime"]
                    .as_str()
                    .or_else(|| resource["issued"].as_str());
                let mut payload = json!({
                    "concept_id": concept_id,
                    "display_name": display_name,
                    "source": {
                        "kind": organization.vendor.source_kind(),
                        "connection_id": connection_id.to_string(),
                        "record_id": source_record_id,
                        "issuer": organization.name,
                        "organization_id": organization.id,
                        "statuses": {
                            "status": resource["status"].as_str().unwrap_or("final"),
                        },
                    },
                    "source_record": {
                        "format": "fhir-json",
                        "content": resource,
                    },
                });
                if let Some(at) = measured_at {
                    payload["measured_at"] = json!(at);
                }
                items.push(PortalFetchedItem::Readable(EvidenceMutation {
                    operation_id,
                    canonical_record_id,
                    record_id: source_record_id,
                    entity: EvidenceEntity::HealthEvidence,
                    expected_revision: None,
                    source_generation: None,
                    operation: EvidenceOperation::Upsert,
                    payload,
                }));
            }
        }
        Ok(PortalFetchPage {
            items,
            has_more: next_cursor.is_some(),
            next_cursor,
        })
    }
}

async fn parse_token_response(
    response: reqwest::Response,
) -> Result<PortalTokenSet, PortalFetchError> {
    let status = response.status();
    if status == reqwest::StatusCode::UNAUTHORIZED
        || status == reqwest::StatusCode::BAD_REQUEST
        || status == reqwest::StatusCode::FORBIDDEN
    {
        return Err(PortalFetchError::ReauthorizationRequired {
            reason: "portal rejected OAuth credentials".to_owned(),
        });
    }
    if !status.is_success() {
        return Err(PortalFetchError::UpstreamUnavailable {
            reason: format!("portal token endpoint returned HTTP {}", status.as_u16()),
        });
    }
    let token: OAuthTokenWire =
        response
            .json()
            .await
            .map_err(|_| PortalFetchError::InvalidResponse {
                reason: "portal token response was not valid JSON".to_owned(),
            })?;
    let granted_scopes = token
        .scope
        .unwrap_or_default()
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    Ok(PortalTokenSet {
        access_token: token.access_token,
        refresh_token: token.refresh_token,
        expires_in_seconds: token.expires_in,
        granted_scopes,
    })
}

/// Deterministic canonical record UUID for a portal upstream record identity.
pub fn stable_portal_uuid(
    vendor: PortalVendor,
    connection_id: Uuid,
    source_record_id: &str,
) -> Uuid {
    let mut hasher = Sha256::new();
    hasher.update(b"duet-portal-record-v1:");
    hasher.update(vendor.as_str().as_bytes());
    hasher.update(b":");
    hasher.update(connection_id.as_bytes());
    hasher.update(b":");
    hasher.update(source_record_id.as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}

/// Summary of one daily refresh sweep across due connections (`intent/duet.md` D2.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshSweepReport {
    /// Number of due connections attempted.
    pub attempted: usize,
    /// Connections that finished cleanly.
    pub complete: usize,
    /// Connections that finished partially (readable records published, next-day retry scheduled).
    pub partial: usize,
    /// Connections whose refresh failed (isolated from all other connections).
    pub failed: usize,
}

/// Content-free engineering diagnostic for an overdue, partial, or broken portal connection
/// (`intent/duet.md` Scope "How it is built today").
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverduePortalConnection {
    /// Stable connection UUID.
    pub connection_id: Uuid,
    /// Portal vendor.
    pub vendor: PortalVendor,
    /// Directory organization id.
    pub organization_id: String,
    /// Current lifecycle state (`connected`, `partial`, `broken`).
    pub state: String,
    /// Latest completed or partial sync timestamp.
    pub last_synced_at: Option<DateTime<Utc>>,
    /// Scheduled refresh timestamp.
    pub next_refresh_at: Option<DateTime<Utc>>,
    /// First failure timestamp when broken.
    pub failing_since: Option<DateTime<Utc>>,
    /// Seconds past `next_refresh_at` (0 when not past schedule, e.g. broken within window).
    pub overdue_seconds: i64,
    /// Operational reason explaining why engineering attention is warranted.
    pub reason: String,
}

/// Shared service owning portal directory search, OAuth handshakes, daily refresh,
/// vendor kill-switches, overdue diagnostics, and phone-source disconnect sync.
#[derive(Clone)]
pub struct PortalService {
    database: AccountDatabase,
    evidence: EvidenceStore,
    directory: Arc<dyn PortalDirectory>,
    client: Arc<dyn PortalFhirClient>,
    switches: Arc<RwLock<VendorSwitches>>,
    epic_client_id: String,
    oracle_client_id: String,
    now: Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>,
}

impl PortalService {
    /// Construct a portal service with the built-in directory and HTTP SMART client.
    pub fn new(
        database: AccountDatabase,
        evidence: EvidenceStore,
        switches: VendorSwitches,
        epic_client_id: impl Into<String>,
        oracle_client_id: impl Into<String>,
    ) -> Self {
        let epic_client_id = epic_client_id.into();
        let oracle_client_id = oracle_client_id.into();
        let client = Arc::new(HttpPortalFhirClient::new(
            epic_client_id.clone(),
            oracle_client_id.clone(),
        ));
        Self {
            database,
            evidence,
            directory: Arc::new(StaticPortalDirectory::default()),
            client,
            switches: Arc::new(RwLock::new(switches)),
            epic_client_id,
            oracle_client_id,
            now: Arc::new(Utc::now),
        }
    }

    /// Replace the organization directory (deployments or tests).
    pub fn with_directory(mut self, directory: Arc<dyn PortalDirectory>) -> Self {
        self.directory = directory;
        self
    }

    /// Replace the OAuth/FHIR client (tests).
    pub fn with_client(mut self, client: Arc<dyn PortalFhirClient>) -> Self {
        self.client = client;
        self
    }

    /// Replace the clock (deterministic scheduler tests).
    pub fn with_clock(mut self, now: Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>) -> Self {
        self.now = now;
        self
    }

    /// Read the active vendor kill-switch configuration.
    pub fn switches(&self) -> VendorSwitches {
        self.switches.read().map(|guard| *guard).unwrap_or_default()
    }

    /// Update the active vendor kill-switch configuration at runtime.
    pub fn set_switches(&self, switches: VendorSwitches) {
        if let Ok(mut guard) = self.switches.write() {
            *guard = switches;
        }
    }

    /// Search `"Doctors & hospitals"` organizations across switched-on vendors (`D2.1`, `D2.12`).
    pub fn search_organizations(&self, query: &str) -> PortalOrganizationsSearchResponse {
        let switches = self.switches();
        PortalOrganizationsSearchResponse {
            vendors: switches.enabled_vendors(),
            organizations: self.directory.search(query, switches),
        }
    }

    /// Unified setup + `Sources & data` snapshot for an authenticated patient.
    pub async fn snapshot(&self, context: &AccountContext) -> Result<PortalsSnapshot, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let user = orm::model::<user_connectors::Model>(
            &mut tx,
            &user_connectors::Entity::find_by_id(context.account_id).into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;
        let rows: Vec<connection::Model> = orm::models(
            &mut tx,
            &connection::Entity::find()
                .filter(connection::Column::UserId.eq(context.account_id))
                .order_by_asc(connection::Column::OrganizationName)
                .order_by_asc(connection::Column::Id)
                .into_query(),
        )
        .await?;
        tx.commit().await?;

        let connections = rows
            .into_iter()
            .map(PortalConnectionView::from_model)
            .collect::<Result<Vec<_>, _>>()?;
        let setup_state = ConnectorSetupState::from_db(&user.setup_connectors_status)?;
        let phone_sources = vec![
            PhoneSourceStatusView {
                source: PhoneSourceKind::AppleHealth,
                generation: user.apple_health_generation,
                disconnected_at: user.apple_health_disconnected_at,
                guidance: APPLE_HEALTH_WEARABLE_GUIDANCE.to_owned(),
            },
            PhoneSourceStatusView {
                source: PhoneSourceKind::HealthConnect,
                generation: user.health_connect_generation,
                disconnected_at: user.health_connect_disconnected_at,
                guidance: HEALTH_CONNECT_LIMIT_GUIDANCE.to_owned(),
            },
        ];
        Ok(PortalsSnapshot {
            setup_state,
            setup_complete: user.setup_complete,
            vendors: self.switches().enabled_vendors(),
            connections,
            phone_sources,
        })
    }

    /// Persist `"Not now"` on the post-signup connector panel (`intent/duet.md` D1.4).
    pub async fn skip_setup_not_now(
        &self,
        context: &AccountContext,
    ) -> Result<SetupStatusResponse, ApiError> {
        let now = (self.now)();
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let user = orm::model::<user_connectors::Model>(
            &mut tx,
            &user_connectors::Entity::find_by_id(context.account_id).into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;
        let current = ConnectorSetupState::from_db(&user.setup_connectors_status)?;
        let next = match current {
            ConnectorSetupState::Completed => ConnectorSetupState::Completed,
            ConnectorSetupState::Pending | ConnectorSetupState::NotNow => {
                ConnectorSetupState::NotNow
            }
        };
        orm::execute(
            &mut tx,
            &user_connectors::Entity::update_many()
                .col_expr(user_connectors::Column::SetupComplete, Expr::val(true))
                .col_expr(
                    user_connectors::Column::SetupConnectorsStatus,
                    Expr::val(next.as_db()),
                )
                .col_expr(
                    user_connectors::Column::SetupConnectorsUpdatedAt,
                    Expr::val(Some(now)),
                )
                .filter(user_connectors::Column::Id.eq(context.account_id))
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(SetupStatusResponse {
            setup_state: next,
            setup_complete: true,
        })
    }

    /// Begin a SMART on FHIR OAuth sign-in for one organization (`D2.1`, `D2.2`, `D2.12`).
    pub async fn start_oauth(
        &self,
        context: &AccountContext,
        request: &OAuthStartRequest,
    ) -> Result<OAuthStartResponse, ApiError> {
        let redirect_uri = request.redirect_uri.trim();
        if redirect_uri.is_empty()
            || (!redirect_uri.starts_with("https://") && !redirect_uri.starts_with("duet://"))
        {
            return Err(ApiError::Unprocessable("valid redirect_uri is required"));
        }
        let organization = self
            .directory
            .get(request.organization_id.trim())
            .ok_or(ApiError::NotFound)?;
        if !self.switches().is_enabled(organization.vendor) {
            return Err(ApiError::Conflict("portal_vendor_disabled"));
        }

        let now = (self.now)();
        let state = random_urlsafe_token(24);
        let code_verifier = random_urlsafe_token(32);
        let code_challenge = pkce_s256_challenge(&code_verifier);
        let expires_at = now + OAUTH_STATE_TTL;

        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        orm::execute(
            &mut tx,
            &oauth_state::Entity::insert(oauth_state::ActiveModel {
                state: Set(state.clone()),
                user_id: Set(context.account_id),
                organization_id: Set(organization.id.clone()),
                vendor: Set(organization.vendor.as_str().to_owned()),
                code_verifier: Set(code_verifier),
                redirect_uri: Set(redirect_uri.to_owned()),
                created_at: Set(now),
                expires_at: Set(expires_at),
                consumed_at: Set(None),
            })
            .into_query(),
        )
        .await?;
        tx.commit().await?;

        let client_id = match organization.vendor {
            PortalVendor::Epic => &self.epic_client_id,
            PortalVendor::Oracle => &self.oracle_client_id,
        };
        let mut authorize_url = reqwest::Url::parse(&organization.authorize_url)
            .map_err(|_| ApiError::Internal("invalid portal authorize_url"))?;
        authorize_url
            .query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", client_id)
            .append_pair("redirect_uri", redirect_uri)
            .append_pair(
                "scope",
                "launch/patient openid fhirUser offline_access patient/*.read",
            )
            .append_pair("state", &state)
            .append_pair("aud", &organization.fhir_base_url)
            .append_pair("code_challenge", &code_challenge)
            .append_pair("code_challenge_method", "S256");

        Ok(OAuthStartResponse {
            state,
            authorize_url: authorize_url.to_string(),
            organization_id: organization.id,
            organization_name: organization.name,
            vendor: organization.vendor,
            brand: organization.brand,
        })
    }

    /// Complete a SMART on FHIR OAuth handshake, create or reconnect the
    /// organization row (preserving history on reconnect), and run initial import.
    pub async fn complete_oauth(
        &self,
        context: &AccountContext,
        request: &OAuthCompleteRequest,
    ) -> Result<PortalSyncResponse, ApiError> {
        if request.state.trim().is_empty() || request.code.trim().is_empty() {
            return Err(ApiError::Unprocessable("state and code are required"));
        }
        let now = (self.now)();
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let handshake = orm::model::<oauth_state::Model>(
            &mut tx,
            &oauth_state::Entity::find_by_id(request.state.trim().to_owned())
                .filter(oauth_state::Column::UserId.eq(context.account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::Conflict("oauth_state_invalid"))?;
        if handshake.consumed_at.is_some()
            || handshake.expires_at < now
            || handshake.redirect_uri != request.redirect_uri.trim()
        {
            return Err(ApiError::Conflict("oauth_state_invalid"));
        }
        let vendor = PortalVendor::parse(&handshake.vendor)?;
        if !self.switches().is_enabled(vendor) {
            return Err(ApiError::Conflict("portal_vendor_disabled"));
        }
        let organization = self
            .directory
            .get(&handshake.organization_id)
            .ok_or(ApiError::NotFound)?;

        // Consume single-use state before exchanging the authorization code.
        let consumed = orm::execute(
            &mut tx,
            &oauth_state::Entity::update_many()
                .col_expr(oauth_state::Column::ConsumedAt, Expr::val(Some(now)))
                .filter(oauth_state::Column::State.eq(handshake.state.clone()))
                .filter(oauth_state::Column::UserId.eq(context.account_id))
                .filter(oauth_state::Column::ConsumedAt.is_null())
                .into_query(),
        )
        .await?;
        if consumed != 1 {
            return Err(ApiError::Conflict("oauth_state_invalid"));
        }
        tx.commit().await?;

        let tokens = self
            .client
            .exchange_code(
                &organization,
                request.code.trim(),
                &handshake.code_verifier,
                &handshake.redirect_uri,
            )
            .await
            .map_err(map_oauth_exchange_error)?;
        let token_expires_at = tokens
            .expires_in_seconds
            .map(|seconds| now + Duration::seconds(seconds.max(0)));

        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let existing = orm::model::<connection::Model>(
            &mut tx,
            &connection::Entity::find()
                .filter(connection::Column::UserId.eq(context.account_id))
                .filter(connection::Column::OrganizationId.eq(organization.id.clone()))
                .into_query(),
        )
        .await?;

        let connection_id = if let Some(prior) = existing {
            // Reconnecting the same organization preserves its connection id
            // and imported history (`intent/duet.md` D2.11).
            orm::execute(
                &mut tx,
                &connection::Entity::update_many()
                    .col_expr(connection::Column::State, Expr::val("connected"))
                    .col_expr(
                        connection::Column::GrantedScopes,
                        Expr::val(tokens.granted_scopes.clone()),
                    )
                    .col_expr(
                        connection::Column::AccessToken,
                        Expr::val(Some(tokens.access_token.clone())),
                    )
                    .col_expr(
                        connection::Column::RefreshToken,
                        Expr::val(tokens.refresh_token.clone()),
                    )
                    .col_expr(
                        connection::Column::TokenExpiresAt,
                        Expr::val(token_expires_at),
                    )
                    .col_expr(
                        connection::Column::FailingSince,
                        Expr::val(None::<DateTime<Utc>>),
                    )
                    .col_expr(connection::Column::FailureReason, Expr::val(None::<String>))
                    .col_expr(connection::Column::RequiresReconnect, Expr::val(false))
                    .col_expr(connection::Column::PartialFailedCount, Expr::val(0))
                    .col_expr(
                        connection::Column::DisconnectedAt,
                        Expr::val(None::<DateTime<Utc>>),
                    )
                    .col_expr(connection::Column::NextRefreshAt, Expr::val(Some(now)))
                    .col_expr(connection::Column::UpdatedAt, Expr::val(now))
                    .filter(connection::Column::Id.eq(prior.id))
                    .filter(connection::Column::UserId.eq(context.account_id))
                    .into_query(),
            )
            .await?;
            prior.id
        } else {
            let id = Uuid::new_v4();
            orm::execute(
                &mut tx,
                &connection::Entity::insert(connection::ActiveModel {
                    id: Set(id),
                    user_id: Set(context.account_id),
                    organization_id: Set(organization.id.clone()),
                    organization_name: Set(organization.name.clone()),
                    vendor: Set(organization.vendor.as_str().to_owned()),
                    brand: Set(organization.brand.clone()),
                    fhir_base_url: Set(organization.fhir_base_url.clone()),
                    state: Set("connected".to_owned()),
                    granted_scopes: Set(tokens.granted_scopes.clone()),
                    access_token: Set(Some(tokens.access_token.clone())),
                    refresh_token: Set(tokens.refresh_token.clone()),
                    token_expires_at: Set(token_expires_at),
                    last_synced_at: Set(None),
                    failing_since: Set(None),
                    failure_reason: Set(None),
                    requires_reconnect: Set(false),
                    partial_failed_count: Set(0),
                    next_refresh_at: Set(Some(now)),
                    last_attempt_at: Set(None),
                    disconnected_at: Set(None),
                    evidence_revision: Set(0),
                    created_at: Set(now),
                    updated_at: Set(now),
                })
                .into_query(),
            )
            .await?;
            id
        };
        mark_setup_completed(&mut tx, context.account_id, now).await?;
        tx.commit().await?;

        self.refresh_connection(context, connection_id).await
    }

    /// Disconnect one portal connection without erasing any imported history.
    /// Always works even while newer Terms wait (`D2.11`, `D5.6`) or when the
    /// vendor is switched off (`D2.12`).
    pub async fn disconnect_connection(
        &self,
        context: &AccountContext,
        connection_id: Uuid,
    ) -> Result<PortalConnectionView, ApiError> {
        let now = (self.now)();
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let updated = orm::execute(
            &mut tx,
            &connection::Entity::update_many()
                .col_expr(connection::Column::State, Expr::val("disconnected"))
                .col_expr(connection::Column::AccessToken, Expr::val(None::<String>))
                .col_expr(connection::Column::RefreshToken, Expr::val(None::<String>))
                .col_expr(
                    connection::Column::TokenExpiresAt,
                    Expr::val(None::<DateTime<Utc>>),
                )
                .col_expr(
                    connection::Column::NextRefreshAt,
                    Expr::val(None::<DateTime<Utc>>),
                )
                .col_expr(connection::Column::DisconnectedAt, Expr::val(Some(now)))
                .col_expr(connection::Column::RequiresReconnect, Expr::val(false))
                .col_expr(connection::Column::UpdatedAt, Expr::val(now))
                .filter(connection::Column::Id.eq(connection_id))
                .filter(connection::Column::UserId.eq(context.account_id))
                .into_query(),
        )
        .await?;
        if updated != 1 {
            return Err(ApiError::NotFound);
        }
        let row = orm::model::<connection::Model>(
            &mut tx,
            &connection::Entity::find_by_id(connection_id)
                .filter(connection::Column::UserId.eq(context.account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::NotFound)?;
        tx.commit().await?;
        PortalConnectionView::from_model(row)
    }

    /// Refresh one non-disconnected portal connection (works even when the
    /// vendor is switched off for new sign-ins, `D2.12`).
    pub async fn refresh_connection(
        &self,
        context: &AccountContext,
        connection_id: Uuid,
    ) -> Result<PortalSyncResponse, ApiError> {
        let now = (self.now)();
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let row = orm::model::<connection::Model>(
            &mut tx,
            &connection::Entity::find_by_id(connection_id)
                .filter(connection::Column::UserId.eq(context.account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::NotFound)?;
        tx.commit().await?;

        if row.state == "disconnected" {
            return Err(ApiError::Conflict("portal_connection_disconnected"));
        }
        let vendor = PortalVendor::parse(&row.vendor)?;
        let organization =
            self.directory
                .get(&row.organization_id)
                .unwrap_or_else(|| PortalOrganization {
                    id: row.organization_id.clone(),
                    name: row.organization_name.clone(),
                    vendor,
                    brand: row.brand.clone(),
                    location: None,
                    fhir_base_url: row.fhir_base_url.clone(),
                    authorize_url: String::new(),
                    token_url: String::new(),
                });

        let mut active_access_token = row.access_token.clone();
        let mut active_refresh_token = row.refresh_token.clone();
        let mut active_expires_at = row.token_expires_at;
        let mut active_scopes = row.granted_scopes.clone();

        let token_expired = active_expires_at.is_some_and(|exp| exp <= now + Duration::seconds(30));
        if active_access_token.is_none() || token_expired {
            match active_refresh_token.as_deref() {
                Some(refresh) => match self.client.refresh_token(&organization, refresh).await {
                    Ok(tokens) => {
                        active_access_token = Some(tokens.access_token);
                        if tokens.refresh_token.is_some() {
                            active_refresh_token = tokens.refresh_token;
                        }
                        active_expires_at = tokens
                            .expires_in_seconds
                            .map(|s| now + Duration::seconds(s.max(0)));
                        if !tokens.granted_scopes.is_empty() {
                            active_scopes = tokens.granted_scopes;
                        }
                    }
                    Err(error) => {
                        return self.record_refresh_failure(context, &row, now, error).await;
                    }
                },
                None => {
                    return self
                        .record_refresh_failure(
                            context,
                            &row,
                            now,
                            PortalFetchError::ReauthorizationRequired {
                                reason: "portal authorization expired".to_owned(),
                            },
                        )
                        .await;
                }
            }
        }

        let Some(access_token) = active_access_token.as_deref() else {
            return self
                .record_refresh_failure(
                    context,
                    &row,
                    now,
                    PortalFetchError::ReauthorizationRequired {
                        reason: "missing portal access token".to_owned(),
                    },
                )
                .await;
        };

        let authority =
            IngestAuthority::connector(vendor.source_kind(), connection_id.to_string())?;
        let mut accepted = 0usize;
        let mut failed_items: Vec<ItemImportFailure> = Vec::new();
        let mut cursor: Option<String> = None;
        let mut pages = 0usize;
        const MAX_PAGES_PER_REFRESH: usize = 50;

        loop {
            pages += 1;
            let page = match self
                .client
                .fetch_records(
                    &organization,
                    connection_id,
                    access_token,
                    cursor.as_deref(),
                )
                .await
            {
                Ok(page) => page,
                Err(error) => {
                    if accepted == 0 && failed_items.is_empty() {
                        return self.record_refresh_failure(context, &row, now, error).await;
                    }
                    failed_items.push(ItemImportFailure {
                        record_id: cursor.unwrap_or_else(|| format!("page-{pages}")),
                        reason: error.to_string(),
                    });
                    break;
                }
            };

            let mut readable_mutations: Vec<EvidenceMutation> = Vec::new();
            for item in page.items {
                match item {
                    PortalFetchedItem::Readable(mutation) => readable_mutations.push(mutation),
                    PortalFetchedItem::Unreadable { record_id, reason } => {
                        failed_items.push(ItemImportFailure { record_id, reason });
                    }
                }
            }

            for chunk in readable_mutations.chunks(MAX_EVIDENCE_BATCH_ITEMS) {
                self.ensure_connection_active(context, connection_id)
                    .await?;
                let batch = EvidenceBatch {
                    items: chunk.to_vec(),
                };
                let result = self
                    .evidence
                    .ingest_batch_with_authority(context, &authority, &batch)
                    .await?;
                for outcome in result.results {
                    match outcome {
                        EvidenceItemOutcome::Accepted { .. } => accepted += 1,
                        EvidenceItemOutcome::Rejected {
                            index, rejection, ..
                        } => {
                            let record_id = chunk
                                .get(index)
                                .map(|m| m.record_id.clone())
                                .unwrap_or_else(|| format!("item-{index}"));
                            failed_items.push(ItemImportFailure {
                                record_id,
                                reason: rejection.message,
                            });
                        }
                    }
                }
            }

            if !page.has_more || page.next_cursor.is_none() {
                break;
            }
            if pages >= MAX_PAGES_PER_REFRESH {
                failed_items.push(ItemImportFailure {
                    record_id: "pagination".to_owned(),
                    reason: "import page limit reached; remaining pages retry on next refresh"
                        .to_owned(),
                });
                break;
            }
            cursor = page.next_cursor;
        }

        let next_refresh_at = now + DAILY_REFRESH_INTERVAL;
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let user = orm::model::<user_connectors::Model>(
            &mut tx,
            &user_connectors::Entity::find_by_id(context.account_id).into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;

        let (state_str, failure_reason, partial_failed_count, import_outcome) =
            if failed_items.is_empty() {
                ("connected", None, 0, ImportOutcome::Complete { accepted })
            } else {
                let count = i32::try_from(failed_items.len()).unwrap_or(i32::MAX);
                let summary = format!(
                    "{count} record{} could not be read and will retry on the next daily refresh.",
                    if count == 1 { "" } else { "s" }
                );
                (
                    "partial",
                    Some(summary),
                    count,
                    ImportOutcome::Partial {
                        accepted,
                        failed: failed_items,
                    },
                )
            };

        let affected = orm::execute(
            &mut tx,
            &connection::Entity::update_many()
                .col_expr(connection::Column::State, Expr::val(state_str))
                .col_expr(connection::Column::GrantedScopes, Expr::val(active_scopes))
                .col_expr(
                    connection::Column::AccessToken,
                    Expr::val(active_access_token),
                )
                .col_expr(
                    connection::Column::RefreshToken,
                    Expr::val(active_refresh_token),
                )
                .col_expr(
                    connection::Column::TokenExpiresAt,
                    Expr::val(active_expires_at),
                )
                .col_expr(connection::Column::LastSyncedAt, Expr::val(Some(now)))
                .col_expr(
                    connection::Column::FailingSince,
                    Expr::val(None::<DateTime<Utc>>),
                )
                .col_expr(connection::Column::FailureReason, Expr::val(failure_reason))
                .col_expr(connection::Column::RequiresReconnect, Expr::val(false))
                .col_expr(
                    connection::Column::PartialFailedCount,
                    Expr::val(partial_failed_count),
                )
                .col_expr(
                    connection::Column::NextRefreshAt,
                    Expr::val(Some(next_refresh_at)),
                )
                .col_expr(connection::Column::LastAttemptAt, Expr::val(Some(now)))
                .col_expr(
                    connection::Column::EvidenceRevision,
                    Expr::val(user.evidence_revision),
                )
                .col_expr(connection::Column::UpdatedAt, Expr::val(now))
                .filter(connection::Column::Id.eq(connection_id))
                .filter(connection::Column::UserId.eq(context.account_id))
                .filter(connection::Column::State.ne("disconnected"))
                .into_query(),
        )
        .await?;
        if affected != 1 {
            return Err(ApiError::Conflict("portal_connection_disconnected"));
        }

        let updated = orm::model::<connection::Model>(
            &mut tx,
            &connection::Entity::find_by_id(connection_id)
                .filter(connection::Column::UserId.eq(context.account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::NotFound)?;
        tx.commit().await?;

        Ok(PortalSyncResponse {
            connection: PortalConnectionView::from_model(updated)?,
            import: import_outcome,
        })
    }

    async fn ensure_connection_active(
        &self,
        context: &AccountContext,
        connection_id: Uuid,
    ) -> Result<(), ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let current = orm::model::<connection::Model>(
            &mut tx,
            &connection::Entity::find_by_id(connection_id)
                .filter(connection::Column::UserId.eq(context.account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::NotFound)?;
        tx.commit().await?;
        if current.state == "disconnected" {
            return Err(ApiError::Conflict("portal_connection_disconnected"));
        }
        Ok(())
    }

    async fn record_refresh_failure(
        &self,
        context: &AccountContext,
        prior: &connection::Model,
        now: DateTime<Utc>,
        error: PortalFetchError,
    ) -> Result<PortalSyncResponse, ApiError> {
        let (reason, requires_reconnect) = match error {
            PortalFetchError::ReauthorizationRequired { reason } => (reason, true),
            PortalFetchError::UpstreamUnavailable { reason }
            | PortalFetchError::InvalidResponse { reason } => (reason, false),
        };
        // Preserve the original failure day across retries so `"Couldn't update since <day>"` stays truthful.
        let failing_since = prior.failing_since.unwrap_or(now);
        let next_refresh_at = now + DAILY_REFRESH_INTERVAL;

        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let affected = orm::execute(
            &mut tx,
            &connection::Entity::update_many()
                .col_expr(connection::Column::State, Expr::val("broken"))
                .col_expr(
                    connection::Column::FailingSince,
                    Expr::val(Some(failing_since)),
                )
                .col_expr(
                    connection::Column::FailureReason,
                    Expr::val(Some(reason.clone())),
                )
                .col_expr(
                    connection::Column::RequiresReconnect,
                    Expr::val(requires_reconnect),
                )
                .col_expr(
                    connection::Column::NextRefreshAt,
                    Expr::val(Some(next_refresh_at)),
                )
                .col_expr(connection::Column::LastAttemptAt, Expr::val(Some(now)))
                .col_expr(connection::Column::UpdatedAt, Expr::val(now))
                .filter(connection::Column::Id.eq(prior.id))
                .filter(connection::Column::UserId.eq(context.account_id))
                .filter(connection::Column::State.ne("disconnected"))
                .into_query(),
        )
        .await?;
        if affected != 1 {
            return Err(ApiError::Conflict("portal_connection_disconnected"));
        }
        let updated = orm::model::<connection::Model>(
            &mut tx,
            &connection::Entity::find_by_id(prior.id)
                .filter(connection::Column::UserId.eq(context.account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::NotFound)?;
        tx.commit().await?;

        Ok(PortalSyncResponse {
            connection: PortalConnectionView::from_model(updated)?,
            import: ImportOutcome::Failed {
                reason,
                requires_reconnect,
            },
        })
    }

    /// Run one bounded daily refresh sweep over all due non-disconnected connections.
    /// Each connection executes independently: one portal failing never blocks the others (`D2.7`),
    /// and existing connections keep refreshing even if their vendor is switched off (`D2.12`).
    pub async fn run_due_refreshes(&self, limit: i64) -> Result<RefreshSweepReport, ApiError> {
        let now = (self.now)();
        let due = self.database.due_portal_connections(now, limit).await?;
        let mut report = RefreshSweepReport::default();
        for entry in due {
            report.attempted += 1;
            let Some(context) = self.internal_account_context(entry.user_id, now).await? else {
                report.failed += 1;
                continue;
            };
            match self.refresh_connection(&context, entry.id).await {
                Ok(sync) => match sync.import {
                    ImportOutcome::Complete { .. } => report.complete += 1,
                    ImportOutcome::Partial { .. } => report.partial += 1,
                    ImportOutcome::Failed { .. } => report.failed += 1,
                },
                Err(_) => report.failed += 1,
            }
        }
        Ok(report)
    }

    /// Content-free engineering diagnostics listing overdue, partial, or broken connections
    /// with their reason (`intent/duet.md` Scope "How it is built today").
    pub async fn overdue_connections(
        &self,
        grace: Duration,
        limit: i64,
    ) -> Result<Vec<OverduePortalConnection>, ApiError> {
        let now = (self.now)();
        let overdue_before = now - grace;
        let rows = self
            .database
            .overdue_portal_connections(overdue_before, limit)
            .await?;
        rows.into_iter()
            .map(|row| overdue_from_queue_row(row, now))
            .collect()
    }

    /// Return the current generation for a phone source (`appleHealth` or `healthConnect`).
    pub async fn phone_source_generation(
        &self,
        context: &AccountContext,
        source: PhoneSourceKind,
    ) -> Result<i32, ApiError> {
        match source {
            PhoneSourceKind::AppleHealth => self.evidence.apple_health_generation(context).await,
            PhoneSourceKind::HealthConnect => {
                self.evidence.health_connect_generation(context).await
            }
        }
    }

    /// Mark a phone health source connected on the server (clears `disconnected_at`
    /// and marks connector setup completed while preserving existing history).
    pub async fn connect_phone_source(
        &self,
        context: &AccountContext,
        source: PhoneSourceKind,
    ) -> Result<PhoneSourceStatusView, ApiError> {
        let now = (self.now)();
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let column = match source {
            PhoneSourceKind::AppleHealth => user_connectors::Column::AppleHealthDisconnectedAt,
            PhoneSourceKind::HealthConnect => user_connectors::Column::HealthConnectDisconnectedAt,
        };
        orm::execute(
            &mut tx,
            &user_connectors::Entity::update_many()
                .col_expr(column, Expr::val(None::<DateTime<Utc>>))
                .filter(user_connectors::Column::Id.eq(context.account_id))
                .into_query(),
        )
        .await?;
        mark_setup_completed(&mut tx, context.account_id, now).await?;
        let user = orm::model::<user_connectors::Model>(
            &mut tx,
            &user_connectors::Entity::find_by_id(context.account_id).into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;
        tx.commit().await?;
        Ok(phone_source_view(source, &user))
    }

    /// Synchronize a phone health source disconnect to `duet-api` without erasing
    /// imported history or bumping withdrawal generations. Always works even while
    /// newer Terms wait (`intent/duet.md` D2.11, D5.6).
    pub async fn disconnect_phone_source(
        &self,
        context: &AccountContext,
        source: PhoneSourceKind,
    ) -> Result<PhoneSourceStatusView, ApiError> {
        let now = (self.now)();
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let column = match source {
            PhoneSourceKind::AppleHealth => user_connectors::Column::AppleHealthDisconnectedAt,
            PhoneSourceKind::HealthConnect => user_connectors::Column::HealthConnectDisconnectedAt,
        };
        orm::execute(
            &mut tx,
            &user_connectors::Entity::update_many()
                .col_expr(column, Expr::val(Some(now)))
                .filter(user_connectors::Column::Id.eq(context.account_id))
                .into_query(),
        )
        .await?;
        let user = orm::model::<user_connectors::Model>(
            &mut tx,
            &user_connectors::Entity::find_by_id(context.account_id).into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;
        tx.commit().await?;
        Ok(phone_source_view(source, &user))
    }

    async fn internal_account_context(
        &self,
        account_id: Uuid,
        now: DateTime<Utc>,
    ) -> Result<Option<AccountContext>, ApiError> {
        let mut tx = self.database.begin(account_id).await?;
        let user = orm::model::<user_connectors::Model>(
            &mut tx,
            &user_connectors::Entity::find_by_id(account_id).into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(user.map(|row| AccountContext {
            account_id: row.id,
            incarnation: row.duet_incarnation,
            permission_revision: 0,
            authenticated_at: now,
        }))
    }
}

const APPLE_HEALTH_WEARABLE_GUIDANCE: &str = "Wearables linked to Apple Health come through HealthKit. Turn on Apple Health sharing in your wearable's app to include its readings.";
const HEALTH_CONNECT_LIMIT_GUIDANCE: &str = "Health Connect refreshes when you open Duet. Android limits background Health Connect reads unless background access is granted.";

fn phone_source_view(
    source: PhoneSourceKind,
    user: &user_connectors::Model,
) -> PhoneSourceStatusView {
    match source {
        PhoneSourceKind::AppleHealth => PhoneSourceStatusView {
            source,
            generation: user.apple_health_generation,
            disconnected_at: user.apple_health_disconnected_at,
            guidance: APPLE_HEALTH_WEARABLE_GUIDANCE.to_owned(),
        },
        PhoneSourceKind::HealthConnect => PhoneSourceStatusView {
            source,
            generation: user.health_connect_generation,
            disconnected_at: user.health_connect_disconnected_at,
            guidance: HEALTH_CONNECT_LIMIT_GUIDANCE.to_owned(),
        },
    }
}

fn overdue_from_queue_row(
    row: PortalRefreshQueueRow,
    now: DateTime<Utc>,
) -> Result<OverduePortalConnection, ApiError> {
    let vendor = PortalVendor::parse(&row.vendor)?;
    let overdue_seconds = row
        .next_refresh_at
        .map(|due| (now - due).num_seconds().max(0))
        .unwrap_or(0);
    let reason = match row.state.as_str() {
        "broken" => row
            .failure_reason
            .unwrap_or_else(|| "portal connection is broken".to_owned()),
        "partial" => row.failure_reason.unwrap_or_else(|| {
            format!(
                "partial import ({} failed items) awaiting retry",
                row.partial_failed_count
            )
        }),
        _ => format!("scheduled daily refresh overdue by {overdue_seconds}s"),
    };
    Ok(OverduePortalConnection {
        connection_id: row.id,
        vendor,
        organization_id: row.organization_id,
        state: row.state,
        last_synced_at: row.last_synced_at,
        next_refresh_at: row.next_refresh_at,
        failing_since: row.failing_since,
        overdue_seconds,
        reason,
    })
}

async fn mark_setup_completed(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account_id: Uuid,
    now: DateTime<Utc>,
) -> Result<(), ApiError> {
    orm::execute(
        tx,
        &user_connectors::Entity::update_many()
            .col_expr(user_connectors::Column::SetupComplete, Expr::val(true))
            .col_expr(
                user_connectors::Column::SetupConnectorsStatus,
                Expr::val(ConnectorSetupState::Completed.as_db()),
            )
            .col_expr(
                user_connectors::Column::SetupConnectorsUpdatedAt,
                Expr::val(Some(now)),
            )
            .filter(user_connectors::Column::Id.eq(account_id))
            .into_query(),
    )
    .await?;
    Ok(())
}

fn map_oauth_exchange_error(error: PortalFetchError) -> ApiError {
    match error {
        PortalFetchError::ReauthorizationRequired { .. } => {
            ApiError::Conflict("portal_authorization_refused")
        }
        PortalFetchError::UpstreamUnavailable { .. } => {
            ApiError::Unavailable("portal OAuth endpoint unavailable")
        }
        PortalFetchError::InvalidResponse { .. } => {
            ApiError::Unavailable("portal returned an unreadable OAuth response")
        }
    }
}

fn random_urlsafe_token(num_bytes: usize) -> String {
    let mut bytes = vec![0u8; num_bytes];
    rand::rng().fill_bytes(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn pkce_s256_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}

/// Axum router state for `/api/portals/*` and `/api/phone-sources/*`.
#[derive(Clone)]
pub struct PortalState {
    /// Authenticated identity and evidence boundary.
    pub durable: DurableState,
    /// Portal connector and phone-source sync service.
    pub service: PortalService,
    /// Optional operator bearer token authorizing `GET /api/portals/diagnostics/overdue`.
    pub diagnostics_token: Option<String>,
}

#[derive(Deserialize, Default)]
struct SearchQuery {
    #[serde(default)]
    q: String,
}

#[derive(Deserialize, Default)]
struct OverdueQuery {
    #[serde(default)]
    grace_minutes: Option<i64>,
    #[serde(default)]
    limit: Option<i64>,
}

/// Mount authenticated portal and phone-source connector routes.
pub fn router(state: PortalState) -> Router {
    Router::new()
        .route("/api/portals", get(list_portals))
        .route("/api/portals/organizations", get(search_organizations))
        .route("/api/portals/setup/not-now", post(setup_not_now))
        .route("/api/portals/oauth/start", post(oauth_start))
        .route("/api/portals/oauth/complete", post(oauth_complete))
        .route(
            "/api/portals/connections/{id}/refresh",
            post(refresh_connection),
        )
        .route(
            "/api/portals/connections/{id}/disconnect",
            post(disconnect_connection),
        )
        .route(
            "/api/portals/connections/{id}",
            axum::routing::delete(disconnect_connection),
        )
        .route("/api/portals/diagnostics/overdue", get(overdue_diagnostics))
        .route(
            "/api/phone-sources/{source}/generation",
            get(phone_source_generation),
        )
        .route(
            "/api/phone-sources/{source}/connect",
            post(phone_source_connect),
        )
        .route(
            "/api/phone-sources/{source}/disconnect",
            post(phone_source_disconnect),
        )
        .with_state(state)
}

async fn list_portals(
    State(state): State<PortalState>,
    headers: HeaderMap,
) -> Result<Json<PortalsSnapshot>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(state.service.snapshot(&context).await?))
}

async fn search_organizations(
    State(state): State<PortalState>,
    headers: HeaderMap,
    Query(query): Query<SearchQuery>,
) -> Result<Json<PortalOrganizationsSearchResponse>, ApiError> {
    let _ = state.durable.authenticate(&headers).await?;
    Ok(Json(state.service.search_organizations(&query.q)))
}

async fn setup_not_now(
    State(state): State<PortalState>,
    headers: HeaderMap,
) -> Result<Json<SetupStatusResponse>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(state.service.skip_setup_not_now(&context).await?))
}

async fn oauth_start(
    State(state): State<PortalState>,
    headers: HeaderMap,
    Json(body): Json<OAuthStartRequest>,
) -> Result<Json<OAuthStartResponse>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(state.service.start_oauth(&context, &body).await?))
}

async fn oauth_complete(
    State(state): State<PortalState>,
    headers: HeaderMap,
    Json(body): Json<OAuthCompleteRequest>,
) -> Result<Json<PortalSyncResponse>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(state.service.complete_oauth(&context, &body).await?))
}

async fn refresh_connection(
    State(state): State<PortalState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<PortalSyncResponse>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(state.service.refresh_connection(&context, id).await?))
}

async fn disconnect_connection(
    State(state): State<PortalState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<PortalConnectionView>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(
        state.service.disconnect_connection(&context, id).await?,
    ))
}

async fn phone_source_generation(
    State(state): State<PortalState>,
    headers: HeaderMap,
    Path(source): Path<String>,
) -> Result<Json<i32>, ApiError> {
    let source = PhoneSourceKind::parse(&source)?;
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(
        state
            .service
            .phone_source_generation(&context, source)
            .await?,
    ))
}

async fn phone_source_connect(
    State(state): State<PortalState>,
    headers: HeaderMap,
    Path(source): Path<String>,
) -> Result<Json<PhoneSourceStatusView>, ApiError> {
    let source = PhoneSourceKind::parse(&source)?;
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(
        state.service.connect_phone_source(&context, source).await?,
    ))
}

async fn phone_source_disconnect(
    State(state): State<PortalState>,
    headers: HeaderMap,
    Path(source): Path<String>,
) -> Result<Json<PhoneSourceStatusView>, ApiError> {
    let source = PhoneSourceKind::parse(&source)?;
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(
        state
            .service
            .disconnect_phone_source(&context, source)
            .await?,
    ))
}

async fn overdue_diagnostics(
    State(state): State<PortalState>,
    headers: HeaderMap,
    Query(query): Query<OverdueQuery>,
) -> Result<Json<Vec<OverduePortalConnection>>, ApiError> {
    let expected = state
        .diagnostics_token
        .as_deref()
        .filter(|token| !token.trim().is_empty())
        .ok_or(ApiError::NotFound)?;
    let presented = crate::bearer(&headers).ok_or(ApiError::Unauthorized)?;
    if !constant_time_eq(expected.as_bytes(), presented.as_bytes()) {
        return Err(ApiError::Unauthorized);
    }
    let grace = Duration::minutes(query.grace_minutes.unwrap_or(0).max(0));
    let limit = query.limit.unwrap_or(100);
    Ok(Json(state.service.overdue_connections(grace, limit).await?))
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut diff = 0u8;
    for (a, b) in left.iter().zip(right.iter()) {
        diff |= a ^ b;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendor_kill_switch_filters_search_and_preserves_enabled_vendors() {
        let dir = StaticPortalDirectory::default();
        let all = dir.search("", VendorSwitches::default());
        assert!(all.iter().any(|org| org.vendor == PortalVendor::Epic));
        assert!(all.iter().any(|org| org.vendor == PortalVendor::Oracle));

        let no_oracle = VendorSwitches {
            epic: VendorToggle::Enabled,
            oracle: VendorToggle::Disabled,
        };
        let filtered = dir.search("", no_oracle);
        assert!(!filtered.is_empty());
        assert!(filtered.iter().all(|org| org.vendor == PortalVendor::Epic));
        assert_eq!(no_oracle.enabled_vendors().len(), 1);
        assert_eq!(no_oracle.enabled_vendors()[0].vendor, PortalVendor::Epic);

        let ucsf = dir.search("ucsf", VendorSwitches::default());
        assert_eq!(ucsf.len(), 1);
        assert_eq!(ucsf[0].vendor, PortalVendor::Epic);
        assert_eq!(ucsf[0].brand, "Epic / MyChart");

        let advent = dir.search("adventhealth", VendorSwitches::default());
        assert_eq!(advent.len(), 1);
        assert_eq!(advent[0].vendor, PortalVendor::Oracle);
        assert_eq!(advent[0].brand, "Oracle Health");
    }

    #[test]
    fn update_failure_day_formats_plain_copy() {
        let at: DateTime<Utc> = "2026-11-19T14:30:00Z".parse().unwrap();
        assert_eq!(
            format_update_failure_day(at),
            "Couldn't update since Nov 19"
        );
    }

    #[test]
    fn fhir_pagination_url_enforces_exact_https_origin() {
        let base = "https://unified-api.ucsf.edu/clinical/apex/api/FHIR/R4";
        let initial = validated_fhir_page_url(base, None).expect("initial page url");
        assert_eq!(
            initial.as_str(),
            "https://unified-api.ucsf.edu/clinical/apex/api/FHIR/R4/Observation?_count=50"
        );
        let same_origin = validated_fhir_page_url(
            base,
            Some("https://unified-api.ucsf.edu/clinical/apex/api/FHIR/R4/Observation?page=2"),
        )
        .expect("same-origin HTTPS cursor allowed");
        assert_eq!(same_origin.host_str(), Some("unified-api.ucsf.edu"));

        for bad in [
            "https://attacker.example/steal-token",
            "http://unified-api.ucsf.edu/clinical/apex/api/FHIR/R4/Observation?page=2",
            "https://unified-api.ucsf.edu:8443/clinical/apex/api/FHIR/R4/Observation?page=2",
            "https://user:pass@unified-api.ucsf.edu/clinical/apex/api/FHIR/R4/Observation?page=2",
            "not a url",
        ] {
            assert!(
                matches!(
                    validated_fhir_page_url(base, Some(bad)),
                    Err(PortalFetchError::InvalidResponse { .. })
                ),
                "pagination cursor {bad:?} must be refused"
            );
        }
    }

    #[test]
    fn portal_secrets_are_redacted_in_debug_output() {
        let tokens = PortalTokenSet {
            access_token: "SECRET-ACCESS-TOKEN".into(),
            refresh_token: Some("SECRET-REFRESH-TOKEN".into()),
            expires_in_seconds: Some(3600),
            granted_scopes: vec!["patient/Observation.read".into()],
        };
        let rendered = format!("{tokens:?}");
        assert!(rendered.contains("<redacted>"));
        assert!(!rendered.contains("SECRET-ACCESS-TOKEN"));
        assert!(!rendered.contains("SECRET-REFRESH-TOKEN"));

        let now = Utc::now();
        let conn_row = connection::Model {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            organization_id: "epic-ucsf-health".into(),
            organization_name: "UCSF Health".into(),
            vendor: "epic".into(),
            brand: "Epic / MyChart".into(),
            fhir_base_url: "https://unified-api.ucsf.edu/clinical/apex/api/FHIR/R4".into(),
            state: "connected".into(),
            granted_scopes: vec![],
            access_token: Some("SECRET-CONN-ACCESS".into()),
            refresh_token: Some("SECRET-CONN-REFRESH".into()),
            token_expires_at: None,
            last_synced_at: None,
            failing_since: None,
            failure_reason: None,
            requires_reconnect: false,
            partial_failed_count: 0,
            next_refresh_at: None,
            last_attempt_at: None,
            disconnected_at: None,
            evidence_revision: 0,
            created_at: now,
            updated_at: now,
        };
        let conn_debug = format!("{conn_row:?}");
        assert!(conn_debug.contains("<redacted>"));
        assert!(!conn_debug.contains("SECRET-CONN-ACCESS"));
        assert!(!conn_debug.contains("SECRET-CONN-REFRESH"));

        let state_row = oauth_state::Model {
            state: "SECRET-OAUTH-STATE".into(),
            user_id: Uuid::new_v4(),
            organization_id: "epic-ucsf-health".into(),
            vendor: "epic".into(),
            code_verifier: "SECRET-PKCE-VERIFIER".into(),
            redirect_uri: "duet://oauth/callback".into(),
            created_at: now,
            expires_at: now + OAUTH_STATE_TTL,
            consumed_at: None,
        };
        let state_debug = format!("{state_row:?}");
        assert!(state_debug.contains("<redacted>"));
        assert!(!state_debug.contains("SECRET-OAUTH-STATE"));
        assert!(!state_debug.contains("SECRET-PKCE-VERIFIER"));
    }
}
