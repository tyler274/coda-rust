//! The model proxy on the host, as the container sees it: whether it
//! answers, and whether it takes the broker's `model-proxy` key.
//!
//! `coda-sandbox proxy-check` runs in the container and prints one
//! [`ProxyCheck`] as JSON; `doctor` on the host runs it over SSH, so the
//! check crosses the same firewall and uses the same key the agent CLIs do.

use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};

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
    match serde_json::from_str::<WhoAmI>(&body) {
        Ok(who) => ProxyCheck::Accepted {
            client: who.client.map(|client| client.label),
        },
        Err(error) => ProxyCheck::NotTheProxy {
            error: error.to_string(),
        },
    }
}
