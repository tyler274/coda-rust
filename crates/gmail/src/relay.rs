//! Workspace relay transport. No mailbox, OAuth grant, or SMTP password.

use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::extension::ClientId;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::Error;

/// Sends through `smtp-relay.gmail.com:587` with mandatory STARTTLS.
/// Workspace must allow the sending server's exact static IP and domain.
pub struct SmtpRelay {
    from: Mailbox,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpRelay {
    /// Builds a branded sender. No user credential is read or transmitted.
    pub fn google(address: &str, display_name: &str) -> Result<Self, Error> {
        if !crate::header_value_ok(address) || !crate::header_value_ok(display_name) {
            return Err(Error::Headers);
        }
        let address: lettre::Address = address.parse().map_err(|_| Error::RelayConfiguration)?;
        let transport =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp-relay.gmail.com")
                .map_err(|_| Error::RelayConfiguration)?
                .hello_name(ClientId::Domain(address.domain().to_owned()))
                .timeout(Some(std::time::Duration::from_secs(20)))
                .build();
        Ok(Self {
            from: Mailbox::new(Some(display_name.to_owned()), address),
            transport,
        })
    }

    fn message(&self, to: &str, subject: &str, body: &str) -> Result<Message, Error> {
        if !crate::header_value_ok(to) || !crate::header_value_ok(subject) {
            return Err(Error::Headers);
        }
        Message::builder()
            .from(self.from.clone())
            .to(Mailbox::new(None, to.parse().map_err(|_| Error::Headers)?))
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_owned())
            .map_err(|_| Error::Headers)
    }

    /// Returns success only after the relay accepts the message.
    /// Recipient delivery still requires mailbox readback.
    pub async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), Error> {
        self.transport
            .send(self.message(to, subject, body)?)
            .await
            .map(|_| ())
            .map_err(|_| Error::RelayDelivery)
    }

    /// Sends HTML with an equivalent plain-text alternative over the same TLS relay.
    pub async fn send_html(
        &self,
        to: &str,
        subject: &str,
        text: &str,
        html: &str,
    ) -> Result<(), Error> {
        self.transport
            .send(crate::html_message(
                self.from.clone(),
                to,
                subject,
                text,
                html,
            )?)
            .await
            .map(|_| ())
            .map_err(|_| Error::RelayDelivery)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    #[test]
    fn message_uses_the_product_sender_and_refuses_header_injection() {
        let relay = SmtpRelay::google("signin@codahealth.ai", "Coda Health").unwrap();
        let message = relay
            .message(
                "doctor@example.test",
                "Your Coda sign-in link",
                "single-use-code",
            )
            .unwrap();
        assert_eq!(
            message.envelope().from().unwrap().to_string(),
            "signin@codahealth.ai"
        );
        let raw = String::from_utf8(message.formatted()).unwrap();
        assert!(
            raw.contains("From: \"Coda Health\" <signin@codahealth.ai>"),
            "{raw}"
        );
        assert!(
            relay
                .message(
                    "doctor@example.test\r\nBcc: other@example.test",
                    "Sign in",
                    "code"
                )
                .is_err()
        );
        assert!(SmtpRelay::google("", "Coda Health").is_err());
    }

    #[tokio::test]
    async fn a_relay_without_starttls_never_receives_the_token() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            let mut socket = BufReader::new(socket);
            socket
                .get_mut()
                .write_all(b"220 localhost ESMTP\r\n")
                .await
                .unwrap();
            let mut line = String::new();
            socket.read_line(&mut line).await.unwrap();
            assert!(line.starts_with("EHLO "));
            socket
                .get_mut()
                .write_all(b"250 localhost\r\n")
                .await
                .unwrap();
            let mut rest = String::new();
            while socket.read_line(&mut rest).await.unwrap_or(0) > 0 {
                if rest.ends_with("QUIT\r\n") {
                    break;
                }
            }
            rest
        });
        let mut relay = SmtpRelay::google("signin@codahealth.ai", "Coda Health").unwrap();
        relay.transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("localhost")
            .unwrap()
            .port(port)
            .timeout(Some(std::time::Duration::from_secs(2)))
            .build();
        assert!(
            relay
                .send("doctor@example.test", "Sign in", "secret-token")
                .await
                .is_err()
        );
        let commands = server.await.unwrap();
        assert!(!commands.contains("MAIL FROM") && !commands.contains("secret-token"));
    }
}
