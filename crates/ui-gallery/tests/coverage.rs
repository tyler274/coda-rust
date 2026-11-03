//! Holds the gallery to its promise of rendering every public `ui` component.

use std::collections::BTreeSet;
use std::path::PathBuf;

/// Public components the gallery doesn't place itself, and why.
const NOT_SHOWN: [(&str, &str); 0] = [];

#[test]
fn every_public_component_is_in_the_gallery() {
    let public = public_components();
    assert!(
        public.contains("Button") && public.contains("Toast"),
        "the scan found neither a component of lib.rs nor of a module: {public:?}"
    );
    let gallery = read("crates/ui-gallery/src/main.rs");
    let shown = |name: &str| placed(&gallery, name);

    let missing: Vec<&str> = public
        .iter()
        .map(String::as_str)
        .filter(|name| !shown(name) && !NOT_SHOWN.iter().any(|(excepted, _)| excepted == name))
        .collect();
    assert!(
        missing.is_empty(),
        "public ui components missing from the gallery: {missing:?}. Show each in \
         crates/ui-gallery/src/main.rs, or name it in NOT_SHOWN with the reason."
    );

    for (name, reason) in NOT_SHOWN {
        assert!(
            !reason.trim().is_empty(),
            "{name} is excepted with no reason"
        );
        assert!(public.contains(name), "{name} is excepted but isn't public");
        assert!(!shown(name), "{name} is excepted but the gallery shows it");
    }
}

#[test]
fn the_scan_tells_a_placed_component_from_a_similar_name() {
    let gallery = "// Spinner { }\nrsx! { ui::Stylesheet { } TryAgain {\n } IconTiles { } MyAvatar { } }\nuse ui::Avatar;";
    assert!(placed(gallery, "Stylesheet"));
    assert!(placed(gallery, "TryAgain"));
    assert!(!placed(gallery, "Spinner"), "a comment places nothing");
    assert!(!placed(gallery, "IconTile"), "a longer name places nothing");
    assert!(!placed(gallery, "Avatar"), "an import places nothing");

    let source = "#[component]\n#[allow(unused)]\npub fn Shown(a: u8) -> Element {}\n\
                  #[component]\nfn Private() -> Element {}\npub fn plain() {}\n";
    assert_eq!(components(source), ["Shown"]);
}

/// Components `ui` lets a host name: those in `lib.rs`, and those of its private modules it
/// re-exports.
fn public_components() -> BTreeSet<String> {
    let lib = read("crates/ui/src/lib.rs");
    let exports = identifiers(&re_exports(&lib));
    let mut public: BTreeSet<String> = components(&lib).into_iter().collect();
    let modules = std::fs::read_dir(root().join("crates/ui/src")).expect("the ui sources");
    for module in modules {
        let path = module.expect("a ui source").path();
        if path.file_name().is_some_and(|name| name == "lib.rs") {
            continue;
        }
        let source = std::fs::read_to_string(&path).expect("a readable ui source");
        public.extend(
            components(&source)
                .into_iter()
                .filter(|name| exports.contains(name)),
        );
    }
    public
}

/// The names `pub fn` declares straight after a `#[component]` and any further attributes.
fn components(source: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut lines = source.lines().map(str::trim);
    while let Some(line) = lines.next() {
        if line != "#[component]" {
            continue;
        }
        let declared = lines.by_ref().find(|line| !line.starts_with("#["));
        if let Some(name) = declared.and_then(|line| line.strip_prefix("pub fn ")) {
            found.push(identifiers(name).swap_remove(0));
        }
    }
    found
}

/// The text of every `pub use … ;` in `source`.
fn re_exports(source: &str) -> String {
    source
        .split("pub use ")
        .skip(1)
        .filter_map(|rest| rest.split_once(';'))
        .map(|(names, _)| names)
        .collect::<Vec<_>>()
        .join(" ")
}

fn identifiers(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Whether `source` writes `name { … }`, the way `rsx!` places a component, outside a comment.
fn placed(source: &str, name: &str) -> bool {
    source
        .lines()
        .map(|line| line.split("//").next().unwrap_or_default())
        .any(|line| {
            line.match_indices(name).any(|(start, _)| {
                let before = line[..start].chars().next_back();
                let after = line[start + name.len()..].trim_start();
                !before.is_some_and(|c| c.is_alphanumeric() || c == '_') && after.starts_with('{')
            })
        })
}

fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).expect("a readable source")
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
