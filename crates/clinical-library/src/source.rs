//! The sources the library collects from.

use std::fmt;

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
