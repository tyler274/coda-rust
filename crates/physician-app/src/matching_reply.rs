//! Spruce `Messages to match`, `Filtered`, and patient-page reply card editor
//! surfaces using shared `crates/ui` components.

use std::collections::BTreeMap;

use dioxus::prelude::*;
use physician_client::{HttpError, PhysicianClient};
use physician_wire::PatientSearchRow;
use physician_wire::matching_reply::{
    CardArrivalNoticeWire, CardChannelBannerWire, CardReviewStateWire, ChannelSenderRoleWire,
    DisconnectedCardStandingWire, DisconnectedReplyDoneBody, DisplayedReplyTextWire,
    ExpectedDraftBaseWire, FilteredMessageWire, FilteredMessagesResponseWire, MatchSuggestionWire,
    MessagesToMatchResponseWire, MoveDestinationWire, NeedsChoiceReasonWire,
    PatientReplyConversationWire, PendingMoveTargetDisplayWire, PendingSpruceMoveWire,
    PhysicianDraftKindWire, PracticeMessageKindWire, ReplyCardWire,
    RequestReplyProposalRevisionBody, SenderConfirmationPolicyWire, SpruceChannelBannerWire,
    ThisIsAPatientOutcomeWire, UnmatchedMessageWire, UnmatchedUrgencyWire,
};
use ui::{
    ActionRow, Button, ButtonVariant, Card, Checkbox, Dialog, EmptyLine, Icon, IconName, Notice,
    PageTitle, StatePill, TextField, TextKind, Tone, WorkingPresence, copy_text,
};

use crate::shell::{ReadView, SourceChip, TRY_AGAIN, read_view};
use crate::status::ReadNotice;
use crate::today_updates::item_citation_to_chip;

const MATCHING_LOADING: &str = "Loading Messages to match…";
const MATCHING_STALE: &str = "Coda couldn’t refresh Messages to match. Showing the last list.";
const MATCHING_UNAVAILABLE: &str = "Coda couldn’t load Messages to match.";
const FILTERED_LOADING: &str = "Loading Filtered messages…";
const FILTERED_STALE: &str = "Coda couldn’t refresh Filtered messages. Showing the last list.";
const FILTERED_UNAVAILABLE: &str = "Coda couldn’t load Filtered messages.";
const REPLIES_LOADING: &str = "Loading patient messages…";
const REPLIES_STALE: &str = "Coda couldn’t refresh patient messages. Showing the last view.";
const REPLIES_UNAVAILABLE: &str = "Coda couldn’t load messages for this patient.";
const PICKER_SEARCH_UNAVAILABLE: &str = "Coda couldn’t search patients.";
const PICKER_SEARCH_EMPTY: &str = "No patients match.";

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
        HttpError::Status { status: 404, body } => {
            !body.contains("conversation_unlinked") && !body.contains("follow_up_not_found")
        }
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

