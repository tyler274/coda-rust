//! Patient-page `Follow-ups` section: listing, creating, editing, `Review now`,
//! `Stop`, `Stop and discuss`, `Resume`, `Complete`, `Cancel`, and `"Can't restart"`
//! resolution using shared `crates/ui` components.

use std::collections::BTreeMap;

use dioxus::prelude::*;
use physician_client::{HttpError, PhysicianClient};
use physician_wire::FollowUp as LegacyFollowUp;
use physician_wire::follow_ups::{
    CannotRestartResolutionWire, FollowUpOriginWire, FollowUpStandingWire, FollowUpWire,
    PatientFollowUpsResponseWire, UpdateFollowUpBody,
};
use ui::{
    ActionRow, Button, ButtonVariant, Card, DateField, DateKind, EmptyLine, Icon, IconName, Notice,
    StatePill, TextField, Tone, WorkingPresence,
};

use crate::shell::{ReadView, TRY_AGAIN, read_view};
use crate::status::ReadNotice;

const FOLLOW_UPS_LOADING: &str = "Loading follow-ups…";
const FOLLOW_UPS_STALE: &str = "Coda couldn’t refresh follow-ups. Showing the last list.";
const FOLLOW_UPS_UNAVAILABLE: &str = "Coda couldn’t load follow-ups for this patient.";

#[derive(Clone, Debug, PartialEq, Eq)]
struct FollowUpEditDraft {
    summary: String,
    due_date: String,
    note: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct CannotRestartDraft {
    new_date: String,
    note: String,
}

fn is_patient_access_lost_on_list(err: &HttpError) -> bool {
    matches!(
        err,
        HttpError::Status {
            status: 403 | 404,
            ..
        }
    )
}

fn is_patient_access_lost_on_command(err: &HttpError) -> bool {
    match err {
        HttpError::Status { status: 403, .. } => true,
        HttpError::Status { status: 404, body } => !body.contains("follow_up_not_found"),
        _ => false,
    }
}

fn not_saved_message(err: &HttpError, fallback: &str) -> String {
    if let HttpError::Status { body, .. } = err
        && let Ok(val) = serde_json::from_str::<serde_json::Value>(body)
        && let Some(detail) = val.get("detail").and_then(serde_json::Value::as_str)
        && !detail.trim().is_empty()
    {
        if detail.starts_with("Not saved") {
            return detail.to_owned();
        }
        return format!("Not saved. {detail}");
    }
    format!("Not saved. {fallback}")
}

const fn standing_slug(standing: &FollowUpStandingWire) -> &'static str {
    match standing {
        FollowUpStandingWire::Scheduled { .. } => "scheduled",
        FollowUpStandingWire::DueNow { .. } => "due_now",
        FollowUpStandingWire::Held { .. } => "held",
        FollowUpStandingWire::CannotRestart { .. } => "cannot_restart",
        FollowUpStandingWire::Completed { .. } => "completed",
        FollowUpStandingWire::Cancelled { .. } => "cancelled",
    }
}

const fn standing_tone(standing: &FollowUpStandingWire) -> Tone {
    match standing {
        FollowUpStandingWire::Scheduled { .. } | FollowUpStandingWire::DueNow { .. } => {
            Tone::Primary
        }
        FollowUpStandingWire::Held { .. } => Tone::Warning,
        FollowUpStandingWire::CannotRestart { .. } => Tone::Error,
        FollowUpStandingWire::Completed { .. } | FollowUpStandingWire::Cancelled { .. } => {
            Tone::Neutral
        }
    }
}

fn standing_due_date_input(standing: &FollowUpStandingWire) -> String {
    match standing {
        FollowUpStandingWire::Scheduled { due_date_input, .. }
        | FollowUpStandingWire::DueNow { due_date_input, .. }
        | FollowUpStandingWire::Held { due_date_input, .. }
        | FollowUpStandingWire::CannotRestart { due_date_input, .. } => due_date_input.clone(),
        FollowUpStandingWire::Completed { .. } | FollowUpStandingWire::Cancelled { .. } => {
            String::new()
        }
    }
}

fn standing_detail_line(item: &FollowUpWire) -> String {
    let origin_words = match &item.origin {
        FollowUpOriginWire::PhysicianCreated { .. } => "Added by physician",
        FollowUpOriginWire::CodaCreated { .. } => "Added by Coda",
    };
    match &item.standing {
        FollowUpStandingWire::Scheduled { due_date_input, .. } => {
            format!("Due {due_date_input} · {origin_words}")
        }
        FollowUpStandingWire::DueNow { due_date_input, .. } => {
            format!("Reviewing now (scheduled {due_date_input}) · {origin_words}")
        }
        FollowUpStandingWire::Held {
            reason,
            due_date_input,
            ..
        } => match reason.as_deref().filter(|r| !r.trim().is_empty()) {
            Some(r) => format!("Held ({r}) · Target {due_date_input}"),
            None => format!("Held · Target {due_date_input}"),
        },
        FollowUpStandingWire::CannotRestart {
            reason,
            due_date_input,
            ..
        } => format!("Can't restart: {reason} (target {due_date_input})"),
        FollowUpStandingWire::Completed { resolution, .. } => {
            format!("Completed · {resolution}")
        }
        FollowUpStandingWire::Cancelled { reason, .. } => {
            format!("Cancelled · {reason}")
        }
    }
}

