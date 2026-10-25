//! JSON that reads and writes exactly like the Python pipeline did.
//!
//! The bank's files (ledgers, NDJSON charts, manifests, seeds) are compared
//! byte for byte with what Python's `json` module produced, so this module keeps
//! object keys in insertion order, keeps integers apart from floats, parses floats
//! exactly, and serializes the way `json.dumps` does: `repr`-style floats,
//! optional `ensure_ascii` escaping, and the compact, default and indented
//! layouts. It lives here rather than in workspace-wide `serde_json` features,
//! which would change every other crate's JSON.

use std::fmt::Write as _;

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

static NULL: Json = Json::Null;

impl Json {
    pub fn obj() -> Json {
        Json::Obj(Vec::new())
    }

    /// Member lookup; `Null` when absent or not an object (like `dict.get`).
    pub fn get(&self, key: &str) -> &Json {
        match self {
            Json::Obj(m) => m.iter().find(|(k, _)| k == key).map_or(&NULL, |(_, v)| v),
            _ => &NULL,
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Json> {
        match self {
            Json::Obj(m) => m.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        matches!(self, Json::Obj(m) if m.iter().any(|(k, _)| k == key))
    }

    /// Insert or replace, keeping the original position on replace (like a dict).
    pub fn set(&mut self, key: &str, value: Json) {
        if let Json::Obj(m) = self {
            if let Some(slot) = m.iter_mut().find(|(k, _)| k == key) {
                slot.1 = value;
            } else {
                m.push((key.to_string(), value));
            }
        } else {
            panic!("set on a non-object");
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<Json> {
        if let Json::Obj(m) = self {
            let at = m.iter().position(|(k, _)| k == key)?;
            return Some(m.remove(at).1);
        }
        None
    }

    pub fn as_str(&self) -> Option<&str> {
        if let Json::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// The value as a number, for either JSON number kind (Python's `(int, float)`).
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Json::Int(i) => Some(*i as f64),
            Json::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        if let Json::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> &[Json] {
        if let Json::Arr(a) = self { a } else { &[] }
    }

    pub fn as_object(&self) -> &[(String, Json)] {
        if let Json::Obj(m) = self { m } else { &[] }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }

    /// Python truthiness: empty strings, containers, zero and null are false.
    pub fn truthy(&self) -> bool {
        match self {
            Json::Null => false,
            Json::Bool(b) => *b,
            Json::Int(i) => *i != 0,
            Json::Float(f) => *f != 0.0,
            Json::Str(s) => !s.is_empty(),
            Json::Arr(a) => !a.is_empty(),
            Json::Obj(m) => !m.is_empty(),
        }
    }
}

impl From<&str> for Json {
    fn from(s: &str) -> Json {
        Json::Str(s.to_string())
    }
}
impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::Str(s)
    }
}
impl From<i64> for Json {
    fn from(i: i64) -> Json {
        Json::Int(i)
    }
}
impl From<f64> for Json {
    fn from(f: f64) -> Json {
        Json::Float(f)
    }
}
impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Bool(b)
    }
}
impl From<Vec<Json>> for Json {
    fn from(a: Vec<Json>) -> Json {
        Json::Arr(a)
    }
}

/// Build an ordered object: `obj![("a", 1i64), ("b", "x")]`.
#[macro_export]
macro_rules! obj {
    ($(($k:expr, $v:expr)),* $(,)?) => {
        $crate::json::Json::Obj(vec![$(($k.to_string(), $crate::json::Json::from($v))),*])
    };
}

// ---------------------------------------------------------------- parsing

/// A `json.JSONDecodeError`: Python's message and position, in characters.
#[derive(Debug)]
pub struct ParseError {
    /// Byte offset into the text.
    pub at: usize,
    pub message: String,
    /// Python's `pos`, `lineno` and `colno`.
    pub pos: usize,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    /// `str(JSONDecodeError)`: `msg: line L column C (char P)`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: line {} column {} (char {})",
            self.message, self.line, self.column, self.pos
        )
    }
}

impl std::error::Error for ParseError {}

/// `json.loads(text)`: what it accepts, and its error message and position where it
/// does not. Integers outside i64 and lone surrogate escapes are the exceptions:
/// the first is an error here, the second reads as U+FFFD.
pub fn parse(text: &str) -> Result<Json, ParseError> {
    let p = Parser { s: text.as_bytes() };
    if text.starts_with('\u{feff}') {
        return Err(p.err(0, "Unexpected UTF-8 BOM (decode using utf-8-sig)"));
    }
    let start = p.ws(0);
    let (v, end) = p.scan(start).map_err(|e| p.expecting_value(e))?;
    let end = p.ws(end);
    if end != p.s.len() {
        return Err(p.err(end, "Extra data"));
    }
    Ok(v)
}

