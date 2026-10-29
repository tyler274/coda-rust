//! One Android and iOS shell with an explicit physician or client mode.
//!
//! The product crates stay separate. This shell is the only crate that mounts both.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]

use std::path::Path;

use dioxus::prelude::*;
use ui::{ActionRow, Button, ButtonVariant};

#[cfg(target_arch = "wasm32")]
const MODE_KEY: &str = "coda-mode";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Physician,
    Client,
}

impl Mode {
    fn as_str(self) -> &'static str {
        match self {
            Mode::Physician => "physician",
            Mode::Client => "client",
        }
    }
}

/// Reads a stored mode name. Anything else is no choice.
pub fn parse_mode(value: &str) -> Option<&'static str> {
    match value.trim() {
        "physician" => Some("physician"),
        "client" => Some("client"),
        _ => None,
    }
}

/// Writes the mode where this process keeps device choices.
pub fn remember_mode(path: &Path, mode: &str) {
    if parse_mode(mode).is_some() {
        let _ = std::fs::write(path, mode);
    }
}

/// Reads the mode file. A missing or unknown value is no choice.
pub fn recall_mode(path: &Path) -> Option<&'static str> {
    std::fs::read_to_string(path)
        .ok()
        .as_deref()
        .and_then(parse_mode)
}

/// Clears the stored mode so the next launch asks again.
pub fn forget_mode(path: &Path) {
    let _ = std::fs::remove_file(path);
}

#[component]
pub fn ModeShell(physician_url: String, duet_url: String, duet_bearer: String) -> Element {
    let mut mode = use_signal(initial_mode);
    rsx! {
        style { "{ui::STYLESHEET}" }
        match mode() {
            None => rsx! {
                main { class: "mode-chooser", aria_label: "Choose a mode",
                    h1 { "Coda" }
                    ActionRow {
                        Button {
                            onclick: move |_| {
                                persist(Mode::Physician);
                                mode.set(Some(Mode::Physician));
                            },
                            "Physician"
                        }
                        Button {
                            onclick: move |_| {
                                persist(Mode::Client);
                                mode.set(Some(Mode::Client));
                            },
                            "Client"
                        }
                    }
                }
            },
            Some(Mode::Physician) => rsx! {
                Button {
                    variant: ButtonVariant::Quiet,
                    compact: true,
                    class: "mode-switch",
                    onclick: move |_| {
                        clear_persisted();
                        mode.set(None);
                    },
                    "Switch mode"
                }
                physician_app::PhysicianRoot {
                    base_url: physician_base_url(physician_url.clone()),
                    open_patient: String::new(),
                }
            },
            Some(Mode::Client) => rsx! {
                Button {
                    variant: ButtonVariant::Quiet,
                    compact: true,
                    class: "mode-switch",
                    onclick: move |_| {
                        clear_persisted();
                        mode.set(None);
                    },
                    "Switch mode"
                }
                duet_app::DuetRoot {
                    base_url: duet_base_url(duet_url.clone()),
                    bearer: duet_bearer.clone(),
                }
            },
        }
    }
}

fn initial_mode() -> Option<Mode> {
    if let Some(value) = query_mode().or_else(recall_device) {
        let mode = match value {
            "physician" => Mode::Physician,
            "client" => Mode::Client,
            _ => return None,
        };
        persist(mode);
        return Some(mode);
    }
    None
}

fn persist(mode: Mode) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = web_storage() {
            let _ = storage.set_item(MODE_KEY, mode.as_str());
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    remember_mode(&mode_path(), mode.as_str());
}

fn clear_persisted() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = web_storage() {
            let _ = storage.remove_item(MODE_KEY);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    forget_mode(&mode_path());
}

fn recall_device() -> Option<&'static str> {
    #[cfg(target_arch = "wasm32")]
    {
        return web_storage()
            .and_then(|storage| storage.get_item(MODE_KEY).ok().flatten())
            .as_deref()
            .and_then(parse_mode);
    }
    #[cfg(not(target_arch = "wasm32"))]
    recall_mode(&mode_path())
}

#[cfg(not(target_arch = "wasm32"))]
fn mode_path() -> std::path::PathBuf {
    std::env::var_os("CODA_MODE_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("coda-mode"))
}

#[cfg(target_arch = "wasm32")]
fn query_mode() -> Option<&'static str> {
    let search = web_sys::window()?.location().search().ok()?;
    for pair in search.trim_start_matches('?').split('&') {
        let (key, value) = pair.split_once('=')?;
        if key == "mode" {
            return parse_mode(value);
        }
    }
    None
}

