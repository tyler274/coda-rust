//! Typed paths to JSON values inside structured source records.

use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// One step from the root of a JSON record to a child value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Step {
    /// An object member by its exact key.
    Member(String),
    /// An array element by its zero-based position.
    Index(usize),
}

/// The exact path to a JSON value inside a structured source record.
///
/// Serializes and displays using canonical dotted/bracketed notation such as
/// `valueQuantity.value`, `component[0].code.text`, or `["key.with.dots"][1]`.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourcePath(Vec<Step>);

/// Why a string could not be parsed as a [`SourcePath`].
#[derive(Debug)]
pub enum SourcePathError {
    /// Input started with `.` or contained an empty dotted segment (`a..b`, `a.`).
    EmptyMember {
        /// Byte offset where a member name was expected.
        offset: usize,
    },
    /// Input had an unexpected character outside a quoted bracketed key.
    UnexpectedChar {
        /// Byte offset of the unexpected character.
        offset: usize,
        /// The offending character.
        found: char,
    },
    /// A `[` bracket was not closed by a matching `]`.
    UnclosedBracket {
        /// Byte offset where `[` opened.
        offset: usize,
    },
    /// A quoted member key inside `[...]` was not valid JSON string syntax.
    InvalidQuotedMember {
        /// Byte offset of the quoted key.
        offset: usize,
        /// Underlying JSON string decode error.
        source: serde_json::Error,
    },
    /// An array index inside `[...]` was not a valid `usize`.
    InvalidIndex {
        /// Byte offset of the index digits.
        offset: usize,
        /// Underlying integer parse error.
        source: ParseIntError,
    },
}

impl fmt::Display for SourcePathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMember { offset } => {
                write!(f, "empty member segment in source path at byte {offset}")
            }
            Self::UnexpectedChar { offset, found } => {
                write!(
                    f,
                    "unexpected character `{found}` in source path at byte {offset}"
                )
            }
            Self::UnclosedBracket { offset } => {
                write!(f, "unclosed `[` in source path at byte {offset}")
            }
            Self::InvalidQuotedMember { offset, source } => {
                write!(
                    f,
                    "invalid quoted member in source path at byte {offset}: {source}"
                )
            }
            Self::InvalidIndex { offset, source } => {
                write!(
                    f,
                    "invalid array index in source path at byte {offset}: {source}"
                )
            }
        }
    }
}

impl std::error::Error for SourcePathError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidQuotedMember { source, .. } => Some(source),
            Self::InvalidIndex { source, .. } => Some(source),
            Self::EmptyMember { .. }
            | Self::UnexpectedChar { .. }
            | Self::UnclosedBracket { .. } => None,
        }
    }
}

impl SourcePath {
    /// The root of the JSON document (`""`).
    #[must_use]
    pub const fn root() -> Self {
        Self(Vec::new())
    }

    /// True when this path points to the root value rather than a child field or element.
    #[must_use]
    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a child path addressing object member `key`.
    #[must_use]
    pub fn member(&self, key: &str) -> Self {
        let mut next = self.clone();
        next.push_member(key);
        next
    }

    /// Returns a child path addressing array element `index`.
    #[must_use]
    pub fn index(&self, index: usize) -> Self {
        let mut next = self.clone();
        next.push_index(index);
        next
    }

    /// Appends an object member step in place.
    pub fn push_member(&mut self, key: &str) {
        self.0.push(Step::Member(key.to_owned()));
    }

    /// Appends an array index step in place.
    pub fn push_index(&mut self, index: usize) {
        self.0.push(Step::Index(index));
    }

    /// The ordered steps from the document root.
    #[must_use]
    pub fn steps(&self) -> &[Step] {
        &self.0
    }

    /// Whether `self` is `prefix` or a descendant inside it.
    #[must_use]
    pub fn is_under(&self, prefix: &Self) -> bool {
        self.0.starts_with(&prefix.0)
    }

