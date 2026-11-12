//! Apple JS authorization uses the same raw nonce + Identity Platform exchange as native iOS.
//! Apple must register the HTTPS redirect and Services ID; no guest or raw-bearer login fallback.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/assets/apple_sign_in.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = duetAppleSignIn)]
    async fn apple_sign_in(client: &str, redirect: &str) -> Result<JsValue, JsValue>;
}

/// Why Apple's web authorization produced no credential.
#[cfg(target_arch = "wasm32")]
#[derive(Debug, thiserror::Error)]
pub enum AppleWebError {
    /// Apple's script threw or rejected, as given: the SDK is missing, the
    /// popup was closed, or the answer did not match this request.
    #[error("Apple's authorization script failed: {0:?}")]
    Script(JsValue),
    /// The script answered with something other than credential text.
    #[error("Apple's authorization answered {0:?}")]
    NotText(JsValue),
    /// The credential text was not an Apple credential.
    #[error("Apple's credential was unreadable: {0}")]
    Credential(#[source] serde_json::Error),
}

/// Launch official Apple popup for `services_id`, returning to `redirect_uri`;
/// token exchange and account verification remain in Rust.
#[cfg(target_arch = "wasm32")]
pub async fn sign_in(
    services_id: &str,
    redirect_uri: &str,
) -> Result<duet_client::identity::AppleCredential, AppleWebError> {
    let value = apple_sign_in(services_id, redirect_uri)
        .await
        .map_err(AppleWebError::Script)?;
    let text = value.as_string().ok_or(AppleWebError::NotText(value))?;
    serde_json::from_str(&text).map_err(AppleWebError::Credential)
}
