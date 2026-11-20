//! Alert permission, device push-token registration, generic lock-screen alerts,
//! and sign-out alert cleanup.
//! Only Duet crates import this crate.
//! The Swift binding is `platform/ios/NotificationBridge.swift`.
//! The Kotlin binding is `platform/android/NotificationBridge.kt`.
//! A host or web build has no native notification center.
//! Book: `book/src/backend-architecture.md`.
#![deny(missing_docs)]

use std::fmt;

/// Maximum accepted byte length of a native push registration token.
pub const MAX_PUSH_TOKEN_BYTES: usize = 4096;

/// Whether this build may show a notification alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertState {
    /// The person has not been asked.
    NotDetermined,
    /// Alerts are allowed.
    Allowed,
    /// The person turned alerts off.
    Denied,
    /// The notification center could not be read.
    Unreadable,
    /// This build has no notification center.
    NoCenter,
}

impl AlertState {
    /// Returns the sentence shown for this state.
    pub fn sentence(self) -> &'static str {
        match self {
            Self::NotDetermined => "Duet has not asked to send notifications.",
            Self::Allowed => "Notifications are allowed.",
            Self::Denied => "Notifications are off for Duet.",
            Self::Unreadable => "Notifications could not be read.",
            Self::NoCenter => "This build has no notification center.",
        }
    }

    /// Reports whether the person can be asked for alerts.
    pub fn can_request(self) -> bool {
        matches!(self, Self::NotDetermined)
    }

    /// Reports whether You shows this state.
    pub fn shown(self) -> bool {
        !matches!(self, Self::NoCenter)
    }
}

/// The notification permission You can show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlertSnapshot {
    /// The `state` value.
    pub state: AlertState,
}

impl AlertSnapshot {
    /// Returns the sentence shown for this state.
    pub fn sentence(&self) -> &'static str {
        self.state.sentence()
    }
}

/// Native remote push delivery channel for a signed-in device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PushChannel {
    /// Apple Push Notification service on iOS.
    Apns,
    /// Firebase Cloud Messaging on Android.
    Fcm,
}

impl PushChannel {
    /// Wire identifier shared with `duet-api`.
    pub const fn as_wire(self) -> &'static str {
        match self {
            Self::Apns => "apns",
            Self::Fcm => "fcm",
        }
    }

    /// Parses a wire identifier into a typed push channel.
    pub fn from_wire(value: &str) -> Option<Self> {
        match value.trim() {
            "apns" => Some(Self::Apns),
            "fcm" => Some(Self::Fcm),
            _ => None,
        }
    }
}

/// Validation failure when constructing a [`PushDeviceToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PushTokenError {
    /// The token was empty or whitespace-only.
    Empty,
    /// The token exceeded [`MAX_PUSH_TOKEN_BYTES`].
    TooLong,
    /// The token contained whitespace or control characters.
    InvalidCharacter,
}

impl fmt::Display for PushTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("push device token must not be empty"),
            Self::TooLong => f.write_str("push device token exceeds maximum length"),
            Self::InvalidCharacter => {
                f.write_str("push device token contains invalid or control characters")
            }
        }
    }
}

impl std::error::Error for PushTokenError {}

/// Validated native push token for one signed-in device.
///
/// `Debug` is redacted so raw push tokens never appear in logs or diagnostics.
#[derive(Clone, PartialEq, Eq)]
pub struct PushDeviceToken(String);

impl PushDeviceToken {
    /// Validates and wraps a native remote push token.
    pub fn parse(raw: impl AsRef<str>) -> Result<Self, PushTokenError> {
        let trimmed = raw.as_ref().trim();
        if trimmed.is_empty() {
            return Err(PushTokenError::Empty);
        }
        if trimmed.len() > MAX_PUSH_TOKEN_BYTES {
            return Err(PushTokenError::TooLong);
        }
        if !trimmed
            .bytes()
            .all(|byte| byte.is_ascii_graphic() && byte != b' ')
        {
            return Err(PushTokenError::InvalidCharacter);
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Exposes the raw token strictly for authenticated registration transport.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PushDeviceToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PushDeviceToken")
            .field(&"<redacted>")
            .finish()
    }
}

/// Signed-in device registration payload reported by the native OS bridge.
#[derive(Clone, PartialEq, Eq)]
pub struct DevicePushRegistration {
    /// Stable per-installation device identifier scoped to this phone.
    pub device_id: String,
    /// Native push channel for this phone.
    pub channel: PushChannel,
    /// Validated remote push token.
    pub token: PushDeviceToken,
}

impl fmt::Debug for DevicePushRegistration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DevicePushRegistration")
            .field("device_id", &self.device_id)
            .field("channel", &self.channel)
            .field("token", &"<redacted>")
            .finish()
    }
}

