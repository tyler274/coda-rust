//! Physician shell: the shipping sidebar and Today, beside the practice schedule.
//! The React app remains the shipping client. Dictation and drafts stay there.

mod shell;

use dioxus::prelude::*;
use physician_client::PhysicianClient;
use physician_wire::{PatientWorkspaceResponse, RecentConversation};
use shell::{
    ChatView, TODAY_LOADING, TODAY_QUIET, TODAY_QUIET_DETAIL, TODAY_STALE, TODAY_UNAVAILABLE,
    TRY_AGAIN, TodayLine, TodayView, VisitLine, WORKSPACE_CLOSED, WORKSPACE_EMPTY,
    WORKSPACE_UNAVAILABLE, chat_view, connection_lines, format_clock, format_waited,
    need_you_words, today_lines, today_view, upcoming_visits,
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    Today,
    Updates,
    Workspace,
}

#[component]
pub fn PhysicianRoot(base_url: String) -> Element {
    let mut place = use_signal(|| Place::Today);
    let mut selected = use_signal(|| None::<String>);
    let mut collapsed = use_signal(|| false);
    let mut search_open = use_signal(|| false);
    let mut shortcuts_open = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut today_attempt = use_signal(|| 0u32);
    let mut workspace_attempt = use_signal(|| 0u32);
    let mut today_cache = use_signal(|| None::<Vec<TodayLine>>);
    let mut workspace_cache = use_signal(|| None::<PatientWorkspaceResponse>);
    let client = use_signal(|| PhysicianClient::new(base_url));
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
    let today_read = today.read();
    let today_cache_read = today_cache.read();
    let today_result = today_read
        .as_ref()
        .map(|result| result.as_ref().map_err(|_| ()));
    let today_state = today_view(today_cache_read.as_deref(), today_result);
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
    let recent_read = recent.read();
    rsx! {
        style { "{ui::STYLESHEET}{SHELL_STYLES}" }
        div { class: if collapsed() { "glass-app is-collapsed" } else { "glass-app" },
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
                            CollapseIcon {}
                        }
                        button {
                            class: "icon-button sidebar-expand",
                            r#type: "button",
                            aria_label: "Expand navigation",
                            onclick: move |_| collapsed.set(false),
                            ExpandIcon {}
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
                            if place() == Place::Today { HomeIcon {} } else { ArrowLeftIcon {} }
                            strong { class: "item-text", "Today" }
                        }
                        button {
                            class: "sidebar-nav-item",
                            r#type: "button",
                            aria_current: if place() == Place::Updates { "page" } else { "" },
                            onclick: move |_| place.set(Place::Updates),
                            MessageIcon {}
                            strong { class: "item-text", "Updates" }
                        }
                        button {
                            class: "sidebar-nav-item",
                            r#type: "button",
                            aria_label: "Find patient",
                            onclick: move |_| search_open.set(true),
                            SearchIcon {}
                            strong { class: "item-text", "Find patient" }
                            kbd { class: "key-hint", aria_hidden: "true", "⌘ K" }
                        }
                    }
                    div { class: "sidebar-rule" }
                    if place() == Place::Workspace {
                        section { class: "sidebar-patient-context", aria_label: "Patient navigation",
                            div { class: "sidebar-current-patient", "{open_name.clone().unwrap_or_default()}" }
                            div { class: "sidebar-section-heading", h2 { "Conversations" } }
                            match chats.clone() {
                                ChatView::Empty | ChatView::Closed => rsx! { p { class: "sidebar-empty", "{NO_CONVERSATIONS}" } },
                                ChatView::Pending => rsx! {},
                                ChatView::Unavailable => rsx! { p { class: "sidebar-loading", "{WORKSPACE_UNAVAILABLE}" } },
                                ChatView::Chats(titles) => rsx! {
                                    for title in titles {
                                        div { class: "sidebar-conversation-link", "{title}" }
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
                        button {
                            r#type: "button",
                            onclick: move |_| shortcuts_open.set(true),
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
                            connections,
                            visits: visits.clone(),
                            server_now: server_now.clone(),
                            need_you,
                            on_retry: move |_| today_attempt.set(today_attempt() + 1),
                            on_open: move |id: String| {
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
                    Place::Updates => rsx! { UpdatesSurface { read: updates.read().as_ref().map(|result| result.as_ref().map(|page| page.rows.len()).map_err(|_| ())) } },
                    Place::Workspace => rsx! {
                        WorkspaceSurface {
                            name: open_name.clone().unwrap_or_default(),
                            chats: chats.clone(),
                            on_retry: move |_| workspace_attempt.set(workspace_attempt() + 1),
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
                    workspace_attempt.set(workspace_attempt() + 1);
                },
            }
        }
        if shortcuts_open() {
            ShortcutsDialog { on_close: move |_| shortcuts_open.set(false) }
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
            span { class: "sidebar-recent-title", "{title}" }
            if let Some(patient) = conversation.patient_display_name {
                span { class: "sidebar-recent-patient", "{patient}" }
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
    on_retry: EventHandler,
    on_open: EventHandler<String>,
    on_resume: EventHandler<TodayLine>,
) -> Element {
    let words = need_you_words(need_you);
    let scheduled = !visits.is_empty();
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
                        TodayView::Quiet => rsx! {
                            div { class: "today-list today-quiet",
                                strong { "{TODAY_QUIET}" }
                                span { "{TODAY_QUIET_DETAIL}" }
                            }
                        },
                        TodayView::Stale(rows) => rsx! {
                            ErrorState { message: TODAY_STALE.to_owned() }
                            TextButton { label: TRY_AGAIN.to_owned(), quiet: true, on_press: on_retry }
                            TodayRows { rows, server_now: server_now.clone(), on_open, on_resume }
                        },
                        TodayView::Rows(rows) => rsx! { TodayRows { rows, server_now: server_now.clone(), on_open, on_resume } },
                    }
                }
                if !visits.is_empty() {
                    ScheduleRail { visits: visits.clone(), on_open }
                }
            }
        }
    }
}

#[component]
fn TodayRows(
    rows: Vec<TodayLine>,
    server_now: String,
    on_open: EventHandler<String>,
    on_resume: EventHandler<TodayLine>,
) -> Element {
    rsx! {
        div { class: "today-list",
            for row in rows {
                article { class: "today-row",
                    button {
                        class: "today-row-open",
                        r#type: "button",
                        onclick: {
                            let id = row.workspace_id.clone();
                            move |_| on_open.call(id.clone())
                        },
                        span { class: "today-row-line",
                            span { class: "today-row-heading",
                                span { class: "live-dot", aria_hidden: "true" }
                                strong { class: "today-row-who", "{row.patient}" }
                                span { class: "state-pill",
                                    svg { class: "icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                        path { d: "M12 3 2.8 19h18.4L12 3Z" }
                                        path { d: "M12 9v4" }
                                        path { d: "M12 16.5h.01" }
                                    }
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
fn ScheduleRail(visits: Vec<VisitLine>, on_open: EventHandler<String>) -> Element {
    let count = visits.len();
    rsx! {
        aside { class: "schedule-rail", aria_label: "Schedule",
            header { class: "card-header",
                div { class: "card-title",
                    h2 { "Schedule" }
                    span { class: "count", span { class: "count-value", "{count}" } " left" }
                }
            }
            for (index, visit) in visits.iter().enumerate() {
                button {
                    class: if index == 0 { "schedule-visit is-next" } else { "schedule-visit" },
                    r#type: "button",
                    onclick: {
                        let id = visit.workspace_id.clone();
                        move |_| {
                            if let Some(id) = id.clone() {
                                on_open.call(id);
                            }
                        }
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
fn UpdatesSurface(read: Option<Result<usize, ()>>) -> Element {
    rsx! {
        div { class: "reading-workspace",
            header { class: "today-heading", h1 { "Updates" } }
            match read {
                None => rsx! { EmptyState { message: UPDATES_LOADING.to_owned() } },
                Some(Err(())) => rsx! { ErrorState { message: UPDATES_UNAVAILABLE.to_owned() } },
                Some(Ok(0)) => rsx! {
                    div { class: "updates-empty",
                        h2 { "{UPDATES_EMPTY}" }
                        p { "{UPDATES_EMPTY_DETAIL}" }
                    }
                },
                Some(Ok(count)) => rsx! { p { "{count} from Coda today." } },
            }
        }
    }
}

#[component]
fn WorkspaceSurface(name: String, chats: ChatView, on_retry: EventHandler) -> Element {
    rsx! {
        div { class: "reading-workspace",
            h1 { class: "workspace-title", "{name}" }
            match chats {
                ChatView::Closed => rsx! { EmptyState { message: WORKSPACE_CLOSED.to_owned() } },
                ChatView::Pending => rsx! {},
                ChatView::Unavailable => rsx! {
                    ErrorState { message: WORKSPACE_UNAVAILABLE.to_owned() }
                    TextButton { label: TRY_AGAIN.to_owned(), quiet: true, on_press: on_retry }
                },
                ChatView::Empty => rsx! { EmptyState { message: WORKSPACE_EMPTY.to_owned() } },
                ChatView::Chats(titles) => rsx! {
                    for title in titles {
                        div { class: "sidebar-conversation-link", "{title}" }
                    }
                },
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
) -> Element {
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section { class: "dialog", aria_label: "Find patient", onclick: move |event| event.stop_propagation(),
                h2 { "Find patient" }
                input {
                    class: "dialog-search",
                    r#type: "search",
                    placeholder: "Find patient",
                    value: "{query}",
                    oninput: move |event| on_query.call(event.value()),
                }
                ul { class: "dialog-results",
                    for patient in results {
                        li {
                            button {
                                r#type: "button",
                                onclick: {
                                    let id = patient.patient_workspace_id.clone();
                                    move |_| on_open.call(id.clone())
                                },
                                "{patient.patient_display_name}"
                            }
                        }
                    }
                }
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

#[component]
fn HomeIcon() -> Element {
    rsx! { svg { class: "icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        path { d: "m3 11 9-8 9 8" }
        path { d: "M5 10v10h14V10" }
        path { d: "M9 20v-6h6v6" }
    } }
}

#[component]
fn ArrowLeftIcon() -> Element {
    rsx! { svg { class: "icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        path { d: "M19 12H5" }
        path { d: "m12 19-7-7 7-7" }
    } }
}

#[component]
fn MessageIcon() -> Element {
    rsx! { svg { class: "icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        path { d: "M21 15a4 4 0 0 1-4 4H8l-5 3V7a4 4 0 0 1 4-4h10a4 4 0 0 1 4 4Z" }
    } }
}

#[component]
fn SearchIcon() -> Element {
    rsx! { svg { class: "icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        circle { cx: "11", cy: "11", r: "7" }
        path { d: "m20 20-4-4" }
    } }
}

#[component]
fn CollapseIcon() -> Element {
    rsx! { svg { class: "icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        rect { x: "3", y: "4", width: "18", height: "16", rx: "2" }
        path { d: "M9 4v16M15 9l-3 3 3 3" }
    } }
}

#[component]
fn ExpandIcon() -> Element {
    rsx! { svg { class: "icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        rect { x: "3", y: "4", width: "18", height: "16", rx: "2" }
        path { d: "M9 4v16M13 9l3 3-3 3" }
    } }
}
