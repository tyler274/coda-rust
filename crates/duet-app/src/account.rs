//! One account lifecycle above every private route. Keyed remount drops old UI/tasks.
use dioxus::prelude::*;
use duet_client::{
    DuetClient,
    identity::{IdentityError, IdentityPlatformClient, ProviderCredential},
    retention::{RetainedIdentity, RetentionOwner},
};
use std::sync::Arc;
use ui::{ActionRow, Button, ButtonVariant};

/// Account-scoped owner available to editor controls and delayed-response checks.
#[derive(Clone)]
pub struct PatientSession {
    /// Protected draft owner and request fences.
    pub retention: RetentionOwner,
    /// Captured before dispatch; check before applying responses.
    pub fence: duet_client::session::SessionFence,
    /// Current ID token; never persisted. New requests observe rotations; no mutation is replayed.
    pub access_token: Signal<String>,
}
#[derive(Clone)]
pub(crate) struct RefusalSender(
    pub futures_channel::mpsc::UnboundedSender<duet_client::session::SessionFence>,
);

#[derive(Clone)]
struct Active {
    bearer: String,
    fence: duet_client::session::SessionFence,
    expires_at: Option<std::time::Instant>,
}
fn provider() -> IdentityPlatformClient {
    IdentityPlatformClient::new(
        option_env!("DUET_IDENTITY_PLATFORM_API_KEY")
            .unwrap_or_default()
            .into(),
        option_env!("DUET_IOS_BUNDLE_ID").map(str::to_owned),
    )
}
/// Where Apple returns an authorization, web or native: an HTTPS URL fixed at build time.
fn apple_redirect_uri() -> Result<&'static str, SignInError> {
    https_redirect_uri(option_env!("DUET_APPLE_REDIRECT_URI"))
}

fn https_redirect_uri(uri: Option<&str>) -> Result<&str, SignInError> {
    let uri = uri.ok_or(SignInError::NotConfigured)?;
    let parsed = url::Url::parse(uri).map_err(SignInError::RedirectUri)?;
    if parsed.scheme() != "https" {
        return Err(SignInError::RedirectNotHttps);
    }
    Ok(uri)
}

/// Why signing in produced no account session. Each case keeps what went
/// wrong; its `Display` is what the sign-in screen shows.
#[derive(Debug)]
enum SignInError {
    /// The build has no Apple Services ID or return URL.
    NotConfigured,
    /// The build's Apple return URL is not a URL.
    RedirectUri(url::ParseError),
    /// The build's Apple return URL is not HTTPS.
    RedirectNotHttps,
    /// Apple's web authorization produced no credential.
    #[cfg(target_arch = "wasm32")]
    Web(crate::browser_identity::AppleWebError),
    /// This host cannot start native Sign in with Apple.
    #[cfg(not(target_arch = "wasm32"))]
    NativeUnavailable(IdentityError),
    /// Native Sign in with Apple was cancelled or failed on the device.
    #[cfg(not(target_arch = "wasm32"))]
    Native(IdentityError),
    /// Native Sign in with Apple gave no answer in time.
    #[cfg(not(target_arch = "wasm32"))]
    TimedOut,
    /// The identity provider did not accept Apple's credential.
    Exchange(IdentityError),
    /// No client for the account service could be built.
    Client(String),
    /// Duet did not accept the account.
    Account(IdentityError),
    /// Duet could not read the account's context.
    Context(IdentityError),
    /// The protected store could not hold the new identity; its own words.
    Retention(String),
    /// A newer sign-in or sign-out started first.
    Superseded,
}

impl std::fmt::Display for SignInError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotConfigured | Self::RedirectUri(_) | Self::RedirectNotHttps => {
                f.write_str("Sign in with Apple is not set up for this build of Duet.")
            }
            #[cfg(target_arch = "wasm32")]
            Self::Web(crate::browser_identity::AppleWebError::Script(_)) => {
                f.write_str("Sign in with Apple did not complete. Try again.")
            }
            #[cfg(target_arch = "wasm32")]
            Self::Web(_) => f.write_str("Invalid Apple response."),
            #[cfg(not(target_arch = "wasm32"))]
            Self::NativeUnavailable(_) => {
                f.write_str("Native Sign in with Apple is not available in this host.")
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::Native(_) => f.write_str("Sign-in cancelled or unavailable."),
            #[cfg(not(target_arch = "wasm32"))]
            Self::TimedOut => f.write_str("Sign-in timed out. Try again."),
            Self::Exchange(_) => f.write_str("Apple sign-in could not be verified."),
            Self::Client(cause) => write!(f, "Duet is unavailable: {cause}"),
            Self::Account(_) => {
                f.write_str("Duet could not verify your account. Your saved draft is unchanged.")
            }
            Self::Context(_) => f.write_str("Duet could not read your account context."),
            Self::Retention(words) => f.write_str(words),
            Self::Superseded => f.write_str("Account changed; sign-in ignored."),
        }
    }
}

