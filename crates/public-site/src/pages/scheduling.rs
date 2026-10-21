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
        section { class: "pb-20 pt-12 sm:pb-28 sm:pt-14",
            div { class: "wrap",
                div { class: "card overflow-hidden",
                    div { class: "flex min-h-14 flex-wrap items-center justify-between gap-3 border-b border-hairline bg-accent-bg px-4 py-3 sm:px-5",
                        div {
                            p { class: "micro text-ink-light", "Google Calendar" }
                            p { class: "mt-1 text-[0.84rem] font-semibold text-ink", "30 min with Dev" }
                        }
                        span { class: "rounded-full border border-accent-border bg-surface px-3 py-1 micro text-accent", "Live availability" }
                    }
                    iframe { src: EMBED, title: "Schedule time with Dev Patale", class: "h-[720px] w-full border-0 sm:h-[800px] lg:h-[760px]", "loading": "eager", referrerpolicy: "strict-origin-when-cross-origin" }
                }
                div { class: "mt-4 flex flex-col gap-2 text-sm text-ink-light sm:flex-row sm:items-center sm:justify-between",
                    p { "Meetings are scheduled through Google Calendar." }
                    a { class: "ghost-link ghost-link--inline text-[0.68rem]", href: BOOKING, "Open in Google Calendar" }
                }
            }
        }
    }
}
