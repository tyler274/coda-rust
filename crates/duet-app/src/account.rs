//! One account lifecycle above every private route. Keyed remount drops old UI/tasks.
use crate::settings::SettingsScreen;
use dioxus::prelude::*;
use duet_client::{
    DuetClient,
    identity::{Appearance, ChallengeCredential, IdentityError},
    retention::{RetainedIdentity, RetentionOwner},
};
use std::sync::Arc;
use ui::{
    ActionRow, Button, ButtonVariant, Checkbox, Disclosure, Notice, PageTitle, TextField, TextKind,
    Tone,
};

pub(crate) const CURRENT_TERMS_VERSION: &str = "2026-09-23";

/// Account-scoped owner available to editor controls and delayed-response checks.
#[derive(Clone)]
pub struct PatientSession {
    /// Protected draft owner and request fences.
    pub retention: RetentionOwner,
    /// Captured before dispatch; check before applying responses.
    pub fence: duet_client::session::SessionFence,
    /// Current session token; new requests observe updates; no mutation is replayed.
    pub access_token: Signal<String>,
}

/// Live reachability status surfaced in tab headings (`"Connecting…"` while offline/checking).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ConnectionStatus {
    Online,
    Connecting,
}

#[derive(Clone, Copy)]
pub(crate) struct AccountConnection(pub Signal<ConnectionStatus>);

#[derive(Clone)]
pub(crate) struct RefusalSender(
    pub  futures_channel::mpsc::UnboundedSender<(
        duet_client::session::SessionFence,
        duet_client::refusal::SessionRefusal,
    )>,
);

/// Active signed-in session rendered above Terms and private tabs.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ActiveSession {
    pub bearer: String,
    pub fence: duet_client::session::SessionFence,
    pub email: Option<String>,
    pub appearance: Appearance,
    pub account_state: Option<duet_wire::models::AccountState>,
}

/// Incoming practice invitation token or short code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InviteHandoff {
    /// Invitation token carried in `/invite` or `duet://invite` links.
    Token(String),
    /// Human-entered or short invitation code fallback.
    ShortCode(String),
}

impl InviteHandoff {
    /// Underlying invitation credential value.
    pub fn value(&self) -> &str {
        match self {
            Self::Token(value) | Self::ShortCode(value) => value,
        }
    }
}

/// Parsed incoming deep link / universal link routed into Duet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppDeepLink {
    /// Open a practice invitation inside the signed-in (or newly signed-in) account.
    Invite(InviteHandoff),
    /// Complete passwordless email sign-in/sign-up via single-use link token.
    EmailSignIn {
        /// Single-use `dl1.` link token.
        token: String,
        /// Optional practice invitation carried across sign-up.
        invite: Option<String>,
    },
    /// Confirm a verified change-email link while signed in.
    EmailChange {
        /// Single-use `dl1.` email-change token.
        token: String,
    },
}

fn valid_link_param(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

/// Parse an incoming `duet://` or `https://` deep link into a typed [`AppDeepLink`].
pub fn parse_app_link(raw: &str) -> Option<AppDeepLink> {
    let parsed = url::Url::parse(raw.trim()).ok()?;
    let route = match parsed.scheme() {
        "duet" => {
            let host = parsed.host_str().unwrap_or("");
            let path = parsed.path().trim_matches('/');
            if path.is_empty() {
                host.to_owned()
            } else if host.is_empty() {
                path.to_owned()
            } else {
                format!("{host}/{path}")
            }
        }
        "https" | "http" => parsed.path().trim_matches('/').to_owned(),
        _ => return None,
    };
    let mut token: Option<String> = None;
    let mut code: Option<String> = None;
    let mut invite: Option<String> = None;
    for (k, v) in parsed.query_pairs() {
        if !valid_link_param(&v) {
            continue;
        }
        match k.as_ref() {
            "token" => token = Some(v.into_owned()),
            "code" => code = Some(v.into_owned()),
            "invite" => invite = Some(v.into_owned()),
            _ => {}
        }
    }
    match route.as_str() {
        "invite" => {
            if let Some(tok) = token {
                Some(AppDeepLink::Invite(InviteHandoff::Token(tok)))
            } else {
                code.map(|c| AppDeepLink::Invite(InviteHandoff::ShortCode(c)))
            }
        }
        "auth/email" => token.map(|tok| AppDeepLink::EmailSignIn {
            token: tok,
            invite: invite.or(code),
        }),
        "auth/email-change" => token.map(|tok| AppDeepLink::EmailChange { token: tok }),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum EmailSignInStep {
    EnterEmail {
        email: String,
        invite: String,
        submitting: bool,
        error: Option<String>,
    },
    CheckEmail {
        email: String,
        code: String,
        invite: String,
        submitting: bool,
        status: String,
        error: Option<String>,
    },
}

impl EmailSignInStep {
    fn fresh() -> Self {
        Self::EnterEmail {
            email: String::new(),
            invite: String::new(),
            submitting: false,
            error: None,
        }
    }
}

/// Why startup verification produced no online update.
#[derive(Debug)]
enum SignInError {
    Client(String),
    Account(IdentityError),
    Context(IdentityError),
    Retention(String),
    Superseded,
}

impl std::fmt::Display for SignInError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Client(cause) => write!(f, "Duet is unavailable: {cause}"),
            Self::Account(err) | Self::Context(err) => f.write_str(&err.to_string()),
            Self::Retention(words) => f.write_str(words),
            Self::Superseded => f.write_str("Account changed; sign-in ignored."),
        }
    }
}

impl std::error::Error for SignInError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Account(err) | Self::Context(err) => Some(err),
            _ => None,
        }
    }
}