/// Closed set of lock-screen notification categories Duet may deliver.
///
/// Every variant resolves to fixed, generic lock-screen text with zero health,
/// clinician, lab, or medication content (`intent/duet.md` What leaves Duet).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericAlertKind {
    /// A new message arrived from a connected care team or thread.
    DoctorMessage,
    /// New imported records or updates are ready inside Duet.
    NewRecord,
    /// An upcoming care reminder or schedule update is waiting in Duet.
    AppointmentReminder,
    /// A general account notice is ready inside Duet.
    AccountNotice,
}

impl GenericAlertKind {
    /// Wire identifier shared with `duet-api`.
    pub const fn as_wire(self) -> &'static str {
        match self {
            Self::DoctorMessage => "doctor_message",
            Self::NewRecord => "new_record",
            Self::AppointmentReminder => "appointment_reminder",
            Self::AccountNotice => "account_notice",
        }
    }

    /// Parses a wire identifier into a closed generic alert category.
    pub fn from_wire(value: &str) -> Option<Self> {
        match value.trim() {
            "doctor_message" => Some(Self::DoctorMessage),
            "new_record" => Some(Self::NewRecord),
            "appointment_reminder" => Some(Self::AppointmentReminder),
            "account_notice" => Some(Self::AccountNotice),
            _ => None,
        }
    }

    /// Builds the strictly generic lock-screen payload for this category.
    ///
    /// Health content never leaves the app on lock screens; full detail opens inside Duet.
    pub const fn lock_screen_alert(self) -> GenericLockScreenAlert {
        let body = match self {
            Self::DoctorMessage => "You have a new message in Duet.",
            Self::NewRecord => "New information is ready in Duet.",
            Self::AppointmentReminder => "You have an update waiting in Duet.",
            Self::AccountNotice => "Open Duet to view an update.",
        };
        GenericLockScreenAlert {
            kind: self,
            category: self.as_wire(),
            title: "Duet",
            body,
        }
    }
}

/// Strictly generic lock-screen alert payload carrying no health content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenericLockScreenAlert {
    /// Closed category of the notification.
    pub kind: GenericAlertKind,
    /// Wire category identifier.
    pub category: &'static str,
    /// Fixed lock-screen title.
    pub title: &'static str,
    /// Fixed generic lock-screen body with zero PHI.
    pub body: &'static str,
}

/// Outcome of clearing local notification state on sign-out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignOutCleanup {
    /// Cleared the cached device push token and removed pending and delivered local alerts.
    ClearedTokenAndAlerts,
    /// No cached device push token was present; removed pending and delivered local alerts.
    ClearedAlertsOnly,
    /// No native notification center was available or local cleanup could not run.
    Unavailable,
}

impl SignOutCleanup {
    /// Reports whether any locally cached device push token was cleared.
    pub const fn cleared_token(self) -> bool {
        matches!(self, Self::ClearedTokenAndAlerts)
    }

    /// Reports whether local pending and delivered alerts were cleared.
    pub const fn cleared_alerts(self) -> bool {
        matches!(self, Self::ClearedTokenAndAlerts | Self::ClearedAlertsOnly)
    }
}

/// Reads and requests notification alerts, push registration tokens, and sign-out
/// cleanup on this phone.
#[derive(Debug, Default, Clone, Copy)]
pub struct PlatformAlerts;

