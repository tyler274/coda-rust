//! What NLM says about one PubMed article, in NLM's own words.
//!
//! A status record copies NLM's bibliographic assertions: the article's
//! publication types and its comments-and-corrections links, each with NLM's
//! `RefType`. It never makes a clinical call. [`StatusRecord::notices`] reads
//! the assertions NLM uses for retractions, corrections, expressions of
//! concern and updates, so search can tell a reader what NLM has published
//! about an article; anything else NLM sends is kept as it came.
//!
//! The link types are the `RefType` values NLM's PubMed DTD defines
//! (`pubmed_250101.dtd`). Those that point from an article to a notice about
//! it (`…In`) are read; those on the notice itself (`…Of`, `…For`, `…From`)
//! describe the notice, not the article.

use std::fmt;

use chrono::{DateTime, NaiveDate, Utc};

/// A PubMed identifier: a positive whole number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pmid(u64);

impl Pmid {
    /// The largest PMID the records accept, so every one fits Postgres's
    /// `bigint`.
    const MAX: u64 = i64::MAX as u64;

    /// Wraps a PMID, refusing zero or one past `i64::MAX`.
    pub fn new(value: u64) -> Result<Self, InvalidPmid> {
        if value == 0 || value > Self::MAX {
            return Err(InvalidPmid(value.to_string()));
        }
        Ok(Self(value))
    }

    /// Reads a PMID written as digits, with no sign, spaces or leading zero.
    pub fn parse(text: &str) -> Result<Self, InvalidPmid> {
        let digits_only = !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit());
        if !digits_only || text.starts_with('0') {
            return Err(InvalidPmid(text.to_owned()));
        }
        text.parse::<u64>()
            .map_err(|_too_large| InvalidPmid(text.to_owned()))
            .and_then(Self::new)
    }

    /// The PMID as a number.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// The PMID as stored in Postgres.
    pub(crate) fn as_stored(self) -> i64 {
        // `new` bounds every PMID to `i64::MAX`.
        i64::try_from(self.0).unwrap_or(i64::MAX)
    }
}

impl fmt::Display for Pmid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Text that isn't a PMID.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0:?} is not a PMID")]
pub struct InvalidPmid(pub String);

/// NLM's bibliographic assertions about one article.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusRecord {
    /// The article.
    pub pmid: Pmid,
    /// NLM's publication types, in NLM's order and words, such as
    /// `Retracted Publication`.
    pub publication_types: Vec<String>,
    /// NLM's comments-and-corrections links, in NLM's order.
    pub relations: Vec<Relation>,
    /// When NLM last revised the record, if it says.
    pub revised: Option<NaiveDate>,
    /// The SHA-256 of the record exactly as NLM sent it, so a later night can
    /// tell whether NLM changed it.
    pub record_sha256: [u8; 32],
    /// The record exactly as NLM sent it: its `PubmedArticle` or
    /// `PubmedBookArticle` element. Kept so a better reader can read it again
    /// without asking NCBI, and so every stored status can be traced to the
    /// bytes it came from.
    pub record_xml: String,
}

/// A stored status, with when the library first held it and when NLM last
/// confirmed it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredStatus {
    /// NLM's assertions.
    pub record: StatusRecord,
    /// When the library first held this record.
    pub observed_at: DateTime<Utc>,
    /// When NLM last sent this record unchanged: the status is current as
    /// of then.
    pub confirmed_at: DateTime<Utc>,
}

/// One comments-and-corrections link NLM records on an article.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    /// NLM's `RefType`, such as `RetractionIn` or `ErratumIn`.
    pub ref_type: String,
    /// The linked article's PMID, when NLM gives one.
    pub target: Option<Pmid>,
    /// NLM's citation of the linked item, such as a journal reference.
    pub source_reference: Option<String>,
    /// NLM's note on the link.
    pub note: Option<String>,
}

/// What NLM has published about an article, read from its assertions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each notice is independent: NLM can mark one article retracted, corrected and given an expression of concern at once"
)]
pub struct Notices {
    /// NLM marks it retracted: the `Retracted Publication` type, a
    /// `RetractionIn` link, or a `RetractedandRepublishedIn` link to the
    /// version that replaced it.
    pub retracted: bool,
    /// NLM links an expression of concern (`ExpressionOfConcernIn`).
    pub expression_of_concern: bool,
    /// NLM links an erratum or a corrected and republished version
    /// (`ErratumIn`, `CorrectedandRepublishedIn`).
    pub corrected: bool,
    /// NLM links an updated version (`UpdateIn`).
    pub updated: bool,
}

impl StatusRecord {
    /// The retractions, corrections, expressions of concern and updates NLM
    /// asserts. An article with none has no such notice from NLM; it is not
    /// a judgment that the article is sound.
    pub fn notices(&self) -> Notices {
        let links = |types: &[&str]| {
            self.relations
                .iter()
                .any(|relation| types.contains(&relation.ref_type.as_str()))
        };
        Notices {
            retracted: self
                .publication_types
                .iter()
                .any(|kind| kind == "Retracted Publication")
                || links(&["RetractionIn", "RetractedandRepublishedIn"]),
            expression_of_concern: links(&["ExpressionOfConcernIn"]),
            corrected: links(&["ErratumIn", "CorrectedandRepublishedIn"]),
            updated: links(&["UpdateIn"]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with(publication_types: &[&str], ref_types: &[&str]) -> StatusRecord {
        StatusRecord {
            pmid: Pmid::new(1).unwrap(),
            publication_types: publication_types.iter().map(|t| (*t).to_owned()).collect(),
            relations: ref_types
                .iter()
                .map(|ref_type| Relation {
                    ref_type: (*ref_type).to_owned(),
                    target: None,
                    source_reference: None,
                    note: None,
                })
                .collect(),
            revised: None,
            record_sha256: [0; 32],
            record_xml: String::new(),
        }
    }

    /// Every link NLM's DTD defines from an article to a notice about it is
    /// read, and the notice's own links aren't.
    #[test]
    fn notices_follow_nlms_link_types() {
        let read = |ref_type| with(&["Journal Article"], &[ref_type]).notices();
        for retracted in ["RetractionIn", "RetractedandRepublishedIn"] {
            assert!(read(retracted).retracted, "{retracted}");
        }
        for corrected in ["ErratumIn", "CorrectedandRepublishedIn"] {
            assert!(read(corrected).corrected, "{corrected}");
        }
        assert!(read("ExpressionOfConcernIn").expression_of_concern);
        assert!(read("UpdateIn").updated);
        for about_the_notice in [
            "RetractionOf",
            "RetractedandRepublishedFrom",
            "ErratumFor",
            "CorrectedandRepublishedFrom",
            "ExpressionOfConcernFor",
            "UpdateOf",
            "CommentIn",
            "Cites",
        ] {
            assert_eq!(
                read(about_the_notice),
                Notices::default(),
                "{about_the_notice}"
            );
        }
        assert!(with(&["Retracted Publication"], &[]).notices().retracted);
    }

    #[test]
    fn pmids_are_positive_digits() {
        assert_eq!(Pmid::parse("9500320").map(Pmid::get), Ok(9_500_320));
        for bad in ["", "0", "012", "-1", "1 ", "1e3", "99999999999999999999"] {
            assert!(Pmid::parse(bad).is_err(), "{bad:?}");
        }
        assert!(Pmid::new(i64::MAX as u64).is_ok());
        assert!(Pmid::new(i64::MAX as u64 + 1).is_err());
    }
}
