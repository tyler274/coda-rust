//! Passwordless email link/short-code authentication, non-expiring sessions, and profile preferences.
//!
//! One email flow handles both sign-up and sign-in by issuing a single-use app
//! link and a 6-digit fallback code. Verified authentication mints an opaque
//! session token (`ds1.`) stored hashed in `public.duet_sessions` with no
//! time-based expiry: sessions end only on device sign-out, sign-out everywhere,
//! or account deletion.

pub use crate::email_auth_entity::{Appearance, ChallengePurpose};
use crate::{
    ApiError,
    database::{AccountDatabase, orm},
    email_auth_entity::{duet_session, email_challenge, email_directory, user_profile},
    identity::{AccessLevel, AccountContext, check_current},
};
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Duration, Utc};
use rand::RngExt;
use sea_orm::{
    ColumnTrait, EntityName, EntityTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
    sea_query::{Expr, ExprTrait, Func, OnConflict, Query},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Prefix for opaque non-expiring Duet session tokens.
pub const SESSION_TOKEN_PREFIX: &str = "ds1.";

/// Prefix for single-use email magic link tokens.
const LINK_TOKEN_PREFIX: &str = "dl1.";

/// Single-use email verification links and codes expire after 15 minutes.
pub const CHALLENGE_TTL_SECONDS: i64 = 15 * 60;

/// Maximum failed short-code attempts permitted against one challenge.
pub const MAX_CODE_ATTEMPTS: i32 = 5;

/// Validated, normalized email address (lower-cased, trimmed).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Parse and normalize an email address, rejecting blanks and malformed values.
    pub fn parse(raw: &str) -> Result<Self, ApiError> {
        let normalized = raw.trim().to_ascii_lowercase();
        if normalized.is_empty()
            || normalized.len() > 320
            || normalized
                .chars()
                .any(|c| c.is_whitespace() || c.is_control())
        {
            return Err(ApiError::Unprocessable("Enter a valid email address."));
        }
        let Some((local, domain)) = normalized.split_once('@') else {
            return Err(ApiError::Unprocessable("Enter a valid email address."));
        };
        if local.is_empty()
            || domain.is_empty()
            || domain.contains('@')
            || !domain.contains('.')
            || domain.starts_with('.')
            || domain.ends_with('.')
        {
            return Err(ApiError::Unprocessable("Enter a valid email address."));
        }
        Ok(Self(normalized))
    }

    /// Canonical normalized email text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Caller proof submitted to verify an email sign-in or email-change challenge.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ChallengeCredential {
    /// High-entropy single-use token carried by the app link.
    LinkToken(String),
    /// Six-digit numeric fallback code entered in the app.
    ShortCode(String),
}

impl std::fmt::Debug for ChallengeCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LinkToken(_) => f.write_str("ChallengeCredential::LinkToken(<redacted>)"),
            Self::ShortCode(_) => f.write_str("ChallengeCredential::ShortCode(<redacted>)"),
        }
    }
}

/// Outbound verification message passed to an [`AuthMailer`].
#[derive(Clone, PartialEq, Eq)]
pub struct ChallengeMessage {
    /// Recipient address.
    pub to: String,
    /// Whether this signs in / signs up or confirms a verified email change.
    pub purpose: ChallengePurpose,
    /// Single-use verification link targeting the app.
    pub link_url: String,
    /// Six-digit numeric fallback code.
    pub short_code: String,
    /// Raw link token for callers routing into custom schemes in tests.
    pub link_token: String,
}

impl std::fmt::Debug for ChallengeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChallengeMessage")
            .field("to", &self.to)
            .field("purpose", &self.purpose)
            .field("link_url", &"<redacted>")
            .field("short_code", &"<redacted>")
            .field("link_token", &"<redacted>")
            .finish()
    }
}

