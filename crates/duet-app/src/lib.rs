//! Duet's four tab roots. SwiftUI remains the shipping iPhone app.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]

mod roots;

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use dioxus::prelude::*;
use duet_client::DuetClient;
use duet_health::{HealthSnapshot, NO_READINGS, PhoneReading, PlatformHealth};
use roots::{
    AWAITING, CHAT_UNAVAILABLE, CHECK_UNAVAILABLE, CHECK_UNAVAILABLE_DETAIL, CONNECTING,
    ConversationRoot, LIBRARY_EMPTY, LIBRARY_UNAVAILABLE, LibraryRoot, NOT_SHARING, REVIEWS_PAUSED,
    REVIEWS_PAUSED_DETAIL, SHARING_SAVED, SHARING_UNCHECKED, SharedNotice, SharedRoot, TRY_AGAIN,
    TodayRoot, conversation_root, date_line, greeting, library_root, shared_root, today_root,
};
use ui::TabBar;

const TABS: [&str; 4] = ["Duet", "Today", "You", "Shared"];
const DUET_STYLES: &str = include_str!("../assets/duet.css");

pub use roots::{api_base_url, api_bearer};

#[component]
pub fn DuetRoot(base_url: String, bearer: String) -> Element {
    let mut tab = use_signal(|| 0usize);
    let mut phone = use_signal(|| PlatformHealth.snapshot());
    rsx! {
        style { "{ui::STYLESHEET}{DUET_STYLES}" }
        main { class: "duet",
            div { class: "duet-page",
                div { hidden: tab() != 0, ConversationTab { base_url: base_url.clone(), bearer: bearer.clone() } }
                div { hidden: tab() != 1,
                    TodayTab {
                        base_url: base_url.clone(),
                        bearer: bearer.clone(),
                        readings: phone().readings.clone(),
                        connected: phone().state.connected(),
                    }
                }
                div { hidden: tab() != 2,
                    YouTab {
                        base_url: base_url.clone(),
                        bearer: bearer.clone(),
                        phone: phone(),
                        on_allow: move |_| {
                            PlatformHealth.request_access();
                            phone.set(PlatformHealth.snapshot());
                        },
                    }
                }
                div { hidden: tab() != 3, SharedTab { base_url, bearer } }
            }
            TabBar {
                labels: TABS.iter().map(|label| (*label).to_owned()).collect(),
                selected: tab(),
                on_select: move |index| {
                    tab.set(index);
                    if index == 1 || index == 2 {
                        phone.set(PlatformHealth.snapshot());
                    }
                },
            }
        }
    }
}

fn use_deferred_status(loading: Signal<bool>) -> bool {
    let mut shown = use_signal(|| false);
    let tickets = use_hook(|| Rc::new(Cell::new(0u64)));
    let tickets_effect = tickets.clone();
    use_effect(move || {
        let is_loading = loading();
        let next = tickets_effect.get() + 1;
        tickets_effect.set(next);
        if !is_loading {
            shown.set(false);
            return;
        }
        let tickets_effect = tickets_effect.clone();
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(300)).await;
            if tickets_effect.get() == next {
                shown.set(true);
            }
        });
    });
    shown() && loading()
}

#[component]
fn TabHeading(name: String, loading: Signal<bool>) -> Element {
    let waiting = use_deferred_status(loading);
    rsx! {
        div { class: "duet-heading",
            h1 { "{name}" }
            p { class: "duet-status", if waiting { "{CONNECTING}" } }
        }
    }
}

#[component]
fn TryAgain(on_retry: EventHandler) -> Element {
    rsx! {
        button {
            class: "duet-retry",
            r#type: "button",
            onclick: move |_| on_retry.call(()),
            "{TRY_AGAIN}"
        }
    }
}

fn open_client(base_url: String, bearer: String) -> Result<DuetClient, String> {
    DuetClient::new(base_url, bearer, "0", "0.1.0")
}

async fn load_today(
    base_url: String,
    bearer: String,
) -> Result<
    (
        duet_client::IntelligenceSettings,
        duet_client::InsightCyclePage,
    ),
    String,
> {
    let api = open_client(base_url, bearer)?;
    let settings = api.intelligence_settings().await?;
    let cycles = api.list_insight_cycles().await?;
    Ok((settings, cycles))
}