/// Standalone email-authenticated entry. Practice linking stays optional, never a login gate.
#[component]
pub fn DuetRoot(
    base_url: String,
    bearer: String,
    #[props(default)] initial_link: Option<String>,
) -> Element {
    let retention = use_hook(|| {
        let _ = duet_client::retention::install_privacy_cover();
        #[cfg(any(target_os = "ios", target_os = "android"))]
        {
            RetentionOwner::new(Arc::new(duet_client::retention::NativeProtectedStore))
        }
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            RetentionOwner::new(Arc::new(
                duet_client::retention::VolatileSessionStore::default(),
            ))
        }
    });
    let drop_session = retention.session();
    use_drop(move || {
        drop_session.switch(None);
    });

    let mut offline_draft = use_signal(|| None::<(String, duet_client::retention::DoctorDraft)>);
    let mut active = use_signal(|| None::<ActiveSession>);
    let mut connection = use_signal(|| ConnectionStatus::Online);
    let mut message = use_signal(String::new);
    let mut busy = use_signal(|| true);
    let mut retry = use_signal(|| 0u64);
    let mut signed_out = use_signal(|| false);
    let mut cleanup_pending = use_signal(|| false);
    let mut lifecycle = use_signal(|| 0u64);
    let mut confirm_sign_out = use_signal(|| false);
    let mut settings_open = use_signal(|| false);
    let mut agreed_terms = use_signal(|| false);
    let mut terms_submitting = use_signal(|| false);
    let mut terms_error = use_signal(|| None::<String>);
    let mut upgrade_required = use_signal(|| None::<String>);
    let mut pending_invite = use_signal(|| retention.pending_invite().ok().flatten());
    let initial_parsed_link = use_hook(|| {
        initial_link
            .as_deref()
            .and_then(parse_app_link)
            .or_else(browser_deep_link)
    });
    let mut sign_in_step = use_signal(|| {
        let initial_invite = match &initial_parsed_link {
            Some(AppDeepLink::Invite(handoff)) => handoff.value().to_owned(),
            Some(AppDeepLink::EmailSignIn {
                invite: Some(inv), ..
            }) => inv.clone(),
            _ => retention
                .pending_invite()
                .ok()
                .flatten()
                .unwrap_or_default(),
        };
        EmailSignInStep::EnterEmail {
            email: String::new(),
            invite: initial_invite,
            submitting: false,
            error: None,
        }
    });

    let account_ui = AccountUi {
        lifecycle,
        signed_out,
        active,
        offline_draft,
        busy,
        cleanup_pending,
        message,
        confirm: confirm_sign_out,
        settings_open,
        upgrade_required,
        agreed_terms,
        terms_error,
        sign_in_step,
        pending_invite,
    };

    let (refusal_sender, refusal_receiver) = use_hook(|| {
        let (sender, receiver) = futures_channel::mpsc::unbounded();
        (
            sender,
            std::rc::Rc::new(std::cell::RefCell::new(Some(receiver))),
        )
    });
    use_context_provider(|| RefusalSender(refusal_sender));
    use_context_provider(|| AccountConnection(connection));

    let refusal_owner = retention.clone();
    use_future(move || {
        let owner = refusal_owner.clone();
        let receiver = refusal_receiver.borrow_mut().take();
        async move {
            use futures_util::StreamExt;
            let Some(mut receiver) = receiver else {
                return;
            };
            while let Some((fence, refusal)) = receiver.next().await {
                if active
                    .peek()
                    .as_ref()
                    .is_none_or(|value| value.fence != fence)
                {
                    continue;
                }
                match refusal {
                    duet_client::refusal::SessionRefusal::Revoked
                    | duet_client::refusal::SessionRefusal::Absent => {
                        settings_open.set(false);
                        account_ui.ended(&owner);
                    }
                    duet_client::refusal::SessionRefusal::Invalid => {
                        settings_open.set(false);
                        account_ui.close();
                        message.set("Duet could not authorize this session. Sign in again. Unsent text on this screen is no longer shown; protected drafts remain on this device. No pending operation was retried.".into());
                    }
                }
            }
        }
    });

    let restore_owner = retention.clone();
    let restore_url = base_url.clone();
    let supplied = bearer;
    let startup_link = initial_parsed_link.clone();
    use_effect(move || {
        let _ = retry();
        if *signed_out.peek() {
            busy.set(false);
            return;
        }
        let epoch = *lifecycle.peek();
        let owner = restore_owner.clone();
        let base = restore_url.clone();
        let supplied = supplied.clone();
        let startup_link = startup_link.clone();
        busy.set(true);
        spawn(async move {
            let _ = crate::roots::recalled_session();
            if owner.cleanup_required().unwrap_or(true) {
                cleanup_pending.set(true);
                signed_out.set(true);
                message.set("Complete secure sign-out cleanup before opening an account.".into());
                busy.set(false);
                return;
            }
            if let Some(AppDeepLink::Invite(ref handoff)) = startup_link {
                let _ = owner.set_pending_invite(Some(handoff.value()));
                pending_invite.set(Some(handoff.value().to_owned()));
            }
            if !supplied.is_empty() {
                let result =
                    restore_supplied(&base, supplied, &owner, || lifecycle() == epoch).await;
                if lifecycle() != epoch {
                    return;
                }
                connection.set(ConnectionStatus::Online);
                account_ui.restored(&owner, result);
                return;
            }

            // Non-expiring offline-first restore runs BEFORE email sign-in links so opening
            // a sign-in link while already signed in opens Duet as it was.
            match owner.restore_offline_session() {
                Ok(Some((retained, fence))) => {
                    let cached_state = owner.load_account_state(&fence).ok().flatten();
                    let cached_appearance = owner
                        .load_appearance(&fence)
                        .ok()
                        .flatten()
                        .unwrap_or_default();
                    offline_draft.set(None);
                    connection.set(ConnectionStatus::Connecting);
                    active.set(Some(ActiveSession {
                        bearer: retained.session_token.clone(),
                        fence: fence.clone(),
                        email: retained.email.clone(),
                        appearance: cached_appearance,
                        account_state: cached_state,
                    }));
                    busy.set(false);

                    // Validate session in background without dropping cached content if unreachable.
                    let validation = validate_retained(
                        &base,
                        &retained.session_token,
                        &owner,
                        &fence,
                        startup_link.as_ref(),
                        || lifecycle() == epoch,
                    )
                    .await;
                    if lifecycle() != epoch {
                        return;
                    }
                    match validation {
                        Ok((state, profile)) => {
                            let _ = owner.save_account_state(&fence, &state);
                            let _ = owner.save_appearance(&fence, profile.appearance);
                            connection.set(ConnectionStatus::Online);
                            let snapshot = active.peek().clone();
                            if let Some(mut cur) = snapshot
                                && cur.fence == fence
                            {
                                cur.account_state = Some(state);
                                cur.appearance = profile.appearance;
                                if profile.email.is_some() {
                                    cur.email = profile.email;
                                }
                                active.set(Some(cur));
                            }
                            message.set(String::new());
                        }
                        Err(SignInError::Account(IdentityError::Ended))
                        | Err(SignInError::Context(IdentityError::Ended)) => {
                            account_ui.ended(&owner);
                        }
                        Err(
                            SignInError::Account(IdentityError::AgreementChanged)
                            | SignInError::Context(IdentityError::AgreementChanged),
                        ) => {
                            connection.set(ConnectionStatus::Online);
                            upgrade_required.set(Some(IdentityError::AgreementChanged.to_string()));
                        }
                        Err(SignInError::Account(IdentityError::Unauthorized))
                        | Err(SignInError::Context(IdentityError::Unauthorized)) => {
                            account_ui.close();
                            message.set(
                                "Duet could not authorize this session. Sign in again.".into(),
                            );
                        }
                        Err(_) => {
                            // Unreachable/offline: stay mounted with cached data and `"Connecting…"`.
                            connection.set(ConnectionStatus::Connecting);
                        }
                    }
                }
                Ok(None) => {
                    if let Some(AppDeepLink::EmailSignIn {
                        ref token,
                        ref invite,
                    }) = startup_link
                    {
                        if let Some(inv) = invite {
                            let _ = owner.set_pending_invite(Some(inv));
                            pending_invite.set(Some(inv.clone()));
                        }
                        match complete_link_sign_in(&base, token, &owner, || lifecycle() == epoch)
                            .await
                        {
                            Ok(session) => {
                                if lifecycle() != epoch {
                                    return;
                                }
                                connection.set(ConnectionStatus::Online);
                                offline_draft.set(None);
                                account_ui.reset_session_ui();
                                active.set(Some(session));
                                message.set(String::new());
                                busy.set(false);
                                return;
                            }
                            Err(err) => {
                                if lifecycle() != epoch {
                                    return;
                                }
                                message.set(err.to_string());
                                busy.set(false);
                                return;
                            }
                        }
                    }
                    message.set(String::new());
                    busy.set(false);
                }
                Err(error) => {
                    message.set(error);
                    busy.set(false);
                }
            }
        });
    });

    let start_email = {
        let base = base_url.clone();
        let owner = retention.clone();
        move |_| {
            let EmailSignInStep::EnterEmail {
                email,
                invite,
                submitting,
                ..
            } = sign_in_step()
            else {
                return;
            };
            if submitting {
                return;
            }
            let normalized = email.trim().to_owned();
            if normalized.is_empty() {
                sign_in_step.set(EmailSignInStep::EnterEmail {
                    email,
                    invite,
                    submitting: false,
                    error: Some("Enter your email address.".into()),
                });
                return;
            }
            let invite_trimmed = invite.trim().to_owned();
            let _ = owner.set_pending_invite(
                (!invite_trimmed.is_empty()).then_some(invite_trimmed.as_str()),
            );
            if !invite_trimmed.is_empty() {
                pending_invite.set(Some(invite_trimmed.clone()));
            }
            sign_in_step.set(EmailSignInStep::EnterEmail {
                email: normalized.clone(),
                invite: invite_trimmed.clone(),
                submitting: true,
                error: None,
            });
            let base = base.clone();
            spawn(async move {
                let result = async {
                    let api = DuetClient::new(&base, "", "0", "0.1.0")
                        .map_err(|_| IdentityError::Unavailable)?;
                    api.begin_email_sign_in(
                        &normalized,
                        (!invite_trimmed.is_empty()).then_some(invite_trimmed.as_str()),
                    )
                    .await
                }
                .await;
                match result {
                    Ok(status) => {
                        sign_in_step.set(EmailSignInStep::CheckEmail {
                            email: normalized,
                            code: String::new(),
                            invite: invite_trimmed,
                            submitting: false,
                            status,
                            error: None,
                        });
                    }
                    Err(err) => {
                        sign_in_step.set(EmailSignInStep::EnterEmail {
                            email: normalized,
                            invite: invite_trimmed,
                            submitting: false,
                            error: Some(err.to_string()),
                        });
                    }
                }
            });
        }
    };

    let verify_code = {
        let base = base_url.clone();
        let owner = retention.clone();
        move |_| {
            let EmailSignInStep::CheckEmail {
                email,
                code,
                invite,
                submitting,
                status,
                ..
            } = sign_in_step()
            else {
                return;
            };
            if submitting {
                return;
            }
            sign_in_step.set(EmailSignInStep::CheckEmail {
                email: email.clone(),
                code: code.clone(),
                invite: invite.clone(),
                submitting: true,
                status: status.clone(),
                error: None,
            });
            lifecycle.set(lifecycle() + 1);
            let epoch = lifecycle();
            let base = base.clone();
            let owner = owner.clone();
            spawn(async move {
                let result =
                    complete_code_sign_in(&base, &email, &code, &owner, || lifecycle() == epoch)
                        .await;
                if lifecycle() != epoch {
                    return;
                }
                match result {
                    Ok(session) => {
                        connection.set(ConnectionStatus::Online);
                        signed_out.set(false);
                        account_ui.reset_session_ui();
                        let owner_id = session.fence.owner().to_owned();
                        offline_draft.set(step_offline_draft(
                            offline_draft(),
                            DraftTransition::Activate { owner: owner_id },
                        ));
                        active.set(Some(session));
                        message.set(String::new());
                    }
                    Err(err) => {
                        sign_in_step.set(EmailSignInStep::CheckEmail {
                            email,
                            code,
                            invite,
                            submitting: false,
                            status,
                            error: Some(err.to_string()),
                        });
                    }
                }
            });
        }
    };

    let resend_email = {
        let base = base_url.clone();
        move |_| {
            let EmailSignInStep::CheckEmail {
                email,
                code,
                invite,
                submitting,
                ..
            } = sign_in_step()
            else {
                return;
            };
            if submitting {
                return;
            }
            let base = base.clone();
            let email_req = email.clone();
            let invite_req = invite.clone();
            spawn(async move {
                if let Ok(api) = DuetClient::new(&base, "", "0", "0.1.0")
                    && let Ok(status) = api
                        .begin_email_sign_in(
                            &email_req,
                            (!invite_req.is_empty()).then_some(invite_req.as_str()),
                        )
                        .await
                {
                    sign_in_step.set(EmailSignInStep::CheckEmail {
                        email: email_req,
                        code,
                        invite: invite_req,
                        submitting: false,
                        status,
                        error: None,
                    });
                }
            });
        }
    };

    let accept_terms = {
        let base = base_url.clone();
        let owner = retention.clone();
        move |_| {
            let Some(mut cur) = active() else {
                return;
            };
            if !agreed_terms() || terms_submitting() {
                return;
            }
            terms_submitting.set(true);
            terms_error.set(None);
            let base = base.clone();
            let owner = owner.clone();
            let bearer = cur.bearer.clone();
            let fence = cur.fence.clone();
            spawn(async move {
                let result = async {
                    let api = DuetClient::new(&base, &bearer, "0", "0.1.0")
                        .map_err(|_| IdentityError::Unavailable)?;
                    api.accept_terms(CURRENT_TERMS_VERSION).await
                }
                .await;
                match result {
                    Ok(state) => {
                        let _ = owner.save_account_state(&fence, &state);
                        cur.account_state = Some(state);
                        active.set(Some(cur));
                        upgrade_required.set(None);
                        connection.set(ConnectionStatus::Online);
                    }
                    Err(IdentityError::AgreementChanged) => {
                        upgrade_required
                            .set(Some("Update Duet to review the current terms.".into()));
                    }
                    Err(err) => terms_error.set(Some(err.to_string())),
                }
                terms_submitting.set(false);
            });
        }
    };

    let logout_owner = retention.clone();
    let revoke_owner = retention.clone();
    let settings_sign_out_owner = retention.clone();
    let settings_sign_out_base = base_url.clone();
    let upgrade_sign_out_owner = retention.clone();
    let upgrade_sign_out_base = base_url.clone();
    let terms_sign_out_owner = retention.clone();
    let terms_sign_out_base = base_url.clone();
    let confirm_sign_out_owner = retention.clone();
    let confirm_sign_out_base = base_url.clone();
    let retry_cleanup = move |_| {
        apply_sign_out(&logout_owner, account_ui, false);
    };
    let offline_owner = retention.clone();
    let draft_viewer = offline_owner
        .retained_identity()
        .ok()
        .flatten()
        .map(|identity| identity.owner);

    rsx! {
        ui::Stylesheet {}
        if let Some(current) = active() {
            if confirm_sign_out() {
                main { class: "duet theme-duet", "data-theme": "{current.appearance.as_str()}",
                    div { class: "duet-page",
                        Notice {
                            tone: Tone::Warning,
                            title: message(),
                            ActionRow {
                                Button { variant: ButtonVariant::Quiet, compact: true, onclick: move |_| {
                                    confirm_sign_out.set(false);
                                    message.set(String::new());
                                }, "Stay signed in" }
                                Button { variant: ButtonVariant::Primary, compact: true, onclick: move |_| {
                                    start_device_sign_out(
                                        &confirm_sign_out_base,
                                        &confirm_sign_out_owner,
                                        active,
                                        account_ui,
                                        false,
                                    );
                                }, "Sign out anyway" }
                            }
                        }
                    }
                }
            } else if settings_open() {
                main { class: "duet theme-duet", "data-theme": "{current.appearance.as_str()}",
                    SettingsScreen {
                        base_url: base_url.clone(),
                        active: current.clone(),
                        retention: retention.clone(),
                        on_close: move |_| settings_open.set(false),
                        on_profile_updated: move |(new_name, new_appearance, new_email)| {
                            if let Some(mut cur) = active() {
                                cur.appearance = new_appearance;
                                cur.email = new_email;
                                if let Some(ref mut state) = cur.account_state {
                                    state.display_name = Some(new_name);
                                }
                                active.set(Some(cur));
                            }
                        },
                        on_request_sign_out: move |_| {
                            start_device_sign_out(
                                &settings_sign_out_base,
                                &settings_sign_out_owner,
                                active,
                                account_ui,
                                true,
                            );
                        },
                        on_complete_revocation: move |note: Option<String>| {
                            let result = account_ui.erase(&revoke_owner);
                            message.set(match result {
                                Ok(()) => note.unwrap_or_default(),
                                Err(error) => format!(
                                    "Signed out on this screen. Secure cleanup needs retry: {error}"
                                ),
                            });
                        },
                    }
                }
            } else if let Some(upgrade_msg) = upgrade_required() {
                main { class: "duet theme-duet", "data-theme": "{current.appearance.as_str()}",
                    div { class: "duet-page",
                        PageTitle { text: "Update Duet".to_owned(), eyebrow: Some("Terms updated".to_owned()) }
                        Notice { tone: Tone::Warning, title: upgrade_msg }
                        ActionRow {
                            Button { onclick: move |_| settings_open.set(true), "Settings" }
                            Button { variant: ButtonVariant::Quiet, onclick: move |_| {
                                start_device_sign_out(
                                    &upgrade_sign_out_base,
                                    &upgrade_sign_out_owner,
                                    active,
                                    account_ui,
                                    true,
                                );
                            }, "Sign out" }
                        }
                    }
                }
            } else if current.account_state.as_ref().and_then(|s| s.terms_accepted) != Some(true) {
                main { class: "duet theme-duet", "data-theme": "{current.appearance.as_str()}",
                    div { class: "duet-page",
                        PageTitle { text: "Duet Terms".to_owned(), eyebrow: Some("Before you continue".to_owned()) }
                        if connection() == ConnectionStatus::Connecting {
                            p { class: "duet-status", "{crate::roots::CONNECTING}" }
                        }
                        p { "Please review and accept the Duet Terms of Service and Privacy Policy before using You and Shared." }
                        if let Some(err) = terms_error() {
                            Notice { tone: Tone::Error, title: err }
                        }
                        Checkbox {
                            label: "I am 18 or older and live in the United States".to_owned(),
                            detail: Some("Duet is for U.S. adults and does not provide emergency care or medical diagnosis.".to_owned()),
                            checked: agreed_terms(),
                            on_change: move |val| agreed_terms.set(val),
                        }
                        ActionRow {
                            Button {
                                variant: ButtonVariant::Primary,
                                disabled: !agreed_terms() || terms_submitting(),
                                onclick: accept_terms,
                                "I agree and continue"
                            }
                            Button { onclick: move |_| settings_open.set(true), "Settings" }
                            Button { variant: ButtonVariant::Quiet, onclick: move |_| {
                                start_device_sign_out(
                                    &terms_sign_out_base,
                                    &terms_sign_out_owner,
                                    active,
                                    account_ui,
                                    true,
                                );
                            }, "Sign out" }
                        }
                    }
                }
            } else {
                AccountRoutes {
                    key: "{current.fence.generation()}",
                    base_url,
                    bearer: current.bearer.clone(),
                    appearance: current.appearance,
                    retention: retention.clone(),
                    fence: current.fence.clone(),
                    pending_invite: pending_invite(),
                    on_open_settings: move |_| settings_open.set(true),
                    on_clear_invite: move |_| {
                        let _ = retention.set_pending_invite(None);
                        pending_invite.set(None);
                    },
                }
            }
        } else {
            main { class: "duet theme-duet",
                div { class: "duet-page",
                    h1 { "Duet" }
                    p { "Sign in to your Duet account. A practice invitation is not required." }
                    if !message().is_empty() { p { role: "alert", "{message}" } }
                    if busy() { p { "Checking sign-in…" } }
                    if cleanup_pending() {
                        ActionRow {
                            Button { onclick: retry_cleanup, "Retry secure cleanup" }
                        }
                    } else {
                        match sign_in_step() {
                            EmailSignInStep::EnterEmail { email, invite, submitting, error } => {
                                let invite_for_email = invite.clone();
                                rsx! {
                                TextField {
                                    label: "Email".to_owned(),
                                    kind: TextKind::Email,
                                    value: email,
                                    placeholder: "you@example.com".to_owned(),
                                    error,
                                    disabled: submitting || busy(),
                                    on_input: move |value| sign_in_step.set(EmailSignInStep::EnterEmail {
                                        email: value,
                                        invite: invite_for_email.clone(),
                                        submitting: false,
                                        error: None,
                                    }),
                                }
                                Disclosure {
                                    title: "Have an invitation code?".to_owned(),
                                    detail: !invite.is_empty(),
                                    TextField {
                                        label: "Invitation code".to_owned(),
                                        value: invite.clone(),
                                        placeholder: "Paste invitation code".to_owned(),
                                        disabled: submitting,
                                        on_input: move |value| {
                                            if let EmailSignInStep::EnterEmail { email, .. } = sign_in_step() {
                                                sign_in_step.set(EmailSignInStep::EnterEmail {
                                                    email,
                                                    invite: value,
                                                    submitting: false,
                                                    error: None,
                                                });
                                            }
                                        },
                                    }
                                }
                                ActionRow {
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        disabled: submitting || busy(),
                                        onclick: start_email,
                                        "Continue"
                                    }
                                    Button {
                                        variant: ButtonVariant::Quiet,
                                        disabled: busy(),
                                        onclick: move |_| {
                                            signed_out.set(false);
                                            retry.set(retry() + 1);
                                        },
                                        "Retry saved sign-in"
                                    }
                                }
                                }
                            },
                            EmailSignInStep::CheckEmail { email, code, invite, submitting, status, error } => {
                                let email_for_input = email.clone();
                                let invite_for_input = invite.clone();
                                let status_for_input = status.clone();
                                rsx! {
                                Notice {
                                    tone: Tone::Primary,
                                    title: status.clone(),
                                    detail: Some(format!("Open the link or enter the 6-digit code sent to {email}.")),
                                }
                                TextField {
                                    label: "6-digit code".to_owned(),
                                    value: code,
                                    placeholder: "123456".to_owned(),
                                    error,
                                    disabled: submitting,
                                    on_input: move |value| sign_in_step.set(EmailSignInStep::CheckEmail {
                                        email: email_for_input.clone(),
                                        code: value,
                                        invite: invite_for_input.clone(),
                                        submitting: false,
                                        status: status_for_input.clone(),
                                        error: None,
                                    }),
                                }
                                ActionRow {
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        disabled: submitting,
                                        onclick: verify_code,
                                        "Verify"
                                    }
                                    Button {
                                        variant: ButtonVariant::Quiet,
                                        disabled: submitting,
                                        onclick: resend_email,
                                        "Resend"
                                    }
                                    Button {
                                        variant: ButtonVariant::Quiet,
                                        disabled: submitting,
                                        onclick: move |_| sign_in_step.set(EmailSignInStep::EnterEmail {
                                            email: String::new(),
                                            invite: invite.clone(),
                                            submitting: false,
                                            error: None,
                                        }),
                                        "Use a different email"
                                    }
                                }
                                }
                            },
                        }
                    }
                    ActionRow {
                        Button { disabled: busy() || cleanup_pending() || signed_out(), onclick: move |_| {
                            match offline_owner.recover_offline() {
                                Ok(duet_client::retention::DraftRecovery::Draft(draft)) => {
                                    if let Some(owner) = offline_owner
                                        .retained_identity()
                                        .ok()
                                        .flatten()
                                        .map(|identity| identity.owner)
                                    {
                                        offline_draft.set(step_offline_draft(
                                            offline_draft(),
                                            DraftTransition::Recover {
                                                owner,
                                                draft: *draft,
                                            },
                                        ));
                                    }
                                }
                                Ok(duet_client::retention::DraftRecovery::Damaged) => message.set("The protected draft is damaged; it has not been discarded.".into()),
                                Ok(_) => message.set("No retained draft on this device.".into()),
                                Err(error) => message.set(error),
                            }
                        }, "View protected draft offline" }
                    }
                    if let Some(draft) = shown_offline_draft(offline_draft(), true, draft_viewer.as_deref()) {
                        section {
                            h2 { "Unsaved doctor draft — offline" }
                            p { "This is not the approved page. Reconnect and verify your account before saving or sharing." }
                            p { "{crate::field_text(&draft.save.page.name)}" }
                            p { "{crate::field_text(&draft.save.note)}" }
                            for question in draft.save.page.questions.iter().flatten() { p { "{question}" } }
                            for item in draft.base.items.iter().flatten().filter(|item| draft.save.item_ids.contains(&item.id)) { p { "{item.title}" } }
                            Button { onclick: move |_| offline_draft.set(None), "Close protected draft" }
                        }
                    }
                }
            }
        }
    }
}

