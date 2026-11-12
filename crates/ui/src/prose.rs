//! Prose to read: the
//! words as GitHub-flavoured Markdown (paragraphs, headings, lists, emphasis, links, quotes, code,
//! rules and tables), with the host's inline pieces, such as citations, where they were written.
//! Raw HTML shows as the words it was written in and never reaches the page as markup, so nothing
//! written goes unseen. An image shows only its description, and a link opens only on a web, mail
//! or phone address. An inline piece stands outside the Markdown's emphasis, links and code around
//! it, so it is never inside a link and keeps its state when the words around it change; a piece
//! written inside raw HTML or a link's address is not drawn. Code set apart stands as a `CodeBlock`
//! under its language. Code and tables that don't fit scroll across on their own, each a stop for
//! the keys so they can scroll it too. A table's words stand in each column where its divider row
//! puts them, so figures marked `---:` line up at the end. While the words are still `streaming`
//! in, each word fades in as it arrives. A `message`, in a conversation or a card, is set smaller
//! and closer in the host's colour, and keeps its single newlines as line breaks, as someone
//! writing one means them; a `>` opening one of its lines stays as written rather than starting a
//! quote.

use dioxus::prelude::*;
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::{CodeBlock, CopyWords, opens};

/// Where an inline piece stands in the words while they are parsed: the object replacement
/// character, which the words themselves never keep. Each piece is known by where its marker
/// stands in the source, so words that decode to the same character never stand for one.
const INLINE: char = '\u{FFFC}';

const OPTIONS: Options = Options::ENABLE_TABLES.union(Options::ENABLE_STRIKETHROUGH);

/// One run of prose: Markdown words, or a piece the host draws at that point in them.
#[derive(Clone, PartialEq)]
pub enum ProsePart {
    /// Words, in Markdown.
    Words(String),
    /// An element in the flow of the words, such as a citation.
    Inline(Element),
}

/// What assistive technology calls the parts of prose that scroll on their own.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProseNames {
    /// A block of code, before its language when it has one.
    pub code: String,
    /// A table.
    pub table: String,
}

/// Words to read, in Markdown, with the host's inline pieces among them. Markdown that runs across
/// an inline piece reads as one: a paragraph stays one paragraph around its citations. While
/// `streaming`, the words are still arriving, and each new word fades in; `copy` names the control
/// that copies each block of code, and `names` says what its code and tables are called. A
/// `message` is set smaller and closer, in the colour around it, a single newline in it stands as a
/// line break rather than a space, and a `>` opening a line stays as written.
#[component]
pub fn Prose(
    parts: Vec<ProsePart>,
    names: ProseNames,
    #[props(default)] streaming: bool,
    #[props(default)] message: bool,
    copy: Option<CopyWords>,
) -> Element {
    let (source, inlines) = joined(&parts);
    let source = if message {
        quote_marks_as_words(source)
    } else {
        source
    };
    let blocks = parse(&source, &markers(&source));
    let drawing = Drawing {
        inlines,
        streaming,
        breaks: message,
        copy,
        names,
    };
    rsx! {
        div { class: if message { "prose is-message" } else { "prose" },
            for (index, node) in blocks.iter().enumerate() {
                Fragment { key: "{key(index, node)}", {draw(node, &drawing)} }
            }
        }
    }
}

/// What every node is drawn with.
struct Drawing {
    inlines: Vec<Element>,
    streaming: bool,
    breaks: bool,
    copy: Option<CopyWords>,
    names: ProseNames,
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

/// In a message, a `>` opening a line means what it says, such as a threshold, rather than a quote.
/// Markdown decides what is a quote, so each one it finds is escaped until none is left; a `>` in
/// code is never one.
fn quote_marks_as_words(mut source: String) -> String {
    while let Some(mark) = Parser::new_ext(&source, OPTIONS)
        .into_offset_iter()
        .find_map(|(event, range)| match event {
            Event::Start(Tag::BlockQuote(_)) => {
                source[range.start..].find('>').map(|at| range.start + at)
            }
            _ => None,
        })
    {
        source.insert(mark, '\\');
    }
    source
}

/// Where each inline piece's marker stands in the source, in order.
fn markers(source: &str) -> Vec<usize> {
    source.match_indices(INLINE).map(|(at, _)| at).collect()
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
    /// A single newline in the words: a space, or a line break where the host keeps them.
    Newline,
    Rule,
}

#[derive(Clone, Debug, PartialEq)]
enum Kind {
    Paragraph,
    Heading(u8),
    Quote,
    /// Code set apart, with its language when the fence names one.
    CodeBlock(Option<String>),
    List(Option<u64>),
    Item,
    Table,
    Head,
    Row,
    Cell {
        head: bool,
        align: Align,
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

/// Where a table column's words stand in its cells, as the table's divider row says: `---:` puts
/// them at the end, as figures read, and `:---:` in the middle.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Align {
    Start,
    Center,
    End,
}

impl Align {
    fn of(column: Option<&Alignment>) -> Self {
        match column {
            Some(Alignment::Right) => Align::End,
            Some(Alignment::Center) => Align::Center,
            Some(Alignment::Left | Alignment::None) | None => Align::Start,
        }
    }

