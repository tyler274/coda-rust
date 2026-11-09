//! Spruce's REST API, read only.
//!
//! Every call is a GET. The HTTP status decides the error; Spruce's error
//! text is never read. A 401 is the credential. A 403 is only the credential
//! when the organization read refuses it too, which [`crate::changes`] checks,
//! because a 403 on one conversation can be that conversation.
//!
//! See <https://developer.sprucehealth.com/docs/integration-guide>.

use std::time::Duration;

use reqwest::Url;
use serde::Serialize;
use serde_json::Value;

use crate::payload::{
    self, Contact, Conversation, ConversationEntry, Endpoint, FileLink, ItemEntry, Organization,
    Page, Transcription,
};

/// Spruce's production API.
pub const API_BASE: &str = "https://api.sprucehealth.com/v1/";

/// Largest page this client will ask for. Spruce documents 200.
pub const MAX_PAGE_SIZE: u32 = payload::MAX_PAGE_SIZE as u32;

/// How long one request may take.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
/// Response bodies larger than this are a contract failure, not a value.
const MAX_RESPONSE_BYTES: usize = 16 * 1024 * 1024;

/// Why a Spruce read failed.
///
/// The status is the whole decision. The body of an error response is discarded.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    /// Spruce answered 401.
    #[error("spruce refused the credential")]
    AuthorizationRejected,
    /// Spruce answered 404.
    #[error("spruce has no such record")]
    NotFound,
    /// Spruce answered with another 4xx, including 403.
    #[error("spruce rejected the request ({status})")]
    Rejected {
        /// HTTP status.
        status: u16,
    },
    /// Spruce answered 429 or 5xx.
    #[error("spruce is temporarily unavailable ({status})")]
    Temporary {
        /// HTTP status.
        status: u16,
    },
    /// The connection failed, timed out, or was redirected.
    #[error("spruce did not answer")]
    Unavailable,
    /// The body was empty, not JSON, larger than the cap, or the wrong shape.
    #[error("spruce returned an unreadable body")]
    Contract,
    /// The credential, base URL, id, page size, or query combination cannot be sent.
    #[error("spruce request was invalid")]
    InvalidRequest,
    /// A file was larger than the caller's cap. Nothing past the cap was kept.
    #[error("spruce file is larger than {limit} bytes")]
    FileTooLarge {
        /// The cap that was passed.
        limit: usize,
    },
}

/// Which page to read. `start_from` and `pagination_token` cannot both be set:
/// Spruce does not document that combination, so this client refuses it.
#[derive(Debug, Clone, Copy)]
pub struct ListQuery<'a> {
    /// Entries to ask for, from 1 to [`MAX_PAGE_SIZE`].
    pub page_size: u32,
    /// Inclusive lower bound on conversation activity, or the item `startFrom`.
    pub start_from: Option<&'a str>,
    /// Token from the previous page.
    pub pagination_token: Option<&'a str>,
}

impl<'a> ListQuery<'a> {
    /// One page of `page_size` from the start of the list.
    pub fn first(page_size: u32) -> Self {
        Self {
            page_size,
            start_from: None,
            pagination_token: None,
        }
    }
}

/// Item order. Spruce documents no default, so the client always sends it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    /// Newest item first.
    NewestFirst,
    /// Oldest item first.
    OldestFirst,
}

impl Order {
    fn as_str(self) -> &'static str {
        match self {
            Self::NewestFirst => "newest_first",
            Self::OldestFirst => "oldest_first",
        }
    }
}

/// A Spruce credential and the API it reads.
///
/// The credential is sent as `Authorization: Bearer` without being inspected,
/// aside from rejecting an empty value or one that could break the header.
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base: Url,
    credential: String,
    max_bytes: usize,
}

impl Client {
    /// A client for [`API_BASE`] with this Bearer credential.
    pub fn new(credential: impl Into<String>) -> Result<Self, Error> {
        Self::with_base(credential, API_BASE)
    }

    /// A client for `base`, which must be `https`, or `http` on loopback.
    ///
    /// `base` is the version root, such as `https://api.sprucehealth.com/v1/`.
    /// A missing trailing slash is added so joins stay under that root.
    pub fn with_base(credential: impl Into<String>, base: &str) -> Result<Self, Error> {
        let credential = credential.into();
        if !credential_ok(&credential) {
            return Err(Error::InvalidRequest);
        }
        let mut base = Url::parse(base).map_err(|_| Error::InvalidRequest)?;
        if !base_ok(&base) {
            return Err(Error::InvalidRequest);
        }
        if !base.path().ends_with('/') {
            let path = format!("{}/", base.path());
            base.set_path(&path);
        }
        Ok(Self {
            http: http_client(),
            base,
            credential,
            max_bytes: MAX_RESPONSE_BYTES,
        })
    }

