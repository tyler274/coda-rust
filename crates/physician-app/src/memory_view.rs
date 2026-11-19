//! Physician-facing patient memory section (`Memory.Code.20` and `Memory.Code.22`).
//!
//! Displays only `summary.md` and `direction.md` (subject pages and the log
//! are Coda's working files and are never shown here). Inline edits are saved
//! as notes in the physician's own words for patient rounds to fold in.

use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use physician_client::{HttpError, PhysicianClient};
use physician_wire::{
    MemoryDocumentWire, MemoryEditTarget, PatientMemoryResponse, SavePhysicianMemoryEditRequest,
};
use ui::{ActionRow, Button, ButtonVariant, Card, EmptyLine, Notice, TextField, TextKind, Tone};

static MEMORY_EDIT_SEQ: AtomicU64 = AtomicU64::new(1);

fn next_edit_key(workspace_id: &str, target: MemoryEditTarget) -> String {
    let seq = MEMORY_EDIT_SEQ.fetch_add(1, Ordering::Relaxed);
    let kind = match target {
        MemoryEditTarget::Summary => "summary",
        MemoryEditTarget::Direction => "direction",
    };
    format!("ui-memory-edit-{workspace_id}-{kind}-{seq}")
}

fn save_error_message(err: &HttpError) -> String {
    if let HttpError::Status { body, .. } = err
        && let Ok(val) = serde_json::from_str::<serde_json::Value>(body)
        && let Some(detail) = val.get("detail").and_then(serde_json::Value::as_str)
        && !detail.trim().is_empty()
    {
        return detail.to_owned();
    }
    "Not saved. Coda could not save your memory note right now.".to_owned()
}

/// Patient-page section rendering `Summary` and `Direction` with inline
/// note-saving edits.
#[component]
pub(crate) fn PatientMemorySection(
    workspace_id: ReadSignal<String>,
    client: Signal<PhysicianClient>,
    workspace_attempt: Signal<u32>,
) -> Element {
    let mut local_attempt = use_signal(|| 0_u32);
    let mut tracked_workspace = use_signal(|| workspace_id.peek().clone());
    let mut save_generation = use_signal(|| 0_u64);
    let mut editing_target = use_signal(|| None::<MemoryEditTarget>);
    let mut draft_text = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut saved_notice = use_signal(|| None::<String>);
    let mut error_notice = use_signal(|| None::<String>);

    // Reset all editor state and invalidate in-flight saves whenever the open
    // patient changes so draft PHI and notices can never cross patients.
    use_effect(move || {
        let current_ws = workspace_id();
        if *tracked_workspace.peek() != current_ws {
            tracked_workspace.set(current_ws);
            editing_target.set(None);
            draft_text.set(String::new());
            busy.set(false);
            saved_notice.set(None);
            error_notice.set(None);
            save_generation += 1;
        }
    });

    let read_res = use_resource(move || {
        let ws = workspace_id();
        let api = client();
        let _ws_try = workspace_attempt();
        let _local = local_attempt();
        async move { (ws.clone(), api.patient_memory(&ws).await) }
    });

    let current_ws = workspace_id();
    let same_patient = tracked_workspace() == current_ws;
    let loaded = read_res.read();
    let Some((asked, answer)) = loaded.as_ref() else {
        return rsx! {};
    };
    if asked != &current_ws {
        return rsx! {};
    }
    let response: PatientMemoryResponse = match answer {
        Ok(resp) => resp.clone(),
        Err(_) => {
            return rsx! {
                section {
                    class: "patient-section",
                    aria_label: "Patient memory",
                    "data-patient-memory": "true",
                    h2 { "Patient memory" }
                    Notice {
                        tone: Tone::Error,
                        title: "Coda couldn’t load patient memory.".to_owned(),
                    }
                    ActionRow {
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| local_attempt += 1,
                            "Try again"
                        }
                    }
                }
            };
        }
    };

    if !response.durable && response.summary.is_none() && response.direction.is_none() {
        return rsx! {};
    }

    let summary_doc = response.summary.clone();
    let direction_doc = response.direction.clone();
    let current_edit = if same_patient { editing_target() } else { None };

    rsx! {
        section {
            class: "patient-section",
            aria_label: "Patient memory",
            "data-patient-memory": "true",
            h2 { "Patient memory" }
            if same_patient && let Some(notice) = saved_notice() {
                div {
                    "data-memory-saved": "true",
                    Notice {
                        tone: Tone::Primary,
                        title: notice,
                    }
                }
            }
            if same_patient && let Some(err) = error_notice() {
                div {
                    "data-memory-error": "true",
                    Notice {
                        tone: Tone::Error,
                        title: err,
                    }
                }
            }
            MemoryDocumentCard {
                workspace_id,
                save_generation,
                target: MemoryEditTarget::Summary,
                heading: "Summary".to_owned(),
                empty_copy: "No memory summary saved yet.".to_owned(),
                doc: summary_doc,
                is_editing: current_edit == Some(MemoryEditTarget::Summary),
                draft_text,
                busy,
                client,
                on_start_edit: move |initial: String| {
                    saved_notice.set(None);
                    error_notice.set(None);
                    draft_text.set(initial);
                    editing_target.set(Some(MemoryEditTarget::Summary));
                },
                on_cancel: move |_| {
                    error_notice.set(None);
                    editing_target.set(None);
                },
                on_saved: move |msg: String| {
                    error_notice.set(None);
                    saved_notice.set(Some(msg));
                    editing_target.set(None);
                    draft_text.set(String::new());
                    local_attempt += 1;
                },
                on_error: move |msg: String| {
                    error_notice.set(Some(msg));
                },
            }
            MemoryDocumentCard {
                workspace_id,
                save_generation,
                target: MemoryEditTarget::Direction,
                heading: "Direction".to_owned(),
                empty_copy: "No standing direction saved yet.".to_owned(),
                doc: direction_doc,
                is_editing: current_edit == Some(MemoryEditTarget::Direction),
                draft_text,
                busy,
                client,
                on_start_edit: move |initial: String| {
                    saved_notice.set(None);
                    error_notice.set(None);
                    draft_text.set(initial);
                    editing_target.set(Some(MemoryEditTarget::Direction));
                },
                on_cancel: move |_| {
                    error_notice.set(None);
                    editing_target.set(None);
                },
                on_saved: move |msg: String| {
                    error_notice.set(None);
                    saved_notice.set(Some(msg));
                    editing_target.set(None);
                    draft_text.set(String::new());
                    local_attempt += 1;
                },
                on_error: move |msg: String| {
                    error_notice.set(Some(msg));
                },
            }
        }
    }
}

