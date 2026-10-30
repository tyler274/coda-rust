//! Prose to read, after the React app's `ClinicalText` and SwiftUI Duet's `DuetMarkdownProse`: the
//! words as GitHub-flavoured Markdown (paragraphs, headings, lists, emphasis, links, quotes, code,
//! rules and tables), with the host's inline pieces, such as citations, where they were written.
//! Raw HTML is dropped, an image shows only its description, and a link opens only on a web,
//! mail or phone address.

use dioxus::prelude::*;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Where an inline piece stands in the words while they are parsed: the object replacement
/// character, which the words themselves never keep.
const INLINE: char = '\u{FFFC}';

/// One run of prose: Markdown words, or a piece the host draws at that point in them.
#[derive(Clone, PartialEq)]
pub enum ProsePart {
    /// Words, in Markdown.
    Words(String),
    /// An element in the flow of the words, such as a citation.
    Inline(Element),
}

/// Words to read, in Markdown, with the host's inline pieces among them. Markdown that runs across
/// an inline piece reads as one: a paragraph stays one paragraph around its citations.
#[component]
pub fn Prose(parts: Vec<ProsePart>) -> Element {
    let (source, inlines) = joined(&parts);
    let blocks = parse(&source);
    rsx! {
        div { class: "prose",
            for (index, node) in blocks.iter().enumerate() {
                Fragment { key: "{index}", {draw(node, &inlines)} }
            }
        }
    }
}

/// The words as one Markdown source with a marker where each inline piece goes, and the pieces in
/// order.
fn joined(parts: &[ProsePart]) -> (String, Vec<Element>) {
    let mut source = String::new();
    let mut inlines = Vec::new();
    for part in parts {
        match part {
            ProsePart::Words(words) => source.extend(words.chars().filter(|ch| *ch != INLINE)),
            ProsePart::Inline(element) => {
                source.push(INLINE);
                inlines.push(element.clone());
            }
        }
    }
    (source, inlines)
}

/// What prose is made of once parsed.
#[derive(Clone, Debug, PartialEq)]
enum Node {
    Block(Kind, Vec<Node>),
    Text(String),
    Code(String),
    /// The host's nth inline piece.
    Inline(usize),
    Break,
    Rule,
}

#[derive(Clone, Debug, PartialEq)]
enum Kind {
    Paragraph,
    Heading(u8),
    Quote,
    CodeBlock,
    List(Option<u64>),
    Item,
    Table,
    Head,
    Row,
    Cell {
        head: bool,
    },
    Emphasis,
    Strong,
    Strikethrough,
    Link(String),
    /// A link to somewhere that doesn't open from prose: its words stay, unlinked.
    Unlinked,
    Image,
    /// Anything else Markdown wraps words in: its words stay.
    Plain,
}

fn parse(source: &str) -> Vec<Node> {
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let mut stack: Vec<(Kind, Vec<Node>)> = vec![(Kind::Plain, Vec::new())];
    let mut inline = 0;
    let mut in_head = false;
    let mut skip = 0usize;
    for event in Parser::new_ext(source, options) {
        if skip > 0 {
            match event {
                Event::Start(_) => skip += 1,
                Event::End(_) => skip -= 1,
                _ => {}
            }
            continue;
        }
        match event {
            Event::Start(Tag::HtmlBlock) | Event::Start(Tag::MetadataBlock(_)) => skip = 1,
            Event::Start(Tag::FootnoteDefinition(_)) => skip = 1,
            Event::Start(tag) => {
                let kind = match tag {
                    Tag::Paragraph => Kind::Paragraph,
                    Tag::Heading { level, .. } => Kind::Heading(heading(level)),
                    Tag::BlockQuote(_) => Kind::Quote,
                    Tag::CodeBlock(_) => Kind::CodeBlock,
                    Tag::List(start) => Kind::List(start),
                    Tag::Item => Kind::Item,
                    Tag::Table(_) => Kind::Table,
                    Tag::TableHead => {
                        in_head = true;
                        Kind::Head
                    }
                    Tag::TableRow => Kind::Row,
                    Tag::TableCell => Kind::Cell { head: in_head },
                    Tag::Emphasis => Kind::Emphasis,
                    Tag::Strong => Kind::Strong,
                    Tag::Strikethrough => Kind::Strikethrough,
                    Tag::Link { dest_url, .. } if opens(&dest_url) => {
                        Kind::Link(dest_url.to_string())
                    }
                    Tag::Link { .. } => Kind::Unlinked,
                    Tag::Image { .. } => Kind::Image,
                    _ => Kind::Plain,
                };
                stack.push((kind, Vec::new()));
            }
            Event::End(end) => {
                if end == TagEnd::TableHead {
                    in_head = false;
                }
                if let Some((kind, children)) = stack.pop() {
                    let node = Node::Block(kind, children);
                    if let Some((_, parent)) = stack.last_mut() {
                        parent.push(node);
                    }
                }
            }
            Event::Text(text) => {
                let parent = &mut stack.last_mut().expect("root").1;
                for (index, run) in text.split(INLINE).enumerate() {
                    if index > 0 {
                        parent.push(Node::Inline(inline));
                        inline += 1;
                    }
                    if !run.is_empty() {
                        parent.push(Node::Text(run.to_owned()));
                    }
                }
            }
            Event::Code(code) => stack
                .last_mut()
                .expect("root")
                .1
                .push(Node::Code(code.to_string())),
            Event::SoftBreak => stack
                .last_mut()
                .expect("root")
                .1
                .push(Node::Text(" ".to_owned())),
            Event::HardBreak => stack.last_mut().expect("root").1.push(Node::Break),
            Event::Rule => stack.last_mut().expect("root").1.push(Node::Rule),
            _ => {}
        }
    }
    stack.pop().map(|(_, nodes)| nodes).unwrap_or_default()
}

