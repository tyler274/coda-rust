#[path = "../../platform/ios/build_bridge.rs"]
mod bridge;

fn main() {
    bridge::link(
        "duet_secure",
        "SecureRetentionBridge",
        &["Foundation", "Security", "UIKit"],
    );
}
