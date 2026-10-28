//! A plain-text or HTML message through the Gmail API or Workspace SMTP relay.
//!
//! Google Cloud does not run a transactional email product, and Cloud Run
//! cannot use the App Engine Mail API (standard environment only). Workspace's
//! SMTP relay supports unattended domain senders over STARTTLS on port 587,
//! authenticated by an administrator-approved static outbound IP.
//!
//! The Gmail API send call is `users.messages.send`:
//! `POST https://gmail.googleapis.com/gmail/v1/users/me/messages/send`
//! with `{ "raw": "<base64url RFC 2822>" }`.
//! See <https://developers.google.com/workspace/gmail/api/guides/sending>
//! and <https://developers.google.com/gmail/api/reference/rest/v1/users.messages/send>.
//!
//! Products own the message text and the links. This crate owns mail transport
//! and the Gmail API token exchange. A service-account assertion has no `sub` claim,
//! so this process does not use domain-wide delegation.
//!
//! It also owns the product-neutral intake primitives: [`push`] decodes a Gmail
//! push wake and routes it to a connected mailbox, and [`history`] reads
//! `users.history.list` from an opaque cursor that moves only on the last page.
//! A push is a wake only; it never carries a position. The durable cursor,
//! seen-message ledger and effects belong to the product's intake.
//! Book: `book/src/gcp.md`.
#![deny(missing_docs)]

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use base64::Engine;
use serde::{Deserialize, Serialize};

pub mod history;
pub mod push;
mod relay;
pub use relay::SmtpRelay;
pub mod service;

/// Gmail `users.messages.send`.
const GMAIL_SEND: &str = "https://gmail.googleapis.com/gmail/v1/users/me/messages/send";
/// Gmail `users.getProfile`.
const GMAIL_PROFILE: &str = "https://gmail.googleapis.com/gmail/v1/users/me/profile";
/// Gmail `users.history.list`.
const GMAIL_HISTORY: &str = "https://gmail.googleapis.com/gmail/v1/users/me/history";
/// Google's OAuth token endpoint. Also the `aud` claim of a service-account assertion.
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
/// Least privilege for sending one message. Not `gmail.modify` and not domain-wide.
const GMAIL_SEND_SCOPE: &str = "https://www.googleapis.com/auth/gmail.send";
/// Refresh an access token this many seconds before it expires.
const TOKEN_SKEW_SECONDS: u64 = 60;

/// Why mail delivery, configuration, or a Gmail token exchange failed.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The relay sender or TLS configuration is invalid.
    #[error("Workspace SMTP relay is not configured")]
    RelayConfiguration,
    /// A negative SMTP reply confirms that this attempt was refused.
    #[error("Workspace SMTP relay refused the message ({0})")]
    RelayRefused(String),
    /// The connection failed without a confirmed acceptance or refusal.
    #[error("Workspace SMTP relay acceptance could not be confirmed")]
    RelayDelivery,
    /// From, To, or Subject contained a character a header must not carry.
    #[error("gmail message headers were refused")]
    Headers,
    /// No refresh token and no usable service-account key.
    #[error("gmail credentials are not configured")]
    Credentials,
    /// The token or send request never got a response.
    #[error("{0}")]
    Unreachable(&'static str),
    /// Google answered with this HTTP status.
    #[error("gmail {what} failed: {status}")]
    Status {
        /// Which call failed: `send`, `token exchange`, `profile` or `history`.
        what: &'static str,
        /// HTTP status code.
        status: u16,
    },
    /// The token endpoint returned a body this crate could not read.
    #[error("gmail token exchange returned an unreadable body")]
    UnreadableToken,
    /// The token endpoint succeeded without an access token.
    #[error("gmail token exchange returned no access token")]
    EmptyToken,
    /// The service-account private key could not sign the assertion.
    #[error("gmail service-account key could not sign")]
    Key,
}

/// Where the token exchange, `messages.send` and the intake reads go.
#[derive(Debug, Clone)]
pub struct Endpoints {
    /// `users.messages.send` URL.
    pub send: String,
    /// OAuth token URL.
    pub token: String,
    /// `users.getProfile` URL, read for a first history cursor.
    pub profile: String,
    /// `users.history.list` URL.
    pub history: String,
}

