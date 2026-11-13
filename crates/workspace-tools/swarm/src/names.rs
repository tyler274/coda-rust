use std::fmt;
use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};
use unicode_general_category::{GeneralCategory, get_general_category};

use crate::policy::{MAX_CHANNEL_NAME_LEN, MAX_MEMBER_NAME_LEN, RESERVED_MEMBER_NAME};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum NameError {
    #[error("name is empty")]
    Empty,
    #[error("name is {len} bytes; the limit is {max}")]
    TooLong { len: usize, max: usize },
    #[error("channel name must start with a lowercase letter")]
    BadChannelStart,
    #[error("channel name contains {0:?}; only a-z, 0-9 and '-' are allowed")]
    BadChannelChar(char),
    #[error("member name must start with a letter or digit")]
    BadMemberStart,
    #[error("member name contains {0:?}; only letters, digits, '-' and '_' are allowed")]
    BadMemberChar(char),
    #[error("{0:?} is reserved")]
    Reserved(String),
}

fn is_letter_or_digit(c: char) -> bool {
    matches!(
        get_general_category(c),
        GeneralCategory::UppercaseLetter
            | GeneralCategory::LowercaseLetter
            | GeneralCategory::TitlecaseLetter
            | GeneralCategory::ModifierLetter
            | GeneralCategory::OtherLetter
            | GeneralCategory::DecimalNumber
    )
}

/// A character a member name may contain: any Unicode letter or decimal
/// digit, `-` or `_`. Mentions end at the first character that is not one.
pub(crate) fn is_member_char(c: char) -> bool {
    is_letter_or_digit(c) || c == '-' || c == '_'
}

fn check_len(text: &str, max: usize) -> Result<char, NameError> {
    let first = text.chars().next().ok_or(NameError::Empty)?;
    if text.len() > max {
        return Err(NameError::TooLong {
            len: text.len(),
            max,
        });
    }
    Ok(first)
}

/// A roster member's name: 1 to 32 bytes of letters, digits, `-` and `_`,
/// starting with a letter or digit.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct MemberName(String);

impl MemberName {
    pub fn parse(text: &str) -> Result<Self, NameError> {
        let first = check_len(text, MAX_MEMBER_NAME_LEN)?;
        if !is_letter_or_digit(first) {
            return Err(NameError::BadMemberStart);
        }
        if let Some(bad) = text.chars().find(|c| !is_member_char(*c)) {
            return Err(NameError::BadMemberChar(bad));
        }
        if text == RESERVED_MEMBER_NAME {
            return Err(NameError::Reserved(text.to_owned()));
        }
        Ok(Self(text.to_owned()))
    }