#[component]
fn MemoryDocumentCard(
    workspace_id: ReadSignal<String>,
    save_generation: Signal<u64>,
    target: MemoryEditTarget,
    heading: String,
    empty_copy: String,
    doc: Option<MemoryDocumentWire>,
    is_editing: bool,
    mut draft_text: Signal<String>,
    mut busy: Signal<bool>,
    client: Signal<PhysicianClient>,
    on_start_edit: EventHandler<String>,
    on_cancel: EventHandler<()>,
    on_saved: EventHandler<String>,
    on_error: EventHandler<String>,
) -> Element {
    let slug = match target {
        MemoryEditTarget::Summary => "summary",
        MemoryEditTarget::Direction => "direction",
    };
    let edit_label = match target {
        MemoryEditTarget::Summary => "Edit summary",
        MemoryEditTarget::Direction => "Edit direction",
    };
    let target_label = match target {
        MemoryEditTarget::Summary => "Summary",
        MemoryEditTarget::Direction => "Direction",
    };
    let current_text = doc.as_ref().map(|d| d.markdown.clone()).unwrap_or_default();
    let meta = doc
        .as_ref()
        .map(|d| format!("Version {} · Saved {}", d.version, d.saved_at));

    rsx! {
        div {
            "data-memory-doc": "{slug}",
            Card {
                title: heading,
                detail: meta,
                if let Some(existing) = &doc {
                    div {
                        class: "patient-memory-body",
                        "data-memory-body": "{slug}",
                        style: "white-space: pre-wrap;",
                        "{existing.markdown}"
                    }
                } else {
                    EmptyLine { message: empty_copy }
                }
                if is_editing {
                    div {
                        class: "patient-memory-editor",
                        "data-memory-editor": "{slug}",
                        div {
                            "data-memory-input": "{slug}",
                            TextField {
                                label: edit_label.to_owned(),
                                kind: TextKind::Multiline,
                                hint: Some(format!(
                                    "Saved as a note in your words; patient rounds fold it into {target_label}."
                                )),
                                value: draft_text(),
                                disabled: busy(),
                                on_input: move |val| draft_text.set(val),
                                onkeydown: move |ev: KeyboardEvent| ev.stop_propagation(),
                            }
                        }
                        ActionRow {
                            Button {
                                variant: ButtonVariant::Primary,
                                disabled: busy() || draft_text().trim().is_empty(),
                                "data-memory-save": "{slug}",
                                onclick: {
                                    move |_| {
                                        let text = draft_text().trim().to_owned();
                                        if text.is_empty() || busy() {
                                            return;
                                        }
                                        let api = client();
                                        let ws = workspace_id.peek().clone();
                                        let expected_gen = *save_generation.peek();
                                        let key = next_edit_key(&ws, target);
                                        busy.set(true);
                                        spawn(async move {
                                            let req = SavePhysicianMemoryEditRequest {
                                                target,
                                                text,
                                                idempotency_key: key,
                                            };
                                            let outcome =
                                                api.save_patient_memory_edit(&ws, &req).await;
                                            // Drop stale completions if the physician switched patients.
                                            if *save_generation.peek() != expected_gen
                                                || workspace_id.peek().as_str() != ws.as_str()
                                            {
                                                return;
                                            }
                                            match outcome {
                                                Ok(saved) => {
                                                    busy.set(false);
                                                    on_saved.call(format!(
                                                        "Saved as note {} in your words. Patient rounds will fold it into {target_label}.",
                                                        saved.note_number
                                                    ));
                                                }
                                                Err(err) => {
                                                    busy.set(false);
                                                    on_error.call(save_error_message(&err));
                                                }
                                            }
                                        });
                                    }
                                },
                                if busy() { "Saving…" } else { "Save note" }
                            }
                            Button {
                                variant: ButtonVariant::Secondary,
                                disabled: busy(),
                                onclick: move |_| on_cancel.call(()),
                                "Cancel"
                            }
                        }
                    }
                } else {
                    ActionRow {
                        Button {
                            variant: ButtonVariant::Secondary,
                            "data-memory-edit": "{slug}",
                            onclick: {
                                let initial = current_text.clone();
                                move |_| on_start_edit.call(initial.clone())
                            },
                            "{edit_label}"
                        }
                    }
                }
            }
        }
    }
}
