use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::get;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use reqwest::{Client, Url};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use uuid::Uuid;

use super::{Error, ISSUER, RESOURCE, client, directory, exchange, lock, save, saved};

const DYNAMIC: &str = "dynamic_agent_client";

#[derive(Deserialize)]
struct Callback {
    state: String,
    client_id: Option<String>,
    code: Option<String>,
    error: Option<String>,
}

impl Callback {
    fn authorization(self, previous: Option<&str>) -> Result<(String, String), Error> {
        if self.error.is_some() {
            return Err(Error::Refused("ChatGPT sign-in was declined or failed"));
        }
        let id = match (previous, self.client_id) {
            (Some(old), Some(new)) if old != new => {
                return Err(Error::Refused("ChatGPT registration changed"));
            }
            (Some(old), _) => old.to_owned(),
            (None, Some(new)) if !new.is_empty() && new != DYNAMIC => new,
            _ => return Err(Error::Refused("ChatGPT did not issue a client ID")),
        };
        let code = self
            .code
            .filter(|s| !s.is_empty())
            .ok_or(Error::Refused("missing authorization code"))?;
        Ok((id, code))
    }
}

#[derive(Deserialize)]
struct Claims {
    sub: String,
    nonce: String,
}

async fn subject(client: &Client, token: &str, id: &str, nonce: &str) -> Result<String, Error> {
    let header = decode_header(token)?;
    let kid = header
        .kid
        .ok_or(Error::Refused("unsigned ChatGPT identity"))?;
    let keys: JwkSet = client
        .get("https://auth.openai.com/.well-known/jwks.json")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let key = keys
        .find(&kid)
        .ok_or(Error::Refused("unknown ChatGPT identity signer"))?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[id]);
    validation.set_issuer(&[ISSUER]);
    validation.set_required_spec_claims(&["exp", "aud", "iss", "sub"]);
    let claims = decode::<Claims>(token, &DecodingKey::from_jwk(key)?, &validation)?.claims;
    if claims.nonce != nonce || claims.sub.is_empty() {
        return Err(Error::Refused(
            "ChatGPT identity does not match the pending sign-in",
        ));
    }
    Ok(claims.sub)
}

/// Starts a local loopback callback and prints a browser link. Only the
/// developer can consent. Tokens stay in the local grant file.
pub async fn sign_in() -> Result<(), Error> {
    use std::os::unix::fs::DirBuilderExt;
    let dir = directory()?;
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&dir)?;
    let _guard = lock(&dir).await?;
    let previous = saved(&dir)?;
    let host_file = dir.join("host-id");
    let host = match fs::read_to_string(&host_file) {
        Ok(host) => host,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            let host = Uuid::new_v4().urn().to_string();
            harness::transport::write_atomic(&host_file, host.as_bytes())
                .map_err(std::io::Error::other)?;
            host
        }
        Err(err) => return Err(err.into()),
    };
    let listener = TcpListener::bind(("127.0.0.1", 0)).await?;
    let redirect = format!("http://{}/auth/callback", listener.local_addr()?);
    let verifier = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let state = Uuid::new_v4().to_string();
    let nonce = Uuid::new_v4().to_string();
    let mut url = Url::parse("https://auth.openai.com/api/accounts/authorize")
        .expect("fixed authorization URL");
    let previous_id = previous.as_ref().map(|g| g.client_id.as_str());
    url.query_pairs_mut().extend_pairs([
        ("client_id", previous_id.unwrap_or(DYNAMIC)),
        ("ext_agent_host_id", &host),
        ("response_type", "code"),
        ("redirect_uri", &redirect),
        ("resource", RESOURCE),
        (
            "scope",
            "openid profile email offline_access resource.invoke chatgpt.tokens.use.direct",
        ),
        ("state", &state),
        ("nonce", &nonce),
        ("code_challenge", &challenge),
        ("code_challenge_method", "S256"),
    ]);
    if previous.is_none() {
        url.query_pairs_mut()
            .append_pair("agent_name_hint", "Chords");
    }
    let (out, pending) = oneshot::channel();
    let out = Arc::new(Mutex::new(Some(out)));
    let app = axum::Router::new().route(
        "/auth/callback",
        get(move |Query(callback): Query<Callback>| {
            let out = Arc::clone(&out);
            let matches = callback.state == state;
            async move {
                if !matches {
                    return (
                        StatusCode::BAD_REQUEST,
                        "This callback does not match the pending sign-in.",
                    );
                }
                if let Some(tx) = out.lock().expect("local callback").take() {
                    let _ = tx.send(callback);
                }
                (
                    StatusCode::OK,
                    "Return to your terminal for the Chords sign-in result.",
                )
            }
        }),
    );
    let (stop, stopped) = oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
    });
    println!("Local browser callback: {redirect}");
    println!("Continue with ChatGPT to authorize local Chords:\n{url}");
    let result = tokio::time::timeout(Duration::from_secs(300), pending).await;
    drop(stop);
    server.await??;
    let callback = result
        .map_err(|_| Error::Refused("ChatGPT sign-in expired"))?
        .map_err(|_| Error::Refused("ChatGPT callback stopped"))?;
    let (id, code) = callback.authorization(previous_id)?;
    let http = client()?;
    let tokens = exchange(
        &http,
        &[
            ("grant_type", "authorization_code"),
            ("client_id", &id),
            ("code", &code),
            ("code_verifier", &verifier),
            ("redirect_uri", &redirect),
            ("resource", RESOURCE),
        ],
    )
    .await?;
    let identity = subject(
        &http,
        tokens
            .id_token
            .as_deref()
            .ok_or(Error::Refused("missing ChatGPT identity token"))?,
        &id,
        &nonce,
    )
    .await?;
    if previous.is_some_and(|grant| grant.subject != identity) {
        return Err(Error::Refused(
            "signed-in account does not own the saved Chords registration",
        ));
    }
    save(&dir, &tokens.grant(id, identity, String::new())?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_the_issued_or_original_registration_can_exchange_a_code() {
        let callback = |client_id: Option<&str>| Callback {
            state: "matched".into(),
            client_id: client_id.map(str::to_owned),
            code: Some("one-use-code".into()),
            error: None,
        };
        assert!(callback(None).authorization(None).is_err());
        assert!(callback(Some(DYNAMIC)).authorization(None).is_err());
        assert!(
            callback(Some("other"))
                .authorization(Some("issued"))
                .is_err()
        );
        for result in [
            callback(Some("issued")).authorization(None),
            callback(None).authorization(Some("issued")),
        ] {
            assert_eq!(result.unwrap(), ("issued".into(), "one-use-code".into()));
        }
        let mut denied = callback(Some("issued"));
        denied.error = Some("access_denied".into());
        assert!(denied.authorization(None).is_err());
    }
}
