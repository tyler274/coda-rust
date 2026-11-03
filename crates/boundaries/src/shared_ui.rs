//! Holds both apps to principle U5, one shared component set (`intent/PRINCIPLES.md`).
//!
//! The `TABLE` below records the raw controls, `card` family markup and CSS rules naming a library
//! class or a control element that `physician-app` and `duet-app` still have, per file and with its
//! reason. A count that moves either way fails the test, so a new one can't slip in and each move
//! to `crates/ui` shows in the diff.
//!
//! Review still covers what this can't see: a local component duplicating a shared one under a new
//! name, a card under an app's own class such as `simulation-card`, a class held in a variable, a
//! CSS rule reaching a shared control through only an app's own class, and how a screen looks.
//!
//! Each file is read alone, so a test-only module in its own file starts with `#![cfg(test)]`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use proc_macro2::{Delimiter, TokenStream, TokenTree};

use Kind::{Button, CardMarkup, CssOverride, Input, Select, Textarea};

/// What the check counts in one app file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Input,
    Select,
    Textarea,
    Button,
    CardMarkup,
    CssOverride,
}

impl Kind {
    fn label(self) -> &'static str {
        match self {
            Input => "raw `input`",
            Select => "raw `select`",
            Textarea => "raw `textarea`",
            Button => "raw `button`",
            CardMarkup => "hand-built card markup",
            CssOverride => "CSS rule overriding a library class or control",
        }
    }

    fn remedy(self) -> &'static str {
        match self {
            CardMarkup => "use the shared `Card` from `crates/ui`",
            CssOverride => "change the library or drop the override",
            _ => "use the shared component from `crates/ui`",
        }
    }
}

struct Entry {
    file: &'static str,
    kind: Kind,
    count: usize,
    reason: &'static str,
}

const TABLE: &[Entry] = &[
    Entry {
        file: "crates/duet-app/assets/duet.css",
        kind: CssOverride,
        count: 7,
        reason: "Duet restyles `.card-title` and `.empty` everywhere, the shared `NavigationRow` on its thread rows and the title inside it, and the `.tabs` and `.action-row` spacing, until the shared components carry them.",
    },
    Entry {
        file: "crates/duet-app/src/lib.rs",
        kind: Input,
        count: 10,
        reason: "The corrected value, note, visit, question and medication fields, and the medication and insight notification checkboxes, carry Duet's own labels and sit outside `TextField` and `Checkbox` until the Duet forms move.",
    },
    Entry {
        file: "crates/duet-app/src/lib.rs",
        kind: Textarea,
        count: 4,
        reason: "The note, visit, cover note and You editor bodies, which become a multiline `TextField` when the Duet forms move.",
    },
    Entry {
        file: "crates/duet-app/src/lib.rs",
        kind: Button,
        count: 1,
        reason: "Clears the selected record, styled with `.duet-retry` until it moves to `Button`.",
    },
    Entry {
        file: "crates/duet-app/src/lib.rs",
        kind: CardMarkup,
        count: 15,
        reason: "Today, You, thread and saved page preview cards put `card`, `card-title` and `card-text` on plain elements, with voice and number layouts that `Card` would have to hold before they move.",
    },
    Entry {
        file: "crates/duet-app/src/publication.rs",
        kind: Input,
        count: 1,
        reason: "The read-only link field, whose `readonly` state `TextField` doesn't offer.",
    },
    Entry {
        file: "crates/duet-app/src/publication.rs",
        kind: Select,
        count: 1,
        reason: "The link lifetime choice, which moves to the shared `Select` with the Duet forms.",
    },
    Entry {
        file: "crates/duet-app/src/records.rs",
        kind: Button,
        count: 2,
        reason: "Ask about a record and show more records, styled with `.duet-retry` until they move to `Button`.",
    },
    Entry {
        file: "crates/duet-app/src/records.rs",
        kind: CardMarkup,
        count: 1,
        reason: "A record's title uses `card-text` until records are built from `Card`.",
    },
    Entry {
        file: "crates/duet-app/src/resolution.rs",
        kind: Input,
        count: 1,
        reason: "The consent checkbox for ending a conflicting retry, which moves to `Checkbox` with the Duet forms.",
    },
    Entry {
        file: "crates/physician-app/assets/shell.css",
        kind: CssOverride,
        count: 13,
        reason: "The sidebar, dialogs and composer adjust the shared `.icon`, `.key-hint`, `.control`, `.dialog`, `.search-field`, `.dialog-body` and `.composer` to fit the physician layout, and its font reset and `button` rules reach the shared `Button` and `NavigationRow` in the sidebar and patient search, until the library carries those fits.",
    },
    Entry {
        file: "crates/physician-app/src/lib.rs",
        kind: Input,
        count: 1,
        reason: "The sign-in email, which moves to `TextField` with sign-in.",
    },
    Entry {
        file: "crates/physician-app/src/lib.rs",
        kind: Select,
        count: 1,
        reason: "The practice's physician panel choice, which moves to the shared `Select` with the remaining controls.",
    },
    Entry {
        file: "crates/physician-app/src/lib.rs",
        kind: Button,
        count: 8,
        reason: "Sign-in's six actions, plus the logo images that reopen the app and return to Today, which stay raw until the remaining controls move.",
    },
    Entry {
        file: "crates/physician-app/src/lib.rs",
        kind: CardMarkup,
        count: 2,
        reason: "The schedule rail's header uses `card-header` and `card-title` because `Card` has no collapsible rail form yet.",
    },
];