#[component]
fn ConversationTab(base_url: String, bearer: String) -> Element {
    let mut attempt = use_signal(|| 0u32);
    let mut loading = use_signal(|| true);
    let mut cache = use_signal(|| None);
    let resource = use_resource(move || {
        let _attempt = attempt();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        async move {
            match open_client(base_url, bearer) {
                Ok(api) => api.list_messages().await,
                Err(error) => Err(error),
            }
        }
    });
    use_effect(move || {
        loading.set(resource().is_none());
        if let Some(Ok(page)) = resource() {
            cache.set(Some(page));
        }
    });
    let current = resource();
    let failed = matches!(current, Some(Err(_)));
    let page = match current {
        Some(Ok(page)) => Some(page),
        _ => cache(),
    };
    let root = page.as_ref().map(conversation_root);
    let retry = move |_| attempt.set(attempt() + 1);
    rsx! {
        TabHeading { name: "Duet".to_owned(), loading }
        match root {
            None if !failed => rsx! {},
            None => rsx! {
                div { role: "alert",
                    p { class: "empty", "{CHAT_UNAVAILABLE}" }
                    TryAgain { on_retry: retry }
                }
            },
            Some(ConversationRoot::Empty { title, message }) => rsx! {
                p { class: "empty-title", "{title}" }
                p { class: "empty", "{message}" }
                if failed {
                    TryAgain { on_retry: retry }
                }
            },
            Some(ConversationRoot::Messages(lines)) => rsx! {
                for line in lines {
                    article { class: if line.from_duet { "voice-card" } else { "user-bubble" },
                        p { class: "card-title", "{line.role}" }
                        p { class: "card-text", "{line.text}" }
                    }
                }
                if failed {
                    div { role: "alert",
                        p { class: "empty", "{CHAT_UNAVAILABLE}" }
                        TryAgain { on_retry: retry }
                    }
                }
            },
        }
    }
}

#[component]
fn TodayTab(
    base_url: String,
    bearer: String,
    readings: Vec<PhoneReading>,
    connected: bool,
) -> Element {
    let mut attempt = use_signal(|| 0u32);
    let mut loading = use_signal(|| true);
    let mut cache = use_signal(|| None);
    let resource = use_resource(move || {
        let _attempt = attempt();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        load_today(base_url, bearer)
    });
    use_effect(move || {
        loading.set(resource().is_none());
        if let Some(Ok(value)) = resource() {
            cache.set(Some(value));
        }
    });
    let current = resource();
    let failed = matches!(current, Some(Err(_)));
    let ready = match current {
        Some(Ok(value)) => Some(value),
        _ => cache(),
    };
    let root = ready
        .as_ref()
        .map(|(settings, cycles)| today_root(settings, cycles));
    let now = chrono::Local::now();
    let date = date_line(now);
    let hello = greeting(now.format("%H").to_string().parse().unwrap_or(0));
    let retry = move |_| attempt.set(attempt() + 1);
    let show_unavailable = failed && !matches!(root, Some(TodayRoot::Insights(_)));
    rsx! {
        TabHeading { name: "Today".to_owned(), loading }
        p { class: "duet-date", "{date}" }
        p { class: "duet-greeting", "{hello}" }
        PhoneReadings { readings, connected }
        match root {
            Some(TodayRoot::Quiet) => rsx! {},
            Some(TodayRoot::Paused) => rsx! {
                p { class: "status-title", "{REVIEWS_PAUSED}" }
                p { class: "status-detail", "{REVIEWS_PAUSED_DETAIL}" }
            },
            Some(TodayRoot::Insights(cards)) => rsx! {
                for card in cards {
                    article { class: "voice-card",
                        p { class: "card-title", "{card.title}" }
                        p { class: "card-text", "{card.sentence}" }
                    }
                }
            },
            None => rsx! {},
        }
        if show_unavailable {
            div { role: "alert",
                p { class: "status-title", "{CHECK_UNAVAILABLE}" }
                p { class: "status-detail", "{CHECK_UNAVAILABLE_DETAIL}" }
                TryAgain { on_retry: retry }
            }
        }
    }
}

