//! Duet `"For my doctor"` (`Shared` tab, `You` mark/note/send actions, `Choosing` disclosure,
//! `"Not saved yet"` / `409 version_conflict` preservation, `"Couldn't confirm the send"` +
//! `"Try again"`, `COD-50` removal notices, and `COD-51` ended-before-confirm state).

use std::collections::BTreeSet;

use dioxus::prelude::*;
use duet_client::for_my_doctor::{
    AdHocRecordSend, CreateDoctorNoteRequest, ForMyDoctorClientError, ForMyDoctorMark,
    ForMyDoctorNote, ForMyDoctorNotice, ForMyDoctorSend, ForMyDoctorState, MarkForDoctorRequest,
    MarkTargetKind, PendingDoctorSend, PracticeConnection, PracticeStanding, RecordFamily,
    RetainedDoctorNoteDraft, SendForMyDoctorPayload, SendStanding, UpdateDoctorNoteRequest,
};
use ui::{
    ActionRow, Button, ButtonVariant, Card, Checkbox, Choice, DraftConflict, IconName, Notice,
    Paper, PaperSection, RadioGroup, StatePill, TextField, TextKind, Tone, TryAgain,
};
use uuid::Uuid;

use crate::account::PatientSession;
use crate::open_client;
use crate::roots::{LibraryKind, LibraryRecord};

/// Mandatory patient disclosure shown before every `"For my doctor"` send (`Rule 2` / `COD-59`).
pub const SEND_DISCLOSURE_TEXT: &str =
    "A send can't be taken back. This isn't for urgent needs or for questions that need an answer.";

/// Send progress state inside `Choosing`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ChoosingSendStatus {
    /// Ready for confirmation.
    #[default]
    Idle,
    /// Submitting `POST /api/for-my-doctor/send`.
    Sending,
    /// Validation or local error shown inline before closing `Choosing`.
    Failed(String),
}

/// Optional `Choosing` pre-selection passed either from `To send` (all marked items & notes)
/// or from a single open record in `You` (`ad_hoc`).
#[derive(Clone, Debug, PartialEq)]
pub struct ChoosingSession {
    /// Selected mark IDs from `To send`.
    pub mark_ids: BTreeSet<Uuid>,
    /// Selected note IDs from `To send`.
    pub note_ids: BTreeSet<Uuid>,
    /// Single open record passed straight from `You` -> `Choosing`.
    pub ad_hoc: Option<AdHocRecordSend>,
    /// Include the ad-hoc record when sending.
    pub include_ad_hoc: bool,
    /// Chosen connected practice index (`0` default when 1 connected).
    pub practice_index: Option<usize>,
    /// Confirmed the mandatory disclosure checkbox.
    pub disclosure_accepted: bool,
    /// Current submit state inside `Choosing`.
    pub send_status: ChoosingSendStatus,
}

/// Shared reactive handle provided in Dioxus context inside `PrivateRoot` so both `You`
/// and `Shared` reflect marks, notes, sends, and notices immediately.
#[derive(Clone, Copy)]
pub struct ForMyDoctorSession {
    /// Loaded `For my doctor` workspace state (`marks`, `notes`, `sends`, `notices`, `practices`).
    pub state: Signal<Option<ForMyDoctorState>>,
    /// True while fetching `/api/for-my-doctor`.
    pub loading: Signal<bool>,
    /// True when the latest fetch failed without loading state.
    pub load_failed: Signal<bool>,
    /// True while a mark/unmark mutation is in flight.
    pub busy: Signal<bool>,
    /// Honest `"Not saved yet"` failure banner for mark/note actions.
    pub mark_unsaved: Signal<Option<String>>,
    /// Active free-text note editor state (`Rule 9`).
    pub note_editor: Signal<Option<RetainedDoctorNoteDraft>>,
    /// Honest `"Not saved yet"` banner when saving a note fails or conflicts.
    pub note_unsaved: Signal<Option<String>>,
    /// Active `Choosing` selection/disclosure modal state.
    pub choosing: Signal<Option<ChoosingSession>>,
    /// Retained pending send across `"Couldn't confirm the send"` / `COD-51`.
    pub pending_send: Signal<Option<PendingDoctorSend>>,
    /// One-shot status banner after a send completes, fails, or ends.
    pub send_notice: Signal<Option<(Tone, String)>>,
    /// Increment to re-fetch `/api/for-my-doctor`.
    pub reload_tick: Signal<u32>,
}

impl ForMyDoctorSession {
    /// Returns true when `target_key` is currently marked in `To send`.
    pub fn is_marked(&self, target_key: &str) -> bool {
        (self.state)()
            .as_ref()
            .is_some_and(|s| s.marks.iter().any(|m| m.target_key == target_key))
    }

