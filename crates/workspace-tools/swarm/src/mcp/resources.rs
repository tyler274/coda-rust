use serde_json::{Value, json};

/// The MCP Apps media type for an HTML view (SEP-1865).
pub const APP_MIME_TYPE: &str = "text/html;profile=mcp-app";

/// Every resource the server serves. `resources/read` parses its URI into one
/// of these, so content is chosen by type, never by catalog position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    /// The live board view that renders `board_view` results.
    Board,
}

impl Resource {
    pub const ALL: [Self; 1] = [Self::Board];

    pub fn uri(self) -> &'static str {
        match self {
            Self::Board => "ui://coda-swarm/board",
        }
    }

    pub fn parse(uri: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|resource| resource.uri() == uri)
    }

    fn text(self) -> &'static str {
        match self {
            Self::Board => include_str!("board.html"),
        }
    }

    pub fn descriptor(self) -> Value {
        match self {
            Self::Board => json!({
                "uri": self.uri(),
                "name": "swarm_board",
                "title": "Swarm board",
                "description": "Live view of the swarm: members with state and model, channels, and recent messages.",
                "mimeType": APP_MIME_TYPE,
            }),
        }
    }

    /// The `resources/read` result. The view loads nothing from the network,
    /// so it declares no CSP domains and gets the host's restrictive default.
    pub fn contents(self) -> Value {
        json!({
            "contents": [{
                "uri": self.uri(),
                "mimeType": APP_MIME_TYPE,
                "text": self.text(),
                "_meta": { "ui": { "prefersBorder": true } },
            }]
        })
    }
}