#[component]
fn YouTab(
    base_url: String,
    bearer: String,
    phone: HealthSnapshot,
    on_allow: EventHandler<()>,
) -> Element {
    let mut attempt = use_signal(|| 0u32);
    let mut loading = use_signal(|| true);
    let mut cache = use_signal(|| None);
    let resource = use_resource(move || {
        let _attempt = attempt();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        async move {
            match open_client(base_url, bearer) {
                Ok(api) => api.evidence_library().await,
                Err(error) => Err(error),
            }
        }
    });
    use_effect(move || {
        loading.set(resource().is_none());
        if let Some(Ok(page)) = resource() {
            cache.set(Some(page));
        }
    });
    let current = resource();
    let failed = matches!(current, Some(Err(_)));
    let page = match current {
        Some(Ok(page)) => Some(page),
        _ => cache(),
    };
    let root = page.as_ref().map(library_root);
    let retry = move |_| attempt.set(attempt() + 1);
    rsx! {
        TabHeading { name: "You".to_owned(), loading }
        div { class: "source-row",
            span { class: "source-name", "Sources" }
            span { class: "source-detail", "{phone.sentence()}" }
            if phone.state.can_request() {
                button { class: "duet-retry", onclick: move |_| on_allow.call(()), "Allow" }
            }
        }
        PhoneReadings { readings: phone.readings.clone(), connected: phone.state.connected() }
        match root {
            Some(LibraryRoot::Empty) => rsx! { p { class: "empty", "{LIBRARY_EMPTY}" } },
            Some(LibraryRoot::Sections(sections)) => rsx! {
                for section in sections {
                    article { class: if section.measured { "number-card" } else { "voice-card" },
                        p { class: "card-title", "{section.name}" }
                    }
                }
            },
            None => rsx! {},
        }
        if failed && page.is_none() {
            div { role: "alert",
                p { class: "empty", "{LIBRARY_UNAVAILABLE}" }
                TryAgain { on_retry: retry }
            }
        }
    }
}

#[component]
fn PhoneReadings(readings: Vec<PhoneReading>, connected: bool) -> Element {
    rsx! {
        if connected && readings.is_empty() {
            p { class: "empty", "{NO_READINGS}" }
        }
        for reading in readings {
            article { class: "number-card",
                p { class: "card-title", "{reading.name}" }
                p { class: "card-text", "{reading.value} {reading.unit}" }
            }
        }
    }
}

#[component]
fn SharedTab(base_url: String, bearer: String) -> Element {
    let mut attempt = use_signal(|| 0u32);
    let mut loading = use_signal(|| true);
    let mut cache = use_signal(|| None);
    let resource = use_resource(move || {
        let _attempt = attempt();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        async move {
            match open_client(base_url, bearer) {
                Ok(api) => api.sharing_overview().await,
                Err(error) => Err(error),
            }
        }
    });
    use_effect(move || {
        loading.set(resource().is_none());
        if let Some(Ok(overview)) = resource() {
            cache.set(Some(shared_root(&[], Ok(&overview))));
        }
    });
    let current = resource();
    let view = match &current {
        Some(Ok(overview)) => shared_root(&[], Ok(overview)),
        Some(Err(_)) => shared_root(
            &cache().map(|root| root.people).unwrap_or_default(),
            Err(()),
        ),
        None => cache().unwrap_or(SharedRoot {
            people: Vec::new(),
            notice: None,
        }),
    };
    let people = view.people.clone();
    let notice = view.notice.clone();
    let retry = move |_| attempt.set(attempt() + 1);
    rsx! {
        TabHeading { name: "Shared".to_owned(), loading }
        if !people.is_empty() {
            p { class: "status-title", "People" }
            for person in people {
                article { class: "voice-card",
                    p { class: "card-title", "{person}" }
                }
            }
        }
        match notice {
            Some(SharedNotice::NotSharing) => rsx! { p { class: "empty", "{NOT_SHARING}" } },
            Some(SharedNotice::CouldNotCheck) => rsx! {
                div { role: "alert",
                    p { class: "empty", "{SHARING_UNCHECKED}" }
                    TryAgain { on_retry: retry }
                }
            },
            Some(SharedNotice::SavedStayVisible) => rsx! {
                div { role: "alert",
                    p { class: "empty", "{SHARING_SAVED}" }
                    TryAgain { on_retry: retry }
                }
            },
            Some(SharedNotice::AwaitingAcceptance) => rsx! { p { class: "status-title", "{AWAITING}" } },
            None => rsx! {},
        }
    }
}
