//! Public page components, rendered once at startup.
mod brand;
mod duet;
mod home;
mod invite;
mod legal;
mod not_found;
mod phones;

pub use duet::Duet;
pub use home::Home;
pub use invite::Invite;
pub use legal::{
    ClinicalTransparency, ConsumerHealthData, DuetPrivacy, DuetPrivacyArchive, DuetTerms,
    DuetTermsArchive, Privacy, Terms,
};
pub use not_found::NotFound;

#[cfg(test)]
mod tests {
    use crate::{Config, Route, legal::Legal, prerender::render_page};
    use std::rc::Rc;

    #[test]
    fn every_document_renders_its_body() {
        let legal = Rc::new(Legal::load().unwrap());
        for route in Route::pages(&legal) {
            let page = render_page(&route, &Config::default(), &legal);
            let html = std::str::from_utf8(&page.html).unwrap();
            assert!(html.contains("<h1"), "missing page body: {route}");
            assert!(!html.contains("This page is being rebuilt"));
            assert!(html.contains("Consumer Health Data Privacy Policy"));
        }
    }

    #[test]
    fn home_preserves_motion_targets_and_shared_phones() {
        let legal = Rc::new(Legal::load().unwrap());
        let page = render_page(&Route::Home {}, &Config::default(), &legal);
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
        let duet = render_page(&Route::Duet {}, &Config::default(), &legal);
        assert!(
            std::str::from_utf8(&duet.html)
                .unwrap()
                .contains("src=\"/homepage/site.js\"")
                && std::str::from_utf8(&duet.html)
                    .unwrap()
                    .contains("src=\"/homepage/duet.js\"")
        );
        let privacy = render_page(&Route::Privacy {}, &Config::default(), &legal);
        assert!(
            !std::str::from_utf8(&privacy.html)
                .unwrap()
                .contains("site.js")
        );
    }
}
