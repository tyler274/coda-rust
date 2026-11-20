//! Patient and colleague invitation workflows for any active practice member (`A1.5`).

use dioxus::prelude::*;
use physician_client::{HttpError, PhysicianClient};
use physician_wire::{InviteColleagueRequest, InvitePatientRequest};
use ui::{ActionRow, Button, ButtonVariant, Dialog, Notice, TextField, TextKind, Tone};

use physician_wire::{ConnectionStatusWire, PatientLinkWire};

fn invite_error_message(error: &HttpError, fallback: &str) -> String {
    match error {
        HttpError::Status { status: 404, .. } => "This patient is not in your practice.".to_owned(),
        HttpError::Status { status: 409, body } => {
            let trimmed = body.trim();
            if trimmed.is_empty() {
                "An active invitation or link already exists for this email.".to_owned()
            } else {
                trimmed.to_owned()
            }
        }
        HttpError::Status { status: 423, .. } => {
            "This invitation is locked after repeated wrong birth-date answers. Cancel it first."
                .to_owned()
        }
        HttpError::Status { status: 422, .. } => "Enter a valid email address and name.".to_owned(),
        HttpError::Status { status: 403, .. } => {
            "This account is not permitted to send invitations.".to_owned()
        }
        HttpError::Status { .. } | HttpError::Transport(_) => fallback.to_owned(),
    }
}

