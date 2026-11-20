//! Rights-never-wait Duet settings: profile, appearance, verified email change, source disconnect, leave practice, sign-out, and account deletion.

use crate::account::ActiveSession;
use dioxus::prelude::*;
use duet_client::{
    DuetClient,
    identity::{Appearance, ChallengeCredential, IdentityError, PrivacyRestrictionKind},
    retention::RetentionOwner,
};
use ui::{
    ActionRow, Button, ButtonVariant, Choice, Notice, PageTitle, Segmented, TextField, TextKind,
    Tone,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
enum EmailChangeFlow {
    Idle,
    EnterNew {
        email: String,
        submitting: bool,
        error: Option<String>,
    },
    VerifyNew {
        email: String,
        code: String,
        submitting: bool,
        status: Option<String>,
        error: Option<String>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DeleteAccountFlow {
    Idle,
    Confirming,
    Deleting,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum OperationNotice {
    None,
    Status(String),
    Error(String),
}

/// Account and Rights Settings screen. Available both before and after Terms acceptance.
#[component]
pub(crate) fn SettingsScreen(
    base_url: String,
    active: ActiveSession,
    retention: RetentionOwner,
    on_close: EventHandler<()>,
    on_profile_updated: EventHandler<(Option<String>, Appearance, Option<String>)>,
    on_request_sign_out: EventHandler<()>,
    on_complete_revocation: EventHandler<Option<String>>,
) -> Element {
    let initial_name = active
        .account_state
        .as_ref()
        .and_then(|s| s.display_name.as_ref())
        .and_then(|n| n.clone())
        .unwrap_or_default();
    let mut display_name = use_signal(move || initial_name);
    let mut saving_name = use_signal(|| false);
    let mut appearance = use_signal(|| active.appearance);
    let mut email_display = use_signal(|| active.email.clone().unwrap_or_default());
    let mut email_flow = use_signal(|| EmailChangeFlow::Idle);
    let mut delete_flow = use_signal(|| DeleteAccountFlow::Idle);
    let mut notice = use_signal(|| OperationNotice::None);
    let mut working_right = use_signal(|| false);

    let bearer = active.bearer.clone();
    let fence = active.fence.clone();
    let current_email = email_display();

    let save_name = {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        let retention = retention.clone();
        let fence = fence.clone();
        move |_| {
            if saving_name() {
                return;
            }
            saving_name.set(true);
            notice.set(OperationNotice::None);
            let value = display_name();
            let trimmed = value.trim().to_owned();
            let next_name = (!trimmed.is_empty()).then_some(trimmed);
            let base_url = base_url.clone();
            let bearer = bearer.clone();
            let retention = retention.clone();
            let fence = fence.clone();
            spawn(async move {
                let result = async {
                    let api = DuetClient::new(&base_url, &bearer, "0", "0.1.0")
                        .map_err(|_| IdentityError::Unavailable)?;
                    api.update_display_name(next_name.as_deref()).await
                }
                .await;
                match result {
                    Ok(state) => {
                        let _ = retention.save_account_state(&fence, &state);
                        let updated_name = state.display_name.flatten();
                        display_name.set(updated_name.clone().unwrap_or_default());
                        let cur_email = email_display();
                        on_profile_updated.call((
                            updated_name,
                            appearance(),
                            (!cur_email.is_empty()).then_some(cur_email),
                        ));
                        notice.set(OperationNotice::Status("Saved display name.".into()));
                    }
                    Err(error) => notice.set(OperationNotice::Error(error.to_string())),
                }
                saving_name.set(false);
            });
        }
    };

    let select_appearance = {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        let retention = retention.clone();
        let fence = fence.clone();
        move |index: usize| {
            let chosen = match index {
                1 => Appearance::Light,
                2 => Appearance::Dark,
                _ => Appearance::System,
            };
            appearance.set(chosen);
            let _ = retention.save_appearance(&fence, chosen);
            let cur_email = email_display();
            let trimmed_name = display_name().trim().to_owned();
            on_profile_updated.call((
                (!trimmed_name.is_empty()).then_some(trimmed_name),
                chosen,
                (!cur_email.is_empty()).then_some(cur_email),
            ));
            let base_url = base_url.clone();
            let bearer = bearer.clone();
            spawn(async move {
                if let Ok(api) = DuetClient::new(&base_url, &bearer, "0", "0.1.0") {
                    match api.set_appearance(chosen).await {
                        Ok(_) => notice.set(OperationNotice::Status("Appearance updated.".into())),
                        Err(error) => notice.set(OperationNotice::Error(error.to_string())),
                    }
                }
            });
        }
    };

    let start_email_change = {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        move |_| {
            let EmailChangeFlow::EnterNew {
                email, submitting, ..
            } = email_flow()
            else {
                return;
            };
            if submitting {
                return;
            }
            let target = email.trim().to_owned();
            if target.is_empty() {
                email_flow.set(EmailChangeFlow::EnterNew {
                    email,
                    submitting: false,
                    error: Some("Enter a new email address.".into()),
                });
                return;
            }
            email_flow.set(EmailChangeFlow::EnterNew {
                email: target.clone(),
                submitting: true,
                error: None,
            });
            let base_url = base_url.clone();
            let bearer = bearer.clone();
            spawn(async move {
                let result = async {
                    let api = DuetClient::new(&base_url, &bearer, "0", "0.1.0")
                        .map_err(|_| IdentityError::Unavailable)?;
                    api.begin_email_change(&target).await
                }
                .await;
                match result {
                    Ok(status) => {
                        email_flow.set(EmailChangeFlow::VerifyNew {
                            email: target,
                            code: String::new(),
                            submitting: false,
                            status: Some(status),
                            error: None,
                        });
                    }
                    Err(error) => {
                        email_flow.set(EmailChangeFlow::EnterNew {
                            email: target,
                            submitting: false,
                            error: Some(error.to_string()),
                        });
                    }
                }
            });
        }
    };

    let confirm_email_change = {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        move |_| {
            let EmailChangeFlow::VerifyNew {
                email,
                code,
                submitting,
                status,
                ..
            } = email_flow()
            else {
                return;
            };
            if submitting {
                return;
            }
            email_flow.set(EmailChangeFlow::VerifyNew {
                email: email.clone(),
                code: code.clone(),
                submitting: true,
                status: status.clone(),
                error: None,
            });
            let base_url = base_url.clone();
            let bearer = bearer.clone();
            spawn(async move {
                let result = async {
                    let api = DuetClient::new(&base_url, &bearer, "0", "0.1.0")
                        .map_err(|_| IdentityError::Unavailable)?;
                    api.confirm_email_change(
                        Some(&email),
                        &ChallengeCredential::ShortCode(code.clone()),
                    )
                    .await
                }
                .await;
                match result {
                    Ok(profile) => {
                        let verified = profile.email.unwrap_or(email);
                        email_display.set(verified.clone());
                        email_flow.set(EmailChangeFlow::Idle);
                        on_profile_updated.call((
                            (!display_name().trim().is_empty())
                                .then(|| display_name().trim().to_owned()),
                            appearance(),
                            Some(verified),
                        ));
                        notice.set(OperationNotice::Status("Verified email updated.".into()));
                    }
                    Err(error) => {
                        email_flow.set(EmailChangeFlow::VerifyNew {
                            email,
                            code,
                            submitting: false,
                            status,
                            error: Some(error.to_string()),
                        });
                    }
                }
            });
        }
    };

    let run_withdraw = |kind: PrivacyRestrictionKind, label: &'static str| {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        let retention = retention.clone();
        let fence = fence.clone();
        move |_| {
            if working_right() {
                return;
            }
            working_right.set(true);
            notice.set(OperationNotice::None);
            let base_url = base_url.clone();
            let bearer = bearer.clone();
            let retention = retention.clone();
            let fence = fence.clone();
            spawn(async move {
                let result = async {
                    #[cfg(target_os = "ios")]
                    if kind == PrivacyRestrictionKind::DeviceWithdraw
                        && let Ok(source) =
                            duet_client::apple_health::IosAppleHealth::open_on_device(
                                &retention, &fence,
                            )
                    {
                        let _ = source.disconnect();
                    }
                    let _ = (&retention, &fence);
                    let api = DuetClient::new(&base_url, &bearer, "0", "0.1.0")
                        .map_err(|_| IdentityError::Unavailable)?;
                    api.commit_privacy_restriction(Uuid::new_v4(), kind, None, 0)
                        .await
                }
                .await;
                match result {
                    Ok(_) => notice.set(OperationNotice::Status(label.to_owned())),
                    Err(error) => notice.set(OperationNotice::Error(error.to_string())),
                }
                working_right.set(false);
            });
        }
    };

    let leave_practice = {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        let retention = retention.clone();
        move |_| {
            if working_right() {
                return;
            }
            working_right.set(true);
            notice.set(OperationNotice::None);
            let base_url = base_url.clone();
            let bearer = bearer.clone();
            let retention = retention.clone();
            spawn(async move {
                let _ = retention.set_pending_invite(None);
                let result = async {
                    let api = DuetClient::new(&base_url, &bearer, "0", "0.1.0")
                        .map_err(|_| duet_client::FrontDoorError::Unavailable)?;
                    api.withdraw_consent().await
                }
                .await;
                match result {
                    Ok(()) => {
                        notice.set(OperationNotice::Status("Left practice connection.".into()))
                    }
                    Err(error) => notice.set(OperationNotice::Error(error.message())),
                }
                working_right.set(false);
            });
        }
    };

    let sign_out_device = move |_| {
        if working_right() {
            return;
        }
        on_request_sign_out.call(());
    };

    let sign_out_everywhere = {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        move |_| {
            if working_right() {
                return;
            }
            working_right.set(true);
            let base_url = base_url.clone();
            let bearer = bearer.clone();
            spawn(async move {
                let result = async {
                    let api = DuetClient::new(&base_url, &bearer, "0", "0.1.0")
                        .map_err(|_| IdentityError::Unavailable)?;
                    api.commit_privacy_restriction(
                        Uuid::new_v4(),
                        PrivacyRestrictionKind::SessionsRevoke,
                        None,
                        0,
                    )
                    .await
                }
                .await;
                match result {
                    Ok(_) => on_complete_revocation.call(Some("Signed out everywhere.".into())),
                    Err(error) => notice.set(OperationNotice::Error(error.to_string())),
                }
                working_right.set(false);
            });
        }
    };

    let confirm_delete = {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        move |_| {
            if delete_flow() == DeleteAccountFlow::Deleting || working_right() {
                return;
            }
            delete_flow.set(DeleteAccountFlow::Deleting);
            working_right.set(true);
            let base_url = base_url.clone();
            let bearer = bearer.clone();
            spawn(async move {
                let result = async {
                    let api = DuetClient::new(&base_url, &bearer, "0", "0.1.0")
                        .map_err(|_| IdentityError::Unavailable)?;
                    api.commit_privacy_restriction(
                        Uuid::new_v4(),
                        PrivacyRestrictionKind::AccountDelete,
                        None,
                        0,
                    )
                    .await
                }
                .await;
                match result {
                    Ok(_) => on_complete_revocation.call(Some("Account deleted.".into())),
                    Err(error) => {
                        delete_flow.set(DeleteAccountFlow::Confirming);
                        notice.set(OperationNotice::Error(error.to_string()));
                    }
                }
                working_right.set(false);
            });
        }
    };

    let selected_appearance = match appearance() {
        Appearance::System => 0,
        Appearance::Light => 1,
        Appearance::Dark => 2,
    };

    rsx! {
        div { class: "duet-page",
            ActionRow {
                Button { variant: ButtonVariant::Quiet, compact: true, onclick: move |_| on_close.call(()), "Back" }
            }
            PageTitle { text: "Settings".to_owned(), eyebrow: Some("Account and privacy".to_owned()) }
            match notice() {
                OperationNotice::None => rsx! {},
                OperationNotice::Status(text) => rsx! { Notice { tone: Tone::Primary, title: text } },
                OperationNotice::Error(text) => rsx! { Notice { tone: Tone::Error, title: text } },
            }
            section { class: "duet-settings-section",
                h2 { "Profile" }
                TextField {
                    label: "Display name".to_owned(),
                    value: display_name(),
                    placeholder: "Your name".to_owned(),
                    disabled: saving_name(),
                    on_input: move |value| display_name.set(value),
                }
                ActionRow {
                    Button { disabled: saving_name(), onclick: save_name, "Save name" }
                }
                p { class: "record-meta", "Appearance" }
                Segmented {
                    label: "Appearance".to_owned(),
                    options: vec![
                        Choice::from(("System", "Follow device appearance")),
                        Choice::from(("Light", "Light appearance")),
                        Choice::from(("Dark", "Dark appearance")),
                    ],
                    selected: selected_appearance,
                    on_select: select_appearance,
                }
            }
            section { class: "duet-settings-section",
                h2 { "Verified email" }
                p { class: "status-detail", if current_email.is_empty() { "No email on file." } else { "{current_email}" } }
                match email_flow() {
                    EmailChangeFlow::Idle => rsx! {
                        ActionRow {
                            Button {
                                onclick: move |_| email_flow.set(EmailChangeFlow::EnterNew {
                                    email: String::new(),
                                    submitting: false,
                                    error: None,
                                }),
                                "Change email"
                            }
                        }
                    },
                    EmailChangeFlow::EnterNew { email, submitting, error } => rsx! {
                        TextField {
                            label: "New email address".to_owned(),
                            kind: TextKind::Email,
                            value: email.clone(),
                            placeholder: "you@example.com".to_owned(),
                            error,
                            disabled: submitting,
                            on_input: move |value| email_flow.set(EmailChangeFlow::EnterNew {
                                email: value,
                                submitting: false,
                                error: None,
                            }),
                        }
                        ActionRow {
                            Button { variant: ButtonVariant::Primary, disabled: submitting, onclick: start_email_change, "Send confirmation" }
                            Button { variant: ButtonVariant::Quiet, disabled: submitting, onclick: move |_| email_flow.set(EmailChangeFlow::Idle), "Cancel" }
                        }
                    },
                    EmailChangeFlow::VerifyNew { email, code, submitting, status, error } => rsx! {
                        if let Some(status) = status.clone() {
                            Notice { tone: Tone::Neutral, title: status, detail: Some(format!("Open the link or enter the 6-digit code sent to {email}.")) }
                        }
                        TextField {
                            label: "Verification code".to_owned(),
                            value: code.clone(),
                            placeholder: "123456".to_owned(),
                            error,
                            disabled: submitting,
                            on_input: move |value| email_flow.set(EmailChangeFlow::VerifyNew {
                                email: email.clone(),
                                code: value,
                                submitting: false,
                                status: status.clone(),
                                error: None,
                            }),
                        }
                        ActionRow {
                            Button { variant: ButtonVariant::Primary, disabled: submitting, onclick: confirm_email_change, "Confirm new email" }
                            Button { variant: ButtonVariant::Quiet, disabled: submitting, onclick: move |_| email_flow.set(EmailChangeFlow::Idle), "Cancel" }
                        }
                    },
                }
            }
            section { class: "duet-settings-section",
                h2 { "Sources & connections" }
                ActionRow {
                    Button { disabled: working_right(), onclick: run_withdraw(PrivacyRestrictionKind::DeviceWithdraw, "Apple Health disconnected."), "Disconnect Apple Health" }
                    Button { disabled: working_right(), onclick: run_withdraw(PrivacyRestrictionKind::RecordsWithdraw, "Health Records disconnected."), "Disconnect Health Records" }
                    Button { disabled: working_right(), onclick: leave_practice, "Leave practice" }
                }
            }
            section { class: "duet-settings-section",
                h2 { "Sessions & account" }
                ActionRow {
                    Button { variant: ButtonVariant::Quiet, disabled: working_right(), onclick: sign_out_device, "Sign out" }
                    Button { disabled: working_right(), onclick: sign_out_everywhere, "Sign out everywhere" }
                }
                match delete_flow() {
                    DeleteAccountFlow::Idle => rsx! {
                        ActionRow {
                            Button { danger: true, disabled: working_right(), onclick: move |_| delete_flow.set(DeleteAccountFlow::Confirming), "Delete account" }
                        }
                    },
                    DeleteAccountFlow::Confirming | DeleteAccountFlow::Deleting => rsx! {
                        Notice {
                            tone: Tone::Warning,
                            title: "Delete your account?".to_owned(),
                            detail: Some("This removes all health information and stops practice sharing. You can sign up again later with a new empty account.".to_owned()),
                        }
                        ActionRow {
                            Button {
                                variant: ButtonVariant::Primary,
                                danger: true,
                                disabled: delete_flow() == DeleteAccountFlow::Deleting,
                                onclick: confirm_delete,
                                "Delete account permanently"
                            }
                            Button {
                                variant: ButtonVariant::Quiet,
                                disabled: delete_flow() == DeleteAccountFlow::Deleting,
                                onclick: move |_| delete_flow.set(DeleteAccountFlow::Idle),
                                "Keep account"
                            }
                        }
                    },
                }
            }
        }
    }
}