/// The scanner's two failures: `StopIteration(idx)` (no value starts at `idx`),
/// or a decode error.
enum Fail {
    NoValue(usize),
    Error(ParseError),
}

impl From<ParseError> for Fail {
    fn from(e: ParseError) -> Fail {
        Fail::Error(e)
    }
}

struct Parser<'a> {
    s: &'a [u8],
}

impl Parser<'_> {
    fn err(&self, at: usize, message: &str) -> ParseError {
        let before = String::from_utf8_lossy(&self.s[..at]);
        let pos = before.chars().count();
        let line = before.matches('\n').count() + 1;
        let column = before
            .rfind('\n')
            .map_or(pos + 1, |nl| before[nl + 1..].chars().count() + 1);
        ParseError {
            at,
            message: message.to_string(),
            pos,
            line,
            column,
        }
    }

    fn expecting_value(&self, f: Fail) -> ParseError {
        match f {
            Fail::NoValue(at) => self.err(at, "Expecting value"),
            Fail::Error(e) => e,
        }
    }

    fn at(&self, i: usize) -> Option<u8> {
        self.s.get(i).copied()
    }

    /// The end of the whitespace run at `i`.
    fn ws(&self, mut i: usize) -> usize {
        while matches!(self.at(i), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            i += 1;
        }
        i
    }

    /// `scan_once(s, idx)`: a value and the index after it.
    fn scan(&self, i: usize) -> Result<(Json, usize), Fail> {
        let rest = &self.s[i.min(self.s.len())..];
        match rest.first() {
            None => return Err(Fail::NoValue(i)),
            Some(b'"') => return Ok(self.string(i + 1).map(|(s, end)| (Json::Str(s), end))?),
            Some(b'{') => return self.object(i + 1),
            Some(b'[') => return self.array(i + 1),
            _ => {}
        }
        for (lit, v) in [
            (&b"null"[..], Json::Null),
            (b"true", Json::Bool(true)),
            (b"false", Json::Bool(false)),
        ] {
            if rest.starts_with(lit) {
                return Ok((v, i + lit.len()));
            }
        }
        if let Some(found) = self.number(i)? {
            return Ok(found);
        }
        for (lit, f) in [
            (&b"NaN"[..], f64::NAN),
            (b"Infinity", f64::INFINITY),
            (b"-Infinity", f64::NEG_INFINITY),
        ] {
            if rest.starts_with(lit) {
                return Ok((Json::Float(f), i + lit.len()));
            }
        }
        Err(Fail::NoValue(i))
    }

    /// `(-?(?:0|[1-9][0-9]*))(\.[0-9]+)?([eE][-+]?[0-9]+)?` matched at `i`.
    fn number(&self, i: usize) -> Result<Option<(Json, usize)>, ParseError> {
        let digits = |mut j: usize| {
            while self.at(j).is_some_and(|c| c.is_ascii_digit()) {
                j += 1;
            }
            j
        };
        let mut j = i;
        if self.at(j) == Some(b'-') {
            j += 1;
        }
        match self.at(j) {
            Some(b'0') => j += 1,
            Some(b'1'..=b'9') => j = digits(j + 1),
            _ => return Ok(None),
        }
        let mut float = false;
        if self.at(j) == Some(b'.') && self.at(j + 1).is_some_and(|c| c.is_ascii_digit()) {
            j = digits(j + 1);
            float = true;
        }
        if matches!(self.at(j), Some(b'e' | b'E')) {
            let k = if matches!(self.at(j + 1), Some(b'-' | b'+')) {
                j + 2
            } else {
                j + 1
            };
            if self.at(k).is_some_and(|c| c.is_ascii_digit()) {
                j = digits(k);
                float = true;
            }
        }
        let text = std::str::from_utf8(&self.s[i..j]).expect("ascii");
        let v = if float {
            Json::Float(text.parse::<f64>().expect("float syntax"))
        } else {
            Json::Int(
                text.parse::<i64>()
                    .map_err(|_| self.err(i, "integer out of range"))?,
            )
        };
        Ok(Some((v, j)))
    }

    fn object(&self, mut end: usize) -> Result<(Json, usize), Fail> {
        let mut members: Vec<(String, Json)> = Vec::new();
        if self.at(end) != Some(b'"') {
            end = self.ws(end);
            match self.at(end) {
                Some(b'}') => return Ok((Json::Obj(members), end + 1)),
                Some(b'"') => {}
                _ => {
                    return Err(self
                        .err(end, "Expecting property name enclosed in double quotes")
                        .into());
                }
            }
        }
        end += 1;
        loop {
            let (key, after) = self.string(end)?;
            end = self.ws(after);
            if self.at(end) != Some(b':') {
                return Err(self.err(end, "Expecting ':' delimiter").into());
            }
            end = self.ws(end + 1);
            let (value, after) = self.scan(end).map_err(|e| self.expecting_value(e))?;
            // Python keeps the last duplicate, in the first one's position.
            if let Some(slot) = members.iter_mut().find(|(k, _)| *k == key) {
                slot.1 = value;
            } else {
                members.push((key, value));
            }
            end = self.ws(after);
            let next = self.at(end);
            end += 1;
            match next {
                Some(b'}') => return Ok((Json::Obj(members), end)),
                Some(b',') => {}
                _ => return Err(self.err(end - 1, "Expecting ',' delimiter").into()),
            }
            let comma = end - 1;
            end = self.ws(end);
            let next = self.at(end);
            end += 1;
            match next {
                Some(b'"') => {}
                Some(b'}') => {
                    return Err(self
                        .err(comma, "Illegal trailing comma before end of object")
                        .into());
                }
                _ => {
                    return Err(self
                        .err(end - 1, "Expecting property name enclosed in double quotes")
                        .into());
                }
            }
        }
    }

    fn array(&self, mut end: usize) -> Result<(Json, usize), Fail> {
        let mut items = Vec::new();
        end = self.ws(end);
        if self.at(end) == Some(b']') {
            return Ok((Json::Arr(items), end + 1));
        }
        loop {
            let (value, after) = self.scan(end).map_err(|e| self.expecting_value(e))?;
            items.push(value);
            end = self.ws(after);
            let next = self.at(end);
            end += 1;
            match next {
                Some(b']') => return Ok((Json::Arr(items), end)),
                Some(b',') => {}
                _ => return Err(self.err(end - 1, "Expecting ',' delimiter").into()),
            }
            let comma = end - 1;
            end = self.ws(end);
            if self.at(end) == Some(b']') {
                return Err(self
                    .err(comma, "Illegal trailing comma before end of array")
                    .into());
            }
        }
    }

    /// Four hex digits after the `u` at `u`.
    fn hex4(&self, u: usize) -> Result<u32, ParseError> {
        self.s
            .get(u + 1..u + 5)
            .filter(|h| h.iter().all(u8::is_ascii_hexdigit))
            .map(|h| u32::from_str_radix(std::str::from_utf8(h).expect("hex"), 16).expect("hex"))
            .ok_or_else(|| self.err(u, "Invalid \\uXXXX escape"))
    }

    /// `scanstring(s, end)`: `end` is just after the opening quote.
    fn string(&self, mut end: usize) -> Result<(String, usize), ParseError> {
        let begin = end - 1;
        let mut out = String::new();
        loop {
            let start = end;
            while self
                .at(end)
                .is_some_and(|c| c != b'"' && c != b'\\' && c >= 0x20)
            {
                end += 1;
            }
            // The text is a str, so a run of plain bytes is whole characters.
            out.push_str(std::str::from_utf8(&self.s[start..end]).expect("valid UTF-8"));
            match self.at(end) {
                None => return Err(self.err(begin, "Unterminated string starting at")),
                Some(b'"') => return Ok((out, end + 1)),
                Some(b'\\') => {}
                Some(_) => return Err(self.err(end, "Invalid control character at")),
            }
            let backslash = end;
            end += 1;
            let Some(esc) = self.at(end) else {
                return Err(self.err(begin, "Unterminated string starting at"));
            };
            if esc != b'u' {
                out.push(match esc {
                    b'"' => '"',
                    b'\\' => '\\',
                    b'/' => '/',
                    b'b' => '\u{8}',
                    b'f' => '\u{c}',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    _ => return Err(self.err(backslash, "Invalid \\escape")),
                });
                end += 1;
                continue;
            }
            let mut code = self.hex4(end)?;
            end += 5;
            if (0xd800..0xdc00).contains(&code) && self.s[end..].starts_with(b"\\u") {
                let lo = self.hex4(end + 1)?;
                if (0xdc00..0xe000).contains(&lo) {
                    code = 0x10000 + ((code - 0xd800) << 10) + (lo - 0xdc00);
                    end += 6;
                }
            }
            out.push(char::from_u32(code).unwrap_or('\u{fffd}'));
        }
    }
}

