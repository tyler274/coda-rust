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
#[derive(Debug)]
pub enum AppleWebError {
    /// Apple's script threw or rejected, as given: the SDK is missing, the
    /// popup was closed, or the answer did not match this request.
    Script(JsValue),
    /// The script answered with something other than credential text.
    NotText(JsValue),
    /// The credential text was not an Apple credential.
    Credential(serde_json::Error),
}

#[cfg(target_arch = "wasm32")]
impl std::fmt::Display for AppleWebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Script(value) => write!(f, "Apple's authorization script failed: {value:?}"),
            Self::NotText(value) => write!(f, "Apple's authorization answered {value:?}"),
            Self::Credential(error) => write!(f, "Apple's credential was unreadable: {error}"),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl std::error::Error for AppleWebError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Credential(error) => Some(error),
            Self::Script(_) | Self::NotText(_) => None,
        }
    }
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
