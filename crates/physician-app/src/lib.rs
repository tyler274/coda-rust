//! Physician shell: the shipping sidebar, Today, a patient page, and the reader.
//! The React app remains the shipping client. This client renders the same projections.
//! Book: `book/src/north-star.md`.
#![deny(missing_docs)]

mod icons;
mod shell;

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use icons::{Icon, IconName};
use physician_client::PhysicianClient;
use physician_wire::{
    PatientWorkspaceResponse, PracticeAccountSummary, RecentConversation, WhoamiResponse,
};
#[cfg(target_arch = "wasm32")]
use shell::workspace_event_rereads;
use shell::{
    ChatView, KeyInput, NavCommand, SourceChip, SynopsisPiece, TODAY_LOADING, TODAY_QUIET,
    TODAY_QUIET_DETAIL, TODAY_STALE, TODAY_UNAVAILABLE, TRY_AGAIN, TodayLine, TodayView, VisitLine,
    WORKSPACE_CLOSED, WORKSPACE_EMPTY, WORKSPACE_UNAVAILABLE, chat_view, chip_name,
    connection_lines, demographic_card_view, format_calendar_date, format_clock, format_waited,
    keyboard_command, need_you_words, opens_reader, patient_upcoming_visits, prepared_line,
    preview_paragraphs, schedule_empty_copy, shows_today_schedule, summary_times, synopsis_pieces,
    today_lines, today_view, upcoming_visits, work_end_reason, work_heading,
};
use ui::{EmptyState, ErrorState, TextButton};

const SHELL_STYLES: &str = include_str!("../assets/shell.css");
const UPDATES_LOADING: &str = "Loading updates…";
const UPDATES_EMPTY: &str = "Nothing from Coda yet today.";
const UPDATES_EMPTY_DETAIL: &str = "What Coda does for your patients today will appear here.";
const UPDATES_UNAVAILABLE: &str = "Coda couldn’t load Updates.";
const RECENT_NONE: &str = "Conversations with Coda will appear here.";
const RECENT_UNAVAILABLE: &str = "Coda couldn’t load recent conversations.";
const NO_CONVERSATIONS: &str = "No conversations yet.";
const NOT_SAVED: &str = "Not saved";
const CONVERSATION_UNAVAILABLE: &str = "Coda couldn’t open this conversation.";
const DICTATION_UNAVAILABLE: &str =
    "Dictation is unavailable. Check microphone support and server configuration.";
const CONNECTIONS: &str = "Connections";
const SIGN_IN_UNAVAILABLE: &str = "Coda couldn’t send a sign-in link right now.";
const SIGN_IN_INVALID: &str = "That sign-in code is no longer valid.";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    Today,
    Updates,
    Workspace,
}

#[cfg(target_arch = "wasm32")]
const SESSION_STORAGE_KEY: &str = "coda.physician.session";

#[cfg(target_arch = "wasm32")]
fn read_stored_session() -> Option<String> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window()?;
    let storage = js_sys::Reflect::get(&window, &"sessionStorage".into()).ok()?;
    if storage.is_null() || storage.is_undefined() {
        return None;
    }
    let get_item = js_sys::Reflect::get(&storage, &"getItem".into())
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    get_item
        .call1(&storage, &SESSION_STORAGE_KEY.into())
        .ok()?
        .as_string()
        .filter(|value| !value.trim().is_empty())
}