impl std::error::Error for SignInError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(target_arch = "wasm32")]
            Self::Web(error) => Some(error),
            #[cfg(not(target_arch = "wasm32"))]
            Self::NativeUnavailable(error) | Self::Native(error) => Some(error),
            Self::Exchange(error) | Self::Account(error) | Self::Context(error) => Some(error),
            Self::RedirectUri(error) => Some(error),
            _ => None,
        }
    }
}

async fn verify(
    base: &str,
    credential: ProviderCredential,
    owner: &RetentionOwner,
    current: impl Fn() -> bool,
) -> Result<Active, SignInError> {
    let client =
        DuetClient::new(base, &credential.id_token, "0", "0.1.0").map_err(SignInError::Client)?;
    client
        .ensure_account()
        .await
        .map_err(SignInError::Account)?;
    let context = client
        .account_context()
        .await
        .map_err(SignInError::Context)?;
    if !current() {
        return Err(SignInError::Superseded);
    }
    let fence = owner
        .activate(RetainedIdentity {
            subject: credential.subject,
            owner: context.owner(),
            refresh_token: credential.refresh_token,
        })
        .map_err(SignInError::Retention)?;
    Ok(Active {
        bearer: credential.id_token,
        fence,
        expires_at: Some(
            std::time::Instant::now() + std::time::Duration::from_secs(credential.expires_in),
        ),
    })
}

