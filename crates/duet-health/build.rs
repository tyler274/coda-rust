#[path = "../../platform/ios/build_bridge.rs"]
mod bridge;

fn main() {
    bridge::link(
        "duet_healthkit",
        "HealthKitBridge",
        &["Foundation", "HealthKit"],
    );
}
