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
