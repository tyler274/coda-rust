//! Multi-device push token registration and strictly generic lock-screen delivery.
//!
//! Lock-screen payloads derive exclusively from [`duet_notify::GenericAlertKind`] so
//! health content, clinician names, lab values, and clinical prose can never leave Duet
//! on a lock screen (`intent/duet.md` What leaves Duet).
//!
//! Device registrations are bound to the signed-in session's `authenticated_at` timestamp,
//! follow every active device on the account, and clear tokens and pending alerts on
//! single-device sign-out, sign-out everywhere (`SessionsRevoke`), and account deletion.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use duet_notify::{GenericAlertKind, GenericLockScreenAlert, PushChannel, PushDeviceToken};
use sea_orm::sea_query::{Expr, OnConflict, Query};
use sea_orm::{
    ColumnTrait, EntityName, EntityTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ApiError;
use crate::database::{AccountDatabase, orm};
use crate::durable::DurableState;
use crate::identity::{AccountContext, check_current};
use crate::push_entity::{account_fence, delivery, device};

/// Maximum accepted length of a device installation identifier.
const MAX_DEVICE_ID_BYTES: usize = 128;

/// Closed wire push channel matching [`PushChannel`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PushPlatform {
    /// Apple Push Notification service.
    Apns,
    /// Firebase Cloud Messaging.
    Fcm,
}

impl PushPlatform {
    /// Converts to the shared [`PushChannel`].
    pub const fn into_channel(self) -> PushChannel {
        match self {
            Self::Apns => PushChannel::Apns,
            Self::Fcm => PushChannel::Fcm,
        }
    }

    /// Converts from the shared [`PushChannel`].
    pub const fn from_channel(channel: PushChannel) -> Self {
        match channel {
            PushChannel::Apns => Self::Apns,
            PushChannel::Fcm => Self::Fcm,
        }
    }

    /// Stable wire identifier stored in PostgreSQL.
    pub const fn as_wire(self) -> &'static str {
        self.into_channel().as_wire()
    }

    /// Parses a stored wire identifier.
    pub fn from_wire(raw: &str) -> Option<Self> {
        PushChannel::from_wire(raw).map(Self::from_channel)
    }
}

/// Strictly generic lock-screen payload transmitted to an upstream push provider.
///
/// Constructed exclusively from [`GenericAlertKind`] so no free text or PHI can enter
/// the outbound notification body.
#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct GenericPushDispatch {
    /// Stable delivery identifier for provider deduplication.
    pub delivery_id: Uuid,
    /// Target push platform.
    pub platform: PushPlatform,
    /// Target device push token (redacted in `Debug`).
    #[serde(skip_serializing)]
    pub token: PushDeviceToken,
    /// Fixed generic category identifier.
    pub category: &'static str,
    /// Fixed lock-screen title (`"Duet"`).
    pub title: &'static str,
    /// Fixed generic lock-screen body carrying zero health content.
    pub body: &'static str,
}

impl GenericPushDispatch {
    /// Builds an outbound dispatch carrying strictly generic lock-screen copy.
    pub fn new(
        delivery_id: Uuid,
        platform: PushPlatform,
        token: PushDeviceToken,
        kind: GenericAlertKind,
    ) -> Self {
        let alert: GenericLockScreenAlert = kind.lock_screen_alert();
        Self {
            delivery_id,
            platform,
            token,
            category: alert.category,
            title: alert.title,
            body: alert.body,
        }
    }
}

impl fmt::Debug for GenericPushDispatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GenericPushDispatch")
            .field("delivery_id", &self.delivery_id)
            .field("platform", &self.platform)
            .field("token", &"<redacted>")
            .field("category", &self.category)
            .field("title", &self.title)
            .field("body", &self.body)
            .finish()
    }
}

/// Outbound remote push transport error.
#[derive(Debug)]
pub enum PushTransportError {
    /// The remote gateway rejected the registration token as permanently invalid.
    InvalidToken,
    /// Transient transport or gateway outage; delivery stays pending for retry or cleanup.
    Unavailable(String),
}

impl fmt::Display for PushTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken => f.write_str("remote push token rejected as invalid"),
            Self::Unavailable(detail) => write!(f, "remote push transport unavailable: {detail}"),
        }
    }
}

impl std::error::Error for PushTransportError {}

/// Injectable push gateway transport (APNs/FCM relay or test double).
#[async_trait]
pub trait PushTransport: Send + Sync {
    /// Dispatches one strictly generic lock-screen alert to one registered device.
    async fn send(&self, dispatch: &GenericPushDispatch) -> Result<(), PushTransportError>;
}

