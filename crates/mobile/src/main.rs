#[cfg(any(feature = "web", feature = "mobile"))]
use dioxus::prelude::*;

fn main() {
    #[cfg(any(feature = "web", feature = "mobile"))]
    dioxus::launch(App);
    #[cfg(not(any(feature = "web", feature = "mobile")))]
    {
        eprintln!("Build mobile with --features web or --features mobile.");
        std::process::exit(1);
    }
}

#[cfg(any(feature = "web", feature = "mobile"))]
#[component]
fn App() -> Element {
    rsx! {
        mobile::ModeShell {
            physician_url: physician_url(),
            duet_url: duet_url(),
            duet_bearer: duet_app::api_bearer(),
        }
    }
}

#[cfg(any(feature = "web", feature = "mobile"))]
fn physician_url() -> String {
    if let Some(url) = option_env!("PHYSICIAN_API_URL")
        && !url.is_empty()
    {
        return url.to_owned();
    }
    #[cfg(target_arch = "wasm32")]
    if let Some(origin) = page_origin() {
        return origin;
    }
    "http://127.0.0.1:8790".to_owned()
}

#[cfg(any(feature = "web", feature = "mobile"))]
fn duet_url() -> String {
    if let Some(url) = option_env!("DUET_API_URL")
        && !url.is_empty()
    {
        return url.to_owned();
    }
    #[cfg(target_arch = "wasm32")]
    if let Some(origin) = page_origin() {
        return origin;
    }
    duet_app::api_base_url()
}

#[cfg(all(any(feature = "web", feature = "mobile"), target_arch = "wasm32"))]
fn page_origin() -> Option<String> {
    let origin = web_sys::window()?.location().origin().ok()?;
    if origin.is_empty() || origin == "null" {
        None
    } else {
        Some(origin)
    }
}
