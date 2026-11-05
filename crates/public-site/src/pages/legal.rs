//! The legal pages: each current document, and each archived version of the documents
//! that keep their earlier versions.
use super::NotFound;
use crate::{
    Route,
    legal::{Archive, ArchivedVersion, Block, Document, Legal, LinkedParagraph, Section, Table},
};
use dioxus::prelude::*;
use std::rc::Rc;

/// The date that identifies an earlier version: privacy by its last update, terms by
/// its effective date.
fn label(archive: Archive, version: &ArchivedVersion) -> &str {
    match archive {
        Archive::DuetPrivacy => &version.last_updated,
        Archive::DuetTerms => &version.effective,
    }
}

fn current(document: Document) -> Page {
    let legal = use_context::<Rc<Legal>>();
    let record = legal.document(document);
    // A document that keeps no earlier versions has none to link.
    let links = match document.archive() {
        Some(archive) => legal
            .versions(archive)
            .iter()
            .map(|version| {
                (
                    Route::archived(archive, version.slug).to_string(),
                    label(archive, version).to_owned(),
                )
            })
            .collect(),
        None => Vec::new(),
    };
    Page {
        eyebrow: record.eyebrow.clone(),
        title: record.title.clone(),
        intro: record.intro.clone(),
        effective: record.effective.clone(),
        revised: None,
        date_label: record.date_label.clone(),
        version: record.version.clone(),
        summary: record.summary.clone(),
        sections: record.sections.clone(),
        links_title: "Earlier versions".into(),
        links,
    }
}

#[component]
pub fn DuetPrivacy() -> Element {
    rsx! { LegalPage { page: current(Document::WithArchive(Archive::DuetPrivacy)) } }
}
#[component]
pub fn DuetTerms() -> Element {
    rsx! { LegalPage { page: current(Document::WithArchive(Archive::DuetTerms)) } }
}
#[component]
pub fn ConsumerHealthData() -> Element {
    rsx! { LegalPage { page: current(Document::ConsumerHealthData) } }
}
#[component]
pub fn Privacy() -> Element {
    rsx! { LegalPage { page: current(Document::Privacy) } }
}
#[component]
pub fn Terms() -> Element {
    rsx! { LegalPage { page: current(Document::Terms) } }
}
#[component]
pub fn ClinicalTransparency() -> Element {
    rsx! { LegalPage { page: current(Document::ClinicalTransparency) } }
}

fn archived(archive: Archive, slug: &str) -> Element {
    let legal = use_context::<Rc<Legal>>();
    let Some(version) = legal.version(archive, slug) else {
        return rsx! { NotFound { segments: vec![] } };
    };
    let document = Document::WithArchive(archive);
    let current = legal.document(document);
    let intro = match archive {
        Archive::DuetPrivacy => {
            "This is an earlier version of the Duet Privacy Policy, kept unchanged for reference."
        }
        Archive::DuetTerms => {
            "This page keeps this version of the Duet Terms of Service unchanged for reference."
        }
    };
    let page = Page {
        eyebrow: version.eyebrow.clone(),
        title: version.title.clone(),
        intro: intro.into(),
        effective: version.effective.clone(),
        revised: Some(version.last_updated.clone()),
        date_label: "Effective".into(),
        version: version.version.clone(),
        summary: version.summary.clone(),
        sections: version.sections.clone(),
        links_title: "Current version".into(),
        links: vec![(Route::document(document).to_string(), current.title.clone())],
    };
    rsx! { LegalPage { page } }
}
#[component]
pub fn DuetPrivacyArchive(version: String) -> Element {
    archived(Archive::DuetPrivacy, &version)
}
#[component]
pub fn DuetTermsArchive(version: String) -> Element {
    archived(Archive::DuetTerms, &version)
}

/// What a legal page shows: a document's text and the page's links.
#[derive(Clone, PartialEq)]
struct Page {
    eyebrow: String,
    title: String,
    intro: String,
    effective: String,
    revised: Option<String>,
    date_label: String,
    version: Option<String>,
    summary: Vec<String>,
    sections: Vec<Section>,
    links_title: String,
    links: Vec<(String, String)>,
}

#[component]
fn LegalPage(page: Page) -> Element {
    rsx! {
        header { class: "hero legal-hero",
            div { class: "hero-copy",
                p { class: "kicker", "{page.eyebrow}" }
                h1 { "{page.title}" }
                p { class: "lede", "{page.intro}" }
                p { class: "hero-meta micro",
                    if let Some(version) = &page.version { span { class: "legal-version", "Version {version}" } }
                    "{page.date_label} {page.effective}"
                    if let Some(revised) = &page.revised { span { class: "legal-revised", "Last updated {revised}" } }
                }
            }
        }
        div { class: "wrap legal-body",
            if !page.summary.is_empty() {
                div { class: "card legal-summary",
                    h2 { class: "micro", "The short version" }
                    ul {
                        for point in &page.summary {
                            li { "{point}" }
                        }
                    }
                }
            }
            for (index, section) in page.sections.iter().enumerate() {
                section { id: section.id.clone(), class: "legal-section",
                    div {
                        span { class: "idx", i {} "{index + 1:02}" }
                        h2 { "{section.title}" }
                    }
                    div { class: "legal-section-body",
                        for block in section.body.iter().cloned() { LegalBlock { block } }
                    }
                }
            }
            if !page.links.is_empty() {
                p { class: "legal-links",
                    span { class: "micro legal-links-title", "{page.links_title}" }
                    for (index, (href, label)) in page.links.iter().enumerate() {
                        span { class: "legal-links-item",
                            if index > 0 { span { aria_hidden: "true", class: "legal-links-sep", "·" } }
                            a { href: "{href}", "{label}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LegalBlock(block: Block) -> Element {
    match block {
        Block::Paragraph(paragraph) => rsx! { p { "{paragraph}" } },
        // Notice sources are plain https:// addresses, so their authority is the host.
        Block::Source(url) => {
            rsx! { p { a { href: "{url}", class: "legal-source", "Source and license: {url.authority()}" } } }
        }
        Block::Linked(LinkedParagraph {
            before,
            link,
            after,
        }) => {
            rsx! { p {
                "{before}"
                a { href: "{link.href}", class: "legal-link", "{link.label}" }
                "{after}"
            } }
        }
        Block::List(list) => rsx! { ul { class: "legal-list",
            for item in &list {
                li { "{item}" }
            }
        } },
        Block::Table(Table { columns, rows }) => {
            rsx! {
            table { class: "legal-table",
                thead { tr {
                    for column in &columns { th { scope: "col", class: "micro", "{column}" } }
                } }
                tbody {
                    for row in &rows {
                        tr {
                            for (index, cell) in row.iter().enumerate() {
                                if index == 0 { th { scope: "row", "{cell}" } }
                                else { td { "{cell}" } }
                            }
                        }
                    }
                }
            }
            div { class: "legal-cards",
                for row in &rows {
                    dl {
                        for (index, (cell, column)) in row.iter().zip(&columns).enumerate() {
                            div {
                                dt { class: if index == 0 { "sr-only" } else { "micro" }, "{column}" }
                                dd { "{cell}" }
                            }
                        }
                    }
                }
            }
            }
        }
    }
}
