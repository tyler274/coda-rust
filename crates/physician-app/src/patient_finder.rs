//! Shared Find patient search component used by both header search (`SearchDialog`)
//! and message-matching / move-conversation patient pickers.

use dioxus::prelude::*;
use physician_wire::{PatientSearchRow, SearchMatchMode};
use ui::{Button, ButtonVariant, EmptyLine, IconName, Notice, SearchField, Tone};

use crate::shell::{ReadView, RosterContinuation, search_row_detail};
use crate::status::{ReadNotice, ReadStage};

const KEYWORD_FALLBACK_NOTICE: &str = "No exact or prefix match — showing keyword matches.";

/// How activating a search row behaves on the hosting surface.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum FindPatientAction {
    /// Whole row activates opening the selected patient.
    OpenRow,
    /// Explicit primary selection button beside each patient row (`Messages to match`).
    ChooseButton {
        /// Whether a picker submission command is currently in flight.
        busy: bool,
    },
}

/// Shared patient-roster search field, lookup failure notice, keyword-fallback notice,
/// scrollable results list, and continuation pagination.
#[component]
pub(crate) fn FindPatientSearch(
    query: String,
    label: String,
    placeholder: String,
    rows: ReadView<Vec<PatientSearchRow>>,
    match_mode: SearchMatchMode,
    continuation: RosterContinuation,
    loading_copy: String,
    unavailable_copy: String,
    empty_copy: String,
    #[props(default)] may_be_missing_someone: bool,
    action: FindPatientAction,
    focused: Option<i32>,
    on_query: EventHandler<String>,
    on_retry: EventHandler,
    on_scroll_more: EventHandler,
    on_retry_more: EventHandler,
    on_select: EventHandler<PatientSearchRow>,
    on_editing: EventHandler<bool>,
) -> Element {
    let results = rows.content().cloned().unwrap_or_default();
    let first = results.first().cloned();
    let empty_message = if rows.content().is_some_and(Vec::is_empty) && !may_be_missing_someone {
        Some(empty_copy)
    } else {
        None
    };
    let more_stage = match continuation {
        RosterContinuation::Complete | RosterContinuation::HasMore { .. } => ReadStage::Showing,
        RosterContinuation::LoadingMore { .. } => ReadStage::Loading,
        RosterContinuation::FailedMore { .. } => ReadStage::Unavailable,
    };

    rsx! {
        div { class: "patient-search",
            SearchField {
                label,
                placeholder,
                clear_label: "Clear search",
                value: query,
                autofocus: true,
                onfocus: move |_| on_editing.call(true),
                onblur: move |_| on_editing.call(false),
                on_input: move |words| on_query.call(words),
                onkeydown: move |event: KeyboardEvent| {
                    if matches!(action, FindPatientAction::OpenRow)
                        && event.key().to_string() == "Enter"
                    {
                        event.stop_propagation();
                        if let Some(patient) = first.clone() {
                            on_select.call(patient);
                        }
                    }
                },
            }
            ReadNotice {
                stage: rows.stage(),
                loading: loading_copy,
                stale: "",
                unavailable: unavailable_copy,
                retry: crate::TRY_AGAIN,
                on_retry,
            }
            crate::access_notice::MayBeMissingSomeoneNotice {
                visible: may_be_missing_someone,
            }
            if !results.is_empty() && match_mode == SearchMatchMode::KeywordFallback {
                div { class: "mr-banner", "data-search-match-mode": "keyword_fallback",
                    Notice {
                        tone: Tone::Neutral,
                        icon: Some(IconName::Search),
                        title: KEYWORD_FALLBACK_NOTICE.to_owned(),
                    }
                }
            }
            if let Some(copy) = empty_message {
                EmptyLine { message: copy }
            }
            if !results.is_empty() {
                div {
                    class: "patient-search-scroll",
                    onscroll: move |event| {
                        let data = event.data();
                        let from_end = f64::from(data.scroll_height())
                            - f64::from(data.client_height())
                            - data.scroll_top();
                        if from_end <= crate::ROSTER_SCROLL_THRESHOLD_PX {
                            on_scroll_more.call(());
                        }
                    },
                    match action {
                        FindPatientAction::OpenRow => rsx! {
                            ui::RowGroup { class: "dialog-results",
                                for (index, patient) in results.iter().enumerate() {
                                    li { key: "{patient.patient_workspace_id}",
                                        ui::ListRow {
                                            class: if focused == Some(index as i32) { "is-keyboard-focus" } else { "" },
                                            title: patient.patient_display_name.clone(),
                                            detail: search_row_detail(
                                                patient.birth_date.as_deref(),
                                                patient.mrn.as_deref(),
                                            ),
                                            onclick: {
                                                let picked = patient.clone();
                                                move |_| on_select.call(picked.clone())
                                            },
                                        }
                                    }
                                }
                            }
                        },
                        FindPatientAction::ChooseButton { busy } => rsx! {
                            div { class: "mr-picker-results",
                                for patient in results {
                                    div {
                                        key: "{patient.patient_workspace_id}",
                                        class: "mr-picker-row",
                                        "data-picker-patient": "{patient.patient_workspace_id}",
                                        div { class: "mr-picker-who",
                                            strong { "{patient.patient_display_name}" }
                                            {
                                                let detail = search_row_detail(
                                                    patient.birth_date.as_deref(),
                                                    patient.mrn.as_deref(),
                                                );
                                                rsx! {
                                                    if !detail.is_empty() {
                                                        span { class: "mr-picker-meta", "{detail}" }
                                                    }
                                                }
                                            }
                                        }
                                        Button {
                                            compact: true,
                                            variant: ButtonVariant::Primary,
                                            disabled: busy,
                                            "data-picker-select": "{patient.patient_workspace_id}",
                                            onclick: {
                                                let picked = patient.clone();
                                                move |_| on_select.call(picked.clone())
                                            },
                                            "Choose patient"
                                        }
                                    }
                                }
                            }
                        },
                    }
                    ReadNotice {
                        stage: more_stage,
                        loading: crate::ROSTER_MORE_LOADING,
                        stale: "",
                        unavailable: crate::ROSTER_MORE_UNAVAILABLE,
                        retry: crate::TRY_AGAIN,
                        on_retry: on_retry_more,
                    }
                }
            }
        }
    }
}
