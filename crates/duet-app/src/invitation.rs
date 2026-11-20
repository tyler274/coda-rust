//! Duet Connected Care invitation code entry, DOB gate, and explicit one-tap `Share` UI.

use dioxus::prelude::*;
use duet_client::connected_care::{
    ConnectedCareError, InvitationCredential, InvitationOpenStanding, InvitationSharingPreview,
};
use ui::{ActionRow, Button, ButtonVariant, Notice, TextField, Tone};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum InviteFlowState {
    Idle,
    EnterCode {
        code: String,
        error: Option<String>,
    },
    Checking {
        credential: InvitationCredential,
    },
    EnterDob {
        credential: InvitationCredential,
        birth_date: String,
        error: Option<String>,
        submitting: bool,
    },
    NotOpen,
    Locked,
    Underage,
    Preview {
        credential: InvitationCredential,
        birth_date: String,
        preview: InvitationSharingPreview,
        submitting: bool,
        error: Option<String>,
    },
    Connected {
        practice_name: String,
    },
}

pub(crate) fn initial_state(pending_invite: Option<&str>) -> InviteFlowState {
    match pending_invite.map(str::trim).filter(|s| !s.is_empty()) {
        Some(raw) => InviteFlowState::Checking {
            credential: InvitationCredential::from_input(raw),
        },
        None => InviteFlowState::Idle,
    }
}

pub(crate) fn transition_after_inspect(
    credential: InvitationCredential,
    result: Result<duet_client::connected_care::InvitationGateState, ConnectedCareError>,
) -> InviteFlowState {
    match result {
        Ok(gate) => match gate.standing {
            InvitationOpenStanding::Active { .. } => InviteFlowState::EnterDob {
                credential,
                birth_date: String::new(),
                error: None,
                submitting: false,
            },
            InvitationOpenStanding::Locked { .. } => InviteFlowState::Locked,
        },
        Err(ConnectedCareError::NotOpen) => InviteFlowState::NotOpen,
        Err(ConnectedCareError::Locked) => InviteFlowState::Locked,
        Err(err) => InviteFlowState::EnterCode {
            code: match credential {
                InvitationCredential::ShortCode(c) => c,
                InvitationCredential::LinkToken(_) => String::new(),
            },
            error: Some(err.to_string()),
        },
    }
}

pub(crate) fn transition_after_verify_dob(
    credential: InvitationCredential,
    birth_date: String,
    result: Result<InvitationSharingPreview, ConnectedCareError>,
) -> InviteFlowState {
    match result {
        Ok(preview) => InviteFlowState::Preview {
            credential,
            birth_date,
            preview,
            submitting: false,
            error: None,
        },
        Err(ConnectedCareError::NotOpen) => InviteFlowState::NotOpen,
        Err(ConnectedCareError::Locked) => InviteFlowState::Locked,
        Err(ConnectedCareError::Underage) => InviteFlowState::Underage,
        Err(err) => InviteFlowState::EnterDob {
            credential,
            birth_date,
            error: Some(err.to_string()),
            submitting: false,
        },
    }
}

pub(crate) fn transition_after_share(
    credential: InvitationCredential,
    birth_date: String,
    preview: InvitationSharingPreview,
    result: Result<duet_client::connected_care::ConnectedCareShareResult, ConnectedCareError>,
) -> InviteFlowState {
    match result {
        Ok(_) => InviteFlowState::Connected {
            practice_name: preview.practice_name,
        },
        Err(ConnectedCareError::NotOpen) => InviteFlowState::NotOpen,
        Err(ConnectedCareError::Locked) => InviteFlowState::Locked,
        Err(ConnectedCareError::Underage) => InviteFlowState::Underage,
        Err(err) => InviteFlowState::Preview {
            credential,
            birth_date,
            preview,
            submitting: false,
            error: Some(err.to_string()),
        },
    }
}

