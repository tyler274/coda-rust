//! Snapshot-backed parity: never compares a generated document with itself. The
//! oracle is the original source, or what production served from it (fixtures/served).
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

/// The page production serves for a document, from the served snapshot.
fn served_page(path: &str) -> String {
    let file = match path {
        "/" => "index".to_owned(),
        "404" => "not-found".to_owned(),
        _ => path.trim_start_matches('/').to_owned(),
    };
    read(&format!(
        "crates/public-site/tests/fixtures/served/pages/{file}.html"
    ))
}
fn decode(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}
/// Head facts a browser or crawler acts on, independent of tag order and of the
/// framework: title, named metadata, and canonical/icon links. Content-addressed
/// asset prefixes are the port's serving detail, not a different resource.
fn head_facts(html: &str) -> Vec<String> {
    let head = html.split("</head>").next().unwrap();
    let mut facts = vec![format!(
        "title {}",
        decode(
            head.split("<title>")
                .nth(1)
                .unwrap()
                .split("</title>")
                .next()
                .unwrap()
        )
    )];
    for tag in head.split('<').skip(1) {
        let (name, rest) = tag.split_once(' ').unwrap_or((tag, ""));
        if name != "meta" && name != "link" {
            continue;
        }
        let rest = rest.split('>').next().unwrap();
        let mut attributes = Vec::new();
        for part in rest.split("\" ").chain(std::iter::once("")) {
            if let Some((key, value)) = part.trim().trim_end_matches('/').split_once("=\"") {
                attributes.push((key.trim().to_owned(), decode(value.trim_end_matches('"'))));
            }
        }
        let get = |key: &str| {
            attributes
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.clone())
        };
        let fact = match (name, get("name").or_else(|| get("property")), get("rel")) {
            ("meta", Some(key), _) => format!("meta {key}={}", get("content").unwrap_or_default()),
            ("link", _, Some(rel)) if rel != "stylesheet" && !rel.contains("preload") => {
                let mut href = get("href").unwrap_or_default();
                if let Some(rest) = href.strip_prefix("/a/") {
                    href = rest[rest.find('/').unwrap()..].to_owned();
                }
                format!(
                    "link {rel} {href} {} {}",
                    get("sizes").unwrap_or_default(),
                    get("type").unwrap_or_default()
                )
            }
            _ => continue,
        };
        facts.push(fact);
    }
    // Crawlers apply the union of every robots directive (Next adds its own
    // "noindex" to not-found pages), so compare that rather than the tag count.
    let mut robots: Vec<_> = facts
        .iter()
        .filter_map(|fact| fact.strip_prefix("meta robots="))
        .flat_map(|content| {
            content
                .split(',')
                .map(|directive| directive.trim().to_owned())
        })
        .collect();
    robots.sort();
    robots.dedup();
    facts.retain(|fact| !fact.starts_with("meta robots="));
    if !robots.is_empty() {
        facts.push(format!("robots {}", robots.join(",")));
    }
    facts.sort();
    facts
}
/// The port without its site-owned region (`data-owner="site"`, through its matching
/// close), and whether it had one. That region runs ahead of production on purpose.
fn without_site_owned(html: &str) -> (String, bool) {
    let Some(start) = html.find("<div data-owner=\"site\"") else {
        return (html.to_owned(), false);
    };
    let (mut depth, mut at) = (0, start);
    loop {
        let open = html[at..].find("<div").map(|i| i + at);
        let close = html[at..].find("</div>").map(|i| i + at).unwrap();
        match open {
            Some(open) if open < close => (depth, at) = (depth + 1, open + 4),
            _ => (depth, at) = (depth - 1, close + 6),
        }
        if depth == 0 {
            return (format!("{}{}", &html[..start], &html[at..]), true);
        }
    }
}
fn element(html: &str, open: &str, close: &str) -> String {
    let start = html.find(open).unwrap_or_else(|| panic!("missing {open}"));
    let rest = &html[start..];
    visible(&rest[..rest.find(close).unwrap() + close.len()])
}

