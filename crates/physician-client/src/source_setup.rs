//! Typed HTTP client methods for practice-facing source connection and recovery (`/connections`).

use physician_wire::source_setup::{
    ConnectSourceBody, PracticeConnectionsWire, RequestAssistedSetupBody, SimulateSourceEventBody,
    SourceOperationResponseWire, SupportedSourceWire,
};

use crate::PhysicianClient;

impl PhysicianClient {
    /// Reads all practice source connections and their saved standing (`GET /connections`).
    pub async fn connections(&self) -> Result<PracticeConnectionsWire, http::HttpError> {
        self.get("/connections").await
    }

    /// Verifies credentials, browser connector session, or simulated access (`POST /connections/{source}/connect`),
    /// recording `Connected` without implying records were imported.
    pub async fn connect_source(
        &self,
        source: SupportedSourceWire,
        body: &ConnectSourceBody,
    ) -> Result<SourceOperationResponseWire, http::HttpError> {
        self.post(&format!("/connections/{}/connect", source.slug()), body)
            .await
    }

    /// Requests Coda-assisted onboarding for `source` (`POST /connections/{source}/assist`).
    pub async fn request_assisted_setup(
        &self,
        source: SupportedSourceWire,
        body: &RequestAssistedSetupBody,
    ) -> Result<SourceOperationResponseWire, http::HttpError> {
        self.post(&format!("/connections/{}/assist", source.slug()), body)
            .await
    }

    /// Imports initial records for a `Connected` source (`POST /connections/{source}/import`),
    /// transitioning its standing to `RecordsImported`.
    pub async fn import_source_records(
        &self,
        source: SupportedSourceWire,
    ) -> Result<SourceOperationResponseWire, http::HttpError> {
        self.post(
            &format!("/connections/{}/import", source.slug()),
            &serde_json::json!({}),
        )
        .await
    }

    /// Refreshes a connected source (`POST /connections/{source}/refresh`), transitioning its
    /// standing to `UpToDate`.
    pub async fn refresh_source(
        &self,
        source: SupportedSourceWire,
    ) -> Result<SourceOperationResponseWire, http::HttpError> {
        self.post(
            &format!("/connections/{}/refresh", source.slug()),
            &serde_json::json!({}),
        )
        .await
    }

    /// Explicitly accepts reading pre-connection historical Spruce messages (`POST /connections/spruce/accept-backlog`).
    pub async fn accept_spruce_backlog(
        &self,
    ) -> Result<SourceOperationResponseWire, http::HttpError> {
        self.post("/connections/spruce/accept-backlog", &serde_json::json!({}))
            .await
    }

    /// Explicitly resolves an active Spruce release-rollback filing halt (`POST /connections/spruce/rollback/resolve`).
    pub async fn resolve_spruce_rollback(
        &self,
        adopt_running_release: bool,
    ) -> Result<SourceOperationResponseWire, http::HttpError> {
        self.post(
            "/connections/spruce/rollback/resolve",
            &serde_json::json!({ "adopt_running_release": adopt_running_release }),
        )
        .await
    }

    /// Re-authenticates a source after `AccessLost` (`POST /connections/{source}/reconnect`),
    /// restoring current access while preserving previously saved records.
    pub async fn reconnect_source(
        &self,
        source: SupportedSourceWire,
        body: &ConnectSourceBody,
    ) -> Result<SourceOperationResponseWire, http::HttpError> {
        self.post(&format!("/connections/{}/reconnect", source.slug()), body)
            .await
    }

    /// Disconnects `source` (`POST /connections/{source}/disconnect`), purging never-linked
    /// Spruce messages while retaining linked clinical records and reply drafts.
    pub async fn disconnect_source(
        &self,
        source: SupportedSourceWire,
    ) -> Result<SourceOperationResponseWire, http::HttpError> {
        self.post(
            &format!("/connections/{}/disconnect", source.slug()),
            &serde_json::json!({}),
        )
        .await
    }

    /// Triggers a simulated source event (`POST /connections/{source}/simulate`) in rehearsal/test setups.
    pub async fn simulate_source_event(
        &self,
        source: SupportedSourceWire,
        body: &SimulateSourceEventBody,
    ) -> Result<SourceOperationResponseWire, http::HttpError> {
        self.post(&format!("/connections/{}/simulate", source.slug()), body)
            .await
    }
}
