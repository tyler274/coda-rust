//! Practice-facing clinical source connection, backlog acceptance, and recovery UI.
//!
//! Built strictly from `crates/ui` components (`Dialog`, `Card`, `StatePill`, `Notice`,
//! `RadioGroup`, `Checkbox`, `TextField`, `Button`, `ActionRow`, `StatusLine`) with zero
//! raw HTML controls or library CSS overrides.

use dioxus::prelude::*;
use physician_client::{HttpError, PhysicianClient};
use physician_wire::TodayConnection;
use physician_wire::source_setup::{
    ConnectSourceBody, PracticeConnectionsWire, PracticeSourceWire, RequestAssistedSetupBody,
    SourceConnectionMethodWire, SourceStandingWire, SpruceBacklogWire, SupportedSourceWire,
    TodayConnectionRecoveryWire,
};
use ui::{
    ActionRow, Button, ButtonVariant, Card, Checkbox, Choice, Dialog, IconName, Notice, RadioGroup,
    StatePill, StatusLine, TextField, TextKind, Tone,
};

use crate::shell::{format_clock, format_context_time, format_local_date};
use crate::{CONNECTIONS, SessionState, TRY_AGAIN, refusal};

const SOURCES_LOADING: &str = "Loading practice connections…";
const SOURCES_UNAVAILABLE: &str = "Coda couldn’t load practice connections.";
const SAVED_RECORDS_READABLE: &str = "Previously saved records remain readable.";

/// Exhaustive visual mapping for [`SourceStandingWire`].
///
/// `Connected`, `RecordsImported`, and `UpToDate` map to distinct badges so a
/// credential/session check never implies clinical records were imported or up to date.
pub(crate) fn standing_badge(standing: &SourceStandingWire) -> (Tone, IconName, &'static str) {
    match standing {
        SourceStandingWire::Disconnected => (Tone::Neutral, IconName::Link, "Disconnected"),
        SourceStandingWire::CodaAssistedRequested { .. } => {
            (Tone::Primary, IconName::Clock, "Setup requested")
        }
        SourceStandingWire::Connected { .. } => (Tone::Neutral, IconName::Check, "Connected"),
        SourceStandingWire::RecordsImported { .. } => {
            (Tone::Primary, IconName::File, "Records imported")
        }
        SourceStandingWire::UpToDate { .. } => (Tone::Primary, IconName::Check, "Up to date"),
        SourceStandingWire::ReadFailed { .. } => (Tone::Warning, IconName::Alert, "Update failed"),
        SourceStandingWire::AccessLost { .. } => (Tone::Error, IconName::Alert, "Access lost"),
    }
}

fn format_instant(iso: &str, zone: &str, now: &str) -> String {
    if let Some(contextual) = format_context_time(iso, zone, now)
        && (contextual.starts_with("Today,")
            || contextual.starts_with("Yesterday,")
            || contextual.starts_with("Tomorrow,"))
    {
        return contextual;
    }
    match (format_local_date(iso, zone), format_clock(iso, zone)) {
        (Some(date), Some(clock)) => format!("{date}, {clock}"),
        (Some(date), None) => date,
        _ => iso.to_owned(),
    }
}

fn format_last_read(timestamp: Option<&str>, zone: &str, now: &str) -> String {
    match timestamp {
        Some(iso) if !iso.trim().is_empty() => {
            format!("Last successful read {}", format_instant(iso, zone, now))
        }
        _ => "No completed read yet".to_owned(),
    }
}

fn method_choice(method: SourceConnectionMethodWire) -> Choice {
    let detail = match method {
        SourceConnectionMethodWire::BrowserSession => {
            "Verify an active signed-in Chrome connector session."
        }
        SourceConnectionMethodWire::CredentialSignIn => {
            "Authenticate directly with practice API credentials."
        }
        SourceConnectionMethodWire::SimulatedPractice => {
            "Walk through setup and recovery using synthetic practice records."
        }
        SourceConnectionMethodWire::CodaAssisted => {
            "Ask Coda staff to coordinate setup with your practice administrator."
        }
    };
    Choice::from((method.label(), detail))
}

fn error_message(error: &HttpError, fallback: &str) -> String {
    if let HttpError::Status { body, .. } = error
        && let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body)
        && let Some(detail) = parsed
            .get("detail")
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|text| !text.is_empty())
    {
        return detail.to_owned();
    }
    fallback.to_owned()
}

