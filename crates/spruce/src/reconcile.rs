//! Compare a full conversation read with the versions already stored.
//!
//! Polling `lastMessageAt` is not treated as proof of an edit or a deletion.
//! Spruce does not document whether either one moves that timestamp. A product
//! that has read a conversation in full can ask this helper what changed.
//!
//! An id that was stored and is absent from the read is [`Diff::missing`].
//! That is not a deletion. [`Diff::deleted`] holds only items Spruce still
//! returned with its deleted flag set.

use std::collections::HashMap;

use crate::payload::Item;

/// What a full read changed relative to stored versions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diff {
    /// Items in the read that were not stored.
    pub new: Vec<Item>,
    /// Stored items whose [`Item::version`](crate::payload::Item::version) changed.
    pub edited: Vec<Item>,
    /// Stored ids the read did not contain. Not claimed as deletions.
    pub missing: Vec<String>,
    /// Items the read still contains and Spruce marked deleted.
    pub deleted: Vec<Item>,
}

/// `known` is `(item id, version)` in the product's stored order.
///
/// The first copy of an id in `read` wins. A deleted item is reported in
/// `deleted` and not also in `new` or `edited`.
pub fn reconcile(known: &[(&str, &str)], read: &[Item]) -> Diff {
    let stored: HashMap<&str, &str> = known.iter().copied().collect();
    let mut seen = HashMap::<&str, ()>::new();
    let mut diff = Diff {
        new: Vec::new(),
        edited: Vec::new(),
        missing: Vec::new(),
        deleted: Vec::new(),
    };
    for item in read {
        if seen.insert(item.id(), ()).is_some() {
            continue;
        }
        if item.deleted {
            diff.deleted.push(item.clone());
            continue;
        }
        match stored.get(item.id()) {
            None => diff.new.push(item.clone()),
            Some(version) if *version != item.version() => diff.edited.push(item.clone()),
            Some(_) => {}
        }
    }
    for (id, _) in known {
        if !seen.contains_key(id) {
            diff.missing.push((*id).to_owned());
        }
    }
    diff
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payload::{Envelope, Item};

    fn item(id: &str, text: &str, modified: &str, deleted: bool) -> Item {
        Item {
            envelope: Envelope {
                id: id.to_owned(),
                conversation_id: None,
                direction: Some("inbound".into()),
                is_internal_note: Some(false),
                author_name: None,
                created_at: Some(modified.into()),
                modified_at: Some(modified.into()),
                app_url: None,
            },
            text: text.to_owned(),
            attachments: Vec::new(),
            buttons: Vec::new(),
            event: None,
            deleted,
            transcription: None,
        }
    }

    #[test]
    fn new_edited_and_unchanged_items_are_separated() {
        let kept = item("ti_kept", "same", "2026-10-27T18:00:00Z", false);
        let edited = item("ti_edit", "after", "2026-10-27T19:00:00Z", false);
        let before = item("ti_edit", "before", "2026-10-27T18:00:00Z", false);
        let added = item("ti_new", "fresh", "2026-10-27T19:00:00Z", false);
        let diff = reconcile(
            &[
                ("ti_kept", &kept.version()),
                ("ti_edit", &before.version()),
                ("ti_gone", "old"),
            ],
            &[kept, edited.clone(), added.clone()],
        );
        assert_eq!(diff.new, vec![added]);
        assert_eq!(diff.edited, vec![edited]);
        assert_eq!(diff.missing, vec!["ti_gone".to_owned()]);
        assert!(diff.deleted.is_empty());
    }

    #[test]
    fn a_flagged_item_is_deleted_and_an_absent_item_is_only_missing() {
        let gone = item("ti_flag", "x", "2026-10-27T18:00:00Z", true);
        let diff = reconcile(
            &[("ti_flag", "previous"), ("ti_absent", "previous")],
            std::slice::from_ref(&gone),
        );
        assert!(diff.new.is_empty());
        assert!(diff.edited.is_empty());
        assert_eq!(diff.deleted, vec![gone]);
        assert_eq!(diff.missing, vec!["ti_absent".to_owned()]);
    }

    #[test]
    fn a_repeated_id_is_read_once() {
        let first = item("ti_1", "first", "2026-10-27T18:00:00Z", false);
        let second = item("ti_1", "second", "2026-10-27T19:00:00Z", false);
        let diff = reconcile(&[], &[first.clone(), second]);
        assert_eq!(diff.new, vec![first]);
    }
}