fn revision_run_words(run: &physician_wire::ReplyRevisionRun) -> Option<&'static str> {
    Some(match run.status.as_str() {
        "pending" => crate::shell::REPLY_REVISION_PENDING,
        "active" => crate::shell::REPLY_REVISION_ACTIVE,
        "completed" => crate::shell::REPLY_REVISION_COMPLETED,
        "failed" => crate::shell::REPLY_REVISION_FAILED,
        "cancelled" => crate::shell::REPLY_REVISION_SUPERSEDED,
        "held" => "Revision paused. Your saved draft is unchanged.",
        _ => return None,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PatientPickerPurpose {
    MatchMessage {
        message_id: String,
        sender_label: String,
    },
    MoveConversation {
        workspace_id: String,
        conversation_id: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum LinkedBannerNotice {
    Linked {
        patient_workspace_id: String,
        patient_display_name: String,
    },
    LeftUnlinked,
}

fn displayed_expected_base(displayed: &DisplayedReplyTextWire) -> Option<ExpectedDraftBaseWire> {
    match displayed {
        DisplayedReplyTextWire::CodaProposal {
            proposal_version, ..
        } => Some(ExpectedDraftBaseWire::ProposalVersion {
            version: *proposal_version,
        }),
        DisplayedReplyTextWire::PhysicianDraft { revision, .. } => {
            Some(ExpectedDraftBaseWire::PhysicianDraftRevision {
                revision: *revision,
            })
        }
        DisplayedReplyTextWire::Withheld { .. } => None,
    }
}

fn displayed_reply_text(displayed: &DisplayedReplyTextWire) -> Option<&str> {
    match displayed {
        DisplayedReplyTextWire::CodaProposal { text, .. }
        | DisplayedReplyTextWire::PhysicianDraft { text, .. } => Some(text.as_str()),
        DisplayedReplyTextWire::Withheld { .. } => None,
    }
}

const fn needs_choice_label(reason: NeedsChoiceReasonWire) -> &'static str {
    match reason {
        NeedsChoiceReasonWire::AmbiguousCandidates => "Multiple patients match the sender details",
        NeedsChoiceReasonWire::ConflictingEvidence => "Conflicting identity details",
        NeedsChoiceReasonWire::NoMatchFound => "No registered patient matched yet",
        NeedsChoiceReasonWire::ClassificationTimedOut => "Waiting for classification",
    }
}

fn attachment_label(att: &physician_wire::matching_reply::SpruceAttachmentWire) -> &str {
    if let Some(name) = att.filename.as_deref().filter(|s| !s.is_empty()) {
        name
    } else if !att.mime_type.is_empty() {
        att.mime_type.as_str()
    } else {
        "Attachment"
    }
}

#[component]
fn SpruceChannelHealthNotice(banner: SpruceChannelBannerWire) -> Element {
    match banner {
        SpruceChannelBannerWire::NotConfigured => rsx! {},
        SpruceChannelBannerWire::ConnectedReading { .. } => rsx! {},
        SpruceChannelBannerWire::Disconnected {
            reason,
            disconnected_at,
        } => rsx! {
            div { class: "mr-banner", "data-spruce-channel-banner": "disconnected",
                Notice {
                    tone: Tone::Warning,
                    icon: Some(IconName::Alert),
                    title: reason,
                    detail: Some(format!("Disconnected {disconnected_at}")),
                }
            }
        },
        SpruceChannelBannerWire::ReadFailed {
            reason,
            last_read_at,
            failed_at: _,
        } => rsx! {
            div { class: "mr-banner", "data-spruce-channel-banner": "read_failed",
                Notice {
                    tone: Tone::Warning,
                    icon: Some(IconName::Alert),
                    title: format!("Coda couldn’t read Spruce: {reason}"),
                    detail: last_read_at.map(|ts| format!("Last read {ts}")),
                }
            }
        },
        SpruceChannelBannerWire::ReconnectRequired {
            reason,
            last_read_at,
            lost_at: _,
        } => rsx! {
            div { class: "mr-banner", "data-spruce-channel-banner": "reconnect_required",
                Notice {
                    tone: Tone::Error,
                    icon: Some(IconName::Alert),
                    title: format!("Spruce needs reconnection: {reason}"),
                    detail: last_read_at.map(|ts| format!("Last read {ts}")),
                }
            }
        },
    }
}

/// Modal dialog for `Choose another patient` (on `Messages to match`) and
/// `Wrong patient?` (on a patient reply card). Surfaces explicit failure when
/// `/patient-roster` fails instead of `"No patients match."`
#[component]
fn PatientPickerDialog(
    client: Signal<PhysicianClient>,
    purpose: PatientPickerPurpose,
    exclude_workspace_id: Option<String>,
    busy: bool,
    command_error: Option<String>,
    on_pick_patient: EventHandler<(String, SenderConfirmationPolicyWire)>,
    on_pick_not_a_patient: Option<EventHandler>,
    on_close: EventHandler,
) -> Element {
    let mut query = use_signal(String::new);
    let mut lookup_attempt = use_signal(|| 0u32);
    let mut also_confirm_sender = use_signal(|| false);

    let roster = use_resource(move || {
        let q = query();
        let _attempt = lookup_attempt();
        let api = client.read().clone();
        async move { api.search(&q).await.map_err(|_| ()) }
    });

    let title = match &purpose {
        PatientPickerPurpose::MatchMessage { sender_label, .. } => {
            format!("Match message from {sender_label}")
        }
        PatientPickerPurpose::MoveConversation { .. } => "Move conversation".to_owned(),
    };
    let show_sender_policy = matches!(purpose, PatientPickerPurpose::MatchMessage { .. });

    let roster_val = roster.read().clone();
    let screen_hidden = crate::is_screen_hidden();

    rsx! {
        if !screen_hidden {
        Dialog {
            title,
            close_label: "Close",
            class: "mr-picker-dialog",
            on_close,
            div {
                class: "mr-picker-body",
                "data-patient-picker-dialog": "true",
                onkeydown: move |ev: KeyboardEvent| ev.stop_propagation(),
                if let Some(err_msg) = command_error {
                    div { class: "mr-banner", "data-picker-command-error": "true",
                        Notice {
                            tone: Tone::Error,
                            icon: Some(IconName::Alert),
                            title: err_msg,
                        }
                    }
                }
                if let Some(not_patient_handler) = on_pick_not_a_patient {
                    div { class: "mr-picker-not-patient",
                        Card {
                            overline: "Non-patient conversation".to_owned(),
                            title: "Not a patient".to_owned(),
                            detail: "Remove this conversation from the patient chart and withhold any draft reply.".to_owned(),
                            ActionRow {
                                Button {
                                    compact: true,
                                    danger: true,
                                    busy,
                                    "data-move-not-a-patient": "true",
                                    onclick: move |_| not_patient_handler.call(()),
                                    "Move to Not a patient"
                                }
                            }
                        }
                    }
                }
                if show_sender_policy {
                    Checkbox {
                        label: "Also confirm this sender address for future messages".to_owned(),
                        detail: Some("Leave unchecked when a shared phone or caregiver may message about more than one person.".to_owned()),
                        checked: also_confirm_sender(),
                        disabled: busy,
                        on_change: move |checked| also_confirm_sender.set(checked),
                    }
                }
                {
                    let (rows_view, match_mode) = match roster_val {
                        None => (
                            crate::shell::ReadView::Loading,
                            physician_wire::SearchMatchMode::ExactOrPrefix,
                        ),
                        Some(Err(())) => (
                            crate::shell::ReadView::Unavailable,
                            physician_wire::SearchMatchMode::ExactOrPrefix,
                        ),
                        Some(Ok(resp)) => {
                            let eligible: Vec<PatientSearchRow> = resp
                                .patients
                                .into_iter()
                                .filter(|p| {
                                    exclude_workspace_id
                                        .as_deref()
                                        .is_none_or(|ex| ex != p.patient_workspace_id)
                                })
                                .collect();
                            (crate::shell::ReadView::Showing(eligible), resp.match_mode)
                        }
                    };
                    rsx! {
                        crate::patient_finder::FindPatientSearch {
                            query: query(),
                            label: "Search practice patients".to_owned(),
                            placeholder: "Name, birth date, or MRN".to_owned(),
                            rows: rows_view,
                            match_mode,
                            continuation: crate::shell::RosterContinuation::Complete,
                            loading_copy: "Searching…".to_owned(),
                            unavailable_copy: PICKER_SEARCH_UNAVAILABLE.to_owned(),
                            empty_copy: PICKER_SEARCH_EMPTY.to_owned(),
                            action: crate::patient_finder::FindPatientAction::ChooseButton { busy },
                            focused: None,
                            on_query: move |next| query.set(next),
                            on_retry: move |_| *lookup_attempt.write() += 1,
                            on_scroll_more: move |_| {},
                            on_retry_more: move |_| {},
                            on_select: move |picked: PatientSearchRow| {
                                let policy = if also_confirm_sender() {
                                    SenderConfirmationPolicyWire::AlsoConfirmSenderForPatient
                                } else {
                                    SenderConfirmationPolicyWire::ConversationOnly
                                };
                                on_pick_patient.call((picked.patient_workspace_id, policy));
                            },
                            on_editing: move |_| {},
                        }
                    }
                }
            }
        }
        }
    }
}

/// Top-level `Messages to match` screen (`Place::MessagesToMatch`).
#[component]
pub(crate) fn MessagesToMatchSurface(
    client: Signal<PhysicianClient>,
    session: crate::SessionState,
    shown: Signal<bool>,
    mut today_attempt: Signal<u32>,
    on_open_filtered: EventHandler,
    on_open_patient: EventHandler<String>,
) -> Element {
    let mut local_attempt = use_signal(|| 0u32);
    let mut cached = use_signal(|| None::<MessagesToMatchResponseWire>);
    let mut busy_message = use_signal(|| None::<String>);
    let mut command_error = use_signal(|| None::<String>);
    let mut outcome_banner = use_signal(|| None::<LinkedBannerNotice>);
    let mut sender_confirm_checks = use_signal(BTreeMap::<String, bool>::new);
    let mut picker_target = use_signal(|| None::<PatientPickerPurpose>);

    let read_res = use_resource(move || {
        let _t = today_attempt();
        let _l = local_attempt();
        let api = client.read().clone();
        let presented = api.session().get();
        async move {
            match api.messages_to_match().await {
                Ok(resp) => {
                    shown.set(true);
                    cached.set(Some(resp.clone()));
                    Ok(resp)
                }
                Err(err) => {
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    }
                    Err(())
                }
            }
        }
    });

    let view: ReadView<MessagesToMatchResponseWire> =
        read_view(cached.read().clone(), read_res.read().clone());

    let mut run_confirm = move |(message_id, workspace_id, policy): (
        String,
        String,
        SenderConfirmationPolicyWire,
    )| {
        if busy_message.peek().is_some() {
            return;
        }
        busy_message.set(Some(message_id.clone()));
        command_error.set(None);
        let api = client.read().clone();
        let presented = api.session().get();
        let key = crate::idempotency_key();
        spawn(async move {
            match api
                .confirm_spruce_message_link(&message_id, &workspace_id, policy, &key)
                .await
            {
                Ok(saved) => {
                    busy_message.set(None);
                    picker_target.set(None);
                    outcome_banner.set(Some(LinkedBannerNotice::Linked {
                        patient_workspace_id: saved.patient_workspace_id,
                        patient_display_name: saved.patient_display_name,
                    }));
                    *local_attempt.write() += 1;
                    *today_attempt.write() += 1;
                }
                Err(err) => {
                    busy_message.set(None);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else {
                        command_error.set(Some(not_saved_message(
                            &err,
                            "Coda couldn’t confirm that patient match.",
                        )));
                    }
                }
            }
        });
    };

    let run_leave_unlinked = move |message_id: String| {
        if busy_message.peek().is_some() {
            return;
        }
        busy_message.set(Some(message_id.clone()));
        command_error.set(None);
        let api = client.read().clone();
        let presented = api.session().get();
        let key = crate::idempotency_key();
        spawn(async move {
            match api.leave_spruce_message_unlinked(&message_id, &key).await {
                Ok(_) => {
                    busy_message.set(None);
                    outcome_banner.set(Some(LinkedBannerNotice::LeftUnlinked));
                    *local_attempt.write() += 1;
                    *today_attempt.write() += 1;
                }
                Err(err) => {
                    busy_message.set(None);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else {
                        command_error.set(Some(not_saved_message(
                            &err,
                            "Coda couldn’t leave that message unlinked.",
                        )));
                    }
                }
            }
        });
    };

    let mut run_unreadable_done = move |message_id: String| {
        if busy_message.peek().is_some() {
            return;
        }
        busy_message.set(Some(message_id.clone()));
        command_error.set(None);
        let api = client.read().clone();
        let presented = api.session().get();
        let key = crate::idempotency_key();
        spawn(async move {
            match api
                .mark_unreadable_spruce_message_done(&message_id, &key)
                .await
            {
                Ok(_) => {
                    busy_message.set(None);
                    *local_attempt.write() += 1;
                    *today_attempt.write() += 1;
                }
                Err(err) => {
                    busy_message.set(None);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else {
                        command_error.set(Some(not_saved_message(
                            &err,
                            "Coda couldn’t mark that item done.",
                        )));
                    }
                }
            }
        });
    };

    let filtered_count = view.content().map_or(0, |d| d.filtered_count);

    rsx! {
        div { class: "reading-workspace", "data-surface": "messages-to-match",
            PageTitle {
                text: "Messages to match".to_owned(),
                detail: Some("Spruce messages waiting for a confirmed patient chart.".to_owned()),
                actions: Some(rsx! {
                    Button {
                        compact: true,
                        variant: ButtonVariant::Secondary,
                        "data-open-filtered": "true",
                        onclick: move |_| on_open_filtered.call(()),
                        "Filtered ({filtered_count})"
                    }
                }),
            }
            ReadNotice {
                stage: view.stage(),
                loading: MATCHING_LOADING,
                stale: MATCHING_STALE,
                unavailable: MATCHING_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry: move |_| *local_attempt.write() += 1,
            }
            if let Some(err_text) = command_error() {
                div { class: "mr-banner", "data-matching-command-error": "true",
                    Notice {
                        tone: Tone::Error,
                        icon: Some(IconName::Alert),
                        title: err_text,
                        ActionRow {
                            Button {
                                compact: true,
                                variant: ButtonVariant::Quiet,
                                onclick: move |_| command_error.set(None),
                                "Dismiss"
                            }
                        }
                    }
                }
            }
            if let Some(done_notice) = outcome_banner() {
                div { class: "mr-banner", "data-matching-outcome-banner": "true",
                    match done_notice {
                        LinkedBannerNotice::Linked {
                            patient_workspace_id,
                            patient_display_name,
                        } => rsx! {
                            Notice {
                                tone: Tone::Primary,
                                icon: Some(IconName::Check),
                                title: format!("Linked message to {patient_display_name}."),
                                ActionRow {
                                    Button {
                                        compact: true,
                                        variant: ButtonVariant::Primary,
                                        "data-open-linked-patient": "{patient_workspace_id}",
                                        onclick: {
                                            let ws = patient_workspace_id.clone();
                                            move |_| on_open_patient.call(ws.clone())
                                        },
                                        "Open patient"
                                    }
                                    Button {
                                        compact: true,
                                        variant: ButtonVariant::Quiet,
                                        onclick: move |_| outcome_banner.set(None),
                                        "Dismiss"
                                    }
                                }
                            }
                        },
                        LinkedBannerNotice::LeftUnlinked => rsx! {
                            Notice {
                                tone: Tone::Neutral,
                                icon: Some(IconName::Check),
                                title: "Left message unlinked in Spruce.".to_owned(),
                            }
                        },
                    }
                }
            }
            if let Some(data) = view.content().cloned() {
                crate::access_notice::MayBeMissingSomeoneNotice {
                    visible: data.may_be_missing_someone,
                }
                SpruceChannelHealthNotice { banner: data.channel_banner }
                if data.messages.is_empty()
                    && data.unreadable_messages.is_empty()
                    && !data.may_be_missing_someone
                {
                    EmptyLine { message: "No Spruce messages are waiting to be matched.".to_owned() }
                } else {
                    div { class: "mr-stack",
                        for msg in data.messages {
                            UnmatchedMessageCard {
                                key: "{msg.message_id}",
                                msg: msg.clone(),
                                busy: busy_message.read().as_deref() == Some(msg.message_id.as_str()),
                                also_confirm_sender: sender_confirm_checks
                                    .read()
                                    .get(&msg.message_id)
                                    .copied()
                                    .unwrap_or(false),
                                on_toggle_sender_policy: {
                                    let mid = msg.message_id.clone();
                                    move |checked: bool| {
                                        sender_confirm_checks.write().insert(mid.clone(), checked);
                                    }
                                },
                                on_confirm: run_confirm,
                                on_choose_other: {
                                    let mid = msg.message_id.clone();
                                    let sender = msg.sender_display_name.clone();
                                    move |_| {
                                        command_error.set(None);
                                        picker_target.set(Some(PatientPickerPurpose::MatchMessage {
                                            message_id: mid.clone(),
                                            sender_label: sender.clone(),
                                        }));
                                    }
                                },
                                on_leave_unlinked: run_leave_unlinked,
                            }
                        }
                        for unread in data.unreadable_messages {
                            Card {
                                key: "{unread.message_id}",
                                "data-unreadable-row": "{unread.message_id}",
                                overline: "Spruce message".to_owned(),
                                title: "Couldn’t read message in Coda".to_owned(),
                                detail: unread.failure_reason.clone(),
                                ActionRow {
                                    if let Some(url) = unread.spruce_app_url.as_deref() {
                                        a {
                                            class: "citation-open",
                                            href: "{url}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            "data-open-in-spruce": "{unread.message_id}",
                                            "Open in Spruce"
                                        }
                                    }
                                    Button {
                                        compact: true,
                                        busy: busy_message.read().as_deref() == Some(unread.message_id.as_str()),
                                        "data-unreadable-done": "{unread.message_id}",
                                        onclick: {
                                            let mid = unread.message_id.clone();
                                            move |_| run_unreadable_done(mid.clone())
                                        },
                                        "Done"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if let Some(purpose) = picker_target() {
                PatientPickerDialog {
                    client,
                    purpose: purpose.clone(),
                    exclude_workspace_id: None,
                    busy: busy_message().is_some(),
                    command_error: command_error(),
                    on_pick_patient: {
                        let purpose = purpose.clone();
                        move |(ws_id, policy): (String, SenderConfirmationPolicyWire)| {
                            if let PatientPickerPurpose::MatchMessage { message_id, .. } = &purpose {
                                run_confirm((message_id.clone(), ws_id, policy));
                            }
                        }
                    },
                    on_pick_not_a_patient: None,
                    on_close: move |_| picker_target.set(None),
                }
            }
        }
    }
}

#[component]
fn UnmatchedMessageCard(
    msg: UnmatchedMessageWire,
    busy: bool,
    also_confirm_sender: bool,
    on_toggle_sender_policy: EventHandler<bool>,
    on_confirm: EventHandler<(String, String, SenderConfirmationPolicyWire)>,
    on_choose_other: EventHandler,
    on_leave_unlinked: EventHandler<String>,
) -> Element {
    let is_now = matches!(msg.urgency, UnmatchedUrgencyWire::Urgent { .. });
    let urgency_attr = if is_now { "now" } else { "today" };
    let sender_line = match msg.sender_address.as_deref() {
        Some(addr) if !addr.is_empty() && addr != msg.sender_display_name => {
            format!("{} · {addr}", msg.sender_display_name)
        }
        _ => msg.sender_display_name.clone(),
    };
    let card_detail = match &msg.suggestion {
        MatchSuggestionWire::SinglePatient { .. } => {
            "Suggested patient match ready for confirmation".to_owned()
        }
        MatchSuggestionWire::NeedsPhysicianChoice { reason, .. } => {
            needs_choice_label(*reason).to_owned()
        }
    };

    rsx! {
        Card {
            "data-unmatched-row": "{msg.message_id}",
            "data-unmatched-urgency": "{urgency_attr}",
            overline: "Spruce channel sender".to_owned(),
            title: sender_line,
            detail: card_detail,
            div { class: "mr-card-inner",
                if let UnmatchedUrgencyWire::Urgent { reason, .. } = &msg.urgency {
                    div { class: "mr-urgent-row", "data-urgent-now-pill": "{msg.message_id}",
                        StatePill { tone: Tone::Error, icon: Some(IconName::Alert), "Now" }
                        span { class: "mr-urgent-reason", "{reason}" }
                    }
                }
                p { class: "mr-message-body", "{msg.body_text}" }
                if !msg.attachments.is_empty() {
                    div { class: "mr-attachments", "data-message-attachments": "{msg.message_id}",
                        for (idx, att) in msg.attachments.iter().enumerate() {
                            span { key: "{idx}", class: "mr-attachment-chip",
                                Icon { name: IconName::File }
                                "{attachment_label(att)}"
                            }
                        }
                    }
                }
                match &msg.suggestion {
                    MatchSuggestionWire::SinglePatient {
                        patient_workspace_id,
                        patient_display_name,
                        birth_date,
                        mrn,
                        evidence_summary,
                    } => rsx! {
                        div {
                            class: "mr-suggestion-box",
                            "data-suggested-patient": "{patient_workspace_id}",
                            div { class: "mr-suggestion-header",
                                StatePill { tone: Tone::Primary, icon: Some(IconName::User), "Suggested patient" }
                                strong { "{patient_display_name}" }
                                span { class: "mr-picker-meta",
                                    if let Some(dob) = birth_date.as_deref() {
                                        "DOB {dob}"
                                    }
                                    if let Some(m) = mrn.as_deref() {
                                        " · MRN {m}"
                                    }
                                }
                            }
                            if !evidence_summary.is_empty() {
                                p { class: "mr-rationale", "{evidence_summary}" }
                            }
                            Checkbox {
                                label: "Also confirm this sender address for future messages".to_owned(),
                                detail: None,
                                checked: also_confirm_sender,
                                disabled: busy,
                                on_change: move |checked| on_toggle_sender_policy.call(checked),
                            }
                        }
                    },
                    MatchSuggestionWire::NeedsPhysicianChoice {
                        evidence_summary,
                        candidate_patients,
                        ..
                    } => rsx! {
                        if !evidence_summary.is_empty() || !candidate_patients.is_empty() {
                            div { class: "mr-suggestion-box", "data-needs-choice-box": "{msg.message_id}",
                                if !evidence_summary.is_empty() {
                                    p { class: "mr-rationale", "{evidence_summary}" }
                                }
                                if !candidate_patients.is_empty() {
                                    div { class: "mr-picker-results",
                                        for cand in candidate_patients {
                                            div {
                                                key: "{cand.patient_workspace_id}",
                                                class: "mr-picker-row",
                                                "data-candidate-patient": "{cand.patient_workspace_id}",
                                                div { class: "mr-picker-who",
                                                    strong { "{cand.patient_display_name}" }
                                                    span { class: "mr-picker-meta",
                                                        if let Some(dob) = cand.birth_date.as_deref() {
                                                            "DOB {dob}"
                                                        }
                                                        if let Some(m) = cand.mrn.as_deref() {
                                                            " · MRN {m}"
                                                        }
                                                    }
                                                }
                                                Button {
                                                    compact: true,
                                                    variant: ButtonVariant::Secondary,
                                                    disabled: busy,
                                                    "data-confirm-candidate": "{cand.patient_workspace_id}",
                                                    onclick: {
                                                        let mid = msg.message_id.clone();
                                                        let ws = cand.patient_workspace_id.clone();
                                                        move |_| {
                                                            let policy = if also_confirm_sender {
                                                                SenderConfirmationPolicyWire::AlsoConfirmSenderForPatient
                                                            } else {
                                                                SenderConfirmationPolicyWire::ConversationOnly
                                                            };
                                                            on_confirm.call((mid.clone(), ws.clone(), policy));
                                                        }
                                                    },
                                                    "Confirm"
                                                }
                                            }
                                        }
                                    }
                                    Checkbox {
                                        label: "Also confirm this sender address for future messages".to_owned(),
                                        detail: None,
                                        checked: also_confirm_sender,
                                        disabled: busy,
                                        on_change: move |checked| on_toggle_sender_policy.call(checked),
                                    }
                                }
                            }
                        }
                    },
                }
                ActionRow {
                    if let MatchSuggestionWire::SinglePatient {
                        patient_workspace_id,
                        ..
                    } = &msg.suggestion
                    {
                        Button {
                            compact: true,
                            variant: ButtonVariant::Primary,
                            busy,
                            "data-confirm-match": "{msg.message_id}",
                            onclick: {
                                let mid = msg.message_id.clone();
                                let ws = patient_workspace_id.clone();
                                move |_| {
                                    let policy = if also_confirm_sender {
                                        SenderConfirmationPolicyWire::AlsoConfirmSenderForPatient
                                    } else {
                                        SenderConfirmationPolicyWire::ConversationOnly
                                    };
                                    on_confirm.call((mid.clone(), ws.clone(), policy));
                                }
                            },
                            "Confirm"
                        }
                    }
                    Button {
                        compact: true,
                        variant: ButtonVariant::Secondary,
                        disabled: busy,
                        "data-choose-another-patient": "{msg.message_id}",
                        onclick: move |_| on_choose_other.call(()),
                        "Choose another patient"
                    }
                    Button {
                        compact: true,
                        variant: ButtonVariant::Quiet,
                        disabled: busy,
                        "data-leave-unlinked": "{msg.message_id}",
                        onclick: {
                            let mid = msg.message_id.clone();
                            move |_| on_leave_unlinked.call(mid.clone())
                        },
                        "Leave unlinked"
                    }
                    if let Some(url) = msg.spruce_app_url.as_deref() {
                        a {
                            class: "citation-open",
                            href: "{url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "data-open-in-spruce": "{msg.message_id}",
                            "Open in Spruce"
                        }
                    }
                }
            }
        }
    }
}

/// `Filtered` non-patient Spruce mail view (`Place::Filtered`).
#[component]
pub(crate) fn FilteredMessagesSurface(
    client: Signal<PhysicianClient>,
    session: crate::SessionState,
    shown: Signal<bool>,
    mut today_attempt: Signal<u32>,
    on_back_to_matching: EventHandler,
    on_open_patient: EventHandler<String>,
) -> Element {
    let mut local_attempt = use_signal(|| 0u32);
    let mut cached = use_signal(|| None::<FilteredMessagesResponseWire>);
    let mut busy_message = use_signal(|| None::<String>);
    let mut command_error = use_signal(|| None::<String>);
    let mut routing_outcome = use_signal(|| None::<ThisIsAPatientOutcomeWire>);

    let read_res = use_resource(move || {
        let _t = today_attempt();
        let _l = local_attempt();
        let api = client.read().clone();
        let presented = api.session().get();
        async move {
            match api.filtered_spruce_messages().await {
                Ok(resp) => {
                    shown.set(true);
                    cached.set(Some(resp.clone()));
                    Ok(resp)
                }
                Err(err) => {
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    }
                    Err(())
                }
            }
        }
    });

    let view: ReadView<FilteredMessagesResponseWire> =
        read_view(cached.read().clone(), read_res.read().clone());

    let run_this_is_a_patient = move |message_id: String| {
        if busy_message.peek().is_some() {
            return;
        }
        busy_message.set(Some(message_id.clone()));
        command_error.set(None);
        let api = client.read().clone();
        let presented = api.session().get();
        let key = crate::idempotency_key();
        spawn(async move {
            match api.mark_spruce_message_as_patient(&message_id, &key).await {
                Ok(outcome) => {
                    busy_message.set(None);
                    routing_outcome.set(Some(outcome));
                    *local_attempt.write() += 1;
                    *today_attempt.write() += 1;
                }
                Err(err) => {
                    busy_message.set(None);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else {
                        command_error.set(Some(not_saved_message(
                            &err,
                            "Coda couldn’t return that message to patient routing.",
                        )));
                    }
                }
            }
        });
    };

    rsx! {
        div { class: "reading-workspace", "data-surface": "filtered-messages",
            PageTitle {
                text: "Filtered".to_owned(),
                detail: Some("Non-patient Spruce mail classified outside patient charts.".to_owned()),
                actions: Some(rsx! {
                    Button {
                        compact: true,
                        variant: ButtonVariant::Secondary,
                        "data-back-to-matching": "true",
                        onclick: move |_| on_back_to_matching.call(()),
                        Icon { name: IconName::ArrowLeft }
                        "Messages to match"
                    }
                }),
            }
            ReadNotice {
                stage: view.stage(),
                loading: FILTERED_LOADING,
                stale: FILTERED_STALE,
                unavailable: FILTERED_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry: move |_| *local_attempt.write() += 1,
            }
            if let Some(err_text) = command_error() {
                div { class: "mr-banner", "data-filtered-command-error": "true",
                    Notice {
                        tone: Tone::Error,
                        icon: Some(IconName::Alert),
                        title: err_text,
                    }
                }
            }
            if let Some(outcome) = routing_outcome() {
                div { class: "mr-banner", "data-this-is-patient-outcome": "true",
                    match outcome {
                        ThisIsAPatientOutcomeWire::Linked {
                            patient_workspace_id,
                            patient_display_name,
                            ..
                        } => {
                            let label = patient_display_name
                                .unwrap_or_else(|| "patient chart".to_owned());
                            rsx! {
                                Notice {
                                    tone: Tone::Primary,
                                    icon: Some(IconName::Check),
                                    title: format!("Linked message to {label}."),
                                    ActionRow {
                                        if let Some(ws) = patient_workspace_id {
                                            Button {
                                                compact: true,
                                                variant: ButtonVariant::Primary,
                                                "data-open-routed-patient": "{ws}",
                                                onclick: {
                                                    let ws = ws.clone();
                                                    move |_| on_open_patient.call(ws.clone())
                                                },
                                                "Open patient"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        ThisIsAPatientOutcomeWire::WaitingInMessagesToMatch { .. } => rsx! {
                            Notice {
                                tone: Tone::Primary,
                                icon: Some(IconName::Check),
                                title: "Returned message to Messages to match.".to_owned(),
                                ActionRow {
                                    Button {
                                        compact: true,
                                        variant: ButtonVariant::Secondary,
                                        "data-view-returned-matching": "true",
                                        onclick: move |_| on_back_to_matching.call(()),
                                        "Open Messages to match"
                                    }
                                }
                            }
                        },
                    }
                }
            }
            if let Some(data) = view.content().cloned() {
                if data.messages.is_empty() {
                    EmptyLine { message: "No filtered non-patient messages.".to_owned() }
                } else {
                    div { class: "mr-stack",
                        for msg in data.messages {
                            FilteredMessageCard {
                                key: "{msg.message_id}",
                                msg: msg.clone(),
                                busy: busy_message.read().as_deref() == Some(msg.message_id.as_str()),
                                on_this_is_patient: run_this_is_a_patient,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FilteredMessageCard(
    msg: FilteredMessageWire,
    busy: bool,
    on_this_is_patient: EventHandler<String>,
) -> Element {
    let sender_line = match msg.sender_address.as_deref() {
        Some(addr) if !addr.is_empty() && addr != msg.sender_display_name => {
            format!("{} · {addr}", msg.sender_display_name)
        }
        _ => msg.sender_display_name.clone(),
    };
    rsx! {
        Card {
            "data-filtered-row": "{msg.message_id}",
            overline: msg.category.label().to_owned(),
            title: sender_line,
            detail: msg.rationale.clone(),
            div { class: "mr-card-inner",
                p { class: "mr-message-body", "{msg.body_text}" }
                ActionRow {
                    Button {
                        compact: true,
                        variant: ButtonVariant::Primary,
                        busy,
                        "data-this-is-a-patient": "{msg.message_id}",
                        onclick: {
                            let mid = msg.message_id.clone();
                            move |_| on_this_is_patient.call(mid.clone())
                        },
                        "This is a patient"
                    }
                    if let Some(url) = msg.spruce_app_url.as_deref() {
                        a {
                            class: "citation-open",
                            href: "{url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Open in Spruce"
                        }
                    }
                }
            }
        }
    }
}

/// Reactive read state for the open on-panel patient's Spruce conversations,
/// registered in `PhysicianShell` ahead of `snapshot` so chat snapshot reads stay last.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct PatientSpruceRepliesRead {
    pub(crate) local_attempt: Signal<u32>,
    pub(crate) cached: Signal<Option<(String, Vec<PatientReplyConversationWire>)>>,
    pub(crate) read_res: Resource<(String, Result<Vec<PatientReplyConversationWire>, ()>)>,
}

pub(crate) fn use_patient_spruce_replies(
    on_panel_workspace: Memo<Option<String>>,
    context: crate::patient_reviews_panel::PatientReviewsContext,
    on_patient_gone: Callback<String>,
) -> PatientSpruceRepliesRead {
    let local_attempt = use_signal(|| 0u32);
    let mut cached = use_signal(|| None::<(String, Vec<PatientReplyConversationWire>)>);
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
                return (ws, Ok(Vec::new()));
            }
            match api.patient_spruce_replies(&ws).await {
                Ok(resp) => {
                    shown.set(true);
                    cached.set(Some((ws.clone(), resp.conversations.clone())));
                    (ws, Ok(resp.conversations))
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

    PatientSpruceRepliesRead {
        local_attempt,
        cached,
        read_res,
    }
}

/// Scopes unsaved inline draft text and its pinned `ExpectedDraftBaseWire` to
/// `(patient_workspace_id, conversation_id, reply_id)` so moves across patients
/// or reopened reply cycles never leak edits or overwrite newer proposals.
type DraftEditKey = (String, String, u64);

/// Patient-page Spruce conversations & reply editor section.
#[component]
pub(crate) fn PatientSpruceRepliesSection(
    workspace_id: String,
    patient_name: String,
    browser_tz: String,
    read_state: PatientSpruceRepliesRead,
    client: Signal<PhysicianClient>,
    session: crate::SessionState,
    shown: Signal<bool>,
    workspace_attempt: Signal<u32>,
    mut today_attempt: Signal<u32>,
    on_open_source: EventHandler<SourceChip>,
    on_open_basis: EventHandler<(String, String)>,
    on_open_patient: EventHandler<String>,
    on_patient_gone: EventHandler<String>,
) -> Element {
    let _ = &patient_name;
    let mut local_attempt = read_state.local_attempt;
    let mut cached = read_state.cached;
    let read_res = read_state.read_res;
    // Local unsaved draft edits keyed by `(workspace_id, conversation_id, reply_id)`,
    // storing `(pinned_base_when_edit_began, edited_text)`.
    // Preserved across failed commands, background reviews, and newer Coda update offers!
    let mut draft_edits =
        use_signal(BTreeMap::<DraftEditKey, (ExpectedDraftBaseWire, String)>::new);
    let mut revise_instructions = use_signal(BTreeMap::<DraftEditKey, String>::new);
    let mut busy_conv = use_signal(|| None::<String>);
    let mut card_errors = use_signal(BTreeMap::<String, String>::new);
    let mut copied_notices = use_signal(BTreeMap::<String, String>::new);
    let mut move_dialog = use_signal(|| None::<String>);
    let mut move_error = use_signal(|| None::<String>);

    let previous = cached
        .read()
        .as_ref()
        .filter(|(asked, _)| asked == &workspace_id)
        .map(|(_, list)| list.clone());
    let current = read_res
        .read()
        .as_ref()
        .filter(|(asked, _)| asked == &workspace_id)
        .map(|(_, ans)| ans.clone());
    let view = read_view(previous, current);

    // Poll gently while any conversation has an active Coda review or revision run.
    #[cfg(target_arch = "wasm32")]
    {
        let any_running = view.content().is_some_and(|convs| {
            convs.iter().any(|c| match c {
                PatientReplyConversationWire::OpenCard(card) => {
                    matches!(card.review_state, CardReviewStateWire::Reviewing { .. })
                        || card
                            .revision_run
                            .as_ref()
                            .is_some_and(physician_wire::ReplyRevisionRun::running)
                }
                PatientReplyConversationWire::NoOpenCard { review_state, .. } => {
                    matches!(review_state, CardReviewStateWire::Reviewing { .. })
                }
                PatientReplyConversationWire::ClosedUndoBanner { .. } => false,
            })
        });
        use_effect(use_reactive!(|any_running| {
            if any_running {
                spawn(async move {
                    crate::sleep_ms(1_500).await;
                    *local_attempt.write() += 1;
                });
            }
        }));
    }

    let apply_updated_surface = {
        let ws = workspace_id.clone();
        move |updated: PatientReplyConversationWire| {
            let mut cache_guard = cached.write();
            if let Some((cached_ws, list)) = cache_guard.as_mut()
                && cached_ws == &ws
            {
                let target_cid = match &updated {
                    PatientReplyConversationWire::OpenCard(c) => c.conversation_id.as_str(),
                    PatientReplyConversationWire::ClosedUndoBanner {
                        conversation_id, ..
                    } => conversation_id.as_str(),
                    PatientReplyConversationWire::NoOpenCard {
                        conversation_id, ..
                    } => conversation_id.as_str(),
                };
                if let Some(slot) = list.iter_mut().find(|item| {
                    let cid = match item {
                        PatientReplyConversationWire::OpenCard(c) => c.conversation_id.as_str(),
                        PatientReplyConversationWire::ClosedUndoBanner {
                            conversation_id, ..
                        } => conversation_id.as_str(),
                        PatientReplyConversationWire::NoOpenCard {
                            conversation_id, ..
                        } => conversation_id.as_str(),
                    };
                    cid == target_cid
                }) {
                    *slot = updated;
                }
            }
            *local_attempt.write() += 1;
            *today_attempt.write() += 1;
        }
    };

    // If healthy and empty, stay quiet on the patient page unless loading failed.
    if matches!(view, ReadView::Loading) {
        return rsx! {};
    }
    if let ReadView::Showing(list) = &view
        && list.is_empty()
    {
        return rsx! {};
    }

    let ws_for_move = workspace_id.clone();
    let run_move = move |(cid, dest): (String, MoveDestinationWire)| {
        if busy_conv.peek().is_some() {
            return;
        }
        busy_conv.set(Some(cid.clone()));
        move_error.set(None);
        card_errors.write().remove(&cid);
        let api = client.read().clone();
        let presented = api.session().get();
        let ws = ws_for_move.clone();
        let key = crate::idempotency_key();
        spawn(async move {
            match api
                .move_spruce_conversation(&ws, &cid, dest, None, &key)
                .await
            {
                Ok(_) => {
                    busy_conv.set(None);
                    move_dialog.set(None);
                    *local_attempt.write() += 1;
                    *today_attempt.write() += 1;
                }
                Err(err) => {
                    busy_conv.set(None);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else if is_patient_access_lost_on_command(&err) {
                        move_dialog.set(None);
                        on_patient_gone.call(ws);
                    } else {
                        let msg = not_saved_message(&err, "Coda couldn’t move that conversation.");
                        move_error.set(Some(msg.clone()));
                        card_errors.write().insert(cid, msg);
                    }
                }
            }
        });
    };

    let ws_for_undo_move = workspace_id.clone();
    let run_undo_move = move |cid: String| {
        if busy_conv.peek().is_some() {
            return;
        }
        busy_conv.set(Some(cid.clone()));
        card_errors.write().remove(&cid);
        let api = client.read().clone();
        let presented = api.session().get();
        let ws = ws_for_undo_move.clone();
        let key = crate::idempotency_key();
        spawn(async move {
            match api.undo_spruce_conversation_move(&ws, &cid, &key).await {
                Ok(_) => {
                    busy_conv.set(None);
                    *local_attempt.write() += 1;
                    *today_attempt.write() += 1;
                }
                Err(err) => {
                    busy_conv.set(None);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else if is_patient_access_lost_on_command(&err) {
                        on_patient_gone.call(ws);
                    } else {
                        card_errors.write().insert(
                            cid,
                            not_saved_message(&err, "Coda couldn’t undo that move."),
                        );
                    }
                }
            }
        });
    };

    rsx! {
        section {
            class: "patient-section",
            aria_label: "Spruce messages",
            "data-patient-spruce-section": "true",
            onkeydown: move |ev: KeyboardEvent| ev.stop_propagation(),
            h2 { "Spruce messages" }
            ReadNotice {
                stage: view.stage(),
                loading: REPLIES_LOADING,
                stale: REPLIES_STALE,
                unavailable: REPLIES_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry: move |_| *local_attempt.write() += 1,
            }
            if let Some(conversations) = view.content().cloned() {
                div { class: "mr-stack",
                    for conv in conversations {
                        match conv {
                            PatientReplyConversationWire::OpenCard(card) => {
                                let cid = card.conversation_id.clone();
                                let edit_key: DraftEditKey =
                                    (workspace_id.clone(), cid.clone(), card.reply_id);
                                let server_base = displayed_expected_base(&card.displayed_reply);
                                let server_text = displayed_reply_text(&card.displayed_reply)
                                    .unwrap_or_default()
                                    .to_owned();
                                let existing_edit = draft_edits.read().get(&edit_key).cloned();
                                let pinned_base = existing_edit.as_ref().map(|(b, _)| *b);
                                let current_draft_text = existing_edit
                                    .map(|(_, txt)| txt)
                                    .unwrap_or_else(|| server_text.clone());
                                let current_revise_inst = revise_instructions
                                    .read()
                                    .get(&edit_key)
                                    .cloned()
                                    .unwrap_or_default();
                                let err_msg = card_errors.read().get(&cid).cloned();
                                let copied_msg = copied_notices.read().get(&cid).cloned();
                                let is_busy = busy_conv.read().as_deref() == Some(cid.as_str());

                                rsx! {
                                    OpenReplyCardView {
                                        key: "{cid}:{card.reply_id}",
                                        workspace_id: workspace_id.clone(),
                                        card: *card,
                                        pinned_base,
                                        draft_text: current_draft_text,
                                        revise_instruction: current_revise_inst,
                                        busy: is_busy,
                                        error_message: err_msg,
                                        copied_notice: copied_msg,
                                        on_edit_text: {
                                            let edit_key = edit_key.clone();
                                            let base = server_base;
                                            move |next_text: String| {
                                                let mut guard = draft_edits.write();
                                                if let Some(entry) = guard.get_mut(&edit_key) {
                                                    // Keep the pinned `ExpectedDraftBaseWire` from when editing began!
                                                    entry.1 = next_text;
                                                } else if let Some(initial_base) = base {
                                                    guard.insert(
                                                        edit_key.clone(),
                                                        (initial_base, next_text),
                                                    );
                                                }
                                            }
                                        },
                                        on_discard_stale_edits: {
                                            let edit_key = edit_key.clone();
                                            let cid = cid.clone();
                                            move |_| {
                                                draft_edits.write().remove(&edit_key);
                                                card_errors.write().remove(&cid);
                                            }
                                        },
                                        on_edit_revise_instruction: {
                                            let edit_key = edit_key.clone();
                                            move |next_inst: String| {
                                                revise_instructions
                                                    .write()
                                                    .insert(edit_key.clone(), next_inst);
                                            }
                                        },
                                        on_save_draft: {
                                            let ws = workspace_id.clone();
                                            let cid = cid.clone();
                                            let edit_key = edit_key.clone();
                                            let apply = apply_updated_surface.clone();
                                            move |(expected_base, text_to_save): (ExpectedDraftBaseWire, String)| {
                                                if busy_conv.peek().is_some() {
                                                    return;
                                                }
                                                busy_conv.set(Some(cid.clone()));
                                                card_errors.write().remove(&cid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let ws = ws.clone();
                                                let cid = cid.clone();
                                                let edit_key = edit_key.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api.save_reply_proposal_draft(&ws, &cid, expected_base, &text_to_save).await {
                                                        Ok(updated) => {
                                                            busy_conv.set(None);
                                                            draft_edits.write().remove(&edit_key);
                                                            apply(updated);
                                                        }
                                                        Err(err) => {
                                                            busy_conv.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                card_errors.write().insert(
                                                                    cid,
                                                                    not_saved_message(&err, "Coda couldn’t save your reply draft."),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_request_revision: {
                                            let ws = workspace_id.clone();
                                            let cid = cid.clone();
                                            let edit_key = edit_key.clone();
                                            let tz = browser_tz.clone();
                                            let apply = apply_updated_surface.clone();
                                            move |(expected_base, current_text, instruction): (ExpectedDraftBaseWire, String, String)| {
                                                if busy_conv.peek().is_some() {
                                                    return;
                                                }
                                                busy_conv.set(Some(cid.clone()));
                                                card_errors.write().remove(&cid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let cid = cid.clone();
                                                let edit_key = edit_key.clone();
                                                let tz = tz.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    let body = RequestReplyProposalRevisionBody {
                                                        expected_base,
                                                        current_text,
                                                        instruction,
                                                        physician_time_zone: tz,
                                                        idempotency_key: key,
                                                    };
                                                    match api.request_reply_proposal_revision(&ws, &cid, &body).await {
                                                        Ok(updated) => {
                                                            busy_conv.set(None);
                                                            revise_instructions.write().remove(&edit_key);
                                                            draft_edits.write().remove(&edit_key);
                                                            apply(updated);
                                                        }
                                                        Err(err) => {
                                                            busy_conv.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                card_errors.write().insert(
                                                                    cid,
                                                                    not_saved_message(&err, "Coda couldn’t start that revision."),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_restore_my_version: {
                                            let ws = workspace_id.clone();
                                            let cid = cid.clone();
                                            let edit_key = edit_key.clone();
                                            let apply = apply_updated_surface.clone();
                                            move |expected_revision: u64| {
                                                if busy_conv.peek().is_some() {
                                                    return;
                                                }
                                                busy_conv.set(Some(cid.clone()));
                                                card_errors.write().remove(&cid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let cid = cid.clone();
                                                let edit_key = edit_key.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api.restore_my_reply_version(&ws, &cid, expected_revision, &key).await {
                                                        Ok(updated) => {
                                                            busy_conv.set(None);
                                                            draft_edits.write().remove(&edit_key);
                                                            apply(updated);
                                                        }
                                                        Err(err) => {
                                                            busy_conv.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                card_errors.write().insert(
                                                                    cid,
                                                                    not_saved_message(&err, "Coda couldn’t restore your version."),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_choose_update: {
                                            let ws = workspace_id.clone();
                                            let cid = cid.clone();
                                            let edit_key = edit_key.clone();
                                            let apply = apply_updated_surface.clone();
                                            move |(expected_draft_revision, target_proposal_version): (u64, u64)| {
                                                if busy_conv.peek().is_some() {
                                                    return;
                                                }
                                                busy_conv.set(Some(cid.clone()));
                                                card_errors.write().remove(&cid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let cid = cid.clone();
                                                let edit_key = edit_key.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api
                                                        .choose_coda_reply_update(
                                                            &ws,
                                                            &cid,
                                                            expected_draft_revision,
                                                            target_proposal_version,
                                                            &key,
                                                        )
                                                        .await
                                                    {
                                                        Ok(updated) => {
                                                            busy_conv.set(None);
                                                            draft_edits.write().remove(&edit_key);
                                                            apply(updated);
                                                        }
                                                        Err(err) => {
                                                            busy_conv.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                card_errors.write().insert(
                                                                    cid,
                                                                    not_saved_message(&err, "Coda couldn’t apply that update."),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_copy_reply: {
                                            let ws = workspace_id.clone();
                                            let cid = cid.clone();
                                            let edit_key = edit_key.clone();
                                            let apply = apply_updated_surface.clone();
                                            move |(expected_base, current_text, server_saved_text): (ExpectedDraftBaseWire, String, String)| {
                                                if busy_conv.peek().is_some() {
                                                    return;
                                                }
                                                busy_conv.set(Some(cid.clone()));
                                                card_errors.write().remove(&cid);
                                                copied_notices.write().remove(&cid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let ws = ws.clone();
                                                let cid = cid.clone();
                                                let edit_key = edit_key.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    let mut base_for_copy = expected_base;
                                                    // If the physician has unsaved inline edits, save them first so `copy_reply`
                                                    // records the exact displayed text at its saved draft revision!
                                                    if current_text != server_saved_text {
                                                        match api
                                                            .save_reply_proposal_draft(
                                                                &ws,
                                                                &cid,
                                                                base_for_copy,
                                                                &current_text,
                                                            )
                                                            .await
                                                        {
                                                            Ok(saved_surface) => {
                                                                if let PatientReplyConversationWire::OpenCard(ref sc) = saved_surface
                                                                    && let Some(nb) = displayed_expected_base(&sc.displayed_reply)
                                                                {
                                                                    base_for_copy = nb;
                                                                }
                                                            }
                                                            Err(err) => {
                                                                busy_conv.set(None);
                                                                if let Some(refused) = crate::refusal(&err) {
                                                                    session.refused(refused, *shown.peek(), presented);
                                                                } else if is_patient_access_lost_on_command(&err) {
                                                                    on_patient_gone.call(ws);
                                                                } else {
                                                                    card_errors.write().insert(
                                                                        cid,
                                                                        not_saved_message(&err, "Coda couldn’t save your draft before copying."),
                                                                    );
                                                                }
                                                                return;
                                                            }
                                                        }
                                                    }

                                                    match api
                                                        .copy_spruce_reply(
                                                            &ws,
                                                            &cid,
                                                            base_for_copy,
                                                            &crate::idempotency_key(),
                                                        )
                                                        .await
                                                    {
                                                        Ok(copied_resp) => {
                                                            let clip_res = copy_text(copied_resp.copied_text.clone()).await;
                                                            busy_conv.set(None);
                                                            draft_edits.write().remove(&edit_key);
                                                            if clip_res.is_ok() {
                                                                copied_notices.write().insert(
                                                                    cid.clone(),
                                                                    "Copied reply to clipboard.".to_owned(),
                                                                );
                                                            } else {
                                                                copied_notices.write().insert(
                                                                    cid.clone(),
                                                                    "Recorded copy; browser clipboard access was unavailable.".to_owned(),
                                                                );
                                                            }
                                                            apply(copied_resp.surface);
                                                        }
                                                        Err(err) => {
                                                            busy_conv.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                card_errors.write().insert(
                                                                    cid,
                                                                    not_saved_message(&err, "Coda couldn’t record copying that reply."),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_no_reply_needed: {
                                            let ws = workspace_id.clone();
                                            let cid = cid.clone();
                                            let edit_key = edit_key.clone();
                                            let apply = apply_updated_surface.clone();
                                            move |expected_proposal_version: u64| {
                                                if busy_conv.peek().is_some() {
                                                    return;
                                                }
                                                busy_conv.set(Some(cid.clone()));
                                                card_errors.write().remove(&cid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let cid = cid.clone();
                                                let edit_key = edit_key.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    match api
                                                        .close_reply_no_reply_needed(
                                                            &ws,
                                                            &cid,
                                                            expected_proposal_version,
                                                            &key,
                                                        )
                                                        .await
                                                    {
                                                        Ok(updated) => {
                                                            busy_conv.set(None);
                                                            draft_edits.write().remove(&edit_key);
                                                            apply(updated);
                                                        }
                                                        Err(err) => {
                                                            busy_conv.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                card_errors.write().insert(
                                                                    cid,
                                                                    not_saved_message(&err, "Coda couldn’t close that reply."),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_disconnected_done: {
                                            let ws = workspace_id.clone();
                                            let cid = cid.clone();
                                            let apply = apply_updated_surface.clone();
                                            move |(expected_reply_id, standing): (u64, DisconnectedCardStandingWire)| {
                                                if busy_conv.peek().is_some() {
                                                    return;
                                                }
                                                busy_conv.set(Some(cid.clone()));
                                                card_errors.write().remove(&cid);
                                                let api = client.read().clone();
                                                let presented = api.session().get();
                                                let key = crate::idempotency_key();
                                                let ws = ws.clone();
                                                let cid = cid.clone();
                                                let mut apply = apply.clone();
                                                spawn(async move {
                                                    let body = DisconnectedReplyDoneBody {
                                                        expected_reply_id,
                                                        standing,
                                                        idempotency_key: key,
                                                    };
                                                    match api.mark_disconnected_reply_done(&ws, &cid, &body).await {
                                                        Ok(updated) => {
                                                            busy_conv.set(None);
                                                            apply(updated);
                                                        }
                                                        Err(err) => {
                                                            busy_conv.set(None);
                                                            if let Some(refused) = crate::refusal(&err) {
                                                                session.refused(refused, *shown.peek(), presented);
                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                on_patient_gone.call(ws);
                                                            } else {
                                                                card_errors.write().insert(
                                                                    cid,
                                                                    not_saved_message(&err, "Coda couldn’t mark that disconnected reply done."),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        on_wrong_patient: {
                                            let cid = cid.clone();
                                            move |_| {
                                                move_error.set(None);
                                                move_dialog.set(Some(cid.clone()));
                                            }
                                        },
                                        on_undo_move: run_undo_move.clone(),
                                        on_open_source,
                                        on_open_basis,
                                        on_open_patient,
                                    }
                                }
                            }
                            PatientReplyConversationWire::ClosedUndoBanner {
                                conversation_id,
                                primary_sender_name,
                                spruce_app_url,
                                closed_by_physician,
                                pending_move,
                                ..
                            } => {
                                let cid = conversation_id.clone();
                                let is_busy = busy_conv.read().as_deref() == Some(cid.as_str());
                                let err_msg = card_errors.read().get(&cid).cloned();
                                let detail_text = if closed_by_physician.trim().is_empty() {
                                    "Marked no reply needed.".to_owned()
                                } else {
                                    format!("Marked no reply needed by {closed_by_physician}.")
                                };
                                rsx! {
                                    Card {
                                        key: "{cid}",
                                        "data-closed-undo-banner": "{cid}",
                                        overline: "Spruce conversation".to_owned(),
                                        title: primary_sender_name,
                                        detail: detail_text,
                                        div { class: "mr-card-inner",
                                            if let Some(err_text) = err_msg {
                                                div { class: "mr-banner", "data-reply-command-error": "{cid}",
                                                    Notice {
                                                        tone: Tone::Error,
                                                        icon: Some(IconName::Alert),
                                                        title: err_text,
                                                    }
                                                }
                                            }
                                            if let Some(pm) = pending_move {
                                                PendingMoveNotice {
                                                    conversation_id: cid.clone(),
                                                    pending_move: pm,
                                                    busy: is_busy,
                                                    on_undo: run_undo_move.clone(),
                                                    on_open_patient,
                                                }
                                            }
                                            ActionRow {
                                                Button {
                                                    compact: true,
                                                    variant: ButtonVariant::Secondary,
                                                    busy: is_busy,
                                                    "data-undo-no-reply-needed": "{cid}",
                                                    onclick: {
                                                        let ws = workspace_id.clone();
                                                        let cid = cid.clone();
                                                        let apply = apply_updated_surface.clone();
                                                        move |_| {
                                                            if busy_conv.peek().is_some() {
                                                                return;
                                                            }
                                                            busy_conv.set(Some(cid.clone()));
                                                            card_errors.write().remove(&cid);
                                                            let api = client.read().clone();
                                                            let presented = api.session().get();
                                                            let key = crate::idempotency_key();
                                                            let ws = ws.clone();
                                                            let cid = cid.clone();
                                                            let mut apply = apply.clone();
                                                            spawn(async move {
                                                                match api.undo_close_no_reply_needed(&ws, &cid, &key).await {
                                                                    Ok(updated) => {
                                                                        busy_conv.set(None);
                                                                        apply(updated);
                                                                    }
                                                                    Err(err) => {
                                                                        busy_conv.set(None);
                                                                        if let Some(refused) = crate::refusal(&err) {
                                                                            session.refused(refused, *shown.peek(), presented);
                                                                        } else if is_patient_access_lost_on_command(&err) {
                                                                            on_patient_gone.call(ws);
                                                                        } else {
                                                                            card_errors.write().insert(
                                                                                cid,
                                                                                not_saved_message(&err, "Coda couldn’t undo closing that reply."),
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            });
                                                        }
                                                    },
                                                    Icon { name: IconName::Undo }
                                                    "Undo"
                                                }
                                                if let Some(url) = spruce_app_url.as_deref() {
                                                    a {
                                                        class: "citation-open",
                                                        href: "{url}",
                                                        target: "_blank",
                                                        rel: "noopener noreferrer",
                                                        "data-open-in-spruce": "{cid}",
                                                        "Open in Spruce"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            PatientReplyConversationWire::NoOpenCard {
                                conversation_id,
                                primary_sender_name,
                                primary_sender_role,
                                non_patient_sender_names,
                                spruce_app_url,
                                patient_messages,
                                practice_messages,
                                review_state,
                                can_ask_coda_to_reply,
                                pending_move,
                                ..
                            } => {
                                let cid = conversation_id.clone();
                                let is_busy = busy_conv.read().as_deref() == Some(cid.as_str());
                                let err_msg = card_errors.read().get(&cid).cloned();
                                let overline_text = if primary_sender_role == ChannelSenderRoleWire::NonPatient {
                                    "Spruce sender".to_owned()
                                } else {
                                    "Spruce conversation".to_owned()
                                };
                                rsx! {
                                    Card {
                                        key: "{cid}",
                                        "data-spruce-thread-card": "{cid}",
                                        overline: overline_text,
                                        title: primary_sender_name.clone(),
                                        div { class: "mr-card-inner",
                                            if !non_patient_sender_names.is_empty() {
                                                p { class: "mr-non-patient-note", "data-non-patient-sender": "{cid}",
                                                    "Channel sender: {non_patient_sender_names.join(\", \")}"
                                                }
                                            }
                                            if let Some(err_text) = err_msg {
                                                div { class: "mr-banner", "data-reply-command-error": "{cid}",
                                                    Notice {
                                                        tone: Tone::Error,
                                                        icon: Some(IconName::Alert),
                                                        title: err_text,
                                                    }
                                                }
                                            }
                                            if let Some(pm) = pending_move {
                                                PendingMoveNotice {
                                                    conversation_id: cid.clone(),
                                                    pending_move: pm,
                                                    busy: is_busy,
                                                    on_undo: run_undo_move.clone(),
                                                    on_open_patient,
                                                }
                                            }
                                            if matches!(review_state, CardReviewStateWire::Reviewing { .. }) {
                                                div { "data-coda-reviewing": "{cid}",
                                                    WorkingPresence {
                                                        compact: true,
                                                        label: "Coda is reviewing".to_owned(),
                                                    }
                                                }
                                            }
                                            MessageThreadList {
                                                patient_messages,
                                                practice_messages,
                                            }
                                            ActionRow {
                                                if can_ask_coda_to_reply {
                                                    Button {
                                                        compact: true,
                                                        variant: ButtonVariant::Secondary,
                                                        busy: is_busy,
                                                        "data-ask-coda-to-reply": "{cid}",
                                                        onclick: {
                                                            let ws = workspace_id.clone();
                                                            let cid = cid.clone();
                                                            let apply = apply_updated_surface.clone();
                                                            move |_| {
                                                                if busy_conv.peek().is_some() {
                                                                    return;
                                                                }
                                                                busy_conv.set(Some(cid.clone()));
                                                                card_errors.write().remove(&cid);
                                                                let api = client.read().clone();
                                                                let presented = api.session().get();
                                                                let key = crate::idempotency_key();
                                                                let ws = ws.clone();
                                                                let cid = cid.clone();
                                                                let mut apply = apply.clone();
                                                                spawn(async move {
                                                                    match api.ask_coda_to_reply(&ws, &cid, &key).await {
                                                                        Ok(updated) => {
                                                                            busy_conv.set(None);
                                                                            apply(updated);
                                                                        }
                                                                        Err(err) => {
                                                                            busy_conv.set(None);
                                                                            if let Some(refused) = crate::refusal(&err) {
                                                                                session.refused(refused, *shown.peek(), presented);
                                                                            } else if is_patient_access_lost_on_command(&err) {
                                                                                on_patient_gone.call(ws);
                                                                            } else {
                                                                                card_errors.write().insert(
                                                                                    cid,
                                                                                    not_saved_message(&err, "Coda couldn’t start a reply review."),
                                                                                );
                                                                            }
                                                                        }
                                                                    }
                                                                });
                                                            }
                                                        },
                                                        "Ask Coda to reply"
                                                    }
                                                }
                                                Button {
                                                    compact: true,
                                                    variant: ButtonVariant::Quiet,
                                                    disabled: is_busy,
                                                    "data-wrong-patient": "{cid}",
                                                    onclick: {
                                                        let cid = cid.clone();
                                                        move |_| {
                                                            move_error.set(None);
                                                            move_dialog.set(Some(cid.clone()));
                                                        }
                                                    },
                                                    "Wrong patient?"
                                                }
                                                if let Some(url) = spruce_app_url.as_deref() {
                                                    a {
                                                        class: "citation-open",
                                                        href: "{url}",
                                                        target: "_blank",
                                                        rel: "noopener noreferrer",
                                                        "data-open-in-spruce": "{cid}",
                                                        "Open in Spruce"
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
            }
            if let Some(target_cid) = move_dialog() {
                PatientPickerDialog {
                    client,
                    purpose: PatientPickerPurpose::MoveConversation {
                        workspace_id: workspace_id.clone(),
                        conversation_id: target_cid.clone(),
                    },
                    exclude_workspace_id: Some(workspace_id.clone()),
                    busy: busy_conv().is_some(),
                    command_error: move_error(),
                    on_pick_patient: {
                        let cid = target_cid.clone();
                        let mut run = run_move.clone();
                        move |(dest_ws, _policy): (String, SenderConfirmationPolicyWire)| {
                            run((
                                cid.clone(),
                                MoveDestinationWire::AnotherPatient {
                                    patient_workspace_id: dest_ws,
                                },
                            ));
                        }
                    },
                    on_pick_not_a_patient: Some(EventHandler::new({
                        let cid = target_cid.clone();
                        let mut run = run_move.clone();
                        move |()| {
                            run((cid.clone(), MoveDestinationWire::NotAPatient));
                        }
                    })),
                    on_close: move |_| move_dialog.set(None),
                }
            }
        }
    }
}

#[component]
fn PendingMoveNotice(
    conversation_id: String,
    pending_move: PendingSpruceMoveWire,
    busy: bool,
    on_undo: EventHandler<String>,
    on_open_patient: EventHandler<String>,
) -> Element {
    let by_clause = if pending_move.moved_by_physician.trim().is_empty() {
        String::new()
    } else {
        format!(" by {}", pending_move.moved_by_physician)
    };
    let (title_text, dest_ws) = match &pending_move.destination {
        PendingMoveTargetDisplayWire::AnotherPatient {
            patient_workspace_id,
            patient_label,
        } => (
            format!("Moved conversation to {patient_label}{by_clause}."),
            patient_workspace_id.clone(),
        ),
        PendingMoveTargetDisplayWire::NotAPatient => (
            format!("Moved conversation to Not a patient{by_clause}."),
            None,
        ),
    };
    rsx! {
        div { class: "mr-banner", "data-pending-move-banner": "{conversation_id}",
            Notice {
                tone: Tone::Warning,
                icon: Some(IconName::Undo),
                title: title_text,
                detail: Some("Undo is available briefly before Coda settles the move.".to_owned()),
                ActionRow {
                    Button {
                        compact: true,
                        busy,
                        "data-undo-move": "{conversation_id}",
                        onclick: {
                            let cid = conversation_id.clone();
                            move |_| on_undo.call(cid.clone())
                        },
                        Icon { name: IconName::Undo }
                        "Undo"
                    }
                    if let Some(ws) = dest_ws {
                        Button {
                            compact: true,
                            variant: ButtonVariant::Quiet,
                            onclick: {
                                let ws = ws.clone();
                                move |_| on_open_patient.call(ws.clone())
                            },
                            "Open destination patient"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MessageThreadList(
    patient_messages: Vec<physician_wire::matching_reply::ReplyPatientMessageWire>,
    practice_messages: Vec<physician_wire::matching_reply::ReplyPracticeMessageWire>,
) -> Element {
    if patient_messages.is_empty() && practice_messages.is_empty() {
        return rsx! {};
    }
    rsx! {
        div { class: "mr-thread-messages",
            for pm in patient_messages {
                div {
                    key: "{pm.message_id}",
                    class: "mr-thread-msg",
                    "data-reply-patient-message": "{pm.message_id}",
                    div { class: "mr-thread-msg-head",
                        strong { "{pm.sender_display_name}" }
                        if pm.sender_role == ChannelSenderRoleWire::NonPatient {
                            StatePill { tone: Tone::Neutral, "Outside sender" }
                        }
                        if pm.arrived_after_proposal {
                            StatePill { tone: Tone::Primary, "New message" }
                        }
                    }
                    p { class: "mr-message-body", "{pm.text}" }
                }
            }
            for pr in practice_messages {
                div {
                    key: "{pr.practice_message_id}",
                    class: "mr-thread-msg mr-thread-practice",
                    "data-reply-practice-message": "{pr.practice_message_id}",
                    div { class: "mr-thread-msg-head",
                        strong {
                            match pr.kind {
                                PracticeMessageKindWire::HumanReply => "Practice reply",
                                PracticeMessageKindWire::AutomatedAcknowledgement => "Automatic acknowledgement",
                            }
                        }
                    }
                    p { class: "mr-message-body", "{pr.text}" }
                }
            }
        }
    }
}

fn card_arrival_notice_words(notice: &CardArrivalNoticeWire) -> String {
    match notice {
        CardArrivalNoticeWire::NewerReplyReplacedPrevious {
            previous_version,
            current_version,
        } => format!(
            "Coda’s newer reply (v{current_version}) replaced its earlier reply (v{previous_version})."
        ),
        CardArrivalNoticeWire::OlderMessageReadLate => {
            "Includes an earlier message from this conversation that Coda read after the previous reply.".to_owned()
        }
        CardArrivalNoticeWire::PatientWroteAgain => {
            "The sender wrote again since this reply was drafted.".to_owned()
        }
    }
}

#[component]
fn OpenReplyCardView(
    workspace_id: String,
    card: ReplyCardWire,
    pinned_base: Option<ExpectedDraftBaseWire>,
    draft_text: String,
    revise_instruction: String,
    busy: bool,
    error_message: Option<String>,
    copied_notice: Option<String>,
    on_edit_text: EventHandler<String>,
    on_discard_stale_edits: EventHandler,
    on_edit_revise_instruction: EventHandler<String>,
    on_save_draft: EventHandler<(ExpectedDraftBaseWire, String)>,
    on_request_revision: EventHandler<(ExpectedDraftBaseWire, String, String)>,
    on_restore_my_version: EventHandler<u64>,
    on_choose_update: EventHandler<(u64, u64)>,
    on_copy_reply: EventHandler<(ExpectedDraftBaseWire, String, String)>,
    on_no_reply_needed: EventHandler<u64>,
    on_disconnected_done: EventHandler<(u64, DisconnectedCardStandingWire)>,
    on_wrong_patient: EventHandler,
    on_undo_move: EventHandler<String>,
    on_open_source: EventHandler<SourceChip>,
    on_open_basis: EventHandler<(String, String)>,
    on_open_patient: EventHandler<String>,
) -> Element {
    let cid = card.conversation_id.clone();
    let _ = workspace_id;
    let overline_text = if card.primary_sender_role == ChannelSenderRoleWire::NonPatient {
        "Spruce sender".to_owned()
    } else {
        "Spruce reply".to_owned()
    };
    let server_expected_base = displayed_expected_base(&card.displayed_reply);
    let action_base = pinned_base.or(server_expected_base);
    let newer_server_proposal_while_editing = match (pinned_base, &card.displayed_reply) {
        (
            Some(pinned),
            DisplayedReplyTextWire::CodaProposal {
                proposal_version,
                text,
            },
        ) if Some(pinned) != server_expected_base => Some((*proposal_version, text.clone())),
        _ => None,
    };
    let server_saved_text = displayed_reply_text(&card.displayed_reply)
        .unwrap_or_default()
        .to_owned();
    let restore_revision = match &card.displayed_reply {
        DisplayedReplyTextWire::PhysicianDraft {
            revision,
            kind: PhysicianDraftKindWire::CodaRevision { .. },
            ..
        } => Some(*revision),
        _ => None,
    };
    let is_coda_revision_displayed = restore_revision.is_some();
    let active_draft_revision = match &card.displayed_reply {
        DisplayedReplyTextWire::PhysicianDraft { revision, .. } => Some(*revision),
        _ => None,
    };
    let disconnected_standing = match &card.displayed_reply {
        DisplayedReplyTextWire::Withheld { .. } => DisconnectedCardStandingWire::Withheld,
        _ => DisconnectedCardStandingWire::Ready {
            proposal_version: card.proposal_version,
        },
    };
    let revision_running = card
        .revision_run
        .as_ref()
        .is_some_and(physician_wire::ReplyRevisionRun::running);

    let chips: Vec<SourceChip> = card
        .citations
        .iter()
        .enumerate()
        .map(|(idx, c)| item_citation_to_chip(idx + 1, c))
        .collect();

    rsx! {
        Card {
            "data-spruce-reply-card": "{cid}",
            "data-reply-sender": "{card.primary_sender_name}",
            overline: overline_text,
            title: card.primary_sender_name.clone(),
            div { class: "mr-card-inner",
                if !card.non_patient_sender_names.is_empty() {
                    p { class: "mr-non-patient-note", "data-non-patient-sender": "{cid}",
                        "Channel sender: {card.non_patient_sender_names.join(\", \")}"
                    }
                }
                for (notice_idx, notice) in card.notices.iter().enumerate() {
                    p {
                        key: "arrival-notice-{notice_idx}",
                        class: "mr-non-patient-note",
                        role: "status",
                        "data-card-arrival-notice": "{cid}",
                        "{card_arrival_notice_words(notice)}"
                    }
                }
                if let CardChannelBannerWire::Disconnected {
                    reason,
                    spruce_app_url,
                    ..
                } = &card.channel_banner
                {
                    div { class: "mr-banner", "data-disconnected-reply-banner": "{cid}",
                        Notice {
                            tone: Tone::Warning,
                            icon: Some(IconName::Alert),
                            title: reason.clone(),
                            ActionRow {
                                if let Some(url) = spruce_app_url.as_deref().or(card.spruce_app_url.as_deref()) {
                                    a {
                                        class: "citation-open",
                                        href: "{url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "data-open-in-spruce": "{cid}",
                                        "Open in Spruce"
                                    }
                                }
                                Button {
                                    compact: true,
                                    busy,
                                    "data-disconnected-reply-done": "{cid}",
                                    onclick: {
                                        let rid = card.reply_id;
                                        let st = disconnected_standing;
                                        move |_| on_disconnected_done.call((rid, st))
                                    },
                                    "Done"
                                }
                            }
                        }
                    }
                }
                if let Some(pm) = &card.pending_move {
                    PendingMoveNotice {
                        conversation_id: cid.clone(),
                        pending_move: pm.clone(),
                        busy,
                        on_undo: on_undo_move,
                        on_open_patient,
                    }
                }
                if let Some(err_text) = error_message {
                    div { class: "mr-banner", "data-reply-command-error": "{cid}",
                        Notice {
                            tone: Tone::Error,
                            icon: Some(IconName::Alert),
                            title: err_text,
                        }
                    }
                }
                if let Some(copied_text) = copied_notice {
                    div { class: "mr-banner", "data-reply-copied-notice": "{cid}",
                        Notice {
                            tone: Tone::Primary,
                            icon: Some(IconName::Check),
                            title: copied_text,
                        }
                    }
                }
                if matches!(card.review_state, CardReviewStateWire::Reviewing { .. }) {
                    div { "data-coda-reviewing": "{cid}",
                        WorkingPresence {
                            compact: true,
                            label: "Coda is reviewing".to_owned(),
                        }
                    }
                }
                if !card.other_physician_copies.is_empty() {
                    div { class: "mr-copy-attributions", "data-other-physician-copy": "{cid}",
                        for attr in &card.other_physician_copies {
                            {
                                let zone = crate::schedule_briefing::browser_time_zone("America/New_York");
                                let when = crate::shell::format_local_date(&attr.copied_at, &zone)
                                    .zip(crate::shell::format_clock(&attr.copied_at, &zone))
                                    .map(|(date, clock)| format!("{date}, {clock}"))
                                    .unwrap_or_else(|| attr.copied_at.clone());
                                rsx! {
                                    p { class: "mr-copy-attribution-line",
                                        "Copied by {attr.copied_by_physician} · {when}"
                                    }
                                }
                            }
                        }
                    }
                }
                MessageThreadList {
                    patient_messages: card.patient_messages.clone(),
                    practice_messages: card.practice_messages.clone(),
                }
                if let Some(offer) = &card.coda_update_offer {
                    div { class: "mr-banner", "data-coda-update-offer": "{cid}",
                        Notice {
                            tone: Tone::Primary,
                            icon: Some(IconName::Sparkle),
                            title: format!("Coda drafted an updated reply (v{}).", offer.offered_proposal_version),
                            detail: Some(offer.text.clone()),
                            ActionRow {
                                if let Some(draft_rev) = active_draft_revision {
                                    Button {
                                        compact: true,
                                        variant: ButtonVariant::Primary,
                                        busy,
                                        "data-choose-coda-update": "{cid}",
                                        onclick: {
                                            let target_ver = offer.offered_proposal_version;
                                            move |_| on_choose_update.call((draft_rev, target_ver))
                                        },
                                        "Choose update"
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some((newer_ver, newer_text)) = &newer_server_proposal_while_editing {
                    div { class: "mr-banner", "data-newer-proposal-while-editing": "{cid}",
                        Notice {
                            tone: Tone::Primary,
                            icon: Some(IconName::Sparkle),
                            title: format!("Coda drafted a newer reply (v{newer_ver}) while you were editing."),
                            detail: Some(newer_text.clone()),
                            ActionRow {
                                Button {
                                    compact: true,
                                    variant: ButtonVariant::Secondary,
                                    disabled: busy,
                                    "data-use-newer-proposal": "{cid}",
                                    onclick: move |_| on_discard_stale_edits.call(()),
                                    "Use newer reply"
                                }
                            }
                        }
                    }
                }
                match &card.displayed_reply {
                    DisplayedReplyTextWire::Withheld { reason, .. } => rsx! {
                        div { class: "mr-banner", "data-reply-withheld": "{cid}",
                            Notice {
                                tone: Tone::Warning,
                                icon: Some(IconName::Alert),
                                title: "Reply withheld".to_owned(),
                                detail: (!reason.trim().is_empty()).then(|| reason.clone()),
                            }
                        }
                    },
                    DisplayedReplyTextWire::CodaProposal { proposal_version, .. } => rsx! {
                        div { class: "mr-editor-box", "data-reply-editor": "{cid}",
                            div { class: "mr-editor-head",
                                StatePill { tone: Tone::Primary, icon: Some(IconName::Reply), "Coda proposal v{proposal_version}" }
                            }
                            TextField {
                                label: "Reply draft".to_owned(),
                                value: draft_text.clone(),
                                kind: TextKind::Multiline,
                                disabled: busy || revision_running,
                                on_input: move |val| on_edit_text.call(val),
                            }
                        }
                    },
                    DisplayedReplyTextWire::PhysicianDraft { revision, kind, .. } => rsx! {
                        div { class: "mr-editor-box", "data-reply-editor": "{cid}",
                            div { class: "mr-editor-head",
                                StatePill {
                                    tone: Tone::Primary,
                                    icon: Some(IconName::Pencil),
                                    match kind {
                                        PhysicianDraftKindWire::YourText => format!("Your draft (r{revision})"),
                                        PhysicianDraftKindWire::CodaRevision { .. } => format!("Coda’s revision (r{revision})"),
                                    }
                                }
                            }
                            TextField {
                                label: "Reply draft".to_owned(),
                                value: draft_text.clone(),
                                kind: TextKind::Multiline,
                                disabled: busy || revision_running,
                                on_input: move |val| on_edit_text.call(val),
                            }
                        }
                    },
                }

                if !card.basis_and_limitations.is_empty() || !chips.is_empty() {
                    div { class: "mr-evidence-bar", "data-reply-evidence": "{cid}",
                        if !card.basis_and_limitations.is_empty() {
                            Button {
                                compact: true,
                                variant: ButtonVariant::Quiet,
                                "data-open-reply-basis": "{cid}",
                                onclick: {
                                    let sender = card.primary_sender_name.clone();
                                    let basis = card.basis_and_limitations.clone();
                                    move |_| {
                                        on_open_basis.call((
                                            format!("Basis · Reply to {sender}"),
                                            basis.clone(),
                                        ));
                                    }
                                },
                                Icon { name: IconName::Book }
                                "Basis"
                            }
                        }
                        if !chips.is_empty() {
                            div { class: "mr-sources-row", "data-reply-sources": "{cid}",
                                span { class: "mr-sources-label", "Sources:" }
                                for chip in chips {
                                    crate::CitationChip {
                                        key: "{chip.index}",
                                        chip: chip.clone(),
                                        on_open: on_open_source,
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(base) = action_base {
                    div { class: "mr-revise-box",
                        if let Some(run) = &card.revision_run {
                            if run.status != "completed" || is_coda_revision_displayed {
                                if let Some(words) = revision_run_words(run) {
                                    p { class: "mr-revision-status", role: "status", "data-revision-status": "{run.status}",
                                        "{words}"
                                    }
                                }
                            }
                        }
                        TextField {
                            label: "Tell Coda what to change".to_owned(),
                            placeholder: "Make it shorter, mention recent lab values…".to_owned(),
                            value: revise_instruction.clone(),
                            disabled: busy || revision_running,
                            on_input: move |val| on_edit_revise_instruction.call(val),
                        }
                        ActionRow {
                            Button {
                                compact: true,
                                variant: ButtonVariant::Primary,
                                busy,
                                disabled: draft_text.trim().is_empty(),
                                "data-copy-reply": "{cid}",
                                onclick: {
                                    let b = base;
                                    let txt = draft_text.clone();
                                    let srv = server_saved_text.clone();
                                    move |_| on_copy_reply.call((b, txt.clone(), srv.clone()))
                                },
                                Icon { name: IconName::Copy }
                                "Copy reply"
                            }
                            Button {
                                compact: true,
                                variant: ButtonVariant::Secondary,
                                busy,
                                disabled: draft_text.trim().is_empty() || revision_running,
                                "data-save-reply-draft": "{cid}",
                                onclick: {
                                    let b = base;
                                    let txt = draft_text.clone();
                                    move |_| on_save_draft.call((b, txt.clone()))
                                },
                                "Save draft"
                            }
                            Button {
                                compact: true,
                                variant: ButtonVariant::Secondary,
                                busy: revision_running,
                                disabled: busy || revise_instruction.trim().is_empty() || draft_text.trim().is_empty(),
                                "data-ask-coda-revise": "{cid}",
                                onclick: {
                                    let b = base;
                                    let txt = draft_text.clone();
                                    let inst = revise_instruction.clone();
                                    move |_| on_request_revision.call((b, txt.clone(), inst.clone()))
                                },
                                "Ask Coda to revise"
                            }
                            if let Some(rev) = restore_revision {
                                Button {
                                    compact: true,
                                    variant: ButtonVariant::Secondary,
                                    disabled: busy || revision_running,
                                    "data-restore-my-version": "{cid}",
                                    onclick: move |_| on_restore_my_version.call(rev),
                                    Icon { name: IconName::Undo }
                                    "Restore my version"
                                }
                            }
                            Button {
                                compact: true,
                                variant: ButtonVariant::Quiet,
                                disabled: busy,
                                "data-no-reply-needed": "{cid}",
                                onclick: {
                                    let prop_ver = card.proposal_version;
                                    move |_| on_no_reply_needed.call(prop_ver)
                                },
                                "No reply needed"
                            }
                            Button {
                                compact: true,
                                variant: ButtonVariant::Quiet,
                                disabled: busy,
                                "data-wrong-patient": "{cid}",
                                onclick: move |_| on_wrong_patient.call(()),
                                "Wrong patient?"
                            }
                            if let Some(url) = card.spruce_app_url.as_deref() {
                                a {
                                    class: "citation-open",
                                    href: "{url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "data-open-in-spruce": "{cid}",
                                    "Open in Spruce"
                                }
                            }
                        }
                    }
                } else {
                    ActionRow {
                        Button {
                            compact: true,
                            variant: ButtonVariant::Quiet,
                            disabled: busy,
                            "data-wrong-patient": "{cid}",
                            onclick: move |_| on_wrong_patient.call(()),
                            "Wrong patient?"
                        }
                        if let Some(url) = card.spruce_app_url.as_deref() {
                            a {
                                class: "citation-open",
                                href: "{url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "data-open-in-spruce": "{cid}",
                                "Open in Spruce"
                            }
                        }
                    }
                }
            }
        }
    }
}
