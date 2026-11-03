//! On-device health samples and acquisition primitives for the Duet client.
//! Apple HealthKit is reached through the Swift binding in `platform/ios`.
//! Android reads granted Health Connect records through `platform/android`.
//! A missing permission or an empty store contributes no sample.
//! Book: `book/src/backend-architecture.md`.
//! Durable acquisition lives in [`acquisition`] (records and the native
//! adapter contract), [`native_adapters`] (HealthKit and the gated Health
//! Connect adapter over thin native bridges) and [`outbox`] (the
//! transactional upload outbox).
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

pub mod acquisition;
pub mod native_adapters;
pub mod outbox;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
/// The `HealthStatus` cases.
pub enum HealthStatus {
    /// The `Available` case.
    Available,
    /// The store could not be read.
    Unavailable {
        /// Why the store is unavailable.
        reason: &'static str,
    },
}

/// Behavior implemented by `HealthSource`.
pub trait HealthSource {
    /// Returns whether the store can be read.
    fn status(&self) -> HealthStatus;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
/// The `PhoneReading` value exchanged by this crate.
pub struct PhoneReading {
    /// The `name` value.
    pub name: String,
    /// The `value` value.
    pub value: String,
    /// The `unit` value.
    pub unit: String,
    /// The app that wrote this reading. Empty when the phone did not report one.
    #[serde(default)]
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The `StoreState` cases.
pub enum StoreState {
    /// The `Missing` case.
    Missing,
    /// The `UpdateRequired` case.
    UpdateRequired,
    /// The `PermissionRequired` case.
    PermissionRequired,
    /// The `Connected` case.
    Connected,
    /// The `Unreadable` case.
    Unreadable,
    /// The `NoStore` case.
    NoStore,
    /// The `HealthKitAvailable` case.
    HealthKitAvailable,
    /// The `HealthKitUnavailable` case.
    HealthKitUnavailable,
}

impl StoreState {
    /// Returns the sentence shown for this state.
    pub fn sentence(self) -> &'static str {
        match self {
            Self::Missing => "Health Connect is not installed on this phone.",
            Self::UpdateRequired => "Health Connect needs an update before Duet can read it.",
            Self::PermissionRequired => {
                if cfg!(target_os = "ios") {
                    "HealthKit is available. Duet does not have permission to read it yet."
                } else {
                    "Health Connect is installed. Duet does not have permission to read it yet."
                }
            }
            Self::Connected => {
                if cfg!(target_os = "ios") {
                    "HealthKit is connected."
                } else {
                    "Health Connect is connected."
                }
            }
            Self::Unreadable => {
                if cfg!(target_os = "ios") {
                    "HealthKit could not be read."
                } else {
                    "Health Connect could not be read."
                }
            }
            Self::NoStore => "This build has no on-device health store.",
            Self::HealthKitAvailable => "HealthKit is available.",
            Self::HealthKitUnavailable => "HealthKit is not available on this iPhone.",
        }
    }

    /// Reports whether the person can be asked for access.
    pub fn can_request(self) -> bool {
        matches!(self, Self::PermissionRequired)
    }

    /// Reports whether the store is connected.
    pub fn connected(self) -> bool {
        matches!(self, Self::Connected)
    }
}

/// Maps a store status code to `StoreState`.
pub fn parse_status(code: &str) -> StoreState {
    match code {
        "missing" => StoreState::Missing,
        "update" => StoreState::UpdateRequired,
        "permission" => StoreState::PermissionRequired,
        "connected" => StoreState::Connected,
        _ => StoreState::Unreadable,
    }
}

/// Names the app that wrote a Health Connect record.
/// An empty package stays empty. Samsung Health and Google Health are named.
/// Every other package, including Health Connect itself, stays Health Connect.
pub fn source_name(package: &str) -> &'static str {
    match package {
        "" => "",
        "com.sec.android.app.shealth" => "Samsung Health",
        "com.google.android.apps.fitness" => "Google Health",
        _ => "Health Connect",
    }
}

/// Keeps a row only when its name was granted. The bridge is expected to do
/// this too; the filter is what a test can see without a phone.
pub fn granted_readings(rows: &[PhoneReading], granted: &[String]) -> Vec<PhoneReading> {
    rows.iter()
        .filter(|row| granted.iter().any(|name| name == &row.name))
        .cloned()
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The `HealthSnapshot` value exchanged by this crate.
pub struct HealthSnapshot {
    /// The `state` value.
    pub state: StoreState,
    /// The `readings` value.
    pub readings: Vec<PhoneReading>,
}

impl HealthSnapshot {
    /// Returns the sentence shown for this state.
    pub fn sentence(&self) -> &'static str {
        self.state.sentence()
    }
}

#[derive(Debug, Default, Clone, Copy)]
/// The `PlatformHealth` value exchanged by this crate.
pub struct PlatformHealth;

