//! Only an explicit product error envelope can end an authenticated client session.
use crate::{
    DuetClient,
    session::{AccountSession, SessionFence},
};
use futures_channel::mpsc::UnboundedSender;

/// Definitive product refusal, not a proxy challenge or transport failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionRefusal {
    /// Duet has rejected the supplied credential.
    Invalid,
    /// Duet has ended this session.
    Revoked,
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
        _ => None,
    }
}

#[derive(Clone)]
pub(crate) struct RefusalBinding {
    session: AccountSession,
    fence: SessionFence,
    sender: UnboundedSender<SessionFence>,
}
impl DuetClient {
    /// Bind responses to the exact mounted session which dispatched them.
    pub fn bind_session(
        mut self,
        session: AccountSession,
        fence: SessionFence,
        sender: UnboundedSender<SessionFence>,
    ) -> Self {
        self.refusal = Some(RefusalBinding {
            session,
            fence,
            sender,
        });
        self
    }
    pub(crate) fn observe_refusal(&self, status: reqwest::StatusCode, body: &str) {
        if classify(status, body).is_none() {
            return;
        }
        let Some(binding) = &self.refusal else {
            return;
        };
        let invalidated = binding.session.invalidate(&binding.fence);
        if invalidated {
            // A dropped receiver means the root has already unmounted.
            let _ = binding.sender.unbounded_send(binding.fence.clone());
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