async fn complete_link_sign_in(
    base: &str,
    token: &str,
    owner: &RetentionOwner,
    current: impl Fn() -> bool,
) -> Result<ActiveSession, SignInError> {
    let api = DuetClient::new(base, "", "0", "0.1.0").map_err(SignInError::Client)?;
    let signed = api
        .complete_email_sign_in(None, &ChallengeCredential::LinkToken(token.to_owned()))
        .await
        .map_err(SignInError::Account)?;
    finish_email_sign_in(base, signed, owner, current).await
}

async fn complete_code_sign_in(
    base: &str,
    email: &str,
    code: &str,
    owner: &RetentionOwner,
    current: impl Fn() -> bool,
) -> Result<ActiveSession, SignInError> {
    let api = DuetClient::new(base, "", "0", "0.1.0").map_err(SignInError::Client)?;
    let signed = api
        .complete_email_sign_in(
            Some(email),
            &ChallengeCredential::ShortCode(code.to_owned()),
        )
        .await
        .map_err(SignInError::Account)?;
    finish_email_sign_in(base, signed, owner, current).await
}

async fn finish_email_sign_in(
    base: &str,
    signed: duet_client::identity::EmailSignInResult,
    owner: &RetentionOwner,
    current: impl Fn() -> bool,
) -> Result<ActiveSession, SignInError> {
    if !current() {
        return Err(SignInError::Superseded);
    }
    let client =
        DuetClient::new(base, &signed.bearer, "0", "0.1.0").map_err(SignInError::Client)?;
    let state = client.account_state().await.map_err(SignInError::Account)?;
    if !current() {
        return Err(SignInError::Superseded);
    }
    let owner_id = signed.owner();
    let fence = owner
        .activate(RetainedIdentity {
            subject: format!("email:{}", signed.account_id),
            owner: owner_id,
            session_token: signed.bearer.clone(),
            email: Some(signed.email.clone()),
        })
        .map_err(SignInError::Retention)?;
    let _ = owner.save_account_state(&fence, &state);
    let _ = owner.save_appearance(&fence, signed.appearance);
    Ok(ActiveSession {
        bearer: signed.bearer,
        fence,
        email: Some(signed.email),
        appearance: signed.appearance,
        account_state: Some(state),
    })
}

