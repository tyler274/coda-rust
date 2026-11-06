//! Account-session-local provisional presentation. Never serialize into retained
//! drafts, transcript messages, citations, exports, or clinical context.
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

/// One durable SSE envelope. Cursor identity is scoped by the requested command.
#[derive(Clone, Debug, Deserialize)]
pub struct Event {
    /// Monotonic server sequence (gaps across commands are normal).
    pub id: i64,
    /// Product event, not a provider transcript.
    pub event: Value,
}

/// Transient reducer owned by one mounted authenticated patient session.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Provisional {
    /// Command being observed.
    pub command: Option<Uuid>,
    /// Last applied cursor; duplicate/replayed envelopes are ignored.
    pub after: i64,
    /// Unapproved preview only.
    pub text: String,
    /// Terminal or interrupted streams require transcript readback.
    pub stopped: bool,
    claim: i64,
    exchange: u64,
}
impl Provisional {
    /// Start/reload one command from its replayable beginning.
    pub fn new(command: Uuid) -> Self {
        Self {
            command: Some(command),
            ..Self::default()
        }
    }
    /// Apply a whole validated batch in server order. Never sort malformed
    /// input into apparent validity or mix envelopes from another command.
    pub fn apply(&mut self, command: Uuid, events: &[Event]) -> Result<(), String> {
        if self.command != Some(command) {
            return Err("stream command changed".into());
        }
        if events.windows(2).any(|w| w[0].id >= w[1].id) {
            return Err("stream cursor ordering invalid".into());
        }
        let mut next = self.clone();
        for envelope in events {
            if envelope.id <= next.after {
                continue;
            }
            let event = &envelope.event;
            match event["type"].as_str() {
                Some("claimed") => {
                    next.text.clear();
                    next.stopped = false;
                    next.claim = event["claim"].as_i64().ok_or("missing claim")?;
                    next.exchange = 0;
                }
                Some("stream_reset") => {
                    next.text.clear();
                    next.stopped = false;
                    next.claim = event["claim"].as_i64().ok_or("missing claim")?;
                    next.exchange = event["exchange"].as_u64().ok_or("missing exchange")?;
                }
                Some("provisional_text") if !next.stopped => {
                    if event["claim"].as_i64() != Some(next.claim)
                        || event["exchange"].as_u64() != Some(next.exchange)
                    {
                        return Err("stream attempt changed without reset".into());
                    }
                    let text = event["text"].as_str().ok_or("missing provisional text")?;
                    if next.text.len() + text.len() > 1_048_576 {
                        return Err("stream exceeds bound".into());
                    }
                    next.text.push_str(text);
                }
                Some("completed" | "assessment_completed" | "interrupted" | "cancelled") => {
                    next.text.clear();
                    next.stopped = true;
                }
                _ => {}
            }
            next.after = envelope.id;
        }
        *self = next;
        Ok(())
    }
    /// A lost authorization or uncertain transport must not leave old health
    /// text visible. Reset cursor too so reconnect reconstructs exact state.
    pub fn clear(&mut self) {
        let command = self.command;
        *self = Self {
            command,
            ..Self::default()
        };
    }
}

/// Parse finite SSE batches. Unknown fields and retry hints are ignored; every
/// conversation event requires one sequence and JSON payload.
pub fn parse_batch(body: &str) -> Result<Vec<Event>, String> {
    if body.len() > 2_097_152 {
        return Err("event batch exceeds bound".into());
    }
    let normalized = body.replace("\r\n", "\n");
    let mut result = Vec::new();
    for block in normalized.split("\n\n") {
        let mut id = None;
        let mut data = Vec::new();
        let mut kind = None;
        for line in block.lines() {
            if let Some(value) = line.strip_prefix("id:") {
                id = value.trim().parse::<i64>().ok();
            }
            if let Some(value) = line.strip_prefix("event:") {
                kind = Some(value.trim());
            }
            if let Some(value) = line.strip_prefix("data:") {
                data.push(value.trim_start());
            }
        }
        if kind != Some("conversation") {
            continue;
        }
        result.push(Event {
            id: id.filter(|id| *id > 0).ok_or("invalid event cursor")?,
            event: serde_json::from_str(&data.join("\n")).map_err(|_| "invalid event payload")?,
        });
    }
    Ok(result)
}

impl crate::DuetClient {
    /// Reconnectable authenticated batch; unlike EventSource this also carries
    /// the native account bearer. Never places credentials in query strings.
    pub async fn conversation_events(
        &self,
        command: Uuid,
        after: i64,
    ) -> Result<Vec<Event>, String> {
        let response = self
            .configuration
            .client
            .get(format!(
                "{}/api/conversation/commands/{command}/events",
                self.configuration.base_path
            ))
            .bearer_auth(
                self.configuration
                    .bearer_access_token
                    .as_deref()
                    .unwrap_or_default(),
            )
            .query(&[("after", after)])
            .send()
            .await
            .map_err(|_| "stream unavailable")?;
        let status = response.status();
        let body = response.text().await.map_err(|_| "stream interrupted")?;
        if !status.is_success() {
            self.observe_refusal(status, &body);
            return Err("stream not authorized or unavailable".into());
        }
        parse_batch(&body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn event(id: i64, event: Value) -> Event {
        Event { id, event }
    }
    #[test]
    fn replay_reset_terminal_and_command_isolation() {
        let id = Uuid::new_v4();
        let mut state = Provisional::new(id);
        let batch = vec![
            event(2, json!({"type":"stream_reset","claim":1,"exchange":1})),
            event(
                4,
                json!({"type":"provisional_text","claim":1,"exchange":1,"text":"Draft"}),
            ),
        ];
        state.apply(id, &batch).unwrap();
        state.apply(id, &batch).unwrap();
        assert_eq!(state.text, "Draft");
        assert!(state.apply(Uuid::new_v4(), &batch).is_err());
        assert!(
            state
                .apply(id, &[event(6, json!({})), event(5, json!({}))])
                .is_err()
        );
        state
            .apply(id, &[event(7, json!({"type":"interrupted"}))])
            .unwrap();
        assert!(state.text.is_empty());
        assert!(state.stopped);
        state
            .apply(
                id,
                &[
                    event(8, json!({"type":"stream_reset","claim":2,"exchange":2})),
                    event(
                        9,
                        json!({"type":"provisional_text","claim":2,"exchange":2,"text":"New"}),
                    ),
                ],
            )
            .unwrap();
        assert_eq!(state.text, "New");
        state
            .apply(
                id,
                &[event(
                    10,
                    json!({"type":"completed","reply":"Final is read separately"}),
                )],
            )
            .unwrap();
        assert!(state.text.is_empty());
        assert!(state.stopped);
        state.clear();
        assert_eq!(state.after, 0);
    }
    #[test]
    fn finite_sse_parser_keeps_order_and_rejects_missing_cursor() {
        let events=parse_batch("retry: 1000\n\nid: 9\nevent: conversation\ndata: {\"type\":\"claimed\",\"claim\":1}\n\n").unwrap();
        assert_eq!(events[0].id, 9);
        assert!(parse_batch("event: conversation\ndata: {}\n\n").is_err());
    }
}