/// Fail-closed transport used when no external push relay URL is configured.
#[derive(Clone, Copy, Debug, Default)]
pub struct UnconfiguredPushTransport;

#[async_trait]
impl PushTransport for UnconfiguredPushTransport {
    async fn send(&self, _dispatch: &GenericPushDispatch) -> Result<(), PushTransportError> {
        Err(PushTransportError::Unavailable(
            "push relay is not configured".into(),
        ))
    }
}

/// Bounded, no-redirect HTTPS push relay transport.
#[derive(Clone)]
pub struct HttpPushTransport {
    client: reqwest::Client,
    endpoint: reqwest::Url,
    bearer_secret: Option<String>,
}

impl fmt::Debug for HttpPushTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpPushTransport")
            .field("endpoint", &self.endpoint.as_str())
            .field(
                "bearer_secret",
                &self.bearer_secret.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

impl HttpPushTransport {
    /// Builds a bounded push relay transport. Rejects non-HTTPS endpoints except loopback HTTP.
    pub fn new(endpoint: &str, bearer_secret: Option<String>) -> Result<Self, ApiError> {
        let parsed = reqwest::Url::parse(endpoint.trim())
            .map_err(|_| ApiError::Unprocessable("invalid push relay endpoint URL"))?;
        let loopback = matches!(parsed.host_str(), Some("127.0.0.1" | "localhost" | "::1"));
        if parsed.scheme() != "https" && !(parsed.scheme() == "http" && loopback) {
            return Err(ApiError::Unprocessable(
                "push relay endpoint must use HTTPS",
            ));
        }
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|_| ApiError::Unavailable("push HTTP client unavailable"))?;
        Ok(Self {
            client,
            endpoint: parsed,
            bearer_secret,
        })
    }
}

#[async_trait]
impl PushTransport for HttpPushTransport {
    async fn send(&self, dispatch: &GenericPushDispatch) -> Result<(), PushTransportError> {
        let mut request = self
            .client
            .post(self.endpoint.clone())
            .json(&serde_json::json!({
                "delivery_id": dispatch.delivery_id,
                "platform": dispatch.platform.as_wire(),
                "token": dispatch.token.as_str(),
                "category": dispatch.category,
                "title": dispatch.title,
                "body": dispatch.body,
            }));
        if let Some(secret) = &self.bearer_secret {
            request = request.bearer_auth(secret);
        }
        let response = request
            .send()
            .await
            .map_err(|error| PushTransportError::Unavailable(error.without_url().to_string()))?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else if status == reqwest::StatusCode::GONE || status == reqwest::StatusCode::NOT_FOUND {
            Err(PushTransportError::InvalidToken)
        } else {
            Err(PushTransportError::Unavailable(format!(
                "gateway returned HTTP {}",
                status.as_u16()
            )))
        }
    }
}

/// Request body for `PUT /api/push/devices`.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegisterDeviceRequest {
    /// Stable per-installation device identifier.
    pub device_id: String,
    /// Native push channel (`apns` or `fcm`).
    pub platform: PushPlatform,
    /// Remote device push token.
    pub push_token: String,
}

impl fmt::Debug for RegisterDeviceRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegisterDeviceRequest")
            .field("device_id", &self.device_id)
            .field("platform", &self.platform)
            .field("push_token", &"<redacted>")
            .finish()
    }
}

/// Account-visible metadata for one registered push device. Never returns the raw push token.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredDeviceSummary {
    /// Stable per-installation device identifier.
    pub device_id: String,
    /// Native push channel.
    pub platform: PushPlatform,
    /// When this device first registered.
    pub registered_at: DateTime<Utc>,
    /// When this device registration was last refreshed.
    pub updated_at: DateTime<Utc>,
}

/// Outcome of clearing device registrations and pending alerts on sign-out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceCleanupOutcome {
    /// Number of device push tokens cleared.
    pub cleared_devices: u64,
    /// Number of pending alerts cancelled and removed.
    pub cancelled_pending_alerts: u64,
}

/// Result of fanning out one generic lock-screen alert across all active devices.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericDeliverySummary {
    /// Closed generic category that was dispatched.
    pub category: String,
    /// Active device IDs that received the generic lock-screen alert.
    pub delivered_devices: Vec<String>,
    /// Active device IDs whose delivery remains pending after transient transport failure.
    pub pending_devices: Vec<String>,
    /// Device IDs revoked after the gateway reported an invalid token or a concurrent sign-out.
    pub revoked_devices: Vec<String>,
}