async fn restore_supplied(
    base: &str,
    supplied: String,
    owner: &RetentionOwner,
    current: impl Fn() -> bool,
) -> Result<Option<ActiveSession>, SignInError> {
    let api = DuetClient::new(base, &supplied, "0", "0.1.0").map_err(SignInError::Client)?;
    let state = api.ensure_account().await.map_err(SignInError::Account)?;
    let context = api.account_context().await.map_err(SignInError::Context)?;
    if !current() {
        return Err(SignInError::Superseded);
    }
    Ok(Some(ActiveSession {
        bearer: supplied,
        fence: owner.activate_ephemeral(context.owner()),
        email: None,
        appearance: Appearance::System,
        account_state: Some(state),
    }))
}

async fn validate_retained(
    base: &str,
    bearer: &str,
    owner: &RetentionOwner,
    fence: &duet_client::session::SessionFence,
    startup_link: Option<&AppDeepLink>,
    current: impl Fn() -> bool,
) -> Result<
    (
        duet_wire::models::AccountState,
        duet_client::identity::AccountProfile,
    ),
    SignInError,
> {
    let api = DuetClient::new(base, bearer, "0", "0.1.0").map_err(SignInError::Client)?;
    let context = api.account_context().await.map_err(SignInError::Context)?;
    if !current() || !owner.session().is_current(fence) || context.owner() != fence.owner() {
        return Err(SignInError::Superseded);
    }
    if let Some(AppDeepLink::EmailChange { token }) = startup_link {
        let _ = api
            .confirm_email_change(None, &ChallengeCredential::LinkToken(token.clone()))
            .await;
    }
    let state = api.account_state().await.map_err(SignInError::Account)?;
    let profile = api
        .account_profile()
        .await
        .unwrap_or(duet_client::identity::AccountProfile {
            email: None,
            appearance: Appearance::System,
        });
    Ok((state, profile))
}

