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
    rsx! { physician_app::PhysicianRoot { base_url: base, open_patient } }
}

fn open_patient() -> String {
    #[cfg(target_arch = "wasm32")]
    if let Some(id) = query_value("patient") {
        return id;
    }
    String::new()
}

fn api_base() -> String {
    #[cfg(target_arch = "wasm32")]
    if let Some(url) = query_api() {
        return url;
    }
    if let Some(url) = option_env!("PHYSICIAN_API_URL") {
        return url.to_owned();
    }
    #[cfg(target_arch = "wasm32")]
    if let Some(origin) = page_origin() {
        return origin;
    }
    "http://127.0.0.1:8790".to_owned()
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
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(
                std::str::from_utf8(&bytes[index + 1..index + 3]).unwrap_or(""),
                16,
            ) {
                out.push(byte);
                index += 3;
                continue;
            }
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
