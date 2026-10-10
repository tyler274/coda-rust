#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    fn root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    fn manifest(crate_name: &str) -> String {
        std::fs::read_to_string(root().join("crates").join(crate_name).join("Cargo.toml")).unwrap()
    }

    #[test]
    fn apps_do_not_import_each_others_clinical_crates() {
        let physician = manifest("physician-app");
        assert!(!physician.contains("duet-"));
        assert!(!physician.contains("duet_"));
        let duet = manifest("duet-app");
        assert!(!duet.contains("physician-"));
        let health = manifest("duet-health");
        assert!(!health.contains("physician-"));
        let physician_client = manifest("physician-client");
        assert!(!physician_client.contains("duet-"));
    }

    #[test]
    fn shipping_clients_stay_until_the_rust_journeys_match() {
        let root = root();
        assert!(
            root.join("submodules/coda-app/ios/DuetApp/Sources/DuetApp")
                .exists()
        );
        assert!(
            root.join("submodules/coda-agents/web/src/main.tsx")
                .exists()
        );
    }
}
