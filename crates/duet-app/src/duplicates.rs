//! Likely-duplicate detection and non-destructive review sheet (`D2.5`).
//!
//! Only exact duplicates are collapsed on the server. When two different sources carry
//! a likely duplicate result, Duet presents `"looks like the same result as ..."` and lets
//! the person choose one or both without deleting external originals (`D2.6`).
use std::collections::HashSet;

use dioxus::prelude::*;
use duet_wire::models::EvidenceLibraryEntry;
use ui::{ActionRow, Button, ButtonVariant, Card};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LikelyDuplicate {
    pub pair_key: String,
    pub first_id: String,
    pub first_title: String,
    pub first_source: String,
    pub second_id: String,
    pub second_title: String,
    pub second_source: String,
    pub prompt: String,
}

/// Identifies likely duplicates across distinct sources in a library page.
pub fn find_likely_duplicates(entries: &[EvidenceLibraryEntry]) -> Vec<LikelyDuplicate> {
    let mut pairs = Vec::new();
    for (i, a) in entries.iter().enumerate() {
        let Some(source_a) = source_name(a) else {
            continue;
        };
        for b in entries.iter().skip(i + 1) {
            if a.id == b.id {
                continue;
            }
            let Some(source_b) = source_name(b) else {
                continue;
            };
            if source_a.eq_ignore_ascii_case(source_b) {
                continue;
            }
            if !entries_look_alike(a, b) {
                continue;
            }
            let (first_id, second_id) = if a.id <= b.id {
                (a.id.clone(), b.id.clone())
            } else {
                (b.id.clone(), a.id.clone())
            };
            let pair_key = format!("{first_id}::{second_id}");
            let prompt = format!(
                "{} ({source_b}) looks like the same result as {} ({source_a}).",
                b.title.trim(),
                a.title.trim()
            );
            pairs.push(LikelyDuplicate {
                pair_key,
                first_id: a.id.clone(),
                first_title: a.title.clone(),
                first_source: source_a.to_owned(),
                second_id: b.id.clone(),
                second_title: b.title.clone(),
                second_source: source_b.to_owned(),
                prompt,
            });
        }
    }
    pairs
}

fn source_name(entry: &EvidenceLibraryEntry) -> Option<&str> {
    entry
        .source_label
        .as_ref()
        .and_then(|s| s.as_deref())
        .map(str::trim)
        .filter(|s| !s.is_empty())
}

fn entries_look_alike(a: &EvidenceLibraryEntry, b: &EvidenceLibraryEntry) -> bool {
    let shared_code = a.coding_identities.iter().any(|code| {
        let sys = code.system.trim();
        let val = code.code.trim();
        !sys.is_empty()
            && !val.is_empty()
            && b.coding_identities.iter().any(|other| {
                other.system.trim().eq_ignore_ascii_case(sys)
                    && other.code.trim().eq_ignore_ascii_case(val)
            })
    });
    let same_title =
        !a.title.trim().is_empty() && a.title.trim().eq_ignore_ascii_case(b.title.trim());
    if !shared_code && !same_title {
        return false;
    }
    let same_day = match (entry_day(a), entry_day(b)) {
        (Some(da), Some(db)) => da == db,
        _ => true,
    };
    let same_summary = match (
        a.summary.as_ref().and_then(|s| s.as_deref()).map(str::trim),
        b.summary.as_ref().and_then(|s| s.as_deref()).map(str::trim),
    ) {
        (Some(sa), Some(sb)) if !sa.is_empty() && !sb.is_empty() => sa.eq_ignore_ascii_case(sb),
        _ => true,
    };
    same_day && same_summary
}

fn entry_day(entry: &EvidenceLibraryEntry) -> Option<chrono::NaiveDate> {
    entry
        .day
        .flatten()
        .or_else(|| entry.date.map(|dt| dt.date_naive()))
}

