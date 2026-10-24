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

#[derive(Debug)]
pub struct ParseError {
    pub at: usize,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at byte {}", self.message, self.at)
    }
}

impl std::error::Error for ParseError {}

pub fn parse(text: &str) -> Result<Json, ParseError> {
    let mut p = Parser {
        s: text.as_bytes(),
        i: 0,
    };
    p.ws();
    let v = p.value()?;
    p.ws();
    if p.i != p.s.len() {
        return Err(p.err("extra data"));
    }
    Ok(v)
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn err(&self, message: &str) -> ParseError {
        ParseError {
            at: self.i,
            message: message.to_string(),
        }
    }

    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }

    fn eat(&mut self, lit: &[u8]) -> bool {
        if self.s[self.i..].starts_with(lit) {
            self.i += lit.len();
            true
        } else {
            false
        }
    }

    fn value(&mut self) -> Result<Json, ParseError> {
        match self.s.get(self.i) {
            None => Err(self.err("unexpected end")),
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') if self.eat(b"true") => Ok(Json::Bool(true)),
            Some(b'f') if self.eat(b"false") => Ok(Json::Bool(false)),
            Some(b'n') if self.eat(b"null") => Ok(Json::Null),
            Some(b'N') if self.eat(b"NaN") => Ok(Json::Float(f64::NAN)),
            Some(b'I') if self.eat(b"Infinity") => Ok(Json::Float(f64::INFINITY)),
            Some(b'-') if self.s[self.i..].starts_with(b"-Infinity") => {
                self.i += 9;
                Ok(Json::Float(f64::NEG_INFINITY))
            }
            Some(c) if *c == b'-' || c.is_ascii_digit() => self.number(),
            Some(_) => Err(self.err("unexpected character")),
        }
    }

    fn object(&mut self) -> Result<Json, ParseError> {
        self.i += 1;
        let mut members = Vec::new();
        self.ws();
        if self.eat(b"}") {
            return Ok(Json::Obj(members));
        }
        loop {
            self.ws();
            if self.s.get(self.i) != Some(&b'"') {
                return Err(self.err("expected key"));
            }
            let key = self.string()?;
            self.ws();
            if !self.eat(b":") {
                return Err(self.err("expected ':'"));
            }
            self.ws();
            let value = self.value()?;
            // Python keeps the last duplicate, in the first one's position.
            if let Some(slot) = members
                .iter_mut()
                .find(|(k, _): &&mut (String, Json)| *k == key)
            {
                slot.1 = value;
            } else {
                members.push((key, value));
            }
            self.ws();
            if self.eat(b",") {
                continue;
            }
            if self.eat(b"}") {
                return Ok(Json::Obj(members));
            }
            return Err(self.err("expected ',' or '}'"));
        }
    }

    fn array(&mut self) -> Result<Json, ParseError> {
        self.i += 1;
        let mut items = Vec::new();
        self.ws();
        if self.eat(b"]") {
            return Ok(Json::Arr(items));
        }
        loop {
            self.ws();
            items.push(self.value()?);
            self.ws();
            if self.eat(b",") {
                continue;
            }
            if self.eat(b"]") {
                return Ok(Json::Arr(items));
            }
            return Err(self.err("expected ',' or ']'"));
        }
    }

    fn hex4(&mut self) -> Result<u32, ParseError> {
        let h = self
            .s
            .get(self.i..self.i + 4)
            .ok_or_else(|| self.err("short \\u escape"))?;
        let h = std::str::from_utf8(h).map_err(|_| self.err("bad \\u escape"))?;
        let v = u32::from_str_radix(h, 16).map_err(|_| self.err("bad \\u escape"))?;
        self.i += 4;
        Ok(v)
    }

    fn string(&mut self) -> Result<String, ParseError> {
        self.i += 1;
        let mut out = String::new();
        loop {
            let start = self.i;
            while self.i < self.s.len() && self.s[self.i] != b'"' && self.s[self.i] != b'\\' {
                if self.s[self.i] < 0x20 {
                    return Err(self.err("control character in string"));
                }
                self.i += 1;
            }
            out.push_str(
                std::str::from_utf8(&self.s[start..self.i])
                    .map_err(|_| self.err("invalid UTF-8"))?,
            );
            match self.s.get(self.i) {
                None => return Err(self.err("unterminated string")),
                Some(b'"') => {
                    self.i += 1;
                    return Ok(out);
                }
                _ => {}
            }
            self.i += 1;
            let c = *self.s.get(self.i).ok_or_else(|| self.err("bad escape"))?;
            self.i += 1;
            match c {
                b'"' => out.push('"'),
                b'\\' => out.push('\\'),
                b'/' => out.push('/'),
                b'b' => out.push('\u{8}'),
                b'f' => out.push('\u{c}'),
                b'n' => out.push('\n'),
                b'r' => out.push('\r'),
                b't' => out.push('\t'),
                b'u' => {
                    let hi = self.hex4()?;
                    let code =
                        if (0xd800..0xdc00).contains(&hi) && self.s[self.i..].starts_with(b"\\u") {
                            let save = self.i;
                            self.i += 2;
                            let lo = self.hex4()?;
                            if (0xdc00..0xe000).contains(&lo) {
                                0x10000 + ((hi - 0xd800) << 10) + (lo - 0xdc00)
                            } else {
                                self.i = save;
                                hi
                            }
                        } else {
                            hi
                        };
                    out.push(char::from_u32(code).unwrap_or('\u{fffd}'));
                }
                _ => return Err(self.err("bad escape")),
            }
        }
    }

    fn number(&mut self) -> Result<Json, ParseError> {
        let start = self.i;
        if self.s[self.i] == b'-' {
            self.i += 1;
        }
        let mut float = false;
        while self.i < self.s.len() {
            match self.s[self.i] {
                b'0'..=b'9' => {}
                b'.' | b'e' | b'E' => float = true,
                b'+' | b'-' if float => {}
                _ => break,
            }
            self.i += 1;
        }
        let text = std::str::from_utf8(&self.s[start..self.i]).expect("ascii");
        if float {
            text.parse::<f64>()
                .map(Json::Float)
                .map_err(|_| self.err("bad number"))
        } else {
            text.parse::<i64>()
                .map(Json::Int)
                .map_err(|_| self.err("integer out of range"))
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
