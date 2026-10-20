//! Identity Platform authentication, separate from IAP access admission.
//!
//! Production integration must supply fresh Google signing keys and an authoritative
//! Identity Platform account/revocation lookup. Neither a decoded JWT nor an IAP
//! header is enough. No network adapter or credential issuer is implemented here.
//! Email credential issuance (the existing front-door uses one-time proofs) and
//! reauthenticated provider linking remain client/provider-owner integrations.
use async_trait::async_trait;
use front_door::{
    PostgresFrontDoorStore,
    identity::{Access, AuthorityError, Principal},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use std::collections::BTreeMap;

/// Fail-closed authentication failures; never log tokens or provider responses.
#[derive(Debug)]
pub enum IdentityError {
    /// Configuration or signing key freshness is unavailable.
    Unavailable,
    /// Invalid signed identity or disallowed provider.
    Invalid,
    /// Provider or durable account authority revoked the session.
    Revoked,
    /// Durable authority refused or could not establish the account.
    Account(AuthorityError),
}

/// Authoritative current identity state, not token claims or email lookup.
pub struct ProviderState {
    /// Explicit project whose user was fetched.
    pub project: String,
    /// Exact UID whose user was fetched.
    pub subject: String,
    /// Disabled/deleted accounts must not authenticate.
    pub disabled: bool,
    /// Provider cutoff in Unix seconds, compared against original auth_time.
    pub valid_since: i64,
}

/// Production must implement this through the configured project's authenticated
/// Identity Platform Admin API. Errors/unknown state NEVER authorize a request.
#[async_trait]
pub trait RevocationCheck: Send + Sync {
    /// Read current state for this exact project/subject, not matching email.
    async fn current(&self, project: &str, subject: &str) -> Result<ProviderState, IdentityError>;
}

/// Explicitly configured Google verification keys with a bounded freshness lease.
/// Key refresh must use Google's fixed trusted endpoint, never JWT jku/x5u URLs.
pub struct Verifier {
    project: String,
    keys: BTreeMap<String, DecodingKey>,
    keys_valid_until: i64,
}

#[derive(Clone, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
    auth_time: i64,
    email: String,
    email_verified: bool,
    firebase: Firebase,
}
#[derive(Clone, Deserialize)]
struct Firebase {
    sign_in_provider: String,
    #[serde(default)]
    tenant: Option<String>,
}

impl Verifier {
    /// Construct only from an explicit Identity Platform project and trusted keys.
    pub fn new(
        project: String,
        keys: BTreeMap<String, DecodingKey>,
        keys_valid_until: i64,
    ) -> Result<Self, IdentityError> {
        if project.is_empty()
            || project.trim() != project
            || project.contains('/')
            || keys.is_empty()
        {
            return Err(IdentityError::Unavailable);
        }
        Ok(Self {
            project,
            keys,
            keys_valid_until,
        })
    }

    /// Signature, issuer, audience, expiry, provider, and live revocation validation.
    /// The returned principal uses project/UID, never an email-based account lookup.
    pub async fn verify(
        &self,
        token: &str,
        now: i64,
        revocation: &dyn RevocationCheck,
    ) -> Result<Principal, IdentityError> {
        if now <= 0 || now >= self.keys_valid_until {
            return Err(IdentityError::Unavailable);
        }
        if token.len() > 16384 {
            return Err(IdentityError::Invalid);
        }
        let header = decode_header(token).map_err(|_| IdentityError::Invalid)?;
        if header.alg != Algorithm::RS256 {
            return Err(IdentityError::Invalid);
        }
        let key = header
            .kid
            .as_ref()
            .and_then(|kid| self.keys.get(kid))
            .ok_or(IdentityError::Invalid)?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.project]);
        validation.set_issuer(&[format!("https://securetoken.google.com/{}", self.project)]);
        validation.set_required_spec_claims(&["exp", "iat", "sub", "iss", "aud"]);
        // Use the same explicit clock for every time check, including fixtures.
        validation.validate_exp = false;
        let claims = decode::<Claims>(token, key, &validation)
            .map_err(|_| IdentityError::Invalid)?
            .claims;
        if claims.sub.is_empty()
            || claims.sub.len() > 128
            || claims.sub.trim() != claims.sub
            || claims.exp <= now
            || claims.iat > now
            || claims.iat <= 0
            || claims.auth_time <= 0
            || claims.auth_time > claims.iat
            || claims.exp <= claims.iat
            || claims.firebase.tenant.is_some()
            || !claims.email_verified
            || claims.email.trim().is_empty()
            || !matches!(
                claims.firebase.sign_in_provider.as_str(),
                "apple.com" | "google.com" | "password"
            )
        {
            return Err(IdentityError::Invalid);
        }
        // Identity Platform uses 'password' for email/password AND email-link
        // ID tokens; credential issuance policy belongs to the configured client.
        let current = revocation.current(&self.project, &claims.sub).await?;
        if current.project != self.project
            || current.subject != claims.sub
            || current.valid_since < 0
        {
            return Err(IdentityError::Unavailable);
        }
        if current.disabled || claims.auth_time < current.valid_since {
            return Err(IdentityError::Revoked);
        }
        Ok(Principal {
            project: self.project.clone(),
            subject: claims.sub,
            authenticated_at: claims.auth_time,
            email: claims.email,
        })
    }

    /// Ordinary authenticated reads never create a durable account.
    pub async fn resolve(
        &self,
        token: &str,
        now: i64,
        revocation: &dyn RevocationCheck,
        store: &PostgresFrontDoorStore,
    ) -> Result<Access, IdentityError> {
        let principal = self.verify(token, now, revocation).await?;
        store
            .identity_access(&principal)
            .await
            .map_err(IdentityError::Account)
    }

    /// The explicit provisioning endpoint is the only path that creates an owner.
    pub async fn provision(
        &self,
        token: &str,
        now: i64,
        revocation: &dyn RevocationCheck,
        store: &PostgresFrontDoorStore,
    ) -> Result<Access, IdentityError> {
        let principal = self.verify(token, now, revocation).await?;
        store
            .provision_identity(&principal, now)
            .await
            .map_err(IdentityError::Account)
    }
}