/// Durable owner for per-account multi-device push registrations and generic deliveries.
#[derive(Clone)]
pub struct PushStore {
    database: AccountDatabase,
    transport: Arc<dyn PushTransport>,
}

impl PushStore {
    /// Creates a push store backed by `database` and an outbound `transport`.
    pub fn new(database: AccountDatabase, transport: Arc<dyn PushTransport>) -> Self {
        Self {
            database,
            transport,
        }
    }

    /// Registers or refreshes one signed-in device's push token.
    pub async fn register_device(
        &self,
        context: &AccountContext,
        request: RegisterDeviceRequest,
    ) -> Result<RegisteredDeviceSummary, ApiError> {
        let device_id = validate_device_id(&request.device_id)?;
        let token = PushDeviceToken::parse(&request.push_token)
            .map_err(|_| ApiError::Unprocessable("invalid push device token"))?;
        let now = Utc::now();

        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;

        let insert = Query::insert()
            .into_table(device::Entity.table_ref())
            .columns([
                device::Column::UserId,
                device::Column::DeviceId,
                device::Column::SessionAuthenticatedAt,
                device::Column::Platform,
                device::Column::PushToken,
                device::Column::RegisteredAt,
                device::Column::UpdatedAt,
                device::Column::RevokedAt,
            ])
            .values_panic([
                context.account_id.into(),
                device_id.as_str().into(),
                context.authenticated_at.into(),
                request.platform.as_wire().into(),
                token.as_str().into(),
                now.into(),
                now.into(),
                Option::<DateTime<Utc>>::None.into(),
            ])
            .on_conflict(
                OnConflict::columns([device::Column::UserId, device::Column::DeviceId])
                    .update_columns([
                        device::Column::SessionAuthenticatedAt,
                        device::Column::Platform,
                        device::Column::PushToken,
                        device::Column::UpdatedAt,
                        device::Column::RevokedAt,
                    ])
                    .to_owned(),
            )
            .to_owned();
        orm::execute(&mut tx, &insert).await?;

        let stored = orm::model::<device::Model>(
            &mut tx,
            &device::Entity::find()
                .filter(device::Column::UserId.eq(context.account_id))
                .filter(device::Column::DeviceId.eq(&device_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::Internal("push device missing after upsert"))?;
        tx.commit().await?;

        to_device_summary(stored)
    }

    /// Lists all active push-registered devices for the signed-in account.
    ///
    /// Devices bound to a session revoked via `SessionsRevoke` are pruned before returning.
    pub async fn list_active_devices(
        &self,
        context: &AccountContext,
    ) -> Result<Vec<RegisteredDeviceSummary>, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        prune_revoked_sessions_in(&mut tx, context.account_id).await?;

        let rows = orm::models::<device::Model>(
            &mut tx,
            &device::Entity::find()
                .filter(device::Column::UserId.eq(context.account_id))
                .filter(device::Column::RevokedAt.is_null())
                .order_by_asc(device::Column::RegisteredAt)
                .order_by_asc(device::Column::DeviceId)
                .into_query(),
        )
        .await?;
        tx.commit().await?;

        rows.into_iter().map(to_device_summary).collect()
    }

    /// Unregisters one device and removes all pending alerts queued for that device.
    pub async fn unregister_device(
        &self,
        context: &AccountContext,
        raw_device_id: &str,
    ) -> Result<DeviceCleanupOutcome, ApiError> {
        let device_id = validate_device_id(raw_device_id)?;
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;

        let cancelled_pending_alerts = orm::execute(
            &mut tx,
            &delivery::Entity::delete_many()
                .filter(delivery::Column::UserId.eq(context.account_id))
                .filter(delivery::Column::DeviceId.eq(&device_id))
                .filter(delivery::Column::Status.eq("pending"))
                .into_query(),
        )
        .await?;
        let cleared_devices = orm::execute(
            &mut tx,
            &device::Entity::delete_many()
                .filter(device::Column::UserId.eq(context.account_id))
                .filter(device::Column::DeviceId.eq(&device_id))
                .into_query(),
        )
        .await?;
        tx.commit().await?;

        Ok(DeviceCleanupOutcome {
            cleared_devices,
            cancelled_pending_alerts,
        })
    }

    /// Clears every registered push token and pending alert for the signed-in account.
    pub async fn sign_out_all_devices(
        &self,
        context: &AccountContext,
    ) -> Result<DeviceCleanupOutcome, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;

        let cancelled_pending_alerts = orm::execute(
            &mut tx,
            &delivery::Entity::delete_many()
                .filter(delivery::Column::UserId.eq(context.account_id))
                .filter(delivery::Column::Status.eq("pending"))
                .into_query(),
        )
        .await?;
        let cleared_devices = orm::execute(
            &mut tx,
            &device::Entity::delete_many()
                .filter(device::Column::UserId.eq(context.account_id))
                .into_query(),
        )
        .await?;
        tx.commit().await?;

        Ok(DeviceCleanupOutcome {
            cleared_devices,
            cancelled_pending_alerts,
        })
    }

