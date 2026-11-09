//! The identifiers the timeline joins on: ticket keys and Git revisions.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A Linear ticket key such as `ENG-159`: a team key and a number.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TicketKey {
    team: String,
    number: u32,
}

impl TicketKey {
    /// Parses `ENG-159` in any case.
    pub fn parse(text: &str) -> Option<Self> {
        let (team, number) = text.split_once('-')?;
        if team.is_empty() || !team.chars().all(|c| c.is_ascii_alphabetic()) {
            return None;
        }
        if number.is_empty() || !number.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        Some(Self {
            team: team.to_ascii_uppercase(),
            number: number.parse().ok()?,
        })
    }

    /// Finds the ticket a branch is for, as branches name it: `eng-159-chart-evidence`,
    /// `dev/eng-141-remove-person-sharing` or `tyler-eng-225-add-accounts`. Only the
    /// given team keys count, so a word like `ci-384` is not mistaken for a ticket.
    pub fn in_branch(branch: &str, teams: &[String]) -> Option<Self> {
        let words: Vec<&str> = branch.split(|c: char| !c.is_ascii_alphanumeric()).collect();
        words.windows(2).find_map(|pair| {
            let team = teams
                .iter()
                .find(|team| team.eq_ignore_ascii_case(pair[0]))?;
            let key = Self::parse(&format!("{team}-{}", pair[1]))?;
            Some(key)
        })
    }
}

impl fmt::Display for TicketKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}-{}", self.team, self.number)
    }
}

impl Serialize for TicketKey {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for TicketKey {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::parse(&text)
            .ok_or_else(|| serde::de::Error::custom(format!("not a ticket key: {text}")))
    }
}

/// A full 40-character Git object name.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct Revision(String);

impl Revision {
    /// Accepts a full lowercase hexadecimal SHA-1 object name, trimming whitespace.
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();
        (text.len() == 40 && text.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')))
            .then(|| Self(text.to_owned()))
    }

    /// The object name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Revision {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::parse(&text)
            .ok_or_else(|| serde::de::Error::custom(format!("not a full revision: {text}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn teams() -> Vec<String> {
        vec!["ENG".to_owned(), "COD".to_owned()]
    }

    #[test]
    fn branches_name_tickets_in_each_form_the_repository_uses() {
        let key = |branch| TicketKey::in_branch(branch, &teams()).map(|k| k.to_string());
        assert_eq!(key("eng-159-chart-evidence").as_deref(), Some("ENG-159"));
        assert_eq!(
            key("dev/eng-141-remove-person-sharing").as_deref(),
            Some("ENG-141")
        );
        assert_eq!(
            key("tyler-eng-225-add-staging-sign-in-accounts").as_deref(),
            Some("ENG-225")
        );
        assert_eq!(key("COD-7").as_deref(), Some("COD-7"));
    }

    #[test]
    fn words_that_only_look_like_keys_are_not_tickets() {
        let key = |branch| TicketKey::in_branch(branch, &teams());
        assert_eq!(key("ci-restack-384"), None);
        assert_eq!(key("harness-timeline"), None);
        assert_eq!(key("eng-x1"), None);
    }

    #[test]
    fn revisions_are_full_object_names() {
        assert!(Revision::parse("8d9cb4c07b496a06628b9e57387488a142268df7\n").is_some());
        assert!(Revision::parse("8d9cb4c").is_none());
        assert!(Revision::parse("8D9CB4C07B496A06628B9E57387488A142268DF7").is_none());
    }
}