    /// Steps of `self` below `prefix`, or empty when `self` is not a strict descendant of `prefix`.
    #[must_use]
    pub fn below<'a>(&'a self, prefix: &Self) -> &'a [Step] {
        if self.is_under(prefix) {
            &self.0[prefix.0.len()..]
        } else {
            &[]
        }
    }

    /// Resolves this path against a JSON document, returning `None` when any step is absent.
    #[must_use]
    pub fn resolve<'a>(&self, record: &'a Value) -> Option<&'a Value> {
        let mut current = record;
        for step in &self.0 {
            current = match (current, step) {
                (Value::Object(map), Step::Member(key)) => map.get(key)?,
                (Value::Array(items), Step::Index(index)) => items.get(*index)?,
                _ => return None,
            };
        }
        Some(current)
    }

    /// Every value in `record` paired with its exact path, in pre-order traversal.
    #[must_use]
    pub fn nodes(record: &Value) -> Vec<(Self, &Value)> {
        fn visit<'a>(path: SourcePath, value: &'a Value, out: &mut Vec<(SourcePath, &'a Value)>) {
            out.push((path.clone(), value));
            match value {
                Value::Object(map) => {
                    for (key, child) in map {
                        visit(path.member(key), child, out);
                    }
                }
                Value::Array(items) => {
                    for (index, child) in items.iter().enumerate() {
                        visit(path.index(index), child, out);
                    }
                }
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
            }
        }
        let mut out = Vec::new();
        visit(Self::root(), record, &mut out);
        out
    }
}

fn is_bare_key_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
}

fn is_bare_key(key: &str) -> bool {
    !key.is_empty() && key.chars().all(is_bare_key_char)
}

impl fmt::Display for SourcePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (position, step) in self.0.iter().enumerate() {
            match step {
                Step::Member(key) if is_bare_key(key) => {
                    if position > 0 {
                        f.write_str(".")?;
                    }
                    f.write_str(key)?;
                }
                Step::Member(key) => {
                    f.write_str("[\"")?;
                    for ch in key.chars() {
                        match ch {
                            '"' => f.write_str("\\\"")?,
                            '\\' => f.write_str("\\\\")?,
                            '\n' => f.write_str("\\n")?,
                            '\r' => f.write_str("\\r")?,
                            '\t' => f.write_str("\\t")?,
                            c if c.is_control() => write!(f, "\\u{:04x}", c as u32)?,
                            c => write!(f, "{c}")?,
                        }
                    }
                    f.write_str("\"]")?;
                }
                Step::Index(index) => {
                    write!(f, "[{index}]")?;
                }
            }
        }
        Ok(())
    }
}

impl FromStr for SourcePath {
    type Err = SourcePathError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Ok(Self::root());
        }
        let mut steps = Vec::new();
        let mut cursor = 0usize;
        let bytes = input.as_bytes();
        while cursor < bytes.len() {
            let remaining = &input[cursor..];
            if cursor == 0 {
                if remaining.starts_with('.') {
                    return Err(SourcePathError::EmptyMember { offset: 0 });
                }
                if remaining.starts_with('[') {
                    let (step, next) = parse_bracket_step(input, cursor)?;
                    steps.push(step);
                    cursor = next;
                } else {
                    let (step, next) = parse_bare_member(input, cursor)?;
                    steps.push(step);
                    cursor = next;
                }
            } else if remaining.starts_with('.') {
                let member_start = cursor + 1;
                if member_start >= bytes.len() {
                    return Err(SourcePathError::EmptyMember {
                        offset: member_start,
                    });
                }
                let (step, next) = parse_bare_member(input, member_start)?;
                steps.push(step);
                cursor = next;
            } else if remaining.starts_with('[') {
                let (step, next) = parse_bracket_step(input, cursor)?;
                steps.push(step);
                cursor = next;
            } else {
                let found = remaining.chars().next().unwrap_or('\0');
                return Err(SourcePathError::UnexpectedChar {
                    offset: cursor,
                    found,
                });
            }
        }
        Ok(Self(steps))
    }
}

fn parse_bare_member(input: &str, start: usize) -> Result<(Step, usize), SourcePathError> {
    let tail = &input[start..];
    let mut len = 0usize;
    for ch in tail.chars() {
        if is_bare_key_char(ch) {
            len += ch.len_utf8();
        } else if ch == '.' || ch == '[' {
            break;
        } else {
            return Err(SourcePathError::UnexpectedChar {
                offset: start + len,
                found: ch,
            });
        }
    }
    if len == 0 {
        return Err(SourcePathError::EmptyMember { offset: start });
    }
    Ok((
        Step::Member(input[start..start + len].to_owned()),
        start + len,
    ))
}

