use dioxus::prelude::*;
const BOOKING: &str = "https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ3g0UUNb-12RTQS50bhsuRrG2PmsMehtQ9wOGpbjVaGLEmrUp8prf1L6JGJGWYABkunI_4WnuDa";
const EMBED: &str = "https://calendar.google.com/calendar/appointments/schedules/AcZssZ3g0UUNb-12RTQS50bhsuRrG2PmsMehtQ9wOGpbjVaGLEmrUp8prf1L6JGJGWYABkunI_4WnuDa";
#[component]
pub fn Scheduling() -> Element {
    rsx! {
        header { class: "hero", div { class: "hero-copy",
            p { class: "kicker", "Research conversation / 30 minutes" }
            h1 { "Schedule time with Dev." }
            p { class: "lede", "Book a 30-minute conversation about clinical environments, health-agent evaluation, or a research problem your team needs to make executable." }
        } }
        section { class: "schedule",
            div { class: "wrap",
                div { class: "card schedule-card",
                    div { class: "schedule-bar",
                        div {
                            p { class: "micro schedule-provider", "Google Calendar" }
                            p { class: "schedule-title", "30 min with Dev" }
                        }
                        span { class: "micro schedule-live", "Live availability" }
                    }
                    iframe { src: EMBED, title: "Schedule time with Dev Patale", class: "schedule-frame", "loading": "eager", referrerpolicy: "strict-origin-when-cross-origin" }
                }
                div { class: "schedule-note",
                    p { "Meetings are scheduled through Google Calendar." }
                    a { class: "ghost-link ghost-link--inline", href: BOOKING, "Open in Google Calendar" }
                }
            }
        }
    }
}