impl PlatformAlerts {
    /// Reads the current alert permission.
    pub fn snapshot(&self) -> AlertSnapshot {
        #[cfg(target_os = "ios")]
        {
            ios_snapshot()
        }
        #[cfg(target_os = "android")]
        {
            android_snapshot()
        }
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            AlertSnapshot {
                state: AlertState::NoCenter,
            }
        }
    }

    /// Asks the operating system for alert permission.
    pub fn request_access(&self) {
        #[cfg(target_os = "ios")]
        ios_request();
        #[cfg(target_os = "android")]
        android_request();
    }

    /// Opens the system Settings page for this app.
    pub fn open_settings(&self) {
        #[cfg(target_os = "ios")]
        ios_open_settings();
        #[cfg(target_os = "android")]
        android_open_settings();
    }

    /// Reads the signed-in device's current push registration, when available.
    pub fn push_registration(&self, device_id: &str) -> Option<DevicePushRegistration> {
        let device_id = device_id.trim();
        if device_id.is_empty() {
            return None;
        }
        #[cfg(target_os = "ios")]
        {
            ios_push_token()
                .and_then(|raw| parse_bridge_registration(device_id, PushChannel::Apns, &raw))
        }
        #[cfg(target_os = "android")]
        {
            android_push_token()
                .and_then(|raw| parse_bridge_registration(device_id, PushChannel::Fcm, &raw))
        }
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            None
        }
    }

    /// Clears the local remote push token and removes pending and delivered alerts on sign-out.
    pub fn clear_on_sign_out(&self) -> SignOutCleanup {
        #[cfg(target_os = "ios")]
        {
            ios_clear_on_sign_out()
        }
        #[cfg(target_os = "android")]
        {
            android_clear_on_sign_out()
        }
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            SignOutCleanup::Unavailable
        }
    }
}

/// Maps a notification status code onto the alert snapshot.
/// An unknown or empty code is unreadable, not a denial.
#[cfg(any(test, target_os = "ios", target_os = "android"))]
fn alert_snapshot(code: &str) -> AlertSnapshot {
    let state = match code {
        "not_determined" => AlertState::NotDetermined,
        "allowed" => AlertState::Allowed,
        "denied" => AlertState::Denied,
        _ => AlertState::Unreadable,
    };
    AlertSnapshot { state }
}

#[cfg(any(test, target_os = "ios", target_os = "android"))]
fn parse_bridge_registration(
    device_id: &str,
    channel: PushChannel,
    raw_token: &str,
) -> Option<DevicePushRegistration> {
    let device_id = device_id.trim();
    if device_id.is_empty() {
        return None;
    }
    let token = PushDeviceToken::parse(raw_token).ok()?;
    Some(DevicePushRegistration {
        device_id: device_id.to_owned(),
        channel,
        token,
    })
}

#[cfg(any(test, target_os = "ios", target_os = "android"))]
fn parse_cleanup_code(code: &str) -> SignOutCleanup {
    match code.trim() {
        "cleared_all" => SignOutCleanup::ClearedTokenAndAlerts,
        "cleared_alerts_only" => SignOutCleanup::ClearedAlertsOnly,
        _ => SignOutCleanup::Unavailable,
    }
}

#[cfg(target_os = "ios")]
fn ios_snapshot() -> AlertSnapshot {
    unsafe extern "C" {
        fn duet_notify_status() -> *mut std::ffi::c_char;
        fn duet_notify_string_free(pointer: *mut std::ffi::c_char);
    }
    match bridge_string(unsafe { duet_notify_status() }, duet_notify_string_free) {
        Some(code) => alert_snapshot(&code),
        None => AlertSnapshot {
            state: AlertState::Unreadable,
        },
    }
}

#[cfg(target_os = "ios")]
fn ios_request() {
    unsafe extern "C" {
        fn duet_notify_request();
    }
    unsafe { duet_notify_request() }
}

#[cfg(target_os = "ios")]
fn ios_open_settings() {
    unsafe extern "C" {
        fn duet_notify_open_settings();
    }
    unsafe { duet_notify_open_settings() }
}

#[cfg(target_os = "ios")]
fn ios_push_token() -> Option<String> {
    unsafe extern "C" {
        fn duet_notify_push_token() -> *mut std::ffi::c_char;
        fn duet_notify_string_free(pointer: *mut std::ffi::c_char);
    }
    bridge_string(unsafe { duet_notify_push_token() }, duet_notify_string_free)
}

#[cfg(target_os = "ios")]
fn ios_clear_on_sign_out() -> SignOutCleanup {
    unsafe extern "C" {
        fn duet_notify_clear_on_sign_out() -> *mut std::ffi::c_char;
        fn duet_notify_string_free(pointer: *mut std::ffi::c_char);
    }
    match bridge_string(
        unsafe { duet_notify_clear_on_sign_out() },
        duet_notify_string_free,
    ) {
        Some(code) => parse_cleanup_code(&code),
        None => SignOutCleanup::Unavailable,
    }
}

