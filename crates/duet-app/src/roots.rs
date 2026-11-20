//! Root states for the two Duet tabs. Sentences are the ones the iPhone app shows.

use duet_client::EvidenceLibraryPage;
#[cfg(test)]
use duet_wire::models::{DoctorCollection, DoctorCollectionItem};

pub const CONNECTING: &str = "Connecting…";
pub const LIBRARY_EMPTY: &str = "No records or measurements yet.";
pub const LIBRARY_UNAVAILABLE: &str =
    "Your health information isn’t available right now. Try again in a moment.";
pub const TRY_AGAIN: &str = "Try again";
pub const ALLOW_NOTIFICATIONS: &str = "Allow notifications";
pub const OPEN_NOTIFICATION_SETTINGS: &str = "Open iPhone Settings";

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoctorPage {
    /// The collection loaded and has nothing to list.
    Empty,
    /// Items, note, and questions, in that order.
    Lines(Vec<DoctorLine>),
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorLine {
    pub title: String,
    pub detail: Option<String>,
}

/// The lines a loaded For my doctor collection already contains.
#[cfg(test)]
pub fn doctor_page(collection: &DoctorCollection) -> DoctorPage {
    let mut entries = item_entries(collection.items.as_deref().unwrap_or(&[]));
    if let Some(note) = supplied(&collection.note) {
        entries.push(DoctorLine {
            title: note,
            detail: None,
        });
    }
    if let Some(page) = collection.page.as_deref()
        && let Some(questions) = &page.questions
    {
        for question in questions {
            if let Some(title) = nonempty(question) {
                entries.push(DoctorLine {
                    title,
                    detail: None,
                });
            }
        }
    }
    if entries.is_empty() {
        DoctorPage::Empty
    } else {
        DoctorPage::Lines(entries)
    }
}

#[cfg(test)]
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

#[cfg(test)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordEditability {
    PatientEntered,
    ExternalSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryRecord {
    pub id: String,
    pub kind: LibraryKind,
    pub category: String,
    pub title: String,
    pub summary: Option<String>,
    pub when: Option<String>,
    pub source: Option<String>,
    pub status_line: Option<String>,
    pub editability: RecordEditability,
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
    (&["vital", "measurement", "sleep"], "Measurements", true),
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
    let current_conditions: Vec<&duet_wire::models::EvidenceLibraryEntry> = page
        .entries
        .iter()
        .filter(|entry| entry.category == "currentCondition")
        .collect();
    let visible_entries: Vec<&duet_wire::models::EvidenceLibraryEntry> = page
        .entries
        .iter()
        .filter(|entry| !current_condition_covers(entry, &current_conditions))
        .collect();
    let mut present = std::collections::HashSet::new();
    for (key, count) in &page.counts {
        if *count > 0 && key != "pastCondition" {
            present.insert(key.clone());
        }
    }
    for entry in &visible_entries {
        present.insert(entry.category.clone());
    }
    if present.is_empty() && visible_entries.is_empty() && page.total_count == 0 {
        return LibraryRoot::Empty;
    }
    let sections: Vec<LibrarySection> = SECTIONS
        .iter()
        .filter(|(keys, _, _)| keys.iter().any(|key| present.contains(*key)))
        .map(|(keys, name, measured)| LibrarySection {
            name,
            measured: *measured,
            records: visible_entries
                .iter()
                .filter(|entry| keys.contains(&entry.category.as_str()))
                .map(|entry| library_record(entry))
                .collect(),
        })
        .collect();
    if sections.is_empty() {
        LibraryRoot::Empty
    } else {
        LibraryRoot::Sections(sections)
    }
}

pub fn filter_library_root(
    root: &LibraryRoot,
    query: &str,
    hidden_ids: &std::collections::HashSet<String>,
) -> LibraryRoot {
    let LibraryRoot::Sections(sections) = root else {
        return LibraryRoot::Empty;
    };
    let needle = query.trim().to_ascii_lowercase();
    let filtered: Vec<LibrarySection> = sections
        .iter()
        .filter_map(|section| {
            let section_matches =
                !needle.is_empty() && section.name.to_ascii_lowercase().contains(&needle);
            let records: Vec<LibraryRecord> = section
                .records
                .iter()
                .filter(|record| !hidden_ids.contains(&record.id))
                .filter(|record| {
                    needle.is_empty() || section_matches || record_matches_query(record, &needle)
                })
                .cloned()
                .collect();
            if records.is_empty() && !needle.is_empty() {
                None
            } else if records.is_empty() && hidden_ids.is_empty() {
                // Keep count-only summary sections when nothing is searched or hidden.
                Some(section.clone())
            } else if records.is_empty() {
                None
            } else {
                Some(LibrarySection {
                    name: section.name,
                    measured: section.measured,
                    records,
                })
            }
        })
        .collect();
    if filtered.is_empty() {
        LibraryRoot::Empty
    } else {
        LibraryRoot::Sections(filtered)
    }
}

fn record_matches_query(record: &LibraryRecord, needle: &str) -> bool {
    record.title.to_ascii_lowercase().contains(needle)
        || record
            .summary
            .as_deref()
            .is_some_and(|s| s.to_ascii_lowercase().contains(needle))
        || record
            .when
            .as_deref()
            .is_some_and(|s| s.to_ascii_lowercase().contains(needle))
        || record
            .source
            .as_deref()
            .is_some_and(|s| s.to_ascii_lowercase().contains(needle))
        || record
            .status_line
            .as_deref()
            .is_some_and(|s| s.to_ascii_lowercase().contains(needle))
        || record.category.to_ascii_lowercase().contains(needle)
}

fn current_condition_covers(
    entry: &duet_wire::models::EvidenceLibraryEntry,
    currents: &[&duet_wire::models::EvidenceLibraryEntry],
) -> bool {
    if entry.category != "pastCondition" {
        return false;
    }
    let title = entry.title.trim();
    currents.iter().any(|current| {
        let shared_code = entry.coding_identities.iter().any(|code| {
            let sys = code.system.trim();
            let val = code.code.trim();
            !sys.is_empty()
                && !val.is_empty()
                && current.coding_identities.iter().any(|other| {
                    other.system.trim().eq_ignore_ascii_case(sys)
                        && other.code.trim().eq_ignore_ascii_case(val)
                })
        });
        shared_code || (!title.is_empty() && current.title.trim().eq_ignore_ascii_case(title))
    })
}

fn record_editability(entry: &duet_wire::models::EvidenceLibraryEntry) -> RecordEditability {
    let source_label = supplied(&entry.source_label);
    let preview_kind = entry
        .preview
        .as_ref()
        .and_then(|map| map.get("source"))
        .and_then(|source| source.get("kind"))
        .and_then(serde_json::Value::as_str);
    let patient_label = source_label.as_deref() == Some("Added in Duet");
    let patient_kind = preview_kind.is_none_or(|kind| kind == "manual");
    if patient_label && patient_kind {
        RecordEditability::PatientEntered
    } else {
        RecordEditability::ExternalSource
    }
}

fn extract_source_status(
    preview: Option<&std::collections::HashMap<String, serde_json::Value>>,
) -> Option<String> {
    let preview = preview?;
    let raw = preview
        .get("status")
        .and_then(serde_json::Value::as_str)
        .or_else(|| {
            preview
                .get("clinical_record")
                .and_then(|cr| {
                    cr.get("record_status")
                        .or_else(|| cr.get("clinical_status"))
                })
                .and_then(serde_json::Value::as_str)
        })?;
    crate::records::source_status_line(raw)
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
        category: entry.category.clone(),
        title: entry.title.clone(),
        summary: supplied(&entry.summary),
        when,
        source: supplied(&entry.source_label),
        status_line: extract_source_status(entry.preview.as_ref()),
        editability: record_editability(entry),
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
    fn a_saved_doctor_page_excludes_cover_note_and_pdf_copy() {
        let collection = json::<DoctorCollection>(
            r#"{"items":[],"note_generation":1,"page":{"name":"Ada Lovelace","date_of_birth":"1815-12-10","questions":["What changed?"],"cover_note":"Please review."}}"#,
        );
        let DoctorPage::Lines(entries) = doctor_page(&collection) else {
            panic!("expected the saved-page summary");
        };
        assert_eq!(
            entries,
            vec![DoctorLine {
                title: "What changed?".into(),
                detail: None,
            }]
        );
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
                    category: "medication".into(),
                    title: "Lisinopril".into(),
                    summary: Some("10 mg".into()),
                    when: Some("October 1, 2026".into()),
                    source: Some("Pharmacy".into()),
                    status_line: None,
                    editability: RecordEditability::ExternalSource,
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

    #[test]
    fn past_condition_is_hidden_when_covered_by_current_condition() {
        let page = json::<EvidenceLibraryPage>(
            r#"{"catalog_revision":null,"counts":{"currentCondition":1,"pastCondition":1},"entries":[{"category":"currentCondition","coding_identities":[{"code":"44054006","system":"http://snomed.info/sct","version":null}],"condition_status_conflict":false,"date":"2026-10-01T00:00:00Z","entity":"health_evidence","group_kind":null,"id":"cond-current","member_count":1,"preview":null,"record_id":"00000000-0000-0000-0000-000000000031","source_label":"Epic","status_conflict":false,"summary":null,"title":"Type 2 diabetes"},{"category":"pastCondition","coding_identities":[{"code":"44054006","system":"http://snomed.info/sct","version":null}],"condition_status_conflict":false,"date":"2024-01-15T00:00:00Z","entity":"health_evidence","group_kind":null,"id":"cond-past","member_count":1,"preview":null,"record_id":"00000000-0000-0000-0000-000000000032","source_label":"Regional Clinic","status_conflict":false,"summary":null,"title":"Type 2 diabetes mellitus"}],"health_records_generation":0,"next_cursor":null,"revision":1,"total_count":2}"#,
        );
        let LibraryRoot::Sections(sections) = library_root(&page) else {
            panic!("expected Conditions section");
        };
        assert_eq!(sections.len(), 1);
        assert_eq!(
            sections[0].records.len(),
            1,
            "pastCondition must be suppressed when covered by currentCondition"
        );
        assert_eq!(sections[0].records[0].id, "cond-current");
    }

    #[test]
    fn external_and_patient_editability_distinguish_sources() {
        let page = json::<EvidenceLibraryPage>(
            r#"{"catalog_revision":null,"counts":{"medication":2},"entries":[
                {"category":"medication","coding_identities":[],"condition_status_conflict":false,"date":null,"entity":"health_evidence","group_kind":null,"id":"patient-med","member_count":1,"preview":{"source":{"kind":"manual"}},"record_id":"00000000-0000-0000-0000-000000000041","source_label":"Added in Duet","status_conflict":false,"summary":"20 mg","title":"Atorvastatin"},
                {"category":"medication","coding_identities":[],"condition_status_conflict":false,"date":null,"entity":"health_evidence","group_kind":null,"id":"spoofed-med","member_count":1,"preview":{"source":{"kind":"epic"}},"record_id":"00000000-0000-0000-0000-000000000042","source_label":"Added in Duet","status_conflict":false,"summary":"10 mg","title":"Lisinopril"}
            ],"health_records_generation":0,"next_cursor":null,"revision":1,"total_count":2}"#,
        );
        let LibraryRoot::Sections(sections) = library_root(&page) else {
            panic!("expected Medications section");
        };
        assert_eq!(
            sections[0].records[0].editability,
            RecordEditability::PatientEntered
        );
        assert_eq!(
            sections[0].records[1].editability,
            RecordEditability::ExternalSource
        );
    }

    #[test]
    fn filter_library_root_searches_across_fields_and_hides_resolved_duplicates() {
        let page = json::<EvidenceLibraryPage>(
            r#"{"catalog_revision":null,"counts":{"lab":1,"medication":1},"entries":[
                {"category":"medication","coding_identities":[],"condition_status_conflict":false,"date":null,"entity":"health_evidence","group_kind":null,"id":"med-1","member_count":1,"preview":null,"record_id":"00000000-0000-0000-0000-000000000021","source_label":"Pharmacy","status_conflict":false,"summary":"10 mg","title":"Lisinopril"},
                {"category":"lab","coding_identities":[],"condition_status_conflict":false,"date":null,"entity":"health_evidence","group_kind":null,"id":"lab-1","member_count":1,"preview":{"status":"cancelled"},"record_id":"00000000-0000-0000-0000-000000000022","source_label":"Quest","status_conflict":false,"summary":"98 mg/dL","title":"LDL cholesterol"}
            ],"health_records_generation":0,"next_cursor":null,"revision":1,"total_count":2}"#,
        );
        let root = library_root(&page);
        let empty_hidden = std::collections::HashSet::new();
        let by_source = filter_library_root(&root, "quest", &empty_hidden);
        let LibraryRoot::Sections(sections) = by_source else {
            panic!("expected matching lab section");
        };
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].records[0].id, "lab-1");
        assert!(
            sections[0].records[0]
                .status_line
                .as_deref()
                .is_some_and(|line| line.contains("did not happen"))
        );

        let mut hidden = std::collections::HashSet::new();
        hidden.insert("lab-1".to_owned());
        assert_eq!(
            filter_library_root(&root, "quest", &hidden),
            LibraryRoot::Empty
        );
    }
}
