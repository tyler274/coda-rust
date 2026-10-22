//! Source-backed parity: never compares a generated document with itself.
use public_site::{Config, prerender};
use serde_json::Value;
use std::{fs, path::PathBuf, process::Command};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn read(path: &str) -> String {
    fs::read_to_string(root().join(path))
        .unwrap_or_else(|e| panic!("required committed source snapshot {path}: {e}"))
}
fn visible(html: &str) -> String {
    let mut result = String::new();
    let mut rest = html;
    while !rest.is_empty() {
        if rest.starts_with("<!--") {
            rest = &rest[rest.find("-->").unwrap() + 3..];
        } else if rest.starts_with('<') {
            rest = &rest[rest.find('>').unwrap() + 1..];
        } else {
            let end = rest.find('<').unwrap_or(rest.len());
            result.push_str(&rest[..end]);
            rest = &rest[end..];
        }
    }
    for (entity, value) in [
        ("&amp;", "&"),
        ("&nbsp;", " "),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&#39;", "'"),
        ("&apos;", "'"),
    ] {
        result = result.replace(entity, value);
    }
    while let Some(start) = result.find("&#") {
        let end = result[start..].find(';').unwrap() + start;
        let digits = &result[start + 2..end];
        let number = if let Some(hex) = digits.strip_prefix('x') {
            u32::from_str_radix(hex, 16)
        } else {
            digits.parse()
        }
        .unwrap();
        result.replace_range(start..=end, &char::from_u32(number).unwrap().to_string());
    }
    result.chars().filter(|ch| !ch.is_whitespace()).collect()
}
#[test]
fn original_source_snapshots_match_reviewed_provenance() {
    let output = Command::new("node")
        .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/import-snapshots.mjs"))
        .arg("--check")
        .output()
        .expect("Node from the dev shell");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn homepage_visible_copy_equals_reference_including_phone_demos() {
    let source = read("crates/public-site/tests/fixtures/original/app/_homepage/main.html")
        .replace(
            "<!-- include duet-phones.html -->",
            &read("crates/public-site/tests/fixtures/original/app/_homepage/duet-phones.html"),
        );
    let pages = prerender(&Config::default());
    let html = std::str::from_utf8(&pages.pages["/"].html).unwrap();
    let main = html
        .split("<main id=\"main\"")
        .nth(1)
        .unwrap()
        .split_once('>')
        .unwrap()
        .1
        .split("</main>")
        .next()
        .unwrap();
    assert_eq!(visible(main), visible(&source));
}
#[test]
fn duet_and_scheduling_preserve_source_visible_copy() {
    let pages = prerender(&Config::default());
    for (path, file) in [
        ("/duet", "app/duet/page.tsx"),
        ("/scheduling/dev", "app/scheduling/dev/page.tsx"),
    ] {
        let source = read(&format!(
            "crates/public-site/tests/fixtures/original/{file}"
        ));
        let rendered = visible(std::str::from_utf8(&pages.pages[path].html).unwrap());
        let main = source
            .split("<main>")
            .nth(1)
            .unwrap()
            .split("</main>")
            .next()
            .unwrap();
        let mut count = 0;
        for after in main.split('>').skip(1) {
            let text = after.split('<').next().unwrap().trim();
            if text.is_empty() || text.contains(['{', '}']) {
                continue;
            }
            assert!(
                rendered.contains(&visible(text)),
                "{path} missing source text {text}"
            );
            count += 1;
        }
        assert!(count >= 4, "source extraction failed for {path}");
        if path == "/duet" {
            for line in source.lines().filter(|line| {
                line.trim().starts_with("title:") || line.trim().starts_with("body:")
            }) {
                let value = line.split_once('"').unwrap().1.rsplit_once('"').unwrap().0;
                assert!(
                    rendered.contains(&visible(value)),
                    "missing Duet principle: {value}"
                );
            }
        }
    }
}

#[test]
fn legal_data_and_archives_equal_clean_source_and_render_all_sections() {
    let output = Command::new("node")
        .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/reference.mjs"))
        .output()
        .expect("Node from the dev shell");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let source: Value = serde_json::from_slice(&output.stdout).unwrap();
    for (key, file) in [
        ("current", "current.json"),
        ("archives", "archives.json"),
        ("notices", "clinical-terminology-notices.json"),
    ] {
        let port: Value =
            serde_json::from_str(&read(&format!("crates/public-site/data/{file}"))).unwrap();
        assert_eq!(port, source[key], "{file} diverged from source");
    }
    let pages = prerender(&Config::default());
    for (path, sections) in [
        ("/privacy", &source["current"]["coda"]["privacySections"]),
        ("/terms", &source["current"]["coda"]["termsSections"]),
        (
            "/duet/privacy",
            &source["current"]["duet"]["duetPrivacySections"],
        ),
        (
            "/duet/terms",
            &source["current"]["duet"]["duetTermsSections"],
        ),
        (
            "/duet/consumer-health-data",
            &source["current"]["duet"]["duetConsumerHealthDataSections"],
        ),
        (
            "/clinical-transparency",
            &source["current"]["clinical"]["sections"],
        ),
    ] {
        let html = visible(std::str::from_utf8(&pages.pages[path].html).unwrap());
        assert_strings_rendered(sections, &html, path);
    }
    for (kind, key) in [
        ("privacy", "duetPrivacyArchive"),
        ("terms", "duetTermsArchive"),
    ] {
        for archive in source["archives"][key].as_array().unwrap() {
            let path = format!("/duet/{kind}/{}", archive["slug"].as_str().unwrap());
            let html = visible(std::str::from_utf8(&pages.pages[&path].html).unwrap());
            assert_strings_rendered(&archive["sections"], &html, &path);
        }
    }
}
fn assert_strings_rendered(value: &Value, html: &str, path: &str) {
    match value {
        Value::String(text) if text.starts_with("https://") => {
            let host = text
                .strip_prefix("https://")
                .unwrap()
                .split('/')
                .next()
                .unwrap();
            assert!(
                html.contains(&visible(&format!("Source and license: {host}"))),
                "{path}: missing license link label"
            );
        }
        Value::String(text) => assert!(
            html.contains(&visible(text)),
            "{path} missing source copy: {text}"
        ),
        Value::Array(items) => {
            for item in items {
                assert_strings_rendered(item, html, path);
            }
        }
        Value::Object(fields) => {
            for (key, item) in fields {
                if key != "id" && key != "href" {
                    assert_strings_rendered(item, html, path);
                }
            }
        }
        _ => {}
    }
}
/// Motion parity holds by construction: the original styles and the original
/// motion script (which also paints the landscapes) ship byte-for-byte.
#[test]
fn original_styles_and_motion_script_ship_verbatim() {
    for (port, source) in [
        ("assets/css/chrome.css", "app/chrome.css"),
        ("assets/css/site.css", "app/_homepage/site.css"),
        ("assets/homepage/site.js", "public/homepage/site.js"),
    ] {
        assert_eq!(
            read(&format!("crates/public-site/{port}")),
            read(&format!(
                "crates/public-site/tests/fixtures/original/{source}"
            )),
            "{port} must equal the original {source}"
        );
    }
    // Site-owned stylesheets may reserve layout, never compete with site.css motion
    // (an override of a stage child's transition is what made staging stutter).
    for css in ["assets/css/stage.css", "assets/css/noscript.css"] {
        let text = read(&format!("crates/public-site/{css}"));
        for property in ["transition", "animation", "@keyframes"] {
            assert!(
                !text.contains(property),
                "{css} must not declare {property}"
            );
        }
    }
}
