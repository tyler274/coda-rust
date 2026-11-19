//! Non-modal cited source reading panel, hover preview conversion, record
//! status classification, passage range matching, and Answer Sources list builder.

use std::collections::BTreeMap;

use dioxus::prelude::EventHandler;
use physician_wire::{CitationOrigin, CitationTargetKind, CitedFieldWire, CitedSourceWire};
use ui::{
    AnswerSources, CitedAt, ExpandSource, FieldGroup, FieldValue, IndexedSource, PartRange, Places,
    PreviewContent, RawRecord, RecordAbout, RecordBody, RecordField, RecordStatus, SourceLink,
    SourcePane, SourceRecord, SourceState, SourceSummary, TextPart,
};

use crate::shell::{SourceChip, SynopsisPiece, source_provenance_line};

const PASSAGE_NOT_FOUND_NOTICE: &str =
    "Cited passage could not be matched in this version of the record.";

/// Authoritative state of the non-modal cited-source reading panel beside the page.
#[derive(Clone, PartialEq)]
pub(crate) enum SourcePanelLoad {
    /// No source is currently open.
    Idle,
    /// Fetching the authoritative record on demand; monotonic `request_id` drops stale responses.
    Loading {
        request_id: u64,
        chip: Box<SourceChip>,
    },
    /// Authoritative record loaded for the clicked citation claim.
    Loaded {
        chip: Box<SourceChip>,
        source: Box<CitedSourceWire>,
    },
    /// Access or record visibility is no longer available; content is cleared cleanly.
    Unavailable {
        eyebrow: String,
        title: String,
        reason: String,
    },
    /// Transient fetch failure; retry re-requests the same target.
    Failed {
        request_id: u64,
        chip: Box<SourceChip>,
        message: String,
    },
}

impl SourcePanelLoad {
    /// Converts a snapshot/embedded [`SourceChip`] into an immediately loaded panel state
    /// when no separate chat source route applies (e.g., Briefings, Reviews, Today Updates, Demo).
    pub(crate) fn from_chip(chip: SourceChip) -> Self {
        let has_content = !chip.preview.trim().is_empty()
            || !chip.readable_parts.is_empty()
            || !chip.fields.is_empty()
            || chip
                .raw_content
                .as_deref()
                .is_some_and(|raw| !raw.trim().is_empty());
        if !chip.open || !has_content {
            let reason = if !chip.open && !chip.preview.trim().is_empty() {
                chip.preview.clone()
            } else {
                "Source content is unavailable here.".to_owned()
            };
            return Self::Unavailable {
                eyebrow: format!("Source {}", chip.index),
                title: chip.label,
                reason,
            };
        }
        let source = CitedSourceWire {
            source_key: chip.source_key.clone().unwrap_or_default(),
            number: chip.index,
            source_label: chip
                .open_label
                .as_deref()
                .and_then(|label| label.strip_prefix("Open in "))
                .unwrap_or("Patient chart")
                .to_owned(),
            title: chip.label.clone(),
            record_type: chip.record_kind.clone(),
            recorded_at: (!chip.as_of.trim().is_empty()).then(|| chip.as_of.clone()),
            record_status: chip.record_status.clone(),
            quote: chip.quote.clone(),
            body: chip.preview.clone(),
            readable_parts: chip.readable_parts.clone(),
            fields: chip.fields.clone(),
            raw_content: chip.raw_content.clone(),
            external_url: chip.open_uri.clone(),
            origin: chip.origin,
        };
        Self::Loaded {
            chip: Box::new(chip),
            source: Box::new(source),
        }
    }

    /// Transitions an open or loading panel to `Unavailable` with `reason` when access changes.
    pub(crate) fn into_unavailable(self, reason: impl Into<String>) -> Self {
        match self {
            Self::Idle => Self::Idle,
            Self::Loading { chip, .. } | Self::Loaded { chip, .. } | Self::Failed { chip, .. } => {
                Self::Unavailable {
                    eyebrow: format!("Source {}", chip.index),
                    title: chip.label,
                    reason: reason.into(),
                }
            }
            Self::Unavailable {
                eyebrow,
                title,
                reason: _,
            } => Self::Unavailable {
                eyebrow,
                title,
                reason: reason.into(),
            },
        }
    }