const APPS: [&str; 2] = ["physician-app", "duet-app"];

type Counts = BTreeMap<(String, Kind), usize>;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn files_with_extension(directory: &Path, extension: &str, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files_with_extension(&path, extension, found);
        } else if path.extension().is_some_and(|found| found == extension) {
            found.push(path);
        }
    }
}

fn read_all(directory: &Path, extension: &str) -> Vec<(String, String)> {
    let mut paths = Vec::new();
    files_with_extension(directory, extension, &mut paths);
    paths.sort();
    let root = root();
    paths
        .into_iter()
        .map(|path| {
            let name = path.strip_prefix(&root).unwrap().display().to_string();
            (name, std::fs::read_to_string(&path).unwrap())
        })
        .collect()
}

/// Every class the library's stylesheets name; an app rule naming one restyles the library.
fn library_classes() -> BTreeSet<String> {
    read_all(&root().join("crates/ui/assets"), "css")
        .iter()
        .flat_map(|(_, source)| css_selectors(source))
        .flat_map(|selector| selector_classes(&selector))
        .collect()
}

/// The `card` family by name, whether or not the library defines the member, so an app can't leave
/// the count by inventing `card-text`. `thread-card` is an app's own name and doesn't count.
fn is_card_class(class: &str) -> bool {
    class == "card" || class.starts_with("card-")
}

fn actual_counts() -> Counts {
    let library = library_classes();
    let mut counts = Counts::new();
    for app in APPS {
        let directory = root().join("crates").join(app);
        for (file, source) in read_all(&directory.join("src"), "rs") {
            for (kind, count) in rsx_counts(&source) {
                counts.insert((file.clone(), kind), count);
            }
        }
        for (file, source) in read_all(&directory.join("assets"), "css") {
            let count = css_override_count(&source, &library);
            if count > 0 {
                counts.insert((file, CssOverride), count);
            }
        }
    }
    counts
}

/// What each difference from the table asks of the author.
fn mismatches(actual: &Counts, table: &[Entry]) -> Vec<String> {
    let recorded: Counts = table
        .iter()
        .map(|entry| ((entry.file.to_owned(), entry.kind), entry.count))
        .collect();
    let keys: BTreeSet<&(String, Kind)> = actual.keys().chain(recorded.keys()).collect();
    let mut found = Vec::new();
    for key in keys {
        let (file, kind) = key;
        let now = actual.get(key).copied().unwrap_or(0);
        let was = recorded.get(key).copied().unwrap_or(0);
        let label = kind.label();
        if now > was {
            found.push(format!(
                "{file}: {label} went up from {was} to {now}. Principle U5 wants one shared \
                 component set: {}. If it has to stay, add the exception to TABLE in \
                 crates/boundaries/src/shared_ui.rs: raise the count and give the reason.",
                kind.remedy()
            ));
        } else if now < was {
            found.push(format!(
                "{file}: {label} went down from {was} to {now}. Lower the number to {now} in TABLE \
                 in crates/boundaries/src/shared_ui.rs, or remove the entry at 0, so it can't come \
                 back."
            ));
        }
    }
    found
}

