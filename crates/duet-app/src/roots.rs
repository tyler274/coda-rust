//! Root states for the two Duet tabs. Sentences are the ones the iPhone app shows.

use duet_client::EvidenceLibraryPage;
use duet_wire::models::{DoctorCollection, DoctorCollectionItem};

pub const CONNECTING: &str = "Connecting…";
pub const LIBRARY_EMPTY: &str = "No records or measurements yet.";
pub const LIBRARY_UNAVAILABLE: &str =
    "Your health information isn’t available right now. Try again in a moment.";
pub const TRY_AGAIN: &str = "Try again";
pub const ALLOW_NOTIFICATIONS: &str = "Allow notifications";
pub const OPEN_NOTIFICATION_SETTINGS: &str = "Open iPhone Settings";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoctorPage {
    /// The collection loaded and has nothing to list.
    Empty,
    /// Items, note, questions, and cover note, in that order.
    Lines(Vec<DoctorLine>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorLine {
    pub title: String,
    pub detail: Option<String>,
}

/// The lines a loaded For my doctor collection already contains.
pub fn doctor_page(collection: &DoctorCollection) -> DoctorPage {
    let mut entries = item_entries(collection.items.as_deref().unwrap_or(&[]));
    if let Some(note) = supplied(&collection.note) {
        entries.push(DoctorLine {
            title: note,
            detail: None,
        });
    }
    if let Some(page) = collection.page.as_deref() {
        if let Some(questions) = &page.questions {
            for question in questions {
                if let Some(title) = nonempty(question) {
                    entries.push(DoctorLine {
                        title,
                        detail: None,
                    });
                }
            }
        }
        if let Some(cover) = supplied(&page.cover_note) {
            entries.push(DoctorLine {
                title: cover,
                detail: Some("Cover note (not included in PDF)".into()),
            });
        }
    }
    if entries.is_empty() {
        DoctorPage::Empty
    } else {
        DoctorPage::Lines(entries)
    }
}

fn item_entries(items: &[DoctorCollectionItem]) -> Vec<DoctorLine> {
    items
        .iter()
        .filter_map(|item| {
            let title = nonempty(&item.title)?;
            Some(DoctorLine {
                title,
                detail: nonempty(&item.detail),
            })
        })
        .collect()
}

fn nonempty(text: &str) -> Option<String> {
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_owned())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryKind {
    HealthEvidence,
    ClinicalNote,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryRecord {
    pub id: String,
    pub kind: LibraryKind,
    pub title: String,
    pub summary: Option<String>,
    pub when: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibrarySection {
    pub name: &'static str,
    pub measured: bool,
    pub records: Vec<LibraryRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryRoot {
    Empty,
    Sections(Vec<LibrarySection>),
}

const SECTIONS: &[(&[&str], &str, bool)] = &[
    (&["medication"], "Medications", false),
    (&["allergy"], "Allergies", false),
    (
        &["currentCondition", "conditionOnRecord", "pastCondition"],
        "Conditions",
        false,
    ),
    (&["lab"], "Recent labs", true),
    (&["procedure"], "Procedures", false),
    (&["imaging"], "Imaging", false),
    (
        &[
            "resultReport",
            "clinicalDocument",
            "encounter",
            "immunization",
            "coverage",
            "visionPrescription",
            "other",
        ],
        "Records & documents",
        false,
    ),
];

pub fn library_root(page: &EvidenceLibraryPage) -> LibraryRoot {
    let mut present = std::collections::HashSet::new();
    for (key, count) in &page.counts {
        if *count > 0 {
            present.insert(key.clone());
        }
    }
    for entry in &page.entries {
        present.insert(entry.category.clone());
    }
    if present.is_empty() && page.entries.is_empty() && page.total_count == 0 {
        return LibraryRoot::Empty;
    }
    let sections = SECTIONS
        .iter()
        .filter(|(keys, _, _)| keys.iter().any(|key| present.contains(*key)))
        .map(|(keys, name, measured)| LibrarySection {
            name,
            measured: *measured,
            records: page
                .entries
                .iter()
                .filter(|entry| keys.contains(&entry.category.as_str()))
                .map(library_record)
                .collect(),
        })
        .collect();
    LibraryRoot::Sections(sections)
}

fn library_record(entry: &duet_wire::models::EvidenceLibraryEntry) -> LibraryRecord {
    let when = entry
        .date
        .map(|date| calendar_line(date.date_naive()))
        .or_else(|| entry.day.flatten().map(calendar_line));
    LibraryRecord {
        id: entry.id.clone(),
        kind: match entry.entity {
            duet_wire::models::evidence_library_entry::Entity::HealthEvidence => {
                LibraryKind::HealthEvidence
            }
            duet_wire::models::evidence_library_entry::Entity::ClinicalNote => {
                LibraryKind::ClinicalNote
            }
        },
        title: entry.title.clone(),
        summary: supplied(&entry.summary),
        when,
        source: supplied(&entry.source_label),
    }
}

fn supplied(value: &Option<Option<String>>) -> Option<String> {
    value
        .as_ref()
        .and_then(|text| text.as_ref())
        .map(|text| text.trim())
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

fn calendar_line(day: chrono::NaiveDate) -> String {
    let number = day.format("%e").to_string();
    format!(
        "{} {}, {}",
        day.format("%B"),
        number.trim(),
        day.format("%Y")
    )
}

pub fn default_api_base() -> &'static str {
    if cfg!(target_os = "android") {
        "http://10.0.2.2:8000"
    } else {
        "http://127.0.0.1:8000"
    }
}

/// Returns the Duet API origin for this build.
pub fn api_base_url() -> String {
    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    if let Some(window) = web_sys::window() {
        return window.location().origin().unwrap_or_default();
    }
    match option_env!("DUET_API_URL") {
        Some(url) if !url.is_empty() => url.to_owned(),
        _ => default_api_base().to_owned(),
    }
}

/// Returns the bearer this build sends.
pub fn api_bearer() -> String {
    // The dedicated Duet web build never embeds a bearer; the mode shell's
    // synthetic browser journeys still compile one in.
    #[cfg(not(all(target_arch = "wasm32", feature = "web")))]
    if let Some(bearer) = option_env!("DUET_BEARER")
        && !bearer.is_empty()
    {
        return bearer.to_owned();
    }
    recalled_session().unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
const SESSION_KEY: &str = "coda-duet-session";

/// Remove the old insecure browser credential. Native refresh is owned by AccountRoot.
pub fn recalled_session() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.remove_item(SESSION_KEY);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn json<T: serde::de::DeserializeOwned>(body: &str) -> T {
        serde_json::from_str(body).unwrap()
    }

    #[test]
    fn doctor_page_lists_the_saved_question() {
        let collection = json::<DoctorCollection>(
            r#"{"items":[{"detail":"How the week of walks has felt.","evidence":[],"id":"00000000-0000-0000-0000-000000000046","kind":"fact","position":0,"saved_at":"2026-10-09T15:00:02Z","source_id":"question-1","title":"Ask about sleep"}],"note_generation":1}"#,
        );
        assert_eq!(
            doctor_page(&collection),
            DoctorPage::Lines(vec![DoctorLine {
                title: "Ask about sleep".into(),
                detail: Some("How the week of walks has felt.".into()),
            }])
        );
        let empty = json::<DoctorCollection>(r#"{"note_generation":0}"#);
        assert_eq!(doctor_page(&empty), DoctorPage::Empty);
    }

    #[test]
    fn a_saved_doctor_page_uses_the_letter_layout() {
        let collection = json::<DoctorCollection>(
            r#"{"items":[],"note_generation":1,"page":{"name":"Ada Lovelace","date_of_birth":"1815-12-10","questions":["What changed?"],"cover_note":"Please review."}}"#,
        );
        // The opened saved-page summary is rendered outside DoctorLetter's printable preview.
        let DoctorPage::Lines(entries) = doctor_page(&collection) else {
            panic!("expected the saved-page summary");
        };
        assert!(entries.iter().any(|entry| entry.title == "Please review."
            && entry.detail.as_deref() == Some("Cover note (not included in PDF)")));
        let preview = crate::saved_preview(&collection).unwrap();
        assert!(preview.layout.text().contains("Ada Lovelace"));
        assert!(preview.layout.text().contains("What changed?"));
        assert!(!preview.layout.text().contains("Please review."));
        let mut without_cover = collection.clone();
        without_cover.page.as_mut().unwrap().cover_note = None;
        let plain = crate::saved_preview(&without_cover).unwrap();
        assert_eq!(preview.layout.preview_svg(), plain.layout.preview_svg());
        assert_eq!(preview.layout.pdf().unwrap(), plain.layout.pdf().unwrap());
    }

    #[test]
    fn library_empty_and_sections() {
        let empty = json::<EvidenceLibraryPage>(
            r#"{"catalog_revision":null,"counts":{},"entries":[],"health_records_generation":0,"next_cursor":null,"revision":1,"total_count":0}"#,
        );
        assert_eq!(library_root(&empty), LibraryRoot::Empty);
        let loaded = json::<EvidenceLibraryPage>(
            r#"{"catalog_revision":null,"counts":{"medication":1,"lab":2},"entries":[],"health_records_generation":0,"next_cursor":null,"revision":1,"total_count":3}"#,
        );
        assert_eq!(
            library_root(&loaded),
            LibraryRoot::Sections(vec![
                LibrarySection {
                    name: "Medications",
                    measured: false,
                    records: vec![],
                },
                LibrarySection {
                    name: "Recent labs",
                    measured: true,
                    records: vec![],
                },
            ])
        );
        let listed = json::<EvidenceLibraryPage>(
            r#"{"catalog_revision":null,"counts":{"medication":1},"entries":[{"category":"medication","coding_identities":[],"condition_status_conflict":false,"date":"2026-10-01T15:00:00Z","entity":"health_evidence","group_kind":null,"id":"med-1","member_count":1,"preview":null,"record_id":"00000000-0000-0000-0000-000000000021","source_label":"Pharmacy","status_conflict":false,"summary":"10 mg","title":"Lisinopril"}],"health_records_generation":0,"next_cursor":null,"revision":1,"total_count":1}"#,
        );
        assert_eq!(
            library_root(&listed),
            LibraryRoot::Sections(vec![LibrarySection {
                name: "Medications",
                measured: false,
                records: vec![LibraryRecord {
                    id: "med-1".into(),
                    kind: LibraryKind::HealthEvidence,
                    title: "Lisinopril".into(),
                    summary: Some("10 mg".into()),
                    when: Some("October 1, 2026".into()),
                    source: Some("Pharmacy".into()),
                }],
            }])
        );
        assert_eq!(
            LIBRARY_UNAVAILABLE,
            "Your health information isn’t available right now. Try again in a moment."
        );
    }

    #[test]
    fn notification_controls_name_their_permission() {
        assert!(ALLOW_NOTIFICATIONS.contains("notifications"));
        assert_eq!(OPEN_NOTIFICATION_SETTINGS, "Open iPhone Settings");
    }

    #[test]
    fn android_debug_api_reaches_the_emulator_host() {
        let host = if cfg!(target_os = "android") {
            "10.0.2.2"
        } else {
            "127.0.0.1"
        };
        assert!(api_base_url().contains(host) || option_env!("DUET_API_URL").is_some());
    }
}
