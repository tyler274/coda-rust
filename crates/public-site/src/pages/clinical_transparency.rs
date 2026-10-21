use super::legal_content::{CURRENT, Document, LegalPage, items, text};
use dioxus::prelude::*;

#[component]
pub fn ClinicalTransparency() -> Element {
    let document = Document {
        eyebrow: "Clinical Transparency".into(),
        title: "How Coda supports physician-directed care.".into(),
        intro: "Coda helps physicians maintain context, review change, and carry follow-up across the spaces between visits. This page describes what Coda is designed to do, what evidence it uses, what a physician can inspect, and where its authority ends.".into(),
        effective: text(&CURRENT["clinical"]["lastUpdated"]),
        date_label: "Updated".into(), sections: items(&CURRENT["clinical"]["sections"]),
        ..Default::default()
    };
    rsx! { LegalPage { document } }
}