fn rsx_counts(source: &str) -> BTreeMap<Kind, usize> {
    let stream: TokenStream = source.parse().expect("app source tokenizes");
    let mut counts = BTreeMap::new();
    scan(stream, false, &mut counts);
    counts
}

/// Tokens drop comments and keep strings whole, so neither can look like markup. Elements only
/// exist inside `rsx!`, and test items are skipped before their tokens are read.
fn scan(stream: TokenStream, in_rsx: bool, counts: &mut BTreeMap<Kind, usize>) {
    let tokens: Vec<TokenTree> = stream.into_iter().collect();
    let mut index = 0;
    while index < tokens.len() {
        match &tokens[index] {
            TokenTree::Punct(punct) if punct.as_char() == '#' => {
                let inner = is_punct(tokens.get(index + 1), '!');
                let attribute = index + 1 + usize::from(inner);
                let Some(TokenTree::Group(group)) = tokens.get(attribute) else {
                    index += 1;
                    continue;
                };
                if is_test_attribute(group.stream()) {
                    if inner {
                        return;
                    }
                    index = after_item(&tokens, attribute + 1);
                } else {
                    index = attribute + 1;
                }
                continue;
            }
            TokenTree::Ident(ident) if ident == "rsx" && is_punct(tokens.get(index + 1), '!') => {
                if let Some(TokenTree::Group(group)) = tokens.get(index + 2) {
                    scan(group.stream(), true, counts);
                    index += 3;
                    continue;
                }
            }
            TokenTree::Ident(ident) if in_rsx => {
                let name = ident.to_string();
                if let Some(kind) = control_kind(&name) {
                    if opens_element(&tokens, index) {
                        *counts.entry(kind).or_insert(0) += 1;
                    }
                } else if name == "class"
                    && is_punct(tokens.get(index + 1), ':')
                    && class_value_names_card(&tokens[index + 2..])
                {
                    *counts.entry(CardMarkup).or_insert(0) += 1;
                }
            }
            TokenTree::Group(group) => scan(group.stream(), in_rsx, counts),
            _ => {}
        }
        index += 1;
    }
}

fn control_kind(name: &str) -> Option<Kind> {
    match name {
        "input" => Some(Input),
        "select" => Some(Select),
        "textarea" => Some(Textarea),
        "button" => Some(Button),
        _ => None,
    }
}

fn is_punct(token: Option<&TokenTree>, character: char) -> bool {
    matches!(token, Some(TokenTree::Punct(punct)) if punct.as_char() == character)
}

/// `button { ... }` opens an element; `if button { ... }` tests a local of the same name.
fn opens_element(tokens: &[TokenTree], index: usize) -> bool {
    let braces_follow = matches!(
        tokens.get(index + 1),
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace
    );
    let expression_before = index
        .checked_sub(1)
        .is_some_and(|before| match &tokens[before] {
            TokenTree::Punct(punct) => !matches!(punct.as_char(), ',' | ';'),
            TokenTree::Ident(ident) => ["if", "in", "match", "while"]
                .iter()
                .any(|keyword| ident == keyword),
            _ => false,
        });
    braces_follow && !expression_before
}

/// The value runs to the next comma, and may choose between literals: `if a { "card" } else { "x" }`.
fn class_value_names_card(tokens: &[TokenTree]) -> bool {
    tokens
        .iter()
        .take_while(|token| !is_punct(Some(token), ','))
        .any(token_names_card)
}

fn token_names_card(token: &TokenTree) -> bool {
    match token {
        TokenTree::Literal(literal) => literal
            .to_string()
            .trim_matches(['"', '#', 'r'])
            .replace("\\n", " ")
            .replace("\\r", " ")
            .replace("\\t", " ")
            .split_whitespace()
            .any(is_card_class),
        TokenTree::Group(group) => group
            .stream()
            .into_iter()
            .any(|token| token_names_card(&token)),
        _ => false,
    }
}

