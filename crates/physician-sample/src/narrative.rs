//! A FHIR resource's own narrative (`text.div`, XHTML) as plain text. No markup reaches the
//! browser: tags are dropped, block elements become line breaks, entities are decoded, and the
//! content of `script` and `style` elements is never read.
//!
//! Inline elements whose meaning plain concatenation would lose read as marked text, so the
//! copy says what the record says:
//!
//! - A superscript reads as `^` and its text, and a subscript as `_` and its text:
//!   `10<sup>9</sup>/L` reads `10^9/L`, `CO<sub>2</sub>` reads `CO_2`. Text that is not one run
//!   of letters and digits, or that runs into a following letter or digit, is parenthesised:
//!   `10<sup>-3</sup>` reads `10^(-3)` and `H<sub>2</sub>O` reads `H_(2)O`.
//! - Struck-out text (`del`, `s`, `strike`) reads as `[struck out: …]`. It is marked rather
//!   than dropped: the record still holds it, and a reader comparing the copy with the record
//!   should find every word, but never one the record withdrew reading as current.
//!
//! - An inline quotation (`q`) reads inside the quotation marks a browser would show, so words
//!   the record attributes to someone else do not read as its own.
//!
//! Other inline elements keep their meaning as text: inserted text (`ins`) is current text, and
//! emphasis, links and code only style it. A line break (`br`) already starts a new line.
use serde_json::Value;

/// The record's narrative as plain text, when it carries one with readable content. A
/// narrative whose `text.status` is `empty` says it has nothing to read, and one that is only
/// markup or whitespace reads as nothing.
pub(crate) fn narrative(record: &Value) -> Option<String> {
    let text = &record["text"];
    if text["status"] == "empty" {
        return None;
    }
    let plain = plain_text(text["div"].as_str()?);
    (!plain.is_empty()).then_some(plain)
}

/// Elements whose content is never text to read.
const HIDDEN: [&str; 2] = ["script", "style"];

/// Elements that start a new line of text.
const BLOCKS: [&str; 22] = [
    "p",
    "div",
    "br",
    "li",
    "ul",
    "ol",
    "tr",
    "table",
    "thead",
    "tbody",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "blockquote",
    "pre",
    "dt",
    "dd",
    "dl",
    "hr",
];

/// An inline element whose meaning its text alone would lose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Marked {
    Superscript,
    Subscript,
    StruckOut,
    Quotation,
}

impl Marked {
    fn of(name: &str) -> Option<Self> {
        match name {
            "sup" => Some(Self::Superscript),
            "sub" => Some(Self::Subscript),
            "del" | "s" | "strike" => Some(Self::StruckOut),
            "q" => Some(Self::Quotation),
            _ => None,
        }
    }

    /// The element's `content` as it reads in place, followed by `next`.
    fn read(self, content: &str, next: Option<char>) -> String {
        let content = content.split_whitespace().collect::<Vec<_>>().join(" ");
        if content.is_empty() {
            return String::new();
        }
        let mark = match self {
            Self::StruckOut => return format!("[struck out: {content}]"),
            Self::Quotation => return format!("\u{201c}{content}\u{201d}"),
            Self::Superscript => '^',
            Self::Subscript => '_',
        };
        let one_run = content.chars().all(|c| c.is_ascii_alphanumeric());
        if one_run && !next.is_some_and(char::is_alphanumeric) {
            format!("{mark}{content}")
        } else {
            format!("{mark}({content})")
        }
    }
}

/// Closes the marked elements opened at `open[from..]`, innermost first, rewriting the text
/// each one holds as it reads.
fn close(text: &mut String, open: &mut Vec<(Marked, usize)>, from: usize, next: Option<char>) {
    while open.len() > from {
        let Some((marked, start)) = open.pop() else {
            return;
        };
        let content = text.split_off(start);
        text.push_str(&marked.read(&content, next));
    }
}