    /// Queues and delivers one strictly generic lock-screen alert to every active device on
    /// `account_id`.
    ///
    /// Re-verifies session revocation fences before queuing and after transport so a
    /// concurrent sign-out never resurrects or finalizes delivery for a signed-out phone.
    pub async fn deliver_generic_alert(
        &self,
        account_id: Uuid,
        kind: GenericAlertKind,
    ) -> Result<GenericDeliverySummary, ApiError> {
        let mut pending_batch = Vec::new();
        {
            let mut tx = self.database.begin(account_id).await?;
            let account = orm::model::<account_fence::Model>(
                &mut tx,
                &account_fence::Entity::find_by_id(account_id)
                    .lock_exclusive()
                    .into_query(),
            )
            .await?;
            if account.is_none() {
                return Err(ApiError::AbsentAccount);
            }
            prune_revoked_sessions_in(&mut tx, account_id).await?;

            let active = orm::models::<device::Model>(
                &mut tx,
                &device::Entity::find()
                    .filter(device::Column::UserId.eq(account_id))
                    .filter(device::Column::RevokedAt.is_null())
                    .order_by_asc(device::Column::RegisteredAt)
                    .order_by_asc(device::Column::DeviceId)
                    .into_query(),
            )
            .await?;

            let now = Utc::now();
            for row in active {
                let platform = PushPlatform::from_wire(&row.platform)
                    .ok_or(ApiError::Internal("stored push platform is invalid"))?;
                let token = PushDeviceToken::parse(&row.push_token)
                    .map_err(|_| ApiError::Internal("stored push token is invalid"))?;
                let delivery_id = Uuid::new_v4();
                let insert = Query::insert()
                    .into_table(delivery::Entity.table_ref())
                    .columns([
                        delivery::Column::UserId,
                        delivery::Column::Id,
                        delivery::Column::DeviceId,
                        delivery::Column::AlertKind,
                        delivery::Column::Status,
                        delivery::Column::CreatedAt,
                    ])
                    .values_panic([
                        account_id.into(),
                        delivery_id.into(),
                        row.device_id.as_str().into(),
                        kind.as_wire().into(),
                        "pending".into(),
                        now.into(),
                    ])
                    .to_owned();
                orm::execute(&mut tx, &insert).await?;
                pending_batch.push((
                    row.device_id,
                    GenericPushDispatch::new(delivery_id, platform, token, kind),
                ));
            }
            tx.commit().await?;
        }

        let mut delivered_devices = Vec::new();
        let mut pending_devices = Vec::new();
        let mut revoked_devices = Vec::new();

        for (device_id, dispatch) in pending_batch {
            // Re-verify the device is still active and not revoked before hitting the network.
            if !self
                .device_and_delivery_still_active(account_id, &device_id, dispatch.delivery_id)
                .await?
            {
                revoked_devices.push(device_id);
                continue;
            }

            match self.transport.send(&dispatch).await {
                Ok(()) => {
                    let mut tx = self.database.begin(account_id).await?;
                    let updated = orm::execute(
                        &mut tx,
                        &delivery::Entity::update_many()
                            .col_expr(delivery::Column::Status, Expr::val("sent"))
                            .col_expr(delivery::Column::DispatchedAt, Expr::val(Some(Utc::now())))
                            .filter(delivery::Column::UserId.eq(account_id))
                            .filter(delivery::Column::Id.eq(dispatch.delivery_id))
                            .filter(delivery::Column::Status.eq("pending"))
                            .into_query(),
                    )
                    .await?;
                    tx.commit().await?;
                    if updated == 1 {
                        delivered_devices.push(device_id);
                    } else {
                        revoked_devices.push(device_id);
                    }
                }
                Err(PushTransportError::InvalidToken) => {
                    let mut tx = self.database.begin(account_id).await?;
                    orm::execute(
                        &mut tx,
                        &delivery::Entity::delete_many()
                            .filter(delivery::Column::UserId.eq(account_id))
                            .filter(delivery::Column::DeviceId.eq(&device_id))
                            .filter(delivery::Column::Status.eq("pending"))
                            .into_query(),
                    )
                    .await?;
                    orm::execute(
                        &mut tx,
                        &device::Entity::delete_many()
                            .filter(device::Column::UserId.eq(account_id))
                            .filter(device::Column::DeviceId.eq(&device_id))
                            .into_query(),
                    )
                    .await?;
                    tx.commit().await?;
                    revoked_devices.push(device_id);
                }
                Err(PushTransportError::Unavailable(_)) => {
                    pending_devices.push(device_id);
                }
            }
        }

        Ok(GenericDeliverySummary {
            category: kind.as_wire().to_owned(),
            delivered_devices,
            pending_devices,
            revoked_devices,
        })
    }