/// `#[test]`, `#[tokio::test(...)]`, `#[wasm_bindgen_test]`, `#[cfg(test)]`, `#[cfg(all(test, ...))]`.
fn is_test_attribute(attribute: TokenStream) -> bool {
    let tokens: Vec<TokenTree> = attribute.into_iter().collect();
    if let [TokenTree::Ident(ident), TokenTree::Group(group)] = tokens.as_slice()
        && ident == "cfg"
    {
        return cfg_requires_test(group.stream());
    }
    tokens.iter().any(|token| {
        matches!(token, TokenTree::Ident(ident) if ident == "test" || ident == "wasm_bindgen_test")
    })
}

fn cfg_requires_test(predicate: TokenStream) -> bool {
    let tokens: Vec<TokenTree> = predicate.into_iter().collect();
    tokens.iter().enumerate().any(|(index, token)| match token {
        TokenTree::Ident(ident) if ident == "test" => true,
        TokenTree::Ident(ident) if ident == "all" => {
            matches!(tokens.get(index + 1), Some(TokenTree::Group(group)) if cfg_requires_test(group.stream()))
        }
        _ => false,
    })
}

/// The index past an item: past further attributes to its `;` or its braced body.
fn after_item(tokens: &[TokenTree], mut index: usize) -> usize {
    while index < tokens.len() {
        match &tokens[index] {
            TokenTree::Punct(punct) if punct.as_char() == ';' => return index + 1,
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => return index + 1,
            _ => index += 1,
        }
    }
    index
}

fn css_override_count(source: &str, library: &BTreeSet<String>) -> usize {
    css_selectors(source)
        .iter()
        .filter(|selector| {
            selector_names_control(selector)
                || selector_classes(selector)
                    .iter()
                    .any(|class| library.contains(class))
        })
        .count()
}

/// The selector of each style rule, nested in at-rules or not, with comments and strings removed.
fn css_selectors(source: &str) -> Vec<String> {
    let mut selectors = Vec::new();
    let mut prelude = String::new();
    let mut characters = source.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '/' if characters.peek() == Some(&'*') => {
                characters.next();
                let mut previous = ' ';
                for inside in characters.by_ref() {
                    if previous == '*' && inside == '/' {
                        break;
                    }
                    previous = inside;
                }
            }
            '"' | '\'' => {
                while let Some(inside) = characters.next() {
                    if inside == '\\' {
                        characters.next();
                    } else if inside == character {
                        break;
                    }
                }
            }
            '{' => {
                let selector = prelude.trim();
                if !selector.is_empty() && !selector.starts_with('@') {
                    selectors.push(selector.to_owned());
                }
                prelude.clear();
            }
            ';' | '}' => prelude.clear(),
            _ => prelude.push(character),
        }
    }
    selectors
}

/// `button`, `.host input` and `select:focus` reach a control whichever component rendered it;
/// `.button`, `#input` and `[type=button]` name something else.
fn selector_names_control(selector: &str) -> bool {
    let mut name = String::new();
    let mut before = ' ';
    let mut in_brackets = false;
    for character in selector.chars().chain([' ']) {
        if character.is_alphanumeric() || matches!(character, '-' | '_') {
            name.push(character);
            continue;
        }
        // CSS matches HTML element names in any case; RSX's `Button` is the shared component.
        if !in_brackets
            && !matches!(before, '.' | '#' | ':')
            && control_kind(&name.to_ascii_lowercase()).is_some()
        {
            return true;
        }
        name.clear();
        match character {
            '[' => in_brackets = true,
            ']' => in_brackets = false,
            _ => {}
        }
        before = character;
    }
    false
}

fn selector_classes(selector: &str) -> Vec<String> {
    let mut classes = Vec::new();
    for (index, _) in selector.match_indices('.') {
        let name: String = selector[index + 1..]
            .chars()
            .take_while(|character| character.is_alphanumeric() || matches!(character, '-' | '_'))
            .collect();
        if name.starts_with(|first: char| !first.is_ascii_digit()) {
            classes.push(name);
        }
    }
    classes
}

