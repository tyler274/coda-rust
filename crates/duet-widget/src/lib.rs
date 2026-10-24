//! Snapshot the native home-screen widgets read. The widget extensions do not
//! compute a second copy of these lines.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

/// The `FILE_NAME` constant.
pub const FILE_NAME: &str = "widget-snapshot.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// The `WidgetSnapshot` value exchanged by this crate.
pub struct WidgetSnapshot {
    /// The `generated_at` value.
    pub generated_at: String,
    /// The `lines` value.
    pub lines: Vec<String>,
}

/// Encodes the widget snapshot.
pub fn to_json(snapshot: &WidgetSnapshot) -> String {
    serde_json::to_string(snapshot).expect("widget snapshot is json")
}

/// Decodes the widget snapshot.
pub fn from_json(text: &str) -> Result<WidgetSnapshot, serde_json::Error> {
    serde_json::from_str(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_lines() {
        let snapshot = WidgetSnapshot {
            generated_at: "2026-10-09T00:00:00Z".into(),
            lines: vec!["Quiet".into()],
        };
        let read = from_json(&to_json(&snapshot)).unwrap();
        assert_eq!(read.lines, vec!["Quiet".to_owned()]);
    }
}