#[cfg(target_arch = "wasm32")]
fn write_stored_session(session: Option<&str>) {
    use wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(storage) = js_sys::Reflect::get(&window, &"sessionStorage".into()) else {
        return;
    };
    if storage.is_null() || storage.is_undefined() {
        return;
    }
    match session {
        Some(token) if !token.trim().is_empty() => {
            if let Ok(set_item) = js_sys::Reflect::get(&storage, &"setItem".into())
                && let Ok(func) = set_item.dyn_into::<js_sys::Function>()
            {
                let _ = func.call2(&storage, &SESSION_STORAGE_KEY.into(), &token.into());
            }
        }
        _ => {
            if let Ok(remove_item) = js_sys::Reflect::get(&storage, &"removeItem".into())
                && let Ok(func) = remove_item.dyn_into::<js_sys::Function>()
            {
                let _ = func.call1(&storage, &SESSION_STORAGE_KEY.into());
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn follow_workspace_events(
    base_url: &str,
    session: Option<String>,
    mut today_attempt: Signal<u32>,
    mut workspace_attempt: Signal<u32>,
    mut snapshot_attempt: Signal<u32>,
    selected: Signal<Option<String>>,
) {
    let url = format!(
        "{}/patient-workspaces/events?after=0",
        base_url.trim_end_matches('/')
    );
    use wasm_bindgen::JsCast;
    wasm_bindgen_futures::spawn_local(async move {
        let Some(window) = web_sys::window() else {
            return;
        };
        let init = web_sys::RequestInit::new();
        init.set_method("GET");
        if let Some(token) = session.as_deref().filter(|token| !token.is_empty())
            && let Ok(headers) = web_sys::Headers::new()
        {
            let _ = headers.set("Authorization", &format!("Bearer {token}"));
            init.set_headers(&headers);
        }
        let Ok(value) =
            wasm_bindgen_futures::JsFuture::from(window.fetch_with_str_and_init(&url, &init)).await
        else {
            return;
        };
        let Ok(response) = value.dyn_into::<web_sys::Response>() else {
            return;
        };
        if !response.ok() {
            return;
        }
        let event_stream = response
            .headers()
            .get("content-type")
            .ok()
            .flatten()
            .is_some_and(|kind| kind.contains("text/event-stream"));
        if !event_stream {
            return;
        }
        let Some(body) = response.body() else {
            return;
        };
        let reader = body.get_reader();
        let mut buffer = String::new();
        loop {
            let Ok(read) = js_sys::Reflect::get(&reader, &"read".into()) else {
                return;
            };
            let Ok(read) = read.dyn_into::<js_sys::Function>() else {
                return;
            };
            let Ok(promise) = read.call0(&reader) else {
                return;
            };
            let Ok(promise) = promise.dyn_into::<js_sys::Promise>() else {
                return;
            };
            let Ok(chunk) = wasm_bindgen_futures::JsFuture::from(promise).await else {
                today_attempt.set(today_attempt() + 1);
                workspace_attempt.set(workspace_attempt() + 1);
                return;
            };
            let done = js_sys::Reflect::get(&chunk, &"done".into())
                .ok()
                .and_then(|value| value.as_bool())
                .unwrap_or(true);
            if done {
                today_attempt.set(today_attempt() + 1);
                workspace_attempt.set(workspace_attempt() + 1);
                return;
            }
            let Some(bytes) = js_sys::Reflect::get(&chunk, &"value".into())
                .ok()
                .map(|value| js_sys::Uint8Array::new(&value).to_vec())
            else {
                return;
            };
            buffer.push_str(&String::from_utf8_lossy(&bytes));
            apply_event_frames(
                &mut buffer,
                &selected,
                &mut workspace_attempt,
                &mut snapshot_attempt,
            );
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn apply_event_frames(
    buffer: &mut String,
    selected: &Signal<Option<String>>,
    workspace_attempt: &mut Signal<u32>,
    snapshot_attempt: &mut Signal<u32>,
) {
    while let Some(at) = buffer.find("\n\n") {
        let frame: String = buffer.drain(..at + 2).collect();
        let mut name = "message".to_owned();
        let mut data = String::new();
        for line in frame.lines() {
            if let Some(value) = line.strip_prefix("event:") {
                name = value.trim().to_owned();
            } else if let Some(value) = line.strip_prefix("data:") {
                if !data.is_empty() {
                    data.push('\n');
                }
                data.push_str(value.trim_start());
            }
        }
        let open = selected.read().clone();
        if workspace_event_rereads(&name, &data, open.as_deref()) {
            workspace_attempt.set(workspace_attempt() + 1);
            snapshot_attempt.set(snapshot_attempt() + 1);
        }
    }
}

#[component]
pub fn PhysicianRoot(base_url: String, open_patient: String) -> Element {
    let mut signed_in = use_signal(|| true);
    let mut whoami_name = use_signal(String::new);
    let mut whoami = use_signal(|| None::<WhoamiResponse>);
    // The bearer session rides the client, so one client is built once.
    let client = use_signal(|| {
        let client = PhysicianClient::new(base_url.clone());
        #[cfg(target_arch = "wasm32")]
        if let Some(session) = read_stored_session() {
            client.session().set(session);
        }
        client
    });
    if !signed_in() {
        rsx! {
            style { "{ui::STYLESHEET}{SHELL_STYLES}" }
            SignInScreen {
                client,
                on_signed_in: move |who: Option<WhoamiResponse>| {
                    let name = who
                        .as_ref()
                        .map(|account| account.display_name.clone())
                        .unwrap_or_default();
                    whoami_name.set(name);
                    whoami.set(who);
                    signed_in.set(true);
                },
            }
        }
    } else {
        rsx! {
            PhysicianShell {
                base_url,
                open_patient,
                client,
                signed_in,
                whoami_name,
                whoami,
            }
        }
    }
}

#[component]
fn PhysicianShell(
    base_url: String,
    open_patient: String,
    client: Signal<PhysicianClient>,
    mut signed_in: Signal<bool>,
    mut whoami_name: Signal<String>,
    mut whoami: Signal<Option<WhoamiResponse>>,
) -> Element {
    let mut place = use_signal(|| {
        if open_patient.is_empty() {
            Place::Today
        } else {
            Place::Workspace
        }
    });
    let mut selected = use_signal(|| {
        if open_patient.is_empty() {
            None
        } else {
            Some(open_patient)
        }
    });
    let mut sidebar_collapsed = use_signal(|| false);
    let mut app_collapsed = use_signal(|| false);
    let mut schedule_collapsed = use_signal(|| false);
    let mut search_open = use_signal(|| false);
    let mut shortcuts_open = use_signal(|| false);
    let mut connections_open = use_signal(|| false);
    let mut draft = use_signal(String::new);
    let mut notice = use_signal(String::new);
    let mut reading = use_signal(|| None::<OpenReading>);
    let mut focus = use_signal(|| -1i32);
    let mut open_chat = use_signal(|| None::<String>);
    let mut moved = use_signal(|| false);
    let mut editing = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut today_attempt = use_signal(|| 0u32);
    let mut workspace_attempt = use_signal(|| 0u32);
    let snapshot_attempt = use_signal(|| 0u32);
    let mut today_cache = use_signal(|| None::<Vec<TodayLine>>);
    let mut workspace_cache = use_signal(|| None::<PatientWorkspaceResponse>);
    let mut target_physician = use_signal(String::new);
    let mut panel_busy = use_signal(|| false);
    let mut panel_notice = use_signal(String::new);
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let session = client.read().session().get();
        follow_workspace_events(
            &base_url,
            session,
            today_attempt,
            workspace_attempt,
            snapshot_attempt,
            selected,
        );
    });
    let _whoami_load = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move {
            if let Ok(account) = client.whoami().await {
                if target_physician.read().is_empty() {
                    target_physician.set(account.physician_id.clone());
                }
                whoami_name.set(account.display_name.clone());
                whoami.set(Some(account));
            }
        }
    });
    let health = use_resource(move || {
        let client = client.read().clone();
        async move { client.health().await.map_err(|_| ()) }
    });
    let today = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move {
            match client.today().await {
                Ok(response) => Ok(response),
                Err(physician_client::HttpError::Status { status: 401, .. }) => {
                    #[cfg(target_arch = "wasm32")]
                    write_stored_session(None);
                    signed_in.set(false);
                    whoami_name.set(String::new());
                    whoami.set(None);
                    Err(())
                }
                Err(_) => Err(()),
            }
        }
    });
    let patients = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move { client.workspaces().await.map_err(|_| ()) }
    });
    let practice = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move { client.practice_day().await.map_err(|_| ()) }
    });
    let recent = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move { client.recent_conversations().await.map_err(|_| ()) }
    });
    let updates = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move { client.updates().await.map_err(|_| ()) }
    });
    let found = use_resource(move || {
        let _attempt = today_attempt();
        let query = query();
        let client = client.read().clone();
        async move {
            if query.trim().is_empty() {
                return Ok(Vec::new());
            }
            client
                .search(&query)
                .await
                .map(|page| page.patients)
                .map_err(|_| ())
        }
    });
    let patient_today = use_resource(move || {
        let _attempt = workspace_attempt();
        let client = client.read().clone();
        let open_id = selected();
        async move {
            match open_id {
                Some(id) => client.today_for(&id).await.map(Some).map_err(|_| ()),
                None => Ok(None),
            }
        }
    });
    let workspace = use_resource(move || {
        let _attempt = workspace_attempt();
        let client = client.read().clone();
        let open_id = selected();
        async move {
            match open_id {
                Some(id) => match client.workspace(&id).await {
                    Ok(open) => Ok(Some(open)),
                    Err(physician_client::HttpError::Status { status: 401, .. }) => {
                        #[cfg(target_arch = "wasm32")]
                        write_stored_session(None);
                        signed_in.set(false);
                        whoami_name.set(String::new());
                        whoami.set(None);
                        workspace_cache.set(None);
                        Err(())
                    }
                    Err(physician_client::HttpError::Status { status: 404, .. }) => {
                        workspace_cache.set(None);
                        Err(())
                    }
                    Err(_) => Err(()),
                },
                None => Ok(None),
            }
        }
    });
    use_effect(move || {
        if let Some(Ok(list)) = today.read().as_ref() {
            today_cache.set(Some(today_lines(list)));
        }
    });
    use_effect(move || {
        if let Some(Ok(Some(open))) = workspace.read().as_ref() {
            if selected.read().as_deref() == Some(open.patient_workspace_id.as_str()) {
                workspace_cache.set(Some(open.clone()));
            }
        }
    });
    use_effect(move || {
        let current = selected();
        let cached = workspace_cache.read().clone();
        let same = cached
            .as_ref()
            .is_some_and(|open| Some(open.patient_workspace_id.as_str()) == current.as_deref());
        if !same {
            if open_chat.read().is_some() {
                open_chat.set(None);
            }
            return;
        }
        if open_chat.read().is_none() {
            if let Some(chat_id) =
                cached.and_then(|open| open.chats.first().map(|chat| chat.chat_id.clone()))
            {
                open_chat.set(Some(chat_id));
            }
        }
    });
    let today_read = today.read();
    let today_cache_read = today_cache.read();
    let today_result = today_read
        .as_ref()
        .map(|result| result.as_ref().map_err(|_| ()));
    let today_state = today_view(today_cache_read.as_deref(), today_result);
    let connection_accounts = health
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .and_then(|health| health.capabilities.as_ref())
        .is_some_and(|capabilities| capabilities.connection_accounts);
    let connections = today_read
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|list| connection_lines(&list.connections))
        .unwrap_or_default();
    let practice_read = practice.read();
    let day = practice_read
        .as_ref()
        .and_then(|result| result.as_ref().ok());
    let server_now = day.map(|day| day.current_time.clone()).unwrap_or_default();
    let practice_zone = day
        .map(|day| day.time_zone.clone())
        .unwrap_or_else(|| "America/New_York".to_owned());
    let visits = day.map(upcoming_visits).unwrap_or_default();
    let show_schedule = day.is_some_and(shows_today_schedule);
    let schedule_empty = day.map(schedule_empty_copy).unwrap_or("").to_owned();
    let need_you = match &today_state {
        TodayView::Rows(rows) | TodayView::Stale(rows) => rows.len(),
        _ => 0,
    };
    let cached_workspace = workspace_cache.read().clone();
    let workspace_read = workspace.read();
    let workspace_current = match workspace_read.as_ref() {
        Some(Ok(Some(open))) => Some(Ok(open)),
        Some(Ok(None)) => None,
        Some(Err(())) => cached_workspace
            .as_ref()
            .filter(|open| selected.read().as_deref() == Some(open.patient_workspace_id.as_str()))
            .map(Ok)
            .or(Some(Err(()))),
        None => cached_workspace
            .as_ref()
            .filter(|open| selected.read().as_deref() == Some(open.patient_workspace_id.as_str()))
            .map(Ok),
    };
    let chats = chat_view(
        selected.read().as_deref(),
        workspace_current.as_ref().map(|result| match result {
            Ok(open) => Ok(*open),
            Err(_) => Err(()),
        }),
    );
    let patient_list = patients
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|list| list.patients.clone())
        .unwrap_or_default();
    let open_name = cached_workspace
        .as_ref()
        .filter(|open| selected.read().as_deref() == Some(open.patient_workspace_id.as_str()))
        .map(|open| open.display_name.clone())
        .or_else(|| {
            patient_list
                .iter()
                .find(|patient| {
                    Some(patient.patient_workspace_id.as_str()) == selected.read().as_deref()
                })
                .map(|patient| patient.display_name.clone())
        });
    let open_workspace = cached_workspace
        .as_ref()
        .filter(|open| selected.read().as_deref() == Some(open.patient_workspace_id.as_str()))
        .cloned();
    let patient_rows = patient_today
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .and_then(|list| list.as_ref())
        .map(today_lines)
        .unwrap_or_default();
    let patient_visits = day
        .and_then(|day| {
            selected
                .read()
                .as_deref()
                .map(|id| patient_upcoming_visits(day, id))
        })
        .unwrap_or_default();
    let recent_read = recent.read();
    let search_rows = found
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .cloned()
        .unwrap_or_default();
    let modal = shortcuts_open() || connections_open();
    let shown_focus = if moved() { Some(focus()) } else { None };
    let snapshot = use_resource(move || {
        let _attempt = snapshot_attempt();
        let workspace_id = selected();
        let chat_id = open_chat();
        let client = client.read().clone();
        async move {
            match (workspace_id, chat_id) {
                (Some(workspace_id), Some(chat_id)) => client
                    .chat_snapshot(&workspace_id, &chat_id)
                    .await
                    .map(Some)
                    .map_err(|_| ()),
                _ => Ok(None),
            }
        }
    });
    let snapshot_lines = snapshot
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .and_then(|page| page.as_ref())
        .map(|page| {
            page.snapshot
                .conversation
                .iter()
                .filter_map(|turn| turn.body().map(str::to_owned))
                .collect::<Vec<_>>()
        });
    let shell_class = format!(
        "glass-app{}{}",
        if app_collapsed() { " is-collapsed" } else { "" },
        if sidebar_collapsed() {
            " has-collapsed-sidebar"
        } else {
            ""
        }
    );
    rsx! {
        style { "{ui::STYLESHEET}{SHELL_STYLES}" }
        if app_collapsed() {
            button {
                class: "coda-restore",
                r#type: "button",
                aria_label: "Open Coda",
                onclick: move |_| app_collapsed.set(false),
                img { class: "coda-logo", src: "/assets/coda-health-logo-transparent.png", alt: "" }
            }
        }
        div {
            class: "{shell_class}",
            aria_hidden: if app_collapsed() { "true" } else { "false" },
            tabindex: "0",
            onkeydown: move |event| {
                let key = event.key().to_string();
                if key == "Escape" {
                    event.prevent_default();
                    if search_open() {
                        search_open.set(false);
                        editing.set(false);
                    } else if shortcuts_open() {
                        shortcuts_open.set(false);
                    } else if connections_open() {
                        connections_open.set(false);
                    } else if reading.read().is_some() {
                        reading.set(None);
                    }
                    return;
                }
                if key == " " && event.modifiers().ctrl() && event.modifiers().shift() {
                    event.prevent_default();
                    let client = client.read().clone();
                    spawn(async move {
                        let _ = client.dictate().await;
                        notice.set(DICTATION_UNAVAILABLE.to_owned());
                    });
                    return;
                }
                if key == "Enter" && (event.modifiers().meta() || event.modifiers().ctrl()) {
                    event.prevent_default();
                    send_message(
                        client.read().clone(),
                        selected(),
                        open_chat(),
                        workspace_cache
                            .read()
                            .as_ref()
                            .and_then(|open| open.time_zone.clone()),
                        draft(),
                        notice,
                        snapshot_attempt,
                    );
                    return;
                }
                let rail = if place() == Place::Workspace {
                    patient_visits.clone()
                } else {
                    visits.clone()
                };
                let targets = focus_targets(
                    search_open(),
                    place(),
                    &today_state,
                    &rail,
                    &search_rows,
                );
                let Some(command) = keyboard_command(&KeyInput {
                    key,
                    meta: event.modifiers().meta(),
                    ctrl: event.modifiers().ctrl(),
                    alt: event.modifiers().alt(),
                    shift: event.modifiers().shift(),
                    editing: editing(),
                    modal,
                    item_focused: moved(),
                }) else {
                    return;
                };
                event.prevent_default();
                match command {
                    NavCommand::Search => search_open.set(true),
                    NavCommand::Shortcuts => shortcuts_open.set(true),
                    NavCommand::Next => {
                        moved.set(true);
                        let last = targets.len().saturating_sub(1) as i32;
                        let next = if focus() < 0 { 0 } else { focus() + 1 };
                        focus.set(next.min(last));
                    }
                    NavCommand::Previous => {
                        moved.set(true);
                        let last = targets.len().saturating_sub(1) as i32;
                        let next = if focus() < 0 { last } else { focus() - 1 };
                        focus.set(next.max(0));
                    }
                    NavCommand::Activate => {
                        let Some(target) = targets.get(focus().max(0) as usize) else { return };
                        open_target(target.clone(), &mut selected, &mut place, &mut search_open, &mut reading, &mut workspace_attempt);
                    }
                }
            },
            aside { class: "glass-sidebar",
                div { class: "sidebar-content",
                    div { class: "sidebar-brand-row",
                        button {
                            class: "sidebar-brand",
                            r#type: "button",
                            aria_label: "Coda Health",
                            onclick: move |_| place.set(Place::Today),
                            img { class: "coda-logo", src: "/assets/coda-health-logo-transparent.png", alt: "Coda Health" }
                        }
                        button {
                            class: "icon-button sidebar-collapse",
                            r#type: "button",
                            aria_label: "Collapse navigation",
                            onclick: move |_| sidebar_collapsed.set(true),
                            Icon { name: IconName::SidebarCollapse }
                        }
                        button {
                            class: "icon-button sidebar-expand",
                            r#type: "button",
                            aria_label: "Expand navigation",
                            onclick: move |_| sidebar_collapsed.set(false),
                            Icon { name: IconName::SidebarExpand }
                        }
                    }
                    nav { class: "sidebar-primary", aria_label: "Primary",
                        button {
                            class: "sidebar-nav-item",
                            r#type: "button",
                            aria_label: "Open Today",
                            aria_current: if place() == Place::Today { "page" } else { "" },
                            onclick: move |_| {
                                place.set(Place::Today);
                            },
                            if place() == Place::Today {
                                Icon { name: IconName::Home }
                            } else {
                                Icon { name: IconName::ArrowLeft }
                            }
                            strong { class: "item-text", "Today" }
                        }
                        button {
                            class: "sidebar-nav-item",
                            r#type: "button",
                            aria_current: if place() == Place::Updates { "page" } else { "" },
                            onclick: move |_| place.set(Place::Updates),
                            Icon { name: IconName::Message }
                            strong { class: "item-text", "Updates" }
                        }
                        button {
                            class: "sidebar-nav-item",
                            r#type: "button",
                            aria_label: "Find patient",
                            onclick: move |_| search_open.set(true),
                            Icon { name: IconName::Search }
                            strong { class: "item-text", "Find patient" }
                            kbd { class: "key-hint", aria_hidden: "true", "⌘ K" }
                        }
                    }
                    div { class: "sidebar-rule" }
                    if place() == Place::Workspace {
                        section { class: "sidebar-patient-context", aria_label: "Patient navigation",
                            div { class: "sidebar-current-patient",
                                "{open_name.clone().unwrap_or_default()}"
                                Icon { name: IconName::ArrowRight }
                            }
                            if open_workspace.as_ref().is_some_and(PatientWorkspaceResponse::is_clinic_demographic) {
                                div { class: "sidebar-section-heading", h2 { "Clinic record" } }
                                p { class: "sidebar-empty", "Not on your panel" }
                            } else {
                                div { class: "sidebar-section-heading", h2 { "Conversations" } }
                                match chats.clone() {
                                    ChatView::Empty | ChatView::Closed => rsx! { p { class: "sidebar-empty", "{NO_CONVERSATIONS}" } },
                                    ChatView::Pending => rsx! {},
                                    ChatView::Unavailable => rsx! { p { class: "sidebar-loading", "{WORKSPACE_UNAVAILABLE}" } },
                                    ChatView::Chats(lines) => rsx! {
                                        for line in lines {
                                            button {
                                                class: "sidebar-conversation-link",
                                                r#type: "button",
                                                onclick: {
                                                    let id = line.id.clone();
                                                    move |_| open_chat.set(Some(id.clone()))
                                                },
                                                Icon { name: IconName::Message }
                                                "{line.title}"
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    } else {
                        section { class: "sidebar-recent", aria_labelledby: "sidebar-recent-heading",
                            div { class: "sidebar-section-heading", h2 { id: "sidebar-recent-heading", "Recent conversations" } }
                            match recent_read.as_ref() {
                                None => rsx! {},
                                Some(Err(())) => rsx! { p { class: "sidebar-loading", "{RECENT_UNAVAILABLE}" } },
                                Some(Ok(list)) if list.conversations.is_empty() => rsx! { p { class: "sidebar-empty", "{RECENT_NONE}" } },
                                Some(Ok(list)) => rsx! {
                                    for conversation in list.conversations.clone() {
                                        RecentRow { conversation }
                                    }
                                },
                            }
                        }
                    }
                    div { class: "sidebar-footer",
                        if !whoami_name().is_empty() {
                            span { class: "sidebar-physician", "{whoami_name}" }
                        }
                        if connection_accounts {
                            button {
                                r#type: "button",
                                onclick: move |_| connections_open.set(true),
                                Icon { name: IconName::Users }
                                "{CONNECTIONS}"
                            }
                        }
                        button {
                            r#type: "button",
                            onclick: move |_| shortcuts_open.set(true),
                            Icon { name: IconName::Keyboard }
                            "Keyboard shortcuts"
                        }
                        button {
                            r#type: "button",
                            aria_label: "Sign out of Coda",
                            onclick: move |_| {
                                let client = client.read().clone();
                                spawn(async move {
                                    let _ = client.sign_out().await;
                                    #[cfg(target_arch = "wasm32")]
                                    write_stored_session(None);
                                    signed_in.set(false);
                                    whoami_name.set(String::new());
                                    whoami.set(None);
                                });
                            },
                            Icon { name: IconName::Users }
                            "Sign out"
                        }
                    }
                }
            }
            main { class: "glass-stage",
                button {
                    class: "stage-minimize",
                    r#type: "button",
                    onclick: move |_| app_collapsed.set(true),
                    "Collapse Coda"
                }
                match place() {
                    Place::Today => rsx! {
                        TodaySurface {
                            state: today_state.clone(),
                            connections: connections.clone(),
                            visits: visits.clone(),
                            show_schedule,
                            schedule_empty: schedule_empty.clone(),
                            schedule_collapsed: schedule_collapsed(),
                            on_schedule_collapsed: move |collapsed| schedule_collapsed.set(collapsed),
                            server_now: server_now.clone(),
                            need_you,
                            focused: shown_focus,
                            on_retry: move |_| today_attempt.set(today_attempt() + 1),
                            on_open_row: move |line: TodayLine| {
                                let reading_text = if opens_reader(&line.kind) {
                                    Some(prose_reading(line.patient.clone(), line.reason.clone()))
                                } else {
                                    None
                                };
                                reading.set(reading_text);
                                selected.set(Some(line.workspace_id));
                                place.set(Place::Workspace);
                                search_open.set(false);
                                workspace_attempt.set(workspace_attempt() + 1);
                            },
                            on_open_visit: move |visit: VisitLine| {
                                let Some(id) = visit.workspace_id.clone() else { return };
                                selected.set(Some(id));
                                place.set(Place::Workspace);
                                search_open.set(false);
                                workspace_attempt.set(workspace_attempt() + 1);
                            },
                            on_resume: move |line: TodayLine| {
                                let client = client.read().clone();
                                let zone = practice_zone.clone();
                                spawn(async move {
                                    if let (Some(work), Some(run)) = (&line.resume_work_item_id, &line.resume_run_id) {
                                        let _ = client.resume_work(&line.workspace_id, work, run, &zone).await;
                                    }
                                    today_attempt.set(today_attempt() + 1);
                                });
                            },
                        }
                    },
                    Place::Updates => rsx! { UpdatesSurface { read: updates.read().as_ref().map(|result| result.as_ref().map(|page| page.rows.clone()).map_err(|_| ())) } },
                    Place::Workspace => rsx! {
                        PatientSurface {
                            name: open_name.clone().unwrap_or_default(),
                            workspace: open_workspace.clone(),
                            rows: patient_rows.clone(),
                            visits: patient_visits.clone(),
                            schedule_collapsed: schedule_collapsed(),
                            on_schedule_collapsed: move |collapsed| schedule_collapsed.set(collapsed),
                            focused: shown_focus,
                            chats: chats.clone(),
                            server_now: server_now.clone(),
                            draft: draft(),
                            notice: notice(),
                            is_admin: whoami.read().as_ref().is_some_and(|account| account.admin),
                            practice_accounts: whoami
                                .read()
                                .as_ref()
                                .map(|account| account.practice_accounts.clone())
                                .unwrap_or_default(),
                            target_physician_id: {
                                let chosen = target_physician();
                                if !chosen.is_empty() {
                                    chosen
                                } else {
                                    whoami
                                        .read()
                                        .as_ref()
                                        .map(|account| account.physician_id.clone())
                                        .unwrap_or_default()
                                }
                            },
                            panel_busy: panel_busy(),
                            panel_notice: panel_notice(),
                            on_select_physician: move |value: String| target_physician.set(value),
                            on_assign_panel: move |_| {
                                let Some(patient_id) = selected() else { return };
                                let physician_id = {
                                    let chosen = target_physician();
                                    if !chosen.is_empty() {
                                        chosen
                                    } else {
                                        whoami
                                            .read()
                                            .as_ref()
                                            .map(|account| account.physician_id.clone())
                                            .unwrap_or_default()
                                    }
                                };
                                if physician_id.is_empty() {
                                    return;
                                }
                                let client = client.read().clone();
                                spawn(async move {
                                    panel_busy.set(true);
                                    panel_notice.set(String::new());
                                    match client.assign_panel(&physician_id, &patient_id, true).await {
                                        Ok(_) => {
                                            workspace_attempt.set(workspace_attempt() + 1);
                                            today_attempt.set(today_attempt() + 1);
                                        }
                                        Err(_) => {
                                            panel_notice.set("Coda couldn’t update panel assignment.".to_owned());
                                        }
                                    }
                                    panel_busy.set(false);
                                });
                            },
                            on_remove_panel: move |_| {
                                let Some(patient_id) = selected() else { return };
                                let physician_id = {
                                    let chosen = target_physician();
                                    if !chosen.is_empty() {
                                        chosen
                                    } else {
                                        whoami
                                            .read()
                                            .as_ref()
                                            .map(|account| account.physician_id.clone())
                                            .unwrap_or_default()
                                    }
                                };
                                if physician_id.is_empty() {
                                    return;
                                }
                                let client = client.read().clone();
                                spawn(async move {
                                    panel_busy.set(true);
                                    panel_notice.set(String::new());
                                    match client.remove_panel(&physician_id, &patient_id).await {
                                        Ok(_) => {
                                            workspace_attempt.set(workspace_attempt() + 1);
                                            today_attempt.set(today_attempt() + 1);
                                        }
                                        Err(_) => {
                                            panel_notice.set("Coda couldn’t update panel assignment.".to_owned());
                                        }
                                    }
                                    panel_busy.set(false);
                                });
                            },
                            on_retry: move |_| workspace_attempt.set(workspace_attempt() + 1),
                            on_read: move |(title, body): (String, String)| {
                                reading.set(Some(prose_reading(title, body)));
                            },
                            on_open_source: move |chip: SourceChip| {
                                reading.set(Some(source_reading(chip)));
                            },
                            on_draft: move |value: String| draft.set(value),
                            messages: snapshot_lines.clone().unwrap_or_default(),
                            snapshot_failed: snapshot.read().as_ref().is_some_and(|result| result.is_err()),
                            on_open_chat: move |id: String| open_chat.set(Some(id)),
                            on_save: move |_| {
                                let Some(id) = selected() else { return };
                                let client = client.read().clone();
                                let text = draft();
                                let chat_id = open_chat();
                                spawn(async move {
                                    let saved = if let Some(chat_id) = chat_id {
                                        match client.read_draft(&id, &chat_id).await {
                                            Ok(existing) => {
                                                client
                                                    .save_draft(&id, &chat_id, existing.revision, &text)
                                                    .await
                                            }
                                            Err(error) => Err(error),
                                        }
                                    } else {
                                        client.save_draft(&id, "draft", 0, &text).await
                                    };
                                    notice.set(match saved {
                                        Ok(()) => String::new(),
                                        Err(_) => NOT_SAVED.to_owned(),
                                    });
                                });
                            },
                            on_send: move |_| {
                                send_message(
                                    client.read().clone(),
                                    selected(),
                                    open_chat(),
                                    workspace_cache
                                        .read()
                                        .as_ref()
                                        .and_then(|open| open.time_zone.clone()),
                                    draft(),
                                    notice,
                                    snapshot_attempt,
                                );
                            },
                            on_dictate: move |_| {
                                let client = client.read().clone();
                                spawn(async move {
                                    let _ = client.dictate().await;
                                    notice.set(DICTATION_UNAVAILABLE.to_owned());
                                });
                            },
                            on_editing: move |value: bool| editing.set(value),
                        }
                    },
                }
            }
        }
        if search_open() {
            SearchDialog {
                query: query(),
                results: found.read().as_ref().and_then(|result| result.as_ref().ok()).cloned().unwrap_or_default(),
                on_query: move |value: String| query.set(value),
                on_close: move |_| search_open.set(false),
                on_open: move |id: String| {
                    selected.set(Some(id));
                    place.set(Place::Workspace);
                    search_open.set(false);
                    editing.set(false);
                    workspace_attempt.set(workspace_attempt() + 1);
                },
                focused: if search_open() { shown_focus } else { None },
                on_editing: move |value: bool| editing.set(value),
            }
        }
        if shortcuts_open() {
            ShortcutsDialog { on_close: move |_| shortcuts_open.set(false) }
        }
        if connections_open() {
            ConnectionsDialog {
                lines: connections.clone(),
                failed: matches!(today_state, TodayView::Unavailable),
                on_close: move |_| connections_open.set(false),
            }
        }
        if let Some(open) = reading() {
            ReaderDialog {
                reading: open,
                on_close: move |_| reading.set(None),
            }
        }
    }
}

#[component]
fn SignInScreen(
    client: Signal<PhysicianClient>,
    on_signed_in: EventHandler<Option<WhoamiResponse>>,
) -> Element {
    let mut email = use_signal(String::new);
    let mut token = use_signal(String::new);
    let mut proof_token = use_signal(|| None::<String>);
    let mut stage = use_signal(|| SignInStage::Email);
    let mut error = use_signal(String::new);
    let mut busy = use_signal(|| false);

    let request_token = move |_: ()| {
        let client = client.read().clone();
        spawn(async move {
            busy.set(true);
            error.set(String::new());
            let email_value = email.read().trim().to_owned();
            match client.begin_sign_in(&email_value).await {
                Ok(answer) if answer.status == "token_issued" => {
                    proof_token.set(answer.token);
                    token.set(String::new());
                    stage.set(SignInStage::Verify);
                }
                Ok(answer) => {
                    error.set(
                        answer
                            .message
                            .unwrap_or_else(|| SIGN_IN_UNAVAILABLE.to_owned()),
                    );
                }
                Err(_) => error.set(SIGN_IN_UNAVAILABLE.to_owned()),
            }
            busy.set(false);
        });
    };
    let verify_token = move |_: ()| {
        let client = client.read().clone();
        spawn(async move {
            busy.set(true);
            error.set(String::new());
            let email_value = email.read().trim().to_owned();
            let token_value = token.read().trim().to_owned();
            match client.verify(&email_value, &token_value).await {
                Ok(_) => {
                    #[cfg(target_arch = "wasm32")]
                    write_stored_session(client.session().get().as_deref());
                    match client.whoami().await {
                        Ok(who) => on_signed_in.call(Some(who)),
                        Err(_) => on_signed_in.call(None),
                    }
                }
                Err(_) => error.set(SIGN_IN_INVALID.to_owned()),
            }
            busy.set(false);
        });
    };
    let proof_attr = proof_token().unwrap_or_default();
    rsx! {
        div { class: "sign-in-screen",
            section {
                class: "sign-in-card",
                aria_label: "Sign in to Coda",
                "data-proof-token": "{proof_attr}",
                h1 { "Sign in to Coda" }
                match stage() {
                    SignInStage::Email => rsx! {
                        p { class: "sign-in-sub", "We'll email you a single-use sign-in link." }
                        label { "Work email"
                            input {
                                class: "sign-in-input",
                                r#type: "email",
                                autocomplete: "email",
                                placeholder: "you@practice.test",
                                value: "{email}",
                                oninput: move |event| email.set(event.value()),
                                onkeydown: move |event| {
                                    if event.key().to_string() == "Enter" {
                                        request_token(());
                                    }
                                },
                            }
                        }
                        if !error().is_empty() {
                            p { class: "sign-in-error", "{error}" }
                        }
                        div { class: "sign-in-actions",
                            button {
                                class: "sign-in-primary",
                                r#type: "button",
                                disabled: busy() || email.read().trim().is_empty(),
                                onclick: move |_| request_token(()),
                                if busy() { "Sending…" } else { "Send sign-in link" }
                            }
                        }
                    },
                    SignInStage::Verify => rsx! {
                        p { class: "sign-in-sub", "Paste the code from the sign-in link we sent." }
                        label { "Sign-in code"
                            input {
                                class: "sign-in-input",
                                r#type: "text",
                                autocomplete: "one-time-code",
                                placeholder: "Paste the code",
                                value: "{token}",
                                oninput: move |event| token.set(event.value()),
                                onkeydown: move |event| {
                                    if event.key().to_string() == "Enter" {
                                        verify_token(());
                                    }
                                },
                            }
                        }
                        if !error().is_empty() {
                            p { class: "sign-in-error", "{error}" }
                        }
                        div { class: "sign-in-actions",
                            button {
                                class: "sign-in-primary",
                                r#type: "button",
                                disabled: busy() || token.read().trim().is_empty(),
                                onclick: move |_| verify_token(()),
                                if busy() { "Checking…" } else { "Sign in" }
                            }
                            button {
                                class: "control-secondary",
                                r#type: "button",
                                onclick: move |_| stage.set(SignInStage::Email),
                                "Use a different email"
                            }
                        }
                    },
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SignInStage {
    Email,
    Verify,
}

#[component]
fn RecentRow(conversation: RecentConversation) -> Element {
    let title = conversation
        .title
        .clone()
        .unwrap_or_else(|| "Untitled".to_owned());
    rsx! {
        div { class: "sidebar-recent-link",
            Icon { name: IconName::Message }
            span { class: "item-text",
                span { class: "sidebar-recent-title", "{title}" }
                if let Some(patient) = conversation.patient_display_name {
                    span { class: "sidebar-recent-patient", "{patient}" }
                }
            }
        }
    }
}

#[component]
fn TodaySurface(
    state: TodayView,
    connections: Vec<String>,
    visits: Vec<VisitLine>,
    show_schedule: bool,
    schedule_empty: String,
    schedule_collapsed: bool,
    on_schedule_collapsed: EventHandler<bool>,
    server_now: String,
    need_you: usize,
    focused: Option<i32>,
    on_retry: EventHandler,
    on_open_row: EventHandler<TodayLine>,
    on_open_visit: EventHandler<VisitLine>,
    on_resume: EventHandler<TodayLine>,
) -> Element {
    let words = need_you_words(need_you);
    let scheduled = show_schedule;
    let connections_quiet = connections.is_empty();
    let row_len = match &state {
        TodayView::Rows(rows) | TodayView::Stale(rows) => rows.len() as i32,
        _ => 0,
    };
    let row_focus = focused.and_then(|index| {
        if index >= 0 && index < row_len {
            Some(index as usize)
        } else {
            None
        }
    });
    let visit_focus = focused.and_then(|index| {
        let visit_index = index - row_len;
        if visit_index >= 0 {
            Some(visit_index as usize)
        } else {
            None
        }
    });
    rsx! {
        div { class: "reading-workspace",
            header { class: "today-heading",
                h1 { "Today" }
                if need_you > 0 {
                    span { class: "count", span { class: "count-value", "{need_you}" } " {words}" }
                }
            }
            div { class: if scheduled { "today-layout has-schedule" } else { "today-layout" },
                section { class: "today-main", aria_label: "Needs you",
                    for line in connections {
                        p { class: "today-connection", "{line}" }
                    }
                    match state {
                        TodayView::Loading => rsx! { EmptyState { message: TODAY_LOADING.to_owned() } },
                        TodayView::Unavailable => rsx! {
                            ErrorState { message: TODAY_UNAVAILABLE.to_owned() }
                            TextButton { label: TRY_AGAIN.to_owned(), quiet: true, on_press: on_retry }
                        },
                        TodayView::Quiet if connections_quiet => rsx! {
                            div { class: "today-list today-quiet",
                                strong { "{TODAY_QUIET}" }
                                span { "{TODAY_QUIET_DETAIL}" }
                            }
                        },
                        TodayView::Quiet => rsx! {},
                        TodayView::Stale(rows) => rsx! {
                            ErrorState { message: TODAY_STALE.to_owned() }
                            TextButton { label: TRY_AGAIN.to_owned(), quiet: true, on_press: on_retry }
                            TodayRows { rows, server_now: server_now.clone(), focused: row_focus, on_open: on_open_row, on_resume }
                        },
                        TodayView::Rows(rows) => rsx! { TodayRows { rows, server_now: server_now.clone(), focused: row_focus, on_open: on_open_row, on_resume } },
                    }
                }
                if show_schedule {
                    ScheduleRail {
                        visits: visits.clone(),
                        title: "Schedule",
                        empty: schedule_empty.clone(),
                        collapsed: schedule_collapsed,
                        on_collapsed: on_schedule_collapsed,
                        name_the_visit: false,
                        focused: visit_focus,
                        on_open: on_open_visit,
                    }
                }
            }
        }
    }
}

#[component]
fn TodayRows(
    rows: Vec<TodayLine>,
    server_now: String,
    focused: Option<usize>,
    on_open: EventHandler<TodayLine>,
    on_resume: EventHandler<TodayLine>,
) -> Element {
    rsx! {
        div { class: "today-list",
            for (index, row) in rows.iter().enumerate() {
                article { class: "today-row",
                    button {
                        class: if focused == Some(index) { "today-row-open is-keyboard-focus" } else { "today-row-open" },
                        r#type: "button",
                        onclick: {
                            let row = row.clone();
                            move |_| on_open.call(row.clone())
                        },
                        span { class: "today-row-line",
                            span { class: "today-row-heading",
                                span { class: "live-dot", aria_hidden: "true" }
                                strong { class: "today-row-who", "{row.patient}" }
                                span { class: "state-pill",
                                    Icon { name: IconName::Alert }
                                    "{row.did}"
                                }
                            }
                            if let Some(waited) = format_waited(&row.waiting_since, &server_now) {
                                time { class: "today-row-waited", datetime: "{row.waiting_since}", "{waited}" }
                            }
                        }
                        if !row.reason.is_empty() {
                            span { class: "today-row-reason", "{row.reason}" }
                        }
                    }
                    if row.resume_work_item_id.is_some() {
                        div { class: "today-row-action",
                            button {
                                class: "control-secondary",
                                r#type: "button",
                                onclick: {
                                    let row = row.clone();
                                    move |_| on_resume.call(row.clone())
                                },
                                "{TRY_AGAIN}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ScheduleRail(
    visits: Vec<VisitLine>,
    title: &'static str,
    empty: String,
    collapsed: bool,
    on_collapsed: EventHandler<bool>,
    name_the_visit: bool,
    focused: Option<usize>,
    on_open: EventHandler<VisitLine>,
) -> Element {
    let count = visits.len();
    let rail_class = if collapsed {
        "schedule-rail is-collapsed"
    } else {
        "schedule-rail"
    };
    rsx! {
        aside { class: "{rail_class}", aria_label: "{title}",
            if collapsed {
                button {
                    class: "schedule-rail-expand",
                    r#type: "button",
                    aria_expanded: "false",
                    aria_label: "Show {title}",
                    onclick: move |_| on_collapsed.call(false),
                    span { class: "icon-tile", aria_hidden: "true",
                        Icon { name: IconName::Calendar }
                    }
                    if count > 0 {
                        span { class: "count", span { class: "count-value", "{count}" } }
                    }
                }
            }
            div { class: "schedule-rail-body",
            header { class: "card-header",
                span { class: "icon-tile", aria_hidden: "true",
                    Icon { name: IconName::Calendar }
                }
                div { class: "card-title",
                    h2 { "{title}" }
                    if count > 0 {
                        span { class: "count", span { class: "count-value", "{count}" } " left" }
                    }
                }
                button {
                    class: "icon-button schedule-rail-collapse",
                    r#type: "button",
                    aria_expanded: "true",
                    aria_label: "Collapse {title}",
                    onclick: move |_| on_collapsed.call(true),
                    Icon { name: IconName::PanelRightClose }
                }
            }
            if visits.is_empty() {
                p { class: "schedule-rail-empty", "{empty}" }
            }
            for (index, visit) in visits.iter().enumerate() {
                button {
                    class: if focused == Some(index) {
                        "schedule-visit is-next is-keyboard-focus"
                    } else if index == 0 {
                        "schedule-visit is-next"
                    } else {
                        "schedule-visit"
                    },
                    r#type: "button",
                    onclick: {
                        let visit = visit.clone();
                        move |_| on_open.call(visit.clone())
                    },
                    if let Some(clock) = format_clock(&visit.starts_at, &visit.time_zone) {
                        time { datetime: "{visit.starts_at}", "{clock}" }
                    }
                    strong { if name_the_visit { "{visit.label}" } else { "{visit.patient}" } }
                    if name_the_visit {
                        if let Some(cue) = &visit.cue {
                            span { class: "schedule-visit-preparation", "{cue}" }
                        }
                    }
                }
            }
            }
        }
    }
}

#[component]
fn UpdatesSurface(read: Option<Result<Vec<serde_json::Value>, ()>>) -> Element {
    rsx! {
        div { class: "reading-workspace",
            header { class: "today-heading", h1 { "Updates" } }
            match read {
                None => rsx! { EmptyState { message: UPDATES_LOADING.to_owned() } },
                Some(Err(())) => rsx! { ErrorState { message: UPDATES_UNAVAILABLE.to_owned() } },
                Some(Ok(rows)) if rows.is_empty() => rsx! {
                    div { class: "updates-empty",
                        h2 { "{UPDATES_EMPTY}" }
                        p { "{UPDATES_EMPTY_DETAIL}" }
                    }
                },
                Some(Ok(rows)) => rsx! {
                    div { class: "today-list",
                        for row in rows {
                            article { class: "today-row",
                                strong { class: "today-row-who", "{row.get(\"patient_display_name\").and_then(|value| value.as_str()).unwrap_or(\"Update\")}" }
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn PatientSurface(
    name: String,
    workspace: Option<PatientWorkspaceResponse>,
    rows: Vec<TodayLine>,
    visits: Vec<VisitLine>,
    schedule_collapsed: bool,
    on_schedule_collapsed: EventHandler<bool>,
    focused: Option<i32>,
    chats: ChatView,
    server_now: String,
    draft: String,
    notice: String,
    is_admin: bool,
    practice_accounts: Vec<PracticeAccountSummary>,
    target_physician_id: String,
    panel_busy: bool,
    panel_notice: String,
    on_select_physician: EventHandler<String>,
    on_assign_panel: EventHandler,
    on_remove_panel: EventHandler,
    on_retry: EventHandler,
    on_read: EventHandler<(String, String)>,
    on_open_source: EventHandler<SourceChip>,
    on_draft: EventHandler<String>,
    on_save: EventHandler,
    on_send: EventHandler,
    on_dictate: EventHandler,
    on_editing: EventHandler<bool>,
    messages: Vec<String>,
    snapshot_failed: bool,
    on_open_chat: EventHandler<String>,
) -> Element {
    let clinic_demographic = workspace
        .as_ref()
        .is_some_and(PatientWorkspaceResponse::is_clinic_demographic);
    let demographic = workspace.as_ref().map(demographic_card_view);
    let segments = workspace
        .as_ref()
        .and_then(|open| open.patient_synopsis.as_ref())
        .map(|synopsis| synopsis.segments.clone())
        .unwrap_or_default();
    let work = workspace
        .as_ref()
        .and_then(|open| open.automation_run.clone());
    let follow_ups = workspace
        .as_ref()
        .map(|open| open.active_follow_ups.clone())
        .unwrap_or_default();
    let dob = demographic
        .as_ref()
        .and_then(|card| card.birth_date.clone());
    let gender = demographic
        .as_ref()
        .and_then(|card| card.sex_or_gender.clone());
    let age = demographic.as_ref().and_then(|card| card.age.clone());
    let mrn = demographic.as_ref().and_then(|card| card.mrn.clone());
    let record_state = demographic
        .as_ref()
        .and_then(|card| card.record_state.clone());
    let times = workspace
        .as_ref()
        .map(|open| summary_times(open, &server_now))
        .unwrap_or_default();
    let visit_focus = focused.and_then(|index| {
        if index >= 0 && (index as usize) < visits.len() {
            Some(index as usize)
        } else {
            None
        }
    });
    let scheduled = !visits.is_empty();
    let heading_name = demographic
        .as_ref()
        .map(|card| card.display_name.clone())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| name.clone());
    rsx! {
        div { class: "reading-workspace patient-page",
            header { class: "patient-identity",
                div { class: "patient-identity-top",
                    h1 { class: "workspace-title", "{heading_name}" }
                    if is_admin && workspace.is_some() && !clinic_demographic {
                        button {
                            class: "control-secondary remove-panel-button",
                            r#type: "button",
                            disabled: panel_busy,
                            onclick: move |_| on_remove_panel.call(()),
                            if panel_busy { "Updating…" } else { "Remove from panel" }
                        }
                    }
                }
                div { class: "patient-facts",
                    if let Some(age) = age.clone() {
                        span { "Age {age}" }
                    }
                    if let Some(dob) = dob.clone() {
                        span { "DOB {dob}" }
                    }
                    if let Some(gender) = gender.clone() {
                        span { "Gender {gender}" }
                    }
                    if let Some(mrn) = mrn.clone() {
                        span { "MRN {mrn}" }
                    }
                }
                if !clinic_demographic && !panel_notice.is_empty() {
                    span { class: "composer-notice", "{panel_notice}" }
                }
            }
            div { class: if scheduled { "today-layout has-schedule" } else { "today-layout" },
            div { class: "today-main",
            if clinic_demographic {
                section {
                    class: "patient-section clinic-demographic-card",
                    aria_label: "Clinic demographic record",
                    div { class: "clinic-demographic-header",
                        span { class: "clinic-demographic-badge", "Clinic record · Not on your panel" }
                        h2 { "Patient demographics" }
                        p { class: "clinic-demographic-copy",
                            "Basic clinic demographics are visible to physicians in this practice. Assign this patient to a physician panel to open clinical summaries, records, and conversations."
                        }
                    }
                    dl { class: "clinic-demographic-grid",
                        div { class: "clinic-demographic-field",
                            dt { "Full name" }
                            dd { "{heading_name}" }
                        }
                        if let Some(dob) = dob {
                            div { class: "clinic-demographic-field",
                                dt { "Date of birth" }
                                dd { "{dob}" }
                            }
                        }
                        if let Some(age) = age {
                            div { class: "clinic-demographic-field",
                                dt { "Age" }
                                dd { "{age}" }
                            }
                        }
                        if let Some(gender) = gender {
                            div { class: "clinic-demographic-field",
                                dt { "Sex or gender" }
                                dd { "{gender}" }
                            }
                        }
                        if let Some(mrn) = mrn {
                            div { class: "clinic-demographic-field",
                                dt { "Medical record number" }
                                dd { "{mrn}" }
                            }
                        }
                        if let Some(state) = record_state {
                            div { class: "clinic-demographic-field",
                                dt { "Record status" }
                                dd { "{state}" }
                            }
                        }
                    }
                    if is_admin {
                        div {
                            class: "panel-assignment-controls",
                            aria_label: "Panel assignment",
                            if practice_accounts.len() > 1 {
                                label { class: "panel-physician-label",
                                    "Physician panel"
                                    select {
                                        class: "panel-physician-select",
                                        aria_label: "Physician panel",
                                        value: "{target_physician_id}",
                                        onchange: move |event| on_select_physician.call(event.value()),
                                        for account in practice_accounts {
                                            option {
                                                value: "{account.physician_id}",
                                                "{account.display_name} ({account.email})"
                                            }
                                        }
                                    }
                                }
                            }
                            button {
                                class: "sign-in-primary assign-panel-button",
                                r#type: "button",
                                disabled: panel_busy,
                                onclick: move |_| on_assign_panel.call(()),
                                if panel_busy { "Assigning…" } else { "Assign to panel" }
                            }
                            if !panel_notice.is_empty() {
                                span { class: "composer-notice", "{panel_notice}" }
                            }
                        }
                    }
                }
            } else {
            if !rows.is_empty() {
                TodayRows {
                    rows,
                    server_now: server_now.clone(),
                    focused: None,
                    on_open: move |line: TodayLine| {
                        if opens_reader(&line.kind) {
                            on_read.call((line.patient.clone(), line.reason.clone()));
                        }
                    },
                    on_resume: move |_| {},
                }
            }
            if let Some(run) = work {
                if let Some(heading) = work_heading(&run) {
                    section { class: "patient-section", aria_label: "Current work",
                        h2 { "Current work" }
                        button {
                            class: "today-row-open",
                            r#type: "button",
                            onclick: {
                                let heading = heading.clone();
                                let preparation = visits
                                    .first()
                                    .and_then(|visit| visit.preparation.clone());
                                move |_| {
                                    let body = preparation
                                        .clone()
                                        .or_else(|| work_end_reason(run.ended_cause.as_deref()).map(str::to_owned))
                                        .unwrap_or_default();
                                    on_read.call((heading.clone(), body));
                                }
                            },
                            strong { "{heading}" }
                            if let Some(reason) = work_end_reason(run.ended_cause.as_deref()) {
                                span { class: "today-row-reason", "{reason}" }
                            }
                        }
                    }
                }
            }
            if !segments.is_empty() {
                section { class: "patient-section", aria_label: "Patient summary",
                    h2 { "Patient summary" }
                    if !times.is_empty() {
                        p { class: "patient-synopsis-meta",
                            for (index, mark) in times.iter().enumerate() {
                                if index > 0 {
                                    " · "
                                }
                                time { datetime: "{mark.instant}", "{mark.label} {mark.words}" }
                            }
                        }
                    }
                    div { class: "synopsis",
                        for piece in synopsis_pieces(&segments) {
                            match piece {
                                SynopsisPiece::Text(text) => rsx! { span { "{text}" } },
                                SynopsisPiece::Chips(chips) => rsx! {
                                    for chip in chips {
                                        CitationChip { chip: chip.clone(), on_open: on_open_source }
                                    }
                                },
                            }
                        }
                    }
                }
            }
            if !follow_ups.is_empty() {
                section { class: "patient-section", aria_label: "Follow-ups",
                    h2 { "Follow-ups" }
                    for follow_up in follow_ups {
                        p { "{follow_up.description.clone().unwrap_or_default()}" }
                    }
                }
            }
            section { class: "patient-section", aria_label: "Conversation",
                h2 { "Conversation" }
                match chats {
                    ChatView::Closed => rsx! { EmptyState { message: WORKSPACE_CLOSED.to_owned() } },
                    ChatView::Pending => rsx! {},
                    ChatView::Unavailable => rsx! {
                        ErrorState { message: WORKSPACE_UNAVAILABLE.to_owned() }
                        TextButton { label: TRY_AGAIN.to_owned(), quiet: true, on_press: on_retry }
                    },
                    ChatView::Empty => rsx! { p { class: "sidebar-empty", "{WORKSPACE_EMPTY}" } },
                    ChatView::Chats(lines) => rsx! {
                        for line in lines {
                            button {
                                class: "sidebar-conversation-link",
                                r#type: "button",
                                onclick: {
                                    let id = line.id.clone();
                                    move |_| on_open_chat.call(id.clone())
                                },
                                Icon { name: IconName::Message }
                                "{line.title}"
                            }
                        }
                        if snapshot_failed {
                            p { class: "sidebar-loading", "{CONVERSATION_UNAVAILABLE}" }
                        }
                        for message in messages {
                            p { class: "synopsis", "{message}" }
                        }
                        textarea {
                            class: "composer",
                            placeholder: "Write a message",
                            value: "{draft}",
                            onfocus: move |_| on_editing.call(true),
                            onblur: move |_| on_editing.call(false),
                            oninput: move |event| on_draft.call(event.value()),
                        }
                        div { class: "composer-actions",
                            button { class: "control-secondary", r#type: "button", onclick: move |_| on_save.call(()), "Save" }
                            button { class: "control-secondary", r#type: "button", onclick: move |_| on_send.call(()), "Send" }
                            button { class: "control-secondary", r#type: "button", onclick: move |_| on_dictate.call(()), "Dictate" }
                            if !notice.is_empty() {
                                span { class: "composer-notice", "{notice}" }
                            }
                        }
                    },
                }
            }
            }
            }
            if !visits.is_empty() {
                ScheduleRail {
                    visits: visits.clone(),
                    title: "Upcoming visits",
                    empty: String::new(),
                    collapsed: schedule_collapsed,
                    on_collapsed: on_schedule_collapsed,
                    name_the_visit: true,
                    focused: visit_focus,
                    on_open: move |visit: VisitLine| {
                        let clock = format_clock(&visit.starts_at, &visit.time_zone).unwrap_or_default();
                        on_read.call((
                            format!("{} {clock}", visit.label),
                            visit.preparation.clone().unwrap_or_default(),
                        ));
                    },
                }
            }
        }
        }
    }
}

#[component]
fn SearchDialog(
    query: String,
    results: Vec<physician_wire::PatientSearchRow>,
    on_query: EventHandler<String>,
    on_close: EventHandler,
    on_open: EventHandler<String>,
    on_editing: EventHandler<bool>,
    focused: Option<i32>,
) -> Element {
    let first = results
        .first()
        .map(|patient| patient.patient_workspace_id.clone());
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section { class: "dialog", aria_label: "Find patient", onclick: move |event| event.stop_propagation(),
                h2 { "Find patient" }
                input {
                    class: "dialog-search",
                    r#type: "search",
                    placeholder: "Name, DOB or phone",
                    value: "{query}",
                    autofocus: true,
                    onfocus: move |_| on_editing.call(true),
                    onblur: move |_| on_editing.call(false),
                    oninput: move |event| on_query.call(event.value()),
                    onkeydown: move |event| {
                        if event.key().to_string() == "Enter" {
                            event.stop_propagation();
                            if let Some(id) = first.clone() {
                                on_open.call(id);
                            }
                        }
                    },
                }
                ul { class: "dialog-results",
                    for (index, patient) in results.iter().enumerate() {
                        li {
                            button {
                                class: if focused == Some(index as i32) { "is-keyboard-focus" } else { "" },
                                r#type: "button",
                                onclick: {
                                    let id = patient.patient_workspace_id.clone();
                                    move |_| on_open.call(id.clone())
                                },
                                span { "{patient.patient_display_name}" }
                                span { class: "search-dob", "DOB {patient.birth_date.as_deref().map(format_calendar_date).unwrap_or_else(|| \"not recorded\".to_owned())}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ConnectionsDialog(lines: Vec<String>, failed: bool, on_close: EventHandler) -> Element {
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section { class: "dialog", aria_label: "Connections", onclick: move |event| event.stop_propagation(),
                h2 { "{CONNECTIONS}" }
                if failed {
                    p { "{TODAY_UNAVAILABLE}" }
                } else if lines.is_empty() {
                    p { "Nothing needs you right now." }
                } else {
                    for line in lines {
                        p { class: "today-connection", "{line}" }
                    }
                }
                button { class: "control-secondary", r#type: "button", onclick: move |_| on_close.call(()), "Close" }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct OpenReading {
    title: String,
    label: Option<String>,
    paragraphs: Vec<String>,
    provenance: Option<String>,
    link_label: Option<String>,
    link_uri: Option<String>,
}

fn prose_reading(title: String, body: String) -> OpenReading {
    OpenReading {
        title,
        label: None,
        paragraphs: if body.is_empty() {
            Vec::new()
        } else {
            vec![body]
        },
        provenance: None,
        link_label: None,
        link_uri: None,
    }
}

fn source_reading(chip: SourceChip) -> OpenReading {
    OpenReading {
        title: format!("Source {}", chip.index),
        label: Some(chip.label),
        paragraphs: preview_paragraphs(&chip.preview),
        provenance: prepared_line(&chip.as_of),
        link_label: chip.open_label,
        link_uri: chip.open_uri,
    }
}

#[component]
fn CitationChip(chip: SourceChip, on_open: EventHandler<SourceChip>) -> Element {
    let name = chip_name(&chip);
    let index = chip.index;
    if chip.open {
        rsx! {
            button {
                class: "citation-chip",
                r#type: "button",
                aria_label: "{name}",
                onclick: move |_| on_open.call(chip.clone()),
                "[{index}]"
            }
        }
    } else {
        rsx! {
            span { class: "citation-chip", aria_label: "{name}", "[{index}]" }
        }
    }
}

#[component]
fn ReaderDialog(reading: OpenReading, on_close: EventHandler) -> Element {
    let quiet =
        reading.paragraphs.is_empty() && reading.provenance.is_none() && reading.link_uri.is_none();
    let title = reading.title.clone();
    let label = reading.label.clone();
    let provenance = reading.provenance.clone();
    let link_label = reading.link_label.clone();
    let link_uri = reading.link_uri.clone();
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section { class: "dialog reader", aria_label: "Reader", onclick: move |event| event.stop_propagation(),
                h2 { "{title}" }
                if let Some(label) = label {
                    p { strong { "{label}" } }
                }
                if quiet {
                    p { "Nothing is open to read." }
                }
                for paragraph in reading.paragraphs {
                    p { class: "reader-body", "{paragraph}" }
                }
                if let Some(provenance) = provenance {
                    p { class: "reader-provenance", "{provenance}" }
                }
                if let (Some(link_label), Some(link_uri)) = (link_label, link_uri) {
                    a { href: "{link_uri}", target: "_blank", rel: "noreferrer", "{link_label}" }
                }
                button { class: "control-secondary", r#type: "button", onclick: move |_| on_close.call(()), "Close" }
            }
        }
    }
}

#[component]
fn ShortcutsDialog(on_close: EventHandler) -> Element {
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section { class: "dialog", aria_label: "Keyboard shortcuts", onclick: move |event| event.stop_propagation(),
                h2 { "Keyboard shortcuts" }
                div { class: "shortcut", span { "Search your panel" } kbd { "/  ⌘ K" } }
                div { class: "shortcut", span { "Move between patients or conversations" } kbd { "J  K" } }
                div { class: "shortcut", span { "Open the focused item" } kbd { "Enter" } }
                div { class: "shortcut", span { "Update the response in progress" } kbd { "⌘ Enter" } }
                div { class: "shortcut", span { "Close the current preview or dialog" } kbd { "Esc" } }
                div { class: "shortcut", span { "Hold to record; release to transcribe" } kbd { "Ctrl+Shift+Space" } }
                p { "Navigation shortcuts pause while you are typing. Dictation works in text fields while this tab has focus. Esc cancels dictation." }
                button { class: "control-secondary", r#type: "button", onclick: move |_| on_close.call(()), "Close" }
            }
        }
    }
}

#[derive(Clone)]
enum FocusTarget {
    Row(TodayLine),
    Visit(VisitLine),
    Patient(String),
}

fn focus_targets(
    search_open: bool,
    place: Place,
    today: &TodayView,
    visits: &[VisitLine],
    search_rows: &[physician_wire::PatientSearchRow],
) -> Vec<FocusTarget> {
    if search_open {
        return search_rows
            .iter()
            .map(|row| FocusTarget::Patient(row.patient_workspace_id.clone()))
            .collect();
    }
    let mut targets = Vec::new();
    if place == Place::Workspace {
        targets.extend(visits.iter().cloned().map(FocusTarget::Visit));
        return targets;
    }
    if place == Place::Today {
        if let TodayView::Rows(rows) | TodayView::Stale(rows) = today {
            targets.extend(rows.iter().cloned().map(FocusTarget::Row));
        }
        targets.extend(visits.iter().cloned().map(FocusTarget::Visit));
    }
    targets
}

fn open_target(
    target: FocusTarget,
    selected: &mut Signal<Option<String>>,
    place: &mut Signal<Place>,
    search_open: &mut Signal<bool>,
    reading: &mut Signal<Option<OpenReading>>,
    workspace_attempt: &mut Signal<u32>,
) {
    match target {
        FocusTarget::Patient(id) => {
            selected.set(Some(id));
            place.set(Place::Workspace);
            search_open.set(false);
            workspace_attempt.set(workspace_attempt() + 1);
        }
        FocusTarget::Row(line) => {
            if opens_reader(&line.kind) {
                reading.set(Some(prose_reading(
                    line.patient.clone(),
                    line.reason.clone(),
                )));
            }
            selected.set(Some(line.workspace_id));
            place.set(Place::Workspace);
            search_open.set(false);
            workspace_attempt.set(workspace_attempt() + 1);
        }
        FocusTarget::Visit(visit) => {
            let Some(id) = visit.workspace_id.clone() else {
                return;
            };
            let clock = format_clock(&visit.starts_at, &visit.time_zone).unwrap_or_default();
            reading.set(Some(prose_reading(
                format!("{} {clock}", visit.patient),
                visit.preparation.clone().unwrap_or_default(),
            )));
            selected.set(Some(id));
            place.set(Place::Workspace);
            search_open.set(false);
            workspace_attempt.set(workspace_attempt() + 1);
        }
    }
}

fn send_message(
    client: PhysicianClient,
    workspace_id: Option<String>,
    chat_id: Option<String>,
    time_zone: Option<String>,
    text: String,
    mut notice: Signal<String>,
    mut snapshot_attempt: Signal<u32>,
) {
    let (Some(workspace_id), Some(chat_id), Some(time_zone)) = (workspace_id, chat_id, time_zone)
    else {
        notice.set("Coda couldn’t accept that message.".to_owned());
        return;
    };
    spawn(async move {
        let message = command_message(&client, &workspace_id, &chat_id, &time_zone, &text).await;
        if message.is_empty() {
            snapshot_attempt.set(snapshot_attempt() + 1);
        }
        notice.set(message);
    });
}

async fn command_message(
    client: &PhysicianClient,
    workspace_id: &str,
    chat_id: &str,
    time_zone: &str,
    text: &str,
) -> String {
    match client
        .command(&serde_json::json!({
            "type": "message.submit",
            "idempotency_key": idempotency_key(),
            "payload": {
                "patient_workspace_id": workspace_id,
                "chat_id": chat_id,
                "text": text,
                "physician_time_zone": time_zone,
                "intent": "queue",
            }
        }))
        .await
    {
        Ok(value) => {
            if value.get("outcome").and_then(|item| item.as_str()) == Some("accepted") {
                String::new()
            } else {
                value
                    .get("message")
                    .and_then(|message| message.as_str())
                    .unwrap_or("Coda couldn’t accept that message.")
                    .to_owned()
            }
        }
        Err(_) => "Coda couldn’t accept that message.".to_owned(),
    }
}

fn idempotency_key() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    format!("submit-{millis}-{n}")
}
