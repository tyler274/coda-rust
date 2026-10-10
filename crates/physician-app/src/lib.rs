//! Physician shell: the shipping sidebar, Today, a patient page, and the reader.
//! The React app remains the shipping client. This client renders the same projections.
//! Book: `book/src/north-star.md`.
#![deny(missing_docs)]

mod icons;
mod shell;

use dioxus::prelude::*;
use icons::{Icon, IconName};
use physician_client::PhysicianClient;
use physician_wire::{PatientWorkspaceResponse, RecentConversation};
use shell::{
    ChatView, KeyInput, NavCommand, SynopsisPiece, TODAY_LOADING, TODAY_QUIET, TODAY_QUIET_DETAIL,
    TODAY_STALE, TODAY_UNAVAILABLE, TRY_AGAIN, TodayLine, TodayView, VisitLine, WORKSPACE_CLOSED,
    WORKSPACE_EMPTY, WORKSPACE_UNAVAILABLE, chat_view, connection_lines, format_calendar_date,
    format_clock, format_waited, keyboard_command, need_you_words, opens_reader, patient_visit,
    synopsis_pieces, today_lines, today_view, upcoming_visits, work_end_reason, work_heading,
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    Today,
    Updates,
    Workspace,
}

#[component]
pub fn PhysicianRoot(base_url: String, open_patient: String) -> Element {
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
    let mut collapsed = use_signal(|| false);
    let mut search_open = use_signal(|| false);
    let mut shortcuts_open = use_signal(|| false);
    let mut connections_open = use_signal(|| false);
    let mut draft = use_signal(String::new);
    let mut notice = use_signal(String::new);
    let mut reading = use_signal(|| None::<(String, String)>);
    let mut focus = use_signal(|| -1i32);
    let mut open_chat = use_signal(|| None::<String>);
    let mut moved = use_signal(|| false);
    let mut editing = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut today_attempt = use_signal(|| 0u32);
    let mut workspace_attempt = use_signal(|| 0u32);
    let mut today_cache = use_signal(|| None::<Vec<TodayLine>>);
    let mut workspace_cache = use_signal(|| None::<PatientWorkspaceResponse>);
    let client = use_signal(|| PhysicianClient::new(base_url));
    let health = use_resource(move || {
        let client = client.read().clone();
        async move { client.health().await.map_err(|_| ()) }
    });
    let today = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move { client.today().await.map_err(|_| ()) }
    });
    let patients = use_resource(move || {
        let client = client.read().clone();
        async move { client.workspaces().await.map_err(|_| ()) }
    });
    let practice = use_resource(move || {
        let client = client.read().clone();
        async move { client.practice_day().await.map_err(|_| ()) }
    });
    let recent = use_resource(move || {
        let client = client.read().clone();
        async move { client.recent_conversations().await.map_err(|_| ()) }
    });
    let updates = use_resource(move || {
        let client = client.read().clone();
        async move { client.updates().await.map_err(|_| ()) }
    });
    let found = use_resource(move || {
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
                Some(id) => client.workspace(&id).await.map(Some).map_err(|_| ()),
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
    let open_visit = day.and_then(|day| {
        selected
            .read()
            .as_deref()
            .and_then(|id| patient_visit(day, id))
    });
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
    rsx! {
        style { "{ui::STYLESHEET}{SHELL_STYLES}" }
        div {
            class: if collapsed() { "glass-app is-collapsed" } else { "glass-app" },
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
                    let client = client.read().clone();
                    let text = draft();
                    spawn(async move {
                        notice.set(command_message(&client, "message.submit", &text).await);
                    });
                    return;
                }
                let targets = focus_targets(
                    search_open(),
                    place(),
                    &today_state,
                    &visits,
                    open_visit.clone(),
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
                            onclick: move |_| collapsed.set(true),
                            Icon { name: IconName::SidebarCollapse }
                        }
                        button {
                            class: "icon-button sidebar-expand",
                            r#type: "button",
                            aria_label: "Expand navigation",
                            onclick: move |_| collapsed.set(false),
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
                    }
                }
            }
            main { class: "glass-stage",
                button {
                    class: "stage-minimize",
                    r#type: "button",
                    onclick: move |_| collapsed.set(true),
                    "Collapse Coda"
                }
                match place() {
                    Place::Today => rsx! {
                        TodaySurface {
                            state: today_state.clone(),
                            connections: connections.clone(),
                            visits: visits.clone(),
                            server_now: server_now.clone(),
                            need_you,
                            focused: shown_focus,
                            on_retry: move |_| today_attempt.set(today_attempt() + 1),
                            on_open_row: move |line: TodayLine| {
                                let reading_text = if opens_reader(&line.kind) {
                                    Some((line.patient.clone(), line.reason.clone()))
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
                            visit: open_visit.clone(),
                            chats: chats.clone(),
                            server_now: server_now.clone(),
                            draft: draft(),
                            notice: notice(),
                            on_retry: move |_| workspace_attempt.set(workspace_attempt() + 1),
                            on_read: move |(title, body): (String, String)| reading.set(Some((title, body))),
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
                                let client = client.read().clone();
                                let text = draft();
                                spawn(async move {
                                    notice.set(command_message(&client, "message.submit", &text).await);
                                });
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
        if let Some((title, body)) = reading() {
            ReaderDialog {
                title,
                body,
                on_close: move |_| reading.set(None),
            }
        }
    }
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
    server_now: String,
    need_you: usize,
    focused: Option<i32>,
    on_retry: EventHandler,
    on_open_row: EventHandler<TodayLine>,
    on_open_visit: EventHandler<VisitLine>,
    on_resume: EventHandler<TodayLine>,
) -> Element {
    let words = need_you_words(need_you);
    let scheduled = !visits.is_empty();
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
                if !visits.is_empty() {
                    ScheduleRail { visits: visits.clone(), focused: visit_focus, on_open: on_open_visit }
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
    focused: Option<usize>,
    on_open: EventHandler<VisitLine>,
) -> Element {
    let count = visits.len();
    rsx! {
        aside { class: "schedule-rail", aria_label: "Schedule",
            header { class: "card-header",
                span { class: "icon-tile", aria_hidden: "true",
                    Icon { name: IconName::Calendar }
                }
                div { class: "card-title",
                    h2 { "Schedule" }
                    span { class: "count", span { class: "count-value", "{count}" } " left" }
                }
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
                    strong { "{visit.patient}" }
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
    visit: Option<VisitLine>,
    chats: ChatView,
    server_now: String,
    draft: String,
    notice: String,
    on_retry: EventHandler,
    on_read: EventHandler<(String, String)>,
    on_draft: EventHandler<String>,
    on_save: EventHandler,
    on_send: EventHandler,
    on_dictate: EventHandler,
    on_editing: EventHandler<bool>,
    messages: Vec<String>,
    snapshot_failed: bool,
    on_open_chat: EventHandler<String>,
) -> Element {
    let identity = workspace.as_ref().and_then(|open| open.identity.clone());
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
    let dob = identity
        .as_ref()
        .and_then(|identity| identity.birth_date.clone())
        .map(|day| format_calendar_date(&day));
    let gender = identity.as_ref().and_then(|identity| {
        identity.sex_or_gender.as_ref().and_then(|gender| {
            gender.value.as_ref().map(|value| {
                let mut chars = value.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
        })
    });
    let age = identity
        .as_ref()
        .and_then(|identity| identity.age_years)
        .map(|years| format!("{years} years"));
    let mrn = identity.as_ref().and_then(|identity| {
        identity
            .medical_record_numbers
            .iter()
            .find_map(|number| number.value.clone())
    });
    rsx! {
        div { class: "reading-workspace patient-page",
            header { class: "patient-identity",
                h1 { class: "workspace-title", "{name}" }
                div { class: "patient-facts",
                    if let Some(age) = age {
                        span { "Age {age}" }
                    }
                    if let Some(dob) = dob {
                        span { "DOB {dob}" }
                    }
                    if let Some(gender) = gender {
                        span { "Gender {gender}" }
                    }
                    if let Some(mrn) = mrn {
                        span { "MRN {mrn}" }
                    }
                }
            }
            if !rows.is_empty() {
                TodayRows {
                    rows,
                    server_now,
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
                                let visit = visit.clone();
                                move |_| {
                                    let body = visit
                                        .as_ref()
                                        .and_then(|visit| visit.preparation.clone())
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
            if let Some(visit) = visit.clone() {
                section { class: "patient-section", aria_label: "Visit",
                    h2 { "Visit" }
                    button {
                        class: "schedule-visit",
                        r#type: "button",
                        onclick: move |_| {
                            let clock = format_clock(&visit.starts_at, &visit.time_zone).unwrap_or_default();
                            on_read.call((
                                format!("{} {clock}", visit.patient),
                                visit.preparation.clone().unwrap_or_default(),
                            ));
                        },
                        if let Some(clock) = format_clock(&visit.starts_at, &visit.time_zone) {
                            time { "{clock}" }
                        }
                        strong { "{visit.label}" }
                    }
                }
            }
            if !segments.is_empty() {
                section { class: "patient-section", aria_label: "Patient summary",
                    h2 { "Patient summary" }
                    div { class: "synopsis",
                        for piece in synopsis_pieces(&segments) {
                            match piece {
                                SynopsisPiece::Text(text) => rsx! { span { "{text}" } },
                                SynopsisPiece::Chips(chips) => rsx! {
                                    for chip in chips {
                                        span { class: "citation-chip", "[{chip.index}]" }
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

#[component]
fn ReaderDialog(title: String, body: String, on_close: EventHandler) -> Element {
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section { class: "dialog reader", aria_label: "Reader", onclick: move |event| event.stop_propagation(),
                h2 { "{title}" }
                if body.is_empty() {
                    p { "Nothing is open to read." }
                } else {
                    p { class: "reader-body", "{body}" }
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
    open_visit: Option<VisitLine>,
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
        if let Some(visit) = open_visit {
            targets.push(FocusTarget::Visit(visit));
        }
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
    reading: &mut Signal<Option<(String, String)>>,
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
                reading.set(Some((line.patient.clone(), line.reason.clone())));
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
            reading.set(Some((
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

async fn command_message(client: &PhysicianClient, command_type: &str, text: &str) -> String {
    match client
        .command(&serde_json::json!({ "type": command_type, "text": text }))
        .await
    {
        Ok(value) => value
            .get("message")
            .and_then(|message| message.as_str())
            .unwrap_or("Coda couldn’t accept that message.")
            .to_owned(),
        Err(_) => "Coda couldn’t accept that message.".to_owned(),
    }
}
