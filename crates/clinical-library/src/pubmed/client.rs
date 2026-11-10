//! NCBI's E-utilities, within NCBI's rate limit.
//!
//! NCBI allows 10 requests a second with an API key and 3 without, across
//! every E-utilities call a key or address makes. Requests overlap, because
//! each takes over a second while the limit allows one every 100 ms, but one
//! limiter shared by every request in flight spaces their starts to that
//! rate, and backs off when NCBI says it is throttling or
//! fails on its side, honouring `Retry-After` when NCBI sends it. A request
//! NCBI refuses as malformed or unauthorised is never retried: it would fail
//! the same way.

use std::time::Duration;

use async_trait::async_trait;
use reqwest::{StatusCode, header};
use tokio::sync::Mutex;
use tokio::time::Instant;

use super::record::Pmid;
use crate::failure::FailureCode;

/// NCBI's E-utilities address.
pub const EUTILS: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";

/// How many PMIDs one EFetch request asks for. NCBI recommends at most 200
/// IDs in one request.
pub const BATCH: usize = 200;

/// The name NCBI sees as the calling tool.
pub const TOOL: &str = "coda-clinical-library";

/// The largest response the client reads.
const MAX_BODY_BYTES: usize = super::efetch::MAX_RESPONSE_BYTES;

/// Where status records come from. The refresh asks through this, so a test
/// can stand in for NCBI.
#[async_trait]
pub trait StatusSource: Send + Sync {
    /// NLM's EFetch response for `pmids`, as sent.
    async fn efetch(&self, pmids: &[Pmid]) -> Result<Vec<u8>, FetchError>;

    /// How many requests may be in flight at once. The source's own pacing
    /// still spaces their starts.
    fn in_flight(&self) -> usize;
}

/// Why NCBI gave no usable response.
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    /// NCBI kept throttling the requests after every retry.
    #[error("NCBI kept throttling the requests")]
    Throttled,
    /// NCBI kept failing on its side after every retry.
    #[error("NCBI kept answering {0}")]
    Server(StatusCode),
    /// NCBI couldn't be reached, or its answer couldn't be read, after every
    /// retry.
    #[error("NCBI couldn't be reached")]
    Unreachable(#[source] reqwest::Error),
    /// NCBI's answer was larger than the client reads.
    #[error("NCBI's answer was larger than {MAX_BODY_BYTES} bytes")]
    TooLarge,
    /// NCBI refused the request itself, such as an invalid API key. Retrying
    /// would fail the same way, so the whole night fails. The body never
    /// holds the API key or contact address.
    #[error("NCBI refused the request with {status}: {body}")]
    Refused {
        /// NCBI's status.
        status: StatusCode,
        /// The start of NCBI's answer, which says why.
        body: String,
    },
}

impl FetchError {
    /// Whether this failure may not repeat, so the articles it touched wait
    /// to retry. A refusal is the night's own fault.
    pub const fn is_outside(&self) -> bool {
        !matches!(self, Self::Refused { .. })
    }

    /// The failure's stable code.
    pub fn code(&self) -> FailureCode {
        let code = match self {
            Self::Throttled => "ncbi_throttled",
            Self::Server(_) => "ncbi_server_error",
            Self::Unreachable(_) => "ncbi_unreachable",
            Self::TooLarge => "ncbi_response_too_large",
            Self::Refused { .. } => "ncbi_refused_request",
        };
        FailureCode::known(code)
    }
}

/// How the client paces and retries.
#[derive(Clone, Debug)]
pub struct Pacing {
    /// The least time between the starts of two requests.
    pub interval: Duration,
    /// How many times one request is tried in all.
    pub attempts: u32,
    /// The wait before the first retry; each later retry waits twice as
    /// long.
    pub backoff: Duration,
    /// The longest any one wait may be, including one NCBI asks for.
    pub longest_wait: Duration,
    /// How long one request may take.
    pub timeout: Duration,
    /// How many requests may be in flight at once: enough to keep the rate
    /// limit busy while each request waits on NCBI.
    pub in_flight: usize,
}

impl Pacing {
    /// NCBI's limit for a caller: 10 requests a second with an API key, 3
    /// without.
    pub fn ncbi(with_key: bool) -> Self {
        Self {
            interval: if with_key {
                Duration::from_millis(100)
            } else {
                Duration::from_millis(334)
            },
            attempts: 5,
            backoff: Duration::from_secs(1),
            longest_wait: Duration::from_secs(60),
            timeout: Duration::from_secs(60),
            // At about 1.3 s a request, the limit keeps about 13 busy with a
            // key and 4 without; twice that leaves room for slow answers.
            in_flight: if with_key { 32 } else { 8 },
        }
    }
}

/// A client for NCBI's EFetch.
#[derive(Debug)]
pub struct Eutils {
    http: reqwest::Client,
    base: reqwest::Url,
    email: String,
    api_key: Option<String>,
    pacing: Pacing,
    next_start: Mutex<Instant>,
}

