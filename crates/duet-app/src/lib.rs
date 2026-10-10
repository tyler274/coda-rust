//! Duet's four tabs. SwiftUI remains the shipping iPhone app until these
//! journeys match it, including empty and error states of every native screen.

use dioxus::prelude::*;
use duet_health::{HealthSource, PlatformHealth};
use duet_page::{DoctorPage, LaidOut, lay_out};
use duet_widget::WidgetSnapshot;
use ui::{Card, EmptyState, ErrorState, PageTitle, Row, TabBar};

const TABS: [&str; 4] = ["Duet", "Today", "You", "Shared"];

#[component]
pub fn DuetRoot(base_url: String, bearer: String) -> Element {
    let mut tab = use_signal(|| 0usize);
    let health = PlatformHealth.status();
    let moments = use_resource(move || {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        async move {
            match duet_client::DuetClient::new(base_url, bearer, "0", "0.1.0") {
                Ok(client) => client.list_moments().await,
                Err(error) => Err(error),
            }
        }
    });
    let health_detail = match health {
        duet_health::HealthStatus::Available => "HealthKit is available.".to_owned(),
        duet_health::HealthStatus::Unavailable { reason } => reason.to_owned(),
    };
    rsx! {
        style { "{ui::STYLESHEET}" }
        main { class: "app",
            div { class: "page",
                match tab() {
                    0 => rsx! {
                        PageTitle { text: "Duet".to_owned() }
                        Card { EmptyState { message: "One conversation. Messages load from the account when this client can send them.".to_owned() } }
                    },
                    1 => rsx! {
                        PageTitle { text: "Today".to_owned() }
                        Card { EmptyState { message: "Insights are not loaded in this client yet.".to_owned() } }
                    },
                    2 => rsx! { You { health: health_detail, moments: moments() } },
                    _ => rsx! {
                        PageTitle { text: "Shared".to_owned() }
                        Card { EmptyState { message: "Sharing is not loaded in this client yet.".to_owned() } }
                    },
                }
            }
            TabBar {
                labels: TABS.iter().map(|label| (*label).to_owned()).collect(),
                selected: tab(),
                on_select: move |index| tab.set(index),
            }
        }
    }
}

#[component]
fn You(health: String, moments: Option<Result<Vec<duet_client::AccountMoment>, String>>) -> Element {
    let page = lay_out(&DoctorPage {
        name: "Name".into(),
        date_of_birth: "Date of birth".into(),
        questions: Vec::new(),
        cover_note: None,
    });
    let page_detail = match page {
        LaidOut::Page { lines, .. } => format!("{} lines fit on the page.", lines.len()),
        LaidOut::Overflow => "The page is too long to share.".to_owned(),
    };
    let snapshot = WidgetSnapshot { generated_at: "local".into(), lines: vec![health.clone()] };
    rsx! {
        PageTitle { text: "You".to_owned() }
        Card {
            Row { name: "Health".to_owned(), detail: health }
            Row { name: "For my doctor".to_owned(), detail: page_detail }
            Row { name: "Widget snapshot".to_owned(), detail: duet_widget::to_json(&snapshot) }
            match moments {
                None => rsx! { EmptyState { message: "Loading moments.".to_owned() } },
                Some(Err(error)) => rsx! { ErrorState { message: error } },
                Some(Ok(rows)) if rows.is_empty() => rsx! { EmptyState { message: "No moments.".to_owned() } },
                Some(Ok(rows)) => rsx! {
                    for row in rows {
                        Row { name: row.kind.to_string(), detail: row.occurred_at.to_string() }
                    }
                },
            }
        }
    }
}