    /// Counts pending alerts currently queued for this account.
    pub async fn pending_alert_count(&self, account_id: Uuid) -> Result<usize, ApiError> {
        let mut tx = self.database.begin(account_id).await?;
        let pending = orm::models::<delivery::Model>(
            &mut tx,
            &delivery::Entity::find()
                .filter(delivery::Column::UserId.eq(account_id))
                .filter(delivery::Column::Status.eq("pending"))
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(pending.len())
    }

    async fn device_and_delivery_still_active(
        &self,
        account_id: Uuid,
        device_id: &str,
        delivery_id: Uuid,
    ) -> Result<bool, ApiError> {
        let mut tx = self.database.begin(account_id).await?;
        prune_revoked_sessions_in(&mut tx, account_id).await?;
        let active_device = orm::model::<device::Model>(
            &mut tx,
            &device::Entity::find()
                .filter(device::Column::UserId.eq(account_id))
                .filter(device::Column::DeviceId.eq(device_id))
                .filter(device::Column::RevokedAt.is_null())
                .into_query(),
        )
        .await?;
        let pending_delivery = orm::model::<delivery::Model>(
            &mut tx,
            &delivery::Entity::find()
                .filter(delivery::Column::UserId.eq(account_id))
                .filter(delivery::Column::Id.eq(delivery_id))
                .filter(delivery::Column::Status.eq("pending"))
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(active_device.is_some() && pending_delivery.is_some())
    }
}

/// Clears push tokens and pending alerts bound to sessions authenticated at or before `cutoff`.
///
/// Called atomically inside [`crate::privacy::PrivacyStore`] when `Restriction::SessionsRevoke`
/// commits so sign-out everywhere leaves zero active device tokens or pending alerts behind.
pub(crate) async fn revoke_devices_before(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account_id: Uuid,
    cutoff: DateTime<Utc>,
) -> Result<DeviceCleanupOutcome, ApiError> {
    let revoked_devices = orm::models::<device::Model>(
        tx,
        &device::Entity::find()
            .filter(device::Column::UserId.eq(account_id))
            .filter(device::Column::SessionAuthenticatedAt.lte(cutoff))
            .into_query(),
    )
    .await?;
    if revoked_devices.is_empty() {
        return Ok(DeviceCleanupOutcome {
            cleared_devices: 0,
            cancelled_pending_alerts: 0,
        });
    }
    let ids: Vec<String> = revoked_devices
        .into_iter()
        .map(|row| row.device_id)
        .collect();
    let cancelled_pending_alerts = orm::execute(
        tx,
        &delivery::Entity::delete_many()
            .filter(delivery::Column::UserId.eq(account_id))
            .filter(delivery::Column::DeviceId.is_in(ids.clone()))
            .filter(delivery::Column::Status.eq("pending"))
            .into_query(),
    )
    .await?;
    let cleared_devices = orm::execute(
        tx,
        &device::Entity::delete_many()
            .filter(device::Column::UserId.eq(account_id))
            .filter(device::Column::DeviceId.is_in(ids))
            .into_query(),
    )
    .await?;
    Ok(DeviceCleanupOutcome {
        cleared_devices,
        cancelled_pending_alerts,
    })
}

async fn prune_revoked_sessions_in(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account_id: Uuid,
) -> Result<(), ApiError> {
    let fence = orm::model::<account_fence::Model>(
        tx,
        &account_fence::Entity::find_by_id(account_id).into_query(),
    )
    .await?
    .and_then(|row| row.sessions_revoked_before);
    if let Some(cutoff) = fence {
        let _ = revoke_devices_before(tx, account_id, cutoff).await?;
    }
    Ok(())
}

fn validate_device_id(raw: &str) -> Result<String, ApiError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_DEVICE_ID_BYTES {
        return Err(ApiError::Unprocessable(
            "device_id must be 1..=128 non-empty characters",
        ));
    }
    if !trimmed
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'))
    {
        return Err(ApiError::Unprocessable(
            "device_id contains unsupported characters",
        ));
    }
    Ok(trimmed.to_owned())
}

fn to_device_summary(model: device::Model) -> Result<RegisteredDeviceSummary, ApiError> {
    let platform = PushPlatform::from_wire(&model.platform)
        .ok_or(ApiError::Internal("stored push platform is invalid"))?;
    Ok(RegisteredDeviceSummary {
        device_id: model.device_id,
        platform,
        registered_at: model.registered_at,
        updated_at: model.updated_at,
    })
}

/// Router state for `/api/push/*` endpoints.
#[derive(Clone)]
pub struct PushState {
    /// Authenticated identity, privacy, and evidence owners.
    pub durable: DurableState,
    /// Multi-device push registration and generic delivery owner.
    pub push: PushStore,
}

/// Builds the authenticated multi-device push registration router.
pub fn router(state: PushState) -> Router {
    Router::new()
        .route(
            "/api/push/devices",
            get(list_devices).put(register_device_route),
        )
        .route(
            "/api/push/devices/{device_id}",
            delete(unregister_device_route),
        )
        .route("/api/push/sign-out", post(sign_out_all_route))
        .with_state(state)
}

async fn register_device_route(
    State(state): State<PushState>,
    headers: HeaderMap,
    Json(body): Json<RegisterDeviceRequest>,
) -> Result<Json<RegisteredDeviceSummary>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(state.push.register_device(&context, body).await?))
}

