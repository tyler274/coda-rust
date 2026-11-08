//! The physician's remembered panel choices: whether the sidebar and the
//! schedule are collapsed. They live in this browser's localStorage under the
//! physician's id while signed in, and sign-out or a session end removes them.
// Storage is browser-only; native builds read every panel as expanded.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

/// A panel whose collapse is remembered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Panel {
    Sidebar,
    Schedule,
}

fn key(physician_id: &str, panel: Panel) -> String {
    let name = match panel {
        Panel::Sidebar => "sidebar",
        Panel::Schedule => "schedule",
    };
    format!("coda.physician.{physician_id}.{name}-collapsed")
}

/// Whether this physician last left the panel collapsed. Nothing remembered reads as expanded.
#[cfg(target_arch = "wasm32")]
pub(crate) fn collapsed(physician_id: &str, panel: Panel) -> bool {
    storage()
        .and_then(|storage| storage.get_item(&key(physician_id, panel)).ok().flatten())
        .is_some_and(|value| value == "true")
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn collapsed(_physician_id: &str, _panel: Panel) -> bool {
    false
}

/// Remembers this physician's choice for the panel.
#[cfg(target_arch = "wasm32")]
pub(crate) fn remember(physician_id: &str, panel: Panel, collapsed: bool) {
    if let Some(storage) = storage() {
        let _ = storage.set_item(
            &key(physician_id, panel),
            if collapsed { "true" } else { "false" },
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn remember(_physician_id: &str, _panel: Panel, _collapsed: bool) {}

/// Removes every physician's panel choices, so the browser no longer names
/// anyone. A tab whose session ended before it learned who it was still clears them.
#[cfg(target_arch = "wasm32")]
pub(crate) fn forget_all() {
    let Some(storage) = storage() else { return };
    let length = storage.length().unwrap_or(0);
    let keys: Vec<String> = (0..length)
        .filter_map(|index| storage.key(index).ok().flatten())
        .filter(|key| is_panel_key(key))
        .collect();
    for key in keys {
        let _ = storage.remove_item(&key);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn forget_all() {}

fn is_panel_key(key: &str) -> bool {
    key.starts_with("coda.physician.")
        && (key.ends_with(".sidebar-collapsed") || key.ends_with(".schedule-collapsed"))
}

#[cfg(test)]
mod tests {
    use super::{Panel, is_panel_key, key};

    #[test]
    fn panel_keys_are_recognised_and_others_are_not() {
        assert!(is_panel_key(&key("staging-physician", Panel::Sidebar)));
        assert!(is_panel_key(&key("other", Panel::Schedule)));
        assert!(!is_panel_key("coda.physician.session"));
        assert!(!is_panel_key("coda.physician.session.email"));
        assert!(!is_panel_key("coda.physician.draft.v1|x"));
    }
}

#[cfg(target_arch = "wasm32")]
fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}
