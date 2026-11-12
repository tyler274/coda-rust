use crate::names::MemberName;

/// One `@` reference found in post text. Tokens are lowercased and kept as
/// text because a post may name someone who is not on the roster.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mention {
    /// `@all`
    All,
    /// `@name`, which also names every member of the group `name-N`.
    Name(String),
    /// `@#channel`
    Channel(String),
}

impl Mention {
    /// Whether this mention names `member` directly, by group, or via `@all`.
    /// Channel mentions depend on the roster and are resolved there.
    pub fn names(&self, member: &MemberName) -> bool {
        match self {
            Self::All => true,
            Self::Name(token) => {
                let name = member.as_str();
                name == token
                    || name
                        .strip_prefix(token.as_str())
                        .and_then(|rest| rest.strip_prefix('-'))
                        .is_some_and(|index| {
                            !index.is_empty() && index.bytes().all(|b| b.is_ascii_digit())
                        })
            }
            Self::Channel(_) => false,
        }
    }
}

fn is_token_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

/// Finds mentions in post text. A mention starts at a word boundary and runs
/// until the first character that cannot be part of a name, so `a@b` and
/// `@bob_x` never mention `bob`. Matching is case-insensitive.
pub fn parse_mentions(text: &str) -> Vec<Mention> {
    let mut mentions = Vec::new();
    let mut previous: Option<char> = None;
    for (index, c) in text.char_indices() {
        let at_boundary = previous.is_none_or(|p| !is_token_char(p));
        previous = Some(c);
        if c != '@' || !at_boundary {
            continue;
        }
        let rest = &text[index + 1..];
        let (is_channel, body) = match rest.strip_prefix('#') {
            Some(body) => (true, body),
            None => (false, rest),
        };
        let token_len = body.find(|c: char| !is_token_char(c)).unwrap_or(body.len());
        if token_len == 0 {
            continue;
        }
        let token = body[..token_len].to_ascii_lowercase();
        let mention = match (is_channel, token.as_str()) {
            (true, _) => Mention::Channel(token),
            (false, "all") => Mention::All,
            (false, _) => Mention::Name(token),
        };
        if !mentions.contains(&mention) {
            mentions.push(mention);
        }
    }
    mentions
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(text: &str) -> MemberName {
        MemberName::parse(text).expect("valid")
    }

    #[test]
    fn parses_names_groups_all_and_channels() {
        assert_eq!(
            parse_mentions("@Scout please sync with @#ops, cc @all and @scout"),
            vec![
                Mention::Name("scout".into()),
                Mention::Channel("ops".into()),
                Mention::All,
            ]
        );
    }

    #[test]
    fn requires_a_word_boundary_and_a_full_token() {
        assert!(parse_mentions("mail bob@example.com").is_empty());
        assert_eq!(
            parse_mentions("(@bob_x) @ lone"),
            vec![Mention::Name("bob_x".into())]
        );
        assert!(parse_mentions("@@").is_empty());
    }

    #[test]
    fn name_mention_matches_exact_member_and_group() {
        let scout = Mention::Name("scout".into());
        assert!(scout.names(&name("scout")));
        assert!(scout.names(&name("scout-12")));
        assert!(!scout.names(&name("scouts")));
        assert!(!scout.names(&name("scout-x")));
        assert!(!Mention::Name("scout-1".into()).names(&name("scout-12")));
        assert!(Mention::All.names(&name("anyone")));
    }
}