#[cfg(target_arch = "wasm32")]
fn browser_deep_link() -> Option<AppDeepLink> {
    let href = web_sys::window()?.location().href().ok()?;
    parse_app_link(&href)
}

#[cfg(not(target_arch = "wasm32"))]
fn browser_deep_link() -> Option<AppDeepLink> {
    None
}

#[component]
fn AccountRoutes(
    base_url: String,
    bearer: String,
    appearance: Appearance,
    retention: RetentionOwner,
    fence: duet_client::session::SessionFence,
    pending_invite: Option<String>,
    on_open_settings: EventHandler<()>,
    on_clear_invite: EventHandler<()>,
) -> Element {
    let mut access_token = use_signal(|| bearer.clone());
    let fresh = bearer.clone();
    let credential_session = retention.session();
    let credential_fence = fence.clone();
    use_effect(use_reactive(&fresh, move |value| {
        if *access_token.peek() != value {
            credential_session.rotate_credential(&credential_fence);
            access_token.set(value);
        }
    }));
    use_context_provider(|| PatientSession {
        retention,
        fence,
        access_token,
    });
    rsx! {
        crate::PrivateRoot {
            base_url,
            bearer,
            appearance,
            pending_invite,
            on_open_settings,
            on_clear_invite,
        }
    }
}