fn parse_bracket_step(
    input: &str,
    bracket_offset: usize,
) -> Result<(Step, usize), SourcePathError> {
    let content_start = bracket_offset + 1;
    let Some(first) = input[content_start..].chars().next() else {
        return Err(SourcePathError::UnclosedBracket {
            offset: bracket_offset,
        });
    };
    if first == '"' {
        let mut cursor = content_start + 1;
        let mut escaped = false;
        let bytes = input.as_bytes();
        while cursor < bytes.len() {
            let ch = input[cursor..].chars().next().unwrap_or('\0');
            cursor += ch.len_utf8();
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == '"' {
                if input[cursor..].starts_with(']') {
                    let quoted = &input[content_start..cursor];
                    let decoded = serde_json::from_str::<String>(quoted).map_err(|source| {
                        SourcePathError::InvalidQuotedMember {
                            offset: content_start,
                            source,
                        }
                    })?;
                    return Ok((Step::Member(decoded), cursor + 1));
                }
                let found = input[cursor..].chars().next().unwrap_or('\0');
                if found == '\0' {
                    return Err(SourcePathError::UnclosedBracket {
                        offset: bracket_offset,
                    });
                }
                return Err(SourcePathError::UnexpectedChar {
                    offset: cursor,
                    found,
                });
            }
        }
        return Err(SourcePathError::UnclosedBracket {
            offset: bracket_offset,
        });
    }
    if first.is_ascii_digit() {
        let mut len = 0usize;
        for ch in input[content_start..].chars() {
            if ch.is_ascii_digit() {
                len += 1;
            } else if ch == ']' {
                let digits = &input[content_start..content_start + len];
                let index =
                    digits
                        .parse::<usize>()
                        .map_err(|source| SourcePathError::InvalidIndex {
                            offset: content_start,
                            source,
                        })?;
                return Ok((Step::Index(index), content_start + len + 1));
            } else {
                return Err(SourcePathError::UnexpectedChar {
                    offset: content_start + len,
                    found: ch,
                });
            }
        }
        return Err(SourcePathError::UnclosedBracket {
            offset: bracket_offset,
        });
    }
    Err(SourcePathError::UnexpectedChar {
        offset: content_start,
        found: first,
    })
}

impl Serialize for SourcePath {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for SourcePath {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        raw.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{SourcePath, Step};

    #[test]
    fn canonical_paths_roundtrip_and_resolve_values() {
        let record = json!({
            "resourceType": "Observation",
            "valueQuantity": { "value": 5.8, "unit": "mmol/L" },
            "component": [
                { "code": { "coding": [{ "display": "Systolic" }] }, "valueQuantity": { "value": 128 } },
                { "code": { "coding": [{ "display": "Diastolic" }] }, "valueQuantity": { "value": 82 } }
            ],
            "extension.map": { "quote\"key": ["present"] },
            "": "empty-member"
        });

        let cases = [
            (SourcePath::root(), "", Some(&record)),
            (
                SourcePath::root().member("valueQuantity").member("value"),
                "valueQuantity.value",
                Some(&json!(5.8)),
            ),
            (
                SourcePath::root()
                    .member("component")
                    .index(1)
                    .member("code")
                    .member("coding")
                    .index(0)
                    .member("display"),
                "component[1].code.coding[0].display",
                Some(&json!("Diastolic")),
            ),
            (
                SourcePath::root()
                    .member("extension.map")
                    .member("quote\"key")
                    .index(0),
                r#"["extension.map"]["quote\"key"][0]"#,
                Some(&json!("present")),
            ),
            (
                SourcePath::root().member(""),
                r#"[""]"#,
                Some(&json!("empty-member")),
            ),
        ];

        for (path, rendered, expected) in cases {
            assert_eq!(path.to_string(), rendered);
            let parsed: SourcePath = rendered.parse().expect("canonical path must parse");
            assert_eq!(parsed, path);
            let encoded = serde_json::to_value(&path).expect("serialize path");
            assert_eq!(encoded, json!(rendered));
            let decoded: SourcePath = serde_json::from_value(encoded).expect("deserialize path");
            assert_eq!(decoded, path);
            assert_eq!(path.resolve(&record), expected);
        }

        let missing = SourcePath::root().member("component").index(5);
        assert_eq!(missing.resolve(&record), None);

        let prefix = SourcePath::root().member("component").index(1);
        let child = prefix.member("valueQuantity").member("value");
        assert!(child.is_under(&prefix));
        assert_eq!(
            child.below(&prefix),
            &[
                Step::Member("valueQuantity".into()),
                Step::Member("value".into())
            ]
        );
    }

    #[test]
    fn malformed_paths_are_rejected() {
        for invalid in [
            ".leading",
            "trailing.",
            "double..dot",
            "dot.[0]",
            "unclosed[0",
            "empty[]",
            "negative[-1]",
            "unquoted[key]",
            r#"unclosed["key]"#,
            r#"trailing["key"]extra"#,
            "space key",
        ] {
            assert!(
                invalid.parse::<SourcePath>().is_err(),
                "expected `{invalid}` to fail SourcePath::from_str"
            );
        }
    }
}