/// XHTML as plain text: one line per block, whitespace within a line collapsed, blank lines
/// dropped, and marked inline elements read as the module describes.
pub(crate) fn plain_text(xhtml: &str) -> String {
    let mut text = String::new();
    // Marked elements still open, each with where its text starts.
    let mut open: Vec<(Marked, usize)> = Vec::new();
    let mut rest = xhtml;
    while let Some(at) = rest.find(['<', '&']) {
        text.push_str(&rest[..at]);
        rest = &rest[at..];
        if let Some(entity) = rest.strip_prefix('&') {
            let (decoded, after) = decode_entity(entity);
            text.push_str(&decoded);
            rest = after;
            continue;
        }
        if let Some(comment) = rest.strip_prefix("<!--") {
            rest = comment.find("-->").map_or("", |end| &comment[end + 3..]);
            continue;
        }
        let Some(end) = tag_end(rest) else {
            // A `<` that opens no tag is text.
            text.push('<');
            rest = &rest[1..];
            continue;
        };
        let tag = &rest[1..end];
        rest = &rest[end + 1..];
        let closing = tag.starts_with('/');
        let name = local_name(tag);
        let local = name.as_str();
        if !closing && !tag.ends_with('/') && HIDDEN.contains(&local) {
            rest = skip_hidden(rest, local);
            continue;
        }
        if let Some(marked) = Marked::of(local) {
            if !closing && !tag.ends_with('/') {
                open.push((marked, text.len()));
            } else if closing && let Some(at) = open.iter().rposition(|(kind, _)| *kind == marked) {
                // Closing it also closes any marked element left open inside it.
                close(&mut text, &mut open, at, next_text(rest));
            }
            continue;
        }
        if BLOCKS.contains(&local) {
            text.push('\n');
        } else if local == "td" || local == "th" {
            text.push(' ');
        }
    }
    text.push_str(rest);
    // An element never closed runs to the end, and still reads as marked.
    close(&mut text, &mut open, 0, None);
    text.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// The first character of text that `markup` reads as on the same line: past inline tags, with
/// an entity decoded. `None` at a line break or the end, where nothing runs on.
fn next_text(mut markup: &str) -> Option<char> {
    loop {
        if let Some(entity) = markup.strip_prefix('&') {
            return decode_entity(entity).0.chars().next();
        }
        if !markup.starts_with('<') {
            return markup.chars().next();
        }
        let end = tag_end(markup)?;
        let local = local_name(&markup[1..end]);
        if BLOCKS.contains(&local.as_str()) || local == "td" || local == "th" {
            return None;
        }
        markup = &markup[end + 1..];
    }
}

/// The element a tag's text (between `<` and `>`) names, lowercased. A namespace prefix such
/// as `xhtml:p` names the same element.
fn local_name(tag: &str) -> String {
    let name = tag
        .trim_start_matches('/')
        .split(|c: char| c.is_whitespace() || c == '/')
        .next()
        .unwrap_or_default();
    name.rsplit(':')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
}

/// The index of the `>` closing the tag that `markup` opens, skipping quoted attribute values.
fn tag_end(markup: &str) -> Option<usize> {
    let mut quote = None;
    for (at, character) in markup.char_indices().skip(1) {
        match (quote, character) {
            (Some(open), _) if character == open => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'') => quote = Some(character),
            (None, '>') => return Some(at),
            (None, '<') => return None,
            (None, _) => {}
        }
    }
    None
}

/// What follows the closing tag of the hidden element `name`; nothing if it never closes.
fn skip_hidden<'a>(markup: &'a str, name: &str) -> &'a str {
    let lower = markup.to_ascii_lowercase();
    let mut from = 0;
    while let Some(found) = lower[from..].find("</") {
        let start = from + found + 2;
        let tag = lower[start..].split('>').next().unwrap_or_default();
        let closes = tag.trim().rsplit(':').next() == Some(name);
        if closes && let Some(end) = tag_end(&markup[start - 2..]) {
            return &markup[start - 2 + end + 1..];
        }
        from = start;
    }
    ""
}