    /// Caps a response body. Used by tests that prove an oversized body fails.
    pub fn with_max_bytes(mut self, max_bytes: usize) -> Self {
        self.max_bytes = max_bytes;
        self
    }

    /// `GET /organization`.
    pub async fn organization(&self) -> Result<Organization, Error> {
        let body = self.get("organization", &NoQuery {}).await?;
        payload::organization(&body).ok_or(Error::Contract)
    }

    /// `GET /internalendpoints`.
    pub async fn internal_endpoints(&self) -> Result<Vec<Endpoint>, Error> {
        let body = self.get("internalendpoints", &NoQuery {}).await?;
        payload::internal_endpoints(&body).ok_or(Error::Contract)
    }

    /// `GET /conversations`, ordered by last message.
    pub async fn list_conversations(
        &self,
        query: ListQuery<'_>,
    ) -> Result<Page<ConversationEntry>, Error> {
        let query = ConversationQuery::new(query)?;
        let body = self.get("conversations", &query).await?;
        payload::conversation_page(&body).ok_or(Error::Contract)
    }

    /// `GET /conversations/{id}`.
    pub async fn get_conversation(&self, id: &str) -> Result<Conversation, Error> {
        let body = self
            .get(&format!("conversations/{}", path_id(id)?), &NoQuery {})
            .await?;
        payload::conversation_response(&body).ok_or(Error::Contract)
    }

    /// `GET /conversations/{id}/items`.
    pub async fn list_items(
        &self,
        conversation_id: &str,
        order: Order,
        query: ListQuery<'_>,
    ) -> Result<Page<ItemEntry>, Error> {
        let query = ItemQuery::new(order, query)?;
        let path = format!("conversations/{}/items", path_id(conversation_id)?);
        let body = self.get(&path, &query).await?;
        payload::item_page(&body).ok_or(Error::Contract)
    }

    /// `GET /conversationItems/{id}`.
    pub async fn get_item(&self, id: &str) -> Result<ItemEntry, Error> {
        let body = self
            .get(&format!("conversationItems/{}", path_id(id)?), &NoQuery {})
            .await?;
        payload::item_response(&body).ok_or(Error::Contract)
    }

    /// `GET /transcriptions/{id}`.
    pub async fn get_transcription(&self, id: &str) -> Result<Transcription, Error> {
        let body = self
            .get(&format!("transcriptions/{}", path_id(id)?), &NoQuery {})
            .await?;
        payload::transcription(&body).ok_or(Error::Contract)
    }

    /// `GET /contacts/{id}`.
    pub async fn get_contact(&self, id: &str) -> Result<Contact, Error> {
        let body = self
            .get(&format!("contacts/{}", path_id(id)?), &NoQuery {})
            .await?;
        payload::contact(&body).ok_or(Error::Contract)
    }

    /// The bytes behind an attachment link, at most `max_bytes` of them.
    ///
    /// The link is signed, so the Spruce credential is not sent with it. An
    /// expired link answers like any other refusal, usually 403 or 404. A file
    /// past the cap is [`Error::FileTooLarge`]; reading stops at the cap.
    pub async fn download(&self, link: &FileLink, max_bytes: usize) -> Result<Vec<u8>, Error> {
        let url = Url::parse(link.url()).map_err(|_| Error::InvalidRequest)?;
        if !base_ok(&url) {
            return Err(Error::InvalidRequest);
        }
        let response = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|_| Error::Unavailable)?;
        let status = response.status();
        if !status.is_success() {
            return Err(classify(status.as_u16()));
        }
        read_capped(response, max_bytes)
            .await?
            .ok_or(Error::FileTooLarge { limit: max_bytes })
    }

    async fn get<Q: Serialize>(&self, path: &str, query: &Q) -> Result<Value, Error> {
        let url = self.base.join(path).map_err(|_| Error::InvalidRequest)?;
        let response = self
            .http
            .get(url)
            .bearer_auth(&self.credential)
            .header(reqwest::header::ACCEPT, "application/json")
            .query(query)
            .send()
            .await
            .map_err(|_| Error::Unavailable)?;
        let status = response.status();
        if !status.is_success() {
            return Err(classify(status.as_u16()));
        }
        let body = read_capped(response, self.max_bytes)
            .await?
            .ok_or(Error::Contract)?;
        serde_json::from_slice(&body).map_err(|_| Error::Contract)
    }
}