/// A client that couldn't be built.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// The base address isn't a URL.
    #[error("the E-utilities address isn't a URL")]
    Base(#[from] url::ParseError),
    /// The HTTP client couldn't be built.
    #[error("the HTTP client couldn't be built")]
    Http(#[from] reqwest::Error),
    /// NCBI asks every caller for a contact address.
    #[error("NCBI needs a contact email address")]
    NoEmail,
}

impl Eutils {
    /// A client for `base` (normally [`EUTILS`]), identifying itself with
    /// `email` and, if given, `api_key`, paced by `pacing`.
    pub fn new(
        base: &str,
        email: &str,
        api_key: Option<&str>,
        pacing: Pacing,
    ) -> Result<Self, ClientError> {
        if email.trim().is_empty() {
            return Err(ClientError::NoEmail);
        }
        let http = reqwest::Client::builder()
            .timeout(pacing.timeout)
            .user_agent(TOOL)
            .build()?;
        Ok(Self {
            http,
            base: reqwest::Url::parse(base)?,
            email: email.trim().to_owned(),
            api_key: api_key
                .map(str::trim)
                .filter(|key| !key.is_empty())
                .map(str::to_owned),
            pacing,
            next_start: Mutex::new(Instant::now()),
        })
    }

    /// The start of NCBI's answer, with the API key and contact address
    /// taken out: NCBI echoes a rejected key back, and the answer is shown to
    /// people and kept in logs.
    fn redacted(&self, body: &[u8]) -> String {
        let mut text = String::from_utf8_lossy(body).into_owned();
        if let Some(key) = &self.api_key {
            text = text.replace(key.as_str(), "[api key]");
        }
        text = text.replace(self.email.as_str(), "[contact]");
        text.chars().take(200).collect()
    }

    /// Waits until the next request may start, and reserves its turn. Every
    /// request in flight takes its turn here, so their starts keep to the
    /// pacing however many overlap.
    ///
    /// The next turn is counted from when this one actually starts, not when
    /// it was due: a request woken late, say behind the night's writes, still
    /// leaves a full interval before the next.
    async fn turn(&self) {
        // Held across the wait, so requests take their turns in order.
        let mut next = self.next_start.lock().await;
        tokio::time::sleep_until(*next).await;
        *next = Instant::now() + self.pacing.interval;
    }

    /// Sends one request, paced.
    async fn send(&self, pmids: &[Pmid]) -> Result<Reply, reqwest::Error> {
        self.turn().await;
        let ids = pmids
            .iter()
            .map(Pmid::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let mut form = vec![
            ("db", "pubmed"),
            ("retmode", "xml"),
            ("tool", TOOL),
            ("email", self.email.as_str()),
            ("id", ids.as_str()),
        ];
        if let Some(key) = &self.api_key {
            form.push(("api_key", key.as_str()));
        }
        let url = self
            .base
            .join("efetch.fcgi")
            .unwrap_or_else(|_| self.base.clone());
        let mut response = self.http.post(url).form(&form).send().await?;
        let status = response.status();
        let retry_after = response
            .headers()
            .get(header::RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.trim().parse::<u64>().ok())
            .map(Duration::from_secs);
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            if body.len() + chunk.len() > MAX_BODY_BYTES {
                return Ok(Reply::TooLarge);
            }
            body.extend_from_slice(&chunk);
        }
        Ok(Reply::Answered {
            status,
            retry_after,
            body,
        })
    }
}

enum Reply {
    Answered {
        status: StatusCode,
        retry_after: Option<Duration>,
        body: Vec<u8>,
    },
    TooLarge,
}

#[async_trait]
impl StatusSource for Eutils {
    fn in_flight(&self) -> usize {
        self.pacing.in_flight.max(1)
    }

    async fn efetch(&self, pmids: &[Pmid]) -> Result<Vec<u8>, FetchError> {
        let mut wait = self.pacing.backoff;
        let mut attempt = 1;
        loop {
            let (failure, asked) = match self.send(pmids).await {
                Ok(Reply::TooLarge) => return Err(FetchError::TooLarge),
                Ok(Reply::Answered { status, body, .. }) if status.is_success() => {
                    return Ok(body);
                }
                Ok(Reply::Answered {
                    status,
                    retry_after,
                    ..
                }) if status == StatusCode::TOO_MANY_REQUESTS => {
                    (FetchError::Throttled, retry_after)
                }
                Ok(Reply::Answered {
                    status,
                    retry_after,
                    ..
                }) if status.is_server_error() => (FetchError::Server(status), retry_after),
                Ok(Reply::Answered { status, body, .. }) => {
                    let body = self.redacted(&body);
                    return Err(FetchError::Refused { status, body });
                }
                Err(error) => (FetchError::Unreachable(error), None),
            };
            if attempt >= self.pacing.attempts {
                return Err(failure);
            }
            let pause = asked.unwrap_or(wait).min(self.pacing.longest_wait);
            tokio::time::sleep(pause).await;
            wait = (wait * 2).min(self.pacing.longest_wait);
            attempt += 1;
        }
    }
}
