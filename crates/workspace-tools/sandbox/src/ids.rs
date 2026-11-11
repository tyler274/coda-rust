//! Names the sandbox passes between host, container and CI controller.
//! Each parses once at the boundary, so code past it never rechecks a string.

use std::fmt;
use std::str::FromStr;

/// Which kind of name failed to parse, with the rule it must follow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdKind {
    Slug,
    JobName,
    RequestId,
    SeedKey,
}

impl IdKind {
    fn rule(self) -> &'static str {
        match self {
            Self::Slug => {
                "at most 100 lowercase letters, digits, '.', '_' or '-', starting with a letter \
                 or digit, without '..' and not ending in '.' or '.lock'"
            }
            Self::JobName => "1 to 64 letters, digits, '_' or '-'",
            Self::RequestId => "exactly 12 lowercase hex digits",
            Self::SeedKey => "exactly 16 lowercase hex digits",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{value:?} is not a valid {kind:?}: expected {}", kind.rule())]
pub struct ParseIdError {
    pub kind: IdKind,
    pub value: String,
}

/// The longest slug, well under a directory name's 255 bytes.
pub const SLUG_MAX_BYTES: usize = 100;

fn is_lower_hex(byte: u8) -> bool {
    byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
}

fn valid(kind: IdKind, value: &str) -> bool {
    let bytes = value.as_bytes();
    match kind {
        IdKind::Slug => {
            let charset = bytes.split_first().is_some_and(|(first, rest)| {
                (first.is_ascii_lowercase() || first.is_ascii_digit())
                    && rest.iter().all(|&b| {
                        b.is_ascii_lowercase()
                            || b.is_ascii_digit()
                            || matches!(b, b'.' | b'_' | b'-')
                    })
            });
            // Git refuses these as branch names.
            charset
                && bytes.len() <= SLUG_MAX_BYTES
                && !value.contains("..")
                && !value.ends_with('.')
                && !value.ends_with(".lock")
        }
        IdKind::JobName => {
            (1..=64).contains(&bytes.len())
                && bytes
                    .iter()
                    .all(|&b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
        }
        IdKind::RequestId => bytes.len() == 12 && bytes.iter().copied().all(is_lower_hex),
        IdKind::SeedKey => bytes.len() == 16 && bytes.iter().copied().all(is_lower_hex),
    }
}

macro_rules! name_type {
    ($(#[$doc:meta])* $name:ident, $kind:expr) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl FromStr for $name {
            type Err = ParseIdError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                if valid($kind, value) {
                    Ok(Self(value.to_owned()))
                } else {
                    Err(ParseIdError { kind: $kind, value: value.to_owned() })
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

name_type!(
    /// A worktree's name, also its branch and its directory under the build root.
    Slug,
    IdKind::Slug
);
name_type!(
    /// A CI job to run, as named in the workflow.
    JobName,
    IdKind::JobName
);
name_type!(
    /// One CI request in the spool.
    RequestId,
    IdKind::RequestId
);
name_type!(
    /// The key of a seeded build-output snapshot.
    SeedKey,
    IdKind::SeedKey
);

impl SeedKey {
    /// The key for a digest: its first 8 bytes as 16 lowercase hex digits.
    pub fn from_digest(digest: &[u8; 32]) -> Self {
        Self(digest[..8].iter().map(|b| format!("{b:02x}")).collect())
    }
}

impl RequestId {
    /// The id for 6 random bytes, as 12 lowercase hex digits.
    pub fn from_bytes(bytes: [u8; 6]) -> Self {
        Self(bytes.iter().map(|b| format!("{b:02x}")).collect())
    }
}