    /// The cell's attribute; the start needs none.
    fn attribute(self) -> Option<&'static str> {
        match self {
            Align::Start => None,
            Align::Center => Some("center"),
            Align::End => Some("end"),
        }
    }
}

fn parse(source: &str, markers: &[usize]) -> Vec<Node> {
    let mut stack: Vec<(Kind, Vec<Node>)> = vec![(Kind::Plain, Vec::new())];
    let mut in_head = false;
    let mut columns: Vec<Alignment> = Vec::new();
    let mut column = 0usize;
    let mut skip = 0usize;
    for (event, range) in Parser::new_ext(source, OPTIONS).into_offset_iter() {
        // The pieces whose markers stand in this event's source, in order. Text decoded from an
        // entity holds none, so the character it decodes to is dropped.
        let mut pieces = markers.partition_point(|at| *at < range.start)
            ..markers.partition_point(|at| *at < range.end);
        if skip > 0 {
            match event {
                Event::Start(_) => skip += 1,
                Event::End(_) => skip -= 1,
                _ => {}
            }
            continue;
        }
        match event {
            Event::Start(Tag::MetadataBlock(_)) => skip = 1,
            Event::Start(Tag::FootnoteDefinition(_)) => skip = 1,
            Event::Start(tag) => {
                let kind = match tag {
                    Tag::Paragraph | Tag::HtmlBlock => Kind::Paragraph,
                    Tag::Heading { level, .. } => Kind::Heading(heading(level)),
                    Tag::BlockQuote(_) => Kind::Quote,
                    Tag::CodeBlock(CodeBlockKind::Fenced(info)) => {
                        Kind::CodeBlock(info.split_whitespace().next().map(str::to_owned))
                    }
                    Tag::CodeBlock(_) => Kind::CodeBlock(None),
                    Tag::List(start) => Kind::List(start),
                    Tag::Item => Kind::Item,
                    Tag::Table(alignments) => {
                        columns = alignments;
                        Kind::Table
                    }
                    Tag::TableHead => {
                        in_head = true;
                        column = 0;
                        Kind::Head
                    }
                    Tag::TableRow => {
                        column = 0;
                        Kind::Row
                    }
                    Tag::TableCell => {
                        let align = Align::of(columns.get(column));
                        column += 1;
                        Kind::Cell {
                            head: in_head,
                            align,
                        }
                    }
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
                if let Some((kind, mut children)) = stack.pop()
                    && let Some((_, parent)) = stack.last_mut()
                {
                    // Raw HTML's last line ends with a newline that separates nothing.
                    if end == TagEnd::HtmlBlock && children.last() == Some(&Node::Newline) {
                        children.pop();
                    }
                    parent.extend(around_pieces(kind, children));
                }
            }
            Event::Text(text) => {
                let parent = &mut stack.last_mut().expect("root").1;
                split(&text, &mut pieces, parent, |run| Node::Text(run.to_owned()));
            }
            Event::Code(code) => {
                let parent = &mut stack.last_mut().expect("root").1;
                split(&code, &mut pieces, parent, |run| Node::Code(run.to_owned()));
            }
            // Words, never markup; a piece's marker inside it goes undrawn.
            Event::Html(html) | Event::InlineHtml(html) => {
                let words: String = html.chars().filter(|ch| *ch != INLINE).collect();
                let line = words.trim_end_matches('\n');
                let parent = &mut stack.last_mut().expect("root").1;
                if !line.is_empty() {
                    parent.push(Node::Text(line.to_owned()));
                }
                if line.len() < words.len() {
                    parent.push(Node::Newline);
                }
            }
            Event::SoftBreak => stack.last_mut().expect("root").1.push(Node::Newline),
            Event::HardBreak => stack.last_mut().expect("root").1.push(Node::Break),
            Event::Rule => stack.last_mut().expect("root").1.push(Node::Rule),
            _ => {}
        }
    }
    stack.pop().map(|(_, nodes)| nodes).unwrap_or_default()
}

/// Words with the event's pieces where their markers stand; a marker with no piece is dropped.
fn split(
    words: &str,
    pieces: &mut std::ops::Range<usize>,
    parent: &mut Vec<Node>,
    run: impl Fn(&str) -> Node,
) {
    for (index, words) in words.split(INLINE).enumerate() {
        if index > 0
            && let Some(piece) = pieces.next()
        {
            parent.push(Node::Inline(piece));
        }
        if !words.is_empty() {
            parent.push(run(words));
        }
    }
}

/// A finished node, with any inline piece among its children stood outside the emphasis, link,
/// code or image description around it: the wrapper closes before the piece and opens again after
/// it. An image's description is plain words.
fn around_pieces(kind: Kind, children: Vec<Node>) -> Vec<Node> {
    let children = match kind {
        Kind::Image => described(children),
        _ => children,
    };
    let wraps = matches!(
        kind,
        Kind::Emphasis
            | Kind::Strong
            | Kind::Strikethrough
            | Kind::Link(_)
            | Kind::CodeBlock(_)
            | Kind::Image
    );
    if !wraps
        || !children
            .iter()
            .any(|child| matches!(child, Node::Inline(_)))
    {
        return vec![Node::Block(kind, children)];
    }
    let mut nodes = Vec::new();
    let mut run = Vec::new();
    for child in children {
        match child {
            Node::Inline(piece) => {
                if !run.is_empty() {
                    nodes.push(Node::Block(kind.clone(), std::mem::take(&mut run)));
                }
                nodes.push(match kind {
                    Kind::CodeBlock(_) => Node::Block(Kind::Paragraph, vec![Node::Inline(piece)]),
                    _ => Node::Inline(piece),
                });
            }
            child => run.push(child),
        }
    }
    if !run.is_empty() {
        nodes.push(Node::Block(kind, run));
    }
    nodes
}

/// Words without their Markdown, keeping the inline pieces among them.
fn described(nodes: Vec<Node>) -> Vec<Node> {
    nodes
        .into_iter()
        .flat_map(|node| match node {
            Node::Text(words) | Node::Code(words) => vec![Node::Text(words)],
            Node::Block(_, children) => described(children),
            Node::Break | Node::Newline => vec![Node::Text(" ".to_owned())],
            Node::Inline(piece) => vec![Node::Inline(piece)],
            Node::Rule => Vec::new(),
        })
        .collect()
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

/// A child's key: an inline piece by which piece it is, so it keeps its state when the words
/// before it change; anything else by where it stands.
fn key(index: usize, node: &Node) -> String {
    match node {
        Node::Inline(piece) => format!("piece-{piece}"),
        _ => index.to_string(),
    }
}

fn draw(node: &Node, drawing: &Drawing) -> Element {
    match node {
        // Each word its own span, kept by where it stands, so only a word that arrives fades in.
        Node::Text(text) if drawing.streaming => rsx! {
            for (index, word) in text.split_inclusive(char::is_whitespace).enumerate() {
                span { key: "{index}", class: "prose-word", "{word}" }
            }
        },
        Node::Text(text) => rsx! { "{text}" },
        Node::Code(code) => rsx! { code { "data-prose": "", "{code}" } },
        Node::Inline(index) => drawing
            .inlines
            .get(*index)
            .cloned()
            .unwrap_or_else(|| rsx! {}),
        Node::Break => rsx! { br {} },
        Node::Newline if drawing.breaks => rsx! { br {} },
        Node::Newline => rsx! { " " },
        Node::Rule => rsx! { hr { "data-prose": "" } },
        Node::Block(Kind::CodeBlock(language), children) => rsx! {
            CodeBlock {
                code: described(children.clone())
                    .iter()
                    .map(|node| match node {
                        Node::Text(text) => text.as_str(),
                        _ => "",
                    })
                    .collect::<String>()
                    .trim_end_matches('\n')
                    .to_owned(),
                name: drawing.names.code.clone(),
                label: language.clone(),
                copy: drawing.copy.clone(),
            }
        },
        Node::Block(kind, children) => {
            let inner = rsx! {
                for (index, child) in children.iter().enumerate() {
                    Fragment { key: "{key(index, child)}", {draw(child, drawing)} }
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
                Kind::List(None) => rsx! { ul { "data-prose": "", {inner} } },
                Kind::List(Some(start)) => {
                    rsx! { ol { "data-prose": "", start: "{start}", {inner} } }
                }
                Kind::Item => rsx! { li { "data-prose": "", {inner} } },
                Kind::Table => rsx! {
                    div {
                        class: "prose-table-scroll table-frame",
                        role: "region",
                        aria_label: "{drawing.names.table}",
                        tabindex: "0",
                        table { "data-prose": "", {inner} }
                    }
                },
                Kind::Head => rsx! { thead { tr { {inner} } } },
                Kind::Row => rsx! { tr { {inner} } },
                Kind::Cell { head: true, align } => {
                    rsx! { th { "data-prose": "", "data-align": align.attribute(), {inner} } }
                }
                Kind::Cell { head: false, align } => {
                    rsx! { td { "data-prose": "", "data-align": align.attribute(), {inner} } }
                }
                Kind::Emphasis => rsx! { em { "data-prose": "", {inner} } },
                Kind::Strong => rsx! { strong { "data-prose": "", {inner} } },
                Kind::Strikethrough => rsx! { del { "data-prose": "", {inner} } },
                Kind::Link(href) => rsx! { a { "data-prose": "", href: "{href}", {inner} } },
                Kind::Image => rsx! { span { class: "prose-image", {inner} } },
                Kind::Unlinked | Kind::Plain | Kind::CodeBlock(_) => inner,
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
            parse(&source, &markers(&source)),
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
            parse(&source, &markers(&source)),
            vec![Node::Block(Kind::Paragraph, vec![text("ab")])]
        );
    }

    #[test]
    fn raw_html_shows_as_words_and_unsafe_links_do_not_reach_the_page() {
        let nodes = parse(
            "<script>x()</script>\n\nThe <clinic-note> [open](javascript:x()) and [site](https://example.org)",
            &[],
        );
        assert_eq!(
            nodes,
            vec![
                Node::Block(Kind::Paragraph, vec![text("<script>x()</script>")]),
                Node::Block(
                    Kind::Paragraph,
                    vec![
                        text("The "),
                        text("<clinic-note>"),
                        text(" "),
                        Node::Block(Kind::Unlinked, vec![text("open")]),
                        text(" and "),
                        Node::Block(Kind::Link("https://example.org".into()), vec![text("site")]),
                    ]
                ),
            ]
        );
    }

    #[test]
    fn raw_html_across_lines_keeps_its_lines_apart() {
        assert_eq!(
            parse("<div>\nFirst line\nSecond line\n</div>", &[]),
            vec![Node::Block(
                Kind::Paragraph,
                vec![
                    text("<div>"),
                    Node::Newline,
                    text("First line"),
                    Node::Newline,
                    text("Second line"),
                    Node::Newline,
                    text("</div>"),
                ]
            )]
        );
    }

    fn drawn_message(words: &'static str) -> String {
        #[derive(Clone, PartialEq, Props)]
        struct Words {
            words: &'static str,
        }
        fn app(props: Words) -> Element {
            rsx! {
                Prose {
                    message: true,
                    names: names(),
                    parts: vec![ProsePart::Words(props.words.to_owned())],
                }
            }
        }
        let mut dom = VirtualDom::new_with_props(app, Words { words });
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn a_message_keeps_what_markdown_would_read_as_a_quote() {
        for words in [
            "> 180 mg/dL: call",
            ">= 200 mg/dL\n> 180 mg/dL",
            "   > 180 mg/dL",
            "- > 180 mg/dL",
            "> > 180 mg/dL",
        ] {
            let html = drawn_message(words);
            assert!(!html.contains("<blockquote"), "{words}: {html}");
            let shown = html.matches("&#62;").count() + html.matches("&gt;").count();
            assert_eq!(
                shown,
                words.matches('>').count(),
                "{words} lost a mark: {html}"
            );
        }
    }

    #[test]
    fn a_message_keeps_the_marks_in_code_as_written() {
        let html = drawn_message("```\n> 180\n```");
        assert!(!html.contains('\\'), "{html}");
    }

    #[test]
    fn prose_that_is_not_a_message_still_reads_a_quote() {
        assert_eq!(
            parse("> Feels well", &[]),
            vec![Node::Block(
                Kind::Quote,
                vec![Node::Block(Kind::Paragraph, vec![text("Feels well")])]
            )]
        );
    }

    #[test]
    fn raw_html_reaches_the_page_escaped() {
        fn app() -> Element {
            rsx! {
                Prose {
                    names: names(),
                    parts: vec![ProsePart::Words("<script>x()</script>\n\nThe <img src=x onerror=y> dose".into())],
                }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(!html.contains("<script"), "{html}");
        assert!(!html.contains("<img"), "{html}");
        assert!(
            html.contains("&#60;script") || html.contains("&lt;script"),
            "{html}"
        );
    }

    #[test]
    fn a_piece_written_inside_raw_html_is_not_drawn() {
        assert_eq!(
            parsed(&[
                ProsePart::Words("Before <b title=\"".into()),
                piece(),
                ProsePart::Words("\"> after ".into()),
                piece(),
            ]),
            vec![Node::Block(
                Kind::Paragraph,
                vec![
                    text("Before "),
                    text("<b title=\"\">"),
                    text(" after "),
                    Node::Inline(1),
                ]
            )]
        );
    }

    #[test]
    fn a_single_newline_stands_as_a_break_only_in_a_message() {
        fn drawn(message: bool) -> String {
            #[derive(Clone, PartialEq, Props)]
            struct Words {
                message: bool,
            }
            fn app(words: Words) -> Element {
                rsx! {
                    Prose {
                        names: names(),
                        message: words.message,
                        parts: vec![ProsePart::Words("Line one\nLine two".into())],
                    }
                }
            }
            let mut dom = VirtualDom::new_with_props(app, Words { message });
            dom.rebuild_in_place();
            dioxus_ssr::render(&dom)
        }
        assert!(drawn(true).contains("Line one<br"), "{}", drawn(true));
        assert!(
            drawn(false).contains("Line one Line two"),
            "{}",
            drawn(false)
        );
    }

    #[test]
    fn a_citation_marker_the_host_has_not_drawn_stays_as_its_words() {
        let words: String = described(parse("[[patient_source|a|b]] stays", &[]))
            .iter()
            .map(|node| match node {
                Node::Text(words) => words.as_str(),
                _ => "",
            })
            .collect();
        assert_eq!(words, "[[patient_source|a|b]] stays");
    }

    fn piece() -> ProsePart {
        ProsePart::Inline(rsx! { span {} })
    }

    fn parsed(parts: &[ProsePart]) -> Vec<Node> {
        let (source, _) = joined(parts);
        parse(&source, &markers(&source))
    }

    #[test]
    fn each_piece_stands_where_the_host_put_it() {
        // An entity that decodes to the marker character is words, not a piece, and code across a
        // piece doesn't move the pieces after it.
        assert_eq!(
            parsed(&[
                ProsePart::Words("Uncited &#xFFFC; cited ".into()),
                piece(),
                ProsePart::Words(" and `co".into()),
                piece(),
                ProsePart::Words("de` ".into()),
                piece(),
            ]),
            vec![Node::Block(
                Kind::Paragraph,
                vec![
                    text("Uncited "),
                    text(" cited "),
                    Node::Inline(0),
                    text(" and "),
                    Node::Code("co".into()),
                    Node::Inline(1),
                    Node::Code("de".into()),
                    text(" "),
                    Node::Inline(2),
                ]
            )]
        );
    }

    #[test]
    fn a_piece_stands_outside_links_emphasis_and_code_blocks() {
        assert_eq!(
            parsed(&[
                ProsePart::Words("[guidance ".into()),
                piece(),
                ProsePart::Words("](https://example.org) **firm ".into()),
                piece(),
                ProsePart::Words(" dose**\n\n```\nlisinopril".into()),
                piece(),
                ProsePart::Words("\n```".into()),
            ]),
            vec![
                Node::Block(
                    Kind::Paragraph,
                    vec![
                        Node::Block(
                            Kind::Link("https://example.org".into()),
                            vec![text("guidance ")]
                        ),
                        Node::Inline(0),
                        text(" "),
                        Node::Block(Kind::Strong, vec![text("firm ")]),
                        Node::Inline(1),
                        Node::Block(Kind::Strong, vec![text(" dose")]),
                    ]
                ),
                Node::Block(Kind::CodeBlock(None), vec![text("lisinopril")]),
                Node::Block(Kind::Paragraph, vec![Node::Inline(2)]),
                Node::Block(Kind::CodeBlock(None), vec![text("\n")]),
            ]
        );
    }

    #[test]
    fn a_fenced_block_keeps_its_language() {
        assert_eq!(
            parse("```rust title\nfn main() {}\n```", &[]),
            vec![Node::Block(
                Kind::CodeBlock(Some("rust".into())),
                vec![text("fn main() {}\n")]
            )]
        );
    }

    #[test]
    fn an_image_is_described_in_plain_words_with_its_pieces_beside_it() {
        assert_eq!(
            parsed(&[
                ProsePart::Words("[![**scan** ".into()),
                piece(),
                ProsePart::Words(
                    "](image.png)](https://example.org)\n\n![see [here](https://a.org)](i.png)"
                        .into()
                ),
            ]),
            vec![
                Node::Block(
                    Kind::Paragraph,
                    vec![
                        Node::Block(
                            Kind::Link("https://example.org".into()),
                            vec![Node::Block(Kind::Image, vec![text("scan"), text(" ")])]
                        ),
                        Node::Inline(0),
                    ]
                ),
                Node::Block(
                    Kind::Paragraph,
                    vec![Node::Block(Kind::Image, vec![text("see "), text("here")])]
                ),
            ]
        );
    }

    #[test]
    fn a_table_column_keeps_where_its_divider_puts_it() {
        let cell = |head, align, words| Node::Block(Kind::Cell { head, align }, vec![text(words)]);
        assert_eq!(
            parse(
                "| Test | Result | Flag |\n| --- | ---: | :---: |\n| Sodium | 138 | |",
                &[]
            ),
            vec![Node::Block(
                Kind::Table,
                vec![
                    Node::Block(
                        Kind::Head,
                        vec![
                            cell(true, Align::Start, "Test"),
                            cell(true, Align::End, "Result"),
                            cell(true, Align::Center, "Flag"),
                        ]
                    ),
                    Node::Block(
                        Kind::Row,
                        vec![
                            cell(false, Align::Start, "Sodium"),
                            cell(false, Align::End, "138"),
                            Node::Block(
                                Kind::Cell {
                                    head: false,
                                    align: Align::Center
                                },
                                vec![]
                            ),
                        ]
                    ),
                ]
            )]
        );
    }

    fn names() -> ProseNames {
        ProseNames {
            code: "Code".into(),
            table: "Table".into(),
        }
    }

    #[test]
    fn code_and_tables_are_named_stops_for_the_keys() {
        fn app() -> Element {
            rsx! {
                Prose {
                    names: names(),
                    parts: vec![ProsePart::Words(
                        "```rust\nfn main() {}\n```\n\n| Date |\n| --- |\n| Today |".into(),
                    )],
                }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert_eq!(html.matches(r#"role="region""#).count(), 2, "{html}");
        assert_eq!(html.matches(r#"tabindex="0""#).count(), 2, "{html}");
        assert!(html.contains(r#"aria-label="Code: rust""#), "{html}");
        assert!(html.contains(r#"aria-label="Table""#), "{html}");
    }

    static MOUNTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    static BOLD: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    #[component]
    fn Counted() -> Element {
        use_hook(|| MOUNTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
        rsx! { span {} }
    }

    #[test]
    fn a_piece_keeps_its_state_when_the_words_before_it_change() {
        fn app() -> Element {
            let words = if BOLD.load(std::sync::atomic::Ordering::SeqCst) {
                "**Alpha** "
            } else {
                "Alpha "
            };
            rsx! {
                Prose { names: names(), parts: vec![ProsePart::Words(words.into()), ProsePart::Inline(rsx! { Counted {} })] }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        assert_eq!(MOUNTS.load(std::sync::atomic::Ordering::SeqCst), 1);
        BOLD.store(true, std::sync::atomic::Ordering::SeqCst);
        dom.mark_dirty(ScopeId::APP);
        dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
        assert_eq!(MOUNTS.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
