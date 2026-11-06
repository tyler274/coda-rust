//! Durable standalone Identity Platform account boundary, independent of practice invitations.
use crate::{ApiError, database::AccountDatabase};
use async_trait::async_trait;
use base64::Engine;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

/// Verified provider identity. Never constructed from an email or a front-door UUID.
#[derive(Clone, Debug)]
pub struct VerifiedIdentity {
    /// Identity Platform immutable localId/sub.
    pub subject: String,
    /// Provider authentication time, not token refresh time.
    pub authenticated_at: DateTime<Utc>,
}
/// Injectable verifier; adapters must validate signature, project, expiry and disabled/revoked users.
#[async_trait]
pub trait IdentityVerifier: Send + Sync {
    /// Verify the bearer using the provider authority.
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError>;
}
/// Identity Platform's accounts:lookup validates the supplied ID token online.
/// This keeps provider revocation/disabled state authoritative, without accepting development tokens.
pub struct IdentityPlatform {
    client: reqwest::Client,
    project: String,
    api_key: String,
}
impl IdentityPlatform {
    /// Configure the existing Firebase project and public Identity Platform API key.
    pub fn new(project: String, api_key: String) -> Result<Self, ApiError> {
        if project.is_empty() || api_key.is_empty() {
            return Err(ApiError::Unavailable("Identity Platform is not configured"));
        }
        Ok(Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .map_err(|_| ApiError::Unavailable("Identity transport unavailable"))?,
            project,
            api_key,
        })
    }
}
#[async_trait]
impl IdentityVerifier for IdentityPlatform {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        let response = self
            .client
            .post("https://identitytoolkit.googleapis.com/v1/accounts:lookup")
            .query(&[("key", &self.api_key)])
            .json(&serde_json::json!({"idToken":bearer}))
            .send()
            .await
            .map_err(|_| ApiError::Unavailable("Identity verification unavailable"))?;
        if !response.status().is_success() {
            return Err(ApiError::Unauthorized);
        }
        let lookup: serde_json::Value =
            response.json().await.map_err(|_| ApiError::Unauthorized)?;
        // Parse claims only AFTER the provider has verified this exact bearer.
        let parts: Vec<_> = bearer.split('.').collect();
        if parts.len() != 3 {
            return Err(ApiError::Unauthorized);
        }
        let claims: serde_json::Value = serde_json::from_slice(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(parts[1])
                .map_err(|_| ApiError::Unauthorized)?,
        )
        .map_err(|_| ApiError::Unauthorized)?;
        let user = lookup
            .get("users")
            .and_then(|x| x.get(0))
            .ok_or(ApiError::Unauthorized)?;
        let subject = claims["sub"]
            .as_str()
            .filter(|s| !s.is_empty() && s.len() <= 128)
            .ok_or(ApiError::Unauthorized)?;
        let auth = claims["auth_time"].as_i64().ok_or(ApiError::Unauthorized)?;
        let now = Utc::now().timestamp();
        let valid_since = user["validSince"]
            .as_str()
            .and_then(|x| x.parse::<i64>().ok())
            .unwrap_or(0);
        if user["localId"].as_str() != Some(subject)
            || user["disabled"].as_bool() == Some(true)
            || claims["aud"].as_str() != Some(&self.project)
            || claims["iss"].as_str()
                != Some(&format!("https://securetoken.google.com/{}", self.project))
            || claims["exp"].as_i64().unwrap_or(0) <= now
            || auth > now + 30
            || auth < valid_since
        {
            return Err(ApiError::Unauthorized);
        }
        Ok(VerifiedIdentity {
            subject: subject.to_owned(),
            authenticated_at: DateTime::from_timestamp(auth, 0).ok_or(ApiError::Unauthorized)?,
        })
    }
}
/// Issuer and audience of [`LocalIdentity`] tokens.
pub const LOCAL_ISSUER: &str = "duet-local";

