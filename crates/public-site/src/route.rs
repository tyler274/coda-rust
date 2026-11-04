use crate::{chrome::Chrome, pages::*, site};
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

/// The site's two page systems. Each decides a document's stylesheets and the markup
/// around its main content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shell {
    /// The homepage and /duet: the homepage styles and motion script.
    Homepage,
    /// Every other page: the compiled Tailwind stylesheet.
    Utility,
}

impl Route {
    pub fn shell(&self) -> Shell {
        match self {
            Self::Home {} | Self::Duet {} => Shell::Homepage,
            _ => Shell::Utility,
        }
    }

    /// Finite documents use the same frozen archive enumeration as their legal links.
    pub fn pages() -> Vec<Self> {
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
        use crate::pages::legal_content::{ARCHIVES, items, text};
        pages.extend(items(&ARCHIVES["duetPrivacyArchive"]).iter().map(|entry| {
            Self::DuetPrivacyArchive {
                version: text(&entry["slug"]),
            }
        }));
        pages.extend(items(&ARCHIVES["duetTermsArchive"]).iter().map(|entry| {
            Self::DuetTermsArchive {
                version: text(&entry["slug"]),
            }
        }));
        pages
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

    pub fn meta(&self) -> PageMeta {
        let (title, description) = match self {
            Self::Home {} => (site::HOME_TITLE.to_owned(), "Coda is a physician-directed AI care team that handles what happens between visits.".to_owned()),
            Self::Duet {} => ("Duet".into(), "Duet helps you bring your health information together, notice what matters, and prepare for conversations with your doctor.".into()),
            Self::DuetPrivacy {} => ("Duet Privacy Policy".into(), "How Duet handles the health information and sources you choose to connect.".into()),
            Self::DuetPrivacyArchive { version } => {
                use crate::pages::legal_content::{ARCHIVES, items, text};
                let updated = items(&ARCHIVES["duetPrivacyArchive"]).iter().find(|entry| entry["slug"].as_str() == Some(version.as_str())).map(|entry| text(&entry["lastUpdated"])).unwrap_or_default();
                (format!("Duet Privacy Policy, last updated {updated}"), "An earlier version of the Privacy Policy for Duet by Coda Health.".into())
            }
            Self::DuetTerms {} => ("Duet Terms of Service".into(), "Terms of Service for Duet, Coda Health's patient-facing iPhone app.".into()),
            Self::DuetTermsArchive { version } => (format!("Duet Terms of Service, version {version}"), format!("Version {version} of the Terms of Service for Duet by Coda Health.")),
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
                site::absolute_url(&self.to_string())
            },
            image_url: match self {
                Self::Duet {} => site::absolute_url("/brand/duet/social-preview-v1.png"),
                Self::Scheduling {} => site::absolute_url("/scheduling/dev/opengraph-image"),
                Self::NotFound { .. } => String::new(),
                _ => site::absolute_url("/brand/coda-health-social-preview-v4.png"),
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