/// Reactive read state for the open on-panel patient's follow-ups, registered in
/// `PhysicianShell` ahead of `snapshot` so chat snapshot reads stay last.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct PatientFollowUpsRead {
    pub(crate) local_attempt: Signal<u32>,
    pub(crate) cached: Signal<Option<(String, PatientFollowUpsResponseWire)>>,
    pub(crate) read_res: Resource<(String, Result<PatientFollowUpsResponseWire, ()>)>,
}

pub(crate) fn use_patient_follow_ups(
    on_panel_workspace: Memo<Option<String>>,
    context: crate::patient_reviews_panel::PatientReviewsContext,
    on_patient_gone: Callback<String>,
) -> PatientFollowUpsRead {
    let local_attempt = use_signal(|| 0u32);
    let mut cached = use_signal(|| None::<(String, PatientFollowUpsResponseWire)>);
    let mut shown = context.shown;
    let session = context.session;
    let client = context.client;
    let workspace_attempt = context.workspace_attempt;

    let read_res = use_resource(move || {
        let ws = on_panel_workspace().unwrap_or_default();
        let _wa = workspace_attempt();
        let _la = local_attempt();
        let api = client.read().clone();
        let presented = api.session().get();
        async move {
            if ws.is_empty() {
                return (
                    ws.clone(),
                    Ok(PatientFollowUpsResponseWire {
                        patient_workspace_id: ws,
                        durable: false,
                        follow_ups: Vec::new(),
                    }),
                );
            }
            match api.patient_follow_ups(&ws).await {
                Ok(resp) => {
                    shown.set(true);
                    cached.set(Some((ws.clone(), resp.clone())));
                    (ws, Ok(resp))
                }
                Err(err) => {
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else if is_patient_access_lost_on_list(&err) {
                        on_patient_gone.call(ws.clone());
                    }
                    (ws, Err(()))
                }
            }
        }
    });

    PatientFollowUpsRead {
        local_attempt,
        cached,
        read_res,
    }
}

