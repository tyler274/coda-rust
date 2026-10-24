//! Physician sign-in and invitation mail.
//!
//! The Gmail transport lives in [`gmail`]. This module chooses proof or hosted
//! delivery and writes the two messages. `PHYSICIAN_GMAIL_FROM` is the
//! Workspace sender (`signin@codahealth.ai` in hosted environments). Doppler
//! holds that address and either a refresh token or a service-account key.
//! Nothing here is written into Secret Manager.

use gmail::{Endpoints, Sender};

/// How sign-in and invitation secrets leave the process.
pub(crate) enum Delivery {
    /// No Gmail sender is configured. Callers keep the current JSON token.
    Proof,
    /// `PHYSICIAN_GMAIL_FROM` is set. Callers send mail and omit the token.
    Gmail(Box<Outbound>),
}

/// Hosted mail: one [`Sender`](gmail::Sender) plus the link origin.
pub(crate) struct Outbound {
    origin: String,
    sender: Sender,
}

impl Outbound {
    pub(crate) fn new(origin: String, sender: Sender) -> Self {
        Self { origin, sender }
    }

    /// Mails the single-use sign-in link. The token is only in the message.
    pub(crate) async fn send_sign_in(&self, to: &str, token: &str) -> Result<(), String> {
        let link = self.link("/sign-in", "code", token)?;
        let hours = physician_auth::SIGN_IN_TOKEN_TTL_SECONDS / 3_600;
        let body = format!(
            "Sign in to Coda with this single-use link. It expires in {hours} hours.\r\n\r\n{link}\r\n"
        );
        self.sender
            .send(to, "Your Coda sign-in link", &body)
            .await
            .map_err(|error| error.to_string())
    }

    /// Mails the single-use front-door redeem link. The token is only in the message.
    pub(crate) async fn send_invite(&self, to: &str, token: &str) -> Result<(), String> {
        let link = self.link("/invite", "token", token)?;
        let days = front_door::INVITATION_TTL_SECONDS / 86_400;
        let body = format!(
            "Your practice invited you to Coda. This single-use link expires in {days} days.\r\n\r\n{link}\r\n"
        );
        self.sender
            .send(to, "Your Coda invitation", &body)
            .await
            .map_err(|error| error.to_string())
    }

    fn link(&self, path: &str, key: &str, token: &str) -> Result<String, String> {
        let origin = self.origin.trim_end_matches('/');
        if !(origin.starts_with("https://") || origin.starts_with("http://"))
            || !header_value_ok(origin)
        {
            return Err("gmail link origin is not configured".into());
        }
        Ok(format!("{origin}{path}?{key}={}", query_encode(token)))
    }
}

impl Delivery {
    /// Proof mode unless `PHYSICIAN_GMAIL_FROM` is non-empty.
    ///
    /// Hosted mode also reads:
    /// - `PHYSICIAN_PUBLIC_ORIGIN` — link origin (the crate has no public origin)
    /// - `PHYSICIAN_GMAIL_CLIENT_ID`, `PHYSICIAN_GMAIL_CLIENT_SECRET`, and
    ///   `PHYSICIAN_GMAIL_REFRESH_TOKEN` — a Workspace user's refresh token
    /// - `PHYSICIAN_GMAIL_SERVICE_ACCOUNT_KEY` — JSON key, used only when the
    ///   refresh triple is incomplete
    /// - `PHYSICIAN_GMAIL_SEND_URL` and `PHYSICIAN_GMAIL_TOKEN_URL` — optional
    ///   endpoint overrides for a local stand-in of the Gmail HTTP API
    ///
    /// A set sender with missing credentials stays hosted: a send fails and
    /// the token is not returned.
    pub(crate) fn from_env(get: impl Fn(&str) -> Option<String>) -> Self {
        let Some(from) = nonempty(get("PHYSICIAN_GMAIL_FROM")) else {
            return Self::Proof;
        };
        let origin = nonempty(get("PHYSICIAN_PUBLIC_ORIGIN")).unwrap_or_default();
        let endpoints = Endpoints {
            send: nonempty(get("PHYSICIAN_GMAIL_SEND_URL"))
                .unwrap_or_else(|| Endpoints::google().send),
            token: nonempty(get("PHYSICIAN_GMAIL_TOKEN_URL"))
                .unwrap_or_else(|| Endpoints::google().token),
        };
        let sender = if let (Some(client_id), Some(client_secret), Some(refresh_token)) = (
            nonempty(get("PHYSICIAN_GMAIL_CLIENT_ID")),
            nonempty(get("PHYSICIAN_GMAIL_CLIENT_SECRET")),
            nonempty(get("PHYSICIAN_GMAIL_REFRESH_TOKEN")),
        ) {
            Sender::refresh(from, endpoints, client_id, client_secret, refresh_token)
        } else if let Some(raw) = nonempty(get("PHYSICIAN_GMAIL_SERVICE_ACCOUNT_KEY")) {
            Sender::service_account(from, endpoints, &raw)
        } else {
            Sender::refresh(from, endpoints, "", "", "")
        };
        Self::Gmail(Box::new(Outbound::new(origin, sender)))
    }
}

fn nonempty(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim().to_owned();
        (!value.is_empty()).then_some(value)
    })
}

fn header_value_ok(value: &str) -> bool {
    !value.is_empty() && !value.bytes().any(|byte| matches!(byte, b'\r' | b'\n' | 0))
}

fn query_encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(key: &str) -> Option<String> {
        match key {
            "PHYSICIAN_GMAIL_FROM" => Some("signin@codahealth.ai".into()),
            "PHYSICIAN_PUBLIC_ORIGIN" => Some("https://physician.example".into()),
            "PHYSICIAN_GMAIL_REFRESH_TOKEN" => Some("refresh".into()),
            "PHYSICIAN_GMAIL_CLIENT_ID" => Some("client".into()),
            "PHYSICIAN_GMAIL_CLIENT_SECRET" => Some("secret".into()),
            _ => None,
        }
    }

    #[test]
    fn absent_sender_is_proof_mode() {
        assert!(matches!(Delivery::from_env(|_| None), Delivery::Proof));
        assert!(matches!(
            Delivery::from_env(
                |key| (key == "PHYSICIAN_GMAIL_REFRESH_TOKEN").then(|| "refresh".into())
            ),
            Delivery::Proof
        ));
    }

    #[test]
    fn a_sender_address_selects_hosted_mode() {
        assert!(matches!(Delivery::from_env(env), Delivery::Gmail(_)));
        assert!(matches!(
            Delivery::from_env(|key| match key {
                "PHYSICIAN_GMAIL_FROM" => Some("  signin@codahealth.ai  ".into()),
                _ => None,
            }),
            Delivery::Gmail(_)
        ));
    }
}