/// The text an entity reference stands for, and what follows it. An unknown or unterminated
/// reference reads as the `&` it starts with.
fn decode_entity(after_ampersand: &str) -> (String, &str) {
    let literal = ("&".to_owned(), after_ampersand);
    let Some(end) = after_ampersand
        .char_indices()
        .take(12)
        .find(|(_, character)| *character == ';')
        .map(|(at, _)| at)
    else {
        return literal;
    };
    let name = &after_ampersand[..end];
    let rest = &after_ampersand[end + 1..];
    let named = match name {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        "nbsp" => Some('\u{a0}'),
        "ndash" => Some('–'),
        "mdash" => Some('—'),
        "middot" => Some('·'),
        "deg" => Some('°'),
        "plusmn" => Some('±'),
        "micro" => Some('µ'),
        "times" => Some('×'),
        "le" => Some('≤'),
        "ge" => Some('≥'),
        _ => None,
    };
    let numeric = || {
        let digits = name.strip_prefix('#')?;
        let code = match digits.strip_prefix(['x', 'X']) {
            Some(hex) => u32::from_str_radix(hex, 16).ok()?,
            None => digits.parse().ok()?,
        };
        char::from_u32(code)
    };
    match named.or_else(numeric) {
        Some(character) => (character.to_string(), rest),
        None => literal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn markup_reads_as_lines_of_plain_text() {
        let div = r#"<div xmlns="http://www.w3.org/1999/xhtml"><p>Hemoglobin A1c <b>7.2 %</b></p><p>Reference&#160;range 4.0&ndash;5.6</p><ul><li>Fasting</li><li>Repeat in 3 months &amp; review</li></ul><br/>Seen by <a href="Practitioner/1">Dr.  Lab</a></div>"#;
        assert_eq!(
            plain_text(div),
            "Hemoglobin A1c 7.2 %\nReference range 4.0–5.6\nFasting\nRepeat in 3 months & review\nSeen by Dr. Lab"
        );
    }

    #[test]
    fn script_and_style_content_is_never_read() {
        let div = r#"<div><style>p { color: red }</style><p>Stable</p><script type="text/javascript">alert("<p>x</p>")</script><SCRIPT>steal()</SCRIPT> on review</div>"#;
        assert_eq!(plain_text(div), "Stable\non review");
    }

    #[test]
    fn a_table_reads_one_row_per_line() {
        let div = "<div><table><tr><th>Test</th><th>Value</th></tr><tr><td>Sodium</td><td>139 mmol/L</td></tr></table></div>";
        assert_eq!(plain_text(div), "Test Value\nSodium 139 mmol/L");
    }

    #[test]
    fn an_empty_or_markup_only_narrative_reads_as_nothing() {
        for div in [
            "",
            "   ",
            "<div xmlns=\"http://www.w3.org/1999/xhtml\"></div>",
            "<div>\n  <p> </p>\n</div>",
            "<div><!-- nothing --></div>",
            "<div><p>&nbsp;&#160;</p><br/></div>",
            "<div><script>alert(1)</script><style>p{}</style></div>",
        ] {
            let record = json!({"text":{"status":"generated","div":div}});
            assert_eq!(narrative(&record), None, "{div:?}");
        }
        let empty = json!({"text":{"status":"empty","div":"<div>No narrative</div>"}});
        assert_eq!(narrative(&empty), None);
        assert_eq!(narrative(&json!({"text":"plain"})), None);
    }

    #[test]
    fn superscripts_and_subscripts_read_marked() {
        for (div, read) in [
            (
                "<p>Platelets 150 × 10<sup>9</sup>/L</p>",
                "Platelets 150 × 10^9/L",
            ),
            ("<p>10<sup>-3</sup> mol</p>", "10^(-3) mol"),
            ("<p>10<sup> 12 </sup></p>", "10^12"),
            ("<p>pCO<sub>2</sub> 41 mmHg</p>", "pCO_2 41 mmHg"),
            ("<p>H<sub>2</sub>O</p>", "H_(2)O"),
            ("<p>H<sub>2</sub><b>O</b></p>", "H_(2)O"),
            ("<p>x<sup>2</sup>&amp;y</p>", "x^2&y"),
            ("<p>m<sup>2</sup></p><p>B</p>", "m^2\nB"),
            ("<p>e<sup>i<sub>k</sub></sup></p>", "e^(i_k)"),
            ("<p>10<sup>9</p>", "10^9"),
            ("<p>A<sup></sup>B</p>", "AB"),
        ] {
            assert_eq!(plain_text(div), read, "{div}");
        }
    }

    #[test]
    fn struck_out_text_never_reads_as_current() {
        for (div, read) in [
            (
                "<p>Dose <del>20 mg</del> <ins>10 mg</ins> daily</p>",
                "Dose [struck out: 20 mg] 10 mg daily",
            ),
            (
                "<p><s>Penicillin</s> <STRIKE>Sulfa</STRIKE></p>",
                "[struck out: Penicillin] [struck out: Sulfa]",
            ),
            (
                "<p><del>Old <b>value</b>\n 7</del></p>",
                "[struck out: Old value 7]",
            ),
            ("<p><xhtml:del>5</xhtml:del></p>", "[struck out: 5]"),
            ("<p><del>  </del>kept</p>", "kept"),
            ("<p>no <del>close", "no [struck out: close]"),
        ] {
            assert_eq!(plain_text(div), read, "{div}");
        }
    }

    #[test]
    fn a_quotation_reads_in_quotation_marks() {
        assert_eq!(
            plain_text("<p>Patient reports <q>no pain today</q>.</p>"),
            "Patient reports \u{201c}no pain today\u{201d}."
        );
    }

    #[test]
    fn stray_angle_brackets_and_unknown_entities_read_as_written() {
        assert_eq!(
            plain_text("<div>K < 3.5 &unknown; &#x41;</div>"),
            "K < 3.5 &unknown; A"
        );
    }
}
