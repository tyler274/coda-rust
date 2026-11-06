//! Existing Identity Platform transport, separate from optional practice invitations.
use crate::DuetClient;
use serde::{Deserialize, Serialize};

/// Verified internal account context returned by `/api/auth/account/context`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountContext {
    /// Internal owner; never inferred from email.
    pub account_id: uuid::Uuid,
    /// Restored/deleted/recreated owner fence.
    pub incarnation: uuid::Uuid,
    /// Current permission fence.
    pub permission_revision: i64,
    /// Provider authentication time.
    pub authenticated_at: String,
}
impl AccountContext {
    /// Protected cache namespace includes account incarnation.
    pub fn owner(&self) -> String {
        format!("{}:{}", self.account_id, self.incarnation)
    }
}
/// Errors do not include credential-bearing URLs or response bodies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentityError {
    /// No valid credential; do not infer account deletion.
    Unauthorized,
    /// The server refused a verified session because it was revoked or the account is gone.
    /// Local content for that account is removed. An expired token is not this error.
    Ended,
    /// Provider explicitly rejected an expired/revoked/disabled refresh credential.
    Expired,
    /// Configuration, network, locked storage or service outage; keep drafts.
    Unavailable,
    /// A response could not be read; the redacted transport cause is retained.
    Response(crate::refusal::TransportCause),
}
impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unauthorized => "no valid credential",
            Self::Ended => "this session has ended",
            Self::Expired => "the credential is expired, revoked or disabled",
            Self::Unavailable | Self::Response(_) => {
                "the identity service is unavailable or not configured"
            }
        })
    }
}
impl std::error::Error for IdentityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Response(cause) => Some(cause),
            _ => None,
        }
    }
}
impl DuetClient {
    /// Establish standalone account with the existing identity DTO; no invitation required.
    pub async fn ensure_account(&self) -> Result<duet_wire::models::AccountState, IdentityError> {
        let response = self
            .configuration
            .client
            .put(format!("{}/api/auth/account", self.configuration.base_path))
            .bearer_auth(
                self.configuration
                    .bearer_access_token
                    .as_deref()
                    .unwrap_or_default(),
            )
            .json(&duet_wire::models::AccountIdentityRequest::new())
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        decode(response).await
    }
    /// Read the verified account/incarnation before rendering private routes.
    pub async fn account_context(&self) -> Result<AccountContext, IdentityError> {
        let response = self
            .configuration
            .client
            .get(format!(
                "{}/api/auth/account/context",
                self.configuration.base_path
            ))
            .bearer_auth(
                self.configuration
                    .bearer_access_token
                    .as_deref()
                    .unwrap_or_default(),
            )
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        decode(response).await
    }
}
async fn decode<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, IdentityError> {
    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|error| IdentityError::Response(error.into()))?;
        return Err(match crate::refusal::classify(status, &body) {
            Some(
                crate::refusal::SessionRefusal::Revoked | crate::refusal::SessionRefusal::Absent,
            ) => IdentityError::Ended,
            Some(crate::refusal::SessionRefusal::Invalid) => IdentityError::Unauthorized,
            None => IdentityError::Unavailable,
        });
    }
    response
        .json()
        .await
        .map_err(|_| IdentityError::Unavailable)
}
/// Short-lived access plus refresh credential. Intentionally does not implement Debug.
#[derive(Clone)]
pub struct ProviderCredential {
    /// Provider-verified subject, compared on every refresh.
    pub subject: String,
    /// ID token, memory only.
    pub id_token: String,
    /// Device-protected refresh token.
    pub refresh_token: String,
    /// Provider TTL; never guessed from an unverified JWT.
    pub expires_in: u64,
}
/// Existing public provider configuration, not a new backend login contract.
#[derive(Clone)]
pub struct IdentityPlatformClient {
    api_key: String,
    client: reqwest::Client,
    bundle_id: Option<String>,
}
impl IdentityPlatformClient {
    /// Use the existing Identity Platform project API key and restricted iOS bundle identifier.
    pub fn new(api_key: String, bundle_id: Option<String>) -> Self {
        Self {
            api_key,
            bundle_id,
            client: {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    reqwest::Client::builder()
                        .timeout(std::time::Duration::from_secs(20))
                        .build()
                        .expect("Identity HTTP client")
                }
                #[cfg(target_arch = "wasm32")]
                {
                    reqwest::Client::new()
                }
            },
        }
    }
    fn request(&self, url: &str) -> Result<reqwest::RequestBuilder, IdentityError> {
        if self.api_key.is_empty() {
            return Err(IdentityError::Unavailable);
        }
        let mut request = self.client.post(url).query(&[("key", &self.api_key)]);
        if let Some(bundle) = &self.bundle_id {
            request = request.header("X-Ios-Bundle-Identifier", bundle);
        }
        Ok(request)
    }
    /// Refresh directly with Google's existing securetoken API. Different subjects fail closed.
    pub async fn refresh(
        &self,
        subject: &str,
        refresh_token: &str,
    ) -> Result<ProviderCredential, IdentityError> {
        let form = encoded(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ]);
        let response = self
            .request("https://securetoken.googleapis.com/v1/token")?
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form)
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        #[derive(Deserialize)]
        struct Token {
            user_id: String,
            id_token: String,
            refresh_token: String,
            expires_in: String,
        }
        if response.status() == reqwest::StatusCode::BAD_REQUEST
            || response.status() == reqwest::StatusCode::UNAUTHORIZED
        {
            let body: serde_json::Value = response
                .json()
                .await
                .map_err(|_| IdentityError::Unavailable)?;
            return Err(match body["error"]["message"].as_str() {
                Some(
                    "TOKEN_EXPIRED" | "INVALID_REFRESH_TOKEN" | "USER_DISABLED" | "USER_NOT_FOUND",
                ) => IdentityError::Expired,
                _ => IdentityError::Unavailable,
            });
        }
        let token: Token = decode(response).await?;
        if token.user_id != subject || token.id_token.is_empty() || token.refresh_token.is_empty() {
            return Err(IdentityError::Unauthorized);
        }
        Ok(ProviderCredential {
            subject: token.user_id,
            id_token: token.id_token,
            refresh_token: token.refresh_token,
            expires_in: token
                .expires_in
                .parse::<u64>()
                .ok()
                .filter(|ttl| *ttl > 60 && *ttl <= 86400)
                .ok_or(IdentityError::Unavailable)?,
        })
    }
    /// Exchange the native Apple authorization using the original Identity Platform contract.
    pub async fn apple(
        &self,
        credential: AppleCredential,
        redirect_uri: &str,
    ) -> Result<ProviderCredential, IdentityError> {
        if !redirect_uri.starts_with("https://") {
            return Err(IdentityError::Unavailable);
        }
        let body = encoded(&[
            ("id_token", &credential.identity_token),
            ("providerId", "apple.com"),
            ("nonce", &credential.nonce),
        ]);
        let response = self.request("https://identitytoolkit.googleapis.com/v1/accounts:signInWithIdp")?
            .json(&serde_json::json!({"requestUri":redirect_uri,"postBody":body,"returnSecureToken":true,"returnIdpCredential":true}))
            .send().await.map_err(|_| IdentityError::Unavailable)?;
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Token {
            local_id: String,
            id_token: String,
            refresh_token: String,
            expires_in: String,
        }
        let token: Token = decode(response).await?;
        if token.local_id.is_empty() || token.id_token.is_empty() || token.refresh_token.is_empty()
        {
            return Err(IdentityError::Unauthorized);
        }
        Ok(ProviderCredential {
            subject: token.local_id,
            id_token: token.id_token,
            refresh_token: token.refresh_token,
            expires_in: token
                .expires_in
                .parse::<u64>()
                .ok()
                .filter(|ttl| *ttl > 60 && *ttl <= 86400)
                .ok_or(IdentityError::Unavailable)?,
        })
    }
}
fn encoded(pairs: &[(&str, &str)]) -> String {
    let mut url = reqwest::Url::parse("https://localhost/").unwrap();
    url.query_pairs_mut().extend_pairs(pairs.iter().copied());
    url.query().unwrap_or_default().to_owned()
}
/// Native Apple response, held only in memory. Rust passes it directly to Identity Platform.
#[derive(Deserialize)]
pub struct AppleCredential {
    /// Apple-signed identity token.
    pub identity_token: String,
    /// Cryptographic nonce passed to the native authorization.
    pub nonce: String,
}
/// Start native Sign in with Apple. Unsupported hosts fail closed rather than require invitations.
pub fn start_native_sign_in() -> Result<(), IdentityError> {
    crate::retention::platform_call(serde_json::json!({"op":"apple_start"}))
        .map(|_| ())
        .map_err(|_| IdentityError::Unavailable)
}
/// Poll the native authorization result. The UI bounds polling and never persists Apple credentials.
pub fn poll_native_sign_in() -> Result<Option<AppleCredential>, IdentityError> {
    let result = crate::retention::platform_call(serde_json::json!({"op":"apple_poll"}))
        .map_err(|_| IdentityError::Unavailable)?;
    if result.is_null() {
        return Ok(None);
    }
    serde_json::from_value(result)
        .map(Some)
        .map_err(|_| IdentityError::Unauthorized)
}
