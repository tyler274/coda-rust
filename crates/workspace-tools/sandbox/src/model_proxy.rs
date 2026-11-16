//! The model proxy on the host, as the container sees it: whether it
//! answers, and whether it takes the broker's `model-proxy` key.
//!
//! `coda-sandbox proxy-check` runs in the container and prints one
//! [`ProxyCheck`] as JSON; `doctor` on the host runs it over SSH, so the
//! check crosses the same firewall and uses the same key the agent CLIs do.

use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::broker::Secret;
use crate::broker::client::{self, Want};
use crate::broker::error::BrokerError;
use crate::broker::wire::Unavailable;
use crate::config::ModelProxy;
use crate::error::chain;

/// How long the proxy gets to answer.
const TIMEOUT: Duration = Duration::from_secs(5);

/// What the container found.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum ProxyCheck {
    /// The proxy took the key. `client` is the label it knows the key by,
    /// or `None` when it has no client keys and serves anyone.
    Accepted { client: Option<String> },
    /// The proxy answered with this status instead; 403 is a key it does
    /// not know or has revoked.
    Refused { status: u16 },
    /// No answer from the proxy: refused, filtered or timed out.
    Unreachable { error: String },
    /// Something answered 200 with a body that is not the proxy's: another
    /// server on its port, or a proxy without `GET /v1/client`.
    NotTheProxy { error: String },
    /// The broker has no key to send.
    NoKey { reason: Unavailable },
    /// The broker did not answer.
    Broker { error: String },
}

/// The proxy's `GET /v1/client` answer.
#[derive(Debug, Deserialize)]
struct WhoAmI {
    /// Required: an explicit `null` means the proxy has no client keys; a body without the
    /// field is not the proxy's answer.
    #[serde(deserialize_with = "Option::deserialize")]
    client: Option<Client>,
}

#[derive(Debug, Deserialize)]
struct Client {
    label: String,
}

pub fn check(socket: &Path, proxy: &ModelProxy) -> ProxyCheck {
    let key = match client::secret(socket, Want::ModelProxy) {
        Ok(key) => key,
        Err(BrokerError::Unavailable { reason, .. }) => return ProxyCheck::NoKey { reason },
        Err(error) => {
            return ProxyCheck::Broker {
                error: chain(&error),
            };
        }
    };
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(TIMEOUT))
        .http_status_as_error(false)
        .build()
        .into();
    let url = format!("{}/v1/client", proxy.url.trim_end_matches('/'));
    let mut response = match agent
        .get(&url)
        .header("Authorization", format!("Bearer {}", key.expose()))
        .call()
    {
        Ok(response) => response,
        Err(error) => {
            return ProxyCheck::Unreachable {
                error: chain(&error),
            };
        }
    };
    let status = response.status().as_u16();
    if status != 200 {
        return ProxyCheck::Refused { status };
    }
    let body = match response.body_mut().read_to_string() {
        Ok(body) => body,
        Err(error) => {
            return ProxyCheck::Unreachable {
                error: chain(&error),
            };
        }
    };
    read_answer(&body)
}

/// A client the proxy knows a key by; never the key.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ClientView {
    pub id: String,
    pub label: String,
}

/// `POST /clients`: the only answer that carries the key.
#[derive(Deserialize)]
struct Created {
    client: ClientView,
    key: String,
}

/// The proxy's routes on the host's loopback, where it answers its admin
/// routes to direct local callers only.
pub struct Admin {
    url: String,
    agent: ureq::Agent,
}

impl Admin {
    pub fn new(proxy: &ModelProxy) -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(TIMEOUT))
            .http_status_as_error(false)
            .build()
            .into();
        Self {
            url: proxy.admin_url.trim_end_matches('/').to_owned(),
            agent,
        }
    }

    /// Mints a client key labelled `label`.
    pub fn create(&self, label: &str) -> Result<(ClientView, Secret), BrokerError> {
        let url = format!("{}/clients", self.url);
        let body = serde_json::json!({ "label": label }).to_string();
        let request = self
            .agent
            .post(&url)
            .header("Content-Type", "application/json")
            .send(body);
        let created: Created = read_json(&url, request, 200)?;
        let key = Secret::new(&created.key).map_err(|source| BrokerError::ProxyKey {
            url: url.clone(),
            source,
        })?;
        Ok((created.client, key))
    }

    /// The client `key` belongs to; `None` when the proxy refuses it, as
    /// it does a revoked key, or serves anyone.
    pub fn client_of(&self, key: &Secret) -> Result<Option<ClientView>, BrokerError> {
        let url = format!("{}/v1/client", self.url);
        let response = self
            .agent
            .get(&url)
            .header("Authorization", format!("Bearer {}", key.expose()))
            .call();
        if let Ok(refused) = &response
            && matches!(refused.status().as_u16(), 401 | 403)
        {
            return Ok(None);
        }
        #[derive(Deserialize)]
        struct Named {
            #[serde(deserialize_with = "Option::deserialize")]
            client: Option<ClientView>,
        }
        let named: Named = read_json(&url, response, 200)?;
        Ok(named.client)
    }

    pub fn revoke(&self, id: &str) -> Result<(), BrokerError> {
        let url = format!("{}/clients/{id}/revoke", self.url);
        let response = self.agent.post(&url).send_empty();
        read_json::<serde_json::Value>(&url, response, 200).map(drop)
    }
}

/// The body of a `want` answer, read as `T`; any other status is an error
/// that carries the proxy's own message.
fn read_json<T: serde::de::DeserializeOwned>(
    url: &str,
    response: Result<ureq::http::Response<ureq::Body>, ureq::Error>,
    want: u16,
) -> Result<T, BrokerError> {
    let request_error = |source| BrokerError::ProxyRequest {
        url: url.to_owned(),
        source: Box::new(source),
    };
    let mut response = response.map_err(request_error)?;
    let status = response.status().as_u16();
    let body = response
        .body_mut()
        .read_to_string()
        .map_err(request_error)?;
    if status != want {
        return Err(BrokerError::ProxyStatus {
            url: url.to_owned(),
            status,
            body,
        });
    }
    serde_json::from_str(&body).map_err(|source| BrokerError::ProxyAnswer {
        url: url.to_owned(),
        source,
    })
}

/// What a 200 from `GET /v1/client` says.
fn read_answer(body: &str) -> ProxyCheck {
    match serde_json::from_str::<WhoAmI>(body) {
        Ok(who) => ProxyCheck::Accepted {
            client: who.client.map(|client| client.label),
        },
        Err(error) => ProxyCheck::NotTheProxy {
            error: error.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{ProxyCheck, read_answer};

    #[test]
    fn a_named_client_is_accepted() {
        assert_eq!(
            read_answer(r#"{"client":{"id":"x","label":"coda-agent"}}"#),
            ProxyCheck::Accepted {
                client: Some("coda-agent".to_owned())
            }
        );
    }

    #[test]
    fn an_explicit_null_client_is_an_open_proxy() {
        assert_eq!(
            read_answer(r#"{"client":null}"#),
            ProxyCheck::Accepted { client: None }
        );
    }

    #[test]
    fn a_body_without_client_is_not_the_proxy() {
        for body in ["{}", r#"{"status":"ok"}"#, "<html></html>"] {
            assert!(
                matches!(read_answer(body), ProxyCheck::NotTheProxy { .. }),
                "{body}"
            );
        }
    }
}