    /// Names for `count` members added together: `base` alone for one,
    /// otherwise `base-1` through `base-count`.
    pub fn group(base: &str, count: NonZeroUsize) -> Result<Vec<Self>, NameError> {
        if count.get() == 1 {
            return Ok(vec![Self::parse(base)?]);
        }
        (1..=count.get())
            .map(|index| Self::parse(&format!("{base}-{index}")))
            .collect()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for MemberName {
    type Error = NameError;
    fn try_from(text: String) -> Result<Self, NameError> {
        Self::parse(&text)
    }
}

impl From<MemberName> for String {
    fn from(name: MemberName) -> Self {
        name.0
    }
}

impl fmt::Display for MemberName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A channel's name. `main` is a valid name for the channel every member is
/// in; whether a name may be created is the roster's decision.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ChannelName(String);

impl ChannelName {
    pub fn parse(text: &str) -> Result<Self, NameError> {
        let first = check_len(text, MAX_CHANNEL_NAME_LEN)?;
        if !first.is_ascii_lowercase() {
            return Err(NameError::BadChannelStart);
        }
        let allowed = |c: &char| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-';
        if let Some(bad) = text.chars().find(|c| !allowed(c)) {
            return Err(NameError::BadChannelChar(bad));
        }
        Ok(Self(text.to_owned()))
    }

    pub fn main() -> Self {
        Self("main".to_owned())
    }

    pub fn is_main(&self) -> bool {
        self.0 == "main"
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ChannelName {
    type Error = NameError;
    fn try_from(text: String) -> Result<Self, NameError> {
        Self::parse(&text)
    }
}

impl From<ChannelName> for String {
    fn from(name: ChannelName) -> Self {
        name.0
    }
}

impl fmt::Display for ChannelName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(n: usize) -> NonZeroUsize {
        NonZeroUsize::new(n).expect("nonzero")
    }

    #[test]
    fn member_names_accept_unicode_letters_digits_dash_and_underscore() {
        for ok in [
            "scout-2",
            "a_b",
            "Bob",
            "2x",
            "ā",
            "研究員",
            "x".repeat(32).as_str(),
        ] {
            assert!(MemberName::parse(ok).is_ok(), "{ok:?} is valid");
        }
        assert_eq!(MemberName::parse(""), Err(NameError::Empty));
        assert_eq!(
            MemberName::parse(&"x".repeat(33)),
            Err(NameError::TooLong { len: 33, max: 32 })
        );
        assert_eq!(
            MemberName::parse(&"研".repeat(11)),
            Err(NameError::TooLong { len: 33, max: 32 })
        );
        assert_eq!(MemberName::parse("-x"), Err(NameError::BadMemberStart));
        assert_eq!(MemberName::parse("_x"), Err(NameError::BadMemberStart));
        assert_eq!(MemberName::parse("a b"), Err(NameError::BadMemberChar(' ')));
        assert_eq!(MemberName::parse("a.b"), Err(NameError::BadMemberChar('.')));
        assert_eq!(MemberName::parse("a@b"), Err(NameError::BadMemberChar('@')));
        assert_eq!(
            MemberName::parse("all"),
            Err(NameError::Reserved("all".into()))
        );
    }

    #[test]
    fn group_names_number_from_one() {
        let one = MemberName::group("scout", count(1)).expect("valid");
        assert_eq!(one, vec![MemberName::parse("scout").expect("valid")]);
        let three: Vec<String> = MemberName::group("scout", count(3))
            .expect("valid")
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(three, ["scout-1", "scout-2", "scout-3"]);
    }

    #[test]
    fn group_suffix_counts_toward_length() {
        let base = "a".repeat(MAX_MEMBER_NAME_LEN - 1);
        assert!(matches!(
            MemberName::group(&base, count(2)),
            Err(NameError::TooLong { .. })
        ));
    }

    #[test]
    fn channel_names_match_lowercase_start_then_a_z_digits_dash() {
        for ok in ["c", "ops-2", "a1-", "c".repeat(24).as_str()] {
            assert!(ChannelName::parse(ok).is_ok(), "{ok:?} is valid");
        }
        assert_eq!(
            ChannelName::parse(&"c".repeat(25)),
            Err(NameError::TooLong { len: 25, max: 24 })
        );
        assert_eq!(ChannelName::parse(""), Err(NameError::Empty));
        assert_eq!(ChannelName::parse("1ops"), Err(NameError::BadChannelStart));
        assert_eq!(ChannelName::parse("-ops"), Err(NameError::BadChannelStart));
        assert_eq!(ChannelName::parse("Ops"), Err(NameError::BadChannelStart));
        assert_eq!(
            ChannelName::parse("o_ps"),
            Err(NameError::BadChannelChar('_'))
        );
        assert_eq!(
            ChannelName::parse("oPs"),
            Err(NameError::BadChannelChar('P'))
        );
        assert!(ChannelName::parse("main").expect("valid").is_main());
    }

    #[test]
    fn names_validate_when_deserialized() {
        assert!(serde_json::from_str::<MemberName>("\"Bad Name\"").is_err());
        let name: ChannelName = serde_json::from_str("\"ops\"").expect("valid");
        assert_eq!(name.as_str(), "ops");
    }
}