    /// Lists all active (`Connected`) practices in display order.
    pub fn connected_practices(&self) -> Vec<PracticeConnection> {
        (self.state)()
            .as_ref()
            .map(|s| {
                s.practices
                    .iter()
                    .filter(|p| matches!(p.standing, PracticeStanding::Connected { .. }))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Un-dismissed `COD-50` source-removal notices shown in `You`.
    pub fn notices(&self) -> Vec<ForMyDoctorNotice> {
        (self.state)()
            .as_ref()
            .map(|s| s.notices.clone())
            .unwrap_or_default()
    }
}

/// Builds a stable deduplicating `target_key` for one canonical record in `You`.
pub fn record_target_key(record_id: &str) -> String {
    format!("record:{record_id}")
}

/// Maps `LibraryKind` into canonical `RecordFamily`.
pub fn library_record_family(kind: LibraryKind) -> RecordFamily {
    match kind {
        LibraryKind::HealthEvidence => RecordFamily::HealthEvidence,
        LibraryKind::ClinicalNote => RecordFamily::ClinicalNote,
    }
}

/// Builds a `MarkForDoctorRequest` for one open or listed `LibraryRecord` in `You`.
pub fn mark_request_for_library_record(record: &LibraryRecord) -> MarkForDoctorRequest {
    let parsed_id = Uuid::parse_str(record.id.trim()).ok();
    MarkForDoctorRequest {
        target_kind: MarkTargetKind::Record,
        target_key: record_target_key(&record.id),
        record_id: parsed_id,
        entity: parsed_id.map(|_| library_record_family(record.kind)),
        title: record.title.clone(),
        summary_text: record.summary.clone().unwrap_or_default(),
        range_label: None,
        source_label: record.source.clone(),
        payload: serde_json::json!({
            "when": record.when,
            "status_line": record.status_line,
        }),
    }
}

/// Strips reference range noise from a summary string for safe client-side display.
pub fn plain_summary_without_ranges(raw: &str) -> String {
    let filtered: Vec<&str> = raw
        .lines()
        .map(str::trim)
        .filter(|line| {
            let lower = line.to_ascii_lowercase();
            !lower.contains("reference range")
                && !lower.contains("ref range")
                && !lower.contains("normal range")
        })
        .collect();
    filtered.join(" · ")
}

/// Initializes [`ForMyDoctorSession`] inside `PrivateRoot`, restoring retained pending sends
/// and unsaved free-text notes from [`PatientSession`] retention storage.
pub fn use_for_my_doctor_session(
    base_url: String,
    bearer: String,
    patient: &PatientSession,
) -> ForMyDoctorSession {
    let cached_state = patient
        .retention
        .load_for_my_doctor_cache(&patient.fence)
        .ok()
        .flatten();
    let retained_send = patient
        .retention
        .load_pending_doctor_send(&patient.fence)
        .ok()
        .flatten();
    let retained_note = patient
        .retention
        .load_doctor_note_draft(&patient.fence)
        .ok()
        .flatten();

    let mut session = ForMyDoctorSession {
        state: use_signal(move || cached_state),
        loading: use_signal(|| true),
        load_failed: use_signal(|| false),
        busy: use_signal(|| false),
        mark_unsaved: use_signal(|| None),
        note_editor: use_signal(move || retained_note),
        note_unsaved: use_signal(|| None),
        choosing: use_signal(|| None),
        pending_send: use_signal(move || retained_send),
        send_notice: use_signal(|| None),
        reload_tick: use_signal(|| 0u32),
    };

    let resource = use_resource(move || {
        let _tick = (session.reload_tick)();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        async move {
            match open_client(base_url, bearer) {
                Ok(api) => api.for_my_doctor_state(None).await,
                Err(err) => Err(ForMyDoctorClientError::Uncertain(err)),
            }
        }
    });

    let cache_owner = patient.retention.clone();
    let cache_fence = patient.fence.clone();
    use_effect(move || {
        let res = resource();
        session.loading.set(res.is_none());
        match res {
            Some(Ok(loaded)) => {
                let _ = cache_owner.save_for_my_doctor_cache(&cache_fence, &loaded);
                session.state.set(Some(loaded));
                session.load_failed.set(false);
            }
            Some(Err(_)) => {
                session.load_failed.set(true);
            }
            None => {}
        }
    });

    session
}

/// Toggles a `Mark` / `Unmark` target in `You` with honest `"Not saved yet"` failure feedback.
pub fn toggle_mark_action(
    mut fmd: ForMyDoctorSession,
    base_url: String,
    bearer: String,
    request: MarkForDoctorRequest,
) {
    if (fmd.busy)() {
        return;
    }
    let currently_marked = fmd.is_marked(&request.target_key);
    fmd.busy.set(true);
    fmd.mark_unsaved.set(None);
    spawn(async move {
        let result = match open_client(base_url, bearer) {
            Ok(api) => {
                if currently_marked {
                    api.unmark_for_doctor(&request.target_key)
                        .await
                        .map(|()| None)
                } else {
                    api.mark_for_doctor(&request).await.map(Some)
                }
            }
            Err(err) => Err(ForMyDoctorClientError::Uncertain(err)),
        };
        match result {
            Ok(maybe_mark) => {
                if let Some(mut st) = (fmd.state)() {
                    if let Some(mark) = maybe_mark {
                        if let Some(existing) = st
                            .marks
                            .iter_mut()
                            .find(|m| m.target_key == mark.target_key)
                        {
                            *existing = mark;
                        } else {
                            st.marks.push(mark);
                        }
                    } else {
                        st.marks.retain(|m| m.target_key != request.target_key);
                    }
                    fmd.state.set(Some(st));
                } else {
                    fmd.reload_tick.set((fmd.reload_tick)() + 1);
                }
                fmd.mark_unsaved.set(None);
            }
            Err(_) => {
                fmd.mark_unsaved.set(Some(format!(
                    "Not saved yet — \"{}\" couldn't be updated. Your change was kept here so you can try again.",
                    request.title
                )));
            }
        }
        fmd.busy.set(false);
    });
}

/// Opens a new note draft linked to an open record in `You`.
pub fn start_note_for_record(mut fmd: ForMyDoctorSession, record: &LibraryRecord) {
    let parsed_id = Uuid::parse_str(record.id.trim()).ok();
    fmd.note_unsaved.set(None);
    fmd.note_editor.set(Some(RetainedDoctorNoteDraft {
        note_id: Uuid::new_v4(),
        is_new: true,
        expected_version: None,
        unsaved_body: String::new(),
        linked_record_id: parsed_id,
        linked_record_title: Some(record.title.clone()),
        linked_entity: parsed_id.map(|_| library_record_family(record.kind)),
        conflict_latest: None,
    }));
}

/// Opens `Choosing` pre-selected with only the one open record from `You` (`Rule 4`).
pub fn start_single_record_send(mut fmd: ForMyDoctorSession, record: &LibraryRecord) {
    let parsed_id = Uuid::parse_str(record.id.trim()).ok();
    let connected = fmd.connected_practices();
    let practice_index = (connected.len() == 1).then_some(0);
    fmd.send_notice.set(None);
    fmd.choosing.set(Some(ChoosingSession {
        mark_ids: BTreeSet::new(),
        note_ids: BTreeSet::new(),
        ad_hoc: Some(AdHocRecordSend {
            record_id: parsed_id,
            entity: parsed_id.map(|_| library_record_family(record.kind)),
            title: record.title.clone(),
            summary_text: plain_summary_without_ranges(
                record.summary.as_deref().unwrap_or_default(),
            ),
            source_label: record.source.clone(),
        }),
        include_ad_hoc: true,
        practice_index,
        disclosure_accepted: false,
        send_status: ChoosingSendStatus::Idle,
    }));
}

/// Opens `Choosing` from `Shared` pre-selecting every mark and note in `To send`.
fn start_to_send_choosing(mut fmd: ForMyDoctorSession, state: &ForMyDoctorState) {
    let connected = fmd.connected_practices();
    let practice_index = (connected.len() == 1).then_some(0);
    fmd.send_notice.set(None);
    fmd.choosing.set(Some(ChoosingSession {
        mark_ids: state.marks.iter().map(|m| m.id).collect(),
        note_ids: state.notes.iter().map(|n| n.id).collect(),
        ad_hoc: None,
        include_ad_hoc: false,
        practice_index,
        disclosure_accepted: false,
        send_status: ChoosingSendStatus::Idle,
    }));
}

/// Renders one `Mark` / `Unmark` toggle button using the shared `Button` component.
#[component]
pub fn MarkToggleButton(
    target_key: String,
    request: MarkForDoctorRequest,
    base_url: String,
    bearer: String,
    #[props(default = true)] compact: bool,
) -> Element {
    let fmd = use_context::<ForMyDoctorSession>();
    let marked = fmd.is_marked(&target_key);
    let label = if marked { "Unmark" } else { "Mark" };
    let aria = if marked {
        format!("Unmark {}", request.title)
    } else {
        format!("Mark {}", request.title)
    };
    rsx! {
        Button {
            compact,
            variant: if marked { ButtonVariant::Primary } else { ButtonVariant::Secondary },
            disabled: (fmd.busy)(),
            aria_label: aria,
            onclick: move |_| {
                toggle_mark_action(fmd, base_url.clone(), bearer.clone(), request.clone());
            },
            "{label}"
        }
    }
}

/// Renders un-dismissed `COD-50` notices in `You` when a clinical source removed a marked record.
#[component]
pub fn YouRemovalNotices(base_url: String, bearer: String) -> Element {
    let mut fmd = use_context::<ForMyDoctorSession>();
    let notices = fmd.notices();
    let unsaved = (fmd.mark_unsaved)();
    rsx! {
        if let Some(msg) = unsaved {
            Notice {
                tone: Tone::Warning,
                icon: Some(IconName::Alert),
                title: "Not saved yet".to_owned(),
                detail: Some(msg),
            }
        }
        for notice in notices {
            {
                let notice_id = notice.id;
                let url = base_url.clone();
                let token = bearer.clone();
                rsx! {
                    Notice {
                        key: "{notice_id}",
                        tone: Tone::Neutral,
                        icon: Some(IconName::File),
                        title: format!("{} removed from To send", notice.title),
                        detail: Some(notice.message.clone()),
                        ActionRow {
                            Button {
                                compact: true,
                                variant: ButtonVariant::Quiet,
                                onclick: move |_| {
                                    let url = url.clone();
                                    let token = token.clone();
                                    spawn(async move {
                                        if let Ok(api) = open_client(url, token)
                                            && api.dismiss_for_my_doctor_notice(notice_id).await.is_ok()
                                            && let Some(mut st) = (fmd.state)()
                                        {
                                            st.notices.retain(|n| n.id != notice_id);
                                            fmd.state.set(Some(st));
                                        }
                                    });
                                },
                                "Dismiss"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Renders an opened record sheet in `You` allowing value-level marking (e.g. Systolic / Diastolic),
/// whole-record `Mark`/`Unmark`, linked `"Add note"`, and single-record `"Send"` -> `Shared` `Choosing`.
#[component]
pub fn OpenRecordSheet(
    record: LibraryRecord,
    base_url: String,
    bearer: String,
    on_close: EventHandler<()>,
    on_open_shared: EventHandler<()>,
) -> Element {
    let mut fmd = use_context::<ForMyDoctorSession>();
    let patient = use_context::<PatientSession>();
    let parsed_id = Uuid::parse_str(record.id.trim()).ok();
    let entity = parsed_id.map(|_| library_record_family(record.kind));
    let summary_clean = plain_summary_without_ranges(record.summary.as_deref().unwrap_or_default());
    let bp_values = parse_bp_values(&summary_clean);
    let record_req = mark_request_for_library_record(&record);

    rsx! {
        div { role: "region", aria_label: format!("Open record {}", record.title),
            Paper {
                title: record.title.clone(),
                detail: record.source.clone(),
                if !summary_clean.is_empty() {
                    p { "{summary_clean}" }
                }
                if let Some(when) = &record.when {
                    p { class: "record-meta", "{when}" }
                }
                if let Some(status) = &record.status_line {
                    p { class: "record-meta", "{status}" }
                }
                ActionRow {
                    MarkToggleButton {
                        target_key: record_req.target_key.clone(),
                        request: record_req.clone(),
                        base_url: base_url.clone(),
                        bearer: bearer.clone(),
                    }
                    Button {
                        compact: true,
                        onclick: {
                            let rec = record.clone();
                            move |_| start_note_for_record(fmd, &rec)
                        },
                        "Add note"
                    }
                    if !fmd.connected_practices().is_empty() && (fmd.pending_send)().is_none() {
                        Button {
                            compact: true,
                            variant: ButtonVariant::Primary,
                            onclick: {
                                let rec = record.clone();
                                move |_| {
                                    start_single_record_send(fmd, &rec);
                                    on_close.call(());
                                    on_open_shared.call(());
                                }
                            },
                            "Send"
                        }
                    }
                    Button {
                        compact: true,
                        variant: ButtonVariant::Quiet,
                        onclick: move |_| on_close.call(()),
                        "Done"
                    }
                }
                if let Some((sys, dia)) = bp_values {
                    PaperSection { heading: "Values".to_owned(),
                        ActionRow {
                            MarkToggleButton {
                                target_key: format!("value:{}:systolic", record.id),
                                request: MarkForDoctorRequest {
                                    target_kind: MarkTargetKind::Value,
                                    target_key: format!("value:{}:systolic", record.id),
                                    record_id: parsed_id,
                                    entity,
                                    title: format!("{} — Systolic", record.title),
                                    summary_text: format!("{sys} mmHg"),
                                    range_label: None,
                                    source_label: record.source.clone(),
                                    payload: serde_json::json!({ "field": "systolic", "value": sys, "unit": "mmHg" }),
                                },
                                base_url: base_url.clone(),
                                bearer: bearer.clone(),
                            }
                            MarkToggleButton {
                                target_key: format!("value:{}:diastolic", record.id),
                                request: MarkForDoctorRequest {
                                    target_kind: MarkTargetKind::Value,
                                    target_key: format!("value:{}:diastolic", record.id),
                                    record_id: parsed_id,
                                    entity,
                                    title: format!("{} — Diastolic", record.title),
                                    summary_text: format!("{dia} mmHg"),
                                    range_label: None,
                                    source_label: record.source.clone(),
                                    payload: serde_json::json!({ "field": "diastolic", "value": dia, "unit": "mmHg" }),
                                },
                                base_url: base_url.clone(),
                                bearer: bearer.clone(),
                            }
                        }
                    }
                }
                if let Some(draft) = (fmd.note_editor)()
                    && draft.linked_record_id == parsed_id
                {
                    DoctorNoteEditor {
                        draft,
                        base_url: base_url.clone(),
                        bearer: bearer.clone(),
                        on_saved: {
                            let retention = patient.retention.clone();
                            let fence = patient.fence.clone();
                            move |saved: ForMyDoctorNote| {
                                if let Some(mut st) = (fmd.state)() {
                                    st.notes.retain(|n| n.id != saved.id);
                                    st.notes.insert(0, saved);
                                    fmd.state.set(Some(st));
                                }
                                let _ = retention.clear_doctor_note_draft(&fence);
                                fmd.note_editor.set(None);
                                fmd.note_unsaved.set(None);
                            }
                        },
                        on_cancel: {
                            let retention = patient.retention.clone();
                            let fence = patient.fence.clone();
                            move |_| {
                                let _ = retention.clear_doctor_note_draft(&fence);
                                fmd.note_editor.set(None);
                                fmd.note_unsaved.set(None);
                            }
                        },
                    }
                }
            }
        }
    }
}

/// Free-text note editor shared between `Shared` (`For my doctor`) and open records in `You`.
/// Preserves unsaved text locally (`"Not saved yet"`) and displays `409 version_conflict`
/// via `ui::DraftConflict` while keeping the patient's unsaved draft words intact (`Rule 9`).
#[component]
fn DoctorNoteEditor(
    draft: RetainedDoctorNoteDraft,
    base_url: String,
    bearer: String,
    on_saved: EventHandler<ForMyDoctorNote>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut fmd = use_context::<ForMyDoctorSession>();
    let patient = use_context::<PatientSession>();
    let mut body_text = use_signal(|| draft.unsaved_body.clone());
    let mut saving = use_signal(|| false);
    let mut conflict_note = use_signal(|| draft.conflict_latest.clone());
    let mut expected_version = use_signal(|| draft.expected_version);
    let is_new = use_signal(|| draft.is_new);

    let persist_draft = {
        let retention = patient.retention.clone();
        let fence = patient.fence.clone();
        let note_id = draft.note_id;
        let linked_record_id = draft.linked_record_id;
        let linked_record_title = draft.linked_record_title.clone();
        let linked_entity = draft.linked_entity;
        move |text: String, latest: Option<ForMyDoctorNote>, ver: Option<i64>, is_new_flag: bool| {
            let next = RetainedDoctorNoteDraft {
                note_id,
                is_new: is_new_flag,
                expected_version: ver,
                unsaved_body: text,
                linked_record_id,
                linked_record_title: linked_record_title.clone(),
                linked_entity,
                conflict_latest: latest,
            };
            let _ = retention.save_doctor_note_draft(&fence, &next);
            fmd.note_editor.set(Some(next));
        }
    };

    let save_note = {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        let note_id = draft.note_id;
        let linked_record_id = draft.linked_record_id;
        let linked_record_title = draft.linked_record_title.clone();
        let linked_entity = draft.linked_entity;
        let persist = persist_draft.clone();
        move |_| {
            let trimmed = body_text().trim().to_owned();
            if trimmed.is_empty() || saving() {
                return;
            }
            saving.set(true);
            fmd.note_unsaved.set(None);
            let url = base_url.clone();
            let token = bearer.clone();
            let mut persist = persist.clone();
            let current_ver = expected_version();
            let current_is_new = is_new();
            let raw_body = body_text();
            let linked_title = linked_record_title.clone();
            spawn(async move {
                let res = match open_client(url, token) {
                    Ok(api) => {
                        if current_is_new {
                            api.create_for_my_doctor_note(&CreateDoctorNoteRequest {
                                id: note_id,
                                body: raw_body.clone(),
                                linked_record_id,
                                linked_record_title: linked_title,
                                linked_entity,
                            })
                            .await
                        } else {
                            api.update_for_my_doctor_note(
                                note_id,
                                &UpdateDoctorNoteRequest {
                                    body: raw_body.clone(),
                                    expected_version: current_ver.unwrap_or(1),
                                },
                            )
                            .await
                        }
                    }
                    Err(err) => Err(ForMyDoctorClientError::Uncertain(err)),
                };
                match res {
                    Ok(saved) => {
                        conflict_note.set(None);
                        on_saved.call(saved);
                    }
                    Err(ForMyDoctorClientError::VersionConflict(latest)) => {
                        expected_version.set(Some(latest.version));
                        conflict_note.set(Some((*latest).clone()));
                        persist(
                            raw_body,
                            Some(*latest),
                            Some(current_ver.unwrap_or(1)),
                            false,
                        );
                        fmd.note_unsaved.set(Some(
                            "Not saved yet — another device saved a newer version. Your words below were kept.".to_owned(),
                        ));
                    }
                    Err(_) => {
                        persist(raw_body, conflict_note(), current_ver, current_is_new);
                        fmd.note_unsaved.set(Some(
                            "Not saved yet — couldn't reach Duet. Your unsaved words are kept here.".to_owned(),
                        ));
                    }
                }
                saving.set(false);
            });
        }
    };

    rsx! {
        Card {
            overline: Some(
                draft
                    .linked_record_title
                    .as_ref()
                    .map_or_else(|| "Note for my doctor".to_owned(), |t| format!("Note about {t}")),
            ),
            if let Some(unsaved_msg) = (fmd.note_unsaved)() {
                Notice {
                    tone: Tone::Warning,
                    icon: Some(IconName::Alert),
                    title: "Not saved yet".to_owned(),
                    detail: Some(unsaved_msg),
                }
            }
            if let Some(latest) = conflict_note() {
                DraftConflict {
                    message: "Edited on another device while you were writing.".to_owned(),
                    label: "Newer saved note".to_owned(),
                    saved: latest.body.clone(),
                    ActionRow {
                        Button {
                            compact: true,
                            onclick: move |_| {
                                expected_version.set(Some(latest.version));
                                body_text.set(latest.body.clone());
                                conflict_note.set(None);
                                fmd.note_unsaved.set(None);
                            },
                            "Use saved note"
                        }
                        Button {
                            compact: true,
                            variant: ButtonVariant::Primary,
                            onclick: {
                                let ver = latest.version;
                                move |_| {
                                    expected_version.set(Some(ver));
                                    conflict_note.set(None);
                                }
                            },
                            "Keep my words and save"
                        }
                    }
                }
            }
            TextField {
                label: "Note".to_owned(),
                kind: TextKind::Multiline,
                value: body_text(),
                placeholder: "Write what you want to share or remember for your visit".to_owned(),
                disabled: saving(),
                on_input: {
                    let mut persist = persist_draft.clone();
                    move |next: String| {
                        body_text.set(next.clone());
                        persist(next, conflict_note(), expected_version(), is_new());
                    }
                },
            }
            ActionRow {
                Button {
                    variant: ButtonVariant::Primary,
                    disabled: saving() || body_text().trim().is_empty(),
                    aria_label: "Save note",
                    onclick: save_note,
                    if saving() { "Saving…" } else { "Save note" }
                }
                Button {
                    variant: ButtonVariant::Quiet,
                    disabled: saving(),
                    onclick: move |_| on_cancel.call(()),
                    "Cancel"
                }
            }
        }
    }
}

/// Complete `"For my doctor"` tab (`Shared`): `To send`, `Choosing` disclosure flow,
/// `"Couldn't confirm the send"` + `"Try again"` / `COD-51`, and the flat `Sent` list.
#[component]
pub fn SharedForMyDoctor(base_url: String, bearer: String) -> Element {
    let mut fmd = use_context::<ForMyDoctorSession>();
    let patient = use_context::<PatientSession>();
    let state_opt = (fmd.state)();
    let loading = (fmd.loading)();
    let load_failed = (fmd.load_failed)();

    let pending_send = (fmd.pending_send)();
    let choosing_opt = (fmd.choosing)();
    let active_note = (fmd.note_editor)();
    let connected = fmd.connected_practices();

    let mut retrying_pending = use_signal(|| false);

    let retry_pending = {
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        let retention = patient.retention.clone();
        let fence = patient.fence.clone();
        move |_| {
            let Some(pending) = (fmd.pending_send)() else {
                return;
            };
            if pending.ended_before_confirm || retrying_pending() {
                return;
            }
            retrying_pending.set(true);
            let url = base_url.clone();
            let token = bearer.clone();
            let retention = retention.clone();
            let fence = fence.clone();
            spawn(async move {
                let res = match open_client(url, token) {
                    Ok(api) => api.send_for_my_doctor(&pending.payload, None).await,
                    Err(err) => Err(ForMyDoctorClientError::Uncertain(err)),
                };
                match res {
                    Ok(_sent) => {
                        let _ = retention.clear_pending_doctor_send(&fence);
                        fmd.pending_send.set(None);
                        fmd.send_notice.set(Some((
                            Tone::Primary,
                            format!("Sent to {}.", pending.payload.practice_name),
                        )));
                        fmd.reload_tick.set((fmd.reload_tick)() + 1);
                    }
                    Err(ForMyDoctorClientError::ConnectionEnded { reason, .. }) => {
                        // COD-51: connection ended while `"Couldn't confirm the send"`.
                        // Remove `"Try again"`, keep marks/notes in `To send`.
                        let updated = PendingDoctorSend {
                            payload: pending.payload,
                            ended_before_confirm: true,
                            ended_reason: Some(reason),
                        };
                        let _ = retention.save_pending_doctor_send(&fence, &updated);
                        fmd.pending_send.set(Some(updated));
                    }
                    Err(ForMyDoctorClientError::Uncertain(_)) => {}
                    Err(err) => {
                        let _ = retention.clear_pending_doctor_send(&fence);
                        fmd.pending_send.set(None);
                        fmd.send_notice.set(Some((Tone::Error, err.to_string())));
                    }
                }
                retrying_pending.set(false);
            });
        }
    };

    rsx! {
        div { class: "duet-heading",
            h1 { "For my doctor" }
            p { class: "duet-status", role: "status",
                if loading && state_opt.is_none() { "Loading…" }
            }
        }

        if let Some(pending) = pending_send.clone() {
            if pending.ended_before_confirm {
                Notice {
                    tone: Tone::Warning,
                    icon: Some(IconName::Alert),
                    title: "Couldn't confirm before connection ended".to_owned(),
                    detail: Some(format!(
                        "{} Everything you chose is still in To send.",
                        pending
                            .ended_reason
                            .as_deref()
                            .unwrap_or("The practice connection ended.")
                    )),
                    ActionRow {
                        Button {
                            compact: true,
                            variant: ButtonVariant::Quiet,
                            onclick: {
                                let retention = patient.retention.clone();
                                let fence = patient.fence.clone();
                                move |_| {
                                    let _ = retention.clear_pending_doctor_send(&fence);
                                    fmd.pending_send.set(None);
                                }
                            },
                            "Dismiss"
                        }
                    }
                }
            } else {
                Notice {
                    tone: Tone::Warning,
                    icon: Some(IconName::Alert),
                    title: "Couldn't confirm the send".to_owned(),
                    detail: Some(format!(
                        "Sending to {} wasn't confirmed yet. Your marks and notes stayed in To send.",
                        pending.payload.practice_name
                    )),
                    TryAgain {
                        label: "Try again".to_owned(),
                        disabled: retrying_pending(),
                        running: retrying_pending().then(|| "Trying again".to_owned()),
                        on_retry: retry_pending,
                    }
                }
            }
        }

        if let Some((tone, msg)) = (fmd.send_notice)() {
            Notice {
                tone,
                title: msg,
                ActionRow {
                    Button {
                        compact: true,
                        variant: ButtonVariant::Quiet,
                        onclick: move |_| fmd.send_notice.set(None),
                        "Dismiss"
                    }
                }
            }
        }

        if load_failed && state_opt.is_none() {
            Notice {
                tone: Tone::Error,
                icon: Some(IconName::Alert),
                title: "For my doctor couldn't load".to_owned(),
                TryAgain {
                    label: "Try again".to_owned(),
                    on_retry: move |_| fmd.reload_tick.set((fmd.reload_tick)() + 1),
                }
            }
        } else if let Some(state) = state_opt {
            if let Some(choosing) = choosing_opt {
                ChoosingPanel {
                    state: state.clone(),
                    choosing,
                    connected: connected.clone(),
                    base_url: base_url.clone(),
                    bearer: bearer.clone(),
                }
            } else {
                // `To send` section
                Card {
                    overline: Some("To send".to_owned()),
                    if let Some(unsaved_msg) = (fmd.mark_unsaved)() {
                        Notice {
                            tone: Tone::Warning,
                            icon: Some(IconName::Alert),
                            title: "Not saved yet".to_owned(),
                            detail: Some(unsaved_msg),
                        }
                    }
                    if state.marks.is_empty() && state.notes.is_empty() {
                        p {
                            "Nothing marked to send yet. Mark a record, reading, or trend in You, or add a note here."
                        }
                    }
                    for mark in state.marks.clone() {
                        MarkedItemRow {
                            key: "{mark.id}",
                            mark,
                            base_url: base_url.clone(),
                            bearer: bearer.clone(),
                        }
                    }
                    for note in state.notes.clone() {
                        NoteItemRow {
                            key: "{note.id}",
                            note,
                            base_url: base_url.clone(),
                            bearer: bearer.clone(),
                        }
                    }
                    if let Some(draft) = active_note.clone()
                        && draft.linked_record_id.is_none()
                    {
                        DoctorNoteEditor {
                            draft,
                            base_url: base_url.clone(),
                            bearer: bearer.clone(),
                            on_saved: {
                                let retention = patient.retention.clone();
                                let fence = patient.fence.clone();
                                move |saved: ForMyDoctorNote| {
                                    if let Some(mut st) = (fmd.state)() {
                                        st.notes.retain(|n| n.id != saved.id);
                                        st.notes.insert(0, saved);
                                        fmd.state.set(Some(st));
                                    }
                                    let _ = retention.clear_doctor_note_draft(&fence);
                                    fmd.note_editor.set(None);
                                    fmd.note_unsaved.set(None);
                                }
                            },
                            on_cancel: {
                                let retention = patient.retention.clone();
                                let fence = patient.fence.clone();
                                move |_| {
                                    let _ = retention.clear_doctor_note_draft(&fence);
                                    fmd.note_editor.set(None);
                                    fmd.note_unsaved.set(None);
                                }
                            },
                        }
                    }
                    ActionRow {
                        Button {
                            compact: true,
                            aria_label: "Add note",
                            onclick: move |_| {
                                fmd.note_unsaved.set(None);
                                fmd.note_editor.set(Some(RetainedDoctorNoteDraft {
                                    note_id: Uuid::new_v4(),
                                    is_new: true,
                                    expected_version: None,
                                    unsaved_body: String::new(),
                                    linked_record_id: None,
                                    linked_record_title: None,
                                    linked_entity: None,
                                    conflict_latest: None,
                                }));
                            },
                            "Add note"
                        }
                        if connected.is_empty() {
                            span { class: "record-meta", "No practice connected yet. Connect with a care team to send." }
                        } else if pending_send.as_ref().is_some_and(|p| !p.ended_before_confirm) {
                            span { class: "record-meta", "Confirm or retry your earlier send first." }
                        } else {
                            Button {
                                variant: ButtonVariant::Primary,
                                compact: true,
                                disabled: state.marks.is_empty() && state.notes.is_empty(),
                                aria_label: if connected.len() == 1 {
                                    format!("Send to {}", connected[0].practice_name)
                                } else {
                                    "Send".to_owned()
                                },
                                onclick: {
                                    let st = state.clone();
                                    move |_| start_to_send_choosing(fmd, &st)
                                },
                                if connected.len() == 1 {
                                    "Send to {connected[0].practice_name}"
                                } else {
                                    "Send"
                                }
                            }
                        }
                    }
                }

                // `Sent` flat list (Rule 6: never a chat thread, never `"physician looked"`)
                Paper { title: "Sent".to_owned(),
                    if state.sends.is_empty() {
                        p { "Nothing sent yet." }
                    } else {
                        for send in state.sends.clone() {
                            SentItemCard { key: "{send.id}", send }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MarkedItemRow(mark: ForMyDoctorMark, base_url: String, bearer: String) -> Element {
    let mut fmd = use_context::<ForMyDoctorSession>();
    let summary_clean = plain_summary_without_ranges(&mark.summary_text);
    let mark_id = mark.id;
    rsx! {
        Card {
            tinted: true,
            overline: mark.range_label.clone().or_else(|| mark.source_label.clone()),
            title: Some(mark.title.clone()),
            detail: (!summary_clean.is_empty()).then_some(summary_clean),
            if mark.source_corrected {
                StatePill {
                    tone: Tone::Primary,
                    icon: Some(IconName::Check),
                    if let Some(date) = &mark.correction_date {
                        "Updated by source ({date})"
                    } else {
                        "Updated by source"
                    }
                }
            }
            ActionRow {
                Button {
                    compact: true,
                    variant: ButtonVariant::Quiet,
                    aria_label: format!("Unmark {}", mark.title),
                    onclick: move |_| {
                        let url = base_url.clone();
                        let token = bearer.clone();
                        spawn(async move {
                            if let Ok(api) = open_client(url, token) {
                                match api.unmark_for_doctor_by_id(mark_id).await {
                                    Ok(()) => {
                                        if let Some(mut st) = (fmd.state)() {
                                            st.marks.retain(|m| m.id != mark_id);
                                            fmd.state.set(Some(st));
                                        }
                                        fmd.mark_unsaved.set(None);
                                    }
                                    Err(_) => {
                                        fmd.mark_unsaved.set(Some(
                                            "Not saved yet — couldn't unmark that item right now.".to_owned(),
                                        ));
                                    }
                                }
                            }
                        });
                    },
                    "Unmark"
                }
            }
        }
    }
}

#[component]
fn NoteItemRow(note: ForMyDoctorNote, base_url: String, bearer: String) -> Element {
    let mut fmd = use_context::<ForMyDoctorSession>();
    let note_for_edit = note.clone();
    let note_id = note.id;
    rsx! {
        Card {
            tinted: true,
            overline: note
                .linked_record_title
                .as_ref()
                .map(|t| format!("Note about {t}")),
            p { "{note.body}" }
            ActionRow {
                Button {
                    compact: true,
                    variant: ButtonVariant::Quiet,
                    onclick: move |_| {
                        fmd.note_unsaved.set(None);
                        fmd.note_editor.set(Some(RetainedDoctorNoteDraft {
                            note_id: note_for_edit.id,
                            is_new: false,
                            expected_version: Some(note_for_edit.version),
                            unsaved_body: note_for_edit.body.clone(),
                            linked_record_id: note_for_edit.linked_record_id,
                            linked_record_title: note_for_edit.linked_record_title.clone(),
                            linked_entity: note_for_edit.linked_entity,
                            conflict_latest: None,
                        }));
                    },
                    "Edit"
                }
                Button {
                    compact: true,
                    variant: ButtonVariant::Quiet,
                    danger: true,
                    onclick: move |_| {
                        let url = base_url.clone();
                        let token = bearer.clone();
                        spawn(async move {
                            if let Ok(api) = open_client(url, token) {
                                match api.delete_for_my_doctor_note(note_id).await {
                                    Ok(()) => {
                                        if let Some(mut st) = (fmd.state)() {
                                            st.notes.retain(|n| n.id != note_id);
                                            fmd.state.set(Some(st));
                                        }
                                        fmd.mark_unsaved.set(None);
                                    }
                                    Err(_) => {
                                        fmd.mark_unsaved.set(Some(
                                            "Not saved yet — couldn't remove that note right now.".to_owned(),
                                        ));
                                    }
                                }
                            }
                        });
                    },
                    "Delete"
                }
            }
        }
    }
}

#[component]
fn ChoosingPanel(
    state: ForMyDoctorState,
    choosing: ChoosingSession,
    connected: Vec<PracticeConnection>,
    base_url: String,
    bearer: String,
) -> Element {
    let mut fmd = use_context::<ForMyDoctorSession>();
    let patient = use_context::<PatientSession>();
    let selected_practice = choosing
        .practice_index
        .and_then(|idx| connected.get(idx))
        .cloned();
    let has_selection = (choosing.ad_hoc.is_some() && choosing.include_ad_hoc)
        || !choosing.mark_ids.is_empty()
        || !choosing.note_ids.is_empty();
    let is_sending = matches!(choosing.send_status, ChoosingSendStatus::Sending);
    let can_confirm =
        selected_practice.is_some() && has_selection && choosing.disclosure_accepted && !is_sending;

    let practice_choices: Vec<Choice> = connected
        .iter()
        .map(|p| Choice::from(p.practice_name.as_str()))
        .collect();

    let confirm_send = {
        let retention = patient.retention.clone();
        let fence = patient.fence.clone();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        let practice = selected_practice.clone();
        let session = choosing.clone();
        move |_| {
            let Some(practice) = practice.clone() else {
                return;
            };
            if !session.disclosure_accepted
                || matches!(session.send_status, ChoosingSendStatus::Sending)
            {
                return;
            }
            let payload = SendForMyDoctorPayload {
                share_key: format!("fmd-{}", Uuid::new_v4()),
                practice_id: practice.practice_id.clone(),
                practice_name: practice.practice_name.clone(),
                patient_id: practice.patient_id.clone(),
                disclosure_accepted: true,
                selected_mark_ids: session.mark_ids.iter().copied().collect(),
                selected_note_ids: session.note_ids.iter().copied().collect(),
                ad_hoc_record: session
                    .include_ad_hoc
                    .then(|| session.ad_hoc.clone())
                    .flatten(),
            };
            let pending = PendingDoctorSend {
                payload: payload.clone(),
                ended_before_confirm: false,
                ended_reason: None,
            };
            let _ = retention.save_pending_doctor_send(&fence, &pending);
            let mut next = session.clone();
            next.send_status = ChoosingSendStatus::Sending;
            fmd.choosing.set(Some(next));

            let url = base_url.clone();
            let token = bearer.clone();
            let retention = retention.clone();
            let fence = fence.clone();
            spawn(async move {
                let res = match open_client(url, token) {
                    Ok(api) => api.send_for_my_doctor(&payload, None).await,
                    Err(err) => Err(ForMyDoctorClientError::Uncertain(err)),
                };
                match res {
                    Ok(_) => {
                        let _ = retention.clear_pending_doctor_send(&fence);
                        fmd.pending_send.set(None);
                        fmd.choosing.set(None);
                        fmd.send_notice.set(Some((
                            Tone::Primary,
                            format!("Sent to {}.", payload.practice_name),
                        )));
                        fmd.reload_tick.set((fmd.reload_tick)() + 1);
                    }
                    Err(ForMyDoctorClientError::Uncertain(_)) => {
                        // Hold `"Couldn't confirm the send"` + `"Try again"` with exact same `share_key`.
                        fmd.pending_send.set(Some(pending));
                        fmd.choosing.set(None);
                    }
                    Err(ForMyDoctorClientError::ConnectionEnded { reason, .. }) => {
                        // Rule 5: connection ended during initial `Sending` -> `"Not sent"`, marks/notes kept.
                        let _ = retention.clear_pending_doctor_send(&fence);
                        fmd.pending_send.set(None);
                        fmd.choosing.set(None);
                        fmd.send_notice
                            .set(Some((Tone::Error, format!("Not sent — {reason}"))));
                    }
                    Err(err) => {
                        let _ = retention.clear_pending_doctor_send(&fence);
                        if let Some(mut cur) = (fmd.choosing)() {
                            cur.send_status = ChoosingSendStatus::Failed(err.to_string());
                            fmd.choosing.set(Some(cur));
                        }
                    }
                }
            });
        }
    };

    rsx! {
        Paper {
            title: "Choose what to send".to_owned(),
            detail: selected_practice
                .as_ref()
                .map(|p| format!("Sending to {}", p.practice_name)),
            if connected.len() > 1 {
                RadioGroup {
                    label: "Choose practice".to_owned(),
                    options: practice_choices,
                    selected: choosing.practice_index,
                    on_select: move |idx| {
                        if let Some(mut cur) = (fmd.choosing)() {
                            cur.practice_index = Some(idx);
                            fmd.choosing.set(Some(cur));
                        }
                    },
                }
            }
            if let Some(rec) = &choosing.ad_hoc {
                Checkbox {
                    label: rec.title.clone(),
                    detail: (!rec.summary_text.is_empty()).then_some(rec.summary_text.clone()),
                    checked: choosing.include_ad_hoc,
                    on_change: move |checked| {
                        if let Some(mut cur) = (fmd.choosing)() {
                            cur.include_ad_hoc = checked;
                            fmd.choosing.set(Some(cur));
                        }
                    },
                }
            }
            for mark in state.marks.clone() {
                {
                    let id = mark.id;
                    let checked = choosing.mark_ids.contains(&id);
                    let detail = plain_summary_without_ranges(&mark.summary_text);
                    rsx! {
                        Checkbox {
                            key: "{id}",
                            label: mark.title.clone(),
                            detail: (!detail.is_empty()).then_some(detail),
                            checked,
                            on_change: move |on| {
                                if let Some(mut cur) = (fmd.choosing)() {
                                    if on {
                                        cur.mark_ids.insert(id);
                                    } else {
                                        cur.mark_ids.remove(&id);
                                    }
                                    fmd.choosing.set(Some(cur));
                                }
                            },
                        }
                    }
                }
            }
            for note in state.notes.clone() {
                {
                    let id = note.id;
                    let checked = choosing.note_ids.contains(&id);
                    rsx! {
                        Checkbox {
                            key: "{id}",
                            label: note
                                .linked_record_title
                                .as_ref()
                                .map_or_else(|| "Note".to_owned(), |t| format!("Note about {t}")),
                            detail: Some(note.body.clone()),
                            checked,
                            on_change: move |on| {
                                if let Some(mut cur) = (fmd.choosing)() {
                                    if on {
                                        cur.note_ids.insert(id);
                                    } else {
                                        cur.note_ids.remove(&id);
                                    }
                                    fmd.choosing.set(Some(cur));
                                }
                            },
                        }
                    }
                }
            }
            Notice {
                tone: Tone::Warning,
                icon: Some(IconName::Alert),
                title: "Before you send".to_owned(),
                detail: Some(SEND_DISCLOSURE_TEXT.to_owned()),
            }
            Checkbox {
                label: "I understand this can't be taken back and isn't for urgent needs.".to_owned(),
                detail: None,
                checked: choosing.disclosure_accepted,
                on_change: move |checked| {
                    if let Some(mut cur) = (fmd.choosing)() {
                        cur.disclosure_accepted = checked;
                        fmd.choosing.set(Some(cur));
                    }
                },
            }
            if let ChoosingSendStatus::Failed(err) = &choosing.send_status {
                Notice {
                    tone: Tone::Error,
                    icon: Some(IconName::Alert),
                    title: err.clone(),
                }
            }
            ActionRow {
                Button {
                    variant: ButtonVariant::Primary,
                    disabled: !can_confirm,
                    aria_label: "Send now",
                    onclick: confirm_send,
                    if is_sending { "Sending…" } else { "Send now" }
                }
                Button {
                    variant: ButtonVariant::Quiet,
                    disabled: is_sending,
                    onclick: move |_| fmd.choosing.set(None),
                    "Cancel"
                }
            }
        }
    }
}

/// Formats the plain standing text for one immutable `ForMyDoctorSend` row.
pub fn send_standing_label(send: &ForMyDoctorSend) -> String {
    match &send.standing {
        SendStanding::Received { received_at } => {
            format!("Received by the care team ({received_at})")
        }
        SendStanding::Sent => {
            if let Some(ended) = &send.connection_ended_at {
                format!("Sent ({}) · Connection ended ({ended})", send.sent_at)
            } else {
                format!("Sent ({})", send.sent_at)
            }
        }
    }
}

#[component]
fn SentItemCard(send: ForMyDoctorSend) -> Element {
    let standing_text = send_standing_label(&send);
    let summary_clean = plain_summary_without_ranges(&send.summary_text);
    let received = matches!(send.standing, SendStanding::Received { .. });
    rsx! {
        Card {
            overline: Some(send.practice_name.clone()),
            title: Some(send.title.clone()),
            detail: (!summary_clean.is_empty()).then_some(summary_clean),
            StatePill {
                tone: if received { Tone::Primary } else { Tone::Neutral },
                icon: received.then_some(IconName::Check),
                "{standing_text}"
            }
        }
    }
}

fn parse_bp_values(summary: &str) -> Option<(i32, i32)> {
    let token = summary.split_whitespace().next()?;
    let (sys, dia) = token.split_once('/')?;
    let sys: i32 = sys.parse().ok()?;
    let dia: i32 = dia.parse().ok()?;
    Some((sys, dia))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_standing_label_never_mentions_physician_reading_status() {
        let sent = ForMyDoctorSend {
            id: Uuid::new_v4(),
            share_key: "k1".into(),
            practice_id: "p1".into(),
            practice_name: "Northside Clinic".into(),
            patient_id: "pt1".into(),
            title: "LDL cholesterol".into(),
            summary_text: "98 mg/dL".into(),
            records: vec![],
            notes: vec![],
            standing: SendStanding::Sent,
            sent_at: "2026-11-20T10:00:00Z".into(),
            connection_ended_at: Some("2026-11-20T12:00:00Z".into()),
        };
        let label = send_standing_label(&sent);
        assert!(label.contains("Sent") && label.contains("Connection ended"));
        assert!(!label.to_ascii_lowercase().contains("looked"));

        let received = ForMyDoctorSend {
            standing: SendStanding::Received {
                received_at: "2026-11-20T10:05:00Z".into(),
            },
            ..sent
        };
        assert!(send_standing_label(&received).starts_with("Received by the care team"));
    }

    #[test]
    fn plain_summary_strips_reference_range_lines() {
        let cleaned =
            plain_summary_without_ranges("142 mg/dL\nReference range: <100 mg/dL\nFasting sample");
        assert_eq!(cleaned, "142 mg/dL · Fasting sample");
    }
}
