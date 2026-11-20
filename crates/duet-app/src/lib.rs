//! Duet's two tab roots, You and Shared. SwiftUI remains the shipping iPhone app.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

mod account;
#[cfg(target_os = "ios")]
mod apple_health;
mod duplicates;
mod export;
pub mod for_my_doctor;
mod records;
mod roots;
mod settings;
mod trends;
pub use account::{AppDeepLink, DuetRoot, InviteHandoff, PatientSession, parse_app_link};

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use dioxus::prelude::*;
use duet_client::DuetClient;
use duet_notify::{AlertSnapshot, AlertState, PlatformAlerts};
use roots::{
    ALLOW_NOTIFICATIONS, CONNECTING, LIBRARY_EMPTY, LIBRARY_UNAVAILABLE, LibraryKind,
    LibraryRecord, LibraryRoot, OPEN_NOTIFICATION_SETTINGS, RecordEditability, TRY_AGAIN,
    filter_library_root, library_root,
};
use ui::{ActionRow, Button, ButtonVariant, IconName, Notice, SearchField, TabBar, Tone, TryAgain};

const TABS: [&str; 2] = ["You", "Shared"];

const DUET_STYLES: &str = include_str!("../assets/duet.css");

pub use roots::{api_base_url, api_bearer};

#[component]
fn PrivateRoot(
    base_url: String,
    bearer: String,
    appearance: duet_client::identity::Appearance,
    pending_invite: Option<String>,
    on_open_settings: EventHandler<()>,
    on_clear_invite: EventHandler<()>,
) -> Element {
    let patient = use_context::<PatientSession>();
    let fmd = for_my_doctor::use_for_my_doctor_session(base_url.clone(), bearer.clone(), &patient);
    use_context_provider(|| fmd);
    let initial_tab = patient
        .retention
        .load_active_tab(&patient.fence)
        .ok()
        .flatten()
        .filter(|index| *index < TABS.len())
        .unwrap_or(0);
    let mut tab = use_signal(move || initial_tab);
    let mut alerts = use_signal(|| PlatformAlerts.snapshot());
    let tab_owner = patient.retention.clone();
    let tab_fence = patient.fence.clone();
    rsx! {
        ui::Stylesheet { app: DUET_STYLES }
        main { class: "duet theme-duet", "data-theme": "{appearance.as_str()}",
            div { class: "duet-page",
                ActionRow {
                    Button { variant: ButtonVariant::Quiet, compact: true, onclick: move |_| on_open_settings.call(()), "Settings" }
                }
                if let Some(invite) = pending_invite {
                    Notice {
                        tone: Tone::Primary,
                        title: "Practice invitation".to_owned(),
                        detail: Some(format!("Invitation {invite} is open in your Duet account.")),
                        ActionRow {
                            Button { compact: true, onclick: move |_| on_clear_invite.call(()), "Done" }
                        }
                    }
                }
                div { hidden: tab() != 0,
                    YouTab {
                        base_url: base_url.clone(),
                        bearer: bearer.clone(),
                        alerts: alerts(),
                        on_allow_alerts: move |_| {
                            PlatformAlerts.request_access();
                            alerts.set(PlatformAlerts.snapshot());
                        },
                        on_open_settings: move |_| PlatformAlerts.open_settings(),
                        on_open_shared: {
                            let tab_owner = tab_owner.clone();
                            let tab_fence = tab_fence.clone();
                            move |_| {
                                tab.set(1);
                                let _ = tab_owner.save_active_tab(&tab_fence, 1);
                            }
                        },
                    }
                }
                div { hidden: tab() != 1, SharedTab { base_url: base_url.clone(), bearer: bearer.clone() } }
            }
            TabBar {
                label: "Sections",
                labels: TABS.iter().map(|label| (*label).to_owned()).collect(),
                icons: vec![IconName::User, IconName::Users],
                selected: tab(),
                on_select: move |index| {
                    tab.set(index);
                    let _ = tab_owner.save_active_tab(&tab_fence, index);
                    if index == 0 {
                        alerts.set(PlatformAlerts.snapshot());
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
    let offline = try_consume_context::<account::AccountConnection>()
        .is_some_and(|conn| (conn.0)() == account::ConnectionStatus::Connecting);
    rsx! {
        div { class: "duet-heading",
            h1 { "{name}" }
            p { class: "duet-status", if waiting || offline { "{CONNECTING}" } }
        }
    }
}

fn open_client(base_url: String, bearer: String) -> Result<DuetClient, String> {
    if let Some(session) = try_consume_context::<PatientSession>() {
        if !session.retention.session().is_current(&session.fence) {
            return Err("Account session ended".into());
        }
        let client = DuetClient::new(base_url, session.access_token.peek().clone(), "0", "0.1.0")?;
        let sender = consume_context::<account::RefusalSender>();
        return Ok(client.bind_session(session.retention.session(), session.fence, sender.0));
    }
    DuetClient::new(base_url, bearer, "0", "0.1.0")
}

fn field_text(value: &Option<Option<String>>) -> String {
    value
        .as_ref()
        .and_then(|inner| inner.clone())
        .unwrap_or_default()
}

#[derive(Clone, Copy, PartialEq)]
enum YouEditor {
    Medication,
    Note,
}

#[derive(Clone)]
struct EvidenceEdit {
    id: String,
    kind: LibraryKind,
    title: String,
}

#[component]
fn SharedTab(base_url: String, bearer: String) -> Element {
    rsx! {
        for_my_doctor::SharedForMyDoctor { base_url, bearer }
    }
}

fn evidence_entity(kind: LibraryKind) -> duet_wire::models::evidence_item::Entity {
    match kind {
        LibraryKind::HealthEvidence => duet_wire::models::evidence_item::Entity::HealthEvidence,
        LibraryKind::ClinicalNote => duet_wire::models::evidence_item::Entity::ClinicalNote,
    }
}

#[component]
fn EvidenceRow(
    record: LibraryRecord,
    base_url: String,
    bearer: String,
    editing: Signal<Option<EvidenceEdit>>,
    notice: Signal<Option<String>>,
    attempt: Signal<u32>,
    saving: Signal<bool>,
    on_open: EventHandler<LibraryRecord>,
    on_open_trend: EventHandler<LibraryRecord>,
) -> Element {
    let title = record.title.clone();
    let summary = record.summary.clone();
    let when = record.when.clone();
    let source = record.source.clone();
    let status_line = record.status_line.clone();
    let can_modify = record.editability == RecordEditability::PatientEntered;
    let can_trend = trends::supports_trend(&record);
    let trend_label = format!("View trend for {title}");
    let open_label = format!("Open {title}");
    let correct_label = format!("Correct {title}");
    let delete_label = format!("Delete {title}");
    let kind = record.kind;
    let correct_id = record.id.clone();
    let correct_title = title.clone();
    let open_record = record.clone();
    let trend_record = record.clone();
    let mark_key = for_my_doctor::record_target_key(&record.id);
    let mark_req = for_my_doctor::mark_request_for_library_record(&record);
    let start_edit = move |_| {
        notice.set(None);
        editing.set(Some(EvidenceEdit {
            id: correct_id.clone(),
            kind,
            title: correct_title.clone(),
        }));
    };
    let delete_id = record.id;
    let delete_url = base_url.clone();
    let delete_bearer = bearer.clone();
    let remove = move |_| {
        if saving() {
            return;
        }
        saving.set(true);
        notice.set(None);
        let id = delete_id.clone();
        let base_url = delete_url.clone();
        let bearer = delete_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.delete_evidence(&id, evidence_entity(kind)).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(()) => {
                    notice.set(Some("Removed from your records.".to_owned()));
                    attempt.set(attempt() + 1);
                }
                Err(_) => {
                    notice.set(Some("This record couldn’t be removed.".to_owned()));
                }
            }
            saving.set(false);
        });
    };
    rsx! {
        p { class: "card-text", "{title}" }
        if let Some(summary) = summary {
            p { class: "record-meta", "{summary}" }
        }
        if let Some(when) = when {
            p { class: "record-meta", "{when}" }
        }
        if let Some(source) = source {
            p { class: "record-meta", "{source}" }
        }
        if let Some(status) = status_line {
            p { class: "record-meta", "{status}" }
        }
        ActionRow {
            for_my_doctor::MarkToggleButton {
                target_key: mark_key,
                request: mark_req,
                base_url: base_url.clone(),
                bearer: bearer.clone(),
            }
            Button {
                compact: true,
                aria_label: "{open_label}",
                onclick: move |_| on_open.call(open_record.clone()),
                "Open"
            }
            if can_trend {
                Button {
                    compact: true,
                    aria_label: "{trend_label}",
                    onclick: move |_| on_open_trend.call(trend_record.clone()),
                    "View trend"
                }
            }
            if can_modify {
                Button {
                    compact: true,
                    aria_label: "{correct_label}",
                    onclick: start_edit,
                    "Correct"
                }
                Button {
                    compact: true,
                    danger: true,
                    aria_label: "{delete_label}",
                    onclick: remove,
                    "Delete"
                }
            }
        }
    }
}

#[component]
fn PhoneHealthControl(base_url: String, attempt: Signal<u32>) -> Element {
    #[cfg(target_os = "ios")]
    return rsx! { apple_health::AppleHealthControl { base_url, refreshed: attempt } };
    #[cfg(target_os = "android")]
    {
        let mut failed = use_signal(|| false);
        return rsx! { div { class: "source-row",
            span { class: "source-name", "Health Connect" }
            span { class: "source-detail", "You can set phone permissions. Health Connect imports are unavailable in this build." }
            Button { onclick: move |_| failed.set(duet_client::request_health_connect_permission().is_err()), "Allow" }
            if failed() { p { role: "alert", "Could not open Health Connect permissions." } }
        } };
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    rsx! {}
}

#[component]
fn YouTab(
    base_url: String,
    bearer: String,
    alerts: AlertSnapshot,
    on_allow_alerts: EventHandler<()>,
    on_open_settings: EventHandler<()>,
    on_open_shared: EventHandler<()>,
) -> Element {
    let mut attempt = use_signal(|| 0u32);
    let mut loading = use_signal(|| true);
    let patient = use_context::<PatientSession>();
    let cached_you = patient
        .retention
        .load_you_cache(&patient.fence)
        .ok()
        .flatten();
    let mut cache = use_signal(move || cached_you);
    let mut menu = use_signal(|| false);
    let mut editor = use_signal(|| None::<YouEditor>);
    let mut medication = use_signal(String::new);
    let mut note_title = use_signal(String::new);
    let mut note_body = use_signal(String::new);
    let mut medication_id = use_signal(|| None::<uuid::Uuid>);
    let mut note_id = use_signal(|| None::<uuid::Uuid>);
    let mut saving = use_signal(|| false);
    let mut save_failed = use_signal(|| false);
    let mut editing = use_signal(|| None::<EvidenceEdit>);
    let mut evidence_notice = use_signal(|| None::<String>);
    let mut search_query = use_signal(String::new);
    let mut open_record = use_signal(|| None::<LibraryRecord>);
    let mut active_trend = use_signal(|| None::<trends::TrendView>);
    let mut review_duplicates = use_signal(|| false);
    let hidden_duplicate_ids = use_signal(std::collections::HashSet::<String>::new);
    let reviewed_duplicate_pairs = use_signal(std::collections::HashSet::<String>::new);
    let save_url = base_url.clone();
    let save_bearer = bearer.clone();
    let evidence_url = base_url.clone();
    let evidence_bearer = bearer.clone();
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
    let you_retention = patient.retention.clone();
    let you_fence = patient.fence.clone();
    use_effect(move || {
        loading.set(resource().is_none());
        if let Some(Ok(page)) = resource() {
            let _ = you_retention.save_you_cache(&you_fence, &page);
            cache.set(Some(page));
        }
    });
    let current = resource();
    let failed = matches!(current, Some(Err(_)));
    let page = match current {
        Some(Ok(page)) => Some(page),
        _ => cache(),
    };
    let page_entries = page.as_ref().map(|p| p.entries.clone()).unwrap_or_default();
    let likely_duplicates = duplicates::find_likely_duplicates(&page_entries);
    let pending_duplicate_count = likely_duplicates
        .iter()
        .filter(|pair| !reviewed_duplicate_pairs().contains(&pair.pair_key))
        .count();
    let root = page
        .as_ref()
        .map(library_root)
        .map(|base| filter_library_root(&base, &search_query(), &hidden_duplicate_ids()));
    let trend_url = evidence_url.clone();
    let trend_bearer = evidence_bearer.clone();
    let trend_entries = page_entries.clone();
    let open_trend = move |record: LibraryRecord| {
        let entries = trend_entries.clone();
        let base_url = trend_url.clone();
        let bearer = trend_bearer.clone();
        evidence_notice.set(None);
        spawn(async move {
            let browse = match open_client(base_url, bearer) {
                Ok(api) => api.all_records().await,
                Err(error) => Err(error),
            };
            match browse {
                Ok(browse) => {
                    if let Some(view) = trends::build_trend_view(&record, &entries, &browse) {
                        active_trend.set(Some(view));
                    } else {
                        evidence_notice
                            .set(Some("No numeric trend readings available.".to_owned()));
                    }
                }
                Err(_) => {
                    evidence_notice
                        .set(Some("Trend readings couldn’t load. Try again.".to_owned()));
                }
            }
        });
    };
    let retry = move |_| attempt.set(attempt() + 1);
    // Saving shares the closure shapes with the other save handlers.
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
    let correction_url = evidence_url.clone();
    let correction_bearer = evidence_bearer.clone();
    let save_correction = move |_| {
        let Some(current) = editing() else {
            return;
        };
        if current.title.trim().is_empty() || saving() {
            return;
        }
        saving.set(true);
        evidence_notice.set(None);
        let base_url = correction_url.clone();
        let bearer = correction_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => {
                    api.correct_evidence(
                        &current.id,
                        evidence_entity(current.kind),
                        current.title.trim(),
                    )
                    .await
                }
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(()) => {
                    evidence_notice.set(Some("Saved the correction.".to_owned()));
                    editing.set(None);
                    attempt.set(attempt() + 1);
                }
                Err(_) => {
                    evidence_notice.set(Some("This correction couldn’t be saved.".to_owned()));
                }
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
            ActionRow {
                Button {
                    onclick: move |_| {
                        menu.set(false);
                        editor.set(Some(YouEditor::Medication));
                    },
                    "New medication"
                }
                Button {
                    onclick: move |_| {
                        menu.set(false);
                        editor.set(Some(YouEditor::Note));
                    },
                    "New note"
                }
                Button {
                    onclick: move |_| menu.set(false),
                    "Close"
                }
            }
        } else {
        PhoneHealthControl { base_url: evidence_url.clone(), attempt }
        if alerts.state.shown() {
            div { class: "source-row",
                span { class: "source-name", "Notifications" }
                span { class: "source-detail", "{alerts.sentence()}" }
                if alerts.state.can_request() {
                    Button {
                        aria_label: "{ALLOW_NOTIFICATIONS}",
                        onclick: move |_| on_allow_alerts.call(()),
                        "Allow"
                    }
                }
            }
            if alerts.state == AlertState::Denied {
                Button {
                    aria_label: "{OPEN_NOTIFICATION_SETTINGS}",
                    onclick: move |_| on_open_settings.call(()),
                    "{OPEN_NOTIFICATION_SETTINGS}"
                }
            }
        }
        if let Some(edit) = editing() {
            input {
                class: "field duet-field",
                r#type: "text",
                aria_label: "Corrected value",
                value: "{edit.title}",
                oninput: move |event| {
                    if let Some(mut current) = editing() {
                        current.title = event.value();
                        editing.set(Some(current));
                    }
                },
            }
            ActionRow {
                Button {
                    variant: ButtonVariant::Primary,
                    aria_label: "Save correction",
                    disabled: saving() || edit.title.trim().is_empty(),
                    onclick: save_correction,
                    "Save correction"
                }
                Button {
                    onclick: move |_| editing.set(None),
                    "Cancel"
                }
            }
        }
        if let Some(notice) = evidence_notice() {
            p { class: "record-meta", role: "status", "{notice}" }
        }
        for_my_doctor::YouRemovalNotices {
            base_url: evidence_url.clone(),
            bearer: evidence_bearer.clone(),
        }
        if let Some(record) = open_record() {
            for_my_doctor::OpenRecordSheet {
                record,
                base_url: evidence_url.clone(),
                bearer: evidence_bearer.clone(),
                on_close: move |_| open_record.set(None),
                on_open_shared,
            }
        }
        if page.is_some() {
            SearchField {
                label: "Search You".to_owned(),
                value: search_query(),
                on_input: move |value| search_query.set(value),
                placeholder: "Search records, medications, labs, or sources".to_owned(),
                clear_label: "Clear search".to_owned(),
            }
        }
        if pending_duplicate_count > 0 {
            Button {
                aria_label: "Review possible duplicates",
                onclick: move |_| review_duplicates.set(true),
                "Review possible duplicates ({pending_duplicate_count})"
            }
        }
        if review_duplicates() {
            duplicates::DuplicateReviewSheet {
                duplicates: likely_duplicates.clone(),
                hidden_ids: hidden_duplicate_ids,
                reviewed_pairs: reviewed_duplicate_pairs,
                on_close: move |_| review_duplicates.set(false),
            }
        }
        if let Some(trend) = active_trend() {
            trends::TrendPanel {
                trend,
                base_url: evidence_url.clone(),
                bearer: evidence_bearer.clone(),
                on_close: move |_| active_trend.set(None),
            }
        }
        match root {
            Some(LibraryRoot::Empty) if !search_query().trim().is_empty() => rsx! {
                p { class: "empty", "No matching records." }
            },
            Some(LibraryRoot::Empty) => rsx! { p { class: "empty", "{LIBRARY_EMPTY}" } },
            Some(LibraryRoot::Sections(sections)) => rsx! {
                for section in sections {
                    article { class: if section.measured { "card number-card" } else { "card voice-card" },
                        p { class: "card-title", "{section.name}" }
                        for record in section.records {
                            EvidenceRow {
                                record: record.clone(),
                                base_url: evidence_url.clone(),
                                bearer: evidence_bearer.clone(),
                                editing,
                                notice: evidence_notice,
                                attempt,
                                saving,
                                on_open: move |rec| open_record.set(Some(rec)),
                                on_open_trend: open_trend.clone(),
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
                TryAgain { label: TRY_AGAIN, on_retry: retry }
            }
        }
        records::RecordBrowser { base_url: evidence_url.clone(), bearer: evidence_bearer.clone(), reload: attempt }
        Button {
            onclick: move |_| menu.set(true),
            "Add health information"
        }
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
        Button {
            variant: ButtonVariant::Quiet,
            compact: true,
            onclick: move |_| on_close.call(()),
            "Close"
        }
        match kind {
            YouEditor::Medication => rsx! {
                p { class: "status-detail", "Enter the medication as one descriptive line, including dose when you know it." }
                input {
                    class: "field duet-field",
                    r#type: "text",
                    aria_label: "Medication name",
                    value: "{medication}",
                    placeholder: "Atorvastatin 20 mg",
                    oninput: move |event| medication.set(event.value()),
                }
                if failed() {
                    p { class: "empty", role: "alert", "This medication couldn’t be saved. Nothing was changed." }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    disabled: saving() || !ready,
                    onclick: move |_| on_save.call(()),
                    "Add medication"
                }
            },
            YouEditor::Note => rsx! {
                input {
                    class: "field duet-field",
                    r#type: "text",
                    value: "{note_title}",
                    placeholder: "What is this note about?",
                    oninput: move |event| note_title.set(event.value()),
                }
                textarea {
                    class: "field duet-field",
                    value: "{note_body}",
                    oninput: move |event| note_body.set(event.value()),
                }
                if failed() {
                    p { class: "empty", role: "alert", "This note couldn’t be saved. Nothing was changed." }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    disabled: saving() || !ready,
                    onclick: move |_| on_save.call(()),
                    "Save note"
                }
            },
        }
    }
}