/// Account-shell transition for the offline doctor draft.
#[derive(Clone, Debug, PartialEq, Eq)]
enum DraftTransition<T> {
    Recover { owner: String, draft: T },
    Activate { owner: String },
    SignOut,
}

fn step_offline_draft<T>(
    _current: Option<(String, T)>,
    event: DraftTransition<T>,
) -> Option<(String, T)> {
    match event {
        DraftTransition::Recover { owner, draft } => Some((owner, draft)),
        DraftTransition::Activate { owner: _ } | DraftTransition::SignOut => None,
    }
}

fn shown_offline_draft<T>(
    current: Option<(String, T)>,
    signed_out: bool,
    viewer: Option<&str>,
) -> Option<T> {
    let (owner, draft) = current?;
    (signed_out && viewer == Some(owner.as_str())).then_some(draft)
}

#[derive(Clone, Copy)]
struct AccountUi {
    lifecycle: Signal<u64>,
    signed_out: Signal<bool>,
    active: Signal<Option<ActiveSession>>,
    offline_draft: Signal<Option<(String, duet_client::retention::DoctorDraft)>>,
    busy: Signal<bool>,
    cleanup_pending: Signal<bool>,
    message: Signal<String>,
    confirm: Signal<bool>,
    settings_open: Signal<bool>,
    upgrade_required: Signal<Option<String>>,
    agreed_terms: Signal<bool>,
    terms_error: Signal<Option<String>>,
    sign_in_step: Signal<EmailSignInStep>,
    pending_invite: Signal<Option<String>>,
}