#[component]
pub fn DuplicateReviewSheet(
    duplicates: Vec<LikelyDuplicate>,
    mut hidden_ids: Signal<HashSet<String>>,
    mut reviewed_pairs: Signal<HashSet<String>>,
    on_close: EventHandler<()>,
) -> Element {
    let pending: Vec<LikelyDuplicate> = duplicates
        .into_iter()
        .filter(|pair| !reviewed_pairs().contains(&pair.pair_key))
        .collect();
    rsx! {
        div { role: "dialog", aria_label: "Possible duplicates",
            h2 { class: "status-title", "Possible duplicates" }
            p { class: "record-meta", "Originals from every source remain untouched. Choose what to display in You." }
            if pending.is_empty() {
                p { class: "empty", "No possible duplicates left to review." }
            }
            for pair in pending {
                {
                    let pair_key_a = pair.pair_key.clone();
                    let pair_key_b = pair.pair_key.clone();
                    let pair_key_both = pair.pair_key.clone();
                    let hide_second = pair.second_id.clone();
                    let hide_first = pair.first_id.clone();
                    let first_source = pair.first_source.clone();
                    let second_source = pair.second_source.clone();
                    let first_label = format!("Keep {first_source}");
                    let second_label = format!("Keep {second_source}");
                    rsx! {
                        Card {
                            p { "{pair.prompt}" }
                            ActionRow {
                                Button {
                                    compact: true,
                                    aria_label: "{first_label}",
                                    onclick: move |_| {
                                        let mut hidden = hidden_ids();
                                        hidden.insert(hide_second.clone());
                                        hidden_ids.set(hidden);
                                        let mut reviewed = reviewed_pairs();
                                        reviewed.insert(pair_key_a.clone());
                                        reviewed_pairs.set(reviewed);
                                    },
                                    "{first_label}"
                                }
                                Button {
                                    compact: true,
                                    aria_label: "{second_label}",
                                    onclick: move |_| {
                                        let mut hidden = hidden_ids();
                                        hidden.insert(hide_first.clone());
                                        hidden_ids.set(hidden);
                                        let mut reviewed = reviewed_pairs();
                                        reviewed.insert(pair_key_b.clone());
                                        reviewed_pairs.set(reviewed);
                                    },
                                    "{second_label}"
                                }
                                Button {
                                    compact: true,
                                    variant: ButtonVariant::Primary,
                                    aria_label: "Keep both",
                                    onclick: move |_| {
                                        let mut reviewed = reviewed_pairs();
                                        reviewed.insert(pair_key_both.clone());
                                        reviewed_pairs.set(reviewed);
                                    },
                                    "Keep both"
                                }
                            }
                        }
                    }
                }
            }
            Button {
                variant: ButtonVariant::Quiet,
                aria_label: "Done reviewing duplicates",
                onclick: move |_| on_close.call(()),
                "Done"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_likely_duplicates_across_distinct_sources() {
        let page: duet_wire::models::EvidenceLibraryPage = serde_json::from_str(
            r#"{"catalog_revision":null,"counts":{"lab":2},"entries":[
                {"category":"lab","coding_identities":[],"condition_status_conflict":false,"date":"2026-09-12T15:00:00Z","entity":"health_evidence","group_kind":null,"id":"lab-quest","member_count":1,"preview":null,"record_id":"00000000-0000-0000-0000-000000000022","source_label":"Quest","status_conflict":false,"summary":"98 mg/dL","title":"LDL cholesterol"},
                {"category":"lab","coding_identities":[],"condition_status_conflict":false,"date":"2026-09-12T18:00:00Z","entity":"health_evidence","group_kind":null,"id":"lab-epic","member_count":1,"preview":null,"record_id":"00000000-0000-0000-0000-000000000023","source_label":"Epic","status_conflict":false,"summary":"98 mg/dL","title":"LDL cholesterol"}
            ],"health_records_generation":0,"next_cursor":null,"revision":1,"total_count":2}"#,
        )
        .unwrap();
        let pairs = find_likely_duplicates(&page.entries);
        assert_eq!(pairs.len(), 1);
        assert!(pairs[0].prompt.contains("looks like the same result as"));
    }
}
