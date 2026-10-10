#[cfg(any(feature = "mobile", feature = "desktop"))]
use dioxus::prelude::*;

fn main() {
    #[cfg(any(feature = "mobile", feature = "desktop"))]
    dioxus::launch(App);
    #[cfg(not(any(feature = "mobile", feature = "desktop")))]
    {
        eprintln!("Build duet-app with --features desktop or --features mobile.");
        std::process::exit(1);
    }
}

#[cfg(any(feature = "mobile", feature = "desktop"))]
#[component]
fn App() -> Element {
    let base = std::env::var("DUET_API_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".into());
    let bearer = std::env::var("DUET_BEARER").unwrap_or_default();
    rsx! { duet_app::DuetRoot { base_url: base, bearer } }
}