impl Endpoints {
    /// Google's production Gmail and OAuth endpoints.
    pub fn google() -> Self {
        Self {
            send: GMAIL_SEND.to_owned(),
            token: TOKEN_URL.to_owned(),
            profile: GMAIL_PROFILE.to_owned(),
            history: GMAIL_HISTORY.to_owned(),
        }
    }
}

/// Posts one RFC 2822 message to the Gmail API.
pub struct Sender {
    from: String,
    endpoints: Endpoints,
    tokens: Tokens,
    http: reqwest::Client,
}

/// The OAuth token exchange and its cached access token, shared by sending
/// and by the intake reader.
pub(crate) struct Tokens {
    url: String,
    scope: &'static str,
    credentials: Credentials,
    http: reqwest::Client,
    access: std::sync::Mutex<Option<CachedAccess>>,
}

pub(crate) enum Credentials {
    Refresh(RefreshToken),
    ServiceAccount(ServiceAccountKey),
    Missing,
}

impl Credentials {
    /// Refresh-token credentials; any empty field leaves them unconfigured.
    pub(crate) fn refresh(client_id: String, client_secret: String, refresh_token: String) -> Self {
        if client_id.is_empty() || client_secret.is_empty() || refresh_token.is_empty() {
            Self::Missing
        } else {
            Self::Refresh(RefreshToken {
                client_id,
                client_secret,
                refresh_token,
            })
        }
    }
}

pub(crate) struct RefreshToken {
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

pub(crate) struct ServiceAccountKey {
    client_email: String,
    private_key: String,
}

#[derive(Deserialize)]
struct ServiceAccountJson {
    client_email: String,
    private_key: String,
}

struct CachedAccess {
    token: String,
    until: Instant,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    expires_in: Option<u64>,
}

/// Claims Google documents for a service-account assertion.
///
/// `sub` is the domain-wide-delegation claim. It is intentionally absent:
/// this process does not impersonate a Workspace user.
/// <https://developers.google.com/identity/protocols/oauth2/service-account>
#[derive(Serialize)]
struct Assertion<'a> {
    iss: &'a str,
    scope: &'static str,
    aud: &'static str,
    iat: u64,
    exp: u64,
}

impl Sender {
    fn new(from: String, endpoints: Endpoints, credentials: Credentials) -> Self {
        let http = http_client();
        Self {
            from,
            tokens: Tokens::new(
                endpoints.token.clone(),
                GMAIL_SEND_SCOPE,
                credentials,
                http.clone(),
            ),
            endpoints,
            http,
        }
    }

    /// A refresh-token sender. Empty credential fields stay unconfigured:
    /// [`Sender::send`](Self::send) then returns [`Error::Credentials`](Error::Credentials).
    pub fn refresh(
        from: impl Into<String>,
        endpoints: Endpoints,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        let credentials =
            Credentials::refresh(client_id.into(), client_secret.into(), refresh_token.into());
        Self::new(from.into(), endpoints, credentials)
    }

    /// A service-account sender. The signed assertion does not include `sub`.
    ///
    /// A key JSON this crate cannot read stays unconfigured, and
    /// [`Sender::send`](Self::send) returns [`Error::Credentials`](Error::Credentials).
    pub fn service_account(from: impl Into<String>, endpoints: Endpoints, key_json: &str) -> Self {
        let credentials = serde_json::from_str::<ServiceAccountJson>(key_json)
            .ok()
            .filter(|key| !key.client_email.is_empty() && !key.private_key.is_empty())
            .map(|key| {
                Credentials::ServiceAccount(ServiceAccountKey {
                    client_email: key.client_email,
                    private_key: key.private_key,
                })
            })
            .unwrap_or(Credentials::Missing);
        Self::new(from.into(), endpoints, credentials)
    }