#[test]
fn the_apps_match_the_shared_ui_table() {
    let found = mismatches(&actual_counts(), TABLE);
    assert!(found.is_empty(), "\n{}\n", found.join("\n"));
}

#[test]
fn every_table_entry_has_one_row_and_a_reason() {
    let mut seen = BTreeSet::new();
    for entry in TABLE {
        assert!(
            seen.insert((entry.file, entry.kind)),
            "{} has two rows for {}",
            entry.file,
            entry.kind.label()
        );
        assert!(
            !entry.reason.trim().is_empty(),
            "{} needs a reason for its {}",
            entry.file,
            entry.kind.label()
        );
        assert!(
            !entry.reason.contains("ENG-"),
            "{} names a ticket in its reason; say why it stays instead",
            entry.file
        );
    }
}

/// A broken walk or stylesheet path would find nothing and pass as "all migrated" otherwise.
#[test]
fn the_check_reads_the_real_library() {
    let library = library_classes();
    assert!(library.contains("card") && library.contains("card-title"));
    assert!(!library.iter().any(|class| class == "duet"));
}

fn count(source: &str, kind: Kind) -> usize {
    rsx_counts(source).get(&kind).copied().unwrap_or(0)
}

#[test]
fn counts_each_raw_control_inside_rsx() {
    let source = r#"
        fn view() -> Element {
            rsx! {
                input { value: "a" }
                div {
                    select { option { "one" } }
                    textarea {}
                    if open { button { "Go" } }
                    for row in rows { button { "{row}" } }
                }
            }
        }
    "#;
    assert_eq!(count(source, Input), 1);
    assert_eq!(count(source, Select), 1);
    assert_eq!(count(source, Textarea), 1);
    assert_eq!(count(source, Button), 2);
}

