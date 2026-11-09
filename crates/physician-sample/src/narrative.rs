//! A FHIR resource's own narrative (`text.div`, XHTML) as plain text. No markup reaches the
//! browser: tags are dropped, block elements become line breaks, entities are decoded, and the
//! content of `script` and `style` elements is never read.
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

/// XHTML as plain text: one line per block, whitespace within a line collapsed, blank lines
/// dropped.
pub(crate) fn plain_text(xhtml: &str) -> String {
    let mut text = String::new();
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
        let name = tag
            .trim_start_matches('/')
            .split(|c: char| c.is_whitespace() || c == '/')
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        // A namespace prefix such as `xhtml:p` names the same element.
        let local = name.rsplit(':').next().unwrap_or_default();
        if !closing && !tag.ends_with('/') && HIDDEN.contains(&local) {
            rest = skip_hidden(rest, local);
            continue;
        }
        if BLOCKS.contains(&local) {
            text.push('\n');
        } else if local == "td" || local == "th" {
            text.push(' ');
        }
    }
    text.push_str(rest);
    text.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
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
        ] {
            let record = json!({"text":{"status":"generated","div":div}});
            assert_eq!(narrative(&record), None, "{div:?}");
        }
        let empty = json!({"text":{"status":"empty","div":"<div>No narrative</div>"}});
        assert_eq!(narrative(&empty), None);
        assert_eq!(narrative(&json!({"text":"plain"})), None);
    }

    #[test]
    fn stray_angle_brackets_and_unknown_entities_read_as_written() {
        assert_eq!(
            plain_text("<div>K < 3.5 &unknown; &#x41;</div>"),
            "K < 3.5 &unknown; A"
        );
    }
}