// ---------------------------------------------------------------- writing

/// `json.dumps` layouts.
#[derive(Clone, Copy)]
pub enum Layout {
    /// `separators=(",", ":")`
    Compact,
    /// the default: `", "` and `": "`
    Default,
    /// `indent=n`: `","` plus newline, and `": "`
    Indent(usize),
}

/// Python's `repr(float)`: shortest round-trip digits, fixed notation for
/// decimal exponents in [-4, 16), otherwise `d.ddde±XX`.
pub fn float_repr(f: f64) -> String {
    if f.is_nan() {
        return "NaN".into();
    }
    if f.is_infinite() {
        return if f > 0.0 {
            "Infinity".into()
        } else {
            "-Infinity".into()
        };
    }
    if f == 0.0 {
        return if f.is_sign_negative() {
            "-0.0".into()
        } else {
            "0.0".into()
        };
    }
    let sci = format!("{f:e}"); // shortest digits, e.g. "-1.8778981204688794e-2"
    let (mantissa, exp) = sci.split_once('e').expect("exponent");
    let exp: i32 = exp.parse().expect("exponent digits");
    let (sign, mantissa) = mantissa
        .strip_prefix('-')
        .map_or(("", mantissa), |m| ("-", m));
    let digits = half_even(f, mantissa.chars().filter(|c| *c != '.').collect());
    if (-4..16).contains(&exp) {
        let point = exp + 1; // digits before the decimal point
        let body = if point <= 0 {
            format!("0.{}{}", "0".repeat((-point) as usize), digits)
        } else if point as usize >= digits.len() {
            format!("{}{}.0", digits, "0".repeat(point as usize - digits.len()))
        } else {
            format!(
                "{}.{}",
                &digits[..point as usize],
                &digits[point as usize..]
            )
        };
        format!("{sign}{body}")
    } else {
        let frac = if digits.len() > 1 {
            format!(".{}", &digits[1..])
        } else {
            String::new()
        };
        let esign = if exp < 0 { '-' } else { '+' };
        format!("{sign}{}{frac}e{esign}{:02}", &digits[..1], exp.abs())
    }
}

