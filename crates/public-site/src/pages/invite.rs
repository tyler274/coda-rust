use dioxus::prelude::*;

/// Fallback page for `/invite` when visited on desktop or without the Duet app installed.
#[component]
pub fn Invite() -> Element {
    rsx! {
        header { class: "hero",
            div { class: "hero-copy",
                p { class: "kicker", "Practice invitation" }
                h1 { "Open your invitation in Duet." }
                p { class: "lede", "Duet works only in the iPhone and Android app. Open the app on your phone to sign in with email and connect with your practice." }
                div { class: "hero-cta",
                    a { href: "duet://invite", class: "btn", "Open Duet" }
                    a { class: "ghost-link ghost-link--inline", href: "https://apps.apple.com/app/duet-by-coda-health", "App Store" }
                    a { class: "ghost-link ghost-link--inline", href: "https://play.google.com/store/apps/details?id=ai.codahealth.duet", "Google Play" }
                }
            }
        }
    }
}
