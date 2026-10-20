//! Frozen legal copy imported from the reference publication, not product runtime types.
use dioxus::prelude::*;
use serde_json::Value;
use std::sync::LazyLock;

pub static CURRENT: LazyLock<Value> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../data/current.json")).expect("valid current legal copy")
});
pub static ARCHIVES: LazyLock<Value> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../data/archives.json"))
        .expect("valid frozen legal archives")
});
pub fn text(value: &Value) -> String {
    value.as_str().unwrap_or_default().to_owned()
}
pub fn items(value: &Value) -> Vec<Value> {
    value.as_array().cloned().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publication_dates_and_acceptance_version_are_distinct() {
        assert_eq!(CURRENT["duet"]["duetTermsVersion"], "2026-09-23");
        assert_eq!(CURRENT["duet"]["duetTermsEffective"], "September 24, 2026");
        assert_eq!(CURRENT["duet"]["duetPrivacyEffective"], "October 3, 2026");
    }

    #[test]
    fn archives_keep_the_frozen_sections() {
        for (kind, slug, original) in [
            (
                "duetPrivacyArchive",
                "2026-09-09",
                include_str!("../data/privacy-2026-09-09.json"),
            ),
            (
                "duetPrivacyArchive",
                "2026-09-23",
                include_str!("../data/privacy-2026-09-23.json"),
            ),
            (
                "duetPrivacyArchive",
                "2026-09-24",
                include_str!("../data/privacy-2026-09-24.json"),
            ),
            (
                "duetTermsArchive",
                "2026-09-08",
                include_str!("../data/terms-2026-09-08.json"),
            ),
        ] {
            let entry = ARCHIVES[kind]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["slug"] == slug)
                .unwrap();
            assert_eq!(
                entry["sections"],
                serde_json::from_str::<Value>(original).unwrap()
            );
        }
    }

    #[test]
    fn terminology_notices_remain_in_current_terms() {
        let notices: Vec<Value> =
            serde_json::from_str(include_str!("../data/clinical-terminology-notices.json"))
                .unwrap();
        assert!(items(&CURRENT["duet"]["duetTermsSections"]).ends_with(&notices));
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct Document {
    pub eyebrow: String,
    pub title: String,
    pub intro: String,
    pub effective: String,
    pub revised: Option<String>,
    pub date_label: String,
    pub version: Option<String>,
    pub summary: Vec<Value>,
    pub sections: Vec<Value>,
    pub links_title: String,
    pub links: Vec<(String, String)>,
}

#[component]
pub fn LegalPage(document: Document) -> Element {
    rsx! {
        header { class: "hero pb-14 sm:pb-16",
            div { class: "hero-copy",
                p { class: "kicker", "{document.eyebrow}" }
                h1 { "{document.title}" }
                p { class: "lede", "{document.intro}" }
                p { class: "hero-meta micro",
                    if let Some(version) = &document.version { span { class: "mb-2 block", "Version {version}" } }
                    "{document.date_label} {document.effective}"
                    if let Some(revised) = &document.revised { span { class: "mt-2 block", "Last updated {revised}" } }
                }
            }
        }
        div { class: "wrap pb-20 sm:pb-28",
            if !document.summary.is_empty() {
                div { class: "card mb-10 p-6 sm:mb-14 sm:p-8",
                    h2 { class: "micro text-accent", "The short version" }
                    ul { class: "mt-5 grid gap-x-10 gap-y-3 min-[900px]:grid-cols-2",
                        for point in &document.summary {
                            li { class: "relative pl-5 text-[16px] leading-[1.6] text-ink before:absolute before:left-0 before:top-[0.72em] before:h-1.5 before:w-1.5 before:rounded-full before:bg-accent", "{text(point)}" }
                        }
                    }
                }
            }
            for (index, section) in document.sections.iter().enumerate() {
                section { id: section["id"].as_str().map(str::to_owned), class: "scroll-mt-28 border-t border-hairline py-10 sm:py-12 min-[900px]:grid min-[900px]:grid-cols-[5fr_7fr] min-[900px]:gap-x-16",
                    div {
                        span { class: "idx", i {} "{index + 1:02}" }
                        h2 { class: "mt-3 max-w-[14em] text-[22px] leading-snug tracking-[-0.02em] text-ink min-[900px]:text-[26px]", {text(&section["title"])} }
                    }
                    div { class: "mt-5 min-w-0 space-y-4 min-[900px]:mt-0",
                        for block in items(&section["body"]) { LegalBlock { block } }
                    }
                }
            }
            if !document.links.is_empty() {
                p { class: "flex flex-wrap items-baseline gap-x-3 gap-y-2 border-t border-hairline pt-8 text-[14px] font-medium text-ink-mid",
                    span { class: "micro mr-2", "{document.links_title}" }
                    for (index, (href, label)) in document.links.iter().enumerate() {
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
fn LegalBlock(block: Value) -> Element {
    let paragraph_class = "text-[16px] leading-[1.7] text-ink-mid";
    if let Some(paragraph) = block.as_str() {
        if paragraph.starts_with("https://") {
            let host = paragraph
                .strip_prefix("https://")
                .unwrap()
                .split('/')
                .next()
                .unwrap_or_default();
            return rsx! { p { class: paragraph_class, a { href: paragraph, class: "break-words underline underline-offset-4", "Source and license: {host}" } } };
        }
        return rsx! { p { class: paragraph_class, "{paragraph}" } };
    }
    if let Some(link) = block.get("link") {
        let paragraph = text(&block["text"]);
        let label = text(&link["label"]);
        let (before, after) = paragraph.split_once(&label).unwrap_or((&paragraph, ""));
        return rsx! { p { class: paragraph_class,
            "{before}"
            a { href: text(&link["href"]), class: "text-ink! underline! decoration-accent! decoration-1! underline-offset-4 hover:text-accent!", "{label}" }
            "{after}"
        } };
    }
    if let Some(list) = block.get("list") {
        return rsx! { ul { class: "space-y-2",
            for item in items(list) {
                li { class: "relative pl-5 text-[16px] leading-[1.7] text-ink-mid before:absolute before:left-0 before:top-[0.72em] before:h-1.5 before:w-1.5 before:rounded-full before:bg-accent", "{text(&item)}" }
            }
        } };
    }
    let columns = items(&block["table"]["columns"]);
    let rows = items(&block["table"]["rows"]);
    rsx! {
        table { class: "w-full border-collapse text-left text-[0.92rem] leading-7 text-ink-mid max-md:hidden",
            thead { tr { class: "border-b border-hairline-strong",
                for column in &columns { th { scope: "col", class: "py-2.5 pr-5 align-bottom last:pr-0 micro text-ink-faint", "{text(column)}" } }
            } }
            tbody {
                for row in &rows {
                    tr { class: "border-b border-hairline last:border-b-0",
                        for (index, cell) in items(row).iter().enumerate() {
                            if index == 0 { th { scope: "row", class: "py-3 pr-5 align-top font-medium text-ink", "{text(cell)}" } }
                            else { td { class: "py-3 pr-5 align-top last:pr-0", "{text(cell)}" } }
                        }
                    }
                }
            }
        }
        div { class: "md:hidden",
            for row in &rows {
                dl { class: "border-b border-hairline py-3 text-[0.92rem] leading-7 text-ink-mid last:border-b-0",
                    for (index, cell) in items(row).iter().enumerate() {
                        div { class: "py-1",
                            dt { class: if index == 0 { "sr-only" } else { "micro text-ink-faint" }, "{text(&columns[index])}" }
                            dd { class: if index == 0 { "font-medium text-ink" } else { "" }, "{text(cell)}" }
                        }
                    }
                }
            }
        }
    }
}
