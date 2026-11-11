//! The developer's opt-in, local ChatGPT plan grant for Chords inference.
//! Physician accounts and hosted credentials have separate owners.

mod login;

use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub use login::sign_in;

const ISSUER: &str = "https://auth.openai.com";
const TOKEN_ENDPOINT: &str = "https://auth.openai.com/api/accounts/oauth/token";
const RESOURCE: &str = "https://api.openai.com/v1";
const SIGN_IN: Error = Error::Refused(
    "Run cargo run -p local-product --bin chords-login to authorize local Chords with your ChatGPT account.",
);
const PLAN_SCOPE: &str = "chatgpt.tokens.use.direct";

/// A local credential operation failed. Token bodies never enter its display text.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Local configuration or OAuth protocol refusal.
    #[error("{0}")]
    Refused(&'static str),
    /// Credential storage or callback listener failure.
    #[error("local ChatGPT sign-in I/O failed: {0}")]
    Io(#[from] io::Error),
    /// Account provider or network failure.
    #[error("ChatGPT authorization request failed: {0}")]
    Network(#[from] reqwest::Error),
    /// Invalid saved credential.
    #[error("invalid local ChatGPT credential record")]
    Record(#[from] serde_json::Error),
    /// ID token signature or claims failure.
    #[error("OpenAI identity token validation failed")]
    Identity(#[from] jsonwebtoken::errors::Error),
    /// Time could not be established.
    #[error("could not determine credential lifetime")]
    Clock(#[from] std::time::SystemTimeError),
    /// Failure acquiring the cross-process credential lock.
    #[error("credential lock task stopped")]
    Task(#[from] tokio::task::JoinError),
}

#[derive(Deserialize, Serialize)]
struct Grant {
    client_id: String,
    subject: String,
    access_token: String,
    refresh_token: String,
    expires_at: u64,
    scope: String,
}

#[derive(Deserialize)]
struct Tokens {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: u64,
    id_token: Option<String>,
    scope: Option<String>,
}

impl Tokens {
    fn grant(self, client_id: String, subject: String, scope: String) -> Result<Grant, Error> {
        let scope = self.scope.unwrap_or(scope);
        if !scope.split_ascii_whitespace().any(|s| s == PLAN_SCOPE) {
            return Err(Error::Refused("ChatGPT plan permission was not granted"));
        }
        if !self.token_type.eq_ignore_ascii_case("Bearer")
            || self.access_token.is_empty()
            || self.refresh_token.is_empty()
            || self.expires_in == 0
        {
            return Err(Error::Refused("incomplete ChatGPT token response"));
        }
        Ok(Grant {
            client_id,
            subject,
            scope,
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            expires_at: now()?
                .checked_add(self.expires_in)
                .ok_or(Error::Refused("invalid ChatGPT token lifetime"))?,
        })
    }
}

fn now() -> Result<u64, Error> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

fn directory() -> Result<PathBuf, Error> {
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".config")))
        .ok_or(Error::Refused(
            "local Chords sign-in needs a config directory",
        ))?;
    Ok(config.join("coda/chords-chatgpt"))
}

async fn lock(dir: &Path) -> Result<File, Error> {
    let path = dir.join("grant.lock");
    Ok(tokio::task::spawn_blocking(move || -> io::Result<File> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;
        file.lock()?;
        Ok(file)
    })
    .await??)
}

fn saved(dir: &Path) -> Result<Option<Grant>, Error> {
    match fs::read(dir.join("grant.json")) {
        Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.into()),
    }
}

fn save(dir: &Path, grant: &Grant) -> Result<(), Error> {
    // Same 0600, synced atomic replacement used for the local execution store.
    harness::transport::write_atomic(&dir.join("grant.json"), &serde_json::to_vec(grant)?)
        .map_err(io::Error::other)?;
    Ok(())
}

fn client() -> Result<Client, Error> {
    Ok(Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .timeout(Duration::from_secs(30))
        .build()?)
}

async fn exchange(client: &Client, fields: &[(&str, &str)]) -> Result<Tokens, Error> {
    let response = client.post(TOKEN_ENDPOINT).form(fields).send().await?;
    if response.status() == reqwest::StatusCode::BAD_REQUEST
        || response.status() == reqwest::StatusCode::UNAUTHORIZED
    {
        return Err(Error::Refused(
            "ChatGPT rejected this authorization. Run chords-login to sign in again.",
        ));
    }
    Ok(response.error_for_status()?.json().await?)
}

/// Checks the developer explicitly signed in before a local subscription server starts.
pub fn require_sign_in() -> Result<(), Error> {
    saved(&directory()?)?.ok_or(SIGN_IN)?;
    Ok(())
}

/// Returns a short-lived access token. Refresh and replacement are serialized
/// across local processes so a rotating refresh token is used just once.
pub async fn access_token() -> Result<String, Error> {
    let dir = directory()?;
    if !dir.try_exists()? {
        return Err(SIGN_IN);
    }
    let _guard = lock(&dir).await?;
    let grant = saved(&dir)?.ok_or(SIGN_IN)?;
    if grant.expires_at > now()?.saturating_add(60) {
        return Ok(grant.access_token);
    }
    let tokens = exchange(
        &client()?,
        &[
            ("grant_type", "refresh_token"),
            ("client_id", &grant.client_id),
            ("refresh_token", &grant.refresh_token),
            ("resource", RESOURCE),
        ],
    )
    .await?;
    let refreshed = tokens.grant(grant.client_id, grant.subject, grant.scope)?;
    save(&dir, &refreshed)?;
    Ok(refreshed.access_token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saving_a_refresh_replaces_the_whole_grant_and_requires_plan_permission() {
        use std::os::unix::fs::PermissionsExt;
        let tokens = |access: &str, refresh: &str, scope: Option<&str>| Tokens {
            access_token: access.into(),
            refresh_token: refresh.into(),
            token_type: "Bearer".into(),
            expires_in: 3600,
            id_token: None,
            scope: scope.map(str::to_owned),
        };
        assert!(
            tokens("access", "refresh", Some("openid"))
                .grant("client".into(), "developer".into(), String::new())
                .is_err()
        );
        let dir = std::env::temp_dir().join(uuid::Uuid::new_v4().to_string());
        fs::create_dir(&dir).unwrap();
        let first = tokens("first", "first-refresh", Some(PLAN_SCOPE))
            .grant("client".into(), "developer".into(), String::new())
            .unwrap();
        save(&dir, &first).unwrap();
        let updated = tokens("second", "second-refresh", None)
            .grant(first.client_id, first.subject, first.scope)
            .unwrap();
        save(&dir, &updated).unwrap();
        let reloaded = saved(&dir).unwrap().unwrap();
        assert_eq!(
            (
                reloaded.access_token.as_str(),
                reloaded.refresh_token.as_str(),
                reloaded.scope.as_str()
            ),
            ("second", "second-refresh", PLAN_SCOPE)
        );
        assert_eq!(
            fs::metadata(dir.join("grant.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
        fs::remove_dir_all(dir).unwrap();
    }
}
