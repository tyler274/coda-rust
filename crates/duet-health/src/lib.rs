//! On-device health samples. Only the Duet app imports this crate.
//! Apple HealthKit is reached through the Swift binding in `platform/ios`.
//! Android reads granted Health Connect records through `platform/android`.
//! A missing permission or an empty store contributes no sample.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Available,
    Unavailable { reason: &'static str },
}

pub trait HealthSource {
    fn status(&self) -> HealthStatus;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PhoneReading {
    pub name: String,
    pub value: String,
    pub unit: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreState {
    Missing,
    UpdateRequired,
    PermissionRequired,
    Connected,
    Unreadable,
    NoStore,
    HealthKitAvailable,
    HealthKitUnavailable,
}

impl StoreState {
    pub fn sentence(self) -> &'static str {
        match self {
            Self::Missing => "Health Connect is not installed on this phone.",
            Self::UpdateRequired => "Health Connect needs an update before Duet can read it.",
            Self::PermissionRequired => {
                "Health Connect is installed. Duet does not have permission to read it yet."
            }
            Self::Connected => "Health Connect is connected.",
            Self::Unreadable => "Health Connect could not be read.",
            Self::NoStore => "This build has no on-device health store.",
            Self::HealthKitAvailable => "HealthKit is available.",
            Self::HealthKitUnavailable => "HealthKit is not available on this iPhone.",
        }
    }

    pub fn can_request(self) -> bool {
        matches!(self, Self::PermissionRequired)
    }

    pub fn connected(self) -> bool {
        matches!(self, Self::Connected)
    }
}

pub fn parse_status(code: &str) -> StoreState {
    match code {
        "missing" => StoreState::Missing,
        "update" => StoreState::UpdateRequired,
        "permission" => StoreState::PermissionRequired,
        "connected" => StoreState::Connected,
        _ => StoreState::Unreadable,
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
pub struct HealthSnapshot {
    pub state: StoreState,
    pub readings: Vec<PhoneReading>,
}

impl HealthSnapshot {
    pub fn sentence(&self) -> &'static str {
        self.state.sentence()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PlatformHealth;

impl PlatformHealth {
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

    pub fn request_access(&self) {
        #[cfg(target_os = "android")]
        android_request();
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
    extern "C" {
        fn duet_healthkit_available() -> i32;
    }
    let state = if unsafe { duet_healthkit_available() } == 1 {
        StoreState::HealthKitAvailable
    } else {
        StoreState::HealthKitUnavailable
    };
    HealthSnapshot {
        state,
        readings: Vec::new(),
    }
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
    let _ = android_call("requestPermission", "()V");
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
            let value = env
                .call_static_method(
                    bridge,
                    method,
                    "(Landroid/content/Context;)Ljava/lang/String;",
                    &[JValue::Object(&context)],
                )
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

pub fn decode_readings(body: &str) -> Vec<PhoneReading> {
    let page: ReadingPage = match serde_json::from_str(body) {
        Ok(page) => page,
        Err(_) => return Vec::new(),
    };
    granted_readings(&page.rows, &page.granted)
}

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
    }
}
