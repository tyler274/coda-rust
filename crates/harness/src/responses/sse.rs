//! Bounded Responses SSE framing and identity validation. No tool dispatch here.
use std::collections::{BTreeMap, HashSet};

use serde_json::Value;

use crate::engine::{DeltaSink, Failure, TextDelta, Turn};

fn protocol(message: &str) -> Failure {
    Failure::Protocol(message.into())
}
fn string(value: &Value, key: &str) -> Result<String, Failure> {
    super::string(value, key).map_err(Failure::Protocol)
}
fn index(value: &Value, key: &str) -> Result<usize, Failure> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| protocol("missing stream index"))
}

pub(super) struct Frame {
    event: Option<String>,
    data: String,
}

/// Bytes are retained until a complete line exists, so UTF-8 may span chunks.
/// Total stream bytes bound line, frame, event count and terminal payload storage.
pub(super) struct Decoder {
    cap: usize,
    consumed: usize,
    line: Vec<u8>,
    data: Vec<String>,
    event: Option<String>,
    cr: bool,
}
impl Decoder {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            consumed: 0,
            line: vec![],
            data: vec![],
            event: None,
            cr: false,
        }
    }
    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<Frame>, Failure> {
        if bytes.len() > self.cap.saturating_sub(self.consumed) {
            return Err(protocol("SSE byte limit"));
        }
        self.consumed += bytes.len();
        let mut frames = vec![];
        for byte in bytes {
            if self.cr {
                self.cr = false;
                if *byte == b'\n' {
                    continue;
                }
            }
            if matches!(*byte, b'\n' | b'\r') {
                self.cr = *byte == b'\r';
                let line = String::from_utf8(std::mem::take(&mut self.line))
                    .map_err(|_| protocol("invalid SSE UTF-8"))?;
                if line.is_empty() {
                    if !self.data.is_empty() {
                        frames.push(Frame {
                            event: self.event.take(),
                            data: self.data.join("\n"),
                        });
                        self.data.clear();
                    } else {
                        self.event = None;
                    }
                } else if !line.starts_with(':') {
                    let (field, value) = line.split_once(':').unwrap_or((&line, ""));
                    let value = value.strip_prefix(' ').unwrap_or(value);
                    match field {
                        "data" => self.data.push(value.into()),
                        "event" => self.event = Some(value.into()),
                        "id" | "retry" => (), // Never reconnect; JSON sequence owns identity.
                        _ => (),
                    }
                }
            } else {
                self.line.push(*byte);
            }
        }
        Ok(frames)
    }
    pub fn finish(&self) -> Result<(), Failure> {
        if !self.line.is_empty() || !self.data.is_empty() || self.event.is_some() {
            return Err(protocol("truncated SSE frame"));
        }
        Ok(())
    }
}

