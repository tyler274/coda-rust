//! On-device health samples. Only the Duet app imports this crate.
//! Apple HealthKit is reached through the Swift binding in `platform/ios`.
//! Android reports Health Connect as unavailable until that adapter is in scope.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Available,
    Unavailable { reason: &'static str },
}

pub trait HealthSource {
    fn status(&self) -> HealthStatus;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PlatformHealth;

impl HealthSource for PlatformHealth {
    fn status(&self) -> HealthStatus {
        #[cfg(target_os = "ios")]
        {
            return ios_status();
        }
        #[cfg(target_os = "android")]
        {
            return HealthStatus::Unavailable {
                reason: "Health Connect is not connected on this Android build.",
            };
        }
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            HealthStatus::Unavailable { reason: "This build has no on-device health store." }
        }
    }
}

#[cfg(target_os = "ios")]
fn ios_status() -> HealthStatus {
    extern "C" {
        fn duet_healthkit_available() -> i32;
    }
    if unsafe { duet_healthkit_available() } == 1 {
        HealthStatus::Available
    } else {
        HealthStatus::Unavailable { reason: "HealthKit is not available on this iPhone." }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_build_does_not_invent_samples() {
        let status = PlatformHealth.status();
        assert!(matches!(status, HealthStatus::Unavailable { .. }));
    }
}
