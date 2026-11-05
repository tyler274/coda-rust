//! The site's legal documents. Each current document is one record in
//! `data/<document>.json` and each archived version one in
//! `data/archive/<document>/<slug>.json`, holding all of its wording. Startup loads them
//! into a [`Legal`] before rendering; a record that doesn't parse is a [`LegalError`]
//! naming the file, and startup stops.
use crate::Route;
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use url::Url;

/// A current legal document the site publishes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Document {
    /// A document whose earlier versions stay published.
    WithArchive(Archive),
    ConsumerHealthData,
    Privacy,
    Terms,
    ClinicalTransparency,
}

/// A document whose earlier versions stay published.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Archive {
    DuetPrivacy,
    DuetTerms,
}

impl Document {
    /// The archive of this document's earlier versions, if it keeps one.
    pub fn archive(self) -> Option<Archive> {
        match self {
            Self::WithArchive(archive) => Some(archive),
            Self::ConsumerHealthData | Self::Privacy | Self::Terms | Self::ClinicalTransparency => {
                None
            }
        }
    }
}

/// A current document: its page heading and every word of its text.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LegalDocument {
    pub eyebrow: String,
    pub title: String,
    pub intro: String,
    /// The word before the date, such as "Effective".
    pub date_label: String,
    pub effective: String,
    /// The version users accept, on a document they accept by version.
    pub version: Option<String>,
    #[serde(default)]
    pub summary: Vec<String>,
    pub sections: Vec<Section>,
}

/// An earlier version of a document, kept unchanged with its own heading and text.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArchivedVersion {
    /// The version's URL segment under its document: its record's file name.
    #[serde(skip)]
    pub slug: &'static str,
    pub eyebrow: String,
    pub title: String,
    pub effective: String,
    pub last_updated: String,
    /// The version users accepted, on a document they accept by version.
    pub version: Option<String>,
    #[serde(default)]
    pub summary: Vec<String>,
    pub sections: Vec<Section>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    /// The anchor other documents link to, where one does.
    pub id: Option<String>,
    pub title: String,
    pub body: Vec<Block>,
}

/// One block of a section's body, written as an object with one key naming its kind.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Block {
    Paragraph(String),
    Linked(LinkedParagraph),
    List(Vec<String>),
    Table(Table),
    /// Where a notice's terms are published, shown by its host.
    Source(Url),
}
/// A paragraph with one phrase linked: `before`, the link's label, then `after`.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedParagraph {
    pub before: String,
    pub link: Link,
    pub after: String,
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub label: String,
    #[serde(deserialize_with = "route")]
    pub href: Route,
}
/// Rows of cells under named columns, one cell per column; the first cell names the row.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

/// Reads a link target as one of the site's pages.
fn route<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Route, D::Error> {
    let href = String::deserialize(deserializer)?;
    match href.parse::<Route>() {
        Ok(Route::NotFound { .. }) | Err(_) => Err(serde::de::Error::custom(format_args!(
            "link target {href} is not one of the site's pages"
        ))),
        Ok(route) => Ok(route),
    }
}

/// A legal record in `data/` that doesn't parse.
#[derive(Debug, thiserror::Error)]
#[error("{file} is not a valid legal record")]
pub struct LegalError {
    file: &'static str,
    #[source]
    source: serde_json::Error,
}

fn parse<T: DeserializeOwned>(file: &'static str, text: &str) -> Result<T, LegalError> {
    serde_json::from_str(text).map_err(|source| LegalError { file, source })
}

/// Parses the record at `data/<path>`, naming that path if it fails.
macro_rules! record {
    ($path:expr) => {
        parse(
            concat!("data/", $path),
            include_str!(concat!("../data/", $path)),
        )
    };
}

/// Parses an archived version at `data/archive/<directory>/<slug>.json`; its slug is
/// its file name.
macro_rules! archived {
    ($directory:literal, $slug:literal) => {
        record!(concat!("archive/", $directory, "/", $slug, ".json")).map(
            |version: ArchivedVersion| ArchivedVersion {
                slug: $slug,
                ..version
            },
        )
    };
}

/// Every legal record, loaded once at startup and read by the routes and pages.
pub struct Legal {
    duet_privacy: LegalDocument,
    duet_terms: LegalDocument,
    consumer_health_data: LegalDocument,
    privacy: LegalDocument,
    terms: LegalDocument,
    clinical_transparency: LegalDocument,
    /// Each archive's versions, newest first.
    duet_privacy_archive: Vec<ArchivedVersion>,
    duet_terms_archive: Vec<ArchivedVersion>,
}

impl Legal {
    pub fn load() -> Result<Self, LegalError> {
        Ok(Self {
            duet_privacy: record!("duet-privacy.json")?,
            duet_terms: record!("duet-terms.json")?,
            consumer_health_data: record!("consumer-health-data.json")?,
            privacy: record!("privacy.json")?,
            terms: record!("terms.json")?,
            clinical_transparency: record!("clinical-transparency.json")?,
            duet_privacy_archive: vec![
                archived!("duet-privacy", "2026-09-24")?,
                archived!("duet-privacy", "2026-09-23")?,
                archived!("duet-privacy", "2026-09-09")?,
            ],
            duet_terms_archive: vec![archived!("duet-terms", "2026-09-08")?],
        })
    }

    pub fn document(&self, document: Document) -> &LegalDocument {
        match document {
            Document::WithArchive(Archive::DuetPrivacy) => &self.duet_privacy,
            Document::WithArchive(Archive::DuetTerms) => &self.duet_terms,
            Document::ConsumerHealthData => &self.consumer_health_data,
            Document::Privacy => &self.privacy,
            Document::Terms => &self.terms,
            Document::ClinicalTransparency => &self.clinical_transparency,
        }
    }

    /// An archive's earlier versions, newest first.
    pub fn versions(&self, archive: Archive) -> &[ArchivedVersion] {
        match archive {
            Archive::DuetPrivacy => &self.duet_privacy_archive,
            Archive::DuetTerms => &self.duet_terms_archive,
        }
    }

    pub fn version(&self, archive: Archive, slug: &str) -> Option<&ArchivedVersion> {
        self.versions(archive)
            .iter()
            .find(|version| version.slug == slug)
    }
}