#[derive(Default)]
struct Output {
    added: Value,
    done: Option<Value>,
    parts: BTreeMap<usize, String>,
    closed_parts: HashSet<usize>,
    finished_parts: HashSet<usize>,
    arguments: String,
    arguments_done: bool,
}
#[derive(Default)]
pub(super) struct Stream {
    response: Option<String>,
    sequence: Option<u64>,
    outputs: BTreeMap<usize, Output>,
    ids: HashSet<String>,
    calls: HashSet<String>,
    terminal: Option<Turn>,
}
impl Stream {
    pub async fn accept(&mut self, frame: Frame, sink: &mut dyn DeltaSink) -> Result<(), Failure> {
        if self.terminal.is_some() {
            return Err(protocol("event after terminal response"));
        }
        let event: Value =
            serde_json::from_str(&frame.data).map_err(|_| protocol("invalid SSE JSON"))?;
        let kind = string(&event, "type")?;
        if frame.event.as_ref().is_some_and(|name| name != &kind) {
            return Err(protocol("SSE event type mismatch"));
        }
        let sequence = event
            .get("sequence_number")
            .and_then(Value::as_u64)
            .ok_or_else(|| protocol("missing event sequence"))?;
        if self.sequence.is_some_and(|prior| sequence <= prior) {
            return Err(protocol("repeated or reordered event sequence"));
        }
        if let Some(prior) = self.sequence
            && prior.checked_add(1) != Some(sequence)
        {
            return Err(protocol("gap in event sequence"));
        }
        self.sequence = Some(sequence);
        if kind == "response.created" {
            if self.response.is_some() {
                return Err(protocol("repeated response creation"));
            }
            let id = string(&event["response"], "id")?;
            if id.is_empty() {
                return Err(protocol("empty response identity"));
            }
            self.response = Some(id);
            return Ok(());
        }
        let response = self
            .response
            .clone()
            .ok_or_else(|| protocol("event before response creation"))?;
        if event
            .get("response_id")
            .and_then(Value::as_str)
            .is_some_and(|id| id != response)
        {
            return Err(protocol("response identity changed"));
        }
        match kind.as_str() {
            "response.in_progress" => {
                if string(&event["response"], "id")? != response {
                    return Err(protocol("response identity changed"));
                }
            }
            "response.output_item.added" => {
                let position = index(&event, "output_index")?;
                let item = &event["item"];
                let id = string(item, "id")?;
                if id.is_empty() || !self.ids.insert(id) || self.outputs.contains_key(&position) {
                    return Err(protocol("duplicate output item identity/index"));
                }
                match item.get("type").and_then(Value::as_str) {
                    Some("message")
                        if item.get("role").and_then(Value::as_str) == Some("assistant") => {}
                    Some("function_call") => {
                        let call = string(item, "call_id")?;
                        if call.is_empty()
                            || !self.calls.insert(call)
                            || string(item, "name")?.is_empty()
                        {
                            return Err(protocol("duplicate or empty tool identity"));
                        }
                    }
                    Some("reasoning") => (),
                    _ => return Err(protocol("unsupported stream output item")),
                }
                self.outputs.insert(
                    position,
                    Output {
                        added: item.clone(),
                        ..Output::default()
                    },
                );
            }
            "response.completed" => {
                let terminal = &event["response"];
                if string(terminal, "id")? != response {
                    return Err(protocol("terminal response identity changed"));
                }
                let items = terminal["output"]
                    .as_array()
                    .ok_or_else(|| protocol("missing terminal output"))?;
                if items.len() != self.outputs.len() {
                    return Err(protocol("terminal output count mismatch"));
                }
                for (position, item) in items.iter().enumerate() {
                    if self
                        .outputs
                        .get(&position)
                        .and_then(|output| output.done.as_ref())
                        != Some(item)
                    {
                        return Err(protocol("terminal output differs from acknowledged stream"));
                    }
                }
                self.terminal = Some(
                    super::decode(
                        &serde_json::to_vec(terminal).map_err(|_| protocol("invalid terminal"))?,
                    )
                    .map_err(Failure::Protocol)?,
                );
            }
            "error" | "response.failed" | "response.incomplete" => {
                return Err(Failure::Provider(
                    "provider stream failed/incomplete; no retry".into(),
                ));
            }
            "response.content_part.added"
            | "response.output_text.delta"
            | "response.output_text.done"
            | "response.content_part.done"
            | "response.function_call_arguments.delta"
            | "response.function_call_arguments.done"
            | "response.output_item.done" => {
                let position = index(&event, "output_index")?;
                let output = self
                    .outputs
                    .get_mut(&position)
                    .ok_or_else(|| protocol("unknown output index"))?;
                if output.done.is_some() {
                    return Err(protocol("event after output item done"));
                }
                let item_id = if kind == "response.output_item.done" {
                    string(&event["item"], "id")?
                } else {
                    string(&event, "item_id")?
                };
                if output.added["id"].as_str() != Some(&item_id) {
                    return Err(protocol("output item identity changed"));
                }
                match kind.as_str() {
                    "response.content_part.added" => {
                        let part = index(&event, "content_index")?;
                        if output.added["type"] != "message"
                            || event["part"]["type"] != "output_text"
                            || output.parts.insert(part, String::new()).is_some()
                        {
                            return Err(protocol("invalid or duplicate text part"));
                        }
                    }
                    "response.output_text.delta"
                    | "response.output_text.done"
                    | "response.content_part.done" => {
                        let part = index(&event, "content_index")?;
                        let text = output
                            .parts
                            .get_mut(&part)
                            .ok_or_else(|| protocol("unknown text part"))?;
                        if kind == "response.output_text.delta" {
                            if output.closed_parts.contains(&part) {
                                return Err(protocol("delta after text done"));
                            }
                            let delta = string(&event, "delta")?;
                            text.push_str(&delta);
                            sink.deliver(TextDelta {
                                response_id: response,
                                item_id,
                                sequence,
                                output_index: position,
                                content_index: part,
                                text: delta,
                            })
                            .await?;
                        } else if kind == "response.output_text.done" {
                            if !output.closed_parts.insert(part) || string(&event, "text")? != *text
                            {
                                return Err(protocol("text done differs from deltas"));
                            }
                        } else if !output.finished_parts.insert(part)
                            || !output.closed_parts.contains(&part)
                            || event["part"]["type"] != "output_text"
                            || string(&event["part"], "text")? != *text
                        {
                            return Err(protocol("content done differs from deltas"));
                        }
                    }
                    "response.function_call_arguments.delta" => {
                        if output.added["type"] != "function_call" || output.arguments_done {
                            return Err(protocol("invalid arguments delta"));
                        }
                        output.arguments.push_str(&string(&event, "delta")?);
                    }
                    "response.function_call_arguments.done" => {
                        if output.added["type"] != "function_call"
                            || output.arguments_done
                            || string(&event, "arguments")? != output.arguments
                        {
                            return Err(protocol("arguments done differs from deltas"));
                        }
                        output.arguments_done = true;
                    }
                    "response.output_item.done" => {
                        let item = &event["item"];
                        if item["type"] != output.added["type"] {
                            return Err(protocol("output type changed"));
                        }
                        if item["type"] == "function_call" {
                            if !output.arguments_done
                                || item["call_id"] != output.added["call_id"]
                                || item["name"] != output.added["name"]
                                || string(item, "arguments")? != output.arguments
                            {
                                return Err(protocol("tool completion identity/arguments changed"));
                            }
                        } else if item["type"] == "message" {
                            if item["role"] != "assistant" {
                                return Err(protocol("message role changed"));
                            }
                            let parts = item["content"]
                                .as_array()
                                .ok_or_else(|| protocol("missing message content"))?;
                            if parts.len() != output.parts.len() {
                                return Err(protocol("message part count changed"));
                            }
                            for (part, value) in parts.iter().enumerate() {
                                if !output.finished_parts.contains(&part)
                                    || output.parts.get(&part) != Some(&string(value, "text")?)
                                    || value["type"] != "output_text"
                                {
                                    return Err(protocol("message differs from streamed text"));
                                }
                            }
                        }
                        output.done = Some(item.clone());
                    }
                    _ => unreachable!(),
                }
            }
            // Reasoning is not public prose. Preserve its terminal opaque item,
            // but refuse unfamiliar event types rather than silently losing data.
            "response.reasoning_summary_part.added"
            | "response.reasoning_summary_text.delta"
            | "response.reasoning_summary_text.done"
            | "response.reasoning_summary_part.done" => {
                let output = self
                    .outputs
                    .get(&index(&event, "output_index")?)
                    .ok_or_else(|| protocol("unknown reasoning output"))?;
                if output.done.is_some()
                    || output.added["type"] != "reasoning"
                    || string(&event, "item_id")? != string(&output.added, "id")?
                {
                    return Err(protocol("invalid reasoning item identity"));
                }
            }
            _ => return Err(protocol("unsupported Responses stream event")),
        }
        Ok(())
    }
    pub fn finish(self) -> Result<Turn, Failure> {
        self.terminal
            .ok_or_else(|| protocol("stream ended without completed response"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragmented_utf8_crlf_multiline_and_comments() {
        let bytes = ": note\r\nevent: test\r\ndata: {\"value\":\r\ndata: \"é\"}\r\n\r\n".as_bytes();
        let mut decoder = Decoder::new(bytes.len());
        let mut frames = Vec::new();
        for byte in bytes {
            frames.extend(decoder.push(&[*byte]).unwrap());
        }
        decoder.finish().unwrap();
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].event.as_deref(), Some("test"));
        let value: Value = serde_json::from_str(&frames[0].data).unwrap();
        assert_eq!(value["value"], "é");
    }

    #[test]
    fn malformed_utf8_truncated_frames_and_bounds_are_rejected() {
        let mut decoder = Decoder::new(100);
        assert!(decoder.push(b"data: \xff\n\n").is_err());
        let mut decoder = Decoder::new(100);
        decoder.push(b"data: {}\n").unwrap();
        assert!(decoder.finish().is_err());
        let mut decoder = Decoder::new(4);
        assert!(decoder.push(b"12345").is_err());
    }
}