/// Failure delivering a verification email while preserving underlying cause.
#[derive(Debug, thiserror::Error)]
pub enum EmailDeliveryError {
    /// Mail transport is not configured for this environment.
    #[error("email delivery is not configured")]
    Unconfigured,
    /// Mail transport failed cleanly with an underlying cause.
    #[error("email delivery failed")]
    Transport(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Delivery port for single-use sign-in and email-change messages.
#[async_trait]
pub trait AuthMailer: Send + Sync {
    /// Send a verification link and 6-digit fallback code to the recipient.
    async fn send_challenge(&self, message: ChallengeMessage) -> Result<(), EmailDeliveryError>;
}

/// Fail-closed mailer when no mail transport has been configured.
#[derive(Clone, Copy, Default)]
pub struct UnconfiguredMailer;

#[async_trait]
impl AuthMailer for UnconfiguredMailer {
    async fn send_challenge(&self, _message: ChallengeMessage) -> Result<(), EmailDeliveryError> {
        Err(EmailDeliveryError::Unconfigured)
    }
}

/// Workspace SMTP relay mailer via `gmail::SmtpRelay`.
pub struct SmtpRelayMailer {
    relay: gmail::SmtpRelay,
}

impl SmtpRelayMailer {
    /// Configure SMTP relay delivery from a validated sender address.
    pub fn new(from: &str) -> Result<Self, EmailDeliveryError> {
        let relay = gmail::SmtpRelay::google(from, "Duet by Coda Health")
            .map_err(|err| EmailDeliveryError::Transport(Box::new(err)))?;
        Ok(Self { relay })
    }
}

#[async_trait]
impl AuthMailer for SmtpRelayMailer {
    async fn send_challenge(&self, message: ChallengeMessage) -> Result<(), EmailDeliveryError> {
        let (subject, heading) = match message.purpose {
            ChallengePurpose::SignIn => (
                "Sign in to Duet",
                "Open the single-use link below on your phone to sign in to Duet, or enter the 6-digit code in the app.",
            ),
            ChallengePurpose::EmailChange => (
                "Confirm your new Duet email address",
                "Open the single-use link below to confirm your new Duet email address, or enter the 6-digit code in the app.",
            ),
        };
        let body = format!(
            "{heading}\n\nLink: {}\nCode: {}\n\nThis link and code work once and expire in 15 minutes.",
            message.link_url, message.short_code
        );
        self.relay
            .send(&message.to, subject, &body)
            .await
            .map_err(|err| EmailDeliveryError::Transport(Box::new(err)))
    }
}

/// In-memory mailer for deterministic local and integration tests.
#[derive(Clone, Default)]
pub struct MemoryMailer {
    sent: Arc<Mutex<Vec<ChallengeMessage>>>,
}

impl MemoryMailer {
    /// Create an empty recording mailer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Take and return all messages recorded so far in delivery order.
    pub fn take(&self) -> Vec<ChallengeMessage> {
        std::mem::take(&mut self.sent.lock().expect("mailer lock"))
    }

    /// Read all messages recorded so far without clearing them.
    pub fn snapshot(&self) -> Vec<ChallengeMessage> {
        self.sent.lock().expect("mailer lock").clone()
    }
}

#[async_trait]
impl AuthMailer for MemoryMailer {
    async fn send_challenge(&self, message: ChallengeMessage) -> Result<(), EmailDeliveryError> {
        self.sent.lock().expect("mailer lock").push(message);
        Ok(())
    }
}

/// Profile details exposed outside `duet_wire::AccountState`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountProfile {
    /// Verified email address for email-authenticated accounts.
    pub email: Option<String>,
    /// Selected UI appearance preference (`system`, `light`, or `dark`).
    pub appearance: Appearance,
}

/// Response returned after starting an email sign-in or email-change challenge.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeStarted {
    /// Confirmation message safe to render directly in the UI.
    pub message: String,
}

/// Verified non-expiring session returned on completing email sign-in/sign-up.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailSignInResult {
    /// Opaque bearer token (`ds1.`).
    pub bearer: String,
    /// Stable internal account ID.
    pub account_id: Uuid,
    /// Active account incarnation.
    pub incarnation: Uuid,
    /// Verified account email.
    pub email: String,
    /// Selected appearance preference.
    pub appearance: Appearance,
}

