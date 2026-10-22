use super::phones::Phones;
use dioxus::prelude::*;

#[component]
pub fn Duet() -> Element {
    let principles = [
        (
            "Bring your health together",
            "Connect Apple Health and one or more Epic MyChart organizations when you choose. Duet preserves where information came from and never writes back to those sources.",
        ),
        (
            "Notice the few things that matter",
            "If you allow AI, Duet can show changes and patterns from your health information, with the source available for you to review.",
        ),
        (
            "Carry the useful parts into care",
            "Save an insight, add what you want to discuss, review the prepared summary, and share it through the iPhone share sheet when you are ready.",
        ),
    ];
    rsx! {
        header { class: "hero",
            div { class: "hero-copy",
                div { class: "duet-brand duet-brand-hero rv",
                    span { class: "duet-brand-lockup",
                        img { class: "duet-brand-mark", src: "/brand/duet/mark.svg", width: "400", height: "400", alt: "" }
                        img { class: "duet-brand-wordmark", src: "/brand/duet/wordmark.svg", width: "440", height: "170", alt: "Duet" }
                    }
                }
                p { class: "kicker rv", style: "--d:0.03s", "Your health, in context" }
                h1 { class: "rv", style: "--d:0.06s", "A clearer picture of you." }
                p { class: "lede rv", style: "--d:0.12s", "Duet brings together the health information you choose, notices a few supported patterns, and helps you prepare the useful parts for a conversation with your doctor." }
                p { class: "hero-meta micro rv", style: "--d:0.18s", "iPhone · U.S. adults 18+ · Free · No advertising" }
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
