use crate::{Config, Route, route::Shell, site};
use dioxus::prelude::*;

/// Shared server-rendered layout. Ordinary anchors deliberately need no JS router.
/// Document pages wrap their content so the footer stays at the foot of a short page.
/// The homepage family's pages are never short and stay unwrapped: inside the wrapper
/// the floating nav would sit lower once the page scrolls.
#[component]
pub fn Chrome() -> Element {
    match use_route::<Route>().shell() {
        Shell::Homepage => rsx! {
            Nav {}
            main { Outlet::<Route> {} }
            Footer {}
        },
        Shell::Document => rsx! {
            div { class: "page",
                Nav {}
                div { class: "page-content",
                    main { Outlet::<Route> {} }
                }
                Footer {}
            }
        },
    }
}

#[component]
fn Nav() -> Element {
    let sign_in = use_context::<Config>().physician_sign_in;
    rsx! {
        nav { class: "nav", "aria-label": "Main navigation",
            div { class: "nav-in",
                a { class: "logo", href: "/", "aria-label": "Coda Health home",
                    img { src: "/brand/coda-health-logo-transparent.webp", alt: "Coda Health" }
                }
                div { class: "nav-links",
                    a { href: "/#physicians", "For physicians" }
                    a { href: "/#patients", "For patients" }
                    if sign_in {
                        a { href: "/physician/", "Sign in" }
                    }
                }
                a { class: "btn btn-sm", href: site::contact_mailto(), "Partner with us" }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "foot",
            div { class: "wrap foot-in",
                div { class: "foot-side",
                    a { class: "foot-brand", href: "/", aria_label: "Coda Health home",
                        img { src: "/brand/coda-health-logo-transparent.webp", alt: "Coda Health" }
                    }
                    span { class: "foot-copy", "© 2026 Coda Health" }
                }
                // One column per product and one for the company, whose privacy
                // policy and terms cover the site and both products.
                div { class: "foot-cols",
                    div {
                        span { class: "micro", "Chords" }
                        a { href: "/#physicians", "For physicians" }
                        a { href: "/clinical-transparency", "Clinical Transparency" }
                    }
                    div {
                        span { class: "micro", "Duet" }
                        a { href: "/duet", "For patients" }
                        a { href: "/duet/privacy", "Privacy" }
                        a { href: "/duet/terms", "Terms" }
                        a { href: "/duet/consumer-health-data", "Consumer Health Data Privacy Policy" }
                    }
                    div {
                        span { class: "micro", "Coda Health" }
                        a { href: site::contact_mailto(), "{site::CONTACT_EMAIL}" }
                        a { href: "/privacy", "Privacy" }
                        a { href: "/terms", "Terms" }
                    }
                }
            }
        }
    }
}
