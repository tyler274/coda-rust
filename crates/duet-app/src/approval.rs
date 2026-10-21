//! Explicit proposal review does not save; approval retains generation, proposal and medication fences.
use duet_wire::models::{DoctorCollection, DoctorCollectionItem, DoctorPageSave};
/// Medication identity follows the server's exact source group, never title heuristics.
pub fn is_medication(item: &DoctorCollectionItem) -> bool {
    matches!(
        item.page_entry
            .as_ref()
            .and_then(|e| e.as_ref())
            .map(|e| e.group),
        Some(
            duet_wire::models::doctor_page_entry::Group::ConfirmedMedications
                | duet_wire::models::doctor_page_entry::Group::RecordedMedications
        )
    )
}
/// Prepare the exact reviewed proposal without touching the approved collection.
/// A local draft or stale generation must be resolved before this action is available.
pub fn proposal_save(
    collection: &DoctorCollection,
    confirmed: &[uuid::Uuid],
    has_local_draft: bool,
) -> Result<DoctorPageSave, &'static str> {
    if has_local_draft {
        return Err("Keep or discard your local draft before approving a proposal");
    }
    let proposal = collection
        .proposal
        .as_ref()
        .and_then(|p| p.as_deref())
        .ok_or("No proposal to review")?;
    if proposal.base_generation != collection.note_generation {
        return Err("This proposal is stale; your newer words are unchanged");
    }
    let ids: Vec<_> = proposal.items.iter().map(|item| item.id).collect();
    if confirmed.iter().any(|id| {
        !proposal
            .items
            .iter()
            .any(|item| item.id == *id && is_medication(item))
    }) {
        return Err("Only reviewed medications may be confirmed");
    }
    let mut save = DoctorPageSave::new(proposal.base_generation, ids, *proposal.page.clone());
    save.note = Some(proposal.note.clone());
    save.proposal_id = Some(Some(proposal.id));
    save.confirmed_medication_ids = Some(confirmed.to_vec());
    Ok(save)
}