/// Filters `/today`'s `connections` down to structured practice source notices.
pub(crate) fn structured_today_connections(
    connections: &[TodayConnection],
) -> Vec<TodayConnection> {
    connections
        .iter()
        .filter(|item| item.recovery.is_some())
        .cloned()
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TodaySourceNoticeView {
    pub(crate) source_id: Option<SupportedSourceWire>,
    pub(crate) tone: Tone,
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) action_label: &'static str,
    pub(crate) primary_action: bool,
}

pub(crate) fn project_today_notice(
    item: &TodayConnection,
    zone: &str,
    now: &str,
) -> Option<TodaySourceNoticeView> {
    let recovery = item.recovery?;
    let source_id = item.source_id;
    let label = item
        .source_label
        .clone()
        .or_else(|| source_id.map(|kind| kind.label().to_owned()))
        .unwrap_or_else(|| "Clinical source".to_owned());
    let reason = item
        .reason
        .clone()
        .unwrap_or_else(|| "The source reported an error.".to_owned());
    let last_read = format_last_read(item.last_successful_read_at.as_deref(), zone, now);
    let detail = format!("{reason} · {last_read}. {SAVED_RECORDS_READABLE}");
    Some(match recovery {
        TodayConnectionRecoveryWire::ReadFailed => TodaySourceNoticeView {
            source_id,
            tone: Tone::Warning,
            title: format!("{label}: Records couldn’t be updated"),
            detail,
            action_label: "Inspect source",
            primary_action: false,
        },
        TodayConnectionRecoveryWire::ReconnectRequired => TodaySourceNoticeView {
            source_id,
            tone: Tone::Error,
            title: format!("{label}: Access lost"),
            detail,
            action_label: "Reconnect",
            primary_action: true,
        },
    })
}

