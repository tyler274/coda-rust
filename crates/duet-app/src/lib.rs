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
use duet_wire::models::ConversationMessage;
use roots::{
    AWAITING, CHAT_EMPTY_MESSAGE, CHAT_UNAVAILABLE, CHECK_UNAVAILABLE, CHECK_UNAVAILABLE_DETAIL,
    CONNECTING, ConversationNote, ConversationRoot, DoctorPage, LIBRARY_EMPTY, LIBRARY_UNAVAILABLE,
    LibraryRoot, NOT_SHARING, NoteOpen, REVIEWS_PAUSED, REVIEWS_PAUSED_DETAIL, SHARING_SAVED,
    SHARING_UNCHECKED, SavedItemPage, SharedNotice, SharedRoot, TRY_AGAIN, TodayRoot, closed_send,
    composer_open, composer_text, conversation_root, date_line, doctor_page, draft_after_send,
    greeting, library_root, note_open, page_with_accepted, plan_page, reminder_page, send_id,
    shared_root, today_root,
};
use ui::TabBar;

const TABS: [&str; 4] = ["Duet", "Today", "You", "Shared"];

#[derive(Clone)]
struct ThreadLoad {
    page: duet_client::ConversationPage,
    routines: Option<Vec<duet_wire::models::SavedRoutine>>,
    follow_ups: Option<Vec<duet_wire::models::FollowUp>>,
    doctor: Option<duet_wire::models::DoctorCollection>,
}
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

fn doctor_page_save(
    collection: &duet_wire::models::DoctorCollection,
) -> duet_wire::models::DoctorPageSave {
    let item_ids = collection
        .items
        .iter()
        .flatten()
        .map(|item| item.id)
        .collect();
    let page = collection
        .page
        .as_ref()
        .map(|page| (**page).clone())
        .unwrap_or_else(duet_wire::models::DoctorPageContent::new);
    let mut save =
        duet_wire::models::DoctorPageSave::new(collection.note_generation, item_ids, page);
    save.note = collection.note.clone();
    save
}

#[derive(Clone, Copy, PartialEq)]
enum YouEditor {
    Medication,
    Note,
}

#[derive(Clone)]
enum ShownPage {
    Doctor(DoctorPage),
    Item(SavedItemPage),
}

