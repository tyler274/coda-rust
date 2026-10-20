use crate::{Config, Route, site};
use dioxus::prelude::*;

/// Shared server-rendered layout. Ordinary anchors deliberately need no JS router.
#[component]
pub fn Chrome() -> Element {
    let config = use_context::<Config>();
    let route = use_route::<Route>();
    let utility = !matches!(route, Route::Home {} | Route::Duet {});
    rsx! {
        div { class: if utility { "utility-shell" } else { "marketing-shell" },
        nav { class: "nav", "aria-label": "Main navigation",
            div { class: "nav-in",
                a { class: "logo", href: "/", "aria-label": "Coda Health home",
                    img { src: "/brand/coda-health-logo-transparent.webp", alt: "Coda Health" }
                }
                div { class: "nav-links",
                    a { href: "/#physicians", "For physicians" }
                    a { href: "/#patients", "For patients" }
                    a { href: config.physician_app_url, "Sign in" }
                }
                a { class: "btn btn-sm", href: site::contact_mailto(), "Partner with us" }
            }
        }
        main { id: "main", class: match route { Route::Home {} | Route::Duet {} => "", Route::Scheduling {} | Route::NotFound { .. } => "utility-page", _ => "utility-page legal-page" }, Outlet::<Route> {} }
        footer { class: "foot",
            div { class: "wrap foot-in",
                div { class: "foot-side",
                    a { class: "foot-brand", href: "/", aria_label: "Coda Health home",
                        img { src: "/brand/coda-health-logo-transparent.webp", alt: "Coda Health" }
                    }
                    span { class: "foot-copy", "© 2026 Coda Health" }
                }
                div { class: "foot-cols",
                    div {
                        span { class: "micro", "Coda" }
                        a { href: "/#physicians", "For physicians" }
                        a { href: "/clinical-transparency", "Clinical Transparency" }
                        a { href: "/privacy", "Privacy" }
                        a { href: "/terms", "Terms" }
                    }
                    div {
                        span { class: "micro", "Duet" }
                        a { href: "/duet", "For patients" }
                        a { href: "/duet/privacy", "Privacy" }
                        a { href: "/duet/terms", "Terms" }
                        a { href: "/duet/consumer-health-data", "Consumer Health Data Privacy Policy" }
                    }
                    div {
                        span { class: "micro", "Contact" }
                        a { href: site::contact_mailto(), "{site::CONTACT_EMAIL}" }
                    }
                }
            }
        }
        }
    }
}