impl AccountUi {
    fn restored(
        mut self,
        owner: &RetentionOwner,
        result: Result<Option<ActiveSession>, SignInError>,
    ) {
        match result {
            Ok(Some(value)) => {
                self.offline_draft.set(None);
                self.reset_session_ui();
                self.active.set(Some(value));
                self.message.set(String::new());
            }
            Ok(None) => self.message.set(String::new()),
            Err(
                SignInError::Account(IdentityError::Ended)
                | SignInError::Context(IdentityError::Ended),
            ) => self.ended(owner),
            Err(error) => self.message.set(error.to_string()),
        }
        self.busy.set(false);
    }

    fn reset_session_ui(mut self) {
        self.confirm.set(false);
        self.settings_open.set(false);
        self.upgrade_required.set(None);
        self.agreed_terms.set(false);
        self.terms_error.set(None);
        self.sign_in_step.set(EmailSignInStep::fresh());
    }

    fn close(mut self) {
        self.reset_session_ui();
        self.pending_invite.set(None);
        self.lifecycle.set((self.lifecycle)() + 1);
        self.signed_out.set(true);
        self.active.set(None);
        self.offline_draft.set(step_offline_draft(
            (self.offline_draft)(),
            DraftTransition::SignOut,
        ));
        self.busy.set(false);
    }

    fn erase(mut self, owner: &RetentionOwner) -> Result<(), String> {
        self.close();
        let result = owner.sign_out();
        self.cleanup_pending.set(result.is_err());
        result
    }

    fn ended(mut self, owner: &RetentionOwner) {
        let leaving = owner.account_leaving().unwrap_or_default();
        let result = self.erase(owner);
        self.message.set(match result {
            Ok(()) => leaving.removed(),
            Err(error) => format!("This session ended. Secure cleanup needs retry: {error}"),
        });
    }
}

fn start_device_sign_out(
    base: &str,
    owner: &RetentionOwner,
    active: Signal<Option<ActiveSession>>,
    mut ui: AccountUi,
    warn: bool,
) {
    if warn
        && !(ui.confirm)()
        && let Ok(leaving) = owner.account_leaving()
        && !leaving.is_empty()
    {
        ui.confirm.set(true);
        ui.message.set(leaving.warning());
        return;
    }
    let owner = owner.clone();
    let base = base.to_owned();
    let bearer = active.peek().as_ref().map(|cur| cur.bearer.clone());
    spawn(async move {
        if let Some(bearer) = bearer
            && let Ok(api) = DuetClient::new(&base, &bearer, "0", "0.1.0")
        {
            let _ = api.sign_out_device().await;
        }
        apply_sign_out(&owner, ui, false);
    });
}