/// Rust's shortest digits break an exact tie between two shortest candidates upward;
/// Python's `repr` (David Gay's dtoa, mode 0) takes the even one. A tie needs 16 or more
/// digits (shorter decimals are coarser than the binary grid), and differs only when Rust's
/// last digit is odd: then the value's exact expansion is the lower candidate followed by a 5.
fn half_even(f: f64, mut digits: String) -> String {
    let n = digits.len();
    let last = digits.as_bytes()[n - 1];
    if n < 16 || last.is_multiple_of(2) {
        return digits;
    }
    let exact = format!("{:.800e}", f.abs());
    let exact: Vec<u8> = exact
        .split_once('e')
        .expect("exponent")
        .0
        .bytes()
        .filter(|c| *c != b'.')
        .collect();
    let lower = &digits.as_bytes()[..n - 1];
    if exact[..n - 1] == *lower
        && exact[n - 1] == last - 1
        && exact[n] == b'5'
        && exact[n + 1..].iter().all(|c| *c == b'0')
    {
        digits.pop();
        digits.push(char::from(last - 1));
    }
    digits
}

fn write_str(out: &mut String, s: &str, ascii: bool) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c if ascii && (c as u32) > 0x7e => {
                let mut buf = [0u16; 2];
                for unit in c.encode_utf16(&mut buf) {
                    let _ = write!(out, "\\u{unit:04x}");
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn write_value(out: &mut String, v: &Json, layout: Layout, ascii: bool, depth: usize) {
    let (item_sep, key_sep) = match layout {
        Layout::Compact => (",", ":"),
        Layout::Default => (", ", ": "),
        Layout::Indent(_) => (",", ": "),
    };
    let newline = |out: &mut String, depth: usize| {
        if let Layout::Indent(n) = layout {
            out.push('\n');
            out.push_str(&" ".repeat(n * depth));
        }
    };
    match v {
        Json::Null => out.push_str("null"),
        Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Json::Int(i) => {
            let _ = write!(out, "{i}");
        }
        Json::Float(f) => out.push_str(&float_repr(*f)),
        Json::Str(s) => write_str(out, s, ascii),
        Json::Arr(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (n, item) in items.iter().enumerate() {
                if n > 0 {
                    out.push_str(item_sep);
                }
                newline(out, depth + 1);
                write_value(out, item, layout, ascii, depth + 1);
            }
            newline(out, depth);
            out.push(']');
        }
        Json::Obj(members) => {
            if members.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push('{');
            for (n, (k, item)) in members.iter().enumerate() {
                if n > 0 {
                    out.push_str(item_sep);
                }
                newline(out, depth + 1);
                write_str(out, k, ascii);
                out.push_str(key_sep);
                write_value(out, item, layout, ascii, depth + 1);
            }
            newline(out, depth);
            out.push('}');
        }
    }
}

/// `json.dumps(v, ...)` with the given layout and `ensure_ascii`.
pub fn dumps(v: &Json, layout: Layout, ensure_ascii: bool) -> String {
    let mut out = String::new();
    write_value(&mut out, v, layout, ensure_ascii, 0);
    out
}