impl std::fmt::Debug for EmailSignInResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmailSignInResult")
            .field("bearer", &"<redacted>")
            .field("account_id", &self.account_id)
            .field("incarnation", &self.incarnation)
            .field("email", &self.email)
            .field("appearance", &self.appearance)
            .finish()
    }
}

/// Store coordinating passwordless email challenges, opaque non-expiring sessions, and profile settings.
#[derive(Clone)]
pub struct EmailAuthStore {
    database: AccountDatabase,
    mailer: Arc<dyn AuthMailer>,
    app_origin: String,
}

impl EmailAuthStore {
    /// Construct an email auth store with explicit database, mailer, and canonical app link origin.
    pub fn new(
        database: AccountDatabase,
        mailer: Arc<dyn AuthMailer>,
        app_origin: impl Into<String>,
    ) -> Self {
        Self {
            database,
            mailer,
            app_origin: app_origin.into().trim_end_matches('/').to_owned(),
        }
    }

    /// Send a single-use magic link + 6-digit fallback code to sign up or sign in.
    pub async fn begin_sign_in(
        &self,
        raw_email: &str,
        invite: Option<&str>,
    ) -> Result<ChallengeStarted, ApiError> {
        let email = EmailAddress::parse(raw_email)?;
        let now = Utc::now();
        let (challenge_id, message) = self
            .create_challenge(ChallengePurpose::SignIn, &email, None, invite, now)
            .await?;
        if let Err(err) = self.mailer.send_challenge(message).await {
            let _ = self.delete_challenge(challenge_id).await;
            return Err(match err {
                EmailDeliveryError::Unconfigured => {
                    ApiError::Unavailable("Email delivery is not configured.")
                }
                EmailDeliveryError::Transport(_) => {
                    ApiError::Unavailable("We couldn’t send the email right now. Try again.")
                }
            });
        }
        Ok(ChallengeStarted {
            message: "Check your email".to_owned(),
        })
    }

