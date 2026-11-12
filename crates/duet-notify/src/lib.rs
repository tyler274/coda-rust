//! Alert permission.
//! Only the Duet app imports this crate.
//! The Swift binding is `platform/ios/NotificationBridge.swift`.
//! The Kotlin binding is `platform/android/NotificationBridge.kt`.
//! A host build has no center.
//! Book: `book/src/backend-architecture.md`.
#![deny(missing_docs)]

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

/// Reads and requests notification alerts on this phone.
#[derive(Debug, Default, Clone, Copy)]
pub struct PlatformAlerts;

impl PlatformAlerts {
    /// Reads the current alert permission.
    pub fn snapshot(&self) -> AlertSnapshot {
        #[cfg(target_os = "ios")]
        {
            return ios_snapshot();
        }
        #[cfg(target_os = "android")]
        {
            return android_snapshot();
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
    match android_call("status", "()Ljava/lang/String;") {
        Ok(code) => alert_snapshot(&code),
        Err(_) => AlertSnapshot {
            state: AlertState::Unreadable,
        },
    }
}

#[cfg(target_os = "android")]
fn android_request() {
    let _ = android_call("requestPermission", "()V");
}

#[cfg(target_os = "android")]
fn android_open_settings() {
    let _ = android_call("openSettings", "()V");
}

#[cfg(target_os = "android")]
fn android_call(method: &str, signature: &str) -> Result<String, String> {
    use jni::objects::{JClass, JObject, JString, JValue};
    let ctx = ndk_context::android_context();
    let vm =
        unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.map_err(|error| error.to_string())?;
    let mut env = vm
        .attach_current_thread()
        .map_err(|error| error.to_string())?;
    let context = unsafe { JObject::from_raw(ctx.context().cast()) };
    let result = (|| -> Result<String, String> {
        let activity_class = env
            .get_object_class(&context)
            .map_err(|error| error.to_string())?;
        let loader = env
            .call_method(
                activity_class,
                "getClassLoader",
                "()Ljava/lang/ClassLoader;",
                &[],
            )
            .map_err(|error| error.to_string())?
            .l()
            .map_err(|error| error.to_string())?;
        let class_name = env
            .new_string("coda.duet.NotificationBridge")
            .map_err(|error| error.to_string())?;
        let bridge = env
            .call_method(
                loader,
                "loadClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                &[JValue::Object(&class_name)],
            )
            .map_err(|error| error.to_string())?
            .l()
            .map_err(|error| error.to_string())?;
        let bridge = JClass::from(bridge);
        let returns_void = signature.ends_with(")V");
        let value = if returns_void {
            env.call_static_method(
                bridge,
                method,
                "(Landroid/content/Context;)V",
                &[JValue::Object(&context)],
            )
        } else {
            env.call_static_method(
                bridge,
                method,
                "(Landroid/content/Context;)Ljava/lang/String;",
                &[JValue::Object(&context)],
            )
        }
        .map_err(|error| error.to_string())?;
        if returns_void {
            return Ok(String::new());
        }
        let object = value.l().map_err(|error| error.to_string())?;
        let jstring = JString::from(object);
        let text = env
            .get_string(&jstring)
            .map_err(|error| error.to_string())?;
        Ok(text.to_str().map_err(|error| error.to_string())?.to_owned())
    })();
    if env.exception_check().unwrap_or(false) {
        env.exception_clear().ok();
    }
    std::mem::forget(context);
    result
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
    }
}