#[test]
fn ignores_comments_strings_and_lookalikes() {
    let source = r##"
        // rsx! { button {} }
        /* rsx! { button {} } */
        /// rsx! { button {} }
        const HTML: &str = "rsx! { button {} }";
        fn view(button: bool, input: bool, inputs: Vec<u8>) -> Element {
            let select = Select { input: 1 };
            rsx! {
                // button { "commented out" }
                /* input {} */
                p { "button { }" }
                p { r#"input { }"# }
                Button { "shared" }
                Input { value: "shared" }
                if button { p {} }
                if let Some(input) = input { p {} }
                for textarea in inputs { p {} }
                match select { _ => p {} }
            }
        }
    "##;
    assert!(rsx_counts(source).is_empty());
}

#[test]
fn ignores_test_code_but_not_code_kept_out_of_tests() {
    let source = r#"
        #[cfg(test)]
        mod tests {
            fn fixture() -> Element { rsx! { button {} } }
        }
        #[test]
        fn renders() { let _ = rsx! { button {} }; }
        #[tokio::test(flavor = "current_thread")]
        async fn loads() { let _ = rsx! { button {} }; }
        #[cfg(all(test, feature = "web"))]
        fn helper() -> Element { rsx! { button {} } }
        #[cfg(test)]
        #[allow(dead_code)]
        const FIXTURE: u8 = 1;
        #[cfg(not(test))]
        fn production() -> Element { rsx! { input {} } }
        #[cfg(any(test, feature = "web"))]
        fn both() -> Element { rsx! { select {} } }
        fn after() -> Element { rsx! { textarea {} } }
    "#;
    assert_eq!(count(source, Button), 0);
    assert_eq!(count(source, Input), 1);
    assert_eq!(count(source, Select), 1);
    assert_eq!(count(source, Textarea), 1);
}

#[test]
fn a_whole_test_file_counts_nothing() {
    let source = "#![cfg(test)]\nfn view() -> Element { rsx! { button {} } }";
    assert_eq!(count(source, Button), 0);
}

#[test]
fn counts_rsx_held_in_other_macros_and_nested_rsx() {
    let source = r#"
        fn view() -> Vec<Element> {
            vec![rsx! { div { {rsx! { button {} }} } }]
        }
    "#;
    assert_eq!(count(source, Button), 1);
}

#[test]
fn counts_card_markup_by_class_family_once_per_element() {
    let source = r#"
        fn view() -> Element {
            rsx! {
                article { class: "card voice-card",
                    p { class: "card-title", "A" }
                    p { class: "card-text other", "card" }
                }
                article { class: if measured { "card one" } else { "card two" }, id: "card" }
                div { class: "thread-card" }
                div { class: "wildcard" }
                p { "card" }
                Card { title: "shared" }
            }
        }
        const CLASS: &str = "card";
    "#;
    assert_eq!(count(source, CardMarkup), 4);
}

#[test]
fn counts_card_classes_set_apart_by_escaped_whitespace() {
    let source = r#"
        fn view() -> Element {
            rsx! {
                article { class: "card\tvoice-card" }
                article { class: "voice-card\ncard-title" }
                article { class: "voice-card\rcard" }
                article { class: "voice-card\tthread-card" }
            }
        }
    "#;
    assert_eq!(count(source, CardMarkup), 3);
}

#[test]
fn counts_css_rules_naming_library_classes() {
    let library: BTreeSet<String> = ["card-title", "tabs", "empty"].map(str::to_owned).into();
    let source = r#"
        /* .card-title { color: red; } */
        .duet .tabs { gap: 1px; }
        .card-title, .empty { color: red; }
        .own-class { background: url(".tabs"); content: ".empty"; }
        [data-name=".tabs"] { margin: 0; }
        @media (min-width: 1.5em) {
            .empty:hover { color: blue; }
            .own-other { width: calc(1.5em + .5em); }
        }
        @keyframes pulse { 12.5% { opacity: 0; } }
        .own-parent { color: red; .tabs { color: blue; } }
    "#;
    assert_eq!(css_override_count(source, &library), 4);
}

#[test]
fn counts_css_rules_naming_control_elements() {
    let source = r#"
        /* button { color: red; } */
        button, input { font: inherit; }
        .dialog-results button { width: 100%; }
        button.sidebar-expand:hover { display: none; }
        .own-class > select, .own-other { margin: 0; }
        @media (min-width: 1em) { textarea:focus { color: blue; } }
        .own-class BUTTON { width: 100%; }
        Input:focus { color: blue; }
        .button, .BUTTON, .input-row, #select, [type=button], [data-x="input"] { color: red; }
        button-group, my-input { margin: 0; }
        .own-class:hover::before { content: "button"; }
        @keyframes pulse { to { opacity: 0; } }
    "#;
    assert_eq!(css_override_count(source, &BTreeSet::new()), 7);
}

fn entry(count: usize) -> Entry {
    Entry {
        file: "crates/duet-app/src/lib.rs",
        kind: Button,
        count,
        reason: "A reason",
    }
}

fn actual(count: usize) -> Counts {
    [(("crates/duet-app/src/lib.rs".to_owned(), Button), count)].into()
}

#[test]
fn a_matching_count_passes() {
    assert!(mismatches(&actual(3), &[entry(3)]).is_empty());
}

#[test]
fn a_count_going_up_says_to_use_the_shared_component_or_add_an_exception() {
    let found = mismatches(&actual(4), &[entry(3)]);
    assert_eq!(found.len(), 1);
    assert!(found[0].contains("went up from 3 to 4"));
    assert!(found[0].contains("U5"));
    assert!(found[0].contains("use the shared component"));
    assert!(found[0].contains("add the exception"));
    assert!(found[0].contains("give the reason"));
}

#[test]
fn a_new_file_counts_as_going_up_from_zero() {
    let found = mismatches(&actual(1), &[]);
    assert!(found[0].contains("went up from 0 to 1"));
}

#[test]
fn a_count_going_down_says_to_lower_the_number() {
    let found = mismatches(&actual(2), &[entry(3)]);
    assert_eq!(found.len(), 1);
    assert!(found[0].contains("went down from 3 to 2"));
    assert!(found[0].contains("Lower the number to 2"));
}

#[test]
fn a_count_reaching_zero_says_to_remove_the_entry() {
    let found = mismatches(&Counts::new(), &[entry(1)]);
    assert!(found[0].contains("went down from 1 to 0"));
    assert!(found[0].contains("remove the entry"));
}