    pub(crate) fn active_chip(&self) -> Option<&SourceChip> {
        match self {
            Self::Loading { chip, .. } | Self::Loaded { chip, .. } | Self::Failed { chip, .. } => {
                Some(chip.as_ref())
            }
            Self::Idle | Self::Unavailable { .. } => None,
        }
    }
}

/// Classifies a source record's own status string (`"preliminary"`, `"inactive"`,
/// `"refuted"`, `"entered-in-error"` vs settled statuses such as `"final"`).
pub(crate) fn classify_record_status(status: Option<&str>) -> Option<RecordStatus> {
    let trimmed = status?.trim();
    if trimmed.is_empty() {
        return None;
    }
    let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
    if matches!(
        normalized.as_str(),
        "inactive" | "refuted" | "entered-in-error" | "entered in error" | "preliminary"
    ) {
        Some(RecordStatus::Flagged(trimmed.to_owned()))
    } else {
        Some(RecordStatus::Settled(trimmed.to_owned()))
    }
}

fn kind_if_distinct(kind: Option<&str>, title: &str) -> Option<String> {
    let trimmed = kind?.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case(title.trim()) {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn source_link_from_parts(open_label: Option<&str>, open_uri: Option<&str>) -> Option<SourceLink> {
    let url = open_uri?.trim();
    if !ui::opens(url) {
        return None;
    }
    Some(SourceLink {
        url: url.to_owned(),
        label: open_label
            .map(str::trim)
            .filter(|label| !label.is_empty())
            .unwrap_or("Open original")
            .to_owned(),
        new_tab_hint: Some("Opens in a new tab".to_owned()),
    })
}

pub(crate) fn expand_source(
    chip: &SourceChip,
    on_open: EventHandler<SourceChip>,
) -> Option<ExpandSource> {
    if !chip.open {
        return None;
    }
    let chip = chip.clone();
    Some(ExpandSource {
        label: "Expand".to_owned(),
        on_expand: EventHandler::new(move |_| on_open.call(chip.clone())),
    })
}

pub(crate) fn chip_original_link(chip: &SourceChip) -> Option<SourceLink> {
    source_link_from_parts(chip.open_label.as_deref(), chip.open_uri.as_deref())
}

/// Builds inline hover/focus preview content for [`ui::CitationPreview`].
pub(crate) fn preview_content(chip: &SourceChip) -> PreviewContent {
    if !chip.open {
        let message = if chip.preview.trim().is_empty() {
            "Unavailable".to_owned()
        } else {
            chip.preview.clone()
        };
        return PreviewContent::Described(SourceSummary {
            text: message,
            prepared: "Unavailable".to_owned(),
        });
    }
    if chip.origin == CitationOrigin::PreparedByCoda {
        let text = chip
            .preview
            .trim()
            .strip_prefix("Prepared by Coda\n")
            .unwrap_or(chip.preview.trim())
            .to_owned();
        let prepared = source_provenance_line(chip.origin, &chip.as_of)
            .unwrap_or_else(|| "Prepared by Coda".to_owned());
        return PreviewContent::Described(SourceSummary { text, prepared });
    }
    let about = RecordAbout {
        kind: kind_if_distinct(chip.record_kind.as_deref(), &chip.label),
        date: (!chip.as_of.trim().is_empty()).then(|| chip.as_of.clone()),
        status: classify_record_status(chip.record_status.as_deref()),
    };
    let place = match chip.target {
        CitationTargetKind::Passage if !chip.quote.trim().is_empty() => {
            Some(ui::CitedPlace::Quoted(Places::passage(chip.quote.clone())))
        }
        CitationTargetKind::WholeRecord
        | CitationTargetKind::Passage
        | CitationTargetKind::PassageNotFound => None,
    };
    PreviewContent::Cited { about, place }
}

/// Deduplicates unique cited sources in citation-number order for `AnswerSources`.
/// Returns `None` when the answer cites zero sources.
pub(crate) fn answer_sources(
    body: &[SynopsisPiece],
    basis: &[SynopsisPiece],
) -> Option<AnswerSources> {
    let mut unique: BTreeMap<usize, IndexedSource> = BTreeMap::new();
    for piece in body.iter().chain(basis.iter()) {
        if let SynopsisPiece::Chips(chips) = piece {
            for chip in chips {
                unique.entry(chip.index).or_insert_with(|| IndexedSource {
                    number: chip.index,
                    title: chip.label.clone(),
                    detail: (!chip.as_of.trim().is_empty()).then(|| chip.as_of.clone()),
                });
            }
        }
    }
    if unique.is_empty() {
        return None;
    }
    Some(AnswerSources {
        label: "Sources".to_owned(),
        sources: unique.into_values().collect(),
    })
}

/// Renders a [`SourcePanelLoad`] into `ui::SourceReading`'s `Option<SourcePane>`.
pub(crate) fn build_source_pane(
    panel: &SourcePanelLoad,
    on_retry: EventHandler,
) -> Option<SourcePane> {
    build_source_pane_with(panel, || on_retry)
}

/// Renders a [`SourcePanelLoad`] while building the retry callback lazily only for failures.
pub(crate) fn build_source_pane_with(
    panel: &SourcePanelLoad,
    on_retry: impl FnOnce() -> EventHandler,
) -> Option<SourcePane> {
    match panel {
        SourcePanelLoad::Idle => None,
        SourcePanelLoad::Loading { chip, .. } => Some(SourcePane {
            eyebrow: format!("Source {}", chip.index),
            title: chip.label.clone(),
            state: SourceState::Loading("Loading cited source…".to_owned()),
            back: "Back".to_owned(),
            close: "Close".to_owned(),
        }),
        SourcePanelLoad::Unavailable {
            eyebrow,
            title,
            reason,
        } => Some(SourcePane {
            eyebrow: eyebrow.clone(),
            title: title.clone(),
            state: SourceState::Unavailable(reason.clone()),
            back: "Back".to_owned(),
            close: "Close".to_owned(),
        }),
        SourcePanelLoad::Failed { chip, message, .. } => Some(SourcePane {
            eyebrow: format!("Source {}", chip.index),
            title: chip.label.clone(),
            state: SourceState::Failed {
                message: message.clone(),
                retry: crate::TRY_AGAIN.to_owned(),
                on_retry: on_retry(),
            },
            back: "Back".to_owned(),
            close: "Close".to_owned(),
        }),
        SourcePanelLoad::Loaded { chip, source } => {
            let eyebrow = format!("Source {}", source.number.max(chip.index));
            let title = if source.title.trim().is_empty() {
                chip.label.clone()
            } else {
                source.title.clone()
            };
            let has_raw = source
                .raw_content
                .as_deref()
                .is_some_and(|raw| !raw.trim().is_empty());
            if source.origin == CitationOrigin::PreparedByCoda
                && source.readable_parts.is_empty()
                && source.fields.is_empty()
                && !has_raw
            {
                let text = source
                    .body
                    .trim()
                    .strip_prefix("Prepared by Coda\n")
                    .unwrap_or(source.body.trim())
                    .to_owned();
                let prepared = source_provenance_line(
                    source.origin,
                    source.recorded_at.as_deref().unwrap_or(&chip.as_of),
                )
                .unwrap_or_else(|| "Prepared by Coda".to_owned());
                return Some(SourcePane {
                    eyebrow,
                    title,
                    state: SourceState::Described(SourceSummary { text, prepared }),
                    back: "Back".to_owned(),
                    close: "Close".to_owned(),
                });
            }

            let date = source
                .recorded_at
                .clone()
                .filter(|d| !d.trim().is_empty())
                .or_else(|| (!chip.as_of.trim().is_empty()).then(|| chip.as_of.clone()));
            let about = RecordAbout {
                kind: kind_if_distinct(source.record_type.as_deref(), &title),
                date,
                status: classify_record_status(
                    source
                        .record_status
                        .as_deref()
                        .or(chip.record_status.as_deref()),
                ),
            };

            let mut parts: Vec<TextPart> = source
                .readable_parts
                .iter()
                .map(|part| TextPart {
                    label: part.label.clone(),
                    text: part.text.clone(),
                })
                .collect();
            if parts.is_empty() && source.fields.is_empty() {
                let body_text = source.body.trim();
                if !body_text.is_empty() {
                    parts.push(TextPart {
                        label: None,
                        text: body_text.to_owned(),
                    });
                }
            }

            if parts.is_empty() && source.fields.is_empty() && !has_raw {
                return Some(SourcePane {
                    eyebrow,
                    title,
                    state: SourceState::Unavailable(
                        "Source content is unavailable here.".to_owned(),
                    ),
                    back: "Back".to_owned(),
                    close: "Close".to_owned(),
                });
            }

            let groups = fields_to_groups(&source.fields);
            let cited = locate_cited_at(chip.target, &chip.quote, &parts, &source.fields);
            let open_label = source
                .external_url
                .as_ref()
                .map(|_| {
                    if source.source_label.trim().is_empty() {
                        "Open original".to_owned()
                    } else {
                        format!("Open in {}", source.source_label.trim())
                    }
                })
                .or_else(|| chip.open_label.clone());
            let original = source_link_from_parts(
                open_label.as_deref(),
                source.external_url.as_deref().or(chip.open_uri.as_deref()),
            );

            Some(SourcePane {
                eyebrow,
                title,
                state: SourceState::Shown(Box::new(SourceRecord {
                    about,
                    body: RecordBody {
                        parts,
                        groups,
                        range_label: "Reference range".to_owned(),
                        cited,
                    },
                    original,
                    raw: RawRecord {
                        label: "Technical details".to_owned(),
                        text: source.raw_content.clone().unwrap_or_default(),
                    },
                })),
                back: "Back".to_owned(),
                close: "Close".to_owned(),
            })
        }
    }
}

fn fields_to_groups(fields: &[CitedFieldWire]) -> Vec<FieldGroup> {
    if fields.is_empty() {
        return Vec::new();
    }
    vec![FieldGroup {
        heading: None,
        fields: fields
            .iter()
            .map(|f| RecordField {
                key: f.key.clone(),
                name: f.name.clone(),
                value: FieldValue::Given {
                    value: f.value.clone(),
                    unit: f.unit.clone(),
                    range: f.range.clone(),
                    status: f.status.clone(),
                },
            })
            .collect(),
    }]
}

/// Resolves where `quote` points inside `parts` or `fields` according to `target`.
pub(crate) fn locate_cited_at(
    target: CitationTargetKind,
    quote: &str,
    parts: &[TextPart],
    fields: &[CitedFieldWire],
) -> CitedAt {
    match target {
        CitationTargetKind::WholeRecord => CitedAt::Whole,
        CitationTargetKind::PassageNotFound => {
            CitedAt::NotFound(PASSAGE_NOT_FOUND_NOTICE.to_owned())
        }
        CitationTargetKind::Passage => {
            let trimmed = quote
                .trim()
                .trim_end_matches('…')
                .trim_end_matches("...")
                .trim();
            if trimmed.is_empty() {
                return CitedAt::Whole;
            }
            for (part_idx, part) in parts.iter().enumerate() {
                if let Some(range) = find_passage_range(&part.text, trimmed) {
                    return CitedAt::Places(Places::passage(PartRange {
                        part: part_idx,
                        range,
                    }));
                }
            }
            let norm_quote = collapse_norm(trimmed);
            for field in fields {
                let norm_val = collapse_norm(&field.value);
                let norm_name = collapse_norm(&field.name);
                if (!norm_val.is_empty() && norm_quote.contains(&norm_val))
                    || (!norm_val.is_empty() && norm_val.contains(&norm_quote))
                    || (!norm_name.is_empty() && norm_quote.contains(&norm_name))
                {
                    return CitedAt::Places(Places::field(field.key.clone()));
                }
            }
            CitedAt::NotFound(PASSAGE_NOT_FOUND_NOTICE.to_owned())
        }
    }
}

fn find_passage_range(haystack: &str, needle: &str) -> Option<(usize, usize)> {
    if let Some(start) = haystack.find(needle) {
        return Some((start, start + needle.len()));
    }
    let hay_tokens = whitespace_word_spans(haystack);
    let needle_words: Vec<String> = needle.split_whitespace().map(str::to_lowercase).collect();
    if needle_words.is_empty() || hay_tokens.len() < needle_words.len() {
        return None;
    }
    for window in hay_tokens.windows(needle_words.len()) {
        if window
            .iter()
            .zip(needle_words.iter())
            .all(|((_, _, hay_word), needle_word)| hay_word == needle_word)
        {
            let start = window.first()?.0;
            let end = window.last()?.1;
            return Some((start, end));
        }
    }
    None
}

fn whitespace_word_spans(text: &str) -> Vec<(usize, usize, String)> {
    let mut out = Vec::new();
    let mut word_start: Option<usize> = None;
    for (idx, ch) in text.char_indices() {
        if ch.is_whitespace() {
            if let Some(start) = word_start.take() {
                out.push((start, idx, text[start..idx].to_lowercase()));
            }
        } else if word_start.is_none() {
            word_start = Some(idx);
        }
    }
    if let Some(start) = word_start {
        out.push((start, text.len(), text[start..].to_lowercase()));
    }
    out
}

fn collapse_norm(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flagged_statuses_classify_before_settled_ones() {
        assert_eq!(
            classify_record_status(Some("preliminary")),
            Some(RecordStatus::Flagged("preliminary".into()))
        );
        assert_eq!(
            classify_record_status(Some("entered-in-error")),
            Some(RecordStatus::Flagged("entered-in-error".into()))
        );
        assert_eq!(
            classify_record_status(Some("Final")),
            Some(RecordStatus::Settled("Final".into()))
        );
    }

    #[test]
    fn locate_cited_at_highlights_matched_passage_and_flags_missing_passage() {
        let parts = vec![TextPart {
            label: Some("Results".into()),
            text: "Potassium   5.4 mmol/L\nCreatinine 1.2 mg/dL".into(),
        }];
        assert_eq!(
            locate_cited_at(
                CitationTargetKind::Passage,
                "Potassium 5.4 mmol/L",
                &parts,
                &[],
            ),
            CitedAt::Places(Places::passage(PartRange {
                part: 0,
                range: (0, 22),
            }))
        );
        assert!(matches!(
            locate_cited_at(
                CitationTargetKind::Passage,
                "Sodium 128 mmol/L",
                &parts,
                &[],
            ),
            CitedAt::NotFound(_)
        ));
        assert_eq!(
            locate_cited_at(
                CitationTargetKind::WholeRecord,
                "Potassium 5.4 mmol/L",
                &parts,
                &[],
            ),
            CitedAt::Whole
        );
    }

    #[test]
    fn remove_panel_access_turns_open_citation_unavailable() {
        let chip = SourceChip {
            index: 1,
            source_key: Some("r1:lab-1:v1".into()),
            label: "BMP".into(),
            open: true,
            target: CitationTargetKind::Passage,
            quote: "Potassium 5.4".into(),
            preview: "Potassium 5.4".into(),
            as_of: "2026-11-10".into(),
            record_kind: None,
            record_status: None,
            readable_parts: Vec::new(),
            fields: Vec::new(),
            raw_content: None,
            open_label: None,
            open_uri: None,
            origin: CitationOrigin::SourceRecord,
        };
        let state = SourcePanelLoad::from_chip(chip).into_unavailable("Panel access revoked.");
        assert!(matches!(
            state,
            SourcePanelLoad::Unavailable { reason, .. } if reason == "Panel access revoked."
        ));
    }

    #[test]
    fn native_source_and_fallback_summary_provenance_stay_distinct() {
        let native_chip = SourceChip {
            index: 1,
            source_key: None,
            label: "Basic metabolic panel".to_owned(),
            open: true,
            target: CitationTargetKind::Passage,
            quote: "Potassium 4.4 mmol/L".to_owned(),
            preview: "Potassium 4.4 mmol/L".to_owned(),
            as_of: "April 13, 2026".to_owned(),
            record_kind: None,
            record_status: None,
            readable_parts: Vec::new(),
            fields: Vec::new(),
            raw_content: None,
            open_label: Some("Open in EHR".to_owned()),
            open_uri: Some("https://ehr.example/lab/1".to_owned()),
            origin: CitationOrigin::SourceRecord,
        };
        let fallback_chip = SourceChip {
            origin: CitationOrigin::PreparedByCoda,
            ..native_chip.clone()
        };
        let native_pane = build_source_pane_with(&SourcePanelLoad::from_chip(native_chip), || {
            panic!("non-failure should not allocate retry handler")
        })
        .expect("open pane");
        assert!(matches!(
            native_pane.state,
            ui::SourceState::Shown(ref record)
                if record.about.date.as_deref() == Some("April 13, 2026")
        ));
        let fallback_pane =
            build_source_pane_with(&SourcePanelLoad::from_chip(fallback_chip), || {
                panic!("non-failure should not allocate retry handler")
            })
            .expect("open pane");
        assert!(matches!(
            fallback_pane.state,
            ui::SourceState::Described(ref summary)
                if summary.prepared == "Prepared by Coda · as of April 13, 2026"
        ));
    }
}
