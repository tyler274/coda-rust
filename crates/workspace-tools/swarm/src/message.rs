use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::mention::{Mention, parse_mentions};
use crate::names::{ChannelName, MemberName};

/// A board position. The first message is seq 1; `Seq::ZERO` means "before
/// any message", the starting read cursor.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Seq(pub u64);

impl Seq {
    pub const ZERO: Self = Self(0);

    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Display for Seq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageKind {
    #[default]
    Post,
    /// The brief posted when a member is added.
    Brief,
}

impl MessageKind {
    fn is_post(&self) -> bool {
        *self == Self::Post
    }
}

/// A caller-chosen key that makes a retried post land once.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestId(pub String);

/// One line of `board.jsonl`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub seq: Seq,
    pub agent: MemberName,
    pub text: String,
    pub time: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "MessageKind::is_post")]
    pub kind: MessageKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestId>,
    #[serde(default = "ChannelName::main")]
    pub channel: ChannelName,
}

impl Message {
    pub fn mentions(&self) -> Vec<Mention> {
        parse_mentions(&self.text)
    }
}

/// A post before the store assigns its seq and time. Without a channel it
/// goes to the author's home channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Draft {
    pub author: MemberName,
    pub text: String,
    pub channel: Option<ChannelName>,
    pub kind: MessageKind,
    pub request_id: Option<RequestId>,
}

impl Draft {
    pub fn post(author: MemberName, text: impl Into<String>) -> Self {
        Self {
            author,
            text: text.into(),
            channel: None,
            kind: MessageKind::Post,
            request_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_line_round_trips_and_omits_defaults() {
        let message = Message {
            seq: Seq(3),
            agent: MemberName::parse("lead").expect("valid"),
            text: "hi".into(),
            time: DateTime::from_timestamp(1_800_000_000, 0).expect("valid"),
            kind: MessageKind::Post,
            request_id: None,
            channel: ChannelName::main(),
        };
        let line = serde_json::to_string(&message).expect("encodes");
        assert_eq!(
            line,
            r#"{"seq":3,"agent":"lead","text":"hi","time":"2027-01-15T08:00:00Z","channel":"main"}"#
        );
        assert_eq!(
            serde_json::from_str::<Message>(&line).expect("decodes"),
            message
        );
    }

    #[test]
    fn missing_channel_reads_as_main() {
        let message: Message = serde_json::from_str(
            r#"{"seq":1,"agent":"lead","text":"x","time":"2027-01-15T08:00:00Z","kind":"brief"}"#,
        )
        .expect("decodes");
        assert!(message.channel.is_main());
        assert_eq!(message.kind, MessageKind::Brief);
    }
}