    /// Verify a single-use link token or 6-digit code and mint a non-expiring `ds1.` session.
    pub async fn complete_sign_in(
        &self,
        raw_email: Option<&str>,
        credential: &ChallengeCredential,
    ) -> Result<EmailSignInResult, ApiError> {
        let now = Utc::now();
        let challenge = self
            .consume_challenge(ChallengePurpose::SignIn, None, raw_email, credential, now)
            .await?;

        let email = EmailAddress::parse(&challenge.email)?;
        let (account_id, appearance) = self.find_or_provision_email_account(&email).await?;

        let mut account_tx = self.database.begin(account_id).await?;
        let user = orm::model::<user_profile::Model>(
            &mut account_tx,
            &user_profile::Entity::find()
                .filter(user_profile::Column::Id.eq(account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;
        account_tx.commit().await?;

        let mut raw_session = [0u8; 32];
        rand::rng().fill(&mut raw_session);
        let bearer = format!(
            "{SESSION_TOKEN_PREFIX}{}",
            URL_SAFE_NO_PAD.encode(raw_session)
        );
        let token_hash = sha256_bytes(bearer.as_bytes());
        let session_id = Uuid::new_v4();

        let mut tx = self.database.unauthenticated_tx().await?;
        let insert_session = Query::insert()
            .into_table(duet_session::Entity.table_ref())
            .columns([
                duet_session::Column::Id,
                duet_session::Column::UserId,
                duet_session::Column::AccountIncarnation,
                duet_session::Column::TokenHash,
                duet_session::Column::CreatedAt,
            ])
            .values_panic([
                session_id.into(),
                account_id.into(),
                user.duet_incarnation.into(),
                token_hash.into(),
                now.into(),
            ])
            .to_owned();
        orm::execute(&mut tx, &insert_session).await?;
        tx.commit().await?;

        Ok(EmailSignInResult {
            bearer,
            account_id,
            incarnation: user.duet_incarnation,
            email: email.0,
            appearance,
        })
    }

    async fn load_session_row(&self, bearer: &str) -> Result<duet_session::Model, ApiError> {
        if !bearer.starts_with(SESSION_TOKEN_PREFIX) || bearer.len() <= SESSION_TOKEN_PREFIX.len() {
            return Err(ApiError::Unauthorized);
        }
        let token_hash = sha256_bytes(bearer.as_bytes());
        let mut tx = self.database.unauthenticated_tx().await?;
        let session = orm::model::<duet_session::Model>(
            &mut tx,
            &duet_session::Entity::find()
                .filter(duet_session::Column::TokenHash.eq(token_hash))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::Unauthorized)?;
        tx.commit().await?;

        if session.revoked_at.is_some() {
            return Err(ApiError::SessionRevoked);
        }
        Ok(session)
    }

    /// Resolve a `ds1.` session bearer to an [`AccountContext`]; sessions never time-expire.
    pub(crate) async fn verify_session(&self, bearer: &str) -> Result<AccountContext, ApiError> {
        let session = self.load_session_row(bearer).await?;

        let mut account_tx = self.database.begin(session.user_id).await?;
        let user = orm::model::<user_profile::Model>(
            &mut account_tx,
            &user_profile::Entity::find()
                .filter(user_profile::Column::Id.eq(session.user_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;

        if user
            .sessions_revoked_before
            .is_some_and(|fence| session.created_at <= fence)
        {
            return Err(ApiError::SessionRevoked);
        }
        account_tx.commit().await?;

        Ok(AccountContext {
            account_id: session.user_id,
            incarnation: user.duet_incarnation,
            permission_revision: user.intelligence_control_revision,
            authenticated_at: session.created_at,
        })
    }

    /// Resolve a `ds1.` bearer strictly for idempotent privacy receipt readback after
    /// `SessionsRevoke` or `AccountDelete`. `PrivacyStore::restrict` enforces `check_current`
    /// whenever the requested `event_id` has not already been committed.
    pub(crate) async fn verify_session_for_receipt_retry(
        &self,
        bearer: &str,
    ) -> Result<AccountContext, ApiError> {
        let session = self.load_session_row(bearer).await?;

        let mut account_tx = self.database.begin(session.user_id).await?;
        let user = orm::model::<user_profile::Model>(
            &mut account_tx,
            &user_profile::Entity::find()
                .filter(user_profile::Column::Id.eq(session.user_id))
                .into_query(),
        )
        .await?;
        account_tx.commit().await?;

        match user {
            Some(user) => Ok(AccountContext {
                account_id: session.user_id,
                incarnation: user.duet_incarnation,
                permission_revision: user.intelligence_control_revision,
                authenticated_at: session.created_at,
            }),
            None => Ok(AccountContext {
                account_id: session.user_id,
                incarnation: session.account_incarnation,
                permission_revision: 0,
                authenticated_at: session.created_at,
            }),
        }
    }

    /// Revoke only the caller's current `ds1.` device session (other devices stay signed in).
    pub async fn sign_out_device(&self, bearer: &str) -> Result<(), ApiError> {
        if !bearer.starts_with(SESSION_TOKEN_PREFIX) {
            return Ok(());
        }
        let token_hash = sha256_bytes(bearer.as_bytes());
        let now = Utc::now();
        let mut tx = self.database.unauthenticated_tx().await?;
        orm::execute(
            &mut tx,
            &duet_session::Entity::update_many()
                .col_expr(duet_session::Column::RevokedAt, Expr::val(Some(now)))
                .filter(duet_session::Column::TokenHash.eq(token_hash))
                .filter(duet_session::Column::RevokedAt.is_null())
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Start verified change-email confirmation for the signed-in user (accessible while Terms wait).
    pub async fn begin_email_change(
        &self,
        context: &AccountContext,
        raw_new_email: &str,
    ) -> Result<ChallengeStarted, ApiError> {
        let new_email = EmailAddress::parse(raw_new_email)?;
        let mut account_tx = self.database.begin(context.account_id).await?;
        check_current(&mut account_tx, context, AccessLevel::RightsOnly).await?;
        let current = orm::model::<user_profile::Model>(
            &mut account_tx,
            &user_profile::Entity::find()
                .filter(user_profile::Column::Id.eq(context.account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;
        if current.email.as_deref() == Some(new_email.as_str()) {
            return Err(ApiError::Conflict("That is already your email address."));
        }
        account_tx.commit().await?;

        self.ensure_email_unclaimed(&new_email, Some(context.account_id))
            .await?;

        let now = Utc::now();
        let (challenge_id, message) = self
            .create_challenge(
                ChallengePurpose::EmailChange,
                &new_email,
                Some(context.account_id),
                None,
                now,
            )
            .await?;
        if let Err(err) = self.mailer.send_challenge(message).await {
            let _ = self.delete_challenge(challenge_id).await;
            return Err(match err {
                EmailDeliveryError::Unconfigured => {
                    ApiError::Unavailable("Email delivery is not configured.")
                }
                EmailDeliveryError::Transport(_) => {
                    ApiError::Unavailable("We couldn’t send the verification email right now.")
                }
            });
        }
        Ok(ChallengeStarted {
            message: "Check your email".to_owned(),
        })
    }

    /// Confirm verified email change for the signed-in user without altering account ownership.
    pub async fn confirm_email_change(
        &self,
        context: &AccountContext,
        raw_new_email: Option<&str>,
        credential: &ChallengeCredential,
    ) -> Result<AccountProfile, ApiError> {
        let now = Utc::now();
        let challenge = self
            .consume_challenge(
                ChallengePurpose::EmailChange,
                Some(context.account_id),
                raw_new_email,
                credential,
                now,
            )
            .await?;
        let new_email = EmailAddress::parse(&challenge.email)?;
        self.ensure_email_unclaimed(&new_email, Some(context.account_id))
            .await?;

        let mut account_tx = self.database.begin(context.account_id).await?;
        check_current(&mut account_tx, context, AccessLevel::RightsOnly).await?;
        orm::execute(
            &mut account_tx,
            &user_profile::Entity::update_many()
                .col_expr(
                    user_profile::Column::Email,
                    Expr::val(Some(new_email.0.clone())),
                )
                .filter(user_profile::Column::Id.eq(context.account_id))
                .into_query(),
        )
        .await
        .map_err(|err| {
            if err.to_string().contains("duet_users_email_unique") {
                ApiError::Conflict("That email address is already linked to another account.")
            } else {
                ApiError::Storage(err.to_string())
            }
        })?;
        let user = orm::model::<user_profile::Model>(
            &mut account_tx,
            &user_profile::Entity::find()
                .filter(user_profile::Column::Id.eq(context.account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;
        account_tx.commit().await?;

        Ok(AccountProfile {
            email: user.email,
            appearance: user.appearance,
        })
    }

    /// Read signed-in account email and appearance preference (allowed while Terms wait).
    pub async fn read_profile(&self, context: &AccountContext) -> Result<AccountProfile, ApiError> {
        let mut account_tx = self.database.begin(context.account_id).await?;
        check_current(&mut account_tx, context, AccessLevel::RightsOnly).await?;
        let user = orm::model::<user_profile::Model>(
            &mut account_tx,
            &user_profile::Entity::find()
                .filter(user_profile::Column::Id.eq(context.account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;
        account_tx.commit().await?;
        Ok(AccountProfile {
            email: user.email,
            appearance: user.appearance,
        })
    }

    /// Update UI appearance preference (allowed while Terms wait).
    pub async fn set_appearance(
        &self,
        context: &AccountContext,
        appearance: Appearance,
    ) -> Result<AccountProfile, ApiError> {
        let mut account_tx = self.database.begin(context.account_id).await?;
        check_current(&mut account_tx, context, AccessLevel::RightsOnly).await?;
        orm::execute(
            &mut account_tx,
            &user_profile::Entity::update_many()
                .col_expr(user_profile::Column::Appearance, Expr::val(appearance))
                .filter(user_profile::Column::Id.eq(context.account_id))
                .into_query(),
        )
        .await?;
        let user = orm::model::<user_profile::Model>(
            &mut account_tx,
            &user_profile::Entity::find()
                .filter(user_profile::Column::Id.eq(context.account_id))
                .into_query(),
        )
        .await?
        .ok_or(ApiError::AbsentAccount)?;
        account_tx.commit().await?;
        Ok(AccountProfile {
            email: user.email,
            appearance: user.appearance,
        })
    }

    async fn create_challenge(
        &self,
        purpose: ChallengePurpose,
        email: &EmailAddress,
        user_id: Option<Uuid>,
        invite: Option<&str>,
        now: DateTime<Utc>,
    ) -> Result<(Uuid, ChallengeMessage), ApiError> {
        let challenge_id = Uuid::new_v4();
        let (link_token, short_code) = generate_challenge_secrets();
        let link_token_hash = sha256_bytes(link_token.as_bytes());
        let short_code_hash = short_code_digest(purpose, email.as_str(), &short_code);
        let expires_at = now + Duration::seconds(CHALLENGE_TTL_SECONDS);

        let mut tx = self.database.unauthenticated_tx().await?;
        let insert = Query::insert()
            .into_table(email_challenge::Entity.table_ref())
            .columns([
                email_challenge::Column::Id,
                email_challenge::Column::Purpose,
                email_challenge::Column::Email,
                email_challenge::Column::UserId,
                email_challenge::Column::LinkTokenHash,
                email_challenge::Column::ShortCodeHash,
                email_challenge::Column::ExpiresAt,
                email_challenge::Column::CreatedAt,
            ])
            .values_panic([
                challenge_id.into(),
                purpose.as_str().into(),
                email.as_str().into(),
                user_id.into(),
                link_token_hash.into(),
                short_code_hash.into(),
                expires_at.into(),
                now.into(),
            ])
            .to_owned();
        orm::execute(&mut tx, &insert).await?;
        tx.commit().await?;

        let link_url = build_challenge_link(&self.app_origin, purpose, &link_token, invite)?;
        Ok((
            challenge_id,
            ChallengeMessage {
                to: email.0.clone(),
                purpose,
                link_url,
                short_code,
                link_token,
            },
        ))
    }

    async fn delete_challenge(&self, challenge_id: Uuid) -> Result<(), ApiError> {
        let mut tx = self.database.unauthenticated_tx().await?;
        orm::execute(
            &mut tx,
            &email_challenge::Entity::delete_many()
                .filter(email_challenge::Column::Id.eq(challenge_id))
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn consume_challenge(
        &self,
        purpose: ChallengePurpose,
        required_user: Option<Uuid>,
        raw_email: Option<&str>,
        credential: &ChallengeCredential,
        now: DateTime<Utc>,
    ) -> Result<email_challenge::Model, ApiError> {
        let normalized_email = raw_email.map(EmailAddress::parse).transpose()?;
        let mut tx = self.database.unauthenticated_tx().await?;

        let candidate = match credential {
            ChallengeCredential::LinkToken(raw_token) => {
                let token = raw_token.trim();
                if !token.starts_with(LINK_TOKEN_PREFIX) {
                    return Err(ApiError::InvalidChallenge("This link no longer works"));
                }
                let digest = sha256_bytes(token.as_bytes());
                let mut query = email_challenge::Entity::find()
                    .filter(email_challenge::Column::Purpose.eq(purpose))
                    .filter(email_challenge::Column::LinkTokenHash.eq(digest));
                if let Some(uid) = required_user {
                    query = query.filter(email_challenge::Column::UserId.eq(uid));
                }
                let found = orm::model::<email_challenge::Model>(&mut tx, &query.into_query())
                    .await?
                    .ok_or(ApiError::InvalidChallenge("This link no longer works"))?;
                if let Some(ref email) = normalized_email
                    && found.email != email.as_str()
                {
                    return Err(ApiError::InvalidChallenge("This link no longer works"));
                }
                found
            }
            ChallengeCredential::ShortCode(raw_code) => {
                let email = normalized_email.ok_or(ApiError::InvalidChallenge(
                    "Enter your email address and code to continue.",
                ))?;
                let code: String = raw_code.chars().filter(char::is_ascii_digit).collect();
                if code.len() != 6 {
                    return Err(ApiError::InvalidChallenge("This link no longer works"));
                }
                let mut query = email_challenge::Entity::find()
                    .filter(email_challenge::Column::Purpose.eq(purpose))
                    .filter(email_challenge::Column::Email.eq(email.as_str()))
                    .filter(email_challenge::Column::ConsumedAt.is_null())
                    .order_by_desc(email_challenge::Column::CreatedAt)
                    .limit(1);
                if let Some(uid) = required_user {
                    query = query.filter(email_challenge::Column::UserId.eq(uid));
                }
                let latest = orm::model::<email_challenge::Model>(&mut tx, &query.into_query())
                    .await?
                    .ok_or(ApiError::InvalidChallenge("This link no longer works"))?;
                if latest.expires_at <= now || latest.attempts >= MAX_CODE_ATTEMPTS {
                    return Err(ApiError::InvalidChallenge("This link no longer works"));
                }
                let expected_hash = short_code_digest(purpose, email.as_str(), &code);
                if !constant_time_eq(&latest.short_code_hash, &expected_hash) {
                    orm::execute(
                        &mut tx,
                        &email_challenge::Entity::update_many()
                            .col_expr(
                                email_challenge::Column::Attempts,
                                Expr::col(email_challenge::Column::Attempts).add(1),
                            )
                            .filter(email_challenge::Column::Id.eq(latest.id))
                            .filter(email_challenge::Column::ConsumedAt.is_null())
                            .into_query(),
                    )
                    .await?;
                    tx.commit().await?;
                    return Err(ApiError::InvalidChallenge("This link no longer works"));
                }
                latest
            }
        };

        if candidate.consumed_at.is_some()
            || candidate.expires_at <= now
            || candidate.attempts >= MAX_CODE_ATTEMPTS
        {
            return Err(ApiError::InvalidChallenge("This link no longer works"));
        }

        let affected = orm::execute(
            &mut tx,
            &email_challenge::Entity::update_many()
                .col_expr(email_challenge::Column::ConsumedAt, Expr::val(Some(now)))
                .filter(email_challenge::Column::Id.eq(candidate.id))
                .filter(email_challenge::Column::ConsumedAt.is_null())
                .filter(email_challenge::Column::ExpiresAt.gt(now))
                .filter(email_challenge::Column::Attempts.lt(MAX_CODE_ATTEMPTS))
                .into_query(),
        )
        .await?;
        if affected != 1 {
            return Err(ApiError::InvalidChallenge("This link no longer works"));
        }
        tx.commit().await?;
        Ok(candidate)
    }

    async fn ensure_email_unclaimed(
        &self,
        email: &EmailAddress,
        except_user: Option<Uuid>,
    ) -> Result<(), ApiError> {
        let mut tx = self.database.unauthenticated_tx().await?;
        let existing = orm::model::<email_directory::Model>(
            &mut tx,
            &email_directory::Entity::find()
                .filter(
                    Expr::expr(Func::lower(Expr::col(email_directory::Column::Email)))
                        .eq(email.as_str()),
                )
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        if let Some(found) = existing
            && Some(found.id) != except_user
        {
            return Err(ApiError::Conflict(
                "That email address is already linked to another account.",
            ));
        }
        Ok(())
    }

    async fn find_or_provision_email_account(
        &self,
        email: &EmailAddress,
    ) -> Result<(Uuid, Appearance), ApiError> {
        let mut lookup_tx = self.database.unauthenticated_tx().await?;
        if let Some(existing) = orm::model::<email_directory::Model>(
            &mut lookup_tx,
            &email_directory::Entity::find()
                .filter(
                    Expr::expr(Func::lower(Expr::col(email_directory::Column::Email)))
                        .eq(email.as_str()),
                )
                .into_query(),
        )
        .await?
        {
            lookup_tx.commit().await?;
            return Ok((existing.id, existing.appearance));
        }
        lookup_tx.commit().await?;

        let new_id = Uuid::new_v4();
        let subject = format!("email:{new_id}");
        let mut account_tx = self.database.begin(new_id).await?;
        // Serialize concurrent sign-up for the same email before RLS insert.
        orm::row(
            &mut account_tx,
            &Query::select()
                .expr(
                    Func::cust("pg_advisory_xact_lock").args([Expr::expr(
                        Func::cust("hashtextextended")
                            .args([Expr::val(email.as_str()), Expr::val(4603)]),
                    )]),
                )
                .to_owned(),
        )
        .await?;

        if let Some(existing) = orm::model::<email_directory::Model>(
            &mut account_tx,
            &email_directory::Entity::find()
                .filter(
                    Expr::expr(Func::lower(Expr::col(email_directory::Column::Email)))
                        .eq(email.as_str()),
                )
                .into_query(),
        )
        .await?
        {
            account_tx.rollback().await?;
            return Ok((existing.id, existing.appearance));
        }

        let insert = Query::insert()
            .into_table(user_profile::Entity.table_ref())
            .columns([
                user_profile::Column::Id,
                user_profile::Column::AuthUserId,
                user_profile::Column::Email,
                user_profile::Column::SharingLabel,
            ])
            .values_panic([
                new_id.into(),
                subject.into(),
                email.as_str().into(),
                "Duet".into(),
            ])
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .to_owned();
        orm::execute(&mut account_tx, &insert).await?;
        account_tx.commit().await?;
        Ok((new_id, Appearance::System))
    }
}

fn generate_challenge_secrets() -> (String, String) {
    let mut rng = rand::rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    let link_token = format!("{LINK_TOKEN_PREFIX}{}", URL_SAFE_NO_PAD.encode(bytes));
    let code_number: u32 = rng.random_range(0..1_000_000);
    let short_code = format!("{code_number:06}");
    (link_token, short_code)
}

fn sha256_bytes(bytes: &[u8]) -> Vec<u8> {
    Sha256::digest(bytes).to_vec()
}

fn short_code_digest(purpose: ChallengePurpose, email: &str, code: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(purpose.as_str().as_bytes());
    hasher.update(b":");
    hasher.update(email.as_bytes());
    hasher.update(b":");
    hasher.update(code.as_bytes());
    hasher.finalize().to_vec()
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

fn build_challenge_link(
    origin: &str,
    purpose: ChallengePurpose,
    link_token: &str,
    invite: Option<&str>,
) -> Result<String, ApiError> {
    let path = match purpose {
        ChallengePurpose::SignIn => "auth/email",
        ChallengePurpose::EmailChange => "auth/email-change",
    };
    let mut parsed = url::Url::parse(origin)
        .map_err(|_| ApiError::Internal("invalid application origin for email link"))?;
    parsed.set_path(path);
    {
        let mut pairs = parsed.query_pairs_mut();
        pairs.append_pair("token", link_token);
        if let Some(invite) = invite.map(str::trim).filter(|v| {
            !v.is_empty()
                && v.len() <= 128
                && v.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        }) {
            pairs.append_pair("invite", invite);
        }
    }
    Ok(parsed.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_validation_normalizes_and_rejects_invalid_inputs() {
        assert_eq!(
            EmailAddress::parse("  Ada.Lovelace@Example.COM ")
                .unwrap()
                .as_str(),
            "ada.lovelace@example.com"
        );
        for bad in [
            "",
            "ada",
            "ada@",
            "@example.com",
            "ada@localhost",
            "a b@c.com",
        ] {
            assert!(EmailAddress::parse(bad).is_err(), "{bad:?} should fail");
        }
    }

    #[test]
    fn challenge_link_builds_cleanly_with_sanitized_invite() {
        let url = build_challenge_link(
            "https://codahealth.ai",
            ChallengePurpose::SignIn,
            "dl1.abc_123",
            Some("inv_99-ok"),
        )
        .unwrap();
        assert_eq!(
            url,
            "https://codahealth.ai/auth/email?token=dl1.abc_123&invite=inv_99-ok"
        );
        let stripped = build_challenge_link(
            "https://codahealth.ai",
            ChallengePurpose::EmailChange,
            "dl1.xyz",
            Some("bad/invite?x=1"),
        )
        .unwrap();
        assert_eq!(
            stripped,
            "https://codahealth.ai/auth/email-change?token=dl1.xyz"
        );
    }
}