/// Patient-page action allowing any active practice member to invite, resend, or
/// cancel a patient's Duet connection on their practice roster.
#[component]
pub(crate) fn PatientInvitePanel(
    client: Signal<PhysicianClient>,
    session: crate::SessionState,
    shown: Signal<bool>,
    workspace_id: String,
    default_display_name: String,
    default_email: String,
    patient_link: Option<PatientLinkWire>,
    on_updated: EventHandler<()>,
) -> Element {
    let mut open = use_signal(|| false);
    let mut email = use_signal(String::new);
    let mut display_name = use_signal(|| default_display_name.clone());
    let mut busy = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut confirmation = use_signal(|| None::<String>);

    if client.read().is_demo() || workspace_id.trim().is_empty() {
        return rsx! {};
    }

    let ws_for_submit = workspace_id.clone();
    let ws_for_resend = workspace_id.clone();
    let ws_for_cancel = workspace_id.clone();
    let ws_for_attr = workspace_id.clone();
    let seed_name = default_display_name.clone();
    let seed_email = default_email.clone();

    let status = patient_link
        .as_ref()
        .map(|link| link.connection_status)
        .unwrap_or(ConnectionStatusWire::NotInvited);
    let invited_email = patient_link
        .as_ref()
        .and_then(|link| link.invited_email.clone())
        .unwrap_or_default();
    let is_locked = patient_link
        .as_ref()
        .is_some_and(|link| link.locked_at.is_some());
    let delivery_failed = patient_link
        .as_ref()
        .is_some_and(|link| link.delivery_failed_at.is_some());
    let delivery_error = patient_link
        .as_ref()
        .and_then(|link| link.delivery_error.clone());

    let trigger_label = match status {
        ConnectionStatusWire::NotInvited => "Invite patient",
        ConnectionStatusWire::Invited => {
            if is_locked {
                "Invitation locked"
            } else if delivery_failed {
                "Mail failed"
            } else {
                "Invitation sent"
            }
        }
        ConnectionStatusWire::Connected => "Duet connected",
        ConnectionStatusWire::Ended => "Re-invite patient",
    };

    let submit_invite = move |_| {
        if busy() {
            return;
        }
        let target_email = email().trim().to_owned();
        if target_email.is_empty() {
            error.set(Some("Enter the patient's email address.".to_owned()));
            return;
        }
        let name = display_name().trim().to_owned();
        let req = InvitePatientRequest {
            patient_workspace_id: ws_for_submit.clone(),
            email: target_email.clone(),
            display_name: if name.is_empty() {
                ws_for_submit.clone()
            } else {
                name
            },
        };
        busy.set(true);
        error.set(None);
        confirmation.set(None);
        let api = client.read().clone();
        let presented = api.session().get();
        spawn(async move {
            match api.invite_patient(&req).await {
                Ok(saved) => {
                    busy.set(false);
                    confirmation.set(Some(format!(
                        "Invited {} ({}) to connect via Duet.",
                        saved.display_name, saved.email
                    )));
                    on_updated.call(());
                }
                Err(err) => {
                    busy.set(false);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else {
                        error.set(Some(invite_error_message(
                            &err,
                            "Coda couldn’t send that patient invitation.",
                        )));
                        on_updated.call(());
                    }
                }
            }
        });
    };

    let resend_invite = move |_| {
        if busy() || is_locked {
            return;
        }
        let ws = ws_for_resend.clone();
        busy.set(true);
        error.set(None);
        confirmation.set(None);
        let api = client.read().clone();
        let presented = api.session().get();
        spawn(async move {
            match api.resend_patient_invitation(&ws).await {
                Ok(saved) => {
                    busy.set(false);
                    confirmation.set(Some(format!(
                        "Sent a fresh Duet link and code to {}.",
                        saved.email
                    )));
                    on_updated.call(());
                }
                Err(err) => {
                    busy.set(false);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else {
                        error.set(Some(invite_error_message(
                            &err,
                            "Coda couldn’t resend that invitation.",
                        )));
                        on_updated.call(());
                    }
                }
            }
        });
    };

    let cancel_invite = move |_| {
        if busy() {
            return;
        }
        let ws = ws_for_cancel.clone();
        let reset_email = seed_email.clone();
        busy.set(true);
        error.set(None);
        confirmation.set(None);
        let api = client.read().clone();
        let presented = api.session().get();
        spawn(async move {
            match api.cancel_patient_invitation(&ws).await {
                Ok(_) => {
                    busy.set(false);
                    confirmation.set(Some("Invitation cancelled.".to_owned()));
                    email.set(reset_email);
                    on_updated.call(());
                }
                Err(err) => {
                    busy.set(false);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else {
                        error.set(Some(invite_error_message(
                            &err,
                            "Coda couldn’t cancel that invitation.",
                        )));
                    }
                }
            }
        });
    };

    let seed_email_for_open = default_email.clone();

    rsx! {
        Button {
            compact: true,
            class: "invite-patient-button",
            "data-invite-patient": "{ws_for_attr}",
            onclick: move |_| {
                display_name.set(seed_name.clone());
                email.set(seed_email_for_open.clone());
                error.set(None);
                confirmation.set(None);
                open.set(true);
            },
            "{trigger_label}"
        }
        if open() {
            Dialog {
                close_label: "Close",
                title: "Duet connection",
                on_close: move |_| open.set(false),
                div { class: "invite-dialog-body", "data-invite-patient-dialog": "true",
                    if status == ConnectionStatusWire::Connected {
                        Notice {
                            tone: Tone::Primary,
                            title: "Connected in Duet".to_owned(),
                            detail: Some("This patient shares Duet records with your practice.".to_owned()),
                        }
                        ActionRow {
                            Button {
                                variant: ButtonVariant::Quiet,
                                onclick: move |_| open.set(false),
                                "Done"
                            }
                        }
                    } else if status == ConnectionStatusWire::Invited {
                        p { class: "invite-dialog-copy",
                            "Open invitation sent to {invited_email}. Earlier invitation links and codes remain valid until accepted or cancelled."
                        }
                        if is_locked {
                            Notice {
                                tone: Tone::Error,
                                title: "Locked after 5 wrong birth dates".to_owned(),
                                detail: Some("Cancel this invitation before sending a new one.".to_owned()),
                            }
                        } else if delivery_failed {
                            Notice {
                                tone: Tone::Error,
                                title: "Invitation email failed to send".to_owned(),
                                detail: delivery_error.clone(),
                            }
                        }
                        if let Some(message) = error() {
                            Notice {
                                tone: Tone::Error,
                                title: message,
                            }
                        }
                        if let Some(message) = confirmation() {
                            Notice {
                                tone: Tone::Primary,
                                title: message,
                            }
                        }
                        ActionRow {
                            Button {
                                variant: ButtonVariant::Primary,
                                disabled: busy() || is_locked,
                                onclick: resend_invite,
                                if busy() { "Working…" } else { "Resend invitation" }
                            }
                            Button {
                                variant: ButtonVariant::Quiet,
                                disabled: busy(),
                                onclick: cancel_invite,
                                "Cancel invitation"
                            }
                            Button {
                                variant: ButtonVariant::Quiet,
                                disabled: busy(),
                                onclick: move |_| open.set(false),
                                "Done"
                            }
                        }
                    } else {
                        p { class: "invite-dialog-copy",
                            if status == ConnectionStatusWire::Ended {
                                "This connection previously ended. Send a fresh Duet link and code linked to this patient's record."
                            } else {
                                "Send a Duet invitation link and code linked to this patient's record."
                            }
                        }
                        TextField {
                            label: "Patient name".to_owned(),
                            value: display_name(),
                            placeholder: "Patient name".to_owned(),
                            disabled: busy(),
                            on_input: move |value: String| display_name.set(value),
                        }
                        TextField {
                            label: "Patient email".to_owned(),
                            value: email(),
                            kind: TextKind::Email,
                            placeholder: "patient@example.com".to_owned(),
                            disabled: busy(),
                            on_input: move |value: String| email.set(value),
                        }
                        if let Some(message) = error() {
                            Notice {
                                tone: Tone::Error,
                                title: message,
                            }
                        }
                        if let Some(message) = confirmation() {
                            Notice {
                                tone: Tone::Primary,
                                title: message,
                            }
                        }
                        ActionRow {
                            Button {
                                variant: ButtonVariant::Primary,
                                disabled: busy(),
                                onclick: submit_invite,
                                if busy() { "Sending…" } else { "Send invitation" }
                            }
                            Button {
                                variant: ButtonVariant::Quiet,
                                disabled: busy(),
                                onclick: move |_| open.set(false),
                                "Done"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Sidebar footer trigger letting any active practice physician invite a colleague
/// physician into their practice.
#[component]
pub(crate) fn ColleagueInviteTrigger(
    client: Signal<PhysicianClient>,
    session: crate::SessionState,
    shown: Signal<bool>,
) -> Element {
    let mut open = use_signal(|| false);
    let mut email = use_signal(String::new);
    let mut display_name = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut confirmation = use_signal(|| None::<String>);

    if client.read().is_demo() {
        return rsx! {};
    }

    let submit = move |_| {
        if busy() {
            return;
        }
        let target_email = email().trim().to_owned();
        if target_email.is_empty() {
            error.set(Some("Enter your colleague's email address.".to_owned()));
            return;
        }
        let name = display_name().trim().to_owned();
        if name.is_empty() {
            error.set(Some("Enter your colleague's name.".to_owned()));
            return;
        }
        let req = InviteColleagueRequest {
            email: target_email,
            display_name: name,
        };
        busy.set(true);
        error.set(None);
        confirmation.set(None);
        let api = client.read().clone();
        let presented = api.session().get();
        spawn(async move {
            match api.invite_colleague(&req).await {
                Ok(saved) => {
                    busy.set(false);
                    email.set(String::new());
                    display_name.set(String::new());
                    confirmation.set(Some(format!(
                        "Invited {} ({}) to join your practice.",
                        saved.display_name, saved.email
                    )));
                }
                Err(err) => {
                    busy.set(false);
                    if let Some(refused) = crate::refusal(&err) {
                        session.refused(refused, *shown.peek(), presented);
                    } else {
                        error.set(Some(invite_error_message(
                            &err,
                            "Coda couldn’t send that colleague invitation.",
                        )));
                    }
                }
            }
        });
    };

    rsx! {
        Button {
            variant: ButtonVariant::Quiet,
            aria_label: "Invite colleague",
            onclick: move |_| {
                error.set(None);
                confirmation.set(None);
                open.set(true);
            },
            "Invite colleague"
        }
        if open() {
            Dialog {
                close_label: "Close",
                title: "Invite colleague",
                on_close: move |_| open.set(false),
                div { class: "invite-dialog-body", "data-invite-colleague-dialog": "true",
                    p { class: "invite-dialog-copy",
                        "Invite another clinician in your practice to sign in to Coda."
                    }
                    TextField {
                        label: "Colleague name".to_owned(),
                        value: display_name(),
                        placeholder: "Dr. Morgan Patel".to_owned(),
                        disabled: busy(),
                        on_input: move |value: String| display_name.set(value),
                    }
                    TextField {
                        label: "Work email".to_owned(),
                        value: email(),
                        kind: TextKind::Email,
                        placeholder: "colleague@practice.org".to_owned(),
                        disabled: busy(),
                        on_input: move |value: String| email.set(value),
                    }
                    if let Some(message) = error() {
                        Notice {
                            tone: Tone::Error,
                            title: message,
                        }
                    }
                    if let Some(message) = confirmation() {
                        Notice {
                            tone: Tone::Primary,
                            title: message,
                        }
                    }
                    ActionRow {
                        Button {
                            variant: ButtonVariant::Primary,
                            disabled: busy(),
                            onclick: submit,
                            if busy() { "Sending…" } else { "Send invitation" }
                        }
                        Button {
                            variant: ButtonVariant::Quiet,
                            disabled: busy(),
                            onclick: move |_| open.set(false),
                            "Done"
                        }
                    }
                }
            }
        }
    }
}
