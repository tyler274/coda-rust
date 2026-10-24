//! Physician sign-in and invitation mail.
//!
//! This module owns the product messages and their return links. The shared
//! `gmail` crate owns delivery through the Workspace relay or Gmail API.

use gmail::{Endpoints, Sender, SmtpRelay};

/// How sign-in and invitation secrets leave the process.
pub(crate) enum Delivery {
    /// No mail transport is configured. Callers keep the current JSON token.
    Proof,
    /// A mail transport is configured. Callers send mail and omit the token.
    Hosted(Box<Outbound>),
}

/// Hosted mail: one delivery transport plus the link origin.
pub(crate) struct Outbound {
    origin: String,
    sender: Transport,
}

enum Transport {
    Gmail(Sender),
    Relay(SmtpRelay),
    Unavailable,
}

impl Transport {
    async fn send_html(
        &self,
        to: &str,
        subject: &str,
        text: &str,
        html: &str,
    ) -> Result<(), String> {
        match self {
            Self::Gmail(sender) => sender.send_html(to, subject, text, html).await,
            Self::Relay(sender) => sender.send_html(to, subject, text, html).await,
            Self::Unavailable => return Err("mail transport is not configured".into()),
        }
        .map_err(|e| e.to_string())
    }

    async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        match self {
            Self::Gmail(sender) => sender
                .send(to, subject, body)
                .await
                .map_err(|e| e.to_string()),
            Self::Relay(sender) => sender
                .send(to, subject, body)
                .await
                .map_err(|e| e.to_string()),
            Self::Unavailable => Err("mail transport is not configured".into()),
        }
    }
}

impl Outbound {
    pub(crate) fn new(origin: String, sender: Sender) -> Self {
        Self {
            origin,
            sender: Transport::Gmail(sender),
        }
    }

    /// Mails the single-use sign-in link. The token is only in the message.
    pub(crate) async fn send_sign_in(&self, to: &str, token: &str) -> Result<(), String> {
        let link = format!(
            "{}&email={}",
            self.link("/", "code", token)?,
            query_encode(to)
        );
        let hours = physician_auth::SIGN_IN_TOKEN_TTL_SECONDS / 3_600;
        let detail = format!("This link expires in {hours} hours and can be used once.");
        let text = format!(
            "Sign in to Coda\r\n\r\n{link}\r\n\r\n{detail}\r\nIf you didn't request this, you can ignore this email.\r\n"
        );
        let html = include_str!("sign-in-email.html")
            .replace("{{link}}", &html_escape(&link))
            .replace("{{detail}}", &detail);
        self.sender
            .send_html(to, "Sign in to Coda", &text, &html)
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
            return Err("mail link origin is not configured".into());
        }
        Ok(format!("{origin}{path}?{key}={}", query_encode(token)))
    }
}

impl Delivery {
    /// `PHYSICIAN_MAIL_TRANSPORT=smtp-relay` uses Google's mandatory-TLS relay
    /// with `PHYSICIAN_MAIL_FROM` and `PHYSICIAN_PUBLIC_ORIGIN`. It needs no
    /// mailbox credentials. The relay allows the server's exact static IP.
    /// An unknown transport or invalid sender fails closed in hosted mode.
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
        if let Some(transport) = nonempty(get("PHYSICIAN_MAIL_TRANSPORT")) {
            let sender = match transport.as_str() {
                "smtp-relay" => nonempty(get("PHYSICIAN_MAIL_FROM"))
                    .and_then(|from| SmtpRelay::google(&from, "Coda Health").ok())
                    .map(Transport::Relay)
                    .unwrap_or(Transport::Unavailable),
                _ => Transport::Unavailable,
            };
            return Self::Hosted(Box::new(Outbound {
                origin: nonempty(get("PHYSICIAN_PUBLIC_ORIGIN")).unwrap_or_default(),
                sender,
            }));
        }
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
        Self::Hosted(Box::new(Outbound::new(origin, sender)))
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

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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
        assert!(matches!(Delivery::from_env(env), Delivery::Hosted(_)));
        assert!(matches!(
            Delivery::from_env(|key| match key {
                "PHYSICIAN_GMAIL_FROM" => Some("  signin@codahealth.ai  ".into()),
                _ => None,
            }),
            Delivery::Hosted(_)
        ));
    }

    #[tokio::test]
    async fn a_misconfigured_relay_never_falls_back_to_returning_tokens() {
        for transport in ["smtp-relay", "typo"] {
            let Delivery::Hosted(mail) = Delivery::from_env(|key| match key {
                "PHYSICIAN_MAIL_TRANSPORT" => Some(transport.into()),
                "PHYSICIAN_PUBLIC_ORIGIN" => Some("https://example.test/physician".into()),
                _ => None,
            }) else {
                panic!("configured mail must stay hosted")
            };
            assert!(
                mail.send_sign_in("doctor@example.test", "secret")
                    .await
                    .is_err()
            );
        }
    }

    #[test]
    fn relay_needs_no_personal_mailbox_credentials() {
        let Delivery::Hosted(mail) = Delivery::from_env(|key| match key {
            "PHYSICIAN_MAIL_TRANSPORT" => Some("smtp-relay".into()),
            "PHYSICIAN_MAIL_FROM" => Some("signin@codahealth.ai".into()),
            _ => None,
        }) else {
            panic!("expected hosted delivery")
        };
        assert!(matches!(mail.sender, Transport::Relay(_)));
    }
}