impl ShownPage {
    fn heading(&self) -> String {
        match self {
            ShownPage::Doctor(_) => "For my doctor".to_owned(),
            ShownPage::Item(item) => item.heading.clone(),
        }
    }
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
    let mut draft = use_signal(String::new);
    let mut pending_id = use_signal(|| None::<(uuid::Uuid, String)>);
    let mut sending = use_signal(|| false);
    let mut send_failed = use_signal(|| false);
    let mut accepted = use_signal(|| None::<ConversationMessage>);
    let mut routines = use_signal(Vec::new);
    let mut follow_ups = use_signal(Vec::new);
    let mut doctor = use_signal(|| None);
    let mut open_item = use_signal(|| None::<NoteOpen>);
    let mut saving_page = use_signal(|| false);
    let mut page_failed = use_signal(|| false);
    let mut editing = use_signal(|| false);
    let mut plan_title = use_signal(String::new);
    let mut plan_body = use_signal(String::new);
    let mut confirm_remove = use_signal(|| false);
    let mut removed_plan = use_signal(|| false);
    let mut plan_save_failed = use_signal(|| false);
    let mut plan_remove_failed = use_signal(|| false);
    let mut follow_failed = use_signal(|| false);
    let mut changing = use_signal(|| false);
    let send_url = base_url.clone();
    let send_bearer = bearer.clone();
    let resource = use_resource(move || {
        let _attempt = attempt();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        async move {
            let api = open_client(base_url, bearer)?;
            let page = api.list_messages().await?;
            let routines = match api.list_saved_routines().await {
                Ok(list) => Some(list.items),
                Err(_) => None,
            };
            let follow_ups = match api.list_follow_ups().await {
                Ok(page) => Some(page.items),
                Err(_) => None,
            };
            let doctor = match api.doctor_collection().await {
                Ok(collection) => Some(collection),
                Err(_) => None,
            };
            Ok::<_, String>(ThreadLoad {
                page,
                routines,
                follow_ups,
                doctor,
            })
        }
    });
    use_effect(move || {
        loading.set(resource().is_none());
        if let Some(Ok(loaded)) = resource() {
            if let Some(items) = loaded.routines.clone() {
                routines.set(items);
            }
            if let Some(items) = loaded.follow_ups.clone() {
                follow_ups.set(items);
            }
            if let Some(collection) = loaded.doctor.clone() {
                doctor.set(Some(collection));
            }
            let page = match accepted() {
                Some(message) => {
                    let merged = page_with_accepted(loaded.page.clone(), message.clone());
                    if loaded
                        .page
                        .messages
                        .iter()
                        .any(|item| item.id == message.id)
                    {
                        accepted.set(None);
                    }
                    merged
                }
                None => loaded.page,
            };
            cache.set(Some(page));
        }
    });
    let current = resource();
    let failed = matches!(current, Some(Err(_)));
    let page = if accepted().is_some() {
        cache()
    } else {
        match current {
            Some(Ok(loaded)) => Some(loaded.page),
            _ => cache(),
        }
    };
    let root = page
        .as_ref()
        .map(|page| conversation_root(page, &routines(), &follow_ups(), doctor().as_ref()));
    let retry = move |_| attempt.set(attempt() + 1);
    let shown = match open_item() {
        Some(NoteOpen::Doctor) => doctor().as_ref().map(doctor_page).map(ShownPage::Doctor),
        Some(NoteOpen::Routine(_)) if removed_plan() => Some(ShownPage::Item(SavedItemPage {
            heading: "Plan".to_owned(),
            title: None,
            detail: Some("This saved plan was removed.".to_owned()),
        })),
        Some(NoteOpen::Routine(id)) => routines()
            .iter()
            .find(|routine| routine.id == id)
            .map(plan_page)
            .map(ShownPage::Item),
        Some(NoteOpen::FollowUp(id)) => follow_ups()
            .iter()
            .find(|follow_up| follow_up.id == id)
            .map(reminder_page)
            .map(ShownPage::Item),
        None => None,
    };
    let can_compose = page.as_ref().is_some_and(composer_open) && shown.is_none();
    let closed = page.as_ref().and_then(closed_send);
    let page_url = send_url.clone();
    let page_bearer = send_bearer.clone();
    let mut submit = move |text: String| {
        if sending() {
            return;
        }
        if text.trim().is_empty() {
            return;
        }
        draft.set(text.clone());
        let pending = pending_id();
        let id = send_id(
            pending
                .as_ref()
                .map(|(id, pending_text)| (*id, pending_text.as_str())),
            &text,
            uuid::Uuid::new_v4(),
        );
        pending_id.set(Some((id, text.clone())));
        sending.set(true);
        send_failed.set(false);
        let base_url = send_url.clone();
        let bearer = send_bearer.clone();
        let shown = page.clone();
        spawn(async move {
            let timezone = match iana_time_zone::get_timezone() {
                Ok(timezone) => timezone,
                Err(_) => {
                    send_failed.set(true);
                    sending.set(false);
                    return;
                }
            };
            let api = match open_client(base_url, bearer) {
                Ok(api) => api,
                Err(_) => {
                    send_failed.set(true);
                    sending.set(false);
                    return;
                }
            };
            match api.send_message(id, &text, &timezone).await {
                Ok(admission) => {
                    let message = *admission.message;
                    accepted.set(Some(message.clone()));
                    if let Some(base) = shown {
                        cache.set(Some(page_with_accepted(base, message)));
                    }
                    draft.set(draft_after_send(&draft(), true));
                    pending_id.set(None);
                    send_failed.set(false);
                    attempt.set(attempt() + 1);
                }
                Err(_) => {
                    draft.set(draft_after_send(&draft(), false));
                    send_failed.set(true);
                }
            }
            sending.set(false);
        });
    };
    let plan_url = page_url.clone();
    let plan_bearer = page_bearer.clone();
    let remove_url = page_url.clone();
    let remove_bearer = page_bearer.clone();
    let pause_url = page_url.clone();
    let pause_bearer = page_bearer.clone();
    let resume_url = page_url.clone();
    let resume_bearer = page_bearer.clone();
    let save_plan = move |_| {
        if changing() {
            return;
        }
        let Some(NoteOpen::Routine(id)) = open_item() else {
            return;
        };
        let title = plan_title().trim().to_owned();
        let body = plan_body().trim().to_owned();
        if title.is_empty() || body.is_empty() {
            return;
        }
        let Some(revision) = routines()
            .iter()
            .find(|routine| routine.id == id)
            .map(|routine| routine.revision)
        else {
            return;
        };
        changing.set(true);
        plan_save_failed.set(false);
        let base_url = plan_url.clone();
        let bearer = plan_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.update_saved_routine(id, revision, &title, &body).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(routine) => {
                    let mut items = routines();
                    if let Some(item) = items.iter_mut().find(|item| item.id == routine.id) {
                        *item = routine;
                    }
                    routines.set(items);
                    editing.set(false);
                    plan_save_failed.set(false);
                }
                Err(_) => plan_save_failed.set(true),
            }
            changing.set(false);
        });
    };
    let remove_plan = move |_| {
        if changing() {
            return;
        }
        let Some(NoteOpen::Routine(id)) = open_item() else {
            return;
        };
        let Some(revision) = routines()
            .iter()
            .find(|routine| routine.id == id)
            .map(|routine| routine.revision)
        else {
            return;
        };
        changing.set(true);
        plan_remove_failed.set(false);
        let base_url = remove_url.clone();
        let bearer = remove_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.delete_saved_routine(id, revision).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(()) => {
                    let mut items = routines();
                    items.retain(|item| item.id != id);
                    routines.set(items);
                    editing.set(false);
                    confirm_remove.set(false);
                    removed_plan.set(true);
                    plan_remove_failed.set(false);
                }
                Err(_) => plan_remove_failed.set(true),
            }
            changing.set(false);
        });
    };
    let pause_reminder = move |_| {
        if changing() {
            return;
        }
        let Some(NoteOpen::FollowUp(id)) = open_item() else {
            return;
        };
        let Some(revision) = follow_ups()
            .iter()
            .find(|follow_up| follow_up.id == id)
            .map(|follow_up| follow_up.revision)
        else {
            return;
        };
        changing.set(true);
        follow_failed.set(false);
        let base_url = pause_url.clone();
        let bearer = pause_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.pause_follow_up(id, revision).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(follow_up) => {
                    let mut items = follow_ups();
                    if let Some(item) = items.iter_mut().find(|item| item.id == follow_up.id) {
                        *item = follow_up;
                    }
                    follow_ups.set(items);
                    follow_failed.set(false);
                }
                Err(_) => follow_failed.set(true),
            }
            changing.set(false);
        });
    };
    let resume_reminder = move |_| {
        if changing() {
            return;
        }
        let Some(NoteOpen::FollowUp(id)) = open_item() else {
            return;
        };
        let Some(revision) = follow_ups()
            .iter()
            .find(|follow_up| follow_up.id == id)
            .map(|follow_up| follow_up.revision)
        else {
            return;
        };
        changing.set(true);
        follow_failed.set(false);
        let base_url = resume_url.clone();
        let bearer = resume_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.resume_follow_up(id, revision).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(follow_up) => {
                    let mut items = follow_ups();
                    if let Some(item) = items.iter_mut().find(|item| item.id == follow_up.id) {
                        *item = follow_up;
                    }
                    follow_ups.set(items);
                    follow_failed.set(false);
                }
                Err(_) => follow_failed.set(true),
            }
            changing.set(false);
        });
    };
    let save_page = move |_| {
        if saving_page() {
            return;
        }
        let Some(collection) = doctor() else {
            return;
        };
        saving_page.set(true);
        page_failed.set(false);
        let base_url = page_url.clone();
        let bearer = page_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.save_doctor_page(doctor_page_save(&collection)).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(collection) => {
                    doctor.set(Some(collection));
                    page_failed.set(false);
                }
                Err(_) => page_failed.set(true),
            }
            saving_page.set(false);
        });
    };
    rsx! {
        if let Some(shown) = shown {
            TabHeading { name: shown.heading(), loading }
            button {
                class: "duet-retry",
                r#type: "button",
                onclick: move |_| {
                    open_item.set(None);
                    editing.set(false);
                    confirm_remove.set(false);
                    removed_plan.set(false);
                    plan_save_failed.set(false);
                    plan_remove_failed.set(false);
                    follow_failed.set(false);
                },
                "Close"
            }
            match shown {
                ShownPage::Doctor(DoctorPage::Lines(entries)) => rsx! {
                    div { class: "duet-thread",
                        for entry in entries {
                            p { class: "card-text", "{entry.title}" }
                            if let Some(detail) = entry.detail {
                                p { class: "record-meta", "{detail}" }
                            }
                        }
                    }
                },
                ShownPage::Doctor(DoctorPage::Empty) => rsx! {
                    p { class: "empty", "{LIBRARY_EMPTY}" }
                },
                ShownPage::Item(item) if !editing() => rsx! {
                    div { class: "duet-thread",
                        if let Some(title) = item.title {
                            p { class: "card-text", "{title}" }
                        }
                        if let Some(detail) = item.detail {
                            p { class: "record-meta", "{detail}" }
                        }
                        if matches!(open_item(), Some(NoteOpen::FollowUp(id)) if follow_ups().iter().any(|follow_up| follow_up.id == id && follow_up.state == duet_wire::models::FollowUpState::Paused)) {
                            p { class: "record-meta", "Paused" }
                        }
                    }
                },
                ShownPage::Item(_) => rsx! {},
            }
            if editing() {
                PlanEditor {
                    title: plan_title,
                    body: plan_body,
                    saving: changing,
                    failed: plan_save_failed,
                    on_save: save_plan,
                }
            }
            if !editing() && !removed_plan() && !confirm_remove() && matches!(open_item(), Some(NoteOpen::Routine(_))) {
                button {
                    class: "duet-retry",
                    r#type: "button",
                    disabled: changing(),
                    onclick: move |_| {
                        let Some(NoteOpen::Routine(id)) = open_item() else {
                            return;
                        };
                        let Some(routine) = routines().iter().find(|routine| routine.id == id).cloned() else {
                            return;
                        };
                        plan_title.set(routine.title);
                        plan_body.set(routine.body);
                        plan_save_failed.set(false);
                        editing.set(true);
                    },
                    "Edit plan"
                }
                button {
                    class: "duet-retry",
                    r#type: "button",
                    disabled: changing(),
                    onclick: move |_| {
                        plan_remove_failed.set(false);
                        confirm_remove.set(true);
                    },
                    "Remove plan"
                }
                if plan_remove_failed() {
                    p { class: "empty", role: "alert", "The plan couldn’t be removed. Review the current version and try again." }
                }
            }
            if confirm_remove() {
                p { class: "status-title", "Remove this plan?" }
                p { class: "status-detail", "This removes the saved plan and discards your unsaved edits. Your conversation stays in Duet." }
                button {
                    class: "duet-retry",
                    r#type: "button",
                    disabled: changing(),
                    onclick: move |_| confirm_remove.set(false),
                    "Cancel"
                }
                button {
                    class: "duet-send",
                    r#type: "button",
                    disabled: changing(),
                    onclick: remove_plan,
                    "Remove"
                }
            }
            if matches!(open_item(), Some(NoteOpen::FollowUp(id)) if follow_ups().iter().any(|follow_up| follow_up.id == id)) {
                if follow_failed() {
                    p { class: "empty", role: "alert", "Duet couldn’t change this scheduled item. Its last saved state is still here." }
                }
                if matches!(open_item(), Some(NoteOpen::FollowUp(id)) if follow_ups().iter().any(|follow_up| follow_up.id == id && follow_up.state == duet_wire::models::FollowUpState::Paused)) {
                    button {
                        class: "duet-send",
                        r#type: "button",
                        disabled: changing(),
                        onclick: resume_reminder,
                        "Resume reminder"
                    }
                } else {
                    button {
                        class: "duet-send",
                        r#type: "button",
                        disabled: changing(),
                        onclick: pause_reminder,
                        "Pause reminder"
                    }
                }
            }
            if matches!(open_item(), Some(NoteOpen::Doctor)) {
                if page_failed() {
                    p { class: "empty", role: "alert", "This page couldn’t be saved." }
                }
                button {
                    class: "duet-send",
                    r#type: "button",
                    disabled: saving_page(),
                    onclick: save_page,
                    "Save page"
                }
            }
        } else {
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
                div { class: "duet-thread",
                    p { class: "empty-title", "{title}" }
                    p { class: "empty", "{message}" }
                    if failed {
                        TryAgain { on_retry: retry }
                    }
                }
            },
            Some(ConversationRoot::Messages(lines)) => rsx! {
                div { class: "duet-thread",
                    for line in lines {
                        article { class: if line.from_duet { "voice-card" } else { "user-bubble" },
                            p { class: "card-title", "{line.role}" }
                            p { class: "card-text", "{line.text}" }
                            for note in line.notes {
                                ThreadNote {
                                    note: note.clone(),
                                    open: note_open(&note, doctor().is_some(), &routines(), &follow_ups()),
                                    on_open: move |which| open_item.set(Some(which)),
                                }
                            }
                        }
                    }
                    if let Some(closed) = closed {
                        p { class: "empty-title", "{closed.title}" }
                        p { class: "empty", "{closed.message}" }
                    }
                    if failed {
                        div { role: "alert",
                            p { class: "empty", "{CHAT_UNAVAILABLE}" }
                            TryAgain { on_retry: retry }
                        }
                    }
                }
            },
        }
        if can_compose {
            div { class: "duet-composer-wrap",
                if send_failed() {
                    p { class: "empty", role: "alert", "{CHAT_UNAVAILABLE}" }
                }
                form {
                    class: "duet-composer",
                    onsubmit: move |event| {
                        event.prevent_default();
                        let typed = event.get_first("draft").and_then(|value| match value {
                            dioxus::events::FormValue::Text(text) => Some(text),
                            dioxus::events::FormValue::File(_) => None,
                        });
                        submit(composer_text(typed.as_deref(), &draft()));
                    },
                    textarea {
                        class: "duet-composer-field",
                        name: "draft",
                        aria_label: "{CHAT_EMPTY_MESSAGE}",
                        rows: "1",
                        value: "{draft}",
                        oninput: move |event| draft.set(event.value()),
                    }
                    button {
                        class: "duet-send",
                        r#type: "submit",
                        disabled: sending(),
                        "Send"
                    }
                }
            }
        }
        }
    }
}