impl PlatformHealth {
    /// Reads the current on-device snapshot.
    pub fn snapshot(&self) -> HealthSnapshot {
        #[cfg(target_os = "android")]
        {
            return android_snapshot();
        }
        #[cfg(target_os = "ios")]
        {
            return ios_snapshot();
        }
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            HealthSnapshot {
                state: StoreState::NoStore,
                readings: Vec::new(),
            }
        }
    }

    /// Asks the operating system for health access.
    pub fn request_access(&self) {
        #[cfg(target_os = "android")]
        android_request();
        #[cfg(target_os = "ios")]
        ios_request();
    }
}

impl HealthSource for PlatformHealth {
    fn status(&self) -> HealthStatus {
        let snapshot = self.snapshot();
        if snapshot.state.connected() || matches!(snapshot.state, StoreState::HealthKitAvailable) {
            HealthStatus::Available
        } else {
            HealthStatus::Unavailable {
                reason: snapshot.sentence(),
            }
        }
    }
}

#[cfg(target_os = "ios")]
fn ios_snapshot() -> HealthSnapshot {
    unsafe extern "C" {
        fn duet_healthkit_status() -> *mut std::ffi::c_char;
        fn duet_healthkit_readings() -> *mut std::ffi::c_char;
        fn duet_healthkit_string_free(pointer: *mut std::ffi::c_char);
    }
    let Some(code) = bridge_string(
        unsafe { duet_healthkit_status() },
        duet_healthkit_string_free,
    ) else {
        return unreadable_snapshot();
    };
    if code != "connected" {
        return healthkit_snapshot(&code, Ok(""));
    }
    match bridge_string(
        unsafe { duet_healthkit_readings() },
        duet_healthkit_string_free,
    ) {
        Some(body) => healthkit_snapshot("connected", Ok(&body)),
        None => healthkit_snapshot("connected", Err(())),
    }
}