/// Local-stack verifier for HS256 tokens signed with an operator-chosen secret.
/// Hosted startup refuses it (`hosting::validate_startup`); it stands in for
/// Identity Platform so the served binary runs without cloud identity.
pub struct LocalIdentity {
    key: jsonwebtoken::DecodingKey,
}
impl LocalIdentity {
    /// The secret must carry at least 32 bytes.
    pub fn new(secret: &str) -> Result<Self, ApiError> {
        if secret.len() < 32 {
            return Err(ApiError::Unavailable(
                "local identity secret must be at least 32 bytes",
            ));
        }
        Ok(Self {
            key: jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        })
    }
}
#[derive(Deserialize)]
struct LocalClaims {
    sub: String,
    auth_time: i64,
}
#[async_trait]
impl IdentityVerifier for LocalIdentity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_issuer(&[LOCAL_ISSUER]);
        validation.set_audience(&[LOCAL_ISSUER]);
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);
        let claims = jsonwebtoken::decode::<LocalClaims>(bearer, &self.key, &validation)
            .map_err(|_| ApiError::Unauthorized)?
            .claims;
        if claims.sub.is_empty()
            || claims.sub.len() > 128
            || claims.auth_time > Utc::now().timestamp() + 30
        {
            return Err(ApiError::Unauthorized);
        }
        Ok(VerifiedIdentity {
            subject: claims.sub,
            authenticated_at: DateTime::from_timestamp(claims.auth_time, 0)
                .ok_or(ApiError::Unauthorized)?,
        })
    }
}
/// An account incarnation and permission/session fence captured for a request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountContext {
    /// Existing public.users primary key.
    pub account_id: Uuid,
    /// Distinguishes a restored/deleted/recreated account from a current owner.
    pub incarnation: Uuid,
    /// Permission revision for accepted work and late-output fencing.
    pub permission_revision: i64,
    /// Authentication time checked again by mutation owners.
    pub authenticated_at: DateTime<Utc>,
}
/// Durable identity and account settings owner.
#[derive(Clone)]
pub struct IdentityStore {
    /// Shared account transaction boundary.
    pub database: AccountDatabase,
    verifier: Arc<dyn IdentityVerifier>,
}
impl IdentityStore {
    /// Construct with a production or explicit test verifier.
    pub fn new(database: AccountDatabase, verifier: Arc<dyn IdentityVerifier>) -> Self {
        Self { database, verifier }
    }
    /// Verify and map a subject; provisioning is explicit and never invitation gated.
    pub async fn authenticate(
        &self,
        bearer: &str,
        provision: bool,
    ) -> Result<AccountContext, ApiError> {
        let identity = self.verifier.verify(bearer).await?;
        if self.database.subject_deleted(&identity.subject).await? {
            return Err(ApiError::AbsentAccount);
        }
        let account = match self.database.resolve_subject(&identity.subject).await? {
            Some(id) => id,
            None if provision => {
                let id = Uuid::new_v4();
                let mut tx = self.database.begin(id).await?;
                // Serialize first provisioning by immutable subject, not email.
                sqlx::query("select pg_advisory_xact_lock(hashtextextended($1,4602))")
                    .bind(&identity.subject)
                    .execute(&mut *tx)
                    .await?;
                let existing: Option<Uuid> = sqlx::query_scalar(
                    "select id from public.duet_identity_directory where auth_user_id=$1",
                )
                .bind(&identity.subject)
                .fetch_optional(&mut *tx)
                .await?;
                if let Some(existing) = existing {
                    tx.rollback().await?;
                    existing
                } else {
                    sqlx::query("insert into public.users(id,auth_user_id,sharing_label) values($1,$2,'Duet')").bind(id).bind(&identity.subject).execute(&mut *tx).await?;
                    tx.commit().await?;
                    id
                }
            }
            None => return Err(ApiError::AbsentAccount),
        };
        let mut tx = self.database.begin(account).await?;
        let row=sqlx::query("select duet_incarnation,intelligence_control_revision,sessions_revoked_before from public.users where id=$1").bind(account).fetch_one(&mut *tx).await?;
        let fence: Option<DateTime<Utc>> = row.get("sessions_revoked_before");
        if fence.is_some_and(|f| identity.authenticated_at <= f) {
            return Err(ApiError::SessionRevoked);
        }
        Ok(AccountContext {
            account_id: account,
            incarnation: row.get("duet_incarnation"),
            permission_revision: row.get("intelligence_control_revision"),
            authenticated_at: identity.authenticated_at,
        })
    }
    /// Verify provider identity for readback of an already committed privacy action.
    /// This does not authorize a new mutation; `PrivacyStore` checks the session before creating an event.
    pub async fn authenticate_receipt_retry(
        &self,
        bearer: &str,
    ) -> Result<AccountContext, ApiError> {
        let identity = self.verifier.verify(bearer).await?;
        if let Some((account_id, incarnation)) =
            self.database.deleted_subject(&identity.subject).await?
        {
            return Ok(AccountContext {
                account_id,
                incarnation,
                permission_revision: 0,
                authenticated_at: identity.authenticated_at,
            });
        }
        let account = self
            .database
            .resolve_subject(&identity.subject)
            .await?
            .ok_or(ApiError::AbsentAccount)?;
        let mut tx = self.database.begin(account).await?;
        let row = sqlx::query(
            "select duet_incarnation,intelligence_control_revision from public.users where id=$1",
        )
        .bind(account)
        .fetch_one(&mut *tx)
        .await?;
        Ok(AccountContext {
            account_id: account,
            incarnation: row.get("duet_incarnation"),
            permission_revision: row.get("intelligence_control_revision"),
            authenticated_at: identity.authenticated_at,
        })
    }
    /// Explicit operator migration mapping. Caller must prove both identities; never infer from email.
    pub async fn map_front_door(
        &self,
        context: &AccountContext,
        front_door_account: Uuid,
    ) -> Result<(), ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        sqlx::query("insert into public.duet_front_door_mapping(front_door_account_id,user_id) values($1,$2) on conflict(user_id) do nothing").bind(front_door_account).bind(context.account_id).execute(&mut *tx).await?;
        let actual: Uuid = sqlx::query_scalar(
            "select front_door_account_id from public.duet_front_door_mapping where user_id=$1",
        )
        .bind(context.account_id)
        .fetch_one(&mut *tx)
        .await?;
        if actual != front_door_account {
            return Err(ApiError::Conflict("identity mapping already exists"));
        }
        tx.commit().await?;
        Ok(())
    }
}
/// Lock the account before mutations/accepted work; denies stale authentication, permissions or incarnations.
pub async fn check_current(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    context: &AccountContext,
    require_luna: bool,
) -> Result<(), ApiError> {
    let row=sqlx::query("select duet_incarnation,intelligence_control_revision,sessions_revoked_before,terms_version,luna_consent_decision,luna_consent_version,intelligence_enabled from public.users where id=$1 for update").bind(context.account_id).fetch_optional(&mut **tx).await?.ok_or(ApiError::AbsentAccount)?;
    if row.get::<Uuid, _>("duet_incarnation") != context.incarnation {
        return Err(ApiError::SessionRevoked);
    }
    if row
        .get::<Option<DateTime<Utc>>, _>("sessions_revoked_before")
        .is_some_and(|f| context.authenticated_at <= f)
    {
        return Err(ApiError::SessionRevoked);
    }
    if require_luna
        && (row.get::<i64, _>("intelligence_control_revision") != context.permission_revision
            || !row.get::<bool, _>("intelligence_enabled")
            || row.get::<Option<String>, _>("terms_version").as_deref()
                != Some(crate::session::CURRENT_TERMS_VERSION)
            || row
                .get::<Option<String>, _>("luna_consent_decision")
                .as_deref()
                != Some("accepted")
            || row
                .get::<Option<String>, _>("luna_consent_version")
                .as_deref()
                != Some("2026-09-24"))
    {
        return Err(ApiError::Forbidden);
    }
    Ok(())
}

