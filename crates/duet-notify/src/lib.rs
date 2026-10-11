//! Alert permission for the iPhone notification center.
//! Only the Duet app imports this crate.
//! The Swift binding is `platform/ios/NotificationBridge.swift`.
//! A host or Android build has no center and does not invent a prompt.
//! Book: `book/src/behaviour-contract.md`.
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
    /// This build has no iPhone notification center.
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

/// Reads and requests iPhone notification alerts.
#[derive(Debug, Default, Clone, Copy)]
pub struct PlatformAlerts;

impl PlatformAlerts {
    /// Reads the current alert permission.
    pub fn snapshot(&self) -> AlertSnapshot {
        #[cfg(target_os = "ios")]
        {
            return ios_snapshot();
        }
        #[cfg(not(target_os = "ios"))]
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
    }
}

/// Maps a notification status code onto the alert snapshot.
/// An unknown or empty code is unreadable, not a denial.
#[cfg(any(test, target_os = "ios"))]
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
    extern "C" {
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
    extern "C" {
        fn duet_notify_request();
    }
    unsafe { duet_notify_request() }
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