/// Renders structured source failure and recovery notices on Today.
///
/// - [`TodayConnectionRecoveryWire::ReadFailed`] shows `'Records couldn’t be updated'`,
///   the failure reason, and last successful read time with an inspection button — never `Reconnect`.
/// - [`TodayConnectionRecoveryWire::ReconnectRequired`] shows `'Access lost'`, the specific
///   reason, last successful read time, and a `Reconnect` button that opens `Connections` on that source.
#[component]
pub(crate) fn TodaySourceConnections(
    connections: Vec<TodayConnection>,
    practice_zone: String,
    server_now: String,
    on_open_source: EventHandler<Option<SupportedSourceWire>>,
) -> Element {
    let views: Vec<TodaySourceNoticeView> = connections
        .iter()
        .filter_map(|item| project_today_notice(item, &practice_zone, &server_now))
        .collect();
    if views.is_empty() {
        return rsx! {};
    }
    rsx! {
        div { class: "ps-today-connections",
            for (idx, view) in views.into_iter().enumerate() {
                {
                    let source_id = view.source_id;
                    rsx! {
                        Notice {
                            key: "today-conn-{idx}",
                            tone: view.tone,
                            icon: IconName::Alert,
                            title: view.title,
                            detail: view.detail,
                            Button {
                                compact: true,
                                variant: if view.primary_action {
                                    ButtonVariant::Primary
                                } else {
                                    ButtonVariant::Secondary
                                },
                                onclick: move |_| on_open_source.call(source_id),
                                "{view.action_label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Modal dialog for setting up, inspecting, recovering, and disconnecting practice clinical sources.
#[component]
pub(crate) fn PracticeSourcesDialog(
    client: Signal<PhysicianClient>,
    session: SessionState,
    shown: bool,
    practice_zone: String,
    server_now: String,
    initial_source: Option<SupportedSourceWire>,
    on_changed: EventHandler,
    on_close: EventHandler,
) -> Element {
    let mut reload_tick = use_signal(|| 0u32);
    let mut active_source = use_signal(|| initial_source.unwrap_or(SupportedSourceWire::Hint));
    let mut state = use_signal(|| None::<Result<PracticeConnectionsWire, String>>);

    use_effect(move || {
        let _ = reload_tick();
        let api = client.read().clone();
        let presented = session.bearer();
        state.set(None);
        spawn(async move {
            match api.connections().await {
                Ok(payload) => state.set(Some(Ok(payload))),
                Err(err) => {
                    if let Some(why) = refusal(&err) {
                        session.refused(why, shown, presented);
                        return;
                    }
                    state.set(Some(Err(error_message(&err, SOURCES_UNAVAILABLE))));
                }
            }
        });
    });

    let current_state = state.read().clone();
    rsx! {
        Dialog {
            class: "ps-source-dialog",
            close_label: "Close",
            title: CONNECTIONS,
            subtitle: Some("Connect practice clinical sources, review historical Spruce messages, or recover interrupted access.".to_owned()),
            on_close,
            match current_state {
                None => rsx! {
                    StatusLine {
                        message: SOURCES_LOADING,
                        pending: true,
                    }
                },
                Some(Err(message)) => rsx! {
                    StatusLine {
                        message,
                        alert: true,
                        retry: TRY_AGAIN,
                        on_retry: move |_| reload_tick.set(reload_tick() + 1),
                    }
                },
                Some(Ok(connections)) => {
                    let chosen = active_source();
                    let selected_source = connections
                        .sources
                        .iter()
                        .find(|src| src.source_id == chosen)
                        .or_else(|| connections.sources.first())
                        .cloned();
                    rsx! {
                        div { class: "ps-source-stack",
                            ActionRow {
                                for src in &connections.sources {
                                    {
                                        let kind = src.source_id;
                                        let is_active = kind == chosen;
                                        let (_, _, badge_words) = standing_badge(&src.standing);
                                        rsx! {
                                            Button {
                                                key: "{kind.slug()}",
                                                compact: true,
                                                variant: if is_active {
                                                    ButtonVariant::Primary
                                                } else {
                                                    ButtonVariant::Secondary
                                                },
                                                aria_pressed: if is_active { "true" } else { "false" },
                                                onclick: move |_| active_source.set(kind),
                                                "{src.label} · {badge_words}"
                                            }
                                        }
                                    }
                                }
                            }
                            if let Some(source) = selected_source {
                                SourceSetupCard {
                                    key: "{source.source_id.slug()}",
                                    client,
                                    session,
                                    shown,
                                    practice_zone: practice_zone.clone(),
                                    server_now: server_now.clone(),
                                    source,
                                    on_updated: move |updated: PracticeSourceWire| {
                                        if let Some(Ok(ref mut list)) = *state.write()
                                            && let Some(slot) = list
                                                .sources
                                                .iter_mut()
                                                .find(|item| item.source_id == updated.source_id)
                                        {
                                            *slot = updated;
                                        }
                                        on_changed.call(());
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SourceSetupCard(
    client: Signal<PhysicianClient>,
    session: SessionState,
    shown: bool,
    practice_zone: String,
    server_now: String,
    source: PracticeSourceWire,
    on_updated: EventHandler<PracticeSourceWire>,
) -> Element {
    let kind = source.source_id;
    let default_method = match &source.standing {
        SourceStandingWire::AccessLost { method, .. } => *method,
        _ => source
            .supported_methods
            .first()
            .copied()
            .unwrap_or(SourceConnectionMethodWire::SimulatedPractice),
    };
    let default_account = match &source.standing {
        SourceStandingWire::AccessLost { account_label, .. } => account_label.clone(),
        _ => match kind {
            SupportedSourceWire::Hint => "Hint Health Chrome session".to_owned(),
            SupportedSourceWire::ElationFhir => "https://fhir.practice.example/r4".to_owned(),
            SupportedSourceWire::Spruce => "Practice Spruce main line".to_owned(),
        },
    };

    let chosen_method = use_signal(move || default_method);
    let account_label = use_signal(move || default_account);
    let secret_or_code = use_signal(String::new);
    let prerequisites_confirmed = use_signal(|| false);
    let contact_email = use_signal(String::new);
    let assist_notes = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut action_error = use_signal(|| None::<String>);

    let (pill_tone, pill_icon, pill_words) = standing_badge(&source.standing);

    #[derive(Clone)]
    enum SourceMutation {
        Connect(ConnectSourceBody),
        RequestAssist(RequestAssistedSetupBody),
        ImportInitial,
        Refresh,
        AcceptSpruceBacklog,
        Reconnect(ConnectSourceBody),
        Disconnect,
    }

    let mut run_mutation = move |mutation: SourceMutation| {
        if busy() {
            return;
        }
        busy.set(true);
        action_error.set(None);
        let api = client.read().clone();
        let presented = session.bearer();
        spawn(async move {
            let result = match mutation {
                SourceMutation::Connect(body) => api.connect_source(kind, &body).await,
                SourceMutation::RequestAssist(body) => {
                    api.request_assisted_setup(kind, &body).await
                }
                SourceMutation::ImportInitial => api.import_source_records(kind).await,
                SourceMutation::Refresh => api.refresh_source(kind).await,
                SourceMutation::AcceptSpruceBacklog => api.accept_spruce_backlog().await,
                SourceMutation::Reconnect(body) => api.reconnect_source(kind, &body).await,
                SourceMutation::Disconnect => api.disconnect_source(kind).await,
            };
            busy.set(false);
            match result {
                Ok(resp) => on_updated.call(resp.source),
                Err(err) => {
                    if let Some(why) = refusal(&err) {
                        session.refused(why, shown, presented);
                        return;
                    }
                    action_error.set(Some(error_message(
                        &err,
                        "Coda couldn’t complete that source update.",
                    )));
                }
            }
        });
    };

    let setup_methods = source.supported_methods.clone();
    let reconnect_methods: Vec<SourceConnectionMethodWire> = source
        .supported_methods
        .iter()
        .copied()
        .filter(|m| *m != SourceConnectionMethodWire::CodaAssisted)
        .collect();

    rsx! {
        Card {
            class: "ps-source-card",
            overline: Some("Practice clinical source".to_owned()),
            mark: Some(rsx! {
                StatePill { tone: pill_tone, icon: Some(pill_icon), "{pill_words}" }
            }),
            title: Some(source.label.clone()),
            detail: Some(source.summary.clone()),
            div { class: "ps-source-stack",
                p { class: "ps-guidance", "{source.guidance}" }
                if !source.prerequisites.is_empty() {
                    div { class: "ps-prereq-block",
                        p { class: "ps-section-label", "Prerequisites" }
                        ul { class: "ps-prereq-list",
                            for item in &source.prerequisites {
                                li { "{item}" }
                            }
                        }
                    }
                }

                if let Some(err) = action_error() {
                    Notice {
                        tone: Tone::Error,
                        icon: IconName::Alert,
                        title: "Couldn’t update source",
                        detail: Some(err),
                        span {}
                    }
                }

                match &source.standing {
                    SourceStandingWire::Disconnected => {
                        let show_disconnect_retention = kind == SupportedSourceWire::Spruce
                            && (source.retained_linked_records > 0
                                || source.retained_reply_drafts > 0);
                        rsx! {
                            if show_disconnect_retention {
                                Notice {
                                    tone: Tone::Neutral,
                                    icon: IconName::Check,
                                    title: "Spruce disconnected",
                                    detail: Some(format!(
                                        "{} retained (never-linked channel messages deleted on disconnect). {} and {} remain readable under the practice record policy.",
                                        count_words(
                                            source.unlinked_spruce_messages,
                                            "unlinked Spruce message",
                                            "unlinked Spruce messages",
                                        ),
                                        count_words(
                                            source.retained_linked_records,
                                            "linked chart record",
                                            "linked chart records",
                                        ),
                                        count_words(
                                            source.retained_reply_drafts,
                                            "physician reply draft",
                                            "physician reply drafts",
                                        ),
                                    )),
                                    span {}
                                }
                            }
                            SourceConnectionForm {
                                kind,
                                methods: setup_methods,
                                chosen_method,
                                account_label,
                                secret_or_code,
                                prerequisites_confirmed,
                                contact_email,
                                assist_notes,
                                busy: busy(),
                                reconnect_mode: false,
                                on_connect: move |body| run_mutation(SourceMutation::Connect(body)),
                                on_assist: move |body| run_mutation(SourceMutation::RequestAssist(body)),
                            }
                        }
                    }
                    SourceStandingWire::CodaAssistedRequested {
                        requested_at,
                        contact_email: saved_email,
                        notes,
                    } => {
                        let when = format_instant(requested_at, &practice_zone, &server_now);
                        let note_part = notes
                            .as_deref()
                            .filter(|text| !text.is_empty())
                            .map(|text| format!(" · Notes: {text}"))
                            .unwrap_or_default();
                        let can_direct_connect = setup_methods
                            .iter()
                            .any(|m| *m != SourceConnectionMethodWire::CodaAssisted);
                        let follow_up = if can_direct_connect {
                            "Coda staff will coordinate onboarding, or you can connect directly below."
                        } else {
                            "Coda staff will coordinate onboarding; update your contact details below if needed."
                        };
                        rsx! {
                            Notice {
                                tone: Tone::Primary,
                                icon: IconName::Clock,
                                title: "Coda-assisted setup requested",
                                detail: Some(format!(
                                    "Requested {when} for {saved_email}{note_part}. {follow_up}"
                                )),
                                span {}
                            }
                            SourceConnectionForm {
                                kind,
                                methods: setup_methods,
                                chosen_method,
                                account_label,
                                secret_or_code,
                                prerequisites_confirmed,
                                contact_email,
                                assist_notes,
                                busy: busy(),
                                reconnect_mode: false,
                                on_connect: move |body| run_mutation(SourceMutation::Connect(body)),
                                on_assist: move |body| run_mutation(SourceMutation::RequestAssist(body)),
                            }
                        }
                    }
                    SourceStandingWire::Connected {
                        connected_at,
                        method,
                        account_label: saved_account,
                    } => {
                        let when = format_instant(connected_at, &practice_zone, &server_now);
                        rsx! {
                            Notice {
                                tone: Tone::Neutral,
                                icon: IconName::Check,
                                title: "Connection verified — no records imported yet",
                                detail: Some(format!(
                                    "{saved_account} ({}) verified {when}. Credential verification does not import records; start the initial import when ready.",
                                    method.label()
                                )),
                                span {}
                            }
                            ActionRow {
                                Button {
                                    variant: ButtonVariant::Primary,
                                    busy: busy(),
                                    onclick: move |_| run_mutation(SourceMutation::ImportInitial),
                                    "Import initial records"
                                }
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    danger: true,
                                    disabled: busy(),
                                    onclick: move |_| run_mutation(SourceMutation::Disconnect),
                                    "Disconnect"
                                }
                            }
                        }
                    }
                    SourceStandingWire::RecordsImported {
                        method,
                        account_label: saved_account,
                        imported_records,
                        last_imported_at,
                        ..
                    } => {
                        let when = format_instant(last_imported_at, &practice_zone, &server_now);
                        rsx! {
                            Notice {
                                tone: Tone::Primary,
                                icon: IconName::File,
                                title: "Initial records imported",
                                detail: Some(format!(
                                    "{imported_records} linked chart records imported {when} from {saved_account} ({}). Run a live sync check to confirm the source is up to date.",
                                    method.label()
                                )),
                                span {}
                            }
                            SpruceBacklogSection {
                                backlog: source.backlog.clone(),
                                practice_zone: practice_zone.clone(),
                                server_now: server_now.clone(),
                                busy: busy(),
                                on_accept: move |_| run_mutation(SourceMutation::AcceptSpruceBacklog),
                            }
                            SpruceRetentionSummary { source: source.clone() }
                            ActionRow {
                                Button {
                                    variant: ButtonVariant::Primary,
                                    busy: busy(),
                                    onclick: move |_| run_mutation(SourceMutation::Refresh),
                                    "Sync latest records"
                                }
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    danger: true,
                                    disabled: busy(),
                                    onclick: move |_| run_mutation(SourceMutation::Disconnect),
                                    "Disconnect"
                                }
                            }
                        }
                    }
                    SourceStandingWire::UpToDate {
                        method,
                        account_label: saved_account,
                        imported_records,
                        last_read_at,
                        ..
                    } => {
                        let when = format_instant(last_read_at, &practice_zone, &server_now);
                        rsx! {
                            Notice {
                                tone: Tone::Primary,
                                icon: IconName::Check,
                                title: "Up to date",
                                detail: Some(format!(
                                    "{imported_records} linked chart records · {saved_account} ({}) · Last read {when}.",
                                    method.label()
                                )),
                                span {}
                            }
                            SpruceBacklogSection {
                                backlog: source.backlog.clone(),
                                practice_zone: practice_zone.clone(),
                                server_now: server_now.clone(),
                                busy: busy(),
                                on_accept: move |_| run_mutation(SourceMutation::AcceptSpruceBacklog),
                            }
                            SpruceRetentionSummary { source: source.clone() }
                            ActionRow {
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    busy: busy(),
                                    onclick: move |_| run_mutation(SourceMutation::Refresh),
                                    "Refresh records"
                                }
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    danger: true,
                                    disabled: busy(),
                                    onclick: move |_| run_mutation(SourceMutation::Disconnect),
                                    "Disconnect"
                                }
                            }
                        }
                    }
                    SourceStandingWire::ReadFailed {
                        method,
                        account_label: saved_account,
                        last_successful_read_at,
                        reason,
                        ..
                    } => {
                        let last_read = format_last_read(
                            last_successful_read_at.as_deref(),
                            &practice_zone,
                            &server_now,
                        );
                        rsx! {
                            Notice {
                                tone: Tone::Warning,
                                icon: IconName::Alert,
                                title: "Records couldn’t be updated",
                                detail: Some(format!(
                                    "{reason} · {saved_account} ({}) · {last_read}. {SAVED_RECORDS_READABLE}",
                                    method.label()
                                )),
                                span {}
                            }
                            SpruceBacklogSection {
                                backlog: source.backlog.clone(),
                                practice_zone: practice_zone.clone(),
                                server_now: server_now.clone(),
                                busy: busy(),
                                on_accept: move |_| run_mutation(SourceMutation::AcceptSpruceBacklog),
                            }
                            SpruceRetentionSummary { source: source.clone() }
                            ActionRow {
                                Button {
                                    variant: ButtonVariant::Primary,
                                    busy: busy(),
                                    onclick: move |_| run_mutation(SourceMutation::Refresh),
                                    "Retry read"
                                }
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    danger: true,
                                    disabled: busy(),
                                    onclick: move |_| run_mutation(SourceMutation::Disconnect),
                                    "Disconnect"
                                }
                            }
                        }
                    }
                    SourceStandingWire::AccessLost {
                        method,
                        account_label: saved_account,
                        last_successful_read_at,
                        cause,
                        reason,
                        ..
                    } => {
                        let last_read = format_last_read(
                            last_successful_read_at.as_deref(),
                            &practice_zone,
                            &server_now,
                        );
                        rsx! {
                            Notice {
                                tone: Tone::Error,
                                icon: IconName::Alert,
                                title: format!("Access lost: {}", cause.label()),
                                detail: Some(format!(
                                    "{reason} · {saved_account} ({}) · {last_read}. {SAVED_RECORDS_READABLE}",
                                    method.label()
                                )),
                                span {}
                            }
                            SpruceRetentionSummary { source: source.clone() }
                            if !reconnect_methods.is_empty() {
                                SourceConnectionForm {
                                    kind,
                                    methods: reconnect_methods,
                                    chosen_method,
                                    account_label,
                                    secret_or_code,
                                    prerequisites_confirmed,
                                    contact_email,
                                    assist_notes,
                                    busy: busy(),
                                    reconnect_mode: true,
                                    on_connect: move |body| run_mutation(SourceMutation::Reconnect(body)),
                                    on_assist: move |_| {},
                                }
                            } else {
                                SourceConnectionForm {
                                    kind,
                                    methods: setup_methods,
                                    chosen_method,
                                    account_label,
                                    secret_or_code,
                                    prerequisites_confirmed,
                                    contact_email,
                                    assist_notes,
                                    busy: busy(),
                                    reconnect_mode: false,
                                    on_connect: move |body| run_mutation(SourceMutation::Connect(body)),
                                    on_assist: move |body| run_mutation(SourceMutation::RequestAssist(body)),
                                }
                            }
                            ActionRow {
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    danger: true,
                                    disabled: busy(),
                                    onclick: move |_| run_mutation(SourceMutation::Disconnect),
                                    "Disconnect"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SpruceBacklogSection(
    backlog: SpruceBacklogWire,
    practice_zone: String,
    server_now: String,
    busy: bool,
    on_accept: EventHandler,
) -> Element {
    match backlog {
        SpruceBacklogWire::NotApplicable | SpruceBacklogWire::NonePending => rsx! {},
        SpruceBacklogWire::Offered {
            message_count,
            earliest_message_at,
        } => {
            let earliest = format_instant(&earliest_message_at, &practice_zone, &server_now);
            rsx! {
                Notice {
                    tone: Tone::Warning,
                    icon: IconName::Message,
                    title: "Pre-connection Spruce backlog held",
                    detail: Some(format!(
                        "{message_count} messages sent before connection (since {earliest}) were not imported automatically. Coda reads historical Spruce messages only if you accept backlog review."
                    )),
                    Button {
                        compact: true,
                        variant: ButtonVariant::Primary,
                        busy,
                        onclick: move |_| on_accept.call(()),
                        "Review backlog"
                    }
                }
            }
        }
        SpruceBacklogWire::Accepted {
            accepted_at,
            imported_message_count,
        } => {
            let when = format_instant(&accepted_at, &practice_zone, &server_now);
            rsx! {
                p { class: "ps-meta-line",
                    "Historical Spruce backlog reviewed ({imported_message_count} messages imported {when})."
                }
            }
        }
    }
}

fn count_words(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("1 {singular}")
    } else {
        format!("{count} {plural}")
    }
}

#[component]
fn SpruceRetentionSummary(source: PracticeSourceWire) -> Element {
    if source.source_id != SupportedSourceWire::Spruce {
        return rsx! {};
    }
    if source.retained_linked_records == 0
        && source.retained_reply_drafts == 0
        && source.unlinked_spruce_messages == 0
    {
        return rsx! {};
    }
    let records = count_words(
        source.retained_linked_records,
        "linked chart record",
        "linked chart records",
    );
    let drafts = count_words(
        source.retained_reply_drafts,
        "physician reply draft",
        "physician reply drafts",
    );
    let unlinked = count_words(
        source.unlinked_spruce_messages,
        "unlinked channel message",
        "unlinked channel messages",
    );
    rsx! {
        p { class: "ps-meta-line",
            "{records} · {drafts} · {unlinked}"
        }
    }
}

#[component]
fn SourceConnectionForm(
    kind: SupportedSourceWire,
    methods: Vec<SourceConnectionMethodWire>,
    mut chosen_method: Signal<SourceConnectionMethodWire>,
    mut account_label: Signal<String>,
    mut secret_or_code: Signal<String>,
    mut prerequisites_confirmed: Signal<bool>,
    mut contact_email: Signal<String>,
    mut assist_notes: Signal<String>,
    busy: bool,
    reconnect_mode: bool,
    on_connect: EventHandler<ConnectSourceBody>,
    on_assist: EventHandler<RequestAssistedSetupBody>,
) -> Element {
    if methods.is_empty() {
        return rsx! {};
    }
    let selected_idx = methods
        .iter()
        .position(|m| *m == chosen_method())
        .unwrap_or(0);
    let active_method = methods[selected_idx];
    let choices: Vec<Choice> = methods.iter().copied().map(method_choice).collect();
    let methods_for_select = methods.clone();

    let account_field_label = match active_method {
        SourceConnectionMethodWire::BrowserSession => "Connector session or practice name",
        SourceConnectionMethodWire::CredentialSignIn => "Practice endpoint or account identifier",
        SourceConnectionMethodWire::SimulatedPractice => "Simulated practice dataset",
        SourceConnectionMethodWire::CodaAssisted => "Practice contact email",
    };

    let secret_field_label = match active_method {
        SourceConnectionMethodWire::BrowserSession => "Chrome connector verification code",
        SourceConnectionMethodWire::CredentialSignIn => "API credential or bearer secret",
        SourceConnectionMethodWire::SimulatedPractice
        | SourceConnectionMethodWire::CodaAssisted => "",
    };

    let requires_hint_checkbox = kind == SupportedSourceWire::Hint
        && active_method == SourceConnectionMethodWire::BrowserSession;

    let submit_words = if reconnect_mode {
        "Reconnect"
    } else {
        match active_method {
            SourceConnectionMethodWire::SimulatedPractice => "Connect simulated source",
            SourceConnectionMethodWire::BrowserSession => "Verify Chrome connector",
            SourceConnectionMethodWire::CredentialSignIn => "Verify credentials",
            SourceConnectionMethodWire::CodaAssisted => "Request Coda-assisted setup",
        }
    };

    rsx! {
        div { class: "ps-form-stack",
            RadioGroup {
                label: if reconnect_mode {
                    "Recovery method".to_owned()
                } else {
                    "Setup method".to_owned()
                },
                options: choices,
                selected: Some(selected_idx),
                on_select: move |idx: usize| {
                    if let Some(&picked) = methods_for_select.get(idx) {
                        chosen_method.set(picked);
                    }
                },
            }

            if active_method == SourceConnectionMethodWire::CodaAssisted && !reconnect_mode {
                TextField {
                    label: "Practice contact email",
                    kind: TextKind::Email,
                    placeholder: "admin@practice.org",
                    value: contact_email(),
                    disabled: busy,
                    on_input: move |val| contact_email.set(val),
                }
                TextField {
                    label: "Setup notes (optional)",
                    kind: TextKind::Multiline,
                    placeholder: "EHR administrator contact or preferred setup window",
                    value: assist_notes(),
                    disabled: busy,
                    on_input: move |val| assist_notes.set(val),
                }
                ActionRow {
                    Button {
                        variant: ButtonVariant::Primary,
                        busy,
                        onclick: move |_| {
                            let notes = assist_notes().trim().to_owned();
                            on_assist.call(RequestAssistedSetupBody {
                                contact_email: contact_email(),
                                notes: (!notes.is_empty()).then_some(notes),
                            });
                        },
                        "{submit_words}"
                    }
                }
            } else {
                TextField {
                    label: account_field_label,
                    kind: TextKind::Line,
                    placeholder: "Enter practice account or session label",
                    value: account_label(),
                    disabled: busy,
                    on_input: move |val| account_label.set(val),
                }
                if !secret_field_label.is_empty() {
                    TextField {
                        label: secret_field_label,
                        kind: TextKind::Line,
                        placeholder: "Enter verification token or credential",
                        value: secret_or_code(),
                        disabled: busy,
                        on_input: move |val| secret_or_code.set(val),
                    }
                }
                if requires_hint_checkbox {
                    Checkbox {
                        label: "I am signed in to Hint Health in Chrome with the Coda Hint connector enabled",
                        detail: Some("Confirms whole-chart and full-note read access before connecting.".to_owned()),
                        checked: prerequisites_confirmed(),
                        disabled: busy,
                        on_change: move |checked| prerequisites_confirmed.set(checked),
                    }
                }
                ActionRow {
                    Button {
                        variant: ButtonVariant::Primary,
                        busy,
                        onclick: move |_| {
                            let secret = secret_or_code().trim().to_owned();
                            on_connect.call(ConnectSourceBody {
                                method: active_method,
                                account_label: account_label(),
                                secret_or_code: (!secret.is_empty()).then_some(secret),
                                prerequisites_confirmed: prerequisites_confirmed(),
                            });
                        },
                        "{submit_words}"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physician_wire::source_setup::AccessLossCauseWire;

    #[test]
    fn connected_imported_and_up_to_date_map_to_three_distinct_badges() {
        let (_, _, connected) = standing_badge(&SourceStandingWire::Connected {
            connected_at: "2026-08-27T16:00:00Z".to_owned(),
            method: SourceConnectionMethodWire::CredentialSignIn,
            account_label: "Spruce Main".to_owned(),
        });
        let (_, _, imported) = standing_badge(&SourceStandingWire::RecordsImported {
            connected_at: "2026-08-27T16:00:00Z".to_owned(),
            method: SourceConnectionMethodWire::CredentialSignIn,
            account_label: "Spruce Main".to_owned(),
            imported_records: 2,
            last_imported_at: "2026-08-27T16:01:00Z".to_owned(),
        });
        let (_, _, up_to_date) = standing_badge(&SourceStandingWire::UpToDate {
            connected_at: "2026-08-27T16:00:00Z".to_owned(),
            method: SourceConnectionMethodWire::CredentialSignIn,
            account_label: "Spruce Main".to_owned(),
            imported_records: 2,
            last_read_at: "2026-08-27T16:02:00Z".to_owned(),
        });
        assert_eq!(connected, "Connected");
        assert_eq!(imported, "Records imported");
        assert_eq!(up_to_date, "Up to date");
        assert_ne!(connected, imported);
        assert_ne!(imported, up_to_date);
    }

    #[test]
    fn today_read_failure_never_asks_to_reconnect_while_access_lost_does() {
        let read_failed = project_today_notice(
            &TodayConnection {
                condition: "source_read_failed".to_owned(),
                channel: Some("spruce".to_owned()),
                source_id: Some(SupportedSourceWire::Spruce),
                source_label: Some("Spruce Health".to_owned()),
                reason: Some("Upstream 503 timeout".to_owned()),
                last_successful_read_at: Some("2026-08-27T16:00:00Z".to_owned()),
                recovery: Some(TodayConnectionRecoveryWire::ReadFailed),
            },
            "America/New_York",
            "2026-08-27T16:05:00Z",
        )
        .expect("read_failed notice");
        assert_eq!(read_failed.tone, Tone::Warning);
        assert!(read_failed.title.contains("Records couldn’t be updated"));
        assert!(read_failed.detail.contains("Upstream 503 timeout"));
        assert_eq!(read_failed.action_label, "Inspect source");
        assert!(!read_failed.primary_action);

        let access_lost = project_today_notice(
            &TodayConnection {
                condition: "reconnect_required".to_owned(),
                channel: Some("spruce".to_owned()),
                source_id: Some(SupportedSourceWire::Spruce),
                source_label: Some("Spruce Health".to_owned()),
                reason: Some(AccessLossCauseWire::RevokedGrant.label().to_owned()),
                last_successful_read_at: Some("2026-08-27T16:00:00Z".to_owned()),
                recovery: Some(TodayConnectionRecoveryWire::ReconnectRequired),
            },
            "America/New_York",
            "2026-08-27T16:05:00Z",
        )
        .expect("access_lost notice");
        assert_eq!(access_lost.tone, Tone::Error);
        assert!(access_lost.title.contains("Access lost"));
        assert_eq!(access_lost.action_label, "Reconnect");
        assert!(access_lost.primary_action);
    }

    #[test]
    fn format_instant_preserves_calendar_date_across_multi_day_and_practice_zones() {
        assert_eq!(
            format_instant(
                "2026-08-27T16:00:00Z",
                "America/New_York",
                "2026-08-27T16:05:00Z"
            ),
            "Today, 12:00 PM"
        );
        assert_eq!(
            format_instant(
                "2026-08-22T16:00:00Z",
                "America/New_York",
                "2026-08-27T16:00:00Z"
            ),
            "Aug 22, 2026, 12:00 PM"
        );
        assert_eq!(
            format_instant(
                "2026-08-22T16:00:00Z",
                "America/Los_Angeles",
                "2026-08-27T16:00:00Z"
            ),
            "Aug 22, 2026, 9:00 AM"
        );
        assert_eq!(
            format_instant("2026-08-22T16:00:00Z", "America/New_York", ""),
            "Aug 22, 2026, 12:00 PM"
        );
    }
}
