//! Literal saved-page projection. Never substitutes summary text for an exact page entry.
use duet_wire::models::{DoctorCollection, doctor_page_entry::Group};

/// Full saved snapshot plus explicit incomplete-source condition.
pub struct SavedPreview {
    /// Shared literal renderer output, including overflow recovery text.
    pub layout: duet_page::PageLayout,
    /// Missing exact projection or withdrawn/stale source blocks export.
    pub incomplete: bool,
}
/// Project every source-owned field exactly; cover note intentionally has no renderer field.
pub fn saved_preview(collection: &DoctorCollection) -> Option<SavedPreview> {
    let content = collection.page.as_ref()?;
    let mut incomplete = false;
    let mut items: Vec<_> = collection.items.iter().flatten().collect();
    items.sort_by_key(|item| item.position);
    let entries = items.into_iter().map(|item| {
        if item.review_required == Some(true) { incomplete = true; }
        match item.page_entry.as_ref().and_then(|entry| entry.as_deref()) {
            Some(entry) => duet_page::PageEntry {
                group: match entry.group {
                    Group::Records => duet_page::EvidenceGroup::Records,
                    Group::Devices => duet_page::EvidenceGroup::Devices,
                    Group::ConfirmedMedications => duet_page::EvidenceGroup::ConfirmedMedications,
                    Group::RecordedMedications => duet_page::EvidenceGroup::RecordedMedications,
                },
                title: entry.title.clone(), lines: entry.lines.clone(),
            },
            None => {
                incomplete = true;
                duet_page::PageEntry { group: duet_page::EvidenceGroup::Records, title: item.title.clone(), lines: vec!["Exact saved source projection is unavailable. Reconnect and review this item before export.".into()] }
            }
        }
    }).collect();
    let page = duet_page::ApprovedPage {
        name: content.name.clone().flatten(),
        date_of_birth: content
            .date_of_birth
            .flatten()
            .map(|d| d.format("%Y-%m-%d").to_string()),
        prepared_date: collection
            .page_updated_at
            .flatten()
            .map(|d| d.format("%Y-%m-%d").to_string()),
        concern: collection.note.clone().flatten(),
        questions: content.questions.clone().unwrap_or_default(),
        entries,
    };
    Some(SavedPreview {
        layout: duet_page::layout_approved(&page),
        incomplete,
    })
}
