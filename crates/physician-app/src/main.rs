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
    let base = option_env!("PHYSICIAN_API_URL").unwrap_or("http://127.0.0.1:8787").to_owned();
    rsx! { physician_app::PhysicianRoot { base_url: base } }
}