/// Result of an unsuccessful HealthKit authorization request.
#[derive(Debug, thiserror::Error)]
pub enum AuthorizationError {
    /// HealthKit reported that the request did not complete.
    #[error("HealthKit did not complete the authorization request")]
    RequestFailed,
    /// The error domain and code supplied by HealthKit.
    #[error("HealthKit authorization failed ({domain} {code})")]
    Native {
        /// NSError domain.
        domain: String,
        /// NSError code.
        code: i64,
    },
    /// The native callback was dropped.
    #[error("authorization callback dropped")]
    Canceled(#[from] futures_channel::oneshot::Canceled),
}

/// Wait for the HealthKit permission sheet before querying. Completion only means the
/// request finished; HealthKit deliberately does not disclose which read types were denied.
#[cfg(target_os = "ios")]
pub async fn request_apple_health() -> Result<(), AuthorizationError> {
    use futures_channel::oneshot;
    unsafe extern "C" {
        fn duet_healthkit_authorize(
            identifiers: *const std::ffi::c_char,
            context: usize,
            callback: extern "C" fn(usize, i32, *const std::ffi::c_char, i64),
        );
    }
    type Reply = Result<(), AuthorizationError>;
    extern "C" fn completed(
        context: usize,
        success: i32,
        domain: *const std::ffi::c_char,
        code: i64,
    ) {
        // Swift invokes the callback exactly once, even when HealthKit is unavailable.
        let sender = unsafe { Box::from_raw(context as *mut oneshot::Sender<Reply>) };
        let reply = if !domain.is_null() {
            // Swift borrows NSError.domain for the duration of this callback.
            let domain = unsafe { std::ffi::CStr::from_ptr(domain) }
                .to_string_lossy()
                .into_owned();
            Err(AuthorizationError::Native { domain, code })
        } else if success != 0 {
            Ok(())
        } else {
            Err(AuthorizationError::RequestFailed)
        };
        let _ = sender.send(reply);
    }
    let (sender, result) = oneshot::channel::<Reply>();
    let types = native_adapters::default_healthkit_types()
        .into_iter()
        .map(|s| s.data_type)
        .collect::<Vec<_>>()
        .join(",");
    let types = std::ffi::CString::new(types).expect("code-owned HealthKit identifiers");
    unsafe {
        duet_healthkit_authorize(
            types.as_ptr(),
            Box::into_raw(Box::new(sender)) as usize,
            completed,
        )
    };
    result.await?
}

#[cfg(target_os = "ios")]
fn ios_request() {
    unsafe extern "C" {
        fn duet_healthkit_request();
    }
    unsafe { duet_healthkit_request() }
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

/// Maps a HealthKit status code and readings body onto the store snapshot.
/// An empty connected store stays connected. A failed read is unreadable.
#[cfg(any(test, target_os = "ios"))]
fn healthkit_snapshot(code: &str, body: Result<&str, ()>) -> HealthSnapshot {
    let state = match code {
        "unavailable" => StoreState::HealthKitUnavailable,
        "permission" => StoreState::PermissionRequired,
        "connected" => StoreState::Connected,
        _ => StoreState::Unreadable,
    };
    if !state.connected() {
        return HealthSnapshot {
            state,
            readings: Vec::new(),
        };
    }
    match body {
        Ok(body) => HealthSnapshot {
            state,
            readings: decode_readings(body),
        },
        Err(()) => unreadable_snapshot(),
    }
}

#[cfg(any(test, target_os = "ios"))]
fn unreadable_snapshot() -> HealthSnapshot {
    HealthSnapshot {
        state: StoreState::Unreadable,
        readings: Vec::new(),
    }
}

/// Opens the system permission chooser without reading the Health Connect store.
#[cfg(target_os = "android")]
pub fn request_health_connect_permission() -> Result<(), String> {
    android_call("requestPermission", "()V").map(|_| ())
}

#[cfg(target_os = "android")]
fn android_snapshot() -> HealthSnapshot {
    let state = match android_call("status", "()Ljava/lang/String;") {
        Ok(code) => parse_status(&code),
        Err(_) => StoreState::Unreadable,
    };
    let readings = if state.connected() {
        match android_call("readings", "()Ljava/lang/String;") {
            Ok(body) => decode_readings(&body),
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    };
    HealthSnapshot { state, readings }
}

#[cfg(target_os = "android")]
fn android_request() {
    let _ = request_health_connect_permission();
}

#[cfg(target_os = "android")]
fn android_call(method: &str, signature: &str) -> Result<String, String> {
    android_invoke(method, signature, None)
}

/// Calls `HealthConnectBridge.<method>(Context, String): String`.
#[cfg(target_os = "android")]
pub(crate) fn android_call_string(method: &str, argument: &str) -> Result<String, String> {
    android_invoke(
        method,
        "(Ljava/lang/String;)Ljava/lang/String;",
        Some(argument),
    )
}

#[cfg(target_os = "android")]
fn android_invoke(method: &str, signature: &str, argument: Option<&str>) -> Result<String, String> {
    use jni::objects::{JClass, JObject, JString, JValue};
    let ctx = ndk_context::android_context();
    let vm =
        unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.map_err(|error| error.to_string())?;
    let mut env = vm
        .attach_current_thread()
        .map_err(|error| error.to_string())?;
    let context = unsafe { JObject::from_raw(ctx.context().cast()) };
    let result = (|| -> Result<String, String> {
        // A native thread's FindClass cannot see the app's classes. Load the
        // bridge through the activity class loader instead.
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
            .new_string("coda.duet.HealthConnectBridge")
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
        if signature.ends_with(")V") {
            env.call_static_method(
                bridge,
                method,
                "(Landroid/content/Context;)V",
                &[JValue::Object(&context)],
            )
            .map(|_| String::new())
            .map_err(|error| error.to_string())
        } else {
            let value = match argument {
                Some(argument) => {
                    let argument = env
                        .new_string(argument)
                        .map_err(|error| error.to_string())?;
                    env.call_static_method(
                        bridge,
                        method,
                        "(Landroid/content/Context;Ljava/lang/String;)Ljava/lang/String;",
                        &[JValue::Object(&context), JValue::Object(&argument)],
                    )
                }
                None => env.call_static_method(
                    bridge,
                    method,
                    "(Landroid/content/Context;)Ljava/lang/String;",
                    &[JValue::Object(&context)],
                ),
            }
            .map_err(|error| error.to_string())?;
            let object = value.l().map_err(|error| error.to_string())?;
            let jstring = JString::from(object);
            let text = env
                .get_string(&jstring)
                .map_err(|error| error.to_string())?;
            Ok(text.to_str().map_err(|error| error.to_string())?.to_owned())
        }
    })();
    if env.exception_check().unwrap_or(false) {
        env.exception_clear().ok();
    }
    // The activity context is a global reference owned by the runtime.
    std::mem::forget(context);
    result
}

#[derive(Debug, Deserialize)]
struct ReadingPage {
    #[serde(default)]
    granted: Vec<String>,
    #[serde(default)]
    rows: Vec<PhoneReading>,
}

/// Decodes Health Connect rows from the bridge JSON.
pub fn decode_readings(body: &str) -> Vec<PhoneReading> {
    let page: ReadingPage = match serde_json::from_str(body) {
        Ok(page) => page,
        Err(_) => return Vec::new(),
    };
    granted_readings(&page.rows, &page.granted)
        .into_iter()
        .map(|mut row| {
            row.source = source_name(&row.source).to_owned();
            row
        })
        .collect()
}

/// The `NO_READINGS` constant.
pub const NO_READINGS: &str = "Health Connect has no readings to show.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_build_does_not_invent_samples() {
        let snapshot = PlatformHealth.snapshot();
        assert!(snapshot.readings.is_empty());
        assert!(!snapshot.state.connected());
        assert!(matches!(
            PlatformHealth.status(),
            HealthStatus::Unavailable { .. }
        ));
    }

    #[test]
    fn status_codes_are_the_four_store_states() {
        assert_eq!(parse_status("missing"), StoreState::Missing);
        assert_eq!(parse_status("update"), StoreState::UpdateRequired);
        assert_eq!(parse_status("permission"), StoreState::PermissionRequired);
        assert_eq!(parse_status("connected"), StoreState::Connected);
        assert!(parse_status("permission").can_request());
        assert!(parse_status("connected").connected());
        assert_eq!(
            parse_status("connected").sentence(),
            "Health Connect is connected."
        );
        assert_eq!(parse_status("unread"), StoreState::Unreadable);
    }

    #[test]
    fn ungranted_readings_are_omitted() {
        let body = r#"{"granted":["Steps"],"rows":[
            {"name":"Steps","value":"1200","unit":"steps"},
            {"name":"Weight","value":"70","unit":"kg"}
        ]}"#;
        let readings = decode_readings(body);
        assert_eq!(readings.len(), 1);
        assert_eq!(readings[0].name, "Steps");
        assert_eq!(readings[0].value, "1200");
        assert!(readings[0].source.is_empty());
    }

    #[test]
    fn health_connect_origins_keep_their_names() {
        let body = r#"{"granted":["Steps","Weight"],"rows":[
            {"name":"Steps","value":"800","unit":"steps","source":"com.sec.android.app.shealth"},
            {"name":"Steps","value":"400","unit":"steps","source":"com.google.android.apps.fitness"},
            {"name":"Weight","value":"70","unit":"kg","source":"com.google.android.apps.healthdata"},
            {"name":"Weight","value":"71","unit":"kg","source":"com.example.other"}
        ]}"#;
        let readings = decode_readings(body);
        assert_eq!(readings.len(), 4);
        assert_eq!(readings[0].source, "Samsung Health");
        assert_eq!(readings[1].source, "Google Health");
        assert_eq!(readings[2].source, "Health Connect");
        assert_eq!(readings[3].source, "Health Connect");
        assert_eq!(source_name(""), "");
    }

    #[test]
    fn a_host_build_does_not_invent_a_phone_source() {
        let snapshot = PlatformHealth.snapshot();
        assert!(
            snapshot
                .readings
                .iter()
                .all(|reading| reading.source.is_empty())
        );
    }

    #[test]
    fn healthkit_permission_can_be_requested_and_hides_readings() {
        let body =
            r#"{"granted":["Steps"],"rows":[{"name":"Steps","value":"1200","unit":"steps"}]}"#;
        let snapshot = healthkit_snapshot("permission", Ok(body));
        assert_eq!(snapshot.state, StoreState::PermissionRequired);
        assert!(snapshot.state.can_request());
        assert!(snapshot.readings.is_empty());
    }

    #[test]
    fn healthkit_unavailable_has_no_readings() {
        let snapshot = healthkit_snapshot("unavailable", Ok(""));
        assert_eq!(snapshot.state, StoreState::HealthKitUnavailable);
        assert!(snapshot.readings.is_empty());
        assert!(!snapshot.state.can_request());
    }

    #[test]
    fn healthkit_connected_keeps_an_empty_store() {
        let snapshot = healthkit_snapshot("connected", Ok(r#"{"granted":[],"rows":[]}"#));
        assert!(snapshot.state.connected());
        assert!(snapshot.readings.is_empty());
        assert_eq!(snapshot.state.sentence(), "Health Connect is connected.");
    }

    #[test]
    fn healthkit_connected_omits_ungranted_rows() {
        let body = r#"{"granted":["Steps"],"rows":[
            {"name":"Steps","value":"1200","unit":"steps"},
            {"name":"Weight","value":"70","unit":"kg"}
        ]}"#;
        let snapshot = healthkit_snapshot("connected", Ok(body));
        assert_eq!(snapshot.readings.len(), 1);
        assert_eq!(snapshot.readings[0].name, "Steps");
    }

    #[test]
    fn healthkit_thrown_read_is_unreadable() {
        let snapshot = healthkit_snapshot("connected", Err(()));
        assert_eq!(snapshot.state, StoreState::Unreadable);
        assert!(snapshot.readings.is_empty());
        assert!(!snapshot.state.connected());
    }

    #[test]
    fn healthkit_unknown_code_is_unreadable() {
        let snapshot = healthkit_snapshot("unread", Ok(""));
        assert_eq!(snapshot.state, StoreState::Unreadable);
        assert!(snapshot.readings.is_empty());
    }
}
