//! Product-neutral Pub/Sub mechanics shared by the physician runtime and the
//! Gmail intake primitives.
//!
//! - [`envelope`] decodes the body Pub/Sub POSTs to a push endpoint, and the
//!   message object a pull returns, into one [`envelope::PushMessage`].
//! - [`validate_emulator`] accepts only a literal loopback `IP:port`, and never
//!   in hosted execution.
//! - With the `publisher` feature, [`publisher`] builds the official Google
//!   publisher, against Google with ADC or against a checked loopback emulator
//!   with anonymous credentials.
//! - With the `test-util` feature, `testing::Emulator` is a disposable
//!   `gcloud beta emulators pubsub` for another crate's tests. That feature is
//!   allowed only under `[dev-dependencies]` (`crates/boundaries`).
//!
//! Products own what a message means, whether it is acked, and any durable
//! state behind it. This crate carries no product types. Book: `book/src/gcp.md`.
#![deny(missing_docs)]

pub mod envelope;

#[cfg(feature = "test-util")]
pub mod testing;

/// Accept only literal loopback IP endpoints; no DNS or URL credential tricks.
/// Hosted markers always prohibit emulator endpoints.
pub fn validate_emulator(endpoint: &str, hosted: bool) -> Result<(), String> {
    if hosted {
        return Err("Pub/Sub emulator is forbidden in hosted execution".into());
    }
    let address: std::net::SocketAddr = endpoint
        .parse()
        .map_err(|_| "emulator must be a loopback IP:port")?;
    if !address.ip().is_loopback() || address.port() == 0 {
        return Err("emulator must be a loopback IP:port".into());
    }
    Ok(())
}

/// The official Google publisher for one fully qualified topic.
///
/// Without `emulator` it uses Application Default Credentials. With it, the
/// endpoint must pass [`validate_emulator`] and the publisher sends no
/// credentials. Hosted execution always refuses an emulator.
#[cfg(feature = "publisher")]
pub async fn publisher(
    topic: &str,
    emulator: Option<&str>,
    hosted: bool,
) -> Result<google_cloud_pubsub::client::Publisher, String> {
    if !topic.starts_with("projects/") || !topic.contains("/topics/") {
        return Err("Pub/Sub requires a fully qualified topic".into());
    }
    let mut builder = google_cloud_pubsub::client::Publisher::builder(topic);
    if let Some(endpoint) = emulator {
        validate_emulator(endpoint, hosted)?;
        builder = builder
            .with_endpoint(format!("http://{endpoint}"))
            .with_credentials(google_cloud_auth::credentials::anonymous::Builder::new().build());
    }
    builder
        .build()
        .await
        .map_err(|_| "Pub/Sub publisher initialization failed".into())
}

#[cfg(test)]
mod tests {
    use super::validate_emulator;

    #[test]
    fn only_literal_loopback_emulators_are_accepted() {
        assert!(validate_emulator("127.0.0.1:8085", false).is_ok());
        assert!(validate_emulator("[::1]:8085", false).is_ok());
        for refused in [
            "localhost:8085",
            "10.0.0.1:8085",
            "0.0.0.0:8085",
            "127.0.0.1:0",
            "127.0.0.1",
            "http://127.0.0.1:8085",
            "user@127.0.0.1:8085",
            "",
        ] {
            assert!(validate_emulator(refused, false).is_err(), "{refused}");
        }
    }

    #[test]
    fn hosted_execution_refuses_every_emulator() {
        assert!(validate_emulator("127.0.0.1:8085", true).is_err());
    }

    #[cfg(feature = "publisher")]
    #[tokio::test]
    async fn the_publisher_refuses_partial_topics_and_remote_emulators() {
        for topic in ["physician-chat-turns", "projects/p/subscriptions/s"] {
            assert!(
                super::publisher(topic, None, false).await.is_err(),
                "{topic}"
            );
        }
        assert!(
            super::publisher("projects/p/topics/t", Some("10.0.0.1:8085"), false)
                .await
                .is_err()
        );
        assert!(
            super::publisher("projects/p/topics/t", Some("127.0.0.1:8085"), true)
                .await
                .is_err()
        );
    }
}
