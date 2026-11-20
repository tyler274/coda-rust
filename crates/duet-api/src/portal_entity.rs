//! Duet-owned SeaORM mappings for portal connections, OAuth state handshakes,
//! and account setup/phone-source connector columns.
//!
//! Statements built from these entities run inside `AccountDatabase::begin` so
//! row-level security and account locking stay in one transaction.

/// Per-organization portal connections (`public.duet_portal_connections`).
pub mod connection {
    use sea_orm::entity::prelude::*;

    /// One `public.duet_portal_connections` row.
    #[derive(Clone, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_portal_connections")]
    pub struct Model {
        /// Stable connection UUID reused across disconnect/reconnect of the same org.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning account.
        pub user_id: Uuid,
        /// Stable directory organization id.
        pub organization_id: String,
        /// Organization display name.
        pub organization_name: String,
        /// Vendor identifier (`epic` or `oracle`).
        pub vendor: String,
        /// Patient-facing vendor brand label.
        pub brand: String,
        /// Organization FHIR R4 base URL.
        pub fhir_base_url: String,
        /// Lifecycle state (`connected`, `partial`, `broken`, `disconnected`).
        pub state: String,
        /// Granted SMART scopes.
        pub granted_scopes: Vec<String>,
        /// Current OAuth access token (cleared on disconnect).
        pub access_token: Option<String>,
        /// Current OAuth refresh token (cleared on disconnect).
        pub refresh_token: Option<String>,
        /// Access token expiry.
        pub token_expires_at: Option<DateTimeUtc>,
        /// Latest successful or partial sync timestamp.
        pub last_synced_at: Option<DateTimeUtc>,
        /// First failure timestamp for the active broken stretch.
        pub failing_since: Option<DateTimeUtc>,
        /// Plain reason for partial or broken state.
        pub failure_reason: Option<String>,
        /// Whether patient re-authorization is required.
        pub requires_reconnect: bool,
        /// Count of unreadable/rejected items on the latest partial import.
        pub partial_failed_count: i32,
        /// Scheduled next daily refresh timestamp (`None` when disconnected).
        pub next_refresh_at: Option<DateTimeUtc>,
        /// Latest refresh attempt timestamp.
        pub last_attempt_at: Option<DateTimeUtc>,
        /// Explicit disconnect timestamp (`Some` while disconnected).
        pub disconnected_at: Option<DateTimeUtc>,
        /// Latest account evidence revision reached by this connection.
        pub evidence_revision: i64,
        /// Creation timestamp.
        pub created_at: DateTimeUtc,
        /// Last mutation timestamp.
        pub updated_at: DateTimeUtc,
    }

    impl std::fmt::Debug for Model {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PortalConnectionModel")
                .field("id", &self.id)
                .field("user_id", &self.user_id)
                .field("organization_id", &self.organization_id)
                .field("vendor", &self.vendor)
                .field("state", &self.state)
                .field(
                    "access_token",
                    &self.access_token.as_ref().map(|_| "<redacted>"),
                )
                .field(
                    "refresh_token",
                    &self.refresh_token.as_ref().map(|_| "<redacted>"),
                )
                .field("last_synced_at", &self.last_synced_at)
                .field("failing_since", &self.failing_since)
                .field("next_refresh_at", &self.next_refresh_at)
                .field("disconnected_at", &self.disconnected_at)
                .finish()
        }
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Single-use PKCE OAuth handshake state (`public.duet_portal_oauth_states`).
pub mod oauth_state {
    use sea_orm::entity::prelude::*;

    /// One `public.duet_portal_oauth_states` row.
    #[derive(Clone, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_portal_oauth_states")]
    pub struct Model {
        /// Cryptographically random OAuth `state` parameter.
        #[sea_orm(primary_key, auto_increment = false)]
        pub state: String,
        /// Owning account.
        pub user_id: Uuid,
        /// Target organization id.
        pub organization_id: String,
        /// Target vendor (`epic` or `oracle`).
        pub vendor: String,
        /// PKCE S256 verifier.
        pub code_verifier: String,
        /// Redirect URI registered for this authorization request.
        pub redirect_uri: String,
        /// Creation timestamp.
        pub created_at: DateTimeUtc,
        /// Expiration timestamp.
        pub expires_at: DateTimeUtc,
        /// Consumption timestamp (`None` until redeemed once).
        pub consumed_at: Option<DateTimeUtc>,
    }

    impl std::fmt::Debug for Model {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PortalOAuthStateModel")
                .field("state", &"<redacted>")
                .field("user_id", &self.user_id)
                .field("organization_id", &self.organization_id)
                .field("vendor", &self.vendor)
                .field("code_verifier", &"<redacted>")
                .field("redirect_uri", &self.redirect_uri)
                .field("expires_at", &self.expires_at)
                .field("consumed_at", &self.consumed_at)
                .finish()
        }
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Account connector setup and phone-source columns on `public.users`.
pub mod user_connectors {
    use sea_orm::entity::prelude::*;

    /// Connector-related columns of `public.users`.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "users")]
    pub struct Model {
        /// Account identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Current account incarnation for internal refresh context.
        pub duet_incarnation: Uuid,
        /// Current Apple Health generation.
        pub apple_health_generation: i32,
        /// Explicit Apple Health disconnect timestamp (history retained).
        pub apple_health_disconnected_at: Option<DateTimeUtc>,
        /// Current Android Health Connect generation.
        pub health_connect_generation: i32,
        /// Explicit Android Health Connect disconnect timestamp (history retained).
        pub health_connect_disconnected_at: Option<DateTimeUtc>,
        /// Whether post-signup setup completed or was skipped via "Not now".
        pub setup_complete: bool,
        /// Connector setup state (`pending`, `skipped_not_now`, `completed`).
        pub setup_connectors_status: String,
        /// Last time connector setup state changed.
        pub setup_connectors_updated_at: Option<DateTimeUtc>,
        /// Latest account evidence revision.
        pub evidence_revision: i64,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
