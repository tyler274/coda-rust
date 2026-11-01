use crate::{Config, Route, route::Shell, site};
use dioxus::prelude::*;

/// Shared server-rendered layout. Ordinary anchors deliberately need no JS router.
/// Each shell's markup is the original layout's, so the original stylesheets apply as-is.
#[component]
pub fn Chrome() -> Element {
    let route = use_route::<Route>();
    // The original's legal pages share one page component, which marks its main.
    let main_class = match route {
        Route::Home {} | Route::Duet {} | Route::Scheduling {} | Route::NotFound { .. } => None,
        _ => Some("legal-page"),
    };
    match route.shell() {
        Shell::Homepage => rsx! {
            Nav {}
            main { Outlet::<Route> {} }
            Footer {}
        },
        Shell::Utility => rsx! {
            div { class: "flex min-h-svh flex-col bg-bg text-ink",
                Nav {}
                div { class: "flex-1",
                    main { class: main_class, Outlet::<Route> {} }
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
