use crate::site;
use dioxus::prelude::*;
#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        header { class: "hero pb-24 sm:pb-32",
            div { class: "hero-copy",
                p { class: "kicker", "404 / Page not found" }
                h1 { "This page isn't here." }
                p { class: "lede", "The address may be outdated, or the page may have moved. Return to the Coda Health homepage to continue." }
                div { class: "hero-cta",
                    a { href: "/", class: "btn", "Return home" }
                    a { class: "ghost-link ghost-link--inline", href: site::contact_mailto(), "Contact Coda Health" }
                }
            }
        }
    }
}
