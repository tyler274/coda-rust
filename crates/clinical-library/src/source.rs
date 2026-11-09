//! The sources the library collects from, and the keys of their items.

use std::fmt;

use crate::pubmed::Pmid;

/// One source the library collects from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Source {
    /// PubMed's retraction and correction status.
    PubMed,
    /// PubMed Central full-text articles.
    Pmc,
    /// DailyMed drug labels.
    DailyMed,
    /// CDC's Morbidity and Mortality Weekly Report.
    Mmwr,
    /// USPSTF recommendations.
    Uspstf,
}

impl Source {
    /// Every source.
    pub const ALL: [Self; 5] = [
        Self::PubMed,
        Self::Pmc,
        Self::DailyMed,
        Self::Mmwr,
        Self::Uspstf,
    ];

    /// The stored name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PubMed => "pubmed",
            Self::Pmc => "pmc",
            Self::DailyMed => "dailymed",
            Self::Mmwr => "mmwr",
            Self::Uspstf => "uspstf",
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One item's key: its source, and the source's own identifier for it.
///
/// A PubMed item's key is always a [`Pmid`], so a PMCID or other text can't
/// land under PubMed. Other sources' keys are their identifiers as text, at
/// most 256 characters with no surrounding or control whitespace, until each
/// source has its own identifier type.
///
/// ```compile_fail
/// use clinical_library::{ItemKey, Source};
/// // A PubMed key is built only from a PMID.
/// let _ = ItemKey::other(Source::PubMed, "PMC1").unwrap();
/// let _ = ItemKey::pubmed("PMC1");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ItemKey {
    source: Source,
    key: String,
}

impl ItemKey {
    /// A PubMed article's key.
    pub fn pubmed(pmid: Pmid) -> Self {
        Self {
            source: Source::PubMed,
            key: pmid.to_string(),
        }
    }

    /// Another source's item key. PubMed's keys come only from
    /// [`ItemKey::pubmed`].
    pub fn other(source: OtherSource, key: &str) -> Result<Self, InvalidItemKey> {
        let fits = (1..=256).contains(&key.chars().count())
            && key.trim() == key
            && !key.chars().any(char::is_control);
        if !fits {
            return Err(InvalidItemKey(key.to_owned()));
        }
        Ok(Self {
            source: source.into(),
            key: key.to_owned(),
        })
    }

    /// The item's source.
    pub const fn source(&self) -> Source {
        self.source
    }

    /// The key as stored.
    pub fn as_str(&self) -> &str {
        &self.key
    }
}

impl fmt::Display for ItemKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.source, self.key)
    }
}

/// A source whose item keys are text for now: every source but PubMed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OtherSource {
    /// PubMed Central full-text articles.
    Pmc,
    /// DailyMed drug labels.
    DailyMed,
    /// CDC's Morbidity and Mortality Weekly Report.
    Mmwr,
    /// USPSTF recommendations.
    Uspstf,
}

impl From<OtherSource> for Source {
    fn from(source: OtherSource) -> Self {
        match source {
            OtherSource::Pmc => Self::Pmc,
            OtherSource::DailyMed => Self::DailyMed,
            OtherSource::Mmwr => Self::Mmwr,
            OtherSource::Uspstf => Self::Uspstf,
        }
    }
}

/// Text that can't be an item key.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0:?} is not an item key")]
pub struct InvalidItemKey(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_keys_are_bounded_and_pubmeds_are_pmids() {
        let pmid = Pmid::parse("9500320").unwrap();
        let key = ItemKey::pubmed(pmid);
        assert_eq!((key.source(), key.as_str()), (Source::PubMed, "9500320"));
        assert!(ItemKey::other(OtherSource::Pmc, "PMC123").is_ok());
        for bad in ["", " PMC1", "PMC1\n", "PMC\u{7}1", &"x".repeat(257)] {
            assert!(ItemKey::other(OtherSource::Pmc, bad).is_err(), "{bad:?}");
        }
    }
}
