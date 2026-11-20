//! Android and iOS mobile shell for Duet.
//!
//! Duet has no doctor accounts and exposes no runtime Physician/Duet mode switcher.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]

use dioxus::prelude::*;

/// Renders the Duet application directly without any runtime Physician/Duet mode toggle.
#[component]
pub fn ModeShell(
    #[props(default)] physician_url: String,
    duet_url: String,
    duet_bearer: String,
) -> Element {
    let _ = physician_url;
    rsx! {
        ui::Stylesheet {}
        duet_app::DuetRoot {
            base_url: duet_base_url(duet_url),
            bearer: duet_bearer,
        }
    }
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

/// The web build can point the shell at a loopback origin with `?api=`.
/// Native builds keep their compile-time or default origins. Any other
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

#[cfg(test)]
mod tests {
    use super::*;

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
    }
}
