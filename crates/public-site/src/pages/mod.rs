//! Public page components, rendered once at startup.
mod brand;
mod clinical_transparency;
mod duet;
mod home;
mod legal;
#[path = "../legal.rs"]
pub(crate) mod legal_content;
mod not_found;
mod phones;
mod scheduling;

pub use clinical_transparency::ClinicalTransparency;
pub use duet::Duet;
pub use home::Home;
pub use legal::{
    ConsumerHealthData, DuetPrivacy, DuetPrivacyArchive, DuetTerms, DuetTermsArchive, Privacy,
    Terms,
};
pub use not_found::NotFound;
pub use scheduling::Scheduling;

#[cfg(test)]
mod tests {
    use crate::{Config, Route, prerender::render_page};

    #[test]
    fn every_document_renders_its_body() {
        for route in Route::pages() {
            let page = render_page(&route, &Config::default());
            let html = std::str::from_utf8(&page.html).unwrap();
            assert!(html.contains("<h1"), "missing page body: {route}");
            assert!(!html.contains("This page is being rebuilt"));
            assert!(html.contains("Consumer Health Data Privacy Policy"));
        }
    }

    #[test]
    fn home_preserves_motion_targets_and_shared_phones() {
        let page = render_page(&Route::Home {}, &Config::default());
        let html = std::str::from_utf8(&page.html).unwrap();
        for marker in [
            "id=\"heroStage\"",
            "id=\"memStage\"",
            "id=\"patients\"",
            "data-art=\"mist\"",
            "data-w=\"\"",
            "class=\"stage duet-story\"",
            "src=\"/homepage/site.js\"",
            "src=\"/homepage/duet.js\"",
        ] {
            assert!(html.contains(marker), "missing motion target {marker}");
        }
        let duet = render_page(&Route::Duet {}, &Config::default());
        assert!(
            std::str::from_utf8(&duet.html)
                .unwrap()
                .contains("src=\"/homepage/site.js\"")
                && std::str::from_utf8(&duet.html)
                    .unwrap()
                    .contains("src=\"/homepage/duet.js\"")
        );
        let privacy = render_page(&Route::Privacy {}, &Config::default());
        assert!(
            !std::str::from_utf8(&privacy.html)
                .unwrap()
                .contains("site.js")
        );
    }
}
