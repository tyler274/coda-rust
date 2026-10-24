//! The physician's remembered panel choices: whether the sidebar and the
//! schedule are collapsed. They belong to the physician, not the session, so
//! they live in this browser's localStorage under the physician's id and
//! outlive sign-out (physician.mmd, {panel} preference, M8).
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

#[cfg(target_arch = "wasm32")]
fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}
