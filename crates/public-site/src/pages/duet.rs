use super::brand::{Product, ProductLockup};
use super::phones::Phones;
use crate::{Config, PhysicianLink};
use dioxus::prelude::*;

#[component]
pub fn Duet() -> Element {
    let demo = use_context::<Config>().physician_link() == PhysicianLink::LiveDemo;
    let principles = [
        (
            "Bring your health together",
            "Connect Apple Health on iPhone, Health Connect on Android, and Epic MyChart or Oracle Health organizations when you choose. Duet preserves exact source provenance and never writes back to those sources.",
        ),
        (
            "Your records, kept faithful",
            "Browse labs, measurements, medications, and visits in You with exact values, dates, and per-source trends you can inspect directly from the original record.",
        ),
        (
            "For my doctor, in your own words",
            "Mark any record, value, reading, or trend in one tap, add notes in For my doctor, and send them to your connected Coda practice when you choose.",
        ),
    ];
    rsx! {
        header { class: "hero",
            div { class: "hero-copy",
                ProductLockup { product: Product::Duet, hero: true }
                p { class: "kicker rv", style: "--d:0.03s", "Your health, in context" }
                h1 { class: "rv", style: "--d:0.06s", "A clearer picture of you." }
                p { class: "lede rv", style: "--d:0.12s", "Duet brings together the health records and readings you choose, keeps every source faithful in You, and lets you mark records and write notes in For my doctor for your connected practice." }
                p { class: "hero-meta micro rv", style: "--d:0.18s", "iPhone & Android · U.S. adults 18+ · Free · No advertising" }
                if demo {
                    div { class: "hero-cta rv", style: "--d:0.22s",
                        div { class: "hero-cta-row",
                            a { class: "btn", href: "/physician/", "Try Chords clinician demo" }
                            a { class: "btn btn-secondary", href: "/#patients", "See Connected Care" }
                        }
                    }
                }
            }
        }
        section { class: "sec sec-tight", div { class: "wrap", Phones {} } }
        section { class: "sec",
            div { class: "wrap tiles",
                for (index, (title, body)) in principles.iter().enumerate() {
                    div { class: "tile rv", style: format!("--d:{}s", index as f64 * 0.06),
                        h3 { "{title}" }
                        p { "{body}" }
                    }
                }
            }
        }
        section { class: "sec sec-close",
            div { class: "wrap duet-note",
                p { "Duet is not emergency care and does not diagnose, prescribe, or replace your healthcare professional." }
            }
        }
        script { src: "/homepage/site.js", defer: true }
        script { src: "/homepage/duet.js", defer: true }
    }
}
