// Without `web` or `desktop` the binary only explains how to build it, so the
// address helpers below have no caller.
#![cfg_attr(not(any(feature = "web", feature = "desktop")), allow(dead_code))]

#[cfg(any(feature = "web", feature = "desktop"))]
use dioxus::prelude::*;

fn main() {
    #[cfg(any(feature = "web", feature = "desktop"))]
    dioxus::launch(App);
    #[cfg(not(any(feature = "web", feature = "desktop")))]
    {
        eprintln!("Build physician-app with --features web or --features desktop.");
        std::process::exit(1);
    }
}

#[cfg(any(feature = "web", feature = "desktop"))]
#[component]
fn App() -> Element {
    let base = api_base();
    let open_patient = open_patient();
    let open_chat = open_chat();
    let sign_in = use_hook(sign_in_link);
    let preload = preload_trigger();
    rsx! { physician_app::PhysicianRoot { base_url: base, open_patient, open_chat, sign_in, preload } }
}

/// `?preload=click`, `hover` or `predict` chooses what starts reading a
/// patient before it opens, so the triggers can be compared on one build.
#[cfg(any(feature = "web", feature = "desktop"))]
fn preload_trigger() -> physician_app::PreloadTrigger {
    #[cfg(target_arch = "wasm32")]
    if let Some(trigger) =
        query_value("preload").and_then(|value| physician_app::PreloadTrigger::parse(&value))
    {
        return trigger;
    }
    physician_app::PreloadTrigger::default()
}

#[cfg(all(any(feature = "web", feature = "desktop"), target_arch = "wasm32"))]
fn sign_in_link() -> Option<physician_app::SignInLink> {
    let email = query_value("email")?;
    let code = query_value("code")?;
    // Keep the secret in component state, not the address bar or a later reload.
    // The sign-in screen redeems it automatically after the URL is cleared.
    if let Some(window) = web_sys::window()
        && let Ok(href) = window.location().href()
        && let Ok(url) = web_sys::Url::new(&href)
    {
        url.search_params().delete("email");
        url.search_params().delete("code");
        if let Ok(history) = window.history() {
            let _ =
                history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url.href()));
        }
    }
    Some(physician_app::SignInLink { email, code })
}

/// Sign-in links arrive in the page address, so only the browser build reads one.
#[cfg(all(any(feature = "web", feature = "desktop"), not(target_arch = "wasm32")))]
fn sign_in_link() -> Option<physician_app::SignInLink> {
    None
}

// The query-parameter and origin lookups are wasm-only; on other targets the
// fallbacks below are the whole function.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn open_patient() -> String {
    #[cfg(target_arch = "wasm32")]
    if let Some(id) = query_value("patient") {
        return id;
    }
    String::new()
}

/// `?chat=` opens that conversation of the `?patient=` workspace, including
/// one the workspace does not list yet (a chat started outside this page).
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn open_chat() -> String {
    #[cfg(target_arch = "wasm32")]
    if let Some(id) = query_value("chat") {
        return id;
    }
    String::new()
}

// See open_patient: the wasm branches compile out on other targets.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn api_base() -> String {
    #[cfg(target_arch = "wasm32")]
    let query = query_api();
    #[cfg(not(target_arch = "wasm32"))]
    let query: Option<String> = None;
    #[cfg(target_arch = "wasm32")]
    let page = web_sys::window()
        .and_then(|window| Some((page_origin()?, window.location().pathname().ok()?)));
    #[cfg(not(target_arch = "wasm32"))]
    let page: Option<(String, String)> = None;
    resolve_api_base(
        query.as_deref(),
        option_env!("PHYSICIAN_API_URL"),
        page.as_ref()
            .map(|(origin, path)| (origin.as_str(), path.as_str())),
    )
}

fn resolve_api_base(
    query: Option<&str>,
    configured: Option<&str>,
    page: Option<(&str, &str)>,
) -> String {
    // `query` is already percent-decoded. Only a loopback http(s) URL may
    // replace the compile-time URL or the page origin.
    if let Some(url) = query.filter(|url| is_loopback_api_url(url)) {
        return url.to_owned();
    }
    if let Some(url) = configured {
        return url.to_owned();
    }
    if let Some((origin, path)) = page {
        // Only the declared mount is special, not arbitrary prefixes such as /physicians.
        let prefix = if path == "/physician" || path.starts_with("/physician/") {
            "/physician"
        } else {
            ""
        };
        return format!("{origin}{prefix}");
    }
    "http://127.0.0.1:8790".to_owned()
}

/// `http` or `https` whose host is `localhost` or `127.0.0.1`, with no userinfo.
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

#[cfg(target_arch = "wasm32")]
fn page_origin() -> Option<String> {
    let origin = web_sys::window()?.location().origin().ok()?;
    if origin.is_empty() || origin == "null" {
        None
    } else {
        Some(origin)
    }
}

#[cfg(target_arch = "wasm32")]
fn query_api() -> Option<String> {
    query_value("api")
}

#[cfg(target_arch = "wasm32")]
fn query_value(name: &str) -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    for pair in search.trim_start_matches('?').split('&') {
        let (key, value) = pair.split_once('=')?;
        if key == name && !value.is_empty() {
            return Some(percent_decode(value));
        }
    }
    None
}

#[cfg(target_arch = "wasm32")]
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

#[cfg(test)]
mod tests {
    use super::resolve_api_base;

    #[test]
    fn runtime_mount_is_segment_bounded() {
        for path in ["/physician", "/physician/", "/physician/index.html"] {
            assert_eq!(
                resolve_api_base(None, None, Some(("https://coda.test", path))),
                "https://coda.test/physician"
            );
        }
        for path in ["/", "/index.html", "/physicians", "/physician-other/"] {
            assert_eq!(
                resolve_api_base(None, None, Some(("https://coda.test", path))),
                "https://coda.test"
            );
        }
    }

    #[test]
    fn explicit_api_overrides_are_not_prefixed() {
        let page = Some(("https://coda.test", "/physician/"));
        assert_eq!(
            resolve_api_base(
                Some("http://localhost:8790/"),
                Some("https://configured.test/api"),
                page
            ),
            "http://localhost:8790/"
        );
        assert_eq!(
            resolve_api_base(None, Some("https://configured.test/api"), page),
            "https://configured.test/api"
        );
        assert_eq!(resolve_api_base(None, None, None), "http://127.0.0.1:8790");
    }

    #[test]
    fn query_api_override_is_loopback_only() {
        let page = Some(("https://coda.test", "/physician/"));
        let configured = Some("https://configured.test/api");
        for url in [
            "http://localhost:8790/",
            "https://localhost/physician",
            "http://127.0.0.1:8790",
            "HTTP://LOCALHOST",
        ] {
            assert_eq!(
                resolve_api_base(Some(url), configured, page),
                url,
                "{url} is loopback"
            );
        }
        for url in [
            "https://evil.example/api",
            "http://user:pass@127.0.0.1:8790/",
            "http://user@localhost",
            "http://localhost.evil.com",
            "http://127.0.0.1.evil.com",
            "http://2130706433",
            "javascript:alert(1)",
            "http://[::1]:8790/",
            "http://127.0.0.1\\@evil.com",
        ] {
            assert_eq!(
                resolve_api_base(Some(url), configured, page),
                "https://configured.test/api",
                "{url} must be ignored"
            );
            assert_eq!(
                resolve_api_base(Some(url), None, page),
                "https://coda.test/physician",
                "{url} falls through to the page origin"
            );
        }
    }
}
