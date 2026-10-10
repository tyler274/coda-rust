import HealthKit

/// Binding for `duet-health`. Screens stay in Rust.
@_cdecl("duet_healthkit_available")
public func duetHealthKitAvailable() -> Int32 {
    HKHealthStore.isHealthDataAvailable() ? 1 : 0
}