#[component]
fn ThreadNote(
    note: ConversationNote,
    open: Option<NoteOpen>,
    on_open: EventHandler<NoteOpen>,
) -> Element {
    if let Some(open) = open {
        rsx! {
            button {
                class: "thread-open",
                r#type: "button",
                onclick: move |_| on_open.call(open),
                {note_body(note)}
            }
        }
    } else {
        note_body(note)
    }
}

fn note_body(note: ConversationNote) -> Element {
    match note {
        ConversationNote::Card(card) => rsx! {
            div { class: "thread-card",
                p { class: "card-title", "{card.receipt}" }
                if let Some(title) = card.title {
                    p { class: "card-text", "{title}" }
                }
                if let Some(detail) = card.detail {
                    p { class: "record-meta", "{detail}" }
                }
                if let Some(schedule) = card.schedule {
                    p { class: "record-meta", "{schedule}" }
                }
                for entry in card.entries {
                    p { class: "card-text", "{entry.title}" }
                    if let Some(detail) = entry.detail {
                        p { class: "record-meta", "{detail}" }
                    }
                }
            }
        },
        ConversationNote::Receipt(sentence) => rsx! {
            p { class: "record-meta thread-card", "{sentence}" }
        },
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
    let mut menu = use_signal(|| false);
    let mut editor = use_signal(|| None::<YouEditor>);
    let mut medication = use_signal(String::new);
    let mut note_title = use_signal(String::new);
    let mut note_body = use_signal(String::new);
    let mut medication_id = use_signal(|| None::<uuid::Uuid>);
    let mut note_id = use_signal(|| None::<uuid::Uuid>);
    let mut saving = use_signal(|| false);
    let mut save_failed = use_signal(|| false);
    let save_url = base_url.clone();
    let save_bearer = bearer.clone();
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
    let save_record = move |_| {
        let Some(kind) = editor() else {
            return;
        };
        if saving() {
            return;
        }
        let (id, name, title, body) = match kind {
            YouEditor::Medication => {
                let name = medication().trim().to_owned();
                if name.is_empty() {
                    return;
                }
                let id = medication_id().unwrap_or_else(uuid::Uuid::new_v4);
                medication_id.set(Some(id));
                (id, Some(name), None, None)
            }
            YouEditor::Note => {
                let title = note_title().trim().to_owned();
                let body = note_body().trim().to_owned();
                if title.is_empty() || body.is_empty() {
                    return;
                }
                let id = note_id().unwrap_or_else(uuid::Uuid::new_v4);
                note_id.set(Some(id));
                (id, None, Some(title), Some(body))
            }
        };
        saving.set(true);
        save_failed.set(false);
        let base_url = save_url.clone();
        let bearer = save_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => match kind {
                    YouEditor::Medication => api.add_medication(id, &name.unwrap()).await,
                    YouEditor::Note => api.add_note(id, &title.unwrap(), &body.unwrap()).await,
                },
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(()) => {
                    match kind {
                        YouEditor::Medication => {
                            medication.set(String::new());
                            medication_id.set(None);
                        }
                        YouEditor::Note => {
                            note_title.set(String::new());
                            note_body.set(String::new());
                            note_id.set(None);
                        }
                    }
                    save_failed.set(false);
                    editor.set(None);
                    attempt.set(attempt() + 1);
                }
                Err(_) => save_failed.set(true),
            }
            saving.set(false);
        });
    };
    rsx! {
        TabHeading { name: "You".to_owned(), loading }
        if let Some(kind) = editor() {
            HealthDraft {
                kind,
                medication,
                note_title,
                note_body,
                saving,
                failed: save_failed,
                on_close: move |_| {
                    save_failed.set(false);
                    editor.set(None);
                },
                on_save: save_record,
            }
        } else if menu() {
            button {
                class: "duet-retry",
                r#type: "button",
                onclick: move |_| {
                    menu.set(false);
                    editor.set(Some(YouEditor::Medication));
                },
                "New medication"
            }
            button {
                class: "duet-retry",
                r#type: "button",
                onclick: move |_| {
                    menu.set(false);
                    editor.set(Some(YouEditor::Note));
                },
                "New note"
            }
            button {
                class: "duet-retry",
                r#type: "button",
                onclick: move |_| menu.set(false),
                "Close"
            }
        } else {
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
                        for record in section.records {
                            p { class: "card-text", "{record.title}" }
                            if let Some(summary) = record.summary {
                                p { class: "record-meta", "{summary}" }
                            }
                            if let Some(when) = record.when {
                                p { class: "record-meta", "{when}" }
                            }
                            if let Some(source) = record.source {
                                p { class: "record-meta", "{source}" }
                            }
                        }
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
        button {
            class: "duet-retry",
            r#type: "button",
            onclick: move |_| menu.set(true),
            "Add health information"
        }
        }
    }
}