/// The body, or `None` once it passes `max_bytes`.
async fn read_capped(
    mut response: reqwest::Response,
    max_bytes: usize,
) -> Result<Option<Vec<u8>>, Error> {
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| Error::Unavailable)? {
        if body.len().saturating_add(chunk.len()) > max_bytes {
            return Ok(None);
        }
        body.extend_from_slice(&chunk);
    }
    Ok(Some(body))
}

fn classify(status: u16) -> Error {
    match status {
        401 => Error::AuthorizationRejected,
        404 => Error::NotFound,
        429 | 500..=599 => Error::Temporary { status },
        _ => Error::Rejected { status },
    }
}

fn credential_ok(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 48 * 1024
        && !value
            .chars()
            .any(|character| matches!(character, '\r' | '\n' | '\0'))
}

fn base_ok(url: &Url) -> bool {
    match url.scheme() {
        "https" => true,
        "http" => matches!(url.host_str(), Some("127.0.0.1" | "localhost" | "[::1]")),
        _ => false,
    }
}

fn path_id(id: &str) -> Result<String, Error> {
    if !payload::valid_id(id) {
        return Err(Error::InvalidRequest);
    }
    Ok(encode_path_segment(id))
}

fn encode_path_segment(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

fn page_ok(query: ListQuery<'_>) -> Result<(), Error> {
    if !(1..=MAX_PAGE_SIZE).contains(&query.page_size) {
        return Err(Error::InvalidRequest);
    }
    if query.start_from.is_some() && query.pagination_token.is_some() {
        return Err(Error::InvalidRequest);
    }
    Ok(())
}

#[derive(Serialize)]
struct NoQuery {}

#[derive(Serialize)]
struct ConversationQuery<'a> {
    #[serde(rename = "pageSize")]
    page_size: u32,
    #[serde(rename = "orderBy")]
    order_by: &'static str,
    #[serde(rename = "startFrom", skip_serializing_if = "Option::is_none")]
    start_from: Option<&'a str>,
    #[serde(rename = "paginationToken", skip_serializing_if = "Option::is_none")]
    pagination_token: Option<&'a str>,
}

impl<'a> ConversationQuery<'a> {
    fn new(query: ListQuery<'a>) -> Result<Self, Error> {
        page_ok(query)?;
        Ok(Self {
            page_size: query.page_size,
            order_by: "last_message",
            start_from: query.start_from,
            pagination_token: query.pagination_token,
        })
    }
}

#[derive(Serialize)]
struct ItemQuery<'a> {
    #[serde(rename = "pageSize")]
    page_size: u32,
    order: &'static str,
    #[serde(rename = "startFrom", skip_serializing_if = "Option::is_none")]
    start_from: Option<&'a str>,
    #[serde(rename = "paginationToken", skip_serializing_if = "Option::is_none")]
    pagination_token: Option<&'a str>,
}

impl<'a> ItemQuery<'a> {
    fn new(order: Order, query: ListQuery<'a>) -> Result<Self, Error> {
        page_ok(query)?;
        Ok(Self {
            page_size: query.page_size,
            order: order.as_str(),
            start_from: query.start_from,
            pagination_token: query.pagination_token,
        })
    }
}

fn http_client() -> reqwest::Client {
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .no_proxy()
                .connect_timeout(Duration::from_secs(5))
                .timeout(REQUEST_TIMEOUT)
                .build()
                .expect("spruce http client")
        })
        .expect("spruce http thread")
        .join()
        .expect("spruce http client")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_credential_or_base_that_cannot_be_sent_is_refused() {
        assert!(Client::new("").is_err());
        assert!(Client::new("bad\nkey").is_err());
        assert!(Client::with_base("token", "ftp://spruce.example/v1/").is_err());
        assert!(Client::with_base("token", "http://spruce.example/v1/").is_err());
        assert!(Client::new("token").is_ok());
    }

    #[tokio::test]
    async fn start_from_and_a_page_token_are_not_sent_together() {
        let client = Client::new("token").unwrap();
        let query = ListQuery {
            page_size: 10,
            start_from: Some("2026-10-27T18:12:50Z"),
            pagination_token: Some("next"),
        };
        let error = client.list_conversations(query).await.unwrap_err();
        assert_eq!(error, Error::InvalidRequest);
    }
}
