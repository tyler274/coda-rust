//! Apple JS authorization uses the same raw nonce + Identity Platform exchange as native iOS.
//! Apple must register the HTTPS redirect and Services ID; no guest or raw-bearer login fallback.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = r#"
export async function duetAppleSignIn(clientId, redirectURI) {
  if (!clientId || !redirectURI || new URL(redirectURI).protocol !== 'https:') throw new Error('configuration');
  if (!globalThis.AppleID?.auth) throw new Error('Load the official Apple authorization SDK before enabling sign in');
  const random = crypto.getRandomValues(new Uint8Array(32));
  const nonce = Array.from(random, b => b.toString(16).padStart(2,'0')).join('');
  const hash = new Uint8Array(await crypto.subtle.digest('SHA-256', new TextEncoder().encode(nonce)));
  const state = Array.from(crypto.getRandomValues(new Uint8Array(32)), b => b.toString(16).padStart(2,'0')).join('');
  AppleID.auth.init({clientId, redirectURI, scope:'name email', state, nonce:Array.from(hash,b=>b.toString(16).padStart(2,'0')).join(''), usePopup:true});
  const response = await AppleID.auth.signIn();
  if (response.authorization?.state !== state || !response.authorization?.id_token) throw new Error('authorization');
  return JSON.stringify({identity_token:response.authorization.id_token, nonce});
}
"#)]
extern "C" {
    #[wasm_bindgen(catch, js_name = duetAppleSignIn)]
    async fn apple_sign_in(client: &str, redirect: &str) -> Result<JsValue, JsValue>;
}
/// Launch official Apple popup; token exchange and account verification remain in Rust.
#[cfg(target_arch = "wasm32")]
pub async fn sign_in() -> Result<duet_client::identity::AppleCredential, String> {
    let value = apple_sign_in(option_env!("DUET_APPLE_SERVICES_ID").unwrap_or_default(), option_env!("DUET_APPLE_REDIRECT_URI").unwrap_or_default()).await.map_err(|_| "Apple web sign-in requires a registered Services ID, HTTPS return URL and the official Apple JS SDK; sign-in was not completed".to_owned())?;
    serde_json::from_str(&value.as_string().ok_or("Invalid Apple response")?)
        .map_err(|_| "Invalid Apple response".into())
}