async fn list_devices(
    State(state): State<PushState>,
    headers: HeaderMap,
) -> Result<Json<Vec<RegisteredDeviceSummary>>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(state.push.list_active_devices(&context).await?))
}

async fn unregister_device_route(
    State(state): State<PushState>,
    headers: HeaderMap,
    Path(device_id): Path<String>,
) -> Result<Json<DeviceCleanupOutcome>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(
        state.push.unregister_device(&context, &device_id).await?,
    ))
}

async fn sign_out_all_route(
    State(state): State<PushState>,
    headers: HeaderMap,
) -> Result<Json<DeviceCleanupOutcome>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(state.push.sign_out_all_devices(&context).await?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_secrets_are_redacted_in_debug_output() {
        let secret = "apns-secret-token-should-never-log-918273";
        let token = PushDeviceToken::parse(secret).unwrap();
        let dispatch = GenericPushDispatch::new(
            Uuid::new_v4(),
            PushPlatform::Apns,
            token,
            GenericAlertKind::DoctorMessage,
        );
        assert!(!format!("{dispatch:?}").contains(secret));
        assert_eq!(dispatch.title, "Duet");
        assert_eq!(dispatch.body, "You have a new message in Duet.");

        let req = RegisterDeviceRequest {
            device_id: "iphone-pro".into(),
            platform: PushPlatform::Apns,
            push_token: secret.into(),
        };
        assert!(!format!("{req:?}").contains(secret));

        let model = device::Model {
            user_id: Uuid::new_v4(),
            device_id: "iphone-pro".into(),
            session_authenticated_at: Utc::now(),
            platform: "apns".into(),
            push_token: secret.into(),
            registered_at: Utc::now(),
            updated_at: Utc::now(),
            revoked_at: None,
        };
        assert!(!format!("{model:?}").contains(secret));
    }

    #[test]
    fn http_push_transport_enforces_https_except_loopback() {
        assert!(HttpPushTransport::new("https://push.example.test/v1/send", None).is_ok());
        assert!(HttpPushTransport::new("http://127.0.0.1:9099/send", None).is_ok());
        assert!(HttpPushTransport::new("http://push.example.test/v1/send", None).is_err());
    }

    #[test]
    fn device_id_validation_rejects_blank_or_control_characters() {
        assert!(validate_device_id("").is_err());
        assert!(validate_device_id("   ").is_err());
        assert!(validate_device_id("phone/../../etc").is_err());
        assert_eq!(
            validate_device_id("iphone-15_pro:1").unwrap(),
            "iphone-15_pro:1"
        );
    }
}