fn heading(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Whether a link opens from prose: a web, mail or phone address, or a place on the same page.
fn opens(url: &str) -> bool {
    let url = url.trim();
    match url.split_once(':') {
        Some((scheme, _)) if !scheme.contains(['/', '?', '#']) => {
            matches!(
                scheme.to_ascii_lowercase().as_str(),
                "http" | "https" | "mailto" | "tel"
            )
        }
        _ => !url.is_empty(),
    }
}

fn draw(node: &Node, inlines: &[Element]) -> Element {
    match node {
        Node::Text(text) => rsx! { "{text}" },
        Node::Code(code) => rsx! { code { "data-prose": "", "{code}" } },
        Node::Inline(index) => inlines.get(*index).cloned().unwrap_or_else(|| rsx! {}),
        Node::Break => rsx! { br {} },
        Node::Rule => rsx! { hr { "data-prose": "" } },
        Node::Block(kind, children) => {
            let inner = rsx! {
                for (index, child) in children.iter().enumerate() {
                    Fragment { key: "{index}", {draw(child, inlines)} }
                }
            };
            match kind {
                Kind::Paragraph => rsx! { p { "data-prose": "", {inner} } },
                Kind::Heading(1) => rsx! { h1 { "data-prose": "", {inner} } },
                Kind::Heading(2) => rsx! { h2 { "data-prose": "", {inner} } },
                Kind::Heading(3) => rsx! { h3 { "data-prose": "", {inner} } },
                Kind::Heading(4) => rsx! { h4 { "data-prose": "", {inner} } },
                Kind::Heading(5) => rsx! { h5 { "data-prose": "", {inner} } },
                Kind::Heading(_) => rsx! { h6 { "data-prose": "", {inner} } },
                Kind::Quote => rsx! { blockquote { "data-prose": "", {inner} } },
                Kind::CodeBlock => rsx! { pre { "data-prose": "", code { {inner} } } },
                Kind::List(None) => rsx! { ul { "data-prose": "", {inner} } },
                Kind::List(Some(start)) => {
                    rsx! { ol { "data-prose": "", start: "{start}", {inner} } }
                }
                Kind::Item => rsx! { li { "data-prose": "", {inner} } },
                Kind::Table => rsx! {
                    div { class: "prose-table-scroll", table { {inner} } }
                },
                Kind::Head => rsx! { thead { tr { {inner} } } },
                Kind::Row => rsx! { tr { {inner} } },
                Kind::Cell { head: true } => rsx! { th { {inner} } },
                Kind::Cell { head: false } => rsx! { td { {inner} } },
                Kind::Emphasis => rsx! { em { "data-prose": "", {inner} } },
                Kind::Strong => rsx! { strong { "data-prose": "", {inner} } },
                Kind::Strikethrough => rsx! { del { "data-prose": "", {inner} } },
                Kind::Link(href) => rsx! { a { "data-prose": "", href: "{href}", {inner} } },
                Kind::Image => rsx! { span { class: "prose-image", {inner} } },
                Kind::Unlinked | Kind::Plain => inner,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(words: &str) -> Node {
        Node::Text(words.to_owned())
    }

    #[test]
    fn a_citation_stays_inside_its_paragraph() {
        let (source, inlines) = joined(&[
            ProsePart::Words("Potassium was **5.4**".into()),
            ProsePart::Inline(rsx! { span {} }),
            ProsePart::Words(" on Tuesday.\n\nRecheck next week.".into()),
        ]);
        assert_eq!(inlines.len(), 1);
        assert_eq!(
            parse(&source),
            vec![
                Node::Block(
                    Kind::Paragraph,
                    vec![
                        text("Potassium was "),
                        Node::Block(Kind::Strong, vec![text("5.4")]),
                        Node::Inline(0),
                        text(" on Tuesday."),
                    ]
                ),
                Node::Block(Kind::Paragraph, vec![text("Recheck next week.")]),
            ]
        );
    }

    #[test]
    fn words_never_stand_for_an_inline_piece() {
        let (source, inlines) = joined(&[ProsePart::Words("a\u{FFFC}b".into())]);
        assert!(inlines.is_empty());
        assert_eq!(
            parse(&source),
            vec![Node::Block(Kind::Paragraph, vec![text("ab")])]
        );
    }

    #[test]
    fn raw_html_and_unsafe_links_do_not_reach_the_page() {
        let nodes =
            parse("<script>x()</script>\n\n[open](javascript:x()) and [site](https://example.org)");
        assert_eq!(
            nodes,
            vec![Node::Block(
                Kind::Paragraph,
                vec![
                    Node::Block(Kind::Unlinked, vec![text("open")]),
                    text(" and "),
                    Node::Block(Kind::Link("https://example.org".into()), vec![text("site")]),
                ]
            )]
        );
    }
}
