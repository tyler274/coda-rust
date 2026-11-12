use std::fmt;
use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

use crate::policy::{MAX_CHANNEL_NAME_LEN, MAX_MEMBER_NAME_LEN, RESERVED_MEMBER_NAMES};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum NameError {
    #[error("name is empty")]
    Empty,
    #[error("name has {len} characters; the limit is {max}")]
    TooLong { len: usize, max: usize },
    #[error("name must start with a lowercase letter")]
    BadStart,
    #[error("name contains {0:?}; only a-z, 0-9 and '-' are allowed")]
    BadChar(char),
    #[error("{0:?} is reserved")]
    Reserved(String),
}

fn validate(text: &str, max: usize) -> Result<(), NameError> {
    let Some(first) = text.chars().next() else {
        return Err(NameError::Empty);
    };
    let len = text.chars().count();
    if len > max {
        return Err(NameError::TooLong { len, max });
    }
    if !first.is_ascii_lowercase() {
        return Err(NameError::BadStart);
    }
    match text
        .chars()
        .find(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-'))
    {
        Some(bad) => Err(NameError::BadChar(bad)),
        None => Ok(()),
    }
}

/// A roster member's name: lowercase, starting with a letter, `[a-z0-9-]`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct MemberName(String);

impl MemberName {
    pub fn parse(text: &str) -> Result<Self, NameError> {
        validate(text, MAX_MEMBER_NAME_LEN)?;
        if RESERVED_MEMBER_NAMES.contains(&text) {
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
        validate(text, MAX_CHANNEL_NAME_LEN)?;
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
    fn member_names_follow_the_charset() {
        assert!(MemberName::parse("scout-2").is_ok());
        assert_eq!(MemberName::parse(""), Err(NameError::Empty));
        assert_eq!(MemberName::parse("2x"), Err(NameError::BadStart));
        assert_eq!(MemberName::parse("Bob"), Err(NameError::BadStart));
        assert_eq!(MemberName::parse("a_b"), Err(NameError::BadChar('_')));
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
    fn channel_names_have_at_most_24_characters() {
        assert!(ChannelName::parse(&"c".repeat(24)).is_ok());
        assert!(matches!(
            ChannelName::parse(&"c".repeat(25)),
            Err(NameError::TooLong { len: 25, max: 24 })
        ));
        assert!(ChannelName::parse("main").expect("valid").is_main());
    }

    #[test]
    fn names_validate_when_deserialized() {
        assert!(serde_json::from_str::<MemberName>("\"Bad Name\"").is_err());
        let name: ChannelName = serde_json::from_str("\"ops\"").expect("valid");
        assert_eq!(name.as_str(), "ops");
    }
}
