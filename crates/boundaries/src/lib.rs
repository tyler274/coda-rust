//! Keeps Duet and the physician client from sharing clinical crates, holds both to the shared
//! component set, and keeps the UI library's browser behaviour out of `eval`.
//!
//! The book chapter is `book/src/reading.md`.

#![deny(missing_docs)]

#[cfg(test)]
mod shared_ui;

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

    #[test]
    fn duet_screens_do_not_read_device_health_directly() {
        assert!(
            !manifest("duet-app").contains("duet-health"),
            "Duet screens must read account-owned evidence through duet-client"
        );
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
                // A crate's own tests may enable its test features via `path = "."`.
                if path == "." {
                    continue;
                }
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
        let allowed = ["harness", "agent-skills", "coda-http"];
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

    /// Gmail transport and the Spruce reader are shared by both products, so
    /// they reach no product crate, not even through a test dependency.
    #[test]
    fn gmail_and_spruce_are_product_neutral() {
        let allowed = ["gmail", "spruce"];
        for neutral in allowed {
            for dependency in local_closure(neutral) {
                assert!(
                    allowed.contains(&dependency.as_str()),
                    "{neutral} reached product-owned crate {dependency}"
                );
            }
        }
    }

    /// The clinical library serves both products, so it reaches no other
    /// local crate, not even through a test dependency.
    #[test]
    fn the_clinical_library_is_product_neutral() {
        for dependency in local_closure("clinical-library") {
            assert_eq!(
                dependency, "clinical-library",
                "clinical-library reached local crate {dependency}"
            );
        }
    }

    /// The `ui` crate's Rust sources, by path relative to its `src`.
    fn ui_sources() -> Vec<(String, String)> {
        fn visit(dir: &std::path::Path, src: &std::path::Path, found: &mut Vec<(String, String)>) {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    visit(&path, src, found);
                } else if path.extension().is_some_and(|extension| extension == "rs") {
                    let name = path.strip_prefix(src).unwrap().display().to_string();
                    found.push((name, std::fs::read_to_string(&path).unwrap()));
                }
            }
        }
        let src = root().join("crates/ui/src");
        let mut found = Vec::new();
        visit(&src, &src, &mut found);
        found
    }

    /// A source's tokens, in order, with each group's contents in place of the group: comments
    /// are gone and strings are literals, so only code is read.
    fn tokens(source: &str) -> Vec<proc_macro2::TokenTree> {
        fn flatten(stream: proc_macro2::TokenStream, found: &mut Vec<proc_macro2::TokenTree>) {
            for token in stream {
                match token {
                    proc_macro2::TokenTree::Group(group) => {
                        found.push(proc_macro2::TokenTree::Group(group.clone()));
                        flatten(group.stream(), found);
                    }
                    other => found.push(other),
                }
            }
        }
        let mut found = Vec::new();
        flatten(source.parse().expect("ui source tokenizes"), &mut found);
        found
    }

    fn is_ident(token: Option<&proc_macro2::TokenTree>, name: &str) -> bool {
        matches!(token, Some(proc_macro2::TokenTree::Ident(ident)) if ident == name)
    }

    /// A page that refuses `eval`, such as a Chrome extension's, runs the UI library only if
    /// its browser behaviour lives in `assets/behaviour.js`; `script.rs` alone may evaluate it,
    /// for the native webview. Dioxus's `eval` counts however it is reached: as
    /// `document::eval`, or imported by name.
    #[test]
    fn the_ui_library_runs_its_browser_behaviour_without_eval() {
        let sources = ui_sources();
        assert!(sources.iter().any(|(name, _)| name == "script.rs"));
        for (name, source) in &sources {
            if name == "script.rs" {
                continue;
            }
            assert!(
                !tokens(source)
                    .iter()
                    .any(|token| is_ident(Some(token), "eval")),
                "crates/ui/src/{name} uses Dioxus's eval; add the behaviour to assets/behaviour.js"
            );
        }
    }

    /// Each kind of behaviour `script.rs` runs by name is exactly what `assets/behaviour.js` lists
    /// for that kind. Only `script.rs` can name one: its `fire`, `ask` and `listen` are private.
    #[test]
    fn every_ui_behaviour_is_run_as_its_kind() {
        let behaviour =
            std::fs::read_to_string(root().join("crates/ui/assets/behaviour.js")).unwrap();
        let script = std::fs::read_to_string(root().join("crates/ui/src/script.rs")).unwrap();
        let tokens = tokens(&script);
        for (kind, list) in [("fire", "fires"), ("ask", "asks"), ("listen", "listens")] {
            let listed = behaviour
                .split_once(&format!("const {list} = {{"))
                .and_then(|(_, rest)| rest.split_once('}'))
                .unwrap_or_else(|| panic!("behaviour.js lists `const {list} = {{ ... }}`"))
                .0;
            let defined: std::collections::BTreeSet<String> = listed
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(str::to_owned)
                .collect();
            let run: std::collections::BTreeSet<String> = tokens
                .windows(3)
                .filter(|window| is_ident(window.first(), kind))
                .filter_map(|window| match &window[2] {
                    proc_macro2::TokenTree::Literal(literal) => Some(literal.to_string()),
                    _ => None,
                })
                .filter_map(|literal| {
                    literal
                        .strip_prefix('"')
                        .and_then(|named| named.strip_suffix('"'))
                        .map(str::to_owned)
                })
                .collect();
            assert!(!run.is_empty(), "the check found no `{kind}` behaviours");
            assert_eq!(
                run, defined,
                "script.rs and assets/behaviour.js disagree on which behaviours `{kind}` runs"
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
}
