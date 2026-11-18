use crate::{
    chrome::Chrome,
    legal::{Archive, Document, Legal},
    pages::*,
    site,
};
use dioxus::prelude::*;

/// Single route authority for both Dioxus layout rendering and startup enumeration.
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Chrome)]
    #[route("/")]
    Home {},
    #[route("/duet")]
    Duet {},
    #[route("/duet/privacy")]
    DuetPrivacy {},
    #[route("/duet/privacy/:version")]
    DuetPrivacyArchive { version: String },
    #[route("/duet/terms")]
    DuetTerms {},
    #[route("/duet/terms/:version")]
    DuetTermsArchive { version: String },
    #[route("/duet/consumer-health-data")]
    ConsumerHealthData {},
    #[route("/privacy")]
    Privacy {},
    #[route("/terms")]
    Terms {},
    #[route("/clinical-transparency")]
    ClinicalTransparency {},
    #[route("/scheduling/dev")]
    Scheduling {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageMeta {
    pub title: String,
    pub description: String,
    pub canonical_url: String,
    pub image_url: String,
    pub image_alt: String,
    pub noindex: bool,
}

/// The site's two page families, which differ in the stylesheets they load and the
/// layout around their main content. Both use `chrome.css`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shell {
    /// The homepage and /duet: the homepage styles and motion script.
    Homepage,
    /// Every other page: the legal documents, scheduling and not-found, styled by
    /// `document.css`.
    Document,
}

impl Route {
    pub fn shell(&self) -> Shell {
        match self {
            Self::Home {} | Self::Duet {} => Shell::Homepage,
            _ => Shell::Document,
        }
    }

    /// Finite documents use the same archive enumeration as their legal links.
    pub fn pages(legal: &Legal) -> Vec<Self> {
        let mut pages = vec![
            Self::Home {},
            Self::Duet {},
            Self::DuetPrivacy {},
            Self::DuetTerms {},
            Self::ConsumerHealthData {},
            Self::Privacy {},
            Self::Terms {},
            Self::ClinicalTransparency {},
            Self::Scheduling {},
        ];
        for archive in [Archive::DuetPrivacy, Archive::DuetTerms] {
            pages.extend(
                legal
                    .versions(archive)
                    .iter()
                    .map(|version| Self::archived(archive, version.slug)),
            );
        }
        pages
    }

    /// A current legal document's page.
    pub fn document(document: Document) -> Self {
        match document {
            Document::WithArchive(Archive::DuetPrivacy) => Self::DuetPrivacy {},
            Document::WithArchive(Archive::DuetTerms) => Self::DuetTerms {},
            Document::ConsumerHealthData => Self::ConsumerHealthData {},
            Document::Privacy => Self::Privacy {},
            Document::Terms => Self::Terms {},
            Document::ClinicalTransparency => Self::ClinicalTransparency {},
        }
    }

    /// The page of one archived version.
    pub fn archived(archive: Archive, slug: &str) -> Self {
        let version = slug.to_owned();
        match archive {
            Archive::DuetPrivacy => Self::DuetPrivacyArchive { version },
            Archive::DuetTerms => Self::DuetTermsArchive { version },
        }
    }

    /// The pages the sitemap lists, in order: current documents only.
    /// Archived policy versions stay reachable from their policy pages, not advertised.
    pub fn sitemap() -> Vec<Self> {
        vec![
            Self::Home {},
            Self::Duet {},
            Self::ClinicalTransparency {},
            Self::Privacy {},
            Self::Terms {},
            Self::DuetPrivacy {},
            Self::DuetTerms {},
            Self::ConsumerHealthData {},
        ]
    }

    /// The page's metadata against the default canonical origin (`/`).
    pub fn meta(&self, legal: &Legal) -> PageMeta {
        self.meta_for(legal, &site::SiteOrigin::default(), &site::BasePath::root())
    }

    /// The page's metadata resolved against an explicit origin and base path.
    /// An archived version the records don't hold gets the not-found page's.
    pub fn meta_for(
        &self,
        legal: &Legal,
        origin: &site::SiteOrigin,
        base_path: &site::BasePath,
    ) -> PageMeta {
        let (title, description) = match self {
            Self::Home {} => (site::HOME_TITLE.to_owned(), "Coda is a physician-directed AI care team that handles what happens between visits.".to_owned()),
            Self::Duet {} => ("Duet".into(), "Duet helps you bring your health information together, notice what matters, and prepare for conversations with your doctor.".into()),
            Self::DuetPrivacy {} => ("Duet Privacy Policy".into(), "How Duet handles the health information and sources you choose to connect.".into()),
            Self::DuetPrivacyArchive { version } => match legal.version(Archive::DuetPrivacy, version) {
                Some(entry) => (format!("Duet Privacy Policy, last updated {}", entry.last_updated), "An earlier version of the Privacy Policy for Duet by Coda Health.".into()),
                None => return Self::NotFound { segments: Vec::new() }.meta_for(legal, origin, base_path),
            },
            Self::DuetTerms {} => ("Duet Terms of Service".into(), "Terms of Service for Duet, Coda Health's patient-facing iPhone app.".into()),
            Self::DuetTermsArchive { version } => match legal.version(Archive::DuetTerms, version).and_then(|entry| entry.version.as_deref()) {
                Some(accepted) => (format!("Duet Terms of Service, version {accepted}"), format!("Version {accepted} of the Terms of Service for Duet by Coda Health.")),
                None => return Self::NotFound { segments: Vec::new() }.meta_for(legal, origin, base_path),
            },
            Self::ConsumerHealthData {} => ("Duet Consumer Health Data Privacy Policy".into(), "Consumer Health Data Privacy Policy for Duet by Coda Health, under Washington's My Health My Data Act.".into()),
            Self::Privacy {} => ("Privacy Policy".into(), "How Coda Health handles information through its website and related services.".into()),
            Self::Terms {} => ("Terms of Service".into(), "Terms of Service for the Coda Health website and related services.".into()),
            Self::ClinicalTransparency {} => ("Clinical Transparency".into(), "How Chords supports physician-directed care, preserves reviewable evidence, and keeps clinical authority with the physician.".into()),
            Self::Scheduling {} => ("Schedule with Dev Patale".into(), "Book a conversation with Dev Patale about clinical environments, health-agent evaluation, and health AI research.".into()),
            Self::NotFound { .. } => ("Page Not Found".into(), "The requested page could not be found.".into()),
        };
        PageMeta {
            title: if matches!(self, Self::Home {}) {
                title
            } else {
                format!("{title} | {}", site::SITE_NAME)
            },
            description,
            canonical_url: if matches!(self, Self::NotFound { .. }) {
                String::new()
            } else {
                origin.absolute_url(base_path, &self.to_string())
            },
            image_url: match self {
                Self::Duet {} => {
                    origin.absolute_url(base_path, "/brand/duet/social-preview-v1.png")
                }
                Self::Scheduling {} => {
                    origin.absolute_url(base_path, "/scheduling/dev/opengraph-image")
                }
                Self::NotFound { .. } => String::new(),
                _ => origin.absolute_url(base_path, "/brand/coda-health-social-preview-v4.png"),
            },
            image_alt: match self {
                Self::Duet {} => "Duet by Coda Health. A clearer picture of you.",
                Self::Scheduling {} => "Schedule with Dev Patale at Coda Health",
                Self::NotFound { .. } => "",
                _ => "Coda Health: Coda handles what happens between visits.",
            }
            .into(),
            noindex: matches!(self, Self::Scheduling {} | Self::NotFound { .. }),
        }
    }
}