#[cfg(target_arch = "wasm32")]
fn query_override(key: &str) -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    for pair in search.trim_start_matches('?').split('&') {
        let (name, value) = pair.split_once('=')?;
        if name == key && !value.is_empty() {
            return Some(value.to_owned());
        }
    }
    None
}

/// The web build can point either shell at a loopback origin with `?api=`.
/// The native builds keep their compile-time or default origins. Any other
/// query value is ignored.
pub fn duet_base_url(compile_time: String) -> String {
    #[cfg(target_arch = "wasm32")]
    if let Some(url) = query_override("api")
        .as_deref()
        .and_then(loopback_api_override)
    {
        return url;
    }
    compile_time
}

/// The same override for the physician shell.
pub fn physician_base_url(compile_time: String) -> String {
    #[cfg(target_arch = "wasm32")]
    if let Some(url) = query_override("api")
        .as_deref()
        .and_then(loopback_api_override)
    {
        return url;
    }
    compile_time
}

/// Accept a query override only when the URL the app will request is loopback.
///
/// `query_override` does not percent-decode. This decodes once and returns
/// that decoded URL, so the allowlist and the request see the same value.
pub fn loopback_api_override(raw: &str) -> Option<String> {
    let decoded = percent_decode(raw);
    is_loopback_api_url(&decoded).then_some(decoded)
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let Ok(byte) = u8::from_str_radix(
                std::str::from_utf8(&bytes[index + 1..index + 3]).unwrap_or(""),
                16,
            )
        {
            out.push(byte);
            index += 3;
            continue;
        }
        if bytes[index] == b'+' {
            out.push(b' ');
        } else {
            out.push(bytes[index]);
        }
        index += 1;
    }
    String::from_utf8(out).unwrap_or_else(|_| value.to_owned())
}

fn is_loopback_api_url(value: &str) -> bool {
    let value = value.trim();
    let Some((scheme, rest)) = value.split_once("://") else {
        return false;
    };
    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return false;
    }
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
    if authority.is_empty()
        || authority.contains('@')
        || authority.contains('\\')
        || authority.chars().any(|c| c.is_control() || c == ' ')
    {
        return false;
    }
    let host = match authority.rsplit_once(':') {
        Some((host, port))
            if !host.is_empty() && !port.is_empty() && port.chars().all(|c| c.is_ascii_digit()) =>
        {
            if port.parse::<u16>().is_err() {
                return false;
            }
            host
        }
        Some(_) => return false,
        None => authority,
    };
    if host.contains(':') || host.contains('[') {
        return false;
    }
    host.eq_ignore_ascii_case("localhost") || host == "127.0.0.1"
}

#[cfg(not(target_arch = "wasm32"))]
fn query_mode() -> Option<&'static str> {
    None
}

#[cfg(target_arch = "wasm32")]
fn web_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_mode_file_round_trips_and_forgets() {
        let path = std::env::temp_dir().join(format!("coda-mode-{}", std::process::id()));
        remember_mode(&path, "physician");
        assert_eq!(recall_mode(&path), Some("physician"));
        remember_mode(&path, "nope");
        assert_eq!(recall_mode(&path), Some("physician"));
        forget_mode(&path);
        assert_eq!(recall_mode(&path), None);
        remember_mode(&path, "client");
        assert_eq!(recall_mode(&path), Some("client"));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn api_query_override_is_the_loopback_url_that_would_be_requested() {
        assert_eq!(
            loopback_api_override("http://127.0.0.1:8790/"),
            Some("http://127.0.0.1:8790/".to_owned())
        );
        assert_eq!(
            loopback_api_override("http%3A%2F%2Flocalhost%3A8790"),
            Some("http://localhost:8790".to_owned())
        );
        assert_eq!(loopback_api_override("http%3A%2F%2Fevil.example"), None);
        assert_eq!(loopback_api_override("http://user%40localhost:8790"), None);
        assert_eq!(loopback_api_override("https://localhost.evil.com"), None);
        assert_eq!(
            duet_base_url("http://127.0.0.1:8000".into()),
            "http://127.0.0.1:8000"
        );
        assert_eq!(
            physician_base_url("http://127.0.0.1:8790".into()),
            "http://127.0.0.1:8790"
        );
    }
}
