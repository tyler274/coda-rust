//! Content-free operational signals for Duet's background loops.
//!
//! Failure events carry only a static event name, the failing step and a
//! coarse error class. They never carry error text, account or command
//! identifiers, or conversation content.

use crate::{ApiError, privacy::PrivacyStore};
use tracing::Instrument;

/// Error class for failures reported by the database driver.
pub const DATABASE: &str = "database";

const PRIVACY_RECOVERY_FAILED: &str = "duet.privacy_recovery.failed";

/// Retries pending privacy journal receipts every 30 seconds.
pub async fn recover_privacy(recovery: PrivacyStore) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    loop {
        interval.tick().await;
        privacy_sweep(&recovery)
            .instrument(tracing::info_span!(
                "duet.privacy_recovery.sweep",
                otel.status_code = tracing::field::Empty
            ))
            .await;
    }
}

async fn privacy_sweep(recovery: &PrivacyStore) {
    let accounts = match recovery.database.pending_privacy_accounts().await {
        Ok(accounts) => accounts,
        Err(_) => {
            failed(PRIVACY_RECOVERY_FAILED, "pending", DATABASE);
            return;
        }
    };
    for account in accounts {
        if let Err(e) = recovery.recover(account).await {
            failed(PRIVACY_RECOVERY_FAILED, "recover", api_error_class(&e));
        }
    }
}

/// Coarse, content-free class of an [`ApiError`].
pub fn api_error_class(e: &ApiError) -> &'static str {
    match e {
        ApiError::Unauthorized
        | ApiError::InvalidToken(_)
        | ApiError::SessionRevoked
        | ApiError::SessionExpired
        | ApiError::AbsentAccount
        | ApiError::Forbidden
        | ApiError::TermsRequired
        | ApiError::AgreementChanged => "identity",
        ApiError::Conflict(_) => "conflict",
        ApiError::NotFound | ApiError::JoinInvalid | ApiError::ConsentUnlinked => "not_found",
        ApiError::Unavailable(_) | ApiError::IdentityResponse(_) => "unavailable",
        ApiError::Storage(_) => DATABASE,
        ApiError::Unprocessable(_) => "invalid",
        ApiError::Internal(_) => "internal",
    }
}

/// Reports a failed background step and marks the current span as errored.
pub fn failed(event: &'static str, step: &'static str, error_class: &'static str) {
    tracing::Span::current().record("otel.status_code", "ERROR");
    tracing::error!(event, step, error_class, "duet background step failed");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::sync::{Arc, Mutex};

    #[test]
    fn every_api_error_has_a_class() {
        let cases = [
            (ApiError::Unauthorized, "identity"),
            (
                ApiError::InvalidToken(jsonwebtoken::errors::ErrorKind::InvalidToken.into()),
                "identity",
            ),
            (ApiError::SessionRevoked, "identity"),
            (ApiError::AbsentAccount, "identity"),
            (ApiError::Forbidden, "identity"),
            (ApiError::TermsRequired, "identity"),
            (ApiError::AgreementChanged, "identity"),
            (ApiError::Conflict("claim"), "conflict"),
            (ApiError::NotFound, "not_found"),
            (ApiError::JoinInvalid, "not_found"),
            (ApiError::ConsentUnlinked, "not_found"),
            (ApiError::Unavailable("provider"), "unavailable"),
            (ApiError::Storage("SECRET patient text".into()), "database"),
            (ApiError::Unprocessable("body"), "invalid"),
            (ApiError::SessionExpired, "identity"),
            (ApiError::Internal("shape"), "internal"),
        ];
        for (error, class) in cases {
            assert_eq!(api_error_class(&error), class);
        }
    }

    #[derive(Clone, Default)]
    struct Capture(Arc<Mutex<Vec<u8>>>);

    impl io::Write for Capture {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0
                .lock()
                .map_err(|_| io::Error::other("capture poisoned"))?
                .extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn failure_event_names_the_class_without_error_text() {
        let capture = Capture::default();
        let writer = capture.clone();
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(false)
            .with_writer(move || writer.clone())
            .finish();
        tracing::subscriber::with_default(subscriber, || {
            let span = tracing::info_span!(
                "duet.recovery.sweep",
                otel.status_code = tracing::field::Empty
            );
            let _entered = span.enter();
            failed(
                "duet.recovery.failed",
                "launch",
                api_error_class(&ApiError::Storage("SECRET-PATIENT-TEXT".into())),
            );
        });
        let bytes = capture.0.lock().map(|b| b.clone()).unwrap_or_default();
        let output = String::from_utf8_lossy(&bytes);
        assert!(output.contains("duet.recovery.sweep"), "{output}");
        assert!(output.contains("otel.status_code=\"ERROR\""), "{output}");
        assert!(output.contains("duet.recovery.failed"), "{output}");
        assert!(output.contains("step=\"launch\""), "{output}");
        assert!(output.contains("error_class"), "{output}");
        assert!(output.contains("database"), "{output}");
        assert!(!output.contains("SECRET-PATIENT-TEXT"), "{output}");
    }
}