    /// Mails one plain-text message. The body is the caller's.
    pub async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), Error> {
        if !header_value_ok(&self.from) || !header_value_ok(to) || !header_value_ok(subject) {
            return Err(Error::Headers);
        }
        self.send_raw(rfc2822(&self.from, to, subject, body).as_bytes())
            .await
    }

    /// Mails HTML with an equivalent plain-text alternative. Products own both bodies.
    pub async fn send_html(
        &self,
        to: &str,
        subject: &str,
        text: &str,
        html: &str,
    ) -> Result<(), Error> {
        if !header_value_ok(&self.from) {
            return Err(Error::Headers);
        }
        let message = html_message(
            self.from.parse().map_err(|_| Error::Headers)?,
            to,
            subject,
            text,
            html,
        )?;
        self.send_raw(&message.formatted()).await
    }

    async fn send_raw(&self, message: &[u8]) -> Result<(), Error> {
        let access = self.tokens.access_token().await?;
        let raw = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(message);
        let response = self
            .http
            .post(&self.endpoints.send)
            .bearer_auth(access)
            .json(&serde_json::json!({ "raw": raw }))
            .send()
            .await
            .map_err(|_| Error::Unreachable("gmail send did not reach the API"))?;
        let status = response.status();
        if status.is_success() {
            return Ok(());
        }
        Err(Error::Status {
            what: "send",
            status: status.as_u16(),
        })
    }
}

impl Tokens {
    pub(crate) fn new(
        url: String,
        scope: &'static str,
        credentials: Credentials,
        http: reqwest::Client,
    ) -> Self {
        Self {
            url,
            scope,
            credentials,
            http,
            access: std::sync::Mutex::new(None),
        }
    }

    pub(crate) async fn access_token(&self) -> Result<String, Error> {
        if let Some(token) = self.cached_access() {
            return Ok(token);
        }
        let (token, expires_in) = match &self.credentials {
            Credentials::Refresh(refresh) => self.exchange_refresh(refresh).await?,
            Credentials::ServiceAccount(key) => self.exchange_service_account(key).await?,
            Credentials::Missing => return Err(Error::Credentials),
        };
        self.store_access(&token, expires_in);
        Ok(token)
    }

    fn cached_access(&self) -> Option<String> {
        let guard = self.access.lock().expect("gmail access");
        let cached = guard.as_ref()?;
        (cached.until > Instant::now()).then(|| cached.token.clone())
    }

    fn store_access(&self, token: &str, expires_in: u64) {
        let lifetime = expires_in.saturating_sub(TOKEN_SKEW_SECONDS).max(1);
        *self.access.lock().expect("gmail access") = Some(CachedAccess {
            token: token.to_owned(),
            until: Instant::now() + Duration::from_secs(lifetime),
        });
    }

    async fn exchange_refresh(&self, refresh: &RefreshToken) -> Result<(String, u64), Error> {
        let response = self
            .http
            .post(&self.url)
            .form(&[
                ("grant_type", "refresh_token"),
                ("client_id", refresh.client_id.as_str()),
                ("client_secret", refresh.client_secret.as_str()),
                ("refresh_token", refresh.refresh_token.as_str()),
            ])
            .send()
            .await
            .map_err(|_| Error::Unreachable("gmail token exchange did not reach the API"))?;
        read_token(response).await
    }

    async fn exchange_service_account(
        &self,
        key: &ServiceAccountKey,
    ) -> Result<(String, u64), Error> {
        let assertion = sign_assertion(key, self.scope)?;
        let response = self
            .http
            .post(&self.url)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", assertion.as_str()),
            ])
            .send()
            .await
            .map_err(|_| Error::Unreachable("gmail token exchange did not reach the API"))?;
        read_token(response).await
    }
}

fn sign_assertion(key: &ServiceAccountKey, scope: &'static str) -> Result<String, Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or(0);
    let claims = Assertion {
        iss: &key.client_email,
        scope,
        aud: TOKEN_URL,
        iat: now,
        exp: now.saturating_add(3_600),
    };
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    let encoding = jsonwebtoken::EncodingKey::from_rsa_pem(key.private_key.as_bytes())
        .map_err(|_| Error::Key)?;
    jsonwebtoken::encode(&header, &claims, &encoding).map_err(|_| Error::Key)
}

async fn read_token(response: reqwest::Response) -> Result<(String, u64), Error> {
    let status = response.status();
    if !status.is_success() {
        return Err(Error::Status {
            what: "token exchange",
            status: status.as_u16(),
        });
    }
    let body: TokenResponse = response.json().await.map_err(|_| Error::UnreadableToken)?;
    if body.access_token.is_empty() {
        return Err(Error::EmptyToken);
    }
    Ok((body.access_token, body.expires_in.unwrap_or(300)))
}