#[cfg(target_os = "ios")]
fn bridge_string(
    pointer: *mut std::ffi::c_char,
    free_string: unsafe extern "C" fn(*mut std::ffi::c_char),
) -> Option<String> {
    if pointer.is_null() {
        return None;
    }
    let text = unsafe { std::ffi::CStr::from_ptr(pointer) }
        .to_str()
        .ok()
        .map(str::to_owned);
    unsafe { free_string(pointer) };
    text
}

#[cfg(target_os = "android")]
fn android_snapshot() -> AlertSnapshot {
    match android_call(jni::jni_str!("status"), BridgeReturn::String) {
        Ok(code) => alert_snapshot(&code),
        Err(_) => AlertSnapshot {
            state: AlertState::Unreadable,
        },
    }
}

#[cfg(target_os = "android")]
fn android_request() {
    let _ = android_call(jni::jni_str!("requestPermission"), BridgeReturn::Void);
}

#[cfg(target_os = "android")]
fn android_open_settings() {
    let _ = android_call(jni::jni_str!("openSettings"), BridgeReturn::Void);
}

#[cfg(target_os = "android")]
fn android_push_token() -> Option<String> {
    android_call(jni::jni_str!("pushToken"), BridgeReturn::String)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

#[cfg(target_os = "android")]
fn android_clear_on_sign_out() -> SignOutCleanup {
    match android_call(jni::jni_str!("clearOnSignOut"), BridgeReturn::String) {
        Ok(code) => parse_cleanup_code(&code),
        Err(_) => SignOutCleanup::Unavailable,
    }
}

#[cfg(target_os = "android")]
enum BridgeReturn {
    Void,
    String,
}

#[cfg(target_os = "android")]
fn android_call(method: &jni::strings::JNIStr, ret: BridgeReturn) -> Result<String, String> {
    use jni::objects::{JClass, JObject, JString, JValue};
    use jni::{jni_sig, jni_str};

    let ctx = ndk_context::android_context();
    let vm_ptr = ctx.vm();
    let context_ptr = ctx.context();
    if vm_ptr.is_null() || context_ptr.is_null() {
        return Err("Android context unavailable".into());
    }
    let vm = unsafe { jni::JavaVM::from_raw(vm_ptr.cast()) };
    vm.attach_current_thread(|env| -> jni::errors::Result<String> {
        let context = unsafe { JObject::from_raw(env, context_ptr.cast()) };
        let activity_class = env.get_object_class(&context)?;
        let loader = env
            .call_method(
                activity_class,
                jni_str!("getClassLoader"),
                jni_sig!("()Ljava/lang/ClassLoader;"),
                &[],
            )?
            .l()?;
        let class_name = env.new_string("coda.duet.NotificationBridge")?;
        let bridge = env
            .call_method(
                loader,
                jni_str!("loadClass"),
                jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let bridge = env.cast_local::<JClass>(bridge)?;
        match ret {
            BridgeReturn::Void => {
                env.call_static_method(
                    bridge,
                    method,
                    jni_sig!("(Landroid/content/Context;)V"),
                    &[JValue::Object(&context)],
                )?;
                Ok(String::new())
            }
            BridgeReturn::String => {
                let value = env.call_static_method(
                    bridge,
                    method,
                    jni_sig!("(Landroid/content/Context;)Ljava/lang/String;"),
                    &[JValue::Object(&context)],
                )?;
                let object = value.l()?;
                if object.is_null() {
                    return Ok(String::new());
                }
                let jstring = env.cast_local::<JString>(object)?;
                jstring.try_to_string(env)
            }
        }
    })
    .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_determined_can_be_requested() {
        let snapshot = alert_snapshot("not_determined");
        assert_eq!(snapshot.state, AlertState::NotDetermined);
        assert!(snapshot.state.can_request());
        assert!(snapshot.state.shown());
        assert_eq!(
            snapshot.sentence(),
            "Duet has not asked to send notifications."
        );
    }

    #[test]
    fn allowed_hides_the_request() {
        let snapshot = alert_snapshot("allowed");
        assert_eq!(snapshot.state, AlertState::Allowed);
        assert!(!snapshot.state.can_request());
        assert!(snapshot.state.shown());
        assert_eq!(snapshot.sentence(), "Notifications are allowed.");
    }

    #[test]
    fn denied_is_not_allowed_and_cannot_be_asked_again() {
        let snapshot = alert_snapshot("denied");
        assert_eq!(snapshot.state, AlertState::Denied);
        assert!(!snapshot.state.can_request());
        assert!(snapshot.state.shown());
        assert_ne!(snapshot.state, AlertState::Allowed);
    }

    #[test]
    fn an_unknown_code_is_unreadable_not_a_denial() {
        let snapshot = alert_snapshot("");
        assert_eq!(snapshot.state, AlertState::Unreadable);
        assert!(!snapshot.state.can_request());
        assert!(snapshot.state.shown());
        assert_ne!(snapshot.state, AlertState::Denied);
    }

    #[test]
    fn a_build_without_a_center_does_not_invent_a_prompt() {
        let snapshot = PlatformAlerts.snapshot();
        assert_eq!(snapshot.state, AlertState::NoCenter);
        assert!(!snapshot.state.shown());
        assert!(!snapshot.state.can_request());
        assert!(PlatformAlerts.push_registration("phone-1").is_none());
        assert_eq!(
            PlatformAlerts.clear_on_sign_out(),
            SignOutCleanup::Unavailable
        );
    }

    #[test]
    fn push_token_validation_and_redaction() {
        assert_eq!(PushDeviceToken::parse("   "), Err(PushTokenError::Empty));
        assert_eq!(
            PushDeviceToken::parse("abc\ndef"),
            Err(PushTokenError::InvalidCharacter)
        );
        assert_eq!(
            PushDeviceToken::parse("a".repeat(MAX_PUSH_TOKEN_BYTES + 1)),
            Err(PushTokenError::TooLong)
        );

        let secret = "apns-live-device-token-secret-998877";
        let token = PushDeviceToken::parse(secret).unwrap();
        assert_eq!(token.as_str(), secret);
        assert!(!format!("{token:?}").contains(secret));

        let reg = parse_bridge_registration("iphone-1", PushChannel::Apns, secret).unwrap();
        assert_eq!(reg.device_id, "iphone-1");
        assert_eq!(reg.channel, PushChannel::Apns);
        assert!(!format!("{reg:?}").contains(secret));
        assert!(parse_bridge_registration("   ", PushChannel::Fcm, secret).is_none());
    }

    #[test]
    fn generic_lock_screen_alerts_contain_no_health_content() {
        let forbidden = [
            "a1c",
            "glucose",
            "lab",
            "doctor",
            "dr.",
            "medication",
            "diagnosis",
            "patient",
            "blood",
        ];
        for kind in [
            GenericAlertKind::DoctorMessage,
            GenericAlertKind::NewRecord,
            GenericAlertKind::AppointmentReminder,
            GenericAlertKind::AccountNotice,
        ] {
            let alert = kind.lock_screen_alert();
            assert_eq!(GenericAlertKind::from_wire(kind.as_wire()), Some(kind));
            assert_eq!(alert.title, "Duet");
            assert!(!alert.body.is_empty());
            let lower = alert.body.to_ascii_lowercase();
            for term in forbidden {
                assert!(
                    !lower.contains(term),
                    "lock-screen body for {kind:?} must stay generic, found {term:?}"
                );
            }
        }
        assert_eq!(GenericAlertKind::from_wire("unknown"), None);
    }

    #[test]
    fn sign_out_cleanup_codes_decode_honestly() {
        let all = parse_cleanup_code("cleared_all");
        assert_eq!(all, SignOutCleanup::ClearedTokenAndAlerts);
        assert!(all.cleared_token());
        assert!(all.cleared_alerts());

        let alerts_only = parse_cleanup_code("cleared_alerts_only");
        assert_eq!(alerts_only, SignOutCleanup::ClearedAlertsOnly);
        assert!(!alerts_only.cleared_token());
        assert!(alerts_only.cleared_alerts());

        let unavailable = parse_cleanup_code("");
        assert_eq!(unavailable, SignOutCleanup::Unavailable);
        assert!(!unavailable.cleared_token());
        assert!(!unavailable.cleared_alerts());
    }
}
