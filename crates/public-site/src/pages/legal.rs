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
        header { class: "hero pb-14 sm:pb-16",
            div { class: "hero-copy",
                p { class: "kicker", "{page.eyebrow}" }
                h1 { "{page.title}" }
                p { class: "lede", "{page.intro}" }
                p { class: "hero-meta micro",
                    if let Some(version) = &page.version { span { class: "mb-2 block", "Version {version}" } }
                    "{page.date_label} {page.effective}"
                    if let Some(revised) = &page.revised { span { class: "mt-2 block", "Last updated {revised}" } }
                }
            }
        }
        div { class: "wrap pb-20 sm:pb-28",
            if !page.summary.is_empty() {
                div { class: "card mb-10 p-6 sm:mb-14 sm:p-8",
                    h2 { class: "micro text-accent", "The short version" }
                    ul { class: "mt-5 grid gap-x-10 gap-y-3 min-[900px]:grid-cols-2",
                        for point in &page.summary {
                            li { class: "relative pl-5 text-[16px] leading-[1.6] text-ink before:absolute before:left-0 before:top-[0.72em] before:h-1.5 before:w-1.5 before:rounded-full before:bg-accent", "{point}" }
                        }
                    }
                }
            }
            for (index, section) in page.sections.iter().enumerate() {
                section { id: section.id.clone(), class: "scroll-mt-28 border-t border-hairline py-10 sm:py-12 min-[900px]:grid min-[900px]:grid-cols-[5fr_7fr] min-[900px]:gap-x-16",
                    div {
                        span { class: "idx", i {} "{index + 1:02}" }
                        h2 { class: "mt-3 max-w-[14em] text-[22px] leading-snug tracking-[-0.02em] text-ink min-[900px]:text-[26px]", "{section.title}" }
                    }
                    div { class: "mt-5 min-w-0 space-y-4 min-[900px]:mt-0",
                        for block in section.body.iter().cloned() { LegalBlock { block } }
                    }
                }
            }
            if !page.links.is_empty() {
                p { class: "flex flex-wrap items-baseline gap-x-3 gap-y-2 border-t border-hairline pt-8 text-[14px] font-medium text-ink-mid",
                    span { class: "micro mr-2", "{page.links_title}" }
                    for (index, (href, label)) in page.links.iter().enumerate() {
                        span { class: "flex items-baseline gap-x-3",
                            if index > 0 { span { aria_hidden: "true", class: "text-ink-faint", "·" } }
                            a { href: "{href}", class: "underline decoration-accent decoration-1 underline-offset-4 hover:text-accent", "{label}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LegalBlock(block: Block) -> Element {
    let paragraph_class = "text-[16px] leading-[1.7] text-ink-mid";
    match block {
        Block::Paragraph(paragraph) => rsx! { p { class: paragraph_class, "{paragraph}" } },
        // Notice sources are plain https:// addresses, so their authority is the host.
        Block::Source(url) => {
            rsx! { p { class: paragraph_class, a { href: "{url}", class: "break-words underline underline-offset-4", "Source and license: {url.authority()}" } } }
        }
        Block::Linked(LinkedParagraph {
            before,
            link,
            after,
        }) => {
            rsx! { p { class: paragraph_class,
                "{before}"
                a { href: "{link.href}", class: "text-ink! underline! decoration-accent! decoration-1! underline-offset-4 hover:text-accent!", "{link.label}" }
                "{after}"
            } }
        }
        Block::List(list) => rsx! { ul { class: "space-y-2",
            for item in &list {
                li { class: "relative pl-5 text-[16px] leading-[1.7] text-ink-mid before:absolute before:left-0 before:top-[0.72em] before:h-1.5 before:w-1.5 before:rounded-full before:bg-accent", "{item}" }
            }
        } },
        Block::Table(Table { columns, rows }) => {
            rsx! {
            table { class: "w-full border-collapse text-left text-[0.92rem] leading-7 text-ink-mid max-md:hidden",
                thead { tr { class: "border-b border-hairline-strong",
                    for column in &columns { th { scope: "col", class: "py-2.5 pr-5 align-bottom last:pr-0 micro text-ink-faint", "{column}" } }
                } }
                tbody {
                    for row in &rows {
                        tr { class: "border-b border-hairline last:border-b-0",
                            for (index, cell) in row.iter().enumerate() {
                                if index == 0 { th { scope: "row", class: "py-3 pr-5 align-top font-medium text-ink", "{cell}" } }
                                else { td { class: "py-3 pr-5 align-top last:pr-0", "{cell}" } }
                            }
                        }
                    }
                }
            }
            div { class: "md:hidden",
                for row in &rows {
                    dl { class: "border-b border-hairline py-3 text-[0.92rem] leading-7 text-ink-mid last:border-b-0",
                        for (index, (cell, column)) in row.iter().zip(&columns).enumerate() {
                            div { class: "py-1",
                                dt { class: if index == 0 { "sr-only" } else { "micro text-ink-faint" }, "{column}" }
                                dd { class: if index == 0 { "font-medium text-ink" } else { "" }, "{cell}" }
                            }
                        }
                    }
                }
            }
            }
        }
    }
}