fn http_client() -> reqwest::Client {
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .no_proxy()
                .connect_timeout(Duration::from_secs(5))
                .timeout(Duration::from_secs(10))
                .build()
                .expect("gmail http client")
        })
        .expect("gmail http thread")
        .join()
        .expect("gmail http client")
}

fn header_value_ok(value: &str) -> bool {
    !value.is_empty() && !value.bytes().any(|byte| matches!(byte, b'\r' | b'\n' | 0))
}

fn rfc2822(from: &str, to: &str, subject: &str, body: &str) -> String {
    format!(
        "From: {from}\r\nTo: {to}\r\nSubject: {subject}\r\nMIME-Version: 1.0\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{body}"
    )
}

fn html_message(
    from: lettre::message::Mailbox,
    to: &str,
    subject: &str,
    text: &str,
    html: &str,
) -> Result<lettre::Message, Error> {
    if !header_value_ok(to) || !header_value_ok(subject) {
        return Err(Error::Headers);
    }
    lettre::Message::builder()
        .from(from)
        .to(to.parse().map_err(|_| Error::Headers)?)
        .subject(subject)
        .multipart(lettre::message::MultiPart::alternative_plain_html(
            text.to_owned(),
            html.to_owned(),
        ))
        .map_err(|_| Error::Headers)
}

/// A local stand-in for the Gmail HTTP API.
#[cfg(any(test, feature = "test-util"))]
pub mod testing {
    use super::Endpoints;

    /// The fake server and the messages it recorded.
    pub struct FakeGmail {
        /// Origin of the fake server, without a trailing slash.
        pub base: String,
        /// JSON bodies posted to `users.messages.send`.
        pub sent: std::sync::Arc<std::sync::Mutex<Vec<serde_json::Value>>>,
        /// Form bodies posted to the token endpoint.
        pub token_bodies: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
        /// The one mailbox `users.getProfile` and `users.history.list` serve.
        pub mailbox: std::sync::Arc<std::sync::Mutex<FakeMailbox>>,
    }

    impl FakeGmail {
        /// The URLs a [`Sender`](super::Sender) or
        /// [`History`](super::history::History) should use.
        pub fn endpoints(&self) -> Endpoints {
            Endpoints {
                send: format!("{}/gmail/v1/users/me/messages/send", self.base),
                token: format!("{}/token", self.base),
                profile: format!("{}/gmail/v1/users/me/profile", self.base),
                history: format!("{}/gmail/v1/users/me/history", self.base),
            }
        }
    }

    /// A mailbox's history: one `messagesAdded` record per delivered message.
    #[derive(Debug)]
    pub struct FakeMailbox {
        /// Gmail's current history id; each delivery advances it by one.
        pub history_id: u64,
        /// `(history id, added message id)` in history order.
        pub records: Vec<(u64, String)>,
        /// A `startHistoryId` below this answers 404 (history no longer kept).
        pub oldest_start: u64,
        /// Records per `history.list` page.
        pub page_size: usize,
        /// `getProfile` and `history.list` calls answered.
        pub reads: usize,
        /// Every `startHistoryId` `history.list` was asked for, in order.
        pub starts: Vec<u64>,
        /// How long `history.list` waits before answering, so concurrent
        /// readers of one cursor overlap.
        pub read_delay: std::time::Duration,
    }

    impl FakeMailbox {
        /// A message arrives: history moves on by one record.
        pub fn deliver(&mut self, message_id: &str) {
            self.history_id += 1;
            self.records.push((self.history_id, message_id.to_owned()));
        }

        /// Gmail drops all history up to now: every stored start expires.
        pub fn expire_through_current(&mut self) {
            self.oldest_start = self.history_id + 1;
        }
    }

    impl Default for FakeMailbox {
        fn default() -> Self {
            Self {
                history_id: 1000,
                records: Vec::new(),
                oldest_start: 0,
                page_size: 100,
                reads: 0,
                starts: Vec::new(),
                read_delay: std::time::Duration::ZERO,
            }
        }
    }

