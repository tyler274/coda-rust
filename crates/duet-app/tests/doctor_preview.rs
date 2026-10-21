use duet_app::{proposal_save, saved_preview};
use duet_wire::models::*;
#[test]
fn preview_preserves_full_literal_fields_and_excludes_cover_note() {
    let mut collection = DoctorCollection::new(4);
    let content = DoctorPageContent {
        name: Some(Some(" Zoë ".into())),
        questions: Some(vec!["  Why  now?  ".into()]),
        cover_note: Some(Some("NEVER IN PDF".into())),
        ..Default::default()
    };
    collection.page = Some(Box::new(content));
    collection.note = Some(Some("My exact concern ≤ 5".into()));
    let item = DoctorCollectionItem {
        page_entry: Some(Some(Box::new(DoctorPageEntry::new(
            doctor_page_entry::Group::Devices,
            vec![
                "Source: device".into(),
                "Value: 5.00 mg".into(),
                "Unknown field: literal".into(),
            ],
            "Device title".into(),
        )))),
        ..Default::default()
    };
    collection.items = Some(vec![item]);
    let preview = saved_preview(&collection).unwrap();
    assert!(!preview.incomplete);
    let text = preview.layout.text();
    for value in [
        "Zoë",
        "My exact concern ≤ 5",
        "Device title",
        "Value: 5.00 mg",
        "Unknown field: literal",
        "From my devices",
    ] {
        assert!(text.contains(value), "missing {value}");
    }
    assert!(!text.contains("NEVER IN PDF"));
    assert!(preview.layout.preview_svg().contains("<svg"));
}
#[test]
fn missing_exact_projection_and_overflow_remain_visible_but_block_export() {
    let mut collection = DoctorCollection::new(4);
    collection.page = Some(Box::new(DoctorPageContent::default()));
    collection.items = Some(vec![DoctorCollectionItem::default()]);
    assert!(saved_preview(&collection).unwrap().incomplete);
    collection.note = Some(Some("long concern ".repeat(2000)));
    let preview = saved_preview(&collection).unwrap();
    assert!(!preview.layout.fits());
    assert!(preview.layout.text().contains("long concern"));
}
#[test]
fn proposal_review_is_explicit_and_fenced_against_local_or_server_edits() {
    let mut collection = DoctorCollection::new(4);
    let proposal = DoctorPageProposal {
        base_generation: 4,
        id: uuid::Uuid::new_v4(),
        ..Default::default()
    };
    let id = proposal.id;
    collection.proposal = Some(Some(Box::new(proposal)));
    let save = proposal_save(&collection, &[], false).unwrap();
    assert_eq!(save.proposal_id, Some(Some(id)));
    assert_eq!(save.expected_generation, 4);
    assert!(proposal_save(&collection, &[], true).is_err());
    assert!(proposal_save(&collection, &[uuid::Uuid::new_v4()], false).is_err());
    collection.note_generation = 5;
    assert!(proposal_save(&collection, &[], false).is_err());
}