#[cfg(test)]
mod local_tests {
    use super::*;
    use jsonwebtoken::{EncodingKey, Header, encode};

    const SECRET: &str = "local-stack-secret-of-at-least-32-bytes";

    fn token(secret: &str, claims: serde_json::Value) -> String {
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap()
    }

    fn claims(sub: &str) -> serde_json::Value {
        let now = Utc::now().timestamp();
        serde_json::json!({"iss":LOCAL_ISSUER,"aud":LOCAL_ISSUER,"sub":sub,"auth_time":now,"exp":now+300})
    }

    #[tokio::test]
    async fn verifies_a_signed_local_token() {
        let verified = LocalIdentity::new(SECRET)
            .unwrap()
            .verify(&token(SECRET, claims("patient-1")))
            .await
            .unwrap();
        assert_eq!(verified.subject, "patient-1");
    }

    #[tokio::test]
    async fn refuses_forged_expired_foreign_and_future_tokens() {
        let local = LocalIdentity::new(SECRET).unwrap();
        let now = Utc::now().timestamp();
        let mut expired = claims("p");
        expired["exp"] = (now - 600).into();
        let mut foreign = claims("p");
        foreign["aud"] = "coda-rust-stg".into();
        let mut future = claims("p");
        future["auth_time"] = (now + 600).into();
        let mut no_exp = claims("p");
        no_exp.as_object_mut().unwrap().remove("exp");
        for bearer in [
            token("another-secret-that-is-also-32-bytes-long", claims("p")),
            token(SECRET, expired),
            token(SECRET, foreign),
            token(SECRET, future),
            token(SECRET, no_exp),
            token(SECRET, claims("")),
            "not-a-token".into(),
        ] {
            assert!(matches!(
                local.verify(&bearer).await,
                Err(ApiError::Unauthorized)
            ));
        }
    }

    #[test]
    fn a_short_secret_is_refused() {
        assert!(LocalIdentity::new("short").is_err());
    }
}
