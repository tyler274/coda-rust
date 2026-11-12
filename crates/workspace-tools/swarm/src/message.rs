use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::mention::{Mention, parse_mentions};
use crate::names::{ChannelName, MemberName};
use crate::roster::Topic;

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

/// A channel change recorded on the board. Replaying these rows in seq
/// order is the only source of channel state and subscriptions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum Control {
    Create {
        channel: ChannelName,
        topic: Topic,
    },
    Archive {
        channel: ChannelName,
    },
    Subscribe {
        channel: ChannelName,
        members: Vec<MemberName>,
    },
    Unsubscribe {
        channel: ChannelName,
        members: Vec<MemberName>,
    },
}

impl Control {
    /// The row's text, so a reader of the board sees what changed.
    pub fn describe(&self) -> String {
        let list = |members: &[MemberName]| {
            members
                .iter()
                .map(MemberName::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        };
        match self {
            Self::Create { channel, topic } => match topic.as_str() {
                "" => format!("[channel] created #{channel}"),
                topic => format!("[channel] created #{channel}: {topic}"),
            },
            Self::Archive { channel } => format!("[channel] archived #{channel}"),
            Self::Subscribe { channel, members } => {
                format!("[channel] subscribed {} to #{channel}", list(members))
            }
            Self::Unsubscribe { channel, members } => {
                format!("[channel] unsubscribed {} from #{channel}", list(members))
            }
        }
    }
}

/// What a board row carries: a post of some kind, or a channel change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Post(MessageKind),
    Control(Control),
}

/// A caller-chosen key that makes a retried post land once.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestId(pub String);

/// One line of `board.jsonl`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Line", into = "Line")]
pub struct Message {
    pub seq: Seq,
    pub agent: MemberName,
    pub text: String,
    pub time: DateTime<Utc>,
    pub body: Body,
    pub request_id: Option<RequestId>,
    pub channel: ChannelName,
}

impl Message {
    pub fn mentions(&self) -> Vec<Mention> {
        parse_mentions(&self.text)
    }
}

/// The wire shape of a board line: `kind` and `control` are separate
/// optional fields, and at most one of them is set.
#[derive(Serialize, Deserialize)]
struct Line {
    seq: Seq,
    agent: MemberName,
    text: String,
    time: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "MessageKind::is_post")]
    kind: MessageKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    request_id: Option<RequestId>,
    #[serde(default = "ChannelName::main")]
    channel: ChannelName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    control: Option<Control>,
}

#[derive(Debug, thiserror::Error)]
#[error("a control row cannot also have kind {0:?}")]
pub struct KindedControl(MessageKind);

impl TryFrom<Line> for Message {
    type Error = KindedControl;
    fn try_from(line: Line) -> Result<Self, KindedControl> {
        let body = match (line.kind, line.control) {
            (kind, None) => Body::Post(kind),
            (MessageKind::Post, Some(control)) => Body::Control(control),
            (kind, Some(_)) => return Err(KindedControl(kind)),
        };
        Ok(Self {
            seq: line.seq,
            agent: line.agent,
            text: line.text,
            time: line.time,
            body,
            request_id: line.request_id,
            channel: line.channel,
        })
    }
}

impl From<Message> for Line {
    fn from(message: Message) -> Self {
        let (kind, control) = match message.body {
            Body::Post(kind) => (kind, None),
            Body::Control(control) => (MessageKind::Post, Some(control)),
        };
        Self {
            seq: message.seq,
            agent: message.agent,
            text: message.text,
            time: message.time,
            kind,
            request_id: message.request_id,
            channel: message.channel,
            control,
        }
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
            body: Body::Post(MessageKind::Post),
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
        assert_eq!(message.body, Body::Post(MessageKind::Brief));
    }

    #[test]
    fn control_rows_carry_an_op_and_no_kind() {
        let line = r#"{"seq":2,"agent":"lead","text":"t","time":"2027-01-15T08:00:00Z","control":{"op":"subscribe","channel":"ops","members":["scout"]}}"#;
        let message: Message = serde_json::from_str(line).expect("decodes");
        assert_eq!(
            message.body,
            Body::Control(Control::Subscribe {
                channel: ChannelName::parse("ops").expect("valid"),
                members: vec![MemberName::parse("scout").expect("valid")],
            })
        );
        let encoded = serde_json::to_value(&message).expect("encodes");
        assert_eq!(encoded["control"]["op"], "subscribe");
        assert!(encoded.get("kind").is_none());
        let kinded = line.replace(r#""control""#, r#""kind":"brief","control""#);
        assert!(serde_json::from_str::<Message>(&kinded).is_err());
    }
}
