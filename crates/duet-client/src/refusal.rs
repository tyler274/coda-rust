//! Only an explicit product error envelope can end an authenticated client session.
use crate::{
    DuetClient,
    session::{AccountSession, SessionFence},
};
use futures_channel::mpsc::UnboundedSender;

/// A transport cause with its credential-bearing URL removed.
#[derive(Clone, Debug)]
pub struct TransportCause(std::sync::Arc<reqwest::Error>);
impl From<reqwest::Error> for TransportCause {
    fn from(error: reqwest::Error) -> Self {
        Self(std::sync::Arc::new(error.without_url()))
    }
}
impl PartialEq for TransportCause {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for TransportCause {}
impl std::fmt::Display for TransportCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("response unavailable")
    }
}
impl std::error::Error for TransportCause {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}

/// Definitive product refusal, not a proxy challenge or transport failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionRefusal {
    /// Duet has rejected the supplied credential.
    Invalid,
    /// Duet has ended this session.
    Revoked,
    /// The verified subject has no Duet account.
    Absent,
}

/// Decode only the product's machine-readable contract. Unknown 401s stay recoverable.
pub fn classify(status: reqwest::StatusCode, body: &str) -> Option<SessionRefusal> {
    if status != reqwest::StatusCode::UNAUTHORIZED {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(body).ok()?;
    match value["detail"]["code"].as_str()? {
        "session_invalid" => Some(SessionRefusal::Invalid),
        "session_revoked" => Some(SessionRefusal::Revoked),
        "account_absent" => Some(SessionRefusal::Absent),
        // An expired token refreshes first. It does not end the session.
        "session_expired" => None,
        _ => None,
    }
}

#[derive(Clone)]
pub(crate) struct RefusalBinding {
    session: AccountSession,
    fence: SessionFence,
    credential_revision: u64,
    sender: UnboundedSender<(SessionFence, SessionRefusal)>,
}
impl DuetClient {
    /// Bind responses to the exact mounted session which dispatched them.
    pub fn bind_session(
        mut self,
        session: AccountSession,
        fence: SessionFence,
        sender: UnboundedSender<(SessionFence, SessionRefusal)>,
    ) -> Self {
        self.refusal = Some(RefusalBinding {
            credential_revision: session.credential_revision(),
            session,
            fence,
            sender,
        });
        self
    }
    pub(crate) fn observe_refusal(&self, status: reqwest::StatusCode, body: &str) {
        let Some(refusal) = classify(status, body) else {
            return;
        };
        let Some(binding) = &self.refusal else {
            return;
        };
        let invalidated = binding
            .session
            .refuse_credential(&binding.fence, binding.credential_revision);
        if invalidated {
            // A dropped receiver means the root has already unmounted.
            let _ = binding
                .sender
                .unbounded_send((binding.fence.clone(), refusal));
        }
    }
    pub(crate) fn wire_error<T>(&self, error: duet_wire::apis::Error<T>) -> String {
        if let duet_wire::apis::Error::ResponseError(response) = &error {
            self.observe_refusal(response.status, &response.content);
        }
        error.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn health_generation_and_upload_refusals_invalidate_bound_session() {
        use crate::health_upload::EvidenceTransport;
        let app = axum::Router::new().fallback(|| async {
            (
                reqwest::StatusCode::UNAUTHORIZED,
                r#"{"detail":{"code":"session_revoked"}}"#,
            )
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let session = AccountSession::default();
        for upload in [false, true] {
            let fence = session.switch(Some("a".into())).unwrap();
            let (sender, _receiver) = futures_channel::mpsc::unbounded();
            let client = DuetClient::new(&url, "test", "0", "0")
                .unwrap()
                .bind_session(session.clone(), fence.clone(), sender);
            if upload {
                client
                    .send(&duet_health::outbox::UploadBatch {
                        source_generation: 1,
                        items: vec![],
                    })
                    .await;
            } else {
                assert!(crate::apple_health::generation(&client).await.is_err());
            }
            assert!(!session.is_current(&fence));
        }
        server.abort();
    }
    #[tokio::test]
    async fn refreshed_credential_survives_refusal_of_original_credential() {
        use axum::{Router, routing::get};
        let router = Router::new().fallback(get(|headers: axum::http::HeaderMap| async move {
            let json = [(axum::http::header::CONTENT_TYPE, "application/json")];
            if headers.get("authorization").and_then(|v| v.to_str().ok()) == Some("Bearer fresh") {
                (reqwest::StatusCode::OK, json, "[]")
            } else {
                (
                    reqwest::StatusCode::UNAUTHORIZED,
                    json,
                    r#"{"detail":{"code":"session_invalid"}}"#,
                )
            }
        }));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        let session = AccountSession::default();
        let fence = session.switch(Some("a".into())).unwrap();
        let (sender, _receiver) = futures_channel::mpsc::unbounded();
        let old = DuetClient::new(&url, "expired", "0", "0")
            .unwrap()
            .bind_session(session.clone(), fence.clone(), sender.clone());
        session.rotate_credential(&fence);
        let fresh = DuetClient::new(&url, "fresh", "0", "0")
            .unwrap()
            .bind_session(session.clone(), fence.clone(), sender);
        assert!(old.list_moments().await.is_err());
        assert!(session.is_current(&fence));
        assert!(fresh.list_moments().await.is_ok());
        assert!(session.is_current(&fence));
        server.abort();
    }
    #[test]
    fn only_product_refusal_ends_the_current_session() {
        let session = AccountSession::default();
        let a = session.switch(Some("a".into())).unwrap();
        let (sender, _receiver) = futures_channel::mpsc::unbounded();
        let client = DuetClient::new("http://localhost", "test", "0", "0")
            .unwrap()
            .bind_session(session.clone(), a.clone(), sender);
        for (status, body) in [
            (401, "<html>IAP sign in</html>"),
            (401, r#"{"error":{"message":"INVALID_API_KEY"}}"#),
            (401, r#"{"detail":"Invalid bearer token."}"#),
            (503, r#"{"detail":{"code":"session_revoked"}}"#),
            (403, r#"{"detail":{"code":"terms_acceptance_required"}}"#),
        ] {
            client.observe_refusal(reqwest::StatusCode::from_u16(status).unwrap(), body);
            assert!(session.is_current(&a));
        }
        session.rotate_credential(&a);
        client.observe_refusal(
            reqwest::StatusCode::UNAUTHORIZED,
            r#"{"detail":{"code":"session_invalid"}}"#,
        );
        assert!(
            session.is_current(&a),
            "refusal from replaced access token must not end rotated session"
        );
        let b = session.switch(Some("b".into())).unwrap();
        client.observe_refusal(
            reqwest::StatusCode::UNAUTHORIZED,
            r#"{"detail":{"code":"session_revoked"}}"#,
        );
        assert!(session.is_current(&b));
        let (sender, _receiver) = futures_channel::mpsc::unbounded();
        let client = client.bind_session(session.clone(), b.clone(), sender);
        client.observe_refusal(
            reqwest::StatusCode::UNAUTHORIZED,
            r#"{"detail":{"code":"session_invalid"}}"#,
        );
        assert!(!session.is_current(&b));
        assert!(!session.store_draft(&b, "late".into(), vec![1]));
    }
}
