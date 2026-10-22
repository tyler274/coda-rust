#[cfg(any(feature = "mobile", feature = "desktop", feature = "web"))]
use dioxus::prelude::*;

fn main() {
    #[cfg(any(feature = "mobile", feature = "desktop", feature = "web"))]
    dioxus::launch(App);
    #[cfg(not(any(feature = "mobile", feature = "desktop", feature = "web")))]
    {
        eprintln!("Build duet-app with --features web, desktop, or mobile.");
        std::process::exit(1);
    }
}

#[cfg(any(feature = "mobile", feature = "desktop", feature = "web"))]
#[component]
fn App() -> Element {
    rsx! { duet_app::DuetRoot { base_url: duet_app::api_base_url(), bearer: duet_app::api_bearer() } }
}
