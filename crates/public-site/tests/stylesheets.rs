//! Rules the site's stylesheets keep: `site.js` owns motion, and the Duet story's styles
//! reach nothing outside its stage.
use std::{fs, path::PathBuf};

fn read(path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
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

/// The frame-reservation and no-script stylesheets reserve layout and never compete
/// with `site.js` motion (an override of a stage child's transition is what made
/// staging stutter).
#[test]
fn layout_stylesheets_declare_no_motion() {
    for css in ["assets/css/stage.css", "assets/css/noscript.css"] {
        let text = read(css);
        for property in ["transition", "animation", "@keyframes"] {
            assert!(
                !text.contains(property),
                "{css} must not declare {property}"
            );
        }
    }
}

/// Every duet.css selector is scoped to `.duet-story`. Its markup is its last frame,
/// so no-script needs no rules of its own for it.
#[test]
fn duet_story_styles_stay_inside_its_stage() {
    for selector in selectors(&read("assets/css/duet.css")) {
        assert!(
            selector.starts_with(".duet-story") || selector.starts_with(".compact .duet-story"),
            "duet.css: {selector}"
        );
    }
    let noscript = read("assets/css/noscript.css");
    assert!(!noscript.contains("duet-story") && !noscript.contains(".ds-"));
}