#[component]
pub(crate) fn InvitePanel(
    base_url: String,
    bearer: String,
    pending_invite: Option<String>,
    on_clear_invite: EventHandler<()>,
) -> Element {
    let mut state = use_signal(|| initial_state(pending_invite.as_deref()));
    let mut checked_key = use_signal(|| None::<String>);

    let incoming_clean = pending_invite
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned);
    if let Some(raw) = incoming_clean.clone()
        && matches!(state(), InviteFlowState::Idle)
    {
        state.set(InviteFlowState::Checking {
            credential: InvitationCredential::from_input(&raw),
        });
    }

    let current = state();
    if let InviteFlowState::Checking { credential } = &current {
        let key = format!("{credential:?}");
        if checked_key().as_deref() != Some(key.as_str()) {
            checked_key.set(Some(key));
            let cred = credential.clone();
            let base_url = base_url.clone();
            let bearer = bearer.clone();
            spawn(async move {
                let res = match crate::open_client(base_url, bearer) {
                    Ok(client) => client.inspect_connected_care_invitation(&cred).await,
                    Err(_) => Err(ConnectedCareError::Unavailable),
                };
                state.set(transition_after_inspect(cred, res));
            });
        }
    }

    let dismiss = move |_| {
        checked_key.set(None);
        state.set(InviteFlowState::Idle);
        on_clear_invite.call(());
    };

    match current {
        InviteFlowState::Idle => rsx! {
            ActionRow {
                Button {
                    variant: ButtonVariant::Quiet,
                    compact: true,
                    onclick: move |_| {
                        state.set(InviteFlowState::EnterCode {
                            code: String::new(),
                            error: None,
                        });
                    },
                    "Enter invite code"
                }
            }
        },
        InviteFlowState::EnterCode { code, error } => {
            let trimmed = code.trim().to_owned();
            rsx! {
                div { class: "duet-card duet-invite-card", "data-invite-flow": "enter-code",
                    Notice {
                        tone: Tone::Primary,
                        title: "Connect to your doctor".to_owned(),
                        detail: Some("Enter the code from your practice invitation email.".to_owned()),
                    }
                    TextField {
                        label: "Invitation code".to_owned(),
                        value: code.clone(),
                        placeholder: "XXXX-XXXX".to_owned(),
                        on_input: move |value: String| {
                            state.set(InviteFlowState::EnterCode {
                                code: value,
                                error: None,
                            });
                        },
                    }
                    if let Some(message) = error {
                        Notice { tone: Tone::Error, title: message }
                    }
                    ActionRow {
                        Button {
                            variant: ButtonVariant::Primary,
                            compact: true,
                            disabled: trimmed.is_empty(),
                            onclick: move |_| {
                                let clean = code.trim();
                                if clean.is_empty() {
                                    state.set(InviteFlowState::EnterCode {
                                        code: String::new(),
                                        error: Some("Enter your invitation code.".to_owned()),
                                    });
                                    return;
                                }
                                checked_key.set(None);
                                state.set(InviteFlowState::Checking {
                                    credential: InvitationCredential::from_input(clean),
                                });
                            },
                            "Continue"
                        }
                        Button { variant: ButtonVariant::Quiet, compact: true, onclick: dismiss, "Cancel" }
                    }
                }
            }
        }
        InviteFlowState::Checking { .. } => rsx! {
            Notice {
                tone: Tone::Primary,
                title: "Checking invitation…".to_owned(),
                ActionRow {
                    Button { variant: ButtonVariant::Quiet, compact: true, onclick: dismiss, "Cancel" }
                }
            }
        },
        InviteFlowState::EnterDob {
            credential,
            birth_date,
            error,
            submitting,
        } => {
            let dob_ready = !birth_date.trim().is_empty() && !submitting;
            let cred_for_input = credential.clone();
            let cred_for_submit = credential.clone();
            let dob_for_submit = birth_date.clone();
            let base_for_submit = base_url.clone();
            let bearer_for_submit = bearer.clone();
            rsx! {
                div { class: "duet-card duet-invite-card", "data-invite-flow": "enter-dob",
                    Notice {
                        tone: Tone::Primary,
                        title: "Confirm your date of birth".to_owned(),
                        detail: Some("Enter your date of birth before seeing invitation details.".to_owned()),
                    }
                    TextField {
                        label: "Date of birth".to_owned(),
                        value: birth_date.clone(),
                        placeholder: "YYYY-MM-DD".to_owned(),
                        disabled: submitting,
                        on_input: move |value: String| {
                            state.set(InviteFlowState::EnterDob {
                                credential: cred_for_input.clone(),
                                birth_date: value,
                                error: None,
                                submitting: false,
                            });
                        },
                    }
                    if let Some(message) = error {
                        Notice { tone: Tone::Error, title: message }
                    }
                    ActionRow {
                        Button {
                            variant: ButtonVariant::Primary,
                            compact: true,
                            disabled: !dob_ready,
                            onclick: move |_| {
                                let dob = dob_for_submit.trim().to_owned();
                                let cred = cred_for_submit.clone();
                                state.set(InviteFlowState::EnterDob {
                                    credential: cred.clone(),
                                    birth_date: dob.clone(),
                                    error: None,
                                    submitting: true,
                                });
                                let base = base_for_submit.clone();
                                let tok = bearer_for_submit.clone();
                                spawn(async move {
                                    let res = match crate::open_client(base, tok) {
                                        Ok(client) => client.verify_connected_care_dob(&cred, &dob).await,
                                        Err(_) => Err(ConnectedCareError::Unavailable),
                                    };
                                    state.set(transition_after_verify_dob(cred, dob, res));
                                });
                            },
                            if submitting { "Checking…" } else { "Continue" }
                        }
                        Button { variant: ButtonVariant::Quiet, compact: true, disabled: submitting, onclick: dismiss, "Not now" }
                    }
                }
            }
        }
        InviteFlowState::NotOpen => rsx! {
            Notice {
                tone: Tone::Neutral,
                title: "Invitation not open".to_owned(),
                detail: Some("This invitation link or code is not open.".to_owned()),
                ActionRow {
                    Button { variant: ButtonVariant::Quiet, compact: true, onclick: dismiss, "Done" }
                }
            }
        },
        InviteFlowState::Locked => rsx! {
            Notice {
                tone: Tone::Error,
                title: "Invitation locked".to_owned(),
                detail: Some("Too many wrong birth-date answers locked this invitation. Ask your practice to cancel and send a new invitation.".to_owned()),
                ActionRow {
                    Button { variant: ButtonVariant::Quiet, compact: true, onclick: dismiss, "Done" }
                }
            }
        },
        InviteFlowState::Underage => rsx! {
            Notice {
                tone: Tone::Error,
                title: "18 or older required".to_owned(),
                detail: Some("Duet Connected Care requires being 18 or older.".to_owned()),
                ActionRow {
                    Button { variant: ButtonVariant::Quiet, compact: true, onclick: dismiss, "Done" }
                }
            }
        },
        InviteFlowState::Preview {
            credential,
            birth_date,
            preview,
            submitting,
            error,
        } => {
            let cred_for_share = credential.clone();
            let dob_for_share = birth_date.clone();
            let preview_for_share = preview.clone();
            let base_for_share = base_url.clone();
            let bearer_for_share = bearer.clone();
            rsx! {
                div { class: "duet-card duet-invite-card", "data-invite-flow": "preview",
                    Notice {
                        tone: Tone::Primary,
                        title: if preview.physician_name.trim().is_empty() {
                            preview.practice_name.clone()
                        } else {
                            format!("{} ({})", preview.practice_name, preview.physician_name)
                        },
                        detail: Some(preview.shares_summary.clone()),
                    }
                    if let Some(message) = error {
                        Notice { tone: Tone::Error, title: message }
                    }
                    ActionRow {
                        Button {
                            variant: ButtonVariant::Primary,
                            compact: true,
                            disabled: submitting,
                            onclick: move |_| {
                                let cred = cred_for_share.clone();
                                let dob = dob_for_share.clone();
                                let prev = preview_for_share.clone();
                                state.set(InviteFlowState::Preview {
                                    credential: cred.clone(),
                                    birth_date: dob.clone(),
                                    preview: prev.clone(),
                                    submitting: true,
                                    error: None,
                                });
                                let base = base_for_share.clone();
                                let tok = bearer_for_share.clone();
                                spawn(async move {
                                    let res = match crate::open_client(base, tok) {
                                        Ok(client) => client.accept_connected_care_share(&cred, &dob).await,
                                        Err(_) => Err(ConnectedCareError::Unavailable),
                                    };
                                    state.set(transition_after_share(cred, dob, prev, res));
                                });
                            },
                            if submitting { "Sharing…" } else { "Share" }
                        }
                        Button { variant: ButtonVariant::Quiet, compact: true, disabled: submitting, onclick: dismiss, "Not now" }
                    }
                }
            }
        }
        InviteFlowState::Connected { practice_name } => rsx! {
            Notice {
                tone: Tone::Primary,
                title: format!("Connected to {practice_name}"),
                detail: Some("Your signed-in Duet account is now sharing with this practice.".to_owned()),
                ActionRow {
                    Button { variant: ButtonVariant::Quiet, compact: true, onclick: dismiss, "Done" }
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use duet_client::connected_care::{
        ConnectedCareConnection, ConnectedCareConnectionStanding, ConnectedCareRosterPatient,
        ConnectedCareShareResult, InvitationGateState,
    };
    use uuid::Uuid;

    #[test]
    fn dob_gate_withholds_preview_until_dob_matches_and_share_connects() {
        let cred = InvitationCredential::ShortCode("ABCD-EFGH".into());
        assert_eq!(
            initial_state(Some("ABCD-EFGH")),
            InviteFlowState::Checking {
                credential: cred.clone(),
            }
        );

        let after_inspect = transition_after_inspect(
            cred.clone(),
            Ok(InvitationGateState {
                standing: InvitationOpenStanding::Active {
                    wrong_dob_attempts: 0,
                    attempts_remaining: 5,
                },
            }),
        );
        assert!(matches!(after_inspect, InviteFlowState::EnterDob { .. }));

        let after_wrong_dob = transition_after_verify_dob(
            cred.clone(),
            "1987-02-19".into(),
            Err(ConnectedCareError::WrongBirthDate {
                attempts_remaining: 4,
            }),
        );
        assert!(matches!(
            after_wrong_dob,
            InviteFlowState::EnterDob { error: Some(_), .. }
        ));

        let after_lock = transition_after_verify_dob(
            cred.clone(),
            "1987-02-19".into(),
            Err(ConnectedCareError::Locked),
        );
        assert_eq!(after_lock, InviteFlowState::Locked);

        let after_underage = transition_after_verify_dob(
            cred.clone(),
            "2010-01-01".into(),
            Err(ConnectedCareError::Underage),
        );
        assert_eq!(after_underage, InviteFlowState::Underage);

        let preview = InvitationSharingPreview {
            practice_id: "prac-1".into(),
            practice_name: "River Bend Clinic".into(),
            physician_name: "Dr. Roger".into(),
            display_name: "Tamika".into(),
            shares_summary: "Shares labs and medications".into(),
        };
        let after_dob =
            transition_after_verify_dob(cred.clone(), "1987-02-18".into(), Ok(preview.clone()));
        assert!(matches!(after_dob, InviteFlowState::Preview { .. }));

        let after_share = transition_after_share(
            cred,
            "1987-02-18".into(),
            preview,
            Ok(ConnectedCareShareResult {
                front_door_account_id: Uuid::nil(),
                connection: ConnectedCareConnection {
                    patient: ConnectedCareRosterPatient {
                        practice_id: "prac-1".into(),
                        patient_id: "pat-1".into(),
                    },
                    standing: ConnectedCareConnectionStanding::Connected {
                        connected_at: 1_760_000_000,
                    },
                },
            }),
        );
        assert_eq!(
            after_share,
            InviteFlowState::Connected {
                practice_name: "River Bend Clinic".into(),
            }
        );
    }

    #[test]
    fn invite_panel_ssr_renders_dob_prompt_with_no_preview_leak() {
        #[component]
        fn Harness() -> Element {
            rsx! {
                InvitePanel {
                    base_url: "http://127.0.0.1:8080".to_owned(),
                    bearer: "ds1.test".to_owned(),
                    pending_invite: None,
                    on_clear_invite: move |_| {},
                }
            }
        }
        let mut dom = VirtualDom::new(Harness);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Enter invite code"), "{html}");
    }
}
