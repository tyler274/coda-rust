//! Alert permission and one local insight alert for the iPhone.
//! Only the Duet app imports this crate.
//! The Swift binding is `platform/ios/NotificationBridge.swift`.
//! A host or Android build has no center, schedules nothing, and does not invent a token.
//! Book: `book/src/behaviour-contract.md`.
#![deny(missing_docs)]

use chrono::{DateTime, Datelike, Duration, TimeZone};
use uuid::Uuid;

/// Request id prefix for the one daily insight alert.
pub const INSIGHT_REQUEST_PREFIX: &str = "health.duet.notification.daily.";

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

    /// Asks iOS to register for remote notifications. The token is not uploaded.
    pub fn register_remote(&self) {
        #[cfg(target_os = "ios")]
        ios_register_remote();
    }

    /// Reads the cycle id of the last insight alert this phone planned.
    pub fn last_planned_cycle(&self) -> Option<Uuid> {
        #[cfg(target_os = "ios")]
        {
            return ios_last_planned();
        }
        #[cfg(not(target_os = "ios"))]
        {
            None
        }
    }

    /// Submits one insight alert. A host or Android build does nothing.
    pub fn submit_insight(&self, delivery: &InsightDelivery) {
        #[cfg(target_os = "ios")]
        ios_submit(delivery);
        #[cfg(not(target_os = "ios"))]
        {
            let _ = delivery;
        }
    }
}

/// One local insight alert.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsightDelivery {
    /// The cycle this alert is for.
    pub cycle_id: Uuid,
    /// The notification request id.
    pub identifier: String,
    /// Local year of the 9:00 delivery.
    pub year: i32,
    /// Local month of the 9:00 delivery.
    pub month: u32,
    /// Local day of the 9:00 delivery.
    pub day: u32,
    /// Local hour of the delivery. This is 9.
    pub hour: u32,
    /// Local minute of the delivery. This is 0.
    pub minute: u32,
}

/// Plans at most one insight alert.
///
/// `cycles` is the loaded page, in order, as `(id, has insights)`. The first
/// cycle that has insights is the candidate. The same id as `last_planned`
/// plans nothing. A page with no such cycle plans nothing. The delivery is
/// the next local 9:00 strictly after `now`.
pub fn plan_insight_delivery<Tz: TimeZone>(
    now: DateTime<Tz>,
    cycles: &[(Uuid, bool)],
    last_planned: Option<Uuid>,
) -> Option<InsightDelivery> {
    let cycle_id = cycles.iter().find(|(_, has)| *has).map(|(id, _)| *id)?;
    if last_planned == Some(cycle_id) {
        return None;
    }
    let today = now.date_naive();
    let today_nine = today.and_hms_opt(9, 0, 0)?;
    let fire_naive = if today_nine > now.naive_local() {
        today_nine
    } else {
        today
            .checked_add_signed(Duration::days(1))?
            .and_hms_opt(9, 0, 0)?
    };
    let chrono::LocalResult::Single(fire) = now.timezone().from_local_datetime(&fire_naive) else {
        return None;
    };
    Some(InsightDelivery {
        cycle_id,
        identifier: format!(
            "{INSIGHT_REQUEST_PREFIX}{:04}-{:02}-{:02}",
            fire.year(),
            fire.month(),
            fire.day()
        ),
        year: fire.year(),
        month: fire.month(),
        day: fire.day(),
        hour: 9,
        minute: 0,
    })
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
fn ios_register_remote() {
    extern "C" {
        fn duet_notify_register_remote();
    }
    unsafe { duet_notify_register_remote() }
}

#[cfg(target_os = "ios")]
fn ios_last_planned() -> Option<Uuid> {
    extern "C" {
        fn duet_notify_last_planned() -> *mut std::ffi::c_char;
        fn duet_notify_string_free(pointer: *mut std::ffi::c_char);
    }
    let text = bridge_string(
        unsafe { duet_notify_last_planned() },
        duet_notify_string_free,
    )?;
    Uuid::parse_str(&text).ok()
}

#[cfg(target_os = "ios")]
fn ios_submit(delivery: &InsightDelivery) {
    extern "C" {
        fn duet_notify_schedule(
            identifier: *const std::ffi::c_char,
            year: i32,
            month: i32,
            day: i32,
            hour: i32,
            minute: i32,
            cycle_id: *const std::ffi::c_char,
        );
    }
    let Ok(identifier) = std::ffi::CString::new(delivery.identifier.as_str()) else {
        return;
    };
    let Ok(cycle_id) = std::ffi::CString::new(delivery.cycle_id.to_string()) else {
        return;
    };
    unsafe {
        duet_notify_schedule(
            identifier.as_ptr(),
            delivery.year,
            delivery.month as i32,
            delivery.day as i32,
            delivery.hour as i32,
            delivery.minute as i32,
            cycle_id.as_ptr(),
        );
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
        assert!(PlatformAlerts.last_planned_cycle().is_none());
    }

    #[test]
    fn a_new_cycle_before_nine_is_delivered_today() {
        let offset = chrono::FixedOffset::east_opt(0).unwrap();
        let now = offset.with_ymd_and_hms(2026, 10, 11, 8, 0, 0).unwrap();
        let cycle = Uuid::nil();
        let delivery = plan_insight_delivery(now, &[(cycle, true)], None).unwrap();
        assert_eq!(delivery.cycle_id, cycle);
        assert_eq!(
            delivery.identifier,
            "health.duet.notification.daily.2026-10-11"
        );
        assert_eq!(
            (delivery.year, delivery.month, delivery.day),
            (2026, 10, 11)
        );
        assert_eq!((delivery.hour, delivery.minute), (9, 0));
    }

    #[test]
    fn a_new_cycle_at_or_after_nine_is_delivered_the_next_morning() {
        let offset = chrono::FixedOffset::east_opt(0).unwrap();
        let now = offset.with_ymd_and_hms(2026, 10, 11, 9, 0, 0).unwrap();
        let cycle = Uuid::from_u128(1);
        let delivery =
            plan_insight_delivery(now, &[(Uuid::nil(), false), (cycle, true)], None).unwrap();
        assert_eq!(delivery.cycle_id, cycle);
        assert_eq!(
            delivery.identifier,
            "health.duet.notification.daily.2026-10-12"
        );
        assert_eq!((delivery.hour, delivery.minute), (9, 0));
    }

    #[test]
    fn the_same_cycle_is_not_planned_again() {
        let offset = chrono::FixedOffset::east_opt(0).unwrap();
        let now = offset.with_ymd_and_hms(2026, 10, 11, 8, 0, 0).unwrap();
        let cycle = Uuid::from_u128(2);
        assert!(plan_insight_delivery(now, &[(cycle, true)], Some(cycle)).is_none());
    }

    #[test]
    fn a_page_without_an_insight_cycle_plans_nothing() {
        let offset = chrono::FixedOffset::east_opt(0).unwrap();
        let now = offset.with_ymd_and_hms(2026, 10, 11, 8, 0, 0).unwrap();
        assert!(plan_insight_delivery(now, &[], None).is_none());
        assert!(plan_insight_delivery(now, &[(Uuid::nil(), false)], None).is_none());
    }
}