/// Standalone verified Identity Platform entry. Practice linking is optional, never a login gate.
#[component]
pub fn DuetRoot(base_url: String, bearer: String) -> Element {
    let retention = use_hook(|| {
        #[cfg(target_arch = "wasm32")]
        {
            RetentionOwner::new(Arc::new(
                duet_client::retention::VolatileSessionStore::default(),
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            RetentionOwner::new(Arc::new(duet_client::retention::NativeProtectedStore))
        }
    });
    let drop_session = retention.session();
    use_drop(move || {
        drop_session.switch(None);
    });
    let mut offline_draft = use_signal(|| None::<(String, duet_client::retention::DoctorDraft)>);
    let mut active = use_signal(|| None::<Active>);
    let mut message = use_signal(String::new);
    let mut busy = use_signal(|| true);
    let mut retry = use_signal(|| 0u64);
    let mut signed_out = use_signal(|| false);
    let mut cleanup_pending = use_signal(|| false);
    let mut lifecycle = use_signal(|| 0u64);
    let (refusal_sender, refusal_receiver) = use_hook(|| {
        let (sender, receiver) = futures_channel::mpsc::unbounded();
        (
            sender,
            std::rc::Rc::new(std::cell::RefCell::new(Some(receiver))),
        )
    });
    use_context_provider(|| RefusalSender(refusal_sender));
    use_future(move || {
        let receiver = refusal_receiver.borrow_mut().take();
        async move {
            use futures_util::StreamExt;
            let Some(mut receiver) = receiver else {
                return;
            };
            while let Some(fence) = receiver.next().await {
                if active
                    .peek()
                    .as_ref()
                    .is_none_or(|value| value.fence != fence)
                {
                    continue;
                }
                lifecycle.set(lifecycle() + 1);
                active.set(None);
                offline_draft.set(None);
                signed_out.set(true);
                busy.set(false);
                message.set("Duet could not authorize this session. Sign in again. Unsent text on this screen is no longer shown; protected drafts remain on this device. No pending operation was retried.".into());
            }
        }
    });
    let restore_owner = retention.clone();
    let restore_url = base_url.clone();
    let supplied = bearer;
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
        busy.set(true);
        spawn(async move {
            // Remove historical browser storage; never use its contents as credentials.
            let _ = crate::roots::recalled_session();
            if owner.cleanup_required().unwrap_or(true) {
                cleanup_pending.set(true);
                signed_out.set(true);
                message.set("Complete secure sign-out cleanup before opening an account.".into());
                busy.set(false);
                return;
            }
            let result = if !supplied.is_empty() {
                match DuetClient::new(&base, supplied.clone(), "0", "0.1.0") {
                    Ok(api) => match api.ensure_account().await {
                        Ok(_) => match api.account_context().await {
                            Ok(context) if lifecycle() == epoch => Ok(Active {
                                bearer: supplied,
                                fence: owner.activate_ephemeral(context.owner()),
                                expires_at: None,
                            }),
                            _ => Err("Could not verify account context. Try again.".into()),
                        },
                        Err(_) => Err("Sign in to Duet to continue.".into()),
                    },
                    Err(_) => Err("Duet is unavailable.".into()),
                }
            } else {
                match owner.retained_identity() {
                    Ok(Some(identity)) => match provider().refresh(&identity.subject, &identity.refresh_token).await {
                        Ok(credential) if lifecycle() == epoch => verify(&base, credential, &owner, || lifecycle() == epoch).await.map_err(|error| error.to_string()),
                        _ => Err("Could not restore sign-in. Your protected draft has not been discarded. Unlock your device or retry when online.".into()),
                    },
                    Ok(None) => Err(String::new()),
                    Err(error) => Err(error),
                }
            };
            if lifecycle() != epoch {
                return;
            }
            match result {
                Ok(value) => {
                    let owner = value.fence.owner().to_owned();
                    offline_draft.set(step_offline_draft(
                        offline_draft(),
                        DraftTransition::Activate { owner },
                    ));
                    active.set(Some(value));
                    message.set(String::new());
                }
                Err(error) => message.set(error),
            }
            busy.set(false);
        });
    });
    // Independent expiry watchdog: a hung provider request must not keep expired routes open.
    let expiry_owner = retention.clone();
    use_future(move || {
        let owner = expiry_owner.clone();
        async move {
            loop {
                futures_timer::Delay::new(std::time::Duration::from_secs(5)).await;
                let snapshot = active.peek().clone();
                if let Some(current) = snapshot
                    && current
                        .expires_at
                        .is_some_and(|deadline| std::time::Instant::now() >= deadline)
                {
                    owner.session().switch(None);
                    active.set(None);
                    offline_draft.set(step_offline_draft(offline_draft(), DraftTransition::Expire));
                    message.set("Sign-in expired. Your draft remains retained; no pending operation was replayed.".into());
                }
            }
        }
    });
    let refresh_owner = retention.clone();
    let refresh_url = base_url.clone();
    use_future(move || {
        let owner = refresh_owner.clone();
        let base = refresh_url.clone();
        async move {
            loop {
                futures_timer::Delay::new(std::time::Duration::from_secs(30)).await;
                let Some(current) = active.peek().clone() else {
                    continue;
                };
                let Some(expiry) = current.expires_at else {
                    continue;
                };
                if expiry
                    .saturating_duration_since(std::time::Instant::now())
                    .as_secs()
                    > 120
                {
                    continue;
                }
                let identity = match owner.retained_identity() {
                    Ok(Some(i)) => i,
                    _ => {
                        if std::time::Instant::now() >= expiry {
                            owner.session().switch(None);
                            active.set(None);
                            offline_draft
                                .set(step_offline_draft(offline_draft(), DraftTransition::Expire));
                            message.set("Sign-in expired while secure storage was unavailable. Your draft remains protected.".into());
                        }
                        continue;
                    }
                };
                let result = provider()
                    .refresh(&identity.subject, &identity.refresh_token)
                    .await;
                if !owner.session().is_current(&current.fence) {
                    continue;
                }
                match result {
                    Ok(token) => {
                        let context = match DuetClient::new(&base, &token.id_token, "0", "0.1.0") {
                            Ok(api) => api.account_context().await,
                            Err(_) => Err(duet_client::identity::IdentityError::Unavailable),
                        };
                        if !owner.session().is_current(&current.fence) {
                            continue;
                        }
                        if matches!(context, Err(IdentityError::Unauthorized)) {
                            if owner.session().invalidate(&current.fence) {
                                active.set(None);
                                offline_draft.set(None);
                                signed_out.set(true);
                                lifecycle.set(lifecycle() + 1);
                                message.set("Duet could not authorize this session. Sign in again. Protected drafts remain on this device; no pending operation was retried.".into());
                            }
                            continue;
                        }
                        if let Ok(context) = context
                            && context.owner() == current.fence.owner()
                            && owner
                                .rotate_credential(
                                    &current.fence,
                                    &token.subject,
                                    token.refresh_token,
                                )
                                .is_ok()
                        {
                            active.set(Some(Active {
                                bearer: token.id_token,
                                fence: current.fence,
                                expires_at: Some(
                                    std::time::Instant::now()
                                        + std::time::Duration::from_secs(token.expires_in),
                                ),
                            }));
                            continue;
                        }
                    }
                    Err(duet_client::identity::IdentityError::Expired) => {
                        owner.session().switch(None);
                        active.set(None);
                        offline_draft
                            .set(step_offline_draft(offline_draft(), DraftTransition::Expire));
                        message.set("Your sign-in expired or was revoked. Sign in again; your draft is retained. No pending save was replayed.".into());
                        continue;
                    }
                    Err(_) => (),
                }
                if std::time::Instant::now() >= expiry {
                    owner.session().switch(None);
                    active.set(None);
                    offline_draft.set(step_offline_draft(offline_draft(), DraftTransition::Expire));
                    message.set("Sign-in could not be refreshed. Reconnect or sign in again; uncertain saves were not replayed.".into());
                }
            }
        }
    });
    let logout_owner = retention.clone();
    let sign_out = move |_| {
        lifecycle.set(lifecycle() + 1);
        signed_out.set(true);
        active.set(None); // Unmount every private component before disk cleanup.
        offline_draft.set(step_offline_draft(
            offline_draft(),
            DraftTransition::SignOut,
        ));
        busy.set(false);
        #[cfg(target_arch = "wasm32")]
        let cleanup = logout_owner.sign_out();
        #[cfg(not(target_arch = "wasm32"))]
        let cleanup = logout_owner.sign_out();
        match cleanup {
            Ok(()) => {
                cleanup_pending.set(false);
                message.set(String::new());
            }
            Err(error) => {
                cleanup_pending.set(true);
                message.set(format!(
                    "Signed out on this screen. Secure cleanup needs retry: {error}"
                ));
            }
        }
    };
    let login_owner = retention.clone();
    let login_url = base_url.clone();
    let login = move |_| {
        if busy() || cleanup_pending() {
            return;
        }
        busy.set(true);
        lifecycle.set(lifecycle() + 1);
        let epoch = lifecycle();
        let owner = login_owner.clone();
        let base = login_url.clone();
        spawn(async move {
            #[cfg(target_arch = "wasm32")]
            let result = async {
                let redirect_uri = apple_redirect_uri()?;
                let services_id = option_env!("DUET_APPLE_SERVICES_ID")
                    .filter(|id| !id.is_empty())
                    .ok_or(SignInError::NotConfigured)?;
                let apple = crate::browser_identity::sign_in(services_id, redirect_uri)
                    .await
                    .map_err(SignInError::Web)?;
                let credential = provider()
                    .apple(apple, redirect_uri)
                    .await
                    .map_err(SignInError::Exchange)?;
                verify(&base, credential, &owner, || lifecycle() == epoch).await
            }
            .await;
            #[cfg(not(target_arch = "wasm32"))]
            let result = async {
                let redirect_uri = apple_redirect_uri()?;
                duet_client::identity::start_native_sign_in()
                    .map_err(SignInError::NativeUnavailable)?;
                for _ in 0..240 {
                    futures_timer::Delay::new(std::time::Duration::from_millis(500)).await;
                    if lifecycle() != epoch {
                        return Err(SignInError::Superseded);
                    }
                    let apple = match duet_client::identity::poll_native_sign_in() {
                        Ok(Some(apple)) => apple,
                        Ok(None) => continue,
                        Err(error) => return Err(SignInError::Native(error)),
                    };
                    let credential = provider()
                        .apple(apple, redirect_uri)
                        .await
                        .map_err(SignInError::Exchange)?;
                    if lifecycle() != epoch {
                        return Err(SignInError::Superseded);
                    }
                    return verify(&base, credential, &owner, || lifecycle() == epoch).await;
                }
                Err(SignInError::TimedOut)
            }
            .await;
            if lifecycle() != epoch {
                return;
            }
            match result {
                Ok(value) => {
                    let owner = value.fence.owner().to_owned();
                    offline_draft.set(step_offline_draft(
                        offline_draft(),
                        DraftTransition::Activate { owner },
                    ));
                    active.set(Some(value));
                    message.set(String::new());
                }
                Err(error) => message.set(error.to_string()),
            }
            busy.set(false);
        });
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
            AccountRoutes { key: "{current.fence.generation()}", base_url, bearer: current.bearer, retention: retention.clone(), fence: current.fence }
            if cfg!(target_arch = "wasm32") { p { role: "status", "Browser sign-in, drafts and operation retry identities are memory-only. Keep this tab open while any write is uncertain. After a reload, sign in and use ‘Recover or resolve a previous operation’ to read current server state before a new approval; do not assume the previous operation succeeded or recreate it as a retry." } }
            Button { variant: ButtonVariant::Quiet, compact: true, onclick: sign_out, "Sign out" }
        } else {
            main { class: "duet theme-duet",
                h1 { "Duet" }
                p { "Sign in to your Duet account. A practice invitation is not required." }
                if !message().is_empty() { p { role: "alert", "{message}" } }
                if busy() { p { "Checking sign-in…" } }
                ActionRow {
                    Button { variant: ButtonVariant::Primary, disabled: busy() || cleanup_pending(), onclick: login, "Sign in with Apple" }
                    Button { disabled: busy() || cleanup_pending(), onclick: move |_| { signed_out.set(false); retry.set(retry() + 1); }, "Retry saved sign-in" }
                    if cleanup_pending() { Button { onclick: sign_out, "Retry secure cleanup" } }
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

#[component]
fn AccountRoutes(
    base_url: String,
    bearer: String,
    retention: RetentionOwner,
    fence: duet_client::session::SessionFence,
) -> Element {
    let mut access_token = use_signal(|| bearer.clone());
    let fresh = bearer.clone();
    use_effect(use_reactive(&fresh, move |value| {
        access_token.set(value);
    }));
    use_context_provider(|| PatientSession {
        retention,
        fence,
        access_token,
    });
    rsx! { crate::PrivateRoot { base_url, bearer } }
}

/// Account-shell transition for the offline doctor draft.
///
/// Recovery remembers the owner. Activation, expiry, and sign-out drop it, so
/// a later signed-out render cannot show another person's draft.
#[derive(Clone, Debug, PartialEq, Eq)]
enum DraftTransition<T> {
    /// A signed-out recovery of this owner's protected draft.
    Recover { owner: String, draft: T },
    /// A verified sign-in, including a different person without a remount.
    Activate { owner: String },
    /// A watchdog or refresh path that clears the active session.
    Expire,
    /// An explicit sign-out.
    SignOut,
}

fn step_offline_draft<T>(
    _current: Option<(String, T)>,
    event: DraftTransition<T>,
) -> Option<(String, T)> {
    match event {
        DraftTransition::Recover { owner, draft } => Some((owner, draft)),
        DraftTransition::Activate { owner: _ }
        | DraftTransition::Expire
        | DraftTransition::SignOut => None,
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

#[cfg(test)]
mod tests {
    use super::{
        DraftTransition, SignInError, https_redirect_uri, shown_offline_draft, step_offline_draft,
    };

    #[test]
    fn only_a_parsed_https_return_url_lets_apple_sign_in_start() {
        assert!(matches!(
            https_redirect_uri(None),
            Err(SignInError::NotConfigured)
        ));
        for malformed in ["https://", "https://[", "https://exa mple.com", ""] {
            assert!(
                matches!(
                    https_redirect_uri(Some(malformed)),
                    Err(SignInError::RedirectUri(_))
                ),
                "{malformed:?} was accepted"
            );
        }
        assert!(matches!(
            https_redirect_uri(Some("http://duet.example.test/apple")),
            Err(SignInError::RedirectNotHttps)
        ));
        assert_eq!(
            https_redirect_uri(Some("https://duet.example.test/apple")).unwrap(),
            "https://duet.example.test/apple"
        );
    }

    #[test]
    fn recovered_draft_is_empty_after_another_account_signs_in_and_expires() {
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
        // Sign in as someone else without remounting the account shell.
        draft = step_offline_draft(
            draft,
            DraftTransition::Activate {
                owner: "bob".into(),
            },
        );
        assert!(draft.is_none());
        draft = step_offline_draft(draft, DraftTransition::Expire);
        assert!(draft.is_none());
        assert!(shown_offline_draft(draft, true, Some("bob")).is_none());
    }
}
