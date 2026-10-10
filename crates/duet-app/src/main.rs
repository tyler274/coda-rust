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
    rsx! { duet_app::DuetRoot { base_url: duet_app::api_base_url(), bearer: duet_app::api_bearer() } }
}
