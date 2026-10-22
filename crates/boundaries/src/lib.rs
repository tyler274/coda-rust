//! Keeps Duet and the physician client from sharing clinical crates.
//!
//! The book chapter is `book/src/reading.md`.

#![deny(missing_docs)]

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
        let shell = manifest("mobile");
        assert!(shell.contains("duet-app"));
        assert!(shell.contains("physician-app"));
    }

    /// Follow local dependencies, including renamed dependencies, rather than
    /// trusting direct package-name string checks. No generated clinical crate
    /// can slip into the neutral mechanics through a transitive dependency.
    fn local_closure(crate_name: &str) -> std::collections::BTreeSet<String> {
        fn visit(name: &str, seen: &mut std::collections::BTreeSet<String>) {
            if !seen.insert(name.to_owned()) {
                return;
            }
            fn paths(value: &toml::Value, found: &mut Vec<String>) {
                if let Some(table) = value.as_table() {
                    if let Some(path) = table.get("path").and_then(toml::Value::as_str) {
                        found.push(path.to_owned());
                    }
                    // Resolve workspace-inherited dependencies explicitly before
                    // allowing them into the boundary (currently none are used).
                    assert!(
                        table.get("workspace").and_then(toml::Value::as_bool) != Some(true),
                        "workspace-inherited dependencies need boundary resolution"
                    );
                    for child in table.values() {
                        paths(child, found);
                    }
                }
            }
            // toml 1.x parses a whole document as a `Table`; `Value` parses one value.
            let document: toml::Table = toml::from_str(&manifest(name)).unwrap();
            let mut dependencies = Vec::new();
            for section in [
                "dependencies",
                "dev-dependencies",
                "build-dependencies",
                "target",
            ] {
                if let Some(value) = document.get(section) {
                    paths(value, &mut dependencies);
                }
            }
            for path in dependencies {
                let dependency = path.strip_prefix("../").expect("local crate dependency");
                assert!(
                    !dependency.contains('/'),
                    "unexpected local dependency path"
                );
                visit(dependency, seen);
            }
        }
        let mut seen = std::collections::BTreeSet::new();
        visit(crate_name, &mut seen);
        seen
    }

    #[test]
    fn shared_harness_is_product_neutral_and_has_no_product_store() {
        let allowed = ["harness", "agent-skills", "http"];
        for dependency in local_closure("harness") {
            assert!(
                allowed.contains(&dependency.as_str()),
                "neutral harness imported product-owned crate {dependency}"
            );
        }
        for duet in ["duet-api", "duet-app", "duet-client", "duet-page"] {
            for dependency in local_closure(duet) {
                assert!(!dependency.starts_with("physician-"));
                assert!(
                    !["local-product", "patient-work"].contains(&dependency.as_str()),
                    "Duet must reuse harness mechanics, not physician state/transport"
                );
            }
        }
        for physician in ["physician-app", "physician-sample", "local-product"] {
            for dependency in local_closure(physician) {
                assert!(!dependency.starts_with("duet-"));
            }
        }
    }

    #[test]
    fn npi_registry_holds_only_public_directory_data() {
        for dependency in local_closure("npi-registry") {
            assert!(
                !dependency.starts_with("duet-")
                    && !dependency.starts_with("duet_")
                    && !dependency.starts_with("physician-")
                    && !["local-product", "patient-work"].contains(&dependency.as_str()),
                "shared npi-registry imported product-owned crate {dependency}"
            );
        }
    }

    #[test]
    fn shipping_clients_stay_until_the_rust_journeys_match() {
        let root = root();
        let ios = root.join("submodules/coda-app/ios/DuetApp/Sources/DuetApp");
        let web = root.join("submodules/coda-agents/web/src/main.tsx");
        if !ios.exists() && !web.exists() {
            return;
        }
        assert!(ios.exists());
        assert!(web.exists());
    }

    #[test]
    fn the_book_cites_every_duet_doc_heading() {
        let docs = root().join("submodules/coda-app/docs");
        if !docs.exists() {
            return;
        }
        let mut corpus = String::new();
        for entry in std::fs::read_dir(root().join("book/src")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                corpus.push_str(&std::fs::read_to_string(&path).unwrap());
                corpus.push('\n');
            }
        }
        for entry in std::fs::read_dir(&docs).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            for title in headings(&text) {
                assert!(
                    corpus.contains(&title),
                    "{} missing from the book: {title}",
                    path.display()
                );
            }
        }
    }

    fn headings(text: &str) -> Vec<String> {
        let mut found = Vec::new();
        let mut fenced = false;
        for line in text.lines() {
            if line.starts_with("```") {
                fenced = !fenced;
                continue;
            }
            if fenced {
                continue;
            }
            let hashes = line.chars().take_while(|ch| *ch == '#').count();
            if (1..=6).contains(&hashes) && line[hashes..].starts_with([' ', '\t']) {
                let title = line[hashes..].trim();
                if !title.is_empty() {
                    found.push(title.to_owned());
                }
            }
        }
        found
    }
}
