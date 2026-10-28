//! The standard Pub/Sub push body, and the message a pull returns.
//!
//! Push: `{"message": {"data", "messageId", "orderingKey", "attributes", ...},
//! "subscription", "deliveryAttempt"}`. Pull's `receivedMessages[]` carry the
//! same message object beside an `ackId`. Pub/Sub sends both the camelCase and
//! the snake_case id; either is accepted. See
//! <https://cloud.google.com/pubsub/docs/push#receive_push>.
//!
//! Decoding says nothing about what the data means. A product decides whether
//! a message it cannot read is acked (dropped) or left for redelivery.
use std::collections::BTreeMap;

use base64::Engine;
use serde_json::Value;

/// One delivered message, decoded from a push body or a pulled message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushMessage {
    /// The publisher's bytes, base64-decoded. Empty when the message had none.
    pub data: Vec<u8>,
    /// Pub/Sub's id for the published message; the same on every redelivery.
    pub message_id: String,
    /// The ordering key, when the publisher set one.
    pub ordering_key: Option<String>,
    /// Publisher attributes.
    pub attributes: BTreeMap<String, String>,
    /// The push subscription, when the body named one.
    pub subscription: Option<String>,
    /// Which delivery this is, present when the subscription dead-letters.
    pub delivery_attempt: Option<u32>,
}

/// Why a body is not a Pub/Sub delivery.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Not JSON, or no `message` object.
    #[error("not a Pub/Sub push body")]
    Shape,
    /// No non-empty message id.
    #[error("Pub/Sub message has no id")]
    MissingId,
    /// `data` is present but not standard base64.
    #[error("Pub/Sub message data is not base64")]
    Data,
}

/// Decodes a push request body.
pub fn decode(bytes: &[u8]) -> Result<PushMessage, Error> {
    let body: Value = serde_json::from_slice(bytes).map_err(|_| Error::Shape)?;
    let message = body.get("message").ok_or(Error::Shape)?;
    let mut decoded = decode_message(message)?;
    decoded.subscription = body
        .get("subscription")
        .and_then(Value::as_str)
        .map(str::to_owned);
    decoded.delivery_attempt = body
        .get("deliveryAttempt")
        .and_then(Value::as_u64)
        .and_then(|attempt| u32::try_from(attempt).ok());
    Ok(decoded)
}

/// Decodes one Pub/Sub message object, as in a push body's `message` or a
/// pull's `receivedMessages[].message`.
pub fn decode_message(message: &Value) -> Result<PushMessage, Error> {
    if !message.is_object() {
        return Err(Error::Shape);
    }
    let text = |camel: &str, snake: &str| {
        message
            .get(camel)
            .and_then(Value::as_str)
            .or_else(|| message.get(snake).and_then(Value::as_str))
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
    };
    let message_id = text("messageId", "message_id").ok_or(Error::MissingId)?;
    let data = match message.get("data").and_then(Value::as_str) {
        Some(data) => base64::engine::general_purpose::STANDARD
            .decode(data.trim())
            .map_err(|_| Error::Data)?,
        None => Vec::new(),
    };
    let attributes = message
        .get("attributes")
        .and_then(Value::as_object)
        .map(|attributes| {
            attributes
                .iter()
                .filter_map(|(key, value)| Some((key.clone(), value.as_str()?.to_owned())))
                .collect()
        })
        .unwrap_or_default();
    Ok(PushMessage {
        data,
        message_id,
        ordering_key: text("orderingKey", "ordering_key"),
        attributes,
        subscription: None,
        delivery_attempt: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn encoded(text: &str) -> String {
        base64::engine::general_purpose::STANDARD.encode(text)
    }

    #[test]
    fn a_push_body_decodes_data_ids_ordering_and_attempt() {
        let body = json!({
            "message": {
                "data": encoded(r#"{"k":1}"#),
                "messageId": "m1",
                "message_id": "m1",
                "orderingKey": "mailbox-a",
                "attributes": {"kind": "wake"},
                "publishTime": "2026-01-01T00:00:00Z"
            },
            "subscription": "projects/p/subscriptions/s",
            "deliveryAttempt": 2
        });
        let message = decode(body.to_string().as_bytes()).unwrap();
        assert_eq!(message.data, br#"{"k":1}"#);
        assert_eq!(message.message_id, "m1");
        assert_eq!(message.ordering_key.as_deref(), Some("mailbox-a"));
        assert_eq!(message.attributes["kind"], "wake");
        assert_eq!(
            message.subscription.as_deref(),
            Some("projects/p/subscriptions/s")
        );
        assert_eq!(message.delivery_attempt, Some(2));
    }

    #[test]
    fn either_id_spelling_is_accepted_and_empty_ordering_is_none() {
        let body =
            json!({"message": {"data": encoded("x"), "message_id": "m2", "orderingKey": ""}});
        let message = decode(body.to_string().as_bytes()).unwrap();
        assert_eq!(message.message_id, "m2");
        assert_eq!(message.ordering_key, None);
        assert_eq!(message.delivery_attempt, None);
    }

    #[test]
    fn malformed_bodies_are_refused() {
        assert_eq!(decode(b"not json"), Err(Error::Shape));
        assert_eq!(decode(b"{}"), Err(Error::Shape));
        assert_eq!(decode(br#"{"message": "m"}"#), Err(Error::Shape));
        assert_eq!(
            decode(br#"{"message": {"data": "eA=="}}"#),
            Err(Error::MissingId)
        );
        assert_eq!(
            decode(br#"{"message": {"data": "%%%", "messageId": "m"}}"#),
            Err(Error::Data)
        );
    }
}
