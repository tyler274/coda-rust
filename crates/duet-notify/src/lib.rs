//! Alert permission and one local insight alert.
//! Only the Duet app imports this crate.
//! The Swift binding is `platform/ios/NotificationBridge.swift`.
//! Android local alerts go through `platform/android/NotificationBridge.kt`.
//! A host build has no center and schedules nothing. Android does not create a remote token.
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

    /// Asks iOS to register for remote notifications. The token is not uploaded.
    /// Android local alerts do not register a remote token.
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
        #[cfg(target_os = "android")]
        {
            return android_last_planned();
        }
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            None
        }
    }

    /// Submits one insight alert. A host build does nothing.
    pub fn submit_insight(&self, delivery: &InsightDelivery) {
        #[cfg(target_os = "ios")]
        ios_submit(delivery);
        #[cfg(target_os = "android")]
        android_submit(delivery);
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            let _ = delivery;
        }
    }

    /// Reads whether new insight notifications are on. They start off.
    pub fn insights_enabled(&self) -> bool {
        #[cfg(target_os = "ios")]
        {
            return ios_insights_enabled();
        }
        #[cfg(target_os = "android")]
        {
            return android_insights_enabled();
        }
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            false
        }
    }

    /// Turns new insight notifications on or off.
    ///
    /// Turning them on asks for alerts when none have been asked, and saves
    /// only when alerts are allowed. A host build cannot turn them on.
    /// The returned value is whether they are on after the call.
    pub fn set_insights(&self, enabled: bool) -> bool {
        #[cfg(target_os = "ios")]
        {
            return ios_set_insights(enabled);
        }
        #[cfg(target_os = "android")]
        {
            return android_set_insights(enabled);
        }
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            let _ = enabled;
            false
        }
    }

    /// Removes a pending daily insight alert. The planned cycle id stays.
    pub fn cancel_insight(&self) {
        #[cfg(target_os = "ios")]
        ios_cancel_insight();
        #[cfg(target_os = "android")]
        android_cancel();
    }

    /// Opens the system Settings page for this app.
    pub fn open_settings(&self) {
        #[cfg(target_os = "ios")]
        ios_open_settings();
        #[cfg(target_os = "android")]
        android_open_settings();
    }
}

