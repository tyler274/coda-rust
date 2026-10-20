use super::NotFound;
use super::legal_content::{ARCHIVES, CURRENT, Document, LegalPage, items, text};
use dioxus::prelude::*;

fn current(kind: &str) -> Document {
    let duet = &CURRENT["duet"];
    let coda = &CURRENT["coda"];
    let (eyebrow, title, intro, effective, summary, sections, archive) = match kind {
        "duet-privacy" => (
            "Duet",
            "Privacy Policy",
            "Coda Compute, Inc., doing business as Coda Health, provides Duet. This policy explains what Duet collects, how we use and protect it, and the choices you have.",
            &duet["duetPrivacyEffective"],
            &duet["duetPrivacySummary"],
            &duet["duetPrivacySections"],
            "duetPrivacyArchive",
        ),
        "duet-terms" => (
            "Duet",
            "Terms of Service",
            "These Terms govern your use of Duet and its related services, provided by Coda Compute, Inc., doing business as Coda Health.",
            &duet["duetTermsEffective"],
            &duet["duetTermsSummary"],
            &duet["duetTermsSections"],
            "duetTermsArchive",
        ),
        "consumer" => (
            "Duet",
            "Consumer Health Data Privacy Policy",
            "Coda Compute, Inc., doing business as Coda Health, provides Duet. This policy describes the consumer health data Duet collects and shares, and how to use your rights under Washington's My Health My Data Act (RCW 19.373).",
            &duet["duetPrivacyEffective"],
            &serde_json::Value::Null,
            &duet["duetConsumerHealthDataSections"],
            "",
        ),
        "privacy" => (
            "Coda Health",
            "Privacy Policy",
            "This Privacy Policy explains how Coda Health collects, uses, shares, and protects information through its website and related services.",
            &coda["lastUpdated"],
            &serde_json::Value::Null,
            &coda["privacySections"],
            "",
        ),
        _ => (
            "Coda Health",
            "Terms of Service",
            "These Terms govern use of the Coda Health website and related services.",
            &coda["lastUpdated"],
            &serde_json::Value::Null,
            &coda["termsSections"],
            "",
        ),
    };
    let base = if kind == "duet-privacy" {
        "/duet/privacy"
    } else {
        "/duet/terms"
    };
    Document {
        eyebrow: eyebrow.into(),
        title: title.into(),
        intro: intro.into(),
        effective: text(effective),
        date_label: "Effective".into(),
        summary: items(summary),
        sections: items(sections),
        version: (kind == "duet-terms").then(|| text(&duet["duetTermsVersion"])),
        links_title: "Earlier versions".into(),
        links: items(&ARCHIVES[archive])
            .iter()
            .map(|entry| {
                (
                    format!("{base}/{}", text(&entry["slug"])),
                    text(
                        &entry[if kind == "duet-privacy" {
                            "lastUpdated"
                        } else {
                            "effective"
                        }],
                    ),
                )
            })
            .collect(),
        ..Default::default()
    }
}
#[component]
pub fn DuetPrivacy() -> Element {
    rsx! { LegalPage { document: current("duet-privacy") } }
}
#[component]
pub fn DuetTerms() -> Element {
    rsx! { LegalPage { document: current("duet-terms") } }
}
#[component]
pub fn ConsumerHealthData() -> Element {
    rsx! { LegalPage { document: current("consumer") } }
}
#[component]
pub fn Privacy() -> Element {
    rsx! { LegalPage { document: current("privacy") } }
}
#[component]
pub fn Terms() -> Element {
    rsx! { LegalPage { document: current("terms") } }
}

fn archived(version: String, privacy: bool) -> Element {
    let archive = if privacy {
        "duetPrivacyArchive"
    } else {
        "duetTermsArchive"
    };
    let entries = items(&ARCHIVES[archive]);
    let Some(entry) = entries
        .iter()
        .find(|entry| entry["slug"].as_str() == Some(&version))
    else {
        return rsx! { NotFound { segments: vec![] } };
    };
    let title = if privacy {
        "Privacy Policy"
    } else {
        "Terms of Service"
    };
    let document = Document {
        eyebrow: "Duet".into(),
        title: title.into(),
        intro: if privacy {
            "This is an earlier version of the Duet Privacy Policy, kept unchanged for reference."
        } else {
            "This page keeps this version of the Duet Terms of Service unchanged for reference."
        }
        .into(),
        effective: text(&entry["effective"]),
        revised: Some(text(&entry["lastUpdated"])),
        date_label: "Effective".into(),
        version: entry["version"].as_str().map(str::to_owned),
        summary: items(&entry["summary"]),
        sections: items(&entry["sections"]),
        links_title: "Current version".into(),
        links: vec![(
            if privacy {
                "/duet/privacy"
            } else {
                "/duet/terms"
            }
            .into(),
            title.into(),
        )],
    };
    rsx! { LegalPage { document } }
}
#[component]
pub fn DuetPrivacyArchive(version: String) -> Element {
    archived(version, true)
}
#[component]
pub fn DuetTermsArchive(version: String) -> Element {
    archived(version, false)
}