/// Patient-page `Follow-ups` section with durable creation, editing, lifecycle
/// controls, `Stop and discuss`, and `"Can't restart"` resolution.
#[component]
pub(crate) fn PatientFollowUpsSection(
    workspace_id: String,
    browser_tz: String,
    legacy_follow_ups: Vec<LegacyFollowUp>,
    read_state: PatientFollowUpsRead,
    client: Signal<PhysicianClient>,
    session: crate::SessionState,
    shown: Signal<bool>,
    mut workspace_attempt: Signal<u32>,
    mut today_attempt: Signal<u32>,
    on_open_discuss_chat: EventHandler<String>,
    on_patient_gone: EventHandler<String>,
) -> Element {
    let mut local_attempt = read_state.local_attempt;
    let mut cached = read_state.cached;
    let read_res = read_state.read_res;

    // Create form state (preserved across failed writes).
    let mut create_open = use_signal(|| false);
    let mut new_summary = use_signal(String::new);
    let mut new_due_date = use_signal(String::new);
    let mut create_busy = use_signal(|| false);
    let mut create_error = use_signal(|| None::<String>);

    // Per-follow-up edit drafts, `"Can't restart"` inputs, settle notes, busy flags, errors, and review status.
    let mut edit_drafts = use_signal(BTreeMap::<String, FollowUpEditDraft>::new);
    let mut cannot_restart_drafts = use_signal(BTreeMap::<String, CannotRestartDraft>::new);
    let mut settle_notes = use_signal(BTreeMap::<String, String>::new);
    let mut busy_item = use_signal(|| None::<String>);
    let mut item_errors = use_signal(BTreeMap::<String, String>::new);
    let mut review_started_items = use_signal(BTreeMap::<String, bool>::new);

    let previous = cached
        .read()
        .as_ref()
        .filter(|(asked, _)| asked == &workspace_id)
        .map(|(_, resp)| resp.clone());
    let current = read_res
        .read()
        .as_ref()
        .filter(|(asked, _)| asked == &workspace_id)
        .map(|(_, ans)| ans.clone());
    let view = read_view(previous, current);

    // On non-durable fixture sites with no follow-ups, remain quiet unless legacy follow-ups exist or read failed.
    if matches!(view, ReadView::Loading) && legacy_follow_ups.is_empty() {
        return rsx! {};
    }
    if let ReadView::Showing(resp) = &view
        && !resp.durable
        && resp.follow_ups.is_empty()
        && legacy_follow_ups.is_empty()
    {
        return rsx! {};
    }

    let apply_saved_follow_up = {
        let ws = workspace_id.clone();
        move |saved: FollowUpWire, review_started: bool| {
            if review_started {
                review_started_items
                    .write()
                    .insert(saved.follow_up_id.clone(), true);
            }
            let mut guard = cached.write();
            if let Some((cached_ws, resp)) = guard.as_mut()
                && cached_ws == &ws
            {
                if let Some(slot) = resp
                    .follow_ups
                    .iter_mut()
                    .find(|f| f.follow_up_id == saved.follow_up_id)
                {
                    *slot = saved;
                } else {
                    resp.follow_ups.insert(0, saved);
                }
            }
            *local_attempt.write() += 1;
            *workspace_attempt.write() += 1;
            *today_attempt.write() += 1;
        }
    };

    let ws_for_create = workspace_id.clone();
    let apply_create = apply_saved_follow_up.clone();
    let mut submit_create = move || {
        if *create_busy.peek() {
            return;
        }
        let summary = new_summary.read().trim().to_owned();
        let due_date = new_due_date.read().trim().to_owned();
        if summary.is_empty() {
            create_error.set(Some("Not saved. Enter what Coda should check.".to_owned()));
            return;
        }
        if due_date.is_empty() {
            create_error.set(Some("Not saved. Choose a target review date.".to_owned()));
            return;
        }
        create_busy.set(true);
        create_error.set(None);
        let api = client.read().clone();
        let presented = api.session().get();
        let ws = ws_for_create.clone();
        let key = crate::idempotency_key();
        let mut apply = apply_create.clone();
        spawn(async move {
            match api.create_follow_up(&ws, &summary, &due_date, &key).await {
                Ok(resp) => {
                    create_busy.set(false);
                    new_summary.set(String::new());
                    create_open.set(false);
                    apply(resp.follow_up, resp.review_started);
                }
                Err(err) => {
                    create_busy.set(false);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else if is_patient_access_lost_on_command(&err) {
                        on_patient_gone.call(ws);
                    } else {
                        create_error.set(Some(not_saved_message(
                            &err,
                            "Coda couldn’t create that follow-up.",
                        )));
                    }
                }
            }
        });
    };

    let durable_enabled = view.content().is_some_and(|r| r.durable);

    rsx! {
        section {
            class: "patient-section",
            aria_label: "Follow-ups",
            "data-patient-follow-ups-section": "true",
            onkeydown: move |event: KeyboardEvent| {
                // Keep typing inside follow-up inputs isolated from page-level J/K/? shortcuts.
                event.stop_propagation();
            },
            div { class: "fu-section-header",
                h2 { "Follow-ups" }
                if durable_enabled && !create_open() {
                    Button {
                        compact: true,
                        variant: ButtonVariant::Secondary,
                        "data-open-create-follow-up": "true",
                        onclick: move |_| {
                            create_error.set(None);
                            create_open.set(true);
                        },
                        Icon { name: IconName::Plus }
                        "New follow-up"
                    }
                }
            }
            ReadNotice {
                stage: view.stage(),
                loading: FOLLOW_UPS_LOADING,
                stale: FOLLOW_UPS_STALE,
                unavailable: FOLLOW_UPS_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry: move |_| *local_attempt.write() += 1,
            }
            if durable_enabled && create_open() {
                Card {
                    "data-create-follow-up-card": "true",
                    overline: "Schedule follow-up".to_owned(),
                    title: "New follow-up".to_owned(),
                    div { class: "fu-form-stack",
                        if let Some(err_text) = create_error() {
                            div { "data-create-follow-up-error": "true",
                                Notice {
                                    tone: Tone::Error,
                                    icon: Some(IconName::Alert),
                                    title: err_text,
                                }
                            }
                        }
                        div { "data-create-follow-up-summary": "true",
                            TextField {
                                label: "What Coda should check".to_owned(),
                                placeholder: "e.g., Recheck BMP potassium after lisinopril increase".to_owned(),
                                value: new_summary(),
                                disabled: create_busy(),
                                on_input: move |val| new_summary.set(val),
                                onkeydown: move |ev: KeyboardEvent| ev.stop_propagation(),
                            }
                        }
                        div { "data-create-follow-up-due-date": "true",
                            DateField {
                                label: "Review date".to_owned(),
                                kind: DateKind::Date,
                                value: new_due_date(),
                                disabled: create_busy(),
                                on_change: move |val| new_due_date.set(val),
                            }
                        }
                        ActionRow {
                            Button {
                                compact: true,
                                variant: ButtonVariant::Primary,
                                busy: create_busy(),
                                "data-submit-create-follow-up": "true",
                                onclick: move |_| submit_create(),
                                "Add follow-up"
                            }
                            Button {
                                compact: true,
                                variant: ButtonVariant::Quiet,
                                disabled: create_busy(),
                                "data-cancel-create-follow-up": "true",
                                onclick: move |_| {
                                    create_error.set(None);
                                    create_open.set(false);
                                },
                                "Cancel"
                            }
                        }
                    }
                }
            }
            if let Some(resp) = view.content().cloned() {
                if resp.follow_ups.is_empty() && legacy_follow_ups.is_empty() {
                    EmptyLine { message: "No follow-ups scheduled for this patient.".to_owned() }
                } else if resp.follow_ups.is_empty() {
                    for legacy in &legacy_follow_ups {
                        p { "{legacy.description.as_deref().unwrap_or_default()}" }
                    }
                } else {
                    div { class: "fu-stack",
                        for item in resp.follow_ups {
                            {
                                let fid = item.follow_up_id.clone();
                                let edit_draft = edit_drafts.read().get(&fid).cloned();
                                let cr_draft = cannot_restart_drafts
                                    .read()
                                    .get(&fid)
                                    .cloned()
                                    .unwrap_or_else(|| CannotRestartDraft {
                                        new_date: standing_due_date_input(&item.standing),
                                        note: String::new(),
                                    });
                                let settle_note = settle_notes
                                    .read()
                                    .get(&fid)
                                    .cloned()
                                    .unwrap_or_default();
                                let err_text = item_errors.read().get(&fid).cloned();
                                let is_busy = busy_item.read().as_deref() == Some(fid.as_str());
                                let rev_started = review_started_items
                                    .read()
                                    .get(&fid)
                                    .copied()
                                    .unwrap_or(false);

                                rsx! {
                                    FollowUpItemCard {
                                        key: "{fid}",
                                        workspace_id: workspace_id.clone(),
                                        browser_tz: browser_tz.clone(),
                                        item: item.clone(),
                                        edit_draft,
                                        cannot_restart_draft: cr_draft,
                                        settle_note,
                                        busy: is_busy,
                                        error_message: err_text,
                                        review_started: rev_started,
                                        on_begin_edit: {
                                            let fid = fid.clone();
                                            let init_summary = item.summary.clone();
                                            let init_due = standing_due_date_input(&item.standing);
                                            move |_| {
                                                item_errors.write().remove(&fid);
                                                edit_drafts.write().insert(
                                                    fid.clone(),
                                                    FollowUpEditDraft {
                                                        summary: init_summary.clone(),
                                                        due_date: init_due.clone(),
                                                        note: String::new(),
                                                    },
                                                );
                                            }
                                        },
                                        on_change_edit: {
                                            let fid = fid.clone();
                                            move |draft: FollowUpEditDraft| {
                                                edit_drafts.write().insert(fid.clone(), draft);
                                            }
                                        },
                                        on_cancel_edit: {
                                            let fid = fid.clone();
                                            move |_| {
                                                edit_drafts.write().remove(&fid);
                                            }
                                        },
                                        on_save_edit: {
                                            let ws = workspace_id.clone();
                                            let fid = fid.clone();
                                            let apply = apply_saved_follow_up.clone();
                                            move |draft: FollowUpEditDraft| {
                                                if busy_item.peek().is_some() {
                                                    return;
                                                }
                                                if draft.summary.trim().is_empty() {
                                                    item_errors.write().insert(
                                                        fid.clone(),
                                                        "Not saved. Follow-up summary cannot be empty.".to_owned(),
                                                    );
                                                    return;
                                                }
                                                if draft.due_date.trim().is_empty() {
                                                    item_errors.write().insert(
                                                        fid.clone(),
                                                        "Not saved. Choose a valid review date.".to_owned(),
                                                    );
                                                    return;
                                                }
                                                busy_item.set(Some(fid.clone()));
                                                item_errors.write().remove(&fid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let fid = fid.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    let body = UpdateFollowUpBody {
                                                        summary: Some(draft.summary.trim().to_owned()),
                                                        due_date: Some(draft.due_date.trim().to_owned()),
                                                        note: (!draft.note.trim().is_empty())
                                                            .then(|| draft.note.trim().to_owned()),
                                                        idempotency_key: key,
                                                    };
                                                    match api.update_follow_up(&ws, &fid, &body).await {
                                                        Ok(resp) => {
                                                            busy_item.set(None);
                                                            edit_drafts.write().remove(&fid);
                                                            apply(resp.follow_up, resp.review_started);
                                                        }
                                                        Err(err) => {
                                                            busy_item.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                item_errors.write().insert(
                                                                    fid,
                                                                    not_saved_message(
                                                                        &err,
                                                                        "Coda couldn’t save changes to that follow-up.",
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_change_settle_note: {
                                            let fid = fid.clone();
                                            move |val: String| {
                                                settle_notes.write().insert(fid.clone(), val);
                                            }
                                        },
                                        on_review_now: {
                                            let ws = workspace_id.clone();
                                            let fid = fid.clone();
                                            let apply = apply_saved_follow_up.clone();
                                            move |_| {
                                                if busy_item.peek().is_some() {
                                                    return;
                                                }
                                                busy_item.set(Some(fid.clone()));
                                                item_errors.write().remove(&fid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let fid = fid.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api.review_follow_up_now(&ws, &fid, &key).await {
                                                        Ok(resp) => {
                                                            busy_item.set(None);
                                                            apply(resp.follow_up, resp.review_started);
                                                        }
                                                        Err(err) => {
                                                            busy_item.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                item_errors.write().insert(
                                                                    fid,
                                                                    not_saved_message(
                                                                        &err,
                                                                        "Coda couldn’t start an immediate review.",
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_stop: {
                                            let ws = workspace_id.clone();
                                            let fid = fid.clone();
                                            let apply = apply_saved_follow_up.clone();
                                            move |note: Option<String>| {
                                                if busy_item.peek().is_some() {
                                                    return;
                                                }
                                                busy_item.set(Some(fid.clone()));
                                                item_errors.write().remove(&fid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let fid = fid.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api.stop_follow_up(&ws, &fid, note, &key).await {
                                                        Ok(resp) => {
                                                            busy_item.set(None);
                                                            review_started_items.write().remove(&fid);
                                                            apply(resp.follow_up, false);
                                                        }
                                                        Err(err) => {
                                                            busy_item.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                item_errors.write().insert(
                                                                    fid,
                                                                    not_saved_message(
                                                                        &err,
                                                                        "Coda couldn’t stop that follow-up.",
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_stop_and_discuss: {
                                            let ws = workspace_id.clone();
                                            let fid = fid.clone();
                                            let tz = browser_tz.clone();
                                            let apply = apply_saved_follow_up.clone();
                                            move |note: Option<String>| {
                                                if busy_item.peek().is_some() {
                                                    return;
                                                }
                                                busy_item.set(Some(fid.clone()));
                                                item_errors.write().remove(&fid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let fid = fid.clone();
                                                let tz = tz.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api
                                                        .stop_and_discuss_follow_up(&ws, &fid, note, &tz, &key)
                                                        .await
                                                    {
                                                        Ok(resp) => {
                                                            busy_item.set(None);
                                                            review_started_items.write().remove(&fid);
                                                            let chat_id = resp.discuss_chat_id.clone();
                                                            apply(resp.follow_up, false);
                                                            if let Some(cid) = chat_id {
                                                                on_open_discuss_chat.call(cid);
                                                            } else {
                                                                item_errors.write().insert(
                                                                    fid,
                                                                    "Not saved. Follow-up was paused, but Coda couldn’t open a discussion conversation.".to_owned(),
                                                                );
                                                            }
                                                        }
                                                        Err(err) => {
                                                            busy_item.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                item_errors.write().insert(
                                                                    fid,
                                                                    not_saved_message(
                                                                        &err,
                                                                        "Coda couldn’t stop and start a discussion.",
                                                                    ),
                                                                );
                                                                *local_attempt.write() += 1;
                                                                *today_attempt.write() += 1;
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_resume: {
                                            let ws = workspace_id.clone();
                                            let fid = fid.clone();
                                            let apply = apply_saved_follow_up.clone();
                                            move |_| {
                                                if busy_item.peek().is_some() {
                                                    return;
                                                }
                                                busy_item.set(Some(fid.clone()));
                                                item_errors.write().remove(&fid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let fid = fid.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api.resume_follow_up(&ws, &fid, &key).await {
                                                        Ok(resp) => {
                                                            busy_item.set(None);
                                                            apply(resp.follow_up, resp.review_started);
                                                        }
                                                        Err(err) => {
                                                            busy_item.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                item_errors.write().insert(
                                                                    fid,
                                                                    not_saved_message(
                                                                        &err,
                                                                        "Coda couldn’t resume that follow-up.",
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_complete: {
                                            let ws = workspace_id.clone();
                                            let fid = fid.clone();
                                            let apply = apply_saved_follow_up.clone();
                                            move |note: String| {
                                                if busy_item.peek().is_some() {
                                                    return;
                                                }
                                                busy_item.set(Some(fid.clone()));
                                                item_errors.write().remove(&fid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let fid = fid.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api.complete_follow_up(&ws, &fid, &note, &key).await {
                                                        Ok(resp) => {
                                                            busy_item.set(None);
                                                            settle_notes.write().remove(&fid);
                                                            review_started_items.write().remove(&fid);
                                                            apply(resp.follow_up, false);
                                                        }
                                                        Err(err) => {
                                                            busy_item.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                item_errors.write().insert(
                                                                    fid,
                                                                    not_saved_message(
                                                                        &err,
                                                                        "Coda couldn’t complete that follow-up.",
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_cancel: {
                                            let ws = workspace_id.clone();
                                            let fid = fid.clone();
                                            let apply = apply_saved_follow_up.clone();
                                            move |note: String| {
                                                if busy_item.peek().is_some() {
                                                    return;
                                                }
                                                busy_item.set(Some(fid.clone()));
                                                item_errors.write().remove(&fid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let fid = fid.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api.cancel_follow_up(&ws, &fid, &note, &key).await {
                                                        Ok(resp) => {
                                                            busy_item.set(None);
                                                            settle_notes.write().remove(&fid);
                                                            review_started_items.write().remove(&fid);
                                                            apply(resp.follow_up, false);
                                                        }
                                                        Err(err) => {
                                                            busy_item.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                item_errors.write().insert(
                                                                    fid,
                                                                    not_saved_message(
                                                                        &err,
                                                                        "Coda couldn’t cancel that follow-up.",
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_change_cannot_restart: {
                                            let fid = fid.clone();
                                            move |draft: CannotRestartDraft| {
                                                cannot_restart_drafts.write().insert(fid.clone(), draft);
                                            }
                                        },
                                        on_resolve_cannot_restart: {
                                            let ws = workspace_id.clone();
                                            let fid = fid.clone();
                                            let apply = apply_saved_follow_up.clone();
                                            move |resolution: CannotRestartResolutionWire| {
                                                if busy_item.peek().is_some() {
                                                    return;
                                                }
                                                if let CannotRestartResolutionWire::NewDate {
                                                    ref due_date,
                                                    ..
                                                } = resolution
                                                    && due_date.trim().is_empty()
                                                {
                                                    item_errors.write().insert(
                                                        fid.clone(),
                                                        "Not saved. Choose a new review date.".to_owned(),
                                                    );
                                                    return;
                                                }
                                                busy_item.set(Some(fid.clone()));
                                                item_errors.write().remove(&fid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let fid = fid.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api
                                                        .resolve_cannot_restart_follow_up(
                                                            &ws,
                                                            &fid,
                                                            resolution,
                                                            &key,
                                                        )
                                                        .await
                                                    {
                                                        Ok(resp) => {
                                                            busy_item.set(None);
                                                            cannot_restart_drafts.write().remove(&fid);
                                                            apply(resp.follow_up, false);
                                                        }
                                                        Err(err) => {
                                                            busy_item.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                item_errors.write().insert(
                                                                    fid,
                                                                    not_saved_message(
                                                                        &err,
                                                                        "Coda couldn’t save that resolution.",
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
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
}

#[component]
fn FollowUpItemCard(
    workspace_id: String,
    browser_tz: String,
    item: FollowUpWire,
    edit_draft: Option<FollowUpEditDraft>,
    cannot_restart_draft: CannotRestartDraft,
    settle_note: String,
    busy: bool,
    error_message: Option<String>,
    review_started: bool,
    on_begin_edit: EventHandler,
    on_change_edit: EventHandler<FollowUpEditDraft>,
    on_cancel_edit: EventHandler,
    on_save_edit: EventHandler<FollowUpEditDraft>,
    on_change_settle_note: EventHandler<String>,
    on_review_now: EventHandler,
    on_stop: EventHandler<Option<String>>,
    on_stop_and_discuss: EventHandler<Option<String>>,
    on_resume: EventHandler,
    on_complete: EventHandler<String>,
    on_cancel: EventHandler<String>,
    on_change_cannot_restart: EventHandler<CannotRestartDraft>,
    on_resolve_cannot_restart: EventHandler<CannotRestartResolutionWire>,
) -> Element {
    let _ = (&workspace_id, &browser_tz);
    let fid = item.follow_up_id.clone();
    let slug = standing_slug(&item.standing);
    let tone = standing_tone(&item.standing);
    let badge = item.standing.badge_label();
    let detail = standing_detail_line(&item);

    rsx! {
        Card {
            "data-follow-up-card": "{fid}",
            "data-follow-up-standing": "{slug}",
            "data-follow-up-version": "{item.version}",
            mark: rsx! {
                StatePill { tone, "{badge}" }
            },
            title: item.summary.clone(),
            detail,
            div { class: "fu-card-inner",
                if let Some(err_text) = error_message {
                    div { "data-follow-up-command-error": "{fid}",
                        Notice {
                            tone: Tone::Error,
                            icon: Some(IconName::Alert),
                            title: err_text,
                        }
                    }
                }
                if let Some(note) = item.latest_note.as_deref().filter(|n| !n.trim().is_empty()) {
                    p { class: "fu-latest-note", "Latest note: {note}" }
                }
                if review_started || matches!(item.standing, FollowUpStandingWire::DueNow { .. }) {
                    div { "data-follow-up-review-started": "{fid}",
                        WorkingPresence {
                            label: "Patient chart review queued".to_owned(),
                        }
                    }
                }

                if let Some(draft) = edit_draft {
                    div {
                        class: "fu-form-stack",
                        "data-follow-up-edit-form": "{fid}",
                        div { "data-edit-follow-up-summary": "{fid}",
                            TextField {
                                label: "What Coda should check".to_owned(),
                                value: draft.summary.clone(),
                                disabled: busy,
                                on_input: {
                                    let current = draft.clone();
                                    move |val| {
                                        on_change_edit.call(FollowUpEditDraft {
                                            summary: val,
                                            ..current.clone()
                                        });
                                    }
                                },
                                onkeydown: move |ev: KeyboardEvent| ev.stop_propagation(),
                            }
                        }
                        div { "data-edit-follow-up-due-date": "{fid}",
                            DateField {
                                label: "Review date".to_owned(),
                                kind: DateKind::Date,
                                value: draft.due_date.clone(),
                                disabled: busy,
                                on_change: {
                                    let current = draft.clone();
                                    move |val| {
                                        on_change_edit.call(FollowUpEditDraft {
                                            due_date: val,
                                            ..current.clone()
                                        });
                                    }
                                },
                            }
                        }
                        div { "data-edit-follow-up-note": "{fid}",
                            TextField {
                                label: "Change note (optional)".to_owned(),
                                placeholder: "Optional note on why this changed".to_owned(),
                                value: draft.note.clone(),
                                disabled: busy,
                                on_input: {
                                    let current = draft.clone();
                                    move |val| {
                                        on_change_edit.call(FollowUpEditDraft {
                                            note: val,
                                            ..current.clone()
                                        });
                                    }
                                },
                                onkeydown: move |ev: KeyboardEvent| ev.stop_propagation(),
                            }
                        }
                        ActionRow {
                            Button {
                                compact: true,
                                variant: ButtonVariant::Primary,
                                busy,
                                "data-save-follow-up-edit": "{fid}",
                                onclick: {
                                    let draft = draft.clone();
                                    move |_| on_save_edit.call(draft.clone())
                                },
                                "Save changes"
                            }
                            Button {
                                compact: true,
                                variant: ButtonVariant::Quiet,
                                disabled: busy,
                                "data-cancel-follow-up-edit": "{fid}",
                                onclick: move |_| on_cancel_edit.call(()),
                                "Cancel"
                            }
                        }
                    }
                } else {
                    match &item.standing {
                        FollowUpStandingWire::Scheduled { .. }
                        | FollowUpStandingWire::DueNow { .. } => {
                            let note_opt = (!settle_note.trim().is_empty())
                                .then(|| settle_note.trim().to_owned());
                            rsx! {
                                div { class: "fu-controls-stack",
                                    div { "data-follow-up-note-input": "{fid}",
                                        TextField {
                                            label: "Action note (optional)".to_owned(),
                                            placeholder: "Optional note for pause, completion, or cancellation".to_owned(),
                                            value: settle_note.clone(),
                                            disabled: busy,
                                            on_input: move |val| on_change_settle_note.call(val),
                                            onkeydown: move |ev: KeyboardEvent| ev.stop_propagation(),
                                        }
                                    }
                                    ActionRow {
                                        if matches!(item.standing, FollowUpStandingWire::Scheduled { .. }) {
                                            Button {
                                                compact: true,
                                                variant: ButtonVariant::Primary,
                                                busy,
                                                "data-follow-up-review-now": "{fid}",
                                                onclick: move |_| on_review_now.call(()),
                                                "Review now"
                                            }
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Secondary,
                                            disabled: busy,
                                            "data-follow-up-edit": "{fid}",
                                            onclick: move |_| on_begin_edit.call(()),
                                            "Edit"
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Secondary,
                                            disabled: busy,
                                            "data-follow-up-stop": "{fid}",
                                            onclick: {
                                                let n = note_opt.clone();
                                                move |_| on_stop.call(n.clone())
                                            },
                                            "Stop"
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Secondary,
                                            disabled: busy,
                                            "data-follow-up-stop-discuss": "{fid}",
                                            onclick: {
                                                let n = note_opt.clone();
                                                move |_| on_stop_and_discuss.call(n.clone())
                                            },
                                            "Stop and discuss"
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Secondary,
                                            disabled: busy,
                                            "data-follow-up-complete": "{fid}",
                                            onclick: {
                                                let s = settle_note.clone();
                                                move |_| on_complete.call(s.clone())
                                            },
                                            "Complete"
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Quiet,
                                            danger: true,
                                            disabled: busy,
                                            "data-follow-up-cancel": "{fid}",
                                            onclick: {
                                                let s = settle_note.clone();
                                                move |_| on_cancel.call(s.clone())
                                            },
                                            "Cancel follow-up"
                                        }
                                    }
                                }
                            }
                        }
                        FollowUpStandingWire::Held { .. } => {
                            rsx! {
                                div { class: "fu-controls-stack",
                                    div { "data-follow-up-note-input": "{fid}",
                                        TextField {
                                            label: "Action note (optional)".to_owned(),
                                            placeholder: "Optional note for completion or cancellation".to_owned(),
                                            value: settle_note.clone(),
                                            disabled: busy,
                                            on_input: move |val| on_change_settle_note.call(val),
                                            onkeydown: move |ev: KeyboardEvent| ev.stop_propagation(),
                                        }
                                    }
                                    ActionRow {
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Primary,
                                            busy,
                                            "data-follow-up-resume": "{fid}",
                                            onclick: move |_| on_resume.call(()),
                                            "Resume"
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Secondary,
                                            disabled: busy,
                                            "data-follow-up-edit": "{fid}",
                                            onclick: move |_| on_begin_edit.call(()),
                                            "Edit"
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Secondary,
                                            disabled: busy,
                                            "data-follow-up-complete": "{fid}",
                                            onclick: {
                                                let s = settle_note.clone();
                                                move |_| on_complete.call(s.clone())
                                            },
                                            "Complete"
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Quiet,
                                            danger: true,
                                            disabled: busy,
                                            "data-follow-up-cancel": "{fid}",
                                            onclick: {
                                                let s = settle_note.clone();
                                                move |_| on_cancel.call(s.clone())
                                            },
                                            "Cancel follow-up"
                                        }
                                    }
                                }
                            }
                        }
                        FollowUpStandingWire::CannotRestart { reason, .. } => {
                            rsx! {
                                div {
                                    class: "fu-form-stack",
                                    "data-cannot-restart-controls": "{fid}",
                                    Notice {
                                        tone: Tone::Warning,
                                        icon: Some(IconName::Alert),
                                        title: "Automated review couldn’t restart.".to_owned(),
                                        detail: Some(reason.clone()),
                                    }
                                    div { "data-cannot-restart-date": "{fid}",
                                        DateField {
                                            label: "New review date".to_owned(),
                                            kind: DateKind::Date,
                                            value: cannot_restart_draft.new_date.clone(),
                                            disabled: busy,
                                            on_change: {
                                                let cur = cannot_restart_draft.clone();
                                                move |val| {
                                                    on_change_cannot_restart.call(CannotRestartDraft {
                                                        new_date: val,
                                                        ..cur.clone()
                                                    });
                                                }
                                            },
                                        }
                                    }
                                    div { "data-cannot-restart-note": "{fid}",
                                        TextField {
                                            label: "Resolution note (optional)".to_owned(),
                                            placeholder: "Optional note for completion, reschedule, or cancellation".to_owned(),
                                            value: cannot_restart_draft.note.clone(),
                                            disabled: busy,
                                            on_input: {
                                                let cur = cannot_restart_draft.clone();
                                                move |val| {
                                                    on_change_cannot_restart.call(CannotRestartDraft {
                                                        note: val,
                                                        ..cur.clone()
                                                    });
                                                }
                                            },
                                            onkeydown: move |ev: KeyboardEvent| ev.stop_propagation(),
                                        }
                                    }
                                    ActionRow {
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Primary,
                                            busy,
                                            "data-cannot-restart-new-date": "{fid}",
                                            onclick: {
                                                let cur = cannot_restart_draft.clone();
                                                move |_| {
                                                    on_resolve_cannot_restart.call(
                                                        CannotRestartResolutionWire::NewDate {
                                                            due_date: cur.new_date.trim().to_owned(),
                                                            note: (!cur.note.trim().is_empty())
                                                                .then(|| cur.note.trim().to_owned()),
                                                        },
                                                    );
                                                }
                                            },
                                            "New date"
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Secondary,
                                            disabled: busy,
                                            "data-cannot-restart-complete": "{fid}",
                                            onclick: {
                                                let cur = cannot_restart_draft.clone();
                                                move |_| {
                                                    on_resolve_cannot_restart.call(
                                                        CannotRestartResolutionWire::Complete {
                                                            resolution: cur.note.trim().to_owned(),
                                                        },
                                                    );
                                                }
                                            },
                                            "Complete"
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Quiet,
                                            danger: true,
                                            disabled: busy,
                                            "data-cannot-restart-cancel": "{fid}",
                                            onclick: {
                                                let cur = cannot_restart_draft.clone();
                                                move |_| {
                                                    on_resolve_cannot_restart.call(
                                                        CannotRestartResolutionWire::Cancel {
                                                            reason: cur.note.trim().to_owned(),
                                                        },
                                                    );
                                                }
                                            },
                                            "Cancel"
                                        }
                                    }
                                }
                            }
                        }
                        FollowUpStandingWire::Completed { .. }
                        | FollowUpStandingWire::Cancelled { .. } => rsx! {},
                    }
                }
            }
        }
    }
}
