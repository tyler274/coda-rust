use serde_json::{Value, json};

use super::resources::Resource;

/// Every tool the server advertises. `tools/call` parses its name into one of
/// these, so a handler is chosen by type, never by catalog position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Swarm,
    PostMessage,
    ReadBoard,
    SwarmWait,
    BoardView,
}

impl Tool {
    pub const ALL: [Self; 5] = [
        Self::Swarm,
        Self::PostMessage,
        Self::ReadBoard,
        Self::SwarmWait,
        Self::BoardView,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Swarm => "swarm",
            Self::PostMessage => "post_message",
            Self::ReadBoard => "read_board",
            Self::SwarmWait => "swarm_wait",
            Self::BoardView => "board_view",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|tool| tool.name() == name)
    }

    fn description(self) -> &'static str {
        match self {
            Self::Swarm => {
                "Manage this swarm. `action` picks the operation: status (roster and board head), \
                 models (models `add` accepts), add (new member or group `name-1..n`, posting its \
                 brief as you), model (switch a member's model, default yourself), kill (retire a \
                 member or a group), title (coordinator only), channel (create, archive, \
                 subscribe, unsubscribe or list channels), join (not supported yet). Workers may \
                 kill or re-model only the helpers they added."
            }
            Self::PostMessage => {
                "Post to the swarm board. Without `channel` the post goes to your home channel. \
                 Address members with @name, a group with @base, everyone with @all, or a \
                 channel with @#channel. Returns the new seq."
            }
            Self::ReadBoard => {
                "Read board messages in your view (main, your channels, and posts that mention \
                 you). Without `since`, returns the newest `limit`; with `since`, the oldest \
                 after it, for paging forward. Advances your read cursor."
            }
            Self::SwarmWait => {
                "Park until a post wakes you, then return every message in your view after \
                 `since`. Pass the highest seq you have read. By default only a post that \
                 mentions you wakes you; set mentions_only=false to wake for any visible post. \
                 Returns woken=false after `timeout` seconds."
            }
            Self::BoardView => {
                "Snapshot of the whole swarm for display: title, members with state and model, \
                 channels, recent messages on every channel, and the head seq. Hosts with MCP \
                 Apps show it as a live board. Does not move your read cursor."
            }
        }
    }

    pub fn input_schema(self) -> Value {
        match self {
            Self::Swarm => object(
                json!({
                    "action": {
                        "type": "string",
                        "enum": ["status", "models", "add", "model", "kill", "title", "channel", "join"]
                    },
                    "name": {
                        "type": "string",
                        "description": "Member name. add: the new member or group base. model: whose model (default you). kill: member or group base. channel: who subscribes (default you)."
                    },
                    "brief": { "type": "string", "description": "add: the task brief posted to the new member." },
                    "model": { "type": "string", "description": "add, model: model name (default your model)." },
                    "workspace": {
                        "type": "string",
                        "enum": ["inherit", "share", "branch"],
                        "description": "add: inherit your checkout (default), or give each member its own worktree (share) or clone (branch)."
                    },
                    "channels": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "add: channels the new member subscribes to; the first is its home."
                    },
                    "n": { "type": "integer", "minimum": 1, "description": "add: number of members in the group." },
                    "tasks": {
                        "type": "array",
                        "items": { "type": "string" },
                        "minItems": 1,
                        "description": "add: one task per group member, appended to the shared brief."
                    },
                    "title": { "type": "string", "description": "title: the swarm's display title." },
                    "op": {
                        "type": "string",
                        "enum": ["create", "archive", "subscribe", "unsubscribe", "list"],
                        "description": "channel: the channel operation."
                    },
                    "channel": { "type": "string", "description": "channel: the channel name." },
                    "topic": { "type": "string", "description": "channel create: the channel's topic." },
                    "swarm": { "type": "string", "description": "join: the swarm id." },
                    "brief_seq": { "type": "integer", "minimum": 1, "description": "join: seq of the brief to start from." }
                }),
                &["action"],
            ),
            Self::PostMessage => object(
                json!({
                    "text": { "type": "string", "minLength": 1 },
                    "channel": { "type": "string", "description": "Target channel (default your home channel)." }
                }),
                &["text"],
            ),
            Self::ReadBoard => object(
                json!({
                    "since": { "type": "integer", "minimum": 0, "description": "Return messages after this seq." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": MAX_LIMIT, "default": DEFAULT_LIMIT }
                }),
                &[],
            ),
            Self::SwarmWait => object(
                json!({
                    "since": { "type": "integer", "minimum": 0, "description": "The highest seq you have read." },
                    "mentions_only": { "type": "boolean", "default": true },
                    "timeout": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": MAX_WAIT_SECS,
                        "default": DEFAULT_WAIT_SECS,
                        "description": "Seconds to wait before returning woken=false."
                    }
                }),
                &["since"],
            ),
            Self::BoardView => object(
                json!({
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": MAX_LIMIT,
                        "default": DEFAULT_VIEW_LIMIT,
                        "description": "How many recent messages to include."
                    }
                }),
                &[],
            ),
        }
    }

    /// The MCP Apps view that renders this tool's results, if any.
    pub fn view(self) -> Option<Resource> {
        match self {
            Self::BoardView => Some(Resource::Board),
            Self::Swarm | Self::PostMessage | Self::ReadBoard | Self::SwarmWait => None,
        }
    }

    pub fn descriptor(self) -> Value {
        let mut descriptor = json!({
            "name": self.name(),
            "description": self.description(),
            "inputSchema": self.input_schema(),
        });
        if let Some(view) = self.view() {
            descriptor["_meta"] = json!({
                "ui": { "resourceUri": view.uri(), "visibility": ["model", "app"] }
            });
        }
        descriptor
    }
}

pub const DEFAULT_LIMIT: usize = 20;
pub const DEFAULT_VIEW_LIMIT: usize = 50;
pub const MAX_LIMIT: usize = 500;
pub const DEFAULT_WAIT_SECS: u64 = 120;
pub const MAX_WAIT_SECS: u64 = 3600;

fn object(properties: Value, required: &[&str]) -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": properties,
        "required": required,
        "additionalProperties": false,
    })
}
