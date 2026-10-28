//! Gmail push notifications: which connected mailbox a wake names.
//!
//! Gmail's `users.watch` publishes `{"emailAddress", "historyId"}` to a Pub/Sub
//! topic, and nothing about which messages changed. A push is a wake only: it
//! names the mailbox, and the intake then reads `users.history.list` from its
//! own stored cursor ([`crate::history`]). The `historyId` in the payload is
//! checked for shape and is never exposed, so it cannot move a cursor.
//! See <https://developers.google.com/workspace/gmail/api/guides/push>.
//!
//! Pub/Sub delivers at least once and out of order. Duplicates and stale wakes
//! are harmless only because a wake carries no position.
use std::collections::HashMap;

use serde_json::Value;

/// A Gmail push payload in Gmail's shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    /// The watched mailbox, as Gmail sent it.
    pub email_address: String,
}

/// Decodes Pub/Sub message data as a Gmail push. `None` for anything not in
/// Gmail's shape: not JSON, a missing address, or a `historyId` that is not a
/// decimal string of 1 to 40 digits.
pub fn decode(data: &[u8]) -> Option<Notification> {
    let value: Value = serde_json::from_slice(data).ok()?;
    let email_address = value.get("emailAddress")?.as_str()?;
    let history_id = value.get("historyId")?.as_str()?;
    if email_address.is_empty() || !history_id_shape(history_id) {
        return None;
    }
    Some(Notification {
        email_address: email_address.to_owned(),
    })
}

/// `^\d{1,40}$`, the shape Gmail history ids take.
pub(crate) fn history_id_shape(value: &str) -> bool {
    (1..=40).contains(&value.len()) && value.bytes().all(|byte| byte.is_ascii_digit())
}

/// Connected mailboxes by address, matched without case.
#[derive(Debug, Clone)]
pub struct Routes<T> {
    by_address: HashMap<String, T>,
}

impl<T> Routes<T> {
    /// Routes each mailbox address to the caller's connection key.
    pub fn new<A: AsRef<str>>(mailboxes: impl IntoIterator<Item = (A, T)>) -> Self {
        Self {
            by_address: mailboxes
                .into_iter()
                .map(|(address, key)| (address.as_ref().to_lowercase(), key))
                .collect(),
        }
    }

    /// The connection a Pub/Sub message wakes. `None` for any other mailbox or
    /// a payload not in Gmail's shape; the caller acks those and reads nothing.
    pub fn route(&self, message: &pubsub::envelope::PushMessage) -> Option<&T> {
        self.route_data(&message.data)
    }

    /// [`route`](Self::route) over raw message data.
    pub fn route_data(&self, data: &[u8]) -> Option<&T> {
        let notification = decode(data)?;
        self.by_address
            .get(&notification.email_address.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn routes() -> Routes<&'static str> {
        Routes::new([("Clinic@CodaHealth.test", "clinic")])
    }

    #[test]
    fn a_known_mailbox_routes_without_case() {
        let data = br#"{"emailAddress":"clinic@codahealth.TEST","historyId":"12345"}"#;
        assert_eq!(routes().route_data(data), Some(&"clinic"));
    }

    #[test]
    fn other_mailboxes_and_other_shapes_route_nowhere() {
        for data in [
            &br#"{"emailAddress":"other@codahealth.test","historyId":"1"}"#[..],
            br#"{"emailAddress":"clinic@codahealth.test","historyId":12}"#,
            br#"{"emailAddress":"clinic@codahealth.test","historyId":"-1"}"#,
            br#"{"emailAddress":"clinic@codahealth.test","historyId":""}"#,
            br#"{"emailAddress":"clinic@codahealth.test"}"#,
            br#"{"historyId":"1"}"#,
            b"not json",
            b"",
        ] {
            assert_eq!(
                routes().route_data(data),
                None,
                "{}",
                String::from_utf8_lossy(data)
            );
        }
        let long = format!(
            r#"{{"emailAddress":"clinic@codahealth.test","historyId":"{}"}}"#,
            "9".repeat(41)
        );
        assert_eq!(routes().route_data(long.as_bytes()), None);
    }
}