#[component]
fn PlanEditor(
    title: Signal<String>,
    body: Signal<String>,
    saving: Signal<bool>,
    failed: Signal<bool>,
    on_save: EventHandler,
) -> Element {
    let ready = !title().trim().is_empty() && !body().trim().is_empty();
    rsx! {
        input {
            class: "duet-field",
            r#type: "text",
            value: "{title}",
            oninput: move |event| title.set(event.value()),
        }
        textarea {
            class: "duet-field",
            value: "{body}",
            oninput: move |event| body.set(event.value()),
        }
        if failed() {
            p { class: "empty", role: "alert", "Your changes haven’t been confirmed. Your words are kept; refresh to check the saved version." }
        }
        button {
            class: "duet-send",
            r#type: "button",
            disabled: saving() || !ready,
            onclick: move |_| on_save.call(()),
            "Save changes"
        }
    }
}

#[component]
fn HealthDraft(
    kind: YouEditor,
    medication: Signal<String>,
    note_title: Signal<String>,
    note_body: Signal<String>,
    saving: Signal<bool>,
    failed: Signal<bool>,
    on_close: EventHandler,
    on_save: EventHandler,
) -> Element {
    let ready = match kind {
        YouEditor::Medication => !medication().trim().is_empty(),
        YouEditor::Note => !note_title().trim().is_empty() && !note_body().trim().is_empty(),
    };
    rsx! {
        button {
            class: "duet-retry",
            r#type: "button",
            onclick: move |_| on_close.call(()),
            "Close"
        }
        match kind {
            YouEditor::Medication => rsx! {
                p { class: "status-detail", "Enter the medication as one descriptive line, including dose when you know it." }
                input {
                    class: "duet-field",
                    r#type: "text",
                    value: "{medication}",
                    placeholder: "Atorvastatin 20 mg",
                    oninput: move |event| medication.set(event.value()),
                }
                if failed() {
                    p { class: "empty", role: "alert", "This medication couldn’t be saved. Nothing was changed." }
                }
                button {
                    class: "duet-send",
                    r#type: "button",
                    disabled: saving() || !ready,
                    onclick: move |_| on_save.call(()),
                    "Add medication"
                }
            },
            YouEditor::Note => rsx! {
                input {
                    class: "duet-field",
                    r#type: "text",
                    value: "{note_title}",
                    placeholder: "What is this note about?",
                    oninput: move |event| note_title.set(event.value()),
                }
                textarea {
                    class: "duet-field",
                    value: "{note_body}",
                    oninput: move |event| note_body.set(event.value()),
                }
                if failed() {
                    p { class: "empty", role: "alert", "This note couldn’t be saved. Nothing was changed." }
                }
                button {
                    class: "duet-send",
                    r#type: "button",
                    disabled: saving() || !ready,
                    onclick: move |_| on_save.call(()),
                    "Save note"
                }
            },
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