    struct FakeState {
        fail_send: bool,
        sent: std::sync::Arc<std::sync::Mutex<Vec<serde_json::Value>>>,
        token_bodies: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
        mailbox: std::sync::Arc<std::sync::Mutex<FakeMailbox>>,
    }

    /// Binds a fake Gmail API. `fail_send` makes `messages.send` answer 500.
    pub async fn spawn(fail_send: bool) -> FakeGmail {
        let sent = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let token_bodies = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mailbox = std::sync::Arc::new(std::sync::Mutex::new(FakeMailbox::default()));
        let state = std::sync::Arc::new(FakeState {
            fail_send,
            sent: std::sync::Arc::clone(&sent),
            token_bodies: std::sync::Arc::clone(&token_bodies),
            mailbox: std::sync::Arc::clone(&mailbox),
        });
        let app = axum::Router::new()
            .route("/token", axum::routing::post(fake_token))
            .route(
                "/gmail/v1/users/me/messages/send",
                axum::routing::post(fake_send),
            )
            .route(
                "/gmail/v1/users/me/profile",
                axum::routing::get(fake_profile),
            )
            .route(
                "/gmail/v1/users/me/history",
                axum::routing::get(fake_history),
            )
            .with_state(state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        FakeGmail {
            base: format!("http://{address}"),
            sent,
            token_bodies,
            mailbox,
        }
    }

    fn authorized(headers: &axum::http::HeaderMap) -> bool {
        headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            == Some("Bearer gmail-access-test")
    }

    async fn fake_profile(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<FakeState>>,
        headers: axum::http::HeaderMap,
    ) -> axum::response::Response {
        use axum::response::IntoResponse;
        if !authorized(&headers) {
            return axum::http::StatusCode::UNAUTHORIZED.into_response();
        }
        let mut mailbox = state.mailbox.lock().expect("mailbox");
        mailbox.reads += 1;
        axum::Json(serde_json::json!({
            "emailAddress": "mailbox@codahealth.test",
            "historyId": mailbox.history_id.to_string(),
        }))
        .into_response()
    }

    /// `startHistoryId` is exclusive; `pageToken` is an offset this fake wrote.
    async fn fake_history(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<FakeState>>,
        headers: axum::http::HeaderMap,
        axum::extract::Query(query): axum::extract::Query<
            std::collections::HashMap<String, String>,
        >,
    ) -> axum::response::Response {
        use axum::response::IntoResponse;
        if !authorized(&headers) {
            return axum::http::StatusCode::UNAUTHORIZED.into_response();
        }
        let delay = state.mailbox.lock().expect("mailbox").read_delay;
        if !delay.is_zero() {
            tokio::time::sleep(delay).await;
        }
        let mut mailbox = state.mailbox.lock().expect("mailbox");
        mailbox.reads += 1;
        let Some(start) = query
            .get("startHistoryId")
            .and_then(|start| start.parse::<u64>().ok())
        else {
            return axum::http::StatusCode::BAD_REQUEST.into_response();
        };
        mailbox.starts.push(start);
        if start < mailbox.oldest_start {
            return axum::http::StatusCode::NOT_FOUND.into_response();
        }
        let offset = match query.get("pageToken") {
            Some(token) => match token.strip_prefix("offset-").and_then(|n| n.parse().ok()) {
                Some(offset) => offset,
                None => return axum::http::StatusCode::BAD_REQUEST.into_response(),
            },
            None => 0usize,
        };
        let after: Vec<&(u64, String)> = mailbox
            .records
            .iter()
            .filter(|(id, _)| *id > start)
            .collect();
        let end = (offset + mailbox.page_size).min(after.len());
        let page: Vec<serde_json::Value> = after
            .get(offset..end)
            .unwrap_or_default()
            .iter()
            .map(|(id, message)| {
                serde_json::json!({
                    "id": id.to_string(),
                    "messagesAdded": [{"message": {"id": message, "threadId": message}}],
                })
            })
            .collect();
        let mut body = serde_json::json!({
            "history": page,
            "historyId": mailbox.history_id.to_string(),
        });
        if end < after.len() {
            body["nextPageToken"] = serde_json::json!(format!("offset-{end}"));
        }
        axum::Json(body).into_response()
    }

    async fn fake_token(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<FakeState>>,
        body: axum::body::Bytes,
    ) -> axum::Json<serde_json::Value> {
        state
            .token_bodies
            .lock()
            .expect("token bodies")
            .push(String::from_utf8_lossy(&body).into_owned());
        axum::Json(serde_json::json!({
            "access_token": "gmail-access-test",
            "expires_in": 3600,
            "token_type": "Bearer",
        }))
    }

    async fn fake_send(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<FakeState>>,
        headers: axum::http::HeaderMap,
        body: axum::body::Bytes,
    ) -> axum::response::Response {
        use axum::response::IntoResponse;
        let authorization = headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        if authorization != "Bearer gmail-access-test" {
            return axum::http::StatusCode::UNAUTHORIZED.into_response();
        }
        if state.fail_send {
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        let value: serde_json::Value =
            serde_json::from_slice(&body).unwrap_or(serde_json::Value::Null);
        state.sent.lock().expect("sent").push(value);
        axum::Json(serde_json::json!({ "id": "msg-test" })).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;

    /// Test-only RSA key. It signs assertions against the local fake token endpoint.
    const TEST_KEY: &str = r#"
-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC52TwDzEtybsW4
3JZ9I0MzAufo+KYZMHExOXTCIyMiv3ZITEBENJhQ8LyIG9DMtGW+Du9KE1OeKY68
DL4aQTO/qD5NVp0wmo8dy+25E4CnQ+XnQnQGPMQDlh2yra+67b+/WS5cBViKwjS7
oGiOp5yrpO0CrZLIVmt6/nRTc0j+i0esPfiY3WAH6/hIzgunZXtsYpatnKC0FSnT
MSJjUZ+MpplpBtmLHtpbQhMXe3fFho5O9N2HH9AHVUuKeew2STNmfsWCQrowcqS/
YXSrsnBcp2/CCUNtDJsI1HJAmUogaOFRgmt/nZTK+602QF8eZZVvNxqarDtHrEBM
KGHpoZWDAgMBAAECggEAB7kURyztK0zMU9NIc0JmeRWZoR/I3zd/hkXwXF6GoXU6
K0tbFe2z/eETJJQE7qZfqzDTSgCMAADGf3mg4lYFTX8AH8lsPM6fb31ZXDbuqLf0
mZ8BIH7T8mybJu3hMjABDajDrJ7epncaWkjfL3AEEOOIo3NARanGftTy7PWkeeQq
En0g8xsOwjs81YMxfydTG/vDNNFA+ZkiBokTSV6yAZPTmDMnMBuBAVdPPMtyf4h/
VP+Br0CfLU4qxi3gPIQXHWq9j6pvnLy4IUX5WqC+govSD8Z04APyhVxqApusL6iH
DGZubvbLg+7/48ZmIM4R2msLokjnc4T1MTFMl2TzgQKBgQDggxZCBfsToSWvpslw
/BsyOMWfC8PkrTHYxyerHh4kPnjJF00RU9Q65OUzD9kYlLMMaH0kc8a/VWOOsVEU
xB47Yx0J4lLedDIw7Izv8I82SHz1PdLxT1zdiAk/IFvYAt62J3ryOqLweTZI1L3k
s5saYLNUmSwuTirUjcHEyyCEwQKBgQDT6ffYPctbOe/TXOzgr3Y3N/fPXUvKl5AW
LX3WBKmMUASukhoYhiYj54Jm6QUVAfVI8i0sTggXOb3+FuTllZgP/fF/sI5ys73m
EnCOYA+f0Ofs/X+2G0cpYuOWB+LeJZggguiBxHprD62bniZ2DiA0Dz0ZaMsbPdyS
XRojARGXQwKBgGMqI2OoJp0l+XJbNfCcuhp1rvwwAyd3RGQemyve0qtpmIohq52y
rDQTNBVYs6hCzc72PAgkxy1tttqpAUot0bijb4kVGUs+gYjQgvPcCjWM0Py1gP2q
8za6RnNGhKqlod9NEuX49k1HDK4goXtpxWFiQvNj8HrHde2Ka4LTwZpBAoGBAMuk
hEMupvRkyb4hB5Wnp//Kbf69Fi961MErOEjptBEdeFJWHnPAvng0atEATpzwg8l0
yMpRtl6zY4wAbjjeOAHuNBxUC5/YcL3erIF1eK5I8AeDWC4lKo5v9Grh6zc1g2qC
wnKp4EZwC5KHOUirrHC60ClR1hS4rvWMOKi4xzHHAoGAKtT8WxQpzEmbCOXsZana
2chM9Cwb/PDUEY/TBmQmuU8s62Zdmn3MQRVORH9C8XTTXF8lFCfGUeBxnKbpDOms
9ab9hejyMr5BEuFKOXPKqTDeaw1FsauEGmQFk88+90Zgq/cOhJNjfEMc/3FB10oz
Fo214BMEdSosIRwAyQKuEzM=
-----END PRIVATE KEY-----
"#;

    #[tokio::test]
    async fn a_service_account_assertion_has_no_domain_wide_subject() {
        let fake = testing::spawn(false).await;
        let key = serde_json::json!({
            "client_email": "sender@project.iam.gserviceaccount.com",
            "private_key": TEST_KEY.trim(),
        })
        .to_string();
        let sender = Sender::service_account("signin@codahealth.ai", fake.endpoints(), &key);
        sender
            .send(
                "physician@codahealth.test",
                "A note",
                "The body is the caller's.\r\n",
            )
            .await
            .expect("send");
        let bodies = fake.token_bodies.lock().expect("tokens").clone();
        let body = bodies.first().expect("token exchange");
        let assertion = form_value(body, "assertion");
        assert_eq!(
            form_value(body, "grant_type"),
            "urn:ietf:params:oauth:grant-type:jwt-bearer"
        );
        let payload = assertion.split('.').nth(1).expect("payload");
        let claims: serde_json::Value = serde_json::from_slice(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(payload)
                .expect("payload bytes"),
        )
        .expect("claims");
        assert!(
            claims.get("sub").is_none(),
            "assertion impersonated a user: {claims}"
        );
        assert_eq!(claims["scope"], GMAIL_SEND_SCOPE);
        assert_eq!(claims["aud"], TOKEN_URL);
        assert_eq!(claims["iss"], "sender@project.iam.gserviceaccount.com");
        let message = decode_raw(&fake.sent.lock().expect("sent")[0]);
        assert!(message.contains("From: signin@codahealth.ai"), "{message}");
        assert!(
            message.contains("To: physician@codahealth.test"),
            "{message}"
        );
        assert!(message.contains("Subject: A note"), "{message}");
        assert!(message.contains("The body is the caller's."), "{message}");
    }

    #[tokio::test]
    async fn a_failed_send_returns_the_status() {
        let fake = testing::spawn(true).await;
        let sender = Sender::refresh(
            "signin@codahealth.ai",
            fake.endpoints(),
            "client",
            "secret",
            "refresh",
        );
        let error = sender.send("to@example.test", "Hi", "body\r\n").await;
        assert!(matches!(
            error,
            Err(Error::Status {
                what: "send",
                status: 500
            })
        ));
    }

    fn form_value(body: &str, key: &str) -> String {
        body.split('&')
            .find_map(|pair| {
                let (name, value) = pair.split_once('=')?;
                (name == key).then(|| url_decode(value))
            })
            .unwrap_or_else(|| panic!("missing {key}"))
    }

    fn url_decode(value: &str) -> String {
        let bytes = value.as_bytes();
        let mut out = Vec::new();
        let mut index = 0;
        while index < bytes.len() {
            match bytes[index] {
                b'+' => out.push(b' '),
                b'%' if index + 2 < bytes.len() => {
                    let hex = std::str::from_utf8(&bytes[index + 1..index + 3]).unwrap_or("00");
                    out.push(u8::from_str_radix(hex, 16).unwrap_or(0));
                    index += 2;
                }
                byte => out.push(byte),
            }
            index += 1;
        }
        String::from_utf8(out).unwrap_or_default()
    }

    fn decode_raw(sent: &serde_json::Value) -> String {
        let raw = sent["raw"].as_str().expect("raw");
        assert!(!raw.contains('='), "raw was not unpadded base64url: {raw}");
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(raw)
            .expect("raw bytes");
        String::from_utf8(bytes).expect("message")
    }
}