fn apply_sign_out(logout_owner: &RetentionOwner, mut ui: AccountUi, warn: bool) {
    if warn
        && !(ui.confirm)()
        && let Ok(leaving) = logout_owner.account_leaving()
        && !leaving.is_empty()
    {
        ui.confirm.set(true);
        ui.message.set(leaving.warning());
        return;
    }
    let result = ui.erase(logout_owner);
    ui.message.set(match result {
        Ok(()) => String::new(),
        Err(error) => format!("Signed out on this screen. Secure cleanup needs retry: {error}"),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use duet_client::retention::{
        DoctorDraft, DraftRecovery, ProtectedStore, VolatileSessionStore,
    };
    use std::sync::atomic::{AtomicBool, Ordering};

    fn startup_ui() -> AccountUi {
        AccountUi {
            lifecycle: Signal::new(0),
            signed_out: Signal::new(false),
            active: Signal::new(None),
            offline_draft: Signal::new(None),
            busy: Signal::new(true),
            cleanup_pending: Signal::new(false),
            message: Signal::new(String::new()),
            confirm: Signal::new(false),
            settings_open: Signal::new(false),
            upgrade_required: Signal::new(None),
            agreed_terms: Signal::new(false),
            terms_error: Signal::new(None),
            sign_in_step: Signal::new(EmailSignInStep::fresh()),
            pending_invite: Signal::new(None),
        }
    }

    fn retained_work(store: Arc<dyn ProtectedStore>) -> DoctorDraft {
        let owner = RetentionOwner::new(store);
        let fence = owner
            .activate(RetainedIdentity {
                subject: "synthetic-patient".into(),
                owner: "synthetic-account:incarnation".into(),
                session_token: "synthetic-session".into(),
                email: Some("ada@example.test".into()),
            })
            .unwrap();
        let draft = DoctorDraft {
            base: duet_wire::models::DoctorCollection::new(7),
            save: duet_wire::models::DoctorPageSave::default(),
            uncertain_save: true,
        };
        owner.save(&fence, draft.clone()).unwrap();
        owner
            .retain_operation(
                &fence,
                &duet_client::doctor::PendingDoctorOperation::Clear {
                    operation_id: uuid::Uuid::nil(),
                    expected_generation: 7,
                },
            )
            .unwrap();
        owner
            .write_health_outbox(&fence, "test-installation", b"synthetic queued upload")
            .unwrap();
        draft
    }

    #[test]
    fn startup_refusal_erases_retained_work_but_temporary_failure_preserves_it() {
        let dom = VirtualDom::new(|| rsx! {});
        dom.in_scope(ScopeId::ROOT, || {
            for (error, ended) in [
                (SignInError::Account(IdentityError::Ended), true),
                (SignInError::Context(IdentityError::Ended), true),
                (SignInError::Account(IdentityError::Unavailable), false),
                (SignInError::Account(IdentityError::Unauthorized), false),
                (SignInError::Context(IdentityError::Unavailable), false),
                (SignInError::Context(IdentityError::Unauthorized), false),
                (SignInError::Retention("locked".into()), false),
            ] {
                let case = format!("{error:?}");
                let store = Arc::new(VolatileSessionStore::default());
                let draft = retained_work(store.clone());
                let owner = RetentionOwner::new(store.clone());
                let ui = startup_ui();
                ui.restored(&owner, Err(error));
                assert!(!(ui.busy)(), "{case}");
                assert_eq!((ui.signed_out)(), ended, "{case}");
                assert!(!(ui.cleanup_pending)(), "{case}");
                let restarted = RetentionOwner::new(store);
                if ended {
                    assert!(restarted.retained_identity().unwrap().is_none(), "{case}");
                    assert!(restarted.recover_offline().is_err(), "{case}");
                    assert!((ui.message)().contains("unsaved doctor draft"), "{case}");
                    assert!((ui.message)().contains("1 unsent health upload"), "{case}");
                } else {
                    assert_eq!(
                        restarted.recover_offline().unwrap(),
                        DraftRecovery::Draft(Box::new(draft)),
                        "{case}"
                    );
                    let fence = restarted.session().fence().unwrap();
                    assert!(
                        restarted.pending_operation(&fence).unwrap().is_some(),
                        "{case}"
                    );
                    assert_eq!(
                        restarted
                            .read_health_outbox(&fence, "test-installation")
                            .unwrap(),
                        Some(b"synthetic queued upload".to_vec()),
                        "{case}"
                    );
                }
            }
        });
    }

    #[derive(Default)]
    struct EraseFailure {
        store: VolatileSessionStore,
        fail: AtomicBool,
    }
    impl ProtectedStore for EraseFailure {
        fn read(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
            self.store.read(key)
        }
        fn write(&self, key: &str, value: &[u8]) -> Result<(), String> {
            self.store.write(key, value)
        }
        fn remove(&self, key: &str) -> Result<(), String> {
            if self.fail.load(Ordering::SeqCst) {
                return Err("synthetic erase failure".into());
            }
            self.store.remove(key)
        }
    }

    #[test]
    fn startup_refusal_blocks_recovery_across_restart_until_cleanup_retry_succeeds() {
        let dom = VirtualDom::new(|| rsx! {});
        dom.in_scope(ScopeId::ROOT, || {
            let store = Arc::new(EraseFailure::default());
            retained_work(store.clone());
            store.fail.store(true, Ordering::SeqCst);
            let owner = RetentionOwner::new(store.clone());
            let ui = startup_ui();
            ui.restored(&owner, Err(SignInError::Account(IdentityError::Ended)));
            assert!((ui.signed_out)());
            assert!((ui.cleanup_pending)());
            assert!((ui.message)().contains("Secure cleanup needs retry"));
            let restarted = RetentionOwner::new(store.clone());
            assert!(restarted.cleanup_required().unwrap());
            assert!(restarted.recover_offline().is_err());
            store.fail.store(false, Ordering::SeqCst);
            apply_sign_out(&restarted, ui, false);
            assert!(!(ui.cleanup_pending)());
            assert!(
                RetentionOwner::new(store)
                    .retained_identity()
                    .unwrap()
                    .is_none()
            );
        });
    }

    #[test]
    fn parse_app_link_accepts_invite_sign_in_and_email_change_links() {
        assert_eq!(
            parse_app_link("duet://invite?token=inv_123"),
            Some(AppDeepLink::Invite(InviteHandoff::Token("inv_123".into())))
        );
        assert_eq!(
            parse_app_link("https://codahealth.ai/invite?code=CODE99"),
            Some(AppDeepLink::Invite(InviteHandoff::ShortCode(
                "CODE99".into()
            )))
        );
        assert_eq!(
            parse_app_link("https://codahealth.ai/auth/email?token=dl1.tok&invite=inv_1"),
            Some(AppDeepLink::EmailSignIn {
                token: "dl1.tok".into(),
                invite: Some("inv_1".into()),
            })
        );
        assert_eq!(
            parse_app_link("duet://auth/email-change?token=dl1.chg"),
            Some(AppDeepLink::EmailChange {
                token: "dl1.chg".into(),
            })
        );
        assert_eq!(parse_app_link("duet://invite?token=bad/param"), None);
    }

    #[test]
    fn recovered_draft_is_empty_after_another_account_signs_in() {
        let mut draft = None;
        draft = step_offline_draft(
            draft,
            DraftTransition::Recover {
                owner: "alice".into(),
                draft: "alice-note",
            },
        );
        assert_eq!(
            shown_offline_draft(draft.clone(), true, Some("alice")),
            Some("alice-note")
        );
        draft = step_offline_draft(
            draft,
            DraftTransition::Activate {
                owner: "bob".into(),
            },
        );
        assert!(draft.is_none());
        assert!(shown_offline_draft(draft, true, Some("bob")).is_none());
    }

    #[test]
    fn close_resets_session_ui_state_between_accounts() {
        let dom = VirtualDom::new(|| rsx! {});
        dom.in_scope(ScopeId::ROOT, || {
            let mut ui = startup_ui();
            ui.settings_open.set(true);
            ui.agreed_terms.set(true);
            ui.terms_error.set(Some("old error".into()));
            ui.upgrade_required.set(Some("old upgrade".into()));
            ui.pending_invite.set(Some("inv-alice".into()));
            ui.sign_in_step.set(EmailSignInStep::CheckEmail {
                email: "alice@example.test".into(),
                code: "123456".into(),
                invite: "inv-alice".into(),
                submitting: true,
                status: "Check your email".into(),
                error: Some("bad code".into()),
            });

            ui.close();

            assert!(!(ui.settings_open)());
            assert!(!(ui.agreed_terms)());
            assert!((ui.terms_error)().is_none());
            assert!((ui.upgrade_required)().is_none());
            assert!((ui.pending_invite)().is_none());
            assert_eq!((ui.sign_in_step)(), EmailSignInStep::fresh());
        });
    }
}