/// Reports whether a new insight cycle may be scheduled.
/// The preference starts off. Alerts must also be allowed.
pub fn insight_alerts_wanted(preference: bool, state: AlertState) -> bool {
    preference && state == AlertState::Allowed
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

/// The JSON body the Android bridge schedules from one delivery.
#[cfg(any(test, target_os = "android"))]
fn insight_schedule_json(delivery: &InsightDelivery) -> String {
    format!(
        r#"{{"identifier":"{}","year":{},"month":{},"day":{},"hour":{},"minute":{},"cycle_id":"{}"}}"#,
        delivery.identifier,
        delivery.year,
        delivery.month,
        delivery.day,
        delivery.hour,
        delivery.minute,
        delivery.cycle_id
    )
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
fn ios_insights_enabled() -> bool {
    extern "C" {
        fn duet_notify_insights_enabled() -> i32;
    }
    unsafe { duet_notify_insights_enabled() != 0 }
}

#[cfg(target_os = "ios")]
fn ios_set_insights(enabled: bool) -> bool {
    extern "C" {
        fn duet_notify_set_insights(enabled: i32) -> i32;
    }
    unsafe { duet_notify_set_insights(i32::from(enabled)) != 0 }
}

#[cfg(target_os = "ios")]
fn ios_cancel_insight() {
    extern "C" {
        fn duet_notify_cancel_insights();
    }
    unsafe { duet_notify_cancel_insights() }
}

#[cfg(target_os = "ios")]
fn ios_open_settings() {
    extern "C" {
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
    match android_call("status", "()Ljava/lang/String;", None) {
        Ok(code) => alert_snapshot(&code),
        Err(_) => AlertSnapshot {
            state: AlertState::Unreadable,
        },
    }
}

#[cfg(target_os = "android")]
fn android_request() {
    let _ = android_call("requestPermission", "()V", None);
}

#[cfg(target_os = "android")]
fn android_last_planned() -> Option<Uuid> {
    let text = android_call("lastPlanned", "()Ljava/lang/String;", None).ok()?;
    Uuid::parse_str(&text).ok()
}

#[cfg(target_os = "android")]
fn android_submit(delivery: &InsightDelivery) {
    let _ = android_call(
        "schedule",
        "(Landroid/content/Context;Ljava/lang/String;)V",
        Some(&insight_schedule_json(delivery)),
    );
}

#[cfg(target_os = "android")]
fn android_insights_enabled() -> bool {
    android_call("insightsEnabled", "()Ljava/lang/String;", None)
        .ok()
        .is_some_and(|value| value == "true")
}

#[cfg(target_os = "android")]
fn android_set_insights(enabled: bool) -> bool {
    let flag = if enabled { "true" } else { "false" };
    android_call(
        "setInsights",
        "(Landroid/content/Context;Ljava/lang/String;)Ljava/lang/String;",
        Some(flag),
    )
    .ok()
    .is_some_and(|value| value == "true")
}

#[cfg(target_os = "android")]
fn android_cancel() {
    let _ = android_call("cancelInsights", "()V", None);
}

#[cfg(target_os = "android")]
fn android_open_settings() {
    let _ = android_call("openSettings", "()V", None);
}

#[cfg(target_os = "android")]
fn android_call(method: &str, signature: &str, extra: Option<&str>) -> Result<String, String> {
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
        let argument = extra
            .map(|text| env.new_string(text))
            .transpose()
            .map_err(|error| error.to_string())?;
        let returns_void = signature.ends_with(")V");
        let value = if let Some(argument) = argument.as_ref() {
            let full = if returns_void {
                "(Landroid/content/Context;Ljava/lang/String;)V"
            } else {
                "(Landroid/content/Context;Ljava/lang/String;)Ljava/lang/String;"
            };
            env.call_static_method(
                bridge,
                method,
                full,
                &[JValue::Object(&context), JValue::Object(argument)],
            )
        } else if returns_void {
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
        assert!(PlatformAlerts.last_planned_cycle().is_none());
        assert!(!PlatformAlerts.insights_enabled());
        assert!(!PlatformAlerts.set_insights(true));
        assert!(!PlatformAlerts.insights_enabled());
    }

    #[test]
    fn insight_alerts_stay_off_until_they_are_allowed() {
        assert!(!insight_alerts_wanted(false, AlertState::Allowed));
        assert!(!insight_alerts_wanted(true, AlertState::Denied));
        assert!(!insight_alerts_wanted(true, AlertState::NotDetermined));
        assert!(!insight_alerts_wanted(true, AlertState::Unreadable));
        assert!(!insight_alerts_wanted(true, AlertState::NoCenter));
        assert!(insight_alerts_wanted(true, AlertState::Allowed));
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
    fn an_insight_delivery_names_the_android_alarm() {
        let offset = chrono::FixedOffset::east_opt(0).unwrap();
        let now = offset.with_ymd_and_hms(2026, 10, 11, 8, 0, 0).unwrap();
        let cycle = Uuid::nil();
        let delivery = plan_insight_delivery(now, &[(cycle, true)], None).unwrap();
        let body = insight_schedule_json(&delivery);
        assert!(body.contains("\"identifier\":\"health.duet.notification.daily.2026-10-11\""));
        assert!(body.contains("\"hour\":9"));
        assert!(body.contains(&format!("\"cycle_id\":\"{cycle}\"")));
    }

    #[test]
    fn a_page_without_an_insight_cycle_plans_nothing() {
        let offset = chrono::FixedOffset::east_opt(0).unwrap();
        let now = offset.with_ymd_and_hms(2026, 10, 11, 8, 0, 0).unwrap();
        assert!(plan_insight_delivery(now, &[], None).is_none());
        assert!(plan_insight_delivery(now, &[(Uuid::nil(), false)], None).is_none());
    }
}