/// Every document against the page production serves for it: head metadata and
/// icons, the visible text of main and footer, and which compiled stylesheet it
/// uses. The served snapshot is the oracle, never a re-derivation from source.
#[test]
fn every_document_matches_what_production_serves() {
    let pages = prerender(&Config::default());
    let provenance: Value = serde_json::from_str(&read(
        "crates/public-site/tests/fixtures/original/provenance.json",
    ))
    .unwrap();
    let served_files = provenance["served"]["files"].as_array().unwrap();
    let mut served: Vec<_> = served_files
        .iter()
        .filter_map(|entry| {
            let page = entry["path"].as_str()?.strip_prefix("pages/")?;
            Some(page.strip_suffix(".html")?.to_owned())
        })
        .collect();
    served.sort();
    let mut documents: Vec<_> = pages
        .pages
        .keys()
        .map(|path| match path.as_str() {
            "/" => "index".to_owned(),
            _ => path[1..].to_owned(),
        })
        .chain(["not-found".to_owned()])
        .collect();
    documents.sort();
    assert_eq!(
        documents, served,
        "the port serves exactly production's documents"
    );
    // The URL path of the compiled stylesheet production links on utility pages.
    let utility_css = served_files
        .iter()
        .find(|entry| entry["path"] == "utility.css")
        .and_then(|entry| entry["url"].as_str())
        .unwrap()
        .trim_start_matches(provenance["served"]["origin"].as_str().unwrap())
        .to_owned();
    for (path, page) in pages
        .pages
        .iter()
        .map(|(path, page)| (path.as_str(), page))
        .chain([("404", &pages.not_found)])
    {
        let port = std::str::from_utf8(&page.html).unwrap();
        let original = served_page(path);
        assert_eq!(head_facts(port), head_facts(&original), "{path} head");
        // The site-owned Duet story stands where production serves its original phones.
        let (port_main, site_owned) = without_site_owned(port);
        let mut original_main = element(&original, "<main", "</main>");
        if site_owned {
            let phones = visible(&read(
                "crates/public-site/tests/fixtures/original/app/_homepage/duet-phones.html",
            ));
            assert!(
                original_main.contains(&phones),
                "{path}: production serves the original Duet phones"
            );
            original_main = original_main.replacen(&phones, "", 1);
        }
        assert_eq!(
            element(&port_main, "<main", "</main>"),
            original_main,
            "{path} main"
        );
        assert_eq!(
            element(port, "<footer", "</footer>"),
            element(&original, "<footer", "</footer>"),
            "{path} footer"
        );
        assert_eq!(
            port.contains("rel=\"stylesheet\" href=\"/css/utility.css\""),
            original.contains(&format!("rel=\"stylesheet\" href=\"{utility_css}\"")),
            "{path} stylesheet"
        );
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
    // What production compiled (Tailwind) or rendered (ImageResponse), byte-for-byte.
    for (port, served) in [
        ("assets/css/utility.css", "utility.css"),
        (
            "assets/scheduling/dev/opengraph-image.png",
            "scheduling/dev/opengraph-image.png",
        ),
    ] {
        assert!(
            fs::read(root().join(format!("crates/public-site/{port}"))).unwrap()
                == fs::read(
                    root().join(format!("crates/public-site/tests/fixtures/served/{served}"))
                )
                .unwrap(),
            "{port} must equal what production serves"
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
/// Every selector of a stylesheet, comments removed and keyframe steps skipped.
fn selectors(css: &str) -> Vec<String> {
    let mut text = css.to_owned();
    while let Some(start) = text.find("/*") {
        let end = text[start..].find("*/").unwrap() + start + 2;
        text.replace_range(start..end, "");
    }
    let (mut found, mut prelude, mut skip) = (Vec::new(), String::new(), 0usize);
    for ch in text.chars() {
        match ch {
            '{' if skip > 0 => skip += 1,
            '{' if prelude.trim().starts_with("@keyframes") => skip = 1,
            '{' if !prelude.trim().starts_with('@') => {
                // Commas inside :is(...) belong to one selector.
                let mut depth = 0;
                let list = prelude.split(|c| {
                    depth += i32::from(c == '(') - i32::from(c == ')');
                    c == ',' && depth == 0
                });
                found.extend(list.map(|s| s.trim().to_owned()));
                skip = 1;
            }
            '{' => {}
            '}' => skip = skip.saturating_sub(1),
            _ if skip > 0 => {}
            _ => {
                prelude.push(ch);
                continue;
            }
        }
        prelude.clear();
    }
    found
}

/// The Duet story is site-owned and reaches nothing outside its stage: every duet.css
/// selector is scoped to `.duet-story`. Its markup is its last frame, so no-script
/// needs no rules of its own for it.
#[test]
fn duet_story_styles_stay_inside_its_stage() {
    for selector in selectors(&read("crates/public-site/assets/css/duet.css")) {
        assert!(
            selector.starts_with(".duet-story") || selector.starts_with(".compact .duet-story"),
            "duet.css: {selector}"
        );
    }
    let noscript = read("crates/public-site/assets/css/noscript.css");
    assert!(!noscript.contains("duet-story") && !noscript.contains(".ds-"));
}
